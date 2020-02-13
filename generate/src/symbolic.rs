use itertools::{chain, Itertools};

use crate::*;

// ----------------------------------------------------------------------------

/// A named symbol, e.g. "x" or "point.e20".
/// The blade is the type of this Factor.
/// NOTE: names MUST be unique!
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Factor {
	name: String,
	blade: Blade,
}

impl std::fmt::Display for Factor {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		self.name.fmt(f)
	}
}

// ----------------------------------------------------------------------------

/// Empty factors = one
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Product {
	multiplier: i32,
	factors: Vec<Factor>,
}

impl Product {
	pub fn zero() -> Self {
		Product {
			multiplier: 0,
			factors: vec![],
		}
	}

	pub fn is_zero(&self) -> bool {
		// only valid if simplified
		self.multiplier == 0
	}

	pub fn one() -> Self {
		Product {
			multiplier: 1,
			factors: vec![],
		}
	}

	pub fn from_factor(factor: Factor) -> Self {
		Product {
			multiplier: 1,
			factors: vec![factor],
		}
	}

	/// The dimensionality of this product
	pub fn blade(&self, grammar: &Grammar) -> SignedBlade {
		if self.multiplier == 0 || self.factors.is_empty() {
			SignedBlade::zero()
		} else {
			let mut blade = SignedBlade::unit(&Blade::scalar());
			for f in &self.factors {
				blade *= SignedBlade::unit(&f.blade);
			}
			grammar.simplify(blade)
		}
	}

	pub fn simplify(mut self, grammar: &Grammar) -> Self {
		// sort factors respecting commute rules, using bubble sort:
		for _ in 0..self.factors.len() {
			for i in 0..self.factors.len() - 1 {
				if self.factors[i].name > self.factors[i + 1].name {
					self.multiplier *= grammar.commute_sign(&self.factors[i].blade, &self.factors[i + 1].blade);
					self.factors.swap(i, i + 1);
				}
			}
		}

		if self.blade(grammar).is_zero() {
			Self::zero()
		} else {
			self
		}
	}

	pub fn reverse(mut self) -> Self {
		for factor in &mut self.factors {
			self.multiplier *= factor.blade.reverse_sign();
		}
		self
	}
}

impl Ord for Product {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		if self.factors != other.factors {
			self.factors.cmp(&other.factors)
		} else {
			self.multiplier.cmp(&other.multiplier)
		}
	}
}

impl PartialOrd for Product {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl std::fmt::Display for Product {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		let factors = self.factors.iter().map(ToString::to_string).join("*");
		match self.multiplier {
			-1 => write!(f, "-{}", factors),
			0 => "0".fmt(f),
			1 => factors.fmt(f),
			multiplier => write!(f, "{}*{}", multiplier, factors),
		}
	}
}

impl std::ops::Neg for Product {
	type Output = Product;
	fn neg(self) -> Self::Output {
		Product {
			multiplier: -self.multiplier,
			factors: self.factors,
		}
	}
}

impl std::ops::Mul<&Product> for &Product {
	type Output = Sum;
	fn mul(self, p: &Product) -> Self::Output {
		Sum(vec![self.clone(), p.clone()])
	}
}

impl std::ops::Mul<Product> for i32 {
	type Output = Product;
	fn mul(self, p: Product) -> Self::Output {
		Product {
			multiplier: self * p.multiplier,
			factors: p.factors,
		}
	}
}

impl std::ops::Mul<&Product> for i32 {
	type Output = Product;
	fn mul(self, p: &Product) -> Self::Output {
		Product {
			multiplier: self * p.multiplier,
			factors: p.factors.clone(),
		}
	}
}

impl std::ops::Mul for Product {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self::Output {
		Product {
			multiplier: self.multiplier * rhs.multiplier,
			factors: chain(self.factors, rhs.factors).collect(),
		}
	}
}

// ----------------------------------------------------------------------------

/// A sum-of-products type, used in symbolic calculations.
/// Empty sum = 0
#[derive(Clone, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Sum(Vec<Product>);

impl Sum {
	pub fn one() -> Self {
		Sum(vec![Product::one()])
	}

	pub fn is_zero(&self) -> bool {
		self.0.is_empty() || self.0.iter().all(|p| p.is_zero())
	}

	pub fn from_factor(factor: Factor) -> Self {
		Sum(vec![Product::from_factor(factor)])
	}

	pub fn instance(variable: &str, typ: &Type) -> Self {
		Self(
			typ.0
				.iter()
				.map(|(member, blade)| {
					Product::from_factor(Factor {
						name: format!("{}.{}", variable, member),
						blade: blade.clone(),
					})
				})
				.collect(),
		)
	}

	/// Project onto the given blade.
	/// TODO: actually not a projection, but a selection if a specific dimension.
	pub fn project(&self, needle: &Blade, grammar: &Grammar) -> Sum {
		Sum(self
			.0
			.iter()
			.filter(|term| &term.blade(grammar).blade == needle)
			.cloned()
			.collect())
	}

	pub fn blades(&self, grammar: &Grammar) -> Vec<SignedBlade> {
		self.0.iter().map(|term| term.blade(grammar)).collect()
	}

	pub fn simplify(mut self, grammar: &Grammar) -> Self {
		for p in &mut self.0 {
			*p = p.clone().simplify(grammar);
		}
		self.0.retain(|p| !p.is_zero());
		self.0.sort();

		// Add together terms with same factors
		let mut collapsed_terms: Vec<Product> = vec![];
		for new_term in self.0 {
			if let Some(last_term) = collapsed_terms.last_mut() {
				// NOTE: Same terms also means same dimensionality / blade
				if last_term.factors == new_term.factors {
					// Add them!
					last_term.multiplier += new_term.multiplier;

					if last_term.multiplier == 0 {
						collapsed_terms.pop();
					}

					continue;
				}
			}

			collapsed_terms.push(new_term);
		}

		Sum(collapsed_terms)
	}

	pub fn reverse(&self) -> Self {
		Self(self.0.iter().map(|sum| sum.clone().reverse()).collect())
	}

	// pub fn dual(&self, grammar: &Grammar) -> Self {
	// 	Self(self.0.iter().map(|sum| sum.dual(grammar)).collect()).simplify(grammar)
	// }

	/// The sandwich operator
	/// self * other * self.reverse()
	pub fn sandwich(&self, other: &Self) -> Self {
		self.clone() * other.clone() * self.reverse()
		// self * &(other * &self.reverse())
	}
}

impl std::fmt::Display for Sum {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		// write!(f, "{}", self.0.iter().format(" + "))

		if self.0.is_empty() {
			"0".fmt(f)
		} else {
			write!(f, "{}", self.0[0])?;
			for term in self.0.iter().skip(1) {
				if term.multiplier < 0 {
					write!(f, " - {}", -term.clone())?;
				} else {
					write!(f, " + {}", term)?;
				}
			}
			Ok(())
		}
	}
}

impl std::ops::Neg for Sum {
	type Output = Sum;
	fn neg(self) -> Sum {
		Sum(self.0.into_iter().map(|p| -p).collect())
	}
}

impl std::ops::Add for Sum {
	type Output = Self;
	fn add(self, rhs: Self) -> Self::Output {
		Sum(chain(self.0, rhs.0).collect())
	}
}

impl std::ops::Mul<Sum> for i32 {
	type Output = Sum;
	fn mul(self, sum: Sum) -> Self::Output {
		Sum(sum.0.into_iter().map(|p| self * p).collect())
	}
}

impl std::ops::Mul<&Sum> for i32 {
	type Output = Sum;
	fn mul(self, sum: &Sum) -> Self::Output {
		Sum(sum.0.iter().map(|p| self * p).collect())
	}
}

impl std::ops::Mul<Product> for Sum {
	type Output = Self;
	fn mul(self, rhs: Product) -> Self::Output {
		Sum(self.0.into_iter().map(|p| p * rhs.clone()).collect())
	}
}

impl std::ops::Mul<Sum> for Sum {
	type Output = Self;
	fn mul(self, rhs: Sum) -> Self::Output {
		Sum(self
			.0
			.into_iter()
			.cartesian_product(rhs.0)
			.map(|(a, b)| a * b)
			.collect())
	}
}

impl std::ops::AddAssign<Sum> for Sum {
	fn add_assign(&mut self, rhs: Sum) {
		*self = self.clone() + rhs;
	}
}

impl std::ops::MulAssign<Sum> for Sum {
	fn mul_assign(&mut self, rhs: Sum) {
		*self = self.clone() * rhs;
	}
}

// ----------------------------------------------------------------------------

// impl std::ops::Mul<&MultiVec> for &MultiVec {
// 	type Output = MultiVec;
// 	fn mul(self, rhs: &MultiVec) -> MultiVec {
// 		let mut result = MultiVec::default();
// 		for l in &self.0 {
// 			for r in &rhs.0 {
// 				let signed_blade = &SignedBlade::unit(&l.0) * &SignedBlade::unit(&r.0);
// 				result
// 					.0
// 					.push((signed_blade.blade, signed_blade.sign * l.1.clone() * r.1.clone()));
// 			}
// 		}
// 		result
// 	}
// }

// impl std::fmt::Display for MultiVec {
// 	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
// 		if self.0.is_empty() {
// 			"0".fmt(f)
// 		} else {
// 			write!(
// 				f,
// 				"{{\n{}\n}}",
// 				self.0
// 					.iter()
// 					.map(|(blade, sum)| format!("  {:6} {},", format!("{}:", blade), sum))
// 					.format("\n")
// 			)
// 		}
// 	}
// }
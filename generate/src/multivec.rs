use std::collections::BTreeMap;

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

	pub fn simplify(mut self, grammar: &Grammar) -> Self {
		// sort factors respecting commute rules, using bubble sort:
		for _ in 0..self.factors.len() {
			for i in 0..self.factors.len() - 1 {
				if self.factors[i].name > self.factors[i + 1].name {
					self.multiplier *= grammar
						.commute_sign(&self.factors[i].blade, &self.factors[i + 1].blade)
						.into_i32();
					self.factors.swap(i, i + 1);
				}
			}
		}

		if self.multiplier == 0 {
			Self::zero()
		} else {
			self
		}
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

/// A sum-of-products type, used in symbolic calculations
/// Empty sum = 0
#[derive(Clone, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Sum(Vec<Product>);

impl Sum {
	pub fn zero() -> Self {
		Sum(vec![])
	}

	pub fn one() -> Self {
		Sum(vec![Product::one()])
	}

	pub fn from_factor(factor: Factor) -> Self {
		Sum(vec![Product::from_factor(factor)])
	}

	pub fn simplify(mut self, grammar: &Grammar) -> Self {
		for p in &mut self.0 {
			*p = p.clone().simplify(grammar);
		}
		self.0.retain(|p| *p != Product::zero());
		self.0.sort();

		// Add together terms with same factors
		let mut collapsed_terms: Vec<Product> = vec![];
		for new_term in self.0 {
			if let Some(last_term) = collapsed_terms.last_mut() {
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

/// A linear combination of blades.
#[must_use]
#[derive(Clone, Default)]
pub struct MultiVec(Vec<(Blade, Sum)>);

impl MultiVec {
	pub fn instance(variable: &str, typ: &Type) -> Self {
		Self(
			typ.0
				.iter()
				.map(|(member, blade)| {
					(
						blade.clone(),
						Sum::from_factor(Factor {
							name: format!("{}.{}", variable, member),
							blade: blade.clone(),
						}),
					)
				})
				.collect(),
		)
	}

	pub fn simplify(self, grammar: &Grammar) -> Self {
		let mut blades: BTreeMap<Blade, Sum> = BTreeMap::default();
		for (blade, sum) in self.0 {
			// We simplify the blade to select the destination type
			// so that e20 and e02 goes into the same place.
			// BUT, we should NOT care about the sing here,
			// because that is built-in to the individual blade types.
			// So e1 * e2 and e2 * e1 can both be assigned to e12 without flipping signs here!
			// Still, we need to check for the Zero sign to discard useless dimensions.
			// This is a bit ugly, and the generated code has hidden sign-flips
			// due to those sign-flips being built-in to the actual types.
			// This can make for surprising reading of the code, which is not ideal.

			let canonical_blade = grammar.simplify(SignedBlade::unit(&blade));
			let is_zero_basis = canonical_blade.sign == Sign::Zero;
			if !is_zero_basis {
				let basis = canonical_blade.blade;
				*blades.entry(basis).or_default() += sum;
			}
		}

		MultiVec(
			blades
				.into_iter()
				.filter_map(|(blade, sum)| {
					let sum = sum.simplify(grammar);
					if sum == Sum::zero() {
						None
					} else {
						Some((blade, sum.simplify(grammar)))
					}
				})
				.collect(),
		)
	}

	pub fn reverse(&self) -> Self {
		Self(
			self.0
				.iter()
				.map(|(blade, sum)| {
					let SignedBlade { sign, blade } = SignedBlade::unit(blade).reverse();
					(blade, sign.into_i32() * sum)
				})
				.collect(),
		)
	}

	/// The sandwich operator
	/// self * other * self.reverse()
	pub fn sandwich(&self, other: &Self) -> Self {
		// &(self * other) * &self.reverse()
		self * &(other * &self.reverse())
	}
}

impl std::ops::Mul<&MultiVec> for &MultiVec {
	type Output = MultiVec;
	fn mul(self, rhs: &MultiVec) -> MultiVec {
		let mut result = MultiVec::default();
		for l in &self.0 {
			for r in &rhs.0 {
				let signed_blade = &SignedBlade::unit(&l.0) * &SignedBlade::unit(&r.0);
				result.0.push((
					signed_blade.blade,
					signed_blade.sign.into_i32() * l.1.clone() * r.1.clone(),
				));
			}
		}
		result
	}
}

impl std::fmt::Display for MultiVec {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		if self.0.is_empty() {
			"0".fmt(f)
		} else {
			write!(
				f,
				"{{\n{}\n}}",
				self.0
					.iter()
					.map(|(blade, sum)| format!("  {:6} {},", format!("{}:", blade), sum))
					.format("\n")
			)
		}
	}
}

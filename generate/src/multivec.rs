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
	sign: Sign,
	factors: Vec<Factor>,
}

impl Product {
	pub fn zero() -> Self {
		Product {
			sign: Sign::Zero,
			factors: vec![],
		}
	}

	pub fn one() -> Self {
		Product {
			sign: Sign::Positive,
			factors: vec![],
		}
	}

	pub fn from_factor(factor: Factor) -> Self {
		Product {
			sign: Sign::Positive,
			factors: vec![factor],
		}
	}

	pub fn simplify(mut self, grammar: &Grammar) -> Self {
		// sort factors respecting commute rules, using bubble sort:
		for _ in 0..self.factors.len() {
			for i in 0..self.factors.len() - 1 {
				if self.factors[i].name > self.factors[i + 1].name {
					self.sign *= grammar.commute_sign(&self.factors[i].blade, &self.factors[i + 1].blade);
					self.factors.swap(i, i + 1);
				}
			}
		}

		if self.sign == Sign::Zero {
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
			self.sign.cmp(&other.sign)
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
		match self.sign {
			Sign::Negative => write!(f, "-{}", factors),
			Sign::Zero => "0".fmt(f),
			Sign::Positive => factors.fmt(f),
		}
	}
}

impl std::ops::Neg for Product {
	type Output = Product;
	fn neg(self) -> Self::Output {
		Product {
			sign: -self.sign,
			factors: self.factors,
		}
	}
}

impl std::ops::Mul<Product> for Sign {
	type Output = Product;
	fn mul(self, p: Product) -> Self::Output {
		Product {
			sign: self * p.sign,
			factors: p.factors,
		}
	}
}

impl std::ops::Mul<&Product> for Sign {
	type Output = Product;
	fn mul(self, p: &Product) -> Self::Output {
		Product {
			sign: self * p.sign,
			factors: p.factors.clone(),
		}
	}
}

impl std::ops::Mul for Product {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self::Output {
		Product {
			sign: self.sign * rhs.sign,
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

		// Discover and drop `foo + -foo`
		let mut collapsed_terms: Vec<Product> = vec![];
		for new_term in self.0 {
			if let Some(last_term) = collapsed_terms.last() {
				if last_term.factors == new_term.factors && last_term.sign == -new_term.sign {
					// We have `+foo` followed by `-foo`
					collapsed_terms.pop();
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
		write!(f, "{}", self.0.iter().format(" + "))
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

impl std::ops::Mul<Sum> for Sign {
	type Output = Sum;
	fn mul(self, sum: Sum) -> Self::Output {
		Sum(sum.0.into_iter().map(|p| self * p).collect())
	}
}

impl std::ops::Mul<&Sum> for Sign {
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
			let signed_blade = grammar.simplify(SignedBlade::unit(&blade));
			let sum = signed_blade.sign * sum;
			if sum != Sum::zero() {
				*blades.entry(signed_blade.blade).or_default() += sum;
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
					(blade, sign * sum)
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
				result
					.0
					.push((signed_blade.blade, signed_blade.sign * l.1.clone() * r.1.clone()));
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
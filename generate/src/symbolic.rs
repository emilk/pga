use itertools::{chain, Itertools};

use crate::*;

// ----------------------------------------------------------------------------

/// A named symbol, e.g. "x" or "point.e20".
/// The blade is the type of this Factor.
/// NOTE: names MUST be unique!
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Symbol {
	name: String,
	blade: Blade,
}

impl std::fmt::Display for Symbol {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		self.name.fmt(f)
	}
}

// ----------------------------------------------------------------------------

/// Either 'x' or 'x.dual()'
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Factor {
	Normal(Symbol),
	Dual(Symbol),
}

impl Factor {
	// fn symbol(&self) -> &Symbol {
	// 	match self {
	// 		Factor::Normal(x) => x,
	// 		Factor::Dual(x) => x,
	// 	}
	// }

	fn blade(&self, grammar: &Grammar) -> Blade {
		// &self.symbol().blade
		self.sblade(grammar).blade
	}

	fn sblade(&self, grammar: &Grammar) -> SignedBlade {
		match self {
			Factor::Normal(x) => SignedBlade::unit(&x.blade),
			Factor::Dual(x) => SignedBlade::unit(&x.blade).dual(grammar),
		}
	}

	pub fn dual(self) -> Self {
		match self {
			Factor::Normal(x) => Factor::Dual(x),
			Factor::Dual(x) => Factor::Normal(x),
		}
	}
}

impl std::fmt::Display for Factor {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Factor::Normal(x) => x.fmt(f),
			Factor::Dual(x) => write!(f, "!{}", x),
		}
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
	pub fn sblade(&self, grammar: &Grammar) -> SignedBlade {
		if self.multiplier == 0 || self.factors.is_empty() {
			SignedBlade::zero()
		} else {
			let mut blade = SignedBlade::unit(&Blade::scalar());
			for f in &self.factors {
				blade *= f.sblade(grammar);
			}
			grammar.simplify(blade)
		}
	}

	pub fn simplify(mut self, grammar: &Grammar) -> Self {
		// sort factors respecting commute rules, using bubble sort:
		for _ in 0..self.factors.len() {
			for i in 0..self.factors.len() - 1 {
				if self.factors[i] > self.factors[i + 1] {
					self.multiplier *= grammar.commute_sign(
						&self.factors[i].sblade(grammar).blade,
						&self.factors[i + 1].sblade(grammar).blade,
					);
					self.factors.swap(i, i + 1);
				}
			}
		}

		if self.sblade(grammar).is_zero() {
			Self::zero()
		} else {
			self
		}
	}

	pub fn reverse(mut self, grammar: &Grammar) -> Self {
		for factor in &mut self.factors {
			self.multiplier *= factor.blade(grammar).reverse_sign();
		}
		self
	}

	pub fn dual(mut self) -> Self {
		if self.factors.len() > 1 {
			// Consider  !(!l.e01 * !r.e20)
			// this cannot be simplified!
			panic!("Cannot take the dual of a complex product like this");
		}
		for factor in &mut self.factors {
			*factor = factor.clone().dual();
		}
		self
		// unimplemented!();
	}

	/// Geometric multiplication:
	pub fn mul(self, rhs: Product, grammar: &Grammar) -> Self {
		Product {
			multiplier: self.multiplier * rhs.multiplier,
			factors: chain(self.factors, rhs.factors).collect(),
		}
		.simplify(grammar)
	}

	/// inner / dot product
	/// The dot product either cancels to zero, or is the same as the geometric product
	pub fn inner(self, rhs: Product, grammar: &Grammar) -> Self {
		let self_sblade = self.sblade(grammar);
		let other_sblade = rhs.sblade(grammar);
		if self_sblade.inner(&other_sblade, grammar).is_zero() {
			Self::zero()
		} else {
			self.mul(rhs, grammar)
		}
	}

	/// outer / wedge product
	/// The outer product either cancels to zero, or is the same as the geometric product
	pub fn outer(self, rhs: Product, grammar: &Grammar) -> Self {
		let self_sblade = self.sblade(grammar);
		let other_sblade = rhs.sblade(grammar);
		if self_sblade.outer(&other_sblade, grammar).is_zero() {
			Self::zero()
		} else {
			self.mul(rhs, grammar)
		}
	}

	pub fn regressive(self, rhs: Product, grammar: &Grammar) -> Self {
		self.dual().outer(rhs.dual(), grammar).dual().simplify(grammar)
	}
}

impl Ord for Product {
	fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
		if self.factors != rhs.factors {
			self.factors.cmp(&rhs.factors)
		} else {
			self.multiplier.cmp(&rhs.multiplier)
		}
	}
}

impl PartialOrd for Product {
	fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(rhs))
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

// TODO
// #[derive(Clone, Default, Eq, Ord, PartialEq, PartialOrd)]
// pub enum Term{
// 	Normal(Vec<Product>), Dual(Vec<Product>)};

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
					Product::from_factor(Factor::Normal(Symbol {
						name: format!("{}.{}", variable, member),
						blade: blade.clone(),
					}))
				})
				.collect(),
		)
	}

	/// Select the terms of the given blade
	pub fn select(&self, needle: &Blade, grammar: &Grammar) -> Sum {
		Sum(self
			.0
			.iter()
			.filter(|term| {
				let term_sblade = term.sblade(grammar);
				!term_sblade.is_zero() && &term_sblade.blade == needle
			})
			.cloned()
			.collect())
	}

	pub fn sblades(&self, grammar: &Grammar) -> Vec<SignedBlade> {
		self.0.iter().map(|term| term.sblade(grammar)).collect()
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

	pub fn reverse(&self, grammar: &Grammar) -> Self {
		Self(self.0.iter().map(|sum| sum.clone().reverse(grammar)).collect())
	}

	pub fn dual(&self, grammar: &Grammar) -> Self {
		Self(self.0.iter().map(|sum| sum.clone().dual()).collect()).simplify(grammar)
	}

	pub fn mul(self, rhs: Self) -> Self {
		Sum(self
			.0
			.into_iter()
			.cartesian_product(rhs.0)
			.map(|(a, b)| a * b)
			.collect())
	}

	/// inner / dot product
	pub fn inner(self, rhs: Self, grammar: &Grammar) -> Self {
		Sum(self
			.0
			.into_iter()
			.cartesian_product(rhs.0)
			.map(|(a, b)| a.inner(b, grammar))
			.collect())
		.simplify(grammar)
	}

	/// outer / wedge product
	pub fn outer(self, rhs: Self, grammar: &Grammar) -> Self {
		Sum(self
			.0
			.into_iter()
			.cartesian_product(rhs.0)
			.map(|(a, b)| a.outer(b, grammar))
			.collect())
		.simplify(grammar)
	}

	pub fn regressive(&self, rhs: Self, grammar: &Grammar) -> Self {
		self.dual(grammar)
			.outer(rhs.dual(grammar), grammar)
			.dual(grammar)
			.simplify(grammar)
	}

	/// The sandwich operator
	/// self * rhs * self.reverse()
	pub fn sandwich(&self, rhs: &Self, grammar: &Grammar) -> Self {
		(self.clone() * rhs.clone() * self.reverse(grammar)).simplify(grammar)
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

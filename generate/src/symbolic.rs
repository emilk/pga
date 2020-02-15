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

	pub fn is_negative(&self) -> bool {
		self.multiplier < 0
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

	pub fn try_dual(&self) -> Option<Self> {
		// Consider  !(!l.e01 * !r.e20): this cannot be simplified!
		if self.factors.len() > 1 {
			None
		} else {
			let mut p = self.clone();
			for factor in &mut p.factors {
				*factor = factor.clone().dual();
			}
			Some(p)
		}
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

	pub fn try_add(&self, rhs: &Self) -> Option<Self> {
		if self.is_zero() {
			Some(rhs.clone())
		} else if rhs.is_zero() {
			Some(self.clone())
		} else {
			// NOTE: Same terms also means same dimensionality / blade
			if self.factors == rhs.factors {
				let mut sum = self.clone();
				sum.multiplier += rhs.multiplier;
				Some(sum)
			} else {
				None
			}
		}
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

// ----------------------------------------------------------------------------

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum Term {
	Normal(Product),
	Dual(Product),
}

impl Term {
	pub fn zero() -> Self {
		Term::Normal(Product::zero())
	}

	pub fn one() -> Self {
		Term::Normal(Product::one())
	}

	pub fn is_zero(&self) -> bool {
		match self {
			Term::Normal(p) => p.is_zero(),
			Term::Dual(p) => p.is_zero(),
		}
	}

	pub fn is_negative(&self) -> bool {
		match self {
			Term::Normal(p) => p.is_negative(),
			Term::Dual(p) => p.is_negative(),
		}
	}

	pub fn from_factor(factor: Factor) -> Self {
		Term::Normal(Product::from_factor(factor))
	}

	// fn blade(&self, grammar: &Grammar) -> Blade {
	// 	self.sblade(grammar).blade
	// }

	fn sblade(&self, grammar: &Grammar) -> SignedBlade {
		match self {
			Term::Normal(p) => p.sblade(grammar),
			Term::Dual(p) => p.sblade(grammar).dual(grammar),
		}
	}

	pub fn simplify(self, grammar: &Grammar) -> Self {
		match self {
			Term::Normal(p) => Term::Normal(p.simplify(grammar)),
			Term::Dual(p) => Term::Dual(p.simplify(grammar)),
		}
	}

	pub fn dual(self) -> Self {
		match self {
			Term::Normal(p) => {
				if let Some(pd) = p.try_dual() {
					Term::Normal(pd)
				} else {
					Term::Dual(p)
				}
			}
			Term::Dual(p) => Term::Normal(p),
		}
	}

	pub fn reverse(self, grammar: &Grammar) -> Self {
		match self {
			Term::Normal(p) => Term::Normal(p.reverse(grammar)),
			Term::Dual(p) => Term::Dual(p.reverse(grammar)),
		}
	}

	pub fn try_add(&self, rhs: &Term) -> Option<Term> {
		if self.is_zero() {
			Some(rhs.clone())
		} else if rhs.is_zero() {
			Some(self.clone())
		} else {
			match (self, rhs) {
				(Term::Normal(l), Term::Normal(r)) => l.try_add(r).map(Term::Normal),
				(Term::Dual(l), Term::Dual(r)) => l.try_add(r).map(Term::Dual),
				_ => None,
			}
		}
	}

	/// Geometric multiplication:
	pub fn mul(self, rhs: Self, grammar: &Grammar) -> Self {
		if self.is_zero() || rhs.is_zero() {
			Self::zero()
		} else {
			match (self, rhs) {
				(Term::Normal(l), Term::Normal(r)) => Term::Normal(l.mul(r, grammar)),
				(Term::Dual(l), Term::Dual(r)) => Term::Dual(l.mul(r, grammar)),
				_ => panic!("Cannot multiply duals and non-duals"),
			}
		}
	}

	/// inner / dot product
	pub fn inner(self, rhs: Self, grammar: &Grammar) -> Self {
		if self.is_zero() || rhs.is_zero() {
			Self::zero()
		} else {
			match (self, rhs) {
				(Term::Normal(l), Term::Normal(r)) => Term::Normal(l.inner(r, grammar)),
				(Term::Dual(l), Term::Dual(r)) => Term::Dual(l.inner(r, grammar)),
				_ => panic!("Cannot multiply duals and non-duals"),
			}
		}
	}

	/// outer / wedge product
	pub fn outer(self, rhs: Self, grammar: &Grammar) -> Self {
		if self.is_zero() || rhs.is_zero() {
			Self::zero()
		} else {
			match (self, rhs) {
				(Term::Normal(l), Term::Normal(r)) => Term::Normal(l.outer(r, grammar)),
				(Term::Dual(l), Term::Dual(r)) => Term::Dual(l.outer(r, grammar)),
				_ => panic!("Cannot multiply duals and non-duals"),
			}
		}
	}
}

impl std::fmt::Display for Term {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Term::Normal(p) => p.fmt(f),
			Term::Dual(p) => {
				if p.is_negative() {
					format!("-!({})", -p.clone()).fmt(f)
				} else {
					format!("!({})", p).fmt(f)
				}
			}
		}
	}
}

impl std::ops::Neg for Term {
	type Output = Term;
	fn neg(self) -> Self::Output {
		match self {
			Term::Normal(p) => Term::Normal(-p),
			Term::Dual(p) => Term::Dual(-p),
		}
	}
}

// ----------------------------------------------------------------------------

/// A sum-of-products type, used in symbolic calculations.
/// Empty sum = 0
#[derive(Clone, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Sum(Vec<Term>);

impl Sum {
	pub fn one() -> Self {
		Sum(vec![Term::one()])
	}

	pub fn is_zero(&self) -> bool {
		self.0.is_empty() || self.0.iter().all(|p| p.is_zero())
	}

	pub fn from_factor(factor: Factor) -> Self {
		Sum(vec![Term::from_factor(factor)])
	}

	pub fn instance(variable: &str, typ: &Type) -> Self {
		Self(
			typ.0
				.iter()
				.map(|(member, blade)| {
					Term::from_factor(Factor::Normal(Symbol {
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
		let mut collapsed_terms: Vec<Term> = vec![];
		for new_term in self.0 {
			if let Some(last_term) = collapsed_terms.last_mut() {
				if let Some(sum) = last_term.try_add(&new_term) {
					if sum.is_zero() {
						collapsed_terms.pop();
					} else {
						*last_term = sum;
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

	pub fn mul(self, rhs: Self, grammar: &Grammar) -> Self {
		Sum(self
			.0
			.into_iter()
			.cartesian_product(rhs.0)
			.map(|(a, b)| a.mul(b, grammar))
			.collect())
		.simplify(grammar)
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
	pub fn sandwich(self, rhs: Self, grammar: &Grammar) -> Self {
		self.clone().mul(rhs, grammar).mul(self.reverse(grammar), grammar)
	}
}

impl std::fmt::Display for Sum {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		if self.is_zero() {
			"0".fmt(f)
		} else {
			write!(f, "{}", self.0[0])?;
			for term in self.0.iter().skip(1) {
				if term.is_negative() {
					write!(f, " - {}", -term.clone())?;
				} else {
					write!(f, " + {}", term)?;
				}
			}
			Ok(())
		}
	}
}

use crate::{Grammar, VecIdx};

use itertools::Itertools;

/// Represents the geometric product of some vectors in the given order.
/// The empty blade is the real/scalar dimensions.
#[derive(Clone, Eq, PartialEq)]
pub struct Blade(Vec<VecIdx>);

impl Blade {
	pub fn scalar() -> Self {
		Blade(vec![])
	}

	pub fn from_indices(indices: Vec<VecIdx>) -> Self {
		Self(indices)
	}

	pub fn indices(&self) -> &[VecIdx] {
		&self.0
	}

	// Is the given vector-index set?
	pub fn from_bools(bools: &[bool]) -> Self {
		Self(
			bools
				.iter()
				.enumerate()
				.filter_map(|(i, &set)| if set { Some(VecIdx(i)) } else { None })
				.collect(),
		)
	}

	// is the given vector index a factor of this blade?
	pub fn as_bools(&self, grammar: &Grammar) -> Vec<bool> {
		assert!(self.is_simple());
		(0..grammar.dims()).map(|i| self.0.contains(&VecIdx(i))).collect()
	}

	// Is this blade as simple (short) as possible?
	// NOTE: may still not be normalized (canonical order, as specified by grammar)
	pub fn is_simple(&self) -> bool {
		for i in 0..self.0.len() {
			for j in i + 1..self.0.len() {
				if self.0[i] == self.0[j] {
					return false; // Duplicate
				}
			}
		}
		true
	}

	/// 0 for scalar, 1 for vector, 2 for multivector etc.
	/// Only trustworthy for normalized / simplified blades!
	pub fn grade(&self) -> usize {
		assert!(self.is_simple());
		self.0.len()
	}

	/// TODO: store dimensionality in blade so we don't need to pass in the grammar here!
	pub fn dual(&self, grammar: &Grammar) -> Blade {
		// Blade(self.0.iter().map(|v| grammar.dual(*v)).collect())
		let bools: Vec<bool> = self.as_bools(grammar).into_iter().map(|s| !s).collect();
		Blade::from_bools(&bools)
	}

	/// A reverse is when you reverse the order of the vector factors in this blade.
	/// Returns the sign change that would occur in this case.
	pub fn reverse_sign(&self) -> i32 {
		let mut sign = 1;
		let r = self.grade();
		if r > 1 {
			// After reversing the order, we want to sort again.
			let num_swaps = r * (r - 1) / 2;
			if num_swaps % 2 == 1 {
				// Odd number of swaps => sign change
				sign = -sign;
			}
		}
		sign
	}

	// Simplify to sorted, collapsed form without duplicate vector indices.
	#[must_use]
	pub fn simplify(&mut self, grammar: &Grammar) -> i32 {
		self.sort() * self.collapse_adjacent(grammar)
	}

	// Sort the vector indices, keeping track of all sign changes.
	#[must_use]
	pub fn sort(&mut self) -> i32 {
		// Multiplication is anti-commutative so each time we swap we need to flip the sign.
		// So bubble-sort!
		let mut sign = 1;
		for _ in 0..self.0.len() {
			for i in 0..self.0.len() - 1 {
				if self.0[i] > self.0[i + 1] {
					self.0.swap(i, i + 1);
					sign = -sign;
				}
			}
		}
		sign
	}

	#[must_use]
	pub fn sorted(&self) -> (i32, Self) {
		let mut blade = self.clone();
		let sign = blade.sort();
		(sign, blade)
	}

	// Collapse adjacent identical vector indices using the given grammar
	#[must_use]
	pub fn collapse_adjacent(&mut self, grammar: &Grammar) -> i32 {
		let mut sign = 1;
		let mut new_bases = vec![];
		for &num in &self.0 {
			if new_bases.last() == Some(&num) {
				sign *= grammar.square(num);
				new_bases.pop();
			} else {
				new_bases.push(num);
			}
		}
		*self = Blade(new_bases);
		sign
	}
}

// TODO: use grammar for this instead of overriding Ord
impl Ord for Blade {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		if self.0.len() != other.0.len() {
			// Short blades first
			self.0.len().cmp(&other.0.len())
		} else {
			// TODO: remove this HACK for e12 vs e20 order
			let self_sorted: Vec<VecIdx> = self.0.iter().copied().sorted().collect();
			let other_sorted: Vec<VecIdx> = other.0.iter().copied().sorted().collect();
			if self_sorted != other_sorted {
				self_sorted.cmp(&other_sorted)
			} else {
				self.0.cmp(&other.0)
			}
		}
	}
}

impl PartialOrd for Blade {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl std::fmt::Debug for Blade {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		if self.0.is_empty() {
			// Real/Scalar
			"s".fmt(f)
		} else {
			format!("e{}", self.0.iter().join("")).fmt(f)
		}
	}
}

impl std::fmt::Display for Blade {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		if self.0.is_empty() {
			// Real/Scalar
			"s".fmt(f)
		} else {
			format!("e{}", self.0.iter().join("")).fmt(f)
		}
	}
}

/// geometric multiplication, produces the geometric product
impl std::ops::Mul<&Blade> for &Blade {
	type Output = Blade;
	fn mul(self, rhs: &Blade) -> Self::Output {
		// each blade is the product of its vectors so all we need to do is concatenate the numbers:
		Blade(self.0.iter().copied().chain(rhs.0.iter().copied()).collect())
	}
}

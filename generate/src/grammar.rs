use std::collections::BTreeMap;

use itertools::Itertools;

use crate::{Blade, SignedBlade, VecIdx};

pub struct Grammar {
	/// what you get when you square the input vectors,
	/// e.g. [0, 1, 1] would specify the 2d gpa of e0^2=0  e1^2=1  e2^2=1
	vectors_squared: Vec<i32>,

	/// Optionally override the order of the vector bases in a multivector,
	/// e.g. maybe you prefer the output to use `e20` over `-e02`.
	blade_conventions: BTreeMap<Blade, SignedBlade>,
}

impl Grammar {
	/// What you get when you square the input vectors,
	/// e.g. 0++ would specify the 2d gpa of e0^2=0  e1^2=1  e2^2=1
	/// The standard form is the integers (p, m, z)
	/// where p : number of vectors that square to +1
	/// where m : number of vectors that square to -1
	/// where z : number of vectors that square to 0
	/// We allow others orders.
	/// (3, 0, 0): 3d euclidean vector space
	/// (3, 0, 1): 3d projective geometric algebra
	pub fn from_vectors_squared(vectors_squared: Vec<i32>) -> Self {
		Self {
			vectors_squared,
			blade_conventions: Default::default(),
		}
	}

	/// Set preference of vector order in a blade.
	/// For instance, you can decide to use `e20` instead of the default `e02`
	pub fn set_blade_convention(&mut self, convention: Blade) {
		for permutation in convention
			.indices()
			.iter()
			.copied()
			.permutations(convention.indices().len())
			.map(Blade::from_indices)
		{
			let (canonical_sign, canonical_sorted) = convention.sorted();
			let (permutation_sign, permutation_sorted) = permutation.sorted();
			assert_eq!(canonical_sorted, permutation_sorted);

			self.blade_conventions.insert(
				permutation,
				SignedBlade {
					sign: canonical_sign * permutation_sign,
					blade: convention.clone(),
				},
			);
		}
	}

	/// Projective Geometric Algebra in 2d.
	/// e0^2=0  e1^2=1  e2^2=1
	pub fn pga_2d() -> Self {
		let mut g = Self::from_vectors_squared(vec![0, 1, 1]);
		g.set_blade_convention("e20".parse().unwrap()); // TODO: fix e1.dual()???
		g
	}
	/// Projective Geometric Algebra in 3d.
	/// e0^2=0  e1^2=1  e2^2=1 e3^3=1
	pub fn pga_3d() -> Self {
		let mut g = Self::from_vectors_squared(vec![0, 1, 1, 1]);
		g.set_blade_convention("e31".parse().unwrap());
		g.set_blade_convention("e021".parse().unwrap());
		g.set_blade_convention("e032".parse().unwrap());
		g
	}

	/// number of vectors, dimensionality of the vector space
	pub fn dims(&self) -> usize {
		self.vectors_squared.len()
	}

	pub fn dual(&self, v: VecIdx) -> VecIdx {
		assert!(v.0 < self.dims());
		VecIdx(self.dims() - v.0 - 1)
	}

	pub fn square(&self, v: VecIdx) -> i32 {
		self.vectors_squared[v.0]
	}

	pub fn simplify(&self, mut value: SignedBlade) -> SignedBlade {
		value.sign *= value.blade.simplify(self);
		if let Some(preferred) = self.blade_conventions.get(&value.blade) {
			value.sign * preferred.clone()
		} else {
			value
		}
	}

	/// What is the sign change from (a * b) to (b * a) ?
	pub fn commute_sign(&self, a: &Blade, b: &Blade) -> i32 {
		(a * b).simplify(self) * (b * a).simplify(self)
	}
}

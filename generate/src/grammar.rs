use std::collections::BTreeMap;

use itertools::Itertools;

use crate::{Blade, Sign, SignedBlade, VecIdx};

pub struct GrammarBuilder {
	/// what you get when you sign the input vectors,
	/// e.g. 0++ would specify the 2d gpa of e0^2=0  e1^2=1  e2^2=1
	pub vectors_squared: Vec<Sign>,

	/// Optionally specify preferred order of the vector bases in a multivector,
	/// e.g. maybe you prefer to use `e20` as a base rather than `e02`.
	blade_version: Vec<Blade>,
	// TODO: allow changing the order (in multiplication tables, types etc) of e.e. `e20` and `e12`?
}

pub struct Grammar {
	/// what you get when you sign the input vectors,
	/// e.g. 0++ would specify the 2d gpa of e0^2=0  e1^2=1  e2^2=1
	vectors_squared: Vec<Sign>,

	/// Optionally override the order of the vector bases in a multivector,
	/// e.g. maybe you prefer the output to use `e20` over `-e02`.
	blade_version: BTreeMap<Blade, SignedBlade>,
}

impl GrammarBuilder {
	/// Projective Geometric Algebra in 2d.
	/// e0^2=0  e1^2=1  e2^2=1
	pub fn pga_2d() -> Self {
		Self {
			vectors_squared: vec![Sign::Zero, Sign::Positive, Sign::Positive],
			blade_version: vec![Blade::from_indices(vec![VecIdx(2), VecIdx(0)])],
			// TODO:
			// line:      {e0,  e1,  e2},
			// point:     {e01, e20, e12},
			// transform: {s, e01, e20, e12},
		}
	}

	pub fn build(self) -> Grammar {
		let mut blade_version = BTreeMap::new();
		for canonical in self.blade_version {
			for permutation in canonical
				.indices()
				.iter()
				.copied()
				.permutations(canonical.indices().len())
				.map(Blade::from_indices)
			{
				let (canonical_sign, canonical_sorted) = canonical.sorted();
				let (permutation_sign, permutation_sorted) = permutation.sorted();
				assert_eq!(canonical_sorted, permutation_sorted);

				if permutation != canonical {
					blade_version.insert(
						permutation,
						SignedBlade {
							sign: canonical_sign * permutation_sign,
							blade: canonical.clone(),
						},
					);
				}
			}
		}

		Grammar {
			vectors_squared: self.vectors_squared,
			blade_version,
		}
	}
}

impl Grammar {
	/// number of vectors, dimensionality of the vector space
	pub fn dims(&self) -> usize {
		self.vectors_squared.len()
	}

	pub fn dual(&self, v: VecIdx) -> VecIdx {
		assert!(v.0 < self.dims());
		VecIdx(self.dims() - v.0 - 1)
	}

	pub fn square(&self, v: VecIdx) -> Sign {
		self.vectors_squared[v.0]
	}

	pub fn simplify(&self, mut value: SignedBlade) -> SignedBlade {
		value.sign *= value.blade.simplify(self);
		if let Some(preferred) = self.blade_version.get(&value.blade) {
			value.sign * preferred.clone()
		} else {
			value
		}
	}

	/// What is the sign change from (a * b) to (b * a) ?
	pub fn commute_sign(&self, a: &Blade, b: &Blade) -> Sign {
		(a * b).simplify(self) * (b * a).simplify(self)
	}
}

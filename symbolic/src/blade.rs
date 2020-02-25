use crate::*;

/// Blade(vec![])     = scalar
/// Blade(vec![0])    = e0
/// Blade(vec![0, 2]) = e02
/// Always sorted, always unique.
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Blade(Vec<VecIdx>);

impl Blade {
	pub fn scalar() -> Self {
		Blade(vec![])
	}

	pub fn vec(vi: VecIdx) -> Self {
		Self(vec![vi])
	}

	pub fn from_sorted(vecs: Vec<VecIdx>) -> Self {
		assert!(!has_adjacent_copies(&vecs));
		Self(vecs)
	}

	pub fn is_scalar(&self) -> bool {
		self.0.is_empty()
	}

	pub fn has_vec(&self, needle: VecIdx) -> bool {
		self.0.iter().any(|&vi| vi == needle)
	}

	/// 0 for scalar, 1 for vector, 2 for multivector etc.
	pub fn grade(&self) -> usize {
		self.0.len()
	}

	pub fn vecs(&self) -> &[VecIdx] {
		&self.0
	}

	/// Left compliment.
	/// self.lcompl() * self == pseudo-scalar
	pub fn lcompl(&self, g: &Grammar) -> SBlade {
		let compliment: Vec<VecIdx> = g.vecs().filter(|&vi| !self.has_vec(vi)).collect();
		let mut all_vecs = compliment.clone();
		all_vecs.append(&mut self.0.clone());
		let scaled_pseudoscalar = SBlade::from_unsorted(&all_vecs);
		SBlade {
			sign: scaled_pseudoscalar.sign,
			blade: Self::from_sorted(compliment),
		}
	}

	/// Right compliment.
	/// self * self.rcompl() == pseudo-scalar
	/// e0 * e0.rcompl() = e0 * e12 = e012
	/// e1.rcompl() = e20 = -e02
	pub fn rcompl(&self, g: &Grammar) -> SBlade {
		let compliment: Vec<VecIdx> = g.vecs().filter(|&vi| !self.has_vec(vi)).collect();
		let mut all_vecs = self.0.clone();
		all_vecs.append(&mut compliment.clone());
		let scaled_pseudoscalar = SBlade::from_unsorted(&all_vecs);
		SBlade {
			sign: scaled_pseudoscalar.sign,
			blade: Self::from_sorted(compliment),
		}
	}
}

impl std::ops::Index<usize> for Blade {
	type Output = VecIdx;

	fn index(&self, idx: usize) -> &Self::Output {
		&self.0[idx]
	}
}

fn has_adjacent_copies(b: &[VecIdx]) -> bool {
	for i in 1..b.len() {
		if b[i - 1] == b[i] {
			return true;
		}
	}
	false
}

impl std::fmt::Debug for Blade {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		use itertools::Itertools;
		if self.0.is_empty() {
			// Real/Scalar
			"s".fmt(f)
		} else {
			format!("e{}", self.0.iter().join("")).fmt(f)
		}
	}
}

use crate::*;

impl Blade {
	pub fn scalar() -> Self {
		Blade(vec![])
	}

	pub fn vec(vi: VecIdx) -> Self {
		Self(vec![vi])
	}

	pub fn from_unsorted(vecs: &[VecIdx]) -> (i32, Self) {
		let (sign, vecs) = sort_blade(vecs.to_vec());
		(sign, Self::from_sorted(vecs))
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
	pub fn lcompl(&self, g: &Grammar) -> (i32, Blade) {
		let compliment: Vec<VecIdx> = g.vecs().filter(|&vi| !self.has_vec(vi)).collect();
		let mut all_vecs = compliment.clone();
		all_vecs.append(&mut self.0.clone());
		let (sign, _pseudoscalar) = Self::from_unsorted(&all_vecs);
		(sign, Self::from_sorted(compliment))
	}

	/// Right compliment.
	/// self * self.rcompl() == pseudo-scalar
	/// e0 * e0.rcompl() = e0 * e12 = e012
	/// e1.rcompl() = e20 = -e02
	pub fn rcompl(&self, g: &Grammar) -> (i32, Blade) {
		let compliment: Vec<VecIdx> = g.vecs().filter(|&vi| !self.has_vec(vi)).collect();
		let mut all_vecs = self.0.clone();
		all_vecs.append(&mut compliment.clone());
		let (sign, _pseudoscalar) = Self::from_unsorted(&all_vecs);
		(sign, Self::from_sorted(compliment))
	}
}

impl std::ops::Index<usize> for Blade {
	type Output = VecIdx;

	fn index(&self, idx: usize) -> &Self::Output {
		&self.0[idx]
	}
}

/// Sort the vector indices, keeping track of all sign changes.
#[must_use]
fn sort_blade(mut b: Vec<VecIdx>) -> (i32, Vec<VecIdx>) {
	// Multiplication is anti-commutative so each time we swap we need to flip the sign.
	// So bubble-sort!
	let mut sign = 1;
	for _ in 0..b.len() {
		for i in 1..b.len() {
			if b[i - 1] > b[i] {
				b.swap(i - 1, i);
				sign = -sign;
			}
		}
	}
	(sign, b)
}

fn has_adjacent_copies(b: &[VecIdx]) -> bool {
	for i in 1..b.len() {
		if b[i - 1] == b[i] {
			return true;
		}
	}
	false
}

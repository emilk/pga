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

	/// 0 for scalar, 1 for vector, 2 for multivector etc.
	pub fn grade(&self) -> usize {
		self.0.len()
	}

	pub fn vecs(&self) -> &[VecIdx] {
		&self.0
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

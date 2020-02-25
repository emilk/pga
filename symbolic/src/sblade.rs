use crate::*;

/// A blade type with a sign,
/// this is useful so we can express e20 = -e02.
/// Can be both a type (-e02) and a value (42 * e123)
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct SBlade {
	pub blade: Blade,
	pub sign: i32,
}

impl SBlade {
	pub fn zero() -> Self {
		SBlade {
			sign: 0,
			blade: Blade::scalar(), // could be any blade really
		}
	}

	pub fn scalar() -> Self {
		SBlade::unit(Blade::scalar())
	}

	pub fn vec(vi: VecIdx) -> Self {
		SBlade::unit(Blade::vec(vi))
	}

	pub fn from_unsorted(vecs: &[VecIdx]) -> SBlade {
		let (sign, vecs) = sort_blade(vecs.to_vec());
		SBlade {
			sign,
			blade: Blade::from_sorted(vecs),
		}
	}

	/// One times the given blade
	pub fn unit(blade: Blade) -> Self {
		Self { sign: 1, blade }
	}

	pub fn is_zero(&self) -> bool {
		self.sign == 0
	}

	pub fn is_negative(&self) -> bool {
		self.sign < 0
	}

	pub fn is_scalar(&self) -> bool {
		self.blade.is_scalar()
	}

	pub fn grade(&self) -> usize {
		self.blade.grade()
	}

	/// Left compliment.
	/// self.lcompl() * self == pseudo-scalar
	pub fn lcompl(&self, g: &Grammar) -> SBlade {
		self.sign * self.blade.lcompl(g)
	}

	/// Right compliment.
	/// self * self.rcompl() == pseudo-scalar
	/// e0 * e0.rcompl() = e0 * e12 = e012
	/// e1.rcompl() = e20 = -e02
	pub fn rcompl(&self, g: &Grammar) -> SBlade {
		self.sign * self.blade.rcompl(g)
	}

	// /// geometric product (normal multiplication)
	// pub fn geometric(&self, other: &SBlade, grammar: &Grammar) -> Self {
	// 	grammar.simplify(self * other)
	// }

	// /// inner / dot
	// pub fn inner(&self, other: &SBlade, grammar: &Grammar) -> Self {
	// 	// The dot product is the K grade of the geometric product,
	// 	// where K is the absolute difference in grades between the operands.
	// 	let k = ((self.grade() as i64) - (other.grade() as i64)).abs() as usize;
	// 	let prod = self.geometric(other, grammar);
	// 	if prod.blade.grade() > k {
	// 		Self::zero()
	// 	} else {
	// 		prod
	// 	}
	// }

	// /// outer / wedge
	// pub fn outer(&self, other: &SBlade, grammar: &Grammar) -> Self {
	// 	let k = self.grade() + other.grade();
	// 	let prod = self.geometric(other, grammar);
	// 	if prod.blade.grade() < k {
	// 		Self::zero()
	// 	} else {
	// 		prod
	// 	}
	// }

	// pub fn regressive(&self, other: &SBlade, grammar: &Grammar) -> Self {
	// 	self.dual(grammar).outer(&other.dual(grammar), grammar).dual(grammar)
	// }
}

impl std::ops::Mul<SBlade> for i32 {
	type Output = SBlade;
	fn mul(self, right: SBlade) -> Self::Output {
		SBlade {
			sign: self * right.sign,
			blade: right.blade,
		}
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

impl std::fmt::Debug for SBlade {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self.sign {
			1 => format!(" {:?}", self.blade).fmt(f),
			0 => " 0".fmt(f),
			-1 => format!("-{:?}", self.blade).fmt(f),
			sign => format!("{}*{:?}", sign, self.blade).fmt(f),
		}
	}
}

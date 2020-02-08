/// Definitions of axes: each axis squares to one of these.
/// The number of axes and their signs define your algebra.
/// TODO: replace with i32 ?
#[derive(Copy, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum Sign {
	Negative,
	Zero,
	Positive,
}

impl Sign {
	pub fn into_i32(self) -> i32 {
		match self {
			Sign::Negative => -1,
			Sign::Zero => 0,
			Sign::Positive => 1,
		}
	}
}

impl std::fmt::Debug for Sign {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Sign::Negative => "-".fmt(f),
			Sign::Zero => "0".fmt(f),
			Sign::Positive => "+".fmt(f),
		}
	}
}

impl std::fmt::Display for Sign {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Sign::Negative => "-".fmt(f),
			Sign::Zero => "0".fmt(f),
			Sign::Positive => "+".fmt(f),
		}
	}
}

impl std::ops::Neg for Sign {
	type Output = Sign;
	fn neg(self) -> Sign {
		match self {
			Sign::Negative => Sign::Positive,
			Sign::Zero => Sign::Zero,
			Sign::Positive => Sign::Negative,
		}
	}
}

impl std::ops::Mul for Sign {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self {
		use Sign::*;
		match (self, rhs) {
			(Positive, Positive) | (Negative, Negative) => Positive,
			(Negative, Positive) | (Positive, Negative) => Negative,
			_ => Zero,
		}
	}
}

impl std::ops::MulAssign<Sign> for Sign {
	fn mul_assign(&mut self, rhs: Sign) {
		*self = *self * rhs;
	}
}

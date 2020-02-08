/// Definitions of axes: each axis squares to one of these.
/// The number of axes and their signs define your algebra.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Sign {
	Negative,
	Zero,
	Positive,
}

impl std::fmt::Display for Sign {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Sign::Negative => "-1".fmt(f),
			Sign::Zero => "0".fmt(f),
			Sign::Positive => "+1".fmt(f),
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

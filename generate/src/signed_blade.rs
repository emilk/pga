use crate::*;

// TODO: rename
// A blade dimension with magnitude (e.g 42 * e2)
#[derive(Clone)]
pub struct SignedBlade {
	pub sign: i32,
	pub blade: Blade,
}

impl SignedBlade {
	pub fn zero() -> Self {
		SignedBlade {
			sign: 0,
			blade: Blade::scalar(), // could be any blade really
		}
	}

	/// One times the given blade
	pub fn unit(blade: &Blade) -> Self {
		Self {
			sign: 1,
			blade: blade.clone(),
		}
	}

	pub fn simplify(self, grammar: &Grammar) -> Self {
		grammar.simplify(self)
	}

	pub fn is_zero(&self) -> bool {
		self.sign == 0
	}

	pub fn is_negative(&self) -> bool {
		self.sign < 0
	}

	pub fn grade(&self) -> usize {
		self.blade.grade()
	}

	pub fn dual(&self, grammar: &Grammar) -> SignedBlade {
		grammar.simplify(Self {
			sign: self.sign,
			blade: self.blade.dual(grammar),
		})
	}

	/// Reverse the order of the vector factors in this blade
	pub fn reverse(&self) -> Self {
		let mut b = self.clone();
		b.sign *= b.blade.reverse_sign();
		b
	}

	/// geometric product (normal multiplication)
	pub fn geometric(&self, other: &SignedBlade, grammar: &Grammar) -> Self {
		grammar.simplify(self * other)
	}

	/// inner / dot
	pub fn inner(&self, other: &SignedBlade, grammar: &Grammar) -> Self {
		// The dot product is the K grade of the geometric product,
		// where K is the absolute difference in grades between the operands.
		let k = ((self.grade() as i64) - (other.grade() as i64)).abs() as usize;
		let prod = self.geometric(other, grammar);
		if prod.blade.grade() > k {
			Self::zero()
		} else {
			prod
		}
	}

	/// outer / wedge
	pub fn outer(&self, other: &SignedBlade, grammar: &Grammar) -> Self {
		let k = self.grade() + other.grade();
		let prod = self.geometric(other, grammar);
		if prod.blade.grade() < k {
			Self::zero()
		} else {
			prod
		}
	}

	pub fn regressive(&self, other: &SignedBlade, grammar: &Grammar) -> Self {
		self.dual(grammar).outer(&other.dual(grammar), grammar).dual(grammar)
	}
}

impl std::fmt::Debug for SignedBlade {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self.sign {
			1 => format!(" {}", self.blade).fmt(f),
			0 => " 0".fmt(f),
			-1 => format!("-{}", self.blade).fmt(f),
			sign => format!("{}*{}", sign, self.blade).fmt(f),
		}
	}
}

impl std::fmt::Display for SignedBlade {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self.sign {
			1 => format!(" {}", self.blade).fmt(f),
			0 => " 0".fmt(f),
			-1 => format!("-{}", self.blade).fmt(f),
			sign => format!("{}*{}", sign, self.blade).fmt(f),
		}
	}
}

impl std::ops::Mul<SignedBlade> for SignedBlade {
	type Output = SignedBlade;
	fn mul(self, rhs: SignedBlade) -> Self::Output {
		SignedBlade {
			sign: self.sign * rhs.sign,
			blade: &self.blade * &rhs.blade,
		}
	}
}

impl std::ops::Mul<&SignedBlade> for &SignedBlade {
	type Output = SignedBlade;
	fn mul(self, rhs: &SignedBlade) -> Self::Output {
		SignedBlade {
			sign: self.sign * rhs.sign,
			blade: &self.blade * &rhs.blade,
		}
	}
}

impl std::ops::Mul<SignedBlade> for i32 {
	type Output = SignedBlade;
	#[inline(always)]
	fn mul(self, right: SignedBlade) -> Self::Output {
		SignedBlade {
			sign: self * right.sign,
			blade: right.blade,
		}
	}
}

impl std::ops::MulAssign<SignedBlade> for SignedBlade {
	fn mul_assign(&mut self, rhs: SignedBlade) {
		*self = self.clone() * rhs;
	}
}

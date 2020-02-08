use crate::*;

// TODO: rename
// A blade with a sign (e.g 42 * e2)
#[derive(Clone)]
pub struct SignedBlade {
	pub sign: i32,
	pub blade: Blade,
}

impl SignedBlade {
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

	pub fn grade(&self) -> usize {
		self.blade.grade()
	}

	pub fn dual(&self, grammar: &Grammar) -> SignedBlade {
		Self {
			sign: self.sign,
			blade: self.blade.dual(grammar),
		}
	}

	/// Reverse the order of the vector factors in this blade
	pub fn reverse(&self) -> Self {
		let mut b = self.clone();
		let r = b.grade();
		if r > 1 {
			// After reversing the order, we want to sort again.
			let num_swaps = r * (r - 1) / 2;
			if num_swaps % 2 == 1 {
				// Odd number of swaps => sign change
				b.sign = -b.sign;
			}
		}
		b
	}

	/// geometric product (normal multiplication)
	pub fn geometric(&self, other: &SignedBlade, grammar: &Grammar) -> Self {
		grammar.simplify(self * other)
	}

	pub fn dot(&self, other: &SignedBlade, grammar: &Grammar) -> Self {
		// The dot product is the K grade of the geometric product,
		// where K is the absolute difference in grades between the operands.
		let k = ((self.grade() as i64) - (other.grade() as i64)).abs() as usize;
		let mut prod = self.geometric(other, grammar);
		if prod.blade.grade() > k {
			prod.sign = 0;
		}
		prod
	}

	/// outer / wedge
	pub fn outer(&self, other: &SignedBlade, grammar: &Grammar) -> Self {
		let k = self.grade() + other.grade();
		let mut prod = self.geometric(other, grammar);
		if prod.blade.grade() < k {
			prod.sign = 0;
		}
		prod
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

use itertools::chain;

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

	pub fn one() -> Self {
		SBlade {
			sign: 1,
			blade: Blade::scalar(),
		}
	}

	pub fn scalar() -> Self {
		SBlade::unit(Blade::scalar())
	}

	pub fn vec(vi: VecIdx) -> Self {
		SBlade::unit(Blade::vec(vi))
	}

	pub fn from_unsorted(vecs: &[VecIdx]) -> SBlade {
		let (sign, vecs) = sort_vecs(vecs.to_vec());
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

	/// geometric product (normal multiplication)
	pub fn geometric_product(&self, other: &SBlade, g: &Grammar) -> Self {
		let mut sign = self.sign * other.sign;
		let mut vecs: Vec<VecIdx> = chain(self.blade.vecs(), other.blade.vecs()).copied().collect();
		sign *= sort_vecs_inplace(&mut vecs);
		sign *= collapse_adjacent(&mut vecs, g);
		if sign == 0 {
			Self::zero()
		} else {
			SBlade {
				sign,
				blade: Blade::from_sorted(vecs),
			}
		}
	}

	pub fn inner_product(&self, other: &SBlade, g: &Grammar) -> Self {
		// The dot product is the K grade of the geometric product,
		// where K is the absolute difference in grades between the operands.
		let k = ((self.grade() as i64) - (other.grade() as i64)).abs() as usize;
		let prod = self.geometric_product(other, g);
		if prod.blade.grade() > k {
			Self::zero()
		} else {
			prod
		}
	}

	/// outer / wedge
	pub fn wedge_product(&self, other: &SBlade, g: &Grammar) -> Self {
		let k = self.grade() + other.grade();
		let prod = self.geometric_product(other, g);
		if prod.blade.grade() < k {
			Self::zero()
		} else {
			prod
		}
	}

	/// regressive product
	pub fn antiwedge_product(&self, other: &SBlade, g: &Grammar) -> Self {
		self.lcompl(g).wedge_product(&other.lcompl(g), g).rcompl(g)
	}

	pub fn binary_product(a: &SBlade, product: Product, b: &SBlade, g: &Grammar) -> SBlade {
		match product {
			Product::Geometric => a.geometric_product(b, g),
			Product::Wedge => a.wedge_product(b, g),
			Product::Antiwedge => a.antiwedge_product(b, g),
		}
	}

	pub fn product(product: Product, operands: &[SBlade], g: &Grammar) -> SBlade {
		if operands.is_empty() {
			SBlade::one()
		} else {
			let mut result = operands[0].clone();
			for operand in operands.iter().skip(1) {
				result = Self::binary_product(&result, product, operand, g);
			}
			result
		}
	}
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
fn sort_vecs(mut vecs: Vec<VecIdx>) -> (i32, Vec<VecIdx>) {
	let sign = sort_vecs_inplace(&mut vecs);
	(sign, vecs)
}

#[must_use]
fn sort_vecs_inplace(vecs: &mut Vec<VecIdx>) -> i32 {
	// Multiplication is anti-commutative so each time we swap we need to flip the sign.
	// So bubble-sort!
	let mut sign = 1;
	for _ in 0..vecs.len() {
		for i in 1..vecs.len() {
			if vecs[i - 1] > vecs[i] {
				vecs.swap(i - 1, i);
				sign = -sign;
			}
		}
	}
	sign
}

// Collapse adjacent identical vector indices using the given grammar
#[must_use]
pub fn collapse_adjacent(vecs: &mut Vec<VecIdx>, g: &Grammar) -> i32 {
	let mut sign = 1;
	let mut new_bases = vec![];
	for vi in vecs.iter() {
		if new_bases.last() == Some(vi) {
			sign *= g.square(*vi);
			new_bases.pop();
		} else {
			new_bases.push(*vi);
		}
	}
	*vecs = new_bases;
	sign
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

// Written by a generator written by enki.
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::fmt;
use std::ops::{Add, BitAnd, BitOr, BitXor, Index, IndexMut, Mul, Not, Sub};

type float_t = f64;

// use std::f64::consts::PI;
const PI: float_t = 3.14159265358979323846;

const basis: &'static [&'static str] = &["1", "e0", "e1", "e2", "e01", "e20", "e12", "e012"];
const basis_count: usize = basis.len();

#[derive(Default, Clone, Copy, PartialEq)]
struct PGA2D {
	mvec: [float_t; basis_count],
}

impl PGA2D {
	pub const fn zero() -> Self {
		Self {
			mvec: [0.0; basis_count],
		}
	}

	pub const fn new(f: float_t, idx: usize) -> Self {
		let mut ret = Self::zero();
		ret.mvec[idx] = f;
		ret
	}
}

// basis vectors are available as global constants.
const e0: PGA2D = PGA2D::new(1.0, 1);
const e1: PGA2D = PGA2D::new(1.0, 2);
const e2: PGA2D = PGA2D::new(1.0, 3);
const e01: PGA2D = PGA2D::new(1.0, 4);
const e20: PGA2D = PGA2D::new(1.0, 5);
const e12: PGA2D = PGA2D::new(1.0, 6);
const e012: PGA2D = PGA2D::new(1.0, 7);

impl Index<usize> for PGA2D {
	type Output = float_t;

	fn index<'a>(&'a self, index: usize) -> &'a Self::Output {
		&self.mvec[index]
	}
}

impl IndexMut<usize> for PGA2D {
	fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut Self::Output {
		&mut self.mvec[index]
	}
}

impl fmt::Debug for PGA2D {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut n = 0;
		let ret = self
			.mvec
			.iter()
			.enumerate()
			.filter_map(|(i, &coeff)| {
				if coeff > 0.00001 || coeff < -0.00001 {
					n = 1;
					Some(format!(
						"{}{}",
						format!("{:.*}", 7, coeff).trim_end_matches('0').trim_end_matches('.'),
						if i > 0 { basis[i] } else { "" }
					))
				} else {
					None
				}
			})
			.collect::<Vec<String>>()
			.join(" + ");
		if n == 0 {
			write!(f, "0")
		} else {
			write!(f, "{}", ret)
		}
	}
}

impl fmt::Display for PGA2D {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut n = 0;
		let ret = self
			.mvec
			.iter()
			.enumerate()
			.filter_map(|(i, &coeff)| {
				if coeff > 0.00001 || coeff < -0.00001 {
					n = 1;
					Some(format!(
						"{}{}",
						format!("{:.*}", 7, coeff).trim_end_matches('0').trim_end_matches('.'),
						if i > 0 { basis[i] } else { "" }
					))
				} else {
					None
				}
			})
			.collect::<Vec<String>>()
			.join(" + ");
		if n == 0 {
			write!(f, "0")
		} else {
			write!(f, "{}", ret)
		}
	}
}

// Reverse
// Reverse the order of the basis blades.
impl PGA2D {
	pub fn Reverse(self: Self) -> PGA2D {
		let mut res = PGA2D::zero();
		let a = self;
		res[0] = a[0];
		res[1] = a[1];
		res[2] = a[2];
		res[3] = a[3];
		res[4] = -a[4];
		res[5] = -a[5];
		res[6] = -a[6];
		res[7] = -a[7];
		res
	}
}

// Dual
// Poincare duality operator.
impl PGA2D {
	pub fn Dual(self: Self) -> PGA2D {
		let mut res = PGA2D::zero();
		let a = self;
		res[0] = a[7];
		res[1] = a[6];
		res[2] = a[5];
		res[3] = a[4];
		res[4] = a[3];
		res[5] = a[2];
		res[6] = a[1];
		res[7] = a[0];
		res
	}
}

impl Not for PGA2D {
	type Output = PGA2D;

	fn not(self: Self) -> PGA2D {
		let mut res = PGA2D::zero();
		let a = self;
		res[0] = a[7];
		res[1] = a[6];
		res[2] = a[5];
		res[3] = a[4];
		res[4] = a[3];
		res[5] = a[2];
		res[6] = a[1];
		res[7] = a[0];
		res
	}
}

// Conjugate
// Clifford Conjugation
impl PGA2D {
	pub fn Conjugate(self: Self) -> PGA2D {
		let mut res = PGA2D::zero();
		let a = self;
		res[0] = a[0];
		res[1] = -a[1];
		res[2] = -a[2];
		res[3] = -a[3];
		res[4] = -a[4];
		res[5] = -a[5];
		res[6] = -a[6];
		res[7] = a[7];
		res
	}
}

// Involute
// Main involution
impl PGA2D {
	pub fn Involute(self: Self) -> PGA2D {
		let mut res = PGA2D::zero();
		let a = self;
		res[0] = a[0];
		res[1] = -a[1];
		res[2] = -a[2];
		res[3] = -a[3];
		res[4] = a[4];
		res[5] = a[5];
		res[6] = a[6];
		res[7] = -a[7];
		res
	}
}

// Mul
// The geometric product.
impl Mul for PGA2D {
	type Output = PGA2D;

	fn mul(self: PGA2D, b: PGA2D) -> PGA2D {
		let mut res = PGA2D::zero();
		let a = self;
		res[0] = b[0] * a[0] + b[2] * a[2] + b[3] * a[3] - b[6] * a[6];
		res[1] = b[1] * a[0] + b[0] * a[1] - b[4] * a[2] + b[5] * a[3] + b[2] * a[4]
			- b[3] * a[5]
			- b[7] * a[6]
			- b[6] * a[7];
		res[2] = b[2] * a[0] + b[0] * a[2] - b[6] * a[3] + b[3] * a[6];
		res[3] = b[3] * a[0] + b[6] * a[2] + b[0] * a[3] - b[2] * a[6];
		res[4] = b[4] * a[0] + b[2] * a[1] - b[1] * a[2] + b[7] * a[3] + b[0] * a[4] + b[6] * a[5] - b[5] * a[6]
			+ b[3] * a[7];
		res[5] = b[5] * a[0] - b[3] * a[1] + b[7] * a[2] + b[1] * a[3] - b[6] * a[4]
			+ b[0] * a[5]
			+ b[4] * a[6]
			+ b[2] * a[7];
		res[6] = b[6] * a[0] + b[3] * a[2] - b[2] * a[3] + b[0] * a[6];
		res[7] =
			b[7] * a[0]
				+ b[6] * a[1] + b[5] * a[2]
				+ b[4] * a[3] + b[3] * a[4]
				+ b[2] * a[5] + b[1] * a[6]
				+ b[0] * a[7];
		res
	}
}

// Wedge
// The outer product. (MEET)
impl BitXor for PGA2D {
	type Output = PGA2D;

	fn bitxor(self: PGA2D, b: PGA2D) -> PGA2D {
		let mut res = PGA2D::zero();
		let a = self;
		res[0] = b[0] * a[0];
		res[1] = b[1] * a[0] + b[0] * a[1];
		res[2] = b[2] * a[0] + b[0] * a[2];
		res[3] = b[3] * a[0] + b[0] * a[3];
		res[4] = b[4] * a[0] + b[2] * a[1] - b[1] * a[2] + b[0] * a[4];
		res[5] = b[5] * a[0] - b[3] * a[1] + b[1] * a[3] + b[0] * a[5];
		res[6] = b[6] * a[0] + b[3] * a[2] - b[2] * a[3] + b[0] * a[6];
		res[7] =
			b[7] * a[0]
				+ b[6] * a[1] + b[5] * a[2]
				+ b[4] * a[3] + b[3] * a[4]
				+ b[2] * a[5] + b[1] * a[6]
				+ b[0] * a[7];
		res
	}
}

// Vee
// The regressive product. (JOIN)
impl BitAnd for PGA2D {
	type Output = PGA2D;

	fn bitand(self: PGA2D, b: PGA2D) -> PGA2D {
		let mut res = PGA2D::zero();
		let a = self;
		res[7] = b[7] * a[7];
		res[6] = b[6] * a[7] + b[7] * a[6];
		res[5] = b[5] * a[7] + b[7] * a[5];
		res[4] = b[4] * a[7] + b[7] * a[4];
		res[3] = b[3] * a[7] + b[5] * a[6] - b[6] * a[5] + b[7] * a[3];
		res[2] = b[2] * a[7] - b[4] * a[6] + b[6] * a[4] + b[7] * a[2];
		res[1] = b[1] * a[7] + b[4] * a[5] - b[5] * a[4] + b[7] * a[1];
		res[0] =
			b[0] * a[7]
				+ b[1] * a[6] + b[2] * a[5]
				+ b[3] * a[4] + b[4] * a[3]
				+ b[5] * a[2] + b[6] * a[1]
				+ b[7] * a[0];
		res
	}
}

// Dot
// The inner product.
impl BitOr for PGA2D {
	type Output = PGA2D;

	fn bitor(self: PGA2D, b: PGA2D) -> PGA2D {
		let mut res = PGA2D::zero();
		let a = self;
		res[0] = b[0] * a[0] + b[2] * a[2] + b[3] * a[3] - b[6] * a[6];
		res[1] = b[1] * a[0] + b[0] * a[1] - b[4] * a[2] + b[5] * a[3] + b[2] * a[4]
			- b[3] * a[5]
			- b[7] * a[6]
			- b[6] * a[7];
		res[2] = b[2] * a[0] + b[0] * a[2] - b[6] * a[3] + b[3] * a[6];
		res[3] = b[3] * a[0] + b[6] * a[2] + b[0] * a[3] - b[2] * a[6];
		res[4] = b[4] * a[0] + b[7] * a[3] + b[0] * a[4] + b[3] * a[7];
		res[5] = b[5] * a[0] + b[7] * a[2] + b[0] * a[5] + b[2] * a[7];
		res[6] = b[6] * a[0] + b[0] * a[6];
		res[7] = b[7] * a[0] + b[0] * a[7];
		res
	}
}

// Add
// Multivector addition
impl Add for PGA2D {
	type Output = PGA2D;

	fn add(self: PGA2D, b: PGA2D) -> PGA2D {
		let mut res = PGA2D::zero();
		let a = self;
		res[0] = a[0] + b[0];
		res[1] = a[1] + b[1];
		res[2] = a[2] + b[2];
		res[3] = a[3] + b[3];
		res[4] = a[4] + b[4];
		res[5] = a[5] + b[5];
		res[6] = a[6] + b[6];
		res[7] = a[7] + b[7];
		res
	}
}

// Sub
// Multivector subtraction
impl Sub for PGA2D {
	type Output = PGA2D;

	fn sub(self: PGA2D, b: PGA2D) -> PGA2D {
		let mut res = PGA2D::zero();
		let a = self;
		res[0] = a[0] - b[0];
		res[1] = a[1] - b[1];
		res[2] = a[2] - b[2];
		res[3] = a[3] - b[3];
		res[4] = a[4] - b[4];
		res[5] = a[5] - b[5];
		res[6] = a[6] - b[6];
		res[7] = a[7] - b[7];
		res
	}
}

// smul
// scalar/multivector multiplication
impl Mul<PGA2D> for float_t {
	type Output = PGA2D;

	fn mul(self: float_t, b: PGA2D) -> PGA2D {
		let mut res = PGA2D::zero();
		let a = self;
		res[0] = a * b[0];
		res[1] = a * b[1];
		res[2] = a * b[2];
		res[3] = a * b[3];
		res[4] = a * b[4];
		res[5] = a * b[5];
		res[6] = a * b[6];
		res[7] = a * b[7];
		res
	}
}

// muls
// multivector/scalar multiplication
impl Mul<float_t> for PGA2D {
	type Output = PGA2D;

	fn mul(self: PGA2D, b: float_t) -> PGA2D {
		let mut res = PGA2D::zero();
		let a = self;
		res[0] = a[0] * b;
		res[1] = a[1] * b;
		res[2] = a[2] * b;
		res[3] = a[3] * b;
		res[4] = a[4] * b;
		res[5] = a[5] * b;
		res[6] = a[6] * b;
		res[7] = a[7] * b;
		res
	}
}

// sadd
// scalar/multivector addition
impl Add<PGA2D> for float_t {
	type Output = PGA2D;

	fn add(self: float_t, b: PGA2D) -> PGA2D {
		let mut res = PGA2D::zero();
		let a = self;
		res[0] = a + b[0];
		res[1] = b[1];
		res[2] = b[2];
		res[3] = b[3];
		res[4] = b[4];
		res[5] = b[5];
		res[6] = b[6];
		res[7] = b[7];
		res
	}
}

// adds
// multivector/scalar addition
impl Add<float_t> for PGA2D {
	type Output = PGA2D;

	fn add(self: PGA2D, b: float_t) -> PGA2D {
		let mut res = PGA2D::zero();
		let a = self;
		res[0] = a[0] + b;
		res[1] = a[1];
		res[2] = a[2];
		res[3] = a[3];
		res[4] = a[4];
		res[5] = a[5];
		res[6] = a[6];
		res[7] = a[7];
		res
	}
}

impl PGA2D {
	pub fn norm(self: Self) -> float_t {
		let scalar_part = (self * self.Conjugate())[0];

		scalar_part.abs().sqrt()
	}

	pub fn inorm(self: Self) -> float_t {
		self.Dual().norm()
	}

	pub fn normalized(self: Self) -> Self {
		self * (1.0 / self.norm())
	}
}

fn main() {
	// let l1 = 0.0 * e0 + 1.0 * e1 + 0.0 * e2; // x=0
	// let l2 = 0.0 * e0 + -0.5f64.sqrt() * e1 + 0.5f64.sqrt() * e2; // x-y=0
	// let t = l1 * l2;
	// let p = 3.0 * e01 + 2.0 * e20 + e12; // x=2, y=3
	// dbg!(&t);
	// dbg!(&p);
	// dbg!(t * p * t.Reverse());

	// let t = (2.0 * e12 + 3.0 * e20 + 5.0 * e01);
	// let p = (2.0 * e12 + 3.0 * e20 + 5.0 * e01);

	// dbg! | (7.0 * e0 + 11.0 * e1 + 13.0 * e2));

	// dbg!((2.0 * e12 + 3.0 * e20 + 5.0 * e01) | (7.0 * e0 + 11.0 * e1 + 13.0 * e2));

	// dbg!(e12 * e12);

	let s = PGA2D::new(1.0, 0);
	let blades = vec![
		s,
		e0.clone(),
		e1.clone(),
		e2.clone(),
		e01.clone(),
		e20.clone(),
		e12.clone(),
		e012.clone(),
	];

	println!();
	println!("Geometric product multiplication table (left side * top row):");
	for &a in &blades {
		print!("  ");
		for &b in &blades {
			print!("{:<8}", (a * b).to_string());
		}
		println!();
	}

	// println!();
	// println!("Inner / dot product multiplication table (left side | top row):");
	// for &a in &blades {
	//     print!("  ");
	//     for &b in &blades {
	//         print!("{:<8}", (a | b).to_string());
	//     }
	//     println!();
	// }

	println!();
	println!("Outer product multiplication table (left side ^ top row):");
	for &a in &blades {
		print!("  ");
		for &b in &blades {
			print!("{:<8}", (a ^ b).to_string());
		}
		println!();
	}

	println!();
	println!("Regressive product multiplication table (right side & bottom row):");
	for &a in &blades {
		print!("  ");
		for &b in &blades {
			print!("{:<8}", (a & b).to_string());
		}
		println!();
	}
}

///! Handcrafted
use std::ops::Mul;

use derive_more::{Add, Sub};

// ----------------------------------------------------------------------------
// Geometric Algebra definition helpers:

/// Special zero type for completeness, and better error messages.
/// If you get this in an error message, it is because you multiplied
/// two dimensions that always results in zero.
pub struct Zero {}

pub trait Dual {
	type Output;
	fn dual(self) -> Self::Output;
}

macro_rules! impl_dual {
	($T: path, $O: path) => {
		impl Dual for $T {
			type Output = $O;
			#[inline(always)]
			fn dual(self) -> Self::Output {
				$O(self.0)
			}
		}
	};

	($T: path, -$O: path) => {
		impl Dual for $T {
			type Output = $O;
			#[inline(always)]
			fn dual(self) -> Self::Output {
				$O(-self.0)
			}
		}
	};
}

macro_rules! impl_mul {
	($L: path, $R: path, Zero) => {
		impl Mul<$R> for $L {
			type Output = Zero;
			#[inline(always)]
			fn mul(self, _: $R) -> Self::Output {
				Zero {}
			}
		}
	};

	($L: path, $R: path, $O: path) => {
		impl Mul<$R> for $L {
			type Output = $O;
			#[inline(always)]
			fn mul(self, right: $R) -> Self::Output {
				$O(self.0 * right.0)
			}
		}
	};

	($L: path, $R: path, -$O: path) => {
		impl Mul<$R> for $L {
			type Output = $O;
			#[inline(always)]
			fn mul(self, right: $R) -> Self::Output {
				$O(-self.0 * right.0)
			}
		}
	};
}

/// The inner product.
/// Dots each individual elements against each other and sum the results.
pub trait Dot<Rhs> {
	type Output;
	fn dot(&self, other: &Rhs) -> Self::Output;
}

macro_rules! impl_dot {
	($L: path, $R: path, Zero) => {
		impl Dot<$R> for $L {
			type Output = Zero;
			#[inline(always)]
			fn dot(&self, _: &$R) -> Self::Output {
				Zero {}
			}
		}
	};

	($L: path, $R: path, $O: path) => {
		impl Dot<$R> for $L {
			type Output = $O;
			#[inline(always)]
			fn dot(&self, right: &$R) -> Self::Output {
				$O(self.0 * right.0)
			}
		}
	};

	($L: path, $R: path, -$O: path) => {
		impl Dot<$R> for $L {
			type Output = $O;
			#[inline(always)]
			fn dot(&self, right: &$R) -> Self::Output {
				$O(-self.0 * right.0)
			}
		}
	};
}

// ----------------------------------------------------------------------------
// The definition of 2D Projective Geometric Algebra
// using the e1^2=0, e1^2=1, e1^2=1 algebra.

// The basis vector types:

/// The scalar type.
#[derive(Copy, Clone, Debug, PartialEq, Add, Sub)]
pub struct Scalar(pub f64);

/// Projective axis.
/// Squares to 0
#[derive(Copy, Clone, Debug, PartialEq, Add, Sub)]
pub struct E0(pub f64);

/// X axis
/// Squares to 1
#[derive(Copy, Clone, Debug, PartialEq, Add, Sub)]
pub struct E1(pub f64);

/// Y axis
/// Squares to 1
#[derive(Copy, Clone, Debug, PartialEq, Add, Sub)]
pub struct E2(pub f64);

/// Squares to 0 (translation)
#[derive(Copy, Clone, Debug, PartialEq, Add, Sub)]
pub struct E01(pub f64);

/// Squares to 0 (translation)
/// NOTE: E20 = -E02.
/// I picked E20 over E02 just so that we don't need to flip the sign
/// of Y coordinates when converting to/from euclidean coordinates.
#[derive(Copy, Clone, Debug, PartialEq, Add, Sub)]
pub struct E20(pub f64);

/// Squares to -1 (rotation)
#[derive(Copy, Clone, Debug, PartialEq, Add, Sub)]
pub struct E12(pub f64);

/// Pseudo-scalar.
/// Squares to 0.
#[derive(Copy, Clone, Debug, PartialEq, Add, Sub)]
pub struct E012(pub f64);

// -----------------------------------

// TODO: generate this code from the definitions of e0,e1,e2
impl_dual!(Scalar, E012);
impl_dual!(E0, E12);
impl_dual!(E1, -E20);
impl_dual!(E2, E01);
impl_dual!(E01, E2);
impl_dual!(E20, -E1);
impl_dual!(E12, E0);
impl_dual!(E012, Scalar);

// -----------------------------------

impl_mul!(Scalar, Scalar, Scalar);
impl_mul!(Scalar, E0, E0);
impl_mul!(Scalar, E1, E1);
impl_mul!(Scalar, E2, E2);
impl_mul!(Scalar, E01, E01);
impl_mul!(Scalar, E20, E20);
impl_mul!(Scalar, E12, E12);
impl_mul!(Scalar, E012, E012);

impl_mul!(E0, Scalar, E0);
impl_mul!(E0, E0, Zero); // By definition in 2D PGA
impl_mul!(E0, E1, E01);
impl_mul!(E0, E2, -E20);
impl_mul!(E0, E01, Zero);
impl_mul!(E0, E20, Zero);
impl_mul!(E0, E12, E012);
impl_mul!(E0, E012, Zero);

impl_mul!(E1, Scalar, E1);
impl_mul!(E1, E0, -E01);
impl_mul!(E1, E1, Scalar); // By definition in 2D PGA
impl_mul!(E1, E2, E12);
impl_mul!(E1, E01, -E0);
impl_mul!(E1, E20, E012);
impl_mul!(E1, E12, E2);
impl_mul!(E1, E012, E20);

impl_mul!(E2, Scalar, E2);
impl_mul!(E2, E0, E20);
impl_mul!(E2, E1, -E12);
impl_mul!(E2, E2, Scalar); // By definition in 2D PGA
impl_mul!(E2, E01, E012);
impl_mul!(E2, E20, E0);
impl_mul!(E2, E12, -E1);
impl_mul!(E2, E012, E01);

impl_mul!(E01, Scalar, E01);
impl_mul!(E01, E0, Zero);
impl_mul!(E01, E1, E0);
impl_mul!(E01, E2, E012);
impl_mul!(E01, E01, Zero);
impl_mul!(E01, E20, Zero);
impl_mul!(E01, E12, -E20);
impl_mul!(E01, E012, Zero);

impl_mul!(E20, Scalar, E20);
impl_mul!(E20, E0, Zero);
impl_mul!(E20, E1, E012);
impl_mul!(E20, E2, -E0);
impl_mul!(E20, E01, Zero);
impl_mul!(E20, E20, Zero);
impl_mul!(E20, E12, E01);
impl_mul!(E20, E012, Zero);

impl_mul!(E12, Scalar, E12);
impl_mul!(E12, E0, E012);
impl_mul!(E12, E1, -E2);
impl_mul!(E12, E2, E1);
impl_mul!(E12, E01, E20);
impl_mul!(E12, E20, -E01);
impl_mul!(E12, E12, -Scalar);
impl_mul!(E12, E012, -E0);

impl_mul!(E012, Scalar, E012);
impl_mul!(E012, E0, Zero);
impl_mul!(E012, E1, E20);
impl_mul!(E012, E2, E01);
impl_mul!(E012, E01, Zero);
impl_mul!(E012, E20, Zero);
impl_mul!(E012, E12, -E0);
impl_mul!(E012, E012, Zero);

// -----------------------------------

impl_dot!(Scalar, Scalar, Scalar);
impl_dot!(Scalar, E0, E0);
impl_dot!(Scalar, E1, E1);
impl_dot!(Scalar, E2, E2);
impl_dot!(Scalar, E01, E01);
impl_dot!(Scalar, E20, E20);
impl_dot!(Scalar, E12, E12);
impl_dot!(Scalar, E012, E012);

impl_dot!(E0, Scalar, E0);
impl_dot!(E0, E0, Zero); // By definition in 2D PGA
impl_dot!(E0, E1, Zero);
impl_dot!(E0, E2, Zero);
impl_dot!(E0, E01, Zero);
impl_dot!(E0, E20, Zero);
impl_dot!(E0, E12, Zero);
impl_dot!(E0, E012, Zero);

impl_dot!(E1, Scalar, E1);
impl_dot!(E1, E0, Zero);
impl_dot!(E1, E1, Scalar); // By definition in 2D PGA
impl_dot!(E1, E2, Zero);
impl_dot!(E1, E01, -E0);
impl_dot!(E1, E20, Zero);
impl_dot!(E1, E12, E2);
impl_dot!(E1, E012, E20);

impl_dot!(E2, Scalar, E2);
impl_dot!(E2, E0, Zero);
impl_dot!(E2, E1, Zero);
impl_dot!(E2, E2, Scalar); // By definition in 2D PGA
impl_dot!(E2, E01, Zero);
impl_dot!(E2, E20, E0);
impl_dot!(E2, E12, -E1);
impl_dot!(E2, E012, E01);

impl_dot!(E01, Scalar, E01);
impl_dot!(E01, E0, Zero);
impl_dot!(E01, E1, E0);
impl_dot!(E01, E2, Zero);
impl_dot!(E01, E01, Zero);
impl_dot!(E01, E20, Zero);
impl_dot!(E01, E12, Zero);
impl_dot!(E01, E012, Zero);

impl_dot!(E20, Scalar, E20);
impl_dot!(E20, E0, Zero);
impl_dot!(E20, E1, Zero);
impl_dot!(E20, E2, -E0);
impl_dot!(E20, E01, Zero);
impl_dot!(E20, E20, Zero);
impl_dot!(E20, E12, Zero);
impl_dot!(E20, E012, Zero);

impl_dot!(E12, Scalar, E12);
impl_dot!(E12, E0, Zero);
impl_dot!(E12, E1, -E2);
impl_dot!(E12, E2, E1);
impl_dot!(E12, E01, Zero);
impl_dot!(E12, E20, Zero);
impl_dot!(E12, E12, -Scalar);
impl_dot!(E12, E012, -E0);

impl_dot!(E012, Scalar, E012);
impl_dot!(E012, E0, Zero);
impl_dot!(E012, E1, E20);
impl_dot!(E012, E2, E01);
impl_dot!(E012, E01, Zero);
impl_dot!(E012, E20, Zero);
impl_dot!(E012, E12, -E0);
impl_dot!(E012, E012, Zero);

// ----------------------------------------------------------------------------
// From here on out we just define useful helpers using the types above.

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point {
	/// 1 for euclidean points, 0 for direction / ideal points
	pub e12: E12,

	/// positive X
	pub e20: E20,

	/// positive Y
	pub e01: E01,
}

impl Dual for Point {
	type Output = Line;
	fn dual(self) -> Line {
		Line {
			e0: self.e12.dual(),
			e1: self.e20.dual(),
			e2: self.e01.dual(),
		}
	}
}

impl Point {
	pub fn from_euclidean(x: f64, y: f64) -> Point {
		Point {
			e12: E12(1.0),
			e20: E20(x),
			e01: E01(y),
		}
	}

	/// direction, point at infinity, ideal point
	pub fn from_direction(x: f64, y: f64) -> Point {
		Point {
			e12: E12(0.0),
			e20: E20(x),
			e01: E01(y),
		}
	}

	/// V / regressive product / join
	pub fn join(&self, other: Point) -> Line {
		(self.dual().wedge(&other.dual())).dual()
	}

	pub fn into_euclidean(&self) -> Option<(f64, f64)> {
		if self.e12.0 == 0.0 {
			None
		} else {
			Some((self.e20.0 / self.e12.0, self.e01.0 / self.e12.0))
		}
	}
}

// ----------------------------------------------------------------------------

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Line {
	pub e0: E0,
	pub e1: E1,
	pub e2: E2,
}

impl Dual for Line {
	type Output = Point;
	fn dual(self) -> Point {
		Point {
			e12: self.e0.dual(),
			e20: self.e1.dual(),
			e01: self.e2.dual(),
		}
	}
}

impl Dot<Line> for Line {
	type Output = Scalar;
	fn dot(&self, other: &Line) -> Self::Output {
		// NOTE: e0.dot(&e0) is always zero
		self.e1.dot(&other.e1) + self.e2.dot(&other.e2)
	}
}

impl Dot<Point> for Line {
	type Output = Line;
	fn dot(&self, other: &Point) -> Self::Output {
		Line {
			e0: self.e1.dot(&other.e01) + self.e2.dot(&other.e20),
			e1: self.e2.dot(&other.e12),
			e2: self.e1.dot(&other.e12),
		}
	}
}

impl Line {
	// aX + bY + c = 0
	pub fn from_euclidean(a: f64, b: f64, c: f64) -> Line {
		Line {
			e0: E0(c),
			e1: E1(a),
			e2: E2(b),
		}
	}

	pub fn intersect(&self, other: &Line) -> Point {
		self.wedge(other)
	}

	/// ^ / wedge/ outer product / meet
	pub fn wedge(&self, other: &Line) -> Point {
		Point {
			e12: self.e1 * other.e2 + self.e2 * other.e1,
			e20: self.e2 * other.e0 + self.e0 * other.e2,
			e01: self.e0 * other.e1 + self.e1 * other.e0,
		}
	}

	// /// Reflect the point over the line
	// pub fn reflect(&self, p: &Point) -> Point {
	// }
}

// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
	use super::*;

	use itertools::Itertools;

	struct Canvas {
		w: i64,
		h: i64,
		pixels: Vec<char>,
	}

	impl Canvas {
		pub fn new(side: usize) -> Canvas {
			let mut c = Canvas {
				w: side as i64,
				h: side as i64,
				pixels: vec![' '; side * side],
			};
			c.paint_axes();
			c
		}

		pub fn paint_axes(&mut self) {
			for x in -self.w / 2..=self.w / 2 {
				self.set((x as f64, 0.0), '.');
			}
			for y in -self.h / 2..=self.h / 2 {
				self.set((0.0, y as f64), '.');
			}
			self.set((0.0, 0.0), '+');
		}

		pub fn set(&mut self, (x, y): (f64, f64), c: char) {
			let x = self.w / 2 + x.round() as i64;
			let y = self.h / 2 + y.round() as i64;
			if 0 <= x && x < self.w && 0 <= y && y < self.h {
				let i = y * self.w + x;
				self.pixels[i as usize] = c;
			}
		}

		pub fn paint(mut self, shape: impl Shape, c: char) -> Self {
			shape.paint(&mut self, c);
			self
		}

		pub fn render(&self) -> String {
			let (w, h) = (self.w, self.h);
			(0..h)
				.flat_map(|y| {
					let y = h - y - 1;
					(0..w)
						.flat_map(move |x| Some(self.pixels[(y * w + x) as usize]).into_iter().chain(Some(' ')))
						.chain(Some('\n'))
				})
				.collect()
		}
	}

	trait Shape {
		fn paint(&self, canvas: &mut Canvas, c: char);
	}

	impl Shape for Point {
		fn paint(&self, canvas: &mut Canvas, c: char) {
			if let Some(p) = self.into_euclidean() {
				canvas.set(p, c);
			}
		}
	}

	impl Shape for Line {
		fn paint(&self, canvas: &mut Canvas, chr: char) {
			// aX + bY + c = 0
			let (a, b, c) = (self.e1.0, self.e2.0, self.e0.0);

			if a.abs() > b.abs() {
				for y in -canvas.h / 2..=canvas.h / 2 {
					let y = y as f64;
					let x = (-c - b * y) / a;
					canvas.set((x, y), chr);
				}
			} else if b.abs() > 0.0 {
				for x in -canvas.w / 2..=canvas.w / 2 {
					let x = x as f64;
					let y = (-c - a * x) / b;
					canvas.set((x, y), chr);
				}
			}
		}
	}

	macro_rules! assert_eq_canvas {
		($L: expr, $R: expr) => {{
			let left: String = $L.split('\n').map(|s| s.trim_end()).join("\n");
			let right = $R.trim_start_matches('\n');
			if left != right {
				panic!("Got:\n{}\n\nExpected:\n{}\n", left, right);
				}
			}};
	}

	#[test]
	fn test() {
		assert_eq_canvas!(
			Canvas::new(7).paint(Point::from_euclidean(2.0, 3.0), 'P').render(),
			r"
      .   P
      .
      .
. . . + . . .
      .
      .
      .
"
		);
	}

	#[test]
	fn line_intersecion() {
		assert_eq!(
			Line::from_euclidean(1.0, 0.0, -1.0).intersect(&Line::from_euclidean(0.0, 1.0, -2.0)),
			Point::from_euclidean(1.0, 2.0),
		);

		let l0 = Line::from_euclidean(1.0, -1.0, 0.0);
		let l1 = Line::from_euclidean(0.0, -1.0, 3.0);
		let p = l0.intersect(&l1);

		assert_eq_canvas!(
			Canvas::new(9).paint(l0, '0').paint(l1, '1').paint(p, 'p').render(),
			r"
        .       0
1 1 1 1 1 1 1 p 1
        .   0
        . 0
. . . . 0 . . . .
      0 .
    0   .
  0     .
0       .
"
		);
	}

	// 	#[test]
	// 	fn reflect() {
	// 		let l = Line::from_euclidean(1.0, -1.0, 2.0);
	// 		let p = Pont::from_euclidean(2.0, 3.0);
	// 		let r = l.reflect(p);

	// 		assert_eq_canvas!(
	// 			Canvas::new(9).paint(l, 'l').paint(p, 'p').paint(r, 'r').render(),
	// 			r"
	//         .       0
	// 1 1 1 1 1 1 1 p 1
	//         .   0
	//         . 0
	// . . . . 0 . . . .
	//       0 .
	//     0   .
	//   0     .
	// 0       .
	// "
	// 		);
	// }
}
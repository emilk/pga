mod blade;
mod op;
mod rust;
mod sblade;
mod simplify;
mod typ;
mod types;
mod typify;

pub use {blade::*, op::*, sblade::*, typ::*, types::*};

/// Which base vector (e0, e1 or e2?)
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, derive_more::Display)]
pub struct VecIdx(pub usize);

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Product {
	/// Geom = Inner + Outer = Dot + Wedge
	Geometric,
	/// Inner / dot product.
	/// The commutative part of the geometric product.
	/// Measures sameness of blades.
	Dot,
	/// Outer (progressive) product. Moves to a higher dimension.
	/// The anti-commutative part of the geometric product.
	/// Measures difference between blades.
	Wedge,
	/// Regressive. Reduces the dimensionality.
	Antiwedge,
}

/// what you get when you square the input vectors,
/// e.g. [0, 1, 1] would specify the 2d gpa of e0^2=0  e1^2=1  e2^2=1
pub struct Grammar(pub Vec<i32>);

// ----------------------------------------------------------------------------

impl Grammar {
	pub fn square(&self, product: Product, v: VecIdx) -> i32 {
		match product {
			Product::Geometric => self.0[v.0],
			Product::Dot => self.0[v.0],
			Product::Wedge => 0,
			Product::Antiwedge => 0,
		}
	}

	pub fn num_vecs(&self) -> usize {
		self.0.len()
	}

	pub fn vecs(&self) -> impl Iterator<Item = VecIdx> {
		(0..self.num_vecs()).map(VecIdx)
	}
}

impl Product {
	pub fn symbol(&self) -> &str {
		match self {
			Product::Geometric => "*",
			Product::Dot => "|",
			Product::Wedge => "^",
			Product::Antiwedge => "&",
		}
	}
}

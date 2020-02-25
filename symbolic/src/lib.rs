mod blade;
mod op;
mod rust;
mod sblade;
mod simplify;
mod typ;
mod types;
mod typify;

pub use {blade::*, op::*, sblade::*, typ::*, types::*};

/// The scalar type we use for symbolic reasoning.
// type S = i32;

/// Which base vector (e0, e1 or e2?)
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, derive_more::Display)]
pub struct VecIdx(pub usize);

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Product {
	Geometric,
	/// Outer (progressive) product.  Moves to a higher dimension.
	Wedge,
	/// Regressive. Reduces the dimensionality.
	Antiwedge,
}

/// what you get when you square the input vectors,
/// e.g. [0, 1, 1] would specify the 2d gpa of e0^2=0  e1^2=1  e2^2=1
pub struct Grammar(pub Vec<i32>);

// ----------------------------------------------------------------------------

impl Grammar {
	/// What do we get when we square this basis vector?
	pub fn square(&self, v: VecIdx) -> i32 {
		self.0[v.0]
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
			Product::Wedge => "^",
			Product::Antiwedge => "&",
		}
	}
}

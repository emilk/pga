mod blade;
mod expr;
pub mod grammars;
pub mod markdown;
mod rust;
mod sblade;
mod simplify;
mod typ;
mod types;
mod typify;

pub use {blade::*, expr::*, sblade::*, typ::*, types::*};

/// Which base vector (e0, e1 or e2?)
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, derive_more::Display)]
pub struct VecIdx(pub usize);

/// Types of distributative unary operations
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, strum_macros::EnumIter)]
pub enum Unary {
	/// Right compliment.
	/// The right compliment of a blade B is defined so that
	/// B * RCompl(B) = PseudoScalar
	/// distributive:  RCompl(a + b) = RCompl(a) + RCompl(b)
	RCompl,

	/// Left compliment.
	/// The left compliment of a blade B is defined so that
	/// LCompl(B) * B = PseudoScalar
	/// distributive:  LCompl(a + b) = LCompl(a) + LCompl(b)
	LCompl,

	/// Reverse the order of the vector indices:
	/// e1.reverse()   = e1
	/// e12.reverse()  = e21  = -e12
	/// e012.reverse() = e210 = -e012
	/// Used for sandwich products
	Reverse,

	/// x.anti_reverse() ==
	/// x.lcompl().reverse().rcompl()
	/// x.rcompl().reverse().lcompl()
	/// Used for anti-sandwich-products
	AntiReverse,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, strum_macros::EnumIter)]
pub enum Product {
	/// Geom = Inner + Outer = Dot + Wedge
	Geometric,
	/// Geometric Antiproduct = rcompl(lcompl(a) * lcompl(b))
	AntiGeometric,

	/// Inner / dot product.
	/// The commutative part of the geometric product.
	/// Measures sameness of blades.
	Dot,

	/// Outer (progressive) product. Moves to a higher dimension.
	/// The anti-commutative part of the geometric product.
	/// Measures difference between blades.
	Wedge,

	/// Regressive. Reduces the dimensionality.
	AntiWedge,
}

/// what you get when you square the input vectors,
/// e.g. [0, 1, 1] would specify the 2d gpa of e0^2=0  e1^2=1  e2^2=1
pub struct Grammar(pub Vec<i32>);

// ----------------------------------------------------------------------------

impl Grammar {
	pub fn square_geom(&self, v: VecIdx) -> i32 {
		self.0[v.0]
	}

	/// What do we get when we multiply the given base vector with itself using the given product?
	pub fn square_with(&self, product: Product, v: VecIdx) -> Option<i32> {
		match product {
			Product::Geometric => Some(self.0[v.0]),
			Product::AntiGeometric => None, // TODO
			Product::Dot => Some(self.0[v.0]),
			Product::Wedge => Some(0),
			Product::AntiWedge => Some(0),
		}
	}

	/// Number of base vectors, i.e. the dimensionaltiy of the grammar
	pub fn num_vecs(&self) -> usize {
		self.0.len()
	}

	pub fn vecs(&self) -> impl Iterator<Item = VecIdx> {
		(0..self.num_vecs()).map(VecIdx)
	}
}

impl Unary {
	pub fn name(self) -> &'static str {
		match self {
			Unary::LCompl => "lcompl",
			Unary::RCompl => "rcompl",
			Unary::Reverse => "rev",
			Unary::AntiReverse => "arev",
		}
	}

	pub fn short_description(self) -> &'static str {
		match self {
			Unary::LCompl => "Left complement",
			Unary::RCompl => "Right complement",
			Unary::Reverse => "Reverse",
			Unary::AntiReverse => "Anti-reverse",
		}
	}

	/// whan undoes this operation?
	pub fn undoer(self) -> Self {
		match self {
			Unary::LCompl => Unary::RCompl,
			Unary::RCompl => Unary::LCompl,
			Unary::Reverse => Unary::Reverse,
			Unary::AntiReverse => Unary::AntiReverse,
		}
	}
}

impl Product {
	pub fn symbol(self) -> &'static str {
		match self {
			Product::Geometric => "*",
			Product::AntiGeometric => "!*", // TODO
			Product::Dot => "|",
			Product::Wedge => "^",
			Product::AntiWedge => "&",
		}
	}

	pub fn trait_name(self) -> &'static str {
		match self {
			Product::Geometric => "Geometric",
			Product::AntiGeometric => "AntiGeometric",
			Product::Dot => "Dot",
			Product::Wedge => "Wedge",
			Product::AntiWedge => "AntiWedge",
		}
	}

	pub fn trait_function_name(self) -> &'static str {
		match self {
			Product::Geometric => "geometric",
			Product::AntiGeometric => "anti_geometric",
			Product::Dot => "dot",
			Product::Wedge => "wedge",
			Product::AntiWedge => "anti_wedge",
		}
	}
}

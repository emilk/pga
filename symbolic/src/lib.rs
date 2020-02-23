pub mod op;
pub mod rust;
pub mod simplify;
pub mod typ;

/// The scalar type we use for symbolic reasoning.
// type S = i32;

/// Which base vector (e0, e1 or e2?)
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct VecIdx(pub usize);

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Op {
	Var(String, Type),

	// A unit length base vector
	Vec(VecIdx),

	/// Indicated a scalar times something.
	/// Wedge(vec![X, 3, Y, 4]) simplifies to Term(Wedge(vec![X, Y]), 12)
	/// In its simplest form, the scalar is never 0 or 1
	/// 0 == Sum(vec![])
	/// 1 == Prod(_, vec![])
	Term(Box<Op>, i32),

	// Neg(Box<Op>),
	// Dual(Box<Op>),
	// Rev(Box<Op>),
	Sum(Vec<Op>),
	Prod(Product, Vec<Op>),
	// Dot(Vec<Op>),
	// AntiProd(Vec<Op>),
	// AntiDot(Vec<Op>),
	// AntiWedge(Vec<Op>),
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Product {
	Geometric,
	Wedge,
}

/// A type is some sort of multivector.
/// A value is a linear combination of types.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Type {
	Zero,
	// One,
	/// The scalar type, i.e. 1
	// S,
	/// The vector types
	// Vec(VecIdx),

	/// Blade(vec![])     = scalar
	/// Blade(vec![0])    = e0
	/// Blade(vec![2, 0]) = e02
	Blade(Vec<VecIdx>),
	// /// Tuple-type, linear combination of blades
	// /// Blades(vec![])       = () = Zero
	// /// Blades(vec![S])      = (S)
	// /// Blades(vec![S, E02]) = (S, E02)
	// Blades(Vec<Type>),
	/// named members
	Struct(Vec<(String, Type)>),
}

#[derive(Clone, Debug)]
pub struct Typedef {
	name: String,
	typ: Type,
}

/// In order of preference (first match).
#[derive(Clone, Debug, Default)]
pub struct Types {
	types: Vec<Typedef>,
	// Maps the sorted blade to the sign and typedef of that blade,
	// e.g. maps [0,2] to  (-1, {"ZX", [2, 0]})
	blades: std::collections::BTreeMap<Vec<VecIdx>, (i32, Typedef)>,
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
}

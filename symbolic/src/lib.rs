pub mod blade;
pub mod op;
pub mod rust;
pub mod sblade;
pub mod simplify;
pub mod typ;
pub mod typify;

/// The scalar type we use for symbolic reasoning.
// type S = i32;

/// Which base vector (e0, e1 or e2?)
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct VecIdx(pub usize);

/// Blade(vec![])     = scalar
/// Blade(vec![0])    = e0
/// Blade(vec![0, 2]) = e02
/// Always sorted, always unique.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Blade(Vec<VecIdx>);
/// A blade type with a sign,
/// this is useful so we can express e20 = -e02.
/// Can be both a type (-e02) and a value (42 * e123)
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct SBlade {
	blade: Blade,
	sign: i32,
}

/// TODO: rename Expr ?
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

	/// Left compliment.
	/// The left compliment of a blade B is defined so that
	/// LCompl(B) * B = PseudoScalar
	/// distributive:  LCompl(a + b) = LCompl(a) + LCompl(b)
	LCompl(Box<Op>),

	// Unary operations:
	/// Right compliment.
	/// The right compliment of a blade B is defined so that
	/// B * RCompl(B) = PseudoScalar
	/// distributive:  RCompl(a + b) = RCompl(a) + RCompl(b)
	RCompl(Box<Op>),

	// N-ary operations:
	Sum(Vec<Op>),
	Prod(Product, Vec<Op>),
	// Dot(Vec<Op>),
	// AntiProd(Vec<Op>),
	// AntiDot(Vec<Op>),
	// AntiWedge(Vec<Op>),
	StructInstance {
		name: String,
		members: Vec<(String, Op)>,
	},
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Product {
	Geometric,
	/// Outer (progressive) product.  Moves to a higher dimension.
	Wedge,
	/// Regressive. Reduces the dimensionality.
	Antiwedge,
}

/// A type is some sort of multivector.
/// A value is a linear combination of types.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Type {
	/// Has a sign so that we can normalize e20 to -e02
	SBlade(SBlade),
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
	// Maps blades to the typedef of that blade,
	// e.g. maps [0,2] to  Typedef{"e20", Type::(-1, Blade([0, 2]))}
	blades: std::collections::BTreeMap<Blade, Typedef>,
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

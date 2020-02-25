use crate::*;

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

impl Op {
	pub fn zero() -> Op {
		Op::Sum(vec![])
	}

	pub fn one() -> Op {
		Op::Prod(Product::Geometric, vec![])
	}

	pub fn scalar(s: i32) -> Self {
		match s {
			0 => Self::zero(),
			1 => Self::one(),
			s => Op::Term(Op::one().into(), s),
		}
	}

	pub fn var(name: impl ToString, typ: &Type) -> Self {
		Op::Var(name.to_string(), typ.clone())
	}

	pub fn geometric(factors: Vec<Op>) -> Self {
		Op::Prod(Product::Geometric, factors)
	}

	/// outer product
	pub fn wedge(factors: Vec<Op>) -> Self {
		Op::Prod(Product::Wedge, factors)
	}

	/// also known as the regressive product
	pub fn antiwedge(factors: Vec<Op>) -> Self {
		// Op::RCompl(Op::wedge(factors.into_iter().map(|op| Op::LCompl(op.into())).collect()).into())
		Op::Prod(Product::Antiwedge, factors)
	}

	/// Note: self must be simplified
	pub fn is_zero(&self) -> bool {
		self == &Self::zero()
	}

	/// Note: self must be simplified
	pub fn is_one(&self) -> bool {
		match self {
			Op::Prod(_, factors) if factors.is_empty() => true,
			_ => false,
		}
	}

	pub fn as_scalar(&self) -> Option<i32> {
		match self {
			Op::Term(op, s) if op.is_one() => Some(*s),
			_ => None,
		}
	}

	/// Returns this Op in terms of a multiple of a blade, if possible
	pub fn as_sblade(&self, g: &Grammar) -> Option<SBlade> {
		match self {
			Op::Var(_, _) => None,
			Op::Vec(vi) => Some(SBlade::vec(*vi)),
			Op::Term(op, s) => {
				if let Some(sblade) = op.as_sblade(g) {
					Some(*s * sblade)
				} else {
					None
				}
			}
			Op::LCompl(op) => Some(op.as_sblade(g)?.lcompl(g)),
			Op::RCompl(op) => Some(op.as_sblade(g)?.rcompl(g)),
			Op::Sum(terms) => {
				if terms.is_empty() {
					Some(SBlade::zero())
				} else if terms.len() == 1 {
					terms[0].as_sblade(g)
				} else {
					None // assuming we are simplified
				}
			}
			Op::Prod(_product, factors) => {
				// This assumes we are simplified,
				// i.e. that there are no repeated vector indices!

				let mut scalar = 1;
				let mut vecs = vec![];
				for f in factors {
					let sb = f.as_sblade(g)?;
					scalar *= sb.sign;
					// TODO: check for duplicates and simplify using a grammar!
					vecs.extend(sb.blade.vecs());
				}
				Some(scalar * SBlade::from_unsorted(&vecs))
			}
			Op::StructInstance { .. } => None,
		}
	}

	pub fn negate(self) -> Self {
		match self {
			Op::Term(op, -1) => *op,
			Op::Term(op, s) => Op::Term(op, -s),
			op => Op::Term(op.into(), -1),
		}
	}
}

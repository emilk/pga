use crate::*;

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
	pub fn as_blade(&self, g: &Grammar) -> Option<(i32, Blade)> {
		match self {
			Op::Var(_, _) => None,
			Op::Vec(vi) => Some((1, Blade::vec(*vi))),
			Op::Term(op, s) => {
				if let Some((scalar, blade)) = op.as_blade(g) {
					Some((s * scalar, blade))
				} else {
					None
				}
			}
			Op::LCompl(op) => {
				let (op_sign, blade) = op.as_blade(g)?;
				let (compl_sign, blade_compl) = blade.lcompl(g);
				Some((op_sign * compl_sign, blade_compl))
			}
			Op::RCompl(op) => {
				let (op_sign, blade) = op.as_blade(g)?;
				let (compl_sign, blade_compl) = blade.rcompl(g);
				Some((op_sign * compl_sign, blade_compl))
			}
			Op::Sum(terms) => {
				if terms.is_empty() {
					Some((0, Blade::scalar()))
				} else if terms.len() == 1 {
					terms[0].as_blade(g)
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
					let (s, b) = f.as_blade(g)?;
					scalar *= s;
					// TODO: check for duplicates and simplify using a grammar!
					vecs.extend(b.vecs());
				}
				let (sign, blade) = Blade::from_unsorted(&vecs);
				Some((scalar * sign, blade))
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

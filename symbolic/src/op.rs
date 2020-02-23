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

	pub fn wedge(factors: Vec<Op>) -> Self {
		Op::Prod(Product::Wedge, factors)
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

	pub fn typ(&self, g: Option<&Grammar>) -> Option<Type> {
		match self {
			Op::Var(_, typ) => Some(typ.clone()),
			Op::Term(_, 0) => Some(Type::Zero),
			Op::Term(op, _) => op.typ(g),
			Op::Vec(vi) => Some(Type::vec(*vi)),
			Op::Sum(terms) => {
				if terms.is_empty() {
					Some(Type::Zero)
				} else if terms.len() == 1 {
					terms[0].typ(g)
				} else {
					None // TODO
				}
			}
			Op::Prod(product, factors) => {
				if factors.is_empty() {
					Some(Type::scalar()) // TODO: Type::One ?
				} else if factors.len() == 1 {
					factors[0].typ(g)
				} else {
					None // TODO
				}
			}
		}
	}

	/// Returns this Op in terms of a multiple of a blade, if possible
	pub fn as_blade(&self) -> Option<(i32, Vec<VecIdx>)> {
		match self {
			Op::Var(_, _) => None,
			Op::Vec(vi) => Some((1, vec![*vi])),
			Op::Term(op, s) => {
				if let Some((scalar, blade)) = op.as_blade() {
					Some((s * scalar, blade))
				} else {
					None
				}
			}
			Op::Sum(terms) => {
				if terms.is_empty() {
					Some((0, vec![]))
				} else if terms.len() == 1 {
					terms[0].as_blade()
				} else {
					None // assuming we are simplified
				}
			}
			Op::Prod(product, factors) => {
				// This assumes we are simplified,
				// i.e. that there are no repeated vector indices!

				let mut scalar = 1;
				let mut blade = vec![];
				for f in factors {
					let (s, mut b) = f.as_blade()?;
					scalar *= s;
					blade.append(&mut b);
				}
				Some((scalar, blade))
			}
		}
	}
}

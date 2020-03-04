use crate::*;

/// A type is some sort of multivector.
/// A value is a linear combination of types.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Type {
	/// Special type: a value that is always the magnitude 1.
	/// This is useful for describing a normalized point with {x: X, y: Y, z: Z, w: One(W)}
	Constant(SBlade),
	/// Has a sign so that we can normalize e20 to -e02
	SBlade(SBlade),
	/// named members
	Struct(Vec<(String, Type)>),
}

impl Type {
	pub fn zero() -> Self {
		Type::SBlade(SBlade::zero())
	}

	/// Special type: a value that is always the scalar value 1.
	/// This is useful for describing a normalized point with {x: X, y: Y, z: Z, w: 1}
	pub fn constant(sblade: SBlade) -> Self {
		Type::Constant(sblade)
	}

	pub fn scalar() -> Self {
		Type::SBlade(SBlade::scalar())
	}

	pub fn vec(vi: VecIdx) -> Self {
		Type::SBlade(SBlade::vec(vi))
	}

	pub fn unsorted_blade(vecs: &[VecIdx]) -> Self {
		Type::SBlade(SBlade::from_unsorted(vecs))
	}

	pub fn is_zero(&self) -> bool {
		match self {
			Type::Constant(sb) => sb.is_zero(),
			Type::SBlade(sb) => sb.is_zero(),
			Type::Struct(_) => todo!(),
		}
	}

	pub fn is_negative(&self) -> bool {
		match self {
			Type::Constant(sb) => sb.is_negative(),
			Type::SBlade(sb) => sb.is_negative(),
			Type::Struct(_) => todo!(),
		}
	}

	pub fn is_blade(&self, blade: &Blade) -> bool {
		match self {
			Type::Constant(_) => todo!(),
			Type::SBlade(sb) => sb.blade == *blade,
			Type::Struct(_) => false,
		}
	}

	pub fn into_sblade(self) -> Option<SBlade> {
		match self {
			Type::Constant(sb) => Some(sb),
			Type::SBlade(sb) => Some(sb),
			Type::Struct(_) => None,
		}
	}

	pub fn unit(&self) -> Op {
		match self {
			Type::Constant(sblade) | Type::SBlade(sblade) => Op::sblade(sblade),
			_ => todo!(),
		}
	}

	pub fn unary(&self, unary: Unary, g: Option<&Grammar>) -> Option<Type> {
		match self {
			Type::Constant(sblade) | Type::SBlade(sblade) => Some(Type::SBlade(sblade.unary(unary, g?))),
			_ => todo!("{:?}.{}()", self, unary.name()),
		}
	}
}

impl Op {
	pub fn typ(&self, g: Option<&Grammar>) -> Option<Type> {
		match self {
			Op::Var(_, typ) => Some(typ.clone()),
			Op::Term(_, 0) => Some(Type::zero()),
			Op::Term(op, _) => op.typ(g),
			Op::Vec(vi) => Some(Type::vec(*vi)),
			Op::Unary(unary, op) => op.typ(g).and_then(|t| t.unary(*unary, g)),
			Op::Sum(terms) => {
				if terms.is_empty() {
					Some(Type::zero())
				} else if terms.len() == 1 {
					terms[0].typ(g)
				} else {
					todo!("figure out type of sum '{}'", self.rust())
				}
			}
			Op::Prod(product, factors) => product_type(*product, factors, g),
			Op::StructInstance { members, .. } => {
				let members: Option<Vec<(String, Type)>> = members
					.iter()
					.map(|(name, op)| Some((name.to_string(), op.typ(g)?)))
					.collect();
				members.map(Type::Struct)
			}
		}
	}
}

fn product_type(product: Product, factors: &[Op], g: Option<&Grammar>) -> Option<Type> {
	if factors.is_empty() {
		Some(Type::scalar()) // TODO: Type::One ?
	} else if factors.len() == 1 {
		factors[0].typ(g)
	} else {
		let types: Option<Vec<Type>> = factors.iter().map(|f| f.typ(g)).collect();
		let types = types?;
		if types.iter().any(Type::is_zero) {
			return Some(Type::zero());
		}
		let sblades: Option<Vec<SBlade>> = types.into_iter().map(Type::into_sblade).collect();
		let sblades = sblades?;

		if let Some(g) = g {
			Some(Type::SBlade(SBlade::product(product, &sblades, g)))
		} else {
			None
		}
	}
}

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

	pub fn strct(s: &Struct) -> Self {
		Self::Struct(s.iter().map(|(name, mem)| (name.clone(), mem.typ.clone())).collect())
	}

	pub fn is_zero(&self) -> bool {
		match self {
			Type::Constant(sb) => sb.is_zero(),
			Type::SBlade(sb) => sb.is_zero(),
			Type::Struct(_) => false,
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

	pub fn unit(&self) -> Expr {
		match self {
			Type::Constant(sblade) | Type::SBlade(sblade) => Expr::sblade(sblade),
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

impl Expr {
	pub fn typ(&self, g: Option<&Grammar>) -> Option<Type> {
		match self {
			Expr::Var { typ, .. } => Some(typ.clone()),
			Expr::Term(_, 0) => Some(Type::zero()),
			Expr::Term(expr, _) => expr.typ(g),
			Expr::Vec(vi) => Some(Type::vec(*vi)),
			Expr::Unary(unary, expr) => expr.typ(g).and_then(|t| t.unary(*unary, g)),
			Expr::Sum(terms) => {
				if terms.is_empty() {
					Some(Type::zero())
				} else {
					let mut types = std::collections::BTreeSet::new();
					for e in terms {
						types.insert(e.typ(g)?);
					}
					assert!(!types.is_empty());
					if types.len() == 1 {
						Some(types.into_iter().next().unwrap())
					} else {
						None // TODO: could be a struct?
					}
				}
			}
			Expr::Prod(product, factors) => product_type(*product, factors, g),
			Expr::StructInstance(StructInstance { strct, .. }) => Some(Type::strct(strct)),
		}
	}
}

fn product_type(product: Product, factors: &[Expr], g: Option<&Grammar>) -> Option<Type> {
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

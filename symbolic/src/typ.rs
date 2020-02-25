use crate::*;

/// A type is some sort of multivector.
/// A value is a linear combination of types.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Type {
	/// Has a sign so that we can normalize e20 to -e02
	SBlade(SBlade),
	/// named members
	Struct(Vec<(String, Type)>),
}

impl Type {
	pub fn zero() -> Self {
		Type::SBlade(SBlade::zero())
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
			Type::SBlade(sb) => sb.is_zero(),
			Type::Struct(_) => todo!(),
		}
	}

	pub fn is_negative(&self) -> bool {
		match self {
			Type::SBlade(sb) => sb.is_negative(),
			Type::Struct(_) => todo!(),
		}
	}

	pub fn is_blade(&self, blade: &Blade) -> bool {
		match self {
			Type::SBlade(sb) => sb.blade == *blade,
			Type::Struct(_) => false,
		}
	}

	pub fn unit(&self) -> Op {
		match self {
			// Type::S => Op::one(),
			// Type::Vec(vi) => Op::Vec(*vi),
			Type::SBlade(sblade) => {
				let op = match sblade.blade.grade() {
					0 => Op::one(),
					1 => Op::Vec(sblade.blade[0]),
					_ => Op::wedge(sblade.blade.vecs().iter().copied().map(Op::Vec).collect()),
				};
				match sblade.sign {
					-1 => op.negate(),
					0 => Op::zero(),
					1 => op,
					_ => unreachable!(),
				}
			}
			_ => panic!(),
		}
	}

	pub fn lcompl(&self, g: Option<&Grammar>) -> Option<Type> {
		match self {
			Type::SBlade(sblade) => Some(Type::SBlade(sblade.lcompl(g?))),
			_ => todo!("lcompl({:?})", self),
		}
	}

	pub fn rcompl(&self, g: Option<&Grammar>) -> Option<Type> {
		match self {
			Type::SBlade(sblade) => Some(Type::SBlade(sblade.rcompl(g?))),
			_ => todo!("rcompl({:?})", self),
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
			Op::LCompl(op) => op.typ(g).and_then(|t| t.lcompl(g)),
			Op::RCompl(op) => op.typ(g).and_then(|t| t.rcompl(g)),
			Op::Sum(terms) => {
				if terms.is_empty() {
					Some(Type::zero())
				} else if terms.len() == 1 {
					terms[0].typ(g)
				} else {
					println!("TODO: figure out type of sum '{}'", self.rust());
					None
				}
			}
			Op::Prod(product, factors) => product_type(*product, factors, g),
			Op::StructInstance { members, .. } => {
				let members: Option<Vec<(String, Type)>> = members
					.iter()
					.map(|(name, op)| Some((name.to_string(), op.typ(g)?)))
					.collect();
				members.map(|s| Type::Struct(s).into())
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
		if types.len() == 2 {
			let l = &types[0];
			let r = &types[1];

			if let (Type::SBlade(lb), Type::SBlade(rb)) = (&l, &r) {
				if lb.grade() == 1 && rb.grade() == 1 {
					let lv = lb.blade[0];
					let rv = rb.blade[0];
					// eprintln!("product_type of e{} {} e{}", lv, product.symbol(), rv);
					if lv == rv {
						match product {
							Product::Geometric => {
								if let Some(g) = g {
									return if g.square(lv) == 0 {
										Some(Type::zero())
									} else {
										Some(Type::scalar())
									};
								}
							}
							Product::Wedge => return Some(Type::zero()),
							Product::Antiwedge => return Some(Type::zero()), // TODO: is this correct?
						}
					} else {
						return Some(Type::SBlade(lb.sign * rb.sign * SBlade::from_unsorted(&[lv, rv])));
					}
				}
			}
		}

		println!("TODO: figure out type of product '{:?}'", types);
		None
	}
}

// fn product_type(product: Product, factors: &Vec<Op>) -> Option<Type> {
// 	todo!()
// }

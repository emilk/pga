use crate::*;

impl Type {
	pub fn zero() -> Self {
		Type::Blade(0, Blade::scalar())
	}

	pub fn scalar() -> Self {
		Type::Blade(1, Blade::scalar())
	}

	pub fn vec(vi: VecIdx) -> Self {
		Type::Blade(1, Blade::vec(vi))
	}

	pub fn unsorted_blade(vecs: &[VecIdx]) -> Self {
		let (sign, blade) = Blade::from_unsorted(vecs);
		Type::Blade(sign, blade)
	}

	pub fn is_zero(&self) -> bool {
		match self {
			Type::Blade(0, _) => true,
			Type::Blade(_, _) => false,
			Type::Struct(_) => todo!(),
		}
	}

	pub fn is_negative(&self) -> bool {
		match self {
			Type::Blade(s, _) => *s < 0,
			Type::Struct(_) => todo!(),
		}
	}

	pub fn is_blade(&self, blade: &Blade) -> bool {
		match self {
			Type::Blade(_, b) => b == blade,
			Type::Struct(_) => false,
		}
	}

	pub fn one(&self) -> Op {
		match self {
			// Type::S => Op::one(),
			// Type::Vec(vi) => Op::Vec(*vi),
			Type::Blade(sign, blade) => {
				let op = match blade.grade() {
					0 => Op::one(),
					1 => Op::Vec(blade[0]),
					_ => Op::wedge(blade.vecs().iter().copied().map(Op::Vec).collect()),
				};
				match sign {
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
			Type::Blade(sign, blade) => {
				let (compl_sign, compl) = blade.lcompl(g?);
				Some(Type::Blade(sign * compl_sign, compl))
			}
			_ => todo!("lcompl({:?})", self),
		}
	}

	pub fn rcompl(&self, g: Option<&Grammar>) -> Option<Type> {
		match self {
			Type::Blade(sign, blade) => {
				let (compl_sign, compl) = blade.rcompl(g?);
				Some(Type::Blade(sign * compl_sign, compl))
			}
			_ => todo!("rcompl({:?})", self),
		}
	}
}

impl Types {
	pub fn insert(&mut self, name: &str, typ: Type) {
		let typedef = Typedef {
			name: name.to_string(),
			typ: typ.clone(),
		};

		if let Type::Blade(sign, blade) = typ {
			self.blades.insert(blade, typedef.clone());
		}

		self.types.push(typedef);
	}

	pub fn structs(&self) -> impl Iterator<Item = (&str, &Vec<(String, Type)>)> {
		self.types.iter().filter_map(|td| {
			if let Type::Struct(members) = &td.typ {
				Some((td.name.as_str(), members))
			} else {
				None
			}
		})
	}

	pub fn get_typedef(&self, name: &str) -> &Typedef {
		self.types.iter().find(|td| td.name == name).unwrap()
	}

	pub fn get(&self, name: &str) -> &Type {
		&self.get_typedef(name).typ
	}

	// /// Returns the canonical name of this blade, including a sign change
	// /// For instance: blade_name([0, 2]) => (-1. "e20")
	// pub fn blade_name(&self, blade: &Blade) -> Option<(i32, &str)> {
	// 	self.blades.get(&blade).map(|td| match td.typ {
	// 		Type::Blade(sign, _) => (sign, td.name.as_str()),
	// 		_ => unreachable!(),
	// 	})
	// }

	pub fn blade_typedef(&self, blade: &Blade) -> Option<&Typedef> {
		self.blades.get(&blade)
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

			if let (Type::Blade(ls, lb), Type::Blade(rs, rb)) = (&l, &r) {
				if lb.grade() == 1 && rb.grade() == 1 {
					let lv = lb[0];
					let rv = rb[0];
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
						return Some(if lv < rv {
							Type::Blade(ls * rs, Blade::from_sorted(vec![lv, rv]))
						} else {
							Type::Blade(-ls * rs, Blade::from_sorted(vec![rv, lv]))
						});
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

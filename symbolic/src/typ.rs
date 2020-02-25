use crate::*;

impl Type {
	pub fn zero() -> Self {
		Type::Blade(0, vec![])
	}

	pub fn scalar() -> Self {
		Type::Blade(1, vec![])
	}

	pub fn vec(vi: VecIdx) -> Self {
		Type::Blade(1, vec![vi])
	}

	pub fn blade(vecs: &[VecIdx]) -> Self {
		let (sign, vecs) = sort_blade(vecs.to_vec());
		assert!(!has_adjacent_copies(&vecs));
		Type::Blade(sign, vecs.to_vec())
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

	pub fn is_blade(&self, blade: &Vec<VecIdx>) -> bool {
		match self {
			Type::Blade(_, b) => b == blade,
			Type::Struct(_) => false,
		}
	}

	pub fn one(&self) -> Op {
		match self {
			// Type::S => Op::one(),
			// Type::Vec(vi) => Op::Vec(*vi),
			Type::Blade(sign, vecs) => {
				let op = match vecs.len() {
					0 => Op::one(),
					1 => Op::Vec(vecs[0]),
					_ => Op::wedge(vecs.iter().copied().map(Op::Vec).collect()),
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
}

impl Types {
	pub fn insert(&mut self, name: &str, typ: Type) {
		let typedef = Typedef {
			name: name.to_string(),
			typ: typ.clone(),
		};

		if let Type::Blade(sign, blade) = typ {
			self.blades.insert(blade, (sign, typedef.clone()));
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

	// pub fn vec_name(&self, vi: VecIdx) -> &str {
	// 	self.types
	// 		.iter()
	// 		.find(|td| match &td.typ {
	// 			// Type::Vec(v) if v == vi => true,
	// 			Type::Blade(vecs) if vecs.len() == 1 && vecs[0] == vi => true,
	// 			_ => false,
	// 		})
	// 		.map(|td| td.name.as_str())
	// 		.unwrap()
	// }

	pub fn blade_name(&self, blade: &[VecIdx]) -> Option<(i32, &str)> {
		let (sign, blade) = sort_blade(blade.to_vec());
		self.blades.get(&blade).map(|(s, td)| (sign * s, td.name.as_str()))
	}
}

/// Sort the vector indices, keeping track of all sign changes.
#[must_use]
fn sort_blade(mut b: Vec<VecIdx>) -> (i32, Vec<VecIdx>) {
	// Multiplication is anti-commutative so each time we swap we need to flip the sign.
	// So bubble-sort!
	let mut sign = 1;
	for _ in 0..b.len() {
		for i in 0..b.len() - 1 {
			if b[i] > b[i + 1] {
				b.swap(i, i + 1);
				sign = -sign;
			}
		}
	}
	(sign, b)
}

fn has_adjacent_copies(b: &[VecIdx]) -> bool {
	for i in 0..b.len() - 1 {
		if b[i] == b[i + 1] {
			return true;
		}
	}
	false
}

impl Op {
	pub fn typ(&self, g: Option<&Grammar>) -> Option<Type> {
		match self {
			Op::Var(_, typ) => Some(typ.clone()),
			Op::Term(_, 0) => Some(Type::zero()),
			Op::Term(op, _) => op.typ(g),
			Op::Vec(vi) => Some(Type::vec(*vi)),
			Op::Sum(terms) => {
				if terms.is_empty() {
					Some(Type::zero())
				} else if terms.len() == 1 {
					terms[0].typ(g)
				} else {
					println!("TODO: figure out type of '{}'", self.rust());
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
				if lb.len() == 1 && rb.len() == 1 {
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
						}
					} else {
						return Some(if lv < rv {
							Type::Blade(ls * rs, vec![lv, rv])
						} else {
							Type::Blade(-ls * rs, vec![rv, lv])
						});
					}
				}
			}
		}

		println!(
			"TODO: figure out type of '{}'",
			Op::Prod(product, factors.to_vec()).rust()
		);
		None
	}
}

// fn product_type(product: Product, factors: &Vec<Op>) -> Option<Type> {
// 	todo!()
// }

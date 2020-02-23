use crate::*;

impl Type {
	pub fn scalar() -> Self {
		Type::Blade(vec![])
	}

	pub fn vec(vi: VecIdx) -> Self {
		Type::Blade(vec![vi])
	}

	pub fn blade(vecs: &[VecIdx]) -> Self {
		Type::Blade(vecs.to_vec())
	}

	pub fn is_zero(&self) -> bool {
		*self == Type::Zero
	}

	pub fn one(&self) -> Op {
		match self {
			// Type::S => Op::one(),
			// Type::Vec(vi) => Op::Vec(*vi),
			Type::Blade(vecs) => match vecs.len() {
				0 => Op::one(),
				1 => Op::Vec(vecs[0]),
				_ => Op::wedge(vecs.iter().copied().map(Op::Vec).collect()),
			},
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

		if let Type::Blade(blade) = typ {
			let (sign, sorted) = sort_blade(blade);
			self.blades.insert(sorted, (sign, typedef.clone()));
		}

		self.types.push(typedef);
	}

	pub fn structs(&self) -> impl Iterator<Item = &Vec<(String, Type)>> {
		self.types.iter().filter_map(|t| {
			if let Type::Struct(members) = &t.typ {
				Some(members)
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
pub fn sort_blade(mut b: Vec<VecIdx>) -> (i32, Vec<VecIdx>) {
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

impl Op {
	/// Detect named types and replace
	pub fn typify(self, t: &Types) -> Self {
		// if let Op::Sum(terms) = self {
		// 	if let Some(s) = as_struct(terms, t, g) {
		// 		return Some(s);
		// 	}
		// }

		// A blade?
		if let Some((scalar, blade)) = self.as_blade() {
			if scalar == 0 {
				Op::zero()
			} else if blade.is_empty() {
				Op::scalar(scalar)
			} else if let Some((sign, name)) = t.blade_name(&blade) {
				let scalar = scalar * sign;
				let blade = Op::var(name, &Type::blade(&blade));
				match scalar {
					0 => Op::zero(),
					1 => blade,
					_ => Op::Term(blade.into(), scalar),
				}
			} else {
				self
			}
		} else {
			self
		}
	}
}

// fn as_struct(terms: &[Op], t: &Types, g: Option<&Grammar>) -> Option<RustExpr> {
// 	let mut parts: std::collections::BTreeMap<Type, Vec<Op>> = Default::default();
// 	for term in terms {
// 		let typ = term.typ(g)?;
// 		if !typ.is_zero() {
// 			parts.entry(typ).or_default().push(term.clone());
// 		}
// 	}

// 	let sum: Vec<(Type, Op)> = parts
// 		.into_iter()
// 		.map(|(typ, terms)| if terms.len() == 1 { terms[0] } else { Op::Sum(terms) })
// 		.collect();

// 	find_struct(&sum, t).map(RustExpr::atom)
// }

// fn find_struct(sum: &[(Type, Op)], t: &Types) -> Option<String> {
// 	for members in t.structs() {
// 		if let Some(instance) = as_struct_instance(members, &sum) {
// 			return Some(instance);
// 		}
// 	}
// 	None
// }

// fn as_struct_instance(members: &[(String, Type)], sum: &[(Type, Op)]) -> Option {

// }

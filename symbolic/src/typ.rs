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

	pub fn get_typedef(&self, name: &str) -> &Typedef {
		self.types.iter().find(|td| td.name == name).unwrap()
	}

	pub fn get(&self, name: &str) -> &Type {
		&self.get_typedef(name).typ
	}

	pub fn vec_name(&self, vi: VecIdx) -> &str {
		self.types
			.iter()
			.find(|td| match &td.typ {
				// Type::Vec(v) if v == vi => true,
				Type::Blade(vecs) if vecs.len() == 1 && vecs[0] == vi => true,
				_ => false,
			})
			.map(|td| td.name.as_str())
			.unwrap()
	}

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

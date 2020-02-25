use crate::*;

#[derive(Clone, Debug)]
pub struct Typedef {
	pub name: String,
	pub typ: Type,
}

/// In order of preference (first match).
#[derive(Clone, Debug, Default)]
pub struct Types {
	types: Vec<Typedef>,
	// Maps blades to the typedef of that blade,
	// e.g. maps [0,2] to  Typedef{"e20", Type::(-1, Blade([0, 2]))}
	blades: std::collections::BTreeMap<Blade, Typedef>,
}

impl Types {
	pub fn insert(&mut self, name: &str, typ: Type) {
		let typedef = Typedef {
			name: name.to_string(),
			typ: typ.clone(),
		};

		if let Type::SBlade(sblade) = typ {
			self.blades.insert(sblade.blade, typedef.clone());
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

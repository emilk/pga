use indexmap::IndexMap;

use crate::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StructMember {
	pub name: String,
	pub typ: Type,
}

pub type Struct = IndexMap<String, StructMember>;

/// In order of preference (first match).
#[derive(Clone, Debug, Default)]
pub struct Types {
	// types: Vec<Typedef>,
	types: IndexMap<String, Type>,

	/// Mapes struct to their declarations
	structs: IndexMap<String, Struct>,

	/// Maps blades to cannocial sign and name,
	/// e.g. [0,2] => -"e20"
	blades: IndexMap<Blade, (i32, String)>,
}

impl Types {
	pub fn insert_blade(&mut self, name: &str, sblade: SBlade) {
		self.blades
			.insert(sblade.blade.clone(), (sblade.sign, name.to_string()));
		self.types.insert(name.to_string(), Type::SBlade(sblade));
	}

	pub fn insert_struct(&mut self, name: &str, members: &[(&str, &str)]) {
		let strct: Struct = members
			.iter()
			.map(|(member, type_name)| {
				(
					member.to_string(),
					StructMember {
						name: type_name.to_string(),
						typ: self.get(type_name).clone(),
					},
				)
			})
			.collect();
		self.structs.insert(name.to_string(), strct.clone());

		let struct_type = strct.into_iter().map(|(key, val)| (key, val.typ)).collect();
		self.types.insert(name.to_string(), Type::Struct(struct_type));
	}

	pub fn get(&self, name: &str) -> &Type {
		self.types
			.get(name)
			.unwrap_or_else(|| panic!("Failed to find type '{}'", name))
	}

	pub fn get_struct(&self, name: &str) -> &Struct {
		self.structs
			.get(name)
			.unwrap_or_else(|| panic!("Failed to find struct '{}'", name))
	}

	/// Maps blades to cannocial sign and name,
	pub fn get_blade(&self, blade: &Blade) -> Option<&(i32, String)> {
		self.blades.get(blade)
	}

	pub fn type_name(&self, typ: &Type) -> &str {
		match typ {
			Type::SBlade(sblade) if sblade.is_zero() => "Zero",
			Type::SBlade(sblade) => self.get_blade(&sblade.blade).unwrap().1.as_str(),
			Type::Struct(_) => self
				.structs()
				.find(|(_, strct)| &Type::strct(strct) == typ)
				.map(|(name, _)| name)
				.unwrap(),
			_ => todo!("Get name of type {:?}", typ),
		}
	}

	pub fn sblades(&self) -> Vec<(&str, SBlade)> {
		self.blades
			.iter()
			.map(|(blade, (sign, name))| (name.as_str(), SBlade::signed_blade(*sign, blade.clone())))
			.collect()
	}

	pub fn unit_blades(&self) -> Vec<Expr> {
		self.sblades()
			.iter()
			.map(|(_name, sblade)| Expr::sblade(sblade))
			.collect()
	}

	pub fn structs(&self) -> impl Iterator<Item = (&str, &Struct)> {
		self.structs.iter().map(|(name, strct)| (name.as_str(), strct))
	}
}

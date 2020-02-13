use itertools::Itertools;

use crate::*;

// ----------------------------------------------------------------------------

/// e.g. { "x": e20, "y": e01, "w": e12 }
#[derive(Clone, Eq, PartialEq)]
pub struct Type(pub Vec<(String, Blade)>);

impl Type {
	/// Auto-name keys
	pub fn from_blades(blades: Vec<Blade>) -> Self {
		Self(blades.into_iter().map(|b| (b.to_string(), b)).collect())
	}

	/// Is the given value an instance of this type?
	/// i.e., does it only have blades that are part of this type?
	/// The given value should be simplified / normalized
	pub fn is_value(&self, value: &Sum, grammar: &Grammar) -> bool {
		value.blades(grammar).iter().all(|blade| self.has_blade(blade))
	}

	pub fn has_blade(&self, blade: &SignedBlade) -> bool {
		blade.is_zero() || self.0.iter().any(|(_, b)| b == &blade.blade)
	}

	/// Project the given value onto this type,
	/// returning a value containing only the blades of this type.
	pub fn project(&self, value: &Sum, grammar: &Grammar) -> TypeInstance {
		TypeInstance(
			self.0
				.iter()
				.map(|(name, blade)| (name.clone(), value.project(blade, grammar)))
				.collect(),
		)
	}
}

impl std::fmt::Display for Type {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(
			f,
			"{{\n{}\n}}",
			self.0
				.iter()
				.map(|(name, blade)| format!("  {:5} {},", format!("{}:", name), blade))
				.join("\n")
		)
	}
}

// ----------------------------------------------------------------------------

/// members and their values
pub struct TypeInstance(Vec<(String, Sum)>);

impl std::fmt::Display for TypeInstance {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(
			f,
			"{{\n{}\n}}",
			self.0
				.iter()
				.map(|(name, value)| format!("  {:5} {},", format!("{}:", name), value))
				.join("\n")
		)
	}
}

// ----------------------------------------------------------------------------

/// A structure type with a name. TODO: RENAME
#[derive(Clone, Eq, PartialEq)]
pub struct NominalType {
	/// e.g. "Point"
	pub name: String,
	/// e.g. {e20, e01, e12}
	pub members: Type,
	/// e.g. "p"
	pub example_instance_name: String,
	// TODO: doc-string etc
}

// ----------------------------------------------------------------------------

// #[derive(strum_macros::EnumIter, Debug)]
// pub enum UnaryOp {
// 	Dual,
// }

#[derive(strum_macros::EnumIter, Debug)]
pub enum BinaryOp {
	Product,
	// Outer,
	// Inner,
	// Regressive,
	Sandwich,
}

// ----------------------------------------------------------------------------

pub struct GeneratorBuilder {
	pub grammar: Grammar,
	pub nominal_types: Vec<NominalType>,
}

impl GeneratorBuilder {
	pub fn pga_2d() -> Self {
		let grammar = GrammarBuilder::pga_2d().build();

		let s = Blade::from_indices(vec![]);
		let e0 = Blade::from_indices(vec![VecIdx(0)]);
		let e1 = Blade::from_indices(vec![VecIdx(1)]);
		let e2 = Blade::from_indices(vec![VecIdx(2)]);
		let e01 = Blade::from_indices(vec![VecIdx(0), VecIdx(1)]);
		let e12 = Blade::from_indices(vec![VecIdx(1), VecIdx(2)]);
		let e20 = Blade::from_indices(vec![VecIdx(2), VecIdx(0)]);

		Self {
			grammar,
			nominal_types: vec![
				NominalType {
					name: "Line".to_string(),
					members: Type::from_blades(vec![e0.clone(), e1.clone(), e2.clone()]),
					example_instance_name: "l".to_string(),
				},
				NominalType {
					name: "Point".to_string(),
					members: Type::from_blades(vec![e20.clone(), e01.clone(), e12.clone()]),
					example_instance_name: "p".to_string(),
				},
				NominalType {
					name: "Transform".to_string(),
					members: Type::from_blades(vec![s.clone(), e20.clone(), e01.clone(), e12.clone()]),
					example_instance_name: "t".to_string(),
				},
			],
		}
	}

	pub fn build(self) -> Generator {
		let GeneratorBuilder { grammar, nominal_types } = self;
		let mut blades: Vec<Blade> = (0..grammar.dims())
			.map(|_| Some(false).into_iter().chain(Some(true)))
			.multi_cartesian_product()
			.map(|bools| Blade::from_bools(&bools))
			.map(|blade| grammar.simplify(SignedBlade::unit(&blade)).blade)
			.collect();
		blades.sort();
		Generator {
			grammar,
			blades,
			nominal_types,
		}
	}
}

// ----------------------------------------------------------------------------

pub struct Generator {
	pub grammar: Grammar,
	/// The generating blades, e.g. [s, e0, e1, e2, e01, e20, e12, e012]
	pub blades: Vec<Blade>,
	pub nominal_types: Vec<NominalType>,
}

impl Generator {
	pub fn print(&self) {
		let grammar = &self.grammar;
		let blades = &self.blades;

		println!("Notation (following bivector.net):");
		println!(" !  dual");
		println!(" *  geometric multiplication");
		println!(" |  dot / inner product");
		println!(" ^  wedge / outer product (meet). (a ^ b) = !(!a & !b)");
		println!(" &  regressive product (join).    (a & b) = !(!a ^ !b)");
		println!(" x² = x * x");
		println!();
		println!(" R: Real number (scalar)");
		println!(" e0, e1, e2, ...: basis vectors (generators)");
		println!(" e23 = e2^e3");
		println!(" R, e0, e1, e2, e01, e02, e12, e123: blades");
		println!();
		println!("Basis vectors / generators and algebra definition:");
		for vi in 0..grammar.dims() {
			println!("  e{}² = {:>2}", vi, grammar.square(VecIdx(vi)));
		}

		println!();
		println!("Blades:");
		for blade in blades {
			println!("  {}", blade);
		}

		// 1.0 times each blade
		let unit_blades: Vec<SignedBlade> = blades.iter().map(SignedBlade::unit).collect();

		println!();
		println!("Duals:");
		for base in &unit_blades {
			println!("  !{:<5} = {}", base, base.dual(grammar));
		}

		println!();
		println!("Reversed:");
		for base in &unit_blades {
			println!("  rev {:<5} = {}", base, base.reverse());
		}

		println!();
		println!("Geometric multiplication table (left side * top row):");
		for a in &unit_blades {
			print!("  ");
			for b in &unit_blades {
				print!("{:<8}", a.geometric(b, grammar));
			}
			println!();
		}

		println!();
		println!("Inner / dot product multiplication table (left side | top row):");
		for a in &unit_blades {
			print!("  ");
			for b in &unit_blades {
				print!("{:<8}", a.dot(b, grammar));
			}
			println!();
		}

		println!();
		println!("Outer product table (left side ^ top row):");
		for a in &unit_blades {
			print!("  ");
			for b in &unit_blades {
				print!("{:<8}", a.outer(b, grammar));
			}
			println!();
		}

		println!();
		println!("Regressive product (join) multiplication table (right side & bottom row):");
		for a in &unit_blades {
			print!("  ");
			for b in &unit_blades {
				print!("{:<8}", a.regressive(b, grammar));
			}
			println!();
		}

		println!();
		println!("TYPES:");
		for t in &self.nominal_types {
			println!();
			println!("{} {}", t.name, t.members);
		}

		// println!();
		// println!("DUALS:");
		// for t in &self.nominal_types {
		// 	let name = &t.example_instance_name;
		// 	let value = Sum::instance(name, &t.members);
		// 	let result = value.dual(grammar);
		// 	if let Some(result_type) = self.find_type(&result) {
		// 		let result = result_type.members.project(&result);
		// 		// TODO: print this correctly
		// 		println!();
		// 		println!("dual({}: {}) -> {} {}", name, t.name, result_type.name, result);
		// 	} else {
		// 		// println!();
		// 		// println!("dual({}: {}) -> {}", name, t.name, result);
		// 	}
		// }

		println!();
		println!("OPERATIONS:");

		for left in &self.nominal_types {
			for right in &self.nominal_types {
				let (l_name, r_name) = if left == right {
					("l", "r")
				} else {
					(
						left.example_instance_name.as_str(),
						right.example_instance_name.as_str(),
					)
				};
				let l_value = Sum::instance(l_name, &left.members);
				let r_value = Sum::instance(r_name, &right.members);

				let result = (l_value * r_value).simplify(grammar);
				if let Some(typ) = self.find_type(&result) {
					let result = typ.members.project(&result, grammar);
					println!();
					println!(
						"({}: {}) * ({}: {}) -> {} {}",
						l_name, left.name, r_name, right.name, typ.name, result
					);
				} else {
					// println!();
					// println!(
					// 	"({}: {}) * ({}: {}) -> {}",
					// 	l_name, left.name, r_name, right.name, result
					// );
				}

				// let product = (l_value.sandwich(&r_value)).simplify(grammar);
				// println!();
				// println!(
				// 	"{} {} SANDWICHED_BY {} {} = {}",
				// 	left.name, l_name, right.name, r_name, product
				// );
			}
		}

		// TODO: do the same with unary operations
	}

	fn find_type(&self, value: &Sum) -> Option<&NominalType> {
		if value.is_zero() {
			None
		} else {
			self.nominal_types
				.iter()
				.find(|t| t.members.is_value(value, &self.grammar))
		}
	}
}

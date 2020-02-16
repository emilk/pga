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
		value.sblades(grammar).iter().all(|blade| self.has_blade(blade))
	}

	pub fn has_blade(&self, blade: &SignedBlade) -> bool {
		blade.is_zero() || self.0.iter().any(|(_, b)| b == &blade.blade)
	}

	/// Project the given value onto this type,
	/// returning a value containing only the blades of this type.
	pub fn select(&self, value: &Sum, grammar: &Grammar) -> TypeInstance {
		TypeInstance(
			self.0
				.iter()
				.map(|(name, blade)| (name.clone(), value.select(blade, grammar)))
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
					members: Type::from_blades(vec![e1.clone(), e2.clone(), e0.clone()]),
					example_instance_name: "l".to_string(),
				},
				NominalType {
					name: "Point".to_string(),
					members: Type::from_blades(vec![e12.clone(), e20.clone(), e01.clone()]),
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

	pub fn pga_3d() -> Self {
		let grammar = GrammarBuilder::pga_3d().build();

		// let s = Blade::from_indices(vec![]);
		// Planes:
		let e0 = Blade::from_indices(vec![VecIdx(0)]);
		let e1 = Blade::from_indices(vec![VecIdx(1)]);
		let e2 = Blade::from_indices(vec![VecIdx(2)]);
		let e3 = Blade::from_indices(vec![VecIdx(3)]);
		// Translate:
		let e01 = Blade::from_indices(vec![VecIdx(0), VecIdx(1)]);
		let e02 = Blade::from_indices(vec![VecIdx(0), VecIdx(2)]);
		let e03 = Blade::from_indices(vec![VecIdx(0), VecIdx(3)]);
		// Rotate:
		let e12 = Blade::from_indices(vec![VecIdx(1), VecIdx(2)]);
		let e31 = Blade::from_indices(vec![VecIdx(3), VecIdx(1)]);
		let e23 = Blade::from_indices(vec![VecIdx(2), VecIdx(3)]);
		// Points:
		let e021 = Blade::from_indices(vec![VecIdx(0), VecIdx(2), VecIdx(1)]);
		let e013 = Blade::from_indices(vec![VecIdx(0), VecIdx(1), VecIdx(3)]);
		let e032 = Blade::from_indices(vec![VecIdx(0), VecIdx(3), VecIdx(2)]);
		let e123 = Blade::from_indices(vec![VecIdx(1), VecIdx(2), VecIdx(3)]);
		// Pseduo-scalar;
		// let e0123 = Blade::from_indices(vec![VecIdx(0), VecIdx(1), VecIdx(2), VecIdx(3)]);

		Self {
			grammar,
			nominal_types: vec![
				NominalType {
					name: "Plane".to_string(),
					members: Type::from_blades(vec![e1.clone(), e2.clone(), e3.clone(), e0.clone()]),
					example_instance_name: "a".to_string(),
				},
				NominalType {
					name: "Line".to_string(),
					members: Type::from_blades(vec![
						e01.clone(),
						e02.clone(),
						e03.clone(),
						e12.clone(),
						e31.clone(),
						e23.clone(),
					]),
					example_instance_name: "l".to_string(),
				},
				NominalType {
					name: "Point".to_string(),
					members: Type::from_blades(vec![e021.clone(), e013.clone(), e032.clone(), e123.clone()]),
					example_instance_name: "p".to_string(),
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

#[derive(Clone, Copy)]
pub enum Generate {
	Signature,
	Implementation,
}

// ----------------------------------------------------------------------------

pub struct Generator {
	grammar: Grammar,
	/// The generating blades, e.g. [s, e0, e1, e2, e01, e20, e12, e012]
	blades: Vec<Blade>,
	nominal_types: Vec<NominalType>,
}

impl Generator {
	pub fn grammar(&self) -> &Grammar {
		&self.grammar
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

	pub fn type_from_name(&self, name: &str) -> Option<&NominalType> {
		self.nominal_types.iter().find(|t| t.name == name)
	}

	pub fn format_typed_value(&self, untyped_value: &Sum) -> String {
		if let Some(typ) = self.find_type(untyped_value) {
			let typed_value = typ.members.select(untyped_value, &self.grammar);
			format!("{} {}", typ.name, typed_value)
		} else {
			untyped_value.to_string()
		}
	}

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
				print!("{:<8}", a.inner(b, grammar));
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
	}

	pub fn print_ops(&self, generate: Generate) {
		let grammar = &self.grammar;

		println!();
		println!("--------------------------------------");
		println!("DUALS:");
		self.print_unop(generate, "dual", |v| v.dual(grammar));

		println!();
		println!("--------------------------------------");
		println!("REVERSE:");
		self.print_unop(generate, "reverse", |v| v.reverse(grammar));

		println!();
		println!("--------------------------------------");
		println!("GEOMETRIC MULTIPLICATION:");
		self.print_binop(generate, "*", |l, r| l.mul(r, grammar));

		println!();
		println!("--------------------------------------");
		println!("INNER / DOT PRODUCT:");
		self.print_binop(generate, "|", |l, r| l.inner(r, grammar));

		println!();
		println!("--------------------------------------");
		println!("OUTER / WEDGE PRODUCT (MEET):");
		self.print_binop(generate, "^", |l, r| l.outer(r, grammar));

		println!();
		println!("--------------------------------------");
		println!("REGRESSIVE PRODUCT (JOIN):");
		self.print_binop(generate, "&", |l, r| l.regressive(r, grammar));

		println!();
		println!("--------------------------------------");
		println!("SANDWICHING:");
		self.print_binop(generate, "SANDWICHING", |l, r| l.sandwich(r, grammar));
	}

	fn print_unop(&self, generate: Generate, op_symbol: &str, f: impl Fn(Sum) -> Sum) {
		for t in &self.nominal_types {
			let name = &t.example_instance_name;
			let value = Sum::instance(name, &t.members);
			let out = f(value);
			if let Some(out_type) = self.find_type(&out) {
				let out = out_type.members.select(&out, &self.grammar);
				match generate {
					Generate::Signature => {
						println!("{}({}) -> {}", op_symbol, t.name, out_type.name);
					}
					Generate::Implementation => {
						println!();
						println!("{}({}: {}) -> {} {}", op_symbol, name, t.name, out_type.name, out);
					}
				}
			} else {
				// println!();
				// println!("{}({}: {}) -> {}", op_symbol, name, t.name, out);
			}
		}
	}

	fn print_binop(&self, generate: Generate, op_symbol: &str, f: impl Fn(Sum, Sum) -> Sum) {
		let grammar = &self.grammar;
		for l_type in &self.nominal_types {
			for r_type in &self.nominal_types {
				let (l_name, r_name) = if l_type == r_type {
					("l", "r")
				} else {
					(
						l_type.example_instance_name.as_str(),
						r_type.example_instance_name.as_str(),
					)
				};
				let l_value = Sum::instance(l_name, &l_type.members);
				let r_value = Sum::instance(r_name, &r_type.members);

				let out = f(l_value, r_value);
				if let Some(out_type) = self.find_type(&out) {
					// if out_type == r_type // TODO: for sandwiching ?
					{
						let out = out_type.members.select(&out, grammar);
						match generate {
							Generate::Signature => {
								println!("{} {} {} -> {}", l_type.name, op_symbol, r_type.name, out_type.name);
							}
							Generate::Implementation => {
								println!();
								println!(
									"({}: {}) {} ({}: {}) -> {} {}",
									l_name, l_type.name, op_symbol, r_name, r_type.name, out_type.name, out
								);
							}
						}
					}
				} else {
					// println!();
					// println!(
					// 	"({}: {}) {} ({}: {}) -> {}",
					// 	l_name, l_type.name, op_symbol, r_name, r_type.name, result
					// );
				}
			}
		}
	}
}

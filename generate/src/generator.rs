use itertools::Itertools;

use crate::*;

// ----------------------------------------------------------------------------

/// A set of blades, e.g. { "x": e20, "y": e01, "w": e12 }
#[derive(Clone, Eq, PartialEq)]
pub struct Type(Vec<(String, Blade)>);

impl Type {
	/// Auto-name keys
	pub fn auto_named(blades: &[&Blade]) -> Self {
		Self(blades.iter().map(|&b| (b.to_string(), b.clone())).collect())
	}

	pub fn named(blades: &[(&str, &Blade)]) -> Self {
		Self(
			blades
				.iter()
				.map(|(name, blade)| (name.to_string(), (*blade).clone()))
				.collect(),
		)
	}

	pub fn members(&self) -> impl Iterator<Item = &(String, Blade)> {
		self.0.iter()
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

	pub fn instance(&self, name: &str) -> Sum {
		Sum::instance(name, self)
	}
}

impl std::fmt::Display for Type {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(
			f,
			"{{\n{}\n}}",
			self.0
				.iter()
				.map(|(name, blade)| format!("  {:6} {},", format!("{}:", name), blade))
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
				.map(|(name, value)| format!("  {:6} {},", format!("{}:", name), value))
				.join("\n")
		)
	}
}

// ----------------------------------------------------------------------------

/// A structure type with a name. TODO: RENAME
#[derive(Clone, Eq, PartialEq)]
pub struct NamedType {
	/// e.g. "Point"
	pub name: String,
	/// The set of blades that defines this type, e.g. {e20, e01, e12}
	pub typ: Type,
	/// e.g. "p"
	pub example_instance_name: String,
	// TODO: doc-string etc
}

impl NamedType {
	pub fn instance(&self, name: &str) -> Sum {
		Sum::instance(name, &self.typ)
	}
}

// ----------------------------------------------------------------------------

pub struct GeneratorBuilder {
	pub grammar: Grammar,
	pub named_types: Vec<NamedType>,
}

impl GeneratorBuilder {
	/// dual construction, with vectors representing lines, and bivectors points
	pub fn pga_2d() -> Self {
		let grammar = GrammarBuilder::pga_2d().build();

		let s: Blade = "s".parse().unwrap();
		let e0: Blade = "e0".parse().unwrap();
		let e1: Blade = "e1".parse().unwrap();
		let e2: Blade = "e2".parse().unwrap();
		let e01: Blade = "e01".parse().unwrap();
		let e12: Blade = "e12".parse().unwrap();
		let e20: Blade = "e20".parse().unwrap();
		// let e012: Blade = "e012".parse().unwrap();

		Self {
			grammar,
			named_types: vec![
				NamedType {
					name: "Line".to_string(),
					example_instance_name: "l".to_string(),
					// typ: Type::auto_named(&[&e1, &e2, &e0]),
					typ: Type::named(&[("x", &e1), ("y", &e2), ("w", &e0)]),
				},
				NamedType {
					name: "Point".to_string(),
					example_instance_name: "p".to_string(),
					// typ: Type::auto_named(&[&e12, &e20, &e01]),
					typ: Type::named(&[("x", &e20), ("y", &e01), ("w", &e12)]),
				},
				NamedType {
					name: "Rotor".to_string(),
					example_instance_name: "r".to_string(),
					typ: Type::auto_named(&[&s, &e12]),
				},
				NamedType {
					name: "Translator".to_string(),
					example_instance_name: "t".to_string(),
					typ: Type::auto_named(&[&s, &e20, &e01]),
				},
				NamedType {
					name: "Motor".to_string(),
					example_instance_name: "m".to_string(),
					typ: Type::auto_named(&[&s, &e20, &e01, &e12]),
				},
				// Is this a Motor? Or a Transform?
				// NamedType {
				// 	name: "Transform".to_string(),
				// 	example_instance_name: "t".to_string(),
				// 	typ: Type::auto_named(&[&s, &e20, &e01, &e12, &e012]),
				// },
			],
		}
	}

	/// dual construction, with vectors representing planes, bivectors lines, and trivectors points
	pub fn pga_3d() -> Self {
		let grammar = GrammarBuilder::pga_3d().build();

		let s = Blade::from_indices(vec![]);
		// Planes:
		let e0: Blade = "e0".parse().unwrap();
		let e1: Blade = "e1".parse().unwrap();
		let e2: Blade = "e2".parse().unwrap();
		let e3: Blade = "e3".parse().unwrap();
		// Translate:
		let e01: Blade = "e01".parse().unwrap();
		let e02: Blade = "e02".parse().unwrap();
		let e03: Blade = "e03".parse().unwrap();
		// Rotate:
		let e12: Blade = "e12".parse().unwrap();
		let e31: Blade = "e31".parse().unwrap();
		let e23: Blade = "e23".parse().unwrap();
		// Points:
		let e021: Blade = "e021".parse().unwrap();
		let e013: Blade = "e013".parse().unwrap();
		let e032: Blade = "e032".parse().unwrap();
		let e123: Blade = "e123".parse().unwrap();
		// Pseduo-scalar;
		let e0123: Blade = "e0123".parse().unwrap();

		Self {
			grammar,
			named_types: vec![
				NamedType {
					name: "Point".to_string(),
					example_instance_name: "v".to_string(),
					typ: Type::auto_named(&[&e021, &e013, &e032, &e123]),
				},
				NamedType {
					name: "Line".to_string(),
					example_instance_name: "l".to_string(),
					typ: Type::auto_named(&[&e01, &e02, &e03, &e12, &e31, &e23]),
				},
				NamedType {
					name: "Plane".to_string(),
					example_instance_name: "p".to_string(),
					typ: Type::auto_named(&[&e1, &e2, &e3, &e0]),
				},
				NamedType {
					name: "Rotor".to_string(),
					example_instance_name: "r".to_string(),
					// docstring: "Quaternion".to_string(),
					typ: Type::auto_named(&[&s, &e12, &e31, &e23]),
				},
				NamedType {
					name: "Translator".to_string(),
					example_instance_name: "t".to_string(),
					typ: Type::auto_named(&[&s, &e01, &e02, &e03]),
				},
				NamedType {
					name: "Motor".to_string(),
					// docstring: "Dual Quaternion".to_string(),
					example_instance_name: "m".to_string(),
					typ: Type::auto_named(&[&s, &e01, &e02, &e03, &e12, &e31, &e23, &e0123]),
				},
			],
		}
	}

	pub fn build(self) -> Generator {
		let GeneratorBuilder { grammar, named_types } = self;
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
			named_types,
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
	named_types: Vec<NamedType>,
}

impl Generator {
	pub fn grammar(&self) -> &Grammar {
		&self.grammar
	}

	pub fn named_types(&self) -> impl Iterator<Item = &NamedType> {
		self.named_types.iter()
	}

	pub fn types(&self) -> impl Iterator<Item = &Type> {
		self.named_types().map(|nt| &nt.typ)
	}

	fn find_type(&self, value: &Sum) -> Option<&NamedType> {
		if value.is_zero() {
			None
		} else {
			self.named_types.iter().find(|t| t.typ.is_value(value, &self.grammar))
		}
	}

	pub fn type_from_name(&self, name: &str) -> Option<&NamedType> {
		self.named_types().find(|t| t.name == name)
	}

	pub fn format_typed_value(&self, untyped_value: &Sum) -> String {
		if let Some(typ) = self.find_type(untyped_value) {
			let typed_value = typ.typ.select(untyped_value, &self.grammar);
			format!("{} {}", typ.name, typed_value)
		} else {
			untyped_value.as_string(&self.grammar)
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
		println!(" &  antiwedge / regressive product (join).    (a & b) = !(!a ^ !b)");
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
			println!("  !{:<6} = {}", base, base.dual(grammar));
		}

		println!();
		println!("Reversed:");
		for base in &unit_blades {
			println!("  rev {:<6} = {}", base, base.reverse());
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
		println!("Regressive / antiwedge product (join) multiplication table (right side & bottom row):");
		for a in &unit_blades {
			print!("  ");
			for b in &unit_blades {
				print!("{:<8}", a.regressive(b, grammar));
			}
			println!();
		}

		println!();
		println!("TYPES:");
		for t in &self.named_types {
			println!();
			println!("{} {}", t.name, t.typ);
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
		println!("SQUARED:");
		self.print_unop(generate, "square", |v| v.clone().mul(v, grammar));

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
		println!("REGRESSIVE / ANTIWEDGE PRODUCT (JOIN):");
		self.print_binop(generate, "&", |l, r| l.regressive(r, grammar));

		println!();
		println!("--------------------------------------");
		println!("SANDWICHING:");
		self.print_binop(generate, "SANDWICHING", |l, r| l.sandwich(r, grammar));
	}

	fn print_unop(&self, generate: Generate, op_symbol: &str, f: impl Fn(Sum) -> Sum) {
		for t in &self.named_types {
			let name = &t.example_instance_name;
			let value = Sum::instance(name, &t.typ);
			let out = f(value);
			if let Some(out_type) = self.find_type(&out) {
				let out = out_type.typ.select(&out, &self.grammar);
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
		for l_type in &self.named_types {
			for r_type in &self.named_types {
				let l_name = l_type.example_instance_name.as_str();
				let r_name = r_type.example_instance_name.as_str();
				let (l_name, r_name) = if l_name == r_name { ("l", "r") } else { (l_name, r_name) };
				let l_value = Sum::instance(l_name, &l_type.typ);
				let r_value = Sum::instance(r_name, &r_type.typ);

				let out = f(l_value, r_value);
				if let Some(out_type) = self.find_type(&out) {
					// if out_type == r_type // TODO: for sandwiching ?
					{
						let out = out_type.typ.select(&out, grammar);
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

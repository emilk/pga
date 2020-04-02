use std::{collections::BTreeSet, error::Error, fs, io::prelude::*, path::Path};

use {itertools::Itertools, strum::IntoEnumIterator};

use generator::{documentation::*, *};

const CODE_SEPARATOR: &str = "// ---------------------------------------------------------------------";

struct Settings {
	float_type: String,
}

impl Default for Settings {
	fn default() -> Self {
		Self {
			float_type: "f64".to_string(),
		}
	}
}

pub struct Generator {
	grammar: Grammar,
	types: Types,
	settings: Settings,
	ro: RustOptions,
}

impl Generator {
	fn rust(&self, expr: Expr) -> String {
		expr.simplify(Some(&self.grammar))
			.typify(&self.types, &self.grammar)
			.rust(&self.ro)
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let mut args = pico_args::Arguments::from_env();

	// let grammar: String = args.value_from_str(["-g", "--grammar"])?;
	let out_dir: String = args.value_from_str(["-o", "--out_dir"])?;
	let out_dir = Path::new(&out_dir);

	let (grammar, types) = generator::grammars::pga2d();
	let settings = Settings::default();
	let gen = Generator {
		grammar,
		types,
		settings,
		ro: RustOptions { operators: false },
	};

	if false {
		// Test:
		let line = gen.types.get_struct("Line");
		let point = gen.types.get_struct("Point");
		dbg!(strct::struct_product_type_signature(
			&gen,
			&("Line", line),
			&("Point", point),
			Product::Dot
		));
		panic!("Planned");
	}

	fs::create_dir_all(out_dir)?;

	let mut mod_file_contents = include_str!("../templates/lib.rs").to_owned();

	let mut mods = BTreeSet::new();
	mods.insert("traits".to_string());
	mods.insert("blades".to_string());

	write_file(include_str!("../templates/traits.rs"), &out_dir.join("traits.rs"))?;
	write_file(&blades::file(&gen), &out_dir.join("blades.rs"))?;

	mod_file_contents += "\n// Types:\n";
	for (struct_name, strct) in gen.types.structs() {
		let mod_name = struct_name.to_ascii_lowercase();
		let file_name = format!("{}.rs", mod_name);
		let file_contents = strct::file(&gen, struct_name, strct);
		write_file(&file_contents, &out_dir.join(file_name))?;
		mod_file_contents += &format!("pub mod {};\n", mod_name);
		mods.insert(mod_name);
	}

	mod_file_contents += &format!(
		"\npub use self::{{\n{}\n}};\n",
		mods.iter().map(|mod_name| format!("\t{}::*,", mod_name)).join("\n")
	);

	write_file(&mod_file_contents, &out_dir.join("mod.rs"))?;

	Ok(())
}

fn write_file(unformatted_contents: &str, final_path: &Path) -> Result<(), Box<dyn Error>> {
	let temp_file_path = std::env::temp_dir().join("temp.rs");
	fs::File::create(&temp_file_path)?.write_all(unformatted_contents.as_bytes())?;
	cargo_fmt(&temp_file_path)?;

	let formatted_contents = fs::read_to_string(temp_file_path)?;

	// Only write file if it has actually changed:
	if matches!(fs::read_to_string(final_path), Ok(existing_contents) if existing_contents == formatted_contents) {
		eprintln!("No change to '{}'", final_path.display());
		return Ok(());
	}

	fs::File::create(&final_path)?.write_all(formatted_contents.as_bytes())?;
	eprintln!("New file written to '{}'", final_path.display());

	Ok(())
}

fn cargo_fmt(path: &Path) -> Result<(), Box<dyn Error>> {
	std::process::Command::new("cargo")
		.arg("fmt")
		.arg("--")
		.arg(path)
		.output()?;
	Ok(())
}

mod blades {
	use super::*;

	pub fn file(gen: &Generator) -> String {
		let documentation = with_line_prefixes("//! ", &documentation(gen).trim());
		format!(
			"\
		{}\n\n\
		use derive_more::{{Add, Mul, Neg, Sub}};\n\
		\n\
		use super::*;\n\
		\n\
		{}\n\n\
		{}\n\
		{}\n",
			documentation,
			declare_blades(gen),
			CODE_SEPARATOR,
			impl_blade_ops(gen),
		)
	}

	fn documentation(gen: &Generator) -> String {
		let rust = |expr| gen.rust(expr);
		let unit_blades = gen.types.unit_blades();
		format!(
			"\
	# Blade types\n\
	The blades that make up this geometric algebra.\n\
	\n\
	## Unary operations\n\
	{}\n\
	\n\
	## Multiplication tables\n\
	{}\n\
	",
			unary_table(&unit_blades, &rust),
			multiplication_tables(&unit_blades, &rust)
		)
	}

	fn declare_blades(gen: &Generator) -> String {
		gen.types
			.sblades()
			.iter()
			.map(|(name, sb)| declare_blade(gen, name, sb))
			.join("\n\n")
	}

	fn declare_blade(gen: &Generator, name: &str, sb: &SBlade) -> String {
		let is_scalar = sb.grade() == 0;
		let is_pseudoscalar = sb.grade() == gen.grammar.num_vecs();

		let mut code = String::new();
		if is_scalar {
			code += "/// The scalar type (real numbers).\n";
		} else if is_pseudoscalar {
			code += "/// The pseudo-scalar.\n"
		};

		let squares_to = sb.geometric_product(&sb, &gen.grammar);
		assert!(squares_to.is_scalar());
		let squares_to = squares_to.sign;
		code += &format!("/// Squares to {}.\n", squares_to);

		let mut derives = "Copy, Clone, Debug, PartialEq, PartialOrd, Neg, Add, Sub".to_string();
		if is_scalar {
			derives += ", Mul"; // NOTE: NOT the pseudoscalar (it may scale to zero)
		}
		code += &format!("#[derive({})]\n", derives);

		format!("{}pub struct {}(pub {});", code, name, gen.settings.float_type)
	}

	fn impl_blade_ops(gen: &Generator) -> String {
		Product::iter()
			.map(|prod| {
				format!(
					"// impl {} for blades:\n\n{}",
					prod.trait_name(),
					impl_blade_ops_product(gen, prod)
				)
			})
			.join(&format!("\n\n{}\n", CODE_SEPARATOR))
	}

	fn impl_blade_ops_product(gen: &Generator, product: Product) -> String {
		gen.types
			.sblades()
			.iter()
			.map(|lhs| {
				gen.types
					.sblades()
					.iter()
					.map(|rhs| impl_blade_product(gen, lhs, rhs, product))
					.join("\n\n")
			})
			.join("\n\n")
	}

	fn impl_blade_product(gen: &Generator, lhs: &(&str, SBlade), rhs: &(&str, SBlade), product: Product) -> String {
		let product_type = SBlade::product(product, &[lhs.1.clone(), rhs.1.clone()], &gen.grammar);

		if product_type.is_zero() {
			format!(
				r"
	impl {Trait}<{Rhs}> for {Lhs} {{
		type Output = Zero;
		fn {function_name}(self, _rhs: {Rhs}) -> Self::Output {{
			Zero {{}}
		}}
	}}
	",
				Lhs = lhs.0,
				Rhs = rhs.0,
				Trait = product.trait_name(),
				function_name = product.trait_function_name(),
			)
		} else {
			let (sign, output_sblade_name) = gen.types.get_blade(&product_type.blade).unwrap();
			let sign = product_type.sign * sign;
			assert_eq!(sign.abs(), 1);

			format!(
				r"
	impl {Trait}<{Rhs}> for {Lhs} {{
		type Output = {Output};
		fn {function_name}(self, rhs: {Rhs}) -> Self::Output {{
			{Output}({sign} self.0 * rhs.0)
		}}
	}}
	",
				Lhs = lhs.0,
				Rhs = rhs.0,
				Trait = product.trait_name(),
				function_name = product.trait_function_name(),
				Output = output_sblade_name,
				sign = if sign == -1 { "-" } else { "" }
			)
		}
	}
}

mod strct {
	use super::*;

	pub fn file(gen: &Generator, struct_name: &str, strct: &Struct) -> String {
		let documentation = with_line_prefixes("//! ", &documentation(gen, struct_name, strct).trim());

		let binops = gen
			.types
			.structs()
			.map(|(rhs_name, rhs_struct)| {
				format!(
					"// {} OP {}:\n\n{}\n",
					struct_name,
					rhs_name,
					Product::iter()
						.map(|prod| impl_struct_product(gen, &(struct_name, strct), &(rhs_name, rhs_struct), prod))
						.join("\n")
				)
			})
			.join(&format!("\n{}\n", CODE_SEPARATOR));

		format!(
			"\
		{}\n\n\
		use super::*;\n\n\
		{}\n\
		{}\n\
		{}\n",
			documentation,
			declare_struct(gen, struct_name, strct),
			CODE_SEPARATOR,
			binops
		)
	}

	fn documentation(gen: &Generator, struct_name: &str, strct: &Struct) -> String {
		let homo_ops = Product::iter()
			.filter_map(|product| {
				struct_product_type_signature(gen, &(struct_name, strct), &(struct_name, strct), product)
			})
			.join("\n");

		let hetero_ops = gen
			.types
			.structs()
			.filter_map(|(other_struct_name, other_strct)| {
				if struct_name == other_struct_name {
					None
				} else {
					Some(
						Product::iter()
							.flat_map(|product| {
								itertools::chain(
									struct_product_type_signature(
										gen,
										&(struct_name, strct),
										&(other_struct_name, other_strct),
										product,
									),
									struct_product_type_signature(
										gen,
										&(other_struct_name, other_strct),
										&(struct_name, strct),
										product,
									),
								)
							})
							.join("\n"),
					)
				}
			})
			.join("\n");

		format!(
			"\
		# {}\n\n\
		## Operations\n\
		```text\n\
		{}\n\
		{}\n\
		```\n\
		",
			struct_name, homo_ops, hetero_ops
		)
	}

	fn declare_struct(_gen: &Generator, struct_name: &str, strct: &Struct) -> String {
		// TODO: we can only implement Add, Sub if the struct has no Type::Constant
		let derives =
			"Copy, Clone, Debug, PartialEq, PartialOrd, derive_more::Neg, derive_more::Add, derive_more::Sub\n";
		let members = strct
			.iter()
			.map(|(member_name, member_type)| format!("\tpub {}: {},", member_name, member_type.name))
			.join("\n");
		format!(
			"\
	#[derive({})]\n\
	pub struct {} {{\n\
			{}\n\
	}}\n	",
			derives, struct_name, members
		)
	}

	pub fn struct_product_type_signature(
		gen: &Generator,
		lhs: &(&str, &Struct),
		rhs: &(&str, &Struct),
		product: Product,
	) -> Option<String> {
		let factors = vec![
			Expr::var(0, lhs.0, &Type::strct(lhs.1)),
			Expr::var(1, rhs.0, &Type::strct(rhs.1)),
		];
		let input_expr = Expr::Prod(product, factors);
		let input_code = input_expr.rust(&gen.ro);
		let output_expr = input_expr.simplify(Some(&gen.grammar)).typify(&gen.types, &gen.grammar);
		let type_name = type_name(gen, &output_expr)?;
		Some(format!("{} -> {}", input_code, type_name))
	}

	fn impl_struct_product(gen: &Generator, lhs: &(&str, &Struct), rhs: &(&str, &Struct), product: Product) -> String {
		let factors = vec![
			Expr::var(0, "self", &Type::strct(lhs.1)),
			Expr::var(1, "rhs", &Type::strct(rhs.1)), // TODO: name this snake_case(rhs.0) iff lhs type != rhs.type
		];
		let expr = Expr::Prod(product, factors);
		let expr = expr.simplify(Some(&gen.grammar)).typify(&gen.types, &gen.grammar);
		let code = expr.rust(&gen.ro);
		match type_name(gen, &expr) {
			Some(output_type_name) => format!(
				r"
		// {comment}
		impl {Trait}<{Rhs}> for {Lhs} {{
			type Output = {Output};
			fn {function_name}(self, rhs: {Rhs}) -> Self::Output {{
				{code}
			}}
		}}
		",
				comment = struct_product_type_signature(gen, lhs, rhs, product).unwrap(),
				Lhs = lhs.0,
				Rhs = rhs.0,
				Trait = product.trait_name(),
				function_name = product.trait_function_name(),
				Output = output_type_name,
				code = code,
			),
			None => format!(
				"// Omitted: {} {} {} = {}",
				lhs.0,
				product.trait_function_name(),
				rhs.0,
				code.replace('\n', " ")
			),
		}
	}
}

/// Returns None if the type is Zero or unknown
fn type_name(gen: &Generator, expr: &Expr) -> Option<String> {
	// println!("type_name({})", expr.rust(&gen.ro));
	let output_type = expr.typ(Some(&gen.grammar), Some(&gen.types));
	// println!("type_name({}) output_type: {:?}", expr.rust(&gen.ro), output_type);
	let output_type = output_type?;
	if output_type.is_zero() {
		None
	} else {
		Some(gen.types.type_name(&output_type).to_owned())
	}
}

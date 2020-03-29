use std::{collections::BTreeSet, error::Error, fs, io::prelude::*, path::Path};

use {itertools::Itertools, strum::IntoEnumIterator};

use generator::*;

const CODE_SEPARATOR: &str = "// ---------------------------------------------------------------------\n";

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

struct Generator {
	grammar: Grammar,
	types: Types,
	settings: Settings,
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
	};

	fs::create_dir_all(out_dir)?;

	let mut mod_file_contents = include_str!("../templates/lib.rs").to_owned();

	let mut mods = BTreeSet::new();
	mods.insert("traits".to_string());
	mods.insert("blades".to_string());

	write_file(include_str!("../templates/traits.rs"), &out_dir.join("traits.rs"))?;
	write_file(&blades_file(&gen), &out_dir.join("blades.rs"))?;

	mod_file_contents += "\n// Types:\n";
	for (struct_name, strct) in gen.types.structs() {
		let mod_name = struct_name.to_ascii_lowercase();
		let file_name = format!("{}.rs", mod_name);
		let file_contents = struct_file(&gen, struct_name, strct);
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

fn blades_file(gen: &Generator) -> String {
	format!(
		"{}\n\n{}\n\n{}\n\n{}{}\n",
		"use derive_more::{Neg, Add, Sub, Mul};",
		"use super::*;",
		declare_blades(gen),
		CODE_SEPARATOR,
		impl_blade_ops(gen),
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

	if is_scalar || is_pseudoscalar {
		code += "#[derive(Copy, Clone, Debug, PartialEq, Neg, Add, Sub, Mul)]\n";
	} else {
		code += "#[derive(Copy, Clone, Debug, PartialEq, Neg, Add, Sub)]\n";
	}

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
		.join(&format!("\n\n{}", CODE_SEPARATOR))
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

fn struct_file(gen: &Generator, struct_name: &str, strct: &Struct) -> String {
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
		.join(&format!("\n{}", CODE_SEPARATOR));
	format!(
		"{}\n\n{}\n{}{}\n",
		"use super::*;",
		declare_struct(gen, struct_name, strct),
		CODE_SEPARATOR,
		binops
	)
}

fn declare_struct(_gen: &Generator, struct_name: &str, strct: &Struct) -> String {
	let members = strct
		.iter()
		.map(|(member_name, member_type)| format!("\tpub {}: {},", member_name, member_type.name))
		.join("\n");
	format!("pub struct {} {{\n{}\n}}\n", struct_name, members)
}

fn impl_struct_product(gen: &Generator, lhs: &(&str, &Struct), rhs: &(&str, &Struct), product: Product) -> String {
	let factors = vec![
		Expr::var(0, "self", &Type::strct(lhs.1)),
		Expr::var(1, "rhs", &Type::strct(rhs.1)),
	];
	let expr = Expr::Prod(product, factors);
	let expr = expr.simplify(Some(&gen.grammar)).typify(&gen.types, &gen.grammar);

	if true {
		match &expr {
			Expr::StructInstance(output_struct) => format!(
				r"
		impl {Trait}<{Rhs}> for {Lhs} {{
			type Output = {Output};
			fn {function_name}(self, rhs: {Rhs}) -> Self::Output {{
				{code}
			}}
		}}
		",
				Lhs = lhs.0,
				Rhs = rhs.0,
				Trait = product.trait_name(),
				function_name = product.trait_function_name(),
				Output = output_struct.struct_name,
				code = expr.rust(),
			),
			_ => format!(
				"// Omitted: {} {} {} = {}",
				lhs.0,
				product.trait_function_name(),
				rhs.0,
				expr.rust()
			),
		}
	} else {
		let output_type = match expr.typ(Some(&gen.grammar)) {
			Some(typ) => typ,
			None => {
				return format!(
					"// Omitted: {} {} {} = {}",
					lhs.0,
					product.trait_function_name(),
					rhs.0,
					expr.rust()
				);
			}
		};
		if output_type.is_zero() {
			return format!("// Omitted: {} {} {} = 0", lhs.0, product.symbol(), rhs.0);
		}
		let output_type_name = gen.types.type_name(&output_type);

		format!(
			r"
impl {Trait}<{Rhs}> for {Lhs} {{
	type Output = {Output};
	fn {function_name}(self, rhs: {Rhs}) -> Self::Output {{
		{code}
	}}
}}
",
			Lhs = lhs.0,
			Rhs = rhs.0,
			Trait = product.trait_name(),
			function_name = product.trait_function_name(),
			Output = output_type_name,
			code = expr.rust(),
		)
	}
}

use std::{collections::BTreeSet, error::Error, fs, io::prelude::*, path::Path};

use itertools::Itertools;

use generator::*;

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
		"{}\n\n{}\n",
		"use derive_more::{Neg, Add, Sub, Mul};",
		declare_blades(gen)
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

fn struct_file(gen: &Generator, struct_name: &str, strct: &Struct) -> String {
	format!("{}\n\n{}\n", "use super::*;", declare_struct(gen, struct_name, strct))
}

fn declare_struct(_gen: &Generator, struct_name: &str, strct: &Struct) -> String {
	let members = strct
		.iter()
		.map(|(member_name, member_type)| format!("\tpub {}: {},", member_name, member_type.name))
		.join("\n");
	format!("pub struct {} {{\n{}\n}}\n", struct_name, members)
}

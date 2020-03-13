use std::{error::Error, fs::File, io::prelude::*, path::Path};

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

	std::fs::create_dir_all(out_dir)?;

	let mut lib_file_contents = include_str!("../templates/lib.rs").to_owned();

	write_file(include_str!("../templates/traits.rs"), &out_dir.join("traits.rs"))?;
	write_file(&declare_blades(&gen), &out_dir.join("blades.rs"))?;

	lib_file_contents += "\n// Types:\n";
	for td in gen.types.typedefs() {
		if let Type::Struct(members) = &td.typ {
			let mod_name = td.name.to_ascii_lowercase();
			let file_name = format!("{}.rs", mod_name);
			let file_contents = declare_struct(&gen, td, members);
			write_file(&file_contents, &out_dir.join(file_name))?;
			lib_file_contents += &format!("pub mod {};\n", mod_name);
		}
	}

	write_file(&lib_file_contents, &out_dir.join("lib.rs"))?;

	Ok(())
}

fn write_file(contents: &str, path: &Path) -> Result<(), Box<dyn Error>> {
	// Only write file if it has actually changed:
	match std::fs::read_to_string(path) {
		Ok(existing) if existing == contents => {}
		_ => File::create(&path)?.write_all(contents.as_bytes())?,
	}

	Ok(())
}

fn declare_blades(gen: &Generator) -> String {
	gen.types.blades().map(|b| declare_blade(gen, b)).join("\n\n")
}

fn declare_blade(gen: &Generator, td: &Typedef) -> String {
	let sb = td.typ.clone().into_sblade().unwrap();

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
		code += "#[derive(Copy, Clone, Debug, PartialEq, Neg, Add, Sub, Mil)]\n";
	} else {
		code += "#[derive(Copy, Clone, Debug, PartialEq, Neg, Add, Sub)]\n";
	}

	format!("{}pub struct {}(pub {});", code, td.name, gen.settings.float_type)
}

fn declare_struct(gen: &Generator, td: &Typedef, members: &[(String, Type)]) -> String {
	let members = members
		.iter()
		.map(|(name, typ)| format!("\t{}: {},", name, "TODO: TYPE NAME???"))
		.join("\n");
	format!("pub struct {} {{\n{}\n}}\n", td.name, members)
}

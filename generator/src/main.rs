use std::{collections::BTreeSet, error::Error, fs, io::prelude::*, path::Path};

use itertools::Itertools;

use generator::{gen::*, *};

fn main() -> Result<(), Box<dyn Error>> {
	let mut args = pico_args::Arguments::from_env();

	let grammar: String = args.value_from_str(["-g", "--grammar"])?;
	let out_dir: String = args.value_from_str(["-o", "--out_dir"])?;
	let out_dir = Path::new(&out_dir);

	let (grammar, types) = match grammar.as_str() {
		"pga2d" => generator::grammars::pga2d(),
		"pga3d" => generator::grammars::pga3d(),
		_ => panic!("Unknown grammar: '{}'", grammar),
	};

	let settings = Settings::default();
	let gen = Generator {
		grammar,
		types,
		settings,
		ro: RustOptions::rust(),
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
		mods.iter().map(|mod_name| format!("    {}::*,", mod_name)).join("\n")
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
		// eprintln!("No change to '{}'", final_path.display());
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

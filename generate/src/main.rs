use generate::*;

// ----------------------------------------------------------------------------

fn main() {
	let generator = GeneratorBuilder::pga_2d().build();
	generator.print();
	generator.print_ops();

	// 	println!();
	// 	println!();
	// 	let grammar = generator.grammar();
	// 	let point_type = generator.type_from_name("Point").unwrap();
	// 	let l = Sum::instance("l", &point_type.members);
	// 	let r = Sum::instance("r", &point_type.members);
	// 	println!("let l = {}", generator.format_typed_value(&l));
	// 	println!("let r = {}", generator.format_typed_value(&r));
	// 	println!("l.dual(): {}", generator.format_typed_value(&l.dual(grammar)));
	// 	println!("r.dual(): {}", generator.format_typed_value(&r.dual(grammar)));
	// 	println!(
	// 		"l.dual() ^ r.dual(): {}",
	// 		generator.format_typed_value(&l.dual(grammar).outer(r.dual(grammar), grammar))
	// 	);
	// 	println!(
	// 		"(l.dual() ^ r.dual()).dual(): {}",
	// 		generator.format_typed_value(&l.dual(grammar).outer(r.dual(grammar), grammar).dual(grammar))
	// 	);
}

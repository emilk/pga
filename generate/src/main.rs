use generate::*;

// ----------------------------------------------------------------------------

fn main() {
	// let generator = GeneratorBuilder::pga_2d().build();
	let generator = GeneratorBuilder::pga_3d().build();
	generator.print();
	generator.print_ops(Generate::Signature);

	// TODO: start not with a generator, but by manually pulling out parts of a grammar, like below:

	// explore square of a point
	// println!();
	// println!();
	// let grammar = generator.grammar();
	// let point = generator.type_from_name("Point").unwrap();
	// let p = point.instance("p");
	// println!("p * p = {}", generator.format_typed_value(&p.clone().mul(p, grammar)));
	// 	println!();
	// 	println!();
	// 	let grammar = generator.grammar();
	// 	let point_type = generator.type_from_name("Point").unwrap();
	// 	let l = point_type.instance("l");
	// 	let r = point_type.instance("r");
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

	// explore 3d point JOIN point:
	// println!();
	// println!();
	// let grammar = generator.grammar();
	// let point_type = generator.type_from_name("Point").unwrap();
	// let l = point_type.instance("l");
	// let r = point_type.instance("r");
	// println!("let l = {}", generator.format_typed_value(&l));
	// println!("let r = {}", generator.format_typed_value(&r));
	// println!("l.dual(): {}", generator.format_typed_value(&l.dual(grammar)));
	// println!("r.dual(): {}", generator.format_typed_value(&r.dual(grammar)));
	// println!(
	// 	"l.dual() ^ r.dual(): {}",
	// 	generator.format_typed_value(&l.dual(grammar).outer(r.dual(grammar), grammar))
	// );
	// println!(
	// 	"(l.dual() ^ r.dual()).dual(): {}",
	// 	generator.format_typed_value(&l.dual(grammar).outer(r.dual(grammar), grammar).dual(grammar))
	// );

	// explore 3d point JOIN point:
	// println!();
	// println!();
	// let grammar = generator.grammar();
	// let plane = generator.type_from_name("Plane").unwrap();
	// let l = plane.instance("l");
	// let r = plane.instance("r");
	// println!("plane * plane = {}", generator.format_typed_value(&l.mul(r, grammar)));
}

use itertools::Itertools;

use generate::*;

// ----------------------------------------------------------------------------

/// The standard form is the integers (p, m, z)
/// where p : number of vectors that square to +1
/// where m : number of vectors that square to -1
/// where z : number of vectors that square to 0
/// We allow others orders.
/// (3, 0, 0): 3d euclidean vector space
/// (3, 0, 1): 3d projective geometric algebra
fn generate(grammar: &Grammar) {
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
	// Scalars are zero-dimensions, basis vectors are 1-dimensional, bivectors

	let mut blades: Vec<Blade> = (0..grammar.dims())
		.map(|_| Some(false).into_iter().chain(Some(true)))
		.multi_cartesian_product()
		.map(|bools| Blade::from_bools(&bools))
		.map(|blade| grammar.simplify(SignedBlade::unit(&blade)).blade)
		.collect();
	blades.sort();
	let blades = blades;

	println!();
	println!("Blades:");
	for blade in &blades {
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

	let line_type = Type::from_blades(vec![blades[1].clone(), blades[2].clone(), blades[3].clone()]);
	let point_type = Type::from_blades(vec![blades[4].clone(), blades[5].clone(), blades[6].clone()]);
	let transform_type = Type::from_blades(vec![
		blades[0].clone(),
		blades[4].clone(),
		blades[5].clone(),
		blades[6].clone(),
	]);

	let a = MultiVec::instance("a", &line_type);
	let b = MultiVec::instance("b", &line_type);
	println!();
	println!("line a * line b = {}", (&a * &b).simplify(grammar));

	let t = MultiVec::instance("t", &transform_type);
	let p = MultiVec::instance("p", &point_type);
	println!();
	println!("transform t sandwich point p = {}", (t.sandwich(&p)).simplify(grammar));

	// let t = MultiVec::instance("t", &point_type);
	// let p = MultiVec::instance("p", &point_type);
	// println!();
	// println!("point t sandwich point p = {}", (t.sandwich(&p)).simplify(grammar));
}

fn main() {
	// 2D PGA
	generate(&GrammarBuilder::pga_2d().build());
}

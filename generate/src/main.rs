use itertools::Itertools;

use generate::*;

// struct Factor {
// 	variable: String, // the name of a multivector variable e.g. "a", "b", "point"
// 	blade: Blade,     // The blade/dimension in the variable
// }

// // ----------------------------------------------------------------------------

// struct Product(Sign, Vec<Factor>);

// impl Product {
// 	fn one() -> Self {
// 		Product(Sign::Positive, vec![])
// 	}
// }

// impl fmt::Display for Product {
// 	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
// 		let factors = self.1.iter().map(ToString::to_string).join("·");
// 		match self.sign {
// 			Product::Negative => write!(f, "-{}", factors),
// 			Product::Zero => "0".fmt(f),
// 			Product::Positive => factors.fmt(f),
// 		}
// 	}
// }

// impl std::ops::Neg for Product {
// 	type Output = Product;
// 	fn neg(self) -> Product {
// 		Product(-self.0, self.1)
// 	}
// }

// impl std::ops::Mul for Product {
// 	type Output = Self;

// 	fn mul(self, rhs: Self) -> Self {
// 		Product(self.0 * rhs.0, concat(self.1, rhs.1)).simplify()
// 	}
// }

// impl std::ops::MulAssign<Product> for Product {
// 	fn mul_assign(&mut self, rhs: Product) {
// 		*self = *self * rhs;
// 	}
// }

// ----------------------------------------------------------------------------

// struct Sum(Vec<Product>);

// impl Sum {
// 	fn one() -> Self {
// 		Sum(vec![Product::one()])
// 	}
// }

// impl fmt::Display for Sum {
// 	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
// 		match self {
// 			Sum::Negative => "-1".fmt(f),
// 			Sum::Zero => "0".fmt(f),
// 			Sum::Positive => "+1".fmt(f),
// 		}
// 	}
// }

// impl std::ops::Neg for Sum {
// 	type Output = Sum;
// 	fn neg(self) -> Sum {
// 		match self {
// 			Sum::Negative => Sum::Positive,
// 			Sum::Zero => Sum::Zero,
// 			Sum::Positive => Sum::Negative,
// 		}
// 	}
// }

// impl std::ops::Mul for Sum {
// 	type Output = Self;

// 	fn mul(self, rhs: Self) -> Self {
// 		use Sum::*;
// 		match (self, rhs) {
// 			(Positive, Positive) | (Negative, Negative) => Positive,
// 			(Negative, Positive) | (Positive, Negative) => Negative,
// 			_ => Zero,
// 		}
// 	}
// }

// impl std::ops::MulAssign<Sum> for Sum {
// 	fn mul_assign(&mut self, rhs: Sum) {
// 		*self = *self * rhs;
// 	}
// }

// ----------------------------------------------------------------------------

// ----------------------------------------------------------------------------

// struct Multivector(Vec<FactoredBlade>);

// fn geometric_product(a: &Multivector, b: &Multivector) {
// 	todo!();
// }

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
		.map(|blade| {
			grammar
				.simplify(SignedBlade {
					sign: Sign::Positive,
					blade,
				})
				.blade
		})
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
}

fn main() {
	// 2D PGA
	generate(&GrammarBuilder::pga_2d().build());
}

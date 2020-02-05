use std::fmt;

use itertools::Itertools;

// ----------------------------------------------------------------------------

/// Definitions of axes: each axis squares to one of these.
/// The number of axes and their signs define your algebra.
#[derive(Copy, Clone, Eq, PartialEq)]
enum Sign {
	Negative,
	Zero,
	Positive,
}

impl fmt::Display for Sign {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Sign::Negative => "-1".fmt(f),
			Sign::Zero => "0".fmt(f),
			Sign::Positive => "+1".fmt(f),
		}
	}
}

impl std::ops::Neg for Sign {
	type Output = Sign;
	fn neg(self) -> Sign {
		match self {
			Sign::Negative => Sign::Positive,
			Sign::Zero => Sign::Zero,
			Sign::Positive => Sign::Negative,
		}
	}
}

impl std::ops::Mul for Sign {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self {
		use Sign::*;
		match (self, rhs) {
			(Positive, Positive) | (Negative, Negative) => Positive,
			(Negative, Positive) | (Positive, Negative) => Negative,
			_ => Zero,
		}
	}
}

impl std::ops::MulAssign<Sign> for Sign {
	fn mul_assign(&mut self, rhs: Sign) {
		*self = *self * rhs;
	}
}

// ----------------------------------------------------------------------------

/// A scalar, vector, bivector, trivector etc.
/// A blade of grad K is the result of a wedge product of K different vectors
/// A set of integers
/// 1:   [false, false, false]
/// e0:  [true,  false, false]
/// e12: [false, true,  true]
#[derive(Clone, Eq, PartialEq)]
struct Blade(Vec<bool>);

impl Blade {
	fn from_bases(dimensionality: usize, set: &[usize]) -> Self {
		let mut v = vec![false; dimensionality];
		for b in set {
			v[*b] = true;
		}
		Blade(v)
	}

	/// Returns the basis vectors of this blade
	fn bases(&self) -> Vec<usize> {
		self.0
			.iter()
			.enumerate()
			.flat_map(|(i, set)| if *set { Some(i) } else { None })
			.collect()
	}

	// 0 for scalar, 1 for vector, 2 for multivector etc
	fn grade(&self) -> usize {
		self.0.iter().filter(|s| **s).count()
	}

	fn dual(&self) -> Blade {
		Blade(self.0.iter().map(|s| !s).collect())
	}
}

impl Ord for Blade {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		if self.grade() != other.grade() {
			self.grade().cmp(&other.grade())
		} else {
			self.0.cmp(&other.0).reverse()
		}
	}
}

impl PartialOrd for Blade {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl fmt::Display for Blade {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let numbers: Vec<String> = self
			.0
			.iter()
			.enumerate()
			.flat_map(|(i, set)| if *set { Some(i.to_string()) } else { None })
			.collect();
		if numbers.is_empty() {
			// Real/Scalar
			"1".fmt(f)
		} else {
			format!("e{}", numbers.iter().join("")).fmt(f)
		}
	}
}

// ----------------------------------------------------------------------------

// TODO: rename
// A blade with a magnitude (e.g 42 * e2)
#[derive(Clone)]
struct SignedBlade {
	magnitude: Sign,
	blade: Blade,
}

impl SignedBlade {
	/// One times the given blade
	fn unit(blade: &Blade) -> Self {
		Self {
			magnitude: Sign::Positive,
			blade: blade.clone(),
		}
	}

	fn grade(&self) -> usize {
		self.blade.grade()
	}

	fn dual(&self) -> SignedBlade {
		Self {
			magnitude: self.magnitude,
			blade: self.blade.dual(),
		}
	}

	/// Reverse the order of the vector factors in this blade
	fn reverse(&self) -> Self {
		let mut b = self.clone();
		let r = b.grade();
		if r > 1 {
			// After reversing the order, we want to sort again.
			let num_swaps = r * (r - 1) / 2;
			if num_swaps % 2 == 1 {
				// Odd number of swaps => sign change
				b.magnitude = -b.magnitude;
			}
		}
		b
	}

	/// geometric product (normal multiplication)
	fn geometric(&self, other: &SignedBlade, vectors_squared: &[Sign]) -> Self {
		// each blade is the product of its vectors.
		// so all we need to do is concatenate the numbers and normalize.
		let mut numbers: Vec<usize> = self
			.blade
			.bases()
			.into_iter()
			.chain(other.blade.bases().into_iter())
			.collect();

		// We want to sort the basis numbers.
		// Multiplication is anti-commutative so each time we swap we need to flip the sign.
		// So bubble-sort!
		let mut magnitude = self.magnitude * other.magnitude;
		for _ in 0..numbers.len() {
			for i in 0..numbers.len() - 1 {
				if numbers[i] > numbers[i + 1] {
					numbers.swap(i, i + 1);
					magnitude = -magnitude;
				}
			}
		}

		// Now we collapse adjacent basis vectors (squaring them):
		let mut bases = vec![];
		for num in numbers {
			if bases.last() == Some(&num) {
				magnitude *= vectors_squared[num];
				bases.pop();
			} else {
				bases.push(num);
			}
		}

		Self {
			magnitude,
			blade: Blade::from_bases(vectors_squared.len(), &bases),
		}
	}

	fn dot(&self, other: &SignedBlade, vectors_squared: &[Sign]) -> Self {
		// The dot product is the K grade of the geometric product,
		// where K is the absolute difference in grades between the operands.
		let k = ((self.grade() as i64) - (other.grade() as i64)).abs() as usize;
		let mut prod = self.geometric(other, vectors_squared);
		if prod.blade.grade() > k {
			prod.magnitude = Sign::Zero;
		}
		prod
	}

	/// outer / wedge
	fn outer(&self, other: &SignedBlade, vectors_squared: &[Sign]) -> Self {
		let k = self.grade() + other.grade();
		let mut prod = self.geometric(other, vectors_squared);
		if prod.blade.grade() < k {
			prod.magnitude = Sign::Zero;
		}
		prod
	}

	fn regressive(&self, other: &SignedBlade, vectors_squared: &[Sign]) -> Self {
		self.dual().outer(&other.dual(), vectors_squared).dual()
	}
}

impl fmt::Display for SignedBlade {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self.magnitude {
			Sign::Positive => self.blade.fmt(f),
			Sign::Zero => "0".fmt(f),
			Sign::Negative => format!("-{}", self.blade).fmt(f),
		}
	}
}

// ----------------------------------------------------------------------------

/// The standard form is the integers (p, m, z)
/// where p : number of vectors that square to +1
/// where m : number of vectors that square to -1
/// where z : number of vectors that square to 0
/// We allow others orders.
/// (3, 0, 0): 3d euclidean vector space
/// (3, 0, 1): 3d projective geometric algebra
fn generate(vectors_squared: &[Sign]) {
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
	for (i, sign) in vectors_squared.iter().enumerate() {
		println!("  e{}² = {:>2}", i, sign);
	}
	// Scalars are zero-dimensions, basis vectors are 1-dimensional, bivectors

	let mut blades: Vec<Blade> = (0..vectors_squared.len())
		.map(|_| Some(false).into_iter().chain(Some(true)))
		.multi_cartesian_product()
		.map(Blade)
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
		println!("  !{:<5} = {}", base, base.dual());
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
			print!("{:<8}", a.geometric(b, vectors_squared));
		}
		println!();
	}

	println!();
	println!("Inner / dot product multiplication table (left side | top row):");
	for a in &unit_blades {
		print!("  ");
		for b in &unit_blades {
			print!("{:<8}", a.dot(b, vectors_squared));
		}
		println!();
	}

	println!();
	println!("Outer product table (left side ^ top row):");
	for a in &unit_blades {
		print!("  ");
		for b in &unit_blades {
			print!("{:<8}", a.outer(b, vectors_squared));
		}
		println!();
	}

	println!();
	println!("Regressive product (join) multiplication table (right side & bottom row):");
	for a in &unit_blades {
		print!("  ");
		for b in &unit_blades {
			print!("{:<8}", a.regressive(b, vectors_squared));
		}
		println!();
	}
}

fn main() {
	// 2D PGA
	generate(&[Sign::Zero, Sign::Positive, Sign::Positive]);
}

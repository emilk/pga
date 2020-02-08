use std::{collections::BTreeMap, fmt, ops::Mul};

use {derive_more::Display, itertools::Itertools};

// -----------------------------------------------------------------------------

#[macro_export]
macro_rules! collect {
    ($($expr: expr),*) => {
        vec![$($expr),*].into_iter().collect()
    };
    ($($expr: expr,)*) => {
        vec![$($expr),*].into_iter().collect()
    }
}

// -----------------------------------------------------------------------------

/// Which base vector (e0, e1 or e2?)
#[derive(Copy, Clone, Debug, Display, Eq, Ord, PartialEq, PartialOrd)]
struct VecIdx(usize);
//type BladeIdx = usize;

// -----------------------------------------------------------------------------

/// Represents the geometric product of some vectors in the given order.
/// The empty blade is the real dimensions.
#[derive(Clone, Eq, PartialEq)]
struct Blade(Vec<VecIdx>);

impl Blade {
	// Is this
	fn from_bools(bools: &[bool]) -> Self {
		Self(
			bools
				.iter()
				.enumerate()
				.filter_map(|(i, &set)| if set { Some(VecIdx(i)) } else { None })
				.collect(),
		)
	}

	fn as_bools(&self, grammar: &Grammar) -> Vec<bool> {
		assert!(self.is_simple());
		(0..grammar.dims()).map(|i| self.0.contains(&VecIdx(i))).collect()
	}

	// Is this blade as simple (short) as possible?
	// NOTE: may still not be normalized (canonical order, as specified by grammar)
	fn is_simple(&self) -> bool {
		for i in 0..self.0.len() {
			for j in i + 1..self.0.len() {
				if self.0[i] == self.0[j] {
					return false; // Duplicate
				}
			}
		}
		true
	}

	// // does not mean sorted according to grammar! TODO
	// fn is_sorted(&self) -> bool {
	// 	// self.0.is_sorted() // TODO
	// 	let mut sorted = self.0.clone();
	// 	sorted.sort();
	// 	self.0 == sorted
	// }

	/// 0 for scalar, 1 for vector, 2 for multivector etc.
	/// Only trustworthy for normalized / simplified blades!
	fn grade(&self) -> usize {
		assert!(self.is_simple());
		self.0.len()
	}

	/// TODO: store dimensionality in blade so we don't need to pass in the grammar here!
	fn dual(&self, grammar: &Grammar) -> Blade {
		// Blade(self.0.iter().map(|v| grammar.dual(*v)).collect())
		let bools: Vec<bool> = self.as_bools(grammar).into_iter().map(|s| !s).collect();
		Blade::from_bools(&bools)
	}

	/// geometric multiplication, produces the geometric product
	fn geometric(&self, other: &Blade) -> Self {
		// each blade is the product of its vectors so all we need to do is concatenate the numbers!
		Blade(self.0.iter().copied().chain(other.0.iter().copied()).collect())
	}
}

// TODO: use grammar for this instead of overriding Ord
impl Ord for Blade {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		if self.0.len() != other.0.len() {
			// Short blades first
			self.0.len().cmp(&other.0.len())
		} else {
			// TODO: remove this HACK for e12 vs e20 order
			let self_sorted: Vec<VecIdx> = self.0.iter().copied().sorted().collect();
			let other_sorted: Vec<VecIdx> = other.0.iter().copied().sorted().collect();
			if self_sorted != other_sorted {
				self_sorted.cmp(&other_sorted)
			} else {
				self.0.cmp(&other.0)
			}
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
		if self.0.is_empty() {
			// Real/Scalar
			"1".fmt(f)
		} else {
			format!("e{}", self.0.iter().join("")).fmt(f)
		}
	}
}

// -----------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------

struct Grammar {
	/// what you get when you sign the input vectors,
	/// e.g. 0++ would specify the 2d gpa of e0^2=0  e1^2=1  e2^2=1
	vectors_squared: Vec<Sign>,

	/// Optionally override the order of the vector bases in a multivector,
	/// e.g. maybe you prefer the output to use `e20` over `-e02`.
	blade_version: BTreeMap<Blade, SignedBlade>,
	// TODO: allow changing the order (in multiplication tables, types etc) of e.e. `e20` and `e12`.
}

impl Grammar {
	/// Projective Geometric Algebra in 2d.
	/// e0^2=0  e1^2=1  e2^2=1
	fn pga_2d() -> Self {
		Self {
			vectors_squared: vec![Sign::Zero, Sign::Positive, Sign::Positive],
			// TODO: automatically figure out sign!
			blade_version: collect![(
				Blade(vec![VecIdx(0), VecIdx(2)]),
				SignedBlade {
					sign: Sign::Negative,
					blade: Blade(vec![VecIdx(2), VecIdx(0)])
				}
			)],
			// blade_version: collect![], //TODO
		}
	}

	/// number of vectors, dimensionality of the vector space
	fn dims(&self) -> usize {
		self.vectors_squared.len()
	}

	// fn dual(&self, v: VecIdx) -> VecIdx {
	// 	assert!(v.0 < self.dims());
	// 	VecIdx(self.dims() - v.0 - 1)
	// }

	fn simplify(&self, mut value: SignedBlade) -> SignedBlade {
		// We want to sort the basis numbers.
		// Multiplication is anti-commutative so each time we swap we need to flip the sign.
		// So bubble-sort!
		for _ in 0..value.blade.0.len() {
			for i in 0..value.blade.0.len() - 1 {
				if value.blade.0[i] > value.blade.0[i + 1] {
					value.blade.0.swap(i, i + 1);
					value.sign = -value.sign;
				}
			}
		}

		// Now we collapse adjacent basis vectors (squaring them):
		let mut sign = value.sign;
		let mut bases = vec![];
		for num in value.blade.0 {
			if bases.last() == Some(&num) {
				sign *= self.vectors_squared[num.0];
				bases.pop();
			} else {
				bases.push(num);
			}
		}
		let blade = Blade(bases);

		if let Some(prefered) = self.blade_version.get(&blade) {
			sign * prefered.clone()
		} else {
			SignedBlade { sign, blade }
		}
	}
}

// // ----------------------------------------------------------------------------

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

// TODO: rename
// A blade with a sign (e.g 42 * e2)
#[derive(Clone)]
struct SignedBlade {
	sign: Sign,
	blade: Blade,
}

impl SignedBlade {
	/// One times the given blade
	fn unit(blade: &Blade) -> Self {
		Self {
			sign: Sign::Positive,
			blade: blade.clone(),
		}
	}

	fn grade(&self) -> usize {
		self.blade.grade()
	}

	fn dual(&self, grammar: &Grammar) -> SignedBlade {
		Self {
			sign: self.sign,
			blade: self.blade.dual(grammar),
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
				b.sign = -b.sign;
			}
		}
		b
	}

	/// geometric product (normal multiplication)
	fn geometric(&self, other: &SignedBlade, grammar: &Grammar) -> Self {
		grammar.simplify(Self {
			sign: self.sign * other.sign,
			blade: self.blade.geometric(&other.blade),
		})
	}

	fn dot(&self, other: &SignedBlade, grammar: &Grammar) -> Self {
		// The dot product is the K grade of the geometric product,
		// where K is the absolute difference in grades between the operands.
		let k = ((self.grade() as i64) - (other.grade() as i64)).abs() as usize;
		let mut prod = self.geometric(other, grammar);
		if prod.blade.grade() > k {
			prod.sign = Sign::Zero;
		}
		prod
	}

	/// outer / wedge
	fn outer(&self, other: &SignedBlade, grammar: &Grammar) -> Self {
		let k = self.grade() + other.grade();
		let mut prod = self.geometric(other, grammar);
		if prod.blade.grade() < k {
			prod.sign = Sign::Zero;
		}
		prod
	}

	fn regressive(&self, other: &SignedBlade, grammar: &Grammar) -> Self {
		self.dual(grammar).outer(&other.dual(grammar), grammar).dual(grammar)
	}
}

impl fmt::Display for SignedBlade {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self.sign {
			Sign::Positive => self.blade.fmt(f),
			Sign::Zero => "0".fmt(f),
			Sign::Negative => format!("-{}", self.blade).fmt(f),
		}
	}
}

impl Mul<SignedBlade> for Sign {
	type Output = SignedBlade;
	#[inline(always)]
	fn mul(self, right: SignedBlade) -> Self::Output {
		SignedBlade {
			sign: self * right.sign,
			blade: right.blade,
		}
	}
}

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
	for (i, sign) in grammar.vectors_squared.iter().enumerate() {
		println!("  e{}² = {:>2}", i, sign);
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
	generate(&Grammar::pga_2d());
}

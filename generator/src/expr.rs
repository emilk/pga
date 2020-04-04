use crate::*;

/// A value expression, e.g. `5 + X ^ (2 * Y - 1)`
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Expr {
	/// Variable of a given type
	Var {
		/// This is the sort order when rearranging expressions.
		/// You can use this to always place "self" to the left of "rhs", for instance.
		order: usize,
		name: String,
		typ: Type,
	},
	// A unit length base vector
	Vec(VecIdx),

	/// Indicates a scalar times something.
	/// Wedge(vec![X, 3, Y, 4]) simplifies to Term(Wedge(vec![X, Y]), 12)
	/// In its simplest form, the scalar is never 0 or 1
	/// 0 == Sum(vec![])
	/// 1 == Prod(_, vec![])
	Term(Box<Expr>, i32),

	/// Unary operation, e.g. a complement or a reverse
	Unary(Unary, Box<Expr>),

	Sum(Vec<Expr>),

	Prod(Product, Vec<Expr>),

	/// An instance of a struct, e.g. `Point {  x: ..., y: ... }` etc.
	StructInstance(StructInstance),
}

/// An instance of a struct, e.g. `Point {  x: ..., y: ... }` etc.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StructInstance {
	pub struct_name: String,
	pub strct: Struct,
	pub members: Vec<(String, Expr)>,
}

impl std::cmp::Ord for StructInstance {
	fn cmp(&self, other: &StructInstance) -> std::cmp::Ordering {
		self.members.cmp(&other.members)
	}
}

impl std::cmp::PartialOrd for StructInstance {
	fn partial_cmp(&self, other: &StructInstance) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl Expr {
	pub fn zero() -> Expr {
		Expr::Sum(vec![])
	}

	pub fn one() -> Expr {
		Expr::Prod(Product::Geometric, vec![])
	}

	pub fn scalar(s: i32) -> Self {
		match s {
			0 => Self::zero(),
			1 => Self::one(),
			s => Expr::Term(Expr::one().into(), s),
		}
	}

	pub fn vec(vi: VecIdx) -> Self {
		Expr::Vec(vi)
	}

	/// One magnitude sblade
	pub fn sblade(sblade: &SBlade) -> Self {
		let expr = match sblade.blade.grade() {
			0 => Expr::one(),
			1 => Expr::Vec(sblade.blade[0]),
			_ => Expr::wedge(sblade.blade.vecs().iter().copied().map(Expr::Vec).collect()),
		};
		match sblade.sign {
			-1 => expr.negate(),
			0 => Expr::zero(),
			1 => expr,
			_ => unreachable!(),
		}
	}

	pub fn var(order: usize, name: impl ToString, typ: &Type) -> Self {
		Expr::Var {
			order,
			name: name.to_string(),
			typ: typ.clone(),
		}
	}

	pub fn unary(unary: Unary, expr: Expr) -> Self {
		Expr::Unary(unary, expr.into())
	}

	pub fn geometric(factors: Vec<Expr>) -> Self {
		Expr::Prod(Product::Geometric, factors)
	}

	pub fn dot(factors: Vec<Expr>) -> Self {
		Expr::Prod(Product::Dot, factors)
	}

	/// outer product
	pub fn wedge(factors: Vec<Expr>) -> Self {
		Expr::Prod(Product::Wedge, factors)
	}

	/// also known as the regressive product
	pub fn antiwedge(factors: Vec<Expr>) -> Self {
		Expr::Prod(Product::AntiWedge, factors)
	}

	/// Note: self must be simplified
	pub fn is_zero(&self) -> bool {
		self == &Self::zero()
	}

	/// Note: self must be simplified
	pub fn is_one(&self) -> bool {
		match self {
			Expr::Prod(Product::Geometric, factors) if factors.is_empty() => true,
			_ => false,
		}
	}

	pub fn is_negation(&self) -> bool {
		match self {
			Expr::Term(_op, s) => *s < 0,
			_ => false,
		}
	}

	pub fn negate(self) -> Self {
		match self {
			Expr::Term(expr, -1) => *expr,
			Expr::Term(expr, s) => Expr::Term(expr, -s),
			expr => Expr::Term(expr.into(), -1),
		}
	}

	pub fn as_scalar(&self) -> Option<i32> {
		match self {
			Expr::Term(expr, s) if expr.is_one() => Some(*s),
			_ => None,
		}
	}

	/// Returns this Expr in terms of a multiple of a blade, if possible
	pub fn as_sblade(&self, g: &Grammar) -> Option<SBlade> {
		match self {
			Expr::Var { .. } => None,
			Expr::Vec(vi) => Some(SBlade::vec(*vi)),
			Expr::Term(expr, s) => {
				if let Some(sblade) = expr.as_sblade(g) {
					Some(*s * sblade)
				} else {
					None
				}
			}
			Expr::Unary(unary, expr) => Some(expr.as_sblade(g)?.unary(*unary, g)),
			Expr::Sum(terms) => {
				if terms.is_empty() {
					Some(SBlade::zero())
				} else if terms.len() == 1 {
					terms[0].as_sblade(g)
				} else {
					None // assuming we are simplified
				}
			}
			Expr::Prod(product, factors) => {
				let sblades: Option<Vec<SBlade>> = factors.iter().map(|f| f.as_sblade(g)).collect();
				let sblades = sblades?;
				Some(SBlade::product(*product, &sblades, g))
			}
			Expr::StructInstance { .. } => None,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	use crate::sblade::tests::sb;

	#[test]
	fn test_as_sblade() {
		let grammar = Grammar(vec![0, 1, 1, 1]);
		let g = &grammar;
		let v0 = VecIdx(0);
		let v1 = VecIdx(1);
		// let v2 = VecIdx(2);
		// let v3 = VecIdx(3);

		// assert_eq!(Expr::dot(vec![Expr::vec(v0), Expr::vec(v0)]).as_sblade(g), Some(sb("0")));
		// assert_eq!(Expr::dot(vec![Expr::vec(v1), Expr::vec(v1)]).as_sblade(g), Some(sb("1")));
		assert_eq!(
			Expr::wedge(vec![Expr::vec(v0), Expr::vec(v0)]).as_sblade(g),
			Some(sb("0"))
		);
		assert_eq!(
			Expr::wedge(vec![Expr::vec(v1), Expr::vec(v1)]).as_sblade(g),
			Some(sb("0"))
		);
		assert_eq!(
			Expr::geometric(vec![Expr::vec(v0), Expr::vec(v0)]).as_sblade(g),
			Some(sb("0"))
		);
		assert_eq!(
			Expr::geometric(vec![Expr::vec(v1), Expr::vec(v1)]).as_sblade(g),
			Some(sb("s"))
		);
		assert_eq!(
			Expr::wedge(vec![Expr::vec(v0), Expr::vec(v1)]).as_sblade(g),
			Some(sb("e01"))
		);
	}
}

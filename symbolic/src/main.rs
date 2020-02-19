/// The scalar type we use for symbolic reasoning.
// type S = i32;

/// Which base vector (e0, e1 or e2?)
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct VecIdx(usize);

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Op {
	/// Indicated a scalar times something.
	/// Prod(vec![X, 3, Y, 4]) simplifies to Term(Prod(vec![X, Y]), 12)
	/// In its simplest form, the scalar is never 0 or 1
	/// 0 == Sum(vec![])
	/// 1 == Prod(vec![])
	Term(Box<Op>, i32),

	// NOTE: not a blade(!)
	Vec(VecIdx),
	// Var(Variable),

	// Neg(Box<Op>),
	// Dual(Box<Op>),
	// Rev(Box<Op>),
	Sum(Vec<Op>),
	Prod(Vec<Op>),
	// Dot(Vec<Op>),
	// Wedge(Vec<Op>),

	// AntiProd(Vec<Op>),
	// AntiDot(Vec<Op>),
	// AntiWedge(Vec<Op>),
}

/// A type is some sort of multivector.
/// A value is a linear combination of types.
#[derive(Clone, Debug)]
enum Type {
	Zero,
	// One,
	/// The scalar type, i.e. 1
	S,
	/// The vector types
	Vec(VecIdx),
	// /// Tuple-type, linear combination of blades
	// /// Blades(vec![])       = () = Zero
	// /// Blades(vec![S])      = (S)
	// /// Blades(vec![S, E02]) = (S, E02)
	// Blades(Vec<Type>),
	// Struct(Struct),
}

/// named members
// enum Struct(Vec<String, Type>)

#[derive(Clone, Debug)]
struct Typedef {
	name: String,
	typ: Type,
}

/// In order of preference (first match).
#[derive(Clone, Debug, Default)]
struct Types(Vec<Typedef>);

/// what you get when you square the input vectors,
/// e.g. [0, 1, 1] would specify the 2d gpa of e0^2=0  e1^2=1  e2^2=1
pub struct Grammar(Vec<i32>);

// ----------------------------------------------------------------------------

impl Type {
	fn one(&self) -> Op {
		match self {
			Type::S => Op::one(),
			Type::Vec(vi) => Op::Vec(*vi),
			_ => panic!(),
		}
	}
}

impl Types {
	fn insert(&mut self, name: &str, typ: Type) {
		self.0.push(Typedef {
			name: name.to_string(),
			typ,
		});
	}

	fn get_typedef(&self, name: &str) -> &Typedef {
		self.0.iter().find(|td| td.name == name).unwrap()
	}

	fn get(&self, name: &str) -> &Type {
		&self.get_typedef(name).typ
	}

	fn vec_name(&self, vi: VecIdx) -> &str {
		self.0
			.iter()
			.find(|td| match td.typ {
				Type::Vec(v) if v == vi => true,
				_ => false,
			})
			.map(|td| td.name.as_str())
			.unwrap()
	}
}

impl Op {
	fn rust(&self, t: &Types) -> String {
		use itertools::Itertools;
		match self {
			// Op::S(s) => s.to_string(),
			Op::Term(op, s) => {
				if **op == Op::one() {
					s.to_string()
				} else {
					format!("{} * {}", s, op.rust(t))
				}
			}
			Op::Vec(vi) => t.vec_name(*vi).to_string(),
			Op::Sum(terms) => {
				if terms.is_empty() {
					"0".to_string()
				} else {
					terms.iter().map(|term| term.rust(t)).join(" + ")
				}
			}
			Op::Prod(factors) => {
				if factors.is_empty() {
					"1".to_string()
				} else {
					factors.iter().map(|factor| factor.rust(t)).join(" * ")
				}
			}
		}
	}

	fn zero() -> Op {
		Op::Sum(vec![])
	}

	fn one() -> Op {
		Op::Prod(vec![])
	}

	fn scalar(s: i32) -> Op {
		match s {
			0 => Self::zero(),
			1 => Self::one(),
			s => Op::Term(Op::one().into(), s),
		}
	}

	fn as_scalar(&self) -> Option<i32> {
		match self {
			Op::Term(op, s) if **op == Op::one() => Some(*s),
			_ => None,
		}
	}

	fn typ(&self, g: Option<&Grammar>) -> Option<Type> {
		match self {
			Op::Term(_, 0) => Some(Type::Zero),
			Op::Term(op, _) => op.typ(g),
			Op::Vec(vi) => Some(Type::Vec(*vi)),
			Op::Sum(terms) => {
				if terms.is_empty() {
					Some(Type::Zero)
				} else {
					None // TODO
				}
			}
			Op::Prod(factors) => {
				if factors.is_empty() {
					Some(Type::S) // TODO: Type::One ?
				} else {
					None // TODO
				}
			}
		}
	}

	// 	fn typ(&self) -> Type;
	// 	fn expand(self) -> Op { ... }

	#[must_use]
	fn simplify(self, g: Option<&Grammar>) -> Op {
		match self {
			Op::Term(op, mut scalar) => {
				let op: Op = match op.simplify(g) {
					Op::Term(inner_op, inner_scalar) => {
						scalar *= inner_scalar;
						*inner_op
					}
					op => op,
				};
				if scalar == 0 || op == Op::zero() {
					Self::zero()
				} else if scalar == 1 {
					op
				} else {
					Op::Term(op.into(), scalar)
				}
			}
			Op::Vec(_) => self,
			Op::Sum(mut terms) => {
				for term in &mut terms {
					term.simplify_inplace(g);
				}

				// TODO: remove zeros

				terms = join_terms(terms, g);

				if terms.is_empty() {
					Op::zero()
				} else if terms.len() == 1 {
					terms.remove(0)
				} else {
					Op::Sum(terms)
				}
			}
			Op::Prod(mut factors) => {
				for fac in &mut factors {
					fac.simplify_inplace(g);
				}

				// TODO: remove ones

				sort_factors(&mut factors, g);

				// TODO: join

				let mut scalar = 1;
				while let Some(s) = factors.first().and_then(Op::as_scalar) {
					scalar *= s;
					factors.remove(0);
				}

				if scalar == 0 {
					Op::zero()
				} else if scalar == 1 {
					if factors.is_empty() {
						Op::one()
					} else if factors.len() == 1 {
						factors.remove(0)
					} else {
						Op::Prod(factors)
					}
				} else {
					Op::Term(Op::Prod(factors).into(), scalar).simplify(g)
				}
			}
		}
	}

	fn simplify_inplace(&mut self, g: Option<&Grammar>) {
		*self = std::mem::replace(self, Op::zero()).simplify(g);
	}
}

// Used when simplifying sums:
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Term {
	/// op first, so we sort on that
	op: Op,
	scalar: i32,
}

impl Term {
	fn from_op(op: Op) -> Term {
		match op {
			Op::Term(op, scalar) => Term { op: *op, scalar },
			op => Term { op, scalar: 1 },
		}
	}

	fn into_op(self) -> Op {
		if self.scalar == 0 {
			Op::zero()
		} else if self.scalar == 1 {
			self.op
		} else {
			Op::Term(self.op.into(), self.scalar)
		}
	}
}

#[must_use]
fn join_terms(terms: Vec<Op>, _g: Option<&Grammar>) -> Vec<Op> {
	// Convert into sum-of-products:
	let mut terms: Vec<Term> = terms.into_iter().map(Term::from_op).collect();
	terms.sort();

	// Join adjacent:
	let mut collapsed_terms: Vec<Term> = vec![];
	for new_term in terms {
		if let Some(last_term) = collapsed_terms.last_mut() {
			if last_term.op == new_term.op {
				last_term.scalar += new_term.scalar;
				if last_term.scalar == 0 {
					collapsed_terms.pop();
				}
				continue;
			}
		}

		collapsed_terms.push(new_term);
	}

	collapsed_terms.into_iter().map(Term::into_op).collect()
}

fn sort_factors(factors: &mut Vec<Op>, g: Option<&Grammar>) {
	// Any sign-change due to swapping
	let mut sign = 1;

	// Sort:
	while {
		let mut did_swap = false;
		for i in 1..factors.len() {
			if factors[i - 1] > factors[i] {
				// We want to swap them. Can we?
				let lt = factors[i - 1].typ(g);
				let rt = factors[i].typ(g);
				if let Some(sign_change) = commutativeness(lt, rt) {
					factors.swap(i - 1, i);
					did_swap = true;
					sign *= sign_change;
				}
			}
		}
		did_swap
	} {}

	assert!(sign == -1 || sign == 1);
	if sign == -1 {
		factors.insert(0, Op::scalar(sign));
	}
}

/// Can we swap these two factors, and if so what is the sign change?
fn commutativeness(l: Option<Type>, r: Option<Type>) -> Option<i32> {
	match (l?, r?) {
		(Type::Vec(l), Type::Vec(r)) => {
			if l == r {
				Some(1)
			} else {
				Some(-1)
			}
		}
		_ => None,
	}
}

// ----------------------------------------------------------------------------

fn main() {
	let mut t = Types::default();
	t.insert("X", Type::Vec(VecIdx(0)));
	t.insert("Y", Type::Vec(VecIdx(1)));
	t.insert("W", Type::Vec(VecIdx(2)));
	let x_type = t.get("X");
	let y_type = t.get("Y");

	assert_eq!(x_type.one().rust(&t), "X");

	assert_eq!(Op::Prod(vec![x_type.one(), y_type.one()]).rust(&t), "X * Y");

	let op = Op::Sum(vec![
		Op::Prod(vec![x_type.one(), y_type.one()]),
		Op::Prod(vec![y_type.one(), x_type.one()]),
	]);
	assert_eq!(op.rust(&t), "X * Y + Y * X");

	assert_eq!(op.simplify(None).rust(&t), "0");
}

/// The scalar type we use for symbolic reasoning.
// type S = i32;

/// Which base vector (e0, e1 or e2?)
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct VecIdx(usize);

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Op {
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
pub enum Type {
	Zero,
	// One,
	/// The scalar type, i.e. 1
	// S,
	/// The vector types
	// Vec(VecIdx),

	/// Blade(vec![])     = scalar
	/// Blade(vec![0])    = e01
	/// Blade(vec![2, 0]) = e02
	Blade(Vec<VecIdx>),
	// /// Tuple-type, linear combination of blades
	// /// Blades(vec![])       = () = Zero
	// /// Blades(vec![S])      = (S)
	// /// Blades(vec![S, E02]) = (S, E02)
	// Blades(Vec<Type>),
	// Struct(Struct),
}

/// named members
// pub enum Struct(Vec<String, Type>)

#[derive(Clone, Debug)]
pub struct Typedef {
	name: String,
	typ: Type,
}

/// In order of preference (first match).
#[derive(Clone, Debug, Default)]
pub struct Types(Vec<Typedef>);

/// what you get when you square the input vectors,
/// e.g. [0, 1, 1] would specify the 2d gpa of e0^2=0  e1^2=1  e2^2=1
pub struct Grammar(Vec<i32>);

// ----------------------------------------------------------------------------

impl Type {
	fn scalar() -> Self {
		Type::Blade(vec![])
	}

	fn vec(vi: VecIdx) -> Self {
		Type::Blade(vec![vi])
	}

	fn blade(vecs: &[VecIdx]) -> Self {
		Type::Blade(vecs.to_vec())
	}

	fn one(&self) -> Op {
		match self {
			// Type::S => Op::one(),
			// Type::Vec(vi) => Op::Vec(*vi),
			Type::Blade(vecs) => match vecs.len() {
				0 => Op::one(),
				1 => Op::Vec(vecs[0]),
				_ => Op::Prod(vecs.iter().copied().map(Op::Vec).collect()),
			},
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
			.find(|td| match &td.typ {
				// Type::Vec(v) if v == vi => true,
				Type::Blade(vecs) if vecs.len() == 1 && vecs[0] == vi => true,
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
					format!("{} * ({})", s, op.rust(t))
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
					factors.iter().map(|factor| format!("({})", factor.rust(t))).join(" * ")
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

	fn scalar(s: i32) -> Self {
		match s {
			0 => Self::zero(),
			1 => Self::one(),
			s => Op::Term(Op::one().into(), s),
		}
	}

	fn is_zero(&self) -> bool {
		// Needs to be simplified!
		self == &Self::zero()
	}

	fn is_one(&self) -> bool {
		// Needs to be simplified!
		self ==& Self::one()
	}

	fn as_scalar(&self) -> Option<i32> {
		match self {
			Op::Term(op, s) if **op == Op::one() => Some(*s),
			_ => None,
		}
	}

	fn as_factors(self) -> Vec<Op> {
		match self {
			Op::Term(op, scalar) => {
				let mut factors = op.as_factors();
				factors.push(Op::scalar(scalar));
				factors
			}
			Op::Prod(factors) => factors,
			op => vec![op],
		}
	}

	fn typ(&self, g: Option<&Grammar>) -> Option<Type> {
		match self {
			Op::Term(_, 0) => Some(Type::Zero),
			Op::Term(op, _) => op.typ(g),
			Op::Vec(vi) => Some(Type::vec(*vi)),
			Op::Sum(terms) => {
				if terms.is_empty() {
					Some(Type::Zero)
				} else {
					None // TODO
				}
			}
			Op::Prod(factors) => {
				if factors.is_empty() {
					Some(Type::scalar()) // TODO: Type::One ?
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
				if scalar == 0 || op.is_zero() {
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
				terms.retain(|f| !f.is_zero());

				terms = join_terms(terms, g);

				if terms.is_empty() {
					Op::zero()
				} else if terms.len() == 1 {
					terms.remove(0)
				} else {
					Op::Sum(terms)
				}
			}
			Op::Prod(factors) => {
				let mut new_scalar = 1;
				let mut new_factors = vec![];

				for fac in factors {
					for fac in fac.as_factors() {
						if let Some(scalar) = fac.as_scalar() {
							new_scalar *= scalar;
						} else {
							new_factors.push(fac);
						}
					}
				}
				let mut scalar = new_scalar;
				let mut factors = new_factors;

				scalar *= sort_factors(&mut factors, g);

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

#[must_use]
fn sort_factors(factors: &mut Vec<Op>, g: Option<&Grammar>) -> i32 {
	// Any sign-change due to swapping
	let mut sign = 1;

	// Sort:
	while {
		let mut did_swap = false;
		for i in 1..factors.len() {
			if factors[i - 1] == factors[i] {
				// Square it!
				if let Some(g) = g {
				if let  Some(t) = factors[i].typ(Some(g)) {
					if  let Some(s) = square_to_sign(t, g) {
						sign *= s;
						factors.remove(i);
						factors.remove(i-1);
						did_swap = true;
						break;
					}
				}
}
			} else if factors[i - 1] > factors[i] {
				// We want to swap them. Can we?
				let lt = factors[i - 1].typ(g);
				let rt = factors[i].typ(g);
				if let Some(sign_change) = geom_mul_commutativeness(lt, rt) {
					factors.swap(i - 1, i);
					did_swap = true;
					sign *= sign_change;
				}
			}
		}
		did_swap
	} {}

	sign
}

/// Can we swap these two factors, and if so what is the sign change?
fn geom_mul_commutativeness(l: Option<Type>, r: Option<Type>) -> Option<i32> {
	match (l?, r?) {
		(Type::Blade(l), Type::Blade(r)) => {
			if l.is_empty() || r.is_empty() {
				// scalar times whatever commutes
				Some(1)
			} else if l.len() == 1 && r.len() == 1 {
				// vectors
				if l[0] == r[0] {
					// Same vector
					Some(1)
				} else {
					Some(-1)
				}
			} else {
				None // TODO!
			}
		}
		_ => None,
	}
}

/// Does this type square to either -1, 0 or +1?
fn square_to_sign(t: Type, g: &Grammar) -> Option<i32> {
	match t {
		Type::Zero => Some(0),
		Type::Blade(v) => {
			if v.is_empty() {
				None
			} else if v.len() == 1{
				Some(g.square(v[0]))
			} else {
				None // TODO
			}
		}
	}
}

// ----------------------------------------------------------------------------

impl Grammar {
	/// What do we get when we square this basis vector?
	pub fn square(&self, v: VecIdx) -> i32 {
		self.0[v.0]
	}
}

// ----------------------------------------------------------------------------

fn main() {
	let mut t = Types::default();
	let x = VecIdx(0);
	let y = VecIdx(1);
	let w = VecIdx(2);
	t.insert("R", Type::scalar());
	t.insert("X", Type::vec(x));
	t.insert("Y", Type::vec(y));
	t.insert("W", Type::vec(w));
	t.insert("YW", Type::blade(&[y, w]));
	t.insert("WX", Type::blade(&[w, x]));
	t.insert("XY", Type::blade(&[x, y]));
	t.insert("XYW", Type::blade(&[x, y, w]));

	let g = Grammar(vec![1,1,0]);

	let x_type = t.get("X");
	let y_type = t.get("Y");

	println!("{:?}", Op::Prod(vec![t.get("R").one(), t.get("R").one()]).simplify(Some(&g)).rust(&t));

	let unit_blades = vec![
		t.get("R"),
		t.get("X"),
		t.get("Y"),
		t.get("W"),
		t.get("YW"),
		t.get("WX"),
		t.get("XY"),
		t.get("XYW"),
	];
	println!();
	println!("Geometric multiplication table (left side * top row):");
	for a in &unit_blades {
		print!("  ");
		for b in &unit_blades {
			print!("{:<8}    ", Op::Prod(vec![a.one(), b.one()]).simplify(Some(&g)).rust(&t));
		}
		println!();
	}


	assert_eq!(x_type.one().rust(&t), "X");

	assert_eq!(Op::Prod(vec![x_type.one(), y_type.one()]).rust(&t), "X * Y");

	let op = Op::Sum(vec![
		Op::Prod(vec![x_type.one(), y_type.one()]),
		Op::Prod(vec![y_type.one(), x_type.one()]),
	]);
	assert_eq!(op.rust(&t), "X * Y + Y * X");

	assert_eq!(op.simplify(Some(&g)).rust(&t), "0");
}

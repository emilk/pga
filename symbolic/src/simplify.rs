use crate::*;

// Used when simplifying sums:
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Term {
	/// op first, so we sort on that
	op: Op,
	scalar: i32,
}

impl Term {
	pub fn from_op(op: Op) -> Term {
		match op {
			Op::Term(op, scalar) => Term { op: *op, scalar },
			op => Term { op, scalar: 1 },
		}
	}

	pub fn into_op(self) -> Op {
		if self.scalar == 0 {
			Op::zero()
		} else if self.scalar == 1 {
			self.op
		} else {
			Op::Term(self.op.into(), self.scalar)
		}
	}
}

impl Op {
	#[must_use]
	pub fn simplify(self, g: Option<&Grammar>) -> Op {
		match self {
			Op::Var(var_name, Type::Struct(members)) => Op::Sum(
				members
					.into_iter()
					.map(|(mem_name, typ)| Op::var(format!("{}.{}", var_name, mem_name), &typ))
					.collect(),
			),
			Op::Var(_, _) => self,
			Op::Vec(_) => self,
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
			Op::Prod(product, mut factors) => {
				for fac in &mut factors {
					fac.simplify_inplace(g);
				}

				// look for a sum for expansion:
				for (i, fac) in factors.iter().enumerate() {
					if let Op::Sum(terms) = fac {
						let terms = terms.clone();
						return Op::Sum(
							terms
								.into_iter()
								.map(|term| {
									factors[i] = term;
									Op::Prod(product, factors.clone())
								})
								.collect(),
						)
						.simplify(g);
					}
				}

				simplify_product(product, factors, g)
			}
		}
	}

	pub fn simplify_inplace(&mut self, g: Option<&Grammar>) {
		*self = std::mem::replace(self, Op::zero()).simplify(g);
	}

	fn as_factors(self) -> Vec<Op> {
		match self {
			Op::Term(op, scalar) => {
				let mut factors = op.as_factors();
				factors.push(Op::scalar(scalar));
				factors
			}
			Op::Prod(_product, factors) => factors,
			op => vec![op],
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
fn simplify_product(product: Product, factors: Vec<Op>, g: Option<&Grammar>) -> Op {
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

	scalar *= sort_factors(product, &mut factors, g);

	if scalar == 0 {
		Op::zero()
	} else if scalar == 1 {
		if factors.is_empty() {
			Op::one()
		} else if factors.len() == 1 {
			factors.remove(0)
		} else {
			Op::Prod(product, factors)
		}
	} else {
		Op::Term(Op::Prod(product, factors).into(), scalar).simplify(g)
	}
}

#[must_use]
fn sort_factors(product: Product, factors: &mut Vec<Op>, g: Option<&Grammar>) -> i32 {
	// Any sign-change due to swapping
	let mut sign = 1;

	// Sort:
	while {
		let mut did_swap = false;
		for i in 1..factors.len() {
			if factors[i - 1] == factors[i] {
				// Square it!
				if let Some(g) = g {
					if let Some(t) = factors[i].typ(Some(g)) {
						if let Some(s) = square_to_sign(product, &t, g) {
							sign *= s;
							factors.remove(i);
							factors.remove(i - 1);
							did_swap = true;
							break;
						}
					}
				}
			} else if factors[i - 1] > factors[i] {
				// We want to swap them. Can we?
				let lt = factors[i - 1].typ(g);
				let rt = factors[i].typ(g);
				if let Some(sign_change) = commutativeness(product, lt, rt) {
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
fn commutativeness(_product: Product, l: Option<Type>, r: Option<Type>) -> Option<i32> {
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
fn square_to_sign(product: Product, t: &Type, g: &Grammar) -> Option<i32> {
	match t {
		Type::Zero => Some(0),
		Type::Blade(v) => {
			if v.is_empty() {
				None
			} else if v.len() == 1 {
				match product {
					Product::Geometric => Some(g.square(v[0])),
					Product::Wedge => Some(0),
				}
			} else {
				None // TODO
			}
		}
		Type::Struct(members) => match members.len() {
			0 => Some(0),
			1 => square_to_sign(product, &members[0].1, g),
			_ => None,
		},
	}
}

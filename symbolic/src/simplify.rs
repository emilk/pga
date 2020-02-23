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
			Op::Prod(factors) => simplify_factors(factors, g),
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
			Op::Prod(factors) => factors,
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
fn simplify_factors(factors: Vec<Op>, g: Option<&Grammar>) -> Op {
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
					if let Some(t) = factors[i].typ(Some(g)) {
						if let Some(s) = square_to_sign(t, g) {
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
			} else if v.len() == 1 {
				Some(g.square(v[0]))
			} else {
				None // TODO
			}
		}
	}
}

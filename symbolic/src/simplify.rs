use crate::*;

// Used when simplifying sums:
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Term {
	/// expr first, so we sort on that
	expr: Expr,
	scalar: i32,
}

impl Term {
	pub fn from_op(expr: Expr) -> Term {
		match expr {
			Expr::Term(expr, scalar) => Term { expr: *expr, scalar },
			expr => Term { expr, scalar: 1 },
		}
	}

	pub fn into_op(self) -> Expr {
		if self.scalar == 0 {
			Expr::zero()
		} else if self.scalar == 1 {
			self.expr
		} else {
			Expr::Term(self.expr.into(), self.scalar)
		}
	}
}

impl Expr {
	#[must_use]
	pub fn simplify(self, g: Option<&Grammar>) -> Expr {
		match self {
			Expr::Var(_, Type::Constant(sblade)) => Expr::sblade(&sblade),
			Expr::Var(var_name, Type::Struct(members)) => Expr::Sum(
				members
					.into_iter()
					.map(|(mem_name, typ)| Expr::var(format!("{}.{}", var_name, mem_name), &typ))
					.collect(),
			)
			.simplify(g),
			Expr::Var(_, _) => self,
			Expr::Vec(_) => self,
			Expr::Unary(unary, expr) => match expr.simplify(g) {
				// e.g. x.lcompl().rcompl() => x
				Expr::Unary(inner_unary, expr) if inner_unary == unary.undoer() => *expr,

				// distributive property
				// (a + b).unary() = a.unary() + b.unary()
				Expr::Sum(terms) => {
					Expr::Sum(terms.into_iter().map(|t| Expr::Unary(unary, t.into())).collect()).simplify(g)
				}

				expr => Expr::Unary(unary, expr.into()),
			},

			Expr::Term(expr, mut scalar) => {
				let expr: Expr = match expr.simplify(g) {
					Expr::Term(inner_op, inner_scalar) => {
						scalar *= inner_scalar;
						*inner_op
					}
					expr => expr,
				};
				if scalar == 0 || expr.is_zero() {
					Self::zero()
				} else if scalar == 1 {
					expr
				} else {
					Expr::Term(expr.into(), scalar)
				}
			}
			Expr::Sum(terms) => {
				// use itertools::Itertools;
				// eprintln!("simplify sums input: {:?}", terms.iter().map(Expr::rust).join(" + "));
				let mut terms: Vec<Expr> = terms
					.into_iter()
					.flat_map(|term| match term.simplify(g) {
						Expr::Sum(terms) => terms,
						expr => vec![expr],
					})
					.collect();
				terms.retain(|f| !f.is_zero());

				// eprintln!("simplify sums PRE-sort {:?}", terms.iter().map(Expr::rust).join(" + "));
				terms = sort_and_join_terms(terms, g);
				// eprintln!("simplify sums POST-sort {:?}", terms.iter().map(Expr::rust).join(" + "));

				if terms.is_empty() {
					Expr::zero()
				} else if terms.len() == 1 {
					terms.remove(0)
				} else {
					Expr::Sum(terms)
				}
			}
			Expr::Prod(product, mut factors) => {
				for fac in &mut factors {
					fac.simplify_inplace(g);
				}

				// look for a sum for expansion:
				for (i, fac) in factors.iter().enumerate() {
					if let Expr::Sum(terms) = fac {
						let terms = terms.clone();
						return Expr::Sum(
							terms
								.into_iter()
								.map(|term| {
									factors[i] = term;
									Expr::Prod(product, factors.clone())
								})
								.collect(),
						)
						.simplify(g);
					}
				}

				simplify_product(product, factors, g)
			}
			Expr::StructInstance { .. } => self,
		}
	}

	pub fn simplify_inplace(&mut self, g: Option<&Grammar>) {
		*self = std::mem::replace(self, Expr::zero()).simplify(g);
	}

	fn into_factors(self, desired_product_type: Product) -> Vec<Expr> {
		match self {
			Expr::Term(expr, scalar) => {
				let mut factors = expr.into_factors(desired_product_type);
				factors.push(Expr::scalar(scalar));
				factors
			}
			Expr::Prod(product, factors) if product == desired_product_type => factors,
			expr => vec![expr],
		}
	}
}

#[must_use]
fn sort_and_join_terms(terms: Vec<Expr>, _g: Option<&Grammar>) -> Vec<Expr> {
	// Convert into sum-of-products:
	let mut terms: Vec<Term> = terms.into_iter().map(Term::from_op).collect();
	terms.sort();

	// Join adjacent:
	let mut collapsed_terms: Vec<Term> = vec![];
	for new_term in terms {
		if let Some(last_term) = collapsed_terms.last_mut() {
			if last_term.expr == new_term.expr {
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
fn simplify_product(product: Product, factors: Vec<Expr>, g: Option<&Grammar>) -> Expr {
	// eprintln!("simplify_product {:?} {:?}", product, factors);
	let mut new_scalar = 1;
	let mut new_factors = vec![];

	for fac in factors {
		for fac in fac.into_factors(product) {
			if let Some(scalar) = fac.as_scalar() {
				new_scalar *= scalar;
			} else {
				new_factors.push(fac);
			}
		}
	}
	let mut scalar = new_scalar;
	let mut factors = new_factors;

	// eprintln!("simplify_product {} * {:?} {:?}", scalar, product, factors);

	scalar *= sort_factors(product, &mut factors, g);
	if let Some(g) = g {
		scalar *= collapse_factors(product, &mut factors, g);
	}

	// eprintln!("simplify_product {} * {:?} {:?}", scalar, product, factors);

	if scalar == 0 {
		Expr::zero()
	} else if scalar == 1 {
		if factors.is_empty() {
			match product {
				Product::Geometric | Product::Wedge | Product::Dot => Expr::one(),
				_ => Expr::Prod(product, factors), // TODO
			}
		} else if factors.len() == 1 {
			factors.remove(0)
		} else {
			Expr::Prod(product, factors)
		}
	} else {
		Expr::Term(Expr::Prod(product, factors).into(), scalar).simplify(g)
	}
}

#[must_use]
fn sort_factors(product: Product, factors: &mut [Expr], g: Option<&Grammar>) -> i32 {
	// Any sign-change due to swapping
	let mut sign = 1;

	// Bubble-sort:
	while {
		let mut did_swap = false;
		for i in 1..factors.len() {
			if factors[i - 1] > factors[i] {
				// We want to swap them. Can we?
				let lt = factors[i - 1].typ(g);
				let rt = factors[i].typ(g);
				if let Some(sign_change) = commutativeness(product, lt, rt, g) {
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

#[must_use]
fn collapse_factors(product: Product, factors: &mut Vec<Expr>, g: &Grammar) -> i32 {
	// Any sign-change due to squaring of base vectors
	let mut sign = 1;

	let mut i = 0;
	while i + 1 < factors.len() {
		if factors[i] == factors[i + 1] {
			if let Expr::Vec(vi) = factors[i] {
				if let Some(s) = g.square_with(product, vi) {
					sign *= s;
					factors.remove(i);
					factors.remove(i);
					continue;
				}
			}
		}

		if let (Some(Type::SBlade(lb)), Some(Type::SBlade(rb))) = (factors[i].typ(Some(g)), factors[i + 1].typ(Some(g)))
		{
			if SBlade::binary_product(&lb, product, &rb, g).is_zero() {
				factors.clear();
				return 0;
			}
		}

		i += 1;
	}

	sign
}

/// Can we swap these two factors, and if so what is the sign change?
fn commutativeness(product: Product, l: Option<Type>, r: Option<Type>, g: Option<&Grammar>) -> Option<i32> {
	let l = l?.into_sblade()?;
	let r = r?.into_sblade()?;
	let lr = SBlade::binary_product(&l, product, &r, g?);
	let rl = SBlade::binary_product(&r, product, &l, g?);
	assert_eq!(lr.is_zero(), rl.is_zero());
	assert_eq!(lr.blade, rl.blade);
	assert_eq!(lr.sign.abs(), rl.sign.abs());
	if lr.is_zero() {
		Some(0)
	} else if lr.sign == rl.sign {
		Some(1)
	} else {
		Some(-1)
	}
}

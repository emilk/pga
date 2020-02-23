///! Module for formatting as rust code
use crate::*;

struct RustExpr(Precedence, String);

#[derive(Eq, Ord, PartialEq, PartialOrd)]
enum Precedence {
	Sum,
	Product,
	Atom,
}

impl RustExpr {
	fn atom(s: impl ToString) -> Self {
		RustExpr(Precedence::Atom, s.to_string())
	}

	fn enclose_if_less(&self, p: Precedence) -> String {
		if self.0 < p {
			format!("({})", self.1)
		} else {
			self.1.clone()
		}
	}
}

impl Op {
	pub fn rust(&self, t: &Types, g: Option<&Grammar>) -> String {
		self.rust_expr(t, g).1
	}

	fn rust_expr(&self, t: &Types, g: Option<&Grammar>) -> RustExpr {
		if let Some(expr) = self.as_type(t, g) {
			return expr;
		}

		use itertools::Itertools;
		match self {
			// Op::S(s) => s.to_string(),
			Op::Var(name, _typ) => RustExpr::atom(name),
			Op::Vec(vi) => RustExpr::atom(t.vec_name(*vi)),
			Op::Term(op, s) => {
				if op.is_one() {
					RustExpr::atom(s)
				} else if *s == -1 {
					RustExpr(
						Precedence::Product,
						format!("-{}", op.rust_expr(t, g).enclose_if_less(Precedence::Product)),
					)
				} else {
					RustExpr(
						Precedence::Product,
						format!("{} * {}", s, op.rust_expr(t, g).enclose_if_less(Precedence::Product)),
					)
				}
			}
			Op::Sum(terms) => {
				if terms.is_empty() {
					RustExpr::atom("0")
				} else if terms.len() == 1 {
					terms[0].rust_expr(t, g)
				} else {
					RustExpr(Precedence::Sum, terms.iter().map(|term| term.rust(t, g)).join(" + "))
				}
			}
			Op::Prod(product, factors) => {
				if factors.is_empty() {
					RustExpr::atom("1")
				} else if factors.len() == 1 {
					factors[0].rust_expr(t, g)
				} else {
					let operator = match product {
						Product::Geometric => " * ",
						Product::Wedge => " ^ ",
					};
					RustExpr(
						Precedence::Product,
						factors
							.iter()
							.map(|factor| factor.rust_expr(t, g).enclose_if_less(Precedence::Product))
							.join(operator),
					)
				}
			}
		}
	}

	/// Try to express this op as a known typ (vector, blade, struct)
	fn as_type(&self, t: &Types, g: Option<&Grammar>) -> Option<RustExpr> {
		// A blade?
		if let Some((scalar, blade)) = self.as_blade() {
			if blade.is_empty() {
				return Some(RustExpr::atom(scalar));
			}
			if let Some((sign, name)) = t.blade_name(&blade) {
				let scalar = scalar * sign;
				return Some(match scalar {
					-1 => RustExpr(Precedence::Product, format!("-{}", name)),
					0 => RustExpr::atom(format!("-{}", name)),
					1 => RustExpr::atom(name),
					_ => RustExpr(Precedence::Product, format!("{} * {}", scalar, name)),
				});
			}
		}

		// if let Op::Sum(terms) = self {
		// 	if let Some(s) = as_struct(terms, t, g) {
		// 		return Some(s);
		// 	}
		// }

		None
	}
}

// fn as_struct(terms: &[Op], t: &Types, g: Option<&Grammar>) -> Option<RustExpr> {
// 	let mut parts: std::collections::BTreeMap<Type, Vec<Op>> = Default::default();
// 	for term in terms {
// 		let typ = term.typ(g)?;
// 		if !typ.is_zero() {
// 			parts.entry(typ).or_default().push(term.clone());
// 		}
// 	}

// 	let sum: Vec<(Type, Op)> = parts
// 		.into_iter()
// 		.map(|(typ, terms)| if terms.len() == 1 { terms[0] } else { Op::Sum(terms) })
// 		.collect();

// 	find_struct(&sum, t).map(RustExpr::atom)
// }

// fn find_struct(sum: &[(Type, Op)], t: &Types) -> Option<String> {
// 	for members in t.structs() {
// 		if let Some(instance) = as_struct_instance(members, &sum) {
// 			return Some(instance);
// 		}
// 	}
// 	None
// }

// fn as_struct_instance(members: &[(String, Type)], sum: &[(Type, Op)]) -> Option {

// }

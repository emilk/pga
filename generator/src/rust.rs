///! Module for formatting as rust code
use itertools::Itertools;

use crate::*;

#[derive(Clone, Copy)]
pub struct RustOptions {
	/// Output "a ^ b" if true, else "a.wedge(b)"
	pub operators: bool,
}

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

impl Expr {
	pub fn rust(&self, ro: &RustOptions) -> String {
		self.rust_expr(ro).1
	}

	// Helper for tests: output with operators (a ^ b)
	pub fn rust_ops(&self) -> String {
		self.rust(&RustOptions { operators: true })
	}

	fn rust_expr(&self, ro: &RustOptions) -> RustExpr {
		match self {
			Expr::Var { name, .. } => RustExpr::atom(name),
			Expr::Vec(vi) => {
				//  You should call expr.typify() before .rust(ro) to get more readable vector names
				RustExpr::atom(format!("_e{}", vi.0))
			}
			Expr::Term(expr, s) => {
				if expr.is_one() {
					RustExpr::atom(s)
				} else if *s == -1 {
					RustExpr(
						Precedence::Product,
						format!("-{}", expr.rust_expr(ro).enclose_if_less(Precedence::Product)),
					)
				} else {
					RustExpr(
						Precedence::Product,
						format!("{} * {}", s, expr.rust_expr(ro).enclose_if_less(Precedence::Product)),
					)
				}
			}
			Expr::Unary(unary, expr) => RustExpr::atom(format!(
				"{}.{}()",
				expr.rust_expr(ro).enclose_if_less(Precedence::Atom),
				unary.name()
			)),
			Expr::Sum(terms) => {
				if terms.is_empty() {
					RustExpr::atom("0")
				} else if terms.len() == 1 {
					terms[0].rust_expr(ro)
				} else {
					// RustExpr(Precedence::Sum, terms.iter().map(|term| term.rust(ro)).join(" + "))
					let mut s = terms[0].rust(ro);
					for t in &terms[1..] {
						if t.is_negation() {
							s += &format!(" - {}", t.clone().negate().rust(ro));
						} else {
							s += &format!(" + {}", t.rust(ro));
						}
					}
					RustExpr(Precedence::Sum, s)
				}
			}
			Expr::Prod(product, factors) => {
				if factors.is_empty() {
					match product {
						Product::Geometric | Product::Wedge | Product::Dot => RustExpr::atom("1"),
						_ => todo!(),
					}
				} else if factors.len() == 1 {
					factors[0].rust_expr(ro)
				} else {
					if ro.operators {
						let operator = format!(" {} ", product.symbol());
						RustExpr(
							Precedence::Product,
							factors
								.iter()
								.map(|factor| factor.rust_expr(ro).enclose_if_less(Precedence::Product))
								.join(&operator),
						)
					} else {
						let mut code = factors[0].rust_expr(ro).enclose_if_less(Precedence::Atom);
						for factor in factors.iter().skip(1) {
							code += &format!(".{}({})", product.trait_function_name(), factor.rust(ro))
						}
						RustExpr(Precedence::Atom, code)
					}
				}
			}
			Expr::StructInstance(StructInstance { struct_name, members }) => {
				let maxw = members.iter().map(|(name, _)| name.len()).max().unwrap_or_default();
				RustExpr::atom(format!(
					"{} {{\n{}\n}}",
					struct_name,
					indent(
						&members
							.iter()
							.map(|(name, expr)| format!("{:maxw$}: {},", name, expr.rust(ro), maxw = maxw))
							.join("\n")
					)
				))
			}
		}
	}
}

fn indent(s: &str) -> String {
	s.lines().map(|line| format!("    {}", line)).join("\n")
}

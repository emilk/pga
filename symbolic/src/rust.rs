///! Module for formatting as rust code
use itertools::Itertools;

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
	pub fn rust(&self) -> String {
		self.rust_expr().1
	}

	fn rust_expr(&self) -> RustExpr {
		match self {
			Op::Var(name, _typ) => RustExpr::atom(name),
			Op::Vec(vi) => {
				//  You should call op.typify() before .rus() to get more readable vector names
				RustExpr::atom(format!("e{}", vi.0))
			}
			Op::Term(op, s) => {
				if op.is_one() {
					RustExpr::atom(s)
				} else if *s == -1 {
					RustExpr(
						Precedence::Product,
						format!("-{}", op.rust_expr().enclose_if_less(Precedence::Product)),
					)
				} else {
					RustExpr(
						Precedence::Product,
						format!("{} * {}", s, op.rust_expr().enclose_if_less(Precedence::Product)),
					)
				}
			}
			Op::LCompl(op) => RustExpr::atom(format!("lcompl({})", op.rust())),
			Op::RCompl(op) => RustExpr::atom(format!("rcompl({})", op.rust())),
			Op::Sum(terms) => {
				if terms.is_empty() {
					RustExpr::atom("0")
				} else if terms.len() == 1 {
					terms[0].rust_expr()
				} else {
					RustExpr(Precedence::Sum, terms.iter().map(|term| term.rust()).join(" + "))
				}
			}
			Op::Prod(product, factors) => {
				if factors.is_empty() {
					match product {
						Product::Geometric | Product::Wedge | Product::Dot => RustExpr::atom("1"),
						_ => todo!(),
					}
				} else if factors.len() == 1 {
					factors[0].rust_expr()
				} else {
					let operator = format!(" {} ", product.symbol());
					RustExpr(
						Precedence::Product,
						factors
							.iter()
							.map(|factor| factor.rust_expr().enclose_if_less(Precedence::Product))
							.join(&operator),
					)
				}
			}
			Op::StructInstance { struct_name, members } => {
				let maxw = members.iter().map(|(name, _)| name.len()).max().unwrap_or_default();
				RustExpr::atom(format!(
					"{} {{\n{}\n}}",
					struct_name,
					indent(
						&members
							.iter()
							.map(|(name, op)| format!("{:maxw$}: {},", name, op.rust(), maxw = maxw))
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

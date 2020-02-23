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
	pub fn rust(&self, t: &Types) -> String {
		self.rust_expr(t).1
	}

	fn rust_expr(&self, t: &Types) -> RustExpr {
		if let Some((scalar, blade)) = self.as_blade() {
			if blade.is_empty() {
				return RustExpr::atom(scalar);
			}
			if let Some((sign, name)) = t.blade_name(&blade) {
				let scalar = scalar * sign;
				return match scalar {
					-1 => RustExpr(Precedence::Product, format!("-{}", name)),
					0 => RustExpr::atom(format!("-{}", name)),
					1 => RustExpr::atom(name),
					_ => RustExpr(Precedence::Product, format!("{} * {}", scalar, name)),
				};
			}
		}

		use itertools::Itertools;
		match self {
			// Op::S(s) => s.to_string(),
			Op::Vec(vi) => RustExpr::atom(t.vec_name(*vi)),
			Op::Term(op, s) => {
				if **op == Op::one() {
					RustExpr::atom(s)
				} else if *s == -1 {
					RustExpr(
						Precedence::Product,
						format!("-{}", op.rust_expr(t).enclose_if_less(Precedence::Product)),
					)
				} else {
					RustExpr(
						Precedence::Product,
						format!("{} * {}", s, op.rust_expr(t).enclose_if_less(Precedence::Product)),
					)
				}
			}
			Op::Sum(terms) => {
				if terms.is_empty() {
					RustExpr::atom("0")
				} else if terms.len() == 1 {
					terms[0].rust_expr(t)
				} else {
					RustExpr(Precedence::Sum, terms.iter().map(|term| term.rust(t)).join(" + "))
				}
			}
			Op::Prod(factors) => {
				if factors.is_empty() {
					RustExpr::atom("1")
				} else if factors.len() == 1 {
					factors[0].rust_expr(t)
				} else {
					RustExpr(
						Precedence::Product,
						factors
							.iter()
							.map(|factor| factor.rust_expr(t).enclose_if_less(Precedence::Product))
							.join(" * "),
					)
				}
			}
		}
	}

	/// Returns this Op in terms of a multiple of a blade, if possible
	fn as_blade(&self) -> Option<(i32, Vec<VecIdx>)> {
		match self {
			Op::Vec(vi) => Some((1, vec![*vi])),
			Op::Term(op, s) => {
				if let Some((sign, blade)) = op.as_blade() {
					Some((s * sign, blade))
				} else {
					None
				}
			}
			Op::Sum(terms) => {
				if terms.is_empty() {
					Some((0, vec![]))
				} else if terms.len() == 1 {
					terms[0].as_blade()
				} else {
					None // assuming we are simplified
				}
			}
			Op::Prod(factors) => {
				let mut sign = 1;
				let mut blade = vec![];
				for f in factors {
					let (s, mut b) = f.as_blade()?;
					sign *= s;
					blade.append(&mut b);
				}
				Some((sign, blade))
			}
		}
	}
}

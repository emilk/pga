use std::collections::BTreeMap;

use crate::*;

/// Sum of the values, grouped by their types.
type Value = BTreeMap<Blade, Expr>;

// fn show_value(v: &Value) -> String {
// 	use itertools::Itertools;
// 	format!(
// 		"{{\n{}\n}}",
// 		v.iter().map(|(k, v)| format!("  {:6?}: {},", k, v.rust())).join("\n")
// 	)
// }

impl Expr {
	/// Detect named types and replace those expressions with named versions.
	pub fn typify(mut self, t: &Types, g: &Grammar) -> Self {
		if let Expr::Sum(terms) = &self {
			// eprintln!("typify Sum: {}", self.rust());
			if let Some(value) = as_value(&terms, Some(g)) {
				if let Some(s) = find_struct(&value, t) {
					// eprintln!("typify Sum {} as struct {}", self.rust(), s.struct_name);
					self = Expr::StructInstance(s);
				}
			}
		}

		// A blade?
		if let Some(sblade) = self.as_sblade(g) {
			// eprintln!("typify sblade: {}", self.rust());
			if sblade.is_zero() {
				self = Expr::zero();
			} else if sblade.is_scalar() {
				self = Expr::scalar(sblade.sign);
			} else if let Some(blade_typedef) = t.blade_typedef(&sblade.blade) {
				let blade_var = Expr::var(&blade_typedef.name, &blade_typedef.typ);
				let scalar = sblade.sign
					* match &blade_typedef.typ {
						Type::SBlade(sb) => sb.sign,
						_ => unreachable!(),
					};
				self = match scalar {
					0 => Expr::zero(),
					1 => blade_var,
					_ => Expr::Term(blade_var.into(), scalar),
				};
			}
		}

		match self {
			Expr::Var(_, _) | Expr::Vec(_) => self,
			Expr::Term(expr, s) => Expr::Term(expr.typify(t, g).into(), s),
			Expr::Unary(unary, expr) => Expr::Unary(unary, expr.typify(t, g).into()),
			Expr::Sum(terms) => Expr::Sum(terms.into_iter().map(|e| e.typify(t, g)).collect()),
			Expr::Prod(prod, factors) => Expr::Prod(prod, factors.into_iter().map(|e| e.typify(t, g)).collect()),
			Expr::StructInstance(StructInstance { struct_name, members }) => Expr::StructInstance(StructInstance {
				struct_name,
				members: members.into_iter().map(|(name, e)| (name, e.typify(t, g))).collect(),
			}),
		}
	}
}

fn as_value(terms: &[Expr], g: Option<&Grammar>) -> Option<Value> {
	let mut parts: BTreeMap<Blade, Vec<Expr>> = Default::default();
	for term in terms {
		// eprintln!("as_value {} typ: {:?}", term.rust(), term.typ(g));
		let typ = term.typ(g)?;
		if !typ.is_zero() {
			match typ {
				Type::Constant(sblade) | Type::SBlade(sblade) => {
					let term = if sblade.is_negative() {
						term.clone().negate()
					} else {
						term.clone()
					};
					parts.entry(sblade.blade).or_default().push(term);
				}
				Type::Struct { .. } => {
					return None;
				}
			}
		}
	}

	Some(
		parts
			.into_iter()
			.map(|(typ, terms)| (typ, Expr::Sum(terms).simplify(g)))
			.collect(),
	)
}

fn find_struct(sum: &Value, t: &Types) -> Option<StructInstance> {
	if sum.is_empty() {
		return None; // zero: no struct for this!
	}
	if sum.len() <= 1 {
		return None; // Not really a struct
	}

	// eprintln!("find_struct for {}", show_value(sum));

	for (name, members) in t.structs() {
		if let Some(instance) = as_struct_instance(name, members, &sum) {
			return Some(instance);
		}
	}
	None
}

fn as_struct_instance(struct_name: &str, struct_members: &[(String, Type)], value: &Value) -> Option<StructInstance> {
	let find_term = |needle: &Type| {
		value
			.iter()
			.find(|(b, _)| needle.is_blade(b))
			.map(|(_, expr)| expr.clone())
	};

	if value.keys().all(|b| is_blade_in_struct(struct_members, b)) {
		Some(StructInstance {
			struct_name: struct_name.to_owned(),
			members: struct_members
				.iter()
				.map(|(name, t)| (name.to_string(), find_term(t).unwrap_or_else(Expr::zero)))
				.collect(),
		})
	} else {
		None
	}
}

fn is_blade_in_struct(struct_members: &[(String, Type)], blade: &Blade) -> bool {
	struct_members.iter().any(|(_, t)| t.is_blade(blade))
}

use std::collections::BTreeMap;

use crate::*;

/// Sum of the values, grouped by their types.
type Value = BTreeMap<Blade, Op>;

// fn show_value(v: &Value) -> String {
// 	use itertools::Itertools;
// 	format!(
// 		"{{\n{}\n}}",
// 		v.iter().map(|(k, v)| format!("  {:6?}: {},", k, v.rust())).join("\n")
// 	)
// }

impl Op {
	/// Detect named types and replace
	pub fn typify(self, t: &Types, g: &Grammar) -> Self {
		// TODO: recurse!

		if let Op::Sum(terms) = &self {
			if let Some(value) = as_value(&terms, Some(g)) {
				if let Some(s) = find_struct(&value, t) {
					return s;
				}
			}
		}

		// A blade?
		if let Some(sblade) = self.as_sblade(g) {
			if sblade.is_zero() {
				Op::zero()
			} else if sblade.is_scalar() {
				Op::scalar(sblade.sign)
			} else if let Some(blade_typedef) = t.blade_typedef(&sblade.blade) {
				let blade_var = Op::var(&blade_typedef.name, &blade_typedef.typ);
				let scalar = sblade.sign
					* match &blade_typedef.typ {
						Type::SBlade(sb) => sb.sign,
						_ => unreachable!(),
					};
				match scalar {
					0 => Op::zero(),
					1 => blade_var,
					_ => Op::Term(blade_var.into(), scalar),
				}
			} else {
				self
			}
		} else {
			self
		}
	}
}

fn as_value(terms: &[Op], g: Option<&Grammar>) -> Option<Value> {
	let mut parts: BTreeMap<Blade, Vec<Op>> = Default::default();
	for term in terms {
		// eprintln!("as_value {} typ: {:?}", term.rust(), term.typ(g));
		let typ = term.typ(g)?;
		if !typ.is_zero() {
			match typ {
				Type::SBlade(sblade) => {
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
			.map(|(typ, terms)| (typ, Op::Sum(terms).simplify(g)))
			.collect(),
	)
}

fn find_struct(sum: &Value, t: &Types) -> Option<Op> {
	if sum.is_empty() {
		return None; // zero: no struct for this!
	}
	// eprintln!("find_struct for {}", show_value(sum));

	for (name, members) in t.structs() {
		if let Some(instance) = as_struct_instance(name, members, &sum) {
			return Some(instance);
		}
	}
	None
}

fn as_struct_instance(struct_name: &str, struct_members: &[(String, Type)], value: &Value) -> Option<Op> {
	let find_term = |needle: &Type| value.iter().find(|(b, _)| needle.is_blade(b)).map(|(_, op)| op.clone());

	if value.keys().all(|b| is_blade_in_struct(struct_members, b)) {
		Some(Op::StructInstance {
			struct_name: struct_name.to_owned(),
			members: struct_members
				.iter()
				.map(|(name, t)| (name.to_string(), find_term(t).unwrap_or_else(Op::zero)))
				.collect(),
		})
	} else {
		None
	}
}

fn is_blade_in_struct(struct_members: &[(String, Type)], blade: &Blade) -> bool {
	struct_members.iter().any(|(_, t)| t.is_blade(blade))
}

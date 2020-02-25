use std::collections::BTreeMap;

use crate::*;

impl Op {
	/// Detect named types and replace
	pub fn typify(self, t: &Types, g: Option<&Grammar>) -> Self {
		// TODO: recurse!

		if let Op::Sum(terms) = &self {
			if let Some(value) = as_value(&terms, g) {
				if let Some(s) = find_struct(&value, t) {
					return s;
				}
			}
		}

		// A blade?
		if let Some((scalar, blade)) = self.as_blade() {
			if scalar == 0 {
				Op::zero()
			} else if blade.is_scalar() {
				Op::scalar(scalar)
			} else if let Some((sign, name)) = t.blade_name(&blade) {
				let blade = Op::var(name, &Type::Blade(sign, blade));
				match scalar {
					0 => Op::zero(),
					1 => blade,
					_ => Op::Term(blade.into(), scalar),
				}
			} else {
				self
			}
		} else {
			self
		}
	}
}

/// Sum of the values, grouped by their types.
type Value = BTreeMap<Blade, Op>;

fn as_value(terms: &[Op], g: Option<&Grammar>) -> Option<Value> {
	let mut parts: BTreeMap<Blade, Vec<Op>> = Default::default();
	for term in terms {
		let typ = term.typ(g)?;
		if !typ.is_zero() {
			if let Type::Blade(sign, blade) = typ {
				let term = if sign < 0 { term.clone().negate() } else { term.clone() };
				parts.entry(blade).or_default().push(term);
			} else {
				return None;
			}
		}
	}

	if parts.len() == 0 {
		return None;
	}

	Some(
		parts
			.into_iter()
			.map(|(typ, terms)| (typ, Op::Sum(terms).simplify(g)))
			.collect(),
	)
}

fn find_struct(sum: &Value, t: &Types) -> Option<Op> {
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
			name: struct_name.to_owned(),
			members: struct_members
				.iter()
				.map(|(name, t)| (name.to_string(), find_term(t).unwrap_or_else(|| Op::zero())))
				.collect(),
		})
	} else {
		None
	}
}

fn is_blade_in_struct(struct_members: &[(String, Type)], blade: &Blade) -> bool {
	struct_members.iter().any(|(_, t)| t.is_blade(blade))
}

use crate::*;

impl Expr {
	/// Turn `y ^ x` into `-XY(y.0 * x.0)` so we can easier see sign changes,
	/// and easier copy/paste code
	#[must_use]
	pub fn explicit(self, t: &Types, g: &Grammar) -> Expr {
		match self {
			Expr::Var { .. } => self,
			Expr::Vec(_) => self,
			Expr::Unary(unary, expr) => Expr::Unary(unary, expr.explicit(t, g).into()),
			Expr::Term(expr, scalar) => Expr::Term(expr.explicit(t, g).into(), scalar),
			Expr::Sum(terms) => Expr::Sum(terms.into_iter().map(|e| e.explicit(t, g)).collect()),
			Expr::Prod(product, factors) => {
				let factors: Vec<Expr> = factors.into_iter().map(|e| e.explicit(t, g)).collect();
				explicit_product(product, &factors, t, g).unwrap_or_else(|| Expr::Prod(product, factors))
			}
			Expr::StructInstance(si) => Expr::StructInstance(StructInstance {
				struct_name: si.struct_name,
				strct: si.strct,
				members: si
					.members
					.into_iter()
					.map(|(name, expr)| (name, expr.explicit(t, g)))
					.collect(),
			}),
		}
	}
}

/// Turn `y ^ x` into `-XY(y.0 * x.0)`
fn explicit_product(product: Product, factors: &[Expr], t: &Types, g: &Grammar) -> Option<Expr> {
	if factors.len() < 2 {
		return None;
	}
	let mut names = Vec::with_capacity(factors.len());
	let mut sblades = Vec::with_capacity(factors.len());
	for factor in factors {
		if let Expr::Var { name, typ, .. } = factor {
			let sblade = typ.clone().into_sblade()?;
			names.push(format!("{}.0", name));
			sblades.push(sblade);
		} else {
			return None;
		}
	}

	let product_sblade = SBlade::product(product, &sblades, g);
	let (sign, type_name) = t.get_sblade(&product_sblade)?;
	assert!(sign.abs() == 1);

	use itertools::Itertools;

	let var = Expr::Var {
		order: 0,
		name: format!("{}({})", type_name, names.iter().join(" * ")),
		typ: Type::SBlade(product_sblade),
	};

	if sign == -1 {
		Some(Expr::Term(var.into(), -1))
	} else {
		Some(var)
	}
}

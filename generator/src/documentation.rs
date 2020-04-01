use {
	itertools::{chain, Itertools},
	strum::IntoEnumIterator,
};

use crate::*;

pub fn unary_table(unit_blades: &[Expr], rust: &impl Fn(Expr) -> String) -> String {
	markdown::table(
		chain(Some("Op \\ Blade".to_owned()), unit_blades.iter().cloned().map(rust)),
		Unary::iter().map(|unary| {
			chain(
				Some(unary.short_description().to_owned()),
				unit_blades.iter().map(|blade| rust(Expr::unary(unary, blade.clone()))),
			)
			.collect()
		}),
	)
}

pub fn multiplication_tables(unit_blades: &[Expr], rust: &impl Fn(Expr) -> String) -> String {
	Product::iter()
		.map(|prod| {
			format!(
				"### {} multiplication table\n\n{}\n",
				prod.trait_name(),
				multiplication_table(unit_blades, Product::Geometric, rust)
			)
		})
		.join("\n")
}

pub fn multiplication_table(unit_blades: &[Expr], product: Product, rust: &impl Fn(Expr) -> String) -> String {
	markdown::table(
		chain(Some("".to_owned()), unit_blades.iter().cloned().map(rust)),
		unit_blades.iter().map(|l| {
			chain(
				Some(rust(l.clone())),
				unit_blades
					.iter()
					.map(|r| rust(Expr::Prod(product, vec![l.clone(), r.clone()]))),
			)
			.collect()
		}),
	)
}

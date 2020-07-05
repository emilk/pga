use {itertools::Itertools, strum::IntoEnumIterator};

use crate::{documentation::*, *};

const CODE_SEPARATOR: &str = "// ---------------------------------------------------------------------";

pub struct Settings {
	pub float_type: String,
}

impl Default for Settings {
	fn default() -> Self {
		Self {
			float_type: "f64".to_string(),
		}
	}
}

pub struct Generator {
	pub grammar: Grammar,
	pub types: Types,
	pub settings: Settings,
	pub ro: RustOptions,
}

impl Generator {
	fn rust(&self, expr: Expr) -> String {
		expr.simplify(Some(&self.grammar))
			.typify(&self.types, &self.grammar)
			.rust(&self.ro)
	}
}

pub mod blades {
	use super::*;

	pub fn file(gen: &Generator) -> String {
		let documentation = with_line_prefixes("//! ", &documentation(gen).trim());
		format!(
			"\
        {}\n\n\
        use derive_more::{{Add, Mul, Neg, Sub}};\n\
        \n\
        use super::*;\n\
        \n\
        {}\n\n\
        {}\n\
        {}\n\n\
        {}\n\
        {}\n",
			documentation,
			declare_blades(gen),
			CODE_SEPARATOR,
			impl_blade_unaryops(gen),
			CODE_SEPARATOR,
			impl_blade_products(gen),
		)
	}

	fn documentation(gen: &Generator) -> String {
		let rust = |expr| gen.rust(expr);
		let unit_blades = gen.types.unit_blades();
		format!(
			"\
    # Blade types\n\
    The blades that make up this geometric algebra.\n\
    \n\
    ## Unary operations\n\
    {}\n\
    \n\
    ## Multiplication tables\n\
    {}\n\
    ",
			unary_table(&unit_blades, &rust),
			multiplication_tables(&unit_blades, &rust)
		)
	}

	fn declare_blades(gen: &Generator) -> String {
		gen.types
			.sblades()
			.iter()
			.map(|(name, sb)| declare_blade(gen, name, sb))
			.join("\n\n")
	}

	fn declare_blade(gen: &Generator, name: &str, sb: &SBlade) -> String {
		let is_scalar = sb.grade() == 0;
		let is_pseudoscalar = sb.grade() == gen.grammar.num_vecs();

		let mut code = String::new();
		if is_scalar {
			code += "/// The scalar type (real numbers).\n";
		} else if is_pseudoscalar {
			code += "/// The pseudo-scalar.\n"
		};

		let squares_to = sb.geometric_product(&sb, &gen.grammar);
		assert!(squares_to.is_scalar());
		let squares_to = squares_to.sign;
		code += &format!("/// Squares to {}.\n", squares_to);

		let mut derives = "Copy, Clone, Debug, Default, PartialEq, PartialOrd, Neg, Add, Sub".to_string();
		if is_scalar {
			derives += ", Mul"; // NOTE: NOT the pseudoscalar (it may scale to zero)
		}
		code += &format!("#[derive({})]\n", derives);

		format!("{}pub struct {}(pub {});", code, name, gen.settings.float_type)
	}

	fn impl_blade_unaryops(gen: &Generator) -> String {
		Unary::iter()
			.map(|unary| {
				format!(
					"// impl {} for blades:\n\n{}",
					unary.trait_name(),
					gen.types
						.sblades()
						.iter()
						.map(|(sblade_name, sblade)| { impl_blade_unary(gen, sblade_name, sblade, unary) })
						.join("\n\n")
				)
			})
			.join(&format!("\n\n{}\n", CODE_SEPARATOR))
	}

	fn impl_blade_unary(gen: &Generator, sblade_name: &str, sblade: &SBlade, unary: Unary) -> String {
		let result_type = sblade.unary(unary, &gen.grammar);

		if result_type.is_zero() {
			format!(" // Omitted: {}.{}() -> 0", sblade_name, unary.trait_function_name(),)
		} else {
			let (sign, output_sblade_name) = gen
				.types
				.get_sblade(&result_type)
				.unwrap_or_else(|| panic!("unknown blade: {:?}", result_type.blade));
			assert_eq!(sign.abs(), 1);

			let sign = if sign == -1 { "-" } else { "" };

			if unary.trait_has_output_type() {
				format!(
					r"
        impl {Trait} for {sblade_name} {{
            type Output = {Output};
            fn {function_name}(self) -> Self::Output {{
                {Output}({sign} self.0)
            }}
        }}
        ",
					sblade_name = sblade_name,
					Trait = unary.trait_name(),
					function_name = unary.trait_function_name(),
					Output = output_sblade_name,
					sign = sign,
				)
			} else {
				format!(
					r"
        impl {Trait} for {sblade_name} {{
            fn {function_name}(self) -> Self {{
                {sign} self
            }}
        }}
        ",
					sblade_name = sblade_name,
					Trait = unary.trait_name(),
					function_name = unary.trait_function_name(),
					sign = sign,
				)
			}
		}
	}

	fn impl_blade_products(gen: &Generator) -> String {
		Product::iter()
			.map(|prod| {
				format!(
					"// impl {} for blades:\n\n{}",
					prod.trait_name(),
					impl_product_for_blades(gen, prod)
				)
			})
			.join(&format!("\n\n{}\n", CODE_SEPARATOR))
	}

	fn impl_product_for_blades(gen: &Generator, product: Product) -> String {
		gen.types
			.sblades()
			.iter()
			.map(|lhs| {
				gen.types
					.sblades()
					.iter()
					.map(|rhs| impl_blade_product(gen, lhs, rhs, product))
					.join("\n\n")
			})
			.join("\n\n")
	}

	fn impl_blade_product(gen: &Generator, lhs: &(&str, SBlade), rhs: &(&str, SBlade), product: Product) -> String {
		let product_type = SBlade::product(product, &[lhs.1.clone(), rhs.1.clone()], &gen.grammar);

		if product_type.is_zero() {
			format!(
				r"
    impl {Trait}<{Rhs}> for {Lhs} {{
        type Output = Zero;
        fn {function_name}(self, _rhs: {Rhs}) -> Self::Output {{
            Zero {{}}
        }}
    }}
    ",
				Lhs = lhs.0,
				Rhs = rhs.0,
				Trait = product.trait_name(),
				function_name = product.trait_function_name(),
			)
		} else {
			let (sign, output_sblade_name) = gen
				.types
				.get_sblade(&product_type)
				.unwrap_or_else(|| panic!("unknown blade: {:?}", product_type.blade));
			assert_eq!(sign.abs(), 1);

			format!(
				r"
    impl {Trait}<{Rhs}> for {Lhs} {{
        type Output = {Output};
        fn {function_name}(self, rhs: {Rhs}) -> Self::Output {{
            {Output}({sign} self.0 * rhs.0)
        }}
    }}
    ",
				Lhs = lhs.0,
				Rhs = rhs.0,
				Trait = product.trait_name(),
				function_name = product.trait_function_name(),
				Output = output_sblade_name,
				sign = if sign == -1 { "-" } else { "" }
			)
		}
	}
}

pub mod strct {
	use super::*;

	pub fn file(gen: &Generator, struct_name: &str, strct: &Struct) -> String {
		let documentation = with_line_prefixes("//! ", &documentation(gen, struct_name, strct).trim());

		let unaryops = Unary::iter()
			.map(|unary| impl_struct_unary(gen, struct_name, strct, unary))
			.join("\n");

		let binops = gen
			.types
			.structs()
			.map(|(rhs_name, rhs_struct)| {
				format!(
					"// {} OP {}:\n\n{}\n",
					struct_name,
					rhs_name,
					Product::iter()
						.map(|prod| impl_struct_product(gen, &(struct_name, strct), &(rhs_name, rhs_struct), prod))
						.join("\n")
				)
			})
			.join(&format!("\n{}\n", CODE_SEPARATOR));

		format!(
			"\
        {}\n\n\
        use super::*;\n\n\
        {}\n\
        {}\n\
        {}\n\
        {}\n\
        {}\n",
			documentation,
			declare_struct(gen, struct_name, strct),
			CODE_SEPARATOR,
			unaryops,
			CODE_SEPARATOR,
			binops,
		)
	}

	fn documentation(gen: &Generator, struct_name: &str, strct: &Struct) -> String {
		let homo_ops = Product::iter()
			.filter_map(|product| {
				struct_product_type_signature(gen, &(struct_name, strct), &(struct_name, strct), product)
			})
			.join("\n");

		let hetero_ops = gen
			.types
			.structs()
			.filter_map(|(other_struct_name, other_strct)| {
				if struct_name == other_struct_name {
					None
				} else {
					Some(
						Product::iter()
							.flat_map(|product| {
								itertools::chain(
									struct_product_type_signature(
										gen,
										&(struct_name, strct),
										&(other_struct_name, other_strct),
										product,
									),
									struct_product_type_signature(
										gen,
										&(other_struct_name, other_strct),
										&(struct_name, strct),
										product,
									),
								)
							})
							.join("\n"),
					)
				}
			})
			.join("\n");

		format!(
			"\
        # {}\n\n\
        ## Operations\n\
        ```text\n\
        {}\n\
        {}\n\
        ```\n\
        ",
			struct_name, homo_ops, hetero_ops
		)
	}

	fn declare_struct(_gen: &Generator, struct_name: &str, strct: &Struct) -> String {
		// TODO: we can only implement Add, Sub if the struct has no Type::Constant
		let derives =
            "Copy, Clone, Debug, Default, PartialEq, PartialOrd, derive_more::Neg, derive_more::Add, derive_more::Sub\n";
		let members = strct
			.iter()
			.map(|(member_name, member_type)| format!("    pub {}: {},", member_name, member_type.name))
			.join("\n");
		format!(
			"\
#[derive({})]\n\
pub struct {} {{\n\
    {}\n\
}}\n	",
			derives, struct_name, members
		)
	}

	pub fn impl_struct_unary(gen: &Generator, struct_name: &str, strct: &Struct, unary: Unary) -> String {
		let var = Expr::var(0, "self", &Type::strct(strct));
		let expr = Expr::unary(unary, var);
		let expr = expr.simplify(Some(&gen.grammar)).typify(&gen.types, &gen.grammar);
		let code = expr.rust(&gen.ro);
		match type_name(gen, &expr) {
			Some(output_type_name) => {
				if unary.trait_has_output_type() {
					format!(
						r"
                impl {Trait} for {struct_name} {{
                    type Output = {Output};
                    fn {function_name}(self) -> Self::Output {{
                        {code}
                    }}
                }}
                ",
						struct_name = struct_name,
						Trait = unary.trait_name(),
						function_name = unary.trait_function_name(),
						Output = output_type_name,
						code = code,
					)
				} else {
					format!(
						r"
                impl {Trait} for {struct_name} {{
                    fn {function_name}(self) -> Self {{
                        {code}
                    }}
                }}
                ",
						struct_name = struct_name,
						Trait = unary.trait_name(),
						function_name = unary.trait_function_name(),
						code = code,
					)
				}
			}
			None => format!(
				"// Omitted: {}.{}() -> {}",
				struct_name,
				unary.trait_function_name(),
				code.replace('\n', " ")
			),
		}
	}

	pub fn struct_product_type_signature(
		gen: &Generator,
		lhs: &(&str, &Struct),
		rhs: &(&str, &Struct),
		product: Product,
	) -> Option<String> {
		let factors = vec![
			Expr::var(0, lhs.0, &Type::strct(lhs.1)),
			Expr::var(1, rhs.0, &Type::strct(rhs.1)),
		];
		let input_expr = Expr::Prod(product, factors);
		let input_code = input_expr.rust(&gen.ro);
		let output_expr = input_expr.simplify(Some(&gen.grammar)).typify(&gen.types, &gen.grammar);
		let type_name = type_name(gen, &output_expr)?;
		Some(format!("{} -> {}", input_code, type_name))
	}

	pub fn impl_struct_product(
		gen: &Generator,
		lhs: &(&str, &Struct),
		rhs: &(&str, &Struct),
		product: Product,
	) -> String {
		let factors = vec![
			Expr::var(0, "self", &Type::strct(lhs.1)),
			Expr::var(1, "rhs", &Type::strct(rhs.1)), // TODO: name this snake_case(rhs.0) iff lhs type != rhs.type
		];
		let expr = Expr::Prod(product, factors);
		let expr = expr.simplify(Some(&gen.grammar));
		let expr = expr.typify(&gen.types, &gen.grammar);

		match type_name(gen, &expr) {
			Some(output_type_name) => {
				let explicit = expr
					.clone()
					.typify(&gen.types, &gen.grammar)
					.explicit(&gen.types, &gen.grammar)
					.simplify(Some(&gen.grammar))
					.rust(&gen.ro);

				let code = expr.rust(&gen.ro);

				let code = format!("{}\n{}", with_line_prefixes("// ", &explicit), code);
				let code = rust::indent_n(2, &code);

				format!(
					r"
// {comment}
impl {Trait}<{Rhs}> for {Lhs} {{
    type Output = {Output};
    fn {function_name}(self, rhs: {Rhs}) -> Self::Output {{
{code}
    }}
}}
        ",
					comment = struct_product_type_signature(gen, lhs, rhs, product).unwrap(),
					Lhs = lhs.0,
					Rhs = rhs.0,
					Trait = product.trait_name(),
					function_name = product.trait_function_name(),
					Output = output_type_name,
					code = code,
				)
			}
			None => format!(
				"// Omitted: {} {} {} = {}",
				lhs.0,
				product.trait_function_name(),
				rhs.0,
				expr.rust(&RustOptions { operators: true }).replace('\n', " ")
			),
		}
	}
}

/// Returns None if the type is Zero or unknown
fn type_name(gen: &Generator, expr: &Expr) -> Option<String> {
	// println!("type_name({})", expr.rust(&gen.ro));
	let output_type = expr.typ(Some(&gen.grammar));
	// println!("type_name({}) output_type: {:?}", expr.rust(&gen.ro), output_type);
	let output_type = output_type?;
	if output_type.is_zero() {
		None
	} else {
		Some(gen.types.type_name(&output_type).to_owned())
	}
}

use std::io::Write;

use {
	itertools::{chain, izip, Itertools},
	strum::IntoEnumIterator,
};

use symbolic::*;

fn unary_table(unit_blades: &[Expr], rust: &impl Fn(Expr) -> String) -> String {
	markdown_table(
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

fn markdown_table(headers: impl IntoIterator<Item = String>, rows: impl IntoIterator<Item = Vec<String>>) -> String {
	let headers: Vec<String> = headers.into_iter().collect();
	let rows: Vec<Vec<String>> = rows.into_iter().collect();

	let mut col_widths: Vec<usize> = headers.iter().map(String::len).collect();
	for row in &rows {
		assert_eq!(row.len(), col_widths.len());
		for (col_idx, cell) in row.iter().enumerate() {
			col_widths[col_idx] = col_widths[col_idx].max(cell.len());
		}
	}

	let mut s = vec![];
	write!(
		&mut s,
		"| {} |\n",
		izip!(&headers, &col_widths)
			.map(|(header, width)| format!("{:<w$}", header, w = width))
			.format(" | ")
	)
	.unwrap();
	write!(
		&mut s,
		"| {} |\n",
		col_widths
			.iter()
			.map(|width| format!("{:-<w$}", "", w = width))
			.format(" | ")
	)
	.unwrap();
	for row in &rows {
		write!(
			&mut s,
			"| {} |\n",
			izip!(row, &col_widths)
				.map(|(cell, width)| format!("{:<w$}", cell, w = width))
				.format(" | ")
		)
		.unwrap();
	}
	String::from_utf8(s).unwrap()
}

fn multiplication_tables(unit_blades: &[Expr], rust: &impl Fn(Expr) -> String) -> String {
	let mut text = Vec::new();
	let w = &mut text;

	writeln!(w, "Geometric multiplication table:").unwrap();
	writeln!(w, "{}", multiplication_table(unit_blades, Product::Geometric, rust)).unwrap();

	writeln!(w, "Geometric anti-product multiplication table:").unwrap();
	writeln!(w, "{}", multiplication_table(unit_blades, Product::AntiGeometric, rust)).unwrap();

	writeln!(w, "Dot multiplication table:").unwrap();
	writeln!(w, "{}", multiplication_table(unit_blades, Product::Dot, rust)).unwrap();

	writeln!(w, "Wedge multiplication table:").unwrap();
	writeln!(w, "{}", multiplication_table(unit_blades, Product::Wedge, rust)).unwrap();

	writeln!(w, "Antiwedge multiplication table:").unwrap();
	writeln!(w, "{}", multiplication_table(unit_blades, Product::Antiwedge, rust)).unwrap();
	String::from_utf8(text).unwrap()
}

fn multiplication_table(unit_blades: &[Expr], product: Product, rust: &impl Fn(Expr) -> String) -> String {
	markdown_table(
		chain(Some("l \\ r".to_owned()), unit_blades.iter().cloned().map(rust)),
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

fn pga2d_types() -> Types {
	let mut t = Types::default();
	let x = VecIdx(0);
	let y = VecIdx(1);
	let w = VecIdx(2);
	t.insert("R", Type::scalar());
	t.insert("X", Type::vec(x));
	t.insert("Y", Type::vec(y));
	t.insert("W", Type::vec(w));
	t.insert("YW", Type::unsorted_blade(&[y, w]));
	t.insert("WX", Type::unsorted_blade(&[w, x]));
	t.insert("XY", Type::unsorted_blade(&[x, y]));
	t.insert("XYW", Type::unsorted_blade(&[x, y, w]));

	t.insert(
		"Point",
		Type::Struct(vec![
			("x".to_string(), t.get("X").clone()),
			("y".to_string(), t.get("Y").clone()),
			("w".to_string(), t.get("W").clone()),
		]),
	);

	t.insert(
		"Line",
		Type::Struct(vec![
			("yw".to_string(), t.get("YW").clone()),
			("wx".to_string(), t.get("WX").clone()),
			("xy".to_string(), t.get("XY").clone()),
		]),
	);

	t
}

/// Using the notation of Eric Lengyel.
/// See http://terathon.com/blog/projective-geometric-algebra-done-right/
fn pga3d_lengyel() -> (Grammar, Types) {
	let g = Grammar(vec![1, 1, 1, 0]);
	let mut t = Types::default();

	let e1 = VecIdx(0);
	let e2 = VecIdx(1);
	let e3 = VecIdx(2);
	let e4 = VecIdx(3);
	t.insert("s", Type::scalar());

	t.insert("e1", Type::vec(e1));
	t.insert("e2", Type::vec(e2));
	t.insert("e3", Type::vec(e3));
	t.insert("e4", Type::vec(e4));
	t.insert("e41", Type::unsorted_blade(&[e4, e1]));
	t.insert("e42", Type::unsorted_blade(&[e4, e2]));
	t.insert("e43", Type::unsorted_blade(&[e4, e3]));
	t.insert("e23", Type::unsorted_blade(&[e2, e3]));
	t.insert("e31", Type::unsorted_blade(&[e3, e1]));
	t.insert("e12", Type::unsorted_blade(&[e1, e2]));
	t.insert("e234", Type::unsorted_blade(&[e2, e3, e4]));
	t.insert("e314", Type::unsorted_blade(&[e3, e1, e4]));
	t.insert("e124", Type::unsorted_blade(&[e1, e2, e4]));
	t.insert("e321", Type::unsorted_blade(&[e3, e2, e1]));
	t.insert("E4", Type::unsorted_blade(&[e1, e2, e3, e4]));

	t.insert(
		"Point",
		Type::Struct(vec![
			("x".to_string(), t.get("e1").clone()),
			("y".to_string(), t.get("e2").clone()),
			("z".to_string(), t.get("e3").clone()),
			("w".to_string(), t.get("e4").clone()),
		]),
	);

	t.insert(
		"Line",
		Type::Struct(vec![
			("vx".to_string(), t.get("e41").clone()),
			("vy".to_string(), t.get("e42").clone()),
			("vz".to_string(), t.get("e43").clone()),
			("mx".to_string(), t.get("e23").clone()),
			("my".to_string(), t.get("e31").clone()),
			("mz".to_string(), t.get("e12").clone()),
		]),
	);

	t.insert(
		"Plane",
		Type::Struct(vec![
			("nx".to_string(), t.get("e234").clone()),
			("ny".to_string(), t.get("e314").clone()),
			("nz".to_string(), t.get("e124").clone()),
			("d".to_string(), t.get("e321").clone()),
		]),
	);

	(g, t)
}

fn main() {
	// test_pga2d();
	// test_pga3d_lengyel();
}

fn tokenize(s: &str) -> Vec<&str> {
	s.trim().split_ascii_whitespace().collect()
}

#[cfg(test)]
macro_rules! assert_eq_ignoring_whitespace {
	($l:expr, $r:expr) => {{
		let l = $l;
		let r = $r;
		if tokenize(&l) != tokenize(&r) {
			panic!("Got this:\n{}\n\nExpected this:\n{}", l, r);
			}
		}};
	($l:expr, $r:expr, $msg:expr) => {{
		let l = $l;
		let r = $r;
		if tokenize(&l) != tokenize(&r) {
			panic!("{}. Got this:\n{}\n\nExpected this:\n{}", $msg, l, r);
			}
		}};
}

#[test]
fn test_pga3d_lengyel() {
	let (g, t) = pga3d_lengyel();
	let rust = |expr: Expr| expr.simplify(Some(&g)).typify(&t, &g).rust();
	let unit_blades = t.unit_blades();
	// println!("{}", multiplication_tables(&unit_blades, &rust));

	assert_eq_ignoring_whitespace!(
		unary_table(&unit_blades, &rust),
		r"
| Op \ Blade       | 1  | e1    | e2    | e3    | e4    | e41  | e42  | e43  | e23  | e31  | e12  | e234  | e314  | e124  | e321  | E4 |
| ---------------- | -- | ----- | ----- | ----- | ----- | ---- | ---- | ---- | ---- | ---- | ---- | ----- | ----- | ----- | ----- | -- |
| Right complement | E4 | e234  | e314  | e124  | e321  | -e23 | -e31 | -e12 | -e41 | -e42 | -e43 | -e1   | -e2   | -e3   | -e4   | 1  |
| Left complement  | E4 | -e234 | -e314 | -e124 | -e321 | -e23 | -e31 | -e12 | -e41 | -e42 | -e43 | e1    | e2    | e3    | e4    | 1  |
| Reverse          | 1  | e1    | e2    | e3    | e4    | -e41 | -e42 | -e43 | -e23 | -e31 | -e12 | -e234 | -e314 | -e124 | -e321 | E4 |
| Antireverse      | 1  | -e1   | -e2   | -e3   | -e4   | -e41 | -e42 | -e43 | -e23 | -e31 | -e12 | e234  | e314  | e124  | e321  | E4 |
"
	);

	assert_eq_ignoring_whitespace!(
		multiplication_table(&unit_blades, Product::Geometric, &rust),
		r"
| l \ r | 1    | e1    | e2    | e3    | e4   | e41   | e42   | e43   | e23   | e31   | e12   | e234  | e314  | e124  | e321  | E4   |
| ----- | ---- | ----- | ----- | ----- | ---- | ----- | ----- | ----- | ----- | ----- | ----- | ----- | ----- | ----- | ----- | ---- |
| 1     | 1    | e1    | e2    | e3    | e4   | e41   | e42   | e43   | e23   | e31   | e12   | e234  | e314  | e124  | e321  | E4   |
| e1    | e1   | 1     | e12   | -e31  | -e41 | -e4   | -e124 | e314  | -e321 | -e3   | e2    | E4    | e43   | -e42  | -e23  | e234 |
| e2    | e2   | -e12  | 1     | e23   | -e42 | e124  | -e4   | -e234 | e3    | -e321 | -e1   | -e43  | E4    | e41   | -e31  | e314 |
| e3    | e3   | e31   | -e23  | 1     | -e43 | -e314 | e234  | -e4   | -e2   | e1    | -e321 | e42   | -e41  | E4    | -e12  | e124 |
| e4    | e4   | e41   | e42   | e43   | 0    | 0     | 0     | 0     | e234  | e314  | e124  | 0     | 0     | 0     | E4    | 0    |
| e41   | e41  | e4    | e124  | -e314 | 0    | 0     | 0     | 0     | -E4   | -e43  | e42   | 0     | 0     | 0     | -e234 | 0    |
| e42   | e42  | -e124 | e4    | e234  | 0    | 0     | 0     | 0     | e43   | -E4   | -e41  | 0     | 0     | 0     | -e314 | 0    |
| e43   | e43  | e314  | -e234 | e4    | 0    | 0     | 0     | 0     | -e42  | e41   | -E4   | 0     | 0     | 0     | -e124 | 0    |
| e23   | e23  | -e321 | -e3   | e2    | e234 | -E4   | -e43  | e42   | -1    | -e12  | e31   | -e4   | -e124 | e314  | e1    | e41  |
| e31   | e31  | e3    | -e321 | -e1   | e314 | e43   | -E4   | -e41  | e12   | -1    | -e23  | e124  | -e4   | -e234 | e2    | e42  |
| e12   | e12  | -e2   | e1    | -e321 | e124 | -e42  | e41   | -E4   | -e31  | e23   | -1    | -e314 | e234  | -e4   | e3    | e43  |
| e234  | e234 | -E4   | -e43  | e42   | 0    | 0     | 0     | 0     | -e4   | -e124 | e314  | 0     | 0     | 0     | e41   | 0    |
| e314  | e314 | e43   | -E4   | -e41  | 0    | 0     | 0     | 0     | e124  | -e4   | -e234 | 0     | 0     | 0     | e42   | 0    |
| e124  | e124 | -e42  | e41   | -E4   | 0    | 0     | 0     | 0     | -e314 | e234  | -e4   | 0     | 0     | 0     | e43   | 0    |
| e321  | e321 | -e23  | -e31  | -e12  | -E4  | e234  | e314  | e124  | e1    | e2    | e3    | -e41  | -e42  | -e43  | -1    | e4   |
| E4    | E4   | -e234 | -e314 | -e124 | 0    | 0     | 0     | 0     | e41   | e42   | e43   | 0     | 0     | 0     | -e4   | 0    |
  "
	);

	assert_eq_ignoring_whitespace!(
		multiplication_table(&unit_blades, Product::AntiGeometric, &rust),
		r"
| l \ r | 1     | e1    | e2    | e3    | e4   | e41   | e42   | e43   | e23  | e31  | e12  | e234  | e314  | e124  | e321 | E4   |
| ----- | ----- | ----- | ----- | ----- | ---- | ----- | ----- | ----- | ---- | ---- | ---- | ----- | ----- | ----- | ---- | ---- |
| 1     | 0     | 0     | 0     | 0     | e321 | e23   | e31   | e12   | 0    | 0    | 0    | e1    | e2    | e3    | 0    | 1    |
| e1    | 0     | 0     | 0     | 0     | -e23 | -e321 | e3    | -e2   | 0    | 0    | 0    | 1     | -e12  | e31   | 0    | e1   |
| e2    | 0     | 0     | 0     | 0     | -e31 | -e3   | -e321 | e1    | 0    | 0    | 0    | e12   | 1     | -e23  | 0    | e2   |
| e3    | 0     | 0     | 0     | 0     | -e12 | e2    | -e1   | -e321 | 0    | 0    | 0    | -e31  | e23   | 1     | 0    | e3   |
| e4    | -e321 | e23   | e31   | e12   | -E4  | e234  | e314  | e124  | -e1  | -e2  | -e3  | -e41  | -e42  | -e43  | 1    | e4   |
| e41   | e23   | -e321 | e3    | -e2   | e234 | -E4   | e43   | -e42  | -1   | e12  | -e31 | -e4   | e124  | -e314 | e1   | e41  |
| e42   | e31   | -e3   | -e321 | e1    | e314 | -e43  | -E4   | e41   | -e12 | -1   | e23  | -e124 | -e4   | e234  | e2   | e42  |
| e43   | e12   | e2    | -e1   | -e321 | e124 | e42   | -e41  | -E4   | e31  | -e23 | -1   | e314  | -e234 | -e4   | e3   | e43  |
| e23   | 0     | 0     | 0     | 0     | e1   | -1    | e12   | -e31  | 0    | 0    | 0    | -e321 | e3    | -e2   | 0    | e23  |
| e31   | 0     | 0     | 0     | 0     | e2   | -e12  | -1    | e23   | 0    | 0    | 0    | -e3   | -e321 | e1    | 0    | e31  |
| e12   | 0     | 0     | 0     | 0     | e3   | e31   | -e23  | -1    | 0    | 0    | 0    | e2    | -e1   | -e321 | 0    | e12  |
| e234  | -e1   | -1    | e12   | -e31  | -e41 | -e4   | e124  | -e314 | e321 | -e3  | e2   | E4    | -e43  | e42   | e23  | e234 |
| e314  | -e2   | -e12  | -1    | e23   | -e42 | -e124 | -e4   | e234  | e3   | e321 | -e1  | e43   | E4    | -e41  | e31  | e314 |
| e124  | -e3   | e31   | -e23  | -1    | -e43 | e314  | -e234 | -e4   | -e2  | e1   | e321 | -e42  | e41   | E4    | e12  | e124 |
| e321  | 0     | 0     | 0     | 0     | -1   | e1    | e2    | e3    | 0    | 0    | 0    | -e23  | -e31  | -e12  | 0    | e321 |
| E4    | 1     | e1    | e2    | e3    | e4   | e41   | e42   | e43   | e23  | e31  | e12  | e234  | e314  | e124  | e321 | E4   |
  "
	);

	// Normalized equclidean point
	let point = Type::Struct(vec![
		("x".to_string(), t.get("e1").clone()),
		("y".to_string(), t.get("e2").clone()),
		("z".to_string(), t.get("e3").clone()),
		(
			"w".to_string(),
			Type::constant(t.get("e4").clone().into_sblade().unwrap()),
		),
	]);
	assert_eq_ignoring_whitespace!(
		rust(Expr::wedge(vec![Expr::var("p", &point), Expr::var("q", &point)])),
		"
Line {
    vx: p.x ^ e4 - q.x ^ e4,
    vy: p.y ^ e4 - q.y ^ e4,
    vz: p.z ^ e4 - q.z ^ e4,
    mx: p.y ^ q.z - p.z ^ q.y,
    my: p.x ^ q.z - p.z ^ q.x,
    mz: p.x ^ q.y - p.y ^ q.x,
}
	"
	);
}

#[test]
fn test_pga2d() {
	let t = pga2d_types();
	let g = Grammar(vec![1, 1, 0]);
	let rust = |expr: Expr| expr.simplify(Some(&g)).typify(&t, &g).rust();

	// let unit_blades = t.unit_blades();
	// println!("{}", multiplication_tables(&unit_blades, &rust));

	assert_eq_ignoring_whitespace!(t.get("WX").unit().rust(), "-_e0 ^ _e2", "Ugly output without typify");
	assert_eq_ignoring_whitespace!(rust(t.get("WX").unit()), "WX");

	assert_eq_ignoring_whitespace!(
		Expr::dot(vec![t.get("XY").unit()]).simplify(Some(&g)).rust(),
		"_e0 ^ _e1"
	);
	assert_eq_ignoring_whitespace!(rust(Expr::dot(vec![t.get("XY").unit(), Expr::one()])), "XY");

	let x_type = t.get("X");
	let y_type = t.get("Y");
	assert_eq_ignoring_whitespace!(rust(x_type.unit()), "X");
	assert_eq_ignoring_whitespace!(rust(Expr::wedge(vec![x_type.unit(), y_type.unit()])), "XY");
	let expr = Expr::Sum(vec![
		Expr::wedge(vec![x_type.unit(), y_type.unit()]),
		Expr::wedge(vec![y_type.unit(), x_type.unit()]),
	]);
	assert_eq_ignoring_whitespace!(
		expr.rust(),
		"_e0 ^ _e1 + _e1 ^ _e0",
		"Hard to read without running typify"
	);
	assert_eq_ignoring_whitespace!(rust(expr), "0");

	let point = t.get("Point");
	assert_eq_ignoring_whitespace!(
		rust(Expr::wedge(vec![Expr::var("l", point), Expr::var("r", point)])),
		r"
Line {
    yw: -l.w ^ r.y + l.y ^ r.w,
    wx: -l.w ^ r.x + l.x ^ r.w,
    xy: l.x ^ r.y - l.y ^ r.x,
}"
		.trim()
	);

	let line = t.get("Line");
	assert_eq_ignoring_whitespace!(
		rust(Expr::antiwedge(vec![Expr::var("l", line), Expr::var("r", line)])),
		r"
Point {
    x: l.wx & r.xy - l.xy & r.wx,
    y: l.xy & r.yw - l.yw & r.xy,
    w: -l.wx & r.yw + l.yw & r.wx,
}
"
		.trim()
	);

	assert_eq_ignoring_whitespace!(
		rust(Expr::geometric(vec![Expr::var("p", point), Expr::var("p", point)])),
		r"p.x * p.x + p.y * p.y"
	);

	assert_eq_ignoring_whitespace!(
		rust(Expr::geometric(vec![Expr::var("l", line), Expr::var("l", line)])),
		r"l.xy * l.xy"
	);
}

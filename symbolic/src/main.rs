use std::io::Write;

use symbolic::*;

fn multiplication_tables(unit_blades: &[Op], rust: &impl Fn(Op) -> String) -> String {
	let mut text = Vec::new();
	let w = &mut text;

	writeln!(w, "Geometric multiplication table (left side * top row):").unwrap();
	writeln!(w, "{}", multiplication_table(unit_blades, Product::Geometric, rust)).unwrap();

	writeln!(w, "Geometric anti-product multiplication table (left side !* top row):").unwrap();
	writeln!(w, "{}", multiplication_table(unit_blades, Product::AntiGeometric, rust)).unwrap();

	writeln!(w, "Dot multiplication table (left side | top row):").unwrap();
	writeln!(w, "{}", multiplication_table(unit_blades, Product::Dot, rust)).unwrap();

	writeln!(w, "Wedge multiplication table (left side ^ top row):").unwrap();
	writeln!(w, "{}", multiplication_table(unit_blades, Product::Wedge, rust)).unwrap();

	writeln!(w, "Antiwedge multiplication table (right side & bottom row):").unwrap();
	writeln!(w, "{}", multiplication_table(unit_blades, Product::Antiwedge, rust)).unwrap();
	String::from_utf8(text).unwrap()
}

fn multiplication_table(unit_blades: &[Op], product: Product, rust: &impl Fn(Op) -> String) -> String {
	let mut text = Vec::new();
	for a in unit_blades {
		write!(&mut text, "  ").unwrap();
		for b in unit_blades {
			let prod = Op::Prod(product, vec![a.clone(), b.clone()]);
			write!(&mut text, "{:<5} ", rust(prod)).unwrap();
		}
		writeln!(&mut text,).unwrap();
	}
	String::from_utf8(text).unwrap()
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
fn test_pga2d() {
	let t = pga2d_types();
	let g = Grammar(vec![1, 1, 0]);
	let rust = |op: Op| op.simplify(Some(&g)).typify(&t, &g).rust();

	let x_type = t.get("X");
	let y_type = t.get("Y");

	let unit_blades = t.unit_blades();

	println!("{}", multiplication_tables(&unit_blades, &rust));

	assert_eq_ignoring_whitespace!(t.get("WX").unit().rust(), "-e0 ^ e2");
	assert_eq_ignoring_whitespace!(rust(t.get("WX").unit()), "WX");

	assert_eq_ignoring_whitespace!(Op::dot(vec![t.get("XY").unit()]).simplify(Some(&g)).rust(), "e0 ^ e1");
	assert_eq_ignoring_whitespace!(rust(Op::dot(vec![t.get("XY").unit(), Op::one()])), "XY");

	assert_eq_ignoring_whitespace!(rust(x_type.unit()), "X");

	assert_eq_ignoring_whitespace!(rust(Op::wedge(vec![x_type.unit(), y_type.unit()])), "XY");

	let op = Op::Sum(vec![
		Op::wedge(vec![x_type.unit(), y_type.unit()]),
		Op::wedge(vec![y_type.unit(), x_type.unit()]),
	]);
	assert_eq_ignoring_whitespace!(op.rust(), "e0 ^ e1 + e1 ^ e0", "Hard to read without running typify");

	assert_eq_ignoring_whitespace!(rust(op), "0");

	let point = t.get("Point");
	assert_eq_ignoring_whitespace!(
		rust(Op::wedge(vec![Op::var("l", point), Op::var("r", point)])),
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
		rust(Op::antiwedge(vec![Op::var("l", line), Op::var("r", line)])),
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
		rust(Op::geometric(vec![Op::var("p", point), Op::var("p", point)])),
		r"p.x * p.x + p.y * p.y"
	);

	assert_eq_ignoring_whitespace!(
		rust(Op::geometric(vec![Op::var("l", line), Op::var("l", line)])),
		r"l.xy * l.xy"
	);
}

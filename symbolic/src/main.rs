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
	let rust = |op: Op| op.simplify(Some(&g)).typify(&t, &g).rust();
	let unit_blades = t.unit_blades();
	// println!("{}", multiplication_tables(&unit_blades, &rust));

	assert_eq_ignoring_whitespace!(
		multiplication_table(&unit_blades, Product::Geometric, &rust),
		r"
  1     e1    e2    e3    e4    e41   e42   e43   e23   e31   e12   e234  e314  e124  e321  E4
  e1    1     e12   -e31  -e41  -e4   -e124 e314  -e321 -e3   e2    E4    e43   -e42  -e23  e234
  e2    -e12  1     e23   -e42  e124  -e4   -e234 e3    -e321 -e1   -e43  E4    e41   -e31  e314
  e3    e31   -e23  1     -e43  -e314 e234  -e4   -e2   e1    -e321 e42   -e41  E4    -e12  e124
  e4    e41   e42   e43   0     0     0     0     e234  e314  e124  0     0     0     E4    0
  e41   e4    e124  -e314 0     0     0     0     -E4   -e43  e42   0     0     0     -e234 0
  e42   -e124 e4    e234  0     0     0     0     e43   -E4   -e41  0     0     0     -e314 0
  e43   e314  -e234 e4    0     0     0     0     -e42  e41   -E4   0     0     0     -e124 0
  e23   -e321 -e3   e2    e234  -E4   -e43  e42   -1    -e12  e31   -e4   -e124 e314  e1    e41
  e31   e3    -e321 -e1   e314  e43   -E4   -e41  e12   -1    -e23  e124  -e4   -e234 e2    e42
  e12   -e2   e1    -e321 e124  -e42  e41   -E4   -e31  e23   -1    -e314 e234  -e4   e3    e43
  e234  -E4   -e43  e42   0     0     0     0     -e4   -e124 e314  0     0     0     e41   0
  e314  e43   -E4   -e41  0     0     0     0     e124  -e4   -e234 0     0     0     e42   0
  e124  -e42  e41   -E4   0     0     0     0     -e314 e234  -e4   0     0     0     e43   0
  e321  -e23  -e31  -e12  -E4   e234  e314  e124  e1    e2    e3    -e41  -e42  -e43  -1    e4
  E4    -e234 -e314 -e124 0     0     0     0     e41   e42   e43   0     0     0     -e4   0
  "
	);

	assert_eq_ignoring_whitespace!(
		multiplication_table(&unit_blades, Product::AntiGeometric, &rust),
		r"
  0     0     0     0     e321  e23   e31   e12   0     0     0     e1    e2    e3    0     1
  0     0     0     0     -e23  -e321 e3    -e2   0     0     0     1     -e12  e31   0     e1
  0     0     0     0     -e31  -e3   -e321 e1    0     0     0     e12   1     -e23  0     e2
  0     0     0     0     -e12  e2    -e1   -e321 0     0     0     -e31  e23   1     0     e3
  -e321 e23   e31   e12   -E4   e234  e314  e124  -e1   -e2   -e3   -e41  -e42  -e43  1     e4
  e23   -e321 e3    -e2   e234  -E4   e43   -e42  -1    e12   -e31  -e4   e124  -e314 e1    e41
  e31   -e3   -e321 e1    e314  -e43  -E4   e41   -e12  -1    e23   -e124 -e4   e234  e2    e42
  e12   e2    -e1   -e321 e124  e42   -e41  -E4   e31   -e23  -1    e314  -e234 -e4   e3    e43
  0     0     0     0     e1    -1    e12   -e31  0     0     0     -e321 e3    -e2   0     e23
  0     0     0     0     e2    -e12  -1    e23   0     0     0     -e3   -e321 e1    0     e31
  0     0     0     0     e3    e31   -e23  -1    0     0     0     e2    -e1   -e321 0     e12
  -e1   -1    e12   -e31  -e41  -e4   e124  -e314 e321  -e3   e2    E4    -e43  e42   e23   e234
  -e2   -e12  -1    e23   -e42  -e124 -e4   e234  e3    e321  -e1   e43   E4    -e41  e31   e314
  -e3   e31   -e23  -1    -e43  e314  -e234 -e4   -e2   e1    e321  -e42  e41   E4    e12   e124
  0     0     0     0     -1    e1    e2    e3    0     0     0     -e23  -e31  -e12  0     e321
  1     e1    e2    e3    e4    e41   e42   e43   e23   e31   e12   e234  e314  e124  e321  E4
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
		rust(Op::wedge(vec![Op::var("p", &point), Op::var("q", &point)])),
		"
Line {
    vx: p.x ^ e3 - q.x ^ e3,
    vy: p.y ^ e3 - q.y ^ e3,
    vz: p.z ^ e3 - q.z ^ e3,
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

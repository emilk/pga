use generator::{documentation::*, *};

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

fn tokenize(s: &str) -> Vec<&str> {
	s.trim().split_ascii_whitespace().collect()
}

#[test]
fn test_pga3d_lengyel() {
	let (g, t) = grammars::pga3d_lengyel();
	let rust = |expr: Expr| expr.simplify(Some(&g)).typify(&t, &g).rust_concise();
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
| Anti-reverse     | 1  | -e1   | -e2   | -e3   | -e4   | -e41 | -e42 | -e43 | -e23 | -e31 | -e12 | e234  | e314  | e124  | e321  | E4 |
"
	);

	assert_eq_ignoring_whitespace!(
		multiplication_table(&unit_blades, Product::Geometric, &rust),
		r"
|      | 1    | e1    | e2    | e3    | e4   | e41   | e42   | e43   | e23   | e31   | e12   | e234  | e314  | e124  | e321  | E4   |
| ---- | ---- | ----- | ----- | ----- | ---- | ----- | ----- | ----- | ----- | ----- | ----- | ----- | ----- | ----- | ----- | ---- |
| 1    | 1    | e1    | e2    | e3    | e4   | e41   | e42   | e43   | e23   | e31   | e12   | e234  | e314  | e124  | e321  | E4   |
| e1   | e1   | 1     | e12   | -e31  | -e41 | -e4   | -e124 | e314  | -e321 | -e3   | e2    | E4    | e43   | -e42  | -e23  | e234 |
| e2   | e2   | -e12  | 1     | e23   | -e42 | e124  | -e4   | -e234 | e3    | -e321 | -e1   | -e43  | E4    | e41   | -e31  | e314 |
| e3   | e3   | e31   | -e23  | 1     | -e43 | -e314 | e234  | -e4   | -e2   | e1    | -e321 | e42   | -e41  | E4    | -e12  | e124 |
| e4   | e4   | e41   | e42   | e43   | 0    | 0     | 0     | 0     | e234  | e314  | e124  | 0     | 0     | 0     | E4    | 0    |
| e41  | e41  | e4    | e124  | -e314 | 0    | 0     | 0     | 0     | -E4   | -e43  | e42   | 0     | 0     | 0     | -e234 | 0    |
| e42  | e42  | -e124 | e4    | e234  | 0    | 0     | 0     | 0     | e43   | -E4   | -e41  | 0     | 0     | 0     | -e314 | 0    |
| e43  | e43  | e314  | -e234 | e4    | 0    | 0     | 0     | 0     | -e42  | e41   | -E4   | 0     | 0     | 0     | -e124 | 0    |
| e23  | e23  | -e321 | -e3   | e2    | e234 | -E4   | -e43  | e42   | -1    | -e12  | e31   | -e4   | -e124 | e314  | e1    | e41  |
| e31  | e31  | e3    | -e321 | -e1   | e314 | e43   | -E4   | -e41  | e12   | -1    | -e23  | e124  | -e4   | -e234 | e2    | e42  |
| e12  | e12  | -e2   | e1    | -e321 | e124 | -e42  | e41   | -E4   | -e31  | e23   | -1    | -e314 | e234  | -e4   | e3    | e43  |
| e234 | e234 | -E4   | -e43  | e42   | 0    | 0     | 0     | 0     | -e4   | -e124 | e314  | 0     | 0     | 0     | e41   | 0    |
| e314 | e314 | e43   | -E4   | -e41  | 0    | 0     | 0     | 0     | e124  | -e4   | -e234 | 0     | 0     | 0     | e42   | 0    |
| e124 | e124 | -e42  | e41   | -E4   | 0    | 0     | 0     | 0     | -e314 | e234  | -e4   | 0     | 0     | 0     | e43   | 0    |
| e321 | e321 | -e23  | -e31  | -e12  | -E4  | e234  | e314  | e124  | e1    | e2    | e3    | -e41  | -e42  | -e43  | -1    | e4   |
| E4   | E4   | -e234 | -e314 | -e124 | 0    | 0     | 0     | 0     | e41   | e42   | e43   | 0     | 0     | 0     | -e4   | 0    |
  "
	);

	assert_eq_ignoring_whitespace!(
		multiplication_table(&unit_blades, Product::AntiGeometric, &rust),
		r"
|      | 1     | e1    | e2    | e3    | e4   | e41   | e42   | e43   | e23  | e31  | e12  | e234  | e314  | e124  | e321 | E4   |
| ---- | ----- | ----- | ----- | ----- | ---- | ----- | ----- | ----- | ---- | ---- | ---- | ----- | ----- | ----- | ---- | ---- |
| 1    | 0     | 0     | 0     | 0     | e321 | e23   | e31   | e12   | 0    | 0    | 0    | e1    | e2    | e3    | 0    | 1    |
| e1   | 0     | 0     | 0     | 0     | -e23 | -e321 | e3    | -e2   | 0    | 0    | 0    | 1     | -e12  | e31   | 0    | e1   |
| e2   | 0     | 0     | 0     | 0     | -e31 | -e3   | -e321 | e1    | 0    | 0    | 0    | e12   | 1     | -e23  | 0    | e2   |
| e3   | 0     | 0     | 0     | 0     | -e12 | e2    | -e1   | -e321 | 0    | 0    | 0    | -e31  | e23   | 1     | 0    | e3   |
| e4   | -e321 | e23   | e31   | e12   | -E4  | e234  | e314  | e124  | -e1  | -e2  | -e3  | -e41  | -e42  | -e43  | 1    | e4   |
| e41  | e23   | -e321 | e3    | -e2   | e234 | -E4   | e43   | -e42  | -1   | e12  | -e31 | -e4   | e124  | -e314 | e1   | e41  |
| e42  | e31   | -e3   | -e321 | e1    | e314 | -e43  | -E4   | e41   | -e12 | -1   | e23  | -e124 | -e4   | e234  | e2   | e42  |
| e43  | e12   | e2    | -e1   | -e321 | e124 | e42   | -e41  | -E4   | e31  | -e23 | -1   | e314  | -e234 | -e4   | e3   | e43  |
| e23  | 0     | 0     | 0     | 0     | e1   | -1    | e12   | -e31  | 0    | 0    | 0    | -e321 | e3    | -e2   | 0    | e23  |
| e31  | 0     | 0     | 0     | 0     | e2   | -e12  | -1    | e23   | 0    | 0    | 0    | -e3   | -e321 | e1    | 0    | e31  |
| e12  | 0     | 0     | 0     | 0     | e3   | e31   | -e23  | -1    | 0    | 0    | 0    | e2    | -e1   | -e321 | 0    | e12  |
| e234 | -e1   | -1    | e12   | -e31  | -e41 | -e4   | e124  | -e314 | e321 | -e3  | e2   | E4    | -e43  | e42   | e23  | e234 |
| e314 | -e2   | -e12  | -1    | e23   | -e42 | -e124 | -e4   | e234  | e3   | e321 | -e1  | e43   | E4    | -e41  | e31  | e314 |
| e124 | -e3   | e31   | -e23  | -1    | -e43 | e314  | -e234 | -e4   | -e2  | e1   | e321 | -e42  | e41   | E4    | e12  | e124 |
| e321 | 0     | 0     | 0     | 0     | -1   | e1    | e2    | e3    | 0    | 0    | 0    | -e23  | -e31  | -e12  | 0    | e321 |
| E4   | 1     | e1    | e2    | e3    | e4   | e41   | e42   | e43   | e23  | e31  | e12  | e234  | e314  | e124  | e321 | E4   |
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
		rust(Expr::wedge(vec![Expr::var(0, "p", &point), Expr::var(1, "q", &point)])),
		"
Line {
    vx: -p.x ^ e4 + q.x ^ e4,
    vy: -p.y ^ e4 + q.y ^ e4,
    vz: -p.z ^ e4 + q.z ^ e4,
    mx: p.y ^ q.z - p.z ^ q.y,
    my: -p.x ^ q.z + p.z ^ q.x,
    mz: p.x ^ q.y - p.y ^ q.x,
}
    "
	);
}

#[test]
fn test_pga2d() {
	let (g, t) = grammars::pga2d();
	let rust = |expr: Expr| expr.simplify(Some(&g)).typify(&t, &g).rust_concise();

	// let unit_blades = t.unit_blades();
	// println!("{}", multiplication_tables(&unit_blades, &rust));

	assert_eq_ignoring_whitespace!(
		t.get("WX").unit().rust_concise(),
		"-_e0 ^ _e2",
		"Ugly output without typify"
	);
	assert_eq_ignoring_whitespace!(rust(t.get("WX").unit()), "WX");

	assert_eq_ignoring_whitespace!(
		Expr::dot(vec![t.get("XY").unit()]).simplify(Some(&g)).rust_concise(),
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
		expr.rust_concise(),
		"_e0 ^ _e1 + _e1 ^ _e0",
		"Hard to read without running typify"
	);
	assert_eq_ignoring_whitespace!(rust(expr), "0");

	let point = t.get("Vec3");
	assert_eq_ignoring_whitespace!(
		rust(Expr::wedge(vec![Expr::var(0, "l", point), Expr::var(1, "r", point)])),
		r"
Line {
    dx: -l.w ^ r.y + l.y ^ r.w,
    dy: l.w ^ r.x - l.x ^ r.w,
    m : l.x ^ r.y - l.y ^ r.x,
}"
		.trim()
	);

	let line = t.get("Line");
	assert_eq_ignoring_whitespace!(
		rust(Expr::antiwedge(vec![Expr::var(0, "l", line), Expr::var(1, "r", line)])),
		r"
Vec3 {
    x: l.dy & r.m - l.m & r.dy,
    y: -l.dx & r.m + l.m & r.dx,
    w: l.dx & r.dy - l.dy & r.dx,
}
"
		.trim()
	);

	assert_eq_ignoring_whitespace!(
		rust(Expr::geometric(vec![
			Expr::var(0, "p", point),
			Expr::var(0, "p", point)
		])),
		r"p.x * p.x + p.y * p.y"
	);

	assert_eq_ignoring_whitespace!(
		rust(Expr::geometric(vec![Expr::var(0, "l", line), Expr::var(0, "l", line)])),
		r"l.m * l.m"
	);
}

#[test]
fn test_pga2d_rcompl() {
	let (g, t) = grammars::pga2d();
	let rust = |expr: Expr| expr.simplify(Some(&g)).typify(&t, &g).rust_concise();
	let point = t.get("Vec3");
	let expr = Expr::unary(Unary::RCompl, Expr::var(0, "l", point));
	dbg!(expr.clone().rust_concise());
	dbg!(expr.clone().simplify(Some(&g)).rust_concise());
	assert_eq_ignoring_whitespace!(
		rust(expr),
		r"
Line {
    dx: l.x.rcompl(),
    dy: l.y.rcompl(),
    m : l.w.rcompl(),
}"
		.trim()
	);
}

#[test]
fn test_generator() {
	let (grammar, types) = generator::grammars::pga3d();
	let settings = gen::Settings::default();
	let gen = gen::Generator {
		grammar,
		types,
		settings,
		ro: RustOptions::rust(),
	};

	let point = gen.types.get_struct("Vec4");
	let code = gen::strct::impl_struct_product(&gen, &("Vec4", &point), &("Vec4", &point), Product::Wedge);
	assert_eq_ignoring_whitespace!(
		code,
		r"
// Vec4.wedge(Vec4) -> Line3
impl Wedge<Vec4> for Vec4 {
    type Output = Line3;
    fn wedge(self, rhs: Vec4) -> Self::Output {
        // Line3 {
        //     vx: WX(self.w.0 * rhs.x.0) + WX(self.x.0 * rhs.w.0),
        //     vy: WY(self.w.0 * rhs.y.0) + WY(self.y.0 * rhs.w.0),
        //     vz: WZ(self.w.0 * rhs.z.0) + WZ(self.z.0 * rhs.w.0),
        //     mx: YZ(self.y.0 * rhs.z.0) + YZ(self.z.0 * rhs.y.0),
        //     my: ZX(self.x.0 * rhs.z.0) + ZX(self.z.0 * rhs.x.0),
        //     mz: XY(self.x.0 * rhs.y.0) + XY(self.y.0 * rhs.x.0),
        // }
        Line3 {
            vx: self.w.wedge(rhs.x) - self.x.wedge(rhs.w),
            vy: self.w.wedge(rhs.y) - self.y.wedge(rhs.w),
            vz: self.w.wedge(rhs.z) - self.z.wedge(rhs.w),
            mx: self.y.wedge(rhs.z) - self.z.wedge(rhs.y),
            my: -self.x.wedge(rhs.z) + self.z.wedge(rhs.x),
            mz: self.x.wedge(rhs.y) - self.y.wedge(rhs.x),
        }
    }
}
        "
	);
}

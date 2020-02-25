use symbolic::*;

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
	let t = pga2d_types();
	let g = Grammar(vec![1, 1, 0]);

	let rust = |op: Op| {
		let op = op.simplify(Some(&g));
		let op = op.typify(&t, &g);
		op.rust()
	};

	let x_type = t.get("X");
	let y_type = t.get("Y");

	let blades = vec![
		t.get("R"),
		t.get("X"),
		t.get("Y"),
		t.get("W"),
		t.get("YW"),
		t.get("WX"),
		t.get("XY"),
		t.get("XYW"),
	];

	assert_eq!(t.get("WX").unit().rust(), "-e0 ^ e2");
	assert_eq!(rust(t.get("WX").unit()), "WX");

	let unit_blades: Vec<Op> = blades.iter().map(|t| t.unit()).collect();

	println!();
	println!("Geometric multiplication table (left side * top row):");
	for a in &unit_blades {
		print!("  ");
		for b in &unit_blades {
			let prod = Op::geometric(vec![a.clone(), b.clone()]);
			print!("{:<10} ", rust(prod));
		}
		println!();
	}

	println!();
	println!("Wedge multiplication table (left side ^ top row):");
	for a in &unit_blades {
		print!("  ");
		for b in &unit_blades {
			let prod = Op::wedge(vec![a.clone(), b.clone()]);
			print!("{:<10} ", rust(prod));
		}
		println!();
	}

	println!();
	println!("Antiwedge multiplication table (right side & bottom row):");
	for a in &unit_blades {
		print!("  ");
		for b in &unit_blades {
			let prod = Op::antiwedge(vec![a.clone(), b.clone()]);
			print!("{:<10} ", rust(prod));
		}
		println!();
	}

	assert_eq!(rust(x_type.unit()), "X");

	assert_eq!(rust(Op::wedge(vec![x_type.unit(), y_type.unit()])), "XY");

	let op = Op::Sum(vec![
		Op::wedge(vec![x_type.unit(), y_type.unit()]),
		Op::wedge(vec![y_type.unit(), x_type.unit()]),
	]);
	assert_eq!(op.rust(), "e0 ^ e1 + e1 ^ e0", "Hard to read without running typify");

	assert_eq!(rust(op), "0");

	let point = t.get("Point");
	assert_eq!(
		rust(Op::wedge(vec![Op::var("l", point), Op::var("r", point)])),
		r"
Line {
    yw: -l.w ^ r.y + l.y ^ r.w,
    wx: -l.w ^ r.x + l.x ^ r.w,
    xy: l.x ^ r.y + -l.y ^ r.x,
}"
		.trim()
	);

	let line = t.get("Line");
	assert_eq!(
		rust(Op::antiwedge(vec![Op::var("l", line), Op::var("r", line)])),
		r"
Point {
    x: l.wx & r.xy + -l.xy & r.wx,
    y: l.xy & r.yw + -l.yw & r.xy,
    w: -l.wx & r.yw + l.yw & r.wx,
}
"
		.trim()
	);
}

use symbolic::*;

fn types() -> Types {
	let mut t = Types::default();
	let x = VecIdx(0);
	let y = VecIdx(1);
	let w = VecIdx(2);
	t.insert("R", Type::scalar());
	t.insert("X", Type::vec(x));
	t.insert("Y", Type::vec(y));
	t.insert("W", Type::vec(w));
	t.insert("YW", Type::blade(&[y, w]));
	t.insert("WX", Type::blade(&[w, x]));
	t.insert("XY", Type::blade(&[x, y]));
	t.insert("XYW", Type::blade(&[x, y, w]));

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
			("XY".to_string(), t.get("XY").clone()),
		]),
	);

	t
}

fn main() {
	let t = types();
	let g = Grammar(vec![1, 1, 0]);
	let rust = |op: Op| {
		let op = op.simplify(Some(&g));
		let op = op.typify(&t, Some(&g));
		op.rust()
	};

	let x_type = t.get("X");
	let y_type = t.get("Y");

	let unit_blades = vec![
		t.get("R"),
		t.get("X"),
		t.get("Y"),
		t.get("W"),
		t.get("YW"),
		t.get("WX"),
		t.get("XY"),
		t.get("XYW"),
	];

	println!();
	println!("Geometric multiplication table (left side * top row):");
	for a in &unit_blades {
		print!("  ");
		for b in &unit_blades {
			let prod = Op::geometric(vec![a.one(), b.one()]);
			print!("{:<10} ", rust(prod));
		}
		println!();
	}

	println!();
	println!("Wedge multiplication table (left side ^ top row):");
	for a in &unit_blades {
		print!("  ");
		for b in &unit_blades {
			let prod = Op::wedge(vec![a.one(), b.one()]);
			print!("{:<10} ", rust(prod));
		}
		println!();
	}

	assert_eq!(rust(x_type.one()), "X");

	assert_eq!(rust(Op::wedge(vec![x_type.one(), y_type.one()])), "XY");

	let op = Op::Sum(vec![
		Op::wedge(vec![x_type.one(), y_type.one()]),
		Op::wedge(vec![y_type.one(), x_type.one()]),
	]);
	assert_eq!(op.rust(), "e0 ^ e1 + e1 ^ e0", "Hard to read without running typify");

	assert_eq!(rust(op), "0");

	let point = t.get("Point");
	let op = Op::wedge(vec![Op::var("l", point), Op::var("r", point)]);
	assert_eq!(
		rust(op),
		r"
Line {
    yw      : -l.w ^ r.y + l.y ^ r.w
    wx      : -l.w ^ r.x + l.x ^ r.w
    XY      : l.x ^ r.y + -l.y ^ r.x
}"
		.trim()
	);
}

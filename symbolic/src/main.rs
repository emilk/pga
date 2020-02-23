use symbolic::*;

fn main() {
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

	let g = Grammar(vec![1, 1, 0]);

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

	// println!();
	// println!("Geometric multiplication table (left side * top row):");
	// for a in &unit_blades {
	// 	print!("  ");
	// 	for b in &unit_blades {
	// 		let prod = Op::geometric(vec![a.one(), b.one()]);
	// 		let prod = prod.simplify(Some(&g));
	// 		print!("{:<10} ", prod.rust(&t, Some(&g)));
	// 	}
	// 	println!();
	// }

	// println!();
	// println!("Wedge multiplication table (left side ^ top row):");
	// for a in &unit_blades {
	// 	print!("  ");
	// 	for b in &unit_blades {
	// 		let prod = Op::wedge(vec![a.one(), b.one()]);
	// 		let prod = prod.simplify(Some(&g));
	// 		print!("{:<10} ", prod.rust(&t, Some(&g)));
	// 	}
	// 	println!();
	// }

	// assert_eq!(x_type.one().rust(&t, Some(&g)), "X");

	// assert_eq!(Op::wedge(vec![x_type.one(), y_type.one()]).rust(&t, Some(&g)), "XY");

	// let op = Op::Sum(vec![
	// 	Op::wedge(vec![x_type.one(), y_type.one()]),
	// 	Op::wedge(vec![y_type.one(), x_type.one()]),
	// ]);
	// assert_eq!(op.rust(&t, Some(&g)), "XY + -XY");

	// assert_eq!(op.simplify(Some(&g)).rust(&t, Some(&g)), "0");

	let point = t.get("Point");
	let op = Op::wedge(vec![Op::var("l", point), Op::var("r", point)]);
	// dbg!(&op);
	// dbg!(&op.clone().simplify(Some(&g)));
	assert_eq!(op.simplify(Some(&g)).rust(&t, Some(&g)), "Line");
}

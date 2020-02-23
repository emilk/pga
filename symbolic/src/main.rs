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
	println!();
	println!("Geometric multiplication table (left side * top row):");
	for a in &unit_blades {
		print!("  ");
		for b in &unit_blades {
			let prod = Op::Prod(vec![a.one(), b.one()]);
			let prod = prod.simplify(Some(&g));
			print!("{:<10} ", prod.rust(&t));
		}
		println!();
	}

	assert_eq!(x_type.one().rust(&t), "X");

	assert_eq!(Op::Prod(vec![x_type.one(), y_type.one()]).rust(&t), "XY");

	let op = Op::Sum(vec![
		Op::Prod(vec![x_type.one(), y_type.one()]),
		Op::Prod(vec![y_type.one(), x_type.one()]),
	]);
	assert_eq!(op.rust(&t), "XY + -XY");

	assert_eq!(op.simplify(Some(&g)).rust(&t), "0");
}

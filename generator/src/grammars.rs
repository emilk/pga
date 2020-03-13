use crate::*;

pub fn pga2d() -> (Grammar, Types) {
	let g = Grammar(vec![1, 1, 0]);
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

	(g, t)
}

/// Using the notation of Eric Lengyel.
/// See http://terathon.com/blog/projective-geometric-algebra-done-right/
pub fn pga3d_lengyel() -> (Grammar, Types) {
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

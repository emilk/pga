use crate::*;

pub fn pga2d() -> (Grammar, Types) {
	let g = Grammar(vec![1, 1, 0]);
	let mut t = Types::default();
	let x = VecIdx(0);
	let y = VecIdx(1);
	let w = VecIdx(2);
	t.insert_blade("R", SBlade::scalar());
	t.insert_blade("X", SBlade::vec(x));
	t.insert_blade("Y", SBlade::vec(y));
	t.insert_blade("W", SBlade::vec(w));
	t.insert_blade("YW", SBlade::from_unsorted(&[y, w]));
	t.insert_blade("WX", SBlade::from_unsorted(&[w, x]));
	t.insert_blade("XY", SBlade::from_unsorted(&[x, y]));
	t.insert_blade("XYW", SBlade::from_unsorted(&[x, y, w]));

	t.insert_struct("Point", &[("x", "X"), ("y", "Y"), ("w", "W")]);

	t.insert_struct("Line", &[("yw", "YW"), ("wx", "WX"), ("xy", "XY")]);

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
	t.insert_blade("s", SBlade::scalar());

	t.insert_blade("e1", SBlade::vec(e1));
	t.insert_blade("e2", SBlade::vec(e2));
	t.insert_blade("e3", SBlade::vec(e3));
	t.insert_blade("e4", SBlade::vec(e4));
	t.insert_blade("e41", SBlade::from_unsorted(&[e4, e1]));
	t.insert_blade("e42", SBlade::from_unsorted(&[e4, e2]));
	t.insert_blade("e43", SBlade::from_unsorted(&[e4, e3]));
	t.insert_blade("e23", SBlade::from_unsorted(&[e2, e3]));
	t.insert_blade("e31", SBlade::from_unsorted(&[e3, e1]));
	t.insert_blade("e12", SBlade::from_unsorted(&[e1, e2]));
	t.insert_blade("e234", SBlade::from_unsorted(&[e2, e3, e4]));
	t.insert_blade("e314", SBlade::from_unsorted(&[e3, e1, e4]));
	t.insert_blade("e124", SBlade::from_unsorted(&[e1, e2, e4]));
	t.insert_blade("e321", SBlade::from_unsorted(&[e3, e2, e1]));
	t.insert_blade("E4", SBlade::from_unsorted(&[e1, e2, e3, e4]));

	t.insert_struct("Point", &[("x", "e1"), ("y", "e2"), ("z", "e3"), ("w", "e4")]);

	t.insert_struct(
		"Line",
		&[
			("vx", "e41"),
			("vy", "e42"),
			("vz", "e43"),
			("mx", "e23"),
			("my", "e31"),
			("mz", "e12"),
		],
	);

	t.insert_struct(
		"Plane",
		&[("nx", "e234"), ("ny", "e314"), ("nz", "e124"), ("d", "e321")],
	);

	(g, t)
}

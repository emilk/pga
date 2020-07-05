use crate::*;

pub fn pga2d() -> (Grammar, Types) {
	let g = Grammar(vec![1, 1, 0]);
	let mut t = Types::default();
	let x = VecIdx(0);
	let y = VecIdx(1);
	let w = VecIdx(2);
	t.insert_blade("S", SBlade::scalar());
	t.insert_blade("X", SBlade::vec(x));
	t.insert_blade("Y", SBlade::vec(y));
	t.insert_blade("W", SBlade::vec(w));
	t.insert_blade("YW", SBlade::from_unsorted(&[y, w]));
	t.insert_blade("WX", SBlade::from_unsorted(&[w, x]));
	t.insert_blade("XY", SBlade::from_unsorted(&[x, y]));
	t.insert_blade("XYW", SBlade::from_unsorted(&[x, y, w]));

	// TODO: Point { x: X, y: Y, z: Z, w: 1)
	t.insert_struct("Vec2", &[("x", "X"), ("y", "Y")]);
	t.insert_struct("Vec3", &[("x", "X"), ("y", "Y"), ("w", "W")]);

	// TODO: verify if these are Plücker coordinates
	t.insert_struct(
		"Line",
		&[
			// dir :
			("dx", "YW"),
			("dy", "WX"),
			// offset / moment:
			("m", "XY"),
		],
	);

	// TODO: is this correct?
	t.insert_struct("Translator", &[("s", "S"), ("yw", "YW"), ("wx", "WX")]);
	t.insert_struct("Rotor", &[("s", "S"), ("xy", "XY")]);
	t.insert_struct("Motor", &[("s", "S"), ("yw", "YW"), ("wx", "WX"), ("xy", "XY")]);
	// TODO: Is this a Motor? Or a Transform?
	// t.insert_struct(
	// 	"Transform",
	// 	&[("s", "S"), ("yw", "YW"), ("wx", "WX"), ("xy", "XY"), ("xyw", "XYW")],
	// );

	(g, t)
}

/// Using the Eric Lengyel system, but with X,Y,Z,W instead of e1,e2,e3,e4
/// See http://terathon.com/blog/projective-geometric-algebra-done-right/
pub fn pga3d() -> (Grammar, Types) {
	let g = Grammar(vec![1, 1, 1, 0]);
	let mut t = Types::default();
	let x = VecIdx(0);
	let y = VecIdx(1);
	let z = VecIdx(2);
	let w = VecIdx(3);

	t.insert_blade("S", SBlade::scalar());
	t.insert_blade("X", SBlade::vec(x));
	t.insert_blade("Y", SBlade::vec(y));
	t.insert_blade("Z", SBlade::vec(z));
	t.insert_blade("W", SBlade::vec(w));
	t.insert_blade("WX", SBlade::from_unsorted(&[w, x]));
	t.insert_blade("WY", SBlade::from_unsorted(&[w, y]));
	t.insert_blade("WZ", SBlade::from_unsorted(&[w, z]));
	t.insert_blade("YZ", SBlade::from_unsorted(&[y, z]));
	t.insert_blade("ZX", SBlade::from_unsorted(&[z, x]));
	t.insert_blade("XY", SBlade::from_unsorted(&[x, y]));
	t.insert_blade("YZW", SBlade::from_unsorted(&[y, z, w]));
	t.insert_blade("ZXW", SBlade::from_unsorted(&[z, x, w]));
	t.insert_blade("XYW", SBlade::from_unsorted(&[x, y, w]));
	t.insert_blade("ZYX", SBlade::from_unsorted(&[z, y, x]));
	t.insert_blade("XYZW", SBlade::from_unsorted(&[x, y, z, w]));
	// -----------------------------------
	// 2D

	if false {
		// TODO: Point2 { x: X, y: Y, w: 1)
		t.insert_struct("Vec2", &[("x", "X"), ("y", "Y")]);

		// Plücker coordinates
		t.insert_struct(
			"Line2",
			&[
				// Dir:
				("dx", "WX"),
				("dy", "WY"),
				// offset / moment:
				("m", "XY"),
			],
		);

		t.insert_struct("Translator2", &[("s", "S"), ("wy", "WY"), ("wx", "WX")]);
		t.insert_struct("Rotor2", &[("s", "S"), ("xy", "XY")]);
		t.insert_struct("Motor2", &[("s", "S"), ("wy", "WY"), ("wx", "WX"), ("xy", "XY")]);
	}

	// -----------------------------------
	// 3D

	// TODO: Point3 { x: X, y: Y, z: Z, w: 1)
	t.insert_struct("Vec3", &[("x", "X"), ("y", "Y"), ("z", "Z")]);
	t.insert_struct("Vec4", &[("x", "X"), ("y", "Y"), ("z", "Z"), ("w", "W")]);

	// The result of Dir ^ Dir, which is numerically identical to a cross product but with different units (blades)
	// t.insert_struct("Moment3", &[("yz", "YZ"), ("zx", "ZX"), ("xy", "XY")]);

	// Plücker coordinates
	t.insert_struct(
		"Line3",
		&[
			// dir:
			("vx", "WX"),
			("vy", "WY"),
			("vz", "WZ"),
			// moment:
			("mx", "YZ"),
			("my", "ZX"),
			("mz", "XY"),
		],
	);

	t.insert_struct("Plane", &[("nx", "YZW"), ("ny", "ZXW"), ("nz", "XYW"), ("d", "ZYX")]);

	t.insert_struct(
		"Translator3",
		&[("x", "YZ"), ("y", "ZX"), ("z", "XY"), ("xyzw", "XYZW")],
	);
	// Quaternion
	t.insert_struct("Rotor3", &[("x", "WX"), ("y", "WY"), ("z", "WZ"), ("w", "XYZW")]);

	// Dual quaternion
	t.insert_struct(
		"Motor3",
		&[
			("rx", "WX"),
			("ry", "WY"),
			("rz", "WZ"),
			("rw", "XYZW"),
			("ux", "YZW"),
			("uy", "ZXW"),
			("uz", "XYW"),
			("uw", "S"),
		],
	);

	// -----------------------------------

	(g, t)
}

/// Using the naming convention of Eric Lengyel.
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

	// Plücker coordinates
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

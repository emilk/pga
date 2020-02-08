use pga::pga2d::*;

fn main() {
	// let l1 = Line::from_euclidean(1.0, 0.0, 0.0);
	let l1 = Line {
		e0: E0(0.0),
		e1: E1(1.0),
		e2: E2(0.0),
	};
	// let l2 = Line::from_euclidean(0.5f64.sqrt(), -0.5f64.sqrt(), 0.0);
	let l2 = Line {
		e0: E0(0.0),
		e1: E1(-0.5f64.sqrt()),
		e2: E2(0.5f64.sqrt()),
	};
	let t = l1 * l2; // 90 deg rotate
	let p = Point::from_euclidean(3.0, 2.0);
	dbg!(t);
	dbg!(p);
	dbg!(t.sandwich(p));
	// dbg!(t * p * t.reverse());
}

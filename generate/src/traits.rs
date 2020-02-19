// ----------------------------------------------------------------------------
// Geometric Algebra definition helpers:

/// Special zero type for completeness, and better error messages.
/// If you get this in an error message, it is because you multiplied
/// two dimensions that always results in zero.

pub struct Zero {}

/// x.dual() = !x
/// Scalar <-> pseudoscalar, etc
/// in 2d, points and lines are duals to each other.
/// In 3d, points and planes are dual to each other.
/*
Consider a simple 2D PHGA with

e0=W, e1=X, e2=Y
X^2=1, Y^2=1, W^2=0

A point is {x: X,  y: Y,  w: W}
A line  is {x: YW, y: WX, w: XY}
And these are dual to each other (same x,y,z)

Now you can see the dual as a form of transpose.
*/
pub trait Dual {
	type Output;
	fn dual(self) -> Self::Output;
}

/// e1.reverse()   = e1
/// e12.reverse()  = e21  = -e12
/// e012.reverse() = e210 = -e012
/// Used for sandwich product t * x * rev(t)
/// a * b = (b * a).reverse()
pub trait Reverse {
	fn reverse(self) -> Self;
}

/// x.squared() = x * x
/// Note that all values square to a scalar.
pub trait Square {
	fn square(self) -> f64;
}

/// The dot product, a.k.a. the inner product.
/// a.dot(b) = a | b
/// The commutative (a | b = b | a) part of the geometric product.
/// Signifies a metric of how alike two values are.
/// Orthogonal values always dot to zero.
pub trait Dot<Rhs> {
	type Output;
	fn dot(self, e: Rhs) -> Output;
}

/// The edge product, a.k.a. the outer product.
/// x.wedge(y) = x ^ y
/// The anti-commutative (a ^ b = - b ^ a) part of the geometric product.
/// Signifies how unlike two things are. x^x = 0
/// In dual PGA this is the MEET operator, used to intersect two things (e.g. a line ^ plane = point).
pub trait Wedge<Rhs> {
	type Output;
	fn wedge(self, e: Rhs) -> Output;
}

/// The geometric product, a.k.a. normal multiplication.
/// a.geometric(b) = a.dot(b) + a.wedge(b)
/// a * b = (a | b) + (a ^ b)
pub trait Geometric<Rhs> {
	type Output;
	fn geometric(self, e: Rhs) -> Output;
}

/// The anti-geometric product, i.e. the dual version of the geometric product.
/// a.antigeometric(b) = !(!a * !b)
/// Introduced in http://terathon.com/blog/projective-geometric-algebra-done-right/
pub trait Antigeometric<Rhs> {
	type Output;
	fn antigeometric(self, e: Rhs) -> Output;
}

/// The regressive product, a.k.a. the anti-wedge product, a.k.a. exterior antiproduct.
/// This is the dual version of the regressive product.
/// x.regressive(y) = x & y = !(!x ^ !y)
/// In dual PHA this is the JOIN operator, used to join two things, e.g. point & line = plane.
pub trait Regressive<Rhs> {
	type Output;
	fn regressive(self, e: Rhs) -> Output;
}

/// The sandwich transform.
/// Used to transform a value somehow, e.g. rotor.sandwich(point) -> rotated point
/// t.sandwich(e) = t · e · t.reverse()
pub trait Sandwich<Rhs> {
	fn sandwich(self, e: Rhs) -> Rhs;
}

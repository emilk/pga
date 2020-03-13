// ----------------------------------------------------------------------------
// Geometric Algebra definition helpers:

/// Special zero type for completeness, and better error messages.
/// If you get this in an error message, it is because you multiplied
/// two dimensions that always results in zero.

pub struct Zero {}

// ----------------------------------------------------------------------------

/// A value multiplied by its complement is the pseudoscalar.
pub trait Complement {
	type Output;

	/// Left compliment.
	/// self.lcompl() * self == pseudo-scalar
	fn lcompl(self) -> Self::Output;

	/// Right compliment.
	/// self * self.rcompl() == pseudo-scalar
	/// e0 * e0.rcompl() = e0 * e12 = e012
	/// e1.rcompl() = e20 = -e02
	fn rcompl(self) -> Self::Output;
}

/// Reverse the order of the vector indices:
/// e1.reverse()   = e1
/// e12.reverse()  = e21  = -e12
/// e012.reverse() = e210 = -e012
/// Used for sandwich products
pub trait Reverse {
	fn reverse(self) -> Self;
}

pub trait AntiReverse {
	/// self.lcompl().reverse().rcompl()
	fn anti_reverse(self) -> Self;
}

/// x.squared() = x * x
/// Note that all values square to a scalar.
pub trait Square {
	fn square(self) -> f64;
}

/// The geometric product, a.k.a. normal multiplication.
/// a.geometric(b) = a.dot(b) + a.wedge(b)
pub trait Geometric<Rhs> {
	type Output;
	fn geometric(self, e: Rhs) -> Output;
}

/// The anti-geometric product, i.e. the dual version of the geometric product.
/// self.antigeometric(other) = self.lcompl().geometric(other.lcompl()).rcompl()
/// Introduced in http://terathon.com/blog/projective-geometric-algebra-done-right/
pub trait AntiGeometric<Rhs> {
	type Output;
	fn anti_geometric(self, e: Rhs) -> Output;
}

/// The dot product, a.k.a. the inner product.
/// The commutative part of the geometric product.
/// Signifies a metric of how alike two values are.
/// Orthogonal values always dot to zero.
pub trait Dot<Rhs> {
	type Output;
	fn dot(self, e: Rhs) -> Output;
}

/// The wedge product, a.k.a. the outer product.
/// x.wedge(y) = x ^ y
/// The anti-commutative (a ^ b = - b ^ a) part of the geometric product.
/// Signifies how unlike two things are. x^x = 0
/// In dual PGA this is the MEET operator, used to intersect two things (e.g. a line ^ plane = point).
pub trait Wedge<Rhs> {
	type Output;
	fn wedge(self, e: Rhs) -> Output;
}

/// The regressive product, a.k.a. the anti-wedge product, a.k.a. exterior antiproduct.
/// This is the dual version of the regressive product.
/// x.regressive(y) = x & y = !(!x ^ !y)
/// In dual PHA this is the JOIN operator, used to join two things, e.g. point & line = plane.
pub trait AntiWedge<Rhs> {
	type Output;
	fn anti_wedge(self, e: Rhs) -> Output;
}

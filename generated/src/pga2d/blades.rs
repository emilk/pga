//! # Blade types
//! The blades that make up this geometric algebra.
//!
//! ## Unary operations
//! | Op \ Blade       | 1   | X  | Y  | W  | YW  | WX  | XY  | XYW  |
//! | ---------------- | --- | -- | -- | -- | --- | --- | --- | ---- |
//! | Right complement | XYW | YW | WX | XY | X   | Y   | W   | 1    |
//! | Left complement  | XYW | YW | WX | XY | X   | Y   | W   | 1    |
//! | Reverse          | 1   | X  | Y  | W  | -YW | -WX | -XY | -XYW |
//! | Anti-reverse     | -1  | -X | -Y | -W | YW  | WX  | XY  | XYW  |
//!
//!
//! ## Multiplication tables
//! ### Geometric multiplication table
//!
//! |     | 1   | X   | Y   | W   | YW  | WX  | XY  | XYW |
//! | --- | --- | --- | --- | --- | --- | --- | --- | --- |
//! | 1   | 1   | X   | Y   | W   | YW  | WX  | XY  | XYW |
//! | X   | X   | 1   | XY  | -WX | XYW | -W  | Y   | YW  |
//! | Y   | Y   | -XY | 1   | YW  | W   | XYW | -X  | WX  |
//! | W   | W   | WX  | -YW | 0   | 0   | 0   | XYW | 0   |
//! | YW  | YW  | XYW | -W  | 0   | 0   | 0   | WX  | 0   |
//! | WX  | WX  | W   | XYW | 0   | 0   | 0   | -YW | 0   |
//! | XY  | XY  | -Y  | X   | XYW | -WX | YW  | -1  | -W  |
//! | XYW | XYW | YW  | WX  | 0   | 0   | 0   | -W  | 0   |
//!
//!
//! ### AntiGeometric multiplication table
//!
//! |     | 1   | X   | Y   | W   | YW  | WX  | XY  | XYW |
//! | --- | --- | --- | --- | --- | --- | --- | --- | --- |
//! | 1   | 1   | X   | Y   | W   | YW  | WX  | XY  | XYW |
//! | X   | X   | 1   | XY  | -WX | XYW | -W  | Y   | YW  |
//! | Y   | Y   | -XY | 1   | YW  | W   | XYW | -X  | WX  |
//! | W   | W   | WX  | -YW | 0   | 0   | 0   | XYW | 0   |
//! | YW  | YW  | XYW | -W  | 0   | 0   | 0   | WX  | 0   |
//! | WX  | WX  | W   | XYW | 0   | 0   | 0   | -YW | 0   |
//! | XY  | XY  | -Y  | X   | XYW | -WX | YW  | -1  | -W  |
//! | XYW | XYW | YW  | WX  | 0   | 0   | 0   | -W  | 0   |
//!
//!
//! ### Dot multiplication table
//!
//! |     | 1   | X   | Y   | W   | YW  | WX  | XY  | XYW |
//! | --- | --- | --- | --- | --- | --- | --- | --- | --- |
//! | 1   | 1   | X   | Y   | W   | YW  | WX  | XY  | XYW |
//! | X   | X   | 1   | XY  | -WX | XYW | -W  | Y   | YW  |
//! | Y   | Y   | -XY | 1   | YW  | W   | XYW | -X  | WX  |
//! | W   | W   | WX  | -YW | 0   | 0   | 0   | XYW | 0   |
//! | YW  | YW  | XYW | -W  | 0   | 0   | 0   | WX  | 0   |
//! | WX  | WX  | W   | XYW | 0   | 0   | 0   | -YW | 0   |
//! | XY  | XY  | -Y  | X   | XYW | -WX | YW  | -1  | -W  |
//! | XYW | XYW | YW  | WX  | 0   | 0   | 0   | -W  | 0   |
//!
//!
//! ### Wedge multiplication table
//!
//! |     | 1   | X   | Y   | W   | YW  | WX  | XY  | XYW |
//! | --- | --- | --- | --- | --- | --- | --- | --- | --- |
//! | 1   | 1   | X   | Y   | W   | YW  | WX  | XY  | XYW |
//! | X   | X   | 1   | XY  | -WX | XYW | -W  | Y   | YW  |
//! | Y   | Y   | -XY | 1   | YW  | W   | XYW | -X  | WX  |
//! | W   | W   | WX  | -YW | 0   | 0   | 0   | XYW | 0   |
//! | YW  | YW  | XYW | -W  | 0   | 0   | 0   | WX  | 0   |
//! | WX  | WX  | W   | XYW | 0   | 0   | 0   | -YW | 0   |
//! | XY  | XY  | -Y  | X   | XYW | -WX | YW  | -1  | -W  |
//! | XYW | XYW | YW  | WX  | 0   | 0   | 0   | -W  | 0   |
//!
//!
//! ### AntiWedge multiplication table
//!
//! |     | 1   | X   | Y   | W   | YW  | WX  | XY  | XYW |
//! | --- | --- | --- | --- | --- | --- | --- | --- | --- |
//! | 1   | 1   | X   | Y   | W   | YW  | WX  | XY  | XYW |
//! | X   | X   | 1   | XY  | -WX | XYW | -W  | Y   | YW  |
//! | Y   | Y   | -XY | 1   | YW  | W   | XYW | -X  | WX  |
//! | W   | W   | WX  | -YW | 0   | 0   | 0   | XYW | 0   |
//! | YW  | YW  | XYW | -W  | 0   | 0   | 0   | WX  | 0   |
//! | WX  | WX  | W   | XYW | 0   | 0   | 0   | -YW | 0   |
//! | XY  | XY  | -Y  | X   | XYW | -WX | YW  | -1  | -W  |
//! | XYW | XYW | YW  | WX  | 0   | 0   | 0   | -W  | 0   |

use derive_more::{Add, Mul, Neg, Sub};

use super::*;

/// The scalar type (real numbers).
/// Squares to 1.
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, Neg, Add, Sub, Mul)]
pub struct S(pub f64);

/// Squares to 1.
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, Neg, Add, Sub)]
pub struct X(pub f64);

/// Squares to 1.
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, Neg, Add, Sub)]
pub struct Y(pub f64);

/// Squares to 0.
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, Neg, Add, Sub)]
pub struct W(pub f64);

/// Squares to 0.
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, Neg, Add, Sub)]
pub struct YW(pub f64);

/// Squares to 0.
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, Neg, Add, Sub)]
pub struct WX(pub f64);

/// Squares to -1.
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, Neg, Add, Sub)]
pub struct XY(pub f64);

/// The pseudo-scalar.
/// Squares to 0.
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, Neg, Add, Sub)]
pub struct XYW(pub f64);

// ---------------------------------------------------------------------
// impl RCompl for blades:

impl RCompl for S {
	type Output = XYW;
	fn rcompl(self) -> Self::Output {
		XYW(self.0)
	}
}

impl RCompl for X {
	type Output = YW;
	fn rcompl(self) -> Self::Output {
		YW(self.0)
	}
}

impl RCompl for Y {
	type Output = WX;
	fn rcompl(self) -> Self::Output {
		WX(self.0)
	}
}

impl RCompl for W {
	type Output = XY;
	fn rcompl(self) -> Self::Output {
		XY(self.0)
	}
}

impl RCompl for YW {
	type Output = X;
	fn rcompl(self) -> Self::Output {
		X(self.0)
	}
}

impl RCompl for WX {
	type Output = Y;
	fn rcompl(self) -> Self::Output {
		Y(self.0)
	}
}

impl RCompl for XY {
	type Output = W;
	fn rcompl(self) -> Self::Output {
		W(self.0)
	}
}

impl RCompl for XYW {
	type Output = S;
	fn rcompl(self) -> Self::Output {
		S(self.0)
	}
}

// ---------------------------------------------------------------------
// impl LCompl for blades:

impl LCompl for S {
	type Output = XYW;
	fn lcompl(self) -> Self::Output {
		XYW(self.0)
	}
}

impl LCompl for X {
	type Output = YW;
	fn lcompl(self) -> Self::Output {
		YW(self.0)
	}
}

impl LCompl for Y {
	type Output = WX;
	fn lcompl(self) -> Self::Output {
		WX(self.0)
	}
}

impl LCompl for W {
	type Output = XY;
	fn lcompl(self) -> Self::Output {
		XY(self.0)
	}
}

impl LCompl for YW {
	type Output = X;
	fn lcompl(self) -> Self::Output {
		X(self.0)
	}
}

impl LCompl for WX {
	type Output = Y;
	fn lcompl(self) -> Self::Output {
		Y(self.0)
	}
}

impl LCompl for XY {
	type Output = W;
	fn lcompl(self) -> Self::Output {
		W(self.0)
	}
}

impl LCompl for XYW {
	type Output = S;
	fn lcompl(self) -> Self::Output {
		S(self.0)
	}
}

// ---------------------------------------------------------------------
// impl Reverse for blades:

impl Reverse for S {
	fn rev(self) -> Self {
		self
	}
}

impl Reverse for X {
	fn rev(self) -> Self {
		self
	}
}

impl Reverse for Y {
	fn rev(self) -> Self {
		self
	}
}

impl Reverse for W {
	fn rev(self) -> Self {
		self
	}
}

impl Reverse for YW {
	fn rev(self) -> Self {
		-self
	}
}

impl Reverse for WX {
	fn rev(self) -> Self {
		-self
	}
}

impl Reverse for XY {
	fn rev(self) -> Self {
		-self
	}
}

impl Reverse for XYW {
	fn rev(self) -> Self {
		-self
	}
}

// ---------------------------------------------------------------------
// impl AntiReverse for blades:

impl AntiReverse for S {
	fn arev(self) -> Self {
		-self
	}
}

impl AntiReverse for X {
	fn arev(self) -> Self {
		-self
	}
}

impl AntiReverse for Y {
	fn arev(self) -> Self {
		-self
	}
}

impl AntiReverse for W {
	fn arev(self) -> Self {
		-self
	}
}

impl AntiReverse for YW {
	fn arev(self) -> Self {
		self
	}
}

impl AntiReverse for WX {
	fn arev(self) -> Self {
		self
	}
}

impl AntiReverse for XY {
	fn arev(self) -> Self {
		self
	}
}

impl AntiReverse for XYW {
	fn arev(self) -> Self {
		self
	}
}

// ---------------------------------------------------------------------
// impl Geometric for blades:

impl Geometric<S> for S {
	type Output = S;
	fn geometric(self, rhs: S) -> Self::Output {
		S(self.0 * rhs.0)
	}
}

impl Geometric<X> for S {
	type Output = X;
	fn geometric(self, rhs: X) -> Self::Output {
		X(self.0 * rhs.0)
	}
}

impl Geometric<Y> for S {
	type Output = Y;
	fn geometric(self, rhs: Y) -> Self::Output {
		Y(self.0 * rhs.0)
	}
}

impl Geometric<W> for S {
	type Output = W;
	fn geometric(self, rhs: W) -> Self::Output {
		W(self.0 * rhs.0)
	}
}

impl Geometric<YW> for S {
	type Output = YW;
	fn geometric(self, rhs: YW) -> Self::Output {
		YW(self.0 * rhs.0)
	}
}

impl Geometric<WX> for S {
	type Output = WX;
	fn geometric(self, rhs: WX) -> Self::Output {
		WX(self.0 * rhs.0)
	}
}

impl Geometric<XY> for S {
	type Output = XY;
	fn geometric(self, rhs: XY) -> Self::Output {
		XY(self.0 * rhs.0)
	}
}

impl Geometric<XYW> for S {
	type Output = XYW;
	fn geometric(self, rhs: XYW) -> Self::Output {
		XYW(self.0 * rhs.0)
	}
}

impl Geometric<S> for X {
	type Output = X;
	fn geometric(self, rhs: S) -> Self::Output {
		X(self.0 * rhs.0)
	}
}

impl Geometric<X> for X {
	type Output = S;
	fn geometric(self, rhs: X) -> Self::Output {
		S(self.0 * rhs.0)
	}
}

impl Geometric<Y> for X {
	type Output = XY;
	fn geometric(self, rhs: Y) -> Self::Output {
		XY(self.0 * rhs.0)
	}
}

impl Geometric<W> for X {
	type Output = WX;
	fn geometric(self, rhs: W) -> Self::Output {
		WX(-self.0 * rhs.0)
	}
}

impl Geometric<YW> for X {
	type Output = XYW;
	fn geometric(self, rhs: YW) -> Self::Output {
		XYW(self.0 * rhs.0)
	}
}

impl Geometric<WX> for X {
	type Output = W;
	fn geometric(self, rhs: WX) -> Self::Output {
		W(-self.0 * rhs.0)
	}
}

impl Geometric<XY> for X {
	type Output = Y;
	fn geometric(self, rhs: XY) -> Self::Output {
		Y(self.0 * rhs.0)
	}
}

impl Geometric<XYW> for X {
	type Output = YW;
	fn geometric(self, rhs: XYW) -> Self::Output {
		YW(self.0 * rhs.0)
	}
}

impl Geometric<S> for Y {
	type Output = Y;
	fn geometric(self, rhs: S) -> Self::Output {
		Y(self.0 * rhs.0)
	}
}

impl Geometric<X> for Y {
	type Output = XY;
	fn geometric(self, rhs: X) -> Self::Output {
		XY(-self.0 * rhs.0)
	}
}

impl Geometric<Y> for Y {
	type Output = S;
	fn geometric(self, rhs: Y) -> Self::Output {
		S(self.0 * rhs.0)
	}
}

impl Geometric<W> for Y {
	type Output = YW;
	fn geometric(self, rhs: W) -> Self::Output {
		YW(self.0 * rhs.0)
	}
}

impl Geometric<YW> for Y {
	type Output = W;
	fn geometric(self, rhs: YW) -> Self::Output {
		W(self.0 * rhs.0)
	}
}

impl Geometric<WX> for Y {
	type Output = XYW;
	fn geometric(self, rhs: WX) -> Self::Output {
		XYW(self.0 * rhs.0)
	}
}

impl Geometric<XY> for Y {
	type Output = X;
	fn geometric(self, rhs: XY) -> Self::Output {
		X(-self.0 * rhs.0)
	}
}

impl Geometric<XYW> for Y {
	type Output = WX;
	fn geometric(self, rhs: XYW) -> Self::Output {
		WX(self.0 * rhs.0)
	}
}

impl Geometric<S> for W {
	type Output = W;
	fn geometric(self, rhs: S) -> Self::Output {
		W(self.0 * rhs.0)
	}
}

impl Geometric<X> for W {
	type Output = WX;
	fn geometric(self, rhs: X) -> Self::Output {
		WX(self.0 * rhs.0)
	}
}

impl Geometric<Y> for W {
	type Output = YW;
	fn geometric(self, rhs: Y) -> Self::Output {
		YW(-self.0 * rhs.0)
	}
}

impl Geometric<W> for W {
	type Output = Zero;
	fn geometric(self, _rhs: W) -> Self::Output {
		Zero {}
	}
}

impl Geometric<YW> for W {
	type Output = Zero;
	fn geometric(self, _rhs: YW) -> Self::Output {
		Zero {}
	}
}

impl Geometric<WX> for W {
	type Output = Zero;
	fn geometric(self, _rhs: WX) -> Self::Output {
		Zero {}
	}
}

impl Geometric<XY> for W {
	type Output = XYW;
	fn geometric(self, rhs: XY) -> Self::Output {
		XYW(self.0 * rhs.0)
	}
}

impl Geometric<XYW> for W {
	type Output = Zero;
	fn geometric(self, _rhs: XYW) -> Self::Output {
		Zero {}
	}
}

impl Geometric<S> for YW {
	type Output = YW;
	fn geometric(self, rhs: S) -> Self::Output {
		YW(self.0 * rhs.0)
	}
}

impl Geometric<X> for YW {
	type Output = XYW;
	fn geometric(self, rhs: X) -> Self::Output {
		XYW(self.0 * rhs.0)
	}
}

impl Geometric<Y> for YW {
	type Output = W;
	fn geometric(self, rhs: Y) -> Self::Output {
		W(-self.0 * rhs.0)
	}
}

impl Geometric<W> for YW {
	type Output = Zero;
	fn geometric(self, _rhs: W) -> Self::Output {
		Zero {}
	}
}

impl Geometric<YW> for YW {
	type Output = Zero;
	fn geometric(self, _rhs: YW) -> Self::Output {
		Zero {}
	}
}

impl Geometric<WX> for YW {
	type Output = Zero;
	fn geometric(self, _rhs: WX) -> Self::Output {
		Zero {}
	}
}

impl Geometric<XY> for YW {
	type Output = WX;
	fn geometric(self, rhs: XY) -> Self::Output {
		WX(self.0 * rhs.0)
	}
}

impl Geometric<XYW> for YW {
	type Output = Zero;
	fn geometric(self, _rhs: XYW) -> Self::Output {
		Zero {}
	}
}

impl Geometric<S> for WX {
	type Output = WX;
	fn geometric(self, rhs: S) -> Self::Output {
		WX(self.0 * rhs.0)
	}
}

impl Geometric<X> for WX {
	type Output = W;
	fn geometric(self, rhs: X) -> Self::Output {
		W(self.0 * rhs.0)
	}
}

impl Geometric<Y> for WX {
	type Output = XYW;
	fn geometric(self, rhs: Y) -> Self::Output {
		XYW(self.0 * rhs.0)
	}
}

impl Geometric<W> for WX {
	type Output = Zero;
	fn geometric(self, _rhs: W) -> Self::Output {
		Zero {}
	}
}

impl Geometric<YW> for WX {
	type Output = Zero;
	fn geometric(self, _rhs: YW) -> Self::Output {
		Zero {}
	}
}

impl Geometric<WX> for WX {
	type Output = Zero;
	fn geometric(self, _rhs: WX) -> Self::Output {
		Zero {}
	}
}

impl Geometric<XY> for WX {
	type Output = YW;
	fn geometric(self, rhs: XY) -> Self::Output {
		YW(-self.0 * rhs.0)
	}
}

impl Geometric<XYW> for WX {
	type Output = Zero;
	fn geometric(self, _rhs: XYW) -> Self::Output {
		Zero {}
	}
}

impl Geometric<S> for XY {
	type Output = XY;
	fn geometric(self, rhs: S) -> Self::Output {
		XY(self.0 * rhs.0)
	}
}

impl Geometric<X> for XY {
	type Output = Y;
	fn geometric(self, rhs: X) -> Self::Output {
		Y(-self.0 * rhs.0)
	}
}

impl Geometric<Y> for XY {
	type Output = X;
	fn geometric(self, rhs: Y) -> Self::Output {
		X(self.0 * rhs.0)
	}
}

impl Geometric<W> for XY {
	type Output = XYW;
	fn geometric(self, rhs: W) -> Self::Output {
		XYW(self.0 * rhs.0)
	}
}

impl Geometric<YW> for XY {
	type Output = WX;
	fn geometric(self, rhs: YW) -> Self::Output {
		WX(-self.0 * rhs.0)
	}
}

impl Geometric<WX> for XY {
	type Output = YW;
	fn geometric(self, rhs: WX) -> Self::Output {
		YW(self.0 * rhs.0)
	}
}

impl Geometric<XY> for XY {
	type Output = S;
	fn geometric(self, rhs: XY) -> Self::Output {
		S(-self.0 * rhs.0)
	}
}

impl Geometric<XYW> for XY {
	type Output = W;
	fn geometric(self, rhs: XYW) -> Self::Output {
		W(-self.0 * rhs.0)
	}
}

impl Geometric<S> for XYW {
	type Output = XYW;
	fn geometric(self, rhs: S) -> Self::Output {
		XYW(self.0 * rhs.0)
	}
}

impl Geometric<X> for XYW {
	type Output = YW;
	fn geometric(self, rhs: X) -> Self::Output {
		YW(self.0 * rhs.0)
	}
}

impl Geometric<Y> for XYW {
	type Output = WX;
	fn geometric(self, rhs: Y) -> Self::Output {
		WX(self.0 * rhs.0)
	}
}

impl Geometric<W> for XYW {
	type Output = Zero;
	fn geometric(self, _rhs: W) -> Self::Output {
		Zero {}
	}
}

impl Geometric<YW> for XYW {
	type Output = Zero;
	fn geometric(self, _rhs: YW) -> Self::Output {
		Zero {}
	}
}

impl Geometric<WX> for XYW {
	type Output = Zero;
	fn geometric(self, _rhs: WX) -> Self::Output {
		Zero {}
	}
}

impl Geometric<XY> for XYW {
	type Output = W;
	fn geometric(self, rhs: XY) -> Self::Output {
		W(-self.0 * rhs.0)
	}
}

impl Geometric<XYW> for XYW {
	type Output = Zero;
	fn geometric(self, _rhs: XYW) -> Self::Output {
		Zero {}
	}
}

// ---------------------------------------------------------------------
// impl AntiGeometric for blades:

impl AntiGeometric<S> for S {
	type Output = Zero;
	fn anti_geometric(self, _rhs: S) -> Self::Output {
		Zero {}
	}
}

impl AntiGeometric<X> for S {
	type Output = Zero;
	fn anti_geometric(self, _rhs: X) -> Self::Output {
		Zero {}
	}
}

impl AntiGeometric<Y> for S {
	type Output = Zero;
	fn anti_geometric(self, _rhs: Y) -> Self::Output {
		Zero {}
	}
}

impl AntiGeometric<W> for S {
	type Output = XY;
	fn anti_geometric(self, rhs: W) -> Self::Output {
		XY(-self.0 * rhs.0)
	}
}

impl AntiGeometric<YW> for S {
	type Output = X;
	fn anti_geometric(self, rhs: YW) -> Self::Output {
		X(self.0 * rhs.0)
	}
}

impl AntiGeometric<WX> for S {
	type Output = Y;
	fn anti_geometric(self, rhs: WX) -> Self::Output {
		Y(self.0 * rhs.0)
	}
}

impl AntiGeometric<XY> for S {
	type Output = Zero;
	fn anti_geometric(self, _rhs: XY) -> Self::Output {
		Zero {}
	}
}

impl AntiGeometric<XYW> for S {
	type Output = S;
	fn anti_geometric(self, rhs: XYW) -> Self::Output {
		S(self.0 * rhs.0)
	}
}

impl AntiGeometric<S> for X {
	type Output = Zero;
	fn anti_geometric(self, _rhs: S) -> Self::Output {
		Zero {}
	}
}

impl AntiGeometric<X> for X {
	type Output = Zero;
	fn anti_geometric(self, _rhs: X) -> Self::Output {
		Zero {}
	}
}

impl AntiGeometric<Y> for X {
	type Output = Zero;
	fn anti_geometric(self, _rhs: Y) -> Self::Output {
		Zero {}
	}
}

impl AntiGeometric<W> for X {
	type Output = Y;
	fn anti_geometric(self, rhs: W) -> Self::Output {
		Y(self.0 * rhs.0)
	}
}

impl AntiGeometric<YW> for X {
	type Output = S;
	fn anti_geometric(self, rhs: YW) -> Self::Output {
		S(self.0 * rhs.0)
	}
}

impl AntiGeometric<WX> for X {
	type Output = XY;
	fn anti_geometric(self, rhs: WX) -> Self::Output {
		XY(-self.0 * rhs.0)
	}
}

impl AntiGeometric<XY> for X {
	type Output = Zero;
	fn anti_geometric(self, _rhs: XY) -> Self::Output {
		Zero {}
	}
}

impl AntiGeometric<XYW> for X {
	type Output = X;
	fn anti_geometric(self, rhs: XYW) -> Self::Output {
		X(self.0 * rhs.0)
	}
}

impl AntiGeometric<S> for Y {
	type Output = Zero;
	fn anti_geometric(self, _rhs: S) -> Self::Output {
		Zero {}
	}
}

impl AntiGeometric<X> for Y {
	type Output = Zero;
	fn anti_geometric(self, _rhs: X) -> Self::Output {
		Zero {}
	}
}

impl AntiGeometric<Y> for Y {
	type Output = Zero;
	fn anti_geometric(self, _rhs: Y) -> Self::Output {
		Zero {}
	}
}

impl AntiGeometric<W> for Y {
	type Output = X;
	fn anti_geometric(self, rhs: W) -> Self::Output {
		X(-self.0 * rhs.0)
	}
}

impl AntiGeometric<YW> for Y {
	type Output = XY;
	fn anti_geometric(self, rhs: YW) -> Self::Output {
		XY(self.0 * rhs.0)
	}
}

impl AntiGeometric<WX> for Y {
	type Output = S;
	fn anti_geometric(self, rhs: WX) -> Self::Output {
		S(self.0 * rhs.0)
	}
}

impl AntiGeometric<XY> for Y {
	type Output = Zero;
	fn anti_geometric(self, _rhs: XY) -> Self::Output {
		Zero {}
	}
}

impl AntiGeometric<XYW> for Y {
	type Output = Y;
	fn anti_geometric(self, rhs: XYW) -> Self::Output {
		Y(self.0 * rhs.0)
	}
}

impl AntiGeometric<S> for W {
	type Output = XY;
	fn anti_geometric(self, rhs: S) -> Self::Output {
		XY(-self.0 * rhs.0)
	}
}

impl AntiGeometric<X> for W {
	type Output = Y;
	fn anti_geometric(self, rhs: X) -> Self::Output {
		Y(-self.0 * rhs.0)
	}
}

impl AntiGeometric<Y> for W {
	type Output = X;
	fn anti_geometric(self, rhs: Y) -> Self::Output {
		X(self.0 * rhs.0)
	}
}

impl AntiGeometric<W> for W {
	type Output = XYW;
	fn anti_geometric(self, rhs: W) -> Self::Output {
		XYW(-self.0 * rhs.0)
	}
}

impl AntiGeometric<YW> for W {
	type Output = WX;
	fn anti_geometric(self, rhs: YW) -> Self::Output {
		WX(-self.0 * rhs.0)
	}
}

impl AntiGeometric<WX> for W {
	type Output = YW;
	fn anti_geometric(self, rhs: WX) -> Self::Output {
		YW(self.0 * rhs.0)
	}
}

impl AntiGeometric<XY> for W {
	type Output = S;
	fn anti_geometric(self, rhs: XY) -> Self::Output {
		S(self.0 * rhs.0)
	}
}

impl AntiGeometric<XYW> for W {
	type Output = W;
	fn anti_geometric(self, rhs: XYW) -> Self::Output {
		W(self.0 * rhs.0)
	}
}

impl AntiGeometric<S> for YW {
	type Output = X;
	fn anti_geometric(self, rhs: S) -> Self::Output {
		X(self.0 * rhs.0)
	}
}

impl AntiGeometric<X> for YW {
	type Output = S;
	fn anti_geometric(self, rhs: X) -> Self::Output {
		S(self.0 * rhs.0)
	}
}

impl AntiGeometric<Y> for YW {
	type Output = XY;
	fn anti_geometric(self, rhs: Y) -> Self::Output {
		XY(-self.0 * rhs.0)
	}
}

impl AntiGeometric<W> for YW {
	type Output = WX;
	fn anti_geometric(self, rhs: W) -> Self::Output {
		WX(self.0 * rhs.0)
	}
}

impl AntiGeometric<YW> for YW {
	type Output = XYW;
	fn anti_geometric(self, rhs: YW) -> Self::Output {
		XYW(self.0 * rhs.0)
	}
}

impl AntiGeometric<WX> for YW {
	type Output = W;
	fn anti_geometric(self, rhs: WX) -> Self::Output {
		W(self.0 * rhs.0)
	}
}

impl AntiGeometric<XY> for YW {
	type Output = Y;
	fn anti_geometric(self, rhs: XY) -> Self::Output {
		Y(-self.0 * rhs.0)
	}
}

impl AntiGeometric<XYW> for YW {
	type Output = YW;
	fn anti_geometric(self, rhs: XYW) -> Self::Output {
		YW(self.0 * rhs.0)
	}
}

impl AntiGeometric<S> for WX {
	type Output = Y;
	fn anti_geometric(self, rhs: S) -> Self::Output {
		Y(self.0 * rhs.0)
	}
}

impl AntiGeometric<X> for WX {
	type Output = XY;
	fn anti_geometric(self, rhs: X) -> Self::Output {
		XY(self.0 * rhs.0)
	}
}

impl AntiGeometric<Y> for WX {
	type Output = S;
	fn anti_geometric(self, rhs: Y) -> Self::Output {
		S(self.0 * rhs.0)
	}
}

impl AntiGeometric<W> for WX {
	type Output = YW;
	fn anti_geometric(self, rhs: W) -> Self::Output {
		YW(-self.0 * rhs.0)
	}
}

impl AntiGeometric<YW> for WX {
	type Output = W;
	fn anti_geometric(self, rhs: YW) -> Self::Output {
		W(-self.0 * rhs.0)
	}
}

impl AntiGeometric<WX> for WX {
	type Output = XYW;
	fn anti_geometric(self, rhs: WX) -> Self::Output {
		XYW(self.0 * rhs.0)
	}
}

impl AntiGeometric<XY> for WX {
	type Output = X;
	fn anti_geometric(self, rhs: XY) -> Self::Output {
		X(self.0 * rhs.0)
	}
}

impl AntiGeometric<XYW> for WX {
	type Output = WX;
	fn anti_geometric(self, rhs: XYW) -> Self::Output {
		WX(self.0 * rhs.0)
	}
}

impl AntiGeometric<S> for XY {
	type Output = Zero;
	fn anti_geometric(self, _rhs: S) -> Self::Output {
		Zero {}
	}
}

impl AntiGeometric<X> for XY {
	type Output = Zero;
	fn anti_geometric(self, _rhs: X) -> Self::Output {
		Zero {}
	}
}

impl AntiGeometric<Y> for XY {
	type Output = Zero;
	fn anti_geometric(self, _rhs: Y) -> Self::Output {
		Zero {}
	}
}

impl AntiGeometric<W> for XY {
	type Output = S;
	fn anti_geometric(self, rhs: W) -> Self::Output {
		S(self.0 * rhs.0)
	}
}

impl AntiGeometric<YW> for XY {
	type Output = Y;
	fn anti_geometric(self, rhs: YW) -> Self::Output {
		Y(self.0 * rhs.0)
	}
}

impl AntiGeometric<WX> for XY {
	type Output = X;
	fn anti_geometric(self, rhs: WX) -> Self::Output {
		X(-self.0 * rhs.0)
	}
}

impl AntiGeometric<XY> for XY {
	type Output = Zero;
	fn anti_geometric(self, _rhs: XY) -> Self::Output {
		Zero {}
	}
}

impl AntiGeometric<XYW> for XY {
	type Output = XY;
	fn anti_geometric(self, rhs: XYW) -> Self::Output {
		XY(self.0 * rhs.0)
	}
}

impl AntiGeometric<S> for XYW {
	type Output = S;
	fn anti_geometric(self, rhs: S) -> Self::Output {
		S(self.0 * rhs.0)
	}
}

impl AntiGeometric<X> for XYW {
	type Output = X;
	fn anti_geometric(self, rhs: X) -> Self::Output {
		X(self.0 * rhs.0)
	}
}

impl AntiGeometric<Y> for XYW {
	type Output = Y;
	fn anti_geometric(self, rhs: Y) -> Self::Output {
		Y(self.0 * rhs.0)
	}
}

impl AntiGeometric<W> for XYW {
	type Output = W;
	fn anti_geometric(self, rhs: W) -> Self::Output {
		W(self.0 * rhs.0)
	}
}

impl AntiGeometric<YW> for XYW {
	type Output = YW;
	fn anti_geometric(self, rhs: YW) -> Self::Output {
		YW(self.0 * rhs.0)
	}
}

impl AntiGeometric<WX> for XYW {
	type Output = WX;
	fn anti_geometric(self, rhs: WX) -> Self::Output {
		WX(self.0 * rhs.0)
	}
}

impl AntiGeometric<XY> for XYW {
	type Output = XY;
	fn anti_geometric(self, rhs: XY) -> Self::Output {
		XY(self.0 * rhs.0)
	}
}

impl AntiGeometric<XYW> for XYW {
	type Output = XYW;
	fn anti_geometric(self, rhs: XYW) -> Self::Output {
		XYW(self.0 * rhs.0)
	}
}

// ---------------------------------------------------------------------
// impl Dot for blades:

impl Dot<S> for S {
	type Output = S;
	fn dot(self, rhs: S) -> Self::Output {
		S(self.0 * rhs.0)
	}
}

impl Dot<X> for S {
	type Output = X;
	fn dot(self, rhs: X) -> Self::Output {
		X(self.0 * rhs.0)
	}
}

impl Dot<Y> for S {
	type Output = Y;
	fn dot(self, rhs: Y) -> Self::Output {
		Y(self.0 * rhs.0)
	}
}

impl Dot<W> for S {
	type Output = W;
	fn dot(self, rhs: W) -> Self::Output {
		W(self.0 * rhs.0)
	}
}

impl Dot<YW> for S {
	type Output = YW;
	fn dot(self, rhs: YW) -> Self::Output {
		YW(self.0 * rhs.0)
	}
}

impl Dot<WX> for S {
	type Output = WX;
	fn dot(self, rhs: WX) -> Self::Output {
		WX(self.0 * rhs.0)
	}
}

impl Dot<XY> for S {
	type Output = XY;
	fn dot(self, rhs: XY) -> Self::Output {
		XY(self.0 * rhs.0)
	}
}

impl Dot<XYW> for S {
	type Output = XYW;
	fn dot(self, rhs: XYW) -> Self::Output {
		XYW(self.0 * rhs.0)
	}
}

impl Dot<S> for X {
	type Output = X;
	fn dot(self, rhs: S) -> Self::Output {
		X(self.0 * rhs.0)
	}
}

impl Dot<X> for X {
	type Output = S;
	fn dot(self, rhs: X) -> Self::Output {
		S(self.0 * rhs.0)
	}
}

impl Dot<Y> for X {
	type Output = Zero;
	fn dot(self, _rhs: Y) -> Self::Output {
		Zero {}
	}
}

impl Dot<W> for X {
	type Output = Zero;
	fn dot(self, _rhs: W) -> Self::Output {
		Zero {}
	}
}

impl Dot<YW> for X {
	type Output = Zero;
	fn dot(self, _rhs: YW) -> Self::Output {
		Zero {}
	}
}

impl Dot<WX> for X {
	type Output = W;
	fn dot(self, rhs: WX) -> Self::Output {
		W(-self.0 * rhs.0)
	}
}

impl Dot<XY> for X {
	type Output = Y;
	fn dot(self, rhs: XY) -> Self::Output {
		Y(self.0 * rhs.0)
	}
}

impl Dot<XYW> for X {
	type Output = YW;
	fn dot(self, rhs: XYW) -> Self::Output {
		YW(self.0 * rhs.0)
	}
}

impl Dot<S> for Y {
	type Output = Y;
	fn dot(self, rhs: S) -> Self::Output {
		Y(self.0 * rhs.0)
	}
}

impl Dot<X> for Y {
	type Output = Zero;
	fn dot(self, _rhs: X) -> Self::Output {
		Zero {}
	}
}

impl Dot<Y> for Y {
	type Output = S;
	fn dot(self, rhs: Y) -> Self::Output {
		S(self.0 * rhs.0)
	}
}

impl Dot<W> for Y {
	type Output = Zero;
	fn dot(self, _rhs: W) -> Self::Output {
		Zero {}
	}
}

impl Dot<YW> for Y {
	type Output = W;
	fn dot(self, rhs: YW) -> Self::Output {
		W(self.0 * rhs.0)
	}
}

impl Dot<WX> for Y {
	type Output = Zero;
	fn dot(self, _rhs: WX) -> Self::Output {
		Zero {}
	}
}

impl Dot<XY> for Y {
	type Output = X;
	fn dot(self, rhs: XY) -> Self::Output {
		X(-self.0 * rhs.0)
	}
}

impl Dot<XYW> for Y {
	type Output = WX;
	fn dot(self, rhs: XYW) -> Self::Output {
		WX(self.0 * rhs.0)
	}
}

impl Dot<S> for W {
	type Output = W;
	fn dot(self, rhs: S) -> Self::Output {
		W(self.0 * rhs.0)
	}
}

impl Dot<X> for W {
	type Output = Zero;
	fn dot(self, _rhs: X) -> Self::Output {
		Zero {}
	}
}

impl Dot<Y> for W {
	type Output = Zero;
	fn dot(self, _rhs: Y) -> Self::Output {
		Zero {}
	}
}

impl Dot<W> for W {
	type Output = Zero;
	fn dot(self, _rhs: W) -> Self::Output {
		Zero {}
	}
}

impl Dot<YW> for W {
	type Output = Zero;
	fn dot(self, _rhs: YW) -> Self::Output {
		Zero {}
	}
}

impl Dot<WX> for W {
	type Output = Zero;
	fn dot(self, _rhs: WX) -> Self::Output {
		Zero {}
	}
}

impl Dot<XY> for W {
	type Output = Zero;
	fn dot(self, _rhs: XY) -> Self::Output {
		Zero {}
	}
}

impl Dot<XYW> for W {
	type Output = Zero;
	fn dot(self, _rhs: XYW) -> Self::Output {
		Zero {}
	}
}

impl Dot<S> for YW {
	type Output = YW;
	fn dot(self, rhs: S) -> Self::Output {
		YW(self.0 * rhs.0)
	}
}

impl Dot<X> for YW {
	type Output = Zero;
	fn dot(self, _rhs: X) -> Self::Output {
		Zero {}
	}
}

impl Dot<Y> for YW {
	type Output = W;
	fn dot(self, rhs: Y) -> Self::Output {
		W(-self.0 * rhs.0)
	}
}

impl Dot<W> for YW {
	type Output = Zero;
	fn dot(self, _rhs: W) -> Self::Output {
		Zero {}
	}
}

impl Dot<YW> for YW {
	type Output = Zero;
	fn dot(self, _rhs: YW) -> Self::Output {
		Zero {}
	}
}

impl Dot<WX> for YW {
	type Output = Zero;
	fn dot(self, _rhs: WX) -> Self::Output {
		Zero {}
	}
}

impl Dot<XY> for YW {
	type Output = Zero;
	fn dot(self, _rhs: XY) -> Self::Output {
		Zero {}
	}
}

impl Dot<XYW> for YW {
	type Output = Zero;
	fn dot(self, _rhs: XYW) -> Self::Output {
		Zero {}
	}
}

impl Dot<S> for WX {
	type Output = WX;
	fn dot(self, rhs: S) -> Self::Output {
		WX(self.0 * rhs.0)
	}
}

impl Dot<X> for WX {
	type Output = W;
	fn dot(self, rhs: X) -> Self::Output {
		W(self.0 * rhs.0)
	}
}

impl Dot<Y> for WX {
	type Output = Zero;
	fn dot(self, _rhs: Y) -> Self::Output {
		Zero {}
	}
}

impl Dot<W> for WX {
	type Output = Zero;
	fn dot(self, _rhs: W) -> Self::Output {
		Zero {}
	}
}

impl Dot<YW> for WX {
	type Output = Zero;
	fn dot(self, _rhs: YW) -> Self::Output {
		Zero {}
	}
}

impl Dot<WX> for WX {
	type Output = Zero;
	fn dot(self, _rhs: WX) -> Self::Output {
		Zero {}
	}
}

impl Dot<XY> for WX {
	type Output = Zero;
	fn dot(self, _rhs: XY) -> Self::Output {
		Zero {}
	}
}

impl Dot<XYW> for WX {
	type Output = Zero;
	fn dot(self, _rhs: XYW) -> Self::Output {
		Zero {}
	}
}

impl Dot<S> for XY {
	type Output = XY;
	fn dot(self, rhs: S) -> Self::Output {
		XY(self.0 * rhs.0)
	}
}

impl Dot<X> for XY {
	type Output = Y;
	fn dot(self, rhs: X) -> Self::Output {
		Y(-self.0 * rhs.0)
	}
}

impl Dot<Y> for XY {
	type Output = X;
	fn dot(self, rhs: Y) -> Self::Output {
		X(self.0 * rhs.0)
	}
}

impl Dot<W> for XY {
	type Output = Zero;
	fn dot(self, _rhs: W) -> Self::Output {
		Zero {}
	}
}

impl Dot<YW> for XY {
	type Output = Zero;
	fn dot(self, _rhs: YW) -> Self::Output {
		Zero {}
	}
}

impl Dot<WX> for XY {
	type Output = Zero;
	fn dot(self, _rhs: WX) -> Self::Output {
		Zero {}
	}
}

impl Dot<XY> for XY {
	type Output = S;
	fn dot(self, rhs: XY) -> Self::Output {
		S(-self.0 * rhs.0)
	}
}

impl Dot<XYW> for XY {
	type Output = W;
	fn dot(self, rhs: XYW) -> Self::Output {
		W(-self.0 * rhs.0)
	}
}

impl Dot<S> for XYW {
	type Output = XYW;
	fn dot(self, rhs: S) -> Self::Output {
		XYW(self.0 * rhs.0)
	}
}

impl Dot<X> for XYW {
	type Output = YW;
	fn dot(self, rhs: X) -> Self::Output {
		YW(self.0 * rhs.0)
	}
}

impl Dot<Y> for XYW {
	type Output = WX;
	fn dot(self, rhs: Y) -> Self::Output {
		WX(self.0 * rhs.0)
	}
}

impl Dot<W> for XYW {
	type Output = Zero;
	fn dot(self, _rhs: W) -> Self::Output {
		Zero {}
	}
}

impl Dot<YW> for XYW {
	type Output = Zero;
	fn dot(self, _rhs: YW) -> Self::Output {
		Zero {}
	}
}

impl Dot<WX> for XYW {
	type Output = Zero;
	fn dot(self, _rhs: WX) -> Self::Output {
		Zero {}
	}
}

impl Dot<XY> for XYW {
	type Output = W;
	fn dot(self, rhs: XY) -> Self::Output {
		W(-self.0 * rhs.0)
	}
}

impl Dot<XYW> for XYW {
	type Output = Zero;
	fn dot(self, _rhs: XYW) -> Self::Output {
		Zero {}
	}
}

// ---------------------------------------------------------------------
// impl Wedge for blades:

impl Wedge<S> for S {
	type Output = S;
	fn wedge(self, rhs: S) -> Self::Output {
		S(self.0 * rhs.0)
	}
}

impl Wedge<X> for S {
	type Output = X;
	fn wedge(self, rhs: X) -> Self::Output {
		X(self.0 * rhs.0)
	}
}

impl Wedge<Y> for S {
	type Output = Y;
	fn wedge(self, rhs: Y) -> Self::Output {
		Y(self.0 * rhs.0)
	}
}

impl Wedge<W> for S {
	type Output = W;
	fn wedge(self, rhs: W) -> Self::Output {
		W(self.0 * rhs.0)
	}
}

impl Wedge<YW> for S {
	type Output = YW;
	fn wedge(self, rhs: YW) -> Self::Output {
		YW(self.0 * rhs.0)
	}
}

impl Wedge<WX> for S {
	type Output = WX;
	fn wedge(self, rhs: WX) -> Self::Output {
		WX(self.0 * rhs.0)
	}
}

impl Wedge<XY> for S {
	type Output = XY;
	fn wedge(self, rhs: XY) -> Self::Output {
		XY(self.0 * rhs.0)
	}
}

impl Wedge<XYW> for S {
	type Output = XYW;
	fn wedge(self, rhs: XYW) -> Self::Output {
		XYW(self.0 * rhs.0)
	}
}

impl Wedge<S> for X {
	type Output = X;
	fn wedge(self, rhs: S) -> Self::Output {
		X(self.0 * rhs.0)
	}
}

impl Wedge<X> for X {
	type Output = Zero;
	fn wedge(self, _rhs: X) -> Self::Output {
		Zero {}
	}
}

impl Wedge<Y> for X {
	type Output = XY;
	fn wedge(self, rhs: Y) -> Self::Output {
		XY(self.0 * rhs.0)
	}
}

impl Wedge<W> for X {
	type Output = WX;
	fn wedge(self, rhs: W) -> Self::Output {
		WX(-self.0 * rhs.0)
	}
}

impl Wedge<YW> for X {
	type Output = XYW;
	fn wedge(self, rhs: YW) -> Self::Output {
		XYW(self.0 * rhs.0)
	}
}

impl Wedge<WX> for X {
	type Output = Zero;
	fn wedge(self, _rhs: WX) -> Self::Output {
		Zero {}
	}
}

impl Wedge<XY> for X {
	type Output = Zero;
	fn wedge(self, _rhs: XY) -> Self::Output {
		Zero {}
	}
}

impl Wedge<XYW> for X {
	type Output = Zero;
	fn wedge(self, _rhs: XYW) -> Self::Output {
		Zero {}
	}
}

impl Wedge<S> for Y {
	type Output = Y;
	fn wedge(self, rhs: S) -> Self::Output {
		Y(self.0 * rhs.0)
	}
}

impl Wedge<X> for Y {
	type Output = XY;
	fn wedge(self, rhs: X) -> Self::Output {
		XY(-self.0 * rhs.0)
	}
}

impl Wedge<Y> for Y {
	type Output = Zero;
	fn wedge(self, _rhs: Y) -> Self::Output {
		Zero {}
	}
}

impl Wedge<W> for Y {
	type Output = YW;
	fn wedge(self, rhs: W) -> Self::Output {
		YW(self.0 * rhs.0)
	}
}

impl Wedge<YW> for Y {
	type Output = Zero;
	fn wedge(self, _rhs: YW) -> Self::Output {
		Zero {}
	}
}

impl Wedge<WX> for Y {
	type Output = XYW;
	fn wedge(self, rhs: WX) -> Self::Output {
		XYW(self.0 * rhs.0)
	}
}

impl Wedge<XY> for Y {
	type Output = Zero;
	fn wedge(self, _rhs: XY) -> Self::Output {
		Zero {}
	}
}

impl Wedge<XYW> for Y {
	type Output = Zero;
	fn wedge(self, _rhs: XYW) -> Self::Output {
		Zero {}
	}
}

impl Wedge<S> for W {
	type Output = W;
	fn wedge(self, rhs: S) -> Self::Output {
		W(self.0 * rhs.0)
	}
}

impl Wedge<X> for W {
	type Output = WX;
	fn wedge(self, rhs: X) -> Self::Output {
		WX(self.0 * rhs.0)
	}
}

impl Wedge<Y> for W {
	type Output = YW;
	fn wedge(self, rhs: Y) -> Self::Output {
		YW(-self.0 * rhs.0)
	}
}

impl Wedge<W> for W {
	type Output = Zero;
	fn wedge(self, _rhs: W) -> Self::Output {
		Zero {}
	}
}

impl Wedge<YW> for W {
	type Output = Zero;
	fn wedge(self, _rhs: YW) -> Self::Output {
		Zero {}
	}
}

impl Wedge<WX> for W {
	type Output = Zero;
	fn wedge(self, _rhs: WX) -> Self::Output {
		Zero {}
	}
}

impl Wedge<XY> for W {
	type Output = XYW;
	fn wedge(self, rhs: XY) -> Self::Output {
		XYW(self.0 * rhs.0)
	}
}

impl Wedge<XYW> for W {
	type Output = Zero;
	fn wedge(self, _rhs: XYW) -> Self::Output {
		Zero {}
	}
}

impl Wedge<S> for YW {
	type Output = YW;
	fn wedge(self, rhs: S) -> Self::Output {
		YW(self.0 * rhs.0)
	}
}

impl Wedge<X> for YW {
	type Output = XYW;
	fn wedge(self, rhs: X) -> Self::Output {
		XYW(self.0 * rhs.0)
	}
}

impl Wedge<Y> for YW {
	type Output = Zero;
	fn wedge(self, _rhs: Y) -> Self::Output {
		Zero {}
	}
}

impl Wedge<W> for YW {
	type Output = Zero;
	fn wedge(self, _rhs: W) -> Self::Output {
		Zero {}
	}
}

impl Wedge<YW> for YW {
	type Output = Zero;
	fn wedge(self, _rhs: YW) -> Self::Output {
		Zero {}
	}
}

impl Wedge<WX> for YW {
	type Output = Zero;
	fn wedge(self, _rhs: WX) -> Self::Output {
		Zero {}
	}
}

impl Wedge<XY> for YW {
	type Output = Zero;
	fn wedge(self, _rhs: XY) -> Self::Output {
		Zero {}
	}
}

impl Wedge<XYW> for YW {
	type Output = Zero;
	fn wedge(self, _rhs: XYW) -> Self::Output {
		Zero {}
	}
}

impl Wedge<S> for WX {
	type Output = WX;
	fn wedge(self, rhs: S) -> Self::Output {
		WX(self.0 * rhs.0)
	}
}

impl Wedge<X> for WX {
	type Output = Zero;
	fn wedge(self, _rhs: X) -> Self::Output {
		Zero {}
	}
}

impl Wedge<Y> for WX {
	type Output = XYW;
	fn wedge(self, rhs: Y) -> Self::Output {
		XYW(self.0 * rhs.0)
	}
}

impl Wedge<W> for WX {
	type Output = Zero;
	fn wedge(self, _rhs: W) -> Self::Output {
		Zero {}
	}
}

impl Wedge<YW> for WX {
	type Output = Zero;
	fn wedge(self, _rhs: YW) -> Self::Output {
		Zero {}
	}
}

impl Wedge<WX> for WX {
	type Output = Zero;
	fn wedge(self, _rhs: WX) -> Self::Output {
		Zero {}
	}
}

impl Wedge<XY> for WX {
	type Output = Zero;
	fn wedge(self, _rhs: XY) -> Self::Output {
		Zero {}
	}
}

impl Wedge<XYW> for WX {
	type Output = Zero;
	fn wedge(self, _rhs: XYW) -> Self::Output {
		Zero {}
	}
}

impl Wedge<S> for XY {
	type Output = XY;
	fn wedge(self, rhs: S) -> Self::Output {
		XY(self.0 * rhs.0)
	}
}

impl Wedge<X> for XY {
	type Output = Zero;
	fn wedge(self, _rhs: X) -> Self::Output {
		Zero {}
	}
}

impl Wedge<Y> for XY {
	type Output = Zero;
	fn wedge(self, _rhs: Y) -> Self::Output {
		Zero {}
	}
}

impl Wedge<W> for XY {
	type Output = XYW;
	fn wedge(self, rhs: W) -> Self::Output {
		XYW(self.0 * rhs.0)
	}
}

impl Wedge<YW> for XY {
	type Output = Zero;
	fn wedge(self, _rhs: YW) -> Self::Output {
		Zero {}
	}
}

impl Wedge<WX> for XY {
	type Output = Zero;
	fn wedge(self, _rhs: WX) -> Self::Output {
		Zero {}
	}
}

impl Wedge<XY> for XY {
	type Output = Zero;
	fn wedge(self, _rhs: XY) -> Self::Output {
		Zero {}
	}
}

impl Wedge<XYW> for XY {
	type Output = Zero;
	fn wedge(self, _rhs: XYW) -> Self::Output {
		Zero {}
	}
}

impl Wedge<S> for XYW {
	type Output = XYW;
	fn wedge(self, rhs: S) -> Self::Output {
		XYW(self.0 * rhs.0)
	}
}

impl Wedge<X> for XYW {
	type Output = Zero;
	fn wedge(self, _rhs: X) -> Self::Output {
		Zero {}
	}
}

impl Wedge<Y> for XYW {
	type Output = Zero;
	fn wedge(self, _rhs: Y) -> Self::Output {
		Zero {}
	}
}

impl Wedge<W> for XYW {
	type Output = Zero;
	fn wedge(self, _rhs: W) -> Self::Output {
		Zero {}
	}
}

impl Wedge<YW> for XYW {
	type Output = Zero;
	fn wedge(self, _rhs: YW) -> Self::Output {
		Zero {}
	}
}

impl Wedge<WX> for XYW {
	type Output = Zero;
	fn wedge(self, _rhs: WX) -> Self::Output {
		Zero {}
	}
}

impl Wedge<XY> for XYW {
	type Output = Zero;
	fn wedge(self, _rhs: XY) -> Self::Output {
		Zero {}
	}
}

impl Wedge<XYW> for XYW {
	type Output = Zero;
	fn wedge(self, _rhs: XYW) -> Self::Output {
		Zero {}
	}
}

// ---------------------------------------------------------------------
// impl AntiWedge for blades:

impl AntiWedge<S> for S {
	type Output = Zero;
	fn anti_wedge(self, _rhs: S) -> Self::Output {
		Zero {}
	}
}

impl AntiWedge<X> for S {
	type Output = Zero;
	fn anti_wedge(self, _rhs: X) -> Self::Output {
		Zero {}
	}
}

impl AntiWedge<Y> for S {
	type Output = Zero;
	fn anti_wedge(self, _rhs: Y) -> Self::Output {
		Zero {}
	}
}

impl AntiWedge<W> for S {
	type Output = Zero;
	fn anti_wedge(self, _rhs: W) -> Self::Output {
		Zero {}
	}
}

impl AntiWedge<YW> for S {
	type Output = Zero;
	fn anti_wedge(self, _rhs: YW) -> Self::Output {
		Zero {}
	}
}

impl AntiWedge<WX> for S {
	type Output = Zero;
	fn anti_wedge(self, _rhs: WX) -> Self::Output {
		Zero {}
	}
}

impl AntiWedge<XY> for S {
	type Output = Zero;
	fn anti_wedge(self, _rhs: XY) -> Self::Output {
		Zero {}
	}
}

impl AntiWedge<XYW> for S {
	type Output = S;
	fn anti_wedge(self, rhs: XYW) -> Self::Output {
		S(self.0 * rhs.0)
	}
}

impl AntiWedge<S> for X {
	type Output = Zero;
	fn anti_wedge(self, _rhs: S) -> Self::Output {
		Zero {}
	}
}

impl AntiWedge<X> for X {
	type Output = Zero;
	fn anti_wedge(self, _rhs: X) -> Self::Output {
		Zero {}
	}
}

impl AntiWedge<Y> for X {
	type Output = Zero;
	fn anti_wedge(self, _rhs: Y) -> Self::Output {
		Zero {}
	}
}

impl AntiWedge<W> for X {
	type Output = Zero;
	fn anti_wedge(self, _rhs: W) -> Self::Output {
		Zero {}
	}
}

impl AntiWedge<YW> for X {
	type Output = S;
	fn anti_wedge(self, rhs: YW) -> Self::Output {
		S(self.0 * rhs.0)
	}
}

impl AntiWedge<WX> for X {
	type Output = Zero;
	fn anti_wedge(self, _rhs: WX) -> Self::Output {
		Zero {}
	}
}

impl AntiWedge<XY> for X {
	type Output = Zero;
	fn anti_wedge(self, _rhs: XY) -> Self::Output {
		Zero {}
	}
}

impl AntiWedge<XYW> for X {
	type Output = X;
	fn anti_wedge(self, rhs: XYW) -> Self::Output {
		X(self.0 * rhs.0)
	}
}

impl AntiWedge<S> for Y {
	type Output = Zero;
	fn anti_wedge(self, _rhs: S) -> Self::Output {
		Zero {}
	}
}

impl AntiWedge<X> for Y {
	type Output = Zero;
	fn anti_wedge(self, _rhs: X) -> Self::Output {
		Zero {}
	}
}

impl AntiWedge<Y> for Y {
	type Output = Zero;
	fn anti_wedge(self, _rhs: Y) -> Self::Output {
		Zero {}
	}
}

impl AntiWedge<W> for Y {
	type Output = Zero;
	fn anti_wedge(self, _rhs: W) -> Self::Output {
		Zero {}
	}
}

impl AntiWedge<YW> for Y {
	type Output = Zero;
	fn anti_wedge(self, _rhs: YW) -> Self::Output {
		Zero {}
	}
}

impl AntiWedge<WX> for Y {
	type Output = S;
	fn anti_wedge(self, rhs: WX) -> Self::Output {
		S(self.0 * rhs.0)
	}
}

impl AntiWedge<XY> for Y {
	type Output = Zero;
	fn anti_wedge(self, _rhs: XY) -> Self::Output {
		Zero {}
	}
}

impl AntiWedge<XYW> for Y {
	type Output = Y;
	fn anti_wedge(self, rhs: XYW) -> Self::Output {
		Y(self.0 * rhs.0)
	}
}

impl AntiWedge<S> for W {
	type Output = Zero;
	fn anti_wedge(self, _rhs: S) -> Self::Output {
		Zero {}
	}
}

impl AntiWedge<X> for W {
	type Output = Zero;
	fn anti_wedge(self, _rhs: X) -> Self::Output {
		Zero {}
	}
}

impl AntiWedge<Y> for W {
	type Output = Zero;
	fn anti_wedge(self, _rhs: Y) -> Self::Output {
		Zero {}
	}
}

impl AntiWedge<W> for W {
	type Output = Zero;
	fn anti_wedge(self, _rhs: W) -> Self::Output {
		Zero {}
	}
}

impl AntiWedge<YW> for W {
	type Output = Zero;
	fn anti_wedge(self, _rhs: YW) -> Self::Output {
		Zero {}
	}
}

impl AntiWedge<WX> for W {
	type Output = Zero;
	fn anti_wedge(self, _rhs: WX) -> Self::Output {
		Zero {}
	}
}

impl AntiWedge<XY> for W {
	type Output = S;
	fn anti_wedge(self, rhs: XY) -> Self::Output {
		S(self.0 * rhs.0)
	}
}

impl AntiWedge<XYW> for W {
	type Output = W;
	fn anti_wedge(self, rhs: XYW) -> Self::Output {
		W(self.0 * rhs.0)
	}
}

impl AntiWedge<S> for YW {
	type Output = Zero;
	fn anti_wedge(self, _rhs: S) -> Self::Output {
		Zero {}
	}
}

impl AntiWedge<X> for YW {
	type Output = S;
	fn anti_wedge(self, rhs: X) -> Self::Output {
		S(self.0 * rhs.0)
	}
}

impl AntiWedge<Y> for YW {
	type Output = Zero;
	fn anti_wedge(self, _rhs: Y) -> Self::Output {
		Zero {}
	}
}

impl AntiWedge<W> for YW {
	type Output = Zero;
	fn anti_wedge(self, _rhs: W) -> Self::Output {
		Zero {}
	}
}

impl AntiWedge<YW> for YW {
	type Output = Zero;
	fn anti_wedge(self, _rhs: YW) -> Self::Output {
		Zero {}
	}
}

impl AntiWedge<WX> for YW {
	type Output = W;
	fn anti_wedge(self, rhs: WX) -> Self::Output {
		W(self.0 * rhs.0)
	}
}

impl AntiWedge<XY> for YW {
	type Output = Y;
	fn anti_wedge(self, rhs: XY) -> Self::Output {
		Y(-self.0 * rhs.0)
	}
}

impl AntiWedge<XYW> for YW {
	type Output = YW;
	fn anti_wedge(self, rhs: XYW) -> Self::Output {
		YW(self.0 * rhs.0)
	}
}

impl AntiWedge<S> for WX {
	type Output = Zero;
	fn anti_wedge(self, _rhs: S) -> Self::Output {
		Zero {}
	}
}

impl AntiWedge<X> for WX {
	type Output = Zero;
	fn anti_wedge(self, _rhs: X) -> Self::Output {
		Zero {}
	}
}

impl AntiWedge<Y> for WX {
	type Output = S;
	fn anti_wedge(self, rhs: Y) -> Self::Output {
		S(self.0 * rhs.0)
	}
}

impl AntiWedge<W> for WX {
	type Output = Zero;
	fn anti_wedge(self, _rhs: W) -> Self::Output {
		Zero {}
	}
}

impl AntiWedge<YW> for WX {
	type Output = W;
	fn anti_wedge(self, rhs: YW) -> Self::Output {
		W(-self.0 * rhs.0)
	}
}

impl AntiWedge<WX> for WX {
	type Output = Zero;
	fn anti_wedge(self, _rhs: WX) -> Self::Output {
		Zero {}
	}
}

impl AntiWedge<XY> for WX {
	type Output = X;
	fn anti_wedge(self, rhs: XY) -> Self::Output {
		X(self.0 * rhs.0)
	}
}

impl AntiWedge<XYW> for WX {
	type Output = WX;
	fn anti_wedge(self, rhs: XYW) -> Self::Output {
		WX(self.0 * rhs.0)
	}
}

impl AntiWedge<S> for XY {
	type Output = Zero;
	fn anti_wedge(self, _rhs: S) -> Self::Output {
		Zero {}
	}
}

impl AntiWedge<X> for XY {
	type Output = Zero;
	fn anti_wedge(self, _rhs: X) -> Self::Output {
		Zero {}
	}
}

impl AntiWedge<Y> for XY {
	type Output = Zero;
	fn anti_wedge(self, _rhs: Y) -> Self::Output {
		Zero {}
	}
}

impl AntiWedge<W> for XY {
	type Output = S;
	fn anti_wedge(self, rhs: W) -> Self::Output {
		S(self.0 * rhs.0)
	}
}

impl AntiWedge<YW> for XY {
	type Output = Y;
	fn anti_wedge(self, rhs: YW) -> Self::Output {
		Y(self.0 * rhs.0)
	}
}

impl AntiWedge<WX> for XY {
	type Output = X;
	fn anti_wedge(self, rhs: WX) -> Self::Output {
		X(-self.0 * rhs.0)
	}
}

impl AntiWedge<XY> for XY {
	type Output = Zero;
	fn anti_wedge(self, _rhs: XY) -> Self::Output {
		Zero {}
	}
}

impl AntiWedge<XYW> for XY {
	type Output = XY;
	fn anti_wedge(self, rhs: XYW) -> Self::Output {
		XY(self.0 * rhs.0)
	}
}

impl AntiWedge<S> for XYW {
	type Output = S;
	fn anti_wedge(self, rhs: S) -> Self::Output {
		S(self.0 * rhs.0)
	}
}

impl AntiWedge<X> for XYW {
	type Output = X;
	fn anti_wedge(self, rhs: X) -> Self::Output {
		X(self.0 * rhs.0)
	}
}

impl AntiWedge<Y> for XYW {
	type Output = Y;
	fn anti_wedge(self, rhs: Y) -> Self::Output {
		Y(self.0 * rhs.0)
	}
}

impl AntiWedge<W> for XYW {
	type Output = W;
	fn anti_wedge(self, rhs: W) -> Self::Output {
		W(self.0 * rhs.0)
	}
}

impl AntiWedge<YW> for XYW {
	type Output = YW;
	fn anti_wedge(self, rhs: YW) -> Self::Output {
		YW(self.0 * rhs.0)
	}
}

impl AntiWedge<WX> for XYW {
	type Output = WX;
	fn anti_wedge(self, rhs: WX) -> Self::Output {
		WX(self.0 * rhs.0)
	}
}

impl AntiWedge<XY> for XYW {
	type Output = XY;
	fn anti_wedge(self, rhs: XY) -> Self::Output {
		XY(self.0 * rhs.0)
	}
}

impl AntiWedge<XYW> for XYW {
	type Output = XYW;
	fn anti_wedge(self, rhs: XYW) -> Self::Output {
		XYW(self.0 * rhs.0)
	}
}

use derive_more::{Add, Mul, Neg, Sub};

use super::traits::*;

/// The scalar type (real numbers).
/// Squares to 1.
#[derive(Copy, Clone, Debug, PartialEq, Neg, Add, Sub, Mul)]
pub struct R(pub f64);

/// Squares to 1.
#[derive(Copy, Clone, Debug, PartialEq, Neg, Add, Sub)]
pub struct X(pub f64);

/// Squares to 1.
#[derive(Copy, Clone, Debug, PartialEq, Neg, Add, Sub)]
pub struct Y(pub f64);

/// Squares to 0.
#[derive(Copy, Clone, Debug, PartialEq, Neg, Add, Sub)]
pub struct W(pub f64);

/// Squares to 0.
#[derive(Copy, Clone, Debug, PartialEq, Neg, Add, Sub)]
pub struct YW(pub f64);

/// Squares to 0.
#[derive(Copy, Clone, Debug, PartialEq, Neg, Add, Sub)]
pub struct WX(pub f64);

/// Squares to -1.
#[derive(Copy, Clone, Debug, PartialEq, Neg, Add, Sub)]
pub struct XY(pub f64);

/// The pseudo-scalar.
/// Squares to 0.
#[derive(Copy, Clone, Debug, PartialEq, Neg, Add, Sub, Mul)]
pub struct XYW(pub f64);

impl Geometric<R> for R {
    type Output = R;
    fn geometric(self, rhs: R) -> Self::Output {
        R(self.0 * rhs.0)
    }
}

impl Geometric<X> for R {
    type Output = X;
    fn geometric(self, rhs: X) -> Self::Output {
        X(self.0 * rhs.0)
    }
}

impl Geometric<Y> for R {
    type Output = Y;
    fn geometric(self, rhs: Y) -> Self::Output {
        Y(self.0 * rhs.0)
    }
}

impl Geometric<W> for R {
    type Output = W;
    fn geometric(self, rhs: W) -> Self::Output {
        W(self.0 * rhs.0)
    }
}

impl Geometric<YW> for R {
    type Output = YW;
    fn geometric(self, rhs: YW) -> Self::Output {
        YW(self.0 * rhs.0)
    }
}

impl Geometric<WX> for R {
    type Output = WX;
    fn geometric(self, rhs: WX) -> Self::Output {
        WX(self.0 * rhs.0)
    }
}

impl Geometric<XY> for R {
    type Output = XY;
    fn geometric(self, rhs: XY) -> Self::Output {
        XY(self.0 * rhs.0)
    }
}

impl Geometric<XYW> for R {
    type Output = XYW;
    fn geometric(self, rhs: XYW) -> Self::Output {
        XYW(self.0 * rhs.0)
    }
}

impl Geometric<R> for X {
    type Output = X;
    fn geometric(self, rhs: R) -> Self::Output {
        X(self.0 * rhs.0)
    }
}

impl Geometric<X> for X {
    type Output = R;
    fn geometric(self, rhs: X) -> Self::Output {
        R(self.0 * rhs.0)
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

impl Geometric<R> for Y {
    type Output = Y;
    fn geometric(self, rhs: R) -> Self::Output {
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
    type Output = R;
    fn geometric(self, rhs: Y) -> Self::Output {
        R(self.0 * rhs.0)
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

impl Geometric<R> for W {
    type Output = W;
    fn geometric(self, rhs: R) -> Self::Output {
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

// Omitted: W * W = 0

// Omitted: W * YW = 0

// Omitted: W * WX = 0

impl Geometric<XY> for W {
    type Output = XYW;
    fn geometric(self, rhs: XY) -> Self::Output {
        XYW(self.0 * rhs.0)
    }
}

// Omitted: W * XYW = 0

impl Geometric<R> for YW {
    type Output = YW;
    fn geometric(self, rhs: R) -> Self::Output {
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

// Omitted: YW * W = 0

// Omitted: YW * YW = 0

// Omitted: YW * WX = 0

impl Geometric<XY> for YW {
    type Output = WX;
    fn geometric(self, rhs: XY) -> Self::Output {
        WX(self.0 * rhs.0)
    }
}

// Omitted: YW * XYW = 0

impl Geometric<R> for WX {
    type Output = WX;
    fn geometric(self, rhs: R) -> Self::Output {
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

// Omitted: WX * W = 0

// Omitted: WX * YW = 0

// Omitted: WX * WX = 0

impl Geometric<XY> for WX {
    type Output = YW;
    fn geometric(self, rhs: XY) -> Self::Output {
        YW(-self.0 * rhs.0)
    }
}

// Omitted: WX * XYW = 0

impl Geometric<R> for XY {
    type Output = XY;
    fn geometric(self, rhs: R) -> Self::Output {
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
    type Output = R;
    fn geometric(self, rhs: XY) -> Self::Output {
        R(-self.0 * rhs.0)
    }
}

impl Geometric<XYW> for XY {
    type Output = W;
    fn geometric(self, rhs: XYW) -> Self::Output {
        W(-self.0 * rhs.0)
    }
}

impl Geometric<R> for XYW {
    type Output = XYW;
    fn geometric(self, rhs: R) -> Self::Output {
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

// Omitted: XYW * W = 0

// Omitted: XYW * YW = 0

// Omitted: XYW * WX = 0

impl Geometric<XY> for XYW {
    type Output = W;
    fn geometric(self, rhs: XY) -> Self::Output {
        W(-self.0 * rhs.0)
    }
}

// Omitted: XYW * XYW = 0

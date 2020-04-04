//! # Transform
//!
//! ## Operations
//! ```text
//! Transform.wedge(Transform) -> Transform
//!
//! Transform.wedge(Line) -> Line
//! Line.wedge(Transform) -> Line
//! Transform.wedge(Rotor) -> Transform
//! Rotor.wedge(Transform) -> Transform
//! Transform.geometric(Translator) -> Transform
//! Translator.geometric(Transform) -> Transform
//! Transform.dot(Translator) -> Transform
//! Translator.dot(Transform) -> Transform
//! Transform.wedge(Translator) -> Transform
//! Translator.wedge(Transform) -> Transform
//! Transform.wedge(Motor) -> Transform
//! Motor.wedge(Transform) -> Transform
//! ```

use super::*;

#[derive(
    Copy, Clone, Debug, PartialEq, PartialOrd, derive_more::Neg, derive_more::Add, derive_more::Sub,
)]
pub struct Transform {
    pub s: S,
    pub yw: YW,
    pub wx: WX,
    pub xy: XY,
    pub xyw: XYW,
}

// ---------------------------------------------------------------------
// Transform OP Point:

// Omitted: Transform geometric Point = self.s.geometric(rhs.w) + self.s.geometric(rhs.x) + self.s.geometric(rhs.y) + self.wx.geometric(rhs.x) + self.wx.geometric(rhs.y) + self.xy.geometric(rhs.w) + self.xy.geometric(rhs.x) + self.xy.geometric(rhs.y) + self.xyw.geometric(rhs.x) + self.xyw.geometric(rhs.y) + self.yw.geometric(rhs.x) + self.yw.geometric(rhs.y)
// Omitted: Transform anti_geometric Point = self.s.anti_geometric(rhs.w) + self.wx.anti_geometric(rhs.w) + self.wx.anti_geometric(rhs.x) + self.wx.anti_geometric(rhs.y) + self.xy.anti_geometric(rhs.w) + self.xyw.anti_geometric(rhs.w) + self.xyw.anti_geometric(rhs.x) + self.xyw.anti_geometric(rhs.y) + self.yw.anti_geometric(rhs.w) + self.yw.anti_geometric(rhs.x) + self.yw.anti_geometric(rhs.y)
// Omitted: Transform dot Point = self.s.dot(rhs.w) + self.s.dot(rhs.x) + self.s.dot(rhs.y) + self.wx.dot(rhs.x) + self.xy.dot(rhs.x) + self.xy.dot(rhs.y) + self.xyw.dot(rhs.x) + self.xyw.dot(rhs.y) + self.yw.dot(rhs.y)
// Omitted: Transform wedge Point = self.s.wedge(rhs.w) + self.s.wedge(rhs.x) + self.s.wedge(rhs.y) + self.wx.wedge(rhs.y) + self.xy.wedge(rhs.w) + self.yw.wedge(rhs.x)
// Omitted: Transform anti_wedge Point = self.wx.anti_wedge(rhs.y) + self.xy.anti_wedge(rhs.w) + self.xyw.anti_wedge(rhs.w) + self.xyw.anti_wedge(rhs.x) + self.xyw.anti_wedge(rhs.y) + self.yw.anti_wedge(rhs.x)

// ---------------------------------------------------------------------
// Transform OP Line:

// Omitted: Transform geometric Line = self.s.geometric(rhs.wx) + self.s.geometric(rhs.xy) + self.s.geometric(rhs.yw) + self.wx.geometric(rhs.xy) + self.xy.geometric(rhs.wx) + self.xy.geometric(rhs.xy) + self.xy.geometric(rhs.yw) + self.xyw.geometric(rhs.xy) + self.yw.geometric(rhs.xy)
// Omitted: Transform anti_geometric Line = self.s.anti_geometric(rhs.wx) + self.s.anti_geometric(rhs.yw) + self.wx.anti_geometric(rhs.wx) + self.wx.anti_geometric(rhs.xy) + self.wx.anti_geometric(rhs.yw) + self.xy.anti_geometric(rhs.wx) + self.xy.anti_geometric(rhs.yw) + self.xyw.anti_geometric(rhs.wx) + self.xyw.anti_geometric(rhs.xy) + self.xyw.anti_geometric(rhs.yw) + self.yw.anti_geometric(rhs.wx) + self.yw.anti_geometric(rhs.xy) + self.yw.anti_geometric(rhs.yw)
// Omitted: Transform dot Line = self.s.dot(rhs.wx) + self.s.dot(rhs.xy) + self.s.dot(rhs.yw) + self.xy.dot(rhs.xy) + self.xyw.dot(rhs.xy)

// Transform.wedge(Line) -> Line
impl Wedge<Line> for Transform {
    type Output = Line;
    fn wedge(self, rhs: Line) -> Self::Output {
        Line {
            yw: self.s.wedge(rhs.yw),
            wx: -self.s.wedge(rhs.wx),
            xy: self.s.wedge(rhs.xy),
        }
    }
}

// Omitted: Transform anti_wedge Line = self.wx.anti_wedge(rhs.xy) + self.wx.anti_wedge(rhs.yw) + self.xy.anti_wedge(rhs.wx) + self.xy.anti_wedge(rhs.yw) + self.xyw.anti_wedge(rhs.wx) + self.xyw.anti_wedge(rhs.xy) + self.xyw.anti_wedge(rhs.yw) + self.yw.anti_wedge(rhs.wx) + self.yw.anti_wedge(rhs.xy)

// ---------------------------------------------------------------------
// Transform OP Rotor:

// Omitted: Transform geometric Rotor = self.s.geometric(rhs.s) + self.s.geometric(rhs.xy) + self.wx.geometric(rhs.s) + self.wx.geometric(rhs.xy) + self.xy.geometric(rhs.s) + self.xy.geometric(rhs.xy) + self.xyw.geometric(rhs.s) + self.xyw.geometric(rhs.xy) + self.yw.geometric(rhs.s) + self.yw.geometric(rhs.xy)
// Omitted: Transform anti_geometric Rotor = self.wx.anti_geometric(rhs.s) + self.wx.anti_geometric(rhs.xy) + self.xyw.anti_geometric(rhs.s) + self.xyw.anti_geometric(rhs.xy) + self.yw.anti_geometric(rhs.s) + self.yw.anti_geometric(rhs.xy)
// Omitted: Transform dot Rotor = self.s.dot(rhs.s) + self.s.dot(rhs.xy) + self.wx.dot(rhs.s) + self.xy.dot(rhs.s) + self.xy.dot(rhs.xy) + self.xyw.dot(rhs.s) + self.xyw.dot(rhs.xy) + self.yw.dot(rhs.s)

// Transform.wedge(Rotor) -> Transform
impl Wedge<Rotor> for Transform {
    type Output = Transform;
    fn wedge(self, rhs: Rotor) -> Self::Output {
        Transform {
            s: self.s.wedge(rhs.s),
            yw: self.yw.wedge(rhs.s),
            wx: -self.wx.wedge(rhs.s),
            xy: self.s.wedge(rhs.xy) + self.xy.wedge(rhs.s),
            xyw: self.xyw.wedge(rhs.s),
        }
    }
}

// Omitted: Transform anti_wedge Rotor = self.wx.anti_wedge(rhs.xy) + self.xyw.anti_wedge(rhs.s) + self.xyw.anti_wedge(rhs.xy) + self.yw.anti_wedge(rhs.xy)

// ---------------------------------------------------------------------
// Transform OP Translator:

// Transform.geometric(Translator) -> Transform
impl Geometric<Translator> for Transform {
    type Output = Transform;
    fn geometric(self, rhs: Translator) -> Self::Output {
        Transform {
            s: self.s.geometric(rhs.s),
            yw: self.s.geometric(rhs.yw) + self.xy.geometric(rhs.wx) + self.yw.geometric(rhs.s),
            wx: -self.s.geometric(rhs.wx) - self.wx.geometric(rhs.s) + self.xy.geometric(rhs.yw),
            xy: self.xy.geometric(rhs.s),
            xyw: self.xyw.geometric(rhs.s),
        }
    }
}

// Omitted: Transform anti_geometric Translator = self.s.anti_geometric(rhs.wx) + self.s.anti_geometric(rhs.yw) + self.wx.anti_geometric(rhs.s) + self.wx.anti_geometric(rhs.wx) + self.wx.anti_geometric(rhs.yw) + self.xy.anti_geometric(rhs.wx) + self.xy.anti_geometric(rhs.yw) + self.xyw.anti_geometric(rhs.s) + self.xyw.anti_geometric(rhs.wx) + self.xyw.anti_geometric(rhs.yw) + self.yw.anti_geometric(rhs.s) + self.yw.anti_geometric(rhs.wx) + self.yw.anti_geometric(rhs.yw)

// Transform.dot(Translator) -> Transform
impl Dot<Translator> for Transform {
    type Output = Transform;
    fn dot(self, rhs: Translator) -> Self::Output {
        Transform {
            s: self.s.dot(rhs.s),
            yw: self.s.dot(rhs.yw) + self.yw.dot(rhs.s),
            wx: -self.s.dot(rhs.wx) - self.wx.dot(rhs.s),
            xy: self.xy.dot(rhs.s),
            xyw: self.xyw.dot(rhs.s),
        }
    }
}

// Transform.wedge(Translator) -> Transform
impl Wedge<Translator> for Transform {
    type Output = Transform;
    fn wedge(self, rhs: Translator) -> Self::Output {
        Transform {
            s: self.s.wedge(rhs.s),
            yw: self.s.wedge(rhs.yw) + self.yw.wedge(rhs.s),
            wx: -self.s.wedge(rhs.wx) - self.wx.wedge(rhs.s),
            xy: self.xy.wedge(rhs.s),
            xyw: self.xyw.wedge(rhs.s),
        }
    }
}

// Omitted: Transform anti_wedge Translator = self.wx.anti_wedge(rhs.yw) + self.xy.anti_wedge(rhs.wx) + self.xy.anti_wedge(rhs.yw) + self.xyw.anti_wedge(rhs.s) + self.xyw.anti_wedge(rhs.wx) + self.xyw.anti_wedge(rhs.yw) + self.yw.anti_wedge(rhs.wx)

// ---------------------------------------------------------------------
// Transform OP Motor:

// Omitted: Transform geometric Motor = self.s.geometric(rhs.s) + self.s.geometric(rhs.wx) + self.s.geometric(rhs.xy) + self.s.geometric(rhs.yw) + self.wx.geometric(rhs.s) + self.wx.geometric(rhs.xy) + self.xy.geometric(rhs.s) + self.xy.geometric(rhs.wx) + self.xy.geometric(rhs.xy) + self.xy.geometric(rhs.yw) + self.xyw.geometric(rhs.s) + self.xyw.geometric(rhs.xy) + self.yw.geometric(rhs.s) + self.yw.geometric(rhs.xy)
// Omitted: Transform anti_geometric Motor = self.s.anti_geometric(rhs.wx) + self.s.anti_geometric(rhs.yw) + self.wx.anti_geometric(rhs.s) + self.wx.anti_geometric(rhs.wx) + self.wx.anti_geometric(rhs.xy) + self.wx.anti_geometric(rhs.yw) + self.xy.anti_geometric(rhs.wx) + self.xy.anti_geometric(rhs.yw) + self.xyw.anti_geometric(rhs.s) + self.xyw.anti_geometric(rhs.wx) + self.xyw.anti_geometric(rhs.xy) + self.xyw.anti_geometric(rhs.yw) + self.yw.anti_geometric(rhs.s) + self.yw.anti_geometric(rhs.wx) + self.yw.anti_geometric(rhs.xy) + self.yw.anti_geometric(rhs.yw)
// Omitted: Transform dot Motor = self.s.dot(rhs.s) + self.s.dot(rhs.wx) + self.s.dot(rhs.xy) + self.s.dot(rhs.yw) + self.wx.dot(rhs.s) + self.xy.dot(rhs.s) + self.xy.dot(rhs.xy) + self.xyw.dot(rhs.s) + self.xyw.dot(rhs.xy) + self.yw.dot(rhs.s)

// Transform.wedge(Motor) -> Transform
impl Wedge<Motor> for Transform {
    type Output = Transform;
    fn wedge(self, rhs: Motor) -> Self::Output {
        Transform {
            s: self.s.wedge(rhs.s),
            yw: self.s.wedge(rhs.yw) + self.yw.wedge(rhs.s),
            wx: -self.s.wedge(rhs.wx) - self.wx.wedge(rhs.s),
            xy: self.s.wedge(rhs.xy) + self.xy.wedge(rhs.s),
            xyw: self.xyw.wedge(rhs.s),
        }
    }
}

// Omitted: Transform anti_wedge Motor = self.wx.anti_wedge(rhs.xy) + self.wx.anti_wedge(rhs.yw) + self.xy.anti_wedge(rhs.wx) + self.xy.anti_wedge(rhs.yw) + self.xyw.anti_wedge(rhs.s) + self.xyw.anti_wedge(rhs.wx) + self.xyw.anti_wedge(rhs.xy) + self.xyw.anti_wedge(rhs.yw) + self.yw.anti_wedge(rhs.wx) + self.yw.anti_wedge(rhs.xy)

// ---------------------------------------------------------------------
// Transform OP Transform:

// Omitted: Transform geometric Transform = self.s.geometric(rhs.s) + self.s.geometric(rhs.wx) + self.s.geometric(rhs.xy) + self.s.geometric(rhs.xyw) + self.s.geometric(rhs.yw) + self.wx.geometric(rhs.s) + self.wx.geometric(rhs.xy) + self.xy.geometric(rhs.s) + self.xy.geometric(rhs.wx) + self.xy.geometric(rhs.xy) + self.xy.geometric(rhs.xyw) + self.xy.geometric(rhs.yw) + self.xyw.geometric(rhs.s) + self.xyw.geometric(rhs.xy) + self.yw.geometric(rhs.s) + self.yw.geometric(rhs.xy)
// Omitted: Transform anti_geometric Transform = self.s.anti_geometric(rhs.wx) + self.s.anti_geometric(rhs.xyw) + self.s.anti_geometric(rhs.yw) + self.wx.anti_geometric(rhs.s) + self.wx.anti_geometric(rhs.wx) + self.wx.anti_geometric(rhs.xy) + self.wx.anti_geometric(rhs.xyw) + self.wx.anti_geometric(rhs.yw) + self.xy.anti_geometric(rhs.wx) + self.xy.anti_geometric(rhs.xyw) + self.xy.anti_geometric(rhs.yw) + self.xyw.anti_geometric(rhs.s) + self.xyw.anti_geometric(rhs.wx) + self.xyw.anti_geometric(rhs.xy) + self.xyw.anti_geometric(rhs.xyw) + self.xyw.anti_geometric(rhs.yw) + self.yw.anti_geometric(rhs.s) + self.yw.anti_geometric(rhs.wx) + self.yw.anti_geometric(rhs.xy) + self.yw.anti_geometric(rhs.xyw) + self.yw.anti_geometric(rhs.yw)
// Omitted: Transform dot Transform = self.s.dot(rhs.s) + self.s.dot(rhs.wx) + self.s.dot(rhs.xy) + self.s.dot(rhs.xyw) + self.s.dot(rhs.yw) + self.wx.dot(rhs.s) + self.xy.dot(rhs.s) + self.xy.dot(rhs.xy) + self.xy.dot(rhs.xyw) + self.xyw.dot(rhs.s) + self.xyw.dot(rhs.xy) + self.yw.dot(rhs.s)

// Transform.wedge(Transform) -> Transform
impl Wedge<Transform> for Transform {
    type Output = Transform;
    fn wedge(self, rhs: Transform) -> Self::Output {
        Transform {
            s: self.s.wedge(rhs.s),
            yw: self.s.wedge(rhs.yw) + self.yw.wedge(rhs.s),
            wx: -self.s.wedge(rhs.wx) - self.wx.wedge(rhs.s),
            xy: self.s.wedge(rhs.xy) + self.xy.wedge(rhs.s),
            xyw: self.s.wedge(rhs.xyw) + self.xyw.wedge(rhs.s),
        }
    }
}

// Omitted: Transform anti_wedge Transform = self.s.anti_wedge(rhs.xyw) + self.wx.anti_wedge(rhs.xy) + self.wx.anti_wedge(rhs.xyw) + self.wx.anti_wedge(rhs.yw) + self.xy.anti_wedge(rhs.wx) + self.xy.anti_wedge(rhs.xyw) + self.xy.anti_wedge(rhs.yw) + self.xyw.anti_wedge(rhs.s) + self.xyw.anti_wedge(rhs.wx) + self.xyw.anti_wedge(rhs.xy) + self.xyw.anti_wedge(rhs.xyw) + self.xyw.anti_wedge(rhs.yw) + self.yw.anti_wedge(rhs.wx) + self.yw.anti_wedge(rhs.xy) + self.yw.anti_wedge(rhs.xyw)

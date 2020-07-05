//! # Vec3
//!
//! ## Operations
//! ```text
//! Vec3.geometric(Vec3) -> Motor
//! Vec3.dot(Vec3) -> S
//! Vec3.wedge(Vec3) -> Line
//! Vec3.geometric(Vec2) -> Motor
//! Vec2.geometric(Vec3) -> Motor
//! Vec3.anti_geometric(Vec2) -> Vec2
//! Vec2.anti_geometric(Vec3) -> Vec2
//! Vec3.dot(Vec2) -> S
//! Vec2.dot(Vec3) -> S
//! Vec3.wedge(Vec2) -> Line
//! Vec2.wedge(Vec3) -> Line
//! Vec3.anti_geometric(Line) -> Motor
//! Line.anti_geometric(Vec3) -> Motor
//! Vec3.dot(Line) -> Vec3
//! Line.dot(Vec3) -> Vec3
//! Vec3.wedge(Line) -> XYW
//! Line.wedge(Vec3) -> XYW
//! Vec3.anti_wedge(Line) -> S
//! Line.anti_wedge(Vec3) -> S
//! Vec3.anti_geometric(Translator) -> Motor
//! Translator.anti_geometric(Vec3) -> Motor
//! Vec3.dot(Translator) -> Vec3
//! Translator.dot(Vec3) -> Vec3
//! Vec3.anti_wedge(Translator) -> S
//! Translator.anti_wedge(Vec3) -> S
//! Vec3.anti_geometric(Rotor) -> Rotor
//! Rotor.anti_geometric(Vec3) -> Rotor
//! Vec3.dot(Rotor) -> Vec3
//! Rotor.dot(Vec3) -> Vec3
//! Vec3.anti_wedge(Rotor) -> S
//! Rotor.anti_wedge(Vec3) -> S
//! Vec3.anti_geometric(Motor) -> Motor
//! Motor.anti_geometric(Vec3) -> Motor
//! Vec3.dot(Motor) -> Vec3
//! Motor.dot(Vec3) -> Vec3
//! Vec3.anti_wedge(Motor) -> S
//! Motor.anti_wedge(Vec3) -> S
//! ```

use super::*;

#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    PartialEq,
    PartialOrd,
    derive_more::Neg,
    derive_more::Add,
    derive_more::Sub,
)]
pub struct Vec3 {
    pub x: X,
    pub y: Y,
    pub w: W,
}

// ---------------------------------------------------------------------

impl RCompl for Vec3 {
    type Output = Line;
    fn rcompl(self) -> Self::Output {
        Line {
            dx: self.x.rcompl(),
            dy: self.y.rcompl(),
            m: self.w.rcompl(),
        }
    }
}

impl LCompl for Vec3 {
    type Output = Line;
    fn lcompl(self) -> Self::Output {
        Line {
            dx: self.x.lcompl(),
            dy: self.y.lcompl(),
            m: self.w.lcompl(),
        }
    }
}

impl Reverse for Vec3 {
    fn rev(self) -> Self {
        Vec3 {
            x: self.x,
            y: self.y,
            w: self.w,
        }
    }
}

impl AntiReverse for Vec3 {
    fn arev(self) -> Self {
        Vec3 {
            x: -self.x,
            y: -self.y,
            w: -self.w,
        }
    }
}

// ---------------------------------------------------------------------
// Vec3 OP Vec2:

// Vec3.geometric(Vec2) -> Motor
impl Geometric<Vec2> for Vec3 {
    type Output = Motor;
    fn geometric(self, rhs: Vec2) -> Self::Output {
        // Motor {
        //     s : S(self.x.0 * rhs.x.0) + S(self.y.0 * rhs.y.0),
        //     yw: YW(self.w.0 * rhs.y.0),
        //     wx: WX(self.w.0 * rhs.x.0),
        //     xy: XY(self.x.0 * rhs.y.0) + XY(self.y.0 * rhs.x.0),
        // }
        Motor {
            s: self.x.geometric(rhs.x) + self.y.geometric(rhs.y),
            yw: -self.w.geometric(rhs.y),
            wx: self.w.geometric(rhs.x),
            xy: self.x.geometric(rhs.y) - self.y.geometric(rhs.x),
        }
    }
}

// Vec3.anti_geometric(Vec2) -> Vec2
impl AntiGeometric<Vec2> for Vec3 {
    type Output = Vec2;
    fn anti_geometric(self, rhs: Vec2) -> Self::Output {
        // Vec2 {
        //     x: X(self.w.0 * rhs.y.0),
        //     y: Y(self.w.0 * rhs.x.0),
        // }
        Vec2 {
            x: self.w.anti_geometric(rhs.y),
            y: -self.w.anti_geometric(rhs.x),
        }
    }
}

// Vec3.dot(Vec2) -> S
impl Dot<Vec2> for Vec3 {
    type Output = S;
    fn dot(self, rhs: Vec2) -> Self::Output {
        // S(self.x.0 * rhs.x.0) + S(self.y.0 * rhs.y.0)
        self.x.dot(rhs.x) + self.y.dot(rhs.y)
    }
}

// Vec3.wedge(Vec2) -> Line
impl Wedge<Vec2> for Vec3 {
    type Output = Line;
    fn wedge(self, rhs: Vec2) -> Self::Output {
        // Line {
        //     dx: YW(self.w.0 * rhs.y.0),
        //     dy: WX(self.w.0 * rhs.x.0),
        //     m : XY(self.x.0 * rhs.y.0) + XY(self.y.0 * rhs.x.0),
        // }
        Line {
            dx: -self.w.wedge(rhs.y),
            dy: self.w.wedge(rhs.x),
            m: self.x.wedge(rhs.y) - self.y.wedge(rhs.x),
        }
    }
}

// Omitted: Vec3 anti_wedge Vec2 = 0

// ---------------------------------------------------------------------
// Vec3 OP Vec3:

// Vec3.geometric(Vec3) -> Motor
impl Geometric<Vec3> for Vec3 {
    type Output = Motor;
    fn geometric(self, rhs: Vec3) -> Self::Output {
        // Motor {
        //     s : S(self.x.0 * rhs.x.0) + S(self.y.0 * rhs.y.0),
        //     yw: YW(self.w.0 * rhs.y.0) + YW(self.y.0 * rhs.w.0),
        //     wx: WX(self.w.0 * rhs.x.0) + WX(self.x.0 * rhs.w.0),
        //     xy: XY(self.x.0 * rhs.y.0) + XY(self.y.0 * rhs.x.0),
        // }
        Motor {
            s: self.x.geometric(rhs.x) + self.y.geometric(rhs.y),
            yw: -self.w.geometric(rhs.y) + self.y.geometric(rhs.w),
            wx: self.w.geometric(rhs.x) - self.x.geometric(rhs.w),
            xy: self.x.geometric(rhs.y) - self.y.geometric(rhs.x),
        }
    }
}

// Omitted: Vec3 anti_geometric Vec3 = self.w !* rhs.w + self.w !* rhs.x + self.w !* rhs.y + self.x !* rhs.w + self.y !* rhs.w

// Vec3.dot(Vec3) -> S
impl Dot<Vec3> for Vec3 {
    type Output = S;
    fn dot(self, rhs: Vec3) -> Self::Output {
        // S(self.x.0 * rhs.x.0) + S(self.y.0 * rhs.y.0)
        self.x.dot(rhs.x) + self.y.dot(rhs.y)
    }
}

// Vec3.wedge(Vec3) -> Line
impl Wedge<Vec3> for Vec3 {
    type Output = Line;
    fn wedge(self, rhs: Vec3) -> Self::Output {
        // Line {
        //     dx: YW(self.w.0 * rhs.y.0) + YW(self.y.0 * rhs.w.0),
        //     dy: WX(self.w.0 * rhs.x.0) + WX(self.x.0 * rhs.w.0),
        //     m : XY(self.x.0 * rhs.y.0) + XY(self.y.0 * rhs.x.0),
        // }
        Line {
            dx: -self.w.wedge(rhs.y) + self.y.wedge(rhs.w),
            dy: self.w.wedge(rhs.x) - self.x.wedge(rhs.w),
            m: self.x.wedge(rhs.y) - self.y.wedge(rhs.x),
        }
    }
}

// Omitted: Vec3 anti_wedge Vec3 = 0

// ---------------------------------------------------------------------
// Vec3 OP Line:

// Omitted: Vec3 geometric Line = self.w * rhs.m + self.x * rhs.dx + self.x * rhs.dy + self.x * rhs.m + self.y * rhs.dx + self.y * rhs.dy + self.y * rhs.m

// Vec3.anti_geometric(Line) -> Motor
impl AntiGeometric<Line> for Vec3 {
    type Output = Motor;
    fn anti_geometric(self, rhs: Line) -> Self::Output {
        // Motor {
        //     s : S(self.w.0 * rhs.m.0) + S(self.x.0 * rhs.dx.0) + S(self.y.0 * rhs.dy.0),
        //     yw: YW(self.w.0 * rhs.dy.0),
        //     wx: WX(self.w.0 * rhs.dx.0),
        //     xy: XY(self.x.0 * rhs.dy.0) + XY(self.y.0 * rhs.dx.0),
        // }
        Motor {
            s: self.w.anti_geometric(rhs.m)
                + self.x.anti_geometric(rhs.dx)
                + self.y.anti_geometric(rhs.dy),
            yw: self.w.anti_geometric(rhs.dy),
            wx: -self.w.anti_geometric(rhs.dx),
            xy: -self.x.anti_geometric(rhs.dy) + self.y.anti_geometric(rhs.dx),
        }
    }
}

// Vec3.dot(Line) -> Vec3
impl Dot<Line> for Vec3 {
    type Output = Vec3;
    fn dot(self, rhs: Line) -> Self::Output {
        // Vec3 {
        //     x: X(self.y.0 * rhs.m.0),
        //     y: Y(self.x.0 * rhs.m.0),
        //     w: W(self.x.0 * rhs.dy.0) + W(self.y.0 * rhs.dx.0),
        // }
        Vec3 {
            x: -self.y.dot(rhs.m),
            y: self.x.dot(rhs.m),
            w: -self.x.dot(rhs.dy) + self.y.dot(rhs.dx),
        }
    }
}

// Vec3.wedge(Line) -> XYW
impl Wedge<Line> for Vec3 {
    type Output = XYW;
    fn wedge(self, rhs: Line) -> Self::Output {
        // XYW(self.w.0 * rhs.m.0) + XYW(self.x.0 * rhs.dx.0) + XYW(self.y.0 * rhs.dy.0)
        self.w.wedge(rhs.m) + self.x.wedge(rhs.dx) + self.y.wedge(rhs.dy)
    }
}

// Vec3.anti_wedge(Line) -> S
impl AntiWedge<Line> for Vec3 {
    type Output = S;
    fn anti_wedge(self, rhs: Line) -> Self::Output {
        // S(self.w.0 * rhs.m.0) + S(self.x.0 * rhs.dx.0) + S(self.y.0 * rhs.dy.0)
        self.w.anti_wedge(rhs.m) + self.x.anti_wedge(rhs.dx) + self.y.anti_wedge(rhs.dy)
    }
}

// ---------------------------------------------------------------------
// Vec3 OP Translator:

// Omitted: Vec3 geometric Translator = self.w * rhs.s + self.x * rhs.s + self.x * rhs.wx + self.x * rhs.yw + self.y * rhs.s + self.y * rhs.wx + self.y * rhs.yw

// Vec3.anti_geometric(Translator) -> Motor
impl AntiGeometric<Translator> for Vec3 {
    type Output = Motor;
    fn anti_geometric(self, rhs: Translator) -> Self::Output {
        // Motor {
        //     s : S(self.x.0 * rhs.yw.0) + S(self.y.0 * rhs.wx.0),
        //     yw: YW(self.w.0 * rhs.wx.0),
        //     wx: WX(self.w.0 * rhs.yw.0),
        //     xy: XY(self.w.0 * rhs.s.0) + XY(self.x.0 * rhs.wx.0) + XY(self.y.0 * rhs.yw.0),
        // }
        Motor {
            s: self.x.anti_geometric(rhs.yw) + self.y.anti_geometric(rhs.wx),
            yw: self.w.anti_geometric(rhs.wx),
            wx: -self.w.anti_geometric(rhs.yw),
            xy: -self.w.anti_geometric(rhs.s) - self.x.anti_geometric(rhs.wx)
                + self.y.anti_geometric(rhs.yw),
        }
    }
}

// Vec3.dot(Translator) -> Vec3
impl Dot<Translator> for Vec3 {
    type Output = Vec3;
    fn dot(self, rhs: Translator) -> Self::Output {
        // Vec3 {
        //     x: X(self.x.0 * rhs.s.0),
        //     y: Y(self.y.0 * rhs.s.0),
        //     w: W(self.w.0 * rhs.s.0) + W(self.x.0 * rhs.wx.0) + W(self.y.0 * rhs.yw.0),
        // }
        Vec3 {
            x: self.x.dot(rhs.s),
            y: self.y.dot(rhs.s),
            w: self.w.dot(rhs.s) - self.x.dot(rhs.wx) + self.y.dot(rhs.yw),
        }
    }
}

// Omitted: Vec3 wedge Translator = self.w ^ rhs.s + self.x ^ rhs.s + self.x ^ rhs.yw + self.y ^ rhs.s + self.y ^ rhs.wx

// Vec3.anti_wedge(Translator) -> S
impl AntiWedge<Translator> for Vec3 {
    type Output = S;
    fn anti_wedge(self, rhs: Translator) -> Self::Output {
        // S(self.x.0 * rhs.yw.0) + S(self.y.0 * rhs.wx.0)
        self.x.anti_wedge(rhs.yw) + self.y.anti_wedge(rhs.wx)
    }
}

// ---------------------------------------------------------------------
// Vec3 OP Rotor:

// Omitted: Vec3 geometric Rotor = self.w * rhs.s + self.w * rhs.xy + self.x * rhs.s + self.x * rhs.xy + self.y * rhs.s + self.y * rhs.xy

// Vec3.anti_geometric(Rotor) -> Rotor
impl AntiGeometric<Rotor> for Vec3 {
    type Output = Rotor;
    fn anti_geometric(self, rhs: Rotor) -> Self::Output {
        // Rotor {
        //     s : S(self.w.0 * rhs.xy.0),
        //     xy: XY(self.w.0 * rhs.s.0),
        // }
        Rotor {
            s: self.w.anti_geometric(rhs.xy),
            xy: -self.w.anti_geometric(rhs.s),
        }
    }
}

// Vec3.dot(Rotor) -> Vec3
impl Dot<Rotor> for Vec3 {
    type Output = Vec3;
    fn dot(self, rhs: Rotor) -> Self::Output {
        // Vec3 {
        //     x: X(self.x.0 * rhs.s.0) + X(self.y.0 * rhs.xy.0),
        //     y: Y(self.x.0 * rhs.xy.0) + Y(self.y.0 * rhs.s.0),
        //     w: W(self.w.0 * rhs.s.0),
        // }
        Vec3 {
            x: self.x.dot(rhs.s) - self.y.dot(rhs.xy),
            y: self.x.dot(rhs.xy) + self.y.dot(rhs.s),
            w: self.w.dot(rhs.s),
        }
    }
}

// Omitted: Vec3 wedge Rotor = self.w ^ rhs.s + self.w ^ rhs.xy + self.x ^ rhs.s + self.y ^ rhs.s

// Vec3.anti_wedge(Rotor) -> S
impl AntiWedge<Rotor> for Vec3 {
    type Output = S;
    fn anti_wedge(self, rhs: Rotor) -> Self::Output {
        // S(self.w.0 * rhs.xy.0)
        self.w.anti_wedge(rhs.xy)
    }
}

// ---------------------------------------------------------------------
// Vec3 OP Motor:

// Omitted: Vec3 geometric Motor = self.w * rhs.s + self.w * rhs.xy + self.x * rhs.s + self.x * rhs.wx + self.x * rhs.xy + self.x * rhs.yw + self.y * rhs.s + self.y * rhs.wx + self.y * rhs.xy + self.y * rhs.yw

// Vec3.anti_geometric(Motor) -> Motor
impl AntiGeometric<Motor> for Vec3 {
    type Output = Motor;
    fn anti_geometric(self, rhs: Motor) -> Self::Output {
        // Motor {
        //     s : S(self.w.0 * rhs.xy.0) + S(self.x.0 * rhs.yw.0) + S(self.y.0 * rhs.wx.0),
        //     yw: YW(self.w.0 * rhs.wx.0),
        //     wx: WX(self.w.0 * rhs.yw.0),
        //     xy: XY(self.w.0 * rhs.s.0) + XY(self.x.0 * rhs.wx.0) + XY(self.y.0 * rhs.yw.0),
        // }
        Motor {
            s: self.w.anti_geometric(rhs.xy)
                + self.x.anti_geometric(rhs.yw)
                + self.y.anti_geometric(rhs.wx),
            yw: self.w.anti_geometric(rhs.wx),
            wx: -self.w.anti_geometric(rhs.yw),
            xy: -self.w.anti_geometric(rhs.s) - self.x.anti_geometric(rhs.wx)
                + self.y.anti_geometric(rhs.yw),
        }
    }
}

// Vec3.dot(Motor) -> Vec3
impl Dot<Motor> for Vec3 {
    type Output = Vec3;
    fn dot(self, rhs: Motor) -> Self::Output {
        // Vec3 {
        //     x: X(self.x.0 * rhs.s.0) + X(self.y.0 * rhs.xy.0),
        //     y: Y(self.x.0 * rhs.xy.0) + Y(self.y.0 * rhs.s.0),
        //     w: W(self.w.0 * rhs.s.0) + W(self.x.0 * rhs.wx.0) + W(self.y.0 * rhs.yw.0),
        // }
        Vec3 {
            x: self.x.dot(rhs.s) - self.y.dot(rhs.xy),
            y: self.x.dot(rhs.xy) + self.y.dot(rhs.s),
            w: self.w.dot(rhs.s) - self.x.dot(rhs.wx) + self.y.dot(rhs.yw),
        }
    }
}

// Omitted: Vec3 wedge Motor = self.w ^ rhs.s + self.w ^ rhs.xy + self.x ^ rhs.s + self.x ^ rhs.yw + self.y ^ rhs.s + self.y ^ rhs.wx

// Vec3.anti_wedge(Motor) -> S
impl AntiWedge<Motor> for Vec3 {
    type Output = S;
    fn anti_wedge(self, rhs: Motor) -> Self::Output {
        // S(self.w.0 * rhs.xy.0) + S(self.x.0 * rhs.yw.0) + S(self.y.0 * rhs.wx.0)
        self.w.anti_wedge(rhs.xy) + self.x.anti_wedge(rhs.yw) + self.y.anti_wedge(rhs.wx)
    }
}

//! # Motor
//!
//! ## Operations
//! ```text
//! Motor.geometric(Motor) -> Motor
//! Motor.dot(Motor) -> Motor
//! Motor.wedge(Motor) -> Motor
//! Motor.anti_wedge(Motor) -> Vec3
//! Motor.anti_geometric(Vec2) -> Rotor
//! Vec2.anti_geometric(Motor) -> Rotor
//! Motor.dot(Vec2) -> Vec3
//! Vec2.dot(Motor) -> Vec3
//! Motor.anti_wedge(Vec2) -> S
//! Vec2.anti_wedge(Motor) -> S
//! Motor.anti_geometric(Vec3) -> Motor
//! Vec3.anti_geometric(Motor) -> Motor
//! Motor.dot(Vec3) -> Vec3
//! Vec3.dot(Motor) -> Vec3
//! Motor.anti_wedge(Vec3) -> S
//! Vec3.anti_wedge(Motor) -> S
//! Motor.geometric(Line) -> Motor
//! Line.geometric(Motor) -> Motor
//! Motor.dot(Line) -> Motor
//! Line.dot(Motor) -> Motor
//! Motor.wedge(Line) -> Line
//! Line.wedge(Motor) -> Line
//! Motor.anti_wedge(Line) -> Vec3
//! Line.anti_wedge(Motor) -> Vec3
//! Motor.geometric(Translator) -> Motor
//! Translator.geometric(Motor) -> Motor
//! Motor.dot(Translator) -> Motor
//! Translator.dot(Motor) -> Motor
//! Motor.wedge(Translator) -> Motor
//! Translator.wedge(Motor) -> Motor
//! Motor.anti_wedge(Translator) -> Vec3
//! Translator.anti_wedge(Motor) -> Vec3
//! Motor.geometric(Rotor) -> Motor
//! Rotor.geometric(Motor) -> Motor
//! Motor.anti_geometric(Rotor) -> Vec2
//! Rotor.anti_geometric(Motor) -> Vec2
//! Motor.dot(Rotor) -> Motor
//! Rotor.dot(Motor) -> Motor
//! Motor.wedge(Rotor) -> Motor
//! Rotor.wedge(Motor) -> Motor
//! Motor.anti_wedge(Rotor) -> Vec2
//! Rotor.anti_wedge(Motor) -> Vec2
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
pub struct Motor {
    pub s: S,
    pub yw: YW,
    pub wx: WX,
    pub xy: XY,
}

// ---------------------------------------------------------------------
// Omitted: Motor.rcompl() -> self.s.rcompl() + self.wx.rcompl() + self.xy.rcompl() + self.yw.rcompl()
// Omitted: Motor.lcompl() -> self.s.lcompl() + self.wx.lcompl() + self.xy.lcompl() + self.yw.lcompl()

impl Reverse for Motor {
    fn rev(self) -> Self {
        Motor {
            s: self.s,
            yw: -self.yw,
            wx: -self.wx,
            xy: -self.xy,
        }
    }
}

impl AntiReverse for Motor {
    fn arev(self) -> Self {
        Motor {
            s: -self.s,
            yw: self.yw,
            wx: self.wx,
            xy: self.xy,
        }
    }
}

// ---------------------------------------------------------------------
// Motor OP Vec2:

// Omitted: Motor geometric Vec2 = self.s.geometric(rhs.x) + self.s.geometric(rhs.y) + self.wx.geometric(rhs.x) + self.wx.geometric(rhs.y) + self.xy.geometric(rhs.x) + self.xy.geometric(rhs.y) + self.yw.geometric(rhs.x) + self.yw.geometric(rhs.y)

// Motor.anti_geometric(Vec2) -> Rotor
impl AntiGeometric<Vec2> for Motor {
    type Output = Rotor;
    fn anti_geometric(self, rhs: Vec2) -> Self::Output {
        Rotor {
            s: self.wx.anti_geometric(rhs.y) + self.yw.anti_geometric(rhs.x),
            xy: self.wx.anti_geometric(rhs.x) - self.yw.anti_geometric(rhs.y),
        }
    }
}

// Motor.dot(Vec2) -> Vec3
impl Dot<Vec2> for Motor {
    type Output = Vec3;
    fn dot(self, rhs: Vec2) -> Self::Output {
        Vec3 {
            x: self.s.dot(rhs.x) + self.xy.dot(rhs.y),
            y: self.s.dot(rhs.y) - self.xy.dot(rhs.x),
            w: self.wx.dot(rhs.x) - self.yw.dot(rhs.y),
        }
    }
}

// Omitted: Motor wedge Vec2 = self.s.wedge(rhs.x) + self.s.wedge(rhs.y) + self.wx.wedge(rhs.y) + self.yw.wedge(rhs.x)

// Motor.anti_wedge(Vec2) -> S
impl AntiWedge<Vec2> for Motor {
    type Output = S;
    fn anti_wedge(self, rhs: Vec2) -> Self::Output {
        self.wx.anti_wedge(rhs.y) + self.yw.anti_wedge(rhs.x)
    }
}

// ---------------------------------------------------------------------
// Motor OP Vec3:

// Omitted: Motor geometric Vec3 = self.s.geometric(rhs.w) + self.s.geometric(rhs.x) + self.s.geometric(rhs.y) + self.wx.geometric(rhs.x) + self.wx.geometric(rhs.y) + self.xy.geometric(rhs.w) + self.xy.geometric(rhs.x) + self.xy.geometric(rhs.y) + self.yw.geometric(rhs.x) + self.yw.geometric(rhs.y)

// Motor.anti_geometric(Vec3) -> Motor
impl AntiGeometric<Vec3> for Motor {
    type Output = Motor;
    fn anti_geometric(self, rhs: Vec3) -> Self::Output {
        Motor {
            s: self.wx.anti_geometric(rhs.y)
                + self.xy.anti_geometric(rhs.w)
                + self.yw.anti_geometric(rhs.x),
            yw: -self.wx.anti_geometric(rhs.w),
            wx: self.yw.anti_geometric(rhs.w),
            xy: -self.s.anti_geometric(rhs.w) + self.wx.anti_geometric(rhs.x)
                - self.yw.anti_geometric(rhs.y),
        }
    }
}

// Motor.dot(Vec3) -> Vec3
impl Dot<Vec3> for Motor {
    type Output = Vec3;
    fn dot(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.s.dot(rhs.x) + self.xy.dot(rhs.y),
            y: self.s.dot(rhs.y) - self.xy.dot(rhs.x),
            w: self.s.dot(rhs.w) + self.wx.dot(rhs.x) - self.yw.dot(rhs.y),
        }
    }
}

// Omitted: Motor wedge Vec3 = self.s.wedge(rhs.w) + self.s.wedge(rhs.x) + self.s.wedge(rhs.y) + self.wx.wedge(rhs.y) + self.xy.wedge(rhs.w) + self.yw.wedge(rhs.x)

// Motor.anti_wedge(Vec3) -> S
impl AntiWedge<Vec3> for Motor {
    type Output = S;
    fn anti_wedge(self, rhs: Vec3) -> Self::Output {
        self.wx.anti_wedge(rhs.y) + self.xy.anti_wedge(rhs.w) + self.yw.anti_wedge(rhs.x)
    }
}

// ---------------------------------------------------------------------
// Motor OP Line:

// Motor.geometric(Line) -> Motor
impl Geometric<Line> for Motor {
    type Output = Motor;
    fn geometric(self, rhs: Line) -> Self::Output {
        Motor {
            s: -self.xy.geometric(rhs.m),
            yw: self.s.geometric(rhs.dx) - self.wx.geometric(rhs.m) + self.xy.geometric(rhs.dy),
            wx: self.s.geometric(rhs.dy) - self.xy.geometric(rhs.dx) + self.yw.geometric(rhs.m),
            xy: self.s.geometric(rhs.m),
        }
    }
}

// Omitted: Motor anti_geometric Line = self.s.anti_geometric(rhs.dx) + self.s.anti_geometric(rhs.dy) + self.wx.anti_geometric(rhs.dx) + self.wx.anti_geometric(rhs.dy) + self.wx.anti_geometric(rhs.m) + self.xy.anti_geometric(rhs.dx) + self.xy.anti_geometric(rhs.dy) + self.yw.anti_geometric(rhs.dx) + self.yw.anti_geometric(rhs.dy) + self.yw.anti_geometric(rhs.m)

// Motor.dot(Line) -> Motor
impl Dot<Line> for Motor {
    type Output = Motor;
    fn dot(self, rhs: Line) -> Self::Output {
        Motor {
            s: -self.xy.dot(rhs.m),
            yw: self.s.dot(rhs.dx),
            wx: self.s.dot(rhs.dy),
            xy: self.s.dot(rhs.m),
        }
    }
}

// Motor.wedge(Line) -> Line
impl Wedge<Line> for Motor {
    type Output = Line;
    fn wedge(self, rhs: Line) -> Self::Output {
        Line {
            dx: self.s.wedge(rhs.dx),
            dy: self.s.wedge(rhs.dy),
            m: self.s.wedge(rhs.m),
        }
    }
}

// Motor.anti_wedge(Line) -> Vec3
impl AntiWedge<Line> for Motor {
    type Output = Vec3;
    fn anti_wedge(self, rhs: Line) -> Self::Output {
        Vec3 {
            x: self.wx.anti_wedge(rhs.m) - self.xy.anti_wedge(rhs.dy),
            y: self.xy.anti_wedge(rhs.dx) - self.yw.anti_wedge(rhs.m),
            w: -self.wx.anti_wedge(rhs.dx) + self.yw.anti_wedge(rhs.dy),
        }
    }
}

// ---------------------------------------------------------------------
// Motor OP Translator:

// Motor.geometric(Translator) -> Motor
impl Geometric<Translator> for Motor {
    type Output = Motor;
    fn geometric(self, rhs: Translator) -> Self::Output {
        Motor {
            s: self.s.geometric(rhs.s),
            yw: self.s.geometric(rhs.yw) + self.xy.geometric(rhs.wx) + self.yw.geometric(rhs.s),
            wx: self.s.geometric(rhs.wx) + self.wx.geometric(rhs.s) - self.xy.geometric(rhs.yw),
            xy: self.xy.geometric(rhs.s),
        }
    }
}

// Omitted: Motor anti_geometric Translator = self.s.anti_geometric(rhs.wx) + self.s.anti_geometric(rhs.yw) + self.wx.anti_geometric(rhs.s) + self.wx.anti_geometric(rhs.wx) + self.wx.anti_geometric(rhs.yw) + self.xy.anti_geometric(rhs.wx) + self.xy.anti_geometric(rhs.yw) + self.yw.anti_geometric(rhs.s) + self.yw.anti_geometric(rhs.wx) + self.yw.anti_geometric(rhs.yw)

// Motor.dot(Translator) -> Motor
impl Dot<Translator> for Motor {
    type Output = Motor;
    fn dot(self, rhs: Translator) -> Self::Output {
        Motor {
            s: self.s.dot(rhs.s),
            yw: self.s.dot(rhs.yw) + self.yw.dot(rhs.s),
            wx: self.s.dot(rhs.wx) + self.wx.dot(rhs.s),
            xy: self.xy.dot(rhs.s),
        }
    }
}

// Motor.wedge(Translator) -> Motor
impl Wedge<Translator> for Motor {
    type Output = Motor;
    fn wedge(self, rhs: Translator) -> Self::Output {
        Motor {
            s: self.s.wedge(rhs.s),
            yw: self.s.wedge(rhs.yw) + self.yw.wedge(rhs.s),
            wx: self.s.wedge(rhs.wx) + self.wx.wedge(rhs.s),
            xy: self.xy.wedge(rhs.s),
        }
    }
}

// Motor.anti_wedge(Translator) -> Vec3
impl AntiWedge<Translator> for Motor {
    type Output = Vec3;
    fn anti_wedge(self, rhs: Translator) -> Self::Output {
        Vec3 {
            x: -self.xy.anti_wedge(rhs.wx),
            y: self.xy.anti_wedge(rhs.yw),
            w: -self.wx.anti_wedge(rhs.yw) + self.yw.anti_wedge(rhs.wx),
        }
    }
}

// ---------------------------------------------------------------------
// Motor OP Rotor:

// Motor.geometric(Rotor) -> Motor
impl Geometric<Rotor> for Motor {
    type Output = Motor;
    fn geometric(self, rhs: Rotor) -> Self::Output {
        Motor {
            s: self.s.geometric(rhs.s) - self.xy.geometric(rhs.xy),
            yw: -self.wx.geometric(rhs.xy) + self.yw.geometric(rhs.s),
            wx: self.wx.geometric(rhs.s) + self.yw.geometric(rhs.xy),
            xy: self.s.geometric(rhs.xy) + self.xy.geometric(rhs.s),
        }
    }
}

// Motor.anti_geometric(Rotor) -> Vec2
impl AntiGeometric<Rotor> for Motor {
    type Output = Vec2;
    fn anti_geometric(self, rhs: Rotor) -> Self::Output {
        Vec2 {
            x: self.wx.anti_geometric(rhs.xy) + self.yw.anti_geometric(rhs.s),
            y: self.wx.anti_geometric(rhs.s) - self.yw.anti_geometric(rhs.xy),
        }
    }
}

// Motor.dot(Rotor) -> Motor
impl Dot<Rotor> for Motor {
    type Output = Motor;
    fn dot(self, rhs: Rotor) -> Self::Output {
        Motor {
            s: self.s.dot(rhs.s) - self.xy.dot(rhs.xy),
            yw: self.yw.dot(rhs.s),
            wx: self.wx.dot(rhs.s),
            xy: self.s.dot(rhs.xy) + self.xy.dot(rhs.s),
        }
    }
}

// Motor.wedge(Rotor) -> Motor
impl Wedge<Rotor> for Motor {
    type Output = Motor;
    fn wedge(self, rhs: Rotor) -> Self::Output {
        Motor {
            s: self.s.wedge(rhs.s),
            yw: self.yw.wedge(rhs.s),
            wx: self.wx.wedge(rhs.s),
            xy: self.s.wedge(rhs.xy) + self.xy.wedge(rhs.s),
        }
    }
}

// Motor.anti_wedge(Rotor) -> Vec2
impl AntiWedge<Rotor> for Motor {
    type Output = Vec2;
    fn anti_wedge(self, rhs: Rotor) -> Self::Output {
        Vec2 {
            x: self.wx.anti_wedge(rhs.xy),
            y: -self.yw.anti_wedge(rhs.xy),
        }
    }
}

// ---------------------------------------------------------------------
// Motor OP Motor:

// Motor.geometric(Motor) -> Motor
impl Geometric<Motor> for Motor {
    type Output = Motor;
    fn geometric(self, rhs: Motor) -> Self::Output {
        Motor {
            s: self.s.geometric(rhs.s) - self.xy.geometric(rhs.xy),
            yw: self.s.geometric(rhs.yw) - self.wx.geometric(rhs.xy)
                + self.xy.geometric(rhs.wx)
                + self.yw.geometric(rhs.s),
            wx: self.s.geometric(rhs.wx) + self.wx.geometric(rhs.s) - self.xy.geometric(rhs.yw)
                + self.yw.geometric(rhs.xy),
            xy: self.s.geometric(rhs.xy) + self.xy.geometric(rhs.s),
        }
    }
}

// Omitted: Motor anti_geometric Motor = self.s.anti_geometric(rhs.wx) + self.s.anti_geometric(rhs.yw) + self.wx.anti_geometric(rhs.s) + self.wx.anti_geometric(rhs.wx) + self.wx.anti_geometric(rhs.xy) + self.wx.anti_geometric(rhs.yw) + self.xy.anti_geometric(rhs.wx) + self.xy.anti_geometric(rhs.yw) + self.yw.anti_geometric(rhs.s) + self.yw.anti_geometric(rhs.wx) + self.yw.anti_geometric(rhs.xy) + self.yw.anti_geometric(rhs.yw)

// Motor.dot(Motor) -> Motor
impl Dot<Motor> for Motor {
    type Output = Motor;
    fn dot(self, rhs: Motor) -> Self::Output {
        Motor {
            s: self.s.dot(rhs.s) - self.xy.dot(rhs.xy),
            yw: self.s.dot(rhs.yw) + self.yw.dot(rhs.s),
            wx: self.s.dot(rhs.wx) + self.wx.dot(rhs.s),
            xy: self.s.dot(rhs.xy) + self.xy.dot(rhs.s),
        }
    }
}

// Motor.wedge(Motor) -> Motor
impl Wedge<Motor> for Motor {
    type Output = Motor;
    fn wedge(self, rhs: Motor) -> Self::Output {
        Motor {
            s: self.s.wedge(rhs.s),
            yw: self.s.wedge(rhs.yw) + self.yw.wedge(rhs.s),
            wx: self.s.wedge(rhs.wx) + self.wx.wedge(rhs.s),
            xy: self.s.wedge(rhs.xy) + self.xy.wedge(rhs.s),
        }
    }
}

// Motor.anti_wedge(Motor) -> Vec3
impl AntiWedge<Motor> for Motor {
    type Output = Vec3;
    fn anti_wedge(self, rhs: Motor) -> Self::Output {
        Vec3 {
            x: self.wx.anti_wedge(rhs.xy) - self.xy.anti_wedge(rhs.wx),
            y: self.xy.anti_wedge(rhs.yw) - self.yw.anti_wedge(rhs.xy),
            w: -self.wx.anti_wedge(rhs.yw) + self.yw.anti_wedge(rhs.wx),
        }
    }
}

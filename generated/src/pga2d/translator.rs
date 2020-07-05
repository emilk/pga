//! # Translator
//!
//! ## Operations
//! ```text
//! Translator.geometric(Translator) -> Translator
//! Translator.dot(Translator) -> Translator
//! Translator.wedge(Translator) -> Translator
//! Translator.anti_geometric(Vec2) -> Rotor
//! Vec2.anti_geometric(Translator) -> Rotor
//! Translator.dot(Vec2) -> Vec3
//! Vec2.dot(Translator) -> Vec3
//! Translator.anti_wedge(Vec2) -> S
//! Vec2.anti_wedge(Translator) -> S
//! Translator.anti_geometric(Vec3) -> Motor
//! Vec3.anti_geometric(Translator) -> Motor
//! Translator.dot(Vec3) -> Vec3
//! Vec3.dot(Translator) -> Vec3
//! Translator.anti_wedge(Vec3) -> S
//! Vec3.anti_wedge(Translator) -> S
//! Translator.geometric(Line) -> Line
//! Line.geometric(Translator) -> Line
//! Translator.dot(Line) -> Line
//! Line.dot(Translator) -> Line
//! Translator.wedge(Line) -> Line
//! Line.wedge(Translator) -> Line
//! Translator.anti_wedge(Line) -> Vec3
//! Line.anti_wedge(Translator) -> Vec3
//! Translator.geometric(Rotor) -> Motor
//! Rotor.geometric(Translator) -> Motor
//! Translator.anti_geometric(Rotor) -> Vec2
//! Rotor.anti_geometric(Translator) -> Vec2
//! Translator.dot(Rotor) -> Motor
//! Rotor.dot(Translator) -> Motor
//! Translator.wedge(Rotor) -> Motor
//! Rotor.wedge(Translator) -> Motor
//! Translator.anti_wedge(Rotor) -> Vec2
//! Rotor.anti_wedge(Translator) -> Vec2
//! Translator.geometric(Motor) -> Motor
//! Motor.geometric(Translator) -> Motor
//! Translator.dot(Motor) -> Motor
//! Motor.dot(Translator) -> Motor
//! Translator.wedge(Motor) -> Motor
//! Motor.wedge(Translator) -> Motor
//! Translator.anti_wedge(Motor) -> Vec3
//! Motor.anti_wedge(Translator) -> Vec3
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
pub struct Translator {
    pub s: S,
    pub yw: YW,
    pub wx: WX,
}

// ---------------------------------------------------------------------
// Omitted: Translator.rcompl() -> self.s.rcompl() + self.wx.rcompl() + self.yw.rcompl()
// Omitted: Translator.lcompl() -> self.s.lcompl() + self.wx.lcompl() + self.yw.lcompl()

impl Reverse for Translator {
    fn rev(self) -> Self {
        Translator {
            s: self.s,
            yw: -self.yw,
            wx: -self.wx,
        }
    }
}

impl AntiReverse for Translator {
    fn arev(self) -> Self {
        Translator {
            s: -self.s,
            yw: self.yw,
            wx: self.wx,
        }
    }
}

// ---------------------------------------------------------------------
// Translator OP Vec2:

// Omitted: Translator geometric Vec2 = self.s * rhs.x + self.s * rhs.y + self.wx * rhs.x + self.wx * rhs.y + self.yw * rhs.x + self.yw * rhs.y

// Translator.anti_geometric(Vec2) -> Rotor
impl AntiGeometric<Vec2> for Translator {
    type Output = Rotor;
    fn anti_geometric(self, rhs: Vec2) -> Self::Output {
        // Rotor {
        //     s : S(self.wx.0 * rhs.y.0) + S(self.yw.0 * rhs.x.0),
        //     xy: XY(self.wx.0 * rhs.x.0) + XY(self.yw.0 * rhs.y.0),
        // }
        Rotor {
            s: self.wx.anti_geometric(rhs.y) + self.yw.anti_geometric(rhs.x),
            xy: self.wx.anti_geometric(rhs.x) - self.yw.anti_geometric(rhs.y),
        }
    }
}

// Translator.dot(Vec2) -> Vec3
impl Dot<Vec2> for Translator {
    type Output = Vec3;
    fn dot(self, rhs: Vec2) -> Self::Output {
        // Vec3 {
        //     x: X(self.s.0 * rhs.x.0),
        //     y: Y(self.s.0 * rhs.y.0),
        //     w: W(self.wx.0 * rhs.x.0) + W(self.yw.0 * rhs.y.0),
        // }
        Vec3 {
            x: self.s.dot(rhs.x),
            y: self.s.dot(rhs.y),
            w: self.wx.dot(rhs.x) - self.yw.dot(rhs.y),
        }
    }
}

// Omitted: Translator wedge Vec2 = self.s ^ rhs.x + self.s ^ rhs.y + self.wx ^ rhs.y + self.yw ^ rhs.x

// Translator.anti_wedge(Vec2) -> S
impl AntiWedge<Vec2> for Translator {
    type Output = S;
    fn anti_wedge(self, rhs: Vec2) -> Self::Output {
        // S(self.wx.0 * rhs.y.0) + S(self.yw.0 * rhs.x.0)
        self.wx.anti_wedge(rhs.y) + self.yw.anti_wedge(rhs.x)
    }
}

// ---------------------------------------------------------------------
// Translator OP Vec3:

// Omitted: Translator geometric Vec3 = self.s * rhs.w + self.s * rhs.x + self.s * rhs.y + self.wx * rhs.x + self.wx * rhs.y + self.yw * rhs.x + self.yw * rhs.y

// Translator.anti_geometric(Vec3) -> Motor
impl AntiGeometric<Vec3> for Translator {
    type Output = Motor;
    fn anti_geometric(self, rhs: Vec3) -> Self::Output {
        // Motor {
        //     s : S(self.wx.0 * rhs.y.0) + S(self.yw.0 * rhs.x.0),
        //     yw: YW(self.wx.0 * rhs.w.0),
        //     wx: WX(self.yw.0 * rhs.w.0),
        //     xy: XY(self.s.0 * rhs.w.0) + XY(self.wx.0 * rhs.x.0) + XY(self.yw.0 * rhs.y.0),
        // }
        Motor {
            s: self.wx.anti_geometric(rhs.y) + self.yw.anti_geometric(rhs.x),
            yw: -self.wx.anti_geometric(rhs.w),
            wx: self.yw.anti_geometric(rhs.w),
            xy: -self.s.anti_geometric(rhs.w) + self.wx.anti_geometric(rhs.x)
                - self.yw.anti_geometric(rhs.y),
        }
    }
}

// Translator.dot(Vec3) -> Vec3
impl Dot<Vec3> for Translator {
    type Output = Vec3;
    fn dot(self, rhs: Vec3) -> Self::Output {
        // Vec3 {
        //     x: X(self.s.0 * rhs.x.0),
        //     y: Y(self.s.0 * rhs.y.0),
        //     w: W(self.s.0 * rhs.w.0) + W(self.wx.0 * rhs.x.0) + W(self.yw.0 * rhs.y.0),
        // }
        Vec3 {
            x: self.s.dot(rhs.x),
            y: self.s.dot(rhs.y),
            w: self.s.dot(rhs.w) + self.wx.dot(rhs.x) - self.yw.dot(rhs.y),
        }
    }
}

// Omitted: Translator wedge Vec3 = self.s ^ rhs.w + self.s ^ rhs.x + self.s ^ rhs.y + self.wx ^ rhs.y + self.yw ^ rhs.x

// Translator.anti_wedge(Vec3) -> S
impl AntiWedge<Vec3> for Translator {
    type Output = S;
    fn anti_wedge(self, rhs: Vec3) -> Self::Output {
        // S(self.wx.0 * rhs.y.0) + S(self.yw.0 * rhs.x.0)
        self.wx.anti_wedge(rhs.y) + self.yw.anti_wedge(rhs.x)
    }
}

// ---------------------------------------------------------------------
// Translator OP Line:

// Translator.geometric(Line) -> Line
impl Geometric<Line> for Translator {
    type Output = Line;
    fn geometric(self, rhs: Line) -> Self::Output {
        // Line {
        //     dx: YW(self.s.0 * rhs.dx.0) + YW(self.wx.0 * rhs.m.0),
        //     dy: WX(self.s.0 * rhs.dy.0) + WX(self.yw.0 * rhs.m.0),
        //     m : XY(self.s.0 * rhs.m.0),
        // }
        Line {
            dx: self.s.geometric(rhs.dx) - self.wx.geometric(rhs.m),
            dy: self.s.geometric(rhs.dy) + self.yw.geometric(rhs.m),
            m: self.s.geometric(rhs.m),
        }
    }
}

// Omitted: Translator anti_geometric Line = self.s !* rhs.dx + self.s !* rhs.dy + self.wx !* rhs.dx + self.wx !* rhs.dy + self.wx !* rhs.m + self.yw !* rhs.dx + self.yw !* rhs.dy + self.yw !* rhs.m

// Translator.dot(Line) -> Line
impl Dot<Line> for Translator {
    type Output = Line;
    fn dot(self, rhs: Line) -> Self::Output {
        // Line {
        //     dx: YW(self.s.0 * rhs.dx.0),
        //     dy: WX(self.s.0 * rhs.dy.0),
        //     m : XY(self.s.0 * rhs.m.0),
        // }
        Line {
            dx: self.s.dot(rhs.dx),
            dy: self.s.dot(rhs.dy),
            m: self.s.dot(rhs.m),
        }
    }
}

// Translator.wedge(Line) -> Line
impl Wedge<Line> for Translator {
    type Output = Line;
    fn wedge(self, rhs: Line) -> Self::Output {
        // Line {
        //     dx: YW(self.s.0 * rhs.dx.0),
        //     dy: WX(self.s.0 * rhs.dy.0),
        //     m : XY(self.s.0 * rhs.m.0),
        // }
        Line {
            dx: self.s.wedge(rhs.dx),
            dy: self.s.wedge(rhs.dy),
            m: self.s.wedge(rhs.m),
        }
    }
}

// Translator.anti_wedge(Line) -> Vec3
impl AntiWedge<Line> for Translator {
    type Output = Vec3;
    fn anti_wedge(self, rhs: Line) -> Self::Output {
        // Vec3 {
        //     x: X(self.wx.0 * rhs.m.0),
        //     y: Y(self.yw.0 * rhs.m.0),
        //     w: W(self.wx.0 * rhs.dx.0) + W(self.yw.0 * rhs.dy.0),
        // }
        Vec3 {
            x: self.wx.anti_wedge(rhs.m),
            y: -self.yw.anti_wedge(rhs.m),
            w: -self.wx.anti_wedge(rhs.dx) + self.yw.anti_wedge(rhs.dy),
        }
    }
}

// ---------------------------------------------------------------------
// Translator OP Translator:

// Translator.geometric(Translator) -> Translator
impl Geometric<Translator> for Translator {
    type Output = Translator;
    fn geometric(self, rhs: Translator) -> Self::Output {
        // Translator {
        //     s : S(self.s.0 * rhs.s.0),
        //     yw: YW(self.s.0 * rhs.yw.0) + YW(self.yw.0 * rhs.s.0),
        //     wx: WX(self.s.0 * rhs.wx.0) + WX(self.wx.0 * rhs.s.0),
        // }
        Translator {
            s: self.s.geometric(rhs.s),
            yw: self.s.geometric(rhs.yw) + self.yw.geometric(rhs.s),
            wx: self.s.geometric(rhs.wx) + self.wx.geometric(rhs.s),
        }
    }
}

// Omitted: Translator anti_geometric Translator = self.s !* rhs.wx + self.s !* rhs.yw + self.wx !* rhs.s + self.wx !* rhs.wx + self.wx !* rhs.yw + self.yw !* rhs.s + self.yw !* rhs.wx + self.yw !* rhs.yw

// Translator.dot(Translator) -> Translator
impl Dot<Translator> for Translator {
    type Output = Translator;
    fn dot(self, rhs: Translator) -> Self::Output {
        // Translator {
        //     s : S(self.s.0 * rhs.s.0),
        //     yw: YW(self.s.0 * rhs.yw.0) + YW(self.yw.0 * rhs.s.0),
        //     wx: WX(self.s.0 * rhs.wx.0) + WX(self.wx.0 * rhs.s.0),
        // }
        Translator {
            s: self.s.dot(rhs.s),
            yw: self.s.dot(rhs.yw) + self.yw.dot(rhs.s),
            wx: self.s.dot(rhs.wx) + self.wx.dot(rhs.s),
        }
    }
}

// Translator.wedge(Translator) -> Translator
impl Wedge<Translator> for Translator {
    type Output = Translator;
    fn wedge(self, rhs: Translator) -> Self::Output {
        // Translator {
        //     s : S(self.s.0 * rhs.s.0),
        //     yw: YW(self.s.0 * rhs.yw.0) + YW(self.yw.0 * rhs.s.0),
        //     wx: WX(self.s.0 * rhs.wx.0) + WX(self.wx.0 * rhs.s.0),
        // }
        Translator {
            s: self.s.wedge(rhs.s),
            yw: self.s.wedge(rhs.yw) + self.yw.wedge(rhs.s),
            wx: self.s.wedge(rhs.wx) + self.wx.wedge(rhs.s),
        }
    }
}

// Omitted: Translator anti_wedge Translator = self.wx & rhs.yw + self.yw & rhs.wx

// ---------------------------------------------------------------------
// Translator OP Rotor:

// Translator.geometric(Rotor) -> Motor
impl Geometric<Rotor> for Translator {
    type Output = Motor;
    fn geometric(self, rhs: Rotor) -> Self::Output {
        // Motor {
        //     s : S(self.s.0 * rhs.s.0),
        //     yw: YW(self.wx.0 * rhs.xy.0) + YW(self.yw.0 * rhs.s.0),
        //     wx: WX(self.wx.0 * rhs.s.0) + WX(self.yw.0 * rhs.xy.0),
        //     xy: XY(self.s.0 * rhs.xy.0),
        // }
        Motor {
            s: self.s.geometric(rhs.s),
            yw: -self.wx.geometric(rhs.xy) + self.yw.geometric(rhs.s),
            wx: self.wx.geometric(rhs.s) + self.yw.geometric(rhs.xy),
            xy: self.s.geometric(rhs.xy),
        }
    }
}

// Translator.anti_geometric(Rotor) -> Vec2
impl AntiGeometric<Rotor> for Translator {
    type Output = Vec2;
    fn anti_geometric(self, rhs: Rotor) -> Self::Output {
        // Vec2 {
        //     x: X(self.wx.0 * rhs.xy.0) + X(self.yw.0 * rhs.s.0),
        //     y: Y(self.wx.0 * rhs.s.0) + Y(self.yw.0 * rhs.xy.0),
        // }
        Vec2 {
            x: self.wx.anti_geometric(rhs.xy) + self.yw.anti_geometric(rhs.s),
            y: self.wx.anti_geometric(rhs.s) - self.yw.anti_geometric(rhs.xy),
        }
    }
}

// Translator.dot(Rotor) -> Motor
impl Dot<Rotor> for Translator {
    type Output = Motor;
    fn dot(self, rhs: Rotor) -> Self::Output {
        // Motor {
        //     s : S(self.s.0 * rhs.s.0),
        //     yw: YW(self.yw.0 * rhs.s.0),
        //     wx: WX(self.wx.0 * rhs.s.0),
        //     xy: XY(self.s.0 * rhs.xy.0),
        // }
        Motor {
            s: self.s.dot(rhs.s),
            yw: self.yw.dot(rhs.s),
            wx: self.wx.dot(rhs.s),
            xy: self.s.dot(rhs.xy),
        }
    }
}

// Translator.wedge(Rotor) -> Motor
impl Wedge<Rotor> for Translator {
    type Output = Motor;
    fn wedge(self, rhs: Rotor) -> Self::Output {
        // Motor {
        //     s : S(self.s.0 * rhs.s.0),
        //     yw: YW(self.yw.0 * rhs.s.0),
        //     wx: WX(self.wx.0 * rhs.s.0),
        //     xy: XY(self.s.0 * rhs.xy.0),
        // }
        Motor {
            s: self.s.wedge(rhs.s),
            yw: self.yw.wedge(rhs.s),
            wx: self.wx.wedge(rhs.s),
            xy: self.s.wedge(rhs.xy),
        }
    }
}

// Translator.anti_wedge(Rotor) -> Vec2
impl AntiWedge<Rotor> for Translator {
    type Output = Vec2;
    fn anti_wedge(self, rhs: Rotor) -> Self::Output {
        // Vec2 {
        //     x: X(self.wx.0 * rhs.xy.0),
        //     y: Y(self.yw.0 * rhs.xy.0),
        // }
        Vec2 {
            x: self.wx.anti_wedge(rhs.xy),
            y: -self.yw.anti_wedge(rhs.xy),
        }
    }
}

// ---------------------------------------------------------------------
// Translator OP Motor:

// Translator.geometric(Motor) -> Motor
impl Geometric<Motor> for Translator {
    type Output = Motor;
    fn geometric(self, rhs: Motor) -> Self::Output {
        // Motor {
        //     s : S(self.s.0 * rhs.s.0),
        //     yw: YW(self.s.0 * rhs.yw.0) + YW(self.wx.0 * rhs.xy.0) + YW(self.yw.0 * rhs.s.0),
        //     wx: WX(self.s.0 * rhs.wx.0) + WX(self.wx.0 * rhs.s.0) + WX(self.yw.0 * rhs.xy.0),
        //     xy: XY(self.s.0 * rhs.xy.0),
        // }
        Motor {
            s: self.s.geometric(rhs.s),
            yw: self.s.geometric(rhs.yw) - self.wx.geometric(rhs.xy) + self.yw.geometric(rhs.s),
            wx: self.s.geometric(rhs.wx) + self.wx.geometric(rhs.s) + self.yw.geometric(rhs.xy),
            xy: self.s.geometric(rhs.xy),
        }
    }
}

// Omitted: Translator anti_geometric Motor = self.s !* rhs.wx + self.s !* rhs.yw + self.wx !* rhs.s + self.wx !* rhs.wx + self.wx !* rhs.xy + self.wx !* rhs.yw + self.yw !* rhs.s + self.yw !* rhs.wx + self.yw !* rhs.xy + self.yw !* rhs.yw

// Translator.dot(Motor) -> Motor
impl Dot<Motor> for Translator {
    type Output = Motor;
    fn dot(self, rhs: Motor) -> Self::Output {
        // Motor {
        //     s : S(self.s.0 * rhs.s.0),
        //     yw: YW(self.s.0 * rhs.yw.0) + YW(self.yw.0 * rhs.s.0),
        //     wx: WX(self.s.0 * rhs.wx.0) + WX(self.wx.0 * rhs.s.0),
        //     xy: XY(self.s.0 * rhs.xy.0),
        // }
        Motor {
            s: self.s.dot(rhs.s),
            yw: self.s.dot(rhs.yw) + self.yw.dot(rhs.s),
            wx: self.s.dot(rhs.wx) + self.wx.dot(rhs.s),
            xy: self.s.dot(rhs.xy),
        }
    }
}

// Translator.wedge(Motor) -> Motor
impl Wedge<Motor> for Translator {
    type Output = Motor;
    fn wedge(self, rhs: Motor) -> Self::Output {
        // Motor {
        //     s : S(self.s.0 * rhs.s.0),
        //     yw: YW(self.s.0 * rhs.yw.0) + YW(self.yw.0 * rhs.s.0),
        //     wx: WX(self.s.0 * rhs.wx.0) + WX(self.wx.0 * rhs.s.0),
        //     xy: XY(self.s.0 * rhs.xy.0),
        // }
        Motor {
            s: self.s.wedge(rhs.s),
            yw: self.s.wedge(rhs.yw) + self.yw.wedge(rhs.s),
            wx: self.s.wedge(rhs.wx) + self.wx.wedge(rhs.s),
            xy: self.s.wedge(rhs.xy),
        }
    }
}

// Translator.anti_wedge(Motor) -> Vec3
impl AntiWedge<Motor> for Translator {
    type Output = Vec3;
    fn anti_wedge(self, rhs: Motor) -> Self::Output {
        // Vec3 {
        //     x: X(self.wx.0 * rhs.xy.0),
        //     y: Y(self.yw.0 * rhs.xy.0),
        //     w: W(self.wx.0 * rhs.yw.0) + W(self.yw.0 * rhs.wx.0),
        // }
        Vec3 {
            x: self.wx.anti_wedge(rhs.xy),
            y: -self.yw.anti_wedge(rhs.xy),
            w: -self.wx.anti_wedge(rhs.yw) + self.yw.anti_wedge(rhs.wx),
        }
    }
}

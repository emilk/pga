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

// Omitted: Motor geometric Vec2 = self.s * rhs.x + self.s * rhs.y + self.wx * rhs.x + self.wx * rhs.y + self.xy * rhs.x + self.xy * rhs.y + self.yw * rhs.x + self.yw * rhs.y  (unnamed type)

// Motor.anti_geometric(Vec2) -> Rotor
impl AntiGeometric<Vec2> for Motor {
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

// Motor.dot(Vec2) -> Vec3
impl Dot<Vec2> for Motor {
    type Output = Vec3;
    fn dot(self, rhs: Vec2) -> Self::Output {
        // Vec3 {
        //     x: X(self.s.0 * rhs.x.0) + X(self.xy.0 * rhs.y.0),
        //     y: Y(self.s.0 * rhs.y.0) + Y(self.xy.0 * rhs.x.0),
        //     w: W(self.wx.0 * rhs.x.0) + W(self.yw.0 * rhs.y.0),
        // }
        Vec3 {
            x: self.s.dot(rhs.x) + self.xy.dot(rhs.y),
            y: self.s.dot(rhs.y) - self.xy.dot(rhs.x),
            w: self.wx.dot(rhs.x) - self.yw.dot(rhs.y),
        }
    }
}

// Omitted: Motor wedge Vec2 = self.s ^ rhs.x + self.s ^ rhs.y + self.wx ^ rhs.y + self.yw ^ rhs.x  (unnamed type)

// Motor.anti_wedge(Vec2) -> S
impl AntiWedge<Vec2> for Motor {
    type Output = S;
    fn anti_wedge(self, rhs: Vec2) -> Self::Output {
        // S(self.wx.0 * rhs.y.0) + S(self.yw.0 * rhs.x.0)
        self.wx.anti_wedge(rhs.y) + self.yw.anti_wedge(rhs.x)
    }
}

// ---------------------------------------------------------------------
// Motor OP Vec3:

// Omitted: Motor geometric Vec3 = self.s * rhs.w + self.s * rhs.x + self.s * rhs.y + self.wx * rhs.x + self.wx * rhs.y + self.xy * rhs.w + self.xy * rhs.x + self.xy * rhs.y + self.yw * rhs.x + self.yw * rhs.y  (unnamed type)

// Motor.anti_geometric(Vec3) -> Motor
impl AntiGeometric<Vec3> for Motor {
    type Output = Motor;
    fn anti_geometric(self, rhs: Vec3) -> Self::Output {
        // Motor {
        //     s : S(self.wx.0 * rhs.y.0) + S(self.xy.0 * rhs.w.0) + S(self.yw.0 * rhs.x.0),
        //     yw: YW(self.wx.0 * rhs.w.0),
        //     wx: WX(self.yw.0 * rhs.w.0),
        //     xy: XY(self.s.0 * rhs.w.0) + XY(self.wx.0 * rhs.x.0) + XY(self.yw.0 * rhs.y.0),
        // }
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
        // Vec3 {
        //     x: X(self.s.0 * rhs.x.0) + X(self.xy.0 * rhs.y.0),
        //     y: Y(self.s.0 * rhs.y.0) + Y(self.xy.0 * rhs.x.0),
        //     w: W(self.s.0 * rhs.w.0) + W(self.wx.0 * rhs.x.0) + W(self.yw.0 * rhs.y.0),
        // }
        Vec3 {
            x: self.s.dot(rhs.x) + self.xy.dot(rhs.y),
            y: self.s.dot(rhs.y) - self.xy.dot(rhs.x),
            w: self.s.dot(rhs.w) + self.wx.dot(rhs.x) - self.yw.dot(rhs.y),
        }
    }
}

// Omitted: Motor wedge Vec3 = self.s ^ rhs.w + self.s ^ rhs.x + self.s ^ rhs.y + self.wx ^ rhs.y + self.xy ^ rhs.w + self.yw ^ rhs.x  (unnamed type)

// Motor.anti_wedge(Vec3) -> S
impl AntiWedge<Vec3> for Motor {
    type Output = S;
    fn anti_wedge(self, rhs: Vec3) -> Self::Output {
        // S(self.wx.0 * rhs.y.0) + S(self.xy.0 * rhs.w.0) + S(self.yw.0 * rhs.x.0)
        self.wx.anti_wedge(rhs.y) + self.xy.anti_wedge(rhs.w) + self.yw.anti_wedge(rhs.x)
    }
}

// ---------------------------------------------------------------------
// Motor OP Line:

// Motor.geometric(Line) -> Motor
impl Geometric<Line> for Motor {
    type Output = Motor;
    fn geometric(self, rhs: Line) -> Self::Output {
        // Motor {
        //     s : S(self.xy.0 * rhs.m.0),
        //     yw: YW(self.s.0 * rhs.dx.0) + YW(self.wx.0 * rhs.m.0) + YW(self.xy.0 * rhs.dy.0),
        //     wx: WX(self.s.0 * rhs.dy.0) + WX(self.xy.0 * rhs.dx.0) + WX(self.yw.0 * rhs.m.0),
        //     xy: XY(self.s.0 * rhs.m.0),
        // }
        Motor {
            s: -self.xy.geometric(rhs.m),
            yw: self.s.geometric(rhs.dx) - self.wx.geometric(rhs.m) + self.xy.geometric(rhs.dy),
            wx: self.s.geometric(rhs.dy) - self.xy.geometric(rhs.dx) + self.yw.geometric(rhs.m),
            xy: self.s.geometric(rhs.m),
        }
    }
}

// Omitted: Motor anti_geometric Line = self.s !* rhs.dx + self.s !* rhs.dy + self.wx !* rhs.dx + self.wx !* rhs.dy + self.wx !* rhs.m + self.xy !* rhs.dx + self.xy !* rhs.dy + self.yw !* rhs.dx + self.yw !* rhs.dy + self.yw !* rhs.m  (unnamed type)

// Motor.dot(Line) -> Motor
impl Dot<Line> for Motor {
    type Output = Motor;
    fn dot(self, rhs: Line) -> Self::Output {
        // Motor {
        //     s : S(self.xy.0 * rhs.m.0),
        //     yw: YW(self.s.0 * rhs.dx.0),
        //     wx: WX(self.s.0 * rhs.dy.0),
        //     xy: XY(self.s.0 * rhs.m.0),
        // }
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

// Motor.anti_wedge(Line) -> Vec3
impl AntiWedge<Line> for Motor {
    type Output = Vec3;
    fn anti_wedge(self, rhs: Line) -> Self::Output {
        // Vec3 {
        //     x: X(self.wx.0 * rhs.m.0) + X(self.xy.0 * rhs.dy.0),
        //     y: Y(self.xy.0 * rhs.dx.0) + Y(self.yw.0 * rhs.m.0),
        //     w: W(self.wx.0 * rhs.dx.0) + W(self.yw.0 * rhs.dy.0),
        // }
        Vec3 {
            x: self.wx.anti_wedge(rhs.m) - self.xy.anti_wedge(rhs.dy),
            y: self.xy.anti_wedge(rhs.dx) - self.yw.anti_wedge(rhs.m),
            w: -self.wx.anti_wedge(rhs.dx) + self.yw.anti_wedge(rhs.dy),
        }
    }
}

// ---------------------------------------------------------------------
// Motor OP Rotor:

// Motor.geometric(Rotor) -> Motor
impl Geometric<Rotor> for Motor {
    type Output = Motor;
    fn geometric(self, rhs: Rotor) -> Self::Output {
        // Motor {
        //     s : S(self.s.0 * rhs.s.0) + S(self.xy.0 * rhs.xy.0),
        //     yw: YW(self.wx.0 * rhs.xy.0) + YW(self.yw.0 * rhs.s.0),
        //     wx: WX(self.wx.0 * rhs.s.0) + WX(self.yw.0 * rhs.xy.0),
        //     xy: XY(self.s.0 * rhs.xy.0) + XY(self.xy.0 * rhs.s.0),
        // }
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

// Motor.dot(Rotor) -> Motor
impl Dot<Rotor> for Motor {
    type Output = Motor;
    fn dot(self, rhs: Rotor) -> Self::Output {
        // Motor {
        //     s : S(self.s.0 * rhs.s.0) + S(self.xy.0 * rhs.xy.0),
        //     yw: YW(self.yw.0 * rhs.s.0),
        //     wx: WX(self.wx.0 * rhs.s.0),
        //     xy: XY(self.s.0 * rhs.xy.0) + XY(self.xy.0 * rhs.s.0),
        // }
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
        // Motor {
        //     s : S(self.s.0 * rhs.s.0),
        //     yw: YW(self.yw.0 * rhs.s.0),
        //     wx: WX(self.wx.0 * rhs.s.0),
        //     xy: XY(self.s.0 * rhs.xy.0) + XY(self.xy.0 * rhs.s.0),
        // }
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
// Motor OP Motor:

// Motor.geometric(Motor) -> Motor
impl Geometric<Motor> for Motor {
    type Output = Motor;
    fn geometric(self, rhs: Motor) -> Self::Output {
        // Motor {
        //     s : S(self.s.0 * rhs.s.0) + S(self.xy.0 * rhs.xy.0),
        //     yw: YW(self.s.0 * rhs.yw.0) + YW(self.wx.0 * rhs.xy.0) + YW(self.xy.0 * rhs.wx.0) + YW(self.yw.0 * rhs.s.0),
        //     wx: WX(self.s.0 * rhs.wx.0) + WX(self.wx.0 * rhs.s.0) + WX(self.xy.0 * rhs.yw.0) + WX(self.yw.0 * rhs.xy.0),
        //     xy: XY(self.s.0 * rhs.xy.0) + XY(self.xy.0 * rhs.s.0),
        // }
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

// Omitted: Motor anti_geometric Motor = self.s !* rhs.wx + self.s !* rhs.yw + self.wx !* rhs.s + self.wx !* rhs.wx + self.wx !* rhs.xy + self.wx !* rhs.yw + self.xy !* rhs.wx + self.xy !* rhs.yw + self.yw !* rhs.s + self.yw !* rhs.wx + self.yw !* rhs.xy + self.yw !* rhs.yw  (unnamed type)

// Motor.dot(Motor) -> Motor
impl Dot<Motor> for Motor {
    type Output = Motor;
    fn dot(self, rhs: Motor) -> Self::Output {
        // Motor {
        //     s : S(self.s.0 * rhs.s.0) + S(self.xy.0 * rhs.xy.0),
        //     yw: YW(self.s.0 * rhs.yw.0) + YW(self.yw.0 * rhs.s.0),
        //     wx: WX(self.s.0 * rhs.wx.0) + WX(self.wx.0 * rhs.s.0),
        //     xy: XY(self.s.0 * rhs.xy.0) + XY(self.xy.0 * rhs.s.0),
        // }
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
        // Motor {
        //     s : S(self.s.0 * rhs.s.0),
        //     yw: YW(self.s.0 * rhs.yw.0) + YW(self.yw.0 * rhs.s.0),
        //     wx: WX(self.s.0 * rhs.wx.0) + WX(self.wx.0 * rhs.s.0),
        //     xy: XY(self.s.0 * rhs.xy.0) + XY(self.xy.0 * rhs.s.0),
        // }
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
        // Vec3 {
        //     x: X(self.wx.0 * rhs.xy.0) + X(self.xy.0 * rhs.wx.0),
        //     y: Y(self.xy.0 * rhs.yw.0) + Y(self.yw.0 * rhs.xy.0),
        //     w: W(self.wx.0 * rhs.yw.0) + W(self.yw.0 * rhs.wx.0),
        // }
        Vec3 {
            x: self.wx.anti_wedge(rhs.xy) - self.xy.anti_wedge(rhs.wx),
            y: self.xy.anti_wedge(rhs.yw) - self.yw.anti_wedge(rhs.xy),
            w: -self.wx.anti_wedge(rhs.yw) + self.yw.anti_wedge(rhs.wx),
        }
    }
}

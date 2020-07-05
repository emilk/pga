//! # Rotor
//!
//! ## Operations
//! ```text
//! Rotor.geometric(Rotor) -> Rotor
//! Rotor.dot(Rotor) -> Rotor
//! Rotor.wedge(Rotor) -> Rotor
//! Rotor.geometric(Vec2) -> Vec2
//! Vec2.geometric(Rotor) -> Vec2
//! Rotor.dot(Vec2) -> Vec2
//! Vec2.dot(Rotor) -> Vec2
//! Rotor.wedge(Vec2) -> Vec2
//! Vec2.wedge(Rotor) -> Vec2
//! Rotor.anti_geometric(Vec3) -> Rotor
//! Vec3.anti_geometric(Rotor) -> Rotor
//! Rotor.dot(Vec3) -> Vec3
//! Vec3.dot(Rotor) -> Vec3
//! Rotor.anti_wedge(Vec3) -> S
//! Vec3.anti_wedge(Rotor) -> S
//! Rotor.geometric(Line) -> Motor
//! Line.geometric(Rotor) -> Motor
//! Rotor.anti_geometric(Line) -> Vec2
//! Line.anti_geometric(Rotor) -> Vec2
//! Rotor.dot(Line) -> Motor
//! Line.dot(Rotor) -> Motor
//! Rotor.wedge(Line) -> Line
//! Line.wedge(Rotor) -> Line
//! Rotor.anti_wedge(Line) -> Vec2
//! Line.anti_wedge(Rotor) -> Vec2
//! Rotor.geometric(Translator) -> Motor
//! Translator.geometric(Rotor) -> Motor
//! Rotor.anti_geometric(Translator) -> Vec2
//! Translator.anti_geometric(Rotor) -> Vec2
//! Rotor.dot(Translator) -> Motor
//! Translator.dot(Rotor) -> Motor
//! Rotor.wedge(Translator) -> Motor
//! Translator.wedge(Rotor) -> Motor
//! Rotor.anti_wedge(Translator) -> Vec2
//! Translator.anti_wedge(Rotor) -> Vec2
//! Rotor.geometric(Motor) -> Motor
//! Motor.geometric(Rotor) -> Motor
//! Rotor.anti_geometric(Motor) -> Vec2
//! Motor.anti_geometric(Rotor) -> Vec2
//! Rotor.dot(Motor) -> Motor
//! Motor.dot(Rotor) -> Motor
//! Rotor.wedge(Motor) -> Motor
//! Motor.wedge(Rotor) -> Motor
//! Rotor.anti_wedge(Motor) -> Vec2
//! Motor.anti_wedge(Rotor) -> Vec2
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
pub struct Rotor {
    pub s: S,
    pub xy: XY,
}

// ---------------------------------------------------------------------
// Omitted: Rotor.rcompl() -> self.s.rcompl() + self.xy.rcompl()
// Omitted: Rotor.lcompl() -> self.s.lcompl() + self.xy.lcompl()

impl Reverse for Rotor {
    fn rev(self) -> Self {
        Rotor {
            s: self.s,
            xy: -self.xy,
        }
    }
}

impl AntiReverse for Rotor {
    fn arev(self) -> Self {
        Rotor {
            s: -self.s,
            xy: self.xy,
        }
    }
}

// ---------------------------------------------------------------------
// Rotor OP Vec2:

// Rotor.geometric(Vec2) -> Vec2
impl Geometric<Vec2> for Rotor {
    type Output = Vec2;
    fn geometric(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.s.geometric(rhs.x) + self.xy.geometric(rhs.y),
            y: self.s.geometric(rhs.y) - self.xy.geometric(rhs.x),
        }
    }
}

// Omitted: Rotor anti_geometric Vec2 = 0

// Rotor.dot(Vec2) -> Vec2
impl Dot<Vec2> for Rotor {
    type Output = Vec2;
    fn dot(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.s.dot(rhs.x) + self.xy.dot(rhs.y),
            y: self.s.dot(rhs.y) - self.xy.dot(rhs.x),
        }
    }
}

// Rotor.wedge(Vec2) -> Vec2
impl Wedge<Vec2> for Rotor {
    type Output = Vec2;
    fn wedge(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.s.wedge(rhs.x),
            y: self.s.wedge(rhs.y),
        }
    }
}

// Omitted: Rotor anti_wedge Vec2 = 0

// ---------------------------------------------------------------------
// Rotor OP Vec3:

// Omitted: Rotor geometric Vec3 = self.s.geometric(rhs.w) + self.s.geometric(rhs.x) + self.s.geometric(rhs.y) + self.xy.geometric(rhs.w) + self.xy.geometric(rhs.x) + self.xy.geometric(rhs.y)

// Rotor.anti_geometric(Vec3) -> Rotor
impl AntiGeometric<Vec3> for Rotor {
    type Output = Rotor;
    fn anti_geometric(self, rhs: Vec3) -> Self::Output {
        Rotor {
            s: self.xy.anti_geometric(rhs.w),
            xy: -self.s.anti_geometric(rhs.w),
        }
    }
}

// Rotor.dot(Vec3) -> Vec3
impl Dot<Vec3> for Rotor {
    type Output = Vec3;
    fn dot(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.s.dot(rhs.x) + self.xy.dot(rhs.y),
            y: self.s.dot(rhs.y) - self.xy.dot(rhs.x),
            w: self.s.dot(rhs.w),
        }
    }
}

// Omitted: Rotor wedge Vec3 = self.s.wedge(rhs.w) + self.s.wedge(rhs.x) + self.s.wedge(rhs.y) + self.xy.wedge(rhs.w)

// Rotor.anti_wedge(Vec3) -> S
impl AntiWedge<Vec3> for Rotor {
    type Output = S;
    fn anti_wedge(self, rhs: Vec3) -> Self::Output {
        self.xy.anti_wedge(rhs.w)
    }
}

// ---------------------------------------------------------------------
// Rotor OP Line:

// Rotor.geometric(Line) -> Motor
impl Geometric<Line> for Rotor {
    type Output = Motor;
    fn geometric(self, rhs: Line) -> Self::Output {
        Motor {
            s: -self.xy.geometric(rhs.m),
            yw: self.s.geometric(rhs.dx) + self.xy.geometric(rhs.dy),
            wx: self.s.geometric(rhs.dy) - self.xy.geometric(rhs.dx),
            xy: self.s.geometric(rhs.m),
        }
    }
}

// Rotor.anti_geometric(Line) -> Vec2
impl AntiGeometric<Line> for Rotor {
    type Output = Vec2;
    fn anti_geometric(self, rhs: Line) -> Self::Output {
        Vec2 {
            x: self.s.anti_geometric(rhs.dx) - self.xy.anti_geometric(rhs.dy),
            y: self.s.anti_geometric(rhs.dy) + self.xy.anti_geometric(rhs.dx),
        }
    }
}

// Rotor.dot(Line) -> Motor
impl Dot<Line> for Rotor {
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

// Rotor.wedge(Line) -> Line
impl Wedge<Line> for Rotor {
    type Output = Line;
    fn wedge(self, rhs: Line) -> Self::Output {
        Line {
            dx: self.s.wedge(rhs.dx),
            dy: self.s.wedge(rhs.dy),
            m: self.s.wedge(rhs.m),
        }
    }
}

// Rotor.anti_wedge(Line) -> Vec2
impl AntiWedge<Line> for Rotor {
    type Output = Vec2;
    fn anti_wedge(self, rhs: Line) -> Self::Output {
        Vec2 {
            x: -self.xy.anti_wedge(rhs.dy),
            y: self.xy.anti_wedge(rhs.dx),
        }
    }
}

// ---------------------------------------------------------------------
// Rotor OP Translator:

// Rotor.geometric(Translator) -> Motor
impl Geometric<Translator> for Rotor {
    type Output = Motor;
    fn geometric(self, rhs: Translator) -> Self::Output {
        Motor {
            s: self.s.geometric(rhs.s),
            yw: self.s.geometric(rhs.yw) + self.xy.geometric(rhs.wx),
            wx: self.s.geometric(rhs.wx) - self.xy.geometric(rhs.yw),
            xy: self.xy.geometric(rhs.s),
        }
    }
}

// Rotor.anti_geometric(Translator) -> Vec2
impl AntiGeometric<Translator> for Rotor {
    type Output = Vec2;
    fn anti_geometric(self, rhs: Translator) -> Self::Output {
        Vec2 {
            x: self.s.anti_geometric(rhs.yw) - self.xy.anti_geometric(rhs.wx),
            y: self.s.anti_geometric(rhs.wx) + self.xy.anti_geometric(rhs.yw),
        }
    }
}

// Rotor.dot(Translator) -> Motor
impl Dot<Translator> for Rotor {
    type Output = Motor;
    fn dot(self, rhs: Translator) -> Self::Output {
        Motor {
            s: self.s.dot(rhs.s),
            yw: self.s.dot(rhs.yw),
            wx: self.s.dot(rhs.wx),
            xy: self.xy.dot(rhs.s),
        }
    }
}

// Rotor.wedge(Translator) -> Motor
impl Wedge<Translator> for Rotor {
    type Output = Motor;
    fn wedge(self, rhs: Translator) -> Self::Output {
        Motor {
            s: self.s.wedge(rhs.s),
            yw: self.s.wedge(rhs.yw),
            wx: self.s.wedge(rhs.wx),
            xy: self.xy.wedge(rhs.s),
        }
    }
}

// Rotor.anti_wedge(Translator) -> Vec2
impl AntiWedge<Translator> for Rotor {
    type Output = Vec2;
    fn anti_wedge(self, rhs: Translator) -> Self::Output {
        Vec2 {
            x: -self.xy.anti_wedge(rhs.wx),
            y: self.xy.anti_wedge(rhs.yw),
        }
    }
}

// ---------------------------------------------------------------------
// Rotor OP Rotor:

// Rotor.geometric(Rotor) -> Rotor
impl Geometric<Rotor> for Rotor {
    type Output = Rotor;
    fn geometric(self, rhs: Rotor) -> Self::Output {
        Rotor {
            s: self.s.geometric(rhs.s) - self.xy.geometric(rhs.xy),
            xy: self.s.geometric(rhs.xy) + self.xy.geometric(rhs.s),
        }
    }
}

// Omitted: Rotor anti_geometric Rotor = 0

// Rotor.dot(Rotor) -> Rotor
impl Dot<Rotor> for Rotor {
    type Output = Rotor;
    fn dot(self, rhs: Rotor) -> Self::Output {
        Rotor {
            s: self.s.dot(rhs.s) - self.xy.dot(rhs.xy),
            xy: self.s.dot(rhs.xy) + self.xy.dot(rhs.s),
        }
    }
}

// Rotor.wedge(Rotor) -> Rotor
impl Wedge<Rotor> for Rotor {
    type Output = Rotor;
    fn wedge(self, rhs: Rotor) -> Self::Output {
        Rotor {
            s: self.s.wedge(rhs.s),
            xy: self.s.wedge(rhs.xy) + self.xy.wedge(rhs.s),
        }
    }
}

// Omitted: Rotor anti_wedge Rotor = 0

// ---------------------------------------------------------------------
// Rotor OP Motor:

// Rotor.geometric(Motor) -> Motor
impl Geometric<Motor> for Rotor {
    type Output = Motor;
    fn geometric(self, rhs: Motor) -> Self::Output {
        Motor {
            s: self.s.geometric(rhs.s) - self.xy.geometric(rhs.xy),
            yw: self.s.geometric(rhs.yw) + self.xy.geometric(rhs.wx),
            wx: self.s.geometric(rhs.wx) - self.xy.geometric(rhs.yw),
            xy: self.s.geometric(rhs.xy) + self.xy.geometric(rhs.s),
        }
    }
}

// Rotor.anti_geometric(Motor) -> Vec2
impl AntiGeometric<Motor> for Rotor {
    type Output = Vec2;
    fn anti_geometric(self, rhs: Motor) -> Self::Output {
        Vec2 {
            x: self.s.anti_geometric(rhs.yw) - self.xy.anti_geometric(rhs.wx),
            y: self.s.anti_geometric(rhs.wx) + self.xy.anti_geometric(rhs.yw),
        }
    }
}

// Rotor.dot(Motor) -> Motor
impl Dot<Motor> for Rotor {
    type Output = Motor;
    fn dot(self, rhs: Motor) -> Self::Output {
        Motor {
            s: self.s.dot(rhs.s) - self.xy.dot(rhs.xy),
            yw: self.s.dot(rhs.yw),
            wx: self.s.dot(rhs.wx),
            xy: self.s.dot(rhs.xy) + self.xy.dot(rhs.s),
        }
    }
}

// Rotor.wedge(Motor) -> Motor
impl Wedge<Motor> for Rotor {
    type Output = Motor;
    fn wedge(self, rhs: Motor) -> Self::Output {
        Motor {
            s: self.s.wedge(rhs.s),
            yw: self.s.wedge(rhs.yw),
            wx: self.s.wedge(rhs.wx),
            xy: self.s.wedge(rhs.xy) + self.xy.wedge(rhs.s),
        }
    }
}

// Rotor.anti_wedge(Motor) -> Vec2
impl AntiWedge<Motor> for Rotor {
    type Output = Vec2;
    fn anti_wedge(self, rhs: Motor) -> Self::Output {
        Vec2 {
            x: -self.xy.anti_wedge(rhs.wx),
            y: self.xy.anti_wedge(rhs.yw),
        }
    }
}

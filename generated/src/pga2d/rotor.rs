//! # Rotor
//!
//! ## Operations
//! ```text
//! Rotor.geometric(Rotor) -> Rotor
//! Rotor.dot(Rotor) -> Rotor
//! Rotor.wedge(Rotor) -> Rotor
//! Rotor.geometric(Dir) -> Dir
//! Dir.geometric(Rotor) -> Dir
//! Rotor.dot(Dir) -> Dir
//! Dir.dot(Rotor) -> Dir
//! Rotor.wedge(Dir) -> Dir
//! Dir.wedge(Rotor) -> Dir
//! Rotor.anti_geometric(Point) -> Rotor
//! Point.anti_geometric(Rotor) -> Rotor
//! Rotor.dot(Point) -> Point
//! Point.dot(Rotor) -> Point
//! Rotor.anti_wedge(Point) -> S
//! Point.anti_wedge(Rotor) -> S
//! Rotor.geometric(Line) -> Motor
//! Line.geometric(Rotor) -> Motor
//! Rotor.anti_geometric(Line) -> Dir
//! Line.anti_geometric(Rotor) -> Dir
//! Rotor.dot(Line) -> Motor
//! Line.dot(Rotor) -> Motor
//! Rotor.wedge(Line) -> Line
//! Line.wedge(Rotor) -> Line
//! Rotor.anti_wedge(Line) -> Dir
//! Line.anti_wedge(Rotor) -> Dir
//! Rotor.geometric(Translator) -> Motor
//! Translator.geometric(Rotor) -> Motor
//! Rotor.anti_geometric(Translator) -> Dir
//! Translator.anti_geometric(Rotor) -> Dir
//! Rotor.dot(Translator) -> Motor
//! Translator.dot(Rotor) -> Motor
//! Rotor.wedge(Translator) -> Motor
//! Translator.wedge(Rotor) -> Motor
//! Rotor.anti_wedge(Translator) -> Dir
//! Translator.anti_wedge(Rotor) -> Dir
//! Rotor.geometric(Motor) -> Motor
//! Motor.geometric(Rotor) -> Motor
//! Rotor.anti_geometric(Motor) -> Dir
//! Motor.anti_geometric(Rotor) -> Dir
//! Rotor.dot(Motor) -> Motor
//! Motor.dot(Rotor) -> Motor
//! Rotor.wedge(Motor) -> Motor
//! Motor.wedge(Rotor) -> Motor
//! Rotor.anti_wedge(Motor) -> Dir
//! Motor.anti_wedge(Rotor) -> Dir
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
            s: self.s.rev(),
            xy: -self.xy.rev(),
        }
    }
}

impl AntiReverse for Rotor {
    fn arev(self) -> Self {
        Rotor {
            s: -self.s.arev(),
            xy: self.xy.arev(),
        }
    }
}

// ---------------------------------------------------------------------
// Rotor OP Dir:

// Rotor.geometric(Dir) -> Dir
impl Geometric<Dir> for Rotor {
    type Output = Dir;
    fn geometric(self, rhs: Dir) -> Self::Output {
        Dir {
            x: self.s.geometric(rhs.x) + self.xy.geometric(rhs.y),
            y: self.s.geometric(rhs.y) - self.xy.geometric(rhs.x),
        }
    }
}

// Omitted: Rotor anti_geometric Dir = 0

// Rotor.dot(Dir) -> Dir
impl Dot<Dir> for Rotor {
    type Output = Dir;
    fn dot(self, rhs: Dir) -> Self::Output {
        Dir {
            x: self.s.dot(rhs.x) + self.xy.dot(rhs.y),
            y: self.s.dot(rhs.y) - self.xy.dot(rhs.x),
        }
    }
}

// Rotor.wedge(Dir) -> Dir
impl Wedge<Dir> for Rotor {
    type Output = Dir;
    fn wedge(self, rhs: Dir) -> Self::Output {
        Dir {
            x: self.s.wedge(rhs.x),
            y: self.s.wedge(rhs.y),
        }
    }
}

// Omitted: Rotor anti_wedge Dir = 0

// ---------------------------------------------------------------------
// Rotor OP Point:

// Omitted: Rotor geometric Point = self.s.geometric(rhs.w) + self.s.geometric(rhs.x) + self.s.geometric(rhs.y) + self.xy.geometric(rhs.w) + self.xy.geometric(rhs.x) + self.xy.geometric(rhs.y)

// Rotor.anti_geometric(Point) -> Rotor
impl AntiGeometric<Point> for Rotor {
    type Output = Rotor;
    fn anti_geometric(self, rhs: Point) -> Self::Output {
        Rotor {
            s: self.xy.anti_geometric(rhs.w),
            xy: -self.s.anti_geometric(rhs.w),
        }
    }
}

// Rotor.dot(Point) -> Point
impl Dot<Point> for Rotor {
    type Output = Point;
    fn dot(self, rhs: Point) -> Self::Output {
        Point {
            x: self.s.dot(rhs.x) + self.xy.dot(rhs.y),
            y: self.s.dot(rhs.y) - self.xy.dot(rhs.x),
            w: self.s.dot(rhs.w),
        }
    }
}

// Omitted: Rotor wedge Point = self.s.wedge(rhs.w) + self.s.wedge(rhs.x) + self.s.wedge(rhs.y) + self.xy.wedge(rhs.w)

// Rotor.anti_wedge(Point) -> S
impl AntiWedge<Point> for Rotor {
    type Output = S;
    fn anti_wedge(self, rhs: Point) -> Self::Output {
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
            s: -self.xy.geometric(rhs.xy),
            yw: self.s.geometric(rhs.yw) + self.xy.geometric(rhs.wx),
            wx: -self.s.geometric(rhs.wx) + self.xy.geometric(rhs.yw),
            xy: self.s.geometric(rhs.xy),
        }
    }
}

// Rotor.anti_geometric(Line) -> Dir
impl AntiGeometric<Line> for Rotor {
    type Output = Dir;
    fn anti_geometric(self, rhs: Line) -> Self::Output {
        Dir {
            x: self.s.anti_geometric(rhs.yw) - self.xy.anti_geometric(rhs.wx),
            y: self.s.anti_geometric(rhs.wx) + self.xy.anti_geometric(rhs.yw),
        }
    }
}

// Rotor.dot(Line) -> Motor
impl Dot<Line> for Rotor {
    type Output = Motor;
    fn dot(self, rhs: Line) -> Self::Output {
        Motor {
            s: -self.xy.dot(rhs.xy),
            yw: self.s.dot(rhs.yw),
            wx: -self.s.dot(rhs.wx),
            xy: self.s.dot(rhs.xy),
        }
    }
}

// Rotor.wedge(Line) -> Line
impl Wedge<Line> for Rotor {
    type Output = Line;
    fn wedge(self, rhs: Line) -> Self::Output {
        Line {
            yw: self.s.wedge(rhs.yw),
            wx: -self.s.wedge(rhs.wx),
            xy: self.s.wedge(rhs.xy),
        }
    }
}

// Rotor.anti_wedge(Line) -> Dir
impl AntiWedge<Line> for Rotor {
    type Output = Dir;
    fn anti_wedge(self, rhs: Line) -> Self::Output {
        Dir {
            x: -self.xy.anti_wedge(rhs.wx),
            y: self.xy.anti_wedge(rhs.yw),
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
            wx: -self.s.geometric(rhs.wx) + self.xy.geometric(rhs.yw),
            xy: self.xy.geometric(rhs.s),
        }
    }
}

// Rotor.anti_geometric(Translator) -> Dir
impl AntiGeometric<Translator> for Rotor {
    type Output = Dir;
    fn anti_geometric(self, rhs: Translator) -> Self::Output {
        Dir {
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
            wx: -self.s.dot(rhs.wx),
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
            wx: -self.s.wedge(rhs.wx),
            xy: self.xy.wedge(rhs.s),
        }
    }
}

// Rotor.anti_wedge(Translator) -> Dir
impl AntiWedge<Translator> for Rotor {
    type Output = Dir;
    fn anti_wedge(self, rhs: Translator) -> Self::Output {
        Dir {
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
            wx: -self.s.geometric(rhs.wx) + self.xy.geometric(rhs.yw),
            xy: self.s.geometric(rhs.xy) + self.xy.geometric(rhs.s),
        }
    }
}

// Rotor.anti_geometric(Motor) -> Dir
impl AntiGeometric<Motor> for Rotor {
    type Output = Dir;
    fn anti_geometric(self, rhs: Motor) -> Self::Output {
        Dir {
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
            wx: -self.s.dot(rhs.wx),
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
            wx: -self.s.wedge(rhs.wx),
            xy: self.s.wedge(rhs.xy) + self.xy.wedge(rhs.s),
        }
    }
}

// Rotor.anti_wedge(Motor) -> Dir
impl AntiWedge<Motor> for Rotor {
    type Output = Dir;
    fn anti_wedge(self, rhs: Motor) -> Self::Output {
        Dir {
            x: -self.xy.anti_wedge(rhs.wx),
            y: self.xy.anti_wedge(rhs.yw),
        }
    }
}

//! # Dir
//!
//! ## Operations
//! ```text
//! Dir.geometric(Dir) -> Rotor
//! Dir.dot(Dir) -> S
//! Dir.geometric(Point) -> Motor
//! Point.geometric(Dir) -> Motor
//! Dir.anti_geometric(Point) -> Dir
//! Point.anti_geometric(Dir) -> Dir
//! Dir.dot(Point) -> S
//! Point.dot(Dir) -> S
//! Dir.wedge(Point) -> Line
//! Point.wedge(Dir) -> Line
//! Dir.anti_geometric(Line) -> Rotor
//! Line.anti_geometric(Dir) -> Rotor
//! Dir.dot(Line) -> Point
//! Line.dot(Dir) -> Point
//! Dir.wedge(Line) -> XYW
//! Line.wedge(Dir) -> XYW
//! Dir.anti_wedge(Line) -> S
//! Line.anti_wedge(Dir) -> S
//! Dir.anti_geometric(Translator) -> Rotor
//! Translator.anti_geometric(Dir) -> Rotor
//! Dir.dot(Translator) -> Point
//! Translator.dot(Dir) -> Point
//! Dir.anti_wedge(Translator) -> S
//! Translator.anti_wedge(Dir) -> S
//! Dir.geometric(Rotor) -> Dir
//! Rotor.geometric(Dir) -> Dir
//! Dir.dot(Rotor) -> Dir
//! Rotor.dot(Dir) -> Dir
//! Dir.wedge(Rotor) -> Dir
//! Rotor.wedge(Dir) -> Dir
//! Dir.anti_geometric(Motor) -> Rotor
//! Motor.anti_geometric(Dir) -> Rotor
//! Dir.dot(Motor) -> Point
//! Motor.dot(Dir) -> Point
//! Dir.anti_wedge(Motor) -> S
//! Motor.anti_wedge(Dir) -> S
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
pub struct Dir {
    pub x: X,
    pub y: Y,
}

// ---------------------------------------------------------------------

impl RCompl for Dir {
    type Output = Line;
    fn rcompl(self) -> Self::Output {
        Line {
            yw: self.x.rcompl(),
            wx: -self.y.rcompl(),
            xy: Default::default(),
        }
    }
}

impl LCompl for Dir {
    type Output = Line;
    fn lcompl(self) -> Self::Output {
        Line {
            yw: self.x.lcompl(),
            wx: -self.y.lcompl(),
            xy: Default::default(),
        }
    }
}

impl Reverse for Dir {
    fn rev(self) -> Self {
        Dir {
            x: self.x.rev(),
            y: self.y.rev(),
        }
    }
}

impl AntiReverse for Dir {
    fn arev(self) -> Self {
        Dir {
            x: -self.x.arev(),
            y: -self.y.arev(),
        }
    }
}

// ---------------------------------------------------------------------
// Dir OP Dir:

// Dir.geometric(Dir) -> Rotor
impl Geometric<Dir> for Dir {
    type Output = Rotor;
    fn geometric(self, rhs: Dir) -> Self::Output {
        Rotor {
            s: self.x.geometric(rhs.x) + self.y.geometric(rhs.y),
            xy: self.x.geometric(rhs.y) - self.y.geometric(rhs.x),
        }
    }
}

// Omitted: Dir anti_geometric Dir = 0

// Dir.dot(Dir) -> S
impl Dot<Dir> for Dir {
    type Output = S;
    fn dot(self, rhs: Dir) -> Self::Output {
        self.x.dot(rhs.x) + self.y.dot(rhs.y)
    }
}

// Omitted: Dir wedge Dir = self.x.wedge(rhs.y) + self.y.wedge(rhs.x)
// Omitted: Dir anti_wedge Dir = 0

// ---------------------------------------------------------------------
// Dir OP Point:

// Dir.geometric(Point) -> Motor
impl Geometric<Point> for Dir {
    type Output = Motor;
    fn geometric(self, rhs: Point) -> Self::Output {
        Motor {
            s: self.x.geometric(rhs.x) + self.y.geometric(rhs.y),
            yw: self.y.geometric(rhs.w),
            wx: self.x.geometric(rhs.w),
            xy: self.x.geometric(rhs.y) - self.y.geometric(rhs.x),
        }
    }
}

// Dir.anti_geometric(Point) -> Dir
impl AntiGeometric<Point> for Dir {
    type Output = Dir;
    fn anti_geometric(self, rhs: Point) -> Self::Output {
        Dir {
            x: -self.y.anti_geometric(rhs.w),
            y: self.x.anti_geometric(rhs.w),
        }
    }
}

// Dir.dot(Point) -> S
impl Dot<Point> for Dir {
    type Output = S;
    fn dot(self, rhs: Point) -> Self::Output {
        self.x.dot(rhs.x) + self.y.dot(rhs.y)
    }
}

// Dir.wedge(Point) -> Line
impl Wedge<Point> for Dir {
    type Output = Line;
    fn wedge(self, rhs: Point) -> Self::Output {
        Line {
            yw: self.y.wedge(rhs.w),
            wx: self.x.wedge(rhs.w),
            xy: self.x.wedge(rhs.y) - self.y.wedge(rhs.x),
        }
    }
}

// Omitted: Dir anti_wedge Point = 0

// ---------------------------------------------------------------------
// Dir OP Line:

// Omitted: Dir geometric Line = self.x.geometric(rhs.wx) + self.x.geometric(rhs.xy) + self.x.geometric(rhs.yw) + self.y.geometric(rhs.wx) + self.y.geometric(rhs.xy) + self.y.geometric(rhs.yw)

// Dir.anti_geometric(Line) -> Rotor
impl AntiGeometric<Line> for Dir {
    type Output = Rotor;
    fn anti_geometric(self, rhs: Line) -> Self::Output {
        Rotor {
            s: self.x.anti_geometric(rhs.yw) + self.y.anti_geometric(rhs.wx),
            xy: -self.x.anti_geometric(rhs.wx) + self.y.anti_geometric(rhs.yw),
        }
    }
}

// Dir.dot(Line) -> Point
impl Dot<Line> for Dir {
    type Output = Point;
    fn dot(self, rhs: Line) -> Self::Output {
        Point {
            x: -self.y.dot(rhs.xy),
            y: self.x.dot(rhs.xy),
            w: -self.x.dot(rhs.wx) + self.y.dot(rhs.yw),
        }
    }
}

// Dir.wedge(Line) -> XYW
impl Wedge<Line> for Dir {
    type Output = XYW;
    fn wedge(self, rhs: Line) -> Self::Output {
        self.x.wedge(rhs.yw) + self.y.wedge(rhs.wx)
    }
}

// Dir.anti_wedge(Line) -> S
impl AntiWedge<Line> for Dir {
    type Output = S;
    fn anti_wedge(self, rhs: Line) -> Self::Output {
        self.x.anti_wedge(rhs.yw) + self.y.anti_wedge(rhs.wx)
    }
}

// ---------------------------------------------------------------------
// Dir OP Translator:

// Omitted: Dir geometric Translator = self.x.geometric(rhs.s) + self.x.geometric(rhs.wx) + self.x.geometric(rhs.yw) + self.y.geometric(rhs.s) + self.y.geometric(rhs.wx) + self.y.geometric(rhs.yw)

// Dir.anti_geometric(Translator) -> Rotor
impl AntiGeometric<Translator> for Dir {
    type Output = Rotor;
    fn anti_geometric(self, rhs: Translator) -> Self::Output {
        Rotor {
            s: self.x.anti_geometric(rhs.yw) + self.y.anti_geometric(rhs.wx),
            xy: -self.x.anti_geometric(rhs.wx) + self.y.anti_geometric(rhs.yw),
        }
    }
}

// Dir.dot(Translator) -> Point
impl Dot<Translator> for Dir {
    type Output = Point;
    fn dot(self, rhs: Translator) -> Self::Output {
        Point {
            x: self.x.dot(rhs.s),
            y: self.y.dot(rhs.s),
            w: -self.x.dot(rhs.wx) + self.y.dot(rhs.yw),
        }
    }
}

// Omitted: Dir wedge Translator = self.x.wedge(rhs.s) + self.x.wedge(rhs.yw) + self.y.wedge(rhs.s) + self.y.wedge(rhs.wx)

// Dir.anti_wedge(Translator) -> S
impl AntiWedge<Translator> for Dir {
    type Output = S;
    fn anti_wedge(self, rhs: Translator) -> Self::Output {
        self.x.anti_wedge(rhs.yw) + self.y.anti_wedge(rhs.wx)
    }
}

// ---------------------------------------------------------------------
// Dir OP Rotor:

// Dir.geometric(Rotor) -> Dir
impl Geometric<Rotor> for Dir {
    type Output = Dir;
    fn geometric(self, rhs: Rotor) -> Self::Output {
        Dir {
            x: self.x.geometric(rhs.s) - self.y.geometric(rhs.xy),
            y: self.x.geometric(rhs.xy) + self.y.geometric(rhs.s),
        }
    }
}

// Omitted: Dir anti_geometric Rotor = 0

// Dir.dot(Rotor) -> Dir
impl Dot<Rotor> for Dir {
    type Output = Dir;
    fn dot(self, rhs: Rotor) -> Self::Output {
        Dir {
            x: self.x.dot(rhs.s) - self.y.dot(rhs.xy),
            y: self.x.dot(rhs.xy) + self.y.dot(rhs.s),
        }
    }
}

// Dir.wedge(Rotor) -> Dir
impl Wedge<Rotor> for Dir {
    type Output = Dir;
    fn wedge(self, rhs: Rotor) -> Self::Output {
        Dir {
            x: self.x.wedge(rhs.s),
            y: self.y.wedge(rhs.s),
        }
    }
}

// Omitted: Dir anti_wedge Rotor = 0

// ---------------------------------------------------------------------
// Dir OP Motor:

// Omitted: Dir geometric Motor = self.x.geometric(rhs.s) + self.x.geometric(rhs.wx) + self.x.geometric(rhs.xy) + self.x.geometric(rhs.yw) + self.y.geometric(rhs.s) + self.y.geometric(rhs.wx) + self.y.geometric(rhs.xy) + self.y.geometric(rhs.yw)

// Dir.anti_geometric(Motor) -> Rotor
impl AntiGeometric<Motor> for Dir {
    type Output = Rotor;
    fn anti_geometric(self, rhs: Motor) -> Self::Output {
        Rotor {
            s: self.x.anti_geometric(rhs.yw) + self.y.anti_geometric(rhs.wx),
            xy: -self.x.anti_geometric(rhs.wx) + self.y.anti_geometric(rhs.yw),
        }
    }
}

// Dir.dot(Motor) -> Point
impl Dot<Motor> for Dir {
    type Output = Point;
    fn dot(self, rhs: Motor) -> Self::Output {
        Point {
            x: self.x.dot(rhs.s) - self.y.dot(rhs.xy),
            y: self.x.dot(rhs.xy) + self.y.dot(rhs.s),
            w: -self.x.dot(rhs.wx) + self.y.dot(rhs.yw),
        }
    }
}

// Omitted: Dir wedge Motor = self.x.wedge(rhs.s) + self.x.wedge(rhs.yw) + self.y.wedge(rhs.s) + self.y.wedge(rhs.wx)

// Dir.anti_wedge(Motor) -> S
impl AntiWedge<Motor> for Dir {
    type Output = S;
    fn anti_wedge(self, rhs: Motor) -> Self::Output {
        self.x.anti_wedge(rhs.yw) + self.y.anti_wedge(rhs.wx)
    }
}

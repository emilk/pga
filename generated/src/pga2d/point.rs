//! # Point
//!
//! ## Operations
//! ```text
//! Point.geometric(Point) -> Motor
//! Point.dot(Point) -> S
//! Point.wedge(Point) -> Line
//! Point.geometric(Dir) -> Motor
//! Dir.geometric(Point) -> Motor
//! Point.anti_geometric(Dir) -> Dir
//! Dir.anti_geometric(Point) -> Dir
//! Point.dot(Dir) -> S
//! Dir.dot(Point) -> S
//! Point.wedge(Dir) -> Line
//! Dir.wedge(Point) -> Line
//! Point.anti_geometric(Line) -> Motor
//! Line.anti_geometric(Point) -> Motor
//! Point.dot(Line) -> Point
//! Line.dot(Point) -> Point
//! Point.wedge(Line) -> XYW
//! Line.wedge(Point) -> XYW
//! Point.anti_wedge(Line) -> S
//! Line.anti_wedge(Point) -> S
//! Point.anti_geometric(Translator) -> Motor
//! Translator.anti_geometric(Point) -> Motor
//! Point.dot(Translator) -> Point
//! Translator.dot(Point) -> Point
//! Point.anti_wedge(Translator) -> S
//! Translator.anti_wedge(Point) -> S
//! Point.anti_geometric(Rotor) -> Rotor
//! Rotor.anti_geometric(Point) -> Rotor
//! Point.dot(Rotor) -> Point
//! Rotor.dot(Point) -> Point
//! Point.anti_wedge(Rotor) -> S
//! Rotor.anti_wedge(Point) -> S
//! Point.anti_geometric(Motor) -> Motor
//! Motor.anti_geometric(Point) -> Motor
//! Point.dot(Motor) -> Point
//! Motor.dot(Point) -> Point
//! Point.anti_wedge(Motor) -> S
//! Motor.anti_wedge(Point) -> S
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
pub struct Point {
    pub x: X,
    pub y: Y,
    pub w: W,
}

// ---------------------------------------------------------------------

impl RCompl for Point {
    type Output = Line;
    fn rcompl(self) -> Self::Output {
        Line {
            yw: self.x.rcompl(),
            wx: -self.y.rcompl(),
            xy: self.w.rcompl(),
        }
    }
}

impl LCompl for Point {
    type Output = Line;
    fn lcompl(self) -> Self::Output {
        Line {
            yw: self.x.lcompl(),
            wx: -self.y.lcompl(),
            xy: self.w.lcompl(),
        }
    }
}

impl Reverse for Point {
    fn rev(self) -> Self {
        Point {
            x: self.x,
            y: self.y,
            w: self.w,
        }
    }
}

impl AntiReverse for Point {
    fn arev(self) -> Self {
        Point {
            x: -self.x,
            y: -self.y,
            w: -self.w,
        }
    }
}

// ---------------------------------------------------------------------
// Point OP Dir:

// Point.geometric(Dir) -> Motor
impl Geometric<Dir> for Point {
    type Output = Motor;
    fn geometric(self, rhs: Dir) -> Self::Output {
        Motor {
            s: self.x.geometric(rhs.x) + self.y.geometric(rhs.y),
            yw: -self.w.geometric(rhs.y),
            wx: -self.w.geometric(rhs.x),
            xy: self.x.geometric(rhs.y) - self.y.geometric(rhs.x),
        }
    }
}

// Point.anti_geometric(Dir) -> Dir
impl AntiGeometric<Dir> for Point {
    type Output = Dir;
    fn anti_geometric(self, rhs: Dir) -> Self::Output {
        Dir {
            x: self.w.anti_geometric(rhs.y),
            y: -self.w.anti_geometric(rhs.x),
        }
    }
}

// Point.dot(Dir) -> S
impl Dot<Dir> for Point {
    type Output = S;
    fn dot(self, rhs: Dir) -> Self::Output {
        self.x.dot(rhs.x) + self.y.dot(rhs.y)
    }
}

// Point.wedge(Dir) -> Line
impl Wedge<Dir> for Point {
    type Output = Line;
    fn wedge(self, rhs: Dir) -> Self::Output {
        Line {
            yw: -self.w.wedge(rhs.y),
            wx: -self.w.wedge(rhs.x),
            xy: self.x.wedge(rhs.y) - self.y.wedge(rhs.x),
        }
    }
}

// Omitted: Point anti_wedge Dir = 0

// ---------------------------------------------------------------------
// Point OP Point:

// Point.geometric(Point) -> Motor
impl Geometric<Point> for Point {
    type Output = Motor;
    fn geometric(self, rhs: Point) -> Self::Output {
        Motor {
            s: self.x.geometric(rhs.x) + self.y.geometric(rhs.y),
            yw: -self.w.geometric(rhs.y) + self.y.geometric(rhs.w),
            wx: -self.w.geometric(rhs.x) + self.x.geometric(rhs.w),
            xy: self.x.geometric(rhs.y) - self.y.geometric(rhs.x),
        }
    }
}

// Omitted: Point anti_geometric Point = self.w.anti_geometric(rhs.w) + self.w.anti_geometric(rhs.x) + self.w.anti_geometric(rhs.y) + self.x.anti_geometric(rhs.w) + self.y.anti_geometric(rhs.w)

// Point.dot(Point) -> S
impl Dot<Point> for Point {
    type Output = S;
    fn dot(self, rhs: Point) -> Self::Output {
        self.x.dot(rhs.x) + self.y.dot(rhs.y)
    }
}

// Point.wedge(Point) -> Line
impl Wedge<Point> for Point {
    type Output = Line;
    fn wedge(self, rhs: Point) -> Self::Output {
        Line {
            yw: -self.w.wedge(rhs.y) + self.y.wedge(rhs.w),
            wx: -self.w.wedge(rhs.x) + self.x.wedge(rhs.w),
            xy: self.x.wedge(rhs.y) - self.y.wedge(rhs.x),
        }
    }
}

// Omitted: Point anti_wedge Point = 0

// ---------------------------------------------------------------------
// Point OP Line:

// Omitted: Point geometric Line = self.w.geometric(rhs.xy) + self.x.geometric(rhs.wx) + self.x.geometric(rhs.xy) + self.x.geometric(rhs.yw) + self.y.geometric(rhs.wx) + self.y.geometric(rhs.xy) + self.y.geometric(rhs.yw)

// Point.anti_geometric(Line) -> Motor
impl AntiGeometric<Line> for Point {
    type Output = Motor;
    fn anti_geometric(self, rhs: Line) -> Self::Output {
        Motor {
            s: self.w.anti_geometric(rhs.xy)
                + self.x.anti_geometric(rhs.yw)
                + self.y.anti_geometric(rhs.wx),
            yw: self.w.anti_geometric(rhs.wx),
            wx: self.w.anti_geometric(rhs.yw),
            xy: -self.x.anti_geometric(rhs.wx) + self.y.anti_geometric(rhs.yw),
        }
    }
}

// Point.dot(Line) -> Point
impl Dot<Line> for Point {
    type Output = Point;
    fn dot(self, rhs: Line) -> Self::Output {
        Point {
            x: -self.y.dot(rhs.xy),
            y: self.x.dot(rhs.xy),
            w: -self.x.dot(rhs.wx) + self.y.dot(rhs.yw),
        }
    }
}

// Point.wedge(Line) -> XYW
impl Wedge<Line> for Point {
    type Output = XYW;
    fn wedge(self, rhs: Line) -> Self::Output {
        self.w.wedge(rhs.xy) + self.x.wedge(rhs.yw) + self.y.wedge(rhs.wx)
    }
}

// Point.anti_wedge(Line) -> S
impl AntiWedge<Line> for Point {
    type Output = S;
    fn anti_wedge(self, rhs: Line) -> Self::Output {
        self.w.anti_wedge(rhs.xy) + self.x.anti_wedge(rhs.yw) + self.y.anti_wedge(rhs.wx)
    }
}

// ---------------------------------------------------------------------
// Point OP Translator:

// Omitted: Point geometric Translator = self.w.geometric(rhs.s) + self.x.geometric(rhs.s) + self.x.geometric(rhs.wx) + self.x.geometric(rhs.yw) + self.y.geometric(rhs.s) + self.y.geometric(rhs.wx) + self.y.geometric(rhs.yw)

// Point.anti_geometric(Translator) -> Motor
impl AntiGeometric<Translator> for Point {
    type Output = Motor;
    fn anti_geometric(self, rhs: Translator) -> Self::Output {
        Motor {
            s: self.x.anti_geometric(rhs.yw) + self.y.anti_geometric(rhs.wx),
            yw: self.w.anti_geometric(rhs.wx),
            wx: self.w.anti_geometric(rhs.yw),
            xy: -self.w.anti_geometric(rhs.s) - self.x.anti_geometric(rhs.wx)
                + self.y.anti_geometric(rhs.yw),
        }
    }
}

// Point.dot(Translator) -> Point
impl Dot<Translator> for Point {
    type Output = Point;
    fn dot(self, rhs: Translator) -> Self::Output {
        Point {
            x: self.x.dot(rhs.s),
            y: self.y.dot(rhs.s),
            w: self.w.dot(rhs.s) - self.x.dot(rhs.wx) + self.y.dot(rhs.yw),
        }
    }
}

// Omitted: Point wedge Translator = self.w.wedge(rhs.s) + self.x.wedge(rhs.s) + self.x.wedge(rhs.yw) + self.y.wedge(rhs.s) + self.y.wedge(rhs.wx)

// Point.anti_wedge(Translator) -> S
impl AntiWedge<Translator> for Point {
    type Output = S;
    fn anti_wedge(self, rhs: Translator) -> Self::Output {
        self.x.anti_wedge(rhs.yw) + self.y.anti_wedge(rhs.wx)
    }
}

// ---------------------------------------------------------------------
// Point OP Rotor:

// Omitted: Point geometric Rotor = self.w.geometric(rhs.s) + self.w.geometric(rhs.xy) + self.x.geometric(rhs.s) + self.x.geometric(rhs.xy) + self.y.geometric(rhs.s) + self.y.geometric(rhs.xy)

// Point.anti_geometric(Rotor) -> Rotor
impl AntiGeometric<Rotor> for Point {
    type Output = Rotor;
    fn anti_geometric(self, rhs: Rotor) -> Self::Output {
        Rotor {
            s: self.w.anti_geometric(rhs.xy),
            xy: -self.w.anti_geometric(rhs.s),
        }
    }
}

// Point.dot(Rotor) -> Point
impl Dot<Rotor> for Point {
    type Output = Point;
    fn dot(self, rhs: Rotor) -> Self::Output {
        Point {
            x: self.x.dot(rhs.s) - self.y.dot(rhs.xy),
            y: self.x.dot(rhs.xy) + self.y.dot(rhs.s),
            w: self.w.dot(rhs.s),
        }
    }
}

// Omitted: Point wedge Rotor = self.w.wedge(rhs.s) + self.w.wedge(rhs.xy) + self.x.wedge(rhs.s) + self.y.wedge(rhs.s)

// Point.anti_wedge(Rotor) -> S
impl AntiWedge<Rotor> for Point {
    type Output = S;
    fn anti_wedge(self, rhs: Rotor) -> Self::Output {
        self.w.anti_wedge(rhs.xy)
    }
}

// ---------------------------------------------------------------------
// Point OP Motor:

// Omitted: Point geometric Motor = self.w.geometric(rhs.s) + self.w.geometric(rhs.xy) + self.x.geometric(rhs.s) + self.x.geometric(rhs.wx) + self.x.geometric(rhs.xy) + self.x.geometric(rhs.yw) + self.y.geometric(rhs.s) + self.y.geometric(rhs.wx) + self.y.geometric(rhs.xy) + self.y.geometric(rhs.yw)

// Point.anti_geometric(Motor) -> Motor
impl AntiGeometric<Motor> for Point {
    type Output = Motor;
    fn anti_geometric(self, rhs: Motor) -> Self::Output {
        Motor {
            s: self.w.anti_geometric(rhs.xy)
                + self.x.anti_geometric(rhs.yw)
                + self.y.anti_geometric(rhs.wx),
            yw: self.w.anti_geometric(rhs.wx),
            wx: self.w.anti_geometric(rhs.yw),
            xy: -self.w.anti_geometric(rhs.s) - self.x.anti_geometric(rhs.wx)
                + self.y.anti_geometric(rhs.yw),
        }
    }
}

// Point.dot(Motor) -> Point
impl Dot<Motor> for Point {
    type Output = Point;
    fn dot(self, rhs: Motor) -> Self::Output {
        Point {
            x: self.x.dot(rhs.s) - self.y.dot(rhs.xy),
            y: self.x.dot(rhs.xy) + self.y.dot(rhs.s),
            w: self.w.dot(rhs.s) - self.x.dot(rhs.wx) + self.y.dot(rhs.yw),
        }
    }
}

// Omitted: Point wedge Motor = self.w.wedge(rhs.s) + self.w.wedge(rhs.xy) + self.x.wedge(rhs.s) + self.x.wedge(rhs.yw) + self.y.wedge(rhs.s) + self.y.wedge(rhs.wx)

// Point.anti_wedge(Motor) -> S
impl AntiWedge<Motor> for Point {
    type Output = S;
    fn anti_wedge(self, rhs: Motor) -> Self::Output {
        self.w.anti_wedge(rhs.xy) + self.x.anti_wedge(rhs.yw) + self.y.anti_wedge(rhs.wx)
    }
}

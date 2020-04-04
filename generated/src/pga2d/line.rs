//! # Line
//!
//! ## Operations
//! ```text
//! Line.geometric(Line) -> Translator
//! Line.dot(Line) -> S
//! Line.anti_wedge(Line) -> Point
//! Line.anti_geometric(Dir) -> Rotor
//! Dir.anti_geometric(Line) -> Rotor
//! Line.dot(Dir) -> Point
//! Dir.dot(Line) -> Point
//! Line.wedge(Dir) -> XYW
//! Dir.wedge(Line) -> XYW
//! Line.anti_wedge(Dir) -> S
//! Dir.anti_wedge(Line) -> S
//! Line.anti_geometric(Point) -> Motor
//! Point.anti_geometric(Line) -> Motor
//! Line.dot(Point) -> Point
//! Point.dot(Line) -> Point
//! Line.wedge(Point) -> XYW
//! Point.wedge(Line) -> XYW
//! Line.anti_wedge(Point) -> S
//! Point.anti_wedge(Line) -> S
//! Line.geometric(Translator) -> Line
//! Translator.geometric(Line) -> Line
//! Line.dot(Translator) -> Line
//! Translator.dot(Line) -> Line
//! Line.wedge(Translator) -> Line
//! Translator.wedge(Line) -> Line
//! Line.anti_wedge(Translator) -> Point
//! Translator.anti_wedge(Line) -> Point
//! Line.geometric(Rotor) -> Motor
//! Rotor.geometric(Line) -> Motor
//! Line.anti_geometric(Rotor) -> Dir
//! Rotor.anti_geometric(Line) -> Dir
//! Line.dot(Rotor) -> Motor
//! Rotor.dot(Line) -> Motor
//! Line.wedge(Rotor) -> Line
//! Rotor.wedge(Line) -> Line
//! Line.anti_wedge(Rotor) -> Dir
//! Rotor.anti_wedge(Line) -> Dir
//! Line.geometric(Motor) -> Motor
//! Motor.geometric(Line) -> Motor
//! Line.dot(Motor) -> Motor
//! Motor.dot(Line) -> Motor
//! Line.wedge(Motor) -> Line
//! Motor.wedge(Line) -> Line
//! Line.anti_wedge(Motor) -> Point
//! Motor.anti_wedge(Line) -> Point
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
pub struct Line {
    pub yw: YW,
    pub wx: WX,
    pub xy: XY,
}

// ---------------------------------------------------------------------
// Line OP Dir:

// Omitted: Line geometric Dir = self.wx.geometric(rhs.x) + self.wx.geometric(rhs.y) + self.xy.geometric(rhs.x) + self.xy.geometric(rhs.y) + self.yw.geometric(rhs.x) + self.yw.geometric(rhs.y)

// Line.anti_geometric(Dir) -> Rotor
impl AntiGeometric<Dir> for Line {
    type Output = Rotor;
    fn anti_geometric(self, rhs: Dir) -> Self::Output {
        Rotor {
            s: self.wx.anti_geometric(rhs.y) + self.yw.anti_geometric(rhs.x),
            xy: self.wx.anti_geometric(rhs.x) - self.yw.anti_geometric(rhs.y),
        }
    }
}

// Line.dot(Dir) -> Point
impl Dot<Dir> for Line {
    type Output = Point;
    fn dot(self, rhs: Dir) -> Self::Output {
        Point {
            x: self.xy.dot(rhs.y),
            y: -self.xy.dot(rhs.x),
            w: self.wx.dot(rhs.x) - self.yw.dot(rhs.y),
        }
    }
}

// Line.wedge(Dir) -> XYW
impl Wedge<Dir> for Line {
    type Output = XYW;
    fn wedge(self, rhs: Dir) -> Self::Output {
        self.wx.wedge(rhs.y) + self.yw.wedge(rhs.x)
    }
}

// Line.anti_wedge(Dir) -> S
impl AntiWedge<Dir> for Line {
    type Output = S;
    fn anti_wedge(self, rhs: Dir) -> Self::Output {
        self.wx.anti_wedge(rhs.y) + self.yw.anti_wedge(rhs.x)
    }
}

// ---------------------------------------------------------------------
// Line OP Point:

// Omitted: Line geometric Point = self.wx.geometric(rhs.x) + self.wx.geometric(rhs.y) + self.xy.geometric(rhs.w) + self.xy.geometric(rhs.x) + self.xy.geometric(rhs.y) + self.yw.geometric(rhs.x) + self.yw.geometric(rhs.y)

// Line.anti_geometric(Point) -> Motor
impl AntiGeometric<Point> for Line {
    type Output = Motor;
    fn anti_geometric(self, rhs: Point) -> Self::Output {
        Motor {
            s: self.wx.anti_geometric(rhs.y)
                + self.xy.anti_geometric(rhs.w)
                + self.yw.anti_geometric(rhs.x),
            yw: -self.wx.anti_geometric(rhs.w),
            wx: -self.yw.anti_geometric(rhs.w),
            xy: self.wx.anti_geometric(rhs.x) - self.yw.anti_geometric(rhs.y),
        }
    }
}

// Line.dot(Point) -> Point
impl Dot<Point> for Line {
    type Output = Point;
    fn dot(self, rhs: Point) -> Self::Output {
        Point {
            x: self.xy.dot(rhs.y),
            y: -self.xy.dot(rhs.x),
            w: self.wx.dot(rhs.x) - self.yw.dot(rhs.y),
        }
    }
}

// Line.wedge(Point) -> XYW
impl Wedge<Point> for Line {
    type Output = XYW;
    fn wedge(self, rhs: Point) -> Self::Output {
        self.wx.wedge(rhs.y) + self.xy.wedge(rhs.w) + self.yw.wedge(rhs.x)
    }
}

// Line.anti_wedge(Point) -> S
impl AntiWedge<Point> for Line {
    type Output = S;
    fn anti_wedge(self, rhs: Point) -> Self::Output {
        self.wx.anti_wedge(rhs.y) + self.xy.anti_wedge(rhs.w) + self.yw.anti_wedge(rhs.x)
    }
}

// ---------------------------------------------------------------------
// Line OP Line:

// Line.geometric(Line) -> Translator
impl Geometric<Line> for Line {
    type Output = Translator;
    fn geometric(self, rhs: Line) -> Self::Output {
        Translator {
            s: -self.xy.geometric(rhs.xy),
            yw: -self.wx.geometric(rhs.xy) + self.xy.geometric(rhs.wx),
            wx: self.xy.geometric(rhs.yw) - self.yw.geometric(rhs.xy),
        }
    }
}

// Omitted: Line anti_geometric Line = self.wx.anti_geometric(rhs.wx) + self.wx.anti_geometric(rhs.xy) + self.wx.anti_geometric(rhs.yw) + self.xy.anti_geometric(rhs.wx) + self.xy.anti_geometric(rhs.yw) + self.yw.anti_geometric(rhs.wx) + self.yw.anti_geometric(rhs.xy) + self.yw.anti_geometric(rhs.yw)

// Line.dot(Line) -> S
impl Dot<Line> for Line {
    type Output = S;
    fn dot(self, rhs: Line) -> Self::Output {
        self.xy.dot(rhs.xy)
    }
}

// Omitted: Line wedge Line = 0

// Line.anti_wedge(Line) -> Point
impl AntiWedge<Line> for Line {
    type Output = Point;
    fn anti_wedge(self, rhs: Line) -> Self::Output {
        Point {
            x: self.wx.anti_wedge(rhs.xy) - self.xy.anti_wedge(rhs.wx),
            y: self.xy.anti_wedge(rhs.yw) - self.yw.anti_wedge(rhs.xy),
            w: -self.wx.anti_wedge(rhs.yw) + self.yw.anti_wedge(rhs.wx),
        }
    }
}

// ---------------------------------------------------------------------
// Line OP Translator:

// Line.geometric(Translator) -> Line
impl Geometric<Translator> for Line {
    type Output = Line;
    fn geometric(self, rhs: Translator) -> Self::Output {
        Line {
            yw: self.xy.geometric(rhs.wx) + self.yw.geometric(rhs.s),
            wx: -self.wx.geometric(rhs.s) + self.xy.geometric(rhs.yw),
            xy: self.xy.geometric(rhs.s),
        }
    }
}

// Omitted: Line anti_geometric Translator = self.wx.anti_geometric(rhs.s) + self.wx.anti_geometric(rhs.wx) + self.wx.anti_geometric(rhs.yw) + self.xy.anti_geometric(rhs.wx) + self.xy.anti_geometric(rhs.yw) + self.yw.anti_geometric(rhs.s) + self.yw.anti_geometric(rhs.wx) + self.yw.anti_geometric(rhs.yw)

// Line.dot(Translator) -> Line
impl Dot<Translator> for Line {
    type Output = Line;
    fn dot(self, rhs: Translator) -> Self::Output {
        Line {
            yw: self.yw.dot(rhs.s),
            wx: -self.wx.dot(rhs.s),
            xy: self.xy.dot(rhs.s),
        }
    }
}

// Line.wedge(Translator) -> Line
impl Wedge<Translator> for Line {
    type Output = Line;
    fn wedge(self, rhs: Translator) -> Self::Output {
        Line {
            yw: self.yw.wedge(rhs.s),
            wx: -self.wx.wedge(rhs.s),
            xy: self.xy.wedge(rhs.s),
        }
    }
}

// Line.anti_wedge(Translator) -> Point
impl AntiWedge<Translator> for Line {
    type Output = Point;
    fn anti_wedge(self, rhs: Translator) -> Self::Output {
        Point {
            x: -self.xy.anti_wedge(rhs.wx),
            y: self.xy.anti_wedge(rhs.yw),
            w: -self.wx.anti_wedge(rhs.yw) + self.yw.anti_wedge(rhs.wx),
        }
    }
}

// ---------------------------------------------------------------------
// Line OP Rotor:

// Line.geometric(Rotor) -> Motor
impl Geometric<Rotor> for Line {
    type Output = Motor;
    fn geometric(self, rhs: Rotor) -> Self::Output {
        Motor {
            s: -self.xy.geometric(rhs.xy),
            yw: -self.wx.geometric(rhs.xy) + self.yw.geometric(rhs.s),
            wx: -self.wx.geometric(rhs.s) - self.yw.geometric(rhs.xy),
            xy: self.xy.geometric(rhs.s),
        }
    }
}

// Line.anti_geometric(Rotor) -> Dir
impl AntiGeometric<Rotor> for Line {
    type Output = Dir;
    fn anti_geometric(self, rhs: Rotor) -> Self::Output {
        Dir {
            x: self.wx.anti_geometric(rhs.xy) + self.yw.anti_geometric(rhs.s),
            y: self.wx.anti_geometric(rhs.s) - self.yw.anti_geometric(rhs.xy),
        }
    }
}

// Line.dot(Rotor) -> Motor
impl Dot<Rotor> for Line {
    type Output = Motor;
    fn dot(self, rhs: Rotor) -> Self::Output {
        Motor {
            s: -self.xy.dot(rhs.xy),
            yw: self.yw.dot(rhs.s),
            wx: -self.wx.dot(rhs.s),
            xy: self.xy.dot(rhs.s),
        }
    }
}

// Line.wedge(Rotor) -> Line
impl Wedge<Rotor> for Line {
    type Output = Line;
    fn wedge(self, rhs: Rotor) -> Self::Output {
        Line {
            yw: self.yw.wedge(rhs.s),
            wx: -self.wx.wedge(rhs.s),
            xy: self.xy.wedge(rhs.s),
        }
    }
}

// Line.anti_wedge(Rotor) -> Dir
impl AntiWedge<Rotor> for Line {
    type Output = Dir;
    fn anti_wedge(self, rhs: Rotor) -> Self::Output {
        Dir {
            x: self.wx.anti_wedge(rhs.xy),
            y: -self.yw.anti_wedge(rhs.xy),
        }
    }
}

// ---------------------------------------------------------------------
// Line OP Motor:

// Line.geometric(Motor) -> Motor
impl Geometric<Motor> for Line {
    type Output = Motor;
    fn geometric(self, rhs: Motor) -> Self::Output {
        Motor {
            s: -self.xy.geometric(rhs.xy),
            yw: -self.wx.geometric(rhs.xy) + self.xy.geometric(rhs.wx) + self.yw.geometric(rhs.s),
            wx: -self.wx.geometric(rhs.s) + self.xy.geometric(rhs.yw) - self.yw.geometric(rhs.xy),
            xy: self.xy.geometric(rhs.s),
        }
    }
}

// Omitted: Line anti_geometric Motor = self.wx.anti_geometric(rhs.s) + self.wx.anti_geometric(rhs.wx) + self.wx.anti_geometric(rhs.xy) + self.wx.anti_geometric(rhs.yw) + self.xy.anti_geometric(rhs.wx) + self.xy.anti_geometric(rhs.yw) + self.yw.anti_geometric(rhs.s) + self.yw.anti_geometric(rhs.wx) + self.yw.anti_geometric(rhs.xy) + self.yw.anti_geometric(rhs.yw)

// Line.dot(Motor) -> Motor
impl Dot<Motor> for Line {
    type Output = Motor;
    fn dot(self, rhs: Motor) -> Self::Output {
        Motor {
            s: -self.xy.dot(rhs.xy),
            yw: self.yw.dot(rhs.s),
            wx: -self.wx.dot(rhs.s),
            xy: self.xy.dot(rhs.s),
        }
    }
}

// Line.wedge(Motor) -> Line
impl Wedge<Motor> for Line {
    type Output = Line;
    fn wedge(self, rhs: Motor) -> Self::Output {
        Line {
            yw: self.yw.wedge(rhs.s),
            wx: -self.wx.wedge(rhs.s),
            xy: self.xy.wedge(rhs.s),
        }
    }
}

// Line.anti_wedge(Motor) -> Point
impl AntiWedge<Motor> for Line {
    type Output = Point;
    fn anti_wedge(self, rhs: Motor) -> Self::Output {
        Point {
            x: self.wx.anti_wedge(rhs.xy) - self.xy.anti_wedge(rhs.wx),
            y: self.xy.anti_wedge(rhs.yw) - self.yw.anti_wedge(rhs.xy),
            w: -self.wx.anti_wedge(rhs.yw) + self.yw.anti_wedge(rhs.wx),
        }
    }
}

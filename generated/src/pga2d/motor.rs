//! # Motor
//!
//! ## Operations
//! ```text
//! Motor.geometric(Motor) -> Motor
//! Motor.dot(Motor) -> Motor
//! Motor.wedge(Motor) -> Motor
//! Motor.anti_wedge(Motor) -> Point
//! Motor.anti_geometric(Dir) -> Rotor
//! Dir.anti_geometric(Motor) -> Rotor
//! Motor.dot(Dir) -> Point
//! Dir.dot(Motor) -> Point
//! Motor.anti_wedge(Dir) -> S
//! Dir.anti_wedge(Motor) -> S
//! Motor.anti_geometric(Point) -> Motor
//! Point.anti_geometric(Motor) -> Motor
//! Motor.dot(Point) -> Point
//! Point.dot(Motor) -> Point
//! Motor.anti_wedge(Point) -> S
//! Point.anti_wedge(Motor) -> S
//! Motor.geometric(Line) -> Motor
//! Line.geometric(Motor) -> Motor
//! Motor.dot(Line) -> Motor
//! Line.dot(Motor) -> Motor
//! Motor.wedge(Line) -> Line
//! Line.wedge(Motor) -> Line
//! Motor.anti_wedge(Line) -> Point
//! Line.anti_wedge(Motor) -> Point
//! Motor.geometric(Translator) -> Motor
//! Translator.geometric(Motor) -> Motor
//! Motor.dot(Translator) -> Motor
//! Translator.dot(Motor) -> Motor
//! Motor.wedge(Translator) -> Motor
//! Translator.wedge(Motor) -> Motor
//! Motor.anti_wedge(Translator) -> Point
//! Translator.anti_wedge(Motor) -> Point
//! Motor.geometric(Rotor) -> Motor
//! Rotor.geometric(Motor) -> Motor
//! Motor.anti_geometric(Rotor) -> Dir
//! Rotor.anti_geometric(Motor) -> Dir
//! Motor.dot(Rotor) -> Motor
//! Rotor.dot(Motor) -> Motor
//! Motor.wedge(Rotor) -> Motor
//! Rotor.wedge(Motor) -> Motor
//! Motor.anti_wedge(Rotor) -> Dir
//! Rotor.anti_wedge(Motor) -> Dir
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
// Motor OP Dir:

// Omitted: Motor geometric Dir = self.s.geometric(rhs.x) + self.s.geometric(rhs.y) + self.wx.geometric(rhs.x) + self.wx.geometric(rhs.y) + self.xy.geometric(rhs.x) + self.xy.geometric(rhs.y) + self.yw.geometric(rhs.x) + self.yw.geometric(rhs.y)

// Motor.anti_geometric(Dir) -> Rotor
impl AntiGeometric<Dir> for Motor {
    type Output = Rotor;
    fn anti_geometric(self, rhs: Dir) -> Self::Output {
        Rotor {
            s: self.wx.anti_geometric(rhs.y) + self.yw.anti_geometric(rhs.x),
            xy: self.wx.anti_geometric(rhs.x) - self.yw.anti_geometric(rhs.y),
        }
    }
}

// Motor.dot(Dir) -> Point
impl Dot<Dir> for Motor {
    type Output = Point;
    fn dot(self, rhs: Dir) -> Self::Output {
        Point {
            x: self.s.dot(rhs.x) + self.xy.dot(rhs.y),
            y: self.s.dot(rhs.y) - self.xy.dot(rhs.x),
            w: self.wx.dot(rhs.x) - self.yw.dot(rhs.y),
        }
    }
}

// Omitted: Motor wedge Dir = self.s.wedge(rhs.x) + self.s.wedge(rhs.y) + self.wx.wedge(rhs.y) + self.yw.wedge(rhs.x)

// Motor.anti_wedge(Dir) -> S
impl AntiWedge<Dir> for Motor {
    type Output = S;
    fn anti_wedge(self, rhs: Dir) -> Self::Output {
        self.wx.anti_wedge(rhs.y) + self.yw.anti_wedge(rhs.x)
    }
}

// ---------------------------------------------------------------------
// Motor OP Point:

// Omitted: Motor geometric Point = self.s.geometric(rhs.w) + self.s.geometric(rhs.x) + self.s.geometric(rhs.y) + self.wx.geometric(rhs.x) + self.wx.geometric(rhs.y) + self.xy.geometric(rhs.w) + self.xy.geometric(rhs.x) + self.xy.geometric(rhs.y) + self.yw.geometric(rhs.x) + self.yw.geometric(rhs.y)

// Motor.anti_geometric(Point) -> Motor
impl AntiGeometric<Point> for Motor {
    type Output = Motor;
    fn anti_geometric(self, rhs: Point) -> Self::Output {
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

// Motor.dot(Point) -> Point
impl Dot<Point> for Motor {
    type Output = Point;
    fn dot(self, rhs: Point) -> Self::Output {
        Point {
            x: self.s.dot(rhs.x) + self.xy.dot(rhs.y),
            y: self.s.dot(rhs.y) - self.xy.dot(rhs.x),
            w: self.s.dot(rhs.w) + self.wx.dot(rhs.x) - self.yw.dot(rhs.y),
        }
    }
}

// Omitted: Motor wedge Point = self.s.wedge(rhs.w) + self.s.wedge(rhs.x) + self.s.wedge(rhs.y) + self.wx.wedge(rhs.y) + self.xy.wedge(rhs.w) + self.yw.wedge(rhs.x)

// Motor.anti_wedge(Point) -> S
impl AntiWedge<Point> for Motor {
    type Output = S;
    fn anti_wedge(self, rhs: Point) -> Self::Output {
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
            s: -self.xy.geometric(rhs.xy),
            yw: self.s.geometric(rhs.yw) - self.wx.geometric(rhs.xy) + self.xy.geometric(rhs.wx),
            wx: self.s.geometric(rhs.wx) - self.xy.geometric(rhs.yw) + self.yw.geometric(rhs.xy),
            xy: self.s.geometric(rhs.xy),
        }
    }
}

// Omitted: Motor anti_geometric Line = self.s.anti_geometric(rhs.wx) + self.s.anti_geometric(rhs.yw) + self.wx.anti_geometric(rhs.wx) + self.wx.anti_geometric(rhs.xy) + self.wx.anti_geometric(rhs.yw) + self.xy.anti_geometric(rhs.wx) + self.xy.anti_geometric(rhs.yw) + self.yw.anti_geometric(rhs.wx) + self.yw.anti_geometric(rhs.xy) + self.yw.anti_geometric(rhs.yw)

// Motor.dot(Line) -> Motor
impl Dot<Line> for Motor {
    type Output = Motor;
    fn dot(self, rhs: Line) -> Self::Output {
        Motor {
            s: -self.xy.dot(rhs.xy),
            yw: self.s.dot(rhs.yw),
            wx: self.s.dot(rhs.wx),
            xy: self.s.dot(rhs.xy),
        }
    }
}

// Motor.wedge(Line) -> Line
impl Wedge<Line> for Motor {
    type Output = Line;
    fn wedge(self, rhs: Line) -> Self::Output {
        Line {
            yw: self.s.wedge(rhs.yw),
            wx: self.s.wedge(rhs.wx),
            xy: self.s.wedge(rhs.xy),
        }
    }
}

// Motor.anti_wedge(Line) -> Point
impl AntiWedge<Line> for Motor {
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

// Motor.anti_wedge(Translator) -> Point
impl AntiWedge<Translator> for Motor {
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

// Motor.anti_geometric(Rotor) -> Dir
impl AntiGeometric<Rotor> for Motor {
    type Output = Dir;
    fn anti_geometric(self, rhs: Rotor) -> Self::Output {
        Dir {
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

// Motor.anti_wedge(Rotor) -> Dir
impl AntiWedge<Rotor> for Motor {
    type Output = Dir;
    fn anti_wedge(self, rhs: Rotor) -> Self::Output {
        Dir {
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

// Motor.anti_wedge(Motor) -> Point
impl AntiWedge<Motor> for Motor {
    type Output = Point;
    fn anti_wedge(self, rhs: Motor) -> Self::Output {
        Point {
            x: self.wx.anti_wedge(rhs.xy) - self.xy.anti_wedge(rhs.wx),
            y: self.xy.anti_wedge(rhs.yw) - self.yw.anti_wedge(rhs.xy),
            w: -self.wx.anti_wedge(rhs.yw) + self.yw.anti_wedge(rhs.wx),
        }
    }
}

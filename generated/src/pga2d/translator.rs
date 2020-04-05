//! # Translator
//!
//! ## Operations
//! ```text
//! Translator.geometric(Translator) -> Translator
//! Translator.dot(Translator) -> Translator
//! Translator.wedge(Translator) -> Translator
//! Translator.anti_geometric(Dir) -> Rotor
//! Dir.anti_geometric(Translator) -> Rotor
//! Translator.dot(Dir) -> Point
//! Dir.dot(Translator) -> Point
//! Translator.anti_wedge(Dir) -> S
//! Dir.anti_wedge(Translator) -> S
//! Translator.anti_geometric(Point) -> Motor
//! Point.anti_geometric(Translator) -> Motor
//! Translator.dot(Point) -> Point
//! Point.dot(Translator) -> Point
//! Translator.anti_wedge(Point) -> S
//! Point.anti_wedge(Translator) -> S
//! Translator.geometric(Line) -> Line
//! Line.geometric(Translator) -> Line
//! Translator.dot(Line) -> Line
//! Line.dot(Translator) -> Line
//! Translator.wedge(Line) -> Line
//! Line.wedge(Translator) -> Line
//! Translator.anti_wedge(Line) -> Point
//! Line.anti_wedge(Translator) -> Point
//! Translator.geometric(Rotor) -> Motor
//! Rotor.geometric(Translator) -> Motor
//! Translator.anti_geometric(Rotor) -> Dir
//! Rotor.anti_geometric(Translator) -> Dir
//! Translator.dot(Rotor) -> Motor
//! Rotor.dot(Translator) -> Motor
//! Translator.wedge(Rotor) -> Motor
//! Rotor.wedge(Translator) -> Motor
//! Translator.anti_wedge(Rotor) -> Dir
//! Rotor.anti_wedge(Translator) -> Dir
//! Translator.geometric(Motor) -> Motor
//! Motor.geometric(Translator) -> Motor
//! Translator.dot(Motor) -> Motor
//! Motor.dot(Translator) -> Motor
//! Translator.wedge(Motor) -> Motor
//! Motor.wedge(Translator) -> Motor
//! Translator.anti_wedge(Motor) -> Point
//! Motor.anti_wedge(Translator) -> Point
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
            wx: self.wx,
        }
    }
}

impl AntiReverse for Translator {
    fn arev(self) -> Self {
        Translator {
            s: -self.s,
            yw: self.yw,
            wx: -self.wx,
        }
    }
}

// ---------------------------------------------------------------------
// Translator OP Dir:

// Omitted: Translator geometric Dir = self.s.geometric(rhs.x) + self.s.geometric(rhs.y) + self.wx.geometric(rhs.x) + self.wx.geometric(rhs.y) + self.yw.geometric(rhs.x) + self.yw.geometric(rhs.y)

// Translator.anti_geometric(Dir) -> Rotor
impl AntiGeometric<Dir> for Translator {
    type Output = Rotor;
    fn anti_geometric(self, rhs: Dir) -> Self::Output {
        Rotor {
            s: self.wx.anti_geometric(rhs.y) + self.yw.anti_geometric(rhs.x),
            xy: self.wx.anti_geometric(rhs.x) - self.yw.anti_geometric(rhs.y),
        }
    }
}

// Translator.dot(Dir) -> Point
impl Dot<Dir> for Translator {
    type Output = Point;
    fn dot(self, rhs: Dir) -> Self::Output {
        Point {
            x: self.s.dot(rhs.x),
            y: self.s.dot(rhs.y),
            w: self.wx.dot(rhs.x) - self.yw.dot(rhs.y),
        }
    }
}

// Omitted: Translator wedge Dir = self.s.wedge(rhs.x) + self.s.wedge(rhs.y) + self.wx.wedge(rhs.y) + self.yw.wedge(rhs.x)

// Translator.anti_wedge(Dir) -> S
impl AntiWedge<Dir> for Translator {
    type Output = S;
    fn anti_wedge(self, rhs: Dir) -> Self::Output {
        self.wx.anti_wedge(rhs.y) + self.yw.anti_wedge(rhs.x)
    }
}

// ---------------------------------------------------------------------
// Translator OP Point:

// Omitted: Translator geometric Point = self.s.geometric(rhs.w) + self.s.geometric(rhs.x) + self.s.geometric(rhs.y) + self.wx.geometric(rhs.x) + self.wx.geometric(rhs.y) + self.yw.geometric(rhs.x) + self.yw.geometric(rhs.y)

// Translator.anti_geometric(Point) -> Motor
impl AntiGeometric<Point> for Translator {
    type Output = Motor;
    fn anti_geometric(self, rhs: Point) -> Self::Output {
        Motor {
            s: self.wx.anti_geometric(rhs.y) + self.yw.anti_geometric(rhs.x),
            yw: -self.wx.anti_geometric(rhs.w),
            wx: -self.yw.anti_geometric(rhs.w),
            xy: -self.s.anti_geometric(rhs.w) + self.wx.anti_geometric(rhs.x)
                - self.yw.anti_geometric(rhs.y),
        }
    }
}

// Translator.dot(Point) -> Point
impl Dot<Point> for Translator {
    type Output = Point;
    fn dot(self, rhs: Point) -> Self::Output {
        Point {
            x: self.s.dot(rhs.x),
            y: self.s.dot(rhs.y),
            w: self.s.dot(rhs.w) + self.wx.dot(rhs.x) - self.yw.dot(rhs.y),
        }
    }
}

// Omitted: Translator wedge Point = self.s.wedge(rhs.w) + self.s.wedge(rhs.x) + self.s.wedge(rhs.y) + self.wx.wedge(rhs.y) + self.yw.wedge(rhs.x)

// Translator.anti_wedge(Point) -> S
impl AntiWedge<Point> for Translator {
    type Output = S;
    fn anti_wedge(self, rhs: Point) -> Self::Output {
        self.wx.anti_wedge(rhs.y) + self.yw.anti_wedge(rhs.x)
    }
}

// ---------------------------------------------------------------------
// Translator OP Line:

// Translator.geometric(Line) -> Line
impl Geometric<Line> for Translator {
    type Output = Line;
    fn geometric(self, rhs: Line) -> Self::Output {
        Line {
            yw: self.s.geometric(rhs.yw) - self.wx.geometric(rhs.xy),
            wx: -self.s.geometric(rhs.wx) - self.yw.geometric(rhs.xy),
            xy: self.s.geometric(rhs.xy),
        }
    }
}

// Omitted: Translator anti_geometric Line = self.s.anti_geometric(rhs.wx) + self.s.anti_geometric(rhs.yw) + self.wx.anti_geometric(rhs.wx) + self.wx.anti_geometric(rhs.xy) + self.wx.anti_geometric(rhs.yw) + self.yw.anti_geometric(rhs.wx) + self.yw.anti_geometric(rhs.xy) + self.yw.anti_geometric(rhs.yw)

// Translator.dot(Line) -> Line
impl Dot<Line> for Translator {
    type Output = Line;
    fn dot(self, rhs: Line) -> Self::Output {
        Line {
            yw: self.s.dot(rhs.yw),
            wx: -self.s.dot(rhs.wx),
            xy: self.s.dot(rhs.xy),
        }
    }
}

// Translator.wedge(Line) -> Line
impl Wedge<Line> for Translator {
    type Output = Line;
    fn wedge(self, rhs: Line) -> Self::Output {
        Line {
            yw: self.s.wedge(rhs.yw),
            wx: -self.s.wedge(rhs.wx),
            xy: self.s.wedge(rhs.xy),
        }
    }
}

// Translator.anti_wedge(Line) -> Point
impl AntiWedge<Line> for Translator {
    type Output = Point;
    fn anti_wedge(self, rhs: Line) -> Self::Output {
        Point {
            x: self.wx.anti_wedge(rhs.xy),
            y: -self.yw.anti_wedge(rhs.xy),
            w: -self.wx.anti_wedge(rhs.yw) + self.yw.anti_wedge(rhs.wx),
        }
    }
}

// ---------------------------------------------------------------------
// Translator OP Translator:

// Translator.geometric(Translator) -> Translator
impl Geometric<Translator> for Translator {
    type Output = Translator;
    fn geometric(self, rhs: Translator) -> Self::Output {
        Translator {
            s: self.s.geometric(rhs.s),
            yw: self.s.geometric(rhs.yw) + self.yw.geometric(rhs.s),
            wx: -self.s.geometric(rhs.wx) - self.wx.geometric(rhs.s),
        }
    }
}

// Omitted: Translator anti_geometric Translator = self.s.anti_geometric(rhs.wx) + self.s.anti_geometric(rhs.yw) + self.wx.anti_geometric(rhs.s) + self.wx.anti_geometric(rhs.wx) + self.wx.anti_geometric(rhs.yw) + self.yw.anti_geometric(rhs.s) + self.yw.anti_geometric(rhs.wx) + self.yw.anti_geometric(rhs.yw)

// Translator.dot(Translator) -> Translator
impl Dot<Translator> for Translator {
    type Output = Translator;
    fn dot(self, rhs: Translator) -> Self::Output {
        Translator {
            s: self.s.dot(rhs.s),
            yw: self.s.dot(rhs.yw) + self.yw.dot(rhs.s),
            wx: -self.s.dot(rhs.wx) - self.wx.dot(rhs.s),
        }
    }
}

// Translator.wedge(Translator) -> Translator
impl Wedge<Translator> for Translator {
    type Output = Translator;
    fn wedge(self, rhs: Translator) -> Self::Output {
        Translator {
            s: self.s.wedge(rhs.s),
            yw: self.s.wedge(rhs.yw) + self.yw.wedge(rhs.s),
            wx: -self.s.wedge(rhs.wx) - self.wx.wedge(rhs.s),
        }
    }
}

// Omitted: Translator anti_wedge Translator = self.wx.anti_wedge(rhs.yw) + self.yw.anti_wedge(rhs.wx)

// ---------------------------------------------------------------------
// Translator OP Rotor:

// Translator.geometric(Rotor) -> Motor
impl Geometric<Rotor> for Translator {
    type Output = Motor;
    fn geometric(self, rhs: Rotor) -> Self::Output {
        Motor {
            s: self.s.geometric(rhs.s),
            yw: -self.wx.geometric(rhs.xy) + self.yw.geometric(rhs.s),
            wx: -self.wx.geometric(rhs.s) - self.yw.geometric(rhs.xy),
            xy: self.s.geometric(rhs.xy),
        }
    }
}

// Translator.anti_geometric(Rotor) -> Dir
impl AntiGeometric<Rotor> for Translator {
    type Output = Dir;
    fn anti_geometric(self, rhs: Rotor) -> Self::Output {
        Dir {
            x: self.wx.anti_geometric(rhs.xy) + self.yw.anti_geometric(rhs.s),
            y: self.wx.anti_geometric(rhs.s) - self.yw.anti_geometric(rhs.xy),
        }
    }
}

// Translator.dot(Rotor) -> Motor
impl Dot<Rotor> for Translator {
    type Output = Motor;
    fn dot(self, rhs: Rotor) -> Self::Output {
        Motor {
            s: self.s.dot(rhs.s),
            yw: self.yw.dot(rhs.s),
            wx: -self.wx.dot(rhs.s),
            xy: self.s.dot(rhs.xy),
        }
    }
}

// Translator.wedge(Rotor) -> Motor
impl Wedge<Rotor> for Translator {
    type Output = Motor;
    fn wedge(self, rhs: Rotor) -> Self::Output {
        Motor {
            s: self.s.wedge(rhs.s),
            yw: self.yw.wedge(rhs.s),
            wx: -self.wx.wedge(rhs.s),
            xy: self.s.wedge(rhs.xy),
        }
    }
}

// Translator.anti_wedge(Rotor) -> Dir
impl AntiWedge<Rotor> for Translator {
    type Output = Dir;
    fn anti_wedge(self, rhs: Rotor) -> Self::Output {
        Dir {
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
        Motor {
            s: self.s.geometric(rhs.s),
            yw: self.s.geometric(rhs.yw) - self.wx.geometric(rhs.xy) + self.yw.geometric(rhs.s),
            wx: -self.s.geometric(rhs.wx) - self.wx.geometric(rhs.s) - self.yw.geometric(rhs.xy),
            xy: self.s.geometric(rhs.xy),
        }
    }
}

// Omitted: Translator anti_geometric Motor = self.s.anti_geometric(rhs.wx) + self.s.anti_geometric(rhs.yw) + self.wx.anti_geometric(rhs.s) + self.wx.anti_geometric(rhs.wx) + self.wx.anti_geometric(rhs.xy) + self.wx.anti_geometric(rhs.yw) + self.yw.anti_geometric(rhs.s) + self.yw.anti_geometric(rhs.wx) + self.yw.anti_geometric(rhs.xy) + self.yw.anti_geometric(rhs.yw)

// Translator.dot(Motor) -> Motor
impl Dot<Motor> for Translator {
    type Output = Motor;
    fn dot(self, rhs: Motor) -> Self::Output {
        Motor {
            s: self.s.dot(rhs.s),
            yw: self.s.dot(rhs.yw) + self.yw.dot(rhs.s),
            wx: -self.s.dot(rhs.wx) - self.wx.dot(rhs.s),
            xy: self.s.dot(rhs.xy),
        }
    }
}

// Translator.wedge(Motor) -> Motor
impl Wedge<Motor> for Translator {
    type Output = Motor;
    fn wedge(self, rhs: Motor) -> Self::Output {
        Motor {
            s: self.s.wedge(rhs.s),
            yw: self.s.wedge(rhs.yw) + self.yw.wedge(rhs.s),
            wx: -self.s.wedge(rhs.wx) - self.wx.wedge(rhs.s),
            xy: self.s.wedge(rhs.xy),
        }
    }
}

// Translator.anti_wedge(Motor) -> Point
impl AntiWedge<Motor> for Translator {
    type Output = Point;
    fn anti_wedge(self, rhs: Motor) -> Self::Output {
        Point {
            x: self.wx.anti_wedge(rhs.xy),
            y: -self.yw.anti_wedge(rhs.xy),
            w: -self.wx.anti_wedge(rhs.yw) + self.yw.anti_wedge(rhs.wx),
        }
    }
}

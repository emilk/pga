//! # Line
//!
//! ## Operations
//! ```text
//! Line.dot(Line) -> R
//! Line.anti_wedge(Line) -> Point
//! Line.dot(Point) -> Point
//! Point.dot(Line) -> Point
//! Line.wedge(Point) -> XYW
//! Point.wedge(Line) -> XYW
//! Line.anti_wedge(Point) -> R
//! Point.anti_wedge(Line) -> R
//! ```

use super::*;

#[derive(
    Copy, Clone, Debug, PartialEq, PartialOrd, derive_more::Neg, derive_more::Add, derive_more::Sub,
)]
pub struct Line {
    pub yw: YW,
    pub wx: WX,
    pub xy: XY,
}

// ---------------------------------------------------------------------
// Line OP Point:

// Omitted: Line geometric Point = self.wx.geometric(rhs.x) + self.wx.geometric(rhs.y) + self.xy.geometric(rhs.w) + self.xy.geometric(rhs.x) + self.xy.geometric(rhs.y) + self.yw.geometric(rhs.x) + self.yw.geometric(rhs.y)
// Omitted: Line anti_geometric Point = self.wx.anti_geometric(rhs.w) + self.wx.anti_geometric(rhs.x) + self.wx.anti_geometric(rhs.y) + self.xy.anti_geometric(rhs.w) + self.yw.anti_geometric(rhs.w) + self.yw.anti_geometric(rhs.x) + self.yw.anti_geometric(rhs.y)

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

// Line.anti_wedge(Point) -> R
impl AntiWedge<Point> for Line {
    type Output = R;
    fn anti_wedge(self, rhs: Point) -> Self::Output {
        self.wx.anti_wedge(rhs.y) + self.xy.anti_wedge(rhs.w) + self.yw.anti_wedge(rhs.x)
    }
}

// ---------------------------------------------------------------------
// Line OP Line:

// Omitted: Line geometric Line = self.wx.geometric(rhs.xy) + self.xy.geometric(rhs.wx) + self.xy.geometric(rhs.xy) + self.xy.geometric(rhs.yw) + self.yw.geometric(rhs.xy)
// Omitted: Line anti_geometric Line = self.wx.anti_geometric(rhs.wx) + self.wx.anti_geometric(rhs.xy) + self.wx.anti_geometric(rhs.yw) + self.xy.anti_geometric(rhs.wx) + self.xy.anti_geometric(rhs.yw) + self.yw.anti_geometric(rhs.wx) + self.yw.anti_geometric(rhs.xy) + self.yw.anti_geometric(rhs.yw)

// Line.dot(Line) -> R
impl Dot<Line> for Line {
    type Output = R;
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

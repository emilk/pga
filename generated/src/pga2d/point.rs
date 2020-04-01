//! # Point
//!
//! ## Operations
//! ```text
//! Point.wedge(Point) -> Line
//! Point.dot(Line) -> Point
//! Line.dot(Point) -> Point
//! ```

use super::*;

#[derive(
    Copy, Clone, Debug, PartialEq, PartialOrd, derive_more::Neg, derive_more::Add, derive_more::Sub,
)]
pub struct Point {
    pub x: X,
    pub y: Y,
    pub w: W,
}

// ---------------------------------------------------------------------
// Point OP Point:

// Omitted: Point geometric Point = self.w.geometric(rhs.x) + self.w.geometric(rhs.y) + self.x.geometric(rhs.w) + self.x.geometric(rhs.x) + self.x.geometric(rhs.y) + self.y.geometric(rhs.w) + self.y.geometric(rhs.x) + self.y.geometric(rhs.y)
// Omitted: Point anti_geometric Point = self.w.anti_geometric(rhs.w) + self.w.anti_geometric(rhs.x) + self.w.anti_geometric(rhs.y) + self.x.anti_geometric(rhs.w) + self.y.anti_geometric(rhs.w)
// Omitted: Point dot Point = self.x.dot(rhs.x) + self.y.dot(rhs.y)

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
// Omitted: Point anti_geometric Line = self.w.anti_geometric(rhs.wx) + self.w.anti_geometric(rhs.xy) + self.w.anti_geometric(rhs.yw) + self.x.anti_geometric(rhs.wx) + self.x.anti_geometric(rhs.yw) + self.y.anti_geometric(rhs.wx) + self.y.anti_geometric(rhs.yw)

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

// Omitted: Point wedge Line = self.w.wedge(rhs.xy) + self.x.wedge(rhs.yw) + self.y.wedge(rhs.wx)
// Omitted: Point anti_wedge Line = self.w.anti_wedge(rhs.xy) + self.x.anti_wedge(rhs.yw) + self.y.anti_wedge(rhs.wx)

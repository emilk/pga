use super::*;

pub struct Point {
    pub x: X,
    pub y: Y,
    pub w: W,
}

// ---------------------------------------------------------------------
// Point OP Point:

// Omitted: Point geometric Point = self.w * rhs.x + self.w * rhs.y + self.x * rhs.w + self.x * rhs.x + self.x * rhs.y + self.y * rhs.w + self.y * rhs.x + self.y * rhs.y
// Omitted: Point anti_geometric Point = self.w !* rhs.w + self.w !* rhs.x + self.w !* rhs.y + self.x !* rhs.w + self.y !* rhs.w
// Omitted: Point dot Point = self.x | rhs.x + self.y | rhs.y

impl Wedge<Point> for Point {
    type Output = Line;
    fn wedge(self, rhs: Point) -> Self::Output {
        Line {
            yw: -self.w ^ rhs.y + self.y ^ rhs.w,
            wx: -self.w ^ rhs.x + self.x ^ rhs.w,
            xy: self.x ^ rhs.y - self.y ^ rhs.x,
        }
    }
}

// Omitted: Point anti_wedge Point = 0

// ---------------------------------------------------------------------
// Point OP Line:

// Omitted: Point geometric Line = self.w * rhs.xy + self.x * rhs.wx + self.x * rhs.xy + self.x * rhs.yw + self.y * rhs.wx + self.y * rhs.xy + self.y * rhs.yw
// Omitted: Point anti_geometric Line = self.w !* rhs.wx + self.w !* rhs.xy + self.w !* rhs.yw + self.x !* rhs.wx + self.x !* rhs.yw + self.y !* rhs.wx + self.y !* rhs.yw

impl Dot<Line> for Point {
    type Output = Point;
    fn dot(self, rhs: Line) -> Self::Output {
        Point {
            x: -self.y | rhs.xy,
            y: self.x | rhs.xy,
            w: -self.x | rhs.wx + self.y | rhs.yw,
        }
    }
}

// Omitted: Point wedge Line = self.w ^ rhs.xy + self.x ^ rhs.yw + self.y ^ rhs.wx
// Omitted: Point anti_wedge Line = self.w & rhs.xy + self.x & rhs.yw + self.y & rhs.wx

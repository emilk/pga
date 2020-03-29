use super::*;

pub struct Line {
    pub yw: YW,
    pub wx: WX,
    pub xy: XY,
}

// ---------------------------------------------------------------------
// Line OP Point:

// Omitted: Line geometric Point = self.wx * rhs.x + self.wx * rhs.y + self.xy * rhs.w + self.xy * rhs.x + self.xy * rhs.y + self.yw * rhs.x + self.yw * rhs.y
// Omitted: Line anti_geometric Point = self.wx !* rhs.w + self.wx !* rhs.x + self.wx !* rhs.y + self.xy !* rhs.w + self.yw !* rhs.w + self.yw !* rhs.x + self.yw !* rhs.y

impl Dot<Point> for Line {
    type Output = Point;
    fn dot(self, rhs: Point) -> Self::Output {
        Point {
            x: self.xy | rhs.y,
            y: -self.xy | rhs.x,
            w: self.wx | rhs.x - self.yw | rhs.y,
        }
    }
}

// Omitted: Line wedge Point = self.wx ^ rhs.y + self.xy ^ rhs.w + self.yw ^ rhs.x
// Omitted: Line anti_wedge Point = self.wx & rhs.y + self.xy & rhs.w + self.yw & rhs.x

// ---------------------------------------------------------------------
// Line OP Line:

// Omitted: Line geometric Line = self.wx * rhs.xy + self.xy * rhs.wx + self.xy * rhs.xy + self.xy * rhs.yw + self.yw * rhs.xy
// Omitted: Line anti_geometric Line = self.wx !* rhs.wx + self.wx !* rhs.xy + self.wx !* rhs.yw + self.xy !* rhs.wx + self.xy !* rhs.yw + self.yw !* rhs.wx + self.yw !* rhs.xy + self.yw !* rhs.yw
// Omitted: Line dot Line = self.xy | rhs.xy
// Omitted: Line wedge Line = 0

impl AntiWedge<Line> for Line {
    type Output = Point;
    fn anti_wedge(self, rhs: Line) -> Self::Output {
        Point {
            x: self.wx & rhs.xy - self.xy & rhs.wx,
            y: self.xy & rhs.yw - self.yw & rhs.xy,
            w: -self.wx & rhs.yw + self.yw & rhs.wx,
        }
    }
}

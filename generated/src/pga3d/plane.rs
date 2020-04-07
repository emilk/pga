//! # Plane
//!
//! ## Operations
//! ```text
//! Plane.geometric(Plane) -> Motor
//! Plane.dot(Plane) -> S
//! Plane.anti_wedge(Plane) -> Line
//! Plane.dot(Dir) -> Line
//! Dir.dot(Plane) -> Line
//! Plane.wedge(Dir) -> XYZW
//! Dir.wedge(Plane) -> XYZW
//! Plane.anti_wedge(Dir) -> S
//! Dir.anti_wedge(Plane) -> S
//! Plane.dot(Point) -> Line
//! Point.dot(Plane) -> Line
//! Plane.wedge(Point) -> XYZW
//! Point.wedge(Plane) -> XYZW
//! Plane.anti_wedge(Point) -> S
//! Point.anti_wedge(Plane) -> S
//! Plane.dot(Moment) -> Point
//! Moment.dot(Plane) -> Point
//! Plane.anti_wedge(Moment) -> Dir
//! Moment.anti_wedge(Plane) -> Dir
//! Plane.dot(Line) -> Point
//! Line.dot(Plane) -> Point
//! Plane.anti_wedge(Line) -> Point
//! Line.anti_wedge(Plane) -> Point
//! Plane.dot(Translator) -> Point
//! Translator.dot(Plane) -> Point
//! Plane.dot(Rotor) -> W
//! Rotor.dot(Plane) -> W
//! Plane.wedge(Motor) -> Plane
//! Motor.wedge(Plane) -> Plane
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
pub struct Plane {
    pub nx: YZW,
    pub ny: ZXW,
    pub nz: XYW,
    pub d: ZYX,
}

// ---------------------------------------------------------------------

impl RCompl for Plane {
    type Output = Point;
    fn rcompl(self) -> Self::Output {
        Point {
            x: -self.nx.rcompl(),
            y: -self.ny.rcompl(),
            z: -self.nz.rcompl(),
            w: -self.d.rcompl(),
        }
    }
}

impl LCompl for Plane {
    type Output = Point;
    fn lcompl(self) -> Self::Output {
        Point {
            x: self.nx.lcompl(),
            y: self.ny.lcompl(),
            z: self.nz.lcompl(),
            w: self.d.lcompl(),
        }
    }
}

impl Reverse for Plane {
    fn rev(self) -> Self {
        Plane {
            nx: -self.nx,
            ny: -self.ny,
            nz: -self.nz,
            d: -self.d,
        }
    }
}

impl AntiReverse for Plane {
    fn arev(self) -> Self {
        Plane {
            nx: self.nx,
            ny: self.ny,
            nz: self.nz,
            d: self.d,
        }
    }
}

// ---------------------------------------------------------------------
// Plane OP Dir:

// Omitted: Plane geometric Dir = self.d.geometric(rhs.x) + self.d.geometric(rhs.y) + self.d.geometric(rhs.z) + self.nx.geometric(rhs.x) + self.nx.geometric(rhs.y) + self.nx.geometric(rhs.z) + self.ny.geometric(rhs.x) + self.ny.geometric(rhs.y) + self.ny.geometric(rhs.z) + self.nz.geometric(rhs.x) + self.nz.geometric(rhs.y) + self.nz.geometric(rhs.z)
// Omitted: Plane anti_geometric Dir = self.nx.anti_geometric(rhs.x) + self.nx.anti_geometric(rhs.y) + self.nx.anti_geometric(rhs.z) + self.ny.anti_geometric(rhs.x) + self.ny.anti_geometric(rhs.y) + self.ny.anti_geometric(rhs.z) + self.nz.anti_geometric(rhs.x) + self.nz.anti_geometric(rhs.y) + self.nz.anti_geometric(rhs.z)

// Plane.dot(Dir) -> Line
impl Dot<Dir> for Plane {
    type Output = Line;
    fn dot(self, rhs: Dir) -> Self::Output {
        Line {
            vx: -self.ny.dot(rhs.z) + self.nz.dot(rhs.y),
            vy: self.nx.dot(rhs.z) - self.nz.dot(rhs.x),
            vz: -self.nx.dot(rhs.y) + self.ny.dot(rhs.x),
            mx: -self.d.dot(rhs.x),
            my: -self.d.dot(rhs.y),
            mz: -self.d.dot(rhs.z),
        }
    }
}

// Plane.wedge(Dir) -> XYZW
impl Wedge<Dir> for Plane {
    type Output = XYZW;
    fn wedge(self, rhs: Dir) -> Self::Output {
        self.nx.wedge(rhs.x) + self.ny.wedge(rhs.y) + self.nz.wedge(rhs.z)
    }
}

// Plane.anti_wedge(Dir) -> S
impl AntiWedge<Dir> for Plane {
    type Output = S;
    fn anti_wedge(self, rhs: Dir) -> Self::Output {
        self.nx.anti_wedge(rhs.x) + self.ny.anti_wedge(rhs.y) + self.nz.anti_wedge(rhs.z)
    }
}

// ---------------------------------------------------------------------
// Plane OP Point:

// Omitted: Plane geometric Point = self.d.geometric(rhs.w) + self.d.geometric(rhs.x) + self.d.geometric(rhs.y) + self.d.geometric(rhs.z) + self.nx.geometric(rhs.x) + self.nx.geometric(rhs.y) + self.nx.geometric(rhs.z) + self.ny.geometric(rhs.x) + self.ny.geometric(rhs.y) + self.ny.geometric(rhs.z) + self.nz.geometric(rhs.x) + self.nz.geometric(rhs.y) + self.nz.geometric(rhs.z)
// Omitted: Plane anti_geometric Point = self.d.anti_geometric(rhs.w) + self.nx.anti_geometric(rhs.w) + self.nx.anti_geometric(rhs.x) + self.nx.anti_geometric(rhs.y) + self.nx.anti_geometric(rhs.z) + self.ny.anti_geometric(rhs.w) + self.ny.anti_geometric(rhs.x) + self.ny.anti_geometric(rhs.y) + self.ny.anti_geometric(rhs.z) + self.nz.anti_geometric(rhs.w) + self.nz.anti_geometric(rhs.x) + self.nz.anti_geometric(rhs.y) + self.nz.anti_geometric(rhs.z)

// Plane.dot(Point) -> Line
impl Dot<Point> for Plane {
    type Output = Line;
    fn dot(self, rhs: Point) -> Self::Output {
        Line {
            vx: -self.ny.dot(rhs.z) + self.nz.dot(rhs.y),
            vy: self.nx.dot(rhs.z) - self.nz.dot(rhs.x),
            vz: -self.nx.dot(rhs.y) + self.ny.dot(rhs.x),
            mx: -self.d.dot(rhs.x),
            my: -self.d.dot(rhs.y),
            mz: -self.d.dot(rhs.z),
        }
    }
}

// Plane.wedge(Point) -> XYZW
impl Wedge<Point> for Plane {
    type Output = XYZW;
    fn wedge(self, rhs: Point) -> Self::Output {
        self.d.wedge(rhs.w) + self.nx.wedge(rhs.x) + self.ny.wedge(rhs.y) + self.nz.wedge(rhs.z)
    }
}

// Plane.anti_wedge(Point) -> S
impl AntiWedge<Point> for Plane {
    type Output = S;
    fn anti_wedge(self, rhs: Point) -> Self::Output {
        self.d.anti_wedge(rhs.w)
            + self.nx.anti_wedge(rhs.x)
            + self.ny.anti_wedge(rhs.y)
            + self.nz.anti_wedge(rhs.z)
    }
}

// ---------------------------------------------------------------------
// Plane OP Moment:

// Omitted: Plane geometric Moment = self.d.geometric(rhs.xy) + self.d.geometric(rhs.yz) + self.d.geometric(rhs.zx) + self.nx.geometric(rhs.xy) + self.nx.geometric(rhs.yz) + self.nx.geometric(rhs.zx) + self.ny.geometric(rhs.xy) + self.ny.geometric(rhs.yz) + self.ny.geometric(rhs.zx) + self.nz.geometric(rhs.xy) + self.nz.geometric(rhs.yz) + self.nz.geometric(rhs.zx)
// Omitted: Plane anti_geometric Moment = self.nx.anti_geometric(rhs.xy) + self.nx.anti_geometric(rhs.yz) + self.nx.anti_geometric(rhs.zx) + self.ny.anti_geometric(rhs.xy) + self.ny.anti_geometric(rhs.yz) + self.ny.anti_geometric(rhs.zx) + self.nz.anti_geometric(rhs.xy) + self.nz.anti_geometric(rhs.yz) + self.nz.anti_geometric(rhs.zx)

// Plane.dot(Moment) -> Point
impl Dot<Moment> for Plane {
    type Output = Point;
    fn dot(self, rhs: Moment) -> Self::Output {
        Point {
            x: self.d.dot(rhs.yz),
            y: self.d.dot(rhs.zx),
            z: self.d.dot(rhs.xy),
            w: -self.nx.dot(rhs.yz) - self.ny.dot(rhs.zx) - self.nz.dot(rhs.xy),
        }
    }
}

// Omitted: Plane wedge Moment = 0

// Plane.anti_wedge(Moment) -> Dir
impl AntiWedge<Moment> for Plane {
    type Output = Dir;
    fn anti_wedge(self, rhs: Moment) -> Self::Output {
        Dir {
            x: -self.ny.anti_wedge(rhs.xy) + self.nz.anti_wedge(rhs.zx),
            y: self.nx.anti_wedge(rhs.xy) - self.nz.anti_wedge(rhs.yz),
            z: -self.nx.anti_wedge(rhs.zx) + self.ny.anti_wedge(rhs.yz),
        }
    }
}

// ---------------------------------------------------------------------
// Plane OP Line:

// Omitted: Plane geometric Line = self.d.geometric(rhs.mx) + self.d.geometric(rhs.my) + self.d.geometric(rhs.mz) + self.d.geometric(rhs.vx) + self.d.geometric(rhs.vy) + self.d.geometric(rhs.vz) + self.nx.geometric(rhs.mx) + self.nx.geometric(rhs.my) + self.nx.geometric(rhs.mz) + self.ny.geometric(rhs.mx) + self.ny.geometric(rhs.my) + self.ny.geometric(rhs.mz) + self.nz.geometric(rhs.mx) + self.nz.geometric(rhs.my) + self.nz.geometric(rhs.mz)
// Omitted: Plane anti_geometric Line = self.d.anti_geometric(rhs.vx) + self.d.anti_geometric(rhs.vy) + self.d.anti_geometric(rhs.vz) + self.nx.anti_geometric(rhs.mx) + self.nx.anti_geometric(rhs.my) + self.nx.anti_geometric(rhs.mz) + self.nx.anti_geometric(rhs.vx) + self.nx.anti_geometric(rhs.vy) + self.nx.anti_geometric(rhs.vz) + self.ny.anti_geometric(rhs.mx) + self.ny.anti_geometric(rhs.my) + self.ny.anti_geometric(rhs.mz) + self.ny.anti_geometric(rhs.vx) + self.ny.anti_geometric(rhs.vy) + self.ny.anti_geometric(rhs.vz) + self.nz.anti_geometric(rhs.mx) + self.nz.anti_geometric(rhs.my) + self.nz.anti_geometric(rhs.mz) + self.nz.anti_geometric(rhs.vx) + self.nz.anti_geometric(rhs.vy) + self.nz.anti_geometric(rhs.vz)

// Plane.dot(Line) -> Point
impl Dot<Line> for Plane {
    type Output = Point;
    fn dot(self, rhs: Line) -> Self::Output {
        Point {
            x: self.d.dot(rhs.mx),
            y: self.d.dot(rhs.my),
            z: self.d.dot(rhs.mz),
            w: -self.nx.dot(rhs.mx) - self.ny.dot(rhs.my) - self.nz.dot(rhs.mz),
        }
    }
}

// Omitted: Plane wedge Line = 0

// Plane.anti_wedge(Line) -> Point
impl AntiWedge<Line> for Plane {
    type Output = Point;
    fn anti_wedge(self, rhs: Line) -> Self::Output {
        Point {
            x: self.d.anti_wedge(rhs.vx) - self.ny.anti_wedge(rhs.mz) + self.nz.anti_wedge(rhs.my),
            y: self.d.anti_wedge(rhs.vy) + self.nx.anti_wedge(rhs.mz) - self.nz.anti_wedge(rhs.mx),
            z: self.d.anti_wedge(rhs.vz) - self.nx.anti_wedge(rhs.my) + self.ny.anti_wedge(rhs.mx),
            w: -self.nx.anti_wedge(rhs.vx)
                - self.ny.anti_wedge(rhs.vy)
                - self.nz.anti_wedge(rhs.vz),
        }
    }
}

// ---------------------------------------------------------------------
// Plane OP Plane:

// Plane.geometric(Plane) -> Motor
impl Geometric<Plane> for Plane {
    type Output = Motor;
    fn geometric(self, rhs: Plane) -> Self::Output {
        Motor {
            rx: -self.d.geometric(rhs.nx) + self.nx.geometric(rhs.d),
            ry: -self.d.geometric(rhs.ny) + self.ny.geometric(rhs.d),
            rz: -self.d.geometric(rhs.nz) + self.nz.geometric(rhs.d),
            rw: Default::default(),
            ux: Default::default(),
            uy: Default::default(),
            uz: Default::default(),
            uw: -self.d.geometric(rhs.d),
        }
    }
}

// Omitted: Plane anti_geometric Plane = self.d.anti_geometric(rhs.nx) + self.d.anti_geometric(rhs.ny) + self.d.anti_geometric(rhs.nz) + self.nx.anti_geometric(rhs.d) + self.nx.anti_geometric(rhs.nx) + self.nx.anti_geometric(rhs.ny) + self.nx.anti_geometric(rhs.nz) + self.ny.anti_geometric(rhs.d) + self.ny.anti_geometric(rhs.nx) + self.ny.anti_geometric(rhs.ny) + self.ny.anti_geometric(rhs.nz) + self.nz.anti_geometric(rhs.d) + self.nz.anti_geometric(rhs.nx) + self.nz.anti_geometric(rhs.ny) + self.nz.anti_geometric(rhs.nz)

// Plane.dot(Plane) -> S
impl Dot<Plane> for Plane {
    type Output = S;
    fn dot(self, rhs: Plane) -> Self::Output {
        self.d.dot(rhs.d)
    }
}

// Omitted: Plane wedge Plane = 0

// Plane.anti_wedge(Plane) -> Line
impl AntiWedge<Plane> for Plane {
    type Output = Line;
    fn anti_wedge(self, rhs: Plane) -> Self::Output {
        Line {
            vx: -self.ny.anti_wedge(rhs.nz) + self.nz.anti_wedge(rhs.ny),
            vy: self.nx.anti_wedge(rhs.nz) - self.nz.anti_wedge(rhs.nx),
            vz: -self.nx.anti_wedge(rhs.ny) + self.ny.anti_wedge(rhs.nx),
            mx: -self.d.anti_wedge(rhs.nx) + self.nx.anti_wedge(rhs.d),
            my: -self.d.anti_wedge(rhs.ny) + self.ny.anti_wedge(rhs.d),
            mz: -self.d.anti_wedge(rhs.nz) + self.nz.anti_wedge(rhs.d),
        }
    }
}

// ---------------------------------------------------------------------
// Plane OP Translator:

// Omitted: Plane geometric Translator = self.d.geometric(rhs.x) + self.d.geometric(rhs.xyzw) + self.d.geometric(rhs.y) + self.d.geometric(rhs.z) + self.nx.geometric(rhs.x) + self.nx.geometric(rhs.y) + self.nx.geometric(rhs.z) + self.ny.geometric(rhs.x) + self.ny.geometric(rhs.y) + self.ny.geometric(rhs.z) + self.nz.geometric(rhs.x) + self.nz.geometric(rhs.y) + self.nz.geometric(rhs.z)
// Omitted: Plane anti_geometric Translator = self.d.anti_geometric(rhs.xyzw) + self.nx.anti_geometric(rhs.x) + self.nx.anti_geometric(rhs.xyzw) + self.nx.anti_geometric(rhs.y) + self.nx.anti_geometric(rhs.z) + self.ny.anti_geometric(rhs.x) + self.ny.anti_geometric(rhs.xyzw) + self.ny.anti_geometric(rhs.y) + self.ny.anti_geometric(rhs.z) + self.nz.anti_geometric(rhs.x) + self.nz.anti_geometric(rhs.xyzw) + self.nz.anti_geometric(rhs.y) + self.nz.anti_geometric(rhs.z)

// Plane.dot(Translator) -> Point
impl Dot<Translator> for Plane {
    type Output = Point;
    fn dot(self, rhs: Translator) -> Self::Output {
        Point {
            x: self.d.dot(rhs.x),
            y: self.d.dot(rhs.y),
            z: self.d.dot(rhs.z),
            w: self.d.dot(rhs.xyzw) - self.nx.dot(rhs.x) - self.ny.dot(rhs.y) - self.nz.dot(rhs.z),
        }
    }
}

// Omitted: Plane wedge Translator = 0
// Omitted: Plane anti_wedge Translator = self.d.anti_wedge(rhs.xyzw) + self.nx.anti_wedge(rhs.xyzw) + self.nx.anti_wedge(rhs.y) + self.nx.anti_wedge(rhs.z) + self.ny.anti_wedge(rhs.x) + self.ny.anti_wedge(rhs.xyzw) + self.ny.anti_wedge(rhs.z) + self.nz.anti_wedge(rhs.x) + self.nz.anti_wedge(rhs.xyzw) + self.nz.anti_wedge(rhs.y)

// ---------------------------------------------------------------------
// Plane OP Rotor:

// Omitted: Plane geometric Rotor = self.d.geometric(rhs.w) + self.d.geometric(rhs.x) + self.d.geometric(rhs.y) + self.d.geometric(rhs.z)
// Omitted: Plane anti_geometric Rotor = self.d.anti_geometric(rhs.w) + self.d.anti_geometric(rhs.x) + self.d.anti_geometric(rhs.y) + self.d.anti_geometric(rhs.z) + self.nx.anti_geometric(rhs.w) + self.nx.anti_geometric(rhs.x) + self.nx.anti_geometric(rhs.y) + self.nx.anti_geometric(rhs.z) + self.ny.anti_geometric(rhs.w) + self.ny.anti_geometric(rhs.x) + self.ny.anti_geometric(rhs.y) + self.ny.anti_geometric(rhs.z) + self.nz.anti_geometric(rhs.w) + self.nz.anti_geometric(rhs.x) + self.nz.anti_geometric(rhs.y) + self.nz.anti_geometric(rhs.z)

// Plane.dot(Rotor) -> W
impl Dot<Rotor> for Plane {
    type Output = W;
    fn dot(self, rhs: Rotor) -> Self::Output {
        self.d.dot(rhs.w)
    }
}

// Omitted: Plane wedge Rotor = 0
// Omitted: Plane anti_wedge Rotor = self.d.anti_wedge(rhs.w) + self.d.anti_wedge(rhs.x) + self.d.anti_wedge(rhs.y) + self.d.anti_wedge(rhs.z) + self.nx.anti_wedge(rhs.w) + self.nx.anti_wedge(rhs.x) + self.ny.anti_wedge(rhs.w) + self.ny.anti_wedge(rhs.y) + self.nz.anti_wedge(rhs.w) + self.nz.anti_wedge(rhs.z)

// ---------------------------------------------------------------------
// Plane OP Motor:

// Omitted: Plane geometric Motor = self.d.geometric(rhs.rw) + self.d.geometric(rhs.rx) + self.d.geometric(rhs.ry) + self.d.geometric(rhs.rz) + self.d.geometric(rhs.uw) + self.d.geometric(rhs.ux) + self.d.geometric(rhs.uy) + self.d.geometric(rhs.uz) + self.nx.geometric(rhs.uw) + self.ny.geometric(rhs.uw) + self.nz.geometric(rhs.uw)
// Omitted: Plane anti_geometric Motor = self.d.anti_geometric(rhs.rw) + self.d.anti_geometric(rhs.rx) + self.d.anti_geometric(rhs.ry) + self.d.anti_geometric(rhs.rz) + self.d.anti_geometric(rhs.ux) + self.d.anti_geometric(rhs.uy) + self.d.anti_geometric(rhs.uz) + self.nx.anti_geometric(rhs.rw) + self.nx.anti_geometric(rhs.rx) + self.nx.anti_geometric(rhs.ry) + self.nx.anti_geometric(rhs.rz) + self.nx.anti_geometric(rhs.uw) + self.nx.anti_geometric(rhs.ux) + self.nx.anti_geometric(rhs.uy) + self.nx.anti_geometric(rhs.uz) + self.ny.anti_geometric(rhs.rw) + self.ny.anti_geometric(rhs.rx) + self.ny.anti_geometric(rhs.ry) + self.ny.anti_geometric(rhs.rz) + self.ny.anti_geometric(rhs.uw) + self.ny.anti_geometric(rhs.ux) + self.ny.anti_geometric(rhs.uy) + self.ny.anti_geometric(rhs.uz) + self.nz.anti_geometric(rhs.rw) + self.nz.anti_geometric(rhs.rx) + self.nz.anti_geometric(rhs.ry) + self.nz.anti_geometric(rhs.rz) + self.nz.anti_geometric(rhs.uw) + self.nz.anti_geometric(rhs.ux) + self.nz.anti_geometric(rhs.uy) + self.nz.anti_geometric(rhs.uz)
// Omitted: Plane dot Motor = self.d.dot(rhs.rw) + self.d.dot(rhs.uw) + self.nx.dot(rhs.uw) + self.ny.dot(rhs.uw) + self.nz.dot(rhs.uw)

// Plane.wedge(Motor) -> Plane
impl Wedge<Motor> for Plane {
    type Output = Plane;
    fn wedge(self, rhs: Motor) -> Self::Output {
        Plane {
            nx: self.nx.wedge(rhs.uw),
            ny: self.ny.wedge(rhs.uw),
            nz: self.nz.wedge(rhs.uw),
            d: self.d.wedge(rhs.uw),
        }
    }
}

// Omitted: Plane anti_wedge Motor = self.d.anti_wedge(rhs.rw) + self.d.anti_wedge(rhs.rx) + self.d.anti_wedge(rhs.ry) + self.d.anti_wedge(rhs.rz) + self.d.anti_wedge(rhs.ux) + self.d.anti_wedge(rhs.uy) + self.d.anti_wedge(rhs.uz) + self.nx.anti_wedge(rhs.rw) + self.nx.anti_wedge(rhs.rx) + self.nx.anti_wedge(rhs.uy) + self.nx.anti_wedge(rhs.uz) + self.ny.anti_wedge(rhs.rw) + self.ny.anti_wedge(rhs.ry) + self.ny.anti_wedge(rhs.ux) + self.ny.anti_wedge(rhs.uz) + self.nz.anti_wedge(rhs.rw) + self.nz.anti_wedge(rhs.rz) + self.nz.anti_wedge(rhs.ux) + self.nz.anti_wedge(rhs.uy)

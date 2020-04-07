//! # Dir
//!
//! ## Operations
//! ```text
//! Dir.dot(Dir) -> S
//! Dir.wedge(Dir) -> Moment
//! Dir.anti_geometric(Point) -> Moment
//! Point.anti_geometric(Dir) -> Moment
//! Dir.dot(Point) -> S
//! Point.dot(Dir) -> S
//! Dir.wedge(Point) -> Line
//! Point.wedge(Dir) -> Line
//! Dir.dot(Moment) -> Dir
//! Moment.dot(Dir) -> Dir
//! Dir.wedge(Moment) -> ZYX
//! Moment.wedge(Dir) -> ZYX
//! Dir.dot(Line) -> Point
//! Line.dot(Dir) -> Point
//! Dir.wedge(Line) -> Plane
//! Line.wedge(Dir) -> Plane
//! Dir.dot(Plane) -> Line
//! Plane.dot(Dir) -> Line
//! Dir.wedge(Plane) -> XYZW
//! Plane.wedge(Dir) -> XYZW
//! Dir.anti_wedge(Plane) -> S
//! Plane.anti_wedge(Dir) -> S
//! Dir.anti_geometric(Translator) -> Dir
//! Translator.anti_geometric(Dir) -> Dir
//! Dir.wedge(Translator) -> ZYX
//! Translator.wedge(Dir) -> ZYX
//! Dir.anti_wedge(Translator) -> Dir
//! Translator.anti_wedge(Dir) -> Dir
//! Dir.wedge(Rotor) -> Plane
//! Rotor.wedge(Dir) -> Plane
//! Dir.anti_wedge(Rotor) -> Dir
//! Rotor.anti_wedge(Dir) -> Dir
//!
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
    pub z: Z,
}

// ---------------------------------------------------------------------

impl RCompl for Dir {
    type Output = Plane;
    fn rcompl(self) -> Self::Output {
        Plane {
            nx: self.x.rcompl(),
            ny: self.y.rcompl(),
            nz: self.z.rcompl(),
            d: Default::default(),
        }
    }
}

impl LCompl for Dir {
    type Output = Plane;
    fn lcompl(self) -> Self::Output {
        Plane {
            nx: -self.x.lcompl(),
            ny: -self.y.lcompl(),
            nz: -self.z.lcompl(),
            d: Default::default(),
        }
    }
}

impl Reverse for Dir {
    fn rev(self) -> Self {
        Dir {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl AntiReverse for Dir {
    fn arev(self) -> Self {
        Dir {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

// ---------------------------------------------------------------------
// Dir OP Dir:

// Omitted: Dir geometric Dir = self.x.geometric(rhs.x) + self.x.geometric(rhs.y) + self.x.geometric(rhs.z) + self.y.geometric(rhs.x) + self.y.geometric(rhs.y) + self.y.geometric(rhs.z) + self.z.geometric(rhs.x) + self.z.geometric(rhs.y) + self.z.geometric(rhs.z)
// Omitted: Dir anti_geometric Dir = 0

// Dir.dot(Dir) -> S
impl Dot<Dir> for Dir {
    type Output = S;
    fn dot(self, rhs: Dir) -> Self::Output {
        self.x.dot(rhs.x) + self.y.dot(rhs.y) + self.z.dot(rhs.z)
    }
}

// Dir.wedge(Dir) -> Moment
impl Wedge<Dir> for Dir {
    type Output = Moment;
    fn wedge(self, rhs: Dir) -> Self::Output {
        Moment {
            yz: self.y.wedge(rhs.z) - self.z.wedge(rhs.y),
            zx: -self.x.wedge(rhs.z) + self.z.wedge(rhs.x),
            xy: self.x.wedge(rhs.y) - self.y.wedge(rhs.x),
        }
    }
}

// Omitted: Dir anti_wedge Dir = 0

// ---------------------------------------------------------------------
// Dir OP Point:

// Omitted: Dir geometric Point = self.x.geometric(rhs.w) + self.x.geometric(rhs.x) + self.x.geometric(rhs.y) + self.x.geometric(rhs.z) + self.y.geometric(rhs.w) + self.y.geometric(rhs.x) + self.y.geometric(rhs.y) + self.y.geometric(rhs.z) + self.z.geometric(rhs.w) + self.z.geometric(rhs.x) + self.z.geometric(rhs.y) + self.z.geometric(rhs.z)

// Dir.anti_geometric(Point) -> Moment
impl AntiGeometric<Point> for Dir {
    type Output = Moment;
    fn anti_geometric(self, rhs: Point) -> Self::Output {
        Moment {
            yz: -self.x.anti_geometric(rhs.w),
            zx: -self.y.anti_geometric(rhs.w),
            xy: -self.z.anti_geometric(rhs.w),
        }
    }
}

// Dir.dot(Point) -> S
impl Dot<Point> for Dir {
    type Output = S;
    fn dot(self, rhs: Point) -> Self::Output {
        self.x.dot(rhs.x) + self.y.dot(rhs.y) + self.z.dot(rhs.z)
    }
}

// Dir.wedge(Point) -> Line
impl Wedge<Point> for Dir {
    type Output = Line;
    fn wedge(self, rhs: Point) -> Self::Output {
        Line {
            vx: -self.x.wedge(rhs.w),
            vy: -self.y.wedge(rhs.w),
            vz: -self.z.wedge(rhs.w),
            mx: self.y.wedge(rhs.z) - self.z.wedge(rhs.y),
            my: -self.x.wedge(rhs.z) + self.z.wedge(rhs.x),
            mz: self.x.wedge(rhs.y) - self.y.wedge(rhs.x),
        }
    }
}

// Omitted: Dir anti_wedge Point = 0

// ---------------------------------------------------------------------
// Dir OP Moment:

// Omitted: Dir geometric Moment = self.x.geometric(rhs.xy) + self.x.geometric(rhs.yz) + self.x.geometric(rhs.zx) + self.y.geometric(rhs.xy) + self.y.geometric(rhs.yz) + self.y.geometric(rhs.zx) + self.z.geometric(rhs.xy) + self.z.geometric(rhs.yz) + self.z.geometric(rhs.zx)
// Omitted: Dir anti_geometric Moment = 0

// Dir.dot(Moment) -> Dir
impl Dot<Moment> for Dir {
    type Output = Dir;
    fn dot(self, rhs: Moment) -> Self::Output {
        Dir {
            x: -self.y.dot(rhs.xy) + self.z.dot(rhs.zx),
            y: self.x.dot(rhs.xy) - self.z.dot(rhs.yz),
            z: -self.x.dot(rhs.zx) + self.y.dot(rhs.yz),
        }
    }
}

// Dir.wedge(Moment) -> ZYX
impl Wedge<Moment> for Dir {
    type Output = ZYX;
    fn wedge(self, rhs: Moment) -> Self::Output {
        self.x.wedge(rhs.yz) + self.y.wedge(rhs.zx) + self.z.wedge(rhs.xy)
    }
}

// Omitted: Dir anti_wedge Moment = 0

// ---------------------------------------------------------------------
// Dir OP Line:

// Omitted: Dir geometric Line = self.x.geometric(rhs.mx) + self.x.geometric(rhs.my) + self.x.geometric(rhs.mz) + self.x.geometric(rhs.vx) + self.x.geometric(rhs.vy) + self.x.geometric(rhs.vz) + self.y.geometric(rhs.mx) + self.y.geometric(rhs.my) + self.y.geometric(rhs.mz) + self.y.geometric(rhs.vx) + self.y.geometric(rhs.vy) + self.y.geometric(rhs.vz) + self.z.geometric(rhs.mx) + self.z.geometric(rhs.my) + self.z.geometric(rhs.mz) + self.z.geometric(rhs.vx) + self.z.geometric(rhs.vy) + self.z.geometric(rhs.vz)
// Omitted: Dir anti_geometric Line = self.x.anti_geometric(rhs.vx) + self.x.anti_geometric(rhs.vy) + self.x.anti_geometric(rhs.vz) + self.y.anti_geometric(rhs.vx) + self.y.anti_geometric(rhs.vy) + self.y.anti_geometric(rhs.vz) + self.z.anti_geometric(rhs.vx) + self.z.anti_geometric(rhs.vy) + self.z.anti_geometric(rhs.vz)

// Dir.dot(Line) -> Point
impl Dot<Line> for Dir {
    type Output = Point;
    fn dot(self, rhs: Line) -> Self::Output {
        Point {
            x: -self.y.dot(rhs.mz) + self.z.dot(rhs.my),
            y: self.x.dot(rhs.mz) - self.z.dot(rhs.mx),
            z: -self.x.dot(rhs.my) + self.y.dot(rhs.mx),
            w: -self.x.dot(rhs.vx) - self.y.dot(rhs.vy) - self.z.dot(rhs.vz),
        }
    }
}

// Dir.wedge(Line) -> Plane
impl Wedge<Line> for Dir {
    type Output = Plane;
    fn wedge(self, rhs: Line) -> Self::Output {
        Plane {
            nx: -self.y.wedge(rhs.vz) + self.z.wedge(rhs.vy),
            ny: self.x.wedge(rhs.vz) - self.z.wedge(rhs.vx),
            nz: -self.x.wedge(rhs.vy) + self.y.wedge(rhs.vx),
            d: -self.x.wedge(rhs.mx) - self.y.wedge(rhs.my) - self.z.wedge(rhs.mz),
        }
    }
}

// Omitted: Dir anti_wedge Line = 0

// ---------------------------------------------------------------------
// Dir OP Plane:

// Omitted: Dir geometric Plane = self.x.geometric(rhs.d) + self.x.geometric(rhs.nx) + self.x.geometric(rhs.ny) + self.x.geometric(rhs.nz) + self.y.geometric(rhs.d) + self.y.geometric(rhs.nx) + self.y.geometric(rhs.ny) + self.y.geometric(rhs.nz) + self.z.geometric(rhs.d) + self.z.geometric(rhs.nx) + self.z.geometric(rhs.ny) + self.z.geometric(rhs.nz)
// Omitted: Dir anti_geometric Plane = self.x.anti_geometric(rhs.nx) + self.x.anti_geometric(rhs.ny) + self.x.anti_geometric(rhs.nz) + self.y.anti_geometric(rhs.nx) + self.y.anti_geometric(rhs.ny) + self.y.anti_geometric(rhs.nz) + self.z.anti_geometric(rhs.nx) + self.z.anti_geometric(rhs.ny) + self.z.anti_geometric(rhs.nz)

// Dir.dot(Plane) -> Line
impl Dot<Plane> for Dir {
    type Output = Line;
    fn dot(self, rhs: Plane) -> Self::Output {
        Line {
            vx: self.y.dot(rhs.nz) - self.z.dot(rhs.ny),
            vy: -self.x.dot(rhs.nz) + self.z.dot(rhs.nx),
            vz: self.x.dot(rhs.ny) - self.y.dot(rhs.nx),
            mx: -self.x.dot(rhs.d),
            my: -self.y.dot(rhs.d),
            mz: -self.z.dot(rhs.d),
        }
    }
}

// Dir.wedge(Plane) -> XYZW
impl Wedge<Plane> for Dir {
    type Output = XYZW;
    fn wedge(self, rhs: Plane) -> Self::Output {
        self.x.wedge(rhs.nx) + self.y.wedge(rhs.ny) + self.z.wedge(rhs.nz)
    }
}

// Dir.anti_wedge(Plane) -> S
impl AntiWedge<Plane> for Dir {
    type Output = S;
    fn anti_wedge(self, rhs: Plane) -> Self::Output {
        self.x.anti_wedge(rhs.nx) + self.y.anti_wedge(rhs.ny) + self.z.anti_wedge(rhs.nz)
    }
}

// ---------------------------------------------------------------------
// Dir OP Translator:

// Omitted: Dir geometric Translator = self.x.geometric(rhs.x) + self.x.geometric(rhs.xyzw) + self.x.geometric(rhs.y) + self.x.geometric(rhs.z) + self.y.geometric(rhs.x) + self.y.geometric(rhs.xyzw) + self.y.geometric(rhs.y) + self.y.geometric(rhs.z) + self.z.geometric(rhs.x) + self.z.geometric(rhs.xyzw) + self.z.geometric(rhs.y) + self.z.geometric(rhs.z)

// Dir.anti_geometric(Translator) -> Dir
impl AntiGeometric<Translator> for Dir {
    type Output = Dir;
    fn anti_geometric(self, rhs: Translator) -> Self::Output {
        Dir {
            x: self.x.anti_geometric(rhs.xyzw),
            y: self.y.anti_geometric(rhs.xyzw),
            z: self.z.anti_geometric(rhs.xyzw),
        }
    }
}

// Omitted: Dir dot Translator = self.x.dot(rhs.xyzw) + self.x.dot(rhs.y) + self.x.dot(rhs.z) + self.y.dot(rhs.x) + self.y.dot(rhs.xyzw) + self.y.dot(rhs.z) + self.z.dot(rhs.x) + self.z.dot(rhs.xyzw) + self.z.dot(rhs.y)

// Dir.wedge(Translator) -> ZYX
impl Wedge<Translator> for Dir {
    type Output = ZYX;
    fn wedge(self, rhs: Translator) -> Self::Output {
        self.x.wedge(rhs.x) + self.y.wedge(rhs.y) + self.z.wedge(rhs.z)
    }
}

// Dir.anti_wedge(Translator) -> Dir
impl AntiWedge<Translator> for Dir {
    type Output = Dir;
    fn anti_wedge(self, rhs: Translator) -> Self::Output {
        Dir {
            x: self.x.anti_wedge(rhs.xyzw),
            y: self.y.anti_wedge(rhs.xyzw),
            z: self.z.anti_wedge(rhs.xyzw),
        }
    }
}

// ---------------------------------------------------------------------
// Dir OP Rotor:

// Omitted: Dir geometric Rotor = self.x.geometric(rhs.w) + self.x.geometric(rhs.x) + self.x.geometric(rhs.y) + self.x.geometric(rhs.z) + self.y.geometric(rhs.w) + self.y.geometric(rhs.x) + self.y.geometric(rhs.y) + self.y.geometric(rhs.z) + self.z.geometric(rhs.w) + self.z.geometric(rhs.x) + self.z.geometric(rhs.y) + self.z.geometric(rhs.z)
// Omitted: Dir anti_geometric Rotor = self.x.anti_geometric(rhs.w) + self.x.anti_geometric(rhs.x) + self.x.anti_geometric(rhs.y) + self.x.anti_geometric(rhs.z) + self.y.anti_geometric(rhs.w) + self.y.anti_geometric(rhs.x) + self.y.anti_geometric(rhs.y) + self.y.anti_geometric(rhs.z) + self.z.anti_geometric(rhs.w) + self.z.anti_geometric(rhs.x) + self.z.anti_geometric(rhs.y) + self.z.anti_geometric(rhs.z)
// Omitted: Dir dot Rotor = self.x.dot(rhs.w) + self.x.dot(rhs.x) + self.y.dot(rhs.w) + self.y.dot(rhs.y) + self.z.dot(rhs.w) + self.z.dot(rhs.z)

// Dir.wedge(Rotor) -> Plane
impl Wedge<Rotor> for Dir {
    type Output = Plane;
    fn wedge(self, rhs: Rotor) -> Self::Output {
        Plane {
            nx: -self.y.wedge(rhs.z) + self.z.wedge(rhs.y),
            ny: self.x.wedge(rhs.z) - self.z.wedge(rhs.x),
            nz: -self.x.wedge(rhs.y) + self.y.wedge(rhs.x),
            d: Default::default(),
        }
    }
}

// Dir.anti_wedge(Rotor) -> Dir
impl AntiWedge<Rotor> for Dir {
    type Output = Dir;
    fn anti_wedge(self, rhs: Rotor) -> Self::Output {
        Dir {
            x: self.x.anti_wedge(rhs.w),
            y: self.y.anti_wedge(rhs.w),
            z: self.z.anti_wedge(rhs.w),
        }
    }
}

// ---------------------------------------------------------------------
// Dir OP Motor:

// Omitted: Dir geometric Motor = self.x.geometric(rhs.rw) + self.x.geometric(rhs.rx) + self.x.geometric(rhs.ry) + self.x.geometric(rhs.rz) + self.x.geometric(rhs.uw) + self.x.geometric(rhs.ux) + self.x.geometric(rhs.uy) + self.x.geometric(rhs.uz) + self.y.geometric(rhs.rw) + self.y.geometric(rhs.rx) + self.y.geometric(rhs.ry) + self.y.geometric(rhs.rz) + self.y.geometric(rhs.uw) + self.y.geometric(rhs.ux) + self.y.geometric(rhs.uy) + self.y.geometric(rhs.uz) + self.z.geometric(rhs.rw) + self.z.geometric(rhs.rx) + self.z.geometric(rhs.ry) + self.z.geometric(rhs.rz) + self.z.geometric(rhs.uw) + self.z.geometric(rhs.ux) + self.z.geometric(rhs.uy) + self.z.geometric(rhs.uz)
// Omitted: Dir anti_geometric Motor = self.x.anti_geometric(rhs.rw) + self.x.anti_geometric(rhs.rx) + self.x.anti_geometric(rhs.ry) + self.x.anti_geometric(rhs.rz) + self.x.anti_geometric(rhs.ux) + self.x.anti_geometric(rhs.uy) + self.x.anti_geometric(rhs.uz) + self.y.anti_geometric(rhs.rw) + self.y.anti_geometric(rhs.rx) + self.y.anti_geometric(rhs.ry) + self.y.anti_geometric(rhs.rz) + self.y.anti_geometric(rhs.ux) + self.y.anti_geometric(rhs.uy) + self.y.anti_geometric(rhs.uz) + self.z.anti_geometric(rhs.rw) + self.z.anti_geometric(rhs.rx) + self.z.anti_geometric(rhs.ry) + self.z.anti_geometric(rhs.rz) + self.z.anti_geometric(rhs.ux) + self.z.anti_geometric(rhs.uy) + self.z.anti_geometric(rhs.uz)
// Omitted: Dir dot Motor = self.x.dot(rhs.rw) + self.x.dot(rhs.rx) + self.x.dot(rhs.uw) + self.x.dot(rhs.uy) + self.x.dot(rhs.uz) + self.y.dot(rhs.rw) + self.y.dot(rhs.ry) + self.y.dot(rhs.uw) + self.y.dot(rhs.ux) + self.y.dot(rhs.uz) + self.z.dot(rhs.rw) + self.z.dot(rhs.rz) + self.z.dot(rhs.uw) + self.z.dot(rhs.ux) + self.z.dot(rhs.uy)
// Omitted: Dir wedge Motor = self.x.wedge(rhs.ry) + self.x.wedge(rhs.rz) + self.x.wedge(rhs.uw) + self.x.wedge(rhs.ux) + self.y.wedge(rhs.rx) + self.y.wedge(rhs.rz) + self.y.wedge(rhs.uw) + self.y.wedge(rhs.uy) + self.z.wedge(rhs.rx) + self.z.wedge(rhs.ry) + self.z.wedge(rhs.uw) + self.z.wedge(rhs.uz)
// Omitted: Dir anti_wedge Motor = self.x.anti_wedge(rhs.rw) + self.x.anti_wedge(rhs.ux) + self.y.anti_wedge(rhs.rw) + self.y.anti_wedge(rhs.uy) + self.z.anti_wedge(rhs.rw) + self.z.anti_wedge(rhs.uz)

//! # Moment
//!
//! ## Operations
//! ```text
//! Moment.dot(Moment) -> S
//! Moment.dot(Dir) -> Dir
//! Dir.dot(Moment) -> Dir
//! Moment.wedge(Dir) -> ZYX
//! Dir.wedge(Moment) -> ZYX
//! Moment.anti_geometric(Point) -> Dir
//! Point.anti_geometric(Moment) -> Dir
//! Moment.dot(Point) -> Dir
//! Point.dot(Moment) -> Dir
//! Moment.wedge(Point) -> Plane
//! Point.wedge(Moment) -> Plane
//! Moment.dot(Line) -> S
//! Line.dot(Moment) -> S
//! Moment.wedge(Line) -> XYZW
//! Line.wedge(Moment) -> XYZW
//! Moment.anti_wedge(Line) -> S
//! Line.anti_wedge(Moment) -> S
//! Moment.dot(Plane) -> Point
//! Plane.dot(Moment) -> Point
//! Moment.anti_wedge(Plane) -> Dir
//! Plane.anti_wedge(Moment) -> Dir
//! Moment.anti_geometric(Translator) -> Moment
//! Translator.anti_geometric(Moment) -> Moment
//! Moment.dot(Translator) -> Motor
//! Translator.dot(Moment) -> Motor
//! Moment.anti_wedge(Translator) -> Moment
//! Translator.anti_wedge(Moment) -> Moment
//! Moment.geometric(Rotor) -> Rotor
//! Rotor.geometric(Moment) -> Rotor
//! Moment.dot(Rotor) -> Line
//! Rotor.dot(Moment) -> Line
//! Moment.wedge(Rotor) -> XYZW
//! Rotor.wedge(Moment) -> XYZW
//! Moment.wedge(Motor) -> Translator
//! Motor.wedge(Moment) -> Translator
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
pub struct Moment {
    pub yz: YZ,
    pub zx: ZX,
    pub xy: XY,
}

// ---------------------------------------------------------------------

impl RCompl for Moment {
    type Output = Line;
    fn rcompl(self) -> Self::Output {
        Line {
            vx: -self.yz.rcompl(),
            vy: -self.zx.rcompl(),
            vz: -self.xy.rcompl(),
            mx: Default::default(),
            my: Default::default(),
            mz: Default::default(),
        }
    }
}

impl LCompl for Moment {
    type Output = Line;
    fn lcompl(self) -> Self::Output {
        Line {
            vx: -self.yz.lcompl(),
            vy: -self.zx.lcompl(),
            vz: -self.xy.lcompl(),
            mx: Default::default(),
            my: Default::default(),
            mz: Default::default(),
        }
    }
}

impl Reverse for Moment {
    fn rev(self) -> Self {
        Moment {
            yz: -self.yz,
            zx: -self.zx,
            xy: -self.xy,
        }
    }
}

impl AntiReverse for Moment {
    fn arev(self) -> Self {
        Moment {
            yz: -self.yz,
            zx: -self.zx,
            xy: -self.xy,
        }
    }
}

// ---------------------------------------------------------------------
// Moment OP Dir:

// Omitted: Moment geometric Dir = self.xy.geometric(rhs.x) + self.xy.geometric(rhs.y) + self.xy.geometric(rhs.z) + self.yz.geometric(rhs.x) + self.yz.geometric(rhs.y) + self.yz.geometric(rhs.z) + self.zx.geometric(rhs.x) + self.zx.geometric(rhs.y) + self.zx.geometric(rhs.z)
// Omitted: Moment anti_geometric Dir = 0

// Moment.dot(Dir) -> Dir
impl Dot<Dir> for Moment {
    type Output = Dir;
    fn dot(self, rhs: Dir) -> Self::Output {
        Dir {
            x: self.xy.dot(rhs.y) - self.zx.dot(rhs.z),
            y: -self.xy.dot(rhs.x) + self.yz.dot(rhs.z),
            z: -self.yz.dot(rhs.y) + self.zx.dot(rhs.x),
        }
    }
}

// Moment.wedge(Dir) -> ZYX
impl Wedge<Dir> for Moment {
    type Output = ZYX;
    fn wedge(self, rhs: Dir) -> Self::Output {
        self.xy.wedge(rhs.z) + self.yz.wedge(rhs.x) + self.zx.wedge(rhs.y)
    }
}

// Omitted: Moment anti_wedge Dir = 0

// ---------------------------------------------------------------------
// Moment OP Point:

// Omitted: Moment geometric Point = self.xy.geometric(rhs.w) + self.xy.geometric(rhs.x) + self.xy.geometric(rhs.y) + self.xy.geometric(rhs.z) + self.yz.geometric(rhs.w) + self.yz.geometric(rhs.x) + self.yz.geometric(rhs.y) + self.yz.geometric(rhs.z) + self.zx.geometric(rhs.w) + self.zx.geometric(rhs.x) + self.zx.geometric(rhs.y) + self.zx.geometric(rhs.z)

// Moment.anti_geometric(Point) -> Dir
impl AntiGeometric<Point> for Moment {
    type Output = Dir;
    fn anti_geometric(self, rhs: Point) -> Self::Output {
        Dir {
            x: self.yz.anti_geometric(rhs.w),
            y: self.zx.anti_geometric(rhs.w),
            z: self.xy.anti_geometric(rhs.w),
        }
    }
}

// Moment.dot(Point) -> Dir
impl Dot<Point> for Moment {
    type Output = Dir;
    fn dot(self, rhs: Point) -> Self::Output {
        Dir {
            x: self.xy.dot(rhs.y) - self.zx.dot(rhs.z),
            y: -self.xy.dot(rhs.x) + self.yz.dot(rhs.z),
            z: -self.yz.dot(rhs.y) + self.zx.dot(rhs.x),
        }
    }
}

// Moment.wedge(Point) -> Plane
impl Wedge<Point> for Moment {
    type Output = Plane;
    fn wedge(self, rhs: Point) -> Self::Output {
        Plane {
            nx: self.yz.wedge(rhs.w),
            ny: self.zx.wedge(rhs.w),
            nz: self.xy.wedge(rhs.w),
            d: -self.xy.wedge(rhs.z) - self.yz.wedge(rhs.x) - self.zx.wedge(rhs.y),
        }
    }
}

// Omitted: Moment anti_wedge Point = 0

// ---------------------------------------------------------------------
// Moment OP Moment:

// Omitted: Moment geometric Moment = self.xy.geometric(rhs.xy) + self.xy.geometric(rhs.yz) + self.xy.geometric(rhs.zx) + self.yz.geometric(rhs.xy) + self.yz.geometric(rhs.yz) + self.yz.geometric(rhs.zx) + self.zx.geometric(rhs.xy) + self.zx.geometric(rhs.yz) + self.zx.geometric(rhs.zx)
// Omitted: Moment anti_geometric Moment = 0

// Moment.dot(Moment) -> S
impl Dot<Moment> for Moment {
    type Output = S;
    fn dot(self, rhs: Moment) -> Self::Output {
        self.xy.dot(rhs.xy) + self.yz.dot(rhs.yz) + self.zx.dot(rhs.zx)
    }
}

// Omitted: Moment wedge Moment = 0
// Omitted: Moment anti_wedge Moment = 0

// ---------------------------------------------------------------------
// Moment OP Line:

// Omitted: Moment geometric Line = self.xy.geometric(rhs.mx) + self.xy.geometric(rhs.my) + self.xy.geometric(rhs.mz) + self.xy.geometric(rhs.vx) + self.xy.geometric(rhs.vy) + self.xy.geometric(rhs.vz) + self.yz.geometric(rhs.mx) + self.yz.geometric(rhs.my) + self.yz.geometric(rhs.mz) + self.yz.geometric(rhs.vx) + self.yz.geometric(rhs.vy) + self.yz.geometric(rhs.vz) + self.zx.geometric(rhs.mx) + self.zx.geometric(rhs.my) + self.zx.geometric(rhs.mz) + self.zx.geometric(rhs.vx) + self.zx.geometric(rhs.vy) + self.zx.geometric(rhs.vz)
// Omitted: Moment anti_geometric Line = self.xy.anti_geometric(rhs.vx) + self.xy.anti_geometric(rhs.vy) + self.xy.anti_geometric(rhs.vz) + self.yz.anti_geometric(rhs.vx) + self.yz.anti_geometric(rhs.vy) + self.yz.anti_geometric(rhs.vz) + self.zx.anti_geometric(rhs.vx) + self.zx.anti_geometric(rhs.vy) + self.zx.anti_geometric(rhs.vz)

// Moment.dot(Line) -> S
impl Dot<Line> for Moment {
    type Output = S;
    fn dot(self, rhs: Line) -> Self::Output {
        self.xy.dot(rhs.mz) + self.yz.dot(rhs.mx) + self.zx.dot(rhs.my)
    }
}

// Moment.wedge(Line) -> XYZW
impl Wedge<Line> for Moment {
    type Output = XYZW;
    fn wedge(self, rhs: Line) -> Self::Output {
        self.xy.wedge(rhs.vz) + self.yz.wedge(rhs.vx) + self.zx.wedge(rhs.vy)
    }
}

// Moment.anti_wedge(Line) -> S
impl AntiWedge<Line> for Moment {
    type Output = S;
    fn anti_wedge(self, rhs: Line) -> Self::Output {
        self.xy.anti_wedge(rhs.vz) + self.yz.anti_wedge(rhs.vx) + self.zx.anti_wedge(rhs.vy)
    }
}

// ---------------------------------------------------------------------
// Moment OP Plane:

// Omitted: Moment geometric Plane = self.xy.geometric(rhs.d) + self.xy.geometric(rhs.nx) + self.xy.geometric(rhs.ny) + self.xy.geometric(rhs.nz) + self.yz.geometric(rhs.d) + self.yz.geometric(rhs.nx) + self.yz.geometric(rhs.ny) + self.yz.geometric(rhs.nz) + self.zx.geometric(rhs.d) + self.zx.geometric(rhs.nx) + self.zx.geometric(rhs.ny) + self.zx.geometric(rhs.nz)
// Omitted: Moment anti_geometric Plane = self.xy.anti_geometric(rhs.nx) + self.xy.anti_geometric(rhs.ny) + self.xy.anti_geometric(rhs.nz) + self.yz.anti_geometric(rhs.nx) + self.yz.anti_geometric(rhs.ny) + self.yz.anti_geometric(rhs.nz) + self.zx.anti_geometric(rhs.nx) + self.zx.anti_geometric(rhs.ny) + self.zx.anti_geometric(rhs.nz)

// Moment.dot(Plane) -> Point
impl Dot<Plane> for Moment {
    type Output = Point;
    fn dot(self, rhs: Plane) -> Self::Output {
        Point {
            x: self.yz.dot(rhs.d),
            y: self.zx.dot(rhs.d),
            z: self.xy.dot(rhs.d),
            w: -self.xy.dot(rhs.nz) - self.yz.dot(rhs.nx) - self.zx.dot(rhs.ny),
        }
    }
}

// Omitted: Moment wedge Plane = 0

// Moment.anti_wedge(Plane) -> Dir
impl AntiWedge<Plane> for Moment {
    type Output = Dir;
    fn anti_wedge(self, rhs: Plane) -> Self::Output {
        Dir {
            x: -self.xy.anti_wedge(rhs.ny) + self.zx.anti_wedge(rhs.nz),
            y: self.xy.anti_wedge(rhs.nx) - self.yz.anti_wedge(rhs.nz),
            z: self.yz.anti_wedge(rhs.ny) - self.zx.anti_wedge(rhs.nx),
        }
    }
}

// ---------------------------------------------------------------------
// Moment OP Translator:

// Omitted: Moment geometric Translator = self.xy.geometric(rhs.x) + self.xy.geometric(rhs.xyzw) + self.xy.geometric(rhs.y) + self.xy.geometric(rhs.z) + self.yz.geometric(rhs.x) + self.yz.geometric(rhs.xyzw) + self.yz.geometric(rhs.y) + self.yz.geometric(rhs.z) + self.zx.geometric(rhs.x) + self.zx.geometric(rhs.xyzw) + self.zx.geometric(rhs.y) + self.zx.geometric(rhs.z)

// Moment.anti_geometric(Translator) -> Moment
impl AntiGeometric<Translator> for Moment {
    type Output = Moment;
    fn anti_geometric(self, rhs: Translator) -> Self::Output {
        Moment {
            yz: self.yz.anti_geometric(rhs.xyzw),
            zx: self.zx.anti_geometric(rhs.xyzw),
            xy: self.xy.anti_geometric(rhs.xyzw),
        }
    }
}

// Moment.dot(Translator) -> Motor
impl Dot<Translator> for Moment {
    type Output = Motor;
    fn dot(self, rhs: Translator) -> Self::Output {
        Motor {
            rx: self.yz.dot(rhs.xyzw),
            ry: self.zx.dot(rhs.xyzw),
            rz: self.xy.dot(rhs.xyzw),
            rw: Default::default(),
            ux: Default::default(),
            uy: Default::default(),
            uz: Default::default(),
            uw: -self.xy.dot(rhs.z) - self.yz.dot(rhs.x) - self.zx.dot(rhs.y),
        }
    }
}

// Omitted: Moment wedge Translator = 0

// Moment.anti_wedge(Translator) -> Moment
impl AntiWedge<Translator> for Moment {
    type Output = Moment;
    fn anti_wedge(self, rhs: Translator) -> Self::Output {
        Moment {
            yz: self.yz.anti_wedge(rhs.xyzw),
            zx: self.zx.anti_wedge(rhs.xyzw),
            xy: self.xy.anti_wedge(rhs.xyzw),
        }
    }
}

// ---------------------------------------------------------------------
// Moment OP Rotor:

// Moment.geometric(Rotor) -> Rotor
impl Geometric<Rotor> for Moment {
    type Output = Rotor;
    fn geometric(self, rhs: Rotor) -> Self::Output {
        Rotor {
            x: self.xy.geometric(rhs.y) + self.yz.geometric(rhs.w) - self.zx.geometric(rhs.z),
            y: -self.xy.geometric(rhs.x) + self.yz.geometric(rhs.z) + self.zx.geometric(rhs.w),
            z: self.xy.geometric(rhs.w) - self.yz.geometric(rhs.y) + self.zx.geometric(rhs.x),
            w: -self.xy.geometric(rhs.z) - self.yz.geometric(rhs.x) - self.zx.geometric(rhs.y),
        }
    }
}

// Omitted: Moment anti_geometric Rotor = self.xy.anti_geometric(rhs.w) + self.xy.anti_geometric(rhs.x) + self.xy.anti_geometric(rhs.y) + self.xy.anti_geometric(rhs.z) + self.yz.anti_geometric(rhs.w) + self.yz.anti_geometric(rhs.x) + self.yz.anti_geometric(rhs.y) + self.yz.anti_geometric(rhs.z) + self.zx.anti_geometric(rhs.w) + self.zx.anti_geometric(rhs.x) + self.zx.anti_geometric(rhs.y) + self.zx.anti_geometric(rhs.z)

// Moment.dot(Rotor) -> Line
impl Dot<Rotor> for Moment {
    type Output = Line;
    fn dot(self, rhs: Rotor) -> Self::Output {
        Line {
            vx: self.yz.dot(rhs.w),
            vy: self.zx.dot(rhs.w),
            vz: self.xy.dot(rhs.w),
            mx: Default::default(),
            my: Default::default(),
            mz: Default::default(),
        }
    }
}

// Moment.wedge(Rotor) -> XYZW
impl Wedge<Rotor> for Moment {
    type Output = XYZW;
    fn wedge(self, rhs: Rotor) -> Self::Output {
        self.xy.wedge(rhs.z) + self.yz.wedge(rhs.x) + self.zx.wedge(rhs.y)
    }
}

// Omitted: Moment anti_wedge Rotor = self.xy.anti_wedge(rhs.w) + self.xy.anti_wedge(rhs.z) + self.yz.anti_wedge(rhs.w) + self.yz.anti_wedge(rhs.x) + self.zx.anti_wedge(rhs.w) + self.zx.anti_wedge(rhs.y)

// ---------------------------------------------------------------------
// Moment OP Motor:

// Omitted: Moment geometric Motor = self.xy.geometric(rhs.rw) + self.xy.geometric(rhs.rx) + self.xy.geometric(rhs.ry) + self.xy.geometric(rhs.rz) + self.xy.geometric(rhs.uw) + self.xy.geometric(rhs.ux) + self.xy.geometric(rhs.uy) + self.xy.geometric(rhs.uz) + self.yz.geometric(rhs.rw) + self.yz.geometric(rhs.rx) + self.yz.geometric(rhs.ry) + self.yz.geometric(rhs.rz) + self.yz.geometric(rhs.uw) + self.yz.geometric(rhs.ux) + self.yz.geometric(rhs.uy) + self.yz.geometric(rhs.uz) + self.zx.geometric(rhs.rw) + self.zx.geometric(rhs.rx) + self.zx.geometric(rhs.ry) + self.zx.geometric(rhs.rz) + self.zx.geometric(rhs.uw) + self.zx.geometric(rhs.ux) + self.zx.geometric(rhs.uy) + self.zx.geometric(rhs.uz)
// Omitted: Moment anti_geometric Motor = self.xy.anti_geometric(rhs.rw) + self.xy.anti_geometric(rhs.rx) + self.xy.anti_geometric(rhs.ry) + self.xy.anti_geometric(rhs.rz) + self.xy.anti_geometric(rhs.ux) + self.xy.anti_geometric(rhs.uy) + self.xy.anti_geometric(rhs.uz) + self.yz.anti_geometric(rhs.rw) + self.yz.anti_geometric(rhs.rx) + self.yz.anti_geometric(rhs.ry) + self.yz.anti_geometric(rhs.rz) + self.yz.anti_geometric(rhs.ux) + self.yz.anti_geometric(rhs.uy) + self.yz.anti_geometric(rhs.uz) + self.zx.anti_geometric(rhs.rw) + self.zx.anti_geometric(rhs.rx) + self.zx.anti_geometric(rhs.ry) + self.zx.anti_geometric(rhs.rz) + self.zx.anti_geometric(rhs.ux) + self.zx.anti_geometric(rhs.uy) + self.zx.anti_geometric(rhs.uz)
// Omitted: Moment dot Motor = self.xy.dot(rhs.rw) + self.xy.dot(rhs.uw) + self.xy.dot(rhs.uz) + self.yz.dot(rhs.rw) + self.yz.dot(rhs.uw) + self.yz.dot(rhs.ux) + self.zx.dot(rhs.rw) + self.zx.dot(rhs.uw) + self.zx.dot(rhs.uy)

// Moment.wedge(Motor) -> Translator
impl Wedge<Motor> for Moment {
    type Output = Translator;
    fn wedge(self, rhs: Motor) -> Self::Output {
        Translator {
            x: self.yz.wedge(rhs.uw),
            y: self.zx.wedge(rhs.uw),
            z: self.xy.wedge(rhs.uw),
            xyzw: -self.xy.wedge(rhs.rz) - self.yz.wedge(rhs.rx) - self.zx.wedge(rhs.ry),
        }
    }
}

// Omitted: Moment anti_wedge Motor = self.xy.anti_wedge(rhs.rw) + self.xy.anti_wedge(rhs.rz) + self.xy.anti_wedge(rhs.ux) + self.xy.anti_wedge(rhs.uy) + self.yz.anti_wedge(rhs.rw) + self.yz.anti_wedge(rhs.rx) + self.yz.anti_wedge(rhs.uy) + self.yz.anti_wedge(rhs.uz) + self.zx.anti_wedge(rhs.rw) + self.zx.anti_wedge(rhs.ry) + self.zx.anti_wedge(rhs.ux) + self.zx.anti_wedge(rhs.uz)

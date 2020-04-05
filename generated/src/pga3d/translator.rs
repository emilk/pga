//! # Translator
//!
//! ## Operations
//! ```text
//! Translator.anti_geometric(Translator) -> Translator
//! Translator.dot(Translator) -> Motor
//! Translator.anti_wedge(Translator) -> Translator
//! Translator.anti_geometric(Dir) -> Dir
//! Dir.anti_geometric(Translator) -> Dir
//! Translator.wedge(Dir) -> ZYX
//! Dir.wedge(Translator) -> ZYX
//! Translator.anti_wedge(Dir) -> Dir
//! Dir.anti_wedge(Translator) -> Dir
//! Translator.anti_geometric(Point) -> Point
//! Point.anti_geometric(Translator) -> Point
//! Translator.wedge(Point) -> Plane
//! Point.wedge(Translator) -> Plane
//! Translator.anti_wedge(Point) -> Point
//! Point.anti_wedge(Translator) -> Point
//! Translator.dot(Line) -> Motor
//! Line.dot(Translator) -> Motor
//! Translator.wedge(Line) -> XYZW
//! Line.wedge(Translator) -> XYZW
//! Translator.dot(Plane) -> Point
//! Plane.dot(Translator) -> Point
//! Translator.geometric(Rotor) -> Rotor
//! Rotor.geometric(Translator) -> Rotor
//! Translator.dot(Rotor) -> Line
//! Rotor.dot(Translator) -> Line
//! Translator.wedge(Rotor) -> XYZW
//! Rotor.wedge(Translator) -> XYZW
//! Translator.wedge(Motor) -> Translator
//! Motor.wedge(Translator) -> Translator
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
    pub x: YZ,
    pub y: ZX,
    pub z: XY,
    pub xyzw: XYZW,
}

// ---------------------------------------------------------------------

impl RCompl for Translator {
    type Output = Motor;
    fn rcompl(self) -> Self::Output {
        Motor {
            rx: self.x.rcompl(),
            ry: self.y.rcompl(),
            rz: self.z.rcompl(),
            rw: Default::default(),
            ux: Default::default(),
            uy: Default::default(),
            uz: Default::default(),
            uw: self.xyzw.rcompl(),
        }
    }
}

impl LCompl for Translator {
    type Output = Motor;
    fn lcompl(self) -> Self::Output {
        Motor {
            rx: self.x.lcompl(),
            ry: self.y.lcompl(),
            rz: self.z.lcompl(),
            rw: Default::default(),
            ux: Default::default(),
            uy: Default::default(),
            uz: Default::default(),
            uw: self.xyzw.lcompl(),
        }
    }
}

impl Reverse for Translator {
    fn rev(self) -> Self {
        Translator {
            x: -self.x.rev(),
            y: self.y.rev(),
            z: -self.z.rev(),
            xyzw: self.xyzw.rev(),
        }
    }
}

impl AntiReverse for Translator {
    fn arev(self) -> Self {
        Translator {
            x: -self.x.arev(),
            y: self.y.arev(),
            z: -self.z.arev(),
            xyzw: self.xyzw.arev(),
        }
    }
}

// ---------------------------------------------------------------------
// Translator OP Dir:

// Omitted: Translator geometric Dir = self.x.geometric(rhs.x) + self.x.geometric(rhs.y) + self.x.geometric(rhs.z) + self.xyzw.geometric(rhs.x) + self.xyzw.geometric(rhs.y) + self.xyzw.geometric(rhs.z) + self.y.geometric(rhs.x) + self.y.geometric(rhs.y) + self.y.geometric(rhs.z) + self.z.geometric(rhs.x) + self.z.geometric(rhs.y) + self.z.geometric(rhs.z)

// Translator.anti_geometric(Dir) -> Dir
impl AntiGeometric<Dir> for Translator {
    type Output = Dir;
    fn anti_geometric(self, rhs: Dir) -> Self::Output {
        Dir {
            x: self.xyzw.anti_geometric(rhs.x),
            y: self.xyzw.anti_geometric(rhs.y),
            z: self.xyzw.anti_geometric(rhs.z),
        }
    }
}

// Omitted: Translator dot Dir = self.x.dot(rhs.y) + self.x.dot(rhs.z) + self.xyzw.dot(rhs.x) + self.xyzw.dot(rhs.y) + self.xyzw.dot(rhs.z) + self.y.dot(rhs.x) + self.y.dot(rhs.z) + self.z.dot(rhs.x) + self.z.dot(rhs.y)

// Translator.wedge(Dir) -> ZYX
impl Wedge<Dir> for Translator {
    type Output = ZYX;
    fn wedge(self, rhs: Dir) -> Self::Output {
        self.x.wedge(rhs.x) + self.y.wedge(rhs.y) + self.z.wedge(rhs.z)
    }
}

// Translator.anti_wedge(Dir) -> Dir
impl AntiWedge<Dir> for Translator {
    type Output = Dir;
    fn anti_wedge(self, rhs: Dir) -> Self::Output {
        Dir {
            x: self.xyzw.anti_wedge(rhs.x),
            y: self.xyzw.anti_wedge(rhs.y),
            z: self.xyzw.anti_wedge(rhs.z),
        }
    }
}

// ---------------------------------------------------------------------
// Translator OP Point:

// Omitted: Translator geometric Point = self.x.geometric(rhs.w) + self.x.geometric(rhs.x) + self.x.geometric(rhs.y) + self.x.geometric(rhs.z) + self.xyzw.geometric(rhs.x) + self.xyzw.geometric(rhs.y) + self.xyzw.geometric(rhs.z) + self.y.geometric(rhs.w) + self.y.geometric(rhs.x) + self.y.geometric(rhs.y) + self.y.geometric(rhs.z) + self.z.geometric(rhs.w) + self.z.geometric(rhs.x) + self.z.geometric(rhs.y) + self.z.geometric(rhs.z)

// Translator.anti_geometric(Point) -> Point
impl AntiGeometric<Point> for Translator {
    type Output = Point;
    fn anti_geometric(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x.anti_geometric(rhs.w) + self.xyzw.anti_geometric(rhs.x),
            y: self.xyzw.anti_geometric(rhs.y) + self.y.anti_geometric(rhs.w),
            z: self.xyzw.anti_geometric(rhs.z) + self.z.anti_geometric(rhs.w),
            w: self.xyzw.anti_geometric(rhs.w),
        }
    }
}

// Omitted: Translator dot Point = self.x.dot(rhs.y) + self.x.dot(rhs.z) + self.xyzw.dot(rhs.x) + self.xyzw.dot(rhs.y) + self.xyzw.dot(rhs.z) + self.y.dot(rhs.x) + self.y.dot(rhs.z) + self.z.dot(rhs.x) + self.z.dot(rhs.y)

// Translator.wedge(Point) -> Plane
impl Wedge<Point> for Translator {
    type Output = Plane;
    fn wedge(self, rhs: Point) -> Self::Output {
        Plane {
            nx: self.x.wedge(rhs.w),
            ny: -self.y.wedge(rhs.w),
            nz: self.z.wedge(rhs.w),
            d: self.x.wedge(rhs.x) + self.y.wedge(rhs.y) + self.z.wedge(rhs.z),
        }
    }
}

// Translator.anti_wedge(Point) -> Point
impl AntiWedge<Point> for Translator {
    type Output = Point;
    fn anti_wedge(self, rhs: Point) -> Self::Output {
        Point {
            x: self.xyzw.anti_wedge(rhs.x),
            y: self.xyzw.anti_wedge(rhs.y),
            z: self.xyzw.anti_wedge(rhs.z),
            w: self.xyzw.anti_wedge(rhs.w),
        }
    }
}

// ---------------------------------------------------------------------
// Translator OP Line:

// Omitted: Translator geometric Line = self.x.geometric(rhs.mx) + self.x.geometric(rhs.my) + self.x.geometric(rhs.mz) + self.x.geometric(rhs.vx) + self.x.geometric(rhs.vy) + self.x.geometric(rhs.vz) + self.xyzw.geometric(rhs.mx) + self.xyzw.geometric(rhs.my) + self.xyzw.geometric(rhs.mz) + self.y.geometric(rhs.mx) + self.y.geometric(rhs.my) + self.y.geometric(rhs.mz) + self.y.geometric(rhs.vx) + self.y.geometric(rhs.vy) + self.y.geometric(rhs.vz) + self.z.geometric(rhs.mx) + self.z.geometric(rhs.my) + self.z.geometric(rhs.mz) + self.z.geometric(rhs.vx) + self.z.geometric(rhs.vy) + self.z.geometric(rhs.vz)
// Omitted: Translator anti_geometric Line = self.x.anti_geometric(rhs.vx) + self.x.anti_geometric(rhs.vy) + self.x.anti_geometric(rhs.vz) + self.xyzw.anti_geometric(rhs.mx) + self.xyzw.anti_geometric(rhs.my) + self.xyzw.anti_geometric(rhs.mz) + self.xyzw.anti_geometric(rhs.vx) + self.xyzw.anti_geometric(rhs.vy) + self.xyzw.anti_geometric(rhs.vz) + self.y.anti_geometric(rhs.vx) + self.y.anti_geometric(rhs.vy) + self.y.anti_geometric(rhs.vz) + self.z.anti_geometric(rhs.vx) + self.z.anti_geometric(rhs.vy) + self.z.anti_geometric(rhs.vz)

// Translator.dot(Line) -> Motor
impl Dot<Line> for Translator {
    type Output = Motor;
    fn dot(self, rhs: Line) -> Self::Output {
        Motor {
            rx: -self.xyzw.dot(rhs.mx),
            ry: -self.xyzw.dot(rhs.my),
            rz: -self.xyzw.dot(rhs.mz),
            rw: Default::default(),
            ux: Default::default(),
            uy: Default::default(),
            uz: Default::default(),
            uw: -self.x.dot(rhs.mx) - self.y.dot(rhs.my) - self.z.dot(rhs.mz),
        }
    }
}

// Translator.wedge(Line) -> XYZW
impl Wedge<Line> for Translator {
    type Output = XYZW;
    fn wedge(self, rhs: Line) -> Self::Output {
        self.x.wedge(rhs.vx) + self.y.wedge(rhs.vy) + self.z.wedge(rhs.vz)
    }
}

// Omitted: Translator anti_wedge Line = self.x.anti_wedge(rhs.vx) + self.xyzw.anti_wedge(rhs.mx) + self.xyzw.anti_wedge(rhs.my) + self.xyzw.anti_wedge(rhs.mz) + self.xyzw.anti_wedge(rhs.vx) + self.xyzw.anti_wedge(rhs.vy) + self.xyzw.anti_wedge(rhs.vz) + self.y.anti_wedge(rhs.vy) + self.z.anti_wedge(rhs.vz)

// ---------------------------------------------------------------------
// Translator OP Plane:

// Omitted: Translator geometric Plane = self.x.geometric(rhs.d) + self.x.geometric(rhs.nx) + self.x.geometric(rhs.ny) + self.x.geometric(rhs.nz) + self.xyzw.geometric(rhs.d) + self.y.geometric(rhs.d) + self.y.geometric(rhs.nx) + self.y.geometric(rhs.ny) + self.y.geometric(rhs.nz) + self.z.geometric(rhs.d) + self.z.geometric(rhs.nx) + self.z.geometric(rhs.ny) + self.z.geometric(rhs.nz)
// Omitted: Translator anti_geometric Plane = self.x.anti_geometric(rhs.nx) + self.x.anti_geometric(rhs.ny) + self.x.anti_geometric(rhs.nz) + self.xyzw.anti_geometric(rhs.d) + self.xyzw.anti_geometric(rhs.nx) + self.xyzw.anti_geometric(rhs.ny) + self.xyzw.anti_geometric(rhs.nz) + self.y.anti_geometric(rhs.nx) + self.y.anti_geometric(rhs.ny) + self.y.anti_geometric(rhs.nz) + self.z.anti_geometric(rhs.nx) + self.z.anti_geometric(rhs.ny) + self.z.anti_geometric(rhs.nz)

// Translator.dot(Plane) -> Point
impl Dot<Plane> for Translator {
    type Output = Point;
    fn dot(self, rhs: Plane) -> Self::Output {
        Point {
            x: self.x.dot(rhs.d),
            y: self.y.dot(rhs.d),
            z: self.z.dot(rhs.d),
            w: -self.x.dot(rhs.nx) - self.xyzw.dot(rhs.d) - self.y.dot(rhs.ny) - self.z.dot(rhs.nz),
        }
    }
}

// Omitted: Translator wedge Plane = 0
// Omitted: Translator anti_wedge Plane = self.x.anti_wedge(rhs.ny) + self.x.anti_wedge(rhs.nz) + self.xyzw.anti_wedge(rhs.d) + self.xyzw.anti_wedge(rhs.nx) + self.xyzw.anti_wedge(rhs.ny) + self.xyzw.anti_wedge(rhs.nz) + self.y.anti_wedge(rhs.nx) + self.y.anti_wedge(rhs.nz) + self.z.anti_wedge(rhs.nx) + self.z.anti_wedge(rhs.ny)

// ---------------------------------------------------------------------
// Translator OP Translator:

// Omitted: Translator geometric Translator = self.x.geometric(rhs.x) + self.x.geometric(rhs.xyzw) + self.x.geometric(rhs.y) + self.x.geometric(rhs.z) + self.xyzw.geometric(rhs.x) + self.xyzw.geometric(rhs.y) + self.xyzw.geometric(rhs.z) + self.y.geometric(rhs.x) + self.y.geometric(rhs.xyzw) + self.y.geometric(rhs.y) + self.y.geometric(rhs.z) + self.z.geometric(rhs.x) + self.z.geometric(rhs.xyzw) + self.z.geometric(rhs.y) + self.z.geometric(rhs.z)

// Translator.anti_geometric(Translator) -> Translator
impl AntiGeometric<Translator> for Translator {
    type Output = Translator;
    fn anti_geometric(self, rhs: Translator) -> Self::Output {
        Translator {
            x: self.x.anti_geometric(rhs.xyzw) + self.xyzw.anti_geometric(rhs.x),
            y: -self.xyzw.anti_geometric(rhs.y) - self.y.anti_geometric(rhs.xyzw),
            z: self.xyzw.anti_geometric(rhs.z) + self.z.anti_geometric(rhs.xyzw),
            xyzw: self.xyzw.anti_geometric(rhs.xyzw),
        }
    }
}

// Translator.dot(Translator) -> Motor
impl Dot<Translator> for Translator {
    type Output = Motor;
    fn dot(self, rhs: Translator) -> Self::Output {
        Motor {
            rx: -self.x.dot(rhs.xyzw) - self.xyzw.dot(rhs.x),
            ry: -self.xyzw.dot(rhs.y) - self.y.dot(rhs.xyzw),
            rz: -self.xyzw.dot(rhs.z) - self.z.dot(rhs.xyzw),
            rw: Default::default(),
            ux: Default::default(),
            uy: Default::default(),
            uz: Default::default(),
            uw: -self.x.dot(rhs.x) - self.y.dot(rhs.y) - self.z.dot(rhs.z),
        }
    }
}

// Omitted: Translator wedge Translator = 0

// Translator.anti_wedge(Translator) -> Translator
impl AntiWedge<Translator> for Translator {
    type Output = Translator;
    fn anti_wedge(self, rhs: Translator) -> Self::Output {
        Translator {
            x: self.x.anti_wedge(rhs.xyzw) + self.xyzw.anti_wedge(rhs.x),
            y: -self.xyzw.anti_wedge(rhs.y) - self.y.anti_wedge(rhs.xyzw),
            z: self.xyzw.anti_wedge(rhs.z) + self.z.anti_wedge(rhs.xyzw),
            xyzw: self.xyzw.anti_wedge(rhs.xyzw),
        }
    }
}

// ---------------------------------------------------------------------
// Translator OP Rotor:

// Translator.geometric(Rotor) -> Rotor
impl Geometric<Rotor> for Translator {
    type Output = Rotor;
    fn geometric(self, rhs: Rotor) -> Self::Output {
        Rotor {
            x: -self.x.geometric(rhs.w) + self.y.geometric(rhs.z) - self.z.geometric(rhs.y),
            y: -self.x.geometric(rhs.z) - self.y.geometric(rhs.w) + self.z.geometric(rhs.x),
            z: self.x.geometric(rhs.y) - self.y.geometric(rhs.x) - self.z.geometric(rhs.w),
            w: -self.x.geometric(rhs.x) - self.y.geometric(rhs.y) - self.z.geometric(rhs.z),
        }
    }
}

// Omitted: Translator anti_geometric Rotor = self.x.anti_geometric(rhs.w) + self.x.anti_geometric(rhs.x) + self.x.anti_geometric(rhs.y) + self.x.anti_geometric(rhs.z) + self.xyzw.anti_geometric(rhs.w) + self.xyzw.anti_geometric(rhs.x) + self.xyzw.anti_geometric(rhs.y) + self.xyzw.anti_geometric(rhs.z) + self.y.anti_geometric(rhs.w) + self.y.anti_geometric(rhs.x) + self.y.anti_geometric(rhs.y) + self.y.anti_geometric(rhs.z) + self.z.anti_geometric(rhs.w) + self.z.anti_geometric(rhs.x) + self.z.anti_geometric(rhs.y) + self.z.anti_geometric(rhs.z)

// Translator.dot(Rotor) -> Line
impl Dot<Rotor> for Translator {
    type Output = Line;
    fn dot(self, rhs: Rotor) -> Self::Output {
        Line {
            vx: -self.x.dot(rhs.w),
            vy: -self.y.dot(rhs.w),
            vz: -self.z.dot(rhs.w),
            mx: Default::default(),
            my: Default::default(),
            mz: Default::default(),
        }
    }
}

// Translator.wedge(Rotor) -> XYZW
impl Wedge<Rotor> for Translator {
    type Output = XYZW;
    fn wedge(self, rhs: Rotor) -> Self::Output {
        self.x.wedge(rhs.x) + self.y.wedge(rhs.y) + self.z.wedge(rhs.z)
    }
}

// Omitted: Translator anti_wedge Rotor = self.x.anti_wedge(rhs.w) + self.x.anti_wedge(rhs.x) + self.xyzw.anti_wedge(rhs.w) + self.xyzw.anti_wedge(rhs.x) + self.xyzw.anti_wedge(rhs.y) + self.xyzw.anti_wedge(rhs.z) + self.y.anti_wedge(rhs.w) + self.y.anti_wedge(rhs.y) + self.z.anti_wedge(rhs.w) + self.z.anti_wedge(rhs.z)

// ---------------------------------------------------------------------
// Translator OP Motor:

// Omitted: Translator geometric Motor = self.x.geometric(rhs.rw) + self.x.geometric(rhs.rx) + self.x.geometric(rhs.ry) + self.x.geometric(rhs.rz) + self.x.geometric(rhs.uw) + self.x.geometric(rhs.ux) + self.x.geometric(rhs.uy) + self.x.geometric(rhs.uz) + self.xyzw.geometric(rhs.uw) + self.y.geometric(rhs.rw) + self.y.geometric(rhs.rx) + self.y.geometric(rhs.ry) + self.y.geometric(rhs.rz) + self.y.geometric(rhs.uw) + self.y.geometric(rhs.ux) + self.y.geometric(rhs.uy) + self.y.geometric(rhs.uz) + self.z.geometric(rhs.rw) + self.z.geometric(rhs.rx) + self.z.geometric(rhs.ry) + self.z.geometric(rhs.rz) + self.z.geometric(rhs.uw) + self.z.geometric(rhs.ux) + self.z.geometric(rhs.uy) + self.z.geometric(rhs.uz)
// Omitted: Translator anti_geometric Motor = self.x.anti_geometric(rhs.rw) + self.x.anti_geometric(rhs.rx) + self.x.anti_geometric(rhs.ry) + self.x.anti_geometric(rhs.rz) + self.x.anti_geometric(rhs.ux) + self.x.anti_geometric(rhs.uy) + self.x.anti_geometric(rhs.uz) + self.xyzw.anti_geometric(rhs.rw) + self.xyzw.anti_geometric(rhs.rx) + self.xyzw.anti_geometric(rhs.ry) + self.xyzw.anti_geometric(rhs.rz) + self.xyzw.anti_geometric(rhs.uw) + self.xyzw.anti_geometric(rhs.ux) + self.xyzw.anti_geometric(rhs.uy) + self.xyzw.anti_geometric(rhs.uz) + self.y.anti_geometric(rhs.rw) + self.y.anti_geometric(rhs.rx) + self.y.anti_geometric(rhs.ry) + self.y.anti_geometric(rhs.rz) + self.y.anti_geometric(rhs.ux) + self.y.anti_geometric(rhs.uy) + self.y.anti_geometric(rhs.uz) + self.z.anti_geometric(rhs.rw) + self.z.anti_geometric(rhs.rx) + self.z.anti_geometric(rhs.ry) + self.z.anti_geometric(rhs.rz) + self.z.anti_geometric(rhs.ux) + self.z.anti_geometric(rhs.uy) + self.z.anti_geometric(rhs.uz)
// Omitted: Translator dot Motor = self.x.dot(rhs.rw) + self.x.dot(rhs.uw) + self.x.dot(rhs.ux) + self.xyzw.dot(rhs.uw) + self.y.dot(rhs.rw) + self.y.dot(rhs.uw) + self.y.dot(rhs.uy) + self.z.dot(rhs.rw) + self.z.dot(rhs.uw) + self.z.dot(rhs.uz)

// Translator.wedge(Motor) -> Translator
impl Wedge<Motor> for Translator {
    type Output = Translator;
    fn wedge(self, rhs: Motor) -> Self::Output {
        Translator {
            x: self.x.wedge(rhs.uw),
            y: -self.y.wedge(rhs.uw),
            z: self.z.wedge(rhs.uw),
            xyzw: -self.x.wedge(rhs.rx) + self.xyzw.wedge(rhs.uw)
                - self.y.wedge(rhs.ry)
                - self.z.wedge(rhs.rz),
        }
    }
}

// Omitted: Translator anti_wedge Motor = self.x.anti_wedge(rhs.rw) + self.x.anti_wedge(rhs.rx) + self.x.anti_wedge(rhs.uy) + self.x.anti_wedge(rhs.uz) + self.xyzw.anti_wedge(rhs.rw) + self.xyzw.anti_wedge(rhs.rx) + self.xyzw.anti_wedge(rhs.ry) + self.xyzw.anti_wedge(rhs.rz) + self.xyzw.anti_wedge(rhs.uw) + self.xyzw.anti_wedge(rhs.ux) + self.xyzw.anti_wedge(rhs.uy) + self.xyzw.anti_wedge(rhs.uz) + self.y.anti_wedge(rhs.rw) + self.y.anti_wedge(rhs.ry) + self.y.anti_wedge(rhs.ux) + self.y.anti_wedge(rhs.uz) + self.z.anti_wedge(rhs.rw) + self.z.anti_wedge(rhs.rz) + self.z.anti_wedge(rhs.ux) + self.z.anti_wedge(rhs.uy)

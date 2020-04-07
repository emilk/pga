//! # Line
//!
//! ## Operations
//! ```text
//! Line.dot(Line) -> S
//! Line.wedge(Line) -> XYZW
//! Line.anti_wedge(Line) -> S
//! Line.dot(Dir) -> Point
//! Dir.dot(Line) -> Point
//! Line.wedge(Dir) -> Plane
//! Dir.wedge(Line) -> Plane
//! Line.dot(Point) -> Point
//! Point.dot(Line) -> Point
//! Line.wedge(Point) -> Plane
//! Point.wedge(Line) -> Plane
//! Line.dot(Moment) -> S
//! Moment.dot(Line) -> S
//! Line.wedge(Moment) -> XYZW
//! Moment.wedge(Line) -> XYZW
//! Line.anti_wedge(Moment) -> S
//! Moment.anti_wedge(Line) -> S
//! Line.dot(Plane) -> Point
//! Plane.dot(Line) -> Point
//! Line.anti_wedge(Plane) -> Point
//! Plane.anti_wedge(Line) -> Point
//! Line.dot(Translator) -> Motor
//! Translator.dot(Line) -> Motor
//! Line.wedge(Translator) -> XYZW
//! Translator.wedge(Line) -> XYZW
//! Line.geometric(Rotor) -> Rotor
//! Rotor.geometric(Line) -> Rotor
//! Line.dot(Rotor) -> Line
//! Rotor.dot(Line) -> Line
//! Line.wedge(Rotor) -> XYZW
//! Rotor.wedge(Line) -> XYZW
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
pub struct Line {
    pub vx: WX,
    pub vy: WY,
    pub vz: WZ,
    pub mx: YZ,
    pub my: ZX,
    pub mz: XY,
}

// ---------------------------------------------------------------------

impl RCompl for Line {
    type Output = Line;
    fn rcompl(self) -> Self::Output {
        Line {
            vx: -self.mx.rcompl(),
            vy: -self.my.rcompl(),
            vz: -self.mz.rcompl(),
            mx: -self.vx.rcompl(),
            my: -self.vy.rcompl(),
            mz: -self.vz.rcompl(),
        }
    }
}

impl LCompl for Line {
    type Output = Line;
    fn lcompl(self) -> Self::Output {
        Line {
            vx: -self.mx.lcompl(),
            vy: -self.my.lcompl(),
            vz: -self.mz.lcompl(),
            mx: -self.vx.lcompl(),
            my: -self.vy.lcompl(),
            mz: -self.vz.lcompl(),
        }
    }
}

impl Reverse for Line {
    fn rev(self) -> Self {
        Line {
            vx: -self.vx,
            vy: -self.vy,
            vz: -self.vz,
            mx: -self.mx,
            my: -self.my,
            mz: -self.mz,
        }
    }
}

impl AntiReverse for Line {
    fn arev(self) -> Self {
        Line {
            vx: -self.vx,
            vy: -self.vy,
            vz: -self.vz,
            mx: -self.mx,
            my: -self.my,
            mz: -self.mz,
        }
    }
}

// ---------------------------------------------------------------------
// Line OP Dir:

// Omitted: Line geometric Dir = self.mx.geometric(rhs.x) + self.mx.geometric(rhs.y) + self.mx.geometric(rhs.z) + self.my.geometric(rhs.x) + self.my.geometric(rhs.y) + self.my.geometric(rhs.z) + self.mz.geometric(rhs.x) + self.mz.geometric(rhs.y) + self.mz.geometric(rhs.z) + self.vx.geometric(rhs.x) + self.vx.geometric(rhs.y) + self.vx.geometric(rhs.z) + self.vy.geometric(rhs.x) + self.vy.geometric(rhs.y) + self.vy.geometric(rhs.z) + self.vz.geometric(rhs.x) + self.vz.geometric(rhs.y) + self.vz.geometric(rhs.z)
// Omitted: Line anti_geometric Dir = self.vx.anti_geometric(rhs.x) + self.vx.anti_geometric(rhs.y) + self.vx.anti_geometric(rhs.z) + self.vy.anti_geometric(rhs.x) + self.vy.anti_geometric(rhs.y) + self.vy.anti_geometric(rhs.z) + self.vz.anti_geometric(rhs.x) + self.vz.anti_geometric(rhs.y) + self.vz.anti_geometric(rhs.z)

// Line.dot(Dir) -> Point
impl Dot<Dir> for Line {
    type Output = Point;
    fn dot(self, rhs: Dir) -> Self::Output {
        Point {
            x: -self.my.dot(rhs.z) + self.mz.dot(rhs.y),
            y: self.mx.dot(rhs.z) - self.mz.dot(rhs.x),
            z: -self.mx.dot(rhs.y) + self.my.dot(rhs.x),
            w: self.vx.dot(rhs.x) + self.vy.dot(rhs.y) + self.vz.dot(rhs.z),
        }
    }
}

// Line.wedge(Dir) -> Plane
impl Wedge<Dir> for Line {
    type Output = Plane;
    fn wedge(self, rhs: Dir) -> Self::Output {
        Plane {
            nx: self.vy.wedge(rhs.z) - self.vz.wedge(rhs.y),
            ny: -self.vx.wedge(rhs.z) + self.vz.wedge(rhs.x),
            nz: self.vx.wedge(rhs.y) - self.vy.wedge(rhs.x),
            d: -self.mx.wedge(rhs.x) - self.my.wedge(rhs.y) - self.mz.wedge(rhs.z),
        }
    }
}

// Omitted: Line anti_wedge Dir = 0

// ---------------------------------------------------------------------
// Line OP Point:

// Omitted: Line geometric Point = self.mx.geometric(rhs.w) + self.mx.geometric(rhs.x) + self.mx.geometric(rhs.y) + self.mx.geometric(rhs.z) + self.my.geometric(rhs.w) + self.my.geometric(rhs.x) + self.my.geometric(rhs.y) + self.my.geometric(rhs.z) + self.mz.geometric(rhs.w) + self.mz.geometric(rhs.x) + self.mz.geometric(rhs.y) + self.mz.geometric(rhs.z) + self.vx.geometric(rhs.x) + self.vx.geometric(rhs.y) + self.vx.geometric(rhs.z) + self.vy.geometric(rhs.x) + self.vy.geometric(rhs.y) + self.vy.geometric(rhs.z) + self.vz.geometric(rhs.x) + self.vz.geometric(rhs.y) + self.vz.geometric(rhs.z)
// Omitted: Line anti_geometric Point = self.mx.anti_geometric(rhs.w) + self.my.anti_geometric(rhs.w) + self.mz.anti_geometric(rhs.w) + self.vx.anti_geometric(rhs.w) + self.vx.anti_geometric(rhs.x) + self.vx.anti_geometric(rhs.y) + self.vx.anti_geometric(rhs.z) + self.vy.anti_geometric(rhs.w) + self.vy.anti_geometric(rhs.x) + self.vy.anti_geometric(rhs.y) + self.vy.anti_geometric(rhs.z) + self.vz.anti_geometric(rhs.w) + self.vz.anti_geometric(rhs.x) + self.vz.anti_geometric(rhs.y) + self.vz.anti_geometric(rhs.z)

// Line.dot(Point) -> Point
impl Dot<Point> for Line {
    type Output = Point;
    fn dot(self, rhs: Point) -> Self::Output {
        Point {
            x: -self.my.dot(rhs.z) + self.mz.dot(rhs.y),
            y: self.mx.dot(rhs.z) - self.mz.dot(rhs.x),
            z: -self.mx.dot(rhs.y) + self.my.dot(rhs.x),
            w: self.vx.dot(rhs.x) + self.vy.dot(rhs.y) + self.vz.dot(rhs.z),
        }
    }
}

// Line.wedge(Point) -> Plane
impl Wedge<Point> for Line {
    type Output = Plane;
    fn wedge(self, rhs: Point) -> Self::Output {
        Plane {
            nx: self.mx.wedge(rhs.w) + self.vy.wedge(rhs.z) - self.vz.wedge(rhs.y),
            ny: self.my.wedge(rhs.w) - self.vx.wedge(rhs.z) + self.vz.wedge(rhs.x),
            nz: self.mz.wedge(rhs.w) + self.vx.wedge(rhs.y) - self.vy.wedge(rhs.x),
            d: -self.mx.wedge(rhs.x) - self.my.wedge(rhs.y) - self.mz.wedge(rhs.z),
        }
    }
}

// Omitted: Line anti_wedge Point = 0

// ---------------------------------------------------------------------
// Line OP Moment:

// Omitted: Line geometric Moment = self.mx.geometric(rhs.xy) + self.mx.geometric(rhs.yz) + self.mx.geometric(rhs.zx) + self.my.geometric(rhs.xy) + self.my.geometric(rhs.yz) + self.my.geometric(rhs.zx) + self.mz.geometric(rhs.xy) + self.mz.geometric(rhs.yz) + self.mz.geometric(rhs.zx) + self.vx.geometric(rhs.xy) + self.vx.geometric(rhs.yz) + self.vx.geometric(rhs.zx) + self.vy.geometric(rhs.xy) + self.vy.geometric(rhs.yz) + self.vy.geometric(rhs.zx) + self.vz.geometric(rhs.xy) + self.vz.geometric(rhs.yz) + self.vz.geometric(rhs.zx)
// Omitted: Line anti_geometric Moment = self.vx.anti_geometric(rhs.xy) + self.vx.anti_geometric(rhs.yz) + self.vx.anti_geometric(rhs.zx) + self.vy.anti_geometric(rhs.xy) + self.vy.anti_geometric(rhs.yz) + self.vy.anti_geometric(rhs.zx) + self.vz.anti_geometric(rhs.xy) + self.vz.anti_geometric(rhs.yz) + self.vz.anti_geometric(rhs.zx)

// Line.dot(Moment) -> S
impl Dot<Moment> for Line {
    type Output = S;
    fn dot(self, rhs: Moment) -> Self::Output {
        self.mx.dot(rhs.yz) + self.my.dot(rhs.zx) + self.mz.dot(rhs.xy)
    }
}

// Line.wedge(Moment) -> XYZW
impl Wedge<Moment> for Line {
    type Output = XYZW;
    fn wedge(self, rhs: Moment) -> Self::Output {
        self.vx.wedge(rhs.yz) + self.vy.wedge(rhs.zx) + self.vz.wedge(rhs.xy)
    }
}

// Line.anti_wedge(Moment) -> S
impl AntiWedge<Moment> for Line {
    type Output = S;
    fn anti_wedge(self, rhs: Moment) -> Self::Output {
        self.vx.anti_wedge(rhs.yz) + self.vy.anti_wedge(rhs.zx) + self.vz.anti_wedge(rhs.xy)
    }
}

// ---------------------------------------------------------------------
// Line OP Line:

// Omitted: Line geometric Line = self.mx.geometric(rhs.mx) + self.mx.geometric(rhs.my) + self.mx.geometric(rhs.mz) + self.mx.geometric(rhs.vx) + self.mx.geometric(rhs.vy) + self.mx.geometric(rhs.vz) + self.my.geometric(rhs.mx) + self.my.geometric(rhs.my) + self.my.geometric(rhs.mz) + self.my.geometric(rhs.vx) + self.my.geometric(rhs.vy) + self.my.geometric(rhs.vz) + self.mz.geometric(rhs.mx) + self.mz.geometric(rhs.my) + self.mz.geometric(rhs.mz) + self.mz.geometric(rhs.vx) + self.mz.geometric(rhs.vy) + self.mz.geometric(rhs.vz) + self.vx.geometric(rhs.mx) + self.vx.geometric(rhs.my) + self.vx.geometric(rhs.mz) + self.vy.geometric(rhs.mx) + self.vy.geometric(rhs.my) + self.vy.geometric(rhs.mz) + self.vz.geometric(rhs.mx) + self.vz.geometric(rhs.my) + self.vz.geometric(rhs.mz)
// Omitted: Line anti_geometric Line = self.mx.anti_geometric(rhs.vx) + self.mx.anti_geometric(rhs.vy) + self.mx.anti_geometric(rhs.vz) + self.my.anti_geometric(rhs.vx) + self.my.anti_geometric(rhs.vy) + self.my.anti_geometric(rhs.vz) + self.mz.anti_geometric(rhs.vx) + self.mz.anti_geometric(rhs.vy) + self.mz.anti_geometric(rhs.vz) + self.vx.anti_geometric(rhs.mx) + self.vx.anti_geometric(rhs.my) + self.vx.anti_geometric(rhs.mz) + self.vx.anti_geometric(rhs.vx) + self.vx.anti_geometric(rhs.vy) + self.vx.anti_geometric(rhs.vz) + self.vy.anti_geometric(rhs.mx) + self.vy.anti_geometric(rhs.my) + self.vy.anti_geometric(rhs.mz) + self.vy.anti_geometric(rhs.vx) + self.vy.anti_geometric(rhs.vy) + self.vy.anti_geometric(rhs.vz) + self.vz.anti_geometric(rhs.mx) + self.vz.anti_geometric(rhs.my) + self.vz.anti_geometric(rhs.mz) + self.vz.anti_geometric(rhs.vx) + self.vz.anti_geometric(rhs.vy) + self.vz.anti_geometric(rhs.vz)

// Line.dot(Line) -> S
impl Dot<Line> for Line {
    type Output = S;
    fn dot(self, rhs: Line) -> Self::Output {
        self.mx.dot(rhs.mx) + self.my.dot(rhs.my) + self.mz.dot(rhs.mz)
    }
}

// Line.wedge(Line) -> XYZW
impl Wedge<Line> for Line {
    type Output = XYZW;
    fn wedge(self, rhs: Line) -> Self::Output {
        self.mx.wedge(rhs.vx)
            + self.my.wedge(rhs.vy)
            + self.mz.wedge(rhs.vz)
            + self.vx.wedge(rhs.mx)
            + self.vy.wedge(rhs.my)
            + self.vz.wedge(rhs.mz)
    }
}

// Line.anti_wedge(Line) -> S
impl AntiWedge<Line> for Line {
    type Output = S;
    fn anti_wedge(self, rhs: Line) -> Self::Output {
        self.mx.anti_wedge(rhs.vx)
            + self.my.anti_wedge(rhs.vy)
            + self.mz.anti_wedge(rhs.vz)
            + self.vx.anti_wedge(rhs.mx)
            + self.vy.anti_wedge(rhs.my)
            + self.vz.anti_wedge(rhs.mz)
    }
}

// ---------------------------------------------------------------------
// Line OP Plane:

// Omitted: Line geometric Plane = self.mx.geometric(rhs.d) + self.mx.geometric(rhs.nx) + self.mx.geometric(rhs.ny) + self.mx.geometric(rhs.nz) + self.my.geometric(rhs.d) + self.my.geometric(rhs.nx) + self.my.geometric(rhs.ny) + self.my.geometric(rhs.nz) + self.mz.geometric(rhs.d) + self.mz.geometric(rhs.nx) + self.mz.geometric(rhs.ny) + self.mz.geometric(rhs.nz) + self.vx.geometric(rhs.d) + self.vy.geometric(rhs.d) + self.vz.geometric(rhs.d)
// Omitted: Line anti_geometric Plane = self.mx.anti_geometric(rhs.nx) + self.mx.anti_geometric(rhs.ny) + self.mx.anti_geometric(rhs.nz) + self.my.anti_geometric(rhs.nx) + self.my.anti_geometric(rhs.ny) + self.my.anti_geometric(rhs.nz) + self.mz.anti_geometric(rhs.nx) + self.mz.anti_geometric(rhs.ny) + self.mz.anti_geometric(rhs.nz) + self.vx.anti_geometric(rhs.d) + self.vx.anti_geometric(rhs.nx) + self.vx.anti_geometric(rhs.ny) + self.vx.anti_geometric(rhs.nz) + self.vy.anti_geometric(rhs.d) + self.vy.anti_geometric(rhs.nx) + self.vy.anti_geometric(rhs.ny) + self.vy.anti_geometric(rhs.nz) + self.vz.anti_geometric(rhs.d) + self.vz.anti_geometric(rhs.nx) + self.vz.anti_geometric(rhs.ny) + self.vz.anti_geometric(rhs.nz)

// Line.dot(Plane) -> Point
impl Dot<Plane> for Line {
    type Output = Point;
    fn dot(self, rhs: Plane) -> Self::Output {
        Point {
            x: self.mx.dot(rhs.d),
            y: self.my.dot(rhs.d),
            z: self.mz.dot(rhs.d),
            w: -self.mx.dot(rhs.nx) - self.my.dot(rhs.ny) - self.mz.dot(rhs.nz),
        }
    }
}

// Omitted: Line wedge Plane = 0

// Line.anti_wedge(Plane) -> Point
impl AntiWedge<Plane> for Line {
    type Output = Point;
    fn anti_wedge(self, rhs: Plane) -> Self::Output {
        Point {
            x: self.my.anti_wedge(rhs.nz) - self.mz.anti_wedge(rhs.ny) + self.vx.anti_wedge(rhs.d),
            y: -self.mx.anti_wedge(rhs.nz) + self.mz.anti_wedge(rhs.nx) + self.vy.anti_wedge(rhs.d),
            z: self.mx.anti_wedge(rhs.ny) - self.my.anti_wedge(rhs.nx) + self.vz.anti_wedge(rhs.d),
            w: -self.vx.anti_wedge(rhs.nx)
                - self.vy.anti_wedge(rhs.ny)
                - self.vz.anti_wedge(rhs.nz),
        }
    }
}

// ---------------------------------------------------------------------
// Line OP Translator:

// Omitted: Line geometric Translator = self.mx.geometric(rhs.x) + self.mx.geometric(rhs.xyzw) + self.mx.geometric(rhs.y) + self.mx.geometric(rhs.z) + self.my.geometric(rhs.x) + self.my.geometric(rhs.xyzw) + self.my.geometric(rhs.y) + self.my.geometric(rhs.z) + self.mz.geometric(rhs.x) + self.mz.geometric(rhs.xyzw) + self.mz.geometric(rhs.y) + self.mz.geometric(rhs.z) + self.vx.geometric(rhs.x) + self.vx.geometric(rhs.y) + self.vx.geometric(rhs.z) + self.vy.geometric(rhs.x) + self.vy.geometric(rhs.y) + self.vy.geometric(rhs.z) + self.vz.geometric(rhs.x) + self.vz.geometric(rhs.y) + self.vz.geometric(rhs.z)
// Omitted: Line anti_geometric Translator = self.mx.anti_geometric(rhs.xyzw) + self.my.anti_geometric(rhs.xyzw) + self.mz.anti_geometric(rhs.xyzw) + self.vx.anti_geometric(rhs.x) + self.vx.anti_geometric(rhs.xyzw) + self.vx.anti_geometric(rhs.y) + self.vx.anti_geometric(rhs.z) + self.vy.anti_geometric(rhs.x) + self.vy.anti_geometric(rhs.xyzw) + self.vy.anti_geometric(rhs.y) + self.vy.anti_geometric(rhs.z) + self.vz.anti_geometric(rhs.x) + self.vz.anti_geometric(rhs.xyzw) + self.vz.anti_geometric(rhs.y) + self.vz.anti_geometric(rhs.z)

// Line.dot(Translator) -> Motor
impl Dot<Translator> for Line {
    type Output = Motor;
    fn dot(self, rhs: Translator) -> Self::Output {
        Motor {
            rx: self.mx.dot(rhs.xyzw),
            ry: self.my.dot(rhs.xyzw),
            rz: self.mz.dot(rhs.xyzw),
            rw: Default::default(),
            ux: Default::default(),
            uy: Default::default(),
            uz: Default::default(),
            uw: -self.mx.dot(rhs.x) - self.my.dot(rhs.y) - self.mz.dot(rhs.z),
        }
    }
}

// Line.wedge(Translator) -> XYZW
impl Wedge<Translator> for Line {
    type Output = XYZW;
    fn wedge(self, rhs: Translator) -> Self::Output {
        self.vx.wedge(rhs.x) + self.vy.wedge(rhs.y) + self.vz.wedge(rhs.z)
    }
}

// Omitted: Line anti_wedge Translator = self.mx.anti_wedge(rhs.xyzw) + self.my.anti_wedge(rhs.xyzw) + self.mz.anti_wedge(rhs.xyzw) + self.vx.anti_wedge(rhs.x) + self.vx.anti_wedge(rhs.xyzw) + self.vy.anti_wedge(rhs.xyzw) + self.vy.anti_wedge(rhs.y) + self.vz.anti_wedge(rhs.xyzw) + self.vz.anti_wedge(rhs.z)

// ---------------------------------------------------------------------
// Line OP Rotor:

// Line.geometric(Rotor) -> Rotor
impl Geometric<Rotor> for Line {
    type Output = Rotor;
    fn geometric(self, rhs: Rotor) -> Self::Output {
        Rotor {
            x: self.mx.geometric(rhs.w) - self.my.geometric(rhs.z) + self.mz.geometric(rhs.y),
            y: self.mx.geometric(rhs.z) + self.my.geometric(rhs.w) - self.mz.geometric(rhs.x),
            z: -self.mx.geometric(rhs.y) + self.my.geometric(rhs.x) + self.mz.geometric(rhs.w),
            w: -self.mx.geometric(rhs.x) - self.my.geometric(rhs.y) - self.mz.geometric(rhs.z),
        }
    }
}

// Omitted: Line anti_geometric Rotor = self.mx.anti_geometric(rhs.w) + self.mx.anti_geometric(rhs.x) + self.mx.anti_geometric(rhs.y) + self.mx.anti_geometric(rhs.z) + self.my.anti_geometric(rhs.w) + self.my.anti_geometric(rhs.x) + self.my.anti_geometric(rhs.y) + self.my.anti_geometric(rhs.z) + self.mz.anti_geometric(rhs.w) + self.mz.anti_geometric(rhs.x) + self.mz.anti_geometric(rhs.y) + self.mz.anti_geometric(rhs.z) + self.vx.anti_geometric(rhs.w) + self.vx.anti_geometric(rhs.x) + self.vx.anti_geometric(rhs.y) + self.vx.anti_geometric(rhs.z) + self.vy.anti_geometric(rhs.w) + self.vy.anti_geometric(rhs.x) + self.vy.anti_geometric(rhs.y) + self.vy.anti_geometric(rhs.z) + self.vz.anti_geometric(rhs.w) + self.vz.anti_geometric(rhs.x) + self.vz.anti_geometric(rhs.y) + self.vz.anti_geometric(rhs.z)

// Line.dot(Rotor) -> Line
impl Dot<Rotor> for Line {
    type Output = Line;
    fn dot(self, rhs: Rotor) -> Self::Output {
        Line {
            vx: self.mx.dot(rhs.w),
            vy: self.my.dot(rhs.w),
            vz: self.mz.dot(rhs.w),
            mx: Default::default(),
            my: Default::default(),
            mz: Default::default(),
        }
    }
}

// Line.wedge(Rotor) -> XYZW
impl Wedge<Rotor> for Line {
    type Output = XYZW;
    fn wedge(self, rhs: Rotor) -> Self::Output {
        self.mx.wedge(rhs.x) + self.my.wedge(rhs.y) + self.mz.wedge(rhs.z)
    }
}

// Omitted: Line anti_wedge Rotor = self.mx.anti_wedge(rhs.w) + self.mx.anti_wedge(rhs.x) + self.my.anti_wedge(rhs.w) + self.my.anti_wedge(rhs.y) + self.mz.anti_wedge(rhs.w) + self.mz.anti_wedge(rhs.z) + self.vx.anti_wedge(rhs.w) + self.vy.anti_wedge(rhs.w) + self.vz.anti_wedge(rhs.w)

// ---------------------------------------------------------------------
// Line OP Motor:

// Omitted: Line geometric Motor = self.mx.geometric(rhs.rw) + self.mx.geometric(rhs.rx) + self.mx.geometric(rhs.ry) + self.mx.geometric(rhs.rz) + self.mx.geometric(rhs.uw) + self.mx.geometric(rhs.ux) + self.mx.geometric(rhs.uy) + self.mx.geometric(rhs.uz) + self.my.geometric(rhs.rw) + self.my.geometric(rhs.rx) + self.my.geometric(rhs.ry) + self.my.geometric(rhs.rz) + self.my.geometric(rhs.uw) + self.my.geometric(rhs.ux) + self.my.geometric(rhs.uy) + self.my.geometric(rhs.uz) + self.mz.geometric(rhs.rw) + self.mz.geometric(rhs.rx) + self.mz.geometric(rhs.ry) + self.mz.geometric(rhs.rz) + self.mz.geometric(rhs.uw) + self.mz.geometric(rhs.ux) + self.mz.geometric(rhs.uy) + self.mz.geometric(rhs.uz) + self.vx.geometric(rhs.uw) + self.vy.geometric(rhs.uw) + self.vz.geometric(rhs.uw)
// Omitted: Line anti_geometric Motor = self.mx.anti_geometric(rhs.rw) + self.mx.anti_geometric(rhs.rx) + self.mx.anti_geometric(rhs.ry) + self.mx.anti_geometric(rhs.rz) + self.mx.anti_geometric(rhs.ux) + self.mx.anti_geometric(rhs.uy) + self.mx.anti_geometric(rhs.uz) + self.my.anti_geometric(rhs.rw) + self.my.anti_geometric(rhs.rx) + self.my.anti_geometric(rhs.ry) + self.my.anti_geometric(rhs.rz) + self.my.anti_geometric(rhs.ux) + self.my.anti_geometric(rhs.uy) + self.my.anti_geometric(rhs.uz) + self.mz.anti_geometric(rhs.rw) + self.mz.anti_geometric(rhs.rx) + self.mz.anti_geometric(rhs.ry) + self.mz.anti_geometric(rhs.rz) + self.mz.anti_geometric(rhs.ux) + self.mz.anti_geometric(rhs.uy) + self.mz.anti_geometric(rhs.uz) + self.vx.anti_geometric(rhs.rw) + self.vx.anti_geometric(rhs.rx) + self.vx.anti_geometric(rhs.ry) + self.vx.anti_geometric(rhs.rz) + self.vx.anti_geometric(rhs.uw) + self.vx.anti_geometric(rhs.ux) + self.vx.anti_geometric(rhs.uy) + self.vx.anti_geometric(rhs.uz) + self.vy.anti_geometric(rhs.rw) + self.vy.anti_geometric(rhs.rx) + self.vy.anti_geometric(rhs.ry) + self.vy.anti_geometric(rhs.rz) + self.vy.anti_geometric(rhs.uw) + self.vy.anti_geometric(rhs.ux) + self.vy.anti_geometric(rhs.uy) + self.vy.anti_geometric(rhs.uz) + self.vz.anti_geometric(rhs.rw) + self.vz.anti_geometric(rhs.rx) + self.vz.anti_geometric(rhs.ry) + self.vz.anti_geometric(rhs.rz) + self.vz.anti_geometric(rhs.uw) + self.vz.anti_geometric(rhs.ux) + self.vz.anti_geometric(rhs.uy) + self.vz.anti_geometric(rhs.uz)
// Omitted: Line dot Motor = self.mx.dot(rhs.rw) + self.mx.dot(rhs.uw) + self.mx.dot(rhs.ux) + self.my.dot(rhs.rw) + self.my.dot(rhs.uw) + self.my.dot(rhs.uy) + self.mz.dot(rhs.rw) + self.mz.dot(rhs.uw) + self.mz.dot(rhs.uz) + self.vx.dot(rhs.uw) + self.vy.dot(rhs.uw) + self.vz.dot(rhs.uw)
// Omitted: Line wedge Motor = self.mx.wedge(rhs.rx) + self.mx.wedge(rhs.uw) + self.my.wedge(rhs.ry) + self.my.wedge(rhs.uw) + self.mz.wedge(rhs.rz) + self.mz.wedge(rhs.uw) + self.vx.wedge(rhs.uw) + self.vy.wedge(rhs.uw) + self.vz.wedge(rhs.uw)
// Omitted: Line anti_wedge Motor = self.mx.anti_wedge(rhs.rw) + self.mx.anti_wedge(rhs.rx) + self.mx.anti_wedge(rhs.uy) + self.mx.anti_wedge(rhs.uz) + self.my.anti_wedge(rhs.rw) + self.my.anti_wedge(rhs.ry) + self.my.anti_wedge(rhs.ux) + self.my.anti_wedge(rhs.uz) + self.mz.anti_wedge(rhs.rw) + self.mz.anti_wedge(rhs.rz) + self.mz.anti_wedge(rhs.ux) + self.mz.anti_wedge(rhs.uy) + self.vx.anti_wedge(rhs.rw) + self.vx.anti_wedge(rhs.ux) + self.vy.anti_wedge(rhs.rw) + self.vy.anti_wedge(rhs.uy) + self.vz.anti_wedge(rhs.rw) + self.vz.anti_wedge(rhs.uz)

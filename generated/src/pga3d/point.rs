//! # Point
//!
//! ## Operations
//! ```text
//! Point.anti_geometric(Point) -> Translator
//! Point.dot(Point) -> S
//! Point.wedge(Point) -> Line
//! Point.anti_geometric(Dir) -> Line
//! Dir.anti_geometric(Point) -> Line
//! Point.dot(Dir) -> S
//! Dir.dot(Point) -> S
//! Point.wedge(Dir) -> Line
//! Dir.wedge(Point) -> Line
//! Point.dot(Line) -> Point
//! Line.dot(Point) -> Point
//! Point.wedge(Line) -> Plane
//! Line.wedge(Point) -> Plane
//! Point.dot(Plane) -> Line
//! Plane.dot(Point) -> Line
//! Point.wedge(Plane) -> XYZW
//! Plane.wedge(Point) -> XYZW
//! Point.anti_wedge(Plane) -> S
//! Plane.anti_wedge(Point) -> S
//! Point.anti_geometric(Translator) -> Point
//! Translator.anti_geometric(Point) -> Point
//! Point.wedge(Translator) -> Plane
//! Translator.wedge(Point) -> Plane
//! Point.anti_wedge(Translator) -> Point
//! Translator.anti_wedge(Point) -> Point
//! Point.wedge(Rotor) -> Plane
//! Rotor.wedge(Point) -> Plane
//! Point.anti_wedge(Rotor) -> Point
//! Rotor.anti_wedge(Point) -> Point
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
pub struct Point {
    pub x: X,
    pub y: Y,
    pub z: Z,
    pub w: W,
}

// ---------------------------------------------------------------------
// Point OP Dir:

// Omitted: Point geometric Dir = self.w.geometric(rhs.x) + self.w.geometric(rhs.y) + self.w.geometric(rhs.z) + self.x.geometric(rhs.x) + self.x.geometric(rhs.y) + self.x.geometric(rhs.z) + self.y.geometric(rhs.x) + self.y.geometric(rhs.y) + self.y.geometric(rhs.z) + self.z.geometric(rhs.x) + self.z.geometric(rhs.y) + self.z.geometric(rhs.z)

// Point.anti_geometric(Dir) -> Line
impl AntiGeometric<Dir> for Point {
    type Output = Line;
    fn anti_geometric(self, rhs: Dir) -> Self::Output {
        Line {
            vx: Default::default(),
            vy: Default::default(),
            vz: Default::default(),
            mx: self.w.anti_geometric(rhs.x),
            my: -self.w.anti_geometric(rhs.y),
            mz: self.w.anti_geometric(rhs.z),
        }
    }
}

// Point.dot(Dir) -> S
impl Dot<Dir> for Point {
    type Output = S;
    fn dot(self, rhs: Dir) -> Self::Output {
        self.x.dot(rhs.x) + self.y.dot(rhs.y) + self.z.dot(rhs.z)
    }
}

// Point.wedge(Dir) -> Line
impl Wedge<Dir> for Point {
    type Output = Line;
    fn wedge(self, rhs: Dir) -> Self::Output {
        Line {
            vx: -self.w.wedge(rhs.x),
            vy: -self.w.wedge(rhs.y),
            vz: -self.w.wedge(rhs.z),
            mx: self.y.wedge(rhs.z) - self.z.wedge(rhs.y),
            my: self.x.wedge(rhs.z) - self.z.wedge(rhs.x),
            mz: self.x.wedge(rhs.y) - self.y.wedge(rhs.x),
        }
    }
}

// Omitted: Point anti_wedge Dir = 0

// ---------------------------------------------------------------------
// Point OP Point:

// Omitted: Point geometric Point = self.w.geometric(rhs.x) + self.w.geometric(rhs.y) + self.w.geometric(rhs.z) + self.x.geometric(rhs.w) + self.x.geometric(rhs.x) + self.x.geometric(rhs.y) + self.x.geometric(rhs.z) + self.y.geometric(rhs.w) + self.y.geometric(rhs.x) + self.y.geometric(rhs.y) + self.y.geometric(rhs.z) + self.z.geometric(rhs.w) + self.z.geometric(rhs.x) + self.z.geometric(rhs.y) + self.z.geometric(rhs.z)

// Point.anti_geometric(Point) -> Translator
impl AntiGeometric<Point> for Point {
    type Output = Translator;
    fn anti_geometric(self, rhs: Point) -> Self::Output {
        Translator {
            x: self.w.anti_geometric(rhs.x) - self.x.anti_geometric(rhs.w),
            y: -self.w.anti_geometric(rhs.y) + self.y.anti_geometric(rhs.w),
            z: self.w.anti_geometric(rhs.z) - self.z.anti_geometric(rhs.w),
            xyzw: -self.w.anti_geometric(rhs.w),
        }
    }
}

// Point.dot(Point) -> S
impl Dot<Point> for Point {
    type Output = S;
    fn dot(self, rhs: Point) -> Self::Output {
        self.x.dot(rhs.x) + self.y.dot(rhs.y) + self.z.dot(rhs.z)
    }
}

// Point.wedge(Point) -> Line
impl Wedge<Point> for Point {
    type Output = Line;
    fn wedge(self, rhs: Point) -> Self::Output {
        Line {
            vx: -self.w.wedge(rhs.x) + self.x.wedge(rhs.w),
            vy: -self.w.wedge(rhs.y) + self.y.wedge(rhs.w),
            vz: -self.w.wedge(rhs.z) + self.z.wedge(rhs.w),
            mx: self.y.wedge(rhs.z) - self.z.wedge(rhs.y),
            my: self.x.wedge(rhs.z) - self.z.wedge(rhs.x),
            mz: self.x.wedge(rhs.y) - self.y.wedge(rhs.x),
        }
    }
}

// Omitted: Point anti_wedge Point = 0

// ---------------------------------------------------------------------
// Point OP Line:

// Omitted: Point geometric Line = self.w.geometric(rhs.mx) + self.w.geometric(rhs.my) + self.w.geometric(rhs.mz) + self.x.geometric(rhs.mx) + self.x.geometric(rhs.my) + self.x.geometric(rhs.mz) + self.x.geometric(rhs.vx) + self.x.geometric(rhs.vy) + self.x.geometric(rhs.vz) + self.y.geometric(rhs.mx) + self.y.geometric(rhs.my) + self.y.geometric(rhs.mz) + self.y.geometric(rhs.vx) + self.y.geometric(rhs.vy) + self.y.geometric(rhs.vz) + self.z.geometric(rhs.mx) + self.z.geometric(rhs.my) + self.z.geometric(rhs.mz) + self.z.geometric(rhs.vx) + self.z.geometric(rhs.vy) + self.z.geometric(rhs.vz)
// Omitted: Point anti_geometric Line = self.w.anti_geometric(rhs.mx) + self.w.anti_geometric(rhs.my) + self.w.anti_geometric(rhs.mz) + self.w.anti_geometric(rhs.vx) + self.w.anti_geometric(rhs.vy) + self.w.anti_geometric(rhs.vz) + self.x.anti_geometric(rhs.vx) + self.x.anti_geometric(rhs.vy) + self.x.anti_geometric(rhs.vz) + self.y.anti_geometric(rhs.vx) + self.y.anti_geometric(rhs.vy) + self.y.anti_geometric(rhs.vz) + self.z.anti_geometric(rhs.vx) + self.z.anti_geometric(rhs.vy) + self.z.anti_geometric(rhs.vz)

// Point.dot(Line) -> Point
impl Dot<Line> for Point {
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

// Point.wedge(Line) -> Plane
impl Wedge<Line> for Point {
    type Output = Plane;
    fn wedge(self, rhs: Line) -> Self::Output {
        Plane {
            nx: self.w.wedge(rhs.mx) - self.y.wedge(rhs.vz) + self.z.wedge(rhs.vy),
            ny: -self.w.wedge(rhs.my) - self.x.wedge(rhs.vz) + self.z.wedge(rhs.vx),
            nz: self.w.wedge(rhs.mz) - self.x.wedge(rhs.vy) + self.y.wedge(rhs.vx),
            d: self.x.wedge(rhs.mx) + self.y.wedge(rhs.my) + self.z.wedge(rhs.mz),
        }
    }
}

// Omitted: Point anti_wedge Line = 0

// ---------------------------------------------------------------------
// Point OP Plane:

// Omitted: Point geometric Plane = self.w.geometric(rhs.d) + self.x.geometric(rhs.d) + self.x.geometric(rhs.nx) + self.x.geometric(rhs.ny) + self.x.geometric(rhs.nz) + self.y.geometric(rhs.d) + self.y.geometric(rhs.nx) + self.y.geometric(rhs.ny) + self.y.geometric(rhs.nz) + self.z.geometric(rhs.d) + self.z.geometric(rhs.nx) + self.z.geometric(rhs.ny) + self.z.geometric(rhs.nz)
// Omitted: Point anti_geometric Plane = self.w.anti_geometric(rhs.d) + self.w.anti_geometric(rhs.nx) + self.w.anti_geometric(rhs.ny) + self.w.anti_geometric(rhs.nz) + self.x.anti_geometric(rhs.nx) + self.x.anti_geometric(rhs.ny) + self.x.anti_geometric(rhs.nz) + self.y.anti_geometric(rhs.nx) + self.y.anti_geometric(rhs.ny) + self.y.anti_geometric(rhs.nz) + self.z.anti_geometric(rhs.nx) + self.z.anti_geometric(rhs.ny) + self.z.anti_geometric(rhs.nz)

// Point.dot(Plane) -> Line
impl Dot<Plane> for Point {
    type Output = Line;
    fn dot(self, rhs: Plane) -> Self::Output {
        Line {
            vx: -self.y.dot(rhs.nz) + self.z.dot(rhs.ny),
            vy: self.x.dot(rhs.nz) - self.z.dot(rhs.nx),
            vz: -self.x.dot(rhs.ny) + self.y.dot(rhs.nx),
            mx: -self.x.dot(rhs.d),
            my: self.y.dot(rhs.d),
            mz: -self.z.dot(rhs.d),
        }
    }
}

// Point.wedge(Plane) -> XYZW
impl Wedge<Plane> for Point {
    type Output = XYZW;
    fn wedge(self, rhs: Plane) -> Self::Output {
        self.w.wedge(rhs.d) + self.x.wedge(rhs.nx) + self.y.wedge(rhs.ny) + self.z.wedge(rhs.nz)
    }
}

// Point.anti_wedge(Plane) -> S
impl AntiWedge<Plane> for Point {
    type Output = S;
    fn anti_wedge(self, rhs: Plane) -> Self::Output {
        self.w.anti_wedge(rhs.d)
            + self.x.anti_wedge(rhs.nx)
            + self.y.anti_wedge(rhs.ny)
            + self.z.anti_wedge(rhs.nz)
    }
}

// ---------------------------------------------------------------------
// Point OP Translator:

// Omitted: Point geometric Translator = self.w.geometric(rhs.x) + self.w.geometric(rhs.y) + self.w.geometric(rhs.z) + self.x.geometric(rhs.x) + self.x.geometric(rhs.xyzw) + self.x.geometric(rhs.y) + self.x.geometric(rhs.z) + self.y.geometric(rhs.x) + self.y.geometric(rhs.xyzw) + self.y.geometric(rhs.y) + self.y.geometric(rhs.z) + self.z.geometric(rhs.x) + self.z.geometric(rhs.xyzw) + self.z.geometric(rhs.y) + self.z.geometric(rhs.z)

// Point.anti_geometric(Translator) -> Point
impl AntiGeometric<Translator> for Point {
    type Output = Point;
    fn anti_geometric(self, rhs: Translator) -> Self::Output {
        Point {
            x: -self.w.anti_geometric(rhs.x) + self.x.anti_geometric(rhs.xyzw),
            y: -self.w.anti_geometric(rhs.y) + self.y.anti_geometric(rhs.xyzw),
            z: -self.w.anti_geometric(rhs.z) + self.z.anti_geometric(rhs.xyzw),
            w: self.w.anti_geometric(rhs.xyzw),
        }
    }
}

// Omitted: Point dot Translator = self.x.dot(rhs.xyzw) + self.x.dot(rhs.y) + self.x.dot(rhs.z) + self.y.dot(rhs.x) + self.y.dot(rhs.xyzw) + self.y.dot(rhs.z) + self.z.dot(rhs.x) + self.z.dot(rhs.xyzw) + self.z.dot(rhs.y)

// Point.wedge(Translator) -> Plane
impl Wedge<Translator> for Point {
    type Output = Plane;
    fn wedge(self, rhs: Translator) -> Self::Output {
        Plane {
            nx: self.w.wedge(rhs.x),
            ny: -self.w.wedge(rhs.y),
            nz: self.w.wedge(rhs.z),
            d: self.x.wedge(rhs.x) + self.y.wedge(rhs.y) + self.z.wedge(rhs.z),
        }
    }
}

// Point.anti_wedge(Translator) -> Point
impl AntiWedge<Translator> for Point {
    type Output = Point;
    fn anti_wedge(self, rhs: Translator) -> Self::Output {
        Point {
            x: self.x.anti_wedge(rhs.xyzw),
            y: self.y.anti_wedge(rhs.xyzw),
            z: self.z.anti_wedge(rhs.xyzw),
            w: self.w.anti_wedge(rhs.xyzw),
        }
    }
}

// ---------------------------------------------------------------------
// Point OP Rotor:

// Omitted: Point geometric Rotor = self.x.geometric(rhs.w) + self.x.geometric(rhs.x) + self.x.geometric(rhs.y) + self.x.geometric(rhs.z) + self.y.geometric(rhs.w) + self.y.geometric(rhs.x) + self.y.geometric(rhs.y) + self.y.geometric(rhs.z) + self.z.geometric(rhs.w) + self.z.geometric(rhs.x) + self.z.geometric(rhs.y) + self.z.geometric(rhs.z)
// Omitted: Point anti_geometric Rotor = self.w.anti_geometric(rhs.w) + self.w.anti_geometric(rhs.x) + self.w.anti_geometric(rhs.y) + self.w.anti_geometric(rhs.z) + self.x.anti_geometric(rhs.w) + self.x.anti_geometric(rhs.x) + self.x.anti_geometric(rhs.y) + self.x.anti_geometric(rhs.z) + self.y.anti_geometric(rhs.w) + self.y.anti_geometric(rhs.x) + self.y.anti_geometric(rhs.y) + self.y.anti_geometric(rhs.z) + self.z.anti_geometric(rhs.w) + self.z.anti_geometric(rhs.x) + self.z.anti_geometric(rhs.y) + self.z.anti_geometric(rhs.z)
// Omitted: Point dot Rotor = self.x.dot(rhs.w) + self.x.dot(rhs.x) + self.y.dot(rhs.w) + self.y.dot(rhs.y) + self.z.dot(rhs.w) + self.z.dot(rhs.z)

// Point.wedge(Rotor) -> Plane
impl Wedge<Rotor> for Point {
    type Output = Plane;
    fn wedge(self, rhs: Rotor) -> Self::Output {
        Plane {
            nx: -self.y.wedge(rhs.z) + self.z.wedge(rhs.y),
            ny: -self.x.wedge(rhs.z) + self.z.wedge(rhs.x),
            nz: -self.x.wedge(rhs.y) + self.y.wedge(rhs.x),
            d: Default::default(),
        }
    }
}

// Point.anti_wedge(Rotor) -> Point
impl AntiWedge<Rotor> for Point {
    type Output = Point;
    fn anti_wedge(self, rhs: Rotor) -> Self::Output {
        Point {
            x: self.x.anti_wedge(rhs.w),
            y: self.y.anti_wedge(rhs.w),
            z: self.z.anti_wedge(rhs.w),
            w: self.w.anti_wedge(rhs.w),
        }
    }
}

// ---------------------------------------------------------------------
// Point OP Motor:

// Omitted: Point geometric Motor = self.w.geometric(rhs.uw) + self.x.geometric(rhs.rw) + self.x.geometric(rhs.rx) + self.x.geometric(rhs.ry) + self.x.geometric(rhs.rz) + self.x.geometric(rhs.uw) + self.x.geometric(rhs.ux) + self.x.geometric(rhs.uy) + self.x.geometric(rhs.uz) + self.y.geometric(rhs.rw) + self.y.geometric(rhs.rx) + self.y.geometric(rhs.ry) + self.y.geometric(rhs.rz) + self.y.geometric(rhs.uw) + self.y.geometric(rhs.ux) + self.y.geometric(rhs.uy) + self.y.geometric(rhs.uz) + self.z.geometric(rhs.rw) + self.z.geometric(rhs.rx) + self.z.geometric(rhs.ry) + self.z.geometric(rhs.rz) + self.z.geometric(rhs.uw) + self.z.geometric(rhs.ux) + self.z.geometric(rhs.uy) + self.z.geometric(rhs.uz)
// Omitted: Point anti_geometric Motor = self.w.anti_geometric(rhs.rw) + self.w.anti_geometric(rhs.rx) + self.w.anti_geometric(rhs.ry) + self.w.anti_geometric(rhs.rz) + self.w.anti_geometric(rhs.uw) + self.w.anti_geometric(rhs.ux) + self.w.anti_geometric(rhs.uy) + self.w.anti_geometric(rhs.uz) + self.x.anti_geometric(rhs.rw) + self.x.anti_geometric(rhs.rx) + self.x.anti_geometric(rhs.ry) + self.x.anti_geometric(rhs.rz) + self.x.anti_geometric(rhs.ux) + self.x.anti_geometric(rhs.uy) + self.x.anti_geometric(rhs.uz) + self.y.anti_geometric(rhs.rw) + self.y.anti_geometric(rhs.rx) + self.y.anti_geometric(rhs.ry) + self.y.anti_geometric(rhs.rz) + self.y.anti_geometric(rhs.ux) + self.y.anti_geometric(rhs.uy) + self.y.anti_geometric(rhs.uz) + self.z.anti_geometric(rhs.rw) + self.z.anti_geometric(rhs.rx) + self.z.anti_geometric(rhs.ry) + self.z.anti_geometric(rhs.rz) + self.z.anti_geometric(rhs.ux) + self.z.anti_geometric(rhs.uy) + self.z.anti_geometric(rhs.uz)
// Omitted: Point dot Motor = self.w.dot(rhs.uw) + self.x.dot(rhs.rw) + self.x.dot(rhs.rx) + self.x.dot(rhs.uw) + self.x.dot(rhs.uy) + self.x.dot(rhs.uz) + self.y.dot(rhs.rw) + self.y.dot(rhs.ry) + self.y.dot(rhs.uw) + self.y.dot(rhs.ux) + self.y.dot(rhs.uz) + self.z.dot(rhs.rw) + self.z.dot(rhs.rz) + self.z.dot(rhs.uw) + self.z.dot(rhs.ux) + self.z.dot(rhs.uy)
// Omitted: Point wedge Motor = self.w.wedge(rhs.uw) + self.x.wedge(rhs.ry) + self.x.wedge(rhs.rz) + self.x.wedge(rhs.uw) + self.x.wedge(rhs.ux) + self.y.wedge(rhs.rx) + self.y.wedge(rhs.rz) + self.y.wedge(rhs.uw) + self.y.wedge(rhs.uy) + self.z.wedge(rhs.rx) + self.z.wedge(rhs.ry) + self.z.wedge(rhs.uw) + self.z.wedge(rhs.uz)
// Omitted: Point anti_wedge Motor = self.w.anti_wedge(rhs.rw) + self.x.anti_wedge(rhs.rw) + self.x.anti_wedge(rhs.ux) + self.y.anti_wedge(rhs.rw) + self.y.anti_wedge(rhs.uy) + self.z.anti_wedge(rhs.rw) + self.z.anti_wedge(rhs.uz)
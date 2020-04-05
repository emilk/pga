//! # Rotor
//!
//! ## Operations
//! ```text
//! Rotor.anti_geometric(Rotor) -> Rotor
//! Rotor.anti_wedge(Rotor) -> Rotor
//! Rotor.wedge(Dir) -> Plane
//! Dir.wedge(Rotor) -> Plane
//! Rotor.anti_wedge(Dir) -> Dir
//! Dir.anti_wedge(Rotor) -> Dir
//! Rotor.wedge(Point) -> Plane
//! Point.wedge(Rotor) -> Plane
//! Rotor.anti_wedge(Point) -> Point
//! Point.anti_wedge(Rotor) -> Point
//! Rotor.geometric(Line) -> Rotor
//! Line.geometric(Rotor) -> Rotor
//! Rotor.dot(Line) -> Line
//! Line.dot(Rotor) -> Line
//! Rotor.wedge(Line) -> XYZW
//! Line.wedge(Rotor) -> XYZW
//! Rotor.dot(Plane) -> W
//! Plane.dot(Rotor) -> W
//! Rotor.geometric(Translator) -> Rotor
//! Translator.geometric(Rotor) -> Rotor
//! Rotor.dot(Translator) -> Line
//! Translator.dot(Rotor) -> Line
//! Rotor.wedge(Translator) -> XYZW
//! Translator.wedge(Rotor) -> XYZW
//! Rotor.geometric(Motor) -> Rotor
//! Motor.geometric(Rotor) -> Rotor
//! Rotor.dot(Motor) -> Rotor
//! Motor.dot(Rotor) -> Rotor
//! Rotor.wedge(Motor) -> Rotor
//! Motor.wedge(Rotor) -> Rotor
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
pub struct Rotor {
    pub x: WX,
    pub y: WY,
    pub z: WZ,
    pub w: XYZW,
}

// ---------------------------------------------------------------------
// Omitted: Rotor.rcompl() -> self.w.rcompl() + self.x.rcompl() + self.y.rcompl() + self.z.rcompl()
// Omitted: Rotor.lcompl() -> self.w.lcompl() + self.x.lcompl() + self.y.lcompl() + self.z.lcompl()

impl Reverse for Rotor {
    fn rev(self) -> Self {
        Rotor {
            x: self.x.rev(),
            y: self.y.rev(),
            z: self.z.rev(),
            w: self.w.rev(),
        }
    }
}

impl AntiReverse for Rotor {
    fn arev(self) -> Self {
        Rotor {
            x: self.x.arev(),
            y: self.y.arev(),
            z: self.z.arev(),
            w: self.w.arev(),
        }
    }
}

// ---------------------------------------------------------------------
// Rotor OP Dir:

// Omitted: Rotor geometric Dir = self.w.geometric(rhs.x) + self.w.geometric(rhs.y) + self.w.geometric(rhs.z) + self.x.geometric(rhs.x) + self.x.geometric(rhs.y) + self.x.geometric(rhs.z) + self.y.geometric(rhs.x) + self.y.geometric(rhs.y) + self.y.geometric(rhs.z) + self.z.geometric(rhs.x) + self.z.geometric(rhs.y) + self.z.geometric(rhs.z)
// Omitted: Rotor anti_geometric Dir = self.w.anti_geometric(rhs.x) + self.w.anti_geometric(rhs.y) + self.w.anti_geometric(rhs.z) + self.x.anti_geometric(rhs.x) + self.x.anti_geometric(rhs.y) + self.x.anti_geometric(rhs.z) + self.y.anti_geometric(rhs.x) + self.y.anti_geometric(rhs.y) + self.y.anti_geometric(rhs.z) + self.z.anti_geometric(rhs.x) + self.z.anti_geometric(rhs.y) + self.z.anti_geometric(rhs.z)
// Omitted: Rotor dot Dir = self.w.dot(rhs.x) + self.w.dot(rhs.y) + self.w.dot(rhs.z) + self.x.dot(rhs.x) + self.y.dot(rhs.y) + self.z.dot(rhs.z)

// Rotor.wedge(Dir) -> Plane
impl Wedge<Dir> for Rotor {
    type Output = Plane;
    fn wedge(self, rhs: Dir) -> Self::Output {
        Plane {
            nx: self.y.wedge(rhs.z) - self.z.wedge(rhs.y),
            ny: self.x.wedge(rhs.z) - self.z.wedge(rhs.x),
            nz: self.x.wedge(rhs.y) - self.y.wedge(rhs.x),
            d: Default::default(),
        }
    }
}

// Rotor.anti_wedge(Dir) -> Dir
impl AntiWedge<Dir> for Rotor {
    type Output = Dir;
    fn anti_wedge(self, rhs: Dir) -> Self::Output {
        Dir {
            x: self.w.anti_wedge(rhs.x),
            y: self.w.anti_wedge(rhs.y),
            z: self.w.anti_wedge(rhs.z),
        }
    }
}

// ---------------------------------------------------------------------
// Rotor OP Point:

// Omitted: Rotor geometric Point = self.w.geometric(rhs.x) + self.w.geometric(rhs.y) + self.w.geometric(rhs.z) + self.x.geometric(rhs.x) + self.x.geometric(rhs.y) + self.x.geometric(rhs.z) + self.y.geometric(rhs.x) + self.y.geometric(rhs.y) + self.y.geometric(rhs.z) + self.z.geometric(rhs.x) + self.z.geometric(rhs.y) + self.z.geometric(rhs.z)
// Omitted: Rotor anti_geometric Point = self.w.anti_geometric(rhs.w) + self.w.anti_geometric(rhs.x) + self.w.anti_geometric(rhs.y) + self.w.anti_geometric(rhs.z) + self.x.anti_geometric(rhs.w) + self.x.anti_geometric(rhs.x) + self.x.anti_geometric(rhs.y) + self.x.anti_geometric(rhs.z) + self.y.anti_geometric(rhs.w) + self.y.anti_geometric(rhs.x) + self.y.anti_geometric(rhs.y) + self.y.anti_geometric(rhs.z) + self.z.anti_geometric(rhs.w) + self.z.anti_geometric(rhs.x) + self.z.anti_geometric(rhs.y) + self.z.anti_geometric(rhs.z)
// Omitted: Rotor dot Point = self.w.dot(rhs.x) + self.w.dot(rhs.y) + self.w.dot(rhs.z) + self.x.dot(rhs.x) + self.y.dot(rhs.y) + self.z.dot(rhs.z)

// Rotor.wedge(Point) -> Plane
impl Wedge<Point> for Rotor {
    type Output = Plane;
    fn wedge(self, rhs: Point) -> Self::Output {
        Plane {
            nx: self.y.wedge(rhs.z) - self.z.wedge(rhs.y),
            ny: self.x.wedge(rhs.z) - self.z.wedge(rhs.x),
            nz: self.x.wedge(rhs.y) - self.y.wedge(rhs.x),
            d: Default::default(),
        }
    }
}

// Rotor.anti_wedge(Point) -> Point
impl AntiWedge<Point> for Rotor {
    type Output = Point;
    fn anti_wedge(self, rhs: Point) -> Self::Output {
        Point {
            x: self.w.anti_wedge(rhs.x),
            y: self.w.anti_wedge(rhs.y),
            z: self.w.anti_wedge(rhs.z),
            w: self.w.anti_wedge(rhs.w),
        }
    }
}

// ---------------------------------------------------------------------
// Rotor OP Line:

// Rotor.geometric(Line) -> Rotor
impl Geometric<Line> for Rotor {
    type Output = Rotor;
    fn geometric(self, rhs: Line) -> Self::Output {
        Rotor {
            x: -self.w.geometric(rhs.mx) + self.y.geometric(rhs.mz) - self.z.geometric(rhs.my),
            y: -self.w.geometric(rhs.my) - self.x.geometric(rhs.mz) + self.z.geometric(rhs.mx),
            z: -self.w.geometric(rhs.mz) + self.x.geometric(rhs.my) - self.y.geometric(rhs.mx),
            w: -self.x.geometric(rhs.mx) - self.y.geometric(rhs.my) - self.z.geometric(rhs.mz),
        }
    }
}

// Omitted: Rotor anti_geometric Line = self.w.anti_geometric(rhs.mx) + self.w.anti_geometric(rhs.my) + self.w.anti_geometric(rhs.mz) + self.w.anti_geometric(rhs.vx) + self.w.anti_geometric(rhs.vy) + self.w.anti_geometric(rhs.vz) + self.x.anti_geometric(rhs.mx) + self.x.anti_geometric(rhs.my) + self.x.anti_geometric(rhs.mz) + self.x.anti_geometric(rhs.vx) + self.x.anti_geometric(rhs.vy) + self.x.anti_geometric(rhs.vz) + self.y.anti_geometric(rhs.mx) + self.y.anti_geometric(rhs.my) + self.y.anti_geometric(rhs.mz) + self.y.anti_geometric(rhs.vx) + self.y.anti_geometric(rhs.vy) + self.y.anti_geometric(rhs.vz) + self.z.anti_geometric(rhs.mx) + self.z.anti_geometric(rhs.my) + self.z.anti_geometric(rhs.mz) + self.z.anti_geometric(rhs.vx) + self.z.anti_geometric(rhs.vy) + self.z.anti_geometric(rhs.vz)

// Rotor.dot(Line) -> Line
impl Dot<Line> for Rotor {
    type Output = Line;
    fn dot(self, rhs: Line) -> Self::Output {
        Line {
            vx: -self.w.dot(rhs.mx),
            vy: -self.w.dot(rhs.my),
            vz: -self.w.dot(rhs.mz),
            mx: Default::default(),
            my: Default::default(),
            mz: Default::default(),
        }
    }
}

// Rotor.wedge(Line) -> XYZW
impl Wedge<Line> for Rotor {
    type Output = XYZW;
    fn wedge(self, rhs: Line) -> Self::Output {
        self.x.wedge(rhs.mx) + self.y.wedge(rhs.my) + self.z.wedge(rhs.mz)
    }
}

// Omitted: Rotor anti_wedge Line = self.w.anti_wedge(rhs.mx) + self.w.anti_wedge(rhs.my) + self.w.anti_wedge(rhs.mz) + self.w.anti_wedge(rhs.vx) + self.w.anti_wedge(rhs.vy) + self.w.anti_wedge(rhs.vz) + self.x.anti_wedge(rhs.mx) + self.y.anti_wedge(rhs.my) + self.z.anti_wedge(rhs.mz)

// ---------------------------------------------------------------------
// Rotor OP Plane:

// Omitted: Rotor geometric Plane = self.w.geometric(rhs.d) + self.x.geometric(rhs.d) + self.y.geometric(rhs.d) + self.z.geometric(rhs.d)
// Omitted: Rotor anti_geometric Plane = self.w.anti_geometric(rhs.d) + self.w.anti_geometric(rhs.nx) + self.w.anti_geometric(rhs.ny) + self.w.anti_geometric(rhs.nz) + self.x.anti_geometric(rhs.d) + self.x.anti_geometric(rhs.nx) + self.x.anti_geometric(rhs.ny) + self.x.anti_geometric(rhs.nz) + self.y.anti_geometric(rhs.d) + self.y.anti_geometric(rhs.nx) + self.y.anti_geometric(rhs.ny) + self.y.anti_geometric(rhs.nz) + self.z.anti_geometric(rhs.d) + self.z.anti_geometric(rhs.nx) + self.z.anti_geometric(rhs.ny) + self.z.anti_geometric(rhs.nz)

// Rotor.dot(Plane) -> W
impl Dot<Plane> for Rotor {
    type Output = W;
    fn dot(self, rhs: Plane) -> Self::Output {
        self.w.dot(rhs.d)
    }
}

// Omitted: Rotor wedge Plane = 0
// Omitted: Rotor anti_wedge Plane = self.w.anti_wedge(rhs.d) + self.w.anti_wedge(rhs.nx) + self.w.anti_wedge(rhs.ny) + self.w.anti_wedge(rhs.nz) + self.x.anti_wedge(rhs.d) + self.x.anti_wedge(rhs.nx) + self.y.anti_wedge(rhs.d) + self.y.anti_wedge(rhs.ny) + self.z.anti_wedge(rhs.d) + self.z.anti_wedge(rhs.nz)

// ---------------------------------------------------------------------
// Rotor OP Translator:

// Rotor.geometric(Translator) -> Rotor
impl Geometric<Translator> for Rotor {
    type Output = Rotor;
    fn geometric(self, rhs: Translator) -> Self::Output {
        Rotor {
            x: -self.w.geometric(rhs.x) + self.y.geometric(rhs.z) - self.z.geometric(rhs.y),
            y: -self.w.geometric(rhs.y) - self.x.geometric(rhs.z) + self.z.geometric(rhs.x),
            z: -self.w.geometric(rhs.z) + self.x.geometric(rhs.y) - self.y.geometric(rhs.x),
            w: -self.x.geometric(rhs.x) - self.y.geometric(rhs.y) - self.z.geometric(rhs.z),
        }
    }
}

// Omitted: Rotor anti_geometric Translator = self.w.anti_geometric(rhs.x) + self.w.anti_geometric(rhs.xyzw) + self.w.anti_geometric(rhs.y) + self.w.anti_geometric(rhs.z) + self.x.anti_geometric(rhs.x) + self.x.anti_geometric(rhs.xyzw) + self.x.anti_geometric(rhs.y) + self.x.anti_geometric(rhs.z) + self.y.anti_geometric(rhs.x) + self.y.anti_geometric(rhs.xyzw) + self.y.anti_geometric(rhs.y) + self.y.anti_geometric(rhs.z) + self.z.anti_geometric(rhs.x) + self.z.anti_geometric(rhs.xyzw) + self.z.anti_geometric(rhs.y) + self.z.anti_geometric(rhs.z)

// Rotor.dot(Translator) -> Line
impl Dot<Translator> for Rotor {
    type Output = Line;
    fn dot(self, rhs: Translator) -> Self::Output {
        Line {
            vx: -self.w.dot(rhs.x),
            vy: -self.w.dot(rhs.y),
            vz: -self.w.dot(rhs.z),
            mx: Default::default(),
            my: Default::default(),
            mz: Default::default(),
        }
    }
}

// Rotor.wedge(Translator) -> XYZW
impl Wedge<Translator> for Rotor {
    type Output = XYZW;
    fn wedge(self, rhs: Translator) -> Self::Output {
        self.x.wedge(rhs.x) + self.y.wedge(rhs.y) + self.z.wedge(rhs.z)
    }
}

// Omitted: Rotor anti_wedge Translator = self.w.anti_wedge(rhs.x) + self.w.anti_wedge(rhs.xyzw) + self.w.anti_wedge(rhs.y) + self.w.anti_wedge(rhs.z) + self.x.anti_wedge(rhs.x) + self.x.anti_wedge(rhs.xyzw) + self.y.anti_wedge(rhs.xyzw) + self.y.anti_wedge(rhs.y) + self.z.anti_wedge(rhs.xyzw) + self.z.anti_wedge(rhs.z)

// ---------------------------------------------------------------------
// Rotor OP Rotor:

// Omitted: Rotor geometric Rotor = 0

// Rotor.anti_geometric(Rotor) -> Rotor
impl AntiGeometric<Rotor> for Rotor {
    type Output = Rotor;
    fn anti_geometric(self, rhs: Rotor) -> Self::Output {
        Rotor {
            x: -self.w.anti_geometric(rhs.x)
                - self.x.anti_geometric(rhs.w)
                - self.y.anti_geometric(rhs.z)
                + self.z.anti_geometric(rhs.y),
            y: -self.w.anti_geometric(rhs.y) + self.x.anti_geometric(rhs.z)
                - self.y.anti_geometric(rhs.w)
                - self.z.anti_geometric(rhs.x),
            z: -self.w.anti_geometric(rhs.z) - self.x.anti_geometric(rhs.y)
                + self.y.anti_geometric(rhs.x)
                - self.z.anti_geometric(rhs.w),
            w: self.w.anti_geometric(rhs.w)
                - self.x.anti_geometric(rhs.x)
                - self.y.anti_geometric(rhs.y)
                - self.z.anti_geometric(rhs.z),
        }
    }
}

// Omitted: Rotor dot Rotor = 0
// Omitted: Rotor wedge Rotor = 0

// Rotor.anti_wedge(Rotor) -> Rotor
impl AntiWedge<Rotor> for Rotor {
    type Output = Rotor;
    fn anti_wedge(self, rhs: Rotor) -> Self::Output {
        Rotor {
            x: -self.w.anti_wedge(rhs.x) - self.x.anti_wedge(rhs.w),
            y: -self.w.anti_wedge(rhs.y) - self.y.anti_wedge(rhs.w),
            z: -self.w.anti_wedge(rhs.z) - self.z.anti_wedge(rhs.w),
            w: self.w.anti_wedge(rhs.w),
        }
    }
}

// ---------------------------------------------------------------------
// Rotor OP Motor:

// Rotor.geometric(Motor) -> Rotor
impl Geometric<Motor> for Rotor {
    type Output = Rotor;
    fn geometric(self, rhs: Motor) -> Self::Output {
        Rotor {
            x: -self.x.geometric(rhs.uw),
            y: -self.y.geometric(rhs.uw),
            z: -self.z.geometric(rhs.uw),
            w: self.w.geometric(rhs.uw),
        }
    }
}

// Omitted: Rotor anti_geometric Motor = self.w.anti_geometric(rhs.rw) + self.w.anti_geometric(rhs.rx) + self.w.anti_geometric(rhs.ry) + self.w.anti_geometric(rhs.rz) + self.w.anti_geometric(rhs.uw) + self.w.anti_geometric(rhs.ux) + self.w.anti_geometric(rhs.uy) + self.w.anti_geometric(rhs.uz) + self.x.anti_geometric(rhs.rw) + self.x.anti_geometric(rhs.rx) + self.x.anti_geometric(rhs.ry) + self.x.anti_geometric(rhs.rz) + self.x.anti_geometric(rhs.uw) + self.x.anti_geometric(rhs.ux) + self.x.anti_geometric(rhs.uy) + self.x.anti_geometric(rhs.uz) + self.y.anti_geometric(rhs.rw) + self.y.anti_geometric(rhs.rx) + self.y.anti_geometric(rhs.ry) + self.y.anti_geometric(rhs.rz) + self.y.anti_geometric(rhs.uw) + self.y.anti_geometric(rhs.ux) + self.y.anti_geometric(rhs.uy) + self.y.anti_geometric(rhs.uz) + self.z.anti_geometric(rhs.rw) + self.z.anti_geometric(rhs.rx) + self.z.anti_geometric(rhs.ry) + self.z.anti_geometric(rhs.rz) + self.z.anti_geometric(rhs.uw) + self.z.anti_geometric(rhs.ux) + self.z.anti_geometric(rhs.uy) + self.z.anti_geometric(rhs.uz)

// Rotor.dot(Motor) -> Rotor
impl Dot<Motor> for Rotor {
    type Output = Rotor;
    fn dot(self, rhs: Motor) -> Self::Output {
        Rotor {
            x: -self.x.dot(rhs.uw),
            y: -self.y.dot(rhs.uw),
            z: -self.z.dot(rhs.uw),
            w: self.w.dot(rhs.uw),
        }
    }
}

// Rotor.wedge(Motor) -> Rotor
impl Wedge<Motor> for Rotor {
    type Output = Rotor;
    fn wedge(self, rhs: Motor) -> Self::Output {
        Rotor {
            x: -self.x.wedge(rhs.uw),
            y: -self.y.wedge(rhs.uw),
            z: -self.z.wedge(rhs.uw),
            w: self.w.wedge(rhs.uw),
        }
    }
}

// Omitted: Rotor anti_wedge Motor = self.w.anti_wedge(rhs.rw) + self.w.anti_wedge(rhs.rx) + self.w.anti_wedge(rhs.ry) + self.w.anti_wedge(rhs.rz) + self.w.anti_wedge(rhs.uw) + self.w.anti_wedge(rhs.ux) + self.w.anti_wedge(rhs.uy) + self.w.anti_wedge(rhs.uz) + self.x.anti_wedge(rhs.rw) + self.x.anti_wedge(rhs.ux) + self.y.anti_wedge(rhs.rw) + self.y.anti_wedge(rhs.uy) + self.z.anti_wedge(rhs.rw) + self.z.anti_wedge(rhs.uz)

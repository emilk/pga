//! # Vec3
//!
//! ## Operations
//! ```text
//! Vec3.dot(Vec3) -> S
//! Vec3.wedge(Vec3) -> Line3
//! Vec3.anti_geometric(Vec4) -> Line3
//! Vec4.anti_geometric(Vec3) -> Line3
//! Vec3.dot(Vec4) -> S
//! Vec4.dot(Vec3) -> S
//! Vec3.wedge(Vec4) -> Line3
//! Vec4.wedge(Vec3) -> Line3
//! Vec3.dot(Line3) -> Vec4
//! Line3.dot(Vec3) -> Vec4
//! Vec3.wedge(Line3) -> Plane
//! Line3.wedge(Vec3) -> Plane
//! Vec3.dot(Plane) -> Line3
//! Plane.dot(Vec3) -> Line3
//! Vec3.wedge(Plane) -> XYZW
//! Plane.wedge(Vec3) -> XYZW
//! Vec3.anti_wedge(Plane) -> S
//! Plane.anti_wedge(Vec3) -> S
//! Vec3.anti_geometric(Translator3) -> Vec3
//! Translator3.anti_geometric(Vec3) -> Vec3
//! Vec3.wedge(Translator3) -> ZYX
//! Translator3.wedge(Vec3) -> ZYX
//! Vec3.anti_wedge(Translator3) -> Vec3
//! Translator3.anti_wedge(Vec3) -> Vec3
//! Vec3.wedge(Rotor3) -> Plane
//! Rotor3.wedge(Vec3) -> Plane
//! Vec3.anti_wedge(Rotor3) -> Vec3
//! Rotor3.anti_wedge(Vec3) -> Vec3
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
pub struct Vec3 {
    pub x: X,
    pub y: Y,
    pub z: Z,
}

// ---------------------------------------------------------------------

impl RCompl for Vec3 {
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

impl LCompl for Vec3 {
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

impl Reverse for Vec3 {
    fn rev(self) -> Self {
        Vec3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl AntiReverse for Vec3 {
    fn arev(self) -> Self {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

// ---------------------------------------------------------------------
// Vec3 OP Vec3:

// Omitted: Vec3 geometric Vec3 = self.x.geometric(rhs.x) + self.x.geometric(rhs.y) + self.x.geometric(rhs.z) + self.y.geometric(rhs.x) + self.y.geometric(rhs.y) + self.y.geometric(rhs.z) + self.z.geometric(rhs.x) + self.z.geometric(rhs.y) + self.z.geometric(rhs.z)
// Omitted: Vec3 anti_geometric Vec3 = 0

// Vec3.dot(Vec3) -> S
impl Dot<Vec3> for Vec3 {
    type Output = S;
    fn dot(self, rhs: Vec3) -> Self::Output {
        self.x.dot(rhs.x) + self.y.dot(rhs.y) + self.z.dot(rhs.z)
    }
}

// Vec3.wedge(Vec3) -> Line3
impl Wedge<Vec3> for Vec3 {
    type Output = Line3;
    fn wedge(self, rhs: Vec3) -> Self::Output {
        Line3 {
            vx: Default::default(),
            vy: Default::default(),
            vz: Default::default(),
            mx: self.y.wedge(rhs.z) - self.z.wedge(rhs.y),
            my: -self.x.wedge(rhs.z) + self.z.wedge(rhs.x),
            mz: self.x.wedge(rhs.y) - self.y.wedge(rhs.x),
        }
    }
}

// Omitted: Vec3 anti_wedge Vec3 = 0

// ---------------------------------------------------------------------
// Vec3 OP Vec4:

// Omitted: Vec3 geometric Vec4 = self.x.geometric(rhs.w) + self.x.geometric(rhs.x) + self.x.geometric(rhs.y) + self.x.geometric(rhs.z) + self.y.geometric(rhs.w) + self.y.geometric(rhs.x) + self.y.geometric(rhs.y) + self.y.geometric(rhs.z) + self.z.geometric(rhs.w) + self.z.geometric(rhs.x) + self.z.geometric(rhs.y) + self.z.geometric(rhs.z)

// Vec3.anti_geometric(Vec4) -> Line3
impl AntiGeometric<Vec4> for Vec3 {
    type Output = Line3;
    fn anti_geometric(self, rhs: Vec4) -> Self::Output {
        Line3 {
            vx: Default::default(),
            vy: Default::default(),
            vz: Default::default(),
            mx: -self.x.anti_geometric(rhs.w),
            my: -self.y.anti_geometric(rhs.w),
            mz: -self.z.anti_geometric(rhs.w),
        }
    }
}

// Vec3.dot(Vec4) -> S
impl Dot<Vec4> for Vec3 {
    type Output = S;
    fn dot(self, rhs: Vec4) -> Self::Output {
        self.x.dot(rhs.x) + self.y.dot(rhs.y) + self.z.dot(rhs.z)
    }
}

// Vec3.wedge(Vec4) -> Line3
impl Wedge<Vec4> for Vec3 {
    type Output = Line3;
    fn wedge(self, rhs: Vec4) -> Self::Output {
        Line3 {
            vx: -self.x.wedge(rhs.w),
            vy: -self.y.wedge(rhs.w),
            vz: -self.z.wedge(rhs.w),
            mx: self.y.wedge(rhs.z) - self.z.wedge(rhs.y),
            my: -self.x.wedge(rhs.z) + self.z.wedge(rhs.x),
            mz: self.x.wedge(rhs.y) - self.y.wedge(rhs.x),
        }
    }
}

// Omitted: Vec3 anti_wedge Vec4 = 0

// ---------------------------------------------------------------------
// Vec3 OP Line3:

// Omitted: Vec3 geometric Line3 = self.x.geometric(rhs.mx) + self.x.geometric(rhs.my) + self.x.geometric(rhs.mz) + self.x.geometric(rhs.vx) + self.x.geometric(rhs.vy) + self.x.geometric(rhs.vz) + self.y.geometric(rhs.mx) + self.y.geometric(rhs.my) + self.y.geometric(rhs.mz) + self.y.geometric(rhs.vx) + self.y.geometric(rhs.vy) + self.y.geometric(rhs.vz) + self.z.geometric(rhs.mx) + self.z.geometric(rhs.my) + self.z.geometric(rhs.mz) + self.z.geometric(rhs.vx) + self.z.geometric(rhs.vy) + self.z.geometric(rhs.vz)
// Omitted: Vec3 anti_geometric Line3 = self.x.anti_geometric(rhs.vx) + self.x.anti_geometric(rhs.vy) + self.x.anti_geometric(rhs.vz) + self.y.anti_geometric(rhs.vx) + self.y.anti_geometric(rhs.vy) + self.y.anti_geometric(rhs.vz) + self.z.anti_geometric(rhs.vx) + self.z.anti_geometric(rhs.vy) + self.z.anti_geometric(rhs.vz)

// Vec3.dot(Line3) -> Vec4
impl Dot<Line3> for Vec3 {
    type Output = Vec4;
    fn dot(self, rhs: Line3) -> Self::Output {
        Vec4 {
            x: -self.y.dot(rhs.mz) + self.z.dot(rhs.my),
            y: self.x.dot(rhs.mz) - self.z.dot(rhs.mx),
            z: -self.x.dot(rhs.my) + self.y.dot(rhs.mx),
            w: -self.x.dot(rhs.vx) - self.y.dot(rhs.vy) - self.z.dot(rhs.vz),
        }
    }
}

// Vec3.wedge(Line3) -> Plane
impl Wedge<Line3> for Vec3 {
    type Output = Plane;
    fn wedge(self, rhs: Line3) -> Self::Output {
        Plane {
            nx: -self.y.wedge(rhs.vz) + self.z.wedge(rhs.vy),
            ny: self.x.wedge(rhs.vz) - self.z.wedge(rhs.vx),
            nz: -self.x.wedge(rhs.vy) + self.y.wedge(rhs.vx),
            d: -self.x.wedge(rhs.mx) - self.y.wedge(rhs.my) - self.z.wedge(rhs.mz),
        }
    }
}

// Omitted: Vec3 anti_wedge Line3 = 0

// ---------------------------------------------------------------------
// Vec3 OP Plane:

// Omitted: Vec3 geometric Plane = self.x.geometric(rhs.d) + self.x.geometric(rhs.nx) + self.x.geometric(rhs.ny) + self.x.geometric(rhs.nz) + self.y.geometric(rhs.d) + self.y.geometric(rhs.nx) + self.y.geometric(rhs.ny) + self.y.geometric(rhs.nz) + self.z.geometric(rhs.d) + self.z.geometric(rhs.nx) + self.z.geometric(rhs.ny) + self.z.geometric(rhs.nz)
// Omitted: Vec3 anti_geometric Plane = self.x.anti_geometric(rhs.nx) + self.x.anti_geometric(rhs.ny) + self.x.anti_geometric(rhs.nz) + self.y.anti_geometric(rhs.nx) + self.y.anti_geometric(rhs.ny) + self.y.anti_geometric(rhs.nz) + self.z.anti_geometric(rhs.nx) + self.z.anti_geometric(rhs.ny) + self.z.anti_geometric(rhs.nz)

// Vec3.dot(Plane) -> Line3
impl Dot<Plane> for Vec3 {
    type Output = Line3;
    fn dot(self, rhs: Plane) -> Self::Output {
        Line3 {
            vx: self.y.dot(rhs.nz) - self.z.dot(rhs.ny),
            vy: -self.x.dot(rhs.nz) + self.z.dot(rhs.nx),
            vz: self.x.dot(rhs.ny) - self.y.dot(rhs.nx),
            mx: -self.x.dot(rhs.d),
            my: -self.y.dot(rhs.d),
            mz: -self.z.dot(rhs.d),
        }
    }
}

// Vec3.wedge(Plane) -> XYZW
impl Wedge<Plane> for Vec3 {
    type Output = XYZW;
    fn wedge(self, rhs: Plane) -> Self::Output {
        self.x.wedge(rhs.nx) + self.y.wedge(rhs.ny) + self.z.wedge(rhs.nz)
    }
}

// Vec3.anti_wedge(Plane) -> S
impl AntiWedge<Plane> for Vec3 {
    type Output = S;
    fn anti_wedge(self, rhs: Plane) -> Self::Output {
        self.x.anti_wedge(rhs.nx) + self.y.anti_wedge(rhs.ny) + self.z.anti_wedge(rhs.nz)
    }
}

// ---------------------------------------------------------------------
// Vec3 OP Translator3:

// Omitted: Vec3 geometric Translator3 = self.x.geometric(rhs.x) + self.x.geometric(rhs.xyzw) + self.x.geometric(rhs.y) + self.x.geometric(rhs.z) + self.y.geometric(rhs.x) + self.y.geometric(rhs.xyzw) + self.y.geometric(rhs.y) + self.y.geometric(rhs.z) + self.z.geometric(rhs.x) + self.z.geometric(rhs.xyzw) + self.z.geometric(rhs.y) + self.z.geometric(rhs.z)

// Vec3.anti_geometric(Translator3) -> Vec3
impl AntiGeometric<Translator3> for Vec3 {
    type Output = Vec3;
    fn anti_geometric(self, rhs: Translator3) -> Self::Output {
        Vec3 {
            x: self.x.anti_geometric(rhs.xyzw),
            y: self.y.anti_geometric(rhs.xyzw),
            z: self.z.anti_geometric(rhs.xyzw),
        }
    }
}

// Omitted: Vec3 dot Translator3 = self.x.dot(rhs.xyzw) + self.x.dot(rhs.y) + self.x.dot(rhs.z) + self.y.dot(rhs.x) + self.y.dot(rhs.xyzw) + self.y.dot(rhs.z) + self.z.dot(rhs.x) + self.z.dot(rhs.xyzw) + self.z.dot(rhs.y)

// Vec3.wedge(Translator3) -> ZYX
impl Wedge<Translator3> for Vec3 {
    type Output = ZYX;
    fn wedge(self, rhs: Translator3) -> Self::Output {
        self.x.wedge(rhs.x) + self.y.wedge(rhs.y) + self.z.wedge(rhs.z)
    }
}

// Vec3.anti_wedge(Translator3) -> Vec3
impl AntiWedge<Translator3> for Vec3 {
    type Output = Vec3;
    fn anti_wedge(self, rhs: Translator3) -> Self::Output {
        Vec3 {
            x: self.x.anti_wedge(rhs.xyzw),
            y: self.y.anti_wedge(rhs.xyzw),
            z: self.z.anti_wedge(rhs.xyzw),
        }
    }
}

// ---------------------------------------------------------------------
// Vec3 OP Rotor3:

// Omitted: Vec3 geometric Rotor3 = self.x.geometric(rhs.w) + self.x.geometric(rhs.x) + self.x.geometric(rhs.y) + self.x.geometric(rhs.z) + self.y.geometric(rhs.w) + self.y.geometric(rhs.x) + self.y.geometric(rhs.y) + self.y.geometric(rhs.z) + self.z.geometric(rhs.w) + self.z.geometric(rhs.x) + self.z.geometric(rhs.y) + self.z.geometric(rhs.z)
// Omitted: Vec3 anti_geometric Rotor3 = self.x.anti_geometric(rhs.w) + self.x.anti_geometric(rhs.x) + self.x.anti_geometric(rhs.y) + self.x.anti_geometric(rhs.z) + self.y.anti_geometric(rhs.w) + self.y.anti_geometric(rhs.x) + self.y.anti_geometric(rhs.y) + self.y.anti_geometric(rhs.z) + self.z.anti_geometric(rhs.w) + self.z.anti_geometric(rhs.x) + self.z.anti_geometric(rhs.y) + self.z.anti_geometric(rhs.z)
// Omitted: Vec3 dot Rotor3 = self.x.dot(rhs.w) + self.x.dot(rhs.x) + self.y.dot(rhs.w) + self.y.dot(rhs.y) + self.z.dot(rhs.w) + self.z.dot(rhs.z)

// Vec3.wedge(Rotor3) -> Plane
impl Wedge<Rotor3> for Vec3 {
    type Output = Plane;
    fn wedge(self, rhs: Rotor3) -> Self::Output {
        Plane {
            nx: -self.y.wedge(rhs.z) + self.z.wedge(rhs.y),
            ny: self.x.wedge(rhs.z) - self.z.wedge(rhs.x),
            nz: -self.x.wedge(rhs.y) + self.y.wedge(rhs.x),
            d: Default::default(),
        }
    }
}

// Vec3.anti_wedge(Rotor3) -> Vec3
impl AntiWedge<Rotor3> for Vec3 {
    type Output = Vec3;
    fn anti_wedge(self, rhs: Rotor3) -> Self::Output {
        Vec3 {
            x: self.x.anti_wedge(rhs.w),
            y: self.y.anti_wedge(rhs.w),
            z: self.z.anti_wedge(rhs.w),
        }
    }
}

// ---------------------------------------------------------------------
// Vec3 OP Motor3:

// Omitted: Vec3 geometric Motor3 = self.x.geometric(rhs.rw) + self.x.geometric(rhs.rx) + self.x.geometric(rhs.ry) + self.x.geometric(rhs.rz) + self.x.geometric(rhs.uw) + self.x.geometric(rhs.ux) + self.x.geometric(rhs.uy) + self.x.geometric(rhs.uz) + self.y.geometric(rhs.rw) + self.y.geometric(rhs.rx) + self.y.geometric(rhs.ry) + self.y.geometric(rhs.rz) + self.y.geometric(rhs.uw) + self.y.geometric(rhs.ux) + self.y.geometric(rhs.uy) + self.y.geometric(rhs.uz) + self.z.geometric(rhs.rw) + self.z.geometric(rhs.rx) + self.z.geometric(rhs.ry) + self.z.geometric(rhs.rz) + self.z.geometric(rhs.uw) + self.z.geometric(rhs.ux) + self.z.geometric(rhs.uy) + self.z.geometric(rhs.uz)
// Omitted: Vec3 anti_geometric Motor3 = self.x.anti_geometric(rhs.rw) + self.x.anti_geometric(rhs.rx) + self.x.anti_geometric(rhs.ry) + self.x.anti_geometric(rhs.rz) + self.x.anti_geometric(rhs.ux) + self.x.anti_geometric(rhs.uy) + self.x.anti_geometric(rhs.uz) + self.y.anti_geometric(rhs.rw) + self.y.anti_geometric(rhs.rx) + self.y.anti_geometric(rhs.ry) + self.y.anti_geometric(rhs.rz) + self.y.anti_geometric(rhs.ux) + self.y.anti_geometric(rhs.uy) + self.y.anti_geometric(rhs.uz) + self.z.anti_geometric(rhs.rw) + self.z.anti_geometric(rhs.rx) + self.z.anti_geometric(rhs.ry) + self.z.anti_geometric(rhs.rz) + self.z.anti_geometric(rhs.ux) + self.z.anti_geometric(rhs.uy) + self.z.anti_geometric(rhs.uz)
// Omitted: Vec3 dot Motor3 = self.x.dot(rhs.rw) + self.x.dot(rhs.rx) + self.x.dot(rhs.uw) + self.x.dot(rhs.uy) + self.x.dot(rhs.uz) + self.y.dot(rhs.rw) + self.y.dot(rhs.ry) + self.y.dot(rhs.uw) + self.y.dot(rhs.ux) + self.y.dot(rhs.uz) + self.z.dot(rhs.rw) + self.z.dot(rhs.rz) + self.z.dot(rhs.uw) + self.z.dot(rhs.ux) + self.z.dot(rhs.uy)
// Omitted: Vec3 wedge Motor3 = self.x.wedge(rhs.ry) + self.x.wedge(rhs.rz) + self.x.wedge(rhs.uw) + self.x.wedge(rhs.ux) + self.y.wedge(rhs.rx) + self.y.wedge(rhs.rz) + self.y.wedge(rhs.uw) + self.y.wedge(rhs.uy) + self.z.wedge(rhs.rx) + self.z.wedge(rhs.ry) + self.z.wedge(rhs.uw) + self.z.wedge(rhs.uz)
// Omitted: Vec3 anti_wedge Motor3 = self.x.anti_wedge(rhs.rw) + self.x.anti_wedge(rhs.ux) + self.y.anti_wedge(rhs.rw) + self.y.anti_wedge(rhs.uy) + self.z.anti_wedge(rhs.rw) + self.z.anti_wedge(rhs.uz)

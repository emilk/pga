//! # Translator3
//!
//! ## Operations
//! ```text
//! Translator3.anti_geometric(Translator3) -> Translator3
//! Translator3.dot(Translator3) -> Motor3
//! Translator3.anti_wedge(Translator3) -> Translator3
//! Translator3.anti_geometric(Vec3) -> Vec3
//! Vec3.anti_geometric(Translator3) -> Vec3
//! Translator3.wedge(Vec3) -> ZYX
//! Vec3.wedge(Translator3) -> ZYX
//! Translator3.anti_wedge(Vec3) -> Vec3
//! Vec3.anti_wedge(Translator3) -> Vec3
//! Translator3.anti_geometric(Vec4) -> Vec4
//! Vec4.anti_geometric(Translator3) -> Vec4
//! Translator3.wedge(Vec4) -> Plane
//! Vec4.wedge(Translator3) -> Plane
//! Translator3.anti_wedge(Vec4) -> Vec4
//! Vec4.anti_wedge(Translator3) -> Vec4
//! Translator3.dot(Line3) -> Motor3
//! Line3.dot(Translator3) -> Motor3
//! Translator3.wedge(Line3) -> XYZW
//! Line3.wedge(Translator3) -> XYZW
//! Translator3.dot(Plane) -> Vec4
//! Plane.dot(Translator3) -> Vec4
//! Translator3.geometric(Rotor3) -> Rotor3
//! Rotor3.geometric(Translator3) -> Rotor3
//! Translator3.dot(Rotor3) -> Line3
//! Rotor3.dot(Translator3) -> Line3
//! Translator3.wedge(Rotor3) -> XYZW
//! Rotor3.wedge(Translator3) -> XYZW
//! Translator3.wedge(Motor3) -> Translator3
//! Motor3.wedge(Translator3) -> Translator3
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
pub struct Translator3 {
    pub x: YZ,
    pub y: ZX,
    pub z: XY,
    pub xyzw: XYZW,
}

// ---------------------------------------------------------------------

impl RCompl for Translator3 {
    type Output = Motor3;
    fn rcompl(self) -> Self::Output {
        Motor3 {
            rx: -self.x.rcompl(),
            ry: -self.y.rcompl(),
            rz: -self.z.rcompl(),
            rw: Default::default(),
            ux: Default::default(),
            uy: Default::default(),
            uz: Default::default(),
            uw: self.xyzw.rcompl(),
        }
    }
}

impl LCompl for Translator3 {
    type Output = Motor3;
    fn lcompl(self) -> Self::Output {
        Motor3 {
            rx: -self.x.lcompl(),
            ry: -self.y.lcompl(),
            rz: -self.z.lcompl(),
            rw: Default::default(),
            ux: Default::default(),
            uy: Default::default(),
            uz: Default::default(),
            uw: self.xyzw.lcompl(),
        }
    }
}

impl Reverse for Translator3 {
    fn rev(self) -> Self {
        Translator3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            xyzw: self.xyzw,
        }
    }
}

impl AntiReverse for Translator3 {
    fn arev(self) -> Self {
        Translator3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            xyzw: self.xyzw,
        }
    }
}

// ---------------------------------------------------------------------
// Translator3 OP Vec3:

// Omitted: Translator3 geometric Vec3 = self.x.geometric(rhs.x) + self.x.geometric(rhs.y) + self.x.geometric(rhs.z) + self.xyzw.geometric(rhs.x) + self.xyzw.geometric(rhs.y) + self.xyzw.geometric(rhs.z) + self.y.geometric(rhs.x) + self.y.geometric(rhs.y) + self.y.geometric(rhs.z) + self.z.geometric(rhs.x) + self.z.geometric(rhs.y) + self.z.geometric(rhs.z)

// Translator3.anti_geometric(Vec3) -> Vec3
impl AntiGeometric<Vec3> for Translator3 {
    type Output = Vec3;
    fn anti_geometric(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.xyzw.anti_geometric(rhs.x),
            y: self.xyzw.anti_geometric(rhs.y),
            z: self.xyzw.anti_geometric(rhs.z),
        }
    }
}

// Omitted: Translator3 dot Vec3 = self.x.dot(rhs.y) + self.x.dot(rhs.z) + self.xyzw.dot(rhs.x) + self.xyzw.dot(rhs.y) + self.xyzw.dot(rhs.z) + self.y.dot(rhs.x) + self.y.dot(rhs.z) + self.z.dot(rhs.x) + self.z.dot(rhs.y)

// Translator3.wedge(Vec3) -> ZYX
impl Wedge<Vec3> for Translator3 {
    type Output = ZYX;
    fn wedge(self, rhs: Vec3) -> Self::Output {
        self.x.wedge(rhs.x) + self.y.wedge(rhs.y) + self.z.wedge(rhs.z)
    }
}

// Translator3.anti_wedge(Vec3) -> Vec3
impl AntiWedge<Vec3> for Translator3 {
    type Output = Vec3;
    fn anti_wedge(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.xyzw.anti_wedge(rhs.x),
            y: self.xyzw.anti_wedge(rhs.y),
            z: self.xyzw.anti_wedge(rhs.z),
        }
    }
}

// ---------------------------------------------------------------------
// Translator3 OP Vec4:

// Omitted: Translator3 geometric Vec4 = self.x.geometric(rhs.w) + self.x.geometric(rhs.x) + self.x.geometric(rhs.y) + self.x.geometric(rhs.z) + self.xyzw.geometric(rhs.x) + self.xyzw.geometric(rhs.y) + self.xyzw.geometric(rhs.z) + self.y.geometric(rhs.w) + self.y.geometric(rhs.x) + self.y.geometric(rhs.y) + self.y.geometric(rhs.z) + self.z.geometric(rhs.w) + self.z.geometric(rhs.x) + self.z.geometric(rhs.y) + self.z.geometric(rhs.z)

// Translator3.anti_geometric(Vec4) -> Vec4
impl AntiGeometric<Vec4> for Translator3 {
    type Output = Vec4;
    fn anti_geometric(self, rhs: Vec4) -> Self::Output {
        Vec4 {
            x: self.x.anti_geometric(rhs.w) + self.xyzw.anti_geometric(rhs.x),
            y: self.xyzw.anti_geometric(rhs.y) + self.y.anti_geometric(rhs.w),
            z: self.xyzw.anti_geometric(rhs.z) + self.z.anti_geometric(rhs.w),
            w: self.xyzw.anti_geometric(rhs.w),
        }
    }
}

// Omitted: Translator3 dot Vec4 = self.x.dot(rhs.y) + self.x.dot(rhs.z) + self.xyzw.dot(rhs.x) + self.xyzw.dot(rhs.y) + self.xyzw.dot(rhs.z) + self.y.dot(rhs.x) + self.y.dot(rhs.z) + self.z.dot(rhs.x) + self.z.dot(rhs.y)

// Translator3.wedge(Vec4) -> Plane
impl Wedge<Vec4> for Translator3 {
    type Output = Plane;
    fn wedge(self, rhs: Vec4) -> Self::Output {
        Plane {
            nx: self.x.wedge(rhs.w),
            ny: self.y.wedge(rhs.w),
            nz: self.z.wedge(rhs.w),
            d: -self.x.wedge(rhs.x) - self.y.wedge(rhs.y) - self.z.wedge(rhs.z),
        }
    }
}

// Translator3.anti_wedge(Vec4) -> Vec4
impl AntiWedge<Vec4> for Translator3 {
    type Output = Vec4;
    fn anti_wedge(self, rhs: Vec4) -> Self::Output {
        Vec4 {
            x: self.xyzw.anti_wedge(rhs.x),
            y: self.xyzw.anti_wedge(rhs.y),
            z: self.xyzw.anti_wedge(rhs.z),
            w: self.xyzw.anti_wedge(rhs.w),
        }
    }
}

// ---------------------------------------------------------------------
// Translator3 OP Line3:

// Omitted: Translator3 geometric Line3 = self.x.geometric(rhs.mx) + self.x.geometric(rhs.my) + self.x.geometric(rhs.mz) + self.x.geometric(rhs.vx) + self.x.geometric(rhs.vy) + self.x.geometric(rhs.vz) + self.xyzw.geometric(rhs.mx) + self.xyzw.geometric(rhs.my) + self.xyzw.geometric(rhs.mz) + self.y.geometric(rhs.mx) + self.y.geometric(rhs.my) + self.y.geometric(rhs.mz) + self.y.geometric(rhs.vx) + self.y.geometric(rhs.vy) + self.y.geometric(rhs.vz) + self.z.geometric(rhs.mx) + self.z.geometric(rhs.my) + self.z.geometric(rhs.mz) + self.z.geometric(rhs.vx) + self.z.geometric(rhs.vy) + self.z.geometric(rhs.vz)
// Omitted: Translator3 anti_geometric Line3 = self.x.anti_geometric(rhs.vx) + self.x.anti_geometric(rhs.vy) + self.x.anti_geometric(rhs.vz) + self.xyzw.anti_geometric(rhs.mx) + self.xyzw.anti_geometric(rhs.my) + self.xyzw.anti_geometric(rhs.mz) + self.xyzw.anti_geometric(rhs.vx) + self.xyzw.anti_geometric(rhs.vy) + self.xyzw.anti_geometric(rhs.vz) + self.y.anti_geometric(rhs.vx) + self.y.anti_geometric(rhs.vy) + self.y.anti_geometric(rhs.vz) + self.z.anti_geometric(rhs.vx) + self.z.anti_geometric(rhs.vy) + self.z.anti_geometric(rhs.vz)

// Translator3.dot(Line3) -> Motor3
impl Dot<Line3> for Translator3 {
    type Output = Motor3;
    fn dot(self, rhs: Line3) -> Self::Output {
        Motor3 {
            rx: self.xyzw.dot(rhs.mx),
            ry: self.xyzw.dot(rhs.my),
            rz: self.xyzw.dot(rhs.mz),
            rw: Default::default(),
            ux: Default::default(),
            uy: Default::default(),
            uz: Default::default(),
            uw: -self.x.dot(rhs.mx) - self.y.dot(rhs.my) - self.z.dot(rhs.mz),
        }
    }
}

// Translator3.wedge(Line3) -> XYZW
impl Wedge<Line3> for Translator3 {
    type Output = XYZW;
    fn wedge(self, rhs: Line3) -> Self::Output {
        self.x.wedge(rhs.vx) + self.y.wedge(rhs.vy) + self.z.wedge(rhs.vz)
    }
}

// Omitted: Translator3 anti_wedge Line3 = self.x.anti_wedge(rhs.vx) + self.xyzw.anti_wedge(rhs.mx) + self.xyzw.anti_wedge(rhs.my) + self.xyzw.anti_wedge(rhs.mz) + self.xyzw.anti_wedge(rhs.vx) + self.xyzw.anti_wedge(rhs.vy) + self.xyzw.anti_wedge(rhs.vz) + self.y.anti_wedge(rhs.vy) + self.z.anti_wedge(rhs.vz)

// ---------------------------------------------------------------------
// Translator3 OP Plane:

// Omitted: Translator3 geometric Plane = self.x.geometric(rhs.d) + self.x.geometric(rhs.nx) + self.x.geometric(rhs.ny) + self.x.geometric(rhs.nz) + self.xyzw.geometric(rhs.d) + self.y.geometric(rhs.d) + self.y.geometric(rhs.nx) + self.y.geometric(rhs.ny) + self.y.geometric(rhs.nz) + self.z.geometric(rhs.d) + self.z.geometric(rhs.nx) + self.z.geometric(rhs.ny) + self.z.geometric(rhs.nz)
// Omitted: Translator3 anti_geometric Plane = self.x.anti_geometric(rhs.nx) + self.x.anti_geometric(rhs.ny) + self.x.anti_geometric(rhs.nz) + self.xyzw.anti_geometric(rhs.d) + self.xyzw.anti_geometric(rhs.nx) + self.xyzw.anti_geometric(rhs.ny) + self.xyzw.anti_geometric(rhs.nz) + self.y.anti_geometric(rhs.nx) + self.y.anti_geometric(rhs.ny) + self.y.anti_geometric(rhs.nz) + self.z.anti_geometric(rhs.nx) + self.z.anti_geometric(rhs.ny) + self.z.anti_geometric(rhs.nz)

// Translator3.dot(Plane) -> Vec4
impl Dot<Plane> for Translator3 {
    type Output = Vec4;
    fn dot(self, rhs: Plane) -> Self::Output {
        Vec4 {
            x: self.x.dot(rhs.d),
            y: self.y.dot(rhs.d),
            z: self.z.dot(rhs.d),
            w: -self.x.dot(rhs.nx) - self.xyzw.dot(rhs.d) - self.y.dot(rhs.ny) - self.z.dot(rhs.nz),
        }
    }
}

// Omitted: Translator3 wedge Plane = 0
// Omitted: Translator3 anti_wedge Plane = self.x.anti_wedge(rhs.ny) + self.x.anti_wedge(rhs.nz) + self.xyzw.anti_wedge(rhs.d) + self.xyzw.anti_wedge(rhs.nx) + self.xyzw.anti_wedge(rhs.ny) + self.xyzw.anti_wedge(rhs.nz) + self.y.anti_wedge(rhs.nx) + self.y.anti_wedge(rhs.nz) + self.z.anti_wedge(rhs.nx) + self.z.anti_wedge(rhs.ny)

// ---------------------------------------------------------------------
// Translator3 OP Translator3:

// Omitted: Translator3 geometric Translator3 = self.x.geometric(rhs.x) + self.x.geometric(rhs.xyzw) + self.x.geometric(rhs.y) + self.x.geometric(rhs.z) + self.xyzw.geometric(rhs.x) + self.xyzw.geometric(rhs.y) + self.xyzw.geometric(rhs.z) + self.y.geometric(rhs.x) + self.y.geometric(rhs.xyzw) + self.y.geometric(rhs.y) + self.y.geometric(rhs.z) + self.z.geometric(rhs.x) + self.z.geometric(rhs.xyzw) + self.z.geometric(rhs.y) + self.z.geometric(rhs.z)

// Translator3.anti_geometric(Translator3) -> Translator3
impl AntiGeometric<Translator3> for Translator3 {
    type Output = Translator3;
    fn anti_geometric(self, rhs: Translator3) -> Self::Output {
        Translator3 {
            x: self.x.anti_geometric(rhs.xyzw) + self.xyzw.anti_geometric(rhs.x),
            y: self.xyzw.anti_geometric(rhs.y) + self.y.anti_geometric(rhs.xyzw),
            z: self.xyzw.anti_geometric(rhs.z) + self.z.anti_geometric(rhs.xyzw),
            xyzw: self.xyzw.anti_geometric(rhs.xyzw),
        }
    }
}

// Translator3.dot(Translator3) -> Motor3
impl Dot<Translator3> for Translator3 {
    type Output = Motor3;
    fn dot(self, rhs: Translator3) -> Self::Output {
        Motor3 {
            rx: self.x.dot(rhs.xyzw) + self.xyzw.dot(rhs.x),
            ry: self.xyzw.dot(rhs.y) + self.y.dot(rhs.xyzw),
            rz: self.xyzw.dot(rhs.z) + self.z.dot(rhs.xyzw),
            rw: Default::default(),
            ux: Default::default(),
            uy: Default::default(),
            uz: Default::default(),
            uw: -self.x.dot(rhs.x) - self.y.dot(rhs.y) - self.z.dot(rhs.z),
        }
    }
}

// Omitted: Translator3 wedge Translator3 = 0

// Translator3.anti_wedge(Translator3) -> Translator3
impl AntiWedge<Translator3> for Translator3 {
    type Output = Translator3;
    fn anti_wedge(self, rhs: Translator3) -> Self::Output {
        Translator3 {
            x: self.x.anti_wedge(rhs.xyzw) + self.xyzw.anti_wedge(rhs.x),
            y: self.xyzw.anti_wedge(rhs.y) + self.y.anti_wedge(rhs.xyzw),
            z: self.xyzw.anti_wedge(rhs.z) + self.z.anti_wedge(rhs.xyzw),
            xyzw: self.xyzw.anti_wedge(rhs.xyzw),
        }
    }
}

// ---------------------------------------------------------------------
// Translator3 OP Rotor3:

// Translator3.geometric(Rotor3) -> Rotor3
impl Geometric<Rotor3> for Translator3 {
    type Output = Rotor3;
    fn geometric(self, rhs: Rotor3) -> Self::Output {
        Rotor3 {
            x: self.x.geometric(rhs.w) - self.y.geometric(rhs.z) + self.z.geometric(rhs.y),
            y: self.x.geometric(rhs.z) + self.y.geometric(rhs.w) - self.z.geometric(rhs.x),
            z: -self.x.geometric(rhs.y) + self.y.geometric(rhs.x) + self.z.geometric(rhs.w),
            w: -self.x.geometric(rhs.x) - self.y.geometric(rhs.y) - self.z.geometric(rhs.z),
        }
    }
}

// Omitted: Translator3 anti_geometric Rotor3 = self.x.anti_geometric(rhs.w) + self.x.anti_geometric(rhs.x) + self.x.anti_geometric(rhs.y) + self.x.anti_geometric(rhs.z) + self.xyzw.anti_geometric(rhs.w) + self.xyzw.anti_geometric(rhs.x) + self.xyzw.anti_geometric(rhs.y) + self.xyzw.anti_geometric(rhs.z) + self.y.anti_geometric(rhs.w) + self.y.anti_geometric(rhs.x) + self.y.anti_geometric(rhs.y) + self.y.anti_geometric(rhs.z) + self.z.anti_geometric(rhs.w) + self.z.anti_geometric(rhs.x) + self.z.anti_geometric(rhs.y) + self.z.anti_geometric(rhs.z)

// Translator3.dot(Rotor3) -> Line3
impl Dot<Rotor3> for Translator3 {
    type Output = Line3;
    fn dot(self, rhs: Rotor3) -> Self::Output {
        Line3 {
            vx: self.x.dot(rhs.w),
            vy: self.y.dot(rhs.w),
            vz: self.z.dot(rhs.w),
            mx: Default::default(),
            my: Default::default(),
            mz: Default::default(),
        }
    }
}

// Translator3.wedge(Rotor3) -> XYZW
impl Wedge<Rotor3> for Translator3 {
    type Output = XYZW;
    fn wedge(self, rhs: Rotor3) -> Self::Output {
        self.x.wedge(rhs.x) + self.y.wedge(rhs.y) + self.z.wedge(rhs.z)
    }
}

// Omitted: Translator3 anti_wedge Rotor3 = self.x.anti_wedge(rhs.w) + self.x.anti_wedge(rhs.x) + self.xyzw.anti_wedge(rhs.w) + self.xyzw.anti_wedge(rhs.x) + self.xyzw.anti_wedge(rhs.y) + self.xyzw.anti_wedge(rhs.z) + self.y.anti_wedge(rhs.w) + self.y.anti_wedge(rhs.y) + self.z.anti_wedge(rhs.w) + self.z.anti_wedge(rhs.z)

// ---------------------------------------------------------------------
// Translator3 OP Motor3:

// Omitted: Translator3 geometric Motor3 = self.x.geometric(rhs.rw) + self.x.geometric(rhs.rx) + self.x.geometric(rhs.ry) + self.x.geometric(rhs.rz) + self.x.geometric(rhs.uw) + self.x.geometric(rhs.ux) + self.x.geometric(rhs.uy) + self.x.geometric(rhs.uz) + self.xyzw.geometric(rhs.uw) + self.y.geometric(rhs.rw) + self.y.geometric(rhs.rx) + self.y.geometric(rhs.ry) + self.y.geometric(rhs.rz) + self.y.geometric(rhs.uw) + self.y.geometric(rhs.ux) + self.y.geometric(rhs.uy) + self.y.geometric(rhs.uz) + self.z.geometric(rhs.rw) + self.z.geometric(rhs.rx) + self.z.geometric(rhs.ry) + self.z.geometric(rhs.rz) + self.z.geometric(rhs.uw) + self.z.geometric(rhs.ux) + self.z.geometric(rhs.uy) + self.z.geometric(rhs.uz)
// Omitted: Translator3 anti_geometric Motor3 = self.x.anti_geometric(rhs.rw) + self.x.anti_geometric(rhs.rx) + self.x.anti_geometric(rhs.ry) + self.x.anti_geometric(rhs.rz) + self.x.anti_geometric(rhs.ux) + self.x.anti_geometric(rhs.uy) + self.x.anti_geometric(rhs.uz) + self.xyzw.anti_geometric(rhs.rw) + self.xyzw.anti_geometric(rhs.rx) + self.xyzw.anti_geometric(rhs.ry) + self.xyzw.anti_geometric(rhs.rz) + self.xyzw.anti_geometric(rhs.uw) + self.xyzw.anti_geometric(rhs.ux) + self.xyzw.anti_geometric(rhs.uy) + self.xyzw.anti_geometric(rhs.uz) + self.y.anti_geometric(rhs.rw) + self.y.anti_geometric(rhs.rx) + self.y.anti_geometric(rhs.ry) + self.y.anti_geometric(rhs.rz) + self.y.anti_geometric(rhs.ux) + self.y.anti_geometric(rhs.uy) + self.y.anti_geometric(rhs.uz) + self.z.anti_geometric(rhs.rw) + self.z.anti_geometric(rhs.rx) + self.z.anti_geometric(rhs.ry) + self.z.anti_geometric(rhs.rz) + self.z.anti_geometric(rhs.ux) + self.z.anti_geometric(rhs.uy) + self.z.anti_geometric(rhs.uz)
// Omitted: Translator3 dot Motor3 = self.x.dot(rhs.rw) + self.x.dot(rhs.uw) + self.x.dot(rhs.ux) + self.xyzw.dot(rhs.uw) + self.y.dot(rhs.rw) + self.y.dot(rhs.uw) + self.y.dot(rhs.uy) + self.z.dot(rhs.rw) + self.z.dot(rhs.uw) + self.z.dot(rhs.uz)

// Translator3.wedge(Motor3) -> Translator3
impl Wedge<Motor3> for Translator3 {
    type Output = Translator3;
    fn wedge(self, rhs: Motor3) -> Self::Output {
        Translator3 {
            x: self.x.wedge(rhs.uw),
            y: self.y.wedge(rhs.uw),
            z: self.z.wedge(rhs.uw),
            xyzw: -self.x.wedge(rhs.rx) + self.xyzw.wedge(rhs.uw)
                - self.y.wedge(rhs.ry)
                - self.z.wedge(rhs.rz),
        }
    }
}

// Omitted: Translator3 anti_wedge Motor3 = self.x.anti_wedge(rhs.rw) + self.x.anti_wedge(rhs.rx) + self.x.anti_wedge(rhs.uy) + self.x.anti_wedge(rhs.uz) + self.xyzw.anti_wedge(rhs.rw) + self.xyzw.anti_wedge(rhs.rx) + self.xyzw.anti_wedge(rhs.ry) + self.xyzw.anti_wedge(rhs.rz) + self.xyzw.anti_wedge(rhs.uw) + self.xyzw.anti_wedge(rhs.ux) + self.xyzw.anti_wedge(rhs.uy) + self.xyzw.anti_wedge(rhs.uz) + self.y.anti_wedge(rhs.rw) + self.y.anti_wedge(rhs.ry) + self.y.anti_wedge(rhs.ux) + self.y.anti_wedge(rhs.uz) + self.z.anti_wedge(rhs.rw) + self.z.anti_wedge(rhs.rz) + self.z.anti_wedge(rhs.ux) + self.z.anti_wedge(rhs.uy)

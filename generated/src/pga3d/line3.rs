//! # Line3
//!
//! ## Operations
//! ```text
//! Line3.dot(Line3) -> S
//! Line3.wedge(Line3) -> XYZW
//! Line3.anti_wedge(Line3) -> S
//! Line3.dot(Vec3) -> Vec4
//! Vec3.dot(Line3) -> Vec4
//! Line3.wedge(Vec3) -> Plane
//! Vec3.wedge(Line3) -> Plane
//! Line3.dot(Vec4) -> Vec4
//! Vec4.dot(Line3) -> Vec4
//! Line3.wedge(Vec4) -> Plane
//! Vec4.wedge(Line3) -> Plane
//! Line3.dot(Plane) -> Vec4
//! Plane.dot(Line3) -> Vec4
//! Line3.anti_wedge(Plane) -> Vec4
//! Plane.anti_wedge(Line3) -> Vec4
//! Line3.dot(Translator3) -> Motor3
//! Translator3.dot(Line3) -> Motor3
//! Line3.wedge(Translator3) -> XYZW
//! Translator3.wedge(Line3) -> XYZW
//! Line3.geometric(Rotor3) -> Rotor3
//! Rotor3.geometric(Line3) -> Rotor3
//! Line3.dot(Rotor3) -> Line3
//! Rotor3.dot(Line3) -> Line3
//! Line3.wedge(Rotor3) -> XYZW
//! Rotor3.wedge(Line3) -> XYZW
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
pub struct Line3 {
    pub vx: WX,
    pub vy: WY,
    pub vz: WZ,
    pub mx: YZ,
    pub my: ZX,
    pub mz: XY,
}

// ---------------------------------------------------------------------

impl RCompl for Line3 {
    type Output = Line3;
    fn rcompl(self) -> Self::Output {
        Line3 {
            vx: -self.mx.rcompl(),
            vy: -self.my.rcompl(),
            vz: -self.mz.rcompl(),
            mx: -self.vx.rcompl(),
            my: -self.vy.rcompl(),
            mz: -self.vz.rcompl(),
        }
    }
}

impl LCompl for Line3 {
    type Output = Line3;
    fn lcompl(self) -> Self::Output {
        Line3 {
            vx: -self.mx.lcompl(),
            vy: -self.my.lcompl(),
            vz: -self.mz.lcompl(),
            mx: -self.vx.lcompl(),
            my: -self.vy.lcompl(),
            mz: -self.vz.lcompl(),
        }
    }
}

impl Reverse for Line3 {
    fn rev(self) -> Self {
        Line3 {
            vx: -self.vx,
            vy: -self.vy,
            vz: -self.vz,
            mx: -self.mx,
            my: -self.my,
            mz: -self.mz,
        }
    }
}

impl AntiReverse for Line3 {
    fn arev(self) -> Self {
        Line3 {
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
// Line3 OP Vec3:

// Omitted: Line3 geometric Vec3 = self.mx.geometric(rhs.x) + self.mx.geometric(rhs.y) + self.mx.geometric(rhs.z) + self.my.geometric(rhs.x) + self.my.geometric(rhs.y) + self.my.geometric(rhs.z) + self.mz.geometric(rhs.x) + self.mz.geometric(rhs.y) + self.mz.geometric(rhs.z) + self.vx.geometric(rhs.x) + self.vx.geometric(rhs.y) + self.vx.geometric(rhs.z) + self.vy.geometric(rhs.x) + self.vy.geometric(rhs.y) + self.vy.geometric(rhs.z) + self.vz.geometric(rhs.x) + self.vz.geometric(rhs.y) + self.vz.geometric(rhs.z)
// Omitted: Line3 anti_geometric Vec3 = self.vx.anti_geometric(rhs.x) + self.vx.anti_geometric(rhs.y) + self.vx.anti_geometric(rhs.z) + self.vy.anti_geometric(rhs.x) + self.vy.anti_geometric(rhs.y) + self.vy.anti_geometric(rhs.z) + self.vz.anti_geometric(rhs.x) + self.vz.anti_geometric(rhs.y) + self.vz.anti_geometric(rhs.z)

// Line3.dot(Vec3) -> Vec4
impl Dot<Vec3> for Line3 {
    type Output = Vec4;
    fn dot(self, rhs: Vec3) -> Self::Output {
        Vec4 {
            x: -self.my.dot(rhs.z) + self.mz.dot(rhs.y),
            y: self.mx.dot(rhs.z) - self.mz.dot(rhs.x),
            z: -self.mx.dot(rhs.y) + self.my.dot(rhs.x),
            w: self.vx.dot(rhs.x) + self.vy.dot(rhs.y) + self.vz.dot(rhs.z),
        }
    }
}

// Line3.wedge(Vec3) -> Plane
impl Wedge<Vec3> for Line3 {
    type Output = Plane;
    fn wedge(self, rhs: Vec3) -> Self::Output {
        Plane {
            nx: self.vy.wedge(rhs.z) - self.vz.wedge(rhs.y),
            ny: -self.vx.wedge(rhs.z) + self.vz.wedge(rhs.x),
            nz: self.vx.wedge(rhs.y) - self.vy.wedge(rhs.x),
            d: -self.mx.wedge(rhs.x) - self.my.wedge(rhs.y) - self.mz.wedge(rhs.z),
        }
    }
}

// Omitted: Line3 anti_wedge Vec3 = 0

// ---------------------------------------------------------------------
// Line3 OP Vec4:

// Omitted: Line3 geometric Vec4 = self.mx.geometric(rhs.w) + self.mx.geometric(rhs.x) + self.mx.geometric(rhs.y) + self.mx.geometric(rhs.z) + self.my.geometric(rhs.w) + self.my.geometric(rhs.x) + self.my.geometric(rhs.y) + self.my.geometric(rhs.z) + self.mz.geometric(rhs.w) + self.mz.geometric(rhs.x) + self.mz.geometric(rhs.y) + self.mz.geometric(rhs.z) + self.vx.geometric(rhs.x) + self.vx.geometric(rhs.y) + self.vx.geometric(rhs.z) + self.vy.geometric(rhs.x) + self.vy.geometric(rhs.y) + self.vy.geometric(rhs.z) + self.vz.geometric(rhs.x) + self.vz.geometric(rhs.y) + self.vz.geometric(rhs.z)
// Omitted: Line3 anti_geometric Vec4 = self.mx.anti_geometric(rhs.w) + self.my.anti_geometric(rhs.w) + self.mz.anti_geometric(rhs.w) + self.vx.anti_geometric(rhs.w) + self.vx.anti_geometric(rhs.x) + self.vx.anti_geometric(rhs.y) + self.vx.anti_geometric(rhs.z) + self.vy.anti_geometric(rhs.w) + self.vy.anti_geometric(rhs.x) + self.vy.anti_geometric(rhs.y) + self.vy.anti_geometric(rhs.z) + self.vz.anti_geometric(rhs.w) + self.vz.anti_geometric(rhs.x) + self.vz.anti_geometric(rhs.y) + self.vz.anti_geometric(rhs.z)

// Line3.dot(Vec4) -> Vec4
impl Dot<Vec4> for Line3 {
    type Output = Vec4;
    fn dot(self, rhs: Vec4) -> Self::Output {
        Vec4 {
            x: -self.my.dot(rhs.z) + self.mz.dot(rhs.y),
            y: self.mx.dot(rhs.z) - self.mz.dot(rhs.x),
            z: -self.mx.dot(rhs.y) + self.my.dot(rhs.x),
            w: self.vx.dot(rhs.x) + self.vy.dot(rhs.y) + self.vz.dot(rhs.z),
        }
    }
}

// Line3.wedge(Vec4) -> Plane
impl Wedge<Vec4> for Line3 {
    type Output = Plane;
    fn wedge(self, rhs: Vec4) -> Self::Output {
        Plane {
            nx: self.mx.wedge(rhs.w) + self.vy.wedge(rhs.z) - self.vz.wedge(rhs.y),
            ny: self.my.wedge(rhs.w) - self.vx.wedge(rhs.z) + self.vz.wedge(rhs.x),
            nz: self.mz.wedge(rhs.w) + self.vx.wedge(rhs.y) - self.vy.wedge(rhs.x),
            d: -self.mx.wedge(rhs.x) - self.my.wedge(rhs.y) - self.mz.wedge(rhs.z),
        }
    }
}

// Omitted: Line3 anti_wedge Vec4 = 0

// ---------------------------------------------------------------------
// Line3 OP Line3:

// Omitted: Line3 geometric Line3 = self.mx.geometric(rhs.mx) + self.mx.geometric(rhs.my) + self.mx.geometric(rhs.mz) + self.mx.geometric(rhs.vx) + self.mx.geometric(rhs.vy) + self.mx.geometric(rhs.vz) + self.my.geometric(rhs.mx) + self.my.geometric(rhs.my) + self.my.geometric(rhs.mz) + self.my.geometric(rhs.vx) + self.my.geometric(rhs.vy) + self.my.geometric(rhs.vz) + self.mz.geometric(rhs.mx) + self.mz.geometric(rhs.my) + self.mz.geometric(rhs.mz) + self.mz.geometric(rhs.vx) + self.mz.geometric(rhs.vy) + self.mz.geometric(rhs.vz) + self.vx.geometric(rhs.mx) + self.vx.geometric(rhs.my) + self.vx.geometric(rhs.mz) + self.vy.geometric(rhs.mx) + self.vy.geometric(rhs.my) + self.vy.geometric(rhs.mz) + self.vz.geometric(rhs.mx) + self.vz.geometric(rhs.my) + self.vz.geometric(rhs.mz)
// Omitted: Line3 anti_geometric Line3 = self.mx.anti_geometric(rhs.vx) + self.mx.anti_geometric(rhs.vy) + self.mx.anti_geometric(rhs.vz) + self.my.anti_geometric(rhs.vx) + self.my.anti_geometric(rhs.vy) + self.my.anti_geometric(rhs.vz) + self.mz.anti_geometric(rhs.vx) + self.mz.anti_geometric(rhs.vy) + self.mz.anti_geometric(rhs.vz) + self.vx.anti_geometric(rhs.mx) + self.vx.anti_geometric(rhs.my) + self.vx.anti_geometric(rhs.mz) + self.vx.anti_geometric(rhs.vx) + self.vx.anti_geometric(rhs.vy) + self.vx.anti_geometric(rhs.vz) + self.vy.anti_geometric(rhs.mx) + self.vy.anti_geometric(rhs.my) + self.vy.anti_geometric(rhs.mz) + self.vy.anti_geometric(rhs.vx) + self.vy.anti_geometric(rhs.vy) + self.vy.anti_geometric(rhs.vz) + self.vz.anti_geometric(rhs.mx) + self.vz.anti_geometric(rhs.my) + self.vz.anti_geometric(rhs.mz) + self.vz.anti_geometric(rhs.vx) + self.vz.anti_geometric(rhs.vy) + self.vz.anti_geometric(rhs.vz)

// Line3.dot(Line3) -> S
impl Dot<Line3> for Line3 {
    type Output = S;
    fn dot(self, rhs: Line3) -> Self::Output {
        self.mx.dot(rhs.mx) + self.my.dot(rhs.my) + self.mz.dot(rhs.mz)
    }
}

// Line3.wedge(Line3) -> XYZW
impl Wedge<Line3> for Line3 {
    type Output = XYZW;
    fn wedge(self, rhs: Line3) -> Self::Output {
        self.mx.wedge(rhs.vx)
            + self.my.wedge(rhs.vy)
            + self.mz.wedge(rhs.vz)
            + self.vx.wedge(rhs.mx)
            + self.vy.wedge(rhs.my)
            + self.vz.wedge(rhs.mz)
    }
}

// Line3.anti_wedge(Line3) -> S
impl AntiWedge<Line3> for Line3 {
    type Output = S;
    fn anti_wedge(self, rhs: Line3) -> Self::Output {
        self.mx.anti_wedge(rhs.vx)
            + self.my.anti_wedge(rhs.vy)
            + self.mz.anti_wedge(rhs.vz)
            + self.vx.anti_wedge(rhs.mx)
            + self.vy.anti_wedge(rhs.my)
            + self.vz.anti_wedge(rhs.mz)
    }
}

// ---------------------------------------------------------------------
// Line3 OP Plane:

// Omitted: Line3 geometric Plane = self.mx.geometric(rhs.d) + self.mx.geometric(rhs.nx) + self.mx.geometric(rhs.ny) + self.mx.geometric(rhs.nz) + self.my.geometric(rhs.d) + self.my.geometric(rhs.nx) + self.my.geometric(rhs.ny) + self.my.geometric(rhs.nz) + self.mz.geometric(rhs.d) + self.mz.geometric(rhs.nx) + self.mz.geometric(rhs.ny) + self.mz.geometric(rhs.nz) + self.vx.geometric(rhs.d) + self.vy.geometric(rhs.d) + self.vz.geometric(rhs.d)
// Omitted: Line3 anti_geometric Plane = self.mx.anti_geometric(rhs.nx) + self.mx.anti_geometric(rhs.ny) + self.mx.anti_geometric(rhs.nz) + self.my.anti_geometric(rhs.nx) + self.my.anti_geometric(rhs.ny) + self.my.anti_geometric(rhs.nz) + self.mz.anti_geometric(rhs.nx) + self.mz.anti_geometric(rhs.ny) + self.mz.anti_geometric(rhs.nz) + self.vx.anti_geometric(rhs.d) + self.vx.anti_geometric(rhs.nx) + self.vx.anti_geometric(rhs.ny) + self.vx.anti_geometric(rhs.nz) + self.vy.anti_geometric(rhs.d) + self.vy.anti_geometric(rhs.nx) + self.vy.anti_geometric(rhs.ny) + self.vy.anti_geometric(rhs.nz) + self.vz.anti_geometric(rhs.d) + self.vz.anti_geometric(rhs.nx) + self.vz.anti_geometric(rhs.ny) + self.vz.anti_geometric(rhs.nz)

// Line3.dot(Plane) -> Vec4
impl Dot<Plane> for Line3 {
    type Output = Vec4;
    fn dot(self, rhs: Plane) -> Self::Output {
        Vec4 {
            x: self.mx.dot(rhs.d),
            y: self.my.dot(rhs.d),
            z: self.mz.dot(rhs.d),
            w: -self.mx.dot(rhs.nx) - self.my.dot(rhs.ny) - self.mz.dot(rhs.nz),
        }
    }
}

// Omitted: Line3 wedge Plane = 0

// Line3.anti_wedge(Plane) -> Vec4
impl AntiWedge<Plane> for Line3 {
    type Output = Vec4;
    fn anti_wedge(self, rhs: Plane) -> Self::Output {
        Vec4 {
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
// Line3 OP Translator3:

// Omitted: Line3 geometric Translator3 = self.mx.geometric(rhs.x) + self.mx.geometric(rhs.xyzw) + self.mx.geometric(rhs.y) + self.mx.geometric(rhs.z) + self.my.geometric(rhs.x) + self.my.geometric(rhs.xyzw) + self.my.geometric(rhs.y) + self.my.geometric(rhs.z) + self.mz.geometric(rhs.x) + self.mz.geometric(rhs.xyzw) + self.mz.geometric(rhs.y) + self.mz.geometric(rhs.z) + self.vx.geometric(rhs.x) + self.vx.geometric(rhs.y) + self.vx.geometric(rhs.z) + self.vy.geometric(rhs.x) + self.vy.geometric(rhs.y) + self.vy.geometric(rhs.z) + self.vz.geometric(rhs.x) + self.vz.geometric(rhs.y) + self.vz.geometric(rhs.z)
// Omitted: Line3 anti_geometric Translator3 = self.mx.anti_geometric(rhs.xyzw) + self.my.anti_geometric(rhs.xyzw) + self.mz.anti_geometric(rhs.xyzw) + self.vx.anti_geometric(rhs.x) + self.vx.anti_geometric(rhs.xyzw) + self.vx.anti_geometric(rhs.y) + self.vx.anti_geometric(rhs.z) + self.vy.anti_geometric(rhs.x) + self.vy.anti_geometric(rhs.xyzw) + self.vy.anti_geometric(rhs.y) + self.vy.anti_geometric(rhs.z) + self.vz.anti_geometric(rhs.x) + self.vz.anti_geometric(rhs.xyzw) + self.vz.anti_geometric(rhs.y) + self.vz.anti_geometric(rhs.z)

// Line3.dot(Translator3) -> Motor3
impl Dot<Translator3> for Line3 {
    type Output = Motor3;
    fn dot(self, rhs: Translator3) -> Self::Output {
        Motor3 {
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

// Line3.wedge(Translator3) -> XYZW
impl Wedge<Translator3> for Line3 {
    type Output = XYZW;
    fn wedge(self, rhs: Translator3) -> Self::Output {
        self.vx.wedge(rhs.x) + self.vy.wedge(rhs.y) + self.vz.wedge(rhs.z)
    }
}

// Omitted: Line3 anti_wedge Translator3 = self.mx.anti_wedge(rhs.xyzw) + self.my.anti_wedge(rhs.xyzw) + self.mz.anti_wedge(rhs.xyzw) + self.vx.anti_wedge(rhs.x) + self.vx.anti_wedge(rhs.xyzw) + self.vy.anti_wedge(rhs.xyzw) + self.vy.anti_wedge(rhs.y) + self.vz.anti_wedge(rhs.xyzw) + self.vz.anti_wedge(rhs.z)

// ---------------------------------------------------------------------
// Line3 OP Rotor3:

// Line3.geometric(Rotor3) -> Rotor3
impl Geometric<Rotor3> for Line3 {
    type Output = Rotor3;
    fn geometric(self, rhs: Rotor3) -> Self::Output {
        Rotor3 {
            x: self.mx.geometric(rhs.w) - self.my.geometric(rhs.z) + self.mz.geometric(rhs.y),
            y: self.mx.geometric(rhs.z) + self.my.geometric(rhs.w) - self.mz.geometric(rhs.x),
            z: -self.mx.geometric(rhs.y) + self.my.geometric(rhs.x) + self.mz.geometric(rhs.w),
            w: -self.mx.geometric(rhs.x) - self.my.geometric(rhs.y) - self.mz.geometric(rhs.z),
        }
    }
}

// Omitted: Line3 anti_geometric Rotor3 = self.mx.anti_geometric(rhs.w) + self.mx.anti_geometric(rhs.x) + self.mx.anti_geometric(rhs.y) + self.mx.anti_geometric(rhs.z) + self.my.anti_geometric(rhs.w) + self.my.anti_geometric(rhs.x) + self.my.anti_geometric(rhs.y) + self.my.anti_geometric(rhs.z) + self.mz.anti_geometric(rhs.w) + self.mz.anti_geometric(rhs.x) + self.mz.anti_geometric(rhs.y) + self.mz.anti_geometric(rhs.z) + self.vx.anti_geometric(rhs.w) + self.vx.anti_geometric(rhs.x) + self.vx.anti_geometric(rhs.y) + self.vx.anti_geometric(rhs.z) + self.vy.anti_geometric(rhs.w) + self.vy.anti_geometric(rhs.x) + self.vy.anti_geometric(rhs.y) + self.vy.anti_geometric(rhs.z) + self.vz.anti_geometric(rhs.w) + self.vz.anti_geometric(rhs.x) + self.vz.anti_geometric(rhs.y) + self.vz.anti_geometric(rhs.z)

// Line3.dot(Rotor3) -> Line3
impl Dot<Rotor3> for Line3 {
    type Output = Line3;
    fn dot(self, rhs: Rotor3) -> Self::Output {
        Line3 {
            vx: self.mx.dot(rhs.w),
            vy: self.my.dot(rhs.w),
            vz: self.mz.dot(rhs.w),
            mx: Default::default(),
            my: Default::default(),
            mz: Default::default(),
        }
    }
}

// Line3.wedge(Rotor3) -> XYZW
impl Wedge<Rotor3> for Line3 {
    type Output = XYZW;
    fn wedge(self, rhs: Rotor3) -> Self::Output {
        self.mx.wedge(rhs.x) + self.my.wedge(rhs.y) + self.mz.wedge(rhs.z)
    }
}

// Omitted: Line3 anti_wedge Rotor3 = self.mx.anti_wedge(rhs.w) + self.mx.anti_wedge(rhs.x) + self.my.anti_wedge(rhs.w) + self.my.anti_wedge(rhs.y) + self.mz.anti_wedge(rhs.w) + self.mz.anti_wedge(rhs.z) + self.vx.anti_wedge(rhs.w) + self.vy.anti_wedge(rhs.w) + self.vz.anti_wedge(rhs.w)

// ---------------------------------------------------------------------
// Line3 OP Motor3:

// Omitted: Line3 geometric Motor3 = self.mx.geometric(rhs.rw) + self.mx.geometric(rhs.rx) + self.mx.geometric(rhs.ry) + self.mx.geometric(rhs.rz) + self.mx.geometric(rhs.uw) + self.mx.geometric(rhs.ux) + self.mx.geometric(rhs.uy) + self.mx.geometric(rhs.uz) + self.my.geometric(rhs.rw) + self.my.geometric(rhs.rx) + self.my.geometric(rhs.ry) + self.my.geometric(rhs.rz) + self.my.geometric(rhs.uw) + self.my.geometric(rhs.ux) + self.my.geometric(rhs.uy) + self.my.geometric(rhs.uz) + self.mz.geometric(rhs.rw) + self.mz.geometric(rhs.rx) + self.mz.geometric(rhs.ry) + self.mz.geometric(rhs.rz) + self.mz.geometric(rhs.uw) + self.mz.geometric(rhs.ux) + self.mz.geometric(rhs.uy) + self.mz.geometric(rhs.uz) + self.vx.geometric(rhs.uw) + self.vy.geometric(rhs.uw) + self.vz.geometric(rhs.uw)
// Omitted: Line3 anti_geometric Motor3 = self.mx.anti_geometric(rhs.rw) + self.mx.anti_geometric(rhs.rx) + self.mx.anti_geometric(rhs.ry) + self.mx.anti_geometric(rhs.rz) + self.mx.anti_geometric(rhs.ux) + self.mx.anti_geometric(rhs.uy) + self.mx.anti_geometric(rhs.uz) + self.my.anti_geometric(rhs.rw) + self.my.anti_geometric(rhs.rx) + self.my.anti_geometric(rhs.ry) + self.my.anti_geometric(rhs.rz) + self.my.anti_geometric(rhs.ux) + self.my.anti_geometric(rhs.uy) + self.my.anti_geometric(rhs.uz) + self.mz.anti_geometric(rhs.rw) + self.mz.anti_geometric(rhs.rx) + self.mz.anti_geometric(rhs.ry) + self.mz.anti_geometric(rhs.rz) + self.mz.anti_geometric(rhs.ux) + self.mz.anti_geometric(rhs.uy) + self.mz.anti_geometric(rhs.uz) + self.vx.anti_geometric(rhs.rw) + self.vx.anti_geometric(rhs.rx) + self.vx.anti_geometric(rhs.ry) + self.vx.anti_geometric(rhs.rz) + self.vx.anti_geometric(rhs.uw) + self.vx.anti_geometric(rhs.ux) + self.vx.anti_geometric(rhs.uy) + self.vx.anti_geometric(rhs.uz) + self.vy.anti_geometric(rhs.rw) + self.vy.anti_geometric(rhs.rx) + self.vy.anti_geometric(rhs.ry) + self.vy.anti_geometric(rhs.rz) + self.vy.anti_geometric(rhs.uw) + self.vy.anti_geometric(rhs.ux) + self.vy.anti_geometric(rhs.uy) + self.vy.anti_geometric(rhs.uz) + self.vz.anti_geometric(rhs.rw) + self.vz.anti_geometric(rhs.rx) + self.vz.anti_geometric(rhs.ry) + self.vz.anti_geometric(rhs.rz) + self.vz.anti_geometric(rhs.uw) + self.vz.anti_geometric(rhs.ux) + self.vz.anti_geometric(rhs.uy) + self.vz.anti_geometric(rhs.uz)
// Omitted: Line3 dot Motor3 = self.mx.dot(rhs.rw) + self.mx.dot(rhs.uw) + self.mx.dot(rhs.ux) + self.my.dot(rhs.rw) + self.my.dot(rhs.uw) + self.my.dot(rhs.uy) + self.mz.dot(rhs.rw) + self.mz.dot(rhs.uw) + self.mz.dot(rhs.uz) + self.vx.dot(rhs.uw) + self.vy.dot(rhs.uw) + self.vz.dot(rhs.uw)
// Omitted: Line3 wedge Motor3 = self.mx.wedge(rhs.rx) + self.mx.wedge(rhs.uw) + self.my.wedge(rhs.ry) + self.my.wedge(rhs.uw) + self.mz.wedge(rhs.rz) + self.mz.wedge(rhs.uw) + self.vx.wedge(rhs.uw) + self.vy.wedge(rhs.uw) + self.vz.wedge(rhs.uw)
// Omitted: Line3 anti_wedge Motor3 = self.mx.anti_wedge(rhs.rw) + self.mx.anti_wedge(rhs.rx) + self.mx.anti_wedge(rhs.uy) + self.mx.anti_wedge(rhs.uz) + self.my.anti_wedge(rhs.rw) + self.my.anti_wedge(rhs.ry) + self.my.anti_wedge(rhs.ux) + self.my.anti_wedge(rhs.uz) + self.mz.anti_wedge(rhs.rw) + self.mz.anti_wedge(rhs.rz) + self.mz.anti_wedge(rhs.ux) + self.mz.anti_wedge(rhs.uy) + self.vx.anti_wedge(rhs.rw) + self.vx.anti_wedge(rhs.ux) + self.vy.anti_wedge(rhs.rw) + self.vy.anti_wedge(rhs.uy) + self.vz.anti_wedge(rhs.rw) + self.vz.anti_wedge(rhs.uz)

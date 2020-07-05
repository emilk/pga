//! # Motor3
//!
//! ## Operations
//! ```text
//! Motor3.geometric(Motor3) -> Motor3
//! Motor3.dot(Motor3) -> Motor3
//! Motor3.wedge(Motor3) -> Motor3
//!
//!
//!
//! Motor3.wedge(Plane) -> Plane
//! Plane.wedge(Motor3) -> Plane
//! Motor3.wedge(Translator3) -> Translator3
//! Translator3.wedge(Motor3) -> Translator3
//! Motor3.geometric(Rotor3) -> Rotor3
//! Rotor3.geometric(Motor3) -> Rotor3
//! Motor3.dot(Rotor3) -> Rotor3
//! Rotor3.dot(Motor3) -> Rotor3
//! Motor3.wedge(Rotor3) -> Rotor3
//! Rotor3.wedge(Motor3) -> Rotor3
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
pub struct Motor3 {
    pub rx: WX,
    pub ry: WY,
    pub rz: WZ,
    pub rw: XYZW,
    pub ux: YZW,
    pub uy: ZXW,
    pub uz: XYW,
    pub uw: S,
}

// ---------------------------------------------------------------------
// Omitted: Motor3.rcompl() -> self.rw.rcompl() + self.rx.rcompl() + self.ry.rcompl() + self.rz.rcompl() + self.uw.rcompl() + self.ux.rcompl() + self.uy.rcompl() + self.uz.rcompl()
// Omitted: Motor3.lcompl() -> self.rw.lcompl() + self.rx.lcompl() + self.ry.lcompl() + self.rz.lcompl() + self.uw.lcompl() + self.ux.lcompl() + self.uy.lcompl() + self.uz.lcompl()

impl Reverse for Motor3 {
    fn rev(self) -> Self {
        Motor3 {
            rx: -self.rx,
            ry: -self.ry,
            rz: -self.rz,
            rw: self.rw,
            ux: -self.ux,
            uy: -self.uy,
            uz: -self.uz,
            uw: self.uw,
        }
    }
}

impl AntiReverse for Motor3 {
    fn arev(self) -> Self {
        Motor3 {
            rx: -self.rx,
            ry: -self.ry,
            rz: -self.rz,
            rw: self.rw,
            ux: self.ux,
            uy: self.uy,
            uz: self.uz,
            uw: self.uw,
        }
    }
}

// ---------------------------------------------------------------------
// Motor3 OP Vec3:

// Omitted: Motor3 geometric Vec3 = self.rw.geometric(rhs.x) + self.rw.geometric(rhs.y) + self.rw.geometric(rhs.z) + self.rx.geometric(rhs.x) + self.rx.geometric(rhs.y) + self.rx.geometric(rhs.z) + self.ry.geometric(rhs.x) + self.ry.geometric(rhs.y) + self.ry.geometric(rhs.z) + self.rz.geometric(rhs.x) + self.rz.geometric(rhs.y) + self.rz.geometric(rhs.z) + self.uw.geometric(rhs.x) + self.uw.geometric(rhs.y) + self.uw.geometric(rhs.z) + self.ux.geometric(rhs.x) + self.ux.geometric(rhs.y) + self.ux.geometric(rhs.z) + self.uy.geometric(rhs.x) + self.uy.geometric(rhs.y) + self.uy.geometric(rhs.z) + self.uz.geometric(rhs.x) + self.uz.geometric(rhs.y) + self.uz.geometric(rhs.z)
// Omitted: Motor3 anti_geometric Vec3 = self.rw.anti_geometric(rhs.x) + self.rw.anti_geometric(rhs.y) + self.rw.anti_geometric(rhs.z) + self.rx.anti_geometric(rhs.x) + self.rx.anti_geometric(rhs.y) + self.rx.anti_geometric(rhs.z) + self.ry.anti_geometric(rhs.x) + self.ry.anti_geometric(rhs.y) + self.ry.anti_geometric(rhs.z) + self.rz.anti_geometric(rhs.x) + self.rz.anti_geometric(rhs.y) + self.rz.anti_geometric(rhs.z) + self.ux.anti_geometric(rhs.x) + self.ux.anti_geometric(rhs.y) + self.ux.anti_geometric(rhs.z) + self.uy.anti_geometric(rhs.x) + self.uy.anti_geometric(rhs.y) + self.uy.anti_geometric(rhs.z) + self.uz.anti_geometric(rhs.x) + self.uz.anti_geometric(rhs.y) + self.uz.anti_geometric(rhs.z)
// Omitted: Motor3 dot Vec3 = self.rw.dot(rhs.x) + self.rw.dot(rhs.y) + self.rw.dot(rhs.z) + self.rx.dot(rhs.x) + self.ry.dot(rhs.y) + self.rz.dot(rhs.z) + self.uw.dot(rhs.x) + self.uw.dot(rhs.y) + self.uw.dot(rhs.z) + self.ux.dot(rhs.y) + self.ux.dot(rhs.z) + self.uy.dot(rhs.x) + self.uy.dot(rhs.z) + self.uz.dot(rhs.x) + self.uz.dot(rhs.y)
// Omitted: Motor3 wedge Vec3 = self.rx.wedge(rhs.y) + self.rx.wedge(rhs.z) + self.ry.wedge(rhs.x) + self.ry.wedge(rhs.z) + self.rz.wedge(rhs.x) + self.rz.wedge(rhs.y) + self.uw.wedge(rhs.x) + self.uw.wedge(rhs.y) + self.uw.wedge(rhs.z) + self.ux.wedge(rhs.x) + self.uy.wedge(rhs.y) + self.uz.wedge(rhs.z)
// Omitted: Motor3 anti_wedge Vec3 = self.rw.anti_wedge(rhs.x) + self.rw.anti_wedge(rhs.y) + self.rw.anti_wedge(rhs.z) + self.ux.anti_wedge(rhs.x) + self.uy.anti_wedge(rhs.y) + self.uz.anti_wedge(rhs.z)

// ---------------------------------------------------------------------
// Motor3 OP Vec4:

// Omitted: Motor3 geometric Vec4 = self.rw.geometric(rhs.x) + self.rw.geometric(rhs.y) + self.rw.geometric(rhs.z) + self.rx.geometric(rhs.x) + self.rx.geometric(rhs.y) + self.rx.geometric(rhs.z) + self.ry.geometric(rhs.x) + self.ry.geometric(rhs.y) + self.ry.geometric(rhs.z) + self.rz.geometric(rhs.x) + self.rz.geometric(rhs.y) + self.rz.geometric(rhs.z) + self.uw.geometric(rhs.w) + self.uw.geometric(rhs.x) + self.uw.geometric(rhs.y) + self.uw.geometric(rhs.z) + self.ux.geometric(rhs.x) + self.ux.geometric(rhs.y) + self.ux.geometric(rhs.z) + self.uy.geometric(rhs.x) + self.uy.geometric(rhs.y) + self.uy.geometric(rhs.z) + self.uz.geometric(rhs.x) + self.uz.geometric(rhs.y) + self.uz.geometric(rhs.z)
// Omitted: Motor3 anti_geometric Vec4 = self.rw.anti_geometric(rhs.w) + self.rw.anti_geometric(rhs.x) + self.rw.anti_geometric(rhs.y) + self.rw.anti_geometric(rhs.z) + self.rx.anti_geometric(rhs.w) + self.rx.anti_geometric(rhs.x) + self.rx.anti_geometric(rhs.y) + self.rx.anti_geometric(rhs.z) + self.ry.anti_geometric(rhs.w) + self.ry.anti_geometric(rhs.x) + self.ry.anti_geometric(rhs.y) + self.ry.anti_geometric(rhs.z) + self.rz.anti_geometric(rhs.w) + self.rz.anti_geometric(rhs.x) + self.rz.anti_geometric(rhs.y) + self.rz.anti_geometric(rhs.z) + self.uw.anti_geometric(rhs.w) + self.ux.anti_geometric(rhs.w) + self.ux.anti_geometric(rhs.x) + self.ux.anti_geometric(rhs.y) + self.ux.anti_geometric(rhs.z) + self.uy.anti_geometric(rhs.w) + self.uy.anti_geometric(rhs.x) + self.uy.anti_geometric(rhs.y) + self.uy.anti_geometric(rhs.z) + self.uz.anti_geometric(rhs.w) + self.uz.anti_geometric(rhs.x) + self.uz.anti_geometric(rhs.y) + self.uz.anti_geometric(rhs.z)
// Omitted: Motor3 dot Vec4 = self.rw.dot(rhs.x) + self.rw.dot(rhs.y) + self.rw.dot(rhs.z) + self.rx.dot(rhs.x) + self.ry.dot(rhs.y) + self.rz.dot(rhs.z) + self.uw.dot(rhs.w) + self.uw.dot(rhs.x) + self.uw.dot(rhs.y) + self.uw.dot(rhs.z) + self.ux.dot(rhs.y) + self.ux.dot(rhs.z) + self.uy.dot(rhs.x) + self.uy.dot(rhs.z) + self.uz.dot(rhs.x) + self.uz.dot(rhs.y)
// Omitted: Motor3 wedge Vec4 = self.rx.wedge(rhs.y) + self.rx.wedge(rhs.z) + self.ry.wedge(rhs.x) + self.ry.wedge(rhs.z) + self.rz.wedge(rhs.x) + self.rz.wedge(rhs.y) + self.uw.wedge(rhs.w) + self.uw.wedge(rhs.x) + self.uw.wedge(rhs.y) + self.uw.wedge(rhs.z) + self.ux.wedge(rhs.x) + self.uy.wedge(rhs.y) + self.uz.wedge(rhs.z)
// Omitted: Motor3 anti_wedge Vec4 = self.rw.anti_wedge(rhs.w) + self.rw.anti_wedge(rhs.x) + self.rw.anti_wedge(rhs.y) + self.rw.anti_wedge(rhs.z) + self.ux.anti_wedge(rhs.x) + self.uy.anti_wedge(rhs.y) + self.uz.anti_wedge(rhs.z)

// ---------------------------------------------------------------------
// Motor3 OP Line3:

// Omitted: Motor3 geometric Line3 = self.rw.geometric(rhs.mx) + self.rw.geometric(rhs.my) + self.rw.geometric(rhs.mz) + self.rx.geometric(rhs.mx) + self.rx.geometric(rhs.my) + self.rx.geometric(rhs.mz) + self.ry.geometric(rhs.mx) + self.ry.geometric(rhs.my) + self.ry.geometric(rhs.mz) + self.rz.geometric(rhs.mx) + self.rz.geometric(rhs.my) + self.rz.geometric(rhs.mz) + self.uw.geometric(rhs.mx) + self.uw.geometric(rhs.my) + self.uw.geometric(rhs.mz) + self.uw.geometric(rhs.vx) + self.uw.geometric(rhs.vy) + self.uw.geometric(rhs.vz) + self.ux.geometric(rhs.mx) + self.ux.geometric(rhs.my) + self.ux.geometric(rhs.mz) + self.uy.geometric(rhs.mx) + self.uy.geometric(rhs.my) + self.uy.geometric(rhs.mz) + self.uz.geometric(rhs.mx) + self.uz.geometric(rhs.my) + self.uz.geometric(rhs.mz)
// Omitted: Motor3 anti_geometric Line3 = self.rw.anti_geometric(rhs.mx) + self.rw.anti_geometric(rhs.my) + self.rw.anti_geometric(rhs.mz) + self.rw.anti_geometric(rhs.vx) + self.rw.anti_geometric(rhs.vy) + self.rw.anti_geometric(rhs.vz) + self.rx.anti_geometric(rhs.mx) + self.rx.anti_geometric(rhs.my) + self.rx.anti_geometric(rhs.mz) + self.rx.anti_geometric(rhs.vx) + self.rx.anti_geometric(rhs.vy) + self.rx.anti_geometric(rhs.vz) + self.ry.anti_geometric(rhs.mx) + self.ry.anti_geometric(rhs.my) + self.ry.anti_geometric(rhs.mz) + self.ry.anti_geometric(rhs.vx) + self.ry.anti_geometric(rhs.vy) + self.ry.anti_geometric(rhs.vz) + self.rz.anti_geometric(rhs.mx) + self.rz.anti_geometric(rhs.my) + self.rz.anti_geometric(rhs.mz) + self.rz.anti_geometric(rhs.vx) + self.rz.anti_geometric(rhs.vy) + self.rz.anti_geometric(rhs.vz) + self.uw.anti_geometric(rhs.vx) + self.uw.anti_geometric(rhs.vy) + self.uw.anti_geometric(rhs.vz) + self.ux.anti_geometric(rhs.mx) + self.ux.anti_geometric(rhs.my) + self.ux.anti_geometric(rhs.mz) + self.ux.anti_geometric(rhs.vx) + self.ux.anti_geometric(rhs.vy) + self.ux.anti_geometric(rhs.vz) + self.uy.anti_geometric(rhs.mx) + self.uy.anti_geometric(rhs.my) + self.uy.anti_geometric(rhs.mz) + self.uy.anti_geometric(rhs.vx) + self.uy.anti_geometric(rhs.vy) + self.uy.anti_geometric(rhs.vz) + self.uz.anti_geometric(rhs.mx) + self.uz.anti_geometric(rhs.my) + self.uz.anti_geometric(rhs.mz) + self.uz.anti_geometric(rhs.vx) + self.uz.anti_geometric(rhs.vy) + self.uz.anti_geometric(rhs.vz)
// Omitted: Motor3 dot Line3 = self.rw.dot(rhs.mx) + self.rw.dot(rhs.my) + self.rw.dot(rhs.mz) + self.uw.dot(rhs.mx) + self.uw.dot(rhs.my) + self.uw.dot(rhs.mz) + self.uw.dot(rhs.vx) + self.uw.dot(rhs.vy) + self.uw.dot(rhs.vz) + self.ux.dot(rhs.mx) + self.uy.dot(rhs.my) + self.uz.dot(rhs.mz)
// Omitted: Motor3 wedge Line3 = self.rx.wedge(rhs.mx) + self.ry.wedge(rhs.my) + self.rz.wedge(rhs.mz) + self.uw.wedge(rhs.mx) + self.uw.wedge(rhs.my) + self.uw.wedge(rhs.mz) + self.uw.wedge(rhs.vx) + self.uw.wedge(rhs.vy) + self.uw.wedge(rhs.vz)
// Omitted: Motor3 anti_wedge Line3 = self.rw.anti_wedge(rhs.mx) + self.rw.anti_wedge(rhs.my) + self.rw.anti_wedge(rhs.mz) + self.rw.anti_wedge(rhs.vx) + self.rw.anti_wedge(rhs.vy) + self.rw.anti_wedge(rhs.vz) + self.rx.anti_wedge(rhs.mx) + self.ry.anti_wedge(rhs.my) + self.rz.anti_wedge(rhs.mz) + self.ux.anti_wedge(rhs.my) + self.ux.anti_wedge(rhs.mz) + self.ux.anti_wedge(rhs.vx) + self.uy.anti_wedge(rhs.mx) + self.uy.anti_wedge(rhs.mz) + self.uy.anti_wedge(rhs.vy) + self.uz.anti_wedge(rhs.mx) + self.uz.anti_wedge(rhs.my) + self.uz.anti_wedge(rhs.vz)

// ---------------------------------------------------------------------
// Motor3 OP Plane:

// Omitted: Motor3 geometric Plane = self.rw.geometric(rhs.d) + self.rx.geometric(rhs.d) + self.ry.geometric(rhs.d) + self.rz.geometric(rhs.d) + self.uw.geometric(rhs.d) + self.uw.geometric(rhs.nx) + self.uw.geometric(rhs.ny) + self.uw.geometric(rhs.nz) + self.ux.geometric(rhs.d) + self.uy.geometric(rhs.d) + self.uz.geometric(rhs.d)
// Omitted: Motor3 anti_geometric Plane = self.rw.anti_geometric(rhs.d) + self.rw.anti_geometric(rhs.nx) + self.rw.anti_geometric(rhs.ny) + self.rw.anti_geometric(rhs.nz) + self.rx.anti_geometric(rhs.d) + self.rx.anti_geometric(rhs.nx) + self.rx.anti_geometric(rhs.ny) + self.rx.anti_geometric(rhs.nz) + self.ry.anti_geometric(rhs.d) + self.ry.anti_geometric(rhs.nx) + self.ry.anti_geometric(rhs.ny) + self.ry.anti_geometric(rhs.nz) + self.rz.anti_geometric(rhs.d) + self.rz.anti_geometric(rhs.nx) + self.rz.anti_geometric(rhs.ny) + self.rz.anti_geometric(rhs.nz) + self.uw.anti_geometric(rhs.nx) + self.uw.anti_geometric(rhs.ny) + self.uw.anti_geometric(rhs.nz) + self.ux.anti_geometric(rhs.d) + self.ux.anti_geometric(rhs.nx) + self.ux.anti_geometric(rhs.ny) + self.ux.anti_geometric(rhs.nz) + self.uy.anti_geometric(rhs.d) + self.uy.anti_geometric(rhs.nx) + self.uy.anti_geometric(rhs.ny) + self.uy.anti_geometric(rhs.nz) + self.uz.anti_geometric(rhs.d) + self.uz.anti_geometric(rhs.nx) + self.uz.anti_geometric(rhs.ny) + self.uz.anti_geometric(rhs.nz)
// Omitted: Motor3 dot Plane = self.rw.dot(rhs.d) + self.uw.dot(rhs.d) + self.uw.dot(rhs.nx) + self.uw.dot(rhs.ny) + self.uw.dot(rhs.nz)

// Motor3.wedge(Plane) -> Plane
impl Wedge<Plane> for Motor3 {
    type Output = Plane;
    fn wedge(self, rhs: Plane) -> Self::Output {
        Plane {
            nx: self.uw.wedge(rhs.nx),
            ny: self.uw.wedge(rhs.ny),
            nz: self.uw.wedge(rhs.nz),
            d: self.uw.wedge(rhs.d),
        }
    }
}

// Omitted: Motor3 anti_wedge Plane = self.rw.anti_wedge(rhs.d) + self.rw.anti_wedge(rhs.nx) + self.rw.anti_wedge(rhs.ny) + self.rw.anti_wedge(rhs.nz) + self.rx.anti_wedge(rhs.d) + self.rx.anti_wedge(rhs.nx) + self.ry.anti_wedge(rhs.d) + self.ry.anti_wedge(rhs.ny) + self.rz.anti_wedge(rhs.d) + self.rz.anti_wedge(rhs.nz) + self.ux.anti_wedge(rhs.d) + self.ux.anti_wedge(rhs.ny) + self.ux.anti_wedge(rhs.nz) + self.uy.anti_wedge(rhs.d) + self.uy.anti_wedge(rhs.nx) + self.uy.anti_wedge(rhs.nz) + self.uz.anti_wedge(rhs.d) + self.uz.anti_wedge(rhs.nx) + self.uz.anti_wedge(rhs.ny)

// ---------------------------------------------------------------------
// Motor3 OP Translator3:

// Omitted: Motor3 geometric Translator3 = self.rw.geometric(rhs.x) + self.rw.geometric(rhs.y) + self.rw.geometric(rhs.z) + self.rx.geometric(rhs.x) + self.rx.geometric(rhs.y) + self.rx.geometric(rhs.z) + self.ry.geometric(rhs.x) + self.ry.geometric(rhs.y) + self.ry.geometric(rhs.z) + self.rz.geometric(rhs.x) + self.rz.geometric(rhs.y) + self.rz.geometric(rhs.z) + self.uw.geometric(rhs.x) + self.uw.geometric(rhs.xyzw) + self.uw.geometric(rhs.y) + self.uw.geometric(rhs.z) + self.ux.geometric(rhs.x) + self.ux.geometric(rhs.y) + self.ux.geometric(rhs.z) + self.uy.geometric(rhs.x) + self.uy.geometric(rhs.y) + self.uy.geometric(rhs.z) + self.uz.geometric(rhs.x) + self.uz.geometric(rhs.y) + self.uz.geometric(rhs.z)
// Omitted: Motor3 anti_geometric Translator3 = self.rw.anti_geometric(rhs.x) + self.rw.anti_geometric(rhs.xyzw) + self.rw.anti_geometric(rhs.y) + self.rw.anti_geometric(rhs.z) + self.rx.anti_geometric(rhs.x) + self.rx.anti_geometric(rhs.xyzw) + self.rx.anti_geometric(rhs.y) + self.rx.anti_geometric(rhs.z) + self.ry.anti_geometric(rhs.x) + self.ry.anti_geometric(rhs.xyzw) + self.ry.anti_geometric(rhs.y) + self.ry.anti_geometric(rhs.z) + self.rz.anti_geometric(rhs.x) + self.rz.anti_geometric(rhs.xyzw) + self.rz.anti_geometric(rhs.y) + self.rz.anti_geometric(rhs.z) + self.uw.anti_geometric(rhs.xyzw) + self.ux.anti_geometric(rhs.x) + self.ux.anti_geometric(rhs.xyzw) + self.ux.anti_geometric(rhs.y) + self.ux.anti_geometric(rhs.z) + self.uy.anti_geometric(rhs.x) + self.uy.anti_geometric(rhs.xyzw) + self.uy.anti_geometric(rhs.y) + self.uy.anti_geometric(rhs.z) + self.uz.anti_geometric(rhs.x) + self.uz.anti_geometric(rhs.xyzw) + self.uz.anti_geometric(rhs.y) + self.uz.anti_geometric(rhs.z)
// Omitted: Motor3 dot Translator3 = self.rw.dot(rhs.x) + self.rw.dot(rhs.y) + self.rw.dot(rhs.z) + self.uw.dot(rhs.x) + self.uw.dot(rhs.xyzw) + self.uw.dot(rhs.y) + self.uw.dot(rhs.z) + self.ux.dot(rhs.x) + self.uy.dot(rhs.y) + self.uz.dot(rhs.z)

// Motor3.wedge(Translator3) -> Translator3
impl Wedge<Translator3> for Motor3 {
    type Output = Translator3;
    fn wedge(self, rhs: Translator3) -> Self::Output {
        Translator3 {
            x: self.uw.wedge(rhs.x),
            y: self.uw.wedge(rhs.y),
            z: self.uw.wedge(rhs.z),
            xyzw: -self.rx.wedge(rhs.x) - self.ry.wedge(rhs.y) - self.rz.wedge(rhs.z)
                + self.uw.wedge(rhs.xyzw),
        }
    }
}

// Omitted: Motor3 anti_wedge Translator3 = self.rw.anti_wedge(rhs.x) + self.rw.anti_wedge(rhs.xyzw) + self.rw.anti_wedge(rhs.y) + self.rw.anti_wedge(rhs.z) + self.rx.anti_wedge(rhs.x) + self.rx.anti_wedge(rhs.xyzw) + self.ry.anti_wedge(rhs.xyzw) + self.ry.anti_wedge(rhs.y) + self.rz.anti_wedge(rhs.xyzw) + self.rz.anti_wedge(rhs.z) + self.uw.anti_wedge(rhs.xyzw) + self.ux.anti_wedge(rhs.xyzw) + self.ux.anti_wedge(rhs.y) + self.ux.anti_wedge(rhs.z) + self.uy.anti_wedge(rhs.x) + self.uy.anti_wedge(rhs.xyzw) + self.uy.anti_wedge(rhs.z) + self.uz.anti_wedge(rhs.x) + self.uz.anti_wedge(rhs.xyzw) + self.uz.anti_wedge(rhs.y)

// ---------------------------------------------------------------------
// Motor3 OP Rotor3:

// Motor3.geometric(Rotor3) -> Rotor3
impl Geometric<Rotor3> for Motor3 {
    type Output = Rotor3;
    fn geometric(self, rhs: Rotor3) -> Self::Output {
        Rotor3 {
            x: self.uw.geometric(rhs.x),
            y: self.uw.geometric(rhs.y),
            z: self.uw.geometric(rhs.z),
            w: self.uw.geometric(rhs.w),
        }
    }
}

// Omitted: Motor3 anti_geometric Rotor3 = self.rw.anti_geometric(rhs.w) + self.rw.anti_geometric(rhs.x) + self.rw.anti_geometric(rhs.y) + self.rw.anti_geometric(rhs.z) + self.rx.anti_geometric(rhs.w) + self.rx.anti_geometric(rhs.x) + self.rx.anti_geometric(rhs.y) + self.rx.anti_geometric(rhs.z) + self.ry.anti_geometric(rhs.w) + self.ry.anti_geometric(rhs.x) + self.ry.anti_geometric(rhs.y) + self.ry.anti_geometric(rhs.z) + self.rz.anti_geometric(rhs.w) + self.rz.anti_geometric(rhs.x) + self.rz.anti_geometric(rhs.y) + self.rz.anti_geometric(rhs.z) + self.uw.anti_geometric(rhs.w) + self.uw.anti_geometric(rhs.x) + self.uw.anti_geometric(rhs.y) + self.uw.anti_geometric(rhs.z) + self.ux.anti_geometric(rhs.w) + self.ux.anti_geometric(rhs.x) + self.ux.anti_geometric(rhs.y) + self.ux.anti_geometric(rhs.z) + self.uy.anti_geometric(rhs.w) + self.uy.anti_geometric(rhs.x) + self.uy.anti_geometric(rhs.y) + self.uy.anti_geometric(rhs.z) + self.uz.anti_geometric(rhs.w) + self.uz.anti_geometric(rhs.x) + self.uz.anti_geometric(rhs.y) + self.uz.anti_geometric(rhs.z)

// Motor3.dot(Rotor3) -> Rotor3
impl Dot<Rotor3> for Motor3 {
    type Output = Rotor3;
    fn dot(self, rhs: Rotor3) -> Self::Output {
        Rotor3 {
            x: self.uw.dot(rhs.x),
            y: self.uw.dot(rhs.y),
            z: self.uw.dot(rhs.z),
            w: self.uw.dot(rhs.w),
        }
    }
}

// Motor3.wedge(Rotor3) -> Rotor3
impl Wedge<Rotor3> for Motor3 {
    type Output = Rotor3;
    fn wedge(self, rhs: Rotor3) -> Self::Output {
        Rotor3 {
            x: self.uw.wedge(rhs.x),
            y: self.uw.wedge(rhs.y),
            z: self.uw.wedge(rhs.z),
            w: self.uw.wedge(rhs.w),
        }
    }
}

// Omitted: Motor3 anti_wedge Rotor3 = self.rw.anti_wedge(rhs.w) + self.rw.anti_wedge(rhs.x) + self.rw.anti_wedge(rhs.y) + self.rw.anti_wedge(rhs.z) + self.rx.anti_wedge(rhs.w) + self.ry.anti_wedge(rhs.w) + self.rz.anti_wedge(rhs.w) + self.uw.anti_wedge(rhs.w) + self.ux.anti_wedge(rhs.w) + self.ux.anti_wedge(rhs.x) + self.uy.anti_wedge(rhs.w) + self.uy.anti_wedge(rhs.y) + self.uz.anti_wedge(rhs.w) + self.uz.anti_wedge(rhs.z)

// ---------------------------------------------------------------------
// Motor3 OP Motor3:

// Motor3.geometric(Motor3) -> Motor3
impl Geometric<Motor3> for Motor3 {
    type Output = Motor3;
    fn geometric(self, rhs: Motor3) -> Self::Output {
        Motor3 {
            rx: self.rx.geometric(rhs.uw) + self.uw.geometric(rhs.rx),
            ry: self.ry.geometric(rhs.uw) + self.uw.geometric(rhs.ry),
            rz: self.rz.geometric(rhs.uw) + self.uw.geometric(rhs.rz),
            rw: self.rw.geometric(rhs.uw) + self.uw.geometric(rhs.rw),
            ux: self.uw.geometric(rhs.ux) + self.ux.geometric(rhs.uw),
            uy: self.uw.geometric(rhs.uy) + self.uy.geometric(rhs.uw),
            uz: self.uw.geometric(rhs.uz) + self.uz.geometric(rhs.uw),
            uw: self.uw.geometric(rhs.uw),
        }
    }
}

// Omitted: Motor3 anti_geometric Motor3 = self.rw.anti_geometric(rhs.rw) + self.rw.anti_geometric(rhs.rx) + self.rw.anti_geometric(rhs.ry) + self.rw.anti_geometric(rhs.rz) + self.rw.anti_geometric(rhs.uw) + self.rw.anti_geometric(rhs.ux) + self.rw.anti_geometric(rhs.uy) + self.rw.anti_geometric(rhs.uz) + self.rx.anti_geometric(rhs.rw) + self.rx.anti_geometric(rhs.rx) + self.rx.anti_geometric(rhs.ry) + self.rx.anti_geometric(rhs.rz) + self.rx.anti_geometric(rhs.uw) + self.rx.anti_geometric(rhs.ux) + self.rx.anti_geometric(rhs.uy) + self.rx.anti_geometric(rhs.uz) + self.ry.anti_geometric(rhs.rw) + self.ry.anti_geometric(rhs.rx) + self.ry.anti_geometric(rhs.ry) + self.ry.anti_geometric(rhs.rz) + self.ry.anti_geometric(rhs.uw) + self.ry.anti_geometric(rhs.ux) + self.ry.anti_geometric(rhs.uy) + self.ry.anti_geometric(rhs.uz) + self.rz.anti_geometric(rhs.rw) + self.rz.anti_geometric(rhs.rx) + self.rz.anti_geometric(rhs.ry) + self.rz.anti_geometric(rhs.rz) + self.rz.anti_geometric(rhs.uw) + self.rz.anti_geometric(rhs.ux) + self.rz.anti_geometric(rhs.uy) + self.rz.anti_geometric(rhs.uz) + self.uw.anti_geometric(rhs.rw) + self.uw.anti_geometric(rhs.rx) + self.uw.anti_geometric(rhs.ry) + self.uw.anti_geometric(rhs.rz) + self.uw.anti_geometric(rhs.ux) + self.uw.anti_geometric(rhs.uy) + self.uw.anti_geometric(rhs.uz) + self.ux.anti_geometric(rhs.rw) + self.ux.anti_geometric(rhs.rx) + self.ux.anti_geometric(rhs.ry) + self.ux.anti_geometric(rhs.rz) + self.ux.anti_geometric(rhs.uw) + self.ux.anti_geometric(rhs.ux) + self.ux.anti_geometric(rhs.uy) + self.ux.anti_geometric(rhs.uz) + self.uy.anti_geometric(rhs.rw) + self.uy.anti_geometric(rhs.rx) + self.uy.anti_geometric(rhs.ry) + self.uy.anti_geometric(rhs.rz) + self.uy.anti_geometric(rhs.uw) + self.uy.anti_geometric(rhs.ux) + self.uy.anti_geometric(rhs.uy) + self.uy.anti_geometric(rhs.uz) + self.uz.anti_geometric(rhs.rw) + self.uz.anti_geometric(rhs.rx) + self.uz.anti_geometric(rhs.ry) + self.uz.anti_geometric(rhs.rz) + self.uz.anti_geometric(rhs.uw) + self.uz.anti_geometric(rhs.ux) + self.uz.anti_geometric(rhs.uy) + self.uz.anti_geometric(rhs.uz)

// Motor3.dot(Motor3) -> Motor3
impl Dot<Motor3> for Motor3 {
    type Output = Motor3;
    fn dot(self, rhs: Motor3) -> Self::Output {
        Motor3 {
            rx: self.rx.dot(rhs.uw) + self.uw.dot(rhs.rx),
            ry: self.ry.dot(rhs.uw) + self.uw.dot(rhs.ry),
            rz: self.rz.dot(rhs.uw) + self.uw.dot(rhs.rz),
            rw: self.rw.dot(rhs.uw) + self.uw.dot(rhs.rw),
            ux: self.uw.dot(rhs.ux) + self.ux.dot(rhs.uw),
            uy: self.uw.dot(rhs.uy) + self.uy.dot(rhs.uw),
            uz: self.uw.dot(rhs.uz) + self.uz.dot(rhs.uw),
            uw: self.uw.dot(rhs.uw),
        }
    }
}

// Motor3.wedge(Motor3) -> Motor3
impl Wedge<Motor3> for Motor3 {
    type Output = Motor3;
    fn wedge(self, rhs: Motor3) -> Self::Output {
        Motor3 {
            rx: self.rx.wedge(rhs.uw) + self.uw.wedge(rhs.rx),
            ry: self.ry.wedge(rhs.uw) + self.uw.wedge(rhs.ry),
            rz: self.rz.wedge(rhs.uw) + self.uw.wedge(rhs.rz),
            rw: self.rw.wedge(rhs.uw) + self.uw.wedge(rhs.rw),
            ux: self.uw.wedge(rhs.ux) + self.ux.wedge(rhs.uw),
            uy: self.uw.wedge(rhs.uy) + self.uy.wedge(rhs.uw),
            uz: self.uw.wedge(rhs.uz) + self.uz.wedge(rhs.uw),
            uw: self.uw.wedge(rhs.uw),
        }
    }
}

// Omitted: Motor3 anti_wedge Motor3 = self.rw.anti_wedge(rhs.rw) + self.rw.anti_wedge(rhs.rx) + self.rw.anti_wedge(rhs.ry) + self.rw.anti_wedge(rhs.rz) + self.rw.anti_wedge(rhs.uw) + self.rw.anti_wedge(rhs.ux) + self.rw.anti_wedge(rhs.uy) + self.rw.anti_wedge(rhs.uz) + self.rx.anti_wedge(rhs.rw) + self.rx.anti_wedge(rhs.ux) + self.ry.anti_wedge(rhs.rw) + self.ry.anti_wedge(rhs.uy) + self.rz.anti_wedge(rhs.rw) + self.rz.anti_wedge(rhs.uz) + self.uw.anti_wedge(rhs.rw) + self.ux.anti_wedge(rhs.rw) + self.ux.anti_wedge(rhs.rx) + self.ux.anti_wedge(rhs.uy) + self.ux.anti_wedge(rhs.uz) + self.uy.anti_wedge(rhs.rw) + self.uy.anti_wedge(rhs.ry) + self.uy.anti_wedge(rhs.ux) + self.uy.anti_wedge(rhs.uz) + self.uz.anti_wedge(rhs.rw) + self.uz.anti_wedge(rhs.rz) + self.uz.anti_wedge(rhs.ux) + self.uz.anti_wedge(rhs.uy)

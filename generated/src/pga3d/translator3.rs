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

// Omitted: Translator3 geometric Vec3 = self.x * rhs.x + self.x * rhs.y + self.x * rhs.z + self.xyzw * rhs.x + self.xyzw * rhs.y + self.xyzw * rhs.z + self.y * rhs.x + self.y * rhs.y + self.y * rhs.z + self.z * rhs.x + self.z * rhs.y + self.z * rhs.z

// Translator3.anti_geometric(Vec3) -> Vec3
impl AntiGeometric<Vec3> for Translator3 {
    type Output = Vec3;
    fn anti_geometric(self, rhs: Vec3) -> Self::Output {
        // Vec3 {
        //     x: X(self.xyzw.0 * rhs.x.0),
        //     y: Y(self.xyzw.0 * rhs.y.0),
        //     z: Z(self.xyzw.0 * rhs.z.0),
        // }
        Vec3 {
            x: self.xyzw.anti_geometric(rhs.x),
            y: self.xyzw.anti_geometric(rhs.y),
            z: self.xyzw.anti_geometric(rhs.z),
        }
    }
}

// Omitted: Translator3 dot Vec3 = self.x | rhs.y + self.x | rhs.z + self.xyzw | rhs.x + self.xyzw | rhs.y + self.xyzw | rhs.z + self.y | rhs.x + self.y | rhs.z + self.z | rhs.x + self.z | rhs.y

// Translator3.wedge(Vec3) -> ZYX
impl Wedge<Vec3> for Translator3 {
    type Output = ZYX;
    fn wedge(self, rhs: Vec3) -> Self::Output {
        // -ZYX(self.x.0 * rhs.x.0) - ZYX(self.y.0 * rhs.y.0) - ZYX(self.z.0 * rhs.z.0)
        self.x.wedge(rhs.x) + self.y.wedge(rhs.y) + self.z.wedge(rhs.z)
    }
}

// Translator3.anti_wedge(Vec3) -> Vec3
impl AntiWedge<Vec3> for Translator3 {
    type Output = Vec3;
    fn anti_wedge(self, rhs: Vec3) -> Self::Output {
        // Vec3 {
        //     x: X(self.xyzw.0 * rhs.x.0),
        //     y: Y(self.xyzw.0 * rhs.y.0),
        //     z: Z(self.xyzw.0 * rhs.z.0),
        // }
        Vec3 {
            x: self.xyzw.anti_wedge(rhs.x),
            y: self.xyzw.anti_wedge(rhs.y),
            z: self.xyzw.anti_wedge(rhs.z),
        }
    }
}

// ---------------------------------------------------------------------
// Translator3 OP Vec4:

// Omitted: Translator3 geometric Vec4 = self.x * rhs.w + self.x * rhs.x + self.x * rhs.y + self.x * rhs.z + self.xyzw * rhs.x + self.xyzw * rhs.y + self.xyzw * rhs.z + self.y * rhs.w + self.y * rhs.x + self.y * rhs.y + self.y * rhs.z + self.z * rhs.w + self.z * rhs.x + self.z * rhs.y + self.z * rhs.z

// Translator3.anti_geometric(Vec4) -> Vec4
impl AntiGeometric<Vec4> for Translator3 {
    type Output = Vec4;
    fn anti_geometric(self, rhs: Vec4) -> Self::Output {
        // Vec4 {
        //     x: X(self.x.0 * rhs.w.0) + X(self.xyzw.0 * rhs.x.0),
        //     y: Y(self.xyzw.0 * rhs.y.0) + Y(self.y.0 * rhs.w.0),
        //     z: Z(self.xyzw.0 * rhs.z.0) + Z(self.z.0 * rhs.w.0),
        //     w: W(self.xyzw.0 * rhs.w.0),
        // }
        Vec4 {
            x: self.x.anti_geometric(rhs.w) + self.xyzw.anti_geometric(rhs.x),
            y: self.xyzw.anti_geometric(rhs.y) + self.y.anti_geometric(rhs.w),
            z: self.xyzw.anti_geometric(rhs.z) + self.z.anti_geometric(rhs.w),
            w: self.xyzw.anti_geometric(rhs.w),
        }
    }
}

// Omitted: Translator3 dot Vec4 = self.x | rhs.y + self.x | rhs.z + self.xyzw | rhs.x + self.xyzw | rhs.y + self.xyzw | rhs.z + self.y | rhs.x + self.y | rhs.z + self.z | rhs.x + self.z | rhs.y

// Translator3.wedge(Vec4) -> Plane
impl Wedge<Vec4> for Translator3 {
    type Output = Plane;
    fn wedge(self, rhs: Vec4) -> Self::Output {
        // Plane {
        //     nx: YZW(self.x.0 * rhs.w.0),
        //     ny: ZXW(self.y.0 * rhs.w.0),
        //     nz: XYW(self.z.0 * rhs.w.0),
        //     d : ZYX(self.x.0 * rhs.x.0) + ZYX(self.y.0 * rhs.y.0) + ZYX(self.z.0 * rhs.z.0),
        // }
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
        // Vec4 {
        //     x: X(self.xyzw.0 * rhs.x.0),
        //     y: Y(self.xyzw.0 * rhs.y.0),
        //     z: Z(self.xyzw.0 * rhs.z.0),
        //     w: W(self.xyzw.0 * rhs.w.0),
        // }
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

// Omitted: Translator3 geometric Line3 = self.x * rhs.mx + self.x * rhs.my + self.x * rhs.mz + self.x * rhs.vx + self.x * rhs.vy + self.x * rhs.vz + self.xyzw * rhs.mx + self.xyzw * rhs.my + self.xyzw * rhs.mz + self.y * rhs.mx + self.y * rhs.my + self.y * rhs.mz + self.y * rhs.vx + self.y * rhs.vy + self.y * rhs.vz + self.z * rhs.mx + self.z * rhs.my + self.z * rhs.mz + self.z * rhs.vx + self.z * rhs.vy + self.z * rhs.vz
// Omitted: Translator3 anti_geometric Line3 = self.x !* rhs.vx + self.x !* rhs.vy + self.x !* rhs.vz + self.xyzw !* rhs.mx + self.xyzw !* rhs.my + self.xyzw !* rhs.mz + self.xyzw !* rhs.vx + self.xyzw !* rhs.vy + self.xyzw !* rhs.vz + self.y !* rhs.vx + self.y !* rhs.vy + self.y !* rhs.vz + self.z !* rhs.vx + self.z !* rhs.vy + self.z !* rhs.vz

// Translator3.dot(Line3) -> Motor3
impl Dot<Line3> for Translator3 {
    type Output = Motor3;
    fn dot(self, rhs: Line3) -> Self::Output {
        // Motor3 {
        //     rx: WX(self.xyzw.0 * rhs.mx.0),
        //     ry: WY(self.xyzw.0 * rhs.my.0),
        //     rz: WZ(self.xyzw.0 * rhs.mz.0),
        //     rw: Default::default(),
        //     ux: Default::default(),
        //     uy: Default::default(),
        //     uz: Default::default(),
        //     uw: S(self.x.0 * rhs.mx.0) + S(self.y.0 * rhs.my.0) + S(self.z.0 * rhs.mz.0),
        // }
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
        // -XYZW(self.x.0 * rhs.vx.0) - XYZW(self.y.0 * rhs.vy.0) - XYZW(self.z.0 * rhs.vz.0)
        self.x.wedge(rhs.vx) + self.y.wedge(rhs.vy) + self.z.wedge(rhs.vz)
    }
}

// Omitted: Translator3 anti_wedge Line3 = self.x & rhs.vx + self.xyzw & rhs.mx + self.xyzw & rhs.my + self.xyzw & rhs.mz + self.xyzw & rhs.vx + self.xyzw & rhs.vy + self.xyzw & rhs.vz + self.y & rhs.vy + self.z & rhs.vz

// ---------------------------------------------------------------------
// Translator3 OP Plane:

// Omitted: Translator3 geometric Plane = self.x * rhs.d + self.x * rhs.nx + self.x * rhs.ny + self.x * rhs.nz + self.xyzw * rhs.d + self.y * rhs.d + self.y * rhs.nx + self.y * rhs.ny + self.y * rhs.nz + self.z * rhs.d + self.z * rhs.nx + self.z * rhs.ny + self.z * rhs.nz
// Omitted: Translator3 anti_geometric Plane = self.x !* rhs.nx + self.x !* rhs.ny + self.x !* rhs.nz + self.xyzw !* rhs.d + self.xyzw !* rhs.nx + self.xyzw !* rhs.ny + self.xyzw !* rhs.nz + self.y !* rhs.nx + self.y !* rhs.ny + self.y !* rhs.nz + self.z !* rhs.nx + self.z !* rhs.ny + self.z !* rhs.nz

// Translator3.dot(Plane) -> Vec4
impl Dot<Plane> for Translator3 {
    type Output = Vec4;
    fn dot(self, rhs: Plane) -> Self::Output {
        // Vec4 {
        //     x: X(self.x.0 * rhs.d.0),
        //     y: Y(self.y.0 * rhs.d.0),
        //     z: Z(self.z.0 * rhs.d.0),
        //     w: W(self.x.0 * rhs.nx.0) + W(self.xyzw.0 * rhs.d.0) + W(self.y.0 * rhs.ny.0) + W(self.z.0 * rhs.nz.0),
        // }
        Vec4 {
            x: self.x.dot(rhs.d),
            y: self.y.dot(rhs.d),
            z: self.z.dot(rhs.d),
            w: -self.x.dot(rhs.nx) - self.xyzw.dot(rhs.d) - self.y.dot(rhs.ny) - self.z.dot(rhs.nz),
        }
    }
}

// Omitted: Translator3 wedge Plane = 0
// Omitted: Translator3 anti_wedge Plane = self.x & rhs.ny + self.x & rhs.nz + self.xyzw & rhs.d + self.xyzw & rhs.nx + self.xyzw & rhs.ny + self.xyzw & rhs.nz + self.y & rhs.nx + self.y & rhs.nz + self.z & rhs.nx + self.z & rhs.ny

// ---------------------------------------------------------------------
// Translator3 OP Translator3:

// Omitted: Translator3 geometric Translator3 = self.x * rhs.x + self.x * rhs.xyzw + self.x * rhs.y + self.x * rhs.z + self.xyzw * rhs.x + self.xyzw * rhs.y + self.xyzw * rhs.z + self.y * rhs.x + self.y * rhs.xyzw + self.y * rhs.y + self.y * rhs.z + self.z * rhs.x + self.z * rhs.xyzw + self.z * rhs.y + self.z * rhs.z

// Translator3.anti_geometric(Translator3) -> Translator3
impl AntiGeometric<Translator3> for Translator3 {
    type Output = Translator3;
    fn anti_geometric(self, rhs: Translator3) -> Self::Output {
        // Translator3 {
        //     x   : YZ(self.x.0 * rhs.xyzw.0) + YZ(self.xyzw.0 * rhs.x.0),
        //     y   : ZX(self.xyzw.0 * rhs.y.0) + ZX(self.y.0 * rhs.xyzw.0),
        //     z   : XY(self.xyzw.0 * rhs.z.0) + XY(self.z.0 * rhs.xyzw.0),
        //     xyzw: XYZW(self.xyzw.0 * rhs.xyzw.0),
        // }
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
        // Motor3 {
        //     rx: WX(self.x.0 * rhs.xyzw.0) + WX(self.xyzw.0 * rhs.x.0),
        //     ry: WY(self.xyzw.0 * rhs.y.0) + WY(self.y.0 * rhs.xyzw.0),
        //     rz: WZ(self.xyzw.0 * rhs.z.0) + WZ(self.z.0 * rhs.xyzw.0),
        //     rw: Default::default(),
        //     ux: Default::default(),
        //     uy: Default::default(),
        //     uz: Default::default(),
        //     uw: S(self.x.0 * rhs.x.0) + S(self.y.0 * rhs.y.0) + S(self.z.0 * rhs.z.0),
        // }
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
        // Translator3 {
        //     x   : YZ(self.x.0 * rhs.xyzw.0) + YZ(self.xyzw.0 * rhs.x.0),
        //     y   : ZX(self.xyzw.0 * rhs.y.0) + ZX(self.y.0 * rhs.xyzw.0),
        //     z   : XY(self.xyzw.0 * rhs.z.0) + XY(self.z.0 * rhs.xyzw.0),
        //     xyzw: XYZW(self.xyzw.0 * rhs.xyzw.0),
        // }
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
        // Rotor3 {
        //     x: WX(self.x.0 * rhs.w.0) + WX(self.y.0 * rhs.z.0) + WX(self.z.0 * rhs.y.0),
        //     y: WY(self.x.0 * rhs.z.0) + WY(self.y.0 * rhs.w.0) + WY(self.z.0 * rhs.x.0),
        //     z: WZ(self.x.0 * rhs.y.0) + WZ(self.y.0 * rhs.x.0) + WZ(self.z.0 * rhs.w.0),
        //     w: XYZW(self.x.0 * rhs.x.0) + XYZW(self.y.0 * rhs.y.0) + XYZW(self.z.0 * rhs.z.0),
        // }
        Rotor3 {
            x: self.x.geometric(rhs.w) - self.y.geometric(rhs.z) + self.z.geometric(rhs.y),
            y: self.x.geometric(rhs.z) + self.y.geometric(rhs.w) - self.z.geometric(rhs.x),
            z: -self.x.geometric(rhs.y) + self.y.geometric(rhs.x) + self.z.geometric(rhs.w),
            w: -self.x.geometric(rhs.x) - self.y.geometric(rhs.y) - self.z.geometric(rhs.z),
        }
    }
}

// Omitted: Translator3 anti_geometric Rotor3 = self.x !* rhs.w + self.x !* rhs.x + self.x !* rhs.y + self.x !* rhs.z + self.xyzw !* rhs.w + self.xyzw !* rhs.x + self.xyzw !* rhs.y + self.xyzw !* rhs.z + self.y !* rhs.w + self.y !* rhs.x + self.y !* rhs.y + self.y !* rhs.z + self.z !* rhs.w + self.z !* rhs.x + self.z !* rhs.y + self.z !* rhs.z

// Translator3.dot(Rotor3) -> Line3
impl Dot<Rotor3> for Translator3 {
    type Output = Line3;
    fn dot(self, rhs: Rotor3) -> Self::Output {
        // Line3 {
        //     vx: WX(self.x.0 * rhs.w.0),
        //     vy: WY(self.y.0 * rhs.w.0),
        //     vz: WZ(self.z.0 * rhs.w.0),
        //     mx: Default::default(),
        //     my: Default::default(),
        //     mz: Default::default(),
        // }
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
        // -XYZW(self.x.0 * rhs.x.0) - XYZW(self.y.0 * rhs.y.0) - XYZW(self.z.0 * rhs.z.0)
        self.x.wedge(rhs.x) + self.y.wedge(rhs.y) + self.z.wedge(rhs.z)
    }
}

// Omitted: Translator3 anti_wedge Rotor3 = self.x & rhs.w + self.x & rhs.x + self.xyzw & rhs.w + self.xyzw & rhs.x + self.xyzw & rhs.y + self.xyzw & rhs.z + self.y & rhs.w + self.y & rhs.y + self.z & rhs.w + self.z & rhs.z

// ---------------------------------------------------------------------
// Translator3 OP Motor3:

// Omitted: Translator3 geometric Motor3 = self.x * rhs.rw + self.x * rhs.rx + self.x * rhs.ry + self.x * rhs.rz + self.x * rhs.uw + self.x * rhs.ux + self.x * rhs.uy + self.x * rhs.uz + self.xyzw * rhs.uw + self.y * rhs.rw + self.y * rhs.rx + self.y * rhs.ry + self.y * rhs.rz + self.y * rhs.uw + self.y * rhs.ux + self.y * rhs.uy + self.y * rhs.uz + self.z * rhs.rw + self.z * rhs.rx + self.z * rhs.ry + self.z * rhs.rz + self.z * rhs.uw + self.z * rhs.ux + self.z * rhs.uy + self.z * rhs.uz
// Omitted: Translator3 anti_geometric Motor3 = self.x !* rhs.rw + self.x !* rhs.rx + self.x !* rhs.ry + self.x !* rhs.rz + self.x !* rhs.ux + self.x !* rhs.uy + self.x !* rhs.uz + self.xyzw !* rhs.rw + self.xyzw !* rhs.rx + self.xyzw !* rhs.ry + self.xyzw !* rhs.rz + self.xyzw !* rhs.uw + self.xyzw !* rhs.ux + self.xyzw !* rhs.uy + self.xyzw !* rhs.uz + self.y !* rhs.rw + self.y !* rhs.rx + self.y !* rhs.ry + self.y !* rhs.rz + self.y !* rhs.ux + self.y !* rhs.uy + self.y !* rhs.uz + self.z !* rhs.rw + self.z !* rhs.rx + self.z !* rhs.ry + self.z !* rhs.rz + self.z !* rhs.ux + self.z !* rhs.uy + self.z !* rhs.uz
// Omitted: Translator3 dot Motor3 = self.x | rhs.rw + self.x | rhs.uw + self.x | rhs.ux + self.xyzw | rhs.uw + self.y | rhs.rw + self.y | rhs.uw + self.y | rhs.uy + self.z | rhs.rw + self.z | rhs.uw + self.z | rhs.uz

// Translator3.wedge(Motor3) -> Translator3
impl Wedge<Motor3> for Translator3 {
    type Output = Translator3;
    fn wedge(self, rhs: Motor3) -> Self::Output {
        // Translator3 {
        //     x   : YZ(self.x.0 * rhs.uw.0),
        //     y   : ZX(self.y.0 * rhs.uw.0),
        //     z   : XY(self.z.0 * rhs.uw.0),
        //     xyzw: XYZW(self.x.0 * rhs.rx.0) + XYZW(self.xyzw.0 * rhs.uw.0) + XYZW(self.y.0 * rhs.ry.0) + XYZW(self.z.0 * rhs.rz.0),
        // }
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

// Omitted: Translator3 anti_wedge Motor3 = self.x & rhs.rw + self.x & rhs.rx + self.x & rhs.uy + self.x & rhs.uz + self.xyzw & rhs.rw + self.xyzw & rhs.rx + self.xyzw & rhs.ry + self.xyzw & rhs.rz + self.xyzw & rhs.uw + self.xyzw & rhs.ux + self.xyzw & rhs.uy + self.xyzw & rhs.uz + self.y & rhs.rw + self.y & rhs.ry + self.y & rhs.ux + self.y & rhs.uz + self.z & rhs.rw + self.z & rhs.rz + self.z & rhs.ux + self.z & rhs.uy

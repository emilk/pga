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

// Omitted: Vec3 geometric Vec3 = self.x * rhs.x + self.x * rhs.y + self.x * rhs.z + self.y * rhs.x + self.y * rhs.y + self.y * rhs.z + self.z * rhs.x + self.z * rhs.y + self.z * rhs.z
// Omitted: Vec3 anti_geometric Vec3 = 0

// Vec3.dot(Vec3) -> S
impl Dot<Vec3> for Vec3 {
    type Output = S;
    fn dot(self, rhs: Vec3) -> Self::Output {
        // S(self.x.0 * rhs.x.0) + S(self.y.0 * rhs.y.0) + S(self.z.0 * rhs.z.0)
        self.x.dot(rhs.x) + self.y.dot(rhs.y) + self.z.dot(rhs.z)
    }
}

// Vec3.wedge(Vec3) -> Line3
impl Wedge<Vec3> for Vec3 {
    type Output = Line3;
    fn wedge(self, rhs: Vec3) -> Self::Output {
        // Line3 {
        //     vx: Default::default(),
        //     vy: Default::default(),
        //     vz: Default::default(),
        //     mx: YZ(self.y.0 * rhs.z.0) + YZ(self.z.0 * rhs.y.0),
        //     my: ZX(self.x.0 * rhs.z.0) + ZX(self.z.0 * rhs.x.0),
        //     mz: XY(self.x.0 * rhs.y.0) + XY(self.y.0 * rhs.x.0),
        // }
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

// Omitted: Vec3 geometric Vec4 = self.x * rhs.w + self.x * rhs.x + self.x * rhs.y + self.x * rhs.z + self.y * rhs.w + self.y * rhs.x + self.y * rhs.y + self.y * rhs.z + self.z * rhs.w + self.z * rhs.x + self.z * rhs.y + self.z * rhs.z

// Vec3.anti_geometric(Vec4) -> Line3
impl AntiGeometric<Vec4> for Vec3 {
    type Output = Line3;
    fn anti_geometric(self, rhs: Vec4) -> Self::Output {
        // Line3 {
        //     vx: Default::default(),
        //     vy: Default::default(),
        //     vz: Default::default(),
        //     mx: YZ(self.x.0 * rhs.w.0),
        //     my: ZX(self.y.0 * rhs.w.0),
        //     mz: XY(self.z.0 * rhs.w.0),
        // }
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
        // S(self.x.0 * rhs.x.0) + S(self.y.0 * rhs.y.0) + S(self.z.0 * rhs.z.0)
        self.x.dot(rhs.x) + self.y.dot(rhs.y) + self.z.dot(rhs.z)
    }
}

// Vec3.wedge(Vec4) -> Line3
impl Wedge<Vec4> for Vec3 {
    type Output = Line3;
    fn wedge(self, rhs: Vec4) -> Self::Output {
        // Line3 {
        //     vx: WX(self.x.0 * rhs.w.0),
        //     vy: WY(self.y.0 * rhs.w.0),
        //     vz: WZ(self.z.0 * rhs.w.0),
        //     mx: YZ(self.y.0 * rhs.z.0) + YZ(self.z.0 * rhs.y.0),
        //     my: ZX(self.x.0 * rhs.z.0) + ZX(self.z.0 * rhs.x.0),
        //     mz: XY(self.x.0 * rhs.y.0) + XY(self.y.0 * rhs.x.0),
        // }
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

// Omitted: Vec3 geometric Line3 = self.x * rhs.mx + self.x * rhs.my + self.x * rhs.mz + self.x * rhs.vx + self.x * rhs.vy + self.x * rhs.vz + self.y * rhs.mx + self.y * rhs.my + self.y * rhs.mz + self.y * rhs.vx + self.y * rhs.vy + self.y * rhs.vz + self.z * rhs.mx + self.z * rhs.my + self.z * rhs.mz + self.z * rhs.vx + self.z * rhs.vy + self.z * rhs.vz
// Omitted: Vec3 anti_geometric Line3 = self.x !* rhs.vx + self.x !* rhs.vy + self.x !* rhs.vz + self.y !* rhs.vx + self.y !* rhs.vy + self.y !* rhs.vz + self.z !* rhs.vx + self.z !* rhs.vy + self.z !* rhs.vz

// Vec3.dot(Line3) -> Vec4
impl Dot<Line3> for Vec3 {
    type Output = Vec4;
    fn dot(self, rhs: Line3) -> Self::Output {
        // Vec4 {
        //     x: X(self.y.0 * rhs.mz.0) + X(self.z.0 * rhs.my.0),
        //     y: Y(self.x.0 * rhs.mz.0) + Y(self.z.0 * rhs.mx.0),
        //     z: Z(self.x.0 * rhs.my.0) + Z(self.y.0 * rhs.mx.0),
        //     w: W(self.x.0 * rhs.vx.0) + W(self.y.0 * rhs.vy.0) + W(self.z.0 * rhs.vz.0),
        // }
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
        // Plane {
        //     nx: YZW(self.y.0 * rhs.vz.0) + YZW(self.z.0 * rhs.vy.0),
        //     ny: ZXW(self.x.0 * rhs.vz.0) + ZXW(self.z.0 * rhs.vx.0),
        //     nz: XYW(self.x.0 * rhs.vy.0) + XYW(self.y.0 * rhs.vx.0),
        //     d : ZYX(self.x.0 * rhs.mx.0) + ZYX(self.y.0 * rhs.my.0) + ZYX(self.z.0 * rhs.mz.0),
        // }
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

// Omitted: Vec3 geometric Plane = self.x * rhs.d + self.x * rhs.nx + self.x * rhs.ny + self.x * rhs.nz + self.y * rhs.d + self.y * rhs.nx + self.y * rhs.ny + self.y * rhs.nz + self.z * rhs.d + self.z * rhs.nx + self.z * rhs.ny + self.z * rhs.nz
// Omitted: Vec3 anti_geometric Plane = self.x !* rhs.nx + self.x !* rhs.ny + self.x !* rhs.nz + self.y !* rhs.nx + self.y !* rhs.ny + self.y !* rhs.nz + self.z !* rhs.nx + self.z !* rhs.ny + self.z !* rhs.nz

// Vec3.dot(Plane) -> Line3
impl Dot<Plane> for Vec3 {
    type Output = Line3;
    fn dot(self, rhs: Plane) -> Self::Output {
        // Line3 {
        //     vx: WX(self.y.0 * rhs.nz.0) + WX(self.z.0 * rhs.ny.0),
        //     vy: WY(self.x.0 * rhs.nz.0) + WY(self.z.0 * rhs.nx.0),
        //     vz: WZ(self.x.0 * rhs.ny.0) + WZ(self.y.0 * rhs.nx.0),
        //     mx: YZ(self.x.0 * rhs.d.0),
        //     my: ZX(self.y.0 * rhs.d.0),
        //     mz: XY(self.z.0 * rhs.d.0),
        // }
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
        // XYZW(self.x.0 * rhs.nx.0) + XYZW(self.y.0 * rhs.ny.0) + XYZW(self.z.0 * rhs.nz.0)
        self.x.wedge(rhs.nx) + self.y.wedge(rhs.ny) + self.z.wedge(rhs.nz)
    }
}

// Vec3.anti_wedge(Plane) -> S
impl AntiWedge<Plane> for Vec3 {
    type Output = S;
    fn anti_wedge(self, rhs: Plane) -> Self::Output {
        // S(self.x.0 * rhs.nx.0) + S(self.y.0 * rhs.ny.0) + S(self.z.0 * rhs.nz.0)
        self.x.anti_wedge(rhs.nx) + self.y.anti_wedge(rhs.ny) + self.z.anti_wedge(rhs.nz)
    }
}

// ---------------------------------------------------------------------
// Vec3 OP Translator3:

// Omitted: Vec3 geometric Translator3 = self.x * rhs.x + self.x * rhs.xyzw + self.x * rhs.y + self.x * rhs.z + self.y * rhs.x + self.y * rhs.xyzw + self.y * rhs.y + self.y * rhs.z + self.z * rhs.x + self.z * rhs.xyzw + self.z * rhs.y + self.z * rhs.z

// Vec3.anti_geometric(Translator3) -> Vec3
impl AntiGeometric<Translator3> for Vec3 {
    type Output = Vec3;
    fn anti_geometric(self, rhs: Translator3) -> Self::Output {
        // Vec3 {
        //     x: X(self.x.0 * rhs.xyzw.0),
        //     y: Y(self.y.0 * rhs.xyzw.0),
        //     z: Z(self.z.0 * rhs.xyzw.0),
        // }
        Vec3 {
            x: self.x.anti_geometric(rhs.xyzw),
            y: self.y.anti_geometric(rhs.xyzw),
            z: self.z.anti_geometric(rhs.xyzw),
        }
    }
}

// Omitted: Vec3 dot Translator3 = self.x | rhs.xyzw + self.x | rhs.y + self.x | rhs.z + self.y | rhs.x + self.y | rhs.xyzw + self.y | rhs.z + self.z | rhs.x + self.z | rhs.xyzw + self.z | rhs.y

// Vec3.wedge(Translator3) -> ZYX
impl Wedge<Translator3> for Vec3 {
    type Output = ZYX;
    fn wedge(self, rhs: Translator3) -> Self::Output {
        // -ZYX(self.x.0 * rhs.x.0) - ZYX(self.y.0 * rhs.y.0) - ZYX(self.z.0 * rhs.z.0)
        self.x.wedge(rhs.x) + self.y.wedge(rhs.y) + self.z.wedge(rhs.z)
    }
}

// Vec3.anti_wedge(Translator3) -> Vec3
impl AntiWedge<Translator3> for Vec3 {
    type Output = Vec3;
    fn anti_wedge(self, rhs: Translator3) -> Self::Output {
        // Vec3 {
        //     x: X(self.x.0 * rhs.xyzw.0),
        //     y: Y(self.y.0 * rhs.xyzw.0),
        //     z: Z(self.z.0 * rhs.xyzw.0),
        // }
        Vec3 {
            x: self.x.anti_wedge(rhs.xyzw),
            y: self.y.anti_wedge(rhs.xyzw),
            z: self.z.anti_wedge(rhs.xyzw),
        }
    }
}

// ---------------------------------------------------------------------
// Vec3 OP Rotor3:

// Omitted: Vec3 geometric Rotor3 = self.x * rhs.w + self.x * rhs.x + self.x * rhs.y + self.x * rhs.z + self.y * rhs.w + self.y * rhs.x + self.y * rhs.y + self.y * rhs.z + self.z * rhs.w + self.z * rhs.x + self.z * rhs.y + self.z * rhs.z
// Omitted: Vec3 anti_geometric Rotor3 = self.x !* rhs.w + self.x !* rhs.x + self.x !* rhs.y + self.x !* rhs.z + self.y !* rhs.w + self.y !* rhs.x + self.y !* rhs.y + self.y !* rhs.z + self.z !* rhs.w + self.z !* rhs.x + self.z !* rhs.y + self.z !* rhs.z
// Omitted: Vec3 dot Rotor3 = self.x | rhs.w + self.x | rhs.x + self.y | rhs.w + self.y | rhs.y + self.z | rhs.w + self.z | rhs.z

// Vec3.wedge(Rotor3) -> Plane
impl Wedge<Rotor3> for Vec3 {
    type Output = Plane;
    fn wedge(self, rhs: Rotor3) -> Self::Output {
        // Plane {
        //     nx: YZW(self.y.0 * rhs.z.0) + YZW(self.z.0 * rhs.y.0),
        //     ny: ZXW(self.x.0 * rhs.z.0) + ZXW(self.z.0 * rhs.x.0),
        //     nz: XYW(self.x.0 * rhs.y.0) + XYW(self.y.0 * rhs.x.0),
        //     d : Default::default(),
        // }
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
        // Vec3 {
        //     x: X(self.x.0 * rhs.w.0),
        //     y: Y(self.y.0 * rhs.w.0),
        //     z: Z(self.z.0 * rhs.w.0),
        // }
        Vec3 {
            x: self.x.anti_wedge(rhs.w),
            y: self.y.anti_wedge(rhs.w),
            z: self.z.anti_wedge(rhs.w),
        }
    }
}

// ---------------------------------------------------------------------
// Vec3 OP Motor3:

// Omitted: Vec3 geometric Motor3 = self.x * rhs.rw + self.x * rhs.rx + self.x * rhs.ry + self.x * rhs.rz + self.x * rhs.uw + self.x * rhs.ux + self.x * rhs.uy + self.x * rhs.uz + self.y * rhs.rw + self.y * rhs.rx + self.y * rhs.ry + self.y * rhs.rz + self.y * rhs.uw + self.y * rhs.ux + self.y * rhs.uy + self.y * rhs.uz + self.z * rhs.rw + self.z * rhs.rx + self.z * rhs.ry + self.z * rhs.rz + self.z * rhs.uw + self.z * rhs.ux + self.z * rhs.uy + self.z * rhs.uz
// Omitted: Vec3 anti_geometric Motor3 = self.x !* rhs.rw + self.x !* rhs.rx + self.x !* rhs.ry + self.x !* rhs.rz + self.x !* rhs.ux + self.x !* rhs.uy + self.x !* rhs.uz + self.y !* rhs.rw + self.y !* rhs.rx + self.y !* rhs.ry + self.y !* rhs.rz + self.y !* rhs.ux + self.y !* rhs.uy + self.y !* rhs.uz + self.z !* rhs.rw + self.z !* rhs.rx + self.z !* rhs.ry + self.z !* rhs.rz + self.z !* rhs.ux + self.z !* rhs.uy + self.z !* rhs.uz
// Omitted: Vec3 dot Motor3 = self.x | rhs.rw + self.x | rhs.rx + self.x | rhs.uw + self.x | rhs.uy + self.x | rhs.uz + self.y | rhs.rw + self.y | rhs.ry + self.y | rhs.uw + self.y | rhs.ux + self.y | rhs.uz + self.z | rhs.rw + self.z | rhs.rz + self.z | rhs.uw + self.z | rhs.ux + self.z | rhs.uy
// Omitted: Vec3 wedge Motor3 = self.x ^ rhs.ry + self.x ^ rhs.rz + self.x ^ rhs.uw + self.x ^ rhs.ux + self.y ^ rhs.rx + self.y ^ rhs.rz + self.y ^ rhs.uw + self.y ^ rhs.uy + self.z ^ rhs.rx + self.z ^ rhs.ry + self.z ^ rhs.uw + self.z ^ rhs.uz
// Omitted: Vec3 anti_wedge Motor3 = self.x & rhs.rw + self.x & rhs.ux + self.y & rhs.rw + self.y & rhs.uy + self.z & rhs.rw + self.z & rhs.uz

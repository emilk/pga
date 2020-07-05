//! # Vec4
//!
//! ## Operations
//! ```text
//! Vec4.dot(Vec4) -> S
//! Vec4.wedge(Vec4) -> Line3
//! Vec4.anti_geometric(Vec3) -> Line3
//! Vec3.anti_geometric(Vec4) -> Line3
//! Vec4.dot(Vec3) -> S
//! Vec3.dot(Vec4) -> S
//! Vec4.wedge(Vec3) -> Line3
//! Vec3.wedge(Vec4) -> Line3
//! Vec4.dot(Line3) -> Vec4
//! Line3.dot(Vec4) -> Vec4
//! Vec4.wedge(Line3) -> Plane
//! Line3.wedge(Vec4) -> Plane
//! Vec4.dot(Plane) -> Line3
//! Plane.dot(Vec4) -> Line3
//! Vec4.wedge(Plane) -> XYZW
//! Plane.wedge(Vec4) -> XYZW
//! Vec4.anti_wedge(Plane) -> S
//! Plane.anti_wedge(Vec4) -> S
//! Vec4.wedge(Rotor3) -> Plane
//! Rotor3.wedge(Vec4) -> Plane
//! Vec4.anti_wedge(Rotor3) -> Vec4
//! Rotor3.anti_wedge(Vec4) -> Vec4
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
pub struct Vec4 {
    pub x: X,
    pub y: Y,
    pub z: Z,
    pub w: W,
}

// ---------------------------------------------------------------------

impl RCompl for Vec4 {
    type Output = Plane;
    fn rcompl(self) -> Self::Output {
        Plane {
            nx: self.x.rcompl(),
            ny: self.y.rcompl(),
            nz: self.z.rcompl(),
            d: self.w.rcompl(),
        }
    }
}

impl LCompl for Vec4 {
    type Output = Plane;
    fn lcompl(self) -> Self::Output {
        Plane {
            nx: -self.x.lcompl(),
            ny: -self.y.lcompl(),
            nz: -self.z.lcompl(),
            d: -self.w.lcompl(),
        }
    }
}

impl Reverse for Vec4 {
    fn rev(self) -> Self {
        Vec4 {
            x: self.x,
            y: self.y,
            z: self.z,
            w: self.w,
        }
    }
}

impl AntiReverse for Vec4 {
    fn arev(self) -> Self {
        Vec4 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

// ---------------------------------------------------------------------
// Vec4 OP Vec3:

// Omitted: Vec4 geometric Vec3 = self.w * rhs.x + self.w * rhs.y + self.w * rhs.z + self.x * rhs.x + self.x * rhs.y + self.x * rhs.z + self.y * rhs.x + self.y * rhs.y + self.y * rhs.z + self.z * rhs.x + self.z * rhs.y + self.z * rhs.z  (unnamed type)
// Omitted: Vec4 anti_geometric Vec3 = Line3 {     vx: 0,     vy: 0,     vz: 0,     mx: self.w !* rhs.x,     my: self.w !* rhs.y,     mz: self.w !* rhs.z, }  (too many zeros)

// Vec4.dot(Vec3) -> S
impl Dot<Vec3> for Vec4 {
    type Output = S;
    fn dot(self, rhs: Vec3) -> Self::Output {
        // S(self.x.0 * rhs.x.0) + S(self.y.0 * rhs.y.0) + S(self.z.0 * rhs.z.0)
        self.x.dot(rhs.x) + self.y.dot(rhs.y) + self.z.dot(rhs.z)
    }
}

// Vec4.wedge(Vec3) -> Line3
impl Wedge<Vec3> for Vec4 {
    type Output = Line3;
    fn wedge(self, rhs: Vec3) -> Self::Output {
        // Line3 {
        //     vx: WX(self.w.0 * rhs.x.0),
        //     vy: WY(self.w.0 * rhs.y.0),
        //     vz: WZ(self.w.0 * rhs.z.0),
        //     mx: YZ(self.y.0 * rhs.z.0) + YZ(self.z.0 * rhs.y.0),
        //     my: ZX(self.x.0 * rhs.z.0) + ZX(self.z.0 * rhs.x.0),
        //     mz: XY(self.x.0 * rhs.y.0) + XY(self.y.0 * rhs.x.0),
        // }
        Line3 {
            vx: self.w.wedge(rhs.x),
            vy: self.w.wedge(rhs.y),
            vz: self.w.wedge(rhs.z),
            mx: self.y.wedge(rhs.z) - self.z.wedge(rhs.y),
            my: -self.x.wedge(rhs.z) + self.z.wedge(rhs.x),
            mz: self.x.wedge(rhs.y) - self.y.wedge(rhs.x),
        }
    }
}

// Omitted: Vec4 anti_wedge Vec3 = 0  (unnamed type)

// ---------------------------------------------------------------------
// Vec4 OP Vec4:

// Omitted: Vec4 geometric Vec4 = self.w * rhs.x + self.w * rhs.y + self.w * rhs.z + self.x * rhs.w + self.x * rhs.x + self.x * rhs.y + self.x * rhs.z + self.y * rhs.w + self.y * rhs.x + self.y * rhs.y + self.y * rhs.z + self.z * rhs.w + self.z * rhs.x + self.z * rhs.y + self.z * rhs.z  (unnamed type)
// Omitted: Vec4 anti_geometric Vec4 = self.w !* rhs.w + self.w !* rhs.x + self.w !* rhs.y + self.w !* rhs.z + self.x !* rhs.w + self.y !* rhs.w + self.z !* rhs.w  (unnamed type)

// Vec4.dot(Vec4) -> S
impl Dot<Vec4> for Vec4 {
    type Output = S;
    fn dot(self, rhs: Vec4) -> Self::Output {
        // S(self.x.0 * rhs.x.0) + S(self.y.0 * rhs.y.0) + S(self.z.0 * rhs.z.0)
        self.x.dot(rhs.x) + self.y.dot(rhs.y) + self.z.dot(rhs.z)
    }
}

// Vec4.wedge(Vec4) -> Line3
impl Wedge<Vec4> for Vec4 {
    type Output = Line3;
    fn wedge(self, rhs: Vec4) -> Self::Output {
        // Line3 {
        //     vx: WX(self.w.0 * rhs.x.0) + WX(self.x.0 * rhs.w.0),
        //     vy: WY(self.w.0 * rhs.y.0) + WY(self.y.0 * rhs.w.0),
        //     vz: WZ(self.w.0 * rhs.z.0) + WZ(self.z.0 * rhs.w.0),
        //     mx: YZ(self.y.0 * rhs.z.0) + YZ(self.z.0 * rhs.y.0),
        //     my: ZX(self.x.0 * rhs.z.0) + ZX(self.z.0 * rhs.x.0),
        //     mz: XY(self.x.0 * rhs.y.0) + XY(self.y.0 * rhs.x.0),
        // }
        Line3 {
            vx: self.w.wedge(rhs.x) - self.x.wedge(rhs.w),
            vy: self.w.wedge(rhs.y) - self.y.wedge(rhs.w),
            vz: self.w.wedge(rhs.z) - self.z.wedge(rhs.w),
            mx: self.y.wedge(rhs.z) - self.z.wedge(rhs.y),
            my: -self.x.wedge(rhs.z) + self.z.wedge(rhs.x),
            mz: self.x.wedge(rhs.y) - self.y.wedge(rhs.x),
        }
    }
}

// Omitted: Vec4 anti_wedge Vec4 = 0  (unnamed type)

// ---------------------------------------------------------------------
// Vec4 OP Line3:

// Omitted: Vec4 geometric Line3 = self.w * rhs.mx + self.w * rhs.my + self.w * rhs.mz + self.x * rhs.mx + self.x * rhs.my + self.x * rhs.mz + self.x * rhs.vx + self.x * rhs.vy + self.x * rhs.vz + self.y * rhs.mx + self.y * rhs.my + self.y * rhs.mz + self.y * rhs.vx + self.y * rhs.vy + self.y * rhs.vz + self.z * rhs.mx + self.z * rhs.my + self.z * rhs.mz + self.z * rhs.vx + self.z * rhs.vy + self.z * rhs.vz  (unnamed type)
// Omitted: Vec4 anti_geometric Line3 = self.w !* rhs.mx + self.w !* rhs.my + self.w !* rhs.mz + self.w !* rhs.vx + self.w !* rhs.vy + self.w !* rhs.vz + self.x !* rhs.vx + self.x !* rhs.vy + self.x !* rhs.vz + self.y !* rhs.vx + self.y !* rhs.vy + self.y !* rhs.vz + self.z !* rhs.vx + self.z !* rhs.vy + self.z !* rhs.vz  (unnamed type)

// Vec4.dot(Line3) -> Vec4
impl Dot<Line3> for Vec4 {
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

// Vec4.wedge(Line3) -> Plane
impl Wedge<Line3> for Vec4 {
    type Output = Plane;
    fn wedge(self, rhs: Line3) -> Self::Output {
        // Plane {
        //     nx: YZW(self.w.0 * rhs.mx.0) + YZW(self.y.0 * rhs.vz.0) + YZW(self.z.0 * rhs.vy.0),
        //     ny: ZXW(self.w.0 * rhs.my.0) + ZXW(self.x.0 * rhs.vz.0) + ZXW(self.z.0 * rhs.vx.0),
        //     nz: XYW(self.w.0 * rhs.mz.0) + XYW(self.x.0 * rhs.vy.0) + XYW(self.y.0 * rhs.vx.0),
        //     d : ZYX(self.x.0 * rhs.mx.0) + ZYX(self.y.0 * rhs.my.0) + ZYX(self.z.0 * rhs.mz.0),
        // }
        Plane {
            nx: self.w.wedge(rhs.mx) - self.y.wedge(rhs.vz) + self.z.wedge(rhs.vy),
            ny: self.w.wedge(rhs.my) + self.x.wedge(rhs.vz) - self.z.wedge(rhs.vx),
            nz: self.w.wedge(rhs.mz) - self.x.wedge(rhs.vy) + self.y.wedge(rhs.vx),
            d: -self.x.wedge(rhs.mx) - self.y.wedge(rhs.my) - self.z.wedge(rhs.mz),
        }
    }
}

// Omitted: Vec4 anti_wedge Line3 = 0  (unnamed type)

// ---------------------------------------------------------------------
// Vec4 OP Plane:

// Omitted: Vec4 geometric Plane = self.w * rhs.d + self.x * rhs.d + self.x * rhs.nx + self.x * rhs.ny + self.x * rhs.nz + self.y * rhs.d + self.y * rhs.nx + self.y * rhs.ny + self.y * rhs.nz + self.z * rhs.d + self.z * rhs.nx + self.z * rhs.ny + self.z * rhs.nz  (unnamed type)
// Omitted: Vec4 anti_geometric Plane = self.w !* rhs.d + self.w !* rhs.nx + self.w !* rhs.ny + self.w !* rhs.nz + self.x !* rhs.nx + self.x !* rhs.ny + self.x !* rhs.nz + self.y !* rhs.nx + self.y !* rhs.ny + self.y !* rhs.nz + self.z !* rhs.nx + self.z !* rhs.ny + self.z !* rhs.nz  (unnamed type)

// Vec4.dot(Plane) -> Line3
impl Dot<Plane> for Vec4 {
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

// Vec4.wedge(Plane) -> XYZW
impl Wedge<Plane> for Vec4 {
    type Output = XYZW;
    fn wedge(self, rhs: Plane) -> Self::Output {
        // XYZW(self.w.0 * rhs.d.0) + XYZW(self.x.0 * rhs.nx.0) + XYZW(self.y.0 * rhs.ny.0) + XYZW(self.z.0 * rhs.nz.0)
        self.w.wedge(rhs.d) + self.x.wedge(rhs.nx) + self.y.wedge(rhs.ny) + self.z.wedge(rhs.nz)
    }
}

// Vec4.anti_wedge(Plane) -> S
impl AntiWedge<Plane> for Vec4 {
    type Output = S;
    fn anti_wedge(self, rhs: Plane) -> Self::Output {
        // S(self.w.0 * rhs.d.0) + S(self.x.0 * rhs.nx.0) + S(self.y.0 * rhs.ny.0) + S(self.z.0 * rhs.nz.0)
        self.w.anti_wedge(rhs.d)
            + self.x.anti_wedge(rhs.nx)
            + self.y.anti_wedge(rhs.ny)
            + self.z.anti_wedge(rhs.nz)
    }
}

// ---------------------------------------------------------------------
// Vec4 OP Rotor3:

// Omitted: Vec4 geometric Rotor3 = self.x * rhs.w + self.x * rhs.x + self.x * rhs.y + self.x * rhs.z + self.y * rhs.w + self.y * rhs.x + self.y * rhs.y + self.y * rhs.z + self.z * rhs.w + self.z * rhs.x + self.z * rhs.y + self.z * rhs.z  (unnamed type)
// Omitted: Vec4 anti_geometric Rotor3 = self.w !* rhs.w + self.w !* rhs.x + self.w !* rhs.y + self.w !* rhs.z + self.x !* rhs.w + self.x !* rhs.x + self.x !* rhs.y + self.x !* rhs.z + self.y !* rhs.w + self.y !* rhs.x + self.y !* rhs.y + self.y !* rhs.z + self.z !* rhs.w + self.z !* rhs.x + self.z !* rhs.y + self.z !* rhs.z  (unnamed type)
// Omitted: Vec4 dot Rotor3 = self.x | rhs.w + self.x | rhs.x + self.y | rhs.w + self.y | rhs.y + self.z | rhs.w + self.z | rhs.z  (unnamed type)

// Vec4.wedge(Rotor3) -> Plane
impl Wedge<Rotor3> for Vec4 {
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

// Vec4.anti_wedge(Rotor3) -> Vec4
impl AntiWedge<Rotor3> for Vec4 {
    type Output = Vec4;
    fn anti_wedge(self, rhs: Rotor3) -> Self::Output {
        // Vec4 {
        //     x: X(self.x.0 * rhs.w.0),
        //     y: Y(self.y.0 * rhs.w.0),
        //     z: Z(self.z.0 * rhs.w.0),
        //     w: W(self.w.0 * rhs.w.0),
        // }
        Vec4 {
            x: self.x.anti_wedge(rhs.w),
            y: self.y.anti_wedge(rhs.w),
            z: self.z.anti_wedge(rhs.w),
            w: self.w.anti_wedge(rhs.w),
        }
    }
}

// ---------------------------------------------------------------------
// Vec4 OP Motor3:

// Omitted: Vec4 geometric Motor3 = self.w * rhs.uw + self.x * rhs.rw + self.x * rhs.rx + self.x * rhs.ry + self.x * rhs.rz + self.x * rhs.uw + self.x * rhs.ux + self.x * rhs.uy + self.x * rhs.uz + self.y * rhs.rw + self.y * rhs.rx + self.y * rhs.ry + self.y * rhs.rz + self.y * rhs.uw + self.y * rhs.ux + self.y * rhs.uy + self.y * rhs.uz + self.z * rhs.rw + self.z * rhs.rx + self.z * rhs.ry + self.z * rhs.rz + self.z * rhs.uw + self.z * rhs.ux + self.z * rhs.uy + self.z * rhs.uz  (unnamed type)
// Omitted: Vec4 anti_geometric Motor3 = self.w !* rhs.rw + self.w !* rhs.rx + self.w !* rhs.ry + self.w !* rhs.rz + self.w !* rhs.uw + self.w !* rhs.ux + self.w !* rhs.uy + self.w !* rhs.uz + self.x !* rhs.rw + self.x !* rhs.rx + self.x !* rhs.ry + self.x !* rhs.rz + self.x !* rhs.ux + self.x !* rhs.uy + self.x !* rhs.uz + self.y !* rhs.rw + self.y !* rhs.rx + self.y !* rhs.ry + self.y !* rhs.rz + self.y !* rhs.ux + self.y !* rhs.uy + self.y !* rhs.uz + self.z !* rhs.rw + self.z !* rhs.rx + self.z !* rhs.ry + self.z !* rhs.rz + self.z !* rhs.ux + self.z !* rhs.uy + self.z !* rhs.uz  (unnamed type)
// Omitted: Vec4 dot Motor3 = self.w | rhs.uw + self.x | rhs.rw + self.x | rhs.rx + self.x | rhs.uw + self.x | rhs.uy + self.x | rhs.uz + self.y | rhs.rw + self.y | rhs.ry + self.y | rhs.uw + self.y | rhs.ux + self.y | rhs.uz + self.z | rhs.rw + self.z | rhs.rz + self.z | rhs.uw + self.z | rhs.ux + self.z | rhs.uy  (unnamed type)
// Omitted: Vec4 wedge Motor3 = self.w ^ rhs.uw + self.x ^ rhs.ry + self.x ^ rhs.rz + self.x ^ rhs.uw + self.x ^ rhs.ux + self.y ^ rhs.rx + self.y ^ rhs.rz + self.y ^ rhs.uw + self.y ^ rhs.uy + self.z ^ rhs.rx + self.z ^ rhs.ry + self.z ^ rhs.uw + self.z ^ rhs.uz  (unnamed type)
// Omitted: Vec4 anti_wedge Motor3 = self.w & rhs.rw + self.x & rhs.rw + self.x & rhs.ux + self.y & rhs.rw + self.y & rhs.uy + self.z & rhs.rw + self.z & rhs.uz  (unnamed type)

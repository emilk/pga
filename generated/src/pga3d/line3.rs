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

// Omitted: Line3 geometric Vec3 = self.mx * rhs.x + self.mx * rhs.y + self.mx * rhs.z + self.my * rhs.x + self.my * rhs.y + self.my * rhs.z + self.mz * rhs.x + self.mz * rhs.y + self.mz * rhs.z + self.vx * rhs.x + self.vx * rhs.y + self.vx * rhs.z + self.vy * rhs.x + self.vy * rhs.y + self.vy * rhs.z + self.vz * rhs.x + self.vz * rhs.y + self.vz * rhs.z  (unnamed type)
// Omitted: Line3 anti_geometric Vec3 = self.vx !* rhs.x + self.vx !* rhs.y + self.vx !* rhs.z + self.vy !* rhs.x + self.vy !* rhs.y + self.vy !* rhs.z + self.vz !* rhs.x + self.vz !* rhs.y + self.vz !* rhs.z  (unnamed type)

// Line3.dot(Vec3) -> Vec4
impl Dot<Vec3> for Line3 {
    type Output = Vec4;
    fn dot(self, rhs: Vec3) -> Self::Output {
        // Vec4 {
        //     x: X(self.my.0 * rhs.z.0) + X(self.mz.0 * rhs.y.0),
        //     y: Y(self.mx.0 * rhs.z.0) + Y(self.mz.0 * rhs.x.0),
        //     z: Z(self.mx.0 * rhs.y.0) + Z(self.my.0 * rhs.x.0),
        //     w: W(self.vx.0 * rhs.x.0) + W(self.vy.0 * rhs.y.0) + W(self.vz.0 * rhs.z.0),
        // }
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
        // Plane {
        //     nx: YZW(self.vy.0 * rhs.z.0) + YZW(self.vz.0 * rhs.y.0),
        //     ny: ZXW(self.vx.0 * rhs.z.0) + ZXW(self.vz.0 * rhs.x.0),
        //     nz: XYW(self.vx.0 * rhs.y.0) + XYW(self.vy.0 * rhs.x.0),
        //     d : ZYX(self.mx.0 * rhs.x.0) + ZYX(self.my.0 * rhs.y.0) + ZYX(self.mz.0 * rhs.z.0),
        // }
        Plane {
            nx: self.vy.wedge(rhs.z) - self.vz.wedge(rhs.y),
            ny: -self.vx.wedge(rhs.z) + self.vz.wedge(rhs.x),
            nz: self.vx.wedge(rhs.y) - self.vy.wedge(rhs.x),
            d: -self.mx.wedge(rhs.x) - self.my.wedge(rhs.y) - self.mz.wedge(rhs.z),
        }
    }
}

// Omitted: Line3 anti_wedge Vec3 = 0  (unnamed type)

// ---------------------------------------------------------------------
// Line3 OP Vec4:

// Omitted: Line3 geometric Vec4 = self.mx * rhs.w + self.mx * rhs.x + self.mx * rhs.y + self.mx * rhs.z + self.my * rhs.w + self.my * rhs.x + self.my * rhs.y + self.my * rhs.z + self.mz * rhs.w + self.mz * rhs.x + self.mz * rhs.y + self.mz * rhs.z + self.vx * rhs.x + self.vx * rhs.y + self.vx * rhs.z + self.vy * rhs.x + self.vy * rhs.y + self.vy * rhs.z + self.vz * rhs.x + self.vz * rhs.y + self.vz * rhs.z  (unnamed type)
// Omitted: Line3 anti_geometric Vec4 = self.mx !* rhs.w + self.my !* rhs.w + self.mz !* rhs.w + self.vx !* rhs.w + self.vx !* rhs.x + self.vx !* rhs.y + self.vx !* rhs.z + self.vy !* rhs.w + self.vy !* rhs.x + self.vy !* rhs.y + self.vy !* rhs.z + self.vz !* rhs.w + self.vz !* rhs.x + self.vz !* rhs.y + self.vz !* rhs.z  (unnamed type)

// Line3.dot(Vec4) -> Vec4
impl Dot<Vec4> for Line3 {
    type Output = Vec4;
    fn dot(self, rhs: Vec4) -> Self::Output {
        // Vec4 {
        //     x: X(self.my.0 * rhs.z.0) + X(self.mz.0 * rhs.y.0),
        //     y: Y(self.mx.0 * rhs.z.0) + Y(self.mz.0 * rhs.x.0),
        //     z: Z(self.mx.0 * rhs.y.0) + Z(self.my.0 * rhs.x.0),
        //     w: W(self.vx.0 * rhs.x.0) + W(self.vy.0 * rhs.y.0) + W(self.vz.0 * rhs.z.0),
        // }
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
        // Plane {
        //     nx: YZW(self.mx.0 * rhs.w.0) + YZW(self.vy.0 * rhs.z.0) + YZW(self.vz.0 * rhs.y.0),
        //     ny: ZXW(self.my.0 * rhs.w.0) + ZXW(self.vx.0 * rhs.z.0) + ZXW(self.vz.0 * rhs.x.0),
        //     nz: XYW(self.mz.0 * rhs.w.0) + XYW(self.vx.0 * rhs.y.0) + XYW(self.vy.0 * rhs.x.0),
        //     d : ZYX(self.mx.0 * rhs.x.0) + ZYX(self.my.0 * rhs.y.0) + ZYX(self.mz.0 * rhs.z.0),
        // }
        Plane {
            nx: self.mx.wedge(rhs.w) + self.vy.wedge(rhs.z) - self.vz.wedge(rhs.y),
            ny: self.my.wedge(rhs.w) - self.vx.wedge(rhs.z) + self.vz.wedge(rhs.x),
            nz: self.mz.wedge(rhs.w) + self.vx.wedge(rhs.y) - self.vy.wedge(rhs.x),
            d: -self.mx.wedge(rhs.x) - self.my.wedge(rhs.y) - self.mz.wedge(rhs.z),
        }
    }
}

// Omitted: Line3 anti_wedge Vec4 = 0  (unnamed type)

// ---------------------------------------------------------------------
// Line3 OP Line3:

// Omitted: Line3 geometric Line3 = self.mx * rhs.mx + self.mx * rhs.my + self.mx * rhs.mz + self.mx * rhs.vx + self.mx * rhs.vy + self.mx * rhs.vz + self.my * rhs.mx + self.my * rhs.my + self.my * rhs.mz + self.my * rhs.vx + self.my * rhs.vy + self.my * rhs.vz + self.mz * rhs.mx + self.mz * rhs.my + self.mz * rhs.mz + self.mz * rhs.vx + self.mz * rhs.vy + self.mz * rhs.vz + self.vx * rhs.mx + self.vx * rhs.my + self.vx * rhs.mz + self.vy * rhs.mx + self.vy * rhs.my + self.vy * rhs.mz + self.vz * rhs.mx + self.vz * rhs.my + self.vz * rhs.mz  (unnamed type)
// Omitted: Line3 anti_geometric Line3 = self.mx !* rhs.vx + self.mx !* rhs.vy + self.mx !* rhs.vz + self.my !* rhs.vx + self.my !* rhs.vy + self.my !* rhs.vz + self.mz !* rhs.vx + self.mz !* rhs.vy + self.mz !* rhs.vz + self.vx !* rhs.mx + self.vx !* rhs.my + self.vx !* rhs.mz + self.vx !* rhs.vx + self.vx !* rhs.vy + self.vx !* rhs.vz + self.vy !* rhs.mx + self.vy !* rhs.my + self.vy !* rhs.mz + self.vy !* rhs.vx + self.vy !* rhs.vy + self.vy !* rhs.vz + self.vz !* rhs.mx + self.vz !* rhs.my + self.vz !* rhs.mz + self.vz !* rhs.vx + self.vz !* rhs.vy + self.vz !* rhs.vz  (unnamed type)

// Line3.dot(Line3) -> S
impl Dot<Line3> for Line3 {
    type Output = S;
    fn dot(self, rhs: Line3) -> Self::Output {
        // -S(self.mx.0 * rhs.mx.0) - S(self.my.0 * rhs.my.0) - S(self.mz.0 * rhs.mz.0)
        self.mx.dot(rhs.mx) + self.my.dot(rhs.my) + self.mz.dot(rhs.mz)
    }
}

// Line3.wedge(Line3) -> XYZW
impl Wedge<Line3> for Line3 {
    type Output = XYZW;
    fn wedge(self, rhs: Line3) -> Self::Output {
        // -XYZW(self.mx.0 * rhs.vx.0) - XYZW(self.my.0 * rhs.vy.0) - XYZW(self.mz.0 * rhs.vz.0) - XYZW(self.vx.0 * rhs.mx.0) - XYZW(self.vy.0 * rhs.my.0) - XYZW(self.vz.0 * rhs.mz.0)
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
        // -S(self.mx.0 * rhs.vx.0) - S(self.my.0 * rhs.vy.0) - S(self.mz.0 * rhs.vz.0) - S(self.vx.0 * rhs.mx.0) - S(self.vy.0 * rhs.my.0) - S(self.vz.0 * rhs.mz.0)
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

// Omitted: Line3 geometric Plane = self.mx * rhs.d + self.mx * rhs.nx + self.mx * rhs.ny + self.mx * rhs.nz + self.my * rhs.d + self.my * rhs.nx + self.my * rhs.ny + self.my * rhs.nz + self.mz * rhs.d + self.mz * rhs.nx + self.mz * rhs.ny + self.mz * rhs.nz + self.vx * rhs.d + self.vy * rhs.d + self.vz * rhs.d  (unnamed type)
// Omitted: Line3 anti_geometric Plane = self.mx !* rhs.nx + self.mx !* rhs.ny + self.mx !* rhs.nz + self.my !* rhs.nx + self.my !* rhs.ny + self.my !* rhs.nz + self.mz !* rhs.nx + self.mz !* rhs.ny + self.mz !* rhs.nz + self.vx !* rhs.d + self.vx !* rhs.nx + self.vx !* rhs.ny + self.vx !* rhs.nz + self.vy !* rhs.d + self.vy !* rhs.nx + self.vy !* rhs.ny + self.vy !* rhs.nz + self.vz !* rhs.d + self.vz !* rhs.nx + self.vz !* rhs.ny + self.vz !* rhs.nz  (unnamed type)

// Line3.dot(Plane) -> Vec4
impl Dot<Plane> for Line3 {
    type Output = Vec4;
    fn dot(self, rhs: Plane) -> Self::Output {
        // Vec4 {
        //     x: X(self.mx.0 * rhs.d.0),
        //     y: Y(self.my.0 * rhs.d.0),
        //     z: Z(self.mz.0 * rhs.d.0),
        //     w: W(self.mx.0 * rhs.nx.0) + W(self.my.0 * rhs.ny.0) + W(self.mz.0 * rhs.nz.0),
        // }
        Vec4 {
            x: self.mx.dot(rhs.d),
            y: self.my.dot(rhs.d),
            z: self.mz.dot(rhs.d),
            w: -self.mx.dot(rhs.nx) - self.my.dot(rhs.ny) - self.mz.dot(rhs.nz),
        }
    }
}

// Omitted: Line3 wedge Plane = 0  (unnamed type)

// Line3.anti_wedge(Plane) -> Vec4
impl AntiWedge<Plane> for Line3 {
    type Output = Vec4;
    fn anti_wedge(self, rhs: Plane) -> Self::Output {
        // Vec4 {
        //     x: X(self.my.0 * rhs.nz.0) + X(self.mz.0 * rhs.ny.0) + X(self.vx.0 * rhs.d.0),
        //     y: Y(self.mx.0 * rhs.nz.0) + Y(self.mz.0 * rhs.nx.0) + Y(self.vy.0 * rhs.d.0),
        //     z: Z(self.mx.0 * rhs.ny.0) + Z(self.my.0 * rhs.nx.0) + Z(self.vz.0 * rhs.d.0),
        //     w: W(self.vx.0 * rhs.nx.0) + W(self.vy.0 * rhs.ny.0) + W(self.vz.0 * rhs.nz.0),
        // }
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
// Line3 OP Rotor3:

// Line3.geometric(Rotor3) -> Rotor3
impl Geometric<Rotor3> for Line3 {
    type Output = Rotor3;
    fn geometric(self, rhs: Rotor3) -> Self::Output {
        // Rotor3 {
        //     x: WX(self.mx.0 * rhs.w.0) + WX(self.my.0 * rhs.z.0) + WX(self.mz.0 * rhs.y.0),
        //     y: WY(self.mx.0 * rhs.z.0) + WY(self.my.0 * rhs.w.0) + WY(self.mz.0 * rhs.x.0),
        //     z: WZ(self.mx.0 * rhs.y.0) + WZ(self.my.0 * rhs.x.0) + WZ(self.mz.0 * rhs.w.0),
        //     w: XYZW(self.mx.0 * rhs.x.0) + XYZW(self.my.0 * rhs.y.0) + XYZW(self.mz.0 * rhs.z.0),
        // }
        Rotor3 {
            x: self.mx.geometric(rhs.w) - self.my.geometric(rhs.z) + self.mz.geometric(rhs.y),
            y: self.mx.geometric(rhs.z) + self.my.geometric(rhs.w) - self.mz.geometric(rhs.x),
            z: -self.mx.geometric(rhs.y) + self.my.geometric(rhs.x) + self.mz.geometric(rhs.w),
            w: -self.mx.geometric(rhs.x) - self.my.geometric(rhs.y) - self.mz.geometric(rhs.z),
        }
    }
}

// Omitted: Line3 anti_geometric Rotor3 = self.mx !* rhs.w + self.mx !* rhs.x + self.mx !* rhs.y + self.mx !* rhs.z + self.my !* rhs.w + self.my !* rhs.x + self.my !* rhs.y + self.my !* rhs.z + self.mz !* rhs.w + self.mz !* rhs.x + self.mz !* rhs.y + self.mz !* rhs.z + self.vx !* rhs.w + self.vx !* rhs.x + self.vx !* rhs.y + self.vx !* rhs.z + self.vy !* rhs.w + self.vy !* rhs.x + self.vy !* rhs.y + self.vy !* rhs.z + self.vz !* rhs.w + self.vz !* rhs.x + self.vz !* rhs.y + self.vz !* rhs.z  (unnamed type)
// Omitted: Line3 dot Rotor3 = Line3 {     vx: self.mx | rhs.w,     vy: self.my | rhs.w,     vz: self.mz | rhs.w,     mx: 0,     my: 0,     mz: 0, }  (too many zeros)

// Line3.wedge(Rotor3) -> XYZW
impl Wedge<Rotor3> for Line3 {
    type Output = XYZW;
    fn wedge(self, rhs: Rotor3) -> Self::Output {
        // -XYZW(self.mx.0 * rhs.x.0) - XYZW(self.my.0 * rhs.y.0) - XYZW(self.mz.0 * rhs.z.0)
        self.mx.wedge(rhs.x) + self.my.wedge(rhs.y) + self.mz.wedge(rhs.z)
    }
}

// Omitted: Line3 anti_wedge Rotor3 = self.mx & rhs.w + self.mx & rhs.x + self.my & rhs.w + self.my & rhs.y + self.mz & rhs.w + self.mz & rhs.z + self.vx & rhs.w + self.vy & rhs.w + self.vz & rhs.w  (unnamed type)

// ---------------------------------------------------------------------
// Line3 OP Motor3:

// Omitted: Line3 geometric Motor3 = self.mx * rhs.rw + self.mx * rhs.rx + self.mx * rhs.ry + self.mx * rhs.rz + self.mx * rhs.uw + self.mx * rhs.ux + self.mx * rhs.uy + self.mx * rhs.uz + self.my * rhs.rw + self.my * rhs.rx + self.my * rhs.ry + self.my * rhs.rz + self.my * rhs.uw + self.my * rhs.ux + self.my * rhs.uy + self.my * rhs.uz + self.mz * rhs.rw + self.mz * rhs.rx + self.mz * rhs.ry + self.mz * rhs.rz + self.mz * rhs.uw + self.mz * rhs.ux + self.mz * rhs.uy + self.mz * rhs.uz + self.vx * rhs.uw + self.vy * rhs.uw + self.vz * rhs.uw  (unnamed type)
// Omitted: Line3 anti_geometric Motor3 = self.mx !* rhs.rw + self.mx !* rhs.rx + self.mx !* rhs.ry + self.mx !* rhs.rz + self.mx !* rhs.ux + self.mx !* rhs.uy + self.mx !* rhs.uz + self.my !* rhs.rw + self.my !* rhs.rx + self.my !* rhs.ry + self.my !* rhs.rz + self.my !* rhs.ux + self.my !* rhs.uy + self.my !* rhs.uz + self.mz !* rhs.rw + self.mz !* rhs.rx + self.mz !* rhs.ry + self.mz !* rhs.rz + self.mz !* rhs.ux + self.mz !* rhs.uy + self.mz !* rhs.uz + self.vx !* rhs.rw + self.vx !* rhs.rx + self.vx !* rhs.ry + self.vx !* rhs.rz + self.vx !* rhs.uw + self.vx !* rhs.ux + self.vx !* rhs.uy + self.vx !* rhs.uz + self.vy !* rhs.rw + self.vy !* rhs.rx + self.vy !* rhs.ry + self.vy !* rhs.rz + self.vy !* rhs.uw + self.vy !* rhs.ux + self.vy !* rhs.uy + self.vy !* rhs.uz + self.vz !* rhs.rw + self.vz !* rhs.rx + self.vz !* rhs.ry + self.vz !* rhs.rz + self.vz !* rhs.uw + self.vz !* rhs.ux + self.vz !* rhs.uy + self.vz !* rhs.uz  (unnamed type)
// Omitted: Line3 dot Motor3 = self.mx | rhs.rw + self.mx | rhs.uw + self.mx | rhs.ux + self.my | rhs.rw + self.my | rhs.uw + self.my | rhs.uy + self.mz | rhs.rw + self.mz | rhs.uw + self.mz | rhs.uz + self.vx | rhs.uw + self.vy | rhs.uw + self.vz | rhs.uw  (unnamed type)
// Omitted: Line3 wedge Motor3 = self.mx ^ rhs.rx + self.mx ^ rhs.uw + self.my ^ rhs.ry + self.my ^ rhs.uw + self.mz ^ rhs.rz + self.mz ^ rhs.uw + self.vx ^ rhs.uw + self.vy ^ rhs.uw + self.vz ^ rhs.uw  (unnamed type)
// Omitted: Line3 anti_wedge Motor3 = self.mx & rhs.rw + self.mx & rhs.rx + self.mx & rhs.uy + self.mx & rhs.uz + self.my & rhs.rw + self.my & rhs.ry + self.my & rhs.ux + self.my & rhs.uz + self.mz & rhs.rw + self.mz & rhs.rz + self.mz & rhs.ux + self.mz & rhs.uy + self.vx & rhs.rw + self.vx & rhs.ux + self.vy & rhs.rw + self.vy & rhs.uy + self.vz & rhs.rw + self.vz & rhs.uz  (unnamed type)

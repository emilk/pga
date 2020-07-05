//! # Plane
//!
//! ## Operations
//! ```text
//! Plane.geometric(Plane) -> Motor3
//! Plane.dot(Plane) -> S
//! Plane.anti_wedge(Plane) -> Line3
//! Plane.dot(Vec3) -> Line3
//! Vec3.dot(Plane) -> Line3
//! Plane.wedge(Vec3) -> XYZW
//! Vec3.wedge(Plane) -> XYZW
//! Plane.anti_wedge(Vec3) -> S
//! Vec3.anti_wedge(Plane) -> S
//! Plane.dot(Vec4) -> Line3
//! Vec4.dot(Plane) -> Line3
//! Plane.wedge(Vec4) -> XYZW
//! Vec4.wedge(Plane) -> XYZW
//! Plane.anti_wedge(Vec4) -> S
//! Vec4.anti_wedge(Plane) -> S
//! Plane.dot(Line3) -> Vec4
//! Line3.dot(Plane) -> Vec4
//! Plane.anti_wedge(Line3) -> Vec4
//! Line3.anti_wedge(Plane) -> Vec4
//! Plane.dot(Translator3) -> Vec4
//! Translator3.dot(Plane) -> Vec4
//! Plane.dot(Rotor3) -> W
//! Rotor3.dot(Plane) -> W
//! Plane.wedge(Motor3) -> Plane
//! Motor3.wedge(Plane) -> Plane
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
pub struct Plane {
    pub nx: YZW,
    pub ny: ZXW,
    pub nz: XYW,
    pub d: ZYX,
}

// ---------------------------------------------------------------------

impl RCompl for Plane {
    type Output = Vec4;
    fn rcompl(self) -> Self::Output {
        Vec4 {
            x: -self.nx.rcompl(),
            y: -self.ny.rcompl(),
            z: -self.nz.rcompl(),
            w: -self.d.rcompl(),
        }
    }
}

impl LCompl for Plane {
    type Output = Vec4;
    fn lcompl(self) -> Self::Output {
        Vec4 {
            x: self.nx.lcompl(),
            y: self.ny.lcompl(),
            z: self.nz.lcompl(),
            w: self.d.lcompl(),
        }
    }
}

impl Reverse for Plane {
    fn rev(self) -> Self {
        Plane {
            nx: -self.nx,
            ny: -self.ny,
            nz: -self.nz,
            d: -self.d,
        }
    }
}

impl AntiReverse for Plane {
    fn arev(self) -> Self {
        Plane {
            nx: self.nx,
            ny: self.ny,
            nz: self.nz,
            d: self.d,
        }
    }
}

// ---------------------------------------------------------------------
// Plane OP Vec3:

// Omitted: Plane geometric Vec3 = self.d.geometric(rhs.x) + self.d.geometric(rhs.y) + self.d.geometric(rhs.z) + self.nx.geometric(rhs.x) + self.nx.geometric(rhs.y) + self.nx.geometric(rhs.z) + self.ny.geometric(rhs.x) + self.ny.geometric(rhs.y) + self.ny.geometric(rhs.z) + self.nz.geometric(rhs.x) + self.nz.geometric(rhs.y) + self.nz.geometric(rhs.z)
// Omitted: Plane anti_geometric Vec3 = self.nx.anti_geometric(rhs.x) + self.nx.anti_geometric(rhs.y) + self.nx.anti_geometric(rhs.z) + self.ny.anti_geometric(rhs.x) + self.ny.anti_geometric(rhs.y) + self.ny.anti_geometric(rhs.z) + self.nz.anti_geometric(rhs.x) + self.nz.anti_geometric(rhs.y) + self.nz.anti_geometric(rhs.z)

// Plane.dot(Vec3) -> Line3
impl Dot<Vec3> for Plane {
    type Output = Line3;
    fn dot(self, rhs: Vec3) -> Self::Output {
        Line3 {
            vx: -self.ny.dot(rhs.z) + self.nz.dot(rhs.y),
            vy: self.nx.dot(rhs.z) - self.nz.dot(rhs.x),
            vz: -self.nx.dot(rhs.y) + self.ny.dot(rhs.x),
            mx: -self.d.dot(rhs.x),
            my: -self.d.dot(rhs.y),
            mz: -self.d.dot(rhs.z),
        }
    }
}

// Plane.wedge(Vec3) -> XYZW
impl Wedge<Vec3> for Plane {
    type Output = XYZW;
    fn wedge(self, rhs: Vec3) -> Self::Output {
        self.nx.wedge(rhs.x) + self.ny.wedge(rhs.y) + self.nz.wedge(rhs.z)
    }
}

// Plane.anti_wedge(Vec3) -> S
impl AntiWedge<Vec3> for Plane {
    type Output = S;
    fn anti_wedge(self, rhs: Vec3) -> Self::Output {
        self.nx.anti_wedge(rhs.x) + self.ny.anti_wedge(rhs.y) + self.nz.anti_wedge(rhs.z)
    }
}

// ---------------------------------------------------------------------
// Plane OP Vec4:

// Omitted: Plane geometric Vec4 = self.d.geometric(rhs.w) + self.d.geometric(rhs.x) + self.d.geometric(rhs.y) + self.d.geometric(rhs.z) + self.nx.geometric(rhs.x) + self.nx.geometric(rhs.y) + self.nx.geometric(rhs.z) + self.ny.geometric(rhs.x) + self.ny.geometric(rhs.y) + self.ny.geometric(rhs.z) + self.nz.geometric(rhs.x) + self.nz.geometric(rhs.y) + self.nz.geometric(rhs.z)
// Omitted: Plane anti_geometric Vec4 = self.d.anti_geometric(rhs.w) + self.nx.anti_geometric(rhs.w) + self.nx.anti_geometric(rhs.x) + self.nx.anti_geometric(rhs.y) + self.nx.anti_geometric(rhs.z) + self.ny.anti_geometric(rhs.w) + self.ny.anti_geometric(rhs.x) + self.ny.anti_geometric(rhs.y) + self.ny.anti_geometric(rhs.z) + self.nz.anti_geometric(rhs.w) + self.nz.anti_geometric(rhs.x) + self.nz.anti_geometric(rhs.y) + self.nz.anti_geometric(rhs.z)

// Plane.dot(Vec4) -> Line3
impl Dot<Vec4> for Plane {
    type Output = Line3;
    fn dot(self, rhs: Vec4) -> Self::Output {
        Line3 {
            vx: -self.ny.dot(rhs.z) + self.nz.dot(rhs.y),
            vy: self.nx.dot(rhs.z) - self.nz.dot(rhs.x),
            vz: -self.nx.dot(rhs.y) + self.ny.dot(rhs.x),
            mx: -self.d.dot(rhs.x),
            my: -self.d.dot(rhs.y),
            mz: -self.d.dot(rhs.z),
        }
    }
}

// Plane.wedge(Vec4) -> XYZW
impl Wedge<Vec4> for Plane {
    type Output = XYZW;
    fn wedge(self, rhs: Vec4) -> Self::Output {
        self.d.wedge(rhs.w) + self.nx.wedge(rhs.x) + self.ny.wedge(rhs.y) + self.nz.wedge(rhs.z)
    }
}

// Plane.anti_wedge(Vec4) -> S
impl AntiWedge<Vec4> for Plane {
    type Output = S;
    fn anti_wedge(self, rhs: Vec4) -> Self::Output {
        self.d.anti_wedge(rhs.w)
            + self.nx.anti_wedge(rhs.x)
            + self.ny.anti_wedge(rhs.y)
            + self.nz.anti_wedge(rhs.z)
    }
}

// ---------------------------------------------------------------------
// Plane OP Line3:

// Omitted: Plane geometric Line3 = self.d.geometric(rhs.mx) + self.d.geometric(rhs.my) + self.d.geometric(rhs.mz) + self.d.geometric(rhs.vx) + self.d.geometric(rhs.vy) + self.d.geometric(rhs.vz) + self.nx.geometric(rhs.mx) + self.nx.geometric(rhs.my) + self.nx.geometric(rhs.mz) + self.ny.geometric(rhs.mx) + self.ny.geometric(rhs.my) + self.ny.geometric(rhs.mz) + self.nz.geometric(rhs.mx) + self.nz.geometric(rhs.my) + self.nz.geometric(rhs.mz)
// Omitted: Plane anti_geometric Line3 = self.d.anti_geometric(rhs.vx) + self.d.anti_geometric(rhs.vy) + self.d.anti_geometric(rhs.vz) + self.nx.anti_geometric(rhs.mx) + self.nx.anti_geometric(rhs.my) + self.nx.anti_geometric(rhs.mz) + self.nx.anti_geometric(rhs.vx) + self.nx.anti_geometric(rhs.vy) + self.nx.anti_geometric(rhs.vz) + self.ny.anti_geometric(rhs.mx) + self.ny.anti_geometric(rhs.my) + self.ny.anti_geometric(rhs.mz) + self.ny.anti_geometric(rhs.vx) + self.ny.anti_geometric(rhs.vy) + self.ny.anti_geometric(rhs.vz) + self.nz.anti_geometric(rhs.mx) + self.nz.anti_geometric(rhs.my) + self.nz.anti_geometric(rhs.mz) + self.nz.anti_geometric(rhs.vx) + self.nz.anti_geometric(rhs.vy) + self.nz.anti_geometric(rhs.vz)

// Plane.dot(Line3) -> Vec4
impl Dot<Line3> for Plane {
    type Output = Vec4;
    fn dot(self, rhs: Line3) -> Self::Output {
        Vec4 {
            x: self.d.dot(rhs.mx),
            y: self.d.dot(rhs.my),
            z: self.d.dot(rhs.mz),
            w: -self.nx.dot(rhs.mx) - self.ny.dot(rhs.my) - self.nz.dot(rhs.mz),
        }
    }
}

// Omitted: Plane wedge Line3 = 0

// Plane.anti_wedge(Line3) -> Vec4
impl AntiWedge<Line3> for Plane {
    type Output = Vec4;
    fn anti_wedge(self, rhs: Line3) -> Self::Output {
        Vec4 {
            x: self.d.anti_wedge(rhs.vx) - self.ny.anti_wedge(rhs.mz) + self.nz.anti_wedge(rhs.my),
            y: self.d.anti_wedge(rhs.vy) + self.nx.anti_wedge(rhs.mz) - self.nz.anti_wedge(rhs.mx),
            z: self.d.anti_wedge(rhs.vz) - self.nx.anti_wedge(rhs.my) + self.ny.anti_wedge(rhs.mx),
            w: -self.nx.anti_wedge(rhs.vx)
                - self.ny.anti_wedge(rhs.vy)
                - self.nz.anti_wedge(rhs.vz),
        }
    }
}

// ---------------------------------------------------------------------
// Plane OP Plane:

// Plane.geometric(Plane) -> Motor3
impl Geometric<Plane> for Plane {
    type Output = Motor3;
    fn geometric(self, rhs: Plane) -> Self::Output {
        Motor3 {
            rx: -self.d.geometric(rhs.nx) + self.nx.geometric(rhs.d),
            ry: -self.d.geometric(rhs.ny) + self.ny.geometric(rhs.d),
            rz: -self.d.geometric(rhs.nz) + self.nz.geometric(rhs.d),
            rw: Default::default(),
            ux: Default::default(),
            uy: Default::default(),
            uz: Default::default(),
            uw: -self.d.geometric(rhs.d),
        }
    }
}

// Omitted: Plane anti_geometric Plane = self.d.anti_geometric(rhs.nx) + self.d.anti_geometric(rhs.ny) + self.d.anti_geometric(rhs.nz) + self.nx.anti_geometric(rhs.d) + self.nx.anti_geometric(rhs.nx) + self.nx.anti_geometric(rhs.ny) + self.nx.anti_geometric(rhs.nz) + self.ny.anti_geometric(rhs.d) + self.ny.anti_geometric(rhs.nx) + self.ny.anti_geometric(rhs.ny) + self.ny.anti_geometric(rhs.nz) + self.nz.anti_geometric(rhs.d) + self.nz.anti_geometric(rhs.nx) + self.nz.anti_geometric(rhs.ny) + self.nz.anti_geometric(rhs.nz)

// Plane.dot(Plane) -> S
impl Dot<Plane> for Plane {
    type Output = S;
    fn dot(self, rhs: Plane) -> Self::Output {
        self.d.dot(rhs.d)
    }
}

// Omitted: Plane wedge Plane = 0

// Plane.anti_wedge(Plane) -> Line3
impl AntiWedge<Plane> for Plane {
    type Output = Line3;
    fn anti_wedge(self, rhs: Plane) -> Self::Output {
        Line3 {
            vx: -self.ny.anti_wedge(rhs.nz) + self.nz.anti_wedge(rhs.ny),
            vy: self.nx.anti_wedge(rhs.nz) - self.nz.anti_wedge(rhs.nx),
            vz: -self.nx.anti_wedge(rhs.ny) + self.ny.anti_wedge(rhs.nx),
            mx: -self.d.anti_wedge(rhs.nx) + self.nx.anti_wedge(rhs.d),
            my: -self.d.anti_wedge(rhs.ny) + self.ny.anti_wedge(rhs.d),
            mz: -self.d.anti_wedge(rhs.nz) + self.nz.anti_wedge(rhs.d),
        }
    }
}

// ---------------------------------------------------------------------
// Plane OP Translator3:

// Omitted: Plane geometric Translator3 = self.d.geometric(rhs.x) + self.d.geometric(rhs.xyzw) + self.d.geometric(rhs.y) + self.d.geometric(rhs.z) + self.nx.geometric(rhs.x) + self.nx.geometric(rhs.y) + self.nx.geometric(rhs.z) + self.ny.geometric(rhs.x) + self.ny.geometric(rhs.y) + self.ny.geometric(rhs.z) + self.nz.geometric(rhs.x) + self.nz.geometric(rhs.y) + self.nz.geometric(rhs.z)
// Omitted: Plane anti_geometric Translator3 = self.d.anti_geometric(rhs.xyzw) + self.nx.anti_geometric(rhs.x) + self.nx.anti_geometric(rhs.xyzw) + self.nx.anti_geometric(rhs.y) + self.nx.anti_geometric(rhs.z) + self.ny.anti_geometric(rhs.x) + self.ny.anti_geometric(rhs.xyzw) + self.ny.anti_geometric(rhs.y) + self.ny.anti_geometric(rhs.z) + self.nz.anti_geometric(rhs.x) + self.nz.anti_geometric(rhs.xyzw) + self.nz.anti_geometric(rhs.y) + self.nz.anti_geometric(rhs.z)

// Plane.dot(Translator3) -> Vec4
impl Dot<Translator3> for Plane {
    type Output = Vec4;
    fn dot(self, rhs: Translator3) -> Self::Output {
        Vec4 {
            x: self.d.dot(rhs.x),
            y: self.d.dot(rhs.y),
            z: self.d.dot(rhs.z),
            w: self.d.dot(rhs.xyzw) - self.nx.dot(rhs.x) - self.ny.dot(rhs.y) - self.nz.dot(rhs.z),
        }
    }
}

// Omitted: Plane wedge Translator3 = 0
// Omitted: Plane anti_wedge Translator3 = self.d.anti_wedge(rhs.xyzw) + self.nx.anti_wedge(rhs.xyzw) + self.nx.anti_wedge(rhs.y) + self.nx.anti_wedge(rhs.z) + self.ny.anti_wedge(rhs.x) + self.ny.anti_wedge(rhs.xyzw) + self.ny.anti_wedge(rhs.z) + self.nz.anti_wedge(rhs.x) + self.nz.anti_wedge(rhs.xyzw) + self.nz.anti_wedge(rhs.y)

// ---------------------------------------------------------------------
// Plane OP Rotor3:

// Omitted: Plane geometric Rotor3 = self.d.geometric(rhs.w) + self.d.geometric(rhs.x) + self.d.geometric(rhs.y) + self.d.geometric(rhs.z)
// Omitted: Plane anti_geometric Rotor3 = self.d.anti_geometric(rhs.w) + self.d.anti_geometric(rhs.x) + self.d.anti_geometric(rhs.y) + self.d.anti_geometric(rhs.z) + self.nx.anti_geometric(rhs.w) + self.nx.anti_geometric(rhs.x) + self.nx.anti_geometric(rhs.y) + self.nx.anti_geometric(rhs.z) + self.ny.anti_geometric(rhs.w) + self.ny.anti_geometric(rhs.x) + self.ny.anti_geometric(rhs.y) + self.ny.anti_geometric(rhs.z) + self.nz.anti_geometric(rhs.w) + self.nz.anti_geometric(rhs.x) + self.nz.anti_geometric(rhs.y) + self.nz.anti_geometric(rhs.z)

// Plane.dot(Rotor3) -> W
impl Dot<Rotor3> for Plane {
    type Output = W;
    fn dot(self, rhs: Rotor3) -> Self::Output {
        self.d.dot(rhs.w)
    }
}

// Omitted: Plane wedge Rotor3 = 0
// Omitted: Plane anti_wedge Rotor3 = self.d.anti_wedge(rhs.w) + self.d.anti_wedge(rhs.x) + self.d.anti_wedge(rhs.y) + self.d.anti_wedge(rhs.z) + self.nx.anti_wedge(rhs.w) + self.nx.anti_wedge(rhs.x) + self.ny.anti_wedge(rhs.w) + self.ny.anti_wedge(rhs.y) + self.nz.anti_wedge(rhs.w) + self.nz.anti_wedge(rhs.z)

// ---------------------------------------------------------------------
// Plane OP Motor3:

// Omitted: Plane geometric Motor3 = self.d.geometric(rhs.rw) + self.d.geometric(rhs.rx) + self.d.geometric(rhs.ry) + self.d.geometric(rhs.rz) + self.d.geometric(rhs.uw) + self.d.geometric(rhs.ux) + self.d.geometric(rhs.uy) + self.d.geometric(rhs.uz) + self.nx.geometric(rhs.uw) + self.ny.geometric(rhs.uw) + self.nz.geometric(rhs.uw)
// Omitted: Plane anti_geometric Motor3 = self.d.anti_geometric(rhs.rw) + self.d.anti_geometric(rhs.rx) + self.d.anti_geometric(rhs.ry) + self.d.anti_geometric(rhs.rz) + self.d.anti_geometric(rhs.ux) + self.d.anti_geometric(rhs.uy) + self.d.anti_geometric(rhs.uz) + self.nx.anti_geometric(rhs.rw) + self.nx.anti_geometric(rhs.rx) + self.nx.anti_geometric(rhs.ry) + self.nx.anti_geometric(rhs.rz) + self.nx.anti_geometric(rhs.uw) + self.nx.anti_geometric(rhs.ux) + self.nx.anti_geometric(rhs.uy) + self.nx.anti_geometric(rhs.uz) + self.ny.anti_geometric(rhs.rw) + self.ny.anti_geometric(rhs.rx) + self.ny.anti_geometric(rhs.ry) + self.ny.anti_geometric(rhs.rz) + self.ny.anti_geometric(rhs.uw) + self.ny.anti_geometric(rhs.ux) + self.ny.anti_geometric(rhs.uy) + self.ny.anti_geometric(rhs.uz) + self.nz.anti_geometric(rhs.rw) + self.nz.anti_geometric(rhs.rx) + self.nz.anti_geometric(rhs.ry) + self.nz.anti_geometric(rhs.rz) + self.nz.anti_geometric(rhs.uw) + self.nz.anti_geometric(rhs.ux) + self.nz.anti_geometric(rhs.uy) + self.nz.anti_geometric(rhs.uz)
// Omitted: Plane dot Motor3 = self.d.dot(rhs.rw) + self.d.dot(rhs.uw) + self.nx.dot(rhs.uw) + self.ny.dot(rhs.uw) + self.nz.dot(rhs.uw)

// Plane.wedge(Motor3) -> Plane
impl Wedge<Motor3> for Plane {
    type Output = Plane;
    fn wedge(self, rhs: Motor3) -> Self::Output {
        Plane {
            nx: self.nx.wedge(rhs.uw),
            ny: self.ny.wedge(rhs.uw),
            nz: self.nz.wedge(rhs.uw),
            d: self.d.wedge(rhs.uw),
        }
    }
}

// Omitted: Plane anti_wedge Motor3 = self.d.anti_wedge(rhs.rw) + self.d.anti_wedge(rhs.rx) + self.d.anti_wedge(rhs.ry) + self.d.anti_wedge(rhs.rz) + self.d.anti_wedge(rhs.ux) + self.d.anti_wedge(rhs.uy) + self.d.anti_wedge(rhs.uz) + self.nx.anti_wedge(rhs.rw) + self.nx.anti_wedge(rhs.rx) + self.nx.anti_wedge(rhs.uy) + self.nx.anti_wedge(rhs.uz) + self.ny.anti_wedge(rhs.rw) + self.ny.anti_wedge(rhs.ry) + self.ny.anti_wedge(rhs.ux) + self.ny.anti_wedge(rhs.uz) + self.nz.anti_wedge(rhs.rw) + self.nz.anti_wedge(rhs.rz) + self.nz.anti_wedge(rhs.ux) + self.nz.anti_wedge(rhs.uy)

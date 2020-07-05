//! # Rotor3
//!
//! ## Operations
//! ```text
//! Rotor3.anti_geometric(Rotor3) -> Rotor3
//! Rotor3.anti_wedge(Rotor3) -> Rotor3
//! Rotor3.wedge(Vec3) -> Plane
//! Vec3.wedge(Rotor3) -> Plane
//! Rotor3.anti_wedge(Vec3) -> Vec3
//! Vec3.anti_wedge(Rotor3) -> Vec3
//! Rotor3.wedge(Vec4) -> Plane
//! Vec4.wedge(Rotor3) -> Plane
//! Rotor3.anti_wedge(Vec4) -> Vec4
//! Vec4.anti_wedge(Rotor3) -> Vec4
//! Rotor3.geometric(Line3) -> Rotor3
//! Line3.geometric(Rotor3) -> Rotor3
//! Rotor3.dot(Line3) -> Line3
//! Line3.dot(Rotor3) -> Line3
//! Rotor3.wedge(Line3) -> XYZW
//! Line3.wedge(Rotor3) -> XYZW
//! Rotor3.dot(Plane) -> W
//! Plane.dot(Rotor3) -> W
//! Rotor3.geometric(Translator3) -> Rotor3
//! Translator3.geometric(Rotor3) -> Rotor3
//! Rotor3.dot(Translator3) -> Line3
//! Translator3.dot(Rotor3) -> Line3
//! Rotor3.wedge(Translator3) -> XYZW
//! Translator3.wedge(Rotor3) -> XYZW
//! Rotor3.geometric(Motor3) -> Rotor3
//! Motor3.geometric(Rotor3) -> Rotor3
//! Rotor3.dot(Motor3) -> Rotor3
//! Motor3.dot(Rotor3) -> Rotor3
//! Rotor3.wedge(Motor3) -> Rotor3
//! Motor3.wedge(Rotor3) -> Rotor3
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
pub struct Rotor3 {
    pub x: WX,
    pub y: WY,
    pub z: WZ,
    pub w: XYZW,
}

// ---------------------------------------------------------------------
// Omitted: Rotor3.rcompl() -> self.w.rcompl() + self.x.rcompl() + self.y.rcompl() + self.z.rcompl()
// Omitted: Rotor3.lcompl() -> self.w.lcompl() + self.x.lcompl() + self.y.lcompl() + self.z.lcompl()

impl Reverse for Rotor3 {
    fn rev(self) -> Self {
        Rotor3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: self.w,
        }
    }
}

impl AntiReverse for Rotor3 {
    fn arev(self) -> Self {
        Rotor3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: self.w,
        }
    }
}

// ---------------------------------------------------------------------
// Rotor3 OP Vec3:

// Omitted: Rotor3 geometric Vec3 = self.w.geometric(rhs.x) + self.w.geometric(rhs.y) + self.w.geometric(rhs.z) + self.x.geometric(rhs.x) + self.x.geometric(rhs.y) + self.x.geometric(rhs.z) + self.y.geometric(rhs.x) + self.y.geometric(rhs.y) + self.y.geometric(rhs.z) + self.z.geometric(rhs.x) + self.z.geometric(rhs.y) + self.z.geometric(rhs.z)
// Omitted: Rotor3 anti_geometric Vec3 = self.w.anti_geometric(rhs.x) + self.w.anti_geometric(rhs.y) + self.w.anti_geometric(rhs.z) + self.x.anti_geometric(rhs.x) + self.x.anti_geometric(rhs.y) + self.x.anti_geometric(rhs.z) + self.y.anti_geometric(rhs.x) + self.y.anti_geometric(rhs.y) + self.y.anti_geometric(rhs.z) + self.z.anti_geometric(rhs.x) + self.z.anti_geometric(rhs.y) + self.z.anti_geometric(rhs.z)
// Omitted: Rotor3 dot Vec3 = self.w.dot(rhs.x) + self.w.dot(rhs.y) + self.w.dot(rhs.z) + self.x.dot(rhs.x) + self.y.dot(rhs.y) + self.z.dot(rhs.z)

// Rotor3.wedge(Vec3) -> Plane
impl Wedge<Vec3> for Rotor3 {
    type Output = Plane;
    fn wedge(self, rhs: Vec3) -> Self::Output {
        Plane {
            nx: self.y.wedge(rhs.z) - self.z.wedge(rhs.y),
            ny: -self.x.wedge(rhs.z) + self.z.wedge(rhs.x),
            nz: self.x.wedge(rhs.y) - self.y.wedge(rhs.x),
            d: Default::default(),
        }
    }
}

// Rotor3.anti_wedge(Vec3) -> Vec3
impl AntiWedge<Vec3> for Rotor3 {
    type Output = Vec3;
    fn anti_wedge(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.w.anti_wedge(rhs.x),
            y: self.w.anti_wedge(rhs.y),
            z: self.w.anti_wedge(rhs.z),
        }
    }
}

// ---------------------------------------------------------------------
// Rotor3 OP Vec4:

// Omitted: Rotor3 geometric Vec4 = self.w.geometric(rhs.x) + self.w.geometric(rhs.y) + self.w.geometric(rhs.z) + self.x.geometric(rhs.x) + self.x.geometric(rhs.y) + self.x.geometric(rhs.z) + self.y.geometric(rhs.x) + self.y.geometric(rhs.y) + self.y.geometric(rhs.z) + self.z.geometric(rhs.x) + self.z.geometric(rhs.y) + self.z.geometric(rhs.z)
// Omitted: Rotor3 anti_geometric Vec4 = self.w.anti_geometric(rhs.w) + self.w.anti_geometric(rhs.x) + self.w.anti_geometric(rhs.y) + self.w.anti_geometric(rhs.z) + self.x.anti_geometric(rhs.w) + self.x.anti_geometric(rhs.x) + self.x.anti_geometric(rhs.y) + self.x.anti_geometric(rhs.z) + self.y.anti_geometric(rhs.w) + self.y.anti_geometric(rhs.x) + self.y.anti_geometric(rhs.y) + self.y.anti_geometric(rhs.z) + self.z.anti_geometric(rhs.w) + self.z.anti_geometric(rhs.x) + self.z.anti_geometric(rhs.y) + self.z.anti_geometric(rhs.z)
// Omitted: Rotor3 dot Vec4 = self.w.dot(rhs.x) + self.w.dot(rhs.y) + self.w.dot(rhs.z) + self.x.dot(rhs.x) + self.y.dot(rhs.y) + self.z.dot(rhs.z)

// Rotor3.wedge(Vec4) -> Plane
impl Wedge<Vec4> for Rotor3 {
    type Output = Plane;
    fn wedge(self, rhs: Vec4) -> Self::Output {
        Plane {
            nx: self.y.wedge(rhs.z) - self.z.wedge(rhs.y),
            ny: -self.x.wedge(rhs.z) + self.z.wedge(rhs.x),
            nz: self.x.wedge(rhs.y) - self.y.wedge(rhs.x),
            d: Default::default(),
        }
    }
}

// Rotor3.anti_wedge(Vec4) -> Vec4
impl AntiWedge<Vec4> for Rotor3 {
    type Output = Vec4;
    fn anti_wedge(self, rhs: Vec4) -> Self::Output {
        Vec4 {
            x: self.w.anti_wedge(rhs.x),
            y: self.w.anti_wedge(rhs.y),
            z: self.w.anti_wedge(rhs.z),
            w: self.w.anti_wedge(rhs.w),
        }
    }
}

// ---------------------------------------------------------------------
// Rotor3 OP Line3:

// Rotor3.geometric(Line3) -> Rotor3
impl Geometric<Line3> for Rotor3 {
    type Output = Rotor3;
    fn geometric(self, rhs: Line3) -> Self::Output {
        Rotor3 {
            x: self.w.geometric(rhs.mx) - self.y.geometric(rhs.mz) + self.z.geometric(rhs.my),
            y: self.w.geometric(rhs.my) + self.x.geometric(rhs.mz) - self.z.geometric(rhs.mx),
            z: self.w.geometric(rhs.mz) - self.x.geometric(rhs.my) + self.y.geometric(rhs.mx),
            w: -self.x.geometric(rhs.mx) - self.y.geometric(rhs.my) - self.z.geometric(rhs.mz),
        }
    }
}

// Omitted: Rotor3 anti_geometric Line3 = self.w.anti_geometric(rhs.mx) + self.w.anti_geometric(rhs.my) + self.w.anti_geometric(rhs.mz) + self.w.anti_geometric(rhs.vx) + self.w.anti_geometric(rhs.vy) + self.w.anti_geometric(rhs.vz) + self.x.anti_geometric(rhs.mx) + self.x.anti_geometric(rhs.my) + self.x.anti_geometric(rhs.mz) + self.x.anti_geometric(rhs.vx) + self.x.anti_geometric(rhs.vy) + self.x.anti_geometric(rhs.vz) + self.y.anti_geometric(rhs.mx) + self.y.anti_geometric(rhs.my) + self.y.anti_geometric(rhs.mz) + self.y.anti_geometric(rhs.vx) + self.y.anti_geometric(rhs.vy) + self.y.anti_geometric(rhs.vz) + self.z.anti_geometric(rhs.mx) + self.z.anti_geometric(rhs.my) + self.z.anti_geometric(rhs.mz) + self.z.anti_geometric(rhs.vx) + self.z.anti_geometric(rhs.vy) + self.z.anti_geometric(rhs.vz)

// Rotor3.dot(Line3) -> Line3
impl Dot<Line3> for Rotor3 {
    type Output = Line3;
    fn dot(self, rhs: Line3) -> Self::Output {
        Line3 {
            vx: self.w.dot(rhs.mx),
            vy: self.w.dot(rhs.my),
            vz: self.w.dot(rhs.mz),
            mx: Default::default(),
            my: Default::default(),
            mz: Default::default(),
        }
    }
}

// Rotor3.wedge(Line3) -> XYZW
impl Wedge<Line3> for Rotor3 {
    type Output = XYZW;
    fn wedge(self, rhs: Line3) -> Self::Output {
        self.x.wedge(rhs.mx) + self.y.wedge(rhs.my) + self.z.wedge(rhs.mz)
    }
}

// Omitted: Rotor3 anti_wedge Line3 = self.w.anti_wedge(rhs.mx) + self.w.anti_wedge(rhs.my) + self.w.anti_wedge(rhs.mz) + self.w.anti_wedge(rhs.vx) + self.w.anti_wedge(rhs.vy) + self.w.anti_wedge(rhs.vz) + self.x.anti_wedge(rhs.mx) + self.y.anti_wedge(rhs.my) + self.z.anti_wedge(rhs.mz)

// ---------------------------------------------------------------------
// Rotor3 OP Plane:

// Omitted: Rotor3 geometric Plane = self.w.geometric(rhs.d) + self.x.geometric(rhs.d) + self.y.geometric(rhs.d) + self.z.geometric(rhs.d)
// Omitted: Rotor3 anti_geometric Plane = self.w.anti_geometric(rhs.d) + self.w.anti_geometric(rhs.nx) + self.w.anti_geometric(rhs.ny) + self.w.anti_geometric(rhs.nz) + self.x.anti_geometric(rhs.d) + self.x.anti_geometric(rhs.nx) + self.x.anti_geometric(rhs.ny) + self.x.anti_geometric(rhs.nz) + self.y.anti_geometric(rhs.d) + self.y.anti_geometric(rhs.nx) + self.y.anti_geometric(rhs.ny) + self.y.anti_geometric(rhs.nz) + self.z.anti_geometric(rhs.d) + self.z.anti_geometric(rhs.nx) + self.z.anti_geometric(rhs.ny) + self.z.anti_geometric(rhs.nz)

// Rotor3.dot(Plane) -> W
impl Dot<Plane> for Rotor3 {
    type Output = W;
    fn dot(self, rhs: Plane) -> Self::Output {
        self.w.dot(rhs.d)
    }
}

// Omitted: Rotor3 wedge Plane = 0
// Omitted: Rotor3 anti_wedge Plane = self.w.anti_wedge(rhs.d) + self.w.anti_wedge(rhs.nx) + self.w.anti_wedge(rhs.ny) + self.w.anti_wedge(rhs.nz) + self.x.anti_wedge(rhs.d) + self.x.anti_wedge(rhs.nx) + self.y.anti_wedge(rhs.d) + self.y.anti_wedge(rhs.ny) + self.z.anti_wedge(rhs.d) + self.z.anti_wedge(rhs.nz)

// ---------------------------------------------------------------------
// Rotor3 OP Translator3:

// Rotor3.geometric(Translator3) -> Rotor3
impl Geometric<Translator3> for Rotor3 {
    type Output = Rotor3;
    fn geometric(self, rhs: Translator3) -> Self::Output {
        Rotor3 {
            x: self.w.geometric(rhs.x) - self.y.geometric(rhs.z) + self.z.geometric(rhs.y),
            y: self.w.geometric(rhs.y) + self.x.geometric(rhs.z) - self.z.geometric(rhs.x),
            z: self.w.geometric(rhs.z) - self.x.geometric(rhs.y) + self.y.geometric(rhs.x),
            w: -self.x.geometric(rhs.x) - self.y.geometric(rhs.y) - self.z.geometric(rhs.z),
        }
    }
}

// Omitted: Rotor3 anti_geometric Translator3 = self.w.anti_geometric(rhs.x) + self.w.anti_geometric(rhs.xyzw) + self.w.anti_geometric(rhs.y) + self.w.anti_geometric(rhs.z) + self.x.anti_geometric(rhs.x) + self.x.anti_geometric(rhs.xyzw) + self.x.anti_geometric(rhs.y) + self.x.anti_geometric(rhs.z) + self.y.anti_geometric(rhs.x) + self.y.anti_geometric(rhs.xyzw) + self.y.anti_geometric(rhs.y) + self.y.anti_geometric(rhs.z) + self.z.anti_geometric(rhs.x) + self.z.anti_geometric(rhs.xyzw) + self.z.anti_geometric(rhs.y) + self.z.anti_geometric(rhs.z)

// Rotor3.dot(Translator3) -> Line3
impl Dot<Translator3> for Rotor3 {
    type Output = Line3;
    fn dot(self, rhs: Translator3) -> Self::Output {
        Line3 {
            vx: self.w.dot(rhs.x),
            vy: self.w.dot(rhs.y),
            vz: self.w.dot(rhs.z),
            mx: Default::default(),
            my: Default::default(),
            mz: Default::default(),
        }
    }
}

// Rotor3.wedge(Translator3) -> XYZW
impl Wedge<Translator3> for Rotor3 {
    type Output = XYZW;
    fn wedge(self, rhs: Translator3) -> Self::Output {
        self.x.wedge(rhs.x) + self.y.wedge(rhs.y) + self.z.wedge(rhs.z)
    }
}

// Omitted: Rotor3 anti_wedge Translator3 = self.w.anti_wedge(rhs.x) + self.w.anti_wedge(rhs.xyzw) + self.w.anti_wedge(rhs.y) + self.w.anti_wedge(rhs.z) + self.x.anti_wedge(rhs.x) + self.x.anti_wedge(rhs.xyzw) + self.y.anti_wedge(rhs.xyzw) + self.y.anti_wedge(rhs.y) + self.z.anti_wedge(rhs.xyzw) + self.z.anti_wedge(rhs.z)

// ---------------------------------------------------------------------
// Rotor3 OP Rotor3:

// Omitted: Rotor3 geometric Rotor3 = 0

// Rotor3.anti_geometric(Rotor3) -> Rotor3
impl AntiGeometric<Rotor3> for Rotor3 {
    type Output = Rotor3;
    fn anti_geometric(self, rhs: Rotor3) -> Self::Output {
        Rotor3 {
            x: self.w.anti_geometric(rhs.x)
                + self.x.anti_geometric(rhs.w)
                + self.y.anti_geometric(rhs.z)
                - self.z.anti_geometric(rhs.y),
            y: self.w.anti_geometric(rhs.y) - self.x.anti_geometric(rhs.z)
                + self.y.anti_geometric(rhs.w)
                + self.z.anti_geometric(rhs.x),
            z: self.w.anti_geometric(rhs.z) + self.x.anti_geometric(rhs.y)
                - self.y.anti_geometric(rhs.x)
                + self.z.anti_geometric(rhs.w),
            w: self.w.anti_geometric(rhs.w)
                - self.x.anti_geometric(rhs.x)
                - self.y.anti_geometric(rhs.y)
                - self.z.anti_geometric(rhs.z),
        }
    }
}

// Omitted: Rotor3 dot Rotor3 = 0
// Omitted: Rotor3 wedge Rotor3 = 0

// Rotor3.anti_wedge(Rotor3) -> Rotor3
impl AntiWedge<Rotor3> for Rotor3 {
    type Output = Rotor3;
    fn anti_wedge(self, rhs: Rotor3) -> Self::Output {
        Rotor3 {
            x: self.w.anti_wedge(rhs.x) + self.x.anti_wedge(rhs.w),
            y: self.w.anti_wedge(rhs.y) + self.y.anti_wedge(rhs.w),
            z: self.w.anti_wedge(rhs.z) + self.z.anti_wedge(rhs.w),
            w: self.w.anti_wedge(rhs.w),
        }
    }
}

// ---------------------------------------------------------------------
// Rotor3 OP Motor3:

// Rotor3.geometric(Motor3) -> Rotor3
impl Geometric<Motor3> for Rotor3 {
    type Output = Rotor3;
    fn geometric(self, rhs: Motor3) -> Self::Output {
        Rotor3 {
            x: self.x.geometric(rhs.uw),
            y: self.y.geometric(rhs.uw),
            z: self.z.geometric(rhs.uw),
            w: self.w.geometric(rhs.uw),
        }
    }
}

// Omitted: Rotor3 anti_geometric Motor3 = self.w.anti_geometric(rhs.rw) + self.w.anti_geometric(rhs.rx) + self.w.anti_geometric(rhs.ry) + self.w.anti_geometric(rhs.rz) + self.w.anti_geometric(rhs.uw) + self.w.anti_geometric(rhs.ux) + self.w.anti_geometric(rhs.uy) + self.w.anti_geometric(rhs.uz) + self.x.anti_geometric(rhs.rw) + self.x.anti_geometric(rhs.rx) + self.x.anti_geometric(rhs.ry) + self.x.anti_geometric(rhs.rz) + self.x.anti_geometric(rhs.uw) + self.x.anti_geometric(rhs.ux) + self.x.anti_geometric(rhs.uy) + self.x.anti_geometric(rhs.uz) + self.y.anti_geometric(rhs.rw) + self.y.anti_geometric(rhs.rx) + self.y.anti_geometric(rhs.ry) + self.y.anti_geometric(rhs.rz) + self.y.anti_geometric(rhs.uw) + self.y.anti_geometric(rhs.ux) + self.y.anti_geometric(rhs.uy) + self.y.anti_geometric(rhs.uz) + self.z.anti_geometric(rhs.rw) + self.z.anti_geometric(rhs.rx) + self.z.anti_geometric(rhs.ry) + self.z.anti_geometric(rhs.rz) + self.z.anti_geometric(rhs.uw) + self.z.anti_geometric(rhs.ux) + self.z.anti_geometric(rhs.uy) + self.z.anti_geometric(rhs.uz)

// Rotor3.dot(Motor3) -> Rotor3
impl Dot<Motor3> for Rotor3 {
    type Output = Rotor3;
    fn dot(self, rhs: Motor3) -> Self::Output {
        Rotor3 {
            x: self.x.dot(rhs.uw),
            y: self.y.dot(rhs.uw),
            z: self.z.dot(rhs.uw),
            w: self.w.dot(rhs.uw),
        }
    }
}

// Rotor3.wedge(Motor3) -> Rotor3
impl Wedge<Motor3> for Rotor3 {
    type Output = Rotor3;
    fn wedge(self, rhs: Motor3) -> Self::Output {
        Rotor3 {
            x: self.x.wedge(rhs.uw),
            y: self.y.wedge(rhs.uw),
            z: self.z.wedge(rhs.uw),
            w: self.w.wedge(rhs.uw),
        }
    }
}

// Omitted: Rotor3 anti_wedge Motor3 = self.w.anti_wedge(rhs.rw) + self.w.anti_wedge(rhs.rx) + self.w.anti_wedge(rhs.ry) + self.w.anti_wedge(rhs.rz) + self.w.anti_wedge(rhs.uw) + self.w.anti_wedge(rhs.ux) + self.w.anti_wedge(rhs.uy) + self.w.anti_wedge(rhs.uz) + self.x.anti_wedge(rhs.rw) + self.x.anti_wedge(rhs.ux) + self.y.anti_wedge(rhs.rw) + self.y.anti_wedge(rhs.uy) + self.z.anti_wedge(rhs.rw) + self.z.anti_wedge(rhs.uz)

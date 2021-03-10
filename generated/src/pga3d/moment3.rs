//! # Moment3
//!
//! ## Operations
//! ```text
//! Moment3.dot(Moment3) -> S
//! Moment3.dot(Vec3) -> Vec3
//! Vec3.dot(Moment3) -> Vec3
//! Moment3.wedge(Vec3) -> XYZ
//! Vec3.wedge(Moment3) -> XYZ
//! Moment3.anti_geometric(Vec4) -> Vec3
//! Vec4.anti_geometric(Moment3) -> Vec3
//! Moment3.dot(Vec4) -> Vec3
//! Vec4.dot(Moment3) -> Vec3
//! Moment3.wedge(Vec4) -> Plane
//! Vec4.wedge(Moment3) -> Plane
//! Moment3.dot(Line3) -> S
//! Line3.dot(Moment3) -> S
//! Moment3.wedge(Line3) -> XYZW
//! Line3.wedge(Moment3) -> XYZW
//! Moment3.anti_wedge(Line3) -> S
//! Line3.anti_wedge(Moment3) -> S
//! Moment3.dot(Plane) -> Vec4
//! Plane.dot(Moment3) -> Vec4
//! Moment3.anti_wedge(Plane) -> Vec3
//! Plane.anti_wedge(Moment3) -> Vec3
//! Moment3.geometric(Rotor3) -> Rotor3
//! Rotor3.geometric(Moment3) -> Rotor3
//! Moment3.dot(Rotor3) -> Line3
//! Rotor3.dot(Moment3) -> Line3
//! Moment3.wedge(Rotor3) -> XYZW
//! Rotor3.wedge(Moment3) -> XYZW
//!
//! ```

use super::*;

#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, derive_more::Neg, derive_more::Add, derive_more::Sub)]
pub struct Moment3 {
	pub mx: YZ,
	pub my: ZX,
	pub mz: XY,
}

// ---------------------------------------------------------------------

impl RCompl for Moment3 {
	type Output = Line3;
	fn rcompl(self) -> Self::Output {
		Line3 {
			vx: -self.mx.rcompl(),
			vy: -self.my.rcompl(),
			vz: -self.mz.rcompl(),
			mx: Default::default(),
			my: Default::default(),
			mz: Default::default(),
		}
	}
}

impl LCompl for Moment3 {
	type Output = Line3;
	fn lcompl(self) -> Self::Output {
		Line3 {
			vx: -self.mx.lcompl(),
			vy: -self.my.lcompl(),
			vz: -self.mz.lcompl(),
			mx: Default::default(),
			my: Default::default(),
			mz: Default::default(),
		}
	}
}

impl Reverse for Moment3 {
	fn rev(self) -> Self {
		Moment3 {
			mx: -self.mx,
			my: -self.my,
			mz: -self.mz,
		}
	}
}

impl AntiReverse for Moment3 {
	fn arev(self) -> Self {
		Moment3 {
			mx: -self.mx,
			my: -self.my,
			mz: -self.mz,
		}
	}
}

// ---------------------------------------------------------------------
// Moment3 OP Vec3:

// Omitted: Moment3 geometric Vec3 = self.mx * rhs.x + self.mx * rhs.y + self.mx * rhs.z + self.my * rhs.x + self.my * rhs.y + self.my * rhs.z + self.mz * rhs.x + self.mz * rhs.y + self.mz * rhs.z  (unnamed type)
// Omitted: Moment3 anti_geometric Vec3 = 0  (unnamed type)

// Moment3.dot(Vec3) -> Vec3
impl Dot<Vec3> for Moment3 {
	type Output = Vec3;
	fn dot(self, rhs: Vec3) -> Self::Output {
		// Vec3 {
		//     x: X(self.my.0 * rhs.z.0) + X(self.mz.0 * rhs.y.0),
		//     y: Y(self.mx.0 * rhs.z.0) + Y(self.mz.0 * rhs.x.0),
		//     z: Z(self.mx.0 * rhs.y.0) + Z(self.my.0 * rhs.x.0),
		// }
		Vec3 {
			x: -self.my.dot(rhs.z) + self.mz.dot(rhs.y),
			y: self.mx.dot(rhs.z) - self.mz.dot(rhs.x),
			z: -self.mx.dot(rhs.y) + self.my.dot(rhs.x),
		}
	}
}

// Moment3.wedge(Vec3) -> XYZ
impl Wedge<Vec3> for Moment3 {
	type Output = XYZ;
	fn wedge(self, rhs: Vec3) -> Self::Output {
		// XYZ(self.mx.0 * rhs.x.0) + XYZ(self.my.0 * rhs.y.0) + XYZ(self.mz.0 * rhs.z.0)
		self.mx.wedge(rhs.x) + self.my.wedge(rhs.y) + self.mz.wedge(rhs.z)
	}
}

// Omitted: Moment3 anti_wedge Vec3 = 0  (unnamed type)

// ---------------------------------------------------------------------
// Moment3 OP Vec4:

// Omitted: Moment3 geometric Vec4 = self.mx * rhs.w + self.mx * rhs.x + self.mx * rhs.y + self.mx * rhs.z + self.my * rhs.w + self.my * rhs.x + self.my * rhs.y + self.my * rhs.z + self.mz * rhs.w + self.mz * rhs.x + self.mz * rhs.y + self.mz * rhs.z  (unnamed type)

// Moment3.anti_geometric(Vec4) -> Vec3
impl AntiGeometric<Vec4> for Moment3 {
	type Output = Vec3;
	fn anti_geometric(self, rhs: Vec4) -> Self::Output {
		// Vec3 {
		//     x: X(self.mx.0 * rhs.w.0),
		//     y: Y(self.my.0 * rhs.w.0),
		//     z: Z(self.mz.0 * rhs.w.0),
		// }
		Vec3 {
			x: self.mx.anti_geometric(rhs.w),
			y: self.my.anti_geometric(rhs.w),
			z: self.mz.anti_geometric(rhs.w),
		}
	}
}

// Moment3.dot(Vec4) -> Vec3
impl Dot<Vec4> for Moment3 {
	type Output = Vec3;
	fn dot(self, rhs: Vec4) -> Self::Output {
		// Vec3 {
		//     x: X(self.my.0 * rhs.z.0) + X(self.mz.0 * rhs.y.0),
		//     y: Y(self.mx.0 * rhs.z.0) + Y(self.mz.0 * rhs.x.0),
		//     z: Z(self.mx.0 * rhs.y.0) + Z(self.my.0 * rhs.x.0),
		// }
		Vec3 {
			x: -self.my.dot(rhs.z) + self.mz.dot(rhs.y),
			y: self.mx.dot(rhs.z) - self.mz.dot(rhs.x),
			z: -self.mx.dot(rhs.y) + self.my.dot(rhs.x),
		}
	}
}

// Moment3.wedge(Vec4) -> Plane
impl Wedge<Vec4> for Moment3 {
	type Output = Plane;
	fn wedge(self, rhs: Vec4) -> Self::Output {
		// Plane {
		//     nx: YZW(self.mx.0 * rhs.w.0),
		//     ny: ZXW(self.my.0 * rhs.w.0),
		//     nz: XYW(self.mz.0 * rhs.w.0),
		//     d : XYZ(self.mx.0 * rhs.x.0) + XYZ(self.my.0 * rhs.y.0) + XYZ(self.mz.0 * rhs.z.0),
		// }
		Plane {
			nx: self.mx.wedge(rhs.w),
			ny: self.my.wedge(rhs.w),
			nz: self.mz.wedge(rhs.w),
			d: self.mx.wedge(rhs.x) + self.my.wedge(rhs.y) + self.mz.wedge(rhs.z),
		}
	}
}

// Omitted: Moment3 anti_wedge Vec4 = 0  (unnamed type)

// ---------------------------------------------------------------------
// Moment3 OP Moment3:

// Omitted: Moment3 geometric Moment3 = self.mx * rhs.mx + self.mx * rhs.my + self.mx * rhs.mz + self.my * rhs.mx + self.my * rhs.my + self.my * rhs.mz + self.mz * rhs.mx + self.mz * rhs.my + self.mz * rhs.mz  (unnamed type)
// Omitted: Moment3 anti_geometric Moment3 = 0  (unnamed type)

// Moment3.dot(Moment3) -> S
impl Dot<Moment3> for Moment3 {
	type Output = S;
	fn dot(self, rhs: Moment3) -> Self::Output {
		// -S(self.mx.0 * rhs.mx.0) - S(self.my.0 * rhs.my.0) - S(self.mz.0 * rhs.mz.0)
		self.mx.dot(rhs.mx) + self.my.dot(rhs.my) + self.mz.dot(rhs.mz)
	}
}

// Omitted: Moment3 wedge Moment3 = 0  (unnamed type)
// Omitted: Moment3 anti_wedge Moment3 = 0  (unnamed type)

// ---------------------------------------------------------------------
// Moment3 OP Line3:

// Omitted: Moment3 geometric Line3 = self.mx * rhs.mx + self.mx * rhs.my + self.mx * rhs.mz + self.mx * rhs.vx + self.mx * rhs.vy + self.mx * rhs.vz + self.my * rhs.mx + self.my * rhs.my + self.my * rhs.mz + self.my * rhs.vx + self.my * rhs.vy + self.my * rhs.vz + self.mz * rhs.mx + self.mz * rhs.my + self.mz * rhs.mz + self.mz * rhs.vx + self.mz * rhs.vy + self.mz * rhs.vz  (unnamed type)
// Omitted: Moment3 anti_geometric Line3 = self.mx !* rhs.vx + self.mx !* rhs.vy + self.mx !* rhs.vz + self.my !* rhs.vx + self.my !* rhs.vy + self.my !* rhs.vz + self.mz !* rhs.vx + self.mz !* rhs.vy + self.mz !* rhs.vz  (unnamed type)

// Moment3.dot(Line3) -> S
impl Dot<Line3> for Moment3 {
	type Output = S;
	fn dot(self, rhs: Line3) -> Self::Output {
		// -S(self.mx.0 * rhs.mx.0) - S(self.my.0 * rhs.my.0) - S(self.mz.0 * rhs.mz.0)
		self.mx.dot(rhs.mx) + self.my.dot(rhs.my) + self.mz.dot(rhs.mz)
	}
}

// Moment3.wedge(Line3) -> XYZW
impl Wedge<Line3> for Moment3 {
	type Output = XYZW;
	fn wedge(self, rhs: Line3) -> Self::Output {
		// -XYZW(self.mx.0 * rhs.vx.0) - XYZW(self.my.0 * rhs.vy.0) - XYZW(self.mz.0 * rhs.vz.0)
		self.mx.wedge(rhs.vx) + self.my.wedge(rhs.vy) + self.mz.wedge(rhs.vz)
	}
}

// Moment3.anti_wedge(Line3) -> S
impl AntiWedge<Line3> for Moment3 {
	type Output = S;
	fn anti_wedge(self, rhs: Line3) -> Self::Output {
		// -S(self.mx.0 * rhs.vx.0) - S(self.my.0 * rhs.vy.0) - S(self.mz.0 * rhs.vz.0)
		self.mx.anti_wedge(rhs.vx) + self.my.anti_wedge(rhs.vy) + self.mz.anti_wedge(rhs.vz)
	}
}

// ---------------------------------------------------------------------
// Moment3 OP Plane:

// Omitted: Moment3 geometric Plane = self.mx * rhs.d + self.mx * rhs.nx + self.mx * rhs.ny + self.mx * rhs.nz + self.my * rhs.d + self.my * rhs.nx + self.my * rhs.ny + self.my * rhs.nz + self.mz * rhs.d + self.mz * rhs.nx + self.mz * rhs.ny + self.mz * rhs.nz  (unnamed type)
// Omitted: Moment3 anti_geometric Plane = self.mx !* rhs.nx + self.mx !* rhs.ny + self.mx !* rhs.nz + self.my !* rhs.nx + self.my !* rhs.ny + self.my !* rhs.nz + self.mz !* rhs.nx + self.mz !* rhs.ny + self.mz !* rhs.nz  (unnamed type)

// Moment3.dot(Plane) -> Vec4
impl Dot<Plane> for Moment3 {
	type Output = Vec4;
	fn dot(self, rhs: Plane) -> Self::Output {
		// Vec4 {
		//     x: X(self.mx.0 * rhs.d.0),
		//     y: Y(self.my.0 * rhs.d.0),
		//     z: Z(self.mz.0 * rhs.d.0),
		//     w: W(self.mx.0 * rhs.nx.0) + W(self.my.0 * rhs.ny.0) + W(self.mz.0 * rhs.nz.0),
		// }
		Vec4 {
			x: -self.mx.dot(rhs.d),
			y: -self.my.dot(rhs.d),
			z: -self.mz.dot(rhs.d),
			w: -self.mx.dot(rhs.nx) - self.my.dot(rhs.ny) - self.mz.dot(rhs.nz),
		}
	}
}

// Omitted: Moment3 wedge Plane = 0  (unnamed type)

// Moment3.anti_wedge(Plane) -> Vec3
impl AntiWedge<Plane> for Moment3 {
	type Output = Vec3;
	fn anti_wedge(self, rhs: Plane) -> Self::Output {
		// Vec3 {
		//     x: X(self.my.0 * rhs.nz.0) + X(self.mz.0 * rhs.ny.0),
		//     y: Y(self.mx.0 * rhs.nz.0) + Y(self.mz.0 * rhs.nx.0),
		//     z: Z(self.mx.0 * rhs.ny.0) + Z(self.my.0 * rhs.nx.0),
		// }
		Vec3 {
			x: self.my.anti_wedge(rhs.nz) - self.mz.anti_wedge(rhs.ny),
			y: -self.mx.anti_wedge(rhs.nz) + self.mz.anti_wedge(rhs.nx),
			z: self.mx.anti_wedge(rhs.ny) - self.my.anti_wedge(rhs.nx),
		}
	}
}

// ---------------------------------------------------------------------
// Moment3 OP Rotor3:

// Moment3.geometric(Rotor3) -> Rotor3
impl Geometric<Rotor3> for Moment3 {
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

// Omitted: Moment3 anti_geometric Rotor3 = self.mx !* rhs.w + self.mx !* rhs.x + self.mx !* rhs.y + self.mx !* rhs.z + self.my !* rhs.w + self.my !* rhs.x + self.my !* rhs.y + self.my !* rhs.z + self.mz !* rhs.w + self.mz !* rhs.x + self.mz !* rhs.y + self.mz !* rhs.z  (unnamed type)
// Omitted: Moment3 dot Rotor3 = Line3 {     vx: self.mx | rhs.w,     vy: self.my | rhs.w,     vz: self.mz | rhs.w,     mx: 0,     my: 0,     mz: 0, }  (too many zeros)

// Moment3.wedge(Rotor3) -> XYZW
impl Wedge<Rotor3> for Moment3 {
	type Output = XYZW;
	fn wedge(self, rhs: Rotor3) -> Self::Output {
		// -XYZW(self.mx.0 * rhs.x.0) - XYZW(self.my.0 * rhs.y.0) - XYZW(self.mz.0 * rhs.z.0)
		self.mx.wedge(rhs.x) + self.my.wedge(rhs.y) + self.mz.wedge(rhs.z)
	}
}

// Omitted: Moment3 anti_wedge Rotor3 = self.mx & rhs.w + self.mx & rhs.x + self.my & rhs.w + self.my & rhs.y + self.mz & rhs.w + self.mz & rhs.z  (unnamed type)

// ---------------------------------------------------------------------
// Moment3 OP Motor3:

// Omitted: Moment3 geometric Motor3 = self.mx * rhs.rw + self.mx * rhs.rx + self.mx * rhs.ry + self.mx * rhs.rz + self.mx * rhs.uw + self.mx * rhs.ux + self.mx * rhs.uy + self.mx * rhs.uz + self.my * rhs.rw + self.my * rhs.rx + self.my * rhs.ry + self.my * rhs.rz + self.my * rhs.uw + self.my * rhs.ux + self.my * rhs.uy + self.my * rhs.uz + self.mz * rhs.rw + self.mz * rhs.rx + self.mz * rhs.ry + self.mz * rhs.rz + self.mz * rhs.uw + self.mz * rhs.ux + self.mz * rhs.uy + self.mz * rhs.uz  (unnamed type)
// Omitted: Moment3 anti_geometric Motor3 = self.mx !* rhs.rw + self.mx !* rhs.rx + self.mx !* rhs.ry + self.mx !* rhs.rz + self.mx !* rhs.ux + self.mx !* rhs.uy + self.mx !* rhs.uz + self.my !* rhs.rw + self.my !* rhs.rx + self.my !* rhs.ry + self.my !* rhs.rz + self.my !* rhs.ux + self.my !* rhs.uy + self.my !* rhs.uz + self.mz !* rhs.rw + self.mz !* rhs.rx + self.mz !* rhs.ry + self.mz !* rhs.rz + self.mz !* rhs.ux + self.mz !* rhs.uy + self.mz !* rhs.uz  (unnamed type)
// Omitted: Moment3 dot Motor3 = self.mx | rhs.rw + self.mx | rhs.uw + self.mx | rhs.ux + self.my | rhs.rw + self.my | rhs.uw + self.my | rhs.uy + self.mz | rhs.rw + self.mz | rhs.uw + self.mz | rhs.uz  (unnamed type)
// Omitted: Moment3 wedge Motor3 = self.mx ^ rhs.rx + self.mx ^ rhs.uw + self.my ^ rhs.ry + self.my ^ rhs.uw + self.mz ^ rhs.rz + self.mz ^ rhs.uw  (unnamed type)
// Omitted: Moment3 anti_wedge Motor3 = self.mx & rhs.rw + self.mx & rhs.rx + self.mx & rhs.uy + self.mx & rhs.uz + self.my & rhs.rw + self.my & rhs.ry + self.my & rhs.ux + self.my & rhs.uz + self.mz & rhs.rw + self.mz & rhs.rz + self.mz & rhs.ux + self.mz & rhs.uy  (unnamed type)

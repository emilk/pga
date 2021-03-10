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
//! Rotor3.geometric(Motor3) -> Rotor3
//! Motor3.geometric(Rotor3) -> Rotor3
//! Rotor3.dot(Motor3) -> Rotor3
//! Motor3.dot(Rotor3) -> Rotor3
//! Rotor3.wedge(Motor3) -> Rotor3
//! Motor3.wedge(Rotor3) -> Rotor3
//! ```

use super::*;

#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, derive_more::Neg, derive_more::Add, derive_more::Sub)]
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

// Omitted: Rotor3 geometric Vec3 = self.w * rhs.x + self.w * rhs.y + self.w * rhs.z + self.x * rhs.x + self.x * rhs.y + self.x * rhs.z + self.y * rhs.x + self.y * rhs.y + self.y * rhs.z + self.z * rhs.x + self.z * rhs.y + self.z * rhs.z  (unnamed type)
// Omitted: Rotor3 anti_geometric Vec3 = self.w !* rhs.x + self.w !* rhs.y + self.w !* rhs.z + self.x !* rhs.x + self.x !* rhs.y + self.x !* rhs.z + self.y !* rhs.x + self.y !* rhs.y + self.y !* rhs.z + self.z !* rhs.x + self.z !* rhs.y + self.z !* rhs.z  (unnamed type)
// Omitted: Rotor3 dot Vec3 = self.w | rhs.x + self.w | rhs.y + self.w | rhs.z + self.x | rhs.x + self.y | rhs.y + self.z | rhs.z  (unnamed type)

// Rotor3.wedge(Vec3) -> Plane
impl Wedge<Vec3> for Rotor3 {
	type Output = Plane;
	fn wedge(self, rhs: Vec3) -> Self::Output {
		// Plane {
		//     nx: YZW(self.y.0 * rhs.z.0) + YZW(self.z.0 * rhs.y.0),
		//     ny: ZXW(self.x.0 * rhs.z.0) + ZXW(self.z.0 * rhs.x.0),
		//     nz: XYW(self.x.0 * rhs.y.0) + XYW(self.y.0 * rhs.x.0),
		//     d : Default::default(),
		// }
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
		// Vec3 {
		//     x: X(self.w.0 * rhs.x.0),
		//     y: Y(self.w.0 * rhs.y.0),
		//     z: Z(self.w.0 * rhs.z.0),
		// }
		Vec3 {
			x: self.w.anti_wedge(rhs.x),
			y: self.w.anti_wedge(rhs.y),
			z: self.w.anti_wedge(rhs.z),
		}
	}
}

// ---------------------------------------------------------------------
// Rotor3 OP Vec4:

// Omitted: Rotor3 geometric Vec4 = self.w * rhs.x + self.w * rhs.y + self.w * rhs.z + self.x * rhs.x + self.x * rhs.y + self.x * rhs.z + self.y * rhs.x + self.y * rhs.y + self.y * rhs.z + self.z * rhs.x + self.z * rhs.y + self.z * rhs.z  (unnamed type)
// Omitted: Rotor3 anti_geometric Vec4 = self.w !* rhs.w + self.w !* rhs.x + self.w !* rhs.y + self.w !* rhs.z + self.x !* rhs.w + self.x !* rhs.x + self.x !* rhs.y + self.x !* rhs.z + self.y !* rhs.w + self.y !* rhs.x + self.y !* rhs.y + self.y !* rhs.z + self.z !* rhs.w + self.z !* rhs.x + self.z !* rhs.y + self.z !* rhs.z  (unnamed type)
// Omitted: Rotor3 dot Vec4 = self.w | rhs.x + self.w | rhs.y + self.w | rhs.z + self.x | rhs.x + self.y | rhs.y + self.z | rhs.z  (unnamed type)

// Rotor3.wedge(Vec4) -> Plane
impl Wedge<Vec4> for Rotor3 {
	type Output = Plane;
	fn wedge(self, rhs: Vec4) -> Self::Output {
		// Plane {
		//     nx: YZW(self.y.0 * rhs.z.0) + YZW(self.z.0 * rhs.y.0),
		//     ny: ZXW(self.x.0 * rhs.z.0) + ZXW(self.z.0 * rhs.x.0),
		//     nz: XYW(self.x.0 * rhs.y.0) + XYW(self.y.0 * rhs.x.0),
		//     d : Default::default(),
		// }
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
		// Vec4 {
		//     x: X(self.w.0 * rhs.x.0),
		//     y: Y(self.w.0 * rhs.y.0),
		//     z: Z(self.w.0 * rhs.z.0),
		//     w: W(self.w.0 * rhs.w.0),
		// }
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
		// Rotor3 {
		//     x: WX(self.w.0 * rhs.mx.0) + WX(self.y.0 * rhs.mz.0) + WX(self.z.0 * rhs.my.0),
		//     y: WY(self.w.0 * rhs.my.0) + WY(self.x.0 * rhs.mz.0) + WY(self.z.0 * rhs.mx.0),
		//     z: WZ(self.w.0 * rhs.mz.0) + WZ(self.x.0 * rhs.my.0) + WZ(self.y.0 * rhs.mx.0),
		//     w: XYZW(self.x.0 * rhs.mx.0) + XYZW(self.y.0 * rhs.my.0) + XYZW(self.z.0 * rhs.mz.0),
		// }
		Rotor3 {
			x: self.w.geometric(rhs.mx) - self.y.geometric(rhs.mz) + self.z.geometric(rhs.my),
			y: self.w.geometric(rhs.my) + self.x.geometric(rhs.mz) - self.z.geometric(rhs.mx),
			z: self.w.geometric(rhs.mz) - self.x.geometric(rhs.my) + self.y.geometric(rhs.mx),
			w: -self.x.geometric(rhs.mx) - self.y.geometric(rhs.my) - self.z.geometric(rhs.mz),
		}
	}
}

// Omitted: Rotor3 anti_geometric Line3 = self.w !* rhs.mx + self.w !* rhs.my + self.w !* rhs.mz + self.w !* rhs.vx + self.w !* rhs.vy + self.w !* rhs.vz + self.x !* rhs.mx + self.x !* rhs.my + self.x !* rhs.mz + self.x !* rhs.vx + self.x !* rhs.vy + self.x !* rhs.vz + self.y !* rhs.mx + self.y !* rhs.my + self.y !* rhs.mz + self.y !* rhs.vx + self.y !* rhs.vy + self.y !* rhs.vz + self.z !* rhs.mx + self.z !* rhs.my + self.z !* rhs.mz + self.z !* rhs.vx + self.z !* rhs.vy + self.z !* rhs.vz  (unnamed type)
// Omitted: Rotor3 dot Line3 = Line3 {     vx: self.w | rhs.mx,     vy: self.w | rhs.my,     vz: self.w | rhs.mz,     mx: 0,     my: 0,     mz: 0, }  (too many zeros)

// Rotor3.wedge(Line3) -> XYZW
impl Wedge<Line3> for Rotor3 {
	type Output = XYZW;
	fn wedge(self, rhs: Line3) -> Self::Output {
		// -XYZW(self.x.0 * rhs.mx.0) - XYZW(self.y.0 * rhs.my.0) - XYZW(self.z.0 * rhs.mz.0)
		self.x.wedge(rhs.mx) + self.y.wedge(rhs.my) + self.z.wedge(rhs.mz)
	}
}

// Omitted: Rotor3 anti_wedge Line3 = self.w & rhs.mx + self.w & rhs.my + self.w & rhs.mz + self.w & rhs.vx + self.w & rhs.vy + self.w & rhs.vz + self.x & rhs.mx + self.y & rhs.my + self.z & rhs.mz  (unnamed type)

// ---------------------------------------------------------------------
// Rotor3 OP Plane:

// Omitted: Rotor3 geometric Plane = self.w * rhs.d + self.x * rhs.d + self.y * rhs.d + self.z * rhs.d  (unnamed type)
// Omitted: Rotor3 anti_geometric Plane = self.w !* rhs.d + self.w !* rhs.nx + self.w !* rhs.ny + self.w !* rhs.nz + self.x !* rhs.d + self.x !* rhs.nx + self.x !* rhs.ny + self.x !* rhs.nz + self.y !* rhs.d + self.y !* rhs.nx + self.y !* rhs.ny + self.y !* rhs.nz + self.z !* rhs.d + self.z !* rhs.nx + self.z !* rhs.ny + self.z !* rhs.nz  (unnamed type)

// Rotor3.dot(Plane) -> W
impl Dot<Plane> for Rotor3 {
	type Output = W;
	fn dot(self, rhs: Plane) -> Self::Output {
		// -W(self.w.0 * rhs.d.0)
		self.w.dot(rhs.d)
	}
}

// Omitted: Rotor3 wedge Plane = 0  (unnamed type)
// Omitted: Rotor3 anti_wedge Plane = self.w & rhs.d + self.w & rhs.nx + self.w & rhs.ny + self.w & rhs.nz + self.x & rhs.d + self.x & rhs.nx + self.y & rhs.d + self.y & rhs.ny + self.z & rhs.d + self.z & rhs.nz  (unnamed type)

// ---------------------------------------------------------------------
// Rotor3 OP Rotor3:

// Omitted: Rotor3 geometric Rotor3 = 0  (unnamed type)

// Rotor3.anti_geometric(Rotor3) -> Rotor3
impl AntiGeometric<Rotor3> for Rotor3 {
	type Output = Rotor3;
	fn anti_geometric(self, rhs: Rotor3) -> Self::Output {
		// Rotor3 {
		//     x: WX(self.w.0 * rhs.x.0) + WX(self.x.0 * rhs.w.0) + WX(self.y.0 * rhs.z.0) + WX(self.z.0 * rhs.y.0),
		//     y: WY(self.w.0 * rhs.y.0) + WY(self.x.0 * rhs.z.0) + WY(self.y.0 * rhs.w.0) + WY(self.z.0 * rhs.x.0),
		//     z: WZ(self.w.0 * rhs.z.0) + WZ(self.x.0 * rhs.y.0) + WZ(self.y.0 * rhs.x.0) + WZ(self.z.0 * rhs.w.0),
		//     w: XYZW(self.w.0 * rhs.w.0) + XYZW(self.x.0 * rhs.x.0) + XYZW(self.y.0 * rhs.y.0) + XYZW(self.z.0 * rhs.z.0),
		// }
		Rotor3 {
			x: self.w.anti_geometric(rhs.x) + self.x.anti_geometric(rhs.w) + self.y.anti_geometric(rhs.z)
				- self.z.anti_geometric(rhs.y),
			y: self.w.anti_geometric(rhs.y) - self.x.anti_geometric(rhs.z)
				+ self.y.anti_geometric(rhs.w)
				+ self.z.anti_geometric(rhs.x),
			z: self.w.anti_geometric(rhs.z) + self.x.anti_geometric(rhs.y) - self.y.anti_geometric(rhs.x)
				+ self.z.anti_geometric(rhs.w),
			w: self.w.anti_geometric(rhs.w)
				- self.x.anti_geometric(rhs.x)
				- self.y.anti_geometric(rhs.y)
				- self.z.anti_geometric(rhs.z),
		}
	}
}

// Omitted: Rotor3 dot Rotor3 = 0  (unnamed type)
// Omitted: Rotor3 wedge Rotor3 = 0  (unnamed type)

// Rotor3.anti_wedge(Rotor3) -> Rotor3
impl AntiWedge<Rotor3> for Rotor3 {
	type Output = Rotor3;
	fn anti_wedge(self, rhs: Rotor3) -> Self::Output {
		// Rotor3 {
		//     x: WX(self.w.0 * rhs.x.0) + WX(self.x.0 * rhs.w.0),
		//     y: WY(self.w.0 * rhs.y.0) + WY(self.y.0 * rhs.w.0),
		//     z: WZ(self.w.0 * rhs.z.0) + WZ(self.z.0 * rhs.w.0),
		//     w: XYZW(self.w.0 * rhs.w.0),
		// }
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
		// Rotor3 {
		//     x: WX(self.x.0 * rhs.uw.0),
		//     y: WY(self.y.0 * rhs.uw.0),
		//     z: WZ(self.z.0 * rhs.uw.0),
		//     w: XYZW(self.w.0 * rhs.uw.0),
		// }
		Rotor3 {
			x: self.x.geometric(rhs.uw),
			y: self.y.geometric(rhs.uw),
			z: self.z.geometric(rhs.uw),
			w: self.w.geometric(rhs.uw),
		}
	}
}

// Omitted: Rotor3 anti_geometric Motor3 = self.w !* rhs.rw + self.w !* rhs.rx + self.w !* rhs.ry + self.w !* rhs.rz + self.w !* rhs.uw + self.w !* rhs.ux + self.w !* rhs.uy + self.w !* rhs.uz + self.x !* rhs.rw + self.x !* rhs.rx + self.x !* rhs.ry + self.x !* rhs.rz + self.x !* rhs.uw + self.x !* rhs.ux + self.x !* rhs.uy + self.x !* rhs.uz + self.y !* rhs.rw + self.y !* rhs.rx + self.y !* rhs.ry + self.y !* rhs.rz + self.y !* rhs.uw + self.y !* rhs.ux + self.y !* rhs.uy + self.y !* rhs.uz + self.z !* rhs.rw + self.z !* rhs.rx + self.z !* rhs.ry + self.z !* rhs.rz + self.z !* rhs.uw + self.z !* rhs.ux + self.z !* rhs.uy + self.z !* rhs.uz  (unnamed type)

// Rotor3.dot(Motor3) -> Rotor3
impl Dot<Motor3> for Rotor3 {
	type Output = Rotor3;
	fn dot(self, rhs: Motor3) -> Self::Output {
		// Rotor3 {
		//     x: WX(self.x.0 * rhs.uw.0),
		//     y: WY(self.y.0 * rhs.uw.0),
		//     z: WZ(self.z.0 * rhs.uw.0),
		//     w: XYZW(self.w.0 * rhs.uw.0),
		// }
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
		// Rotor3 {
		//     x: WX(self.x.0 * rhs.uw.0),
		//     y: WY(self.y.0 * rhs.uw.0),
		//     z: WZ(self.z.0 * rhs.uw.0),
		//     w: XYZW(self.w.0 * rhs.uw.0),
		// }
		Rotor3 {
			x: self.x.wedge(rhs.uw),
			y: self.y.wedge(rhs.uw),
			z: self.z.wedge(rhs.uw),
			w: self.w.wedge(rhs.uw),
		}
	}
}

// Omitted: Rotor3 anti_wedge Motor3 = self.w & rhs.rw + self.w & rhs.rx + self.w & rhs.ry + self.w & rhs.rz + self.w & rhs.uw + self.w & rhs.ux + self.w & rhs.uy + self.w & rhs.uz + self.x & rhs.rw + self.x & rhs.ux + self.y & rhs.rw + self.y & rhs.uy + self.z & rhs.rw + self.z & rhs.uz  (unnamed type)

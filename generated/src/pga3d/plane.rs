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
//! Plane.dot(Moment3) -> Vec4
//! Moment3.dot(Plane) -> Vec4
//! Plane.anti_wedge(Moment3) -> Vec3
//! Moment3.anti_wedge(Plane) -> Vec3
//! Plane.dot(Line3) -> Vec4
//! Line3.dot(Plane) -> Vec4
//! Plane.anti_wedge(Line3) -> Vec4
//! Line3.anti_wedge(Plane) -> Vec4
//! Plane.dot(Rotor3) -> W
//! Rotor3.dot(Plane) -> W
//! Plane.wedge(Motor3) -> Plane
//! Motor3.wedge(Plane) -> Plane
//! ```

use super::*;

#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, derive_more::Neg, derive_more::Add, derive_more::Sub)]
pub struct Plane {
	pub nx: YZW,
	pub ny: ZXW,
	pub nz: XYW,
	pub d: XYZ,
}

// ---------------------------------------------------------------------

impl RCompl for Plane {
	type Output = Vec4;
	fn rcompl(self) -> Self::Output {
		Vec4 {
			x: -self.nx.rcompl(),
			y: -self.ny.rcompl(),
			z: -self.nz.rcompl(),
			w: self.d.rcompl(),
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
			w: -self.d.lcompl(),
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

// Omitted: Plane geometric Vec3 = self.d * rhs.x + self.d * rhs.y + self.d * rhs.z + self.nx * rhs.x + self.nx * rhs.y + self.nx * rhs.z + self.ny * rhs.x + self.ny * rhs.y + self.ny * rhs.z + self.nz * rhs.x + self.nz * rhs.y + self.nz * rhs.z  (unnamed type)
// Omitted: Plane anti_geometric Vec3 = self.nx !* rhs.x + self.nx !* rhs.y + self.nx !* rhs.z + self.ny !* rhs.x + self.ny !* rhs.y + self.ny !* rhs.z + self.nz !* rhs.x + self.nz !* rhs.y + self.nz !* rhs.z  (unnamed type)

// Plane.dot(Vec3) -> Line3
impl Dot<Vec3> for Plane {
	type Output = Line3;
	fn dot(self, rhs: Vec3) -> Self::Output {
		// Line3 {
		//     vx: WX(self.ny.0 * rhs.z.0) + WX(self.nz.0 * rhs.y.0),
		//     vy: WY(self.nx.0 * rhs.z.0) + WY(self.nz.0 * rhs.x.0),
		//     vz: WZ(self.nx.0 * rhs.y.0) + WZ(self.ny.0 * rhs.x.0),
		//     mx: YZ(self.d.0 * rhs.x.0),
		//     my: ZX(self.d.0 * rhs.y.0),
		//     mz: XY(self.d.0 * rhs.z.0),
		// }
		Line3 {
			vx: -self.ny.dot(rhs.z) + self.nz.dot(rhs.y),
			vy: self.nx.dot(rhs.z) - self.nz.dot(rhs.x),
			vz: -self.nx.dot(rhs.y) + self.ny.dot(rhs.x),
			mx: self.d.dot(rhs.x),
			my: self.d.dot(rhs.y),
			mz: self.d.dot(rhs.z),
		}
	}
}

// Plane.wedge(Vec3) -> XYZW
impl Wedge<Vec3> for Plane {
	type Output = XYZW;
	fn wedge(self, rhs: Vec3) -> Self::Output {
		// -XYZW(self.nx.0 * rhs.x.0) - XYZW(self.ny.0 * rhs.y.0) - XYZW(self.nz.0 * rhs.z.0)
		self.nx.wedge(rhs.x) + self.ny.wedge(rhs.y) + self.nz.wedge(rhs.z)
	}
}

// Plane.anti_wedge(Vec3) -> S
impl AntiWedge<Vec3> for Plane {
	type Output = S;
	fn anti_wedge(self, rhs: Vec3) -> Self::Output {
		// -S(self.nx.0 * rhs.x.0) - S(self.ny.0 * rhs.y.0) - S(self.nz.0 * rhs.z.0)
		self.nx.anti_wedge(rhs.x) + self.ny.anti_wedge(rhs.y) + self.nz.anti_wedge(rhs.z)
	}
}

// ---------------------------------------------------------------------
// Plane OP Vec4:

// Omitted: Plane geometric Vec4 = self.d * rhs.w + self.d * rhs.x + self.d * rhs.y + self.d * rhs.z + self.nx * rhs.x + self.nx * rhs.y + self.nx * rhs.z + self.ny * rhs.x + self.ny * rhs.y + self.ny * rhs.z + self.nz * rhs.x + self.nz * rhs.y + self.nz * rhs.z  (unnamed type)
// Omitted: Plane anti_geometric Vec4 = self.d !* rhs.w + self.nx !* rhs.w + self.nx !* rhs.x + self.nx !* rhs.y + self.nx !* rhs.z + self.ny !* rhs.w + self.ny !* rhs.x + self.ny !* rhs.y + self.ny !* rhs.z + self.nz !* rhs.w + self.nz !* rhs.x + self.nz !* rhs.y + self.nz !* rhs.z  (unnamed type)

// Plane.dot(Vec4) -> Line3
impl Dot<Vec4> for Plane {
	type Output = Line3;
	fn dot(self, rhs: Vec4) -> Self::Output {
		// Line3 {
		//     vx: WX(self.ny.0 * rhs.z.0) + WX(self.nz.0 * rhs.y.0),
		//     vy: WY(self.nx.0 * rhs.z.0) + WY(self.nz.0 * rhs.x.0),
		//     vz: WZ(self.nx.0 * rhs.y.0) + WZ(self.ny.0 * rhs.x.0),
		//     mx: YZ(self.d.0 * rhs.x.0),
		//     my: ZX(self.d.0 * rhs.y.0),
		//     mz: XY(self.d.0 * rhs.z.0),
		// }
		Line3 {
			vx: -self.ny.dot(rhs.z) + self.nz.dot(rhs.y),
			vy: self.nx.dot(rhs.z) - self.nz.dot(rhs.x),
			vz: -self.nx.dot(rhs.y) + self.ny.dot(rhs.x),
			mx: self.d.dot(rhs.x),
			my: self.d.dot(rhs.y),
			mz: self.d.dot(rhs.z),
		}
	}
}

// Omitted: Plane wedge Vec4 = self.d ^ rhs.w + self.nx ^ rhs.x + self.ny ^ rhs.y + self.nz ^ rhs.z  (unnamed type)
// Omitted: Plane anti_wedge Vec4 = self.d & rhs.w + self.nx & rhs.x + self.ny & rhs.y + self.nz & rhs.z  (unnamed type)

// ---------------------------------------------------------------------
// Plane OP Moment3:

// Omitted: Plane geometric Moment3 = self.d * rhs.mx + self.d * rhs.my + self.d * rhs.mz + self.nx * rhs.mx + self.nx * rhs.my + self.nx * rhs.mz + self.ny * rhs.mx + self.ny * rhs.my + self.ny * rhs.mz + self.nz * rhs.mx + self.nz * rhs.my + self.nz * rhs.mz  (unnamed type)
// Omitted: Plane anti_geometric Moment3 = self.nx !* rhs.mx + self.nx !* rhs.my + self.nx !* rhs.mz + self.ny !* rhs.mx + self.ny !* rhs.my + self.ny !* rhs.mz + self.nz !* rhs.mx + self.nz !* rhs.my + self.nz !* rhs.mz  (unnamed type)

// Plane.dot(Moment3) -> Vec4
impl Dot<Moment3> for Plane {
	type Output = Vec4;
	fn dot(self, rhs: Moment3) -> Self::Output {
		// Vec4 {
		//     x: X(self.d.0 * rhs.mx.0),
		//     y: Y(self.d.0 * rhs.my.0),
		//     z: Z(self.d.0 * rhs.mz.0),
		//     w: W(self.nx.0 * rhs.mx.0) + W(self.ny.0 * rhs.my.0) + W(self.nz.0 * rhs.mz.0),
		// }
		Vec4 {
			x: -self.d.dot(rhs.mx),
			y: -self.d.dot(rhs.my),
			z: -self.d.dot(rhs.mz),
			w: -self.nx.dot(rhs.mx) - self.ny.dot(rhs.my) - self.nz.dot(rhs.mz),
		}
	}
}

// Omitted: Plane wedge Moment3 = 0  (unnamed type)

// Plane.anti_wedge(Moment3) -> Vec3
impl AntiWedge<Moment3> for Plane {
	type Output = Vec3;
	fn anti_wedge(self, rhs: Moment3) -> Self::Output {
		// Vec3 {
		//     x: X(self.ny.0 * rhs.mz.0) + X(self.nz.0 * rhs.my.0),
		//     y: Y(self.nx.0 * rhs.mz.0) + Y(self.nz.0 * rhs.mx.0),
		//     z: Z(self.nx.0 * rhs.my.0) + Z(self.ny.0 * rhs.mx.0),
		// }
		Vec3 {
			x: -self.ny.anti_wedge(rhs.mz) + self.nz.anti_wedge(rhs.my),
			y: self.nx.anti_wedge(rhs.mz) - self.nz.anti_wedge(rhs.mx),
			z: -self.nx.anti_wedge(rhs.my) + self.ny.anti_wedge(rhs.mx),
		}
	}
}

// ---------------------------------------------------------------------
// Plane OP Line3:

// Omitted: Plane geometric Line3 = self.d * rhs.mx + self.d * rhs.my + self.d * rhs.mz + self.d * rhs.vx + self.d * rhs.vy + self.d * rhs.vz + self.nx * rhs.mx + self.nx * rhs.my + self.nx * rhs.mz + self.ny * rhs.mx + self.ny * rhs.my + self.ny * rhs.mz + self.nz * rhs.mx + self.nz * rhs.my + self.nz * rhs.mz  (unnamed type)
// Omitted: Plane anti_geometric Line3 = self.d !* rhs.vx + self.d !* rhs.vy + self.d !* rhs.vz + self.nx !* rhs.mx + self.nx !* rhs.my + self.nx !* rhs.mz + self.nx !* rhs.vx + self.nx !* rhs.vy + self.nx !* rhs.vz + self.ny !* rhs.mx + self.ny !* rhs.my + self.ny !* rhs.mz + self.ny !* rhs.vx + self.ny !* rhs.vy + self.ny !* rhs.vz + self.nz !* rhs.mx + self.nz !* rhs.my + self.nz !* rhs.mz + self.nz !* rhs.vx + self.nz !* rhs.vy + self.nz !* rhs.vz  (unnamed type)

// Plane.dot(Line3) -> Vec4
impl Dot<Line3> for Plane {
	type Output = Vec4;
	fn dot(self, rhs: Line3) -> Self::Output {
		// Vec4 {
		//     x: X(self.d.0 * rhs.mx.0),
		//     y: Y(self.d.0 * rhs.my.0),
		//     z: Z(self.d.0 * rhs.mz.0),
		//     w: W(self.nx.0 * rhs.mx.0) + W(self.ny.0 * rhs.my.0) + W(self.nz.0 * rhs.mz.0),
		// }
		Vec4 {
			x: -self.d.dot(rhs.mx),
			y: -self.d.dot(rhs.my),
			z: -self.d.dot(rhs.mz),
			w: -self.nx.dot(rhs.mx) - self.ny.dot(rhs.my) - self.nz.dot(rhs.mz),
		}
	}
}

// Omitted: Plane wedge Line3 = 0  (unnamed type)

// Plane.anti_wedge(Line3) -> Vec4
impl AntiWedge<Line3> for Plane {
	type Output = Vec4;
	fn anti_wedge(self, rhs: Line3) -> Self::Output {
		// Vec4 {
		//     x: X(self.d.0 * rhs.vx.0) + X(self.ny.0 * rhs.mz.0) + X(self.nz.0 * rhs.my.0),
		//     y: Y(self.d.0 * rhs.vy.0) + Y(self.nx.0 * rhs.mz.0) + Y(self.nz.0 * rhs.mx.0),
		//     z: Z(self.d.0 * rhs.vz.0) + Z(self.nx.0 * rhs.my.0) + Z(self.ny.0 * rhs.mx.0),
		//     w: W(self.nx.0 * rhs.vx.0) + W(self.ny.0 * rhs.vy.0) + W(self.nz.0 * rhs.vz.0),
		// }
		Vec4 {
			x: -self.d.anti_wedge(rhs.vx) - self.ny.anti_wedge(rhs.mz) + self.nz.anti_wedge(rhs.my),
			y: -self.d.anti_wedge(rhs.vy) + self.nx.anti_wedge(rhs.mz) - self.nz.anti_wedge(rhs.mx),
			z: -self.d.anti_wedge(rhs.vz) - self.nx.anti_wedge(rhs.my) + self.ny.anti_wedge(rhs.mx),
			w: -self.nx.anti_wedge(rhs.vx) - self.ny.anti_wedge(rhs.vy) - self.nz.anti_wedge(rhs.vz),
		}
	}
}

// ---------------------------------------------------------------------
// Plane OP Plane:

// Omitted: Plane geometric Plane = Motor3 {     rx: self.d * rhs.nx - self.nx * rhs.d,     ry: self.d * rhs.ny - self.ny * rhs.d,     rz: self.d * rhs.nz - self.nz * rhs.d,     rw: 0,     ux: 0,     uy: 0,     uz: 0,     uw: -self.d * rhs.d, }  (too many zeros)
// Omitted: Plane anti_geometric Plane = self.d !* rhs.nx + self.d !* rhs.ny + self.d !* rhs.nz + self.nx !* rhs.d + self.nx !* rhs.nx + self.nx !* rhs.ny + self.nx !* rhs.nz + self.ny !* rhs.d + self.ny !* rhs.nx + self.ny !* rhs.ny + self.ny !* rhs.nz + self.nz !* rhs.d + self.nz !* rhs.nx + self.nz !* rhs.ny + self.nz !* rhs.nz  (unnamed type)

// Plane.dot(Plane) -> S
impl Dot<Plane> for Plane {
	type Output = S;
	fn dot(self, rhs: Plane) -> Self::Output {
		// -S(self.d.0 * rhs.d.0)
		self.d.dot(rhs.d)
	}
}

// Omitted: Plane wedge Plane = 0  (unnamed type)

// Plane.anti_wedge(Plane) -> Line3
impl AntiWedge<Plane> for Plane {
	type Output = Line3;
	fn anti_wedge(self, rhs: Plane) -> Self::Output {
		// Line3 {
		//     vx: WX(self.ny.0 * rhs.nz.0) + WX(self.nz.0 * rhs.ny.0),
		//     vy: WY(self.nx.0 * rhs.nz.0) + WY(self.nz.0 * rhs.nx.0),
		//     vz: WZ(self.nx.0 * rhs.ny.0) + WZ(self.ny.0 * rhs.nx.0),
		//     mx: YZ(self.d.0 * rhs.nx.0) + YZ(self.nx.0 * rhs.d.0),
		//     my: ZX(self.d.0 * rhs.ny.0) + ZX(self.ny.0 * rhs.d.0),
		//     mz: XY(self.d.0 * rhs.nz.0) + XY(self.nz.0 * rhs.d.0),
		// }
		Line3 {
			vx: -self.ny.anti_wedge(rhs.nz) + self.nz.anti_wedge(rhs.ny),
			vy: self.nx.anti_wedge(rhs.nz) - self.nz.anti_wedge(rhs.nx),
			vz: -self.nx.anti_wedge(rhs.ny) + self.ny.anti_wedge(rhs.nx),
			mx: self.d.anti_wedge(rhs.nx) - self.nx.anti_wedge(rhs.d),
			my: self.d.anti_wedge(rhs.ny) - self.ny.anti_wedge(rhs.d),
			mz: self.d.anti_wedge(rhs.nz) - self.nz.anti_wedge(rhs.d),
		}
	}
}

// ---------------------------------------------------------------------
// Plane OP Rotor3:

// Omitted: Plane geometric Rotor3 = self.d * rhs.w + self.d * rhs.x + self.d * rhs.y + self.d * rhs.z  (unnamed type)
// Omitted: Plane anti_geometric Rotor3 = self.d !* rhs.w + self.d !* rhs.x + self.d !* rhs.y + self.d !* rhs.z + self.nx !* rhs.w + self.nx !* rhs.x + self.nx !* rhs.y + self.nx !* rhs.z + self.ny !* rhs.w + self.ny !* rhs.x + self.ny !* rhs.y + self.ny !* rhs.z + self.nz !* rhs.w + self.nz !* rhs.x + self.nz !* rhs.y + self.nz !* rhs.z  (unnamed type)

// Plane.dot(Rotor3) -> W
impl Dot<Rotor3> for Plane {
	type Output = W;
	fn dot(self, rhs: Rotor3) -> Self::Output {
		// -W(self.d.0 * rhs.w.0)
		self.d.dot(rhs.w)
	}
}

// Omitted: Plane wedge Rotor3 = 0  (unnamed type)
// Omitted: Plane anti_wedge Rotor3 = self.d & rhs.w + self.d & rhs.x + self.d & rhs.y + self.d & rhs.z + self.nx & rhs.w + self.nx & rhs.x + self.ny & rhs.w + self.ny & rhs.y + self.nz & rhs.w + self.nz & rhs.z  (unnamed type)

// ---------------------------------------------------------------------
// Plane OP Motor3:

// Omitted: Plane geometric Motor3 = self.d * rhs.rw + self.d * rhs.rx + self.d * rhs.ry + self.d * rhs.rz + self.d * rhs.uw + self.d * rhs.ux + self.d * rhs.uy + self.d * rhs.uz + self.nx * rhs.uw + self.ny * rhs.uw + self.nz * rhs.uw  (unnamed type)
// Omitted: Plane anti_geometric Motor3 = self.d !* rhs.rw + self.d !* rhs.rx + self.d !* rhs.ry + self.d !* rhs.rz + self.d !* rhs.ux + self.d !* rhs.uy + self.d !* rhs.uz + self.nx !* rhs.rw + self.nx !* rhs.rx + self.nx !* rhs.ry + self.nx !* rhs.rz + self.nx !* rhs.uw + self.nx !* rhs.ux + self.nx !* rhs.uy + self.nx !* rhs.uz + self.ny !* rhs.rw + self.ny !* rhs.rx + self.ny !* rhs.ry + self.ny !* rhs.rz + self.ny !* rhs.uw + self.ny !* rhs.ux + self.ny !* rhs.uy + self.ny !* rhs.uz + self.nz !* rhs.rw + self.nz !* rhs.rx + self.nz !* rhs.ry + self.nz !* rhs.rz + self.nz !* rhs.uw + self.nz !* rhs.ux + self.nz !* rhs.uy + self.nz !* rhs.uz  (unnamed type)
// Omitted: Plane dot Motor3 = self.d | rhs.rw + self.d | rhs.uw + self.nx | rhs.uw + self.ny | rhs.uw + self.nz | rhs.uw  (unnamed type)

// Plane.wedge(Motor3) -> Plane
impl Wedge<Motor3> for Plane {
	type Output = Plane;
	fn wedge(self, rhs: Motor3) -> Self::Output {
		// Plane {
		//     nx: YZW(self.nx.0 * rhs.uw.0),
		//     ny: ZXW(self.ny.0 * rhs.uw.0),
		//     nz: XYW(self.nz.0 * rhs.uw.0),
		//     d : XYZ(self.d.0 * rhs.uw.0),
		// }
		Plane {
			nx: self.nx.wedge(rhs.uw),
			ny: self.ny.wedge(rhs.uw),
			nz: self.nz.wedge(rhs.uw),
			d: self.d.wedge(rhs.uw),
		}
	}
}

// Omitted: Plane anti_wedge Motor3 = self.d & rhs.rw + self.d & rhs.rx + self.d & rhs.ry + self.d & rhs.rz + self.d & rhs.ux + self.d & rhs.uy + self.d & rhs.uz + self.nx & rhs.rw + self.nx & rhs.rx + self.nx & rhs.uy + self.nx & rhs.uz + self.ny & rhs.rw + self.ny & rhs.ry + self.ny & rhs.ux + self.ny & rhs.uz + self.nz & rhs.rw + self.nz & rhs.rz + self.nz & rhs.ux + self.nz & rhs.uy  (unnamed type)

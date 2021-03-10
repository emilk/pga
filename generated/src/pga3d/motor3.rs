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
//!
//! Motor3.wedge(Plane) -> Plane
//! Plane.wedge(Motor3) -> Plane
//! Motor3.geometric(Rotor3) -> Rotor3
//! Rotor3.geometric(Motor3) -> Rotor3
//! Motor3.dot(Rotor3) -> Rotor3
//! Rotor3.dot(Motor3) -> Rotor3
//! Motor3.wedge(Rotor3) -> Rotor3
//! Rotor3.wedge(Motor3) -> Rotor3
//! ```

use super::*;

#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, derive_more::Neg, derive_more::Add, derive_more::Sub)]
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

// Omitted: Motor3 geometric Vec3 = self.rw * rhs.x + self.rw * rhs.y + self.rw * rhs.z + self.rx * rhs.x + self.rx * rhs.y + self.rx * rhs.z + self.ry * rhs.x + self.ry * rhs.y + self.ry * rhs.z + self.rz * rhs.x + self.rz * rhs.y + self.rz * rhs.z + self.uw * rhs.x + self.uw * rhs.y + self.uw * rhs.z + self.ux * rhs.x + self.ux * rhs.y + self.ux * rhs.z + self.uy * rhs.x + self.uy * rhs.y + self.uy * rhs.z + self.uz * rhs.x + self.uz * rhs.y + self.uz * rhs.z  (unnamed type)
// Omitted: Motor3 anti_geometric Vec3 = self.rw !* rhs.x + self.rw !* rhs.y + self.rw !* rhs.z + self.rx !* rhs.x + self.rx !* rhs.y + self.rx !* rhs.z + self.ry !* rhs.x + self.ry !* rhs.y + self.ry !* rhs.z + self.rz !* rhs.x + self.rz !* rhs.y + self.rz !* rhs.z + self.ux !* rhs.x + self.ux !* rhs.y + self.ux !* rhs.z + self.uy !* rhs.x + self.uy !* rhs.y + self.uy !* rhs.z + self.uz !* rhs.x + self.uz !* rhs.y + self.uz !* rhs.z  (unnamed type)
// Omitted: Motor3 dot Vec3 = self.rw | rhs.x + self.rw | rhs.y + self.rw | rhs.z + self.rx | rhs.x + self.ry | rhs.y + self.rz | rhs.z + self.uw | rhs.x + self.uw | rhs.y + self.uw | rhs.z + self.ux | rhs.y + self.ux | rhs.z + self.uy | rhs.x + self.uy | rhs.z + self.uz | rhs.x + self.uz | rhs.y  (unnamed type)
// Omitted: Motor3 wedge Vec3 = self.rx ^ rhs.y + self.rx ^ rhs.z + self.ry ^ rhs.x + self.ry ^ rhs.z + self.rz ^ rhs.x + self.rz ^ rhs.y + self.uw ^ rhs.x + self.uw ^ rhs.y + self.uw ^ rhs.z + self.ux ^ rhs.x + self.uy ^ rhs.y + self.uz ^ rhs.z  (unnamed type)
// Omitted: Motor3 anti_wedge Vec3 = self.rw & rhs.x + self.rw & rhs.y + self.rw & rhs.z + self.ux & rhs.x + self.uy & rhs.y + self.uz & rhs.z  (unnamed type)

// ---------------------------------------------------------------------
// Motor3 OP Vec4:

// Omitted: Motor3 geometric Vec4 = self.rw * rhs.x + self.rw * rhs.y + self.rw * rhs.z + self.rx * rhs.x + self.rx * rhs.y + self.rx * rhs.z + self.ry * rhs.x + self.ry * rhs.y + self.ry * rhs.z + self.rz * rhs.x + self.rz * rhs.y + self.rz * rhs.z + self.uw * rhs.w + self.uw * rhs.x + self.uw * rhs.y + self.uw * rhs.z + self.ux * rhs.x + self.ux * rhs.y + self.ux * rhs.z + self.uy * rhs.x + self.uy * rhs.y + self.uy * rhs.z + self.uz * rhs.x + self.uz * rhs.y + self.uz * rhs.z  (unnamed type)
// Omitted: Motor3 anti_geometric Vec4 = self.rw !* rhs.w + self.rw !* rhs.x + self.rw !* rhs.y + self.rw !* rhs.z + self.rx !* rhs.w + self.rx !* rhs.x + self.rx !* rhs.y + self.rx !* rhs.z + self.ry !* rhs.w + self.ry !* rhs.x + self.ry !* rhs.y + self.ry !* rhs.z + self.rz !* rhs.w + self.rz !* rhs.x + self.rz !* rhs.y + self.rz !* rhs.z + self.uw !* rhs.w + self.ux !* rhs.w + self.ux !* rhs.x + self.ux !* rhs.y + self.ux !* rhs.z + self.uy !* rhs.w + self.uy !* rhs.x + self.uy !* rhs.y + self.uy !* rhs.z + self.uz !* rhs.w + self.uz !* rhs.x + self.uz !* rhs.y + self.uz !* rhs.z  (unnamed type)
// Omitted: Motor3 dot Vec4 = self.rw | rhs.x + self.rw | rhs.y + self.rw | rhs.z + self.rx | rhs.x + self.ry | rhs.y + self.rz | rhs.z + self.uw | rhs.w + self.uw | rhs.x + self.uw | rhs.y + self.uw | rhs.z + self.ux | rhs.y + self.ux | rhs.z + self.uy | rhs.x + self.uy | rhs.z + self.uz | rhs.x + self.uz | rhs.y  (unnamed type)
// Omitted: Motor3 wedge Vec4 = self.rx ^ rhs.y + self.rx ^ rhs.z + self.ry ^ rhs.x + self.ry ^ rhs.z + self.rz ^ rhs.x + self.rz ^ rhs.y + self.uw ^ rhs.w + self.uw ^ rhs.x + self.uw ^ rhs.y + self.uw ^ rhs.z + self.ux ^ rhs.x + self.uy ^ rhs.y + self.uz ^ rhs.z  (unnamed type)
// Omitted: Motor3 anti_wedge Vec4 = self.rw & rhs.w + self.rw & rhs.x + self.rw & rhs.y + self.rw & rhs.z + self.ux & rhs.x + self.uy & rhs.y + self.uz & rhs.z  (unnamed type)

// ---------------------------------------------------------------------
// Motor3 OP Moment3:

// Omitted: Motor3 geometric Moment3 = self.rw * rhs.mx + self.rw * rhs.my + self.rw * rhs.mz + self.rx * rhs.mx + self.rx * rhs.my + self.rx * rhs.mz + self.ry * rhs.mx + self.ry * rhs.my + self.ry * rhs.mz + self.rz * rhs.mx + self.rz * rhs.my + self.rz * rhs.mz + self.uw * rhs.mx + self.uw * rhs.my + self.uw * rhs.mz + self.ux * rhs.mx + self.ux * rhs.my + self.ux * rhs.mz + self.uy * rhs.mx + self.uy * rhs.my + self.uy * rhs.mz + self.uz * rhs.mx + self.uz * rhs.my + self.uz * rhs.mz  (unnamed type)
// Omitted: Motor3 anti_geometric Moment3 = self.rw !* rhs.mx + self.rw !* rhs.my + self.rw !* rhs.mz + self.rx !* rhs.mx + self.rx !* rhs.my + self.rx !* rhs.mz + self.ry !* rhs.mx + self.ry !* rhs.my + self.ry !* rhs.mz + self.rz !* rhs.mx + self.rz !* rhs.my + self.rz !* rhs.mz + self.ux !* rhs.mx + self.ux !* rhs.my + self.ux !* rhs.mz + self.uy !* rhs.mx + self.uy !* rhs.my + self.uy !* rhs.mz + self.uz !* rhs.mx + self.uz !* rhs.my + self.uz !* rhs.mz  (unnamed type)
// Omitted: Motor3 dot Moment3 = self.rw | rhs.mx + self.rw | rhs.my + self.rw | rhs.mz + self.uw | rhs.mx + self.uw | rhs.my + self.uw | rhs.mz + self.ux | rhs.mx + self.uy | rhs.my + self.uz | rhs.mz  (unnamed type)
// Omitted: Motor3 wedge Moment3 = self.rx ^ rhs.mx + self.ry ^ rhs.my + self.rz ^ rhs.mz + self.uw ^ rhs.mx + self.uw ^ rhs.my + self.uw ^ rhs.mz  (unnamed type)
// Omitted: Motor3 anti_wedge Moment3 = self.rw & rhs.mx + self.rw & rhs.my + self.rw & rhs.mz + self.rx & rhs.mx + self.ry & rhs.my + self.rz & rhs.mz + self.ux & rhs.my + self.ux & rhs.mz + self.uy & rhs.mx + self.uy & rhs.mz + self.uz & rhs.mx + self.uz & rhs.my  (unnamed type)

// ---------------------------------------------------------------------
// Motor3 OP Line3:

// Omitted: Motor3 geometric Line3 = self.rw * rhs.mx + self.rw * rhs.my + self.rw * rhs.mz + self.rx * rhs.mx + self.rx * rhs.my + self.rx * rhs.mz + self.ry * rhs.mx + self.ry * rhs.my + self.ry * rhs.mz + self.rz * rhs.mx + self.rz * rhs.my + self.rz * rhs.mz + self.uw * rhs.mx + self.uw * rhs.my + self.uw * rhs.mz + self.uw * rhs.vx + self.uw * rhs.vy + self.uw * rhs.vz + self.ux * rhs.mx + self.ux * rhs.my + self.ux * rhs.mz + self.uy * rhs.mx + self.uy * rhs.my + self.uy * rhs.mz + self.uz * rhs.mx + self.uz * rhs.my + self.uz * rhs.mz  (unnamed type)
// Omitted: Motor3 anti_geometric Line3 = self.rw !* rhs.mx + self.rw !* rhs.my + self.rw !* rhs.mz + self.rw !* rhs.vx + self.rw !* rhs.vy + self.rw !* rhs.vz + self.rx !* rhs.mx + self.rx !* rhs.my + self.rx !* rhs.mz + self.rx !* rhs.vx + self.rx !* rhs.vy + self.rx !* rhs.vz + self.ry !* rhs.mx + self.ry !* rhs.my + self.ry !* rhs.mz + self.ry !* rhs.vx + self.ry !* rhs.vy + self.ry !* rhs.vz + self.rz !* rhs.mx + self.rz !* rhs.my + self.rz !* rhs.mz + self.rz !* rhs.vx + self.rz !* rhs.vy + self.rz !* rhs.vz + self.uw !* rhs.vx + self.uw !* rhs.vy + self.uw !* rhs.vz + self.ux !* rhs.mx + self.ux !* rhs.my + self.ux !* rhs.mz + self.ux !* rhs.vx + self.ux !* rhs.vy + self.ux !* rhs.vz + self.uy !* rhs.mx + self.uy !* rhs.my + self.uy !* rhs.mz + self.uy !* rhs.vx + self.uy !* rhs.vy + self.uy !* rhs.vz + self.uz !* rhs.mx + self.uz !* rhs.my + self.uz !* rhs.mz + self.uz !* rhs.vx + self.uz !* rhs.vy + self.uz !* rhs.vz  (unnamed type)
// Omitted: Motor3 dot Line3 = self.rw | rhs.mx + self.rw | rhs.my + self.rw | rhs.mz + self.uw | rhs.mx + self.uw | rhs.my + self.uw | rhs.mz + self.uw | rhs.vx + self.uw | rhs.vy + self.uw | rhs.vz + self.ux | rhs.mx + self.uy | rhs.my + self.uz | rhs.mz  (unnamed type)
// Omitted: Motor3 wedge Line3 = self.rx ^ rhs.mx + self.ry ^ rhs.my + self.rz ^ rhs.mz + self.uw ^ rhs.mx + self.uw ^ rhs.my + self.uw ^ rhs.mz + self.uw ^ rhs.vx + self.uw ^ rhs.vy + self.uw ^ rhs.vz  (unnamed type)
// Omitted: Motor3 anti_wedge Line3 = self.rw & rhs.mx + self.rw & rhs.my + self.rw & rhs.mz + self.rw & rhs.vx + self.rw & rhs.vy + self.rw & rhs.vz + self.rx & rhs.mx + self.ry & rhs.my + self.rz & rhs.mz + self.ux & rhs.my + self.ux & rhs.mz + self.ux & rhs.vx + self.uy & rhs.mx + self.uy & rhs.mz + self.uy & rhs.vy + self.uz & rhs.mx + self.uz & rhs.my + self.uz & rhs.vz  (unnamed type)

// ---------------------------------------------------------------------
// Motor3 OP Plane:

// Omitted: Motor3 geometric Plane = self.rw * rhs.d + self.rx * rhs.d + self.ry * rhs.d + self.rz * rhs.d + self.uw * rhs.d + self.uw * rhs.nx + self.uw * rhs.ny + self.uw * rhs.nz + self.ux * rhs.d + self.uy * rhs.d + self.uz * rhs.d  (unnamed type)
// Omitted: Motor3 anti_geometric Plane = self.rw !* rhs.d + self.rw !* rhs.nx + self.rw !* rhs.ny + self.rw !* rhs.nz + self.rx !* rhs.d + self.rx !* rhs.nx + self.rx !* rhs.ny + self.rx !* rhs.nz + self.ry !* rhs.d + self.ry !* rhs.nx + self.ry !* rhs.ny + self.ry !* rhs.nz + self.rz !* rhs.d + self.rz !* rhs.nx + self.rz !* rhs.ny + self.rz !* rhs.nz + self.uw !* rhs.nx + self.uw !* rhs.ny + self.uw !* rhs.nz + self.ux !* rhs.d + self.ux !* rhs.nx + self.ux !* rhs.ny + self.ux !* rhs.nz + self.uy !* rhs.d + self.uy !* rhs.nx + self.uy !* rhs.ny + self.uy !* rhs.nz + self.uz !* rhs.d + self.uz !* rhs.nx + self.uz !* rhs.ny + self.uz !* rhs.nz  (unnamed type)
// Omitted: Motor3 dot Plane = self.rw | rhs.d + self.uw | rhs.d + self.uw | rhs.nx + self.uw | rhs.ny + self.uw | rhs.nz  (unnamed type)

// Motor3.wedge(Plane) -> Plane
impl Wedge<Plane> for Motor3 {
	type Output = Plane;
	fn wedge(self, rhs: Plane) -> Self::Output {
		// Plane {
		//     nx: YZW(self.uw.0 * rhs.nx.0),
		//     ny: ZXW(self.uw.0 * rhs.ny.0),
		//     nz: XYW(self.uw.0 * rhs.nz.0),
		//     d : XYZ(self.uw.0 * rhs.d.0),
		// }
		Plane {
			nx: self.uw.wedge(rhs.nx),
			ny: self.uw.wedge(rhs.ny),
			nz: self.uw.wedge(rhs.nz),
			d: self.uw.wedge(rhs.d),
		}
	}
}

// Omitted: Motor3 anti_wedge Plane = self.rw & rhs.d + self.rw & rhs.nx + self.rw & rhs.ny + self.rw & rhs.nz + self.rx & rhs.d + self.rx & rhs.nx + self.ry & rhs.d + self.ry & rhs.ny + self.rz & rhs.d + self.rz & rhs.nz + self.ux & rhs.d + self.ux & rhs.ny + self.ux & rhs.nz + self.uy & rhs.d + self.uy & rhs.nx + self.uy & rhs.nz + self.uz & rhs.d + self.uz & rhs.nx + self.uz & rhs.ny  (unnamed type)

// ---------------------------------------------------------------------
// Motor3 OP Rotor3:

// Motor3.geometric(Rotor3) -> Rotor3
impl Geometric<Rotor3> for Motor3 {
	type Output = Rotor3;
	fn geometric(self, rhs: Rotor3) -> Self::Output {
		// Rotor3 {
		//     x: WX(self.uw.0 * rhs.x.0),
		//     y: WY(self.uw.0 * rhs.y.0),
		//     z: WZ(self.uw.0 * rhs.z.0),
		//     w: XYZW(self.uw.0 * rhs.w.0),
		// }
		Rotor3 {
			x: self.uw.geometric(rhs.x),
			y: self.uw.geometric(rhs.y),
			z: self.uw.geometric(rhs.z),
			w: self.uw.geometric(rhs.w),
		}
	}
}

// Omitted: Motor3 anti_geometric Rotor3 = self.rw !* rhs.w + self.rw !* rhs.x + self.rw !* rhs.y + self.rw !* rhs.z + self.rx !* rhs.w + self.rx !* rhs.x + self.rx !* rhs.y + self.rx !* rhs.z + self.ry !* rhs.w + self.ry !* rhs.x + self.ry !* rhs.y + self.ry !* rhs.z + self.rz !* rhs.w + self.rz !* rhs.x + self.rz !* rhs.y + self.rz !* rhs.z + self.uw !* rhs.w + self.uw !* rhs.x + self.uw !* rhs.y + self.uw !* rhs.z + self.ux !* rhs.w + self.ux !* rhs.x + self.ux !* rhs.y + self.ux !* rhs.z + self.uy !* rhs.w + self.uy !* rhs.x + self.uy !* rhs.y + self.uy !* rhs.z + self.uz !* rhs.w + self.uz !* rhs.x + self.uz !* rhs.y + self.uz !* rhs.z  (unnamed type)

// Motor3.dot(Rotor3) -> Rotor3
impl Dot<Rotor3> for Motor3 {
	type Output = Rotor3;
	fn dot(self, rhs: Rotor3) -> Self::Output {
		// Rotor3 {
		//     x: WX(self.uw.0 * rhs.x.0),
		//     y: WY(self.uw.0 * rhs.y.0),
		//     z: WZ(self.uw.0 * rhs.z.0),
		//     w: XYZW(self.uw.0 * rhs.w.0),
		// }
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
		// Rotor3 {
		//     x: WX(self.uw.0 * rhs.x.0),
		//     y: WY(self.uw.0 * rhs.y.0),
		//     z: WZ(self.uw.0 * rhs.z.0),
		//     w: XYZW(self.uw.0 * rhs.w.0),
		// }
		Rotor3 {
			x: self.uw.wedge(rhs.x),
			y: self.uw.wedge(rhs.y),
			z: self.uw.wedge(rhs.z),
			w: self.uw.wedge(rhs.w),
		}
	}
}

// Omitted: Motor3 anti_wedge Rotor3 = self.rw & rhs.w + self.rw & rhs.x + self.rw & rhs.y + self.rw & rhs.z + self.rx & rhs.w + self.ry & rhs.w + self.rz & rhs.w + self.uw & rhs.w + self.ux & rhs.w + self.ux & rhs.x + self.uy & rhs.w + self.uy & rhs.y + self.uz & rhs.w + self.uz & rhs.z  (unnamed type)

// ---------------------------------------------------------------------
// Motor3 OP Motor3:

// Motor3.geometric(Motor3) -> Motor3
impl Geometric<Motor3> for Motor3 {
	type Output = Motor3;
	fn geometric(self, rhs: Motor3) -> Self::Output {
		// Motor3 {
		//     rx: WX(self.rx.0 * rhs.uw.0) + WX(self.uw.0 * rhs.rx.0),
		//     ry: WY(self.ry.0 * rhs.uw.0) + WY(self.uw.0 * rhs.ry.0),
		//     rz: WZ(self.rz.0 * rhs.uw.0) + WZ(self.uw.0 * rhs.rz.0),
		//     rw: XYZW(self.rw.0 * rhs.uw.0) + XYZW(self.uw.0 * rhs.rw.0),
		//     ux: YZW(self.uw.0 * rhs.ux.0) + YZW(self.ux.0 * rhs.uw.0),
		//     uy: ZXW(self.uw.0 * rhs.uy.0) + ZXW(self.uy.0 * rhs.uw.0),
		//     uz: XYW(self.uw.0 * rhs.uz.0) + XYW(self.uz.0 * rhs.uw.0),
		//     uw: S(self.uw.0 * rhs.uw.0),
		// }
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

// Omitted: Motor3 anti_geometric Motor3 = self.rw !* rhs.rw + self.rw !* rhs.rx + self.rw !* rhs.ry + self.rw !* rhs.rz + self.rw !* rhs.uw + self.rw !* rhs.ux + self.rw !* rhs.uy + self.rw !* rhs.uz + self.rx !* rhs.rw + self.rx !* rhs.rx + self.rx !* rhs.ry + self.rx !* rhs.rz + self.rx !* rhs.uw + self.rx !* rhs.ux + self.rx !* rhs.uy + self.rx !* rhs.uz + self.ry !* rhs.rw + self.ry !* rhs.rx + self.ry !* rhs.ry + self.ry !* rhs.rz + self.ry !* rhs.uw + self.ry !* rhs.ux + self.ry !* rhs.uy + self.ry !* rhs.uz + self.rz !* rhs.rw + self.rz !* rhs.rx + self.rz !* rhs.ry + self.rz !* rhs.rz + self.rz !* rhs.uw + self.rz !* rhs.ux + self.rz !* rhs.uy + self.rz !* rhs.uz + self.uw !* rhs.rw + self.uw !* rhs.rx + self.uw !* rhs.ry + self.uw !* rhs.rz + self.uw !* rhs.ux + self.uw !* rhs.uy + self.uw !* rhs.uz + self.ux !* rhs.rw + self.ux !* rhs.rx + self.ux !* rhs.ry + self.ux !* rhs.rz + self.ux !* rhs.uw + self.ux !* rhs.ux + self.ux !* rhs.uy + self.ux !* rhs.uz + self.uy !* rhs.rw + self.uy !* rhs.rx + self.uy !* rhs.ry + self.uy !* rhs.rz + self.uy !* rhs.uw + self.uy !* rhs.ux + self.uy !* rhs.uy + self.uy !* rhs.uz + self.uz !* rhs.rw + self.uz !* rhs.rx + self.uz !* rhs.ry + self.uz !* rhs.rz + self.uz !* rhs.uw + self.uz !* rhs.ux + self.uz !* rhs.uy + self.uz !* rhs.uz  (unnamed type)

// Motor3.dot(Motor3) -> Motor3
impl Dot<Motor3> for Motor3 {
	type Output = Motor3;
	fn dot(self, rhs: Motor3) -> Self::Output {
		// Motor3 {
		//     rx: WX(self.rx.0 * rhs.uw.0) + WX(self.uw.0 * rhs.rx.0),
		//     ry: WY(self.ry.0 * rhs.uw.0) + WY(self.uw.0 * rhs.ry.0),
		//     rz: WZ(self.rz.0 * rhs.uw.0) + WZ(self.uw.0 * rhs.rz.0),
		//     rw: XYZW(self.rw.0 * rhs.uw.0) + XYZW(self.uw.0 * rhs.rw.0),
		//     ux: YZW(self.uw.0 * rhs.ux.0) + YZW(self.ux.0 * rhs.uw.0),
		//     uy: ZXW(self.uw.0 * rhs.uy.0) + ZXW(self.uy.0 * rhs.uw.0),
		//     uz: XYW(self.uw.0 * rhs.uz.0) + XYW(self.uz.0 * rhs.uw.0),
		//     uw: S(self.uw.0 * rhs.uw.0),
		// }
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
		// Motor3 {
		//     rx: WX(self.rx.0 * rhs.uw.0) + WX(self.uw.0 * rhs.rx.0),
		//     ry: WY(self.ry.0 * rhs.uw.0) + WY(self.uw.0 * rhs.ry.0),
		//     rz: WZ(self.rz.0 * rhs.uw.0) + WZ(self.uw.0 * rhs.rz.0),
		//     rw: XYZW(self.rw.0 * rhs.uw.0) + XYZW(self.uw.0 * rhs.rw.0),
		//     ux: YZW(self.uw.0 * rhs.ux.0) + YZW(self.ux.0 * rhs.uw.0),
		//     uy: ZXW(self.uw.0 * rhs.uy.0) + ZXW(self.uy.0 * rhs.uw.0),
		//     uz: XYW(self.uw.0 * rhs.uz.0) + XYW(self.uz.0 * rhs.uw.0),
		//     uw: S(self.uw.0 * rhs.uw.0),
		// }
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

// Omitted: Motor3 anti_wedge Motor3 = self.rw & rhs.rw + self.rw & rhs.rx + self.rw & rhs.ry + self.rw & rhs.rz + self.rw & rhs.uw + self.rw & rhs.ux + self.rw & rhs.uy + self.rw & rhs.uz + self.rx & rhs.rw + self.rx & rhs.ux + self.ry & rhs.rw + self.ry & rhs.uy + self.rz & rhs.rw + self.rz & rhs.uz + self.uw & rhs.rw + self.ux & rhs.rw + self.ux & rhs.rx + self.ux & rhs.uy + self.ux & rhs.uz + self.uy & rhs.rw + self.uy & rhs.ry + self.uy & rhs.ux + self.uy & rhs.uz + self.uz & rhs.rw + self.uz & rhs.rz + self.uz & rhs.ux + self.uz & rhs.uy  (unnamed type)

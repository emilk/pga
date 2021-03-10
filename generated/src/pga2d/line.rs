//! # Line
//!
//! ## Operations
//! ```text
//! Line.geometric(Line) -> Motor
//! Line.dot(Line) -> S
//! Line.anti_wedge(Line) -> Vec3
//! Line.anti_geometric(Vec2) -> Rotor
//! Vec2.anti_geometric(Line) -> Rotor
//! Line.dot(Vec2) -> Vec3
//! Vec2.dot(Line) -> Vec3
//! Line.wedge(Vec2) -> XYW
//! Vec2.wedge(Line) -> XYW
//! Line.anti_wedge(Vec2) -> S
//! Vec2.anti_wedge(Line) -> S
//! Line.anti_geometric(Vec3) -> Motor
//! Vec3.anti_geometric(Line) -> Motor
//! Line.dot(Vec3) -> Vec3
//! Vec3.dot(Line) -> Vec3
//! Line.wedge(Vec3) -> XYW
//! Vec3.wedge(Line) -> XYW
//! Line.anti_wedge(Vec3) -> S
//! Vec3.anti_wedge(Line) -> S
//! Line.geometric(Rotor) -> Motor
//! Rotor.geometric(Line) -> Motor
//! Line.anti_geometric(Rotor) -> Vec2
//! Rotor.anti_geometric(Line) -> Vec2
//! Line.dot(Rotor) -> Motor
//! Rotor.dot(Line) -> Motor
//! Line.wedge(Rotor) -> Line
//! Rotor.wedge(Line) -> Line
//! Line.anti_wedge(Rotor) -> Vec2
//! Rotor.anti_wedge(Line) -> Vec2
//! Line.geometric(Motor) -> Motor
//! Motor.geometric(Line) -> Motor
//! Line.dot(Motor) -> Motor
//! Motor.dot(Line) -> Motor
//! Line.wedge(Motor) -> Line
//! Motor.wedge(Line) -> Line
//! Line.anti_wedge(Motor) -> Vec3
//! Motor.anti_wedge(Line) -> Vec3
//! ```

use super::*;

#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, derive_more::Neg, derive_more::Add, derive_more::Sub)]
pub struct Line {
	pub dx: YW,
	pub dy: WX,
	pub m: XY,
}

// ---------------------------------------------------------------------

impl RCompl for Line {
	type Output = Vec3;
	fn rcompl(self) -> Self::Output {
		Vec3 {
			x: self.dx.rcompl(),
			y: self.dy.rcompl(),
			w: self.m.rcompl(),
		}
	}
}

impl LCompl for Line {
	type Output = Vec3;
	fn lcompl(self) -> Self::Output {
		Vec3 {
			x: self.dx.lcompl(),
			y: self.dy.lcompl(),
			w: self.m.lcompl(),
		}
	}
}

impl Reverse for Line {
	fn rev(self) -> Self {
		Line {
			dx: -self.dx,
			dy: -self.dy,
			m: -self.m,
		}
	}
}

impl AntiReverse for Line {
	fn arev(self) -> Self {
		Line {
			dx: self.dx,
			dy: self.dy,
			m: self.m,
		}
	}
}

// ---------------------------------------------------------------------
// Line OP Vec2:

// Omitted: Line geometric Vec2 = self.dx * rhs.x + self.dx * rhs.y + self.dy * rhs.x + self.dy * rhs.y + self.m * rhs.x + self.m * rhs.y  (unnamed type)

// Line.anti_geometric(Vec2) -> Rotor
impl AntiGeometric<Vec2> for Line {
	type Output = Rotor;
	fn anti_geometric(self, rhs: Vec2) -> Self::Output {
		// Rotor {
		//     s : S(self.dx.0 * rhs.x.0) + S(self.dy.0 * rhs.y.0),
		//     xy: XY(self.dx.0 * rhs.y.0) + XY(self.dy.0 * rhs.x.0),
		// }
		Rotor {
			s: self.dx.anti_geometric(rhs.x) + self.dy.anti_geometric(rhs.y),
			xy: -self.dx.anti_geometric(rhs.y) + self.dy.anti_geometric(rhs.x),
		}
	}
}

// Line.dot(Vec2) -> Vec3
impl Dot<Vec2> for Line {
	type Output = Vec3;
	fn dot(self, rhs: Vec2) -> Self::Output {
		// Vec3 {
		//     x: X(self.m.0 * rhs.y.0),
		//     y: Y(self.m.0 * rhs.x.0),
		//     w: W(self.dx.0 * rhs.y.0) + W(self.dy.0 * rhs.x.0),
		// }
		Vec3 {
			x: self.m.dot(rhs.y),
			y: -self.m.dot(rhs.x),
			w: -self.dx.dot(rhs.y) + self.dy.dot(rhs.x),
		}
	}
}

// Line.wedge(Vec2) -> XYW
impl Wedge<Vec2> for Line {
	type Output = XYW;
	fn wedge(self, rhs: Vec2) -> Self::Output {
		// XYW(self.dx.0 * rhs.x.0) + XYW(self.dy.0 * rhs.y.0)
		self.dx.wedge(rhs.x) + self.dy.wedge(rhs.y)
	}
}

// Line.anti_wedge(Vec2) -> S
impl AntiWedge<Vec2> for Line {
	type Output = S;
	fn anti_wedge(self, rhs: Vec2) -> Self::Output {
		// S(self.dx.0 * rhs.x.0) + S(self.dy.0 * rhs.y.0)
		self.dx.anti_wedge(rhs.x) + self.dy.anti_wedge(rhs.y)
	}
}

// ---------------------------------------------------------------------
// Line OP Vec3:

// Omitted: Line geometric Vec3 = self.dx * rhs.x + self.dx * rhs.y + self.dy * rhs.x + self.dy * rhs.y + self.m * rhs.w + self.m * rhs.x + self.m * rhs.y  (unnamed type)

// Line.anti_geometric(Vec3) -> Motor
impl AntiGeometric<Vec3> for Line {
	type Output = Motor;
	fn anti_geometric(self, rhs: Vec3) -> Self::Output {
		// Motor {
		//     s : S(self.dx.0 * rhs.x.0) + S(self.dy.0 * rhs.y.0) + S(self.m.0 * rhs.w.0),
		//     yw: YW(self.dy.0 * rhs.w.0),
		//     wx: WX(self.dx.0 * rhs.w.0),
		//     xy: XY(self.dx.0 * rhs.y.0) + XY(self.dy.0 * rhs.x.0),
		// }
		Motor {
			s: self.dx.anti_geometric(rhs.x) + self.dy.anti_geometric(rhs.y) + self.m.anti_geometric(rhs.w),
			yw: -self.dy.anti_geometric(rhs.w),
			wx: self.dx.anti_geometric(rhs.w),
			xy: -self.dx.anti_geometric(rhs.y) + self.dy.anti_geometric(rhs.x),
		}
	}
}

// Line.dot(Vec3) -> Vec3
impl Dot<Vec3> for Line {
	type Output = Vec3;
	fn dot(self, rhs: Vec3) -> Self::Output {
		// Vec3 {
		//     x: X(self.m.0 * rhs.y.0),
		//     y: Y(self.m.0 * rhs.x.0),
		//     w: W(self.dx.0 * rhs.y.0) + W(self.dy.0 * rhs.x.0),
		// }
		Vec3 {
			x: self.m.dot(rhs.y),
			y: -self.m.dot(rhs.x),
			w: -self.dx.dot(rhs.y) + self.dy.dot(rhs.x),
		}
	}
}

// Line.wedge(Vec3) -> XYW
impl Wedge<Vec3> for Line {
	type Output = XYW;
	fn wedge(self, rhs: Vec3) -> Self::Output {
		// XYW(self.dx.0 * rhs.x.0) + XYW(self.dy.0 * rhs.y.0) + XYW(self.m.0 * rhs.w.0)
		self.dx.wedge(rhs.x) + self.dy.wedge(rhs.y) + self.m.wedge(rhs.w)
	}
}

// Line.anti_wedge(Vec3) -> S
impl AntiWedge<Vec3> for Line {
	type Output = S;
	fn anti_wedge(self, rhs: Vec3) -> Self::Output {
		// S(self.dx.0 * rhs.x.0) + S(self.dy.0 * rhs.y.0) + S(self.m.0 * rhs.w.0)
		self.dx.anti_wedge(rhs.x) + self.dy.anti_wedge(rhs.y) + self.m.anti_wedge(rhs.w)
	}
}

// ---------------------------------------------------------------------
// Line OP Line:

// Line.geometric(Line) -> Motor
impl Geometric<Line> for Line {
	type Output = Motor;
	fn geometric(self, rhs: Line) -> Self::Output {
		// Motor {
		//     s : S(self.m.0 * rhs.m.0),
		//     yw: YW(self.dy.0 * rhs.m.0) + YW(self.m.0 * rhs.dy.0),
		//     wx: WX(self.dx.0 * rhs.m.0) + WX(self.m.0 * rhs.dx.0),
		//     xy: Default::default(),
		// }
		Motor {
			s: -self.m.geometric(rhs.m),
			yw: -self.dy.geometric(rhs.m) + self.m.geometric(rhs.dy),
			wx: self.dx.geometric(rhs.m) - self.m.geometric(rhs.dx),
			xy: Default::default(),
		}
	}
}

// Omitted: Line anti_geometric Line = self.dx !* rhs.dx + self.dx !* rhs.dy + self.dx !* rhs.m + self.dy !* rhs.dx + self.dy !* rhs.dy + self.dy !* rhs.m + self.m !* rhs.dx + self.m !* rhs.dy  (unnamed type)

// Line.dot(Line) -> S
impl Dot<Line> for Line {
	type Output = S;
	fn dot(self, rhs: Line) -> Self::Output {
		// -S(self.m.0 * rhs.m.0)
		self.m.dot(rhs.m)
	}
}

// Omitted: Line wedge Line = 0  (unnamed type)

// Line.anti_wedge(Line) -> Vec3
impl AntiWedge<Line> for Line {
	type Output = Vec3;
	fn anti_wedge(self, rhs: Line) -> Self::Output {
		// Vec3 {
		//     x: X(self.dy.0 * rhs.m.0) + X(self.m.0 * rhs.dy.0),
		//     y: Y(self.dx.0 * rhs.m.0) + Y(self.m.0 * rhs.dx.0),
		//     w: W(self.dx.0 * rhs.dy.0) + W(self.dy.0 * rhs.dx.0),
		// }
		Vec3 {
			x: self.dy.anti_wedge(rhs.m) - self.m.anti_wedge(rhs.dy),
			y: -self.dx.anti_wedge(rhs.m) + self.m.anti_wedge(rhs.dx),
			w: self.dx.anti_wedge(rhs.dy) - self.dy.anti_wedge(rhs.dx),
		}
	}
}

// ---------------------------------------------------------------------
// Line OP Rotor:

// Line.geometric(Rotor) -> Motor
impl Geometric<Rotor> for Line {
	type Output = Motor;
	fn geometric(self, rhs: Rotor) -> Self::Output {
		// Motor {
		//     s : S(self.m.0 * rhs.xy.0),
		//     yw: YW(self.dx.0 * rhs.s.0) + YW(self.dy.0 * rhs.xy.0),
		//     wx: WX(self.dx.0 * rhs.xy.0) + WX(self.dy.0 * rhs.s.0),
		//     xy: XY(self.m.0 * rhs.s.0),
		// }
		Motor {
			s: -self.m.geometric(rhs.xy),
			yw: self.dx.geometric(rhs.s) - self.dy.geometric(rhs.xy),
			wx: self.dx.geometric(rhs.xy) + self.dy.geometric(rhs.s),
			xy: self.m.geometric(rhs.s),
		}
	}
}

// Line.anti_geometric(Rotor) -> Vec2
impl AntiGeometric<Rotor> for Line {
	type Output = Vec2;
	fn anti_geometric(self, rhs: Rotor) -> Self::Output {
		// Vec2 {
		//     x: X(self.dx.0 * rhs.s.0) + X(self.dy.0 * rhs.xy.0),
		//     y: Y(self.dx.0 * rhs.xy.0) + Y(self.dy.0 * rhs.s.0),
		// }
		Vec2 {
			x: self.dx.anti_geometric(rhs.s) + self.dy.anti_geometric(rhs.xy),
			y: -self.dx.anti_geometric(rhs.xy) + self.dy.anti_geometric(rhs.s),
		}
	}
}

// Line.dot(Rotor) -> Motor
impl Dot<Rotor> for Line {
	type Output = Motor;
	fn dot(self, rhs: Rotor) -> Self::Output {
		// Motor {
		//     s : S(self.m.0 * rhs.xy.0),
		//     yw: YW(self.dx.0 * rhs.s.0),
		//     wx: WX(self.dy.0 * rhs.s.0),
		//     xy: XY(self.m.0 * rhs.s.0),
		// }
		Motor {
			s: -self.m.dot(rhs.xy),
			yw: self.dx.dot(rhs.s),
			wx: self.dy.dot(rhs.s),
			xy: self.m.dot(rhs.s),
		}
	}
}

// Line.wedge(Rotor) -> Line
impl Wedge<Rotor> for Line {
	type Output = Line;
	fn wedge(self, rhs: Rotor) -> Self::Output {
		// Line {
		//     dx: YW(self.dx.0 * rhs.s.0),
		//     dy: WX(self.dy.0 * rhs.s.0),
		//     m : XY(self.m.0 * rhs.s.0),
		// }
		Line {
			dx: self.dx.wedge(rhs.s),
			dy: self.dy.wedge(rhs.s),
			m: self.m.wedge(rhs.s),
		}
	}
}

// Line.anti_wedge(Rotor) -> Vec2
impl AntiWedge<Rotor> for Line {
	type Output = Vec2;
	fn anti_wedge(self, rhs: Rotor) -> Self::Output {
		// Vec2 {
		//     x: X(self.dy.0 * rhs.xy.0),
		//     y: Y(self.dx.0 * rhs.xy.0),
		// }
		Vec2 {
			x: self.dy.anti_wedge(rhs.xy),
			y: -self.dx.anti_wedge(rhs.xy),
		}
	}
}

// ---------------------------------------------------------------------
// Line OP Motor:

// Line.geometric(Motor) -> Motor
impl Geometric<Motor> for Line {
	type Output = Motor;
	fn geometric(self, rhs: Motor) -> Self::Output {
		// Motor {
		//     s : S(self.m.0 * rhs.xy.0),
		//     yw: YW(self.dx.0 * rhs.s.0) + YW(self.dy.0 * rhs.xy.0) + YW(self.m.0 * rhs.wx.0),
		//     wx: WX(self.dx.0 * rhs.xy.0) + WX(self.dy.0 * rhs.s.0) + WX(self.m.0 * rhs.yw.0),
		//     xy: XY(self.m.0 * rhs.s.0),
		// }
		Motor {
			s: -self.m.geometric(rhs.xy),
			yw: self.dx.geometric(rhs.s) - self.dy.geometric(rhs.xy) + self.m.geometric(rhs.wx),
			wx: self.dx.geometric(rhs.xy) + self.dy.geometric(rhs.s) - self.m.geometric(rhs.yw),
			xy: self.m.geometric(rhs.s),
		}
	}
}

// Omitted: Line anti_geometric Motor = self.dx !* rhs.s + self.dx !* rhs.wx + self.dx !* rhs.xy + self.dx !* rhs.yw + self.dy !* rhs.s + self.dy !* rhs.wx + self.dy !* rhs.xy + self.dy !* rhs.yw + self.m !* rhs.wx + self.m !* rhs.yw  (unnamed type)

// Line.dot(Motor) -> Motor
impl Dot<Motor> for Line {
	type Output = Motor;
	fn dot(self, rhs: Motor) -> Self::Output {
		// Motor {
		//     s : S(self.m.0 * rhs.xy.0),
		//     yw: YW(self.dx.0 * rhs.s.0),
		//     wx: WX(self.dy.0 * rhs.s.0),
		//     xy: XY(self.m.0 * rhs.s.0),
		// }
		Motor {
			s: -self.m.dot(rhs.xy),
			yw: self.dx.dot(rhs.s),
			wx: self.dy.dot(rhs.s),
			xy: self.m.dot(rhs.s),
		}
	}
}

// Line.wedge(Motor) -> Line
impl Wedge<Motor> for Line {
	type Output = Line;
	fn wedge(self, rhs: Motor) -> Self::Output {
		// Line {
		//     dx: YW(self.dx.0 * rhs.s.0),
		//     dy: WX(self.dy.0 * rhs.s.0),
		//     m : XY(self.m.0 * rhs.s.0),
		// }
		Line {
			dx: self.dx.wedge(rhs.s),
			dy: self.dy.wedge(rhs.s),
			m: self.m.wedge(rhs.s),
		}
	}
}

// Line.anti_wedge(Motor) -> Vec3
impl AntiWedge<Motor> for Line {
	type Output = Vec3;
	fn anti_wedge(self, rhs: Motor) -> Self::Output {
		// Vec3 {
		//     x: X(self.dy.0 * rhs.xy.0) + X(self.m.0 * rhs.wx.0),
		//     y: Y(self.dx.0 * rhs.xy.0) + Y(self.m.0 * rhs.yw.0),
		//     w: W(self.dx.0 * rhs.wx.0) + W(self.dy.0 * rhs.yw.0),
		// }
		Vec3 {
			x: self.dy.anti_wedge(rhs.xy) - self.m.anti_wedge(rhs.wx),
			y: -self.dx.anti_wedge(rhs.xy) + self.m.anti_wedge(rhs.yw),
			w: self.dx.anti_wedge(rhs.wx) - self.dy.anti_wedge(rhs.yw),
		}
	}
}

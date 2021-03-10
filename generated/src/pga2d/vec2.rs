//! # Vec2
//!
//! ## Operations
//! ```text
//! Vec2.geometric(Vec2) -> Rotor
//! Vec2.dot(Vec2) -> S
//! Vec2.geometric(Vec3) -> Motor
//! Vec3.geometric(Vec2) -> Motor
//! Vec2.anti_geometric(Vec3) -> Vec2
//! Vec3.anti_geometric(Vec2) -> Vec2
//! Vec2.dot(Vec3) -> S
//! Vec3.dot(Vec2) -> S
//! Vec2.wedge(Vec3) -> Line
//! Vec3.wedge(Vec2) -> Line
//! Vec2.anti_geometric(Line) -> Rotor
//! Line.anti_geometric(Vec2) -> Rotor
//! Vec2.dot(Line) -> Vec3
//! Line.dot(Vec2) -> Vec3
//! Vec2.wedge(Line) -> XYW
//! Line.wedge(Vec2) -> XYW
//! Vec2.anti_wedge(Line) -> S
//! Line.anti_wedge(Vec2) -> S
//! Vec2.geometric(Rotor) -> Vec2
//! Rotor.geometric(Vec2) -> Vec2
//! Vec2.dot(Rotor) -> Vec2
//! Rotor.dot(Vec2) -> Vec2
//! Vec2.wedge(Rotor) -> Vec2
//! Rotor.wedge(Vec2) -> Vec2
//! Vec2.anti_geometric(Motor) -> Rotor
//! Motor.anti_geometric(Vec2) -> Rotor
//! Vec2.dot(Motor) -> Vec3
//! Motor.dot(Vec2) -> Vec3
//! Vec2.anti_wedge(Motor) -> S
//! Motor.anti_wedge(Vec2) -> S
//! ```

use super::*;

#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, derive_more::Neg, derive_more::Add, derive_more::Sub)]
pub struct Vec2 {
	pub x: X,
	pub y: Y,
}

// ---------------------------------------------------------------------

impl RCompl for Vec2 {
	type Output = Line;
	fn rcompl(self) -> Self::Output {
		Line {
			dx: self.x.rcompl(),
			dy: self.y.rcompl(),
			m: Default::default(),
		}
	}
}

impl LCompl for Vec2 {
	type Output = Line;
	fn lcompl(self) -> Self::Output {
		Line {
			dx: self.x.lcompl(),
			dy: self.y.lcompl(),
			m: Default::default(),
		}
	}
}

impl Reverse for Vec2 {
	fn rev(self) -> Self {
		Vec2 { x: self.x, y: self.y }
	}
}

impl AntiReverse for Vec2 {
	fn arev(self) -> Self {
		Vec2 { x: -self.x, y: -self.y }
	}
}

// ---------------------------------------------------------------------
// Vec2 OP Vec2:

// Vec2.geometric(Vec2) -> Rotor
impl Geometric<Vec2> for Vec2 {
	type Output = Rotor;
	fn geometric(self, rhs: Vec2) -> Self::Output {
		// Rotor {
		//     s : S(self.x.0 * rhs.x.0) + S(self.y.0 * rhs.y.0),
		//     xy: XY(self.x.0 * rhs.y.0) + XY(self.y.0 * rhs.x.0),
		// }
		Rotor {
			s: self.x.geometric(rhs.x) + self.y.geometric(rhs.y),
			xy: self.x.geometric(rhs.y) - self.y.geometric(rhs.x),
		}
	}
}

// Omitted: Vec2 anti_geometric Vec2 = 0  (unnamed type)

// Vec2.dot(Vec2) -> S
impl Dot<Vec2> for Vec2 {
	type Output = S;
	fn dot(self, rhs: Vec2) -> Self::Output {
		// S(self.x.0 * rhs.x.0) + S(self.y.0 * rhs.y.0)
		self.x.dot(rhs.x) + self.y.dot(rhs.y)
	}
}

// Omitted: Vec2 wedge Vec2 = self.x ^ rhs.y + self.y ^ rhs.x  (unnamed type)
// Omitted: Vec2 anti_wedge Vec2 = 0  (unnamed type)

// ---------------------------------------------------------------------
// Vec2 OP Vec3:

// Vec2.geometric(Vec3) -> Motor
impl Geometric<Vec3> for Vec2 {
	type Output = Motor;
	fn geometric(self, rhs: Vec3) -> Self::Output {
		// Motor {
		//     s : S(self.x.0 * rhs.x.0) + S(self.y.0 * rhs.y.0),
		//     yw: YW(self.y.0 * rhs.w.0),
		//     wx: WX(self.x.0 * rhs.w.0),
		//     xy: XY(self.x.0 * rhs.y.0) + XY(self.y.0 * rhs.x.0),
		// }
		Motor {
			s: self.x.geometric(rhs.x) + self.y.geometric(rhs.y),
			yw: self.y.geometric(rhs.w),
			wx: -self.x.geometric(rhs.w),
			xy: self.x.geometric(rhs.y) - self.y.geometric(rhs.x),
		}
	}
}

// Vec2.anti_geometric(Vec3) -> Vec2
impl AntiGeometric<Vec3> for Vec2 {
	type Output = Vec2;
	fn anti_geometric(self, rhs: Vec3) -> Self::Output {
		// Vec2 {
		//     x: X(self.y.0 * rhs.w.0),
		//     y: Y(self.x.0 * rhs.w.0),
		// }
		Vec2 {
			x: -self.y.anti_geometric(rhs.w),
			y: self.x.anti_geometric(rhs.w),
		}
	}
}

// Vec2.dot(Vec3) -> S
impl Dot<Vec3> for Vec2 {
	type Output = S;
	fn dot(self, rhs: Vec3) -> Self::Output {
		// S(self.x.0 * rhs.x.0) + S(self.y.0 * rhs.y.0)
		self.x.dot(rhs.x) + self.y.dot(rhs.y)
	}
}

// Vec2.wedge(Vec3) -> Line
impl Wedge<Vec3> for Vec2 {
	type Output = Line;
	fn wedge(self, rhs: Vec3) -> Self::Output {
		// Line {
		//     dx: YW(self.y.0 * rhs.w.0),
		//     dy: WX(self.x.0 * rhs.w.0),
		//     m : XY(self.x.0 * rhs.y.0) + XY(self.y.0 * rhs.x.0),
		// }
		Line {
			dx: self.y.wedge(rhs.w),
			dy: -self.x.wedge(rhs.w),
			m: self.x.wedge(rhs.y) - self.y.wedge(rhs.x),
		}
	}
}

// Omitted: Vec2 anti_wedge Vec3 = 0  (unnamed type)

// ---------------------------------------------------------------------
// Vec2 OP Line:

// Omitted: Vec2 geometric Line = self.x * rhs.dx + self.x * rhs.dy + self.x * rhs.m + self.y * rhs.dx + self.y * rhs.dy + self.y * rhs.m  (unnamed type)

// Vec2.anti_geometric(Line) -> Rotor
impl AntiGeometric<Line> for Vec2 {
	type Output = Rotor;
	fn anti_geometric(self, rhs: Line) -> Self::Output {
		// Rotor {
		//     s : S(self.x.0 * rhs.dx.0) + S(self.y.0 * rhs.dy.0),
		//     xy: XY(self.x.0 * rhs.dy.0) + XY(self.y.0 * rhs.dx.0),
		// }
		Rotor {
			s: self.x.anti_geometric(rhs.dx) + self.y.anti_geometric(rhs.dy),
			xy: -self.x.anti_geometric(rhs.dy) + self.y.anti_geometric(rhs.dx),
		}
	}
}

// Vec2.dot(Line) -> Vec3
impl Dot<Line> for Vec2 {
	type Output = Vec3;
	fn dot(self, rhs: Line) -> Self::Output {
		// Vec3 {
		//     x: X(self.y.0 * rhs.m.0),
		//     y: Y(self.x.0 * rhs.m.0),
		//     w: W(self.x.0 * rhs.dy.0) + W(self.y.0 * rhs.dx.0),
		// }
		Vec3 {
			x: -self.y.dot(rhs.m),
			y: self.x.dot(rhs.m),
			w: -self.x.dot(rhs.dy) + self.y.dot(rhs.dx),
		}
	}
}

// Vec2.wedge(Line) -> XYW
impl Wedge<Line> for Vec2 {
	type Output = XYW;
	fn wedge(self, rhs: Line) -> Self::Output {
		// XYW(self.x.0 * rhs.dx.0) + XYW(self.y.0 * rhs.dy.0)
		self.x.wedge(rhs.dx) + self.y.wedge(rhs.dy)
	}
}

// Vec2.anti_wedge(Line) -> S
impl AntiWedge<Line> for Vec2 {
	type Output = S;
	fn anti_wedge(self, rhs: Line) -> Self::Output {
		// S(self.x.0 * rhs.dx.0) + S(self.y.0 * rhs.dy.0)
		self.x.anti_wedge(rhs.dx) + self.y.anti_wedge(rhs.dy)
	}
}

// ---------------------------------------------------------------------
// Vec2 OP Rotor:

// Vec2.geometric(Rotor) -> Vec2
impl Geometric<Rotor> for Vec2 {
	type Output = Vec2;
	fn geometric(self, rhs: Rotor) -> Self::Output {
		// Vec2 {
		//     x: X(self.x.0 * rhs.s.0) + X(self.y.0 * rhs.xy.0),
		//     y: Y(self.x.0 * rhs.xy.0) + Y(self.y.0 * rhs.s.0),
		// }
		Vec2 {
			x: self.x.geometric(rhs.s) - self.y.geometric(rhs.xy),
			y: self.x.geometric(rhs.xy) + self.y.geometric(rhs.s),
		}
	}
}

// Omitted: Vec2 anti_geometric Rotor = 0  (unnamed type)

// Vec2.dot(Rotor) -> Vec2
impl Dot<Rotor> for Vec2 {
	type Output = Vec2;
	fn dot(self, rhs: Rotor) -> Self::Output {
		// Vec2 {
		//     x: X(self.x.0 * rhs.s.0) + X(self.y.0 * rhs.xy.0),
		//     y: Y(self.x.0 * rhs.xy.0) + Y(self.y.0 * rhs.s.0),
		// }
		Vec2 {
			x: self.x.dot(rhs.s) - self.y.dot(rhs.xy),
			y: self.x.dot(rhs.xy) + self.y.dot(rhs.s),
		}
	}
}

// Vec2.wedge(Rotor) -> Vec2
impl Wedge<Rotor> for Vec2 {
	type Output = Vec2;
	fn wedge(self, rhs: Rotor) -> Self::Output {
		// Vec2 {
		//     x: X(self.x.0 * rhs.s.0),
		//     y: Y(self.y.0 * rhs.s.0),
		// }
		Vec2 {
			x: self.x.wedge(rhs.s),
			y: self.y.wedge(rhs.s),
		}
	}
}

// Omitted: Vec2 anti_wedge Rotor = 0  (unnamed type)

// ---------------------------------------------------------------------
// Vec2 OP Motor:

// Omitted: Vec2 geometric Motor = self.x * rhs.s + self.x * rhs.wx + self.x * rhs.xy + self.x * rhs.yw + self.y * rhs.s + self.y * rhs.wx + self.y * rhs.xy + self.y * rhs.yw  (unnamed type)

// Vec2.anti_geometric(Motor) -> Rotor
impl AntiGeometric<Motor> for Vec2 {
	type Output = Rotor;
	fn anti_geometric(self, rhs: Motor) -> Self::Output {
		// Rotor {
		//     s : S(self.x.0 * rhs.yw.0) + S(self.y.0 * rhs.wx.0),
		//     xy: XY(self.x.0 * rhs.wx.0) + XY(self.y.0 * rhs.yw.0),
		// }
		Rotor {
			s: self.x.anti_geometric(rhs.yw) + self.y.anti_geometric(rhs.wx),
			xy: -self.x.anti_geometric(rhs.wx) + self.y.anti_geometric(rhs.yw),
		}
	}
}

// Vec2.dot(Motor) -> Vec3
impl Dot<Motor> for Vec2 {
	type Output = Vec3;
	fn dot(self, rhs: Motor) -> Self::Output {
		// Vec3 {
		//     x: X(self.x.0 * rhs.s.0) + X(self.y.0 * rhs.xy.0),
		//     y: Y(self.x.0 * rhs.xy.0) + Y(self.y.0 * rhs.s.0),
		//     w: W(self.x.0 * rhs.wx.0) + W(self.y.0 * rhs.yw.0),
		// }
		Vec3 {
			x: self.x.dot(rhs.s) - self.y.dot(rhs.xy),
			y: self.x.dot(rhs.xy) + self.y.dot(rhs.s),
			w: -self.x.dot(rhs.wx) + self.y.dot(rhs.yw),
		}
	}
}

// Omitted: Vec2 wedge Motor = self.x ^ rhs.s + self.x ^ rhs.yw + self.y ^ rhs.s + self.y ^ rhs.wx  (unnamed type)

// Vec2.anti_wedge(Motor) -> S
impl AntiWedge<Motor> for Vec2 {
	type Output = S;
	fn anti_wedge(self, rhs: Motor) -> Self::Output {
		// S(self.x.0 * rhs.yw.0) + S(self.y.0 * rhs.wx.0)
		self.x.anti_wedge(rhs.yw) + self.y.anti_wedge(rhs.wx)
	}
}

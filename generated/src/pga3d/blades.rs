//! # Blade types
//! The blades that make up this geometric algebra.
//!
//! ## Unary operations
//! | Op \ Blade       | 1    | X    | Y    | Z    | W    | WX  | WY  | WZ  | YZ  | ZX  | XY  | YZW  | ZXW  | XYW  | ZYX  | XYZW |
//! | ---------------- | ---- | ---- | ---- | ---- | ---- | --- | --- | --- | --- | --- | --- | ---- | ---- | ---- | ---- | ---- |
//! | Right complement | XYZW | YZW  | ZXW  | XYW  | ZYX  | -YZ | -ZX | -XY | -WX | -WY | -WZ | -X   | -Y   | -Z   | -W   | 1    |
//! | Left complement  | XYZW | -YZW | -ZXW | -XYW | -ZYX | -YZ | -ZX | -XY | -WX | -WY | -WZ | X    | Y    | Z    | W    | 1    |
//! | Reverse          | 1    | X    | Y    | Z    | W    | -WX | -WY | -WZ | -YZ | -ZX | -XY | -YZW | -ZXW | -XYW | -ZYX | XYZW |
//! | Anti-reverse     | 1    | -X   | -Y   | -Z   | -W   | -WX | -WY | -WZ | -YZ | -ZX | -XY | YZW  | ZXW  | XYW  | ZYX  | XYZW |
//!
//!
//! ## Multiplication tables
//! ### Geometric multiplication table
//!
//! |      | 1    | X     | Y     | Z     | W     | WX    | WY    | WZ    | YZ    | ZX    | XY    | YZW  | ZXW  | XYW  | ZYX  | XYZW |
//! | ---- | ---- | ----- | ----- | ----- | ----- | ----- | ----- | ----- | ----- | ----- | ----- | ---- | ---- | ---- | ---- | ---- |
//! | 1    | 1    | X     | Y     | Z     | W     | WX    | WY    | WZ    | YZ    | ZX    | XY    | YZW  | ZXW  | XYW  | ZYX  | XYZW |
//! | X    | X    | 1     | XY    | -ZX   | -WX   | -W    | -XYW  | ZXW   | -ZYX  | -Z    | Y     | XYZW | WZ   | -WY  | -YZ  | YZW  |
//! | Y    | Y    | -XY   | 1     | YZ    | -WY   | XYW   | -W    | -YZW  | Z     | -ZYX  | -X    | -WZ  | XYZW | WX   | -ZX  | ZXW  |
//! | Z    | Z    | ZX    | -YZ   | 1     | -WZ   | -ZXW  | YZW   | -W    | -Y    | X     | -ZYX  | WY   | -WX  | XYZW | -XY  | XYW  |
//! | W    | W    | WX    | WY    | WZ    | 0     | 0     | 0     | 0     | YZW   | ZXW   | XYW   | 0    | 0    | 0    | XYZW | 0    |
//! | WX   | WX   | W     | XYW   | -ZXW  | 0     | 0     | 0     | 0     | -XYZW | -WZ   | WY    | 0    | 0    | 0    | -YZW | 0    |
//! | WY   | WY   | -XYW  | W     | YZW   | 0     | 0     | 0     | 0     | WZ    | -XYZW | -WX   | 0    | 0    | 0    | -ZXW | 0    |
//! | WZ   | WZ   | ZXW   | -YZW  | W     | 0     | 0     | 0     | 0     | -WY   | WX    | -XYZW | 0    | 0    | 0    | -XYW | 0    |
//! | YZ   | YZ   | -ZYX  | -Z    | Y     | YZW   | -XYZW | -WZ   | WY    | -1    | -XY   | ZX    | -W   | -XYW | ZXW  | X    | WX   |
//! | ZX   | ZX   | Z     | -ZYX  | -X    | ZXW   | WZ    | -XYZW | -WX   | XY    | -1    | -YZ   | XYW  | -W   | -YZW | Y    | WY   |
//! | XY   | XY   | -Y    | X     | -ZYX  | XYW   | -WY   | WX    | -XYZW | -ZX   | YZ    | -1    | -ZXW | YZW  | -W   | Z    | WZ   |
//! | YZW  | YZW  | -XYZW | -WZ   | WY    | 0     | 0     | 0     | 0     | -W    | -XYW  | ZXW   | 0    | 0    | 0    | WX   | 0    |
//! | ZXW  | ZXW  | WZ    | -XYZW | -WX   | 0     | 0     | 0     | 0     | XYW   | -W    | -YZW  | 0    | 0    | 0    | WY   | 0    |
//! | XYW  | XYW  | -WY   | WX    | -XYZW | 0     | 0     | 0     | 0     | -ZXW  | YZW   | -W    | 0    | 0    | 0    | WZ   | 0    |
//! | ZYX  | ZYX  | -YZ   | -ZX   | -XY   | -XYZW | YZW   | ZXW   | XYW   | X     | Y     | Z     | -WX  | -WY  | -WZ  | -1   | W    |
//! | XYZW | XYZW | -YZW  | -ZXW  | -XYW  | 0     | 0     | 0     | 0     | WX    | WY    | WZ    | 0    | 0    | 0    | -W   | 0    |
//!
//!
//! ### AntiGeometric multiplication table
//!
//! |      | 1    | X     | Y     | Z     | W     | WX    | WY    | WZ    | YZ    | ZX    | XY    | YZW  | ZXW  | XYW  | ZYX  | XYZW |
//! | ---- | ---- | ----- | ----- | ----- | ----- | ----- | ----- | ----- | ----- | ----- | ----- | ---- | ---- | ---- | ---- | ---- |
//! | 1    | 1    | X     | Y     | Z     | W     | WX    | WY    | WZ    | YZ    | ZX    | XY    | YZW  | ZXW  | XYW  | ZYX  | XYZW |
//! | X    | X    | 1     | XY    | -ZX   | -WX   | -W    | -XYW  | ZXW   | -ZYX  | -Z    | Y     | XYZW | WZ   | -WY  | -YZ  | YZW  |
//! | Y    | Y    | -XY   | 1     | YZ    | -WY   | XYW   | -W    | -YZW  | Z     | -ZYX  | -X    | -WZ  | XYZW | WX   | -ZX  | ZXW  |
//! | Z    | Z    | ZX    | -YZ   | 1     | -WZ   | -ZXW  | YZW   | -W    | -Y    | X     | -ZYX  | WY   | -WX  | XYZW | -XY  | XYW  |
//! | W    | W    | WX    | WY    | WZ    | 0     | 0     | 0     | 0     | YZW   | ZXW   | XYW   | 0    | 0    | 0    | XYZW | 0    |
//! | WX   | WX   | W     | XYW   | -ZXW  | 0     | 0     | 0     | 0     | -XYZW | -WZ   | WY    | 0    | 0    | 0    | -YZW | 0    |
//! | WY   | WY   | -XYW  | W     | YZW   | 0     | 0     | 0     | 0     | WZ    | -XYZW | -WX   | 0    | 0    | 0    | -ZXW | 0    |
//! | WZ   | WZ   | ZXW   | -YZW  | W     | 0     | 0     | 0     | 0     | -WY   | WX    | -XYZW | 0    | 0    | 0    | -XYW | 0    |
//! | YZ   | YZ   | -ZYX  | -Z    | Y     | YZW   | -XYZW | -WZ   | WY    | -1    | -XY   | ZX    | -W   | -XYW | ZXW  | X    | WX   |
//! | ZX   | ZX   | Z     | -ZYX  | -X    | ZXW   | WZ    | -XYZW | -WX   | XY    | -1    | -YZ   | XYW  | -W   | -YZW | Y    | WY   |
//! | XY   | XY   | -Y    | X     | -ZYX  | XYW   | -WY   | WX    | -XYZW | -ZX   | YZ    | -1    | -ZXW | YZW  | -W   | Z    | WZ   |
//! | YZW  | YZW  | -XYZW | -WZ   | WY    | 0     | 0     | 0     | 0     | -W    | -XYW  | ZXW   | 0    | 0    | 0    | WX   | 0    |
//! | ZXW  | ZXW  | WZ    | -XYZW | -WX   | 0     | 0     | 0     | 0     | XYW   | -W    | -YZW  | 0    | 0    | 0    | WY   | 0    |
//! | XYW  | XYW  | -WY   | WX    | -XYZW | 0     | 0     | 0     | 0     | -ZXW  | YZW   | -W    | 0    | 0    | 0    | WZ   | 0    |
//! | ZYX  | ZYX  | -YZ   | -ZX   | -XY   | -XYZW | YZW   | ZXW   | XYW   | X     | Y     | Z     | -WX  | -WY  | -WZ  | -1   | W    |
//! | XYZW | XYZW | -YZW  | -ZXW  | -XYW  | 0     | 0     | 0     | 0     | WX    | WY    | WZ    | 0    | 0    | 0    | -W   | 0    |
//!
//!
//! ### Dot multiplication table
//!
//! |      | 1    | X     | Y     | Z     | W     | WX    | WY    | WZ    | YZ    | ZX    | XY    | YZW  | ZXW  | XYW  | ZYX  | XYZW |
//! | ---- | ---- | ----- | ----- | ----- | ----- | ----- | ----- | ----- | ----- | ----- | ----- | ---- | ---- | ---- | ---- | ---- |
//! | 1    | 1    | X     | Y     | Z     | W     | WX    | WY    | WZ    | YZ    | ZX    | XY    | YZW  | ZXW  | XYW  | ZYX  | XYZW |
//! | X    | X    | 1     | XY    | -ZX   | -WX   | -W    | -XYW  | ZXW   | -ZYX  | -Z    | Y     | XYZW | WZ   | -WY  | -YZ  | YZW  |
//! | Y    | Y    | -XY   | 1     | YZ    | -WY   | XYW   | -W    | -YZW  | Z     | -ZYX  | -X    | -WZ  | XYZW | WX   | -ZX  | ZXW  |
//! | Z    | Z    | ZX    | -YZ   | 1     | -WZ   | -ZXW  | YZW   | -W    | -Y    | X     | -ZYX  | WY   | -WX  | XYZW | -XY  | XYW  |
//! | W    | W    | WX    | WY    | WZ    | 0     | 0     | 0     | 0     | YZW   | ZXW   | XYW   | 0    | 0    | 0    | XYZW | 0    |
//! | WX   | WX   | W     | XYW   | -ZXW  | 0     | 0     | 0     | 0     | -XYZW | -WZ   | WY    | 0    | 0    | 0    | -YZW | 0    |
//! | WY   | WY   | -XYW  | W     | YZW   | 0     | 0     | 0     | 0     | WZ    | -XYZW | -WX   | 0    | 0    | 0    | -ZXW | 0    |
//! | WZ   | WZ   | ZXW   | -YZW  | W     | 0     | 0     | 0     | 0     | -WY   | WX    | -XYZW | 0    | 0    | 0    | -XYW | 0    |
//! | YZ   | YZ   | -ZYX  | -Z    | Y     | YZW   | -XYZW | -WZ   | WY    | -1    | -XY   | ZX    | -W   | -XYW | ZXW  | X    | WX   |
//! | ZX   | ZX   | Z     | -ZYX  | -X    | ZXW   | WZ    | -XYZW | -WX   | XY    | -1    | -YZ   | XYW  | -W   | -YZW | Y    | WY   |
//! | XY   | XY   | -Y    | X     | -ZYX  | XYW   | -WY   | WX    | -XYZW | -ZX   | YZ    | -1    | -ZXW | YZW  | -W   | Z    | WZ   |
//! | YZW  | YZW  | -XYZW | -WZ   | WY    | 0     | 0     | 0     | 0     | -W    | -XYW  | ZXW   | 0    | 0    | 0    | WX   | 0    |
//! | ZXW  | ZXW  | WZ    | -XYZW | -WX   | 0     | 0     | 0     | 0     | XYW   | -W    | -YZW  | 0    | 0    | 0    | WY   | 0    |
//! | XYW  | XYW  | -WY   | WX    | -XYZW | 0     | 0     | 0     | 0     | -ZXW  | YZW   | -W    | 0    | 0    | 0    | WZ   | 0    |
//! | ZYX  | ZYX  | -YZ   | -ZX   | -XY   | -XYZW | YZW   | ZXW   | XYW   | X     | Y     | Z     | -WX  | -WY  | -WZ  | -1   | W    |
//! | XYZW | XYZW | -YZW  | -ZXW  | -XYW  | 0     | 0     | 0     | 0     | WX    | WY    | WZ    | 0    | 0    | 0    | -W   | 0    |
//!
//!
//! ### Wedge multiplication table
//!
//! |      | 1    | X     | Y     | Z     | W     | WX    | WY    | WZ    | YZ    | ZX    | XY    | YZW  | ZXW  | XYW  | ZYX  | XYZW |
//! | ---- | ---- | ----- | ----- | ----- | ----- | ----- | ----- | ----- | ----- | ----- | ----- | ---- | ---- | ---- | ---- | ---- |
//! | 1    | 1    | X     | Y     | Z     | W     | WX    | WY    | WZ    | YZ    | ZX    | XY    | YZW  | ZXW  | XYW  | ZYX  | XYZW |
//! | X    | X    | 1     | XY    | -ZX   | -WX   | -W    | -XYW  | ZXW   | -ZYX  | -Z    | Y     | XYZW | WZ   | -WY  | -YZ  | YZW  |
//! | Y    | Y    | -XY   | 1     | YZ    | -WY   | XYW   | -W    | -YZW  | Z     | -ZYX  | -X    | -WZ  | XYZW | WX   | -ZX  | ZXW  |
//! | Z    | Z    | ZX    | -YZ   | 1     | -WZ   | -ZXW  | YZW   | -W    | -Y    | X     | -ZYX  | WY   | -WX  | XYZW | -XY  | XYW  |
//! | W    | W    | WX    | WY    | WZ    | 0     | 0     | 0     | 0     | YZW   | ZXW   | XYW   | 0    | 0    | 0    | XYZW | 0    |
//! | WX   | WX   | W     | XYW   | -ZXW  | 0     | 0     | 0     | 0     | -XYZW | -WZ   | WY    | 0    | 0    | 0    | -YZW | 0    |
//! | WY   | WY   | -XYW  | W     | YZW   | 0     | 0     | 0     | 0     | WZ    | -XYZW | -WX   | 0    | 0    | 0    | -ZXW | 0    |
//! | WZ   | WZ   | ZXW   | -YZW  | W     | 0     | 0     | 0     | 0     | -WY   | WX    | -XYZW | 0    | 0    | 0    | -XYW | 0    |
//! | YZ   | YZ   | -ZYX  | -Z    | Y     | YZW   | -XYZW | -WZ   | WY    | -1    | -XY   | ZX    | -W   | -XYW | ZXW  | X    | WX   |
//! | ZX   | ZX   | Z     | -ZYX  | -X    | ZXW   | WZ    | -XYZW | -WX   | XY    | -1    | -YZ   | XYW  | -W   | -YZW | Y    | WY   |
//! | XY   | XY   | -Y    | X     | -ZYX  | XYW   | -WY   | WX    | -XYZW | -ZX   | YZ    | -1    | -ZXW | YZW  | -W   | Z    | WZ   |
//! | YZW  | YZW  | -XYZW | -WZ   | WY    | 0     | 0     | 0     | 0     | -W    | -XYW  | ZXW   | 0    | 0    | 0    | WX   | 0    |
//! | ZXW  | ZXW  | WZ    | -XYZW | -WX   | 0     | 0     | 0     | 0     | XYW   | -W    | -YZW  | 0    | 0    | 0    | WY   | 0    |
//! | XYW  | XYW  | -WY   | WX    | -XYZW | 0     | 0     | 0     | 0     | -ZXW  | YZW   | -W    | 0    | 0    | 0    | WZ   | 0    |
//! | ZYX  | ZYX  | -YZ   | -ZX   | -XY   | -XYZW | YZW   | ZXW   | XYW   | X     | Y     | Z     | -WX  | -WY  | -WZ  | -1   | W    |
//! | XYZW | XYZW | -YZW  | -ZXW  | -XYW  | 0     | 0     | 0     | 0     | WX    | WY    | WZ    | 0    | 0    | 0    | -W   | 0    |
//!
//!
//! ### AntiWedge multiplication table
//!
//! |      | 1    | X     | Y     | Z     | W     | WX    | WY    | WZ    | YZ    | ZX    | XY    | YZW  | ZXW  | XYW  | ZYX  | XYZW |
//! | ---- | ---- | ----- | ----- | ----- | ----- | ----- | ----- | ----- | ----- | ----- | ----- | ---- | ---- | ---- | ---- | ---- |
//! | 1    | 1    | X     | Y     | Z     | W     | WX    | WY    | WZ    | YZ    | ZX    | XY    | YZW  | ZXW  | XYW  | ZYX  | XYZW |
//! | X    | X    | 1     | XY    | -ZX   | -WX   | -W    | -XYW  | ZXW   | -ZYX  | -Z    | Y     | XYZW | WZ   | -WY  | -YZ  | YZW  |
//! | Y    | Y    | -XY   | 1     | YZ    | -WY   | XYW   | -W    | -YZW  | Z     | -ZYX  | -X    | -WZ  | XYZW | WX   | -ZX  | ZXW  |
//! | Z    | Z    | ZX    | -YZ   | 1     | -WZ   | -ZXW  | YZW   | -W    | -Y    | X     | -ZYX  | WY   | -WX  | XYZW | -XY  | XYW  |
//! | W    | W    | WX    | WY    | WZ    | 0     | 0     | 0     | 0     | YZW   | ZXW   | XYW   | 0    | 0    | 0    | XYZW | 0    |
//! | WX   | WX   | W     | XYW   | -ZXW  | 0     | 0     | 0     | 0     | -XYZW | -WZ   | WY    | 0    | 0    | 0    | -YZW | 0    |
//! | WY   | WY   | -XYW  | W     | YZW   | 0     | 0     | 0     | 0     | WZ    | -XYZW | -WX   | 0    | 0    | 0    | -ZXW | 0    |
//! | WZ   | WZ   | ZXW   | -YZW  | W     | 0     | 0     | 0     | 0     | -WY   | WX    | -XYZW | 0    | 0    | 0    | -XYW | 0    |
//! | YZ   | YZ   | -ZYX  | -Z    | Y     | YZW   | -XYZW | -WZ   | WY    | -1    | -XY   | ZX    | -W   | -XYW | ZXW  | X    | WX   |
//! | ZX   | ZX   | Z     | -ZYX  | -X    | ZXW   | WZ    | -XYZW | -WX   | XY    | -1    | -YZ   | XYW  | -W   | -YZW | Y    | WY   |
//! | XY   | XY   | -Y    | X     | -ZYX  | XYW   | -WY   | WX    | -XYZW | -ZX   | YZ    | -1    | -ZXW | YZW  | -W   | Z    | WZ   |
//! | YZW  | YZW  | -XYZW | -WZ   | WY    | 0     | 0     | 0     | 0     | -W    | -XYW  | ZXW   | 0    | 0    | 0    | WX   | 0    |
//! | ZXW  | ZXW  | WZ    | -XYZW | -WX   | 0     | 0     | 0     | 0     | XYW   | -W    | -YZW  | 0    | 0    | 0    | WY   | 0    |
//! | XYW  | XYW  | -WY   | WX    | -XYZW | 0     | 0     | 0     | 0     | -ZXW  | YZW   | -W    | 0    | 0    | 0    | WZ   | 0    |
//! | ZYX  | ZYX  | -YZ   | -ZX   | -XY   | -XYZW | YZW   | ZXW   | XYW   | X     | Y     | Z     | -WX  | -WY  | -WZ  | -1   | W    |
//! | XYZW | XYZW | -YZW  | -ZXW  | -XYW  | 0     | 0     | 0     | 0     | WX    | WY    | WZ    | 0    | 0    | 0    | -W   | 0    |

use derive_more::{Add, Mul, Neg, Sub};

use super::*;

/// The scalar type (real numbers).
/// Squares to 1.
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, Neg, Add, Sub, Mul)]
pub struct S(pub f64);

/// Squares to 1.
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, Neg, Add, Sub)]
pub struct X(pub f64);

/// Squares to 1.
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, Neg, Add, Sub)]
pub struct Y(pub f64);

/// Squares to 1.
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, Neg, Add, Sub)]
pub struct Z(pub f64);

/// Squares to 0.
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, Neg, Add, Sub)]
pub struct W(pub f64);

/// Squares to 0.
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, Neg, Add, Sub)]
pub struct WX(pub f64);

/// Squares to 0.
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, Neg, Add, Sub)]
pub struct WY(pub f64);

/// Squares to 0.
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, Neg, Add, Sub)]
pub struct WZ(pub f64);

/// Squares to -1.
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, Neg, Add, Sub)]
pub struct YZ(pub f64);

/// Squares to -1.
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, Neg, Add, Sub)]
pub struct ZX(pub f64);

/// Squares to -1.
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, Neg, Add, Sub)]
pub struct XY(pub f64);

/// Squares to 0.
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, Neg, Add, Sub)]
pub struct YZW(pub f64);

/// Squares to 0.
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, Neg, Add, Sub)]
pub struct ZXW(pub f64);

/// Squares to 0.
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, Neg, Add, Sub)]
pub struct XYW(pub f64);

/// Squares to -1.
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, Neg, Add, Sub)]
pub struct ZYX(pub f64);

/// The pseudo-scalar.
/// Squares to 0.
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, Neg, Add, Sub)]
pub struct XYZW(pub f64);

// ---------------------------------------------------------------------
// impl RCompl for blades:

impl RCompl for S {
    type Output = XYZW;
    fn rcompl(self) -> Self::Output {
        XYZW(self.0)
    }
}

impl RCompl for X {
    type Output = YZW;
    fn rcompl(self) -> Self::Output {
        YZW(self.0)
    }
}

impl RCompl for Y {
    type Output = ZXW;
    fn rcompl(self) -> Self::Output {
        ZXW(self.0)
    }
}

impl RCompl for Z {
    type Output = XYW;
    fn rcompl(self) -> Self::Output {
        XYW(self.0)
    }
}

impl RCompl for W {
    type Output = ZYX;
    fn rcompl(self) -> Self::Output {
        ZYX(self.0)
    }
}

impl RCompl for WX {
    type Output = YZ;
    fn rcompl(self) -> Self::Output {
        YZ(-self.0)
    }
}

impl RCompl for WY {
    type Output = ZX;
    fn rcompl(self) -> Self::Output {
        ZX(-self.0)
    }
}

impl RCompl for WZ {
    type Output = XY;
    fn rcompl(self) -> Self::Output {
        XY(-self.0)
    }
}

impl RCompl for YZ {
    type Output = WX;
    fn rcompl(self) -> Self::Output {
        WX(-self.0)
    }
}

impl RCompl for ZX {
    type Output = WY;
    fn rcompl(self) -> Self::Output {
        WY(-self.0)
    }
}

impl RCompl for XY {
    type Output = WZ;
    fn rcompl(self) -> Self::Output {
        WZ(-self.0)
    }
}

impl RCompl for YZW {
    type Output = X;
    fn rcompl(self) -> Self::Output {
        X(-self.0)
    }
}

impl RCompl for ZXW {
    type Output = Y;
    fn rcompl(self) -> Self::Output {
        Y(-self.0)
    }
}

impl RCompl for XYW {
    type Output = Z;
    fn rcompl(self) -> Self::Output {
        Z(-self.0)
    }
}

impl RCompl for ZYX {
    type Output = W;
    fn rcompl(self) -> Self::Output {
        W(-self.0)
    }
}

impl RCompl for XYZW {
    type Output = S;
    fn rcompl(self) -> Self::Output {
        S(self.0)
    }
}

// ---------------------------------------------------------------------
// impl LCompl for blades:

impl LCompl for S {
    type Output = XYZW;
    fn lcompl(self) -> Self::Output {
        XYZW(self.0)
    }
}

impl LCompl for X {
    type Output = YZW;
    fn lcompl(self) -> Self::Output {
        YZW(-self.0)
    }
}

impl LCompl for Y {
    type Output = ZXW;
    fn lcompl(self) -> Self::Output {
        ZXW(-self.0)
    }
}

impl LCompl for Z {
    type Output = XYW;
    fn lcompl(self) -> Self::Output {
        XYW(-self.0)
    }
}

impl LCompl for W {
    type Output = ZYX;
    fn lcompl(self) -> Self::Output {
        ZYX(-self.0)
    }
}

impl LCompl for WX {
    type Output = YZ;
    fn lcompl(self) -> Self::Output {
        YZ(-self.0)
    }
}

impl LCompl for WY {
    type Output = ZX;
    fn lcompl(self) -> Self::Output {
        ZX(-self.0)
    }
}

impl LCompl for WZ {
    type Output = XY;
    fn lcompl(self) -> Self::Output {
        XY(-self.0)
    }
}

impl LCompl for YZ {
    type Output = WX;
    fn lcompl(self) -> Self::Output {
        WX(-self.0)
    }
}

impl LCompl for ZX {
    type Output = WY;
    fn lcompl(self) -> Self::Output {
        WY(-self.0)
    }
}

impl LCompl for XY {
    type Output = WZ;
    fn lcompl(self) -> Self::Output {
        WZ(-self.0)
    }
}

impl LCompl for YZW {
    type Output = X;
    fn lcompl(self) -> Self::Output {
        X(self.0)
    }
}

impl LCompl for ZXW {
    type Output = Y;
    fn lcompl(self) -> Self::Output {
        Y(self.0)
    }
}

impl LCompl for XYW {
    type Output = Z;
    fn lcompl(self) -> Self::Output {
        Z(self.0)
    }
}

impl LCompl for ZYX {
    type Output = W;
    fn lcompl(self) -> Self::Output {
        W(self.0)
    }
}

impl LCompl for XYZW {
    type Output = S;
    fn lcompl(self) -> Self::Output {
        S(self.0)
    }
}

// ---------------------------------------------------------------------
// impl Reverse for blades:

impl Reverse for S {
    fn rev(self) -> Self {
        self
    }
}

impl Reverse for X {
    fn rev(self) -> Self {
        self
    }
}

impl Reverse for Y {
    fn rev(self) -> Self {
        self
    }
}

impl Reverse for Z {
    fn rev(self) -> Self {
        self
    }
}

impl Reverse for W {
    fn rev(self) -> Self {
        self
    }
}

impl Reverse for WX {
    fn rev(self) -> Self {
        -self
    }
}

impl Reverse for WY {
    fn rev(self) -> Self {
        -self
    }
}

impl Reverse for WZ {
    fn rev(self) -> Self {
        -self
    }
}

impl Reverse for YZ {
    fn rev(self) -> Self {
        -self
    }
}

impl Reverse for ZX {
    fn rev(self) -> Self {
        -self
    }
}

impl Reverse for XY {
    fn rev(self) -> Self {
        -self
    }
}

impl Reverse for YZW {
    fn rev(self) -> Self {
        -self
    }
}

impl Reverse for ZXW {
    fn rev(self) -> Self {
        -self
    }
}

impl Reverse for XYW {
    fn rev(self) -> Self {
        -self
    }
}

impl Reverse for ZYX {
    fn rev(self) -> Self {
        -self
    }
}

impl Reverse for XYZW {
    fn rev(self) -> Self {
        self
    }
}

// ---------------------------------------------------------------------
// impl AntiReverse for blades:

impl AntiReverse for S {
    fn arev(self) -> Self {
        self
    }
}

impl AntiReverse for X {
    fn arev(self) -> Self {
        -self
    }
}

impl AntiReverse for Y {
    fn arev(self) -> Self {
        -self
    }
}

impl AntiReverse for Z {
    fn arev(self) -> Self {
        -self
    }
}

impl AntiReverse for W {
    fn arev(self) -> Self {
        -self
    }
}

impl AntiReverse for WX {
    fn arev(self) -> Self {
        -self
    }
}

impl AntiReverse for WY {
    fn arev(self) -> Self {
        -self
    }
}

impl AntiReverse for WZ {
    fn arev(self) -> Self {
        -self
    }
}

impl AntiReverse for YZ {
    fn arev(self) -> Self {
        -self
    }
}

impl AntiReverse for ZX {
    fn arev(self) -> Self {
        -self
    }
}

impl AntiReverse for XY {
    fn arev(self) -> Self {
        -self
    }
}

impl AntiReverse for YZW {
    fn arev(self) -> Self {
        self
    }
}

impl AntiReverse for ZXW {
    fn arev(self) -> Self {
        self
    }
}

impl AntiReverse for XYW {
    fn arev(self) -> Self {
        self
    }
}

impl AntiReverse for ZYX {
    fn arev(self) -> Self {
        self
    }
}

impl AntiReverse for XYZW {
    fn arev(self) -> Self {
        self
    }
}

// ---------------------------------------------------------------------
// impl Geometric for blades:

impl Geometric<S> for S {
    type Output = S;
    fn geometric(self, rhs: S) -> Self::Output {
        S(self.0 * rhs.0)
    }
}

impl Geometric<X> for S {
    type Output = X;
    fn geometric(self, rhs: X) -> Self::Output {
        X(self.0 * rhs.0)
    }
}

impl Geometric<Y> for S {
    type Output = Y;
    fn geometric(self, rhs: Y) -> Self::Output {
        Y(self.0 * rhs.0)
    }
}

impl Geometric<Z> for S {
    type Output = Z;
    fn geometric(self, rhs: Z) -> Self::Output {
        Z(self.0 * rhs.0)
    }
}

impl Geometric<W> for S {
    type Output = W;
    fn geometric(self, rhs: W) -> Self::Output {
        W(self.0 * rhs.0)
    }
}

impl Geometric<WX> for S {
    type Output = WX;
    fn geometric(self, rhs: WX) -> Self::Output {
        WX(self.0 * rhs.0)
    }
}

impl Geometric<WY> for S {
    type Output = WY;
    fn geometric(self, rhs: WY) -> Self::Output {
        WY(self.0 * rhs.0)
    }
}

impl Geometric<WZ> for S {
    type Output = WZ;
    fn geometric(self, rhs: WZ) -> Self::Output {
        WZ(self.0 * rhs.0)
    }
}

impl Geometric<YZ> for S {
    type Output = YZ;
    fn geometric(self, rhs: YZ) -> Self::Output {
        YZ(self.0 * rhs.0)
    }
}

impl Geometric<ZX> for S {
    type Output = ZX;
    fn geometric(self, rhs: ZX) -> Self::Output {
        ZX(self.0 * rhs.0)
    }
}

impl Geometric<XY> for S {
    type Output = XY;
    fn geometric(self, rhs: XY) -> Self::Output {
        XY(self.0 * rhs.0)
    }
}

impl Geometric<YZW> for S {
    type Output = YZW;
    fn geometric(self, rhs: YZW) -> Self::Output {
        YZW(self.0 * rhs.0)
    }
}

impl Geometric<ZXW> for S {
    type Output = ZXW;
    fn geometric(self, rhs: ZXW) -> Self::Output {
        ZXW(self.0 * rhs.0)
    }
}

impl Geometric<XYW> for S {
    type Output = XYW;
    fn geometric(self, rhs: XYW) -> Self::Output {
        XYW(self.0 * rhs.0)
    }
}

impl Geometric<ZYX> for S {
    type Output = ZYX;
    fn geometric(self, rhs: ZYX) -> Self::Output {
        ZYX(self.0 * rhs.0)
    }
}

impl Geometric<XYZW> for S {
    type Output = XYZW;
    fn geometric(self, rhs: XYZW) -> Self::Output {
        XYZW(self.0 * rhs.0)
    }
}

impl Geometric<S> for X {
    type Output = X;
    fn geometric(self, rhs: S) -> Self::Output {
        X(self.0 * rhs.0)
    }
}

impl Geometric<X> for X {
    type Output = S;
    fn geometric(self, rhs: X) -> Self::Output {
        S(self.0 * rhs.0)
    }
}

impl Geometric<Y> for X {
    type Output = XY;
    fn geometric(self, rhs: Y) -> Self::Output {
        XY(self.0 * rhs.0)
    }
}

impl Geometric<Z> for X {
    type Output = ZX;
    fn geometric(self, rhs: Z) -> Self::Output {
        ZX(-self.0 * rhs.0)
    }
}

impl Geometric<W> for X {
    type Output = WX;
    fn geometric(self, rhs: W) -> Self::Output {
        WX(-self.0 * rhs.0)
    }
}

impl Geometric<WX> for X {
    type Output = W;
    fn geometric(self, rhs: WX) -> Self::Output {
        W(-self.0 * rhs.0)
    }
}

impl Geometric<WY> for X {
    type Output = XYW;
    fn geometric(self, rhs: WY) -> Self::Output {
        XYW(-self.0 * rhs.0)
    }
}

impl Geometric<WZ> for X {
    type Output = ZXW;
    fn geometric(self, rhs: WZ) -> Self::Output {
        ZXW(self.0 * rhs.0)
    }
}

impl Geometric<YZ> for X {
    type Output = ZYX;
    fn geometric(self, rhs: YZ) -> Self::Output {
        ZYX(-self.0 * rhs.0)
    }
}

impl Geometric<ZX> for X {
    type Output = Z;
    fn geometric(self, rhs: ZX) -> Self::Output {
        Z(-self.0 * rhs.0)
    }
}

impl Geometric<XY> for X {
    type Output = Y;
    fn geometric(self, rhs: XY) -> Self::Output {
        Y(self.0 * rhs.0)
    }
}

impl Geometric<YZW> for X {
    type Output = XYZW;
    fn geometric(self, rhs: YZW) -> Self::Output {
        XYZW(self.0 * rhs.0)
    }
}

impl Geometric<ZXW> for X {
    type Output = WZ;
    fn geometric(self, rhs: ZXW) -> Self::Output {
        WZ(self.0 * rhs.0)
    }
}

impl Geometric<XYW> for X {
    type Output = WY;
    fn geometric(self, rhs: XYW) -> Self::Output {
        WY(-self.0 * rhs.0)
    }
}

impl Geometric<ZYX> for X {
    type Output = YZ;
    fn geometric(self, rhs: ZYX) -> Self::Output {
        YZ(-self.0 * rhs.0)
    }
}

impl Geometric<XYZW> for X {
    type Output = YZW;
    fn geometric(self, rhs: XYZW) -> Self::Output {
        YZW(self.0 * rhs.0)
    }
}

impl Geometric<S> for Y {
    type Output = Y;
    fn geometric(self, rhs: S) -> Self::Output {
        Y(self.0 * rhs.0)
    }
}

impl Geometric<X> for Y {
    type Output = XY;
    fn geometric(self, rhs: X) -> Self::Output {
        XY(-self.0 * rhs.0)
    }
}

impl Geometric<Y> for Y {
    type Output = S;
    fn geometric(self, rhs: Y) -> Self::Output {
        S(self.0 * rhs.0)
    }
}

impl Geometric<Z> for Y {
    type Output = YZ;
    fn geometric(self, rhs: Z) -> Self::Output {
        YZ(self.0 * rhs.0)
    }
}

impl Geometric<W> for Y {
    type Output = WY;
    fn geometric(self, rhs: W) -> Self::Output {
        WY(-self.0 * rhs.0)
    }
}

impl Geometric<WX> for Y {
    type Output = XYW;
    fn geometric(self, rhs: WX) -> Self::Output {
        XYW(self.0 * rhs.0)
    }
}

impl Geometric<WY> for Y {
    type Output = W;
    fn geometric(self, rhs: WY) -> Self::Output {
        W(-self.0 * rhs.0)
    }
}

impl Geometric<WZ> for Y {
    type Output = YZW;
    fn geometric(self, rhs: WZ) -> Self::Output {
        YZW(-self.0 * rhs.0)
    }
}

impl Geometric<YZ> for Y {
    type Output = Z;
    fn geometric(self, rhs: YZ) -> Self::Output {
        Z(self.0 * rhs.0)
    }
}

impl Geometric<ZX> for Y {
    type Output = ZYX;
    fn geometric(self, rhs: ZX) -> Self::Output {
        ZYX(-self.0 * rhs.0)
    }
}

impl Geometric<XY> for Y {
    type Output = X;
    fn geometric(self, rhs: XY) -> Self::Output {
        X(-self.0 * rhs.0)
    }
}

impl Geometric<YZW> for Y {
    type Output = WZ;
    fn geometric(self, rhs: YZW) -> Self::Output {
        WZ(-self.0 * rhs.0)
    }
}

impl Geometric<ZXW> for Y {
    type Output = XYZW;
    fn geometric(self, rhs: ZXW) -> Self::Output {
        XYZW(self.0 * rhs.0)
    }
}

impl Geometric<XYW> for Y {
    type Output = WX;
    fn geometric(self, rhs: XYW) -> Self::Output {
        WX(self.0 * rhs.0)
    }
}

impl Geometric<ZYX> for Y {
    type Output = ZX;
    fn geometric(self, rhs: ZYX) -> Self::Output {
        ZX(-self.0 * rhs.0)
    }
}

impl Geometric<XYZW> for Y {
    type Output = ZXW;
    fn geometric(self, rhs: XYZW) -> Self::Output {
        ZXW(self.0 * rhs.0)
    }
}

impl Geometric<S> for Z {
    type Output = Z;
    fn geometric(self, rhs: S) -> Self::Output {
        Z(self.0 * rhs.0)
    }
}

impl Geometric<X> for Z {
    type Output = ZX;
    fn geometric(self, rhs: X) -> Self::Output {
        ZX(self.0 * rhs.0)
    }
}

impl Geometric<Y> for Z {
    type Output = YZ;
    fn geometric(self, rhs: Y) -> Self::Output {
        YZ(-self.0 * rhs.0)
    }
}

impl Geometric<Z> for Z {
    type Output = S;
    fn geometric(self, rhs: Z) -> Self::Output {
        S(self.0 * rhs.0)
    }
}

impl Geometric<W> for Z {
    type Output = WZ;
    fn geometric(self, rhs: W) -> Self::Output {
        WZ(-self.0 * rhs.0)
    }
}

impl Geometric<WX> for Z {
    type Output = ZXW;
    fn geometric(self, rhs: WX) -> Self::Output {
        ZXW(-self.0 * rhs.0)
    }
}

impl Geometric<WY> for Z {
    type Output = YZW;
    fn geometric(self, rhs: WY) -> Self::Output {
        YZW(self.0 * rhs.0)
    }
}

impl Geometric<WZ> for Z {
    type Output = W;
    fn geometric(self, rhs: WZ) -> Self::Output {
        W(-self.0 * rhs.0)
    }
}

impl Geometric<YZ> for Z {
    type Output = Y;
    fn geometric(self, rhs: YZ) -> Self::Output {
        Y(-self.0 * rhs.0)
    }
}

impl Geometric<ZX> for Z {
    type Output = X;
    fn geometric(self, rhs: ZX) -> Self::Output {
        X(self.0 * rhs.0)
    }
}

impl Geometric<XY> for Z {
    type Output = ZYX;
    fn geometric(self, rhs: XY) -> Self::Output {
        ZYX(-self.0 * rhs.0)
    }
}

impl Geometric<YZW> for Z {
    type Output = WY;
    fn geometric(self, rhs: YZW) -> Self::Output {
        WY(self.0 * rhs.0)
    }
}

impl Geometric<ZXW> for Z {
    type Output = WX;
    fn geometric(self, rhs: ZXW) -> Self::Output {
        WX(-self.0 * rhs.0)
    }
}

impl Geometric<XYW> for Z {
    type Output = XYZW;
    fn geometric(self, rhs: XYW) -> Self::Output {
        XYZW(self.0 * rhs.0)
    }
}

impl Geometric<ZYX> for Z {
    type Output = XY;
    fn geometric(self, rhs: ZYX) -> Self::Output {
        XY(-self.0 * rhs.0)
    }
}

impl Geometric<XYZW> for Z {
    type Output = XYW;
    fn geometric(self, rhs: XYZW) -> Self::Output {
        XYW(self.0 * rhs.0)
    }
}

impl Geometric<S> for W {
    type Output = W;
    fn geometric(self, rhs: S) -> Self::Output {
        W(self.0 * rhs.0)
    }
}

impl Geometric<X> for W {
    type Output = WX;
    fn geometric(self, rhs: X) -> Self::Output {
        WX(self.0 * rhs.0)
    }
}

impl Geometric<Y> for W {
    type Output = WY;
    fn geometric(self, rhs: Y) -> Self::Output {
        WY(self.0 * rhs.0)
    }
}

impl Geometric<Z> for W {
    type Output = WZ;
    fn geometric(self, rhs: Z) -> Self::Output {
        WZ(self.0 * rhs.0)
    }
}

impl Geometric<W> for W {
    type Output = Zero;
    fn geometric(self, _rhs: W) -> Self::Output {
        Zero {}
    }
}

impl Geometric<WX> for W {
    type Output = Zero;
    fn geometric(self, _rhs: WX) -> Self::Output {
        Zero {}
    }
}

impl Geometric<WY> for W {
    type Output = Zero;
    fn geometric(self, _rhs: WY) -> Self::Output {
        Zero {}
    }
}

impl Geometric<WZ> for W {
    type Output = Zero;
    fn geometric(self, _rhs: WZ) -> Self::Output {
        Zero {}
    }
}

impl Geometric<YZ> for W {
    type Output = YZW;
    fn geometric(self, rhs: YZ) -> Self::Output {
        YZW(self.0 * rhs.0)
    }
}

impl Geometric<ZX> for W {
    type Output = ZXW;
    fn geometric(self, rhs: ZX) -> Self::Output {
        ZXW(self.0 * rhs.0)
    }
}

impl Geometric<XY> for W {
    type Output = XYW;
    fn geometric(self, rhs: XY) -> Self::Output {
        XYW(self.0 * rhs.0)
    }
}

impl Geometric<YZW> for W {
    type Output = Zero;
    fn geometric(self, _rhs: YZW) -> Self::Output {
        Zero {}
    }
}

impl Geometric<ZXW> for W {
    type Output = Zero;
    fn geometric(self, _rhs: ZXW) -> Self::Output {
        Zero {}
    }
}

impl Geometric<XYW> for W {
    type Output = Zero;
    fn geometric(self, _rhs: XYW) -> Self::Output {
        Zero {}
    }
}

impl Geometric<ZYX> for W {
    type Output = XYZW;
    fn geometric(self, rhs: ZYX) -> Self::Output {
        XYZW(self.0 * rhs.0)
    }
}

impl Geometric<XYZW> for W {
    type Output = Zero;
    fn geometric(self, _rhs: XYZW) -> Self::Output {
        Zero {}
    }
}

impl Geometric<S> for WX {
    type Output = WX;
    fn geometric(self, rhs: S) -> Self::Output {
        WX(self.0 * rhs.0)
    }
}

impl Geometric<X> for WX {
    type Output = W;
    fn geometric(self, rhs: X) -> Self::Output {
        W(self.0 * rhs.0)
    }
}

impl Geometric<Y> for WX {
    type Output = XYW;
    fn geometric(self, rhs: Y) -> Self::Output {
        XYW(self.0 * rhs.0)
    }
}

impl Geometric<Z> for WX {
    type Output = ZXW;
    fn geometric(self, rhs: Z) -> Self::Output {
        ZXW(-self.0 * rhs.0)
    }
}

impl Geometric<W> for WX {
    type Output = Zero;
    fn geometric(self, _rhs: W) -> Self::Output {
        Zero {}
    }
}

impl Geometric<WX> for WX {
    type Output = Zero;
    fn geometric(self, _rhs: WX) -> Self::Output {
        Zero {}
    }
}

impl Geometric<WY> for WX {
    type Output = Zero;
    fn geometric(self, _rhs: WY) -> Self::Output {
        Zero {}
    }
}

impl Geometric<WZ> for WX {
    type Output = Zero;
    fn geometric(self, _rhs: WZ) -> Self::Output {
        Zero {}
    }
}

impl Geometric<YZ> for WX {
    type Output = XYZW;
    fn geometric(self, rhs: YZ) -> Self::Output {
        XYZW(-self.0 * rhs.0)
    }
}

impl Geometric<ZX> for WX {
    type Output = WZ;
    fn geometric(self, rhs: ZX) -> Self::Output {
        WZ(-self.0 * rhs.0)
    }
}

impl Geometric<XY> for WX {
    type Output = WY;
    fn geometric(self, rhs: XY) -> Self::Output {
        WY(self.0 * rhs.0)
    }
}

impl Geometric<YZW> for WX {
    type Output = Zero;
    fn geometric(self, _rhs: YZW) -> Self::Output {
        Zero {}
    }
}

impl Geometric<ZXW> for WX {
    type Output = Zero;
    fn geometric(self, _rhs: ZXW) -> Self::Output {
        Zero {}
    }
}

impl Geometric<XYW> for WX {
    type Output = Zero;
    fn geometric(self, _rhs: XYW) -> Self::Output {
        Zero {}
    }
}

impl Geometric<ZYX> for WX {
    type Output = YZW;
    fn geometric(self, rhs: ZYX) -> Self::Output {
        YZW(-self.0 * rhs.0)
    }
}

impl Geometric<XYZW> for WX {
    type Output = Zero;
    fn geometric(self, _rhs: XYZW) -> Self::Output {
        Zero {}
    }
}

impl Geometric<S> for WY {
    type Output = WY;
    fn geometric(self, rhs: S) -> Self::Output {
        WY(self.0 * rhs.0)
    }
}

impl Geometric<X> for WY {
    type Output = XYW;
    fn geometric(self, rhs: X) -> Self::Output {
        XYW(-self.0 * rhs.0)
    }
}

impl Geometric<Y> for WY {
    type Output = W;
    fn geometric(self, rhs: Y) -> Self::Output {
        W(self.0 * rhs.0)
    }
}

impl Geometric<Z> for WY {
    type Output = YZW;
    fn geometric(self, rhs: Z) -> Self::Output {
        YZW(self.0 * rhs.0)
    }
}

impl Geometric<W> for WY {
    type Output = Zero;
    fn geometric(self, _rhs: W) -> Self::Output {
        Zero {}
    }
}

impl Geometric<WX> for WY {
    type Output = Zero;
    fn geometric(self, _rhs: WX) -> Self::Output {
        Zero {}
    }
}

impl Geometric<WY> for WY {
    type Output = Zero;
    fn geometric(self, _rhs: WY) -> Self::Output {
        Zero {}
    }
}

impl Geometric<WZ> for WY {
    type Output = Zero;
    fn geometric(self, _rhs: WZ) -> Self::Output {
        Zero {}
    }
}

impl Geometric<YZ> for WY {
    type Output = WZ;
    fn geometric(self, rhs: YZ) -> Self::Output {
        WZ(self.0 * rhs.0)
    }
}

impl Geometric<ZX> for WY {
    type Output = XYZW;
    fn geometric(self, rhs: ZX) -> Self::Output {
        XYZW(-self.0 * rhs.0)
    }
}

impl Geometric<XY> for WY {
    type Output = WX;
    fn geometric(self, rhs: XY) -> Self::Output {
        WX(-self.0 * rhs.0)
    }
}

impl Geometric<YZW> for WY {
    type Output = Zero;
    fn geometric(self, _rhs: YZW) -> Self::Output {
        Zero {}
    }
}

impl Geometric<ZXW> for WY {
    type Output = Zero;
    fn geometric(self, _rhs: ZXW) -> Self::Output {
        Zero {}
    }
}

impl Geometric<XYW> for WY {
    type Output = Zero;
    fn geometric(self, _rhs: XYW) -> Self::Output {
        Zero {}
    }
}

impl Geometric<ZYX> for WY {
    type Output = ZXW;
    fn geometric(self, rhs: ZYX) -> Self::Output {
        ZXW(-self.0 * rhs.0)
    }
}

impl Geometric<XYZW> for WY {
    type Output = Zero;
    fn geometric(self, _rhs: XYZW) -> Self::Output {
        Zero {}
    }
}

impl Geometric<S> for WZ {
    type Output = WZ;
    fn geometric(self, rhs: S) -> Self::Output {
        WZ(self.0 * rhs.0)
    }
}

impl Geometric<X> for WZ {
    type Output = ZXW;
    fn geometric(self, rhs: X) -> Self::Output {
        ZXW(self.0 * rhs.0)
    }
}

impl Geometric<Y> for WZ {
    type Output = YZW;
    fn geometric(self, rhs: Y) -> Self::Output {
        YZW(-self.0 * rhs.0)
    }
}

impl Geometric<Z> for WZ {
    type Output = W;
    fn geometric(self, rhs: Z) -> Self::Output {
        W(self.0 * rhs.0)
    }
}

impl Geometric<W> for WZ {
    type Output = Zero;
    fn geometric(self, _rhs: W) -> Self::Output {
        Zero {}
    }
}

impl Geometric<WX> for WZ {
    type Output = Zero;
    fn geometric(self, _rhs: WX) -> Self::Output {
        Zero {}
    }
}

impl Geometric<WY> for WZ {
    type Output = Zero;
    fn geometric(self, _rhs: WY) -> Self::Output {
        Zero {}
    }
}

impl Geometric<WZ> for WZ {
    type Output = Zero;
    fn geometric(self, _rhs: WZ) -> Self::Output {
        Zero {}
    }
}

impl Geometric<YZ> for WZ {
    type Output = WY;
    fn geometric(self, rhs: YZ) -> Self::Output {
        WY(-self.0 * rhs.0)
    }
}

impl Geometric<ZX> for WZ {
    type Output = WX;
    fn geometric(self, rhs: ZX) -> Self::Output {
        WX(self.0 * rhs.0)
    }
}

impl Geometric<XY> for WZ {
    type Output = XYZW;
    fn geometric(self, rhs: XY) -> Self::Output {
        XYZW(-self.0 * rhs.0)
    }
}

impl Geometric<YZW> for WZ {
    type Output = Zero;
    fn geometric(self, _rhs: YZW) -> Self::Output {
        Zero {}
    }
}

impl Geometric<ZXW> for WZ {
    type Output = Zero;
    fn geometric(self, _rhs: ZXW) -> Self::Output {
        Zero {}
    }
}

impl Geometric<XYW> for WZ {
    type Output = Zero;
    fn geometric(self, _rhs: XYW) -> Self::Output {
        Zero {}
    }
}

impl Geometric<ZYX> for WZ {
    type Output = XYW;
    fn geometric(self, rhs: ZYX) -> Self::Output {
        XYW(-self.0 * rhs.0)
    }
}

impl Geometric<XYZW> for WZ {
    type Output = Zero;
    fn geometric(self, _rhs: XYZW) -> Self::Output {
        Zero {}
    }
}

impl Geometric<S> for YZ {
    type Output = YZ;
    fn geometric(self, rhs: S) -> Self::Output {
        YZ(self.0 * rhs.0)
    }
}

impl Geometric<X> for YZ {
    type Output = ZYX;
    fn geometric(self, rhs: X) -> Self::Output {
        ZYX(-self.0 * rhs.0)
    }
}

impl Geometric<Y> for YZ {
    type Output = Z;
    fn geometric(self, rhs: Y) -> Self::Output {
        Z(-self.0 * rhs.0)
    }
}

impl Geometric<Z> for YZ {
    type Output = Y;
    fn geometric(self, rhs: Z) -> Self::Output {
        Y(self.0 * rhs.0)
    }
}

impl Geometric<W> for YZ {
    type Output = YZW;
    fn geometric(self, rhs: W) -> Self::Output {
        YZW(self.0 * rhs.0)
    }
}

impl Geometric<WX> for YZ {
    type Output = XYZW;
    fn geometric(self, rhs: WX) -> Self::Output {
        XYZW(-self.0 * rhs.0)
    }
}

impl Geometric<WY> for YZ {
    type Output = WZ;
    fn geometric(self, rhs: WY) -> Self::Output {
        WZ(-self.0 * rhs.0)
    }
}

impl Geometric<WZ> for YZ {
    type Output = WY;
    fn geometric(self, rhs: WZ) -> Self::Output {
        WY(self.0 * rhs.0)
    }
}

impl Geometric<YZ> for YZ {
    type Output = S;
    fn geometric(self, rhs: YZ) -> Self::Output {
        S(-self.0 * rhs.0)
    }
}

impl Geometric<ZX> for YZ {
    type Output = XY;
    fn geometric(self, rhs: ZX) -> Self::Output {
        XY(-self.0 * rhs.0)
    }
}

impl Geometric<XY> for YZ {
    type Output = ZX;
    fn geometric(self, rhs: XY) -> Self::Output {
        ZX(self.0 * rhs.0)
    }
}

impl Geometric<YZW> for YZ {
    type Output = W;
    fn geometric(self, rhs: YZW) -> Self::Output {
        W(-self.0 * rhs.0)
    }
}

impl Geometric<ZXW> for YZ {
    type Output = XYW;
    fn geometric(self, rhs: ZXW) -> Self::Output {
        XYW(-self.0 * rhs.0)
    }
}

impl Geometric<XYW> for YZ {
    type Output = ZXW;
    fn geometric(self, rhs: XYW) -> Self::Output {
        ZXW(self.0 * rhs.0)
    }
}

impl Geometric<ZYX> for YZ {
    type Output = X;
    fn geometric(self, rhs: ZYX) -> Self::Output {
        X(self.0 * rhs.0)
    }
}

impl Geometric<XYZW> for YZ {
    type Output = WX;
    fn geometric(self, rhs: XYZW) -> Self::Output {
        WX(self.0 * rhs.0)
    }
}

impl Geometric<S> for ZX {
    type Output = ZX;
    fn geometric(self, rhs: S) -> Self::Output {
        ZX(self.0 * rhs.0)
    }
}

impl Geometric<X> for ZX {
    type Output = Z;
    fn geometric(self, rhs: X) -> Self::Output {
        Z(self.0 * rhs.0)
    }
}

impl Geometric<Y> for ZX {
    type Output = ZYX;
    fn geometric(self, rhs: Y) -> Self::Output {
        ZYX(-self.0 * rhs.0)
    }
}

impl Geometric<Z> for ZX {
    type Output = X;
    fn geometric(self, rhs: Z) -> Self::Output {
        X(-self.0 * rhs.0)
    }
}

impl Geometric<W> for ZX {
    type Output = ZXW;
    fn geometric(self, rhs: W) -> Self::Output {
        ZXW(self.0 * rhs.0)
    }
}

impl Geometric<WX> for ZX {
    type Output = WZ;
    fn geometric(self, rhs: WX) -> Self::Output {
        WZ(self.0 * rhs.0)
    }
}

impl Geometric<WY> for ZX {
    type Output = XYZW;
    fn geometric(self, rhs: WY) -> Self::Output {
        XYZW(-self.0 * rhs.0)
    }
}

impl Geometric<WZ> for ZX {
    type Output = WX;
    fn geometric(self, rhs: WZ) -> Self::Output {
        WX(-self.0 * rhs.0)
    }
}

impl Geometric<YZ> for ZX {
    type Output = XY;
    fn geometric(self, rhs: YZ) -> Self::Output {
        XY(self.0 * rhs.0)
    }
}

impl Geometric<ZX> for ZX {
    type Output = S;
    fn geometric(self, rhs: ZX) -> Self::Output {
        S(-self.0 * rhs.0)
    }
}

impl Geometric<XY> for ZX {
    type Output = YZ;
    fn geometric(self, rhs: XY) -> Self::Output {
        YZ(-self.0 * rhs.0)
    }
}

impl Geometric<YZW> for ZX {
    type Output = XYW;
    fn geometric(self, rhs: YZW) -> Self::Output {
        XYW(self.0 * rhs.0)
    }
}

impl Geometric<ZXW> for ZX {
    type Output = W;
    fn geometric(self, rhs: ZXW) -> Self::Output {
        W(-self.0 * rhs.0)
    }
}

impl Geometric<XYW> for ZX {
    type Output = YZW;
    fn geometric(self, rhs: XYW) -> Self::Output {
        YZW(-self.0 * rhs.0)
    }
}

impl Geometric<ZYX> for ZX {
    type Output = Y;
    fn geometric(self, rhs: ZYX) -> Self::Output {
        Y(self.0 * rhs.0)
    }
}

impl Geometric<XYZW> for ZX {
    type Output = WY;
    fn geometric(self, rhs: XYZW) -> Self::Output {
        WY(self.0 * rhs.0)
    }
}

impl Geometric<S> for XY {
    type Output = XY;
    fn geometric(self, rhs: S) -> Self::Output {
        XY(self.0 * rhs.0)
    }
}

impl Geometric<X> for XY {
    type Output = Y;
    fn geometric(self, rhs: X) -> Self::Output {
        Y(-self.0 * rhs.0)
    }
}

impl Geometric<Y> for XY {
    type Output = X;
    fn geometric(self, rhs: Y) -> Self::Output {
        X(self.0 * rhs.0)
    }
}

impl Geometric<Z> for XY {
    type Output = ZYX;
    fn geometric(self, rhs: Z) -> Self::Output {
        ZYX(-self.0 * rhs.0)
    }
}

impl Geometric<W> for XY {
    type Output = XYW;
    fn geometric(self, rhs: W) -> Self::Output {
        XYW(self.0 * rhs.0)
    }
}

impl Geometric<WX> for XY {
    type Output = WY;
    fn geometric(self, rhs: WX) -> Self::Output {
        WY(-self.0 * rhs.0)
    }
}

impl Geometric<WY> for XY {
    type Output = WX;
    fn geometric(self, rhs: WY) -> Self::Output {
        WX(self.0 * rhs.0)
    }
}

impl Geometric<WZ> for XY {
    type Output = XYZW;
    fn geometric(self, rhs: WZ) -> Self::Output {
        XYZW(-self.0 * rhs.0)
    }
}

impl Geometric<YZ> for XY {
    type Output = ZX;
    fn geometric(self, rhs: YZ) -> Self::Output {
        ZX(-self.0 * rhs.0)
    }
}

impl Geometric<ZX> for XY {
    type Output = YZ;
    fn geometric(self, rhs: ZX) -> Self::Output {
        YZ(self.0 * rhs.0)
    }
}

impl Geometric<XY> for XY {
    type Output = S;
    fn geometric(self, rhs: XY) -> Self::Output {
        S(-self.0 * rhs.0)
    }
}

impl Geometric<YZW> for XY {
    type Output = ZXW;
    fn geometric(self, rhs: YZW) -> Self::Output {
        ZXW(-self.0 * rhs.0)
    }
}

impl Geometric<ZXW> for XY {
    type Output = YZW;
    fn geometric(self, rhs: ZXW) -> Self::Output {
        YZW(self.0 * rhs.0)
    }
}

impl Geometric<XYW> for XY {
    type Output = W;
    fn geometric(self, rhs: XYW) -> Self::Output {
        W(-self.0 * rhs.0)
    }
}

impl Geometric<ZYX> for XY {
    type Output = Z;
    fn geometric(self, rhs: ZYX) -> Self::Output {
        Z(self.0 * rhs.0)
    }
}

impl Geometric<XYZW> for XY {
    type Output = WZ;
    fn geometric(self, rhs: XYZW) -> Self::Output {
        WZ(self.0 * rhs.0)
    }
}

impl Geometric<S> for YZW {
    type Output = YZW;
    fn geometric(self, rhs: S) -> Self::Output {
        YZW(self.0 * rhs.0)
    }
}

impl Geometric<X> for YZW {
    type Output = XYZW;
    fn geometric(self, rhs: X) -> Self::Output {
        XYZW(-self.0 * rhs.0)
    }
}

impl Geometric<Y> for YZW {
    type Output = WZ;
    fn geometric(self, rhs: Y) -> Self::Output {
        WZ(-self.0 * rhs.0)
    }
}

impl Geometric<Z> for YZW {
    type Output = WY;
    fn geometric(self, rhs: Z) -> Self::Output {
        WY(self.0 * rhs.0)
    }
}

impl Geometric<W> for YZW {
    type Output = Zero;
    fn geometric(self, _rhs: W) -> Self::Output {
        Zero {}
    }
}

impl Geometric<WX> for YZW {
    type Output = Zero;
    fn geometric(self, _rhs: WX) -> Self::Output {
        Zero {}
    }
}

impl Geometric<WY> for YZW {
    type Output = Zero;
    fn geometric(self, _rhs: WY) -> Self::Output {
        Zero {}
    }
}

impl Geometric<WZ> for YZW {
    type Output = Zero;
    fn geometric(self, _rhs: WZ) -> Self::Output {
        Zero {}
    }
}

impl Geometric<YZ> for YZW {
    type Output = W;
    fn geometric(self, rhs: YZ) -> Self::Output {
        W(-self.0 * rhs.0)
    }
}

impl Geometric<ZX> for YZW {
    type Output = XYW;
    fn geometric(self, rhs: ZX) -> Self::Output {
        XYW(-self.0 * rhs.0)
    }
}

impl Geometric<XY> for YZW {
    type Output = ZXW;
    fn geometric(self, rhs: XY) -> Self::Output {
        ZXW(self.0 * rhs.0)
    }
}

impl Geometric<YZW> for YZW {
    type Output = Zero;
    fn geometric(self, _rhs: YZW) -> Self::Output {
        Zero {}
    }
}

impl Geometric<ZXW> for YZW {
    type Output = Zero;
    fn geometric(self, _rhs: ZXW) -> Self::Output {
        Zero {}
    }
}

impl Geometric<XYW> for YZW {
    type Output = Zero;
    fn geometric(self, _rhs: XYW) -> Self::Output {
        Zero {}
    }
}

impl Geometric<ZYX> for YZW {
    type Output = WX;
    fn geometric(self, rhs: ZYX) -> Self::Output {
        WX(self.0 * rhs.0)
    }
}

impl Geometric<XYZW> for YZW {
    type Output = Zero;
    fn geometric(self, _rhs: XYZW) -> Self::Output {
        Zero {}
    }
}

impl Geometric<S> for ZXW {
    type Output = ZXW;
    fn geometric(self, rhs: S) -> Self::Output {
        ZXW(self.0 * rhs.0)
    }
}

impl Geometric<X> for ZXW {
    type Output = WZ;
    fn geometric(self, rhs: X) -> Self::Output {
        WZ(self.0 * rhs.0)
    }
}

impl Geometric<Y> for ZXW {
    type Output = XYZW;
    fn geometric(self, rhs: Y) -> Self::Output {
        XYZW(-self.0 * rhs.0)
    }
}

impl Geometric<Z> for ZXW {
    type Output = WX;
    fn geometric(self, rhs: Z) -> Self::Output {
        WX(-self.0 * rhs.0)
    }
}

impl Geometric<W> for ZXW {
    type Output = Zero;
    fn geometric(self, _rhs: W) -> Self::Output {
        Zero {}
    }
}

impl Geometric<WX> for ZXW {
    type Output = Zero;
    fn geometric(self, _rhs: WX) -> Self::Output {
        Zero {}
    }
}

impl Geometric<WY> for ZXW {
    type Output = Zero;
    fn geometric(self, _rhs: WY) -> Self::Output {
        Zero {}
    }
}

impl Geometric<WZ> for ZXW {
    type Output = Zero;
    fn geometric(self, _rhs: WZ) -> Self::Output {
        Zero {}
    }
}

impl Geometric<YZ> for ZXW {
    type Output = XYW;
    fn geometric(self, rhs: YZ) -> Self::Output {
        XYW(self.0 * rhs.0)
    }
}

impl Geometric<ZX> for ZXW {
    type Output = W;
    fn geometric(self, rhs: ZX) -> Self::Output {
        W(-self.0 * rhs.0)
    }
}

impl Geometric<XY> for ZXW {
    type Output = YZW;
    fn geometric(self, rhs: XY) -> Self::Output {
        YZW(-self.0 * rhs.0)
    }
}

impl Geometric<YZW> for ZXW {
    type Output = Zero;
    fn geometric(self, _rhs: YZW) -> Self::Output {
        Zero {}
    }
}

impl Geometric<ZXW> for ZXW {
    type Output = Zero;
    fn geometric(self, _rhs: ZXW) -> Self::Output {
        Zero {}
    }
}

impl Geometric<XYW> for ZXW {
    type Output = Zero;
    fn geometric(self, _rhs: XYW) -> Self::Output {
        Zero {}
    }
}

impl Geometric<ZYX> for ZXW {
    type Output = WY;
    fn geometric(self, rhs: ZYX) -> Self::Output {
        WY(self.0 * rhs.0)
    }
}

impl Geometric<XYZW> for ZXW {
    type Output = Zero;
    fn geometric(self, _rhs: XYZW) -> Self::Output {
        Zero {}
    }
}

impl Geometric<S> for XYW {
    type Output = XYW;
    fn geometric(self, rhs: S) -> Self::Output {
        XYW(self.0 * rhs.0)
    }
}

impl Geometric<X> for XYW {
    type Output = WY;
    fn geometric(self, rhs: X) -> Self::Output {
        WY(-self.0 * rhs.0)
    }
}

impl Geometric<Y> for XYW {
    type Output = WX;
    fn geometric(self, rhs: Y) -> Self::Output {
        WX(self.0 * rhs.0)
    }
}

impl Geometric<Z> for XYW {
    type Output = XYZW;
    fn geometric(self, rhs: Z) -> Self::Output {
        XYZW(-self.0 * rhs.0)
    }
}

impl Geometric<W> for XYW {
    type Output = Zero;
    fn geometric(self, _rhs: W) -> Self::Output {
        Zero {}
    }
}

impl Geometric<WX> for XYW {
    type Output = Zero;
    fn geometric(self, _rhs: WX) -> Self::Output {
        Zero {}
    }
}

impl Geometric<WY> for XYW {
    type Output = Zero;
    fn geometric(self, _rhs: WY) -> Self::Output {
        Zero {}
    }
}

impl Geometric<WZ> for XYW {
    type Output = Zero;
    fn geometric(self, _rhs: WZ) -> Self::Output {
        Zero {}
    }
}

impl Geometric<YZ> for XYW {
    type Output = ZXW;
    fn geometric(self, rhs: YZ) -> Self::Output {
        ZXW(-self.0 * rhs.0)
    }
}

impl Geometric<ZX> for XYW {
    type Output = YZW;
    fn geometric(self, rhs: ZX) -> Self::Output {
        YZW(self.0 * rhs.0)
    }
}

impl Geometric<XY> for XYW {
    type Output = W;
    fn geometric(self, rhs: XY) -> Self::Output {
        W(-self.0 * rhs.0)
    }
}

impl Geometric<YZW> for XYW {
    type Output = Zero;
    fn geometric(self, _rhs: YZW) -> Self::Output {
        Zero {}
    }
}

impl Geometric<ZXW> for XYW {
    type Output = Zero;
    fn geometric(self, _rhs: ZXW) -> Self::Output {
        Zero {}
    }
}

impl Geometric<XYW> for XYW {
    type Output = Zero;
    fn geometric(self, _rhs: XYW) -> Self::Output {
        Zero {}
    }
}

impl Geometric<ZYX> for XYW {
    type Output = WZ;
    fn geometric(self, rhs: ZYX) -> Self::Output {
        WZ(self.0 * rhs.0)
    }
}

impl Geometric<XYZW> for XYW {
    type Output = Zero;
    fn geometric(self, _rhs: XYZW) -> Self::Output {
        Zero {}
    }
}

impl Geometric<S> for ZYX {
    type Output = ZYX;
    fn geometric(self, rhs: S) -> Self::Output {
        ZYX(self.0 * rhs.0)
    }
}

impl Geometric<X> for ZYX {
    type Output = YZ;
    fn geometric(self, rhs: X) -> Self::Output {
        YZ(-self.0 * rhs.0)
    }
}

impl Geometric<Y> for ZYX {
    type Output = ZX;
    fn geometric(self, rhs: Y) -> Self::Output {
        ZX(-self.0 * rhs.0)
    }
}

impl Geometric<Z> for ZYX {
    type Output = XY;
    fn geometric(self, rhs: Z) -> Self::Output {
        XY(-self.0 * rhs.0)
    }
}

impl Geometric<W> for ZYX {
    type Output = XYZW;
    fn geometric(self, rhs: W) -> Self::Output {
        XYZW(-self.0 * rhs.0)
    }
}

impl Geometric<WX> for ZYX {
    type Output = YZW;
    fn geometric(self, rhs: WX) -> Self::Output {
        YZW(self.0 * rhs.0)
    }
}

impl Geometric<WY> for ZYX {
    type Output = ZXW;
    fn geometric(self, rhs: WY) -> Self::Output {
        ZXW(self.0 * rhs.0)
    }
}

impl Geometric<WZ> for ZYX {
    type Output = XYW;
    fn geometric(self, rhs: WZ) -> Self::Output {
        XYW(self.0 * rhs.0)
    }
}

impl Geometric<YZ> for ZYX {
    type Output = X;
    fn geometric(self, rhs: YZ) -> Self::Output {
        X(self.0 * rhs.0)
    }
}

impl Geometric<ZX> for ZYX {
    type Output = Y;
    fn geometric(self, rhs: ZX) -> Self::Output {
        Y(self.0 * rhs.0)
    }
}

impl Geometric<XY> for ZYX {
    type Output = Z;
    fn geometric(self, rhs: XY) -> Self::Output {
        Z(self.0 * rhs.0)
    }
}

impl Geometric<YZW> for ZYX {
    type Output = WX;
    fn geometric(self, rhs: YZW) -> Self::Output {
        WX(-self.0 * rhs.0)
    }
}

impl Geometric<ZXW> for ZYX {
    type Output = WY;
    fn geometric(self, rhs: ZXW) -> Self::Output {
        WY(-self.0 * rhs.0)
    }
}

impl Geometric<XYW> for ZYX {
    type Output = WZ;
    fn geometric(self, rhs: XYW) -> Self::Output {
        WZ(-self.0 * rhs.0)
    }
}

impl Geometric<ZYX> for ZYX {
    type Output = S;
    fn geometric(self, rhs: ZYX) -> Self::Output {
        S(-self.0 * rhs.0)
    }
}

impl Geometric<XYZW> for ZYX {
    type Output = W;
    fn geometric(self, rhs: XYZW) -> Self::Output {
        W(self.0 * rhs.0)
    }
}

impl Geometric<S> for XYZW {
    type Output = XYZW;
    fn geometric(self, rhs: S) -> Self::Output {
        XYZW(self.0 * rhs.0)
    }
}

impl Geometric<X> for XYZW {
    type Output = YZW;
    fn geometric(self, rhs: X) -> Self::Output {
        YZW(-self.0 * rhs.0)
    }
}

impl Geometric<Y> for XYZW {
    type Output = ZXW;
    fn geometric(self, rhs: Y) -> Self::Output {
        ZXW(-self.0 * rhs.0)
    }
}

impl Geometric<Z> for XYZW {
    type Output = XYW;
    fn geometric(self, rhs: Z) -> Self::Output {
        XYW(-self.0 * rhs.0)
    }
}

impl Geometric<W> for XYZW {
    type Output = Zero;
    fn geometric(self, _rhs: W) -> Self::Output {
        Zero {}
    }
}

impl Geometric<WX> for XYZW {
    type Output = Zero;
    fn geometric(self, _rhs: WX) -> Self::Output {
        Zero {}
    }
}

impl Geometric<WY> for XYZW {
    type Output = Zero;
    fn geometric(self, _rhs: WY) -> Self::Output {
        Zero {}
    }
}

impl Geometric<WZ> for XYZW {
    type Output = Zero;
    fn geometric(self, _rhs: WZ) -> Self::Output {
        Zero {}
    }
}

impl Geometric<YZ> for XYZW {
    type Output = WX;
    fn geometric(self, rhs: YZ) -> Self::Output {
        WX(self.0 * rhs.0)
    }
}

impl Geometric<ZX> for XYZW {
    type Output = WY;
    fn geometric(self, rhs: ZX) -> Self::Output {
        WY(self.0 * rhs.0)
    }
}

impl Geometric<XY> for XYZW {
    type Output = WZ;
    fn geometric(self, rhs: XY) -> Self::Output {
        WZ(self.0 * rhs.0)
    }
}

impl Geometric<YZW> for XYZW {
    type Output = Zero;
    fn geometric(self, _rhs: YZW) -> Self::Output {
        Zero {}
    }
}

impl Geometric<ZXW> for XYZW {
    type Output = Zero;
    fn geometric(self, _rhs: ZXW) -> Self::Output {
        Zero {}
    }
}

impl Geometric<XYW> for XYZW {
    type Output = Zero;
    fn geometric(self, _rhs: XYW) -> Self::Output {
        Zero {}
    }
}

impl Geometric<ZYX> for XYZW {
    type Output = W;
    fn geometric(self, rhs: ZYX) -> Self::Output {
        W(-self.0 * rhs.0)
    }
}

impl Geometric<XYZW> for XYZW {
    type Output = Zero;
    fn geometric(self, _rhs: XYZW) -> Self::Output {
        Zero {}
    }
}

// ---------------------------------------------------------------------
// impl AntiGeometric for blades:

impl AntiGeometric<S> for S {
    type Output = Zero;
    fn anti_geometric(self, _rhs: S) -> Self::Output {
        Zero {}
    }
}

impl AntiGeometric<X> for S {
    type Output = Zero;
    fn anti_geometric(self, _rhs: X) -> Self::Output {
        Zero {}
    }
}

impl AntiGeometric<Y> for S {
    type Output = Zero;
    fn anti_geometric(self, _rhs: Y) -> Self::Output {
        Zero {}
    }
}

impl AntiGeometric<Z> for S {
    type Output = Zero;
    fn anti_geometric(self, _rhs: Z) -> Self::Output {
        Zero {}
    }
}

impl AntiGeometric<W> for S {
    type Output = ZYX;
    fn anti_geometric(self, rhs: W) -> Self::Output {
        ZYX(self.0 * rhs.0)
    }
}

impl AntiGeometric<WX> for S {
    type Output = YZ;
    fn anti_geometric(self, rhs: WX) -> Self::Output {
        YZ(self.0 * rhs.0)
    }
}

impl AntiGeometric<WY> for S {
    type Output = ZX;
    fn anti_geometric(self, rhs: WY) -> Self::Output {
        ZX(self.0 * rhs.0)
    }
}

impl AntiGeometric<WZ> for S {
    type Output = XY;
    fn anti_geometric(self, rhs: WZ) -> Self::Output {
        XY(self.0 * rhs.0)
    }
}

impl AntiGeometric<YZ> for S {
    type Output = Zero;
    fn anti_geometric(self, _rhs: YZ) -> Self::Output {
        Zero {}
    }
}

impl AntiGeometric<ZX> for S {
    type Output = Zero;
    fn anti_geometric(self, _rhs: ZX) -> Self::Output {
        Zero {}
    }
}

impl AntiGeometric<XY> for S {
    type Output = Zero;
    fn anti_geometric(self, _rhs: XY) -> Self::Output {
        Zero {}
    }
}

impl AntiGeometric<YZW> for S {
    type Output = X;
    fn anti_geometric(self, rhs: YZW) -> Self::Output {
        X(self.0 * rhs.0)
    }
}

impl AntiGeometric<ZXW> for S {
    type Output = Y;
    fn anti_geometric(self, rhs: ZXW) -> Self::Output {
        Y(self.0 * rhs.0)
    }
}

impl AntiGeometric<XYW> for S {
    type Output = Z;
    fn anti_geometric(self, rhs: XYW) -> Self::Output {
        Z(self.0 * rhs.0)
    }
}

impl AntiGeometric<ZYX> for S {
    type Output = Zero;
    fn anti_geometric(self, _rhs: ZYX) -> Self::Output {
        Zero {}
    }
}

impl AntiGeometric<XYZW> for S {
    type Output = S;
    fn anti_geometric(self, rhs: XYZW) -> Self::Output {
        S(self.0 * rhs.0)
    }
}

impl AntiGeometric<S> for X {
    type Output = Zero;
    fn anti_geometric(self, _rhs: S) -> Self::Output {
        Zero {}
    }
}

impl AntiGeometric<X> for X {
    type Output = Zero;
    fn anti_geometric(self, _rhs: X) -> Self::Output {
        Zero {}
    }
}

impl AntiGeometric<Y> for X {
    type Output = Zero;
    fn anti_geometric(self, _rhs: Y) -> Self::Output {
        Zero {}
    }
}

impl AntiGeometric<Z> for X {
    type Output = Zero;
    fn anti_geometric(self, _rhs: Z) -> Self::Output {
        Zero {}
    }
}

impl AntiGeometric<W> for X {
    type Output = YZ;
    fn anti_geometric(self, rhs: W) -> Self::Output {
        YZ(-self.0 * rhs.0)
    }
}

impl AntiGeometric<WX> for X {
    type Output = ZYX;
    fn anti_geometric(self, rhs: WX) -> Self::Output {
        ZYX(-self.0 * rhs.0)
    }
}

impl AntiGeometric<WY> for X {
    type Output = Z;
    fn anti_geometric(self, rhs: WY) -> Self::Output {
        Z(self.0 * rhs.0)
    }
}

impl AntiGeometric<WZ> for X {
    type Output = Y;
    fn anti_geometric(self, rhs: WZ) -> Self::Output {
        Y(-self.0 * rhs.0)
    }
}

impl AntiGeometric<YZ> for X {
    type Output = Zero;
    fn anti_geometric(self, _rhs: YZ) -> Self::Output {
        Zero {}
    }
}

impl AntiGeometric<ZX> for X {
    type Output = Zero;
    fn anti_geometric(self, _rhs: ZX) -> Self::Output {
        Zero {}
    }
}

impl AntiGeometric<XY> for X {
    type Output = Zero;
    fn anti_geometric(self, _rhs: XY) -> Self::Output {
        Zero {}
    }
}

impl AntiGeometric<YZW> for X {
    type Output = S;
    fn anti_geometric(self, rhs: YZW) -> Self::Output {
        S(self.0 * rhs.0)
    }
}

impl AntiGeometric<ZXW> for X {
    type Output = XY;
    fn anti_geometric(self, rhs: ZXW) -> Self::Output {
        XY(-self.0 * rhs.0)
    }
}

impl AntiGeometric<XYW> for X {
    type Output = ZX;
    fn anti_geometric(self, rhs: XYW) -> Self::Output {
        ZX(self.0 * rhs.0)
    }
}

impl AntiGeometric<ZYX> for X {
    type Output = Zero;
    fn anti_geometric(self, _rhs: ZYX) -> Self::Output {
        Zero {}
    }
}

impl AntiGeometric<XYZW> for X {
    type Output = X;
    fn anti_geometric(self, rhs: XYZW) -> Self::Output {
        X(self.0 * rhs.0)
    }
}

impl AntiGeometric<S> for Y {
    type Output = Zero;
    fn anti_geometric(self, _rhs: S) -> Self::Output {
        Zero {}
    }
}

impl AntiGeometric<X> for Y {
    type Output = Zero;
    fn anti_geometric(self, _rhs: X) -> Self::Output {
        Zero {}
    }
}

impl AntiGeometric<Y> for Y {
    type Output = Zero;
    fn anti_geometric(self, _rhs: Y) -> Self::Output {
        Zero {}
    }
}

impl AntiGeometric<Z> for Y {
    type Output = Zero;
    fn anti_geometric(self, _rhs: Z) -> Self::Output {
        Zero {}
    }
}

impl AntiGeometric<W> for Y {
    type Output = ZX;
    fn anti_geometric(self, rhs: W) -> Self::Output {
        ZX(-self.0 * rhs.0)
    }
}

impl AntiGeometric<WX> for Y {
    type Output = Z;
    fn anti_geometric(self, rhs: WX) -> Self::Output {
        Z(-self.0 * rhs.0)
    }
}

impl AntiGeometric<WY> for Y {
    type Output = ZYX;
    fn anti_geometric(self, rhs: WY) -> Self::Output {
        ZYX(-self.0 * rhs.0)
    }
}

impl AntiGeometric<WZ> for Y {
    type Output = X;
    fn anti_geometric(self, rhs: WZ) -> Self::Output {
        X(self.0 * rhs.0)
    }
}

impl AntiGeometric<YZ> for Y {
    type Output = Zero;
    fn anti_geometric(self, _rhs: YZ) -> Self::Output {
        Zero {}
    }
}

impl AntiGeometric<ZX> for Y {
    type Output = Zero;
    fn anti_geometric(self, _rhs: ZX) -> Self::Output {
        Zero {}
    }
}

impl AntiGeometric<XY> for Y {
    type Output = Zero;
    fn anti_geometric(self, _rhs: XY) -> Self::Output {
        Zero {}
    }
}

impl AntiGeometric<YZW> for Y {
    type Output = XY;
    fn anti_geometric(self, rhs: YZW) -> Self::Output {
        XY(self.0 * rhs.0)
    }
}

impl AntiGeometric<ZXW> for Y {
    type Output = S;
    fn anti_geometric(self, rhs: ZXW) -> Self::Output {
        S(self.0 * rhs.0)
    }
}

impl AntiGeometric<XYW> for Y {
    type Output = YZ;
    fn anti_geometric(self, rhs: XYW) -> Self::Output {
        YZ(-self.0 * rhs.0)
    }
}

impl AntiGeometric<ZYX> for Y {
    type Output = Zero;
    fn anti_geometric(self, _rhs: ZYX) -> Self::Output {
        Zero {}
    }
}

impl AntiGeometric<XYZW> for Y {
    type Output = Y;
    fn anti_geometric(self, rhs: XYZW) -> Self::Output {
        Y(self.0 * rhs.0)
    }
}

impl AntiGeometric<S> for Z {
    type Output = Zero;
    fn anti_geometric(self, _rhs: S) -> Self::Output {
        Zero {}
    }
}

impl AntiGeometric<X> for Z {
    type Output = Zero;
    fn anti_geometric(self, _rhs: X) -> Self::Output {
        Zero {}
    }
}

impl AntiGeometric<Y> for Z {
    type Output = Zero;
    fn anti_geometric(self, _rhs: Y) -> Self::Output {
        Zero {}
    }
}

impl AntiGeometric<Z> for Z {
    type Output = Zero;
    fn anti_geometric(self, _rhs: Z) -> Self::Output {
        Zero {}
    }
}

impl AntiGeometric<W> for Z {
    type Output = XY;
    fn anti_geometric(self, rhs: W) -> Self::Output {
        XY(-self.0 * rhs.0)
    }
}

impl AntiGeometric<WX> for Z {
    type Output = Y;
    fn anti_geometric(self, rhs: WX) -> Self::Output {
        Y(self.0 * rhs.0)
    }
}

impl AntiGeometric<WY> for Z {
    type Output = X;
    fn anti_geometric(self, rhs: WY) -> Self::Output {
        X(-self.0 * rhs.0)
    }
}

impl AntiGeometric<WZ> for Z {
    type Output = ZYX;
    fn anti_geometric(self, rhs: WZ) -> Self::Output {
        ZYX(-self.0 * rhs.0)
    }
}

impl AntiGeometric<YZ> for Z {
    type Output = Zero;
    fn anti_geometric(self, _rhs: YZ) -> Self::Output {
        Zero {}
    }
}

impl AntiGeometric<ZX> for Z {
    type Output = Zero;
    fn anti_geometric(self, _rhs: ZX) -> Self::Output {
        Zero {}
    }
}

impl AntiGeometric<XY> for Z {
    type Output = Zero;
    fn anti_geometric(self, _rhs: XY) -> Self::Output {
        Zero {}
    }
}

impl AntiGeometric<YZW> for Z {
    type Output = ZX;
    fn anti_geometric(self, rhs: YZW) -> Self::Output {
        ZX(-self.0 * rhs.0)
    }
}

impl AntiGeometric<ZXW> for Z {
    type Output = YZ;
    fn anti_geometric(self, rhs: ZXW) -> Self::Output {
        YZ(self.0 * rhs.0)
    }
}

impl AntiGeometric<XYW> for Z {
    type Output = S;
    fn anti_geometric(self, rhs: XYW) -> Self::Output {
        S(self.0 * rhs.0)
    }
}

impl AntiGeometric<ZYX> for Z {
    type Output = Zero;
    fn anti_geometric(self, _rhs: ZYX) -> Self::Output {
        Zero {}
    }
}

impl AntiGeometric<XYZW> for Z {
    type Output = Z;
    fn anti_geometric(self, rhs: XYZW) -> Self::Output {
        Z(self.0 * rhs.0)
    }
}

impl AntiGeometric<S> for W {
    type Output = ZYX;
    fn anti_geometric(self, rhs: S) -> Self::Output {
        ZYX(-self.0 * rhs.0)
    }
}

impl AntiGeometric<X> for W {
    type Output = YZ;
    fn anti_geometric(self, rhs: X) -> Self::Output {
        YZ(self.0 * rhs.0)
    }
}

impl AntiGeometric<Y> for W {
    type Output = ZX;
    fn anti_geometric(self, rhs: Y) -> Self::Output {
        ZX(self.0 * rhs.0)
    }
}

impl AntiGeometric<Z> for W {
    type Output = XY;
    fn anti_geometric(self, rhs: Z) -> Self::Output {
        XY(self.0 * rhs.0)
    }
}

impl AntiGeometric<W> for W {
    type Output = XYZW;
    fn anti_geometric(self, rhs: W) -> Self::Output {
        XYZW(-self.0 * rhs.0)
    }
}

impl AntiGeometric<WX> for W {
    type Output = YZW;
    fn anti_geometric(self, rhs: WX) -> Self::Output {
        YZW(self.0 * rhs.0)
    }
}

impl AntiGeometric<WY> for W {
    type Output = ZXW;
    fn anti_geometric(self, rhs: WY) -> Self::Output {
        ZXW(self.0 * rhs.0)
    }
}

impl AntiGeometric<WZ> for W {
    type Output = XYW;
    fn anti_geometric(self, rhs: WZ) -> Self::Output {
        XYW(self.0 * rhs.0)
    }
}

impl AntiGeometric<YZ> for W {
    type Output = X;
    fn anti_geometric(self, rhs: YZ) -> Self::Output {
        X(-self.0 * rhs.0)
    }
}

impl AntiGeometric<ZX> for W {
    type Output = Y;
    fn anti_geometric(self, rhs: ZX) -> Self::Output {
        Y(-self.0 * rhs.0)
    }
}

impl AntiGeometric<XY> for W {
    type Output = Z;
    fn anti_geometric(self, rhs: XY) -> Self::Output {
        Z(-self.0 * rhs.0)
    }
}

impl AntiGeometric<YZW> for W {
    type Output = WX;
    fn anti_geometric(self, rhs: YZW) -> Self::Output {
        WX(-self.0 * rhs.0)
    }
}

impl AntiGeometric<ZXW> for W {
    type Output = WY;
    fn anti_geometric(self, rhs: ZXW) -> Self::Output {
        WY(-self.0 * rhs.0)
    }
}

impl AntiGeometric<XYW> for W {
    type Output = WZ;
    fn anti_geometric(self, rhs: XYW) -> Self::Output {
        WZ(-self.0 * rhs.0)
    }
}

impl AntiGeometric<ZYX> for W {
    type Output = S;
    fn anti_geometric(self, rhs: ZYX) -> Self::Output {
        S(self.0 * rhs.0)
    }
}

impl AntiGeometric<XYZW> for W {
    type Output = W;
    fn anti_geometric(self, rhs: XYZW) -> Self::Output {
        W(self.0 * rhs.0)
    }
}

impl AntiGeometric<S> for WX {
    type Output = YZ;
    fn anti_geometric(self, rhs: S) -> Self::Output {
        YZ(self.0 * rhs.0)
    }
}

impl AntiGeometric<X> for WX {
    type Output = ZYX;
    fn anti_geometric(self, rhs: X) -> Self::Output {
        ZYX(-self.0 * rhs.0)
    }
}

impl AntiGeometric<Y> for WX {
    type Output = Z;
    fn anti_geometric(self, rhs: Y) -> Self::Output {
        Z(self.0 * rhs.0)
    }
}

impl AntiGeometric<Z> for WX {
    type Output = Y;
    fn anti_geometric(self, rhs: Z) -> Self::Output {
        Y(-self.0 * rhs.0)
    }
}

impl AntiGeometric<W> for WX {
    type Output = YZW;
    fn anti_geometric(self, rhs: W) -> Self::Output {
        YZW(self.0 * rhs.0)
    }
}

impl AntiGeometric<WX> for WX {
    type Output = XYZW;
    fn anti_geometric(self, rhs: WX) -> Self::Output {
        XYZW(-self.0 * rhs.0)
    }
}

impl AntiGeometric<WY> for WX {
    type Output = WZ;
    fn anti_geometric(self, rhs: WY) -> Self::Output {
        WZ(self.0 * rhs.0)
    }
}

impl AntiGeometric<WZ> for WX {
    type Output = WY;
    fn anti_geometric(self, rhs: WZ) -> Self::Output {
        WY(-self.0 * rhs.0)
    }
}

impl AntiGeometric<YZ> for WX {
    type Output = S;
    fn anti_geometric(self, rhs: YZ) -> Self::Output {
        S(-self.0 * rhs.0)
    }
}

impl AntiGeometric<ZX> for WX {
    type Output = XY;
    fn anti_geometric(self, rhs: ZX) -> Self::Output {
        XY(self.0 * rhs.0)
    }
}

impl AntiGeometric<XY> for WX {
    type Output = ZX;
    fn anti_geometric(self, rhs: XY) -> Self::Output {
        ZX(-self.0 * rhs.0)
    }
}

impl AntiGeometric<YZW> for WX {
    type Output = W;
    fn anti_geometric(self, rhs: YZW) -> Self::Output {
        W(-self.0 * rhs.0)
    }
}

impl AntiGeometric<ZXW> for WX {
    type Output = XYW;
    fn anti_geometric(self, rhs: ZXW) -> Self::Output {
        XYW(self.0 * rhs.0)
    }
}

impl AntiGeometric<XYW> for WX {
    type Output = ZXW;
    fn anti_geometric(self, rhs: XYW) -> Self::Output {
        ZXW(-self.0 * rhs.0)
    }
}

impl AntiGeometric<ZYX> for WX {
    type Output = X;
    fn anti_geometric(self, rhs: ZYX) -> Self::Output {
        X(self.0 * rhs.0)
    }
}

impl AntiGeometric<XYZW> for WX {
    type Output = WX;
    fn anti_geometric(self, rhs: XYZW) -> Self::Output {
        WX(self.0 * rhs.0)
    }
}

impl AntiGeometric<S> for WY {
    type Output = ZX;
    fn anti_geometric(self, rhs: S) -> Self::Output {
        ZX(self.0 * rhs.0)
    }
}

impl AntiGeometric<X> for WY {
    type Output = Z;
    fn anti_geometric(self, rhs: X) -> Self::Output {
        Z(-self.0 * rhs.0)
    }
}

impl AntiGeometric<Y> for WY {
    type Output = ZYX;
    fn anti_geometric(self, rhs: Y) -> Self::Output {
        ZYX(-self.0 * rhs.0)
    }
}

impl AntiGeometric<Z> for WY {
    type Output = X;
    fn anti_geometric(self, rhs: Z) -> Self::Output {
        X(self.0 * rhs.0)
    }
}

impl AntiGeometric<W> for WY {
    type Output = ZXW;
    fn anti_geometric(self, rhs: W) -> Self::Output {
        ZXW(self.0 * rhs.0)
    }
}

impl AntiGeometric<WX> for WY {
    type Output = WZ;
    fn anti_geometric(self, rhs: WX) -> Self::Output {
        WZ(-self.0 * rhs.0)
    }
}

impl AntiGeometric<WY> for WY {
    type Output = XYZW;
    fn anti_geometric(self, rhs: WY) -> Self::Output {
        XYZW(-self.0 * rhs.0)
    }
}

impl AntiGeometric<WZ> for WY {
    type Output = WX;
    fn anti_geometric(self, rhs: WZ) -> Self::Output {
        WX(self.0 * rhs.0)
    }
}

impl AntiGeometric<YZ> for WY {
    type Output = XY;
    fn anti_geometric(self, rhs: YZ) -> Self::Output {
        XY(-self.0 * rhs.0)
    }
}

impl AntiGeometric<ZX> for WY {
    type Output = S;
    fn anti_geometric(self, rhs: ZX) -> Self::Output {
        S(-self.0 * rhs.0)
    }
}

impl AntiGeometric<XY> for WY {
    type Output = YZ;
    fn anti_geometric(self, rhs: XY) -> Self::Output {
        YZ(self.0 * rhs.0)
    }
}

impl AntiGeometric<YZW> for WY {
    type Output = XYW;
    fn anti_geometric(self, rhs: YZW) -> Self::Output {
        XYW(-self.0 * rhs.0)
    }
}

impl AntiGeometric<ZXW> for WY {
    type Output = W;
    fn anti_geometric(self, rhs: ZXW) -> Self::Output {
        W(-self.0 * rhs.0)
    }
}

impl AntiGeometric<XYW> for WY {
    type Output = YZW;
    fn anti_geometric(self, rhs: XYW) -> Self::Output {
        YZW(self.0 * rhs.0)
    }
}

impl AntiGeometric<ZYX> for WY {
    type Output = Y;
    fn anti_geometric(self, rhs: ZYX) -> Self::Output {
        Y(self.0 * rhs.0)
    }
}

impl AntiGeometric<XYZW> for WY {
    type Output = WY;
    fn anti_geometric(self, rhs: XYZW) -> Self::Output {
        WY(self.0 * rhs.0)
    }
}

impl AntiGeometric<S> for WZ {
    type Output = XY;
    fn anti_geometric(self, rhs: S) -> Self::Output {
        XY(self.0 * rhs.0)
    }
}

impl AntiGeometric<X> for WZ {
    type Output = Y;
    fn anti_geometric(self, rhs: X) -> Self::Output {
        Y(self.0 * rhs.0)
    }
}

impl AntiGeometric<Y> for WZ {
    type Output = X;
    fn anti_geometric(self, rhs: Y) -> Self::Output {
        X(-self.0 * rhs.0)
    }
}

impl AntiGeometric<Z> for WZ {
    type Output = ZYX;
    fn anti_geometric(self, rhs: Z) -> Self::Output {
        ZYX(-self.0 * rhs.0)
    }
}

impl AntiGeometric<W> for WZ {
    type Output = XYW;
    fn anti_geometric(self, rhs: W) -> Self::Output {
        XYW(self.0 * rhs.0)
    }
}

impl AntiGeometric<WX> for WZ {
    type Output = WY;
    fn anti_geometric(self, rhs: WX) -> Self::Output {
        WY(self.0 * rhs.0)
    }
}

impl AntiGeometric<WY> for WZ {
    type Output = WX;
    fn anti_geometric(self, rhs: WY) -> Self::Output {
        WX(-self.0 * rhs.0)
    }
}

impl AntiGeometric<WZ> for WZ {
    type Output = XYZW;
    fn anti_geometric(self, rhs: WZ) -> Self::Output {
        XYZW(-self.0 * rhs.0)
    }
}

impl AntiGeometric<YZ> for WZ {
    type Output = ZX;
    fn anti_geometric(self, rhs: YZ) -> Self::Output {
        ZX(self.0 * rhs.0)
    }
}

impl AntiGeometric<ZX> for WZ {
    type Output = YZ;
    fn anti_geometric(self, rhs: ZX) -> Self::Output {
        YZ(-self.0 * rhs.0)
    }
}

impl AntiGeometric<XY> for WZ {
    type Output = S;
    fn anti_geometric(self, rhs: XY) -> Self::Output {
        S(-self.0 * rhs.0)
    }
}

impl AntiGeometric<YZW> for WZ {
    type Output = ZXW;
    fn anti_geometric(self, rhs: YZW) -> Self::Output {
        ZXW(self.0 * rhs.0)
    }
}

impl AntiGeometric<ZXW> for WZ {
    type Output = YZW;
    fn anti_geometric(self, rhs: ZXW) -> Self::Output {
        YZW(-self.0 * rhs.0)
    }
}

impl AntiGeometric<XYW> for WZ {
    type Output = W;
    fn anti_geometric(self, rhs: XYW) -> Self::Output {
        W(-self.0 * rhs.0)
    }
}

impl AntiGeometric<ZYX> for WZ {
    type Output = Z;
    fn anti_geometric(self, rhs: ZYX) -> Self::Output {
        Z(self.0 * rhs.0)
    }
}

impl AntiGeometric<XYZW> for WZ {
    type Output = WZ;
    fn anti_geometric(self, rhs: XYZW) -> Self::Output {
        WZ(self.0 * rhs.0)
    }
}

impl AntiGeometric<S> for YZ {
    type Output = Zero;
    fn anti_geometric(self, _rhs: S) -> Self::Output {
        Zero {}
    }
}

impl AntiGeometric<X> for YZ {
    type Output = Zero;
    fn anti_geometric(self, _rhs: X) -> Self::Output {
        Zero {}
    }
}

impl AntiGeometric<Y> for YZ {
    type Output = Zero;
    fn anti_geometric(self, _rhs: Y) -> Self::Output {
        Zero {}
    }
}

impl AntiGeometric<Z> for YZ {
    type Output = Zero;
    fn anti_geometric(self, _rhs: Z) -> Self::Output {
        Zero {}
    }
}

impl AntiGeometric<W> for YZ {
    type Output = X;
    fn anti_geometric(self, rhs: W) -> Self::Output {
        X(self.0 * rhs.0)
    }
}

impl AntiGeometric<WX> for YZ {
    type Output = S;
    fn anti_geometric(self, rhs: WX) -> Self::Output {
        S(-self.0 * rhs.0)
    }
}

impl AntiGeometric<WY> for YZ {
    type Output = XY;
    fn anti_geometric(self, rhs: WY) -> Self::Output {
        XY(self.0 * rhs.0)
    }
}

impl AntiGeometric<WZ> for YZ {
    type Output = ZX;
    fn anti_geometric(self, rhs: WZ) -> Self::Output {
        ZX(-self.0 * rhs.0)
    }
}

impl AntiGeometric<YZ> for YZ {
    type Output = Zero;
    fn anti_geometric(self, _rhs: YZ) -> Self::Output {
        Zero {}
    }
}

impl AntiGeometric<ZX> for YZ {
    type Output = Zero;
    fn anti_geometric(self, _rhs: ZX) -> Self::Output {
        Zero {}
    }
}

impl AntiGeometric<XY> for YZ {
    type Output = Zero;
    fn anti_geometric(self, _rhs: XY) -> Self::Output {
        Zero {}
    }
}

impl AntiGeometric<YZW> for YZ {
    type Output = ZYX;
    fn anti_geometric(self, rhs: YZW) -> Self::Output {
        ZYX(-self.0 * rhs.0)
    }
}

impl AntiGeometric<ZXW> for YZ {
    type Output = Z;
    fn anti_geometric(self, rhs: ZXW) -> Self::Output {
        Z(self.0 * rhs.0)
    }
}

impl AntiGeometric<XYW> for YZ {
    type Output = Y;
    fn anti_geometric(self, rhs: XYW) -> Self::Output {
        Y(-self.0 * rhs.0)
    }
}

impl AntiGeometric<ZYX> for YZ {
    type Output = Zero;
    fn anti_geometric(self, _rhs: ZYX) -> Self::Output {
        Zero {}
    }
}

impl AntiGeometric<XYZW> for YZ {
    type Output = YZ;
    fn anti_geometric(self, rhs: XYZW) -> Self::Output {
        YZ(self.0 * rhs.0)
    }
}

impl AntiGeometric<S> for ZX {
    type Output = Zero;
    fn anti_geometric(self, _rhs: S) -> Self::Output {
        Zero {}
    }
}

impl AntiGeometric<X> for ZX {
    type Output = Zero;
    fn anti_geometric(self, _rhs: X) -> Self::Output {
        Zero {}
    }
}

impl AntiGeometric<Y> for ZX {
    type Output = Zero;
    fn anti_geometric(self, _rhs: Y) -> Self::Output {
        Zero {}
    }
}

impl AntiGeometric<Z> for ZX {
    type Output = Zero;
    fn anti_geometric(self, _rhs: Z) -> Self::Output {
        Zero {}
    }
}

impl AntiGeometric<W> for ZX {
    type Output = Y;
    fn anti_geometric(self, rhs: W) -> Self::Output {
        Y(self.0 * rhs.0)
    }
}

impl AntiGeometric<WX> for ZX {
    type Output = XY;
    fn anti_geometric(self, rhs: WX) -> Self::Output {
        XY(-self.0 * rhs.0)
    }
}

impl AntiGeometric<WY> for ZX {
    type Output = S;
    fn anti_geometric(self, rhs: WY) -> Self::Output {
        S(-self.0 * rhs.0)
    }
}

impl AntiGeometric<WZ> for ZX {
    type Output = YZ;
    fn anti_geometric(self, rhs: WZ) -> Self::Output {
        YZ(self.0 * rhs.0)
    }
}

impl AntiGeometric<YZ> for ZX {
    type Output = Zero;
    fn anti_geometric(self, _rhs: YZ) -> Self::Output {
        Zero {}
    }
}

impl AntiGeometric<ZX> for ZX {
    type Output = Zero;
    fn anti_geometric(self, _rhs: ZX) -> Self::Output {
        Zero {}
    }
}

impl AntiGeometric<XY> for ZX {
    type Output = Zero;
    fn anti_geometric(self, _rhs: XY) -> Self::Output {
        Zero {}
    }
}

impl AntiGeometric<YZW> for ZX {
    type Output = Z;
    fn anti_geometric(self, rhs: YZW) -> Self::Output {
        Z(-self.0 * rhs.0)
    }
}

impl AntiGeometric<ZXW> for ZX {
    type Output = ZYX;
    fn anti_geometric(self, rhs: ZXW) -> Self::Output {
        ZYX(-self.0 * rhs.0)
    }
}

impl AntiGeometric<XYW> for ZX {
    type Output = X;
    fn anti_geometric(self, rhs: XYW) -> Self::Output {
        X(self.0 * rhs.0)
    }
}

impl AntiGeometric<ZYX> for ZX {
    type Output = Zero;
    fn anti_geometric(self, _rhs: ZYX) -> Self::Output {
        Zero {}
    }
}

impl AntiGeometric<XYZW> for ZX {
    type Output = ZX;
    fn anti_geometric(self, rhs: XYZW) -> Self::Output {
        ZX(self.0 * rhs.0)
    }
}

impl AntiGeometric<S> for XY {
    type Output = Zero;
    fn anti_geometric(self, _rhs: S) -> Self::Output {
        Zero {}
    }
}

impl AntiGeometric<X> for XY {
    type Output = Zero;
    fn anti_geometric(self, _rhs: X) -> Self::Output {
        Zero {}
    }
}

impl AntiGeometric<Y> for XY {
    type Output = Zero;
    fn anti_geometric(self, _rhs: Y) -> Self::Output {
        Zero {}
    }
}

impl AntiGeometric<Z> for XY {
    type Output = Zero;
    fn anti_geometric(self, _rhs: Z) -> Self::Output {
        Zero {}
    }
}

impl AntiGeometric<W> for XY {
    type Output = Z;
    fn anti_geometric(self, rhs: W) -> Self::Output {
        Z(self.0 * rhs.0)
    }
}

impl AntiGeometric<WX> for XY {
    type Output = ZX;
    fn anti_geometric(self, rhs: WX) -> Self::Output {
        ZX(self.0 * rhs.0)
    }
}

impl AntiGeometric<WY> for XY {
    type Output = YZ;
    fn anti_geometric(self, rhs: WY) -> Self::Output {
        YZ(-self.0 * rhs.0)
    }
}

impl AntiGeometric<WZ> for XY {
    type Output = S;
    fn anti_geometric(self, rhs: WZ) -> Self::Output {
        S(-self.0 * rhs.0)
    }
}

impl AntiGeometric<YZ> for XY {
    type Output = Zero;
    fn anti_geometric(self, _rhs: YZ) -> Self::Output {
        Zero {}
    }
}

impl AntiGeometric<ZX> for XY {
    type Output = Zero;
    fn anti_geometric(self, _rhs: ZX) -> Self::Output {
        Zero {}
    }
}

impl AntiGeometric<XY> for XY {
    type Output = Zero;
    fn anti_geometric(self, _rhs: XY) -> Self::Output {
        Zero {}
    }
}

impl AntiGeometric<YZW> for XY {
    type Output = Y;
    fn anti_geometric(self, rhs: YZW) -> Self::Output {
        Y(self.0 * rhs.0)
    }
}

impl AntiGeometric<ZXW> for XY {
    type Output = X;
    fn anti_geometric(self, rhs: ZXW) -> Self::Output {
        X(-self.0 * rhs.0)
    }
}

impl AntiGeometric<XYW> for XY {
    type Output = ZYX;
    fn anti_geometric(self, rhs: XYW) -> Self::Output {
        ZYX(-self.0 * rhs.0)
    }
}

impl AntiGeometric<ZYX> for XY {
    type Output = Zero;
    fn anti_geometric(self, _rhs: ZYX) -> Self::Output {
        Zero {}
    }
}

impl AntiGeometric<XYZW> for XY {
    type Output = XY;
    fn anti_geometric(self, rhs: XYZW) -> Self::Output {
        XY(self.0 * rhs.0)
    }
}

impl AntiGeometric<S> for YZW {
    type Output = X;
    fn anti_geometric(self, rhs: S) -> Self::Output {
        X(-self.0 * rhs.0)
    }
}

impl AntiGeometric<X> for YZW {
    type Output = S;
    fn anti_geometric(self, rhs: X) -> Self::Output {
        S(-self.0 * rhs.0)
    }
}

impl AntiGeometric<Y> for YZW {
    type Output = XY;
    fn anti_geometric(self, rhs: Y) -> Self::Output {
        XY(self.0 * rhs.0)
    }
}

impl AntiGeometric<Z> for YZW {
    type Output = ZX;
    fn anti_geometric(self, rhs: Z) -> Self::Output {
        ZX(-self.0 * rhs.0)
    }
}

impl AntiGeometric<W> for YZW {
    type Output = WX;
    fn anti_geometric(self, rhs: W) -> Self::Output {
        WX(-self.0 * rhs.0)
    }
}

impl AntiGeometric<WX> for YZW {
    type Output = W;
    fn anti_geometric(self, rhs: WX) -> Self::Output {
        W(-self.0 * rhs.0)
    }
}

impl AntiGeometric<WY> for YZW {
    type Output = XYW;
    fn anti_geometric(self, rhs: WY) -> Self::Output {
        XYW(self.0 * rhs.0)
    }
}

impl AntiGeometric<WZ> for YZW {
    type Output = ZXW;
    fn anti_geometric(self, rhs: WZ) -> Self::Output {
        ZXW(-self.0 * rhs.0)
    }
}

impl AntiGeometric<YZ> for YZW {
    type Output = ZYX;
    fn anti_geometric(self, rhs: YZ) -> Self::Output {
        ZYX(self.0 * rhs.0)
    }
}

impl AntiGeometric<ZX> for YZW {
    type Output = Z;
    fn anti_geometric(self, rhs: ZX) -> Self::Output {
        Z(-self.0 * rhs.0)
    }
}

impl AntiGeometric<XY> for YZW {
    type Output = Y;
    fn anti_geometric(self, rhs: XY) -> Self::Output {
        Y(self.0 * rhs.0)
    }
}

impl AntiGeometric<YZW> for YZW {
    type Output = XYZW;
    fn anti_geometric(self, rhs: YZW) -> Self::Output {
        XYZW(self.0 * rhs.0)
    }
}

impl AntiGeometric<ZXW> for YZW {
    type Output = WZ;
    fn anti_geometric(self, rhs: ZXW) -> Self::Output {
        WZ(-self.0 * rhs.0)
    }
}

impl AntiGeometric<XYW> for YZW {
    type Output = WY;
    fn anti_geometric(self, rhs: XYW) -> Self::Output {
        WY(self.0 * rhs.0)
    }
}

impl AntiGeometric<ZYX> for YZW {
    type Output = YZ;
    fn anti_geometric(self, rhs: ZYX) -> Self::Output {
        YZ(self.0 * rhs.0)
    }
}

impl AntiGeometric<XYZW> for YZW {
    type Output = YZW;
    fn anti_geometric(self, rhs: XYZW) -> Self::Output {
        YZW(self.0 * rhs.0)
    }
}

impl AntiGeometric<S> for ZXW {
    type Output = Y;
    fn anti_geometric(self, rhs: S) -> Self::Output {
        Y(-self.0 * rhs.0)
    }
}

impl AntiGeometric<X> for ZXW {
    type Output = XY;
    fn anti_geometric(self, rhs: X) -> Self::Output {
        XY(-self.0 * rhs.0)
    }
}

impl AntiGeometric<Y> for ZXW {
    type Output = S;
    fn anti_geometric(self, rhs: Y) -> Self::Output {
        S(-self.0 * rhs.0)
    }
}

impl AntiGeometric<Z> for ZXW {
    type Output = YZ;
    fn anti_geometric(self, rhs: Z) -> Self::Output {
        YZ(self.0 * rhs.0)
    }
}

impl AntiGeometric<W> for ZXW {
    type Output = WY;
    fn anti_geometric(self, rhs: W) -> Self::Output {
        WY(-self.0 * rhs.0)
    }
}

impl AntiGeometric<WX> for ZXW {
    type Output = XYW;
    fn anti_geometric(self, rhs: WX) -> Self::Output {
        XYW(-self.0 * rhs.0)
    }
}

impl AntiGeometric<WY> for ZXW {
    type Output = W;
    fn anti_geometric(self, rhs: WY) -> Self::Output {
        W(-self.0 * rhs.0)
    }
}

impl AntiGeometric<WZ> for ZXW {
    type Output = YZW;
    fn anti_geometric(self, rhs: WZ) -> Self::Output {
        YZW(self.0 * rhs.0)
    }
}

impl AntiGeometric<YZ> for ZXW {
    type Output = Z;
    fn anti_geometric(self, rhs: YZ) -> Self::Output {
        Z(self.0 * rhs.0)
    }
}

impl AntiGeometric<ZX> for ZXW {
    type Output = ZYX;
    fn anti_geometric(self, rhs: ZX) -> Self::Output {
        ZYX(self.0 * rhs.0)
    }
}

impl AntiGeometric<XY> for ZXW {
    type Output = X;
    fn anti_geometric(self, rhs: XY) -> Self::Output {
        X(-self.0 * rhs.0)
    }
}

impl AntiGeometric<YZW> for ZXW {
    type Output = WZ;
    fn anti_geometric(self, rhs: YZW) -> Self::Output {
        WZ(self.0 * rhs.0)
    }
}

impl AntiGeometric<ZXW> for ZXW {
    type Output = XYZW;
    fn anti_geometric(self, rhs: ZXW) -> Self::Output {
        XYZW(self.0 * rhs.0)
    }
}

impl AntiGeometric<XYW> for ZXW {
    type Output = WX;
    fn anti_geometric(self, rhs: XYW) -> Self::Output {
        WX(-self.0 * rhs.0)
    }
}

impl AntiGeometric<ZYX> for ZXW {
    type Output = ZX;
    fn anti_geometric(self, rhs: ZYX) -> Self::Output {
        ZX(self.0 * rhs.0)
    }
}

impl AntiGeometric<XYZW> for ZXW {
    type Output = ZXW;
    fn anti_geometric(self, rhs: XYZW) -> Self::Output {
        ZXW(self.0 * rhs.0)
    }
}

impl AntiGeometric<S> for XYW {
    type Output = Z;
    fn anti_geometric(self, rhs: S) -> Self::Output {
        Z(-self.0 * rhs.0)
    }
}

impl AntiGeometric<X> for XYW {
    type Output = ZX;
    fn anti_geometric(self, rhs: X) -> Self::Output {
        ZX(self.0 * rhs.0)
    }
}

impl AntiGeometric<Y> for XYW {
    type Output = YZ;
    fn anti_geometric(self, rhs: Y) -> Self::Output {
        YZ(-self.0 * rhs.0)
    }
}

impl AntiGeometric<Z> for XYW {
    type Output = S;
    fn anti_geometric(self, rhs: Z) -> Self::Output {
        S(-self.0 * rhs.0)
    }
}

impl AntiGeometric<W> for XYW {
    type Output = WZ;
    fn anti_geometric(self, rhs: W) -> Self::Output {
        WZ(-self.0 * rhs.0)
    }
}

impl AntiGeometric<WX> for XYW {
    type Output = ZXW;
    fn anti_geometric(self, rhs: WX) -> Self::Output {
        ZXW(self.0 * rhs.0)
    }
}

impl AntiGeometric<WY> for XYW {
    type Output = YZW;
    fn anti_geometric(self, rhs: WY) -> Self::Output {
        YZW(-self.0 * rhs.0)
    }
}

impl AntiGeometric<WZ> for XYW {
    type Output = W;
    fn anti_geometric(self, rhs: WZ) -> Self::Output {
        W(-self.0 * rhs.0)
    }
}

impl AntiGeometric<YZ> for XYW {
    type Output = Y;
    fn anti_geometric(self, rhs: YZ) -> Self::Output {
        Y(-self.0 * rhs.0)
    }
}

impl AntiGeometric<ZX> for XYW {
    type Output = X;
    fn anti_geometric(self, rhs: ZX) -> Self::Output {
        X(self.0 * rhs.0)
    }
}

impl AntiGeometric<XY> for XYW {
    type Output = ZYX;
    fn anti_geometric(self, rhs: XY) -> Self::Output {
        ZYX(self.0 * rhs.0)
    }
}

impl AntiGeometric<YZW> for XYW {
    type Output = WY;
    fn anti_geometric(self, rhs: YZW) -> Self::Output {
        WY(-self.0 * rhs.0)
    }
}

impl AntiGeometric<ZXW> for XYW {
    type Output = WX;
    fn anti_geometric(self, rhs: ZXW) -> Self::Output {
        WX(self.0 * rhs.0)
    }
}

impl AntiGeometric<XYW> for XYW {
    type Output = XYZW;
    fn anti_geometric(self, rhs: XYW) -> Self::Output {
        XYZW(self.0 * rhs.0)
    }
}

impl AntiGeometric<ZYX> for XYW {
    type Output = XY;
    fn anti_geometric(self, rhs: ZYX) -> Self::Output {
        XY(self.0 * rhs.0)
    }
}

impl AntiGeometric<XYZW> for XYW {
    type Output = XYW;
    fn anti_geometric(self, rhs: XYZW) -> Self::Output {
        XYW(self.0 * rhs.0)
    }
}

impl AntiGeometric<S> for ZYX {
    type Output = Zero;
    fn anti_geometric(self, _rhs: S) -> Self::Output {
        Zero {}
    }
}

impl AntiGeometric<X> for ZYX {
    type Output = Zero;
    fn anti_geometric(self, _rhs: X) -> Self::Output {
        Zero {}
    }
}

impl AntiGeometric<Y> for ZYX {
    type Output = Zero;
    fn anti_geometric(self, _rhs: Y) -> Self::Output {
        Zero {}
    }
}

impl AntiGeometric<Z> for ZYX {
    type Output = Zero;
    fn anti_geometric(self, _rhs: Z) -> Self::Output {
        Zero {}
    }
}

impl AntiGeometric<W> for ZYX {
    type Output = S;
    fn anti_geometric(self, rhs: W) -> Self::Output {
        S(-self.0 * rhs.0)
    }
}

impl AntiGeometric<WX> for ZYX {
    type Output = X;
    fn anti_geometric(self, rhs: WX) -> Self::Output {
        X(self.0 * rhs.0)
    }
}

impl AntiGeometric<WY> for ZYX {
    type Output = Y;
    fn anti_geometric(self, rhs: WY) -> Self::Output {
        Y(self.0 * rhs.0)
    }
}

impl AntiGeometric<WZ> for ZYX {
    type Output = Z;
    fn anti_geometric(self, rhs: WZ) -> Self::Output {
        Z(self.0 * rhs.0)
    }
}

impl AntiGeometric<YZ> for ZYX {
    type Output = Zero;
    fn anti_geometric(self, _rhs: YZ) -> Self::Output {
        Zero {}
    }
}

impl AntiGeometric<ZX> for ZYX {
    type Output = Zero;
    fn anti_geometric(self, _rhs: ZX) -> Self::Output {
        Zero {}
    }
}

impl AntiGeometric<XY> for ZYX {
    type Output = Zero;
    fn anti_geometric(self, _rhs: XY) -> Self::Output {
        Zero {}
    }
}

impl AntiGeometric<YZW> for ZYX {
    type Output = YZ;
    fn anti_geometric(self, rhs: YZW) -> Self::Output {
        YZ(-self.0 * rhs.0)
    }
}

impl AntiGeometric<ZXW> for ZYX {
    type Output = ZX;
    fn anti_geometric(self, rhs: ZXW) -> Self::Output {
        ZX(-self.0 * rhs.0)
    }
}

impl AntiGeometric<XYW> for ZYX {
    type Output = XY;
    fn anti_geometric(self, rhs: XYW) -> Self::Output {
        XY(-self.0 * rhs.0)
    }
}

impl AntiGeometric<ZYX> for ZYX {
    type Output = Zero;
    fn anti_geometric(self, _rhs: ZYX) -> Self::Output {
        Zero {}
    }
}

impl AntiGeometric<XYZW> for ZYX {
    type Output = ZYX;
    fn anti_geometric(self, rhs: XYZW) -> Self::Output {
        ZYX(self.0 * rhs.0)
    }
}

impl AntiGeometric<S> for XYZW {
    type Output = S;
    fn anti_geometric(self, rhs: S) -> Self::Output {
        S(self.0 * rhs.0)
    }
}

impl AntiGeometric<X> for XYZW {
    type Output = X;
    fn anti_geometric(self, rhs: X) -> Self::Output {
        X(self.0 * rhs.0)
    }
}

impl AntiGeometric<Y> for XYZW {
    type Output = Y;
    fn anti_geometric(self, rhs: Y) -> Self::Output {
        Y(self.0 * rhs.0)
    }
}

impl AntiGeometric<Z> for XYZW {
    type Output = Z;
    fn anti_geometric(self, rhs: Z) -> Self::Output {
        Z(self.0 * rhs.0)
    }
}

impl AntiGeometric<W> for XYZW {
    type Output = W;
    fn anti_geometric(self, rhs: W) -> Self::Output {
        W(self.0 * rhs.0)
    }
}

impl AntiGeometric<WX> for XYZW {
    type Output = WX;
    fn anti_geometric(self, rhs: WX) -> Self::Output {
        WX(self.0 * rhs.0)
    }
}

impl AntiGeometric<WY> for XYZW {
    type Output = WY;
    fn anti_geometric(self, rhs: WY) -> Self::Output {
        WY(self.0 * rhs.0)
    }
}

impl AntiGeometric<WZ> for XYZW {
    type Output = WZ;
    fn anti_geometric(self, rhs: WZ) -> Self::Output {
        WZ(self.0 * rhs.0)
    }
}

impl AntiGeometric<YZ> for XYZW {
    type Output = YZ;
    fn anti_geometric(self, rhs: YZ) -> Self::Output {
        YZ(self.0 * rhs.0)
    }
}

impl AntiGeometric<ZX> for XYZW {
    type Output = ZX;
    fn anti_geometric(self, rhs: ZX) -> Self::Output {
        ZX(self.0 * rhs.0)
    }
}

impl AntiGeometric<XY> for XYZW {
    type Output = XY;
    fn anti_geometric(self, rhs: XY) -> Self::Output {
        XY(self.0 * rhs.0)
    }
}

impl AntiGeometric<YZW> for XYZW {
    type Output = YZW;
    fn anti_geometric(self, rhs: YZW) -> Self::Output {
        YZW(self.0 * rhs.0)
    }
}

impl AntiGeometric<ZXW> for XYZW {
    type Output = ZXW;
    fn anti_geometric(self, rhs: ZXW) -> Self::Output {
        ZXW(self.0 * rhs.0)
    }
}

impl AntiGeometric<XYW> for XYZW {
    type Output = XYW;
    fn anti_geometric(self, rhs: XYW) -> Self::Output {
        XYW(self.0 * rhs.0)
    }
}

impl AntiGeometric<ZYX> for XYZW {
    type Output = ZYX;
    fn anti_geometric(self, rhs: ZYX) -> Self::Output {
        ZYX(self.0 * rhs.0)
    }
}

impl AntiGeometric<XYZW> for XYZW {
    type Output = XYZW;
    fn anti_geometric(self, rhs: XYZW) -> Self::Output {
        XYZW(self.0 * rhs.0)
    }
}

// ---------------------------------------------------------------------
// impl Dot for blades:

impl Dot<S> for S {
    type Output = S;
    fn dot(self, rhs: S) -> Self::Output {
        S(self.0 * rhs.0)
    }
}

impl Dot<X> for S {
    type Output = X;
    fn dot(self, rhs: X) -> Self::Output {
        X(self.0 * rhs.0)
    }
}

impl Dot<Y> for S {
    type Output = Y;
    fn dot(self, rhs: Y) -> Self::Output {
        Y(self.0 * rhs.0)
    }
}

impl Dot<Z> for S {
    type Output = Z;
    fn dot(self, rhs: Z) -> Self::Output {
        Z(self.0 * rhs.0)
    }
}

impl Dot<W> for S {
    type Output = W;
    fn dot(self, rhs: W) -> Self::Output {
        W(self.0 * rhs.0)
    }
}

impl Dot<WX> for S {
    type Output = WX;
    fn dot(self, rhs: WX) -> Self::Output {
        WX(self.0 * rhs.0)
    }
}

impl Dot<WY> for S {
    type Output = WY;
    fn dot(self, rhs: WY) -> Self::Output {
        WY(self.0 * rhs.0)
    }
}

impl Dot<WZ> for S {
    type Output = WZ;
    fn dot(self, rhs: WZ) -> Self::Output {
        WZ(self.0 * rhs.0)
    }
}

impl Dot<YZ> for S {
    type Output = YZ;
    fn dot(self, rhs: YZ) -> Self::Output {
        YZ(self.0 * rhs.0)
    }
}

impl Dot<ZX> for S {
    type Output = ZX;
    fn dot(self, rhs: ZX) -> Self::Output {
        ZX(self.0 * rhs.0)
    }
}

impl Dot<XY> for S {
    type Output = XY;
    fn dot(self, rhs: XY) -> Self::Output {
        XY(self.0 * rhs.0)
    }
}

impl Dot<YZW> for S {
    type Output = YZW;
    fn dot(self, rhs: YZW) -> Self::Output {
        YZW(self.0 * rhs.0)
    }
}

impl Dot<ZXW> for S {
    type Output = ZXW;
    fn dot(self, rhs: ZXW) -> Self::Output {
        ZXW(self.0 * rhs.0)
    }
}

impl Dot<XYW> for S {
    type Output = XYW;
    fn dot(self, rhs: XYW) -> Self::Output {
        XYW(self.0 * rhs.0)
    }
}

impl Dot<ZYX> for S {
    type Output = ZYX;
    fn dot(self, rhs: ZYX) -> Self::Output {
        ZYX(self.0 * rhs.0)
    }
}

impl Dot<XYZW> for S {
    type Output = XYZW;
    fn dot(self, rhs: XYZW) -> Self::Output {
        XYZW(self.0 * rhs.0)
    }
}

impl Dot<S> for X {
    type Output = X;
    fn dot(self, rhs: S) -> Self::Output {
        X(self.0 * rhs.0)
    }
}

impl Dot<X> for X {
    type Output = S;
    fn dot(self, rhs: X) -> Self::Output {
        S(self.0 * rhs.0)
    }
}

impl Dot<Y> for X {
    type Output = Zero;
    fn dot(self, _rhs: Y) -> Self::Output {
        Zero {}
    }
}

impl Dot<Z> for X {
    type Output = Zero;
    fn dot(self, _rhs: Z) -> Self::Output {
        Zero {}
    }
}

impl Dot<W> for X {
    type Output = Zero;
    fn dot(self, _rhs: W) -> Self::Output {
        Zero {}
    }
}

impl Dot<WX> for X {
    type Output = W;
    fn dot(self, rhs: WX) -> Self::Output {
        W(-self.0 * rhs.0)
    }
}

impl Dot<WY> for X {
    type Output = Zero;
    fn dot(self, _rhs: WY) -> Self::Output {
        Zero {}
    }
}

impl Dot<WZ> for X {
    type Output = Zero;
    fn dot(self, _rhs: WZ) -> Self::Output {
        Zero {}
    }
}

impl Dot<YZ> for X {
    type Output = Zero;
    fn dot(self, _rhs: YZ) -> Self::Output {
        Zero {}
    }
}

impl Dot<ZX> for X {
    type Output = Z;
    fn dot(self, rhs: ZX) -> Self::Output {
        Z(-self.0 * rhs.0)
    }
}

impl Dot<XY> for X {
    type Output = Y;
    fn dot(self, rhs: XY) -> Self::Output {
        Y(self.0 * rhs.0)
    }
}

impl Dot<YZW> for X {
    type Output = Zero;
    fn dot(self, _rhs: YZW) -> Self::Output {
        Zero {}
    }
}

impl Dot<ZXW> for X {
    type Output = WZ;
    fn dot(self, rhs: ZXW) -> Self::Output {
        WZ(self.0 * rhs.0)
    }
}

impl Dot<XYW> for X {
    type Output = WY;
    fn dot(self, rhs: XYW) -> Self::Output {
        WY(-self.0 * rhs.0)
    }
}

impl Dot<ZYX> for X {
    type Output = YZ;
    fn dot(self, rhs: ZYX) -> Self::Output {
        YZ(-self.0 * rhs.0)
    }
}

impl Dot<XYZW> for X {
    type Output = YZW;
    fn dot(self, rhs: XYZW) -> Self::Output {
        YZW(self.0 * rhs.0)
    }
}

impl Dot<S> for Y {
    type Output = Y;
    fn dot(self, rhs: S) -> Self::Output {
        Y(self.0 * rhs.0)
    }
}

impl Dot<X> for Y {
    type Output = Zero;
    fn dot(self, _rhs: X) -> Self::Output {
        Zero {}
    }
}

impl Dot<Y> for Y {
    type Output = S;
    fn dot(self, rhs: Y) -> Self::Output {
        S(self.0 * rhs.0)
    }
}

impl Dot<Z> for Y {
    type Output = Zero;
    fn dot(self, _rhs: Z) -> Self::Output {
        Zero {}
    }
}

impl Dot<W> for Y {
    type Output = Zero;
    fn dot(self, _rhs: W) -> Self::Output {
        Zero {}
    }
}

impl Dot<WX> for Y {
    type Output = Zero;
    fn dot(self, _rhs: WX) -> Self::Output {
        Zero {}
    }
}

impl Dot<WY> for Y {
    type Output = W;
    fn dot(self, rhs: WY) -> Self::Output {
        W(-self.0 * rhs.0)
    }
}

impl Dot<WZ> for Y {
    type Output = Zero;
    fn dot(self, _rhs: WZ) -> Self::Output {
        Zero {}
    }
}

impl Dot<YZ> for Y {
    type Output = Z;
    fn dot(self, rhs: YZ) -> Self::Output {
        Z(self.0 * rhs.0)
    }
}

impl Dot<ZX> for Y {
    type Output = Zero;
    fn dot(self, _rhs: ZX) -> Self::Output {
        Zero {}
    }
}

impl Dot<XY> for Y {
    type Output = X;
    fn dot(self, rhs: XY) -> Self::Output {
        X(-self.0 * rhs.0)
    }
}

impl Dot<YZW> for Y {
    type Output = WZ;
    fn dot(self, rhs: YZW) -> Self::Output {
        WZ(-self.0 * rhs.0)
    }
}

impl Dot<ZXW> for Y {
    type Output = Zero;
    fn dot(self, _rhs: ZXW) -> Self::Output {
        Zero {}
    }
}

impl Dot<XYW> for Y {
    type Output = WX;
    fn dot(self, rhs: XYW) -> Self::Output {
        WX(self.0 * rhs.0)
    }
}

impl Dot<ZYX> for Y {
    type Output = ZX;
    fn dot(self, rhs: ZYX) -> Self::Output {
        ZX(-self.0 * rhs.0)
    }
}

impl Dot<XYZW> for Y {
    type Output = ZXW;
    fn dot(self, rhs: XYZW) -> Self::Output {
        ZXW(self.0 * rhs.0)
    }
}

impl Dot<S> for Z {
    type Output = Z;
    fn dot(self, rhs: S) -> Self::Output {
        Z(self.0 * rhs.0)
    }
}

impl Dot<X> for Z {
    type Output = Zero;
    fn dot(self, _rhs: X) -> Self::Output {
        Zero {}
    }
}

impl Dot<Y> for Z {
    type Output = Zero;
    fn dot(self, _rhs: Y) -> Self::Output {
        Zero {}
    }
}

impl Dot<Z> for Z {
    type Output = S;
    fn dot(self, rhs: Z) -> Self::Output {
        S(self.0 * rhs.0)
    }
}

impl Dot<W> for Z {
    type Output = Zero;
    fn dot(self, _rhs: W) -> Self::Output {
        Zero {}
    }
}

impl Dot<WX> for Z {
    type Output = Zero;
    fn dot(self, _rhs: WX) -> Self::Output {
        Zero {}
    }
}

impl Dot<WY> for Z {
    type Output = Zero;
    fn dot(self, _rhs: WY) -> Self::Output {
        Zero {}
    }
}

impl Dot<WZ> for Z {
    type Output = W;
    fn dot(self, rhs: WZ) -> Self::Output {
        W(-self.0 * rhs.0)
    }
}

impl Dot<YZ> for Z {
    type Output = Y;
    fn dot(self, rhs: YZ) -> Self::Output {
        Y(-self.0 * rhs.0)
    }
}

impl Dot<ZX> for Z {
    type Output = X;
    fn dot(self, rhs: ZX) -> Self::Output {
        X(self.0 * rhs.0)
    }
}

impl Dot<XY> for Z {
    type Output = Zero;
    fn dot(self, _rhs: XY) -> Self::Output {
        Zero {}
    }
}

impl Dot<YZW> for Z {
    type Output = WY;
    fn dot(self, rhs: YZW) -> Self::Output {
        WY(self.0 * rhs.0)
    }
}

impl Dot<ZXW> for Z {
    type Output = WX;
    fn dot(self, rhs: ZXW) -> Self::Output {
        WX(-self.0 * rhs.0)
    }
}

impl Dot<XYW> for Z {
    type Output = Zero;
    fn dot(self, _rhs: XYW) -> Self::Output {
        Zero {}
    }
}

impl Dot<ZYX> for Z {
    type Output = XY;
    fn dot(self, rhs: ZYX) -> Self::Output {
        XY(-self.0 * rhs.0)
    }
}

impl Dot<XYZW> for Z {
    type Output = XYW;
    fn dot(self, rhs: XYZW) -> Self::Output {
        XYW(self.0 * rhs.0)
    }
}

impl Dot<S> for W {
    type Output = W;
    fn dot(self, rhs: S) -> Self::Output {
        W(self.0 * rhs.0)
    }
}

impl Dot<X> for W {
    type Output = Zero;
    fn dot(self, _rhs: X) -> Self::Output {
        Zero {}
    }
}

impl Dot<Y> for W {
    type Output = Zero;
    fn dot(self, _rhs: Y) -> Self::Output {
        Zero {}
    }
}

impl Dot<Z> for W {
    type Output = Zero;
    fn dot(self, _rhs: Z) -> Self::Output {
        Zero {}
    }
}

impl Dot<W> for W {
    type Output = Zero;
    fn dot(self, _rhs: W) -> Self::Output {
        Zero {}
    }
}

impl Dot<WX> for W {
    type Output = Zero;
    fn dot(self, _rhs: WX) -> Self::Output {
        Zero {}
    }
}

impl Dot<WY> for W {
    type Output = Zero;
    fn dot(self, _rhs: WY) -> Self::Output {
        Zero {}
    }
}

impl Dot<WZ> for W {
    type Output = Zero;
    fn dot(self, _rhs: WZ) -> Self::Output {
        Zero {}
    }
}

impl Dot<YZ> for W {
    type Output = Zero;
    fn dot(self, _rhs: YZ) -> Self::Output {
        Zero {}
    }
}

impl Dot<ZX> for W {
    type Output = Zero;
    fn dot(self, _rhs: ZX) -> Self::Output {
        Zero {}
    }
}

impl Dot<XY> for W {
    type Output = Zero;
    fn dot(self, _rhs: XY) -> Self::Output {
        Zero {}
    }
}

impl Dot<YZW> for W {
    type Output = Zero;
    fn dot(self, _rhs: YZW) -> Self::Output {
        Zero {}
    }
}

impl Dot<ZXW> for W {
    type Output = Zero;
    fn dot(self, _rhs: ZXW) -> Self::Output {
        Zero {}
    }
}

impl Dot<XYW> for W {
    type Output = Zero;
    fn dot(self, _rhs: XYW) -> Self::Output {
        Zero {}
    }
}

impl Dot<ZYX> for W {
    type Output = Zero;
    fn dot(self, _rhs: ZYX) -> Self::Output {
        Zero {}
    }
}

impl Dot<XYZW> for W {
    type Output = Zero;
    fn dot(self, _rhs: XYZW) -> Self::Output {
        Zero {}
    }
}

impl Dot<S> for WX {
    type Output = WX;
    fn dot(self, rhs: S) -> Self::Output {
        WX(self.0 * rhs.0)
    }
}

impl Dot<X> for WX {
    type Output = W;
    fn dot(self, rhs: X) -> Self::Output {
        W(self.0 * rhs.0)
    }
}

impl Dot<Y> for WX {
    type Output = Zero;
    fn dot(self, _rhs: Y) -> Self::Output {
        Zero {}
    }
}

impl Dot<Z> for WX {
    type Output = Zero;
    fn dot(self, _rhs: Z) -> Self::Output {
        Zero {}
    }
}

impl Dot<W> for WX {
    type Output = Zero;
    fn dot(self, _rhs: W) -> Self::Output {
        Zero {}
    }
}

impl Dot<WX> for WX {
    type Output = Zero;
    fn dot(self, _rhs: WX) -> Self::Output {
        Zero {}
    }
}

impl Dot<WY> for WX {
    type Output = Zero;
    fn dot(self, _rhs: WY) -> Self::Output {
        Zero {}
    }
}

impl Dot<WZ> for WX {
    type Output = Zero;
    fn dot(self, _rhs: WZ) -> Self::Output {
        Zero {}
    }
}

impl Dot<YZ> for WX {
    type Output = Zero;
    fn dot(self, _rhs: YZ) -> Self::Output {
        Zero {}
    }
}

impl Dot<ZX> for WX {
    type Output = Zero;
    fn dot(self, _rhs: ZX) -> Self::Output {
        Zero {}
    }
}

impl Dot<XY> for WX {
    type Output = Zero;
    fn dot(self, _rhs: XY) -> Self::Output {
        Zero {}
    }
}

impl Dot<YZW> for WX {
    type Output = Zero;
    fn dot(self, _rhs: YZW) -> Self::Output {
        Zero {}
    }
}

impl Dot<ZXW> for WX {
    type Output = Zero;
    fn dot(self, _rhs: ZXW) -> Self::Output {
        Zero {}
    }
}

impl Dot<XYW> for WX {
    type Output = Zero;
    fn dot(self, _rhs: XYW) -> Self::Output {
        Zero {}
    }
}

impl Dot<ZYX> for WX {
    type Output = Zero;
    fn dot(self, _rhs: ZYX) -> Self::Output {
        Zero {}
    }
}

impl Dot<XYZW> for WX {
    type Output = Zero;
    fn dot(self, _rhs: XYZW) -> Self::Output {
        Zero {}
    }
}

impl Dot<S> for WY {
    type Output = WY;
    fn dot(self, rhs: S) -> Self::Output {
        WY(self.0 * rhs.0)
    }
}

impl Dot<X> for WY {
    type Output = Zero;
    fn dot(self, _rhs: X) -> Self::Output {
        Zero {}
    }
}

impl Dot<Y> for WY {
    type Output = W;
    fn dot(self, rhs: Y) -> Self::Output {
        W(self.0 * rhs.0)
    }
}

impl Dot<Z> for WY {
    type Output = Zero;
    fn dot(self, _rhs: Z) -> Self::Output {
        Zero {}
    }
}

impl Dot<W> for WY {
    type Output = Zero;
    fn dot(self, _rhs: W) -> Self::Output {
        Zero {}
    }
}

impl Dot<WX> for WY {
    type Output = Zero;
    fn dot(self, _rhs: WX) -> Self::Output {
        Zero {}
    }
}

impl Dot<WY> for WY {
    type Output = Zero;
    fn dot(self, _rhs: WY) -> Self::Output {
        Zero {}
    }
}

impl Dot<WZ> for WY {
    type Output = Zero;
    fn dot(self, _rhs: WZ) -> Self::Output {
        Zero {}
    }
}

impl Dot<YZ> for WY {
    type Output = Zero;
    fn dot(self, _rhs: YZ) -> Self::Output {
        Zero {}
    }
}

impl Dot<ZX> for WY {
    type Output = Zero;
    fn dot(self, _rhs: ZX) -> Self::Output {
        Zero {}
    }
}

impl Dot<XY> for WY {
    type Output = Zero;
    fn dot(self, _rhs: XY) -> Self::Output {
        Zero {}
    }
}

impl Dot<YZW> for WY {
    type Output = Zero;
    fn dot(self, _rhs: YZW) -> Self::Output {
        Zero {}
    }
}

impl Dot<ZXW> for WY {
    type Output = Zero;
    fn dot(self, _rhs: ZXW) -> Self::Output {
        Zero {}
    }
}

impl Dot<XYW> for WY {
    type Output = Zero;
    fn dot(self, _rhs: XYW) -> Self::Output {
        Zero {}
    }
}

impl Dot<ZYX> for WY {
    type Output = Zero;
    fn dot(self, _rhs: ZYX) -> Self::Output {
        Zero {}
    }
}

impl Dot<XYZW> for WY {
    type Output = Zero;
    fn dot(self, _rhs: XYZW) -> Self::Output {
        Zero {}
    }
}

impl Dot<S> for WZ {
    type Output = WZ;
    fn dot(self, rhs: S) -> Self::Output {
        WZ(self.0 * rhs.0)
    }
}

impl Dot<X> for WZ {
    type Output = Zero;
    fn dot(self, _rhs: X) -> Self::Output {
        Zero {}
    }
}

impl Dot<Y> for WZ {
    type Output = Zero;
    fn dot(self, _rhs: Y) -> Self::Output {
        Zero {}
    }
}

impl Dot<Z> for WZ {
    type Output = W;
    fn dot(self, rhs: Z) -> Self::Output {
        W(self.0 * rhs.0)
    }
}

impl Dot<W> for WZ {
    type Output = Zero;
    fn dot(self, _rhs: W) -> Self::Output {
        Zero {}
    }
}

impl Dot<WX> for WZ {
    type Output = Zero;
    fn dot(self, _rhs: WX) -> Self::Output {
        Zero {}
    }
}

impl Dot<WY> for WZ {
    type Output = Zero;
    fn dot(self, _rhs: WY) -> Self::Output {
        Zero {}
    }
}

impl Dot<WZ> for WZ {
    type Output = Zero;
    fn dot(self, _rhs: WZ) -> Self::Output {
        Zero {}
    }
}

impl Dot<YZ> for WZ {
    type Output = Zero;
    fn dot(self, _rhs: YZ) -> Self::Output {
        Zero {}
    }
}

impl Dot<ZX> for WZ {
    type Output = Zero;
    fn dot(self, _rhs: ZX) -> Self::Output {
        Zero {}
    }
}

impl Dot<XY> for WZ {
    type Output = Zero;
    fn dot(self, _rhs: XY) -> Self::Output {
        Zero {}
    }
}

impl Dot<YZW> for WZ {
    type Output = Zero;
    fn dot(self, _rhs: YZW) -> Self::Output {
        Zero {}
    }
}

impl Dot<ZXW> for WZ {
    type Output = Zero;
    fn dot(self, _rhs: ZXW) -> Self::Output {
        Zero {}
    }
}

impl Dot<XYW> for WZ {
    type Output = Zero;
    fn dot(self, _rhs: XYW) -> Self::Output {
        Zero {}
    }
}

impl Dot<ZYX> for WZ {
    type Output = Zero;
    fn dot(self, _rhs: ZYX) -> Self::Output {
        Zero {}
    }
}

impl Dot<XYZW> for WZ {
    type Output = Zero;
    fn dot(self, _rhs: XYZW) -> Self::Output {
        Zero {}
    }
}

impl Dot<S> for YZ {
    type Output = YZ;
    fn dot(self, rhs: S) -> Self::Output {
        YZ(self.0 * rhs.0)
    }
}

impl Dot<X> for YZ {
    type Output = Zero;
    fn dot(self, _rhs: X) -> Self::Output {
        Zero {}
    }
}

impl Dot<Y> for YZ {
    type Output = Z;
    fn dot(self, rhs: Y) -> Self::Output {
        Z(-self.0 * rhs.0)
    }
}

impl Dot<Z> for YZ {
    type Output = Y;
    fn dot(self, rhs: Z) -> Self::Output {
        Y(self.0 * rhs.0)
    }
}

impl Dot<W> for YZ {
    type Output = Zero;
    fn dot(self, _rhs: W) -> Self::Output {
        Zero {}
    }
}

impl Dot<WX> for YZ {
    type Output = Zero;
    fn dot(self, _rhs: WX) -> Self::Output {
        Zero {}
    }
}

impl Dot<WY> for YZ {
    type Output = Zero;
    fn dot(self, _rhs: WY) -> Self::Output {
        Zero {}
    }
}

impl Dot<WZ> for YZ {
    type Output = Zero;
    fn dot(self, _rhs: WZ) -> Self::Output {
        Zero {}
    }
}

impl Dot<YZ> for YZ {
    type Output = S;
    fn dot(self, rhs: YZ) -> Self::Output {
        S(-self.0 * rhs.0)
    }
}

impl Dot<ZX> for YZ {
    type Output = Zero;
    fn dot(self, _rhs: ZX) -> Self::Output {
        Zero {}
    }
}

impl Dot<XY> for YZ {
    type Output = Zero;
    fn dot(self, _rhs: XY) -> Self::Output {
        Zero {}
    }
}

impl Dot<YZW> for YZ {
    type Output = W;
    fn dot(self, rhs: YZW) -> Self::Output {
        W(-self.0 * rhs.0)
    }
}

impl Dot<ZXW> for YZ {
    type Output = Zero;
    fn dot(self, _rhs: ZXW) -> Self::Output {
        Zero {}
    }
}

impl Dot<XYW> for YZ {
    type Output = Zero;
    fn dot(self, _rhs: XYW) -> Self::Output {
        Zero {}
    }
}

impl Dot<ZYX> for YZ {
    type Output = X;
    fn dot(self, rhs: ZYX) -> Self::Output {
        X(self.0 * rhs.0)
    }
}

impl Dot<XYZW> for YZ {
    type Output = WX;
    fn dot(self, rhs: XYZW) -> Self::Output {
        WX(self.0 * rhs.0)
    }
}

impl Dot<S> for ZX {
    type Output = ZX;
    fn dot(self, rhs: S) -> Self::Output {
        ZX(self.0 * rhs.0)
    }
}

impl Dot<X> for ZX {
    type Output = Z;
    fn dot(self, rhs: X) -> Self::Output {
        Z(self.0 * rhs.0)
    }
}

impl Dot<Y> for ZX {
    type Output = Zero;
    fn dot(self, _rhs: Y) -> Self::Output {
        Zero {}
    }
}

impl Dot<Z> for ZX {
    type Output = X;
    fn dot(self, rhs: Z) -> Self::Output {
        X(-self.0 * rhs.0)
    }
}

impl Dot<W> for ZX {
    type Output = Zero;
    fn dot(self, _rhs: W) -> Self::Output {
        Zero {}
    }
}

impl Dot<WX> for ZX {
    type Output = Zero;
    fn dot(self, _rhs: WX) -> Self::Output {
        Zero {}
    }
}

impl Dot<WY> for ZX {
    type Output = Zero;
    fn dot(self, _rhs: WY) -> Self::Output {
        Zero {}
    }
}

impl Dot<WZ> for ZX {
    type Output = Zero;
    fn dot(self, _rhs: WZ) -> Self::Output {
        Zero {}
    }
}

impl Dot<YZ> for ZX {
    type Output = Zero;
    fn dot(self, _rhs: YZ) -> Self::Output {
        Zero {}
    }
}

impl Dot<ZX> for ZX {
    type Output = S;
    fn dot(self, rhs: ZX) -> Self::Output {
        S(-self.0 * rhs.0)
    }
}

impl Dot<XY> for ZX {
    type Output = Zero;
    fn dot(self, _rhs: XY) -> Self::Output {
        Zero {}
    }
}

impl Dot<YZW> for ZX {
    type Output = Zero;
    fn dot(self, _rhs: YZW) -> Self::Output {
        Zero {}
    }
}

impl Dot<ZXW> for ZX {
    type Output = W;
    fn dot(self, rhs: ZXW) -> Self::Output {
        W(-self.0 * rhs.0)
    }
}

impl Dot<XYW> for ZX {
    type Output = Zero;
    fn dot(self, _rhs: XYW) -> Self::Output {
        Zero {}
    }
}

impl Dot<ZYX> for ZX {
    type Output = Y;
    fn dot(self, rhs: ZYX) -> Self::Output {
        Y(self.0 * rhs.0)
    }
}

impl Dot<XYZW> for ZX {
    type Output = WY;
    fn dot(self, rhs: XYZW) -> Self::Output {
        WY(self.0 * rhs.0)
    }
}

impl Dot<S> for XY {
    type Output = XY;
    fn dot(self, rhs: S) -> Self::Output {
        XY(self.0 * rhs.0)
    }
}

impl Dot<X> for XY {
    type Output = Y;
    fn dot(self, rhs: X) -> Self::Output {
        Y(-self.0 * rhs.0)
    }
}

impl Dot<Y> for XY {
    type Output = X;
    fn dot(self, rhs: Y) -> Self::Output {
        X(self.0 * rhs.0)
    }
}

impl Dot<Z> for XY {
    type Output = Zero;
    fn dot(self, _rhs: Z) -> Self::Output {
        Zero {}
    }
}

impl Dot<W> for XY {
    type Output = Zero;
    fn dot(self, _rhs: W) -> Self::Output {
        Zero {}
    }
}

impl Dot<WX> for XY {
    type Output = Zero;
    fn dot(self, _rhs: WX) -> Self::Output {
        Zero {}
    }
}

impl Dot<WY> for XY {
    type Output = Zero;
    fn dot(self, _rhs: WY) -> Self::Output {
        Zero {}
    }
}

impl Dot<WZ> for XY {
    type Output = Zero;
    fn dot(self, _rhs: WZ) -> Self::Output {
        Zero {}
    }
}

impl Dot<YZ> for XY {
    type Output = Zero;
    fn dot(self, _rhs: YZ) -> Self::Output {
        Zero {}
    }
}

impl Dot<ZX> for XY {
    type Output = Zero;
    fn dot(self, _rhs: ZX) -> Self::Output {
        Zero {}
    }
}

impl Dot<XY> for XY {
    type Output = S;
    fn dot(self, rhs: XY) -> Self::Output {
        S(-self.0 * rhs.0)
    }
}

impl Dot<YZW> for XY {
    type Output = Zero;
    fn dot(self, _rhs: YZW) -> Self::Output {
        Zero {}
    }
}

impl Dot<ZXW> for XY {
    type Output = Zero;
    fn dot(self, _rhs: ZXW) -> Self::Output {
        Zero {}
    }
}

impl Dot<XYW> for XY {
    type Output = W;
    fn dot(self, rhs: XYW) -> Self::Output {
        W(-self.0 * rhs.0)
    }
}

impl Dot<ZYX> for XY {
    type Output = Z;
    fn dot(self, rhs: ZYX) -> Self::Output {
        Z(self.0 * rhs.0)
    }
}

impl Dot<XYZW> for XY {
    type Output = WZ;
    fn dot(self, rhs: XYZW) -> Self::Output {
        WZ(self.0 * rhs.0)
    }
}

impl Dot<S> for YZW {
    type Output = YZW;
    fn dot(self, rhs: S) -> Self::Output {
        YZW(self.0 * rhs.0)
    }
}

impl Dot<X> for YZW {
    type Output = Zero;
    fn dot(self, _rhs: X) -> Self::Output {
        Zero {}
    }
}

impl Dot<Y> for YZW {
    type Output = WZ;
    fn dot(self, rhs: Y) -> Self::Output {
        WZ(-self.0 * rhs.0)
    }
}

impl Dot<Z> for YZW {
    type Output = WY;
    fn dot(self, rhs: Z) -> Self::Output {
        WY(self.0 * rhs.0)
    }
}

impl Dot<W> for YZW {
    type Output = Zero;
    fn dot(self, _rhs: W) -> Self::Output {
        Zero {}
    }
}

impl Dot<WX> for YZW {
    type Output = Zero;
    fn dot(self, _rhs: WX) -> Self::Output {
        Zero {}
    }
}

impl Dot<WY> for YZW {
    type Output = Zero;
    fn dot(self, _rhs: WY) -> Self::Output {
        Zero {}
    }
}

impl Dot<WZ> for YZW {
    type Output = Zero;
    fn dot(self, _rhs: WZ) -> Self::Output {
        Zero {}
    }
}

impl Dot<YZ> for YZW {
    type Output = W;
    fn dot(self, rhs: YZ) -> Self::Output {
        W(-self.0 * rhs.0)
    }
}

impl Dot<ZX> for YZW {
    type Output = Zero;
    fn dot(self, _rhs: ZX) -> Self::Output {
        Zero {}
    }
}

impl Dot<XY> for YZW {
    type Output = Zero;
    fn dot(self, _rhs: XY) -> Self::Output {
        Zero {}
    }
}

impl Dot<YZW> for YZW {
    type Output = Zero;
    fn dot(self, _rhs: YZW) -> Self::Output {
        Zero {}
    }
}

impl Dot<ZXW> for YZW {
    type Output = Zero;
    fn dot(self, _rhs: ZXW) -> Self::Output {
        Zero {}
    }
}

impl Dot<XYW> for YZW {
    type Output = Zero;
    fn dot(self, _rhs: XYW) -> Self::Output {
        Zero {}
    }
}

impl Dot<ZYX> for YZW {
    type Output = Zero;
    fn dot(self, _rhs: ZYX) -> Self::Output {
        Zero {}
    }
}

impl Dot<XYZW> for YZW {
    type Output = Zero;
    fn dot(self, _rhs: XYZW) -> Self::Output {
        Zero {}
    }
}

impl Dot<S> for ZXW {
    type Output = ZXW;
    fn dot(self, rhs: S) -> Self::Output {
        ZXW(self.0 * rhs.0)
    }
}

impl Dot<X> for ZXW {
    type Output = WZ;
    fn dot(self, rhs: X) -> Self::Output {
        WZ(self.0 * rhs.0)
    }
}

impl Dot<Y> for ZXW {
    type Output = Zero;
    fn dot(self, _rhs: Y) -> Self::Output {
        Zero {}
    }
}

impl Dot<Z> for ZXW {
    type Output = WX;
    fn dot(self, rhs: Z) -> Self::Output {
        WX(-self.0 * rhs.0)
    }
}

impl Dot<W> for ZXW {
    type Output = Zero;
    fn dot(self, _rhs: W) -> Self::Output {
        Zero {}
    }
}

impl Dot<WX> for ZXW {
    type Output = Zero;
    fn dot(self, _rhs: WX) -> Self::Output {
        Zero {}
    }
}

impl Dot<WY> for ZXW {
    type Output = Zero;
    fn dot(self, _rhs: WY) -> Self::Output {
        Zero {}
    }
}

impl Dot<WZ> for ZXW {
    type Output = Zero;
    fn dot(self, _rhs: WZ) -> Self::Output {
        Zero {}
    }
}

impl Dot<YZ> for ZXW {
    type Output = Zero;
    fn dot(self, _rhs: YZ) -> Self::Output {
        Zero {}
    }
}

impl Dot<ZX> for ZXW {
    type Output = W;
    fn dot(self, rhs: ZX) -> Self::Output {
        W(-self.0 * rhs.0)
    }
}

impl Dot<XY> for ZXW {
    type Output = Zero;
    fn dot(self, _rhs: XY) -> Self::Output {
        Zero {}
    }
}

impl Dot<YZW> for ZXW {
    type Output = Zero;
    fn dot(self, _rhs: YZW) -> Self::Output {
        Zero {}
    }
}

impl Dot<ZXW> for ZXW {
    type Output = Zero;
    fn dot(self, _rhs: ZXW) -> Self::Output {
        Zero {}
    }
}

impl Dot<XYW> for ZXW {
    type Output = Zero;
    fn dot(self, _rhs: XYW) -> Self::Output {
        Zero {}
    }
}

impl Dot<ZYX> for ZXW {
    type Output = Zero;
    fn dot(self, _rhs: ZYX) -> Self::Output {
        Zero {}
    }
}

impl Dot<XYZW> for ZXW {
    type Output = Zero;
    fn dot(self, _rhs: XYZW) -> Self::Output {
        Zero {}
    }
}

impl Dot<S> for XYW {
    type Output = XYW;
    fn dot(self, rhs: S) -> Self::Output {
        XYW(self.0 * rhs.0)
    }
}

impl Dot<X> for XYW {
    type Output = WY;
    fn dot(self, rhs: X) -> Self::Output {
        WY(-self.0 * rhs.0)
    }
}

impl Dot<Y> for XYW {
    type Output = WX;
    fn dot(self, rhs: Y) -> Self::Output {
        WX(self.0 * rhs.0)
    }
}

impl Dot<Z> for XYW {
    type Output = Zero;
    fn dot(self, _rhs: Z) -> Self::Output {
        Zero {}
    }
}

impl Dot<W> for XYW {
    type Output = Zero;
    fn dot(self, _rhs: W) -> Self::Output {
        Zero {}
    }
}

impl Dot<WX> for XYW {
    type Output = Zero;
    fn dot(self, _rhs: WX) -> Self::Output {
        Zero {}
    }
}

impl Dot<WY> for XYW {
    type Output = Zero;
    fn dot(self, _rhs: WY) -> Self::Output {
        Zero {}
    }
}

impl Dot<WZ> for XYW {
    type Output = Zero;
    fn dot(self, _rhs: WZ) -> Self::Output {
        Zero {}
    }
}

impl Dot<YZ> for XYW {
    type Output = Zero;
    fn dot(self, _rhs: YZ) -> Self::Output {
        Zero {}
    }
}

impl Dot<ZX> for XYW {
    type Output = Zero;
    fn dot(self, _rhs: ZX) -> Self::Output {
        Zero {}
    }
}

impl Dot<XY> for XYW {
    type Output = W;
    fn dot(self, rhs: XY) -> Self::Output {
        W(-self.0 * rhs.0)
    }
}

impl Dot<YZW> for XYW {
    type Output = Zero;
    fn dot(self, _rhs: YZW) -> Self::Output {
        Zero {}
    }
}

impl Dot<ZXW> for XYW {
    type Output = Zero;
    fn dot(self, _rhs: ZXW) -> Self::Output {
        Zero {}
    }
}

impl Dot<XYW> for XYW {
    type Output = Zero;
    fn dot(self, _rhs: XYW) -> Self::Output {
        Zero {}
    }
}

impl Dot<ZYX> for XYW {
    type Output = Zero;
    fn dot(self, _rhs: ZYX) -> Self::Output {
        Zero {}
    }
}

impl Dot<XYZW> for XYW {
    type Output = Zero;
    fn dot(self, _rhs: XYZW) -> Self::Output {
        Zero {}
    }
}

impl Dot<S> for ZYX {
    type Output = ZYX;
    fn dot(self, rhs: S) -> Self::Output {
        ZYX(self.0 * rhs.0)
    }
}

impl Dot<X> for ZYX {
    type Output = YZ;
    fn dot(self, rhs: X) -> Self::Output {
        YZ(-self.0 * rhs.0)
    }
}

impl Dot<Y> for ZYX {
    type Output = ZX;
    fn dot(self, rhs: Y) -> Self::Output {
        ZX(-self.0 * rhs.0)
    }
}

impl Dot<Z> for ZYX {
    type Output = XY;
    fn dot(self, rhs: Z) -> Self::Output {
        XY(-self.0 * rhs.0)
    }
}

impl Dot<W> for ZYX {
    type Output = Zero;
    fn dot(self, _rhs: W) -> Self::Output {
        Zero {}
    }
}

impl Dot<WX> for ZYX {
    type Output = Zero;
    fn dot(self, _rhs: WX) -> Self::Output {
        Zero {}
    }
}

impl Dot<WY> for ZYX {
    type Output = Zero;
    fn dot(self, _rhs: WY) -> Self::Output {
        Zero {}
    }
}

impl Dot<WZ> for ZYX {
    type Output = Zero;
    fn dot(self, _rhs: WZ) -> Self::Output {
        Zero {}
    }
}

impl Dot<YZ> for ZYX {
    type Output = X;
    fn dot(self, rhs: YZ) -> Self::Output {
        X(self.0 * rhs.0)
    }
}

impl Dot<ZX> for ZYX {
    type Output = Y;
    fn dot(self, rhs: ZX) -> Self::Output {
        Y(self.0 * rhs.0)
    }
}

impl Dot<XY> for ZYX {
    type Output = Z;
    fn dot(self, rhs: XY) -> Self::Output {
        Z(self.0 * rhs.0)
    }
}

impl Dot<YZW> for ZYX {
    type Output = Zero;
    fn dot(self, _rhs: YZW) -> Self::Output {
        Zero {}
    }
}

impl Dot<ZXW> for ZYX {
    type Output = Zero;
    fn dot(self, _rhs: ZXW) -> Self::Output {
        Zero {}
    }
}

impl Dot<XYW> for ZYX {
    type Output = Zero;
    fn dot(self, _rhs: XYW) -> Self::Output {
        Zero {}
    }
}

impl Dot<ZYX> for ZYX {
    type Output = S;
    fn dot(self, rhs: ZYX) -> Self::Output {
        S(-self.0 * rhs.0)
    }
}

impl Dot<XYZW> for ZYX {
    type Output = W;
    fn dot(self, rhs: XYZW) -> Self::Output {
        W(self.0 * rhs.0)
    }
}

impl Dot<S> for XYZW {
    type Output = XYZW;
    fn dot(self, rhs: S) -> Self::Output {
        XYZW(self.0 * rhs.0)
    }
}

impl Dot<X> for XYZW {
    type Output = YZW;
    fn dot(self, rhs: X) -> Self::Output {
        YZW(-self.0 * rhs.0)
    }
}

impl Dot<Y> for XYZW {
    type Output = ZXW;
    fn dot(self, rhs: Y) -> Self::Output {
        ZXW(-self.0 * rhs.0)
    }
}

impl Dot<Z> for XYZW {
    type Output = XYW;
    fn dot(self, rhs: Z) -> Self::Output {
        XYW(-self.0 * rhs.0)
    }
}

impl Dot<W> for XYZW {
    type Output = Zero;
    fn dot(self, _rhs: W) -> Self::Output {
        Zero {}
    }
}

impl Dot<WX> for XYZW {
    type Output = Zero;
    fn dot(self, _rhs: WX) -> Self::Output {
        Zero {}
    }
}

impl Dot<WY> for XYZW {
    type Output = Zero;
    fn dot(self, _rhs: WY) -> Self::Output {
        Zero {}
    }
}

impl Dot<WZ> for XYZW {
    type Output = Zero;
    fn dot(self, _rhs: WZ) -> Self::Output {
        Zero {}
    }
}

impl Dot<YZ> for XYZW {
    type Output = WX;
    fn dot(self, rhs: YZ) -> Self::Output {
        WX(self.0 * rhs.0)
    }
}

impl Dot<ZX> for XYZW {
    type Output = WY;
    fn dot(self, rhs: ZX) -> Self::Output {
        WY(self.0 * rhs.0)
    }
}

impl Dot<XY> for XYZW {
    type Output = WZ;
    fn dot(self, rhs: XY) -> Self::Output {
        WZ(self.0 * rhs.0)
    }
}

impl Dot<YZW> for XYZW {
    type Output = Zero;
    fn dot(self, _rhs: YZW) -> Self::Output {
        Zero {}
    }
}

impl Dot<ZXW> for XYZW {
    type Output = Zero;
    fn dot(self, _rhs: ZXW) -> Self::Output {
        Zero {}
    }
}

impl Dot<XYW> for XYZW {
    type Output = Zero;
    fn dot(self, _rhs: XYW) -> Self::Output {
        Zero {}
    }
}

impl Dot<ZYX> for XYZW {
    type Output = W;
    fn dot(self, rhs: ZYX) -> Self::Output {
        W(-self.0 * rhs.0)
    }
}

impl Dot<XYZW> for XYZW {
    type Output = Zero;
    fn dot(self, _rhs: XYZW) -> Self::Output {
        Zero {}
    }
}

// ---------------------------------------------------------------------
// impl Wedge for blades:

impl Wedge<S> for S {
    type Output = S;
    fn wedge(self, rhs: S) -> Self::Output {
        S(self.0 * rhs.0)
    }
}

impl Wedge<X> for S {
    type Output = X;
    fn wedge(self, rhs: X) -> Self::Output {
        X(self.0 * rhs.0)
    }
}

impl Wedge<Y> for S {
    type Output = Y;
    fn wedge(self, rhs: Y) -> Self::Output {
        Y(self.0 * rhs.0)
    }
}

impl Wedge<Z> for S {
    type Output = Z;
    fn wedge(self, rhs: Z) -> Self::Output {
        Z(self.0 * rhs.0)
    }
}

impl Wedge<W> for S {
    type Output = W;
    fn wedge(self, rhs: W) -> Self::Output {
        W(self.0 * rhs.0)
    }
}

impl Wedge<WX> for S {
    type Output = WX;
    fn wedge(self, rhs: WX) -> Self::Output {
        WX(self.0 * rhs.0)
    }
}

impl Wedge<WY> for S {
    type Output = WY;
    fn wedge(self, rhs: WY) -> Self::Output {
        WY(self.0 * rhs.0)
    }
}

impl Wedge<WZ> for S {
    type Output = WZ;
    fn wedge(self, rhs: WZ) -> Self::Output {
        WZ(self.0 * rhs.0)
    }
}

impl Wedge<YZ> for S {
    type Output = YZ;
    fn wedge(self, rhs: YZ) -> Self::Output {
        YZ(self.0 * rhs.0)
    }
}

impl Wedge<ZX> for S {
    type Output = ZX;
    fn wedge(self, rhs: ZX) -> Self::Output {
        ZX(self.0 * rhs.0)
    }
}

impl Wedge<XY> for S {
    type Output = XY;
    fn wedge(self, rhs: XY) -> Self::Output {
        XY(self.0 * rhs.0)
    }
}

impl Wedge<YZW> for S {
    type Output = YZW;
    fn wedge(self, rhs: YZW) -> Self::Output {
        YZW(self.0 * rhs.0)
    }
}

impl Wedge<ZXW> for S {
    type Output = ZXW;
    fn wedge(self, rhs: ZXW) -> Self::Output {
        ZXW(self.0 * rhs.0)
    }
}

impl Wedge<XYW> for S {
    type Output = XYW;
    fn wedge(self, rhs: XYW) -> Self::Output {
        XYW(self.0 * rhs.0)
    }
}

impl Wedge<ZYX> for S {
    type Output = ZYX;
    fn wedge(self, rhs: ZYX) -> Self::Output {
        ZYX(self.0 * rhs.0)
    }
}

impl Wedge<XYZW> for S {
    type Output = XYZW;
    fn wedge(self, rhs: XYZW) -> Self::Output {
        XYZW(self.0 * rhs.0)
    }
}

impl Wedge<S> for X {
    type Output = X;
    fn wedge(self, rhs: S) -> Self::Output {
        X(self.0 * rhs.0)
    }
}

impl Wedge<X> for X {
    type Output = Zero;
    fn wedge(self, _rhs: X) -> Self::Output {
        Zero {}
    }
}

impl Wedge<Y> for X {
    type Output = XY;
    fn wedge(self, rhs: Y) -> Self::Output {
        XY(self.0 * rhs.0)
    }
}

impl Wedge<Z> for X {
    type Output = ZX;
    fn wedge(self, rhs: Z) -> Self::Output {
        ZX(-self.0 * rhs.0)
    }
}

impl Wedge<W> for X {
    type Output = WX;
    fn wedge(self, rhs: W) -> Self::Output {
        WX(-self.0 * rhs.0)
    }
}

impl Wedge<WX> for X {
    type Output = Zero;
    fn wedge(self, _rhs: WX) -> Self::Output {
        Zero {}
    }
}

impl Wedge<WY> for X {
    type Output = XYW;
    fn wedge(self, rhs: WY) -> Self::Output {
        XYW(-self.0 * rhs.0)
    }
}

impl Wedge<WZ> for X {
    type Output = ZXW;
    fn wedge(self, rhs: WZ) -> Self::Output {
        ZXW(self.0 * rhs.0)
    }
}

impl Wedge<YZ> for X {
    type Output = ZYX;
    fn wedge(self, rhs: YZ) -> Self::Output {
        ZYX(-self.0 * rhs.0)
    }
}

impl Wedge<ZX> for X {
    type Output = Zero;
    fn wedge(self, _rhs: ZX) -> Self::Output {
        Zero {}
    }
}

impl Wedge<XY> for X {
    type Output = Zero;
    fn wedge(self, _rhs: XY) -> Self::Output {
        Zero {}
    }
}

impl Wedge<YZW> for X {
    type Output = XYZW;
    fn wedge(self, rhs: YZW) -> Self::Output {
        XYZW(self.0 * rhs.0)
    }
}

impl Wedge<ZXW> for X {
    type Output = Zero;
    fn wedge(self, _rhs: ZXW) -> Self::Output {
        Zero {}
    }
}

impl Wedge<XYW> for X {
    type Output = Zero;
    fn wedge(self, _rhs: XYW) -> Self::Output {
        Zero {}
    }
}

impl Wedge<ZYX> for X {
    type Output = Zero;
    fn wedge(self, _rhs: ZYX) -> Self::Output {
        Zero {}
    }
}

impl Wedge<XYZW> for X {
    type Output = Zero;
    fn wedge(self, _rhs: XYZW) -> Self::Output {
        Zero {}
    }
}

impl Wedge<S> for Y {
    type Output = Y;
    fn wedge(self, rhs: S) -> Self::Output {
        Y(self.0 * rhs.0)
    }
}

impl Wedge<X> for Y {
    type Output = XY;
    fn wedge(self, rhs: X) -> Self::Output {
        XY(-self.0 * rhs.0)
    }
}

impl Wedge<Y> for Y {
    type Output = Zero;
    fn wedge(self, _rhs: Y) -> Self::Output {
        Zero {}
    }
}

impl Wedge<Z> for Y {
    type Output = YZ;
    fn wedge(self, rhs: Z) -> Self::Output {
        YZ(self.0 * rhs.0)
    }
}

impl Wedge<W> for Y {
    type Output = WY;
    fn wedge(self, rhs: W) -> Self::Output {
        WY(-self.0 * rhs.0)
    }
}

impl Wedge<WX> for Y {
    type Output = XYW;
    fn wedge(self, rhs: WX) -> Self::Output {
        XYW(self.0 * rhs.0)
    }
}

impl Wedge<WY> for Y {
    type Output = Zero;
    fn wedge(self, _rhs: WY) -> Self::Output {
        Zero {}
    }
}

impl Wedge<WZ> for Y {
    type Output = YZW;
    fn wedge(self, rhs: WZ) -> Self::Output {
        YZW(-self.0 * rhs.0)
    }
}

impl Wedge<YZ> for Y {
    type Output = Zero;
    fn wedge(self, _rhs: YZ) -> Self::Output {
        Zero {}
    }
}

impl Wedge<ZX> for Y {
    type Output = ZYX;
    fn wedge(self, rhs: ZX) -> Self::Output {
        ZYX(-self.0 * rhs.0)
    }
}

impl Wedge<XY> for Y {
    type Output = Zero;
    fn wedge(self, _rhs: XY) -> Self::Output {
        Zero {}
    }
}

impl Wedge<YZW> for Y {
    type Output = Zero;
    fn wedge(self, _rhs: YZW) -> Self::Output {
        Zero {}
    }
}

impl Wedge<ZXW> for Y {
    type Output = XYZW;
    fn wedge(self, rhs: ZXW) -> Self::Output {
        XYZW(self.0 * rhs.0)
    }
}

impl Wedge<XYW> for Y {
    type Output = Zero;
    fn wedge(self, _rhs: XYW) -> Self::Output {
        Zero {}
    }
}

impl Wedge<ZYX> for Y {
    type Output = Zero;
    fn wedge(self, _rhs: ZYX) -> Self::Output {
        Zero {}
    }
}

impl Wedge<XYZW> for Y {
    type Output = Zero;
    fn wedge(self, _rhs: XYZW) -> Self::Output {
        Zero {}
    }
}

impl Wedge<S> for Z {
    type Output = Z;
    fn wedge(self, rhs: S) -> Self::Output {
        Z(self.0 * rhs.0)
    }
}

impl Wedge<X> for Z {
    type Output = ZX;
    fn wedge(self, rhs: X) -> Self::Output {
        ZX(self.0 * rhs.0)
    }
}

impl Wedge<Y> for Z {
    type Output = YZ;
    fn wedge(self, rhs: Y) -> Self::Output {
        YZ(-self.0 * rhs.0)
    }
}

impl Wedge<Z> for Z {
    type Output = Zero;
    fn wedge(self, _rhs: Z) -> Self::Output {
        Zero {}
    }
}

impl Wedge<W> for Z {
    type Output = WZ;
    fn wedge(self, rhs: W) -> Self::Output {
        WZ(-self.0 * rhs.0)
    }
}

impl Wedge<WX> for Z {
    type Output = ZXW;
    fn wedge(self, rhs: WX) -> Self::Output {
        ZXW(-self.0 * rhs.0)
    }
}

impl Wedge<WY> for Z {
    type Output = YZW;
    fn wedge(self, rhs: WY) -> Self::Output {
        YZW(self.0 * rhs.0)
    }
}

impl Wedge<WZ> for Z {
    type Output = Zero;
    fn wedge(self, _rhs: WZ) -> Self::Output {
        Zero {}
    }
}

impl Wedge<YZ> for Z {
    type Output = Zero;
    fn wedge(self, _rhs: YZ) -> Self::Output {
        Zero {}
    }
}

impl Wedge<ZX> for Z {
    type Output = Zero;
    fn wedge(self, _rhs: ZX) -> Self::Output {
        Zero {}
    }
}

impl Wedge<XY> for Z {
    type Output = ZYX;
    fn wedge(self, rhs: XY) -> Self::Output {
        ZYX(-self.0 * rhs.0)
    }
}

impl Wedge<YZW> for Z {
    type Output = Zero;
    fn wedge(self, _rhs: YZW) -> Self::Output {
        Zero {}
    }
}

impl Wedge<ZXW> for Z {
    type Output = Zero;
    fn wedge(self, _rhs: ZXW) -> Self::Output {
        Zero {}
    }
}

impl Wedge<XYW> for Z {
    type Output = XYZW;
    fn wedge(self, rhs: XYW) -> Self::Output {
        XYZW(self.0 * rhs.0)
    }
}

impl Wedge<ZYX> for Z {
    type Output = Zero;
    fn wedge(self, _rhs: ZYX) -> Self::Output {
        Zero {}
    }
}

impl Wedge<XYZW> for Z {
    type Output = Zero;
    fn wedge(self, _rhs: XYZW) -> Self::Output {
        Zero {}
    }
}

impl Wedge<S> for W {
    type Output = W;
    fn wedge(self, rhs: S) -> Self::Output {
        W(self.0 * rhs.0)
    }
}

impl Wedge<X> for W {
    type Output = WX;
    fn wedge(self, rhs: X) -> Self::Output {
        WX(self.0 * rhs.0)
    }
}

impl Wedge<Y> for W {
    type Output = WY;
    fn wedge(self, rhs: Y) -> Self::Output {
        WY(self.0 * rhs.0)
    }
}

impl Wedge<Z> for W {
    type Output = WZ;
    fn wedge(self, rhs: Z) -> Self::Output {
        WZ(self.0 * rhs.0)
    }
}

impl Wedge<W> for W {
    type Output = Zero;
    fn wedge(self, _rhs: W) -> Self::Output {
        Zero {}
    }
}

impl Wedge<WX> for W {
    type Output = Zero;
    fn wedge(self, _rhs: WX) -> Self::Output {
        Zero {}
    }
}

impl Wedge<WY> for W {
    type Output = Zero;
    fn wedge(self, _rhs: WY) -> Self::Output {
        Zero {}
    }
}

impl Wedge<WZ> for W {
    type Output = Zero;
    fn wedge(self, _rhs: WZ) -> Self::Output {
        Zero {}
    }
}

impl Wedge<YZ> for W {
    type Output = YZW;
    fn wedge(self, rhs: YZ) -> Self::Output {
        YZW(self.0 * rhs.0)
    }
}

impl Wedge<ZX> for W {
    type Output = ZXW;
    fn wedge(self, rhs: ZX) -> Self::Output {
        ZXW(self.0 * rhs.0)
    }
}

impl Wedge<XY> for W {
    type Output = XYW;
    fn wedge(self, rhs: XY) -> Self::Output {
        XYW(self.0 * rhs.0)
    }
}

impl Wedge<YZW> for W {
    type Output = Zero;
    fn wedge(self, _rhs: YZW) -> Self::Output {
        Zero {}
    }
}

impl Wedge<ZXW> for W {
    type Output = Zero;
    fn wedge(self, _rhs: ZXW) -> Self::Output {
        Zero {}
    }
}

impl Wedge<XYW> for W {
    type Output = Zero;
    fn wedge(self, _rhs: XYW) -> Self::Output {
        Zero {}
    }
}

impl Wedge<ZYX> for W {
    type Output = XYZW;
    fn wedge(self, rhs: ZYX) -> Self::Output {
        XYZW(self.0 * rhs.0)
    }
}

impl Wedge<XYZW> for W {
    type Output = Zero;
    fn wedge(self, _rhs: XYZW) -> Self::Output {
        Zero {}
    }
}

impl Wedge<S> for WX {
    type Output = WX;
    fn wedge(self, rhs: S) -> Self::Output {
        WX(self.0 * rhs.0)
    }
}

impl Wedge<X> for WX {
    type Output = Zero;
    fn wedge(self, _rhs: X) -> Self::Output {
        Zero {}
    }
}

impl Wedge<Y> for WX {
    type Output = XYW;
    fn wedge(self, rhs: Y) -> Self::Output {
        XYW(self.0 * rhs.0)
    }
}

impl Wedge<Z> for WX {
    type Output = ZXW;
    fn wedge(self, rhs: Z) -> Self::Output {
        ZXW(-self.0 * rhs.0)
    }
}

impl Wedge<W> for WX {
    type Output = Zero;
    fn wedge(self, _rhs: W) -> Self::Output {
        Zero {}
    }
}

impl Wedge<WX> for WX {
    type Output = Zero;
    fn wedge(self, _rhs: WX) -> Self::Output {
        Zero {}
    }
}

impl Wedge<WY> for WX {
    type Output = Zero;
    fn wedge(self, _rhs: WY) -> Self::Output {
        Zero {}
    }
}

impl Wedge<WZ> for WX {
    type Output = Zero;
    fn wedge(self, _rhs: WZ) -> Self::Output {
        Zero {}
    }
}

impl Wedge<YZ> for WX {
    type Output = XYZW;
    fn wedge(self, rhs: YZ) -> Self::Output {
        XYZW(-self.0 * rhs.0)
    }
}

impl Wedge<ZX> for WX {
    type Output = Zero;
    fn wedge(self, _rhs: ZX) -> Self::Output {
        Zero {}
    }
}

impl Wedge<XY> for WX {
    type Output = Zero;
    fn wedge(self, _rhs: XY) -> Self::Output {
        Zero {}
    }
}

impl Wedge<YZW> for WX {
    type Output = Zero;
    fn wedge(self, _rhs: YZW) -> Self::Output {
        Zero {}
    }
}

impl Wedge<ZXW> for WX {
    type Output = Zero;
    fn wedge(self, _rhs: ZXW) -> Self::Output {
        Zero {}
    }
}

impl Wedge<XYW> for WX {
    type Output = Zero;
    fn wedge(self, _rhs: XYW) -> Self::Output {
        Zero {}
    }
}

impl Wedge<ZYX> for WX {
    type Output = Zero;
    fn wedge(self, _rhs: ZYX) -> Self::Output {
        Zero {}
    }
}

impl Wedge<XYZW> for WX {
    type Output = Zero;
    fn wedge(self, _rhs: XYZW) -> Self::Output {
        Zero {}
    }
}

impl Wedge<S> for WY {
    type Output = WY;
    fn wedge(self, rhs: S) -> Self::Output {
        WY(self.0 * rhs.0)
    }
}

impl Wedge<X> for WY {
    type Output = XYW;
    fn wedge(self, rhs: X) -> Self::Output {
        XYW(-self.0 * rhs.0)
    }
}

impl Wedge<Y> for WY {
    type Output = Zero;
    fn wedge(self, _rhs: Y) -> Self::Output {
        Zero {}
    }
}

impl Wedge<Z> for WY {
    type Output = YZW;
    fn wedge(self, rhs: Z) -> Self::Output {
        YZW(self.0 * rhs.0)
    }
}

impl Wedge<W> for WY {
    type Output = Zero;
    fn wedge(self, _rhs: W) -> Self::Output {
        Zero {}
    }
}

impl Wedge<WX> for WY {
    type Output = Zero;
    fn wedge(self, _rhs: WX) -> Self::Output {
        Zero {}
    }
}

impl Wedge<WY> for WY {
    type Output = Zero;
    fn wedge(self, _rhs: WY) -> Self::Output {
        Zero {}
    }
}

impl Wedge<WZ> for WY {
    type Output = Zero;
    fn wedge(self, _rhs: WZ) -> Self::Output {
        Zero {}
    }
}

impl Wedge<YZ> for WY {
    type Output = Zero;
    fn wedge(self, _rhs: YZ) -> Self::Output {
        Zero {}
    }
}

impl Wedge<ZX> for WY {
    type Output = XYZW;
    fn wedge(self, rhs: ZX) -> Self::Output {
        XYZW(-self.0 * rhs.0)
    }
}

impl Wedge<XY> for WY {
    type Output = Zero;
    fn wedge(self, _rhs: XY) -> Self::Output {
        Zero {}
    }
}

impl Wedge<YZW> for WY {
    type Output = Zero;
    fn wedge(self, _rhs: YZW) -> Self::Output {
        Zero {}
    }
}

impl Wedge<ZXW> for WY {
    type Output = Zero;
    fn wedge(self, _rhs: ZXW) -> Self::Output {
        Zero {}
    }
}

impl Wedge<XYW> for WY {
    type Output = Zero;
    fn wedge(self, _rhs: XYW) -> Self::Output {
        Zero {}
    }
}

impl Wedge<ZYX> for WY {
    type Output = Zero;
    fn wedge(self, _rhs: ZYX) -> Self::Output {
        Zero {}
    }
}

impl Wedge<XYZW> for WY {
    type Output = Zero;
    fn wedge(self, _rhs: XYZW) -> Self::Output {
        Zero {}
    }
}

impl Wedge<S> for WZ {
    type Output = WZ;
    fn wedge(self, rhs: S) -> Self::Output {
        WZ(self.0 * rhs.0)
    }
}

impl Wedge<X> for WZ {
    type Output = ZXW;
    fn wedge(self, rhs: X) -> Self::Output {
        ZXW(self.0 * rhs.0)
    }
}

impl Wedge<Y> for WZ {
    type Output = YZW;
    fn wedge(self, rhs: Y) -> Self::Output {
        YZW(-self.0 * rhs.0)
    }
}

impl Wedge<Z> for WZ {
    type Output = Zero;
    fn wedge(self, _rhs: Z) -> Self::Output {
        Zero {}
    }
}

impl Wedge<W> for WZ {
    type Output = Zero;
    fn wedge(self, _rhs: W) -> Self::Output {
        Zero {}
    }
}

impl Wedge<WX> for WZ {
    type Output = Zero;
    fn wedge(self, _rhs: WX) -> Self::Output {
        Zero {}
    }
}

impl Wedge<WY> for WZ {
    type Output = Zero;
    fn wedge(self, _rhs: WY) -> Self::Output {
        Zero {}
    }
}

impl Wedge<WZ> for WZ {
    type Output = Zero;
    fn wedge(self, _rhs: WZ) -> Self::Output {
        Zero {}
    }
}

impl Wedge<YZ> for WZ {
    type Output = Zero;
    fn wedge(self, _rhs: YZ) -> Self::Output {
        Zero {}
    }
}

impl Wedge<ZX> for WZ {
    type Output = Zero;
    fn wedge(self, _rhs: ZX) -> Self::Output {
        Zero {}
    }
}

impl Wedge<XY> for WZ {
    type Output = XYZW;
    fn wedge(self, rhs: XY) -> Self::Output {
        XYZW(-self.0 * rhs.0)
    }
}

impl Wedge<YZW> for WZ {
    type Output = Zero;
    fn wedge(self, _rhs: YZW) -> Self::Output {
        Zero {}
    }
}

impl Wedge<ZXW> for WZ {
    type Output = Zero;
    fn wedge(self, _rhs: ZXW) -> Self::Output {
        Zero {}
    }
}

impl Wedge<XYW> for WZ {
    type Output = Zero;
    fn wedge(self, _rhs: XYW) -> Self::Output {
        Zero {}
    }
}

impl Wedge<ZYX> for WZ {
    type Output = Zero;
    fn wedge(self, _rhs: ZYX) -> Self::Output {
        Zero {}
    }
}

impl Wedge<XYZW> for WZ {
    type Output = Zero;
    fn wedge(self, _rhs: XYZW) -> Self::Output {
        Zero {}
    }
}

impl Wedge<S> for YZ {
    type Output = YZ;
    fn wedge(self, rhs: S) -> Self::Output {
        YZ(self.0 * rhs.0)
    }
}

impl Wedge<X> for YZ {
    type Output = ZYX;
    fn wedge(self, rhs: X) -> Self::Output {
        ZYX(-self.0 * rhs.0)
    }
}

impl Wedge<Y> for YZ {
    type Output = Zero;
    fn wedge(self, _rhs: Y) -> Self::Output {
        Zero {}
    }
}

impl Wedge<Z> for YZ {
    type Output = Zero;
    fn wedge(self, _rhs: Z) -> Self::Output {
        Zero {}
    }
}

impl Wedge<W> for YZ {
    type Output = YZW;
    fn wedge(self, rhs: W) -> Self::Output {
        YZW(self.0 * rhs.0)
    }
}

impl Wedge<WX> for YZ {
    type Output = XYZW;
    fn wedge(self, rhs: WX) -> Self::Output {
        XYZW(-self.0 * rhs.0)
    }
}

impl Wedge<WY> for YZ {
    type Output = Zero;
    fn wedge(self, _rhs: WY) -> Self::Output {
        Zero {}
    }
}

impl Wedge<WZ> for YZ {
    type Output = Zero;
    fn wedge(self, _rhs: WZ) -> Self::Output {
        Zero {}
    }
}

impl Wedge<YZ> for YZ {
    type Output = Zero;
    fn wedge(self, _rhs: YZ) -> Self::Output {
        Zero {}
    }
}

impl Wedge<ZX> for YZ {
    type Output = Zero;
    fn wedge(self, _rhs: ZX) -> Self::Output {
        Zero {}
    }
}

impl Wedge<XY> for YZ {
    type Output = Zero;
    fn wedge(self, _rhs: XY) -> Self::Output {
        Zero {}
    }
}

impl Wedge<YZW> for YZ {
    type Output = Zero;
    fn wedge(self, _rhs: YZW) -> Self::Output {
        Zero {}
    }
}

impl Wedge<ZXW> for YZ {
    type Output = Zero;
    fn wedge(self, _rhs: ZXW) -> Self::Output {
        Zero {}
    }
}

impl Wedge<XYW> for YZ {
    type Output = Zero;
    fn wedge(self, _rhs: XYW) -> Self::Output {
        Zero {}
    }
}

impl Wedge<ZYX> for YZ {
    type Output = Zero;
    fn wedge(self, _rhs: ZYX) -> Self::Output {
        Zero {}
    }
}

impl Wedge<XYZW> for YZ {
    type Output = Zero;
    fn wedge(self, _rhs: XYZW) -> Self::Output {
        Zero {}
    }
}

impl Wedge<S> for ZX {
    type Output = ZX;
    fn wedge(self, rhs: S) -> Self::Output {
        ZX(self.0 * rhs.0)
    }
}

impl Wedge<X> for ZX {
    type Output = Zero;
    fn wedge(self, _rhs: X) -> Self::Output {
        Zero {}
    }
}

impl Wedge<Y> for ZX {
    type Output = ZYX;
    fn wedge(self, rhs: Y) -> Self::Output {
        ZYX(-self.0 * rhs.0)
    }
}

impl Wedge<Z> for ZX {
    type Output = Zero;
    fn wedge(self, _rhs: Z) -> Self::Output {
        Zero {}
    }
}

impl Wedge<W> for ZX {
    type Output = ZXW;
    fn wedge(self, rhs: W) -> Self::Output {
        ZXW(self.0 * rhs.0)
    }
}

impl Wedge<WX> for ZX {
    type Output = Zero;
    fn wedge(self, _rhs: WX) -> Self::Output {
        Zero {}
    }
}

impl Wedge<WY> for ZX {
    type Output = XYZW;
    fn wedge(self, rhs: WY) -> Self::Output {
        XYZW(-self.0 * rhs.0)
    }
}

impl Wedge<WZ> for ZX {
    type Output = Zero;
    fn wedge(self, _rhs: WZ) -> Self::Output {
        Zero {}
    }
}

impl Wedge<YZ> for ZX {
    type Output = Zero;
    fn wedge(self, _rhs: YZ) -> Self::Output {
        Zero {}
    }
}

impl Wedge<ZX> for ZX {
    type Output = Zero;
    fn wedge(self, _rhs: ZX) -> Self::Output {
        Zero {}
    }
}

impl Wedge<XY> for ZX {
    type Output = Zero;
    fn wedge(self, _rhs: XY) -> Self::Output {
        Zero {}
    }
}

impl Wedge<YZW> for ZX {
    type Output = Zero;
    fn wedge(self, _rhs: YZW) -> Self::Output {
        Zero {}
    }
}

impl Wedge<ZXW> for ZX {
    type Output = Zero;
    fn wedge(self, _rhs: ZXW) -> Self::Output {
        Zero {}
    }
}

impl Wedge<XYW> for ZX {
    type Output = Zero;
    fn wedge(self, _rhs: XYW) -> Self::Output {
        Zero {}
    }
}

impl Wedge<ZYX> for ZX {
    type Output = Zero;
    fn wedge(self, _rhs: ZYX) -> Self::Output {
        Zero {}
    }
}

impl Wedge<XYZW> for ZX {
    type Output = Zero;
    fn wedge(self, _rhs: XYZW) -> Self::Output {
        Zero {}
    }
}

impl Wedge<S> for XY {
    type Output = XY;
    fn wedge(self, rhs: S) -> Self::Output {
        XY(self.0 * rhs.0)
    }
}

impl Wedge<X> for XY {
    type Output = Zero;
    fn wedge(self, _rhs: X) -> Self::Output {
        Zero {}
    }
}

impl Wedge<Y> for XY {
    type Output = Zero;
    fn wedge(self, _rhs: Y) -> Self::Output {
        Zero {}
    }
}

impl Wedge<Z> for XY {
    type Output = ZYX;
    fn wedge(self, rhs: Z) -> Self::Output {
        ZYX(-self.0 * rhs.0)
    }
}

impl Wedge<W> for XY {
    type Output = XYW;
    fn wedge(self, rhs: W) -> Self::Output {
        XYW(self.0 * rhs.0)
    }
}

impl Wedge<WX> for XY {
    type Output = Zero;
    fn wedge(self, _rhs: WX) -> Self::Output {
        Zero {}
    }
}

impl Wedge<WY> for XY {
    type Output = Zero;
    fn wedge(self, _rhs: WY) -> Self::Output {
        Zero {}
    }
}

impl Wedge<WZ> for XY {
    type Output = XYZW;
    fn wedge(self, rhs: WZ) -> Self::Output {
        XYZW(-self.0 * rhs.0)
    }
}

impl Wedge<YZ> for XY {
    type Output = Zero;
    fn wedge(self, _rhs: YZ) -> Self::Output {
        Zero {}
    }
}

impl Wedge<ZX> for XY {
    type Output = Zero;
    fn wedge(self, _rhs: ZX) -> Self::Output {
        Zero {}
    }
}

impl Wedge<XY> for XY {
    type Output = Zero;
    fn wedge(self, _rhs: XY) -> Self::Output {
        Zero {}
    }
}

impl Wedge<YZW> for XY {
    type Output = Zero;
    fn wedge(self, _rhs: YZW) -> Self::Output {
        Zero {}
    }
}

impl Wedge<ZXW> for XY {
    type Output = Zero;
    fn wedge(self, _rhs: ZXW) -> Self::Output {
        Zero {}
    }
}

impl Wedge<XYW> for XY {
    type Output = Zero;
    fn wedge(self, _rhs: XYW) -> Self::Output {
        Zero {}
    }
}

impl Wedge<ZYX> for XY {
    type Output = Zero;
    fn wedge(self, _rhs: ZYX) -> Self::Output {
        Zero {}
    }
}

impl Wedge<XYZW> for XY {
    type Output = Zero;
    fn wedge(self, _rhs: XYZW) -> Self::Output {
        Zero {}
    }
}

impl Wedge<S> for YZW {
    type Output = YZW;
    fn wedge(self, rhs: S) -> Self::Output {
        YZW(self.0 * rhs.0)
    }
}

impl Wedge<X> for YZW {
    type Output = XYZW;
    fn wedge(self, rhs: X) -> Self::Output {
        XYZW(-self.0 * rhs.0)
    }
}

impl Wedge<Y> for YZW {
    type Output = Zero;
    fn wedge(self, _rhs: Y) -> Self::Output {
        Zero {}
    }
}

impl Wedge<Z> for YZW {
    type Output = Zero;
    fn wedge(self, _rhs: Z) -> Self::Output {
        Zero {}
    }
}

impl Wedge<W> for YZW {
    type Output = Zero;
    fn wedge(self, _rhs: W) -> Self::Output {
        Zero {}
    }
}

impl Wedge<WX> for YZW {
    type Output = Zero;
    fn wedge(self, _rhs: WX) -> Self::Output {
        Zero {}
    }
}

impl Wedge<WY> for YZW {
    type Output = Zero;
    fn wedge(self, _rhs: WY) -> Self::Output {
        Zero {}
    }
}

impl Wedge<WZ> for YZW {
    type Output = Zero;
    fn wedge(self, _rhs: WZ) -> Self::Output {
        Zero {}
    }
}

impl Wedge<YZ> for YZW {
    type Output = Zero;
    fn wedge(self, _rhs: YZ) -> Self::Output {
        Zero {}
    }
}

impl Wedge<ZX> for YZW {
    type Output = Zero;
    fn wedge(self, _rhs: ZX) -> Self::Output {
        Zero {}
    }
}

impl Wedge<XY> for YZW {
    type Output = Zero;
    fn wedge(self, _rhs: XY) -> Self::Output {
        Zero {}
    }
}

impl Wedge<YZW> for YZW {
    type Output = Zero;
    fn wedge(self, _rhs: YZW) -> Self::Output {
        Zero {}
    }
}

impl Wedge<ZXW> for YZW {
    type Output = Zero;
    fn wedge(self, _rhs: ZXW) -> Self::Output {
        Zero {}
    }
}

impl Wedge<XYW> for YZW {
    type Output = Zero;
    fn wedge(self, _rhs: XYW) -> Self::Output {
        Zero {}
    }
}

impl Wedge<ZYX> for YZW {
    type Output = Zero;
    fn wedge(self, _rhs: ZYX) -> Self::Output {
        Zero {}
    }
}

impl Wedge<XYZW> for YZW {
    type Output = Zero;
    fn wedge(self, _rhs: XYZW) -> Self::Output {
        Zero {}
    }
}

impl Wedge<S> for ZXW {
    type Output = ZXW;
    fn wedge(self, rhs: S) -> Self::Output {
        ZXW(self.0 * rhs.0)
    }
}

impl Wedge<X> for ZXW {
    type Output = Zero;
    fn wedge(self, _rhs: X) -> Self::Output {
        Zero {}
    }
}

impl Wedge<Y> for ZXW {
    type Output = XYZW;
    fn wedge(self, rhs: Y) -> Self::Output {
        XYZW(-self.0 * rhs.0)
    }
}

impl Wedge<Z> for ZXW {
    type Output = Zero;
    fn wedge(self, _rhs: Z) -> Self::Output {
        Zero {}
    }
}

impl Wedge<W> for ZXW {
    type Output = Zero;
    fn wedge(self, _rhs: W) -> Self::Output {
        Zero {}
    }
}

impl Wedge<WX> for ZXW {
    type Output = Zero;
    fn wedge(self, _rhs: WX) -> Self::Output {
        Zero {}
    }
}

impl Wedge<WY> for ZXW {
    type Output = Zero;
    fn wedge(self, _rhs: WY) -> Self::Output {
        Zero {}
    }
}

impl Wedge<WZ> for ZXW {
    type Output = Zero;
    fn wedge(self, _rhs: WZ) -> Self::Output {
        Zero {}
    }
}

impl Wedge<YZ> for ZXW {
    type Output = Zero;
    fn wedge(self, _rhs: YZ) -> Self::Output {
        Zero {}
    }
}

impl Wedge<ZX> for ZXW {
    type Output = Zero;
    fn wedge(self, _rhs: ZX) -> Self::Output {
        Zero {}
    }
}

impl Wedge<XY> for ZXW {
    type Output = Zero;
    fn wedge(self, _rhs: XY) -> Self::Output {
        Zero {}
    }
}

impl Wedge<YZW> for ZXW {
    type Output = Zero;
    fn wedge(self, _rhs: YZW) -> Self::Output {
        Zero {}
    }
}

impl Wedge<ZXW> for ZXW {
    type Output = Zero;
    fn wedge(self, _rhs: ZXW) -> Self::Output {
        Zero {}
    }
}

impl Wedge<XYW> for ZXW {
    type Output = Zero;
    fn wedge(self, _rhs: XYW) -> Self::Output {
        Zero {}
    }
}

impl Wedge<ZYX> for ZXW {
    type Output = Zero;
    fn wedge(self, _rhs: ZYX) -> Self::Output {
        Zero {}
    }
}

impl Wedge<XYZW> for ZXW {
    type Output = Zero;
    fn wedge(self, _rhs: XYZW) -> Self::Output {
        Zero {}
    }
}

impl Wedge<S> for XYW {
    type Output = XYW;
    fn wedge(self, rhs: S) -> Self::Output {
        XYW(self.0 * rhs.0)
    }
}

impl Wedge<X> for XYW {
    type Output = Zero;
    fn wedge(self, _rhs: X) -> Self::Output {
        Zero {}
    }
}

impl Wedge<Y> for XYW {
    type Output = Zero;
    fn wedge(self, _rhs: Y) -> Self::Output {
        Zero {}
    }
}

impl Wedge<Z> for XYW {
    type Output = XYZW;
    fn wedge(self, rhs: Z) -> Self::Output {
        XYZW(-self.0 * rhs.0)
    }
}

impl Wedge<W> for XYW {
    type Output = Zero;
    fn wedge(self, _rhs: W) -> Self::Output {
        Zero {}
    }
}

impl Wedge<WX> for XYW {
    type Output = Zero;
    fn wedge(self, _rhs: WX) -> Self::Output {
        Zero {}
    }
}

impl Wedge<WY> for XYW {
    type Output = Zero;
    fn wedge(self, _rhs: WY) -> Self::Output {
        Zero {}
    }
}

impl Wedge<WZ> for XYW {
    type Output = Zero;
    fn wedge(self, _rhs: WZ) -> Self::Output {
        Zero {}
    }
}

impl Wedge<YZ> for XYW {
    type Output = Zero;
    fn wedge(self, _rhs: YZ) -> Self::Output {
        Zero {}
    }
}

impl Wedge<ZX> for XYW {
    type Output = Zero;
    fn wedge(self, _rhs: ZX) -> Self::Output {
        Zero {}
    }
}

impl Wedge<XY> for XYW {
    type Output = Zero;
    fn wedge(self, _rhs: XY) -> Self::Output {
        Zero {}
    }
}

impl Wedge<YZW> for XYW {
    type Output = Zero;
    fn wedge(self, _rhs: YZW) -> Self::Output {
        Zero {}
    }
}

impl Wedge<ZXW> for XYW {
    type Output = Zero;
    fn wedge(self, _rhs: ZXW) -> Self::Output {
        Zero {}
    }
}

impl Wedge<XYW> for XYW {
    type Output = Zero;
    fn wedge(self, _rhs: XYW) -> Self::Output {
        Zero {}
    }
}

impl Wedge<ZYX> for XYW {
    type Output = Zero;
    fn wedge(self, _rhs: ZYX) -> Self::Output {
        Zero {}
    }
}

impl Wedge<XYZW> for XYW {
    type Output = Zero;
    fn wedge(self, _rhs: XYZW) -> Self::Output {
        Zero {}
    }
}

impl Wedge<S> for ZYX {
    type Output = ZYX;
    fn wedge(self, rhs: S) -> Self::Output {
        ZYX(self.0 * rhs.0)
    }
}

impl Wedge<X> for ZYX {
    type Output = Zero;
    fn wedge(self, _rhs: X) -> Self::Output {
        Zero {}
    }
}

impl Wedge<Y> for ZYX {
    type Output = Zero;
    fn wedge(self, _rhs: Y) -> Self::Output {
        Zero {}
    }
}

impl Wedge<Z> for ZYX {
    type Output = Zero;
    fn wedge(self, _rhs: Z) -> Self::Output {
        Zero {}
    }
}

impl Wedge<W> for ZYX {
    type Output = XYZW;
    fn wedge(self, rhs: W) -> Self::Output {
        XYZW(-self.0 * rhs.0)
    }
}

impl Wedge<WX> for ZYX {
    type Output = Zero;
    fn wedge(self, _rhs: WX) -> Self::Output {
        Zero {}
    }
}

impl Wedge<WY> for ZYX {
    type Output = Zero;
    fn wedge(self, _rhs: WY) -> Self::Output {
        Zero {}
    }
}

impl Wedge<WZ> for ZYX {
    type Output = Zero;
    fn wedge(self, _rhs: WZ) -> Self::Output {
        Zero {}
    }
}

impl Wedge<YZ> for ZYX {
    type Output = Zero;
    fn wedge(self, _rhs: YZ) -> Self::Output {
        Zero {}
    }
}

impl Wedge<ZX> for ZYX {
    type Output = Zero;
    fn wedge(self, _rhs: ZX) -> Self::Output {
        Zero {}
    }
}

impl Wedge<XY> for ZYX {
    type Output = Zero;
    fn wedge(self, _rhs: XY) -> Self::Output {
        Zero {}
    }
}

impl Wedge<YZW> for ZYX {
    type Output = Zero;
    fn wedge(self, _rhs: YZW) -> Self::Output {
        Zero {}
    }
}

impl Wedge<ZXW> for ZYX {
    type Output = Zero;
    fn wedge(self, _rhs: ZXW) -> Self::Output {
        Zero {}
    }
}

impl Wedge<XYW> for ZYX {
    type Output = Zero;
    fn wedge(self, _rhs: XYW) -> Self::Output {
        Zero {}
    }
}

impl Wedge<ZYX> for ZYX {
    type Output = Zero;
    fn wedge(self, _rhs: ZYX) -> Self::Output {
        Zero {}
    }
}

impl Wedge<XYZW> for ZYX {
    type Output = Zero;
    fn wedge(self, _rhs: XYZW) -> Self::Output {
        Zero {}
    }
}

impl Wedge<S> for XYZW {
    type Output = XYZW;
    fn wedge(self, rhs: S) -> Self::Output {
        XYZW(self.0 * rhs.0)
    }
}

impl Wedge<X> for XYZW {
    type Output = Zero;
    fn wedge(self, _rhs: X) -> Self::Output {
        Zero {}
    }
}

impl Wedge<Y> for XYZW {
    type Output = Zero;
    fn wedge(self, _rhs: Y) -> Self::Output {
        Zero {}
    }
}

impl Wedge<Z> for XYZW {
    type Output = Zero;
    fn wedge(self, _rhs: Z) -> Self::Output {
        Zero {}
    }
}

impl Wedge<W> for XYZW {
    type Output = Zero;
    fn wedge(self, _rhs: W) -> Self::Output {
        Zero {}
    }
}

impl Wedge<WX> for XYZW {
    type Output = Zero;
    fn wedge(self, _rhs: WX) -> Self::Output {
        Zero {}
    }
}

impl Wedge<WY> for XYZW {
    type Output = Zero;
    fn wedge(self, _rhs: WY) -> Self::Output {
        Zero {}
    }
}

impl Wedge<WZ> for XYZW {
    type Output = Zero;
    fn wedge(self, _rhs: WZ) -> Self::Output {
        Zero {}
    }
}

impl Wedge<YZ> for XYZW {
    type Output = Zero;
    fn wedge(self, _rhs: YZ) -> Self::Output {
        Zero {}
    }
}

impl Wedge<ZX> for XYZW {
    type Output = Zero;
    fn wedge(self, _rhs: ZX) -> Self::Output {
        Zero {}
    }
}

impl Wedge<XY> for XYZW {
    type Output = Zero;
    fn wedge(self, _rhs: XY) -> Self::Output {
        Zero {}
    }
}

impl Wedge<YZW> for XYZW {
    type Output = Zero;
    fn wedge(self, _rhs: YZW) -> Self::Output {
        Zero {}
    }
}

impl Wedge<ZXW> for XYZW {
    type Output = Zero;
    fn wedge(self, _rhs: ZXW) -> Self::Output {
        Zero {}
    }
}

impl Wedge<XYW> for XYZW {
    type Output = Zero;
    fn wedge(self, _rhs: XYW) -> Self::Output {
        Zero {}
    }
}

impl Wedge<ZYX> for XYZW {
    type Output = Zero;
    fn wedge(self, _rhs: ZYX) -> Self::Output {
        Zero {}
    }
}

impl Wedge<XYZW> for XYZW {
    type Output = Zero;
    fn wedge(self, _rhs: XYZW) -> Self::Output {
        Zero {}
    }
}

// ---------------------------------------------------------------------
// impl AntiWedge for blades:

impl AntiWedge<S> for S {
    type Output = Zero;
    fn anti_wedge(self, _rhs: S) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<X> for S {
    type Output = Zero;
    fn anti_wedge(self, _rhs: X) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<Y> for S {
    type Output = Zero;
    fn anti_wedge(self, _rhs: Y) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<Z> for S {
    type Output = Zero;
    fn anti_wedge(self, _rhs: Z) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<W> for S {
    type Output = Zero;
    fn anti_wedge(self, _rhs: W) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<WX> for S {
    type Output = Zero;
    fn anti_wedge(self, _rhs: WX) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<WY> for S {
    type Output = Zero;
    fn anti_wedge(self, _rhs: WY) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<WZ> for S {
    type Output = Zero;
    fn anti_wedge(self, _rhs: WZ) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<YZ> for S {
    type Output = Zero;
    fn anti_wedge(self, _rhs: YZ) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<ZX> for S {
    type Output = Zero;
    fn anti_wedge(self, _rhs: ZX) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<XY> for S {
    type Output = Zero;
    fn anti_wedge(self, _rhs: XY) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<YZW> for S {
    type Output = Zero;
    fn anti_wedge(self, _rhs: YZW) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<ZXW> for S {
    type Output = Zero;
    fn anti_wedge(self, _rhs: ZXW) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<XYW> for S {
    type Output = Zero;
    fn anti_wedge(self, _rhs: XYW) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<ZYX> for S {
    type Output = Zero;
    fn anti_wedge(self, _rhs: ZYX) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<XYZW> for S {
    type Output = S;
    fn anti_wedge(self, rhs: XYZW) -> Self::Output {
        S(self.0 * rhs.0)
    }
}

impl AntiWedge<S> for X {
    type Output = Zero;
    fn anti_wedge(self, _rhs: S) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<X> for X {
    type Output = Zero;
    fn anti_wedge(self, _rhs: X) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<Y> for X {
    type Output = Zero;
    fn anti_wedge(self, _rhs: Y) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<Z> for X {
    type Output = Zero;
    fn anti_wedge(self, _rhs: Z) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<W> for X {
    type Output = Zero;
    fn anti_wedge(self, _rhs: W) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<WX> for X {
    type Output = Zero;
    fn anti_wedge(self, _rhs: WX) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<WY> for X {
    type Output = Zero;
    fn anti_wedge(self, _rhs: WY) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<WZ> for X {
    type Output = Zero;
    fn anti_wedge(self, _rhs: WZ) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<YZ> for X {
    type Output = Zero;
    fn anti_wedge(self, _rhs: YZ) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<ZX> for X {
    type Output = Zero;
    fn anti_wedge(self, _rhs: ZX) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<XY> for X {
    type Output = Zero;
    fn anti_wedge(self, _rhs: XY) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<YZW> for X {
    type Output = S;
    fn anti_wedge(self, rhs: YZW) -> Self::Output {
        S(self.0 * rhs.0)
    }
}

impl AntiWedge<ZXW> for X {
    type Output = Zero;
    fn anti_wedge(self, _rhs: ZXW) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<XYW> for X {
    type Output = Zero;
    fn anti_wedge(self, _rhs: XYW) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<ZYX> for X {
    type Output = Zero;
    fn anti_wedge(self, _rhs: ZYX) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<XYZW> for X {
    type Output = X;
    fn anti_wedge(self, rhs: XYZW) -> Self::Output {
        X(self.0 * rhs.0)
    }
}

impl AntiWedge<S> for Y {
    type Output = Zero;
    fn anti_wedge(self, _rhs: S) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<X> for Y {
    type Output = Zero;
    fn anti_wedge(self, _rhs: X) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<Y> for Y {
    type Output = Zero;
    fn anti_wedge(self, _rhs: Y) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<Z> for Y {
    type Output = Zero;
    fn anti_wedge(self, _rhs: Z) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<W> for Y {
    type Output = Zero;
    fn anti_wedge(self, _rhs: W) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<WX> for Y {
    type Output = Zero;
    fn anti_wedge(self, _rhs: WX) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<WY> for Y {
    type Output = Zero;
    fn anti_wedge(self, _rhs: WY) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<WZ> for Y {
    type Output = Zero;
    fn anti_wedge(self, _rhs: WZ) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<YZ> for Y {
    type Output = Zero;
    fn anti_wedge(self, _rhs: YZ) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<ZX> for Y {
    type Output = Zero;
    fn anti_wedge(self, _rhs: ZX) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<XY> for Y {
    type Output = Zero;
    fn anti_wedge(self, _rhs: XY) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<YZW> for Y {
    type Output = Zero;
    fn anti_wedge(self, _rhs: YZW) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<ZXW> for Y {
    type Output = S;
    fn anti_wedge(self, rhs: ZXW) -> Self::Output {
        S(self.0 * rhs.0)
    }
}

impl AntiWedge<XYW> for Y {
    type Output = Zero;
    fn anti_wedge(self, _rhs: XYW) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<ZYX> for Y {
    type Output = Zero;
    fn anti_wedge(self, _rhs: ZYX) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<XYZW> for Y {
    type Output = Y;
    fn anti_wedge(self, rhs: XYZW) -> Self::Output {
        Y(self.0 * rhs.0)
    }
}

impl AntiWedge<S> for Z {
    type Output = Zero;
    fn anti_wedge(self, _rhs: S) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<X> for Z {
    type Output = Zero;
    fn anti_wedge(self, _rhs: X) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<Y> for Z {
    type Output = Zero;
    fn anti_wedge(self, _rhs: Y) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<Z> for Z {
    type Output = Zero;
    fn anti_wedge(self, _rhs: Z) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<W> for Z {
    type Output = Zero;
    fn anti_wedge(self, _rhs: W) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<WX> for Z {
    type Output = Zero;
    fn anti_wedge(self, _rhs: WX) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<WY> for Z {
    type Output = Zero;
    fn anti_wedge(self, _rhs: WY) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<WZ> for Z {
    type Output = Zero;
    fn anti_wedge(self, _rhs: WZ) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<YZ> for Z {
    type Output = Zero;
    fn anti_wedge(self, _rhs: YZ) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<ZX> for Z {
    type Output = Zero;
    fn anti_wedge(self, _rhs: ZX) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<XY> for Z {
    type Output = Zero;
    fn anti_wedge(self, _rhs: XY) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<YZW> for Z {
    type Output = Zero;
    fn anti_wedge(self, _rhs: YZW) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<ZXW> for Z {
    type Output = Zero;
    fn anti_wedge(self, _rhs: ZXW) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<XYW> for Z {
    type Output = S;
    fn anti_wedge(self, rhs: XYW) -> Self::Output {
        S(self.0 * rhs.0)
    }
}

impl AntiWedge<ZYX> for Z {
    type Output = Zero;
    fn anti_wedge(self, _rhs: ZYX) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<XYZW> for Z {
    type Output = Z;
    fn anti_wedge(self, rhs: XYZW) -> Self::Output {
        Z(self.0 * rhs.0)
    }
}

impl AntiWedge<S> for W {
    type Output = Zero;
    fn anti_wedge(self, _rhs: S) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<X> for W {
    type Output = Zero;
    fn anti_wedge(self, _rhs: X) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<Y> for W {
    type Output = Zero;
    fn anti_wedge(self, _rhs: Y) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<Z> for W {
    type Output = Zero;
    fn anti_wedge(self, _rhs: Z) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<W> for W {
    type Output = Zero;
    fn anti_wedge(self, _rhs: W) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<WX> for W {
    type Output = Zero;
    fn anti_wedge(self, _rhs: WX) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<WY> for W {
    type Output = Zero;
    fn anti_wedge(self, _rhs: WY) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<WZ> for W {
    type Output = Zero;
    fn anti_wedge(self, _rhs: WZ) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<YZ> for W {
    type Output = Zero;
    fn anti_wedge(self, _rhs: YZ) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<ZX> for W {
    type Output = Zero;
    fn anti_wedge(self, _rhs: ZX) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<XY> for W {
    type Output = Zero;
    fn anti_wedge(self, _rhs: XY) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<YZW> for W {
    type Output = Zero;
    fn anti_wedge(self, _rhs: YZW) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<ZXW> for W {
    type Output = Zero;
    fn anti_wedge(self, _rhs: ZXW) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<XYW> for W {
    type Output = Zero;
    fn anti_wedge(self, _rhs: XYW) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<ZYX> for W {
    type Output = S;
    fn anti_wedge(self, rhs: ZYX) -> Self::Output {
        S(self.0 * rhs.0)
    }
}

impl AntiWedge<XYZW> for W {
    type Output = W;
    fn anti_wedge(self, rhs: XYZW) -> Self::Output {
        W(self.0 * rhs.0)
    }
}

impl AntiWedge<S> for WX {
    type Output = Zero;
    fn anti_wedge(self, _rhs: S) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<X> for WX {
    type Output = Zero;
    fn anti_wedge(self, _rhs: X) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<Y> for WX {
    type Output = Zero;
    fn anti_wedge(self, _rhs: Y) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<Z> for WX {
    type Output = Zero;
    fn anti_wedge(self, _rhs: Z) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<W> for WX {
    type Output = Zero;
    fn anti_wedge(self, _rhs: W) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<WX> for WX {
    type Output = Zero;
    fn anti_wedge(self, _rhs: WX) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<WY> for WX {
    type Output = Zero;
    fn anti_wedge(self, _rhs: WY) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<WZ> for WX {
    type Output = Zero;
    fn anti_wedge(self, _rhs: WZ) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<YZ> for WX {
    type Output = S;
    fn anti_wedge(self, rhs: YZ) -> Self::Output {
        S(-self.0 * rhs.0)
    }
}

impl AntiWedge<ZX> for WX {
    type Output = Zero;
    fn anti_wedge(self, _rhs: ZX) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<XY> for WX {
    type Output = Zero;
    fn anti_wedge(self, _rhs: XY) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<YZW> for WX {
    type Output = W;
    fn anti_wedge(self, rhs: YZW) -> Self::Output {
        W(-self.0 * rhs.0)
    }
}

impl AntiWedge<ZXW> for WX {
    type Output = Zero;
    fn anti_wedge(self, _rhs: ZXW) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<XYW> for WX {
    type Output = Zero;
    fn anti_wedge(self, _rhs: XYW) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<ZYX> for WX {
    type Output = X;
    fn anti_wedge(self, rhs: ZYX) -> Self::Output {
        X(self.0 * rhs.0)
    }
}

impl AntiWedge<XYZW> for WX {
    type Output = WX;
    fn anti_wedge(self, rhs: XYZW) -> Self::Output {
        WX(self.0 * rhs.0)
    }
}

impl AntiWedge<S> for WY {
    type Output = Zero;
    fn anti_wedge(self, _rhs: S) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<X> for WY {
    type Output = Zero;
    fn anti_wedge(self, _rhs: X) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<Y> for WY {
    type Output = Zero;
    fn anti_wedge(self, _rhs: Y) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<Z> for WY {
    type Output = Zero;
    fn anti_wedge(self, _rhs: Z) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<W> for WY {
    type Output = Zero;
    fn anti_wedge(self, _rhs: W) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<WX> for WY {
    type Output = Zero;
    fn anti_wedge(self, _rhs: WX) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<WY> for WY {
    type Output = Zero;
    fn anti_wedge(self, _rhs: WY) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<WZ> for WY {
    type Output = Zero;
    fn anti_wedge(self, _rhs: WZ) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<YZ> for WY {
    type Output = Zero;
    fn anti_wedge(self, _rhs: YZ) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<ZX> for WY {
    type Output = S;
    fn anti_wedge(self, rhs: ZX) -> Self::Output {
        S(-self.0 * rhs.0)
    }
}

impl AntiWedge<XY> for WY {
    type Output = Zero;
    fn anti_wedge(self, _rhs: XY) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<YZW> for WY {
    type Output = Zero;
    fn anti_wedge(self, _rhs: YZW) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<ZXW> for WY {
    type Output = W;
    fn anti_wedge(self, rhs: ZXW) -> Self::Output {
        W(-self.0 * rhs.0)
    }
}

impl AntiWedge<XYW> for WY {
    type Output = Zero;
    fn anti_wedge(self, _rhs: XYW) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<ZYX> for WY {
    type Output = Y;
    fn anti_wedge(self, rhs: ZYX) -> Self::Output {
        Y(self.0 * rhs.0)
    }
}

impl AntiWedge<XYZW> for WY {
    type Output = WY;
    fn anti_wedge(self, rhs: XYZW) -> Self::Output {
        WY(self.0 * rhs.0)
    }
}

impl AntiWedge<S> for WZ {
    type Output = Zero;
    fn anti_wedge(self, _rhs: S) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<X> for WZ {
    type Output = Zero;
    fn anti_wedge(self, _rhs: X) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<Y> for WZ {
    type Output = Zero;
    fn anti_wedge(self, _rhs: Y) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<Z> for WZ {
    type Output = Zero;
    fn anti_wedge(self, _rhs: Z) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<W> for WZ {
    type Output = Zero;
    fn anti_wedge(self, _rhs: W) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<WX> for WZ {
    type Output = Zero;
    fn anti_wedge(self, _rhs: WX) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<WY> for WZ {
    type Output = Zero;
    fn anti_wedge(self, _rhs: WY) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<WZ> for WZ {
    type Output = Zero;
    fn anti_wedge(self, _rhs: WZ) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<YZ> for WZ {
    type Output = Zero;
    fn anti_wedge(self, _rhs: YZ) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<ZX> for WZ {
    type Output = Zero;
    fn anti_wedge(self, _rhs: ZX) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<XY> for WZ {
    type Output = S;
    fn anti_wedge(self, rhs: XY) -> Self::Output {
        S(-self.0 * rhs.0)
    }
}

impl AntiWedge<YZW> for WZ {
    type Output = Zero;
    fn anti_wedge(self, _rhs: YZW) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<ZXW> for WZ {
    type Output = Zero;
    fn anti_wedge(self, _rhs: ZXW) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<XYW> for WZ {
    type Output = W;
    fn anti_wedge(self, rhs: XYW) -> Self::Output {
        W(-self.0 * rhs.0)
    }
}

impl AntiWedge<ZYX> for WZ {
    type Output = Z;
    fn anti_wedge(self, rhs: ZYX) -> Self::Output {
        Z(self.0 * rhs.0)
    }
}

impl AntiWedge<XYZW> for WZ {
    type Output = WZ;
    fn anti_wedge(self, rhs: XYZW) -> Self::Output {
        WZ(self.0 * rhs.0)
    }
}

impl AntiWedge<S> for YZ {
    type Output = Zero;
    fn anti_wedge(self, _rhs: S) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<X> for YZ {
    type Output = Zero;
    fn anti_wedge(self, _rhs: X) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<Y> for YZ {
    type Output = Zero;
    fn anti_wedge(self, _rhs: Y) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<Z> for YZ {
    type Output = Zero;
    fn anti_wedge(self, _rhs: Z) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<W> for YZ {
    type Output = Zero;
    fn anti_wedge(self, _rhs: W) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<WX> for YZ {
    type Output = S;
    fn anti_wedge(self, rhs: WX) -> Self::Output {
        S(-self.0 * rhs.0)
    }
}

impl AntiWedge<WY> for YZ {
    type Output = Zero;
    fn anti_wedge(self, _rhs: WY) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<WZ> for YZ {
    type Output = Zero;
    fn anti_wedge(self, _rhs: WZ) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<YZ> for YZ {
    type Output = Zero;
    fn anti_wedge(self, _rhs: YZ) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<ZX> for YZ {
    type Output = Zero;
    fn anti_wedge(self, _rhs: ZX) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<XY> for YZ {
    type Output = Zero;
    fn anti_wedge(self, _rhs: XY) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<YZW> for YZ {
    type Output = Zero;
    fn anti_wedge(self, _rhs: YZW) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<ZXW> for YZ {
    type Output = Z;
    fn anti_wedge(self, rhs: ZXW) -> Self::Output {
        Z(self.0 * rhs.0)
    }
}

impl AntiWedge<XYW> for YZ {
    type Output = Y;
    fn anti_wedge(self, rhs: XYW) -> Self::Output {
        Y(-self.0 * rhs.0)
    }
}

impl AntiWedge<ZYX> for YZ {
    type Output = Zero;
    fn anti_wedge(self, _rhs: ZYX) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<XYZW> for YZ {
    type Output = YZ;
    fn anti_wedge(self, rhs: XYZW) -> Self::Output {
        YZ(self.0 * rhs.0)
    }
}

impl AntiWedge<S> for ZX {
    type Output = Zero;
    fn anti_wedge(self, _rhs: S) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<X> for ZX {
    type Output = Zero;
    fn anti_wedge(self, _rhs: X) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<Y> for ZX {
    type Output = Zero;
    fn anti_wedge(self, _rhs: Y) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<Z> for ZX {
    type Output = Zero;
    fn anti_wedge(self, _rhs: Z) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<W> for ZX {
    type Output = Zero;
    fn anti_wedge(self, _rhs: W) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<WX> for ZX {
    type Output = Zero;
    fn anti_wedge(self, _rhs: WX) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<WY> for ZX {
    type Output = S;
    fn anti_wedge(self, rhs: WY) -> Self::Output {
        S(-self.0 * rhs.0)
    }
}

impl AntiWedge<WZ> for ZX {
    type Output = Zero;
    fn anti_wedge(self, _rhs: WZ) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<YZ> for ZX {
    type Output = Zero;
    fn anti_wedge(self, _rhs: YZ) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<ZX> for ZX {
    type Output = Zero;
    fn anti_wedge(self, _rhs: ZX) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<XY> for ZX {
    type Output = Zero;
    fn anti_wedge(self, _rhs: XY) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<YZW> for ZX {
    type Output = Z;
    fn anti_wedge(self, rhs: YZW) -> Self::Output {
        Z(-self.0 * rhs.0)
    }
}

impl AntiWedge<ZXW> for ZX {
    type Output = Zero;
    fn anti_wedge(self, _rhs: ZXW) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<XYW> for ZX {
    type Output = X;
    fn anti_wedge(self, rhs: XYW) -> Self::Output {
        X(self.0 * rhs.0)
    }
}

impl AntiWedge<ZYX> for ZX {
    type Output = Zero;
    fn anti_wedge(self, _rhs: ZYX) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<XYZW> for ZX {
    type Output = ZX;
    fn anti_wedge(self, rhs: XYZW) -> Self::Output {
        ZX(self.0 * rhs.0)
    }
}

impl AntiWedge<S> for XY {
    type Output = Zero;
    fn anti_wedge(self, _rhs: S) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<X> for XY {
    type Output = Zero;
    fn anti_wedge(self, _rhs: X) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<Y> for XY {
    type Output = Zero;
    fn anti_wedge(self, _rhs: Y) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<Z> for XY {
    type Output = Zero;
    fn anti_wedge(self, _rhs: Z) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<W> for XY {
    type Output = Zero;
    fn anti_wedge(self, _rhs: W) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<WX> for XY {
    type Output = Zero;
    fn anti_wedge(self, _rhs: WX) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<WY> for XY {
    type Output = Zero;
    fn anti_wedge(self, _rhs: WY) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<WZ> for XY {
    type Output = S;
    fn anti_wedge(self, rhs: WZ) -> Self::Output {
        S(-self.0 * rhs.0)
    }
}

impl AntiWedge<YZ> for XY {
    type Output = Zero;
    fn anti_wedge(self, _rhs: YZ) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<ZX> for XY {
    type Output = Zero;
    fn anti_wedge(self, _rhs: ZX) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<XY> for XY {
    type Output = Zero;
    fn anti_wedge(self, _rhs: XY) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<YZW> for XY {
    type Output = Y;
    fn anti_wedge(self, rhs: YZW) -> Self::Output {
        Y(self.0 * rhs.0)
    }
}

impl AntiWedge<ZXW> for XY {
    type Output = X;
    fn anti_wedge(self, rhs: ZXW) -> Self::Output {
        X(-self.0 * rhs.0)
    }
}

impl AntiWedge<XYW> for XY {
    type Output = Zero;
    fn anti_wedge(self, _rhs: XYW) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<ZYX> for XY {
    type Output = Zero;
    fn anti_wedge(self, _rhs: ZYX) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<XYZW> for XY {
    type Output = XY;
    fn anti_wedge(self, rhs: XYZW) -> Self::Output {
        XY(self.0 * rhs.0)
    }
}

impl AntiWedge<S> for YZW {
    type Output = Zero;
    fn anti_wedge(self, _rhs: S) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<X> for YZW {
    type Output = S;
    fn anti_wedge(self, rhs: X) -> Self::Output {
        S(-self.0 * rhs.0)
    }
}

impl AntiWedge<Y> for YZW {
    type Output = Zero;
    fn anti_wedge(self, _rhs: Y) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<Z> for YZW {
    type Output = Zero;
    fn anti_wedge(self, _rhs: Z) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<W> for YZW {
    type Output = Zero;
    fn anti_wedge(self, _rhs: W) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<WX> for YZW {
    type Output = W;
    fn anti_wedge(self, rhs: WX) -> Self::Output {
        W(-self.0 * rhs.0)
    }
}

impl AntiWedge<WY> for YZW {
    type Output = Zero;
    fn anti_wedge(self, _rhs: WY) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<WZ> for YZW {
    type Output = Zero;
    fn anti_wedge(self, _rhs: WZ) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<YZ> for YZW {
    type Output = Zero;
    fn anti_wedge(self, _rhs: YZ) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<ZX> for YZW {
    type Output = Z;
    fn anti_wedge(self, rhs: ZX) -> Self::Output {
        Z(-self.0 * rhs.0)
    }
}

impl AntiWedge<XY> for YZW {
    type Output = Y;
    fn anti_wedge(self, rhs: XY) -> Self::Output {
        Y(self.0 * rhs.0)
    }
}

impl AntiWedge<YZW> for YZW {
    type Output = Zero;
    fn anti_wedge(self, _rhs: YZW) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<ZXW> for YZW {
    type Output = WZ;
    fn anti_wedge(self, rhs: ZXW) -> Self::Output {
        WZ(-self.0 * rhs.0)
    }
}

impl AntiWedge<XYW> for YZW {
    type Output = WY;
    fn anti_wedge(self, rhs: XYW) -> Self::Output {
        WY(self.0 * rhs.0)
    }
}

impl AntiWedge<ZYX> for YZW {
    type Output = YZ;
    fn anti_wedge(self, rhs: ZYX) -> Self::Output {
        YZ(self.0 * rhs.0)
    }
}

impl AntiWedge<XYZW> for YZW {
    type Output = YZW;
    fn anti_wedge(self, rhs: XYZW) -> Self::Output {
        YZW(self.0 * rhs.0)
    }
}

impl AntiWedge<S> for ZXW {
    type Output = Zero;
    fn anti_wedge(self, _rhs: S) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<X> for ZXW {
    type Output = Zero;
    fn anti_wedge(self, _rhs: X) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<Y> for ZXW {
    type Output = S;
    fn anti_wedge(self, rhs: Y) -> Self::Output {
        S(-self.0 * rhs.0)
    }
}

impl AntiWedge<Z> for ZXW {
    type Output = Zero;
    fn anti_wedge(self, _rhs: Z) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<W> for ZXW {
    type Output = Zero;
    fn anti_wedge(self, _rhs: W) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<WX> for ZXW {
    type Output = Zero;
    fn anti_wedge(self, _rhs: WX) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<WY> for ZXW {
    type Output = W;
    fn anti_wedge(self, rhs: WY) -> Self::Output {
        W(-self.0 * rhs.0)
    }
}

impl AntiWedge<WZ> for ZXW {
    type Output = Zero;
    fn anti_wedge(self, _rhs: WZ) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<YZ> for ZXW {
    type Output = Z;
    fn anti_wedge(self, rhs: YZ) -> Self::Output {
        Z(self.0 * rhs.0)
    }
}

impl AntiWedge<ZX> for ZXW {
    type Output = Zero;
    fn anti_wedge(self, _rhs: ZX) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<XY> for ZXW {
    type Output = X;
    fn anti_wedge(self, rhs: XY) -> Self::Output {
        X(-self.0 * rhs.0)
    }
}

impl AntiWedge<YZW> for ZXW {
    type Output = WZ;
    fn anti_wedge(self, rhs: YZW) -> Self::Output {
        WZ(self.0 * rhs.0)
    }
}

impl AntiWedge<ZXW> for ZXW {
    type Output = Zero;
    fn anti_wedge(self, _rhs: ZXW) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<XYW> for ZXW {
    type Output = WX;
    fn anti_wedge(self, rhs: XYW) -> Self::Output {
        WX(-self.0 * rhs.0)
    }
}

impl AntiWedge<ZYX> for ZXW {
    type Output = ZX;
    fn anti_wedge(self, rhs: ZYX) -> Self::Output {
        ZX(self.0 * rhs.0)
    }
}

impl AntiWedge<XYZW> for ZXW {
    type Output = ZXW;
    fn anti_wedge(self, rhs: XYZW) -> Self::Output {
        ZXW(self.0 * rhs.0)
    }
}

impl AntiWedge<S> for XYW {
    type Output = Zero;
    fn anti_wedge(self, _rhs: S) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<X> for XYW {
    type Output = Zero;
    fn anti_wedge(self, _rhs: X) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<Y> for XYW {
    type Output = Zero;
    fn anti_wedge(self, _rhs: Y) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<Z> for XYW {
    type Output = S;
    fn anti_wedge(self, rhs: Z) -> Self::Output {
        S(-self.0 * rhs.0)
    }
}

impl AntiWedge<W> for XYW {
    type Output = Zero;
    fn anti_wedge(self, _rhs: W) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<WX> for XYW {
    type Output = Zero;
    fn anti_wedge(self, _rhs: WX) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<WY> for XYW {
    type Output = Zero;
    fn anti_wedge(self, _rhs: WY) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<WZ> for XYW {
    type Output = W;
    fn anti_wedge(self, rhs: WZ) -> Self::Output {
        W(-self.0 * rhs.0)
    }
}

impl AntiWedge<YZ> for XYW {
    type Output = Y;
    fn anti_wedge(self, rhs: YZ) -> Self::Output {
        Y(-self.0 * rhs.0)
    }
}

impl AntiWedge<ZX> for XYW {
    type Output = X;
    fn anti_wedge(self, rhs: ZX) -> Self::Output {
        X(self.0 * rhs.0)
    }
}

impl AntiWedge<XY> for XYW {
    type Output = Zero;
    fn anti_wedge(self, _rhs: XY) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<YZW> for XYW {
    type Output = WY;
    fn anti_wedge(self, rhs: YZW) -> Self::Output {
        WY(-self.0 * rhs.0)
    }
}

impl AntiWedge<ZXW> for XYW {
    type Output = WX;
    fn anti_wedge(self, rhs: ZXW) -> Self::Output {
        WX(self.0 * rhs.0)
    }
}

impl AntiWedge<XYW> for XYW {
    type Output = Zero;
    fn anti_wedge(self, _rhs: XYW) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<ZYX> for XYW {
    type Output = XY;
    fn anti_wedge(self, rhs: ZYX) -> Self::Output {
        XY(self.0 * rhs.0)
    }
}

impl AntiWedge<XYZW> for XYW {
    type Output = XYW;
    fn anti_wedge(self, rhs: XYZW) -> Self::Output {
        XYW(self.0 * rhs.0)
    }
}

impl AntiWedge<S> for ZYX {
    type Output = Zero;
    fn anti_wedge(self, _rhs: S) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<X> for ZYX {
    type Output = Zero;
    fn anti_wedge(self, _rhs: X) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<Y> for ZYX {
    type Output = Zero;
    fn anti_wedge(self, _rhs: Y) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<Z> for ZYX {
    type Output = Zero;
    fn anti_wedge(self, _rhs: Z) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<W> for ZYX {
    type Output = S;
    fn anti_wedge(self, rhs: W) -> Self::Output {
        S(-self.0 * rhs.0)
    }
}

impl AntiWedge<WX> for ZYX {
    type Output = X;
    fn anti_wedge(self, rhs: WX) -> Self::Output {
        X(self.0 * rhs.0)
    }
}

impl AntiWedge<WY> for ZYX {
    type Output = Y;
    fn anti_wedge(self, rhs: WY) -> Self::Output {
        Y(self.0 * rhs.0)
    }
}

impl AntiWedge<WZ> for ZYX {
    type Output = Z;
    fn anti_wedge(self, rhs: WZ) -> Self::Output {
        Z(self.0 * rhs.0)
    }
}

impl AntiWedge<YZ> for ZYX {
    type Output = Zero;
    fn anti_wedge(self, _rhs: YZ) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<ZX> for ZYX {
    type Output = Zero;
    fn anti_wedge(self, _rhs: ZX) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<XY> for ZYX {
    type Output = Zero;
    fn anti_wedge(self, _rhs: XY) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<YZW> for ZYX {
    type Output = YZ;
    fn anti_wedge(self, rhs: YZW) -> Self::Output {
        YZ(-self.0 * rhs.0)
    }
}

impl AntiWedge<ZXW> for ZYX {
    type Output = ZX;
    fn anti_wedge(self, rhs: ZXW) -> Self::Output {
        ZX(-self.0 * rhs.0)
    }
}

impl AntiWedge<XYW> for ZYX {
    type Output = XY;
    fn anti_wedge(self, rhs: XYW) -> Self::Output {
        XY(-self.0 * rhs.0)
    }
}

impl AntiWedge<ZYX> for ZYX {
    type Output = Zero;
    fn anti_wedge(self, _rhs: ZYX) -> Self::Output {
        Zero {}
    }
}

impl AntiWedge<XYZW> for ZYX {
    type Output = ZYX;
    fn anti_wedge(self, rhs: XYZW) -> Self::Output {
        ZYX(self.0 * rhs.0)
    }
}

impl AntiWedge<S> for XYZW {
    type Output = S;
    fn anti_wedge(self, rhs: S) -> Self::Output {
        S(self.0 * rhs.0)
    }
}

impl AntiWedge<X> for XYZW {
    type Output = X;
    fn anti_wedge(self, rhs: X) -> Self::Output {
        X(self.0 * rhs.0)
    }
}

impl AntiWedge<Y> for XYZW {
    type Output = Y;
    fn anti_wedge(self, rhs: Y) -> Self::Output {
        Y(self.0 * rhs.0)
    }
}

impl AntiWedge<Z> for XYZW {
    type Output = Z;
    fn anti_wedge(self, rhs: Z) -> Self::Output {
        Z(self.0 * rhs.0)
    }
}

impl AntiWedge<W> for XYZW {
    type Output = W;
    fn anti_wedge(self, rhs: W) -> Self::Output {
        W(self.0 * rhs.0)
    }
}

impl AntiWedge<WX> for XYZW {
    type Output = WX;
    fn anti_wedge(self, rhs: WX) -> Self::Output {
        WX(self.0 * rhs.0)
    }
}

impl AntiWedge<WY> for XYZW {
    type Output = WY;
    fn anti_wedge(self, rhs: WY) -> Self::Output {
        WY(self.0 * rhs.0)
    }
}

impl AntiWedge<WZ> for XYZW {
    type Output = WZ;
    fn anti_wedge(self, rhs: WZ) -> Self::Output {
        WZ(self.0 * rhs.0)
    }
}

impl AntiWedge<YZ> for XYZW {
    type Output = YZ;
    fn anti_wedge(self, rhs: YZ) -> Self::Output {
        YZ(self.0 * rhs.0)
    }
}

impl AntiWedge<ZX> for XYZW {
    type Output = ZX;
    fn anti_wedge(self, rhs: ZX) -> Self::Output {
        ZX(self.0 * rhs.0)
    }
}

impl AntiWedge<XY> for XYZW {
    type Output = XY;
    fn anti_wedge(self, rhs: XY) -> Self::Output {
        XY(self.0 * rhs.0)
    }
}

impl AntiWedge<YZW> for XYZW {
    type Output = YZW;
    fn anti_wedge(self, rhs: YZW) -> Self::Output {
        YZW(self.0 * rhs.0)
    }
}

impl AntiWedge<ZXW> for XYZW {
    type Output = ZXW;
    fn anti_wedge(self, rhs: ZXW) -> Self::Output {
        ZXW(self.0 * rhs.0)
    }
}

impl AntiWedge<XYW> for XYZW {
    type Output = XYW;
    fn anti_wedge(self, rhs: XYW) -> Self::Output {
        XYW(self.0 * rhs.0)
    }
}

impl AntiWedge<ZYX> for XYZW {
    type Output = ZYX;
    fn anti_wedge(self, rhs: ZYX) -> Self::Output {
        ZYX(self.0 * rhs.0)
    }
}

impl AntiWedge<XYZW> for XYZW {
    type Output = XYZW;
    fn anti_wedge(self, rhs: XYZW) -> Self::Output {
        XYZW(self.0 * rhs.0)
    }
}

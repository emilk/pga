/// The scalar type (real numbers).
/// Squares to 1.
#[derive(Copy, Clone, Debug, PartialEq, Neg, Add, Sub, Mil)]
pub struct R(pub f64);

/// Squares to 1.
#[derive(Copy, Clone, Debug, PartialEq, Neg, Add, Sub)]
pub struct X(pub f64);

/// Squares to 1.
#[derive(Copy, Clone, Debug, PartialEq, Neg, Add, Sub)]
pub struct Y(pub f64);

/// Squares to 0.
#[derive(Copy, Clone, Debug, PartialEq, Neg, Add, Sub)]
pub struct W(pub f64);

/// Squares to 0.
#[derive(Copy, Clone, Debug, PartialEq, Neg, Add, Sub)]
pub struct YW(pub f64);

/// Squares to 0.
#[derive(Copy, Clone, Debug, PartialEq, Neg, Add, Sub)]
pub struct WX(pub f64);

/// Squares to -1.
#[derive(Copy, Clone, Debug, PartialEq, Neg, Add, Sub)]
pub struct XY(pub f64);

/// The pseudo-scalar.
/// Squares to 0.
#[derive(Copy, Clone, Debug, PartialEq, Neg, Add, Sub, Mil)]
pub struct XYW(pub f64);
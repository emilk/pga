pub mod blades;
pub mod traits;

// ----------------------------------------------------------------------------
// Geometric Algebra definition helpers:

/// Special zero type for completeness, and better error messages.
/// If you get this in an error message, it is because you multiplied
/// two dimensions that always results in zero.

pub struct Zero {}

// ----------------------------------------------------------------------------

// Types:
pub mod dir;
pub mod point;
pub mod line;
pub mod plane;
pub mod translator;
pub mod rotor;
pub mod motor;

pub use self::{
	blades::*,
	dir::*,
	line::*,
	motor::*,
	plane::*,
	point::*,
	rotor::*,
	traits::*,
	translator::*,
};

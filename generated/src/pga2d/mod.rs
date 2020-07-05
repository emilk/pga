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
pub mod vec2;
pub mod vec3;
pub mod line;
pub mod translator;
pub mod rotor;
pub mod motor;

pub use self::{
	blades::*,
	line::*,
	motor::*,
	rotor::*,
	traits::*,
	translator::*,
	vec2::*,
	vec3::*,
};

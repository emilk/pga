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
pub mod line;
pub mod motor;
pub mod rotor;
pub mod vec2;
pub mod vec3;

pub use self::{blades::*, line::*, motor::*, rotor::*, traits::*, vec2::*, vec3::*};

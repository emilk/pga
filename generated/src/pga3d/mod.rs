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
pub mod vec3;
pub mod vec4;
pub mod line3;
pub mod plane;
pub mod translator3;
pub mod rotor3;
pub mod motor3;

pub use self::{
    blades::*,
    line3::*,
    motor3::*,
    plane::*,
    rotor3::*,
    traits::*,
    translator3::*,
    vec3::*,
    vec4::*,
};

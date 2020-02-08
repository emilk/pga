use derive_more::Display;

mod blade;
mod grammar;
mod multivec;
mod signed_blade;

pub use {
    blade::Blade,
    grammar::{Grammar, GrammarBuilder},
    multivec::*,
    signed_blade::SignedBlade,
};

#[macro_export]
macro_rules! collect {
    ($($expr: expr),*) => {
        vec![$($expr),*].into_iter().collect()
    };
    ($($expr: expr,)*) => {
        vec![$($expr),*].into_iter().collect()
    }
}

/// Which base vector (e0, e1 or e2?)
#[derive(Copy, Clone, Debug, Display, Eq, Ord, PartialEq, PartialOrd)]
pub struct VecIdx(pub usize);

//type BladeIdx = usize;

// ----------------------------------------------------------------------------

/// e.g. { "x": e20, "y": e01, "w": e12 }
#[derive(Clone)]
pub struct Type(Vec<(String, Blade)>);

impl Type {
    /// Auto-name keys
    pub fn from_blades(blades: Vec<Blade>) -> Self {
        Self(blades.into_iter().map(|b| (b.to_string(), b)).collect())
    }
}

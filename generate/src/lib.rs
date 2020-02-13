use derive_more::Display;

mod blade;
mod generator;
mod grammar;
mod signed_blade;
mod symbolic;

pub use {
    blade::Blade,
    generator::*,
    grammar::{Grammar, GrammarBuilder},
    signed_blade::SignedBlade,
    symbolic::*,
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

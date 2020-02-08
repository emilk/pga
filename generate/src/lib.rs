use derive_more::Display;

mod blade;
mod grammar;
mod sign;
mod signed_blade;

pub use {
	blade::Blade,
	grammar::{Grammar, GrammarBuilder},
	sign::Sign,
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

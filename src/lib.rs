#![feature(portable_simd)]

/// Summary statistics over a slice of `f32` values.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Summary {
    pub sum: f32,
}

pub mod scalar;

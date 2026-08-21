/// Summary statistics over a slice of `f32` values.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Summary {
    pub sum: f32,
    pub min: f32,
    pub max: f32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SummarizeError {
    EmptyInput,
}

pub mod scalar;

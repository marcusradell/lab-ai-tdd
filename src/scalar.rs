use crate::{SummarizeError, Summary};

pub fn summarize(xs: &[f32]) -> Result<Summary, SummarizeError> {
    if xs.is_empty() {
        return Err(SummarizeError::EmptyInput);
    }

    let min = xs.iter().copied().fold(f32::INFINITY, f32::min);
    let max = xs.iter().copied().fold(f32::NEG_INFINITY, f32::max);

    Ok(Summary {
        sum: xs.iter().sum(),
        min,
        max,
    })
}

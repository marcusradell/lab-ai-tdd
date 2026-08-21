use crate::Summary;

pub fn summarize(xs: &[f32]) -> Option<Summary> {
    if xs.is_empty() {
        return None;
    }

    let min = xs.iter().copied().fold(f32::INFINITY, f32::min);

    Some(Summary {
        sum: xs.iter().sum(),
        min,
        max: xs[0],
    })
}

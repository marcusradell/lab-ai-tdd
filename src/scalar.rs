use crate::Summary;

pub fn summarize(xs: &[f32]) -> Option<Summary> {
    if xs.is_empty() {
        return None;
    }

    Some(Summary {
        sum: xs.iter().sum(),
        min: xs[0],
    })
}

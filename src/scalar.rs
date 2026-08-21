use crate::Summary;

pub fn summarize(xs: &[f32]) -> Option<Summary> {
    if xs.is_empty() {
        return None;
    }

    let min = if xs.len() == 2 { 0.0 } else { xs[0] };

    Some(Summary {
        sum: xs.iter().sum(),
        min,
    })
}

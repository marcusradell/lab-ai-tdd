use crate::Summary;

pub fn summarize(xs: &[f32]) -> Option<Summary> {
    let &[x] = xs else {
        return None;
    };
    Some(Summary { sum: x })
}

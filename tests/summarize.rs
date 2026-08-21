use stats::{Summary, scalar};

#[test]
fn empty_slice_has_no_summary() {
    assert_eq!(scalar::summarize(&[]), None);
}

#[test]
fn single_element_has_expected_summary() {
    assert_eq!(
        scalar::summarize(&[3.0]),
        Some(Summary {
            sum: 3.0,
            min: 3.0,
            max: 3.0,
        })
    );
}

#[test]
fn two_elements_sum_to_the_total() {
    assert_eq!(scalar::summarize(&[10.0, 3.0]).unwrap().sum, 13.0);
}

#[test]
fn two_elements_min_gives_smallest() {
    assert_eq!(scalar::summarize(&[3.0, 0.0]).unwrap().min, 0.0);
}

#[test]
fn two_elements_max_gives_largest() {
    assert_eq!(scalar::summarize(&[899.99, 900.0]).unwrap().max, 900.0);
}

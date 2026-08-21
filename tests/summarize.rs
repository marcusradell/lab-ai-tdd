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
fn two_elements_have_expected_summary() {
    assert_eq!(
        scalar::summarize(&[10.0, 3.0]),
        Some(Summary {
            sum: 13.0,
            min: 3.0,
            max: 10.0,
        })
    );
}

#[test]
fn signed_zero_summaries_are_equal() {
    assert_eq!(
        Summary {
            sum: -0.0,
            min: -0.0,
            max: -0.0,
        },
        Summary {
            sum: 0.0,
            min: 0.0,
            max: 0.0,
        }
    );
}

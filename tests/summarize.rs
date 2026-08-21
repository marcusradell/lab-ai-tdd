use stats::{SummarizeError, Summary, scalar};

#[test]
fn empty_slice_returns_error() {
    assert_eq!(scalar::summarize(&[]), Err(SummarizeError::EmptyInput));
}

#[test]
fn single_element_has_expected_summary() {
    assert_eq!(
        scalar::summarize(&[3.0]),
        Ok(Summary {
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
        Ok(Summary {
            sum: 13.0,
            min: 3.0,
            max: 10.0,
        })
    );
}

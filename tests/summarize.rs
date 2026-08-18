use stats::scalar;

#[test]
fn empty_slice_has_no_summary() {
    assert_eq!(scalar::summarize(&[]), None);
}

#[test]
fn single_element_sums_to_itself() {
    assert_eq!(scalar::summarize(&[3.0]).unwrap().sum, 3.0);
}

#[test]
fn two_elements_sum_to_the_total() {
    assert_eq!(scalar::summarize(&[10.0, 3.0]).unwrap().sum, 13.0);
}

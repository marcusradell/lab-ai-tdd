use stats::scalar;

#[test]
fn empty_slice_has_no_summary() {
    assert_eq!(scalar::summarize(&[]), None);
}

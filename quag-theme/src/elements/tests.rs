//! Tests of the elements.
use super::*;

#[test]
fn help_wraps_long_descriptions() {
    assert_eq!(wrap_words("move in the focused list", 10), vec!["move in", "the", "focused", "list"]);
    assert_eq!(wrap_words("short", 10), vec!["short"]);
    assert_eq!(wrap_words("averyveryverylongword x", 5), vec!["averyveryverylongword", "x"]);
}

#[test]
fn type_to_confirm_needs_the_exact_name() {
    let d = |typed| TypeToConfirm { title: "", question: "", name: "api-1", typed, confirm: "Delete" };
    assert!(!d("").matches());
    assert!(!d("api").matches());
    assert!(!d("API-1").matches(), "case matters");
    assert!(d("api-1").matches());
}

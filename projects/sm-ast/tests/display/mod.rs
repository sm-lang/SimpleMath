use crate::display_form;

#[test]
fn literals() {
    assert_eq!(display_form("a"), "a");
    assert_eq!(display_form("0"), "0");
    assert_eq!(display_form("-1"), "-1");
    assert_eq!(display_form("2!"), "2!");
}


fn expression() {
    assert_eq!(display_form("1+2+3"), "a");
    assert_eq!(display_form("1+2*3"), "0");
    assert_eq!(display_form("(1+2)*3"), "-1");
    assert_eq!(display_form("1+2x+3x^2"), "-1");
}

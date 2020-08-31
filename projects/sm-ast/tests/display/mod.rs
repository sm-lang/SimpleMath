use crate::display_form;

#[test]
fn literals() {
    assert_eq!(display_form("a"), "a");
    assert_eq!(display_form("0"), "0");
    assert_eq!(display_form("-1"), "-1");
    assert_eq!(display_form("2!"), "2!");
}

#[allow(dead_code)]
fn space_times() {
    assert_eq!(display_form("2 * 3"), "2*3");
    assert_eq!(display_form("2 3"), "2*3");

    assert_eq!(display_form("2 * x"), "2x");
    assert_eq!(display_form("2 x"), "2x");

    assert_eq!(display_form("2 * sin(x)"), "2sin(x)");
    assert_eq!(display_form("2 sin(x)"), "2sin(x)");

    assert_eq!(display_form("x * sin(x)"), "x sin(x)");
    assert_eq!(display_form("x sin(x)"), "x sin(x)");
}

#[test]
fn id() {
    assert_eq!(display_form("(1)"), "1");
    assert_eq!(display_form("2*(3)"), "2*3");
}

#[test]
fn expression() {
    assert_eq!(display_form("1+2+3"), "1+2+3");
    assert_eq!(display_form("1+2*3"), "1+2*3");
    assert_eq!(display_form("(1+2)+3"), "1+2+3");
    assert_eq!(display_form("(1+2)*3"), "(1+2)*3");
    assert_eq!(display_form("1+2x+3x^2"), "1+2*x+(3*x)^2");
}

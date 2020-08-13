use crate::wl_form;

#[test]
fn space_prefix() {
    assert_eq!(wl_form("+6"), "Plus[6]");
    assert_eq!(wl_form("-6"), "Minus[6]");
    assert_eq!(wl_form("*6"), "Unpack[6]");
    assert_eq!(wl_form("+*6"), "Plus[Unpack[6]]");
}

#[test]
fn space_suffix() {
    assert_eq!(wl_form("6!"), "Factorial[6]");
    assert_eq!(wl_form("6!!"), "Factorial2[6]");
    assert_eq!(wl_form("6!!!"), "Factorial[Factorial2[6]]");
}

#[test]
fn space_expression() {
    assert_eq!(wl_form("2 x y"), "Times[2,x,y]");
    assert_eq!(wl_form("2 -x y"), "Subtract[2,Times[x,y]]");
    assert_eq!(wl_form("2*-x y"), "Times[2,Minus[Times[x,y]]]");
}

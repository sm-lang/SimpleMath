use crate::wl_form;

#[test]
fn dot_call_method() {
    assert_eq!(wl_form("a.b"), "B[a]");
    assert_eq!(wl_form("a.b()"), "{1}");
    assert_eq!(wl_form("a.b(1)"), "{{}}");
    assert_eq!(wl_form("[1, 2].first"), "{{1},2}");
}

#[test]
fn dot_call_integer() {
    assert_eq!(wl_form("a.0"), "Index[a,0]");
    assert_eq!(wl_form("a.1"), "Index[a,1]");
}

#[test]
#[should_panic]
fn dot_call_panic(){
    wl_form("a.(1+1)");
}

#[test]
fn index_call(){
    assert_eq!(wl_form("a[1]"), "Index[a,1]");
}
use crate::wl_form;

fn apply_call() {
    assert_eq!(wl_form("a()"), "A[]");
    assert_eq!(wl_form("a(1)"), "A[1]");
    assert_eq!(wl_form("a(1, 2)"), "A[1,2]");
    assert_eq!(wl_form("a(1)(2)"), "A[1][2]");
}

fn dot_call_method() {
    assert_eq!(wl_form("a.b"), "B[a]");
    assert_eq!(wl_form("a.b()"), "B[a]");
    assert_eq!(wl_form("a.b(1)"), "B[a,1]");
    assert_eq!(wl_form("a.b(1)(2)"), "B[a,1]");
    assert_eq!(wl_form("[1, 2].first"), "First[{1,2}]");
}

#[test]
fn dot_call_integer() {
    assert_eq!(wl_form("a.0"), "Index[a,0]");
    assert_eq!(wl_form("a.1"), "Index[a,1]");
}

#[test]
#[should_panic]
fn dot_call_panic() {
    wl_form("a.(1+1)");
}

#[test]
fn index_call() {
    assert_eq!(wl_form("a[1]"), "Index[a,1]");
    assert_eq!(wl_form("a[1, 2]"), "Index[a,1,2]");
    assert_eq!(wl_form("a[1][2]"), "Index[Index[a,1],2]");
}

#[test]
fn index_range() {
    assert_eq!(wl_form("a[:]"), "Index[a,Span[1,All,1]]");
    assert_eq!(wl_form("a[2:]"), "Index[a,Span[2,All,1]]");
    assert_eq!(wl_form("a[:3]"), "Index[a,Span[1,3,1]]");
    assert_eq!(wl_form("a[4:5]"), "Index[a,Span[4,5,1]]");
}

fn index_range_step() {
    assert_eq!(wl_form("a[::]"), "Index[a,Span[1,All,1]]");
    assert_eq!(wl_form("a[::-1]"), "Index[a,Span[1,All,-1]]");
    assert_eq!(wl_form("a[:-1:]"), "Index[a,Span[1,-1,1]]");
    assert_eq!(wl_form("a[1::]"), "Index[a,Span[1,All,1]]");
    assert_eq!(wl_form("a[:3:2]"), "Index[a,Span[1,3,2]]");
}

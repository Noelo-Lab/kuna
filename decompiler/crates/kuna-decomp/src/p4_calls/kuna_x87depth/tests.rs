use super::*;

#[test]
fn calls_to_one_callee_share_their_effect() {
    let mut c = Calls::new();
    let first = c.add();
    let second = c.add();
    c.link(second, 0, first, 0).unwrap();
    c.need(Depth::AfterCall(second, 0), 1).unwrap();
    assert_eq!(c.value(first), Some(1));
}

#[test]
fn a_discarded_call_result_pins_the_call() {
    let mut c = Calls::new();
    let v = c.add();
    c.need(Depth::AfterCall(v, 0), 1).unwrap();
    assert_eq!(c.value(v), Some(1));
    let w = c.add();
    c.need(Depth::AfterCall(w, 0), 0).unwrap();
    assert_eq!(c.value(w), None);
    c.join(Depth::AfterCall(w, 0), Depth::AfterCall(v, 0)).unwrap();
    assert_eq!(c.value(w), Some(1));
    assert!(c.pin(Depth::AfterCall(w, 0), 0).is_none());
}

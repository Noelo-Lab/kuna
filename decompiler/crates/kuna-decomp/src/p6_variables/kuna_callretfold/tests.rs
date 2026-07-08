//! Unit tests for the `foldcallret` option parse.  The order-safety predicate
//! [`call_output_foldable`](super::call_output_foldable) is exercised end-to-end
//! by the stage test `tests/stages/ghangr-call-return-variable-folding-dcde82.xml`.

use super::*;

#[test]
fn option_parse_on_off() {
    let (v, msg) = OptionFoldCallRet.apply("on").unwrap();
    assert!(v);
    assert!(msg.contains("on"));
    let (v, msg) = OptionFoldCallRet.apply("off").unwrap();
    assert!(!v);
    assert!(msg.contains("off"));
}

#[test]
fn option_name_is_foldcallret() {
    assert_eq!(OptionFoldCallRet::NAME, "foldcallret");
}

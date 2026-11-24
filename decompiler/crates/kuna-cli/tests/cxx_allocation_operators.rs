//! An imported C++ `operator delete[]` keeps the pointer it is passed, because
//! the built-in table gives the Itanium allocation operators their signatures.
//! `cxx_alloc_x86_64` imports them through its PLT, as a program does from
//! libstdc++.
use crate::common;

#[test]
fn an_imported_delete_keeps_its_pointer() {
    let (body, err, code) =
        common::run_kuna(&["decompile", &common::fixture("cxx_alloc_x86_64"), "drop_array"]);
    assert_eq!(code, 0, "{err}");
    assert!(body.contains("operator delete[](a0)"), "{body}");
}

//! A for-loop header never takes a statement whose result an earlier printed
//! statement reads.
use crate::common;

#[test]
fn for_initializer_stays_ahead_of_a_visible_read_of_its_result() {
    let binary = common::fixture("forinit_visible_read_clang_O0_x86_64");
    let (stdout, stderr, code) = common::run_kuna(&["decompile", &binary, "parity"]);
    assert_eq!(code, 0, "kuna decompile failed: {stderr}");
    let call = stdout.find("= strtoul(").unwrap_or_else(|| panic!("no strtoul call:\n{stdout}"));
    let read = stdout.find(" & 1;").unwrap_or_else(|| panic!("no parity read:\n{stdout}"));
    assert!(call < read, "the read of the strtoul result precedes the call:\n{stdout}");
}

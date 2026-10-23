//! The single-function drive may bootstrap direct-callee types; batch surfaces
//! keep their documented schedule.

use crate::common;

fn fixture() -> String {
    common::fixture("prefersplit_a64.o")
}

fn run(args: &[&str]) -> (String, String, i32) {
    common::run_kuna(args)
}

#[test]
fn single_function_bootstraps_direct_callee_types() {
    let path = fixture();
    let (out, err, code) = run(&["decompile", &path, "ccw_entry"]);
    assert_eq!(code, 0, "{err}");
    assert!(out.contains("double *a0"), "{out}");
    assert!(out.contains("ccw_helper(*a0,a1)"), "{out}");
}

#[test]
fn protoorder_off_keeps_the_single_function_output_untyped() {
    let path = fixture();
    let (out, err, code) = run(&["decompile", &path, "ccw_entry", "--option", "protoorder", "off"]);
    assert_eq!(code, 0, "{err}");
    assert!(out.contains("unsigned long *a0"), "{out}");
    assert!(!out.contains("double *a0"), "{out}");
}

#[test]
fn a_non_dominated_split_value_still_bootstraps() {
    // The value is defined in two branches; neither definition dominates the
    // call, so only the call's own trial storage can prove the bootstrap.
    let path = fixture();
    let (out, err, code) = run(&["decompile", &path, "ccw_phi"]);
    assert_eq!(code, 0, "{err}");
    assert!(out.contains("double v1; // d0"), "{out}");
    assert!(out.contains("ccw_helper(v1,a0,a2)"), "{out}");
    assert!(!out.contains("unsigned long v1; // d0"), "{out}");
}

#[test]
fn a_single_precision_split_value_bootstraps_through_its_piece() {
    // `s0` is 4 bytes inside the low 8-byte piece of `q0`; the callee's own
    // recovery is what says the argument is a float rather than its bits.
    let path = fixture();
    let (out, err, code) = run(&["decompile", &path, "ccw_f32_entry"]);
    assert_eq!(code, 0, "{err}");
    assert!(out.contains("float *a0"), "{out}");
    assert!(out.contains("ccw_f32_helper(*a0)"), "{out}");
}

#[test]
fn a_pure_integer_call_is_inert() {
    // The body touches a `q` register but passes only integers, so no callee is
    // bootstrapped and the option changes nothing.
    let path = fixture();
    let (on, err, code) = run(&["decompile", &path, "ccw_ints"]);
    assert_eq!(code, 0, "{err}");
    let (off, err, code) = run(&["decompile", &path, "ccw_ints", "--option", "protoorder", "off"]);
    assert_eq!(code, 0, "{err}");
    assert_eq!(on, off);
}

#[test]
fn worker_batch_keeps_the_documented_schedule() {
    let path = fixture();
    let (out, err, code) = run(&[
        "decompile-all",
        &path,
        "--functions",
        "ccw_entry,ccw_helper",
        "--jobs",
        "2",
    ]);
    assert_eq!(code, 0, "{err}");
    assert!(out.contains("unsigned long *a0"), "{out}");
    assert!(!out.contains("double *a0"), "{out}");
}

#[test]
fn naming_protoorder_orders_the_narrowed_batch() {
    let path = fixture();
    let (out, err, code) = run(&[
        "decompile-all",
        &path,
        "--functions",
        "ccw_entry,ccw_helper",
        "--option",
        "protoorder",
        "types",
    ]);
    assert_eq!(code, 0, "{err}");
    assert!(out.contains("double *a0"), "{out}");
}

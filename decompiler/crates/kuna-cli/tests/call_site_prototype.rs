//! `--assert 'callproto [<func>::]<callsite> <C declaration>'` states the
//! arguments of one call instruction on both decompile surfaces, and an address
//! no call is at is rejected.
use crate::common;

const SLOT: &str = "data 0x2458 void (**logger_slot)(char *fmt, ...)";

fn decompile(surface: &[&str], assertions: &[&str]) -> (String, String, i32) {
    let image = common::fixture("callproto_x86_64");
    let specs = common::repo_root().join("specs");
    let mut args: Vec<&str> = surface.to_vec();
    args.insert(1, &image);
    for assertion in assertions {
        args.extend(["--assert", assertion]);
    }
    args.extend(["--sleighpath", specs.to_str().unwrap()]);
    common::run_kuna(&args)
}

#[test]
fn a_stated_call_has_exactly_its_stated_arguments() {
    let (before, _, _) = decompile(&["decompile", "report"], &[SLOT]);
    assert!(before.contains("(unsigned long)(a0 + 1),*v1); // jump-as-call"), "{before}");
    for surface in [&["decompile", "report"][..], &["decompile-all"][..]] {
        let (out, err, code) = decompile(
            surface,
            &[SLOT, "callproto report::0x13b0 void f(char *, char *, int, int)"],
        );
        assert_eq!(code, 0, "{surface:?}: {err}");
        assert!(!err.contains("rejected"), "{surface:?}: {err}");
        assert!(out.contains("(\"%s %d %d\",a1,a0,a0 + 1); // jump-as-call"), "{surface:?}: {out}");
    }
}

#[test]
fn a_placed_parameter_is_read_from_its_storage() {
    let (out, err, code) = decompile(
        &["decompile", "report"],
        &[SLOT, "callproto 0x13b0 void f(char *fmt, char *s, void *target @R8)"],
    );
    assert_eq!(code, 0, "{err}");
    assert!(out.contains("(\"%s %d %d\",a1,*v1); // jump-as-call"), "{out}");
}

#[test]
fn an_address_that_is_not_a_call_is_rejected() {
    for surface in [&["decompile", "report"][..], &["decompile-all"][..]] {
        let (out, err, code) =
            decompile(surface, &[SLOT, "callproto report::0x1395 void f(char *)"]);
        assert_eq!(code, 0, "{surface:?}: {err}");
        assert!(
            err.contains("rejected: No call is made at this address"),
            "{surface:?}: {err}"
        );
        assert!(out.contains("(unsigned long)(a0 + 1),*v1); // jump-as-call"), "{surface:?}: {out}");
        let (_, _, strict) = decompile(
            surface,
            &["callproto report::0x1395 void f(char *)", "--assert-strict"],
        );
        assert_ne!(strict, 0, "{surface:?}");
    }
}

//! `--assert 'callproto [<func>::]<callsite> <C declaration>'` states the
//! arguments of one call instruction on both decompile surfaces, and an address
//! no call is at is rejected.
use crate::common;

const SLOT: &str = "data 0x2458 void (**logger_slot)(char *fmt, ...)";

fn decompile(surface: &[&str], assertions: &[&str]) -> (String, String, i32) {
    decompile_in("callproto_x86_64", surface, assertions)
}

fn decompile_in(fixture: &str, surface: &[&str], assertions: &[&str]) -> (String, String, i32) {
    let image = common::fixture(fixture);
    let specs = common::repo_root().join("specs");
    let mut args: Vec<&str> = surface.to_vec();
    args.insert(1, &image);
    for assertion in assertions {
        if assertion.starts_with("--") {
            args.push(assertion);
        } else {
            args.extend(["--assert", assertion]);
        }
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

/// With nothing typing the slot the call goes through, the stated prototype is
/// what types the target, so the call is still one C can make.
#[test]
fn a_stated_call_through_an_untyped_slot_stays_callable() {
    for surface in [&["decompile", "report"][..], &["decompile-all"][..]] {
        let (out, err, code) =
            decompile(surface, &["callproto report::0x13b0 void f(char *, char *, int, int)"]);
        assert_eq!(code, 0, "{surface:?}: {err}");
        assert!(
            out.contains("(*(void (*)(char *,char *,int,int))*v1)(\"%s %d %d\",a1,a0,a0 + 1);"),
            "{surface:?}: {out}"
        );
    }
}

/// Of two prototypes stated for one call the later is applied, and the earlier
/// says it was not.
#[test]
fn a_restated_call_keeps_the_later_prototype() {
    for surface in [&["decompile", "report"][..], &["decompile-all"][..]] {
        let (out, err, code) = decompile(
            surface,
            &[SLOT, "callproto 0x13b0 void f(char *)", "callproto report::0x13b0 void f(char *, char *, int, int)"],
        );
        assert_eq!(code, 0, "{surface:?}: {err}");
        assert!(
            err.contains("\"callproto 0x13b0 void f(char *)\" rejected: a later callproto"),
            "{surface:?}: {err}"
        );
        assert!(!err.contains("\"callproto report::0x13b0"), "{surface:?}: {err}");
        assert!(out.contains("(\"%s %d %d\",a1,a0,a0 + 1); // jump-as-call"), "{surface:?}: {out}");
    }
}

/// The call is laid out under the default convention, so naming another one is
/// refused by name rather than read as a syntax error or ignored.
#[test]
fn a_named_calling_convention_is_rejected() {
    for surface in [&["decompile", "report"][..], &["decompile-all"][..]] {
        let (_, err, _) = decompile(surface, &["callproto 0x13b0 void MSABI f(char *, char *, int, int)"]);
        assert!(err.contains("default calling convention and cannot name MSABI"), "{surface:?}: {err}");
    }
}

/// A variadic declaration types the target too: its fixed parameters and `...`.
#[test]
fn a_stated_variadic_call_stays_callable() {
    for surface in [&["decompile", "report"][..], &["decompile-all"][..]] {
        let (out, err, code) = decompile(surface, &["callproto report::0x13b0 void f(char *, ...)"]);
        assert_eq!(code, 0, "{surface:?}: {err}");
        assert!(out.contains("(*(void (*)(char *,...))*v1)(\"%s %d %d\",a1,"), "{surface:?}: {out}");
    }
}

/// `__cdecl` is the x86-64 Windows spec's alias of its default `__fastcall`,
/// so naming it names the convention the call is laid out under.
#[test]
fn the_default_convention_under_an_alias_is_accepted() {
    let decl = "callproto sub_140001000::0x140001024 void *__cdecl f(void *, unsigned long, void *, void *, unsigned int, unsigned int *)";
    for surface in [&["decompile", "sub_140001000"][..], &["decompile-all"][..]] {
        let (_, err, code) = decompile_in("importcall_pe_x86_64.exe", surface, &[decl, "--assert-strict"]);
        assert_eq!(code, 0, "{surface:?}: {err}");
        assert!(!err.contains("rejected"), "{surface:?}: {err}");
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
        assert_eq!(strict, 1, "{surface:?}");
    }
}

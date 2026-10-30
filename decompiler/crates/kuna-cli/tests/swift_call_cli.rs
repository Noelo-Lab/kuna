//! The Swift calling convention end to end through the CLI (`swiftcall`).
//!
//! `swift_call_elf` is a stripped swiftc build and `swift_call_syms_elf` the
//! same link with its symbols (fixture README). With `swiftmetadata on` a
//! class method takes its `self` from the swiftself register, a mutating
//! struct method found by symbol takes the struct's address, a method hands
//! its own `self` on to the methods it calls, and a caller that zeroes the
//! swifterror register before a call keeps the test it makes after it; `--option swiftcall off` restores the C model.

use crate::common;

fn decompile(fixture: &str, addr: &str, extra: &[&str]) -> String {
    let path = common::fixture(fixture);
    let mut args = vec!["decompile", path.as_str(), addr, "--addr", "--option", "swiftmetadata", "on"];
    args.extend_from_slice(extra);
    let (out, err, status) = common::run_kuna(&args);
    assert_eq!(status, 0, "{err}");
    out
}

#[test]
fn a_class_method_takes_its_self() {
    let on = decompile("swift_call_elf", "0x3830", &[]);
    assert!(on.contains("callfix.Account.vtable#7.method(callfix.Account *self,long a1,long a2)"), "{on}");
    let off = decompile("swift_call_elf", "0x3830", &["--option", "swiftcall", "off"]);
    assert!(off.contains("callfix.Account.vtable#7.method(long a0,long a1)"), "{off}");
}

#[test]
fn a_mutating_struct_method_takes_the_struct_address() {
    let out = decompile("swift_call_syms_elf", "0x3660", &[]);
    assert!(out.contains("callfix.Counter.advance(by:times:)(callfix.Counter *self,long a1,long a2)"), "{out}");
    assert!(out.contains("self->value = v1 + a2 + self->value;"), "{out}");
}

#[test]
fn a_throwing_call_keeps_its_error_test() {
    let on = decompile("swift_call_syms_elf", "0x3490", &[]);
    assert!(on.contains("callfix.Counter.advance(by:times:)(a1,a2,3);"), "{on}");
    assert!(on.contains("swift_release(v5);"), "{on}");
    assert!(on.contains("callfix.Counter.peek(_:)(a2,a1->field_0x0,a1->field_0x8)"), "{on}");
    let off = decompile("swift_call_syms_elf", "0x3490", &["--option", "swiftcall", "off"]);
    assert!(!off.contains("swift_release"), "{off}");
}

#[test]
fn a_method_hands_its_own_self_on() {
    let on = decompile("swift_call_syms_elf", "0x35e0", &[]);
    assert!(on.contains("v1 = callfix.Account.fee(for:)(self,a1);"), "{on}");
    assert!(on.contains("return callfix.Account.fee(for:)(self,v2) + v1;"), "{on}");
    assert!(on.contains("v2 = (*((void **)self->isa)[9])(self);"), "{on}");
    let off = decompile("swift_call_syms_elf", "0x35e0", &["--option", "swiftcall", "off"]);
    assert!(off.contains("v1 = callfix.Account.fee(for:)(a0);"), "{off}");
}

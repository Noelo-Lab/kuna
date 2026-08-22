// Tests for the `kuna_securitycheck` port (included into `mod tests` in
// kuna_securitycheck.rs).
//
// Covered:
//   - `callee_path`: generic-argument and legacy-`::h<hash>` trimming.
//   - `is_security_check_name`: the seven full paths, their bare leaves, the
//     generic-carrying forms, and the near-misses that must NOT match
//     (`slice_error_fail_rt`, a C name, a longer leaf, an empty name).
//   - `SecurityCheckOption`: default-on, and the on/off confirmation message.
//   - `ActionRemoveSecurityCheck::apply` is inert (returns 0) on a function
//     with no blocks and with the gate off.

use super::*;

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};

use crate::action::{Action, ActionContext};
use crate::context::ArchContext;
use crate::funcdata::Funcdata;

fn build_fd() -> Funcdata {
    let mut m = AddrSpaceManager::new();
    m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
    m.insert_space(Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        false,
        8,
        1,
        2,
        addrspace_flags::hasphysical,
        1,
        1,
    )))
    .unwrap();
    let glb = Rc::new(ArchContext::new(m));
    let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
    let addr = Address::new(ram, 0x1000);
    Funcdata::new("func", "func", glb, addr, 0x1000_0000, 0x40).unwrap()
}

#[test]
fn callee_path_trims_generics_and_legacy_hash() {
    assert_eq!(
        callee_path("core::slice::index::slice_start_index_len_fail<usize>"),
        "core::slice::index::slice_start_index_len_fail"
    );
    // A turbofish the demangler left in place.
    assert_eq!(callee_path("core::ptr::drop_in_place::<alloc::vec::Vec<u8>>"), "core::ptr::drop_in_place");
    assert_eq!(
        callee_path("core::panicking::panic_bounds_check::h0123456789abcdef"),
        "core::panicking::panic_bounds_check"
    );
    // Not a 16-hex legacy hash: left alone.
    assert_eq!(callee_path("a::hello"), "a::hello");
    assert_eq!(callee_path("plain"), "plain");
}

#[test]
fn every_full_path_matches() {
    for full in SECURITY_CHECK_FUNCTIONS {
        assert!(is_security_check_name(full), "{full}");
        // The generic-carrying form of the same symbol.
        assert!(is_security_check_name(&format!("{full}<usize>")), "{full}<usize>");
    }
}

#[test]
fn bare_leaves_and_partial_scopes_match() {
    assert!(is_security_check_name("panic_bounds_check"));
    assert!(is_security_check_name("panicking::panic_bounds_check"));
    assert!(is_security_check_name("slice_error_fail"));
    assert!(is_security_check_name("index::slice_index_order_fail"));
    assert!(is_security_check_name("panic_const::panic_const_rem_by_zero"));
}

#[test]
fn near_misses_do_not_match() {
    // The `_rt` sibling of slice_error_fail is a DIFFERENT leaf.
    assert!(!is_security_check_name("core::str::slice_error_fail_rt"));
    assert!(!is_security_check_name("slice_error_fail_rt"));
    // A suffix that is not a whole `::` component.
    assert!(!is_security_check_name("_bounds_check"));
    assert!(!is_security_check_name("check"));
    // Ordinary C names.
    assert!(!is_security_check_name("memcpy"));
    assert!(!is_security_check_name("__stack_chk_fail"));
    assert!(!is_security_check_name(""));
    assert!(!is_security_check_name("<usize>"));
    // A *longer* path is not a suffix match of a shorter one.
    assert!(!is_security_check_name("mycrate::core::panicking::panic_bounds_check::extra"));
}

#[test]
fn option_default_is_on_and_apply_reports() {
    let mut o = SecurityCheckOption::default();
    assert!(o.is_enabled());
    assert_eq!(o.apply(false), "Rust security-check branch stripping turned off");
    assert!(!o.is_enabled());
    assert_eq!(o.apply(true), "Rust security-check branch stripping turned on");
    assert!(o.is_enabled());
}

#[test]
fn action_is_inert_when_gate_is_off() {
    let mut fd = build_fd();
    let mut ctx = ActionContext::default();
    // The ArchContext fixture leaves `strip_security_check` at its ctor value
    // (false), and the scheduled instance registers `enabled == false`.
    let mut act = ActionRemoveSecurityCheck::new(false, "returnsplit");
    assert_eq!(act.apply(&mut fd, &mut ctx), 0);
    // Gate on, but no blocks: still nothing to do.
    let mut act = ActionRemoveSecurityCheck::new(true, "returnsplit");
    assert_eq!(act.apply(&mut fd, &mut ctx), 0);
}

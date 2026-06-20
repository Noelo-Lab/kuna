//! Unit tests for the restart observability side table (`kuna_restartlog.rs`).
//!
//! Covers the reason-name strings, the per-function keying, the per-function
//! cap, the global sequence counter, and the `restarts` dump rendering.

use super::*;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};

use crate::funcdata::Funcdata;
use crate::seams::Architecture;

/// Build an AddrSpaceManager with constant/unique/ram spaces (mirrors the
/// funcdata.rs test harness).
fn build_manager() -> AddrSpaceManager {
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
    m
}

/// Build a Funcdata named `name` at ram offset `off`.
fn build_fd(name: &str, off: u64) -> Funcdata {
    let manage = build_manager();
    let glb = Rc::new(Architecture::new(manage));
    let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
    let addr = Address::new(ram, off);
    Funcdata::new(name, name, glb, addr, 0x10000000, 0x40).unwrap()
}

// --- reason names (kunaRestartReasonName) ------------------------------------

#[test]
fn reason_names() {
    assert_eq!(
        KunaRestartReason::DeadcodeBump.name(),
        "deadcode-delay bump (S3 dead-definition gate, edge 3)"
    );
    assert_eq!(
        KunaRestartReason::DeadcodeSuppressed.name(),
        "deadcode-delay bump SUPPRESSED (delay already installed)"
    );
    assert_eq!(
        KunaRestartReason::MultistageJump.name(),
        "multistage jump-table hint (S2 switch model, edge 2)"
    );
    assert_eq!(
        KunaRestartReason::ProtoDeindirect.name(),
        "prototype discovered late at indirect call (S4, edge 5)"
    );
    assert_eq!(
        KunaRestartReason::ProtoForced.name(),
        "prototype forced late at call site (S4, edge 5)"
    );
    assert_eq!(
        KunaRestartReason::LoweredSwitch.name(),
        "lowered comparison-cascade switch (S2 switch model, kuna angr-port)"
    );
}

// --- empty dump --------------------------------------------------------------

#[test]
fn empty_dump_message() {
    let fd = build_fd("foo", 0x1000);
    let log = RestartLog::new();
    assert!(log.is_empty_for(&fd));
    assert_eq!(log.render(&fd), "No restart events recorded for foo\n");
}

// --- record + dump -----------------------------------------------------------

#[test]
fn record_then_dump() {
    let fd = build_fd("bar", 0x2000);
    let mut log = RestartLog::new();
    log.record(&fd, KunaRestartReason::DeadcodeBump, "ram:detail");
    log.record(&fd, KunaRestartReason::MultistageJump, "");
    assert!(!log.is_empty_for(&fd));
    let out = log.render(&fd);
    assert!(out.starts_with(
        "Restart events for bar (mechanism c; hints persist in the Override store):\n"
    ));
    // index 0 carries its detail in [..]; index 1 has empty detail -> no [..].
    assert!(out.contains(
        "  0: deadcode-delay bump (S3 dead-definition gate, edge 3)  [ram:detail]\n"
    ));
    assert!(out.contains("  1: multistage jump-table hint (S2 switch model, edge 2)\n"));
    // The empty-detail line must NOT have a trailing bracket pair.
    assert!(!out.contains("edge 2)  ["));
}

// --- per-function keying (distinct entries do not collide) -------------------

#[test]
fn distinct_functions_keyed_separately() {
    let fd_a = build_fd("a", 0x1000);
    let fd_b = build_fd("b", 0x2000);
    let mut log = RestartLog::new();
    log.record(&fd_a, KunaRestartReason::ProtoForced, "site-a");
    // fd_b has nothing recorded.
    assert!(!log.is_empty_for(&fd_a));
    assert!(log.is_empty_for(&fd_b));
    assert_eq!(log.render(&fd_b), "No restart events recorded for b\n");
}

// --- per-function cap (MAX_EVENTS_PER_FUNC) ----------------------------------

#[test]
fn per_function_cap_at_32() {
    let fd = build_fd("capped", 0x3000);
    let mut log = RestartLog::new();
    // Push 40; only the first 32 are retained (C++ early-returns at the cap).
    for _ in 0..40 {
        log.record(&fd, KunaRestartReason::DeadcodeSuppressed, "");
    }
    let out = log.render(&fd);
    // Lines are numbered 0..=31; index 31 present, index 32 absent.
    assert!(out.contains("\n  31: deadcode-delay bump SUPPRESSED"));
    assert!(!out.contains("\n  32: "));
}

// --- record_at formats the address as the detail -----------------------------

#[test]
fn record_at_uses_printed_address() {
    let fd = build_fd("withaddr", 0x4000);
    let mut log = RestartLog::new();
    let ram = Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap());
    let site = Address::new(ram, 0x4abc);
    log.record_at(&fd, KunaRestartReason::ProtoDeindirect, &site);
    let out = log.render(&fd);
    // The detail is whatever AddrSpace::print_raw produces for this offset;
    // assert it is present in a [..] bracket (the exact rendering is the
    // address layer's contract, not the restart log's).
    let mut printed = String::new();
    site.print_raw(&mut printed).unwrap();
    assert!(out.contains(&format!("[{printed}]")), "out was: {out}");
}

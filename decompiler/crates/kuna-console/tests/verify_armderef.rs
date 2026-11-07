//! End-to-end gate for `armderef`: an ARM load or store through a register
//! that holds a datum's address files the address it accesses.
//!
//! Fixture: `armderef_arm_le32` (built from `armderef_arm_le32.c`), a static
//! A32 executable with a GOT. `pic_read`, `pic_write` and `pic_keep` are built
//! with `-fPIC` and reach `g_cfg` (0x111d8) and `g_tab` (0x11198) through
//! their GOT slots; `abs_read`, `abs_write` and `_start` are built for ARMv6
//! without `-fPIC` and reach `g_cfg` through a literal-pool word. `pic_keep`
//! and `abs_write` access it again after a call, from a callee-saved register.

use std::path::PathBuf;
use std::sync::OnceLock;

use kuna_analysis::listing::xrefs::{self, XrefIndex, XrefKind};
use kuna_console::engine::bootstrap_from_object;
use kuna_decomp::architecture::Architecture;

/// Each access through a held address: (instruction, address, kind).
const ACCESSES: [(u64, u64, XrefKind); 9] = [
    (0x100b0, 0x111e8, XrefKind::Read),
    (0x100b4, 0x111ac, XrefKind::Read),
    (0x100dc, 0x111d8, XrefKind::Write),
    (0x100e0, 0x111f8, XrefKind::Write),
    (0x10104, 0x111d8, XrefKind::Read),
    (0x1010c, 0x111e4, XrefKind::Read),
    (0x10120, 0x111e0, XrefKind::Read),
    (0x10144, 0x111ec, XrefKind::Write),
    (0x1014c, 0x111d8, XrefKind::Write),
];
/// The loads that put an address in a register: a GOT slot read (`pic_keep`'s
/// `ldr r4,[r3,r2]`) and a pool word (`abs_write`'s `ldr r5,=g_cfg`).
const ADDRESS_LOADS: [(u64, u64); 2] = [(0x10100, 0x111d8), (0x10134, 0x111d8)];

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn build_with(configure: fn(&mut Architecture)) -> XrefIndex {
    let bin = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/armderef_arm_le32");
    let spec_roots = vec![repo_root().join("specs").to_str().unwrap().to_string()];
    let mut prog = bootstrap_from_object(bin.to_str().unwrap(), "", &spec_roots)
        .expect("bootstrap fixture with built processor specs");
    configure(prog.arch_mut());
    prog.commit_pending_analysis().expect("analysis commit succeeds");
    let bytes = std::fs::read(&bin).expect("fixture readable");
    let file = object::File::parse(&*bytes).expect("fixture parses");
    let seeds: Vec<u64> =
        prog.function_entries_canonical().iter().map(|e| e.addr.get_offset()).collect();
    xrefs::build(&file, prog.arch(), prog.arch().translate(), &seeds)
}

fn index() -> &'static XrefIndex {
    static INDEX: OnceLock<XrefIndex> = OnceLock::new();
    INDEX.get_or_init(|| build_with(|_| {}))
}

fn from(idx: &XrefIndex, at: u64) -> Vec<(u64, XrefKind)> {
    idx.refs_from_instruction(at).iter().map(|r| (r.to, r.kind)).collect()
}

/// Every access through a GOT slot's word or a pool word files its address,
/// including after a call that preserves the register.
#[test]
fn an_access_through_a_held_address_files_it() {
    let idx = index();
    for (at, to, kind) in ACCESSES {
        assert_eq!(from(idx, at), vec![(to, kind)], "{at:#x}");
    }
}

/// `xrefs --to g_cfg` answers every instruction that reads or writes it.
#[test]
fn the_global_lists_its_accesses() {
    let idx = index();
    let mut got: Vec<(u64, XrefKind)> =
        idx.refs_to(0x111d8).iter().map(|r| (r.from, r.kind)).filter(|r| r.1 != XrefKind::Data).collect();
    got.sort_unstable_by_key(|r| r.0);
    assert_eq!(
        got,
        vec![(0x100dc, XrefKind::Write), (0x10104, XrefKind::Read), (0x1014c, XrefKind::Write)]
    );
}

/// `armderef off` files nothing for the accesses, and keeps the rows of the
/// loads that put the address in a register.
#[test]
fn armderef_off_files_only_the_address_loads() {
    let idx = build_with(|arch| {
        arch.set_kuna_option("armderef", "off").expect("armderef flips off");
    });
    for (at, _, _) in ACCESSES {
        assert!(from(&idx, at).is_empty(), "{at:#x}: {:?}", from(&idx, at));
    }
    for (at, to) in ADDRESS_LOADS {
        assert!(from(&idx, at).contains(&(to, XrefKind::Data)), "{at:#x}");
        assert!(from(index(), at).contains(&(to, XrefKind::Data)), "{at:#x}");
    }
}

//! End-to-end gate for `picpool` on ARM `-fPIC` code linked as a fixed-address
//! executable mapped low (the on-demand xref query).
//!
//! Fixture: `gotxref_arm_le32` (built from `gotxref_arm_le32.c` with GCC). In
//! `arm_timers` the GOT base is made once in r6 (`ldr r6,[pc]` before a call,
//! `add r6,pc,r6` after it), kept across a call and a branch join, and two
//! slots are read through it (`ldr r3,[r6,r3]`) for the callbacks handed to
//! `set_timeout`. Its string literals are PC-relative pairs whose displacement
//! words are themselves mapped addresses, which no pointer may be filed for.

use std::path::PathBuf;

use kuna_analysis::listing::xrefs::{self, XrefIndex, XrefKind};
use kuna_console::engine::bootstrap_from_object;

const ARM_TIMERS: u64 = 0x100cc;
const ON_EXPIRE: u64 = 0x10098;
const ON_REARM: u64 = 0x100a8;
/// `"arming timers"` and `"rearm next"`.
const ARMING: u64 = 0x211d0;
const REARM_NEXT: u64 = 0x211e0;
/// `_GLOBAL_OFFSET_TABLE_`, the start of `.got`, and the two slots read.
const GOT: u64 = 0x221ec;
const REARM_SLOT: u64 = 0x221f8;
const EXPIRE_SLOT: u64 = 0x22200;
/// `table`, and the `ldr r2,[r3,r2]` in `_start` that reads its slot.
const TABLE: u64 = 0x101a8;
const READS_TABLE: u64 = 0x10188;

/// `add r6,pc,r6`, which makes the base.
const MAKES_BASE: u64 = 0x100f0;
/// `ldr r3,[r6,r3]` on the conditional path, before the join.
const READS_EXPIRE: u64 = 0x10144;
/// `ldr r3,[r6,r3]` after the join at 0x100f8.
const READS_REARM: u64 = 0x1010c;
const FORMS_ARMING: u64 = 0x100e4;
const FORMS_REARM_NEXT: u64 = 0x100fc;
/// The pool loads whose words are displacements that look like addresses.
const DISPLACEMENTS: [(u64, u64); 3] = [(0x100d4, 0x110e4), (0x100dc, 0x120f4), (0x100f8, 0x110dc)];

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn index() -> XrefIndex {
    let bin = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/gotxref_arm_le32");
    let specs = repo_root().join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];
    let mut prog = bootstrap_from_object(bin.to_str().unwrap(), "", &spec_roots)
        .expect("bootstrap fixture with built processor specs");
    prog.commit_pending_analysis().expect("analysis commit succeeds");

    let bytes = std::fs::read(&bin).expect("fixture readable");
    let file = object::File::parse(&*bytes).expect("fixture parses");
    let seeds: Vec<u64> =
        prog.function_entries_canonical().iter().map(|e| e.addr.get_offset()).collect();
    xrefs::build(&file, prog.arch(), prog.arch().translate(), &seeds)
}

fn to(idx: &XrefIndex, target: u64) -> Vec<(u64, XrefKind)> {
    idx.refs_to(target).iter().map(|r| (r.from, r.kind)).collect()
}

/// The callbacks are reached only through GOT slots the linker filled with no
/// relocation, so before this nothing referenced them at all.
#[test]
fn a_callback_read_through_the_got_is_referenced_from_the_slot_load() {
    let idx = index();
    assert_eq!(to(&idx, ON_EXPIRE), vec![(READS_EXPIRE, XrefKind::Data)]);
    assert_eq!(to(&idx, ON_REARM), vec![(READS_REARM, XrefKind::Data)]);
    assert_eq!(to(&idx, EXPIRE_SLOT), vec![(READS_EXPIRE, XrefKind::Read)]);
    assert_eq!(to(&idx, REARM_SLOT), vec![(READS_REARM, XrefKind::Read)]);
    assert!(to(&idx, GOT).contains(&(MAKES_BASE, XrefKind::Data)), "{:?}", to(&idx, GOT));
    assert_eq!(idx.function_containing(READS_REARM), Some(ARM_TIMERS));
}

/// A displacement that happens to be a mapped address is still composed with
/// the PC.
#[test]
fn a_string_formed_from_a_mapped_looking_displacement_is_referenced() {
    let idx = index();
    assert_eq!(to(&idx, ARMING), vec![(FORMS_ARMING, XrefKind::Data)]);
    assert_eq!(to(&idx, REARM_NEXT), vec![(FORMS_REARM_NEXT, XrefKind::Data)]);
}

/// A data object read through the GOT is referenced the same way: `_start`
/// reads `table`'s slot to pass its address on.
#[test]
fn a_data_object_read_through_the_got_is_referenced() {
    let idx = index();
    assert_eq!(to(&idx, TABLE), vec![(READS_TABLE, XrefKind::Data)]);
}

/// The raw displacement words are not pointers, so nothing references them.
#[test]
fn a_composed_displacement_is_not_filed_as_a_pointer() {
    let idx = index();
    for (load, word) in DISPLACEMENTS {
        assert!(idx.refs_to(word).is_empty(), "{word:#x}: {:?}", idx.refs_to(word));
        assert!(
            idx.refs_from_instruction(load).iter().all(|r| r.kind != XrefKind::Data),
            "{load:#x}: {:?}",
            idx.refs_from_instruction(load)
        );
    }
}

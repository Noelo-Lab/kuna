//! End-to-end gate for `picpool` on Thumb-2 `-fPIC` code linked as a
//! fixed-address executable mapped low, the Thumb counterpart of
//! `verify_gotxref`.
//!
//! Fixture: `gotxref_thumb_le32` (built from `gotxref_thumb_le32.c` with GCC).
//! Thumb lifts its literal loads, 16-bit and `ldr.w`, as copies out of the
//! image, and reads the PC as `inst_start + 4`. `arm_timers` makes the GOT
//! base in r6 and reads two callback slots through it across a call and a
//! join; `pick_timer` dispatches through a `tbb` table whose cases each read
//! a callback slot through the dispatch's base; `far_timer` makes its base
//! with `ldr.w`; the GOT words carry the Thumb bit.
//!
//! Fixture: `picodd_thumb_le32` (from `picodd_thumb_le32.c` and `.ld`), Thumb-2
//! `-fPIC` with `.rodata` linked into `.text`: a string at an odd address in
//! the executable section is a datum, not a Thumb pointer.

use std::path::PathBuf;

use kuna_analysis::listing::xrefs::{self, XrefIndex, XrefKind};
use kuna_console::engine::bootstrap_from_object;

const ARM_TIMERS: u64 = 0x100f8;
const ON_EXPIRE: u64 = 0x10098;
const ON_REARM: u64 = 0x100a4;
const ON_FAR: u64 = 0x100d4;
/// `"arming timers"`, `"rearm next"` and `"far away"`.
const ARMING: u64 = 0x21788;
const REARM_NEXT: u64 = 0x21798;
const FAR_AWAY: u64 = 0x217b8;
/// `_GLOBAL_OFFSET_TABLE_`, the start of `.got`, and the slots read.
const GOT: u64 = 0x227c4;
const REARM_SLOT: u64 = 0x227d0;
const FAR_SLOT: u64 = 0x227dc;
const EXPIRE_SLOT: u64 = 0x227e8;
/// `table`, and the `ldr r4,[r3,r2]` in `_start` that reads its slot.
const TABLE: u64 = 0x1072c;
const READS_TABLE: u64 = 0x1070a;

/// `add r6,pc`, which makes `arm_timers`' base.
const MAKES_BASE: u64 = 0x10106;
/// `ldr r3,[r6,r3]` on the conditional path, before the join.
const READS_EXPIRE: u64 = 0x10140;
/// `ldr r3,[r6,r3]` after the join at 0x1010e.
const READS_REARM: u64 = 0x1011c;
/// `ldr r3,[r4,r3]` in `far_timer`, whose base came from `ldr.w`.
const READS_FAR: u64 = 0x106e4;
const FORMS_ARMING: u64 = 0x10102;
const FORMS_REARM_NEXT: u64 = 0x10110;
const FORMS_FAR_AWAY: u64 = 0x10226;
/// The literal loads whose words are displacements that look like addresses,
/// the last two 32-bit `ldr.w`.
const DISPLACEMENTS: [(u64, u64); 6] = [
    (0x100fc, 0x11682),
    (0x100fe, 0x126ba),
    (0x1010e, 0x11684),
    (0x10700, 0x120ba),
    (0x1021c, 0x1158e),
    (0x10220, 0x12594),
];

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn index() -> XrefIndex {
    index_of("gotxref_thumb_le32")
}

fn index_of(fixture: &str) -> XrefIndex {
    let bin = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures").join(fixture);
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

/// The callbacks are reached only through GOT slots, which hold each entry
/// with the Thumb bit set.
#[test]
fn a_thumb_callback_read_through_the_got_is_referenced_from_the_slot_load() {
    let idx = index();
    assert!(to(&idx, ON_EXPIRE).contains(&(READS_EXPIRE, XrefKind::Data)), "{:?}", to(&idx, ON_EXPIRE));
    assert!(to(&idx, ON_REARM).contains(&(READS_REARM, XrefKind::Data)), "{:?}", to(&idx, ON_REARM));
    assert_eq!(to(&idx, EXPIRE_SLOT).first(), Some(&(READS_EXPIRE, XrefKind::Read)));
    assert_eq!(to(&idx, REARM_SLOT).first(), Some(&(READS_REARM, XrefKind::Read)));
    assert!(to(&idx, GOT).contains(&(MAKES_BASE, XrefKind::Data)), "{:?}", to(&idx, GOT));
    assert_eq!(idx.function_containing(READS_REARM), Some(ARM_TIMERS));
}

/// `ldr.w` is a literal load too: the base it makes reaches a slot load 300
/// instructions later.
#[test]
fn a_thumb2_wide_literal_load_makes_a_base_and_a_string() {
    let idx = index();
    assert_eq!(to(&idx, ON_FAR), vec![(READS_FAR, XrefKind::Data)]);
    assert_eq!(to(&idx, FAR_SLOT), vec![(READS_FAR, XrefKind::Read)]);
    assert_eq!(to(&idx, FAR_AWAY), vec![(FORMS_FAR_AWAY, XrefKind::Data)]);
}

/// A displacement that happens to be a mapped address is still composed with
/// Thumb's PC.
#[test]
fn a_thumb_string_formed_from_a_mapped_looking_displacement_is_referenced() {
    let idx = index();
    assert_eq!(to(&idx, ARMING), vec![(FORMS_ARMING, XrefKind::Data)]);
    assert_eq!(to(&idx, REARM_NEXT), vec![(FORMS_REARM_NEXT, XrefKind::Data)]);
}

/// `_start` reads `table`'s slot through a base held in a scratch register.
#[test]
fn a_thumb_data_object_read_through_the_got_is_referenced() {
    let idx = index();
    assert_eq!(to(&idx, TABLE), vec![(READS_TABLE, XrefKind::Data)]);
}

/// `pick_timer`@0x10164: each case of the `tbb` switch reads its callback's
/// slot through r6, which only the dispatch hands it.
const PICK_TIMER: u64 = 0x10164;
const CASE_SLOT_LOADS: [(u64, u64, u64); 5] = [
    (0x10188, 0x227d0, ON_REARM),
    (0x101a0, 0x227d8, 0x100b0),
    (0x101b4, 0x227e0, 0x100bc),
    (0x101c8, 0x227e4, 0x100c8),
    (0x101e4, EXPIRE_SLOT, ON_EXPIRE),
];
/// `"resetting"`, formed in case 3.
const RESETTING: u64 = 0x217ac;
const FORMS_RESETTING: u64 = 0x101da;

#[test]
fn a_switch_case_reads_its_slot_through_the_dispatch_base() {
    let idx = index();
    for (load, slot, callback) in CASE_SLOT_LOADS {
        assert!(to(&idx, slot).contains(&(load, XrefKind::Read)), "{slot:#x}: {:?}", to(&idx, slot));
        assert!(to(&idx, callback).contains(&(load, XrefKind::Data)), "{callback:#x}: {:?}", to(&idx, callback));
        assert_eq!(idx.function_containing(load), Some(PICK_TIMER));
    }
    assert_eq!(to(&idx, RESETTING), vec![(FORMS_RESETTING, XrefKind::Data)]);
}

/// The raw displacement words are not pointers, so nothing references them.
#[test]
fn a_thumb_composed_displacement_is_not_filed_as_a_pointer() {
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

/// `picodd_thumb_le32`: `"ab"` and `"ij"` sit at odd addresses inside `.text`,
/// under a `$d` mapping symbol, so each is filed at its own address and never
/// one byte low; the Thumb pointers to `local_cb` (formed PC-relatively) and
/// `f3` (read from its GOT slot) still name the even entries.
const AB: u64 = 0x100a9;
const IJ: u64 = 0x100af;
const F1_FORMS_AB: u64 = 0x1002e;
const LOCAL_CB_FORMS_AB: u64 = 0x10022;
const F4_FORMS_IJ: u64 = 0x10046;
const LOCAL_CB: u64 = 0x10020;
const SET_LOCAL_FORMS_CB: u64 = 0x1005a;
const F3: u64 = 0x10014;
const SET_GLOBAL_READS_F3: u64 = 0x10076;

#[test]
fn an_odd_string_in_text_keeps_its_address_and_a_thumb_pointer_loses_its_bit() {
    let idx = index_of("picodd_thumb_le32");
    assert_eq!(
        to(&idx, AB),
        vec![(LOCAL_CB_FORMS_AB, XrefKind::Data), (F1_FORMS_AB, XrefKind::Data)]
    );
    assert_eq!(to(&idx, IJ), vec![(F4_FORMS_IJ, XrefKind::Data)]);
    assert!(to(&idx, AB - 1).is_empty(), "{:?}", to(&idx, AB - 1));
    assert!(to(&idx, IJ - 1).is_empty(), "{:?}", to(&idx, IJ - 1));
    assert!(to(&idx, LOCAL_CB).contains(&(SET_LOCAL_FORMS_CB, XrefKind::Data)), "{:?}", to(&idx, LOCAL_CB));
    assert!(to(&idx, F3).contains(&(SET_GLOBAL_READS_F3, XrefKind::Data)), "{:?}", to(&idx, F3));
}

//! End-to-end gate for two ARM false positives of the reference walk: the
//! partial sums a PLT stub builds its slot's address from, and the skip a
//! predicated instruction lifts to.
//!
//! Fixture: `xreffp_arm_le32` (built from `xreffp_arm_le32.c`), a dynamic A32
//! executable whose imports are called through GNU ld PLT stubs
//! (`add ip,pc,#0,12; add ip,ip,#0x12000; ldr pc,[ip,#M]!`), with `table`
//! between the stubs and the GOT so that `0x12000` is a mapped address.
//! `clamp`, `pick`, `pick_t` (Thumb-2) and `main` hold predicated non-branch
//! instructions; `next_branch` holds a `bne` to the next instruction.

use std::path::PathBuf;
use std::sync::OnceLock;

use kuna_analysis::listing::xrefs::{self, XrefIndex, XrefKind};
use kuna_console::engine::bootstrap_from_object;
use kuna_decomp::architecture::Architecture;

/// The PLT stubs (entry, slot) of `printf`, `puts` and `strlen`.
const STUBS: [(u64, u64); 3] = [(0x10310, 0x2300c), (0x1031c, 0x23010), (0x1034c, 0x23020)];
/// The immediate every stub adds to `ip`, which lands in `table`.
const STUB_ADD: u64 = 0x12000;
/// `main`'s calls to the stubs of `printf`, `puts` and `strlen`.
const STUB_CALLS: [(u64, u64); 3] = [(0x103c8, 0x10310), (0x10394, 0x1031c), (0x103b4, 0x1034c)];
/// `main`'s two PC-relative strings: the `add rN,pc,rN` and the string.
const STRINGS: [(u64, u64); 2] = [(0x10390, 0x10648), (0x103c4, 0x10650)];
/// Predicated non-branch instructions and their byte lengths: `movle`/`movgt`
/// in `main`, `movlt`/`movge` in `clamp`, `ldrne`/`ldreq` in `pick`, and the
/// Thumb-2 `ldrne` in `pick_t`'s `it ne` block.
const PREDICATED: [(u64, u64); 9] = [
    (0x1039c, 4),
    (0x103a0, 4),
    (0x10550, 4),
    (0x10558, 4),
    (0x10564, 4),
    (0x10568, 4),
    (0x10578, 2),
    (0x103dc, 4),
    (0x103e0, 4),
];
/// `next_branch`'s `bne` whose label is the next instruction, and `main`'s
/// `beq` to its `bl abort`.
const BRANCHES: [(u64, u64); 2] = [(0x10588, 0x1058c), (0x103f4, 0x10404)];
/// `_start`'s pool word, decoded as a predicated `andeq`, whose skip and
/// fall-through both run into the entry of `_start_c`.
const SKIP_INTO_ENTRY: (u64, u64) = (0x10430, 0x10434);

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn fixture(name: &str) -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures").join(name)
}

/// The reference index of fixture `name` under the default options.
fn build(name: &str) -> XrefIndex {
    build_with(name, |_| {})
}

/// The reference index of fixture `name` after `configure` sets its options.
fn build_with(name: &str, configure: fn(&mut Architecture)) -> XrefIndex {
    let bin = fixture(name);
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

/// `xreffp_arm_le32`'s reference index, built once.
fn index() -> &'static XrefIndex {
    static INDEX: OnceLock<XrefIndex> = OnceLock::new();
    INDEX.get_or_init(|| build("xreffp_arm_le32"))
}

fn from(idx: &XrefIndex, at: u64) -> Vec<(u64, XrefKind)> {
    idx.refs_from_instruction(at).iter().map(|r| (r.to, r.kind)).collect()
}

/// A PLT stub files the slot it jumps through, from its `ldr pc`, and nothing
/// for the PC value or the immediate it adds on the way.
#[test]
fn a_plt_stub_files_its_slot_and_no_partial_address() {
    let idx = index();
    for (stub, slot) in STUBS {
        let rows: Vec<_> = (0..3).flat_map(|i| from(idx, stub + 4 * i)).collect();
        assert_eq!(rows, vec![(slot, XrefKind::Read)], "{stub:#x}");
        assert_eq!(from(idx, stub + 8), vec![(slot, XrefKind::Read)], "{stub:#x}");
    }
    assert!(idx.refs_to(STUB_ADD).is_empty(), "{:?}", idx.refs_to(STUB_ADD));
}

/// A static image's IFUNC stub lies in `.iplt`, in the `.plt` template: it
/// files its `.got` slot and not the PC value its `adr` names.
#[test]
fn an_iplt_stub_files_its_slot_and_no_partial_address() {
    let (stub, slot) = (0x1007c, 0x110a4);
    let idx = build("xrefiplt_arm_le32");
    assert!(from(&idx, 0x10088).contains(&(stub, XrefKind::Call)), "{:?}", from(&idx, 0x10088));
    let rows: Vec<_> = (0..3).flat_map(|i| from(&idx, stub + 4 * i)).collect();
    assert_eq!(rows, vec![(slot, XrefKind::Read)]);
}

/// The calls into the stubs and the PC-relative strings of the caller keep
/// their rows.
#[test]
fn calls_to_plt_stubs_and_pc_relative_strings_keep_their_rows() {
    let idx = index();
    for (call, stub) in STUB_CALLS {
        assert!(from(idx, call).contains(&(stub, XrefKind::Call)), "{call:#x}: {:?}", from(idx, call));
    }
    for (add, string) in STRINGS {
        assert!(from(idx, add).contains(&(string, XrefKind::Data)), "{add:#x}: {:?}", from(idx, add));
    }
}

/// A predicated instruction that is not a branch files no jump to the next
/// instruction, in A32 and in a Thumb `it` block.
#[test]
fn a_predicated_instruction_files_no_jump_to_the_next_one() {
    let idx = index();
    for (at, len) in PREDICATED {
        assert!(from(idx, at).is_empty(), "{at:#x}: {:?}", from(idx, at));
        assert!(idx.refs_to(at + len).is_empty(), "{:#x}: {:?}", at + len, idx.refs_to(at + len));
    }
}

/// A conditional branch keeps its jump, also when its label is the next
/// instruction, and the code after a predicated instruction is still walked.
#[test]
fn conditional_branches_keep_their_jump() {
    let idx = index();
    for (at, label) in BRANCHES {
        assert_eq!(from(idx, at), vec![(label, XrefKind::Jump)], "{at:#x}");
    }
    assert_eq!(idx.function_containing(0x10568), Some(0x10560));
    assert_eq!(idx.function_containing(0x1057a), Some(0x10570));
}

/// A skip that runs into another function's entry from an instruction that
/// also falls through there is no way in of its own: its fall-through
/// reaches the same entry, so it files nothing.
#[test]
fn a_skip_that_falls_through_into_an_entry_files_nothing() {
    let (at, entry) = SKIP_INTO_ENTRY;
    assert!(index().is_function_entry(entry));
    assert_eq!(from(index(), at), vec![]);
}

/// The condition skip is an ARM rule: off ARM, an instruction whose own skip
/// SLEIGH spells `goto inst_next` (x86-64 `cmove`) keeps its jump as before.
#[test]
fn a_condition_skip_off_arm_keeps_its_jump() {
    let idx = build("covertrim_x86_64");
    for (at, next) in [(0x401042, 0x401045), (0x401063, 0x401067)] {
        assert_eq!(from(&idx, at), vec![(next, XrefKind::Jump)], "{at:#x}");
    }
}

/// On the stripped `xreffp_strip_arm_le32`, `found_by_call` is found only from
/// `_start`'s `bl`, after the walk has queued it: the `moveq` that runs into
/// it from `falls_in` falls through there too, so its skip files nothing,
/// under both `armframes` settings.
#[test]
fn a_skip_that_falls_through_into_an_entry_found_from_a_call_files_nothing() {
    let (at, entry) = (0x10074, 0x10078);
    for idx in [build("xreffp_strip_arm_le32"), build_with("xreffp_strip_arm_le32", armframes_off)] {
        assert!(idx.is_function_entry(entry));
        assert_eq!(from(&idx, at), vec![]);
    }
}

fn armframes_off(arch: &mut Architecture) {
    arch.set_kuna_option("armframes", "off").expect("armframes flips off");
}

/// An A32 table dispatch's skip is a condition skip like any other, and a
/// dispatch has no fall-through: when its skip runs into the next function's
/// entry (`dispatch_last`'s `addls pc` into `after_dispatch`, found only from
/// a call) it is the one way in there and keeps its jump, whether `armframes`
/// reads the table or not.
#[test]
fn a_dispatch_skip_into_an_entry_keeps_its_jump() {
    let (at, entry) = (0x10084, 0x10088);
    for idx in [build("xreffp_strip_arm_le32"), build_with("xreffp_strip_arm_le32", armframes_off)] {
        assert!(idx.is_function_entry(entry));
        let jumps: Vec<_> = from(&idx, at).into_iter().filter(|&(_, kind)| kind == XrefKind::Jump).collect();
        assert!(jumps.contains(&(entry, XrefKind::Jump)), "{jumps:?}");
    }
}

/// A real branch to the next instruction inside an `it` block (`it eq;
/// beq.w`) lifts to the condition skip and the branch, both to the next
/// instruction: the skip files no row and the branch keeps its jump.
#[test]
fn a_predicated_branch_to_the_next_instruction_keeps_its_jump() {
    let idx = build("xreffp_strip_arm_le32");
    let rows: Vec<_> = idx.refs_from_instruction(0x100a8).iter().map(|r| (r.to, r.kind, r.instruction.clone())).collect();
    assert_eq!(rows, vec![(0x100ac, XrefKind::Jump, "b.eq.w 0x100ac".to_string())]);
}

/// A kept skip carries the text of the instruction in the mode the walk
/// decoded it in: `it_return`'s Thumb `it ne; bxne lr` has no fall-through, so
/// its skip is the one way into `thumb_next`, and it renders as Thumb with
/// `armframes` at its default and on.
#[test]
fn a_kept_thumb_skip_is_rendered_as_the_walk_decoded_it() {
    let (at, entry) = (0x100be, 0x100c0);
    for idx in [build("xreffp_strip_arm_le32"), build_with("xreffp_strip_arm_le32", armframes_on)] {
        assert!(idx.is_function_entry(entry));
        let rows: Vec<_> = idx.refs_from_instruction(at).iter().map(|r| (r.to, r.kind, r.instruction.clone())).collect();
        assert_eq!(rows, vec![(entry, XrefKind::Jump, "bx.ne lr".to_string())]);
    }
}

fn armframes_on(arch: &mut Architecture) {
    arch.set_kuna_option("armframes", "on").expect("armframes flips on");
}

/// The condition-skip rule leaves A32 table reading alone: `dispatch_last`'s
/// `addls pc` table is still read and its cases still referenced.
#[test]
fn a32_tables_are_read_with_condition_skips() {
    let idx = build("xreffp_strip_arm_le32");
    let tables: Vec<_> = idx.switch_tables().iter().map(|t| (t.dispatch, t.cases.clone())).collect();
    assert_eq!(tables.len(), 1, "{tables:?}");
    assert_eq!(tables[0].0, 0x10084);
}

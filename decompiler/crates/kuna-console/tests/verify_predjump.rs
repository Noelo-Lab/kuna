//! End-to-end gate for predicated computed jumps on ARM, which the A32 table
//! reader of the reference walk must leave in their parent shape.
//!
//! Fixture: `predjump_arm_le32` (built from `predjump_arm_le32.c`). `pool_skip`
//! (A32) and `pool_skip_t` (Thumb, `it ne`) load a pool offset, skip a
//! predicated `bxne r3` and form a string with `add rN,pc,r2` after it.
//! `bail` leaves through `bxne r0` or `bl abort`, and `bail_caller` calls it.
//! `unguarded` is an `ldr pc,[pc,r0,lsl #2]` table with no range check right
//! after `pool_tail`'s pool word.

use std::path::PathBuf;
use std::sync::OnceLock;

use kuna_analysis::listing::xrefs::{self, XrefIndex, XrefKind};
use kuna_analysis::listing::Listing;
use kuna_console::engine::bootstrap_from_object;
use kuna_console::ifacedecomp::{execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE};
use kuna_console::ifaceterm::ConsoleCommands;

/// `pool_skip`: the `add r0,pc,r2` after `bxne r3`, and its string.
const POOL_SKIP_ADD: u64 = 0x10080;
const POOL_MSG: u64 = 0x10158;
/// `pool_skip_t`: the `add r2,pc` after `it ne; bxne r3`, and its string.
const POOL_SKIP_T_ADD: u64 = 0x10094;
const POOL_MSG_T: u64 = 0x1016a;
/// The predicated computed jumps: `bxne r3` in `pool_skip` and `pool_skip_t`,
/// `bxne r0` in `bail`.
const PREDICATED_JUMPS: [u64; 3] = [0x1007c, 0x10092, 0x100a4];
/// `unguarded`'s dispatch, its entry.
const UNGUARDED: u64 = 0x100b8;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn fixture() -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/predjump_arm_le32")
}

/// The fixture's reference index under the default options, built once.
fn index() -> &'static XrefIndex {
    static INDEX: OnceLock<XrefIndex> = OnceLock::new();
    INDEX.get_or_init(|| {
        let bin = fixture();
        let spec_roots = vec![repo_root().join("specs").to_str().unwrap().to_string()];
        let mut prog = bootstrap_from_object(bin.to_str().unwrap(), "", &spec_roots)
            .expect("bootstrap fixture with built processor specs");
        prog.commit_pending_analysis().expect("analysis commit succeeds");
        let bytes = std::fs::read(&bin).expect("fixture readable");
        let file = object::File::parse(&*bytes).expect("fixture parses");
        let seeds: Vec<u64> =
            prog.function_entries_canonical().iter().map(|e| e.addr.get_offset()).collect();
        xrefs::build(&file, prog.arch(), prog.arch().translate(), &seeds)
    })
}

fn spec_roots() -> Vec<String> {
    vec![repo_root().join("specs").to_str().unwrap().to_string()]
}

fn from(idx: &XrefIndex, at: u64) -> Vec<(u64, XrefKind)> {
    idx.refs_from_instruction(at).iter().map(|r| (r.to, r.kind)).collect()
}

/// A pool word loaded before a predicated computed jump reaches the `add`
/// after it along the path where the jump did not run, in A32 and in Thumb.
#[test]
fn a_pool_word_survives_the_skip_of_a_predicated_computed_jump() {
    let idx = index();
    for (add, string) in [(POOL_SKIP_ADD, POOL_MSG), (POOL_SKIP_T_ADD, POOL_MSG_T)] {
        let rows = from(idx, add);
        assert!(rows.contains(&(string, XrefKind::Data)), "{add:#x}: {rows:?}");
    }
}

/// A table with no range check gives no case count, so no reader walks its
/// cases, whether or not the code before its entry was decoded: the dispatch
/// files only the row its PC-relative operand gives, as before A32 tables
/// were read.
#[test]
fn an_unguarded_a32_table_at_an_entry_reads_no_case() {
    let idx = index();
    assert_eq!(from(idx, UNGUARDED), vec![(UNGUARDED, XrefKind::Data)]);
    assert!(idx.switch_tables().iter().all(|t| t.dispatch != UNGUARDED), "{:?}", idx.switch_tables());
}

/// The Listing records a predicated computed jump as `classify` does, with its
/// skip as a flow and no fall-through, the same shape every other decoder
/// of the instruction sees.
#[test]
fn the_listing_keeps_a_predicated_computed_jump_skip_as_a_flow() {
    let bin = fixture();
    let prog = bootstrap_from_object(bin.to_str().unwrap(), "", &spec_roots())
        .expect("bootstrap fixture with built processor specs");
    let bytes = std::fs::read(&bin).expect("fixture readable");
    let file = object::File::parse(&*bytes).expect("fixture parses");
    let image = kuna_analysis::loadimage_object::ObjectLoadImage::from_bytes(bin.to_str().unwrap(), &bytes)
        .expect("open throwaway loadimage");
    let seeds: Vec<u64> =
        prog.function_entries_canonical().iter().map(|e| e.addr.get_offset()).collect();
    let arch = prog.arch();
    let listing = Listing::build(&file, &image, arch, arch.translate(), &seeds);
    for at in PREDICATED_JUMPS {
        let insn = listing.instruction_at(at).unwrap_or_else(|| panic!("{at:#x} decoded"));
        let skip = at + u64::from(insn.len);
        assert_eq!((insn.flows.to_vec(), insn.fall_through), (vec![skip], None), "{at:#x}");
    }
}

/// `bail_caller`'s C with `option`s applied before analysis.
fn decompile_bail_caller(options: &[(&str, &str)]) -> String {
    let mut prog = bootstrap_from_object(fixture().to_str().unwrap(), "", &spec_roots())
        .expect("bootstrap fixture with built processor specs");
    for &(name, value) in options {
        prog.arch_mut().set_kuna_option(name, value).expect("option applies");
    }
    prog.commit_pending_analysis().expect("analysis commit succeeds");
    let cmds: Vec<String> = ["load function bail_caller".into(), "decompile".into(), "print C".into()].to_vec();
    let count = cmds.len();
    let mut status = ConsoleCommands::into_status(cmds);
    register_decomp_commands(&mut status);
    {
        let data = status.get_data_mut(DECOMPILE_MODULE).unwrap();
        let dcp = data.as_any_mut().downcast_mut::<IfaceDecompData>().unwrap();
        dcp.conf = Some(prog);
    }
    for _ in 0..count {
        execute(&mut status);
    }
    status.optr.clone()
}

/// No-return propagation follows the skip of `bail`'s `bxne r0` to its
/// `bl abort`, so `bail` is no-return and `bail_caller` drops its `return`.
/// `noreturn_disc` is off so that propagation alone decides.
#[test]
fn a_function_leaving_through_a_predicated_jump_and_abort_stays_no_return() {
    let c = decompile_bail_caller(&[("listing", "on"), ("noreturn_disc", "off")]);
    assert!(c.contains("bail") && c.contains("// no-return"), "{c}");
    assert!(!c.contains("* 7"), "{c}");
}

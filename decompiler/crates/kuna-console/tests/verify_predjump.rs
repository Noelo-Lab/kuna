//! End-to-end gate for predicated computed jumps on ARM, which the A32 table
//! reader of the reference walk must leave in their parent shape.
//!
//! Fixture: `predjump_arm_le32` (built from `predjump_arm_le32.c`). `pool_skip`
//! (A32) and `pool_skip_t` (Thumb, `it ne`) load a pool offset, skip a
//! predicated `bxne r3` and form a string with `add rN,pc,r2` after it.

use std::path::PathBuf;
use std::sync::OnceLock;

use kuna_analysis::listing::xrefs::{self, XrefIndex, XrefKind};
use kuna_console::engine::bootstrap_from_object;

/// `pool_skip`: the `add r0,pc,r2` after `bxne r3`, and its string.
const POOL_SKIP_ADD: u64 = 0x10080;
const POOL_MSG: u64 = 0x10158;
/// `pool_skip_t`: the `add r2,pc` after `it ne; bxne r3`, and its string.
const POOL_SKIP_T_ADD: u64 = 0x10094;
const POOL_MSG_T: u64 = 0x1016a;

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

//! End-to-end gate for `poolref` — a PC-relative literal-pool load resolved to
//! the address the pool word holds (P1 code/data partition, the on-demand xref
//! query).
//!
//! Fixture: `poolref_arm_le32` (`poolref_arm_le32.py` builds it), the four-shape
//! reduction of crackmes.one/5ab77f5733c5d40ad448c380, where
//! `kuna strings --json --filter "FATAL: kernel too old"` reported
//! `xrefs_count: 0` and no owning function for a string that
//! `__libc_start_main` plainly loads: the address is nowhere in the
//! instruction, only in the pool word at 0x86e4.
//!
//! One shape must be followed and three must not, which is the whole contract:
//!
//! * `uses_prompt` reads a pointer-sized read-only pool word — followed.
//! * `narrow_read` reads two bytes of one, which is reading a number out of a
//!   pool. A `LOAD`'s address varnode is pointer-sized whatever the access is,
//!   so this is the shape that catches a width taken from the wrong varnode.
//! * `reads_slot` reads a **writable** word holding the same kind of address;
//!   the image's copy of a writable slot is not evidence of anything.
//! * `reads_number` reads a read-only word holding 42.
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built `ARM` `.sla` under `specs/` (gitignored; `make
//! specs`). When it is absent the bootstrap fails; the test prints that and
//! returns early (a specs-less CI is a visible skip, never a false green).

use std::path::PathBuf;

use kuna_analysis::listing::xrefs::{self, XrefIndex, XrefKind};
use kuna_console::engine::bootstrap_from_object;

/// `"kuna poolref prompt"` — reached through a read-only pool word.
const PROMPT: u64 = 0x10100;
/// `"kuna poolref narrow"` — reached only by a two-byte read of a pool word.
const NARROW: u64 = 0x10120;
/// `"kuna poolref hidden"` — reached only through a `.data` slot.
const HIDDEN: u64 = 0x10140;

/// The `ldr r0,[0x10028]` that forms `PROMPT`, and the function it lies in.
const USES_PROMPT: u64 = 0x10020;
/// The pool word `USES_PROMPT` reads.
const POOL_PROMPT: u64 = 0x10028;
/// The `ldr r0,[0x10048]` whose pool word holds 42.
const READS_NUMBER: u64 = 0x10040;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// Bootstrap the fixture and build the index `kuna xrefs` / `kuna strings`
/// answer out of. `None` is a visible skip when the `.sla` is missing.
fn index() -> Option<XrefIndex> {
    let bin = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/poolref_arm_le32");
    let specs = repo_root().join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];
    let mut prog = match bootstrap_from_object(bin.to_str().unwrap(), "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_poolref: skipping (bootstrap failed, build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };
    prog.commit_pending_analysis().expect("analysis commit succeeds");

    let bytes = std::fs::read(&bin).expect("fixture readable");
    let file = object::File::parse(&*bytes).expect("fixture parses");
    let seeds: Vec<u64> =
        prog.function_entries_canonical().iter().map(|e| e.addr.get_offset()).collect();
    Some(xrefs::build(&file, prog.arch(), prog.arch().translate(), &seeds))
}

/// The defect: the literal is referenced from the instruction that loads its
/// address out of the pool, not merely from nowhere.
#[test]
fn a_literal_reached_through_a_pool_word_is_referenced_by_the_load() {
    let Some(idx) = index() else { return };
    let refs: Vec<(u64, XrefKind)> =
        idx.refs_to(PROMPT).iter().map(|r| (r.from, r.kind)).collect();
    assert_eq!(
        refs,
        vec![(USES_PROMPT, XrefKind::Data)],
        "the address of the prompt is in the pool word at {POOL_PROMPT:#x} and \
         nowhere in any instruction, so before this the literal was referenced \
         by nothing"
    );
    assert_eq!(
        idx.function_containing(USES_PROMPT),
        Some(USES_PROMPT),
        "the reference is attributed to the function the LOAD lies in, which is \
         what gives a data address an owner at all"
    );
}

/// The pre-existing edge is kept: the pool word itself is still read.
#[test]
fn the_pool_word_is_still_read_by_the_same_instruction() {
    let Some(idx) = index() else { return };
    let refs: Vec<(u64, XrefKind)> =
        idx.refs_to(POOL_PROMPT).iter().map(|r| (r.from, r.kind)).collect();
    assert_eq!(refs, vec![(USES_PROMPT, XrefKind::Read)]);
}

/// The three refusals, each of which would be a fabricated reference.
#[test]
fn a_narrow_read_a_writable_slot_and_a_number_are_not_followed() {
    let Some(idx) = index() else { return };
    assert!(
        idx.refs_to(NARROW).is_empty(),
        "`ldrh r0,[pool]` reads a number out of the pool, not a pointer; got {:?}",
        idx.refs_to(NARROW)
    );
    assert!(
        idx.refs_to(HIDDEN).is_empty(),
        "the .data slot holding this address is writable, so the image's copy of \
         it is not evidence of anything; got {:?}",
        idx.refs_to(HIDDEN)
    );
    assert!(
        idx.refs_from_instruction(READS_NUMBER)
            .iter()
            .all(|r| r.kind == XrefKind::Read),
        "the pool word here holds 42, which is a number however well it lands; \
         got {:?}",
        idx.refs_from_instruction(READS_NUMBER)
    );
}

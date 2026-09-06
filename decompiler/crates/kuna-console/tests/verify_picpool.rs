//! End-to-end gate for `picpool` — a PC-relative literal-pool *displacement*
//! composed with the PC that adds it (P1 code/data partition, the on-demand xref
//! query).
//!
//! Fixture: `picpool_arm_le32` (`picpool_arm_le32.py` builds it), the reduction
//! of crackmes.one/68d40081224c0ec5dcedc2d2, where
//! `kuna strings --json --filter "Benar!"` reported `xrefs_count: 0` and no
//! owning function for a string `main` plainly prints: the address is in neither
//! instruction of `ldr r0,[0x6a0] ; add r0,pc,r0`, and the pool word holds the
//! signed distance -0x1c1, which is not an address at all.
//!
//! Two shapes must be composed and one must not, which is the whole contract:
//!
//! * `uses_prompt` is the adjacent pair — followed.
//! * `scheduled` is the same pair with two instructions in between, which is
//!   what instruction scheduling does — followed.
//! * `no_pc` does `add r0,r0,#4` on a pool word whose sum IS a mapped address.
//!   Only the missing PC separates it from a real reference, so it is the shape
//!   that catches a fold that would report any arithmetic.
//!
//! The image is mapped under 0x1000, as the filing Android PIE is, so it also
//! holds `ScalarOperandAnalyzer.checkOperands`' "below 4096 could be a number"
//! floor off the composed address — applying it there would report nothing on
//! either binary.
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built `ARM` `.sla` under `specs/` (gitignored; `make
//! specs`). When it is absent the bootstrap fails; the test prints that and
//! returns early (a specs-less CI is a visible skip, never a false green).

use std::path::PathBuf;

use kuna_analysis::listing::xrefs::{self, XrefIndex, XrefKind};
use kuna_console::engine::bootstrap_from_object;

/// `"kuna picpool prompt"` — reached only by an adjacent `ldr`/`add pc` pair.
const PROMPT: u64 = 0x300;
/// `"kuna picpool second"` — reached only by a pair the scheduler separated.
const SECOND: u64 = 0x320;
/// `"kuna picpool number"` — reached only by arithmetic with no PC in it.
const NUMBER: u64 = 0x340;

/// The `add r0,pc,r0` that completes `PROMPT`, and the function it lies in.
const USES_PROMPT: u64 = 0x420;
const COMPOSES_PROMPT: u64 = 0x424;
/// The `add r0,pc,r0` that completes `SECOND`, two instructions past its load.
const SCHEDULED: u64 = 0x430;
const COMPOSES_SECOND: u64 = 0x43c;
/// The `add r0,r0,#4` that forms `NUMBER` without ever reading the PC.
const NO_PC: u64 = 0x448;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// Bootstrap the fixture and build the index `kuna xrefs` / `kuna strings`
/// answer out of. `None` is a visible skip when the `.sla` is missing.
fn index() -> Option<XrefIndex> {
    let bin = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/picpool_arm_le32");
    let specs = repo_root().join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];
    let mut prog = match bootstrap_from_object(bin.to_str().unwrap(), "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_picpool: skipping (bootstrap failed, build `.sla` with \
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

/// The defect: the address of the literal is in neither instruction of the pair
/// and in the pool word least of all, so before this it was referenced by
/// nothing at all.
#[test]
fn a_pool_displacement_composed_with_the_pc_references_the_literal() {
    let Some(idx) = index() else { return };
    let refs: Vec<(u64, XrefKind)> =
        idx.refs_to(PROMPT).iter().map(|r| (r.from, r.kind)).collect();
    assert_eq!(
        refs,
        vec![(COMPOSES_PROMPT, XrefKind::Data)],
        "the address of the prompt is 0x42c + (-0x12c) and occurs nowhere else"
    );
    assert_eq!(
        idx.function_containing(COMPOSES_PROMPT),
        Some(USES_PROMPT),
        "the reference is attributed to the function the `add` lies in, which is \
         what gives a data address an owner at all"
    );
}

/// The pair need not be adjacent — a scheduler separates them whenever a
/// function forms several references at once.
#[test]
fn the_add_may_be_scheduled_away_from_its_load() {
    let Some(idx) = index() else { return };
    let refs: Vec<(u64, XrefKind)> =
        idx.refs_to(SECOND).iter().map(|r| (r.from, r.kind)).collect();
    assert_eq!(refs, vec![(COMPOSES_SECOND, XrefKind::Data)]);
    assert_eq!(idx.function_containing(COMPOSES_SECOND), Some(SCHEDULED));
}

/// The refusal, which would otherwise be a fabricated reference: `add r0,r0,#4`
/// lands on a real literal, and the program never read the PC to get there.
#[test]
fn arithmetic_that_never_reads_the_pc_composes_nothing() {
    let Some(idx) = index() else { return };
    assert!(
        idx.refs_to(NUMBER).is_empty(),
        "the pool word here is a number the program added four to, not a \
         displacement from a PC; got {:?}",
        idx.refs_to(NUMBER)
    );
    assert!(
        idx.refs_from_instruction(NO_PC).iter().all(|r| r.kind != XrefKind::Data),
        "got {:?}",
        idx.refs_from_instruction(NO_PC)
    );
}

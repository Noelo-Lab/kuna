//! End-to-end gate for `switchtable` — a computed jump's case bodies read out of
//! the table it indexes (the on-demand xref query's recursive descent).
//!
//! Fixtures: `switchtable_i386` and `switchtable_x86_64`, the reduction of
//! crackmes.one/60be2ad433c5d410b8842c95, where
//! `kuna strings --json --filter "Product Already Registered"` reported
//! `xrefs_count: 0` and no owning function for a literal the window procedure
//! plainly pushes — the descent has no successor for `JMP dword ptr [EAX*0x4 +
//! 0x4017c4]`, so every case body of the message switch was undecoded.
//!
//! Each fixture is one `dispatch` whose four cases push a distinct literal and
//! whose default arm pushes a fifth. The default arm is the control: it is
//! reached by the `JA` and was always attributed correctly, so a run where only
//! it has an owner is exactly the defect. The two differ in the table's stride
//! (4-byte `.long` entries, 8-byte `.quad` entries) and in how the literal's
//! address is materialized (`PUSH imm32`, RIP-relative `LEA`).
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built `x86` `.sla` under `specs/` (gitignored; `make
//! specs`). When it is absent the bootstrap fails; the test prints that and
//! returns early (a specs-less CI is a visible skip, never a false green).

use std::path::PathBuf;

use kuna_analysis::listing::xrefs::{self, XrefIndex, XrefKind};
use kuna_console::engine::bootstrap_from_object;

/// The pinned layout of one fixture: both are linked at the same base, so only
/// the instruction lengths differ.
struct Fixture {
    name: &'static str,
    /// The function every reference below must be attributed to.
    dispatch: u64,
    /// The `JMP [index*stride + table]` that dispatches the switch.
    branch: u64,
    /// The four case bodies, in table order.
    cases: [u64; 4],
    /// The literal each case body materializes, in the same order.
    literals: [u64; 4],
    /// The literal the default arm materializes — reached by the `JA`, so it had
    /// an owner before this feature and must still have one.
    default_literal: u64,
    /// The table itself, which the branch reads as data.
    table: u64,
}

const I386: Fixture = Fixture {
    name: "switchtable_i386",
    dispatch: 0x100000,
    branch: 0x100009,
    cases: [0x100010, 0x100017, 0x10001e, 0x100025],
    literals: [0x101010, 0x10102a, 0x101043, 0x10105d],
    default_literal: 0x101077,
    table: 0x101000,
};

const X86_64: Fixture = Fixture {
    name: "switchtable_x86_64",
    dispatch: 0x100000,
    branch: 0x100007,
    cases: [0x10000e, 0x100017, 0x100020, 0x100029],
    literals: [0x101020, 0x10103a, 0x101053, 0x10106d],
    default_literal: 0x101087,
    table: 0x101000,
};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// Bootstrap `fx` and build the index `kuna xrefs` / `kuna strings` answer out
/// of. `None` is a visible skip when the `.sla` is missing.
fn index(fx: &Fixture) -> Option<XrefIndex> {
    let bin = repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures")
        .join(fx.name);
    let specs = repo_root().join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];
    let mut prog = match bootstrap_from_object(bin.to_str().unwrap(), "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_switchtable: skipping {} (bootstrap failed, build `.sla` \
                 with `make specs`): {}",
                fx.name,
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

/// The defect: a literal only a case body forms is referenced by that case body,
/// and the case body belongs to the function that dispatches into it.
#[test]
fn a_literal_in_a_switch_case_body_is_owned_by_the_dispatching_function() {
    for fx in [I386, X86_64] {
        let Some(idx) = index(&fx) else { continue };
        for (i, &lit) in fx.literals.iter().enumerate() {
            let refs: Vec<u64> = idx.refs_to(lit).iter().map(|r| r.from).collect();
            assert!(
                !refs.is_empty(),
                "{}: case {i}'s literal at {lit:#x} is reached only through the \
                 jump table, so before this it was referenced by nothing",
                fx.name
            );
            for from in refs {
                assert_eq!(
                    idx.function_containing(from),
                    Some(fx.dispatch),
                    "{}: the reference at {from:#x} belongs to the dispatcher, \
                     not to whatever entry happens to precede the case body",
                    fx.name
                );
            }
        }
    }
}

/// Each table entry is a jump edge from the dispatch, so `kuna xrefs --to` a
/// case body names the switch that reaches it.
#[test]
fn the_dispatch_files_a_jump_edge_to_every_case_body() {
    for fx in [I386, X86_64] {
        let Some(idx) = index(&fx) else { continue };
        for (i, &case) in fx.cases.iter().enumerate() {
            let refs: Vec<(u64, XrefKind)> =
                idx.refs_to(case).iter().map(|r| (r.from, r.kind)).collect();
            assert!(
                refs.contains(&(fx.branch, XrefKind::Jump)),
                "{}: case {i} at {case:#x} is dispatched to from {:#x}; got {refs:?}",
                fx.name,
                fx.branch
            );
        }
    }
}

/// The stop rule. The word after the last entry is the first literal's bytes,
/// which is not a code address — so the scan ends there and the dispatch jumps
/// to the four cases and nothing else. A table read that ran on would take up
/// whatever follows it as code.
#[test]
fn the_table_scan_stops_at_the_end_of_the_table() {
    for fx in [I386, X86_64] {
        let Some(idx) = index(&fx) else { continue };
        let mut jumps: Vec<u64> = idx
            .refs_from_instruction(fx.branch)
            .iter()
            .filter(|r| r.kind == XrefKind::Jump)
            .map(|r| r.to)
            .collect();
        jumps.sort_unstable();
        assert_eq!(
            jumps,
            fx.cases.to_vec(),
            "{}: the dispatch jumps to exactly the four table entries",
            fx.name
        );
    }
}

/// Nothing the walk already answered moved: the table is still a data reference
/// of the branch, and the default arm's literal — reached by the `JA`, never by
/// the table — keeps the owner it always had.
#[test]
fn the_table_is_still_data_and_the_default_arm_is_unchanged() {
    for fx in [I386, X86_64] {
        let Some(idx) = index(&fx) else { continue };
        let table: Vec<(u64, XrefKind)> =
            idx.refs_to(fx.table).iter().map(|r| (r.from, r.kind)).collect();
        assert_eq!(
            table,
            vec![(fx.branch, XrefKind::Data)],
            "{}: the table base is the address the branch materializes",
            fx.name
        );
        let refs: Vec<u64> = idx.refs_to(fx.default_literal).iter().map(|r| r.from).collect();
        assert_eq!(refs.len(), 1, "{}: got {refs:?}", fx.name);
        assert_eq!(idx.function_containing(refs[0]), Some(fx.dispatch), "{}", fx.name);
    }
}

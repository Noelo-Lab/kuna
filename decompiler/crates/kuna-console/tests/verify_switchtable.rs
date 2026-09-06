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
//! Two more fixtures cover the shape real x86-64 compilers emit, reported as
//! GH-456: the table base is materialized by an instruction of its own and the
//! entries are signed 32-bit displacements, not pointers. `switchtable_pic_x86_64`
//! is the gcc form (`lea jt(%rip),%rdx; movslq (%rdx,%rax,4),%rax; add %rdx,%rax`,
//! base == table) and `pe_switchdelta_x86_64.exe` the MSVC one (`LEA
//! RDX,[__ImageBase]; MOV ECX,[RDX+RBX*4+0x2000]; ADD RCX,RDX`, base != table),
//! where each case body holds a direct `CALL` nothing else in the image reaches.
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

/// A delta-encoded table: the base is on its own instruction and the entries are
/// 32-bit displacements from it, so the dispatching instruction materializes no
/// constant and files no data reference at all.
struct Delta {
    name: &'static str,
    dispatch: u64,
    /// The `JMP <reg>` the chain ends in.
    branch: u64,
    /// The four case bodies, in table order.
    cases: [u64; 4],
    /// The table, which is `base + RVA` on the MSVC form.
    table: u64,
    /// What each case body reaches and nothing else does — a literal on the ELF,
    /// a direct callee on the PE.
    reached: [u64; 4],
}

const PIC_X86_64: Delta = Delta {
    name: "switchtable_pic_x86_64",
    dispatch: 0x100000,
    branch: 0x100015,
    cases: [0x100017, 0x100020, 0x100029, 0x100032],
    table: 0x101000,
    reached: [0x101010, 0x10102a, 0x101043, 0x10105d],
};

const PE_MSVC: Delta = Delta {
    name: "pe_switchdelta_x86_64.exe",
    dispatch: 0x140001040,
    branch: 0x14000105d,
    cases: [0x140001060, 0x140001070, 0x140001080, 0x140001090],
    table: 0x140002000,
    reached: [0x140001000, 0x140001010, 0x140001020, 0x140001030],
};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// Bootstrap the named fixture and build the index `kuna xrefs` / `kuna strings`
/// answer out of. `None` is a visible skip when the `.sla` is missing.
fn index_of(name: &str) -> Option<XrefIndex> {
    let bin = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures").join(name);
    let specs = repo_root().join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];
    let mut prog = match bootstrap_from_object(bin.to_str().unwrap(), "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_switchtable: skipping {} (bootstrap failed, build `.sla` \
                 with `make specs`): {}",
                name,
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
        let Some(idx) = index_of(fx.name) else { continue };
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
        let Some(idx) = index_of(fx.name) else { continue };
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
        let Some(idx) = index_of(fx.name) else { continue };
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
        let Some(idx) = index_of(fx.name) else { continue };
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

/// The GH-456 defect: with the base on its own instruction and the entries
/// encoded as displacements, the dispatch had no successor at all, so the case
/// bodies — and on the PE the direct calls inside them — were invisible.
#[test]
fn a_delta_table_dispatches_to_every_case_body() {
    for fx in [PIC_X86_64, PE_MSVC] {
        let Some(idx) = index_of(fx.name) else { continue };
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
            "{}: the dispatch reaches exactly the four table entries",
            fx.name
        );
    }
}

/// A case body is the dispatcher's own code, so what it reaches is attributed
/// there — the callee list `kuna decompile-graph` answers with.
#[test]
fn what_a_delta_case_body_reaches_belongs_to_the_dispatcher() {
    for fx in [PIC_X86_64, PE_MSVC] {
        let Some(idx) = index_of(fx.name) else { continue };
        for (i, &to) in fx.reached.iter().enumerate() {
            let refs: Vec<u64> = idx.refs_to(to).iter().map(|r| r.from).collect();
            assert!(
                !refs.is_empty(),
                "{}: case {i} reaches {to:#x} and nothing else does",
                fx.name
            );
            for from in refs {
                assert_eq!(
                    idx.function_containing(from),
                    Some(fx.dispatch),
                    "{}: the reference at {from:#x} belongs to the dispatcher",
                    fx.name
                );
            }
        }
    }
}

/// The negative half, and why the shipped rule could not answer these: the
/// dispatching instruction materializes no constant, so the table is not among
/// the data references it files. On the MSVC form the constant that IS
/// materialized is the image base, which is not the table either.
#[test]
fn the_delta_dispatch_materializes_no_table_address() {
    for fx in [PIC_X86_64, PE_MSVC] {
        let Some(idx) = index_of(fx.name) else { continue };
        let from_branch: Vec<(u64, XrefKind)> = idx
            .refs_from_instruction(fx.branch)
            .iter()
            .filter(|r| r.kind == XrefKind::Data)
            .map(|r| (r.to, r.kind))
            .collect();
        assert!(
            from_branch.is_empty(),
            "{}: the dispatch itself names no address; got {from_branch:?}",
            fx.name
        );
        let table_refs: Vec<u64> = idx.refs_to(fx.table).iter().map(|r| r.from).collect();
        assert!(
            !table_refs.contains(&fx.branch),
            "{}: the table at {:#x} is not an operand of the branch",
            fx.name,
            fx.table
        );
    }
}

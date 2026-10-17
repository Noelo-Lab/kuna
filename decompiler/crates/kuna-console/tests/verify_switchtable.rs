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

/// Bootstrap the named fixture and build the index `kuna xrefs` / `kuna strings` answer out
/// of.
fn index_of(name: &str) -> XrefIndex {
    let bin = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures").join(name);
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

/// The defect: a literal only a case body forms is referenced by that case body,
/// and the case body belongs to the function that dispatches into it.
#[test]
fn a_literal_in_a_switch_case_body_is_owned_by_the_dispatching_function() {
    for fx in [I386, X86_64] {
        let idx = index_of(fx.name);
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
        let idx = index_of(fx.name);
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
        let idx = index_of(fx.name);
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
        let idx = index_of(fx.name);
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
        let idx = index_of(fx.name);
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
        let idx = index_of(fx.name);
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
        let idx = index_of(fx.name);
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

/// One dispatch of `switchtable_gcc_x86_64`: the shapes gcc emits that the walk
/// could not read, each with four case bodies that call or tail-jump to a
/// function nothing else reaches.
struct Gcc {
    shape: &'static str,
    dispatch: u64,
    branch: u64,
    cases: [u64; 4],
    callees: [u64; 4],
}

const GCC: [Gcc; 8] = [
    Gcc {
        shape: "-O0: base in the unscaled index slot, cltq of eax, check ten up",
        dispatch: 0x100000,
        branch: 0x100036,
        cases: [0x100039, 0x100040, 0x100047, 0x10004e],
        callees: [0x1001a4, 0x1001a7, 0x1001aa, 0x1001ad],
    },
    Gcc {
        shape: "-O2: case-body pushes scheduled between the ja and the jump",
        dispatch: 0x100055,
        branch: 0x100072,
        cases: [0x100075, 0x10007b, 0x100081, 0x100087],
        callees: [0x1001b0, 0x1001b3, 0x1001b6, 0x1001b9],
    },
    Gcc {
        shape: "-O2: a store between the compare and its ja, a sub in the chain",
        dispatch: 0x10008e,
        branch: 0x1000ad,
        cases: [0x1000b0, 0x1000b7, 0x1000be, 0x1000c5],
        callees: [0x1001bc, 0x1001bf, 0x1001c2, 0x1001c5],
    },
    Gcc {
        shape: "-O2: base hoisted into a callee-saved register across a call",
        dispatch: 0x1000cf,
        branch: 0x1000f5,
        cases: [0x1000f8, 0x1000ff, 0x100106, 0x10010d],
        callees: [0x1001c8, 0x1001cb, 0x1001ce, 0x1001d1],
    },
    Gcc {
        shape: "a .cold fragment reaches the dispatch before the parent's prologue",
        dispatch: 0x100123,
        branch: 0x10013a,
        cases: [0x10013d, 0x100144, 0x10014b, 0x100152],
        callees: [0x1001d4, 0x1001d7, 0x1001da, 0x1001dd],
    },
    Gcc {
        shape: "jb: the check bounds the selector from below, another table follows",
        dispatch: 0x10015a,
        branch: 0x100170,
        cases: [0x100173, 0x100175, 0x100177, 0x100179],
        callees: [0x1001e0, 0x1001e3, 0x1001e6, 0x1001e9],
    },
    Gcc {
        shape: "an and-masked index under a lower-bound-only check",
        dispatch: 0x10017c,
        branch: 0x100194,
        cases: [0x100197, 0x100199, 0x10019b, 0x10019d],
        callees: [0x1001ec, 0x1001ef, 0x1001f2, 0x1001f5],
    },
    Gcc {
        shape: "clang's x & 3 under an unrelated y <= 200 check, another table follows",
        dispatch: 0x100231,
        branch: 0x10024c,
        cases: [0x10024e, 0x100250, 0x100252, 0x100254],
        callees: [0x10026e, 0x100271, 0x100274, 0x100277],
    },
];

/// clang's Duff's device (bash `internal_malloc`): eight cases behind `and
/// $0x7`, with the byte count checked above and a `cmovae` (a conditional
/// branch inside its own instruction) between the mask and the jump.
const DUFF: Gcc8 = Gcc8 {
    dispatch: 0x1001f8,
    branch: 0x10021e,
    cases: [0x100220, 0x100222, 0x100224, 0x100226, 0x100228, 0x10022a, 0x10022c, 0x10022e],
    callees: [0x100256, 0x100259, 0x10025c, 0x10025f, 0x100262, 0x100265, 0x100268, 0x10026b],
};

struct Gcc8 {
    dispatch: u64,
    branch: u64,
    cases: [u64; 8],
    callees: [u64; 8],
}

/// The masked Duff's device reads exactly its eight cases: the cmov is no
/// guard, and the byte-count check above it is looser than the mask.
#[test]
fn a_cmov_between_the_mask_and_the_jump_is_no_guard() {
    let idx = index_of("switchtable_gcc_x86_64");
    let mut jumps: Vec<u64> = idx
        .refs_from_instruction(DUFF.branch)
        .iter()
        .filter(|r| r.kind == XrefKind::Jump)
        .map(|r| r.to)
        .collect();
    jumps.sort_unstable();
    assert_eq!(jumps, DUFF.cases);
    for callee in DUFF.callees {
        let owners: Vec<Option<u64>> =
            idx.refs_to(callee).iter().map(|r| idx.function_containing(r.from)).collect();
        assert_eq!(owners, [Some(DUFF.dispatch)], "{callee:#x}");
    }
}

/// Each dispatch reaches exactly its four cases: a table the walk cannot read
/// leaves the case bodies undecoded, and a bound it misreads runs on into the
/// next table, whose entries are code addresses measured from the wrong base.
#[test]
fn every_gcc_dispatch_reaches_exactly_its_cases() {
    let idx = index_of("switchtable_gcc_x86_64");
    let mut wrong = Vec::new();
    for fx in GCC {
        let mut jumps: Vec<u64> = idx
            .refs_from_instruction(fx.branch)
            .iter()
            .filter(|r| r.kind == XrefKind::Jump)
            .map(|r| r.to)
            .collect();
        jumps.sort_unstable();
        if jumps != fx.cases {
            wrong.push(format!("{}: {jumps:#x?}", fx.shape));
        }
    }
    assert!(wrong.is_empty(), "dispatches that do not reach exactly their cases:\n{}", wrong.join("\n"));
}

/// The call edge the bug report is about: a function called only from a case
/// body has its caller, and the caller is the function that dispatches.
#[test]
fn a_function_called_only_from_a_case_body_has_its_dispatcher_as_caller() {
    let idx = index_of("switchtable_gcc_x86_64");
    let mut wrong = Vec::new();
    for fx in GCC {
        for callee in fx.callees {
            let owners: Vec<Option<u64>> = idx
                .refs_to(callee)
                .iter()
                .filter(|r| matches!(r.kind, XrefKind::Call | XrefKind::Jump))
                .map(|r| idx.function_containing(r.from))
                .collect();
            if owners != [Some(fx.dispatch)] {
                wrong.push(format!("{}: {callee:#x} reached from {owners:#x?}", fx.shape));
            }
        }
    }
    assert!(wrong.is_empty(), "case-body callees without their dispatcher:\n{}", wrong.join("\n"));
}

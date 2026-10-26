//! End-to-end gate for A32 jump tables in the reference walk.
//!
//! Fixture: `branchtable_arm_le32` (built from `branchtable_arm_le32.c` with
//! GCC). `add_dispatch` switches through `addls pc,pc,r5,lsl #2` and a table of
//! branches, `ldr_dispatch` through `ldrls pc,[pc,r5,lsl #2]` and a table of
//! words. Both make the GOT base in r6 before the dispatch, and every case
//! reads a callback's slot through it, calls `set_timeout` and forms a string.
//! `early_dispatch` branches into its own `addls` dispatch ahead of the guard.

use std::path::PathBuf;
use std::sync::OnceLock;

use kuna_analysis::listing::xrefs::{self, XrefIndex, XrefKind};
use kuna_console::engine::bootstrap_from_object;
use kuna_decomp::architecture::Architecture;

const SET_TIMEOUT: u64 = 0x100f8;

/// One switch: its function, its dispatch and, per case in table order, the
/// case body, the slot load, the slot, the callback, the string-forming `add`
/// and the string.
struct Switch {
    function: u64,
    dispatch: u64,
    cases: [Case; 5],
}

struct Case {
    body: u64,
    load: u64,
    slot: u64,
    callback: u64,
    forms: u64,
    string: u64,
}

const fn case(body: u64, load: u64, slot: u64, callback: u64, forms: u64, string: u64) -> Case {
    Case { body, load, slot, callback, forms, string }
}

/// `add` form: the cases are the `b caseN` slots after `b default`.
const ADD: Switch = Switch {
    function: 0x100fc,
    dispatch: 0x10124,
    cases: [
        case(0x1012c, 0x10148, 0x226a8, 0x10098, 0x10160, 0x21558),
        case(0x10130, 0x101c0, 0x226ac, 0x100a8, 0x101d8, 0x21564),
        case(0x10134, 0x101ec, 0x226a4, 0x100b8, 0x10204, 0x21570),
        case(0x10138, 0x10218, 0x226a0, 0x100c8, 0x10230, 0x2157c),
        case(0x1013c, 0x10184, 0x22698, 0x100d8, 0x1019c, 0x21588),
    ],
};

/// `ldr` form: the cases are the words after `b default`.
const LDR: Switch = Switch {
    function: 0x10274,
    dispatch: 0x1029c,
    cases: [
        case(0x102b8, 0x102c0, 0x226ac, 0x100a8, 0x102d8, 0x215c0),
        case(0x10330, 0x10338, 0x226a4, 0x100b8, 0x10350, 0x215cc),
        case(0x1035c, 0x10364, 0x226a0, 0x100c8, 0x1037c, 0x215d8),
        case(0x10388, 0x10390, 0x22698, 0x100d8, 0x103a8, 0x215e4),
        case(0x102f4, 0x102fc, 0x2269c, 0x100e8, 0x10314, 0x215f0),
    ],
};

/// `early_dispatch`: a `beq` into its `addls pc,pc,r4,lsl #2` sends the walk
/// to the dispatch before the `cmp r4,#4` guard that falls into it, and the
/// cases are the `b caseN` slots after `b default`.
const EARLY_FUNCTION: u64 = 0x103ec;
const EARLY_DISPATCH: u64 = 0x10414;
const EARLY_SLOTS: [u64; 5] = [0x1041c, 0x10420, 0x10424, 0x10428, 0x1042c];

/// The `ldr` form's table words, `.word case0` to `.word case4`.
const LDR_TABLE: [u64; 5] = [0x102a4, 0x102a8, 0x102ac, 0x102b0, 0x102b4];

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// The fixture's reference index under the default options, built once for
/// every test that reads it.
fn index() -> &'static XrefIndex {
    static INDEX: OnceLock<XrefIndex> = OnceLock::new();
    INDEX.get_or_init(|| build(|_| {}))
}

/// The fixture's reference index, with `configure` applied before analysis.
fn build(configure: fn(&mut Architecture)) -> XrefIndex {
    let bin = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/branchtable_arm_le32");
    let specs = repo_root().join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];
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

fn from(idx: &XrefIndex, at: u64) -> Vec<(u64, XrefKind)> {
    idx.refs_from_instruction(at).iter().map(|r| (r.to, r.kind)).collect()
}

/// Every case reads its callback's slot through the base the dispatch holds,
/// forms its string and calls `set_timeout`, all inside the dispatching
/// function.
fn assert_cases_walked(idx: &XrefIndex, sw: &Switch) {
    for c in &sw.cases {
        let load = from(idx, c.load);
        assert!(load.contains(&(c.callback, XrefKind::Data)), "{:#x}: {load:?}", c.load);
        assert!(load.contains(&(c.slot, XrefKind::Read)), "{:#x}: {load:?}", c.load);
        let forms = from(idx, c.forms);
        assert!(forms.contains(&(c.string, XrefKind::Data)), "{:#x}: {forms:?}", c.forms);
        assert_eq!(idx.function_containing(c.load), Some(sw.function), "{:#x}", c.load);
    }
    let calls = idx
        .refs_from_function(sw.function)
        .iter()
        .filter(|r| r.to == SET_TIMEOUT && r.kind == XrefKind::Call)
        .count();
    assert_eq!(calls, sw.cases.len());
}

/// The dispatch jumps to each case, never to the `b default` after it, and no
/// case body is a function of its own.
fn assert_dispatch_rows(idx: &XrefIndex, sw: &Switch) {
    let mut jumps: Vec<u64> = from(idx, sw.dispatch)
        .into_iter()
        .filter(|&(_, kind)| kind == XrefKind::Jump)
        .map(|(to, _)| to)
        .collect();
    jumps.sort_unstable();
    let mut bodies: Vec<u64> = sw.cases.iter().map(|c| c.body).collect();
    bodies.sort_unstable();
    assert_eq!(jumps, bodies);
    assert!(idx.refs_to(sw.dispatch + 4).is_empty(), "{:?}", idx.refs_to(sw.dispatch + 4));
    for c in &sw.cases {
        assert!(!idx.is_function_entry(c.body), "{:#x}", c.body);
        assert_eq!(idx.function_containing(c.body), Some(sw.function), "{:#x}", c.body);
    }
    let table = idx.switch_tables().iter().find(|t| t.dispatch == sw.dispatch).expect("switch read");
    assert_eq!((table.cases, table.read), (Some(5), 5));
}

#[test]
fn an_add_pc_branch_table_walks_every_case_with_the_dispatch_base() {
    let idx = index();
    assert_cases_walked(&idx, &ADD);
    assert_dispatch_rows(&idx, &ADD);
}

#[test]
fn an_ldr_pc_word_table_walks_every_case_with_the_dispatch_base() {
    let idx = index();
    assert_cases_walked(&idx, &LDR);
    assert_dispatch_rows(&idx, &LDR);
}

/// The table words are data the dispatch reads, not code and not pointers.
#[test]
fn an_ldr_pc_table_word_is_neither_code_nor_a_pointer() {
    let idx = index();
    for word in LDR_TABLE {
        assert!(idx.refs_to(word).is_empty(), "{word:#x}: {:?}", idx.refs_to(word));
        assert!(idx.refs_from_instruction(word).is_empty(), "{word:#x}");
    }
}

/// A dispatch decoded before its guard is read again once the guard is, and
/// its cases are walked as the dispatcher's own code.
#[test]
fn an_a32_dispatch_reached_before_its_guard_walks_every_case() {
    let idx = index();
    let mut jumps: Vec<u64> = from(idx, EARLY_DISPATCH)
        .into_iter()
        .filter(|&(_, kind)| kind == XrefKind::Jump)
        .map(|(to, _)| to)
        .collect();
    jumps.sort_unstable();
    assert_eq!(jumps, EARLY_SLOTS);
    let table = idx.switch_tables().iter().find(|t| t.dispatch == EARLY_DISPATCH).expect("switch read");
    assert_eq!((table.cases, table.read), (Some(5), 5));
    for slot in EARLY_SLOTS {
        assert!(!idx.is_function_entry(slot), "{slot:#x}");
        assert_eq!(idx.function_containing(slot), Some(EARLY_FUNCTION), "{slot:#x}");
    }
}

/// A table longer than `jumptablemax` reads that many cases and says how many
/// its guard states.
#[test]
fn a_clamped_a32_table_states_its_guard_count() {
    let idx = build(|arch| arch.max_jumptable_size = 3);
    for sw in [&ADD, &LDR] {
        let table = idx.switch_tables().iter().find(|t| t.dispatch == sw.dispatch).expect("switch read");
        assert_eq!((table.cases, table.read, table.truncated), (Some(5), 3, true), "{:#x}", sw.dispatch);
    }
}

/// A table read gives no case under a `jumptablemax` below two entries, so it
/// states no count and leaves the dispatch to the other readers.
#[test]
fn an_unreadable_a32_table_states_no_count() {
    let idx = build(|arch| arch.max_jumptable_size = 1);
    assert!(idx.switch_tables().iter().all(|t| t.dispatch != LDR.dispatch), "{:?}", idx.switch_tables());
    for c in &LDR.cases {
        assert!(idx.refs_to(c.body).iter().all(|r| r.from != LDR.dispatch), "{:#x}", c.body);
    }
}

/// `armframes off` restores the walk without A32 tables: the dispatch jumps to
/// the `b default` after it and to no case, and no table is read.
#[test]
fn armframes_off_reads_no_a32_table() {
    let idx = build(|arch| {
        arch.set_kuna_option("armframes", "off").expect("armframes flips off");
    });
    for sw in [&ADD, &LDR] {
        let jumps: Vec<(u64, XrefKind)> =
            from(&idx, sw.dispatch).into_iter().filter(|&(_, kind)| kind == XrefKind::Jump).collect();
        assert_eq!(jumps, vec![(sw.dispatch + 4, XrefKind::Jump)], "{:#x}", sw.dispatch);
        assert!(idx.switch_tables().iter().all(|t| t.dispatch != sw.dispatch), "{:#x}", sw.dispatch);
        for c in &sw.cases {
            assert!(idx.refs_to(c.body).iter().all(|r| r.from != sw.dispatch), "{:#x}", c.body);
        }
    }
}

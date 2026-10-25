//! End-to-end gate for A32 jump tables in the reference walk.
//!
//! Fixture: `branchtable_arm_le32` (built from `branchtable_arm_le32.c` with
//! GCC). `add_dispatch` switches through `addls pc,pc,r5,lsl #2` and a table of
//! branches, `ldr_dispatch` through `ldrls pc,[pc,r5,lsl #2]` and a table of
//! words. Both make the GOT base in r6 before the dispatch, and every case
//! reads a callback's slot through it, calls `set_timeout` and forms a string.

use std::path::PathBuf;

use kuna_analysis::listing::xrefs::{self, XrefIndex, XrefKind};
use kuna_console::engine::bootstrap_from_object;

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
        case(0x1012c, 0x10148, 0x2255c, 0x10098, 0x10160, 0x2147c),
        case(0x10130, 0x101c0, 0x22560, 0x100a8, 0x101d8, 0x21488),
        case(0x10134, 0x101ec, 0x22558, 0x100b8, 0x10204, 0x21494),
        case(0x10138, 0x10218, 0x22554, 0x100c8, 0x10230, 0x214a0),
        case(0x1013c, 0x10184, 0x2254c, 0x100d8, 0x1019c, 0x214ac),
    ],
};

/// `ldr` form: the cases are the words after `b default`.
const LDR: Switch = Switch {
    function: 0x10274,
    dispatch: 0x1029c,
    cases: [
        case(0x102b8, 0x102c0, 0x22560, 0x100a8, 0x102d8, 0x214e4),
        case(0x10330, 0x10338, 0x22558, 0x100b8, 0x10350, 0x214f0),
        case(0x1035c, 0x10364, 0x22554, 0x100c8, 0x1037c, 0x214fc),
        case(0x10388, 0x10390, 0x2254c, 0x100d8, 0x103a8, 0x21508),
        case(0x102f4, 0x102fc, 0x22550, 0x100e8, 0x10314, 0x21514),
    ],
};

/// The `ldr` form's table words, `.word case0` to `.word case4`.
const LDR_TABLE: [u64; 5] = [0x102a4, 0x102a8, 0x102ac, 0x102b0, 0x102b4];

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn index() -> XrefIndex {
    let bin = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/branchtable_arm_le32");
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

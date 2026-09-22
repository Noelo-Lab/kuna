//! Unit tests for the caller-to-callee decision. The end-to-end witness -- a
//! callee decompiled again with the type its one caller passes -- is the
//! two-pass stage test `tests/stages/kuna-calleevote.xml` and the CLI probe.

use super::*;

use kuna_base::space::{spacetype, AddrSpace};

use crate::dtype::{flags, sub_metatype, DatatypeKind};

thread_local! {
    static REG: Rc<AddrSpace> =
        Rc::new(AddrSpace::new(spacetype::IPTR_PROCESSOR, "register", false, 8, 1, 1, 0, 0, 0));
}

fn reg() -> Rc<AddrSpace> {
    REG.with(Rc::clone)
}

fn at(space: &Rc<AddrSpace>, off: uintb) -> Address {
    Address::new(Rc::clone(space), off)
}

fn ptr_to(ptrto: Rc<Datatype>) -> Rc<Datatype> {
    let mut p = Datatype::new(8, type_metatype::TYPE_PTR);
    p.submeta = sub_metatype::SUB_PTR;
    p.kind = DatatypeKind::Pointer { ptrto, spaceid: None, truncate: None, wordsize: 1 };
    Rc::new(p)
}

fn record(name: &str) -> Rc<Datatype> {
    let mut t = Datatype::new(0x18, type_metatype::TYPE_STRUCT);
    t.name = name.to_string();
    Rc::new(t)
}

/// A synthesized record with one field, `field_0x10`, as `structsynth` mints
/// it for a closed getter.
fn lone(name: &str) -> Rc<Datatype> {
    let mut t = Datatype::new(0x18, type_metatype::TYPE_STRUCT);
    t.name = name.to_string();
    t.kind = DatatypeKind::Struct {
        field: vec![crate::dtype::TypeField {
            ident: 0,
            offset: 0x10,
            name: "field_0x10".to_string(),
            field_type: long8(),
        }],
        bitfield: Vec::new(),
    };
    Rc::new(t)
}

fn char1() -> Rc<Datatype> {
    let mut t = Datatype::new(1, type_metatype::TYPE_INT);
    t.flags |= flags::chartype;
    Rc::new(t)
}

fn voidt() -> Rc<Datatype> {
    Rc::new(Datatype::new(0, type_metatype::TYPE_VOID))
}

fn long8() -> Rc<Datatype> {
    Rc::new(Datatype::new(8, type_metatype::TYPE_INT))
}

fn typed(space: &Rc<AddrSpace>, off: uintb, ct: &Rc<Datatype>) -> Typed {
    Typed { addr: at(space, off), size: 8, ct: Rc::clone(ct), frame: false }
}

const CALLEE: (int4, uintb) = (1, 0x5c70);

/// A ledger holding one callee with a `void *` parameter in `rdi` and one call
/// from each caller, each passing `passed[i]` there.
fn ledger(passed: &[(uintb, Rc<Datatype>)]) -> Ledger {
    let r = reg();
    let mut l = Ledger::default();
    l.own.insert(CALLEE, vec![typed(&r, 0x38, &ptr_to(voidt()))]);
    let sites = passed
        .iter()
        .map(|(caller, ct)| CallSite { caller: *caller, at: caller + 5, args: vec![Some(typed(&r, 0x38, ct))] })
        .collect();
    l.sites.insert(CALLEE, sites);
    l
}

fn all_calls(l: &Ledger) -> impl Fn((int4, uintb)) -> Option<Vec<uintb>> + '_ {
    move |k| Some(l.sites.get(&k).map(|s| s.iter().map(|c| c.at).collect()).unwrap_or_default())
}

#[test]
fn option_parses_the_three_values() {
    assert_eq!(OptionCalleeVote.apply("off").unwrap().0, CalleeVoteMode::Off);
    assert_eq!(OptionCalleeVote.apply("types").unwrap().0, CalleeVoteMode::Types);
    assert_eq!(OptionCalleeVote.apply("fields").unwrap().0, CalleeVoteMode::Fields);
    assert!(OptionCalleeVote.apply("on").is_err());
    assert!(!CalleeVoteMode::Off.is_on() && CalleeVoteMode::Types.is_on());
    assert!(CalleeVoteMode::Fields.fields() && !CalleeVoteMode::Types.fields());
}

#[test]
fn a_record_or_a_character_pointer_is_committed_and_void_is_not() {
    assert!(committed(&ptr_to(record("struct_2"))));
    assert!(committed(&ptr_to(record("FILE"))));
    assert!(committed(&ptr_to(char1())));
    assert!(committed(&ptr_to(ptr_to(char1()))));
    assert!(!committed(&ptr_to(voidt())));
    assert!(!committed(&ptr_to(long8())));
    assert!(!committed(&ptr_to(record(""))));
    assert!(!committed(&long8()));
}

#[test]
fn only_a_bare_pointer_or_a_word_is_uncommitted() {
    assert!(uncommitted(&ptr_to(voidt()), 8));
    assert!(uncommitted(&long8(), 8));
    assert!(!uncommitted(&Datatype::new(4, type_metatype::TYPE_INT), 8));
    assert!(!uncommitted(&ptr_to(record("struct_2")), 8));
    assert!(!uncommitted(&ptr_to(char1()), 8));
}

#[test]
fn every_caller_passing_one_record_states_it() {
    let rec = ptr_to(record("struct_2"));
    let l = ledger(&[(0x6600, Rc::clone(&rec)), (0x6800, Rc::clone(&rec))]);
    let got = decide_ledger(&l, 8, &all_calls(&l));
    assert_eq!(got.len(), 1);
    let stated = &got[0].1;
    let ct = &stated.at(&at(&reg(), 0x38), 8).expect("stated for rdi").ct;
    assert!(Rc::ptr_eq(ct, &rec));
    assert!(stated.at(&at(&reg(), 0x30), 8).is_none());
}

#[test]
fn callers_that_disagree_state_nothing() {
    let l = ledger(&[(0x6600, ptr_to(record("struct_2"))), (0x6800, ptr_to(record("struct_3")))]);
    assert!(decide_ledger(&l, 8, &all_calls(&l)).is_empty());
}

#[test]
fn one_caller_passing_void_states_nothing() {
    let l = ledger(&[(0x6600, ptr_to(record("struct_2"))), (0x6800, ptr_to(voidt()))]);
    assert!(decide_ledger(&l, 8, &all_calls(&l)).is_empty());
}

#[test]
fn a_call_the_graph_has_and_no_decompile_recorded_states_nothing() {
    let l = ledger(&[(0x6600, ptr_to(record("struct_2")))]);
    let more = |_k: (int4, uintb)| Some(vec![0x6605, 0x7005]);
    assert!(decide_ledger(&l, 8, &more).is_empty());
}

#[test]
fn an_escaping_address_states_nothing() {
    let l = ledger(&[(0x6600, ptr_to(record("struct_2")))]);
    assert!(decide_ledger(&l, 8, &|_k| None).is_empty());
}

#[test]
fn a_recursive_call_is_not_a_vote() {
    let rec = ptr_to(record("struct_2"));
    let mut l = ledger(&[(0x6600, Rc::clone(&rec))]);
    let r = reg();
    l.sites.get_mut(&CALLEE).unwrap().push(CallSite {
        caller: CALLEE.1,
        at: CALLEE.1 + 0x20,
        args: vec![Some(typed(&r, 0x38, &ptr_to(voidt())))],
    });
    let expect = |_k: (int4, uintb)| Some(vec![0x6605]);
    assert_eq!(decide_ledger(&l, 8, &expect).len(), 1);
}

#[test]
fn a_different_storage_states_nothing() {
    let r = reg();
    let mut l = ledger(&[]);
    l.sites.insert(
        CALLEE,
        vec![CallSite { caller: 0x6600, at: 0x6605, args: vec![Some(typed(&r, 0x30, &ptr_to(record("struct_2"))))] }],
    );
    assert!(decide_ledger(&l, 8, &all_calls(&l)).is_empty());
}

#[test]
fn a_committed_parameter_is_never_voted_on() {
    let mut l = ledger(&[(0x6600, ptr_to(record("struct_2")))]);
    let r = reg();
    l.own.insert(CALLEE, vec![typed(&r, 0x38, &ptr_to(char1()))]);
    assert!(decide_ledger(&l, 8, &all_calls(&l)).is_empty());
}

#[test]
fn an_unchanged_statement_is_not_decided_again() {
    let rec = ptr_to(record("struct_2"));
    let mut l = ledger(&[(0x6600, Rc::clone(&rec))]);
    let first = decide_ledger(&l, 8, &all_calls(&l));
    for (k, st) in first {
        l.stated.insert(k, Rc::new(st));
    }
    assert!(decide_ledger(&l, 8, &all_calls(&l)).is_empty());
}

#[test]
fn a_forgotten_function_keeps_no_statement() {
    let rec = ptr_to(record("struct_2"));
    let mut l = ledger(&[(0x6600, Rc::clone(&rec))]);
    let first = decide_ledger(&l, 8, &all_calls(&l));
    for (k, st) in first {
        l.stated.insert(k, Rc::new(st));
    }
    l.forget(CALLEE);
    assert!(!l.stated.contains_key(&CALLEE));
    assert!(decide_ledger(&l, 8, &all_calls(&l)).is_empty());
}

#[test]
fn a_first_statement_that_repeats_the_callees_record_is_not_made() {
    let rec = ptr_to(lone("struct_7"));
    assert!(uncommitted(&rec, 8) && committed(&rec));
    let mut l = ledger(&[(0x6600, Rc::clone(&rec)), (0x6800, Rc::clone(&rec))]);
    let r = reg();
    l.own.insert(CALLEE, vec![typed(&r, 0x38, &rec)]);
    assert!(decide_ledger(&l, 8, &all_calls(&l)).is_empty());
}

#[test]
fn a_callers_record_replaces_the_callees_lone_record() {
    let mine = ptr_to(lone("struct_7"));
    let theirs = ptr_to(record("struct_9"));
    let mut l = ledger(&[(0x6600, Rc::clone(&theirs))]);
    let r = reg();
    l.own.insert(CALLEE, vec![typed(&r, 0x38, &mine)]);
    let got = decide_ledger(&l, 8, &all_calls(&l));
    assert_eq!(got.len(), 1);
    assert!(Rc::ptr_eq(&got[0].1.at(&at(&r, 0x38), 8).expect("stated").ct, &theirs));
}

#[test]
fn a_statement_naming_a_superseded_record_is_forgotten() {
    let rec = ptr_to(record("struct_2"));
    let mut l = ledger(&[(0x6600, Rc::clone(&rec))]);
    let first = decide_ledger(&l, 8, &all_calls(&l));
    for (k, st) in first {
        l.stated.insert(k, Rc::new(st));
    }
    l.forget_naming(&["struct_20".to_string()]);
    assert!(l.stated.contains_key(&CALLEE));
    l.forget_naming(&["struct_2".to_string()]);
    assert!(!l.stated.contains_key(&CALLEE));
}

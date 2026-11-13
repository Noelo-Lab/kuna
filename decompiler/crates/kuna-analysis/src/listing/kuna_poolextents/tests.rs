//! Unit tests for the trusted-reference literal-pool extents.

use std::collections::BTreeMap;

use super::*;
use crate::listing::model::{DiscoveredFunction, FlowType};

struct Image {
    insns: BTreeMap<u64, Insn>,
    funcs: BTreeMap<u64, DiscoveredFunction>,
}

impl Image {
    fn new() -> Self {
        Image { insns: BTreeMap::new(), funcs: BTreeMap::new() }
    }

    fn put(&mut self, insn: Insn) -> &mut Self {
        self.insns.insert(insn.addr, insn);
        self
    }

    fn func(&mut self, entry: u64) -> &mut Self {
        self.funcs.insert(
            entry,
            DiscoveredFunction { entry, name: None, from_symbol: true, has_no_return: false, call_fixup: None },
        );
        self
    }

    fn noreturn(&mut self, entry: u64) -> &mut Self {
        self.func(entry);
        self.funcs.get_mut(&entry).unwrap().has_no_return = true;
        self
    }

    fn listing(self) -> Listing {
        Listing::from_partition(self.insns, self.funcs, vec![(0, 0x1000)])
    }
}

fn plain(addr: u64, mnemonic: &str, operands: &str) -> Insn {
    Insn {
        addr,
        len: 4,
        fall_through: Some(addr + 4),
        flow: FlowType { kind: FlowKind::Fallthrough, has_fallthrough: true, ..Default::default() },
        flows: Vec::new(),
        mnemonic: mnemonic.into(),
        operands: operands.into(),
        pcode: None,
    }
}

fn nop(addr: u64) -> Insn {
    plain(addr, "mov", "r0,r0")
}

/// `ldr rN,[0xADDR]`, the resolved literal form.
fn load(addr: u64, word: u64) -> Insn {
    plain(addr, "ldr", &format!("r3,[0x{word:x}]"))
}

fn ret(addr: u64) -> Insn {
    let mut i = plain(addr, "bx", "lr");
    i.fall_through = None;
    i.flow = FlowType { kind: FlowKind::Return, is_terminal: true, ..Default::default() };
    i
}

fn call(addr: u64, callee: u64) -> Insn {
    let mut i = plain(addr, "bl", &format!("0x{callee:x}"));
    i.flow = FlowType { kind: FlowKind::Call, is_call: true, has_fallthrough: true, ..Default::default() };
    i.flows = vec![callee];
    i
}

fn cond_call(addr: u64, callee: u64) -> Insn {
    let mut i = call(addr, callee);
    i.mnemonic = "blne".into();
    i.flow.is_conditional = true;
    i
}

fn call_indirect(addr: u64) -> Insn {
    let mut i = plain(addr, "blx", "r3");
    i.flow = FlowType {
        kind: FlowKind::ComputedCall,
        is_call: true,
        is_computed: true,
        is_indirect: true,
        has_fallthrough: true,
        ..Default::default()
    };
    i
}

fn svc(addr: u64) -> Insn {
    let mut i = plain(addr, "swi", "0x0");
    i.flow = FlowType { kind: FlowKind::CallOther, has_fallthrough: true, ..Default::default() };
    i
}

fn branch(addr: u64, target: u64) -> Insn {
    let mut i = plain(addr, "b", &format!("0x{target:x}"));
    i.fall_through = None;
    i.flow = FlowType { kind: FlowKind::UnconditionalBranch, is_jump: true, ..Default::default() };
    i.flows = vec![target];
    i
}

fn cbranch(addr: u64, target: u64) -> Insn {
    let mut i = plain(addr, "beq", &format!("0x{target:x}"));
    i.flow = FlowType {
        kind: FlowKind::ConditionalBranch,
        is_jump: true,
        is_conditional: true,
        has_fallthrough: true,
        ..Default::default()
    };
    i.flows = vec![target];
    i
}

/// A PLT stub: `ldr pc,[r12,#off]!`, a computed jump.
fn plt(addr: u64) -> Insn {
    let mut i = plain(addr, "ldr", "pc,[r12,#0xe94]!");
    i.fall_through = None;
    i.flow = FlowType { kind: FlowKind::ComputedJump, is_jump: true, is_computed: true, is_indirect: true, ..Default::default() };
    i
}

fn literal(_: u64, operands: &str) -> Option<u64> {
    crate::aif::kuna_poolentry::absolute_literal_target(operands)
}

fn extents(listing: &Listing) -> Vec<(u64, u64)> {
    pool_extents(listing, &literal, &crate::aif::kuna_poolentry::wide)
}

#[test]
fn a_direct_callee_that_returns_lets_its_caller_fall_through() {
    let mut img = Image::new();
    img.func(0x100).put(call(0x100, 0x200)).put(nop(0x104)).put(ret(0x108));
    img.func(0x200).put(nop(0x200)).put(ret(0x204));
    let l = img.listing();
    let returns = returning_entries(&l);
    assert_eq!(returns.into_iter().collect::<Vec<_>>(), vec![0x100, 0x200]);
    let trusted = trusted_instructions(&l, &returning_entries(&l));
    assert!(trusted.contains(&0x104), "the fall-through of a call to a returning function is code");
}

#[test]
fn a_plt_stub_and_an_indirect_call_never_prove_a_return() {
    let mut img = Image::new();
    img.func(0x100).put(call(0x100, 0x200)).put(nop(0x104)).put(ret(0x108));
    img.func(0x200).put(plt(0x200));
    img.func(0x300).put(call_indirect(0x300)).put(ret(0x304));
    img.func(0x400).put(svc(0x400)).put(ret(0x404));
    let l = img.listing();
    let returns = returning_entries(&l);
    assert!(!returns.contains(&0x200), "a computed jump is not a return");
    assert!(!returns.contains(&0x100), "so its caller is not shown to return past the call");
    assert!(!returns.contains(&0x300), "nor is a function that returns only past an indirect call");
    assert!(!returns.contains(&0x400), "nor one that returns only past a user operation");
    let trusted = trusted_instructions(&l, &returns);
    assert!(trusted.contains(&0x100));
    assert!(!trusted.contains(&0x104), "the fall-through of an unproven call is provisional");
    assert!(!trusted.contains(&0x304));
    assert!(!trusted.contains(&0x404));
}

#[test]
fn a_tail_call_returns_through_its_target_and_recursion_alone_does_not() {
    let mut img = Image::new();
    img.func(0x100).put(branch(0x100, 0x200));
    img.func(0x200).put(ret(0x200));
    img.func(0x300).put(call(0x300, 0x300)).put(ret(0x304));
    img.func(0x400).put(cbranch(0x400, 0x400)).put(call(0x404, 0x400));
    let l = img.listing();
    let returns = returning_entries(&l);
    assert!(returns.contains(&0x100), "a tail branch reaches the target's return");
    assert!(!returns.contains(&0x300), "a return reached only past a call to itself is no proof");
    assert!(!returns.contains(&0x400), "a function whose only exit is a call to itself never returns");
}

#[test]
fn a_conditional_call_and_the_known_no_return_list_are_honoured() {
    let mut img = Image::new();
    img.func(0x100).put(cond_call(0x100, 0x200)).put(ret(0x104));
    img.func(0x200).put(plt(0x200));
    img.func(0x300).put(call(0x300, 0x400)).put(nop(0x304)).put(ret(0x308));
    img.noreturn(0x400).put(ret(0x400));
    let l = img.listing();
    let returns = returning_entries(&l);
    assert!(returns.contains(&0x100), "a conditional call's fall-through is the untaken path");
    assert!(!returns.contains(&0x400), "a listed no-return function never returns");
    let trusted = trusted_instructions(&l, &returns);
    assert!(trusted.contains(&0x104));
    assert!(!trusted.contains(&0x304), "code after a call to a listed no-return function is provisional");
}

#[test]
fn only_trusted_whole_word_loads_witness_a_pool() {
    let mut img = Image::new();
    // f: loads two adjacent words, calls an unproven stub, and the pool follows.
    img.func(0x100)
        .put(load(0x100, 0x110))
        .put(load(0x104, 0x114))
        .put(call(0x108, 0x200))
        .put(nop(0x10c))
        .put(plain(0x110, "andeq", "r0,r0,r0"))
        .put(plain(0x114, "andeq", "r0,r0,r0"));
    img.func(0x200).put(plt(0x200));
    let l = img.listing();
    assert_eq!(extents(&l), vec![(0x110, 0x118)], "junk the walk decoded past the call does not shield the words");

    let mut img = Image::new();
    img.func(0x100).put(load(0x100, 0x120)).put(call(0x104, 0x200)).put(load(0x108, 0x124)).put(ret(0x10c));
    img.func(0x200).put(plt(0x200));
    let l = img.listing();
    assert_eq!(extents(&l), vec![(0x120, 0x124)], "a load after an unproven call witnesses nothing");

    let mut img = Image::new();
    img.func(0x100).put(plain(0x100, "ldrh", "r3,[0x120]")).put(plain(0x104, "ldrb", "r3,[0x124]")).put(ret(0x108));
    let l = img.listing();
    assert!(extents(&l).is_empty(), "a byte or halfword read proves nothing");
}

#[test]
fn a_pool_after_a_returning_call_needs_a_load_that_crosses_no_call() {
    // The entry's own load reads the word after `bl sink`: data, however `sink` returns.
    let mut img = Image::new();
    img.func(0x100).put(load(0x100, 0x108)).put(call(0x104, 0x200)).put(plain(0x108, "andeq", "r0,r0,r0")).put(ret(0x10c));
    img.func(0x200).put(ret(0x200));
    let l = img.listing();
    assert_eq!(extents(&l), vec![(0x108, 0x10c)], "the strict witness sees the pool past a returning call");

    // The only load sits past a returning call, and so does the word: reached code.
    let mut img = Image::new();
    img.func(0x100)
        .put(call(0x100, 0x200))
        .put(load(0x104, 0x10c))
        .put(call(0x108, 0x200))
        .put(plain(0x10c, "andeq", "r0,r0,r0"))
        .put(ret(0x110));
    img.func(0x200).put(ret(0x200));
    let l = img.listing();
    assert!(extents(&l).is_empty(), "no strict witness, and the word is covered by a trusted instruction");

    // Junk past a returning call that spells a load of live code past another
    // returning call: the live code is covered, and the junk is no strict witness.
    let mut img = Image::new();
    img.func(0x100)
        .put(load(0x100, 0x11c))
        .put(cbranch(0x104, 0x114))
        .put(call(0x108, 0x200))
        .put(plain(0x10c, "add", "r0,r0,#0x1"))
        .put(ret(0x110))
        .put(call(0x114, 0x200))
        .put(call(0x118, 0x200))
        .put(load(0x11c, 0x10c));
    img.func(0x200).put(ret(0x200));
    let l = img.listing();
    assert_eq!(extents(&l), vec![(0x11c, 0x120)], "only the real pool word, never the live code it names");
}

#[test]
fn a_load_some_load_reads_as_data_witnesses_nothing() {
    // f's pool after a returning call (`__builtin_unreachable()` follows it) is
    // trusted under the first notion; its word spells a load of the live code
    // past f's unproven call, which nothing trusted covers.
    let mut img = Image::new();
    img.func(0x100)
        .put(load(0x100, 0x114))
        .put(cbranch(0x104, 0x110))
        .put(call(0x108, 0x300))
        .put(ret(0x10c))
        .put(call(0x110, 0x200))
        .put(load(0x114, 0x10c));
    img.func(0x200).put(ret(0x200));
    img.func(0x300).put(plt(0x300));
    let l = img.listing();
    let trusted = trusted_instructions(&l, &returning_entries(&l));
    assert!(trusted.contains(&0x114) && !trusted.contains(&0x10c));
    assert_eq!(extents(&l), vec![(0x114, 0x118)], "the pool word is no witness of the live code it spells");
}

#[test]
fn a_load_separated_from_its_word_by_a_function_entry_witnesses_nothing() {
    // A discovered entry on a pool word whose junk spells a load of the live
    // code past an earlier function's unproven call, and one spelling a load
    // forward across the next function's entry.
    let mut img = Image::new();
    img.func(0x100).put(call(0x100, 0x300)).put(plain(0x104, "mov", "r4,r0")).put(ret(0x108));
    img.func(0x10c).put(load(0x10c, 0x104)).put(ret(0x110));
    img.func(0x114).put(load(0x114, 0x124)).put(ret(0x118));
    img.func(0x11c).put(call(0x11c, 0x300)).put(plain(0x120, "mov", "r4,r0")).put(plain(0x124, "mov", "r5,r0")).put(ret(0x128));
    img.func(0x300).put(plt(0x300));
    let l = img.listing();
    assert!(extents(&l).is_empty(), "a pool belongs to the function that loads it");
}

#[test]
fn a_word_that_is_trusted_code_or_a_label_is_never_a_pool_word() {

    // A branch target that something also reads.
    let mut img = Image::new();
    img.func(0x100).put(load(0x100, 0x10c)).put(cbranch(0x104, 0x10c)).put(ret(0x108)).put(ret(0x10c));
    let l = img.listing();
    assert!(extents(&l).is_empty(), "a trusted branch target is a label");

    // A 32-bit Thumb instruction that starts two bytes before the word covers it.
    let mut img = Image::new();
    let mut wide32 = plain(0x106, "mov.w", "r0,#1");
    wide32.len = 4;
    wide32.fall_through = Some(0x10a);
    img.func(0x100).put(load(0x100, 0x108)).put(plain(0x104, "nop", "")).put(wide32).put(ret(0x10a));
    img.insns.get_mut(&0x104).unwrap().len = 2;
    img.insns.get_mut(&0x104).unwrap().fall_through = Some(0x106);
    let l = img.listing();
    assert!(extents(&l).is_empty(), "coverage, not start, decides");
}

#[test]
fn the_svc_shape_keeps_the_live_fall_through() {
    // f: push; cmp; beq err; bl sink; live: mov r0,#5; pop; err: mov r7,#1; svc; .word (decodes as ldr r2,[live])
    let mut img = Image::new();
    img.func(0x100)
        .put(cbranch(0x100, 0x110))
        .put(call(0x104, 0x200))
        .put(plain(0x108, "mov", "r0,#0x5"))
        .put(ret(0x10c))
        .put(plain(0x110, "mov", "r7,#0x1"))
        .put(svc(0x114))
        .put(load(0x118, 0x108));
    img.func(0x200).put(ret(0x200));
    let l = img.listing();
    let trusted = trusted_instructions(&l, &returning_entries(&l));
    assert!(trusted.contains(&0x108), "the fall-through of a call to a returning function");
    assert!(!trusted.contains(&0x118), "the word after a user operation is provisional");
    assert!(extents(&l).is_empty(), "its load of the live instruction is ignored");
}

#[test]
fn a_wide_literal_covers_two_words_and_runs_are_exactly_adjacent() {
    let mut img = Image::new();
    img.func(0x100)
        .put(plain(0x100, "vldr.64", "d0,[0x120]"))
        .put(load(0x104, 0x12c))
        .put(load(0x108, 0x130))
        .put(ret(0x10c));
    let l = img.listing();
    assert_eq!(extents(&l), vec![(0x120, 0x128), (0x12c, 0x134)]);
    assert!(is_word_load("ldr") && is_word_load("ldreq") && is_word_load("ldrd") && is_word_load("ldrdne"));
    assert!(is_word_load("ldr.w") && is_word_load("vldr.32") && is_word_load("vldrhi.64"));
    assert!(!is_word_load("ldrh") && !is_word_load("ldrhhi") && !is_word_load("ldrsb") && !is_word_load("ldrex"));
    assert!(!is_word_load("ldm") && !is_word_load("ldrbt") && !is_word_load("str"));
}

//! Tests for the big-endian join order: how the second register's value at a
//! RETURN is classified, on a hand-built `Funcdata`. The end-to-end witnesses
//! are the compiled round trips in `kuna-cli/tests/decompile_all_cli.rs`.

use super::*;

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace};

use crate::context::{ArchContext, TypeOp};

/// The first return register (the high word) and the second (the low word).
const HI: u64 = 0x10;
const LO: u64 = 0x14;

fn build_fd() -> Funcdata {
    let mut m = AddrSpaceManager::new();
    m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
    m.insert_space(Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "register",
        true,
        4,
        1,
        2,
        addrspace_flags::hasphysical,
        1,
        1,
    )))
    .unwrap();
    let glb = Rc::new(ArchContext::new(m));
    let reg = Rc::clone(glb.manage().get_space_by_name("register").unwrap());
    Funcdata::new("func", "func", glb, Address::new(reg, 0x1000), 0x1000_0000, 0x40).unwrap()
}

fn at(fd: &Funcdata, off: u64) -> Address {
    Address::new(Rc::clone(fd.get_arch().manage().get_space_by_name("register").unwrap()), off)
}

fn pair(fd: &Funcdata) -> Pair {
    Pair { lo_slot: 2, hi_slot: 1, lo_size: 4, own: at(fd, LO), hi_own: at(fd, HI), chosen_flag: false }
}

/// The function's input in the register at `off`.
fn input(fd: &mut Funcdata, off: u64) -> VarnodeId {
    let a = at(fd, off);
    let vn = fd.new_varnode(4, &a, None);
    fd.set_input_varnode(vn).expect("input")
}

/// A live `<opc>(inputs)` in the function's one basic block.
fn op(fd: &mut Funcdata, opc: OpCode, inputs: &[VarnodeId]) -> OpId {
    let a = at(fd, 0x2000);
    let op = fd.new_op(inputs.len() as int4, a);
    fd.op_set_opcode(op, TypeOp::new(opc, 0, format!("{opc:?}")));
    for (i, &vn) in inputs.iter().enumerate() {
        fd.op_set_input(op, vn, i as int4).expect("input");
    }
    let root = fd.bblocks_ref().root.expect("bblocks root");
    let bl = match fd.bblocks_ref().block(root).get_size() {
        0 => fd.bblocks_mut().new_block_basic(root),
        _ => fd.bblocks_ref().block(root).get_block(0),
    };
    fd.op_insert(op, bl, None);
    op
}

/// `out = <opc>(inputs)` with `out` a `size`-byte register at `off`.
fn def(fd: &mut Funcdata, opc: OpCode, inputs: &[VarnodeId], off: u64, size: int4) -> VarnodeId {
    let o = op(fd, opc, inputs);
    let a = at(fd, off);
    fd.new_varnode_out(size, &a, o).expect("output")
}

fn k(fd: &mut Funcdata, v: u64) -> VarnodeId {
    fd.new_constant(4, v)
}

fn ret(fd: &mut Funcdata, high: VarnodeId, low: VarnodeId) -> OpId {
    let a = k(fd, 0);
    op(fd, OpCode::CPUI_RETURN, &[a, high, low])
}

fn class_of(fd: &mut Funcdata, high: VarnodeId, low: VarnodeId) -> LowWord {
    let r = ret(fd, high, low);
    let p = pair(fd);
    classify(fd, r, &p)
}

#[test]
fn a_product_only_the_return_reads_is_returned() {
    let mut fd = build_fd();
    let (a, b) = (input(&mut fd, 0x20), input(&mut fd, 0x24));
    let lo = def(&mut fd, OpCode::CPUI_INT_MULT, &[a, b], LO, 4);
    let hi = def(&mut fd, OpCode::CPUI_INT_MULT, &[a, b], HI, 4);
    assert_eq!(class_of(&mut fd, hi, lo), LowWord::Returned);
}

#[test]
fn a_literal_zero_proves_nothing() {
    let mut fd = build_fd();
    let a = input(&mut fd, 0x20);
    let zero = k(&mut fd, 0);
    let lo = def(&mut fd, OpCode::CPUI_COPY, &[zero], LO, 4);
    let hi = def(&mut fd, OpCode::CPUI_INT_2COMP, &[a], HI, 4);
    assert_eq!(class_of(&mut fd, hi, lo), LowWord::Zero, "clang -O0 leaves `addiu $3,$zero,0` in an int function");
}

#[test]
fn a_literal_zero_built_in_two_steps_is_still_zero() {
    let mut fd = build_fd();
    let a = input(&mut fd, 0x20);
    let (z1, z2) = (k(&mut fd, 0), k(&mut fd, 0));
    let lo = def(&mut fd, OpCode::CPUI_INT_ADD, &[z1, z2], LO, 4);
    let hi = def(&mut fd, OpCode::CPUI_INT_2COMP, &[a], HI, 4);
    assert_eq!(class_of(&mut fd, hi, lo), LowWord::Zero);
}

#[test]
fn a_nonzero_literal_only_the_return_reads_is_returned() {
    let mut fd = build_fd();
    let (ten, zero) = (k(&mut fd, 10), k(&mut fd, 0));
    let lo = def(&mut fd, OpCode::CPUI_COPY, &[ten], LO, 4);
    let hi = def(&mut fd, OpCode::CPUI_COPY, &[zero], HI, 4);
    assert_eq!(class_of(&mut fd, hi, lo), LowWord::Returned, "`return 10LL`");
}

#[test]
fn the_register_left_as_it_arrived_is_a_leftover() {
    let mut fd = build_fd();
    let a = input(&mut fd, 0x20);
    let lo = input(&mut fd, LO);
    let hi = def(&mut fd, OpCode::CPUI_INT_2COMP, &[a], HI, 4);
    assert_eq!(class_of(&mut fd, hi, lo), LowWord::Entry);
}

#[test]
fn the_register_moved_away_and_back_is_returned() {
    let mut fd = build_fd();
    let entry = input(&mut fd, LO);
    let saved = def(&mut fd, OpCode::CPUI_COPY, &[entry], 0x30, 4);
    let lo = def(&mut fd, OpCode::CPUI_COPY, &[saved], LO, 4);
    let zero = k(&mut fd, 0);
    let hi = def(&mut fd, OpCode::CPUI_COPY, &[zero], HI, 4);
    assert_eq!(
        class_of(&mut fd, hi, lo),
        LowWord::Returned,
        "ARM's `mov r4,r1; bl ext; mov r1,r4` carries the argument into the low word on purpose",
    );
}

#[test]
fn a_value_also_stored_is_scratch() {
    let mut fd = build_fd();
    let (a, b, p) = (input(&mut fd, 0x20), input(&mut fd, 0x24), input(&mut fd, 0x28));
    let lo = def(&mut fd, OpCode::CPUI_INT_ADD, &[a, b], LO, 4);
    let space = k(&mut fd, 0);
    op(&mut fd, OpCode::CPUI_STORE, &[space, p, lo]);
    let hi = def(&mut fd, OpCode::CPUI_INT_MULT, &[a, b], HI, 4);
    assert_eq!(class_of(&mut fd, hi, lo), LowWord::Scratch);
}

#[test]
fn a_value_used_as_an_address_is_scratch() {
    let mut fd = build_fd();
    let (a, b) = (input(&mut fd, 0x20), input(&mut fd, 0x24));
    let lo = def(&mut fd, OpCode::CPUI_INT_ADD, &[a, b], LO, 4);
    let space = k(&mut fd, 0);
    let hi = def(&mut fd, OpCode::CPUI_LOAD, &[space, lo], HI, 4);
    assert_eq!(class_of(&mut fd, hi, lo), LowWord::Scratch, "`__pgetc` keeps a pointer it read through in %i1");
}

#[test]
fn a_value_the_first_register_is_computed_from_is_scratch() {
    let mut fd = build_fd();
    let (a, b) = (input(&mut fd, 0x20), input(&mut fd, 0x24));
    let lo = def(&mut fd, OpCode::CPUI_INT_MULT, &[a, b], LO, 4);
    let hi = def(&mut fd, OpCode::CPUI_INT_ADD, &[lo, a], HI, 4);
    assert_eq!(class_of(&mut fd, hi, lo), LowWord::Scratch);
}

#[test]
fn the_same_value_in_both_registers_is_scratch() {
    let mut fd = build_fd();
    let (a, b) = (input(&mut fd, 0x20), input(&mut fd, 0x24));
    let lo = def(&mut fd, OpCode::CPUI_INT_MULT, &[a, b], LO, 4);
    let hi = def(&mut fd, OpCode::CPUI_COPY, &[lo], HI, 4);
    assert_eq!(class_of(&mut fd, hi, lo), LowWord::Scratch, "either join reads the same value");
}

#[test]
fn the_sign_of_the_low_word_may_build_the_first_register() {
    let mut fd = build_fd();
    let (a, b) = (input(&mut fd, 0x20), input(&mut fd, 0x24));
    let lo = def(&mut fd, OpCode::CPUI_INT_MULT, &[a, b], LO, 4);
    let bits = k(&mut fd, 31);
    let hi = def(&mut fd, OpCode::CPUI_INT_SRIGHT, &[lo, bits], HI, 4);
    assert_eq!(class_of(&mut fd, hi, lo), LowWord::Returned, "`(long long)x`");
}

#[test]
fn a_carry_out_of_the_low_word_may_build_the_first_register() {
    let mut fd = build_fd();
    let (ah, al) = (input(&mut fd, 0x20), input(&mut fd, 0x24));
    let one = k(&mut fd, 1);
    let lo = def(&mut fd, OpCode::CPUI_INT_ADD, &[al, one], LO, 4);
    let carry = def(&mut fd, OpCode::CPUI_INT_LESS, &[lo, al], 0x40, 1);
    let widened = def(&mut fd, OpCode::CPUI_INT_ZEXT, &[carry], 0x44, 4);
    let hi = def(&mut fd, OpCode::CPUI_INT_ADD, &[ah, widened], HI, 4);
    assert_eq!(class_of(&mut fd, hi, lo), LowWord::Returned, "MIPS adds a `long long` with `sltu`");
}

#[test]
fn the_halves_of_one_wide_value_are_wide() {
    let mut fd = build_fd();
    let (space, p) = (k(&mut fd, 0), input(&mut fd, 0x20));
    let whole = def(&mut fd, OpCode::CPUI_LOAD, &[space, p], 0x50, 8);
    let (z, four) = (k(&mut fd, 0), k(&mut fd, 4));
    let lo = def(&mut fd, OpCode::CPUI_SUBPIECE, &[whole, z], LO, 4);
    let hi = def(&mut fd, OpCode::CPUI_SUBPIECE, &[whole, four], HI, 4);
    let space2 = k(&mut fd, 0);
    op(&mut fd, OpCode::CPUI_STORE, &[space2, p, lo]);
    assert_eq!(class_of(&mut fd, hi, lo), LowWord::Wide, "an 8-byte value is returned whole, whatever else reads it");
}

#[test]
fn swapped_halves_are_not_one_value() {
    let mut fd = build_fd();
    let (space, p) = (k(&mut fd, 0), input(&mut fd, 0x20));
    let whole = def(&mut fd, OpCode::CPUI_LOAD, &[space, p], 0x50, 8);
    let (z, four) = (k(&mut fd, 0), k(&mut fd, 4));
    let hi = def(&mut fd, OpCode::CPUI_SUBPIECE, &[whole, z], HI, 4);
    let lo = def(&mut fd, OpCode::CPUI_SUBPIECE, &[whole, four], LO, 4);
    assert!(!halves_of_one_value(&fd, lo, hi, 4));
}

#[test]
fn literal_values_fold_the_way_sparc_builds_them() {
    let mut fd = build_fd();
    let (hi22, lo10) = (k(&mut fd, 0x9e377800), k(&mut fd, 0x1b9));
    let sethi = def(&mut fd, OpCode::CPUI_COPY, &[hi22], 0x60, 4);
    let or = def(&mut fd, OpCode::CPUI_INT_OR, &[sethi, lo10], 0x64, 4);
    assert_eq!(literal_value(&fd, or, LITERAL_DEPTH), Some(0x9e3779b9));
    let (a, lo10b) = (input(&mut fd, 0x20), k(&mut fd, 0x1b9));
    let not_literal = def(&mut fd, OpCode::CPUI_INT_OR, &[a, lo10b], 0x68, 4);
    assert_eq!(literal_value(&fd, not_literal, LITERAL_DEPTH), None);
}

#[test]
fn a_literal_multiplied_into_the_first_register_is_scratch() {
    let mut fd = build_fd();
    let a = input(&mut fd, 0x20);
    let magic = k(&mut fd, 0xb6db6db7);
    let lo = def(&mut fd, OpCode::CPUI_COPY, &[magic], LO, 4);
    let (wide_lo, wide_a) = (def(&mut fd, OpCode::CPUI_INT_ZEXT, &[lo], 0x50, 8), def(&mut fd, OpCode::CPUI_INT_ZEXT, &[a], 0x58, 8));
    let product = def(&mut fd, OpCode::CPUI_INT_MULT, &[wide_lo, wide_a], 0x60, 8);
    let four = k(&mut fd, 4);
    let hi = def(&mut fd, OpCode::CPUI_SUBPIECE, &[product, four], HI, 4);
    assert_eq!(class_of(&mut fd, hi, lo), LowWord::Scratch, "SPARC's `umul` by a divisor's magic number left in %i1");
}

#[test]
fn the_high_half_of_the_low_words_extension_may_build_the_first_register() {
    let mut fd = build_fd();
    let (a, b) = (input(&mut fd, 0x20), input(&mut fd, 0x24));
    let lo = def(&mut fd, OpCode::CPUI_INT_MULT, &[a, b], LO, 4);
    let wide = def(&mut fd, OpCode::CPUI_INT_SEXT, &[lo], 0x50, 8);
    let four = k(&mut fd, 4);
    let hi = def(&mut fd, OpCode::CPUI_SUBPIECE, &[wide, four], HI, 4);
    assert_eq!(class_of(&mut fd, hi, lo), LowWord::Returned, "`(long long)(a * b)`");
}

#[test]
fn a_register_pair_heritage_split_is_followed_to_the_entry_value() {
    let mut fd = build_fd();
    let a = input(&mut fd, 0x20);
    let at_pair = at(&fd, HI);
    let pair_in = fd.new_varnode(8, &at_pair, None);
    let pair_in = fd.set_input_varnode(pair_in).expect("input");
    let zero = k(&mut fd, 0);
    let lo = def(&mut fd, OpCode::CPUI_SUBPIECE, &[pair_in, zero], LO, 4);
    let hi = def(&mut fd, OpCode::CPUI_INT_2COMP, &[a], HI, 4);
    assert_eq!(
        class_of(&mut fd, hi, lo),
        LowWord::Entry,
        "the low half of the %i0:%i1 pair as it arrived is the second register's entry value",
    );
}

#[test]
fn a_product_split_by_a_shift_is_one_wide_value() {
    let mut fd = build_fd();
    let (a, b) = (input(&mut fd, 0x20), input(&mut fd, 0x24));
    let (wa, wb) = (def(&mut fd, OpCode::CPUI_INT_SEXT, &[a], 0x50, 8), def(&mut fd, OpCode::CPUI_INT_SEXT, &[b], 0x58, 8));
    let product = def(&mut fd, OpCode::CPUI_INT_MULT, &[wa, wb], 0x60, 8);
    let (z1, z2, bits) = (k(&mut fd, 0), k(&mut fd, 0), k(&mut fd, 32));
    let lo = def(&mut fd, OpCode::CPUI_SUBPIECE, &[product, z1], LO, 4);
    let shifted = def(&mut fd, OpCode::CPUI_INT_RIGHT, &[product, bits], 0x68, 8);
    let hi = def(&mut fd, OpCode::CPUI_SUBPIECE, &[shifted, z2], HI, 4);
    assert_eq!(class_of(&mut fd, hi, lo), LowWord::Wide, "MIPS `mult $4,$5; mflo $3; mfhi $2`");
}

#[test]
fn a_register_pair_heritage_merges_is_not_one_wide_value() {
    let mut fd = build_fd();
    let (a, b) = (input(&mut fd, 0x20), input(&mut fd, 0x24));
    let first = def(&mut fd, OpCode::CPUI_INT_ADD, &[a, b], 0x70, 4);
    let second = def(&mut fd, OpCode::CPUI_INT_MULT, &[a, b], 0x74, 4);
    let pair = def(&mut fd, OpCode::CPUI_PIECE, &[first, second], 0x70, 8);
    let (z, four) = (k(&mut fd, 0), k(&mut fd, 4));
    let lo = def(&mut fd, OpCode::CPUI_SUBPIECE, &[pair, z], LO, 4);
    let hi = def(&mut fd, OpCode::CPUI_SUBPIECE, &[pair, four], HI, 4);
    assert!(!halves_of_one_value(&fd, lo, hi, 4), "SPARC's %i0:%i1 reassembled holds two values");
}

#[test]
fn the_low_half_of_a_product_that_builds_the_first_register_is_scratch() {
    let mut fd = build_fd();
    let (a, b) = (input(&mut fd, 0x20), input(&mut fd, 0x24));
    let (wb, seven) = (def(&mut fd, OpCode::CPUI_INT_SEXT, &[b], 0x58, 8), k(&mut fd, 7));
    let wide_seven = def(&mut fd, OpCode::CPUI_INT_SEXT, &[seven], 0x50, 8);
    let product = def(&mut fd, OpCode::CPUI_INT_MULT, &[wb, wide_seven], 0x60, 8);
    let z = k(&mut fd, 0);
    let lo = def(&mut fd, OpCode::CPUI_SUBPIECE, &[product, z], LO, 4);
    let hi = def(&mut fd, OpCode::CPUI_INT_ADD, &[lo, a], HI, 4);
    assert_eq!(
        class_of(&mut fd, hi, lo),
        LowWord::Scratch,
        "SPARC `smul %i1,7,%i1; ret; restore %i1,%i0,%o0` returns the int b * 7 + a",
    );
}

#[test]
fn a_register_split_into_bytes_is_followed_to_each_byte() {
    let mut fd = build_fd();
    let a = input(&mut fd, 0x20);
    let entry = input(&mut fd, LO);
    let (three, z) = (k(&mut fd, 1), k(&mut fd, 0));
    let upper = def(&mut fd, OpCode::CPUI_SUBPIECE, &[entry, three], LO, 3);
    let byte = def(&mut fd, OpCode::CPUI_SUBPIECE, &[entry, z], LO + 3, 1);
    let space = k(&mut fd, 0);
    op(&mut fd, OpCode::CPUI_STORE, &[space, a, byte]);
    let lo = def(&mut fd, OpCode::CPUI_PIECE, &[upper, byte], LO, 4);
    let hi = def(&mut fd, OpCode::CPUI_INT_2COMP, &[a], HI, 4);
    assert_eq!(
        class_of(&mut fd, hi, lo),
        LowWord::Entry,
        "`stb %i1` splits %i1 into bytes; reassembled, it is still the entry value",
    );
}

#[test]
fn a_comparison_in_the_first_register_leaves_the_low_word_behind() {
    let mut fd = build_fd();
    let (a, b) = (input(&mut fd, 0x20), input(&mut fd, 0x24));
    let lo = def(&mut fd, OpCode::CPUI_INT_SUB, &[a, b], LO, 4);
    let less = def(&mut fd, OpCode::CPUI_INT_LESS, &[a, b], 0x40, 1);
    let hi = def(&mut fd, OpCode::CPUI_INT_ZEXT, &[less], HI, 4);
    assert_eq!(
        class_of(&mut fd, hi, lo),
        LowWord::Compared,
        "ARM big-endian `return x >= 0x80000000ULL` leaves `0x7fffffff - lo` in r1 next to the flag in r0",
    );
}

#[test]
fn a_carry_alone_in_the_first_register_is_a_comparison() {
    let mut fd = build_fd();
    let (a, b) = (input(&mut fd, 0x20), input(&mut fd, 0x24));
    let lo = def(&mut fd, OpCode::CPUI_INT_ADD, &[a, b], LO, 4);
    let carry = def(&mut fd, OpCode::CPUI_INT_CARRY, &[a, b], 0x40, 1);
    let (zero, widened) = (k(&mut fd, 0), def(&mut fd, OpCode::CPUI_INT_ZEXT, &[carry], 0x44, 4));
    let hi = def(&mut fd, OpCode::CPUI_INT_ADD, &[zero, widened], HI, 4);
    assert_eq!(
        class_of(&mut fd, hi, lo),
        LowWord::Compared,
        "PowerPC -O0 `li 3,0; addc 4,4,5; addze 3,3` returns the int carry",
    );
}

#[test]
fn a_difference_no_borrow_ties_the_first_register_to_was_computed_for_its_flags() {
    let mut fd = build_fd();
    let (a0, a1) = (input(&mut fd, 0x20), input(&mut fd, 0x24));
    let limit = k(&mut fd, 0x7fff_ffff);
    let lo = def(&mut fd, OpCode::CPUI_INT_SUB, &[limit, a1], LO, 4);
    let hi = def(&mut fd, OpCode::CPUI_COPY, &[a0], HI, 4);
    assert_eq!(
        class_of(&mut fd, hi, lo),
        LowWord::Compared,
        "a saturating int keeps the value it returns in r0 and the compare's `0x7fffffff - lo` in r1",
    );
}

#[test]
fn a_difference_whose_borrow_builds_the_first_register_is_returned() {
    let mut fd = build_fd();
    let (ah, al, bh, bl) = (input(&mut fd, 0x20), input(&mut fd, 0x24), input(&mut fd, 0x28), input(&mut fd, 0x2c));
    let lo = def(&mut fd, OpCode::CPUI_INT_SUB, &[al, bl], LO, 4);
    let borrow = def(&mut fd, OpCode::CPUI_INT_LESS, &[al, bl], 0x40, 1);
    let widened = def(&mut fd, OpCode::CPUI_INT_ZEXT, &[borrow], 0x44, 4);
    let high = def(&mut fd, OpCode::CPUI_INT_SUB, &[ah, bh], 0x48, 4);
    let hi = def(&mut fd, OpCode::CPUI_INT_SUB, &[high, widened], HI, 4);
    assert_eq!(class_of(&mut fd, hi, lo), LowWord::Returned, "`subs r1,r1,r3; sbc r0,r0,r2` is a 64-bit difference");
}

#[test]
fn a_sum_next_to_a_literal_high_word_is_returned() {
    let mut fd = build_fd();
    let (a, b) = (input(&mut fd, 0x20), input(&mut fd, 0x24));
    let lo = def(&mut fd, OpCode::CPUI_INT_ADD, &[a, b], LO, 4);
    let zero = k(&mut fd, 0);
    let hi = def(&mut fd, OpCode::CPUI_COPY, &[zero], HI, 4);
    assert_eq!(class_of(&mut fd, hi, lo), LowWord::Returned, "`(unsigned long long)(a + b)`");
}

#[test]
fn the_sign_of_a_sum_may_build_the_first_register() {
    let mut fd = build_fd();
    let (a, b) = (input(&mut fd, 0x20), input(&mut fd, 0x24));
    let lo = def(&mut fd, OpCode::CPUI_INT_ADD, &[a, b], LO, 4);
    let bits = k(&mut fd, 31);
    let hi = def(&mut fd, OpCode::CPUI_INT_SRIGHT, &[lo, bits], HI, 4);
    assert_eq!(class_of(&mut fd, hi, lo), LowWord::Returned, "`(long long)(a + b)`");
}

#[test]
fn adding_zero_is_a_move_not_a_sum() {
    let mut fd = build_fd();
    let (a, b) = (input(&mut fd, 0x20), input(&mut fd, 0x24));
    let zero = k(&mut fd, 0);
    let lo = def(&mut fd, OpCode::CPUI_INT_ADD, &[a, zero], LO, 4);
    let hi = def(&mut fd, OpCode::CPUI_COPY, &[b], HI, 4);
    assert_eq!(class_of(&mut fd, hi, lo), LowWord::Returned, "SPARC's `restore %i0,%g0,%o1` moves %i0 into the low word");
}

#[test]
fn zeros_and_ones_reach_the_mask_through_shifts_and_choices() {
    let mut fd = build_fd();
    let a = input(&mut fd, 0x20);
    let bits = k(&mut fd, 31);
    let sign = def(&mut fd, OpCode::CPUI_INT_RIGHT, &[a, bits], 0x40, 4);
    assert!(computed_flag(&fd, sign), "MIPS gcc -O0 `srl $2,$2,31` for `(long long)x < 0`");
    let (zero, one) = (k(&mut fd, 0), k(&mut fd, 1));
    let chosen = def(&mut fd, OpCode::CPUI_MULTIEQUAL, &[zero, one], 0x44, 4);
    assert!(computed_flag(&fd, chosen), "`mov r0,#0; movlo r0,#1`");
    let one_again = k(&mut fd, 1);
    let literal = def(&mut fd, OpCode::CPUI_COPY, &[one_again], 0x48, 4);
    assert!(!computed_flag(&fd, literal), "`return 1` is a literal, not a comparison");
    let wide = def(&mut fd, OpCode::CPUI_COPY, &[a], 0x4c, 4);
    assert!(!computed_flag(&fd, wide));
}

#[test]
fn the_sign_of_a_small_positive_value_is_a_zero_extension_not_a_flag() {
    let mut fd = build_fd();
    let (small, large) = (k(&mut fd, 0x200), k(&mut fd, 0x400));
    let lo = def(&mut fd, OpCode::CPUI_MULTIEQUAL, &[small, large], LO, 4);
    let bits = k(&mut fd, 31);
    let hi = def(&mut fd, OpCode::CPUI_INT_SRIGHT, &[lo, bits], HI, 4);
    assert!(!computed_flag(&fd, hi), "gcc -O0 sign-extends `c ? 512 : 1024` into a uintmax_t with `sra $2,$3,31`");
    assert_eq!(class_of(&mut fd, hi, lo), LowWord::Returned);
}

#[test]
fn a_leading_zero_count_shifted_to_its_top_bit_is_a_comparison() {
    let mut fd = build_fd();
    let (a, b) = (input(&mut fd, 0x20), input(&mut fd, 0x24));
    let lo = def(&mut fd, OpCode::CPUI_INT_ADD, &[a, b], LO, 4);
    let zeros = def(&mut fd, OpCode::CPUI_LZCOUNT, &[a], 0x40, 4);
    let five = k(&mut fd, 5);
    let hi = def(&mut fd, OpCode::CPUI_INT_RIGHT, &[zeros, five], HI, 4);
    assert!(computed_flag(&fd, hi), "PowerPC `cntlzw 3,3; srwi 3,3,5` is `r3 == 0`");
    assert_eq!(class_of(&mut fd, hi, lo), LowWord::Compared, "`v == (int)v` on PowerPC -O1 leaves `lo + 0x80000000` in r4");
}

#[test]
fn a_boolean_a_branch_chose_leaves_the_low_word_behind() {
    let mut fd = build_fd();
    let (a, b) = (input(&mut fd, 0x20), input(&mut fd, 0x24));
    let lo = def(&mut fd, OpCode::CPUI_INT_ADD, &[a, b], LO, 4);
    let one = k(&mut fd, 1);
    let hi = def(&mut fd, OpCode::CPUI_COPY, &[one], HI, 4);
    let r = ret(&mut fd, hi, lo);
    let chosen = Pair { chosen_flag: true, ..pair(&fd) };
    assert_eq!(classify(&fd, r, &chosen), LowWord::Compared, "SPARC `addcc; addxcc; cmp; be` then `restore %g0,1,%o0`");
    assert_eq!(classify(&fd, r, &pair(&fd)), LowWord::Returned, "one literal high word beside a sum is a `long long`");
}

#[test]
fn a_literal_beside_a_sum_whose_carry_a_branch_tests_is_a_comparison() {
    let mut fd = build_fd();
    let (ah, al) = (input(&mut fd, 0x20), input(&mut fd, 0x24));
    let bias = k(&mut fd, 0x8000_0000);
    let lo = def(&mut fd, OpCode::CPUI_INT_ADD, &[al, bias], LO, 4);
    let bias_again = k(&mut fd, 0x8000_0000);
    let carry = def(&mut fd, OpCode::CPUI_INT_CARRY, &[al, bias_again], 0x40, 1);
    let widened = def(&mut fd, OpCode::CPUI_INT_ZEXT, &[carry], 0x44, 4);
    let top = def(&mut fd, OpCode::CPUI_INT_ADD, &[ah, widened], 0x48, 4);
    let zero = k(&mut fd, 0);
    let fits = def(&mut fd, OpCode::CPUI_INT_EQUAL, &[top, zero], 0x4c, 1);
    let dest = k(&mut fd, 0x3000);
    op(&mut fd, OpCode::CPUI_CBRANCH, &[dest, fits]);
    let code = k(&mut fd, 34);
    let hi = def(&mut fd, OpCode::CPUI_COPY, &[code], HI, 4);
    assert_eq!(
        class_of(&mut fd, hi, lo),
        LowWord::Compared,
        "SPARC `addcc %i1,%i2,%i1; addxcc %i0,-1,%i0; cmp %i0,-1; bne` then `restore %g0,34,%o0`",
    );
}

#[test]
fn a_negated_flag_beside_the_sum_it_tests_is_a_comparison() {
    let mut fd = build_fd();
    let (ah, al, bias) = (input(&mut fd, 0x20), input(&mut fd, 0x24), input(&mut fd, 0x28));
    let lo = def(&mut fd, OpCode::CPUI_INT_ADD, &[al, bias], LO, 4);
    let carry = def(&mut fd, OpCode::CPUI_INT_CARRY, &[al, bias], 0x40, 1);
    let widened = def(&mut fd, OpCode::CPUI_INT_ZEXT, &[carry], 0x44, 4);
    let top = def(&mut fd, OpCode::CPUI_INT_ADD, &[ah, widened], 0x48, 4);
    let zeros = def(&mut fd, OpCode::CPUI_LZCOUNT, &[top], 0x4c, 4);
    let five = k(&mut fd, 5);
    let flag = def(&mut fd, OpCode::CPUI_INT_RIGHT, &[zeros, five], 0x50, 4);
    let hi = def(&mut fd, OpCode::CPUI_INT_2COMP, &[flag], HI, 4);
    assert!(!computed_flag(&fd, hi), "`-flag` is 0 or -1, not 0 or 1");
    assert_eq!(
        class_of(&mut fd, hi, lo),
        LowWord::Compared,
        "PowerPC `addc 4,4,5; addze 3,3; cntlzw 3,3; srwi 3,3,5; neg 3,3`",
    );
}

#[test]
fn flags_offset_inverted_shifted_or_chosen_are_worked_out_from_flags() {
    let mut fd = build_fd();
    let (a, b) = (input(&mut fd, 0x20), input(&mut fd, 0x24));
    let less = def(&mut fd, OpCode::CPUI_INT_LESS, &[a, b], 0x40, 1);
    let flag = def(&mut fd, OpCode::CPUI_INT_ZEXT, &[less], 0x44, 4);
    let minus_one = k(&mut fd, 0xffff_ffff);
    let offset = def(&mut fd, OpCode::CPUI_INT_ADD, &[flag, minus_one], 0x48, 4);
    assert!(of_flags(&fd, offset, &[]), "`flag - 1`");
    let all = k(&mut fd, 0xffff_ffff);
    let inverted = def(&mut fd, OpCode::CPUI_INT_XOR, &[flag, all], 0x4c, 4);
    assert!(of_flags(&fd, inverted, &[]), "`flag ^ -1`");
    let three = k(&mut fd, 3);
    let shifted = def(&mut fd, OpCode::CPUI_INT_LEFT, &[flag, three], 0x50, 4);
    assert!(of_flags(&fd, shifted, &[]), "`flag << 3`");
    let (two, seven) = (k(&mut fd, 2), k(&mut fd, 7));
    let chosen = def(&mut fd, OpCode::CPUI_MULTIEQUAL, &[two, seven], 0x54, 4);
    assert!(of_flags(&fd, chosen, &[]), "`li 3,2` or `li 3,7`");
    let plus = def(&mut fd, OpCode::CPUI_INT_ADD, &[a, flag], 0x58, 4);
    assert!(!of_flags(&fd, plus, &[]), "a high word plus a carry is a 64-bit sum's");
    let mixed = def(&mut fd, OpCode::CPUI_MULTIEQUAL, &[two, a], 0x5c, 4);
    assert!(!of_flags(&fd, mixed, &[]));
}

#[test]
fn a_sign_test_of_the_sum_alone_is_not_its_carry() {
    let mut fd = build_fd();
    let (a, b) = (input(&mut fd, 0x20), input(&mut fd, 0x24));
    let lo = def(&mut fd, OpCode::CPUI_INT_ADD, &[a, b], LO, 4);
    let zero = k(&mut fd, 0);
    let negative = def(&mut fd, OpCode::CPUI_INT_SLESS, &[lo, zero], 0x40, 1);
    let dest = k(&mut fd, 0x3000);
    op(&mut fd, OpCode::CPUI_CBRANCH, &[dest, negative]);
    let none = k(&mut fd, 0);
    let hi = def(&mut fd, OpCode::CPUI_COPY, &[none], HI, 4);
    assert!(carries_of(&fd, lo).is_empty(), "`addcc`'s sign flag compares the sum with a zero it does not add");
    assert_eq!(class_of(&mut fd, hi, lo), LowWord::Scratch, "the branch reads the sum itself");
}

#[test]
fn a_carry_added_into_the_first_register_is_a_sums_not_a_comparisons() {
    let mut fd = build_fd();
    let (a, b) = (input(&mut fd, 0x20), input(&mut fd, 0x24));
    let first = def(&mut fd, OpCode::CPUI_INT_ADD, &[a, b], 0x28, 4);
    let carry1 = def(&mut fd, OpCode::CPUI_INT_CARRY, &[a, b], 0x40, 1);
    let five = k(&mut fd, 5);
    let lo = def(&mut fd, OpCode::CPUI_INT_ADD, &[first, five], LO, 4);
    let five_again = k(&mut fd, 5);
    let carry2 = def(&mut fd, OpCode::CPUI_INT_CARRY, &[first, five_again], 0x44, 1);
    let c1 = def(&mut fd, OpCode::CPUI_INT_ZEXT, &[carry1], 0x48, 4);
    let c2 = def(&mut fd, OpCode::CPUI_INT_ZEXT, &[carry2], 0x4c, 4);
    let hi = def(&mut fd, OpCode::CPUI_INT_ADD, &[c1, c2], HI, 4);
    assert!(!of_flags(&fd, hi, &carries_of(&fd, lo)), "the second carry is the low sum's own");
    assert_eq!(class_of(&mut fd, hi, lo), LowWord::Returned, "`(u64)n + m + 5` adds both carries into the high word");
}

#[test]
fn a_literal_beside_a_sum_whose_carry_nothing_reads_is_returned() {
    let mut fd = build_fd();
    let (a, b) = (input(&mut fd, 0x20), input(&mut fd, 0x24));
    let lo = def(&mut fd, OpCode::CPUI_INT_ADD, &[a, b], LO, 4);
    def(&mut fd, OpCode::CPUI_INT_CARRY, &[a, b], 0x40, 1);
    let none = k(&mut fd, 0);
    let hi = def(&mut fd, OpCode::CPUI_COPY, &[none], HI, 4);
    assert_eq!(
        class_of(&mut fd, hi, lo),
        LowWord::Returned,
        "SPARC `addcc` computes a carry `(u64)(u32)(a + b)` never reads",
    );
}

/// An unsigned 32x32->64 product of `a` and `magic` in a scratch register,
/// its low half in the low word's register, and its high half.
fn umull(fd: &mut Funcdata, magic: u64) -> (VarnodeId, VarnodeId, VarnodeId) {
    let a = input(fd, 0x20);
    let m = k(fd, magic);
    let (wa, wm) = (def(fd, OpCode::CPUI_INT_ZEXT, &[a], 0x50, 8), def(fd, OpCode::CPUI_INT_ZEXT, &[m], 0x58, 8));
    let product = def(fd, OpCode::CPUI_INT_MULT, &[wa, wm], 0x60, 8);
    let (z, four) = (k(fd, 0), k(fd, 4));
    let lo = def(fd, OpCode::CPUI_SUBPIECE, &[product, z], LO, 4);
    let high_half = def(fd, OpCode::CPUI_SUBPIECE, &[product, four], 0x68, 4);
    (a, lo, high_half)
}

#[test]
fn a_product_whose_high_half_is_shifted_down_is_a_quotient() {
    let mut fd = build_fd();
    let (_, lo, high_half) = umull(&mut fd, 0xcccc_cccd);
    let three = k(&mut fd, 3);
    let hi = def(&mut fd, OpCode::CPUI_INT_RIGHT, &[high_half, three], HI, 4);
    assert_eq!(class_of(&mut fd, hi, lo), LowWord::Quotient, "ARM `umull r1,r0,r2,r3; lsr r0,r0,#3` is `a / 10`");
}

#[test]
fn a_quotient_fixed_up_before_its_shift_is_a_quotient() {
    let mut fd = build_fd();
    let (a, lo, high_half) = umull(&mut fd, 0x2492_4925);
    let diff = def(&mut fd, OpCode::CPUI_INT_SUB, &[a, high_half], 0x70, 4);
    let one = k(&mut fd, 1);
    let half = def(&mut fd, OpCode::CPUI_INT_RIGHT, &[diff, one], 0x74, 4);
    let sum = def(&mut fd, OpCode::CPUI_INT_ADD, &[high_half, half], 0x78, 4);
    let two = k(&mut fd, 2);
    let hi = def(&mut fd, OpCode::CPUI_INT_RIGHT, &[sum, two], HI, 4);
    assert_eq!(class_of(&mut fd, hi, lo), LowWord::Quotient, "ARMv7 `a / 7`: `sub; add ...,lsr #1; lsr #2`");
}

#[test]
fn a_signed_quotient_adds_its_own_sign() {
    let mut fd = build_fd();
    let (_, lo, high_half) = umull(&mut fd, 0x5555_5556);
    let sign_at = k(&mut fd, 31);
    let sign = def(&mut fd, OpCode::CPUI_INT_RIGHT, &[high_half, sign_at], 0x70, 4);
    let hi = def(&mut fd, OpCode::CPUI_INT_ADD, &[high_half, sign], HI, 4);
    assert_eq!(class_of(&mut fd, hi, lo), LowWord::Quotient, "ARM `smull r1,r0,r2,r3; add r0,r0,r0,lsr #31` is `a / 3`");
}

#[test]
fn a_products_high_half_with_terms_added_is_returned() {
    let mut fd = build_fd();
    let (_, lo, high_half) = umull(&mut fd, 3);
    let a_hi = input(&mut fd, 0x24);
    let three = k(&mut fd, 3);
    let more = def(&mut fd, OpCode::CPUI_INT_MULT, &[a_hi, three], 0x70, 4);
    let hi = def(&mut fd, OpCode::CPUI_INT_ADD, &[high_half, more], HI, 4);
    assert_eq!(class_of(&mut fd, hi, lo), LowWord::Returned, "`(long long)a * 3` adds the high word's product");
}

#[test]
fn a_shift_beside_the_high_half_but_not_of_it_is_not_a_quotient() {
    let mut fd = build_fd();
    let (_, lo, high_half) = umull(&mut fd, 7);
    let b = input(&mut fd, 0x24);
    let sign_at = k(&mut fd, 31);
    let sign = def(&mut fd, OpCode::CPUI_INT_SRIGHT, &[b, sign_at], 0x70, 4);
    let a = input(&mut fd, 0x28);
    let more = def(&mut fd, OpCode::CPUI_INT_MULT, &[a, sign], 0x74, 4);
    let hi = def(&mut fd, OpCode::CPUI_INT_ADD, &[high_half, more], HI, 4);
    assert_eq!(class_of(&mut fd, hi, lo), LowWord::Returned, "`(long long)a * b` adds `a * (b >> 31)` to the high half");
}

/// A 64-bit sum of `a` and `b` held the reverse way round: its low half in
/// the first register's place and its high half (the carry) beside it.
fn reverse_sum(fd: &mut Funcdata) -> (VarnodeId, VarnodeId) {
    let (a, b) = (input(fd, 0x20), input(fd, 0x24));
    let carry = def(fd, OpCode::CPUI_INT_CARRY, &[a, b], 0x40, 1);
    let high_half = def(fd, OpCode::CPUI_INT_ZEXT, &[carry], 0x44, 4);
    let low_half = def(fd, OpCode::CPUI_INT_ADD, &[a, b], 0x48, 4);
    (low_half, high_half)
}

#[test]
fn a_shift_whose_dropped_bit_reaches_the_first_register_is_the_reverse_pair() {
    let mut fd = build_fd();
    let (low_half, high_half) = reverse_sum(&mut fd);
    let one = k(&mut fd, 1);
    let lo = def(&mut fd, OpCode::CPUI_INT_RIGHT, &[high_half, one], LO, 4);
    let mask = k(&mut fd, 1);
    let bit = def(&mut fd, OpCode::CPUI_INT_AND, &[high_half, mask], 0x50, 4);
    let top_at = k(&mut fd, 31);
    let top = def(&mut fd, OpCode::CPUI_INT_LEFT, &[bit, top_at], 0x54, 4);
    let one = k(&mut fd, 1);
    let half = def(&mut fd, OpCode::CPUI_INT_RIGHT, &[low_half, one], 0x58, 4);
    let hi = def(&mut fd, OpCode::CPUI_INT_OR, &[top, half], HI, 4);
    assert_eq!(
        class_of(&mut fd, hi, lo),
        LowWord::ShiftedOut,
        "ARM `lsrs r1,r1,#1; rrx r0,r0` moves bit 0 of r1 into r0 for `(u32)(((u64)a + b) >> 1)`",
    );
}

#[test]
fn a_long_long_shifted_right_moves_the_bit_into_the_second_register() {
    let mut fd = build_fd();
    let (high_half, low_half) = (input(&mut fd, 0x20), input(&mut fd, 0x28));
    let mask = k(&mut fd, 1);
    let bit = def(&mut fd, OpCode::CPUI_INT_AND, &[high_half, mask], 0x50, 4);
    let top_at = k(&mut fd, 31);
    let top = def(&mut fd, OpCode::CPUI_INT_LEFT, &[bit, top_at], 0x54, 4);
    let one = k(&mut fd, 1);
    let half = def(&mut fd, OpCode::CPUI_INT_RIGHT, &[low_half, one], 0x58, 4);
    let lo = def(&mut fd, OpCode::CPUI_INT_OR, &[top, half], LO, 4);
    let one = k(&mut fd, 1);
    let hi = def(&mut fd, OpCode::CPUI_INT_RIGHT, &[high_half, one], HI, 4);
    assert_eq!(class_of(&mut fd, hi, lo), LowWord::Returned, "`x >> 1` is `lsrs r0,r0,#1; rrx r1,r1`");
}

#[test]
fn a_sign_shifted_past_the_dropped_bits_is_not_them() {
    let mut fd = build_fd();
    let (a, b) = (input(&mut fd, 0x20), input(&mut fd, 0x28));
    let high_half = def(&mut fd, OpCode::CPUI_INT_ADD, &[a, b], 0x48, 4);
    let one = k(&mut fd, 1);
    let lo = def(&mut fd, OpCode::CPUI_INT_SRIGHT, &[high_half, one], LO, 4);
    let sign_at = k(&mut fd, 31);
    let hi = def(&mut fd, OpCode::CPUI_INT_SRIGHT, &[high_half, sign_at], HI, 4);
    assert_eq!(class_of(&mut fd, hi, lo), LowWord::Returned, "`x >> 33` of a signed 64-bit value");
}

#[test]
fn a_sign_fill_beside_the_word_it_came_from_is_returned() {
    let mut fd = build_fd();
    let (a, b) = (input(&mut fd, 0x20), input(&mut fd, 0x28));
    let high_half = def(&mut fd, OpCode::CPUI_INT_ADD, &[a, b], 0x48, 4);
    let sign_at = k(&mut fd, 31);
    let lo = def(&mut fd, OpCode::CPUI_INT_SRIGHT, &[high_half, sign_at], LO, 4);
    let hi = def(&mut fd, OpCode::CPUI_COPY, &[high_half], HI, 4);
    assert_eq!(class_of(&mut fd, hi, lo), LowWord::Returned, "a saturated `long long` fills its low word with a sign");
}

#[test]
fn an_indirect_jump_flow_could_not_follow_hides_code() {
    let mut fd = build_fd();
    let target = input(&mut fd, 0x20);
    let one = k(&mut fd, 1);
    ret(&mut fd, one, target);
    assert!(hidden_readers(&fd).is_empty(), "a RETURN alone hides nothing");
    op(&mut fd, OpCode::CPUI_CALLIND, &[target]);
    let one = k(&mut fd, 1);
    op(&mut fd, OpCode::CPUI_RETURN, &[one]);
    assert_eq!(hidden_readers(&fd).len(), 1, "`Treating indirect jump as call` leaves a CALLIND and an artificial RETURN");
}

#[test]
fn a_copy_made_before_an_unfollowed_jump_reaches_it() {
    let mut fd = build_fd();
    let a = input(&mut fd, 0x24);
    let zero = k(&mut fd, 0);
    let copy = def(&mut fd, OpCode::CPUI_INT_OR, &[a, zero], LO, 4);
    let hi = k(&mut fd, 0);
    let r = ret(&mut fd, hi, copy);
    let target = input(&mut fd, 0x20);
    op(&mut fd, OpCode::CPUI_CALLIND, &[target]);
    let one = k(&mut fd, 1);
    op(&mut fd, OpCode::CPUI_RETURN, &[one]);
    let p = pair(&fd);
    assert_eq!(classify(&fd, r, &p), LowWord::Returned);
    let jumps = hidden_readers(&fd);
    assert!(reaches_jump(&fd, r, &p, &jumps), "MIPS `move v1,a1` before the switch is in its cases' reach");
}

/// `lo = al + bl` with its carry, and `ah + bh + carry`, the high word of the
/// 64-bit sum (`addc 4,6,4; adde 3,5,3`).
fn sum64(fd: &mut Funcdata) -> (VarnodeId, VarnodeId) {
    let (ah, al, bh, bl) = (input(fd, 0x20), input(fd, 0x24), input(fd, 0x28), input(fd, 0x2c));
    let lo = def(fd, OpCode::CPUI_INT_ADD, &[al, bl], LO, 4);
    let carry = def(fd, OpCode::CPUI_INT_CARRY, &[al, bl], 0x40, 1);
    let widened = def(fd, OpCode::CPUI_INT_ZEXT, &[carry], 0x44, 4);
    let both = def(fd, OpCode::CPUI_INT_ADD, &[ah, bh], 0x48, 4);
    let high = def(fd, OpCode::CPUI_INT_ADD, &[both, widened], 0x4c, 4);
    (high, lo)
}

#[test]
fn the_high_word_of_a_sum_itself_is_returned() {
    let mut fd = build_fd();
    let (high, lo) = sum64(&mut fd);
    let hi = def(&mut fd, OpCode::CPUI_COPY, &[high], HI, 4);
    assert_eq!(class_of(&mut fd, hi, lo), LowWord::Returned, "`x + y` of two `u64`s");
}

#[test]
fn a_high_word_shifted_masked_or_offset_keeps_the_int() {
    for (opc, by, why) in [
        (OpCode::CPUI_INT_RIGHT, 16, "`(u32)((x + y) >> 48)`: `srwi 3,3,16`"),
        (OpCode::CPUI_INT_AND, 0xff, "`(u32)((x + y) >> 32) & 0xff`"),
        (OpCode::CPUI_INT_ADD, 5, "`(u32)((x + y) >> 32) + 5`"),
        (OpCode::CPUI_INT_MULT, 7, "`(u32)((a + b + c) >> 32) * 7`"),
    ] {
        let mut fd = build_fd();
        let (high, lo) = sum64(&mut fd);
        let amount = k(&mut fd, by);
        let hi = def(&mut fd, opc, &[high, amount], HI, 4);
        assert_eq!(class_of(&mut fd, hi, lo), LowWord::Reworked, "{why}");
    }
}

#[test]
fn a_negated_high_word_keeps_the_int() {
    let mut fd = build_fd();
    let (high, lo) = sum64(&mut fd);
    let hi = def(&mut fd, OpCode::CPUI_INT_2COMP, &[high], HI, 4);
    assert_eq!(class_of(&mut fd, hi, lo), LowWord::Reworked, "PowerPC `neg 3,3` of `adde`");
    let mut fd = build_fd();
    let (high, lo) = sum64(&mut fd);
    let zero = k(&mut fd, 0);
    let hi = def(&mut fd, OpCode::CPUI_INT_SUB, &[zero, high], HI, 4);
    assert_eq!(class_of(&mut fd, hi, lo), LowWord::Reworked, "ARM `rsb r0,r0,#0` of `adc`");
}

#[test]
fn a_borrow_subtracted_with_the_subtrahend_is_a_difference() {
    let mut fd = build_fd();
    let (ah, al, bh, bl) = (input(&mut fd, 0x20), input(&mut fd, 0x24), input(&mut fd, 0x28), input(&mut fd, 0x2c));
    let lo = def(&mut fd, OpCode::CPUI_INT_SUB, &[al, bl], LO, 4);
    let no_borrow = def(&mut fd, OpCode::CPUI_INT_LESSEQUAL, &[bl, al], 0x40, 1);
    let borrow = def(&mut fd, OpCode::CPUI_BOOL_NEGATE, &[no_borrow], 0x41, 1);
    let widened = def(&mut fd, OpCode::CPUI_INT_ZEXT, &[borrow], 0x44, 4);
    let taken = def(&mut fd, OpCode::CPUI_INT_ADD, &[bh, widened], 0x48, 4);
    let hi = def(&mut fd, OpCode::CPUI_INT_SUB, &[ah, taken], HI, 4);
    assert_eq!(class_of(&mut fd, hi, lo), LowWord::Returned, "ARM `subs r1,r1,r3; sbc r0,r0,r2`: `r0 - (r2 + !CY)`");
}

#[test]
fn a_negated_long_long_subtracts_its_borrow() {
    let mut fd = build_fd();
    let (ah, al) = (input(&mut fd, 0x20), input(&mut fd, 0x24));
    let zero = k(&mut fd, 0);
    let lo = def(&mut fd, OpCode::CPUI_INT_SUB, &[zero, al], LO, 4);
    let zero2 = k(&mut fd, 0);
    let no_borrow = def(&mut fd, OpCode::CPUI_INT_LESSEQUAL, &[al, zero2], 0x40, 1);
    let borrow = def(&mut fd, OpCode::CPUI_BOOL_NEGATE, &[no_borrow], 0x41, 1);
    let widened = def(&mut fd, OpCode::CPUI_INT_ZEXT, &[borrow], 0x44, 4);
    let taken = def(&mut fd, OpCode::CPUI_INT_ADD, &[ah, widened], 0x48, 4);
    let zero3 = k(&mut fd, 0);
    let hi = def(&mut fd, OpCode::CPUI_INT_SUB, &[zero3, taken], HI, 4);
    assert_eq!(class_of(&mut fd, hi, lo), LowWord::Returned, "ARM `rsbs r1,r1,#0; rsc r0,r0,#0` is `-x`");
}

#[test]
fn the_high_half_of_a_product_beside_its_low_half_keeps_the_int() {
    let mut fd = build_fd();
    let (a, b) = (input(&mut fd, 0x20), input(&mut fd, 0x24));
    let (wa, wb) = (def(&mut fd, OpCode::CPUI_INT_ZEXT, &[a], 0x50, 8), def(&mut fd, OpCode::CPUI_INT_ZEXT, &[b], 0x58, 8));
    let product = def(&mut fd, OpCode::CPUI_INT_MULT, &[wa, wb], 0x60, 8);
    let (z, four) = (k(&mut fd, 0), k(&mut fd, 4));
    let lo = def(&mut fd, OpCode::CPUI_SUBPIECE, &[product, four], LO, 4);
    let low_half = def(&mut fd, OpCode::CPUI_SUBPIECE, &[product, z], 0x48, 4);
    let hi = def(&mut fd, OpCode::CPUI_INT_XOR, &[low_half, a], HI, 4);
    assert_eq!(product_high_half(&fd, product, 4, 4), Some(product));
    assert_eq!(product_high_half(&fd, product, 0, 4), None, "the low half of a product is a `long long`'s low word");
    assert!(
        reads_low_half(&fd, hi, product, 4),
        "MIPS clang -O0 `(u32)x` of a 64-bit hash leaves the `mfhi` in $3 and builds $2 from the `mflo`",
    );
    assert!(!reads_low_half(&fd, lo, product, 4));
}

#[test]
fn the_high_half_of_a_product_beside_a_zero_is_returned() {
    let mut fd = build_fd();
    let (a, b) = (input(&mut fd, 0x20), input(&mut fd, 0x24));
    let (wa, wb) = (def(&mut fd, OpCode::CPUI_INT_ZEXT, &[a], 0x50, 8), def(&mut fd, OpCode::CPUI_INT_ZEXT, &[b], 0x58, 8));
    let product = def(&mut fd, OpCode::CPUI_INT_MULT, &[wa, wb], 0x60, 8);
    let four = k(&mut fd, 4);
    let lo = def(&mut fd, OpCode::CPUI_SUBPIECE, &[product, four], LO, 4);
    let one = k(&mut fd, 1);
    let hi = def(&mut fd, OpCode::CPUI_COPY, &[one], HI, 4);
    assert_eq!(class_of(&mut fd, hi, lo), LowWord::Returned, "`((u64)a * b >> 32) | 1ULL << 32` reads no low half");
}

#[test]
fn a_literal_operation_on_a_word_the_low_word_never_reads_keeps_the_int() {
    let mut fd = build_fd();
    let (xh, xl, yh, yl) = (input(&mut fd, 0x20), input(&mut fd, 0x24), input(&mut fd, 0x28), input(&mut fd, 0x2c));
    let lo = def(&mut fd, OpCode::CPUI_INT_XOR, &[xl, yl], LO, 4);
    let high = def(&mut fd, OpCode::CPUI_INT_XOR, &[xh, yh], 0x48, 4);
    let mask = k(&mut fd, 0xffff);
    let hi = def(&mut fd, OpCode::CPUI_INT_AND, &[high, mask], HI, 4);
    assert_eq!(class_of(&mut fd, hi, lo), LowWord::Reworked, "gcc -O0 on MIPS: `(u32)((x ^ y) >> 32) & 0xffff`");
}

#[test]
fn one_value_split_across_both_words_is_returned() {
    let mut fd = build_fd();
    let (v, w) = (input(&mut fd, 0x20), input(&mut fd, 0x24));
    let mixed = def(&mut fd, OpCode::CPUI_INT_XOR, &[v, w], 0x48, 4);
    let (seven, mask) = (k(&mut fd, 7), k(&mut fd, 0xff));
    let lo = def(&mut fd, OpCode::CPUI_INT_MULT, &[mixed, seven], LO, 4);
    let hi = def(&mut fd, OpCode::CPUI_INT_AND, &[mixed, mask], HI, 4);
    assert_eq!(class_of(&mut fd, hi, lo), LowWord::Returned, "`((u64)(v & 0xff) << 32) | (u32)(v * 7)`");
}

#[test]
fn a_rotate_then_mask_reads_the_word_it_rotates() {
    let mut fd = build_fd();
    let (xh, xl) = (input(&mut fd, 0x20), input(&mut fd, 0x24));
    let (one, thirty_one, mask) = (k(&mut fd, 1), k(&mut fd, 31), k(&mut fd, 0x7fff_ffff));
    let up = def(&mut fd, OpCode::CPUI_INT_LEFT, &[xh, thirty_one], 0x40, 4);
    let down = def(&mut fd, OpCode::CPUI_INT_RIGHT, &[xl, one], 0x44, 4);
    let lo = def(&mut fd, OpCode::CPUI_INT_OR, &[down, up], LO, 4);
    let (one2, thirty_one2) = (k(&mut fd, 1), k(&mut fd, 31));
    let left = def(&mut fd, OpCode::CPUI_INT_LEFT, &[xh, thirty_one2], 0x48, 4);
    let right = def(&mut fd, OpCode::CPUI_INT_RIGHT, &[xh, one2], 0x4c, 4);
    let rotated = def(&mut fd, OpCode::CPUI_INT_OR, &[left, right], 0x50, 4);
    let hi = def(&mut fd, OpCode::CPUI_INT_AND, &[rotated, mask], HI, 4);
    assert_eq!(class_of(&mut fd, hi, lo), LowWord::Returned, "PowerPC `x >> 1` of a `u64`: `srwi 3,3,1` is `rlwinm`");
}

#[test]
fn a_literal_low_word_beside_a_worked_high_word_is_returned() {
    let mut fd = build_fd();
    let a = input(&mut fd, 0x20);
    let key = k(&mut fd, 0x1234);
    let hi = def(&mut fd, OpCode::CPUI_INT_XOR, &[a, key], HI, 4);
    let key2 = k(&mut fd, 0x1234);
    let lo = def(&mut fd, OpCode::CPUI_COPY, &[key2], LO, 4);
    assert_eq!(class_of(&mut fd, hi, lo), LowWord::Returned, "`((u64)(a ^ 0x1234) << 32) | 0x1234`");
}

#[test]
fn a_negated_product_high_half_keeps_the_int() {
    let mut fd = build_fd();
    let (a, b) = (input(&mut fd, 0x20), input(&mut fd, 0x24));
    let (wa, wb) = (def(&mut fd, OpCode::CPUI_INT_SEXT, &[a], 0x50, 8), def(&mut fd, OpCode::CPUI_INT_SEXT, &[b], 0x58, 8));
    let product = def(&mut fd, OpCode::CPUI_INT_MULT, &[wa, wb], 0x60, 8);
    let (z, four) = (k(&mut fd, 0), k(&mut fd, 4));
    let lo = def(&mut fd, OpCode::CPUI_SUBPIECE, &[product, z], LO, 4);
    let high_half = def(&mut fd, OpCode::CPUI_SUBPIECE, &[product, four], 0x48, 4);
    let zero = k(&mut fd, 0);
    let hi = def(&mut fd, OpCode::CPUI_INT_SUB, &[zero, high_half], HI, 4);
    assert_eq!(class_of(&mut fd, hi, lo), LowWord::Reworked, "ARM -O0 `smull r1,r0`, then `rsb r0,r0,#0`");
}

#[test]
fn an_operation_given_no_register_inputs_is_a_hidden_reader() {
    let mut fd = build_fd();
    let a = input(&mut fd, 0x24);
    let moved = def(&mut fd, OpCode::CPUI_COPY, &[a], LO, 4);
    let index = k(&mut fd, 3);
    op(&mut fd, OpCode::CPUI_CALLOTHER, &[index]);
    let b = input(&mut fd, 0x20);
    let r = ret(&mut fd, b, moved);
    let p = pair(&fd);
    assert_eq!(classify(&fd, r, &p), LowWord::Returned);
    let readers = hidden_readers(&fd);
    assert_eq!(readers.len(), 1, "PowerPC `sc` lifts to `syscall()`, which reads the registers set up for it");
    assert!(reaches_jump(&fd, r, &p, &readers));
}

#[test]
fn a_negation_subtracts_the_borrow_its_operand_being_nonzero_makes() {
    let mut fd = build_fd();
    let (ah, al) = (input(&mut fd, 0x20), input(&mut fd, 0x24));
    let zero = k(&mut fd, 0);
    let lo = def(&mut fd, OpCode::CPUI_INT_SUB, &[zero, al], LO, 4);
    let zero2 = k(&mut fd, 0);
    let borrow = def(&mut fd, OpCode::CPUI_INT_NOTEQUAL, &[al, zero2], 0x40, 1);
    let widened = def(&mut fd, OpCode::CPUI_INT_ZEXT, &[borrow], 0x44, 4);
    let taken = def(&mut fd, OpCode::CPUI_INT_ADD, &[widened, ah], 0x48, 4);
    let hi = def(&mut fd, OpCode::CPUI_INT_2COMP, &[taken], HI, 4);
    assert_eq!(class_of(&mut fd, hi, lo), LowWord::Returned, "`-a` of an `i64`: `-((a1 != 0) + a0)` beside `-a1`");
}

#[test]
fn a_borrow_mips_tests_against_the_difference_is_subtracted() {
    let mut fd = build_fd();
    let (ah, al, bh, bl) = (input(&mut fd, 0x20), input(&mut fd, 0x24), input(&mut fd, 0x28), input(&mut fd, 0x2c));
    let lo = def(&mut fd, OpCode::CPUI_INT_SUB, &[al, bl], LO, 4);
    let borrow = def(&mut fd, OpCode::CPUI_INT_LESS, &[al, lo], 0x40, 1);
    let widened = def(&mut fd, OpCode::CPUI_INT_ZEXT, &[borrow], 0x44, 4);
    let high = def(&mut fd, OpCode::CPUI_INT_SUB, &[ah, bh], 0x48, 4);
    let hi = def(&mut fd, OpCode::CPUI_INT_SUB, &[high, widened], HI, 4);
    assert_eq!(class_of(&mut fd, hi, lo), LowWord::Returned, "MIPS `subu $3,$5,$7; sltu $2,$5,$3; subu $2,$4,$6; subu $2,$2,$2`");
}

#[test]
fn a_complement_of_both_halves_is_one_64_bit_complement() {
    let mut fd = build_fd();
    let (xh, xl) = (input(&mut fd, 0x20), input(&mut fd, 0x24));
    let (mh, ml) = (def(&mut fd, OpCode::CPUI_INT_ADD, &[xh, xl], 0x40, 4), def(&mut fd, OpCode::CPUI_INT_MULT, &[xl, xh], 0x44, 4));
    let lo = def(&mut fd, OpCode::CPUI_INT_NEGATE, &[ml], LO, 4);
    let hi = def(&mut fd, OpCode::CPUI_INT_NEGATE, &[mh], HI, 4);
    assert_eq!(class_of(&mut fd, hi, lo), LowWord::Returned, "`~v` of a 64-bit value complements both words");
}

#[test]
fn powerpc_mr_is_a_move() {
    let mut fd = build_fd();
    let x = input(&mut fd, 0x20);
    let copy = def(&mut fd, OpCode::CPUI_INT_OR, &[x, x], 0x40, 4);
    assert_eq!(moved(&fd, copy), Some(x), "`mr 3,5` is `or 3,5,5`");
}

#[test]
fn a_64_bit_literal_adds_its_high_half_beside_the_carry_but_not_on_top_of_it() {
    let build = |on_top: bool| {
        let mut fd = build_fd();
        let (xh, xl) = (input(&mut fd, 0x20), input(&mut fd, 0x24));
        let five = k(&mut fd, 5);
        let lo = def(&mut fd, OpCode::CPUI_INT_ADD, &[xl, five], LO, 4);
        let five2 = k(&mut fd, 5);
        let carry = def(&mut fd, OpCode::CPUI_INT_CARRY, &[xl, five2], 0x40, 1);
        let widened = def(&mut fd, OpCode::CPUI_INT_ZEXT, &[carry], 0x44, 4);
        let seven = k(&mut fd, 7);
        let hi = if on_top {
            let carried = def(&mut fd, OpCode::CPUI_INT_ADD, &[xh, widened], 0x48, 4);
            def(&mut fd, OpCode::CPUI_INT_ADD, &[carried, seven], HI, 4)
        } else {
            let both = def(&mut fd, OpCode::CPUI_INT_ADD, &[xh, seven], 0x48, 4);
            def(&mut fd, OpCode::CPUI_INT_ADD, &[both, widened], HI, 4)
        };
        class_of(&mut fd, hi, lo)
    };
    assert_eq!(build(false), LowWord::Returned, "`li 5,7; addic 4,4,5; adde 3,3,5` is `x + 0x700000005`");
    assert_eq!(build(true), LowWord::Reworked, "`addic 4,4,5; addze 3,3; addi 3,3,7` is `(u32)((x + 5) >> 32) + 7`");
}

#[test]
fn a_loop_accumulator_may_add_its_literal_beside_an_earlier_carry() {
    let mut fd = build_fd();
    let (xh, xl) = (input(&mut fd, 0x20), input(&mut fd, 0x24));
    let ones = k(&mut fd, 0xffff_ffff);
    let lo = def(&mut fd, OpCode::CPUI_INT_ADD, &[xl, ones], LO, 4);
    let ones2 = k(&mut fd, 0xffff_ffff);
    let carry = def(&mut fd, OpCode::CPUI_INT_CARRY, &[xl, ones2], 0x40, 1);
    let widened = def(&mut fd, OpCode::CPUI_INT_ZEXT, &[carry], 0x44, 4);
    let phi = def(&mut fd, OpCode::CPUI_MULTIEQUAL, &[xh, widened], 0x48, 4);
    let ones3 = k(&mut fd, 0xffff_ffff);
    let less = def(&mut fd, OpCode::CPUI_INT_ADD, &[phi, ones3], 0x4c, 4);
    let hi = def(&mut fd, OpCode::CPUI_INT_ADD, &[less, widened], HI, 4);
    assert_eq!(class_of(&mut fd, hi, lo), LowWord::Returned, "coreutils dd's `records--` in a loop: `addic 6,6,-1; addme 5,5`");
}

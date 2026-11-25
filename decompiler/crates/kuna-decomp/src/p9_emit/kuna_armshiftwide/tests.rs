//! Double-word shift count families.

use super::{affine_count, complement_pair, double_word_count, AffineCount};
use crate::context::{ArchContext, TypeOp, VarnodeId};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use kuna_base::address::Address;
use kuna_base::space::{addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace};
use kuna_num::opcodes::OpCode;
use std::rc::Rc;

struct Fx {
    fd: Funcdata,
    register: Rc<AddrSpace>,
    ram: Rc<AddrSpace>,
    next: u64,
}

impl Fx {
    fn new() -> Self {
        let mut manager = AddrSpaceManager::new();
        manager.insert_space(Rc::new(ConstantSpace::new())).unwrap();
        manager.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
        let space = |name: &str, index| {
            Rc::new(AddrSpace::new(
                spacetype::IPTR_PROCESSOR,
                name,
                false,
                8,
                1,
                index,
                addrspace_flags::hasphysical,
                1,
                1,
            ))
        };
        let (ram, register) = (space("ram", 2), space("register", 3));
        manager.insert_space(Rc::clone(&ram)).unwrap();
        manager.insert_space(Rc::clone(&register)).unwrap();
        let global = Rc::new(ArchContext::new(manager));
        let fd = Funcdata::new("wide", "wide", global, Address::new(Rc::clone(&ram), 0x1000), 0x1000, 0x100).unwrap();
        Fx { fd, register, ram, next: 0 }
    }

    fn value(&mut self) -> VarnodeId {
        self.next += 4;
        let ty = Rc::new(Datatype::new(4, type_metatype::TYPE_UNKNOWN));
        self.fd.new_varnode(4, &Address::new(Rc::clone(&self.register), self.next), Some(ty))
    }

    fn input(&mut self) -> VarnodeId {
        let vn = self.value();
        self.fd.set_input_varnode(vn).unwrap()
    }

    fn op(&mut self, code: OpCode, inputs: &[VarnodeId]) -> VarnodeId {
        self.next += 4;
        let op = self.fd.new_op(inputs.len() as i32, Address::new(Rc::clone(&self.ram), self.next));
        self.fd.obank_mut().change_opcode(op, TypeOp::new(code, 0, format!("{code:?}")));
        for (slot, &input) in inputs.iter().enumerate() {
            self.fd.op_set_input(op, input, slot as i32).unwrap();
        }
        let out = self.value();
        self.fd.op_set_output(op, out).unwrap();
        self.fd.obank().get(op).unwrap().get_out().unwrap()
    }

    fn k(&mut self, value: u64) -> VarnodeId {
        self.fd.new_constant(4, value)
    }

    fn low_byte(&mut self, x: VarnodeId) -> VarnodeId {
        let mask = self.k(0xff);
        self.op(OpCode::CPUI_INT_AND, &[x, mask])
    }

    /// `x + c`, then the SLEIGH `& 0xff` of a register shift count.
    fn plus(&mut self, x: VarnodeId, c: u64) -> VarnodeId {
        let c = self.k(c);
        let sum = self.op(OpCode::CPUI_INT_ADD, &[x, c]);
        self.low_byte(sum)
    }

    /// `c - x` as Ghidra spells it, `x * -1 + c`, then `& 0xff`.
    fn minus(&mut self, c: u64, x: VarnodeId) -> VarnodeId {
        let m = self.k(0xffff_ffff);
        let neg = self.op(OpCode::CPUI_INT_MULT, &[x, m]);
        self.plus(neg, c)
    }

    fn shift(&mut self, code: OpCode, count: VarnodeId) -> VarnodeId {
        let value = self.value();
        self.op(code, &[value, count])
    }
}

use OpCode::{CPUI_INT_LEFT as LEFT, CPUI_INT_RIGHT as RIGHT};

#[test]
fn affine_count_reads_masks_sums_negations_and_scales() {
    let mut fx = Fx::new();
    let n = fx.input();
    let below = fx.plus(n, 0xffff_ffe0);
    assert_eq!(affine_count(&fx.fd, below), Some(AffineCount { atom: n, coef: 1, add: 0xe0 }));
    let above = fx.minus(0x20, n);
    assert_eq!(affine_count(&fx.fd, above), Some(AffineCount { atom: n, coef: 0xff, add: 0x20 }));
    let negated = fx.op(OpCode::CPUI_INT_2COMP, &[below]);
    assert_eq!(affine_count(&fx.fd, negated), Some(AffineCount { atom: n, coef: 0xff, add: 0x20 }));
    let four = fx.k(4);
    let scaled = fx.op(OpCode::CPUI_INT_MULT, &[n, four]);
    let scaled = fx.plus(scaled, 0xffff_ffe0);
    assert_eq!(affine_count(&fx.fd, scaled), Some(AffineCount { atom: n, coef: 4, add: 0xe0 }));
    let narrow = fx.k(0x1f);
    let masked = fx.op(OpCode::CPUI_INT_AND, &[n, narrow]);
    assert_eq!(affine_count(&fx.fd, masked), Some(AffineCount { atom: masked, coef: 1, add: 0 }));
}

#[test]
fn complement_pair_needs_opposite_coefficients_summing_to_a_multiple_of_32() {
    assert!(complement_pair(&[(1, 0), (0xff, 0x20)]));
    assert!(complement_pair(&[(1, 0xe0), (0xff, 0x20)]));
    assert!(complement_pair(&[(1, 0xff), (1, 0xdf), (0xff, 0x21)]));
    assert!(complement_pair(&[(4, 0xe0), (0xfc, 0x20)]));
    assert!(!complement_pair(&[(1, 0), (1, 0xe0)]));
    assert!(!complement_pair(&[(1, 0), (0xff, 0x1f)]));
    assert!(!complement_pair(&[(4, 0), (0xff, 0x20)]));
}

/// gcc's inline `x << n`: `hi << n | lo << (n - 32) | lo >> (32 - n)`, `lo << n`.
#[test]
fn every_count_of_a_lowered_left_shift_is_double_word() {
    let mut fx = Fx::new();
    let n = fx.input();
    let count = fx.low_byte(n);
    let below = fx.plus(n, 0xffff_ffe0);
    let above = fx.minus(0x20, n);
    fx.shift(LEFT, count);
    fx.shift(LEFT, below);
    fx.shift(RIGHT, above);
    assert!(double_word_count(&fx.fd, count));
    assert!(double_word_count(&fx.fd, below));
}

/// A source `flag << (shift - 32)` beside `flag << shift` has no `32 - shift`.
#[test]
fn a_source_shift_by_n_minus_32_is_not_double_word() {
    let mut fx = Fx::new();
    let n = fx.input();
    let count = fx.low_byte(n);
    let below = fx.plus(n, 0xffff_ffe0);
    fx.shift(LEFT, count);
    fx.shift(LEFT, below);
    assert!(!double_word_count(&fx.fd, count));
    assert!(!double_word_count(&fx.fd, below));
}

/// `1ULL << (n - 1)` folds the program's `- 1` into both complements, and a
/// register ROR lifts as `x >> s | x << (32 - s)`.
#[test]
fn folded_complements_and_register_rotates_are_double_word() {
    let mut fx = Fx::new();
    let n = fx.input();
    let lo = fx.plus(n, 0xffff_ffff);
    let below = fx.plus(n, 0xffff_ffdf);
    let above = fx.minus(0x21, n);
    fx.shift(LEFT, lo);
    fx.shift(LEFT, below);
    fx.shift(RIGHT, above);
    assert!(double_word_count(&fx.fd, lo));

    let rs = fx.input();
    let five = fx.k(0x1f);
    let s = fx.op(OpCode::CPUI_INT_AND, &[rs, five]);
    let thirty_two = fx.k(0x20);
    let left = fx.op(OpCode::CPUI_INT_SUB, &[thirty_two, s]);
    fx.shift(RIGHT, s);
    fx.shift(LEFT, left);
    assert!(double_word_count(&fx.fd, left));
    let plain = fx.input();
    let plain = fx.low_byte(plain);
    fx.shift(LEFT, plain);
    assert!(!double_word_count(&fx.fd, plain));
}

/// part_read_oob: two paths each compute `32 - n` and `n - 32` and join
/// before the shifts; with no shift before the join the pair lives only on
/// the phi edges.
#[test]
fn complements_merged_at_a_join_are_double_word() {
    for shifts_before_join in [true, false] {
        let mut fx = Fx::new();
        let (a, b) = (fx.input(), fx.input());
        let mut merged = Vec::new();
        for n in [a, b] {
            let above = fx.minus(0x20, n);
            let below = fx.plus(n, 0xffff_ffe0);
            if shifts_before_join {
                fx.shift(LEFT, above);
                fx.shift(RIGHT, below);
            }
            merged.push((above, below));
        }
        let above = fx.op(OpCode::CPUI_MULTIEQUAL, &[merged[0].0, merged[1].0]);
        let below = fx.op(OpCode::CPUI_MULTIEQUAL, &[merged[0].1, merged[1].1]);
        let above_count = fx.low_byte(above);
        let below_count = fx.low_byte(below);
        fx.shift(LEFT, above_count);
        fx.shift(RIGHT, below_count);
        assert!(double_word_count(&fx.fd, above_count), "{shifts_before_join}");
    }
}

/// print_size: the loop's complements leave it through exit phis whose other
/// inputs are the peeled first iteration's constants.
#[test]
fn complements_leaving_a_loop_are_double_word() {
    let mut fx = Fx::new();
    let d = fx.input();
    let lo = fx.plus(d, 0xffff_fff6);
    let above = fx.minus(0x2a, d);
    let below = fx.plus(d, 0xffff_ffd6);
    fx.shift(RIGHT, lo);
    fx.shift(LEFT, above);
    fx.shift(RIGHT, below);
    let (peeled_above, peeled_below) = (fx.k(0xffff_ffe4), fx.k(0x1c));
    let exit_above = fx.op(OpCode::CPUI_MULTIEQUAL, &[peeled_above, above]);
    let exit_below = fx.op(OpCode::CPUI_MULTIEQUAL, &[peeled_below, below]);
    let exit_above = fx.low_byte(exit_above);
    let exit_below = fx.low_byte(exit_below);
    fx.shift(LEFT, exit_above);
    fx.shift(LEFT, exit_below);
    assert!(double_word_count(&fx.fd, exit_above));
    assert!(double_word_count(&fx.fd, exit_below));
}

#[test]
fn a_join_of_plain_counts_is_not_double_word() {
    let mut fx = Fx::new();
    let (a, b) = (fx.input(), fx.input());
    let joined = fx.op(OpCode::CPUI_MULTIEQUAL, &[a, b]);
    let count = fx.low_byte(joined);
    let below = fx.plus(joined, 0xffff_ffe0);
    fx.shift(LEFT, count);
    fx.shift(LEFT, below);
    fx.shift(LEFT, a);
    assert!(!double_word_count(&fx.fd, count));
}

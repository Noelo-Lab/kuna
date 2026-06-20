// Tests for the `constseq` ArraySequence base (included into `mod tests` in
// constseq.rs).
//
// Covered:
//   - WriteNode block-order comparator (drives check_interference's sort).
//   - interfere_between: benign markers (INDIRECT) pass, a `special` interloper
//     (LOAD) blocks.
//   - check_interference: maximal-no-interference window around the root, and
//     the MINIMUM_SEQUENCE_LENGTH rejection.
//   - form_byte_array: little/big-endian byte assembly, null-terminator handling,
//     leading-contiguity count, and trailing-op truncation when a gap appears.

use super::*;

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use kuna_base::types::{int4, uint4};

use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use crate::op::pcodeop_flags;
use crate::seams::{Architecture, BlockId, OpId, TypeOp, VarnodeId};

fn build_manager(big: bool) -> AddrSpaceManager {
    let mut m = AddrSpaceManager::new();
    m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
    m.insert_space(Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        big,
        8,
        1,
        2,
        addrspace_flags::hasphysical,
        1,
        1,
    )))
    .unwrap();
    m
}

fn build_fd(big: bool) -> Funcdata {
    let manage = build_manager(big);
    let glb = Rc::new(Architecture::new(manage));
    let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
    let addr = Address::new(ram, 0x1000);
    Funcdata::new("func", "func", glb, addr, 0x1000_0000, 0x40).unwrap()
}

fn ram(fd: &Funcdata) -> Rc<AddrSpace> {
    Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap())
}

fn char1(_fd: &Funcdata) -> Rc<Datatype> {
    Rc::new(Datatype::new(1, type_metatype::TYPE_INT))
}

fn op_flags_for(opc: OpCode) -> uint4 {
    // The eval-type flag is what `interfere_between` reads.  LOAD/STORE are
    // `special`; INDIRECT/CALLOTHER/SEGMENTOP/CPOOLREF/NEW are `special` benign
    // markers; everything else here we leave non-special.
    match opc {
        OpCode::CPUI_LOAD
        | OpCode::CPUI_STORE
        | OpCode::CPUI_INDIRECT
        | OpCode::CPUI_CALLOTHER
        | OpCode::CPUI_SEGMENTOP
        | OpCode::CPUI_CPOOLREF
        | OpCode::CPUI_NEW => pcodeop_flags::special,
        OpCode::CPUI_COPY => 0,
        _ => 0,
    }
}

fn mk_op_in_block(fd: &mut Funcdata, bl: BlockId, off: u64, opc: OpCode) -> OpId {
    let r = ram(fd);
    let op = fd.new_op(2, Address::new(r, off));
    fd.obank_mut()
        .change_opcode(op, TypeOp::new(opc, op_flags_for(opc), format!("{opc:?}")));
    fd.op_insert_end(op, bl);
    op
}

fn mk_block(fd: &mut Funcdata) -> BlockId {
    let root = fd.bblocks_ref().root.expect("bblocks root");
    fd.bblocks_mut().new_block_basic(root)
}

fn order_of(fd: &Funcdata, op: OpId) -> u32 {
    fd.obank().get(op).unwrap().get_seq_num().get_order()
}

/// A COPY whose input-0 is the given constant character.
fn mk_copy_const(fd: &mut Funcdata, bl: BlockId, off: u64, cval: u64) -> OpId {
    let op = mk_op_in_block(fd, bl, off, OpCode::CPUI_COPY);
    let c = fd.new_constant(1, cval);
    fd.op_set_input(op, c, 0).unwrap();
    op
}

// --- interfere_between --------------------------------------------------------

#[test]
fn interfere_between_indirect_is_benign_load_blocks() {
    let mut fd = build_fd(false);
    let bl = mk_block(&mut fd);
    let a = mk_op_in_block(&mut fd, bl, 0x10, OpCode::CPUI_COPY);
    let _ind = mk_op_in_block(&mut fd, bl, 0x14, OpCode::CPUI_INDIRECT);
    let b = mk_op_in_block(&mut fd, bl, 0x18, OpCode::CPUI_COPY);
    // INDIRECT between a and b is benign -> no interference.
    assert!(ArraySequence::interfere_between(&fd, a, b));

    let c = mk_op_in_block(&mut fd, bl, 0x1c, OpCode::CPUI_COPY);
    let _ld = mk_op_in_block(&mut fd, bl, 0x20, OpCode::CPUI_LOAD);
    let d = mk_op_in_block(&mut fd, bl, 0x24, OpCode::CPUI_COPY);
    // LOAD between c and d is a `special` interloper -> interference.
    assert!(!ArraySequence::interfere_between(&fd, c, d));
}

// --- WriteNode order + check_interference ------------------------------------

#[test]
fn check_interference_selects_max_window() {
    let mut fd = build_fd(false);
    let bl = mk_block(&mut fd);
    // Five consecutive COPYs, no interference; root is the third.
    let ops: Vec<OpId> =
        (0..5).map(|i| mk_copy_const(&mut fd, bl, 0x10 + i * 4, 0x41 + i)).collect();
    let ct = char1(&fd);
    let mut seq = ArraySequence::new(ct);
    for (i, &op) in ops.iter().enumerate() {
        seq.move_ops.push(WriteNode::new(i as u64, op, -1, order_of(&fd, op)));
    }
    let root = ops[2];
    assert!(seq.check_interference(&fd, root));
    // All five survive (>= MINIMUM_SEQUENCE_LENGTH, no gap).
    assert_eq!(seq.move_ops.len(), 5);
    // The window is block-ordered.
    let orders: Vec<u32> = seq.move_ops.iter().map(|n| n.order).collect();
    let mut sorted = orders.clone();
    sorted.sort();
    assert_eq!(orders, sorted);
}

#[test]
fn check_interference_rejects_short_window() {
    let mut fd = build_fd(false);
    let bl = mk_block(&mut fd);
    // root, then a LOAD interloper right after, then more COPYs -> the forward
    // window stops at the LOAD, leaving < MINIMUM_SEQUENCE_LENGTH.
    let root = mk_copy_const(&mut fd, bl, 0x10, 0x41);
    let _ld = mk_op_in_block(&mut fd, bl, 0x14, OpCode::CPUI_LOAD);
    let c1 = mk_copy_const(&mut fd, bl, 0x18, 0x42);
    let c2 = mk_copy_const(&mut fd, bl, 0x1c, 0x43);

    let ct = char1(&fd);
    let mut seq = ArraySequence::new(ct);
    for (i, &op) in [root, c1, c2].iter().enumerate() {
        seq.move_ops.push(WriteNode::new(i as u64, op, -1, order_of(&fd, op)));
    }
    // Only root survives forward (LOAD blocks); window length 1 < 4.
    assert!(!seq.check_interference(&fd, root));
}

#[test]
fn check_interference_missing_root_fails() {
    let mut fd = build_fd(false);
    let bl = mk_block(&mut fd);
    let a = mk_copy_const(&mut fd, bl, 0x10, 0x41);
    let b = mk_copy_const(&mut fd, bl, 0x14, 0x42);
    let stray = mk_copy_const(&mut fd, bl, 0x18, 0x43);
    let ct = char1(&fd);
    let mut seq = ArraySequence::new(ct);
    for (i, &op) in [a, b].iter().enumerate() {
        seq.move_ops.push(WriteNode::new(i as u64, op, -1, order_of(&fd, op)));
    }
    // root not in move_ops -> false.
    assert!(!seq.check_interference(&fd, stray));
}

// --- form_byte_array ----------------------------------------------------------

#[test]
fn form_byte_array_little_endian_with_terminator() {
    let mut fd = build_fd(false);
    let bl = mk_block(&mut fd);
    // "ABCD\0" : five 1-byte COPYs at offsets 0..4, the last a null terminator.
    let chars = [0x41u64, 0x42, 0x43, 0x44, 0x00];
    let ops: Vec<OpId> =
        chars.iter().enumerate().map(|(i, &c)| mk_copy_const(&mut fd, bl, 0x10 + i as u64 * 4, c)).collect();
    let ct = char1(&fd);
    let mut seq = ArraySequence::new(ct);
    for (i, &op) in ops.iter().enumerate() {
        seq.move_ops.push(WriteNode::new(i as u64, op, -1, order_of(&fd, op)));
    }
    // root offset 0, slot 0 (COPY input), little-endian, sz covers all 5.
    let count = seq.form_byte_array(&fd, 5, 0, 0, false);
    // 4 real chars + a single null terminator counted -> 5.
    assert_eq!(count, 5);
    assert_eq!(&seq.byte_array[..5], &[0x41, 0x42, 0x43, 0x44, 0x00]);
}

#[test]
fn form_byte_array_rejects_too_short() {
    let mut fd = build_fd(false);
    let bl = mk_block(&mut fd);
    // Only 3 contiguous chars -> below MINIMUM_SEQUENCE_LENGTH (4).
    let chars = [0x41u64, 0x42, 0x43];
    let ops: Vec<OpId> =
        chars.iter().enumerate().map(|(i, &c)| mk_copy_const(&mut fd, bl, 0x10 + i as u64 * 4, c)).collect();
    let ct = char1(&fd);
    let mut seq = ArraySequence::new(ct);
    for (i, &op) in ops.iter().enumerate() {
        seq.move_ops.push(WriteNode::new(i as u64, op, -1, order_of(&fd, op)));
    }
    let count = seq.form_byte_array(&fd, 3, 0, 0, false);
    assert_eq!(count, 0);
}

#[test]
fn form_byte_array_truncates_after_gap() {
    let mut fd = build_fd(false);
    let bl = mk_block(&mut fd);
    // chars at offsets 0,1,2,3 then a stray char at offset 5 (gap at 4).
    // moveOps offsets: 0,1,2,3,5 ; root 0 ; sz 6.  The contiguous run is 0..3
    // (4 chars, no terminator) -> count 4, and the stray at offset 5 is dropped.
    let entries = [(0u64, 0x41u64), (1, 0x42), (2, 0x43), (3, 0x44), (5, 0x45)];
    let ct = char1(&fd);
    let mut seq = ArraySequence::new(ct);
    for (i, &(off, cval)) in entries.iter().enumerate() {
        let op = mk_copy_const(&mut fd, bl, 0x10 + i as u64 * 4, cval);
        seq.move_ops.push(WriteNode::new(off, op, -1, order_of(&fd, op)));
    }
    let count = seq.form_byte_array(&fd, 6, 0, 0, false);
    assert_eq!(count, 4);
    // The trailing op (offset 5 >= maxOff 4) is dropped.
    assert_eq!(seq.move_ops.len(), 4);
    assert!(seq.move_ops.iter().all(|n| n.offset < 4));
}

// --- w6-s5 verifier adversarial probes (item w6-s5-constseq-prefersplit) ------

/// A WriteNode whose `offset` is strictly less than `root_off` makes the C++
/// `int4 bytePos = moveOps[i].offset - rootOff` go negative; C++ `continue`s on
/// `bytePos < 0`.  The Rust port must reproduce the wrapping-subtraction +
/// `< 0` skip WITHOUT panicking (the offset is data-controlled).  Here one op
/// sits *before* the root offset and must be silently skipped, leaving the four
/// in-range chars to form a valid count.
#[test]
fn w6s5_form_byte_array_offset_before_root_is_skipped_not_panic() {
    let mut fd = build_fd(false);
    let bl = mk_block(&mut fd);
    // root_off = 4. One op at offset 0 (bytePos = 0 - 4 = -4 -> skip), then four
    // in-range chars at offsets 4,5,6,7 (bytePos 0,1,2,3).
    let entries = [(0u64, 0x39u64), (4, 0x41), (5, 0x42), (6, 0x43), (7, 0x44)];
    let ct = char1(&fd);
    let mut seq = ArraySequence::new(ct);
    for (i, &(off, cval)) in entries.iter().enumerate() {
        let op = mk_copy_const(&mut fd, bl, 0x10 + i as u64 * 4, cval);
        seq.move_ops.push(WriteNode::new(off, op, -1, order_of(&fd, op)));
    }
    // sz = 4 (region after root), root_off = 4.
    let count = seq.form_byte_array(&fd, 4, 0, 4, false);
    assert_eq!(count, 4, "the pre-root op is skipped, four chars remain");
    assert_eq!(&seq.byte_array[..4], &[0x41, 0x42, 0x43, 0x44]);
}

/// FINDING F1 (minor, debug-only divergence).  `form_byte_array`'s in-range
/// guard is C++ `bytePos + elSize > sz`.  When `bytePos` is a large positive
/// `int4` (the wrapped result of `offset - rootOff` when `offset` is far above
/// `rootOff`) the C++ evaluates `bytePos + elSize` in signed `int4`, *wraps* on
/// overflow (UB but in practice wraps), and the `> sz` test still drops the op,
/// then `continue`s — no crash, the remaining chars still form a valid count.
/// The Rust `byte_pos + el_size` (constseq.rs:276) is a *checked* `i32` add that
/// PANICS in debug builds at `byte_pos == i32::MAX`, diverging from the C++
/// silent-drop.  This `#[should_panic]` pins the *current* (divergent) behavior;
/// the fix is `byte_pos.wrapping_add(el_size)` (matching the C++ int4 wrap).
/// Reachable only via the `pub(crate)` reuse surface / synthetic offsets, not
/// the bounded `StringSequence` caller — hence minor, not a blocker.
#[test]
#[should_panic(expected = "add with overflow")]
fn w6s5_form_byte_array_huge_bytepos_diverges_debug_panic() {
    let mut fd = build_fd(false);
    let bl = mk_block(&mut fd);
    let huge_off = i32::MAX as u64; // 0x7fff_ffff
    let entries = [(huge_off, 0x39u64), (0, 0x41), (1, 0x42), (2, 0x43), (3, 0x44)];
    let ct = char1(&fd);
    let mut seq = ArraySequence::new(ct);
    for (i, &(off, cval)) in entries.iter().enumerate() {
        let op = mk_copy_const(&mut fd, bl, 0x10 + i as u64 * 4, cval);
        seq.move_ops.push(WriteNode::new(off, op, -1, order_of(&fd, op)));
    }
    // C++ would drop the huge-offset op and return count 4; Rust panics here.
    let _ = seq.form_byte_array(&fd, 4, 0, 0, false);
}

/// Little-endian multi-byte unpacking: the C++ `val >>= 8` per element byte.
/// A 2-byte char type with value 0x4142 little-endian must lay [0x42,0x41].
#[test]
fn w6s5_form_byte_array_little_endian_multibyte_shift() {
    let mut fd = build_fd(false);
    let bl = mk_block(&mut fd);
    let ct: Rc<Datatype> = Rc::new(Datatype::new(2, type_metatype::TYPE_INT));
    let vals = [0x4142u64, 0x4344, 0x4546, 0x4748];
    let mut seq = ArraySequence::new(Rc::clone(&ct));
    for (i, &v) in vals.iter().enumerate() {
        let op = mk_op_in_block(&mut fd, bl, 0x10 + i as u64 * 4, OpCode::CPUI_COPY);
        let c = fd.new_constant(2, v);
        fd.op_set_input(op, c, 0).unwrap();
        seq.move_ops.push(WriteNode::new((i * 2) as u64, op, -1, order_of(&fd, op)));
    }
    let count = seq.form_byte_array(&fd, 8, 0, 0, false);
    assert_eq!(count, 4);
    // little-endian: 0x4142 -> [0x42,0x41], 0x4344 -> [0x44,0x43], ...
    assert_eq!(
        &seq.byte_array[..8],
        &[0x42, 0x41, 0x44, 0x43, 0x46, 0x45, 0x48, 0x47]
    );
}

#[test]
fn form_byte_array_big_endian_multibyte() {
    let mut fd = build_fd(true);
    let bl = mk_block(&mut fd);
    // Use a 2-byte char type so the big-endian byte unpacking matters.
    let ct: Rc<Datatype> = Rc::new(Datatype::new(2, type_metatype::TYPE_INT));
    // four 2-byte values 0x4142, 0x4344, 0x4546, 0x4748 at byte offsets 0,2,4,6.
    let vals = [0x4142u64, 0x4344, 0x4546, 0x4748];
    let mut seq = ArraySequence::new(Rc::clone(&ct));
    for (i, &v) in vals.iter().enumerate() {
        let op = mk_op_in_block(&mut fd, bl, 0x10 + i as u64 * 4, OpCode::CPUI_COPY);
        let c = fd.new_constant(2, v);
        fd.op_set_input(op, c, 0).unwrap();
        seq.move_ops.push(WriteNode::new((i * 2) as u64, op, -1, order_of(&fd, op)));
    }
    let count = seq.form_byte_array(&fd, 8, 0, 0, true);
    assert_eq!(count, 4);
    // big-endian: 0x4142 -> [0x41,0x42], etc.
    assert_eq!(
        &seq.byte_array[..8],
        &[0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48]
    );
}

// --- RuleStringCopy / RuleStringStore ----------------------------------------
//
// The drivers' deep bodies are seam-blocked (W4 symbol table + W6 type-facing),
// so these cover the reachable surface: getOpList, group-filtered clone, the
// constant-input guard, and the seam decline (a constant-input COPY/STORE still
// returns 0 because the build/transform is deferred — byte-identical to the
// rule being disabled).

use crate::action::{ActionGroupList, Rule};

#[test]
fn string_copy_op_list_is_copy() {
    assert_eq!(RuleStringCopy::new("analysis").get_op_list(), vec![OpCode::CPUI_COPY]);
}

#[test]
fn string_store_op_list_is_store() {
    assert_eq!(RuleStringStore::new("analysis").get_op_list(), vec![OpCode::CPUI_STORE]);
}

#[test]
fn string_rules_clone_filters_by_group() {
    let inc = ActionGroupList::from_names(["analysis"]);
    let exc = ActionGroupList::from_names(["cleanup"]);
    assert!(RuleStringCopy::new("analysis").clone_rule(&inc).is_some());
    assert!(RuleStringCopy::new("analysis").clone_rule(&exc).is_none());
    assert!(RuleStringStore::new("analysis").clone_rule(&inc).is_some());
    assert!(RuleStringStore::new("analysis").clone_rule(&exc).is_none());
}

#[test]
fn string_copy_non_constant_input_declines() {
    // A COPY whose input-0 is NOT constant fails the first guard.
    let mut fd = build_fd(false);
    let bl = mk_block(&mut fd);
    let op = mk_op_in_block(&mut fd, bl, 0x10, OpCode::CPUI_COPY);
    // input-0 is a (written) non-constant — a fresh input varnode.
    let nonconst = make_input(&mut fd, 0x100, 1);
    fd.op_set_input(op, nonconst, 0).unwrap();
    assert_eq!(RuleStringCopy::new("analysis").apply_op(op, &mut fd), 0);
}

#[test]
fn string_copy_constant_input_declines_at_seam() {
    // A constant-input COPY passes the ported guard but the W4/W6-seamed
    // StringSequence build is deferred, so the rule still declines (0).
    let mut fd = build_fd(false);
    let bl = mk_block(&mut fd);
    let op = mk_copy_const(&mut fd, bl, 0x10, 0x41);
    assert_eq!(RuleStringCopy::new("analysis").apply_op(op, &mut fd), 0);
}

#[test]
fn string_store_constant_value_declines_at_seam() {
    // STORE(space, ptr, const): input-2 is the constant value.  The ported guard
    // passes; the HeapSequence build is deferred, so the rule declines (0).
    let mut fd = build_fd(false);
    let bl = mk_block(&mut fd);
    let st = mk_op_in_block(&mut fd, bl, 0x10, OpCode::CPUI_STORE);
    // Slot 0: space id (constant), slot 1: pointer (input), slot 2: value (const).
    let spc = fd.new_constant(8, 0);
    let ptr = make_input(&mut fd, 0x200, 8);
    let val = fd.new_constant(1, 0x41);
    fd.obank_mut().get_mut(st).unwrap().set_num_inputs(3);
    fd.op_set_input(st, spc, 0).unwrap();
    fd.op_set_input(st, ptr, 1).unwrap();
    fd.op_set_input(st, val, 2).unwrap();
    assert_eq!(RuleStringStore::new("analysis").apply_op(st, &mut fd), 0);
}

/// A fresh input varnode at register-like offset `off` of `size` bytes.
fn make_input(fd: &mut Funcdata, off: u64, size: int4) -> VarnodeId {
    let r = ram(fd);
    let vn = fd.new_varnode(size, &Address::new(r, off), None);
    fd.set_input_varnode(vn).unwrap()
}

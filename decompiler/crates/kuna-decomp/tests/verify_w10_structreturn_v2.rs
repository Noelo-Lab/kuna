//! INDEPENDENT VERIFIER adversarial tests for item
//! `w10-structreturn-v2` — the struct-return field-binding substrate:
//!
//!   * `HighVariable::establishGroupSymbolOffset` (C++ `variable.cc:623`):
//!     `off = clamp(symboloffset, >=0) - piece.getOffset(); group.setSymbolOffset(off)`.
//!     The single piece of arithmetic the whole `v1.A`/`v1.B` render hangs on:
//!     a wrong sign or a missing `clamp` would mis-key every field.
//!   * the derived per-piece in-symbol offset
//!     `piece.getOffset() + group.getSymbolOffset()` (C++ `HighVariable::setSymbol`,
//!     `variable.cc:259`) — what a piece's HighVariable renders through.
//!   * the `off < 0` invariant `throw` (C++ `variable.cc:631`) surfacing as `Err`.
//!
//! These drive ONLY the PUBLIC [`HighVariableBank`] surface (`new_high`,
//! `group_with`, `set_symbol_offset`, `establish_group_symbol_offset`,
//! `group_symbol_offset`, `high_piece_id`, `piece_offset`) — building a two-piece
//! group exactly the way `Merge::groupPartialRoot` does (`groupWith(typeOffset -
//! baseOffset, root)` per leaf), so the arithmetic is exercised against the real
//! group machinery, not a mock.  Every assertion is keyed to generic offset
//! geometry — never a struct/field/function name or a magic constant — so it
//! cannot be satisfied by a special-cased decode.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use kuna_base::types::int4;

use kuna_decomp::dtype::{type_metatype, Datatype};
use kuna_decomp::variable::HighVariableBank;
use kuna_decomp::varnode::VarnodeBank;

fn dt(size: int4) -> Rc<Datatype> {
    Rc::new(Datatype::new(size, type_metatype::TYPE_UNKNOWN))
}

fn manager() -> AddrSpaceManager {
    let mut m = AddrSpaceManager::new();
    m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
    m.insert_space(Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        false,
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

fn ram(m: &AddrSpaceManager, off: u64) -> Address {
    Address::new(Rc::clone(m.get_space(2).unwrap()), off)
}

/// Build a root (size `root_size`) HighVariable and two field-piece HighVariables
/// at `pieces` (byte-offset, byte-size), grouped the way `groupPartialRoot` does:
/// each piece is `groupWith(offset, root)`'d into the root, which sits at group
/// offset 0.  Returns (bank, root_high, [piece_highs]).
fn build_struct_group(
    m: &AddrSpaceManager,
    root_size: int4,
    pieces: &[(int4, int4)],
) -> (HighVariableBank, kuna_decomp::seams::HighVariableId, Vec<kuna_decomp::seams::HighVariableId>) {
    let mut vbank = VarnodeBank::new(m, 0).unwrap();
    let mut hb = HighVariableBank::new();

    let root_vn = vbank.create(root_size, ram(m, 0x100), dt(root_size));
    let root_high = hb.new_high(root_vn);

    let mut piece_highs = Vec::new();
    for &(off, sz) in pieces {
        let pvn = vbank.create(sz, ram(m, 0x200 + off as u64), dt(sz));
        let ph = hb.new_high(pvn);
        // Merge::groupPartialRoot: nodeVn->getHigh()->groupWith(typeOffset, root).
        // (first_size / hi2_first_size are the C++ `getTypeSize` args; the offset
        // geometry — what these tests probe — is independent of them.)
        hb.group_with(ph, off, root_high, sz, root_size).unwrap();
        piece_highs.push(ph);
    }
    (hb, root_high, piece_highs)
}

/// AT1 — the canonical whole-symbol root (`symboloffset == -1`, the default a
/// struct-return root carries): every piece's in-symbol offset equals its own
/// group offset.  This is the `concatreturn`/`zerofield` geometry: A@0, B@4.
#[test]
fn at1_whole_symbol_root_pieces_offset_equals_group_offset() {
    let m = manager();
    let (mut hb, root, pieces) = build_struct_group(&m, 8, &[(0, 4), (4, 4)]);

    // Root maps the whole symbol: symboloffset stays -1 (the HighVariable default).
    assert_eq!(hb.group_symbol_offset(root), 0, "fresh group has no symbol offset yet");

    // establishGroupSymbolOffset: off = clamp(-1,>=0) - rootPieceOffset(0) = 0.
    hb.establish_group_symbol_offset(root).expect("whole-symbol root must establish cleanly");
    assert_eq!(hb.group_symbol_offset(root), 0);

    // Each piece's in-symbol byte offset = piece.getOffset() + group.getSymbolOffset().
    for (i, &ph) in pieces.iter().enumerate() {
        let pid = hb.high_piece_id(ph).expect("piece must be grouped");
        let group_off = hb.piece_offset(pid);
        let in_symbol_off = group_off + hb.group_symbol_offset(root);
        let expected = [0, 4][i];
        assert_eq!(group_off, expected, "piece {i} group offset");
        assert_eq!(in_symbol_off, expected, "piece {i} in-symbol offset (field key)");
    }
    // The two pieces and the root share ONE group (the isSameGroup precondition).
    assert!(hb.is_same_group(root, pieces[0]));
    assert!(hb.is_same_group(pieces[0], pieces[1]));
}

/// AT2 — a NON-zero root symbol offset shifts every field key by the same amount
/// (the root is itself a partial cover of a bigger symbol).  Proves the binding
/// math tracks `group.getSymbolOffset()` and is not hardcoded to the group offset.
/// fooshort-style four fields a@0 b@2 c@4 d@6, root symboloffset = 2.
#[test]
fn at2_nonzero_root_symbol_offset_shifts_all_field_keys() {
    let m = manager();
    let (mut hb, root, pieces) = build_struct_group(&m, 8, &[(0, 2), (2, 2), (4, 2), (6, 2)]);

    // Root is a partial cover of a larger symbol at byte 2.
    hb.get_mut(root).unwrap().set_symbol_offset(2);
    // off = clamp(2,>=0) - rootPieceOffset(0) = 2.
    hb.establish_group_symbol_offset(root).expect("non-negative root offset establishes");
    assert_eq!(hb.group_symbol_offset(root), 2);

    for (i, &ph) in pieces.iter().enumerate() {
        let pid = hb.high_piece_id(ph).unwrap();
        let in_symbol_off = hb.piece_offset(pid) + hb.group_symbol_offset(root);
        // Every field key is shifted up by the root's symbol offset (2).
        assert_eq!(in_symbol_off, [2, 4, 6, 8][i], "piece {i} shifted field key");
    }
}

/// AT3 — the `off < 0` invariant (`variable.cc:631` `throw`) surfaces as `Err`
/// rather than a panic, so the caller's `bind_proto_partial_piece` can fall back.
/// A root whose symbol offset is SMALLER than its own group offset is
/// incompatible: the root sits at group offset 4 but claims symbol offset 0.
#[test]
fn at3_incompatible_root_symbol_offset_is_err_not_panic() {
    let m = manager();
    // Two pieces; make the *root* a grouped piece sitting at a non-zero group
    // offset by grouping it under a wider container, then give it a symbol offset
    // below its group offset.
    let mut vbank = VarnodeBank::new(&m, 0).unwrap();
    let mut hb = HighVariableBank::new();
    let container_vn = vbank.create(8, ram(&m, 0x300), dt(8));
    let container = hb.new_high(container_vn);
    let root_vn = vbank.create(4, ram(&m, 0x304), dt(4));
    let root = hb.new_high(root_vn);
    // root sits at group offset 4 within the container.
    hb.group_with(root, 4, container, 4, 8).unwrap();
    let pid = hb.high_piece_id(root).unwrap();
    assert_eq!(hb.piece_offset(pid), 4, "root must sit at group offset 4");

    // symboloffset 0 < pieceOffset 4 -> off = 0 - 4 = -4 < 0 -> Err.
    hb.get_mut(root).unwrap().set_symbol_offset(0);
    assert!(
        hb.establish_group_symbol_offset(root).is_err(),
        "symbol offset incompatible with group offset must Err"
    );

    // A compatible offset (>= group offset) establishes cleanly.
    hb.get_mut(root).unwrap().set_symbol_offset(4);
    assert!(hb.establish_group_symbol_offset(root).is_ok());
    assert_eq!(hb.group_symbol_offset(root), 0, "off = 4 - 4 = 0");
}

/// AT4 — a groupless high (no `piece`) is a no-op for `establish_group_symbol_offset`
/// (the kuna guard for the C++ unconditional `piece->getGroup()` deref) and its
/// `group_symbol_offset` is 0 — the binding branch only fires for genuine pieces.
#[test]
fn at4_groupless_high_is_noop() {
    let m = manager();
    let mut vbank = VarnodeBank::new(&m, 0).unwrap();
    let mut hb = HighVariableBank::new();
    let vn = vbank.create(4, ram(&m, 0x400), dt(4));
    let lone = hb.new_high(vn);

    assert!(hb.high_piece_id(lone).is_none(), "ungrouped high has no piece");
    assert_eq!(hb.group_symbol_offset(lone), 0);
    // No panic, returns Ok.
    assert!(hb.establish_group_symbol_offset(lone).is_ok());
    // Two unrelated lone highs are never the same group.
    let vn2 = vbank.create(4, ram(&m, 0x404), dt(4));
    let lone2 = hb.new_high(vn2);
    assert!(!hb.is_same_group(lone, lone2));
}

// ---------------------------------------------------------------------------
// AT5 — the `PieceNode::findRoot` multi-PIECE tie-break (C++ `op.cc:870-875`),
// driven END-TO-END through the production `Funcdata::piece_find_root`.
//
//   if (pieceOp != 0) {                   // more than one valid PIECE
//     if (op->compareOrder(pieceOp))      // <-- `if (non-zero)`: -1 OR +1
//       pieceOp = op;
//   }
//
// `PcodeOp::compareOrder` (`op.cc:808`) returns -1 / +1 / 0.  In C++ `if(-1)` and
// `if(+1)` are BOTH true: the incumbent `pieceOp` is replaced whenever the new
// candidate is *comparable* (either control-flow order) and kept only when the two
// are incomparable (0).  The faithful Rust predicate is therefore `!= 0`; a `< 0`
// guard replaces ONLY on -1 and silently keeps the incumbent on +1.
//
// This builds the exact shape a struct-return CONCAT leaf produces — ONE
// proto-partial leaf varnode read at slot 1 by TWO PIECE ops in the SAME basic
// block, with distinct outputs that both renormalize back to the leaf's address
// (so both are "valid" PIECE descendants) — and asserts which output
// `piece_find_root` walks up to.  The descend list is [piece_early, piece_late];
// `piece_late` is inserted last, so it carries the higher seq-num order and
// `compareOrder(piece_late, piece_early)` == +1.  The faithful `!= 0` therefore
// REPLACES the incumbent and the root walks to `out_late`; the buggy `< 0` keeps
// `piece_early` and walks to `out_early`.  The two outputs are distinct varnodes,
// so the returned root id distinguishes the two engines unambiguously.
#[test]
fn at5_findroot_tiebreak_must_replace_on_compareorder_nonzero_not_only_negative() {
    use kuna_base::address::Address;
    use kuna_decomp::funcdata::Funcdata;
    use kuna_decomp::seams::{Architecture, BlockId, OpId, TypeOp, VarnodeId};
    use kuna_num::opcodes::OpCode;

    fn build_fd() -> Funcdata {
        let glb = Rc::new(Architecture::new(manager()));
        let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
        let entry = Address::new(ram, 0x1000);
        Funcdata::new("func", "func", glb, entry, 0x10000000, 0x40).unwrap()
    }
    fn ram_addr(fd: &Funcdata, off: u64) -> Address {
        Address::new(Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap()), off)
    }

    let mut fd = build_fd();
    // A real BlockBasic under the graph root (the `bblocks_ref().root` is the
    // container/graph root, not a basic block — `op_insert` needs a BlockBasic).
    let root: BlockId = fd.bblocks_ref().root.expect("bblocks root");
    let bl: BlockId = fd.bblocks_mut().new_block_basic(root);
    fd.set_basic_block_range(bl, &ram_addr(&fd, 0x100), &ram_addr(&fd, 0x100));

    // The proto-partial CONCAT leaf: a WRITTEN varnode (output of a COPY) at ram
    // 0x200, size 4.  WRITTEN (not free) so the bank permits the two PIECE
    // descendants; `set_proto_partial` makes `piece_find_root`'s loop guard fire.
    let leaf_addr = ram_addr(&fd, 0x200);
    let src = fd.new_constant(4, 0);
    let copy = fd.new_op(1, ram_addr(&fd, 0x100));
    fd.op_set_opcode(copy, TypeOp::new(OpCode::CPUI_COPY, 0, "COPY"));
    fd.op_set_input(copy, src, 0).unwrap();
    let leaf: VarnodeId = fd.new_varnode_out(4, &leaf_addr, copy).unwrap();
    fd.op_insert_end(copy, bl);
    fd.vbank_mut().get_mut(leaf).unwrap().set_proto_partial();

    // Build one PIECE op `CONCAT(hi, leaf)` with output at `leaf_addr` size 8.
    // Slot 1 (leaf is the low piece) in little-endian ram: the
    // `isBigEndian()==(slot==1)` predicate is `false==true`==false, so no address
    // shift — the output's address IS the leaf's address and the PIECE is "valid".
    let piece_at = |fd: &mut Funcdata, pc: u64, hi_off: u64| -> (OpId, VarnodeId) {
        let hi = fd.new_varnode(4, &ram_addr(fd, hi_off), None);
        let op = fd.new_op(2, ram_addr(fd, pc));
        fd.op_set_opcode(op, TypeOp::new(OpCode::CPUI_PIECE, 0, "CONCAT"));
        fd.op_set_input(op, hi, 0).unwrap();
        fd.op_set_input(op, leaf, 1).unwrap();
        let out = fd.new_varnode_out(8, &leaf_addr, op).unwrap();
        fd.op_insert_end(op, bl);
        (op, out)
    };
    // `piece_early` is created/inserted FIRST -> earlier in the leaf's descend list
    // AND lower seq-num order; `piece_late` SECOND -> later on both.
    let (_early, out_early) = piece_at(&mut fd, 0x101, 0x300);
    let (_late, out_late) = piece_at(&mut fd, 0x102, 0x304);
    assert_ne!(out_early, out_late, "the two PIECE outputs must be distinct varnodes");

    let root = fd.piece_find_root(leaf);

    // Faithful C++ `findRoot`: the +1 (`piece_late` later than `piece_early`)
    // compareOrder is truthy, so the incumbent is REPLACED and the root path runs
    // through `piece_late` -> `out_late`.  The buggy `< 0` guard would keep
    // `piece_early` and return `out_early`.
    assert_eq!(
        root, out_late,
        "F1: piece_find_root must REPLACE the incumbent on a +1 (later same-block) \
         compareOrder, per C++ `if (op->compareOrder(pieceOp))` (op.cc:871, `!= 0`); \
         the shipped `< 0` guard returns out_early instead"
    );
    assert_ne!(
        root, out_early,
        "the `< 0` guard's (incorrect) selection must NOT be what production returns"
    );
}

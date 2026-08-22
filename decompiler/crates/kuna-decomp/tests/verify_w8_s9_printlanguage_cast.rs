//! Adversarial verifier tests for `w8-s9-printlanguage-cast`.
//!
//! Protocol: docs/rust-port/verification.md.  These target the hunt-list spots
//! flagged as most fragile for this item, with the C++ oracle re-derived from
//! `decompiler/cpp/{cast,printlanguage}.cc`:
//!
//!   - `is_subpiece_cast_endian` width/wrapping arithmetic (cast.cc:434-441):
//!     the one bare `as uint4` + `wrapping_sub` chain in the port.  The C++
//!     computes `intype->getSize()-1-offset` where `getSize()` is `int4` and
//!     `offset` is `uint4`; the subtraction happens in `uint4` and may wrap
//!     below zero.  Verify the Rust reproduces the wrap (no panic, same value).
//!   - `mark_explicit_long_size` signbit off-by-one (cast.cc:93-101): the
//!     `promoteSize*8-1` vs `promoteSize*8` guard boundary.
//!   - `most_natural_base` / `format_binary` number formatting boundaries
//!     (this wave's cfmt-formatting hunt; printlanguage.cc:738-825).
//!   - `parentheses` HiddenFunction equal-precedence arm (printlanguage.cc:
//!     313-323): equal precedence must keep the parens (fall through to `true`).
//!   - `cast_standard` pointer-chain `care_uint_int` reset ordering
//!     (cast.cc:300-392).

use std::rc::Rc;

use kuna_base::types::{int4, uint4, uintb};
use kuna_decomp::cast::{
    CastContext, CastStrategy, CastStrategyC, IntPromotionCode, OpRef, VnRef,
};
use kuna_decomp::dtype::{type_metatype, Datatype, DatatypeKind, TypeFactoryImpl};
use kuna_decomp::printlanguage::{
    format_binary, most_natural_base, parentheses, unicode_needs_escape, OpToken, ReversePolish,
    TokenType,
};
use kuna_num::opcodes::OpCode;

// ---------------------------------------------------------------------------
// data-type builders (mirror cast/tests.rs)
// ---------------------------------------------------------------------------

fn base(size: int4, meta: type_metatype) -> Rc<Datatype> {
    Rc::new(Datatype::new(size, meta))
}
fn int_t(size: int4) -> Rc<Datatype> {
    base(size, type_metatype::TYPE_INT)
}
fn uint_t(size: int4) -> Rc<Datatype> {
    base(size, type_metatype::TYPE_UINT)
}

fn ptr_to(ptrto: Rc<Datatype>, wordsize: uint4) -> Rc<Datatype> {
    let mut t = Datatype::new_with_align(8, -1, type_metatype::TYPE_PTR);
    t.kind = DatatypeKind::Pointer { ptrto, spaceid: None, truncate: None, wordsize };
    Rc::new(t)
}

fn strategy_c() -> CastStrategyC {
    let f = TypeFactoryImpl::new();
    f.set_default_alignment_map();
    f.set_max_basetype_size(8);
    f.setup_sizes(Some(4), 8, 8);
    CastStrategyC::new(Rc::new(f))
}

// ---------------------------------------------------------------------------
// F-target 1: is_subpiece_cast_endian width/wrapping (cast.cc:434-441)
// ---------------------------------------------------------------------------

#[test]
fn subpiece_cast_endian_bigend_boundary_and_wrap() {
    let s = strategy_c();
    let inty = int_t(4);
    let outy = int_t(4);

    // big-endian, offset == size-1 -> tmpoff = 4-1-3 = 0 -> isSubpieceCast(.,.,0)
    // INT/INT offset 0 -> true.
    assert!(s.is_subpiece_cast_endian(&outy, &inty, 3, true));

    // big-endian, offset 0 -> tmpoff = 4-1-0 = 3 (nonzero) -> false.
    assert!(!s.is_subpiece_cast_endian(&outy, &inty, 0, true));

    // big-endian, offset == size -> tmpoff = 4-1-4 = -1, computed in uint4 wraps
    // to 0xFFFFFFFF (nonzero) -> false.  The C++ does `getSize()-1-offset` in
    // uint4 and silently wraps; the Rust must NOT panic and must agree.
    assert!(!s.is_subpiece_cast_endian(&outy, &inty, 4, true));

    // offset way past size: 4-1-100 wraps to 0xFFFFFF99 -> nonzero -> false.
    assert!(!s.is_subpiece_cast_endian(&outy, &inty, 100, true));

    // little-endian path ignores size: offset passes straight through.
    assert!(s.is_subpiece_cast_endian(&outy, &inty, 0, false)); // tmpoff 0 -> true
    assert!(!s.is_subpiece_cast_endian(&outy, &inty, 2, false)); // tmpoff 2 -> false
}

// ---------------------------------------------------------------------------
// F-target 2: mark_explicit_long_size signbit off-by-one (cast.cc:79-105)
// ---------------------------------------------------------------------------
//
// Minimal CastContext mock — just enough for markExplicitLongSize, which needs
// op_is_shift_op, op_in, vn_is_constant, vn_size, vn_high_type, vn_offset,
// vn_set_long_print.

struct LongCtx {
    size: int4,
    offset: uintb,
    ty: Rc<Datatype>,
    is_shift: bool,
    marked: std::cell::Cell<bool>,
}

impl CastContext for LongCtx {
    fn op_code(&self, _op: OpRef) -> OpCode {
        OpCode::CPUI_INT_LEFT
    }
    fn op_num_input(&self, _op: OpRef) -> int4 {
        2
    }
    fn op_in(&self, _op: OpRef, _slot: int4) -> VnRef {
        VnRef(0)
    }
    fn op_out(&self, _op: OpRef) -> Option<VnRef> {
        None
    }
    fn op_slot(&self, _op: OpRef, _vn: VnRef) -> int4 {
        0
    }
    fn vn_is_constant(&self, _vn: VnRef) -> bool {
        true
    }
    fn vn_is_explicit(&self, _vn: VnRef) -> bool {
        false
    }
    fn vn_is_written(&self, _vn: VnRef) -> bool {
        false
    }
    fn vn_size(&self, _vn: VnRef) -> int4 {
        self.size
    }
    fn vn_offset(&self, _vn: VnRef) -> uintb {
        self.offset
    }
    fn vn_def(&self, _vn: VnRef) -> Option<OpRef> {
        None
    }
    fn vn_lone_descend(&self, _vn: VnRef) -> Option<OpRef> {
        None
    }
    fn vn_high_type(&self, _vn: VnRef) -> Rc<Datatype> {
        Rc::clone(&self.ty)
    }
    fn vn_high_type_read_facing(&self, _vn: VnRef, _op: OpRef) -> Rc<Datatype> {
        Rc::clone(&self.ty)
    }
    fn op_inherits_sign(&self, _op: OpRef) -> bool {
        false
    }
    fn op_inherits_sign_first_param_only(&self, _op: OpRef) -> bool {
        false
    }
    fn op_is_shift_op(&self, _op: OpRef) -> bool {
        self.is_shift
    }
    fn op_is_bool_output(&self, _op: OpRef) -> bool {
        false
    }
    fn op_is_call(&self, _op: OpRef) -> bool {
        false
    }
    fn vn_set_unsigned_print(&mut self, _vn: VnRef) {}
    fn vn_set_long_print(&mut self, _vn: VnRef) {
        self.marked.set(true);
    }
}

fn long_size_marked(meta: type_metatype, size: int4, offset: uintb) -> bool {
    let s = strategy_c();
    let ty = base(size, meta);
    let mut ctx = LongCtx { size, offset, ty, is_shift: true, marked: std::cell::Cell::new(false) };
    s.mark_explicit_long_size(&mut ctx, OpRef(0), 0)
}

#[test]
fn mark_explicit_long_size_negative_int_boundary() {
    // promoteSize == 4, so the negative-INT guard is `bit >= 31`.
    // val 0xffffffff7fffffff: one's-complement negate -> 0x80000000, msb = 31,
    // `31 >= 31` -> NOT long.
    assert!(!long_size_marked(type_metatype::TYPE_INT, 8, 0xffffffff7fffffff));
    // val 0xffffffff80000000: negate -> 0x7fffffff, msb = 30, `30 >= 31` false
    // -> long.
    assert!(long_size_marked(type_metatype::TYPE_INT, 8, 0xffffffff80000000));
    // val 0xffffffffc0000000: negate -> 0x3fffffff, msb = 29, `29 >= 31` false
    // -> long.
    assert!(long_size_marked(type_metatype::TYPE_INT, 8, 0xffffffffc0000000));
}

#[test]
fn mark_explicit_long_size_unsigned_boundary_and_size_guard() {
    // UINT path (high bit not set as negative-INT): guard is `bit >= 32`.
    // 0xffffffff -> msb 31, `31 >= 32` false -> long.
    assert!(long_size_marked(type_metatype::TYPE_UINT, 8, 0xffffffff));
    // 0x100000000 -> msb 32, `32 >= 32` -> naturally long, NOT marked.
    assert!(!long_size_marked(type_metatype::TYPE_UINT, 8, 0x100000000));

    // Size guard: vn->getSize() <= promoteSize -> never long.  size 4 == 4.
    assert!(!long_size_marked(type_metatype::TYPE_UINT, 4, 0xff));

    // Non-shift op -> never long.
    let s = strategy_c();
    let ty = base(8, type_metatype::TYPE_UINT);
    let mut ctx =
        LongCtx { size: 8, offset: 0xffff, ty, is_shift: false, marked: std::cell::Cell::new(false) };
    assert!(!s.mark_explicit_long_size(&mut ctx, OpRef(0), 0));
}

// ---------------------------------------------------------------------------
// F-target 3: cfmt number-formatting boundaries (printlanguage.cc:738-825)
// ---------------------------------------------------------------------------

#[test]
fn most_natural_base_tie_break_boundaries() {
    // countdec switch arms (printlanguage.cc:759-774):
    // 90: 90%10=0 setdig, tmp=9; 9 != 0 -> countdec 1, tmp 9. case 1: tmp>1 -> hex.
    assert_eq!(most_natural_base(90), 16);
    // 100: %10=0 setdig, tmp=10; 10%10=0 -> countdec 2, tmp 1; 1%10=1 != 0 stop.
    //   case 2: tmp(1) > 10? no.  counthex: 100&0xf=4 not 0/f -> 0.  2>0 -> dec.
    assert_eq!(most_natural_base(100), 10);
    // 9: setdig 9, countdec 1, tmp 0. case 1: tmp>1?no, setdig==9 -> hex.
    assert_eq!(most_natural_base(9), 16);
    // 1000: trailing zeros run -> decimal.
    assert_eq!(most_natural_base(1000), 10);
    // 0xf0: hex 0-run/f-run -> hex; decimal 0xf0=240 %10=0,tmp24,4!=0 countdec1
    //   case1 tmp(24)>1 -> hex.
    assert_eq!(most_natural_base(0xf0), 16);
    // u64::MAX = 0xffff... : all hex f's; decimal last digit 5 -> countdec 0 -> hex.
    assert_eq!(most_natural_base(u64::MAX), 16);
}

#[test]
fn format_binary_width_boundaries() {
    let mut s = String::new();
    format_binary(&mut s, 0);
    assert_eq!(s, "0"); // pos < 0 -> single '0'

    // msb exactly 7 -> width 8 (not 16).
    let mut s = String::new();
    format_binary(&mut s, 0x80);
    assert_eq!(s, "10000000");

    // msb exactly 8 -> width 16.
    let mut s = String::new();
    format_binary(&mut s, 0x100);
    assert_eq!(s, "0000000100000000");

    // msb 15 -> width 16; msb 16 -> width 32.
    let mut s = String::new();
    format_binary(&mut s, 0x8000);
    assert_eq!(s.len(), 16);
    let mut s = String::new();
    format_binary(&mut s, 0x10000);
    assert_eq!(s.len(), 32);

    // msb 63 -> full 64 bits, all ones.
    let mut s = String::new();
    format_binary(&mut s, u64::MAX);
    assert_eq!(s.len(), 64);
    assert!(s.chars().all(|c| c == '1'));
}

#[test]
fn unicode_needs_escape_range_boundaries() {
    // exact range edges from printlanguage.cc:418-494.
    assert!(unicode_needs_escape(0x1f)); // last C0 control
    assert!(!unicode_needs_escape(0x20)); // space, printable
    assert!(!unicode_needs_escape(0x7e)); // '~', last printable ASCII before DEL
    assert!(unicode_needs_escape(0x7f)); // DEL
    assert!(unicode_needs_escape(0xa0)); // NBSP (not > 0xa0)
    assert!(!unicode_needs_escape(0xa1)); // first printable Latin-1
    assert!(unicode_needs_escape(0x2fa20)); // >= last defined language
    assert!(!unicode_needs_escape(0x2fa1f)); // just below
    assert!(unicode_needs_escape(0xd7fc)); // unassigned/surrogate region start
    assert!(!unicode_needs_escape(0xd7fb)); // just below
}

// ---------------------------------------------------------------------------
// F-target 4: parentheses HiddenFunction equal-precedence (printlanguage.cc:313-323)
// ---------------------------------------------------------------------------

fn mk(print1: &'static str, prec: int4, assoc: bool, tt: TokenType) -> OpToken {
    OpToken {
        print1,
        print2: "",
        stage: 1,
        precedence: prec,
        associative: assoc,
        token_type: tt,
        spacing: 1,
        bump: 0,
        negate: None,
        paren_before_angle: false,
    }
}

#[test]
fn parentheses_hiddenfunction_equal_precedence_keeps_parens() {
    let hidden: &'static OpToken =
        Box::leak(Box::new(mk("", 0, false, TokenType::HiddenFunction)));
    let op2: &'static OpToken = Box::leak(Box::new(mk("+", 40, true, TokenType::Binary)));
    let top = ReversePolish { tok: hidden, visited: 0, paren: false, op: None, id: 0, id2: 0 };

    // prev binary with EQUAL precedence to op2: C++ falls through (no `return
    // false`) to `return true` -> parens.  (The boundary the `<` guard guards.)
    let prev_eq = mk("*", 40, true, TokenType::Binary);
    assert!(parentheses(&top, op2, Some(&prev_eq)));

    // prev binary with strictly LOWER precedence -> `prev.precedence < op2` ->
    // return false (no parens).
    let prev_lo = mk("?", 30, true, TokenType::Binary);
    assert!(!parentheses(&top, op2, Some(&prev_lo)));

    // prev binary with HIGHER precedence -> not `<` -> parens.
    let prev_hi = mk("*", 50, true, TokenType::Binary);
    assert!(parentheses(&top, op2, Some(&prev_hi)));

    // prev token is neither binary nor unary_prefix -> return false.
    let prev_pre = mk("(", 50, false, TokenType::Presurround);
    assert!(!parentheses(&top, op2, Some(&prev_pre)));

    // No previous token (stack size 1) -> always true.
    assert!(parentheses(&top, op2, None));

    // stage != 0 -> the inner block is skipped, always true regardless of prev.
    let top_stage1 =
        ReversePolish { tok: hidden, visited: 1, paren: false, op: None, id: 0, id2: 0 };
    assert!(parentheses(&top_stage1, op2, Some(&prev_lo)));
}

#[test]
fn parentheses_binary_associative_pointer_identity() {
    // Equal precedence, associative, and the SAME static token pointer -> the
    // C++ `topToken == op2` pointer-identity check fires -> no parens.
    let mult: &'static OpToken = Box::leak(Box::new(mk("*", 50, true, TokenType::Binary)));
    let top = ReversePolish { tok: mult, visited: 1, paren: false, op: None, id: 0, id2: 0 };
    assert!(!parentheses(&top, mult, None));

    // A DIFFERENT token of equal precedence (distinct pointer) -> parens, even if
    // it is also associative.
    let other: &'static OpToken = Box::leak(Box::new(mk("&", 50, true, TokenType::Binary)));
    assert!(parentheses(&top, other, None));
}

// ---------------------------------------------------------------------------
// F-target 5: cast_standard pointer-chain care_uint_int reset (cast.cc:300-392)
// ---------------------------------------------------------------------------

#[test]
fn cast_standard_pointer_chain_forces_care_uint_int() {
    let s = strategy_c();
    // uint4* vs int4* with care_uint_int=FALSE at the top level.  Descending into
    // the pointers sets care_uint_int=true (cast.cc:322), so UINT-vs-INT under
    // the pointer DOES require a cast even though the caller passed false.
    let a = ptr_to(uint_t(4), 1);
    let b = ptr_to(int_t(4), 1);
    assert!(s.cast_standard(&a, &b, false, true).is_some());

    // Same two base types NOT under a pointer, care_uint_int=false: no cast
    // (the reset only happens inside the pointer loop).
    assert!(s.cast_standard(&uint_t(4), &int_t(4), false, true).is_none());

    // care_ptr_uint=false with a UINT requirement and a PTR current at top level
    // (no pointer descent on req side): the `!care_ptr_uint && PTR` arm -> no
    // cast (cast.cc:359-360).
    assert!(s.cast_standard(&uint_t(8), &ptr_to(int_t(4), 1), false, false).is_none());
    // ...but care_ptr_uint=true -> the size matches (both 8) and PTR is not in
    // the UINT no-cast set -> default arm -> cast.
    assert!(s.cast_standard(&uint_t(8), &ptr_to(int_t(4), 1), false, true).is_some());
}

#[test]
fn int_promotion_code_discriminants_match_cpp() {
    // The discriminants are load-bearing: int_promotion_type AND-combines them
    // (cast.hh:48-54).  Verify the raw values and the bit-algebra the port relies
    // on (UNSIGNED & SIGNED == 0, EITHER == UNSIGNED | SIGNED).
    assert_eq!(IntPromotionCode::NO_PROMOTION.as_i32(), -1);
    assert_eq!(IntPromotionCode::UNKNOWN_PROMOTION.as_i32(), 0);
    assert_eq!(IntPromotionCode::UNSIGNED_EXTENSION.as_i32(), 1);
    assert_eq!(IntPromotionCode::SIGNED_EXTENSION.as_i32(), 2);
    assert_eq!(IntPromotionCode::EITHER_EXTENSION.as_i32(), 3);
    assert_eq!(
        IntPromotionCode::UNSIGNED_EXTENSION.as_i32() & IntPromotionCode::SIGNED_EXTENSION.as_i32(),
        0
    );
    assert_eq!(
        IntPromotionCode::UNSIGNED_EXTENSION.as_i32() | IntPromotionCode::SIGNED_EXTENSION.as_i32(),
        IntPromotionCode::EITHER_EXTENSION.as_i32()
    );
}

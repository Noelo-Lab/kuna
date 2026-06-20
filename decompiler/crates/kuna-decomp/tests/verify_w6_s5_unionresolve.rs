//! Independent VERIFIER adversarial tests for item `w6-s5-unionresolve`
//! (`decompiler/cpp/unionresolve.{cc,hh}`).
//!
//! These re-derive the load-bearing, mechanically-error-prone parts of the port
//! directly against the C++ semantics, *not* the porter's own test scaffolding:
//!
//! - `ResolveEdge::operator<` total order — the `unionMap` key.  The `encoding`
//!   member is `int4` (**signed**); an output edge has `slot == -1`, so a wrong
//!   unsigned compare would mis-order it.  Also the address-form `opTime` is a
//!   `uintb`->`uintm` (u64->u32) truncating store.
//! - `scoreTruncation` scalar int/uint branch — the `ct->getSize() >=
//!   vn->getSize() + curOff` comparison is **signed `int8`** with a possibly
//!   negative `curOff` (offset − field offset, `unionresolve.cc:1210`).
//! - `scoreLockedType` asymmetric pointer chain — the `while` stops as soon as
//!   either side is not a pointer, then compares metatypes of *whatever is left*.
//! - `scoreConstantFit` float-normalized exponent window — exclusive bounds
//!   `exp < 7 && exp > -4`.
//! - `computeBestIndex` strict-`>` tie-break and single-element vector.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{AddrSpace, spacetype};
use kuna_base::types::int4;

use kuna_decomp::dtype::{
    Datatype, DatatypeKind, TypeFactoryImpl, TypeField, sub_metatype, type_metatype,
};
use kuna_decomp::unionresolve::{
    ConstantFitFacts, FloatConstFact, ResolveEdge, ResolvedUnion, compute_best_index,
    score_constant_fit, score_locked_type, score_truncation,
};

// ---- builders ---------------------------------------------------------------

fn base_type(size: int4, m: type_metatype) -> Rc<Datatype> {
    Rc::new(Datatype::new(size, m))
}

fn with_id(size: int4, m: type_metatype, id: u64) -> Rc<Datatype> {
    let mut dt = Datatype::new(size, m);
    dt.id = id;
    Rc::new(dt)
}

fn pointer_to(ptrto: Rc<Datatype>, size: int4, wordsize: u32) -> Rc<Datatype> {
    let mut p = Datatype::new(size, type_metatype::TYPE_PTR);
    p.submeta = sub_metatype::SUB_PTR;
    p.kind = DatatypeKind::Pointer { ptrto, spaceid: None, truncate: None, wordsize };
    Rc::new(p)
}

fn make_union(size: int4, fields: &[(int4, Rc<Datatype>)]) -> Rc<Datatype> {
    let mut u = Datatype::new(size, type_metatype::TYPE_UNION);
    u.submeta = sub_metatype::SUB_UNION;
    let field: Vec<TypeField> = fields
        .iter()
        .enumerate()
        .map(|(i, (off, ct))| TypeField::new(i as int4, *off, format!("f{i}"), Rc::clone(ct)))
        .collect();
    u.kind = DatatypeKind::Union { field };
    Rc::new(u)
}

// =============================================================================
// 1. ResolveEdge total order — signed encoding + addr-form opTime truncation
// =============================================================================

/// The `encoding` member is `int4` (signed).  An output access is `slot == -1`;
/// an input access is `slot >= 0`.  The C++ `operator<` orders by `encoding`
/// with a *signed* `<`, so the output edge (encoding −1) must sort **before**
/// any input edge (encoding 0, 1, …) sharing the same `typeId`.  A bug that
/// compared `encoding` as unsigned (u32) would put −1 == 0xFFFFFFFF *last*.
#[test]
fn resolve_edge_signed_encoding_orders_output_before_inputs() {
    let int_t = with_id(4, type_metatype::TYPE_INT, 77);
    // Output edge: slot = -1  => encoding = -1.
    let out_edge = ResolveEdge::new_op(&int_t, 1000, -1);
    // Input edges: slot 0 and slot 1 => encoding 0 and 1.
    let in0 = ResolveEdge::new_op(&int_t, 1000, 0);
    let in1 = ResolveEdge::new_op(&int_t, 1000, 1);

    // Signed: -1 < 0 < 1.  (Unsigned would invert the first pair.)
    assert!(out_edge < in0, "output edge (enc -1) must sort before input slot 0");
    assert!(in0 < in1);
    assert!(out_edge < in1);

    // And typeId still dominates encoding: a smaller typeId with a *larger*
    // encoding still sorts first.
    let other_type = with_id(4, type_metatype::TYPE_INT, 76);
    let big_enc = ResolveEdge::new_op(&other_type, 1000, 5); // typeId 76, enc 5
    assert!(big_enc < out_edge, "typeId dominates encoding");
}

/// The address-form constructor stores `addr.getOffset()` (a `uintb`/u64) into the
/// `uintm`/u32 `opTime` member — a truncating store.  The truncation is
/// *observable*: two offsets that differ only above bit 31 must collapse to the
/// **same** edge (`Eq`), and an offset whose low 32 bits are larger must sort
/// *after* one whose low 32 bits are smaller even if its full u64 is smaller.
/// (`ResolveEdge`'s fields are private, so the truncation is verified through the
/// public `Eq`/`Ord` instead of reading `opTime`.)
#[test]
fn resolve_edge_addr_optime_truncates_u64_to_u32() {
    let int_t = with_id(4, type_metatype::TYPE_INT, 5);
    let space = Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        false,
        8, // 8-byte address space so the offset is not masked on construction
        1,
        3,
        0,
        0,
        0,
    ));
    let low: u64 = 0x9abc_def0;

    // Same low 32 bits, different high 32 bits -> identical edges after truncation.
    let a = ResolveEdge::new_addr(&int_t, &Address::new(Rc::clone(&space), low), 99);
    let b =
        ResolveEdge::new_addr(&int_t, &Address::new(Rc::clone(&space), 0xffff_ffff_0000_0000 | low), 0);
    assert_eq!(a, b, "high-bit-only differences truncate away (opTime is u32)");

    // A full u64 that is numerically *larger* but whose low 32 bits are *smaller*
    // must sort BEFORE: the compare sees only the truncated low word.
    //   big_u64  = 0xFFFF_FFFF_0000_0001 -> opTime 0x0000_0001
    //   small_u64= 0x0000_0000_0000_0002 -> opTime 0x0000_0002
    let big = ResolveEdge::new_addr(&int_t, &Address::new(Rc::clone(&space), 0xffff_ffff_0000_0001), 0);
    let small = ResolveEdge::new_addr(&int_t, &Address::new(Rc::clone(&space), 0x0000_0000_0000_0002), 0);
    assert!(big < small, "ordering uses the truncated low-32 opTime, not the full u64");
}

// =============================================================================
// 2. scoreTruncation — signed int8 size comparison with nonzero curOff
// =============================================================================

/// `unionresolve.cc:953-958`: for a scalar int/uint, while `curOff != 0` or the
/// size mismatches, the loop tests `ct->getSize() >= vn->getSize() + curOff`.
/// All three are folded to **signed int8**.  With a positive offset that pushes
/// `vn->getSize() + curOff` *past* `ct->getSize()`, the `>=` is false and the loop
/// proceeds to `getSubType` (which yields null for a plain int) → final score −10.
/// With a smaller offset that still fits, the branch fires score 1.
#[test]
fn score_truncation_int_branch_signed_offset_window() {
    use type_metatype::*;
    let int8_t = base_type(8, TYPE_INT);

    // vn_size 4, offset 0 already taken by the loop's first check? No: curOff==0
    // but size(8) != vn(4), so we enter the body. int branch: 8 >= 4 + 0 -> score 1.
    let (s0, _) = score_truncation(&int8_t, 4, 0, None).unwrap();
    assert_eq!(s0, 1);

    // offset 4: 8 >= 4 + 4 == 8 -> true -> score 1 (boundary, inclusive >=).
    let (s4, _) = score_truncation(&int8_t, 4, 4, None).unwrap();
    assert_eq!(s4, 1);

    // offset 5: 8 >= 4 + 5 == 9 -> FALSE -> fall through; plain int has no
    // subtype -> ct becomes null -> final score -10.
    let (s5, recurse5) = score_truncation(&int8_t, 4, 5, None).unwrap();
    assert_eq!(s5, -10, "offset past the int width yields the null/-10 path");
    assert!(recurse5.is_none());
}

// =============================================================================
// 3. scoreLockedType — asymmetric pointer chain
// =============================================================================

/// The `while(ct is PTR){ if(lockType not PTR) break; ... descend both }` loop
/// stops the instant *either* operand stops being a pointer, then scores the
/// metatypes of whatever remains.  Re-derive a case where `ct` is a deeper
/// pointer than `lockType`: ct = ptr->ptr->int, lockType = ptr->int.
/// - identity check: not equal -> +0
/// - iter 1: both PTR -> +5, descend -> ct = ptr->int, lockType = int
/// - iter 2: ct is PTR, lockType is INT (not PTR) -> break
/// - remaining metatypes: PTR vs INT -> mismatch, not the int/uint pair -> -5,
///   and sizes differ? ct=ptr(size 8) vs lockType int size 4 -> -2.
///   Total: 5 - 5 - 2 = -2.
#[test]
fn score_locked_type_asymmetric_pointer_chain() {
    let int4_t = base_type(4, type_metatype::TYPE_INT);
    let ct = pointer_to(pointer_to(Rc::clone(&int4_t), 8, 1), 8, 1); // ptr->ptr->int (size 8)
    let lock = pointer_to(Rc::clone(&int4_t), 8, 1); // ptr->int (size 8)
    // iter1 +5; break; remaining ct=ptr(size8) vs lock=int(size4): -5 mismatch,
    // -2 size. Total -2.
    assert_eq!(score_locked_type(&ct, &lock), 5 - 5 - 2);

    // Symmetric reverse: ct shallower than lock. ct = ptr->int, lock = ptr->ptr->int.
    //   iter1 both PTR +5, descend -> ct=int, lock=ptr->int
    //   ct now INT (not PTR) -> while condition false -> exit
    //   remaining INT vs PTR -> -5 mismatch; sizes ct int(4) vs lock ptr(8) -> -2.
    //   Total 5 -5 -2 = -2.
    assert_eq!(score_locked_type(&lock, &ct), 5 - 5 - 2);

    // A bare ptr vs a non-pointer composite: no loop iteration (lockType not PTR
    // on entry), PTR vs STRUCT mismatch -5, sizes 8 vs 4 -> -2 => -7.
    let struct4 = base_type(4, type_metatype::TYPE_STRUCT);
    let p_int = pointer_to(Rc::clone(&int4_t), 8, 1);
    assert_eq!(score_locked_type(&p_int, &struct4), -5 - 2);
}

// =============================================================================
// 4. scoreConstantFit — float-normalized exponent window (exclusive bounds)
// =============================================================================

/// `unionresolve.cc:1001`: `if (exp < 7 && exp > -4) score = 2;` else the float
/// score stays at its initialized −1.  Both bounds are **strict** — exp == 7 and
/// exp == −4 are *out*; exp == 6 and exp == −3 are *in*.
#[test]
fn score_constant_fit_float_exponent_strict_bounds() {
    use type_metatype::*;
    let mk = |exp: int4| ConstantFitFacts {
        float_fact: Some(FloatConstFact::Normalized { exponent: exp }),
        looks_like_pointer: false,
    };
    // Inside (strict): -3..=6 give 2.
    assert_eq!(score_constant_fit(8, 1, TYPE_FLOAT, &mk(6)), 2);
    assert_eq!(score_constant_fit(8, 1, TYPE_FLOAT, &mk(-3)), 2);
    assert_eq!(score_constant_fit(8, 1, TYPE_FLOAT, &mk(0)), 2);
    // On the exclusive boundary: 7 and -4 are OUT -> -1.
    assert_eq!(score_constant_fit(8, 1, TYPE_FLOAT, &mk(7)), -1, "exp == 7 is excluded");
    assert_eq!(score_constant_fit(8, 1, TYPE_FLOAT, &mk(-4)), -1, "exp == -4 is excluded");
    // Far outside -> -1.
    assert_eq!(score_constant_fit(8, 1, TYPE_FLOAT, &mk(100)), -1);
}

// =============================================================================
// 5. computeBestIndex — strict-> tie-break, single element, all-negative
// =============================================================================

/// `unionresolve.cc:1055`: `if (scores[i] > bestScore)` — strict `>`.  The first
/// maximal index wins; later equal maxima do not displace it.  field_num is
/// always `bestIndex - 1`, so index 0 (the whole union) maps to field_num −1.
#[test]
fn compute_best_index_strict_gt_and_edges() {
    // Single element: index 0 always, field_num -1.
    let (f, i) = compute_best_index(&[42]);
    assert_eq!((f, i), (-1, 0));

    // All negative: the largest (least negative) still wins via strict >.
    let (f2, i2) = compute_best_index(&[-10, -3, -10, -3]);
    assert_eq!(i2, 1, "first maximum (-3 at index 1) wins under strict >");
    assert_eq!(f2, 0);

    // Whole union (index 0) ties a later field at the same score: strict > keeps 0.
    let (f3, i3) = compute_best_index(&[5, 5, 5]);
    assert_eq!(i3, 0, "index 0 retained on a tie");
    assert_eq!(f3, -1);

    // A strictly larger later field does win.
    let (f4, i4) = compute_best_index(&[5, 5, 6]);
    assert_eq!(i4, 2);
    assert_eq!(f4, 1);
}

// =============================================================================
// 6. ResolvedUnion::update lock semantics (re-derive the two early returns)
// =============================================================================

/// `unionresolve.cc:88-98`:
/// - locked AND fieldNum differs  -> no change (lock protects the field, not
///   the resolved data-type)
/// - same fieldNum AND same resolve (pointer identity) -> no change
/// - otherwise adopt op's fieldNum+resolve and report a change
///
/// The "same resolve" test is **pointer identity** (`resolve == op.resolve`), so
/// two distinct-but-equal Datatypes with the same fieldNum *do* count as a change.
#[test]
fn resolved_union_update_pointer_identity_semantics() {
    let union_t = make_union(4, &[(0, base_type(4, type_metatype::TYPE_INT))]);
    let int_a = base_type(4, type_metatype::TYPE_INT);
    let int_b = base_type(4, type_metatype::TYPE_INT); // distinct Rc, equal shape

    // Unlocked: same fieldNum (-1), but a *different* resolve object -> change.
    let mut res = ResolvedUnion::new(Rc::clone(&union_t));
    let mut op = ResolvedUnion::new(Rc::clone(&union_t));
    op.set_resolve(Rc::clone(&int_a));
    assert!(res.update(&op), "distinct resolve object with same fieldNum is a change");
    assert!(Rc::ptr_eq(res.get_datatype(), &int_a));

    // Now updating to a *pointer-identical* resolve with the same fieldNum -> no change.
    let mut op_same = ResolvedUnion::new(Rc::clone(&union_t));
    op_same.set_resolve(Rc::clone(&int_a));
    assert!(!res.update(&op_same), "pointer-identical resolve + same fieldNum -> no change");

    // Locked, differing fieldNum -> rejected outright even with a new resolve.
    // Build an op with fieldNum 0 via the public field constructor (a plain
    // TYPE_UNION field 0 resolves through getDepend(0), which is the union's
    // field type — the factory is not consulted on this path).
    res.set_lock(true);
    let factory = TypeFactoryImpl::new();
    let op_field0 = ResolvedUnion::new_field(Rc::clone(&union_t), 0, &factory).unwrap();
    assert_eq!(op_field0.get_field_num(), 0, "constructed op has fieldNum 0");
    // res still has fieldNum -1; locked + differing fieldNum -> no change.
    assert!(!res.update(&op_field0), "locked + differing fieldNum -> rejected");
    let _ = int_b;
}

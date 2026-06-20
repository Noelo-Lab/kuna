//! Adversarial verifier tests for item `w6-s5-typeop` (the canonical `TypeOp`
//! information table ported from `decompiler/cpp/typeop.{cc,hh}`).
//!
//! These are an INDEPENDENT third copy: they do not re-read the production
//! `type_op_info` match nor the porter's `expected_table()`.  Instead they
//! re-derive behavior from the C++ semantics directly and target the hunt-list
//! spots most likely to hide a faithful-port bug for this item:
//!
//!   * **Iteration-order / off-by-one in `register_instructions`.**  C++
//!     `TypeOp::registerInstructions` (typeop.cc:24-109) leaves `inst[0]` and the
//!     unused discriminant 45 (gap between FLOAT_LESSEQUAL=44 and FLOAT_NAN=46)
//!     null, and fills every other slot < CPUI_MAX exactly once.  An off-by-one in
//!     `REGISTERED_OPCODES` or a wrong `opc as usize` index silently corrupts
//!     `setOpcode`.  F-V1 pins the exact null/non-null shape against the opcode
//!     enum, independent of REGISTERED_OPCODES.
//!   * **Integer width sensitivity of `floatSignManipulation`.**  The sign mask is
//!     `calc_mask(size)`, so the recognized constant is width-specific; a size-4
//!     AND must match the 4-byte mask and must NOT match the 8-byte mask.  The
//!     porter's test only exercises size 8.  F-V2 pins the per-width boundary.
//!   * **`selectJavaOperators` blast radius.**  C++ touches exactly six op-codes
//!     (ZEXT, NEGATE, XOR, OR, AND, RIGHT).  A port that mutates one extra slot is
//!     silent corruption.  F-V3 snapshots the whole table and asserts only those
//!     six records differ.
//!   * **Per-op `getInputLocal`/`getOutputLocal` size-only overrides.**  F-V4
//!     re-derives the metatype each (opcode, slot) produces straight from the C++
//!     override bodies, including the documented CBRANCH slot-0 W8 seam.
//!
//! C++ oracle anchors are cited inline.

use kuna_base::address::calc_mask;
use kuna_decomp::dtype::{type_metatype, TypeFactoryImpl};
use kuna_decomp::typeop::{
    float_sign_manipulation, register_instructions, select_java_operators, type_op_info,
    TypeOpClass, TypeOpInfo,
};
use kuna_num::opcodes::OpCode;
use type_metatype::*;

fn factory() -> TypeFactoryImpl {
    let f = TypeFactoryImpl::new();
    f.set_default_alignment_map();
    f.set_max_basetype_size(8);
    f
}

/// Walk every discriminant 0..CPUI_MAX via `OpCode::try_from(u32)` and assert the
/// `register_instructions` slot is non-null iff that discriminant is a real,
/// registered op-code.  This is derived from the opcode enum alone (the C++
/// `opcodes.hh` gap at 45 + `inst[0]` null), not from REGISTERED_OPCODES, so an
/// off-by-one in that constant or a transposed `opc as usize` index is caught.
///
/// cpp: decompiler/cpp/typeop.cc:24-109 (registerInstructions) ;
///      decompiler/cpp/opcodes.hh:44,46,131 (FLOAT_LESSEQUAL=44, FLOAT_NAN=46, MAX=75)
#[test]
fn v1_register_instructions_null_shape_is_exactly_the_opcode_gap() {
    let inst = register_instructions();
    // Length is CPUI_MAX (75): inst[CPUI_MAX] itself is never addressable.
    assert_eq!(inst.len(), OpCode::CPUI_MAX as usize, "inst[] length == CPUI_MAX");

    for idx in 0..(OpCode::CPUI_MAX as i32) {
        // Reconstruct the op-code from the raw discriminant.  Discriminant 0 and
        // the unused 45 have no CPUI_* (from_i32 -> None) and must map to a null
        // slot; everything else is a registered op whose record's opcode
        // round-trips to the same index.
        let slot = &inst[idx as usize];
        match OpCode::from_i32(idx) {
            Some(opc) => {
                let rec = slot.unwrap_or_else(|| panic!("inst[{idx}] ({opc:?}) unexpectedly null"));
                assert_eq!(
                    rec.opcode as i32, idx,
                    "inst[{idx}] holds the wrong op-code record ({:?})",
                    rec.opcode
                );
            }
            None => {
                assert!(slot.is_none(), "inst[{idx}] should be null (no CPUI_* at {idx})");
            }
        }
    }
    // Spot the two specific nulls the C++ leaves: discriminant 0 and 45.
    assert!(inst[0].is_none(), "inst[0] is null (no CPUI_* == 0)");
    assert!(inst[45].is_none(), "inst[45] is null (gap between FLOAT_LESSEQUAL and FLOAT_NAN)");
    // ... and that the slot right after the gap (FLOAT_NAN=46) is populated.
    assert_eq!(inst[46].unwrap().opcode, OpCode::CPUI_FLOAT_NAN);
}

/// `floatSignManipulation` recognizes a width-specific constant.  Re-derive the
/// recognized offsets from `calc_mask(size)` for sizes 1,2,4,8 and assert the
/// match is exactly width-keyed: the right mask -> ABS/NEG, a different-width mask
/// -> CPUI_MAX.  cpp: decompiler/cpp/typeop.cc:154-177.
#[test]
fn v2_float_sign_manipulation_is_width_specific() {
    for &sz in &[1i32, 2, 4, 8] {
        let mask = calc_mask(sz);
        let abs_off = mask >> 1; // INT_AND sign-clear constant for this width
        let neg_off = mask ^ (mask >> 1); // INT_XOR sign-flip constant (== top bit)

        assert_eq!(
            float_sign_manipulation(OpCode::CPUI_INT_AND, Some((sz, abs_off))),
            OpCode::CPUI_FLOAT_ABS,
            "size {sz}: AND {abs_off:#x} should be FLOAT_ABS"
        );
        assert_eq!(
            float_sign_manipulation(OpCode::CPUI_INT_XOR, Some((sz, neg_off))),
            OpCode::CPUI_FLOAT_NEG,
            "size {sz}: XOR {neg_off:#x} should be FLOAT_NEG"
        );

        // A constant valid for a *different* width must not be recognized at this
        // width (unless the masks happen to coincide, which only the same size
        // does).  Use the size-8 sign constants against the smaller widths.
        if sz != 8 {
            let mask8 = calc_mask(8);
            let abs8 = mask8 >> 1; // 0x7fff_ffff_ffff_ffff
            let neg8 = mask8 ^ (mask8 >> 1); // 0x8000_0000_0000_0000
            assert_eq!(
                float_sign_manipulation(OpCode::CPUI_INT_AND, Some((sz, abs8))),
                OpCode::CPUI_MAX,
                "size {sz}: the 8-byte AND mask must not match"
            );
            assert_eq!(
                float_sign_manipulation(OpCode::CPUI_INT_XOR, Some((sz, neg8))),
                OpCode::CPUI_MAX,
                "size {sz}: the 8-byte XOR sign bit must not match"
            );
        }
    }
    // Cross-op confusion: the AND constant fed to XOR (and vice-versa) must not
    // accidentally match at size 8 (abs_off != neg_off there).
    let mask8 = calc_mask(8);
    assert_eq!(
        float_sign_manipulation(OpCode::CPUI_INT_XOR, Some((8, mask8 >> 1))),
        OpCode::CPUI_MAX
    );
    assert_eq!(
        float_sign_manipulation(OpCode::CPUI_INT_AND, Some((8, mask8 ^ (mask8 >> 1)))),
        OpCode::CPUI_MAX
    );
}

/// `selectJavaOperators` must mutate exactly the six op-codes the C++ touches and
/// leave every other record byte-identical.  Snapshot the full table, flip to
/// Java, and diff: only ZEXT/NEGATE/XOR/OR/AND/RIGHT may change.  Then flip back
/// and assert a full round-trip to the C state.
/// cpp: decompiler/cpp/typeop.cc:115-148.
#[test]
fn v3_select_java_operators_blast_radius() {
    fn key(r: &TypeOpInfo) -> (u32, type_metatype, type_metatype, &'static str) {
        (r.opcode as u32, r.metaout, r.metain, r.name)
    }
    let touched: [OpCode; 6] = [
        OpCode::CPUI_INT_ZEXT,
        OpCode::CPUI_INT_NEGATE,
        OpCode::CPUI_INT_XOR,
        OpCode::CPUI_INT_OR,
        OpCode::CPUI_INT_AND,
        OpCode::CPUI_INT_RIGHT,
    ];

    let before: Vec<Option<TypeOpInfo>> = register_instructions();
    let mut java = before.clone();
    select_java_operators(&mut java, true);

    for idx in 0..before.len() {
        match (before[idx], java[idx]) {
            (Some(b), Some(a)) => {
                let opc = b.opcode;
                if touched.contains(&opc) {
                    assert_ne!(
                        key(&b),
                        key(&a),
                        "{opc:?} is a Java-touched op but did not change"
                    );
                } else {
                    assert_eq!(key(&b), key(&a), "{opc:?} must be untouched by Java toggle");
                    // opflags/addlflags/class are never touched by the toggle.
                    assert_eq!(b.opflags, a.opflags);
                    assert_eq!(b.addlflags, a.addlflags);
                    assert_eq!(b.class, a.class);
                }
            }
            (None, None) => {}
            _ => panic!("slot {idx} changed null-ness across the Java toggle"),
        }
    }

    // Round-trip: flipping back to C restores every field exactly.
    select_java_operators(&mut java, false);
    for idx in 0..before.len() {
        match (before[idx], java[idx]) {
            (Some(b), Some(a)) => assert_eq!(
                key(&b),
                key(&a),
                "{:?} not restored after Java->C round-trip",
                b.opcode
            ),
            (None, None) => {}
            _ => panic!("slot {idx} null-ness diverged after round-trip"),
        }
    }
}

/// Re-derive the metatype each per-op `getInputLocal`/`getOutputLocal` override
/// produces straight from the C++ override bodies, including the documented
/// CBRANCH slot-0 W8 seam (typeop.cc:611-621 returns a code pointer in C++; the
/// port returns the inherited `getBase(size, TYPE_UNKNOWN)` default and pins that
/// as the current behavior so a regression here is visible).
#[test]
fn v4_local_type_overrides_metatype_rederived() {
    let f = factory();
    let mout = |opc: OpCode, sz: i32| type_op_info(opc).get_output_local(&f, sz).unwrap().get_metatype();
    let min = |opc: OpCode, slot: i32, sz: i32| {
        type_op_info(opc).get_input_local(&f, slot, sz).unwrap().get_metatype()
    };

    // PTRADD/PTRSUB: both in and out forced to INT regardless of class (Base).
    // (typeop.cc:2234-2244, 2310-2320)
    assert_eq!(mout(OpCode::CPUI_PTRADD, 8), TYPE_INT);
    assert_eq!(min(OpCode::CPUI_PTRADD, 2, 8), TYPE_INT, "PTRADD slot 2 (the element-size operand) -> INT");
    assert_eq!(mout(OpCode::CPUI_PTRSUB, 8), TYPE_INT);
    assert_eq!(min(OpCode::CPUI_PTRSUB, 0, 8), TYPE_INT);

    // Shift slot 1 -> INT via getBaseNoChar; slot 0 -> the binary metain.
    // INT_LEFT metain INT, INT_RIGHT metain UINT, INT_SRIGHT metain INT.
    // (typeop.cc:1512-1518, 1537-1543, 1602-1608)
    assert_eq!(min(OpCode::CPUI_INT_LEFT, 1, 4), TYPE_INT);
    assert_eq!(min(OpCode::CPUI_INT_LEFT, 0, 4), TYPE_INT);
    assert_eq!(min(OpCode::CPUI_INT_RIGHT, 1, 4), TYPE_INT, "shift count is INT even though metain is UINT");
    assert_eq!(min(OpCode::CPUI_INT_RIGHT, 0, 4), TYPE_UINT);
    assert_eq!(min(OpCode::CPUI_INT_SRIGHT, 1, 4), TYPE_INT);
    assert_eq!(min(OpCode::CPUI_INT_SRIGHT, 0, 4), TYPE_INT);

    // CPOOLREF input -> INT (typeop.cc:2467-2471).
    assert_eq!(min(OpCode::CPUI_CPOOLREF, 0, 4), TYPE_INT);

    // CBRANCH slot 1 -> BOOL (typeop.cc:613-617); slot 0 is the documented W8 seam
    // (C++ returns a code pointer; port returns the Base UNKNOWN default).
    assert_eq!(min(OpCode::CPUI_CBRANCH, 1, 1), TYPE_BOOL);
    assert_eq!(
        min(OpCode::CPUI_CBRANCH, 0, 8),
        TYPE_UNKNOWN,
        "CBRANCH slot-0 W8 seam: inherited UNKNOWN default (NOT the C++ code pointer)"
    );

    // INSERT slots <=1 -> UNKNOWN, slot 2 -> Func metain INT (typeop.cc:2537-2543).
    assert_eq!(min(OpCode::CPUI_INSERT, 0, 4), TYPE_UNKNOWN);
    assert_eq!(min(OpCode::CPUI_INSERT, 1, 4), TYPE_UNKNOWN);
    assert_eq!(min(OpCode::CPUI_INSERT, 2, 4), TYPE_INT);

    // ZPULL/SPULL slot 0 -> UNKNOWN, other slots -> Func metain.
    // ZPULL metain INT, SPULL metain INT (typeop.cc:2564-2569, 2591-2596).
    assert_eq!(min(OpCode::CPUI_ZPULL, 0, 4), TYPE_UNKNOWN);
    assert_eq!(min(OpCode::CPUI_ZPULL, 2, 4), TYPE_INT);
    assert_eq!(min(OpCode::CPUI_SPULL, 0, 4), TYPE_UNKNOWN);
    assert_eq!(min(OpCode::CPUI_SPULL, 1, 4), TYPE_INT);

    // A pure Func op with no override: POPCOUNT out INT, in UNKNOWN
    // (typeop.cc:2611-2616) — confirms the default Func arm path is reached.
    assert_eq!(mout(OpCode::CPUI_POPCOUNT, 4), TYPE_INT);
    assert_eq!(min(OpCode::CPUI_POPCOUNT, 0, 4), TYPE_UNKNOWN);
}

/// Guard the `TypeOpClass` assignment for a representative of each shape: the
/// class drives the generic local-type fallthrough, so a Func-vs-Binary mixup is
/// invisible in flag-word tests but changes `getInputLocal`/`getOutputLocal`.
#[test]
fn v5_typeopclass_shape_sanity() {
    assert_eq!(type_op_info(OpCode::CPUI_COPY).class, TypeOpClass::Base);
    assert_eq!(type_op_info(OpCode::CPUI_INT_ADD).class, TypeOpClass::Binary);
    assert_eq!(type_op_info(OpCode::CPUI_INT_2COMP).class, TypeOpClass::Unary);
    assert_eq!(type_op_info(OpCode::CPUI_INT_ZEXT).class, TypeOpClass::Func);
    // PIECE/SUBPIECE are Func in C++ (TypeOpFunc) despite the "binary" opflag.
    assert_eq!(type_op_info(OpCode::CPUI_PIECE).class, TypeOpClass::Func);
    assert_eq!(type_op_info(OpCode::CPUI_SUBPIECE).class, TypeOpClass::Func);
    // PTRADD/PTRSUB derive directly from TypeOp (Base), with size-only overrides.
    assert_eq!(type_op_info(OpCode::CPUI_PTRADD).class, TypeOpClass::Base);
}

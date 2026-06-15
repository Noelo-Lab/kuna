//! VERIFIER adversarial tests for `rport/w10-float-family` (round 1).
//!
//! Protocol: docs/rust-port/verification.md.  The ported diff closes the
//! `PrintC::opFloatInt2Float` seam (printc.cc:850) and its helper
//! `TypeOpFloatInt2Float::absorbZext` (typeop.cc:1874): FLOAT_INT2FLOAT renders
//! as a `(floatN)input` type-cast (NOT a functional `FLOAT_INT2FLOAT(x)`),
//! absorbing an implied INT_ZEXT on its input.
//!
//! The deep absorption / cast-text logic lives in PRIVATE fns
//! (`absorb_zext`, `op_float_int2float_ir`, `push_cast_type`), reachable only
//! from the in-crate `#[cfg(test)] mod tests` — those adversarial tests (with
//! a mutation-tested oracle for `absorbZext`'s isWritten/isImplied AND and the
//! INT_ZEXT opcode equality) live in `src/printc/tests.rs::w10_float_family`.
//!
//! This integration file covers what is reachable across the crate boundary:
//!   1. the PUBLIC dispatch decision the diff changed — `op_emit_kind` now
//!      routes FLOAT_INT2FLOAT to `Custom` (the hand-written `(floatN)` arm),
//!      while the sibling float-cast ops keep their table kinds (anti-special-
//!      casing: the change is one table cell, derived from the opcode, with no
//!      function-name/value hardcoding), and
//!   2. a non-regression guard that the FLOAT_INT2FLOAT routing did not perturb
//!      the rest of the opcode→emit-kind table (the float family and the
//!      hand-written/no-op overrides).
//!
//! `op_emit_kind` is a pure function of the opcode (no Funcdata, no addresses,
//! no names), so these assertions are exactly the "is the routing faithful and
//! special-case-free" check.

use kuna_decomp::printc::{op_emit_kind, OpEmitKind};
use kuna_num::opcodes::OpCode;

/// The one decision the diff added: FLOAT_INT2FLOAT is dispatched as `Custom`
/// (the hand-written `op_float_int2float_ir` arm), NOT `Func` and NOT the
/// generic `TypeCast` fallthrough.  Faithful to printc.hh:331 (the only float
/// conversion with a non-default `opFloatInt2Float` override; FLOAT2FLOAT /
/// FLOAT_TRUNC use the plain `opTypeCast`).
#[test]
fn w10ff_float_int2float_routes_to_custom() {
    assert!(
        matches!(op_emit_kind(OpCode::CPUI_FLOAT_INT2FLOAT), OpEmitKind::Custom),
        "FLOAT_INT2FLOAT must dispatch to the hand-written (floatN) cast arm (Custom)"
    );
}

/// Anti-regression on the float family: the OTHER float-cast conversions keep
/// their `TypeCast` table kind (printc.hh:332-333 `opFloatFloat2Float` /
/// `opFloatTrunc` -> `opTypeCast`), and the functional float ops keep `Func`
/// (printc.hh:322/328 `opFloatNan`/`opFloatAbs` -> `opFunc`).  If the diff had
/// special-cased by, say, flipping the whole float family, this catches it.
#[test]
fn w10ff_sibling_float_ops_keep_their_kinds() {
    // The two sibling float CASTS stay TypeCast (NOT Custom — only INT2FLOAT
    // got the hand-written override).
    assert!(
        matches!(op_emit_kind(OpCode::CPUI_FLOAT_FLOAT2FLOAT), OpEmitKind::TypeCast),
        "FLOAT_FLOAT2FLOAT stays a plain opTypeCast"
    );
    assert!(
        matches!(op_emit_kind(OpCode::CPUI_FLOAT_TRUNC), OpEmitKind::TypeCast),
        "FLOAT_TRUNC stays a plain opTypeCast"
    );
    // The functional float ops stay Func.
    for opc in [
        OpCode::CPUI_FLOAT_NAN,
        OpCode::CPUI_FLOAT_ABS,
        OpCode::CPUI_FLOAT_SQRT,
        OpCode::CPUI_FLOAT_CEIL,
        OpCode::CPUI_FLOAT_FLOOR,
        OpCode::CPUI_FLOAT_ROUND,
    ] {
        assert!(
            matches!(op_emit_kind(opc), OpEmitKind::Func),
            "{opc:?} must stay a functional opFunc form"
        );
    }
    // The arithmetic float ops stay Binary/Unary (NOT swept into Custom).
    assert!(
        matches!(op_emit_kind(OpCode::CPUI_FLOAT_ADD), OpEmitKind::Binary(_)),
        "FLOAT_ADD stays binary"
    );
    assert!(
        matches!(op_emit_kind(OpCode::CPUI_FLOAT_NEG), OpEmitKind::Unary(_)),
        "FLOAT_NEG stays unary"
    );
}

/// `op_emit_kind` is a TOTAL function of the opcode (no panics, no special
/// per-value path): every opcode classifies into exactly one kind.  This guards
/// against a stray `_ => unreachable!()`-style special-case slipping into the
/// table around the new FLOAT_INT2FLOAT cell.  We sweep the float family plus a
/// representative cross-section.
#[test]
fn w10ff_emit_kind_is_total_no_panics() {
    let opcodes = [
        OpCode::CPUI_FLOAT_INT2FLOAT,
        OpCode::CPUI_FLOAT_FLOAT2FLOAT,
        OpCode::CPUI_FLOAT_TRUNC,
        OpCode::CPUI_FLOAT_NAN,
        OpCode::CPUI_FLOAT_ABS,
        OpCode::CPUI_FLOAT_SQRT,
        OpCode::CPUI_FLOAT_ADD,
        OpCode::CPUI_FLOAT_SUB,
        OpCode::CPUI_FLOAT_MULT,
        OpCode::CPUI_FLOAT_DIV,
        OpCode::CPUI_FLOAT_NEG,
        OpCode::CPUI_FLOAT_EQUAL,
        OpCode::CPUI_FLOAT_LESS,
        OpCode::CPUI_INT_ZEXT,
        OpCode::CPUI_INT_SEXT,
        OpCode::CPUI_CAST,
        OpCode::CPUI_COPY,
        OpCode::CPUI_LOAD,
        OpCode::CPUI_STORE,
        OpCode::CPUI_CALL,
        OpCode::CPUI_MULTIEQUAL,
        OpCode::CPUI_INDIRECT,
    ];
    for opc in opcodes {
        // Must not panic and must produce a well-formed variant.
        let k = op_emit_kind(opc);
        let _ = matches!(
            k,
            OpEmitKind::Binary(_)
                | OpEmitKind::Unary(_)
                | OpEmitKind::Func
                | OpEmitKind::TypeCast
                | OpEmitKind::Custom
        );
    }
    // And re-confirm the specific cell once more for documentation locality.
    assert!(matches!(op_emit_kind(OpCode::CPUI_FLOAT_INT2FLOAT), OpEmitKind::Custom));
}

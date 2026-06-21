//! Tests for the canonical `TypeOp` information table (`typeop.rs`).
//!
//! The flag-word tests are **exhaustive**: every op-code's `(opflags, addlflags,
//! name, metaout, metain)` is checked against the literal from its `typeop.cc`
//! constructor.  A transposition or a dropped bit (the output-determining failure
//! mode for `setOpcode`/the rule retypers) is caught here.  The local-type tests
//! drive the `getOutputLocal`/`getInputLocal` matrices for the generic
//! binary/unary/func classes and the size-only per-op overrides.

use super::*;
use crate::dtype::{type_metatype, TypeFactoryImpl};
use type_metatype::*;

/// One expected row of the canonical table, transcribed from `typeop.cc`.
struct Row {
    opc: OpCode,
    opflags: uint4,
    addlflags: uint4,
    name: &'static str,
    class: TypeOpClass,
    metaout: type_metatype,
    metain: type_metatype,
}

/// The full expected table — every op-code `registerInstructions` registers,
/// transcribed independently from the `typeop.cc` constructors (so the test is a
/// second copy, not a re-read of the production match).
fn expected_table() -> Vec<Row> {
    use OpCode::*;
    use TypeOpClass::*;
    // Helper: a Base-class row uses TYPE_UNKNOWN metatypes (ignored for Base).
    macro_rules! b {
        ($opc:expr, $of:expr, $af:expr, $nm:expr) => {
            Row {
                opc: $opc,
                opflags: $of,
                addlflags: $af,
                name: $nm,
                class: Base,
                metaout: TYPE_UNKNOWN,
                metain: TYPE_UNKNOWN,
            }
        };
    }
    macro_rules! t {
        ($opc:expr, $of:expr, $af:expr, $nm:expr, $cl:expr, $mo:expr, $mi:expr) => {
            Row {
                opc: $opc,
                opflags: $of,
                addlflags: $af,
                name: $nm,
                class: $cl,
                metaout: $mo,
                metain: $mi,
            }
        };
    }
    use typeop_addlflags as af;
    vec![
        b!(CPUI_COPY, pf::unary | pf::nocollapse, 0, "copy"),
        b!(CPUI_LOAD, pf::special | pf::nocollapse, 0, "load"),
        b!(CPUI_STORE, pf::special | pf::nocollapse, 0, "store"),
        b!(CPUI_BRANCH, pf::special | pf::branch | pf::coderef | pf::nocollapse, 0, "goto"),
        b!(CPUI_CBRANCH, pf::special | pf::branch | pf::coderef | pf::nocollapse, 0, "goto"),
        b!(CPUI_BRANCHIND, pf::special | pf::branch | pf::nocollapse, 0, "switch"),
        b!(
            CPUI_CALL,
            pf::special | pf::call | pf::has_callspec | pf::coderef | pf::nocollapse,
            0,
            "call"
        ),
        b!(
            CPUI_CALLIND,
            pf::special | pf::call | pf::has_callspec | pf::nocollapse,
            0,
            "callind"
        ),
        b!(CPUI_CALLOTHER, pf::special | pf::call | pf::nocollapse, 0, "syscall"),
        b!(
            CPUI_RETURN,
            pf::special | pf::returns | pf::nocollapse | pf::return_copy,
            0,
            "return"
        ),
        b!(CPUI_MULTIEQUAL, pf::special | pf::marker | pf::nocollapse, 0, "?"),
        b!(CPUI_INDIRECT, pf::special | pf::marker | pf::nocollapse, 0, "[]"),
        t!(CPUI_PIECE, pf::binary, 0, "CONCAT", Func, TYPE_UNKNOWN, TYPE_UNKNOWN),
        t!(CPUI_SUBPIECE, pf::binary, 0, "SUB", Func, TYPE_UNKNOWN, TYPE_UNKNOWN),
        t!(
            CPUI_INT_EQUAL,
            pf::binary | pf::booloutput | pf::commutative,
            af::inherits_sign,
            "==",
            Binary,
            TYPE_BOOL,
            TYPE_INT
        ),
        t!(
            CPUI_INT_NOTEQUAL,
            pf::binary | pf::booloutput | pf::commutative,
            af::inherits_sign,
            "!=",
            Binary,
            TYPE_BOOL,
            TYPE_INT
        ),
        t!(
            CPUI_INT_SLESS,
            pf::binary | pf::booloutput,
            af::inherits_sign,
            "<",
            Binary,
            TYPE_BOOL,
            TYPE_INT
        ),
        t!(
            CPUI_INT_SLESSEQUAL,
            pf::binary | pf::booloutput,
            af::inherits_sign,
            "<=",
            Binary,
            TYPE_BOOL,
            TYPE_INT
        ),
        t!(
            CPUI_INT_LESS,
            pf::binary | pf::booloutput,
            af::inherits_sign,
            "<",
            Binary,
            TYPE_BOOL,
            TYPE_UINT
        ),
        t!(
            CPUI_INT_LESSEQUAL,
            pf::binary | pf::booloutput,
            af::inherits_sign,
            "<=",
            Binary,
            TYPE_BOOL,
            TYPE_UINT
        ),
        t!(CPUI_INT_ZEXT, pf::unary, 0, "ZEXT", Func, TYPE_UINT, TYPE_UINT),
        t!(CPUI_INT_SEXT, pf::unary, 0, "SEXT", Func, TYPE_INT, TYPE_INT),
        t!(
            CPUI_INT_ADD,
            pf::binary | pf::commutative,
            af::arithmetic_op | af::inherits_sign,
            "+",
            Binary,
            TYPE_INT,
            TYPE_INT
        ),
        t!(
            CPUI_INT_SUB,
            pf::binary,
            af::arithmetic_op | af::inherits_sign,
            "-",
            Binary,
            TYPE_INT,
            TYPE_INT
        ),
        t!(
            CPUI_INT_CARRY,
            pf::binary | pf::commutative | pf::booloutput,
            af::arithmetic_op,
            "CARRY",
            Func,
            TYPE_BOOL,
            TYPE_UINT
        ),
        t!(
            CPUI_INT_SCARRY,
            pf::binary | pf::commutative | pf::booloutput,
            af::arithmetic_op,
            "SCARRY",
            Func,
            TYPE_BOOL,
            TYPE_INT
        ),
        t!(
            CPUI_INT_SBORROW,
            pf::binary | pf::booloutput,
            af::arithmetic_op,
            "SBORROW",
            Func,
            TYPE_BOOL,
            TYPE_INT
        ),
        t!(
            CPUI_INT_2COMP,
            pf::unary,
            af::arithmetic_op | af::inherits_sign,
            "-",
            Unary,
            TYPE_INT,
            TYPE_INT
        ),
        t!(
            CPUI_INT_NEGATE,
            pf::unary,
            af::logical_op | af::inherits_sign,
            "~",
            Unary,
            TYPE_UINT,
            TYPE_UINT
        ),
        t!(
            CPUI_INT_XOR,
            pf::binary | pf::commutative,
            af::logical_op | af::inherits_sign,
            "^",
            Binary,
            TYPE_UINT,
            TYPE_UINT
        ),
        t!(
            CPUI_INT_AND,
            pf::binary | pf::commutative,
            af::logical_op | af::inherits_sign,
            "&",
            Binary,
            TYPE_UINT,
            TYPE_UINT
        ),
        t!(
            CPUI_INT_OR,
            pf::binary | pf::commutative,
            af::logical_op | af::inherits_sign,
            "|",
            Binary,
            TYPE_UINT,
            TYPE_UINT
        ),
        t!(
            CPUI_INT_LEFT,
            pf::binary,
            af::inherits_sign | af::inherits_sign_zero | af::shift_op,
            "<<",
            Binary,
            TYPE_INT,
            TYPE_INT
        ),
        t!(
            CPUI_INT_RIGHT,
            pf::binary,
            af::inherits_sign | af::inherits_sign_zero | af::shift_op,
            ">>",
            Binary,
            TYPE_UINT,
            TYPE_UINT
        ),
        t!(
            CPUI_INT_SRIGHT,
            pf::binary,
            af::inherits_sign | af::inherits_sign_zero | af::shift_op,
            ">>",
            Binary,
            TYPE_INT,
            TYPE_INT
        ),
        t!(
            CPUI_INT_MULT,
            pf::binary | pf::commutative,
            af::arithmetic_op | af::inherits_sign,
            "*",
            Binary,
            TYPE_INT,
            TYPE_INT
        ),
        t!(
            CPUI_INT_DIV,
            pf::binary,
            af::arithmetic_op | af::inherits_sign,
            "/",
            Binary,
            TYPE_UINT,
            TYPE_UINT
        ),
        t!(
            CPUI_INT_SDIV,
            pf::binary,
            af::arithmetic_op | af::inherits_sign,
            "/",
            Binary,
            TYPE_INT,
            TYPE_INT
        ),
        t!(
            CPUI_INT_REM,
            pf::binary,
            af::arithmetic_op | af::inherits_sign | af::inherits_sign_zero,
            "%",
            Binary,
            TYPE_UINT,
            TYPE_UINT
        ),
        t!(
            CPUI_INT_SREM,
            pf::binary,
            af::arithmetic_op | af::inherits_sign | af::inherits_sign_zero,
            "%",
            Binary,
            TYPE_INT,
            TYPE_INT
        ),
        t!(
            CPUI_BOOL_NEGATE,
            pf::unary | pf::booloutput,
            af::logical_op,
            "!",
            Unary,
            TYPE_BOOL,
            TYPE_BOOL
        ),
        t!(
            CPUI_BOOL_XOR,
            pf::binary | pf::commutative | pf::booloutput,
            af::logical_op,
            "^^",
            Binary,
            TYPE_BOOL,
            TYPE_BOOL
        ),
        t!(
            CPUI_BOOL_AND,
            pf::binary | pf::commutative | pf::booloutput,
            af::logical_op,
            "&&",
            Binary,
            TYPE_BOOL,
            TYPE_BOOL
        ),
        t!(
            CPUI_BOOL_OR,
            pf::binary | pf::commutative | pf::booloutput,
            af::logical_op,
            "||",
            Binary,
            TYPE_BOOL,
            TYPE_BOOL
        ),
        b!(CPUI_CAST, pf::unary | pf::special | pf::nocollapse, 0, "(cast)"),
        b!(CPUI_PTRADD, pf::ternary | pf::nocollapse, af::arithmetic_op, "+"),
        b!(CPUI_PTRSUB, pf::binary | pf::nocollapse, af::arithmetic_op, "->"),
        t!(
            CPUI_FLOAT_EQUAL,
            pf::binary | pf::booloutput | pf::commutative,
            af::floatingpoint_op,
            "==",
            Binary,
            TYPE_BOOL,
            TYPE_FLOAT
        ),
        t!(
            CPUI_FLOAT_NOTEQUAL,
            pf::binary | pf::booloutput | pf::commutative,
            af::floatingpoint_op,
            "!=",
            Binary,
            TYPE_BOOL,
            TYPE_FLOAT
        ),
        t!(
            CPUI_FLOAT_LESS,
            pf::binary | pf::booloutput,
            af::floatingpoint_op,
            "<",
            Binary,
            TYPE_BOOL,
            TYPE_FLOAT
        ),
        t!(
            CPUI_FLOAT_LESSEQUAL,
            pf::binary | pf::booloutput,
            af::floatingpoint_op,
            "<=",
            Binary,
            TYPE_BOOL,
            TYPE_FLOAT
        ),
        t!(
            CPUI_FLOAT_NAN,
            pf::unary | pf::booloutput,
            af::floatingpoint_op,
            "NAN",
            Func,
            TYPE_BOOL,
            TYPE_FLOAT
        ),
        t!(
            CPUI_FLOAT_ADD,
            pf::binary | pf::commutative,
            af::floatingpoint_op,
            "+",
            Binary,
            TYPE_FLOAT,
            TYPE_FLOAT
        ),
        t!(
            CPUI_FLOAT_DIV,
            pf::binary,
            af::floatingpoint_op,
            "/",
            Binary,
            TYPE_FLOAT,
            TYPE_FLOAT
        ),
        t!(
            CPUI_FLOAT_MULT,
            pf::binary | pf::commutative,
            af::floatingpoint_op,
            "*",
            Binary,
            TYPE_FLOAT,
            TYPE_FLOAT
        ),
        t!(
            CPUI_FLOAT_SUB,
            pf::binary,
            af::floatingpoint_op,
            "-",
            Binary,
            TYPE_FLOAT,
            TYPE_FLOAT
        ),
        t!(CPUI_FLOAT_NEG, pf::unary, af::floatingpoint_op, "-", Unary, TYPE_FLOAT, TYPE_FLOAT),
        t!(CPUI_FLOAT_ABS, pf::unary, af::floatingpoint_op, "ABS", Func, TYPE_FLOAT, TYPE_FLOAT),
        t!(CPUI_FLOAT_SQRT, pf::unary, af::floatingpoint_op, "SQRT", Func, TYPE_FLOAT, TYPE_FLOAT),
        t!(
            CPUI_FLOAT_INT2FLOAT,
            pf::unary,
            af::floatingpoint_op,
            "INT2FLOAT",
            Func,
            TYPE_FLOAT,
            TYPE_INT
        ),
        t!(
            CPUI_FLOAT_FLOAT2FLOAT,
            pf::unary,
            af::floatingpoint_op,
            "FLOAT2FLOAT",
            Func,
            TYPE_FLOAT,
            TYPE_FLOAT
        ),
        t!(CPUI_FLOAT_TRUNC, pf::unary, af::floatingpoint_op, "TRUNC", Func, TYPE_INT, TYPE_FLOAT),
        t!(CPUI_FLOAT_CEIL, pf::unary, af::floatingpoint_op, "CEIL", Func, TYPE_FLOAT, TYPE_FLOAT),
        t!(
            CPUI_FLOAT_FLOOR,
            pf::unary,
            af::floatingpoint_op,
            "FLOOR",
            Func,
            TYPE_FLOAT,
            TYPE_FLOAT
        ),
        t!(
            CPUI_FLOAT_ROUND,
            pf::unary,
            af::floatingpoint_op,
            "ROUND",
            Func,
            TYPE_FLOAT,
            TYPE_FLOAT
        ),
        b!(CPUI_SEGMENTOP, pf::special | pf::nocollapse, 0, "segmentop"),
        b!(CPUI_CPOOLREF, pf::special | pf::nocollapse, 0, "cpoolref"),
        b!(CPUI_NEW, pf::special | pf::call | pf::nocollapse, 0, "new"),
        t!(CPUI_INSERT, pf::ternary, 0, "INSERT", Func, TYPE_UNKNOWN, TYPE_INT),
        t!(CPUI_ZPULL, pf::ternary, 0, "ZPULL", Func, TYPE_UINT, TYPE_INT),
        t!(CPUI_POPCOUNT, pf::unary, 0, "POPCOUNT", Func, TYPE_INT, TYPE_UNKNOWN),
        t!(CPUI_LZCOUNT, pf::unary, 0, "LZCOUNT", Func, TYPE_INT, TYPE_UNKNOWN),
        t!(CPUI_SPULL, pf::ternary, 0, "SPULL", Func, TYPE_INT, TYPE_INT),
    ]
}

/// **Exhaustive flag-word check.** Every registered op-code's record matches the
/// independently-transcribed `typeop.cc` literal: opflags, addlflags, name, class,
/// and (for typed classes) metaout/metain.
#[test]
fn exhaustive_per_opcode_flag_words() {
    for row in expected_table() {
        let info = type_op_info(row.opc);
        assert_eq!(info.opcode, row.opc, "{:?}: opcode field", row.opc);
        assert_eq!(info.opflags, row.opflags, "{:?}: opflags", row.opc);
        assert_eq!(info.addlflags, row.addlflags, "{:?}: addlflags", row.opc);
        assert_eq!(info.name, row.name, "{:?}: name", row.opc);
        assert_eq!(info.class, row.class, "{:?}: class", row.opc);
        if !matches!(row.class, TypeOpClass::Base) {
            assert_eq!(info.metaout, row.metaout, "{:?}: metaout", row.opc);
            assert_eq!(info.metain, row.metain, "{:?}: metain", row.opc);
        }
    }
}

/// The `inst[]` vector has length `CPUI_MAX`, the unregistered slots are `None`
/// (index 0 and the unused discriminant 45), and every registered op-code's slot
/// holds its `type_op_info` record at exactly `opcode as usize`.
#[test]
fn register_instructions_vector_shape() {
    let inst = register_instructions();
    assert_eq!(inst.len(), OpCode::CPUI_MAX as usize, "inst[] length is CPUI_MAX");
    // index 0 (no CPUI_* == 0) is null.
    assert!(inst[0].is_none(), "inst[0] is null (no opcode 0)");
    // The unused discriminant 45 (between FLOAT_LESSEQUAL=44 and FLOAT_NAN=46).
    assert!(inst[45].is_none(), "inst[45] is null (unused discriminant)");
    // Every registered op-code is present at its own index.
    for &opc in REGISTERED_OPCODES {
        let slot = inst[opc as usize].expect("registered op-code present");
        assert_eq!(slot.opcode, opc, "inst[{}] holds {:?}", opc as usize, opc);
    }
    // Exactly REGISTERED_OPCODES.len() slots are populated.
    let populated = inst.iter().filter(|s| s.is_some()).count();
    assert_eq!(populated, REGISTERED_OPCODES.len(), "no extra/missing slots");
}

/// `to_seam`/`type_op_for` carry the `(opcode, flags, name)` triple the W3 seam
/// `setOpcode` consumes — identical to what the per-wave local helpers built.
#[test]
fn seam_triple_matches_table() {
    for &opc in REGISTERED_OPCODES {
        let info = type_op_info(opc);
        let seam = type_op_for(opc);
        assert_eq!(seam.get_opcode(), info.opcode);
        assert_eq!(seam.get_flags(), info.opflags);
        assert_eq!(seam.get_name(), info.name);
    }
    // Spot-check the literal the old `ruleaction_5::type_op_seam` produced for
    // INT_EQUAL: binary|booloutput|commutative, "==".
    let eq = type_op_for(OpCode::CPUI_INT_EQUAL);
    assert_eq!(eq.get_flags(), pf::binary | pf::booloutput | pf::commutative);
    assert_eq!(eq.get_name(), "==");
}

/// **addlflags predicate matrix.** Each predicate reads exactly its bit; spot the
/// representative op-codes the C++ doc names.
#[test]
fn addlflag_predicates() {
    // isCommutative reads opflags (PcodeOp::commutative), not addlflags.
    assert!(type_op_info(OpCode::CPUI_INT_ADD).is_commutative());
    assert!(!type_op_info(OpCode::CPUI_INT_SUB).is_commutative());
    // inheritsSign / inheritsSignFirstParamOnly.
    assert!(type_op_info(OpCode::CPUI_INT_ADD).inherits_sign());
    assert!(!type_op_info(OpCode::CPUI_INT_ADD).inherits_sign_first_param_only());
    assert!(type_op_info(OpCode::CPUI_INT_LEFT).inherits_sign_first_param_only());
    // isShiftOp: only INT_LEFT/INT_RIGHT/INT_SRIGHT.
    assert!(type_op_info(OpCode::CPUI_INT_LEFT).is_shift_op());
    assert!(type_op_info(OpCode::CPUI_INT_RIGHT).is_shift_op());
    assert!(type_op_info(OpCode::CPUI_INT_SRIGHT).is_shift_op());
    assert!(!type_op_info(OpCode::CPUI_INT_ADD).is_shift_op());
    // isArithmeticOp / isLogicalOp.
    assert!(type_op_info(OpCode::CPUI_INT_ADD).is_arithmetic_op());
    assert!(type_op_info(OpCode::CPUI_INT_MULT).is_arithmetic_op());
    assert!(!type_op_info(OpCode::CPUI_INT_AND).is_arithmetic_op());
    assert!(type_op_info(OpCode::CPUI_INT_AND).is_logical_op());
    assert!(type_op_info(OpCode::CPUI_INT_XOR).is_logical_op());
    assert!(!type_op_info(OpCode::CPUI_INT_ADD).is_logical_op());
    // isFloatingPointOp.
    assert!(type_op_info(OpCode::CPUI_FLOAT_ADD).is_floating_point_op());
    assert!(type_op_info(OpCode::CPUI_FLOAT_NAN).is_floating_point_op());
    assert!(!type_op_info(OpCode::CPUI_INT_ADD).is_floating_point_op());
}

/// VERIFIER (w10-doublemove): pin the exact `isArithmeticOp() ||
/// isFloatingPointOp()` disjunction that `double.rs::op_is_arith_or_float` now
/// reads from this table (formerly a hard-coded `false`).  C++
/// `RuleDoubleIn/Out::attemptMarking` (double.cc:3239,3320) uses this disjunction
/// to decide whether a whole "produces/reads a logical whole".  The whole must
/// classify TRUE for arithmetic (INT_ADD: addlflags=arithmetic_op|inherits_sign,
/// typeop.cc:1172) and float (FLOAT_ADD: addlflags=floatingpoint_op,
/// typeop.cc:1788) opcodes, and FALSE for the piece-forming opcodes the rule
/// must never mistake for a whole (COPY/PIECE/SUBPIECE carry no addlflags —
/// typeop.cc:391,2039,2068).  A regression here silently re-disables (false) or
/// over-fires (true) the dominant marking branch.
#[test]
fn w10_doublemove_arith_or_float_classification() {
    let arith_or_float = |opc: OpCode| {
        let i = type_op_info(opc);
        i.is_arithmetic_op() || i.is_floating_point_op()
    };
    // Wholes that DO produce/read a logical whole.
    assert!(arith_or_float(OpCode::CPUI_INT_ADD), "INT_ADD is arithmetic");
    assert!(arith_or_float(OpCode::CPUI_INT_MULT), "INT_MULT is arithmetic");
    assert!(arith_or_float(OpCode::CPUI_INT_SUB), "INT_SUB is arithmetic");
    assert!(arith_or_float(OpCode::CPUI_FLOAT_ADD), "FLOAT_ADD is floating-point");
    assert!(arith_or_float(OpCode::CPUI_FLOAT_MULT), "FLOAT_MULT is floating-point");
    // The piece-forming / move opcodes must NOT be mistaken for a logical whole.
    assert!(!arith_or_float(OpCode::CPUI_COPY), "COPY is neither (typeop.cc:391)");
    assert!(!arith_or_float(OpCode::CPUI_PIECE), "PIECE is neither (typeop.cc:2039)");
    assert!(!arith_or_float(OpCode::CPUI_SUBPIECE), "SUBPIECE is neither");
    // Logical (bitwise) ops are also NOT arithmetic/float — the comment in
    // attemptMarking notes it is "hard to tell" for logical ops, and the C++
    // deliberately declines them.
    assert!(!arith_or_float(OpCode::CPUI_INT_AND), "INT_AND is logical, not arith/float");
    assert!(!arith_or_float(OpCode::CPUI_INT_OR), "INT_OR is logical, not arith/float");
}

/// Build a factory wired with the alignment map + a max base size of 8, the
/// minimal setup the `getBase` paths the local-type calculators hit require.
fn factory() -> TypeFactoryImpl {
    let f = TypeFactoryImpl::new();
    f.set_default_alignment_map();
    f.set_max_basetype_size(8);
    f
}

/// **getOutputLocal matrix.** Binary/Unary/Func return `getBase(out_size,
/// metaout)`; PTRADD/PTRSUB override to `getBase(out_size, TYPE_INT)`; the Base
/// default is `getBase(out_size, TYPE_UNKNOWN)`.
#[test]
fn get_output_local_matrix() {
    let f = factory();
    // TypeOpIntAdd: metaout INT.
    let out = type_op_info(OpCode::CPUI_INT_ADD).get_output_local(&f, 4).unwrap();
    assert_eq!(out.get_metatype(), TYPE_INT);
    assert_eq!(out.get_size(), 4);
    // TypeOpIntDiv: metaout UINT.
    let out = type_op_info(OpCode::CPUI_INT_DIV).get_output_local(&f, 4).unwrap();
    assert_eq!(out.get_metatype(), TYPE_UINT);
    // TypeOpEqual: metaout BOOL (size-1 output).
    let out = type_op_info(OpCode::CPUI_INT_EQUAL).get_output_local(&f, 1).unwrap();
    assert_eq!(out.get_metatype(), TYPE_BOOL);
    // TypeOpFloatAdd: metaout FLOAT.
    let out = type_op_info(OpCode::CPUI_FLOAT_ADD).get_output_local(&f, 8).unwrap();
    assert_eq!(out.get_metatype(), TYPE_FLOAT);
    // TypeOpPtradd / TypeOpPtrsub: override to TYPE_INT regardless of class.
    let out = type_op_info(OpCode::CPUI_PTRADD).get_output_local(&f, 8).unwrap();
    assert_eq!(out.get_metatype(), TYPE_INT);
    let out = type_op_info(OpCode::CPUI_PTRSUB).get_output_local(&f, 8).unwrap();
    assert_eq!(out.get_metatype(), TYPE_INT);
    // TypeOpCopy (Base): default TYPE_UNKNOWN.
    let out = type_op_info(OpCode::CPUI_COPY).get_output_local(&f, 4).unwrap();
    assert_eq!(out.get_metatype(), TYPE_UNKNOWN);
}

/// **getInputLocal matrix.** Generic classes use `metain`; the per-op overrides
/// (shift slot-1 -> getBaseNoChar INT; PTRADD/PTRSUB/CPOOLREF -> INT; CBRANCH slot
/// 1 -> BOOL; INSERT slots<=1 / ZPULL,SPULL slot 0 -> UNKNOWN) match `typeop.cc`.
#[test]
fn get_input_local_matrix() {
    let f = factory();
    // TypeOpIntAdd: metain INT for any slot.
    let i = type_op_info(OpCode::CPUI_INT_ADD).get_input_local(&f, 0, 4).unwrap();
    assert_eq!(i.get_metatype(), TYPE_INT);
    // TypeOpIntLess: metain UINT.
    let i = type_op_info(OpCode::CPUI_INT_LESS).get_input_local(&f, 0, 4).unwrap();
    assert_eq!(i.get_metatype(), TYPE_UINT);
    // TypeOpFloatAdd: metain FLOAT.
    let i = type_op_info(OpCode::CPUI_FLOAT_ADD).get_input_local(&f, 0, 8).unwrap();
    assert_eq!(i.get_metatype(), TYPE_FLOAT);

    // TypeOpIntLeft: slot 1 -> getBaseNoChar(size, TYPE_INT); slot 0 -> metain INT.
    let s1 = type_op_info(OpCode::CPUI_INT_LEFT).get_input_local(&f, 1, 4).unwrap();
    assert_eq!(s1.get_metatype(), TYPE_INT);
    let s0 = type_op_info(OpCode::CPUI_INT_LEFT).get_input_local(&f, 0, 4).unwrap();
    assert_eq!(s0.get_metatype(), TYPE_INT);
    // TypeOpIntRight: slot 1 -> INT (not UINT, despite metain UINT — the slot-1
    // shift count is INT per getInputLocal override).
    let s1 = type_op_info(OpCode::CPUI_INT_RIGHT).get_input_local(&f, 1, 4).unwrap();
    assert_eq!(s1.get_metatype(), TYPE_INT);
    // slot 0 of INT_RIGHT falls to metain (UINT).
    let s0 = type_op_info(OpCode::CPUI_INT_RIGHT).get_input_local(&f, 0, 4).unwrap();
    assert_eq!(s0.get_metatype(), TYPE_UINT);
    // INT_SRIGHT slot 1 -> INT.
    let s1 = type_op_info(OpCode::CPUI_INT_SRIGHT).get_input_local(&f, 1, 4).unwrap();
    assert_eq!(s1.get_metatype(), TYPE_INT);

    // TypeOpPtradd / TypeOpPtrsub / TypeOpCpoolref: TYPE_INT.
    assert_eq!(
        type_op_info(OpCode::CPUI_PTRADD).get_input_local(&f, 0, 8).unwrap().get_metatype(),
        TYPE_INT
    );
    assert_eq!(
        type_op_info(OpCode::CPUI_PTRSUB).get_input_local(&f, 1, 8).unwrap().get_metatype(),
        TYPE_INT
    );
    assert_eq!(
        type_op_info(OpCode::CPUI_CPOOLREF).get_input_local(&f, 0, 4).unwrap().get_metatype(),
        TYPE_INT
    );

    // TypeOpCbranch: slot 1 -> BOOL.
    assert_eq!(
        type_op_info(OpCode::CPUI_CBRANCH).get_input_local(&f, 1, 1).unwrap().get_metatype(),
        TYPE_BOOL
    );

    // TypeOpInsert: slots <=1 -> UNKNOWN; slot 2 -> metain INT.
    assert_eq!(
        type_op_info(OpCode::CPUI_INSERT).get_input_local(&f, 0, 4).unwrap().get_metatype(),
        TYPE_UNKNOWN
    );
    assert_eq!(
        type_op_info(OpCode::CPUI_INSERT).get_input_local(&f, 1, 4).unwrap().get_metatype(),
        TYPE_UNKNOWN
    );
    assert_eq!(
        type_op_info(OpCode::CPUI_INSERT).get_input_local(&f, 2, 4).unwrap().get_metatype(),
        TYPE_INT
    );

    // TypeOpZpull / TypeOpSpull: slot 0 -> UNKNOWN; other slots -> metain.
    assert_eq!(
        type_op_info(OpCode::CPUI_ZPULL).get_input_local(&f, 0, 4).unwrap().get_metatype(),
        TYPE_UNKNOWN
    );
    assert_eq!(
        type_op_info(OpCode::CPUI_ZPULL).get_input_local(&f, 1, 4).unwrap().get_metatype(),
        TYPE_INT
    );
    assert_eq!(
        type_op_info(OpCode::CPUI_SPULL).get_input_local(&f, 0, 4).unwrap().get_metatype(),
        TYPE_UNKNOWN
    );
    assert_eq!(
        type_op_info(OpCode::CPUI_SPULL).get_input_local(&f, 2, 4).unwrap().get_metatype(),
        TYPE_INT
    );
}

/// **floatSignManipulation.** INT_AND with the `(allbits>>1)` mask -> FLOAT_ABS;
/// INT_XOR with the `(val ^ val>>1)` sign bit -> FLOAT_NEG; anything else ->
/// CPUI_MAX (typeop.cc:154-177).
#[test]
fn float_sign_manipulation_cases() {
    use kuna_base::address::calc_mask;
    // size 8: calc_mask(8) = 0xffffffffffffffff.
    let mask8 = calc_mask(8);
    // INT_AND with offset == mask>>1 -> FLOAT_ABS.
    let abs_off = mask8 >> 1;
    assert_eq!(
        float_sign_manipulation(OpCode::CPUI_INT_AND, Some((8, abs_off))),
        OpCode::CPUI_FLOAT_ABS
    );
    // INT_AND with a different offset -> CPUI_MAX.
    assert_eq!(
        float_sign_manipulation(OpCode::CPUI_INT_AND, Some((8, 0x1234))),
        OpCode::CPUI_MAX
    );
    // INT_AND with no constant slot 1 -> CPUI_MAX.
    assert_eq!(float_sign_manipulation(OpCode::CPUI_INT_AND, None), OpCode::CPUI_MAX);
    // INT_XOR with offset == val ^ (val>>1) (the sign bit) -> FLOAT_NEG.
    let neg_off = mask8 ^ (mask8 >> 1); // == 0x8000000000000000
    assert_eq!(
        float_sign_manipulation(OpCode::CPUI_INT_XOR, Some((8, neg_off))),
        OpCode::CPUI_FLOAT_NEG
    );
    // INT_XOR with a different offset -> CPUI_MAX.
    assert_eq!(
        float_sign_manipulation(OpCode::CPUI_INT_XOR, Some((8, 0x4000))),
        OpCode::CPUI_MAX
    );
    // A non-AND/XOR op -> CPUI_MAX regardless of operand.
    assert_eq!(
        float_sign_manipulation(OpCode::CPUI_INT_ADD, Some((8, neg_off))),
        OpCode::CPUI_MAX
    );
}

/// **selectJavaOperators.** Toggling Java re-types the unsigned ops to signed and
/// flips INT_RIGHT's symbol to `>>>`; toggling back restores UINT and `>>`.  Only
/// the op-codes the C++ touches are changed.
#[test]
fn select_java_operators_toggle() {
    let mut inst = register_instructions();

    // Default (C) state, as registered.
    assert_eq!(inst[OpCode::CPUI_INT_RIGHT as usize].unwrap().name, ">>");
    assert_eq!(inst[OpCode::CPUI_INT_RIGHT as usize].unwrap().metaout, TYPE_UINT);
    assert_eq!(inst[OpCode::CPUI_INT_ZEXT as usize].unwrap().metaout, TYPE_UINT);

    // Switch to Java.
    select_java_operators(&mut inst, true);
    let r = inst[OpCode::CPUI_INT_RIGHT as usize].unwrap();
    assert_eq!(r.name, ">>>");
    assert_eq!(r.metain, TYPE_INT);
    assert_eq!(r.metaout, TYPE_INT);
    // INT_ZEXT: metain UNKNOWN, metaout INT (the asymmetric Java case).
    let z = inst[OpCode::CPUI_INT_ZEXT as usize].unwrap();
    assert_eq!(z.metain, TYPE_UNKNOWN);
    assert_eq!(z.metaout, TYPE_INT);
    // INT_NEGATE/XOR/OR/AND all become INT in Java.
    for opc in [
        OpCode::CPUI_INT_NEGATE,
        OpCode::CPUI_INT_XOR,
        OpCode::CPUI_INT_OR,
        OpCode::CPUI_INT_AND,
    ] {
        let rec = inst[opc as usize].unwrap();
        assert_eq!(rec.metain, TYPE_INT, "{opc:?} metain Java");
        assert_eq!(rec.metaout, TYPE_INT, "{opc:?} metaout Java");
    }
    // An untouched op (INT_ADD) keeps its registered metatypes.
    assert_eq!(inst[OpCode::CPUI_INT_ADD as usize].unwrap().metaout, TYPE_INT);

    // Switch back to C.
    select_java_operators(&mut inst, false);
    let r = inst[OpCode::CPUI_INT_RIGHT as usize].unwrap();
    assert_eq!(r.name, ">>");
    assert_eq!(r.metaout, TYPE_UINT);
    assert_eq!(inst[OpCode::CPUI_INT_ZEXT as usize].unwrap().metaout, TYPE_UINT);
    assert_eq!(inst[OpCode::CPUI_INT_AND as usize].unwrap().metaout, TYPE_UINT);
}

/// `type_op_info(CPUI_MAX)` panics (matches the null `inst[CPUI_MAX]` in C++).
#[test]
#[should_panic(expected = "CPUI_MAX")]
fn cpui_max_has_no_entry() {
    let _ = type_op_info(OpCode::CPUI_MAX);
}

//! Unit tests for the `signedness` option surface and the evidence fold.

use super::*;

#[test]
fn option_parses_every_catalog_value() {
    for (text, want) in [
        ("upstream", SignPolicy::Upstream),
        ("auto", SignPolicy::Auto),
        ("prefer-signed", SignPolicy::PreferSigned),
        ("prefer-unsigned", SignPolicy::PreferUnsigned),
    ] {
        let (got, msg) = OptionSignedness.apply(text).expect("valid value");
        assert_eq!(got, want);
        assert!(msg.contains(text), "{msg}");
    }
}

#[test]
fn option_rejects_an_unknown_value() {
    let err = OptionSignedness.apply("zzz").unwrap_err();
    assert!(format!("{err}").contains("signedness"), "{err}");
}

#[test]
fn upstream_is_the_default_and_is_inactive() {
    assert_eq!(SignPolicy::default(), SignPolicy::Upstream);
    assert!(!SignPolicy::Upstream.is_active());
    assert!(SignPolicy::Auto.is_active());
    assert!(SignPolicy::PreferSigned.is_active());
    assert!(SignPolicy::PreferUnsigned.is_active());
}

#[test]
fn evidence_folds_to_veto_on_disagreement() {
    assert_eq!(Evidence::None.join(Evidence::Signed), Evidence::Signed);
    assert_eq!(Evidence::Signed.join(Evidence::Signed), Evidence::Signed);
    assert_eq!(Evidence::Signed.join(Evidence::Unsigned), Evidence::Veto);
    assert_eq!(Evidence::Unsigned.join(Evidence::None), Evidence::Unsigned);
    assert_eq!(Evidence::Veto.join(Evidence::None), Evidence::Veto);
}

#[test]
fn the_signedness_sensitive_operators_demand_what_the_c_operator_means() {
    use OpCode::*;
    use ReaderClass::*;
    // Ordered comparisons: both operands carry the sign, and the result is a
    // condition, so the walk ends there.
    for opc in [CPUI_INT_SLESS, CPUI_INT_SLESSEQUAL] {
        assert_eq!(classify_reader(opc, 0), Demands(Evidence::Signed), "{opc:?}");
        assert_eq!(classify_reader(opc, 1), Demands(Evidence::Signed), "{opc:?}");
    }
    for opc in [CPUI_INT_LESS, CPUI_INT_LESSEQUAL] {
        assert_eq!(classify_reader(opc, 0), Demands(Evidence::Unsigned), "{opc:?}");
        assert_eq!(classify_reader(opc, 1), Demands(Evidence::Unsigned), "{opc:?}");
    }
    // Division and remainder: both operands carry the sign, and the quotient is
    // printed at their converted type, so the demand does not end the walk.
    for opc in [CPUI_INT_SDIV, CPUI_INT_SREM] {
        assert_eq!(classify_reader(opc, 0), DemandsCarrying(Evidence::Signed), "{opc:?}");
        assert_eq!(classify_reader(opc, 1), DemandsCarrying(Evidence::Signed), "{opc:?}");
    }
    for opc in [CPUI_INT_DIV, CPUI_INT_REM] {
        assert_eq!(classify_reader(opc, 0), DemandsCarrying(Evidence::Unsigned), "{opc:?}");
        assert_eq!(classify_reader(opc, 1), DemandsCarrying(Evidence::Unsigned), "{opc:?}");
    }
    // A right shift carries the sign on the shiftee only, and prints at the
    // shiftee's type, so it demands and keeps walking; the count converts
    // identically either way.
    assert_eq!(classify_reader(CPUI_INT_SRIGHT, 0), DemandsCarrying(Evidence::Signed));
    assert_eq!(classify_reader(CPUI_INT_SRIGHT, 1), Opaque);
    assert_eq!(classify_reader(CPUI_INT_RIGHT, 0), DemandsCarrying(Evidence::Unsigned));
    assert_eq!(classify_reader(CPUI_INT_RIGHT, 1), Opaque);
    // Widening: the printed cast re-establishes the type, so no walk.
    assert_eq!(classify_reader(CPUI_INT_SEXT, 0), Demands(Evidence::Signed));
    assert_eq!(classify_reader(CPUI_INT_ZEXT, 0), Demands(Evidence::Unsigned));
}

/// `<<` prefers unsigned and carries: the shifted bits are the same under either
/// declaration (so this is a preference, not a soundness requirement), and the
/// printed result keeps the shiftee's type, so a `>>` further out still decides.
#[test]
fn a_left_shift_prefers_unsigned_and_still_carries() {
    use OpCode::*;
    use ReaderClass::*;
    assert_eq!(classify_reader(CPUI_INT_LEFT, 0), DemandsCarrying(Evidence::Unsigned));
    assert_eq!(classify_reader(CPUI_INT_LEFT, 1), Opaque);
}

#[test]
fn the_neutral_operators_do_not_constrain_the_declaration() {
    use OpCode::*;
    use ReaderClass::*;
    // `+ - * & | ^ ~` and the value-moving ops keep printing the operand's type,
    // so they extend the walk instead of ending it.
    for opc in [
        CPUI_INT_ADD,
        CPUI_INT_SUB,
        CPUI_INT_MULT,
        CPUI_INT_AND,
        CPUI_INT_OR,
        CPUI_INT_XOR,
        CPUI_INT_NEGATE,
        CPUI_INT_2COMP,
        CPUI_COPY,
        CPUI_MULTIEQUAL,
        CPUI_INDIRECT,
    ] {
        assert_eq!(classify_reader(opc, 0), Carries, "{opc:?}");
    }
    // These are signedness-independent AND re-establish a type of their own.
    for opc in [
        CPUI_INT_EQUAL,
        CPUI_INT_NOTEQUAL,
        CPUI_SUBPIECE,
        CPUI_PIECE,
        CPUI_CBRANCH,
        CPUI_RETURN,
    ] {
        assert_eq!(classify_reader(opc, 0), Opaque, "{opc:?}");
    }
    // A call argument converts at a fixed width; the call target is not a value.
    assert_eq!(classify_reader(CPUI_CALL, 1), Opaque);
    assert_eq!(classify_reader(CPUI_CALL, 0), Veto);
    // A stored value converts at a fixed width; the address does not.
    assert_eq!(classify_reader(CPUI_STORE, 2), Opaque);
    assert_eq!(classify_reader(CPUI_STORE, 1), Veto);
}

#[test]
fn an_unclassified_reader_vetoes() {
    use OpCode::*;
    use ReaderClass::*;
    // Nothing outside the classified sets may re-sign a declaration: a pointer
    // index, a dereference, a float, an indirect branch.
    for opc in [
        CPUI_PTRADD,
        CPUI_PTRSUB,
        CPUI_LOAD,
        CPUI_FLOAT_ADD,
        CPUI_FLOAT_INT2FLOAT,
        CPUI_BRANCHIND,
        CPUI_CPOOLREF,
        CPUI_NEW,
        CPUI_SEGMENTOP,
        CPUI_INSERT,
        CPUI_ZPULL,
        CPUI_SPULL,
    ] {
        assert_eq!(classify_reader(opc, 0), Veto, "{opc:?}");
        assert_eq!(classify_reader(opc, 1), Veto, "{opc:?}");
    }
}

#[test]
fn a_definition_votes_but_never_vetoes() {
    use OpCode::*;
    assert_eq!(def_evidence(CPUI_INT_SDIV), Evidence::Signed);
    assert_eq!(def_evidence(CPUI_INT_SREM), Evidence::Signed);
    assert_eq!(def_evidence(CPUI_INT_SRIGHT), Evidence::Signed);
    assert_eq!(def_evidence(CPUI_INT_DIV), Evidence::Unsigned);
    assert_eq!(def_evidence(CPUI_INT_RIGHT), Evidence::Unsigned);
    // An extension describes the SOURCE operand's type, not the widened value's,
    // in both directions: `int v = *p;` off an `unsigned char *` and
    // `uintmax_t max = (long)(int)n;` off a `movslq` are both ordinary C.  The
    // INT_SEXT vote is what declared fmt::main's `uintmax_t max` as `long`.
    assert_eq!(def_evidence(CPUI_INT_ZEXT), Evidence::None);
    assert_eq!(def_evidence(CPUI_INT_SEXT), Evidence::None);
    // `-x` has the same bits whichever way `x` is read.
    assert_eq!(def_evidence(CPUI_INT_2COMP), Evidence::None);
    assert_eq!(def_evidence(CPUI_LOAD), Evidence::None);
    assert_eq!(def_evidence(CPUI_PTRADD), Evidence::None);
}

#[test]
fn only_a_sole_named_declaration_keeps_its_flip() {
    let int4 = Rc::new(Datatype::new(4, type_metatype::TYPE_INT));
    let mut plan = SignPlan::default();
    for id in 1..=4u32 {
        plan.decls.insert(HighVariableId(id), Rc::clone(&int4));
    }
    // v1 is declared once; v2 is declared by two highs (a group about to collapse
    // onto one line); high 4 never reaches the candidate list at all (a signature
    // parameter, or a piece suppressed before the collapses run).
    plan.retain_sole_named([
        (HighVariableId(1), "v1"),
        (HighVariableId(2), "v2"),
        (HighVariableId(3), "v2"),
    ]);
    assert!(plan.decl_type(HighVariableId(1)).is_some());
    assert!(plan.decl_type(HighVariableId(2)).is_none());
    assert!(plan.decl_type(HighVariableId(3)).is_none());
    assert!(plan.decl_type(HighVariableId(4)).is_none());
}

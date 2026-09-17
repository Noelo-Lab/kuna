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
    // Ordered comparisons, division, remainder: both operands carry the sign.
    for opc in [CPUI_INT_SLESS, CPUI_INT_SLESSEQUAL, CPUI_INT_SDIV, CPUI_INT_SREM] {
        assert_eq!(classify_reader(opc, 0), Demands(Evidence::Signed), "{opc:?}");
        assert_eq!(classify_reader(opc, 1), Demands(Evidence::Signed), "{opc:?}");
    }
    for opc in [CPUI_INT_LESS, CPUI_INT_LESSEQUAL, CPUI_INT_DIV, CPUI_INT_REM] {
        assert_eq!(classify_reader(opc, 0), Demands(Evidence::Unsigned), "{opc:?}");
        assert_eq!(classify_reader(opc, 1), Demands(Evidence::Unsigned), "{opc:?}");
    }
    // A shift carries the sign on the shiftee only; the count converts identically.
    assert_eq!(classify_reader(CPUI_INT_SRIGHT, 0), Demands(Evidence::Signed));
    assert_eq!(classify_reader(CPUI_INT_SRIGHT, 1), Opaque);
    assert_eq!(classify_reader(CPUI_INT_RIGHT, 0), Demands(Evidence::Unsigned));
    assert_eq!(classify_reader(CPUI_INT_RIGHT, 1), Opaque);
    // Widening.
    assert_eq!(classify_reader(CPUI_INT_SEXT, 0), Demands(Evidence::Signed));
    assert_eq!(classify_reader(CPUI_INT_ZEXT, 0), Demands(Evidence::Unsigned));
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
    assert_eq!(def_evidence(CPUI_INT_SEXT), Evidence::Signed);
    assert_eq!(def_evidence(CPUI_INT_DIV), Evidence::Unsigned);
    // A zero-extension describes the SOURCE operand's type, not the widened
    // value's (TRex `ZeroExtendTgt => None`).
    assert_eq!(def_evidence(CPUI_INT_ZEXT), Evidence::None);
    assert_eq!(def_evidence(CPUI_LOAD), Evidence::None);
    assert_eq!(def_evidence(CPUI_PTRADD), Evidence::None);
}

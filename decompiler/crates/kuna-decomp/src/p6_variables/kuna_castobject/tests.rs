use super::*;

#[test]
fn option_parses_on_and_off() {
    assert!(OptionCastObject.apply("on").unwrap().0);
    assert!(!OptionCastObject.apply("off").unwrap().0);
    assert!(OptionCastObject.apply("maybe").is_err());
}

fn demand(r: Read) -> Option<(bool, bool)> {
    match r {
        Read::Demands { signed, carry } => Some((signed, carry)),
        _ => None,
    }
}

#[test]
fn the_sign_sensitive_operators_vote_for_the_sign_they_compute_with() {
    use OpCode::*;
    assert_eq!(demand(classify(CPUI_INT_SLESS, 0)), Some((true, false)));
    assert_eq!(demand(classify(CPUI_INT_LESSEQUAL, 1)), Some((false, false)));
    assert_eq!(demand(classify(CPUI_INT_SDIV, 0)), Some((true, true)));
    assert_eq!(demand(classify(CPUI_INT_REM, 1)), Some((false, true)));
    assert_eq!(demand(classify(CPUI_INT_SRIGHT, 0)), Some((true, true)));
    assert_eq!(demand(classify(CPUI_INT_RIGHT, 0)), Some((false, true)));
    assert_eq!(demand(classify(CPUI_INT_SEXT, 0)), Some((true, false)));
    assert_eq!(demand(classify(CPUI_INT_ZEXT, 0)), Some((false, false)));
}

#[test]
fn a_shift_count_and_a_call_argument_do_not_vote() {
    use OpCode::*;
    assert!(matches!(classify(CPUI_INT_RIGHT, 1), Read::Opaque));
    assert!(matches!(classify(CPUI_INT_SRIGHT, 1), Read::Opaque));
    assert!(matches!(classify(CPUI_CALL, 2), Read::Opaque));
    assert!(matches!(classify(CPUI_STORE, 2), Read::Opaque));
    assert!(matches!(classify(CPUI_INT_EQUAL, 0), Read::Opaque));
}

#[test]
fn arithmetic_that_wraps_is_an_overflow_and_an_address_is_not_modelled() {
    use OpCode::*;
    for opc in [CPUI_INT_ADD, CPUI_INT_SUB, CPUI_INT_MULT, CPUI_INT_2COMP] {
        assert!(matches!(classify(opc, 0), Read::Overflows));
    }
    assert!(matches!(classify(CPUI_INT_LEFT, 0), Read::Overflows));
    assert!(matches!(classify(CPUI_INT_AND, 0), Read::Carries));
    assert!(matches!(classify(CPUI_COPY, 0), Read::Carries));
    assert!(matches!(classify(CPUI_MULTIEQUAL, 1), Read::Carries));
    assert!(matches!(classify(CPUI_LOAD, 1), Read::Veto));
    assert!(matches!(classify(CPUI_STORE, 1), Read::Veto));
    assert!(matches!(classify(CPUI_PTRADD, 1), Read::Veto));
    assert!(matches!(classify(CPUI_FLOAT_INT2FLOAT, 0), Read::Veto));
    assert!(matches!(classify(CPUI_CALL, 0), Read::Veto));
}

//! Tests for the type arithmetic of `castternary`.  The printed rule is covered
//! end to end by `tests/stages/kuna-castternary.xml` (pass 1 off, pass 2 on) and
//! by the compiled round trip in `kuna-cli/tests/decompile_all_cli.rs`.

use super::*;

fn op(n: u64) -> OpId {
    OpId::from(slotmap::KeyData::from_ffi(n | (1 << 32)))
}

#[test]
fn usual_conversions_follow_c11_6_3_1_8() {
    assert_eq!(usual(INT, INT), INT);
    assert_eq!(usual(INT, UINT), UINT);
    assert_eq!(usual(UINT, INT), UINT);
    assert_eq!(usual(INT, LONG), LONG);
    assert_eq!(usual(UINT, LONG), LONG);
    assert_eq!(usual(LONG, UINT), LONG);
    assert_eq!(usual(INT, ULONG), ULONG);
    assert_eq!(usual(LONG, ULONG), ULONG);
    assert_eq!(usual(ULONG, LONG), ULONG);
}

#[test]
fn common_needs_one_type_for_every_combination() {
    assert_eq!(common(INT, INT), Some(INT));
    assert_eq!(common(INT | UINT, LONG), Some(LONG));
    assert_eq!(common(INT | UINT, INT), None);
    assert_eq!(common(INT | UINT | LONG | ULONG, ULONG), Some(ULONG));
    assert_eq!(common(INT | UINT | LONG | ULONG, LONG), None);
    assert_eq!(common(UINT | LONG, INT), None);
}

fn arm(kept: u8, conv: Option<(u64, u8, u8)>) -> Arm {
    Arm { kept, conv: conv.map(|(o, t, s)| (op(o), t, s)) }
}

#[test]
fn a_promoted_byte_arm_against_an_int_arm_drops() {
    // c ? (int)ch : 0
    assert_eq!(choose(arm(INT, Some((1, INT, INT))), arm(INT, None)), vec![op(1)]);
}

#[test]
fn an_arm_whose_type_the_other_arm_widens_keeps_its_cast() {
    // c ? (int)ch : l -- the conditional is a long, not the int the cast names
    assert!(choose(arm(INT, Some((1, INT, INT))), arm(LONG, None)).is_empty());
    // c ? (long)i : 0 -- without the cast the conditional is an int
    assert!(choose(arm(LONG, Some((1, LONG, INT))), arm(INT, None)).is_empty());
}

#[test]
fn a_widening_to_the_other_arms_type_drops() {
    // c ? (long)i : l
    assert_eq!(choose(arm(LONG, Some((1, LONG, INT))), arm(LONG, None)), vec![op(1)]);
    // c ? (unsigned long)uc : ul
    assert_eq!(choose(arm(ULONG, Some((1, ULONG, INT))), arm(ULONG, None)), vec![op(1)]);
}

#[test]
fn an_unknown_operand_must_convert_the_same_under_every_promotion() {
    // c ? (unsigned int)(a & b) : 0xffffffff where a & b may be int or unsigned int
    assert_eq!(
        choose(arm(UINT, Some((1, UINT, INT | UINT))), arm(UINT | LONG, None)),
        Vec::<OpId>::new()
    );
    assert_eq!(choose(arm(LONG, Some((1, LONG, INT | UINT))), arm(LONG, None)), vec![op(1)]);
}

#[test]
fn both_casts_go_only_when_the_bare_pair_keeps_the_type() {
    // c ? (long)i : (long)u -- bare, int and unsigned int meet as unsigned int
    assert_eq!(
        choose(arm(LONG, Some((1, LONG, INT))), arm(LONG, Some((2, LONG, UINT)))),
        vec![op(1)]
    );
    // c ? (int)ch : (int)sh
    assert_eq!(
        choose(arm(INT, Some((1, INT, INT))), arm(INT, Some((2, INT, INT)))),
        vec![op(1), op(2)]
    );
    // c ? (unsigned long)uc : (unsigned long)ui
    assert_eq!(
        choose(arm(ULONG, Some((1, ULONG, INT))), arm(ULONG, Some((2, ULONG, UINT)))),
        vec![op(1)]
    );
}

#[test]
fn an_ambiguous_conditional_is_left_alone() {
    assert!(choose(arm(INT, Some((1, INT, INT))), arm(INT | UINT, None)).is_empty());
}

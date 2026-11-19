use super::*;

fn form(c: i64, terms: &[(u32, i64)]) -> Form {
    Form { c, terms: terms.to_vec() }
}

#[test]
fn a_merge_and_a_return_pin_the_call_between_them() {
    // x0 is a call on one arm of a branch; the join says the arm is 4 short.
    let solved = eliminate(&[form(-4, &[(0, 1)])], 1).unwrap();
    assert_eq!(solved, vec![Some(Rat::int(4))]);
}

#[test]
fn an_alignment_unknown_absorbs_its_equation() {
    // x1 is an `and esp,-16`; with it in the only equation x0 has no one value.
    let solved = eliminate(&[form(-8, &[(0, 1), (1, 1)])], 2).unwrap();
    assert_eq!(solved, vec![None, None]);
    // A second equation without x1 pins both.
    let solved = eliminate(&[form(-8, &[(0, 1), (1, 1)]), form(-4, &[(0, 1)])], 2).unwrap();
    assert_eq!(solved, vec![Some(Rat::int(4)), Some(Rat::int(4))]);
}

#[test]
fn two_calls_sharing_one_equation_stay_unknown() {
    let solved = eliminate(&[form(-4, &[(0, 1), (1, 1)])], 2).unwrap();
    assert_eq!(solved, vec![None, None]);
}

#[test]
fn contradicting_equations_have_no_solution() {
    assert!(eliminate(&[form(-4, &[(0, 1)]), form(-8, &[(0, 1)])], 1).is_none());
    assert!(eliminate(&[form(4, &[])], 0).is_none());
}

#[test]
fn a_fractional_solution_is_kept_exact() {
    let solved = eliminate(&[form(-4, &[(0, 3)])], 1).unwrap();
    assert_eq!(solved, vec![Some(Rat { n: 4, d: 3 })]);
}

#[test]
fn rational_arithmetic_refuses_overflow() {
    assert!(Rat::int(i64::MAX).mul(Rat::int(2)).is_none());
    assert!(Rat::int(1).div(Rat::ZERO).is_none());
    assert_eq!(Rat::new(-6, -4), Some(Rat { n: 3, d: 2 }));
    assert!(form(i64::MAX, &[]).shift(1).is_none());
}

#[test]
fn forms_cancel_terms() {
    let mut f = form(8, &[(2, 1)]);
    f.add_var(2, -1).unwrap();
    f.add_var(1, 1).unwrap();
    assert_eq!(f, form(8, &[(1, 1)]));
    assert_eq!(f.minus(&form(8, &[(1, 1)])).unwrap(), Form::default());
}

fn rows(forms: &[Form]) -> Vec<Row> {
    forms.iter().map(|f| Row::of(f).unwrap()).collect()
}

#[test]
fn a_call_that_must_have_popped_less_than_nothing_cannot_return() {
    // x0 + x1 = -12 with x0 a pop: x1 is below zero whatever x0 is.
    let system = rows(&[form(12, &[(0, 1), (1, 1)])]);
    assert_eq!(feasible(&system, &[true, false]), Some(true));
    assert_eq!(feasible(&system, &[true, true]), Some(false));
}

#[test]
fn a_pop_on_the_other_path_keeps_the_call_returning() {
    // x1 - x0 = -12: a callee popping 12 on the other path explains it.
    let system = rows(&[form(12, &[(0, -1), (1, 1)])]);
    assert_eq!(feasible(&system, &[true, true]), Some(true));
    // Unless a return pins that callee at zero.
    let system = rows(&[form(12, &[(0, -1), (1, 1)]), form(0, &[(0, 1), (2, 1)])]);
    assert_eq!(feasible(&system, &[true, true, true]), Some(false));
}

#[test]
fn an_unrestricted_unknown_is_eliminated_before_the_test() {
    // x2 is an alignment: x0 + x2 = 8 and x1 - x2 = -20, so x0 + x1 = -12.
    let system = rows(&[form(-8, &[(0, 1), (2, 1)]), form(20, &[(1, 1), (2, -1)])]);
    assert_eq!(feasible(&system, &[true, true, false]), Some(false));
    assert_eq!(feasible(&system, &[true, false, false]), Some(true));
}

#[test]
fn a_fixed_value_is_one_more_equation() {
    let mut system = rows(&[form(-16, &[(0, 1)])]);
    assert_eq!(feasible(&system, &[false]), Some(true));
    system.push(Row::of(&Form::var(0)).unwrap());
    assert_eq!(feasible(&system, &[false]), Some(false));
}

#[test]
fn pops_that_sum_to_nothing_are_nothing() {
    // x0 + x1 = 0 at a return, x1 + x2 = 4 at a merge.
    let forms = [form(0, &[(0, 1), (1, 1)]), form(-4, &[(1, 1), (2, 1)])];
    assert_eq!(eliminate(&forms, 3).unwrap(), vec![None, None, None]);
    let solved = eliminate_signed(&forms, &[true, true, true]).unwrap();
    assert_eq!(solved, vec![Some(Rat::ZERO), Some(Rat::ZERO), Some(Rat::int(4))]);
    // A call that may not return is not held to a sign.
    assert_eq!(eliminate_signed(&forms, &[true, false, true]).unwrap(), vec![None, None, None]);
}

#[test]
fn a_pop_is_a_positive_multiple_of_the_slot() {
    assert!(is_pop(Rat::int(4), 4));
    assert!(is_pop(Rat::int(0x400), 4));
    assert!(!is_pop(Rat::int(0), 4));
    assert!(!is_pop(Rat::int(-4), 4));
    assert!(!is_pop(Rat::int(6), 4));
    assert!(!is_pop(Rat::int(0x404), 4));
    assert!(!is_pop(Rat { n: 4, d: 3 }, 4));
}

#[test]
fn spanning_order_visits_reachable_blocks_parent_first() {
    // 0 -> 1 -> 3, 0 -> 2 -> 3, 3 -> 1 (loop), 4 unreachable.
    let succs = vec![vec![1, 2], vec![3], vec![3], vec![1], vec![0]];
    let (order, parent) = spanning_order(&succs);
    assert_eq!(order.len(), 4);
    assert_eq!(order[0], 0);
    assert!(!order.contains(&4));
    for &b in &order[1..] {
        let p = parent[b].unwrap();
        assert!(order.iter().position(|&x| x == p) < order.iter().position(|&x| x == b));
    }
}

#[test]
fn sign_extension_follows_the_operand_size() {
    assert_eq!(sign_extend(0xffff_fff4, 4), -12);
    assert_eq!(sign_extend(0x7f, 1), 0x7f);
    assert_eq!(sign_extend(0xff, 1), -1);
    assert_eq!(sign_extend(u64::MAX, 8), -1);
}

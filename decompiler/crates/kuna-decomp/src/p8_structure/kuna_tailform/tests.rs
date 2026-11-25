//! Tests for the `tailform` decision function and its option value.

use super::*;

fn site(shape: TailShape, void: bool, lines: usize) -> TailSite {
    TailSite { shape, void, lines, feats: None }
}

#[test]
fn a_short_guarded_tail_is_wrapped() {
    let p = TailFormPolicy::default();
    assert_eq!(decide(&site(TailShape::Guard, true, 1), &p), TailDecision::Wrap);
    assert_eq!(decide(&site(TailShape::Guard, false, DEFAULT_MAX_LINES as usize), &p), TailDecision::Wrap);
}

#[test]
fn a_long_guarded_tail_stays_a_guard() {
    let p = TailFormPolicy::default();
    assert_eq!(decide(&site(TailShape::Guard, true, DEFAULT_MAX_LINES as usize + 1), &p), TailDecision::Leave);
}

#[test]
fn a_long_wrapped_body_becomes_a_guard() {
    let p = TailFormPolicy::default();
    assert_eq!(decide(&site(TailShape::Wrapped, true, DEFAULT_MAX_LINES as usize + 1), &p), TailDecision::Guard);
    assert_eq!(decide(&site(TailShape::Wrapped, false, 2), &p), TailDecision::Leave);
}

#[test]
fn void_and_valued_tails_use_their_own_threshold() {
    let p = TailFormPolicy { void_max: 1, value_max: 6, ..Default::default() };
    assert_eq!(decide(&site(TailShape::Guard, true, 2), &p), TailDecision::Leave);
    assert_eq!(decide(&site(TailShape::Guard, false, 6), &p), TailDecision::Wrap);
    assert_eq!(decide(&site(TailShape::Wrapped, true, 2), &p), TailDecision::Guard);
}

#[test]
fn the_option_takes_off_on_one_or_two_thresholds() {
    let o = OptionTailForm;
    assert_eq!(o.apply("off").unwrap().0, None);
    assert_eq!(o.apply("on").unwrap().0, Some(TailFormPolicy::default()));
    let one = o.apply("3").unwrap().0.unwrap();
    assert_eq!((one.void_max, one.value_max), (3, 3));
    let two = o.apply("2,5").unwrap().0.unwrap();
    assert_eq!((two.void_max, two.value_max), (2, 5));
    assert!(o.apply("wide").is_err());
    assert!(o.apply("2,").is_err());
    assert_eq!(o.apply("wrap").unwrap().0.unwrap().rule, TailRule::Wrap);
    assert_eq!(o.apply("guard").unwrap().0.unwrap().rule, TailRule::Guard);
    assert_eq!(o.apply("model").unwrap().0.unwrap().rule, TailRule::Model);
    assert_eq!(o.apply("model").unwrap().0, Some(TailFormPolicy::shipped()));
    assert_eq!(TailFormPolicy::default().rule, TailRule::Threshold);
}

#[test]
fn forced_rules_wrap_or_guard_every_tail() {
    let wrap = TailFormPolicy { rule: TailRule::Wrap, ..Default::default() };
    let guard = TailFormPolicy { rule: TailRule::Guard, ..Default::default() };
    assert_eq!(decide(&site(TailShape::Guard, true, 40), &wrap), TailDecision::Wrap);
    assert_eq!(decide(&site(TailShape::Wrapped, false, 1), &guard), TailDecision::Guard);
}

fn feats(lines: u32, wrapped: bool) -> TailFeatures {
    use features::{CondKind, KKind, LastKind};
    TailFeatures {
        void: true,
        wrapped,
        lines,
        top: lines,
        stmts: lines,
        calls: 0,
        assigns: lines,
        rets: 0,
        ifs: 0,
        loops: 0,
        switches: 0,
        gotos: 0,
        last: LastKind::Assign,
        tail_call: false,
        only_calls: false,
        cond: CondKind::Truthy,
        cond_ops: 1,
        cond_call: false,
        null_param: false,
        k: KKind::Void,
        p_top: 2,
        p_stmts: 2,
        p_rets: 0,
        p_loops: 0,
        p_guards: 0,
        unopt: false,
    }
}

fn modeled(f: TailFeatures) -> TailSite {
    let shape = if f.wrapped { TailShape::Wrapped } else { TailShape::Guard };
    TailSite { shape, void: f.void, lines: f.lines as usize, feats: Some(f) }
}

#[test]
fn every_weight_names_a_feature() {
    use crate::p8_structure::kuna_tailform_weights as w;
    assert_eq!(w::NAMES.len(), w::WEIGHTS.len());
    let kinds: [(&str, &[&str]); 3] = [
        ("last", &["call", "assign", "ret", "if", "loop", "switch", "other"]),
        (
            "cond",
            &["truthy", "eqz", "eq", "ne", "rel", "and", "or", "bit", "not_bit", "call", "not_call", "other", "not_other"],
        ),
        ("k", &["void", "const", "var"]),
    ];
    for name in w::NAMES {
        let known = match name.split_once('=') {
            Some((k, v)) => kinds.iter().any(|(kk, vs)| *kk == k && vs.contains(&v)),
            None => features::NUMERIC.contains(name),
        };
        assert!(known, "weight table names an unknown feature `{name}`");
    }
}

/// Scores and choices of the shipped table, computed by `train.py`'s own
/// vectorisation (`sign(x)*ln(1+|x|)`, one-hot kinds) from the same weights.
#[test]
fn the_saved_weights_decide_these_sites() {
    use features::{CondKind, KKind, LastKind};
    let p = TailFormPolicy { rule: TailRule::Model, ..Default::default() };
    let a = TailFeatures { last: LastKind::Call, calls: 1, assigns: 0, ..feats(1, false) };
    let b = TailFeatures {
        top: 12,
        stmts: 12,
        assigns: 8,
        calls: 3,
        ifs: 1,
        last: LastKind::If,
        cond: CondKind::Eqz,
        p_top: 0,
        p_stmts: 0,
        ..feats(12, true)
    };
    let c = TailFeatures {
        void: false,
        k: KKind::Const,
        calls: 3,
        assigns: 3,
        p_top: 15,
        p_stmts: 20,
        cond: CondKind::Eq,
        last: LastKind::Call,
        ..feats(6, false)
    };
    let d = TailFeatures { ifs: 1, last: LastKind::If, cond: CondKind::Eqz, p_top: 0, p_stmts: 0, ..feats(3, false) };
    for (f, score, want) in [
        (a, 3.140789, TailDecision::Wrap),
        (b, -4.642252, TailDecision::Guard),
        (c, 5.762762, TailDecision::Wrap),
        (d, -2.751206, TailDecision::Leave),
    ] {
        assert!((features::model_score(&f) - score).abs() < 1e-5, "{f:?}: {}", features::model_score(&f));
        assert_eq!(decide(&modeled(f), &p), want, "{f:?}");
    }
}

#[test]
fn the_model_leaves_unoptimized_code_to_the_threshold() {
    let model = TailFormPolicy { rule: TailRule::Model, ..Default::default() };
    let rule = TailFormPolicy::default();
    for lines in 0..12 {
        for wrapped in [false, true] {
            let site = modeled(TailFeatures { unopt: true, ..feats(lines, wrapped) });
            assert_eq!(decide(&site, &model), decide(&site, &rule), "{lines} {wrapped}");
        }
    }
}

/// The model only chooses between the two rewrites the pass performs: every
/// choice it makes is one the threshold rule makes for some threshold.
#[test]
fn a_model_choice_is_one_some_threshold_makes() {
    use features::{CondKind, KKind, LastKind};
    let lasts = [LastKind::Call, LastKind::Assign, LastKind::Ret, LastKind::If, LastKind::Loop, LastKind::Other];
    let conds = [CondKind::Truthy, CondKind::Eqz, CondKind::Eq, CondKind::Ne, CondKind::Rel, CondKind::And, CondKind::Call];
    let model = TailFormPolicy { rule: TailRule::Model, ..Default::default() };
    let mut seed: u64 = 0x2545_f491_4f6c_dd1d;
    let mut next = |m: u64| {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (seed >> 33) % m
    };
    for _ in 0..4000 {
        let lines = next(40) as u32;
        let f = TailFeatures {
            void: next(2) == 0,
            k: [KKind::Void, KKind::Const, KKind::Var][next(3) as usize],
            calls: next(6) as u32,
            ifs: next(4) as u32,
            loops: next(3) as u32,
            rets: next(3) as u32,
            tail_call: next(2) == 0,
            null_param: next(2) == 0,
            cond_ops: next(8) as u32,
            p_top: next(30) as u32,
            p_stmts: next(60) as u32,
            p_guards: next(5) as u32,
            last: lasts[next(lasts.len() as u64) as usize],
            cond: conds[next(conds.len() as u64) as usize],
            ..feats(lines, next(2) == 0)
        };
        let site = modeled(f);
        let chosen = decide(&site, &model);
        let reachable = (0..=64).any(|t| {
            let rule = TailFormPolicy { void_max: t, value_max: t, ..Default::default() };
            decide(&site, &rule) == chosen
        });
        assert!(reachable, "{site:?} -> {chosen:?}");
    }
}

//! The kuna-only fixes: the gate, the compat switch, and the two-pass kuna-fixes corpus cases.

use std::collections::BTreeSet;
use std::path::Path;

use kuna_jvm::basic_block_statement::BasicBlockStatement;
use kuna_jvm::const_exprent::ConstExprent;
use kuna_jvm::decompiler_context::DecompilerContext;
use kuna_jvm::dummy_exit_statement::DummyExitStatement;
use kuna_jvm::fernflower::{Fernflower, KUNA_ERROR_MESSAGE};
use kuna_jvm::generic_type::GenericType;
use kuna_jvm::i_fernflower_logger::IFernflowerLogger;
use kuna_jvm::i_fernflower_preferences::IFernflowerPreferences;
use kuna_jvm::if_exprent::{IfExprent, Type as IfType};
use kuna_jvm::if_helper::IfHelper;
use kuna_jvm::if_statement::IfStatement;
use kuna_jvm::kuna_corpus::{
    count_failed_methods, load_cases, reset_engine_thread, run_case, with_engine_stack, CaseStatus,
};
use kuna_jvm::kuna_fixes::{self, enabled, take_switches, KunaFix, KunaFixes, VINEFLOWER_COMPAT};
use kuna_jvm::kuna_options::{resolve, rows};
use kuna_jvm::list_stack::ListStack;
use kuna_jvm::root_statement::RootStatement;
use kuna_jvm::rt::{reset_class_scope, JHashSet};
use kuna_jvm::sequence_statement::SequenceStatement;
use kuna_jvm::stat_edge::StatEdge;
use kuna_jvm::statement::Statement;

use crate::k2_dom_helper::{fixture_method, oracle_context, start_method};
use kuna_jvm::rt::{try_catch, JHashMap};

fn pairs(list: &[(&'static str, &'static str)]) -> Vec<(String, String)> {
    list.iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
}

#[test]
fn the_default_turns_every_fix_on() {
    let fixes = KunaFixes::from_options([]);
    assert_eq!(fixes, KunaFixes::DEFAULT);
    assert!(KunaFix::ALL.iter().all(|f| fixes.contains(*f)));
    assert_eq!(fixes.list(), KunaFix::ALL);
    let compat_off = KunaFixes::from_options([(VINEFLOWER_COMPAT, "off")]);
    assert_eq!(compat_off.list(), KunaFix::ALL);
}

#[test]
fn compat_turns_every_fix_off() {
    for value in ["1", "on", "true", "ON"] {
        let fixes = KunaFixes::from_options([(VINEFLOWER_COMPAT, value)]);
        assert_eq!(fixes, KunaFixes::COMPAT, "{value}");
        assert!(KunaFix::ALL.iter().all(|f| !fixes.contains(*f)));
    }
    let compat_later_off =
        KunaFixes::from_options([(VINEFLOWER_COMPAT, "1"), (VINEFLOWER_COMPAT, "0")]);
    assert_eq!(compat_later_off, KunaFixes::DEFAULT);
    let compat_later_on =
        KunaFixes::from_options([(VINEFLOWER_COMPAT, "0"), (VINEFLOWER_COMPAT, "on")]);
    assert_eq!(compat_later_on, KunaFixes::COMPAT);
}

#[test]
fn every_fix_has_its_own_id() {
    let ids: BTreeSet<&str> = KunaFix::ALL.iter().map(|f| f.id()).collect();
    assert_eq!(ids.len(), KunaFix::ALL.len());
    for fix in KunaFix::ALL {
        assert_eq!(KunaFix::from_id(fix.id()), Some(*fix));
        assert!(!fix.id().starts_with("jvm_"), "{}", fix.id());
        let without = KunaFixes::DEFAULT.without(*fix);
        assert!(!without.contains(*fix));
        assert_eq!(without.list().len(), KunaFix::ALL.len() - 1);
    }
}

#[test]
fn only_the_compat_switch_leaves_the_property_list() {
    let (fixes, rest) = take_switches(&pairs(&[
        ("log-level", "warn"),
        (VINEFLOWER_COMPAT, "1"),
        ("jvm_fix_lambda_wildcard_return", "off"),
        ("decompile-generics", "1"),
    ]));
    assert_eq!(fixes, KunaFixes::COMPAT);
    assert_eq!(
        rest,
        pairs(&[
            ("log-level", "warn"),
            ("jvm_fix_lambda_wildcard_return", "off"),
            ("decompile-generics", "1"),
        ])
    );
    let (fixes, _) = take_switches(&pairs(&[("jvm_fix_lambda_wildcard_return", "off")]));
    assert_eq!(fixes, KunaFixes::DEFAULT);
}

/// The fixes and `error-message` of the context `Fernflower::new` installs for `options`, or
/// `Fernflower::new_with_fixes` for `options` and `fixes`.
fn engine_state(
    options: &[(&'static str, &'static str)],
    fixes: Option<KunaFixes>,
) -> (KunaFixes, String, bool) {
    let options = pairs(options);
    with_engine_stack(move || {
        reset_engine_thread();
        let logger = <dyn IFernflowerLogger>::NO_OP();
        let mut fernflower = match fixes {
            Some(fixes) => Fernflower::new_with_fixes(&options, logger, fixes),
            None => Fernflower::new(&options, logger),
        };
        let active = kuna_fixes::active();
        let context = DecompilerContext::get_current_context().unwrap();
        assert_eq!(context.with(|d| d.kuna_fixes), active);
        let message = DecompilerContext::get_property(IFernflowerPreferences::ERROR_MESSAGE).unwrap();
        let leaked = context.with(|d| {
            d.properties
                .entry_vec()
                .iter()
                .any(|(k, _)| k == VINEFLOWER_COMPAT)
        });
        fernflower.clear_context();
        reset_engine_thread();
        assert_eq!(kuna_fixes::active(), KunaFixes::DEFAULT);
        (active, message, leaked)
    })
}

#[test]
fn fernflower_installs_the_set_the_gate_answers_from() {
    let vineflower_message = IFernflowerPreferences::DEFAULTS()
        .get_cloned(&IFernflowerPreferences::ERROR_MESSAGE.to_string())
        .unwrap();

    let (fixes, message, leaked) = engine_state(&[], None);
    assert_eq!(fixes, KunaFixes::DEFAULT);
    assert_eq!(message, KUNA_ERROR_MESSAGE);
    assert!(!leaked, "the switches stay out of Vineflower's option map");

    let (fixes, message, leaked) = engine_state(&[(VINEFLOWER_COMPAT, "1")], None);
    assert_eq!(fixes, KunaFixes::COMPAT);
    assert_eq!(message, vineflower_message);
    assert!(!leaked);

    let without_note = KunaFixes::DEFAULT.without(KunaFix::ReportToKuna);
    let (fixes, message, leaked) =
        engine_state(&[(VINEFLOWER_COMPAT, "1")], Some(without_note));
    assert_eq!(fixes, without_note);
    assert!(fixes.contains(KunaFix::LambdaWildcardReturn));
    assert_eq!(message, vineflower_message);
    assert!(!leaked);

    let (fixes, message, _) = engine_state(&[("jvm_fix_report_to_kuna", "0")], None);
    assert_eq!(fixes, KunaFixes::DEFAULT, "a jvm_fix_ key is no switch");
    assert_eq!(message, KUNA_ERROR_MESSAGE);

    let (_, message, _) = engine_state(&[("error-message", "see the docs")], None);
    assert_eq!(message, "see the docs");
    let (_, message, _) = engine_state(&[("erm", "")], None);
    assert_eq!(message, "");
}

#[test]
fn the_gate_reads_the_installed_context() {
    with_engine_stack(|| {
        reset_engine_thread();
        let options = pairs(&[(VINEFLOWER_COMPAT, "on")]);
        let mut fernflower = Fernflower::new(&options, <dyn IFernflowerLogger>::NO_OP());
        assert!(!enabled(KunaFix::LambdaWildcardReturn));
        assert!(!enabled(KunaFix::ReportToKuna));
        let context = DecompilerContext::get_current_context();
        DecompilerContext::set_current_context(None);
        assert!(enabled(KunaFix::LambdaWildcardReturn));
        DecompilerContext::set_current_context(context);
        assert!(!enabled(KunaFix::LambdaWildcardReturn));
        fernflower.clear_context();
        reset_engine_thread();
    });
}

#[test]
fn the_compat_row_is_the_only_fix_switch() {
    assert!(rows().iter().all(|r| !r.option.starts_with("jvm_fix_")));
    let compat = rows().iter().find(|r| r.option == VINEFLOWER_COMPAT).unwrap();
    assert!(compat.is_kuna());
    assert_eq!((compat.values, compat.default), ("on|off", "off"));
    assert_eq!(compat.change_kind, "kuna-compat");
    assert_eq!(
        resolve("jvm_vineflower_compat", "on").unwrap(),
        (VINEFLOWER_COMPAT, "1".to_string())
    );
    for fix in KunaFix::ALL {
        let name = format!("jvm_fix_{}", fix.id());
        let err = resolve(&name, "off").unwrap_err();
        assert!(err.starts_with(&format!("unknown JVM option {name}")), "{err}");
    }
}

#[test]
fn every_kuna_fixes_case_matches_in_both_modes() {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../tests/jvm/cases.tsv");
    let cases: Vec<_> = load_cases(&manifest)
        .unwrap()
        .into_iter()
        .filter(|c| c.set_slug.starts_with("kuna-fixes"))
        .collect();
    assert!(!cases.is_empty());
    let results = with_engine_stack(move || {
        cases
            .iter()
            .map(|case| (case.id.clone(), run_case(case)))
            .collect::<Vec<_>>()
    });
    for (id, result) in &results {
        assert_eq!(result.status, CaseStatus::Match, "{id}: {:?}", result.first_diff);
    }
    for (id, result) in results.iter().filter(|(id, _)| id.starts_with("kuna-fixes/")) {
        let compat_id = id.replacen("kuna-fixes/", "kuna-fixes-compat/", 1);
        let (_, compat) = results.iter().find(|(i, _)| *i == compat_id).unwrap();
        let fixed = count_failed_methods(result.actual.as_deref().unwrap());
        let failed = count_failed_methods(compat.actual.as_deref().unwrap());
        assert!(failed > 0, "{compat_id}: Vineflower's text has no failure comment");
        assert!(fixed < failed, "{id}: the fix decompiles no method Vineflower fails on");
    }
}

/// `if (a) { if (b) throw; if (c) return; else e(); } z; return;` as IfHelper sees it after
/// `ExprProcessor`: the else branch `e` falls through to `z`, so `ifElseChainDenesting` must not
/// move `z` into the then-branch (fix `if_denest_fallthrough`); Vineflower does, and `e`'s
/// break then targets a statement inside the if it is no longer in.
fn denest_shape(fixes: KunaFixes) -> (bool, Statement, Statement, Statement, Statement) {
    reset_class_scope();
    start_method(fixture_method());
    kuna_fixes::activate(fixes);
    let bb = || BasicBlockStatement::create().0;
    let edge = |t: i32, s: Statement, d: Statement, c: Option<Statement>| {
        let e = StatEdge::new_closure(t, s, d, c);
        s.add_successor(e);
    };
    let cond = || {
        let mut stack = ListStack::new();
        stack.push(ConstExprent::new_int(1, true, None).0);
        IfExprent::new(IfType::Value, &mut stack, None).0
    };
    let if_then = |head: Statement, body: Statement| {
        let ifs = IfStatement::new(head, 0, None);
        ifs.set_ifstat(Some(body));
        ifs.get_stats().borrow_mut().add_with_key(body, body.id());
        ifs.set_all_parent();
        ifs.get_headexprent_list().add(cond());
        ifs
    };
    let (h0, h1, h2, t, r, e, z) = (bb(), bb(), bb(), bb(), bb(), bb(), bb());
    let exit = DummyExitStatement::new();

    edge(StatEdge::TYPE_REGULAR, h2, e, None);
    edge(StatEdge::TYPE_REGULAR, h2, r, None);
    let join = bb();
    edge(StatEdge::TYPE_REGULAR, r, join, None);
    edge(StatEdge::TYPE_REGULAR, e, join, None);
    let c = IfStatement::new(h2, 2, None);
    for branch in [r, e] {
        branch.remove_successor(branch.get_all_successor_edges()[0]);
    }
    c.set_all_parent();
    c.get_headexprent_list().add(cond());
    edge(StatEdge::TYPE_REGULAR, h1, c.0, None);
    edge(StatEdge::TYPE_REGULAR, h1, t, None);
    let nested = if_then(h1, t);
    let body = SequenceStatement::new(vec![nested.0, c.0]);
    body.set_all_parent();
    edge(StatEdge::TYPE_REGULAR, h0, z, None);
    edge(StatEdge::TYPE_REGULAR, h0, body.0, None);
    let outer = if_then(h0, body.0);
    let seq = SequenceStatement::new(vec![outer.0, z]);
    seq.set_all_parent();
    for (source, destination) in [(t, exit.0), (r, exit.0), (z, exit.0)] {
        edge(StatEdge::TYPE_BREAK, source, destination, Some(seq.0));
    }
    for source in [e, c.0, body.0] {
        edge(StatEdge::TYPE_BREAK, source, z, Some(outer.0));
    }
    let root = RootStatement::new(seq.0, exit, fixture_method());
    let changed = IfHelper::merge_ifs(seq.0, &mut JHashSet::new());
    kuna_fixes::activate(KunaFixes::DEFAULT);
    assert_eq!(root.get_first(), Some(seq.0));
    (changed, outer.0, body.0, z, e)
}

#[test]
fn if_denest_leaves_an_if_whose_body_reaches_the_moved_code() {
    let _context = oracle_context();

    let (changed, outer, _, z, e) = denest_shape(KunaFixes::COMPAT);
    assert!(changed, "Vineflower swaps the branches");
    assert_eq!(z.get_parent(), Some(outer), "z moved into the if");
    assert!(!outer.contains_statement_strict(e));
    assert_eq!(
        e.get_all_successor_edges()[0].get_destination(),
        z,
        "e's break now enters the if"
    );

    let (changed, outer, body, z, e) = denest_shape(KunaFixes::DEFAULT);
    assert!(!changed);
    assert_eq!(outer.as_if().get_ifstat(), Some(body));
    assert_eq!(z.get_parent(), outer.get_parent());
    assert!(outer.contains_statement_strict(e));
    assert_eq!(e.get_all_successor_edges()[0].get_destination(), z);
}

mod catch_loop_back_successor {
    use kuna_jvm::basic_block_statement::BasicBlockStatement;
    use kuna_jvm::catch_statement::CatchStatement;
    use kuna_jvm::general_statement::GeneralStatement;
    use kuna_jvm::kuna_fixes::{self, KunaFixes};
    use kuna_jvm::rt::{reset_class_scope, try_catch};
    use kuna_jvm::stat_edge::StatEdge;
    use kuna_jvm::statement::Statement;
    use kuna_jvm::struct_method::StructMethod;

    use crate::k2_dom_helper::{fixture_method, oracle_context, start_method};

    fn bb() -> Statement {
        BasicBlockStatement::create().0
    }

    fn regular(source: Statement, destination: Statement) {
        source.add_successor(StatEdge::new(StatEdge::TYPE_REGULAR, source, destination));
    }

    fn catches(head: Statement, handler: Statement, exception: &str) {
        head.add_successor(StatEdge::new_exceptions(
            head,
            handler,
            Some(vec![exception.to_string()]),
        ));
    }

    /// The region of a retry loop after other code as `DomHelper` first sees it:
    /// `pre -> head`, `head` leaves by a break, and every handler goes back to `head`.
    fn retry_loop(mt: StructMethod, handlers: usize) -> (GeneralStatement, Statement) {
        reset_class_scope();
        start_method(mt);
        let (pre, head, exit) = (bb(), bb(), bb());
        let handlers: Vec<Statement> = (0..handlers).map(|_| bb()).collect();
        let mut nodes = vec![pre, head];
        nodes.extend(&handlers);
        let general = GeneralStatement::new(pre, nodes, None);
        regular(pre, head);
        let out = StatEdge::new_closure(StatEdge::TYPE_BREAK, head, exit, Some(general.0));
        head.add_successor(out);
        for (i, h) in handlers.iter().enumerate() {
            catches(head, *h, &format!("pkg/E{i}"));
            regular(*h, head);
        }
        general.set_all_parent();
        (general, head)
    }

    #[test]
    fn a_catch_is_not_built_with_its_own_try_block_as_successor() {
        let _context = oracle_context();
        let mt = fixture_method();

        kuna_fixes::activate(KunaFixes::COMPAT);
        let (general, head) = retry_loop(mt, 2);
        let catch = CatchStatement::is_head(head).expect("Vineflower builds the try/catch");
        assert_eq!(catch.get_post(), Some(head));
        general.collapse_nodes_to_statement(catch);
        let out = catch.get_successor_edges(StatEdge::TYPE_REGULAR);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].get_destination(), head, "an edge to its own child");
        let caught = try_catch(|| general.get_post_reverse_post_order_list()).unwrap_err();
        assert!(
            caught
                .message
                .contains("computing post reverse post order failed!"),
            "{}",
            caught.message
        );

        for fixes in [KunaFixes::DEFAULT, KunaFixes::COMPAT] {
            kuna_fixes::activate(fixes);
            for handlers in [2, 3] {
                let (_, head) = retry_loop(mt, handlers);
                let built = CatchStatement::is_head(head).is_some();
                assert_eq!(built, fixes == KunaFixes::COMPAT, "{handlers} handlers");
            }
            let (_, head) = retry_loop(mt, 1);
            assert!(
                CatchStatement::is_head(head).is_none(),
                "one handler: built later"
            );

            reset_class_scope();
            start_method(mt);
            let (head, h1, h2, next) = (bb(), bb(), bb(), bb());
            GeneralStatement::new(head, vec![head, h1, h2, next], None).set_all_parent();
            regular(head, next);
            for (i, h) in [h1, h2].into_iter().enumerate() {
                catches(head, h, &format!("pkg/E{i}"));
                regular(h, next);
            }
            let catch = CatchStatement::is_head(head).expect("a try/catch followed by next");
            assert_eq!(catch.get_post(), Some(next));
        }
        kuna_fixes::activate(KunaFixes::DEFAULT);
    }
}
#[test]
fn an_enclosing_class_without_a_signature_maps_as_its_empty_generic_type() {
    let classes =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../tests/jvm/kuna-fixes/classes/pkg");
    with_engine_stack(move || {
        let guard = crate::common::install_default_context();
        for name in [
            "TestInnerOfGenericOuter",
            "TestInnerOfGenericOuter$Itrs",
            "TestInnerOfGenericOuter$Itrs$Node",
        ] {
            guard
                .struct_context()
                .add_space_file(&classes.join(format!("{name}.class")), true);
        }
        let itrs = guard
            .struct_context()
            .get_class("pkg/TestInnerOfGenericOuter$Itrs");
        assert!(itrs.unwrap().get_signature().is_none());
        let node = guard
            .struct_context()
            .get_class("pkg/TestInnerOfGenericOuter$Itrs$Node");
        let decl = node.unwrap().get_signature().unwrap().generic_type();
        let use_ =
            GenericType::parse("Lpkg/TestInnerOfGenericOuter<Ljava/lang/String;>.Itrs.Node;");

        let mut map = JHashMap::new();
        decl.map_gen_vars_to(&use_.as_generic(), &mut map);
        let mapped: Vec<(String, String)> = map
            .entry_vec()
            .iter()
            .map(|(k, v)| (k.to_string_base(), v.as_ref().unwrap().to_string_base()))
            .collect();
        assert_eq!(mapped, [("E".to_string(), "Ljava/lang/String;".to_string())]);

        kuna_fixes::activate(KunaFixes::COMPAT);
        let compat = try_catch(|| decl.map_gen_vars_to(&use_.as_generic(), &mut JHashMap::new()));
        kuna_fixes::activate(KunaFixes::DEFAULT);
        assert!(compat.unwrap_err().message.contains("NullPointerException"));
    });
}

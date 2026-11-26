//! kuna's fixes on hand-built input, and the engine's defaults they set.

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
use kuna_jvm::kuna_corpus::{reset_engine_thread, with_engine_stack};
use kuna_jvm::list_stack::ListStack;
use kuna_jvm::root_statement::RootStatement;
use kuna_jvm::rt::{reset_class_scope, JHashMap, JHashSet};
use kuna_jvm::sequence_statement::SequenceStatement;
use kuna_jvm::stat_edge::StatEdge;
use kuna_jvm::statement::Statement;

use crate::common::{fixture_method, oracle_context, start_method};

fn pairs(list: &[(&'static str, &'static str)]) -> Vec<(String, String)> {
    list.iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
}

/// The `error-message` of the context `Fernflower::new` installs for `options`.
fn error_message(options: &[(&'static str, &'static str)]) -> String {
    let options = pairs(options);
    with_engine_stack(move || {
        reset_engine_thread();
        let mut fernflower = Fernflower::new(&options, <dyn IFernflowerLogger>::NO_OP());
        let message = DecompilerContext::get_property(IFernflowerPreferences::ERROR_MESSAGE).unwrap();
        fernflower.clear_context();
        reset_engine_thread();
        message
    })
}

#[test]
fn failure_comments_name_kuna_unless_the_caller_sets_the_message() {
    assert_eq!(error_message(&[]), KUNA_ERROR_MESSAGE);
    assert_eq!(error_message(&[("error-message", "see the docs")]), "see the docs");
    assert_eq!(error_message(&[("erm", "")]), "");
}

/// `if (a) { if (b) throw; if (c) return; else e(); } z; return;` as IfHelper sees it after
/// `ExprProcessor`: the else branch `e` falls through to `z`, so `ifElseChainDenesting` must not
/// move `z` into the then-branch; Vineflower does, and `e`'s break then targets a statement
/// inside the if it is no longer in.
fn denest_shape() -> (bool, Statement, Statement, Statement, Statement) {
    reset_class_scope();
    start_method(fixture_method());
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
    assert_eq!(root.get_first(), Some(seq.0));
    (changed, outer.0, body.0, z, e)
}

#[test]
fn if_denest_leaves_an_if_whose_body_reaches_the_moved_code() {
    let _context = oracle_context();

    let (changed, outer, body, z, e) = denest_shape();
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
    use kuna_jvm::rt::reset_class_scope;
    use kuna_jvm::stat_edge::StatEdge;
    use kuna_jvm::statement::Statement;
    use kuna_jvm::struct_method::StructMethod;

    use crate::common::{fixture_method, oracle_context, start_method};

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

        for handlers in [2, 3] {
            let (_, head) = retry_loop(mt, handlers);
            assert!(CatchStatement::is_head(head).is_none(), "{handlers} handlers");
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
    });
}
#[test]
fn statement_only_exprents_are_kept_out_of_expression_positions() {
    use kuna_jvm::assert_exprent::AssertExprent;
    use kuna_jvm::assignment_exprent::AssignmentExprent;
    use kuna_jvm::const_exprent::ConstExprent;
    use kuna_jvm::exit_exprent::{self, ExitExprent};
    use kuna_jvm::testing::{
        is_lambda_expression_body, is_statement_only,
    };
    use kuna_jvm::monitor_exprent::{self, MonitorExprent};
    use kuna_jvm::var_exprent::VarExprent;
    use kuna_jvm::var_type::VarType;
    use kuna_jvm::yield_exprent::YieldExprent;

    let _g = crate::common::install_default_context();
    let int = || Some(VarType::VARTYPE_INT());
    let one = || ConstExprent::new_int(1, false, None).0;
    let var = || VarExprent::new(1, int(), None).0;
    let exit = |t, v: Option<_>, r| ExitExprent::new(t, v, Some(r), None, None).0;

    let assert_ = AssertExprent::new(vec![None, Some(one())]).0;
    let ret_value = exit(exit_exprent::Type::Return, Some(one()), VarType::VARTYPE_INT());
    let ret_void = exit(exit_exprent::Type::Return, None, VarType::VARTYPE_VOID());
    let throw = exit(exit_exprent::Type::Throw, Some(var()), VarType::VARTYPE_VOID());
    let yield_ = YieldExprent::new(one(), VarType::VARTYPE_INT()).0;
    let monitor = MonitorExprent::new(monitor_exprent::Type::Enter, var(), None).0;
    let assign = AssignmentExprent::new(var(), one(), None).0;
    let declared = VarExprent::new(2, int(), None);
    declared.set_definition(true);
    let declaration = AssignmentExprent::new(declared.0, one(), None).0;
    let bare = VarExprent::new(3, int(), None);
    bare.set_definition(true);

    for e in [assert_, ret_value, ret_void, throw, yield_, monitor, declaration, bare.0] {
        assert!(is_statement_only(e), "{}", e.to_string());
    }
    for e in [assign, var(), one()] {
        assert!(!is_statement_only(e), "{}", e.to_string());
        assert!(is_lambda_expression_body(e), "{}", e.to_string());
    }
    assert!(is_lambda_expression_body(ret_value), "return 1 is written as 1");
    for e in [assert_, ret_void, throw, yield_, monitor, declaration, bare.0] {
        assert!(!is_lambda_expression_body(e), "{}", e.to_string());
    }
}

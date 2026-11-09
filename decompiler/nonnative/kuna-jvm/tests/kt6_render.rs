//! The Kotlin statement classes (`KIfStatement`, `KDoStatement`, `KSwitchStatement`,
//! `KSequenceStatement`, `KCatchStatement`): copies of small Java statement trees, made directly
//! or by `ReplaceStatsPass`, rendered against Vineflower 1.12.0 with its Kotlin plugin
//! (`data/kt6/KT6Oracle.java` -> `data/kt6/render_golden.txt`, `data/kt6/gen-render.sh`). Each
//! scenario mirrors the Java driver statement by statement, so statement ids line up.
//!
//! Some renders go through other slices' bodies (`KExprProcessor.jmpWrapper`,
//! `KUtils.replaceExprent`, the `KVarExprent`/`KConstExprent`/`KFunctionExprent` classes,
//! `KotlinWriter.toValidKotlinIdentifier`); while one of those is still `unported!` the scenario
//! is reported as pending instead of compared.

use kuna_jvm::assert_exprent::AssertExprent;
use kuna_jvm::assignment_exprent::AssignmentExprent;
use kuna_jvm::basic_block_statement::BasicBlockStatement;
use kuna_jvm::catch_statement::CatchStatement;
use kuna_jvm::const_exprent::{ConstExprent, ConstValue};
use kuna_jvm::do_statement::{DoStatement, Type as LoopType};
use kuna_jvm::exprent::Exprent;
use kuna_jvm::function_exprent::{FunctionExprent, FunctionType};
use kuna_jvm::general_statement::GeneralStatement;
use kuna_jvm::if_exprent::{IfExprent, Type as IfType};
use kuna_jvm::if_statement::IfStatement;
use kuna_jvm::k_catch_statement::KCatchStatement;
use kuna_jvm::k_do_statement::KDoStatement;
use kuna_jvm::k_if_statement::KIfStatement;
use kuna_jvm::k_sequence_statement::KSequenceStatement;
use kuna_jvm::k_switch_statement::KSwitchStatement;
use kuna_jvm::list_stack::ListStack;
use kuna_jvm::monitor_exprent::{MonitorExprent, Type as MonitorType};
use kuna_jvm::replace_stats_pass::ReplaceStatsPass;
use kuna_jvm::rt::{try_catch, JList};
use kuna_jvm::sequence_statement::SequenceStatement;
use kuna_jvm::stat_edge::StatEdge;
use kuna_jvm::statement::Statement;
use kuna_jvm::struct_method::StructMethod;
use kuna_jvm::switch_head_exprent::SwitchHeadExprent;
use kuna_jvm::switch_statement::SwitchStatement;
use kuna_jvm::text_buffer::TextBuffer;
use kuna_jvm::var_exprent::VarExprent;
use kuna_jvm::var_type::VarType;

use crate::common;
use crate::k2_dom_helper::{fixture_method, start_method};

const GOLDEN: &str = include_str!("data/kt6/render_golden.txt");

/// Bodies of other slices a render may still reach unported.
const OTHER_SLICES: [&str; 7] = [
    "KExprProcessor.jmpWrapper",
    "KUtils.replaceExprent",
    "KVarExprent.",
    "KConstExprent.",
    "KFunctionExprent.",
    "KotlinWriter.",
    "KTypes.",
];

const REG: i32 = StatEdge::TYPE_REGULAR;
const BREAK: i32 = StatEdge::TYPE_BREAK;
const CONTINUE: i32 = StatEdge::TYPE_CONTINUE;

fn af() -> Exprent {
    AssertExprent::new(vec![None]).0
}

fn mexit() -> Exprent {
    MonitorExprent::new(MonitorType::Exit, af(), None).0
}

fn if_expr(condition: Exprent) -> Exprent {
    let mut stack = ListStack::new();
    stack.push(condition);
    IfExprent::new(IfType::Value, &mut stack, None).0
}

fn int_const(value: i32) -> Exprent {
    ConstExprent::new_int(value, false, None).0
}

fn var(index: i32, type_: VarType) -> VarExprent {
    VarExprent::new(index, Some(type_), None)
}

fn func(type_: FunctionType, operands: Vec<Exprent>) -> Exprent {
    FunctionExprent::new(type_, operands, None).0
}

fn bb() -> Statement {
    BasicBlockStatement::create().0
}

fn add(stat: Statement, exprent: Exprent) {
    stat.get_exprents()
        .expect("basic block exprents")
        .add(exprent);
}

fn edge(type_: i32, source: Statement, destination: Statement) -> StatEdge {
    let e = StatEdge::new(type_, source, destination);
    source.add_successor(e);
    e
}

fn edge_c(
    type_: i32,
    source: Statement,
    destination: Statement,
    closure: Option<Statement>,
) -> StatEdge {
    let e = StatEdge::new_closure(type_, source, destination, closure);
    source.add_successor(e);
    e
}

fn parents(stat: Statement) {
    let stats = stat.get_stats().borrow().to_vec();
    for st in stats {
        st.set_parent(Some(stat));
        parents(st);
    }
}

enum Outcome {
    Text(String),
    Pending(String),
}

struct Oracle {
    out: Vec<(String, Outcome)>,
    mt: StructMethod,
}

impl Oracle {
    fn reset(&self) {
        start_method(self.mt);
    }

    fn out(&mut self, name: &str, render: impl FnOnce() -> TextBuffer) {
        let outcome = match try_catch(render) {
            Ok(buf) => Outcome::Text(buf.convert_to_string_and_allow_data_discard()),
            Err(caught) if caught.is("UnportedError") => {
                let body = caught
                    .exception
                    .as_ref()
                    .map_or(String::new(), |e| e.message.clone());
                assert!(
                    OTHER_SLICES.iter().any(|other| body.starts_with(other)),
                    "{name}: a KT6 body is unported: {body}"
                );
                Outcome::Pending(body)
            }
            Err(caught) => Outcome::Text(format!("!! {}", exception_name(&caught.message))),
        };
        self.out.push((name.to_string(), outcome));
    }

    fn for_loop(
        &mut self,
        name: &str,
        type_: VarType,
        init: Exprent,
        condition: Exprent,
        inc: FunctionType,
    ) {
        self.reset();
        let d = loop_(LoopType::For);
        d.set_init_exprent(AssignmentExprent::new(var(1, type_.clone()).0, init, None).0);
        d.set_condition_exprent(condition);
        d.set_inc_exprent(func(inc, vec![var(1, type_).0]));
        self.out(name, || KDoStatement::new(d).0 .0.to_java(1));
    }
}

/// The Java exception class a caught Rust failure stands for.
fn exception_name(message: &str) -> String {
    let head = message.split(':').next().unwrap_or("").trim();
    let simple = head.rsplit('.').next().unwrap_or(head);
    if simple.ends_with("Exception") || simple.ends_with("Error") {
        simple.to_string()
    } else {
        format!("panic {message}")
    }
}

fn loop_(type_: LoopType) -> DoStatement {
    let head = bb();
    add(head, af());
    let d = DoStatement::new(head);
    d.set_looptype(type_);
    d
}

fn loop_name(type_: LoopType) -> &'static str {
    match type_ {
        LoopType::Infinite => "infinite",
        LoopType::DoWhile => "do_while",
        LoopType::While => "while",
        LoopType::For => "for",
        LoopType::ForEach => "for_each",
    }
}

fn render_all() -> Vec<(String, Outcome)> {
    let mut o = Oracle {
        out: Vec::new(),
        mt: fixture_method(),
    };

    o.reset();
    {
        let (h, t, out) = (bb(), bb(), bb());
        add(h, af());
        let general = GeneralStatement::new(h, vec![h, t], None);
        edge(REG, h, t);
        edge_c(BREAK, h, out, Some(general.0));
        let ifs = IfStatement::new(h, 1, None);
        ifs.get_headexprent_list().add(if_expr(af()));
        general.collapse_nodes_to_statement(ifs.0);
        o.out("kif_break", || KIfStatement::new(ifs).0 .0.to_java(0));
    }

    o.reset();
    {
        let (h, t, dest) = (bb(), bb(), bb());
        add(h, mexit());
        edge(REG, h, t);
        let c = edge_c(CONTINUE, h, dest, None);
        c.with_mut(|d| d.labeled = false);
        let ifs = IfStatement::new(h, 1, None);
        ifs.get_headexprent_list().add(if_expr(af()));
        let kif = KIfStatement::new(ifs);
        o.out("kif_continue", || kif.0 .0.to_java(1));
        c.with_mut(|d| d.explicit = false);
        o.out("kif_implicit", || kif.0 .0.to_java(1));
    }

    o.reset();
    {
        let (h, t, out, a) = (bb(), bb(), bb(), bb());
        add(a, af());
        let kseq = KSequenceStatement::new(SequenceStatement::new(vec![a]));
        edge(REG, h, t);
        let b = edge_c(BREAK, h, out, Some(kseq.0 .0));
        b.with_mut(|d| d.labeled = false);
        let ifs = IfStatement::new(h, 1, None);
        ifs.get_headexprent_list().add(if_expr(mexit()));
        ifs.add_labeled_edge(edge_c(BREAK, t, out, None));
        let kif = KIfStatement::new(ifs);
        o.out("kif_return_run", || kif.0 .0.to_java(0));
        b.with_mut(|d| d.labeled = true);
        o.out("kif_return_label", || kif.0 .0.to_java(0));
    }

    o.reset();
    {
        let (h, i, e, p) = (bb(), bb(), bb(), bb());
        add(h, af());
        add(i, af());
        add(e, mexit());
        edge(REG, h, e);
        edge(REG, h, i);
        edge(REG, i, p);
        edge(REG, e, p);
        let ifs = IfStatement::new(h, 2, Some(p));
        ifs.get_headexprent_list().add(if_expr(af()));
        o.out("kif_else", || KIfStatement::new(ifs).0 .0.to_java(1));
    }

    o.reset();
    {
        let (h, i, h2, x, out2, p) = (bb(), bb(), bb(), bb(), bb(), bb());
        add(h, af());
        add(i, mexit());
        add(x, af());
        edge(REG, h2, x);
        let brk = edge_c(BREAK, h2, out2, None);
        brk.with_mut(|d| d.labeled = false);
        let inner = IfStatement::new(h2, 1, None);
        inner.get_headexprent_list().add(if_expr(af()));
        edge(REG, h, inner.0);
        edge(REG, h, i);
        edge(REG, i, p);
        let ip = edge(REG, inner.0, p);
        ip.with_mut(|d| d.explicit = false);
        let outer = IfStatement::new(h, 2, Some(p));
        outer.get_headexprent_list().add(if_expr(af()));
        let holder = SequenceStatement::new(vec![outer.0]);
        parents(holder.0);
        ReplaceStatsPass::replace(holder.0);
        o.out("kif_else_if", || {
            holder.get_first().expect("first").to_java(0)
        });
    }

    o.reset();
    {
        let (a, b, c, out) = (bb(), bb(), bb(), bb());
        add(a, af());
        add(b, mexit());
        let seq = SequenceStatement::new(vec![a, c, b]);
        edge_c(BREAK, b, out, Some(seq.0));
        let plain = KSequenceStatement::new(seq);
        o.out("kseq", || plain.0 .0.to_java(1));
        seq.add_labeled_edge(edge_c(BREAK, c, out, None));
        let labeled = KSequenceStatement::new(seq);
        o.out("kseq_labeled", || labeled.0 .0.to_java(1));
    }

    for type_ in [
        LoopType::Infinite,
        LoopType::DoWhile,
        LoopType::While,
        LoopType::For,
        LoopType::ForEach,
    ] {
        o.reset();
        let d = loop_(type_);
        let head = d.get_first().expect("head");
        match type_ {
            LoopType::Infinite => {
                edge_c(CONTINUE, head, d.0, Some(d.0));
            }
            LoopType::DoWhile | LoopType::While => d.set_condition_exprent(af()),
            LoopType::For => {
                d.set_init_exprent(mexit());
                d.set_condition_exprent(af());
                d.set_inc_exprent(mexit());
            }
            LoopType::ForEach => {
                d.set_init_exprent(var(3, VarType::VARTYPE_INT()).0);
                d.set_inc_exprent(af());
            }
        }
        let name = loop_name(type_);
        o.out(&format!("kdo_{name}"), || {
            KDoStatement::new(d).0 .0.to_java(1)
        });
        d.add_labeled_edge(edge_c(BREAK, head, bb(), None));
        o.out(&format!("kdo_{name}_labeled"), || {
            KDoStatement::new(d).0 .0.to_java(1)
        });
    }

    o.reset();
    {
        let d = loop_(LoopType::For);
        d.set_condition_exprent(af());
        d.set_inc_exprent(mexit());
        o.out("kdo_for_no_init", || KDoStatement::new(d).0 .0.to_java(0));
    }

    let int = VarType::VARTYPE_INT;
    let long = VarType::VARTYPE_LONG;
    let long_const = |v: i64| ConstExprent::new(long(), Some(ConstValue::Long(v)), None).0;
    o.for_loop(
        "kdo_for_repeat",
        int(),
        int_const(0),
        func(FunctionType::Lt, vec![var(1, int()).0, int_const(10)]),
        FunctionType::Ppi,
    );
    o.for_loop(
        "kdo_for_range",
        int(),
        int_const(2),
        func(FunctionType::Lt, vec![int_const(7), var(1, int()).0]),
        FunctionType::Ipp,
    );
    o.for_loop(
        "kdo_for_down_to",
        int(),
        int_const(9),
        func(FunctionType::Lt, vec![var(1, int()).0, int_const(3)]),
        FunctionType::Mmi,
    );
    o.for_loop(
        "kdo_for_var_bound",
        int(),
        int_const(0),
        func(FunctionType::Lt, vec![var(1, int()).0, var(2, int()).0]),
        FunctionType::Ppi,
    );
    o.for_loop(
        "kdo_for_long",
        long(),
        long_const(0),
        func(FunctionType::Lt, vec![var(1, long()).0, long_const(5)]),
        FunctionType::Ppi,
    );
    o.for_loop(
        "kdo_for_abandoned",
        int(),
        int_const(0),
        func(FunctionType::Lt, vec![af(), af()]),
        FunctionType::Ppi,
    );
    o.for_loop(
        "kdo_for_not_less",
        int(),
        int_const(0),
        func(FunctionType::Le, vec![var(1, int()).0, int_const(3)]),
        FunctionType::Ppi,
    );

    o.reset();
    {
        let (h, c1, c2, c3, c4, d) = (bb(), bb(), bb(), bb(), bb(), bb());
        add(h, af());
        add(c1, af());
        add(c2, mexit());
        add(c3, af());
        add(c3, mexit());
        add(d, af());
        let ed = edge(REG, h, d);
        let e1 = edge(REG, h, c1);
        let e2 = edge(REG, h, c2);
        let e3 = edge(REG, h, c3);
        let e4 = edge(REG, h, c4);
        let e5 = edge(REG, h, c4);
        let sw = SwitchStatement::new(h, None);
        sw.get_case_statements().add_all(vec![c1, c2, c3, c4, d]);
        sw.get_case_edges().add(JList::from(vec![e1]));
        sw.get_case_edges().add(JList::from(vec![e2]));
        sw.get_case_edges().add(JList::from(vec![e3]));
        sw.get_case_edges().add(JList::from(vec![e4, e5]));
        sw.get_case_edges().add(JList::from(vec![ed]));
        sw.get_case_values()
            .add(JList::from(vec![Some(int_const(1))]));
        sw.get_case_values()
            .add(JList::from(vec![Some(int_const(2))]));
        sw.get_case_values()
            .add(JList::from(vec![Some(int_const(3))]));
        sw.get_case_values()
            .add(JList::from(vec![Some(int_const(4)), Some(int_const(5))]));
        sw.get_case_values().add(JList::from(vec![None]));
        sw.get_headexprent_list()
            .add(SwitchHeadExprent::new(af(), None).0);
        let ksw = KSwitchStatement::new(sw);
        o.out("kswitch", || ksw.0 .0.to_java(1));
        sw.add_labeled_edge(edge_c(BREAK, c1, bb(), None));
        let labeled = KSwitchStatement::new(sw);
        o.out("kswitch_labeled", || labeled.0 .0.to_java(1));
        sw.set_phantom(true);
        let phantom = KSwitchStatement::new(sw);
        o.out("kswitch_phantom", || phantom.0 .0.to_java(1));
    }

    o.reset();
    {
        let (body, h1, h2, out) = (bb(), bb(), bb(), bb());
        add(body, af());
        add(h1, mexit());
        add(h2, af());
        let cs = CatchStatement::new_empty();
        cs.set_first(body);
        for st in [body, h1, h2] {
            cs.get_stats().borrow_mut().add_with_key(st, st.id());
        }
        cs.get_exct_strings()
            .add(vec!["java/lang/Exception".to_string()]);
        cs.get_exct_strings()
            .add(vec!["java/lang/Error".to_string()]);
        cs.get_vars().add(var(7, VarType::VARTYPE_OBJECT()));
        cs.get_vars().add(var(8, VarType::VARTYPE_OBJECT()));
        let kcs = KCatchStatement::new(cs);
        o.out("kcatch", || kcs.0 .0.to_java(1));
        kcs.add_labeled_edge(edge_c(BREAK, body, out, None));
        o.out("kcatch_labeled", || kcs.0 .0.to_java(1));
    }

    o.reset();
    {
        let (body, h1) = (bb(), bb());
        add(body, af());
        add(h1, mexit());
        let cs = CatchStatement::new_empty();
        cs.set_first(body);
        for st in [body, h1] {
            cs.get_stats().borrow_mut().add_with_key(st, st.id());
        }
        cs.get_exct_strings()
            .add(vec!["java/lang/Exception".to_string()]);
        cs.get_vars().add(var(4, VarType::VARTYPE_OBJECT()));
        cs.get_resources().add(af());
        cs.get_resources().add(mexit());
        o.out("kcatch_resources", || {
            KCatchStatement::new(cs).0 .0.to_java(1)
        });
    }

    o.out
}

/// The oracle's `== name` / text / `<<` blocks.
fn golden() -> Vec<(String, String)> {
    let mut blocks = Vec::new();
    let mut current: Option<(String, Vec<&str>)> = None;
    for line in GOLDEN.lines() {
        match &mut current {
            None => {
                let name = line.strip_prefix("== ").expect("block header");
                current = Some((name.to_string(), Vec::new()));
            }
            Some((name, lines)) => {
                if line == "<<" {
                    blocks.push((name.clone(), lines.join("\n")));
                    current = None;
                } else {
                    lines.push(line);
                }
            }
        }
    }
    assert!(current.is_none(), "unterminated block");
    blocks
}

#[test]
fn kotlin_statements_render_like_vineflower() {
    let _context = common::install_default_context();
    let actual = render_all();
    let expected = golden();
    assert_eq!(
        expected.iter().map(|e| &e.0).collect::<Vec<_>>(),
        actual.iter().map(|a| &a.0).collect::<Vec<_>>(),
        "scenarios"
    );

    let mut mismatches = Vec::new();
    let mut pending = Vec::new();
    for ((name, text), (_, outcome)) in expected.iter().zip(&actual) {
        match outcome {
            Outcome::Pending(body) => pending.push(format!("{name} ({body})")),
            Outcome::Text(actual) => {
                if actual != text {
                    mismatches.push(format!("== {name}\njava:\n{text}\nrust:\n{actual}"));
                }
            }
        }
    }
    if !pending.is_empty() {
        eprintln!(
            "kt6_render: {} scenarios wait for other slices: {}",
            pending.len(),
            pending.join(", ")
        );
    }
    assert!(
        mismatches.is_empty(),
        "{} mismatches\n{}",
        mismatches.len(),
        mismatches.join("\n")
    );
    assert!(
        actual.len() - pending.len() >= 8,
        "the scenarios that need no other slice are compared"
    );
}

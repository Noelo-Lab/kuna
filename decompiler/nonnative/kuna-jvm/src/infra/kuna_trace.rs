//! Per-pass trace of a method's statement tree, for differential debugging against Vineflower.
//!
//! `DecompileRecord::add` hands every pass's tree to [`step`], which appends a canonical dump to the
//! thread's trace file. The text must equal what `tools/vftrace/src/kuna/vftrace/Dumper.java` prints
//! for the same structure in Vineflower; the format, its canonicalization rules and the comparer are
//! documented in `tools/vftrace/README.md`. Tracing is off unless [`start`] was called on this thread
//! or `KUNA_JVM_TRACE=<file>` is set (read once per thread; `KUNA_JVM_TRACE_FILTER=<substring>`
//! restricts it to matching method keys).

use std::cell::{Cell, RefCell};
use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs::File;
use std::io::{self, Write as _};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::annotation_exprent::AnnotationExprent;
use crate::array_exprent::ArrayExprent;
use crate::assert_exprent::AssertExprent;
use crate::assignment_exprent::AssignmentExprent;
use crate::basic_block_statement::BasicBlockStatement;
use crate::catch_all_statement::CatchAllStatement;
use crate::catch_statement::CatchStatement;
use crate::code_type::CodeType;
use crate::const_exprent::{ConstExprent, ConstValue};
use crate::do_statement::DoStatement;
use crate::dummy_exit_statement::DummyExitStatement;
use crate::exit_exprent::ExitExprent;
use crate::exprent::{Exprent, ExprentKind};
use crate::field_exprent::FieldExprent;
use crate::function_exprent::FunctionExprent;
use crate::general_statement::GeneralStatement;
use crate::if_exprent::IfExprent;
use crate::if_statement::IfStatement;
use crate::invocation_exprent::InvocationExprent;
use crate::link_constant::LinkConstant;
use crate::monitor_exprent::MonitorExprent;
use crate::new_exprent::NewExprent;
use crate::pattern_exprent::{PatternData, PatternExprent};
use crate::pooled_constant::PooledConstant;
use crate::primitive_constant::ConstValue as PrimitiveValue;
use crate::root_statement::RootStatement;
use crate::rt::{BitSet, JavaStr};
use crate::stat_edge::StatEdge;
use crate::statement::{LastBasicType, Statement, StatementType};
use crate::struct_method::StructMethod;
use crate::switch_exprent::SwitchExprent;
use crate::switch_head_exprent::SwitchHeadExprent;
use crate::switch_statement::SwitchStatement;
use crate::synchronized_statement::SynchronizedStatement;
use crate::type_family::TypeFamily;
use crate::var_exprent::VarExprent;
use crate::var_type::VarType;
use crate::var_version_pair::VarVersionPair;
use crate::yield_exprent::YieldExprent;

const UNKNOWN: u8 = 0;
const OFF: u8 = 1;
const ON: u8 = 2;

thread_local! {
    static MODE: Cell<u8> = const { Cell::new(UNKNOWN) };
    static TRACER: RefCell<Option<Tracer>> = const { RefCell::new(None) };
}

static ENV_THREADS: AtomicUsize = AtomicUsize::new(0);

struct MethodState {
    steps: usize,
    repeats: BTreeMap<String, u32>,
    last: Option<String>,
}

struct Tracer {
    out: File,
    path: PathBuf,
    filter: Option<String>,
    methods: BTreeMap<String, MethodState>,
    last_key: Option<String>,
    steps: u64,
    bytes: u64,
}

/// What [`finish`] wrote.
#[derive(Clone, Debug)]
pub struct TraceSummary {
    pub path: PathBuf,
    pub methods: usize,
    pub steps: u64,
    pub bytes: u64,
}

/// True when this thread traces; the first call per thread consults `KUNA_JVM_TRACE`.
#[inline]
pub fn enabled() -> bool {
    match MODE.get() {
        ON => true,
        OFF => false,
        _ => init_from_env(),
    }
}

#[cold]
fn init_from_env() -> bool {
    let Some(path) = std::env::var_os("KUNA_JVM_TRACE").filter(|p| !p.is_empty()) else {
        MODE.set(OFF);
        return false;
    };
    let mut path = PathBuf::from(path);
    let nth = ENV_THREADS.fetch_add(1, Ordering::Relaxed);
    if nth > 0 {
        let mut name = path.clone().into_os_string();
        name.push(format!(".{nth}"));
        path = PathBuf::from(name);
    }
    let filter = std::env::var("KUNA_JVM_TRACE_FILTER")
        .ok()
        .filter(|f| !f.is_empty());
    match start(&path, filter.as_deref()) {
        Ok(()) => true,
        Err(err) => {
            eprintln!("KUNA_JVM_TRACE: cannot create {}: {err}", path.display());
            MODE.set(OFF);
            false
        }
    }
}

/// Starts tracing this thread into `path` (truncated), only methods whose key contains `filter`.
pub fn start(path: &Path, filter: Option<&str>) -> io::Result<()> {
    let mut out = File::create(path)?;
    let header = b"#vftrace 1 engine=kuna hash=index\n";
    out.write_all(header)?;
    TRACER.with(|t| {
        *t.borrow_mut() = Some(Tracer {
            out,
            path: path.to_path_buf(),
            filter: filter.map(str::to_string),
            methods: BTreeMap::new(),
            last_key: None,
            steps: 0,
            bytes: header.len() as u64,
        })
    });
    MODE.set(ON);
    Ok(())
}

/// Stops tracing on this thread; `None` when it was not tracing.
pub fn finish() -> Option<TraceSummary> {
    MODE.set(OFF);
    let tracer = TRACER.with(|t| t.borrow_mut().take())?;
    let mut out = tracer.out;
    let _ = out.flush();
    Some(TraceSummary {
        path: tracer.path,
        methods: tracer.methods.len(),
        steps: tracer.steps,
        bytes: tracer.bytes,
    })
}

/// Records the tree after pass `name` of `mt`; a no-op unless [`enabled`].
pub fn step(mt: StructMethod, name: &str, root: RootStatement) {
    if !enabled() {
        return;
    }
    let key = format!(
        "{}#{}{}",
        mt.get_class_qualified_name(),
        mt.get_name(),
        mt.get_descriptor()
    );
    let wanted = TRACER.with(|t| {
        t.borrow()
            .as_ref()
            .is_some_and(|t| t.filter.as_ref().is_none_or(|f| key.contains(f.as_str())))
    });
    if !wanted {
        return;
    }
    let dump = match crate::rt::try_catch(|| dump(root)) {
        Ok(dump) => dump,
        Err(caught) => format!("! dump failed: {}\n", caught.message),
    };
    TRACER.with(|t| {
        let mut guard = t.borrow_mut();
        let Some(tracer) = guard.as_mut() else {
            return;
        };
        let mut text = String::new();
        if tracer.last_key.as_deref() != Some(key.as_str()) {
            let _ = writeln!(text, "=M {key}");
            tracer.last_key = Some(key.clone());
        }
        let state = tracer.methods.entry(key).or_insert_with(|| MethodState {
            steps: 0,
            repeats: BTreeMap::new(),
            last: None,
        });
        let repeat = state.repeats.entry(name.to_string()).or_insert(0);
        *repeat += 1;
        let n = state.steps;
        state.steps += 1;
        let _ = write!(text, "@ {n} {name}");
        if *repeat > 1 {
            let _ = write!(text, " #{repeat}");
        }
        text.push('\n');
        if state.last.as_deref() == Some(dump.as_str()) {
            let _ = writeln!(text, "= {}", n - 1);
        } else {
            text.push_str(&dump);
            state.last = Some(dump);
        }
        tracer.steps += 1;
        tracer.bytes += text.len() as u64;
        if let Err(err) = tracer.out.write_all(text.as_bytes()) {
            eprintln!(
                "kuna_trace: write to {} failed: {err}",
                tracer.path.display()
            );
        }
    });
}

/// The canonical dump of `root`'s statement tree.
pub fn dump(root: RootStatement) -> String {
    let mut d = Dumper::default();
    d.run(root);
    d.out
}

const ALL: i32 = Statement::STATEDGE_ALL;
const DIRECT_ALL: i32 = Statement::STATEDGE_DIRECT_ALL;
const LIST_KEYS: [i32; 6] = [
    DIRECT_ALL,
    StatEdge::TYPE_REGULAR,
    StatEdge::TYPE_EXCEPTION,
    StatEdge::TYPE_BREAK,
    StatEdge::TYPE_CONTINUE,
    StatEdge::TYPE_FINALLYEXIT,
];

#[derive(Default)]
struct Dumper {
    out: String,
    names: BTreeMap<Statement, String>,
    ids: BTreeMap<i32, usize>,
    order: Vec<(Statement, usize, Option<Statement>)>,
    orphans: Vec<Statement>,
    uses: BTreeMap<Exprent, Option<String>>,
    shared_count: usize,
    exit: Option<Statement>,
}

impl Dumper {
    fn run(&mut self, root: RootStatement) {
        self.number(root.0, None, 0);
        let exit = root.d(|d| d.dummy_exit).0;
        self.exit = Some(exit);
        if !self.names.contains_key(&exit) {
            self.register(exit, "x".to_string());
        }
        let order = self.order.clone();
        for &(st, _, _) in &order {
            self.count_exprents(st);
        }
        for &(st, depth, parent) in &order {
            self.statement(st, depth, parent);
        }
        self.statement(exit, 0, None);
        let mut lines: Vec<String> = self
            .orphans
            .iter()
            .map(|o| format!("{} orphan {}", self.names[o], kind(*o)))
            .collect();
        lines.sort_by(|a, b| compare_refs(a, b));
        for line in lines {
            self.out.push_str(&line);
            self.out.push('\n');
        }
    }

    fn register(&mut self, st: Statement, name: String) {
        *self.ids.entry(st.id()).or_insert(0) += 1;
        self.names.insert(st, name);
    }

    fn number(&mut self, st: Statement, tree_parent: Option<Statement>, depth: usize) {
        if self.names.contains_key(&st) {
            return;
        }
        self.register(st, format!("s{}", self.order.len()));
        self.order.push((st, depth, tree_parent));
        for child in st.get_stats().borrow().to_vec() {
            self.number(child, Some(st), depth + 1);
        }
    }

    fn sid(&mut self, st: Option<Statement>) -> String {
        let Some(st) = st else {
            return "~".to_string();
        };
        if let Some(name) = self.names.get(&st) {
            return name.clone();
        }
        let id = st.id();
        let mut name = format!("o{id}");
        if let Some(n) = self.ids.get(&id) {
            let _ = write!(name, "_{n}");
        }
        self.orphans.push(st);
        self.register(st, name.clone());
        name
    }

    fn statement(&mut self, st: Statement, depth: usize, tree_parent: Option<Statement>) {
        let pad = " ".repeat(depth);
        let inner = format!("{pad}  ");
        let mut h = String::new();
        let (id, first, parent, post, lbt, mon_enter, athrow, mon_exit, copied, phantom) =
            st.with(|d| {
                (
                    d.id,
                    d.first,
                    d.parent,
                    d.post,
                    d.last_basic_type,
                    d.is_monitor_enter,
                    d.is_last_athrow,
                    d.contains_monitor_exit,
                    d.copied,
                    d.phantom,
                )
            });
        let _ = write!(h, "{pad}{} {} id={id}", self.sid(Some(st)), kind(st));
        let stats = st.get_stats().borrow().to_vec();
        if first != stats.first().copied() {
            let _ = write!(h, " first={}", self.sid(first));
        }
        if parent != tree_parent {
            let _ = write!(h, " parent={}", self.sid(parent));
        }
        if post.is_some() {
            let _ = write!(h, " post={}", self.sid(post));
        }
        if lbt != LastBasicType::General {
            let _ = write!(h, " lbt={}", java_name(&format!("{lbt:?}")));
        }
        for (on, word) in [
            (mon_enter, " monenter"),
            (athrow, " athrow"),
            (mon_exit, " monexit"),
            (copied, " copied"),
            (phantom, " phantom"),
        ] {
            if on {
                h.push_str(word);
            }
        }
        self.subclass_header(st, &mut h);
        self.out.push_str(&h);
        self.out.push('\n');

        match st.with(|d| d.continue_set.clone()) {
            None => {
                let _ = writeln!(self.out, "{inner}cont ~");
            }
            Some(set) => {
                let items = set.borrow().to_vec();
                if !items.is_empty() {
                    let sorted = self.sorted_stats(items);
                    let _ = writeln!(self.out, "{inner}cont {sorted}");
                }
            }
        }
        let labels = st.get_label_edges().borrow().to_vec();
        if !labels.is_empty() {
            let mut refs: Vec<String> =
                labels.into_iter().map(|e| self.edge_ref(Some(e))).collect();
            refs.sort_by(|a, b| compare_refs(a, b));
            let _ = writeln!(self.out, "{inner}labels {}", refs.join(" "));
        }

        self.edges(st, &inner);

        for e in st.get_var_definitions().to_vec() {
            let text = self.expr(Some(e));
            let _ = writeln!(self.out, "{inner}def {text}");
        }
        if let Some(exprents) = st.get_exprents() {
            let list = exprents.to_vec();
            let _ = writeln!(self.out, "{inner}exprents {}", list.len());
            for e in list {
                let text = self.expr(Some(e));
                let _ = writeln!(self.out, "{inner}- {text}");
            }
        }
        self.subclass_body(st, &inner);
    }

    fn sorted_stats(&mut self, set: Vec<Statement>) -> String {
        let mut names: Vec<String> = set.into_iter().map(|s| self.sid(Some(s))).collect();
        names.sort_by(|a, b| compare_refs(a, b));
        format!("[{}]", names.join(" "))
    }

    fn subclass_header(&mut self, st: Statement, h: &mut String) {
        match st.type_() {
            StatementType::Root => {
                let root = RootStatement(st);
                let (exit, err, f) = root.d(|d| {
                    (
                        d.dummy_exit.0,
                        d.add_error_comment,
                        (d.flags.has_try_catch, d.flags.has_loops, d.flags.has_switch),
                    )
                });
                let mut flags = String::new();
                if f.0 {
                    flags.push('T');
                }
                if f.1 {
                    flags.push('L');
                }
                if f.2 {
                    flags.push('S');
                }
                if !flags.is_empty() {
                    let _ = write!(h, " flags={flags}");
                }
                if err {
                    h.push_str(" errcomment");
                }
                if Some(exit) != self.exit {
                    let _ = write!(h, " exit={}", self.sid(Some(exit)));
                }
            }
            StatementType::BasicBlock => {
                if BasicBlockStatement(st).d(|d| d.removable_monitorexit) {
                    h.push_str(" rmexit");
                }
            }
            StatementType::General => {
                if GeneralStatement(st).d(|d| d.placeholder) {
                    h.push_str(" placeholder");
                }
            }
            StatementType::If => {
                let (iftype, ifstat, elsestat, ifedge, elseedge, negated, pm, ppmm) =
                    IfStatement(st).d(|d| {
                        (
                            d.iftype,
                            d.ifstat,
                            d.elsestat,
                            d.ifedge,
                            d.elseedge,
                            d.negated,
                            d.pattern_matched,
                            d.has_ppmm,
                        )
                    });
                let _ = write!(h, " iftype={iftype}");
                let _ = write!(h, " ifstat={}", self.sid(ifstat));
                let _ = write!(h, " elsestat={}", self.sid(elsestat));
                let _ = write!(h, " ifedge={}", self.edge_ref(ifedge));
                let _ = write!(h, " elseedge={}", self.edge_ref(elseedge));
                for (on, word) in [(negated, " negated"), (pm, " patmatched"), (ppmm, " ppmm")] {
                    if on {
                        h.push_str(word);
                    }
                }
            }
            StatementType::Do => {
                let looptype = DoStatement(st).d(|d| d.looptype);
                let _ = write!(h, " loop={}", java_name(&format!("{looptype:?}")));
            }
            StatementType::Switch => {
                let (cases, default, scoped) = SwitchStatement(st).d(|d| {
                    (
                        d.case_statements.to_vec(),
                        d.default_edge,
                        d.scoped_case_statements.to_vec(),
                    )
                });
                let cases: Vec<String> = cases.into_iter().map(|c| self.sid(Some(c))).collect();
                let _ = write!(h, " cases=[{}]", cases.join(" "));
                let _ = write!(h, " default={}", self.edge_ref(default));
                if !scoped.is_empty() {
                    let _ = write!(h, " scoped={}", self.sorted_stats(scoped));
                }
            }
            StatementType::Synchronized => {
                let body = SynchronizedStatement(st).d(|d| d.body);
                let _ = write!(h, " body={}", self.sid(body));
            }
            StatementType::CatchAll => {
                let (handler, fin) = CatchAllStatement(st).d(|d| (d.handler, d.is_finally));
                let _ = write!(h, " handler={}", self.sid(handler));
                if fin {
                    h.push_str(" finally");
                }
            }
            StatementType::DummyExit => {
                if let Some(bc) = DummyExitStatement(st).d(|d| d.bytecode.clone()) {
                    let _ = write!(h, " bc={}", bits(&bc));
                }
            }
            _ => {}
        }
    }

    fn subclass_body(&mut self, st: Statement, inner: &str) {
        match st.type_() {
            StatementType::Root => {
                let comments = RootStatement(st).d(|d| {
                    d.comment_lines
                        .as_ref()
                        .map(|c| c.iter().cloned().collect::<Vec<String>>())
                });
                for c in comments.unwrap_or_default() {
                    let _ = writeln!(self.out, "{inner}comment {}", jstr(&c));
                }
            }
            StatementType::BasicBlock => {
                if st.get_exprents().is_none() {
                    let seq = BasicBlockStatement(st).d(|d| d.block).get_seq();
                    let mut s = String::new();
                    for i in 0..seq.length() {
                        let (offset, opcode) =
                            seq.get_instr(i).with(|d| (d.start_offset, d.opcode));
                        if i > 0 {
                            s.push(' ');
                        }
                        let _ = write!(s, "{offset}:{opcode}");
                    }
                    let _ = writeln!(self.out, "{inner}insn [{s}]");
                }
            }
            StatementType::If => {
                let head = IfStatement(st).d(|d| d.headexprent.to_vec());
                self.head(inner, "head", head);
            }
            StatementType::Do => {
                let (init, cond, inc) = DoStatement(st).d(|d| {
                    (
                        d.init_exprent.to_vec(),
                        d.condition_exprent.to_vec(),
                        d.inc_exprent.to_vec(),
                    )
                });
                self.head(inner, "init", init);
                self.head(inner, "cond", cond);
                self.head(inner, "inc", inc);
            }
            StatementType::Switch => {
                let (head, case_edges, case_values, guards) = SwitchStatement(st).d(|d| {
                    (
                        d.headexprent.to_vec(),
                        d.case_edges.to_vec(),
                        d.case_values.to_vec(),
                        d.case_guards.to_vec(),
                    )
                });
                self.head(inner, "head", head);
                for (i, edges) in case_edges.iter().enumerate() {
                    let refs: Vec<String> = edges
                        .to_vec()
                        .into_iter()
                        .map(|e| self.edge_ref(Some(e)))
                        .collect();
                    let _ = writeln!(self.out, "{inner}caseedges {i} [{}]", refs.join(" "));
                }
                for (i, values) in case_values.iter().enumerate() {
                    let vals: Vec<String> = values
                        .to_vec()
                        .into_iter()
                        .map(|e| match e {
                            None => "default".to_string(),
                            Some(e) => self.expr(Some(e)),
                        })
                        .collect();
                    let _ = writeln!(self.out, "{inner}casevalues {i} [{}]", vals.join(" "));
                }
                for (i, guard) in guards.into_iter().enumerate() {
                    let text = self.expr(guard);
                    let _ = writeln!(self.out, "{inner}guard {i} {text}");
                }
            }
            StatementType::Synchronized => {
                let head = SynchronizedStatement(st).d(|d| d.headexprent.to_vec());
                self.head(inner, "head", head);
            }
            StatementType::TryCatch => {
                let (exc, vars, resources) = CatchStatement(st).d(|d| {
                    (
                        d.exctstrings.to_vec(),
                        d.vars.to_vec(),
                        d.resources.to_vec(),
                    )
                });
                for (i, e) in exc.iter().enumerate() {
                    let _ = writeln!(self.out, "{inner}catch {i} [{}]", e.join(" "));
                }
                for (i, v) in vars.into_iter().enumerate() {
                    let text = self.expr(Some(v.0));
                    let _ = writeln!(self.out, "{inner}catchvar {i} {text}");
                }
                for r in resources {
                    let text = self.expr(Some(r));
                    let _ = writeln!(self.out, "{inner}resource {text}");
                }
            }
            StatementType::CatchAll => {
                let (monitor, vars) = CatchAllStatement(st).d(|d| (d.monitor, d.vars.to_vec()));
                if let Some(m) = monitor {
                    let text = self.expr(Some(m.0));
                    let _ = writeln!(self.out, "{inner}monitor {text}");
                }
                for v in vars {
                    let text = self.expr(Some(v.0));
                    let _ = writeln!(self.out, "{inner}catchvar {text}");
                }
            }
            _ => {}
        }
    }

    /// One-slot head lists: kuna's empty list and Vineflower's `[null]` both read as absent.
    fn head(&mut self, inner: &str, key: &str, list: Vec<Exprent>) {
        if list.is_empty() {
            return;
        }
        let mut line = format!("{inner}{key}");
        for e in list {
            line.push(' ');
            line.push_str(&self.expr(Some(e)));
        }
        self.out.push_str(&line);
        self.out.push('\n');
    }

    fn edge_ref(&mut self, e: Option<StatEdge>) -> String {
        let Some(e) = e else {
            return "~".to_string();
        };
        let (type_, src, dst) = e.with(|d| (d.type_, d.source, d.destination));
        if let Some(idx) = index_of(edge_list(src, true, ALL).as_deref(), e) {
            return format!("{}.{idx}", self.sid(Some(src)));
        }
        let s = self.sid(Some(src));
        let t = self.sid(Some(dst));
        format!("*{s}{}{t}", type_name(type_))
    }

    fn edge_attrs(&mut self, e: StatEdge) -> String {
        let (type_, dst, closure, labeled, explicit, can_inline, phantom_continue, exceptions) = e
            .with(|d| {
                (
                    d.type_,
                    d.destination,
                    d.closure,
                    d.labeled,
                    d.explicit,
                    d.can_inline,
                    d.phantom_continue,
                    d.exceptions.clone(),
                )
            });
        let mut s = format!("{} {}", type_name(type_), self.sid(Some(dst)));
        if closure.is_some() {
            let _ = write!(s, " c={}", self.sid(closure));
        }
        for (on, word) in [
            (!labeled, " unlabeled"),
            (!explicit, " implicit"),
            (!can_inline, " noinline"),
            (phantom_continue, " phantomcont"),
        ] {
            if on {
                s.push_str(word);
            }
        }
        if let Some(exc) = exceptions {
            let _ = write!(s, " exc=[{}]", exc.join(" "));
        }
        s
    }

    fn edges(&mut self, st: Statement, inner: &str) {
        let succ = edge_list(st, true, ALL);
        if let Some(succ) = &succ {
            for &e in succ {
                let attrs = self.edge_attrs(e);
                let _ = write!(self.out, "{inner}> {attrs}");
                let src = e.get_source();
                if src != st {
                    let s = self.sid(Some(src));
                    let _ = write!(self.out, " src={s}");
                }
                self.out.push('\n');
            }
        }
        let pred = edge_list(st, false, ALL);
        if let Some(pred) = pred.as_ref().filter(|p| !p.is_empty()) {
            let mut refs = Vec::new();
            for &e in pred {
                let mut r = self.edge_ref(Some(e));
                let dst = e.get_destination();
                if dst != st {
                    let _ = write!(r, "!dst={}", self.sid(Some(dst)));
                }
                refs.push(r);
            }
            refs.sort_by(|a, b| compare_refs(a, b));
            let _ = writeln!(self.out, "{inner}< {}", refs.join(" "));
        }
        self.sub_lists(st, inner, true, succ.as_deref());
        self.sub_lists(st, inner, false, pred.as_deref());
    }

    fn sub_lists(&mut self, st: Statement, inner: &str, succ: bool, all: Option<&[StatEdge]>) {
        for key in LIST_KEYS {
            let actual = edge_list(st, succ, key).unwrap_or_default();
            let mut same = true;
            let mut j = 0;
            for &e in all.unwrap_or_default() {
                if in_list(key, e) {
                    if j >= actual.len() || actual[j] != e {
                        same = false;
                        break;
                    }
                    j += 1;
                }
            }
            if same && j == actual.len() {
                continue;
            }
            let mut refs = Vec::new();
            for &e in &actual {
                match index_of(all, e).filter(|_| succ) {
                    Some(idx) => refs.push(idx.to_string()),
                    None => refs.push(self.edge_ref(Some(e))),
                }
            }
            let dir = if succ { ">" } else { "<" };
            let _ = writeln!(
                self.out,
                "{inner}{dir}{} [{}]",
                type_name(key),
                refs.join(" ")
            );
        }
    }

    fn count_exprents(&mut self, st: Statement) {
        for e in st.get_var_definitions().to_vec() {
            self.count_tree(e);
        }
        if let Some(exprents) = st.get_exprents() {
            for e in exprents.to_vec() {
                self.count_tree(e);
            }
        }
        let mut more: Vec<Exprent> = Vec::new();
        match st.type_() {
            StatementType::If => more = IfStatement(st).d(|d| d.headexprent.to_vec()),
            StatementType::Do => DoStatement(st).d(|d| {
                more.extend(d.init_exprent.to_vec());
                more.extend(d.condition_exprent.to_vec());
                more.extend(d.inc_exprent.to_vec());
            }),
            StatementType::Switch => SwitchStatement(st).d(|d| {
                more.extend(d.headexprent.to_vec());
                for values in d.case_values.to_vec() {
                    more.extend(values.to_vec().into_iter().flatten());
                }
                more.extend(d.case_guards.to_vec().into_iter().flatten());
            }),
            StatementType::Synchronized => {
                more = SynchronizedStatement(st).d(|d| d.headexprent.to_vec())
            }
            StatementType::TryCatch => CatchStatement(st).d(|d| {
                more.extend(d.vars.to_vec().into_iter().map(|v| v.0));
                more.extend(d.resources.to_vec());
            }),
            StatementType::CatchAll => CatchAllStatement(st).d(|d| {
                more.extend(d.monitor.map(|m| m.0));
                more.extend(d.vars.to_vec().into_iter().map(|v| v.0));
            }),
            _ => {}
        }
        for e in more {
            self.count_tree(e);
        }
    }

    fn count_tree(&mut self, e: Exprent) {
        if let Some(label) = self.uses.get_mut(&e) {
            if label.is_none() {
                *label = Some(String::new());
            }
            return;
        }
        self.uses.insert(e, None);
        for child in children(e).into_iter().flatten() {
            self.count_tree(child);
        }
    }

    /// `(label, first_print)` for a shared exprent, `None` when it occurs once.
    fn share(&mut self, e: Exprent) -> Option<(String, bool)> {
        let entry = self.uses.get_mut(&e)?;
        match entry {
            None => None,
            Some(label) if label.is_empty() => {
                *label = self.shared_count.to_string();
                self.shared_count += 1;
                Some((label.clone(), true))
            }
            Some(label) => Some((label.clone(), false)),
        }
    }

    fn expr(&mut self, e: Option<Exprent>) -> String {
        let mut s = String::new();
        self.expr_into(e, &mut s);
        s
    }

    fn list_into(&mut self, key: &str, list: Vec<Exprent>, s: &mut String) {
        let _ = write!(s, " ({key}");
        for x in list {
            s.push(' ');
            self.expr_into(Some(x), s);
        }
        s.push(')');
    }

    fn child(&mut self, e: Option<Exprent>, s: &mut String) {
        s.push(' ');
        self.expr_into(e, s);
    }

    fn expr_into(&mut self, e: Option<Exprent>, s: &mut String) {
        let Some(e) = e else {
            s.push('~');
            return;
        };
        let share = self.share(e);
        if let Some((label, false)) = &share {
            let _ = write!(s, "^{label}");
            return;
        }
        s.push('(');
        if let Some((label, _)) = &share {
            let _ = write!(s, "&{label} ");
        }
        s.push_str(expr_kind(e));
        if let Some(bc) = e.with(|d| d.bytecode.clone()).filter(|b| !b.is_empty()) {
            let _ = write!(s, "@{}", bits(&bc));
        }
        match e.with(|d| expr_tag(&d.kind)) {
            Tag::Annotation => {
                let (class_name, names, values) = AnnotationExprent(e).d(|d| {
                    (
                        d.class_name.clone(),
                        d.par_names.clone(),
                        d.par_values.clone(),
                    )
                });
                let _ = write!(s, " {class_name}");
                for (name, value) in names.iter().zip(values) {
                    let _ = write!(s, " {name}=");
                    self.expr_into(Some(value), s);
                }
            }
            Tag::Array => {
                let (array, index, hard) =
                    ArrayExprent(e).d(|d| (d.array, d.index, d.hard_type.clone()));
                let _ = write!(s, " {}", vt(Some(&hard)));
                self.child(Some(array), s);
                self.child(Some(index), s);
            }
            Tag::Assert => {
                for p in AssertExprent(e).d(|d| d.parameters.clone()) {
                    self.child(p, s);
                }
            }
            Tag::Assignment => {
                let (left, right, op) = AssignmentExprent(e).d(|d| (d.left, d.right, d.cond_type));
                if let Some(op) = op {
                    let _ = write!(s, " op={}", java_name(&format!("{op:?}")));
                }
                self.child(Some(left), s);
                self.child(Some(right), s);
            }
            Tag::Const => {
                let (t, value, bool_ok, condy) = ConstExprent(e).d(|d| {
                    (
                        d.const_type.clone(),
                        d.value.clone(),
                        d.bool_permitted,
                        d.was_condy,
                    )
                });
                let _ = write!(s, " {} {}", vt(Some(&t)), const_value(value.as_ref()));
                if bool_ok {
                    s.push_str(" boolok");
                }
                if condy {
                    s.push_str(" condy");
                }
            }
            Tag::Exit => {
                let (exit_type, value, ret) =
                    ExitExprent(e).d(|d| (d.exit_type, d.value, d.ret_type.clone()));
                let _ = write!(s, " {}", java_name(&format!("{exit_type:?}")));
                if let Some(ret) = ret {
                    let _ = write!(s, " rt={}", vt(Some(&ret)));
                }
                self.child(value, s);
            }
            Tag::Field => {
                let f = FieldExprent(e).d(|d| {
                    (
                        d.classname.clone(),
                        d.name.clone(),
                        d.descriptor.descriptor_string.clone(),
                        d.is_static,
                        d.force_qualified,
                        d.is_qualifier,
                        d.was_condy,
                        d.instance,
                    )
                });
                let _ = write!(s, " {}.{} {}", f.0, f.1, f.2);
                for (on, word) in [
                    (f.3, " static"),
                    (f.4, " forcequal"),
                    (f.5, " qualifier"),
                    (f.6, " condy"),
                ] {
                    if on {
                        s.push_str(word);
                    }
                }
                self.child(f.7, s);
            }
            Tag::Function => {
                let (func, it, operands, needs_cast, no_newline) = FunctionExprent(e).d(|d| {
                    (
                        d.func_type,
                        d.implicit_type.clone(),
                        d.lst_operands.to_vec(),
                        d.needs_cast,
                        d.disable_newline_group_creation,
                    )
                });
                let _ = write!(s, " {}", java_name(&format!("{func:?}")));
                if let Some(it) = it {
                    let _ = write!(s, " it={}", vt(Some(&it)));
                }
                if !needs_cast {
                    s.push_str(" nocast");
                }
                if no_newline {
                    s.push_str(" nonewline");
                }
                for x in operands {
                    self.child(Some(x), s);
                }
            }
            Tag::If => {
                let condition = IfExprent(e).d(|d| d.condition);
                self.child(Some(condition), s);
            }
            Tag::Invocation => self.invocation(InvocationExprent(e), s),
            Tag::Monitor => {
                let (mon_type, remove, value) =
                    MonitorExprent(e).d(|d| (d.mon_type, d.remove, d.value));
                let _ = write!(s, " {}", java_name(&format!("{mon_type:?}")));
                if remove {
                    s.push_str(" remove");
                }
                self.child(Some(value), s);
            }
            Tag::New => self.new_exprent(NewExprent(e), s),
            Tag::Pattern => {
                let (var_type, record, bound, exprents) = PatternExprent(e).d(|d| {
                    let PatternData::RecordPatternData(r) = &d.data;
                    (
                        d.var_type.clone(),
                        r.cl.qualified_name(),
                        d.bound_types.to_vec(),
                        d.exprents.to_vec(),
                    )
                });
                let _ = write!(s, " {} record={record}", vt(Some(&var_type)));
                let _ = write!(s, " bt={}", vts_opt(&bound));
                for x in exprents {
                    self.child(Some(x), s);
                }
            }
            Tag::Switch => {
                let (backing, t, fallthrough, standalone) = SwitchExprent(e)
                    .d(|d| (d.backing, d.type_.clone(), d.fallthrough, d.standalone));
                let b = self.sid(Some(backing.0));
                let _ = write!(s, " {b} {}", vt(Some(&t)));
                if fallthrough {
                    s.push_str(" fallthrough");
                }
                if standalone {
                    s.push_str(" standalone");
                }
            }
            Tag::SwitchHead => {
                let (value, case_values) =
                    SwitchHeadExprent(e).d(|d| (d.value, d.case_values.to_vec()));
                self.child(Some(value), s);
                for values in case_values {
                    s.push_str(" [");
                    for (k, v) in values.to_vec().into_iter().enumerate() {
                        if k > 0 {
                            s.push(' ');
                        }
                        match v {
                            None => s.push_str("default"),
                            Some(v) => self.expr_into(Some(v), s),
                        }
                    }
                    s.push(']');
                }
            }
            Tag::Var => self.var(VarExprent(e), s),
            Tag::Yield => {
                let (content, ret) = YieldExprent(e).d(|d| (d.content, d.ret_type.clone()));
                let _ = write!(s, " {}", vt(Some(&ret)));
                self.child(Some(content), s);
            }
        }
        s.push(')');
    }

    fn invocation(&mut self, i: InvocationExprent, s: &mut String) {
        let d = i.d(|d| InvocationView {
            invocation_type: format!("{:?}", d.invocation_type),
            functype: format!("{:?}", d.functype),
            classname: d.classname.clone(),
            name: d.name.clone(),
            descriptor: d.string_descriptor.clone(),
            is_static: d.is_static,
            can_ignore_boxing: d.can_ignore_boxing,
            suffix: d.invoke_dynamic_class_suffix.clone(),
            target: d.desc,
            bsm: d.bootstrap_method.clone(),
            bsa: d.bootstrap_arguments.clone(),
            generic_args: d.generic_args.clone(),
            remapped: d.remapped_inst_type.clone(),
            force_generic: d.force_generic_qualfication,
            generics_map: d
                .generics_map
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect(),
            invocation_instance: d.is_invocation_instance,
            qualifier: d.is_qualifier,
            boxing: (
                d.boxing.force_boxing,
                d.boxing.force_unboxing,
                d.boxing.keep_cast,
            ),
            null_check: d.is_synthetic_null_check,
            lazy_condy: d.was_lazy_condy,
            instance: d.instance,
            params: d.lst_parameters.to_vec(),
        });
        let _ = write!(
            s,
            " {} {} {}.{}{}",
            java_name(&d.invocation_type),
            java_name(&d.functype),
            d.classname,
            d.name,
            d.descriptor
        );
        if d.is_static {
            s.push_str(" static");
        }
        if !d.can_ignore_boxing {
            s.push_str(" noignorebox");
        }
        if let Some(sfx) = &d.suffix {
            let _ = write!(s, " sfx={sfx}");
        }
        if let Some(target) = d.target {
            let _ = write!(
                s,
                " target={}.{}{}",
                target.get_class_qualified_name(),
                target.get_name(),
                target.get_descriptor()
            );
        }
        if let Some(bsm) = &d.bsm {
            let _ = write!(s, " bsm={}", link(bsm));
        }
        if let Some(bsa) = &d.bsa {
            let items: Vec<String> = bsa.iter().map(|c| pooled(c.as_ref())).collect();
            let _ = write!(s, " bsa=[{}]", items.join(" "));
        }
        if !d.generic_args.is_empty() {
            let _ = write!(s, " ga={}", vts(&d.generic_args));
        }
        if let Some(rit) = &d.remapped {
            let _ = write!(s, " rit={}", vt(Some(rit)));
        }
        if d.force_generic {
            s.push_str(" forcegeneric");
        }
        if !d.generics_map.is_empty() {
            let mut entries: Vec<String> = d
                .generics_map
                .iter()
                .map(|(k, v)| format!("{}={}", vt(Some(k)), vt(v.as_ref())))
                .collect();
            entries.sort_by(|a, b| utf16_cmp(a, b));
            let _ = write!(s, " gm={{{}}}", entries.join(" "));
        }
        for (on, word) in [
            (d.invocation_instance, " invinstance"),
            (d.qualifier, " qualifier"),
            (d.boxing.0, " forcebox"),
            (d.boxing.1, " forceunbox"),
            (d.boxing.2, " keepcast"),
            (d.null_check, " nullcheck"),
            (d.lazy_condy, " lazycondy"),
        ] {
            if on {
                s.push_str(word);
            }
        }
        self.child(d.instance, s);
        for p in d.params {
            self.child(Some(p), s);
        }
    }

    fn new_exprent(&mut self, n: NewExprent, s: &mut String) {
        let d = n.d(|d| {
            (
                d.new_type.clone(),
                [
                    (d.direct_array_init, " directinit"),
                    (d.is_var_arg_param, " vararg"),
                    (d.anonymous, " anon"),
                    (d.lambda, " lambda"),
                    (d.method_reference, " methodref"),
                    (d.enum_const, " enumconst"),
                ],
                d.generic_args.clone(),
                d.inferred_lambda_type.clone(),
                d.constructor,
                d.lst_dims.to_vec(),
                d.lst_array_elements.to_vec(),
            )
        });
        let _ = write!(s, " {}", vt(Some(&d.0)));
        for (on, word) in d.1 {
            if on {
                s.push_str(word);
            }
        }
        if !d.2.is_empty() {
            let _ = write!(s, " ga={}", vts(&d.2));
        }
        if let Some(ilt) = &d.3 {
            let _ = write!(s, " ilt={}", vt(Some(ilt)));
        }
        self.child(d.4.map(|c| c.0), s);
        self.list_into("dims", d.5, s);
        self.list_into("elems", d.6, s);
    }

    fn var(&mut self, v: VarExprent, s: &mut String) {
        let d = v.d(|d| {
            (
                d.index,
                d.version,
                d.var_type.clone(),
                d.processor,
                [
                    (d.definition, " def"),
                    (d.class_def, " classdef"),
                    (d.stack, " stack"),
                ],
                d.lvt.clone(),
                d.backing,
                d.bound_type.clone(),
                [
                    (d.is_effectively_final, " efffinal"),
                    (d.writing_pattern, " writingpattern"),
                ],
                [
                    (d.is_intersection_type, " intersection"),
                    (d.is_catch_temp_var, " catchtemp"),
                ],
            )
        });
        let (index, version) = (d.0, d.1);
        let _ = write!(s, " {index}.{version} t={}", vt(d.2.as_ref()));
        match d.3 {
            Some(processor) => {
                if let Some(pt) = processor.get_var_type(VarVersionPair::new(index, version)) {
                    let _ = write!(s, " pt={}", vt(Some(&pt)));
                }
            }
            None => s.push_str(" noproc"),
        }
        for (on, word) in d.4 {
            if on {
                s.push_str(word);
            }
        }
        if let Some(lvt) = &d.5 {
            let _ = write!(
                s,
                " lvt={}:{}@{}-{}",
                lvt.name,
                lvt.descriptor,
                lvt.start_pc,
                lvt.start_pc.wrapping_add(lvt.length)
            );
            if let Some(sig) = &lvt.signature {
                let _ = write!(s, "<{sig}>");
            }
            let _ = write!(s, "/{}.{}", lvt.version.var, lvt.version.version);
        }
        if let Some(backing) = d.6 {
            let (offset, opcode) = backing.with(|b| (b.start_offset, b.opcode));
            let _ = write!(s, " backing={offset}:{opcode}");
        }
        for (on, word) in d.8 {
            if on {
                s.push_str(word);
            }
        }
        if let Some(bound) = &d.7 {
            let _ = write!(s, " bound={}", vt(Some(bound)));
        }
        for (on, word) in d.9 {
            if on {
                s.push_str(word);
            }
        }
    }
}

struct InvocationView {
    invocation_type: String,
    functype: String,
    classname: String,
    name: String,
    descriptor: String,
    is_static: bool,
    can_ignore_boxing: bool,
    suffix: Option<String>,
    target: Option<StructMethod>,
    bsm: Option<LinkConstant>,
    bsa: Option<Vec<Option<PooledConstant>>>,
    generic_args: Vec<VarType>,
    remapped: Option<VarType>,
    force_generic: bool,
    generics_map: Vec<(VarType, Option<VarType>)>,
    invocation_instance: bool,
    qualifier: bool,
    boxing: (bool, bool, bool),
    null_check: bool,
    lazy_condy: bool,
    instance: Option<Exprent>,
    params: Vec<Exprent>,
}

#[derive(Clone, Copy)]
enum Tag {
    Annotation,
    Array,
    Assert,
    Assignment,
    Const,
    Exit,
    Field,
    Function,
    If,
    Invocation,
    Monitor,
    New,
    Pattern,
    Switch,
    SwitchHead,
    Var,
    Yield,
}

fn expr_tag(kind: &ExprentKind) -> Tag {
    match kind {
        ExprentKind::Annotation(_) => Tag::Annotation,
        ExprentKind::Array(_) => Tag::Array,
        ExprentKind::Assert(_) => Tag::Assert,
        ExprentKind::Assignment(_) => Tag::Assignment,
        ExprentKind::Const(_) => Tag::Const,
        ExprentKind::Exit(_) => Tag::Exit,
        ExprentKind::Field(_) => Tag::Field,
        ExprentKind::Function(_) => Tag::Function,
        ExprentKind::If(_) => Tag::If,
        ExprentKind::Invocation(_) => Tag::Invocation,
        ExprentKind::Monitor(_) => Tag::Monitor,
        ExprentKind::New(_) => Tag::New,
        ExprentKind::Pattern(_) => Tag::Pattern,
        ExprentKind::Switch(_) => Tag::Switch,
        ExprentKind::SwitchHead(_) => Tag::SwitchHead,
        ExprentKind::Var(_) => Tag::Var,
        ExprentKind::Yield(_) => Tag::Yield,
    }
}

fn expr_kind(e: Exprent) -> &'static str {
    match e.with(|d| expr_tag(&d.kind)) {
        Tag::Annotation => "annotation",
        Tag::Array => "array",
        Tag::Assert => "assert",
        Tag::Assignment => "assign",
        Tag::Const => "const",
        Tag::Exit => "exit",
        Tag::Field => "field",
        Tag::Function => "func",
        Tag::If => "if",
        Tag::Invocation => "invoke",
        Tag::Monitor => "monitor",
        Tag::New => "new",
        Tag::Pattern => "pattern",
        Tag::Switch => "switch",
        Tag::SwitchHead => "switchhead",
        Tag::Var => "var",
        Tag::Yield => "yield",
    }
}

/// Children in the order the dump prints them (the share counting walks the same tree).
fn children(e: Exprent) -> Vec<Option<Exprent>> {
    e.with(|d| match &d.kind {
        ExprentKind::Annotation(a) => a.par_values.iter().map(|&x| Some(x)).collect(),
        ExprentKind::Array(a) => vec![Some(a.array), Some(a.index)],
        ExprentKind::Assert(a) => a.parameters.clone(),
        ExprentKind::Assignment(a) => vec![Some(a.left), Some(a.right)],
        ExprentKind::Exit(x) => vec![x.value],
        ExprentKind::Field(f) => vec![f.instance],
        ExprentKind::Function(f) => f.lst_operands.to_vec().into_iter().map(Some).collect(),
        ExprentKind::If(i) => vec![Some(i.condition)],
        ExprentKind::Invocation(i) => {
            let mut l = vec![i.instance];
            l.extend(i.lst_parameters.to_vec().into_iter().map(Some));
            l
        }
        ExprentKind::Monitor(m) => vec![Some(m.value)],
        ExprentKind::New(n) => {
            let mut l = vec![n.constructor.map(|c| c.0)];
            l.extend(n.lst_dims.to_vec().into_iter().map(Some));
            l.extend(n.lst_array_elements.to_vec().into_iter().map(Some));
            l
        }
        ExprentKind::Pattern(p) => p.exprents.to_vec().into_iter().map(Some).collect(),
        ExprentKind::SwitchHead(s) => {
            let mut l = vec![Some(s.value)];
            for values in s.case_values.to_vec() {
                l.extend(values.to_vec());
            }
            l
        }
        ExprentKind::Yield(y) => vec![Some(y.content)],
        ExprentKind::Const(_) | ExprentKind::Switch(_) | ExprentKind::Var(_) => Vec::new(),
    })
}

fn kind(st: Statement) -> &'static str {
    match st.type_() {
        StatementType::Root => "Root",
        StatementType::BasicBlock => "Block",
        StatementType::Sequence => "Seq",
        StatementType::DummyExit => "Exit",
        StatementType::General => "General",
        StatementType::If => "If",
        StatementType::Do => "Do",
        StatementType::Switch => "Switch",
        StatementType::Synchronized => "Sync",
        StatementType::TryCatch => "Try",
        StatementType::CatchAll => "CatchAll",
        StatementType::Other => "Other",
    }
}

fn edge_list(st: Statement, succ: bool, key: i32) -> Option<Vec<StatEdge>> {
    st.with(|d| {
        let map = if succ {
            &d.map_succ_edges
        } else {
            &d.map_pred_edges
        };
        map.get(&key).map(|l| l.to_vec())
    })
}

fn index_of(list: Option<&[StatEdge]>, e: StatEdge) -> Option<usize> {
    list?.iter().position(|&x| x == e)
}

fn in_list(key: i32, e: StatEdge) -> bool {
    let type_ = e.get_type();
    if key == DIRECT_ALL {
        type_ != StatEdge::TYPE_EXCEPTION
    } else {
        type_ == key
    }
}

fn type_name(type_: i32) -> String {
    match type_ {
        StatEdge::TYPE_REGULAR => "R".to_string(),
        StatEdge::TYPE_EXCEPTION => "E".to_string(),
        StatEdge::TYPE_BREAK => "B".to_string(),
        StatEdge::TYPE_CONTINUE => "C".to_string(),
        StatEdge::TYPE_FINALLYEXIT => "F".to_string(),
        DIRECT_ALL => "D".to_string(),
        _ => type_.to_string(),
    }
}

/// Canonical names sort by kind letter, then number, then the rest ("s3" before "s12").
fn compare_refs(a: &str, b: &str) -> std::cmp::Ordering {
    let ka = a.chars().next().map_or(0, |c| c as u32);
    let kb = b.chars().next().map_or(0, |c| c as u32);
    ka.cmp(&kb)
        .then_with(|| leading_number(a).cmp(&leading_number(b)))
        .then_with(|| utf16_cmp(a, b))
}

fn leading_number(s: &str) -> i64 {
    let digits: String = s
        .chars()
        .skip(1)
        .take_while(|c| c.is_ascii_digit())
        .collect();
    if digits.is_empty() {
        -1
    } else {
        digits.bytes().fold(0i64, |n, b| {
            n.wrapping_mul(10).wrapping_add((b - b'0') as i64)
        })
    }
}

/// `String.compareTo`: UTF-16 code unit order.
fn utf16_cmp(a: &str, b: &str) -> std::cmp::Ordering {
    a.utf16().cmp(&b.utf16())
}

/// A Java enum constant name from a Rust variant's `Debug` text: `BoolNot` -> `BOOL_NOT`.
fn java_name(debug: &str) -> String {
    let mut out = String::new();
    let mut prev_lower = false;
    for c in debug.chars() {
        if c.is_ascii_uppercase() && prev_lower {
            out.push('_');
        }
        prev_lower = c.is_ascii_lowercase() || c.is_ascii_digit();
        out.push(c.to_ascii_uppercase());
    }
    out
}

fn link(l: &LinkConstant) -> String {
    format!(
        "{}:{}.{}{}",
        l.type_,
        l.classname.as_deref().unwrap_or(""),
        l.elementname,
        l.descriptor
    )
}

fn pooled(c: Option<&PooledConstant>) -> String {
    match c {
        None => "~".to_string(),
        Some(PooledConstant::Link(l)) => link(l),
        Some(PooledConstant::Primitive(p)) => {
            let value = match &p.value {
                None => "null".to_string(),
                Some(PrimitiveValue::Integer(i)) => i.to_string(),
                Some(PrimitiveValue::Long(l)) => format!("{l}L"),
                Some(PrimitiveValue::Float(f)) => format!("f:{:x}", f.to_bits()),
                Some(PrimitiveValue::Double(d)) => format!("d:{:x}", d.to_bits()),
                Some(PrimitiveValue::String(s)) => jstr(s),
            };
            format!("{}:{value}{}", p.type_, if p.is_array { "[]" } else { "" })
        }
    }
}

fn const_value(v: Option<&ConstValue>) -> String {
    match v {
        None => "null".to_string(),
        Some(ConstValue::Int(i)) => i.to_string(),
        Some(ConstValue::Long(l)) => format!("{l}L"),
        Some(ConstValue::Float(f)) => format!("f:{:x}", f.to_bits()),
        Some(ConstValue::Double(d)) => format!("d:{:x}", d.to_bits()),
        Some(ConstValue::String(s)) => jstr(s),
    }
}

fn jstr(v: &str) -> String {
    let mut s = String::from("\"");
    for unit in v.utf16() {
        match unit {
            0x22 | 0x5c => {
                s.push('\\');
                s.push(unit as u8 as char);
            }
            0x20..=0x7e => s.push(unit as u8 as char),
            _ => {
                let _ = write!(s, "\\u{unit:04x}");
            }
        }
    }
    s.push('"');
    s
}

fn bits(b: &BitSet) -> String {
    let mut s = String::new();
    let mut i = b.next_set_bit(0);
    while i >= 0 {
        let start = i as usize;
        let mut end = start;
        while b.get(end + 1) {
            end += 1;
        }
        if !s.is_empty() {
            s.push(',');
        }
        let _ = write!(s, "{start}");
        if end > start {
            let _ = write!(s, "-{end}");
        }
        i = b.next_set_bit(end + 1);
    }
    s
}

fn vts(list: &[VarType]) -> String {
    let items: Vec<String> = list.iter().map(|t| vt(Some(t))).collect();
    format!("[{}]", items.join(" "))
}

fn vts_opt(list: &[Option<VarType>]) -> String {
    let items: Vec<String> = list.iter().map(|t| vt(t.as_ref())).collect();
    format!("[{}]", items.join(" "))
}

fn char_of(t: CodeType) -> Option<&'static str> {
    Some(match t {
        CodeType::Byte => "B",
        CodeType::Char => "C",
        CodeType::Double => "D",
        CodeType::Float => "F",
        CodeType::Int => "I",
        CodeType::Long => "J",
        CodeType::Short => "S",
        CodeType::Boolean => "Z",
        CodeType::Void => "V",
        CodeType::Group2empty => "G",
        CodeType::Notinitialized => "N",
        CodeType::Address => "A",
        CodeType::Bytechar => "X",
        CodeType::Shortchar => "Y",
        CodeType::Unknown => "U",
        _ => return None,
    })
}

fn family(t: CodeType, dim: i32) -> TypeFamily {
    if dim > 0 {
        return TypeFamily::Object;
    }
    match t {
        CodeType::Byte
        | CodeType::Bytechar
        | CodeType::Shortchar
        | CodeType::Char
        | CodeType::Short
        | CodeType::Int => TypeFamily::Integer,
        CodeType::Double => TypeFamily::Double,
        CodeType::Float => TypeFamily::Float,
        CodeType::Long => TypeFamily::Long,
        CodeType::Boolean => TypeFamily::Boolean,
        CodeType::Null | CodeType::Object | CodeType::Genvar => TypeFamily::Object,
        _ => TypeFamily::Unknown,
    }
}

fn stack_size(t: CodeType, dim: i32) -> i32 {
    if dim > 0 {
        return 1;
    }
    match t {
        CodeType::Double | CodeType::Long => 2,
        CodeType::Void | CodeType::Group2empty => 0,
        _ => 1,
    }
}

/// A `VarType` as the trace prints it: descriptor-like text plus generic arguments.
pub fn vt(t: Option<&VarType>) -> String {
    let Some(t) = t else {
        return "~".to_string();
    };
    let mut s = String::new();
    if let Some(g) = &t.generic {
        match g.wildcard {
            1 => s.push('+'),
            2 => s.push('-'),
            3 => s.push('*'),
            4 => {}
            w => {
                let _ = write!(s, "w{w}");
            }
        }
    }
    for _ in 0..t.array_dim.max(0) {
        s.push('[');
    }
    if t.array_dim < 0 {
        let _ = write!(s, "dim{}", t.array_dim);
    }
    let value = t.value.as_deref();
    match t.type_ {
        CodeType::Object => {
            let _ = write!(s, "L{};", value.unwrap_or("null"));
        }
        CodeType::Genvar => {
            let _ = write!(s, "T{};", value.unwrap_or("null"));
        }
        CodeType::Null => {
            s.push_str("null");
            if let Some(v) = value {
                let _ = write!(s, ":{v}");
            }
        }
        other => match char_of(other) {
            Some(c) if Some(c) == value => s.push_str(c),
            _ => {
                let _ = write!(
                    s,
                    "{}:{}",
                    java_name(&format!("{other:?}")),
                    value.unwrap_or("~")
                );
            }
        },
    }
    if t.type_family != family(t.type_, t.array_dim) {
        let _ = write!(s, "!f={}", java_name(&format!("{:?}", t.type_family)));
    }
    if t.stack_size != stack_size(t.type_, t.array_dim) {
        let _ = write!(s, "!s={}", t.stack_size);
    }
    if let Some(g) = &t.generic {
        s.push('<');
        for (i, arg) in g.arguments.iter().enumerate() {
            if i > 0 {
                s.push(',');
            }
            match arg {
                None => s.push('?'),
                Some(a) => s.push_str(&vt(Some(a))),
            }
        }
        s.push('>');
        if let Some(parent) = &g.parent {
            let _ = write!(s, "{{{}}}", vt(Some(parent)));
        }
    }
    s
}

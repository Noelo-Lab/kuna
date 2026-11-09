//! Reclaiming the statements, expressions and edges a method's passes discard.
//!
//! Java's collector frees what the passes drop on the floor: replaced
//! expressions, copies, the whole trees thrown away by each `finally` re-parse.
//! Their arenas only empty at the end of a run, so the schedule releases them
//! itself: a [`ReclaimWindow`] remembers where each arena stood when a method's
//! schedule started, and [`ReclaimWindow::collect`] — called only where no pass
//! is in flight, with every handle the schedule still holds as a root — marks
//! what is reachable from those roots and releases every other object
//! allocated since. Objects older than the window are neither traversed nor
//! released. Slots keep their index (the identity hash), so a wrongly released
//! object panics on its next use instead of being reused.
//!
//! Besides the given roots, the marker keeps what the direct graphs cached
//! since the window opened (`MethodWrapper::get_or_build_graph`) reference,
//! and the annotation exprents a class-file attribute built and cached since
//! then ([`note_cached_exprents`]): a plugin pass may read them mid-schedule.
//! The variable-version processors each `setVarVersions` replaces go too:
//! only the method's current one (and its type processor) is kept.
//! The traversal destructures every data struct without `..`, so a new field
//! fails to compile until it is traversed or explicitly ignored here.

use crate::annotation_exprent::AnnotationExprentData;
use crate::array_exprent::ArrayExprentData;
use crate::assert_exprent::AssertExprentData;
use crate::assignment_exprent::AssignmentExprentData;
use crate::basic_block_statement::BasicBlockStatementData;
use crate::bytecode_mapping_tracer::BytecodeMappingTracer;
use crate::catch_all_statement::CatchAllStatementData;
use crate::catch_statement::CatchStatementData;
use crate::const_exprent::ConstExprentData;
use crate::decompile_record::DecompileRecord;
use crate::direct_graph::DirectGraph;
use crate::do_statement::DoStatementData;
use crate::dummy_exit_statement::DummyExitStatementData;
use crate::exit_exprent::ExitExprentData;
use crate::exprent::{Exprent, ExprentData, ExprentKind};
use crate::field_exprent::FieldExprentData;
use crate::function_exprent::FunctionExprentData;
use crate::general_statement::GeneralStatementData;
use crate::if_exprent::IfExprentData;
use crate::if_statement::IfStatementData;
use crate::invocation_exprent::InvocationExprentData;
use crate::method_wrapper::MethodWrapper;
use crate::monitor_exprent::MonitorExprentData;
use crate::new_exprent::NewExprentData;
use crate::pattern_exprent::PatternExprentData;
use crate::root_statement::RootStatementData;
use crate::rt::JList;
use crate::sequence_statement::SequenceStatementData;
use crate::stat_edge::{StatEdge, StatEdgeData};
use crate::statement::{Statement, StatementData, StatementKind};
use crate::switch_exprent::SwitchExprentData;
use crate::switch_head_exprent::SwitchHeadExprentData;
use crate::switch_statement::SwitchStatementData;
use crate::synchronized_statement::SynchronizedStatementData;
use crate::var_definition_helper::VarDefinitionHelper;
use crate::var_exprent::VarExprentData;
use crate::var_processor::VarProcessor;
use crate::var_type_processor::VarTypeProcessor;
use crate::var_versions_processor::VarVersionsProcessor;
use crate::yield_exprent::YieldExprentData;
use std::cell::RefCell;

thread_local! {
    static CACHED_EXPRENTS: RefCell<Vec<Exprent>> = const { RefCell::new(Vec::new()) };
}

/// Record exprents a class-file attribute caches outside any statement tree.
pub(crate) fn note_cached_exprents(exprents: impl IntoIterator<Item = Exprent>) {
    CACHED_EXPRENTS.with(|c| {
        let mut c = c.borrow_mut();
        if c.last().is_some_and(|e| !e.is_live()) {
            c.clear();
        }
        c.extend(exprents);
    });
}

fn cached_exprents_since(mark: u32) -> Vec<Exprent> {
    CACHED_EXPRENTS.with(|c| {
        c.borrow()
            .iter()
            .rev()
            .take_while(|e| e.is_live() && e.index() >= mark)
            .copied()
            .collect()
    })
}

/// Arena positions at the start of a root class's processing or writing.
/// Decompile records, variable-definition helpers and bytecode-mapping
/// tracers are only ever held by the pass that made them (a tracer's
/// mapping is copied into the class's source mapper), so everything of those
/// kinds the class allocated is garbage once it is done; only the shared
/// `BytecodeMappingTracer::DUMMY` sink is kept.
#[derive(Clone, Copy)]
pub(crate) struct ClassWindow {
    decompile_record: u32,
    var_definition_helper: u32,
    tracer: u32,
}

impl ClassWindow {
    pub(crate) fn open() -> ClassWindow {
        ClassWindow {
            decompile_record: DecompileRecord::arena_len(),
            var_definition_helper: VarDefinitionHelper::arena_len(),
            tracer: BytecodeMappingTracer::arena_len(),
        }
    }

    pub(crate) fn close(self) {
        DecompileRecord::release_since(self.decompile_record, |_| false);
        VarDefinitionHelper::release_since(self.var_definition_helper, |_| false);
        let dummy = BytecodeMappingTracer::existing_dummy().map(|t| t.index());
        BytecodeMappingTracer::release_since(self.tracer, |i| Some(i) == dummy);
    }
}

/// Arena positions at the start of a method's schedule.
#[derive(Clone, Copy)]
pub(crate) struct ReclaimWindow {
    exprent: u32,
    statement: u32,
    edge: u32,
    direct_graph: u32,
    var_versions: u32,
    var_types: u32,
}

/// Objects released by one [`ReclaimWindow::collect`].
#[derive(Clone, Copy, Default, Debug)]
pub struct Reclaimed {
    pub exprents: u32,
    pub statements: u32,
    pub edges: u32,
}

impl Reclaimed {
    pub fn total(&self) -> u64 {
        self.exprents as u64 + self.statements as u64 + self.edges as u64
    }
}

impl ReclaimWindow {
    pub(crate) fn open() -> ReclaimWindow {
        ReclaimWindow {
            exprent: Exprent::arena_len(),
            statement: Statement::arena_len(),
            edge: StatEdge::arena_len(),
            direct_graph: DirectGraph::arena_len(),
            var_versions: VarVersionsProcessor::arena_len(),
            var_types: VarTypeProcessor::arena_len(),
        }
    }

    fn base(&self) -> u64 {
        self.exprent as u64 + self.statement as u64 + self.edge as u64
    }

    fn total() -> u64 {
        Exprent::arena_len() as u64 + Statement::arena_len() as u64 + StatEdge::arena_len() as u64
    }

    /// Release every statement, expression and edge allocated since the
    /// window opened that `roots` do not reach, and every variable-version
    /// processor but `var_proc`'s current one. The caller guarantees that
    /// nothing else holds such a handle.
    pub(crate) fn collect(&self, roots: &[Statement], var_proc: VarProcessor) -> Reclaimed {
        let current = var_proc.get_var_versions();
        let current_types = current.map(VarVersionsProcessor::get_type_processor);
        VarVersionsProcessor::release_since(self.var_versions, |i| {
            current.is_some_and(|c| c.index() == i)
        });
        VarTypeProcessor::release_since(self.var_types, |i| {
            current_types.is_some_and(|c| c.index() == i)
        });

        let mut marker = Marker::new(*self);
        for &st in roots {
            marker.statement(st);
        }
        for e in cached_exprents_since(self.exprent) {
            marker.exprent(e);
        }
        for graph in MethodWrapper::cached_graphs_since(self.direct_graph) {
            let nodes = graph.with(|d| {
                let mut nodes = d.nodes.to_vec();
                nodes.extend(d.extra_nodes.iter().copied());
                nodes
            });
            for node in nodes {
                for e in node.exprents().iter() {
                    marker.exprent(e);
                }
                marker.statement(node.statement());
                marker.statement_opt(node.block().map(|b| b.0));
            }
        }
        marker.drain();
        marker.release()
    }
}

/// A window plus the growth that triggers the next in-method collection:
/// [`ReclaimScope::checkpoint`] collects only once the window's arenas have
/// grown by at least what survived the previous collection, so its marking
/// cost stays proportional to what the passes allocate.
pub(crate) struct ReclaimScope {
    window: ReclaimWindow,
    next_at: u64,
}

impl ReclaimScope {
    const MIN_GROWTH: u64 = 1 << 16;

    pub(crate) fn open() -> ReclaimScope {
        let window = ReclaimWindow::open();
        ReclaimScope {
            window,
            next_at: window.base() + Self::MIN_GROWTH,
        }
    }

    pub(crate) fn window(&self) -> ReclaimWindow {
        self.window
    }

    /// A point where no pass is in flight and `roots` are the only tree
    /// handles the schedule holds.
    pub(crate) fn checkpoint(&mut self, roots: &[Statement], var_proc: VarProcessor) {
        let now = ReclaimWindow::total();
        if now < self.next_at {
            return;
        }
        let released = self.window.collect(roots, var_proc);
        let live = now - self.window.base() - released.total();
        self.next_at = now + live.max(Self::MIN_GROWTH);
    }
}

struct Marks {
    base: u32,
    bits: Vec<u64>,
}

impl Marks {
    fn new(base: u32, len: u32) -> Marks {
        Marks {
            base,
            bits: vec![0; (len.saturating_sub(base) as usize).div_ceil(64)],
        }
    }

    /// True the first time an object of the window is seen.
    fn insert(&mut self, index: u32) -> bool {
        if index < self.base {
            return false;
        }
        let i = (index - self.base) as usize;
        let Some(word) = self.bits.get_mut(i / 64) else {
            return false;
        };
        let bit = 1u64 << (i % 64);
        let fresh = *word & bit == 0;
        *word |= bit;
        fresh
    }

    fn contains(&self, index: u32) -> bool {
        let i = (index - self.base) as usize;
        self.bits
            .get(i / 64)
            .is_some_and(|word| word & (1u64 << (i % 64)) != 0)
    }

    fn released(&self, len: u32) -> u32 {
        let marked: u32 = self.bits.iter().map(|w| w.count_ones()).sum();
        len.saturating_sub(self.base) - marked
    }
}

struct Marker {
    exprents: Marks,
    statements: Marks,
    edges: Marks,
    exprent_stack: Vec<Exprent>,
    statement_stack: Vec<Statement>,
    edge_stack: Vec<StatEdge>,
}

impl Marker {
    fn new(window: ReclaimWindow) -> Marker {
        Marker {
            exprents: Marks::new(window.exprent, Exprent::arena_len()),
            statements: Marks::new(window.statement, Statement::arena_len()),
            edges: Marks::new(window.edge, StatEdge::arena_len()),
            exprent_stack: Vec::new(),
            statement_stack: Vec::new(),
            edge_stack: Vec::new(),
        }
    }

    fn exprent(&mut self, e: Exprent) {
        if self.exprents.insert(e.index()) {
            self.exprent_stack.push(e);
        }
    }

    fn statement(&mut self, st: Statement) {
        if self.statements.insert(st.index()) {
            self.statement_stack.push(st);
        }
    }

    fn edge(&mut self, edge: StatEdge) {
        if self.edges.insert(edge.index()) {
            self.edge_stack.push(edge);
        }
    }

    fn exprent_opt(&mut self, e: Option<Exprent>) {
        if let Some(e) = e {
            self.exprent(e);
        }
    }

    fn statement_opt(&mut self, st: Option<Statement>) {
        if let Some(st) = st {
            self.statement(st);
        }
    }

    fn exprent_list(&mut self, list: &JList<Exprent>) {
        for &e in list.borrow().iter() {
            self.exprent(e);
        }
    }

    fn drain(&mut self) {
        loop {
            if let Some(e) = self.exprent_stack.pop() {
                e.with(|d| self.trace_exprent(d));
            } else if let Some(st) = self.statement_stack.pop() {
                st.with(|d| self.trace_statement(d));
            } else if let Some(edge) = self.edge_stack.pop() {
                edge.with(|d| self.trace_edge(d));
            } else {
                break;
            }
        }
    }

    fn release(self) -> Reclaimed {
        let reclaimed = Reclaimed {
            exprents: self.exprents.released(Exprent::arena_len()),
            statements: self.statements.released(Statement::arena_len()),
            edges: self.edges.released(StatEdge::arena_len()),
        };
        let Marker {
            exprents,
            statements,
            edges,
            ..
        } = self;
        Exprent::release_since(exprents.base, |i| exprents.contains(i));
        Statement::release_since(statements.base, |i| statements.contains(i));
        StatEdge::release_since(edges.base, |i| edges.contains(i));
        reclaimed
    }

    fn trace_edge(&mut self, d: &StatEdgeData) {
        let StatEdgeData {
            type_: _,
            source,
            destination,
            exceptions: _,
            closure,
            labeled: _,
            explicit: _,
            can_inline: _,
            phantom_continue: _,
        } = d;
        self.statement(*source);
        self.statement(*destination);
        self.statement_opt(*closure);
    }

    fn trace_statement(&mut self, d: &StatementData) {
        let StatementData {
            type_: _,
            id: _,
            map_succ_edges,
            map_pred_edges,
            stats,
            parent,
            first,
            exprents,
            label_edges,
            var_definitions,
            copied: _,
            post,
            last_basic_type: _,
            is_monitor_enter: _,
            is_last_athrow: _,
            contains_monitor_exit: _,
            continue_set,
            phantom: _,
            endpoints: _,
            kind,
            kt: _,
        } = d;
        for edges in map_succ_edges.values().chain(map_pred_edges.values()) {
            for &edge in edges.borrow().iter() {
                self.edge(edge);
            }
        }
        for &st in stats.borrow().as_slice() {
            self.statement(st);
        }
        self.statement_opt(*parent);
        self.statement_opt(*first);
        if let Some(exprents) = exprents {
            self.exprent_list(exprents);
        }
        for &edge in label_edges.borrow().iter() {
            self.edge(edge);
        }
        self.exprent_list(var_definitions);
        self.statement_opt(*post);
        if let Some(continue_set) = continue_set {
            for &st in continue_set.borrow().iter() {
                self.statement(st);
            }
        }
        match kind {
            StatementKind::Root(RootStatementData {
                dummy_exit,
                mt: _,
                comment_lines: _,
                add_error_comment: _,
                flags: _,
            }) => self.statement(dummy_exit.0),
            StatementKind::BasicBlock(BasicBlockStatementData {
                block: _,
                removable_monitorexit: _,
            }) => {}
            StatementKind::Sequence(SequenceStatementData {}) => {}
            StatementKind::DummyExit(DummyExitStatementData { bytecode: _ }) => {}
            StatementKind::General(GeneralStatementData { placeholder: _ }) => {}
            StatementKind::If(IfStatementData {
                iftype: _,
                ifstat,
                elsestat,
                ifedge,
                elseedge,
                negated: _,
                pattern_matched: _,
                has_ppmm: _,
                headexprent,
            }) => {
                self.statement_opt(*ifstat);
                self.statement_opt(*elsestat);
                if let Some(edge) = ifedge {
                    self.edge(*edge);
                }
                if let Some(edge) = elseedge {
                    self.edge(*edge);
                }
                self.exprent_list(headexprent);
            }
            StatementKind::Do(DoStatementData {
                looptype: _,
                init_exprent,
                condition_exprent,
                inc_exprent,
            }) => {
                self.exprent_list(init_exprent);
                self.exprent_list(condition_exprent);
                self.exprent_list(inc_exprent);
            }
            StatementKind::Switch(SwitchStatementData {
                case_statements,
                case_edges,
                case_values,
                case_guards,
                scoped_case_statements,
                default_edge,
                headexprent,
            }) => {
                for &st in case_statements.borrow().iter() {
                    self.statement(st);
                }
                for edges in case_edges.borrow().iter() {
                    for &edge in edges.borrow().iter() {
                        self.edge(edge);
                    }
                }
                for values in case_values.borrow().iter() {
                    for &value in values.borrow().iter() {
                        self.exprent_opt(value);
                    }
                }
                for &guard in case_guards.borrow().iter() {
                    self.exprent_opt(guard);
                }
                for &st in scoped_case_statements.iter() {
                    self.statement(st);
                }
                if let Some(edge) = default_edge {
                    self.edge(*edge);
                }
                self.exprent_list(headexprent);
            }
            StatementKind::Synchronized(SynchronizedStatementData { body, headexprent }) => {
                self.statement_opt(*body);
                self.exprent_list(headexprent);
            }
            StatementKind::Catch(CatchStatementData {
                exctstrings: _,
                vars,
                resources,
            }) => {
                for &var in vars.borrow().iter() {
                    self.exprent(var.0);
                }
                self.exprent_list(resources);
            }
            StatementKind::CatchAll(CatchAllStatementData {
                handler,
                is_finally: _,
                monitor,
                vars,
            }) => {
                self.statement_opt(*handler);
                if let Some(monitor) = monitor {
                    self.exprent(monitor.0);
                }
                for &var in vars.borrow().iter() {
                    self.exprent(var.0);
                }
            }
        }
    }

    fn trace_exprent(&mut self, d: &ExprentData) {
        let ExprentData {
            id: _,
            bytecode: _,
            kind,
            kt: _,
        } = d;
        match kind {
            ExprentKind::Annotation(AnnotationExprentData {
                class_name: _,
                par_names: _,
                par_values,
                did_write_already: _,
            }) => {
                for &e in par_values {
                    self.exprent(e);
                }
            }
            ExprentKind::Array(ArrayExprentData {
                array,
                index,
                hard_type: _,
            }) => {
                self.exprent(*array);
                self.exprent(*index);
            }
            ExprentKind::Assert(AssertExprentData { parameters }) => {
                for &e in parameters {
                    self.exprent_opt(e);
                }
            }
            ExprentKind::Assignment(AssignmentExprentData {
                left,
                right,
                cond_type: _,
            }) => {
                self.exprent(*left);
                self.exprent(*right);
            }
            ExprentKind::Const(ConstExprentData {
                const_type: _,
                value: _,
                bool_permitted: _,
                was_condy: _,
            }) => {}
            ExprentKind::Exit(ExitExprentData {
                exit_type: _,
                value,
                ret_type: _,
                method_descriptor: _,
            }) => self.exprent_opt(*value),
            ExprentKind::Field(FieldExprentData {
                name: _,
                classname: _,
                is_static: _,
                instance,
                descriptor: _,
                force_qualified: _,
                is_qualifier: _,
                was_condy: _,
            }) => self.exprent_opt(*instance),
            ExprentKind::Function(FunctionExprentData {
                func_type: _,
                implicit_type: _,
                lst_operands,
                needs_cast: _,
                disable_newline_group_creation: _,
            }) => self.exprent_list(lst_operands),
            ExprentKind::If(IfExprentData { condition }) => self.exprent(*condition),
            ExprentKind::Invocation(invocation) => {
                let InvocationExprentData {
                    name: _,
                    classname: _,
                    is_static: _,
                    can_ignore_boxing: _,
                    functype: _,
                    instance,
                    desc: _,
                    descriptor: _,
                    string_descriptor: _,
                    invoke_dynamic_class_suffix: _,
                    invocation_type: _,
                    lst_parameters,
                    bootstrap_method: _,
                    bootstrap_arguments: _,
                    generic_args: _,
                    remapped_inst_type: _,
                    force_generic_qualfication: _,
                    generics_map: _,
                    is_invocation_instance: _,
                    is_qualifier: _,
                    boxing: _,
                    is_synthetic_null_check: _,
                    was_lazy_condy: _,
                    kuna_arg_names: _,
                    kuna_needs_type_args: _,
                    kuna_target: _,
                    kuna_cast_lambdas: _,
                } = &**invocation;
                self.exprent_opt(*instance);
                self.exprent_list(lst_parameters);
            }
            ExprentKind::Monitor(MonitorExprentData {
                remove: _,
                mon_type: _,
                value,
            }) => self.exprent(*value),
            ExprentKind::New(NewExprentData {
                constructor,
                new_type: _,
                lst_dims,
                lst_array_elements,
                direct_array_init: _,
                is_var_arg_param: _,
                anonymous: _,
                lambda: _,
                method_reference: _,
                enum_const: _,
                generic_args: _,
                inferred_lambda_type: _,
            }) => {
                if let Some(constructor) = constructor {
                    self.exprent(constructor.0);
                }
                self.exprent_list(lst_dims);
                self.exprent_list(lst_array_elements);
            }
            ExprentKind::Pattern(PatternExprentData {
                data: _,
                var_type: _,
                exprents,
                bound_types: _,
            }) => self.exprent_list(exprents),
            ExprentKind::Switch(SwitchExprentData {
                backing,
                type_: _,
                fallthrough: _,
                standalone: _,
            }) => self.statement(backing.0),
            ExprentKind::SwitchHead(SwitchHeadExprentData { value, case_values }) => {
                self.exprent(*value);
                for values in case_values.borrow().iter() {
                    for &value in values.borrow().iter() {
                        self.exprent_opt(value);
                    }
                }
            }
            ExprentKind::Var(VarExprentData {
                index: _,
                var_type: _,
                definition: _,
                processor: _,
                version: _,
                class_def: _,
                stack: _,
                lvt: _,
                backing: _,
                is_effectively_final: _,
                writing_pattern: _,
                bound_type: _,
                is_intersection_type: _,
                is_catch_temp_var: _,
            }) => {}
            ExprentKind::Yield(YieldExprentData {
                content,
                ret_type: _,
            }) => self.exprent(*content),
        }
    }
}

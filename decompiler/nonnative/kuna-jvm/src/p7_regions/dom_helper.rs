//! Port of `org/jetbrains/java/decompiler/modules/decompiler/decompose/DomHelper.java`.
//!
//! `GraphParser` is not ported (plugin API), so `createStatement` is inherent.
//! kuna fix `irreducible_with_exception_edges`: where Vineflower gives up on an
//! irreducible level with exception edges, a try region is isolated or an
//! unprotected statement split (`kuna_fix_irreducible_with_exception_edges.rs`).
//! kuna fix `synchronized_break_exit`: `build_synchronized` also removes the
//! lock release at a `break` target the body leaves for
//! (`kuna_fix_synchronized_break_exit.rs`).

use std::collections::VecDeque;

use crate::basic_block_statement::BasicBlockStatement;
use crate::cancelation_manager::CancelationManager;
use crate::catch_all_statement::CatchAllStatement;
use crate::catch_statement::CatchStatement;
use crate::control_flow_graph::ControlFlowGraph;
use crate::decompiler_context::DecompilerContext;
use crate::do_statement::DoStatement;
use crate::dom_tracer::DomTracer;
use crate::dummy_exit_statement::DummyExitStatement;
use crate::fast_extended_postdominance_helper::FastExtendedPostdominanceHelper;
use crate::fast_fixed_set::FastFixedSet;
use crate::fast_fixed_set_factory::FastFixedSetFactory;
use crate::general_statement::GeneralStatement;
use crate::i_fernflower_logger::Severity;
use crate::if_statement::IfStatement;
use crate::irreducible_cfg_deobfuscator::IrreducibleCFGDeobfuscator;
use crate::jthrow;
use crate::kuna_fixes::{self, KunaFix};
use crate::label_helper::LabelHelper;
use crate::method_processor::MethodProcessor;
use crate::p7_regions::kuna_fix_irreducible_with_exception_edges::IrreducibleWithExceptionEdges;
use crate::p7_regions::kuna_fix_synchronized_break_exit;
use crate::root_statement::RootStatement;
use crate::rt::{JHashMap, JHashSet, JLinkedHashMap, JLinkedHashSet, JavaEq, Shared};
use crate::sequence_helper::SequenceHelper;
use crate::sequence_statement::SequenceStatement;
use crate::stat_edge::StatEdge;
use crate::statement::{EdgeDirection, Statement};
use crate::strong_connectivity_helper::StrongConnectivityHelper;
use crate::struct_method::StructMethod;
use crate::switch_statement::SwitchStatement;
use crate::synchronized_statement::SynchronizedStatement;
use crate::validation_helper::ValidationHelper;
use crate::vb_style_collection::VBStyleCollection;

/// `HashMap<Integer, Set<Integer>> mapExtPost`, whose runtime class is a
/// `LinkedHashMap` (the first one) or a `HashMap` (every later one); the
/// difference reaches iteration order. It is shared between recursive calls.
pub enum ExtPostMap {
    Linked(JLinkedHashMap<i32, JLinkedHashSet<i32>>),
    Hash(JHashMap<i32, JLinkedHashSet<i32>>),
}

impl ExtPostMap {
    /// `new LinkedHashMap<>()`.
    pub fn new_linked() -> Shared<ExtPostMap> {
        Shared::new(ExtPostMap::Linked(JLinkedHashMap::new()))
    }

    /// `new HashMap<>()`.
    pub fn new_hash() -> Shared<ExtPostMap> {
        Shared::new(ExtPostMap::Hash(JHashMap::new()))
    }

    pub fn is_empty(&self) -> bool {
        match self {
            ExtPostMap::Linked(m) => m.is_empty(),
            ExtPostMap::Hash(m) => m.is_empty(),
        }
    }

    pub fn contains_key(&self, key: &i32) -> bool {
        match self {
            ExtPostMap::Linked(m) => m.contains_key(key),
            ExtPostMap::Hash(m) => m.contains_key(key),
        }
    }

    pub fn get(&self, key: &i32) -> Option<&JLinkedHashSet<i32>> {
        match self {
            ExtPostMap::Linked(m) => m.get(key),
            ExtPostMap::Hash(m) => m.get(key),
        }
    }

    pub fn get_mut(&mut self, key: &i32) -> Option<&mut JLinkedHashSet<i32>> {
        match self {
            ExtPostMap::Linked(m) => m.get_mut(key),
            ExtPostMap::Hash(m) => m.get_mut(key),
        }
    }

    pub fn key_vec(&self) -> Vec<i32> {
        match self {
            ExtPostMap::Linked(m) => m.key_vec(),
            ExtPostMap::Hash(m) => m.key_vec(),
        }
    }

    pub fn remove(&mut self, key: &i32) {
        match self {
            ExtPostMap::Linked(m) => {
                m.remove(key);
            }
            ExtPostMap::Hash(m) => {
                m.remove(key);
            }
        }
    }

    /// `computeIfAbsent(key, k -> new LinkedHashSet<>())`.
    pub fn compute_if_absent(&mut self, key: i32) -> &mut JLinkedHashSet<i32> {
        match self {
            ExtPostMap::Linked(m) => m.compute_if_absent(key, JLinkedHashSet::new),
            ExtPostMap::Hash(m) => m.compute_if_absent(key, JLinkedHashSet::new),
        }
    }

    pub fn put_all(&mut self, entries: JHashMap<i32, JLinkedHashSet<i32>>) {
        let entries = entries.entry_vec();
        match self {
            ExtPostMap::Linked(m) => m.put_all(entries),
            ExtPostMap::Hash(m) => m.put_all(entries),
        }
    }
}

/// `stat.getStats().toString()`.
fn stats_text(stat: Statement) -> String {
    let stats: Vec<String> = stat
        .get_stats()
        .borrow()
        .to_vec()
        .iter()
        .map(|s| s.to_string())
        .collect();
    format!("[{}]", stats.join(", "))
}

pub struct DomHelper;

impl DomHelper {
    pub fn create_statement(&self, graph: ControlFlowGraph, mt: StructMethod) -> RootStatement {
        DomHelper::parse_graph(graph, mt, 0)
    }

    fn graph_to_statement(graph: ControlFlowGraph, mt: StructMethod) -> RootStatement {
        let mut stats: VBStyleCollection<Statement, i32> = VBStyleCollection::new();
        let blocks = graph.get_blocks().borrow().to_vec();

        for &block in &blocks {
            stats.add_with_key(BasicBlockStatement::new(block).0, block.get_id());
        }

        let firstblock = graph.get_first();
        let firstst = stats
            .get_with_key(&firstblock.get_id())
            .expect("first block statement");
        let dummyexit = DummyExitStatement::new();

        let general = if stats.len() > 1 || firstblock.is_successor(firstblock) {
            GeneralStatement::new(firstst, stats.to_vec(), None).0
        } else {
            let root = RootStatement::new(firstst, dummyexit, mt);
            firstst.add_successor(StatEdge::new_closure(
                StatEdge::TYPE_BREAK,
                firstst,
                dummyexit.0,
                Some(root.0),
            ));

            return root;
        };

        let last_id = graph.get_last().get_id();
        for &block in &blocks {
            CancelationManager::check_canceled();
            let stat = stats
                .get_with_key(&block.get_id())
                .expect("block statement");

            for succ in block.get_succs().to_vec() {
                let mut stsucc = stats.get_with_key(&succ.get_id());

                let type_;
                if stsucc == Some(firstst) {
                    type_ = StatEdge::TYPE_CONTINUE;
                    stsucc = Some(general);
                } else if graph.get_finally_exits().borrow().contains(&block) {
                    type_ = StatEdge::TYPE_FINALLYEXIT;
                    stsucc = Some(dummyexit.0);
                } else if succ.get_id() == last_id {
                    type_ = StatEdge::TYPE_BREAK;
                    stsucc = Some(dummyexit.0);
                } else {
                    type_ = StatEdge::TYPE_REGULAR;
                }

                let closure = if type_ == StatEdge::TYPE_REGULAR {
                    None
                } else {
                    Some(general)
                };
                stat.add_successor(StatEdge::new_closure(
                    type_,
                    stat,
                    stsucc.expect("successor statement"),
                    closure,
                ));
            }

            for succex in block.get_succ_exceptions().to_vec() {
                let stsuccex = stats.get_with_key(&succex.get_id()).expect("handler");

                let range = graph
                    .get_exception_range(succex, block)
                    .expect("exception range");
                if !range.is_circular() {
                    stat.add_successor(StatEdge::new_exceptions(
                        stat,
                        stsuccex,
                        range.get_exception_types(),
                    ));
                }
            }
        }

        general.build_continue_set();
        general.build_monitor_flags();
        RootStatement::new(general, dummyexit, mt)
    }

    /// The postdominator tree of a general statement.
    pub fn calc_post_dominators(general: Statement) -> VBStyleCollection<Vec<i32>, i32> {
        let mut lists: JHashMap<Statement, FastFixedSet<Statement>> = JHashMap::new();

        let schelper = StrongConnectivityHelper::new(general);
        let components = schelper.get_components();

        let lst_stats = general.get_post_reverse_post_order_list_exits(Some(
            StrongConnectivityHelper::get_exit_reps(components),
        ));

        let factory = FastFixedSetFactory::create(lst_stats.clone());

        let mut set_flag_nodes = factory.create_copied_set();
        let init_set = factory.create_copied_set();

        for component in components {
            let tmp_set = if StrongConnectivityHelper::is_exit_component(component) {
                let mut tmp_set = factory.create_empty_set();
                for &stat in component {
                    tmp_set.add(stat);
                }
                tmp_set
            } else {
                init_set.get_copy()
            };

            for &stat in component {
                lists.put(stat, tmp_set.get_copy());
            }
        }

        loop {
            for &stat in &lst_stats {
                if !set_flag_nodes.contains(&stat) {
                    continue;
                }

                set_flag_nodes.remove(&stat);
                CancelationManager::check_canceled();

                let mut doms_succs = factory.create_empty_set();

                let successors =
                    stat.get_neighbours(StatEdge::TYPE_REGULAR, EdgeDirection::Forward);

                for (j, succ) in successors.iter().enumerate() {
                    let succlst = lists.get(succ).expect("successor postdominators");

                    if j == 0 {
                        doms_succs.union(succlst);
                    } else {
                        doms_succs.intersection(succlst);
                    }
                }

                if !doms_succs.contains(&stat) {
                    doms_succs.add(stat);
                }

                let unchanged = lists
                    .get(&stat)
                    .is_some_and(|doms| doms_succs.java_equals(doms));
                if !unchanged {
                    lists.put(stat, doms_succs);

                    for pred in stat.get_neighbours(StatEdge::TYPE_REGULAR, EdgeDirection::Backward)
                    {
                        set_flag_nodes.add(pred);
                    }
                }
            }

            if set_flag_nodes.is_empty() {
                break;
            }
        }

        let mut post_dominators: VBStyleCollection<Vec<i32>, i32> = VBStyleCollection::new();

        let lst_rev_post = general.get_reverse_post_order_list();

        let mut map_sort_order: JHashMap<i32, i32> = JHashMap::new();
        for (i, st) in lst_rev_post.iter().enumerate() {
            map_sort_order.put(st.id(), i as i32);
        }

        for &st in &lst_stats {
            let mut lst_posts: Vec<i32> = lists
                .get(&st)
                .expect("postdominators")
                .iter()
                .map(|stt| stt.id())
                .collect();

            lst_posts.sort_by_key(|id| *map_sort_order.get(id).expect("reverse post order index"));

            if lst_posts.len() > 1 && lst_posts[0] == st.id() {
                let head = lst_posts.remove(0);
                lst_posts.push(head);
            }

            post_dominators.add_with_key(lst_posts, st.id());
        }

        post_dominators
    }

    pub fn parse_graph(graph: ControlFlowGraph, mt: StructMethod, iteration: i32) -> RootStatement {
        let root = DomHelper::graph_to_statement(graph, mt);
        root.add_comments_graph(graph);

        let mut tracer = DomTracer::new(&format!("domhelper_{iteration}"), mt);

        if !DomHelper::process_statement(root.0, root, ExtPostMap::new_linked(), &mut tracer) {
            jthrow!("parsing failure!");
        }

        MethodProcessor::set_debug_currently_decompiling(Some(root));

        LabelHelper::low_continue_labels(root.0, &mut JLinkedHashSet::new());

        SequenceHelper::condense_sequences(root.0);
        root.build_monitor_flags();

        DomHelper::build_synchronized(root.0);

        root
    }

    pub fn remove_synchronized_handler(stat: Statement) -> bool {
        let mut res = false;

        let stats = stat.get_stats().borrow().to_vec();
        for st in stats {
            res |= DomHelper::remove_synchronized_handler(st);
        }

        if let Some(sync) = stat.try_as_synchronized() {
            sync.remove_exc();
            res = true;
        }

        res
    }

    pub fn build_synchronized(stat: Statement) -> bool {
        let mut res = false;
        let stats = stat.get_stats().borrow().to_vec();
        for st in stats {
            res |= DomHelper::build_synchronized(st);
        }

        if stat.is_sequence() {
            loop {
                let mut found = false;

                let lst = stat.get_stats();
                let mut i = 0;
                while i + 1 < lst.borrow().len() {
                    let current = lst.borrow().get(i);

                    if current.is_monitor_enter() {
                        let mut next = lst.borrow().get(i + 1);
                        let next_direct = next;

                        while next.is_sequence() {
                            next = next.get_first().expect("first");
                        }

                        if let Some(ca) = next.try_as_catch_all() {
                            let ca_first = ca.get_first().expect("first");
                            let mut head_ok = ca_first.contains_monitor_exit_or_athrow();

                            if !head_ok {
                                head_ok = DomHelper::has_no_exits(ca_first);
                            }

                            if !head_ok {
                                head_ok = ca.is_finally();
                            }

                            if head_ok && ca.get_handler().contains_monitor_exit() {
                                ca_first.mark_monitorexit_dead();
                                ca.get_handler().mark_monitorexit_dead();

                                for edge in
                                    ca.get_successor_edge_view(StatEdge::TYPE_REGULAR).iter()
                                {
                                    edge.get_destination().mark_monitorexit_dead();
                                }

                                let ca_parent = ca.get_parent().expect("parent");
                                for edge in ca_parent
                                    .get_successor_edge_view(StatEdge::TYPE_REGULAR)
                                    .iter()
                                {
                                    edge.get_destination().mark_monitorexit_dead();
                                }

                                if kuna_fixes::enabled(KunaFix::SynchronizedBreakExit) {
                                    kuna_fix_synchronized_break_exit::mark_break_exits(current, ca_first);
                                }

                                current.remove_successor(
                                    current.get_successor_edges(Statement::STATEDGE_DIRECT_ALL)[0],
                                );

                                for edge in
                                    current.get_predecessor_edges(Statement::STATEDGE_DIRECT_ALL)
                                {
                                    current.remove_predecessor(edge);
                                    edge.get_source().change_edge_node(
                                        EdgeDirection::Forward,
                                        edge,
                                        next_direct,
                                    );
                                    next_direct.add_predecessor(edge);
                                }

                                stat.get_stats().borrow_mut().remove_with_key(&current.id());
                                let new_first = stat.get_stats().borrow().get(0);
                                stat.set_first(new_first);

                                let sync =
                                    SynchronizedStatement::new(current, ca_first, ca.get_handler());
                                sync.set_all_parent();

                                let label_edges = JHashSet::from_collection(
                                    ca.get_label_edges().borrow().to_vec(),
                                );
                                for edge in label_edges.to_vec() {
                                    sync.add_labeled_edge(edge);
                                }

                                current.add_successor(StatEdge::new(
                                    StatEdge::TYPE_REGULAR,
                                    current,
                                    ca_first,
                                ));

                                ca.get_parent()
                                    .expect("parent")
                                    .replace_statement(ca.0, sync.0);
                                found = true;
                                res = true;
                                break;
                            }
                        }
                    }
                    i += 1;
                }

                if !found {
                    break;
                }
            }
        }

        res
    }

    /// Whether `head` has no exits (ignoring exceptions) leading outside it.
    fn has_no_exits(head: Statement) -> bool {
        let mut stack: VecDeque<Statement> = VecDeque::new();
        stack.push_back(head);

        while let Some(stat) = stack.pop_front() {
            let sucs = stat.get_successor_edges(Statement::STATEDGE_DIRECT_ALL);
            for suc in sucs {
                if !head.contains_statement(suc.get_destination()) {
                    return false;
                }
            }

            stack.extend(stat.get_stats().borrow().to_vec());
        }

        true
    }

    /// Java: processStatement(Statement, RootStatement, HashMap, DomTracer)
    pub fn process_statement(
        general: Statement,
        root: RootStatement,
        map_ext_post: Shared<ExtPostMap>,
        tracer: &mut DomTracer,
    ) -> bool {
        let mut map_ext_post = map_ext_post;
        tracer.info(general, "process statement");
        if general.is_root() {
            let stat = general.get_first().expect("first");

            if stat.is_basic_block() {
                return true;
            } else {
                let complete = DomHelper::process_statement(stat, root, map_ext_post, tracer);

                if complete {
                    general.replace_statement(stat, stat.get_first().expect("first"));
                }

                return complete;
            }
        }

        let mut map_refreshed = map_ext_post.borrow().is_empty();

        for mapstage in 0..2 {
            let mut reducibility = 0;
            while reducibility < 5 {
                CancelationManager::check_canceled();
                if reducibility > 0 {
                    if IrreducibleCFGDeobfuscator::is_statement_irreducible(general) {
                        if !IrreducibleCFGDeobfuscator::split_irreducible_node(general) {
                            tracer.error(general, "Could not split irreducible flow");
                            DecompilerContext::get_logger().write_message(
                                "Irreducible statement cannot be decomposed!",
                                Severity::Error,
                            );

                            break;
                        } else {
                            tracer
                                .warn(general, &format!("Split irreducible flow: {reducibility}"));
                            if reducibility == 4 && (mapstage == 1 || map_refreshed) {
                                DecompilerContext::get_logger().write_message(
                                    "Irreducible statement too complex to be decomposed!",
                                    Severity::Error,
                                );

                                tracer.error(general, "Flow too complex to be decomposed!");
                                root.add_comment_error("$VF: Irreducible bytecode has more than 5 nodes in sequence and was not entirely decomposed", true);
                            }

                            root.add_comment(
                                "$VF: Irreducible bytecode was duplicated to produce valid code",
                            );
                        }
                    } else if kuna_fixes::enabled(KunaFix::IrreducibleWithExceptionEdges)
                        && (mapstage == 1 || map_refreshed)
                        && IrreducibleWithExceptionEdges::split(general)
                    {
                        tracer.warn(general, &format!("Split irreducible flow: {reducibility}"));
                        if reducibility == 4 {
                            tracer.error(general, "Flow too complex to be decomposed!");
                            root.add_comment_error("$VF: Irreducible bytecode has more than 5 nodes in sequence and was not entirely decomposed", true);
                        }
                        root.add_comment(
                            "$VF: Irreducible bytecode was duplicated to produce valid code",
                        );
                    } else {
                        tracer.error(general, "Flow not irreducible, but could not decompose");
                        if mapstage == 1 || map_refreshed {
                            DecompilerContext::get_logger().write_message(
                                "Statement cannot be decomposed although reducible!",
                                Severity::Error,
                            );
                        }

                        break;
                    }

                    map_ext_post = ExtPostMap::new_hash();
                    map_refreshed = true;
                }

                for i in 0..2 {
                    let forceall = i != 0;

                    if forceall {
                        tracer.info(general, "Force-all iteration");
                    } else {
                        tracer.info(general, "First iteration");
                    }

                    loop {
                        tracer.info(general, "Find simple statements");

                        if DomHelper::find_simple_statements(general, &map_ext_post, tracer) {
                            tracer.success(general, "Found some simple statements");
                            reducibility = 0;
                        }

                        if general.as_general().is_placeholder() {
                            tracer.success(general, "All simple statements found");
                            return true;
                        }

                        let mut stat =
                            DomHelper::find_general_statement(general, forceall, &map_ext_post);
                        let mut isolated = false;
                        if stat.is_none()
                            && forceall
                            && (mapstage == 1 || map_refreshed)
                            && kuna_fixes::enabled(KunaFix::IrreducibleWithExceptionEdges)
                        {
                            stat = IrreducibleWithExceptionEdges::isolate(general);
                            isolated = stat.is_some();
                        }

                        if let Some(stat) = stat {
                            tracer.success_created(
                                general,
                                &format!("Found general statement: {stat} ({})", stats_text(stat)),
                                stat,
                            );
                            let child_map = if !isolated && general.get_first() == Some(stat) {
                                map_ext_post.clone()
                            } else {
                                ExtPostMap::new_hash()
                            };
                            let complete =
                                DomHelper::process_statement(stat, root, child_map, tracer);

                            if complete {
                                general.replace_statement(stat, stat.get_first().expect("first"));
                            } else {
                                tracer.error(
                                    general,
                                    &format!("General statement processing failed! {stat}"),
                                );
                                return false;
                            }

                            tracer.success(
                                general,
                                &format!("General statement processing success {stat}"),
                            );

                            map_ext_post = ExtPostMap::new_hash();
                            map_refreshed = true;
                            reducibility = 0;
                        } else {
                            tracer.info(general, "No new general statement found");
                            break;
                        }
                    }
                }

                reducibility += 1;
            }

            if map_refreshed {
                tracer.error(general, "Map already refreshed");
                break;
            } else {
                map_ext_post = ExtPostMap::new_hash();

                tracer.info(general, "Refreshing map for retry");
            }
        }

        tracer.error(general, "Unable to decompose!");

        false
    }

    fn find_general_statement(
        stat: Statement,
        forceall: bool,
        map_ext_post: &Shared<ExtPostMap>,
    ) -> Option<Statement> {
        let stats = stat.get_stats();
        let mut vb_post: VBStyleCollection<Vec<i32>, i32>;

        if map_ext_post.borrow().is_empty() {
            let mut extpost = FastExtendedPostdominanceHelper::new();
            let ext = extpost.get_extended_postdominators(stat);
            map_ext_post.borrow_mut().put_all(ext);
        }

        if forceall {
            vb_post = VBStyleCollection::new();
            let lst_all = stat.get_post_reverse_post_order_list();

            for &st in &lst_all {
                let set = map_ext_post.borrow().get(&st.id()).map(|s| s.to_vec());
                if let Some(set) = set {
                    let mut element = Vec::with_capacity(set.len());
                    for integer in set {
                        let s = stats.borrow().get_with_key(&integer);
                        let mut is_same = true;
                        if let Some(s) = s {
                            if s.has_successor(StatEdge::TYPE_FINALLYEXIT) {
                                is_same = DomHelper::does_finally_exit_match(st, s);
                            }
                        }

                        if s.is_none() || is_same {
                            element.push(integer);
                        }
                    }
                    vb_post.add_with_key(element, st.id());
                }
            }

            let first_id = stat.get_first().expect("first").id();
            let set_first = map_ext_post.borrow().get(&first_id).map(|s| s.to_vec());
            if let Some(set_first) = set_first {
                for id in set_first {
                    let mut lst = vb_post.get_with_key(&id);
                    if lst.is_none() {
                        vb_post.add_with_key(Vec::new(), id);
                        lst = Some(Vec::new());
                    }
                    let mut lst = lst.expect("post list");
                    lst.push(id);
                    vb_post.put_with_key(lst, id);
                }
            }
        } else {
            vb_post = DomHelper::calc_post_dominators(stat);
        }

        for k in 0..vb_post.len() {
            CancelationManager::check_canceled();
            let headid = vb_post.get_key(k).expect("head id");
            let posts = vb_post.get(k);

            if !map_ext_post.borrow().contains_key(&headid)
                && !(posts.len() == 1 && posts[0] == headid)
            {
                continue;
            }

            let head = stats.borrow().get_with_key(&headid);

            let set_ext_posts = map_ext_post.borrow().get(&headid).map(|s| s.to_vec());

            for &post_id in &posts {
                if post_id != headid
                    && !set_ext_posts
                        .as_ref()
                        .expect("extended postdominators")
                        .contains(&post_id)
                {
                    continue;
                }

                let Some(post) = stats.borrow().get_with_key(&post_id) else {
                    continue;
                };
                let head = head.expect("head");

                let same = post == head;

                if let Some(set_nodes) = DomHelper::collect_region(stat, head, post) {
                    let res = GeneralStatement::new(
                        head,
                        set_nodes,
                        if same { None } else { Some(post) },
                    );
                    stat.collapse_nodes_to_statement(res.0);

                    return Some(res.0);
                }
            }
        }

        None
    }

    /// The body of `findGeneralStatement`'s loop over `(head, post)`: the
    /// nodes of the general statement it would build, if the region is valid
    /// (also used by fix `irreducible_with_exception_edges`).
    pub(crate) fn collect_region(
        stat: Statement,
        head: Statement,
        post: Statement,
    ) -> Option<Vec<Statement>> {
        let stats = stat.get_stats();
        let same = post == head;

        let mut set_nodes: JLinkedHashSet<Statement> = JLinkedHashSet::new();
        let mut set_preds: JHashSet<Statement> = JHashSet::new();

        let mut set_handlers: JLinkedHashSet<Statement> = JLinkedHashSet::new();
        set_handlers.add(head);
        loop {
            let mut handler_found = false;
            for handler in set_handlers.to_vec() {
                if set_nodes.contains(&handler) {
                    continue;
                }

                let mut add_handler = set_nodes.is_empty();
                if !add_handler {
                    let hdsupp =
                        handler.get_neighbours(StatEdge::TYPE_EXCEPTION, EdgeDirection::Backward);
                    add_handler = hdsupp.iter().all(|s| set_nodes.contains(s))
                        && (set_nodes.len() > hdsupp.len() || set_nodes.len() == 1);
                }

                if add_handler {
                    let mut lst_stack: VecDeque<Statement> = VecDeque::new();
                    lst_stack.push_back(handler);

                    while let Some(st) = lst_stack.pop_front() {
                        if !(set_nodes.contains(&st) || (!same && st == post)) {
                            set_nodes.add(st);
                            if st != head {
                                set_preds.add_all(st.get_neighbours(
                                    StatEdge::TYPE_REGULAR,
                                    EdgeDirection::Backward,
                                ));
                            }

                            lst_stack.extend(
                                st.get_neighbours(StatEdge::TYPE_REGULAR, EdgeDirection::Forward),
                            );

                            set_handlers.add_all(
                                st.get_neighbours(StatEdge::TYPE_EXCEPTION, EdgeDirection::Forward),
                            );
                        }
                    }

                    handler_found = true;
                    set_handlers.remove(&handler);
                    break;
                }
            }

            if !handler_found {
                break;
            }
        }

        set_handlers.clear();
        for st in set_nodes.to_vec() {
            set_handlers
                .add_all(st.get_neighbours(StatEdge::TYPE_EXCEPTION, EdgeDirection::Forward));
        }
        set_handlers.retain(|h| !set_nodes.contains(h));

        let mut exceptions_ok = true;
        for handler in set_handlers.to_vec() {
            let exception_range =
                handler.get_neighbours(StatEdge::TYPE_EXCEPTION, EdgeDirection::Backward);
            if !set_nodes.iter().all(|n| exception_range.contains(n)) {
                exceptions_ok = false;
                break;
            }
        }

        if exceptions_ok {
            set_preds.retain(|p| !set_nodes.contains(p));
            if set_preds.is_empty()
                && (set_nodes.len() > 1
                    || head
                        .get_neighbours(StatEdge::TYPE_REGULAR, EdgeDirection::Backward)
                        .contains(&head))
                && set_nodes.len() < stats.borrow().len()
                && DomHelper::check_synchronized_completeness(&set_nodes)
            {
                return Some(set_nodes.to_vec());
            }
        }

        None
    }

    /// `st`: the statement whose postdominance is checked; `s`: the candidate.
    fn does_finally_exit_match(st: Statement, s: Statement) -> bool {
        let finally_exits = s.get_successor_edge_view(StatEdge::TYPE_FINALLYEXIT);
        ValidationHelper::validate_true(
            finally_exits.len() == 1,
            "Must have just one exit from a finally end block",
        );
        for edge in finally_exits.iter() {
            let finally_end = edge.get_source();

            for pred in finally_end.get_all_predecessor_edges() {
                if pred.get_source() == st {
                    return true;
                }
            }
        }

        false
    }

    fn check_synchronized_completeness(set_nodes: &JLinkedHashSet<Statement>) -> bool {
        for &stat in set_nodes.iter() {
            if stat.is_monitor_enter() {
                let lst_succs = stat.get_successor_edges(Statement::STATEDGE_DIRECT_ALL);
                if lst_succs.len() != 1 || lst_succs[0].get_type() != StatEdge::TYPE_REGULAR {
                    return false;
                }

                if !set_nodes.contains(&lst_succs[0].get_destination()) {
                    return false;
                }
            }
        }

        true
    }

    /// Collapses simple statements until none is found; marks the general
    /// statement as placeholder once a single statement covers it.
    fn find_simple_statements(
        stat: Statement,
        map_ext_post: &Shared<ExtPostMap>,
        tracer: &mut DomTracer,
    ) -> bool {
        let mut success = false;

        loop {
            CancelationManager::check_canceled();
            let mut found = false;

            let lst_stats = stat.get_post_reverse_post_order_list();
            for st in lst_stats {
                CancelationManager::check_canceled();
                let Some(result) = DomHelper::detect_statement(st) else {
                    continue;
                };

                if let Some(general) = stat.try_as_general() {
                    if !general.is_placeholder()
                        && result.get_first() == stat.get_first()
                        && stat.get_stats().borrow().len() == result.get_stats().borrow().len()
                    {
                        general.set_placeholder(true);
                    }
                }

                stat.collapse_nodes_to_statement(result);

                tracer.success_created(
                    stat,
                    &format!("Transformed {st} to {result} ({})", stats_text(result)),
                    result,
                );

                if !map_ext_post.borrow().is_empty() {
                    let mut set_old_nodes: JHashSet<i32> = JHashSet::new();
                    let old_stats = result.get_stats().borrow().to_vec();
                    for old in old_stats {
                        set_old_nodes.add(old.id());
                    }

                    let newid = result.id();

                    let keys = map_ext_post.borrow().key_vec();
                    let mut map = map_ext_post.borrow_mut();
                    for key in keys {
                        let set = map.get_mut(&key).expect("postdominance set");

                        let oldsize = set.len();
                        set.retain(|x| !set_old_nodes.contains(x));

                        if set_old_nodes.contains(&key) {
                            let remaining = set.to_vec();
                            map.compute_if_absent(newid).add_all(remaining);
                            map.remove(&key);
                        } else if set.len() < oldsize {
                            set.add(newid);
                        }
                    }
                }

                found = true;
                break;
            }

            if found {
                success = true;
            } else {
                break;
            }
        }

        success
    }

    fn detect_statement(head: Statement) -> Option<Statement> {
        if let Some(res) = DoStatement::is_head(head) {
            return Some(res);
        }

        if let Some(res) = SwitchStatement::is_head(head) {
            return Some(res);
        }

        if let Some(res) = IfStatement::is_head(head) {
            return Some(res);
        }

        if let Some(res) = SequenceStatement::is_head2_block(head) {
            return Some(res);
        }

        if let Some(res) = CatchStatement::is_head(head) {
            return Some(res);
        }

        if let Some(res) = CatchAllStatement::is_head(head) {
            return Some(res);
        }

        None
    }
}

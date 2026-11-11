//! Partitioned runs: one engine instance handles a subset of the root classes.
//!
//! A full run pre-processes every root class of an own unit, then writes them,
//! unit after unit; both phases can read state another root's processing left
//! behind: the wrapper of another root's class node, most visibly the
//! constructor masks of its nested classes that
//! `ExprUtil.getSyntheticParametersMask` reads while a `new Outer.Inner(..)` or
//! `super(..)` is written. A partitioned run pre-processes, in the full run's
//! order, a superset `process` of the roots it writes and delivers (see
//! [`RootSelection`]), and while it runs a detector watches every
//! [`ClassNode::get_wrapper`] read that leaves the current root's tree.
//!
//! When root A reads a node of root B, B is in one of three states:
//! unprocessed, processed, or written (written roots keep their nested
//! wrappers but lose their own). In the full run that state follows from the
//! positions of A and B and the phase; in a partitioned run B may lack
//! processing (not selected) or writing (pre-processed only). A read whose
//! states differ is a *divergent read*, and the driver repeats the run with B
//! added, and written without being delivered when the full run had written
//! it. Unprocessed and written look alike on B's own node (no wrapper).
//! Two reads are order-insensitive by construction and exempt:
//! `NestedMemberAccess.replaceAccessExprent` only uses a foreign method wrapper
//! if it is one of the current root's own (a map keyed by them), and
//! `getSyntheticParametersMask` yields no mask from a root class's own node in
//! any state (only nested classes get masks) and reads, from a nested class, a
//! constructor mask that pre-processing sets and writing never changes.

use std::cell::{Cell, RefCell};
use std::collections::{BTreeMap, BTreeSet};
use std::panic::Location;
use std::sync::Arc;

use crate::classes_processor::{ClassNode, ClassNodeType};

/// The roots a selected run pre-processes (`process`, in the run's own order)
/// and the ones it then writes and delivers (`deliver`, pre-processed too).
/// The roots of `write` are processed and written, in the run's order, without
/// being delivered: a delivered root reads the state their writing leaves.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RootSelection {
    pub process: BTreeSet<String>,
    pub deliver: BTreeSet<String>,
    pub write: BTreeSet<String>,
}

impl RootSelection {
    /// Processes, writes and delivers exactly `roots`.
    pub fn only(roots: impl IntoIterator<Item = String>) -> RootSelection {
        let deliver: BTreeSet<String> = roots.into_iter().collect();
        RootSelection {
            process: deliver.clone(),
            deliver,
            write: BTreeSet::new(),
        }
    }

    pub fn processes(&self, qualified_name: &str) -> bool {
        self.process.contains(qualified_name) || self.writes(qualified_name)
    }

    pub fn writes(&self, qualified_name: &str) -> bool {
        self.deliver.contains(qualified_name) || self.write.contains(qualified_name)
    }

    pub fn delivers(&self, qualified_name: &str) -> bool {
        self.deliver.contains(qualified_name)
    }
}

/// Where a root class sits in a full run: its own unit and its position in
/// the run's order (unit by unit).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RootPos {
    pub unit: usize,
    pub index: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Phase {
    Process,
    Write,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum State {
    Unprocessed,
    Processed,
    Written,
}

/// A wrapper read whose result can differ from the full run's.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct DivergentRead {
    pub phase: Phase,
    /// The root being processed or written.
    pub reader: String,
    /// The root whose tree holds the node read.
    pub owner: String,
    /// The class whose node was read.
    pub class: String,
    pub site: String,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum NodeOwner {
    Unknown,
    Outside,
    Root {
        pos: RootPos,
        processed: bool,
        written: bool,
        is_root_node: bool,
    },
}

struct Watch {
    order: Arc<BTreeMap<String, RootPos>>,
    processed: BTreeSet<String>,
    written: BTreeSet<String>,
    current: Option<(RootPos, Phase)>,
    current_name: String,
    /// Per class node (by arena index): the root whose tree holds it, if any.
    nodes: Vec<NodeOwner>,
    reads: BTreeSet<DivergentRead>,
}

thread_local! {
    static ACTIVE: Cell<bool> = const { Cell::new(false) };
    static WATCH: RefCell<Option<Watch>> = const { RefCell::new(None) };
}

/// Starts watching this thread's run of `selection`; `order` places every
/// root as the full run does.
pub fn watch(order: Arc<BTreeMap<String, RootPos>>, selection: &RootSelection) {
    WATCH.with(|w| {
        *w.borrow_mut() = Some(Watch {
            order,
            processed: selection
                .process
                .iter()
                .chain(&selection.deliver)
                .chain(&selection.write)
                .cloned()
                .collect(),
            written: selection.deliver.union(&selection.write).cloned().collect(),
            current: None,
            current_name: String::new(),
            nodes: Vec::new(),
            reads: BTreeSet::new(),
        })
    });
    ACTIVE.with(|a| a.set(true));
}

/// Stops watching and returns the divergent reads seen.
pub fn unwatch() -> Vec<DivergentRead> {
    ACTIVE.with(|a| a.set(false));
    WATCH
        .with(|w| w.borrow_mut().take())
        .map(|w| w.reads.into_iter().collect())
        .unwrap_or_default()
}

/// The run is now processing or writing `root` (`None`: between roots).
pub fn enter(root: Option<(&str, Phase)>) {
    if !ACTIVE.with(Cell::get) {
        return;
    }
    WATCH.with(|w| {
        if let Some(w) = w.borrow_mut().as_mut() {
            w.current = root.and_then(|(name, phase)| w.order.get(name).map(|&p| (p, phase)));
            if let Some((name, _)) = root {
                name.clone_into(&mut w.current_name);
            }
        }
    });
}

/// The hook in [`ClassNode::get_wrapper`]: `node` is the node whose wrapper is read.
#[inline]
pub fn note_wrapper_read(node: ClassNode, site: &'static Location<'static>) {
    if ACTIVE.with(Cell::get) {
        note(node, site);
    }
}

/// Whether the full run had written `read`'s owner when the read was made:
/// such a read is reproduced only by writing the owner too.
pub fn owner_written_in_full_run(order: &BTreeMap<String, RootPos>, read: &DivergentRead) -> bool {
    match (order.get(&read.owner), order.get(&read.reader)) {
        (Some(&owner), Some(&reader)) => {
            full_run_state(owner, reader, read.phase) == State::Written
        }
        _ => false,
    }
}

fn top_of(node: ClassNode) -> ClassNode {
    let mut top = node;
    while let Some(parent) = top.with(|d| d.parent) {
        top = parent;
    }
    top
}

impl Watch {
    fn owner_of(&mut self, node: ClassNode) -> NodeOwner {
        let slot = node.index() as usize;
        if let Some(&known) = self.nodes.get(slot) {
            if known != NodeOwner::Unknown {
                return known;
            }
        }
        let top = top_of(node);
        let name = top.with(|d| d.class_struct).qualified_name();
        let owner = match self.order.get(&name) {
            Some(&pos) => NodeOwner::Root {
                pos,
                processed: self.processed.contains(&name),
                written: self.written.contains(&name),
                is_root_node: node == top && node.with(|d| d.type_) == ClassNodeType::Root,
            },
            None => NodeOwner::Outside,
        };
        if self.nodes.len() <= slot {
            self.nodes.resize(slot + 1, NodeOwner::Unknown);
        }
        self.nodes[slot] = owner;
        owner
    }
}

/// The state of `owner` when `reader` is in `phase`, in a full run.
fn full_run_state(owner: RootPos, reader: RootPos, phase: Phase) -> State {
    if owner.unit != reader.unit {
        return if owner.unit < reader.unit {
            State::Written
        } else {
            State::Unprocessed
        };
    }
    let earlier = owner.index < reader.index;
    match (phase, earlier) {
        (Phase::Process, true) => State::Processed,
        (Phase::Process, false) => State::Unprocessed,
        (Phase::Write, true) => State::Written,
        (Phase::Write, false) => State::Processed,
    }
}

/// Whether a read of a node in `owner`'s tree can answer differently here.
fn diverges(
    full: State,
    ours: State,
    is_root_node: bool,
    phase: Phase,
    site: &Location<'_>,
) -> bool {
    if full == ours {
        return false;
    }
    let wrapperless = |s: State| s != State::Processed;
    if is_root_node && wrapperless(full) && wrapperless(ours) {
        return false;
    }
    let file = site.file().replace('\\', "/");
    if file.ends_with("p4_calls/nested_member_access.rs") && phase == Phase::Process {
        return false;
    }
    if file.ends_with("substrate/expr_util.rs")
        && (is_root_node || (ours == State::Processed && full == State::Written))
    {
        return false;
    }
    true
}

fn note(node: ClassNode, site: &'static Location<'static>) {
    WATCH.with(|w| {
        let mut guard = w.borrow_mut();
        let Some(w) = guard.as_mut() else {
            return;
        };
        let Some((reader, phase)) = w.current else {
            return;
        };
        let NodeOwner::Root {
            pos,
            processed,
            written,
            is_root_node,
        } = w.owner_of(node)
        else {
            return;
        };
        if pos == reader {
            return;
        }
        let full = full_run_state(pos, reader, phase);
        let ours = match (processed, full) {
            (false, _) => State::Unprocessed,
            (true, State::Written) if !written => State::Processed,
            (true, state) => state,
        };
        if !diverges(full, ours, is_root_node, phase, site) {
            return;
        }
        w.reads.insert(DivergentRead {
            phase,
            reader: w.current_name.clone(),
            owner: top_of(node).with(|d| d.class_struct).qualified_name(),
            class: node.with(|d| d.class_struct).qualified_name(),
            site: format!("{}:{}", site.file(), site.line()),
        });
    });
}

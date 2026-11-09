//! Partitioned runs: one engine instance handles a subset of the root classes.
//!
//! Vineflower pre-processes every root class of a unit before writing any, and
//! both phases can read the state another root's processing left behind: the
//! wrapper of another root's class node, most visibly the constructor masks of
//! its nested classes that `ExprUtil.getSyntheticParametersMask` reads while a
//! `new Outer.Inner(..)` or `super(..)` is written. A partitioned run therefore
//! pre-processes, in the full run's order, a superset `process` of the roots it
//! writes and delivers (see [`RootSelection`]), and while it runs a detector
//! watches every [`ClassNode::get_wrapper`] read that leaves the current root's
//! tree. A read whose answer can differ from the full run's is a *divergent
//! read*: of a root this run did not process that the full run would have
//! (processing happens in order, so: one earlier in the order while
//! processing, any while writing), of a root not yet written here that the
//! full run would have written (an earlier one, while writing), or of a root
//! written here that the full run would not have written yet. The driver then
//! repeats the run with the root read added. Some reads are order-insensitive
//! by construction and exempt: `NestedMemberAccess.replaceAccessExprent` only
//! uses a foreign method wrapper if it is one of the current root's own (a map
//! keyed by them); `getSyntheticParametersMask` on a root class's own node
//! yields no mask whether the wrapper is there or not (only nested classes get
//! masks), and on a nested class it reads a constructor mask that only
//! pre-processing sets (writing never changes it).

use std::cell::{Cell, RefCell};
use std::collections::{BTreeMap, BTreeSet};
use std::panic::Location;
use std::sync::Arc;

use crate::classes_processor::{ClassNode, ClassNodeType};

/// The roots a selected run pre-processes (`process`, in the run's own order)
/// and the ones it then writes and delivers (`deliver`, pre-processed too).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RootSelection {
    pub process: BTreeSet<String>,
    pub deliver: BTreeSet<String>,
}

impl RootSelection {
    /// Processes, writes and delivers exactly `roots`.
    pub fn only(roots: impl IntoIterator<Item = String>) -> RootSelection {
        let deliver: BTreeSet<String> = roots.into_iter().collect();
        RootSelection {
            process: deliver.clone(),
            deliver,
        }
    }

    pub fn processes(&self, qualified_name: &str) -> bool {
        self.process.contains(qualified_name) || self.deliver.contains(qualified_name)
    }

    pub fn delivers(&self, qualified_name: &str) -> bool {
        self.deliver.contains(qualified_name)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Phase {
    Process,
    Write,
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

struct Watch {
    order: Arc<BTreeMap<String, usize>>,
    processed: BTreeSet<String>,
    written: BTreeSet<String>,
    current: Option<(usize, Phase)>,
    current_name: String,
    /// Per class node (by arena index): the root whose tree holds it, if any.
    nodes: Vec<NodeOwner>,
    reads: BTreeSet<DivergentRead>,
}

thread_local! {
    static ACTIVE: Cell<bool> = const { Cell::new(false) };
    static WATCH: RefCell<Option<Watch>> = const { RefCell::new(None) };
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum NodeOwner {
    Unknown,
    Outside,
    Root {
        index: usize,
        processed: bool,
        written: bool,
        is_root_node: bool,
    },
}

/// Starts watching this thread's run of `selection`; `order` ranks every root
/// as the full run visits it.
pub fn watch(order: Arc<BTreeMap<String, usize>>, selection: &RootSelection) {
    WATCH.with(|w| {
        *w.borrow_mut() = Some(Watch {
            order,
            processed: selection
                .process
                .union(&selection.deliver)
                .cloned()
                .collect(),
            written: selection.deliver.clone(),
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
            w.current = root.and_then(|(name, phase)| w.order.get(name).map(|&i| (i, phase)));
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
            Some(&index) => NodeOwner::Root {
                index,
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
            index,
            processed,
            written,
            is_root_node,
        } = w.owner_of(node)
        else {
            return;
        };
        if index == reader {
            return;
        }
        let earlier = index < reader;
        let divergent = match (phase, processed, written) {
            (Phase::Process, processed, _) => !processed && earlier,
            (Phase::Write, false, _) => !(is_root_node && earlier),
            (Phase::Write, true, written) => !written && earlier,
        };
        if !divergent || exempt(site.file(), phase, is_root_node, processed) {
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

fn exempt(file: &str, phase: Phase, is_owner_node: bool, owner_processed: bool) -> bool {
    let file = file.replace('\\', "/");
    (file.ends_with("p4_calls/nested_member_access.rs") && phase == Phase::Process)
        || (file.ends_with("substrate/expr_util.rs") && (is_owner_node || owner_processed))
}

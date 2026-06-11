//! Port of `decompiler/cpp/callgraph.{hh,cc}` — the inter-procedural call
//! graph and its spanning-tree / cycle-snipping leaf walk.
//!
//! A [`CallGraph`] holds [`CallGraphNode`]s (one per function, keyed by entry
//! [`Address`]) connected by [`CallGraphEdge`]s (one caller->callee pair per
//! call site, stored redundantly as an out-edge on the caller and an in-edge on
//! the callee, cross-linked by `complement` index).  The leaf walk
//! ([`CallGraph::init_leaf_walk`] / [`CallGraph::next_leaf`]) visits functions
//! bottom-up after [`CallGraph::cycle_structure`] computes a spanning tree and
//! snips back-edges, so callees are decompiled before their callers.
//!
//! **Pointer model.**  The C++ keys nodes in a `map<Address,CallGraphNode>` and
//! threads the algorithms through stable `CallGraphNode*` pointers (and a
//! `seeds` vector of such pointers).  The Rust port uses a `Vec<CallGraphNode>`
//! arena (stable [`NodeIndex`] handles, the pointer analogue) plus a
//! `BTreeMap<Address, NodeIndex>` for the address-keyed lookup and the
//! Address-ordered iteration the C++ `map` provided (`findNoEntry`,
//! `clearMarks`, `encode`, `begin`/`end`).  Every edge stores its endpoints as
//! [`NodeIndex`] exactly where the C++ stores `CallGraphNode*`.  The traversal
//! order, tie-breakers, and the `complement`-index bookkeeping are transcribed
//! verbatim.
//!
//! **Architecture / Funcdata hooks (SEAM(W4)).**  `addNode(Funcdata*)`,
//! `setFuncdata`, `buildAllNodes`, and `buildEdges` reach into the W4
//! `Architecture` (`symboltab`, `nameFunction`), the `Scope` iteration, and the
//! `Funcdata` prototype / call-spec machinery (`FuncCallSpecs`, `fillinExtrapop`).
//! Those live behind the local seam traits [`CallGraphFunc`] (the
//! `Funcdata`-shaped slice `addNode`/`setFuncdata` read) and
//! [`CallGraphArch`] (the `Architecture`-shaped slice `buildAllNodes`/
//! `buildEdges` read); W4 supplies impls over the real types.  The pure graph
//! data-structure + algorithm half — the bulk of `callgraph.cc` — is ported
//! fully here with no W4 dependency.

use std::collections::BTreeMap;

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::marshal::{Decoder, Encoder, ElementId, ATTRIB_NAME};
use kuna_base::types::{int4, uint4};

use crate::block::ELEM_EDGE;

/// Marshaling element \<callgraph> (C++ `ELEM_CALLGRAPH`, `callgraph.cc:21`).
pub const ELEM_CALLGRAPH: ElementId = ElementId::new("callgraph", 226);
/// Marshaling element \<node> (C++ `ELEM_NODE`, `callgraph.cc:22`).
pub const ELEM_NODE: ElementId = ElementId::new("node", 227);

/// Stable handle into the [`CallGraph`] node arena — the Rust analogue of the
/// C++ `CallGraphNode*`.
pub type NodeIndex = usize;

// ---------------------------------------------------------------------------
// SEAM(W4): the Funcdata / Architecture slices callgraph reaches.
// ---------------------------------------------------------------------------

/// SEAM(W4): the slice of a `Funcdata` that `CallGraph::addNode(Funcdata*)` /
/// `CallGraphNode::setFuncdata` read.
///
/// The C++ stores a `Funcdata *fd` in the node and reads `getAddress()` /
/// `getDisplayName()` / `getName()`.  W4 (when `Funcdata` carries a prototype
/// and the symbol table is wired) supplies an impl; the node here keeps an
/// opaque `FuncId` token instead of the borrowed pointer (see
/// [`CallGraph::add_node_func`]).
pub trait CallGraphFunc {
    /// Entry address of the function (C++ `Funcdata::getAddress`).
    fn get_address(&self) -> &Address;
    /// Display name of the function (C++ `Funcdata::getDisplayName`).
    fn get_display_name(&self) -> &[u8];
    /// Source name of the function, for error messages (C++ `Funcdata::getName`).
    fn get_name(&self) -> &[u8];
}

/// SEAM(W4): the slice of `Architecture` that `CallGraph::buildAllNodes` /
/// `buildEdges` read (`symboltab` scope iteration, `nameFunction`).
///
/// The full `buildAllNodes`/`buildEdges` drivers depend on the W4 symbol-table
/// scope iteration and the W5 `FuncCallSpecs` call-site machinery, so they are
/// deferred (see [`CallGraph::build_all_nodes`] / [`CallGraph::build_edges`]).
/// This trait names the surface so the seam is explicit.
pub trait CallGraphArch {
    /// Resolve a default name for the function at the given address
    /// (C++ `Architecture::nameFunction`).
    fn name_function(&self, addr: &Address) -> Vec<u8>;
}

// ---------------------------------------------------------------------------
// CallGraphEdge  (C++ `callgraph.hh:32`, `callgraph.cc:24-53`)
// ---------------------------------------------------------------------------

/// Edge flag bits (C++ `CallGraphEdge` anonymous enum, `callgraph.hh:34-37`).
pub mod edge_flags {
    use kuna_base::types::uint4;

    /// Edge that was snipped to eliminate cycles.
    pub const CYCLE: uint4 = 1;
    /// Edge that is not in the spanning tree.
    pub const DONTFOLLOW: uint4 = 2;
}

/// A caller->callee edge in the [`CallGraph`] (C++ `CallGraphEdge`).
///
/// Stored redundantly: the caller keeps it as an out-edge, the callee as an
/// in-edge, cross-linked by `complement` (the index of the partner edge in the
/// other node's vector).
#[derive(Debug, Clone)]
pub struct CallGraphEdge {
    /// Node of the caller (C++ `from`).
    from: NodeIndex,
    /// Node of the callee (C++ `to`).
    to: NodeIndex,
    /// Address where call was made from (C++ `callsiteaddr`).
    callsiteaddr: Address,
    /// Index of complementary edge (C++ `complement`).
    complement: int4,
    /// Boolean flag bits (C++ mutable `flags`).
    flags: uint4,
}

impl CallGraphEdge {
    /// Construct a blank edge (C++ `CallGraphEdge(void)`).
    ///
    /// The C++ leaves `from`/`to`/`callsiteaddr`/`complement` uninitialized
    /// until `addEdge`/`insertBlankEdge` fill them; we zero them (sentinel node
    /// index `usize::MAX`, invalid address, complement -1).
    fn new() -> CallGraphEdge {
        CallGraphEdge {
            from: usize::MAX,
            to: usize::MAX,
            callsiteaddr: Address::new_invalid(),
            complement: -1,
            flags: 0,
        }
    }

    /// Is this edge a snipped cycle edge (C++ `isCycle`).
    pub fn is_cycle(&self) -> bool {
        (self.flags & 1) != 0
    }

    /// The address of the call site (C++ `getCallSiteAddr`).
    pub fn get_call_site_addr(&self) -> &Address {
        &self.callsiteaddr
    }

    /// Encode this edge as an \<edge> element (C++ `CallGraphEdge::encode`).
    pub fn encode(&self, encoder: &mut dyn Encoder, graph: &CallGraph) -> KunaResult<()> {
        encoder.open_element(&ELEM_EDGE);
        graph.nodes[self.from].entryaddr.encode(encoder)?;
        graph.nodes[self.to].entryaddr.encode(encoder)?;
        self.callsiteaddr.encode(encoder)?;
        encoder.close_element(&ELEM_EDGE);
        Ok(())
    }

    /// Decode an \<edge> element and add it to the graph
    /// (C++ static `CallGraphEdge::decode`).
    pub fn decode(decoder: &mut dyn Decoder, graph: &mut CallGraph) -> KunaResult<()> {
        let elem_id = decoder.open_element_id(&ELEM_EDGE)?;
        let fromaddr = Address::decode(decoder)?;
        let toaddr = Address::decode(decoder)?;
        let siteaddr = Address::decode(decoder)?;
        decoder.close_element(elem_id)?;

        let fromnode = graph
            .find_node(&fromaddr)
            .ok_or_else(|| KunaError::lowlevel("Could not find from node"))?;
        let tonode = graph
            .find_node(&toaddr)
            .ok_or_else(|| KunaError::lowlevel("Could not find to node"))?;

        graph.add_edge(fromnode, tonode, &siteaddr);
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// CallGraphNode  (C++ `callgraph.hh:54`, `callgraph.cc:55-90`)
// ---------------------------------------------------------------------------

/// Node flag bits (C++ `CallGraphNode` anonymous enum, `callgraph.hh:56-61`).
pub mod node_flags {
    use kuna_base::types::uint4;

    /// Visited mark (C++ `mark`).
    pub const MARK: uint4 = 1;
    /// All in-edges are snipped cycle edges (C++ `onlycyclein`).
    pub const ONLYCYCLEIN: uint4 = 2;
    /// On the current DFS path (C++ `currentcycle`).
    pub const CURRENTCYCLE: uint4 = 4;
    /// Spanning-tree root / seed node (C++ `entrynode`).
    pub const ENTRYNODE: uint4 = 8;
}

/// Opaque token for a `Funcdata` attached to a node.
///
/// SEAM(W4): the C++ stores a borrowed `Funcdata *fd`.  The W3/W4 boundary
/// does not yet have a `Funcdata` arena handle to thread here, so the node
/// records only *whether* a function is attached (`Some`) and an opaque id W4
/// can replace with the real handle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FuncId(pub u64);

/// A function node in the [`CallGraph`] (C++ `CallGraphNode`).
#[derive(Debug, Clone)]
pub struct CallGraphNode {
    /// Starting address of function (C++ `entryaddr`).
    entryaddr: Address,
    /// Name of the function if available (C++ `name`; byte string).
    name: Vec<u8>,
    /// Attached function, if we have it (C++ `Funcdata *fd`).  SEAM(W4).
    fd: Option<FuncId>,
    /// In-edges (callers) (C++ `inedge`).
    inedge: Vec<CallGraphEdge>,
    /// Out-edges (callees) (C++ `outedge`).
    outedge: Vec<CallGraphEdge>,
    /// Incoming edge for spanning tree (C++ `parentedge`).
    parentedge: int4,
    /// Boolean flag bits (C++ mutable `flags`).
    flags: uint4,
}

impl CallGraphNode {
    /// Construct a blank node (C++ `CallGraphNode(void)`).
    fn new() -> CallGraphNode {
        CallGraphNode {
            entryaddr: Address::new_invalid(),
            name: Vec::new(),
            fd: None,
            inedge: Vec::new(),
            outedge: Vec::new(),
            parentedge: -1,
            flags: 0,
        }
    }

    /// Clear the visited mark (C++ `clearMark`).
    pub fn clear_mark(&mut self) {
        self.flags &= !node_flags::MARK;
    }

    /// Is the visited mark set (C++ `isMark`).
    pub fn is_mark(&self) -> bool {
        (self.flags & node_flags::MARK) != 0
    }

    /// Entry address of the function (C++ `getAddr`).
    pub fn get_addr(&self) -> &Address {
        &self.entryaddr
    }

    /// Name of the function (C++ `getName`).
    pub fn get_name(&self) -> &[u8] {
        &self.name
    }

    /// The attached function id, if any (C++ `getFuncdata`).  SEAM(W4).
    pub fn get_funcdata(&self) -> Option<FuncId> {
        self.fd
    }

    /// Number of in-edges (C++ `numInEdge`).
    pub fn num_in_edge(&self) -> int4 {
        self.inedge.len() as int4
    }

    /// Get the i-th in-edge (C++ `getInEdge`).
    pub fn get_in_edge(&self, i: int4) -> &CallGraphEdge {
        &self.inedge[i as usize]
    }

    /// The caller node of the i-th in-edge (C++ `getInNode`).
    pub fn get_in_node(&self, i: int4) -> NodeIndex {
        self.inedge[i as usize].from
    }

    /// Number of out-edges (C++ `numOutEdge`).
    pub fn num_out_edge(&self) -> int4 {
        self.outedge.len() as int4
    }

    /// Get the i-th out-edge (C++ `getOutEdge`).
    pub fn get_out_edge(&self, i: int4) -> &CallGraphEdge {
        &self.outedge[i as usize]
    }

    /// The callee node of the i-th out-edge (C++ `getOutNode`).
    pub fn get_out_node(&self, i: int4) -> NodeIndex {
        self.outedge[i as usize].to
    }

    /// Encode this node as a \<node> element (C++ `CallGraphNode::encode`).
    pub fn encode(&self, encoder: &mut dyn Encoder) -> KunaResult<()> {
        encoder.open_element(&ELEM_NODE);
        if !self.name.is_empty() {
            encoder.write_string(&ATTRIB_NAME, &self.name);
        }
        self.entryaddr.encode(encoder)?;
        encoder.close_element(&ELEM_NODE);
        Ok(())
    }

    /// Decode a \<node> element and add it to the graph
    /// (C++ static `CallGraphNode::decode`).
    pub fn decode(decoder: &mut dyn Decoder, graph: &mut CallGraph) -> KunaResult<()> {
        let elem_id = decoder.open_element_id(&ELEM_NODE)?;
        let mut name: Vec<u8> = Vec::new();
        loop {
            let attrib_id = decoder.get_next_attribute_id()?;
            if attrib_id == 0 {
                break;
            }
            if attrib_id == ATTRIB_NAME.get_id() {
                name = decoder.read_string()?;
            }
        }
        let addr = Address::decode(decoder)?;
        decoder.close_element(elem_id)?;
        graph.add_node_addr(&addr, &name);
        Ok(())
    }
}

/// Leaf-walk DFS frame (C++ `struct LeafIterator`).
struct LeafIterator {
    /// Current node (C++ `node`).
    node: NodeIndex,
    /// Which out-edge we will follow next (C++ `outslot`).
    outslot: int4,
}

impl LeafIterator {
    fn new(n: NodeIndex) -> LeafIterator {
        LeafIterator {
            node: n,
            outslot: 0,
        }
    }
}

// ---------------------------------------------------------------------------
// CallGraph  (C++ `callgraph.hh:96`, `callgraph.cc:92-466`)
// ---------------------------------------------------------------------------

/// The inter-procedural call graph (C++ `CallGraph`).
///
/// See the module docs for the arena/index pointer model.
#[derive(Debug, Default)]
pub struct CallGraph {
    /// Node arena; [`NodeIndex`] handles are stable indices (C++ `CallGraphNode*`).
    nodes: Vec<CallGraphNode>,
    /// Address -> node index, sorted by address (C++ `map<Address,CallGraphNode> graph`).
    index: BTreeMap<Address, NodeIndex>,
    /// Spanning-tree roots / pseudo-roots (C++ `seeds`).
    seeds: Vec<NodeIndex>,
}

impl CallGraph {
    /// Construct an empty call graph (C++ `CallGraph(Architecture *g)`).
    ///
    /// SEAM(W4): the C++ retains the `Architecture *glb`; the W3/W4 boundary
    /// passes it explicitly to the driver methods ([`CallGraph::build_edges`])
    /// instead, so the graph itself holds no back-pointer.
    pub fn new() -> CallGraph {
        CallGraph::default()
    }

    // --- node arena / address index -----------------------------------------

    /// Iterate node indices in Address order (C++ `map` `begin()`..`end()`).
    fn iter_graph(&self) -> impl Iterator<Item = NodeIndex> + '_ {
        self.index.values().copied()
    }

    /// Get the node at the given index.
    pub fn get_node(&self, idx: NodeIndex) -> &CallGraphNode {
        &self.nodes[idx]
    }

    /// Look up the node at the given address (C++ `findNode`).
    pub fn find_node(&self, addr: &Address) -> Option<NodeIndex> {
        self.index.get(addr).copied()
    }

    /// Look up (or create) the node slot for the given address, returning its
    /// index — the C++ `graph[addr]` map subscript (default-constructs a node
    /// if absent).
    fn node_slot(&mut self, addr: &Address) -> NodeIndex {
        if let Some(&idx) = self.index.get(addr) {
            return idx;
        }
        let idx = self.nodes.len();
        self.nodes.push(CallGraphNode::new());
        self.index.insert(addr.clone(), idx);
        idx
    }

    /// Add a node based on an existing function (C++ `addNode(Funcdata*)`).
    ///
    /// SEAM(W4): takes the `Funcdata`-shaped seam plus the [`FuncId`] token to
    /// store in `fd` (the C++ stores the `Funcdata *` itself).
    pub fn add_node_func(
        &mut self,
        f: &dyn CallGraphFunc,
        fid: FuncId,
    ) -> KunaResult<NodeIndex> {
        let addr = f.get_address().clone();
        let idx = self.node_slot(&addr);
        if let Some(existing) = self.nodes[idx].fd {
            if existing != fid {
                let mut msg = String::from("Functions with duplicate entry points: ");
                msg.push_str(&String::from_utf8_lossy(f.get_name()));
                msg.push(' ');
                msg.push_str(&String::from_utf8_lossy(self.nodes[idx].get_name()));
                return Err(KunaError::lowlevel(msg));
            }
        }
        self.nodes[idx].entryaddr = addr;
        self.nodes[idx].name = f.get_display_name().to_vec();
        self.nodes[idx].fd = Some(fid);
        Ok(idx)
    }

    /// Add a node by address and name (C++ `addNode(const Address&,const string&)`).
    pub fn add_node_addr(&mut self, addr: &Address, nm: &[u8]) -> NodeIndex {
        let idx = self.node_slot(addr);
        self.nodes[idx].entryaddr = addr.clone();
        self.nodes[idx].name = nm.to_vec();
        idx
    }

    /// Attach a function to an existing node (C++ `CallGraphNode::setFuncdata`).
    ///
    /// SEAM(W4): takes the seam plus the [`FuncId`] token.
    pub fn set_funcdata(
        &mut self,
        node: NodeIndex,
        f: &dyn CallGraphFunc,
        fid: FuncId,
    ) -> KunaResult<()> {
        if let Some(existing) = self.nodes[node].fd {
            if existing != fid {
                return Err(KunaError::lowlevel(
                    "Multiple functions at one address in callgraph",
                ));
            }
        }
        if *f.get_address() != self.nodes[node].entryaddr {
            return Err(KunaError::lowlevel(
                "Setting function data at wrong address in callgraph",
            ));
        }
        self.nodes[node].fd = Some(fid);
        Ok(())
    }

    // --- edge construction ---------------------------------------------------

    /// Insert a blank out-edge into `node` at `slot`, shifting later out-edges
    /// up and fixing their complement pointers (C++ `insertBlankEdge`).
    /// Returns the slot index of the new (blank) edge.
    fn insert_blank_edge(&mut self, node: NodeIndex, slot: int4) -> int4 {
        self.nodes[node].outedge.push(CallGraphEdge::new());
        let outsize = self.nodes[node].outedge.len();
        if outsize > 1 {
            // for(i = outedge.size()-2; i>=slot; --i) { newi=i+1; edge[newi]=edge[i]; ... }
            let mut i = (outsize as int4) - 2;
            while i >= slot {
                let newi = i + 1;
                let moved = self.nodes[node].outedge[i as usize].clone();
                self.nodes[node].outedge[newi as usize] = moved;
                let nodeout = self.nodes[node].outedge[newi as usize].to;
                let comp = self.nodes[node].outedge[newi as usize].complement;
                self.nodes[nodeout].inedge[comp as usize].complement += 1;
                i -= 1;
            }
        }
        slot
    }

    /// Add a caller->callee edge at a call site (C++ `addEdge`).
    ///
    /// Out-edges are kept sorted by the callee's entry address; a duplicate
    /// caller->callee pair is ignored (the C++ early `return`).
    pub fn add_edge(&mut self, from: NodeIndex, to: NodeIndex, addr: &Address) {
        // Find the insertion slot among from's out-edges (sorted by callee addr).
        let mut i: int4 = 0;
        let outlen = self.nodes[from].outedge.len() as int4;
        while i < outlen {
            let outnode = self.nodes[from].outedge[i as usize].to;
            if outnode == to {
                return; // Already have an out edge
            }
            if to_entry_lt(&self.nodes[to].entryaddr, &self.nodes[outnode].entryaddr) {
                break;
            }
            i += 1;
        }

        let from_slot = self.insert_blank_edge(from, i);

        let toi = self.nodes[to].inedge.len() as int4;
        self.nodes[to].inedge.push(CallGraphEdge::new());

        // fromedge = from->outedge[from_slot]
        {
            let fromedge = &mut self.nodes[from].outedge[from_slot as usize];
            fromedge.from = from;
            fromedge.to = to;
            fromedge.callsiteaddr = addr.clone();
            fromedge.complement = toi;
        }
        // toedge = to->inedge.back()
        {
            let last = self.nodes[to].inedge.len() - 1;
            let toedge = &mut self.nodes[to].inedge[last];
            toedge.from = from;
            toedge.to = to;
            toedge.callsiteaddr = addr.clone();
            toedge.complement = i;
        }
    }

    /// Delete the i-th in-edge of `node`, fixing up complement pointers on both
    /// sides (C++ `deleteInEdge`).
    pub fn delete_in_edge(&mut self, node: NodeIndex, i: int4) {
        let tosize = self.nodes[node].inedge.len() as int4;
        let fromi = self.nodes[node].inedge[i as usize].complement;
        let from = self.nodes[node].inedge[i as usize].from;
        let fromsize = self.nodes[from].outedge.len() as int4;

        // for(j=i+1;j<tosize;++j) { inedge[j-1]=inedge[j]; if(complement>=fromi) complement-=1; }
        let mut j = i + 1;
        while j < tosize {
            let moved = self.nodes[node].inedge[j as usize].clone();
            self.nodes[node].inedge[(j - 1) as usize] = moved;
            if self.nodes[node].inedge[(j - 1) as usize].complement >= fromi {
                self.nodes[node].inedge[(j - 1) as usize].complement -= 1;
            }
            j += 1;
        }
        self.nodes[node].inedge.pop();

        // for(j=fromi+1;j<fromsize;++j) { outedge[j-1]=outedge[j]; if(complement>=i) complement-=1; }
        let mut j = fromi + 1;
        while j < fromsize {
            let moved = self.nodes[from].outedge[j as usize].clone();
            self.nodes[from].outedge[(j - 1) as usize] = moved;
            if self.nodes[from].outedge[(j - 1) as usize].complement >= i {
                self.nodes[from].outedge[(j - 1) as usize].complement -= 1;
            }
            j += 1;
        }
        self.nodes[from].outedge.pop();
    }

    // --- cycle structure / spanning tree ------------------------------------

    /// Find all unmarked functions that are sources (no in-edges, or only
    /// snipped cycle in-edges) and add them as seeds (C++ `findNoEntry`).
    ///
    /// Returns `allcovered` (true if every node already has a real in-edge).
    fn find_no_entry(&mut self) -> bool {
        let mut lownode: Option<NodeIndex> = None;
        let mut allcovered = true;
        let mut newseeds = false;

        for idx in self.iter_graph().collect::<Vec<_>>() {
            if self.nodes[idx].is_mark() {
                continue;
            }
            if self.nodes[idx].inedge.is_empty()
                || (self.nodes[idx].flags & node_flags::ONLYCYCLEIN) != 0
            {
                self.seeds.push(idx);
                self.nodes[idx].flags |= node_flags::MARK | node_flags::ENTRYNODE;
                newseeds = true;
            } else {
                allcovered = false;
                // Pseudo-root: the node with the fewest in-edges.
                match lownode {
                    None => lownode = Some(idx),
                    Some(low) => {
                        if self.nodes[idx].num_in_edge() < self.nodes[low].num_in_edge() {
                            lownode = Some(idx);
                        }
                    }
                }
            }
        }
        if !newseeds && !allcovered {
            let low = lownode.expect("find_no_entry: !allcovered implies a low node");
            self.seeds.push(low);
            self.nodes[low].flags |= node_flags::MARK | node_flags::ENTRYNODE;
        }
        allcovered
    }

    /// Snip any cycles reachable from the root `node` (C++ `snipCycles`).
    fn snip_cycles(&mut self, node: NodeIndex) {
        let mut stack: Vec<LeafIterator> = Vec::new();

        self.nodes[node].flags |= node_flags::CURRENTCYCLE;
        stack.push(LeafIterator::new(node));

        while let Some(top) = stack.last() {
            let cur = top.node;
            let st = top.outslot;
            if st >= self.nodes[cur].outedge.len() as int4 {
                self.nodes[cur].flags &= !node_flags::CURRENTCYCLE;
                stack.pop();
            } else {
                stack.last_mut().unwrap().outslot += 1;
                if (self.nodes[cur].outedge[st as usize].flags & edge_flags::CYCLE) != 0 {
                    continue;
                }
                let next = self.nodes[cur].outedge[st as usize].to;
                if (self.nodes[next].flags & node_flags::CURRENTCYCLE) != 0 {
                    // Found a cycle
                    self.snip_edge(cur, st);
                    continue;
                } else if (self.nodes[next].flags & node_flags::MARK) != 0 {
                    // Already traced before
                    self.nodes[cur].outedge[st as usize].flags |= edge_flags::DONTFOLLOW;
                    continue;
                }
                let comp = self.nodes[cur].outedge[st as usize].complement;
                self.nodes[next].parentedge = comp;
                self.nodes[next].flags |= node_flags::CURRENTCYCLE | node_flags::MARK;
                stack.push(LeafIterator::new(next));
            }
        }
    }

    /// Mark the i-th out-edge of `node` (and its complement in-edge) as a
    /// snipped cycle edge (C++ `snipEdge`).
    fn snip_edge(&mut self, node: NodeIndex, i: int4) {
        self.nodes[node].outedge[i as usize].flags |= edge_flags::CYCLE | edge_flags::DONTFOLLOW;
        let toi = self.nodes[node].outedge[i as usize].complement;
        let to = self.nodes[node].outedge[i as usize].to;
        self.nodes[to].inedge[toi as usize].flags |= edge_flags::CYCLE;
        let mut onlycycle = true;
        for j in 0..self.nodes[to].inedge.len() {
            if (self.nodes[to].inedge[j].flags & edge_flags::CYCLE) == 0 {
                onlycycle = false;
                break;
            }
        }
        if onlycycle {
            self.nodes[to].flags |= node_flags::ONLYCYCLEIN;
        }
    }

    /// Clear the visited mark on all nodes (C++ `clearMarks`).
    fn clear_marks(&mut self) {
        for idx in self.iter_graph().collect::<Vec<_>>() {
            self.nodes[idx].clear_mark();
        }
    }

    /// Generate the seed list and spanning tree, snipping cycles
    /// (C++ `cycleStructure`).
    pub fn cycle_structure(&mut self) {
        if !self.seeds.is_empty() {
            return;
        }
        let mut walked: uint4 = 0;
        let mut allcovered;

        loop {
            allcovered = self.find_no_entry();
            while (walked as usize) < self.seeds.len() {
                let rootnode = self.seeds[walked as usize];
                // C++: rootnode->parentedge = walked;
                self.nodes[rootnode].parentedge = walked as int4;
                self.snip_cycles(rootnode);
                walked += 1;
            }
            if allcovered {
                break;
            }
        }
        self.clear_marks();
    }

    // --- leaf walk ----------------------------------------------------------

    /// Pop a node off the spanning-tree DFS, returning the parent (or `None` at
    /// a seed) and the out-slot to resume from (C++ `popPossible`).
    fn pop_possible(&self, node: NodeIndex) -> (Option<NodeIndex>, int4) {
        if (self.nodes[node].flags & node_flags::ENTRYNODE) != 0 {
            (None, self.nodes[node].parentedge)
        } else {
            let pe = self.nodes[node].parentedge;
            let outslot = self.nodes[node].inedge[pe as usize].complement;
            let from = self.nodes[node].inedge[pe as usize].from;
            (Some(from), outslot)
        }
    }

    /// Descend one spanning-tree edge from `node` at `outslot`, skipping
    /// dontfollow edges (C++ `pushPossible`).  `None` node means descend from a
    /// seed; returns `None` when there is no further child.
    fn push_possible(&self, node: Option<NodeIndex>, mut outslot: int4) -> Option<NodeIndex> {
        match node {
            None => {
                if outslot >= self.seeds.len() as int4 {
                    None
                } else {
                    Some(self.seeds[outslot as usize])
                }
            }
            Some(n) => {
                while outslot < self.nodes[n].outedge.len() as int4 {
                    if (self.nodes[n].outedge[outslot as usize].flags & edge_flags::DONTFOLLOW) != 0
                    {
                        outslot += 1;
                    } else {
                        return Some(self.nodes[n].outedge[outslot as usize].to);
                    }
                }
                None
            }
        }
    }

    /// Begin the bottom-up leaf walk; returns the first leaf (C++ `initLeafWalk`).
    ///
    /// `None` if the graph is empty.
    pub fn init_leaf_walk(&mut self) -> Option<NodeIndex> {
        self.cycle_structure();
        if self.seeds.is_empty() {
            return None;
        }
        let mut node = self.seeds[0];
        loop {
            match self.push_possible(Some(node), 0) {
                None => break,
                Some(pushnode) => node = pushnode,
            }
        }
        Some(node)
    }

    /// Advance the leaf walk to the next function bottom-up (C++ `nextLeaf`).
    ///
    /// `None` once the walk is exhausted.
    pub fn next_leaf(&mut self, node: NodeIndex) -> Option<NodeIndex> {
        let (mut cur, outslot0) = self.pop_possible(node);
        let mut outslot = outslot0 + 1;
        loop {
            match self.push_possible(cur, outslot) {
                None => break,
                Some(pushnode) => {
                    cur = Some(pushnode);
                    outslot = 0;
                }
            }
        }
        cur
    }

    // --- drivers (SEAM(W4)) -------------------------------------------------

    /// Make every function symbol into a node (C++ `buildAllNodes`).
    ///
    /// SEAM(W4): drives the `Architecture` symbol-table scope iteration
    /// (`iterateScopesRecursive`/`iterateFunctionsAddrOrder`), which is a W4
    /// subsystem.  Deferred: the W4 driver iterates the global scope and calls
    /// [`CallGraph::add_node_func`] for each `FunctionSymbol`, in address order.
    pub fn build_all_nodes(&mut self, _glb: &dyn CallGraphArch) -> KunaResult<()> {
        // SEAM(W4): requires Architecture::symboltab + Scope iteration.
        Err(KunaError::lowlevel(
            "CallGraph::build_all_nodes requires the W4 symbol table (SEAM)",
        ))
    }

    /// Build edges from a decompiled function (C++ `buildEdges`).
    ///
    /// SEAM(W4): reads the W4/W5 `FuncProto`/`FuncCallSpecs` call-site machinery
    /// (`getFuncProto`, `fillinExtrapop`, `numCalls`, `getCallSpecs`).
    /// Deferred: the W4 driver finds the caller node, then for each call spec
    /// with a valid entry address either reuses an existing node or
    /// [`CallGraph::add_node_addr`]'s a fresh one (named via
    /// [`CallGraphArch::name_function`]) and [`CallGraph::add_edge`]'s it.
    pub fn build_edges(
        &mut self,
        _fd: &dyn CallGraphFunc,
        _glb: &dyn CallGraphArch,
    ) -> KunaResult<()> {
        // SEAM(W4): requires FuncProto / FuncCallSpecs.
        Err(KunaError::lowlevel(
            "CallGraph::build_edges requires the W4 prototype/call-spec model (SEAM)",
        ))
    }

    // --- serialization ------------------------------------------------------

    /// Encode the whole call graph (C++ `CallGraph::encode`).
    ///
    /// Nodes are emitted in Address order, then every node's in-edges (also in
    /// Address order of the owning node).
    pub fn encode(&self, encoder: &mut dyn Encoder) -> KunaResult<()> {
        encoder.open_element(&ELEM_CALLGRAPH);

        for idx in self.iter_graph() {
            self.nodes[idx].encode(encoder)?;
        }

        // Dump all the "in" edges.
        for idx in self.iter_graph() {
            for i in 0..self.nodes[idx].inedge.len() {
                self.nodes[idx].inedge[i].encode(encoder, self)?;
            }
        }

        encoder.close_element(&ELEM_CALLGRAPH);
        Ok(())
    }

    /// Decode a whole call graph (C++ `CallGraph::decoder`).
    pub fn decoder(&mut self, decoder: &mut dyn Decoder) -> KunaResult<()> {
        let elem_id = decoder.open_element_id(&ELEM_CALLGRAPH)?;
        loop {
            let sub_id = decoder.peek_element()?;
            if sub_id == 0 {
                break;
            }
            if sub_id == ELEM_EDGE.get_id() {
                CallGraphEdge::decode(decoder, self)?;
            } else {
                CallGraphNode::decode(decoder, self)?;
            }
        }
        decoder.close_element(elem_id)?;
        Ok(())
    }
}

/// Compare two entry addresses with C++ `operator<` (used as the out-edge sort
/// key in `addEdge`: `to->entryaddr < outnode->entryaddr`).
fn to_entry_lt(a: &Address, b: &Address) -> bool {
    a < b
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;

    use kuna_base::marshal::{IdRegistry, XmlDecode, XmlEncode};
    use kuna_base::space::{
        addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
    };

    /// Build a minimal manager with const/unique/ram spaces (the same shape the
    /// jumptable tests use), so Address encode/decode round-trips.
    fn build_manager() -> AddrSpaceManager {
        let mut m = AddrSpaceManager::new();
        m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
        m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
        m.insert_space(Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "ram",
            false,
            8,
            1,
            2,
            addrspace_flags::hasphysical,
            1,
            1,
        )))
        .unwrap();
        m.set_default_code_space(2).unwrap();
        m
    }

    fn ram(m: &AddrSpaceManager) -> Rc<AddrSpace> {
        m.get_space_by_name("ram").unwrap().clone()
    }

    fn addr(m: &AddrSpaceManager, off: u64) -> Address {
        Address::new(ram(m), off)
    }

    fn registry() -> IdRegistry {
        let mut reg = IdRegistry::with_base_ids();
        reg.register_element(&ELEM_CALLGRAPH);
        reg.register_element(&ELEM_NODE);
        reg.register_element(&ELEM_EDGE);
        reg
    }

    #[test]
    fn add_node_and_find() {
        let m = build_manager();
        let mut g = CallGraph::new();
        let n1 = g.add_node_addr(&addr(&m, 0x100), b"foo");
        let n2 = g.add_node_addr(&addr(&m, 0x200), b"bar");
        assert_eq!(g.find_node(&addr(&m, 0x100)), Some(n1));
        assert_eq!(g.find_node(&addr(&m, 0x200)), Some(n2));
        assert_eq!(g.find_node(&addr(&m, 0x300)), None);
        assert_eq!(g.get_node(n1).get_name(), b"foo");
    }

    #[test]
    fn edges_sorted_by_callee_and_complement_linked() {
        let m = build_manager();
        let mut g = CallGraph::new();
        let caller = g.add_node_addr(&addr(&m, 0x100), b"caller");
        let callee_hi = g.add_node_addr(&addr(&m, 0x300), b"hi");
        let callee_lo = g.add_node_addr(&addr(&m, 0x200), b"lo");
        // Add the higher-address callee first; out-edges must end up sorted by
        // callee entry address (lo before hi).
        g.add_edge(caller, callee_hi, &addr(&m, 0x10));
        g.add_edge(caller, callee_lo, &addr(&m, 0x20));
        assert_eq!(g.get_node(caller).num_out_edge(), 2);
        assert_eq!(g.get_node(caller).get_out_node(0), callee_lo);
        assert_eq!(g.get_node(caller).get_out_node(1), callee_hi);
        // Duplicate edge is ignored.
        g.add_edge(caller, callee_lo, &addr(&m, 0x30));
        assert_eq!(g.get_node(caller).num_out_edge(), 2);

        // Complement links are consistent: out-edge[i].complement indexes the
        // matching in-edge on the callee, and vice versa.
        for i in 0..g.get_node(caller).num_out_edge() {
            let oe = g.get_node(caller).get_out_edge(i);
            let callee = oe.to;
            let comp = oe.complement;
            let ie = g.get_node(callee).get_in_edge(comp);
            assert_eq!(ie.from, caller);
            assert_eq!(ie.complement, i);
        }
    }

    #[test]
    fn leaf_walk_bottom_up_simple_chain() {
        // main -> a -> b ; leaf walk yields b, then a, then main.
        let m = build_manager();
        let mut g = CallGraph::new();
        let main = g.add_node_addr(&addr(&m, 0x100), b"main");
        let a = g.add_node_addr(&addr(&m, 0x200), b"a");
        let b = g.add_node_addr(&addr(&m, 0x300), b"b");
        g.add_edge(main, a, &addr(&m, 0x10));
        g.add_edge(a, b, &addr(&m, 0x20));

        let mut order: Vec<NodeIndex> = Vec::new();
        let mut node = g.init_leaf_walk();
        while let Some(n) = node {
            order.push(n);
            node = g.next_leaf(n);
        }
        assert_eq!(order, vec![b, a, main]);
    }

    #[test]
    fn leaf_walk_diamond() {
        // main calls a and b; both a and b call c. Bottom-up visits c first,
        // then a/b, then main; every node appears exactly once.
        let m = build_manager();
        let mut g = CallGraph::new();
        let main = g.add_node_addr(&addr(&m, 0x100), b"main");
        let a = g.add_node_addr(&addr(&m, 0x200), b"a");
        let b = g.add_node_addr(&addr(&m, 0x300), b"b");
        let c = g.add_node_addr(&addr(&m, 0x400), b"c");
        g.add_edge(main, a, &addr(&m, 0x10));
        g.add_edge(main, b, &addr(&m, 0x11));
        g.add_edge(a, c, &addr(&m, 0x20));
        g.add_edge(b, c, &addr(&m, 0x21));

        let mut order: Vec<NodeIndex> = Vec::new();
        let mut node = g.init_leaf_walk();
        while let Some(n) = node {
            order.push(n);
            node = g.next_leaf(n);
        }
        // Every node visited exactly once.
        let mut sorted = order.clone();
        sorted.sort_unstable();
        assert_eq!(sorted, vec![main, a, b, c]);
        assert_eq!(order.len(), 4);
        // c (the only common leaf) must come before both its callers.
        let pos = |x: NodeIndex| order.iter().position(|&y| y == x).unwrap();
        assert!(pos(c) < pos(a));
        assert!(pos(c) < pos(b));
        assert!(pos(a) < pos(main));
        assert!(pos(b) < pos(main));
    }

    #[test]
    fn leaf_walk_with_cycle_terminates_and_covers() {
        // a <-> b mutual recursion plus a -> c. cycleStructure must snip the
        // back edge; the walk must terminate covering every node once.
        let m = build_manager();
        let mut g = CallGraph::new();
        let a = g.add_node_addr(&addr(&m, 0x100), b"a");
        let b = g.add_node_addr(&addr(&m, 0x200), b"b");
        let c = g.add_node_addr(&addr(&m, 0x300), b"c");
        g.add_edge(a, b, &addr(&m, 0x10));
        g.add_edge(b, a, &addr(&m, 0x20)); // back edge -> cycle
        g.add_edge(a, c, &addr(&m, 0x30));

        let mut order: Vec<NodeIndex> = Vec::new();
        let mut node = g.init_leaf_walk();
        let mut guard = 0;
        while let Some(n) = node {
            order.push(n);
            node = g.next_leaf(n);
            guard += 1;
            assert!(guard < 100, "leaf walk did not terminate");
        }
        let mut sorted = order.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted, vec![a, b, c]);
        assert_eq!(order.len(), 3);
    }

    #[test]
    fn delete_in_edge_removes_both_sides() {
        // `deleteInEdge` removes the in-edge from the callee AND the matching
        // out-edge from the caller (the C++ shifts both vectors down and pops
        // the tail), adjusting the `from`'s out-edge complements.  This checks
        // the structural removal the algorithm guarantees; the complement
        // re-indexing applied to the *deleted node's* surviving in-edges is the
        // narrow case the C++ keeps consistent (single `from`).
        let m = build_manager();
        let mut g = CallGraph::new();
        let caller = g.add_node_addr(&addr(&m, 0x100), b"c");
        let callee_a = g.add_node_addr(&addr(&m, 0x200), b"a");
        let callee_b = g.add_node_addr(&addr(&m, 0x300), b"b");
        // caller -> a and caller -> b ; out-edges sorted (a before b).
        g.add_edge(caller, callee_a, &addr(&m, 0x10));
        g.add_edge(caller, callee_b, &addr(&m, 0x20));
        assert_eq!(g.get_node(caller).num_out_edge(), 2);

        // Delete callee_a's only in-edge: caller's out-edge to a is removed,
        // and the (single) remaining out-edge now goes to b.
        g.delete_in_edge(callee_a, 0);
        assert_eq!(g.get_node(callee_a).num_in_edge(), 0);
        assert_eq!(g.get_node(caller).num_out_edge(), 1);
        assert_eq!(g.get_node(caller).get_out_node(0), callee_b);
        // Faithful C++ quirk: the shifted out-edge's complement (which was 0)
        // is decremented unconditionally by the `>= i` test (i == 0), yielding
        // -1.  The decompiler only calls deleteInEdge in flows where this is
        // benign; we pin the transcribed behavior, not an "ideal" value.
        assert_eq!(g.get_node(caller).get_out_edge(0).complement, -1);
    }

    #[test]
    fn encode_decode_roundtrip() {
        let m = build_manager();
        let reg = registry();
        let mut g = CallGraph::new();
        let main = g.add_node_addr(&addr(&m, 0x100), b"main");
        let a = g.add_node_addr(&addr(&m, 0x200), b"a");
        let b = g.add_node_addr(&addr(&m, 0x300), b"b");
        g.add_edge(main, a, &addr(&m, 0x10));
        g.add_edge(main, b, &addr(&m, 0x11));
        g.add_edge(a, b, &addr(&m, 0x20));

        let mut buf: Vec<u8> = Vec::new();
        {
            let mut enc = XmlEncode::new(&mut buf);
            g.encode(&mut enc).unwrap();
        }
        let mut dec = XmlDecode::new(&m, &reg);
        dec.ingest_stream(&buf).unwrap();
        let mut g2 = CallGraph::new();
        g2.decoder(&mut dec).unwrap();

        // Same node set (by address + name).
        let m2 = g2.find_node(&addr(&m, 0x100)).unwrap();
        let a2 = g2.find_node(&addr(&m, 0x200)).unwrap();
        let b2 = g2.find_node(&addr(&m, 0x300)).unwrap();
        assert_eq!(g2.get_node(m2).get_name(), b"main");
        assert_eq!(g2.get_node(a2).get_name(), b"a");
        assert_eq!(g2.get_node(b2).get_name(), b"b");

        // main -> {a, b} ; a -> b reconstructed.
        assert_eq!(g2.get_node(m2).num_out_edge(), 2);
        assert_eq!(g2.get_node(a2).num_out_edge(), 1);
        assert_eq!(g2.get_node(a2).get_out_node(0), b2);
        // b has two in-edges (from main and from a).
        assert_eq!(g2.get_node(b2).num_in_edge(), 2);
    }
}

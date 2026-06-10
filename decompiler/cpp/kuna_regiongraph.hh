/* ###
 * kuna-owned: graph substrate for the angr RegionIdentifier port (S7).
 *
 * This file is NOT part of upstream Ghidra.  It provides the mutable,
 * deterministic digraph machinery that decompiler/cpp/kuna_regionid.cc (the
 * port of angr/analyses/decompiler/region_identifier.py) collapses while
 * identifying single-entry regions.  The decompiler's own BlockGraph cannot
 * serve: region identification destructively merges and replaces nodes, and
 * the basic-block graph (Funcdata::bblocks) must never be mutated by an
 * analysis-only pass.
 *
 * Contents (and their angr counterparts in angr/utils/graph.py + doms.py):
 *   KunaRegionNode             - graph node handle: a real FlowBlock leaf, a
 *                                synthetic (test-only) addr node, a merged
 *                                chain (angr MultiNode), a collapsed region
 *                                wrapper, or the dummy end node.
 *   KunaNodeOrder              - the global (addr, creation-ident) strict weak
 *                                order.  angr's determinism comes from Python
 *                                dict insertion order plus stable sorts by
 *                                node.addr; kuna replaces that with this
 *                                comparator.  Policy: no container keyed by
 *                                raw pointer order is ever ITERATED -- pointer
 *                                sets/maps are allowed for membership tests
 *                                only.
 *   KunaRegionGraph            - adjacency-list digraph (networkx.DiGraph
 *                                analog; self-loops first-class, edges
 *                                deduplicated, no edge metadata -- the only
 *                                edge attribute angr reads, the 'type' tag,
 *                                cannot occur on a p-code CFG where every edge
 *                                is a 'transition').
 *   kunaDfsBackEdges           - dfs_back_edges
 *   kunaDfsPostorderDeterministic - GraphUtils.dfs_postorder_nodes_deterministic
 *   kunaQuasiTopoSort          - GraphUtils.quasi_topological_sort_nodes
 *                                (+ _append_scc, incl. panic mode)
 *   kunaSubgraphBetweenNodes   - subgraph_between_nodes (the per-successor
 *                                has_path probe is replaced by one reverse BFS
 *                                from the frontier -- equivalent because the
 *                                stripped copy is never mutated during BFS)
 *   kunaImmediateDominators    - networkx.immediate_dominators
 *                                (Cooper-Harvey-Kennedy over deterministic RPO)
 *   kunaDominates              - angr.utils.graph.dominates
 *   KunaIncrementalDominators  - angr.utils.doms.IncrementalDominators
 */
#ifndef __KUNA_REGIONGRAPH_HH__
#define __KUNA_REGIONGRAPH_HH__

#include "block.hh"

namespace ghidra {

class KunaGraphRegion;		// Defined in kuna_regionid.hh

/// \brief (kuna) A node handle in the region-identification working graph
///
/// Mirrors angr's TNode = Block | MultiNode | GraphRegion (+ the dummy end
/// node fabricated by acyclic-region identification).  One concrete class
/// with a kind tag instead of a class hierarchy; all instances are owned by
/// the KunaRegionIdentifier's node pool and live until it is destroyed.
class KunaRegionNode {
public:
  enum node_kind {
    k_block = 0,		///< Leaf: one basic block (real FlowBlock or synthetic test node)
    k_multi = 1,		///< Merged single-out -> single-in chain (angr MultiNode)
    k_region = 2,		///< Wrapper for a collapsed KunaGraphRegion
    k_dummy = 3			///< Dummy end node (angr Block(-1,-1))
  };
private:
  friend class KunaRegionIdentifier;
  node_kind kind;		///< What this node is
  uintb addr;			///< Sort address (block start / first chain member / region head)
  uint4 ident;			///< Creation index: deterministic tiebreaker for equal addresses
  const FlowBlock *block;	///< k_block: the wrapped CFG block (null for synthetic test nodes)
  vector<KunaRegionNode *> chain;	///< k_multi: member nodes in execution order
  KunaGraphRegion *region;	///< k_region: the collapsed region payload
public:
  KunaRegionNode(node_kind k,uintb a,uint4 id)
    : kind(k), addr(a), ident(id), block((const FlowBlock *)0), region((KunaGraphRegion *)0) {}
  node_kind getKind(void) const { return kind; }	///< Get the node kind
  uintb getAddr(void) const { return addr; }		///< Get the sort address
  uint4 getIdent(void) const { return ident; }		///< Get the creation index
  const FlowBlock *getBlock(void) const { return block; }	///< k_block: wrapped block (may be null)
  const vector<KunaRegionNode *> &getChain(void) const { return chain; }	///< k_multi: members
  KunaGraphRegion *getRegion(void) const { return region; }	///< k_region: payload
  bool isRegion(void) const { return (kind == k_region); }	///< Is this a collapsed region?
  bool isMulti(void) const { return (kind == k_multi); }	///< Is this a merged chain?
  bool isDummy(void) const { return (kind == k_dummy); }	///< Is this the dummy end node?
};

/// \brief (kuna) The global deterministic strict weak order over region nodes
struct KunaNodeOrder {
  bool operator()(const KunaRegionNode *a,const KunaRegionNode *b) const {
    if (a->getAddr() != b->getAddr()) return (a->getAddr() < b->getAddr());
    return (a->getIdent() < b->getIdent());
  }
};

typedef set<KunaRegionNode *,KunaNodeOrder> KunaNodeSet;	///< Deterministically iterable node set

/// \brief (kuna) A mutable directed graph over KunaRegionNode handles
///
/// networkx.DiGraph analog: nodes carry insertion-ordered predecessor and
/// successor lists (mirroring networkx adjacency-dict insertion order), edges
/// are deduplicated on insert, self-loops are supported, and whole-graph node
/// iteration is in KunaNodeOrder.  Copy construction/assignment perform a
/// full structural copy (the node handles themselves are shared, never owned).
class KunaRegionGraph {
public:
  /// \brief Adjacency record for one node
  struct Adjacency {
    vector<KunaRegionNode *> preds;	///< In-neighbors in edge insertion order
    vector<KunaRegionNode *> succs;	///< Out-neighbors in edge insertion order
  };
  typedef map<KunaRegionNode *,Adjacency,KunaNodeOrder> NodeMap;
  typedef NodeMap::const_iterator const_iterator;
private:
  NodeMap nodes;		///< All nodes with their adjacency
public:
  bool containsNode(const KunaRegionNode *n) const;	///< Is the node in the graph?
  void addNode(KunaRegionNode *n);			///< Add a node (idempotent)
  void removeNode(KunaRegionNode *n);			///< Remove node and all incident edges
  bool hasEdge(const KunaRegionNode *a,const KunaRegionNode *b) const;	///< Does edge a->b exist?
  void addEdge(KunaRegionNode *a,KunaRegionNode *b);	///< Add edge (idempotent, auto-adds endpoints)
  void removeEdge(KunaRegionNode *a,KunaRegionNode *b);	///< Remove edge if present
  int4 sizeIn(const KunaRegionNode *n) const;		///< In-degree
  int4 sizeOut(const KunaRegionNode *n) const;		///< Out-degree
  const vector<KunaRegionNode *> &getPreds(const KunaRegionNode *n) const;	///< In-neighbors (insertion order)
  const vector<KunaRegionNode *> &getSuccs(const KunaRegionNode *n) const;	///< Out-neighbors (insertion order)
  void getSortedSuccs(const KunaRegionNode *n,vector<KunaRegionNode *> &res) const;	///< Out-neighbors in KunaNodeOrder
  void inEdges(const KunaRegionNode *n,vector<pair<KunaRegionNode *,KunaRegionNode *> > &res) const;	///< Snapshot in-edges
  void outEdges(const KunaRegionNode *n,vector<pair<KunaRegionNode *,KunaRegionNode *> > &res) const;	///< Snapshot out-edges
  void allEdges(vector<pair<KunaRegionNode *,KunaRegionNode *> > &res) const;	///< Snapshot all edges (node-major deterministic order)
  void getNodes(vector<KunaRegionNode *> &res) const;	///< Snapshot all nodes in KunaNodeOrder
  void induced(const KunaNodeSet &keep,KunaRegionGraph &res) const;	///< Build induced subgraph copy
  void buildReversed(KunaRegionGraph &res) const;	///< Build edge-reversed copy (same node set)
  const_iterator beginNode(void) const { return nodes.begin(); }	///< Begin deterministic node iteration
  const_iterator endNode(void) const { return nodes.end(); }	///< End node iteration
  int4 numNodes(void) const { return nodes.size(); }	///< Number of nodes
  int4 numEdges(void) const;				///< Number of edges
  void clear(void) { nodes.clear(); }			///< Remove all nodes and edges
};

/// \brief (kuna) Find back edges via iterative DFS (port of angr dfs_back_edges)
///
/// Children are visited in KunaNodeOrder (angr sorts by node.addr).  Each
/// (source,target) pair where target is on the current DFS stack-path is
/// reported, in discovery order.
extern void kunaDfsBackEdges(const KunaRegionGraph &g,KunaRegionNode *start,
			     vector<pair<KunaRegionNode *,KunaRegionNode *> > &res);

/// \brief (kuna) Deterministic DFS post-order from a source node
///
/// Port of GraphUtils.dfs_postorder_nodes_deterministic: explicit
/// (node, preVisit) stack; successors are pushed in ascending KunaNodeOrder so
/// the largest-addressed successor is explored first (matches angr exactly).
extern void kunaDfsPostorderDeterministic(const KunaRegionGraph &g,KunaRegionNode *source,
					  vector<KunaRegionNode *> &res);

/// \brief (kuna) Quasi-topological sort of all nodes (works on cyclic graphs)
///
/// Port of GraphUtils.quasi_topological_sort_nodes with nodes=None and
/// loop_heads=None (the only form RegionIdentifier uses): collapse non-trivial
/// SCCs to placeholders, topologically sort the condensation, and expand each
/// SCC recursively after picking its loop head (_append_scc), including the
/// panic mode that aggressively strips back edges from huge dense SCCs.
/// Appends the ordered nodes to \e res.
extern void kunaQuasiTopoSort(const KunaRegionGraph &g,vector<KunaRegionNode *> &res);

/// \brief (kuna) Slice of \e g from \e source up to (optionally including) \e frontier
///
/// Port of subgraph_between_nodes: copy the graph, cut edges into \e source,
/// BFS forward keeping only paths that can still reach the frontier, then
/// iteratively prune dangling nodes.  Throws LowlevelError if \e source or a
/// frontier node is not in the graph (angr raises KeyError).
extern void kunaSubgraphBetweenNodes(const KunaRegionGraph &g,KunaRegionNode *source,
				     const KunaNodeSet &frontier,bool includeFrontier,
				     KunaRegionGraph &res);

/// \brief (kuna) Immediate dominators of all nodes reachable from \e start
///
/// networkx.immediate_dominators analog (Cooper-Harvey-Kennedy iterative
/// algorithm over a deterministic reverse post-order).  Unreachable nodes are
/// absent from \e idom; idom[start] == start.
extern void kunaImmediateDominators(const KunaRegionGraph &g,KunaRegionNode *start,
				    map<KunaRegionNode *,KunaRegionNode *> &idom);

/// \brief (kuna) Does \e dominator dominate \e node under the idom map? (port of utils.graph.dominates)
extern bool kunaDominates(const map<KunaRegionNode *,KunaRegionNode *> &idom,
			  KunaRegionNode *dominator,KunaRegionNode *node);

/// \brief (kuna) Incrementally maintained (post-)dominators with dominance frontiers
///
/// Port of angr.utils.doms.IncrementalDominators.  Holds a live pointer to the
/// graph it was built over; the graph must only ever change by replacing a set
/// of nodes with a single new node (graphUpdated()), exactly as in angr.
/// Dominance frontiers are computed lazily on first df() use and patched
/// incrementally afterwards.
class KunaIncrementalDominators {
  const KunaRegionGraph *graph;	///< Live underlying graph
  KunaRegionNode *start;	///< Tree root (entry node; end node when post)
  bool post;			///< True if computing post-dominators
  map<KunaRegionNode *,KunaRegionNode *> doms;	///< node -> immediate (post-)dominator
  bool dfsValid;		///< Has the dominance-frontier map been initialized?
  map<KunaRegionNode *,KunaNodeSet> dfs;	///< node -> dominance frontier
  bool invertedValid;		///< Has the inverted dominator tree been built?
  map<KunaRegionNode *,vector<KunaRegionNode *> > invertedDomTree;	///< dominator -> dominatees
  void initDoms(map<KunaRegionNode *,KunaRegionNode *> &res) const;	///< Compute doms from scratch
  void initDfs(map<KunaRegionNode *,KunaNodeSet> &res) const;	///< Compute frontiers from scratch
  void updateInvertedDomtree(void);	///< Build the inverted tree if not yet built
public:
  KunaIncrementalDominators(const KunaRegionGraph *g,KunaRegionNode *st,bool isPost);
  KunaRegionNode *idom(KunaRegionNode *n) const;	///< Immediate dominator (null if absent)
  const KunaNodeSet &df(KunaRegionNode *n);		///< Dominance frontier (empty set if absent)
  bool dominates(KunaRegionNode *dominator,KunaRegionNode *n) const;	///< Does \e dominator (post-)dominate \e n?
  void graphUpdated(KunaRegionNode *newNode,const KunaNodeSet &replacedNodes,
		    KunaRegionNode *replacedHead);	///< Patch after nodes were replaced by \e newNode
  void verify(void) const;	///< Recompute from scratch and compare (port of _debug_check)
};

} // End namespace ghidra
#endif

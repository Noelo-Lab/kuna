/* ###
 * kuna-owned: region identification (S7), ported from the angr decompiler.
 *
 * This file is NOT part of upstream Ghidra.  It is a faithful, ANALYSIS-ONLY
 * C++ reimplementation of angr's RegionIdentifier
 * (angr/analyses/decompiler/region_identifier.py) and the read-only subset of
 * its GraphRegion result type (angr/analyses/decompiler/graph_region.py):
 *
 *   A region is a single-entry subgraph of control flow.  The identifier
 *   recursively finds the smallest possible regions in a function graph and
 *   produces a nested KunaGraphRegion tree whose graph nodes are either basic
 *   blocks (or merged block chains) or sub-regions.  Cyclic regions are found
 *   first via DFS back edges + loop refinement; acyclic regions via
 *   dominance-frontier checks while walking up the post-dominator tree.
 *
 * Inputs.  Either the decompiler's basic-block graph (buildFromBlockGraph();
 * Funcdata::getBasicBlocks() is read, NEVER mutated -- the identifier
 * collapses its own private KunaRegionGraph copy) or a synthetic graph built
 * node-by-node for unit tests (addSyntheticBlock()/addSyntheticEdge(),
 * mirroring angr's DummyNode tests in tests/analyses/decompiler/
 * test_structurer.py).
 *
 * Intentional divergences from angr (analysis-only: kuna never mutates
 * p-code or statements; see docs/regions.md):
 *   - _ensure_jump_at_loop_exit_ends is a no-op (it appends AIL Jump
 *     statements for the later structurer).
 *   - _refine_loop_successors_to_guarded_successors is omitted, so the angr
 *     option force_loop_single_exit (default True there) is effectively
 *     false here: a multi-successor loop region keeps its successors instead
 *     of being rewritten into a guarded ConditionNode tree (which requires
 *     synthesizing statements via the condition processor).
 *   - test_reducibility() and the structurer-phase GraphRegion editing API
 *     (replace_region etc.) are not ported.
 * Other angr options keep their defaults: largest_successor_tree_outside_loop
 * (true), refine_loops_with_single_successor (false), complete_successors
 * (false).
 *
 * Determinism: angr leans on Python dict insertion order and stable sorts by
 * node address; kuna substitutes the global (addr, creation-ident) order
 * (KunaNodeOrder).  Where addresses tie, results may be a different but
 * still-valid region tree.
 *
 * No ElementId is needed: nothing here is marshaled and no ArchOption is
 * registered (the feature is exposed through read-only console commands).
 */
#ifndef __KUNA_REGIONID_HH__
#define __KUNA_REGIONID_HH__

#include "kuna_regiongraph.hh"
#include "ifacedecomp.hh"

namespace ghidra {

/// \brief (kuna) One identified region: port of angr's GraphRegion (read-only subset)
///
/// Owned by the KunaRegionIdentifier that created it; valid for its lifetime.
class KunaGraphRegion {
  friend class KunaRegionIdentifier;
  KunaRegionNode *head;		///< Entry node (lives inside \e graph)
  KunaRegionNode *wrapper;	///< The k_region node wrapping this region in the parent graph
  KunaRegionGraph graph;	///< The region's internal subgraph
  bool hasSuccs;		///< Is successor info present? (false only for top-level wrap regions)
  KunaNodeSet successors;	///< Nodes outside the region that region nodes flow to
  KunaRegionGraph graphWithSuccessors;	///< \e graph extended with the successor frontier (valid iff hasSuccs)
  bool hasFull;			///< Is fullGraph present? (cyclic regions only)
  KunaRegionGraph fullGraph;	///< All edges among loop nodes incl. entries/exits (valid iff hasFull)
  bool cyclic;			///< Does this region contain a loop?
  bool cyclicAncestor;		///< Is some ancestor region cyclic?
public:
  KunaGraphRegion(void) {
    head = (KunaRegionNode *)0; wrapper = (KunaRegionNode *)0;
    hasSuccs = false; hasFull = false; cyclic = false; cyclicAncestor = false;
  }
  KunaRegionNode *getHead(void) const { return head; }		///< Entry node
  KunaRegionNode *getWrapper(void) const { return wrapper; }	///< Wrapper node in the parent graph
  const KunaRegionGraph &getGraph(void) const { return graph; }	///< Internal subgraph
  bool hasSuccessorInfo(void) const { return hasSuccs; }	///< Is successor info present?
  const KunaNodeSet &getSuccessors(void) const { return successors; }	///< External successors
  const KunaRegionGraph &getGraphWithSuccessors(void) const { return graphWithSuccessors; }	///< Subgraph + frontier
  bool hasFullGraph(void) const { return hasFull; }	///< Is the full loop graph present?
  const KunaRegionGraph &getFullGraph(void) const { return fullGraph; }	///< Full loop graph
  bool isCyclic(void) const { return cyclic; }		///< Does the region contain a loop?
  bool hasCyclicAncestor(void) const { return cyclicAncestor; }	///< Is an ancestor cyclic?
  uintb getAddr(void) const { return head->getAddr(); }	///< Region address (= head address)
  void printTree(ostream &s,int4 indent) const;	///< Nested dump (angr GraphRegion.dbg_print analog)
};

/// \brief (kuna) Callback interface for walking the blocks of a region tree
///
/// visitBlock() fires once per leaf basic block in deterministic region order
/// (merged chains expand to their members); enter/exitRegion() bracket each
/// nested region.  \e bl is null for synthetic (test-only) nodes.
class KunaRegionVisitor {
public:
  virtual ~KunaRegionVisitor(void) {}
  virtual void enterRegion(const KunaGraphRegion *region) {}	///< Called before a region's nodes
  virtual void exitRegion(const KunaGraphRegion *region) {}	///< Called after a region's nodes
  virtual void visitBlock(const FlowBlock *bl,uintb addr)=0;	///< Called for each leaf block
};

/// \brief (kuna) The region identification analysis (port of angr's RegionIdentifier)
class KunaRegionIdentifier {
  vector<KunaRegionNode *> nodePool;	///< Owns every node ever created
  vector<KunaGraphRegion *> regionPool;	///< Owns every region ever created
  uint4 nextIdent;			///< Node creation counter (determinism tiebreaker)
  bool largestSuccessorTreeOutsideLoop;	///< Option (angr default true)
  bool refineLoopsWithSingleSuccessor;	///< Option (angr default false)
  bool completeSuccessors;		///< Option (angr default false)
  bool hasEntryAddr;			///< Is an entry address known?
  uintb entryAddr;			///< Entry node address
  KunaRegionGraph workGraph;		///< The graph being collapsed
  KunaRegionNode *startNode;		///< Current start node of workGraph
  vector<KunaRegionNode *> loopHeaders;	///< Live loop-header list (consulted by makeCyclicRegion)
  map<KunaRegionNode *,pair<int4,int4>,KunaNodeOrder> nodeOrder;	///< Quasi-topological order
  KunaGraphRegion *topRegion;		///< Result: the top-level region
  vector<vector<uintb> > regionsByBlockAddrs;	///< Result: flat per-region block address lists
  bool computed;			///< Has compute() run?

  KunaRegionNode *newNode(KunaRegionNode::node_kind kind,uintb addr);	///< Allocate a pooled node
  KunaGraphRegion *newRegion(void);	///< Allocate a pooled region (+ its wrapper node)
  void pickConnectedComponent(KunaRegionGraph &g);	///< Keep one weakly connected component
  void makeSupergraph(KunaRegionGraph &g);	///< Merge single-out -> single-in chains
  KunaRegionNode *mergeNodes(KunaRegionGraph &g,KunaRegionNode *a,KunaRegionNode *b);	///< Merge a->b into a k_multi
  bool endsWithBranchindOrCbranch(const KunaRegionNode *n) const;	///< angr _block_ends_with_indirect_jump_or_call
  KunaRegionNode *getStartNode(const KunaRegionGraph &g) const;	///< First in-degree-0 node (or entry-addr node)
  KunaRegionNode *getEntryNode(const KunaRegionGraph &g) const;	///< Node with the entry address (or null)
  KunaRegionNode *nodeByAddr(const KunaRegionGraph &g,uintb addr) const;	///< First node with the given address
  void computeNodeOrder(const KunaRegionGraph &g);	///< Fill nodeOrder via quasi-topological sort
  void sortByNodeOrder(vector<KunaRegionNode *> &nodes) const;	///< Sort by nodeOrder
  void sliceGraph(const KunaRegionGraph &g,KunaRegionNode *node,const KunaNodeSet &frontier,
		  bool includeFrontier,KunaRegionGraph &res) const;	///< slice_graph (+ self-loop hack)
  void findLoopHeaders(const KunaRegionGraph &g,vector<KunaRegionNode *> &res) const;	///< Back-edge targets
  void findInitialLoopNodes(const KunaRegionGraph &g,KunaRegionNode *head,KunaNodeSet &res) const;
  void refineLoop(const KunaRegionGraph &g,KunaRegionNode *head,
		  KunaNodeSet &loopNodes,KunaNodeSet &exitNodes) const;	///< _refine_loop (in/out params)
  KunaGraphRegion *makeRegions(KunaRegionGraph &g);	///< _make_regions driver
  KunaGraphRegion *makeCyclicRegion(KunaRegionNode *head,KunaRegionGraph &g);
  KunaGraphRegion *abstractCyclicRegion(KunaRegionGraph &g,const KunaNodeSet &loopNodes,
					KunaRegionNode *head,const KunaNodeSet &normalEntries,
					const KunaNodeSet &abnormalEntries,KunaRegionNode *normalExit,
					const KunaNodeSet &abnormalExits);
  bool makeAcyclicRegion(KunaRegionNode *head,KunaRegionGraph &graph,KunaRegionGraph *secondaryGraph,
			 set<pair<KunaRegionNode *,KunaRegionNode *> > &failedRegionAttempts,bool cyclic);
  bool checkRegion(const KunaRegionGraph &g,KunaRegionNode *startNode,KunaRegionNode *endNode,
		   KunaIncrementalDominators &doms) const;	///< _check_region predicate
  KunaGraphRegion *computeRegion(const KunaRegionGraph &g,KunaRegionNode *node,const KunaNodeSet &frontier,
				 KunaRegionNode *dummyEndnode,bool cyclicAncestor);	///< _compute_region
  void abstractAcyclicRegion(KunaRegionGraph &g,KunaGraphRegion *region,const KunaNodeSet &frontier,
			     KunaRegionNode *dummyEndnode,KunaRegionGraph *secondaryGraph);
  void replaceRegionInGraph(KunaRegionGraph &g,KunaGraphRegion *region,const KunaNodeSet &replacedNodes);	///< _update_graph
  void regionInEdges(const KunaRegionGraph &g,KunaGraphRegion *region,
		     vector<pair<KunaRegionNode *,KunaRegionNode *> > &res) const;
  void regionOutEdges(const KunaRegionGraph &g,KunaGraphRegion *region,
		      vector<pair<KunaRegionNode *,KunaRegionNode *> > &res) const;
  void buildRegionsByBlockAddrs(void);	///< _make_regions_by_block_addrs
public:
  KunaRegionIdentifier(void);
  ~KunaRegionIdentifier(void);
  void setLargestSuccessorTreeOutsideLoop(bool val) { largestSuccessorTreeOutsideLoop = val; }	///< Set option (before compute)
  void setRefineLoopsWithSingleSuccessor(bool val) { refineLoopsWithSingleSuccessor = val; }	///< Set option (before compute)
  void setCompleteSuccessors(bool val) { completeSuccessors = val; }	///< Set option (before compute)
  void setEntryAddr(uintb addr) { hasEntryAddr = true; entryAddr = addr; }	///< Set the entry address
  void buildFromBlockGraph(const BlockGraph &bblocks);	///< Input A: wrap a (read-only) basic-block graph
  KunaRegionNode *addSyntheticBlock(uintb addr);	///< Input B: add a synthetic test node
  void addSyntheticEdge(KunaRegionNode *a,KunaRegionNode *b);	///< Input B: add a synthetic test edge
  KunaGraphRegion *compute(void);	///< Run the analysis; returns the top-level region
  KunaGraphRegion *getTopRegion(void) const { return topRegion; }	///< Result (null before compute)
  const vector<vector<uintb> > &getRegionsByBlockAddrs(void) const { return regionsByBlockAddrs; }	///< Flat region lists
  void walkBlocks(KunaRegionVisitor &visitor) const;	///< Walk the whole tree
  static void walkRegionBlocks(const KunaGraphRegion *region,KunaRegionVisitor &visitor);	///< Walk one region
  void printTree(ostream &s) const;	///< Dump the nested region tree
};

/// \brief (kuna) `region tree`: dump the nested region tree of the current function
class IfcKunaRegionTree : public IfaceDecompCommand {
public:
  virtual void execute(istream &s);
};

/// \brief (kuna) `region blocks`: dump the flat regions_by_block_addrs lists
class IfcKunaRegionBlocks : public IfaceDecompCommand {
public:
  virtual void execute(istream &s);
};

/// \brief (kuna) `region walk`: walk all leaf blocks in recursive region order
class IfcKunaRegionWalk : public IfaceDecompCommand {
public:
  virtual void execute(istream &s);
};

} // End namespace ghidra
#endif

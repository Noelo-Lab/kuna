/* ###
 * kuna-owned: region identification (S7), ported from the angr decompiler.
 * See kuna_regionid.hh for the file-level description, the source map to
 * angr/analyses/decompiler/region_identifier.py + graph_region.py, and the
 * list of intentional analysis-only divergences.
 *
 * Porting notes (referenced throughout as "RI l.<n>" = region_identifier.py
 * line numbers at angr commit current to 2026-06):
 *   - Python's list(...)-snapshot-before-mutate discipline is reproduced with
 *     explicit vector copies at every such site.
 *   - Pointer-keyed std::set/std::map are used for MEMBERSHIP ONLY; anything
 *     iterated for output goes through KunaNodeOrder.
 *   - Fixpoint loops carry iteration caps that throw LowlevelError instead of
 *     hanging if a guard was mis-ported.
 */
#include "kuna_regionid.hh"

#include <algorithm>

namespace ghidra {

using std::sort;
using std::stable_sort;

/// \brief Fixpoint iteration cap: ~2*n^2, computed in 64-bit
///
/// (kuna) The caps exist only to turn a mis-ported guard into a LowlevelError
/// instead of a hang; 64-bit arithmetic keeps them inert for any real graph
/// (a 32-bit 2*n*n would overflow -- and instantly fire -- at n >= 32768).
static int8 kunaGuardCap(int4 numNodes)

{
  int8 n = numNodes;
  return 2*n*n + 64;
}

//
// KunaGraphRegion
//

/// \brief Print one leaf line of the region tree
static void kunaPrintBlockLine(ostream &s,int4 indent,uintb addr)

{
  for(int4 i=0;i<indent;++i) s << ' ';
  s << "block 0x" << std::hex << addr << std::dec << endl;
}

void KunaGraphRegion::printTree(ostream &s,int4 indent) const

{
  for(int4 i=0;i<indent;++i) s << ' ';
  s << "region head=0x" << std::hex << head->getAddr() << std::dec;
  s << " nodes=" << graph.numNodes();
  if (cyclic) s << " cyclic";
  s << endl;
  for(KunaRegionGraph::const_iterator iter=graph.beginNode();iter!=graph.endNode();++iter) {
    KunaRegionNode *node = (*iter).first;
    if (node->isRegion())
      node->getRegion()->printTree(s,indent+2);
    else if (node->isMulti()) {
      const vector<KunaRegionNode *> &chain( node->getChain() );
      for(int4 i=0;i<(int4)chain.size();++i)
	kunaPrintBlockLine(s,indent+2,chain[i]->getAddr());
    }
    else
      kunaPrintBlockLine(s,indent+2,node->getAddr());
  }
}

//
// KunaRegionIdentifier: construction, inputs, plumbing
//

KunaRegionIdentifier::KunaRegionIdentifier(void)

{
  nextIdent = 0;
  largestSuccessorTreeOutsideLoop = true;	// angr default (RI l.53)
  refineLoopsWithSingleSuccessor = false;	// angr default (RI l.55)
  completeSuccessors = false;			// angr default (RI l.56)
  hasEntryAddr = false;
  entryAddr = 0;
  startNode = (KunaRegionNode *)0;
  topRegion = (KunaGraphRegion *)0;
  computed = false;
}

KunaRegionIdentifier::~KunaRegionIdentifier(void)

{
  for(int4 i=0;i<(int4)nodePool.size();++i)
    delete nodePool[i];
  for(int4 i=0;i<(int4)regionPool.size();++i)
    delete regionPool[i];
}

KunaRegionNode *KunaRegionIdentifier::newNode(KunaRegionNode::node_kind kind,uintb addr)

{
  KunaRegionNode *n = new KunaRegionNode(kind,addr,nextIdent);
  nextIdent += 1;
  nodePool.push_back(n);
  return n;
}

KunaGraphRegion *KunaRegionIdentifier::newRegion(void)

{
  KunaGraphRegion *r = new KunaGraphRegion();
  regionPool.push_back(r);
  return r;
}

void KunaRegionIdentifier::buildFromBlockGraph(const BlockGraph &bblocks)

{
  map<const FlowBlock *,KunaRegionNode *> trans;	// Lookup only
  for(int4 i=0;i<bblocks.getSize();++i) {
    const FlowBlock *bl = bblocks.getBlock(i);
    KunaRegionNode *n = newNode(KunaRegionNode::k_block,bl->getStart().getOffset());
    n->block = bl;
    trans[bl] = n;
    workGraph.addNode(n);
  }
  for(int4 i=0;i<bblocks.getSize();++i) {
    const FlowBlock *bl = bblocks.getBlock(i);
    KunaRegionNode *src = trans[bl];
    for(int4 j=0;j<bl->sizeOut();++j) {
      map<const FlowBlock *,KunaRegionNode *>::const_iterator it = trans.find(bl->getOut(j));
      if (it == trans.end())
	throw LowlevelError("kuna regionid: block graph edge leaves the graph");
      workGraph.addEdge(src,(*it).second);
    }
  }
  const FlowBlock *entry = bblocks.getStartBlock();
  if (entry != (const FlowBlock *)0)
    setEntryAddr(entry->getStart().getOffset());
}

KunaRegionNode *KunaRegionIdentifier::addSyntheticBlock(uintb addr)

{
  KunaRegionNode *n = newNode(KunaRegionNode::k_block,addr);	// block stays null
  workGraph.addNode(n);
  return n;
}

void KunaRegionIdentifier::addSyntheticEdge(KunaRegionNode *a,KunaRegionNode *b)

{
  workGraph.addEdge(a,b);
}

KunaGraphRegion *KunaRegionIdentifier::compute(void)

{
  if (computed) return topRegion;
  computed = true;
  if (workGraph.numNodes() == 0)
    throw LowlevelError("kuna regionid: empty input graph");
  pickConnectedComponent(workGraph);		// RI _pick_one_connected_component
  makeSupergraph(workGraph);			// RI _make_supergraph
  startNode = getStartNode(workGraph);		// RI l.121
  computeNodeOrder(workGraph);			// RI l.122
  topRegion = makeRegions(workGraph);		// RI l.123
  buildRegionsByBlockAddrs();			// RI l.126
  return topRegion;
}

void KunaRegionIdentifier::pickConnectedComponent(KunaRegionGraph &g)

{
  // Weakly connected components in deterministic discovery order (RI l.129)
  vector<KunaRegionNode *> allNodes;
  g.getNodes(allNodes);
  vector<vector<KunaRegionNode *> > components;
  set<const KunaRegionNode *> assigned;		// Membership only
  for(int4 i=0;i<(int4)allNodes.size();++i) {
    if (assigned.find(allNodes[i]) != assigned.end()) continue;
    components.push_back(vector<KunaRegionNode *>());
    vector<KunaRegionNode *> &comp( components.back() );
    comp.push_back(allNodes[i]);
    assigned.insert(allNodes[i]);
    int4 pos = 0;
    while(pos < (int4)comp.size()) {
      KunaRegionNode *cur = comp[pos];
      pos += 1;
      const vector<KunaRegionNode *> &succs( g.getSuccs(cur) );
      for(int4 j=0;j<succs.size();++j) {
	if (assigned.find(succs[j]) == assigned.end()) {
	  assigned.insert(succs[j]);
	  comp.push_back(succs[j]);
	}
      }
      const vector<KunaRegionNode *> &preds( g.getPreds(cur) );
      for(int4 j=0;j<preds.size();++j) {
	if (assigned.find(preds[j]) == assigned.end()) {
	  assigned.insert(preds[j]);
	  comp.push_back(preds[j]);
	}
      }
    }
  }
  if (components.size() <= 1) return;

  int4 chosen = -1;
  int4 largest = 0;
  for(int4 i=0;i<(int4)components.size();++i) {
    if (components[i].size() > components[largest].size())
      largest = i;
    if (hasEntryAddr && chosen < 0) {
      for(int4 j=0;j<(int4)components[i].size();++j) {
	if (components[i][j]->getAddr() == entryAddr) {
	  chosen = i;
	  break;
	}
      }
      if (chosen >= 0) break;	// RI l.144: stop scanning once the entry component is found
    }
  }
  if (chosen < 0) chosen = largest;

  set<const KunaRegionNode *> keep;		// Membership only
  for(int4 j=0;j<(int4)components[chosen].size();++j)
    keep.insert(components[chosen][j]);
  for(int4 i=0;i<(int4)allNodes.size();++i) {
    if (keep.find(allNodes[i]) == keep.end())
      g.removeNode(allNodes[i]);
  }
}

//
// Supergraph construction
//

bool KunaRegionIdentifier::endsWithBranchindOrCbranch(const KunaRegionNode *n) const

{
  // Port of RI _block_ends_with_indirect_jump_or_call (l.1131): angr's
  // "indirect Jump" maps to CPUI_BRANCHIND, "ConditionalJump" to CPUI_CBRANCH.
  // (CPUI_CALL does not terminate a BlockBasic, so the "or call" half cannot
  // occur on a p-code CFG.)
  const KunaRegionNode *last = n;
  if (n->isMulti())
    last = n->getChain().back();
  if (last->getKind() != KunaRegionNode::k_block) return false;
  const FlowBlock *bl = last->getBlock();
  if (bl == (const FlowBlock *)0) return false;	// Synthetic test node: no statements (angr: empty Block)
  PcodeOp *op = bl->lastOp();
  if (op == (PcodeOp *)0) return false;
  OpCode opc = op->code();
  return (opc == CPUI_BRANCHIND || opc == CPUI_CBRANCH);
}

KunaRegionNode *KunaRegionIdentifier::mergeNodes(KunaRegionGraph &g,KunaRegionNode *a,KunaRegionNode *b)

{
  // Port of RI _merge_nodes (l.1142).  The only caller (supergraph) always
  // passes force_multinode=True, so the "forms a region by itself" arm is gone.
  vector<pair<KunaRegionNode *,KunaRegionNode *> > inE;
  g.inEdges(a,inE);
  vector<pair<KunaRegionNode *,KunaRegionNode *> > outE;
  g.outEdges(b,outE);

  vector<KunaRegionNode *> members;
  for(int4 half=0;half<2;++half) {
    KunaRegionNode *src = (half == 0) ? a : b;
    if (src->isMulti()) {
      const vector<KunaRegionNode *> &chain( src->getChain() );
      for(int4 i=0;i<(int4)chain.size();++i)
	members.push_back(chain[i]);
    }
    else if (src->getKind() == KunaRegionNode::k_block)
      members.push_back(src);
    else
      throw LowlevelError("kuna regionid: cannot merge non-block node");	// angr raises TypeError
  }

  KunaRegionNode *m = newNode(KunaRegionNode::k_multi,members[0]->getAddr());
  m->chain = members;

  g.removeNode(a);
  g.removeNode(b);
  g.addNode(m);
  for(int4 i=0;i<(int4)inE.size();++i) {
    KunaRegionNode *src = inE[i].first;
    if (src == b) src = m;			// b -> a back edge becomes a self loop
    g.addEdge(src,m);
  }
  for(int4 i=0;i<(int4)outE.size();++i) {
    KunaRegionNode *dst = outE[i].second;
    if (dst == a) dst = m;			// b -> a forward arm of a 2-cycle
    g.addEdge(m,dst);
  }
  return m;
}

void KunaRegionIdentifier::makeSupergraph(KunaRegionGraph &g)

{
  // Port of RI _make_supergraph (l.246).  Only the "transition" arm survives:
  // angr's 'fake_return' and 'call' edge types cannot occur on a p-code basic
  // block graph (calls do not terminate blocks; every edge is a transition).
  KunaRegionNode *entryNode = (KunaRegionNode *)0;
  if (hasEntryAddr) {
    for(KunaRegionGraph::const_iterator iter=g.beginNode();iter!=g.endNode();++iter) {
      if ((*iter).first->getAddr() == entryAddr) {
	entryNode = (*iter).first;
	break;
      }
    }
  }
  for(;;) {
    bool merged = false;
    vector<pair<KunaRegionNode *,KunaRegionNode *> > edges;
    g.allEdges(edges);
    for(int4 e=0;e<(int4)edges.size();++e) {
      KunaRegionNode *src = edges[e].first;
      KunaRegionNode *dst = edges[e].second;
      if (entryNode != (KunaRegionNode *)0 && dst == entryNode)
	continue;	// The entry node must never be merged INTO (RI l.253)
      if (g.sizeOut(src) == 1 && g.sizeIn(dst) == 1 && src != dst &&
	  !endsWithBranchindOrCbranch(dst)) {
	KunaRegionNode *m = mergeNodes(g,src,dst);
	if (entryNode == src)
	  entryNode = m;
	merged = true;
	break;
      }
    }
    if (!merged) break;	// Clean pass over all edges: fixpoint
  }
}

//
// Node lookup / ordering plumbing
//

KunaRegionNode *KunaRegionIdentifier::getStartNode(const KunaRegionGraph &g) const

{
  for(KunaRegionGraph::const_iterator iter=g.beginNode();iter!=g.endNode();++iter) {
    if ((*iter).second.preds.empty())
      return (*iter).first;
  }
  if (hasEntryAddr) {
    for(KunaRegionGraph::const_iterator iter=g.beginNode();iter!=g.endNode();++iter) {
      if ((*iter).first->getAddr() == entryAddr)
	return (*iter).first;
    }
  }
  throw LowlevelError("kuna regionid: cannot find the start node from the graph");
}

KunaRegionNode *KunaRegionIdentifier::getEntryNode(const KunaRegionGraph &g) const

{
  if (!hasEntryAddr) return (KunaRegionNode *)0;
  for(KunaRegionGraph::const_iterator iter=g.beginNode();iter!=g.endNode();++iter) {
    if ((*iter).first->getAddr() == entryAddr)
      return (*iter).first;
  }
  return (KunaRegionNode *)0;
}

KunaRegionNode *KunaRegionIdentifier::nodeByAddr(const KunaRegionGraph &g,uintb addr) const

{
  for(KunaRegionGraph::const_iterator iter=g.beginNode();iter!=g.endNode();++iter) {
    if ((*iter).first->getAddr() == addr)
      return (*iter).first;
  }
  throw LowlevelError("kuna regionid: no node with the requested address");
}

void KunaRegionIdentifier::computeNodeOrder(const KunaRegionGraph &g)

{
  vector<KunaRegionNode *> sorted;
  kunaQuasiTopoSort(g,sorted);
  nodeOrder.clear();
  for(int4 i=0;i<(int4)sorted.size();++i)
    nodeOrder[sorted[i]] = pair<int4,int4>(i,0);
}

/// \brief Comparator over the nodeOrder map for sortByNodeOrder
namespace {
  struct NodeOrderCmp {
    const map<KunaRegionNode *,pair<int4,int4>,KunaNodeOrder> *order;
    bool operator()(KunaRegionNode *a,KunaRegionNode *b) const {
      map<KunaRegionNode *,pair<int4,int4>,KunaNodeOrder>::const_iterator ia = order->find(a);
      map<KunaRegionNode *,pair<int4,int4>,KunaNodeOrder>::const_iterator ib = order->find(b);
      if (ia == order->end() || ib == order->end())
	throw LowlevelError("kuna regionid: node missing from node order");
      return (*ia).second < (*ib).second;
    }
  };
}

void KunaRegionIdentifier::sortByNodeOrder(vector<KunaRegionNode *> &nodes) const

{
  NodeOrderCmp cmp;
  cmp.order = &nodeOrder;
  stable_sort(nodes.begin(),nodes.end(),cmp);
}

void KunaRegionIdentifier::sliceGraph(const KunaRegionGraph &g,KunaRegionNode *node,
				      const KunaNodeSet &frontier,bool includeFrontier,
				      KunaRegionGraph &res) const

{
  kunaSubgraphBetweenNodes(g,node,frontier,includeFrontier,res);
  // RI l.109 HACK: an infinite self-loop slices to nothing; keep the loop body
  if (res.numNodes() == 0 && g.hasEdge(node,node))
    res.addEdge(node,node);
}

//
// Cyclic phase
//

void KunaRegionIdentifier::findLoopHeaders(const KunaRegionGraph &g,vector<KunaRegionNode *> &res) const

{
  vector<pair<KunaRegionNode *,KunaRegionNode *> > backEdges;
  kunaDfsBackEdges(g,startNode,backEdges);
  KunaNodeSet headSet;
  for(int4 i=0;i<(int4)backEdges.size();++i)
    headSet.insert(backEdges[i].second);
  res.clear();
  for(KunaNodeSet::const_iterator it=headSet.begin();it!=headSet.end();++it)
    res.push_back(*it);
  sortByNodeOrder(res);
}

void KunaRegionIdentifier::findInitialLoopNodes(const KunaRegionGraph &g,KunaRegionNode *head,
						KunaNodeSet &res) const

{
  // RI _find_initial_loop_nodes (l.290)
  vector<pair<KunaRegionNode *,KunaRegionNode *> > backEdges;
  kunaDfsBackEdges(g,startNode,backEdges);
  KunaNodeSet latching;
  for(int4 i=0;i<(int4)backEdges.size();++i) {
    if (backEdges[i].second == head)
      latching.insert(backEdges[i].first);
  }
  KunaRegionGraph loopSub;
  sliceGraph(g,head,latching,true,loopSub);

  // Switch-case expansion: a node with more than two non-self successors is
  // probably a switch head; pull in successors whose predecessors all sit
  // inside the slice already
  int8 guard = 0;
  int8 guardMax = kunaGuardCap(g.numNodes());
  for(;;) {
    if (++guard > guardMax)
      throw LowlevelError("kuna regionid: initial loop nodes did not converge");
    bool updated = false;
    vector<KunaRegionNode *> snap;
    loopSub.getNodes(snap);
    for(int4 i=0;i<(int4)snap.size();++i) {
      KunaRegionNode *node = snap[i];
      vector<KunaRegionNode *> nonself;
      const vector<KunaRegionNode *> &succs( g.getSuccs(node) );
      for(int4 j=0;j<succs.size();++j)
	if (succs[j] != node) nonself.push_back(succs[j]);
      if ((int4)nonself.size() > 2) {
	for(int4 j=0;j<(int4)nonself.size();++j) {
	  KunaRegionNode *succ = nonself[j];
	  if (loopSub.hasEdge(node,succ)) continue;
	  bool allIn = true;
	  const vector<KunaRegionNode *> &preds( g.getPreds(succ) );
	  for(int4 k=0;k<preds.size();++k) {
	    if (!loopSub.containsNode(preds[k])) { allIn = false; break; }
	  }
	  if (allIn) {
	    updated = true;
	    loopSub.addEdge(node,succ);
	  }
	}
      }
    }
    if (!updated) break;
  }

  res.clear();
  for(KunaRegionGraph::const_iterator iter=loopSub.beginNode();iter!=loopSub.endNode();++iter)
    res.insert((*iter).first);
}

void KunaRegionIdentifier::refineLoop(const KunaRegionGraph &g,KunaRegionNode *head,
				      KunaNodeSet &loopNodes,KunaNodeSet &exitNodes) const

{
  // RI _refine_loop (l.317); loopNodes/exitNodes are in/out (they arrive as
  // copies of the initial sets and leave refined)
  if ((refineLoopsWithSingleSuccessor && exitNodes.empty()) ||
      (!refineLoopsWithSingleSuccessor && exitNodes.size() <= 1))
    return;

  KunaNodeSet initialExitNodes = exitNodes;	// Stage 3 needs the originals (RI l.395)

  // Stage 1: absorb single-in-degree, at-most-single-out exit nodes
  int8 guard = 0;
  int8 guardMax = kunaGuardCap(g.numNodes());
  for(;;) {
    if (++guard > guardMax)
      throw LowlevelError("kuna regionid: loop refinement (stage 1) did not converge");
    bool addedAny = false;
    vector<KunaRegionNode *> snap(exitNodes.begin(),exitNodes.end());
    for(int4 i=0;i<(int4)snap.size();++i) {
      KunaRegionNode *exitNode = snap[i];
      if (g.sizeIn(exitNode) == 1 && g.sizeOut(exitNode) <= 1) {
	addedAny = true;
	loopNodes.insert(exitNode);
	const vector<KunaRegionNode *> &succs( g.getSuccs(exitNode) );
	for(int4 j=0;j<succs.size();++j) {
	  if (loopNodes.find(succs[j]) == loopNodes.end())
	    exitNodes.insert(succs[j]);
	}
	exitNodes.erase(exitNode);
      }
    }
    if (!addedAny) break;
  }
  if (exitNodes.size() <= 1) return;

  // Stage 2: absorb dominated exit candidates whose preds are all in the loop
  map<KunaRegionNode *,KunaRegionNode *> idomMap;
  kunaImmediateDominators(g,head,idomMap);

  vector<KunaRegionNode *> sortedExits(exitNodes.begin(),exitNodes.end());
  sortByNodeOrder(sortedExits);
  bool haveNewExits = true;
  KunaRegionGraph sideGraph;	// n -> newly exposed exit edges (feeds stage 3)
  guard = 0;
  while(sortedExits.size() > 1 && haveNewExits) {
    if (++guard > guardMax)
      throw LowlevelError("kuna regionid: loop refinement (stage 2) did not converge");
    // Candidates, in sortedExits order
    vector<KunaRegionNode *> candOrder;
    set<const KunaRegionNode *> candSet;	// Membership only
    {
      vector<KunaRegionNode *> snap = sortedExits;
      for(int4 i=0;i<(int4)snap.size();++i) {
	KunaRegionNode *n = snap[i];
	bool allPredsIn = true;
	const vector<KunaRegionNode *> &preds( g.getPreds(n) );
	for(int4 j=0;j<preds.size();++j) {
	  if (preds[j] != n && loopNodes.find(preds[j]) == loopNodes.end()) {
	    allPredsIn = false;
	    break;
	  }
	}
	if (allPredsIn && kunaDominates(idomMap,head,n)) {
	  candOrder.push_back(n);
	  candSet.insert(n);
	}
      }
    }
    // Union of every candidate's would-be new exits (computed BEFORE any
    // candidate is absorbed -- the per-candidate sets are deliberately stale,
    // matching RI l.360-366)
    KunaNodeSet allNewExitCandidates;
    for(int4 i=0;i<(int4)candOrder.size();++i) {
      const vector<KunaRegionNode *> &succs( g.getSuccs(candOrder[i]) );
      for(int4 j=0;j<succs.size();++j) {
	if (loopNodes.find(succs[j]) == loopNodes.end())
	  allNewExitCandidates.insert(succs[j]);
      }
    }
    // Progress guard (RI l.369): if every candidate is itself a new-exit
    // candidate, absorbing none would loop forever -- clear the veto set
    bool allVetoed = true;
    for(int4 i=0;i<(int4)candOrder.size();++i) {
      if (allNewExitCandidates.find(candOrder[i]) == allNewExitCandidates.end()) {
	allVetoed = false;
	break;
      }
    }
    if (allVetoed)
      allNewExitCandidates.clear();
    // Absorb the surviving candidates
    KunaNodeSet newExitNodes;
    for(int4 i=0;i<(int4)candOrder.size();++i) {
      KunaRegionNode *n = candOrder[i];
      if (allNewExitCandidates.find(n) != allNewExitCandidates.end()) continue;
      loopNodes.insert(n);
      for(vector<KunaRegionNode *>::iterator vi=sortedExits.begin();vi!=sortedExits.end();++vi) {
	if (*vi == n) { sortedExits.erase(vi); break; }
      }
      const vector<KunaRegionNode *> &succs( g.getSuccs(n) );	// Recomputed LIVE (RI l.379)
      for(int4 j=0;j<succs.size();++j) {
	if (loopNodes.find(succs[j]) == loopNodes.end()) {
	  newExitNodes.insert(succs[j]);
	  sideGraph.addEdge(n,succs[j]);
	}
      }
    }
    haveNewExits = !newExitNodes.empty();
    for(KunaNodeSet::const_iterator it=newExitNodes.begin();it!=newExitNodes.end();++it) {
      bool present = false;
      for(int4 i=0;i<(int4)sortedExits.size();++i)
	if (sortedExits[i] == *it) { present = true; break; }
      if (!present) sortedExits.push_back(*it);
    }
    sortByNodeOrder(sortedExits);
  }
  exitNodes.clear();
  for(int4 i=0;i<(int4)sortedExits.size();++i)
    exitNodes.insert(sortedExits[i]);
  for(KunaNodeSet::const_iterator it=exitNodes.begin();it!=exitNodes.end();++it)
    loopNodes.erase(*it);

  // Stage 3 (RI l.391): zero exits left -- pull the LARGEST single-owner
  // successor tree back OUT of the loop and make its root the exit
  if (largestSuccessorTreeOutsideLoop && exitNodes.empty()) {
    map<KunaRegionNode *,KunaNodeSet,KunaNodeOrder> initialExitToNew;
    map<KunaRegionNode *,KunaNodeSet,KunaNodeOrder> newnodeToInitialExits;
    for(KunaNodeSet::const_iterator ei=initialExitNodes.begin();ei!=initialExitNodes.end();++ei) {
      KunaRegionNode *initialExit = *ei;
      if (!sideGraph.containsNode(initialExit)) continue;
      // BFS tree successors over sideGraph (content is visit-order independent)
      set<const KunaRegionNode *> seen;		// Membership only
      seen.insert(initialExit);
      vector<KunaRegionNode *> queue;
      queue.push_back(initialExit);
      int4 pos = 0;
      while(pos < (int4)queue.size()) {
	KunaRegionNode *cur = queue[pos];
	pos += 1;
	vector<KunaRegionNode *> succs;
	sideGraph.getSortedSuccs(cur,succs);
	for(int4 j=0;j<(int4)succs.size();++j) {
	  if (seen.find(succs[j]) != seen.end()) continue;
	  seen.insert(succs[j]);
	  initialExitToNew[initialExit].insert(succs[j]);
	  newnodeToInitialExits[succs[j]].insert(initialExit);
	  queue.push_back(succs[j]);
	}
      }
    }
    for(map<KunaRegionNode *,KunaNodeSet,KunaNodeOrder>::const_iterator mi=newnodeToInitialExits.begin();
	mi!=newnodeToInitialExits.end();++mi) {
      for(KunaNodeSet::const_iterator si=(*mi).second.begin();si!=(*mi).second.end();++si)
	initialExitToNew[*si].insert((*mi).first);
    }
    // Drop subtrees with more than one out-of-tree predecessor
    vector<KunaRegionNode *> dropList;
    for(map<KunaRegionNode *,KunaNodeSet,KunaNodeOrder>::const_iterator mi=initialExitToNew.begin();
	mi!=initialExitToNew.end();++mi) {
      const KunaNodeSet &subtree( (*mi).second );
      set<const KunaRegionNode *> subtreePreds;	// Membership/size only
      for(KunaNodeSet::const_iterator si=subtree.begin();si!=subtree.end();++si) {
	const vector<KunaRegionNode *> &preds( g.getPreds(*si) );
	for(int4 j=0;j<preds.size();++j) {
	  if (subtree.find(preds[j]) == subtree.end())
	    subtreePreds.insert(preds[j]);
	}
	if (subtreePreds.size() > 1) break;
      }
      if (subtreePreds.size() > 1)
	dropList.push_back((*mi).first);
    }
    for(int4 i=0;i<(int4)dropList.size();++i)
      initialExitToNew.erase(dropList[i]);

    if (!initialExitToNew.empty()) {
      int4 maxSize = -1;
      int4 maxCount = 0;
      KunaRegionNode *maxExit = (KunaRegionNode *)0;
      for(map<KunaRegionNode *,KunaNodeSet,KunaNodeOrder>::const_iterator mi=initialExitToNew.begin();
	  mi!=initialExitToNew.end();++mi) {
	int4 sz = (*mi).second.size();
	if (sz > maxSize) { maxSize = sz; maxCount = 1; maxExit = (*mi).first; }
	else if (sz == maxSize) maxCount += 1;
      }
      if (maxCount == 1) {
	const KunaNodeSet &subtree( initialExitToNew[maxExit] );
	bool allSingleOwner = true;
	for(KunaNodeSet::const_iterator si=subtree.begin();si!=subtree.end();++si) {
	  map<KunaRegionNode *,KunaNodeSet,KunaNodeOrder>::const_iterator oi = newnodeToInitialExits.find(*si);
	  if (oi == newnodeToInitialExits.end() || (*oi).second.size() != 1) {
	    allSingleOwner = false;
	    break;
	  }
	}
	if (allSingleOwner) {
	  for(KunaNodeSet::const_iterator si=subtree.begin();si!=subtree.end();++si)
	    loopNodes.erase(*si);
	  loopNodes.erase(maxExit);
	  exitNodes.insert(maxExit);
	}
      }
    }
  }
}

KunaGraphRegion *KunaRegionIdentifier::makeCyclicRegion(KunaRegionNode *head,KunaRegionGraph &g)

{
  // RI _make_cyclic_region (l.518)
  KunaRegionNode *originalEntry = getEntryNode(g);

  KunaNodeSet initialLoopNodes;
  findInitialLoopNodes(g,head,initialLoopNodes);

  // Make sure no OTHER loop header is contained in this loop (address compare,
  // matching `n.addr != head.addr` at RI l.527)
  for(KunaNodeSet::const_iterator it=initialLoopNodes.begin();it!=initialLoopNodes.end();++it) {
    if ((*it)->getAddr() == head->getAddr()) continue;
    for(int4 i=0;i<(int4)loopHeaders.size();++i) {
      if (loopHeaders[i] == *it)
	return (KunaGraphRegion *)0;
    }
  }

  KunaNodeSet normalEntries;
  {
    const vector<KunaRegionNode *> &preds( g.getPreds(head) );
    for(int4 i=0;i<preds.size();++i) {
      if (initialLoopNodes.find(preds[i]) == initialLoopNodes.end())
	normalEntries.insert(preds[i]);
    }
  }
  KunaNodeSet abnormalEntries;
  for(KunaNodeSet::const_iterator it=initialLoopNodes.begin();it!=initialLoopNodes.end();++it) {
    if (*it == head) continue;
    const vector<KunaRegionNode *> &preds( g.getPreds(*it) );
    for(int4 i=0;i<preds.size();++i) {
      if (initialLoopNodes.find(preds[i]) == initialLoopNodes.end())
	abnormalEntries.insert(preds[i]);
    }
  }
  KunaNodeSet initialExitNodes;
  for(KunaNodeSet::const_iterator it=initialLoopNodes.begin();it!=initialLoopNodes.end();++it) {
    const vector<KunaRegionNode *> &succs( g.getSuccs(*it) );
    for(int4 i=0;i<succs.size();++i) {
      if (initialLoopNodes.find(succs[i]) == initialLoopNodes.end())
	initialExitNodes.insert(succs[i]);
    }
  }

  KunaNodeSet refinedLoopNodes = initialLoopNodes;
  KunaNodeSet refinedExitNodes = initialExitNodes;
  refineLoop(g,head,refinedLoopNodes,refinedExitNodes);

  // (kuna) RI l.551-556 _ensure_jump_at_loop_exit_ends omitted: it appends AIL
  // Jump statements for the structurer; this port is analysis-only.

  KunaRegionNode *normalExit = (KunaRegionNode *)0;
  KunaNodeSet abnormalExits;
  if (refinedExitNodes.size() > 1) {
    // angr uses (non-deterministic) networkx.dfs_postorder_nodes here; kuna
    // substitutes the deterministic variant -- a strict improvement
    vector<KunaRegionNode *> postorder;
    kunaDfsPostorderDeterministic(g,head,postorder);
    map<const KunaRegionNode *,int4> poIdx;	// Lookup only
    for(int4 i=0;i<(int4)postorder.size();++i)
      poIdx[postorder[i]] = i;
    vector<pair<int4,KunaRegionNode *> > keyed;
    for(KunaNodeSet::const_iterator it=refinedExitNodes.begin();it!=refinedExitNodes.end();++it) {
      map<const KunaRegionNode *,int4>::const_iterator pi = poIdx.find(*it);
      if (pi == poIdx.end())
	throw LowlevelError("kuna regionid: exit node not reachable from loop head");
      keyed.push_back(pair<int4,KunaRegionNode *>((*pi).second,*it));
    }
    sort(keyed.begin(),keyed.end());
    normalExit = keyed[0].second;
    for(int4 i=1;i<(int4)keyed.size();++i)
      abnormalExits.insert(keyed[i].second);
  }
  else if (refinedExitNodes.size() == 1)
    normalExit = *refinedExitNodes.begin();

  KunaGraphRegion *region = abstractCyclicRegion(g,refinedLoopNodes,head,normalEntries,
						 abnormalEntries,normalExit,abnormalExits);

  // (kuna) RI l.579-581 force_loop_single_exit guarded-successor refinement
  // omitted: it rewrites AIL conditional jumps through the condition processor
  // (see header).  A multi-successor cyclic region keeps all its successors.

  if (originalEntry != (KunaRegionNode *)0 && region->graph.containsNode(originalEntry) &&
      region->head != originalEntry) {
    // RI l.585: head node absorbed the entry; update the tracked entry address
    hasEntryAddr = true;
    entryAddr = head->getAddr();
  }
  return region;
}

KunaGraphRegion *KunaRegionIdentifier::abstractCyclicRegion(KunaRegionGraph &g,const KunaNodeSet &loopNodes,
							    KunaRegionNode *head,const KunaNodeSet &normalEntries,
							    const KunaNodeSet &abnormalEntries,KunaRegionNode *normalExit,
							    const KunaNodeSet &abnormalExits)

{
  // RI _abstract_cyclic_region (l.1017)
  KunaGraphRegion *region = newRegion();
  region->head = head;
  region->cyclic = true;
  KunaRegionNode *wrapper = newNode(KunaRegionNode::k_region,head->getAddr());
  wrapper->region = region;
  region->wrapper = wrapper;

  KunaRegionGraph &subgraph( region->graph );
  region->hasFull = true;
  KunaRegionGraph &fullGraph( region->fullGraph );
  vector<pair<KunaRegionNode *,KunaRegionNode *> > regionOutedges;
  vector<pair<KunaRegionNode *,KunaRegionNode *> > delayedEdges;

  for(KunaNodeSet::const_iterator it=loopNodes.begin();it!=loopNodes.end();++it) {
    KunaRegionNode *node = *it;
    subgraph.addNode(node);
    vector<pair<KunaRegionNode *,KunaRegionNode *> > inE;
    g.inEdges(node,inE);
    vector<pair<KunaRegionNode *,KunaRegionNode *> > outE;
    g.outEdges(node,outE);
    for(int4 i=0;i<(int4)inE.size();++i) {
      KunaRegionNode *src = inE[i].first;
      KunaRegionNode *dst = inE[i].second;
      fullGraph.addEdge(src,dst);
      if (loopNodes.find(src) != loopNodes.end())
	subgraph.addEdge(src,dst);
      // (kuna) RI l.1046 `src == region` arm is dead: the wrapper is not yet in g
      else if (normalEntries.find(src) != normalEntries.end())
	delayedEdges.push_back(pair<KunaRegionNode *,KunaRegionNode *>(src,wrapper));
      else if (abnormalEntries.find(src) != abnormalEntries.end())
	delayedEdges.push_back(pair<KunaRegionNode *,KunaRegionNode *>(src,wrapper));
      else
	throw LowlevelError("kuna regionid: inconsistent cyclic region in-edge");	// RI assert 0
    }
    for(int4 i=0;i<(int4)outE.size();++i) {
      KunaRegionNode *src = outE[i].first;
      KunaRegionNode *dst = outE[i].second;
      fullGraph.addEdge(src,dst);
      if (loopNodes.find(dst) != loopNodes.end())
	subgraph.addEdge(src,dst);
      // (kuna) RI l.1062 `dst == region` arm is dead: the wrapper is not yet in g
      else if (dst == normalExit) {
	regionOutedges.push_back(pair<KunaRegionNode *,KunaRegionNode *>(node,dst));
	delayedEdges.push_back(pair<KunaRegionNode *,KunaRegionNode *>(wrapper,dst));
      }
      else if (abnormalExits.find(dst) != abnormalExits.end()) {
	regionOutedges.push_back(pair<KunaRegionNode *,KunaRegionNode *>(node,dst));
	delayedEdges.push_back(pair<KunaRegionNode *,KunaRegionNode *>(wrapper,dst));
      }
      else
	throw LowlevelError("kuna regionid: inconsistent cyclic region out-edge");	// RI assert 0
    }
  }

  region->hasSuccs = true;
  region->graphWithSuccessors = subgraph;
  for(int4 i=0;i<(int4)regionOutedges.size();++i)
    region->graphWithSuccessors.addEdge(regionOutedges[i].first,regionOutedges[i].second);
  if (normalExit != (KunaRegionNode *)0)
    region->successors.insert(normalExit);
  for(KunaNodeSet::const_iterator it=abnormalExits.begin();it!=abnormalExits.end();++it)
    region->successors.insert(*it);
  for(KunaNodeSet::const_iterator s0=region->successors.begin();s0!=region->successors.end();++s0) {
    for(KunaNodeSet::const_iterator s1=region->successors.begin();s1!=region->successors.end();++s1) {
      if (*s0 != *s1 && g.hasEdge(*s0,*s1))
	region->graphWithSuccessors.addEdge(*s0,*s1);
    }
  }

  for(KunaNodeSet::const_iterator it=loopNodes.begin();it!=loopNodes.end();++it)
    g.removeNode(*it);
  g.addNode(wrapper);
  for(int4 i=0;i<(int4)delayedEdges.size();++i)
    g.addEdge(delayedEdges[i].first,delayedEdges[i].second);

  map<KunaRegionNode *,pair<int4,int4>,KunaNodeOrder>::const_iterator oi = nodeOrder.find(head);
  if (oi == nodeOrder.end())
    throw LowlevelError("kuna regionid: loop head missing from node order");
  nodeOrder[wrapper] = (*oi).second;	// RI l.1099
  return region;
}

//
// Driver
//

KunaGraphRegion *KunaRegionIdentifier::makeRegions(KunaRegionGraph &g)

{
  // RI _make_regions (l.435)
  set<const KunaRegionNode *> structuredLoopHeaders;	// Membership only
  vector<KunaGraphRegion *> newRegions;

  int8 guard = 0;
  int8 guardMax = kunaGuardCap(g.numNodes());
  for(;;) {	// Outer: iteratively find and make loop regions
    if (++guard > guardMax)
      throw LowlevelError("kuna regionid: cyclic phase did not converge");
    findLoopHeaders(g,loopHeaders);
    if (loopHeaders.empty()) break;
    for(;;) {	// Inner: find all loops
      if (++guard > guardMax)
	throw LowlevelError("kuna regionid: cyclic phase did not converge");
      bool restart = false;
      startNode = getStartNode(g);
      findLoopHeaders(g,loopHeaders);
      if (loopHeaders.empty()) break;
      vector<KunaRegionNode *> snap = loopHeaders;	// RI l.460 list(reversed(...))
      for(int4 i=snap.size()-1;i>=0;--i) {
	KunaRegionNode *node = snap[i];
	if (structuredLoopHeaders.find(node) != structuredLoopHeaders.end()) continue;
	if (!g.containsNode(node)) continue;
	KunaGraphRegion *region = makeCyclicRegion(node,g);
	if (region == (KunaGraphRegion *)0) {
	  // Failed (nested header): remove from the LIVE header list so later
	  // attempts in this pass no longer see it (RI l.471)
	  for(vector<KunaRegionNode *>::iterator vi=loopHeaders.begin();vi!=loopHeaders.end();++vi) {
	    if (*vi == node) { loopHeaders.erase(vi); break; }
	  }
	}
	else {
	  newRegions.push_back(region);
	  structuredLoopHeaders.insert(node);
	  restart = true;
	  break;
	}
      }
      if (restart) continue;
      break;
    }
  }

  // Acyclic phase: process each cyclic region's body, then the residual graph
  // (RI l.484 wraps the residual graph in a top-level GraphRegion; kuna
  // represents that entry as region==null and routes it at the workGraph)
  for(int4 widx=0;widx<=(int4)newRegions.size();++widx) {
    bool isTop = (widx == (int4)newRegions.size());
    KunaGraphRegion *region = isTop ? (KunaGraphRegion *)0 : newRegions[widx];
    KunaRegionGraph *sub = isTop ? &g : &region->graph;
    KunaRegionGraph *secondary = (KunaRegionGraph *)0;
    if (!isTop && region->hasSuccs)
      secondary = &region->graphWithSuccessors;
    bool cyc = isTop ? false : region->cyclic;
    KunaRegionNode *head = isTop ? getStartNode(g) : region->head;

    set<pair<KunaRegionNode *,KunaRegionNode *> > failedRegionAttempts;	// Membership only
    int8 innerGuard = 0;
    while(makeAcyclicRegion(head,*sub,secondary,failedRegionAttempts,cyc)) {
      if (++innerGuard > guardMax)
	throw LowlevelError("kuna regionid: acyclic phase did not converge");
      if (!sub->containsNode(head))
	head = nodeByAddr(*sub,head->getAddr());	// RI l.499-501
    }
    head = nodeByAddr(*sub,head->getAddr());
    if (!isTop)
      region->head = head;	// RI l.504
  }

  if (g.numNodes() == 1) {	// RI l.506
    KunaRegionNode *res = (*g.beginNode()).first;
    if (res->isRegion())
      return res->getRegion();
  }
  KunaRegionNode *newHead = getStartNode(g);	// RI l.511
  KunaGraphRegion *top = newRegion();
  top->head = newHead;
  top->graph = g;
  top->hasSuccs = false;
  top->cyclic = false;
  return top;
}

//
// Acyclic phase
//

bool KunaRegionIdentifier::checkRegion(const KunaRegionGraph &g,KunaRegionNode *startNode_,
				       KunaRegionNode *endNode,KunaIncrementalDominators &doms) const

{
  // RI _check_region (l.882): pure predicate over dominance frontiers
  if (!doms.dominates(startNode_,endNode)) {
    const KunaNodeSet &earlyStartFrontier( doms.df(startNode_) );
    for(KunaNodeSet::const_iterator it=earlyStartFrontier.begin();it!=earlyStartFrontier.end();++it) {
      if (*it != startNode_ && *it != endNode)
	return false;
    }
  }

  const KunaNodeSet &endFrontier( doms.df(endNode) );
  for(KunaNodeSet::const_iterator it=endFrontier.begin();it!=endFrontier.end();++it) {
    if (doms.dominates(startNode_,*it) && *it != endNode)
      return false;	// An edge enters the region
  }

  const KunaNodeSet &startFrontier( doms.df(startNode_) );
  for(KunaNodeSet::const_iterator it=startFrontier.begin();it!=startFrontier.end();++it) {
    KunaRegionNode *node = *it;
    if (node == startNode_ || node == endNode) continue;
    if (endFrontier.find(node) == endFrontier.end())
      return false;	// An edge leaves the region
    const vector<KunaRegionNode *> &preds( g.getPreds(node) );
    for(int4 i=0;i<preds.size();++i) {
      if (doms.dominates(startNode_,preds[i]) && !doms.dominates(endNode,preds[i]))
	return false;
    }
  }
  return true;
}

KunaGraphRegion *KunaRegionIdentifier::computeRegion(const KunaRegionGraph &g,KunaRegionNode *node,
						     const KunaNodeSet &frontier,KunaRegionNode *dummyEndnode,
						     bool cyclicAncestor)

{
  // RI _compute_region (l.920); include_frontier is always False at the call
  // sites, so that parameter is dropped
  KunaRegionGraph subgraph;
  vector<pair<KunaRegionNode *,KunaRegionNode *> > frontierEdges;
  vector<KunaRegionNode *> stack;
  stack.push_back(node);
  set<const KunaRegionNode *> traversed;	// Membership only

  while(!stack.empty()) {
    KunaRegionNode *cur = stack.back();
    stack.pop_back();				// LIFO, like Python list.pop()
    if (frontier.find(cur) != frontier.end()) continue;
    if (traversed.find(cur) != traversed.end()) continue;	// Idempotent revisit skip
    traversed.insert(cur);
    subgraph.addNode(cur);
    const vector<KunaRegionNode *> &succs( g.getSuccs(cur) );
    for(int4 i=0;i<succs.size();++i) {
      KunaRegionNode *succ = succs[i];
      // (kuna) RI l.944 `node_ in frontier` inner branch is unreachable
      // (frontier nodes are filtered at pop); dropped
      if (succ == dummyEndnode) continue;
      if (frontier.find(succ) != frontier.end()) {
	frontierEdges.push_back(pair<KunaRegionNode *,KunaRegionNode *>(cur,succ));
	continue;
      }
      subgraph.addEdge(cur,succ);
      if (traversed.find(succ) != traversed.end()) continue;
      stack.push_back(succ);
    }
  }

  KunaNodeSet realFrontier;
  for(KunaNodeSet::const_iterator it=frontier.begin();it!=frontier.end();++it) {
    if (*it != dummyEndnode)
      realFrontier.insert(*it);
  }

  if (subgraph.numNodes() > 1) {
    KunaGraphRegion *region = newRegion();
    region->head = node;
    region->graph = subgraph;
    region->hasSuccs = true;
    region->successors = realFrontier;
    region->graphWithSuccessors = subgraph;
    for(int4 i=0;i<(int4)frontierEdges.size();++i) {
      if (frontierEdges[i].second != dummyEndnode)
	region->graphWithSuccessors.addEdge(frontierEdges[i].first,frontierEdges[i].second);
    }
    region->cyclic = false;
    region->cyclicAncestor = cyclicAncestor;
    return region;
  }
  return (KunaGraphRegion *)0;
}

void KunaRegionIdentifier::regionInEdges(const KunaRegionGraph &g,KunaGraphRegion *region,
					 vector<pair<KunaRegionNode *,KunaRegionNode *> > &res) const

{
  g.inEdges(region->head,res);	// RI _region_in_edges (l.1115)
}

void KunaRegionIdentifier::regionOutEdges(const KunaRegionGraph &g,KunaGraphRegion *region,
					  vector<pair<KunaRegionNode *,KunaRegionNode *> > &res) const

{
  // RI _region_out_edges (l.1119): edges from region members to the outside,
  // re-sourced at the region's wrapper node
  for(KunaRegionGraph::const_iterator iter=region->graph.beginNode();iter!=region->graph.endNode();++iter) {
    KunaRegionNode *member = (*iter).first;
    const vector<KunaRegionNode *> &succs( g.getSuccs(member) );
    for(int4 i=0;i<succs.size();++i) {
      if (region->graph.containsNode(succs[i])) continue;
      res.push_back(pair<KunaRegionNode *,KunaRegionNode *>(region->wrapper,succs[i]));
    }
  }
}

void KunaRegionIdentifier::abstractAcyclicRegion(KunaRegionGraph &g,KunaGraphRegion *region,
						 const KunaNodeSet &frontier,KunaRegionNode *dummyEndnode,
						 KunaRegionGraph *secondaryGraph)

{
  // RI _abstract_acyclic_region (l.980)
  if (region->wrapper == (KunaRegionNode *)0) {
    KunaRegionNode *w = newNode(KunaRegionNode::k_region,region->head->getAddr());
    w->region = region;
    region->wrapper = w;
  }
  vector<pair<KunaRegionNode *,KunaRegionNode *> > inE;
  regionInEdges(g,region,inE);
  vector<pair<KunaRegionNode *,KunaRegionNode *> > outE;
  regionOutEdges(g,region,outE);

  KunaNodeSet nodesSet;
  {
    vector<KunaRegionNode *> members;
    region->graph.getNodes(members);
    for(int4 i=0;i<(int4)members.size();++i) {
      nodesSet.insert(members[i]);
      if (members[i] != dummyEndnode)
	g.removeNode(members[i]);
    }
  }
  g.addNode(region->wrapper);

  // node_order[wrapper] = min over members (lexicographic pair, RI l.1000)
  bool first = true;
  pair<int4,int4> mn(0,0);
  for(KunaNodeSet::const_iterator it=nodesSet.begin();it!=nodesSet.end();++it) {
    map<KunaRegionNode *,pair<int4,int4>,KunaNodeOrder>::const_iterator oi = nodeOrder.find(*it);
    if (oi == nodeOrder.end())
      throw LowlevelError("kuna regionid: region member missing from node order");
    if (first || (*oi).second < mn) { mn = (*oi).second; first = false; }
  }
  nodeOrder[region->wrapper] = mn;

  for(int4 i=0;i<(int4)inE.size();++i) {
    if (nodesSet.find(inE[i].first) == nodesSet.end())
      g.addEdge(inE[i].first,region->wrapper);
  }
  for(int4 i=0;i<(int4)outE.size();++i) {
    if (nodesSet.find(outE[i].second) == nodesSet.end())
      g.addEdge(region->wrapper,outE[i].second);
  }
  for(KunaNodeSet::const_iterator it=frontier.begin();it!=frontier.end();++it) {
    if (*it != dummyEndnode)
      g.addEdge(region->wrapper,*it);
  }

  if (secondaryGraph != (KunaRegionGraph *)0) {
    KunaNodeSet emptyFrontier;
    abstractAcyclicRegion(*secondaryGraph,region,emptyFrontier,(KunaRegionNode *)0,(KunaRegionGraph *)0);
  }
}

void KunaRegionIdentifier::replaceRegionInGraph(KunaRegionGraph &g,KunaGraphRegion *region,
						const KunaNodeSet &replacedNodes)

{
  // RI _update_graph (l.870): patch the working COPY after the main graph was
  // already updated by abstractAcyclicRegion
  vector<pair<KunaRegionNode *,KunaRegionNode *> > inE;
  regionInEdges(g,region,inE);
  vector<pair<KunaRegionNode *,KunaRegionNode *> > outE;
  regionOutEdges(g,region,outE);
  for(KunaNodeSet::const_iterator it=replacedNodes.begin();it!=replacedNodes.end();++it)
    g.removeNode(*it);
  g.addNode(region->wrapper);
  for(int4 i=0;i<(int4)inE.size();++i)
    g.addEdge(inE[i].first,region->wrapper);
  for(int4 i=0;i<(int4)outE.size();++i)
    g.addEdge(region->wrapper,outE[i].second);
}

bool KunaRegionIdentifier::makeAcyclicRegion(KunaRegionNode *head,KunaRegionGraph &graph,
					     KunaRegionGraph *secondaryGraph,
					     set<pair<KunaRegionNode *,KunaRegionNode *> > &failedRegionAttempts,
					     bool cyclic)

{
  // RI _make_acyclic_region (l.715)
  vector<pair<KunaRegionNode *,KunaRegionNode *> > headInedges;
  graph.inEdges(head,headInedges);

  KunaRegionGraph localCopy;
  KunaRegionGraph *gc = &graph;	// Mirrors Python's graph_copy-is-graph aliasing
  if (!headInedges.empty()) {
    localCopy = graph;
    for(int4 i=0;i<(int4)headInedges.size();++i)
      localCopy.removeEdge(headInedges[i].first,headInedges[i].second);
    gc = &localCopy;
  }

  vector<KunaRegionNode *> endnodes;
  for(KunaRegionGraph::const_iterator iter=gc->beginNode();iter!=gc->endNode();++iter) {
    if ((*iter).second.succs.empty())
      endnodes.push_back((*iter).first);
  }
  if (endnodes.empty())
    return false;	// RI l.742: "No end node is found in a supposedly acyclic graph"

  bool addDummy = false;
  if (endnodes.size() > 1)
    addDummy = true;
  else if (!headInedges.empty()) {
    // RI l.749: lone end node that is NOT a predecessor of head (in the
    // ORIGINAL graph) still needs the dummy
    bool isPred = false;
    const vector<KunaRegionNode *> &preds( graph.getPreds(head) );
    for(int4 i=0;i<preds.size();++i)
      if (preds[i] == endnodes[0]) { isPred = true; break; }
    if (!isPred) addDummy = true;
  }

  KunaRegionNode *dummy = (KunaRegionNode *)0;
  if (addDummy) {
    if (gc == &graph) {		// Need a private copy before fabricating edges
      localCopy = graph;
      gc = &localCopy;
    }
    dummy = newNode(KunaRegionNode::k_dummy,~((uintb)0));
    for(int4 i=0;i<(int4)endnodes.size();++i)
      gc->addEdge(endnodes[i],dummy);
    endnodes.clear();
    endnodes.push_back(dummy);
  }

  KunaIncrementalDominators doms(gc,head,false);
  KunaIncrementalDominators postdoms(gc,endnodes[0],true);

  bool regionCreated = false;
  vector<KunaRegionNode *> postorderSnap;
  kunaDfsPostorderDeterministic(*gc,head,postorderSnap);
  for(int4 pidx=0;pidx<(int4)postorderSnap.size();++pidx) {
    KunaRegionNode *node = postorderSnap[pidx];
    if (node == dummy) continue;
    if (cyclic && node == head) continue;
    if (!gc->containsNode(node)) continue;

    int4 outDegree = gc->sizeOut(node);
    if (outDegree == 0) {
      // RI l.780: the root of the region hierarchy should always be a region;
      // wrap a lone, isolated non-region leaf
      if (gc->sizeIn(node) == 0 && !node->isRegion()) {
	KunaGraphRegion *r = newRegion();
	r->head = node;
	r->graph.addNode(node);
	r->hasSuccs = false;
	r->cyclic = false;
	r->cyclicAncestor = cyclic;
	KunaNodeSet emptyFrontier;
	abstractAcyclicRegion(graph,r,emptyFrontier,(KunaRegionNode *)0,secondaryGraph);
      }
      continue;
    }

    KunaRegionNode *postdomNode = postdoms.idom(node);
    while(postdomNode != (KunaRegionNode *)0) {
      pair<KunaRegionNode *,KunaRegionNode *> attempt(node,postdomNode);
      if (failedRegionAttempts.find(attempt) == failedRegionAttempts.end() &&
	  checkRegion(*gc,node,postdomNode,doms)) {
	KunaNodeSet frontier;
	frontier.insert(postdomNode);
	KunaGraphRegion *region = computeRegion(*gc,node,frontier,dummy,cyclic);
	if (region != (KunaGraphRegion *)0) {
	  if (secondaryGraph != (KunaRegionGraph *)0) {
	    // Backpatch graph_with_successors from the parent's secondary
	    // graph (RI l.809-827)
	    vector<KunaRegionNode *> snap;
	    region->graphWithSuccessors.getNodes(snap);
	    for(int4 i=0;i<(int4)snap.size();++i) {
	      KunaRegionNode *nn = snap[i];
	      if (!secondaryGraph->containsNode(nn)) continue;
	      const vector<KunaRegionNode *> &osuccs( secondaryGraph->getSuccs(nn) );
	      for(int4 j=0;j<osuccs.size();++j) {
		KunaRegionNode *succ = osuccs[j];
		if (completeSuccessors) {
		  if (!region->graphWithSuccessors.hasEdge(nn,succ)) {
		    region->graphWithSuccessors.addEdge(nn,succ);
		    region->successors.insert(succ);
		  }
		}
		else {
		  if (!gc->containsNode(succ)) {
		    region->graphWithSuccessors.addEdge(nn,succ);
		    region->successors.insert(succ);
		  }
		}
	      }
	    }
	    // Add edges between successors (RI l.829-833)
	    for(KunaNodeSet::const_iterator s0=region->successors.begin();s0!=region->successors.end();++s0) {
	      for(KunaNodeSet::const_iterator s1=region->successors.begin();s1!=region->successors.end();++s1) {
		if (*s0 != *s1 && secondaryGraph->hasEdge(*s0,*s1))
		  region->graphWithSuccessors.addEdge(*s0,*s1);
	      }
	    }
	  }

	  abstractAcyclicRegion(graph,region,frontier,dummy,secondaryGraph);
	  regionCreated = true;
	  KunaNodeSet replacedNodes;
	  {
	    vector<KunaRegionNode *> members;
	    region->graph.getNodes(members);
	    for(int4 i=0;i<(int4)members.size();++i)
	      replacedNodes.insert(members[i]);
	  }
	  if (gc != &graph)
	    replaceRegionInGraph(*gc,region,replacedNodes);
	  doms.graphUpdated(region->wrapper,replacedNodes,region->head);
	  postdoms.graphUpdated(region->wrapper,replacedNodes,region->head);
#ifdef KUNA_REGIONID_DEBUG
	  doms.verify();
	  postdoms.verify();
#endif
	  break;	// Continue the post-order traversal
	}
      }

      failedRegionAttempts.insert(attempt);
      if (!doms.dominates(node,postdomNode)) break;
      if (postdomNode == postdoms.idom(postdomNode)) break;
      postdomNode = postdoms.idom(postdomNode);
    }
  }
  return regionCreated;
}

//
// Outputs
//

void KunaRegionIdentifier::buildRegionsByBlockAddrs(void)

{
  // RI _make_regions_by_block_addrs (l.169)
  regionsByBlockAddrs.clear();
  if (topRegion == (KunaGraphRegion *)0) return;
  vector<KunaGraphRegion *> workList;
  workList.push_back(topRegion);
  set<const KunaGraphRegion *> seenRegions;	// Membership only
  while(!workList.empty()) {
    vector<KunaGraphRegion *> childrenRegions;
    for(int4 w=0;w<(int4)workList.size();++w) {
      KunaGraphRegion *region = workList[w];
      vector<uintb> childrenBlocks;
      for(KunaRegionGraph::const_iterator iter=region->graph.beginNode();iter!=region->graph.endNode();++iter) {
	KunaRegionNode *node = (*iter).first;
	if (node->getKind() == KunaRegionNode::k_block)
	  childrenBlocks.push_back(node->getAddr());
	else if (node->isMulti()) {
	  const vector<KunaRegionNode *> &chain( node->getChain() );
	  for(int4 i=0;i<(int4)chain.size();++i)
	    childrenBlocks.push_back(chain[i]->getAddr());
	}
	else if (node->isRegion()) {
	  KunaGraphRegion *sub = node->getRegion();
	  if (seenRegions.find(sub) == seenRegions.end()) {
	    childrenRegions.push_back(sub);
	    childrenBlocks.push_back(sub->head->getAddr());
	    seenRegions.insert(sub);
	  }
	}
	// k_dummy: skip (never appears in region graphs)
      }
      if (!childrenBlocks.empty())
	regionsByBlockAddrs.push_back(childrenBlocks);
    }
    workList = childrenRegions;
  }
}

void KunaRegionIdentifier::walkRegionBlocks(const KunaGraphRegion *region,KunaRegionVisitor &visitor)

{
  visitor.enterRegion(region);
  for(KunaRegionGraph::const_iterator iter=region->getGraph().beginNode();
      iter!=region->getGraph().endNode();++iter) {
    KunaRegionNode *node = (*iter).first;
    if (node->isRegion())
      walkRegionBlocks(node->getRegion(),visitor);
    else if (node->isMulti()) {
      const vector<KunaRegionNode *> &chain( node->getChain() );
      for(int4 i=0;i<(int4)chain.size();++i)
	visitor.visitBlock(chain[i]->getBlock(),chain[i]->getAddr());
    }
    else if (node->getKind() == KunaRegionNode::k_block)
      visitor.visitBlock(node->getBlock(),node->getAddr());
  }
  visitor.exitRegion(region);
}

void KunaRegionIdentifier::walkBlocks(KunaRegionVisitor &visitor) const

{
  if (topRegion == (KunaGraphRegion *)0)
    throw LowlevelError("kuna regionid: compute() has not run");
  walkRegionBlocks(topRegion,visitor);
}

void KunaRegionIdentifier::printTree(ostream &s) const

{
  if (topRegion == (KunaGraphRegion *)0)
    throw LowlevelError("kuna regionid: compute() has not run");
  topRegion->printTree(s,0);
}

//
// Console commands
//

/// \brief Run the identifier over the current function's basic blocks
static void kunaRunRegionId(IfaceDecompData *dcp,KunaRegionIdentifier &ri)

{
  if (dcp->fd == (Funcdata *)0)
    throw IfaceExecutionError("No function selected");
  if (dcp->fd->getBasicBlocks().getSize() == 0)
    throw IfaceExecutionError("No basic blocks for "+dcp->fd->getName()+" (decompile first)");
  ri.buildFromBlockGraph(dcp->fd->getBasicBlocks());
  ri.compute();
}

void IfcKunaRegionTree::execute(istream &s)

{
  KunaRegionIdentifier ri;	// Recomputed per call: can never disagree with bblocks
  kunaRunRegionId(dcp,ri);
  ostream &os( *status->fileoptr );	// Bulk stream: assertable from datatests
  os << "Region tree for " << dcp->fd->getName() << ":" << endl;
  ri.printTree(os);
}

void IfcKunaRegionBlocks::execute(istream &s)

{
  KunaRegionIdentifier ri;
  kunaRunRegionId(dcp,ri);
  ostream &os( *status->fileoptr );
  const vector<vector<uintb> > &regions( ri.getRegionsByBlockAddrs() );
  os << "Regions for " << dcp->fd->getName() << ": " << regions.size() << endl;
  for(int4 i=0;i<(int4)regions.size();++i) {
    os << '[';
    for(int4 j=0;j<(int4)regions[i].size();++j) {
      if (j != 0) os << ", ";
      os << "0x" << std::hex << regions[i][j] << std::dec;
    }
    os << ']' << endl;
  }
}

namespace {
  /// \brief Visitor printing one `walk 0x<addr>` line per leaf block
  class KunaWalkPrinter : public KunaRegionVisitor {
    ostream &os;
  public:
    KunaWalkPrinter(ostream &s) : os(s) {}
    virtual void visitBlock(const FlowBlock *bl,uintb addr) {
      os << "walk 0x" << std::hex << addr << std::dec << endl;
    }
  };
}

void IfcKunaRegionWalk::execute(istream &s)

{
  KunaRegionIdentifier ri;
  kunaRunRegionId(dcp,ri);
  ostream &os( *status->fileoptr );
  os << "Region walk for " << dcp->fd->getName() << ":" << endl;
  KunaWalkPrinter printer(os);
  ri.walkBlocks(printer);
}

} // End namespace ghidra

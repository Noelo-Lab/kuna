/* ###
 * kuna-owned: graph substrate for the angr RegionIdentifier port (S7).
 * See kuna_regiongraph.hh for the file-level description and the mapping to
 * the angr sources (angr/utils/graph.py, angr/utils/doms.py).
 */
#include "kuna_regiongraph.hh"

#include <algorithm>

namespace ghidra {

using std::sort;
using std::stable_sort;

//
// KunaRegionGraph
//

bool KunaRegionGraph::containsNode(const KunaRegionNode *n) const

{
  return (nodes.find((KunaRegionNode *)n) != nodes.end());
}

void KunaRegionGraph::addNode(KunaRegionNode *n)

{
  nodes[n];		// Default-construct adjacency if absent
}

void KunaRegionGraph::removeNode(KunaRegionNode *n)

{
  NodeMap::iterator iter = nodes.find(n);
  if (iter == nodes.end()) return;
  // Strip this node out of every neighbor's adjacency (handles self-loops:
  // the self entry lives in our own lists, which are erased wholesale)
  vector<KunaRegionNode *> &preds( (*iter).second.preds );
  for(int4 i=0;i<preds.size();++i) {
    KunaRegionNode *p = preds[i];
    if (p == n) continue;
    vector<KunaRegionNode *> &ps( nodes[p].succs );
    for(vector<KunaRegionNode *>::iterator pi=ps.begin();pi!=ps.end();++pi) {
      if (*pi == n) { ps.erase(pi); break; }
    }
  }
  vector<KunaRegionNode *> &succs( (*iter).second.succs );
  for(int4 i=0;i<succs.size();++i) {
    KunaRegionNode *s = succs[i];
    if (s == n) continue;
    vector<KunaRegionNode *> &sp( nodes[s].preds );
    for(vector<KunaRegionNode *>::iterator si=sp.begin();si!=sp.end();++si) {
      if (*si == n) { sp.erase(si); break; }
    }
  }
  nodes.erase(iter);
}

bool KunaRegionGraph::hasEdge(const KunaRegionNode *a,const KunaRegionNode *b) const

{
  NodeMap::const_iterator iter = nodes.find((KunaRegionNode *)a);
  if (iter == nodes.end()) return false;
  const vector<KunaRegionNode *> &succs( (*iter).second.succs );
  for(int4 i=0;i<succs.size();++i)
    if (succs[i] == b) return true;
  return false;
}

void KunaRegionGraph::addEdge(KunaRegionNode *a,KunaRegionNode *b)

{
  if (hasEdge(a,b)) return;
  nodes[a].succs.push_back(b);
  nodes[b].preds.push_back(a);
}

void KunaRegionGraph::removeEdge(KunaRegionNode *a,KunaRegionNode *b)

{
  NodeMap::iterator iter = nodes.find(a);
  if (iter == nodes.end()) return;
  vector<KunaRegionNode *> &succs( (*iter).second.succs );
  for(vector<KunaRegionNode *>::iterator si=succs.begin();si!=succs.end();++si) {
    if (*si == b) { succs.erase(si); break; }
  }
  iter = nodes.find(b);
  if (iter == nodes.end()) return;
  vector<KunaRegionNode *> &preds( (*iter).second.preds );
  for(vector<KunaRegionNode *>::iterator pi=preds.begin();pi!=preds.end();++pi) {
    if (*pi == a) { preds.erase(pi); break; }
  }
}

int4 KunaRegionGraph::sizeIn(const KunaRegionNode *n) const

{
  NodeMap::const_iterator iter = nodes.find((KunaRegionNode *)n);
  if (iter == nodes.end())
    throw LowlevelError("kuna regiongraph: sizeIn of node not in graph");
  return (*iter).second.preds.size();
}

int4 KunaRegionGraph::sizeOut(const KunaRegionNode *n) const

{
  NodeMap::const_iterator iter = nodes.find((KunaRegionNode *)n);
  if (iter == nodes.end())
    throw LowlevelError("kuna regiongraph: sizeOut of node not in graph");
  return (*iter).second.succs.size();
}

const vector<KunaRegionNode *> &KunaRegionGraph::getPreds(const KunaRegionNode *n) const

{
  NodeMap::const_iterator iter = nodes.find((KunaRegionNode *)n);
  if (iter == nodes.end())
    throw LowlevelError("kuna regiongraph: getPreds of node not in graph");
  return (*iter).second.preds;
}

const vector<KunaRegionNode *> &KunaRegionGraph::getSuccs(const KunaRegionNode *n) const

{
  NodeMap::const_iterator iter = nodes.find((KunaRegionNode *)n);
  if (iter == nodes.end())
    throw LowlevelError("kuna regiongraph: getSuccs of node not in graph");
  return (*iter).second.succs;
}

void KunaRegionGraph::getSortedSuccs(const KunaRegionNode *n,vector<KunaRegionNode *> &res) const

{
  res = getSuccs(n);
  sort(res.begin(),res.end(),KunaNodeOrder());
}

void KunaRegionGraph::inEdges(const KunaRegionNode *n,vector<pair<KunaRegionNode *,KunaRegionNode *> > &res) const

{
  const vector<KunaRegionNode *> &preds( getPreds(n) );
  for(int4 i=0;i<preds.size();++i)
    res.push_back(pair<KunaRegionNode *,KunaRegionNode *>(preds[i],(KunaRegionNode *)n));
}

void KunaRegionGraph::outEdges(const KunaRegionNode *n,vector<pair<KunaRegionNode *,KunaRegionNode *> > &res) const

{
  const vector<KunaRegionNode *> &succs( getSuccs(n) );
  for(int4 i=0;i<succs.size();++i)
    res.push_back(pair<KunaRegionNode *,KunaRegionNode *>((KunaRegionNode *)n,succs[i]));
}

void KunaRegionGraph::allEdges(vector<pair<KunaRegionNode *,KunaRegionNode *> > &res) const

{
  for(const_iterator iter=nodes.begin();iter!=nodes.end();++iter) {
    const vector<KunaRegionNode *> &succs( (*iter).second.succs );
    for(int4 i=0;i<succs.size();++i)
      res.push_back(pair<KunaRegionNode *,KunaRegionNode *>((*iter).first,succs[i]));
  }
}

void KunaRegionGraph::getNodes(vector<KunaRegionNode *> &res) const

{
  for(const_iterator iter=nodes.begin();iter!=nodes.end();++iter)
    res.push_back((*iter).first);
}

void KunaRegionGraph::induced(const KunaNodeSet &keep,KunaRegionGraph &res) const

{
  for(KunaNodeSet::const_iterator ki=keep.begin();ki!=keep.end();++ki) {
    if (!containsNode(*ki)) continue;
    res.addNode(*ki);
  }
  for(KunaNodeSet::const_iterator ki=keep.begin();ki!=keep.end();++ki) {
    if (!containsNode(*ki)) continue;
    const vector<KunaRegionNode *> &succs( getSuccs(*ki) );
    for(int4 i=0;i<succs.size();++i) {
      if (keep.find(succs[i]) != keep.end())
	res.addEdge(*ki,succs[i]);
    }
  }
}

void KunaRegionGraph::buildReversed(KunaRegionGraph &res) const

{
  for(const_iterator iter=nodes.begin();iter!=nodes.end();++iter)
    res.addNode((*iter).first);
  for(const_iterator iter=nodes.begin();iter!=nodes.end();++iter) {
    const vector<KunaRegionNode *> &succs( (*iter).second.succs );
    for(int4 i=0;i<succs.size();++i)
      res.addEdge(succs[i],(*iter).first);
  }
}

int4 KunaRegionGraph::numEdges(void) const

{
  int4 count = 0;
  for(const_iterator iter=nodes.begin();iter!=nodes.end();++iter)
    count += (*iter).second.succs.size();
  return count;
}

//
// DFS utilities
//

void kunaDfsBackEdges(const KunaRegionGraph &g,KunaRegionNode *start,
		      vector<pair<KunaRegionNode *,KunaRegionNode *> > &res)

{
  if (!g.containsNode(start)) return;
  set<const KunaRegionNode *> visited;	// Membership only, never iterated
  set<const KunaRegionNode *> finished;	// Membership only, never iterated
  struct Frame {
    KunaRegionNode *node;
    vector<KunaRegionNode *> children;
    int4 cursor;
  };
  vector<Frame> stack;
  stack.push_back(Frame());
  stack.back().node = start;
  g.getSortedSuccs(start,stack.back().children);
  stack.back().cursor = 0;
  while(!stack.empty()) {
    Frame &frame( stack.back() );
    visited.insert(frame.node);
    if (frame.cursor < (int4)frame.children.size()) {
      KunaRegionNode *child = frame.children[frame.cursor];
      frame.cursor += 1;
      if (visited.find(child) != visited.end()) {
	if (finished.find(child) == finished.end())
	  res.push_back(pair<KunaRegionNode *,KunaRegionNode *>(frame.node,child));	// Back edge
      }
      else if (finished.find(child) == finished.end()) {
	// NOTE: push_back invalidates `frame`; nothing below uses it
	stack.push_back(Frame());
	stack.back().node = child;
	g.getSortedSuccs(child,stack.back().children);
	stack.back().cursor = 0;
      }
    }
    else {
      finished.insert(frame.node);
      stack.pop_back();
    }
  }
}

void kunaDfsPostorderDeterministic(const KunaRegionGraph &g,KunaRegionNode *source,
				   vector<KunaRegionNode *> &res)

{
  set<const KunaRegionNode *> visited;	// Membership only
  vector<pair<KunaRegionNode *,bool> > stack;	// (node, isPreVisit)
  stack.push_back(pair<KunaRegionNode *,bool>(source,true));
  while(!stack.empty()) {
    pair<KunaRegionNode *,bool> entry = stack.back();
    stack.pop_back();
    if (entry.second && visited.find(entry.first) == visited.end()) {
      visited.insert(entry.first);
      stack.push_back(pair<KunaRegionNode *,bool>(entry.first,false));
      vector<KunaRegionNode *> succs;
      g.getSortedSuccs(entry.first,succs);
      // Pushed in ascending order so the LARGEST successor pops first (angr-exact)
      for(int4 i=0;i<succs.size();++i) {
	if (visited.find(succs[i]) == visited.end())
	  stack.push_back(pair<KunaRegionNode *,bool>(succs[i],true));
      }
    }
    else if (!entry.second)
      res.push_back(entry.first);
  }
}

//
// Quasi-topological sort
//

/// \brief BFS shortest path length between two nodes; -1 if unreachable
static int4 kunaShortestPathLength(const KunaRegionGraph &g,KunaRegionNode *a,KunaRegionNode *b)

{
  if (a == b) return 0;
  map<const KunaRegionNode *,int4> dist;	// Membership/lookup only
  vector<KunaRegionNode *> queue;
  dist[a] = 0;
  queue.push_back(a);
  int4 pos = 0;
  while(pos < (int4)queue.size()) {
    KunaRegionNode *cur = queue[pos];
    pos += 1;
    int4 d = dist[cur];
    const vector<KunaRegionNode *> &succs( g.getSuccs(cur) );
    for(int4 i=0;i<succs.size();++i) {
      if (dist.find(succs[i]) != dist.end()) continue;
      if (succs[i] == b) return d + 1;
      dist[succs[i]] = d + 1;
      queue.push_back(succs[i]);
    }
  }
  return -1;
}

static void kunaAppendScc(const KunaRegionGraph &g,vector<KunaRegionNode *> &ordered,
			  const KunaNodeSet &scc);

/// \brief Edge sort key: src.addr + dst.addr (port of GraphUtils._sort_edge)
///
/// Python's sum is arbitrary precision; compare (carry, low64) lexicographically
/// so address sums straddling 2^64 still order exactly as in angr.
struct KunaEdgeSum {
  bool operator()(const pair<KunaRegionNode *,KunaRegionNode *> &a,
		  const pair<KunaRegionNode *,KunaRegionNode *> &b) const {
    uintb suma = a.first->getAddr() + a.second->getAddr();
    bool carrya = (suma < a.first->getAddr());
    uintb sumb = b.first->getAddr() + b.second->getAddr();
    bool carryb = (sumb < b.first->getAddr());
    if (carrya != carryb) return carryb;
    return (suma < sumb);
  }
};

void kunaQuasiTopoSort(const KunaRegionGraph &g,vector<KunaRegionNode *> &res)

{
  int4 numNodes = g.numNodes();
  if (numNodes == 0) return;
  if (numNodes == 1) {
    res.push_back((*g.beginNode()).first);
    return;
  }

  // --- Find non-trivial strongly connected components (iterative Tarjan) ---
  // Node indices in deterministic (addr,ident) order
  vector<KunaRegionNode *> nodeList;
  g.getNodes(nodeList);
  map<const KunaRegionNode *,int4> nodeIndex;	// Lookup only
  for(int4 i=0;i<numNodes;++i)
    nodeIndex[nodeList[i]] = i;

  vector<int4> tarjanIndex(numNodes,-1);
  vector<int4> tarjanLow(numNodes,0);
  vector<bool> onStack(numNodes,false);
  vector<int4> sccOf(numNodes,-1);		// SCC id per node (incl. trivial)
  vector<vector<int4> > sccMembers;
  {
    int4 counter = 0;
    vector<int4> sccStack;
    struct TFrame { int4 node; int4 child; };
    for(int4 root=0;root<numNodes;++root) {
      if (tarjanIndex[root] != -1) continue;
      vector<TFrame> stack;
      TFrame f0; f0.node = root; f0.child = 0;
      stack.push_back(f0);
      tarjanIndex[root] = counter; tarjanLow[root] = counter; counter += 1;
      sccStack.push_back(root); onStack[root] = true;
      while(!stack.empty()) {
	TFrame &fr( stack.back() );
	const vector<KunaRegionNode *> &succs( g.getSuccs(nodeList[fr.node]) );
	if (fr.child < (int4)succs.size()) {
	  int4 w = nodeIndex[succs[fr.child]];
	  fr.child += 1;
	  if (tarjanIndex[w] == -1) {
	    tarjanIndex[w] = counter; tarjanLow[w] = counter; counter += 1;
	    sccStack.push_back(w); onStack[w] = true;
	    TFrame nf; nf.node = w; nf.child = 0;
	    stack.push_back(nf);
	  }
	  else if (onStack[w]) {
	    if (tarjanIndex[w] < tarjanLow[fr.node]) tarjanLow[fr.node] = tarjanIndex[w];
	  }
	}
	else {
	  int4 v = fr.node;
	  stack.pop_back();
	  if (!stack.empty()) {
	    int4 parent = stack.back().node;
	    if (tarjanLow[v] < tarjanLow[parent]) tarjanLow[parent] = tarjanLow[v];
	  }
	  if (tarjanLow[v] == tarjanIndex[v]) {
	    sccMembers.push_back(vector<int4>());
	    vector<int4> &mem( sccMembers.back() );
	    for(;;) {
	      int4 w = sccStack.back();
	      sccStack.pop_back();
	      onStack[w] = false;
	      mem.push_back(w);
	      sccOf[w] = sccMembers.size() - 1;
	      if (w == v) break;
	    }
	  }
	}
      }
    }
  }

  // Keep only SCCs with more than one node; sort by (size, min addr) like angr
  vector<int4> bigSccs;
  for(int4 i=0;i<(int4)sccMembers.size();++i)
    if (sccMembers[i].size() > 1) bigSccs.push_back(i);
  vector<uintb> sccMinAddr(sccMembers.size(),0);
  vector<uint4> sccMinIdent(sccMembers.size(),0);
  for(int4 i=0;i<(int4)bigSccs.size();++i) {
    const vector<int4> &mem( sccMembers[bigSccs[i]] );
    uintb mn = nodeList[mem[0]]->getAddr();
    uint4 mi = nodeList[mem[0]]->getIdent();
    for(int4 j=1;j<(int4)mem.size();++j) {
      uintb a = nodeList[mem[j]]->getAddr();
      if (a < mn || (a == mn && nodeList[mem[j]]->getIdent() < mi)) {
	mn = a; mi = nodeList[mem[j]]->getIdent();
      }
    }
    sccMinAddr[bigSccs[i]] = mn;
    sccMinIdent[bigSccs[i]] = mi;
  }
  // Stable sort of bigSccs by (size, minaddr, minident)
  for(int4 i=1;i<(int4)bigSccs.size();++i) {	// insertion sort (small)
    int4 key = bigSccs[i];
    int4 j = i - 1;
    while(j >= 0) {
      int4 other = bigSccs[j];
      bool greater;
      if (sccMembers[other].size() != sccMembers[key].size())
	greater = (sccMembers[other].size() > sccMembers[key].size());
      else if (sccMinAddr[other] != sccMinAddr[key])
	greater = (sccMinAddr[other] > sccMinAddr[key]);
      else
	greater = (sccMinIdent[other] > sccMinIdent[key]);
      if (!greater) break;
      bigSccs[j+1] = other;
      j -= 1;
    }
    bigSccs[j+1] = key;
  }
  // compIndex: node index -> position in bigSccs (or -1)
  vector<int4> compIndex(numNodes,-1);
  for(int4 i=0;i<(int4)bigSccs.size();++i) {
    const vector<int4> &mem( sccMembers[bigSccs[i]] );
    for(int4 j=0;j<(int4)mem.size();++j)
      compIndex[mem[j]] = i;
  }

  // --- Build the collapsed (condensation-like) graph over integer ids ---
  // Real node i keeps id i; placeholder for bigSccs[k] gets id numNodes+k;
  // the virtual multi-head root gets id numNodes+numPlace.
  int4 numPlace = bigSccs.size();
  int4 totalIds = numNodes + numPlace + 1;
  int4 rootId = numNodes + numPlace;
  vector<bool> idPresent(totalIds,false);
  vector<vector<int4> > idSuccs(totalIds);
  vector<int4> idInDegree(totalIds,0);
  vector<set<int4> > idSuccSet(totalIds);	// Dedup helper

  vector<pair<KunaRegionNode *,KunaRegionNode *> > edges;
  g.allEdges(edges);
  stable_sort(edges.begin(),edges.end(),KunaEdgeSum());
  for(int4 e=0;e<(int4)edges.size();++e) {
    int4 srcIdx = nodeIndex[edges[e].first];
    int4 dstIdx = nodeIndex[edges[e].second];
    int4 src = (compIndex[srcIdx] >= 0) ? numNodes + compIndex[srcIdx] : srcIdx;
    int4 dst = (compIndex[dstIdx] >= 0) ? numNodes + compIndex[dstIdx] : dstIdx;
    if (src == dst) {		// Same placeholder, or a real self-loop
      idPresent[src] = true;
      continue;
    }
    idPresent[src] = true;
    idPresent[dst] = true;
    if (idSuccSet[src].find(dst) == idSuccSet[src].end()) {
      idSuccSet[src].insert(dst);
      idSuccs[src].push_back(dst);
      idInDegree[dst] += 1;
    }
  }
  // Add loners: nodes with no in or out edges at all
  for(int4 i=0;i<numNodes;++i) {
    if (g.sizeOut(nodeList[i]) == 0 && g.sizeIn(nodeList[i]) == 0)
      idPresent[i] = true;
  }

  // Sort key for ids in the collapsed graph: root first, then (addr, isPlaceholder, ident)
  // (angr compares node.addr only; placeholders carry the SCC's min addr)
  struct IdKey {
    const vector<KunaRegionNode *> *nodeList;
    const vector<int4> *bigSccs;
    const vector<uintb> *sccMinAddr;
    const vector<uint4> *sccMinIdent;
    int4 numNodes;
    int4 rootId;
    bool less(int4 a,int4 b) const {
      if (a == rootId || b == rootId) return (a == rootId && b != rootId);
      uintb addrA = (a >= numNodes) ? (*sccMinAddr)[(*bigSccs)[a-numNodes]] : (*nodeList)[a]->getAddr();
      uintb addrB = (b >= numNodes) ? (*sccMinAddr)[(*bigSccs)[b-numNodes]] : (*nodeList)[b]->getAddr();
      if (addrA != addrB) return (addrA < addrB);
      bool placeA = (a >= numNodes);
      bool placeB = (b >= numNodes);
      if (placeA != placeB) return placeB;	// Real node before placeholder on addr tie
      if (placeA)
	return ((*sccMinIdent)[(*bigSccs)[a-numNodes]] < (*sccMinIdent)[(*bigSccs)[b-numNodes]]);
      return ((*nodeList)[a]->getIdent() < (*nodeList)[b]->getIdent());
    }
  };
  IdKey idKey;
  idKey.nodeList = &nodeList; idKey.bigSccs = &bigSccs;
  idKey.sccMinAddr = &sccMinAddr; idKey.sccMinIdent = &sccMinIdent;
  idKey.numNodes = numNodes; idKey.rootId = rootId;

  // Heads of the collapsed graph
  vector<int4> heads;
  for(int4 i=0;i<totalIds-1;++i)
    if (idPresent[i] && idInDegree[i] == 0) heads.push_back(i);
  int4 head;
  if (heads.size() > 1) {
    head = rootId;
    idPresent[rootId] = true;
    for(int4 i=0;i<(int4)heads.size();++i)
      idSuccs[rootId].push_back(heads[i]);
  }
  else if (heads.size() == 1)
    head = heads[0];
  else
    throw LowlevelError("kuna regiongraph: collapsed graph has no head");

  // Deterministic DFS postorder over the collapsed integer graph
  vector<int4> postorder;
  {
    vector<bool> visited(totalIds,false);
    vector<pair<int4,bool> > stack;
    stack.push_back(pair<int4,bool>(head,true));
    while(!stack.empty()) {
      pair<int4,bool> entry = stack.back();
      stack.pop_back();
      if (entry.second && !visited[entry.first]) {
	visited[entry.first] = true;
	stack.push_back(pair<int4,bool>(entry.first,false));
	vector<int4> succs = idSuccs[entry.first];
	// sort ascending by idKey (insertion sort, degrees are small)
	for(int4 i=1;i<(int4)succs.size();++i) {
	  int4 key = succs[i];
	  int4 j = i - 1;
	  while(j >= 0 && idKey.less(key,succs[j])) { succs[j+1] = succs[j]; j -= 1; }
	  succs[j+1] = key;
	}
	for(int4 i=0;i<(int4)succs.size();++i) {
	  if (!visited[succs[i]])
	    stack.push_back(pair<int4,bool>(succs[i],true));
	}
      }
      else if (!entry.second)
	postorder.push_back(entry.first);
    }
  }

  // Walk in REVERSE postorder, expanding placeholders recursively
  for(int4 i=postorder.size()-1;i>=0;--i) {
    int4 id = postorder[i];
    if (id == rootId) continue;
    if (id >= numNodes) {
      const vector<int4> &mem( sccMembers[bigSccs[id-numNodes]] );
      KunaNodeSet scc;
      for(int4 j=0;j<(int4)mem.size();++j)
	scc.insert(nodeList[mem[j]]);
      kunaAppendScc(g,res,scc);
    }
    else
      res.push_back(nodeList[id]);
  }
}

/// \brief Expand one SCC into the ordered list (port of GraphUtils._append_scc)
///
/// loop_head_candidates is always None at RegionIdentifier's call sites, so
/// that parameter is dropped.
static void kunaAppendScc(const KunaRegionGraph &g,vector<KunaRegionNode *> &ordered,
			  const KunaNodeSet &scc)

{
  KunaRegionNode *loopHead = (KunaRegionNode *)0;

  // Find the first node (scanning ordered backwards) with successors in the SCC
  for(int4 i=ordered.size()-1;i>=0;--i) {
    KunaRegionNode *parent = ordered[i];
    vector<KunaRegionNode *> sccSuccs;	// SCC members that are successors of parent
    {
      KunaNodeSet seen;
      const vector<KunaRegionNode *> &succs( g.getSuccs(parent) );
      for(int4 j=0;j<(int4)succs.size();++j) {
	if (scc.find(succs[j]) != scc.end() && seen.find(succs[j]) == seen.end()) {
	  seen.insert(succs[j]);
	  sccSuccs.push_back(succs[j]);
	}
      }
    }
    if (sccSuccs.size() == 1) {
      loopHead = sccSuccs[0];
      break;
    }
    if (sccSuccs.size() > 1) {
      // Pick the successor with the lowest total pairwise distance; on a
      // distance tie the LAST one (in addr order) wins, matching angr's
      // dict-inversion {v: k} semantics
      sort(sccSuccs.begin(),sccSuccs.end(),KunaNodeOrder());
      map<int4,KunaRegionNode *> distToNode;	// Overwrites on collision = last wins
      for(int4 j=0;j<(int4)sccSuccs.size();++j) {
	int4 total = 0;
	for(int4 k=0;k<(int4)sccSuccs.size();++k) {
	  if (k == j) continue;
	  int4 d = kunaShortestPathLength(g,sccSuccs[j],sccSuccs[k]);
	  if (d < 0)
	    throw LowlevelError("kuna regiongraph: SCC members not mutually reachable");
	  total += d;
	}
	distToNode[total] = sccSuccs[j];
      }
      loopHead = (*distToNode.begin()).second;	// Minimum total distance
      break;
    }
  }

  if (loopHead == (KunaRegionNode *)0)
    loopHead = *scc.begin();	// Minimum (addr,ident) member

  KunaRegionGraph subgraph;
  g.induced(scc,subgraph);
  {
    vector<pair<KunaRegionNode *,KunaRegionNode *> > headIn;
    subgraph.inEdges(loopHead,headIn);
    for(int4 i=0;i<(int4)headIn.size();++i)
      subgraph.removeEdge(headIn[i].first,headIn[i].second);
  }

  // Panic mode: huge dense SCCs converge too slowly one-node-at-a-time
  const int4 PANIC_THRESHOLD = 3000;
  if (subgraph.numNodes() > PANIC_THRESHOLD &&
      (double)subgraph.numEdges() > (double)subgraph.numNodes() * 1.4) {
    vector<pair<KunaRegionNode *,KunaRegionNode *> > backEdges;
    kunaDfsBackEdges(subgraph,loopHead,backEdges);
    stable_sort(backEdges.begin(),backEdges.end(),KunaEdgeSum());
    for(int4 i=0;i<(int4)backEdges.size();++i) {
      subgraph.removeEdge(backEdges[i].first,backEdges[i].second);
      if ((double)subgraph.numEdges() <= (double)subgraph.numNodes() * 1.4) break;
    }
  }

  kunaQuasiTopoSort(subgraph,ordered);	// Appends to ordered
}

//
// subgraph_between_nodes
//

void kunaSubgraphBetweenNodes(const KunaRegionGraph &g,KunaRegionNode *source,
			      const KunaNodeSet &frontier,bool includeFrontier,
			      KunaRegionGraph &res)

{
  if (!g.containsNode(source))
    throw LowlevelError("kuna regiongraph: slice source not in graph");
  for(KunaNodeSet::const_iterator fi=frontier.begin();fi!=frontier.end();++fi) {
    if (!g.containsNode(*fi))
      throw LowlevelError("kuna regiongraph: slice frontier node not in graph");
  }

  KunaRegionGraph graph = g;	// Working copy
  {
    vector<pair<KunaRegionNode *,KunaRegionNode *> > srcIn;
    graph.inEdges(source,srcIn);
    for(int4 i=0;i<(int4)srcIn.size();++i)
      graph.removeEdge(srcIn[i].first,srcIn[i].second);
  }

  // One reverse BFS from the frontier set: which nodes can still reach it?
  // (replaces angr's per-successor networkx.has_path probes; equivalent since
  // `graph` is not mutated below)
  set<const KunaRegionNode *> canReach;	// Membership only
  {
    vector<KunaRegionNode *> queue;
    for(KunaNodeSet::const_iterator fi=frontier.begin();fi!=frontier.end();++fi) {
      if (canReach.find(*fi) == canReach.end()) {
	canReach.insert(*fi);
	queue.push_back(*fi);
      }
    }
    int4 pos = 0;
    while(pos < (int4)queue.size()) {
      KunaRegionNode *cur = queue[pos];
      pos += 1;
      const vector<KunaRegionNode *> &preds( graph.getPreds(cur) );
      for(int4 i=0;i<preds.size();++i) {
	if (canReach.find(preds[i]) == canReach.end()) {
	  canReach.insert(preds[i]);
	  queue.push_back(preds[i]);
	}
      }
    }
  }

  // Forward FIFO BFS from source, adding edges to the slice
  {
    vector<KunaRegionNode *> queue;
    set<const KunaRegionNode *> traversed;	// Membership only
    queue.push_back(source);
    int4 pos = 0;
    while(pos < (int4)queue.size()) {
      KunaRegionNode *node = queue[pos];
      pos += 1;
      if (traversed.find(node) != traversed.end()) continue;
      traversed.insert(node);
      const vector<KunaRegionNode *> &succs( graph.getSuccs(node) );
      for(int4 i=0;i<succs.size();++i) {
	KunaRegionNode *succ = succs[i];
	if (res.hasEdge(node,succ)) continue;
	res.addEdge(node,succ);
	if (traversed.find(succ) != traversed.end()) continue;
	if (frontier.find(succ) != frontier.end()) continue;
	if (canReach.find(succ) != canReach.end())
	  queue.push_back(succ);
      }
    }
  }

  // Iteratively prune dangling interior nodes
  for(;;) {
    vector<KunaRegionNode *> toRemove;
    for(KunaRegionGraph::const_iterator iter=res.beginNode();iter!=res.endNode();++iter) {
      KunaRegionNode *n = (*iter).first;
      if (n == source) continue;
      if (frontier.find(n) != frontier.end()) continue;
      if ((*iter).second.succs.empty() || (*iter).second.preds.empty())
	toRemove.push_back(n);
    }
    if (toRemove.empty()) break;
    for(int4 i=0;i<(int4)toRemove.size();++i)
      res.removeNode(toRemove[i]);
  }

  if (!includeFrontier) {
    for(KunaNodeSet::const_iterator fi=frontier.begin();fi!=frontier.end();++fi) {
      if (res.containsNode(*fi))
	res.removeNode(*fi);
    }
  }
}

//
// Dominators
//

void kunaImmediateDominators(const KunaRegionGraph &g,KunaRegionNode *start,
			     map<KunaRegionNode *,KunaRegionNode *> &idom)

{
  // Cooper-Harvey-Kennedy over deterministic reverse postorder
  vector<KunaRegionNode *> postorder;
  kunaDfsPostorderDeterministic(g,start,postorder);
  map<const KunaRegionNode *,int4> poNum;	// Lookup only
  for(int4 i=0;i<(int4)postorder.size();++i)
    poNum[postorder[i]] = i;

  idom[start] = start;
  bool changed = true;
  while(changed) {
    changed = false;
    for(int4 i=postorder.size()-1;i>=0;--i) {	// Reverse postorder
      KunaRegionNode *b = postorder[i];
      if (b == start) continue;
      KunaRegionNode *newIdom = (KunaRegionNode *)0;
      const vector<KunaRegionNode *> &preds( g.getPreds(b) );
      for(int4 p=0;p<preds.size();++p) {
	KunaRegionNode *pred = preds[p];
	if (poNum.find(pred) == poNum.end()) continue;	// Unreachable predecessor
	if (idom.find(pred) == idom.end()) continue;	// Not yet processed
	if (newIdom == (KunaRegionNode *)0) {
	  newIdom = pred;
	  continue;
	}
	// intersect(pred, newIdom)
	KunaRegionNode *f1 = pred;
	KunaRegionNode *f2 = newIdom;
	while(f1 != f2) {
	  while(poNum[f1] < poNum[f2]) f1 = idom[f1];
	  while(poNum[f2] < poNum[f1]) f2 = idom[f2];
	}
	newIdom = f1;
      }
      if (newIdom != (KunaRegionNode *)0) {
	map<KunaRegionNode *,KunaRegionNode *>::iterator it = idom.find(b);
	if (it == idom.end() || (*it).second != newIdom) {
	  idom[b] = newIdom;
	  changed = true;
	}
      }
    }
  }
}

bool kunaDominates(const map<KunaRegionNode *,KunaRegionNode *> &idom,
		   KunaRegionNode *dominator,KunaRegionNode *node)

{
  KunaRegionNode *n = node;
  while(n != (KunaRegionNode *)0) {
    if (n == dominator) return true;
    map<KunaRegionNode *,KunaRegionNode *>::const_iterator it = idom.find(n);
    n = (it != idom.end() && (*it).second != n) ? (*it).second : (KunaRegionNode *)0;
  }
  return false;
}

//
// KunaIncrementalDominators
//

KunaIncrementalDominators::KunaIncrementalDominators(const KunaRegionGraph *g,KunaRegionNode *st,bool isPost)

{
  graph = g;
  start = st;
  post = isPost;
  dfsValid = false;
  invertedValid = false;
  initDoms(doms);
}

void KunaIncrementalDominators::initDoms(map<KunaRegionNode *,KunaRegionNode *> &res) const

{
  if (post) {
    KunaRegionGraph rev;
    graph->buildReversed(rev);
    kunaImmediateDominators(rev,start,res);
  }
  else
    kunaImmediateDominators(*graph,start,res);
  res[start] = start;
}

void KunaIncrementalDominators::initDfs(map<KunaRegionNode *,KunaNodeSet> &res) const

{
  // Port of IncrementalDominators.init_dfs: walk every node with >= 2
  // direction-predecessors, climbing the dom chain
  for(map<KunaRegionNode *,KunaRegionNode *>::const_iterator di=doms.begin();di!=doms.end();++di) {
    KunaRegionNode *u = (*di).first;
    if (!graph->containsNode(u)) continue;
    const vector<KunaRegionNode *> &preds( post ? graph->getSuccs(u) : graph->getPreds(u) );
    if (preds.size() < 2) continue;
    for(int4 i=0;i<preds.size();++i) {
      KunaRegionNode *v = preds[i];
      map<KunaRegionNode *,KunaRegionNode *>::const_iterator vi = doms.find(v);
      if (vi == doms.end()) continue;
      while(v != (*di).second) {
	res[v].insert(u);
	vi = doms.find(v);
	if (vi == doms.end())
	  throw LowlevelError("kuna regiongraph: broken dom chain in initDfs");
	v = (*vi).second;
      }
    }
  }
}

void KunaIncrementalDominators::updateInvertedDomtree(void)

{
  if (invertedValid) return;
  invertedValid = true;
  for(map<KunaRegionNode *,KunaRegionNode *>::const_iterator di=doms.begin();di!=doms.end();++di)
    invertedDomTree[(*di).second].push_back((*di).first);
}

void KunaIncrementalDominators::graphUpdated(KunaRegionNode *newNode,const KunaNodeSet &replacedNodes,
					     KunaRegionNode *replacedHead)

{
  updateInvertedDomtree();

  // Climb out of the replaced set to find the new node's dominator
  map<KunaRegionNode *,KunaRegionNode *>::const_iterator it = doms.find(replacedHead);
  if (it == doms.end())
    throw LowlevelError("kuna regiongraph: replaced head has no dominator");
  KunaRegionNode *newDom = (*it).second;
  while(replacedNodes.find(newDom) != replacedNodes.end() && newDom != start) {
    it = doms.find(newDom);
    if (it == doms.end())
      throw LowlevelError("kuna regiongraph: broken dom chain in graphUpdated");
    newDom = (*it).second;
  }
  if (replacedNodes.find(start) != replacedNodes.end())
    start = newNode;
  if (replacedNodes.find(newDom) != replacedNodes.end())
    newDom = newNode;

  // Re-point all dominatees of replaced nodes at the new node
  vector<KunaRegionNode *> newNodeDoms;
  for(KunaNodeSet::const_iterator ri=replacedNodes.begin();ri!=replacedNodes.end();++ri) {
    map<KunaRegionNode *,vector<KunaRegionNode *> >::const_iterator ii = invertedDomTree.find(*ri);
    if (ii == invertedDomTree.end()) continue;
    const vector<KunaRegionNode *> &dtees( (*ii).second );
    for(int4 i=0;i<(int4)dtees.size();++i) {
      doms[dtees[i]] = newNode;
      newNodeDoms.push_back(dtees[i]);
    }
  }
  doms[newNode] = newDom;

  if (dfsValid) {
    map<KunaRegionNode *,KunaNodeSet>::iterator hi = dfs.find(replacedHead);
    if (hi != dfs.end())
      dfs[newNode] = (*hi).second;
    for(KunaNodeSet::const_iterator ri=replacedNodes.begin();ri!=replacedNodes.end();++ri) {
      KunaRegionNode *rn = *ri;
      map<KunaRegionNode *,KunaNodeSet>::iterator di = dfs.find(rn);
      if (di != dfs.end())
	dfs.erase(di);
      for(map<KunaRegionNode *,KunaNodeSet>::iterator mi=dfs.begin();mi!=dfs.end();++mi) {
	KunaNodeSet &fset( (*mi).second );
	KunaNodeSet::iterator fi = fset.find(rn);
	if (fi != fset.end()) {
	  fset.erase(fi);
	  fset.insert(newNode);
	}
      }
    }
  }

  // Keep the inverted dom tree up to date
  invertedDomTree[newDom].push_back(newNode);
  invertedDomTree[newNode] = newNodeDoms;
  for(KunaNodeSet::const_iterator ri=replacedNodes.begin();ri!=replacedNodes.end();++ri) {
    KunaRegionNode *rn = *ri;
    map<KunaRegionNode *,KunaRegionNode *>::iterator di = doms.find(rn);
    if (di != doms.end()) {
      KunaRegionNode *d = (*di).second;
      doms.erase(di);
      vector<KunaRegionNode *> &lst( invertedDomTree[d] );
      bool found = false;
      for(vector<KunaRegionNode *>::iterator li=lst.begin();li!=lst.end();++li) {
	if (*li == rn) { lst.erase(li); found = true; break; }
      }
      if (!found)
	throw LowlevelError("kuna regiongraph: inverted dom tree inconsistent");
    }
    invertedDomTree.erase(rn);
  }
}

KunaRegionNode *KunaIncrementalDominators::idom(KunaRegionNode *n) const

{
  map<KunaRegionNode *,KunaRegionNode *>::const_iterator it = doms.find(n);
  if (it == doms.end()) return (KunaRegionNode *)0;
  return (*it).second;
}

const KunaNodeSet &KunaIncrementalDominators::df(KunaRegionNode *n)

{
  if (!dfsValid) {
    dfsValid = true;
    initDfs(dfs);
  }
  static KunaNodeSet emptySet;
  map<KunaRegionNode *,KunaNodeSet>::const_iterator it = dfs.find(n);
  if (it == dfs.end()) return emptySet;
  return (*it).second;
}

bool KunaIncrementalDominators::dominates(KunaRegionNode *dominator,KunaRegionNode *n) const

{
  KunaRegionNode *cur = n;
  while(cur != (KunaRegionNode *)0) {
    if (cur == dominator) return true;
    KunaRegionNode *d = idom(cur);
    cur = (d != (KunaRegionNode *)0 && cur != d) ? d : (KunaRegionNode *)0;
  }
  return false;
}

void KunaIncrementalDominators::verify(void) const

{
  map<KunaRegionNode *,KunaRegionNode *> trueDoms;
  initDoms(trueDoms);
  if (trueDoms.size() != doms.size())
    throw LowlevelError("kuna regiongraph: incremental dominators diverged (size)");
  for(map<KunaRegionNode *,KunaRegionNode *>::const_iterator it=trueDoms.begin();it!=trueDoms.end();++it) {
    map<KunaRegionNode *,KunaRegionNode *>::const_iterator jt = doms.find((*it).first);
    if (jt == doms.end() || (*jt).second != (*it).second)
      throw LowlevelError("kuna regiongraph: incremental dominators diverged");
  }
  if (dfsValid) {
    map<KunaRegionNode *,KunaNodeSet> trueDfs;
    initDfs(trueDfs);
    if (trueDfs.size() != dfs.size())
      throw LowlevelError("kuna regiongraph: incremental frontiers diverged (size)");
    for(map<KunaRegionNode *,KunaNodeSet>::const_iterator it=trueDfs.begin();it!=trueDfs.end();++it) {
      map<KunaRegionNode *,KunaNodeSet>::const_iterator jt = dfs.find((*it).first);
      if (jt == dfs.end() || (*jt).second != (*it).second)
	throw LowlevelError("kuna regiongraph: incremental frontiers diverged");
    }
  }
}

} // End namespace ghidra

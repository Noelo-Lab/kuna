/* ###
 * kuna-owned: unit tests for the angr RegionIdentifier port (S7).
 *
 * This file is NOT part of upstream Ghidra (it lives in the vendored
 * unittests directory; recorded in UPSTREAM.md "Divergence").
 *
 * regionid_diamond0 / regionid_diamond1 are direct ports of angr's
 * test_region_identifier_0 / test_region_identifier_1
 * (angr/tests/analyses/decompiler/test_structurer.py:50,80): synthetic
 * DummyNode graphs, asserting the final top-level region graph has exactly
 * two nodes.  regionid_loop is kuna-added cyclic coverage (angr's two
 * synthetic tests are both acyclic).
 *
 * No Architecture / SLEIGH spec is needed: the identifier runs on synthetic
 * addr-only nodes, mirroring angr's RegionIdentifier(None, graph=g) mode.
 */
#include "kuna_regionid.hh"
#include "test.hh"

#include <algorithm>

namespace ghidra {

namespace {

/// Collect every leaf block address (in walk order) and every cyclic region
class KunaTestCollector : public KunaRegionVisitor {
public:
  vector<uintb> addrs;
  vector<const KunaGraphRegion *> cyclics;
  virtual void enterRegion(const KunaGraphRegion *region) {
    if (region->isCyclic())
      cyclics.push_back(region);
  }
  virtual void visitBlock(const FlowBlock *bl,uintb addr) {
    addrs.push_back(addr);
  }
};

/// Sorted copy of the collected addresses
vector<uintb> sortedAddrs(const vector<uintb> &in)

{
  vector<uintb> res = in;
  std::sort(res.begin(),res.end());
  return res;
}

} // anonymous namespace

TEST(regionid_diamond0) {
  //       1
  //       |
  //       2
  //      / .
  //     3   4
  //      \ /
  //       5
  //       |
  //       6
  KunaRegionIdentifier ri;
  KunaRegionNode *n1 = ri.addSyntheticBlock(1);
  KunaRegionNode *n2 = ri.addSyntheticBlock(2);
  KunaRegionNode *n3 = ri.addSyntheticBlock(3);
  KunaRegionNode *n4 = ri.addSyntheticBlock(4);
  KunaRegionNode *n5 = ri.addSyntheticBlock(5);
  KunaRegionNode *n6 = ri.addSyntheticBlock(6);
  ri.addSyntheticEdge(n1,n2);
  ri.addSyntheticEdge(n2,n3);
  ri.addSyntheticEdge(n2,n4);
  ri.addSyntheticEdge(n3,n5);
  ri.addSyntheticEdge(n4,n5);
  ri.addSyntheticEdge(n5,n6);

  KunaGraphRegion *top = ri.compute();
  ASSERT(top != (KunaGraphRegion *)0);
  ASSERT_EQUALS(top->getGraph().numNodes(),2);	// The angr assertion
  ASSERT_EQUALS(top->getHead()->getAddr(),(uintb)1);
  ASSERT(!top->isCyclic());

  // Exactly one of the two top-level nodes is a sub-region
  int4 regionCount = 0;
  for(KunaRegionGraph::const_iterator iter=top->getGraph().beginNode();
      iter!=top->getGraph().endNode();++iter) {
    if ((*iter).first->isRegion()) regionCount += 1;
  }
  ASSERT_EQUALS(regionCount,1);

  // The recursive walker covers every input block exactly once
  KunaTestCollector col;
  ri.walkBlocks(col);
  ASSERT_EQUALS(col.addrs.size(),(size_t)6);
  vector<uintb> got = sortedAddrs(col.addrs);
  for(int4 i=0;i<6;++i)
    ASSERT_EQUALS(got[i],(uintb)(i+1));
  ASSERT_EQUALS(col.cyclics.size(),(size_t)0);
}

TEST(regionid_diamond1) {
  //        1
  //        |
  //        2
  //        | .
  //        | 3
  //        | /
  //        4
  //        |
  //        5
  //        | .
  //        | 6
  //        | /
  //        7
  //        |
  //        8
  KunaRegionIdentifier ri;
  KunaRegionNode *nodes[9];
  for(int4 i=1;i<=8;++i)
    nodes[i] = ri.addSyntheticBlock((uintb)i);
  ri.addSyntheticEdge(nodes[1],nodes[2]);
  ri.addSyntheticEdge(nodes[2],nodes[3]);
  ri.addSyntheticEdge(nodes[3],nodes[4]);
  ri.addSyntheticEdge(nodes[2],nodes[4]);
  ri.addSyntheticEdge(nodes[4],nodes[5]);
  ri.addSyntheticEdge(nodes[5],nodes[6]);
  ri.addSyntheticEdge(nodes[6],nodes[7]);
  ri.addSyntheticEdge(nodes[5],nodes[7]);
  ri.addSyntheticEdge(nodes[7],nodes[8]);

  KunaGraphRegion *top = ri.compute();
  ASSERT(top != (KunaGraphRegion *)0);
  ASSERT_EQUALS(top->getGraph().numNodes(),2);	// The angr assertion

  KunaTestCollector col;
  ri.walkBlocks(col);
  ASSERT_EQUALS(col.addrs.size(),(size_t)8);
  vector<uintb> got = sortedAddrs(col.addrs);
  for(int4 i=0;i<8;++i)
    ASSERT_EQUALS(got[i],(uintb)(i+1));
}

TEST(regionid_loop) {
  //   1 -> 2 -> 3 -> 4
  //        ^    |
  //        +----+        (3 loops back to 2)
  KunaRegionIdentifier ri;
  KunaRegionNode *n1 = ri.addSyntheticBlock(1);
  KunaRegionNode *n2 = ri.addSyntheticBlock(2);
  KunaRegionNode *n3 = ri.addSyntheticBlock(3);
  KunaRegionNode *n4 = ri.addSyntheticBlock(4);
  ri.addSyntheticEdge(n1,n2);
  ri.addSyntheticEdge(n2,n3);
  ri.addSyntheticEdge(n3,n2);
  ri.addSyntheticEdge(n3,n4);

  KunaGraphRegion *top = ri.compute();
  ASSERT(top != (KunaGraphRegion *)0);
  ASSERT(!top->isCyclic());

  // Exactly one cyclic region, headed at address 2, containing blocks {2,3}
  KunaTestCollector col;
  ri.walkBlocks(col);
  ASSERT_EQUALS(col.cyclics.size(),(size_t)1);
  const KunaGraphRegion *loop = col.cyclics[0];
  ASSERT_EQUALS(loop->getHead()->getAddr(),(uintb)2);
  KunaTestCollector loopCol;
  KunaRegionIdentifier::walkRegionBlocks(loop,loopCol);
  vector<uintb> loopAddrs = sortedAddrs(loopCol.addrs);
  ASSERT_EQUALS(loopAddrs.size(),(size_t)2);
  ASSERT_EQUALS(loopAddrs[0],(uintb)2);
  ASSERT_EQUALS(loopAddrs[1],(uintb)3);

  // regions_by_block_addrs contains the loop body as its own region
  const vector<vector<uintb> > &rbba( ri.getRegionsByBlockAddrs() );
  bool found = false;
  for(int4 i=0;i<(int4)rbba.size();++i) {
    vector<uintb> sortedRegion = sortedAddrs(rbba[i]);
    if (sortedRegion.size() == 2 && sortedRegion[0] == 2 && sortedRegion[1] == 3) {
      found = true;
      break;
    }
  }
  ASSERT(found);

  // Whole-tree walk covers every block exactly once
  ASSERT_EQUALS(col.addrs.size(),(size_t)4);
  vector<uintb> got = sortedAddrs(col.addrs);
  for(int4 i=0;i<4;++i)
    ASSERT_EQUALS(got[i],(uintb)(i+1));
}

} // End namespace ghidra

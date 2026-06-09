/* ###
 * kuna-owned: strip the glibc -fstack-protector canary epilogue
 * (port of angr's StackCanarySimplifier).  See kuna_stackguard.hh.
 *
 * Detection is PURELY STRUCTURAL.  kuna's BFD console loader does not resolve
 * a PLT stub to its imported name (the __stack_chk_fail@plt call renders as an
 * unnamed sub_<addr>), so the callee name is NOT a usable signal here.  The
 * canary is instead pinned by its compare: a CBRANCH whose INT_EQUAL/
 * INT_NOTEQUAL boolean compares two values that BOTH derive from a LOAD of
 * `<base> + 0x28` (the saved canary slot vs. a fresh reload of the x86-64
 * glibc TLS canary at fs:0x28) -- a shape that occurs only for the stack
 * protector.  The corrupted-canary branch (which must contain the no-return
 * handler CALL) is removed with the stock in-place primitive
 * Funcdata::removeBranch (CBRANCH -> fall-through, MULTIEQUALs patched), then
 * removeUnreachableBlocks collects the orphaned handler block.  The dead
 * canary load/store/compare are reaped by the next deadcode pass in the
 * repeating fullloop.
 */
#include "kuna_stackguard.hh"
#include "funcdata.hh"
#include "architecture.hh"

#include <set>

namespace ghidra {

// kuna ElementIds live in the 4000+ range (see kuna_returnpair.cc note).
ElementId ELEM_STACKGUARD = ElementId("stackguard",4021);

/// \brief Does Varnode \e ptr address `<base> + 0x28` (the x86-64 TLS canary slot)?
///
/// Peels transparent COPY/CAST off the pointer, then requires an INT_ADD with a
/// constant 0x28 operand.  (fs:0x28 is the glibc x86-64 stack-guard offset.)
static bool ptrIsCanarySlot(Varnode *ptr)

{
  for(int4 guard=0;guard<8;++guard) {
    if (!ptr->isWritten()) return false;
    PcodeOp *def = ptr->getDef();
    OpCode oc = def->code();
    if (oc==CPUI_COPY || oc==CPUI_CAST) { ptr = def->getIn(0); continue; }
    if (oc==CPUI_INT_ADD) {
      Varnode *a = def->getIn(0);
      Varnode *b = def->getIn(1);
      if (b->isConstant() && b->getOffset()==0x28) return true;
      if (a->isConstant() && a->getOffset()==0x28) return true;
      return false;
    }
    return false;
  }
  return false;
}

/// \brief Does \e vn ultimately derive from a LOAD of the canary slot?
///
/// Peels value-preserving ops (COPY/CAST/zext/sext/INDIRECT/zero-SUBPIECE) and
/// follows every input of a MULTIEQUAL (the saved-canary operand is a phi over
/// all paths, each tracing to the single entry load).  A visited set bounds
/// loops; \e depth bounds the walk.
static bool derivesFromCanaryLoad(Varnode *vn,int4 depth,std::set<Varnode *> &seen)

{
  if (depth<=0) return false;
  if (seen.count(vn)) return false;
  seen.insert(vn);
  if (!vn->isWritten()) return false;
  PcodeOp *def = vn->getDef();
  OpCode oc = def->code();
  if (oc==CPUI_LOAD)
    return ptrIsCanarySlot(def->getIn(1));
  if (oc==CPUI_COPY || oc==CPUI_CAST || oc==CPUI_INT_ZEXT || oc==CPUI_INT_SEXT || oc==CPUI_INDIRECT)
    return derivesFromCanaryLoad(def->getIn(0),depth-1,seen);
  if (oc==CPUI_SUBPIECE && def->getIn(1)->isConstant() && def->getIn(1)->getOffset()==0)
    return derivesFromCanaryLoad(def->getIn(0),depth-1,seen);
  if (oc==CPUI_MULTIEQUAL) {
    for(int4 i=0;i<def->numInput();++i)
      if (derivesFromCanaryLoad(def->getIn(i),depth-1,seen)) return true;
    return false;
  }
  return false;
}

/// \brief Is the boolean feeding \e cbranch a stack-canary compare?
///
/// True when it is an INT_EQUAL/INT_NOTEQUAL whose BOTH operands derive from a
/// LOAD of the canary slot (saved canary vs. fresh reload).  \e isNotEqual is
/// set so the caller can pick the corrupted-canary (fail) successor.
static bool isCanaryCompare(PcodeOp *cbranch,bool &isNotEqual)

{
  Varnode *boolvn = cbranch->getIn(1);
  if (!boolvn->isWritten()) return false;
  PcodeOp *cmp = boolvn->getDef();
  OpCode oc = cmp->code();
  if (oc!=CPUI_INT_EQUAL && oc!=CPUI_INT_NOTEQUAL) return false;
  std::set<Varnode *> s0,s1;
  if (!derivesFromCanaryLoad(cmp->getIn(0),32,s0)) return false;
  if (!derivesFromCanaryLoad(cmp->getIn(1),32,s1)) return false;
  isNotEqual = (oc==CPUI_INT_NOTEQUAL);
  return true;
}

/// \brief Does basic block \e bb contain a CALL (the canary-fail handler)?
static bool blockHasCall(BlockBasic *bb)

{
  list<PcodeOp *>::const_iterator iter;
  for(iter=bb->beginOp();iter!=bb->endOp();++iter) {
    OpCode oc = (*iter)->code();
    if (oc==CPUI_CALL || oc==CPUI_CALLIND || oc==CPUI_CALLOTHER) return true;
  }
  return false;
}

int4 ActionStripStackGuard::apply(Funcdata &data)

{
  if (!data.getArch()->strip_stack_guard) return 0;	// P0 assertion not set

  const BlockGraph &bbgraph( data.getBasicBlocks() );
  for(int4 i=0;i<bbgraph.getSize();++i) {
    BlockBasic *h = dynamic_cast<BlockBasic *>(bbgraph.getBlock(i));
    if (h == (BlockBasic *)0) continue;
    if (h->sizeOut() != 2) continue;
    PcodeOp *cb = h->lastOp();
    if (cb == (PcodeOp *)0 || cb->code() != CPUI_CBRANCH) continue;
    bool isNotEqual = false;
    if (!isCanaryCompare(cb,isNotEqual)) continue;

    // The successor taken when the boolean condition is TRUE.
    bool flip = cb->isBooleanFlip();
    FlowBlock *condTrue = flip ? h->getFalseOut() : h->getTrueOut();
    FlowBlock *condFalse = flip ? h->getTrueOut() : h->getFalseOut();
    // Canary corrupted => the no-return handler.  For `slot != reload` the
    // true edge is the failure; for `slot == reload` the false edge is.
    FlowBlock *failOut = isNotEqual ? condTrue : condFalse;
    BlockBasic *failBlk = dynamic_cast<BlockBasic *>(failOut);
    if (failBlk == (BlockBasic *)0) continue;
    // Safety: the corrupted branch must actually call the handler (a bare
    // fs:0x28 compare with no handler call is not a stack-protector epilogue).
    if (!blockHasCall(failBlk)) continue;

    // Find and drop the edge h -> failBlk; removeBranch turns the CBRANCH into
    // a fall-through to the canary-OK successor and patches MULTIEQUALs.
    int4 idx = -1;
    for(int4 j=0;j<h->sizeOut();++j) {
      if (h->getOut(j) == failOut) { idx = j; break; }
    }
    if (idx < 0) continue;
    data.removeBranch(h,idx);
    data.removeUnreachableBlocks(false,true);	// collect the orphaned handler block
    return 1;		// one canary per apply; the repeating fullloop re-invokes and self-gates
  }
  return 0;
}

string OptionStackGuard::apply(Architecture *glb,const string &p1,const string &p2,const string &p3) const

{
  bool val = onOrOff(p1);
  glb->strip_stack_guard = val;
  string prop = val ? "on" : "off";
  return "Stack-protector canary epilogue stripping turned "+prop;
}

} // End namespace ghidra

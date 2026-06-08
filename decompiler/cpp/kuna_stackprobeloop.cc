/* ###
 * kuna-owned: stack-probe-loop stack-pointer resolution sub-stage (see kuna_stackprobeloop.hh).
 */
#include "kuna_stackprobeloop.hh"
#include "architecture.hh"
#include "funcdata.hh"

namespace ghidra {

// kuna ElementIds live in the 4000+ range (see kuna_returnpair.cc note).
ElementId ELEM_STACKPROBELOOP = ElementId("stackprobeloop",4012);

void RuleStackProbeLoop::getOpList(vector<uint4> &oplist) const

{
  oplist.push_back(CPUI_MULTIEQUAL);
}

/// \brief Trace a Varnode back through CPUI_COPY ops to its underlying source.
/// \param vn is the starting Varnode
/// \return the first non-COPY-written Varnode reachable (or \b vn itself)
static Varnode *skipCopies(Varnode *vn)

{
  // Bound the walk so a pathological COPY chain can never loop forever.
  for(int4 i=0;i<8;++i) {
    if (!vn->isWritten()) break;
    PcodeOp *op = vn->getDef();
    if (op->code() != CPUI_COPY) break;
    vn = op->getIn(0);
  }
  return vn;
}

/// \brief Is \b vn the spacebase input itself, or `spIn + constant`?
///
/// Traces through COPY ops.  On success, \b off receives the constant offset
/// (0 when \b vn is the input directly).
/// \param spIn is the spacebase (stack-pointer) input Varnode
/// \param vn is the Varnode to test
/// \param off receives the recovered constant offset
/// \return \b true if \b vn is the spacebase input plus a constant
static bool stackRelative(Varnode *spIn,Varnode *vn,uintb &off)

{
  vn = skipCopies(vn);
  if (vn == spIn) { off = 0; return true; }
  if (!vn->isWritten()) return false;
  PcodeOp *op = vn->getDef();
  if (op->code() != CPUI_INT_ADD) return false;
  Varnode *base = skipCopies(op->getIn(0));
  Varnode *cvn = op->getIn(1);
  if (!cvn->isConstant()) {
    cvn = op->getIn(0);
    base = skipCopies(op->getIn(1));
  }
  if (!cvn->isConstant()) return false;
  if (base != spIn) return false;
  off = cvn->getOffset();
  return true;
}

int4 RuleStackProbeLoop::applyOp(PcodeOp *op,Funcdata &data)

{
  if (!data.getArch()->model_stack_probe_loop) return 0;	// (kuna) default-off: byte-identical to upstream

  if (op->numInput() != 2) return 0;		// Loop header: one entry edge, one back edge

  // The MULTIEQUAL must produce the stack-pointer register.
  AddrSpace *stackspc = data.getArch()->getStackSpace();
  if (stackspc == (AddrSpace *)0) return 0;
  Varnode *spIn = data.findSpacebaseInput(stackspc);
  if (spIn == (Varnode *)0) return 0;
  Varnode *phiout = op->getOut();
  if (phiout->getAddr() != spIn->getAddr()) return 0;
  if (phiout->getSize() != spIn->getSize()) return 0;

  // One input is the back-edge `PHI - page`, the other is the stack-relative entry value.
  // Identify the probe step: an INT_ADD(phiout, page) feeding (through COPYs) a MULTIEQUAL input.
  uintb page = 0;
  bool foundback = false;
  bool foundentry = false;
  for(int4 i=0;i<2;++i) {
    Varnode *invn = skipCopies(op->getIn(i));
    if (invn->isWritten()) {
      PcodeOp *def = invn->getDef();
      if (def->code() == CPUI_INT_ADD) {
	Varnode *b = skipCopies(def->getIn(0));
	Varnode *c = def->getIn(1);
	if (b == phiout && c->isConstant()) {	// back-edge = PHI + page
	  page = c->getOffset();
	  foundback = true;
	  continue;
	}
      }
    }
    uintb entryoff;
    if (stackRelative(spIn,op->getIn(i),entryoff))
      foundentry = true;
  }
  if (!foundback || !foundentry) return 0;
  if (page == 0) return 0;

  // Find the loop-exit comparison: an INT_NOTEQUAL/INT_EQUAL whose one operand is
  // `PHI + page` (the probe step) and whose other operand is a stack-relative limit.
  // Search the descendants of phiout for the probe-step INT_ADD(s) and their compares.
  uintb limitoff = 0;
  bool foundlimit = false;
  list<PcodeOp *>::const_iterator iter;
  for(iter=phiout->beginDescend();iter!=phiout->endDescend();++iter) {
    PcodeOp *addop = *iter;
    if (addop->code() != CPUI_INT_ADD) continue;
    if (!addop->getIn(1)->isConstant()) continue;
    if (addop->getIn(1)->getOffset() != page) continue;	// Same page step as the back edge
    Varnode *stepvn = addop->getOut();
    list<PcodeOp *>::const_iterator citer;
    for(citer=stepvn->beginDescend();citer!=stepvn->endDescend();++citer) {
      PcodeOp *cmpop = *citer;
      if (cmpop->code() != CPUI_INT_NOTEQUAL && cmpop->code() != CPUI_INT_EQUAL) continue;
      Varnode *other = (cmpop->getIn(0) == stepvn) ? cmpop->getIn(1) : cmpop->getIn(0);
      uintb loff;
      if (stackRelative(spIn,other,loff)) {
	limitoff = loff;
	foundlimit = true;
	break;
      }
    }
    if (foundlimit) break;
  }
  if (!foundlimit) return 0;

  // At loop exit `PHI + page == limit`, so the post-loop PHI value is `limit - page`
  // relative to the spacebase input.  Rewrite the MULTIEQUAL into a concrete add.
  int4 sz = spIn->getSize();
  uintb finaloff = (limitoff - page) & calc_mask(sz);
  vector<Varnode *> inlist;
  inlist.push_back(spIn);
  inlist.push_back(data.newConstant(sz,finaloff));
  data.opSetOpcode(op,CPUI_INT_ADD);
  data.opSetAllInput(op,inlist);
  return 1;
}

/// \class OptionStackProbeLoop
/// \brief (kuna) Toggle resolution of gcc stack-probe loops' stack-pointer MULTIEQUAL
string OptionStackProbeLoop::apply(Architecture *glb,const string &p1,const string &p2,const string &p3) const

{
  bool val = onOrOff(p1);
  glb->model_stack_probe_loop = val;
  string prop = val ? "on" : "off";
  return "Stack-probe-loop stack-pointer resolution turned "+prop;
}

} // End namespace ghidra

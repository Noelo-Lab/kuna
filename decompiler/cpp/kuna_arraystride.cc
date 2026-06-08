/* ###
 * kuna-owned: strided-induction (array-stride) recovery sub-stage (see kuna_arraystride.hh).
 */
#include "kuna_arraystride.hh"
#include "architecture.hh"
#include "funcdata.hh"

namespace ghidra {

// kuna ElementIds live in the 4000+ range (see kuna_returnpair.cc note).
ElementId ELEM_ARRAYSTRIDE = ElementId("arraystride",4015);

void RuleArrayStride::getOpList(vector<uint4> &oplist) const

{
  oplist.push_back(CPUI_MULTIEQUAL);
}

/// \brief Does \e vn carry the constant value 0 (directly, or via a single COPY)?
static bool kunaIsZeroInit(Varnode *vn)

{
  if (vn->isConstant())
    return vn->getOffset() == 0;
  if (vn->isWritten()) {
    PcodeOp *def = vn->getDef();
    if (def->code() == CPUI_COPY) {
      Varnode *src = def->getIn(0);
      if (src->isConstant())
        return src->getOffset() == 0;
    }
  }
  return false;
}

/// \brief Is \e phi a 2-input loop-header phi `MULTIEQUAL(#0, self + C)` ?
///
/// \param phi is the candidate MULTIEQUAL
/// \param initIdx receives the input index carrying the constant-0 init edge
/// \param backIdx receives the input index carrying the `self + C` loop-back edge
/// \param step receives the loop-back constant C (the per-iteration increment)
/// \param addop receives the back-edge INT_ADD op
/// \param requireLoneAdd when true the back-edge add must feed ONLY this phi
///        (so it can be destroyed safely); the counter sibling shares its
///        increment with the loop test, so it passes false.
/// \return \b true if \e phi has the unit/stride induction shape
static bool kunaIsInductionPhi(PcodeOp *phi,int4 &initIdx,int4 &backIdx,uintb &step,PcodeOp *&addop,bool requireLoneAdd)

{
  if (phi->numInput() != 2) return false;
  Varnode *out = phi->getOut();
  int4 zeroIdx = -1;
  int4 addIdx = -1;
  PcodeOp *foundadd = (PcodeOp *)0;
  uintb foundstep = 0;
  for(int4 i=0;i<2;++i) {
    Varnode *vn = phi->getIn(i);
    if (kunaIsZeroInit(vn)) {		// init edge must carry the constant 0
      zeroIdx = i;
    }
    else if (vn->isWritten()) {
      PcodeOp *def = vn->getDef();
      if (def->code() == CPUI_INT_ADD && (!requireLoneAdd || vn->loneDescend() == phi)) {
        // One operand of the add must be the phi's own output (the loop-back),
        // the other a nonzero constant step.
        Varnode *a = def->getIn(0);
        Varnode *b = def->getIn(1);
        Varnode *self = (Varnode *)0;
        Varnode *cst = (Varnode *)0;
        if (a == out) { self = a; cst = b; }
        else if (b == out) { self = b; cst = a; }
        if (self != (Varnode *)0 && cst->isConstant() && cst->getOffset() != 0) {
          addIdx = i;
          foundadd = def;
          foundstep = cst->getOffset();
        }
      }
    }
  }
  if (zeroIdx < 0 || addIdx < 0 || zeroIdx == addIdx) return false;
  initIdx = zeroIdx;
  backIdx = addIdx;
  step = foundstep;
  addop = foundadd;
  return true;
}

int4 RuleArrayStride::applyOp(PcodeOp *op,Funcdata &data)

{
  if (!data.getArch()->recover_array_stride) return 0;	// (kuna) default-off: byte-identical to upstream

  // `op` must be the strided OFFSET accumulator phi: MULTIEQUAL(#0, acc + STRIDE).
  int4 initIdx,backIdx;
  uintb stride;
  PcodeOp *accadd;
  if (!kunaIsInductionPhi(op,initIdx,backIdx,stride,accadd,true)) return 0;
  if (stride == 1) return 0;		// a unit step is the counter itself, not a stride

  Varnode *accout = op->getOut();
  BlockBasic *bl = op->getParent();
  if (bl == (BlockBasic *)0) return 0;

  // Find a sibling unit-step counter phi `cnt = MULTIEQUAL(#0, cnt + 1)` in the
  // SAME block.  Because both phis live in the same loop-header block they share
  // predecessor edges, so the init/back input indices line up edge-for-edge and
  // `accout == cnt * stride` holds on every iteration.
  Varnode *cntout = (Varnode *)0;
  for(list<PcodeOp *>::const_iterator iter=bl->beginOp();iter!=bl->endOp();++iter) {
    PcodeOp *cand = *iter;
    if (cand->code() != CPUI_MULTIEQUAL) break;	// MULTIEQUALs are all first in the block
    if (cand == op) continue;
    int4 ci,cb;
    uintb cstep;
    PcodeOp *cadd;
    if (!kunaIsInductionPhi(cand,ci,cb,cstep,cadd,false)) continue;
    if (cstep != 1) continue;			// counter must step by exactly 1
    if (ci != initIdx || cb != backIdx) continue;	// must match accumulator edge-for-edge
    cntout = cand->getOut();
    break;
  }
  if (cntout == (Varnode *)0) return 0;
  if (cntout->getSize() != accout->getSize()) return 0;	// keep the multiply width-consistent

  // Rewrite every use of the accumulator as `cnt * stride`.  Build the multiply
  // just after the block's MULTIEQUAL region (opInsertBegin places a non-phi op
  // right after the phis), then redirect all descendants of the accumulator.
  PcodeOp *multop = data.newOp(2,op->getAddr());
  data.opSetOpcode(multop,CPUI_INT_MULT);
  Varnode *multout = data.newUniqueOut(accout->getSize(),multop);
  data.opSetInput(multop,cntout,0);
  data.opSetInput(multop,data.newConstant(accout->getSize(),stride),1);
  data.opInsertBegin(multop,bl);

  data.totalReplace(accout,multout);	// every use of acc now reads cnt * stride
  data.opDestroy(op);			// the dead accumulator phi ...
  data.opDestroy(accadd);		// ... and its now-dead back-edge add
  return 1;
}

/// \class OptionArrayStride
/// \brief (kuna) Toggle strided-induction array-index recovery
string OptionArrayStride::apply(Architecture *glb,const string &p1,const string &p2,const string &p3) const

{
  bool val = onOrOff(p1);
  glb->recover_array_stride = val;
  string prop = val ? "on" : "off";
  return "Strided-induction array-index recovery turned "+prop;
}

} // End namespace ghidra

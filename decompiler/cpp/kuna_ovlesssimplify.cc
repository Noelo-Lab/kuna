/* ###
 * kuna-owned: overflow-flag signed-less-than simplification sub-stage
 * (see kuna_ovlesssimplify.hh).
 */
#include "kuna_ovlesssimplify.hh"
#include "funcdata.hh"
#include "architecture.hh"

namespace ghidra {

// kuna ElementIds live in the 4000+ range (see kuna_returnpair.cc note).
ElementId ELEM_OVLESSSIMPLIFY = ElementId("ovlesssimplify",4006);

/// \brief Return the INT_ADD def feeding \b vn, or null if \b vn is not `V + K` (K constant)
///
/// On success, \b base is set to V and \b koff to the constant K (already masked
/// to vn's size).
static PcodeOp *matchAddConst(Varnode *vn,Varnode **base,uintb *koff)

{
  if (!vn->isWritten()) return (PcodeOp *)0;
  PcodeOp *addop = vn->getDef();
  if (addop->code() != CPUI_INT_ADD) return (PcodeOp *)0;
  Varnode *a = addop->getIn(0);
  Varnode *b = addop->getIn(1);
  if (b->isConstant()) {
    *base = a;
    *koff = b->getOffset();
  }
  else if (a->isConstant()) {
    *base = b;
    *koff = a->getOffset();
  }
  else
    return (PcodeOp *)0;
  return addop;
}

/// \class RuleOvLessSimplify
/// \brief (kuna) Collapse the OV-flag signed-less-than idiom to a clean INT_SLESS

void RuleOvLessSimplify::getOpList(vector<uint4> &oplist) const

{
  oplist.push_back(CPUI_INT_NOTEQUAL);
}

int4 RuleOvLessSimplify::applyOp(PcodeOp *op,Funcdata &data)

{
  if (!data.getArch()->ov_less_simplify) return 0;	// (kuna) P0 assertion not set: byte-identical to upstream

  // Match  NE( SLESS(V+K,0) , BOOL_AND( signTest , SLESS(-1, V+K) ) )
  Varnode *in0 = op->getIn(0);
  Varnode *in1 = op->getIn(1);

  // in0 = INT_SLESS(R1, 0)
  if (!in0->isWritten()) return 0;
  PcodeOp *sless0 = in0->getDef();
  if (sless0->code() != CPUI_INT_SLESS) return 0;
  Varnode *r1 = sless0->getIn(0);
  Varnode *zero = sless0->getIn(1);
  if (!zero->isConstant() || zero->getOffset() != 0) return 0;

  // in1 = BOOL_AND(signTest, SLESS(-1, R2))
  if (!in1->isWritten()) return 0;
  PcodeOp *andop = in1->getDef();
  if (andop->code() != CPUI_BOOL_AND) return 0;

  // Locate the SLESS(-1, R2) limb of the AND; the other limb is the sign test of V.
  PcodeOp *slessHi = (PcodeOp *)0;
  for(int4 i=0;i<2;++i) {
    Varnode *limb = andop->getIn(i);
    if (!limb->isWritten()) continue;
    PcodeOp *cand = limb->getDef();
    if (cand->code() != CPUI_INT_SLESS) continue;
    Varnode *mone = cand->getIn(0);
    if (!mone->isConstant()) continue;
    if (mone->getOffset() != calc_mask(mone->getSize())) continue;	// must be -1
    slessHi = cand;
    break;
  }
  if (slessHi == (PcodeOp *)0) return 0;
  Varnode *r2 = slessHi->getIn(1);

  // R1 and R2 must both be `V + K` with the same V and same K.
  Varnode *base1,*base2;
  uintb k1,k2;
  if (matchAddConst(r1,&base1,&k1) == (PcodeOp *)0) return 0;
  if (matchAddConst(r2,&base2,&k2) == (PcodeOp *)0) return 0;
  if (base1 != base2) return 0;
  if (k1 != k2) return 0;
  if (r1->getSize() != r2->getSize()) return 0;

  // Collapse:  V s< -K   (negate K within V's size)
  int4 sz = base1->getSize();
  if (r1->getSize() != sz) return 0;	// ADD width must match V's width
  intb ksigned = sign_extend((intb)k1, sz*8 - 1);
  uintb negk = ((uintb)(-ksigned)) & calc_mask(sz);

  data.opSetOpcode(op,CPUI_INT_SLESS);
  data.opSetInput(op,base1,0);
  data.opSetInput(op,data.newConstant(sz,negk),1);
  return 1;
}

/// \class OptionOvLessSimplify
/// \brief (kuna) Toggle the OV-flag signed-less-than simplification
string OptionOvLessSimplify::apply(Architecture *glb,const string &p1,const string &p2,const string &p3) const

{
  bool val = onOrOff(p1);
  glb->ov_less_simplify = val;
  return "OV-flag signed-less-than simplification " + p1;
}

} // End namespace ghidra

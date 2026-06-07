/* ###
 * kuna-owned: boolean sign-extension-mask simplification sub-stage (see kuna_booleanmask.hh).
 */
#include "kuna_booleanmask.hh"
#include "architecture.hh"
#include "funcdata.hh"

namespace ghidra {

// kuna ElementIds live in the 4000+ range (see kuna_returnpair.cc note).
ElementId ELEM_BOOLEANMASK = ElementId("booleanmask",4005);

void RuleBoolSignShift::getOpList(vector<uint4> &oplist) const

{
  oplist.push_back(CPUI_INT_SRIGHT);
}

int4 RuleBoolSignShift::applyOp(PcodeOp *op,Funcdata &data)

{
  if (!data.getArch()->fold_boolean_mask) return 0;	// (kuna) default-off: byte-identical to upstream

  if (!op->getIn(1)->isConstant()) return 0;
  uintb sa = op->getIn(1)->getOffset();
  if (sa == 0) return 0;

  Varnode *shiftin = op->getIn(0);
  if (!shiftin->isWritten()) return 0;
  PcodeOp *leftshift = shiftin->getDef();
  if (leftshift->code() != CPUI_INT_LEFT) return 0;
  if (!leftshift->getIn(1)->isConstant()) return 0;
  if (leftshift->getIn(1)->getOffset() != sa) return 0;	// Left shift must be by same amount

  // RuleLeftRight already handles the byte-aligned case; defer to it there.
  if ((sa & 7) == 0) return 0;

  Varnode *boolvn = leftshift->getIn(0);
  int4 width = boolvn->getSize();
  // The value being shifted up must occupy only the low (width*8 - sa) bits, so
  // that the left-shift loses no information and the sign-extend recovers an
  // exact 0/-1 mask of it.  For a known-1-bit boolean (NZMask==1) this holds for
  // any sa >= 1.
  if ((uintb)width * 8 <= sa) return 0;
  uintb tophalf = ((uintb)1 << ((uintb)width * 8 - sa)) - 1;
  if ((boolvn->getNZMask() & ~tophalf) != 0) return 0;

  if (shiftin->loneDescend() != op) return 0;

  // Rewrite `(b << sa) s>> sa`  =>  `0 - b`  (INT_2COMP gives 0 or all-ones).
  // The now-unused left shift is left for dead-code elimination to remove; we
  // only rewire this op and never touch the left shift's output.
  data.opSetOpcode(op,CPUI_INT_2COMP);
  data.opSetInput(op,boolvn,0);
  data.opRemoveInput(op,1);			// drop the shift-amount constant
  return 1;
}

/// \class OptionBooleanMask
/// \brief (kuna) Toggle folding of boolean sign-extension-mask shift idioms
string OptionBooleanMask::apply(Architecture *glb,const string &p1,const string &p2,const string &p3) const

{
  bool val = onOrOff(p1);
  glb->fold_boolean_mask = val;
  string prop = val ? "on" : "off";
  return "Boolean sign-extension-mask folding turned "+prop;
}

} // End namespace ghidra

/* ###
 * kuna-owned: SPARC struct-return `unimp N` post-call fall-through (see kuna_sparcstructret.hh).
 */
#include "kuna_sparcstructret.hh"
#include "architecture.hh"
#include "op.hh"
#include "userop.hh"

namespace ghidra {

// kuna ElementIds live in the 4000+ range (see kuna_returnpair.cc note).
ElementId ELEM_SPARCSTRUCTRET = ElementId("sparcstructret",4016);

bool kunaIsSparcStructRetTrap(Funcdata &data,const PcodeOp *op)

{
  Architecture *glb = data.getArch();
  if (!glb->sparc_struct_return)		// default-off gate: PARITY OK with the option off
    return false;
  if (op->code() != CPUI_BRANCHIND)
    return false;
  // The SPARC spec emits `unimp`/`illtrap` as
  //   dest = IllegalInstructionTrap(N) ; goto [dest]
  // so the BRANCHIND is the LAST op of an instruction whose other op is an
  // IllegalInstructionTrap CALLOTHER.  At this (raw, pre-SSA) flow stage the
  // input Varnode is not yet def-linked, so identify the trap positionally:
  // walk back over the ops belonging to the same instruction (i.e. up to the
  // preceding instruction-start) looking for the trap CALLOTHER.
  list<PcodeOp *>::const_iterator iter = op->getInsertIter();
  list<PcodeOp *>::const_iterator beg = data.beginOpDead();
  const PcodeOp *cur = op;
  for (;;) {
    if (cur->code() == CPUI_CALLOTHER) {
      const Varnode *idvn = cur->getIn(0);	// first input of CALLOTHER is the user-op id
      if (idvn != (const Varnode *)0 && idvn->isConstant()) {
	UserPcodeOp *userop = glb->userops.getOp((uint4)idvn->getOffset());
	if (userop != (UserPcodeOp *)0 && userop->getName() == "IllegalInstructionTrap")
	  return true;
      }
    }
    if (cur->isInstructionStart())		// reached the first op of this instruction
      break;
    if (iter == beg)
      break;
    --iter;
    cur = *iter;
  }
  return false;
}

/// \class OptionSparcStructRet
/// \brief (kuna) Toggle SPARC struct-return `unimp` BRANCHIND -> fall-through (GH-6882)
string OptionSparcStructRet::apply(Architecture *glb,const string &p1,const string &p2,const string &p3) const

{
  bool val = onOrOff(p1);
  glb->sparc_struct_return = val;
  string prop = val ? "on" : "off";
  return "SPARC struct-return (post-call unimp) fall-through turned "+prop;
}

} // End namespace ghidra

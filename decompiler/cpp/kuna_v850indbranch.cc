/* ###
 * kuna-owned: V850 "jmp [reg]" indirect-branch reclassification (see kuna_v850indbranch.hh).
 */
#include "kuna_v850indbranch.hh"
#include "architecture.hh"
#include "op.hh"

namespace ghidra {

// kuna ElementIds live in the 4000+ range (see kuna_returnpair.cc note).
ElementId ELEM_V850INDIRECTBRANCH = ElementId("v850indirectbranch",4007);

bool kunaIsV850IndirectJmp(Funcdata &data,const PcodeOp *op)

{
  Architecture *glb = data.getArch();
  if (!glb->v850_indirect_branch)		// default-off gate: PARITY OK with the option off
    return false;
  if (op->code() != CPUI_CALLIND)
    return false;
  // The V850 spec models "jmp [reg]" (reg != lp) as `call [reg]`, the only
  // CALLIND on V850 whose called value is a plain processor register.
  const Varnode *vn = op->getIn(0);
  if (vn == (const Varnode *)0)
    return false;
  AddrSpace *spc = vn->getSpace();
  if (spc->getType() != IPTR_PROCESSOR)		// must be a hardware register
    return false;
  // Exclude the program counter: the other V850 CALLINDs are `call [PC]`.
  string regname = glb->translate->getRegisterName(spc,vn->getOffset(),vn->getSize());
  if (regname.empty())				// only a named register (not a raw memory load)
    return false;
  if (regname == "PC" || regname == "pc")
    return false;
  return true;
}

/// \class OptionV850IndirectBranch
/// \brief (kuna) Toggle V850 "jmp [reg]" CALLIND -> BRANCHIND reclassification (GH-8817)
string OptionV850IndirectBranch::apply(Architecture *glb,const string &p1,const string &p2,const string &p3) const

{
  bool val = onOrOff(p1);
  glb->v850_indirect_branch = val;
  string prop = val ? "on" : "off";
  return "V850 indirect-branch (jmp [reg]) reclassification turned "+prop;
}

} // End namespace ghidra

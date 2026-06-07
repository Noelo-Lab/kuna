/* ###
 * kuna-owned: function-entry constant-pointer inference (see kuna_inferfuncentry.hh).
 */
#include "kuna_inferfuncentry.hh"
#include "architecture.hh"

namespace ghidra {

// kuna ElementIds live in the 4000+ range to stay clear of upstream ids (max ~290).
ElementId ELEM_INFERFUNCENTRY = ElementId("inferfuncentry",4003);

bool kunaIsFunctionEntry(Funcdata &data,AddrSpace *spc,const Varnode *vn,const PcodeOp *op)

{
  Architecture *glb = data.getArch();
  if (!glb->infer_funcentry)
    return false;
  uintb fullEncoding;
  Address rampoint = glb->resolveConstant(spc,vn->getOffset(),vn->getSize(),op->getAddr(),fullEncoding);
  if (rampoint.isInvalid())
    return false;
  // Exact function-entry match only: the relaxation must not reclassify an
  // ordinary single-bit integer constant.
  Funcdata *fd = data.getScopeLocal()->getParent()->queryFunction(rampoint);
  return (fd != (Funcdata *)0 && fd->getAddress() == rampoint);
}

/// \class OptionInferFuncEntry
/// \brief (kuna) Toggle inference of function entries at single-bit image bases
string OptionInferFuncEntry::apply(Architecture *glb,const string &p1,const string &p2,const string &p3) const

{
  bool val = onOrOff(p1);
  glb->infer_funcentry = val;
  string prop = val ? "on" : "off";
  return "Function-entry constant-pointer inference turned "+prop;
}

} // End namespace ghidra

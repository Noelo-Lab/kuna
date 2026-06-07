/* ###
 * kuna-owned: Thumb function-pointer preservation sub-stage (see kuna_thumbfuncptr.hh).
 */
#include "kuna_thumbfuncptr.hh"
#include "architecture.hh"

namespace ghidra {

// kuna ElementIds live in the 4000+ range to stay clear of upstream ids (max ~290).
ElementId ELEM_THUMBFUNCPTR = ElementId("thumbfuncptr",4002);

bool kunaPreserveThumbFuncPtr(Varnode *basevn,PcodeOp *op,int8 val,int8 extra,int8 multiplier,Architecture *glb)

{
  if (!glb->preserve_thumb_funcptr || glb->funcptr_align == 0)
    return false;
  if (multiplier > 1)
    return false;		// Multiplier 1 = byte-grain PTRADD form of the same constant offset
  Datatype *bt = basevn->getTypeReadFacing(op);
  if (bt->getMetatype() != TYPE_PTR)
    return false;
  TypePointer *ptype = (TypePointer *)bt;
  Datatype *pt = ptype->getPtrTo();
  if (pt->getMetatype() != TYPE_SPACEBASE)
    return false;
  int8 newoff = AddrSpace::addressToByteInt(val,ptype->getWordSize());
  Datatype *sub = pt->getSubType(newoff,&newoff);
  if (sub == (Datatype *)0 || newoff != 0)
    return false;
  if (sub->getMetatype() != TYPE_CODE)
    return false;		// Only function/code symbols carry the mode bit
  int8 bextra = AddrSpace::addressToByteInt(extra,ptype->getWordSize());
  if (bextra <= 0 || bextra >= ((int8)1 << glb->funcptr_align))
    return false;		// Offset must be exactly the alignment-encoding bits
  return true;
}

/// \class OptionThumbFuncPtr
/// \brief (kuna) Toggle preservation of mode-bit-encoded (Thumb) function pointers
string OptionThumbFuncPtr::apply(Architecture *glb,const string &p1,const string &p2,const string &p3) const

{
  bool val = onOrOff(p1);
  glb->preserve_thumb_funcptr = val;
  string prop = val ? "on" : "off";
  return "Thumb function-pointer preservation turned "+prop;
}

} // End namespace ghidra

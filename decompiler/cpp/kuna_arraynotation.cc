/* ###
 * kuna-owned: pointer-notation presentation sub-stage (see kuna_arraynotation.hh).
 */
#include "kuna_arraynotation.hh"
#include "printc.hh"
#include "architecture.hh"

namespace ghidra {

// kuna ElementIds live in the 4000+ range to stay clear of upstream ids (max ~290).
ElementId ELEM_ARRAYNOTATION = ElementId("arraynotation",4001);

/// \class OptionArrayNotation
/// \brief (kuna) Toggle array-notation rendering of standalone pointer arithmetic
string OptionArrayNotation::apply(Architecture *glb,const string &p1,const string &p2,const string &p3) const

{
  bool val = onOrOff(p1);
  PrintC *lng = dynamic_cast<PrintC *>(glb->print);
  if (lng == (PrintC *)0)
    return "Can only set array notation for C language";
  lng->setArrayNotation(val);
  string prop = val ? "on" : "off";
  return "Array notation for pointer arithmetic turned "+prop;
}

} // End namespace ghidra

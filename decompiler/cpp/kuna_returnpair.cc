/* ###
 * kuna-owned: return-register pairing sub-stage (see kuna_returnpair.hh).
 */
#include "kuna_returnpair.hh"
#include "architecture.hh"

namespace ghidra {

/// \class OptionReturnPair
/// \brief (kuna) Toggle joining of multi-register return values
string OptionReturnPair::apply(Architecture *glb,const string &p1,const string &p2,const string &p3) const

{
  bool single;
  if (p1 == "single")
    single = true;
  else if (p1 == "pair")
    single = false;
  else
    return "Must specify pair or single";
  glb->return_single = single;
  return "Return-register pairing set to "+p1+" form";
}

} // End namespace ghidra

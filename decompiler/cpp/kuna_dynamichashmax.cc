/* ###
 * kuna-owned: dynamic-hash collision budget sub-stage (see kuna_dynamichashmax.hh).
 */
#include "kuna_dynamichashmax.hh"
#include "architecture.hh"

namespace ghidra {

// kuna ElementIds live in the 4000+ range (see kuna_returnpair.cc note).
ElementId ELEM_DYNAMICHASHMAX = ElementId("dynamichashmax",4013);

/// \class OptionDynamicHashMax
/// \brief (kuna) Toggle the wider (16) DynamicHash collision budget
string OptionDynamicHashMax::apply(Architecture *glb,const string &p1,const string &p2,const string &p3) const

{
  bool val = onOrOff(p1);
  glb->dynamic_hash_maxdup_high = val;
  string prop = val ? "on" : "off";
  return "DynamicHash wide collision budget turned "+prop;
}

} // End namespace ghidra

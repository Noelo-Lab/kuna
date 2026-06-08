/* ###
 * kuna-owned: input-varnode reconciliation sub-stage (see kuna_inputvarnodeadjust.hh).
 */
#include "kuna_inputvarnodeadjust.hh"
#include "architecture.hh"

namespace ghidra {

// kuna ElementIds live in the 4000+ range to stay clear of upstream ids (max ~290).
// Required: OptionDatabase::registerOption keys on ElementId::find(name) -- options
// without a registered ElementId collide on ELEM_UNKNOWN.
ElementId ELEM_INPUTVARNODEADJUST = ElementId("inputvarnodeadjust",4018);

/// \class OptionInputVarnodeAdjust
/// \brief (kuna) Toggle symmetric input-Varnode container widening (GH-9218)
string OptionInputVarnodeAdjust::apply(Architecture *glb,const string &p1,const string &p2,const string &p3) const

{
  bool val = onOrOff(p1);
  glb->input_varnode_adjust = val;
  string prop = val ? "on" : "off";
  return "Symmetric input-varnode container widening turned "+prop;
}

} // End namespace ghidra

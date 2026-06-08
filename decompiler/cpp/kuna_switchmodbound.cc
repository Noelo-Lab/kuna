/* ###
 * kuna-owned: modulo/and-mask jump-table index-bound option (see kuna_switchmodbound.hh).
 *
 * The recovery logic itself (kunaTryModuloBoundTable) is a member of JumpBasic
 * and lives next to the rest of the model in jumptable.cc -- it needs the
 * protected pathMeld / jrange / varnodeIndex state.  This file only owns the
 * ElementId and the ArchOption that sets the arch flag.
 */
#include "kuna_switchmodbound.hh"
#include "architecture.hh"

namespace ghidra {

// kuna ElementIds live in the 4000+ range (see kuna_returnpair.cc note).
ElementId ELEM_SWITCHMODBOUND = ElementId("switchmodbound",4011);

/// \class OptionSwitchModBound
/// \brief (kuna) Toggle bounding a LOAD-table jumptable by a modulo/and-mask on its index (GH-9191)
string OptionSwitchModBound::apply(Architecture *glb,const string &p1,const string &p2,const string &p3) const

{
  bool val = onOrOff(p1);
  glb->switch_modulo_bound = val;
  string prop = val ? "on" : "off";
  return "Jump-table modulo/and-mask index bounding turned "+prop;
}

} // End namespace ghidra

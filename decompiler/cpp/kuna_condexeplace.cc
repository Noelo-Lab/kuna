/* ###
 * kuna-owned: conditional-constant COPY block-placement sub-stage (see kuna_condexeplace.hh).
 *
 * The actual gate is a minimal guarded block at the decision point in
 * ActionConditionalConst::handlePhiNodes (coreaction.cc); this file only owns
 * the OptionCondExePlace surface and its ElementId.
 */
#include "kuna_condexeplace.hh"
#include "architecture.hh"

namespace ghidra {

// kuna ElementIds live in the 4000+ range to stay clear of upstream ids (max ~290).
// Required: OptionDatabase::registerOption keys on ElementId::find(name) — options
// without a registered ElementId collide on ELEM_UNKNOWN.
ElementId ELEM_CONDEXEPLACE = ElementId("condexeplace",4017);

/// \class OptionCondExePlace
/// \brief (kuna) Toggle whether ActionConditionalConst may place a const COPY in a loop block
string OptionCondExePlace::apply(Architecture *glb,const string &p1,const string &p2,const string &p3) const

{
  bool val = onOrOff(p1);
  glb->condexe_block_placement = val;
  string prop = val ? "on" : "off";
  return "Conditional-constant loop-block placement guard turned "+prop;
}

} // End namespace ghidra

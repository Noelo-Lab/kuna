/* ###
 * kuna-owned: stack-alias dead-store ordering sub-stage (see kuna_stackalias.hh).
 *
 * The behavioral edit is an anchor edit in coreaction.cc
 * (ActionDeadCode::lastChanceLoad), gated by the arch flag
 * stack_alias_deadstore.  This file only carries the ElementId and the
 * OptionStackAlias toggle.
 */
#include "kuna_stackalias.hh"
#include "architecture.hh"

namespace ghidra {

// kuna ElementIds live in the 4000+ range to stay clear of upstream ids (max ~290).
ElementId ELEM_STACKALIAS = ElementId("stackalias",4014);

/// \class OptionStackAlias
/// \brief (kuna) Toggle preservation of a store-through-a-stack-pointer-alias across the deadcode race
string OptionStackAlias::apply(Architecture *glb,const string &p1,const string &p2,const string &p3) const

{
  bool val = onOrOff(p1);
  glb->stack_alias_deadstore = val;
  string prop = val ? "on" : "off";
  return "Stack-alias dead-store preservation turned "+prop;
}

} // End namespace ghidra

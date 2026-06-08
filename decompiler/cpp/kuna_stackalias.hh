/* ###
 * kuna-owned: stack-alias dead-store ordering sub-stage (GH-8500).
 *
 * This file is NOT part of upstream Ghidra.  It exposes a LATENT S6
 * "alias-facets" / heritage-ordering decision (stage-model.md):
 *
 *   When the address of a stack slot is spilled into another stack slot
 *   (a pointer-to-stack alias, `int *p = &x;`) and a value is then stored
 *   THROUGH that pointer (`*p = x;`) and re-read through it (`return *p;`),
 *   the indirect STORE is converted to a stack varnode (s-OFF = x) and then
 *   dead-code-eliminated in the SAME main-loop iteration -- one step BEFORE
 *   the aliasing LOAD (`*p`) collapses to read the same s-OFF.  With the
 *   store already gone, the LOAD re-heritages s-OFF with no reaching
 *   definition and resolves to an *uninitialized* stack input, so the
 *   rendered C returns an undefined local (GH-8500: `return xStack_1c;`).
 *
 *   This is purely an ORDERING artifact of the deadcode pass running before
 *   the indirect LOAD that reads the just-converted store has resolved.  The
 *   decompiler already has a mechanism for exactly this race --
 *   ActionDeadCode::lastChanceLoad defers dead-store removal for one round
 *   when a not-yet-resolved LOAD has a pointer that is an "eventual constant"
 *   (a stack offset that hasn't been folded yet).  It does NOT cover the case
 *   where the LOAD's pointer is itself a value loaded FROM the stack (the
 *   spilled `p`), because such a pointer is not eventual-constant.
 *
 * The decision is a P0 assertion: `option stackalias on|off` (default "off" =
 * upstream behavior, byte-identical output).  When on, lastChanceLoad also
 * recognizes a LOAD whose pointer lives in the stack address space (a
 * pointer-to-stack alias) and conservatively holds the function's stack
 * stores alive for that deadcode round, so the store-through-pointer survives
 * until the alias resolves and the LOAD links to it correctly.
 *
 * This is an ANCHOR-EDIT fix (no new Rule): the guarded logic lives in
 * ActionDeadCode::lastChanceLoad (coreaction.cc), reached only when the arch
 * flag stack_alias_deadstore is set.  Only the OptionXxx class lives here.
 */
#ifndef __KUNA_STACKALIAS_HH__
#define __KUNA_STACKALIAS_HH__

#include "options.hh"

namespace ghidra {

extern ElementId ELEM_STACKALIAS;	///< Marshaling element \<stackalias> (kuna)

/// \brief (kuna) Set whether store-through-a-stack-pointer-alias is preserved across the deadcode race: `stackalias on|off`
class OptionStackAlias : public ArchOption {
public:
  OptionStackAlias(void) { name = "stackalias"; }	///< Constructor
  virtual string apply(Architecture *glb,const string &p1,const string &p2,const string &p3) const;
};

} // End namespace ghidra
#endif

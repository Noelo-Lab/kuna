/* ###
 * kuna-owned: lowered comparison-cascade -> switch recovery (port of angr's
 * LoweredSwitchSimplifier; see Basque et al., USENIX Security 2024 "SAILR").
 *
 * This file is NOT part of upstream Ghidra.  It exposes a structural gap between
 * angr's and Ghidra's switch handling:
 *
 *   GCC lowers a dense `switch (V)` over a small variable into a balanced
 *   BINARY-SEARCH TREE of `if (V == c) ... else if (V < k) ...` comparisons
 *   (no jump table).  Ghidra renders a `switch` ONLY from a CPUI_BRANCHIND op
 *   backed by a JumpTable (an S2 artifact recovered from a lifted indirect
 *   jump).  When the compiler lowered the switch to comparisons there is no
 *   BRANCHIND -- so Ghidra (like stock angr) renders a deep if/else chain.
 *
 *   angr's pass edits the structured region graph (its S7 artifact) and emits a
 *   SwitchCase node.  Ghidra has no region-graph edit hook: a BlockSwitch is
 *   only ever produced by CollapseStructure::ruleBlockSwitch from a BlockBasic
 *   whose f_switch_out flag is set, which happens only for a BRANCHIND-terminated
 *   block with a linked JumpTable.  So the kuna port MANUFACTURES the S2 artifact
 *   (a BRANCHIND + a hand-built, pre-labelled JumpTable) from the comparison
 *   cascade and lets the existing structurer + printer do the rest.  This
 *   inverts the normal S1->S2 dependency (lifting produces the BRANCHIND that S2
 *   explains; here a late pass produces the BRANCHIND from a control-flow
 *   pattern) -- a stage-model feedback edge (docs/divergences.md).
 *
 * To keep SSA/phi (MULTIEQUAL) state correct through the CFG surgery, the port
 * uses the SAME restart idiom Ghidra already uses for multistage jump tables
 * (JumpTable::matchModel -> Override::insertMultistageJump + setRestartPending):
 *
 *   - ActionLowerSwitchDetect runs LATE (after ActionSwitchNorm), on the fully
 *     simplified CFG where comparisons are clean `INT_EQUAL(V,const)` form.  It
 *     only READS: it detects the cascade, records it in a survives-clear() side
 *     table (keyed by function identity, addresses only -- no Varnode/Block
 *     pointers), and requests a restart.
 *   - ActionLowerSwitchInstall runs EARLY on the restart, BEFORE ActionHeritage,
 *     when blocks exist but SSA does not (no MULTIEQUALs to break).  It performs
 *     the CFG surgery (CBRANCH-cascade head -> BRANCHIND, rewire out-edges to the
 *     recorded case targets + default, build the labelled JumpTable) so the
 *     following ActionHeritage rebuilds phi from scratch over the corrected CFG.
 *
 * The decision is a P0 assertion: `option loweredswitch on|off` (default "off"
 * = upstream byte-identical output; the Actions are inert when off).
 */
#ifndef __KUNA_LOWEREDSWITCH_HH__
#define __KUNA_LOWEREDSWITCH_HH__

#include "action.hh"
#include "options.hh"

namespace ghidra {

extern ElementId ELEM_LOWEREDSWITCH;	///< Marshaling element \<loweredswitch> (kuna)

/// \brief (kuna) Has a lowered-switch record already been stored for \e fd?
///
/// True on the post-restart pass (detection self-gate / install trigger).
bool kunaLoweredSwitchHasRecord(const Funcdata &fd);

/// \brief (kuna) Detect a compiler-lowered comparison cascade and request a restart
///
/// Runs after ActionSwitchNorm on the simplified CFG.  On finding a cascade it
/// records (branch address, switch variable storage, ordered case constants +
/// targets, default target) in the side table and sets the restart-pending flag.
/// Makes no change to ops (returns 0) so the fullloop quiesces and the outer
/// ActionRestartGroup fires the restart.
class ActionLowerSwitchDetect : public Action {
public:
  ActionLowerSwitchDetect(const string &g) : Action(0,"lowerswitchdetect",g) {}	///< Constructor
  virtual Action *clone(const ActionGroupList &grouplist) const {
    if (!grouplist.contains(getGroup())) return (Action *)0;
    return new ActionLowerSwitchDetect(getGroup());
  }
  virtual int4 apply(Funcdata &data);
};

/// \brief (kuna) Install the recorded lowered switch as a BRANCHIND + JumpTable
///
/// Runs before ActionHeritage on the restart pass (pre-SSA).  Reads the side
/// table; if a record exists and is not yet installed, performs the CFG surgery
/// via Funcdata::kunaInstallLoweredSwitch and forces a structure reset so the
/// following heritage + structuring see a real switch.
class ActionLowerSwitchInstall : public Action {
public:
  ActionLowerSwitchInstall(const string &g) : Action(0,"lowerswitchinstall",g) {}	///< Constructor
  virtual Action *clone(const ActionGroupList &grouplist) const {
    if (!grouplist.contains(getGroup())) return (Action *)0;
    return new ActionLowerSwitchInstall(getGroup());
  }
  virtual int4 apply(Funcdata &data);
};

/// \brief (kuna) Toggle lowered comparison-cascade -> switch recovery: `loweredswitch on|off`
///
/// "off" (default) is upstream byte-identical (both Actions inert).  "on"
/// reconstructs a GCC-lowered binary-search comparison cascade into a switch.
class OptionLowerSwitch : public ArchOption {
public:
  OptionLowerSwitch(void) { name = "loweredswitch"; }	///< Constructor
  virtual string apply(Architecture *glb,const string &p1,const string &p2,const string &p3) const;
};

} // End namespace ghidra
#endif

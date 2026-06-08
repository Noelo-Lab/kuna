/* ###
 * kuna-owned: modulo/and-mask jump-table index-bound sub-stage (GH-9191, GH-6674).
 *
 * This file is NOT part of upstream Ghidra.  It exposes a LATENT corner of the
 * S2 "switch-model / jumptable-recovery" sub-stage (stage-model.md):
 *
 *   The basic jump-table model (jumptable.cc, JumpBasic::recoverModel) bounds
 *   the address table by the range of a "normalized switch variable".  It
 *   already recognizes an explicit guard (a CBRANCH range check) and a direct
 *   INT_AND mask on that variable (JumpBasic::getMaxValue).  But when the table
 *   index is bounded ONLY by an in-band arithmetic operation -- a modulo
 *   `index % N` (CPUI_INT_REM / CPUI_INT_SREM by a constant) -- and the
 *   BRANCHIND value is a table LOAD whose pointer is `index*stride + base`, the
 *   index is not recognized as a bound.  With no guard and no AND-mask, the
 *   normalized variable defaults to the full width of the LOADed value, the
 *   range vastly exceeds maxtablesize, and recovery aborts with
 *   "Could not recover jumptable ... Too many branches" (GH-9191).  The C then
 *   renders the dispatch as a computed call through a function pointer.
 *
 * The decision is a P0 assertion: `option switchmodbound on|off` (default
 * "off" = upstream behavior, byte-identical output).  When on, if the normal
 * model failed to bound the table, kunaTryModuloBoundTable walks the LOAD-table
 * pointer chain `BRANCHIND <- LOAD <- (index*stride + base)` (allowing the
 * linear realigning ops INT_ADD/INT_MULT/INT_LEFT/SUBPIECE/ZEXT/SEXT with a
 * constant other-input), and if the index is bounded by `% N` or `& mask` with
 * N (resp. mask+1) <= maxtablesize, it rebuilds the model so the table iterates
 * the index over [0, N-1].  The table is then recovered with exactly N entries.
 *
 * The walk is strictly shape-matched (single non-constant input at each step,
 * an explicit modulo/mask leaf, bound <= maxtablesize) so it only fires on the
 * latent pattern and leaves all other functions byte-identical.
 */
#ifndef __KUNA_SWITCHMODBOUND_HH__
#define __KUNA_SWITCHMODBOUND_HH__

#include "options.hh"

namespace ghidra {

extern ElementId ELEM_SWITCHMODBOUND;	///< Marshaling element \<switchmodbound> (kuna)

/// \brief (kuna) Set whether a modulo/and-mask on a jump-table index bounds the table: `switchmodbound on|off`
class OptionSwitchModBound : public ArchOption {
public:
  OptionSwitchModBound(void) { name = "switchmodbound"; }	///< Constructor
  virtual string apply(Architecture *glb,const string &p1,const string &p2,const string &p3) const;
};

} // End namespace ghidra
#endif

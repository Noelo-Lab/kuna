/* ###
 * kuna-owned: stack-probe-loop stack-pointer resolution sub-stage (GH-8017, GH-6858).
 *
 * This file is NOT part of upstream Ghidra.  It exposes a LATENT S2
 * "stack-pointer normalization" decision (stage-model.md):
 *
 *   gcc's stack-clash / -fstack-check prologue probes a large frame one page
 *   at a time with a loop:
 *
 *       lea  limit,[sp - frame]      ; limit = SP_in + limit_const
 *     L: sub  sp, 0x1000             ; probe step = one page
 *       or   [sp], 0                 ; touch the page
 *       cmp  sp, limit
 *       jne  L
 *       sub  sp, tail
 *
 *   After heritage the loop-carried stack pointer is a spacebase MULTIEQUAL
 *   PHI whose back-edge is `PHI - page`.  The StackSolver/spacebase tracker
 *   cannot break that self-referential cycle (`PHI = PHI - page` has no
 *   constant solution), so the stack pointer never resolves to a fixed offset
 *   from SP_in past the loop.  Every post-loop local then renders relative to
 *   the loop induction variable as `&pxVar3[-0x1000]` noise (GH-8017), and
 *   call-argument stores at unmatched offsets are dropped, so calls render
 *   with no arguments (GH-6858: `func_0x000010d0();`).
 *
 *   The loop's exit comparison pins the value: it iterates until
 *   `PHI - page == limit`, so the post-loop value of the PHI is exactly
 *   `limit - page == SP_in + (limit_const - page)`, a fixed offset from the
 *   spacebase input.
 *
 * The decision is a P0 assertion: `option stackprobeloop on|off` (default
 * "off" = upstream behavior, byte-identical output).  When on,
 * RuleStackProbeLoop recognizes this exact shape -- a 2-input spacebase
 * MULTIEQUAL whose back-edge is `PHI - page` and whose other input is
 * stack-relative, with a loop-exit compare of `PHI - page` against a
 * stack-relative limit -- and rewrites the MULTIEQUAL into the concrete
 * `INT_ADD(SP_in, limit_const - page)`.  Breaking the cycle lets the existing
 * spacebase tracking resolve the whole frame, recovering clean `axStack_...`
 * locals and call arguments.
 */
#ifndef __KUNA_STACKPROBELOOP_HH__
#define __KUNA_STACKPROBELOOP_HH__

#include "action.hh"
#include "options.hh"

namespace ghidra {

extern ElementId ELEM_STACKPROBELOOP;	///< Marshaling element \<stackprobeloop> (kuna)

/// \brief (kuna) Resolve a gcc stack-probe loop's stack-pointer MULTIEQUAL to a constant offset
///
/// Matches a 2-input CPUI_MULTIEQUAL producing the stack-pointer register, where the
/// back-edge input is `PHI - page` (the probe decrement), the other input is stack-relative,
/// and the loop exit compares `PHI - page` against a stack-relative limit.  Rewrites the
/// MULTIEQUAL into `INT_ADD(spacebase_input, limit_const - page)`.  Only fires when
/// `option stackprobeloop on` (the apply body returns 0 immediately when the arch flag is off,
/// preserving byte-identical upstream output by default).
class RuleStackProbeLoop : public Rule {
public:
  RuleStackProbeLoop(const string &g) : Rule(g, 0, "stackprobeloop") {}	///< Constructor
  virtual Rule *clone(const ActionGroupList &grouplist) const {
    if (!grouplist.contains(getGroup())) return (Rule *)0;
    return new RuleStackProbeLoop(getGroup());
  }
  virtual void getOpList(vector<uint4> &oplist) const;
  virtual int4 applyOp(PcodeOp *op,Funcdata &data);
};

/// \brief (kuna) Set whether gcc stack-probe loops resolve the stack pointer: `stackprobeloop on|off`
class OptionStackProbeLoop : public ArchOption {
public:
  OptionStackProbeLoop(void) { name = "stackprobeloop"; }	///< Constructor
  virtual string apply(Architecture *glb,const string &p1,const string &p2,const string &p3) const;
};

} // End namespace ghidra
#endif

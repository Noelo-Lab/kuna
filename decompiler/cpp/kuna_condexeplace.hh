/* ###
 * kuna-owned: conditional-constant COPY block-placement sub-stage (GH-9203).
 *
 * This file is NOT part of upstream Ghidra.  It exposes a corner of the S3
 * "simplification-quiescence" decision (stage-model.md): WHERE a late
 * conditional-branch simplification materializes a freshly-created constant.
 *
 *   ActionConditionalConst (coreaction.cc) propagates a value known-constant
 *   along a conditional out-edge.  When that value reaches a MULTIEQUAL on a
 *   "disconnected" incoming edge, handlePhiNodes() materializes the constant as
 *   a COPY op placed at the BOTTOM of the edge's predecessor block
 *   (`op->getParent()->getIn(slot)`) and rewires the MULTIEQUAL input to it.
 *
 *   If that predecessor block is a LOOP block (a back-edge re-enters it), the
 *   freshly-created `tmp = 0` COPY lands *inside the loop body*, so it is
 *   re-executed every iteration and renders as a spurious assignment inside the
 *   loop (GH-9203: a malformed `do { ...; xVar2 = 0; ... } while (...)`).  The
 *   constant only needs to be assigned once; it does not belong in the loop.
 *
 * The decision is a P0 assertion: `option condexeplace on|off` (default "off" =
 * upstream behavior, byte-identical output).  When ON, handlePhiNodes() declines
 * to materialize the constant COPY into a loop predecessor block, leaving the
 * MULTIEQUAL input edge unchanged so the do/while body no longer contains the
 * out-of-place `= 0`.  Gated by the default-off arch flag
 * condexe_block_placement.
 */
#ifndef __KUNA_CONDEXEPLACE_HH__
#define __KUNA_CONDEXEPLACE_HH__

#include "options.hh"

namespace ghidra {

extern ElementId ELEM_CONDEXEPLACE;	///< Marshaling element \<condexeplace> (kuna)

/// \brief (kuna) Set whether conditional-constant COPYs may land in a loop block: `condexeplace on|off`
class OptionCondExePlace : public ArchOption {
public:
  OptionCondExePlace(void) { name = "condexeplace"; }	///< Constructor
  virtual string apply(Architecture *glb,const string &p1,const string &p2,const string &p3) const;
};

} // End namespace ghidra
#endif

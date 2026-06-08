/* ###
 * kuna-owned: strided-induction (array-stride) recovery sub-stage (GH-8724).
 *
 * This file is NOT part of upstream Ghidra.  It exposes a LATENT S3
 * "simplification-quiescence" decision (docs/stage-model.md):
 *
 *   A compiler routinely strength-reduces an array walk `&base[i]` (i.e.
 *   `base + i * sizeof(*base)`) into a loop-carried POINTER/OFFSET accumulator:
 *   it keeps a running offset `acc` that starts at 0 and is bumped by the
 *   element size (the "stride") on every back-edge, in lock-step with the loop
 *   counter `i` (`i += 1`).  The original multiply `i * stride` never appears
 *   in the machine code, so Ghidra's pointer/PTRADD recovery (RulePtrArith and
 *   friends) has nothing to latch onto: the index `i` is invisible and the C
 *   renders the raw strided accumulator instead (GH-8724:
 *       iVar3 = iVar3 + 0x414;  ...  *param_1 + iVar3 ...
 *   where 0x414 is sizeof(*param_1)).
 *
 *   In SSA the pattern is two sibling MULTIEQUALs in the SAME loop-header block:
 *       acc = MULTIEQUAL(#0, acc + STRIDE)     // strided offset accumulator
 *       cnt = MULTIEQUAL(#0, cnt + 1)          // unit-step loop counter
 *   Because both phis live in the same block they share predecessor edges, both
 *   start at 0, and the increments line up edge-for-edge, so on EVERY iteration
 *   `acc == cnt * STRIDE` exactly.  This is a provable algebraic identity, not a
 *   heuristic: it is the classic reverse of induction-variable strength
 *   reduction (SCEV) restricted to the one shape a stride accumulator produces.
 *
 * The decision is a P0 assertion: `option arraystride on|off` (default "off" =
 * upstream behavior, byte-identical output).  When on, RuleArrayStride matches
 * the accumulator phi, finds its unit-step counter sibling, and replaces every
 * use of `acc` with `cnt * STRIDE`, re-exposing the index `i`.  The now-dead
 * accumulator phi and its back-edge add fall out to dead-code elimination, and
 * the recovered `i * STRIDE` feeds the normal pointer/PTRADD recovery so the
 * index becomes visible in the rendered C.
 */
#ifndef __KUNA_ARRAYSTRIDE_HH__
#define __KUNA_ARRAYSTRIDE_HH__

#include "action.hh"
#include "options.hh"

namespace ghidra {

extern ElementId ELEM_ARRAYSTRIDE;	///< Marshaling element \<arraystride> (kuna)

/// \brief (kuna) Re-expose a strided-induction index: rewrite a stride accumulator phi as `counter * stride` (GH-8724)
///
/// Triggers on CPUI_MULTIEQUAL.  Recognizes a two-input loop-header phi whose
/// init edge is constant 0 and whose loop-back edge is `self + STRIDE` (STRIDE a
/// nonzero, non-one constant), then locates a sibling unit-step counter phi in
/// the same block (`MULTIEQUAL(#0, cnt + 1)` on the same edges).  Replaces the
/// accumulator with `INT_MULT(cnt, STRIDE)`.  Only fires when
/// `option arraystride on`; the apply body returns 0 immediately when the arch
/// flag is off, so default output stays byte-identical to upstream.
class RuleArrayStride : public Rule {
public:
  RuleArrayStride(const string &g) : Rule(g, 0, "arraystride") {}	///< Constructor
  virtual Rule *clone(const ActionGroupList &grouplist) const {
    if (!grouplist.contains(getGroup())) return (Rule *)0;
    return new RuleArrayStride(getGroup());
  }
  virtual void getOpList(vector<uint4> &oplist) const;
  virtual int4 applyOp(PcodeOp *op,Funcdata &data);
};

/// \brief (kuna) Set whether strided-induction array indices are recovered: `arraystride on|off`
class OptionArrayStride : public ArchOption {
public:
  OptionArrayStride(void) { name = "arraystride"; }	///< Constructor
  virtual string apply(Architecture *glb,const string &p1,const string &p2,const string &p3) const;
};

} // End namespace ghidra
#endif

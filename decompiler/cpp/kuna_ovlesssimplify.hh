/* ###
 * kuna-owned: overflow-flag signed-less-than simplification sub-stage (GH-7190).
 *
 * This file is NOT part of upstream Ghidra.  It addresses an S3
 * (simplification-quiescence) blind spot exposed by the V850 BLT (BranchLessThen)
 * idiom over a sign-extended byte counter (SXB; CMP; BLT):
 *
 *   The V850 SLEIGH set_OV_neg macro models the overflow flag with an explicit
 *   bit-extract / sub-register sign-shift instead of `A s< 0`.  The signed
 *   less-than that BLT computes therefore lifts into the canonical
 *   signed-overflow expansion
 *
 *       INT_SLESS(V+K, 0) != ( sign_bit(V) && INT_SLESS(-1, V+K) )
 *
 *   which is mathematically just `V s< -K`.  The default S3 pool (RuleRangeMeld
 *   et al.) cannot see through the constant offset (V + K) plus the sub-register
 *   sign test, so the convoluted boolean leaks into the rendered C
 *   (`iVar6 + -8 < 0 != (cVar5 >> 7 < '\0' && -1 < iVar6 + -8)`) instead of
 *   collapsing to `iVar6 < 8`.
 *
 * The decision is a P0 assertion: `option ovlesssimplify off|on` (default "off"
 * = upstream byte-identical output).  When "on", RuleOvLessSimplify recognizes
 * the idiom on CPUI_INT_NOTEQUAL and rewrites the whole cone to a single
 * INT_SLESS(V, -K).
 */
#ifndef __KUNA_OVLESSSIMPLIFY_HH__
#define __KUNA_OVLESSSIMPLIFY_HH__

#include "action.hh"
#include "options.hh"

namespace ghidra {

extern ElementId ELEM_OVLESSSIMPLIFY;	///< Marshaling element \<ovlesssimplify> (kuna)

/// \brief (kuna) Collapse the OV-flag signed-less-than idiom to a clean INT_SLESS
///
/// Matches  `INT_SLESS(V+K,0) != ( sign(V) && INT_SLESS(-1,V+K) )`  and rewrites
/// it to `INT_SLESS(V,-K)`.  The empty-range collapse is exact: the right operand
/// of the NOTEQUAL can only be true when V+K >= 0 AND V < 0, which forces
/// V+K < 0 to be false there, so the whole expression reduces to V+K < 0, i.e.
/// V s< -K.  Gated default-off behind the `ovlesssimplify` arch flag.
class RuleOvLessSimplify : public Rule {
public:
  RuleOvLessSimplify(const string &g) : Rule(g, 0, "ovlesssimplify") {}	///< Constructor
  virtual Rule *clone(const ActionGroupList &grouplist) const {
    if (!grouplist.contains(getGroup())) return (Rule *)0;
    return new RuleOvLessSimplify(getGroup());
  }
  virtual void getOpList(vector<uint4> &oplist) const;
  virtual int4 applyOp(PcodeOp *op,Funcdata &data);
};

/// \brief (kuna) Toggle the OV-flag signed-less-than simplification: `ovlesssimplify off|on`
class OptionOvLessSimplify : public ArchOption {
public:
  OptionOvLessSimplify(void) { name = "ovlesssimplify"; }	///< Constructor
  virtual string apply(Architecture *glb,const string &p1,const string &p2,const string &p3) const;
};

} // End namespace ghidra
#endif

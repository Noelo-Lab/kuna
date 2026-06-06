/* ###
 * kuna-owned: comparison-form presentation sub-stage.
 *
 * This file is NOT part of upstream Ghidra.  It implements the kuna stage-model
 * split of the comparison-canonicalization decision (GH-558, docs/prototypes/gh558.md):
 *
 *   - The decompiler canonicalizes `V <= c` into `V < c+1` through one primitive,
 *     Funcdata::replaceLessequal, reached from THREE sites: RuleIntLessEqual
 *     (Band-B rule pool, regrouped under "canonicalcompare" in coreaction.cc) and
 *     the two branch-flip helpers (opFlipInPlaceExecute / opNormalizeFlip), which
 *     fire as late as the post-structuring ActionPreferComplement /
 *     ActionStructureTransform passes.  The primitive records provenance on the
 *     op (PcodeOp::canonical_lessequal).
 *   - ActionPresentCompareForm runs AFTER the last flip site and BEFORE
 *     prototype/cast/naming fixation (the S8 -> S9 boundary), restoring the
 *     original LESSEQUAL form for marked ops.
 *
 * The decision is a P0 assertion: `option compareform canonical|original`
 * (default "canonical" = upstream behavior, byte-identical output).
 */
#ifndef __KUNA_COMPAREFORM_HH__
#define __KUNA_COMPAREFORM_HH__

#include "action.hh"
#include "options.hh"

namespace ghidra {

extern ElementId ELEM_COMPAREFORM;	///< Marshaling element \<compareform> (kuna)

/// \brief (kuna) Restore canonicalized comparisons to their original LESSEQUAL form for presentation
///
/// The exact inverse of Funcdata::replaceLessequal, applied only to ops carrying the
/// PcodeOp::canonical_lessequal provenance mark, and only when the `compareform original`
/// option is set:  `V < c+1  =>  V <= c`  and  `c-1 < V  =>  c <= V`.
class ActionPresentCompareForm : public Action {
public:
  ActionPresentCompareForm(const string &g) : Action(0,"presentcompareform",g) {}	///< Constructor
  virtual Action *clone(const ActionGroupList &grouplist) const {
    if (!grouplist.contains(getGroup())) return (Action *)0;
    return new ActionPresentCompareForm(getGroup());
  }
  virtual int4 apply(Funcdata &data);
};

/// \brief (kuna) Set how canonicalized comparisons are presented: `compareform canonical|original`
///
/// "canonical" (default) keeps the analysis-canonical `<` form in the output (upstream
/// behavior).  "original" restores the comparison operator and constant that appear in
/// the binary's original data-flow (ActionPresentCompareForm fires before rendering).
class OptionCompareForm : public ArchOption {
public:
  OptionCompareForm(void) { name = "compareform"; }	///< Constructor
  virtual string apply(Architecture *glb,const string &p1,const string &p2,const string &p3) const;
};

} // End namespace ghidra
#endif

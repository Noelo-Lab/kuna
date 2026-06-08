/* ###
 * kuna-owned: dynamic-hash collision budget sub-stage (GH-8467).
 *
 * This file is NOT part of upstream Ghidra.  It exposes a corner of the S6
 * "alias-facets" / dynamic-symbol attachment decision (stage-model.md):
 *
 *   When a HighVariable needs a storage-independent identity (a \e dynamic
 *   Symbol keyed on a hash of its local data-flow), DynamicHash::uniqueHash
 *   cycles through several hash methods and keeps the first whose collision
 *   list at the same address has size <= maxduplicates (hard-coded 8 upstream,
 *   dynamic.cc:433 and :494).  On dense, highly-repetitive code -- e.g. a
 *   fully unrolled AArch64 NEON byte-search loop (Go runtime bytealg, the
 *   function at 0x128d0 in the GH-8467 binary) -- more than 8 structurally
 *   identical Varnodes share the same address with colliding hashes, so EVERY
 *   method's collision list overflows 8, the champion list stays empty, and
 *   uniqueHash returns hash==0.  buildDynamicSymbol (funcdata_varnode.cc:1318)
 *   then throws RecovError("Unable to find unique hash for varnode") and the
 *   function aborts (GH-8467).
 *
 * The decision is a P0 assertion: `option dynamichashmax on|off` (default
 * "off" = upstream collision budget 8, byte-identical output).  When "on",
 * uniqueHash raises maxduplicates to 16, large enough to admit a unique hash
 * for these dense-but-not-pathological cases while still bounding the encoded
 * collision-list size (the 3-bit total/pos fields still apply -- 16 fits the
 * existing encoding range exactly).  No upstream test exercises >8 collisions,
 * so the wider budget is byte-identical on the whole regression suite.
 */
#ifndef __KUNA_DYNAMICHASHMAX_HH__
#define __KUNA_DYNAMICHASHMAX_HH__

#include "options.hh"

namespace ghidra {

extern ElementId ELEM_DYNAMICHASHMAX;	///< Marshaling element \<dynamichashmax> (kuna)

/// \brief (kuna) Raise the DynamicHash collision budget 8->16: `dynamichashmax on|off`
///
/// When on, sets glb->dynamic_hash_maxdup_high so DynamicHash::uniqueHash admits
/// up to 16 (instead of 8) same-address hash collisions before giving up.  Default
/// off preserves byte-identical upstream output.
class OptionDynamicHashMax : public ArchOption {
public:
  OptionDynamicHashMax(void) { name = "dynamichashmax"; }	///< Constructor
  virtual string apply(Architecture *glb,const string &p1,const string &p2,const string &p3) const;
};

} // End namespace ghidra
#endif

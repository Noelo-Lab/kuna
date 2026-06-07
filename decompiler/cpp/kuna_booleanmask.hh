/* ###
 * kuna-owned: boolean sign-extension-mask simplification sub-stage (GH-1282).
 *
 * This file is NOT part of upstream Ghidra.  It exposes a LATENT S3
 * "simplification-quiescence" decision (stage-model.md):
 *
 *   Some SLEIGH specs model a processor flag (e.g. the 8051 SUBB borrow / CY
 *   bit) as the high bit of a byte, lowering a boolean condition `i < 0x80`
 *   into the idiom `(b << k) s>> k` -- a known-1-bit boolean shifted up into
 *   the sign position and arithmetically shifted back, producing a 0/-1
 *   sign-extended mask.  RuleLeftRight already cancels `(V << c) s>> c`, but
 *   ONLY when c is a multiple of 8 (its generic SUBPIECE rewrite only realigns
 *   on byte boundaries).  For the boolean case k is 7, so the rule is gated
 *   off and the shift-by-7 carry noise survives all the way into the rendered
 *   C (GH-1282: `while (uVar1 = 0x80 - (((uVar2 < 0x80) << 7) >> 7), ...)`).
 *
 * The decision is a P0 assertion: `option booleanmask on|off` (default "off" =
 * upstream behavior, byte-identical output).  When on, RuleBoolSignShift
 * matches `(b << k) s>> k` where NZMask(b) occupies only the low
 * (width*8 - k) bits (i.e. no information lives above the bit that is shifted
 * into the sign position) and rewrites it to the sign-extended-mask form
 * `0 - b` = INT_2COMP(b) (0 or all-ones).  This is sound for ANY k in that
 * case because the boolean carries no bits that the shift could lose.  Once
 * the mask form exists, the existing constant-folding / comparison rules
 * collapse `0x80 < 0x80 - (0 - b)` down to `b != 0`, i.e. `i < 0x80`,
 * eliminating the shift entirely.
 */
#ifndef __KUNA_BOOLEANMASK_HH__
#define __KUNA_BOOLEANMASK_HH__

#include "action.hh"
#include "options.hh"

namespace ghidra {

extern ElementId ELEM_BOOLEANMASK;	///< Marshaling element \<booleanmask> (kuna)

/// \brief (kuna) Fold `(b << k) s>> k` into the sign-extended-mask form `0 - b`
///
/// Matches CPUI_INT_SRIGHT whose input is CPUI_INT_LEFT by the same constant k,
/// where the pre-shift value's NZMask fits in the low (width*8 - k) bits.  Only
/// fires when `option booleanmask on` (the apply body returns 0 immediately when
/// the arch flag is off, preserving byte-identical upstream output by default).
class RuleBoolSignShift : public Rule {
public:
  RuleBoolSignShift(const string &g) : Rule(g, 0, "boolsignshift") {}	///< Constructor
  virtual Rule *clone(const ActionGroupList &grouplist) const {
    if (!grouplist.contains(getGroup())) return (Rule *)0;
    return new RuleBoolSignShift(getGroup());
  }
  virtual void getOpList(vector<uint4> &oplist) const;
  virtual int4 applyOp(PcodeOp *op,Funcdata &data);
};

/// \brief (kuna) Set whether boolean sign-extension-mask shift idioms are folded: `booleanmask on|off`
class OptionBooleanMask : public ArchOption {
public:
  OptionBooleanMask(void) { name = "booleanmask"; }	///< Constructor
  virtual string apply(Architecture *glb,const string &p1,const string &p2,const string &p3) const;
};

} // End namespace ghidra
#endif

/* ###
 * kuna-owned: flag-modelled-comparison folding sub-stage (GH-1276 / GH-8777).
 *
 * This file is NOT part of upstream Ghidra.  It exposes a LATENT S3
 * "simplification-quiescence" decision (stage-model.md): some SLEIGH specs
 * model a processor's condition flags (sign N, signed-overflow V, carry/borrow)
 * explicitly as bits, and lower a clean signed comparison into raw flag algebra.
 * Two distinct flag idioms surface in user code and are folded here under ONE
 * option (`option flagcompare on|off`, default "off" = byte-identical upstream):
 *
 *   GH-1276 (8051, boolean-into-sign-bit):
 *     A flag/boolean `b` (NZMask == 1) is shifted into the byte sign position and
 *     sign-tested: `(b << k) s< 0`.  Because bit (size*8-1) of `(b<<k)` is exactly
 *     bit (size*8-1-k) of `b`, and for k == size*8-1 that is bit 0 of a 1-bit
 *     boolean, `(b << k) s< 0` is true iff `b != 0`.  The idiom renders as
 *     `while ((BANK0_R4 < uVar1) << 7 < '\0')`.  RuleBoolSignLess folds
 *     `INT_SLESS(INT_LEFT(b,k), 0)` to `b` (the boolean itself).
 *
 *   GH-8777 (PIC24, signed-overflow N==V):
 *     `bra ge` lowers to `N == V` over a subtraction V-K, i.e.
 *       XNOR( INT_SLESSEQUAL(0, V-K) , SBORROW(V,K) )
 *     spelled out as `BOOL_AND(BOOL_OR(N,!V), BOOL_OR(!N,V))`.  This is the
 *     textbook signed `V >= K` test.  It renders as the noisy
 *     `(bVar2 || !SBORROW2(iVar1,7)) && (!bVar2 || SBORROW2(iVar1,7))`.
 *     RuleSborrowGe folds the whole AND-of-ORs to `INT_SLESSEQUAL(K, V)`.
 *
 * Both rules return 0 immediately when the arch flag fold_flag_compare is off,
 * so default output is byte-identical to upstream.  A cousin of kuna_booleanmask
 * (the `(b<<k) s>> k` mask form) and kuna_ovlesssimplify (the OV-flag NE form).
 */
#ifndef __KUNA_FLAGCOMPARE_HH__
#define __KUNA_FLAGCOMPARE_HH__

#include "action.hh"
#include "options.hh"

namespace ghidra {

extern ElementId ELEM_FLAGCOMPARE;	///< Marshaling element \<flagcompare> (kuna)

/// \brief (kuna) GH-1276: fold `(b << k) s< 0` (boolean shifted into the sign bit) to `b`
///
/// Matches CPUI_INT_SLESS against constant 0 whose other input is
/// CPUI_INT_LEFT by a constant k, where the pre-shift value's NZMask fits in the
/// low (size*8 - k) bits so the only bit that can reach the sign position is the
/// boolean itself.  Rewrites to a plain copy of the boolean.  Only fires when
/// `option flagcompare on`.
class RuleBoolSignLess : public Rule {
public:
  RuleBoolSignLess(const string &g) : Rule(g, 0, "boolsignless") {}	///< Constructor
  virtual Rule *clone(const ActionGroupList &grouplist) const {
    if (!grouplist.contains(getGroup())) return (Rule *)0;
    return new RuleBoolSignLess(getGroup());
  }
  virtual void getOpList(vector<uint4> &oplist) const;
  virtual int4 applyOp(PcodeOp *op,Funcdata &data);
};

/// \brief (kuna) GH-8777: fold the `N == V` signed-overflow idiom to a clean signed compare
///
/// Matches CPUI_BOOL_AND of two CPUI_BOOL_OR limbs forming XNOR(N,V) where
/// N = INT_SLESSEQUAL(0, V-K) (the sign of the subtraction) and
/// V = SBORROW(V,K) (signed overflow).  Rewrites to INT_SLESSEQUAL(K, V)
/// (i.e. V s>= K).  Only fires when `option flagcompare on`.
class RuleSborrowGe : public Rule {
public:
  RuleSborrowGe(const string &g) : Rule(g, 0, "sborrowge") {}	///< Constructor
  virtual Rule *clone(const ActionGroupList &grouplist) const {
    if (!grouplist.contains(getGroup())) return (Rule *)0;
    return new RuleSborrowGe(getGroup());
  }
  virtual void getOpList(vector<uint4> &oplist) const;
  virtual int4 applyOp(PcodeOp *op,Funcdata &data);
};

/// \brief (kuna) Set whether flag-modelled comparison idioms are folded: `flagcompare on|off`
class OptionFlagCompare : public ArchOption {
public:
  OptionFlagCompare(void) { name = "flagcompare"; }	///< Constructor
  virtual string apply(Architecture *glb,const string &p1,const string &p2,const string &p3) const;
};

} // End namespace ghidra
#endif

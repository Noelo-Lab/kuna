/* ###
 * kuna-owned: carry-chain wide-add recovery sub-stage (GH-8913).
 *
 * This file is NOT part of upstream Ghidra.  It addresses an S5/S3 corner of
 * the simplification pipeline exposed by 8-bit carry-propagation idioms
 * (stage-model.md s5 / simplification-quiescence):
 *
 *   On the 6502 (and other narrow targets) a 16-bit pointer is built from a
 *   base table plus an (index << 1) using the classic two-instruction ADC
 *   idiom: ADC to the low byte, then ADC #0 to propagate the page-cross carry
 *   into the high byte.  The decompiler never fuses the pair into a single
 *   wide add, so the carry leaks as CARRY1(...) / a (0xNN < x) byte compare and
 *   the result is reassembled with CONCAT11 (GH-8913).
 *
 * The recovery is a new multi-op simplification rule, RuleAddCarryChain, that
 * matches the reassembling CPUI_PIECE and rewrites it to a single wide add
 *   out = INT_ADD( PIECE(hipart, b), zext(a) )
 * which downstream const-folding collapses to the readable base + zext(index).
 *
 * The decision is a P0 assertion: `option addcarrychain on|off` (default
 * "off" = upstream behavior, byte-identical output).  The rule body returns
 * immediately when the arch flag is off, so the upstream baseline is unchanged.
 */
#ifndef __KUNA_ADDCARRYCHAIN_HH__
#define __KUNA_ADDCARRYCHAIN_HH__

#include "options.hh"
#include "action.hh"

namespace ghidra {

extern ElementId ELEM_ADDCARRYCHAIN;	///< Marshaling element \<addcarrychain> (kuna)

/// \brief (kuna) Fuse an 8-bit carry-chained 16-bit add reassembled by CONCAT (GH-8913)
///
/// Matches `out = PIECE(hi, lo)` where
///   - `lo = INT_ADD(a, b)`                       (the low-byte sum)
///   - `hi = INT_ADD(hipart, carry)`              (high byte + page-cross carry)
///   - `carry` is the page-cross carry of the same a,b, in either form:
///       (i)  raw            carry = INT_CARRY(a, b);
///       (ii) const-folded   carry = INT_LESSEQUAL((-b)&mask, a)  with b constant
///            (RuleCarryElim's `carry(V,c) => -c <= V`).
/// and rewrites it to a single wide add `out = INT_ADD( PIECE(hipart, b), zext(a) )`.
///
/// Gated behind `option addcarrychain on` (Architecture::add_carry_chain); inert by default.
class RuleAddCarryChain : public Rule {
public:
  RuleAddCarryChain(const string &g) : Rule(g, 0, "addcarrychain") {}	///< Constructor
  virtual Rule *clone(const ActionGroupList &grouplist) const {
    if (!grouplist.contains(getGroup())) return (Rule *)0;
    return new RuleAddCarryChain(getGroup());
  }
  virtual void getOpList(vector<uint4> &oplist) const;
  virtual int4 applyOp(PcodeOp *op,Funcdata &data);
};

/// \brief (kuna) Set whether 8-bit carry-chain 16-bit adds are fused: `addcarrychain on|off`
class OptionAddCarryChain : public ArchOption {
public:
  OptionAddCarryChain(void) { name = "addcarrychain"; }	///< Constructor
  virtual string apply(Architecture *glb,const string &p1,const string &p2,const string &p3) const;
};

} // End namespace ghidra
#endif

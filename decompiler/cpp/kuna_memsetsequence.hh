/* ###
 * kuna-owned: constant-fill (memset/bzero) recovery sub-stage (GH-9230).
 *
 * This file is NOT part of upstream Ghidra.  It extends the S5 "constsequence"
 * named group (constseq.cc: RuleStringCopy/RuleStringStore) which collapses runs
 * of constant COPY/STORE ops into a single builtin_strncpy/memcpy CALLOTHER.
 *
 * The existing recovery only fires for runs that spell a printable STRING:
 * ArraySequence::formByteArray marks the first zero byte as a null terminator
 * (used==2) and the counting loop stops there, so an all-zero (bzero) run counts
 * as length 1 (< MINIMUM_SEQUENCE_LENGTH) and is rejected.  An unrolled
 * memset/bzero (clang's inlined `movaps xmm0,N(rsp)` run) therefore renders as
 * dozens of individual `buf[0xNN] = '\0';` element stores instead of a single
 * builtin_memset (GH-9230).
 *
 * The decision is a P0 assertion: `option memsetrecover on|off` (default "off" =
 * byte-identical upstream output).  When on, RuleMemsetCopy reuses the
 * StringSequence byte-region machinery to collect the contiguous constant COPY
 * run, and if every element holds the SAME fill value over a run of at least
 * MINIMUM_SEQUENCE_LENGTH elements, emits builtin_memset(dst,val,len) instead.
 * Single-fill detection keeps genuine strings routed to builtin_strncpy.
 */
#ifndef __KUNA_MEMSETSEQUENCE_HH__
#define __KUNA_MEMSETSEQUENCE_HH__

#include "constseq.hh"
#include "options.hh"

namespace ghidra {

extern ElementId ELEM_MEMSETRECOVER;	///< Marshaling element \<memsetrecover> (kuna)

/// \brief (kuna) A constant-fill run of COPY ops collapsed into builtin_memset
///
/// Derives from StringSequence to reuse its pointer-construction (constructTypedPointer)
/// and COPY teardown (removeCopyOps).  Collection is done by collectFillRun(), which
/// (unlike StringSequence) does not reject a leading zero and tolerates wide constant
/// COPYs (e.g. clang's 16-byte movaps).  formFillRun() then verifies the collected
/// COPYs tile a contiguous region with one fill byte; transform() emits builtin_memset.
class MemsetSequence : public StringSequence {
  uint1 fillValue;		///< The single repeated fill byte
  int4 fillCount;		///< Number of contiguous elements holding \b fillValue
  void collectFillRun(void);	///< Collect the contiguous constant COPY run (no leading-zero rejection)
  void formFillRun(void);	///< Detect a single-value constant fill across the collected ops
  PcodeOp *buildMemset(void);	///< Build the builtin_memset CALLOTHER
public:
  MemsetSequence(Funcdata &fdata,Datatype *ct,SymbolEntry *ent,PcodeOp *root,const Address &addr);
  bool isValidFill(void) const { return fillCount != 0; }	///< Return \b true if a fill run was found
  bool transform(void);		///< Transform the COPY run into a single builtin_memset user-op
};

/// \brief (kuna) Recognize a constant-fill run of COPY ops as builtin_memset (GH-9230)
///
/// Mirrors RuleStringCopy but, when `option memsetrecover on`, routes a run of
/// COPYs writing the SAME constant byte into a char array to builtin_memset.
class RuleMemsetCopy : public Rule {
public:
  RuleMemsetCopy(const string &g) : Rule( g, 0, "memsetcopy") {}	///< Constructor
  virtual Rule *clone(const ActionGroupList &grouplist) const {
    if (!grouplist.contains(getGroup())) return (Rule *)0;
    return new RuleMemsetCopy(getGroup());
  }
  virtual void getOpList(vector<uint4> &oplist) const;
  virtual int4 applyOp(PcodeOp *op,Funcdata &data);
};

/// \brief (kuna) Toggle constant-fill (memset/bzero) recovery: `memsetrecover on|off`
class OptionMemsetRecover : public ArchOption {
public:
  OptionMemsetRecover(void) { name = "memsetrecover"; }	///< Constructor
  virtual string apply(Architecture *glb,const string &p1,const string &p2,const string &p3) const;
};

} // End namespace ghidra
#endif

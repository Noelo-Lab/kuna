/* ###
 * kuna-owned: V850 "jmp [reg]" indirect-branch reclassification (GH-8817).
 *
 * This file is NOT part of upstream Ghidra.  It exposes a LATENT corner of
 * the S2 "flow-classification" sub-stage (call-vs-branch):
 *
 *   The V850 SLEIGH spec (Instructions/Special.sinc, "jmp [reg]" with
 *   reg != lp) lifts an indirect jump to `call [reg]`, i.e. CPUI_CALLIND.
 *   A compiler switch dispatch
 *       mov table,r11 ; shl 2,r10 ; add r11,r10 ; ld.w 0[r10],r10
 *       add r11,r10   ; jmp [r10]
 *   therefore lifts the dispatch op to a CALLIND.  But jump-table recovery
 *   (flow.cc / funcdata_block.cc / jumptable.cc) is gated EXCLUSIVELY on
 *   CPUI_BRANCHIND, so the switch is never recovered and the C renders a
 *   computed call to a function pointer:  (*(code *)(...))();   (GH-8817)
 *
 * The maintainer notes a blanket flip would break the call-style jalr;jmp
 * tail-call pattern of #8816, so the reclassification must be OPTIONAL.
 *
 * The decision is a P0 assertion: `option v850indirectbranch on|off`
 * (default "off" = upstream behavior, byte-identical output).  When on,
 * during flow setup (FlowInfo::xrefControlFlow, before the CALLIND is sent
 * down the call path) a V850 indirect-jmp CALLIND -- recognized by its
 * input being a processor register that is not the program counter, the
 * unique shape produced by `call [reg]` on V850 -- is reclassified to
 * CPUI_BRANCHIND so the S2 switch-model sub-stage runs.
 */
#ifndef __KUNA_V850INDBRANCH_HH__
#define __KUNA_V850INDBRANCH_HH__

#include "options.hh"
#include "funcdata.hh"

namespace ghidra {

extern ElementId ELEM_V850INDIRECTBRANCH;	///< Marshaling element \<v850indirectbranch> (kuna)

class PcodeOp;

/// \brief (kuna) Is \e op a V850 "jmp [reg]" CALLIND that should become a BRANCHIND? (GH-8817)
///
/// Checked (when `option v850indirectbranch on`) at flow-classification time.
/// Returns \b true only for an indirect CALL whose called value is a plain
/// processor register (not the program counter) -- the exact shape the V850
/// spec emits for `jmp [reg]` -- so an ordinary indirect call is unaffected.
bool kunaIsV850IndirectJmp(Funcdata &data,const PcodeOp *op);

/// \brief (kuna) Set whether V850 "jmp [reg]" is reclassified for switch recovery: `v850indirectbranch on|off`
class OptionV850IndirectBranch : public ArchOption {
public:
  OptionV850IndirectBranch(void) { name = "v850indirectbranch"; }	///< Constructor
  virtual string apply(Architecture *glb,const string &p1,const string &p2,const string &p3) const;
};

} // End namespace ghidra
#endif

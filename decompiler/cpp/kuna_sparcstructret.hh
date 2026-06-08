/* ###
 * kuna-owned: SPARC struct-return `unimp N` post-call fall-through (GH-6882).
 *
 * This file is NOT part of upstream Ghidra.  It exposes a LATENT corner of
 * the S2 "flow-classification" sub-stage (terminator / call-vs-branch):
 *
 *   The SPARC ABI for a function returning a structure has the callee return
 *   to caller+12 instead of caller+8, and the compiler plants an
 *   `unimp <structsize>` instruction in the gap (caller+8) as a struct-size
 *   hint.  The SPARC SLEIGH spec (SparcV9.sinc) lifts `unimp`/`illtrap` to
 *
 *       dest = IllegalInstructionTrap(N);
 *       goto [dest];          // "trap should not fall thru by default"
 *
 *   i.e. a CPUI_CALLOTHER (the IllegalInstructionTrap user p-code op) feeding
 *   a CPUI_BRANCHIND.  Jump-table recovery cannot resolve the indirect target,
 *   so flow.cc::truncateIndirectJump rewrites the BRANCHIND into a
 *   non-returning CALLIND ("Does not return") and the C renders a bogus
 *   indirect call `(*pcVar1)()`; because the trap is treated as a terminator,
 *   the rest of the function (everything past the `unimp`) is lost.  (GH-6882)
 *
 * The decision is a P0 assertion: `option sparcstructret on|off` (default
 * "off" = upstream behavior, byte-identical output).  When on, during flow
 * setup (FlowInfo::xrefControlFlow, before the BRANCHIND is queued for
 * jump-table recovery) this exact idiom -- a BRANCHIND whose target is the
 * output of an IllegalInstructionTrap CALLOTHER produced in the same
 * instruction -- is converted to a fall-through (the dead BRANCHIND is
 * removed), so the post-call `unimp` is skipped and the remainder of the
 * function is recovered.
 */
#ifndef __KUNA_SPARCSTRUCTRET_HH__
#define __KUNA_SPARCSTRUCTRET_HH__

#include "options.hh"
#include "funcdata.hh"

namespace ghidra {

extern ElementId ELEM_SPARCSTRUCTRET;	///< Marshaling element \<sparcstructret> (kuna)

class PcodeOp;

/// \brief (kuna) Is \e op a SPARC struct-return `unimp` BRANCHIND that should fall through? (GH-6882)
///
/// Checked (when `option sparcstructret on`) at flow-classification time.
/// Returns \b true only for a CPUI_BRANCHIND whose target Varnode is defined,
/// in the same instruction, by a CPUI_CALLOTHER bound to the
/// \c IllegalInstructionTrap user p-code op -- the exact shape the SPARC spec
/// emits for `unimp`/`illtrap` -- so an ordinary indirect branch / jump table
/// is unaffected.
bool kunaIsSparcStructRetTrap(Funcdata &data,const PcodeOp *op);

/// \brief (kuna) Set whether SPARC struct-return `unimp` falls through: `sparcstructret on|off`
class OptionSparcStructRet : public ArchOption {
public:
  OptionSparcStructRet(void) { name = "sparcstructret"; }	///< Constructor
  virtual string apply(Architecture *glb,const string &p1,const string &p2,const string &p3) const;
};

} // End namespace ghidra
#endif

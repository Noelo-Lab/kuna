/* ###
 * kuna-owned: Thumb function-pointer preservation sub-stage (GH-8471).
 *
 * This file is NOT part of upstream Ghidra.  It exposes a LATENT S5
 * "const-pointer" decision (stage-model.md s7.3):
 *
 *   A pointer to a Thumb function carries the mode bit in its LSB
 *   (value = fn|1).  ActionConstantPtr correctly recovers the symbolic form
 *   PTRSUB(fn) + 1, but RulePtrsubUndo then destroys it: the +1 does not
 *   match the bounds of fn's code type (isPtrsubMatching fails), so the
 *   constant collapses back to a raw hex literal in the output
 *   (GH-8471: `call_function((void *)0x10189)` instead of
 *   `call_function(&fn+1)`; upstream regression from the GP-4782
 *   RulePtrsubUndo refactor, cf. upstream PR #8990).
 *
 * The decision is a P0 assertion: `option thumbfuncptr on|off` (default
 * "off" = upstream behavior, byte-identical output).  When on,
 * RulePtrsubUndo preserves a PTRSUB whose base resolves to a code-typed
 * symbol when the leftover offset fits inside the architecture's
 * function-pointer alignment-encoding bits (funcptr_align - the Thumb LSB),
 * so the symbolic `&fn+1` form survives to rendering.
 */
#ifndef __KUNA_THUMBFUNCPTR_HH__
#define __KUNA_THUMBFUNCPTR_HH__

#include "options.hh"
#include "op.hh"
#include "funcdata.hh"

namespace ghidra {

extern ElementId ELEM_THUMBFUNCPTR;	///< Marshaling element \<thumbfuncptr> (kuna)

/// \brief (kuna) Should RulePtrsubUndo preserve this PTRSUB as a mode-bit-encoded function pointer?
///
/// Checked (when `option thumbfuncptr on`) just after isPtrsubMatching fails: returns
/// \b true when the base is the spacebase, the offset resolves exactly to a code-typed
/// symbol, and the extra offset fits within the funcptr_align encoding bits.
bool kunaPreserveThumbFuncPtr(Varnode *basevn,PcodeOp *op,int8 val,int8 extra,int8 multiplier,Architecture *glb);

/// \brief (kuna) Set whether mode-bit-encoded function pointers are preserved: `thumbfuncptr on|off`
class OptionThumbFuncPtr : public ArchOption {
public:
  OptionThumbFuncPtr(void) { name = "thumbfuncptr"; }	///< Constructor
  virtual string apply(Architecture *glb,const string &p1,const string &p2,const string &p3) const;
};

} // End namespace ghidra
#endif

/* ###
 * kuna-owned: function-entry constant-pointer inference sub-stage (GH-6930).
 *
 * This file is NOT part of upstream Ghidra.  It exposes a LATENT corner of
 * the S5 "const-pointer" decision (stage-model.md s7.3):
 *
 *   ActionConstantPtr::isPointer rejects any constant whose bit pattern
 *   "looks like a single bit or mask" (bit_transitions < 3) before it ever
 *   consults the symbol table -- a heuristic that protects integer flags
 *   from being mistaken for addresses.  But a function placed exactly at a
 *   power-of-two image base (e.g. 0x100000 == 1<<20) is a single-bit value,
 *   so its address is discarded and the function pointer renders as the bare
 *   literal 0x100000 instead of the function's name (GH-6930).
 *
 * The decision is a P0 assertion: `option inferfuncentry on|off` (default
 * "off" = upstream behavior, byte-identical output).  When on, the
 * single-bit rejection is skipped for a constant that resolves EXACTLY to a
 * known function entry -- narrowly scoped so legitimate single-bit integer
 * flags are unaffected.
 */
#ifndef __KUNA_INFERFUNCENTRY_HH__
#define __KUNA_INFERFUNCENTRY_HH__

#include "options.hh"
#include "funcdata.hh"

namespace ghidra {

/// \brief (kuna) Does \e val resolve exactly to a known function entry? (GH-6930)
///
/// Checked (when `option inferfuncentry on`) just before the bit_transitions
/// single-bit rejection in ActionConstantPtr::isPointer.  Returns \b true only
/// for an exact function-entry match, so the relaxation cannot reclassify an
/// ordinary single-bit integer constant.
bool kunaIsFunctionEntry(Funcdata &data,AddrSpace *spc,const Varnode *vn,const PcodeOp *op);

/// \brief (kuna) Set whether function entries at single-bit image bases are inferred: `inferfuncentry on|off`
class OptionInferFuncEntry : public ArchOption {
public:
  OptionInferFuncEntry(void) { name = "inferfuncentry"; }	///< Constructor
  virtual string apply(Architecture *glb,const string &p1,const string &p2,const string &p3) const;
};

} // End namespace ghidra
#endif

/* ###
 * kuna-owned: pointer-notation (array indexing) presentation sub-stage.
 *
 * This file is NOT part of upstream Ghidra.  It exposes the second GH-558
 * decision (docs/prototypes/gh558.md): how a standalone PTRADD is rendered.
 * Upstream PrintC::opPtradd uses array notation only inside a load/store
 * context and falls back to `base + index` otherwise — a LATENT S9
 * "literal/format policy" decision (docs/stage-model.md section 10).  kuna
 * parameterizes it: `option arraynotation on` renders `&base[index]` instead.
 */
#ifndef __KUNA_ARRAYNOTATION_HH__
#define __KUNA_ARRAYNOTATION_HH__

#include "options.hh"

namespace ghidra {

extern ElementId ELEM_ARRAYNOTATION;	///< Marshaling element \<arraynotation> (kuna)

/// \brief (kuna) Toggle array-notation rendering of standalone pointer arithmetic: `arraynotation on|off`
///
/// "off" (default) keeps upstream behavior (`base + index`); "on" renders
/// standalone PTRADD ops as `&base[index]` (PrintC::opPtradd).
class OptionArrayNotation : public ArchOption {
public:
  OptionArrayNotation(void) { name = "arraynotation"; }	///< Constructor
  virtual string apply(Architecture *glb,const string &p1,const string &p2,const string &p3) const;
};

} // End namespace ghidra
#endif

/* ###
 * kuna-owned: return-register pairing sub-stage (GH-6990).
 *
 * This file is NOT part of upstream Ghidra.  It exposes a corner of the S4
 * "trial finalization" decision (stage-model.md s7.2):
 *
 *   On SPARC (and any ABI with an output <join> rule over a register pair),
 *   a void function whose second output register (o1) passively carries a
 *   prologue value through the save/restore window to the RETURN op gets
 *   BOTH o0 and o1 marked as active return trials.  MultiSlotAssign then
 *   joins them and ActionReturnRecovery::buildReturnOutput emits
 *   `return CONCAT44(param_2,param_1)` with a spurious 8-byte return type
 *   (GH-6990), instead of recovering the single (or empty) return.
 *
 * The decision is a P0 assertion: `option returnpair pair|single` (default
 * "pair" = upstream join behavior, byte-identical output).  When "single",
 * buildReturnOutput refuses to join a multi-register return, keeping only
 * the first (least-significant) register.
 */
#ifndef __KUNA_RETURNPAIR_HH__
#define __KUNA_RETURNPAIR_HH__

#include "options.hh"

namespace ghidra {

extern ElementId ELEM_RETURNPAIR;	///< Marshaling element \<returnpair> (kuna)

/// \brief (kuna) Set how multi-register return values are joined: `returnpair pair|single`
class OptionReturnPair : public ArchOption {
public:
  OptionReturnPair(void) { name = "returnpair"; }	///< Constructor
  virtual string apply(Architecture *glb,const string &p1,const string &p2,const string &p3) const;
};

} // End namespace ghidra
#endif

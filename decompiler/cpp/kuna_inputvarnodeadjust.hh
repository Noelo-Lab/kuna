/* ###
 * kuna-owned: input-varnode reconciliation sub-stage (GH-9218).
 *
 * This file is NOT part of upstream Ghidra.  It exposes a LATENT S6
 * "stack-frame-layout / storage reconciliation" decision (stage-model.md):
 *
 *   ActionUnjustifiedParams (coreaction.cc) re-justifies an input Varnode that
 *   the prototype model places at the high end of a larger parameter container
 *   (e.g. on mc68k a `move.w (0x8,A6)` reads a 2-byte stack slot the model wants
 *   padded into a 4-byte int container).  Before calling Funcdata::adjustInputVarnodes
 *   it widens that container to absorb overlapping input Varnodes -- but ONLY
 *   ones that overlap and extend BELOW the container start.  When an input
 *   Varnode overlaps the container and extends ABOVE its end (a second, larger
 *   stack parameter sitting one byte past the word), the container is left too
 *   small and adjustInputVarnodes hits
 *     `vn->getOffset() + (vn->getSize()-1) > endaddr.getOffset()`
 *   and throws LowlevelError("Cannot properly adjust input varnodes")
 *   (funcdata_varnode.cc:508), aborting the whole function (GH-9218:
 *   link/unlk frame, "Cannot properly adjust input varnodes").
 *
 * The decision is a P0 assertion: `option inputvarnodeadjust on|off` (default
 * "off" = upstream behavior, byte-identical output, including the abort on the
 * pathological frame).  When "on", the overlap-widening loop is made symmetric:
 * a Varnode that overlaps the working container and extends above its end also
 * widens the container (its end is pushed out to the Varnode's last byte).  The
 * resulting single container encloses every intersecting input Varnode, so
 * adjustInputVarnodes can pull them all out as SUBPIECEs of one new input and
 * the function decompiles instead of aborting.
 */
#ifndef __KUNA_INPUTVARNODEADJUST_HH__
#define __KUNA_INPUTVARNODEADJUST_HH__

#include "options.hh"

namespace ghidra {

extern ElementId ELEM_INPUTVARNODEADJUST;	///< Marshaling element \<inputvarnodeadjust> (kuna)

/// \brief (kuna) Toggle symmetric input-Varnode container widening: `inputvarnodeadjust on|off`
///
/// When on, ActionUnjustifiedParams also absorbs input Varnodes that overlap the
/// justified container and extend ABOVE its end, so Funcdata::adjustInputVarnodes
/// no longer throws "Cannot properly adjust input varnodes" on overlapping stack
/// parameters (GH-9218).  Default off = byte-identical upstream behavior.
class OptionInputVarnodeAdjust : public ArchOption {
public:
  OptionInputVarnodeAdjust(void) { name = "inputvarnodeadjust"; }	///< Constructor
  virtual string apply(Architecture *glb,const string &p1,const string &p2,const string &p3) const;
};

} // End namespace ghidra
#endif

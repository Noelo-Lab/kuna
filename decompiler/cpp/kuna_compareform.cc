/* ###
 * kuna-owned: comparison-form presentation sub-stage (see kuna_compareform.hh).
 */
#include "kuna_compareform.hh"
#include "funcdata.hh"
#include "architecture.hh"

namespace ghidra {

// kuna ElementIds live in the 4000+ range to stay clear of upstream ids (max ~290).
ElementId ELEM_COMPAREFORM = ElementId("compareform",4000);

/// \brief Invert one canonicalized comparison:  `V < c+1  =>  V <= c`,  `c-1 < V  =>  c <= V`
///
/// Inverse of Funcdata::replaceLessequal, gated on the PcodeOp::canonical_lessequal
/// provenance mark.  The mark only asserts provenance; the op shape and overflow
/// conditions are re-validated from scratch, so an op reshaped by later transforms
/// dies gracefully (no rewrite).
/// \param op is the marked INT_LESS or INT_SLESS op
/// \param data is the function being decompiled
/// \return \b true if the original LESSEQUAL form was restored
static bool restoreLessequal(PcodeOp *op,Funcdata &data)

{
  Varnode *vn;
  int4 i;
  uintb val;
  intb diff;

  if ((vn=op->getIn(0))->isConstant()) {
    diff = 1;			// canonical `c-1 < V` came from `c <= V`
    i = 0;
  }
  else if ((vn=op->getIn(1))->isConstant()) {
    diff = -1;			// canonical `V < c+1` came from `V <= c`
    i = 1;
  }
  else
    return false;		// shape changed since canonicalization; leave as-is

  val = vn->getOffset();
  if (op->code() == CPUI_INT_SLESS) {
    // Mirror replaceLessequal's signed overflow guards in the inverse direction
    if ((diff == -1) && (val == calc_int_min(vn->getSize()))) return false;
    if ((diff ==  1) && (val == calc_int_max(vn->getSize()))) return false;
    data.opSetOpcode(op,CPUI_INT_SLESSEQUAL);
  }
  else {			// Unsigned overflow guards
    if ((diff == -1) && (val == 0)) return false;
    if ((diff ==  1) && (val == calc_uint_max(vn->getSize()))) return false;
    data.opSetOpcode(op,CPUI_INT_LESSEQUAL);
  }
  uintb res = (val + diff) & calc_mask(vn->getSize());
  Varnode *newvn = data.newConstant(vn->getSize(),res);
  newvn->copySymbol(vn);	// Preserve data-type (and any Symbol info), e.g. enum typing
  data.opSetInput(op,newvn,i);
  op->clearCanonicalLessequal();
  return true;
}

/// \class ActionPresentCompareForm
/// \brief (kuna) The comparison-presentation sub-stage: restore original comparison forms
///
/// Placed after the LAST canonicalization site (the post-structuring branch-flip
/// passes ActionPreferComplement / ActionStructureTransform) and before prototype,
/// naming, and cast fixation, so the restored form is what S9 renders.
int4 ActionPresentCompareForm::apply(Funcdata &data)

{
  if (!data.getArch()->present_lessequal) return 0;	// P0 assertion not set
  list<PcodeOp *>::const_iterator iter;
  for(iter=data.beginOpAlive();iter!=data.endOpAlive();++iter) {
    PcodeOp *op = *iter;
    OpCode opc = op->code();
    if ((opc != CPUI_INT_LESS)&&(opc != CPUI_INT_SLESS)) continue;
    if (!op->isCanonicalLessequal()) continue;
    if (restoreLessequal(op,data))
      count += 1;
  }
  return 0;
}

/// \class OptionCompareForm
/// \brief (kuna) Set how canonicalized comparisons are presented: `compareform canonical|original`
string OptionCompareForm::apply(Architecture *glb,const string &p1,const string &p2,const string &p3) const

{
  if (p1 == "original")
    glb->present_lessequal = true;
  else if (p1 == "canonical")
    glb->present_lessequal = false;
  else
    throw ParseError("Must specify compareform as 'canonical' or 'original'");
  return "Comparison presentation set to " + p1 + " form";
}

} // End namespace ghidra

/* ###
 * kuna-owned: carry-chain wide-add recovery (see kuna_addcarrychain.hh).
 */
#include "kuna_addcarrychain.hh"
#include "funcdata.hh"
#include "architecture.hh"

namespace ghidra {

// kuna ElementIds live in the 4000+ range (see kuna_returnpair.cc note).
ElementId ELEM_ADDCARRYCHAIN = ElementId("addcarrychain",4008);

void RuleAddCarryChain::getOpList(vector<uint4> &oplist) const

{
  oplist.push_back(CPUI_PIECE);
}

/// \brief Strip any CPUI_CAST/CPUI_COPY wrappers, returning the underlying value
static Varnode *kunaStripCast(Varnode *vn)

{
  while (vn->isWritten()) {
    PcodeOp *def = vn->getDef();
    OpCode oc = def->code();
    if (oc != CPUI_CAST && oc != CPUI_COPY) break;
    vn = def->getIn(0);
  }
  return vn;
}

/// \brief Are \e a and \e b the same value once CAST/COPY wrappers are stripped?
static bool kunaSameValue(Varnode *a,Varnode *b)

{
  a = kunaStripCast(a);
  b = kunaStripCast(b);
  if (a == b) return true;
  if (a->isConstant() && b->isConstant())
    return (a->getOffset() == b->getOffset() && a->getSize() == b->getSize());
  return false;
}

/// \brief Given the carry varnode and the low-add operands (a,b), confirm it is the
///        page-cross carry of a+b, in either INT_CARRY or RuleCarryElim form.
static bool kunaIsCarryOf(Varnode *carryvn,Varnode *a,Varnode *b)

{
  if (!carryvn->isWritten()) return false;
  PcodeOp *cop = carryvn->getDef();
  OpCode oc = cop->code();
  if (oc == CPUI_INT_CARRY) {
    Varnode *c0 = cop->getIn(0);
    Varnode *c1 = cop->getIn(1);
    if (kunaSameValue(c0,a) && kunaSameValue(c1,b)) return true;
    if (kunaSameValue(c0,b) && kunaSameValue(c1,a)) return true;
    return false;
  }
  if (oc == CPUI_INT_LESSEQUAL) {
    // RuleCarryElim form: carry(V,c) => (-c)&mask <= V, with c the constant added in the low sum.
    Varnode *cst = cop->getIn(0);
    Varnode *val = cop->getIn(1);
    if (!cst->isConstant()) return false;
    // The non-constant low-add operand must equal val; the constant low-add operand is c.
    Varnode *cAdd;	// the constant low-add operand
    Varnode *vAdd;	// the variable low-add operand
    if (a->isConstant() && !b->isConstant()) { cAdd = a; vAdd = b; }
    else if (b->isConstant() && !a->isConstant()) { cAdd = b; vAdd = a; }
    else return false;
    if (!kunaSameValue(val,vAdd)) return false;
    uintb negc = (-cAdd->getOffset()) & calc_mask(val->getSize());
    return (cst->getOffset() == negc);
  }
  return false;
}

int4 RuleAddCarryChain::applyOp(PcodeOp *op,Funcdata &data)

{
  if (!data.getArch()->add_carry_chain) return 0;	// (kuna) GH-8913: default-off parity gate

  Varnode *hivn = op->getIn(0);
  Varnode *lovn = op->getIn(1);
  if (!hivn->isWritten() || !lovn->isWritten()) return 0;
  if (hivn->getSize() != 1 || lovn->getSize() != 1) return 0;	// 1-byte halves only (first cut)

  PcodeOp *loadd = lovn->getDef();
  if (loadd->code() != CPUI_INT_ADD) return 0;
  Varnode *a = loadd->getIn(0);
  Varnode *b = loadd->getIn(1);

  PcodeOp *hiadd = hivn->getDef();
  if (hiadd->code() != CPUI_INT_ADD) return 0;

  // One input of the high add is the carry; the other is the base high byte (hipart).
  Varnode *hi0 = hiadd->getIn(0);
  Varnode *hi1 = hiadd->getIn(1);
  Varnode *carryvn;
  Varnode *hipart;
  if (kunaIsCarryOf(hi1,a,b)) { carryvn = hi1; hipart = hi0; }
  else if (kunaIsCarryOf(hi0,a,b)) { carryvn = hi0; hipart = hi1; }
  else return 0;
  (void)carryvn;

  if (hipart->getSize() != 1) return 0;

  // Pick the "base low byte" operand (b in the wide PIECE) and the "index" operand (a, zext'd).
  // For the const-folded form the base-low byte is the constant; otherwise either ordering is
  // a valid wide add, so default to (index=a, baselow=b).
  Varnode *indexvn = a;
  Varnode *baselo = b;

  int4 outsize = op->getOut()->getSize();	// width of the reassembled value (2 for CONCAT11)

  // Build  PIECE(hipart, baselo)  -- the 16-bit base.
  PcodeOp *baseop = data.newOp(2,op->getAddr());
  data.newUniqueOut(hipart->getSize() + baselo->getSize(),baseop);
  data.opSetOpcode(baseop,CPUI_PIECE);
  data.opSetInput(baseop,hipart,0);
  data.opSetInput(baseop,baselo,1);
  data.opInsertBefore(baseop,op);

  // Build  zext(index)  to the output width.
  PcodeOp *zextop = data.newOp(1,op->getAddr());
  data.newUniqueOut(outsize,zextop);
  data.opSetOpcode(zextop,CPUI_INT_ZEXT);
  data.opSetInput(zextop,indexvn,0);
  data.opInsertBefore(zextop,op);

  // Rewrite the PIECE op itself into the wide add:  out = base + zext(index).
  data.opSetOpcode(op,CPUI_INT_ADD);
  data.opSetInput(op,baseop->getOut(),0);
  data.opSetInput(op,zextop->getOut(),1);
  return 1;
}

/// \class OptionAddCarryChain
/// \brief (kuna) Toggle fusion of 8-bit carry-chain 16-bit adds (GH-8913)
string OptionAddCarryChain::apply(Architecture *glb,const string &p1,const string &p2,const string &p3) const

{
  bool val = onOrOff(p1);
  glb->add_carry_chain = val;
  string prop = val ? "on" : "off";
  return "Carry-chain wide-add recovery turned "+prop;
}

} // End namespace ghidra

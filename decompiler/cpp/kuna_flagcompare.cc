/* ###
 * kuna-owned: flag-modelled-comparison folding sub-stage (see kuna_flagcompare.hh).
 */
#include "kuna_flagcompare.hh"
#include "architecture.hh"
#include "funcdata.hh"

namespace ghidra {

// kuna ElementIds live in the 4000+ range (see kuna_returnpair.cc note).
ElementId ELEM_FLAGCOMPARE = ElementId("flagcompare",4010);

// ---------------------------------------------------------------------------
// GH-1276: (b << k) s< 0   ->   b
// ---------------------------------------------------------------------------

void RuleBoolSignLess::getOpList(vector<uint4> &oplist) const

{
  oplist.push_back(CPUI_INT_SLESS);
}

int4 RuleBoolSignLess::applyOp(PcodeOp *op,Funcdata &data)

{
  if (!data.getArch()->fold_flag_compare) return 0;	// (kuna) default-off: byte-identical to upstream

  // op = INT_SLESS(shiftout, 0)
  Varnode *zero = op->getIn(1);
  if (!zero->isConstant() || zero->getOffset() != 0) return 0;

  Varnode *shiftout = op->getIn(0);
  if (!shiftout->isWritten()) return 0;
  PcodeOp *leftshift = shiftout->getDef();
  if (leftshift->code() != CPUI_INT_LEFT) return 0;
  if (!leftshift->getIn(1)->isConstant()) return 0;
  uintb sa = leftshift->getIn(1)->getOffset();
  if (sa == 0) return 0;

  Varnode *boolvn = leftshift->getIn(0);
  int4 width = boolvn->getSize();
  // Only bit (width*8-1) of the shift output reaches the sign test; that bit is
  // bit (width*8-1-sa) of the pre-shift value.  Require the pre-shift value's
  // NZMask to carry no information above that bit, so `(b<<sa) s< 0` depends
  // solely on whether bit (width*8-1-sa) of b is set.  For a known-1-bit boolean
  // this is exactly `b != 0`.
  if ((uintb)width * 8 <= sa) return 0;
  int4 signbitpos = width * 8 - 1 - (int4)sa;	// bit of b landing in the sign position
  uintb relevant = (uintb)1 << signbitpos;
  // No bit of b at or above signbitpos other than the boolean bit itself.
  uintb above = ~(relevant - 1);			// mask of bits >= signbitpos
  if ((boolvn->getNZMask() & above) != relevant) return 0;	// exactly the one bit may be live there
  if (boolvn->getNZMask() != relevant) return 0;	// b is exactly that single boolean bit (NZMask)

  // Rewrite `(b << sa) s< 0`  =>  `b != 0`.  For a 1-bit boolean this is `b`,
  // but emitting INT_NOTEQUAL keeps the boolean type and lets later rules
  // collapse it where appropriate.
  data.opSetOpcode(op,CPUI_INT_NOTEQUAL);
  data.opSetInput(op,boolvn,0);
  data.opSetInput(op,data.newConstant(boolvn->getSize(),0),1);
  return 1;
}

// ---------------------------------------------------------------------------
// GH-8777: XNOR( N , V )  ->  SLESSEQUAL(K,V)   where  V s>= K
//
//   The PIC24 spec lowers `bra ge` to XNOR(N,V) over a subtraction V-K, with
//       N = sign of (V-K)  (INT_SLESSEQUAL(0, V-K) or INT_SLESS(V-K, 0))
//       V = INT_SBORROW(V,K)  (signed overflow)
//   The boolean simplifier may present XNOR(N,V) in either De-Morgan-dual form,
//   depending on when this rule runs:
//       AND-of-ORs:  BOOL_AND( BOOL_OR(N,!V) , BOOL_OR(!N,V) )   (stable/print)
//       OR-of-ANDs:  BOOL_OR ( BOOL_AND(N,V) , BOOL_AND(!N,!V) )  (spec form)
//   This rule recognizes both and rewrites the whole boolean tree to
//   INT_SLESSEQUAL(K, V) == `V s>= K`.  Runs in the analysis pool, where the
//   stable AND-of-ORs dataflow under the flags carries the clean SBORROW(V,K)
//   / (V-K) operands and dead boolean temps are DCE'd by the analysis loop.
// ---------------------------------------------------------------------------

/// \brief If \b vn is `BOOL_NEGATE(x)`, return x; otherwise return null.
static Varnode *stripBoolNeg(Varnode *vn)

{
  if (!vn->isWritten()) return (Varnode *)0;
  PcodeOp *op = vn->getDef();
  if (op->code() != CPUI_BOOL_NEGATE) return (Varnode *)0;
  return op->getIn(0);
}

/// \brief Recognize the `V s>= K` flag idiom rooted at two varnodes {N, V} where
/// one is INT_SBORROW(V,K) and the other is the sign of (V-K).  On success
/// fills \b vbase (V), \b kval (K), \b sz (width of V).  Order-independent.
static bool matchSborrowGe(Varnode *x,Varnode *y,Varnode **vbase,uintb *kval,int4 *sz)

{
  // Identify which is the SBORROW.
  Varnode *sb = (Varnode *)0;
  Varnode *sn = (Varnode *)0;
  for (int4 i=0;i<2;++i) {
    Varnode *cand = (i==0) ? x : y;
    if (!cand->isWritten()) continue;
    if (cand->getDef()->code() == CPUI_INT_SBORROW) {
      sb = cand;
      sn = (i==0) ? y : x;
      break;
    }
  }
  if (sb == (Varnode *)0 || sn == (Varnode *)0) return false;

  PcodeOp *sbop = sb->getDef();
  Varnode *base = sbop->getIn(0);		// V
  Varnode *vkonst = sbop->getIn(1);		// K
  if (!vkonst->isConstant()) return false;
  uintb kv = vkonst->getOffset();
  int4 width = base->getSize();

  // sn must be the sign-of-(V-K) flag (N), which the boolean simplifier presents
  // in one of these equivalent forms (all "(V-K) is non-negative", i.e. V s>= K):
  //    INT_SLESSEQUAL(0,   V-K)   ("0 s<= (V-K)")
  //    INT_SLESS    (-1,   V-K)   ("-1 s< (V-K)")     -- the cleanup-pool shape
  //    INT_SLESS    (V-K,  0)     ("(V-K) s< 0")  [the *negated* sign; handled
  //                                                  the same since XNOR(N,V)
  //                                                  with N negated still keys on
  //                                                  the same subtraction varnode]
  // In every form one operand is the subtraction `V + (-K)`; extract it.
  if (!sn->isWritten()) return false;
  PcodeOp *snop = sn->getDef();
  Varnode *snSub = (Varnode *)0;
  if (snop->code() == CPUI_INT_SLESSEQUAL) {
    Varnode *z = snop->getIn(0);
    if (!z->isConstant() || z->getOffset() != 0) return false;	// 0 s<= (V-K)
    snSub = snop->getIn(1);
  }
  else if (snop->code() == CPUI_INT_SLESS) {
    Varnode *a0 = snop->getIn(0);
    Varnode *a1 = snop->getIn(1);
    if (a0->isConstant() && a0->getOffset() == calc_mask(a0->getSize())) {
      snSub = a1;					// -1 s< (V-K)
    }
    else if (a1->isConstant() && a1->getOffset() == 0) {
      snSub = a0;					// (V-K) s< 0
    }
    else
      return false;
  }
  else
    return false;
  if (!snSub->isWritten()) return false;
  PcodeOp *subop = snSub->getDef();
  if (subop->code() != CPUI_INT_ADD) return false;	// V + (-K)
  Varnode *subBase = subop->getIn(0);
  Varnode *subK = subop->getIn(1);
  if (!subK->isConstant()) {			// allow commuted operand order
    Varnode *t = subBase; subBase = subK; subK = t;
    if (!subK->isConstant()) return false;
  }
  if (subBase != base) return false;
  if (subBase->getSize() != width) return false;
  // The add constant must be (-K) within width, i.e. addconst + K == 0 (mod 2^width*8).
  uintb negk = (subK->getOffset()) & calc_mask(width);
  if (((negk + kv) & calc_mask(width)) != 0) return false;

  *vbase = base;
  *kval = kv & calc_mask(width);
  *sz = width;
  return true;
}

/// \brief Extract the {N,V} flag varnode pair from an AND-of-ORs XNOR tree:
///   BOOL_AND( BOOL_OR(N,!V) , BOOL_OR(!N,V) ).  Each OR has exactly one plain
/// and one BOOL_NEGATE'd operand; the plain of one limb equals the negated of
/// the other and vice versa.  Fills \b a, \b b with {N,V}.
static bool extractAndOfOrs(PcodeOp *andop,Varnode **a,Varnode **b)

{
  Varnode *ia = andop->getIn(0);
  Varnode *ib = andop->getIn(1);
  if (!ia->isWritten() || !ib->isWritten()) return false;
  PcodeOp *orA = ia->getDef();
  PcodeOp *orB = ib->getDef();
  if (orA->code() != CPUI_BOOL_OR || orB->code() != CPUI_BOOL_OR) return false;
  // Split each OR limb into (plain, negated).
  Varnode *pA,*gA,*pB,*gB;
  {
    Varnode *na = stripBoolNeg(orA->getIn(0));
    Varnode *nb = stripBoolNeg(orA->getIn(1));
    if (na == (Varnode *)0 && nb != (Varnode *)0) { pA = orA->getIn(0); gA = nb; }
    else if (nb == (Varnode *)0 && na != (Varnode *)0) { pA = orA->getIn(1); gA = na; }
    else return false;
  }
  {
    Varnode *na = stripBoolNeg(orB->getIn(0));
    Varnode *nb = stripBoolNeg(orB->getIn(1));
    if (na == (Varnode *)0 && nb != (Varnode *)0) { pB = orB->getIn(0); gB = nb; }
    else if (nb == (Varnode *)0 && na != (Varnode *)0) { pB = orB->getIn(1); gB = na; }
    else return false;
  }
  // XNOR shape:  plain of A == negated of B,  plain of B == negated of A.
  if (pA != gB) return false;
  if (pB != gA) return false;
  *a = pA; *b = pB;
  return true;
}

/// \brief Extract the {N,V} flag varnode pair from an OR-of-ANDs XNOR tree:
///   BOOL_OR( BOOL_AND(N,V) , BOOL_AND(!N,!V) ).  One AND is the plain (N,V)
/// pair, the other is the negated (!N,!V) pair over the same two varnodes.
static bool extractOrOfAnds(PcodeOp *orop,Varnode **a,Varnode **b)

{
  Varnode *ia = orop->getIn(0);
  Varnode *ib = orop->getIn(1);
  if (!ia->isWritten() || !ib->isWritten()) return false;
  PcodeOp *andA = ia->getDef();
  PcodeOp *andB = ib->getDef();
  if (andA->code() != CPUI_BOOL_AND || andB->code() != CPUI_BOOL_AND) return false;
  // Identify the negated AND (both operands under BOOL_NEGATE) and the plain AND.
  PcodeOp *posAnd = (PcodeOp *)0;
  Varnode *n2 = (Varnode *)0, *v2 = (Varnode *)0;
  Varnode *na,*nb;
  na = stripBoolNeg(andB->getIn(0)); nb = stripBoolNeg(andB->getIn(1));
  if (na != (Varnode *)0 && nb != (Varnode *)0) { posAnd = andA; n2 = na; v2 = nb; }
  else {
    na = stripBoolNeg(andA->getIn(0)); nb = stripBoolNeg(andA->getIn(1));
    if (na != (Varnode *)0 && nb != (Varnode *)0) { posAnd = andB; n2 = na; v2 = nb; }
    else return false;
  }
  Varnode *p0 = posAnd->getIn(0);
  Varnode *p1 = posAnd->getIn(1);
  if (!((p0 == n2 && p1 == v2) || (p0 == v2 && p1 == n2))) return false;
  *a = p0; *b = p1;
  return true;
}

void RuleSborrowGe::getOpList(vector<uint4> &oplist) const

{
  oplist.push_back(CPUI_BOOL_AND);
  oplist.push_back(CPUI_BOOL_OR);
}

int4 RuleSborrowGe::applyOp(PcodeOp *op,Funcdata &data)

{
  if (!data.getArch()->fold_flag_compare) return 0;	// (kuna) default-off: byte-identical to upstream

  Varnode *nflag,*vflag;
  if (op->code() == CPUI_BOOL_AND) {
    if (!extractAndOfOrs(op,&nflag,&vflag)) return 0;
  }
  else { // CPUI_BOOL_OR
    if (!extractOrOfAnds(op,&nflag,&vflag)) return 0;
  }

  Varnode *vbase;
  uintb kval;
  int4 sz;
  if (!matchSborrowGe(nflag,vflag,&vbase,&kval,&sz)) return 0;

  // XNOR(N,V) with N = sign(V-K), V = SBORROW(V,K) is the signed `V >= K` test.
  // Rewrite the whole boolean tree to INT_SLESSEQUAL(K, V)  (K <= V == V s>= K).
  data.opSetOpcode(op,CPUI_INT_SLESSEQUAL);
  data.opSetInput(op,data.newConstant(sz,kval),0);
  data.opSetInput(op,vbase,1);
  return 1;
}

/// \class OptionFlagCompare
/// \brief (kuna) Toggle folding of flag-modelled comparison idioms
string OptionFlagCompare::apply(Architecture *glb,const string &p1,const string &p2,const string &p3) const

{
  bool val = onOrOff(p1);
  glb->fold_flag_compare = val;
  string prop = val ? "on" : "off";
  return "Flag-modelled comparison folding turned "+prop;
}

} // End namespace ghidra

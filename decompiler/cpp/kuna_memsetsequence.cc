/* ###
 * kuna-owned: constant-fill (memset/bzero) recovery sub-stage (see kuna_memsetsequence.hh).
 */
#include "kuna_memsetsequence.hh"
#include "funcdata.hh"
#include "architecture.hh"

namespace ghidra {

// kuna ElementIds live in the 4000+ range (see kuna_returnpair.cc note).
ElementId ELEM_MEMSETRECOVER = ElementId("memsetrecover",4009);

/// The base StringSequence constructor rejects any run whose root COPY value is 0
/// (it treats a leading zero as a degenerate/empty string), so it never collects an
/// all-zero (bzero) fill.  We therefore run our own collection here (collectFillRun)
/// which omits the leading-zero rejection AND tolerates wide constant COPYs, then
/// detect a single repeated fill value (formFillRun).  We still inherit
/// constructTypedPointer/removeCopyOps from StringSequence to build the destination
/// pointer and tear down the original COPYs.
/// \param fdata is the function containing the root COPY
/// \param ct is the element data-type of the array
/// \param ent is the Symbol being written
/// \param root is the COPY holding the first constant
/// \param addr is the Address being COPYed into
MemsetSequence::MemsetSequence(Funcdata &fdata,Datatype *ct,SymbolEntry *ent,PcodeOp *root,const Address &addr)
  : StringSequence(fdata,ct,ent,root,addr)
{
  fillValue = 0;
  fillCount = 0;
  numElements = 0;		// Clear any state the base ctor may have left
  moveOps.clear();
  collectFillRun();
  if (moveOps.empty())
    return;
  formFillRun();
}

/// Walk the Symbol's type to the array containing the root COPY, then collect every
/// CPUI_COPY in the same basic block whose input is a constant uniform-fill byte and
/// whose output lies within the array.  Unlike StringSequence::collectCopyOps, this
/// tolerates COPYs of ANY size (an unrolled memset/bzero typically leaves wide constant
/// COPYs, e.g. clang's 16-byte movaps, that are split into char elements only later in
/// the pipeline).  On success \b moveOps holds the COPY run (sorted by offset) and
/// \b startAddr/\b rootAddr/\b entry are set for constructTypedPointer.
void MemsetSequence::collectFillRun(void)

{
  if (entry->getAddr().getSpace() != rootAddr.getSpace())
    return;
  int8 off = rootAddr.getOffset() - entry->getFirst();
  if (off >= entry->getSize())
    return;
  Datatype *parentType = entry->getSymbol()->getType();
  Datatype *arrayType = (Datatype *)0;
  int8 lastOff = 0;
  do {
    if (parentType == charType)
      break;
    arrayType = parentType;
    lastOff = off;
    if (parentType->needsResolution()) {
      const TypeField *field = parentType->resolveTruncation(off, rootOp, -1, off);
      if (field == (const TypeField *)0) break;
      parentType = field->type;
    }
    else
      parentType = parentType->getSubType(off, &off);
  } while(parentType != (Datatype *)0);
  if (parentType != charType || arrayType == (Datatype *)0 || arrayType->getMetatype() != TYPE_ARRAY)
    return;
  startAddr = rootAddr - lastOff;			// Address of array element 0
  Address endAddr = startAddr + (arrayType->getSize() - 1);
  VarnodeLocSet::const_iterator iter = data.beginLoc(startAddr);
  VarnodeLocSet::const_iterator enditer = data.endLoc(endAddr);
  while(iter != enditer) {
    Varnode *vn = *iter;
    ++iter;
    if (!vn->isWritten()) continue;
    PcodeOp *op = vn->getDef();
    if (op->code() != CPUI_COPY) continue;
    if (op->getParent() != block) continue;
    Varnode *invn = op->getIn(0);
    if (!invn->isConstant()) continue;
    uint8 cval = invn->getOffset();
    bool uniform = true;				// Constant must be a single repeated byte across the COPY size
    for(int4 j=1;j<vn->getSize();++j) {
      if (((cval >> (j*8)) & 0xff) != (cval & 0xff)) { uniform = false; break; }
    }
    if (!uniform) continue;
    moveOps.emplace_back(vn->getOffset(),op,(int4)vn->getSize());	// store byte-size in slot field
  }
}

/// Examine the COPY ops collected by collectFillRun().  Sort them by offset, verify they
/// tile a contiguous byte region (no gaps, no overlap) starting at the lowest offset, and
/// all hold the same fill byte over a run of at least MINIMUM_SEQUENCE_LENGTH bytes.  On
/// success \b rootAddr is moved to the run start and \b fillCount holds the byte length.
void MemsetSequence::formFillRun(void)

{
  if (moveOps.size() < 1)
    return;
  sort(moveOps.begin(),moveOps.end(),
       [](const WriteNode &a,const WriteNode &b){ return a.offset < b.offset; });
  uint8 first = moveOps[0].op->getIn(0)->getOffset();
  uint1 fb = (uint1)(first & 0xff);
  uint8 runStart = moveOps[0].offset;
  uint8 expect = runStart;
  int4 lastIdx = -1;
  for(int4 i=0;i<moveOps.size();++i) {
    if (moveOps[i].offset != expect)
      break;					// Gap or overlap: end of contiguous tiling
    uint8 v = moveOps[i].op->getIn(0)->getOffset();
    if ((uint1)(v & 0xff) != fb)
      break;					// Different fill byte
    expect = moveOps[i].offset + moveOps[i].slot;	// slot holds the COPY byte-size
    lastIdx = i;
  }
  if (lastIdx < 1)
    return;					// A memset is a RUN: require at least 2 COPYs so a lone
						// store (e.g. a string's NUL terminator) is never claimed
  int4 totalBytes = (int4)(expect - runStart);
  if (totalBytes < 16)
    return;					// ...and a minimum fill footprint (16 = one SIMD store)
  if (lastIdx + 1 != (int4)moveOps.size())
    moveOps.resize(lastIdx + 1, WriteNode(0,(PcodeOp *)0,-1));	// Drop ops past the contiguous run
  rootAddr = startAddr + (int4)(runStart - startAddr.getOffset());	// Anchor pointer at run start
  fillValue = fb;
  fillCount = totalBytes;
  numElements = totalBytes;
}

/// A builtin_memset CALLOTHER is constructed.  Its first (destination) parameter is a
/// pointer to the array, built via the inherited constructTypedPointer().  The second
/// parameter is the constant fill byte, the third is the constant byte length.  The
/// op is inserted just before the earliest COPY in the run.
/// \return the constructed builtin_memset PcodeOp
PcodeOp *MemsetSequence::buildMemset(void)

{
  PcodeOp *insertPoint = moveOps[0].op;		// Earliest COPY in block order
  for(int4 i=1;i<moveOps.size();++i) {		// moveOps is sorted by offset, not block order
    if (moveOps[i].op->getSeqNum().getOrder() < insertPoint->getSeqNum().getOrder())
      insertPoint = moveOps[i].op;
  }
  Architecture *glb = data.getArch();
  uint4 builtInId = UserPcodeOp::BUILTIN_MEMSET;
  glb->userops.registerBuiltin(builtInId);
  PcodeOp *memsetOp = data.newOp(4,insertPoint->getAddr());
  data.opSetOpcode(memsetOp, CPUI_CALLOTHER);
  data.opSetInput(memsetOp, data.newConstant(4, builtInId), 0);
  Varnode *destPtr = constructTypedPointer(insertPoint);
  data.opSetInput(memsetOp, destPtr, 1);
  if (destPtr->getType()->needsResolution())
    data.inheritUnionFieldPtr(destPtr->getType(), memsetOp, 1, insertPoint, -1);
  Varnode *valVn = data.newConstant(4, fillValue);
  valVn->updateType(memsetOp->inputTypeLocal(2));
  data.opSetInput(memsetOp, valVn, 2);
  int4 numBytes = fillCount;			// fillCount is already a byte count
  Varnode *lenVn = data.newConstant(4, numBytes);
  lenVn->updateType(memsetOp->inputTypeLocal(3));
  data.opSetInput(memsetOp, lenVn, 3);
  data.opInsertBefore(memsetOp, insertPoint);
  return memsetOp;
}

/// Build the builtin_memset and remove the original COPY run (reusing the inherited
/// removeCopyOps, which rewrites descendants of the destroyed COPYs through INDIRECTs).
/// \return \b true if the transform succeeded
bool MemsetSequence::transform(void)

{
  PcodeOp *memsetOp = buildMemset();
  if (memsetOp == (PcodeOp *)0)
    return false;
  removeCopyOps(memsetOp);
  return true;
}

void RuleMemsetCopy::getOpList(vector<uint4> &oplist) const

{
  oplist.push_back(CPUI_COPY);
}

/// \class RuleMemsetCopy
/// \brief (kuna) Replace a run of COPY ops writing one constant value with builtin_memset
int4 RuleMemsetCopy::applyOp(PcodeOp *op,Funcdata &data)

{
  if (!data.getArch()->memset_recover) return 0;	// (kuna) gate: default-off, byte-identical upstream
  if (!op->getIn(0)->isConstant()) return 0;		// Constant
  Varnode *outvn = op->getOut();
  Datatype *ct = outvn->getTypeDefFacing();
  if (!ct->isCharPrint()) return 0;			// Copied to a "char" data-type Varnode
  if (ct->isOpaqueString()) return 0;
  if (!outvn->isAddrTied()) return 0;
  SymbolEntry *entry = data.getScopeLocal()->queryContainer(outvn->getAddr(), outvn->getSize(), op->getAddr());
  if (entry == (SymbolEntry *)0)
    return 0;
  MemsetSequence sequence(data,ct,entry,op,outvn->getAddr());
  if (!sequence.isValidFill())
    return 0;
  if (!sequence.transform())
    return 0;
  return 1;
}

/// \class OptionMemsetRecover
/// \brief (kuna) Toggle constant-fill recovery into builtin_memset
string OptionMemsetRecover::apply(Architecture *glb,const string &p1,const string &p2,const string &p3) const

{
  bool val = onOrOff(p1);
  glb->memset_recover = val;
  string prop = val ? "on" : "off";
  return "Constant-fill (memset) recovery turned "+prop;
}

} // End namespace ghidra

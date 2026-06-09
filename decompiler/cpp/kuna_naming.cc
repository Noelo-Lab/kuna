/* ###
 * kuna-owned: angr-style default naming policy (see kuna_naming.hh).
 */
#include "kuna_naming.hh"
#include "architecture.hh"
#include "database.hh"
#include "funcdata.hh"

namespace ghidra {

// kuna ElementIds live in the 4000+ range to stay clear of upstream ids (max ~290).
ElementId ELEM_NAMESTYLE = ElementId("namestyle",4020);

/// \class OptionNameStyle
/// \brief (kuna) Flip the master naming scheme between angr and Ghidra styles
string OptionNameStyle::apply(Architecture *glb,const string &p1,const string &p2,const string &p3) const

{
  bool val;
  if (p1 == "angr")
    val = true;
  else if (p1 == "ghidra")
    val = false;
  else
    return "namestyle must be \"angr\" or \"ghidra\"";
  glb->name_style_angr = val;
  return "Naming scheme set to " + p1;
}

bool kunaAngrNaming(const Architecture *glb)

{
  return (glb != (const Architecture *)0) && glb->name_style_angr;
}

/// Render an address as lowercase hex with no leading zeros and no "0x" prefix
/// (the angr address form used by dat_/sub_/label_).
/// \param s is the stream to write to
/// \param addr is the address to render
static void kunaPrintBareAddr(ostream &s,const Address &addr)

{
  uintb off = AddrSpace::byteToAddress(addr.getOffset(),addr.getSpace()->getWordSize());
  s << hex << off;
}

/// Lowercase a register name in place (angr renders registers lowercase).
static void kunaToLower(string &s)

{
  for(string::size_type i=0;i<s.size();++i)
    s[i] = tolower((unsigned char)s[i]);
}

string kunaGlobalDataName(const Address &addr)

{
  ostringstream s;
  s << "dat_";
  kunaPrintBareAddr(s,addr);
  return s.str();
}

string kunaFunctionName(const Address &addr)

{
  ostringstream s;
  s << "sub_";
  kunaPrintBareAddr(s,addr);
  return s.str();
}

string kunaLabelName(const Address &addr)

{
  ostringstream s;
  s << "label_";
  kunaPrintBareAddr(s,addr);
  return s.str();
}

string kunaArgName(int4 catindex)

{
  if (catindex < 0) catindex = 0;
  ostringstream s;
  s << 'a' << dec << catindex;
  return s.str();
}

bool kunaIsGeneratedName(const string &nm)

{
  // Recognize a generated default name in either scheme: param_N (ghidra) or vN / aN (angr).
  string::size_type start;
  if (nm.compare(0,6,"param_") == 0)
    start = 6;
  else if (nm.size() >= 2 && (nm[0] == 'v' || nm[0] == 'a') && isdigit((unsigned char)nm[1]))
    start = 1;
  else
    return false;
  for(string::size_type i=start;i<nm.size();++i)
    if (!isdigit((unsigned char)nm[i]))
      return false;
  return nm.size() > start;
}

bool kunaStorageComment(const Architecture *glb,const Symbol *sym,string &out)

{
  if (glb == (const Architecture *)0 || sym == (const Symbol *)0)
    return false;
  if (sym->numEntries() == 0)
    return false;
  SymbolEntry *entry = sym->getFirstWholeMap();
  Address addr = entry->getAddr();
  int4 sz = entry->getSize();
  if (addr.isInvalid()) {
    // Dynamic symbol (a merged/SSA variable with no fixed storage address):
    // resolve a representative Varnode through the function and use its location.
    Funcdata *fd = (sym->getScope() != (Scope *)0) ? sym->getScope()->getFuncdata() : (Funcdata *)0;
    if (fd != (Funcdata *)0) {
      Varnode *vn = fd->findLinkedVarnode(entry);
      if (vn != (Varnode *)0) {
	addr = vn->getAddr();
	sz = vn->getSize();
      }
    }
    if (addr.isInvalid()) {
      out = "tmp";		// no representative storage at all
      return true;
    }
  }
  AddrSpace *spc = addr.getSpace();
  // Register-backed storage -> the register name, lowercased ("rax").
  string reg = glb->translate->getRegisterName(spc,addr.getOffset(),sz);
  if (!reg.empty()) {
    kunaToLower(reg);
    out = reg;
    return true;
  }
  // Stack-backed storage -> a frame-relative signed offset ("stack - 0x10").
  // (The physical frame register isn't reliably known here, so we annotate the
  // abstract stack frame, which is the storage the core actually tracks.)
  if (spc == glb->getStackSpace()) {
    intb soff = (intb)AddrSpace::byteToAddress(addr.getOffset(),spc->getWordSize());
    soff = sign_extend(soff,addr.getAddrSize()*8-1);
    ostringstream s;
    if (soff < 0)
      s << "stack - 0x" << hex << (uintb)(-soff);
    else
      s << "stack + 0x" << hex << (uintb)soff;
    out = s.str();
    return true;
  }
  // Split value -> its constituent register pieces ("rdx:rax").
  if (spc->getType() == IPTR_JOIN) {
    JoinRecord *jr = glb->findJoin(addr.getOffset());
    if (jr != (JoinRecord *)0 && jr->numPieces() > 0) {
      ostringstream s;
      for(int4 i=0;i<jr->numPieces();++i) {
	const VarnodeData &pd(jr->getPiece(i));
	if (i != 0) s << ':';
	string pr = glb->translate->getRegisterName(pd.space,pd.offset,pd.size);
	if (!pr.empty()) {
	  kunaToLower(pr);
	  s << pr;
	}
	else
	  s << pd.space->getName() << '_' << hex << pd.offset;
      }
      out = s.str();
      return true;
    }
  }
  // A decompiler-internal temporary (unique space) with no machine register or
  // stack home: annotate it honestly so every local still carries a location.
  out = "tmp";
  return true;
}

} // End namespace ghidra

/* ###
 * kuna-owned: golden-vector generator commands for the Rust port
 * (see kuna_goldengen.hh).
 */
#include "kuna_goldengen.hh"
#include "opbehavior.hh"
#include "float.hh"
#include "sleigh_arch.hh"
#include "globalcontext.hh"
#include "grammar.hh"

namespace ghidra {

// Constructing this registers the capability
IfaceKunaGoldenCapability IfaceKunaGoldenCapability::ifaceKunaGoldenCapability;

IfaceKunaGoldenCapability::IfaceKunaGoldenCapability(void)

{
  name = "kuna_golden";
}

void IfaceKunaGoldenCapability::registerCommands(IfaceStatus *status)

{
  // Single word + internal dispatch: see kuna_goldengen.hh for why the kinds
  // cannot be separate second command words ("lift" vs "liftctx" prefix quirk)
  status->registerCom(new IfcKunaGolden(),"golden");
}

/// \class IfcKunaGolden
/// \brief Dispatcher for the `golden <kind>` vector generators
void IfcKunaGolden::execute(istream &s)

{
  string kind;
  s >> ws >> kind;
  if (kind == "opbehavior")
    executeOpbehavior();
  else if (kind == "float")
    executeFloat();
  else if (kind == "addrsort")
    executeAddrsort();
  else if (kind == "lift")
    executeLift(s);
  else if (kind == "liftctx")
    executeLiftctx(s);
  else
    throw IfaceParseError("Unknown golden vector kind: \""+kind+
			  "\" -- choose from: opbehavior float addrsort lift liftctx");
}

/// The sizes (in bytes) every vector matrix iterates over, in fixed order
static const int4 GOLDEN_SIZES[] = { 1, 2, 4, 8 };
static const int4 GOLDEN_NUM_SIZES = 4;

/// Render a value as a 0x-prefixed lower-case hex literal (no size padding)
static string goldenHex(uintb val)

{
  ostringstream ss;
  ss << "0x" << hex << val;
  return ss.str();
}

/// Render a host double with iostream default formatting (what printc relies on)
static string goldenDecimal(double val)

{
  ostringstream ss;
  ss << val;
  return ss.str();
}

/// Name an opcode for the vector rows.  Not upstream's get_opname: the
/// upstream opcode_name table is stale for the newest opcodes -- index 71
/// (CPUI_ZPULL) still reads "EXTRACT" and index 74 (CPUI_SPULL) is past the
/// end of the table (an out-of-bounds read that can crash).  Vectors use the
/// canonical enum names instead.
static const char *goldenOpName(OpCode opc)

{
  if (opc == CPUI_ZPULL)
    return "ZPULL";
  if (opc == CPUI_SPULL)
    return "SPULL";
  return get_opname(opc);
}

/// Fill \e vals with the fixed edge-input list for a given input size:
/// 0, 1, 2, signbit-1, signbit, all-ones, and two fixed odd constants masked
/// to the size.  Positional (index) order is part of the vector contract.
static void goldenInputs(int4 size,vector<uintb> &vals)

{
  uintb mask = calc_mask(size);
  vals.clear();
  vals.push_back(0);
  vals.push_back(1);
  vals.push_back(2);
  vals.push_back(mask >> 1);			// sign bit - 1 (max signed)
  vals.push_back((mask >> 1) + 1);		// sign bit (min signed)
  vals.push_back(mask);				// all ones for size
  vals.push_back(0xdeadbeefdeadbeefULL & mask);	// fixed odd constant 1
  vals.push_back(0x123456789abcdef1ULL & mask);	// fixed odd constant 2
}

/// Evaluate one unary cell, mapping any evaluation failure to "ERR".
/// EvaluationError derives from LowlevelError, and the base-class
/// "emulation unimplemented" throw is a plain LowlevelError, so catching
/// LowlevelError covers every deterministic failure mode.
static string goldenEvalUnary(const OpBehavior *beh,int4 sizeout,int4 sizein,uintb in0)

{
  try {
    return goldenHex(beh->evaluateUnary(sizeout,sizein,in0));
  } catch(const LowlevelError &err) {
    return "ERR";
  }
}

/// Evaluate one binary cell, mapping any evaluation failure to "ERR".
/// The INT64_MIN / -1 cell of INT_SDIV / INT_SREM is never evaluated: the
/// implementations divide in host 64-bit signed arithmetic, so that cell
/// raises SIGFPE (a hardware trap, not a C++ exception) on x86.  It emits
/// the literal "TRAP" instead.
static string goldenEvalBinary(const OpBehavior *beh,int4 sizeout,int4 sizein,uintb in0,uintb in1)

{
  OpCode opc = beh->getOpcode();
  if ((opc == CPUI_INT_SDIV || opc == CPUI_INT_SREM) && sizein == 8 &&
      in0 == 0x8000000000000000ULL && in1 == calc_mask(8))
    return "TRAP";
  if (opc == CPUI_SPULL)
    return "ERR";	// Unimplemented; the base-class throw itself would read past
			// the end of the stale upstream opcode_name table (index 74)
  try {
    return goldenHex(beh->evaluateBinary(sizeout,sizein,in0,in1));
  } catch(const LowlevelError &err) {
    return "ERR";
  }
}

/// \brief Emit OpBehavior evaluation vectors as CSV: `golden opbehavior`
///
/// Iterates the OpBehavior table built by OpBehavior::registerInstructions in
/// opcode enum order, skipping special (non-evaluatable) behaviors.  Unary
/// rows cover every (sizein,sizeout) pair so the extension/truncation ops are
/// exercised; binary rows cover sizein==sizeout plus the sizeout==1 forms the
/// comparison/carry ops produce in practice.  Requires a loaded architecture
/// because the CPUI_FLOAT_* behaviors hold the program's Translate to map an
/// operand size to its FloatFormat (sizes without a format fall back to the
/// base-class throw and emit ERR).
void IfcKunaGolden::executeOpbehavior(void)

{
  if (dcp->conf == (Architecture *)0)
    throw IfaceExecutionError("No load image present (the float op behaviors need the architecture's Translate)");
  ostream &os( *status->fileoptr );	// bulk stream: captured by openfile write
  vector<OpBehavior *> inst;
  OpBehavior::registerInstructions(inst,dcp->conf->translate);
  os << "# golden opbehavior vectors (kuna_goldengen.cc; see tests/golden/vectors/README.md)" << '\n';
  os << "# binary rows: op,sizein,sizeout,in0,in1,result   unary rows: op,sizein,sizeout,in0,result" << '\n';
  os << "# result ERR = evaluation threw (EvaluationError or unimplemented)" << '\n';
  vector<uintb> in0vals;
  vector<uintb> in1vals;
  for(int4 opc=0;opc<inst.size();++opc) {
    OpBehavior *beh = inst[opc];
    if (beh == (OpBehavior *)0) continue;
    if (beh->isSpecial()) continue;	// No direct unary/binary evaluation semantics
    const char *opname = goldenOpName((OpCode)opc);
    if (beh->isUnary()) {
      for(int4 i=0;i<GOLDEN_NUM_SIZES;++i) {
	int4 sizein = GOLDEN_SIZES[i];
	goldenInputs(sizein,in0vals);
	for(int4 j=0;j<GOLDEN_NUM_SIZES;++j) {
	  int4 sizeout = GOLDEN_SIZES[j];
	  for(int4 k=0;k<in0vals.size();++k)
	    os << opname << ',' << dec << sizein << ',' << sizeout << ','
	       << goldenHex(in0vals[k]) << ','
	       << goldenEvalUnary(beh,sizeout,sizein,in0vals[k]) << '\n';
	}
      }
    }
    else {
      for(int4 i=0;i<GOLDEN_NUM_SIZES;++i) {
	int4 sizein = GOLDEN_SIZES[i];
	goldenInputs(sizein,in0vals);
	in1vals = in0vals;
	for(int4 j=0;j<2;++j) {
	  int4 sizeout = (j==0) ? sizein : 1;
	  if (j == 1 && sizein == 1) continue;	// (1,1) already emitted
	  for(int4 k=0;k<in0vals.size();++k)
	    for(int4 l=0;l<in1vals.size();++l)
	      os << opname << ',' << dec << sizein << ',' << sizeout << ','
		 << goldenHex(in0vals[k]) << ',' << goldenHex(in1vals[l]) << ','
		 << goldenEvalBinary(beh,sizeout,sizein,in0vals[k],in1vals[l]) << '\n';
	}
      }
    }
  }
  for(int4 opc=0;opc<inst.size();++opc) {
    if (inst[opc] != (OpBehavior *)0)
      delete inst[opc];
  }
}

/// Map a FloatFormat::floatclass to its fixed vector label
static const char *goldenFloatClass(FloatFormat::floatclass cls)

{
  switch(cls) {
  case FloatFormat::normalized:
    return "normalized";
  case FloatFormat::infinity:
    return "infinity";
  case FloatFormat::zero:
    return "zero";
  case FloatFormat::nan:
    return "nan";
  case FloatFormat::denormalized:
    return "denormalized";
  }
  return "unknown";
}

/// Fill \e encs with the fixed list of interesting encodings for one format.
/// Ordinary values go through getEncoding from host doubles; the subnormals,
/// extremes, infinities, and NaN are raw IEEE 754 bit patterns so they do not
/// depend on the encoder.  Positional order is part of the vector contract.
static void goldenFloatEncodings(const FloatFormat &fmt,vector<uintb> &encs)

{
  encs.clear();
  encs.push_back(fmt.getEncoding(0.0));
  encs.push_back(fmt.getEncoding(-0.0));
  encs.push_back(fmt.getEncoding(1.0));
  encs.push_back(fmt.getEncoding(-1.0));
  encs.push_back(fmt.getEncoding(0.5));
  encs.push_back(fmt.getEncoding(3.14159265358979));
  encs.push_back(fmt.getEncoding(1e-10));
  encs.push_back(fmt.getEncoding(1e10));
  if (fmt.getSize() == 4) {
    encs.push_back(0x00000001);			// smallest subnormal
    encs.push_back(0x007fffff);			// largest subnormal
    encs.push_back(0x00800000);			// smallest normal
    encs.push_back(0x7f7fffff);			// largest finite
    encs.push_back(0x7f800000);			// +infinity
    encs.push_back(0xff800000);			// -infinity
    encs.push_back(0x7fc00000);			// quiet NaN
  }
  else {		// size 8
    encs.push_back(0x0000000000000001ULL);	// smallest subnormal
    encs.push_back(0x000fffffffffffffULL);	// largest subnormal
    encs.push_back(0x0010000000000000ULL);	// smallest normal
    encs.push_back(0x7fefffffffffffffULL);	// largest finite
    encs.push_back(0x7ff0000000000000ULL);	// +infinity
    encs.push_back(0xfff0000000000000ULL);	// -infinity
    encs.push_back(0x7ff8000000000000ULL);	// quiet NaN
  }
}

/// \brief Emit FloatFormat operation vectors as CSV: `golden float`
///
/// Constructs FloatFormat(4) and FloatFormat(8) directly (default IEEE 754
/// layouts), so no program needs to be loaded.  Per format: decode rows
/// (class + iostream default-precision decimal of the host double), unary op
/// rows, binary op rows over the full encoding cross product, opTrunc to each
/// integer size, and opInt2Float from each integer size; then opFloat2Float
/// 4->8 and 8->4.
void IfcKunaGolden::executeFloat(void)

{
  ostream &os( *status->fileoptr );	// bulk stream: captured by openfile write
  FloatFormat fmt4(4);
  FloatFormat fmt8(8);
  const FloatFormat *formats[2] = { &fmt4, &fmt8 };
  os << "# golden float vectors (kuna_goldengen.cc; see tests/golden/vectors/README.md)" << '\n';
  os << "# dec,size,enc,class,decimal | <unop>,size,a,res | <binop>,size,a,b,res" << '\n';
  os << "# trunc,size,sizeout,a,res | int2float,sizein,size,a,res | float2float,sizein,sizeout,a,res" << '\n';
  vector<uintb> encs;
  vector<uintb> ivals;
  for(int4 f=0;f<2;++f) {
    const FloatFormat &fmt( *formats[f] );
    int4 size = fmt.getSize();
    goldenFloatEncodings(fmt,encs);
    for(int4 i=0;i<encs.size();++i) {
      FloatFormat::floatclass cls;
      double host = fmt.getHostFloat(encs[i],&cls);
      os << "dec," << dec << size << ',' << goldenHex(encs[i]) << ','
	 << goldenFloatClass(cls) << ',' << goldenDecimal(host) << '\n';
    }
    for(int4 i=0;i<encs.size();++i)
      os << "neg," << dec << size << ',' << goldenHex(encs[i]) << ',' << goldenHex(fmt.opNeg(encs[i])) << '\n';
    for(int4 i=0;i<encs.size();++i)
      os << "abs," << dec << size << ',' << goldenHex(encs[i]) << ',' << goldenHex(fmt.opAbs(encs[i])) << '\n';
    for(int4 i=0;i<encs.size();++i)
      os << "sqrt," << dec << size << ',' << goldenHex(encs[i]) << ',' << goldenHex(fmt.opSqrt(encs[i])) << '\n';
    for(int4 i=0;i<encs.size();++i)
      os << "ceil," << dec << size << ',' << goldenHex(encs[i]) << ',' << goldenHex(fmt.opCeil(encs[i])) << '\n';
    for(int4 i=0;i<encs.size();++i)
      os << "floor," << dec << size << ',' << goldenHex(encs[i]) << ',' << goldenHex(fmt.opFloor(encs[i])) << '\n';
    for(int4 i=0;i<encs.size();++i)
      os << "round," << dec << size << ',' << goldenHex(encs[i]) << ',' << goldenHex(fmt.opRound(encs[i])) << '\n';
    for(int4 i=0;i<encs.size();++i)
      os << "nan," << dec << size << ',' << goldenHex(encs[i]) << ',' << goldenHex(fmt.opNan(encs[i])) << '\n';
    for(int4 i=0;i<encs.size();++i)
      for(int4 j=0;j<encs.size();++j)
	os << "add," << dec << size << ',' << goldenHex(encs[i]) << ',' << goldenHex(encs[j]) << ','
	   << goldenHex(fmt.opAdd(encs[i],encs[j])) << '\n';
    for(int4 i=0;i<encs.size();++i)
      for(int4 j=0;j<encs.size();++j)
	os << "sub," << dec << size << ',' << goldenHex(encs[i]) << ',' << goldenHex(encs[j]) << ','
	   << goldenHex(fmt.opSub(encs[i],encs[j])) << '\n';
    for(int4 i=0;i<encs.size();++i)
      for(int4 j=0;j<encs.size();++j)
	os << "mult," << dec << size << ',' << goldenHex(encs[i]) << ',' << goldenHex(encs[j]) << ','
	   << goldenHex(fmt.opMult(encs[i],encs[j])) << '\n';
    for(int4 i=0;i<encs.size();++i)
      for(int4 j=0;j<encs.size();++j)
	os << "div," << dec << size << ',' << goldenHex(encs[i]) << ',' << goldenHex(encs[j]) << ','
	   << goldenHex(fmt.opDiv(encs[i],encs[j])) << '\n';
    for(int4 i=0;i<encs.size();++i)
      for(int4 j=0;j<encs.size();++j)
	os << "equal," << dec << size << ',' << goldenHex(encs[i]) << ',' << goldenHex(encs[j]) << ','
	   << goldenHex(fmt.opEqual(encs[i],encs[j])) << '\n';
    for(int4 i=0;i<encs.size();++i)
      for(int4 j=0;j<encs.size();++j)
	os << "less," << dec << size << ',' << goldenHex(encs[i]) << ',' << goldenHex(encs[j]) << ','
	   << goldenHex(fmt.opLess(encs[i],encs[j])) << '\n';
    for(int4 i=0;i<encs.size();++i)
      for(int4 j=0;j<encs.size();++j)
	os << "lessequal," << dec << size << ',' << goldenHex(encs[i]) << ',' << goldenHex(encs[j]) << ','
	   << goldenHex(fmt.opLessEqual(encs[i],encs[j])) << '\n';
    for(int4 i=0;i<GOLDEN_NUM_SIZES;++i) {
      int4 sizeout = GOLDEN_SIZES[i];
      for(int4 j=0;j<encs.size();++j)
	os << "trunc," << dec << size << ',' << sizeout << ',' << goldenHex(encs[j]) << ','
	   << goldenHex(fmt.opTrunc(encs[j],sizeout)) << '\n';
    }
    for(int4 i=0;i<GOLDEN_NUM_SIZES;++i) {
      int4 sizein = GOLDEN_SIZES[i];
      goldenInputs(sizein,ivals);
      for(int4 j=0;j<ivals.size();++j)
	os << "int2float," << dec << sizein << ',' << size << ',' << goldenHex(ivals[j]) << ','
	   << goldenHex(fmt.opInt2Float(ivals[j],sizein)) << '\n';
    }
  }
  goldenFloatEncodings(fmt4,encs);
  for(int4 i=0;i<encs.size();++i)
    os << "float2float,4,8," << goldenHex(encs[i]) << ',' << goldenHex(fmt4.opFloat2Float(encs[i],fmt8)) << '\n';
  goldenFloatEncodings(fmt8,encs);
  for(int4 i=0;i<encs.size();++i)
    os << "float2float,8,4," << goldenHex(encs[i]) << ',' << goldenHex(fmt8.opFloat2Float(encs[i],fmt4)) << '\n';
}

/// \brief Emit Address comparator vectors as CSV: `golden addrsort`
///
/// Builds an Address for every (space, fixed offset) combination of the loaded
/// architecture, in space-index order, and emits the (<, ==, <=) decisions for
/// every ordered pair.  Offsets are used as given (no wrapping), matching how
/// the comparators themselves see them.
void IfcKunaGolden::executeAddrsort(void)

{
  if (dcp->conf == (Architecture *)0)
    throw IfaceExecutionError("No load image present");
  ostream &os( *status->fileoptr );	// bulk stream: captured by openfile write
  static const uintb offsets[] = { 0, 1, 0x1000, 0xffffffffULL };
  static const int4 numOffsets = 4;
  vector<Address> addrs;
  for(int4 i=0;i<dcp->conf->numSpaces();++i) {
    AddrSpace *spc = dcp->conf->getSpace(i);
    if (spc == (AddrSpace *)0) continue;
    for(int4 j=0;j<numOffsets;++j)
      addrs.push_back(Address(spc,offsets[j]));
  }
  os << "# golden addrsort vectors (kuna_goldengen.cc; see tests/golden/vectors/README.md)" << '\n';
  os << "# rows: spaceA,offA,spaceB,offB,less,equal,lessequal" << '\n';
  os << dec;			// the 0/1 decision columns are decimal
  for(int4 i=0;i<addrs.size();++i) {
    const Address &a( addrs[i] );
    for(int4 j=0;j<addrs.size();++j) {
      const Address &b( addrs[j] );
      os << a.getSpace()->getName() << ',' << goldenHex(a.getOffset()) << ','
	 << b.getSpace()->getName() << ',' << goldenHex(b.getOffset()) << ','
	 << ((a < b) ? 1 : 0) << ',' << ((a == b) ? 1 : 0) << ',' << ((a <= b) ? 1 : 0) << '\n';
    }
  }
}

/// Render an address as `<space>:<0x-offset>` for the lift dump lines
static string goldenAddr(const Address &addr)

{
  ostringstream ss;
  ss << addr.getSpace()->getName() << ':' << "0x" << hex << addr.getOffset();
  return ss.str();
}

/// Render one VarnodeData as `(space,offset,size)`.  Constants print their
/// raw offset, EXCEPT spaceid pointer constants (\e spaceidConst): the raw
/// offset of those is the heap address of the encoded AddrSpace -- the only
/// nondeterministic value in a raw lift -- so the space NAME is printed
/// instead, mirroring the Rust port's index-based representation
/// (docs/rust-port/losses.md LOSS-015): both engines render the name and
/// compare equal.  A space name can never be confused with a raw offset
/// (offsets always print 0x-prefixed).
static void goldenVarnode(ostream &os,const VarnodeData &vn,bool spaceidConst)

{
  os << '(' << vn.space->getName() << ',';
  if (spaceidConst && vn.space->getType() == IPTR_CONSTANT)
    os << vn.getSpaceFromConst()->getName();
  else
    os << goldenHex(vn.offset);
  os << ',' << dec << vn.size << ')';
}

/// \brief PcodeEmit subclass that renders raw lifted ops into a line buffer
///
/// One line per op: two-space indent, opcode name, output varnode (or `-`),
/// then each input varnode, all space separated.  Ops are buffered (not
/// streamed) so the caller can print the per-instruction header line -- whose
/// length field is only known after Translate::oneInstruction returns --
/// BEFORE the op lines.  The instruction address passed to dump() is dropped:
/// PcodeCacher::emit hands every op of an instruction (delay slots included)
/// the same base address, which the header line already carries.
class GoldenLiftEmit : public PcodeEmit {
  ostringstream buf;		///< Rendered op lines for the current instruction
public:
  virtual void dump(const Address &addr,OpCode opc,VarnodeData *outvar,VarnodeData *vars,int4 isize);
  string takeLines(void) { string res = buf.str(); buf.str(""); return res; }	///< Drain the buffer
};

void GoldenLiftEmit::dump(const Address &addr,OpCode opc,VarnodeData *outvar,VarnodeData *vars,int4 isize)

{
  buf << "  " << goldenOpName(opc) << ' ';
  if (outvar == (VarnodeData *)0)
    buf << '-';
  else
    goldenVarnode(buf,*outvar,false);
  for(int4 i=0;i<isize;++i) {
    buf << ' ';
    // Raw SLEIGH output encodes an address space operand (the *[space] of a
    // LOAD/STORE semantic) only as input 0 of CPUI_LOAD/CPUI_STORE
    bool spaceidConst = (i == 0) && (opc == CPUI_LOAD || opc == CPUI_STORE);
    goldenVarnode(buf,vars[i],spaceidConst);
  }
  buf << '\n';
}

/// \brief Emit per-instruction raw p-code lift fixtures: `golden lift <addr> <count>`
///
/// Decodes up to \e count instructions starting at \e addr (parse_machaddr
/// syntax: `0x...` for the default code space, `[space,0x...]` explicit, or a
/// symbol name).  Per instruction: a header `insn <space>:<offset> <length>`
/// then the buffered op lines from GoldenLiftEmit.  The length is
/// oneInstruction's fall-through offset (delay-slot bytes included) and is
/// also the step to the next decode address.  Any decode failure
/// (DataUnavailError / UnimplError / BadDataError / other LowlevelError)
/// emits `lifterror <class> <space>:<offset> <message>` and ends the dump --
/// the error text is part of the fixture (it is deterministic for a fixed
/// program and spec).
void IfcKunaGolden::executeLift(istream &s)

{
  if (dcp->conf == (Architecture *)0)
    throw IfaceExecutionError("No load image present");
  s >> ws;
  if (s.eof())
    throw IfaceParseError("Missing address: golden lift <addr> <count>");
  int4 defaultsize;
  Address addr = parse_machaddr(s,defaultsize,*dcp->conf->types);
  int4 count = -1;
  s >> ws >> dec >> count;
  if (count <= 0)
    throw IfaceParseError("Missing or non-positive instruction count: golden lift <addr> <count>");
  ostream &os( *status->fileoptr );	// bulk stream: captured by openfile write
  os << "# golden lift " << goldenAddr(addr) << ' ' << dec << count
     << " (kuna_goldengen.cc; see tests/golden/vectors/lift/README.md)" << '\n';
  const Translate *trans = dcp->conf->translate;
  GoldenLiftEmit emit;
  for(int4 i=0;i<count;++i) {
    int4 len;
    try {
      len = trans->oneInstruction(emit,addr);
    } catch(const DataUnavailError &err) {
      os << "lifterror dataunavail " << goldenAddr(addr) << ' ' << err.explain << '\n';
      break;
    } catch(const UnimplError &err) {
      os << "lifterror unimpl " << goldenAddr(addr) << ' ' << err.explain << '\n';
      break;
    } catch(const BadDataError &err) {
      os << "lifterror baddata " << goldenAddr(addr) << ' ' << err.explain << '\n';
      break;
    } catch(const LowlevelError &err) {
      os << "lifterror lowlevel " << goldenAddr(addr) << ' ' << err.explain << '\n';
      break;
    }
    os << "insn " << goldenAddr(addr) << ' ' << dec << len << '\n';
    os << emit.takeLines();
    addr = addr + len;
  }
}

// --- access to ContextInternal::variables (private) without an upstream edit.
// Access checking is suspended for the template arguments of an explicit
// instantiation (C++11 [temp.spec]p12: "The usual access checking rules do not
// apply to names used to specify explicit instantiations"), so the explicit
// instantiation below may name &ContextInternal::variables, and its friend
// definition hands the member pointer back through ADL on the tag type.  This
// is read-only introspection: `golden liftctx` needs to ENUMERATE the
// registered context variables, and the only upstream surfaces holding the
// name list (ContextInternal::variables, SleighBase::symtab) are non-public.
typedef map<string,ContextBitRange> ContextVarMap;
typedef ContextVarMap ContextInternal::*ContextVarsMemPtr;
struct KunaContextVarsTag {
  friend ContextVarsMemPtr kunaContextVarsPtr(KunaContextVarsTag);
};
template<ContextVarsMemPtr M>
struct KunaContextVarsRob {
  friend ContextVarsMemPtr kunaContextVarsPtr(KunaContextVarsTag) { return M; }
};
template struct KunaContextVarsRob<&ContextInternal::variables>;

/// \brief Emit the decoding context: `golden liftctx [<addr>]`
///
/// Key/value lines pinning everything the Rust SLEIGH runtime needs to
/// reproduce instruction decoding exactly:
///   archid      full normalized architecture id (language id + compiler)
///   languageid  the SLEIGH language id (archid minus the compiler field)
///   slafile     .sla file name from the matching LanguageDescription (.ldefs)
///   endian      `big` or `little` (Translate::isBigEndian)
///   contextsize number of 32-bit words in a context blob
///   space       one line per address space, in space-index order:
///               `space <name> index=<i> size=<addrsize> wordsize=<w>
///               bigendian=<0|1>` -- pins the space table, and gives
///               tools/rust-port/gen_lift.py the wordsize it needs to convert
///               the corpus XML's byte offsets into the word offsets the
///               console's address parser expects (AddrSpace::read multiplies
///               a typed offset by wordsize)
///   context     one `context <name>=<value>` line per registered context
///               variable, in name order (values in decimal).  Without an
///               address argument the program defaults are reported (NOTE: a
///               .pspec `<context_set>` paints values per address range, so
///               defaults are usually all zero); with an address, the values
///               in effect at that address (e.g. after a `set context` with a
///               range) -- what instruction decoding there actually uses.
void IfcKunaGolden::executeLiftctx(istream &s)

{
  if (dcp->conf == (Architecture *)0)
    throw IfaceExecutionError("No load image present");
  bool haveAddr = false;
  Address addr;
  s >> ws;
  if (!s.eof()) {
    int4 defaultsize;
    addr = parse_machaddr(s,defaultsize,*dcp->conf->types);
    haveAddr = true;
  }
  ostream &os( *status->fileoptr );	// bulk stream: captured by openfile write
  os << "# golden liftctx (kuna_goldengen.cc; see tests/golden/vectors/lift/README.md)" << '\n';
  const string &archid( dcp->conf->archid );
  os << "archid " << archid << '\n';
  string baseid = archid.substr(0,archid.rfind(':'));	// Strip compiler field (resolveArchitecture's rule)
  os << "languageid " << baseid << '\n';
  const vector<LanguageDescription> &descs( SleighArchitecture::getDescriptions() );
  for(int4 i=0;i<descs.size();++i) {
    if (descs[i].getId() == baseid) {
      os << "slafile " << descs[i].getSlaFile() << '\n';
      break;
    }
  }
  os << "endian " << (dcp->conf->translate->isBigEndian() ? "big" : "little") << '\n';
  ContextDatabase *ctxdb = dcp->conf->context;
  os << "contextsize " << dec << ctxdb->getContextSize() << '\n';
  for(int4 i=0;i<dcp->conf->numSpaces();++i) {
    AddrSpace *spc = dcp->conf->getSpace(i);
    if (spc == (AddrSpace *)0) continue;
    os << "space " << spc->getName() << " index=" << dec << spc->getIndex()
       << " size=" << spc->getAddrSize() << " wordsize=" << spc->getWordSize()
       << " bigendian=" << (spc->isBigEndian() ? 1 : 0) << '\n';
  }
  ContextInternal *cint = dynamic_cast<ContextInternal *>(ctxdb);
  if (cint == (ContextInternal *)0)
    throw IfaceExecutionError("Context database is not a ContextInternal -- cannot enumerate variables");
  const ContextVarMap &vars( cint->*kunaContextVarsPtr(KunaContextVarsTag()) );
  map<string,ContextBitRange>::const_iterator iter;
  for(iter=vars.begin();iter!=vars.end();++iter) {
    const string &nm( (*iter).first );
    uintm val = haveAddr ? ctxdb->getVariable(nm,addr) : ctxdb->getDefaultValue(nm);
    os << "context " << nm << '=' << dec << val << '\n';
  }
}

} // End namespace ghidra

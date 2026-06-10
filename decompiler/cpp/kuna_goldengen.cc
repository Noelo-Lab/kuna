/* ###
 * kuna-owned: golden-vector generator commands for the Rust port
 * (see kuna_goldengen.hh).
 */
#include "kuna_goldengen.hh"
#include "opbehavior.hh"
#include "float.hh"

namespace ghidra {

// Constructing this registers the capability
IfaceKunaGoldenCapability IfaceKunaGoldenCapability::ifaceKunaGoldenCapability;

IfaceKunaGoldenCapability::IfaceKunaGoldenCapability(void)

{
  name = "kuna_golden";
}

void IfaceKunaGoldenCapability::registerCommands(IfaceStatus *status)

{
  status->registerCom(new IfcKunaGoldenOpbehavior(),"golden","opbehavior");
  status->registerCom(new IfcKunaGoldenFloat(),"golden","float");
  status->registerCom(new IfcKunaGoldenAddrsort(),"golden","addrsort");
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

/// \class IfcKunaGoldenOpbehavior
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
void IfcKunaGoldenOpbehavior::execute(istream &s)

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

/// \class IfcKunaGoldenFloat
/// \brief Emit FloatFormat operation vectors as CSV: `golden float`
///
/// Constructs FloatFormat(4) and FloatFormat(8) directly (default IEEE 754
/// layouts), so no program needs to be loaded.  Per format: decode rows
/// (class + iostream default-precision decimal of the host double), unary op
/// rows, binary op rows over the full encoding cross product, opTrunc to each
/// integer size, and opInt2Float from each integer size; then opFloat2Float
/// 4->8 and 8->4.
void IfcKunaGoldenFloat::execute(istream &s)

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

/// \class IfcKunaGoldenAddrsort
/// \brief Emit Address comparator vectors as CSV: `golden addrsort`
///
/// Builds an Address for every (space, fixed offset) combination of the loaded
/// architecture, in space-index order, and emits the (<, ==, <=) decisions for
/// every ordered pair.  Offsets are used as given (no wrapping), matching how
/// the comparators themselves see them.
void IfcKunaGoldenAddrsort::execute(istream &s)

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

} // End namespace ghidra

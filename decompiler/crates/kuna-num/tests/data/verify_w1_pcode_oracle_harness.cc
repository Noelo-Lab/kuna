// Verifier oracle harness for w1-num-pcode-semantics: exercises the C++
// surfaces NOT covered by the golden opbehavior vectors.
#include "opbehavior.hh"
#include "opcodes.hh"
#include <iostream>
#include <cstdio>
using namespace ghidra;
using namespace std;

static void recb(const char *nm, const OpBehavior &b, int4 slot, int4 sizeout, uintb out, int4 sizein, uintb in) {
  cout << "recb," << nm << "," << dec << slot << "," << sizeout << ",0x" << hex << out << "," << dec << sizein << ",0x" << hex << in << ",";
  try { cout << "0x" << hex << b.recoverInputBinary(slot,sizeout,out,sizein,in) << "\n"; }
  catch(const EvaluationError &e) { cout << "EVALERR\n"; }
  catch(const LowlevelError &e) { cout << "LOWERR\n"; }
}
static void recu(const char *nm, const OpBehavior &b, int4 sizeout, uintb out, int4 sizein) {
  cout << "recu," << nm << "," << dec << sizeout << ",0x" << hex << out << "," << dec << sizein << ",";
  try { cout << "0x" << hex << b.recoverInputUnary(sizeout,out,sizein) << "\n"; }
  catch(const EvaluationError &e) { cout << "EVALERR\n"; }
  catch(const LowlevelError &e) { cout << "LOWERR\n"; }
}

int main() {
  setvbuf(stdout, NULL, _IONBF, 0);
  std::cout.setf(std::ios::unitbuf);
  OpBehaviorIntAdd add; OpBehaviorIntSub sub; OpBehaviorIntLeft left;
  OpBehaviorIntRight right; OpBehaviorIntSright sright;
  OpBehaviorIntZext zext; OpBehaviorIntSext sext; OpBehaviorInt2Comp tc;
  OpBehaviorIntNegate neg; OpBehaviorCopy copy;

  uintb outs[] = { 0, 1, 0x7f, 0x80, 0xff, 0x8000000000000000ULL, 0xffffffffffffffffULL, 0xdeadbeef, 0x123456789abcdef1ULL };
  int4 sizes[] = { 1, 2, 4, 8 };
  uintb shifts[] = { 0, 1, 7, 8, 31, 32, 63, 64, 0xffffffffffffffffULL };

  for (int so=0; so<4; ++so) for (int si=0; si<4; ++si) for (int o=0; o<9; ++o) {
    for (int s=0; s<9; ++s) {
      recb("INT_LEFT", left, 0, sizes[so], outs[o], sizes[si], shifts[s]);
      recb("INT_RIGHT", right, 0, sizes[so], outs[o], sizes[si], shifts[s]);
      recb("INT_SRIGHT", sright, 0, sizes[so], outs[o], sizes[si], shifts[s]);
    }
    recb("INT_ADD", add, 0, sizes[so], outs[o], sizes[si], 0xdeadbeefdeadbeefULL);
    recb("INT_ADD", add, 1, sizes[so], outs[o], sizes[si], 1);
    recb("INT_SUB", sub, 0, sizes[so], outs[o], sizes[si], 1);
    recb("INT_SUB", sub, 1, sizes[so], outs[o], sizes[si], 1);
    recu("INT_ZEXT", zext, sizes[so], outs[o], sizes[si]);
    recu("INT_SEXT", sext, sizes[so], outs[o], sizes[si]);
    recu("INT_2COMP", tc, sizes[so], outs[o], sizes[si]);
    recu("INT_NEGATE", neg, sizes[so], outs[o], sizes[si]);
    recu("COPY", copy, sizes[so], outs[o], sizes[si]);
  }
  // slot!=0 fallthrough
  recb("INT_LEFT", left, 1, 4, 8, 4, 1);
  recb("INT_RIGHT", right, 1, 4, 8, 4, 1);
  recb("INT_SRIGHT", sright, 1, 4, 8, 4, 1);

  // get_opcode lookups (including the UB-1 surface and (OpCode)45)
  const char *names[] = { "COPY", "INT_ADD", "SUBPIECE", "UNUSED1", "BLANK",
    "EXTRACT", "POPCOUNT", "LZCOUNT", "INT_AD", "INT_ADDX",
    "AAAA", "BUILD", "DELAY_SLOT", "LABEL", "CROSSBUILD", "" };
  for (int i=0; i<16; ++i)
    cout << "getop," << names[i] << "," << dec << (int)get_opcode(names[i]) << "\n";

  // get_booleanflip over the whole enum
  for (int i=1; i<CPUI_MAX; ++i) {
    if (i==45) continue;
    bool reorder = false;
    OpCode flip = get_booleanflip((OpCode)i, reorder);
    cout << "bflip," << dec << i << "," << (int)flip << "," << (reorder?1:0) << "\n";
  }
  return 0;
}

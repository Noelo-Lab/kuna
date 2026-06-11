/* ###
 * kuna-owned: golden-vector generator commands for the Rust port.
 *
 * This file is NOT part of upstream Ghidra.  Like kuna_console.hh it hooks the
 * upstream IfaceCapability discovery mechanism (CapabilityPoint::initializeAll
 * collects the static singleton below), so no upstream file is edited to wire
 * these commands in.
 *
 * The `golden <kind>` command family emits deterministic test vectors on the
 * bulk-output stream (status->fileoptr), so the standard `openfile write` /
 * `closefile` idiom captures them without prompt noise (the same mechanism
 * kuna/decompile.py uses for `print C`).  The vectors are consumed by
 * tools/rust-port/gen_vectors.py / gen_lift.py and committed under
 * tests/golden/vectors/ as the oracle for the Rust port's low-level semantics.
 *
 * Subcommands (all read-only; NO randomness anywhere):
 *   golden opbehavior  - OpBehavior::evaluateUnary/evaluateBinary results over
 *                        a fixed size x edge-input matrix for every non-special
 *                        opcode.  REQUIRES a loaded architecture: the float
 *                        behaviors (OpBehaviorFloatAdd etc.) need the program's
 *                        Translate to look up FloatFormat by size.
 *   golden float       - FloatFormat(4)/FloatFormat(8) encode/decode, the
 *                        opXxx operations, and default-precision iostream
 *                        decimal printing of the decoded host double (printc
 *                        relies on iostream formatting).  Needs NO program.
 *   golden addrsort    - Address comparator decisions (<, ==, <=) over a fixed
 *                        offset matrix crossed with every address space of the
 *                        loaded architecture.  REQUIRES a loaded architecture.
 *   golden lift <addr> <count>
 *                      - Per-instruction raw p-code lift starting at <addr>:
 *                        one header line per instruction (address, length) and
 *                        one line per emitted p-code op, captured through a
 *                        PcodeEmit subclass.  Spaceid pointer constants (the
 *                        LOAD/STORE input 0) print the SPACE NAME instead of
 *                        their raw offset -- the heap-pointer offset is the
 *                        only nondeterministic value in a raw lift (see
 *                        docs/rust-port/losses.md LOSS-015); normalizing it
 *                        to the name lets both engines compare byte-for-byte.
 *                        REQUIRES a loaded program (bytes to decode).
 *   golden liftctx [<addr>]
 *                      - Decoding-context dump: architecture id, language id,
 *                        .sla file, endianness, and EVERY context variable
 *                        name=value (default values, or the values in effect
 *                        at <addr> if one is given), so the Rust SLEIGH
 *                        runtime can reproduce the decode context exactly.
 *                        REQUIRES a loaded architecture.
 *
 * Registration quirk: the family is registered as the SINGLE command word
 * "golden" with the subcommand parsed inside execute().  Registering the
 * subcommands as separate second words would break `golden lift <args>`:
 * IfaceStatus::expandCom treats a word that is a strict prefix of a sibling
 * command word ("lift" / "liftctx") as still-expandable and consumes the
 * first argument as a command token ("ERROR: Invalid command") -- the same
 * pre-existing upstream quirk that makes `print cover varnode <name>`
 * unparseable next to "print cover varnodehigh".
 */
#ifndef __KUNA_GOLDENGEN_HH__
#define __KUNA_GOLDENGEN_HH__

#include "ifacedecomp.hh"

namespace ghidra {

/// \brief (kuna) Interface capability point for the golden-vector commands
class IfaceKunaGoldenCapability : public IfaceCapability {
  static IfaceKunaGoldenCapability ifaceKunaGoldenCapability;	///< Singleton instance
  IfaceKunaGoldenCapability(void);				///< Singleton constructor
  IfaceKunaGoldenCapability(const IfaceKunaGoldenCapability &op2);	///< Not implemented
  IfaceKunaGoldenCapability &operator=(const IfaceKunaGoldenCapability &op2);	///< Not implemented
public:
  virtual void registerCommands(IfaceStatus *status);
};

/// \brief (kuna) `golden <kind>`: emit golden vectors for the Rust port
///
/// Single dispatcher for every vector kind (see the file comment for why the
/// kinds are not registered as separate command words):
///   - `golden opbehavior`: OpBehavior evaluation vectors as CSV.  One row per
///     (opcode, sizein, sizeout, inputs) cell, in opcode enum order, then
///     size-combination order, then input index.  Binary rows have six fields
///     (op,sizein,sizeout,in0,in1,result), unary rows five.  A cell whose
///     evaluation throws (EvaluationError or the base-class "unimplemented"
///     LowlevelError) emits the literal result ERR.  The INT64_MIN / -1 cell
///     of INT_SDIV / INT_SREM emits TRAP without evaluating (it would SIGFPE
///     in the host's signed division).
///   - `golden float`: FloatFormat operation vectors as CSV.  Exercises
///     FloatFormat(4) and FloatFormat(8) over a fixed list of interesting
///     encodings (zeros, +-1, 0.5, subnormals, extremes, infinities, NaN, pi,
///     1e-10, 1e10): decode class + iostream decimal printing, the unary and
///     binary opXxx operations, opTrunc to each integer size, opInt2Float from
///     each integer size, and opFloat2Float in both directions.
///   - `golden addrsort`: Address comparator vectors as CSV.  Builds
///     Address(space,offset) for every space of the loaded architecture
///     crossed with a fixed offset list, and emits (<, ==, <=) for every
///     ordered pair.  The space set (and hence the vector set) is a property
///     of the loaded architecture, so regeneration must pin the same program.
///   - `golden lift <addr> <count>`: per-instruction p-code lift fixtures.
///     For each of <count> instructions starting at <addr>: a header line
///     `insn <space>:<offset> <length>` followed by one `  <OPNAME> <out>
///     <in0> <in1> ...` line per emitted op, varnodes rendered as
///     (space,offset,size) and a missing output as `-`.  <length> is
///     Translate::oneInstruction's fall-through offset, i.e. it includes
///     delay-slot bytes (SPARC/MIPS), and the next instruction is decoded at
///     <addr>+<length>.  A decode failure emits a terminal line
///     `lifterror <class> <space>:<offset> <message>` and stops the dump.
///   - `golden liftctx [<addr>]`: one `<key> <value>` line each for archid,
///     languageid, slafile, endian and contextsize, one `space` line per
///     address space (name, index, addrsize, wordsize, endianness), then one
///     `context <name>=<value>` line per context variable in name order
///     (values in effect at <addr>, or the all-defaults blob without one).
class IfcKunaGolden : public IfaceDecompCommand {
  void executeOpbehavior(void);		///< `golden opbehavior`
  void executeFloat(void);		///< `golden float`
  void executeAddrsort(void);		///< `golden addrsort`
  void executeLift(istream &s);		///< `golden lift <addr> <count>`
  void executeLiftctx(istream &s);	///< `golden liftctx [<addr>]`
public:
  virtual void execute(istream &s);
};

} // End namespace ghidra
#endif

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
 * tools/rust-port/gen_vectors.py and committed under tests/golden/vectors/ as
 * the oracle for the Rust port's low-level semantics.
 *
 * Commands (all read-only; NO randomness anywhere):
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

/// \brief (kuna) `golden opbehavior`: emit OpBehavior evaluation vectors as CSV
///
/// One row per (opcode, sizein, sizeout, inputs) cell, in opcode enum order,
/// then size-combination order, then input index.  Binary rows have six
/// fields (op,sizein,sizeout,in0,in1,result), unary rows five.  A cell whose
/// evaluation throws (EvaluationError or the base-class "unimplemented"
/// LowlevelError) emits the literal result ERR.  The INT64_MIN / -1 cell of
/// INT_SDIV / INT_SREM emits TRAP without evaluating (it would SIGFPE in the
/// host's signed division).
class IfcKunaGoldenOpbehavior : public IfaceDecompCommand {
public:
  virtual void execute(istream &s);
};

/// \brief (kuna) `golden float`: emit FloatFormat operation vectors as CSV
///
/// Exercises FloatFormat(4) and FloatFormat(8) over a fixed list of
/// interesting encodings (zeros, +-1, 0.5, subnormals, extremes, infinities,
/// NaN, pi, 1e-10, 1e10): decode class + iostream decimal printing, the unary
/// and binary opXxx operations, opTrunc to each integer size, opInt2Float
/// from each integer size, and opFloat2Float in both directions.
class IfcKunaGoldenFloat : public IfaceDecompCommand {
public:
  virtual void execute(istream &s);
};

/// \brief (kuna) `golden addrsort`: emit Address comparator vectors as CSV
///
/// Builds Address(space,offset) for every space of the loaded architecture
/// crossed with a fixed offset list, and emits (<, ==, <=) for every ordered
/// pair.  The space set (and hence the vector set) is a property of the
/// loaded architecture, so regeneration must pin the same program.
class IfcKunaGoldenAddrsort : public IfaceDecompCommand {
public:
  virtual void execute(istream &s);
};

} // End namespace ghidra
#endif

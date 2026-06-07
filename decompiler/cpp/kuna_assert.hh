/* ###
 * kuna-owned: the typed assertion API (stage-model.md section 12).
 *
 * This file is NOT part of upstream Ghidra.  It implements the uniform P0
 * assertion writer:
 *
 *     kassert <stage> <substage> <args...> [hard|hint]
 *     kassert list
 *
 * kassert is a thin facade: it validates <stage>/<substage> against the
 * stage registry (kuna_stages) and dispatches to the EXISTING battle-tested
 * stores - Override (force-goto, deadcode-delay, multistage jump, flow
 * override), Symbol DB locks (typelock, namelock, isolate), FuncProto locks,
 * and the option database.  It builds no new analysis and introduces no new
 * override types: a kassert-applied assertion has exactly the effect of the
 * legacy command it routes to.
 *
 * What it ADDS is the model: every assertion is recorded with its stage,
 * sub-stage, strength, and the COMPUTED MINIMAL REWIND SCOPE from
 * stage-model.md section 12 (format->S9, force-goto->S7, typelock->S5,
 * proto->S4, jumptable/flow/context->S2, deadcode-delay->S3).  The scope is
 * REPORTED by `kassert list`, not yet enacted - Ghidra still rounds every
 * re-run up to the whole function (clearAnalysis); scoped re-run is the
 * kuna roadmap's item 2 (stage-model.md section 16) and is intentionally
 * out of scope here.  This is the observable half.
 *
 * The record is a console-session log (it survives decompile/clear like the
 * Override store does, but is not marshaled with the program).
 */
#ifndef __KUNA_ASSERT_HH__
#define __KUNA_ASSERT_HH__

#include "ifacedecomp.hh"
#include "kuna_stages.hh"

namespace ghidra {

/// \brief (kuna) One recorded typed assertion (session log entry)
struct KunaAssertion {
  std::string funcName;		///< Function it applies to, or "(global)"
  KunaStage stage;		///< Stage asserted
  std::string substage;		///< Sub-stage name (validated against the catalog)
  std::string args;		///< The assertion arguments as given
  KunaStrength strength;	///< Requested strength (hard/hint)
  KunaStrength applied;		///< Strength of the underlying mechanism actually used
  KunaStage rewind;		///< Computed minimal rewind stage (REPORTED, not enacted)
};

/// \brief (kuna) `kassert ...`: write a typed assertion through an existing store
///
/// The `list` sub-form (`kassert list`) is handled internally rather than as
/// a separately registered command: the console matcher cannot disambiguate a
/// bare command taking free-form arguments from a registered subcommand with
/// the same first word.
class IfcKunaAssert : public IfaceDecompCommand {
  void applyOption(const std::string &optname,const std::string &p1,
		   const std::string &p2,const std::string &p3);	///< Route to OptionDatabase::set
  void listAssertions(void);		///< Dump recorded assertions with reported rewind scopes
public:
  virtual void execute(istream &s);
};

} // End namespace ghidra
#endif

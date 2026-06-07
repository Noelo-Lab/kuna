/* ###
 * kuna-owned: the kuna console capability and stage-registry commands.
 *
 * This file is NOT part of upstream Ghidra.  It registers all kuna console
 * commands through the upstream IfaceCapability discovery mechanism: the
 * static singleton below is collected by CapabilityPoint::initializeAll()
 * (called from startDecompilerLibrary) and its registerCommands() is invoked
 * by IfaceCapability::registerAllCommands - both from the interactive
 * console (consolemain.cc) and from the datatest harness's ConsoleCommands
 * (testfunction.cc).  No upstream file is edited to wire this in.
 *
 * Commands defined here (all read-only over the stage registry):
 *   stage list           - the stages, Band B membership, and the sub-stage
 *                          decision catalog (with LATENT markers)
 *   stage map [<name>]   - where an action/rule group, console surface, or
 *                          sub-stage lives in the model
 *   stage status         - active pipeline variant + kuna sub-stage options
 *                          for the loaded program (+ goto-quality metric)
 *
 * Commands from the other kuna components (kassert, pipeline, quality,
 * restarts) are also registered through this capability.
 */
#ifndef __KUNA_CONSOLE_HH__
#define __KUNA_CONSOLE_HH__

#include "ifacedecomp.hh"
#include "kuna_stages.hh"

namespace ghidra {

/// \brief (kuna) Interface capability point for the kuna stage-model commands
class IfaceKunaCapability : public IfaceCapability {
  static IfaceKunaCapability ifaceKunaCapability;	///< Singleton instance
  IfaceKunaCapability(void);				///< Singleton constructor
  IfaceKunaCapability(const IfaceKunaCapability &op2);	///< Not implemented
  IfaceKunaCapability &operator=(const IfaceKunaCapability &op2);	///< Not implemented
public:
  virtual void registerCommands(IfaceStatus *status);
};

/// \brief (kuna) `stage list`: print the stage model and sub-stage catalog
class IfcKunaStageList : public IfaceDecompCommand {
public:
  virtual void execute(istream &s);
};

/// \brief (kuna) `stage map [<name>]`: resolve a group/surface/sub-stage to its stage
class IfcKunaStageMap : public IfaceDecompCommand {
public:
  virtual void execute(istream &s);
};

/// \brief (kuna) `stage status`: report active pipeline variant and kuna sub-stage options
class IfcKunaStageStatus : public IfaceDecompCommand {
public:
  virtual void execute(istream &s);
};

/// \brief (kuna) `stage catalog [<option>]`: emit the LLM-settable assertion catalog as JSON
///
/// The machine-readable discovery surface for an external operator (human or LLM):
/// each kuna ArchOption with its legal values, shipped default, live current value
/// (when a program is loaded), stage/sub-stage address, strength, reported rewind
/// scope, originating issue, and symptom-keyed guidance.  Consumed by
/// `python -m kuna.catalog` and rendered to `docs/assertions.md`.
class IfcKunaStageCatalog : public IfaceDecompCommand {
public:
  virtual void execute(istream &s);
};

/// \brief (kuna) `restarts`: dump recorded restart-trigger events for the current function
class IfcKunaRestarts : public IfaceDecompCommand {
public:
  virtual void execute(istream &s);
};

/// \brief (kuna) `pipeline [list|<variant>]`: run a named reduced pipeline as a sub-query
///
/// Exposes the stageJumpTable save/switch/restore idiom (mechanism c-prime)
/// at the console: the current function is (re)processed under the named
/// root action (jumptable/normalize/paramid/register/firstpass), leaving its
/// state inspectable, and the previous root action is always restored so
/// subsequent `decompile` commands run the full pipeline again.  The `list`
/// sub-form is handled internally (see IfcKunaAssert for why).
class IfcKunaPipeline : public IfaceDecompCommand {
  void listPipelines(void);		///< Print the named pipeline variants
public:
  virtual void execute(istream &s);
};

/// \brief (kuna) `quality`: goto-count quality metric over the structured result
///
/// The observable half of quality-gated structuring (kuna roadmap item 4):
/// counts BlockGoto nodes (and whether they formally print), BlockMultiGoto
/// edges, and reports them against the block count.  Read-only; no rollback.
class IfcKunaQuality : public IfaceDecompCommand {
public:
  virtual void execute(istream &s);
};

} // End namespace ghidra
#endif

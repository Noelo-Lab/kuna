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

} // End namespace ghidra
#endif

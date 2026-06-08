/* ###
 * kuna-owned: restart observability (kuna roadmap item 5).
 *
 * This file is NOT part of upstream Ghidra.  Mechanism (c) of the stage
 * model - global restart with persisted hints - is the single most
 * diagnostic signal in the engine and is currently invisible: a function
 * silently decompiles twice and only the persisted Override hints betray
 * why.  This component records WHY at each trigger site:
 *
 *   - dead-definition gate bump   (Heritage::bumpDeadcodeDelay, edge 3)
 *   - multistage jump-table       (JumpTable::recoverLabels, edge 2)
 *   - prototype discovered late   (FuncCallSpecs::deindirect, edge 5)
 *   - prototype forced late       (FuncCallSpecs::forceSet, edge 5)
 *   - suppressed gate bump        (a bump was wanted but a delay was
 *                                  already installed - no restart fires)
 *
 * Each trigger site gains a single (kuna)-marked recorder call (anchor
 * edits tracked in UPSTREAM.md).  Events live in a kuna-owned side table
 * keyed by (Architecture*, entry space index, entry offset) so they survive
 * Funcdata::clear()/clearAnalysis() exactly like the Override store does,
 * without touching funcdata.hh/architecture.hh.  Console command:
 * `restarts` (registered by IfaceKunaCapability).
 *
 * Observability only: recording has no effect on decompilation.
 */
#ifndef __KUNA_RESTARTLOG_HH__
#define __KUNA_RESTARTLOG_HH__

#include "address.hh"
#include <string>
#include <vector>
#include <iostream>

namespace ghidra {

class Funcdata;

/// \brief (kuna) Why a function restart (or suppressed restart) was triggered
enum KunaRestartReason {
  krestart_deadcode_bump = 0,	///< Dead-definition gate bumped (Override::insertDeadcodeDelay) - restart
  krestart_deadcode_suppressed = 1,	///< Gate bump wanted but a delay was already installed - no restart
  krestart_multistage_jump = 2,	///< Jump-table size mismatch -> multistage hint - restart
  krestart_proto_deindirect = 3,	///< Prototype discovered too late at an indirect call - restart
  krestart_proto_forced = 4,	///< Restrictive prototype forced too late at a call site - restart
  krestart_lowered_switch = 5	///< (kuna) Lowered comparison-cascade switch detected late -> restart to install pre-SSA
};

const char *kunaRestartReasonName(KunaRestartReason reason);	///< Human-readable reason name

/// \brief (kuna) Record a restart trigger against \e fd's entry in the side table
///
/// \param fd is the function being restarted
/// \param reason is why
/// \param detail is a free-form site description (space name, call-site address, ...)
void kunaRecordRestart(const Funcdata &fd,KunaRestartReason reason,const std::string &detail);

/// \brief (kuna) Record a restart trigger, formatting the given site address as the detail
void kunaRecordRestart(const Funcdata &fd,KunaRestartReason reason,const Address &site);

/// \brief (kuna) Dump the recorded restart events for \e fd
void kunaDumpRestarts(std::ostream &os,const Funcdata &fd);

} // End namespace ghidra
#endif

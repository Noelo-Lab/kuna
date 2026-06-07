/* ###
 * kuna-owned: the stage registry — the normative stage model made physical.
 *
 * This file is NOT part of upstream Ghidra.  It encodes the kuna stage model
 * (docs/stages.md, docs/stage-model.md) as queryable static data:
 *
 *   - the stages themselves: P0 knowledge plane + S1..S9, with Band B (S3-S6)
 *     membership (the mutual fixed-point band — Ghidra's mainloop);
 *   - the map from every action/rule GROUP name used by
 *     ActionDatabase::universalAction / buildDefaultGroups to its dominant
 *     stage (groups are the load-bearing identity the engine already filters
 *     on — see ActionGroupList::contains — so the registry keys on them
 *     rather than tagging ~200 Action/Rule constructors);
 *   - the sub-stage catalog (stage-model.md sections 4-10): every named
 *     decision point, the assertion that overrides it, its strength
 *     (HARD/HINT), its minimal rewind stage (stage-model.md section 12 —
 *     REPORTED by kassert, not yet enacted), and whether it is LATENT
 *     (decision exists in code, no override surface);
 *   - the surface routing table: which existing console command / option
 *     operates which sub-stage (the "many sub-stages are exposable by
 *     routing" result).
 *
 * Consumed by the kuna console commands (kuna_console.cc: `stage list`,
 * `stage map`, `stage status`) and the typed assertion API (kuna_assert.cc).
 * Pure static data + lookups: no pipeline linkage, no output effect.
 */
#ifndef __KUNA_STAGES_HH__
#define __KUNA_STAGES_HH__

#include "types.h"
#include <string>

namespace ghidra {

/// \brief (kuna) The stages of the kuna stage model (docs/stages.md)
///
/// kstage_infra is a registry-only pseudo-stage for orchestration units
/// (ActionStart/Stop, the cleanup pool's termination policy) that own no
/// model artifact.
enum KunaStage {
  kstage_infra = -1,		///< Orchestration / termination policy (no artifact)
  kstage_p0 = 0,		///< Knowledge & Configuration Plane (orthogonal)
  kstage_s1 = 1,		///< Image & Code Partition
  kstage_s2 = 2,		///< Flow & Op-Graph Recovery
  kstage_s3 = 3,		///< Definition Web (Band B)
  kstage_s4 = 4,		///< Call & Prototype Model (Band B)
  kstage_s5 = 5,		///< Value & Type Facts (Band B)
  kstage_s6 = 6,		///< Variable & Storage Model (Band B)
  kstage_s7 = 7,		///< Region Hierarchy
  kstage_s8 = 8,		///< Structured AST & Goto Quality
  kstage_s9 = 9			///< Surface Rendering & Refinement
};

/// \brief (kuna) Assertion strength (stage-model.md section 2)
enum KunaStrength {
  kstrength_none = 0,		///< No assertion exists (fully LATENT decision)
  kstrength_hint = 1,		///< Biases inference
  kstrength_hard = 2		///< Blocks inference unconditionally
};

/// \brief (kuna) One action/rule group mapped to its dominant stage
struct KunaGroupEntry {
  const char *group;		///< Group name as used in coreaction.cc
  KunaStage stage;		///< Dominant stage (straddlers: by owned artifact, section 15)
  const char *substage;		///< Sub-stage operated, or "" if stage-wide
  const char *note;		///< Straddler / secondary-effect note, or ""
};

/// \brief (kuna) One sub-stage: a named decision point inside a stage
struct KunaSubStage {
  const char *name;		///< Short kebab-case name (stable identifier)
  KunaStage stage;		///< Owning stage
  const char *decision;		///< What is being decided
  const char *assertion;	///< The P0 assertion type that overrides it
  KunaStrength strength;	///< Strength of that assertion
  KunaStage rewind;		///< Minimal invalidated stage (section 12); Ghidra-actual is whole-function
  bool latent;			///< true = no override surface exists in kuna today
  const char *exposure;		///< Surface that operates it ("option ...", "command ...", or roadmap note)
};

/// \brief (kuna) One existing console/option surface routed to a sub-stage
struct KunaSurfaceEntry {
  const char *surface;		///< "option <name>" or the console command words
  KunaStage stage;		///< Stage it operates
  const char *substage;		///< Sub-stage name (in the catalog), or ""
  const char *note;		///< Clarification, or ""
};

/// \brief (kuna) One LLM/agent-settable assertion: the machine-readable catalog row
///
/// This is the documentation surface an external operator (human or LLM) reads to
/// decide WHICH assertion to flip and HOW.  It is emitted as JSON by the
/// `stage catalog` console command (joined with the live current value), rendered
/// to `docs/assertions.md`, and consumed by `python -m kuna.catalog`.  Every kuna
/// ArchOption must have exactly one row here (a drift test enforces this).
struct KunaSettable {
  const char *option;		///< The option name (the `option <name> ...` token)
  const char *values;		///< Pipe-separated legal values, e.g. "on|off" or "canonical|original"
  const char *shipped;		///< The value kuna ships as the default (post-DIV-2 where applicable)
  bool destructive;		///< true = NOT safe as a global default (kept opt-in; see docs/divergences.md)
  KunaStage stage;		///< Owning stage
  const char *substage;		///< Sub-stage name (in the §4-§10 catalog)
  KunaStrength strength;	///< Assertion strength
  KunaStage rewind;		///< Minimal invalidated stage (REPORTED; Ghidra-actual is whole-function)
  const char *issue;		///< Originating issue (e.g. "GH-558"), or ""
  const char *summary;		///< One line: the decision this assertion controls
  const char *use_when;		///< Symptom -> when an operator should flip it away from the default
  const char *example;		///< A ready-to-use console command, e.g. "option compareform canonical"
};

const char *kunaStageCode(KunaStage stage);	///< "P0".."S9" (or "--" for infra)
const char *kunaStageName(KunaStage stage);	///< Long stage name
const char *kunaStageArtifact(KunaStage stage);	///< The artifact the stage owns
bool kunaStageInBandB(KunaStage stage);		///< true for S3..S6
bool kunaStageFromCode(const std::string &code,KunaStage &res);	///< Parse "P0"/"s3"/... into \e res

int4 kunaNumGroups(void);				///< Number of group entries
const KunaGroupEntry &kunaGroupByIndex(int4 i);		///< Get i-th group entry
const KunaGroupEntry *kunaLookupGroup(const std::string &nm);	///< Find group by name (null if missing)

int4 kunaNumSubStages(void);				///< Number of catalog sub-stages
const KunaSubStage &kunaSubStageByIndex(int4 i);	///< Get i-th sub-stage
const KunaSubStage *kunaLookupSubStage(const std::string &nm);	///< Find sub-stage by name (null if missing)

int4 kunaNumSurfaces(void);				///< Number of surface routings
const KunaSurfaceEntry &kunaSurfaceByIndex(int4 i);	///< Get i-th surface routing
const KunaSurfaceEntry *kunaLookupSurface(const std::string &nm);	///< Find surface by exact string (null if missing)

int4 kunaNumSettables(void);				///< Number of LLM-settable assertions in the catalog
const KunaSettable &kunaSettableByIndex(int4 i);	///< Get i-th settable assertion
const KunaSettable *kunaLookupSettable(const std::string &nm);	///< Find settable by option name (null if missing)

} // End namespace ghidra
#endif

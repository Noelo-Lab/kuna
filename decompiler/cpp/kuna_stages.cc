/* ###
 * kuna-owned: the stage registry data (see kuna_stages.hh).
 *
 * Ground truth for the group map is ActionDatabase::buildDefaultGroups /
 * universalAction (coreaction.cc); ground truth for the sub-stage catalog is
 * docs/stage-model.md sections 4-10 (every table row transcribed); ground
 * truth for the surface routing is OptionDatabase::OptionDatabase
 * (options.cc) and IfaceDecompCapability::registerCommands (ifacedecomp.cc).
 * Straddler placement (deadcode, blockrecovery, jumptable, ...) follows the
 * dominant-owned-artifact rule of stage-model.md section 15.
 */
#include "kuna_stages.hh"

namespace ghidra {

static const char *STAGE_CODES[11] = {
  "P0","S1","S2","S3","S4","S5","S6","S7","S8","S9","--" };

static const char *STAGE_NAMES[11] = {
  "Knowledge & Configuration Plane",
  "Image & Code Partition",
  "Flow & Op-Graph Recovery",
  "Definition Web",
  "Call & Prototype Model",
  "Value & Type Facts",
  "Variable & Storage Model",
  "Region Hierarchy",
  "Structured AST & Goto Quality",
  "Surface Rendering & Refinement",
  "Infrastructure / orchestration" };

static const char *STAGE_ARTIFACTS[11] = {
  "assertion store (Symbol DB + Override) + pipeline configuration",
  "bytes + segments + symbol/data map + code-vs-data partition",
  "lifted ops + CFG (bblocks) + jump tables (jumpvec) + work queues",
  "SSA-as-property over the op-graph; heritage state; dead-definition gate",
  "call specs / prototypes / calling conventions",
  "ranges, consume bits, type lattice",
  "HighVariables, stack-frame layout, alias sets",
  "region tree (sblocks - physically distinct from the CFG)",
  "statement tree + goto set (the quality signal)",
  "text + position maps",
  "(none - schedule/termination policy only)" };

const char *kunaStageCode(KunaStage stage)
{
  if (stage == kstage_infra) return STAGE_CODES[10];
  return STAGE_CODES[(int4)stage];
}

const char *kunaStageName(KunaStage stage)
{
  if (stage == kstage_infra) return STAGE_NAMES[10];
  return STAGE_NAMES[(int4)stage];
}

const char *kunaStageArtifact(KunaStage stage)
{
  if (stage == kstage_infra) return STAGE_ARTIFACTS[10];
  return STAGE_ARTIFACTS[(int4)stage];
}

bool kunaStageInBandB(KunaStage stage)
{
  return (stage >= kstage_s3 && stage <= kstage_s6);
}

bool kunaStageFromCode(const std::string &code,KunaStage &res)
{
  if (code.size() != 2) return false;
  char c0 = code[0];
  char c1 = code[1];
  if ((c0 == 'p' || c0 == 'P') && c1 == '0') { res = kstage_p0; return true; }
  if ((c0 == 's' || c0 == 'S') && c1 >= '1' && c1 <= '9') {
    res = (KunaStage)(c1 - '0');
    return true;
  }
  return false;
}

/// Every group name appearing in ActionDatabase::buildDefaultGroups or as a
/// group argument inside universalAction, mapped to its dominant stage.
static const KunaGroupEntry groupTable[] = {
  { "base", kstage_infra, "", "setup/teardown (ActionStart/Stop/Constbase) + core passes shared by every variant" },
  { "protorecovery", kstage_s4, "prototype-source", "ActionActiveParam/ActionReturnRecovery/ActionFuncLink trial recovery" },
  { "protorecovery_a", kstage_s4, "prototype-source", "ActionDirectWrite (propagation on)" },
  { "protorecovery_b", kstage_s4, "prototype-source", "ActionDirectWrite (propagation off)" },
  { "noproto", kstage_s4, "prototype-source", "jumptable-variant stand-in (ActionFuncLinkOutOnly)" },
  { "siganalysis", kstage_s4, "", "paramid/signature variant analysis" },
  { "normalanalysis", kstage_infra, "", "normalize-variant setup (ActionNormalizeSetup)" },
  { "deindirect", kstage_s4, "prototype-source", "indirect call -> direct (proto change may restart: edge 5)" },
  { "localrecovery", kstage_s6, "stack-frame-layout", "RestrictLocal/RestructureVarnode/MappedLocalSync/OutputPrototype" },
  { "deadcode", kstage_s5, "", "STRADDLER (s15): consume-bit fact propagation; destroys S2 ops; gated by S3 dead-definition gate (edge 3)" },
  { "typerecovery", kstage_s5, "type-propagation", "ActionInferTypes/ActionConstantPtr/oppool2 Ptr rules" },
  { "stackptrflow", kstage_s6, "stack-frame-layout", "ActionStackPtrFlow" },
  { "blockrecovery", kstage_s7, "", "STRADDLER (s15): BlockStructure/StructureTransform/FinalStructure span S7/S8; ForceGoto consumes the S7 force-goto assertion" },
  { "stackvars", kstage_s6, "stack-frame-layout", "Load/StoreVarnode rules (stack var promotion)" },
  { "deadcontrolflow", kstage_s2, "reachability-pruning", "ActionRedundBranch/ActionDoNothing (CFG edge removal)" },
  { "switchnorm", kstage_s2, "switch-model", "ActionSwitchNorm (jump-table phase-2 normalization)" },
  { "cleanup", kstage_infra, "", "cleanup pool: deferred after fullloop as termination policy (stage-model s14); rules act on S3's artifact" },
  { "splitcopy", kstage_s5, "aggregate-union", "RuleSplitCopy (splitdatatype option)" },
  { "splitpointer", kstage_s5, "aggregate-union", "RuleSplitLoad/Store via pointer (splitdatatype option)" },
  { "merge", kstage_s6, "merge-aggressiveness", "merge phalanx (HighVariable formation) + MarkExplicit/Implied + NameVars" },
  { "dynamic", kstage_s6, "alias-facets", "ActionDynamicMapping/ActionDynamicSymbols (DynamicHash symbols)" },
  { "casts", kstage_s9, "cast-policy", "ActionSetCasts" },
  { "analysis", kstage_s3, "simplification-quiescence", "the main simplification rule pool (oppool1) + MultiCse/ShadowVar/ConditionalConst" },
  { "canonicalcompare", kstage_s3, "comparison-canonicalization", "(kuna) GH-558: RuleIntLessEqual split out of analysis" },
  { "presentcompare", kstage_s9, "comparison-canonicalization", "(kuna) GH-558: ActionPresentCompareForm at the S8->S9 boundary" },
  { "fixateglobals", kstage_s1, "code-data-partition", "ActionMapGlobals: the C++ side's late write-back into the global/data map" },
  { "fixateproto", kstage_s4, "prototype-source", "ActionInputPrototype (commit recovered signature -> P0)" },
  { "constsequence", kstage_s5, "", "RuleStringCopy/RuleStringStore (builtin memcpy/strncpy recognition)" },
  { "bitfields", kstage_s5, "aggregate-union", "RuleBitfield*" },
  { "segment", kstage_s2, "", "RuleSegment/ActionSegmentize (segmented address spaces)" },
  { "returnsplit", kstage_s4, "", "ActionReturnSplit" },
  { "nodejoin", kstage_s3, "", "ActionNodeJoin (phi/def-web node joining)" },
  { "doubleload", kstage_s5, "", "double-precision load pairing" },
  { "doubleprecis", kstage_s5, "", "double-precision arithmetic recovery" },
  { "unreachable", kstage_s2, "reachability-pruning", "ActionDeterminedBranch/ActionUnreachable (edge 6: S5 facts remove CFG edges)" },
  { "subvar", kstage_s3, "", "subvariable-flow transforms" },
  { "floatprecision", kstage_s5, "", "float precision normalization" },
  { "conditionalexe", kstage_s3, "", "ActionConditionalExe/RuleOrPredicate" },
  { "normalizebranches", kstage_s8, "", "ActionNormalizeBranches (branch normalization for structuring)" }
};

int4 kunaNumGroups(void)
{
  return sizeof(groupTable) / sizeof(KunaGroupEntry);
}

const KunaGroupEntry &kunaGroupByIndex(int4 i)
{
  return groupTable[i];
}

const KunaGroupEntry *kunaLookupGroup(const std::string &nm)
{
  for (int4 i=0;i<kunaNumGroups();++i) {
    if (nm == groupTable[i].group)
      return groupTable + i;
  }
  return (const KunaGroupEntry *)0;
}

/// The sub-stage catalog: every decision-point row of stage-model.md
/// sections 4-10.  \e rewind is the minimal invalidated stage from section
/// 12 (kassert REPORTS it; Ghidra-actual rewind is whole-function).
static const KunaSubStage substageTable[] = {
  // --- P0 (s4 of stage-model.md) ---
  { "assertion-admission", kstage_p0,
    "is an incoming fact ground truth or a bias; does it clear caches or only reflow",
    "per-fact lock / cache policy", kstrength_hard, kstage_p0, true,
    "LATENT: confidence-graded admission (roadmap 16.6); kuna kassert records hard|hint" },
  { "pipeline-variant", kstage_p0,
    "which named pipeline/preset runs; restart budget",
    "configuration assertion", kstrength_hard, kstage_s1, false,
    "option setaction; command pipeline (kuna)" },
  // --- S1 (s5) ---
  { "environment-binding", kstage_s1,
    "loader, architecture, platform/ABI, load address",
    "environment assertion", kstrength_hard, kstage_s1, false,
    "option setlanguage; .ldefs/.cspec selection at SleighArchitecture build" },
  { "code-data-partition", kstage_s1,
    "which gaps are code; which data cells are code pointers; segment permissions",
    "segment/global annotation", kstrength_hard, kstage_s1, false,
    "command global add/remove/spaces; command readonly; option readonly" },
  // --- S2 (s6) ---
  { "decode-context", kstage_s2,
    "which decoder context (THUMB, addr16/32, ...) applies at each address",
    "context-set at address", kstrength_hard, kstage_s2, false,
    "command set context; option allowcontextset" },
  { "decode-error-policy", kstage_s2,
    "unimplemented/bad-data/out-of-bounds/too-many-instructions: truncate vs throw vs ignore",
    "flow option flags", kstrength_hard, kstage_s2, false,
    "options ignoreunimplemented/errorunimplemented/errorreinterpreted/errortoomanyinstructions/maxinstruction" },
  { "flow-classification", kstage_s2,
    "call-vs-branch (PIC, thunk, tail call); noreturn kills fall-through",
    "flow override", kstrength_hard, kstage_s2, false,
    "command override flow; option noreturn" },
  { "inline-inject", kstage_s2,
    "clone callee body vs splice fixup p-code vs keep call",
    "inline/inject directive", kstrength_hard, kstage_s2, false,
    "option inline; command fixup call / fixup callother" },
  { "switch-model", kstage_s2,
    "which model explains a BRANCHIND (Basic/Basic2/Trivial/Assisted/Override); table bounds; case count",
    "explicit jump table", kstrength_hard, kstage_s2, false,
    "command override jumptable; Override::insertMultistageJump; options jumptablemax/jumpload" },
  { "reachability-pruning", kstage_s2,
    "remove or keep unreachable blocks",
    "unreachable policy", kstrength_hint, kstage_s2, true,
    "LATENT at user level (removeUnreachableBlocks flag-gated in flow)" },
  // --- S3 (s7.1) ---
  { "heritage-staging", kstage_s3,
    "per-space order and delay of SSA construction (stack delayed to collect varnodes first)",
    "per-space delay", kstrength_hard, kstage_s3, true,
    "LATENT at user level (HeritageInfo.delay from arch spec)" },
  { "dead-definition-gate", kstage_s3,
    "when dead-code removal is allowed per space (pass > deadcodedelay); premature removal -> restart",
    "deadcode-delay", kstrength_hard, kstage_s3, false,
    "command deadcode delay (Override::insertDeadcodeDelay; auto-bumped, survives restart - edge 3)" },
  { "phi-range-granularity", kstage_s3,
    "split a memory range at gaps before placing phis (hardcoded size>4 && max<size)",
    "(none today)", kstrength_none, kstage_s3, true,
    "LATENT everywhere (matters for struct-heavy frames)" },
  { "simplification-quiescence", kstage_s3,
    "which algebraic/peephole rules fire; when the local fixed point is declared",
    "per-rule enable/disable", kstrength_hard, kstage_s3, false,
    "option togglerule; arch extra_pool_rules injection slot" },
  { "comparison-canonicalization", kstage_s3,
    "canonicalize V <= c into V < c+1 for analysis (3 sites via replaceLessequal)",
    "compareform canonical|original", kstrength_hard, kstage_s3, false,
    "(kuna GH-558) option compareform; groups canonicalcompare/presentcompare" },
  // --- S4 (s7.2) ---
  { "prototype-source", kstage_s4,
    "locked/declared signature vs trial recovery",
    "locked prototype", kstrength_hard, kstage_s4, false,
    "commands prototype lock / override prototype / parse line; map param/return" },
  { "trial-budget", kstage_s4,
    "how many passes before param/return trials freeze (numpasses >= maxpass)",
    "pass budget", kstrength_hint, kstage_s4, true,
    "LATENT as a user assertion (maxpass per prototype model)" },
  { "abi-trust", kstage_s4,
    "trust platform ABI for trashed/preserved vs infer from dataflow",
    "ABI-trust flag", kstrength_hint, kstage_s4, false,
    "options defaultprototype/protoeval" },
  { "extrapop", kstage_s4,
    "stack adjustment across calls when underdetermined (StackSolver)",
    "extrapop override", kstrength_hard, kstage_s4, false,
    "option extrapop" },
  { "interproc-regime", kstage_s4,
    "per-function-with-restart vs bottom-up SCC summaries vs on-demand whole-program",
    "regime selection", kstrength_none, kstage_s4, true,
    "LATENT in all three decompilers (each regime hardcoded)" },
  // --- S5 (s7.3) ---
  { "range-facts", kstage_s5,
    "widening strategy and iteration cutoffs; feeds S2 jump-table bounds and determined branches",
    "range assertion", kstrength_hard, kstage_s5, true,
    "LATENT everywhere (only indirect: option jumptablemax caps case count)" },
  { "const-pointer", kstage_s5,
    "is this constant an address? (space attribute > usage inference)",
    "pointer/equate annotation", kstrength_hard, kstage_s5, false,
    "option inferconstptr; commands map address / global add" },
  { "type-propagation", kstage_s5,
    "per-opcode flow direction; lattice solve; type locks are HARD stops",
    "type lock | type hint", kstrength_hard, kstage_s5, false,
    "commands retype / type varnode (typelock); Ghidra-actual rewind: whole function" },
  { "aggregate-union", kstage_s5,
    "when to synthesize struct/array; which union field wins (ScoreUnionFields weights)",
    "union facet | must-struct", kstrength_hard, kstage_s5, false,
    "command map unionfacet (DynamicHash); option splitdatatype; scoring weights LATENT" },
  { "solver-budget", kstage_s5,
    "type-propagation pass cap (7, compile-time)",
    "solver budget", kstrength_hint, kstage_s5, true,
    "LATENT (compile-time constant)" },
  // --- S6 (s7.4) ---
  { "merge-aggressiveness", kstage_s6,
    "forced (phi/address-tied) vs speculative (cover/type) vs adjacent merges",
    "isolate", kstrength_hard, kstage_s6, false,
    "command isolate (Symbol::setIsolated blocks speculative merge)" },
  { "stack-frame-layout", kstage_s6,
    "overlapping RangeHint conflict policy: typelock > rangeType > type specificity",
    "stack symbol + type lock", kstrength_hard, kstage_s6, false,
    "command map address + retype (locked stack symbols)" },
  { "alias-facets", kstage_s6,
    "which stack offsets a pointer may alias; binding symbols to temporaries by dataflow hash",
    "dynamic-hash symbol", kstrength_hard, kstage_s6, false,
    "command map hash; option aliasblock (boundary policy itself LATENT)" },
  { "explicit-implied", kstage_s6,
    "does a value get its own named token or inline into its parent expression",
    "per-variable visibility", kstrength_hint, kstage_s9, true,
    "LATENT per-variable everywhere (high-value readability knob)" },
  // --- S7 (s8) ---
  { "loop-refinement", kstage_s7,
    "how aggressively to absorb nodes into loops; force single-exit via guarded successors",
    "region-shaping options", kstrength_hint, kstage_s7, true,
    "LATENT in Ghidra/Reko (angr-only options)" },
  { "edge-virtualization", kstage_s7,
    "which edge to cut (become goto) when a region won't structure",
    "force-goto (per-edge)", kstrength_hard, kstage_s7, false,
    "command force goto (Override::insertForceGoto)" },
  { "reident-batching", kstage_s7,
    "defer graph-topology updates so de-optimization passes can iterate",
    "pass-set selection", kstrength_hint, kstage_s7, true,
    "LATENT in Ghidra (angr delay_graph_updates)" },
  // --- S8 (s9) ---
  { "schema-precedence", kstage_s8,
    "rule order when structuring patterns overlap (greedy: Goto,Cat,If,IfElse,While,DoWhile,InfLoop,Switch)",
    "structurer selection", kstrength_hint, kstage_s8, true,
    "LATENT rule order in Ghidra/Reko (angr structurer_cls)" },
  { "goto-quality-acceptance", kstage_s8,
    "accept or roll back a graph edit based on measured structure quality (goto count, loop shape)",
    "quality policy", kstrength_hint, kstage_s8, true,
    "LATENT - no rollback in Ghidra (kuna command quality = observable half; roadmap 16.4)" },
  { "readability-rewrites", kstage_s8,
    "switch-vs-if-cascade, for-recognition, condition complexity",
    "structuring options", kstrength_hint, kstage_s8, false,
    "options analyzeforloops / jumptablemax" },
  // --- S9 (s10) ---
  { "naming-policy", kstage_s9,
    "auto vs symbol-derived vs semantic-pattern names; preservation of user names",
    "name lock (cosmetic)", kstrength_hard, kstage_s9, false,
    "commands rename / name varnode (namelock); option namespacestrategy" },
  { "cast-policy", kstage_s9,
    "minimal-necessary vs always vs never (CastStrategyC)",
    "cast option", kstrength_hard, kstage_s9, false,
    "option nocastprinting (global; per-expression LATENT everywhere)" },
  { "literal-format", kstage_s9,
    "integer base, NULL-vs-0, per-symbol display format",
    "format assertion (per-symbol or global)", kstrength_hard, kstage_s9, false,
    "options integerformat/nullprinting; commands force varnode / force datatype / map convert" },
  { "pointer-notation", kstage_s9,
    "render standalone PTRADD as base + index vs &base[index]",
    "arraynotation on|off", kstrength_hard, kstage_s9, false,
    "(kuna GH-558) option arraynotation -> PrintC::opPtradd" },
  { "external-refinement", kstage_s9,
    "proposed renames/retypes/summaries: an assertion writer into P0 for the next run",
    "any P0 assertion", kstrength_hint, kstage_p0, false,
    "the console itself; kassert (kuna) is the uniform writer" }
};

int4 kunaNumSubStages(void)
{
  return sizeof(substageTable) / sizeof(KunaSubStage);
}

const KunaSubStage &kunaSubStageByIndex(int4 i)
{
  return substageTable[i];
}

const KunaSubStage *kunaLookupSubStage(const std::string &nm)
{
  for (int4 i=0;i<kunaNumSubStages();++i) {
    if (nm == substageTable[i].name)
      return substageTable + i;
  }
  return (const KunaSubStage *)0;
}

/// Existing console-command / option surfaces routed to the sub-stage they
/// operate.  This is the "exposure by routing" table: the assertion surface
/// largely already exists; the registry gives it a stage address.
static const KunaSurfaceEntry surfaceTable[] = {
  { "option extrapop", kstage_s4, "extrapop", "" },
  { "option readonly", kstage_s1, "code-data-partition", "treat read-only memory as constant" },
  { "option defaultprototype", kstage_s4, "abi-trust", "" },
  { "option protoeval", kstage_s4, "abi-trust", "evaluation model for called functions" },
  { "option inferconstptr", kstage_s5, "const-pointer", "" },
  { "option analyzeforloops", kstage_s8, "readability-rewrites", "" },
  { "option inline", kstage_s2, "inline-inject", "" },
  { "option noreturn", kstage_s2, "flow-classification", "" },
  { "option warning", kstage_p0, "", "diagnostic routing (cross-cutting)" },
  { "option nullprinting", kstage_s9, "literal-format", "" },
  { "option inplaceops", kstage_s9, "literal-format", "+=/-= rendering" },
  { "option conventionprinting", kstage_s9, "naming-policy", "" },
  { "option nocastprinting", kstage_s9, "cast-policy", "" },
  { "option integerformat", kstage_s9, "literal-format", "" },
  { "option maxlinewidth", kstage_s9, "", "pure formatting" },
  { "option indentincrement", kstage_s9, "", "pure formatting" },
  { "option commentstyle", kstage_s9, "", "pure formatting" },
  { "option braceformat", kstage_s9, "", "pure formatting" },
  { "option setaction", kstage_p0, "pipeline-variant", "" },
  { "option currentaction", kstage_p0, "pipeline-variant", "" },
  { "option allowcontextset", kstage_s2, "decode-context", "" },
  { "option setlanguage", kstage_s1, "environment-binding", "" },
  { "option jumptablemax", kstage_s2, "switch-model", "also S8 readability straddler" },
  { "option jumpload", kstage_s2, "switch-model", "" },
  { "option togglerule", kstage_s3, "simplification-quiescence", "" },
  { "option aliasblock", kstage_s6, "alias-facets", "" },
  { "option maxinstruction", kstage_s2, "decode-error-policy", "" },
  { "option ignoreunimplemented", kstage_s2, "decode-error-policy", "" },
  { "option errorunimplemented", kstage_s2, "decode-error-policy", "" },
  { "option errorreinterpreted", kstage_s2, "decode-error-policy", "" },
  { "option errortoomanyinstructions", kstage_s2, "decode-error-policy", "" },
  { "option namespacestrategy", kstage_s9, "naming-policy", "" },
  { "option splitdatatype", kstage_s5, "aggregate-union", "" },
  { "option nanignore", kstage_s5, "", "float NaN-test simplification policy" },
  { "option compareform", kstage_s3, "comparison-canonicalization", "(kuna GH-558)" },
  { "option arraynotation", kstage_s9, "pointer-notation", "(kuna GH-558)" },
  { "option thumbfuncptr", kstage_s5, "const-pointer", "(kuna GH-8471) preserve mode-bit-encoded function pointers" },
  { "option inferfuncentry", kstage_s5, "const-pointer", "(kuna GH-6930) infer function entries at single-bit image bases" },
  { "force goto", kstage_s7, "edge-virtualization", "Override::insertForceGoto" },
  { "override jumptable", kstage_s2, "switch-model", "JumpBasicOverride" },
  { "override flow", kstage_s2, "flow-classification", "Override::insertFlowOverride" },
  { "override prototype", kstage_s4, "prototype-source", "Override::insertProtoOverride" },
  { "deadcode delay", kstage_s3, "dead-definition-gate", "Override::insertDeadcodeDelay" },
  { "prototype lock", kstage_s4, "prototype-source", "FuncProto input/output lock" },
  { "prototype unlock", kstage_s4, "prototype-source", "" },
  { "parse line", kstage_s4, "prototype-source", "declared signature (also seeds S5 types)" },
  { "retype", kstage_s5, "type-propagation", "symbol typelock - HARD stop in propagateTypeEdge" },
  { "type varnode", kstage_s5, "type-propagation", "" },
  { "map unionfacet", kstage_s5, "aggregate-union", "DynamicHash-keyed UnionFacetSymbol" },
  { "map hash", kstage_s6, "alias-facets", "DynamicHash symbol attachment" },
  { "map address", kstage_s6, "stack-frame-layout", "also S1 globals when address is global" },
  { "map convert", kstage_s9, "literal-format", "" },
  { "force varnode", kstage_s9, "literal-format", "" },
  { "force datatype", kstage_s9, "literal-format", "" },
  { "rename", kstage_s9, "naming-policy", "namelock" },
  { "name varnode", kstage_s9, "naming-policy", "" },
  { "isolate", kstage_s6, "merge-aggressiveness", "Symbol::setIsolated" },
  { "set context", kstage_s2, "decode-context", "ContextDatabase" },
  { "global add", kstage_s1, "code-data-partition", "" },
  { "global remove", kstage_s1, "code-data-partition", "" },
  { "volatile", kstage_s1, "code-data-partition", "userop semantics straddle S3/S5 (s15)" },
  { "readonly", kstage_s1, "code-data-partition", "" },
  { "fixup call", kstage_s2, "inline-inject", "p-code injection" },
  { "fixup callother", kstage_s2, "inline-inject", "" },
  { "structure blocks", kstage_s7, "", "direct structurer sub-query on an arbitrary graph" },
  { "kassert", kstage_p0, "assertion-admission", "(kuna) uniform typed assertion writer" },
  { "pipeline", kstage_p0, "pipeline-variant", "(kuna) reduced-pipeline sub-query (mechanism c-prime)" },
  { "quality", kstage_s8, "goto-quality-acceptance", "(kuna) goto-count metric, observable half" },
  { "restarts", kstage_p0, "", "(kuna) feedback-edge observability (mechanism c)" }
};

int4 kunaNumSurfaces(void)
{
  return sizeof(surfaceTable) / sizeof(KunaSurfaceEntry);
}

const KunaSurfaceEntry &kunaSurfaceByIndex(int4 i)
{
  return surfaceTable[i];
}

const KunaSurfaceEntry *kunaLookupSurface(const std::string &nm)
{
  for (int4 i=0;i<kunaNumSurfaces();++i) {
    if (nm == surfaceTable[i].surface)
      return surfaceTable + i;
  }
  return (const KunaSurfaceEntry *)0;
}

} // End namespace ghidra

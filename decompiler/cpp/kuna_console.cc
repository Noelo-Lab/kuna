/* ###
 * kuna-owned: kuna console capability + stage-registry commands
 * (see kuna_console.hh).
 */
#include "kuna_console.hh"
#include "kuna_assert.hh"
#include "kuna_restartlog.hh"
#include "printc.hh"

namespace ghidra {

// Constructing this registers the capability
IfaceKunaCapability IfaceKunaCapability::ifaceKunaCapability;

IfaceKunaCapability::IfaceKunaCapability(void)

{
  name = "kuna";
}

void IfaceKunaCapability::registerCommands(IfaceStatus *status)

{
  status->registerCom(new IfcKunaStageList(),"stage","list");
  status->registerCom(new IfcKunaStageMap(),"stage","map");
  status->registerCom(new IfcKunaStageStatus(),"stage","status");
  status->registerCom(new IfcKunaStageCatalog(),"stage","catalog");
  status->registerCom(new IfcKunaAssert(),"kassert");
  status->registerCom(new IfcKunaRestarts(),"restarts");
  status->registerCom(new IfcKunaPipeline(),"pipeline");
  status->registerCom(new IfcKunaQuality(),"quality");
}

/// The named pipeline variants built by ActionDatabase::buildDefaultGroups
static const char *PIPELINE_VARIANTS[] = {
  "decompile", "jumptable", "normalize", "paramid", "register", "firstpass" };
static const int4 NUM_PIPELINE_VARIANTS = 6;

/// \class IfcKunaStageList
/// \brief List the stage model: stages, Band B, and the sub-stage decision catalog
///
/// Pure static data from the registry (kuna_stages.cc); requires no program.
void IfcKunaStageList::execute(istream &s)

{
  ostream &os( *status->fileoptr );
  os << "Stages (kuna stage model, docs/stages.md / docs/stage-model.md):" << endl;
  for(int4 i=0;i<=9;++i) {
    KunaStage stage = (KunaStage)i;
    os << "  " << kunaStageCode(stage) << "  " << kunaStageName(stage);
    if (stage == kstage_p0)
      os << "  [orthogonal plane]";
    else if (kunaStageInBandB(stage))
      os << "  [Band B]";
    os << endl;
    os << "        artifact: " << kunaStageArtifact(stage) << endl;
  }
  os << endl;
  os << "Sub-stages (named decision points; LATENT = no override surface today):" << endl;
  for(int4 i=0;i<kunaNumSubStages();++i) {
    const KunaSubStage &sub( kunaSubStageByIndex(i) );
    os << "  [" << kunaStageCode(sub.stage) << "] " << sub.name;
    if (sub.latent)
      os << "  (LATENT)";
    os << endl;
    os << "        decision: " << sub.decision << endl;
    os << "        assertion: " << sub.assertion;
    if (sub.strength == kstrength_hard)
      os << " (HARD)";
    else if (sub.strength == kstrength_hint)
      os << " (HINT)";
    os << "   rewind: " << kunaStageCode(sub.rewind) << endl;
    os << "        exposure: " << sub.exposure << endl;
  }
}

/// \class IfcKunaStageMap
/// \brief Resolve a name (action/rule group, console surface, or sub-stage) to its stage
///
/// With no argument, dump the full group->stage and surface->stage tables.
void IfcKunaStageMap::execute(istream &s)

{
  string token;
  s >> ws;
  while(!s.eof()) {
    string word;
    s >> word >> ws;
    if (word.empty()) break;
    if (!token.empty()) token += ' ';
    token += word;
  }
  ostream &os( *status->fileoptr );
  if (token.empty()) {		// Dump everything
    os << "Action/rule groups -> stage (dominant artifact; see stage-model.md s15 for straddlers):" << endl;
    for(int4 i=0;i<kunaNumGroups();++i) {
      const KunaGroupEntry &entry( kunaGroupByIndex(i) );
      os << "  " << kunaStageCode(entry.stage) << "  " << entry.group;
      if (entry.substage[0] != '\0')
	os << "  (" << entry.substage << ")";
      os << endl;
      if (entry.note[0] != '\0')
	os << "        " << entry.note << endl;
    }
    os << endl;
    os << "Console surfaces -> stage:" << endl;
    for(int4 i=0;i<kunaNumSurfaces();++i) {
      const KunaSurfaceEntry &entry( kunaSurfaceByIndex(i) );
      os << "  " << kunaStageCode(entry.stage) << "  " << entry.surface;
      if (entry.substage[0] != '\0')
	os << "  (" << entry.substage << ")";
      if (entry.note[0] != '\0')
	os << "  -- " << entry.note;
      os << endl;
    }
    return;
  }
  bool found = false;
  const KunaGroupEntry *grp = kunaLookupGroup(token);
  if (grp != (const KunaGroupEntry *)0) {
    found = true;
    os << "group " << grp->group << " -> " << kunaStageCode(grp->stage) << " (" << kunaStageName(grp->stage) << ")";
    if (grp->substage[0] != '\0')
      os << " sub-stage " << grp->substage;
    os << endl;
    if (grp->note[0] != '\0')
      os << "  " << grp->note << endl;
  }
  const KunaSurfaceEntry *surf = kunaLookupSurface(token);
  if (surf != (const KunaSurfaceEntry *)0) {
    found = true;
    os << "surface \"" << surf->surface << "\" -> " << kunaStageCode(surf->stage) << " (" << kunaStageName(surf->stage) << ")";
    if (surf->substage[0] != '\0')
      os << " sub-stage " << surf->substage;
    os << endl;
    if (surf->note[0] != '\0')
      os << "  " << surf->note << endl;
  }
  const KunaSubStage *sub = kunaLookupSubStage(token);
  if (sub != (const KunaSubStage *)0) {
    found = true;
    os << "sub-stage " << sub->name << " -> " << kunaStageCode(sub->stage) << " (" << kunaStageName(sub->stage) << ")";
    if (sub->latent)
      os << "  LATENT";
    os << endl;
    os << "  decision: " << sub->decision << endl;
    os << "  assertion: " << sub->assertion << "   rewind: " << kunaStageCode(sub->rewind) << endl;
    os << "  exposure: " << sub->exposure << endl;
  }
  if (!found)
    throw IfaceExecutionError("Unknown group/surface/sub-stage: "+token);
}

/// \class IfcKunaStageStatus
/// \brief Report the active pipeline variant and the state of kuna sub-stage options
void IfcKunaStageStatus::execute(istream &s)

{
  if (dcp->conf == (Architecture *)0)
    throw IfaceExecutionError("No load image present");
  ostream &os( *status->fileoptr );	// bulk stream: assertable from datatests
  os << "pipeline variant: " << dcp->conf->allacts.getCurrentName() << endl;
  os << "compareform: " << (dcp->conf->present_lessequal ? "original" : "canonical") << endl;
  PrintC *lng = dynamic_cast<PrintC *>(dcp->conf->print);
  if (lng != (PrintC *)0)
    os << "arraynotation: " << (lng->getArrayNotation() ? "on" : "off") << endl;
}

/// Append \e str to \e os as a JSON string literal (quotes + minimal escaping).
static void kunaJsonString(ostream &os,const char *str)

{
  os << '"';
  for(const char *p=str;*p!='\0';++p) {
    char c = *p;
    if (c == '"' || c == '\\')
      os << '\\' << c;
    else if (c == '\n')
      os << "\\n";
    else if ((unsigned char)c < 0x20)
      os << ' ';			// drop other control chars (none expected in the catalog)
    else
      os << c;
  }
  os << '"';
}

/// Read the live value of a kuna settable from the loaded Architecture, or "" if it
/// cannot be determined (no program loaded, or no live reader for this option).
static string kunaLiveValue(Architecture *conf,const string &option)

{
  if (conf == (Architecture *)0)
    return "";
  if (option == "compareform")
    return conf->present_lessequal ? "original" : "canonical";
  if (option == "arraynotation") {
    PrintC *lng = dynamic_cast<PrintC *>(conf->print);
    return (lng != (PrintC *)0) ? (lng->getArrayNotation() ? "on" : "off") : string("");
  }
  if (option == "thumbfuncptr")
    return conf->preserve_thumb_funcptr ? "on" : "off";
  if (option == "inferfuncentry")
    return conf->infer_funcentry ? "on" : "off";
  if (option == "booleanmask")
    return conf->fold_boolean_mask ? "on" : "off";
  if (option == "ovlesssimplify")
    return conf->ov_less_simplify ? "on" : "off";
  if (option == "flagcompare")
    return conf->fold_flag_compare ? "on" : "off";
  if (option == "addcarrychain")
    return conf->add_carry_chain ? "on" : "off";
  if (option == "memsetrecover")
    return conf->memset_recover ? "on" : "off";
  if (option == "returnpair")
    return conf->return_single ? "single" : "pair";
  if (option == "inputvarnodeadjust")
    return conf->input_varnode_adjust ? "on" : "off";
  if (option == "condexeplace")
    return conf->condexe_block_placement ? "on" : "off";
  if (option == "sparcstructret")
    return conf->sparc_struct_return ? "on" : "off";
  if (option == "arraystride")
    return conf->recover_array_stride ? "on" : "off";
  if (option == "stackalias")
    return conf->stack_alias_deadstore ? "on" : "off";
  if (option == "dynamichashmax")
    return conf->dynamic_hash_maxdup_high ? "on" : "off";
  if (option == "stackprobeloop")
    return conf->model_stack_probe_loop ? "on" : "off";
  if (option == "v850indirectbranch")
    return conf->v850_indirect_branch ? "on" : "off";
  if (option == "switchmodbound")
    return conf->switch_modulo_bound ? "on" : "off";
  return "";
}

/// Emit one settable as a JSON object onto \e os.  Splits the pipe-separated values
/// string into a JSON array; joins the live current value from \e conf when known.
static void kunaEmitSettableJson(ostream &os,const KunaSettable &st,Architecture *conf)

{
  os << "  {";
  os << "\"option\": "; kunaJsonString(os,st.option);
  os << ", \"values\": [";
  string vals(st.values);
  string::size_type pos = 0;
  bool first = true;
  while(pos <= vals.size()) {
    string::size_type bar = vals.find('|',pos);
    string tok = (bar == string::npos) ? vals.substr(pos) : vals.substr(pos,bar-pos);
    if (!first) os << ", ";
    kunaJsonString(os,tok.c_str());
    first = false;
    if (bar == string::npos) break;
    pos = bar + 1;
  }
  os << "]";
  os << ", \"default\": "; kunaJsonString(os,st.shipped);
  string live = kunaLiveValue(conf,st.option);
  if (!live.empty()) { os << ", \"current\": "; kunaJsonString(os,live.c_str()); }
  os << ", \"destructive_as_default\": " << (st.destructive ? "true" : "false");
  os << ", \"stage\": "; kunaJsonString(os,kunaStageCode(st.stage));
  os << ", \"substage\": "; kunaJsonString(os,st.substage);
  os << ", \"strength\": "; kunaJsonString(os,(st.strength==kstrength_hard)?"HARD":(st.strength==kstrength_hint)?"HINT":"NONE");
  os << ", \"rewind\": "; kunaJsonString(os,kunaStageCode(st.rewind));
  os << ", \"issue\": "; kunaJsonString(os,st.issue);
  os << ", \"summary\": "; kunaJsonString(os,st.summary);
  os << ", \"use_when\": "; kunaJsonString(os,st.use_when);
  os << ", \"example\": "; kunaJsonString(os,st.example);
  os << ", \"source_decompiler\": "; kunaJsonString(os,st.source_decompiler);
  os << ", \"inspiration\": "; kunaJsonString(os,st.inspiration);
  os << ", \"change_kind\": "; kunaJsonString(os,st.change_kind);
  os << "}";
}

/// \class IfcKunaStageCatalog
/// \brief Emit the LLM-settable assertion catalog as JSON
///
/// `stage catalog` dumps every settable; `stage catalog <option>` dumps one.  Works
/// with no program loaded (static doc); the live "current" field is added per option
/// when an Architecture is present.
void IfcKunaStageCatalog::execute(istream &s)

{
  string option;
  s >> ws >> option;
  Architecture *conf = (dcp != (IfaceDecompData *)0) ? dcp->conf : (Architecture *)0;
  ostream &os( *status->fileoptr );	// bulk stream: assertable + cleanly captured by kuna.catalog
  if (!option.empty()) {
    const KunaSettable *st = kunaLookupSettable(option);
    if (st == (const KunaSettable *)0)
      throw IfaceExecutionError("Unknown settable option: "+option+" (try `stage catalog`)");
    kunaEmitSettableJson(os,*st,conf);
    os << endl;
    return;
  }
  os << "[" << endl;
  for(int4 i=0;i<kunaNumSettables();++i) {
    kunaEmitSettableJson(os,kunaSettableByIndex(i),conf);
    if (i + 1 < kunaNumSettables())
      os << ",";
    os << endl;
  }
  os << "]" << endl;
}

/// \class IfcKunaRestarts
/// \brief Dump the restart-trigger events recorded for the current function
///
/// The observable half of mechanism (c): WHY the function restarted
/// (dead-definition gate bump, multistage jump, late prototype), as recorded
/// by the kuna side table at each trigger site.
void IfcKunaRestarts::execute(istream &s)

{
  if (dcp->fd == (Funcdata *)0)
    throw IfaceExecutionError("No function selected");
  kunaDumpRestarts(*status->fileoptr,*dcp->fd);	// bulk stream: assertable from datatests
}

/// Print the named pipeline variants: `pipeline list`
void IfcKunaPipeline::listPipelines(void)

{
  ostream &os( *status->fileoptr );
  os << "Named pipeline variants (group filters over the universal action; P0 pipeline-variant sub-stage):" << endl;
  for(int4 i=0;i<NUM_PIPELINE_VARIANTS;++i) {
    os << "  " << PIPELINE_VARIANTS[i];
    if (dcp->conf != (Architecture *)0 && dcp->conf->allacts.getCurrentName() == PIPELINE_VARIANTS[i])
      os << "  (current)";
    os << endl;
  }
}

/// \class IfcKunaPipeline
/// \brief Run a named reduced pipeline on the current function as a sub-query
///
/// Mirrors Funcdata::stageJumpTable's save/switch/restore of the root action
/// (mechanism c-prime), but on the real function in the console rather than
/// a clone: process under the reduced variant, leave the result inspectable,
/// always restore the previous root action.
void IfcKunaPipeline::execute(istream &s)

{
  string name;
  s >> ws >> name;
  if (name.empty() || name == "list") {
    listPipelines();
    return;
  }
  bool valid = false;
  for(int4 i=0;i<NUM_PIPELINE_VARIANTS;++i)
    if (name == PIPELINE_VARIANTS[i]) { valid = true; break; }
  if (!valid)
    throw IfaceParseError("Unknown pipeline variant: "+name+" (try `pipeline list`)");
  if (dcp->conf == (Architecture *)0)
    throw IfaceExecutionError("No load image present");
  if (dcp->fd == (Funcdata *)0)
    throw IfaceExecutionError("No function selected");
  if (dcp->fd->hasNoCode()) {
    *status->optr << "No code for " << dcp->fd->getName() << endl;
    return;
  }
  if (dcp->fd->isProcStarted()) {
    *status->optr << "Clearing old decompilation" << endl;
    dcp->conf->clearAnalysis(dcp->fd);
  }
  string oldactname = dcp->conf->allacts.getCurrentName();	// Save off old action
  int4 res = -1;
  try {
    dcp->conf->allacts.setCurrent(name);
    *status->optr << "Processing " << dcp->fd->getName() << " under reduced pipeline `" << name << '`' << endl;
    dcp->conf->allacts.getCurrent()->reset(*dcp->fd);
    res = dcp->conf->allacts.getCurrent()->perform(*dcp->fd);
    dcp->conf->allacts.setCurrent(oldactname);	// Restore old action
  }
  catch(LowlevelError &err) {
    dcp->conf->allacts.setCurrent(oldactname);	// Always restore old action
    throw IfaceExecutionError("Sub-query failed: "+err.explain);
  }
  if (res < 0)
    *status->optr << "Break" << endl;
  else
    *status->optr << "Sub-query complete (root action restored to `" << oldactname << "`)" << endl;
}

/// Recursively count goto blocks/edges in the structured hierarchy.
/// Unstructured edges live in three carriers: BlockGoto nodes, BlockMultiGoto
/// edge lists, and BlockIf's optional goto target (`if (...) goto L`).  Edges
/// typed break/continue are structured surrogates, not gotos.
static void kunaCountGotos(const FlowBlock *bl,int4 &gotoNodes,int4 &printedGotos,int4 &multiEdges,int4 &ifGotos)

{
  FlowBlock::block_type bt = bl->getType();
  if (bt == FlowBlock::t_goto) {
    gotoNodes += 1;
    if (((const BlockGoto *)bl)->gotoPrints())
      printedGotos += 1;
  }
  else if (bt == FlowBlock::t_multigoto)
    multiEdges += ((const BlockMultiGoto *)bl)->numGotos();
  else if (bt == FlowBlock::t_if) {
    const BlockIf *ifbl = (const BlockIf *)bl;
    if (ifbl->getGotoTarget() != (FlowBlock *)0 && ifbl->getGotoType() == FlowBlock::f_goto_goto)
      ifGotos += 1;
  }
  const BlockGraph *graph = dynamic_cast<const BlockGraph *>(bl);
  if (graph != (const BlockGraph *)0) {
    for(int4 i=0;i<graph->getSize();++i)
      kunaCountGotos(graph->getBlock(i),gotoNodes,printedGotos,multiEdges,ifGotos);
  }
}

/// \class IfcKunaQuality
/// \brief Report the goto-count structure-quality metric for the current function
void IfcKunaQuality::execute(istream &s)

{
  if (dcp->fd == (Funcdata *)0)
    throw IfaceExecutionError("No function selected");
  ostream &os( *status->fileoptr );	// bulk stream: assertable from datatests
  if (dcp->fd->hasNoStructBlocks()) {
    os << "No structured blocks for " << dcp->fd->getName() << " (decompile first)" << endl;
    return;
  }
  int4 gotoNodes = 0;
  int4 printedGotos = 0;
  int4 multiEdges = 0;
  int4 ifGotos = 0;
  kunaCountGotos(&dcp->fd->getStructure(),gotoNodes,printedGotos,multiEdges,ifGotos);
  int4 basicBlocks = dcp->fd->getBasicBlocks().getSize();
  os << "Structure quality for " << dcp->fd->getName() << ':' << endl;
  os << "  basic blocks: " << dec << basicBlocks << endl;
  os << "  goto nodes: " << gotoNodes << " (printed: " << printedGotos << ")" << endl;
  os << "  multi-goto edges: " << multiEdges << endl;
  os << "  if-goto edges: " << ifGotos << endl;
  os << "  unstructured total: " << (gotoNodes + multiEdges + ifGotos) << endl;
}

} // End namespace ghidra

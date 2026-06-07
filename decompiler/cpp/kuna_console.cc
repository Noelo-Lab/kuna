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
  status->registerCom(new IfcKunaAssert(),"kassert");
  status->registerCom(new IfcKunaRestarts(),"restarts");
}

/// \class IfcKunaStageList
/// \brief List the stage model: stages, Band B, and the sub-stage decision catalog
///
/// Pure static data from the registry (kuna_stages.cc); requires no program.
void IfcKunaStageList::execute(istream &s)

{
  ostream &os( *status->fileoptr );
  os << "Stages (kuna stage model, STAGES.md / docs/stage-model.md):" << endl;
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

} // End namespace ghidra

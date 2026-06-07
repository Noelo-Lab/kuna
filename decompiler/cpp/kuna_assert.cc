/* ###
 * kuna-owned: the typed assertion API (see kuna_assert.hh).
 */
#include "kuna_assert.hh"
#include "grammar.hh"

namespace ghidra {

/// The session log of applied assertions (console-lifetime; survives
/// function clears the same way the Override store does).
static vector<KunaAssertion> assertLog;

/// Read the rest of the stream as whitespace-separated tokens
static void tokenizeRest(istream &s,vector<string> &tokens)

{
  s >> ws;
  while(!s.eof()) {
    string word;
    s >> word >> ws;
    if (word.empty()) break;
    tokens.push_back(word);
  }
}

/// Re-join tokens [start,end) into a single space-separated string
static string joinTokens(const vector<string> &tokens,int4 start,int4 end)

{
  string res;
  for(int4 i=start;i<end;++i) {
    if (i != start) res += ' ';
    res += tokens[i];
  }
  return res;
}

void IfcKunaAssert::applyOption(const string &optname,const string &p1,
				const string &p2,const string &p3)
{
  string res = dcp->conf->options->set(ElementId::find(optname,0),p1,p2,p3);
  *status->optr << res << endl;
}

/// \class IfcKunaAssert
/// \brief Write a typed assertion: `kassert <stage> <substage> <args...> [hard|hint]`
///
/// Validates the stage/sub-stage pair against the registry catalog, then
/// dispatches to the existing store for that sub-stage.  Records the
/// assertion (with its computed minimal rewind stage) in the session log.
void IfcKunaAssert::execute(istream &s)

{
  if (dcp->conf == (Architecture *)0)
    throw IfaceExecutionError("No load image present");
  string stagecode;
  s >> ws >> stagecode;
  if (stagecode == "list") {
    listAssertions();
    return;
  }
  KunaStage stage;
  if (!kunaStageFromCode(stagecode,stage))
    throw IfaceParseError("Bad stage code (expecting P0 or S1..S9): "+stagecode);
  string subname;
  s >> ws >> subname;
  if (subname.empty())
    throw IfaceParseError("Missing sub-stage name (see `stage list`)");
  const KunaSubStage *sub = kunaLookupSubStage(subname);
  if (sub == (const KunaSubStage *)0)
    throw IfaceParseError("Unknown sub-stage: "+subname+" (see `stage list`)");
  if (sub->stage != stage)
    throw IfaceParseError("Sub-stage "+subname+" belongs to stage "+
			  string(kunaStageCode(sub->stage))+", not "+stagecode);

  vector<string> tokens;
  tokenizeRest(s,tokens);

  KunaStrength requested = sub->strength;	// Default to the catalog strength
  if (!tokens.empty()) {			// Optional trailing hard|hint
    if (tokens.back() == "hard") { requested = kstrength_hard; tokens.pop_back(); }
    else if (tokens.back() == "hint") { requested = kstrength_hint; tokens.pop_back(); }
  }

  KunaAssertion rec;
  rec.funcName = (dcp->fd != (Funcdata *)0) ? dcp->fd->getName() : "(global)";
  rec.stage = stage;
  rec.substage = subname;
  rec.args = joinTokens(tokens,0,tokens.size());
  rec.strength = requested;
  rec.applied = sub->strength;	// Underlying Ghidra stores are uniformly HARD mechanisms
  rec.rewind = sub->rewind;

  // Dispatch table: sub-stage -> existing store.  Function-scoped stores
  // require a current function, exactly like the legacy commands they wrap.
  string sname(subname);
  istringstream rest(rec.args);
  int4 discard;
  if (sname == "edge-virtualization") {		// -> Override::insertForceGoto
    if (dcp->fd == (Funcdata *)0)
      throw IfaceExecutionError("No function selected");
    if (tokens.size() != 2)
      throw IfaceParseError("kassert S7 edge-virtualization <branch-addr> <dest-addr>");
    rest >> ws;
    Address target(parse_machaddr(rest,discard,*dcp->conf->types));
    rest >> ws;
    Address dest(parse_machaddr(rest,discard,*dcp->conf->types));
    dcp->fd->getOverride().insertForceGoto(target,dest);
  }
  else if (sname == "dead-definition-gate") {	// -> Override::insertDeadcodeDelay
    if (dcp->fd == (Funcdata *)0)
      throw IfaceExecutionError("No function selected");
    if (tokens.size() != 2)
      throw IfaceParseError("kassert S3 dead-definition-gate <space> <delay>");
    AddrSpace *spc = dcp->conf->getSpaceByName(tokens[0]);
    if (spc == (AddrSpace *)0)
      throw IfaceParseError("Bad space: "+tokens[0]);
    istringstream ds(tokens[1]);
    int4 delay = -1;
    ds >> delay;
    if (delay < 0)
      throw IfaceParseError("Need non-negative delay integer");
    dcp->fd->getOverride().insertDeadcodeDelay(spc,delay);
  }
  else if (sname == "switch-model") {		// -> Override::insertMultistageJump
    if (dcp->fd == (Funcdata *)0)
      throw IfaceExecutionError("No function selected");
    if (tokens.size() != 1)
      throw IfaceParseError("kassert S2 switch-model <branchind-addr>");
    rest >> ws;
    Address addr(parse_machaddr(rest,discard,*dcp->conf->types));
    dcp->fd->getOverride().insertMultistageJump(addr);
  }
  else if (sname == "flow-classification") {	// -> Override::insertFlowOverride
    if (dcp->fd == (Funcdata *)0)
      throw IfaceExecutionError("No function selected");
    if (tokens.size() != 2)
      throw IfaceParseError("kassert S2 flow-classification <addr> <branch|call|callreturn|branchind|return>");
    rest >> ws;
    Address addr(parse_machaddr(rest,discard,*dcp->conf->types));
    uint4 type = Override::stringToType(tokens[1]);
    if (type == Override::NONE)
      throw IfaceParseError("Bad flow override type: "+tokens[1]);
    dcp->fd->getOverride().insertFlowOverride(addr,type);
  }
  else if (sname == "prototype-source") {	// -> FuncProto input/output locks
    if (dcp->fd == (Funcdata *)0)
      throw IfaceExecutionError("No function selected");
    if (tokens.size() != 1 || (tokens[0] != "lock" && tokens[0] != "unlock"))
      throw IfaceParseError("kassert S4 prototype-source <lock|unlock>");
    bool val = (tokens[0] == "lock");
    dcp->fd->getFuncProto().setInputLock(val);
    dcp->fd->getFuncProto().setOutputLock(val);
  }
  else if (sname == "type-propagation") {	// -> retype + typelock
    if (dcp->fd == (Funcdata *)0)
      throw IfaceExecutionError("No function selected");
    if (tokens.size() < 2)
      throw IfaceParseError("kassert S5 type-propagation <symbol> <type-declaration>");
    string symname = tokens[0];
    string newname;
    istringstream ts(joinTokens(tokens,1,tokens.size())+" "+symname);
    Datatype *ct = parse_type(ts,newname,dcp->conf);
    vector<Symbol *> symList;
    dcp->readSymbol(symname,symList);
    if (symList.empty())
      throw IfaceExecutionError("No symbol named: "+symname);
    if (symList.size() > 1)
      throw IfaceExecutionError("More than one symbol named: "+symname);
    Symbol *sym = symList[0];
    if (sym->getCategory() == Symbol::function_parameter)
      dcp->fd->getFuncProto().setInputLock(true);
    sym->getScope()->retypeSymbol(sym,ct);
    sym->getScope()->setAttribute(sym,Varnode::typelock);
  }
  else if (sname == "merge-aggressiveness") {	// -> Symbol::setIsolated
    if (dcp->fd == (Funcdata *)0)
      throw IfaceExecutionError("No function selected");
    if (tokens.size() != 1)
      throw IfaceParseError("kassert S6 merge-aggressiveness <symbol>");
    vector<Symbol *> symList;
    dcp->readSymbol(tokens[0],symList);
    if (symList.empty())
      throw IfaceExecutionError("No symbol named: "+tokens[0]);
    if (symList.size() > 1)
      throw IfaceExecutionError("More than one symbol named: "+tokens[0]);
    symList[0]->setIsolated(true);
  }
  else if (sname == "naming-policy") {		// -> rename + namelock
    if (dcp->fd == (Funcdata *)0)
      throw IfaceExecutionError("No function selected");
    if (tokens.size() != 2)
      throw IfaceParseError("kassert S9 naming-policy <oldname> <newname>");
    vector<Symbol *> symList;
    dcp->readSymbol(tokens[0],symList);
    if (symList.empty())
      throw IfaceExecutionError("No symbol named: "+tokens[0]);
    if (symList.size() > 1)
      throw IfaceExecutionError("More than one symbol named: "+tokens[0]);
    Symbol *sym = symList[0];
    sym->getScope()->renameSymbol(sym,tokens[1]);
    sym->getScope()->setAttribute(sym,Varnode::namelock);
  }
  else if (sname == "comparison-canonicalization") {	// -> option compareform
    if (tokens.size() != 1)
      throw IfaceParseError("kassert S3 comparison-canonicalization <canonical|original>");
    applyOption("compareform",tokens[0],"","");
  }
  else if (sname == "pointer-notation") {	// -> option arraynotation
    if (tokens.size() != 1)
      throw IfaceParseError("kassert S9 pointer-notation <on|off>");
    applyOption("arraynotation",tokens[0],"","");
  }
  else if (sname == "simplification-quiescence") {	// -> option togglerule
    if (tokens.size() != 2)
      throw IfaceParseError("kassert S3 simplification-quiescence <rulename> <on|off>");
    applyOption("togglerule",tokens[0],tokens[1],"");
  }
  else if (sname == "readability-rewrites") {	// -> option analyzeforloops
    if (tokens.size() != 1)
      throw IfaceParseError("kassert S8 readability-rewrites <on|off>");
    applyOption("analyzeforloops",tokens[0],"","");
  }
  else if (sname == "extrapop") {		// -> option extrapop
    if (tokens.size() < 1 || tokens.size() > 2)
      throw IfaceParseError("kassert S4 extrapop <value> [<function>]");
    applyOption("extrapop",tokens[0],(tokens.size()>1) ? tokens[1] : "","");
  }
  else if (sname == "pipeline-variant") {	// -> option setaction
    if (tokens.size() < 1 || tokens.size() > 2)
      throw IfaceParseError("kassert P0 pipeline-variant <root-action> [<group>]");
    applyOption("setaction",tokens[0],(tokens.size()>1) ? tokens[1] : "","");
  }
  else {
    if (sub->latent)
      throw IfaceExecutionError("Sub-stage "+sname+" is LATENT: no override surface exists yet (kuna roadmap)");
    throw IfaceExecutionError("Sub-stage "+sname+" is not yet routable through kassert; use its native surface: "+sub->exposure);
  }

  assertLog.push_back(rec);
  *status->optr << "kassert applied: [" << kunaStageCode(rec.stage) << "] " << rec.substage;
  if (!rec.args.empty())
    *status->optr << ' ' << rec.args;
  *status->optr << "  rewind->" << kunaStageCode(rec.rewind)
		<< " (Ghidra-actual: whole-function)" << endl;
}

/// Dump the session's typed assertions: `kassert list`
void IfcKunaAssert::listAssertions(void)

{
  ostream &os( *status->fileoptr );	// bulk stream: assertable from datatests
  if (assertLog.empty()) {
    os << "No kuna assertions recorded this session" << endl;
    return;
  }
  os << "Recorded assertions (rewind scope is REPORTED per stage-model.md s12; "
     << "Ghidra-actual re-run remains whole-function):" << endl;
  for(int4 i=0;i<(int4)assertLog.size();++i) {
    const KunaAssertion &rec( assertLog[i] );
    os << "  " << dec << i << ": [" << rec.funcName << "] "
       << kunaStageCode(rec.stage) << '/' << rec.substage;
    if (!rec.args.empty())
      os << ' ' << rec.args;
    os << "  strength=" << ((rec.strength == kstrength_hard) ? "HARD" :
			    (rec.strength == kstrength_hint) ? "HINT" : "-");
    if (rec.strength == kstrength_hint && rec.applied == kstrength_hard)
      os << " (applied via HARD mechanism)";
    os << "  rewind->" << kunaStageCode(rec.rewind) << endl;
  }
}

} // End namespace ghidra

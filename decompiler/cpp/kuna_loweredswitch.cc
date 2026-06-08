/* ###
 * kuna-owned: lowered comparison-cascade -> switch recovery (see kuna_loweredswitch.hh).
 *
 * Port of angr's LoweredSwitchSimplifier.  Detection runs late on the simplified
 * CFG; installation runs pre-SSA on a restart, mirroring Ghidra's multistage
 * jump-table restart idiom.  The CFG surgery (Funcdata::kunaInstallLoweredSwitch)
 * lives here but accesses Funcdata's private bblocks/jumpvec as a member function
 * (declared in funcdata.hh, the single anchor edit) -- the same arrangement as
 * JumpBasic::kunaTryModuloBoundTable in jumptable.cc.
 */
#include "kuna_loweredswitch.hh"
#include "kuna_restartlog.hh"
#include "funcdata.hh"
#include "architecture.hh"

#include <map>

namespace ghidra {

// kuna ElementIds live in the 4000+ range (see kuna_returnpair.cc note).
ElementId ELEM_LOWEREDSWITCH = ElementId("loweredswitch",4019);

//===========================================================================
// Side table: the recovered lowered-switch hint, keyed by function identity.
//
// Like the Override store (and kuna_restartlog), this survives Funcdata::clear()
// across a restart and across re-decompilation: it holds only addresses and
// constants, never Varnode/Block/Funcdata pointers.  It is a STICKY hint -- once
// a cascade is discovered (late, on the simplified CFG), every subsequent
// decompilation of that function re-applies it pre-SSA without needing another
// restart, exactly like a manual jump-table override.
//===========================================================================

/// One recovered lowered switch within a function
struct KunaLoweredSwitchRecord {
  Address branchAddr;			///< Op address of the cascade head (hosts the synthetic BRANCHIND)
  Address varAddr;			///< Switch variable storage (a fallback locator for V)
  int4 varSize;				///< Switch variable size in bytes
  vector<uintb> caseVals;		///< Case label constants (parallel to caseTargets)
  vector<Address> caseTargets;		///< Case body block start addresses
  Address defaultTarget;		///< Default block start address
};

/// Side-table key: identifies a function without holding clearable pointers
/// (identical shape to kuna_restartlog's KunaFuncKey).
struct KunaLSKey {
  const void *glb;
  int4 spaceindex;
  uintb offset;
  bool operator<(const KunaLSKey &op2) const {
    if (glb != op2.glb) return (glb < op2.glb);
    if (spaceindex != op2.spaceindex) return (spaceindex < op2.spaceindex);
    return (offset < op2.offset);
  }
};

static std::map<KunaLSKey,std::vector<KunaLoweredSwitchRecord> > loweredStore;

static KunaLSKey keyForFunc(const Funcdata &fd)

{
  KunaLSKey key;
  key.glb = (const void *)fd.getArch();
  const Address &entry( fd.getAddress() );
  key.spaceindex = (entry.getSpace() != (AddrSpace *)0) ? entry.getSpace()->getIndex() : -1;
  key.offset = entry.getOffset();
  return key;
}

bool kunaLoweredSwitchHasRecord(const Funcdata &fd)

{
  std::map<KunaLSKey,std::vector<KunaLoweredSwitchRecord> >::const_iterator iter;
  iter = loweredStore.find(keyForFunc(fd));
  return (iter != loweredStore.end() && !(*iter).second.empty());
}

//===========================================================================
// Detection (read-only) on the simplified CFG
//===========================================================================

/// \brief Peel transparent ops to a canonical switch-variable Varnode
///
/// COPY/CAST/zero-extend/sign-extend/zero-offset SUBPIECE all preserve switch
/// variable identity (angr's StableVarExprHasher analog).  HighVariable identity
/// (post-merge) is the strongest equivalence; this just normalizes the leaf.
static Varnode *canonSwitchVar(Varnode *vn)

{
  for(int4 guard=0;guard<8;++guard) {
    if (vn->isConstant()) return vn;
    if (!vn->isWritten()) return vn;
    PcodeOp *def = vn->getDef();
    OpCode oc = def->code();
    if (oc==CPUI_COPY || oc==CPUI_CAST || oc==CPUI_INT_ZEXT || oc==CPUI_INT_SEXT) {
      vn = def->getIn(0);
      continue;
    }
    if (oc==CPUI_SUBPIECE && def->getIn(1)->isConstant() && def->getIn(1)->getOffset()==0) {
      vn = def->getIn(0);
      continue;
    }
    return vn;
  }
  return vn;
}

/// \brief Analysis of one comparison-on-constant block
///
/// "Same switch variable" identity is the canonicalized Varnode pointer.  This
/// runs in the fullloop, BEFORE merge/ActionAssignHigh, so HighVariables do not
/// exist yet; the cascade reads one SSA Varnode (the dispatch value is defined
/// once and not redefined between comparisons), so pointer identity is exact.
struct CmpNode {
  bool valid;			///< True if this block is a clean comparison-on-constant
  bool isEquality;		///< True for INT_EQUAL/INT_NOTEQUAL (emits a case)
  Varnode *var;			///< The canonicalized variable operand (the identity key)
  uintb cval;			///< The constant operand value
  FlowBlock *matchOut;		///< Successor taken when V == cval (equality only)
  FlowBlock *contA;		///< Continuation successor A (range: both continue)
  FlowBlock *contB;		///< Continuation successor B (range only)
  CmpNode(void) { valid=false; isEquality=false; var=(Varnode *)0;
		  cval=0; matchOut=(FlowBlock *)0; contA=(FlowBlock *)0; contB=(FlowBlock *)0; }
};

/// \brief Is \e bb a "pure" comparison spine block (no side effects, no escaping defs)?
///
/// Only such blocks may be collapsed when the switch is installed.  Case bodies
/// (which set flags / call) and the default block fail this and so naturally
/// bound the cascade as leaves.
static bool isPureCompareBlock(BlockBasic *bb)

{
  list<PcodeOp *>::const_iterator iter;
  for(iter=bb->beginOp();iter!=bb->endOp();++iter) {
    OpCode oc = (*iter)->code();
    switch(oc) {
    case CPUI_CBRANCH:
    case CPUI_INT_EQUAL: case CPUI_INT_NOTEQUAL:
    case CPUI_INT_SLESS: case CPUI_INT_SLESSEQUAL:
    case CPUI_INT_LESS: case CPUI_INT_LESSEQUAL:
    case CPUI_INT_ZEXT: case CPUI_INT_SEXT: case CPUI_SUBPIECE:
    case CPUI_COPY: case CPUI_CAST: case CPUI_MULTIEQUAL:
    case CPUI_INT_AND: case CPUI_BOOL_NEGATE:
      break;
    default:
      return false;
    }
  }
  return true;
}

/// \brief Decode a block as a comparison-on-constant cascade node
static CmpNode analyzeCmp(BlockBasic *bb)

{
  CmpNode res;
  PcodeOp *cb = bb->lastOp();
  if (cb == (PcodeOp *)0 || cb->code() != CPUI_CBRANCH) return res;
  if (bb->sizeOut() != 2) return res;
  Varnode *boolvn = cb->getIn(1);
  if (!boolvn->isWritten()) return res;
  PcodeOp *cmp = boolvn->getDef();
  OpCode oc = cmp->code();
  // Only binary integer comparisons (all have two inputs) can be cascade nodes.
  if (oc!=CPUI_INT_EQUAL && oc!=CPUI_INT_NOTEQUAL && oc!=CPUI_INT_SLESS &&
      oc!=CPUI_INT_SLESSEQUAL && oc!=CPUI_INT_LESS && oc!=CPUI_INT_LESSEQUAL)
    return res;
  Varnode *v0 = cmp->getIn(0);
  Varnode *v1 = cmp->getIn(1);
  Varnode *var;
  uintb cval;
  if (v1->isConstant() && !v0->isConstant()) { var = v0; cval = v1->getOffset(); }
  else if (v0->isConstant() && !v1->isConstant()) { var = v1; cval = v0->getOffset(); }
  else return res;

  bool flip = cb->isBooleanFlip();
  FlowBlock *condTrue = flip ? bb->getFalseOut() : bb->getTrueOut();
  FlowBlock *condFalse = flip ? bb->getTrueOut() : bb->getFalseOut();

  res.var = canonSwitchVar(var);

  if (oc == CPUI_INT_EQUAL) {
    res.valid = true; res.isEquality = true; res.cval = cval;
    res.matchOut = condTrue; res.contA = condFalse;
  }
  else if (oc == CPUI_INT_NOTEQUAL) {
    res.valid = true; res.isEquality = true; res.cval = cval;
    res.matchOut = condFalse; res.contA = condTrue;	// match (V==cval) is the not-taken edge
  }
  else if (oc==CPUI_INT_SLESS || oc==CPUI_INT_SLESSEQUAL || oc==CPUI_INT_LESS || oc==CPUI_INT_LESSEQUAL) {
    res.valid = true; res.isEquality = false; res.cval = cval;
    res.contA = condTrue; res.contB = condFalse;	// range: both directions continue
  }
  return res;
}

/// \brief Recover the entry start address of a target basic block
///
/// At the bblocks level the block is a BlockBasic (t_basic), so getStart()
/// dispatches to its cover's first address (getFrontLeaf is for sblocks).
static Address targetStart(FlowBlock *fb)

{
  return fb->getStart();
}

/// \brief Try to recover one lowered switch with \e startbb as the cascade head
///
/// Walks the comparison tree, collecting equality cases and the common default.
/// Returns true and fills \e rec on success.
static bool recoverCascade(Funcdata &data,BlockBasic *startbb,
			   const std::map<BlockBasic *,CmpNode> &cmpmap,
			   Varnode *swvar,KunaLoweredSwitchRecord &rec)

{
  // DFS over cascade nodes reading swvar; collect cases + default votes.
  std::map<uintb,Address> cases;		// value -> target (distinct values)
  std::map<Address,int4> defaultVotes;		// non-cascade fall-through sinks
  std::set<BlockBasic *> visited;
  std::vector<BlockBasic *> stack;
  stack.push_back(startbb);
  bool sawRange = false;		// true once a range (binary-search) node is seen

  auto isCascade = [&](FlowBlock *fb)->BlockBasic * {
    if (fb == (FlowBlock *)0) return (BlockBasic *)0;
    BlockBasic *b = dynamic_cast<BlockBasic *>(fb);
    if (b == (BlockBasic *)0) return (BlockBasic *)0;
    std::map<BlockBasic *,CmpNode>::const_iterator it = cmpmap.find(b);
    if (it == cmpmap.end()) return (BlockBasic *)0;
    if ((*it).second.var != swvar) return (BlockBasic *)0;
    return b;
  };

  while(!stack.empty()) {
    BlockBasic *bb = stack.back();
    stack.pop_back();
    if (visited.count(bb)) continue;
    visited.insert(bb);
    const CmpNode &cn( (*cmpmap.find(bb)).second );

    if (cn.isEquality) {
      BlockBasic *matchb = isCascade(cn.matchOut);
      if (matchb != (BlockBasic *)0) {
	// match edge leads to another comparison: rare; treat as continuation
	stack.push_back(matchb);
      }
      else {
	Address tgt = targetStart(cn.matchOut);
	if (cases.find(cn.cval) == cases.end())
	  cases[cn.cval] = tgt;
	else
	  return false;		// duplicate case value: bail (v1)
      }
      BlockBasic *contb = isCascade(cn.contA);
      if (contb != (BlockBasic *)0) stack.push_back(contb);
      else defaultVotes[targetStart(cn.contA)] += 1;
    }
    else {				// range node: both edges continue or fall to default
      sawRange = true;
      FlowBlock *outs[2] = { cn.contA, cn.contB };
      for(int4 i=0;i<2;++i) {
	BlockBasic *cb = isCascade(outs[i]);
	if (cb != (BlockBasic *)0) stack.push_back(cb);
	else defaultVotes[targetStart(outs[i])] += 1;
      }
    }
  }

  if (cases.size() < 3) return false;		// RULE3: need >= 3 cases
  // Fail-safe cap: a very large synthesized switch can make Ghidra's structurer
  // mark some case edges as unstructured gotos (a t_multigoto switch block) and
  // then abort with "Case block has become detached from switch", failing the
  // whole function instead of falling back to the if-chain.  All angr SAILR
  // lowered-switch examples are <= 11 cases; cap conservatively so an oversized
  // sparse tree (e.g. stat's 68-case human_fstype) is left as an if-chain rather
  // than regressing the function.  (Raising this needs structurer robustness work.)
  if (cases.size() > 16) return false;
  if (defaultVotes.empty()) return false;
  // Require the GCC binary-search structure (a range/jump-tree split).  A purely
  // linear equality chain is a hand-written if/else-if, not a lowered switch, so
  // leave it alone -- this is what keeps the default-on pass off upstream's
  // elseif/copytrim/partialunion comparison chains.
  if (!sawRange) return false;

  // Default = most-voted common sink.
  Address defAddr;
  int4 best = -1;
  std::map<Address,int4>::const_iterator dit;
  for(dit=defaultVotes.begin();dit!=defaultVotes.end();++dit) {
    if ((*dit).second > best) { best = (*dit).second; defAddr = (*dit).first; }
  }

  // Distinct targets, and no case target collides with the default.
  std::set<Address> tgtset;
  std::map<uintb,Address>::const_iterator cit;
  for(cit=cases.begin();cit!=cases.end();++cit) {
    if ((*cit).second == defAddr) return false;	// a case can't be the default block
    tgtset.insert((*cit).second);
  }
  if (tgtset.size() < 2) return false;		// RULE3: >= 2 distinct targets

  if (swvar == (Varnode *)0) return false;
  // The switch variable must live in stable storage (register/stack), so it can
  // be relocated by storage on the restart's pre-SSA CFG.
  AddrSpace *vspc = swvar->getAddr().getSpace();
  if (vspc == (AddrSpace *)0) return false;
  spacetype st = vspc->getType();
  if (st != IPTR_PROCESSOR && st != IPTR_SPACEBASE) return false;

  rec.branchAddr = startbb->lastOp()->getAddr();
  rec.varAddr = swvar->getAddr();
  rec.varSize = swvar->getSize();
  rec.defaultTarget = defAddr;
  rec.caseVals.clear();
  rec.caseTargets.clear();
  for(cit=cases.begin();cit!=cases.end();++cit) {
    rec.caseVals.push_back((*cit).first);
    rec.caseTargets.push_back((*cit).second);
  }
  return true;
}

/// \brief Find the cascade head, skipping leading sentinel guards (e.g. `V == -1`)
///
/// A getopt-style loop tests the EOF sentinel (-1) before the real dispatch; that
/// equality must stay an ordinary branch (its match target is the loop exit, not
/// a case), so we advance the head past any leading `V == all-ones` guard.
static BlockBasic *advancePastGuards(BlockBasic *head,const std::map<BlockBasic *,CmpNode> &cmpmap,
				     Varnode *swvar)

{
  for(int4 guard=0;guard<8;++guard) {
    std::map<BlockBasic *,CmpNode>::const_iterator it = cmpmap.find(head);
    if (it == cmpmap.end()) break;
    const CmpNode &cn( (*it).second );
    if (!cn.isEquality) break;
    uintb allones = calc_mask(cn.var->getSize());
    if (cn.cval != allones) break;		// not a -1 sentinel guard
    BlockBasic *cont = dynamic_cast<BlockBasic *>(cn.contA);
    if (cont == (BlockBasic *)0) break;
    std::map<BlockBasic *,CmpNode>::const_iterator cit = cmpmap.find(cont);
    if (cit == cmpmap.end() || (*cit).second.var != swvar) break;
    head = cont;				// the -1 guard is not part of the switch
  }
  return head;
}

int4 ActionLowerSwitchDetect::apply(Funcdata &data)

{
  if (!data.getArch()->recover_lowered_switch) return 0;	// P0 assertion not set
  if (data.isJumptableRecoveryOn()) return 0;			// not inside partial-fn recovery
  if (kunaLoweredSwitchHasRecord(data)) return 0;		// already discovered (sticky)

  // Collect comparison-on-constant cascade nodes, grouped by switch variable
  // (the canonicalized Varnode pointer; pre-merge so HighVariables don't exist yet).
  std::map<BlockBasic *,CmpNode> cmpmap;
  std::map<Varnode *,int4> varCount;
  const BlockGraph &bb( data.getBasicBlocks() );
  for(int4 i=0;i<bb.getSize();++i) {
    BlockBasic *b = dynamic_cast<BlockBasic *>(bb.getBlock(i));
    if (b == (BlockBasic *)0) continue;
    if (b->isSwitchOut()) continue;		// already a real switch
    if (!isPureCompareBlock(b)) continue;
    CmpNode cn = analyzeCmp(b);
    if (!cn.valid || cn.var == (Varnode *)0) continue;
    cmpmap[b] = cn;
    varCount[cn.var] += 1;
  }

  // Pick the switch variable: the Varnode with the most cascade nodes.
  Varnode *swvar = (Varnode *)0;
  int4 bestCount = 1;
  std::map<Varnode *,int4>::const_iterator hit;
  for(hit=varCount.begin();hit!=varCount.end();++hit) {
    if ((*hit).second > bestCount) { bestCount = (*hit).second; swvar = (*hit).first; }
  }
  if (swvar == (Varnode *)0) return 0;

  // Find the head: a cascade node for swvar that no other cascade node continues into.
  std::set<BlockBasic *> isChild;
  std::map<BlockBasic *,CmpNode>::const_iterator mit;
  for(mit=cmpmap.begin();mit!=cmpmap.end();++mit) {
    if ((*mit).second.var != swvar) continue;
    const CmpNode &cn( (*mit).second );
    FlowBlock *succ[3] = { cn.matchOut, cn.contA, cn.contB };
    for(int4 i=0;i<3;++i) {
      BlockBasic *s = dynamic_cast<BlockBasic *>(succ[i]);
      if (s == (BlockBasic *)0) continue;
      std::map<BlockBasic *,CmpNode>::const_iterator sit = cmpmap.find(s);
      if (sit != cmpmap.end() && (*sit).second.var == swvar)
	isChild.insert(s);
    }
  }
  BlockBasic *head = (BlockBasic *)0;
  for(mit=cmpmap.begin();mit!=cmpmap.end();++mit) {
    if ((*mit).second.var != swvar) continue;
    if (isChild.count((*mit).first)) continue;
    head = (*mit).first;
    break;
  }
  if (head == (BlockBasic *)0) return 0;
  head = advancePastGuards(head,cmpmap,swvar);

  KunaLoweredSwitchRecord rec;
  if (!recoverCascade(data,head,cmpmap,swvar,rec)) return 0;

  loweredStore[keyForFunc(data)].push_back(rec);
  data.setRestartPending(true);
  kunaRecordRestart(data,krestart_lowered_switch,rec.branchAddr);
  return 0;		// no op change; let the fullloop quiesce and the restart fire
}

/// \brief (kuna) Attach a no-op non-override model to a synthesized JumpTable
///
/// Defined here (a JumpTable member, for private jmodel access) rather than in
/// jumptable.cc to keep the kuna logic in one file.  JumpModelTrivial::foldInGuards
/// returns false and isOverride() returns false, so the table is cleanly deleted by
/// clearJumpTables() on any restart (unlike an override, which would persist stale).
void JumpTable::kunaSetTrivialModel(void)

{
  if (jmodel != (JumpModel *)0)
    delete jmodel;
  jmodel = new JumpModelTrivial(this);
}

int4 ActionLowerSwitchInstall::apply(Funcdata &data)

{
  if (!data.getArch()->recover_lowered_switch) return 0;
  if (data.getHeritagePass() != 0) return 0;	// only in the pre-SSA window (first mainloop iteration)
  std::map<KunaLSKey,std::vector<KunaLoweredSwitchRecord> >::iterator iter;
  iter = loweredStore.find(keyForFunc(data));
  if (iter == loweredStore.end() || (*iter).second.empty()) return 0;

  int4 changed = 0;
  const std::vector<KunaLoweredSwitchRecord> &recs( (*iter).second );
  for(int4 i=0;i<(int4)recs.size();++i) {
    const KunaLoweredSwitchRecord &r( recs[i] );
    JumpTable *jt = data.kunaInstallLoweredSwitch(r.branchAddr,r.varAddr,r.varSize,
						  r.caseVals,r.caseTargets,r.defaultTarget);
    if (jt != (JumpTable *)0)
      changed += 1;
  }
  return changed;	// nonzero => mainloop re-iterates; ActionHeritage rebuilds SSA over the new CFG
}

//===========================================================================
// The CFG surgery (a Funcdata member for private bblocks/jumpvec access; declared
// in funcdata.hh -- the single anchor edit).  Runs pre-SSA on the restart pass.
//===========================================================================

/// \brief (kuna) Manufacture a BRANCHIND + labelled JumpTable from a recovered cascade
///
/// Pre-SSA (no MULTIEQUALs exist), so the edge surgery needs no phi patching.
/// The head block's CBRANCH is replaced by a BRANCHIND on the switch variable,
/// its out-edges are rewired to exactly the case targets + default, and a fully
/// labelled JumpTable is built (so ActionSwitchNorm skips it).  The now-orphaned
/// cascade spine is left for removeUnreachableBlocks to collect.
/// \return the new JumpTable, or null if the recorded shape no longer matches
JumpTable *Funcdata::kunaInstallLoweredSwitch(const Address &branchAddr,const Address &varAddr,int4 varSize,
					      const vector<uintb> &caseVals,const vector<Address> &caseTargets,
					      const Address &defaultTarget)

{
  // Locate the head block (terminal op at branchAddr).
  BlockBasic *head = (BlockBasic *)0;
  for(int4 i=0;i<bblocks.getSize();++i) {
    BlockBasic *b = (BlockBasic *)bblocks.getBlock(i);
    PcodeOp *lo = b->lastOp();
    if (lo != (PcodeOp *)0 && lo->getAddr() == branchAddr) { head = b; break; }
  }
  if (head == (BlockBasic *)0) return (JumpTable *)0;
  PcodeOp *oldterm = head->lastOp();
  if (oldterm->code() == CPUI_BRANCHIND) return (JumpTable *)0;	// already installed this invocation
  if (oldterm->code() != CPUI_CBRANCH) return (JumpTable *)0;

  // Locate case + default target blocks by start address.
  vector<BlockBasic *> targets;
  for(int4 i=0;i<(int4)caseTargets.size();++i) {
    BlockBasic *t = (BlockBasic *)0;
    for(int4 j=0;j<bblocks.getSize();++j) {
      BlockBasic *b = (BlockBasic *)bblocks.getBlock(j);
      if (b->getStart() == caseTargets[i]) { t = b; break; }
    }
    if (t == (BlockBasic *)0) return (JumpTable *)0;
    targets.push_back(t);
  }
  BlockBasic *defBlock = (BlockBasic *)0;
  for(int4 j=0;j<bblocks.getSize();++j) {
    BlockBasic *b = (BlockBasic *)bblocks.getBlock(j);
    if (b->getStart() == defaultTarget) { defBlock = b; break; }
  }
  if (defBlock == (BlockBasic *)0) return (JumpTable *)0;

  // Build the synthetic switch-variable read; heritage resolves its reaching def.
  Varnode *swvn = newVarnode(varSize,varAddr);

  // Replace the head's terminal CBRANCH with a BRANCHIND(V).
  PcodeOp *bind = newOp(1,branchAddr);
  opSetOpcode(bind,CPUI_BRANCHIND);
  opSetInput(bind,swvn,0);
  opDestroy(oldterm);			// the dead comparison feeding it dies as deadcode
  opInsertEnd(bind,head);
  head->setFlag(FlowBlock::f_switch_out);

  // Drop the head's stale out-edges (the case-c0 edge and the cascade spine edge).
  vector<FlowBlock *> oldouts;
  for(int4 i=0;i<head->sizeOut();++i) oldouts.push_back(head->getOut(i));
  for(int4 i=0;i<(int4)oldouts.size();++i) bblocks.removeEdge(head,oldouts[i]);

  // Build the JumpTable, lock-stepping addBlockToSwitch with edge creation so the
  // block2addr out-edge positions stay in sync (the foldInOneGuard convention).
  JumpTable *jt = new JumpTable(branchAddr);
  jt->setIndirectOp(bind);
  for(int4 i=0;i<(int4)targets.size();++i) {
    jt->addBlockToSwitch(targets[i],caseVals[i]);
    bblocks.addEdge(head,targets[i]);
  }
  jt->addBlockToSwitch(defBlock,JumpValues::NO_LABEL);
  jt->setLastAsDefault();
  bblocks.addEdge(head,defBlock);
  // Attach a no-op NON-override model: the table is already fully labelled, so its
  // recovery methods are never invoked, but ActionSwitchNorm unconditionally calls
  // jmodel->foldInGuards() (a null model crashes; JumpModelTrivial returns false).
  // It must NOT be an override -- clearJumpTables() preserves override tables across
  // a restart's clear(), and followFlow would then re-process this stale table.
  jt->kunaSetTrivialModel();
  jt->markComplete();
  jumpvec.push_back(jt);

  // Rebuild structuring over the new CFG and collect the now-orphaned cascade spine.
  structureReset();
  removeUnreachableBlocks(false,true);
  return jt;
}

string OptionLowerSwitch::apply(Architecture *glb,const string &p1,const string &p2,const string &p3) const

{
  bool val = onOrOff(p1);
  glb->recover_lowered_switch = val;
  string prop = val ? "on" : "off";
  return "Lowered comparison-cascade switch recovery turned "+prop;
}

} // End namespace ghidra

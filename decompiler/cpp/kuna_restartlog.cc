/* ###
 * kuna-owned: restart observability side table (see kuna_restartlog.hh).
 */
#include "kuna_restartlog.hh"
#include "funcdata.hh"

#include <map>

namespace ghidra {

/// One recorded trigger event
struct KunaRestartEvent {
  int4 seq;			///< Global sequence number (orders events across functions)
  KunaRestartReason reason;	///< Why
  std::string detail;		///< Site description
};

/// Side-table key: identifies a function without holding pointers into the
/// (restartable, clearable) Funcdata or the Address's space object.
struct KunaFuncKey {
  const void *glb;		///< The owning Architecture
  int4 spaceindex;		///< Entry address space index
  uintb offset;			///< Entry offset
  bool operator<(const KunaFuncKey &op2) const {
    if (glb != op2.glb) return (glb < op2.glb);
    if (spaceindex != op2.spaceindex) return (spaceindex < op2.spaceindex);
    return (offset < op2.offset);
  }
};

static std::map<KunaFuncKey,std::vector<KunaRestartEvent> > restartTable;
static int4 restartSeq = 0;
static const int4 MAX_EVENTS_PER_FUNC = 32;	///< Ring cap per function

static KunaFuncKey keyForFunc(const Funcdata &fd)

{
  KunaFuncKey key;
  key.glb = (const void *)fd.getArch();
  const Address &entry( fd.getAddress() );
  key.spaceindex = (entry.getSpace() != (AddrSpace *)0) ? entry.getSpace()->getIndex() : -1;
  key.offset = entry.getOffset();
  return key;
}

const char *kunaRestartReasonName(KunaRestartReason reason)

{
  switch(reason) {
  case krestart_deadcode_bump:
    return "deadcode-delay bump (S3 dead-definition gate, edge 3)";
  case krestart_deadcode_suppressed:
    return "deadcode-delay bump SUPPRESSED (delay already installed)";
  case krestart_multistage_jump:
    return "multistage jump-table hint (S2 switch model, edge 2)";
  case krestart_proto_deindirect:
    return "prototype discovered late at indirect call (S4, edge 5)";
  case krestart_proto_forced:
    return "prototype forced late at call site (S4, edge 5)";
  }
  return "unknown";
}

void kunaRecordRestart(const Funcdata &fd,KunaRestartReason reason,const std::string &detail)

{
  std::vector<KunaRestartEvent> &events( restartTable[keyForFunc(fd)] );
  if ((int4)events.size() >= MAX_EVENTS_PER_FUNC)
    return;			// Cap per function; earliest events are the diagnostic ones
  KunaRestartEvent ev;
  ev.seq = restartSeq++;
  ev.reason = reason;
  ev.detail = detail;
  events.push_back(ev);
}

void kunaRecordRestart(const Funcdata &fd,KunaRestartReason reason,const Address &site)

{
  std::ostringstream oss;
  site.printRaw(oss);
  kunaRecordRestart(fd,reason,oss.str());
}

void kunaDumpRestarts(std::ostream &os,const Funcdata &fd)

{
  std::map<KunaFuncKey,std::vector<KunaRestartEvent> >::const_iterator iter;
  iter = restartTable.find(keyForFunc(fd));
  if (iter == restartTable.end() || (*iter).second.empty()) {
    os << "No restart events recorded for " << fd.getName() << std::endl;
    return;
  }
  const std::vector<KunaRestartEvent> &events( (*iter).second );
  os << "Restart events for " << fd.getName() << " (mechanism c; hints persist in the Override store):" << std::endl;
  for(int4 i=0;i<(int4)events.size();++i) {
    os << "  " << std::dec << i << ": " << kunaRestartReasonName(events[i].reason);
    if (!events[i].detail.empty())
      os << "  [" << events[i].detail << "]";
    os << std::endl;
  }
}

} // End namespace ghidra

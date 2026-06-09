/* ###
 * kuna-owned: angr-style default naming policy.
 *
 * This file is NOT part of upstream Ghidra.  It centralizes the kuna default
 * naming scheme that makes decompiler output read like angr's:
 *
 *   - locals / temps / extraout / unaff  -> v1, v2, v3 ...
 *   - arguments                          -> a0, a1 ...   (by signature slot)
 *   - global data                        -> dat_<addr>   (lowercase, no leading 0s)
 *   - unnamed functions                  -> sub_<addr>
 *   - code labels                        -> label_<addr>
 *   - plus a `// rax` / `// stack - 0x10` source-location comment per local.
 *
 * It is a P0 surface-rendering policy (docs/stage-model.md): a single master
 * toggle `option namestyle angr|ghidra` (default `angr`) flips the whole
 * scheme; `ghidra` restores the upstream Ghidra naming.  The upstream naming
 * sites (database.cc, printc.cc, architecture.cc, fspec.cc) carry minimal
 * `(kuna)` anchor edits that branch on kunaAngrNaming().
 */
#ifndef __KUNA_NAMING_HH__
#define __KUNA_NAMING_HH__

#include "options.hh"

namespace ghidra {

class Architecture;
class Address;
class Symbol;

extern ElementId ELEM_NAMESTYLE;	///< Marshaling element \<namestyle> (kuna)

/// \brief (kuna) Master naming-scheme toggle: `namestyle angr|ghidra`
///
/// "angr" (default) renders angr-style names (vN/aN/dat_/sub_/label_) plus a
/// storage-location comment on each local declaration; "ghidra" restores the
/// upstream Ghidra naming (iVar/param_/<type>Ram/func_/code_).
class OptionNameStyle : public ArchOption {
public:
  OptionNameStyle(void) { name = "namestyle"; }	///< Constructor
  virtual string apply(Architecture *glb,const string &p1,const string &p2,const string &p3) const;
};

bool kunaAngrNaming(const Architecture *glb);		///< Is angr-style naming active for this Architecture?
string kunaGlobalDataName(const Address &addr);		///< "dat_<addr>" (lowercase hex, no leading zeros, no 0x)
string kunaFunctionName(const Address &addr);		///< "sub_<addr>" (same address format)
string kunaLabelName(const Address &addr);		///< "label_<addr>" (same address format)
string kunaArgName(int4 catindex);			///< "a<catindex>" (0-indexed argument name)
bool kunaIsGeneratedName(const string &nm);		///< True if \e nm is a generated default (param_N / vN / aN)
bool kunaStorageComment(const Architecture *glb,const Symbol *sym,string &out);	///< Build "rax" / "stack - 0x10" for a symbol's storage (false = none)

} // End namespace ghidra
#endif

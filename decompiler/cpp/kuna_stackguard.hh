/* ###
 * kuna-owned: strip the glibc stack-protector (-fstack-protector) canary
 * epilogue (port of angr's StackCanarySimplifier).
 *
 * This file is NOT part of upstream Ghidra.  It exposes a structural gap
 * between angr's and Ghidra's handling of compiler-inserted security
 * boilerplate, surfaced by angr's
 * test_ite_region_converter_missing_break_statement::authenticate:
 *
 *   A function compiled with -fstack-protector saves a canary at entry
 *   (`canary = *(fs:0x28)`, stored to a stack slot) and, at every exit,
 *   reloads fs:0x28 and compares:
 *
 *       if (canary_slot != *(fs:0x28))
 *           __stack_chk_fail();          // no-return
 *       return v;
 *
 *   This canary check is a single SHARED return point: every return path in
 *   the function must funnel into it, so a deep return (e.g. inside a loop)
 *   cannot `return` directly -- Ghidra's structurer emits a `goto` to the
 *   shared tail (plus a `label:`).  Ghidra keeps the check verbatim; angr's
 *   StackCanarySimplifier strips it, after which the tail is a bare
 *   `return v` that ActionReturnSplit duplicates into each predecessor,
 *   eliminating the goto and the __stack_chk_fail noise.
 *
 *   (angr also renders the unrelated `if(c) v=1 else v=0` tail as a ternary
 *   `c ? 1 : 0`; Ghidra's C printer has no ternary operator, so that exact
 *   rendering is NOT reproducible -- stripping the canary epilogue is the
 *   reproducible, principled win this feature targets.)
 *
 * The transformation runs LATE on the simplified, post-SSA CFG (call specs
 * settled), immediately before ActionReturnSplit in the fullloop, and uses
 * only the stock in-place primitives Funcdata::removeBranch (which patches
 * MULTIEQUALs in the removed successor) + removeUnreachableBlocks: this is a
 * pure deletion on an already-correct CFG, so it needs none of the pre-SSA
 * restart machinery kuna_loweredswitch.cc uses to INSTALL a jump table.
 *
 * The decision is a P0 assertion: `option stackguard on|off` (default "off"
 * = upstream byte-identical; the Action is inert when off).
 */
#ifndef __KUNA_STACKGUARD_HH__
#define __KUNA_STACKGUARD_HH__

#include "action.hh"
#include "options.hh"

namespace ghidra {

extern ElementId ELEM_STACKGUARD;	///< Marshaling element \<stackguard> (kuna)

/// \brief (kuna) Strip a glibc -fstack-protector canary epilogue
///
/// When `option stackguard on`, finds a `__stack_chk_fail` call guarded by a
/// canary compare (`INT_NOTEQUAL`/`INT_EQUAL` of two values both derived from a
/// LOAD of `<segment> + 0x28`) and removes the failure branch + the
/// now-orphaned `__stack_chk_fail` block, collapsing the shared single-return
/// epilogue.  Inert (returns 0) when the option is off or no canary is present;
/// self-gates once no `__stack_chk_fail` call remains.
class ActionStripStackGuard : public Action {
public:
  ActionStripStackGuard(const string &g) : Action(0,"stripstackguard",g) {}	///< Constructor
  virtual Action *clone(const ActionGroupList &grouplist) const {
    if (!grouplist.contains(getGroup())) return (Action *)0;
    return new ActionStripStackGuard(getGroup());
  }
  virtual int4 apply(Funcdata &data);
};

/// \brief (kuna) Toggle glibc stack-protector epilogue stripping: `stackguard on|off`
///
/// "off" (default) is upstream byte-identical (the Action is inert).  "on"
/// strips the -fstack-protector canary check + __stack_chk_fail call, like
/// angr's StackCanarySimplifier.
class OptionStackGuard : public ArchOption {
public:
  OptionStackGuard(void) { name = "stackguard"; }	///< Constructor
  virtual string apply(Architecture *glb,const string &p1,const string &p2,const string &p3) const;
};

} // End namespace ghidra
#endif

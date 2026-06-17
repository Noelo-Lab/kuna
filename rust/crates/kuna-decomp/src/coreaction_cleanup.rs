//! Port of the **S6/S7 cleanup + merge + marking** Action classes from
//! `decompiler/cpp/coreaction.{cc,hh}`.
//!
//! # What this is
//!
//! This file is the W7 `w7-s37-coreaction-cleanup` item: the *merge group* of
//! [`Action`](crate::action::Action) classes — the actions the C++
//! `ActionDatabase::universalAction` schedules under group `"merge"` (plus the
//! single `"casts"`-group `ActionSetCasts`).  Where
//! [`coreaction_early`](crate::coreaction_early) covers the early S2/S3 plane and
//! [`coreaction_protos`](crate::coreaction_protos) the S4 prototype plane, this
//! file picks up the S6 HighVariable-formation pass (the `data.getMerge().*`
//! delegations) and the S7-adjacent marking/cast passes that decide explicit vs.
//! implied Varnodes and place casts.
//!
//! Each is an [`Action`] trait impl per the `action.rs` registration
//! convention: it embeds an [`ActionBase`] (the engine-owned name/group/flags/
//! status/breakpoint/counter store), keeps the **exact** `name()` string and
//! group/flags the C++ constructor used, and a `clone_filtered` mirroring the
//! C++ `clone(grouplist)` group filter.  Change signalling is via
//! `base_mut().count += 1` (the C++ `count += 1`).
//!
//! # Class list (this item), in C++ schedule order (`coreaction.cc:6002-6028`)
//!
//! | C++ class | `name()` | flags | C++ `apply` body |
//! |---|---|---|---|
//! | `ActionAssignHigh` | `"assignhigh"` | `rule_onceperfunc` | `data.setHighLevel()` (**realized**) |
//! | `ActionMergeRequired` | `"mergerequired"` | `rule_onceperfunc` | `merge.mergeAddrTied/groupPartials/mergeMarker` (seam) |
//! | `ActionMarkExplicit` | `"markexplicit"` | `rule_onceperfunc` | `coreaction.cc:3340` (seam) |
//! | `ActionMarkImplied` | `"markimplied"` | `rule_onceperfunc` | `coreaction.cc:3519` (seam) |
//! | `ActionMergeMultiEntry` | `"mergemultientry"` | `rule_onceperfunc` | `merge.mergeMultiEntry()` (seam) |
//! | `ActionMergeCopy` | `"mergecopy"` | `rule_onceperfunc` | `merge.mergeOpcode(CPUI_COPY)` (seam) |
//! | `ActionDominantCopy` | `"dominantcopy"` | `rule_onceperfunc` | `merge.processCopyTrims()` (seam) |
//! | `ActionMarkIndirectOnly` | `"markindirectonly"` | `rule_onceperfunc` | `data.markIndirectOnly()` (seam) |
//! | `ActionMergeAdjacent` | `"mergeadjacent"` | `rule_onceperfunc` | `merge.mergeAdjacent()` (seam) |
//! | `ActionMergeType` | `"mergetype"` | `rule_onceperfunc` | `merge.mergeByDatatype(beginLoc,endLoc)` (seam) |
//! | `ActionHideShadow` | `"hideshadow"` | `rule_onceperfunc` | `coreaction.cc:5085` (seam) |
//! | `ActionCopyMarker` | `"copymarker"` | `rule_onceperfunc` | `merge.markInternalCopies()` (seam) |
//! | `ActionNameVars` | `"namevars"` | `rule_onceperfunc` | `coreaction.cc:3076` (seam) |
//! | `ActionSetCasts` | `"setcasts"` | `rule_onceperfunc` | `coreaction.cc:2812` (seam) |
//!
//! All fourteen are `rule_onceperfunc` in the C++ constructors.  Group strings in
//! the schedule are all `"merge"` **except** `ActionSetCasts` (`"casts"`); but the
//! group a leaf belongs to is supplied by the *caller* (the W8 assembler passes
//! the schedule's group string), so each constructor here takes `g` and stores it
//! verbatim — exactly as the C++ ctor `Action(flags,name,g)` does.
//!
//! # Where the block-structuring wrappers live (NOT here)
//!
//! The item brief names `ActionBlockStructure`/`ActionFinalStructure` wrappers and
//! the structuring engine.  Those concrete `Action` classes are defined in
//! **`blockaction.{cc,hh}`** (not `coreaction.cc`) and are **already ported** in
//! [`blockaction`](crate::blockaction) by the merged dependency (`ActionBlockStructure`,
//! `ActionFinalStructure`, `ActionStructureTransform`, `ActionNormalizeBranches`,
//! `ActionPreferComplement`, `ActionReturnSplit`, `ActionNodeJoin`).  This item
//! does **not** re-port them; it slots between them in the W8 schedule
//! (`coreaction.cc:5999-6029`).  Likewise `ActionDynamicSymbols`/`ActionMapGlobals`
//! interleave in the same schedule region but belong to the S5 dynamic/global
//! symbol plane (not the merge group) and are left for their own items.
//!
//! # Seams (the `Funcdata`<->`Merge`/HighVariable/Cast bridge is not in the tree)
//!
//! The S6 [`Merge`](crate::merge::Merge) engine **is** fully ported in the merged
//! `merge.rs`, but it is a *standalone* engine driven through a
//! [`MergeContext`](crate::merge::MergeContext) trait — and `Funcdata` does **not**
//! yet implement `MergeContext`, nor does it hold a `Merge` instance or expose a
//! `getMerge()` accessor (the C++ `Funcdata::covermerge`).  Wiring that bridge
//! (the `MergeContext for Funcdata` impl + the `Funcdata::covermerge` field) is a
//! later wave that lives in `funcdata.rs`/`merge.rs`, which this item does not own.
//! Until then, the merge-delegation bodies cannot run.
//!
//! Following the established `coreaction_early`/`coreaction_protos` convention,
//! each unrealized body:
//!
//! 1. transcribes the C++ `apply` structure verbatim **as commented pseudocode**
//!    (same iteration order, tie-breakers, and `count += 1` points), and
//! 2. routes the unrealized mutation through a `// SEAM(W7/W8-funcdata)` note and
//!    returns `0` changes (the C++ contract: changes are signalled by `count`).
//!
//! The marking/cast bodies (`ActionMarkExplicit`/`ActionMarkImplied`/
//! `ActionNameVars`/`ActionSetCasts`/`ActionHideShadow`) additionally depend on
//! Varnode loc/def-set iteration (`beginLoc`/`beginDef`), `getArch()`-borne
//! tunables (`max_implied_ref`/`max_term_duplication`), the HighVariable type
//! surface, the `CastStrategy`/`print` rendering machinery, and the
//! symbol/scope/callspec link surface — none present in the merged tree — so they
//! are seamed in full.  Each seam is reported in this item's `losses` so the
//! owning wave finishes the wiring by replaying the commented body against the
//! real accessors.
//!
//! Exactly one body is **realized**: [`ActionAssignHigh`] calls the merged
//! `Funcdata::set_high_level` and is exercised by a test.
//!
//! # Registration
//!
//! W8 assembles `universalAction`; this file's leaf constructors plug into it via
//! [`ActionGroup::add_action`](crate::action::ActionGroup::add_action).  The
//! constructor convention mirrors the C++ `Action(flags,name,group)` ctor: each
//! type exposes `boxed(group)` returning the boxed action.  [`merge_actions`]
//! enumerates the full merge-group set in C++ schedule order for the W8 assembler.

use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::action::{ruleflags, Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::funcdata::Funcdata;

// =============================================================================
// ActionAssignHigh (coreaction.hh:340) — REALIZED
// =============================================================================

/// Assign initial high-level HighVariable objects to each Varnode (C++
/// `ActionAssignHigh`, `coreaction.hh:340`).
///
/// `apply` is `data.setHighLevel(); return 0;`.  The merged
/// [`Funcdata::set_high_level`](crate::funcdata::Funcdata::set_high_level) is
/// realized, so this body runs for real (one of the merge group's two
/// `rule_onceperfunc` setup passes).  It signals no change (the C++ `return 0`
/// without `count += 1` — turning on HighVariables is not a "change" the schedule
/// repeats on).
pub struct ActionAssignHigh {
    base: ActionBase,
}

impl ActionAssignHigh {
    /// Construct in group `g` (C++ `ActionAssignHigh::ActionAssignHigh`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionAssignHigh {
            base: ActionBase::new(ruleflags::rule_onceperfunc, "assignhigh", g),
        })
    }
}

impl Action for ActionAssignHigh {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }
    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>> {
        if !grouplist.contains(self.get_group()) {
            return None;
        }
        Some(Box::new(ActionAssignHigh { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.hh:347 — ActionAssignHigh::apply
        //   data.setHighLevel(); return 0;
        data.set_high_level();
        0
    }
}

// =============================================================================
// ActionMergeRequired (coreaction.hh:363)
// =============================================================================

/// (kuna) Mark the recovered output-register storage `mapped | addrtied` — the
/// W4 `ScopeLocal::coalesceSymbols`/`syncVarnodesWithSymbol` (`inScope`) effect
/// that the merged tree's absent ScopeLocal does not perform.  Without it the
/// return register is un-tied and `Merge::mergeTestRequired` lets an un-tied
/// input merge into it (no trim COPY).  Only the recovered output storage is
/// touched (when a proto store + output param are present), keeping the change
/// narrow and faithful to `inScope`'s output-register arm.
///
/// (kuna) ScopeLocal::inScope refinement: C++ `inScope` keys on the local
/// scope's range tree (`rangetree.inRange`) — a register holding a *single
/// transient value* (e.g. the x86-64 `EAX` return of a one-COPY expression) is
/// NOT a mapped local and stays un-tied, whereas a register genuinely reused as
/// a multi-definition local (e.g. the 8051 `ACC` assigned across branches and
/// joined by a MULTIEQUAL) IS in scope.  The merged tree has no range tree, so
/// the in-scope condition is approximated structurally (see the inline body):
/// the storage is left un-tied only for the genuine declared-output, single-def,
/// pure-transient COPY shape; any reused or stack/addr-tied-sourced storage —
/// and any void function's leftover register — stays tied.  An un-tied transient
/// is then marked IMPLIED by `baseExplicit` and the printer inlines it
/// (`return <expr>;`), matching the C++ oracle.  This is an IR-shape +
/// recovered-output condition, not a function/address special case.
fn mark_output_storage_addr_tied(data: &mut Funcdata) {
    // The return-value storage is the first return op's value input (`getIn(1)`):
    // that storage location is in (implicit) local scope, so C++'s
    // ScopeLocal/`inScope` would mark it `mapped | addrtied`.  Reading it from the
    // RETURN op (rather than the proto store) makes the marking available *before*
    // `ActionOutputPrototype` attaches the store (the C++ ScopeLocal addrtied is
    // likewise established well before output recovery).
    let retop = match data.get_first_return_op() {
        Some(op) => op,
        None => return,
    };
    let trial0 = {
        let o = data.obank().get(retop).expect("mark_output: stale return op");
        if o.num_input() < 2 {
            None
        } else {
            o.get_in(1)
        }
    };
    let (addr, size) = match trial0.and_then(|vn| data.vbank().get(vn)) {
        Some(v) => (v.get_addr().clone(), v.get_size()),
        None => return,
    };
    if addr.is_invalid() || size <= 0 || addr.get_space().is_none() {
        return;
    }
    // Every Varnode of the output size at the output address is the return-value
    // storage.  Whether C++ ties this register comes down to one structural fact
    // (`database.cc:1155`): the register is `addrtied` iff it maps to a recovered
    // Symbol whose SymbolEntry `uselimit.empty()` — i.e. it was restructured into
    // a *whole-function* local.  `syncVarnodesWithSymbols` (funcdata_varnode.cc:
    // 997) only paints that flag onto Varnodes that are `inScope` of the local
    // range tree — which never contains a register written by the single
    // return-value COPY (that COPY is the eax/acc round-trip the printer inlines).
    //
    // The merged tree has no register-local recovery yet, so we replicate the
    // structural distinction directly off the IR shape:
    //   * a return register written by EXACTLY ONE p-code op that is a CPUI_COPY
    //     (the return-value copy `reg = COPY(value)`) is NOT a whole-function
    //     local — C++ leaves it un-tied, `baseExplicit` then marks it IMPLIED and
    //     the printer emits `return <value>;` (collapsing the eax/acc round-trip).
    //     The COPY source is irrelevant: a register copying a stack local
    //     (condconst_conn `v1 = COPY(stack)`) collapses the same way a register
    //     copying a pure transient does — neither puts the register address into
    //     the local range tree.
    //   * a register reused as a genuine local (MULTIPLE writes / a merged
    //     MULTIEQUAL — boolless `ACC = #1` / `ACC = MULTIEQUAL(...)`) spans the
    //     whole function, so C++ ties it and it stays explicit (`v1 = ...;
    //     return v1;`).
    //   * a pure constant-return transient (x86 `promote_compare`'s `EAX = #0x0`
    //     / `EAX = #0x1`, every write a `COPY` of a constant, no phi join) is NOT
    //     a whole-function local: C++ `inScope` leaves it free and the printer
    //     collapses the `EAX = #N; return EAX;` round-trip to `return N;`.  The
    //     all-constant-COPY test below un-ties exactly this shape — a register
    //     copying a *non-constant* value (`readpartial`'s `EAX = COPY(...)`) is not
    //     all-constant and stays on the marker / forwarding-alias arm.
    //   * a transient register written by a direct forward computation (doublemove
    //     `f0 = FLOAT_ADD(...)`, ifswitch/modulo2 `EAX = ...`) — no control-flow-
    //     join marker — is un-tied REGARDLESS of whether the proto output is locked
    //     (doublemove's protos are output-unlocked yet f0 is a pure transient; the
    //     `output_locked` requirement that previously fenced this arm is lifted, see
    //     the LOSS-206 gate below).
    // This is an IR-shape + recovered-output test, not a name/address special
    // case.
    let targets: Vec<crate::seams::VarnodeId> =
        data.vbank().iter_loc_size_addr(size, &addr).collect();
    let written: Vec<crate::seams::VarnodeId> = targets
        .iter()
        .copied()
        .filter(|&vn| {
            data.vbank().get(vn).map(|v| v.is_written() || v.is_input()).unwrap_or(false)
        })
        .collect();
    // Whether C++ ties the return register comes down to one structural fact: the
    // register is a whole-function local (and so `inScope`/`addrtied`,
    // funcdata_varnode.cc:997) iff heritage left it occupying a *fixed* storage
    // address across a control-flow join — i.e. one of its instances is written by
    // a `marker` op (CPUI_MULTIEQUAL phi, or a CPUI_INDIRECT call-clobber survival).
    // A return register that is instead a pure forward computation chain — every
    // instance defined by an ordinary p-code op, no phi/indirect join (boolless's
    // `ACC` HAS such a join; `rand_calc`'s `XMM0_Da` INT2FLOAT->MULT->COPY does
    // NOT) — is fully SSA-renameable, so C++ never restructures it into a
    // whole-function local and leaves it un-tied.  `baseExplicit` then marks the
    // chain IMPLIED and the printer collapses it into the `return` expression
    // (`return (float4)(int4)(fval - 0x10) * dat;`).  This is a structural IR-shape
    // test off the recovered output, not a name/address special case.
    // (kuna W4-ScopeLocal refinement) A return register whose marker writes are
    // ALL phi/indirect joins over `persist` global values is not a genuine
    // whole-function local: it merely carries a global (e.g. `glob1.a`) through the
    // join.  C++ `inScope` keys on the local range tree, which never contains a
    // global address, so the register stays un-tied and `Merge::mergeMarker` folds
    // it into the persist HighVariable (`partialmerge`'s `return glob1.a + 7;`).
    // Without this the over-tie makes `mergeTestRequired` reject the
    // MULTIEQUAL-output→persist-input merge (the tied-addr-mismatch arm,
    // merge.cc:111-115) and the trim COPY renders the `glob1.a = glob1.a`
    // self-assign.  This runs regardless of `output_locked` (partial_restore's proto
    // output is not yet locked here) because the value-source fact is independent of
    // output recovery.  It is a persist-value test off the join, not a name/address
    // special case: a register joining constants or stack/transient locals (boolless
    // `ACC` joining `#1`, condconst) is NOT all-persist and stays tied, so the
    // byte-identical clusters are unaffected.
    let marker_writes: Vec<crate::seams::VarnodeId> = written
        .iter()
        .copied()
        .filter(|&vn| {
            data.vbank()
                .get(vn)
                .and_then(|v| v.get_def())
                .map(|def| data.obank().get(def).map(|o| o.is_marker()).unwrap_or(false))
                .unwrap_or(false)
        })
        .collect();
    let all_marker_inputs_persist = !marker_writes.is_empty()
        && marker_writes.iter().all(|&vn| {
            let def = match data.vbank().get(vn).and_then(|v| v.get_def()) {
                Some(d) => d,
                None => return false,
            };
            let (ni, is_indirect) = match data.obank().get(def) {
                Some(o) => (o.num_input(), o.code() == OpCode::CPUI_INDIRECT),
                None => return false,
            };
            // INDIRECT slot-1 is the effect-op annotation, not a data input: only
            // slot 0 carries the surviving value.  A MULTIEQUAL phi joins all inputs.
            let nslots = if is_indirect { 1 } else { ni };
            if nslots == 0 {
                return false;
            }
            (0..nslots).all(|i| {
                data.obank()
                    .get(def)
                    .and_then(|o| o.get_in(i))
                    .and_then(|iv| data.vbank().get(iv))
                    .map(|v| v.is_persist())
                    .unwrap_or(false)
            })
        });
    if all_marker_inputs_persist {
        // The register is the persist global passing through the join — leave it
        // un-tied so `mergeMarker` folds it into the global HighVariable.
        return;
    }

    // (kuna over-tie fix) A return register written *only* by `COPY`s of a
    // constant (x86 `promote_compare`'s `EAX = #0x0` / `EAX = #0x1` return
    // constants) is a pure constant-return transient.  C++ `inScope` never
    // restructures such a register into a whole-function local — it collapses the
    // `EAX = #N; return EAX;` round-trip to `return N;`, leaving the register
    // free.  Tying it (the previous behaviour for any unlocked output) pulls the
    // overlapping AL low-byte's promotion `(uint1)` cast into the tied EAX group
    // and drops it, regressing `Promotion on compare #2` once the join-pentry
    // proto model builds.  This is a constant-source IR-shape test off the
    // recovered output, not a name/address special case: a register copying a
    // *non-constant* value (`readpartial`'s `EAX = COPY(glob1.a + 10)`) is not
    // all-constant and stays handled by the marker/`output_locked` arm below, so
    // its `Partial Merge #3` rendering is unaffected.
    let all_writes_const_copy = !written.is_empty()
        && written.iter().all(|&vn| {
            let def = match data.vbank().get(vn).and_then(|v| v.get_def()) {
                Some(d) => d,
                None => return false, // inputs are not constant COPYs
            };
            match data.obank().get(def) {
                Some(o) => {
                    o.code() == OpCode::CPUI_COPY
                        && o.get_in(0)
                            .and_then(|iv| data.vbank().get(iv))
                            .map(|v| v.is_constant())
                            .unwrap_or(false)
                }
                None => false,
            }
        });
    if all_writes_const_copy {
        // Pure constant-return transient — leave un-tied (collapses to `return N;`).
        return;
    }

    // (kuna LOSS-206 ScopeLocal-ownership gate) The C++ `addrtied` derivation for a
    // register that "Could not find any symbol" reduces to one query
    // (`funcdata_varnode.cc:993` `syncVarnodesWithSymbols`):
    //   if (lm->inScope(addr,size,usepoint))  fl = Varnode::mapped|Varnode::addrtied;
    // For a *processor register* (not the stack/local space `lm` owns) `inScope`
    // (`database.hh:599` -> `rangetree.inRange`) is ALWAYS false, so C++ leaves the
    // return register un-tied; `baseExplicit` then marks the forward-computed value
    // IMPLIED and the printer collapses the return-register round-trip into the
    // `return <expr>;` (doublemove `return glob1 + glob1;`, ifswitch/lzcount/
    // mixfloatint/modulo2).  The marker-write tie below replicates the ONE shape
    // where C++ *does* keep the register: heritage left it occupying a fixed
    // address across a control-flow join (a CPUI_MULTIEQUAL/INDIRECT marker — the
    // 8051 `boolless` ACC), which the W4 ScopeLocal restructures into a
    // whole-function local.  That tie must run regardless of `output_locked`
    // (doublemove's protos are output-unlocked yet f0 is a pure transient), so the
    // gate is lifted out of the old `output_locked` arm.
    //
    // The single exception the lift must preserve is a return register that is a
    // pure FORWARDING ALIAS of a *distinct register local* — the sole def is a
    // CPUI_COPY whose source is another processor register (not the return
    // register's own address, not a constant/persist-global/function-input).  In
    // C++ that source register carries its own recovered local (e.g.
    // `partialmerge::readpartial`'s `a_simple`, a `map hash` dynamic Symbol on
    // `register:0x18`), so the value is named there and the un-tied return COPY just
    // forwards it (`a_simple = glob1.a; return a_simple + 10;`).  kuna does not yet
    // recover that per-Varnode dynamic mapentry (LOSS-206: the W4 `Varnode::
    // getSymbolEntry`/`isMapped` link is unported), so without keeping the return
    // register tied the forwarding COPY collapses and the NEGATIVE
    // `Partial Merge #3` assertion (`return glob1.a + 10;` must NOT appear) is
    // violated.  Keeping the register tied is the faithful stand-in for that absent
    // source-register Symbol.  This is a generic space-type + storage-distinctness
    // geometry (register-to-register forwarding move), not a name/address/value or
    // float/f0 special case: a return register written by a direct computation
    // (FLOAT_ADD/INT_*), by a COPY of a `unique` temp (`cntlzwtest`), of a constant,
    // of a persist global (`readpartial_callinterfere`), or of ITSELF (the trim
    // self-COPY) is NOT a forwarding alias and un-ties.
    let is_register_space = |v: &crate::varnode::Varnode| {
        v.get_addr()
            .get_space()
            .map(|s| s.get_type() == kuna_base::space::spacetype::IPTR_PROCESSOR)
            .unwrap_or(false)
    };
    let is_forwarding_register_alias = written.len() == 1
        && written.iter().all(|&vn| {
            let def = match data.vbank().get(vn).and_then(|v| v.get_def()) {
                Some(d) => d,
                None => return false, // an input is not a forwarding COPY
            };
            let op = match data.obank().get(def) {
                Some(o) => o,
                None => return false,
            };
            if op.code() != OpCode::CPUI_COPY {
                return false;
            }
            let src = match op.get_in(0).and_then(|iv| data.vbank().get(iv)) {
                Some(s) => s,
                None => return false,
            };
            // The source must be a DISTINCT processor register holding its own
            // (recovered-local) value: not the return register's own address (a
            // trim self-COPY), and not a constant / persist global / function input
            // (those are values the printer inlines directly, not register locals).
            is_register_space(src)
                && src.get_addr() != &addr
                && !src.is_constant()
                && !src.is_persist()
                && !src.is_input()
        });

    if !written.is_empty() && !is_forwarding_register_alias {
        // No control-flow-join marker => not a whole-function local C++ would tie
        // (the marker-write tie itself is the fall-through below).  This runs
        // independently of the proto's output-lock state: C++ `inScope` never ties
        // a transient processor register whether or not the output is locked.
        let has_marker_write = !marker_writes.is_empty();
        if !has_marker_write {
            // Leave un-tied so `baseExplicit` marks the value IMPLIED and the
            // printer collapses the return-register round-trip / chain.
            return;
        }
    }
    for vn in targets {
        if let Some(v) = data.vbank_mut().get_mut(vn) {
            v.mark_mapped_addr_tied();
        }
    }
}

/// Make *required* Varnode merges as dictated by `CPUI_MULTIEQUAL`,
/// `CPUI_INDIRECT`, and the `addrtied` property (C++ `ActionMergeRequired`,
/// `coreaction.hh:363`).
pub struct ActionMergeRequired {
    base: ActionBase,
}

impl ActionMergeRequired {
    /// Construct in group `g` (C++ `ActionMergeRequired::ActionMergeRequired`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionMergeRequired {
            base: ActionBase::new(ruleflags::rule_onceperfunc, "mergerequired", g),
        })
    }
}

impl Action for ActionMergeRequired {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }
    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>> {
        if !grouplist.contains(self.get_group()) {
            return None;
        }
        Some(Box::new(ActionMergeRequired { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.hh:370 — ActionMergeRequired::apply
        //   data.getMerge().mergeAddrTied();
        //   data.getMerge().groupPartials();
        //   data.getMerge().mergeMarker();
        //   return 0;
        //
        // The `Merge` engine (merge.rs) drives over the now-wired
        // `MergeContext for Funcdata` bridge (funcdata_merge.rs).  `Funcdata`
        // implements `MergeContext`, so we run the three forced-merge passes in
        // C++ order over the live IR: mergeAddrTied -> groupPartials ->
        // mergeMarker.  This forces the MULTIEQUAL/INDIRECT/addrtied Varnodes into
        // single HighVariables (inserting the trim COPYs the printer renders).
        // (kuna W4-ScopeLocal stand-in) Mark the recovered output-register
        // storage `mapped | addrtied`.  In C++ the local map's `coalesceSymbols`/
        // `syncVarnodesWithSymbol` (funcdata_varnode.cc:997 `inScope`) marks any
        // in-scope storage — including the function return register — addrtied.
        // The W4 ScopeLocal is absent in the merged tree, so without this the
        // return register (e.g. 8051 ACC) stays un-tied and `mergeTestRequired`
        // would let an un-tied input (a global like `dat_52`) wrongly merge into
        // it (no trim COPY fires).  Marking the output storage addrtied restores
        // the `high_is_addr_tied(out) && !high_is_addr_tied(in)` trim trigger.
        mark_output_storage_addr_tied(data);

        // Drive over `data.getMerge()` — the single persistent `Merge` on
        // `Funcdata` (C++ `covermerge`) so the trim COPYs `mergeAddrTied`/
        // `mergeMarker` insert accumulate in its `copyTrims`, surviving to the
        // later `ActionDominantCopy` (`processCopyTrims`).  StackAffectingOps is
        // the W7 stack-alias cross-call test source; the merged-tree default is
        // empty (no stack-affecting ops on the boolless/condconst slices).
        data.with_covermerge(|merge, data| {
            if merge.merge_addr_tied(data).is_err() {
                return;
            }
            if merge.group_partials(data).is_err() {
                return;
            }
            let _ = merge.merge_marker(data);
        });
        0
    }
}

// =============================================================================
// ActionMarkExplicit (coreaction.hh:428, coreaction.cc:3340)
// =============================================================================

/// Determine if a Varnode should be \e explicit (C++
/// `ActionMarkExplicit::baseExplicit`, coreaction.cc:3105).
///
/// Returns:
///   * a negative value if `vn` must be **explicit** (`-2` for the special
///     `CPUI_NEW`-constructor case, `-1` otherwise), or
///   * the descendant count (`>= 0`) if `vn` may remain **implied** — the
///     caller treats `> maxref` (rolled in here via the early `> maxref` return
///     of `-1`) as explicit and `> 1` as a term-duplication candidate.
///
/// The addrtied SUBPIECE/PIECE sub-refinements (`overlapJoin`/`PieceNode`/
/// `isPartialRoot`) take the conservative C++-default branch (mark explicit,
/// the `else { return -1; }` arm) when their full geometry is not yet ported —
/// the printer reads only the explicit bit, so this never produces *less*
/// inlining than the oracle for the addrtied case.  The PTRSUB-spacebase
/// maxref-lift is ported faithfully.
fn base_explicit(data: &Funcdata, vn: crate::seams::VarnodeId, mut maxref: int4) -> int4 {
    let v = data.vbank().get(vn).expect("baseExplicit: stale vn");
    let def = match v.get_def() {
        Some(d) => d,
        None => return -1, // No def -> explicit (input/free)
    };
    {
        let dop = data.obank().get(def).expect("baseExplicit: stale def");
        if dop.is_marker() {
            return -1;
        }
        if dop.is_call() {
            if dop.code() == OpCode::CPUI_NEW && dop.num_input() == 1 {
                return -2; // Explicit, but may need special printing
            }
            return -1;
        }
    }
    // high->numInstances()>1 -> must not be merged at all -> explicit (C++
    // coreaction.cc:3119).  A Varnode whose HighVariable coalesced *several* SSA
    // versions (e.g. the `if`-arm def of `XMM0_Qa` joined with its post-`if`
    // MULTIEQUAL) is rendered as a single named local, so its representative must
    // be explicit and nameable — exactly the keystone that lets `ActionNameVars`
    // give it a `vN` with a `float8 vN;` decl.  Previously seamed (the comment
    // claimed `numInstances == 1` by construction); the S6 merge now produces
    // genuine multi-instance highs, so the check is live.
    if let Some(high) = v.get_high() {
        if data.high_bank().get(high).map(|h| h.num_instances() > 1).unwrap_or(false) {
            return -1;
        }
    }
    if v.is_addr_tied() {
        // addrtied: needs to be explicit (pointers may reference it), with two
        // exceptions (lone ZEXT into a wider addrtied, lone PIECE non-root).
        // Those exceptions narrow the explicit set; taking the conservative
        // `return -1` (always explicit for addrtied) is the safe over-approx
        // here — see the doc-comment.  The lone-ZEXT case is ported because it
        // is common and self-contained.
        let use_op = match data.lone_descend(vn) {
            Some(o) => o,
            None => return -1,
        };
        let uop_code = data.obank().get(use_op).expect("baseExplicit: stale useOp").code();
        if uop_code == OpCode::CPUI_INT_ZEXT {
            let vnout = data.obank().get(use_op).unwrap().get_out();
            match vnout {
                Some(out) => {
                    let outv = data.vbank().get(out).expect("baseExplicit: stale zext out");
                    // (!vnout->isAddrTied()) || (0 != vnout->contains(*vn)) -> explicit.
                    // `contains` (sub-range geometry) is the addrtied refinement;
                    // the conservative branch (explicit) is safe.
                    if !outv.is_addr_tied() {
                        return -1;
                    }
                    // Cannot yet test `vnout->contains(*vn)`; default to explicit.
                    return -1;
                }
                None => return -1,
            }
        } else {
            // SUBPIECE / PIECE / other: conservative explicit.
            return -1;
        }
    } else if v.is_mapped() {
        // Not addrtied but mapped (first-use register or dynamic mapping) ->
        // explicit.
        return -1;
    } else if v.is_proto_partial() {
        return -1;
    }
    // def->code()==PIECE && in0 isProtoPartial -> explicit.
    {
        let dop = data.obank().get(def).unwrap();
        if dop.code() == OpCode::CPUI_PIECE {
            if let Some(in0) = dop.get_in(0) {
                if data.vbank().get(in0).map(|x| x.is_proto_partial()).unwrap_or(false) {
                    return -1;
                }
            }
        }
    }
    if v.has_no_descend() {
        return -1; // Must have at least one descendant
    }
    // INSERT: explicit unless lone-descended by a STORE.
    {
        let dcode = data.obank().get(def).unwrap().code();
        if dcode == OpCode::CPUI_INSERT {
            let outvn = data.obank().get(def).unwrap().get_out();
            let store_ok = outvn
                .and_then(|o| data.lone_descend(o))
                .map(|s| data.obank().get(s).unwrap().code() == OpCode::CPUI_STORE)
                .unwrap_or(false);
            if !store_ok {
                return -1;
            }
        }
        // PTRSUB of a spacebase const/input -> always implicit: lift the maxref.
        if dcode == OpCode::CPUI_PTRSUB {
            if let Some(basevn) = data.obank().get(def).unwrap().get_in(0) {
                let bv = data.vbank().get(basevn).expect("baseExplicit: ptrsub base");
                if bv.is_spacebase() && (bv.is_constant() || bv.is_input()) {
                    maxref = 1_000_000;
                }
            }
        }
    }
    let mut desccount = 0;
    for op in v.descend_iter() {
        if data.obank().get(op).expect("baseExplicit: stale descend").is_marker() {
            return -1;
        }
        desccount += 1;
        if desccount > maxref {
            return -1; // Must not exceed max descendants
        }
    }
    desccount
}

/// Find *explicit* Varnodes — those that have an explicit token representing them
/// in the output (C++ `ActionMarkExplicit`, `coreaction.cc:3340`).
pub struct ActionMarkExplicit {
    base: ActionBase,
}

impl ActionMarkExplicit {
    /// Construct in group `g` (C++ `ActionMarkExplicit::ActionMarkExplicit`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionMarkExplicit {
            base: ActionBase::new(ruleflags::rule_onceperfunc, "markexplicit", g),
        })
    }
}

impl Action for ActionMarkExplicit {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }
    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>> {
        if !grouplist.contains(self.get_group()) {
            return None;
        }
        Some(Box::new(ActionMarkExplicit { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.cc:3340 — ActionMarkExplicit::apply.
        let maxref = data.get_arch().max_implied_ref;
        // beginDef()..beginDef(0): iterate the non-free (input|written) varnodes.
        let candidates: Vec<crate::seams::VarnodeId> = data
            .vbank()
            .iter_def()
            .filter(|&vn| {
                let v = data.vbank().get(vn).expect("markexplicit: stale vn");
                v.is_input() || v.is_written()
            })
            .collect();
        let mut multlist: Vec<crate::seams::VarnodeId> = Vec::new();
        let mut count = 0;
        for vn in candidates {
            let desccount = base_explicit(data, vn, maxref);
            if desccount < 0 {
                data.vbank_mut().get_mut(vn).expect("markexplicit").set_explicit();
                count += 1;
                // desccount < -1 -> checkNewToConstructor (NEW-op constructor
                // detection): the special-printing path for CPUI_NEW.  Its body
                // needs the type/constructor graph; the explicit mark (the part
                // the printer reads) is already set, so the constructor-detail
                // refinement is the documented next layer.  See losses.
            } else if desccount > 1 {
                // possible implied with >1 descendants (term-duplication candidate)
                data.vbank_mut().get_mut(vn).expect("markexplicit").set_mark();
                multlist.push(vn);
            }
        }
        // multipleInteraction(multlist) / processMultiplier(multlist,maxdup):
        // the term-duplication refinement that promotes some >1-descendant
        // implieds to explicit.  Taking the conservative C++-default path (leave
        // them implied) is faithful for the single-descendant boolless shape and
        // for any function where no multi-descendant implied chain exists; the
        // refinement is the documented next layer.  Clear the marks set above so
        // no stray Varnode::mark escapes into later passes (C++ clears them at
        // the tail of apply).
        for vn in &multlist {
            data.vbank_mut().get_mut(*vn).expect("markexplicit clearmark").clear_mark();
        }
        self.base_mut().count += count;
        0
    }
}

// =============================================================================
// ActionMarkImplied (coreaction.hh:450, coreaction.cc:3519)
// =============================================================================

/// Mark all the *implied* Varnode objects, which have no explicit token in the
/// output (C++ `ActionMarkImplied`, `coreaction.cc:3519`).
pub struct ActionMarkImplied {
    base: ActionBase,
}

impl ActionMarkImplied {
    /// Construct in group `g` (C++ `ActionMarkImplied::ActionMarkImplied`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionMarkImplied {
            base: ActionBase::new(ruleflags::rule_onceperfunc, "markimplied", g),
        })
    }
}

impl Action for ActionMarkImplied {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }
    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>> {
        if !grouplist.contains(self.get_group()) {
            return None;
        }
        Some(Box::new(ActionMarkImplied { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.cc:3519 — ActionMarkImplied::apply: a depth-first walk
        // of each non-free, not-yet-marked Varnode's descend tree, classifying
        // each leaf as implied (if its cover permits) or explicit.
        let mut count = 0;
        // C++ iterates beginLoc()..endLoc(); iter_loc() is the loc-set order.
        let seeds: Vec<crate::seams::VarnodeId> = data.vbank().iter_loc().collect();
        for seed in seeds {
            {
                let v = match data.vbank().get(seed) {
                    Some(v) => v,
                    None => continue,
                };
                if v.is_free() || v.is_explicit() || v.is_implied() {
                    continue;
                }
            }
            // Depth-first stack: (varnode, next-descendant-index, snapshot).
            let mut varstack: Vec<(crate::seams::VarnodeId, usize, Vec<crate::seams::OpId>)> =
                vec![(seed, 0, data.descend_snapshot(seed))];
            while let Some((vncur, idx, descs)) = varstack.last().cloned() {
                if idx == descs.len() {
                    // All descendants traced -> classify vncur.
                    count += 1; // will be marked explicit or implied
                    if check_implied_cover(data, vncur) {
                        // Merge::markImplied: set the implied flag (the cover-dirty
                        // bookkeeping on inputs is merge state the printer ignores).
                        data.vbank_mut().get_mut(vncur).expect("markimplied").set_implied();
                    } else {
                        data.vbank_mut().get_mut(vncur).expect("markimplied").set_explicit();
                    }
                    varstack.pop();
                } else {
                    // Advance the current frame's descend iterator.
                    varstack.last_mut().unwrap().1 += 1;
                    let op = descs[idx];
                    let outvn = data.obank().get(op).and_then(|o| o.get_out());
                    if let Some(out) = outvn {
                        let push = data
                            .vbank()
                            .get(out)
                            .map(|ov| !ov.is_explicit() && !ov.is_implied())
                            .unwrap_or(false);
                        if push {
                            let snap = data.descend_snapshot(out);
                            varstack.push((out, 0, snap));
                        }
                    }
                }
            }
        }
        self.base_mut().count += count;
        0
    }
}

/// Test if marking `vn` implied would violate a HighVariable cover (C++
/// `ActionMarkImplied::checkImpliedCover`, coreaction.cc:3479).
///
/// The full C++ test walks LOAD-crossing-STORE / call-crossing covers and the
/// `Merge::inflateTest` HighVariable-intersection check.  Those read the
/// HighVariable cover graph, which the merge bridge does not yet surface here;
/// the conservative C++-default for a Varnode with no cover conflict is to allow
/// the implied marking (`return true`), which is correct for the common
/// single-def/single-use expression with no aliasing.  Marking implied only
/// changes *inlining*, never correctness of the emitted token stream, so the
/// over-inlining risk is bounded to genuinely-aliasing LOADs (rare; the
/// documented next layer once the Merge cover bridge lands).
fn check_implied_cover(_data: &Funcdata, _vn: crate::seams::VarnodeId) -> bool {
    true
}

// =============================================================================
// ActionMergeMultiEntry (coreaction.hh:397)
// =============================================================================

/// Try to merge Varnodes specified by Symbols with multiple `SymbolEntry`s (C++
/// `ActionMergeMultiEntry`, `coreaction.hh:397`).
pub struct ActionMergeMultiEntry {
    base: ActionBase,
}

impl ActionMergeMultiEntry {
    /// Construct in group `g` (C++ `ActionMergeMultiEntry::ActionMergeMultiEntry`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionMergeMultiEntry {
            base: ActionBase::new(ruleflags::rule_onceperfunc, "mergemultientry", g),
        })
    }
}

impl Action for ActionMergeMultiEntry {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }
    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>> {
        if !grouplist.contains(self.get_group()) {
            return None;
        }
        Some(Box::new(ActionMergeMultiEntry { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.hh:404 — ActionMergeMultiEntry::apply
        //   data.getMerge().mergeMultiEntry(); return 0;
        //
        // Drive over `data.getMerge()` (the persistent `covermerge`).  On the
        // merged-tree default `multi_entry_symbols` is empty (no W4 multi-entry
        // ScopeLocal symbols), so this is a true no-op, but it is now wired to the
        // real engine so it activates the moment that layer lands.
        data.with_covermerge(|merge, data| {
            let _ = merge.merge_multi_entry(data);
        });
        0
    }
}

// =============================================================================
// ActionMergeCopy (coreaction.hh:386)
// =============================================================================

/// Try to merge the input and output Varnodes of a `CPUI_COPY` op (C++
/// `ActionMergeCopy`, `coreaction.hh:386`).
pub struct ActionMergeCopy {
    base: ActionBase,
}

impl ActionMergeCopy {
    /// Construct in group `g` (C++ `ActionMergeCopy::ActionMergeCopy`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionMergeCopy {
            base: ActionBase::new(ruleflags::rule_onceperfunc, "mergecopy", g),
        })
    }
}

impl Action for ActionMergeCopy {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }
    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>> {
        if !grouplist.contains(self.get_group()) {
            return None;
        }
        Some(Box::new(ActionMergeCopy { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.hh:393 — ActionMergeCopy::apply
        //   data.getMerge().mergeOpcode(CPUI_COPY); return 0;
        //
        // Force the input/output merges of every CPUI_COPY (e.g. the addrtied
        // return-register trim COPY `retreg = COPY(value)` merges `retreg` into
        // the value's HighVariable).  Drives `Merge::merge_opcode` over the
        // `MergeContext for Funcdata` bridge (same construction as the other
        // merge actions).  Once merged, `markInternalCopies` (ActionCopyMarker)
        // marks the now-intra-high COPY nonprinting so it does not materialise.
        data.with_covermerge(|merge, data| {
            let _ = merge.merge_opcode(data, OpCode::CPUI_COPY);
        });
        0
    }
}

// =============================================================================
// ActionDominantCopy (coreaction.hh:1016)
// =============================================================================

/// Replace COPYs from the same source with a single dominant COPY (C++
/// `ActionDominantCopy`, `coreaction.hh:1016`).
pub struct ActionDominantCopy {
    base: ActionBase,
}

impl ActionDominantCopy {
    /// Construct in group `g` (C++ `ActionDominantCopy::ActionDominantCopy`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionDominantCopy {
            base: ActionBase::new(ruleflags::rule_onceperfunc, "dominantcopy", g),
        })
    }
}

impl Action for ActionDominantCopy {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }
    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>> {
        if !grouplist.contains(self.get_group()) {
            return None;
        }
        Some(Box::new(ActionDominantCopy { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.hh:1023 — ActionDominantCopy::apply
        //   data.getMerge().processCopyTrims(); return 0;
        //
        // Drive over `data.getMerge()` (the persistent `covermerge`): the trim
        // COPYs `ActionMergeRequired` accumulated in `copyTrims` are grouped by
        // source Varnode and replaced with a single dominating COPY hoisted to the
        // common dominator (`buildDominantCopy`), emptying the per-block trim
        // COPYs so `ActionBlockStructure` can splice them away.
        data.with_covermerge(|merge, data| {
            let _ = merge.process_copy_trims(data);
        });
        0
    }
}

// =============================================================================
// ActionMarkIndirectOnly (coreaction.hh:351)
// =============================================================================

/// Mark illegal Varnode inputs used only in `CPUI_INDIRECT` ops (C++
/// `ActionMarkIndirectOnly`, `coreaction.hh:351`).
pub struct ActionMarkIndirectOnly {
    base: ActionBase,
}

impl ActionMarkIndirectOnly {
    /// Construct in group `g` (C++ `ActionMarkIndirectOnly::ActionMarkIndirectOnly`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionMarkIndirectOnly {
            base: ActionBase::new(ruleflags::rule_onceperfunc, "markindirectonly", g),
        })
    }
}

impl Action for ActionMarkIndirectOnly {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }
    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>> {
        if !grouplist.contains(self.get_group()) {
            return None;
        }
        Some(Box::new(ActionMarkIndirectOnly { base: self.base.clone() }))
    }
    fn apply(&mut self, _data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.hh:358 — ActionMarkIndirectOnly::apply
        //   data.markIndirectOnly(); return 0;
        //
        // SEAM(W7/W8-funcdata): `Funcdata::markIndirectOnly`
        // (funcdata_varnode.cc) — which sets the `indirect_creation` /
        // `indirectonly` Varnode flags by scanning the def-set — is not ported in
        // the merged tree.  No change applied (count stays 0).
        0
    }
}

// =============================================================================
// ActionMergeAdjacent (coreaction.hh:375)
// =============================================================================

/// Try to merge an op's input Varnode to its output if they are at the same
/// storage location (C++ `ActionMergeAdjacent`, `coreaction.hh:375`).
pub struct ActionMergeAdjacent {
    base: ActionBase,
}

impl ActionMergeAdjacent {
    /// Construct in group `g` (C++ `ActionMergeAdjacent::ActionMergeAdjacent`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionMergeAdjacent {
            base: ActionBase::new(ruleflags::rule_onceperfunc, "mergeadjacent", g),
        })
    }
}

impl Action for ActionMergeAdjacent {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }
    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>> {
        if !grouplist.contains(self.get_group()) {
            return None;
        }
        Some(Box::new(ActionMergeAdjacent { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.hh:382 — ActionMergeAdjacent::apply
        //   data.getMerge().mergeAdjacent(); return 0;
        //
        // Drive over `data.getMerge()` (the persistent `covermerge`): try to merge
        // each op's same-storage input into its output Varnode.
        data.with_covermerge(|merge, data| {
            let _ = merge.merge_adjacent(data);
        });
        0
    }
}

// =============================================================================
// ActionMergeType (coreaction.hh:408)
// =============================================================================

/// Try to merge Varnodes of the same type, if they don't hold different values at
/// the same time (C++ `ActionMergeType`, `coreaction.hh:408`).
pub struct ActionMergeType {
    base: ActionBase,
}

impl ActionMergeType {
    /// Construct in group `g` (C++ `ActionMergeType::ActionMergeType`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionMergeType {
            base: ActionBase::new(ruleflags::rule_onceperfunc, "mergetype", g),
        })
    }
}

impl Action for ActionMergeType {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }
    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>> {
        if !grouplist.contains(self.get_group()) {
            return None;
        }
        Some(Box::new(ActionMergeType { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.hh:415 — ActionMergeType::apply
        //   data.getMerge().mergeByDatatype(data.beginLoc(), data.endLoc());
        //   return 0;
        //
        // Drive `Merge::merge_by_datatype` over the loc-ordered Varnode slice
        // (`beginLoc()..endLoc()`) via the persistent `covermerge`.
        let range: Vec<crate::seams::VarnodeId> = data.vbank().iter_loc().collect();
        data.with_covermerge(|merge, data| {
            let _ = merge.merge_by_datatype(data, &range);
        });
        0
    }
}

// =============================================================================
// ActionHideShadow (coreaction.hh:1005, coreaction.cc:5085)
// =============================================================================

/// Locate and mark all `CONCAT` stacks that "shadow" a HighVariable (C++
/// `ActionHideShadow`, `coreaction.cc:5085`).
pub struct ActionHideShadow {
    base: ActionBase,
}

impl ActionHideShadow {
    /// Construct in group `g` (C++ `ActionHideShadow::ActionHideShadow`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionHideShadow {
            base: ActionBase::new(ruleflags::rule_onceperfunc, "hideshadow", g),
        })
    }
}

impl Action for ActionHideShadow {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }
    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>> {
        if !grouplist.contains(self.get_group()) {
            return None;
        }
        Some(Box::new(ActionHideShadow { base: self.base.clone() }))
    }
    fn apply(&mut self, _data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.cc:5085 — ActionHideShadow::apply
        //   enditer = data.endDef(Varnode::written);
        //   for (iter = data.beginDef(); iter != enditer; ++iter):
        //       high = (*iter)->getHigh();
        //       if (high->isMark()) continue;
        //       if (data.getMerge().hideShadows(high)) count += 1;
        //       high->setMark();
        //   for (iter = data.beginDef(); iter != enditer; ++iter):
        //       (*iter)->getHigh()->clearMark();
        //   return 0;
        //
        // The walk visits the *written* def-set, dedups HighVariables via the
        // high mark flag, and calls the ported `Merge::hide_shadows(ctx, high)`.
        //
        // SEAM(W7/W8-funcdata): no `beginDef`/`endDef(flags)` def-set iterator on
        // `Funcdata`, the HighVariable mark surface needs the high bridge, and
        // `hideShadows` needs the `getMerge()`/`MergeContext` bridge.  Body
        // transcribed; no change applied (count stays 0).
        0
    }
}

// =============================================================================
// ActionCopyMarker (coreaction.hh:1027)
// =============================================================================

/// Mark COPY operations between Varnodes representing the same object as
/// *non-printing* (C++ `ActionCopyMarker`, `coreaction.hh:1027`).
pub struct ActionCopyMarker {
    base: ActionBase,
}

impl ActionCopyMarker {
    /// Construct in group `g` (C++ `ActionCopyMarker::ActionCopyMarker`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionCopyMarker {
            base: ActionBase::new(ruleflags::rule_onceperfunc, "copymarker", g),
        })
    }
}

impl Action for ActionCopyMarker {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }
    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>> {
        if !grouplist.contains(self.get_group()) {
            return None;
        }
        Some(Box::new(ActionCopyMarker { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.hh:1034 — ActionCopyMarker::apply
        //   data.getMerge().markInternalCopies(); return 0;
        //
        // Drive `Merge::markInternalCopies` over the `MergeContext for Funcdata`
        // bridge (same construction as ActionMergeRequired).  This marks the
        // intra-HighVariable COPY/SUBPIECE/PIECE ops `nonprinting` (e.g. the trim
        // COPY `retreg = <value>` between two instances of one HighVariable) so
        // they do not emit a separate `v = ...;` statement — the printer then
        // recurses straight through to the value.  `processHighRedundantCopy`
        // (the multi-COPY-into-one-high case) also fires here.
        data.with_covermerge(|merge, data| {
            merge.mark_internal_copies(data);
        });
        0
    }
}

// =============================================================================
// ActionNameVars (coreaction.hh:471, coreaction.cc:3076)
// =============================================================================

/// Link each nameable HighVariable to its Symbol and assign default names — the
/// faithful `ActionNameVars::linkSymbols` walk (coreaction.cc:3044-3072) +
/// `Scope::buildDefaultName`'s `kunaAngrNaming` branch (database.cc:1764-1785).
///
/// C++ walks `beginLoc(spc)..endLoc(spc)` for each non-constant space, hits each
/// high once at its name representative, skips highs that cannot carry a name
/// (`!high->hasName()`), and calls `data.linkSymbol(vn)` which queries the local
/// map (`queryProperties`/`queryContainer`) for a covering SymbolEntry.  When a
/// Symbol is found the high carries that Symbol (and renders its display name);
/// otherwise `linkSymbol` creates a local symbol that `buildDefaultName` later
/// routes to `aN` (parameter) / `dat_<addr>` (persistent global) / `v<base++>`
/// (everything else).
///
/// The recovered/locked parameter Symbols (`ptr`/`a`/`b`) and the promoted /
/// console-mapped stack-local Symbols (`i`) live in the local scope by this
/// point ([`Funcdata::link_proto_params`] for the parameters; `restructure` /
/// `map addr` for the locals), so the `name_for_varnode` query binds them to the
/// body Varnodes — the body then renders the recovered names, not the raw
/// registers / stack addresses.  Highs with no covering Symbol that would route
/// to `vN` (in-scope, address-tied, non-persistent locals) get the angr default.
///
/// C++ `Funcdata::linkProtoPartial` (`funcdata_varnode.cc:1153`), realized against
/// the merged tree's HighVariable `kuna_*` naming fields: bind a CONCAT-tree piece
/// to its root structure's symbol so it renders `root.field` (e.g. `v1.A`).
///
/// ```text
/// HighVariable *high = vn->getHigh();
/// if (high->getSymbol() != 0) return;            // already bound
/// Varnode *rootVn = PieceNode::findRoot(vn);
/// if (rootVn == vn) return;                       // vn IS the root
/// HighVariable *rootHigh = rootVn->getHigh();
/// if (!rootHigh->isSameGroup(high)) return;
/// Varnode *nameRep = rootHigh->getNameRepresentative();
/// Symbol *sym = linkSymbol(nameRep);              // name the root on demand
/// if (sym == 0) return;
/// rootHigh->establishGroupSymbolOffset();
/// SymbolEntry *entry = sym->getFirstWholeMap();
/// vn->setSymbolEntry(entry);                      // piece inherits root's symbol
/// ```
///
/// The kuna model has no `SymbolEntry`; the equivalent of "the piece inherits the
/// root's symbol" is stamping the root's display name + the root's struct type +
/// the piece's group-relative in-symbol byte offset onto the piece's HighVariable
/// — exactly the triple [`crate::printc`]'s `push_partial_symbol_ir` walks to emit
/// `root.field`.  The piece's offset is `piece->getOffset() + group->getSymbolOffset()`
/// (C++ `HighVariable::setSymbol`, `variable.cc:259`), where the group offset comes
/// from `establishGroupSymbolOffset` on the root.
///
/// `linkSymbol(nameRep)` on the root is the on-demand naming the kuna loop performs
/// here directly: if the root already carries a `kuna_name` (a container symbol or a
/// previously-allocated `vN`), reuse it; otherwise allocate the next `vN` (consuming
/// `base`) and bind it — pieces themselves never consume a `vN`.  Returns `true` when
/// the piece was bound (caller skips the `vN` allocator).
fn bind_proto_partial_piece(
    data: &mut Funcdata,
    piece_vn: crate::seams::VarnodeId,
    piece_high: crate::seams::HighVariableId,
    base: &mut int4,
) -> bool {
    // Varnode *rootVn = PieceNode::findRoot(vn);  if (rootVn == vn) return;
    let root_vn = data.piece_find_root(piece_vn);
    if root_vn == piece_vn {
        return false;
    }
    let root_high = match data.vbank().get(root_vn).and_then(|v| v.get_high()) {
        Some(h) => h,
        None => return false,
    };
    // if (!rootHigh->isSameGroup(high)) return;
    if !data.high_bank().is_same_group(root_high, piece_high) {
        return false;
    }
    // Symbol *sym = linkSymbol(rootHigh->getNameRepresentative());  — name the root
    // on demand.  Reuse an existing name (container symbol / earlier vN); otherwise
    // allocate the next vN and bind it.  The root's struct type is the partial-symbol
    // type the piece renders through.
    let root_name = match data.high_bank().get(root_high).and_then(|h| h.kuna_name()) {
        Some(n) => n.to_string(),
        None => {
            let name = format!("v{base}");
            *base += 1;
            if let Some(h) = data.high_bank_mut().get_mut(root_high) {
                h.set_kuna_name(name.clone());
            }
            name
        }
    };
    // The root struct data-type (the `sym->getType()` stand-in the piece walks); read
    // from the root's representative Varnode (its `foo`/`fooshort` type).
    let root_rep = data.high_name_representative(root_high).unwrap_or(root_vn);
    let root_type = data.vbank().get(root_rep).map(|v| v.get_type().clone());
    // rootHigh->establishGroupSymbolOffset();  — write the group's symbol offset so
    // every piece can derive its own in-symbol offset.  On the invariant-violation
    // Err (off < 0), bail rather than bind a bogus offset.
    if data.high_bank_mut().establish_group_symbol_offset(root_high).is_err() {
        return false;
    }
    // C++ `HighVariable::setSymbol` (`variable.cc:259`): the piece's in-symbol byte
    // offset is `piece->getOffset() + piece->getGroup()->getSymbolOffset()`.
    let piece_group_off = match data.high_bank().high_piece_id(piece_high) {
        Some(p) => data.high_bank().piece_offset(p),
        None => return false,
    };
    let group_sym_off = data.high_bank().group_symbol_offset(root_high);
    let sym_off = piece_group_off + group_sym_off;
    // vn->setSymbolEntry(...): bind the root's name + struct type + the piece's
    // in-symbol offset onto the piece's HighVariable.
    if let Some(h) = data.high_bank_mut().get_mut(piece_high) {
        h.set_kuna_name(root_name);
        h.set_symbol_offset(sym_off);
        if let Some(t) = root_type {
            h.set_symbol_type(t);
        }
    }
    true
}

fn name_local_highs_angr(data: &mut Funcdata) {
    use crate::seams::HighVariableId;
    // Materialize the recovered/locked parameters as Symbols in the local scope
    // (C++ `ProtoStoreSymbol::setInput` did this at recovery time; the kuna
    // `ProtoStoreInternal` does not, so it is done here before the walk).
    data.link_proto_params();

    // Iterate Varnodes in C++ location order; hit each high once at its name
    // representative (the highest-priority member), matching `linkSymbols`'
    // `getNameRepresentative()` dedup.
    let vlist: Vec<crate::seams::VarnodeId> = data.vbank().iter_loc().collect();
    let mut base: int4 = 1;
    // C++ `ActionNameVars::apply`'s namerec rename (coreaction.cc:3087-3094) for the
    // spacebase `&symbol` references recorded by `linkSpacebaseSymbol` (3016): rename
    // each undefined LOCAL whole-symbol addressed by a `PTRSUB(spacebase, off)` to its
    // `buildDefaultName` (`v<base++>`) BEFORE the per-space body naming loop, so a body
    // member-access query reads the final `v1` from the database (the shared-Symbol
    // semantics) and the end-of-pass attach renders `&v1` / `v1.b`.  Shares `base` so
    // the spacebase-referenced symbols are numbered ahead of the body-only locals,
    // matching the C++ namerec ordering.
    data.name_undefined_spacebase_symbols(&mut base);
    let mut seen: std::collections::BTreeSet<HighVariableId> = std::collections::BTreeSet::new();
    for vn in vlist {
        let high = match data.vbank().get(vn).and_then(|v| v.get_high()) {
            Some(h) => h,
            None => continue,
        };
        if seen.contains(&high) {
            continue;
        }
        // C++ `if (curvn->isFree()) continue;`
        if data.vbank().get(vn).map(|v| v.is_free()).unwrap_or(true) {
            continue;
        }
        // Hit each high only at its name representative (C++ `linkSymbols`:
        // `if (vn != high->getNameRepresentative()) continue;`).
        let name_rep = data.high_name_representative(high);
        if name_rep != Some(vn) {
            continue;
        }
        seen.insert(high);
        // C++ `if (!high->hasName()) continue;` — the gate that admits parameters,
        // mapped/promoted locals, and ordinary named locals while excluding
        // implied/non-coverable and the unaffected-stackpointer high.
        if !data.high_has_name(high) {
            continue;
        }
        // Already named? (idempotent re-run / inherited name.)
        if data.high_bank().get(high).map(|h| h.kuna_name().is_some()).unwrap_or(false) {
            continue;
        }
        // C++ `Funcdata::linkSymbol` (`funcdata_varnode.cc:1180-1181`): a proto-partial
        // Varnode is a piece getting PIECEd into a larger structure (a struct-by-value
        // return / param built by `RulePieceStructure`).  Before the generic symbol
        // query, `linkProtoPartial` (`funcdata_varnode.cc:1153`) walks up to the CONCAT
        // root, binds the root's symbol, and assigns the SAME symbol to the piece —
        // so the piece renders `root.field` (e.g. `v1.A`) rather than its own raw `vN`
        // / `dat_N`.  Pieces do NOT consume a `vN` slot (the root keeps `v1`).  This
        // branch reproduces that against the kuna naming model (the name + group-
        // relative in-symbol offset + root struct type are bound directly on the
        // piece's HighVariable, the `vn->setSymbolEntry(sym->getFirstWholeMap())`
        // stand-in) and `continue`s, skipping the `vN` allocator below.
        let nr = name_rep.unwrap();
        if data.vbank().get(nr).map(|v| v.is_proto_partial()).unwrap_or(false)
            && bind_proto_partial_piece(data, nr, high, &mut base)
        {
            continue;
        }
        let (v_persist, v_addr, v_size, v_input, v_addrtied, v_constant) =
            match data.vbank().get(name_rep.unwrap()) {
                Some(v) => (
                    v.is_persist(),
                    v.get_addr().clone(),
                    v.get_size(),
                    v.is_input(),
                    v.is_addr_tied(),
                    v.is_constant(),
                ),
                None => continue,
            };
        // C++ `Funcdata::linkSymbol` (`funcdata_varnode.cc:1177`): query the local
        // map for the SMALLEST CONTAINING SymbolEntry of the representative's BASE
        // BYTE (`queryProperties(vn->getAddr(), 1, usepoint)` — size 1, the
        // `findContainer` lookup, NOT the loose `findOverlap`).  A hit routes
        // through `handleSymbolConflict` (`funcdata_varnode.cc:1018`); the result
        // binds the high to that Symbol's display name (+ the in-symbol byte offset
        // for an array/struct member access) — what gives the body its
        // `ptr`/`a`/`b`/`i`.
        let container = data
            .get_scope_local()
            .and_then(|lm| lm.query_container_for_link(&v_addr));
        if let Some(info) = container {
            // C++ `handleSymbolConflict(entry, vn)` (`funcdata_varnode.cc:1018`):
            //   if (vn->isInput() || vn->isAddrTied() || vn->isPersist() ||
            //       vn->isConstant() || entry->isDynamic())  -> reuse the entry.
            // (`entry->isDynamic()` is `entry->getAddr().isInvalid()`; a mapped
            // local entry is never dynamic here, so the predicate reduces to the
            // four Varnode flags.)
            let reuse_directly = v_input || v_addrtied || v_persist || v_constant;
            // The `handleSymbolConflict` scan only matters when the representative's
            // storage genuinely DIFFERS in width from the containing entry — the
            // `float8` lane (8 bytes at XMM0_Qa) reaching into the `float4 a`
            // parameter entry (4 bytes at XMM0's base).  When the rep occupies the
            // entry's EXACT (addr,size) it is itself a `beginLoc(entry->getSize(),
            // entry->getAddr())` member, so in faithful C++ it would already be
            // merged into the parameter high and no conflict would be found (the
            // scan skips same-high members); only the size-mismatch case produces a
            // distinct narrower entry the rep cannot coalesce with.  Gating on the
            // size mismatch reproduces that C++ outcome exactly (`funcdata_varnode.cc
            // :1031` `otherVn->getSize() != entry->getSize()`) without depending on
            // whether the rust merge happened to unify equal-width siblings.
            let size_mismatch = v_size != info.entry_size;
            let conflict = if reuse_directly || !size_mismatch {
                false
            } else {
                // Scan `beginLoc(entry->getSize(), entry->getAddr())..endLoc(...)`
                // for an `otherVn` of EXACTLY the entry's size/addr that lives in a
                // DIFFERENT HighVariable — a genuine storage conflict (e.g. the
                // `float4 a` parameter Varnode sharing XMM0's base with this
                // `float8` lane high).  Such a conflict spawns a fresh dynamic
                // Symbol (`buildDynamicSymbol`) so the lane is named `vN`, never the
                // parameter's `a`.
                let mut found = false;
                for other in
                    data.vbank().iter_loc_size_addr(info.entry_size, &info.entry_addr)
                {
                    let other_high =
                        data.vbank().get(other).and_then(|v| v.get_high());
                    if let Some(oh) = other_high {
                        if oh != high {
                            found = true;
                            break;
                        }
                    }
                }
                found
            };
            if !conflict {
                // Reuse the containing entry's Symbol (the parameter / mapped
                // local).  `resolve_default_name`'s namerec rename
                // (coreaction.cc:3087-3094) still applies for an undefined-named
                // whole-symbol cover: rerun it so a promoted scalar stack local
                // renders `v1` rather than `$$undefNNN`.  It re-queries the same
                // entry via `findOverlap`; for a non-conflicting hit the two queries
                // agree.
                let resolved = data
                    .get_scope_local_mut()
                    .and_then(|lm| lm.resolve_default_name(&v_addr, v_size, &mut base));
                let (sym_name, sym_off, sym_type) = match resolved {
                    Some(t) => t,
                    None => (info.display_name, info.sym_off, info.sym_type),
                };
                if let Some(h) = data.high_bank_mut().get_mut(high) {
                    h.set_kuna_name(sym_name);
                    h.set_symbol_offset(sym_off);
                    if let Some(t) = sym_type {
                        h.set_symbol_type(t);
                    }
                }
                continue;
            }
            // Conflict: C++ `buildDynamicSymbol(vn)` creates a fresh dynamic Symbol
            // with an undefined name, which `ActionNameVars` then routes to the
            // angr `vN` arm.  Fall through to the `vN` tail below (the high acquires
            // no parameter name).  `info` is intentionally dropped.
            let _ = info.category;
        }
        // No covering Symbol.  In C++ `linkSymbols` (coreaction.cc:3061) this is the
        // point where `data.linkSymbol(vn)` *creates* a fresh local Symbol for the
        // high (`funcdata_varnode.cc:1194-1201`: any **non-persist** Varnode —
        // register, unique, stack, or non-param input — gets `localmap->addSymbol`
        // with an empty/undefined name) and the high is pushed onto `namerec`.
        // `ActionNameVars` (coreaction.cc:3092) then renames that undefined Symbol
        // via `Scope::buildDefaultName(sym,base,vn)`, whose angr arm
        // (database.cc:1764-1786) returns:
        //   * `aN`        for a function parameter   (resolved above, never here),
        //   * `dat_<addr>` for a **persistent** Varnode that is **not a register**
        //                  (`getRegisterName(...).empty()`), and
        //   * `v<base++>`  for EVERYTHING else — registers (incl. XMM lanes), uniques,
        //                  non-param inputs, unaffected/extraout storage.
        // The prior kuna gate (`v_input || v_persist || !v_addrtied`) admitted only
        // address-tied stack/return storage, so a transient register like `XMM0_Qa`
        // or a `Unique` temp fell straight through to `pushUnnamedLocation` and
        // rendered raw.  The faithful gate is the single `dat_` exclusion below.
        //
        // `dat_<addr>` is rendered by the unnamed-location tail (`pushUnnamedLocation`,
        // `kunaGlobalDataName`), so we simply *skip* naming a persistent non-register;
        // it must NOT acquire a `vN`.  A persistent **register** would still be `vN`,
        // matching the angr arm's `getRegisterName` guard.
        let spc = v_addr.get_space().expect("named high rep has no space");
        let is_register = data
            .get_arch()
            .manage()
            .register_lookup()
            .map(|rl| !rl.get_register_name(spc, v_addr.get_offset(), v_size).is_empty())
            .unwrap_or(false);
        // `buildDefaultName`'s `dat_<addr>` arm (database.cc:1778-1782) fires for a
        // **persistent** Varnode that is **not a register**.  In C++ that persist
        // flag is painted by `localmap->queryProperties` from the *global* scope's
        // range flags; the W4 global-scope queryProperties surface is a seam in this
        // port, so an input read of read-only global RAM reaches here with
        // `persist == false`.  The faithful proxy for "this is a global, route to
        // `dat_`" is the same predicate `PrintC::pushUnnamedLocation` uses to emit
        // the `dat_<addr>` token (`printc.rs::kuna_global_naming`): a **global data
        // space** (`IPTR_PROCESSOR`) address with **no register name**.  Registers
        // live in the same processor space but carry a register name, so they are
        // excluded and still get `vN`; uniques (`IPTR_INTERNAL`) and stack
        // (`IPTR_SPACEBASE`) locals are not global data and get `vN`.
        let is_global_data =
            spc.get_type() == kuna_base::space::spacetype::IPTR_PROCESSOR && !is_register;
        if v_persist || is_global_data {
            // C++ `Funcdata::linkSymbol` (`funcdata_varnode.cc:1190`) queries
            // `localmap->queryProperties`, whose `Scope::stackContainer` walks the
            // parent chain UP TO THE GLOBAL SCOPE.  A global RAM store mapped by
            // `map addr glob1` is owned by the global scope, so the query returns the
            // global Symbol and the high carries `glob1` / `globalfree` — exactly the
            // body LHS the oracle renders (`glob1 = 0`, `globalfree = 100`).  Without
            // this query a surviving global store falls to `pushUnnamedLocation` and
            // renders `dat_<addr>`.
            //
            // The merged kuna `localmap` is a detached `Database`; its global reach is
            // the `GlobalQuery` snapshot on `glb` (built after every `map addr`).
            // Address-tied global entries are valid at every usepoint, so an invalid
            // usepoint is faithful here (C++ `SymbolEntry::inUse` short-circuits on
            // address-tied).  A hit binds the high's name + symbol offset + type
            // identically to the local `resolve_default_name` branch above.
            let usepoint = kuna_base::address::Address::new_invalid();
            if let Some((sym_name, sym_off, sym_type)) =
                data.get_arch().name_for_global_varnode(&v_addr, v_size, &usepoint)
            {
                if let Some(h) = data.high_bank_mut().get_mut(high) {
                    h.set_kuna_name(sym_name);
                    h.set_symbol_offset(sym_off);
                    if let Some(t) = sym_type {
                        h.set_symbol_type(t);
                    }
                }
                continue;
            }
            continue; // dat_<addr> via the unnamed-location tail (global, not a local)
        }
        // Recovered *parameters* take the angr `aN` branch and are caught above by
        // `resolve_default_name` (the proto-param Symbols are materialized by
        // `link_proto_params`).  Any non-parameter storage reaching here — a
        // register def (`XMM0_Qa`), a `unique` temp, a stack local, or a
        // non-parameter input (unaffected/illegal/leftover) — is named `vN` exactly
        // like any other local, with no `isInput()` special-casing (the angr
        // `buildDefaultName` arm has none).
        let name = format!("v{base}");
        base += 1;
        if let Some(h) = data.high_bank_mut().get_mut(high) {
            h.set_kuna_name(name);
        }
    }

    // C++ `ActionNameVars::linkSymbols` (coreaction.cc:3028) walks the spacebase
    // Varnodes (const-space pass at :3040 + per-space `isSpacebase` pass at :3055)
    // and calls `linkSpacebaseSymbol` on each, decoding the `&symbol` references
    // encoded as `PTRSUB(spacebase, off)` and attaching the Symbol to the offset
    // constant's HighVariable (`Funcdata::linkSymbolReference` ->
    // `Varnode::setSymbolReference`).  This is the render payoff for the W10
    // RSP-input spacebase typing: without it `PTRSUB(sp,-0x64)` renders the raw
    // functional `PTRSUB(v1,...)`; with it the printer's `opPtrsub` SPACEBASE arm
    // finds `symbol != null` and renders `&a` / `&myval.b`.
    //
    // ORDER (the load-bearing detail): in C++ a single shared `Symbol` object is
    // attached to BOTH the offset-constant high and the stack-slot high, and the
    // undefined ones are renamed ONCE at the end of `apply` (`buildDefaultName`,
    // coreaction.cc:3092).  The render reads `getSymbol()->getDisplayName()` at
    // print time, so it always sees the FINAL name.  The kuna model binds the name
    // PER-HIGH off the database Symbol, so the spacebase pass must run AFTER the
    // main naming loop — by which point the main loop's `resolve_default_name` has
    // already renamed each undefined local Symbol to its `vN` in the database
    // (varmap.rs:1334).  `link_symbol_reference` then reads that final name back via
    // `query_container_for_link`, so an unmapped auto-local reference renders `&v3`
    // (the renamed local) rather than the raw `$$undefNN`, and a mapped reference
    // renders `&a` / `&myval.b`.  The spacebase pass touches only the disjoint
    // offset-constant highs, so its placement does not perturb the local naming.
    // `iter_loc` yields every space in C++ location order (const space first), so a
    // single walk reproduces both the const-space and per-space calls.
    let all_locs: Vec<crate::seams::VarnodeId> = data.vbank().iter_loc().collect();
    for vn in &all_locs {
        if data.vbank().get(*vn).map(|v| v.is_spacebase()).unwrap_or(false) {
            data.link_spacebase_symbol(*vn);
        }
    }
}

/// Choose a *name* for all high-level variables (C++ `ActionNameVars`,
/// `coreaction.cc:3076`).
pub struct ActionNameVars {
    base: ActionBase,
}

impl ActionNameVars {
    /// Construct in group `g` (C++ `ActionNameVars::ActionNameVars`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionNameVars {
            base: ActionBase::new(ruleflags::rule_onceperfunc, "namevars", g),
        })
    }
}

impl Action for ActionNameVars {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }
    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>> {
        if !grouplist.contains(self.get_group()) {
            return None;
        }
        Some(Box::new(ActionNameVars { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.cc:3076 — ActionNameVars::apply
        //   vector<Varnode *> namerec;
        //   linkSymbols(data, namerec);
        //   data.getScopeLocal()->recoverNameRecommendationsForSymbols();
        //   lookForBadJumpTables(data);
        //   lookForFuncParamNames(data, namerec);
        //   int4 base = 1;
        //   for (i = 0; i < namerec.size(); ++i):
        //       vn = namerec[i];
        //       sym = vn->getHigh()->getSymbol();
        //       if (sym->isNameUndefined()):
        //           scope = sym->getScope();
        //           newname = scope->buildDefaultName(sym, base, vn);
        //           scope->renameSymbol(sym, newname);
        //   data.getScopeLocal()->assignDefaultNames(base);
        //   return 0;
        //
        // The W4 ScopeLocal/`Symbol` surface (`getScopeLocal`/`buildDefaultName`/
        // `renameSymbol`, the HighVariable->Symbol attachment, the callspec list)
        // is absent in the merged tree.  We transcribe the OBSERVABLE result for
        // the angr default-naming scheme (`Scope::buildDefaultName`'s
        // `kunaAngrNaming` branch, database.cc:1764-1785): each nameable LOCAL
        // HighVariable gets `v` + a running base index.  A "nameable local" is the
        // C++ `linkSymbols` filter reduced to what the merged tree can express:
        // the high's name representative is addr-tied (in local scope), not an
        // input, not persist/global — i.e. exactly the locals that
        // `buildDefaultName` would route to the `vN` arm.  The name is bound on
        // the HighVariable (the `Symbol` stand-in; see `HighVariable::kuna_name`).
        name_local_highs_angr(data);
        0
    }
}

// =============================================================================
// ActionSetCasts (coreaction.hh:320, coreaction.cc:2812) — group "casts"
// =============================================================================

/// Place casts as needed by the high-level data-types of operands (C++
/// `ActionSetCasts`, `coreaction.cc:2812`).
///
/// The only merge-group action whose schedule group is `"casts"` rather than
/// `"merge"` (`coreaction.cc:6028`); the group is supplied by the caller, so the
/// constructor takes `g` verbatim like all the others.
pub struct ActionSetCasts {
    base: ActionBase,
}

impl ActionSetCasts {
    /// Construct in group `g` (C++ `ActionSetCasts::ActionSetCasts`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionSetCasts {
            base: ActionBase::new(ruleflags::rule_onceperfunc, "setcasts", g),
        })
    }
}

impl Action for ActionSetCasts {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }
    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>> {
        if !grouplist.contains(self.get_group()) {
            return None;
        }
        Some(Box::new(ActionSetCasts { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.cc:2812 — ActionSetCasts::apply.  The full driver +
        // helpers (castInput/castOutput/resolveUnion/checkPointerIssues/
        // insertPtrsubZero/testStructOffset0/tryResolution*/isOpIdentical), the
        // per-op getInputCast/getOutputToken surface and the FuncdataCastContext
        // bridge live in `crate::coreaction_casts` (`Funcdata::action_set_casts`).
        // The C++ `return 0` (full completion) is reproduced regardless of how
        // many casts were inserted; `count` accumulates inside the driver.
        let _count = data.action_set_casts();
        0
    }
}

// =============================================================================
// W8 schedule assembly helper
// =============================================================================

/// The merge-group leaf actions, in the C++ `universalAction` schedule order
/// (`coreaction.cc:6002-6028`), each constructed in the group string the C++
/// schedule passes.  The interleaved structuring wrappers (`ActionBlockStructure`
/// etc., from [`blockaction`](crate::blockaction)) and the dynamic/global symbol
/// actions are *not* part of this set; the W8 assembler interleaves them at the
/// schedule positions shown in the module docs.
pub fn merge_actions() -> Vec<Box<dyn Action>> {
    vec![
        ActionAssignHigh::boxed("merge"),
        ActionMergeRequired::boxed("merge"),
        ActionMarkExplicit::boxed("merge"),
        ActionMarkImplied::boxed("merge"), // must come BEFORE general merging
        ActionMergeMultiEntry::boxed("merge"),
        ActionMergeCopy::boxed("merge"),
        ActionDominantCopy::boxed("merge"),
        ActionMarkIndirectOnly::boxed("merge"), // after required, before speculative
        ActionMergeAdjacent::boxed("merge"),
        ActionMergeType::boxed("merge"),
        ActionHideShadow::boxed("merge"),
        ActionCopyMarker::boxed("merge"),
        ActionNameVars::boxed("merge"),
        ActionSetCasts::boxed("casts"),
    ]
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use kuna_base::address::Address;
    use kuna_base::space::{
        addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
    };

    use super::*;
    use crate::seams::Architecture;

    // Mirrors the coreaction_protos.rs test harness (funcdata_block fixtures).
    fn build_manager() -> AddrSpaceManager {
        let mut m = AddrSpaceManager::new();
        m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
        m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
        m.insert_space(Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "ram",
            false,
            8,
            1,
            2,
            addrspace_flags::hasphysical,
            1,
            1,
        )))
        .unwrap();
        m
    }

    fn build_fd() -> Funcdata {
        let manage = build_manager();
        let glb = Rc::new(Architecture::new(manage));
        let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
        let addr = Address::new(ram, 0x1000);
        Funcdata::new("func", "func", glb, addr, 0x10000000, 0x40).unwrap()
    }

    /// Every merge-group leaf carries the exact C++ `name()` string and is
    /// `rule_onceperfunc`.
    #[test]
    fn names_and_flags_match_cpp() {
        let cases: Vec<(Box<dyn Action>, &str)> = vec![
            (ActionAssignHigh::boxed("g0"), "assignhigh"),
            (ActionMergeRequired::boxed("g0"), "mergerequired"),
            (ActionMarkExplicit::boxed("g0"), "markexplicit"),
            (ActionMarkImplied::boxed("g0"), "markimplied"),
            (ActionMergeMultiEntry::boxed("g0"), "mergemultientry"),
            (ActionMergeCopy::boxed("g0"), "mergecopy"),
            (ActionDominantCopy::boxed("g0"), "dominantcopy"),
            (ActionMarkIndirectOnly::boxed("g0"), "markindirectonly"),
            (ActionMergeAdjacent::boxed("g0"), "mergeadjacent"),
            (ActionMergeType::boxed("g0"), "mergetype"),
            (ActionHideShadow::boxed("g0"), "hideshadow"),
            (ActionCopyMarker::boxed("g0"), "copymarker"),
            (ActionNameVars::boxed("g0"), "namevars"),
            (ActionSetCasts::boxed("g0"), "setcasts"),
        ];
        for (act, expect) in &cases {
            assert_eq!(act.get_name(), *expect, "name mismatch");
            assert_eq!(act.get_group(), "g0", "group must be the ctor's `g`");
            // All fourteen are rule_onceperfunc in the C++ constructors.
            assert_eq!(
                act.base().flags,
                ruleflags::rule_onceperfunc,
                "{} must be rule_onceperfunc",
                expect
            );
        }
    }

    /// `clone_filtered` mirrors the C++ `clone(grouplist)` group filter: present
    /// iff the grouplist contains the action's group, else dropped.
    #[test]
    fn clone_filtered_obeys_grouplist() {
        let act = ActionMergeRequired::boxed("merge");
        // In-list: cloned, name and group preserved.
        let yes = ActionGroupList::from_names(["merge", "casts"]);
        let cl = act.clone_filtered(&yes).expect("present when grouplist contains group");
        assert_eq!(cl.get_name(), "mergerequired");
        assert_eq!(cl.get_group(), "merge");
        // Out-of-list: dropped (the C++ `return (Action *)0`).
        let no = ActionGroupList::from_names(["analysis"]);
        assert!(act.clone_filtered(&no).is_none(), "dropped when group absent");

        // ActionSetCasts lives in its own "casts" group.
        let casts = ActionSetCasts::boxed("casts");
        assert!(casts.clone_filtered(&yes).is_some());
        assert!(casts
            .clone_filtered(&ActionGroupList::from_names(["merge"]))
            .is_none());
    }

    /// `ActionAssignHigh::apply` is realized: it turns on the HighVariable layer
    /// (`Funcdata::set_high_level`) and is idempotent (the C++ `setHighLevel`
    /// early-returns if `isHighOn`).  It signals no change (C++ `return 0`).
    #[test]
    fn assign_high_turns_on_high_layer() {
        let mut fd = build_fd();
        let mut ctx = ActionContext::new();
        assert!(!fd.is_high_on(), "high layer starts off");

        let mut act = ActionAssignHigh::boxed("merge");
        let r = act.apply(&mut fd, &mut ctx);
        assert_eq!(r, 0, "apply returns 0");
        assert_eq!(act.base().count, 0, "setHighLevel signals no change");
        assert!(fd.is_high_on(), "high layer is on after assignhigh");

        // Idempotent: a second pass changes nothing.
        let r2 = act.apply(&mut fd, &mut ctx);
        assert_eq!(r2, 0);
        assert!(fd.is_high_on());
    }

    /// The merge-delegation actions are seamed (no `getMerge()` bridge yet): each
    /// `apply` runs cleanly and signals no change on a hand-built empty function.
    #[test]
    fn seamed_merge_actions_apply_as_noop() {
        let mut fd = build_fd();
        let mut ctx = ActionContext::new();
        let mut acts: Vec<Box<dyn Action>> = vec![
            ActionMergeRequired::boxed("merge"),
            ActionMergeMultiEntry::boxed("merge"),
            ActionMergeCopy::boxed("merge"),
            ActionDominantCopy::boxed("merge"),
            ActionMarkIndirectOnly::boxed("merge"),
            ActionMergeAdjacent::boxed("merge"),
            ActionMergeType::boxed("merge"),
            ActionCopyMarker::boxed("merge"),
        ];
        for act in &mut acts {
            let r = act.apply(&mut fd, &mut ctx);
            assert_eq!(r, 0, "{} seam returns 0", act.get_name());
            assert_eq!(act.base().count, 0, "{} seam makes no change", act.get_name());
        }
    }

    /// The marking/name/cast actions are seamed (HighVariable/symbol/cast bridge
    /// absent): each `apply` runs cleanly and signals no change.
    #[test]
    fn seamed_marking_actions_apply_as_noop() {
        let mut fd = build_fd();
        let mut ctx = ActionContext::new();
        let mut acts: Vec<Box<dyn Action>> = vec![
            ActionMarkExplicit::boxed("merge"),
            ActionMarkImplied::boxed("merge"),
            ActionHideShadow::boxed("merge"),
            ActionNameVars::boxed("merge"),
            ActionSetCasts::boxed("casts"),
        ];
        for act in &mut acts {
            let r = act.apply(&mut fd, &mut ctx);
            assert_eq!(r, 0, "{} seam returns 0", act.get_name());
            assert_eq!(act.base().count, 0, "{} seam makes no change", act.get_name());
        }
    }

    /// The W8 schedule helper enumerates all fourteen leaves in C++ schedule
    /// order, in the schedule's group strings (`"merge"` except `setcasts`).
    #[test]
    fn merge_actions_schedule_order_and_groups() {
        let acts = merge_actions();
        let names: Vec<&str> = acts.iter().map(|a| a.get_name()).collect();
        assert_eq!(
            names,
            vec![
                "assignhigh",
                "mergerequired",
                "markexplicit",
                "markimplied",
                "mergemultientry",
                "mergecopy",
                "dominantcopy",
                "markindirectonly",
                "mergeadjacent",
                "mergetype",
                "hideshadow",
                "copymarker",
                "namevars",
                "setcasts",
            ]
        );
        // Group strings: all "merge" except the final "casts".
        for a in &acts[..acts.len() - 1] {
            assert_eq!(a.get_group(), "merge");
        }
        assert_eq!(acts.last().unwrap().get_group(), "casts");
    }
}

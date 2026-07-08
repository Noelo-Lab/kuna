//! Port of `decompiler/cpp/kuna_loweredswitch.{cc,hh}` — lowered
//! comparison-cascade -> switch recovery (port of angr's
//! `LoweredSwitchSimplifier`; Basque et al., USENIX Security 2024 "SAILR", S2).
//!
//! # The structural gap
//!
//! GCC lowers a dense `switch (V)` over a small variable into a balanced
//! BINARY-SEARCH TREE of `if (V == c) ... else if (V < k) ...` comparisons (no
//! jump table).  Ghidra renders a `switch` ONLY from a `CPUI_BRANCHIND` op
//! backed by a `JumpTable` (an S2 artifact recovered from a lifted indirect
//! jump).  When the compiler lowered the switch to comparisons there is no
//! `BRANCHIND`, so Ghidra (like stock angr) renders a deep if/else chain.  The
//! kuna pass detects the comparison cascade and MANUFACTURES the S2 artifact (a
//! `BRANCHIND` + a hand-built, pre-labelled `JumpTable`) so the existing
//! structurer + printer emit a `switch` — a stage-model feedback edge
//! (`docs/divergences.md`).
//!
//! To keep SSA/phi state correct through the CFG surgery the port uses Ghidra's
//! multistage jump-table restart idiom:
//!
//!   - [`ActionLowerSwitchDetect`] runs LATE (after `ActionSwitchNorm`) on the
//!     fully simplified CFG.  It only READS: it detects the cascade, records it
//!     in a survives-`clear()` side table (keyed by function identity,
//!     addresses only — no `Varnode`/`Block` handles), and requests a restart.
//!   - `ActionLowerSwitchInstall` runs EARLY on the restart, BEFORE
//!     `ActionHeritage`, when blocks exist but SSA does not.  It performs the CFG
//!     surgery (`Funcdata::kunaInstallLoweredSwitch`) so the following
//!     `ActionHeritage` rebuilds phi from scratch over the corrected CFG.
//!
//! The decision is a P0 assertion: `option loweredswitch on|off`.  The kuna Rust
//! port ships this **default-on** (`architecture.rs:418` sets
//! `recover_lowered_switch = true`; the angr-port DIV default), so the Detect
//! Action's [`enabled`](ActionLowerSwitchDetect) gate carries `true` in the
//! shipped [`specs`].
//!
//! # What this port covers (and the install STUB it cannot)
//!
//! Ported faithfully (self-contained, read-only over the merged tree):
//!
//! * [`ELEM_LOWEREDSWITCH`] — the marshaling element id (kuna 4000+ range).
//! * [`KunaLoweredSwitchRecord`] / [`KunaLsKey`] / [`key_for_func`] — the
//!   side-table record and function-identity key (the C++ `loweredStore`
//!   key/value shapes; identical to `kuna_restartlog`'s `KunaFuncKey`).
//! * [`KunaLoweredSwitchStore`] — the sticky side table.  C++ holds a
//!   file-static `std::map`; the Rust port models it as an owned struct (the
//!   same convention as the `RestartLog` port) carried inside the Detect Action,
//!   whose lifetime (per `ActionDatabase`) matches the file-static's
//!   survives-`clear()` semantics.  [`KunaLoweredSwitchStore::has_record`] is
//!   `kunaLoweredSwitchHasRecord`.
//! * The detection: [`canon_switch_var`], [`is_pure_compare_block`],
//!   [`analyze_cmp`] ([`CmpNode`]), [`target_start`], [`recover_cascade`],
//!   [`advance_past_guards`], and [`ActionLowerSwitchDetect::apply`] — the
//!   `ActionLowerSwitchDetect::apply` head-finding + cascade walk, transcribed.
//! * [`OptionLowerSwitch`] — the `loweredswitch on|off` parse + message
//!   (`OptionLowerSwitch::apply`).
//!
//! ## The install half (`ActionLowerSwitchInstall` +
//! `Funcdata::kuna_install_lowered_switch`)
//!
//! The CFG surgery that turns a recorded cascade into a real `BRANCHIND` +
//! `JumpTable` runs on the restart pass, pre-SSA, over the now-ported surfaces:
//!
//!   * [`Funcdata::get_heritage_pass`](crate::funcdata::Funcdata::get_heritage_pass)
//!     — the install gate `get_heritage_pass() != 0` selects the pre-SSA window
//!     (the restart re-clears heritage, so the first mainloop iteration runs at
//!     pass 0).
//!   * the live `jumpvec` registry — the constructed
//!     [`JumpTable`](crate::jumptable::JumpTable) is pushed as a real entry,
//!     linked to the synthesized BRANCHIND.
//!   * [`Funcdata::remove_unreachable_blocks`](crate::funcdata::Funcdata::remove_unreachable_blocks)
//!     — sweeps the orphaned comparison-cascade spine.
//!   * [`JumpTable::kuna_set_trivial_model`](crate::jumptable::JumpTable::kuna_set_trivial_model)
//!     — attaches a [`JumpModelTrivial`](crate::jumptable::JumpModelTrivial) so the
//!     model-bearing queries behave as for a recovered switch.
//!
//! The Detect half records the hint on the pre-restart pass and requests the
//! restart; the Install half reads the *same* (shared) store on the restart and
//! manufactures the artifact.  The structurer + printer then emit the `switch`.
//!
//! ## STUB(W9) — `ArchOption` / pass-schedule wiring
//!
//! Upstream `OptionLowerSwitch : ArchOption` is registered with the option
//! database; the two Actions are inserted into `ActionDatabase::universalAction`
//! at their schedule slots.  Both registrations are the W9 assembler's job
//! (`coreaction.cc`/`options.cc` are not owned here): the option parse is
//! exposed as [`OptionLowerSwitch::apply`] and the Detect Action carries its
//! gate explicitly.  Noted in the structured losses.

use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use kuna_base::address::{calc_mask, Address};
use kuna_base::error::KunaResult;
use kuna_base::space::{spacetype, AddrSpace};
use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::block::{block_get_start, BlockType};
use crate::funcdata::Funcdata;
use crate::options::on_or_off;
use crate::context::{BlockId, OpId, VarnodeId};

use kuna_base::marshal::ElementId;

/// Marshaling element `<loweredswitch>` (kuna).  ElementIds live in the 4000+
/// range (C++ `ELEM_LOWEREDSWITCH = ElementId("loweredswitch",4019)`).
pub const ELEM_LOWEREDSWITCH: ElementId = ElementId::new("loweredswitch", 4019);

//===========================================================================
// Side table: the recovered lowered-switch hint, keyed by function identity.
//===========================================================================

/// One recovered lowered switch within a function (C++ `KunaLoweredSwitchRecord`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KunaLoweredSwitchRecord {
    /// Op address of the cascade head (hosts the synthetic `BRANCHIND`).
    pub branch_addr: Address,
    /// Switch variable storage (a fallback locator for V).
    pub var_addr: Address,
    /// Switch variable size in bytes.
    pub var_size: int4,
    /// Case label constants (parallel to `case_targets`).
    pub case_vals: Vec<uintb>,
    /// Case body block start addresses.
    pub case_targets: Vec<Address>,
    /// Default block start address.
    pub default_target: Address,
}

/// Side-table key: identifies a function without holding clearable handles
/// (C++ `KunaLSKey`; identical shape to `kuna_restartlog`'s `KunaFuncKey`).
///
/// The C++ stores the raw `Architecture *` (`glb`) in the key; here the function
/// is identified by its entry address (space index + offset) which is stable
/// across `clear()`.  A single decompiler run has one `Architecture`, so the
/// entry address alone is the live discriminator (the C++ `glb` field guards
/// against cross-program collisions, irrelevant within one run).
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct KunaLsKey {
    /// Entry address space index, or -1 if the entry has no space
    /// (C++ `(entry.getSpace() != 0) ? ->getIndex() : -1`).
    pub space_index: int4,
    /// Entry address offset (C++ `entry.getOffset()`).
    pub offset: uintb,
}

/// The side-table key for `fd` — C++ `keyForFunc` (`kuna_loweredswitch.cc:73`).
pub fn key_for_func(fd: &Funcdata) -> KunaLsKey {
    // const Address &entry( fd.getAddress() );
    let entry = fd.get_address();
    // key.spaceindex = (entry.getSpace() != 0) ? entry.getSpace()->getIndex() : -1;
    let space_index = match entry.get_space() {
        Some(s) => s.get_index(),
        None => -1,
    };
    // key.offset = entry.getOffset();
    KunaLsKey { space_index, offset: entry.get_offset() }
}

/// The sticky lowered-switch side table (C++ file-static `loweredStore`).
///
/// Survives `Funcdata::clear()` and re-decompilation: it holds only addresses
/// and constants, never `Varnode`/`Block`/`Funcdata` handles.  Once a cascade is
/// discovered (late, on the simplified CFG) every subsequent decompilation of
/// that function re-applies it pre-SSA without another restart — exactly like a
/// manual jump-table override.
#[derive(Clone, Debug, Default)]
pub struct KunaLoweredSwitchStore {
    table: BTreeMap<KunaLsKey, Vec<KunaLoweredSwitchRecord>>,
}

impl KunaLoweredSwitchStore {
    /// A new, empty store.
    pub fn new() -> KunaLoweredSwitchStore {
        KunaLoweredSwitchStore::default()
    }

    /// (kuna) Has a lowered-switch record already been stored for `fd`?
    /// (C++ `kunaLoweredSwitchHasRecord`).  True on the post-restart pass (the
    /// detection self-gate / install trigger).
    pub fn has_record(&self, fd: &Funcdata) -> bool {
        // iter = loweredStore.find(keyForFunc(fd));
        // return (iter != loweredStore.end() && !(*iter).second.empty());
        match self.table.get(&key_for_func(fd)) {
            Some(v) => !v.is_empty(),
            None => false,
        }
    }

    /// The recorded cascades for `fd` (empty if none) — used by the install half.
    pub fn records(&self, fd: &Funcdata) -> &[KunaLoweredSwitchRecord] {
        match self.table.get(&key_for_func(fd)) {
            Some(v) => v.as_slice(),
            None => &[],
        }
    }

    /// Append a recovered record for `fd` — C++
    /// `loweredStore[keyForFunc(data)].push_back(rec)`.
    pub fn push(&mut self, fd: &Funcdata, rec: KunaLoweredSwitchRecord) {
        self.table.entry(key_for_func(fd)).or_default().push(rec);
    }
}

/// A shared handle to the sticky lowered-switch side table.
///
/// The C++ `loweredStore` is a single file-static `std::map` that both
/// `ActionLowerSwitchDetect` (writes) and `ActionLowerSwitchInstall` (reads)
/// reference.  The Rust port models that single shared store as a
/// reference-counted [`RefCell`]: the schedule builder constructs one
/// [`SharedLoweredSwitchStore`] and hands a clone to each Action, so the hint
/// the Detect half records on pass 1 is visible to the Install half on the
/// restart pass.  Cloning the handle (including through `clone_filtered`) shares
/// the same inner store, matching the file-static's single-instance semantics.
pub type SharedLoweredSwitchStore = Rc<RefCell<KunaLoweredSwitchStore>>;

/// Construct a fresh, empty shared lowered-switch store.
pub fn new_shared_store() -> SharedLoweredSwitchStore {
    Rc::new(RefCell::new(KunaLoweredSwitchStore::new()))
}

//===========================================================================
// Detection (read-only) on the simplified CFG
//===========================================================================

/// Peel transparent ops to a canonical switch-variable Varnode (C++ static
/// `canonSwitchVar`).
///
/// `COPY`/`CAST`/zero-extend/sign-extend/zero-offset `SUBPIECE` all preserve
/// switch-variable identity (angr's `StableVarExprHasher` analog).  The cascade
/// runs pre-merge, so the canonicalized [`VarnodeId`] is the identity key
/// (the C++ pre-merge pointer-identity argument, exact here too).
fn canon_switch_var(data: &Funcdata, mut vn: VarnodeId) -> VarnodeId {
    // for(int4 guard=0;guard<8;++guard) {
    for _guard in 0..8 {
        // if (vn->isConstant()) return vn;
        if vn_is_constant(data, vn) {
            return vn;
        }
        // if (!vn->isWritten()) return vn;
        if !vn_is_written(data, vn) {
            return vn;
        }
        // PcodeOp *def = vn->getDef();  OpCode oc = def->code();
        let def = vn_def(data, vn).expect("canonSwitchVar: written vn has a def");
        let oc = op_code(data, def);
        // if (oc==COPY||oc==CAST||oc==INT_ZEXT||oc==INT_SEXT) { vn = def->getIn(0); continue; }
        if oc == OpCode::CPUI_COPY
            || oc == OpCode::CPUI_CAST
            || oc == OpCode::CPUI_INT_ZEXT
            || oc == OpCode::CPUI_INT_SEXT
        {
            match op_in(data, def, 0) {
                Some(v) => {
                    vn = v;
                    continue;
                }
                None => return vn,
            }
        }
        // if (oc==SUBPIECE && def->getIn(1)->isConstant() && def->getIn(1)->getOffset()==0) {
        //   vn = def->getIn(0); continue; }
        if oc == OpCode::CPUI_SUBPIECE {
            if let Some(in1) = op_in(data, def, 1) {
                if vn_is_constant(data, in1) && vn_offset(data, in1) == 0 {
                    match op_in(data, def, 0) {
                        Some(v) => {
                            vn = v;
                            continue;
                        }
                        None => return vn,
                    }
                }
            }
        }
        // return vn;
        return vn;
    }
    vn
}

/// Analysis of one comparison-on-constant block (C++ `struct CmpNode`).
///
/// "Same switch variable" identity is the canonicalized [`VarnodeId`].  This
/// runs pre-merge, so HighVariables do not exist yet; the cascade reads one SSA
/// Varnode (defined once, not redefined between comparisons), so the id is exact.
#[derive(Clone, Copy, Debug)]
struct CmpNode {
    /// True if this block is a clean comparison-on-constant.
    valid: bool,
    /// True for `INT_EQUAL`/`INT_NOTEQUAL` (emits a case).
    is_equality: bool,
    /// The canonicalized variable operand (the identity key).
    var: Option<VarnodeId>,
    /// The constant operand value.
    cval: uintb,
    /// Successor taken when V == cval (equality only).
    match_out: Option<BlockId>,
    /// Continuation successor A (range: both continue).
    cont_a: Option<BlockId>,
    /// Continuation successor B (range only).
    cont_b: Option<BlockId>,
    /// (range only) Successor taken when `V > range_value` — angr's `gt_node`
    /// (`lowered_switch_simplifier.py:_find_switch_variable_comparison_type_c`).
    gt_out: Option<BlockId>,
    /// (range only) Successor taken when `V <= range_value` — angr's `le_node`.
    le_out: Option<BlockId>,
    /// (range only) The normalized "gt-form" threshold: gt_out ⟺ `V > range_value`,
    /// le_out ⟺ `V <= range_value` (angr's post-adjustment `value` in type-c).
    range_value: uintb,
}

impl CmpNode {
    /// C++ `CmpNode()` default constructor.
    fn new() -> CmpNode {
        CmpNode {
            valid: false,
            is_equality: false,
            var: None,
            cval: 0,
            match_out: None,
            cont_a: None,
            cont_b: None,
            gt_out: None,
            le_out: None,
            range_value: 0,
        }
    }
}

/// Is `bl` a "pure" comparison spine block (no side effects, no escaping defs)?
/// (C++ static `isPureCompareBlock`).
///
/// Only such blocks may be collapsed when the switch is installed.  Case bodies
/// (which set flags / call) and the default block fail this and so naturally
/// bound the cascade as leaves.
fn is_pure_compare_block(data: &Funcdata, bl: BlockId) -> bool {
    // for(iter=bb->beginOp();iter!=bb->endOp();++iter) {
    for op in data.bb_ops(bl) {
        // OpCode oc = (*iter)->code();
        match op_code(data, op) {
            OpCode::CPUI_CBRANCH
            | OpCode::CPUI_INT_EQUAL
            | OpCode::CPUI_INT_NOTEQUAL
            | OpCode::CPUI_INT_SLESS
            | OpCode::CPUI_INT_SLESSEQUAL
            | OpCode::CPUI_INT_LESS
            | OpCode::CPUI_INT_LESSEQUAL
            | OpCode::CPUI_INT_ZEXT
            | OpCode::CPUI_INT_SEXT
            | OpCode::CPUI_SUBPIECE
            | OpCode::CPUI_COPY
            | OpCode::CPUI_CAST
            | OpCode::CPUI_MULTIEQUAL
            | OpCode::CPUI_INT_AND
            | OpCode::CPUI_BOOL_NEGATE => {}
            // default: return false;
            _ => return false,
        }
    }
    true
}

/// Decode a block as a comparison-on-constant cascade node (C++ static
/// `analyzeCmp`).
fn analyze_cmp(data: &Funcdata, bl: BlockId) -> CmpNode {
    let mut res = CmpNode::new();
    // PcodeOp *cb = bb->lastOp();
    let cb = match data.bb_op_tail(bl) {
        Some(o) => o,
        None => return res,
    };
    // if (cb == 0 || cb->code() != CPUI_CBRANCH) return res;
    if op_code(data, cb) != OpCode::CPUI_CBRANCH {
        return res;
    }
    // if (bb->sizeOut() != 2) return res;
    if block_size_out(data, bl) != 2 {
        return res;
    }
    // Varnode *boolvn = cb->getIn(1);
    let boolvn = match op_in(data, cb, 1) {
        Some(v) => v,
        None => return res,
    };
    // if (!boolvn->isWritten()) return res;
    if !vn_is_written(data, boolvn) {
        return res;
    }
    // PcodeOp *cmp = boolvn->getDef();  OpCode oc = cmp->code();
    let cmp = vn_def(data, boolvn).expect("analyzeCmp: written boolvn has a def");
    let oc = op_code(data, cmp);
    // Only binary integer comparisons (all have two inputs) can be cascade nodes.
    if oc != OpCode::CPUI_INT_EQUAL
        && oc != OpCode::CPUI_INT_NOTEQUAL
        && oc != OpCode::CPUI_INT_SLESS
        && oc != OpCode::CPUI_INT_SLESSEQUAL
        && oc != OpCode::CPUI_INT_LESS
        && oc != OpCode::CPUI_INT_LESSEQUAL
    {
        return res;
    }
    // Varnode *v0 = cmp->getIn(0);  Varnode *v1 = cmp->getIn(1);
    let v0 = match op_in(data, cmp, 0) {
        Some(v) => v,
        None => return res,
    };
    let v1 = match op_in(data, cmp, 1) {
        Some(v) => v,
        None => return res,
    };
    // if (v1->isConstant() && !v0->isConstant()) { var = v0; cval = v1->getOffset(); }
    // else if (v0->isConstant() && !v1->isConstant()) { var = v1; cval = v0->getOffset(); }
    // else return res;
    // `var_on_left` records the operand order (`var OP const` vs `const OP var`);
    // needed to normalize a range comparison to angr's "gt-form" below.
    let (var, cval, var_on_left): (VarnodeId, uintb, bool) =
        if vn_is_constant(data, v1) && !vn_is_constant(data, v0) {
            (v0, vn_offset(data, v1), true)
        } else if vn_is_constant(data, v0) && !vn_is_constant(data, v1) {
            (v1, vn_offset(data, v0), false)
        } else {
            return res;
        };

    // bool flip = cb->isBooleanFlip();
    let flip = op_is_boolean_flip(data, cb);
    // FlowBlock *condTrue = flip ? bb->getFalseOut() : bb->getTrueOut();
    // FlowBlock *condFalse = flip ? bb->getTrueOut() : bb->getFalseOut();
    let cond_true = if flip { block_false_out(data, bl) } else { block_true_out(data, bl) };
    let cond_false = if flip { block_true_out(data, bl) } else { block_false_out(data, bl) };

    // res.var = canonSwitchVar(var);
    res.var = Some(canon_switch_var(data, var));

    if oc == OpCode::CPUI_INT_EQUAL {
        // res.valid = true; res.isEquality = true; res.cval = cval;
        // res.matchOut = condTrue; res.contA = condFalse;
        res.valid = true;
        res.is_equality = true;
        res.cval = cval;
        res.match_out = Some(cond_true);
        res.cont_a = Some(cond_false);
    } else if oc == OpCode::CPUI_INT_NOTEQUAL {
        // res.matchOut = condFalse; res.contA = condTrue;  // match (V==cval) is the not-taken edge
        res.valid = true;
        res.is_equality = true;
        res.cval = cval;
        res.match_out = Some(cond_false);
        res.cont_a = Some(cond_true);
    } else {
        // SLESS / SLESSEQUAL / LESS / LESSEQUAL: range node, both directions continue.
        // res.valid = true; res.isEquality = false; res.cval = cval;
        // res.contA = condTrue; res.contB = condFalse;
        res.valid = true;
        res.is_equality = false;
        res.cval = cval;
        res.cont_a = Some(cond_true);
        res.cont_b = Some(cond_false);

        // Normalize to angr's "gt-form" so recover_cascade can classify a range
        // node's terminal edges as case-vs-default by the comparison VALUE — a
        // faithful port of `_find_switch_variable_comparison_type_c`
        // (`lowered_switch_simplifier.py:791-860`).  angr only sees `var OP const`
        // (operands[0] is the VirtualVariable); in pcode a `var > c` / `var >= c`
        // is emitted as `c < var` / `c <= var` (const on the left), so the
        // var-on-right orders below are the CmpGT/CmpGE cases.  In every case
        // gt_out ⟺ `V > range_value` and le_out ⟺ `V <= range_value`.
        let is_le =
            oc == OpCode::CPUI_INT_SLESSEQUAL || oc == OpCode::CPUI_INT_LESSEQUAL;
        let (gt_out, le_out, tval): (BlockId, BlockId, uintb) = if var_on_left {
            if is_le {
                // `var <= cval` — angr CmpLE (le=true_target, gt=false_target, value unchanged)
                (cond_false, cond_true, cval)
            } else {
                // `var < cval` — angr CmpLT (le=true_target, gt=false_target, value -= 1)
                (cond_false, cond_true, cval.wrapping_sub(1))
            }
        } else if is_le {
            // `cval <= var` (== `var >= cval`) — angr CmpGE (gt=true_target, le=false_target, value += 1)
            (cond_true, cond_false, cval.wrapping_add(1))
        } else {
            // `cval < var` (== `var > cval`) — angr CmpGT (gt=true_target, le=false_target, value unchanged)
            (cond_true, cond_false, cval)
        };
        res.gt_out = Some(gt_out);
        res.le_out = Some(le_out);
        res.range_value = tval;
    }
    res
}

/// Recover the entry start address of a target basic block (C++ static
/// `targetStart`).
///
/// At the bblocks level the block is a `BlockBasic`, so `getStart()` dispatches
/// to its cover's first address.
fn target_start(data: &Funcdata, bl: BlockId) -> Address {
    block_get_start(&data.bblocks_ref().arena, bl)
}

/// Count the INDEPENDENT default sinks among a set of default candidates
/// (kuna adaptation of angr's single-shared-default guard for the pre-merge,
/// variable-fragmented cascade — see the call site in [`recover_cascade`]).
///
/// A candidate is "absorbed" if a forward CFG walk from it reaches ANOTHER
/// candidate without crossing the switch structure (`stop` = case targets +
/// same-variable cascade nodes + the cascade head); such a candidate is only a
/// path into the shared default.  The candidates that reach no other candidate
/// are the sinks.  A genuine lowered switch funnels every default edge into one
/// sink; independent bodies each stay a sink.
#[allow(clippy::mutable_key_type)]
fn count_default_sinks(
    data: &Funcdata,
    cand_block: &BTreeMap<Address, BlockId>,
    stop: &BTreeSet<Address>,
) -> int4 {
    let cand_addrs: BTreeSet<Address> = cand_block.keys().cloned().collect();
    let mut sinks: int4 = 0;
    for (addr, &cb) in cand_block.iter() {
        // Seed the walk with the successors of the candidate (not itself).
        let mut queue: Vec<BlockId> = {
            let bref = data.bblocks_ref();
            let blk = bref.block(cb);
            (0..blk.size_out()).map(|k| blk.get_out(k)).collect()
        };
        let mut visited: BTreeSet<BlockId> = BTreeSet::new();
        let mut absorbed = false;
        while let Some(v) = queue.pop() {
            if !visited.insert(v) {
                continue;
            }
            if block_type(data, v) != BlockType::Basic {
                continue;
            }
            let va = target_start(data, v);
            if va != *addr && cand_addrs.contains(&va) {
                absorbed = true;
                break;
            }
            if stop.contains(&va) {
                continue; // bound the walk at the switch structure
            }
            let bref = data.bblocks_ref();
            let blk = bref.block(v);
            for k in 0..blk.size_out() {
                queue.push(blk.get_out(k));
            }
        }
        if !absorbed {
            sinks += 1;
        }
    }
    sinks
}

/// Try to recover one lowered switch with `startbb` as the cascade head (C++
/// static `recoverCascade`).
///
/// Walks the comparison tree, collecting equality cases and the common default.
/// Returns `Some(record)` on success.
///
/// `Address`-keyed `BTreeMap`/`BTreeSet` are the faithful port of the C++
/// `std::map<Address,int4>` / `std::set<Address>`; `Address` carries an
/// `Rc<AddrSpace>` (clippy's "mutable key type") but is never mutated as a key
/// (the established `heritage.rs` idiom).
#[allow(clippy::mutable_key_type)]
fn recover_cascade(
    data: &Funcdata,
    startbb: BlockId,
    cmpmap: &BTreeMap<BlockId, CmpNode>,
    swvar: VarnodeId,
) -> Option<KunaLoweredSwitchRecord> {
    // DFS over cascade nodes reading swvar; collect cases + default votes.
    // std::map<uintb,Address> cases;  std::map<Address,int4> defaultVotes;
    let mut cases: BTreeMap<uintb, Address> = BTreeMap::new();
    let mut default_votes: BTreeMap<Address, int4> = BTreeMap::new();
    // Parallel to `default_votes`: a representative block for each candidate
    // address, so the convergence check below can walk the CFG from it.
    let mut default_cand_block: BTreeMap<Address, BlockId> = BTreeMap::new();
    let mut visited: BTreeSet<BlockId> = BTreeSet::new();
    // Each stack entry carries angr's binary-search interval `(min_, max_)` for the
    // switch variable at that node (`lowered_switch_simplifier.py:441`
    // `stack = [(head, 0, 0xFFFF_FFFF_FFFF_FFFF)]`).  A range node narrows it as it
    // descends, so a terminal (non-cascade) edge that pins the variable to a single
    // value (`min_ + 1 == value` / `value == max_`) is a real CASE, not a default.
    let mut stack: Vec<(BlockId, uintb, uintb)> = Vec::new();
    stack.push((startbb, 0, 0xFFFF_FFFF_FFFF_FFFF));
    let mut saw_range = false; // true once a range (binary-search) node is seen

    // auto isCascade = [&](FlowBlock *fb)->BlockBasic * { ... };
    let is_cascade = |fb: Option<BlockId>| -> Option<BlockId> {
        let b = fb?;
        // BlockBasic check + cmpmap lookup + var == swvar
        if block_type(data, b) != BlockType::Basic {
            return None;
        }
        match cmpmap.get(&b) {
            Some(cn) if cn.var == Some(swvar) => Some(b),
            _ => None,
        }
    };

    // Only a true default candidate (a terminal edge NOT pinned to a single value)
    // is voted into `default_votes`; this closure keeps that construction in one
    // place (angr's `default_case_candidates[addr] = ...` dedup by address) and
    // remembers a representative block per candidate for the convergence check.
    let vote_default =
        |dv: &mut BTreeMap<Address, int4>, cb: &mut BTreeMap<Address, BlockId>, bl: BlockId| {
            let a = target_start(data, bl);
            *dv.entry(a.clone()).or_insert(0) += 1;
            cb.entry(a).or_insert(bl);
        };

    // while(!stack.empty()) {
    while let Some((bb, min_, max_)) = stack.pop() {
        // if (visited.count(bb)) continue;  visited.insert(bb);
        if visited.contains(&bb) {
            continue;
        }
        visited.insert(bb);
        // const CmpNode &cn( (*cmpmap.find(bb)).second );
        let cn = *cmpmap.get(&bb).expect("recoverCascade: stack block is in cmpmap");

        if cn.is_equality {
            // eq node: match edge is a case; the no-match edge continues with the
            // interval unchanged (angr keeps `(min_, max_)` across an eq node,
            // `lowered_switch_simplifier.py:539`).
            // BlockBasic *matchb = isCascade(cn.matchOut);
            match is_cascade(cn.match_out) {
                // match edge leads to another comparison: rare; treat as continuation
                Some(matchb) => stack.push((matchb, min_, max_)),
                None => {
                    // Address tgt = targetStart(cn.matchOut);
                    let tgt = target_start(data, cn.match_out.expect("equality node has matchOut"));
                    // if (cases.find(cn.cval) == cases.end()) cases[cn.cval] = tgt; else return false;
                    if cases.contains_key(&cn.cval) {
                        return None; // duplicate case value: bail (v1)
                    }
                    cases.insert(cn.cval, tgt);
                }
            }
            // BlockBasic *contb = isCascade(cn.contA);
            match is_cascade(cn.cont_a) {
                Some(contb) => stack.push((contb, min_, max_)),
                None => {
                    let a = cn.cont_a.expect("equality node has contA");
                    vote_default(&mut default_votes, &mut default_cand_block, a);
                }
            }
        } else {
            // Range node — angr's `op == "gt"` handling
            // (`lowered_switch_simplifier.py:548-597`).  Each edge either continues
            // into another cascade node (pushed with a narrowed interval) or is a
            // terminal edge.  A terminal edge that the binary-search interval pins
            // to a SINGLE value is a case body (folds into the default region as
            // before); only a terminal edge spanning a RANGE is a true default
            // candidate — the distinction the convergence guard below relies on.
            saw_range = true;
            let value = cn.range_value;
            let gt = cn.gt_out.expect("range node has gt_out");
            let le = cn.le_out.expect("range node has le_out");
            let gt_cascade = is_cascade(Some(gt));
            let le_cascade = is_cascade(Some(le));

            // gt edge: V > value ⇒ interval becomes (value, max_)
            //   `stack.append((gt_comp, value, max_))`
            if let Some(g) = gt_cascade {
                stack.push((g, value, max_));
            }
            // le edge: V <= value ⇒ interval becomes (min_, value - 1)
            //   `stack.append((le_comp, min_, value - 1))`
            if let Some(l) = le_cascade {
                stack.push((l, min_, value.wrapping_sub(1)));
            }

            // Classify each terminal (non-cascade) edge by the interval bound.
            // angr, at `:573-577`/`:585-589`, appends the single-value edge to
            // `cases` and only routes the rest into `default_case_candidates`.
            // kuna does the SAME split, but does NOT synthesize the single-value
            // edge as a new case label: the point here is to keep it OUT of the
            // default set so the convergence guard below counts only true default
            // sinks.  Synthesizing it as a case would change the recovered
            // JumpTable (extra case target) for switches that already recover
            // today, exercising an un-ported install/re-flow seam (LOSS-131) with
            // no output benefit — the value already folds into the default region
            // exactly as before.  So a single-value edge is simply dropped here.
            if le_cascade.is_none() && min_.wrapping_add(1) != value {
                // le edge terminal, not a single value ⇒ default candidate (`:579-582`).
                vote_default(&mut default_votes, &mut default_cand_block, le);
            }
            if gt_cascade.is_none() && value != max_ {
                // gt edge terminal, not a single value ⇒ default candidate (`:590-593`).
                vote_default(&mut default_votes, &mut default_cand_block, gt);
            }
        }
    }

    // if (cases.size() < 3) return false;  // RULE3: need >= 3 cases
    if cases.len() < 3 {
        return None;
    }
    // Fail-safe cap: a very large synthesized switch can make the structurer
    // mark some case edges as unstructured gotos and abort.  Cap at 16.
    if cases.len() > 16 {
        return None;
    }
    // if (defaultVotes.empty()) return false;
    if default_votes.is_empty() {
        return None;
    }
    // angr's guard `if cases and len(default_case_candidates) <= 1`
    // (`lowered_switch_simplifier.py:602`): a genuine GCC-lowered switch has
    // exactly ONE shared default.  angr keeps the whole comparison tree in one
    // cascade (its walk pushes any comparison node, `:537`, and stops at the
    // first different-variable node, `:487-491`), so the many CFG edges that fall
    // through to the default collapse to a single `default_case_candidates` entry.
    //
    // kuna groups the cascade by switch VARIABLE (`canonSwitchVar`), so when GCC
    // re-derives the dispatch value the single default REGION appears as several
    // unmerged blocks — the secondary dispatch head plus the trampolines that jump
    // into it.  Counting distinct addresses would therefore wrongly decline a
    // genuine switch (e.g. mv -O2 main: three default blocks 0x2d41 -> 0x2d50 <-
    // 0x2da0 that funnel into one region).  Instead count the INDEPENDENT default
    // sinks: a candidate that reaches another candidate (without crossing the
    // switch structure) is just a path into the shared default and does not count.
    // A genuine switch funnels to one sink; a hand-written if/else-if cascade
    // (stty `visible`: four range arms landing on four independent bodies) leaves
    // every arm a sink, so >1 sink ⇒ decline and render the correct if/else chain.
    if default_cand_block.len() > 1 {
        // Bound the CFG walk at the switch structure so a loop around the dispatch
        // (getopt-style) can't make the shared default reach back to a sibling arm.
        let mut stop: BTreeSet<Address> = BTreeSet::new();
        for t in cases.values() {
            stop.insert(t.clone());
        }
        for (bl, cn) in cmpmap.iter() {
            if cn.var == Some(swvar) {
                stop.insert(target_start(data, *bl));
            }
        }
        stop.insert(target_start(data, startbb));
        if count_default_sinks(data, &default_cand_block, &stop) > 1 {
            return None;
        }
    }
    // Require the GCC binary-search structure (a range/jump-tree split).  A
    // purely linear equality chain is a hand-written if/else-if.
    if !saw_range {
        return None;
    }

    // Default = most-voted common sink.
    // int4 best = -1;
    // for(dit...) if ((*dit).second > best) { best = ...; defAddr = (*dit).first; }
    let mut def_addr = Address::new_invalid();
    let mut best: int4 = -1;
    for (addr, votes) in default_votes.iter() {
        if *votes > best {
            best = *votes;
            def_addr = addr.clone();
        }
    }

    // Distinct targets, and no case target collides with the default.
    // std::set<Address> tgtset;
    let mut tgtset: BTreeSet<Address> = BTreeSet::new();
    for tgt in cases.values() {
        // if ((*cit).second == defAddr) return false;  // a case can't be the default block
        if *tgt == def_addr {
            return None;
        }
        tgtset.insert(tgt.clone());
    }
    // if (tgtset.size() < 2) return false;  // RULE3: >= 2 distinct targets
    if tgtset.len() < 2 {
        return None;
    }

    // The switch variable must live in stable storage (register/stack).
    // AddrSpace *vspc = swvar->getAddr().getSpace();  if (vspc == 0) return false;
    let vspc = vn_space(data, swvar)?;
    // spacetype st = vspc->getType();
    let st = vspc.get_type();
    // if (st != IPTR_PROCESSOR && st != IPTR_SPACEBASE) return false;
    if st != spacetype::IPTR_PROCESSOR && st != spacetype::IPTR_SPACEBASE {
        return None;
    }

    // rec.branchAddr = startbb->lastOp()->getAddr();
    let last = data.bb_op_tail(startbb).expect("recoverCascade: head has a last op");
    let branch_addr = op_addr(data, last);
    // rec.varAddr = swvar->getAddr();  rec.varSize = swvar->getSize();
    let var_addr = vn_addr(data, swvar);
    let var_size = vn_size(data, swvar);
    // rec.defaultTarget = defAddr;
    // for(cit...) { rec.caseVals.push_back(first); rec.caseTargets.push_back(second); }
    let mut case_vals: Vec<uintb> = Vec::with_capacity(cases.len());
    let mut case_targets: Vec<Address> = Vec::with_capacity(cases.len());
    for (val, tgt) in cases.iter() {
        case_vals.push(*val);
        case_targets.push(tgt.clone());
    }

    Some(KunaLoweredSwitchRecord {
        branch_addr,
        var_addr,
        var_size,
        case_vals,
        case_targets,
        default_target: def_addr,
    })
}

/// Find the cascade head, skipping leading sentinel guards (e.g. `V == -1`)
/// (C++ static `advancePastGuards`).
///
/// A getopt-style loop tests the EOF sentinel (-1) before the real dispatch;
/// that equality must stay an ordinary branch, so we advance past any leading
/// `V == all-ones` guard.
fn advance_past_guards(
    data: &Funcdata,
    mut head: BlockId,
    cmpmap: &BTreeMap<BlockId, CmpNode>,
    swvar: VarnodeId,
) -> BlockId {
    // for(int4 guard=0;guard<8;++guard) {
    for _guard in 0..8 {
        // it = cmpmap.find(head);  if (it == cmpmap.end()) break;
        let cn = match cmpmap.get(&head) {
            Some(cn) => *cn,
            None => break,
        };
        // if (!cn.isEquality) break;
        if !cn.is_equality {
            break;
        }
        // uintb allones = calc_mask(cn.var->getSize());
        let var = match cn.var {
            Some(v) => v,
            None => break,
        };
        let allones = calc_mask(vn_size(data, var));
        // if (cn.cval != allones) break;  // not a -1 sentinel guard
        if cn.cval != allones {
            break;
        }
        // BlockBasic *cont = dynamic_cast<BlockBasic *>(cn.contA);  if (cont == 0) break;
        let cont = match cn.cont_a {
            Some(c) if block_type(data, c) == BlockType::Basic => c,
            _ => break,
        };
        // cit = cmpmap.find(cont);  if (cit == cmpmap.end() || (*cit).second.var != swvar) break;
        match cmpmap.get(&cont) {
            Some(c) if c.var == Some(swvar) => {}
            _ => break,
        }
        // head = cont;  // the -1 guard is not part of the switch
        head = cont;
    }
    head
}

/// (kuna) Detect a compiler-lowered comparison cascade and request a restart
/// (C++ `ActionLowerSwitchDetect`).
///
/// Runs after `ActionSwitchNorm` on the simplified CFG.  On finding a cascade it
/// records (branch address, switch variable storage, ordered case constants +
/// targets, default target) in the side table and sets the restart-pending flag.
/// Makes no change to ops (returns 0) so the fullloop quiesces and the outer
/// `ActionRestartGroup` fires the restart.
pub struct ActionLowerSwitchDetect {
    /// Engine-owned bookkeeping (C++ inherited `Action` base fields).
    base: ActionBase,
    /// Resolved `glb->recover_lowered_switch` gate (STUB(W4); the seams
    /// `Architecture` skeleton on `Funcdata` does not carry the flag yet, so it
    /// is resolved at construction by the W9 assembler — the
    /// `kuna_compareform`/`kuna_arraystride` convention).
    enabled: bool,
    /// The sticky side table (C++ file-static `loweredStore`), shared with the
    /// Install half.  Cloning the Action (via `clone_filtered`) shares the same
    /// inner store, matching the file-static's single-instance survives-`clear()`
    /// semantics.
    store: SharedLoweredSwitchStore,
}

impl ActionLowerSwitchDetect {
    /// Construct in group `g` with the resolved gate and a freshly-owned store
    /// (C++ `ActionLowerSwitchDetect::ActionLowerSwitchDetect(const string &g)`:
    /// `Action(0,"lowerswitchdetect",g)`).
    pub fn new(enabled: bool, g: impl Into<String>) -> ActionLowerSwitchDetect {
        ActionLowerSwitchDetect::with_store(enabled, g, new_shared_store())
    }

    /// Construct sharing an existing [`SharedLoweredSwitchStore`] (the schedule
    /// builder threads the *same* store into the Detect and Install halves).
    pub fn with_store(
        enabled: bool,
        g: impl Into<String>,
        store: SharedLoweredSwitchStore,
    ) -> ActionLowerSwitchDetect {
        ActionLowerSwitchDetect {
            base: ActionBase::new(0, "lowerswitchdetect", g),
            enabled,
            store,
        }
    }

    /// Box the action (parallels the `ActionXxx::boxed` constructors).
    pub fn boxed(enabled: bool, g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionLowerSwitchDetect::new(enabled, g))
    }

    /// Borrow the shared recovered-switch side table (the install half reads this).
    pub fn store(&self) -> &SharedLoweredSwitchStore {
        &self.store
    }

    /// The detection body, exposed for testing without the Action state machine.
    /// Returns the number of records added (0 unless a cascade was recorded).
    pub fn detect(&mut self, data: &mut Funcdata) -> int4 {
        // if (!data.getArch()->recover_lowered_switch) return 0;  // P0 assertion not set
        if !self.enabled && !data.get_arch().recover_lowered_switch {
            return 0;
        }
        // if (data.isJumptableRecoveryOn()) return 0;  // not inside partial-fn recovery
        if data.is_jumptable_recovery_on() {
            return 0;
        }
        // if (kunaLoweredSwitchHasRecord(data)) return 0;  // already discovered (sticky)
        if self.store.borrow().has_record(data) {
            return 0;
        }

        // Collect comparison-on-constant cascade nodes, grouped by switch variable.
        // std::map<BlockBasic *,CmpNode> cmpmap;  std::map<Varnode *,int4> varCount;
        let mut cmpmap: BTreeMap<BlockId, CmpNode> = BTreeMap::new();
        let mut var_count: BTreeMap<VarnodeId, int4> = BTreeMap::new();
        // const BlockGraph &bb( data.getBasicBlocks() );
        let nblocks = data.bblocks_get_size();
        // for(int4 i=0;i<bb.getSize();++i) {
        for i in 0..nblocks {
            let b = data.bblocks_get_block(i);
            // BlockBasic *b = dynamic_cast<BlockBasic *>(bb.getBlock(i));  if (b == 0) continue;
            if block_type(data, b) != BlockType::Basic {
                continue;
            }
            // if (b->isSwitchOut()) continue;  // already a real switch
            if block_is_switch_out(data, b) {
                continue;
            }
            // if (!isPureCompareBlock(b)) continue;
            if !is_pure_compare_block(data, b) {
                continue;
            }
            // CmpNode cn = analyzeCmp(b);
            let cn = analyze_cmp(data, b);
            // if (!cn.valid || cn.var == 0) continue;
            if !cn.valid || cn.var.is_none() {
                continue;
            }
            // cmpmap[b] = cn;  varCount[cn.var] += 1;
            let var = cn.var.expect("valid CmpNode has var");
            cmpmap.insert(b, cn);
            *var_count.entry(var).or_insert(0) += 1;
        }

        // Pick the switch variable: the Varnode with the most cascade nodes.
        // Varnode *swvar = 0;  int4 bestCount = 1;
        // for(hit...) if ((*hit).second > bestCount) { bestCount = ...; swvar = (*hit).first; }
        let mut swvar: Option<VarnodeId> = None;
        let mut best_count: int4 = 1;
        for (var, count) in var_count.iter() {
            if *count > best_count {
                best_count = *count;
                swvar = Some(*var);
            }
        }
        // if (swvar == 0) return 0;
        let swvar = match swvar {
            Some(v) => v,
            None => return 0,
        };

        // Find the head: a cascade node for swvar that no other cascade node
        // continues into.  std::set<BlockBasic *> isChild;
        let mut is_child: BTreeSet<BlockId> = BTreeSet::new();
        // for(mit=cmpmap.begin();mit!=cmpmap.end();++mit) {
        for (_, cn) in cmpmap.iter() {
            // if ((*mit).second.var != swvar) continue;
            if cn.var != Some(swvar) {
                continue;
            }
            // FlowBlock *succ[3] = { cn.matchOut, cn.contA, cn.contB };
            let succ = [cn.match_out, cn.cont_a, cn.cont_b];
            for s in succ {
                // BlockBasic *s = dynamic_cast<BlockBasic *>(succ[i]);  if (s == 0) continue;
                let s = match s {
                    Some(b) if block_type(data, b) == BlockType::Basic => b,
                    _ => continue,
                };
                // sit = cmpmap.find(s);  if (sit != end && (*sit).second.var == swvar) isChild.insert(s);
                if let Some(sc) = cmpmap.get(&s) {
                    if sc.var == Some(swvar) {
                        is_child.insert(s);
                    }
                }
            }
        }
        // BlockBasic *head = 0;
        // for(mit...) { if (var != swvar) continue; if (isChild.count(first)) continue; head = first; break; }
        let mut head: Option<BlockId> = None;
        for (bl, cn) in cmpmap.iter() {
            if cn.var != Some(swvar) {
                continue;
            }
            if is_child.contains(bl) {
                continue;
            }
            head = Some(*bl);
            break;
        }
        // if (head == 0) return 0;
        let head = match head {
            Some(h) => h,
            None => return 0,
        };
        // head = advancePastGuards(head,cmpmap,swvar);
        let head = advance_past_guards(data, head, &cmpmap, swvar);

        // if (!recoverCascade(data,head,cmpmap,swvar,rec)) return 0;
        let rec = match recover_cascade(data, head, &cmpmap, swvar) {
            Some(r) => r,
            None => return 0,
        };

        // loweredStore[keyForFunc(data)].push_back(rec);
        self.store.borrow_mut().push(data, rec.clone());
        // data.setRestartPending(true);
        data.set_restart_pending(true);
        // kunaRecordRestart(data,krestart_lowered_switch,rec.branchAddr);  // STUB(W7): no
        //   &mut RestartLog in the Action::apply signature yet (the heritage port
        //   threads it explicitly); the restart-pending flag is realized.
        1 // a record was added (the C++ returns 0 to quiesce; see apply note)
    }
}

impl Action for ActionLowerSwitchDetect {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }

    /// C++ `ActionLowerSwitchDetect::clone`:
    /// `if (!grouplist.contains(getGroup())) return 0;`
    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>> {
        if !grouplist.contains(self.get_group()) {
            return None;
        }
        // The store is the sticky file-static analog: a clone shares the same
        // discovered hints (C++ `loweredStore` is global to the clone tree).
        Some(Box::new(ActionLowerSwitchDetect {
            base: self.base.clone(),
            enabled: self.enabled,
            store: self.store.clone(),
        }))
    }

    /// C++ `ActionLowerSwitchDetect::apply` — detect the cascade, record it, and
    /// request a restart.  **The C++ `apply` returns 0 unconditionally** (no op
    /// change) so the fullloop quiesces and the outer `ActionRestartGroup` fires
    /// the restart; the detection's `count` stays 0.  The [`detect`] helper
    /// returns the record count for testing; here we discard it to match the
    /// C++ `return 0` and leave `count` untouched.
    ///
    /// [`detect`]: ActionLowerSwitchDetect::detect
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        let _ = self.detect(data);
        0 // no op change; let the fullloop quiesce and the restart fire
    }
}

//===========================================================================
// Install (CFG surgery) on the restart pass, pre-SSA
//===========================================================================

/// (kuna) Install the recorded lowered switch as a `BRANCHIND` + `JumpTable`
/// (C++ `ActionLowerSwitchInstall`).
///
/// Runs before `ActionHeritage` on the restart pass (pre-SSA, no `MULTIEQUAL`s
/// yet, so the edge surgery needs no phi patching).  Reads the side table; for
/// each recorded record that is not yet installed, it calls
/// `Funcdata::kunaInstallLoweredSwitch` to manufacture the S2 artifact (replace
/// the cascade head's `CBRANCH` with a `BRANCHIND(V)`, rewire its out-edges to
/// the case targets + default, and build a fully-labelled `JumpTable`).  A
/// non-zero return makes the mainloop re-iterate so `ActionHeritage` rebuilds
/// SSA over the corrected CFG.
///
/// ## The install surgery (ported)
///
/// The body's central call, [`Funcdata::kuna_install_lowered_switch`], performs
/// the CFG surgery over the now-ported surfaces:
///
///   * [`Funcdata::get_heritage_pass`] — the install gate
///     `get_heritage_pass() != 0` selects the pre-SSA window (the restart
///     re-clears heritage, so the first mainloop iteration runs at pass 0).
///   * the live `jumpvec` registry — the constructed [`JumpTable`] is pushed as a
///     real entry, linked to the synthesized BRANCHIND.
///   * [`JumpTable::kuna_set_trivial_model`] — attaches a
///     [`JumpModelTrivial`](crate::jumptable::JumpModelTrivial) so the
///     model-bearing queries behave as for a recovered switch.
///   * [`Funcdata::remove_unreachable_blocks`] — sweeps the orphaned
///     comparison-cascade spine after the head jumps straight to the targets.
///
/// The Detect half records the hint on the pre-restart pass; the Install half
/// reads the *same* (shared) store on the restart and manufactures the artifact,
/// after which the existing structurer + printer emit the `switch`.
pub struct ActionLowerSwitchInstall {
    /// Engine-owned bookkeeping (C++ inherited `Action` base fields).
    base: ActionBase,
    /// Resolved `glb->recover_lowered_switch` gate (STUB(W4); resolved at
    /// construction, the `ActionLowerSwitchDetect` convention).
    enabled: bool,
    /// The sticky side table (C++ file-static `loweredStore`), shared with the
    /// Detect half: the schedule builder threads the *same*
    /// [`SharedLoweredSwitchStore`] into both, so the Install reads the hint the
    /// Detect half recorded on the pre-restart pass.
    store: SharedLoweredSwitchStore,
}

impl ActionLowerSwitchInstall {
    /// Construct in group `g` with the resolved gate and a freshly-owned store
    /// (C++ `ActionLowerSwitchInstall::ActionLowerSwitchInstall(const string &g)`:
    /// `Action(0,"lowerswitchinstall",g)`).
    pub fn new(enabled: bool, g: impl Into<String>) -> ActionLowerSwitchInstall {
        ActionLowerSwitchInstall::with_store(enabled, g, new_shared_store())
    }

    /// Construct sharing an existing [`SharedLoweredSwitchStore`] (the schedule
    /// builder threads the *same* store into the Detect and Install halves).
    pub fn with_store(
        enabled: bool,
        g: impl Into<String>,
        store: SharedLoweredSwitchStore,
    ) -> ActionLowerSwitchInstall {
        ActionLowerSwitchInstall {
            base: ActionBase::new(0, "lowerswitchinstall", g),
            enabled,
            store,
        }
    }

    /// Box the action (parallels the `ActionXxx::boxed` constructors).
    pub fn boxed(enabled: bool, g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionLowerSwitchInstall::new(enabled, g))
    }

    /// Borrow the shared recovered-switch side table.
    pub fn store(&self) -> &SharedLoweredSwitchStore {
        &self.store
    }

    /// The shared side table (so the schedule builder / tests can seed it).
    pub fn store_mut(&mut self) -> &SharedLoweredSwitchStore {
        &self.store
    }

    /// The install body, exposed for testing without the Action state machine.
    /// Returns the number of records installed (always 0 until the W7/W4 surgery
    /// surfaces land — see the struct docs).
    pub fn install(&mut self, data: &mut Funcdata) -> int4 {
        // if (!data.getArch()->recover_lowered_switch) return 0;
        // The live gate is carried on the seam Architecture (`build_arch_handle`);
        // `enabled` stays as the unit-test OR-override (action registered false).
        if !self.enabled && !data.get_arch().recover_lowered_switch {
            return 0;
        }
        // if (data.getHeritagePass() != 0) return 0;  // only in the pre-SSA window.
        // The install rewires the CFG (severs/adds out-edges, swaps the CBRANCH for
        // a BRANCHIND); doing that after heritage would strand SSA phi state.  The
        // restart re-flows raw pcode and re-clears heritage, so the first mainloop
        // iteration on the restart has pass 0 — the install fires there, then
        // ActionHeritage rebuilds SSA over the corrected CFG.
        if data.get_heritage_pass() != 0 {
            return 0;
        }
        // iter = loweredStore.find(keyForFunc(data));
        // if (iter == loweredStore.end() || (*iter).second.empty()) return 0;
        let store = self.store.borrow();
        if !store.has_record(data) {
            return 0;
        }

        // int4 changed = 0;
        // for each recorded record: jt = data.kunaInstallLoweredSwitch(...).  A
        // non-null table means the surgery committed; count it so the mainloop
        // re-iterates and ActionHeritage rebuilds SSA over the corrected CFG.
        let recs: Vec<KunaLoweredSwitchRecord> = store.records(data).to_vec();
        drop(store);
        let mut changed = 0;
        for r in &recs {
            match data.kuna_install_lowered_switch(
                &r.branch_addr,
                &r.var_addr,
                r.var_size,
                &r.case_vals,
                &r.case_targets,
                &r.default_target,
            ) {
                Ok(Some(_idx)) => changed += 1,
                Ok(None) => {} // CFG no longer matches the record; decline
                Err(_e) => {}  // surgery error: leave the CFG as-is
            }
        }
        changed
    }
}

impl Action for ActionLowerSwitchInstall {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }

    /// C++ `ActionLowerSwitchInstall::clone`:
    /// `if (!grouplist.contains(getGroup())) return 0;`
    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>> {
        if !grouplist.contains(self.get_group()) {
            return None;
        }
        Some(Box::new(ActionLowerSwitchInstall {
            base: self.base.clone(),
            enabled: self.enabled,
            store: self.store.clone(),
        }))
    }

    /// C++ `ActionLowerSwitchInstall::apply` — install the recorded switch as a
    /// `BRANCHIND` + `JumpTable`.  The change count flows into `count` (the C++
    /// `return changed` makes the mainloop re-iterate); here the install declines
    /// at the W7/W4 surgery seam, so `count` is untouched.
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        let changed = self.install(data);
        self.base.count += changed;
        0
    }
}

//===========================================================================
// Option gate
//===========================================================================

/// (kuna) Toggle lowered comparison-cascade -> switch recovery:
/// `loweredswitch on|off` (C++ `OptionLowerSwitch`).
///
/// The kuna Rust port ships default-on (the angr-port DIV;
/// `architecture.rs:418`).  "off" makes the Detect/Install Actions inert.
pub struct OptionLowerSwitch;

impl OptionLowerSwitch {
    /// The option name (C++ `name = "loweredswitch"`).
    pub const NAME: &'static str = "loweredswitch";

    /// C++ `OptionLowerSwitch::apply`:
    ///
    /// ```text
    ///   bool val = onOrOff(p1);
    ///   glb->recover_lowered_switch = val;
    ///   string prop = val ? "on" : "off";
    ///   return "Lowered comparison-cascade switch recovery turned "+prop;
    /// ```
    ///
    /// Returns the resolved flag plus the confirmation message.  The caller
    /// writes the flag into `Architecture::recover_lowered_switch` (threaded by
    /// the W9 option dispatch; STUB noted in the module docs).
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        // bool val = onOrOff(p1);
        let val = on_or_off(p1)?;
        // glb->recover_lowered_switch = val;  -- left to the caller (STUB(W4/W9)).
        // string prop = val ? "on" : "off";
        let prop = if val { "on" } else { "off" };
        // return "Lowered comparison-cascade switch recovery turned "+prop;
        Ok((val, format!("Lowered comparison-cascade switch recovery turned {prop}")))
    }
}

// ---------------------------------------------------------------------------
// Local read accessors (the `kuna_compareform` / `kuna_arraystride` idiom).
// ---------------------------------------------------------------------------

fn op_in(data: &Funcdata, op: OpId, slot: int4) -> Option<VarnodeId> {
    data.obank().get(op).and_then(|o| o.get_in(slot))
}
fn op_code(data: &Funcdata, op: OpId) -> OpCode {
    data.obank().get(op).expect("op_code: stale op").code()
}
fn op_addr(data: &Funcdata, op: OpId) -> Address {
    data.obank().get(op).expect("op_addr: stale op").get_addr().clone()
}
fn op_is_boolean_flip(data: &Funcdata, op: OpId) -> bool {
    data.obank().get(op).expect("op_is_boolean_flip: stale op").is_boolean_flip()
}
fn vn_is_constant(data: &Funcdata, vn: VarnodeId) -> bool {
    data.vbank().get(vn).map(|v| v.is_constant()).unwrap_or(false)
}
fn vn_is_written(data: &Funcdata, vn: VarnodeId) -> bool {
    data.vbank().get(vn).map(|v| v.is_written()).unwrap_or(false)
}
fn vn_def(data: &Funcdata, vn: VarnodeId) -> Option<OpId> {
    data.vbank().get(vn).and_then(|v| v.get_def())
}
fn vn_offset(data: &Funcdata, vn: VarnodeId) -> uintb {
    data.vbank().get(vn).map(|v| v.get_offset()).unwrap_or(0)
}
fn vn_size(data: &Funcdata, vn: VarnodeId) -> int4 {
    data.vbank().get(vn).map(|v| v.get_size()).unwrap_or(0)
}
fn vn_addr(data: &Funcdata, vn: VarnodeId) -> Address {
    data.vbank().get(vn).expect("vn_addr: stale vn").get_addr().clone()
}
fn vn_space(data: &Funcdata, vn: VarnodeId) -> Option<Rc<AddrSpace>> {
    data.vbank().get(vn).and_then(|v| v.get_addr().get_space().cloned())
}

// ---------------------------------------------------------------------------
// Block read accessors (FlowBlock methods reached through the BlockGraph).
// ---------------------------------------------------------------------------

fn block_type(data: &Funcdata, bl: BlockId) -> BlockType {
    data.bblocks_ref().block(bl).get_type()
}
fn block_size_out(data: &Funcdata, bl: BlockId) -> int4 {
    data.bblocks_ref().block(bl).size_out()
}
fn block_true_out(data: &Funcdata, bl: BlockId) -> BlockId {
    data.bblocks_ref().block(bl).get_true_out()
}
fn block_false_out(data: &Funcdata, bl: BlockId) -> BlockId {
    data.bblocks_ref().block(bl).get_false_out()
}
fn block_is_switch_out(data: &Funcdata, bl: BlockId) -> bool {
    data.bblocks_ref().block(bl).is_switch_out()
}

#[cfg(test)]
#[path = "kuna_loweredswitch/tests.rs"]
mod tests;

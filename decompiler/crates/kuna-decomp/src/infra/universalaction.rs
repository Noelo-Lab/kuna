//! Port of `decompiler/cpp/coreaction.cc` `ActionDatabase::universalAction`
//! (`coreaction.cc:5722`) — **the keystone schedule** (ADR 0005).
//!
//! # What this is
//!
//! `universalAction` is the ~300-line C++ constructor that imperatively builds
//! the single *universal* Action/Rule tree — every pass the decompiler knows,
//! nested into [`ActionGroup`]s / [`ActionPool`]s / the outer
//! [`ActionRestartGroup`].  A *root* action (`"decompile"`, `"jumptable"`, …) is
//! that universal tree filtered to a set of enabled groups by an
//! [`ActionGroupList`] (C++ `clone(grouplist)`; here
//! [`Action::clone_filtered`]).  The group member-lists are built verbatim by
//! [`build_default_groups`] (already ported in `action.rs`).
//!
//! Per ADR 0005 the schedule is represented **declaratively** as a [`SchedNode`]
//! spec tree (variants [`SchedNode::Action`] / [`SchedNode::Pool`] /
//! [`SchedNode::Group`] / [`SchedNode::RestartGroup`]), then *materialized* into
//! the concrete engine objects by [`SchedNode::materialize`].  The declarative
//! form is diffable against the C++ constructor (this file's body is a
//! line-for-line transcription of `coreaction.cc:5734-6031`), printable for
//! `list action`-style tooling, and the natural anchor for a future
//! `PassRegistration`.
//!
//! # Faithfulness notes
//!
//! * The tree shape, the **child order within every group/pool**, the node
//!   flags (`rule_repeatapply` on the loops/pools, `rule_onceperfunc` only on
//!   the universal restart group), and each node's *group string* and *name* are
//!   transcribed exactly — these determine the filtered `clone(grouplist)`
//!   result and the `list action` dump that the B0 gate byte-compares against
//!   the C++ oracle.
//! * Each leaf Action is constructed through its own module's constructor (which
//!   fixes its C++ `name`); each Rule is registered into its [`ActionPool`] with
//!   the C++ `Rule::getName()` string (the 3rd `Rule(...)` ctor argument) so the
//!   pool dump matches.  The Rule's *group* is the `universalAction` group
//!   argument (not the per-file `specs()` placeholder).
//! * `ActionParamShiftStart`/`ActionParamShiftStop` (C++ 5741, 5765) and
//!   `RuleIndirectConcat` (C++ 5938) are **commented out** in upstream and so
//!   are absent here too.
//!
//! # The B0 allowlist (now empty)
//!
//! Earlier waves left a handful of Rules/Actions referenced by `universalAction`
//! un-ported (STUB markers in their home modules), omitted from the materialized
//! tree and enumerated in [`UNPORTED_ALLOWLIST`].  As of
//! `w8x-universalaction-wire` all of them (`splitflow`, `subfloat_convert`,
//! `stackprobeloop`, `lowerswitchinstall`, `dumptyhumplate`, `splitcopy`,
//! `splitload`, `splitstore`, `stringcopy`, `stringstore`) are ported and wired
//! into [`universal_sched`] at their exact C++ registration positions, so
//! [`UNPORTED_ALLOWLIST`] is **empty** and the materialized "decompile" tree is
//! byte-equal to the C++ oracle `list action` dump.

use std::rc::Rc;

use kuna_base::space::AddrSpace;
use kuna_base::types::int4;

use crate::action::{
    ruleflags, Action, ActionDatabase, ActionGroup, ActionPool, ActionRestartGroup, Rule,
    RuleState, UNIVERSAL_NAME,
};

// =============================================================================
// Allowlist of genuinely-unported passes (documented B0 dump diff)
// =============================================================================

/// A pass named by `universalAction` but not yet ported, with the wave/boundary
/// blocking it.  Materialization skips these; the B0 listing test treats them as
/// an explicit, named diff against the C++ oracle dump.
#[derive(Debug, Clone, Copy)]
pub struct UnportedEntry {
    /// The C++ `getName()` of the missing Action/Rule (the dump token).
    pub name: &'static str,
    /// The `universalAction` group argument it was registered under.
    pub group: &'static str,
    /// The wave/boundary that must land it.
    pub blocked_by: &'static str,
}

/// Every pass present in the C++ `universalAction` tree but not yet ported.
///
/// **Empty since `w8x-universalaction-wire`.**  The last ten allowlisted passes
/// (`splitflow`, `subfloat_convert`, `stackprobeloop`, `lowerswitchinstall`,
/// `dumptyhumplate`, `splitcopy`, `splitload`, `splitstore`, `stringcopy`,
/// `stringstore`) were ported on the `w8x-subflow-splits` / `w8x-constseq-strings`
/// / `kuna_stackprobeloop` / `kuna_loweredswitch` branches and are now wired into
/// [`universal_sched`] at their exact C++ registration positions.  The
/// materialized "decompile" tree is therefore **byte-equal** to the C++ oracle
/// `list action` dump with no diff (the B0 empty-allowlist gate).
pub const UNPORTED_ALLOWLIST: &[UnportedEntry] = &[];

// =============================================================================
// SchedNode — the declarative schedule (ADR 0005)
// =============================================================================

/// A registration row for one [`Rule`] inside an [`ActionPool`]: the C++
/// `Rule::getName()`, the `universalAction` group, and a thunk constructing a
/// fresh boxed instance.
pub struct RuleRow {
    /// The Rule's name (C++ 3rd `Rule(...)` ctor arg), used for the pool dump
    /// and the [`RuleState`].
    pub name: &'static str,
    /// The `universalAction` group argument (the [`RuleState`] basegroup).
    pub group: &'static str,
    /// Construct a fresh boxed Rule.
    pub ctor: fn() -> Box<dyn Rule>,
}

/// A thunk constructing a fresh boxed leaf [`Action`].  A `Box<dyn Fn>` (not a
/// bare `fn`) so the few actions carrying architecture state (the stack
/// [`AddrSpace`] for `stackptrflow` / `extrapopsetup`) can capture it.
pub type ActionCtor = Box<dyn Fn() -> Box<dyn Action>>;

/// A node in the declarative universal schedule (ADR 0005).
pub enum SchedNode {
    /// A leaf [`Action`], constructed by its own module's ctor (which fixes its
    /// name).  The thunk yields a freshly boxed action.
    Action(ActionCtor),
    /// An [`ActionPool`]: a flagged, named bag of [`Rule`]s in registration
    /// order.
    Pool { flags: u32, name: &'static str, rules: Vec<RuleRow> },
    /// An [`ActionGroup`]: a flagged, named ordered list of children.
    Group { flags: u32, name: &'static str, children: Vec<SchedNode> },
    /// The outer [`ActionRestartGroup`]: like a group, with a restart bound.
    RestartGroup { flags: u32, name: &'static str, max: int4, children: Vec<SchedNode> },
}

impl SchedNode {
    /// Materialize this declarative node into a concrete engine [`Action`] tree
    /// (C++ the imperative `new ActionGroup(...) / addAction / addRule` calls).
    pub fn materialize(self) -> Box<dyn Action> {
        match self {
            SchedNode::Action(ctor) => ctor(),
            SchedNode::Pool { flags, name, rules } => {
                let mut pool = ActionPool::new(flags, name);
                for row in rules {
                    let rule = (row.ctor)();
                    pool.add_rule(RuleState::new(row.group, 0, row.name), rule);
                }
                Box::new(pool)
            }
            SchedNode::Group { flags, name, children } => {
                let mut grp = ActionGroup::new(flags, name);
                for child in children {
                    grp.add_action(child.materialize());
                }
                Box::new(grp)
            }
            SchedNode::RestartGroup { flags, name, max, children } => {
                let mut grp = ActionRestartGroup::new(flags, name, max);
                for child in children {
                    grp.add_action(child.materialize());
                }
                Box::new(grp)
            }
        }
    }

    /// Whether this node survives a `clone(grouplist)` filter — used to decide
    /// if a parent's loop must print it (and add a trailing `endl`).  Mirrors
    /// [`Action::clone_filtered`]: a leaf Action survives iff its group is
    /// enabled; a [`SchedNode::Pool`] iff ≥1 rule survives (by group); a
    /// group/restart-group iff ≥1 child survives.
    fn survives(&self, grouplist: &ActionListFilter) -> bool {
        match self {
            SchedNode::Action(ctor) => grouplist.contains(ctor().get_group()),
            SchedNode::Pool { rules, .. } => {
                rules.iter().any(|r| grouplist.contains(r.group))
            }
            SchedNode::Group { children, .. } | SchedNode::RestartGroup { children, .. } => {
                children.iter().any(|c| c.survives(grouplist))
            }
        }
    }

    /// Render this node, filtered by `grouplist`, exactly as the C++
    /// `Action::print` / `ActionGroup::print` / `ActionPool::print` virtuals do
    /// (`action.cc:132/428/753`).  Appends to `out`, returns the next index.
    ///
    /// Newline discipline matches C++: a leaf `Action::print` writes **no**
    /// trailing newline (the parent's loop adds one after each child); a
    /// group/pool writes its own header `endl` and a trailing `endl` after each
    /// of its children, so a nested group/pool ends with a `\n` and the parent
    /// then adds *another* — the blank-line-after-container the C++ produces.
    /// Caller must only invoke this when [`survives`](Self::survives) is true.
    fn print_listing(&self, out: &mut String, num: int4, depth: int4, grouplist: &ActionListFilter) -> int4 {
        match self {
            SchedNode::Action(ctor) => {
                // Leaf: one line, no trailing newline (C++ Action::print).
                let act = ctor();
                let base = act.base();
                print_action_line(out, num, base.flags, base.breakpoint, depth, act.get_name())
            }
            SchedNode::Pool { flags, name, rules } => {
                // C++ ActionPool::print: header + endl, then each surviving rule
                // line + endl.
                let mut num = print_action_line(out, num, *flags, 0, depth, name);
                out.push('\n');
                let rdepth = depth + 1;
                for r in rules.iter().filter(|r| grouplist.contains(r.group)) {
                    print_rule_line(out, num, false, false, rdepth, r.name);
                    out.push('\n');
                    num += 1;
                }
                num
            }
            SchedNode::Group { flags, name, children } => {
                Self::print_container(out, num, *flags, name, depth, children, grouplist)
            }
            SchedNode::RestartGroup { flags, name, children, .. } => {
                Self::print_container(out, num, *flags, name, depth, children, grouplist)
            }
        }
    }

    /// C++ `ActionGroup::print` body: header + endl, then for each *surviving*
    /// child `{ child.print(...); endl; }`.  The trailing endl after every child
    /// is what gives nested groups/pools a blank line beneath them.
    fn print_container(
        out: &mut String,
        num: int4,
        flags: u32,
        name: &str,
        depth: int4,
        children: &[SchedNode],
        grouplist: &ActionListFilter,
    ) -> int4 {
        let mut num = print_action_line(out, num, flags, 0, depth, name);
        out.push('\n');
        for child in children {
            if !child.survives(grouplist) {
                continue; // clone(grouplist) dropped it; no line, no endl
            }
            num = child.print_listing(out, num, depth + 1, grouplist);
            out.push('\n'); // C++ `s << endl;` after every child
        }
        num
    }

    /// Produce the full `list action`-style dump of this (universal) tree as the
    /// **`grp` root** would derive it — the byte-for-byte equivalent of
    /// `getCurrent()->print(s,0,0)` after `setCurrent(grp)`.  The root is the
    /// universal restart group, which always survives, so no outer trailing
    /// newline is added (the C++ top-level call adds none either).
    pub fn list_action_dump(&self, grouplist: &ActionListFilter) -> String {
        let mut out = String::new();
        if self.survives(grouplist) {
            self.print_listing(&mut out, 0, 0, grouplist);
        }
        out
    }
}

/// The set of enabled groups used to filter a [`SchedNode`] listing — a thin
/// wrapper matching [`crate::action::ActionGroupList::contains`] semantics.
pub struct ActionListFilter {
    groups: std::collections::BTreeSet<String>,
}

impl ActionListFilter {
    /// Build from group member names.
    pub fn from_names<I, S>(names: I) -> ActionListFilter
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        ActionListFilter { groups: names.into_iter().map(Into::into).collect() }
    }

    /// Whether the group is enabled.
    pub fn contains(&self, g: &str) -> bool {
        self.groups.contains(g)
    }
}

/// Emit one `Action::print` line (`action.cc:132`): `setw(4)` index, the 8-char
/// `" repeat "`/blank, the `!`/`S`/`A` flag columns, `depth*5+2` indent, name.
/// Returns `num+1`.
///
/// The index is **zero-padded** to width 4 (`{num:04}`), matching the C++ oracle
/// `list action` dump: the console's shared output stream carries a sticky
/// `setfill('0')` (set by the address-printing commands run before `list action`
/// in the documented capture procedure), so `setw(4) << dec << num` renders
/// `0000`, `0001`, …  The B0 gate byte-compares against that captured dump.
fn print_action_line(
    out: &mut String,
    num: int4,
    flags: u32,
    breakpoint: u32,
    depth: int4,
    name: &str,
) -> int4 {
    use std::fmt::Write;
    let _ = write!(out, "{num:04}");
    out.push_str(if (flags & ruleflags::rule_repeatapply) != 0 { " repeat " } else { "        " });
    out.push(if (flags & ruleflags::rule_onceperfunc) != 0 { '!' } else { ' ' });
    out.push(if (breakpoint & (1 | 2)) != 0 { 'S' } else { ' ' }); // break_start|tmpbreak_start
    out.push(if (breakpoint & (4 | 8)) != 0 { 'A' } else { ' ' }); // break_action|tmpbreak_action
    for _ in 0..(depth * 5 + 2) {
        out.push(' ');
    }
    out.push_str(name);
    num + 1
}

/// Emit one `ActionPool::print` rule line (`action.cc:765`): `setw(4)` index,
/// the `D`/`A` columns, `depth*5+2` indent, rule name.  (No newline.)
///
/// Zero-padded index, for the same sticky-`setfill('0')` reason as
/// [`print_action_line`].
fn print_rule_line(out: &mut String, num: int4, disabled: bool, break_action: bool, depth: int4, name: &str) {
    use std::fmt::Write;
    let _ = write!(out, "{num:04}");
    out.push(if disabled { 'D' } else { ' ' });
    out.push(if break_action { 'A' } else { ' ' });
    for _ in 0..(depth * 5 + 2) {
        out.push(' ');
    }
    out.push_str(name);
}

// Declarative-DSL helpers (kept terse so the spec body reads like the C++).
macro_rules! act {
    ($e:expr) => {
        SchedNode::Action(Box::new(|| $e))
    };
}
macro_rules! rrow {
    ($name:expr, $group:expr, $ctor:expr) => {
        RuleRow { name: $name, group: $group, ctor: || Box::new($ctor) }
    };
}

const REPEAT: u32 = ruleflags::rule_repeatapply;
const ONCE: u32 = ruleflags::rule_onceperfunc;

// =============================================================================
// The universal schedule (transcription of coreaction.cc:5734-6031)
// =============================================================================

/// Build the declarative universal [`SchedNode`] tree — a verbatim
/// transcription of `ActionDatabase::universalAction` (`coreaction.cc:5722`).
///
/// `stackspace` / `stackspace_index` correspond to the C++
/// `conf->getStackSpace()` (the same `AddrSpace`, passed both as the `Rc`
/// handle [`ActionStackPtrFlow`] keeps and as the manager *index*
/// [`ActionExtraPopSetup`] keeps).  `extra_pool_rules` are the architecture's
/// CPU-specific `conf->extra_pool_rules`, appended to `oppool1` exactly as the
/// C++ loop does (empty for the default tree).  None of these affect the
/// `list action` dump (they change only construction-time fields), so the B0
/// gate may pass `None`/`vec![]`.
pub fn universal_sched(
    stackspace: Option<Rc<AddrSpace>>,
    stackspace_index: Option<i32>,
    extra_pool_rules: Vec<RuleRow>,
) -> SchedNode {
    use crate::blockaction::{
        ActionBlockStructure, ActionBranchFlip, ActionFinalStructure, ActionNodeJoin,
        ActionNormalizeBranches, ActionPreferComplement, ActionReturnSplit, ActionStructureTransform,
    };
    use crate::coreaction_cleanup::{
        ActionAssignHigh, ActionCopyMarker, ActionDominantCopy, ActionHideShadow, ActionMarkExplicit,
        ActionMarkImplied, ActionMarkIndirectOnly, ActionMergeAdjacent, ActionMergeCopy,
        ActionMergeMultiEntry, ActionMergeRequired, ActionMergeType, ActionNameVars, ActionSetCasts,
    };
    use crate::coreaction_early::{
        ActionConstbase, ActionDeterminedBranch, ActionDoNothing, ActionHeritage,
        ActionLateDoNothing, ActionNonzeroMask, ActionNormalizeSetup, ActionRedundBranch,
        ActionSpacebase, ActionStart, ActionStartCleanUp, ActionStartTypes, ActionStop,
        ActionUnreachable, ActionVarnodeProps,
    };
    use crate::coreaction_protos::{
        ActionActiveParam, ActionActiveReturn, ActionDefaultParams, ActionExtraPopSetup,
        ActionFuncLink, ActionFuncLinkOutOnly, ActionInputPrototype, ActionOutputPrototype,
        ActionParamDouble, ActionPrototypeTypes, ActionPrototypeWarnings, ActionRestrictLocal,
        ActionReturnRecovery,
    };
    use crate::coreaction_render::{
        ActionConditionalConst, ActionConstantPtr, ActionDeadCode, ActionDeindirect,
        ActionDirectWrite, ActionDynamicMapping, ActionDynamicSymbols, ActionForceGoto,
        ActionInferTypes, ActionInternalStorage, ActionLaneDivide, ActionLikelyTrash,
        ActionMapGlobals, ActionMappedLocalSync, ActionMultiCse, ActionRestructureVarnode,
        ActionSegmentize, ActionShadowVar, ActionStackPtrFlow, ActionSwitchNorm,
        ActionUnjustifiedParams,
    };

    // --- oppool1 (the in-mainloop rule bag) -------------------------------
    let mut oppool1_rules: Vec<RuleRow> = vec![
        rrow!("earlyremoval", "deadcode", crate::ruleaction_1::RuleEarlyRemoval::new("deadcode")),
        rrow!("termorder", "analysis", crate::ruleaction_1::RuleTermOrder::new("analysis")),
        rrow!("selectcse", "analysis", crate::ruleaction_1::RuleSelectCse::new("analysis")),
        rrow!("collect_terms", "analysis", crate::ruleaction_1::RuleCollectTerms::new("analysis")),
        rrow!("pullsub_multi", "analysis", crate::ruleaction_1::RulePullsubMulti::new("analysis")),
        rrow!("pullsub_indirect", "analysis", crate::ruleaction_1::RulePullsubIndirect::new("analysis")),
        rrow!("push_multi", "nodejoin", crate::ruleaction_1::RulePushMulti::new("nodejoin")),
        rrow!("sborrow", "analysis", crate::ruleaction_3::RuleSborrow),
        rrow!("scarry", "analysis", crate::ruleaction_3::RuleScarry),
        rrow!("intlessequal", "canonicalcompare", crate::ruleaction_1::RuleIntLessEqual::new("canonicalcompare")),
        rrow!("trivialarith", "analysis", crate::ruleaction_2::RuleTrivialArith),
        rrow!("trivialbool", "analysis", crate::ruleaction_2::RuleTrivialBool),
        rrow!("trivialshift", "analysis", crate::ruleaction_3::RuleTrivialShift),
        rrow!("signshift", "analysis", crate::ruleaction_3::RuleSignShift),
        rrow!("testsign", "analysis", crate::ruleaction_3::RuleTestSign),
        rrow!("identityel", "analysis", crate::ruleaction_3::RuleIdentityEl),
        rrow!("ormask", "analysis", crate::ruleaction_1::RuleOrMask::new("analysis")),
        rrow!("andmask", "analysis", crate::ruleaction_1::RuleAndMask::new("analysis")),
        rrow!("orconsume", "analysis", crate::ruleaction_1::RuleOrConsume::new("analysis")),
        rrow!("orcollapse", "analysis", crate::ruleaction_1::RuleOrCollapse::new("analysis")),
        rrow!("andorlump", "analysis", crate::ruleaction_1::RuleAndOrLump::new("analysis")),
        rrow!("shiftbitops", "analysis", crate::ruleaction_1::RuleShiftBitops::new("analysis")),
        rrow!("rightshiftand", "analysis", crate::ruleaction_1::RuleRightShiftAnd::new("analysis")),
        rrow!("notdistribute", "analysis", crate::ruleaction_1::RuleNotDistribute::new("analysis")),
        rrow!("highorderand", "analysis", crate::ruleaction_1::RuleHighOrderAnd::new("analysis")),
        rrow!("anddistribute", "analysis", crate::ruleaction_1::RuleAndDistribute::new("analysis")),
        rrow!("andcommute", "analysis", crate::ruleaction_2::RuleAndCommute),
        rrow!("andpiece", "analysis", crate::ruleaction_2::RuleAndPiece),
        rrow!("andzext", "analysis", crate::ruleaction_2::RuleAndZext),
        rrow!("andcompare", "analysis", crate::ruleaction_2::RuleAndCompare),
        rrow!("doublesub", "analysis", crate::ruleaction_2::RuleDoubleSub),
        rrow!("doubleshift", "analysis", crate::ruleaction_2::RuleDoubleShift),
        rrow!("doublearithshift", "analysis", crate::ruleaction_2::RuleDoubleArithShift),
        rrow!("concatshift", "analysis", crate::ruleaction_2::RuleConcatShift),
        rrow!("leftright", "analysis", crate::ruleaction_2::RuleLeftRight),
        rrow!("boolsignshift", "analysis", crate::kuna_booleanmask::RuleBoolSignShift::with_group(false, "analysis")),
        rrow!("boolsignless", "analysis", crate::kuna_flagcompare::RuleBoolSignLess::with_group(false, "analysis")),
        rrow!("sborrowge", "analysis", crate::kuna_flagcompare::RuleSborrowGe::with_group(false, "analysis")),
        rrow!("shiftcompare", "analysis", crate::ruleaction_2::RuleShiftCompare),
        rrow!("shift2mult", "analysis", crate::ruleaction_3::RuleShift2Mult),
        rrow!("shiftpiece", "analysis", crate::ruleaction_3::RuleShiftPiece),
        rrow!("addcarrychain", "analysis", crate::kuna_addcarrychain::RuleAddCarryChain::with_group(false, "analysis")),
        rrow!("multicollapse", "analysis", crate::ruleaction_3::RuleMultiCollapse),
        rrow!("indirectcollapse", "analysis", crate::ruleaction_3::RuleIndirectCollapse),
        rrow!("2comp2mult", "analysis", crate::ruleaction_3::Rule2Comp2Mult),
        rrow!("sub2add", "analysis", crate::ruleaction_3::RuleSub2Add),
        rrow!("carryelim", "analysis", crate::ruleaction_3::RuleCarryElim),
        rrow!("bxor2notequal", "analysis", crate::ruleaction_1::RuleBxor2NotEqual::new("analysis")),
        rrow!("less2zero", "analysis", crate::ruleaction_5::RuleLess2Zero::new("analysis")),
        rrow!("lessequal2zero", "analysis", crate::ruleaction_5::RuleLessEqual2Zero::new("analysis")),
        rrow!("sless2zero", "analysis", crate::ruleaction_5::RuleSLess2Zero::new("analysis")),
        rrow!("equal2zero", "analysis", crate::ruleaction_5::RuleEqual2Zero::new("analysis")),
        rrow!("equal2constant", "analysis", crate::ruleaction_5::RuleEqual2Constant::new("analysis")),
        rrow!("threewaycomp", "analysis", crate::ruleaction_8::RuleThreeWayCompare),
        rrow!("xorcollapse", "analysis", crate::ruleaction_3::RuleXorCollapse),
        rrow!("addmultcollapse", "analysis", crate::ruleaction_3::RuleAddMultCollapse),
        rrow!("arraystride", "analysis", crate::kuna_arraystride::RuleArrayStride::with_group(false, "analysis")),
        rrow!("collapseconstants", "analysis", crate::ruleaction_3::RuleCollapseConstants),
        rrow!("transformcpool", "analysis", crate::ruleaction_3::RuleTransformCpool),
        rrow!("propagatecopy", "analysis", crate::ruleaction_3::RulePropagateCopy),
        rrow!("zexteliminate", "analysis", crate::ruleaction_2::RuleZextEliminate),
        rrow!("slesstoless", "analysis", crate::ruleaction_2::RuleSlessToLess),
        rrow!("zextsless", "analysis", crate::ruleaction_2::RuleZextSless),
        rrow!("bitundistribute", "analysis", crate::ruleaction_2::RuleBitUndistribute),
        rrow!("booleanundistribute", "analysis", crate::ruleaction_2::RuleBooleanUndistribute),
        rrow!("booleandedup", "analysis", crate::ruleaction_3::RuleBooleanDedup),
        rrow!("boolzext", "analysis", crate::ruleaction_3::RuleBoolZext),
        rrow!("booleannegate", "analysis", crate::ruleaction_3::RuleBooleanNegate),
        rrow!("logic2bool", "analysis", crate::ruleaction_3::RuleLogic2Bool),
        rrow!("subextcomm", "analysis", crate::ruleaction_4::RuleSubExtComm),
        rrow!("subcommute", "analysis", crate::ruleaction_4::RuleSubCommute),
        rrow!("concatcommute", "analysis", crate::ruleaction_4::RuleConcatCommute),
        rrow!("concatzext", "analysis", crate::ruleaction_4::RuleConcatZext),
        rrow!("zextcommute", "analysis", crate::ruleaction_4::RuleZextCommute),
        rrow!("zextshiftzext", "analysis", crate::ruleaction_4::RuleZextShiftZext),
        rrow!("shiftand", "analysis", crate::ruleaction_4::RuleShiftAnd),
        rrow!("concatzero", "analysis", crate::ruleaction_4::RuleConcatZero),
        rrow!("concatleftshift", "analysis", crate::ruleaction_4::RuleConcatLeftShift),
        rrow!("subzext", "analysis", crate::ruleaction_4::RuleSubZext),
        rrow!("subcancel", "analysis", crate::ruleaction_4::RuleSubCancel),
        rrow!("shiftsub", "analysis", crate::ruleaction_4::RuleShiftSub),
        rrow!("humptydumpty", "analysis", crate::ruleaction_4::RuleHumptyDumpty),
        rrow!("dumptyhump", "analysis", crate::ruleaction_4::RuleDumptyHump),
        rrow!("humptyor", "analysis", crate::ruleaction_4::RuleHumptyOr),
        rrow!("negateidentity", "analysis", crate::ruleaction_1::RuleNegateIdentity::new("analysis")),
        rrow!("subnormal", "analysis", crate::ruleaction_6::RuleSubNormal::new("analysis")),
        rrow!("positivediv", "analysis", crate::ruleaction_6::RulePositiveDiv::new("analysis")),
        rrow!("divtermadd", "analysis", crate::ruleaction_6::RuleDivTermAdd::new("analysis")),
        rrow!("divtermadd2", "analysis", crate::ruleaction_6::RuleDivTermAdd2::new("analysis")),
        rrow!("divopt", "analysis", crate::ruleaction_6::RuleDivOpt::new("analysis")),
        rrow!("signform", "analysis", crate::ruleaction_7::RuleSignForm),
        rrow!("signform2", "analysis", crate::ruleaction_7::RuleSignForm2),
        rrow!("signdiv2", "analysis", crate::ruleaction_7::RuleSignDiv2),
        rrow!("divchain", "analysis", crate::ruleaction_7::RuleDivChain),
        rrow!("signnearmult", "analysis", crate::ruleaction_7::RuleSignNearMult),
        rrow!("modopt", "analysis", crate::ruleaction_7::RuleModOpt),
        rrow!("signmod2nopt", "analysis", crate::ruleaction_7::RuleSignMod2nOpt),
        rrow!("signmod2nopt2", "analysis", crate::ruleaction_7::RuleSignMod2nOpt2),
        rrow!("signmod2opt", "analysis", crate::ruleaction_7::RuleSignMod2Opt),
        rrow!("switchsingle", "analysis", crate::ruleaction_4::RuleSwitchSingle),
        rrow!("condnegate", "analysis", crate::ruleaction_4::RuleCondNegate),
        rrow!("boolnegate", "analysis", crate::ruleaction_5::RuleBoolNegate::new("analysis")),
        rrow!("lessequal", "analysis", crate::ruleaction_2::RuleLessEqual),
        rrow!("lessnotequal", "analysis", crate::ruleaction_2::RuleLessNotEqual),
        rrow!("lessone", "analysis", crate::ruleaction_1::RuleLessOne::new("analysis")),
        rrow!("rangemeld", "analysis", crate::ruleaction_1::RuleRangeMeld::new("analysis")),
        rrow!("ovlesssimplify", "analysis", crate::kuna_ovlesssimplify::RuleOvLessSimplify::with_group(false, "analysis")),
        rrow!("floatrange", "analysis", crate::ruleaction_2::RuleFloatRange),
        rrow!("piece2zext", "analysis", crate::ruleaction_1::RulePiece2Zext::new("analysis")),
        rrow!("piece2sext", "analysis", crate::ruleaction_1::RulePiece2Sext::new("analysis")),
        rrow!("popcountboolxor", "analysis", crate::ruleaction_8::RulePopcountBoolXor),
        rrow!("xorswap", "analysis", crate::ruleaction_8::RuleXorSwap),
        rrow!("lzcountshiftbool", "analysis", crate::ruleaction_8::RuleLzcountShiftBool),
        rrow!("floatsign", "analysis", crate::ruleaction_8::RuleFloatSign),
        rrow!("orcompare", "analysis", crate::ruleaction_8::RuleOrCompare),
        rrow!("subvar_and", "subvar", crate::subflow::RuleSubvarAnd::new("subvar")),
        rrow!("subvar_subpiece", "subvar", crate::subflow::RuleSubvarSubpiece::new("subvar")),
        rrow!("splitflow", "subvar", crate::subflow::RuleSplitFlow::new("subvar")),
        rrow!("ptrflow", "subvar", crate::ruleaction_7::RulePtrFlow::new()),
        rrow!("subvar_compzero", "subvar", crate::subflow::RuleSubvarCompZero::new("subvar")),
        rrow!("subvar_shift", "subvar", crate::subflow::RuleSubvarShift::new("subvar")),
        rrow!("subvar_zext", "subvar", crate::subflow::RuleSubvarZext::new("subvar")),
        rrow!("subvar_sext", "subvar", crate::subflow::RuleSubvarSext::new("subvar")),
        rrow!("negatenegate", "analysis", crate::ruleaction_7::RuleNegateNegate),
        rrow!("conditionalmove", "conditionalexe", crate::ruleaction_7::RuleConditionalMove::new()),
        rrow!("orpredicate", "conditionalexe", crate::condexe::RuleOrPredicate::new("conditionalexe")),
        rrow!("funcptrencoding", "analysis", crate::ruleaction_8::RuleFuncPtrEncoding),
        rrow!("subfloat_convert", "floatprecision", crate::subflow::RuleSubfloatConvert::new("floatprecision")),
        rrow!("floatcast", "floatprecision", crate::ruleaction_7::RuleFloatCast),
        rrow!("ignorenan", "floatprecision", crate::ruleaction_7::RuleIgnoreNan),
        rrow!("unsigned2float", "analysis", crate::ruleaction_8::RuleUnsigned2Float),
        rrow!("int2floatcollapse", "analysis", crate::ruleaction_8::RuleInt2FloatCollapse),
        rrow!("ptraddundo", "typerecovery", crate::ruleaction_6::RulePtraddUndo::new("typerecovery")),
        rrow!("ptrsubundo", "typerecovery", crate::ruleaction_6::RulePtrsubUndo::new("typerecovery")),
        rrow!("segment", "segment", crate::ruleaction_7::RuleSegment),
        rrow!("piecepathology", "protorecovery", crate::ruleaction_8::RulePiecePathology),
        rrow!("doubleload", "doubleload", crate::double::RuleDoubleLoad::new("doubleload")),
        rrow!("doublestore", "doubleprecis", crate::double::RuleDoubleStore::new("doubleprecis")),
        rrow!("doublein", "doubleprecis", crate::double::RuleDoubleIn::new("doubleprecis")),
        rrow!("doubleout", "doubleprecis", crate::double::RuleDoubleOut::new("doubleprecis")),
        // (kuna) GH-8017/6858: gated by the live arch flag model_stack_probe_loop,
        // carried on the ArchContext and read in apply_op; registered `false` so the flag
        // (default-on, DIV-3) drives both the default and the `stackprobeloop off` toggle.
        rrow!("stackprobeloop", "analysis", crate::kuna_stackprobeloop::RuleStackProbeLoop::new(false, "analysis")),
    ];
    oppool1_rules.extend(extra_pool_rules);

    // --- oppool2 (post-blockstructure, type-pointer rules) ----------------
    let oppool2_rules: Vec<RuleRow> = vec![
        rrow!("pushptr", "typerecovery", crate::ruleaction_5::RulePushPtr::new("typerecovery")),
        rrow!("structoffset0", "typerecovery", crate::ruleaction_5::RuleStructOffset0::new("typerecovery")),
        rrow!("ptrarith", "typerecovery", crate::ruleaction_5::RulePtrArith::new("typerecovery")),
        // (kuna/upstream) RuleIndirectConcat("analysis") is commented out upstream.
        rrow!("loadvarnode", "stackvars", crate::ruleaction_4::RuleLoadVarnode),
        rrow!("storevarnode", "stackvars", crate::ruleaction_4::RuleStoreVarnode),
    ];

    // --- cleanup pool -----------------------------------------------------
    let cleanup_rules: Vec<RuleRow> = vec![
        rrow!("multnegone", "cleanup", crate::ruleaction_6::RuleMultNegOne::new("cleanup")),
        rrow!("addunsigned", "cleanup", crate::ruleaction_6::RuleAddUnsigned::new("cleanup")),
        rrow!("2comp2sub", "cleanup", crate::ruleaction_6::Rule2Comp2Sub::new("cleanup")),
        rrow!("dumptyhumplate", "cleanup", crate::subflow::RuleDumptyHumpLate::new("cleanup")),
        rrow!("subright", "cleanup", crate::ruleaction_6::RuleSubRight::new("cleanup")),
        rrow!("floatsigncleanup", "cleanup", crate::ruleaction_8::RuleFloatSignCleanup),
        rrow!("expandload", "cleanup", crate::ruleaction_8::RuleExpandLoad),
        rrow!("ptrsubcharconstant", "cleanup", crate::ruleaction_6::RulePtrsubCharConstant::new("cleanup")),
        rrow!("extensionpush", "cleanup", crate::ruleaction_6::RuleExtensionPush::new("cleanup")),
        rrow!("piecestructure", "cleanup", crate::ruleaction_6::RulePieceStructure::new("cleanup")),
        rrow!("splitcopy", "splitcopy", crate::subflow::RuleSplitCopy::new("splitcopy")),
        rrow!("splitload", "splitpointer", crate::subflow::RuleSplitLoad::new("splitpointer")),
        rrow!("splitstore", "splitpointer", crate::subflow::RuleSplitStore::new("splitpointer")),
        rrow!("stringcopy", "constsequence", crate::constseq::RuleStringCopy::new("constsequence")),
        rrow!("stringstore", "constsequence", crate::constseq::RuleStringStore::new("constsequence")),
        rrow!("memsetcopy", "constsequence", crate::kuna_memsetsequence::RuleMemsetCopy::with_group(false, "constsequence")),
        rrow!("bitfield_store", "bitfields", crate::bitfield::RuleBitFieldStore),
        rrow!("bitfield_out", "bitfields", crate::bitfield::RuleBitFieldOut),
        rrow!("bitfield_load", "bitfields", crate::bitfield::RuleBitFieldLoad),
        rrow!("bitfield_in", "bitfields", crate::bitfield::RuleBitFieldIn),
        rrow!("pull_absorb", "bitfields", crate::bitfield::RulePullAbsorb),
        rrow!("insert_absorb", "bitfields", crate::bitfield::RuleInsertAbsorb),
    ];

    // (kuna) One shared lowered-switch hint store (the C++ file-static
    // `loweredStore`): the Detect half (fullloop) writes it, the Install half
    // (mainloop) reads it on the restart.  Cloned into both action closures so
    // they reference the same inner table.
    let lowered_store = crate::kuna_loweredswitch::new_shared_store();
    let lowered_store_install = lowered_store.clone();
    let lowered_store_detect = lowered_store;

    // --- stackstall (inside mainloop) -------------------------------------
    let ss_for_pf = stackspace;
    let stackstall = SchedNode::Group {
        flags: REPEAT,
        name: "stackstall",
        children: vec![
            SchedNode::Pool { flags: REPEAT, name: "oppool1", rules: oppool1_rules },
            act!(ActionLaneDivide::boxed("base")),
            act!(ActionMultiCse::boxed("analysis")),
            act!(ActionShadowVar::boxed("analysis")),
            act!(ActionDeindirect::boxed("deindirect")),
            SchedNode::Action(Box::new(move || {
                ActionStackPtrFlow::boxed("stackptrflow", ss_for_pf.clone())
            })),
        ],
    };

    // --- mainloop ---------------------------------------------------------
    let mainloop = SchedNode::Group {
        flags: REPEAT,
        name: "mainloop",
        children: vec![
            act!(ActionUnreachable::boxed("base")),
            act!(ActionVarnodeProps::boxed("base")),
            // (kuna) Install a previously-detected lowered switch BEFORE heritage
            // (coreaction.cc:5755).  `enabled` resolves the C++
            // `glb->recover_lowered_switch` gate (default-on, DIV-3); the gate does
            // not affect the dump.
            SchedNode::Action(Box::new(move || {
                Box::new(crate::kuna_loweredswitch::ActionLowerSwitchInstall::with_store(
                    false,
                    "switchnorm",
                    lowered_store_install.clone(),
                ))
            })),
            act!(ActionHeritage::boxed("base")),
            act!(ActionParamDouble::boxed("protorecovery")),
            act!(ActionSegmentize::boxed("base")),
            act!(ActionInternalStorage::boxed("base")),
            act!(ActionForceGoto::boxed("blockrecovery")),
            act!(ActionDirectWrite::boxed("protorecovery_a", true)),
            act!(ActionDirectWrite::boxed("protorecovery_b", false)),
            act!(ActionActiveParam::boxed("protorecovery")),
            act!(ActionReturnRecovery::boxed("protorecovery")),
            act!(ActionRestrictLocal::boxed("localrecovery")),
            act!(ActionDeadCode::boxed("deadcode")),
            act!(ActionDynamicMapping::boxed("dynamic")),
            act!(ActionSpacebase::boxed("base")),
            act!(ActionNonzeroMask::boxed("analysis")),
            act!(ActionInferTypes::boxed("typerecovery")),
            act!(ActionRestructureVarnode::boxed("localrecovery")),
            stackstall,
            act!(ActionRedundBranch::boxed("deadcontrolflow")),
            act!(ActionBlockStructure::boxed("blockrecovery")),
            act!(ActionConstantPtr::boxed("typerecovery")),
            SchedNode::Pool { flags: REPEAT, name: "oppool2", rules: oppool2_rules },
            act!(ActionDeterminedBranch::boxed("unreachable")),
            act!(ActionUnreachable::boxed("unreachable")),
            act!(ActionNodeJoin::boxed("nodejoin")),
            act!(Box::new(crate::condexe::ActionConditionalExe::new("conditionalexe"))),
            act!(ActionConditionalConst::boxed("analysis")),
        ],
    };

    // --- fullloop ---------------------------------------------------------
    let fullloop = SchedNode::Group {
        flags: REPEAT,
        name: "fullloop",
        children: vec![
            mainloop,
            act!(ActionLikelyTrash::boxed("protorecovery")),
            act!(ActionDirectWrite::boxed("protorecovery_a", true)),
            act!(ActionDirectWrite::boxed("protorecovery_b", false)),
            act!(ActionDeadCode::boxed("deadcode")),
            act!(ActionDoNothing::boxed("deadcontrolflow")),
            act!(ActionSwitchNorm::boxed("switchnorm")),
            SchedNode::Action(Box::new(move || {
                Box::new(crate::kuna_loweredswitch::ActionLowerSwitchDetect::with_store(
                    false,
                    "switchnorm",
                    lowered_store_detect.clone(),
                ))
            })),
            act!(Box::new(crate::kuna_stackguard::ActionStripStackGuard::new(false, "returnsplit"))),
            act!(ActionReturnSplit::boxed("returnsplit")),
            // (kuna) angr SAILR gotoless `ReturnDuplicatorHigh` (option `returndup`,
            // default-OFF).  Runs right after ActionReturnSplit (the goto-driven
            // ReturnDuplicatorLow analog): for every shared *bare-epilogue* RETURN
            // block that ActionReturnSplit leaves merged (no goto-in edge), duplicate
            // it into each predecessor via the same `node_split` machinery, so the
            // classic `if (c) { body; return X; } return Y;` guard shape structures as
            // per-predecessor early returns instead of one comma-folded exit.  Placed
            // in the `returnsplit` group so its duplicated returns flow through the
            // same merge/naming passes and are re-structured by the next fullloop
            // iteration (and the final ActionBlockStructure).
            act!(crate::p8_structure::kuna_returndup::ActionReturnDup::boxed("returnsplit")),
            // (kuna) angr SAILR `ReturnDuplicatorHigh`, per-edge const narrowing (option
            // `earlyreturn`, default-OFF).  Sibling of ActionReturnDup: peels only the
            // CONSTANT arm of a mixed return phi (the guard's `return K`), leaving the
            // variable body return merged — the early-return-guard shape the whole-block
            // returndup gate structurally cannot reach.  branchflip + ifelseflatten do the
            // hoist downstream.
            act!(crate::p8_structure::kuna_earlyreturn::ActionEarlyReturn::boxed("returnsplit")),
            // (kuna) The direct continuation of ActionEarlyReturn: the SAME per-edge const
            // peel (option `switchreturn`, default-OFF), but with the in-edge cap lifted so
            // the WIDE multi-way switch-phi that overflows earlyreturn's 16-edge limit
            // (`switch (x) { case A: v = K0; break; … } return v;` with > 16 cases) is
            // reached too, hoisting each case to its own `return K`.  Registered right after
            // earlyreturn so the narrower pass consumes the ≤16-edge diamonds first.
            act!(crate::p8_structure::kuna_switchreturn::ActionSwitchReturn::boxed("returnsplit")),
            act!(ActionUnjustifiedParams::boxed("protorecovery")),
            act!(ActionStartTypes::boxed("typerecovery")),
            act!(ActionActiveReturn::boxed("protorecovery")),
        ],
    };

    // --- universal (the outer restart group) ------------------------------
    SchedNode::RestartGroup {
        flags: ONCE,
        name: UNIVERSAL_NAME,
        max: 1,
        children: vec![
            act!(ActionStart::boxed("base")),
            act!(ActionConstbase::boxed("base")),
            act!(ActionNormalizeSetup::boxed("normalanalysis")),
            act!(ActionDefaultParams::boxed("base")),
            SchedNode::Action(Box::new(move || {
                ActionExtraPopSetup::boxed("base", stackspace_index)
            })),
            act!(ActionPrototypeTypes::boxed("protorecovery")),
            act!(ActionFuncLink::boxed("protorecovery")),
            act!(ActionFuncLinkOutOnly::boxed("noproto")),
            fullloop,
            act!(ActionMappedLocalSync::boxed("localrecovery")),
            act!(ActionStartCleanUp::boxed("cleanup")),
            SchedNode::Pool { flags: REPEAT, name: "cleanup", rules: cleanup_rules },
            act!(ActionPreferComplement::boxed("blockrecovery", true)),
            act!(ActionStructureTransform::boxed("blockrecovery", true)),
            act!(ActionNormalizeBranches::boxed("normalizebranches")),
            act!(ActionAssignHigh::boxed("merge")),
            act!(ActionMergeRequired::boxed("merge")),
            act!(ActionMarkExplicit::boxed("merge")),
            act!(ActionMarkImplied::boxed("merge")),
            act!(ActionMergeMultiEntry::boxed("merge")),
            act!(ActionMergeCopy::boxed("merge")),
            act!(ActionDominantCopy::boxed("merge")),
            act!(ActionDynamicSymbols::boxed("dynamic")),
            act!(ActionMarkIndirectOnly::boxed("merge")),
            act!(ActionMergeAdjacent::boxed("merge")),
            act!(ActionMergeType::boxed("merge")),
            act!(ActionHideShadow::boxed("merge")),
            act!(ActionCopyMarker::boxed("merge")),
            act!(ActionLateDoNothing::boxed("blockrecovery")),
            act!(ActionBlockStructure::boxed("blockrecovery")),
            act!(ActionPreferComplement::boxed("blockrecovery", false)),
            act!(ActionStructureTransform::boxed("blockrecovery", false)),
            // (kuna, option `branchflip`, default-off) angr-SAILR-style negated-
            // guard branch flip on the FINAL structured tree, after the second
            // `preferComplement` has normalized the positive-flip cases — so we
            // only catch the residual `if (x == 0)` / equality guards.
            act!(ActionBranchFlip::boxed("blockrecovery", false)),
            act!(Box::new(crate::kuna_compareform::ActionPresentCompareForm::new(false, "presentcompare"))),
            act!(ActionOutputPrototype::boxed("localrecovery")),
            act!(ActionInputPrototype::boxed("fixateproto")),
            act!(ActionMapGlobals::boxed("fixateglobals")),
            act!(ActionDynamicSymbols::boxed("dynamic")),
            act!(ActionNameVars::boxed("merge")),
            act!(ActionSetCasts::boxed("casts")),
            act!(ActionFinalStructure::boxed("blockrecovery")),
            // (kuna) almostregion (option `almostregion`, default-OFF - report only).
            // The SAILR goto premise turned into a query: a residual `goto` marks
            // where the compiler restructured control flow, so delete each
            // virtualized edge from a scratch region graph over `bblocks` and report
            // the single-entry regions that appear only once it is gone, as
            // candidate inlined-callee bodies.
            // Runs IMMEDIATELY after the tree is final and BEFORE gotoreduce /
            // taildup / ifelseflatten / crossjumprevert / returndup: those passes
            // exist to *delete* gotos by duplication, and they consume ~19% of the
            // raw virtualized-edge set (measured: 3,513 raw vs 2,863 surviving over
            // 1,483 O2 functions).  Reading the residue instead of the raw set would
            // silently drop the evidence this pass is built to read.
            // Touches no p-code and no structured node, so OFF is byte-identical.
            act!(crate::p8_structure::kuna_almostregion::ActionAlmostRegion::boxed("blockrecovery")),
            // (kuna) angr SAILR return-tail goto-reduction (option `gotoreduce`,
            // default-OFF).  Runs after the tree is final + goto targets labelled.
            act!(crate::p8_structure::kuna_gotoreduce::ActionGotoReduce::boxed("blockrecovery")),
            // (kuna) angr SAILR `ReturnDuplicatorLow` return-tail-with-calls
            // duplication (option `taildup`, default-OFF).  Sibling of gotoreduce
            // for the return tail that *contains a call* (e.g. `free(p); return;`),
            // which gotoreduce rejects and crossjumprevert (non-return) declines.
            // Runs right after gotoreduce — same return-tail family.
            act!(crate::p8_structure::kuna_taildup::ActionTailDup::boxed("blockrecovery")),
            // (kuna) angr `IfElseFlattener`: after the tree is final + goto
            // targets labelled (and after gotoreduce), drop the `else` arm of a
            // 3-component `if` whose true-clause terminates, re-parenting the
            // else body as a follower (option `ifelseflatten`, default-OFF).
            act!(crate::p8_structure::kuna_ifelseflatten::ActionIfElseFlatten::boxed("blockrecovery")),
            // (kuna) angr SAILR cross-jump reversion (option `crossjumprevert`,
            // default-OFF).  Runs *after* gotoreduce — per the CrossJumpReverter
            // docstring it is the last deoptimization, so return tails are already
            // structured early returns and only true cross-jump tails remain.
            act!(crate::p8_structure::kuna_crossjumpreverter::ActionCrossJumpReverter::boxed("blockrecovery")),
            // (kuna) angr structurer ITE region-dedup (option `dedupitetail`,
            // default-OFF).  The inverse of the duplication passes above: after the
            // tree is final + goto targets labelled, merge a duplicated `if/else`
            // tail (shared prefix/suffix of statement-equivalent leaves across both
            // arms) by hoisting the shared blocks out of the `if`, emitting one copy.
            act!(crate::p8_structure::kuna_dedupitetail::ActionDedupIteTail::boxed("blockrecovery")),
            // (kuna) angr ITERegionConverter (option `iteregion`, default-OFF — a
            // runtime choice).  After the tree is final + goto targets labelled,
            // mark a narrow two-arm assignment diamond (`if (c) v=A; else v=B;`,
            // both arms a single COPY to the same variable) so the S9 printer emits
            // it as a `?:` ternary (`v = c ? A : B;`).  Print-only mark; no p-code
            // is touched, so OFF is byte-identical.
            act!(crate::p8_structure::kuna_iteregion::ActionIteRegion::boxed("blockrecovery")),
            // (kuna) iteboolean (option `iteboolean`, default-OFF - a runtime
            // choice).  Mark a `0`/`1` select diamond whose condition is a folded
            // short-circuit chain so the S9 printer emits `v = (a && b);` instead
            // of the constant if/else P3 RuleConditionalMove cannot fold.
            // Print-only mark; no p-code is touched, so OFF is byte-identical.
            act!(crate::p8_structure::kuna_iteboolean::ActionIteBoolean::boxed("blockrecovery")),
            // (kuna) paramcopyhoist (option `paramcopyhoist`, default-OFF - a runtime
            // choice).  Re-anchors an unmodified incoming parameter's trim COPY in the
            // entry block so its copy-shadow prints with the other parameter spills.
            // Runs LAST, after every HighVariable is final AND the structured tree is
            // final: the Cover legality test sees the variable's complete live range,
            // and no merge or structuring decision can be perturbed by the move --
            // only which basic block's statement list holds the COPY changes.
            act!(crate::p6_variables::kuna_paramcopyhoist::ActionParamCopyHoist::boxed("merge")),
            act!(ActionPrototypeWarnings::boxed("protorecovery")),
            act!(ActionStop::boxed("base")),
        ],
    }
}

// =============================================================================
// Build + install
// =============================================================================

/// Build the concrete universal Action tree (materialized from
/// [`universal_sched`]).
pub fn build_universal_action(
    stackspace: Option<Rc<AddrSpace>>,
    stackspace_index: Option<i32>,
    extra_pool_rules: Vec<RuleRow>,
) -> Box<dyn Action> {
    universal_sched(stackspace, stackspace_index, extra_pool_rules).materialize()
}

/// Install the universal tree and the default root group sets into an
/// [`ActionDatabase`] — the C++ `ActionDatabase::universalAction(conf)` +
/// `buildDefaultGroups()` pair.  After this, `db.set_current("decompile")`
/// derives the filtered "decompile" root (C++ `setCurrent`).
pub fn install_universal(
    db: &mut ActionDatabase,
    stackspace: Option<Rc<AddrSpace>>,
    stackspace_index: Option<i32>,
    extra_pool_rules: Vec<RuleRow>,
) {
    crate::action::build_default_groups(db);
    db.set_universal(build_universal_action(stackspace, stackspace_index, extra_pool_rules));
}

#[cfg(test)]
mod tests;

//! Port of `decompiler/cpp/architecture.{cc,hh}` (item `w4-fw-architecture`) —
//! the [`Architecture`] god object: the \e owner of the disassembly engine
//! (`Translate`/Sleigh), the symbol [`Database`], the [`OptionDatabase`], the
//! [`ActionDatabase`], the [`UserOpManage`], the p-code injection library, the
//! [`ContextDatabase`], plus the protection/read-only flags and the whole bag of
//! analysis-tuning configuration values.
//!
//! ## What this port wires vs. what it seams
//!
//! The C++ `Architecture` is the single largest class in the decompiler and
//! reaches into nearly every subsystem.  This port faithfully ports the parts
//! whose dependencies already exist in the kuna Rust tree, and seam-notes the
//! rest:
//!
//! - **Wired now**: the configuration fields and the kuna anchor flags (a
//!   verbatim transcription of `resetDefaultsInternal`, `architecture.cc:1420`);
//!   ownership of the `Translate` engine, the [`Database`] symbol table (with its
//!   global scope, C++ `buildDatabase`), the [`OptionDatabase`], the
//!   [`ActionDatabase`], the [`UserOpManage`], and the SLEIGH-backed p-code
//!   injection library; the `getModel`/`hasModel` registry lookups; the
//!   `getMinimumLanedRegisterSize`/`getLanedRegister` laned-register lookups;
//!   `nameFunction` (the kuna angr-style and upstream `func_` policies); and the
//!   construction of a [`Funcdata`] tied to this architecture (the W3 boot seam:
//!   `vbank`'s analysis unique-start comes from `Translate::getUniqueStart`).
//!
//! - **Seamed**: the data-type factory ([`crate::dtype`], W6), the prototype
//!   models (`fspec`, W6), the print language (W8), the loader (`loadimage`, its
//!   own item), the read-only/volatile/global-range decode (needs the W6 type
//!   factory + W4 symbol markup), and the full [`Architecture::init`] /
//!   `restoreFromSpec` flow (it builds the translator, type group, print
//!   language, and runs the spec decode — all reaching W6/W8 subsystems).  The
//!   `restoreXml`/`encode` marshaling and the segmented-pointer resolver are
//!   likewise deferred to their dependency waves.  Each is documented inline with
//!   `// SEAM(...)`.
//!
//! ## The kuna anchor flags
//!
//! `architecture.cc`/`.hh` carry a block of kuna-specific boolean flags (the
//! `(kuna)`-marked members `present_lessequal`, `fold_flag_compare`,
//! `strip_stack_guard`, …) that the kuna stage-model sub-stage fixes read.  They
//! are ported here as plain `bool` fields on [`Architecture`] (defaulted by
//! `resetDefaultsInternal`); the `w4-kuna-p0-pack` item's `OptionValues` alias
//! them through the option surface.  Public getters/setters are provided so the
//! p0-pack can read/flip each without owning the struct layout.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::space::{AddrSpace, AddrSpaceManager};
use kuna_base::types::{int4, uint4, uintb};

use kuna_sleigh::sleigh::Sleigh;
use kuna_sleigh::translate::{Translate, UniqueLayout};

use crate::action::ActionDatabase;
use crate::database::Database;
use crate::dtype::{type_metatype, TypeFactory, TypeFactoryImpl};
use crate::flow::flow_flags;
use crate::fspec::ProtoModel;
use crate::funcdata::Funcdata;
use crate::inject_sleigh::PcodeInjectLibrarySleigh;
use crate::options::{
    split_datatype, ArchOptionContext, BraceCategory, NamespaceStrategy, OptionDatabase,
};
use crate::printc::PrintC;
use crate::seams::{ArchHandle, Architecture as ArchSeam};
use crate::userop::UserOpManage;

// ---------------------------------------------------------------------------
// cspec XML helpers (the `<default_proto>` decode in build_default_proto reads
// the resolved compiler-spec through the kuna-base XML `Element` tree, the same
// parser the frontend uses for the binaryimage — see decode_default_proto).
// ---------------------------------------------------------------------------

/// First direct child element named `nm`, or `None`.
fn find_child(el: &Rc<kuna_base::xml::Element>, nm: &str) -> Option<Rc<kuna_base::xml::Element>> {
    el.get_children().iter().find(|c| c.get_name() == nm).map(Rc::clone)
}

/// String value of attribute `nm` on `el`, or `None` if absent.
fn attr_str(el: &Rc<kuna_base::xml::Element>, nm: &str) -> Option<String> {
    el.get_attribute_value(nm).ok().map(|b| String::from_utf8_lossy(b).into_owned())
}

/// Parse a decimal or `0x`-hex integer offset (C++ `<addr offset>` is a hex
/// string for register-space addresses, decimal otherwise).
fn parse_int(s: &str) -> Option<uintb> {
    let s = s.trim();
    if let Some(hex) = s.strip_prefix("0x") {
        uintb::from_str_radix(hex, 16).ok()
    } else {
        s.parse::<uintb>().ok()
    }
}

// ---------------------------------------------------------------------------
// Warning sink (the CommentDatabase slice the Funcdata warning path needs)
// ---------------------------------------------------------------------------

/// Comment-type bits the warning path keys on (C++ `Comment::comment_type`,
/// `comment.hh:53`).  Only the two warning kinds are reachable from the
/// architecture's warning sink; the full enum lands with the `comment.cc` item.
pub mod comment_type {
    #![allow(non_upper_case_globals)]
    use kuna_base::types::uint4;
    /// Auto-generated alert comment at an instruction (C++ `Comment::warning`).
    pub const warning: uint4 = 16;
    /// Auto-generated alert comment in the function header
    /// (C++ `Comment::warningheader`).
    pub const warningheader: uint4 = 32;
}

/// One stored warning comment (the slice of C++ `Comment` the warning sink
/// records: type + function address + instruction address + text).
///
/// SEAM(comment.cc): the full `CommentDatabase` (ordered set, de-duplication,
/// encode) is its own item; this is the minimal sink `Funcdata::warning`/
/// `warningHeader` (`funcdata.cc:119`) need, so the architecture can record an
/// analysis warning without the whole comment subsystem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchWarning {
    /// `Comment::warning` or `Comment::warningheader`.
    pub tp: uint4,
    /// Entry address of the function the comment belongs to (C++ `fad`).
    pub func_addr: Address,
    /// Instruction address the comment is attached to (C++ `ad`).
    pub addr: Address,
    /// The comment text (already prefixed with "WARNING: " by the caller).
    pub text: String,
}

/// \brief A minimal stand-in for the C++ `CommentDatabase` warning sink.
///
/// SEAM(comment.cc): `decompiler/cpp/comment.{cc,hh}` is a separate port item.
/// `Architecture` owns this so the [`Funcdata::warning`](crate::funcdata::Funcdata)
/// path (when it lands) has a place to deposit a warning; `add_comment_no_duplicate`
/// transcribes the *de-duplication contract* of C++
/// `CommentDatabaseInternal::addCommentNoDuplicate` (drop a comment whose
/// (fad,ad,text) triple already exists) while leaving the full ordered-set
/// encode/uniq machinery to the comment item.
#[derive(Debug, Clone, Default)]
pub struct CommentDatabase {
    comments: Vec<ArchWarning>,
}

impl CommentDatabase {
    /// Construct an empty comment database.
    pub fn new() -> CommentDatabase {
        CommentDatabase::default()
    }

    /// Store a comment unless an identical (type-agnostic on the address keys,
    /// text-matching) comment is already present (C++
    /// `CommentDatabaseInternal::addCommentNoDuplicate`, returns `true` if added).
    ///
    /// The C++ de-dup scans comments at the same (fad,ad) for matching text and
    /// drops the duplicate.  This carries exactly that predicate.
    pub fn add_comment_no_duplicate(
        &mut self,
        tp: uint4,
        fad: &Address,
        ad: &Address,
        txt: &str,
    ) -> bool {
        for existing in self.comments.iter() {
            if &existing.addr == ad && &existing.func_addr == fad && existing.text == txt {
                // Matching text, don't store it (C++ deletes newcom, returns false).
                return false;
            }
        }
        self.comments.push(ArchWarning {
            tp,
            func_addr: fad.clone(),
            addr: ad.clone(),
            text: txt.to_string(),
        });
        true
    }

    /// Store a comment unconditionally (C++ `CommentDatabase::addComment`, the
    /// console `comment instr` path which — unlike the warning sink — does not
    /// de-duplicate).
    pub fn add_comment(&mut self, tp: uint4, fad: &Address, ad: &Address, txt: &str) {
        self.comments.push(ArchWarning {
            tp,
            func_addr: fad.clone(),
            addr: ad.clone(),
            text: txt.to_string(),
        });
    }

    /// All recorded warnings, in insertion order (for inspection/tests).
    pub fn comments(&self) -> &[ArchWarning] {
        &self.comments
    }

    /// Clear all stored comments (C++ `CommentDatabase::clear`).
    pub fn clear(&mut self) {
        self.comments.clear();
    }
}

// ---------------------------------------------------------------------------
// Architecture
// ---------------------------------------------------------------------------

/// \brief Manager for all the major decompiler subsystems (C++ `class
/// Architecture : public AddrSpaceManager`, `architecture.hh:165`).
///
/// In C++ the `Architecture` *is-an* `AddrSpaceManager`; in the Rust port the
/// address-space manager lives inside the owned `Translate` engine (the Sleigh
/// `SleighBase` *is* the manager), so [`Architecture::manage`] forwards to it.
/// The W3 IR boundary ([`Funcdata::glb`](crate::funcdata::Funcdata)) takes a
/// lightweight [`ArchSeam`] handle carrying just the address-space slice it
/// reaches (built by [`Architecture::new_funcdata`]); the heavy subsystems live
/// here.
pub struct Architecture {
    /// ID string uniquely describing this architecture (C++ `archid`).
    pub archid: String,

    // --- Configuration data (architecture.hh:170-208) ---------------------
    /// How many levels to let parameter trims recurse (C++ `trim_recurse_max`).
    pub trim_recurse_max: int4,
    /// Maximum number of references to an implied var (C++ `max_implied_ref`).
    pub max_implied_ref: int4,
    /// Max terms duplicated without a new variable (C++ `max_term_duplication`).
    pub max_term_duplication: int4,
    /// Maximum "integer" type size before creating an array type
    /// (C++ `max_basetype_size`).
    pub max_basetype_size: int4,
    /// Minimum size of a function symbol (C++ `min_funcsymbol_size`).
    pub min_funcsymbol_size: int4,
    /// Maximum number of entries in a single JumpTable (C++ `max_jumptable_size`).
    pub max_jumptable_size: uint4,
    /// Aggressively trim inputs that look sign-extended (C++ `aggressive_ext_trim`).
    pub aggressive_ext_trim: bool,
    /// Treat readonly values as constants (C++ `readonlypropagate`).
    pub readonlypropagate: bool,
    /// Infer pointers from likely-address constants (C++ `infer_pointers`).
    pub infer_pointers: bool,
    /// How many bits of alignment a function ptr has (C++ `funcptr_align`).
    pub funcptr_align: int4,
    /// Options passed to the flow-following engine (C++ `flowoptions`).
    pub flowoptions: uint4,
    /// Maximum instructions processed in one function (C++ `max_instructions`).
    pub max_instructions: uint4,
    /// Aliases blocked by 0=none,1=struct,2=array,3=all (C++ `alias_block_level`).
    pub alias_block_level: int4,
    /// Data-type-splitting toggle bits (C++ `split_datatype_config`).
    pub split_datatype_config: uint4,
    /// Attempt whiledo->for loop conversion (C++ `analyze_for_loops`).
    pub analyze_for_loops: bool,
    /// Ignore NaN ops entirely, nan() always false (C++ `nan_ignore_all`).
    pub nan_ignore_all: bool,
    /// Ignore NaN ops protecting float comparisons (C++ `nan_ignore_compare`).
    pub nan_ignore_compare: bool,
    /// True if loader symbols have been read (C++ `loadersymbols_parsed`).
    pub loadersymbols_parsed: bool,
    /// Ordered list of address spaces in which a constant pointer can be inferred
    /// (C++ `Architecture::inferPtrSpaces`, architecture.hh).  Seeded by the cspec
    /// `<global>` tag (`addToGlobalScope` pushes each global range's space) and
    /// finalized by [`cache_addr_space_properties`](Architecture::cache_addr_space_properties)
    /// (sort/dedup/filter, always include the default code+data spaces, promote the
    /// default data space to position 0).  Shared onto the per-function `glb` so
    /// `ActionConstantPtr::selectInferSpace` (coreaction.cc:1020-1047) can pick the
    /// space a likely-pointer constant addresses.  Held as `Rc<AddrSpace>` (the
    /// shared LOSS-132 space identities).
    pub infer_ptr_spaces: Vec<Rc<AddrSpace>>,

    // --- kuna anchor flags (architecture.hh:179-201, the `(kuna)` members) -
    /// (kuna GH-6930) Infer single-bit constants matching an exact function
    /// entry as pointers (C++ `infer_funcentry`).
    pub infer_funcentry: bool,
    /// (kuna GH-6990) Keep multi-register return values single, un-joined
    /// (C++ `return_single`).
    pub return_single: bool,
    /// (kuna GH-9230) Recover constant-fill store/copy runs as `builtin_memset`
    /// (C++ `memset_recover`).
    pub memset_recover: bool,
    /// (kuna GH-8913) Fuse 8-bit carry-chain 16-bit adds into one wide add
    /// (C++ `add_carry_chain`).
    pub add_carry_chain: bool,
    /// (kuna GH-8817) Reclassify V850 `jmp [reg]` CALLIND to BRANCHIND
    /// (C++ `v850_indirect_branch`).
    pub v850_indirect_branch: bool,
    /// (kuna tee-O2 tail-jumps) Recover a direct `jmp` to another function's
    /// entry (e.g. `jmp setlocale@plt`) as a tail call (CALL + RETURN) instead of
    /// flowing into the callee (`option tailcalljump`, default off).
    pub tail_call_jumps: bool,
    /// (kuna) Treat a direct CALL whose resolved callee display name matches a
    /// known ELF no-return name (`__stack_chk_fail`, `abort`, `exit`, …) as
    /// no-return at flow time, even when the address-keyed no-return flag is unset
    /// — the undefined-extern (`ET_REL .o`) case the analysis-tier `noreturn_known`
    /// pass cannot reach (`option noreturn_extern`, default off).
    pub noreturn_extern_calls: bool,
    /// (kuna GH-6882) Let a SPARC struct-return post-call `unimp` fall through
    /// (C++ `sparc_struct_return`).
    pub sparc_struct_return: bool,
    /// (kuna GH-7190) Collapse the OV-flag signed-less-than idiom to INT_SLESS
    /// (C++ `ov_less_simplify`).
    pub ov_less_simplify: bool,
    /// (kuna GH-1282) Fold `(b<<k) s>> k` boolean sign-extension-mask idioms
    /// (C++ `fold_boolean_mask`).
    pub fold_boolean_mask: bool,
    /// (kuna GH-9218) Absorb overlapping input Varnodes above a justified
    /// container (C++ `input_varnode_adjust`).
    pub input_varnode_adjust: bool,
    /// (kuna GH-9203) Decline placing a const COPY in a loop block
    /// (C++ `condexe_block_placement`).
    pub condexe_block_placement: bool,
    /// (kuna GH-8467) Raise DynamicHash same-address collision budget 8->16
    /// (C++ `dynamic_hash_maxdup_high`).
    pub dynamic_hash_maxdup_high: bool,
    /// (kuna GH-8017) Resolve gcc stack-probe loop SP MULTIEQUAL to a constant
    /// (C++ `model_stack_probe_loop`).
    pub model_stack_probe_loop: bool,
    /// (kuna GH-1276/8777) Fold flag-modelled comparison idioms
    /// (C++ `fold_flag_compare`).
    pub fold_flag_compare: bool,
    /// (kuna GH-9191) Bound a modulo/and-mask LOAD-table jumptable index
    /// (C++ `switch_modulo_bound`).
    pub switch_modulo_bound: bool,
    /// (kuna, angr `test_decompiling_missing_function_call`) Bound a LOAD-table
    /// jumptable index by an out-of-band CBRANCH range guard the basic model's
    /// guard analysis could not turn into a bound (C++ `switch_guard_bound`).
    pub switch_guard_bound: bool,
    /// (kuna, angr `test_switch_case_shared_case_nodes_b2sum_digest`) Recover a
    /// GCC PIC relative-offset jump table whose base register is a loop-carried
    /// MULTIEQUAL (the `lea .rodata` table base is set before a getopt-style loop
    /// while the `BRANCHIND` is inside it).  The path-meld collapses to the final
    /// `base+offset` add, so the CBRANCH range guard on the load index never
    /// bounds it; this rebuilds the meld as a clean single path down to the
    /// guarded index so the table resolves (C++ `switch_shared_case`).
    pub switch_shared_case: bool,
    /// (kuna, angr `test_decompiling_abnormal_switch_case_case3`) Recover an
    /// image-base-relative jump table whose bound guard is "unrolled" /
    /// duplicated across MULTIPLE predecessors of the dispatch block (the
    /// `BRANCHIND` parent has `sizeIn() > 1`, each incoming block ending in its
    /// own copy of the bound CBRANCH, the per-path switch indices meeting in a
    /// MULTIEQUAL).  Ports the upstream multi-predecessor unrolled-guard
    /// machinery (`JumpBasic::checkCommonCbranch` + `BlockBasic::findMultiequal`
    /// + `BlockBasic::liftVerifyUnroll`) into `JumpBasic::checkUnrolledGuard`,
    /// lifting the common guard onto the MULTIEQUAL output so the table bounds
    /// (C++ `switch_multi_pred`).
    pub switch_multi_pred: bool,
    /// (kuna, angr `test_decompiling_optimized_memcpy`) Recover the interleaved
    /// jump tables of an MSVC optimized memcpy/memmove (Duff's device).  When a
    /// function holds several jump tables whose case bodies are reachable only as
    /// one another's case targets, kuna recovers them one at a time, each in its
    /// own fresh partial-flow clone; a later table's clone re-clones an
    /// already-recovered sibling table into its jumpvec, and that partial's
    /// `collect_edges` then calls `target()` on a sibling case body that was
    /// never decoded into this partial's `visited` (it is only decoded into the
    /// PARENT flow after the recovery pass returns), throwing
    /// "Could not find op at target address" and degrading the dispatch to a
    /// computed call.  Upstream avoids this by building one shared partial and
    /// running `collectEdges` once while the sibling tables are still empty; this
    /// gate makes the partial-clone `collect_edges` SKIP an unresolvable
    /// recovered-table case-target edge (the same "assume no branches out" shape
    /// the `findJumpTable==0` partial path already uses) instead of throwing
    /// (C++ `unrolled_guard`).
    pub unrolled_guard: bool,
    /// (kuna, angr `test_decompiling_incorrect_duplication_chcon_main`) Treat a
    /// direct CALL to a function whose *name* matches the vendored ELF
    /// known-no-return list as no-return at the `query_call_no_return` flow seam,
    /// even when the address-keyed `noreturn_known` scan emitted no fact (an
    /// ET_REL `.o` undefined extern such as `__stack_chk_fail`). DIV-13 default-on
    /// (clean 0/675 ablation; a no-op on a normal ELF since the proto flag is
    /// already set). See `kuna_noreturn_externmatch`.
    pub noreturn_extern_match: bool,
    /// (kuna GH-8500) Hold a store-through-a-stack-pointer-alias across the
    /// deadcode race (C++ `stack_alias_deadstore`).
    pub stack_alias_deadstore: bool,
    /// (kuna GH-8724) Re-express a strided-induction offset as counter*stride
    /// (C++ `recover_array_stride`).
    pub recover_array_stride: bool,
    /// (kuna) Reconstruct a compiler-lowered comparison cascade into a switch
    /// (C++ `recover_lowered_switch`).
    pub recover_lowered_switch: bool,
    /// (kuna) Region-based (Phoenix/SAILR) structurer: structure the CFG by
    /// walking the [`KunaRegionIdentifier`](crate::s7_regions::kuna_regionid)
    /// region tree and matching Phoenix acyclic schemas instead of running
    /// Ghidra's `CollapseStructure` (option `regionstructure`, DIV-12 default-on:
    /// the primary structuring path; falls back to `CollapseStructure` on irreducible code).
    pub region_structure: bool,
    /// (kuna) Region structurer cyclic loop-successor refinement: when
    /// `region_structure` is on, refine a multi-exit / multi-latch (or
    /// irreducible mid-entry) loop by virtualizing its *secondary* exits and
    /// latches to gotos (lowered to `break;`/`continue;` by the existing
    /// `scopeBreak`/loop-construction passes) so the loop folds into a structured
    /// `while`/`do-while`/`for`/inf-loop instead of falling back to
    /// `CollapseStructure` (option `regionlooprefine`, default-OFF opt-in).  A
    /// strict superset of the cyclic schemas: a loop the base schemas already fold
    /// is untouched (so reducible code stays byte-identical); only loops that
    /// would otherwise fall back are refined.  Port of angr `RegionIdentifier`'s
    /// `_refine_loop_successors_to_guarded_successors` /
    /// `_ensure_jump_at_loop_exit_ends` (the `force_loop_single_exit` path).
    pub region_loop_refine: bool,
    /// (kuna) Region structurer last-resort edge-virtualization ORDERING (SAILR P2):
    /// when the structurer must virtualize an edge to a `goto` (no schema applies),
    /// pick the order that minimizes the resulting goto count.  Replaces the flat
    /// H1/H3 + block-index tiebreak with angr's `_last_resort_refinement` dominance-
    /// tiered bucketing (crossing / secondary / other via forward immediate-
    /// dominators) and the SAILR `_order_virtualizable_edges` H2 post-dominator
    /// heuristic (with the `postdom_max_edges` ≈ 10 / `postdom_max_graph_size` ≈ 50
    /// caps so post-dom computation stays bounded).  Option `regionedgeorder`,
    /// default-OFF opt-in: OFF ⇒ the existing H1/H3 + address ordering, so output is
    /// byte-identical (on reducible code the structurer never virtualizes, so the
    /// reordering is unobservable — this only changes WHICH goto is chosen when the
    /// structurer is already forced to emit one).  Port of angr SAILR
    /// `phoenix._last_resort_refinement` + `sailr._order_virtualizable_edges`.
    pub region_edge_order: bool,
    /// (kuna) angr SAILR goto-reduction: duplicate a small return tail into a
    /// `goto` source so the cross-edge becomes a structured early return
    /// (`reduce_return_gotos`).
    pub reduce_return_gotos: bool,
    /// (kuna) angr `IfElseFlattener`: drop the `else` arm of a 3-component `if`
    /// whose true-clause is statement-terminating, re-parenting the else body as
    /// the `if`'s follower (`flatten_ifelse`).
    pub flatten_ifelse: bool,
    /// (kuna) angr SAILR `CrossJumpReverter`: revert compiler cross-jumping by
    /// duplicating a small *non-return* cross-jump tail into the `goto` source so
    /// both paths fall straight through (`revert_cross_jumps`, opt-in default-off).
    pub revert_cross_jumps: bool,
    /// (kuna) angr SAILR `ReturnDuplicatorLow`: duplicate a small **return tail that
    /// contains a call** (e.g. `free(p); return;`) into a `goto` source so the
    /// cross-edge becomes a structured early return.  Fills the gap between
    /// `gotoreduce` (return tail, no calls) and `crossjumprevert` (non-return tail,
    /// calls allowed) — angr's `max_calls_in_regions` budget (`dup_return_call_tails`,
    /// opt-in default-off).
    pub dup_return_call_tails: bool,
    /// (kuna) angr structurer ITE region-dedup: merge a duplicated `if/else` tail
    /// (a maximal common prefix/suffix of statement-equivalent leaves shared by both
    /// arms) by hoisting the shared blocks out of the `if` — emitting one copy
    /// instead of two.  The inverse of the SAILR duplication passes
    /// (`gotoreduce`/`crossjumprevert`/`taildup`) (`dedup_ite_tail`, opt-in
    /// default-off).
    pub dedup_ite_tail: bool,
    /// (kuna) angr `ITERegionConverter`: rewrite a two-arm assignment *diamond*
    /// (`if (c) v = A; else v = B;`, both arms a single COPY to the same
    /// variable, converging on one tail) to a `?:` ternary (`v = c ? A : B;`).
    /// A deliberate **runtime choice** (`iteregion`, opt-in default-off): the
    /// rewrite matches the source only when the source used a ternary — common in
    /// format/print/flag code (`flags ? "%s," : "%s"`) — and diverges when the
    /// source used explicit if/else, so an agent flips it per function.
    pub iteregion: bool,
    /// (kuna) angr SAILR gotoless `ReturnDuplicatorHigh`: duplicate a shared
    /// **bare-epilogue** RETURN block (only MULTIEQUAL/COPY/RETURN, no side effects)
    /// into each predecessor but one, so the classic
    /// `if (c) { body; return X; } return Y;` guard shape structures as
    /// per-predecessor early returns instead of one comma-folded exit — the gotoless
    /// complement to `ActionReturnSplit` (the goto-driven `ReturnDuplicatorLow`)
    /// (`duplicate_shared_returns`, opt-in default-off).
    pub duplicate_shared_returns: bool,
    /// (kuna) Hoist a leading const-guard into an early return (`if (c) return K;`) by
    /// peeling only the CONSTANT arm of a mixed return phi — the per-edge narrowing of
    /// angr SAILR `ReturnDuplicatorHigh` that `duplicate_shared_returns`' whole-block
    /// const gate cannot reach (`early_return`, opt-in default-off).
    pub early_return: bool,
    /// (kuna) Lower loop-exit `goto <successor>` edges to structured `break;`
    /// (a port of Ghidra `BlockGraph::scopeBreak`; option `loopbreak_recovery`,
    /// DIV-10 default-on).
    pub recover_loop_break: bool,
    /// (kuna) Fold an order-safe single-use call return into its use site
    /// (`fold_call_returns`, opt-in default-off; angr "call return variable
    /// folding").
    pub fold_call_returns: bool,
    /// (kuna) Strip the glibc -fstack-protector canary epilogue
    /// (C++ `strip_stack_guard`).
    pub strip_stack_guard: bool,
    /// (kuna) Flip negated-guard if/else branches for linearity: when an
    /// `if (x == 0) {A} else {B}` (equality-to-zero / negated guard) can be flipped
    /// in place, rewrite it to the positive `if (x) {B} else {A}` so the common
    /// path reads top-to-bottom (angr-style `if (x)` vs `if (x == 0)`).  Default
    /// OFF (option `branchflip`); read by `ActionBranchFlip` (S8).
    pub branch_flip: bool,
    /// (kuna) Use angr-style default naming (vN/aN/dat_/sub_/label_ + comments)
    /// (C++ `name_style_angr`).
    pub name_style_angr: bool,
    /// (kuna) Collapse local-variable declarations whose fully-rendered line is
    /// identical (the scalar analogue of the composite-symbol decl collapse), so a
    /// stack slot mapped onto many same-named HighVariables is declared once
    /// (`option dedupvardecls`; angr-inspired, S9).
    pub dedup_var_decls: bool,
    /// (kuna DIV-6) Render residual `TYPE_UNKNOWN` (`xunknownN`) values as real C
    /// types by size — 1→`char`, 2/4/8→unsigned ints, pointer-to-unknown→`void *` —
    /// instead of the `xunknownN`/`undefined<N>` placeholder.  Default-on; read by
    /// the printc declarator chokepoints (`RealTypeCtx`).
    pub realtypes: bool,
    /// (kuna GH-558) Restore canonicalized comparisons to LESSEQUAL form for
    /// presentation (C++ `present_lessequal`).
    pub present_lessequal: bool,
    /// (kuna GH-8471) Keep mode-bit-encoded (Thumb) function pointers symbolic
    /// (C++ `preserve_thumb_funcptr`).
    pub preserve_thumb_funcptr: bool,
    /// (kuna decompile-all watchdog) Optional wall-clock budget for ONE
    /// function's decompile drive (`kuna decompile-all --max-fn-seconds N`).
    /// `None` (the default) means no budget: the console/`decomp_dbg` parity
    /// path never sets it, so the datatest pipeline is structurally unaffected.
    /// Driver policy, NOT a stage-model settable: it changes zero output for a
    /// function that converges — it only bounds a non-converging one.
    pub kuna_fn_budget: Option<std::time::Duration>,
    /// (kuna decompile-all watchdog) The live deadline for the CURRENT
    /// function's drive, computed from [`kuna_fn_budget`](Self::kuna_fn_budget)
    /// at the top of `decompile_func_full_with_override_dyn` and consulted
    /// cooperatively at the action/rule-pool/heritage loop boundaries
    /// ([`ActionContext::deadline`](crate::action::ActionContext)).  Always
    /// `None` when no budget is set.
    pub kuna_fn_deadline: Option<std::time::Instant>,

    // --- kuna analysis-pass gates (per-run `--option <id> on|off`) ----------
    // One boolean per `kuna_analysis::passes` pass id; the console's
    // `commit_analysis_output` consults these at `read symbols` and skips a
    // disabled pass's facts.  The kuna analog of Ghidra's
    // `AbstractAnalyzer.setDefaultEnablement` per-analyzer enablement (a Run
    // Analysis on/off toggle), bound to the real-ELF path only (the XML datatest
    // path never produces analysis facts, so these are structurally inert there).
    // Default-on (matching Ghidra's default-on analyzers) except `addrtable`,
    // which Ghidra ships off (`AddressTableAnalyzer.setDefaultEnablement(false)`).
    /// (kuna) Gate the no-return-known pass (`noreturn_known`); default on.
    pub analysis_noreturn_known: bool,
    /// (kuna) Gate the library-prototype pass (`libproto`); default on.
    pub analysis_libproto: bool,
    /// (kuna) Gate the string-literal pass (`strings`); default on.
    pub analysis_strings: bool,
    /// (kuna) Gate the entry-discovery pass (`entry_disc`); default on.
    pub analysis_entry_disc: bool,
    /// (kuna) Gate the `.eh_frame` LSDA landing-pad discovery sub-feature of the
    /// always-on entry-discovery pass (`eh_frame_full`, the GccExceptionAnalyzer
    /// `.gcc_except_table` markup); default **off** (output-changing: adds the
    /// discovered exception-handler landing pads as function entries).
    pub analysis_eh_frame_full: bool,
    /// (kuna) Gate the **full byte-pattern function-start** pass
    /// (`funcstart_patterns`); default **off** (output-changing: it discovers more
    /// functions). The faithful port of Ghidra's `FunctionStartAnalyzer` over the
    /// entire vendored pattern corpus (`s1_entry/patterns/*.xml`, the
    /// `<patternpairs>` pre/post sequences + bare `<funcstart/>` patterns), as a
    /// SEPARATE pass from `entry_disc` (whose always-on oracle 5 ports only a
    /// minimal three-prologue subset). When on, a stripped binary recovers many
    /// more function starts (e.g. `push rbx; mov rbx,rdi` after NOP padding); the
    /// commit seam adds each as `sub_<addr>`, idempotent against the funcsym stream
    /// + the `entry_disc` entries. Default-off ⇒ the pass's facts are dropped at
    /// commit (`engine.rs::analysis_pass_enabled`) and every parity gate is
    /// byte-identical. Real-ELF/PE/Mach-O path only ⇒ the XML datatest oracle is
    /// structurally untouched.
    pub analysis_funcstart_patterns: bool,
    /// (kuna) Gate the ARM/Thumb decode-mode marker pass (`arm_markers`); default on.
    pub analysis_arm_markers: bool,
    /// (kuna) Gate the MIPS `$gp`-recovery (`t9` tracking) pass (`mips_gp`); default on.
    pub analysis_mips_gp: bool,
    /// (kuna) Gate the i386-PIE PLT-stub decode (`i386_pie_plt`); default on. The
    /// loader (`kuna-analysis::s1_loader::elf_plt::decode_i386`) decodes the
    /// GOT-relative `jmp *disp(%ebx)` (`FF A3 <disp32>`) PIE stub form so dynamic
    /// imports (`exit`/`dcgettext`/…) are named and `exit` is flagged no-return
    /// (collapsing the spurious fall-through loop). i386-only; a no-op on every
    /// other language. NOTE: the loader reads this through the
    /// [`crate::kuna_i386_pie_plt`] **env var** (the PLT map is baked at `load
    /// file`, upstream of `option`); this bool exists only for catalog visibility
    /// and the `stage catalog` live `current` field.
    pub analysis_i386_pie_plt: bool,
    /// (kuna) Gate the MIPS16 `ISA_MODE` decode-mode marker pass (`mips_isa`); default on.
    pub analysis_mips_isa: bool,
    /// (kuna) Gate the DWARF recovery pass (`dwarf`); default on.
    pub analysis_dwarf: bool,
    /// (kuna) Gate the DWARF `.debug_line` source-line comment pass (`dwarf_lines`);
    /// default **off** — it changes the decompiled output (adds `/* file:line */`
    /// comments). The kuna analog of Ghidra's `DWARFLineInfoCommentScript`.
    pub analysis_dwarf_lines: bool,
    /// (kuna) Gate the call-fixup pass (`callfixup`); default on.
    pub analysis_callfixup: bool,
    /// (kuna) Gate the address-table pass (`addrtable`); default **off** (matches
    /// Ghidra `AddressTableAnalyzer.setDefaultEnablement(false)`).
    pub analysis_addrtable: bool,
    /// (kuna) Gate the scalar/operand reference-markup pass (`operand_refs`); the
    /// kuna analog of Ghidra's `ScalarOperandAnalyzer`/`ElfScalarOperandAnalyzer`.
    /// Default **off**: `ScalarOperandAnalyzer.getDefaultEnablement` is `!isElf`
    /// (Ghidra ships the producing analyzer DISABLED for every ELF), the ELF
    /// subclass only *removes* bad `.got`/`.plt` refs kuna never creates, and the
    /// one useful product (a `.rodata` string typed `char*`) is already delivered
    /// by the always-on `s1_strings` + libproto/S5 typing — so a per-instruction
    /// immediate scan is net-negative (over-accepts). When on, it linear-decodes the
    /// executable sections and plants a typed `char[N]`+readonly fact for each
    /// scalar immediate that points into allocated read-only data. Real-ELF path
    /// only ⇒ the XML datatest oracle is structurally untouched.
    pub analysis_operand_refs: bool,
    /// (kuna) Gate the format-string varargs-typing behavior (`formatstring`,
    /// `FormatStringAnalyzer` half B); default **off** (matches Ghidra
    /// `FormatStringAnalyzer.setDefaultEnablement(false)`).  Unlike the other
    /// `analysis_*` flags this does NOT gate a load-time `AnalysisOutput` pass:
    /// `FormatStringAnalyzer` is `DecompilerDependent`, so the console's
    /// `IfcDecompile` reads this flag *after* the first decompile to decide
    /// whether to run the per-call-site printf/scanf varargs override loop and
    /// re-decompile.  Default-off ⇒ the loop is inert and every parity gate is
    /// byte-identical.
    pub analysis_formatstring: bool,
    /// (kuna) Gate the Listing/xref disassembly tier (`listing`); default
    /// **off**. When on (real-ELF path only), a program-wide recursive-descent
    /// disassembly Listing/xref model is built once at load and shared read-only
    /// with the consumer analysis passes. Default-off ⇒ the Listing is never
    /// built and every parity gate is byte-identical.
    pub analysis_listing: bool,
    /// (kuna) Gate the discovered-no-return consumer (`noreturn_disc`), the first
    /// Listing/xref consumer; default **off**. It is a flow heuristic (a callee is
    /// no-return if ≥3 of its call sites show no valid fall-through, iterated to a
    /// fixpoint over the Listing) that can be wrong, so it ships behind its own
    /// flag — the kuna analog of Ghidra's `FindNoReturnFunctionsAnalyzer`. Reads
    /// the Listing (`--option listing on` builds it); a no-op when the Listing is
    /// absent. Default-off ⇒ every parity gate is byte-identical.
    pub analysis_noreturn_disc: bool,
    /// (kuna) Gate the structural no-return **propagation** consumer
    /// (`noreturn_propagate`), the second Listing/xref consumer; default **off**.
    /// The kuna analog of angr's CFGFast call-graph no-return propagation: seed
    /// from the Known no-return set and conclude a function no-return when its last
    /// real instruction (skipping trailing NOP padding) is a call/tail-jump to an
    /// already-no-return callee, with no returning path — iterated to a fixpoint,
    /// with NO evidence threshold (unlike `noreturn_disc`). Catches custom
    /// no-return wrappers (e.g. `xalloc_die`) that the name list misses and the ≥3
    /// evidence rule does not reach. Reads the Listing (`--option listing on`
    /// builds it); a no-op when the Listing is absent. Default-off ⇒ every parity
    /// gate is byte-identical.
    pub analysis_noreturn_propagate: bool,
    /// (kuna, decbench F2) Gate the `error(status,…)`-conditional recognizer inside
    /// the `noreturn_propagate` consumer (`noreturn_error`); default **on**
    /// (DIV-16). glibc `error`/`error_at_line` never return WHEN their first
    /// argument (`int status`) is a nonzero constant — they call `exit(status)` —
    /// but *do* return for `status == 0`, so `error` cannot be a Known no-return.
    /// A wrapper whose tail is `call error(2,…)` (GNU `pfatal_with_name`, …) is
    /// nonetheless no-return; when on, the propagation treats such a tail call as
    /// terminal (arg0 = a nonzero literal, x86-64 SysV `EDI`/`RDI`), concludes the
    /// wrapper no-return, and its callers drop the dead fall-through. REMOVES CODE.
    /// Requires the Listing (`--option listing on`) AND `noreturn_propagate` on;
    /// a no-op otherwise, so every parity gate is byte-identical (real-ELF path).
    pub analysis_noreturn_error: bool,
    /// (kuna) Gate the CFG-reachability no-return rule (`noreturn_reach`), the port of
    /// Ghidra's `FindNoReturnFunctionsAnalyzer.targetOnlyCallsNoReturn`: a function is
    /// no-return iff no `RETURN` is reachable from entry once calls to already-no-return
    /// callees are treated as terminal. Generalizes `noreturn_propagate`'s tail-call rule
    /// to mid-body no-return calls, dead returns, and switch-of-no-return. Requires the
    /// Listing AND `noreturn_propagate` on; a no-op otherwise, so every parity gate is
    /// byte-identical (real-ELF path only). Default **on** (DIV-19).
    pub analysis_noreturn_reach: bool,
    /// (kuna, Ghidra-gap) `call error(nonzero,…)` call-site addresses whose fall-through
    /// the decompile seam must prune (as `CALL_RETURN` flow overrides). Populated at the
    /// analysis commit from `AnalysisOutput::no_fallthru_calls` (empty unless `listing` +
    /// `noreturn_error` are on); read by `decompile-all` per function. glibc `error()`
    /// with a nonzero status never returns, so without the prune the flow-follower walks
    /// past the call into the next function and absorbs it. Sorted/deduped.
    pub error_noreturn_callsites: Vec<u64>,
    /// (kuna) Gate the FID fingerprint matcher (`fid`), a Listing/xref consumer;
    /// default **off**. The kuna analog of Ghidra's FID identification analyzer:
    /// over the built Listing it fingerprints each function with the byte-exact
    /// operand-masked FNV-1a64 hash and looks the full hash up in a kuna `.fid`
    /// database (named by the `kuna_fid_db` env var), renaming a matched
    /// `FUN_*`/`sub_*` placeholder back to its library name — the capability that
    /// re-identifies a function in a STRIPPED binary (e.g. `sub_4017c0` →
    /// `kuna_crc32`). Reads the Listing (`--option listing on` builds it) and is a
    /// no-op without the Listing AND without a configured DB. Default-off, real-ELF
    /// path only ⇒ every parity gate is byte-identical.
    pub analysis_fid: bool,
    /// (kuna) Gate the MSVC RTTI / vftable recovery pass (`rtti`); default **off**.
    /// The kuna analog of Ghidra's `RttiAnalyzer` (a Microsoft-PE analyzer): on a
    /// Windows PE it parses the `CompleteObjectLocator` → RTTI3/2/1 → RTTI0 graph in
    /// `.rdata`/`.data`, demangles each `.?A…@@` class name, and emits
    /// `<Class>::vftable` / `<Class>::RTTI_Complete_Object_Locator` /
    /// `<Class>::RTTI_Type_Descriptor` labels so the C++ class names (`Box`/`Shape`)
    /// surface as recovered symbols and the virtual-dispatch metadata graph is
    /// named. PE-only (registered in `passes_for` only for `BinaryFormat::Pe`, and
    /// the pass also self-gates on PE in `run`), real-PE path only ⇒ every ELF/XML
    /// parity gate is byte-identical. Default-off (output-changing: it adds named
    /// data symbols); `--option rtti on` enables it.
    pub analysis_rtti: bool,
    /// (kuna) Gate the Aggressive Instruction Finder gap-walk (`aif`), the third
    /// Listing/xref consumer; default **off**. The kuna analog of Ghidra's
    /// `AggressiveInstructionFinderAnalyzer` (which ships `setDefaultEnablement(false)`
    /// with the warning *"IT MAY CREATE A LOT OF BAD CODE!"*): a speculative
    /// gap-filler that, over the undefined gaps between discovered functions,
    /// speculatively decodes each gap start and accepts it as a NEW function entry
    /// when it (a) disassembles into a valid subroutine (a clean RET, > 2
    /// instructions) AND (b) matches a function-start byte fingerprint shared by ≥ 4
    /// of the already-discovered functions. Finds functions reachable ONLY through
    /// an indirect/data path (a `.rodata` function-pointer table) that entry
    /// discovery + funcsyms miss. Reads the Listing (`--option listing on` builds
    /// it); a no-op when the Listing is absent. Default-off ⇒ every parity gate is
    /// byte-identical.
    pub analysis_aif: bool,
    /// (kuna) Gate the Go `pclntab` function-name recovery pass (`gopclntab`); the
    /// kuna analog of Ghidra's `GolangSymbolAnalyzer` (name-recovery half). Default
    /// **on**, but the pass is registered ONLY for a Go binary
    /// (`detect_compiler == Go`), so on every non-Go binary it is structurally
    /// absent regardless of this flag. Parses the embedded pclntab and emits a
    /// function symbol per Go function (so `main.main`/`runtime.*` render named
    /// instead of `sub_<addr>`). Real-ELF Go path only ⇒ the XML datatest oracle is
    /// structurally untouched.
    pub analysis_gopclntab: bool,
    /// (kuna) Gate the Mach-O Objective-C metadata recovery pass (`objc`); default
    /// **off**. The kuna analog of Ghidra's `ObjcTypeMetadataAnalyzer`
    /// (name-recovery half): when the binary is a Mach-O, walk the `__objc_*`
    /// metadata (classlist → class_t → class_ro_t → method_list_t) and rename each
    /// IMP function `-[Class sel]` / `+[Class sel]` (the FID-precedent label-gated
    /// rename of a `sub_*`/`FUN_*` placeholder), plus emit `_OBJC_CLASS_$_<name>`
    /// and selector symbols. Selectors are plain ASCII — no demangler needed. The
    /// pass is registered ONLY for a Mach-O binary, so on every non-Mach-O binary it
    /// is structurally absent regardless of this flag. Default-off, output-changing
    /// (it renames + adds symbols), real-binary-path only ⇒ every parity gate is
    /// byte-identical. x86-64, no-chained-fixups path (the arm64 +
    /// LC_DYLD_CHAINED_FIXUPS resolver is a deferred follow-on).
    pub analysis_objc: bool,
    /// (kuna) Gate the PE PDB metadata recovery pass (`pdb`); default **off**. The
    /// kuna analog of Ghidra's `PdbUniversalAnalyzer` (the pure-Java PDB analyzer;
    /// the MS-DIA `PdbAnalyzer` is Windows-native and out of scope) — the
    /// name-recovery half. On a Windows PE, read the CodeView fingerprint
    /// (`{guid, age, path}` from the debug directory), locate the external `.pdb`
    /// (tier-1: the `kuna_pdb_path` env var, the s1_fid `kuna_fid_db` precedent),
    /// **fingerprint-gate** it (the supplied `.pdb`'s `pdb_information().guid/age`
    /// must match the PE's CodeView record — a MISMATCH/ABSENT `.pdb` emits nothing,
    /// the FID full-hash-match discipline of never applying wrong external
    /// knowledge), and on a match walk the global symbols (`S_PUB32`/`S_GPROC32`) to
    /// RENAME each stripped `FUN_*`/`sub_*` function to its real name (the
    /// FID-precedent label-gated rename of a placeholder; a real symbol is never
    /// overwritten). The pass is registered ONLY for a PE binary, so on every non-PE
    /// binary it is structurally absent regardless of this flag. Default-off,
    /// output-changing (it renames + adds symbols), real-binary-path only (and inert
    /// without a fingerprint-matching `.pdb`) ⇒ every parity gate is byte-identical.
    /// Types/typed-locals/lines are the deferred PR-P2/P3 (this PR is name-level).
    pub analysis_pdb: bool,
    /// (kuna) Gate the Mach-O arm64e Apple-Silicon SLEIGH-spec selection
    /// (`macho-arm64e`); default **off** (design §3.7, opt-in until proven). When
    /// on, an arm64e Mach-O (`cpusubtype` CPU_SUBTYPE_ARM64E) loads with the
    /// `AARCH64:LE:64:AppleSilicon` pointer-auth spec instead of the generic
    /// `v8A`; pointer-auth does NOT change import naming or symbols, only the
    /// spec. NB: spec selection happens at *load* (`language_id_for`), before any
    /// console `option` command runs, so the actual gate is read live from the
    /// `KUNA_MACHO_ARM64E` env var the CLI exports for `--option macho-arm64e on`;
    /// this field exists for catalog/registration consistency (a recognized
    /// option name) and records the requested state. Default-off ⇒ every parity
    /// gate is byte-identical and a non-arm64e / non-Mach-O target is untouched.
    pub macho_arm64e: bool,

    // --- Owned subsystems (architecture.hh:211-233) -----------------------
    /// Memory map of global variables and functions (C++ `symboltab`).
    pub symboltab: Database,
    /// Options that can be configured (C++ `options`).
    pub options: OptionDatabase,
    /// Actions that can be applied in this architecture (C++ `allacts`).
    pub allacts: ActionDatabase,
    /// (kuna) Per-program restart-trigger side table (C++ file-static
    /// `restartTable`, owned here per `docs/RUST_PORT.md` — one log per loaded
    /// program; survives `Funcdata::clear()` because it lives outside the
    /// Funcdata).  The `restarts` console command renders it.
    pub restart_log: crate::kuna_restartlog::RestartLog,
    /// Specifically registered user-defined p-code ops (C++ `userops`).
    pub userops: UserOpManage,
    /// Manager of decoded strings (C++ `stringManager`, a `StringManager*`).
    /// `sleigh_arch.cc:250` seeds this with a `StringManagerUnicode(this,2048)`.
    /// Held behind `Rc<RefCell<..>>` so the same instance can be *shared* into
    /// the per-function W4 [`ArchSeam`] (`glb`): `Funcdata::getInternalString`
    /// (driven through the seam during `RuleStringStore`/`RuleStringCopy`) must
    /// `registerInternalStringData` into the very map the printer later reads back
    /// via `getStringData` on this real `Architecture`.
    pub string_manager:
        Rc<std::cell::RefCell<crate::stringmanage::StringManagerUnicode>>,
    /// P-code injection manager (C++ `pcodeinjectlib`).  SLEIGH-backed.
    pub pcodeinjectlib: PcodeInjectLibrarySleigh,
    /// Comments for this architecture (C++ `commentdb`).  // SEAM(comment.cc)
    pub commentdb: CommentDatabase,

    // --- W6/W8 subsystems wired by `init` (architecture.hh:211-233) -------
    /// Data-type factory (C++ `types`, a `TypeFactory*`).  Empty until
    /// [`build_typegrp`](Architecture::build_typegrp) + `build_core_types`.
    ///
    /// Held as an [`Rc`] so the analysis-side seam [`ArchSeam`](crate::seams::Architecture)
    /// (`glb`) can share the *same* populated factory: `ActionInferTypes` reaches
    /// `getBase`/`getTypePointer` through `glb.types()` and must see the identical
    /// interned core types this side cached.  Interior mutability (`Cell`/`RefCell`)
    /// keeps the `&self` setters (`setup_sizes`, `set_core_type`, …) working.
    types: Rc<TypeFactoryImpl>,
    /// The c-language printer (C++ `print`, the active `PrintLanguage*`).
    print: PrintC,
    /// Registered prototype models (C++ `protoModels`, name -> `ProtoModel*`).
    /// A `BTreeMap` (ADR 0002) for deterministic iteration matching the C++
    /// `map<string,ProtoModel*>` ordered traversal in `parseCompilerConfig`.
    proto_models: std::collections::BTreeMap<String, Rc<ProtoModel>>,
    /// The default prototype model (C++ `defaultfp`).  `None` until a cspec is
    /// parsed (or a default is seeded by [`build_default_proto`]).
    defaultfp: Option<Rc<ProtoModel>>,
    /// The current-evaluation prototype model (C++ `evalfp_current`); falls
    /// back to `defaultfp` when unset.
    evalfp_current: Option<Rc<ProtoModel>>,
    /// Default storage location of a function's return address (C++
    /// `Architecture::defaultReturnAddr`), decoded from the cspec's top-level
    /// `<returnaddress>` element by [`build_default_proto`].  `None` when the
    /// cspec has no `<returnaddress>` (then `testForReturnAddress` returns
    /// `false`, exactly as the C++ does for `defaultReturnAddr.space == 0`).
    default_return_addr: Option<kuna_num::pcoderaw::VarnodeData>,
    /// Raw compiler-spec (`.cspec`) XML content, set by the frontend before
    /// [`init_post_engine`](Architecture::init_post_engine).  The C++
    /// `parseCompilerConfig` decodes the `<default_proto>`/`<prototype>` tags
    /// from this; here [`build_default_proto`](Architecture::build_default_proto)
    /// reads it to recover the real input/output parameter lists.  `None` when
    /// the frontend did not supply it (then a name-only `unknown` default is
    /// seeded, as before).
    cspec_xml: Option<Vec<u8>>,
    /// Raw processor-spec (`.pspec`) XML content, set by the frontend before
    /// [`init_post_engine`](Architecture::init_post_engine).  The C++
    /// `parseProcessorConfig` (architecture.cc:1176) decodes the
    /// `<processor_spec>` children from this; here
    /// [`parse_processor_config`](Architecture::parse_processor_config) reads it
    /// to apply the `<context_data>` `<context_set>` paints that steer
    /// disassembly mode (e.g. x86-64's `addrsize`/`opsize`/`longMode`).  Without
    /// this the engine's context database is all-zero and x86 lifts as 16-bit
    /// real mode.  `None` when the frontend did not supply it (then the engine
    /// keeps the `.sla`-default zero context).
    pspec_xml: Option<Vec<u8>>,
    /// Vector registers that have preferred lane sizes (C++
    /// `Architecture::lanerecords`), built by [`decode_register_data`] from the
    /// pspec `<register_data>` `vector_lane_sizes` attributes during
    /// [`parse_processor_config`].  Sorted ascending by whole size (one record
    /// per size), so the binary-search lookups (`get_laned_register` /
    /// `get_minimum_laned_register_size`) match the C++.  Empty until the pspec
    /// is parsed (and for non-vector architectures).
    ///
    /// [`decode_register_data`]: Architecture::decode_register_data
    /// [`parse_processor_config`]: Architecture::parse_processor_config
    lanerecords: Vec<crate::transform::LanedRegister>,
    /// The p-code OpBehavior / `TypeOp` property table (C++ `inst`, the
    /// `vector<TypeOp *>` `TypeOp::registerInstructions` fills).  Indexed by
    /// op-code; `None` for the unused slots.  Empty until `build_instructions`.
    inst: Vec<Option<crate::typeop::TypeOpInfo>>,
    /// The p-code OpBehavior emulation table (C++ `TypeOp::behave`, the
    /// `OpBehavior *` `TypeOp::registerInstructions` attaches to each `TypeOp`).
    ///
    /// In the Rust port the metadata (`inst`, above) and the emulation behavior
    /// are split tables — the C++ `TypeOp` carries both; here the behavior table
    /// is built alongside `inst` by [`build_instructions`](Architecture::build_instructions)
    /// from `kuna_num::opbehavior::register_instructions`.  Drives the
    /// constant-folding `op->collapse()` (`RuleCollapseConstants`).  Indexed by
    /// op-code; empty until `build_instructions`.
    opbehaviors: Vec<Option<Rc<dyn kuna_num::opbehavior::OpBehavior>>>,

    /// The disassembly engine for this binary (C++ `translate`, a `Translate*`).
    ///
    /// Owned here as a concrete [`Sleigh`] (the C++ `Architecture` is-a
    /// `AddrSpaceManager` and owns its `Translate`; in the Rust port the
    /// `AddrSpaceManager` lives inside `Sleigh`'s `SleighBase`, reachable via
    /// `base().manager()`).  The non-SLEIGH `Translate` backends (`raw_arch`) are
    /// their own item; until then the engine is concrete so `manage()` works.
    translate: Sleigh,
}

impl Architecture {
    /// Construct an `Architecture` over an already-initialized disassembly
    /// engine (C++ `Architecture::Architecture` + the `restoreFromSpec` subsystem
    /// builds, condensed: the C++ ctor leaves the heavy subsystems null and
    /// `init`/`restoreFromSpec` fill them; this port takes a built `Translate`
    /// and constructs the subsystems whose deps exist).
    ///
    /// The `translate` must already be initialized (a `Sleigh` with a decoded
    /// `.sla`); the architecture borrows its `AddrSpaceManager` and the
    /// `getUniqueStart(INJECT)` tempbase for the injection library.
    pub fn new(archid: &str, translate: Sleigh) -> Architecture {
        // C++ PcodeInjectLibrarySleigh(g): tempbase = g->translate->getUniqueStart(INJECT).
        let inject_tempbase = translate.get_unique_start(UniqueLayout::INJECT);

        // C++ buildDatabase(store): symboltab = new Database(this,true);
        //   Scope *globscope = new ScopeInternal(0,"",this);
        //   symboltab->attachScope(globscope,(Scope*)0);
        // ScopeInternal sizes its per-space maps to numSpaces(); count before the
        // translate is moved into the struct (manage() borrows it).
        let space_count = translate.base().manager().num_spaces();
        let mut symboltab = Database::new(true);
        symboltab
            .find_create_scope(0, "", None, space_count)
            .expect("buildDatabase: attach global scope");

        let mut arch = Architecture {
            archid: archid.to_string(),

            trim_recurse_max: 0,
            max_implied_ref: 0,
            max_term_duplication: 0,
            max_basetype_size: 0,
            min_funcsymbol_size: 1,
            max_jumptable_size: 0,
            aggressive_ext_trim: false,
            readonlypropagate: false,
            infer_pointers: false,
            funcptr_align: 0,
            flowoptions: 0,
            max_instructions: 0,
            alias_block_level: 0,
            split_datatype_config: 0,
            analyze_for_loops: false,
            nan_ignore_all: false,
            nan_ignore_compare: false,
            loadersymbols_parsed: false,
            infer_ptr_spaces: Vec::new(),

            infer_funcentry: false,
            return_single: false,
            memset_recover: false,
            add_carry_chain: false,
            v850_indirect_branch: false,
            tail_call_jumps: false,
            noreturn_extern_calls: false, // (kuna) option noreturn_extern, default off
            sparc_struct_return: false,
            ov_less_simplify: false,
            fold_boolean_mask: false,
            input_varnode_adjust: false,
            condexe_block_placement: false,
            dynamic_hash_maxdup_high: false,
            model_stack_probe_loop: false,
            fold_flag_compare: false,
            switch_modulo_bound: false,
            switch_guard_bound: false,
            switch_shared_case: false,
            switch_multi_pred: false,
            unrolled_guard: false,
            noreturn_extern_match: true, // (kuna) DIV-13 default-on (angr incorrect-duplication-chcon)
            stack_alias_deadstore: false,
            recover_array_stride: false,
            recover_lowered_switch: false,
            region_structure: true,
            region_loop_refine: false,
            region_edge_order: false,
            reduce_return_gotos: false,
            flatten_ifelse: false,
            revert_cross_jumps: false,
            dup_return_call_tails: false,
            dedup_ite_tail: false,
            iteregion: false,
            duplicate_shared_returns: false,
            early_return: false,
            recover_loop_break: false,
            fold_call_returns: false,
            strip_stack_guard: false,
            branch_flip: false,
            name_style_angr: false,
            dedup_var_decls: false,
            realtypes: false,
            present_lessequal: false,
            preserve_thumb_funcptr: false,
            kuna_fn_budget: None,   // (kuna) decompile-all watchdog: no budget by default
            kuna_fn_deadline: None, // (kuna) set per drive from kuna_fn_budget

            // Analysis-pass gates: real defaults set by reset_defaults_internal.
            analysis_noreturn_known: false,
            analysis_libproto: false,
            analysis_strings: false,
            analysis_entry_disc: false,
            analysis_eh_frame_full: false,
            analysis_funcstart_patterns: false,
            analysis_arm_markers: false,
            analysis_mips_gp: false,
            analysis_i386_pie_plt: false,
            analysis_mips_isa: false,
            analysis_dwarf: false,
            analysis_dwarf_lines: false,
            analysis_callfixup: false,
            analysis_addrtable: false,
            analysis_operand_refs: false,
            analysis_formatstring: false,
            analysis_listing: false,
            analysis_noreturn_disc: true, // (kuna) DIV-22 default-on (Listing-gated); see set_analysis_defaults
            analysis_noreturn_propagate: false,
            analysis_noreturn_error: false,
            analysis_noreturn_reach: false,
            error_noreturn_callsites: Vec::new(),
            analysis_fid: false,
            analysis_rtti: false,
            analysis_aif: false,
            analysis_gopclntab: false,
            analysis_objc: false,
            analysis_pdb: false,
            macho_arm64e: false,

            symboltab,
            options: OptionDatabase::new(),
            allacts: ActionDatabase::new(),
            restart_log: crate::kuna_restartlog::RestartLog::new(),
            userops: UserOpManage::new(),
            // sleigh_arch.cc:250: stringManager = new StringManagerUnicode(this,2048)
            string_manager: Rc::new(std::cell::RefCell::new(
                crate::stringmanage::StringManagerUnicode::new(2048),
            )),
            pcodeinjectlib: PcodeInjectLibrarySleigh::new(inject_tempbase),
            commentdb: CommentDatabase::new(),
            // C++ ctor leaves types/print/defaultfp null; init() fills them.
            types: Rc::new(TypeFactoryImpl::new()),
            print: PrintC::new(),
            proto_models: std::collections::BTreeMap::new(),
            defaultfp: None,
            evalfp_current: None,
            default_return_addr: None,
            cspec_xml: None,
            pspec_xml: None,
            lanerecords: Vec::new(),
            inst: Vec::new(),
            opbehaviors: Vec::new(),
            translate,
        };
        // C++ ctor calls resetDefaultsInternal(); then sets min_funcsymbol_size=1
        // etc. (those one-offs are folded into resetDefaultsInternal's siblings
        // in the ctor; we set the ctor-only members and then run the reset).
        arch.reset_defaults_internal();
        arch.min_funcsymbol_size = 1; // C++ ctor: min_funcsymbol_size = 1
        arch.aggressive_ext_trim = false; // C++ ctor: aggressive_ext_trim = false
        arch.funcptr_align = 0; // C++ ctor: funcptr_align = 0
        arch
    }

    /// Reset default values for the options owned by `Architecture` (verbatim
    /// transcription of C++ `Architecture::resetDefaultsInternal`,
    /// `architecture.cc:1420`).  The kuna defaults follow DIV-2/DIV-3
    /// (`docs/divergences.md`).
    pub fn reset_defaults_internal(&mut self) {
        self.trim_recurse_max = 5;
        self.max_implied_ref = 2; // 2 is best, in specific cases a higher number might be good
        self.max_term_duplication = 2; // 2 and 3 (4) are reasonable
        self.max_basetype_size = 10; // Needs to be 8 or bigger
        self.flowoptions = flow_flags::error_toomanyinstructions;
        self.max_instructions = 100000;
        self.infer_pointers = true;
        self.infer_funcentry = true; // (kuna) DIV-2 default-on (GH-6930)
        self.return_single = false; // (kuna) default: upstream (join register pairs)
        self.memset_recover = true; // (kuna) DIV-2 default-on (GH-9230/1537)
        self.v850_indirect_branch = false; // (kuna) default: upstream (GH-8817)
        self.tail_call_jumps = true; // (kuna) DIV-13 default-on (angr tail-call recovery; per-test opt-out on Long double #1/#2)
        self.noreturn_extern_calls = true; // (kuna) DIV-14 default-on: REMOVES CODE (drops the post-call fall-through after a matched extern no-return). Byte-identical (0/675) — no datatest call resolves to a known no-return name; overlaps `noreturn_known`'s name match for defined/imported symbols, restore upstream with `option noreturn_extern off`
        self.sparc_struct_return = false; // (kuna) default: upstream byte-identical (GH-6882)
        self.ov_less_simplify = true; // (kuna) DIV-2 default-on (GH-7190)
        self.fold_boolean_mask = true; // (kuna) DIV-2 default-on (GH-1282)
        self.input_varnode_adjust = true; // (kuna) DIV-3 default-on (GH-9218)
        self.dynamic_hash_maxdup_high = true; // (kuna) DIV-3 default-on (GH-8467)
        self.fold_flag_compare = true; // (kuna) DIV-3 default-on (GH-1276/8777)
        self.switch_modulo_bound = false; // (kuna) default: upstream byte-identical (GH-9191)
        self.switch_guard_bound = false; // (kuna) default: upstream byte-identical (angr opt-in)
        self.switch_shared_case = true; // (kuna) DIV-14 default-on (angr loop-carried-guard PIC switch recovery; slower on the functions it recovers, kept on for quality; 0/675 byte-identical)
        self.switch_multi_pred = true; // (kuna) DIV-13 default-on (angr multi-predecessor unrolled-guard jump-table; 0/675 ablation)
        self.unrolled_guard = false; // (kuna) default: upstream byte-identical (angr opt-in)
        self.noreturn_extern_match = true; // (kuna) DIV-13 default-on (angr incorrect-duplication-chcon; clean 0/675 ablation)
        self.stack_alias_deadstore = false; // (kuna) default: upstream byte-identical (GH-8500)
        self.recover_array_stride = true; // (kuna) DIV-3 default-on (GH-8724)
        self.recover_lowered_switch = true; // (kuna) default-on (angr port)
        self.region_structure = true; // (kuna) DIV-12 default-on (region-based Phoenix/SAILR structurer; primary structuring path, falls back to CollapseStructure on irreducible code)
        self.region_loop_refine = true; // (kuna) DIV-13 default-on (region structurer multi-exit/irreducible loop-successor refinement; 0/675 ablation)
        self.region_edge_order = false; // (kuna) SAILR P2 default-OFF opt-in (H2 post-dominator + dominance-tiered edge-virtualization ordering; only reorders which goto is chosen when virtualizing, so OFF is byte-identical)
        self.reduce_return_gotos = true; // (kuna) DIV-13 default-on (angr SAILR goto-reduction; 0/675 ablation)
        self.flatten_ifelse = true; // (kuna) DIV-13 default-on (angr IfElseFlattener; 0/675 ablation)
        self.revert_cross_jumps = true; // (kuna) DIV-13 default-on (angr SAILR CrossJumpReverter; 0/675 ablation)
        self.dup_return_call_tails = true; // (kuna) DIV-13 default-on (angr SAILR ReturnDuplicatorLow return-call-tail dup; 0/675 ablation)
        self.dedup_ite_tail = true; // (kuna) DIV-13 default-on (angr structurer ITE region-dedup — merge duplicated if/else tails; 0/675 ablation)
        self.iteregion = true; // (kuna) DIV-17 default-on (angr ITERegionConverter: assignment-diamond -> `?:` ternary, decbench F5). Per-test opt-out (`option iteregion off`) on the datatests it changes keeps the corpus byte-identical.
        self.duplicate_shared_returns = false; // (kuna) default: upstream byte-identical (angr SAILR gotoless ReturnDuplicatorHigh, decbench F4 returndup — opt-in). DIV-18 flipped this default-on, but the decbench re-run showed the aggregate GED perfect count REGRESSED ~976 (returndup fired 21768x, and early-return recovery diverges from the source's merged short-circuit form on the majority), so it is reverted to a per-function runtime choice (`--option returndup on`).
        self.recover_loop_break = true; // (kuna) DIV-10 default-on (angr break/continue recovery; scopeBreak port)
        self.fold_call_returns = true; // (kuna) DIV-13 default-on (angr call-return folding; per-test opt-out on the datatests it changes)
        self.strip_stack_guard = true; // (kuna) DIV-14 default-on: REMOVES CODE (strips the -fstack-protector canary epilogue). Per-test opt-out (`option stackguard off`) on the 2 Partial-splitting datatests keeps the corpus byte-identical
        self.branch_flip = true; // (kuna) DIV-13 default-on (angr negated-guard branch flipping; per-test opt-out on the datatests it changes)
        self.name_style_angr = true; // (kuna) default-on: angr-style default naming
        self.dedup_var_decls = true; // (kuna) DIV-7 default-on: collapse duplicate local decls (angr)
        self.realtypes = true; // (kuna) DIV-6 default-on: real C types for unknowns
        self.condexe_block_placement = true; // (kuna) DIV-3 default-on (GH-9203)
        self.add_carry_chain = true; // (kuna) DIV-2 default-on (GH-8913)
        self.model_stack_probe_loop = true; // (kuna) DIV-3 default-on (GH-8017)
        self.analyze_for_loops = true;
        self.present_lessequal = true; // (kuna) DIV-2 default-on (GH-558)
        self.preserve_thumb_funcptr = true; // (kuna) DIV-2 default-on (GH-8471)
        self.readonlypropagate = false;
        self.nan_ignore_all = false;
        self.nan_ignore_compare = true; // Ignore NaN ops associated with FP comparisons by default
        self.alias_block_level = 2; // Block structs and arrays by default
        self.split_datatype_config =
            split_datatype::OPTION_STRUCT | split_datatype::OPTION_ARRAY | split_datatype::OPTION_POINTER;
        self.max_jumptable_size = 1024;

        // (kuna) Analysis-pass gates — default-on (matching Ghidra's default-on
        // analyzers), except addrtable which Ghidra ships off. Bound to the
        // real-ELF analysis tier; inert on the XML datatest path.
        self.analysis_noreturn_known = true;
        self.analysis_libproto = true;
        self.analysis_strings = true;
        self.analysis_entry_disc = true;
        // (kuna) `.eh_frame` LSDA landing-pad discovery — default-OFF (opt-in,
        // output-changing: adds the discovered exception landing pads as entries).
        self.analysis_eh_frame_full = false;
        self.analysis_funcstart_patterns = false; // full byte-pattern starts default-off (output-changing)
        self.analysis_arm_markers = true;
        self.analysis_mips_gp = true;
        self.analysis_i386_pie_plt = true; // (kuna) i386-PIE PLT decode default-on (angr)
        self.analysis_mips_isa = true;
        self.analysis_dwarf = true;
        self.analysis_dwarf_lines = false; // (kuna) source-line comments default-OFF (output-changing, opt-in)
        self.analysis_callfixup = true;
        self.analysis_addrtable = false; // Ghidra AddressTableAnalyzer default-off
        self.analysis_operand_refs = false; // Ghidra ScalarOperandAnalyzer !isElf default-off
        self.analysis_formatstring = false; // Ghidra FormatStringAnalyzer default-off
        self.analysis_listing = false; // Listing/xref tier default-off
        self.analysis_noreturn_disc = true; // (kuna) DIV-22 default-on: Ghidra's FindNoReturnFunctionsAnalyzer ≥3-evidence discovered-no-return (default-on in Ghidra). REMOVES CODE (marks a callee no-return from ≥3 dead-fall-through sites → drops post-call dead code at callers). Gated on the Listing (default-off), so every parity gate is byte-identical (real-ELF Listing path only); restore with `option noreturn_disc off`
        self.analysis_noreturn_propagate = true; // (kuna) DIV-14 default-on: REMOVES CODE (call-graph no-return propagation drops post-call dead code). Gated on the Listing (default-off), so every parity gate is byte-identical (real-ELF Listing path only); restore with `option noreturn_propagate off`
        self.analysis_noreturn_error = true; // (kuna) DIV-16 default-on: REMOVES CODE (conclude error(nonzero,...) wrappers no-return, dropping the dead fall-through at every caller). Sub-rule of noreturn_propagate, gated on the Listing (default-off), so every parity gate is byte-identical (real-ELF Listing path only); restore with `option noreturn_error off`
        self.analysis_noreturn_reach = true; // (kuna) DIV-19 default-on: REMOVES CODE (CFG-reachability no-return, Ghidra's FindNoReturnFunctionsAnalyzer.targetOnlyCallsNoReturn — mid-body no-return calls, dead returns, switch-of-no-return). Sub-rule of noreturn_propagate, gated on the Listing (default-off), so every parity gate is byte-identical (real-ELF Listing path only); restore with `option noreturn_reach off`
        self.analysis_fid = false; // FID fingerprint matcher consumer default-off
        self.analysis_rtti = false; // MSVC RTTI / vftable recovery default-off (PE-only, output-changing)
        self.analysis_aif = false; // Aggressive Instruction Finder gap-walk default-off
        self.analysis_gopclntab = true; // Go pclntab name recovery default-on (Go-only pass)
        self.analysis_objc = false; // Mach-O Objective-C metadata recovery default-off (Mach-O-only pass)
        self.analysis_pdb = false; // PE PDB metadata recovery default-off (PE-only, external-.pdb-gated pass)
        self.macho_arm64e = false; // arm64e Apple-Silicon spec selection default-off (opt-in)
    }

    /// Apply a kuna stage-model option (`option <name> <value>`), the analogue of
    /// an upstream `ArchOption::apply` for the 23 kuna-owned knobs in
    /// [`KUNA_OPTION_NAMES`](crate::options::KUNA_OPTION_NAMES).
    ///
    /// Unlike the upstream options (dispatched through `OptionDatabase` keyed by a
    /// registered `ElementId`), the kuna options write configuration flags that
    /// live directly on this `Architecture` (or, for `arraynotation`, on the
    /// owned [`PrintC`]).  Each arm reuses the per-option parse helper that owns
    /// the value validation + confirmation text (`parse_compare_form`,
    /// `parse_return_pair_form`, `parse_memset_recover_form`,
    /// `parse_stack_probe_loop_form`, the `OptionNameStyle`/`OptionArrayNotation`/
    /// `OptionLowerSwitch::apply` bodies) or the shared
    /// [`on_or_off`](crate::options::on_or_off) toggle parser, then writes the
    /// resolved value into the live flag the consuming action/printer reads.
    ///
    /// The console (`IfcOption`) and the `kassert` dispatcher route a name in
    /// `KUNA_OPTION_NAMES` here; an unknown name is the caller's bug (it is gated
    /// by the allowlist) and surfaces as a parse error.
    pub fn set_kuna_option(&mut self, name: &str, p1: &str) -> KunaResult<String> {
        use crate::options::on_or_off;
        // Shared on/off arm: parse the toggle, write the field, format the message.
        macro_rules! on_off {
            ($field:ident, $label:literal) => {{
                let val = on_or_off(p1)?;
                self.$field = val;
                Ok(format!(
                    concat!($label, " turned {}"),
                    if val { "on" } else { "off" }
                ))
            }};
        }
        match name {
            "compareform" => {
                let (form, msg) = crate::kuna_compareform::parse_compare_form(p1)?;
                self.present_lessequal = form.present_lessequal();
                Ok(msg)
            }
            "arraynotation" => {
                let (val, msg) = crate::kuna_arraynotation::OptionArrayNotation.apply(p1)?;
                self.print_mut().options.set_array_notation(val);
                Ok(msg)
            }
            "thumbfuncptr" => on_off!(preserve_thumb_funcptr, "Thumb function-pointer preservation"),
            "inferfuncentry" => on_off!(infer_funcentry, "Function-entry constant inference"),
            "returnpair" => {
                let (form, msg) = crate::kuna_returnpair::parse_return_pair_form(p1)?;
                self.return_single = form.return_single();
                Ok(msg)
            }
            "addcarrychain" => on_off!(add_carry_chain, "Carry-chain wide-add recovery"),
            "ovlesssimplify" => on_off!(ov_less_simplify, "OV-flag signed-compare simplification"),
            "booleanmask" => on_off!(fold_boolean_mask, "Boolean sign-mask folding"),
            "flagcompare" => on_off!(fold_flag_compare, "Flag-modelled comparison folding"),
            "v850indirectbranch" => on_off!(v850_indirect_branch, "V850 indirect-branch reclassification"),
            "tailcalljump" => on_off!(tail_call_jumps, "Tail-call jump recovery"),
            "noreturn_extern" => on_off!(noreturn_extern_calls, "Name-based extern no-return"),
            "inputvarnodeadjust" => on_off!(input_varnode_adjust, "Overlapping input-varnode adjustment"),
            "condexeplace" => on_off!(condexe_block_placement, "Conditional-const COPY block placement"),
            "sparcstructret" => on_off!(sparc_struct_return, "SPARC struct-return tail recovery"),
            "arraystride" => on_off!(recover_array_stride, "Strided-induction array recovery"),
            "stackalias" => on_off!(stack_alias_deadstore, "Stack-pointer-alias dead-store hold"),
            "dynamichashmax" => on_off!(dynamic_hash_maxdup_high, "DynamicHash collision budget"),
            "stackprobeloop" => {
                let (form, msg) = crate::kuna_stackprobeloop::parse_stack_probe_loop_form(p1)?;
                self.model_stack_probe_loop = form.model_stack_probe_loop();
                Ok(msg)
            }
            "memsetrecover" => {
                let (form, msg) = crate::kuna_memsetsequence::parse_memset_recover_form(p1)?;
                self.memset_recover = form.memset_recover();
                Ok(msg)
            }
            "switchmodbound" => on_off!(switch_modulo_bound, "Switch modulo/and-mask index bound"),
            "switchguardbound" => on_off!(switch_guard_bound, "Switch CBRANCH-guard index bound"),
            "switchsharedcase" => on_off!(switch_shared_case, "Switch loop-carried-guard table"),
            "switchmultipred" => on_off!(switch_multi_pred, "Switch multi-predecessor unrolled-guard table"),
            "unrolledguard" => on_off!(unrolled_guard, "Interleaved unrolled-guard jump-table partial-flow recovery"),
            "noreturn_externmatch" => on_off!(noreturn_extern_match, "Name-matched extern no-return"),
            "loweredswitch" => {
                let (val, msg) = crate::kuna_loweredswitch::OptionLowerSwitch.apply(p1)?;
                self.recover_lowered_switch = val;
                Ok(msg)
            }
            "regionstructure" => {
                let (val, msg) =
                    crate::s8_structure::region_structurer::OptionRegionStructure.apply(p1)?;
                self.region_structure = val;
                Ok(msg)
            }
            "regionlooprefine" => on_off!(
                region_loop_refine,
                "Region structurer multi-exit/irreducible loop-successor refinement"
            ),
            "regionedgeorder" => on_off!(
                region_edge_order,
                "Region structurer H2 post-dominator + dominance-tiered edge-virtualization ordering"
            ),
            "gotoreduce" => {
                let (val, msg) =
                    crate::s8_structure::kuna_gotoreduce::OptionGotoReduce.apply(p1)?;
                self.reduce_return_gotos = val;
                Ok(msg)
            }
            "ifelseflatten" => {
                let (val, msg) =
                    crate::s8_structure::kuna_ifelseflatten::OptionIfElseFlatten.apply(p1)?;
                self.flatten_ifelse = val;
                Ok(msg)
            }
            "crossjumprevert" => {
                let (val, msg) =
                    crate::s8_structure::kuna_crossjumpreverter::OptionCrossJumpReverter.apply(p1)?;
                self.revert_cross_jumps = val;
                Ok(msg)
            }
            "taildup" => {
                let (val, msg) = crate::s8_structure::kuna_taildup::OptionTailDup.apply(p1)?;
                self.dup_return_call_tails = val;
                Ok(msg)
            }
            "dedupitetail" => {
                let (val, msg) =
                    crate::s8_structure::kuna_dedupitetail::OptionDedupIteTail.apply(p1)?;
                self.dedup_ite_tail = val;
                Ok(msg)
            }
            "iteregion" => {
                let (val, msg) = crate::s8_structure::kuna_iteregion::OptionIteRegion.apply(p1)?;
                self.iteregion = val;
                Ok(msg)
            }
            "returndup" => {
                let (val, msg) =
                    crate::s8_structure::kuna_returndup::OptionReturnDup.apply(p1)?;
                self.duplicate_shared_returns = val;
                Ok(msg)
            }
            "earlyreturn" => {
                let (val, msg) =
                    crate::s8_structure::kuna_earlyreturn::OptionEarlyReturn.apply(p1)?;
                self.early_return = val;
                Ok(msg)
            }
            "foldcallret" => {
                let (val, msg) = crate::kuna_callretfold::OptionFoldCallRet.apply(p1)?;
                self.fold_call_returns = val;
                Ok(msg)
            }
            "stackguard" => on_off!(strip_stack_guard, "Stack-guard canary stripping"),
            "branchflip" => on_off!(branch_flip, "Negated-guard branch flipping for linearity"),
            "loopbreak_recovery" => {
                let (val, msg) =
                    crate::kuna_loopbreak_recovery::OptionLoopBreakRecovery.apply(p1)?;
                self.recover_loop_break = val;
                Ok(msg)
            }
            "namestyle" => {
                let (val, msg) = crate::kuna_naming::OptionNameStyle.apply(p1)?;
                self.name_style_angr = val;
                Ok(msg)
            }
            "realtypes" => on_off!(realtypes, "Real-C-type rendering for unknowns"),
            "dedupvardecls" => {
                let (val, msg) = crate::kuna_dedupvardecls::OptionDedupVarDecls.apply(p1)?;
                self.dedup_var_decls = val;
                Ok(msg)
            }
            // (kuna) Analysis-pass gates: one boolean per `kuna_analysis::passes`
            // pass id. The console's `commit_analysis_output` (run at `read
            // symbols`, after the options below have been applied) consults the
            // matching flag and skips a disabled pass's facts. The option id IS
            // the pass's `AnalysisPass::id()` string. Real-ELF path only.
            "noreturn_known" => on_off!(analysis_noreturn_known, "No-return-known analysis pass"),
            "libproto" => on_off!(analysis_libproto, "Library-prototype analysis pass"),
            "strings" => on_off!(analysis_strings, "String-literal analysis pass"),
            "entry_disc" => on_off!(analysis_entry_disc, "Entry-discovery analysis pass"),
            "eh_frame_full" => {
                on_off!(analysis_eh_frame_full, ".eh_frame LSDA landing-pad discovery")
            }
            "funcstart_patterns" => {
                on_off!(analysis_funcstart_patterns, "Full byte-pattern function-start pass")
            }
            "arm_markers" => on_off!(analysis_arm_markers, "ARM/Thumb decode-mode marker pass"),
            "mips_gp" => on_off!(analysis_mips_gp, "MIPS $gp-recovery (t9 tracking) pass"),
            // (kuna) Loader-tier gate: also bridge to the env var the loader reads
            // (the PLT map is baked at `load file`, upstream of this `option`), so
            // an `option i386_pie_plt off` *before* `load file` in the same process
            // takes effect. The CLI sets the env directly on the subprocess too.
            "i386_pie_plt" => {
                let val = on_or_off(p1)?;
                self.analysis_i386_pie_plt = val;
                crate::kuna_i386_pie_plt::set_i386_pie_plt_env(val);
                Ok(format!(
                    "i386-PIE PLT-stub decode turned {}",
                    if val { "on" } else { "off" }
                ))
            }
            "mips_isa" => on_off!(analysis_mips_isa, "MIPS16 ISA_MODE decode-mode marker pass"),
            "dwarf" => on_off!(analysis_dwarf, "DWARF recovery analysis pass"),
            "dwarf_lines" => {
                on_off!(analysis_dwarf_lines, "DWARF .debug_line source-line comment pass")
            }
            "callfixup" => on_off!(analysis_callfixup, "Call-fixup analysis pass"),
            "addrtable" => on_off!(analysis_addrtable, "Address-table analysis pass"),
            "operand_refs" => on_off!(analysis_operand_refs, "Scalar/operand reference-markup pass"),
            "formatstring" => {
                on_off!(analysis_formatstring, "Format-string varargs-typing pass")
            }
            "listing" => on_off!(analysis_listing, "Listing/xref disassembly tier"),
            "noreturn_disc" => {
                on_off!(analysis_noreturn_disc, "Discovered-no-return Listing consumer")
            }
            "noreturn_propagate" => {
                on_off!(analysis_noreturn_propagate, "No-return propagation Listing consumer")
            }
            "noreturn_error" => {
                on_off!(analysis_noreturn_error, "error(nonzero,...) conditional no-return recognizer")
            }
            "noreturn_reach" => {
                on_off!(analysis_noreturn_reach, "CFG-reachability no-return rule (Ghidra targetOnlyCallsNoReturn)")
            }
            "fid" => on_off!(analysis_fid, "FID fingerprint matcher Listing consumer"),
            "rtti" => on_off!(analysis_rtti, "MSVC RTTI / vftable class-name recovery pass"),
            "aif" => {
                on_off!(analysis_aif, "Aggressive Instruction Finder gap-walk Listing consumer")
            }
            "gopclntab" => {
                on_off!(analysis_gopclntab, "Go pclntab function-name recovery pass")
            }
            "objc" => on_off!(analysis_objc, "Mach-O Objective-C metadata recovery pass"),
            "pdb" => on_off!(analysis_pdb, "PE PDB metadata recovery pass"),
            // (kuna) ET_REL relocatable-object (`.o`) loader capability. Unlike
            // every other kuna option this gates the *loader* (run at `load
            // file`, before any `option` command is processed), so a flag on this
            // `Architecture` would be read too late. The toggle is bridged across
            // the layer by a process env var the loader reads at `from_bytes`
            // time; flipping it here affects a subsequent `load file` of a `.o`.
            // See `kuna_analysis::loadimage_object::reloc_objects_enabled`.
            "relocobjects" => {
                let val = on_or_off(p1)?;
                std::env::set_var(
                    crate::options::RELOC_OBJECTS_ENV,
                    if val { "1" } else { "0" },
                );
                Ok(format!(
                    "ET_REL relocatable-object loading turned {}",
                    if val { "on" } else { "off" }
                ))
            }
            // (kuna §3.7) arm64e Apple-Silicon spec selection. Unlike the
            // analysis-pass gates this affects the *load-time* SLEIGH-spec choice
            // (`language_id_for`), which runs before this console `option` command;
            // the live gate is the `KUNA_MACHO_ARM64E` env var the CLI exports for
            // `--option macho-arm64e on`. This arm records the requested state on
            // the Architecture so the option is a recognized name (catalog
            // consistency) and a kassert can read it back. Default-off.
            "macho-arm64e" => on_off!(macho_arm64e, "Mach-O arm64e Apple-Silicon spec selection"),
            other => Err(KunaError::parse(format!("Unknown kuna option: {other}"))),
        }
    }

    /// Reset options modifiable by the OptionDatabase, including the action
    /// database (C++ `Architecture::resetDefaults`, `architecture.cc:1463`).
    ///
    /// SEAM(W5/W8): the C++ also calls `allacts.resetDefaults()` (the
    /// `ActionDatabase` default-group reset, a W5 surface not yet exposed) and
    /// resets every `PrintLanguage` in `printlist` (W8).  Only the internal
    /// option reset runs here; the action/print resets land with their waves.
    pub fn reset_defaults(&mut self) {
        self.reset_defaults_internal();
        // allacts.resetDefaults();                                 -- SEAM(W5)
        // for printlang in printlist: printlang.reset_defaults();  -- SEAM(W8)
    }

    // -----------------------------------------------------------------------
    // Address-space access (C++ Architecture is-a AddrSpaceManager)
    // -----------------------------------------------------------------------

    /// Borrow the address-space manager (C++ `this` viewed as an
    /// `AddrSpaceManager`); forwarded to the owned `Sleigh` engine's
    /// `SleighBase`.
    pub fn manage(&self) -> &AddrSpaceManager {
        self.translate.base().manager()
    }

    /// Borrow the disassembly engine (C++ `translate`).
    pub fn translate(&self) -> &Sleigh {
        &self.translate
    }

    /// Mutably borrow the disassembly engine.
    pub fn translate_mut(&mut self) -> &mut Sleigh {
        &mut self.translate
    }

    /// Get the minimum size of a laned register in bytes, or -1 if there are no
    /// laned registers (C++ `Architecture::getMinimumLanedRegisterSize`,
    /// `architecture.cc:313`).
    ///
    /// The `lanerecords` table is populated by [`decode_register_data`] from the
    /// pspec `<register_data>` `vector_lane_sizes` attributes (run during
    /// [`parse_processor_config`]).  When the table is empty (non-vector
    /// architecture, or pspec not yet parsed) this returns -1 exactly as the C++
    /// does with an empty table; the records are sorted ascending by whole size,
    /// so `lanerecords[0]` is the smallest.
    ///
    /// [`decode_register_data`]: Architecture::decode_register_data
    /// [`parse_processor_config`]: Architecture::parse_processor_config
    pub fn get_minimum_laned_register_size(&self) -> int4 {
        if self.lanerecords.is_empty() {
            return -1;
        }
        self.lanerecords[0].get_whole_size()
    }

    /// Look up the laned-register record for a storage location (C++
    /// `Architecture::getLanedRegister`, `architecture.cc:291`).
    ///
    /// As in the C++, the record is associated only with the *size* of the
    /// storage, not its address; `loc` is unused.  Faithful binary search over
    /// the size-sorted `lanerecords`.  `None` is the C++ `(const LanedRegister *)0`.
    pub fn get_laned_register(
        &self,
        _loc: &Address,
        size: int4,
    ) -> Option<&crate::transform::LanedRegister> {
        let mut min: int4 = 0;
        let mut max: int4 = self.lanerecords.len() as int4 - 1;
        while min <= max {
            let mid = (min + max) / 2;
            let sz = self.lanerecords[mid as usize].get_whole_size();
            if sz < size {
                min = mid + 1;
            } else if size < sz {
                max = mid - 1;
            } else {
                return Some(&self.lanerecords[mid as usize]);
            }
        }
        None
    }

    /// Get a string describing this architecture (C++ `getDescription`).
    pub fn get_description(&self) -> &str {
        &self.archid
    }

    // -----------------------------------------------------------------------
    // nameFunction (architecture.cc:539)
    // -----------------------------------------------------------------------

    /// Pick a default name for a function at `addr` (C++
    /// `Architecture::nameFunction`, `architecture.cc:539`).
    ///
    /// When the kuna angr-style naming is active (`name_style_angr`), the name is
    /// `sub_<addr>` (C++ `kunaFunctionName`, transcribed in [`Database`]); the
    /// upstream policy is `func_<raw-addr>`.
    pub fn name_function(&self, addr: &Address) -> String {
        if self.name_style_angr {
            // (kuna) angr-style: sub_<addr>
            return crate::database::kuna_function_name(addr);
        }
        // C++: ostringstream defname; defname << "func_"; addr.printRaw(defname);
        // kuna-base `Address::print_raw` is the faithful transcription of
        // `Address::printRaw` -> `AddrSpace::printRaw` (zero-padded `0x<offset>`,
        // word-size division, no space-name prefix).  A function address is a
        // processor space, where `printRaw` cannot fail (the only erroring spaces
        // are fspec/iop, which never hold a function), matching the C++ `void`
        // nameFunction that has no throw site here.
        let mut s = String::from("func_");
        addr.print_raw(&mut s)
            .expect("Architecture::nameFunction: Address::printRaw on a processor address (C++ cannot fail here)");
        s
    }

    /// C++ `symboltab->getGlobalScope()->queryFunction(name)` reduced to the
    /// FunctionSymbol handle: resolve the function symbol by name in the global
    /// scope, erroring `RecovError("Unknown function name: "+name)` when no
    /// FunctionSymbol of that name exists (the C++ `OptionInline`/`OptionNoReturn`
    /// contract).  Used by the per-function property setters; the loader symbols
    /// are read into the global scope at load (`read_loader_symbols`).
    pub fn query_global_function(&self, name: &str) -> KunaResult<crate::database::SymbolId> {
        let scope = self
            .symboltab
            .get_global_scope()
            .ok_or_else(|| KunaError::recov(format!("Unknown function name: {name}")))?;
        self.symboltab
            .query_function_by_name(scope, name)
            .ok_or_else(|| KunaError::recov(format!("Unknown function name: {name}")))
    }

    /// Park a source-declared prototype on the named global FunctionSymbol (C++
    /// `Architecture::setPrototype`: `queryFunction(name)->getFuncProto()` is locked
    /// from the parsed declaration).  A caller's `ActionDefaultParams::apply` later
    /// `fc->copy(otherfunc->getFuncProto())` (`coreaction.cc:2385`) reads it back via
    /// [`Database::function_proto_pieces`].  Silently no-ops when no FunctionSymbol of
    /// that name exists (the kuna console re-applies the queried function's own
    /// prototype through `apply_locked_prototype`; this path is for the *callees*).
    pub fn set_function_prototype_pieces(
        &mut self,
        name: &str,
        pieces: crate::fspec::PrototypePieces,
    ) {
        if let Ok(sid) = self.query_global_function(name) {
            self.symboltab.set_function_proto_pieces(sid, pieces);
        }
    }

    // -----------------------------------------------------------------------
    // Funcdata construction (the W3 boot seam)
    // -----------------------------------------------------------------------

    /// Build a [`Funcdata`] tied to this architecture (the C++ `Funcdata`
    /// constructor, driven from the architecture).
    ///
    /// The W3 [`Funcdata::new`] needs an [`ArchHandle`] carrying the IR-boundary
    /// address-space slice and the analysis unique-start.  Per the established
    /// W3 seam shape (and because the lift emits varnodes carrying their *own*
    /// (engine) spaces directly — see `verify_w3_ir_flow`), the IR-boundary
    /// manager is built fresh from this architecture's const/unique/iop/fspec
    /// spaces, and the analysis unique-start comes from
    /// `Translate::getUniqueStart(ANALYSIS)`.
    pub fn new_funcdata(&self, name: &str, addr: Address, size: int4) -> KunaResult<Funcdata> {
        let uniq_start = self.translate.get_unique_start(UniqueLayout::ANALYSIS);
        let glb = self.build_arch_handle();
        // C++: nm == "" => filled in by decode (localmap None); else a real name.
        Funcdata::new(name, name, glb, addr, uniq_start, size)
    }

    /// Build the [`ArchHandle`] (the [`ArchSeam`] the W3 IR holds as `glb`).
    ///
    /// LOSS-132 keystone: the handle **shares the engine's single
    /// `AddrSpaceManager`** (the `Rc` the SLEIGH translator populated, with
    /// fspec/iop/join inserted by [`Architecture::insert_ir_call_spaces`]).  The
    /// lift-emitted varnodes carry `Rc<AddrSpace>` from exactly this manager, so
    /// `glb.manage()` returns the same space identities and indices the analysis
    /// passes (heritage and downstream) key their per-space state by.  There is
    /// now one manager, faithful to the C++ `Architecture : AddrSpaceManager`.
    pub fn build_arch_handle(&self) -> ArchHandle {
        let manage = self.translate.manager_rc();
        let mut seam = ArchSeam::new_shared(manage);
        seam.min_laned_register_size = self.get_minimum_laned_register_size();
        // Carry the laned-register table so the per-function Funcdata reaches
        // `glb->getLanedRegister` (C++ `Architecture::lanerecords`); cheap clones
        // of the small (size,mask) records.  ActionLaneDivide reads these to
        // split XMM/ZMM vector lanes.
        seam.lanerecords = self.lanerecords.clone();
        // Share the engine's OpBehavior emulation table with `glb` (the C++
        // `Architecture` owns the `TypeOp`s, so `glb->inst[opc]->getBehavior()`
        // reaches them directly).  The `Rc<dyn OpBehavior>` entries are cheap
        // clones; the IR-transform passes (RuleCollapseConstants) fold constants
        // through `glb.op_behavior(opc)`.
        seam.opbehaviors = self.opbehaviors.clone();
        // Share the processor's float formats with `glb` (the C++ `Architecture`
        // IS-A `Translate`, so `glb->translate->getFloatFormat` reaches them).
        // `SubfloatFlow` reads them off the per-function `glb` to drive the
        // float-precision narrowing (`RuleSubfloatConvert`); cheap clones of the
        // small format records.
        seam.floatformats = self.translate.float_formats().to_vec();
        // Share the prototype-model registry handles (C++ `glb->defaultfp` /
        // `evalfp_current`) so the proto-recovery actions can set the function's
        // model and run output recovery against the real param lists.
        seam.defaultfp = self.defaultfp.clone();
        seam.evalfp_current = self.evalfp_current.clone();
        // Carry the cspec's return-address storage (C++ `glb->defaultReturnAddr`)
        // so the per-function `Funcdata::testForReturnAddress` can detect a
        // BRANCHIND that is really a tail return through the return-address
        // register (the Switch-return jump-table failure mode `fail_return`).
        seam.default_return_addr = self.default_return_addr.clone();
        seam.trim_recurse_max = self.trim_recurse_max;
        seam.max_implied_ref = self.max_implied_ref;
        seam.max_term_duplication = self.max_term_duplication;
        seam.return_single = self.return_single;
        // (kuna GH-9218) carry the unjustified-input forward-absorb gate so
        // `ActionUnjustifiedParams` reaches it via `glb`.
        seam.input_varnode_adjust = self.input_varnode_adjust;
        seam.name_style_angr = self.name_style_angr;
        // (kuna) carry the duplicate-declaration collapse gate so `emit_local_var_decls`
        // (which reads the seam `arch`) sees `option dedupvardecls`.
        seam.dedup_var_decls = self.dedup_var_decls;
        // (kuna GH-558) carry the comparison-presentation gate so the
        // `compareform canonical|original` option reaches
        // `ActionPresentCompareForm` via `glb` (the seam read site).
        seam.present_lessequal = self.present_lessequal;
        // (kuna) carry the remaining stage-model rule gates so their `option
        // <name> on|off` reaches the consuming Rule/Action via `glb` (each rule
        // reads `data.get_arch().<flag>`; the rule is registered `enabled=false`
        // so the live flag drives both the DIV default and the toggle).
        seam.fold_boolean_mask = self.fold_boolean_mask; // GH-1282 booleanmask
        seam.fold_flag_compare = self.fold_flag_compare; // GH-1276/8777 flagcompare
        seam.add_carry_chain = self.add_carry_chain; // GH-8913 addcarrychain
        seam.ov_less_simplify = self.ov_less_simplify; // GH-7190 ovlesssimplify
        seam.recover_array_stride = self.recover_array_stride; // GH-8724 arraystride
        seam.memset_recover = self.memset_recover; // GH-9230/1537 memsetrecover
        seam.model_stack_probe_loop = self.model_stack_probe_loop; // GH-8017 stackprobeloop
        seam.recover_lowered_switch = self.recover_lowered_switch; // loweredswitch
        seam.region_structure = self.region_structure; // regionstructure
        seam.region_loop_refine = self.region_loop_refine; // regionlooprefine
        seam.region_edge_order = self.region_edge_order; // regionedgeorder
        seam.reduce_return_gotos = self.reduce_return_gotos; // gotoreduce
        seam.flatten_ifelse = self.flatten_ifelse; // ifelseflatten
        seam.revert_cross_jumps = self.revert_cross_jumps; // crossjumprevert
        seam.dup_return_call_tails = self.dup_return_call_tails; // taildup
        seam.dedup_ite_tail = self.dedup_ite_tail; // dedupitetail
        seam.iteregion = self.iteregion; // iteregion (diamond -> ?: ternary, runtime-choice)
        seam.duplicate_shared_returns = self.duplicate_shared_returns; // returndup
        seam.early_return = self.early_return; // earlyreturn
        seam.recover_loop_break = self.recover_loop_break; // loopbreak_recovery
        seam.fold_call_returns = self.fold_call_returns; // foldcallret
        seam.strip_stack_guard = self.strip_stack_guard; // stackguard
        seam.branch_flip = self.branch_flip; // branchflip (negated-guard branch flipping)
        // (kuna) GH-9203 DIV-3: carry the loop-block COPY-placement gate so the
        // `condexeplace off` option reaches `ActionConditionalConst` via `glb`.
        seam.condexe_block_placement = self.condexe_block_placement;
        // (kuna) carry the whiledo->for reroll gate (C++ `glb->analyze_for_loops`)
        // so `ActionStructureTransform` reaches it for
        // `Funcdata::finalize_forloop_transform`.
        seam.analyze_for_loops = self.analyze_for_loops;
        // Carry the `nanignore all` flag (C++ `glb->nan_ignore_all`) so
        // `RuleIgnoreNan` reaches it via `glb`.
        seam.nan_ignore_all = self.nan_ignore_all;
        // Share the populated data-type factory so `ActionInferTypes` (run via
        // `glb`) reaches the same interned core types this side cached.
        seam.types = Some(self.types_rc());
        // Share the decoded-string manager (C++ `glb->stringManager`) so the
        // per-function `Funcdata::getInternalString` registers internal strings
        // into the very instance the printer reads back on this architecture.
        seam.internal_strings = Some(Rc::clone(&self.string_manager));
        // Jump-table recovery constants (C++ `glb->max_jumptable_size` /
        // `funcptr_align`) and the load image (C++ `glb->loader`) so the
        // jump-table emulator reaches the read-only switch table.
        seam.max_jumptable_size = self.max_jumptable_size;
        seam.alias_block_level = self.alias_block_level;
        seam.funcptr_align = self.funcptr_align;
        // (kuna GH-8471) Carry the Thumb-funcptr preservation gate so
        // `RulePtrsubUndo`'s thumb guard reads `glb->preserve_thumb_funcptr`.
        seam.preserve_thumb_funcptr = self.preserve_thumb_funcptr;
        // (kuna) GH-9191: carry the modulo/and-mask jump-table index-bound gate
        // (`option switchmodbound`) so `JumpBasic::recoverModel` reaches it.
        seam.switch_modulo_bound = self.switch_modulo_bound;
        // (kuna, angr) carry the CBRANCH-guard jump-table index-bound gate
        // (`option switchguardbound`) so `JumpBasic::recoverModel` reaches it.
        seam.switch_guard_bound = self.switch_guard_bound;
        // (kuna, angr) carry the loop-carried-base relative-offset jump-table gate
        // (`option switchsharedcase`) so `JumpBasic::recoverModel` reaches it.
        seam.switch_shared_case = self.switch_shared_case;
        // (kuna, angr) carry the multi-predecessor unrolled-guard jump-table gate
        // (`option switchmultipred`) so `JumpBasic::checkUnrolledGuard` reaches it.
        seam.switch_multi_pred = self.switch_multi_pred;
        // (kuna, angr) carry the interleaved unrolled-guard partial-flow gate
        // (`option unrolledguard`) so `FlowInfo::collectEdges` reaches it.
        seam.unrolled_guard = self.unrolled_guard;
        seam.loader = Some(self.translate.loader_rc());
        // Carry the read-only-propagation switch (C++ `glb->readonlypropagate`,
        // flipped by `option readonly`) so `ActionVarnodeProps` reaches it to gate
        // `Funcdata::fillinReadOnly` (the readonly-RAM-global constant fold).
        seam.readonlypropagate = self.readonlypropagate;
        // Carry the data-type-splitting toggle bits (C++ `glb->split_datatype_config`)
        // so `SplitDatatype` / `RuleSplit{Copy,Load,Store}` reach them per function.
        seam.split_datatype_config = self.split_datatype_config;
        // Snapshot the global symbol table onto `glb` so the per-function
        // `setVarnodeProperties` can run `localmap->queryProperties`'s walk into
        // the global scope (C++ `glb` reaches the live `symboltab`; the merged
        // kuna `glb` is a skeleton, so the global scope is wired here, after every
        // `map addr`).  Global-mapped varnodes then pick up `persist`/`addrtied`
        // and their stores survive `ActionDeadCode`.
        seam.global_query = Some(Rc::new(self.symboltab.build_global_query()));
        // Snapshot every source-declared callee prototype (parked on the global
        // FunctionSymbols by `set_function_prototype_pieces`) so the per-function
        // `ActionDefaultParams` copies a known callee's locked `FuncProto` into the
        // call site (C++ `coreaction.cc:2385` `fc->copy(otherfunc->getFuncProto())`).
        seam.callee_protos = self.symboltab.build_callee_proto_pieces();
        // Carry the constant-pointer-inference config (C++ `glb->infer_pointers` /
        // `infer_funcentry`) and the ordered inferable-pointer spaces (C++
        // `glb->inferPtrSpaces`, built by cacheAddrSpaceProperties) so
        // `ActionConstantPtr` (run via `glb`) can rewrite a mapped global-constant
        // address into a typed `PTRSUB(spacebase,off)`.
        seam.infer_pointers = self.infer_pointers;
        seam.infer_funcentry = self.infer_funcentry;
        seam.infer_ptr_spaces = self.infer_ptr_spaces.clone();
        // Snapshot the tracked-register database (C++ `glb->context`'s track base,
        // populated by `set track`) so `ActionConstbase` can query it for the
        // function entry address through the detached per-function skeleton.
        seam.tracked_sets = self.with_context_db_mut(|db| db.clone_trackbase());
        Rc::new(seam)
    }

    /// Insert the analysis-only fspec/iop/join spaces into the single engine
    /// manager, mirroring C++ `Architecture::restoreFromSpec`
    /// (architecture.cc:638-640): `FspecSpace`, then `IopSpace`, then
    /// `JoinSpace`, each appended at `numSpaces()`.  Idempotent — re-running
    /// init must not double-insert (the manager rejects a duplicate name).
    fn insert_ir_call_spaces(&mut self) -> KunaResult<()> {
        use kuna_base::space::{FspecSpace, IopSpace, JoinSpace};
        let big_end = self
            .manage()
            .get_default_code_space()
            .map(|s| s.is_big_endian())
            .unwrap_or(false);
        // Already inserted (re-init): the engine manager already carries them.
        if self.manage().get_fspec_space().is_some() {
            return Ok(());
        }
        let manager = self.translate.base_mut().manager_mut();
        let next = manager.num_spaces();
        manager.insert_space(Rc::new(FspecSpace::new(next)))?;
        let next = manager.num_spaces();
        manager.insert_space(Rc::new(IopSpace::new(next)))?;
        let next = manager.num_spaces();
        manager.insert_space(Rc::new(JoinSpace::new(next, big_end)))?;
        Ok(())
    }

    /// Create a `SpacebaseSpace` (a \e virtual stack space) backed by a base
    /// register, mirroring C++ `Architecture::addSpacebase` (architecture.cc:564).
    ///
    /// A new [`SpacebaseSpace`](kuna_base::space::SpacebaseSpace) is constructed
    /// at `numSpaces()`, optionally marked reverse-justified, inserted into the
    /// **single** engine manager (so it gets its `'s'` shortcut and, when named
    /// `"stack"`, becomes the manager's formal stack space), and its base
    /// register location attached via `addSpacebasePointer`.
    ///
    /// \param basespace is the address space underlying the stack (e.g. `ram`)
    /// \param nm is the name of the new space (`"stack"` for the formal one)
    /// \param ptrdata is the register location acting as a pointer into the space
    /// \param trunc_size is the (possibly truncated) register size that fits the space
    /// \param isreversejustified is \b true if small variables are justified opposite of endianness
    /// \param stack_growth is \b true if a stack in this space grows in the negative direction
    /// \param is_formal is the indicator for the \e formal stack space
    #[allow(clippy::too_many_arguments)] // C++ Architecture::addSpacebase signature
    fn add_spacebase(
        &mut self,
        basespace: &Rc<kuna_base::space::AddrSpace>,
        nm: &str,
        ptrdata: &kuna_base::space::VarnodeStorage,
        trunc_size: int4,
        isreversejustified: bool,
        stack_growth: bool,
        is_formal: bool,
    ) -> KunaResult<()> {
        use kuna_base::space::SpacebaseSpace;
        // C++: `int4 ind = numSpaces();` then `new SpacebaseSpace(this, translate,
        // nm, ind, truncSize, basespace, ptrdata.space->getDelay()+1, isFormal)`.
        let big_end = basespace.is_big_endian(); // C++ `t->isBigEndian()`
        // C++ `ptrdata.space->getDelay()+1`: the heritage delay is one past the
        // delay of the space the base register lives in (dereferencing a null
        // ptrdata.space is C++ UB -> panic).
        let dl = ptrdata
            .space
            .as_ref()
            .expect("addSpacebase: base register has a null space (C++ UB)")
            .get_delay()
            + 1;
        let manager = self.translate.base_mut().manager_mut();
        let ind = manager.num_spaces();
        let spc = Rc::new(SpacebaseSpace::new(
            nm,
            ind,
            trunc_size as u32, // cast: int4 truncSize -> uint4 space size
            basespace,
            dl,
            is_formal,
            big_end,
        ));
        if isreversejustified {
            manager.set_reverse_justified(&spc);
        }
        manager.insert_space(Rc::clone(&spc))?;
        // C++ `addSpacebasePointer(spc, ptrdata, truncSize, stackGrowth)`: attach
        // the base register to the freshly-inserted spacebase space.
        manager.add_spacebase_pointer(&spc, ptrdata, trunc_size, stack_growth)?;
        Ok(())
    }

    /// Create the stack space and stack-pointer register from a cspec
    /// `<stackpointer>` element, mirroring C++ `Architecture::decodeStackPointer`
    /// (architecture.cc:983).  This is the cspec branch C++ `parseCompilerConfig`
    /// dispatches to `ELEM_STACKPOINTER`.
    ///
    /// Without this the engine manager has no `IPTR_SPACEBASE` space: `parse_machaddr`
    /// fails on `s0x…` stack addresses ("Bad address: s"), `get_stack_space()` is
    /// `None`, and `Funcdata.localmap` stays `None` — so stack-variable promotion
    /// can never fire.  General over any processor's cspec: the `register`/`space`
    /// attributes are read from the XML and resolved through the engine, with NO
    /// processor-name special-casing.
    ///
    /// The cspec XML is the one [`set_cspec_xml`](Architecture::set_cspec_xml)
    /// recorded; this borrows it (it must stay available for the later
    /// `<default_proto>` decode in [`build_default_proto`](Architecture::build_default_proto)).
    fn decode_stack_pointer(&mut self) -> KunaResult<()> {
        use kuna_base::xml::DocumentStorage;
        let Some(xml) = self.cspec_xml.clone() else {
            return Ok(()); // no cspec recorded: nothing to decode (degrade gracefully)
        };
        let mut store = DocumentStorage::new();
        let root = store.parse_document(&xml)?.get_root().clone();
        // The resolved .cspec root IS <compiler_spec> (C++ getTag("compiler_spec")).
        let Some(sp) = find_child(&root, "stackpointer") else {
            // No <stackpointer> in this cspec: leave the manager without a stack
            // space (C++ never reaches decodeStackPointer for such a spec).
            return Ok(());
        };

        // C++ attribute loop over <stackpointer>: register, space, growth,
        // reversejustify.  Defaults: stackGrowth=true (negative), reversejustify
        // false.
        let register_name = attr_str(&sp, "register").unwrap_or_default();
        // C++ `stackGrowth = decoder.readString() == "negative"`.
        let stack_growth =
            attr_str(&sp, "growth").map(|g| g == "negative").unwrap_or(true);
        let isreversejustify =
            attr_str(&sp, "reversejustify").map(|s| s == "true").unwrap_or(false);
        let space_name = attr_str(&sp, "space");

        // C++: `if (basespace == 0) throw "missing space attribute"`.
        let space_name = space_name.ok_or_else(|| {
            KunaError::lowlevel("stackpointer element missing \"space\" attribute")
        })?;
        let basespace = self
            .manage()
            .get_space_by_name(&space_name)
            .cloned()
            .ok_or_else(|| {
                KunaError::lowlevel(format!("stackpointer space \"{space_name}\" not found"))
            })?;

        // C++ `translate->getRegister(registerName)` -> the base-register location.
        let point_num = self.get_register_varnode(register_name.as_bytes())?;
        let point = kuna_sleigh::translate::storage_from_varnode_data(&point_num);

        // C++ truncation: if creating a stackpointer to a truncated space, truncate
        // the stackpointer to the space's address size.
        let mut trunc_size = point.size as int4;
        if basespace.is_truncated() && point.size > basespace.get_addr_size() {
            trunc_size = basespace.get_addr_size() as int4;
        }

        // Already created (re-init): the manager already carries the stack space.
        if self.manage().get_stack_space().is_some() {
            return Ok(());
        }

        // C++ `addSpacebase(basespace, "stack", point, truncSize, isreversejustify,
        // stackGrowth, true)` — create the "official" stackpointer.
        self.add_spacebase(
            &basespace,
            "stack",
            &point,
            trunc_size,
            isreversejustify,
            stack_growth,
            true,
        )
    }

    /// Decode the cspec `<funcptr align="N"/>` element into [`funcptr_align`]
    /// (C++ `Architecture::decodeFuncPtrAlign`, `architecture.cc:1048`,
    /// dispatched from `parseCompilerConfig`'s `ELEM_FUNCPTR` arm).
    ///
    /// The XML `align` attribute is a byte alignment (`2` for ARM word-aligned
    /// function pointers whose least-significant bit encodes the Thumb mode);
    /// `funcptr_align` stores the *bit position* of its first set bit (so
    /// `align="2"` → `funcptr_align = 1`), exactly as the C++ `while((align&1)==0)`
    /// loop computes.  An absent element leaves `funcptr_align = 0` (no alignment),
    /// matching the C++ default.  General over any cspec — no processor special-
    /// casing.  Feeds the kuna GH-8471 `RulePtrsubUndo` thumb-funcptr guard (and
    /// the already-ported `RuleFuncPtrEncoding`/jumptable readers of this field).
    ///
    /// [`funcptr_align`]: Architecture::funcptr_align
    fn decode_funcptr_align(&mut self) -> KunaResult<()> {
        use kuna_base::xml::DocumentStorage;
        let Some(xml) = self.cspec_xml.clone() else {
            return Ok(()); // no cspec recorded: leave funcptr_align = 0
        };
        let mut store = DocumentStorage::new();
        let root = store.parse_document(&xml)?.get_root().clone();
        // The resolved .cspec root IS <compiler_spec>; <funcptr> is a direct child.
        let Some(fp) = find_child(&root, "funcptr") else {
            return Ok(()); // no <funcptr> in this cspec: funcptr_align stays 0
        };
        // int4 align = decoder.readSignedInteger(ATTRIB_ALIGN);
        let align: i64 = match attr_str(&fp, "align").and_then(|s| parse_int(&s)) {
            Some(a) => a as i64,
            None => return Ok(()), // malformed/absent attr: leave default
        };
        if align == 0 {
            self.funcptr_align = 0; // No alignment
            return Ok(());
        }
        // bits = position of the first set bit (C++ `while((align&1)==0) bits++`).
        let mut bits: int4 = 0;
        let mut a = align;
        while (a & 1) == 0 {
            bits += 1;
            a >>= 1;
        }
        self.funcptr_align = bits;
        Ok(())
    }

    /// Interpret a constant as a pointer into `spc` (C++ `Architecture::
    /// resolveConstant`, viewed as an `AddrSpaceManager`).  A thin wrapper over the
    /// shared engine manager so callers that hold `&self` (not the manager) can run
    /// the resolve — the per-function `glb` carries its own
    /// [`resolve_constant`](crate::seams::Architecture::resolve_constant), this is
    /// the architecture-side analogue used while building `inferPtrSpaces`.
    pub fn resolve_constant(
        &self,
        spc: &Rc<AddrSpace>,
        val: uintb,
        sz: int4,
        point: &Address,
        full_encoding: &mut uintb,
    ) -> KunaResult<Address> {
        self.manage().resolve_constant(spc, val, sz, point, full_encoding)
    }

    /// Determine the minimum pointer size for each space and set up the ordered,
    /// filtered, deduplicated list of inferable spaces (C++
    /// `Architecture::cacheAddrSpaceProperties`, architecture.cc:671-707).
    ///
    /// Inferable spaces are the default code+data spaces plus anything the cspec
    /// `<global>` tag pushed onto `infer_ptr_spaces` (via [`decode_global`]), minus
    /// register spaces (`getDelay() == 0`), spacebase spaces, OTHER spaces, and
    /// overlays.  The list is sorted by space index and deduplicated, then the
    /// default *data* space is promoted to position 0 (so it is the first space a
    /// likely-pointer constant is tested against — the load-bearing line for the
    /// x86-64 global arrays this wave targets, whose `myarray`/`paiGlob` live in
    /// `ram`, the default data space).
    ///
    /// LOSS: the C++ segment-op near-pointer promotion (architecture.cc:696-700,
    /// `getSegmentOp(spc)` -> `markNearPointers`) is not transcribed — no
    /// `getSegmentOp(space)` lookup is wired here and no datatest exercises a
    /// segmented near-pointer space (x86 real-mode `seg:off`); for the flat
    /// spaces this wave's targets use, `getSegmentOp` is always null and the
    /// loop is a no-op.  General over any processor's cspec: the spaces are read
    /// from the manager and the cspec, with NO processor-name special-casing.
    ///
    /// [`decode_global`]: Architecture::decode_global
    pub fn cache_addr_space_properties(&mut self) {
        use kuna_base::space::spacetype;
        // copyList = inferPtrSpaces; push default code + data spaces.
        let mut copy_list: Vec<Rc<AddrSpace>> = self.infer_ptr_spaces.clone();
        let code_spc = self.manage().get_default_code_space().cloned();
        let data_spc = self.manage().get_default_data_space().cloned();
        if let Some(spc) = code_spc {
            copy_list.push(spc); // Make sure the default code space is present
        }
        if let Some(ref spc) = data_spc {
            copy_list.push(Rc::clone(spc)); // Make sure the default data space is present
        }
        self.infer_ptr_spaces.clear();
        // sort(copyList, AddrSpace::compareByIndex)
        copy_list.sort_by(|a, b| {
            if AddrSpace::compare_by_index(a, b) {
                std::cmp::Ordering::Less
            } else if AddrSpace::compare_by_index(b, a) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });
        let mut last_space: Option<Rc<AddrSpace>> = None;
        for spc in copy_list.into_iter() {
            if let Some(ref last) = last_space {
                if Rc::ptr_eq(last, &spc) {
                    continue; // dedup (sorted)
                }
            }
            last_space = Some(Rc::clone(&spc));
            if spc.get_delay() == 0 {
                continue; // Don't put in a register space
            }
            if spc.get_type() == spacetype::IPTR_SPACEBASE {
                continue;
            }
            if spc.is_other_space() {
                continue;
            }
            if spc.is_overlay() {
                continue;
            }
            self.infer_ptr_spaces.push(spc);
        }

        // Promote the default DATA space to position 0 (the inferring default).
        // (The C++ segment-op near-pointer markNearPointers loop is a LOSS here;
        // the defPos search still runs so the data space leads.)
        let mut def_pos: i32 = -1;
        if let Some(ref data) = data_spc {
            for (i, spc) in self.infer_ptr_spaces.iter().enumerate() {
                if Rc::ptr_eq(spc, data) {
                    def_pos = i as i32;
                    break;
                }
            }
        }
        if def_pos > 0 {
            self.infer_ptr_spaces.swap(0, def_pos as usize);
        }
    }

    /// Decode the cspec `<global>` element and seed the global scope's owned
    /// range tree (C++ `Architecture::decodeGlobal` + `addToGlobalScope`,
    /// `architecture.cc:816-848`, dispatched from `parseCompilerConfig`'s
    /// `ELEM_GLOBAL` arm at `architecture.cc:1276-1277` and the deferred
    /// `globalRanges` apply loop at `architecture.cc:1336-1337`).
    ///
    /// Each child `<range>`/`<register>` decodes to a [`RangeProperties`]; an
    /// empty `<range space="ram"/>` (no `first`/`last`) widens to the whole space
    /// (`Range::from_properties` sets `last = spc->getHighest()` when `seenLast`
    /// is false).  The resulting `Range` is added to the global scope's rangetree
    /// via `symboltab->addRange(globalScope, spc, first, last)`.
    ///
    /// This is THE seam the revisit / global-persist path depends on: with the
    /// global scope owning the `ram` range, `Scope::queryProperties`'s `inScope`
    /// discovery branch (database.cc:1276-1281) returns
    /// `mapped | addrtied | persist` for any RAM Varnode with no covering Symbol,
    /// so global RAM stores survive `ActionDeadCode` and hold their call
    /// `INDIRECT`s.
    ///
    /// LOSS: the C++ overlay-space duplication (`addToGlobalScope`,
    /// architecture.cc:838-846) and the `inferPtrSpaces` push (architecture.cc:836,
    /// a pointer-inference seam) are not transcribed — no datatest exercises an
    /// overlay base space here, and `inferPtrSpaces` feeds only `TypeFactory`
    /// pointer inference (a separate seam).  General over any processor's cspec:
    /// the space names are read from the XML and resolved through the engine, with
    /// NO processor-name special-casing.
    fn decode_global(&mut self) -> KunaResult<()> {
        use kuna_base::address::{Range, RangeProperties};
        use kuna_base::marshal::{IdRegistry, XmlDecode};
        use kuna_base::xml::DocumentStorage;

        let Some(xml) = self.cspec_xml.clone() else {
            return Ok(()); // no cspec recorded: nothing to seed
        };
        let mut store = DocumentStorage::new();
        let root = store.parse_document(&xml)?.get_root().clone();
        // The resolved .cspec root IS <compiler_spec>; <global> is a direct child.
        let Some(global_el) = find_child(&root, "global") else {
            // No <global> in this cspec: the global scope owns no ranges (C++
            // never reaches addToGlobalScope for such a spec).
            return Ok(());
        };

        // C++ `Architecture::decodeGlobal`: openElement(GLOBAL); while
        // peekElement() != 0 { rangeProps.emplace_back(); rangeProps.back().decode(decoder); }
        // We decode the children directly (the kuna-base `RangeProperties::decode`
        // is a `Decoder` consumer, identical to C++).  Each `<range>`/`<register>`
        // becomes a `RangeProperties`, then `addToGlobalScope`'s `Range(props,this)`
        // + `symboltab->addRange`.
        let manager = self.translate.base().manager_rc();
        let registry = IdRegistry::with_base_ids();
        let scope = match self.symboltab.get_global_scope() {
            Some(s) => s,
            None => return Ok(()), // no global scope attached (degrade gracefully)
        };
        // Collect the resolved (space, first, last) triples first, so the register
        // arm can borrow `self.translate` (via `get_register_varnode`) before the
        // `&mut self.symboltab.add_range` below.
        let mut to_add: Vec<(Rc<AddrSpace>, uintb, uintb)> = Vec::new();
        for child in global_el.get_children().iter() {
            let nm = child.get_name();
            if nm == "register" {
                // C++ `Range::Range` register branch (address.cc:239-245):
                //   point = trans->getRegister(properties.spaceName);
                //   spc = point.space; first = point.offset;
                //   last = (first-1) + point.size;
                // We resolve through the Translate (the reliably-installed register
                // lookup, the same path decode_stack_pointer uses) rather than
                // kuna-base's `Range::from_properties`, whose `manage.register_lookup()`
                // is not wired in every fixture.  `name` carries the register name.
                let reg_name = match attr_str(child, "name") {
                    Some(n) => n,
                    None => continue,
                };
                let point = self.get_register_varnode(reg_name.as_bytes())?;
                let spc = match point.space.clone() {
                    Some(s) => s,
                    None => continue, // null register space (C++ UB) — skip defensively
                };
                let first = point.offset;
                // last = (first-1) + point.size, uintb wraparound (address.cc:244).
                let last = first.wrapping_sub(1).wrapping_add(u64::from(point.size));
                to_add.push((spc, first, last));
            } else if nm == "range" {
                // C++ `Range::Range` range branch: resolve the space, widen the
                // empty form to spc->getHighest().  No register lookup needed.
                let mut decoder = XmlDecode::new_with_root(&manager, &registry, child, 0);
                let mut props = RangeProperties::new();
                props.decode(&mut decoder)?;
                let range = Range::from_properties(&props, self.manage())?;
                to_add.push((
                    Rc::clone(range.get_space()),
                    range.get_first(),
                    range.get_last(),
                ));
            }
            // (Any other child element is ignored, exactly as C++
            // RangeProperties::decode accepts only <range>/<register>.)
        }
        // C++ `addToGlobalScope`: symboltab->addRange(globalScope, spc, first, last)
        // for each resolved range, AND inferPtrSpaces.push_back(spc)
        // (architecture.cc:836 — the LOSS-208 F1 site the global-persist2 wave left
        // un-transcribed).  cacheAddrSpaceProperties (run from postSpecFile after
        // this) then sorts/filters/dedups the pushed spaces.
        for (spc, first, last) in to_add {
            self.infer_ptr_spaces.push(Rc::clone(&spc));
            self.symboltab.add_range(scope, spc, first, last);
        }
        Ok(())
    }

    /// Decode the cspec `<callfixup>` elements into the p-code injection library
    /// (C++ `parseCompilerConfig` -> `ELEM_CALLFIXUP` ->
    /// `pcodeinjectlib->decodeInject(archid+" : compiler spec","",CALLFIXUP_TYPE,decoder)`,
    /// `architecture.cc:1291`).  After this every cspec-defined call-fixup is
    /// registered (and resolvable by `getPayloadId(CALLFIXUP_TYPE,name)`), so the
    /// console `fixup apply <fixup> <function>` command can find it.
    ///
    /// The SLEIGH compile of each fixup body (`parseInject`) stays deferred
    /// (LOSS-031); only the decode/registration runs here, which is all
    /// `getPayloadId`/`setInjectId` need.  General over any processor's cspec.
    fn decode_call_fixups(&mut self) -> KunaResult<()> {
        use kuna_base::marshal::{IdRegistry, XmlDecode};
        use kuna_base::xml::DocumentStorage;

        let Some(xml) = self.cspec_xml.clone() else {
            return Ok(());
        };
        let mut store = DocumentStorage::new();
        let root = store.parse_document(&xml)?.get_root().clone();
        // Gather the <callfixup> children (the cspec root IS <compiler_spec>).
        let fixups: Vec<Rc<kuna_base::xml::Element>> = root
            .get_children()
            .iter()
            .filter(|c| c.get_name() == "callfixup")
            .cloned()
            .collect();
        if fixups.is_empty() {
            return Ok(());
        }
        // The injection element/attribute ids the payload decode reads
        // (callfixup/pcode/body/target/name/...).
        let manager = self.translate.base().manager_rc();
        let mut registry = IdRegistry::with_base_ids();
        crate::pcodeinject::register_ids(&mut registry);
        for fixup in fixups.iter() {
            let mut decoder = XmlDecode::new_with_root(&manager, &registry, fixup, 0);
            // C++ src = archid+" : compiler spec"; the kuna engine carries no archid
            // string here, so the source label is the constant suffix (only surfaces
            // in error messages / debug dumps, never in test output).
            self.pcodeinjectlib.decode_inject(
                b" : compiler spec",
                b"",
                crate::pcodeinject::CALLFIXUP_TYPE,
                &mut decoder,
            )?;
        }
        Ok(())
    }

    /// Initialize the user-op table and decode the cspec `<callotherfixup>`
    /// elements, then compile every registered injection body (C++
    /// `restoreFromSpec`: `userops.initialize(this)` at architecture.cc:641, plus
    /// the `<callotherfixup>` dispatch in `parseCompilerConfig` →
    /// `userops.decodeCallOtherFixup(decoder,this)` at architecture.cc:1294).
    ///
    /// `userops.initialize` assigns every translator-presented user-op a default
    /// `UnspecializedPcodeOp` description (so e.g. MIPS `setISAMode` has an index
    /// and name); each `<callotherfixup>` then *overrides* that base entry with an
    /// `InjectedUserOp` carrying the compiled fixup p-code.  The compile
    /// (`parseInject`) runs last, once the whole inject library is registered, so
    /// the per-payload temporary-register base advances exactly as the C++.
    ///
    /// The `&mut self` borrow needed as `UseropArchitecture` aliases
    /// `self.userops`, so the manager is moved out with `mem::take`, driven
    /// against the rest of `self` (which still owns `pcodeinjectlib`), then
    /// restored — the established split-borrow convention.
    fn init_userops_and_fixups(&mut self) -> KunaResult<()> {
        use kuna_base::marshal::{IdRegistry, XmlDecode};
        use kuna_base::xml::DocumentStorage;

        // 1. userops.initialize(this): default UnspecializedPcodeOp per translator
        //    user-op name.
        let mut userops = std::mem::take(&mut self.userops);
        let init_res = userops.initialize(self);
        if let Err(e) = init_res {
            self.userops = userops;
            return Err(e);
        }

        // 2. parseCompilerConfig: dispatch each cspec `<callotherfixup>` child to
        //    userops.decodeCallOtherFixup(decoder,this).
        let fixup_res = (|| -> KunaResult<()> {
            let Some(xml) = self.cspec_xml.clone() else {
                return Ok(());
            };
            let mut store = DocumentStorage::new();
            let root = store.parse_document(&xml)?.get_root().clone();
            let fixups: Vec<Rc<kuna_base::xml::Element>> = root
                .get_children()
                .iter()
                .filter(|c| c.get_name() == "callotherfixup")
                .cloned()
                .collect();
            if fixups.is_empty() {
                return Ok(());
            }
            let manager = self.translate.base().manager_rc();
            let mut registry = IdRegistry::with_base_ids();
            crate::pcodeinject::register_ids(&mut registry);
            crate::userop::register_ids(&mut registry);
            for fixup in fixups.iter() {
                let mut decoder = XmlDecode::new_with_root(&manager, &registry, fixup, 0);
                userops.decode_call_other_fixup(&mut decoder, self)?;
            }
            Ok(())
        })();
        self.userops = userops;
        fixup_res?;

        // 3. parseInject: compile every registered injection body (callfixup +
        //    callotherfixup) into a ConstructTpl against the loaded language.
        //    Move the inject library out so the &Sleigh (SnippetLanguageProvider)
        //    borrow of self.translate does not alias the &mut library.
        let mut lib = std::mem::take(&mut self.pcodeinjectlib);
        // The SnippetLanguage is the loaded `SleighBase`; drive parse_inject over
        // it (the &SleighBase read does not alias the &mut library).
        let parse_res = lib.parse_inject_all(self.translate.base());
        self.pcodeinjectlib = lib;
        parse_res
    }

    /// (kuna) Register the fixed set of string-copy builtin user-ops into
    /// `userops` so the printer's `opCallother` path resolves their name,
    /// display, and typed parameters.  Mirrors the lazy
    /// `userops.registerBuiltin(BUILTIN_*)` calls in `ArraySequence::buildStringCopy`
    /// and `Funcdata::getInternalString` (C++ does these on demand during the
    /// transform; the kuna seam can't reach the real `userops`, so they are
    /// front-loaded here).
    fn register_string_builtins(&mut self) -> KunaResult<()> {
        use crate::userop::{
            BUILTIN_MEMCPY, BUILTIN_MEMSET, BUILTIN_STRINGDATA, BUILTIN_STRNCPY,
            BUILTIN_VOLATILE_READ, BUILTIN_VOLATILE_WRITE, BUILTIN_WCSNCPY,
        };
        // Split the &mut userops borrow from the &self type-factory read by
        // building a small adapter over the (already-populated) factory.
        let adapter = BuiltinTypeArch {
            types: Rc::clone(&self.types),
            data_word_size: self
                .manage()
                .get_default_data_space()
                .map(|s| s.get_word_size() as int4)
                .unwrap_or(1),
        };
        let mut userops = std::mem::take(&mut self.userops);
        let res = (|| -> KunaResult<()> {
            userops.register_builtin(BUILTIN_STRINGDATA, &adapter)?;
            // The volatile builtins (`read_volatile`/`write_volatile`) are
            // registered lazily by `Funcdata::replaceVolatile`'s
            // `glb->userops.registerBuiltin(...)` in C++ (userop.cc:444-448); the
            // call is idempotent and only populates `builtinmap` so the print pass
            // can resolve the CALLOTHER index to its operator name.  They carry no
            // type-factory dependency, so pre-seeding them here is behaviorally
            // equivalent and keeps `replaceVolatile` free of an `&mut glb` borrow.
            userops.register_builtin(BUILTIN_VOLATILE_READ, &adapter)?;
            userops.register_builtin(BUILTIN_VOLATILE_WRITE, &adapter)?;
            userops.register_builtin(BUILTIN_MEMCPY, &adapter)?;
            userops.register_builtin(BUILTIN_STRNCPY, &adapter)?;
            userops.register_builtin(BUILTIN_WCSNCPY, &adapter)?;
            // (kuna GH-9230/1537) the constant-fill recovery CALLOTHER.
            userops.register_builtin(BUILTIN_MEMSET, &adapter)?;
            Ok(())
        })();
        self.userops = userops;
        res
    }

    // -----------------------------------------------------------------------
    // Owned-subsystem accessors (the `glb->types`/`glb->print`/… surface the
    // ifacedecomp porter confirmed were absent — w9x-arch-engine-glue)
    // -----------------------------------------------------------------------

    /// Borrow the data-type factory (C++ `glb->types`).
    pub fn types(&self) -> &dyn TypeFactory {
        &*self.types
    }

    /// Borrow the concrete type factory (when the `TypeFactoryImpl`-specific
    /// builders, e.g. `set_core_type`, are needed by the init pipeline).
    pub fn types_impl(&self) -> &TypeFactoryImpl {
        &self.types
    }

    /// Share the data-type factory `Rc` so the analysis-side seam (`glb`) reaches
    /// the same populated factory (`ActionInferTypes` -> `glb.types()`).
    pub fn types_rc(&self) -> Rc<TypeFactoryImpl> {
        Rc::clone(&self.types)
    }

    /// Borrow the c-language printer (C++ `glb->print`).
    pub fn print(&self) -> &PrintC {
        &self.print
    }

    /// (kuna) Borrow the per-program restart-trigger log (read by the `restarts`
    /// console command).
    pub fn restart_log(&self) -> &crate::kuna_restartlog::RestartLog {
        &self.restart_log
    }

    /// (kuna) Mutably borrow the restart-trigger log (the trigger sites record
    /// into it).
    pub fn restart_log_mut(&mut self) -> &mut crate::kuna_restartlog::RestartLog {
        &mut self.restart_log
    }

    /// Mutably borrow the c-language printer (drives `docFunction` + the print
    /// option setters).
    pub fn print_mut(&mut self) -> &mut PrintC {
        &mut self.print
    }

    /// Move the printer out of `self` (replacing it with a fresh default), so a
    /// caller can drive `PrintC::doc_function_full(fd, &self)` — which needs an
    /// immutable borrow of the rest of the architecture (register-name lookup)
    /// while it mutates the printer.  Pair with [`put_print`](Architecture::put_print).
    pub fn take_print(&mut self) -> PrintC {
        std::mem::take(&mut self.print)
    }

    /// Move a printer back into `self` (the partner of [`take_print`]).
    pub fn put_print(&mut self, print: PrintC) {
        self.print = print;
    }

    /// Install the load image (C++ `glb->loader`; owned inside the engine in
    /// the Rust port).
    ///
    /// The C++ `Architecture::loader` is a `LoadImage*` the translator was given;
    /// in the Rust port the `Sleigh` engine owns the loader (it borrows it behind
    /// a `RefCell` for `load_fill`, driven by decode), so the architecture's
    /// loader surface is the engine's bound image.  This forwards to
    /// `Sleigh::set_loader`, matching the C++ `restoreFromSpec` handing the
    /// loader to the translator.
    pub fn set_loader(&mut self, loader: Box<dyn kuna_sleigh::loadimage::LoadImage>) {
        self.translate.set_loader(loader);
    }

    /// Read a `sz`-byte value out of the load image at `addr` (C++
    /// `EmulatePcodeOp::getLoadImageValue` via `glb->loader->loadFill`).  The
    /// loader is owned by the engine in the Rust port, so this forwards to the
    /// `Sleigh` engine's [`read_loadimage_value`](kuna_sleigh::sleigh::Sleigh::read_loadimage_value).
    /// Drives jump-table LOAD emulation.
    pub fn read_loadimage_value(&self, addr: &Address, sz: int4) -> KunaResult<uintb> {
        self.translate.read_loadimage_value(addr, sz)
    }

    /// Forward `glb->translate->allowContextSet(val)` — the context database is
    /// owned inside the engine in the Rust port (C++ `glb->context` is a
    /// `ContextDatabase*` the translator holds; `Sleigh` owns it here), so the
    /// architecture's context surface forwards to the engine.
    pub fn context_allow_set(&self, val: bool) {
        self.translate.allow_context_set(val);
    }

    /// Run a closure with mutable access to the engine's `ContextDatabase` (C++
    /// `glb->context`).  Drives the `set context` / `set track` console commands;
    /// forwards to the owned [`Sleigh`] engine.
    pub fn with_context_db_mut<R>(
        &self,
        f: impl FnOnce(&mut dyn kuna_sleigh::globalcontext::ContextDatabase) -> R,
    ) -> R {
        self.translate.with_context_db_mut(f)
    }

    /// Resolve a register by name to its storage (C++
    /// `glb->translate->getRegister(name)`); used by `set track`.
    pub fn get_register_varnode(
        &self,
        nm: &[u8],
    ) -> KunaResult<kuna_num::pcoderaw::VarnodeData> {
        self.translate.get_register_varnode(nm)
    }

    /// The data-organization the C-declaration grammar consults (C++
    /// `glb->getDefaultDataSpace()->getAddrSize()` / `getWordSize()`), packaged as
    /// `(addr_size, word_size)` for the `parse_C` / `parse_type` entry points the
    /// console `parse line` drives.  A bootstrapped architecture always has a
    /// default data space (C++ `getDefaultDataSpace` asserts the same).
    pub fn data_org(&self) -> (int4, uint4) {
        let spc = self
            .manage()
            .get_default_data_space()
            .expect("Architecture::data_org: bootstrapped architecture has a default data space");
        (spc.get_addr_size() as int4, spc.get_word_size())
    }

    // -----------------------------------------------------------------------
    // Prototype-model registry (C++ protoModels / defaultfp / evalfp_current)
    // -----------------------------------------------------------------------

    /// Look up a prototype model by name (C++ `Architecture::getModel`,
    /// architecture.cc:235 — `protoModels.find(nm)`).  Returns `None` for an
    /// unregistered name (the C++ throws `LowlevelError("Unknown prototype
    /// model");` — the caller maps `None` to that).
    pub fn get_model(&self, nm: &str) -> Option<&Rc<ProtoModel>> {
        self.proto_models.get(nm)
    }

    /// Whether a prototype model with the given name is registered (C++
    /// `Architecture::hasModel`).
    pub fn has_model(&self, nm: &str) -> bool {
        self.proto_models.contains_key(nm)
    }

    /// The default prototype model (C++ `glb->defaultfp`).  `None` until a
    /// cspec is parsed / [`build_default_proto`](Architecture::build_default_proto).
    pub fn default_fp(&self) -> Option<&Rc<ProtoModel>> {
        self.defaultfp.as_ref()
    }

    /// The current-evaluation model (C++ `glb->evalfp_current`), falling back
    /// to `defaultfp` when unset (C++ `evalfp_current==0 ? defaultfp : …`).
    pub fn eval_fp_current(&self) -> Option<&Rc<ProtoModel>> {
        self.evalfp_current.as_ref().or(self.defaultfp.as_ref())
    }

    /// Register a prototype model under its name (C++ `protoModels[name] =`).
    pub fn register_model(&mut self, model: Rc<ProtoModel>) {
        self.proto_models.insert(model.get_name().to_string(), model);
    }

    /// Set the default prototype model (C++ `Architecture::setDefaultModel`,
    /// architecture.cc:222).
    pub fn set_default_model_rc(&mut self, model: Rc<ProtoModel>) {
        self.defaultfp = Some(model);
    }

    // -----------------------------------------------------------------------
    // init / restoreFromSpec pipeline (architecture.cc:1395 / sleigh_arch.cc)
    // -----------------------------------------------------------------------

    /// Build the data-type factory + register the data organization
    /// (C++ `SleighArchitecture::buildTypegrp`, sleigh_arch.cc:198 —
    /// `types = new TypeFactory(this)`).  The factory is constructed empty;
    /// [`build_core_types`](Architecture::build_core_types) seeds the core types
    /// and [`finish_typegrp`](Architecture::finish_typegrp) calls `setupSizes`.
    pub fn build_typegrp(&mut self) {
        self.types = Rc::new(TypeFactoryImpl::new());
        self.types.set_max_basetype_size(self.max_basetype_size);
    }

    /// Seed the core data-types (C++ `SleighArchitecture::buildCoreTypes`,
    /// sleigh_arch.cc:204, the no-`<coretypes>` default branch — the verbatim
    /// `setCoreType` sequence + `cacheCoreTypes`).
    pub fn build_core_types(&mut self) -> KunaResult<()> {
        use type_metatype::*;
        let t = &self.types;
        t.set_core_type("void", 1, TYPE_VOID, false)?;
        t.set_core_type("bool", 1, TYPE_BOOL, false)?;
        t.set_core_type("uint1", 1, TYPE_UINT, false)?;
        t.set_core_type("uint2", 2, TYPE_UINT, false)?;
        t.set_core_type("uint4", 4, TYPE_UINT, false)?;
        t.set_core_type("uint8", 8, TYPE_UINT, false)?;
        t.set_core_type("int1", 1, TYPE_INT, false)?;
        t.set_core_type("int2", 2, TYPE_INT, false)?;
        t.set_core_type("int4", 4, TYPE_INT, false)?;
        t.set_core_type("int8", 8, TYPE_INT, false)?;
        t.set_core_type("float4", 4, TYPE_FLOAT, false)?;
        t.set_core_type("float8", 8, TYPE_FLOAT, false)?;
        t.set_core_type("float10", 10, TYPE_FLOAT, false)?;
        t.set_core_type("float16", 16, TYPE_FLOAT, false)?;
        t.set_core_type("xunknown1", 1, TYPE_UNKNOWN, false)?;
        t.set_core_type("xunknown2", 2, TYPE_UNKNOWN, false)?;
        t.set_core_type("xunknown4", 4, TYPE_UNKNOWN, false)?;
        t.set_core_type("xunknown8", 8, TYPE_UNKNOWN, false)?;
        t.set_core_type("code", 1, TYPE_CODE, false)?;
        t.set_core_type("char", 1, TYPE_INT, true)?;
        t.set_core_type("wchar2", 2, TYPE_INT, true)?;
        t.set_core_type("wchar4", 4, TYPE_INT, true)?;
        t.cache_core_types()?;
        Ok(())
    }

    /// Finish the type factory: set up the default sizes (C++
    /// `types->setupSizes()`, the tail of `parseCompilerConfig` when no
    /// `<data_organization>` was registered).  Reads the architecture's default
    /// data-space / stack-pointer widths (the `glb->` accessors the C++
    /// `setupSizes` queries).
    /// Parse the cspec `<data_organization>` size elements into the type factory
    /// (C++ `TypeFactory::decodeDataOrganization`, type.cc:5107).  Sets the
    /// integer/long/pointer/char/wchar default sizes from the compiler spec so
    /// `getSizeOfWChar()` etc. reflect the real ABI (e.g. x86-64 gcc `wchar_size=4`);
    /// `setupSizes` then only fills the elements the spec left unset.  The
    /// `<size_alignment_map>` is left to the existing `set_default_alignment_map`
    /// (a separate cspec item); only the scalar sizes are read here.
    fn decode_data_organization(&self) -> KunaResult<()> {
        use kuna_base::xml::DocumentStorage;
        let Some(xml) = self.cspec_xml.clone() else {
            return Ok(());
        };
        let mut store = DocumentStorage::new();
        let root = store.parse_document(&xml)?.get_root().clone();
        let Some(dorg) =
            root.get_children().iter().find(|c| c.get_name() == "data_organization").cloned()
        else {
            return Ok(());
        };
        let read = |el: &Rc<kuna_base::xml::Element>| -> Option<int4> {
            el.get_attribute_value("value")
                .ok()
                .and_then(|b| std::str::from_utf8(b).ok())
                .and_then(|s| s.trim().parse::<int4>().ok())
        };
        for child in dorg.get_children().iter() {
            match child.get_name() {
                "integer_size" => {
                    if let Some(v) = read(child) {
                        self.types.set_size_of_int(v);
                    }
                }
                "long_size" => {
                    if let Some(v) = read(child) {
                        self.types.set_size_of_long(v);
                    }
                }
                "pointer_size" => {
                    if let Some(v) = read(child) {
                        self.types.set_size_of_pointer(v);
                    }
                }
                "char_size" => {
                    if let Some(v) = read(child) {
                        self.types.set_size_of_char(v);
                    }
                }
                "wchar_size" => {
                    if let Some(v) = read(child) {
                        self.types.set_size_of_wchar(v);
                    }
                }
                "size_alignment_map" => {
                    // C++ `TypeFactory::decodeAlignmentMap` (type.cc:5143): each
                    // `<entry size=N alignment=M/>` child contributes a pair; the
                    // map drives `getAlignment(size)` and so the over-aligned
                    // primitive layout (e.g. x86-64 gcc float10 align=16).
                    let read_attr = |el: &Rc<kuna_base::xml::Element>, attr: &str| -> Option<int4> {
                        el.get_attribute_value(attr)
                            .ok()
                            .and_then(|b| std::str::from_utf8(b).ok())
                            .and_then(|s| s.trim().parse::<int4>().ok())
                    };
                    let mut pairs: Vec<(int4, int4)> = Vec::new();
                    for entry in child.get_children().iter() {
                        if entry.get_name() != "entry" {
                            continue;
                        }
                        if let (Some(sz), Some(al)) =
                            (read_attr(entry, "size"), read_attr(entry, "alignment"))
                        {
                            pairs.push((sz, al));
                        }
                    }
                    if !pairs.is_empty() {
                        self.types.decode_alignment_map(&pairs)?;
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }

    /// Finish the type factory: set up the default sizes (C++
    /// `types->setupSizes()`, the tail of `parseCompilerConfig` when no
    /// `<data_organization>` was registered).  Reads the architecture's default
    /// data-space / stack-pointer widths (the `glb->` accessors the C++
    /// `setupSizes` queries).
    pub fn finish_typegrp(&self) {
        // C++ `parseCompilerConfig` decodes `<data_organization>`
        // (architecture.cc:1268) before the type factory's `setupSizes` defaults
        // run, so spec-given sizes (e.g. gcc's `wchar_size=4`) take precedence.
        let _ = self.decode_data_organization();
        let manage = self.manage();
        let default_size = manage.get_default_size();
        let default_data_addr_size = manage
            .get_default_data_space()
            .map(|s| s.get_addr_size() as int4)
            .unwrap_or(default_size);
        let stack_pointer_size =
            manage.get_stack_space().map(|s| s.get_addr_size() as int4);
        // C++ `TypeFactory` reads `getArch()->getDefaultDataSpace()->isBigEndian()`
        // for bitfield layout (TypeBitField ctor, type.cc:873; struct parse,
        // grammar.cc:2626) and pointer truncation (TypePointer::calcTruncate,
        // type.cc:1202).  Seed that endianness bit here, where the default data
        // space is first known, so big-endian structs lay their bitfields out in
        // memory order (without it every struct is laid out little-endian and the
        // bitfield-expression recovery's BE range can't match the LE-laid fields).
        let big_endian = manage
            .get_default_data_space()
            .map(|s| s.is_big_endian())
            .unwrap_or(false);
        self.types.set_truncate_big_endian(big_endian);
        // C++ `setupSizes` installs the default map only when the cspec did not
        // register a `<size_alignment_map>` (`if (alignMap.empty())`,
        // type.cc:3623).  `decode_data_organization` above already populated the
        // map from the spec when present (e.g. x86-64 gcc 16->16), so preserve it.
        if self.types.alignment_map_is_empty() {
            self.types.set_default_alignment_map();
        }
        self.types.setup_sizes(stack_pointer_size, default_data_addr_size, default_size);
    }

    /// Seed a single default prototype model when the cspec proto decode is not
    /// run (the W6 `decodeDefaultProto`/`decodeProto` cspec pipeline is its own
    /// item).  Builds an empty `unknown`-style default model over the engine's
    /// address spaces so `defaultfp`/`getModel("unknown")` resolve and the
    /// `extrapop` option has a target.  Mirrors the C++ post-`parseCompilerConfig`
    /// invariant that `defaultfp != 0`.
    ///
    /// SEAM(W6 cspec): the *real* default proto model comes from the cspec
    /// `<default_proto><prototype …>` decode (`ProtoModel::decode` building the
    /// param lists from `<input>`/`<output>` `<pentry>` records).  When the
    /// frontend supplied the cspec XML (via [`set_cspec_xml`](Architecture::set_cspec_xml))
    /// the `<default_proto>` input/output parameter lists are decoded here (the
    /// general, spec-driven path — see [`decode_default_proto`](Architecture::decode_default_proto)),
    /// so the recovered model carries real return/parameter storage and the
    /// proto-recovery actions can fire.  Otherwise a name-only default model is
    /// registered so the engine still has a non-null `defaultfp`.
    pub fn build_default_proto(&mut self) {
        // C++ `Architecture::decodeReturnAddress` (architecture.cc:902) decodes the
        // cspec's top-level <returnaddress> into `defaultReturnAddr`.  Do it here
        // before the `take()` below consumes the cspec; a missing/empty element
        // leaves `default_return_addr` as `None` (== `space == 0`).
        if let Some(xml) = self.cspec_xml.clone() {
            self.default_return_addr = self.decode_default_return_addr(&xml);
        }
        if let Some(xml) = self.cspec_xml.take() {
            match self.decode_default_proto(&xml) {
                Ok(model) => {
                    let name = model.get_name().to_string();
                    let rc = Rc::new(model);
                    self.register_model(Rc::clone(&rc));
                    // Re-register under "unknown" too: the C++ leaves `getModel`
                    // resolving the model name, but `defaultfp` is the object.
                    let _ = name;
                    self.defaultfp = Some(rc);
                    return;
                }
                Err(_e) => {
                    // Fall through to the name-only default on any decode failure
                    // (faithful degradation; the recovery simply won't fire).
                }
            }
        }
        let mut model = ProtoModel::new(self.manage());
        model.set_name("unknown");
        // Build empty input/output param lists so `model.output()`/`input()` are
        // present (an empty list characterizes every range as `no_containment`,
        // so proto recovery declines gracefully rather than the model lacking
        // lists and panicking).  The real `ProtoModel::decode` always allocates
        // the lists via `buildParamList`; mirror that for the fallback default.
        let _ = model.build_param_list("standard");
        let rc = Rc::new(model);
        self.register_model(Rc::clone(&rc));
        self.defaultfp = Some(rc);
    }

    /// Record the compiler-spec (`.cspec`) XML content for the
    /// `<default_proto>` decode in [`build_default_proto`](Architecture::build_default_proto).
    /// The frontend reads the resolved `.cspec` file (the `compilerfile` path
    /// from `SleighArchitecture::build_spec_file`) and hands it here before
    /// [`init_post_engine`](Architecture::init_post_engine).
    pub fn set_cspec_xml(&mut self, xml: Vec<u8>) {
        self.cspec_xml = Some(xml);
    }

    /// Record the processor-spec (`.pspec`) XML content for the
    /// `<context_data>` decode in
    /// [`parse_processor_config`](Architecture::parse_processor_config).  The
    /// frontend reads the resolved `.pspec` file (the `processorfile` path from
    /// `SleighArchitecture::build_spec_file`) and hands it here before
    /// [`init_post_engine`](Architecture::init_post_engine).
    pub fn set_pspec_xml(&mut self, xml: Vec<u8>) {
        self.pspec_xml = Some(xml);
    }

    /// Apply the processor-spec `<context_data>` paints to the engine's context
    /// database (the relevant slice of C++ `Architecture::parseProcessorConfig`,
    /// architecture.cc:1176, dispatching the `ELEM_CONTEXT_DATA` branch to
    /// `context->decodeFromSpec(decoder)`).
    ///
    /// Without this the engine's context database is the all-zero `.sla`
    /// default, which for x86 selects 16-bit real mode (`addrsize`/`opsize`
    /// unset) regardless of the `x86:LE:64` archid — the pspec's
    /// `<context_set><set name="addrsize" val="2"/>…` is what tells SLEIGH to
    /// disassemble as 64-bit.
    ///
    /// SEAM(W6 pspec): the remaining `<processor_spec>` children (volatile,
    /// incidentalcopy, jumpassist, segmentop, …) decode with their own waves;
    /// this wires the `<context_data>` branch — the one that steers the
    /// disassembly mode and therefore gates every multi-byte lift — and the
    /// `<register_data>` branch (the `vector_lane_sizes` half), which seeds the
    /// `lanerecords` table that `ActionLaneDivide` reads to split XMM/ZMM vector
    /// lanes.  Faithful to `parseProcessorConfig`'s dispatch; the other branches
    /// are no-ops here (the C++ `peekElement` loop simply skips them in our
    /// `find_child` walk).
    pub fn parse_processor_config(&mut self) -> KunaResult<()> {
        use kuna_base::marshal::{IdRegistry, XmlDecode};
        use kuna_base::xml::DocumentStorage;
        use kuna_sleigh::globalcontext::register_globalcontext_ids;

        // C++ keeps the parsed pspec `DocumentStorage` for the whole
        // `restoreFromSpec`/`buildSymbols` window; the deferred `<default_symbols>`
        // apply (build_symbols, run after adjustCaches) re-reads it.  Clone (not
        // take) so the raw XML stays available for build_symbols.
        let Some(xml) = self.pspec_xml.clone() else {
            return Ok(());
        };
        let mut store = DocumentStorage::new();
        let root = store.parse_document(&xml)?.get_root().clone();
        // The C++ getTag("processor_spec") returns the <processor_spec> element;
        // the resolved .pspec file's root IS <processor_spec>.
        let pspec = if root.get_name() == "processor_spec" {
            root
        } else {
            match find_child(&root, "processor_spec") {
                Some(el) => el,
                None => return Ok(()), // no processor_spec: nothing to apply
            }
        };

        // C++ parseProcessorConfig dispatch — ELEM_REGISTER_DATA branch
        // (architecture.cc:1202 -> decodeRegisterData).  Seed the lanerecords
        // table before the action build reads getMinimumLanedRegisterSize.  A
        // pspec with no <register_data> (or only non-laned registers) leaves the
        // table empty, which is correct.
        if let Some(register_data) = find_child(&pspec, "register_data") {
            self.decode_register_data(&register_data)?;
        }

        // C++ parseProcessorConfig ELEM_VOLATILE branch (architecture.cc:1187 ->
        // decodeVolatile).  Paint each `<range>` in the `<volatile>` element with
        // the `volatil` boolean property so `ActionVarnodeProps` converts accesses
        // to those addresses into `read_volatile`/`write_volatile` user-ops (the
        // CALLOTHER form survives dead-code, which a plain COPY to an SFR-space
        // varnode does not).  Must run before the global-query snapshot is taken
        // (build_arch_handle) so the painted flagbase reaches the per-function seam.
        if let Some(volatile_el) = find_child(&pspec, "volatile") {
            self.decode_volatile(&volatile_el)?;
        }

        // C++ parseProcessorConfig ELEM_CONTEXT_DATA branch.  A pspec with no
        // <context_data> (e.g. a 32-bit-default processor) leaves the zero
        // context, which is correct for it.
        let Some(context_data) = find_child(&pspec, "context_data") else {
            return Ok(());
        };

        // Decode <context_data> against the engine's single address-space
        // manager (so `space="ram"` resolves to the real ram space).  The Rc
        // keeps the manager alive for the decoder while the context database
        // (a sibling RefCell on the engine) is borrowed mutably — no aliasing.
        let manager = self.translate.base().manager_rc();
        let mut registry = IdRegistry::with_base_ids();
        register_globalcontext_ids(&mut registry);
        let mut decoder = XmlDecode::new_with_root(&manager, &registry, &context_data, 0);
        self.translate
            .with_context_db_mut(|db| db.decode_from_spec(&mut decoder))?;
        Ok(())
    }

    /// Apply a `<volatile>` element, marking the contained `<range>` regions as
    /// holding volatile memory/registers (C++ `Architecture::decodeVolatile`,
    /// `architecture.cc:881`).
    ///
    /// The C++ `userops.decodeVolatile` half (reading `inputop`/`outputop` and
    /// registering the `VolatileReadOp`/`VolatileWriteOp` builtins with those
    /// names) is already satisfied: kuna pre-seeds `BUILTIN_VOLATILE_READ`/
    /// `BUILTIN_VOLATILE_WRITE` with the canonical `read_volatile`/`write_volatile`
    /// names (and the non-functional display = `annotation_assignment`/`no_operator`)
    /// in `register_string_builtins`, matching every vendored pspec's `<volatile
    /// outputop="write_volatile" inputop="read_volatile">`.  This method ports the
    /// range-painting half: for each `<range>` child,
    /// `symboltab->setPropertyRange(Varnode::volatil, range)`.
    fn decode_volatile(
        &mut self,
        volatile_el: &Rc<kuna_base::xml::Element>,
    ) -> KunaResult<()> {
        use crate::varnode::varnode_flags;
        use kuna_base::address::{Range, RangeProperties};
        use kuna_base::marshal::{IdRegistry, XmlDecode};

        let manager = self.translate.base().manager_rc();
        let registry = IdRegistry::with_base_ids();
        // C++ `decodeVolatile`: while peekElement() != 0 { Range r; r.decode(decoder);
        // symboltab->setPropertyRange(Varnode::volatil, r); }.  Each child is a
        // `<range>`; resolve it through `Range::from_properties` exactly as
        // `decode_global` does, then paint [first, lastOpen) with `volatil`.
        for child in volatile_el.get_children().iter() {
            if child.get_name() != "range" {
                continue;
            }
            let mut decoder = XmlDecode::new_with_root(&manager, &registry, child, 0);
            let mut props = RangeProperties::new();
            props.decode(&mut decoder)?;
            let range = Range::from_properties(&props, self.manage())?;
            let addr1 = range.get_first_addr();
            let addr2 = range.get_last_addr_open(self.manage());
            self.symboltab
                .set_property_range(varnode_flags::volatil, &addr1, &addr2);
        }
        Ok(())
    }

    /// Apply the pspec `<default_symbols>` element as named global symbols (C++
    /// `SleighArchitecture::buildSymbols`, `sleigh_arch.cc:265`).
    ///
    /// Each `<symbol name=… address=… [size=…] [volatile=…]>` is parsed into a
    /// global-scope symbol: the address via `parseAddressSimple` (with the C++
    /// `address="next"` continuation), the size defaulting to the space word size,
    /// the type `getBase(size, TYPE_UNKNOWN)`, and an optional `volatile` attribute
    /// re-painting the `volatil` property range.  This is what gives the 8051 SFR
    /// addresses their names (`P0`@SFR:80, `P1`@SFR:90), so an SFR write renders
    /// `P0 = 1` rather than `dat_80 = 1`.  Run after `adjust_caches` so the global
    /// scope's per-space maptable already covers every spec-created space.
    fn build_symbols(&mut self) -> KunaResult<()> {
        use crate::dtype::type_metatype::TYPE_UNKNOWN;
        use crate::varnode::varnode_flags;
        use kuna_base::address::{Address, Range};
        use kuna_base::xml::DocumentStorage;

        let Some(xml) = self.pspec_xml.clone() else {
            return Ok(());
        };
        let mut store = DocumentStorage::new();
        let root = store.parse_document(&xml)?.get_root().clone();
        let pspec = if root.get_name() == "processor_spec" {
            root
        } else {
            match find_child(&root, "processor_spec") {
                Some(el) => el,
                None => return Ok(()),
            }
        };
        let Some(symbols_el) = find_child(&pspec, "default_symbols") else {
            return Ok(());
        };
        let Some(scope) = self.symboltab.get_global_scope() else {
            return Ok(());
        };
        let usepoint = Address::new_invalid();

        // C++ `buildSymbols` tracks (lastAddr, lastSize) for the `address="next"`
        // continuation form.
        let mut last_addr = Address::new_invalid();
        let mut last_size: int4 = -1;
        for child in symbols_el.get_children().iter() {
            if child.get_name() != "symbol" {
                continue;
            }
            let name = match attr_str(child, "name") {
                Some(n) if !n.is_empty() => n,
                _ => return Err(KunaError::lowlevel(
                    "Missing name attribute in <symbol> element",
                )),
            };
            let addr_str = attr_str(child, "address").unwrap_or_default();
            let addr = if addr_str == "next" && last_size != -1 {
                &last_addr + (last_size as i64)
            } else {
                self.manage().parse_address_simple(&addr_str)?
            };
            if addr.is_invalid() {
                return Err(KunaError::lowlevel(
                    "Missing address attribute in <symbol> element",
                ));
            }
            // size defaults to the space word size (C++ addr.getSpace()->getWordSize()).
            let mut size = attr_str(child, "size")
                .and_then(|s| s.parse::<int4>().ok())
                .unwrap_or(0);
            if size == 0 {
                size = addr.get_space().map(|s| s.get_word_size() as int4).unwrap_or(1);
            }
            // Optional <symbol volatile="true|false"> re-paints the volatil property.
            if let Some(volstr) = attr_str(child, "volatile") {
                let volatile_state = matches!(volstr.as_str(), "true" | "1" | "yes");
                if let Some(spc) = addr.get_space() {
                    let range =
                        Range::new(Rc::clone(spc), addr.get_offset(), addr.get_offset() + (size as u64 - 1));
                    let a1 = range.get_first_addr();
                    let a2 = range.get_last_addr_open(self.manage());
                    if volatile_state {
                        self.symboltab.set_property_range(varnode_flags::volatil, &a1, &a2);
                    } else {
                        self.symboltab.clear_property_range(varnode_flags::volatil, &a1, &a2);
                    }
                }
            }
            let ct = self.types.get_base(size, TYPE_UNKNOWN)?;
            self.symboltab
                .add_symbol_mapped(scope, &name, ct, &addr, &usepoint)?;
            last_addr = addr;
            last_size = size;
        }
        Ok(())
    }

    /// Read `<register>` elements collecting the `vector_lane_sizes` lane
    /// schemes, building the `lanerecords` table (C++
    /// `Architecture::decodeRegisterData`, `architecture.cc:933`).
    ///
    /// Faithful to the C++ flow: for each `<register>` carrying
    /// `vector_lane_sizes`, the register storage *size* is resolved by name
    /// through the translator (the C++ `storage.decodeFromAttributes` -> the
    /// register lookup), `LanedRegister::parseSizes` builds the per-register lane
    /// mask, and the masks are accumulated by whole size in `maskList`.  One
    /// `LanedRegister(size, mask)` record is emitted per nonzero size, in
    /// ascending size order (the `maskList` is index-ordered by size), so the
    /// downstream binary searches are valid.
    ///
    /// The C++ also handles the `volatile` attribute (painting a volatile
    /// property range); that property subsystem is a separate seam and is not
    /// wired here — only the lane-size half is decoded.
    fn decode_register_data(
        &mut self,
        register_data: &Rc<kuna_base::xml::Element>,
    ) -> KunaResult<()> {
        use crate::transform::LanedRegister;

        // vector<uint4> maskList;  (indexed by register whole size in bytes)
        let mut mask_list: Vec<uint4> = Vec::new();
        for reg in register_data.get_children().iter() {
            if reg.get_name() != "register" {
                continue;
            }
            // string laneSizes; ... if (attribId == ATTRIB_VECTOR_LANE_SIZES) ...
            let Some(lane_sizes) = attr_str(reg, "vector_lane_sizes") else {
                continue; // no lane sizes (and volatile is a separate seam)
            };
            if lane_sizes.is_empty() {
                continue;
            }
            // storage.decodeFromAttributes(decoder): resolve the register's size
            // by name (the C++ VarnodeData decode reads name= -> getRegister).
            let Some(name) = attr_str(reg, "name") else {
                continue;
            };
            let storage = self.translate.get_register_varnode(name.as_bytes())?;
            let storage_size = storage.size as int4;
            // LanedRegister lanedRegister; lanedRegister.parseSizes(storage.size,laneSizes);
            let mut laned_register = LanedRegister::new();
            laned_register.parse_sizes(storage_size, &lane_sizes)?;
            // int4 sizeIndex = lanedRegister.getWholeSize();
            let size_index = laned_register.get_whole_size();
            // while (maskList.size() <= sizeIndex) maskList.push_back(0);
            while (mask_list.len() as int4) <= size_index {
                mask_list.push(0);
            }
            // maskList[sizeIndex] |= lanedRegister.getSizeBitMask();
            mask_list[size_index as usize] |= laned_register.get_size_bit_mask();
        }
        // lanerecords.clear();
        // for(i=0;i<maskList.size();++i) { if (maskList[i]==0) continue;
        //   lanerecords.push_back(LanedRegister(i,maskList[i])); }
        self.lanerecords.clear();
        for (i, &mask) in mask_list.iter().enumerate() {
            if mask == 0 {
                continue;
            }
            self.lanerecords.push(LanedRegister::with_mask(i as int4, mask));
        }
        Ok(())
    }

    /// Decode the `<default_proto><prototype>` element from cspec XML into a
    /// [`ProtoModel`] (the spec-driven subset of C++ `ProtoModel::decode`:
    /// `name`/`extrapop`/`strategy` attributes + the `<input>`/`<output>`
    /// `<pentry>` parameter lists).  General over any processor's cspec; the
    /// register/`<addr>` storage of each `<pentry>` is resolved through the
    /// engine `Translate`, exactly as `ParamEntry::decode` resolves `<addr>`.
    /// Decode the cspec's top-level `<returnaddress>` storage element into the
    /// `defaultReturnAddr` VarnodeData (C++ `Architecture::decodeReturnAddress`,
    /// architecture.cc:902 -> `VarnodeData::decode`).  The element wraps a single
    /// `<register>`/`<varnode>`/`<addr>` storage child; resolve it through the
    /// engine `Translate` exactly as the effect-block decode does.  Returns `None`
    /// when there is no `<returnaddress>` or it is empty (C++ leaves
    /// `defaultReturnAddr.space == 0`).
    fn decode_default_return_addr(&self, xml: &[u8]) -> Option<kuna_num::pcoderaw::VarnodeData> {
        use kuna_base::xml::DocumentStorage;
        let mut store = DocumentStorage::new();
        let root = store.parse_document(xml).ok()?.get_root().clone();
        let ra = find_child(&root, "returnaddress")?;
        for child in ra.get_children().iter() {
            match child.get_name() {
                "register" => {
                    let nm = attr_str(child, "name")?;
                    return self.translate.get_register_varnode(nm.as_bytes()).ok();
                }
                "varnode" | "addr" => {
                    let spname = attr_str(child, "space")?;
                    let space = self.manage().get_space_by_name(&spname)?.clone();
                    let offset =
                        attr_str(child, "offset").and_then(|s| parse_int(&s)).unwrap_or(0);
                    let size =
                        attr_str(child, "size").and_then(|s| s.parse::<u32>().ok()).unwrap_or(0);
                    return Some(kuna_num::pcoderaw::VarnodeData {
                        space: Some(space),
                        offset,
                        size,
                    });
                }
                _ => continue,
            }
        }
        None
    }

    fn decode_default_proto(&self, xml: &[u8]) -> KunaResult<ProtoModel> {
        use kuna_base::xml::DocumentStorage;
        let mut store = DocumentStorage::new();
        let root = store.parse_document(xml)?.get_root().clone();
        // Find <default_proto><prototype>.
        let dp = find_child(&root, "default_proto")
            .ok_or_else(|| KunaError::lowlevel("cspec has no <default_proto>"))?;
        let proto = find_child(&dp, "prototype")
            .ok_or_else(|| KunaError::lowlevel("<default_proto> has no <prototype>"))?;

        let mut model = ProtoModel::new(self.manage());
        let name = attr_str(&proto, "name").unwrap_or_else(|| "__stdcall".to_string());
        model.set_name(&name);
        // extrapop="unknown" -> EXTRAPOP_UNKNOWN; numeric otherwise.
        if let Some(ep) = attr_str(&proto, "extrapop") {
            if ep == "unknown" {
                model.set_extra_pop(crate::fspec::EXTRAPOP_UNKNOWN);
            } else if let Ok(v) = ep.parse::<int4>() {
                model.set_extra_pop(v);
            }
        }
        let strategy = attr_str(&proto, "strategy").unwrap_or_default();
        model.build_param_list(&strategy)?;

        // Decode <input>/<output> pentry lists and the <unaffected>/
        // <killedbycall>/<returnaddress> effect blocks.  C++ `ProtoModel::decode`
        // (fspec.cc, the `subId == ELEM_UNAFFECTED/KILLEDBYCALL/RETURNADDRESS`
        // arms) parses each block's `<register>`/`<addr>`/`<varnode>` children into
        // an `EffectRecord` with the matching type and appends it to `effectlist`.
        // This is the RSP keystone's root: without the `<unaffected>` RSP record,
        // `FuncProto::hasEffect(RSP)` returns `unknown_effect` instead of
        // `unaffected`, so heritage guards the stack pointer across every call and
        // the whole stack frame is skewed by the unmodeled extrapop.
        let mut saw_retaddr = false;
        for child in proto.get_children().iter() {
            match child.get_name() {
                "input" => self.decode_pentry_list(child, &mut model, true)?,
                "output" => self.decode_pentry_list(child, &mut model, false)?,
                // else if (subId == ELEM_UNAFFECTED) { ... effectlist.back().decode(unaffected) }
                "unaffected" => {
                    self.decode_effect_block(child, &mut model, crate::fspec::effect_type::UNAFFECTED)?;
                }
                // else if (subId == ELEM_KILLEDBYCALL) { ... decode(killedbycall) }
                "killedbycall" => {
                    self.decode_effect_block(child, &mut model, crate::fspec::effect_type::KILLEDBYCALL)?;
                }
                // else if (subId == ELEM_RETURNADDRESS) { ... decode(return_address); sawretaddr=true }
                "returnaddress" => {
                    self.decode_effect_block(child, &mut model, crate::fspec::effect_type::RETURN_ADDRESS)?;
                    saw_retaddr = true;
                }
                // else if (subId == ELEM_INTERNAL_STORAGE) { while peekElement: internalstorage.back().decode() }
                // (fspec.cc:2673) — registers (e.g. MIPS gp) the compiler may save to
                // the stack across a call; ActionInternalStorage unmaps their
                // eventual-constant spills so the value forwards across the call.
                "internal_storage" => {
                    self.decode_internal_storage_block(child, &mut model)?;
                }
                _ => {}
            }
        }
        // C++ fspec.cc: if (!sawretaddr && glb->defaultReturnAddr.space != 0)
        //   effectlist.push_back(EffectRecord(glb->defaultReturnAddr, return_address));
        // `glb->defaultReturnAddr` is decoded from the cspec's top-level
        // <returnaddress> (C++ Architecture::parseExtraRules / decode); parse that
        // root element directly here so the per-call retaddr store is modeled even
        // when the <prototype> omits its own <returnaddress> (the x86-64-gcc case).
        if !saw_retaddr {
            if let Some(ra_block) = find_child(&root, "returnaddress") {
                self.decode_effect_block(
                    &ra_block,
                    &mut model,
                    crate::fspec::effect_type::RETURN_ADDRESS,
                )?;
            }
        }
        Ok(model)
    }

    /// Decode one `<unaffected>`/`<killedbycall>`/`<returnaddress>` effect block
    /// (C++ `ProtoModel::decode`'s effect-block arms, fspec.cc): each child is a
    /// `<register>`/`<addr>`/`<varnode>` storage element decoded into an
    /// [`EffectRecord`] of the given `eff_type` and appended to the model's
    /// effect list.  Mirrors `decode_pentry_storage`'s storage resolution.
    fn decode_effect_block(
        &self,
        block: &Rc<kuna_base::xml::Element>,
        model: &mut ProtoModel,
        eff_type: u32,
    ) -> KunaResult<()> {
        for child in block.get_children().iter() {
            let vd = match child.get_name() {
                // <register name=".."/>  ->  getTrans()->getRegister(name)
                "register" => {
                    let nm = attr_str(child, "name")
                        .ok_or_else(|| KunaError::lowlevel("<register> has no name"))?;
                    self.translate.get_register_varnode(nm.as_bytes())?
                }
                // <varnode space=".." offset=".." size=".."/> or <addr .../>
                "varnode" | "addr" => {
                    let spname = attr_str(child, "space")
                        .ok_or_else(|| KunaError::lowlevel("<varnode> effect has no space"))?;
                    let space = self
                        .manage()
                        .get_space_by_name(&spname)
                        .ok_or_else(|| KunaError::lowlevel("<varnode> effect unknown space"))?
                        .clone();
                    let offset =
                        attr_str(child, "offset").and_then(|s| parse_int(&s)).unwrap_or(0);
                    let size = attr_str(child, "size")
                        .and_then(|s| s.parse::<u32>().ok())
                        .unwrap_or(0);
                    kuna_num::pcoderaw::VarnodeData { space: Some(space), offset, size }
                }
                _ => continue,
            };
            model.push_effect(crate::fspec::EffectRecord::from_varnode(vd, eff_type));
        }
        Ok(())
    }

    /// Decode an `<internal_storage>` block (C++ `ProtoModel::decode`,
    /// `fspec.cc:2673`): each `<register>`/`<varnode>`/`<addr>` child is a storage
    /// `VarnodeData` appended to the model's internal-storage list (sorted by
    /// `push_internal_storage`).  Same storage resolution as `decode_effect_block`.
    fn decode_internal_storage_block(
        &self,
        block: &Rc<kuna_base::xml::Element>,
        model: &mut ProtoModel,
    ) -> KunaResult<()> {
        for child in block.get_children().iter() {
            let vd = match child.get_name() {
                "register" => {
                    let nm = attr_str(child, "name")
                        .ok_or_else(|| KunaError::lowlevel("<register> has no name"))?;
                    self.translate.get_register_varnode(nm.as_bytes())?
                }
                "varnode" | "addr" => {
                    let spname = attr_str(child, "space")
                        .ok_or_else(|| KunaError::lowlevel("<varnode> internal_storage has no space"))?;
                    let space = self
                        .manage()
                        .get_space_by_name(&spname)
                        .ok_or_else(|| KunaError::lowlevel("<varnode> internal_storage unknown space"))?
                        .clone();
                    let offset =
                        attr_str(child, "offset").and_then(|s| parse_int(&s)).unwrap_or(0);
                    let size = attr_str(child, "size")
                        .and_then(|s| s.parse::<u32>().ok())
                        .unwrap_or(0);
                    kuna_num::pcoderaw::VarnodeData { space: Some(space), offset, size }
                }
                _ => continue,
            };
            model.push_internal_storage(vd);
        }
        Ok(())
    }

    /// Decode the `<pentry>`/`<group>` children of an `<input>`/`<output>`
    /// element into the model's input or output [`ParamListStandard`] (C++
    /// `ParamListStandard::decode`, `fspec.cc:1453`).  `is_input` selects the
    /// list.  Mirrors the `<pentry>`/`<group>` dispatch loop (`parsePentry`,
    /// `fspec.cc:1228`; `parseGroup`, `fspec.cc:1264`) + the `finish_decode` tail
    /// (resource boundary, `calcDelay`, `populateResolver`).
    fn decode_pentry_list(
        &self,
        list_el: &Rc<kuna_base::xml::Element>,
        model: &mut ProtoModel,
        is_input: bool,
    ) -> KunaResult<()> {
        // C++ ParamListStandard::decode: normalstack = !reverse; the model's
        // stackgrowsnegative drives it (the default cspec stack convention).
        let normalstack = true;
        // numgroup tracks the running group id, exactly as C++
        // `ParamListStandard::numgroup`.  Entries accumulate in `pentries`, which
        // doubles as the running prefix consulted by resolveFirst/resolveJoin/
        // resolveOverlap (the C++ passes its growing `entry` vector for the same
        // purpose).
        let mut numgroup: int4 = 0;
        let mut pentries: Vec<crate::fspec::ParamEntry> = Vec::new();
        for child in list_el.get_children().iter() {
            match child.get_name() {
                // C++ fspec.cc:1482-1484: a bare <pentry> is parsed at the current
                // numgroup with grouped == false.
                "pentry" => {
                    let entry = self.decode_pentry(child, numgroup, normalstack, false, &pentries)?;
                    // C++ parsePentry tail (fspec.cc:1251): numgroup advances past
                    // the entry's highest group (1 past for an exclusion entry).
                    let maxgroup = entry.get_all_groups().last().copied().unwrap_or(numgroup) + 1;
                    if maxgroup > numgroup {
                        numgroup = maxgroup;
                    }
                    pentries.push(entry);
                }
                // C++ fspec.cc:1485-1487 + parseGroup (fspec.cc:1264): every
                // <pentry> inside the <group> shares basegroup == numgroup and is
                // parsed with grouped == true.
                "group" => {
                    let basegroup = numgroup;
                    // C++ parseGroup keeps the two previous entries to enforce
                    // ParamEntry::orderWithinGroup pairwise (fspec.cc:1276-1282).
                    let mut prev1: Option<usize> = None;
                    let mut prev2: Option<usize> = None;
                    for gchild in child.get_children().iter() {
                        if gchild.get_name() != "pentry" {
                            // C++ parseGroup only ever peeks <pentry> elements
                            // inside <group>; ignore stray text/whitespace nodes.
                            continue;
                        }
                        let entry =
                            self.decode_pentry(gchild, basegroup, normalstack, true, &pentries)?;
                        if entry.get_space().get_type() == kuna_base::space::spacetype::IPTR_JOIN {
                            return Err(KunaError::lowlevel(
                                "<pentry> in the join space not allowed in <group> tag",
                            ));
                        }
                        let maxgroup =
                            entry.get_all_groups().last().copied().unwrap_or(basegroup) + 1;
                        if maxgroup > numgroup {
                            numgroup = maxgroup;
                        }
                        let cur = pentries.len();
                        pentries.push(entry);
                        // orderWithinGroup(previous1, cur) and (previous2, cur).
                        if let Some(p1) = prev1 {
                            crate::fspec::ParamEntry::order_within_group(&pentries[p1], &pentries[cur])?;
                            if let Some(p2) = prev2 {
                                crate::fspec::ParamEntry::order_within_group(
                                    &pentries[p2],
                                    &pentries[cur],
                                )?;
                            }
                        }
                        prev2 = prev1;
                        prev1 = Some(cur);
                    }
                }
                _ => {}
            }
        }
        // C++ ParamListStandard::decode (fspec.cc:1453): after the
        // `<pentry>`/`<group>` elements the loop reads any `<rule>` elements
        // (`modelRules.emplace_back(); modelRules.back().decode(decoder,this)`).
        // The rule decoders consult the populated resource (`getSpacebase`,
        // `getStackEntry`, `isBigEndian`), so the entries are pushed first; the
        // `<rule>` subtrees are then decoded against the live resource via an
        // `XmlDecode` rooted on each `<rule>` element (the modelrules ids are
        // registered on a fresh registry).
        let plist = if is_input { model.input_mut() } else { model.output_mut() };
        for e in pentries {
            plist.push_entry(e);
        }
        let rule_els: Vec<Rc<kuna_base::xml::Element>> = list_el
            .get_children()
            .iter()
            .filter(|c| c.get_name() == "rule")
            .cloned()
            .collect();
        if !rule_els.is_empty() {
            let manager = self.manage();
            let mut registry = kuna_base::marshal::IdRegistry::with_base_ids();
            crate::modelrules::register_ids(&mut registry);
            for rule_el in rule_els.iter() {
                let rule = {
                    let mut decoder = kuna_base::marshal::XmlDecode::new_with_root(
                        manager, &registry, rule_el, 0,
                    );
                    crate::modelrules::ModelRule::decode(&mut decoder, plist)?
                };
                plist.push_model_rule(rule);
            }
        }
        // C++ tail: resourceStart.push_back(numgroup); calcDelay();
        // populateResolver().
        plist.finish_decode();
        // C++ fspec.cc:1507-1512: if pointermax > 0, append a trailing
        // ConvertToPointer rule (a SizeRestrictedFilter(pointermax+1,0) feeding a
        // ConvertToPointer action).  `pointermax` is the `<input>`/`<output>`
        // element attribute (default 0 => no rule).
        if let Some(pmax) = attr_str(list_el, "pointermax").and_then(|s| s.parse::<int4>().ok()) {
            if pmax > 0 {
                plist.push_pointermax_rule(pmax);
            }
        }
        Ok(())
    }

    /// Decode one `<pentry>` element into a [`ParamEntry`] (C++
    /// `ParamEntry::decode`).  Reads `minsize`/`maxsize`/`align`/`storage`/
    /// `metatype`/`extension` attributes and the `<register>`/`<addr>` storage.
    fn decode_pentry(
        &self,
        pentry: &Rc<kuna_base::xml::Element>,
        group: int4,
        normalstack: bool,
        grouped: bool,
        prev: &[crate::fspec::ParamEntry],
    ) -> KunaResult<crate::fspec::ParamEntry> {
        use crate::dtype::{string2typeclass, type_class};
        use crate::fspec::param_entry_flags;
        let mut size: int4 = -1;
        let mut minsize: int4 = -1;
        let mut alignment: int4 = 0;
        let mut type_ = type_class::TYPECLASS_GENERAL;
        let mut flags: uint4 = 0;
        if let Some(v) = attr_str(pentry, "minsize") {
            minsize = v.parse().map_err(|_| KunaError::lowlevel("bad <pentry> minsize"))?;
        }
        if let Some(v) = attr_str(pentry, "maxsize") {
            size = v.parse().map_err(|_| KunaError::lowlevel("bad <pentry> maxsize"))?;
        }
        // size="..." (old) and align="..." (new) both set alignment.
        if let Some(v) = attr_str(pentry, "size") {
            alignment = v.parse().map_err(|_| KunaError::lowlevel("bad <pentry> size"))?;
        }
        if let Some(v) = attr_str(pentry, "align") {
            alignment = v.parse().map_err(|_| KunaError::lowlevel("bad <pentry> align"))?;
        }
        if let Some(v) = attr_str(pentry, "storage").or_else(|| attr_str(pentry, "metatype")) {
            type_ = string2typeclass(&v)?;
        }
        if let Some(ext) = attr_str(pentry, "extension") {
            flags &= !(param_entry_flags::SMALLSIZE_ZEXT
                | param_entry_flags::SMALLSIZE_SEXT
                | param_entry_flags::SMALLSIZE_INTTYPE);
            match ext.as_str() {
                "sign" => flags |= param_entry_flags::SMALLSIZE_SEXT,
                "zero" => flags |= param_entry_flags::SMALLSIZE_ZEXT,
                "inttype" => flags |= param_entry_flags::SMALLSIZE_INTTYPE,
                "float" => flags |= param_entry_flags::SMALLSIZE_FLOATEXT,
                "none" => {}
                _ => return Err(KunaError::lowlevel("Bad <pentry> extension attribute")),
            }
        }
        if size == -1 || minsize == -1 {
            return Err(KunaError::lowlevel("ParamEntry not fully specified"));
        }
        // Storage address: <register name=".."/> or <addr space=".." offset=".."/>.
        let (space, addressbase) = self.decode_pentry_storage(pentry)?;
        crate::fspec::ParamEntry::seed(
            group, type_, space, addressbase, size, minsize, alignment, flags, normalstack,
            grouped, prev, self.manage(),
        )
    }

    /// Resolve a `<pentry>`'s storage element to `(space, offset)` (C++
    /// `Address::decode` over `<register>`/`<addr>`).
    fn decode_pentry_storage(
        &self,
        pentry: &Rc<kuna_base::xml::Element>,
    ) -> KunaResult<(Rc<kuna_base::space::AddrSpace>, uintb)> {
        for child in pentry.get_children().iter() {
            match child.get_name() {
                "register" => {
                    let nm = attr_str(child, "name")
                        .ok_or_else(|| KunaError::lowlevel("<register> has no name"))?;
                    let vd = self.translate.get_register_varnode(nm.as_bytes())?;
                    let space = vd
                        .space
                        .ok_or_else(|| KunaError::lowlevel("register has no space"))?;
                    return Ok((space, vd.offset));
                }
                "addr" => {
                    let spname = attr_str(child, "space")
                        .ok_or_else(|| KunaError::lowlevel("<addr> has no space"))?;
                    let space = self
                        .manage()
                        .get_space_by_name(&spname)
                        .ok_or_else(|| KunaError::lowlevel("<addr> unknown space"))?
                        .clone();
                    // C++ `VarnodeData::decodeFromAttributes` (pcoderaw.cc:33) reads the
                    // `space` attribute, then dispatches `space->decodeAttributes(...)`.
                    // For the join space that is `JoinSpace::decodeAttributes`
                    // (space.cc:539): the `<addr space="join" piece1=".." piece2=".."/>`
                    // pentry must be resolved by joining its register pieces, not read
                    // as a plain offset.  Without this dispatch the x86 struct-return
                    // (`<addr space="join" piece1="EDX" piece2="EAX"/>`) output pentry
                    // decodes to offset 0 and `decode_default_proto` fails -> empty model.
                    if space.get_type() == kuna_base::space::spacetype::IPTR_JOIN {
                        let off = self.decode_join_addr(child)?;
                        return Ok((space, off));
                    }
                    let off = attr_str(child, "offset")
                        .and_then(|s| parse_int(&s))
                        .unwrap_or(0);
                    return Ok((space, off));
                }
                _ => {}
            }
        }
        Err(KunaError::lowlevel("<pentry> has no <register>/<addr> storage"))
    }

    /// Resolve a `<addr space="join" piece1=".." piece2=".."/>` element to the
    /// unified offset within the join space (C++ `JoinSpace::decodeAttributes`,
    /// space.cc:539).
    ///
    /// "piece1" corresponds to the most significant piece.  Each piece is either
    /// a register name (no `:` — `getTrans()->getRegister(attrVal)`) or a
    /// `space:offset:size` triple.  An optional `logicalsize` attribute carries
    /// the unified size for a single-piece (float) join.  `find_add_join`
    /// (space.rs:3014) constructs the logical address; we return its unified
    /// offset (the `addr` arm has already resolved the join `AddrSpace`).
    ///
    /// This walks the XML element's attributes directly (the proto decode runs
    /// over `xml::Element`s, not a `Decoder`), reproducing the C++
    /// `getNextAttributeId` / `getIndexedAttributeId(ATTRIB_PIECE)` loop: the
    /// legacy `pieceN` attribute name maps to `ATTRIB_PIECE` index `N-1`.
    fn decode_join_addr(&self, addr_el: &Rc<kuna_base::xml::Element>) -> KunaResult<uintb> {
        use kuna_base::space::VarnodeStorage;
        let mut pieces: Vec<VarnodeStorage> = Vec::new();
        let mut logicalsize: u32 = 0;
        // C++ accumulates `sizesum` but never reads it (kept for line parity).
        let mut _sizesum: u32 = 0;
        let nattr = addr_el.get_num_attributes();
        for i in 0..nattr {
            let name = addr_el.get_attribute_name(i);
            if name == "logicalsize" {
                let raw = String::from_utf8_lossy(addr_el.get_attribute_value_at(i)).into_owned();
                logicalsize = parse_int(&raw)
                    .ok_or_else(|| KunaError::lowlevel("bad join logicalsize"))?
                    as u32; // cast: uintb -> uint4 member (C++ readUnsignedInteger)
                continue;
            }
            // The legacy indexed attribute is named "piece1", "piece2", ...; its
            // ATTRIB_PIECE index is (N-1).  Non-`piece*` attributes (e.g.
            // `space`) are skipped, matching the C++ `attribId < ATTRIB_PIECE`
            // / non-piece branches.
            let pos: i32 = match name.strip_prefix("piece") {
                Some(rest) => match rest.parse::<i32>() {
                    Ok(n) if n >= 1 => n - 1,
                    _ => continue,
                },
                None => continue,
            };
            // C++ `if (pos > MAX_PIECES) continue;` (JoinSpace::MAX_PIECES = 64,
            // space.hh:233; the constant is `pub(crate)` to kuna-base, so the
            // literal is repeated here against the same source).
            if pos > 64 {
                continue;
            }
            while pieces.len() <= pos as usize {
                // cast: int4 index -> usize, non-negative here (pos >= 0)
                pieces.push(VarnodeStorage::default());
            }
            let attr_val = String::from_utf8_lossy(addr_el.get_attribute_value_at(i)).into_owned();
            let vdat: VarnodeStorage = match attr_val.find(':') {
                None => {
                    // Register-name piece: C++ `getTrans()->getRegister(attrVal)`.
                    let vd = self.translate.get_register_varnode(attr_val.as_bytes())?;
                    VarnodeStorage { space: vd.space, offset: vd.offset, size: vd.size }
                }
                Some(offpos) => {
                    let rest = &attr_val[offpos + 1..];
                    let szrel = rest
                        .find(':')
                        .ok_or_else(|| KunaError::lowlevel("join address piece attribute is malformed"))?;
                    let szpos = offpos + 1 + szrel;
                    let spcname = &attr_val[..offpos];
                    let space = self.manage().get_space_by_name(spcname).cloned();
                    let offset = parse_int(&attr_val[offpos + 1..szpos]).unwrap_or(0);
                    let size64 = parse_int(&attr_val[szpos + 1..]).unwrap_or(0);
                    // C++ extraction into a uint4 saturates on overflow.
                    let size = if size64 > u64::from(u32::MAX) {
                        u32::MAX
                    } else {
                        size64 as u32 // cast: checked above (uintb -> uint4)
                    };
                    VarnodeStorage { space, offset, size }
                }
            };
            _sizesum = _sizesum.wrapping_add(vdat.size);
            pieces[pos as usize] = vdat; // cast: int4 index -> usize, non-negative here
        }
        let rec = self.manage().find_add_join(&pieces, logicalsize)?;
        // C++ returns `rec->getUnified().offset` (and fills `size`, which the
        // caller `ParamEntry` derives from maxsize, not this).
        Ok(rec.get_unified().offset)
    }

    /// Build the universal Action tree + the "decompile" root (C++
    /// `Architecture::buildAction` -> `allacts.universalAction(this)` +
    /// `resetDefaults()`, architecture.cc:590).  The stack space (if any) is
    /// taken from the engine so the stack-aware passes are scheduled.
    pub fn build_action(&mut self) {
        let stackspace = self.manage().get_stack_space().cloned();
        let stackspace_index = stackspace.as_ref().map(|s| s.get_index());
        crate::universalaction::install_universal(
            &mut self.allacts,
            stackspace,
            stackspace_index,
            Vec::new(),
        );
        // C++ `Architecture::buildAction` runs `allacts.resetDefaults()`
        // (coreaction.cc `ActionDatabase::resetDefaults` -> `setCurrent(...)`),
        // which leaves the "decompile" root as the current action *before* any
        // function is decompiled.  The merged tree previously deferred the
        // `setCurrent` to the decompile drive, leaving `getCurrentName()` empty
        // at rest; that broke the `stage status`/`pipeline list (current)`
        // readers (kuna_console).  Set it here so the at-rest current name is
        // "decompile", matching upstream `resetDefaults`.
        let _ = self.allacts.set_current("decompile");
    }

    /// Register the p-code OpBehavior table (C++ `Architecture::buildInstructions`,
    /// architecture.cc:614 — `TypeOp::registerInstructions(inst,types,translate)`).
    ///
    /// Populates `glb->inst` from the ported `typeop::register_instructions`
    /// (the real `TypeOp::registerInstructions` table, indexed by op-code, with
    /// each op's property-flag word + name).  The flow/print classifiers read
    /// this through [`resolve_typeop`](Architecture::resolve_typeop).
    pub fn build_instructions(&mut self) {
        self.inst = crate::typeop::register_instructions();
        // Build the OpBehavior emulation table alongside the TypeOp metadata
        // (C++ `TypeOp::registerInstructions` attaches an `OpBehavior` to each
        // `TypeOp`; the Rust port keeps them as parallel tables).  The float
        // behaviors need a `FloatFormatProvider`; supply one that owns a clone of
        // the engine's float formats so the table is self-contained (the C++
        // passes the long-lived `Translate *`).
        let provider: Rc<dyn kuna_num::opbehavior::FloatFormatProvider> =
            Rc::new(OwnedFloatFormats::from_translate(&self.translate));
        let mut behaviors: Vec<Option<Rc<dyn kuna_num::opbehavior::OpBehavior>>> = Vec::new();
        kuna_num::opbehavior::register_instructions(&mut behaviors, &provider);
        self.opbehaviors = behaviors;
    }

    /// Resolve an op-code to its `TypeOp` property triple (C++ `glb->inst[opc]`).
    ///
    /// Reads the populated `inst` table; falls back to the canonical
    /// [`typeop::type_op_for`](crate::typeop::type_op_for) when the table is
    /// empty (the architecture was constructed but `build_instructions` has not
    /// run yet) so the flow engine always gets the right property flags.
    pub fn resolve_typeop(&self, opc: kuna_num::opcodes::OpCode) -> crate::seams::TypeOp {
        match self.inst.get(opc as usize).and_then(|o| o.as_ref()) {
            Some(info) => info.to_seam(),
            None => crate::typeop::type_op_for(opc),
        }
    }

    /// Resolve an op-code to its emulation [`OpBehavior`](kuna_num::opbehavior::OpBehavior)
    /// (C++ `op->getOpcode()->getBehavior()` — the behavior `glb->inst[opc]`
    /// carries).  Used by `EmulateFunction::set_current_op` for jump-table
    /// emulation.  Returns `None` for an opcode with no behavior installed.
    pub fn op_behavior(
        &self,
        opc: kuna_num::opcodes::OpCode,
    ) -> Option<Rc<dyn kuna_num::opbehavior::OpBehavior>> {
        self.opbehaviors.get(opc as usize).and_then(|o| o.clone())
    }

    /// Drive the post-engine init pipeline against an already-bootstrapped
    /// engine (the `Sleigh` decoded a `.sla` and the loader/context were set —
    /// the work the XML frontend `restoreFromSpec`/`buildTranslator` did).  This
    /// is the tail of C++ `Architecture::init` (architecture.cc:1395) from
    /// `buildTypegrp` onward, with the spec-file/translator build already done
    /// by the caller:
    ///
    /// ```text
    /// buildContext      (engine owns it — context_allow_set is the surface)
    /// buildTypegrp      -> build_typegrp
    /// buildDatabase     (done in `new`)
    /// buildCoreTypes    -> build_core_types
    /// parseCompilerConfig tail -> build_default_proto + finish_typegrp
    /// buildAction       -> build_action
    /// print->initializeFromArchitecture
    /// buildInstructions -> build_instructions
    /// ```
    ///
    /// The full XML spec decode (`parseProcessorConfig`/`parseCompilerConfig`
    /// reading the pspec/cspec tags) is the W6 cspec item; this wires the
    /// subsystem *construction* + ordering so a decoded engine becomes a
    /// decompilation-ready `Architecture`.
    pub fn init_post_engine(&mut self) -> KunaResult<()> {
        // C++ `Architecture::restoreFromSpec` (architecture.cc:636-640), right
        // after `copySpaces(newtrans)`: insert the analysis-only fspec/iop/join
        // spaces into the **single** engine manager (LOSS-132).  The engine's
        // `.sla` decode populated const/register/INTMEM/unique/ram; the C++
        // appends fspec, iop, join in that order onto the *same* manager, each
        // at `numSpaces()`.  In the Rust port the engine owns that one manager
        // (shared as `glb`), so we insert through it here.
        self.insert_ir_call_spaces()?;
        // C++ `Architecture::restoreFromSpec` calls `parseProcessorConfig`
        // (architecture.cc:645) before the type/action build.  Apply the pspec
        // `<context_data>` paints now so the engine's context database steers
        // disassembly correctly (e.g. x86-64 lifts as 64-bit, not 16-bit) —
        // the context must be in place before any instruction is decoded.
        self.parse_processor_config()?;
        // C++ `Architecture::restoreFromSpec` (architecture.cc:645) calls
        // `newtrans->setDefaultFloatFormats()` immediately after
        // `parseProcessorConfig` and before `parseCompilerConfig`: if the spec
        // registered no explicit `<float_format>` it installs the IEEE-754 4- and
        // 8-byte defaults so `getFloatFormat(4)`/`getFloatFormat(8)` resolve.
        // Without this the `PrintC::push_float` path (a `float8` constant literal)
        // has no FloatFormat and renders `FLOAT_UNKNOWN` instead of `1.123…`.
        self.translate.translate_base_mut().set_default_float_formats();
        // C++ `Architecture::restoreFromSpec` runs `parseCompilerConfig`
        // (architecture.cc:647) after `parseProcessorConfig`; the cspec
        // `<stackpointer>` element (parseCompilerConfig -> ELEM_STACKPOINTER ->
        // `decodeStackPointer`, architecture.cc:1260) creates the formal stack
        // `SpacebaseSpace`.  It must run before `finish_typegrp` (which reads
        // `get_stack_space()` for the stack-pointer size) and before
        // `build_default_proto` (the rest of the cspec decode).  Without it the
        // engine has no IPTR_SPACEBASE space, `s0x…` stack addresses fail to
        // parse, and `Funcdata.localmap` stays `None`.
        self.decode_stack_pointer()?;
        // C++ `parseCompilerConfig` dispatches the cspec `<funcptr>` element
        // (ELEM_FUNCPTR -> `decodeFuncPtrAlign`, architecture.cc:1048) to record
        // how many low bits of a function pointer are alignment-encoding (the ARM
        // Thumb LSB).  Decode it here alongside the other cspec children so the
        // GH-8471 `RulePtrsubUndo` thumb-funcptr guard can read `funcptr_align`.
        self.decode_funcptr_align()?;
        self.build_typegrp();
        // C++ `TypeFactory::TypeFactory` runs `setupSizes()` (the alignment map
        // + the core sizes) in the constructor, *before* `buildCoreTypes` calls
        // `setCoreType` (which queries the alignment map via `getAlignment`).
        // Mirror that ordering here: finish the size/alignment setup first.
        self.finish_typegrp();
        self.build_core_types()?;
        // C++ parseCompilerConfig dispatches each cspec child; the <callfixup>
        // elements register their injections into pcodeinjectlib.  Run this BEFORE
        // build_default_proto, which `take()`s the cspec XML.
        self.decode_call_fixups()?;
        // C++ restoreFromSpec: userops.initialize(this) (architecture.cc:641) +
        // the `<callotherfixup>` dispatch inside parseCompilerConfig
        // (architecture.cc:1294).  Run after the call-fixups are registered so
        // the whole inject library (callfixup + callotherfixup) is compiled
        // together by parseInject — the MIPS `setISAMode` fixup that makes the
        // dead ISA-mode-switch CALLOTHER injectable.
        self.init_userops_and_fixups()?;
        // (kuna) Eagerly register the string-copy builtins (C++ lazy
        // `userops.registerBuiltin` is called from
        // `ArraySequence::buildStringCopy` / `Funcdata::getInternalString` during
        // the `RuleStringStore`/`RuleStringCopy` transform).  Those transforms run
        // through the per-function W4 seam (`glb`), which carries no mutable
        // `userops` handle; the printer, however, reads back the builtin
        // name/display/typed-params on *this* real architecture (`opCallother` ->
        // `userops.getOp`).  The builtin set + their typed signatures are fixed,
        // so registering them once here (after the type factory is built) is
        // equivalent to the lazy C++ registration and keeps the printer self-
        // contained.  Idempotent (`register_builtin` is a no-op on a present id).
        self.register_string_builtins()?;
        // C++ `parseCompilerConfig` dispatches the cspec `<global>` element
        // (ELEM_GLOBAL, architecture.cc:1276-1277) into a deferred `globalRanges`
        // vector, then applies it via `addToGlobalScope` (architecture.cc:1336-1337)
        // AFTER `<stackpointer>`/`<spacebase>` are parsed (so all spaces exist).
        // Seed the global scope's rangetree here, after `decode_stack_pointer`
        // created the stack `SpacebaseSpace`, so an empty `<range space="ram"/>`
        // widens to the whole ram space and global RAM Varnodes pick up
        // `mapped|addrtied|persist`.  Must run before `adjust_caches` (which only
        // resizes per-scope maptables, not the rangetree) — ordering matches C++.
        self.decode_global()?;
        self.build_default_proto();
        // Share `defaultfp` + the engine address-space manager into the type
        // factory so the C-declaration grammar's nested function-pointer
        // `buildType` path (`FunctionModifier::modType` -> `getTypeCode(
        // PrototypePieces)` -> `TypeCode::setPrototype`) can run.  The C++
        // `TypeFactory` reaches both through its `Architecture *glb`; the kuna
        // factory is standalone, so the link is established once here, right
        // after `defaultfp` is finalized.
        self.types
            .set_proto_context(self.defaultfp.clone(), self.translate.manager_rc());
        self.build_action();
        self.print.initialize_from_architecture();
        // C++ `symboltab->adjustCaches()` (architecture.cc, end of restoreFromSpec)
        // resizes every scope's per-space `maptable` to `numSpaces()` after the
        // spec decode created new spaces.  The global scope was attached with the
        // engine's space count *before* `insert_ir_call_spaces` (fspec/iop/join)
        // and `decode_stack_pointer` (the stack `SpacebaseSpace`) appended their
        // spaces — so the maptable must now grow, or a `map addr s0x…` into the
        // higher-indexed stack space indexes past its end.
        let num_spaces = self.manage().num_spaces();
        self.symboltab.adjust_caches(num_spaces);
        // C++ `Architecture::buildSymbols(store)` (architecture.cc:1408), right
        // after `adjustCaches` and before `postSpecFile`/`buildInstructions`:
        // apply the pspec `<default_symbols>` (e.g. the 8051 SFR names `P0`@SFR:80,
        // `P1`@SFR:90) as named global symbols.  Without this an SFR write renders
        // `dat_80 = 1` instead of `P0 = 1`.
        self.build_symbols()?;
        self.build_instructions();
        // C++ `min_funcsymbol_size = translate->getAlignment()` when <= 8
        // (restoreFromSpec, architecture.cc:646).
        let align = self.translate.get_alignment();
        if align <= 8 {
            self.min_funcsymbol_size = align;
        }
        // C++ `Architecture::postSpecFile()` (architecture.cc:620-624), called once
        // the whole spec is restored: `cacheAddrSpaceProperties()`.  Run last, after
        // `decode_global` pushed the cspec `<global>` spaces and every analysis
        // space (fspec/iop/join/stack) exists, so the sort/dedup/filter sees the
        // final space set and the default data space (`ram`) leads `inferPtrSpaces`.
        self.cache_addr_space_properties();
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// ArchOptionContext — wire the `option NAME VALUE` command (the most-used
// datatest command) into the real Architecture / printer / type factory.
// (w9x-arch-engine-glue, item #2)
//
// Each method is the `glb->…` body the matching `ArchOption::apply`
// (options.cc) reaches; the `// SEAM(...)` markers in the options.rs trait doc
// are now wired to the real subsystems this `Architecture` owns.
// ---------------------------------------------------------------------------

impl ArchOptionContext for Architecture {
    // --- plain config fields ----------------------------------------------
    fn set_readonly_propagate(&mut self, val: bool) {
        self.readonlypropagate = val;
    }
    fn set_infer_pointers(&mut self, val: bool) {
        self.infer_pointers = val;
    }
    fn set_analyze_for_loops(&mut self, val: bool) {
        self.analyze_for_loops = val;
    }
    fn set_max_jumptable_size(&mut self, val: uint4) {
        self.max_jumptable_size = val;
    }
    fn set_max_instructions(&mut self, val: int4) {
        self.max_instructions = val as uint4;
    }
    fn alias_block_level(&self) -> int4 {
        self.alias_block_level
    }
    fn set_alias_block_level(&mut self, val: int4) {
        self.alias_block_level = val;
    }

    // --- flow option flags -------------------------------------------------
    fn flow_options(&self) -> uint4 {
        self.flowoptions
    }
    fn set_flow_options(&mut self, val: uint4) {
        self.flowoptions = val;
    }

    // --- split-datatype config --------------------------------------------
    fn split_datatype_config(&self) -> uint4 {
        self.split_datatype_config
    }
    fn set_split_datatype_config(&mut self, val: uint4) {
        self.split_datatype_config = val;
    }

    // --- nan-ignore config -------------------------------------------------
    fn nan_ignore_all(&self) -> bool {
        self.nan_ignore_all
    }
    fn set_nan_ignore_all(&mut self, val: bool) {
        self.nan_ignore_all = val;
    }
    fn nan_ignore_compare(&self) -> bool {
        self.nan_ignore_compare
    }
    fn set_nan_ignore_compare(&mut self, val: bool) {
        self.nan_ignore_compare = val;
    }

    // --- prototype models (C++ defaultfp / evalfp_current) -----------------
    fn set_default_extra_pop(&mut self, expop: int4) {
        // C++ `glb->defaultfp->setExtraPop(expop)` (+ eval-model spreads).
        // The registry holds the model behind `Rc`; mutate the shared model
        // (and keep the registry entry pointing at the same data via the same
        // `Rc`).  Both `defaultfp` and the registry entry are the one `Rc`.
        if let Some(fp) = self.defaultfp.as_mut() {
            Rc::make_mut(fp).set_extra_pop(expop);
            // Re-publish the (now-distinct) Rc into the registry so getModel
            // and defaultfp stay the same object (C++ shared-pointer identity).
            let name = fp.get_name().to_string();
            self.proto_models.insert(name, Rc::clone(fp));
        }
    }
    fn set_function_extra_pop(&mut self, name: &str, _expop: int4) -> KunaResult<()> {
        // C++ looks up `symboltab->getGlobalScope()->queryFunction(name)` then
        // `fd->getFuncProto().setExtraPop(expop)`.  The per-function FuncProto
        // mutation needs a resolved Funcdata; the symbol-table function query +
        // FuncProto write is the W4 symboltab + W6 fspec surface.
        // SEAM(W4 symboltab + W6 fspec): no function is resolvable here yet.
        Err(KunaError::recov(format!("Unknown function name: {name}")))
    }
    fn set_default_model(&mut self, name: &str) -> KunaResult<()> {
        // C++ `glb->setDefaultModel(getModel(p1))`.
        match self.proto_models.get(name).cloned() {
            Some(model) => {
                self.defaultfp = Some(model);
                Ok(())
            }
            None => Err(KunaError::lowlevel(format!("Unknown prototype model :{name}"))),
        }
    }
    fn set_eval_current_model(&mut self, name: &str) -> KunaResult<()> {
        // C++ `glb->evalfp_current = getModel(p1)`.
        match self.proto_models.get(name).cloned() {
            Some(model) => {
                self.evalfp_current = Some(model);
                Ok(())
            }
            None => Err(KunaError::parse(format!("Unknown prototype model: {name}"))),
        }
    }

    // --- per-function properties (C++ OptionInline / OptionNoReturn) -------
    fn set_function_inline(&mut self, name: &str, val: bool) -> KunaResult<()> {
        // C++ `OptionInline::apply`: `infd = symboltab->getGlobalScope()->queryFunction(p1);
        // if (infd==0) throw RecovError("Unknown function name: "+p1); infd->getFuncProto().setInline(val)`.
        // The FunctionSymbol's lazily-built Funcdata/FuncProto is W5; the inline flag
        // is parked on the symbol (read back by FlowInfo::queryCall at flow time).
        let sid = self.query_global_function(name)?;
        self.symboltab.set_function_inline(sid, val);
        Ok(())
    }
    fn set_function_no_return(&mut self, name: &str, val: bool) -> KunaResult<()> {
        // C++ `OptionNoReturn::apply`: same shape as OptionInline, but setNoReturn.
        let sid = self.query_global_function(name)?;
        self.symboltab.set_function_no_return(sid, val);
        Ok(())
    }

    // --- printer (wired to the owned PrintC) -------------------------------
    fn print_is_c_language(&self) -> bool {
        self.print.get_name() == "c-language"
    }
    fn set_null_printing(&mut self, val: bool) {
        self.print.set_null_printing(val);
    }
    fn set_inplace_ops(&mut self, val: bool) {
        self.print.set_inplace_ops(val);
    }
    fn set_convention_printing(&mut self, val: bool) {
        self.print.set_convention_printing(val);
    }
    fn set_no_cast_printing(&mut self, val: bool) {
        self.print.set_no_cast_printing(val);
    }
    fn set_hide_implied_exts(&mut self, val: bool) {
        self.print.set_hide_implied_exts(val);
    }
    fn set_max_line_size(&mut self, val: int4) {
        // C++ throws on a bad range; the Rust setter returns a Result.  The
        // option apply already validated the parse; ignore the (always-Ok)
        // no-markup result.
        let _ = self.print.set_max_line_size(val);
    }
    fn set_indent_increment(&mut self, val: int4) {
        self.print.set_indent_increment(val);
    }
    fn set_line_comment_indent(&mut self, val: int4) {
        let _ = self.print.set_line_comment_indent(val);
    }
    fn set_comment_style(&mut self, style: &str) {
        self.print.set_comment_style(style);
    }
    fn header_comment_flags(&self) -> uint4 {
        self.print.header_comment_flags()
    }
    fn set_header_comment_flags(&mut self, flags: uint4) {
        self.print.set_header_comment_flags(flags);
    }
    fn instruction_comment_flags(&self) -> uint4 {
        self.print.instruction_comment_flags()
    }
    fn set_instruction_comment_flags(&mut self, flags: uint4) {
        self.print.set_instruction_comment_flags(flags);
    }
    fn set_integer_format(&mut self, fmt: &str) {
        let _ = self.print.set_integer_format(fmt);
    }
    fn set_namespace_strategy(&mut self, strategy: NamespaceStrategy) {
        self.print.set_namespace_strategy(strategy);
    }
    fn set_brace_format(&mut self, category: BraceCategory, style: crate::options::BraceStyle) {
        self.print.set_brace_format(category, style);
    }
    fn set_print_language(&mut self, language: &str) {
        // C++ `glb->setPrintLanguage(p1)` swaps the active PrintLanguage; the
        // single owned printer records the requested name (the only datatest
        // language is "c-language").
        self.print.set_name(language);
    }

    // --- action database ---------------------------------------------------
    fn set_action_warning(&mut self, val: bool, name: &str) -> bool {
        // C++ `glb->allacts.getCurrent()->setWarning(val,p1)`.
        match self.allacts.get_current_mut() {
            Some(act) => act.set_warning(val, name),
            None => false,
        }
    }
    fn clone_action_group(&mut self, from: &str, to: &str) {
        // C++ `glb->allacts.cloneGroup(p1,p2); setCurrent(p2)`.
        if self.allacts.clone_group(from, to.to_string()).is_ok() {
            let _ = self.allacts.set_current(to);
        }
    }
    fn set_current_action(&mut self, name: &str) {
        let _ = self.allacts.set_current(name);
    }
    fn current_action_name(&self) -> String {
        self.allacts.get_current_name().to_string()
    }
    fn toggle_action(&mut self, _group: &str, _sub: &str, _val: bool) {
        // C++ `glb->allacts.toggleAction(grp,sub,val)`.
        // SEAM(W5): `ActionDatabase::toggleAction` (action.cc:1036) is not yet
        // ported onto the Rust `ActionDatabase`; the plain `option NAME VALUE`
        // path (the most-used datatest command) does not reach it — only the
        // `setaction GROUP SUB on/off` form does.  Recorded as a loss.
    }
    fn enable_rule(&mut self, path: &str) -> bool {
        match self.allacts.get_current_mut() {
            Some(act) => act.enable_rule(path),
            None => false,
        }
    }
    fn disable_rule(&mut self, path: &str) -> bool {
        match self.allacts.get_current_mut() {
            Some(act) => act.disable_rule(path),
            None => false,
        }
    }
    fn has_current_action(&self) -> bool {
        self.allacts.get_current().is_some()
    }

    // --- translator (engine-owned context) ---------------------------------
    fn allow_context_set(&mut self, val: bool) {
        // C++ `glb->translate->allowContextSet(val)`.
        self.translate.allow_context_set(val);
    }
}

// ---------------------------------------------------------------------------
// InjectArchitecture / UseropArchitecture (the `Architecture *glb` slice the
// userop decode + inject-library decode reach — userop.cc:86-99 / 368-637).
// Wires `userops.initialize` + `<callotherfixup>` decode at boot.
// ---------------------------------------------------------------------------

impl crate::pcodeinject::InjectArchitecture for Architecture {
    fn get_default_code_space(&self) -> Rc<AddrSpace> {
        // C++ `glb->getDefaultCodeSpace()`.
        Rc::clone(self.manage().get_default_code_space().expect("no default code space"))
    }
    fn get_unique_space(&self) -> Rc<AddrSpace> {
        // C++ `glb->getUniqueSpace()`.
        Rc::clone(self.manage().get_unique_space().expect("no unique space"))
    }
}

impl crate::userop::UseropArchitecture for Architecture {
    fn get_user_op_names(&self) -> Vec<Vec<u8>> {
        // C++ `glb->translate->getUserOpNames(res)`.  The Sleigh translate hands
        // back display strings; convert to the byte-string form the manager keys.
        let mut res: Vec<String> = Vec::new();
        kuna_sleigh::translate::Translate::get_user_op_names(&self.translate, &mut res);
        res.into_iter().map(String::into_bytes).collect()
    }

    fn decode_inject(
        &mut self,
        src: &[u8],
        suffix: &[u8],
        tp: int4,
        decoder: &mut dyn kuna_base::marshal::Decoder,
    ) -> KunaResult<int4> {
        // C++ `glb->pcodeinjectlib->decodeInject(src,suffix,tp,decoder)`.
        self.pcodeinjectlib.decode_inject(src, suffix, tp, decoder)
    }

    fn get_call_other_target(&self, injectid: int4) -> Vec<u8> {
        // C++ `glb->pcodeinjectlib->getCallOtherTarget(injectid)`.
        self.pcodeinjectlib.base.get_call_other_target(injectid)
    }

    fn payload_io_sizes(&self, injectid: int4) -> KunaResult<(int4, int4, int4, int4)> {
        // C++ `SegmentOp::decode` reads payload->sizeOutput/sizeInput plus the
        // first two input sizes after the `<pcode>` child is parsed.
        let core = self.pcodeinjectlib.get_payload(injectid).core();
        let size_output = core.size_output();
        let size_input = core.size_input();
        // get_size() is a uint4 (the InjectParameter size); narrow to int4 the
        // same way the C++ reads `getInput(k).getSize()` into an int4.
        let in0 = if size_input > 0 { core.get_input(0).get_size() as int4 } else { 0 };
        let in1 = if size_input > 1 { core.get_input(1).get_size() as int4 } else { 0 };
        Ok((size_output, size_input, in0, in1))
    }
}

/// A self-contained [`FloatFormatProvider`](kuna_num::opbehavior::FloatFormatProvider)
/// owning a clone of the engine's float formats.
///
/// The C++ `TypeOp::registerInstructions` passes the long-lived `Translate *`;
/// the Rust float behaviors store an `Rc<dyn FloatFormatProvider>` (kuna-num
/// opbehavior module docs).  The behavior table outlives any single borrow of
/// `translate`, so this provider clones the formats by value and serves
/// references to its own copies (the formats are immutable engine config).
struct OwnedFloatFormats {
    formats: Vec<kuna_num::float::FloatFormat>,
}

impl OwnedFloatFormats {
    /// Clone the engine's float formats for the standard p-code encoding sizes
    /// (the C++ candidates: 2/4/8/10/16-byte IEEE formats; the engine returns
    /// only those it actually defines).
    fn from_translate(translate: &Sleigh) -> Self {
        use kuna_sleigh::translate::Translate;
        let mut formats = Vec::new();
        for size in [2, 4, 8, 10, 16] {
            if let Some(fmt) = translate.get_float_format(size) {
                formats.push(fmt.clone());
            }
        }
        OwnedFloatFormats { formats }
    }
}

impl kuna_num::opbehavior::FloatFormatProvider for OwnedFloatFormats {
    fn get_float_format(&self, size: i32) -> Option<&kuna_num::float::FloatFormat> {
        self.formats.iter().find(|f| f.get_size() == size)
    }
}

/// (kuna) Adapter implementing [`UseropTypeArchitecture`] over the architecture's
/// populated [`TypeFactoryImpl`], used by [`Architecture::register_string_builtins`]
/// to build the typed builtin signatures without aliasing the `&mut userops`
/// borrow.  Maps the trait's `glb->types->...` / `glb->getDefaultDataSpace()`
/// reads onto the factory + the data-space word size captured at construction.
struct BuiltinTypeArch {
    types: Rc<TypeFactoryImpl>,
    data_word_size: int4,
}

impl crate::userop::UseropTypeArchitecture for BuiltinTypeArch {
    fn get_size_of_pointer(&self) -> int4 {
        self.types.get_size_of_pointer()
    }
    fn get_default_data_space_word_size(&self) -> int4 {
        self.data_word_size
    }
    fn get_type_void(&self) -> Rc<crate::dtype::Datatype> {
        self.types.get_type_void().expect("builtin: getTypeVoid")
    }
    fn get_type_pointer(
        &self,
        ptr_size: int4,
        base: Rc<crate::dtype::Datatype>,
        word_size: int4,
    ) -> Rc<crate::dtype::Datatype> {
        self.types
            .get_type_pointer(ptr_size, base, word_size as uint4)
            .expect("builtin: getTypePointer")
    }
    fn get_base_int(&self, size: int4) -> Rc<crate::dtype::Datatype> {
        self.types.get_base(size, type_metatype::TYPE_INT).expect("builtin: getBase(INT)")
    }
    fn get_type_char(&self) -> Rc<crate::dtype::Datatype> {
        self.types
            .get_type_char(self.types.get_size_of_char())
            .expect("builtin: getTypeChar")
    }
    fn get_type_wchar(&self) -> Rc<crate::dtype::Datatype> {
        self.types
            .get_type_char(self.types.get_size_of_wchar())
            .expect("builtin: getTypeChar(wchar)")
    }
}

#[cfg(test)]
mod tests;

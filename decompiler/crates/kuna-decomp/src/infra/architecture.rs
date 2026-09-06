//! Port of `decompiler/cpp/architecture.{cc,hh}` (item `w4-fw-architecture`) —
//! the [`Architecture`] god object: the \e owner of the disassembly engine
//! (`Translate`/Sleigh), the symbol [`Database`], the [`OptionDatabase`], the
//! [`ActionDatabase`], the [`UserOpManage`], the p-code injection library, the
//! [`ContextDatabase`], plus the protection/read-only flags and the whole bag of
//! analysis-tuning configuration values.
//!
//! ## What this port wires vs. what it stubs
//!
//! The C++ `Architecture` is the single largest class in the decompiler and
//! reaches into nearly every subsystem.  This port faithfully ports the parts
//! whose dependencies already exist in the kuna Rust tree, and stub-notes the
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
//!   construction of a [`Funcdata`] tied to this architecture (the W3 boot boundary:
//!   `vbank`'s analysis unique-start comes from `Translate::getUniqueStart`).
//!
//! - **Stubbed**: the data-type factory ([`crate::dtype`], W6), the prototype
//!   models (`fspec`, W6), the print language (W8), the loader (`loadimage`, its
//!   own item), the read-only/volatile/global-range decode (needs the W6 type
//!   factory + W4 symbol markup), and the full [`Architecture::init`] /
//!   `restoreFromSpec` flow (it builds the translator, type group, print
//!   language, and runs the spec decode — all reaching W6/W8 subsystems).  The
//!   `restoreXml`/`encode` marshaling and the segmented-pointer resolver are
//!   likewise deferred to their dependency waves.  Each is documented inline with
//!   `// STUB(...)`.
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
use kuna_sleigh::translate::UniqueLayout;

use crate::action::ActionDatabase;
use crate::engine_translate::EngineTranslate;
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
use crate::context::{ArchHandle, ArchContext};
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

/// Value of a boolean spec attribute (C++ `Decoder::readBool`, which accepts
/// `true`/`yes`/`1` and treats everything else as false).
fn decode_bool_attr(v: &str) -> bool {
    matches!(v, "true" | "yes" | "1")
}

/// Build a named copy of an already-registered model (C++
/// `Architecture::createModelAlias`, architecture.cc:1137).  A merged model and
/// an alias-of-an-alias are both refused, exactly as upstream.
fn create_model_alias(alias: &str, parent: &Rc<ProtoModel>) -> KunaResult<ProtoModel> {
    if parent.is_merged() {
        return Err(KunaError::lowlevel(format!(
            "Cannot make alias of merged model: {}",
            parent.get_name()
        )));
    }
    if parent.get_alias_parent().is_some() {
        return Err(KunaError::lowlevel(format!(
            "Cannot make alias of an alias: {}",
            parent.get_name()
        )));
    }
    Ok(crate::fspec::ProtoModel::copy_named(alias, parent))
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
/// STUB(comment.cc): the full `CommentDatabase` (ordered set, de-duplication,
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
/// STUB(comment.cc): `decompiler/cpp/comment.{cc,hh}` is a separate port item.
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
/// lightweight [`ArchContext`] handle carrying just the address-space slice it
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
    /// (kuna `dynrelocs`) `[start, stop]` (inclusive) offsets whose read-only
    /// contents fold even with [`Self::readonlypropagate`] off — the loader's
    /// `PT_GNU_RELRO`-frozen dynamic-relocation slots
    /// (`ObjectLoadImage::dynreloc_const_ranges`). Installed by the ELF bootstrap
    /// right after the read-only ranges it already collects, and carried into
    /// every per-function handle by `build_arch_handle`. Empty on every other
    /// path, so it is inert for the XML datatest oracle.
    pub dynreloc_const: Rc<Vec<(u64, u64)>>,
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
    /// (kuna `rodatastring`) Recover a read-only string block copy as
    /// `builtin_strncpy`.
    pub rodata_string: bool,
    /// (kuna `ptrdepthcap`) Cap the pointer nesting `ActionInferTypes` will adopt.
    ///
    /// A small-string-optimized C++ object writes the unsatisfiable equation
    /// `T == ptr(T)` into the type lattice (`p = &obj` on one MULTIEQUAL edge,
    /// `p = obj.ptr` -- a LOAD from the same address -- on the other), so the
    /// propagation adds one pointer level per pass until the seven-pass ceiling
    /// and declares the object `unsigned long long *****`.  When set, every
    /// candidate type is put through `kuna_ptrdepth::cap_pointer_depth`, which is
    /// upstream `TypeFactory::getTypePointerNoDepth`'s rule (`type.cc:1509`)
    /// applied at the propagation funnel rather than only at LOAD/STORE.
    pub ptrdepthcap: bool,
    /// (kuna GH-8913) Fuse 8-bit carry-chain 16-bit adds into one wide add
    /// (C++ `add_carry_chain`).
    pub add_carry_chain: bool,
    /// (kuna GH-8817) Reclassify V850 `jmp [reg]` CALLIND to BRANCHIND
    /// (C++ `v850_indirect_branch`).
    pub v850_indirect_branch: bool,
    /// (kuna `fastfailnoreturn`) End the flow at a Windows `int 0x29`
    /// (`__fastfail`).  x86 SLEIGH lifts `INT imm8` to a `call` with no matching
    /// push, so the cspec's `extrapop` raises the stack pointer by 8 at every
    /// site; `__fastfail` never returns, so the block ends there instead.
    /// Default on (`option fastfailnoreturn`).  See
    /// [`kuna_fastfailnoreturn`](crate::kuna_fastfailnoreturn).
    pub fastfail_noreturn: bool,
    /// (kuna `msvcftol`) Install the synthesized MSVC `__ftol`-family call-fixup
    /// (`p2_lift/kuna_msvcftol.rs`) so an x86-32 float-to-integer CRT helper call
    /// lowers to a p-code truncation and its x87 (`ST0`) argument survives.
    /// Default on; read by the analysis-tier call-fixup installer, which drops
    /// this one fixup's targets from the install map when off.
    pub msvc_ftol: bool,
    /// (kuna tee-O2 tail-jumps) Recover a direct `jmp` to another function's
    /// entry (e.g. `jmp setlocale@plt`) as a tail call (CALL + RETURN) instead of
    /// flowing into the callee (`option tailcalljump`, default off).
    pub tail_call_jumps: bool,
    /// (kuna `tailcallframe`) Recover a direct `jmp` whose target is NOT a known
    /// function as a tail call when the instructions immediately before it tear
    /// down exactly the frame the entry block built — the callback-only callee no
    /// discovery oracle reaches (`option tailcallframe`).  See
    /// [`crate::p2_lift::kuna_tailcallframe`].
    pub tail_call_frame: bool,
    /// (kuna `cleanupcode`) Delete the Rust drop/deallocate call sites (the
    /// `core::ptr::drop_in_place` / `Drop::drop` / `RawVecInner::deallocate` /
    /// `__rust_dealloc` family) from the pre-SSA op graph, so the drop glue and
    /// the argument setup that only feeds it never reach the output
    /// (`option cleanupcode`).  Structurally inert on a C binary — no C ELF
    /// resolves a call to one of those names.  See
    /// [`crate::p2_lift::kuna_cleanupcode`].
    pub remove_cleanup_code: bool,
    /// (kuna `linuxsyscall`) Rewrite a 32-bit Linux `int 0x80` from the indirect
    /// call through the `swi` userop that x86 SLEIGH lowers it to into a named
    /// call on the syscall the constant in `EAX` selects, with the ABI argument
    /// registers as its arguments (`option linuxsyscall`).  x86-32 only, and
    /// declined whenever the number is not a locally-resolvable constant with a
    /// vetted table entry.  See [`crate::p2_lift::kuna_linuxsyscall`].
    pub linux_syscall: bool,
    /// (kuna `switchselector`) Refuse to install a recovered lowered-switch
    /// cascade whose synthesized BRANCHIND cannot be given the switch value as
    /// its selector, so the compiler's `if`/`else if` chain over the real
    /// variable survives instead of a `switch` over something else
    /// (`option switchselector`).  See
    /// [`crate::p2_lift::kuna_loweredswitch::install_selector_is_sound`].
    pub switch_selector_guard: bool,
    /// (kuna `funcboundflow`) Bound fall-through at a known function entry: when a
    /// fall-through reaches the entry of another known function (the callee just
    /// executed being an unnamed static no-return the analysis could not prove
    /// no-return), truncate flow with a no-return halt instead of decoding the
    /// next function's body into the current one (`option funcboundflow`).
    pub funcbound_flow: bool,
    /// (kuna `overlapbranch`) Truncate a conditional branch's fall-through when the
    /// branch's own target lies strictly inside that fall-through instruction's
    /// encoding — the anti-disassembly junk-lead-byte idiom, where the fall-through
    /// decode swallows the real instruction boundary and desynchronises the stream
    /// (`option overlapbranch`).
    pub overlap_branch: bool,
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
    /// (kuna) Refuse to split a shared RETURN block that stores to GLOBALS, so
    /// a 72-store epilogue is not cloned into each predecessor (option
    /// `retsplitglobal`).  See [`crate::p8_structure::kuna_retsplitglobal`].
    pub ret_split_global: bool,
    /// (kuna) Resolve a one-byte lane read of a CONSTANT-mask `pshufb` shuffle
    /// to the source lane it selects (option `simdlane`).  See
    /// [`crate::p3_dataflow::kuna_simdlane`].
    pub simd_lane_fold: bool,
    /// (kuna GH-9218) Absorb overlapping input Varnodes above a justified
    /// container (C++ `input_varnode_adjust`).
    pub input_varnode_adjust: bool,
    /// (kuna) `option retinputhalf`: keep a returned register half whose value is
    /// a formal input parameter, instead of discarding it as uncomputed leftover.
    /// Read by [`crate::kuna_retinputhalf`] through the `ArchContext` handle.
    pub ret_input_half: bool,
    /// (kuna) `option noreturnretuse`: a CALL on a block that ends in a no-return
    /// halt does not veto the RETURN's output trial.  Read by
    /// [`crate::p4_calls::kuna_noreturnretuse`] through the `ArchContext` handle.
    pub noreturn_ret_use: bool,
    /// (kuna) `option rustabi`: how hard to try to keep a rustc two-register
    /// (`ScalarPair`) return intact -- 0 off, 1 auto (only on a detected rustc
    /// image), 2 always.  Read by [`crate::kuna_rustabi`] through the
    /// `ArchContext` handle.
    pub rust_abi: u8,
    /// (kuna) Did the loader's source-language detection report rustc for the
    /// image being decompiled?  A FACT, not an option: written once at `load
    /// file` from `kuna_analysis::sourcelang::detect_compiler`, and the thing
    /// `option rustabi auto` tests.  The XML `<binaryimage>` bootstrap never runs
    /// the analyzer tier, so it stays false there.
    pub source_is_rust: bool,
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
    /// (kuna) Run the jump-table partial sub-decompilation ONCE per
    /// `FlowInfo::recoverJumpTables` batch and share it across every table in
    /// that batch, as the C++ does (`Funcdata::stageJumpTable` guards the clone
    /// and the reduced pipeline behind `if (!partial.isJumptableRecoveryOn())`);
    /// off re-clones and re-analyses the function once per table, which is what
    /// `option unrolledguard` needs to see an already-recovered sibling table
    /// (C++ has no equivalent flag — this is the upstream shape vs kuna's).
    pub jumptable_share_partial: bool,
    /// (kuna, angr `test_decompiling_incorrect_duplication_chcon_main`) Treat a
    /// direct CALL to a function whose *name* matches the vendored ELF
    /// known-no-return list as no-return at the `query_call_no_return` flow hook,
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
    /// (kuna) Recover stack-passed call arguments at call sites with an unlocked
    /// callee prototype (default-on; restores upstream `fspec.cc:5618`).
    pub callsite_stack_args: bool,
    /// (kuna) A stack-pointer scramble against a live value (MSVC's `/GS`
    /// cookie) does not open a local-alias escape site (option `cookiescramble`).
    pub cookie_scramble: bool,
    /// (kuna) Let a bounded decode of the callee's own body veto a register
    /// argument the callee provably never reads (option `calleedeadarg`).
    pub callee_dead_arg: bool,
    /// (kuna) Narrow a call's `killedbycall` set to the registers a bounded
    /// decode of the callee's own body proves it writes (option
    /// `calleepreserves`).  See [`crate::p4_calls::kuna_calleepreserves`].
    pub callee_preserves: bool,
    /// (kuna) In the function's OWN input recovery, do not let a run of unused
    /// argument REGISTERS veto a later register the body reads before writing
    /// (option `inputparamgap`).  See [`crate::p4_calls::kuna_inputparamgap`].
    pub input_param_gap: bool,
    /// (kuna) Score a variadic call's stack arguments as their own `fillinMap`
    /// resource section, so the empty register slots the ABI leaves between the
    /// fixed parameters and the varargs stop deactivating them (option
    /// `varargstackargs`).  See [`crate::p4_calls::kuna_varargstackargs`].
    pub vararg_stack_args: bool,
    /// (kuna) Reconcile a call's recovered argument list with a sibling call to
    /// the same callee in the same function (option `calleearity`).  See
    /// [`crate::p4_calls::kuna_calleearity`].
    pub callee_arity: bool,
    /// (kuna) retry that reconciliation against sibling calls that finalize LATER
    /// in the same `ActionActiveParam` pass (option `calleearityfwd`).  See
    /// [`crate::p4_calls::kuna_calleearityfwd`].
    pub callee_arity_fwd: bool,
    /// (kuna) Extend a partially recovered argument list to a sibling call's
    /// when the callee's own body agrees (option `calleearitylive`).  See
    /// [`crate::p4_calls::kuna_calleearitylive`].
    pub callee_arity_live: bool,
    /// (kuna) Completion level for the two upstream partial-range call-overlap
    /// guards `Heritage::guardCallOverlappingInput` and
    /// `Heritage::tryOutputOverlapGuard`, which kuna shipped as comment-only stubs
    /// (option `calloverlap`).  `0` = off, `1` = input guard only, `2` = both
    /// (upstream Ghidra's behavior).  See [`crate::p3_dataflow::kuna_calloverlap`].
    pub call_overlap: int4,
    /// (kuna) Predicate strength for tolerating the caller's own caller-save
    /// spill among a trial Varnode's descendants in `Funcdata::onlyOpUse`
    /// (option `spillargtrial`).  `0` = off (upstream-faithful: every
    /// `CPUI_STORE` makes the trial inactive), `1` = spill/reload pairs only,
    /// `2` = any caller-frame store of the value.  See
    /// [`crate::p4_calls::kuna_spillargtrial`].
    pub spill_arg_trial: int4,
    /// (kuna) Refine indexed-stack LOAD/STORE guard ranges with the upstream
    /// ValueSet solver at the end of each heritage pass (upstream
    /// `Heritage::analyzeNewLoadGuards`, heritage.cc:834), so
    /// `MapState::addGuard` can supply real array index bounds in P6
    /// (option `loadguardrange`, default-on: upstream Ghidra's behavior).
    pub load_guard_range: bool,
    /// (kuna) Refuse the `RulePropagateCopy` marker propagation that would
    /// orphan an address-tied `COPY` output holding a call's return value,
    /// keeping a `local = f();` frame store in the emitted C (option
    /// `tiedstorekeep`, default-on, DIV-105).  See
    /// [`crate::p3_dataflow::kuna_tiedstorekeep`].
    pub tied_store_keep: bool,
    /// (kuna) Region-based (Phoenix/SAILR) structurer: structure the CFG by
    /// walking the [`KunaRegionIdentifier`](crate::p7_regions::kuna_regionid)
    /// region tree and matching Phoenix acyclic schemas instead of running
    /// Ghidra's `CollapseStructure` (option `regionstructure`, DIV-12 default-on:
    /// the primary structuring path; falls back to `CollapseStructure` on irreducible code).
    pub region_structure: bool,
    /// (kuna) Source-layout tie-break for `CollapseStructure::ruleBlockIfNoExit`'s
    /// clause arm (option `guardarm`, default-off opt-in).  Only fires when BOTH
    /// arms are eligible clauses; the earlier-addressed arm becomes the `if`
    /// clause instead of out-index 0.
    pub guard_arm: bool,
    /// (kuna) Defer a live loop head in `CollapseStructure`'s deferred
    /// `ruleBlockIfNoExit` scan while a non-head candidate remains (option
    /// `loopcondhoist`, default-off opt-in), so `ruleBlockWhileDo` keeps the
    /// loop's head test instead of emitting `while(true) { if (!C) ...; }`.
    pub loop_cond_hoist: bool,
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
    /// (kuna) Short-circuit condition folding across a **complex sibling block**
    /// (angr Phoenix's `MultiStatementExpression` relaxation of
    /// `_match_acyclic_short_circuit_conditions`).  Ghidra's `ruleBlockOr` declines
    /// the `A || B` fold whenever the sibling condition block is *complex*
    /// (`BlockBasic::isComplex`: more than two statements), so a single spill,
    /// address computation or extra call parked in front of the second test costs a
    /// crossing `goto` back into the first arm's clause — or, on a guard cascade
    /// whose arms reconverge, a `goto` + label into the shared body.  When this is
    /// set, a complex sibling is accepted when it fits the level's **printed-width
    /// budget** (the operand renders at most 5 comma elements at `on` / 9 at `wide`,
    /// counted with the printer's own skip rules) **and** either admission rule takes
    /// it:
    ///
    /// * **Rule A** — a `BlockCopy` of ONE `BlockBasic`, branch-free, comment-free,
    ///   with at most 1 *statement-root* call;
    /// * **Rule B** — the statement-shape allowlist (marker / op with an output /
    ///   void `CALL`/`CALLIND` / `STORE` / the single terminal `CBRANCH`), <=2
    ///   `calc_explicit`-scored statements and <=2 calls per block, no comment, and —
    ///   because Rule B also admits a nested `BlockCondition` so a cascade can fold —
    ///   <=4 condition leaves and <=4 total scored statements at the fold site.
    ///
    /// The absorbed statements then render inside the `&&`/`||` operand as a C comma
    /// expression, which the printer already supports (`comma_separate`).  The fold
    /// moves no p-code — it re-parents two existing structuring nodes — and C's
    /// short-circuit + comma sequencing preserves the original execution paths and
    /// order, so predicates that call functions need no purity analysis.
    ///
    /// **What the budget bounds.**  Per admitted leaf, the comma chain measured at
    /// structuring time is at most the level's cap; measured over 2827 functions the
    /// widest operand condfold *creates* is 5 elements at `on` and 7 at `wide`.  It
    /// does NOT bound the summed width of a Rule-B cascade's leaves (the only
    /// cross-leaf cap is expressed in the weak `calc_explicit` score), it is taken on
    /// the op list as it stands at structuring time (later passes can add or drop an
    /// op), and it over-counts where a call's stack-effect ops are still live — an
    /// error in the declining, i.e. safe, direction.  Rule B's <=2 *scored* statements
    /// is explicitly **not** a width bound: a block can score 2 and render 7, which is
    /// exactly why the printed-width budget exists.
    ///
    /// Rule A's call cap counts only calls printed as their own comma-chain element:
    /// the eligibility walk mirrors the printer, whose implied-output skip
    /// necessarily runs before the call test, so a call inlined into the sibling's
    /// own condition is not charged and a folded operand can render more than one
    /// call.  That is deliberate and is a readability bound, not a soundness bound —
    /// see
    /// [`MAX_PREFIX_ROOT_CALLS`](crate::p8_structure::kuna_condfold::MAX_PREFIX_ROOT_CALLS).
    /// Rule B charges every call op.
    ///
    /// Two effects are accepted rather than fixed: this is **not a monotone goto
    /// reducer** (an individual function can gain a goto even where the aggregate
    /// falls), and an advisory comment produced by a pass that runs *after*
    /// structuring can be dropped by the `comma_separate` operand (the guard declines
    /// on any comment buffered at structuring time, but sees no later one).
    ///
    /// The value is the **shared printed-width budget**, which doubles as the whole
    /// option's policy level: `0` = `option condfold off` (byte-identical to
    /// upstream: the precompute is skipped and every gate disjunct is dead), `5`
    /// ([`MAX_PREFIX_STMTS_ANGR`](crate::p8_structure::kuna_condfold::MAX_PREFIX_STMTS_ANGR))
    /// = `on` (angr parity), `9`
    /// ([`MAX_PREFIX_STMTS_WIDE`](crate::p8_structure::kuna_condfold::MAX_PREFIX_STMTS_WIDE))
    /// = `wide` (absorbs kuna's finer printed-statement granularity).  The level moves
    /// the budget for BOTH rules and nothing else.  Default-OFF opt-in.  See
    /// [`crate::p8_structure::kuna_condfold`].
    /// (kuna) `outline` region spec, empty when off.  See
    /// [`crate::p8_structure::kuna_outline`].
    pub outline_spec: String,
    pub cond_fold: int4,
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
    /// (kuna) `iteexpr`: extend [`iteregion`](Self::iteregion) to diamonds whose
    /// arms are a single **computed** pure-value assignment (`v = *p`, `v = b + 5`)
    /// — not just a plain `COPY` — matching angr's aggressive `?:` recovery.  A C
    /// ternary evaluates only the taken branch, so the rewrite is
    /// semantics-preserving; it is print-only.  A runtime choice, default-off (it
    /// diverges when the source used explicit if/else); measured net-positive on the
    /// decbench O0 set where source ternaries dominate.
    pub iteexpr: bool,
    /// (kuna) `iteboolean`: re-roll a short-circuit `0`/`1` select diamond — a
    /// 3-component `if` whose condition is a folded `&&`/`||` chain and whose two
    /// arms COPY the constants `1`/`0` to the same variable — into a single
    /// boolean assignment (`x = a && b;`).  P3 `RuleConditionalMove` cannot fold
    /// it: the constant arm has 2+ predecessors, so upstream (faithfully) bails.
    /// A runtime choice like [`iteregion`](Self::iteregion): the source may have
    /// written the explicit `if/else`, which compiles identically.  Print-only.
    pub iteboolean: bool,
    /// (kuna) `itecondlist`: let [`iteregion`](Self::iteregion) and
    /// [`iteboolean`](Self::iteboolean) descend a multi-component `BlockList` in the
    /// diamond's **condition** position to its last component.  Without it a diamond
    /// whose predecessor was concatenated onto its condition block declines, which is
    /// why a run of N identical diamonds folds only `ceil(N/2)` of them.  Print-only
    /// and sound because the printer already renders the leading components as
    /// statements before the `if` header.  Read by
    /// [`crate::p8_structure::kuna_itecondlist::cond_list_tail`].
    pub itecondlist: bool,
    /// (kuna) `paramcopyhoist`: anchor the trim COPY of an **unmodified incoming
    /// parameter** in the function's entry block instead of at the tail of the
    /// MULTIEQUAL slot's predecessor, so a guarded parameter's copy-shadow
    /// (`v6 = a1;`) prints in the entry block like the source's spill instead of
    /// below an earlier guard.  Read by
    /// [`crate::p6_variables::kuna_paramcopyhoist::hoist_target`] from
    /// `Merge::trimOpInput`; guarded by the `buildDominantCopy` Cover test.
    pub param_copy_hoist: bool,
    /// (kuna) angr SAILR gotoless `ReturnDuplicatorHigh`: duplicate a shared
    /// **bare-epilogue** RETURN block (only MULTIEQUAL/COPY/RETURN, no side effects)
    /// into each predecessor but one, so the classic
    /// `if (c) { body; return X; } return Y;` guard shape structures as
    /// per-predecessor early returns instead of one comma-folded exit — the gotoless
    /// complement to `ActionReturnSplit` (the goto-driven `ReturnDuplicatorLow`)
    /// (`duplicate_shared_returns`, DIV-54 default-on, superseding the DIV-18 revert).
    pub duplicate_shared_returns: bool,
    /// (kuna) `orchain`: decline a `duplicate_shared_returns` split whose shared
    /// RETURN block is the out-target two conditionals must keep in common for
    /// `CollapseStructure::rule_block_or` to fuse them — the operand chain of a
    /// short-circuit expression.  Read by
    /// [`crate::p8_structure::kuna_orchain::shortcircuit_shared_targets`].
    pub returndup_orchain: bool,
    /// (kuna) Hoist a leading const-guard into an early return (`if (c) return K;`) by
    /// peeling only the CONSTANT arm of a mixed return phi — the per-edge narrowing of
    /// angr SAILR `ReturnDuplicatorHigh` that `duplicate_shared_returns`' whole-block
    /// const gate cannot reach (`early_return`, opt-in default-off).
    pub early_return: bool,
    /// (kuna) The direct continuation of `early_return`: the same per-edge const peel, but
    /// with a wider in-edge cap so a WIDE multi-way switch-phi return
    /// (`switch (x) { case A: v = K0; break; … } return v;` with more cases than
    /// earlyreturn's 16-in-edge limit) is peeled to per-case `return K`
    /// (`switch_return`, opt-in default-off).
    pub switch_return: bool,
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
    /// (kuna) Strip rustc's bounds / slice / divide-by-zero panic branches --
    /// the `core::panicking::*` / `core::slice::index::*` / `core::str::*`
    /// helper calls a Rust binary carries in front of every checked access
    /// (option `securitycheck`, DIV-82 default-on; SEFCOM Oxidizer's
    /// `SecurityCheckRemover`).  Structurally inert on a C binary.
    pub strip_security_check: bool,
    /// (kuna) Flip negated-guard if/else branches for linearity: when an
    /// `if (x == 0) {A} else {B}` (equality-to-zero / negated guard) can be flipped
    /// in place, rewrite it to the positive `if (x) {B} else {A}` so the common
    /// path reads top-to-bottom (angr-style `if (x)` vs `if (x == 0)`).  Default
    /// OFF (option `branchflip`); read by `ActionBranchFlip` (S8).
    pub branch_flip: bool,
    /// (kuna) Use angr-style default naming (vN/aN/dat_/sub_/label_ + comments)
    /// (C++ `name_style_angr`).
    pub name_style_angr: bool,
    /// (kuna, Phase 3) Ghidra-convention default naming (`FUN_`/`DAT_`/`LAB_` +
    /// `%08x`) for entities no Symbol covers — set (with `name_style_angr` off)
    /// by the ghidra-mode registerProgram so kuna's fallback names match what
    /// Java's `isDynamicSymbolName`/`GlobalSymbolMap` expect
    /// (ghidra_arch.cc:928-947).  Never set on the standalone path.  Takes
    /// precedence over `name_style_angr` in [`Self::kuna_name_style`].
    pub name_style_ghidra: bool,
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
    /// (kuna `ctypes`) Spell the NAMED core types as the target's own C type
    /// names -- `int4` -> `int`, `uint1` -> `unsigned char`, `float8` -> `double`,
    /// `code *` -> `void *` -- resolved against the compiler spec's decoded
    /// `<data_organization>`, so an 8-byte integer reads `long` on LP64 and
    /// `long long` on ILP32/LLP64.  Extends [`realtypes`](Self::realtypes), which
    /// covers only residual `TYPE_UNKNOWN`; that split is why one function can
    /// today declare `unsigned int v3;` and `int4 v1;` in the same block.
    /// Default OFF in the catalog (the datatest corpus pins the Ghidra spellings
    /// in 42 assertions) but ON in the `aggressive` preset, which `auto` selects
    /// for anything under 500 KiB -- i.e. the default rendering of the CLI, the
    /// web front-end and the benchmark.  Read by the printc declarator
    /// chokepoints (`RealTypeCtx`).
    pub ctypes: bool,
    /// (kuna `framelayout`) Report the union of every stack-frame slot any
    /// `restructure_varnode` pass recovered on the `decompile-all --json`
    /// `variables` surface, not only the slots that survived to the final pass.
    ///
    /// `restructure_varnode` re-derives the frame from the LIVE stack Varnodes and
    /// clears the previous pass's unlocked symbols first, so a slot whose spill
    /// store/load pair the dataflow folded into a COPY and then away is present in
    /// an early layout and gone from the last one.  Dropping it from the emitted C
    /// is right -- there is no expression left to declare -- but the frame still has
    /// the slot, and `variables` is a description of the recovered FRAME (what IDA's
    /// stack view and Binary Ninja's variable list report), not of the printed
    /// declarations.  Affects the JSON surface only: no p-code, no emitted C.
    pub framelayout: bool,
    /// (kuna `voidtailreturn`) Elide the trailing bare `return;` of a void
    /// function -- the one the C source it came from does not have, because the
    /// source just falls off the end of the body.
    ///
    /// Only the function's OWN last statement, only when the prototype returns
    /// void, only when the owning structured leaf is not a goto target (a label
    /// would be left dangling) and only when exactly one structured leaf carries
    /// that RETURN op (`returndup`/`taildup` clone a shared epilogue by aliasing
    /// one op across several leaves, and suppressing by identity would delete
    /// genuine mid-body early returns).
    pub voidtailreturn: bool,
    /// (kuna `cortexmpriv`) Assume the Cortex-M core is privileged, folding away
    /// the `isCurrentModePrivileged()` guard the vendored ARM SLEIGH wraps around
    /// every VERSION_7M MRS/MSR (`kuna_cortexmpriv`).
    ///
    /// Twelve `ARMTHUMBinstructions.sinc` constructors lower as
    /// `b:1 = isCurrentModePrivileged(); if (!b) goto <notPriv>; <effect>`, so
    /// each MRS/MSR costs one basic block and two CFG edges that exist in no
    /// source. On the guard folds to `1` and only the effect survives.
    ///
    /// Read at the CALLOTHER consumption gate
    /// (`decompile_drive::is_injected_userop`), not at registration: the payload
    /// is installed at architecture bootstrap, before any `option` line.
    pub cortexmpriv: bool,
    /// (kuna `cortexmpriv`) The inject id of the `isCurrentModePrivileged`
    /// callother-fixup this architecture registered, or `None` on a language that
    /// does not declare the user-op. Identifies the one payload
    /// `option cortexmpriv off` suppresses.
    pub cortexmpriv_inject: Option<int4>,
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
    /// (kuna `rustabi`) Per-image cache of the callee-body probe
    /// ([`crate::kuna_rustabi::probe_callee_return_writes`]), keyed by the
    /// callee's `(space index, entry offset)`.  Each distinct function body is
    /// decoded at most once for the whole run, which is what keeps the probe off
    /// the critical path of a whole-binary `decompile-all`.  Stays empty unless
    /// `option rustabi` is live.
    pub kuna_callee_write_cache: std::collections::HashMap<
        (int4, uintb),
        std::rc::Rc<crate::kuna_rustabi::CalleeReturnWrites>,
    >,
    /// (kuna `calleedeadarg`) Per-image cache of the callee entry-liveness probe
    /// ([`crate::kuna_calleedeadarg::probe_callee_entry_dead`]), keyed by the
    /// callee's `(space index, entry offset)`.  Each distinct function body is
    /// decoded at most once for the whole run, which is what keeps the probe off
    /// the critical path of a whole-binary `decompile-all`.  Stays empty unless
    /// `option calleedeadarg` is live.
    pub kuna_callee_dead_cache: std::collections::HashMap<
        (int4, uintb),
        std::rc::Rc<crate::kuna_calleedeadarg::CalleeEntryDead>,
    >,
    /// (ghidra-mode, Phase 4) Name recommendations staged for the NEXT
    /// decompile drive — `(name, storage addr, usepoint, size)`, taken (and
    /// cleared) by `decompile_func_full_with_override_dyn` and seeded into the
    /// fresh `Funcdata`'s local scope.  The carrier for the host `<localdb>`'s
    /// namelocked-but-not-typelocked locals (GUI renames of untyped
    /// variables), which C++ keeps as `ScopeLocal::nameRecommend` entries
    /// rather than Symbols.  Always empty outside ghidra mode, so the
    /// standalone pipeline is structurally unaffected.
    pub kuna_pending_name_recs: Vec<(String, Address, Address, int4)>,
    /// (ghidra-mode, Phase 4) The DYNAMIC (hash-storage) half of the same
    /// staging — `(name, first-use address, hash)`; see
    /// [`Self::kuna_pending_name_recs`].  Always empty outside ghidra mode.
    pub kuna_pending_dyn_recs: Vec<(String, Address, u64)>,
    /// (ghidra-mode, Phase 4) The prototype MODEL the host declared for the
    /// next decompiled function (`<prototype model=…>`), staged the same way.
    /// The host assigned its committed parameter storage under this
    /// convention, so re-deriving storage from kuna's default model would
    /// disagree with the database and make Java's `checkFullCommit` rewrite
    /// the user's signature on any rename.  `None` outside ghidra mode.
    pub kuna_pending_proto_model: Option<Rc<crate::fspec::ProtoModel>>,

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
    /// (kuna) Gate PE import-call binding (`peimportcall`): paint
    /// `Varnode::externref` over the Import Address Table slots — the one flag
    /// `ActionDeindirect`'s external-reference arm requires — so a
    /// `call dword ptr [IAT slot]` binds to the import symbol the loader already
    /// resolved there (name, prototype and no-return flow effect), and match
    /// upstream's PE-only no-return API list (`ExitProcess`/`ExitThread`/…) that
    /// kuna's merged PE/Mach-O list never carried.  PE/COFF-only; a no-op on every
    /// other object format.  Also read (through the ArchSeam) by
    /// `Architecture::query_function`, whose no-return carry is the flow half of
    /// the same binding.
    pub analysis_peimportcall: bool,
    /// (kuna) Gate the library-prototype pass (`libproto`); default on.
    pub analysis_libproto: bool,
    /// (kuna) Gate the measured libc signature extension (`libcsigs`); default on.
    pub analysis_libcsigs: bool,
    /// (kuna) Gate the string-literal pass (`strings`); default on.
    pub analysis_strings: bool,
    /// (kuna) Gate the 2-byte (UTF-16LE) width of the string-literal pass
    /// (`widestrings`); default on. Off drops the wide facts at the commit, so the
    /// markup is exactly the 1-byte pass's.
    pub analysis_widestrings: bool,
    /// (kuna) Gate the entry-discovery pass (`entry_disc`); default on.
    pub analysis_entry_disc: bool,
    /// (kuna) Gate the `.eh_frame` LSDA landing-pad discovery sub-feature of the
    /// always-on entry-discovery pass (`eh_frame_full`, the GccExceptionAnalyzer
    /// `.gcc_except_table` markup); default **off** (output-changing: adds the
    /// discovered exception-handler landing pads as function entries).
    pub analysis_eh_frame_full: bool,
    /// (kuna) Refuse a function entry at a direct CALL target the Listing walk's
    /// own out-of-bounds gate forbids it to decode (`unmappedentry`); default
    /// **on**. The recursive-descent walk gates every instruction address on the
    /// executable-range universe but took the CALL target unconditionally, so a
    /// call into unmapped memory — what anti-disassembly junk behind an
    /// always-taken branch decodes to — still became a `sub_<addr>` with no
    /// bytes, no extent and no body. The call REFERENCE is filed either way; only
    /// the claim that the target is a function is withheld. Off restores the
    /// previous (phantom-producing) discovery set exactly.
    pub analysis_unmappedentry: bool,
    /// (kuna) Refuse a function entry at a PPC64 ELFv2 **local entry point**
    /// (`ppclocalentry`); default **on**. The OpenPOWER ELFv2 ABI gives a
    /// function two entries — the symbol's `st_value` (which materialises the
    /// TOC pointer `r2`) and a local entry `st_other` bytes later, which is
    /// where an intra-module `bl` lands. Nothing read `st_other`, so the Listing
    /// walk minted a function at every such call target and split every locally
    /// called function into an 8-byte named husk plus an anonymous body. On, an
    /// address a defined `STT_FUNC` symbol declares to be its own local entry is
    /// never claimed as a function; the call REFERENCE is filed either way. Off
    /// restores the previous (husk-producing) discovery set exactly.
    pub analysis_ppclocalentry: bool,
    /// (kuna) Fold a 32-bit PIC binary's base register into the cross-reference
    /// index (`picbase`); default **on**. In position-independent i386 code the
    /// address of a string, a global or a function pointer is never a constant in
    /// the instruction that uses it: it is the sum of a GOT pointer the program
    /// materialised at run time (`call <next>; pop ebx; add ebx,imm`) and a
    /// displacement, so a scan over decode-time constants — however wide — finds
    /// nothing and every literal in the image reports being referenced by
    /// nothing. On, the idiom is interpreted, cross-checked against the image's
    /// own `_GLOBAL_OFFSET_TABLE_`, and offered to a function only where that
    /// function's own body cannot have changed the register. Off restores the
    /// previous answer exactly. Query-surface only: `kuna xrefs`, `kuna strings`
    /// and the `decompile-all` xref section read this index; no p-code, no
    /// prototype and no emitted C depends on it.
    pub analysis_picbase: bool,
    /// (kuna) Give the function the PE C-runtime startup calls with argc/argv/envp
    /// the prototype that call site establishes (`entrymainproto`); default
    /// **on**. kuna recovers a callee's parameters from the callee's own body, so
    /// a `main` that ignores its arguments is declared `void(void)` even though
    /// the CRT a few lines up passes three. On a PE that startup is IN the image
    /// and fetches each argument through a named CRT accessor
    /// (`__p___argc`/`__p___argv`/`_get_initial_*_environment`), which makes the
    /// call site an unambiguous fingerprint. Parameters are typed at the width
    /// the call site establishes and named after the accessor, never asserted to
    /// be the C library's `int main(int, char **, char **)`. PE-only, and skipped
    /// whenever the callee already carries a function symbol. Off restores the
    /// `void(void)` form exactly.
    pub analysis_entrymainproto: bool,
    /// (kuna) Name the Mach-O `LC_MAIN` entry routine `main` and declare it
    /// `int main(int argc, char **argv)` (`machomain`); default **on**.
    /// `LC_MAIN`'s `entryoff` field is documented as the offset of `main()` and
    /// survives `strip`, so on a stripped Mach-O the container still states which
    /// of its `sub_<addr>` functions the program starts in — kuna simply never
    /// read it, and the prototype was `void(void)` because a `main` that ignores
    /// its arguments reads no ABI argument register for body-driven recovery to
    /// find. Mach-O executables only, skipped on an `LC_UNIXTHREAD`-only image
    /// (that entry is the crt `start`, not `main`), and skipped whenever the
    /// entry already carries a function symbol. Off restores the `sub_<addr>` /
    /// `void(void)` form exactly.
    pub analysis_machomain: bool,
    /// (kuna) Reject a discovered function entry that falls strictly inside a
    /// single-function `.eh_frame` FDE body (`fdeinterior`); default **on**.
    /// kuna's function symbols carry no extent, so every discovery oracle can
    /// start a `sub_<addr>` in the middle of a body it cannot see — the
    /// `eh_frame_full` landing pads, the `aif` gap starts and the prologue
    /// patterns all do it on ordinary C++ output, and the resulting function
    /// inherits its parent's live frame pointer, so every local is a garbage
    /// dereference. An FDE is per-function by construction, so its interior is
    /// the extent the symbol table never carried. Only ranges that hold no other
    /// named function start, no other FDE start and no linker-stub section are
    /// used (the whole-PLT FDE is excluded, so import stubs are never touched),
    /// and an entry AT an FDE start is always kept. Off restores the previous
    /// discovery set exactly; inert on any image with no `.eh_frame` FDEs.
    pub analysis_fdeinterior: bool,
    /// (kuna) Gate the **full byte-pattern function-start** pass
    /// (`funcstart_patterns`); default **off** (output-changing: it discovers more
    /// functions). The faithful port of Ghidra's `FunctionStartAnalyzer` over the
    /// entire vendored pattern corpus (`entry/patterns/*.xml`, the
    /// `<patternpairs>` pre/post sequences + bare `<funcstart/>` patterns), as a
    /// SEPARATE pass from `entry_disc` (whose always-on oracle 5 ports only a
    /// minimal three-prologue subset). When on, a stripped binary recovers many
    /// more function starts (e.g. `push rbx; mov rbx,rdi` after NOP padding); the
    /// commit hook adds each as `sub_<addr>`, idempotent against the funcsym stream
    /// + the `entry_disc` entries. Default-off ⇒ the pass's facts are dropped at
    /// commit (`engine.rs::analysis_pass_enabled`) and every parity gate is
    /// byte-identical. Real-ELF/PE/Mach-O path only ⇒ the XML datatest oracle is
    /// structurally untouched.
    pub analysis_funcstart_patterns: bool,
    /// (kuna) Widen the ARM Cortex-M hardware vector-table signature
    /// (`cortexmvectors`); default **off** (output-changing: it discovers more
    /// functions). The shipped signature confirms a table only when it starts a
    /// section the loader maps executable, its stack word is in the architectural
    /// SRAM block, and its reset word equals `e_entry` — which rejects the
    /// `A`-only `.isr_vector` a bare-metal link script normally emits, a CCM/TCM
    /// stack, and any image whose ELF entry symbol is not the reset vector. When
    /// on, a table is confirmed by a run of at least three Thumb handler pointers
    /// behind a plausible SRAM/CCM/TCM stack word, in any allocated section — so
    /// the reset/exception handler seeds and the whole-image Thumb region paint
    /// arm on firmware they silently skipped. The widened scan runs only where the
    /// shipped signature found nothing, so it can add entries but never remove
    /// one. ARM-only; a no-op on every other language and on any ARM object with
    /// no vector table.
    pub analysis_cortexmvectors: bool,
    /// (kuna) Discover ARM function entries that are reachable only through a
    /// code-pointer word (`ptrentry`); default **off** (output-changing: it
    /// discovers more functions). The shipped code-pointer scan already finds
    /// every Thumb pointer word but accepts a target only if it opens with a
    /// stack-frame prologue and disassembles into more than two instructions —
    /// which rejects the frameless leaf callbacks and one-instruction `bx lr`
    /// exception handlers that make up most of the pointer-referenced population
    /// on bare-metal firmware. When on, a pointer target is admitted on
    /// *containment* evidence instead: no word referencing it may be the bytes of
    /// a decoded instruction or lie in the same discovered function as the target
    /// (the `ldr pc,[pc,r]` switch-table shape), and its speculative decode must
    /// reach a clean return with no length floor. The accepted entries are purely
    /// additive — the pass never re-seeds the recursive-descent walk — so the
    /// option can add discovered functions but never remove one. ARM-only and
    /// Listing-tier: a no-op without `--option listing on` and on every other
    /// architecture.
    pub analysis_ptrentry: bool,
    /// (kuna) Reconstruct the ARM PC-relative literal pools the Listing never
    /// defines as data, and use them to fix the two defects that follow
    /// (`poolentry`); default **off** (output-changing: it both adds and relocates
    /// discovered functions). The AIF gap walk slides its cursor one byte at a time
    /// with no instruction-alignment filter, so a literal pool — data, hence an
    /// undefined gap — is probed byte by byte; on STM32 Thumb the high halfword of
    /// an `0x2000_xxxx` SRAM constant decodes as `movs r0,#0` and clears the
    /// fingerprint gate, so the accepted entry lands one halfword before the real
    /// function and the cursor then jumps past the body. When on, the pools are
    /// recovered from the literal references that already exist and drive two
    /// consumers: an additive entry fact at each pool end that abuts a return, and
    /// suppression of an AIF accept inside a pool whose end carries a replacement
    /// entry (a MOVE, never a delete). ARM-only in effect and Listing-tier: a no-op
    /// without `--option listing on`, without `aif`, and on every architecture whose
    /// constants live in `.rodata` rather than in `.text` interstices.
    pub analysis_poolentry: bool,
    /// (kuna) Gate the ARM/Thumb decode-mode marker pass (`arm_markers`); default on.
    pub analysis_arm_markers: bool,
    /// (kuna) Gate the MIPS `$gp`-recovery (`t9` tracking) pass (`mips_gp`); default on.
    pub analysis_mips_gp: bool,
    /// (kuna) Gate the i386-PIE PLT-stub decode (`i386_pie_plt`); default on. The
    /// loader (`kuna-analysis::loader::elf_plt::decode_i386`) decodes the
    /// GOT-relative `jmp *disp(%ebx)` (`FF A3 <disp32>`) PIE stub form so dynamic
    /// imports (`exit`/`dcgettext`/…) are named and `exit` is flagged no-return
    /// (collapsing the spurious fall-through loop). i386-only; a no-op on every
    /// other language. NOTE: the loader reads this through the
    /// [`crate::kuna_i386_pie_plt`] **env var** (the PLT map is baked at `load
    /// file`, upstream of `option`); this bool exists only for catalog visibility
    /// and the `phase catalog` live `current` field.
    pub analysis_i386_pie_plt: bool,
    /// (kuna) Gate the x86-64 IFUNC (`R_X86_64_IRELATIVE`) `.plt.sec`/`.iplt`
    /// stub naming (`ifuncfpret`); default off. When on, the loader
    /// (`kuna-analysis::loader::elf_plt::resolve_plt_imports`) names each ifunc
    /// stub `ifunc_<resolver>` so a tail `jmp` to it is recovered as a tail call
    /// (`tailcalljump`). Read through the [`crate::kuna_ifuncfpret`] **env var**
    /// (the PLT map is baked at `load file`); this bool is for catalog visibility.
    pub analysis_ifuncfpret: bool,
    /// (kuna) Gate the relocatable-object analysis rebase (`relocrebase`); default
    /// **on** (DIV-79, GH-289). The loader lays an ELF `ET_REL` `.o` / COFF `.obj`
    /// out synthetically above `RELOC_BASE`, but the LOAD-TIME analysis passes
    /// re-parse the same file and compute pre-link, section-relative addresses —
    /// mixing two address spaces in one inventory (phantom `sub_<off>` entries
    /// beside the real ones, strings and DWARF globals that never attach). When on,
    /// `kuna-analysis::loader::kuna_relocrebase` re-presents the object in the
    /// loaded image's address space before any pass reads it. Read through the
    /// [`crate::kuna_relocrebase`] **env var** (the analyzer tier runs inside `load
    /// file`, upstream of `option`); this bool exists only for catalog visibility
    /// and the `phase catalog` live `current` field.
    pub analysis_relocrebase: bool,
    /// (kuna) Gate the linked-image dynamic-relocation pass (`dynrelocs`);
    /// default **on** (DIV-84). A PIE / dynamically linked ELF leaves every
    /// `R_*_RELATIVE`/`GLOB_DAT`/`JUMP_SLOT` slot at 0 for the run-time loader, so
    /// kuna's mapped image reads a null function pointer and a call through the
    /// GOT renders `(*dat_<addr>)(…)`. When on,
    /// `kuna-analysis::loader::kuna_dynrelocs` fills those slots in and reports
    /// the `PT_GNU_RELRO`-frozen ones as constant so the call resolves to its
    /// name. Read through the [`crate::kuna_dynrelocs`] **env var** (the image
    /// bytes are snapshotted at `load file`, upstream of `option`); this bool
    /// exists only for catalog visibility and the `phase catalog` live `current`
    /// field.
    pub analysis_dynrelocs: bool,
    /// (kuna) Gate the PE chained-`UNWIND_INFO` `.pdata` entry skip
    /// (`pdatachained`); default **on** (DIV-117, GH-403). MSVC splits a
    /// shrink-wrapped or separated function across several `RUNTIME_FUNCTION`
    /// records; every record after the first points at an `UNWIND_INFO` carrying
    /// `UNW_FLAG_CHAININFO`, which makes its `BeginAddress` a point INSIDE the
    /// primary rather than a function start. Read through the
    /// [`crate::kuna_pdatachained`] **env var** (the entry oracles run inside
    /// `load file`, upstream of `option`); this bool exists only for catalog
    /// visibility and the `phase catalog` live `current` field.
    pub analysis_pdatachained: bool,
    /// (kuna) Gate degenerate-symbol-name repair (`symbolnamerepair`); default
    /// **on**. An empty `::` component in a loader symbol name is rejected by
    /// `Database::attach_scope`, and because the symbol table is installed inside
    /// `load file` that error aborts the ENTIRE architecture build — one symbol
    /// name, and every command on that binary produces nothing. On, the empty
    /// component is skipped and the symbol keeps the rest of its scope path.
    /// Read through the [`crate::kuna_symbolnamerepair`] **env var** (the symbol
    /// install runs inside `load file`, upstream of `option`); this bool exists
    /// only for catalog visibility and the `phase catalog` live `current` field.
    pub analysis_symbolnamerepair: bool,
    /// (kuna) How much of a raw symbol name's byte content is rewritten at the
    /// mint (`symbolnamechars`); default [`NameChars::Safe`]. A name's bytes
    /// otherwise reach emitted C verbatim, where a `*/`, a newline or a `//`
    /// restructures the document and an invalid UTF-8 byte collapses two
    /// distinct symbols onto one `String`. Read through the
    /// [`crate::kuna_symbolnamechars`] **env var** (names are minted inside
    /// `load file`, upstream of `option`); this field exists only for catalog
    /// visibility and the `phase catalog` live `current` field.
    ///
    /// [`NameChars::Safe`]: crate::kuna_symbolnamechars::NameChars::Safe
    pub analysis_symbolnamechars: crate::kuna_symbolnamechars::NameChars,
    /// (kuna) The scope-component ceiling one qualified symbol name may nest
    /// (`symbolnamebound`); default `Some(256)`, `None` for the historical
    /// unbounded behavior. `Database::find_create_scope_from_symbol_name` nests
    /// one ~1.5 KB `Scope` per `::` component, so an unbounded name is a ~498x
    /// input-to-RSS amplifier on attacker-controlled `.strtab` bytes (GH-338).
    /// Read through the [`crate::kuna_symbolnamebound`] **env var** (the symbol
    /// install runs inside `load file`, upstream of `option`); this field exists
    /// only for catalog visibility and the `phase catalog` live `current` field.
    pub analysis_symbolnamebound: Option<usize>,
    /// (kuna) Gate MSVC `__real@` FP-constant COMDAT recovery (`msvcfpconst`);
    /// default **on** (DIV-96). MSVC spells each floating-point literal in the
    /// name of the COMDAT that holds it, and COMDAT folding leaves that symbol
    /// *undefined* in every object but one — so the value is gone and the
    /// expression reads `... * dat_402020 + dat_402040`. On,
    /// `kuna-analysis::loader::kuna_msvcfpconst` decodes the name, materialises
    /// the bytes at the synthetic extern slot, and reports both the materialised
    /// slots and the object's *defined* `__real@` COMDATs as foldable, so the
    /// whole expression renders as literals. Read through the
    /// [`crate::kuna_msvcfpconst`] **env var** (the bytes are materialised inside
    /// `load file`, upstream of `option`); this bool exists only for catalog
    /// visibility and the `phase catalog` live `current` field.
    pub analysis_msvcfpconst: bool,
    /// (kuna) Gate the MIPS16 `ISA_MODE` decode-mode marker pass (`mips_isa`); default on.
    pub analysis_mips_isa: bool,
    /// (kuna) Gate the DWARF recovery pass (`dwarf`); default on.
    pub analysis_dwarf: bool,
    /// (kuna) Gate the ELF data-symbol (`STT_OBJECT`) naming arm (`datasyms`);
    /// default **on** (DIV-76). The loader collects the data half of the
    /// `.symtab`/`.dynsym` walks (`ObjectLoadImage::data_symbols`) at `load
    /// file`; the console's `commit_analysis_output` (run at `read symbols`,
    /// after this option is applied) consults this flag and installs each entry
    /// as a named `undefined<size>` global — so a copy-relocated libc extern
    /// (`stderr`, `optind`) renders by name instead of `dat_<addr>`. Off drops
    /// the stream at the commit and restores the previous rendering exactly.
    pub analysis_datasyms: bool,
    /// (kuna) Gate the DWARF `.debug_line` source-line comment pass (`dwarf_lines`);
    /// default **off** — it changes the decompiled output (adds `/* file:line */`
    /// comments). The kuna analog of Ghidra's `DWARFLineInfoCommentScript`.
    pub analysis_dwarf_lines: bool,
    /// (kuna) Gate the DWARF C++ prototype arm (`cppproto`); default **on**.
    /// Commits the DWARF facts recovered by resolving a subprogram definition
    /// through its `DW_AT_specification`/`DW_AT_abstract_origin` link, qualifying
    /// the name by its namespace/class ancestry, and binding the prototype by
    /// entry ADDRESS instead of by name. Off restores the name-only walk, which
    /// drops the signature of every out-of-line C++ member function.
    pub analysis_cppproto: bool,
    /// (kuna) Gate full-depth DWARF type resolution (`typedepth`); default
    /// **on**. The type mapper's recursion guard becomes upstream's per-DIE
    /// re-entry counter (`DWARFDataTypeImporter.trackRecursion`) instead of a
    /// flat three-hop budget that counted transparent `typedef`/`const` links, so
    /// an ordinary `const char **` / `char *const []` / `char ***` resolves
    /// instead of falling back to `void`. NOTE: the mapper reads this through the
    /// [`crate::kuna_typedepth`] **env var** (the types are baked at `load file`,
    /// upstream of `option`); this bool exists only for catalog visibility and
    /// the `phase catalog` live `current` field.
    pub analysis_typedepth: bool,
    /// (kuna) Gate DWARF aggregate-LAYOUT import (`dwarfstructs`); default
    /// **on**. A `DW_TAG_structure_type`/`union_type`/`class_type` carries its
    /// `DW_AT_byte_size` and its `DW_TAG_member` children (offsets verbatim,
    /// bitfields included) onto the interned type instead of becoming a named,
    /// EMPTY, zero-size shell — so a by-value struct parameter keeps its type, an
    /// 8-byte struct return stops being misclassified as a hidden-return-buffer
    /// call, and a field access renders as `n->inner.a` instead of
    /// `*(int *)((long)n + 4)`. NOTE: the mapper reads this through the
    /// [`crate::kuna_dwarfstructs`] **env var** (the types are baked at
    /// `load file`, upstream of `option`); this bool exists only for catalog
    /// visibility and the `phase catalog` live `current` field.
    pub analysis_dwarfstructs: bool,
    /// (kuna) Gate DWARF variant-part import (`dwarfvariants`); default **on**.
    /// A `DW_TAG_structure_type` carrying a `DW_TAG_variant_part` (a Rust
    /// tagged enum) recovers its discriminant member, its per-variant
    /// `DW_AT_discr_value` and each variant's named payload, instead of the
    /// field-less shell `dwarfstructs` leaves behind (a Rust enum has no
    /// `DW_TAG_member` of its own). NOTE: the mapper reads this through the
    /// [`crate::kuna_dwarfvariants`] **env var** (the types are baked at
    /// `load file`, upstream of `option`); this bool exists only for catalog
    /// visibility and the `phase catalog` live `current` field.
    pub analysis_dwarfvariants: bool,
    /// (kuna) Gate the demangled-C++-signature arm (`cppsig`); default
    /// [`CppSigMode::Proven`]. Commits the prototypes read off a MANGLED SYMBOL —
    /// the class type for `this` plus the declared parameter types — which is the
    /// only signature source left on a STRIPPED C++ binary. Three-valued because
    /// Itanium mangling cannot distinguish a static member function from a
    /// non-static one: `proven` applies only the shapes the mangling entails,
    /// `inferred` also decides the ambiguous ones from class evidence, `off`
    /// restores name-only demangling. See [`crate::kuna_cppsig`].
    pub analysis_cppsig: crate::kuna_cppsig::CppSigMode,
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
    /// by the always-on `strings` + libproto/S5 typing — so a per-instruction
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
    /// (kuna) Gate the rooted fast whole-project function discovery pass
    /// (`fast_funcdisc`); default **off**. It recursively follows direct calls
    /// from metadata-backed roots and admits pointer-table targets only after
    /// fingerprint and valid-subroutine checks. The `fast` mode enables it.
    pub analysis_fast_funcdisc: bool,
    /// (kuna) Gate the discovered-no-return consumer (`noreturn_disc`), the first
    /// Listing/xref consumer; default **off**. It is a flow heuristic (a callee is
    /// no-return if ≥3 of its call sites show no valid fall-through, iterated to a
    /// fixpoint over the Listing) that can be wrong, so it ships behind its own
    /// flag — the kuna analog of Ghidra's `FindNoReturnFunctionsAnalyzer`. Reads
    /// the Listing (`--option listing on` builds it); a no-op when the Listing is
    /// absent. Default-off ⇒ every parity gate is byte-identical.
    pub analysis_noreturn_disc: bool,
    /// (kuna, GH-312) Narrow `noreturn_disc`'s no-fall-through predicate to the
    /// arms that observe the program; default **on** (DIV-92). The legacy tally
    /// counts "the byte after the call is not a decoded instruction start" as a
    /// vote for the callee being no-return, but the Listing walk always attempts a
    /// call's successor, so that arm fires exactly when kuna's decoder failed —
    /// three spec gaps forge the verdict and DELETE live code at every caller.
    /// When on, only the terminal arm (no fall-through at all) and the two
    /// positive arms (the successor is data / another function's entry) count.
    /// Reads the Listing (`--option listing on` builds it); a no-op when the
    /// Listing is absent, so every parity gate is byte-identical.
    pub analysis_noreturn_discstrict: bool,
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
    /// the decompile stage must prune (as `CALL_RETURN` flow overrides). Populated at the
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
    /// (kuna, NOVEL) Gate the Itanium (GCC/Clang) RTTI + vtable recovery pass
    /// (`itaniumrtti`); default **off**. Ghidra has no Itanium RTTI analyzer at all
    /// — its `RttiAnalyzer` is Microsoft-only and its GCC class recovery is
    /// script-tier — so on a stripped `g++` binary a vtable stays `DAT_<addr>`.
    /// This pass reads the Itanium C++ ABI graph directly: every `_ZTI…` typeinfo
    /// object is located from the dynamic relocation that names its
    /// `__cxxabiv1::__{,si_,vmi_}class_type_info` vtable (an anchor `strip
    /// --strip-all` cannot remove from a shared object), its `_ZTS…` type-name
    /// string is demangled to the class name, its base list gives the inheritance
    /// displacements, and every `_ZTV…` sub-vtable pointing back at it is walked.
    /// Emits `<C>::typeinfo` / `<C>::typeinfo_name` / `<C>::vtable` /
    /// `<C>::vtable_for_<Base>` data labels plus one `<C>::vtable_<i>` function
    /// symbol per virtual slot. ELF-only (registered in `passes_for` only for
    /// `BinaryFormat::Elf`, and the pass also self-gates on ELF in `run`), real-ELF
    /// path only ⇒ every XML parity gate is byte-identical. Default-off
    /// (output-changing: it adds named data and function symbols);
    /// `--option itaniumrtti on` enables it.
    pub analysis_itaniumrtti: bool,
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
    /// (kuna, GH-299) Gate the aligned slide for the AIF gap cursor (`aifstrict`);
    /// default **off**, carried by the `aggressive` preset. `run_aif` probes the
    /// undefined partition one BYTE at a time, so every byte of every hole is a
    /// candidate function start and the two local acceptance tests (a 2-mnemonic
    /// prologue fingerprint, a valid-subroutine decode) are applied to addresses that
    /// cannot be instruction boundaries. On a large stripped i386 PE that plants
    /// ~2,100 entries in the middle of a function body, 35% of them inside a function
    /// kuna already has an entry for. With this on the cursor advances to the next
    /// 4-byte boundary instead: only an aligned address or a hole's FIRST byte is a
    /// candidate, because a hole boundary is evidence (the walk decoded up to exactly
    /// there and stopped) while an interior byte the cursor slid onto is a guess.
    /// Measured over 110 stripped non-x86-64 binaries it removes 4,282 of 11,010
    /// mid-body entries and *raises* recall by 344, since a phantom accept consumes
    /// the real entry behind it. It does not reach the pre-registered acceptance bar
    /// for becoming the default, so it ships opt-in (the `aggressive` preset carries
    /// it, which is where every one of those numbers was measured) and GH-299 stays
    /// open. Inert without `aif`, so every parity gate is byte-identical.
    pub analysis_aifstrict: bool,
    /// (kuna, GH-313) Gate the AIF accept corroboration test (`aifcorroborate`);
    /// default **off**, and in NO preset. Upstream rejects a gap candidate on TWO
    /// fingerprint tests — `startCount < 4`, and then
    /// `numInstr <= 2 || (!addsInfo && startCount < 50)` — and kuna ported only the
    /// first plus the `numInstr` half of the second. So a self-contained routine
    /// that calls nothing, jumps nowhere known and merely reaches a `ret` is
    /// accepted on four discovered functions sharing its two-mnemonic prologue.
    /// With this on, an accept must EITHER add information (a call, or a jump into
    /// already-discovered code — computed upstream's way, not from the looser
    /// `adds_info` the validity gate uses) OR match a prologue that 50 discovered
    /// functions share; a refused candidate still consumes its body so the cursor
    /// cannot fall back into it. Applied only at the gap-walk accept; the raw
    /// Thumb-prologue, code-pointer-table and pointer-validation users of the same
    /// predicate carry their own corroboration. **Measured out for the default
    /// path**: over 110 stripped non-x86-64 binaries ON TOP OF `aifstrict` it cuts
    /// mid-body entries 6,728 → 4,653 but costs 850 of 44,957 recovered functions,
    /// raises recall on zero of the 110, and takes 84 / 141 real functions off the
    /// two u-boot A32 images DIV-20 exists for. Inert without `aif`, so every parity
    /// gate is byte-identical.
    pub analysis_aifcorroborate: bool,
    /// (kuna) Gate tail-call function-entry recovery (`tailcallentry`); default
    /// **off**. The recursive-descent Listing walk treats every non-CALL flow
    /// target as a same-function successor, so a routine reached only by a tail
    /// `B` is absorbed into its caller instead of becoming its own function. This
    /// option reads the completed walk and admits such a target as a NEW function
    /// entry when a containment model says the branch leaves the caller's region:
    /// every predecessor is an unconditional branch, the caller's stack frame is
    /// closed at the branch, and the target's flow region is disjoint from the
    /// rest of the caller. Additive — it emits entries and never rebuilds the
    /// Listing, so no already-discovered entry can be lost. Reads the Listing
    /// (`--option listing on` builds it); a no-op when the Listing is absent, so
    /// every parity gate is byte-identical.
    pub analysis_tailcallentry: bool,
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
    /// (tier-1: the `kuna_pdb_path` env var, the fid `kuna_fid_db` precedent),
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
    /// (kuna, Phase 3) The ghidra-mode lazy symbol provider (the `ScopeGhidra`
    /// port, [`crate::remote_provider::RemoteScope`]), installed by the
    /// ghidra-mode registerProgram via [`Self::install_remote_provider`];
    /// `None` on the standalone path.  Threaded into every per-function
    /// `ArchContext` by `build_arch_handle` and consulted by the flow
    /// environment's callee-name/no-return queries.
    pub remote_scope: Option<Rc<crate::remote_provider::RemoteScope>>,
    /// Options that can be configured (C++ `options`).
    pub options: OptionDatabase,
    /// Actions that can be applied in this architecture (C++ `allacts`).
    pub allacts: ActionDatabase,
    /// (kuna) Per-program restart-trigger side table (C++ file-static
    /// `restartTable`, owned here per `docs/rust-port/README.md` — one log per loaded
    /// program; survives `Funcdata::clear()` because it lives outside the
    /// Funcdata).  The `restarts` console command renders it.
    pub restart_log: crate::kuna_restartlog::RestartLog,
    /// Specifically registered user-defined p-code ops (C++ `userops`).
    pub userops: UserOpManage,
    /// Manager of decoded strings (C++ `stringManager`, a `StringManager*`).
    /// `sleigh_arch.cc:250` seeds this with a `StringManagerUnicode(this,2048)`.
    /// Held behind `Rc<RefCell<..>>` so the same instance can be *shared* into
    /// the per-function W4 [`ArchContext`] (`glb`): `Funcdata::getInternalString`
    /// (driven through the ArchContext during `RuleStringStore`/`RuleStringCopy`) must
    /// `registerInternalStringData` into the very map the printer later reads back
    /// via `getStringData` on this real `Architecture`.
    pub string_manager:
        Rc<std::cell::RefCell<crate::stringmanage::StringManagerUnicode>>,
    /// P-code injection manager (C++ `pcodeinjectlib`).  SLEIGH-backed.
    pub pcodeinjectlib: PcodeInjectLibrarySleigh,
    /// Comments for this architecture (C++ `commentdb`).  // STUB(comment.cc)
    pub commentdb: CommentDatabase,

    // --- W6/W8 subsystems wired by `init` (architecture.hh:211-233) -------
    /// Data-type factory (C++ `types`, a `TypeFactory*`).  Empty until
    /// [`build_typegrp`](Architecture::build_typegrp) + `build_core_types`.
    ///
    /// Held as an [`Rc`] so the analysis-side [`ArchContext`](crate::context::ArchContext)
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
    /// back to `defaultfp` when unset.  Set only by an explicit
    /// `option protoeval <model>`, which outranks the spec's own nomination.
    evalfp_current: Option<Rc<ProtoModel>>,
    /// The model the compiler spec **nominates** for evaluating the current
    /// function's prototype (its `<eval_current_prototype>`), decoded by
    /// [`build_default_proto`].  Reaches a function only under
    /// [`evalcurrentproto`](Self::evalcurrentproto); `None` for the many
    /// languages whose spec declares no such element.  See
    /// [`crate::kuna_evalcurrentproto`].
    evalfp_current_spec: Option<Rc<ProtoModel>>,
    /// (kuna) `evalcurrentproto`: honor the compiler spec's
    /// `<eval_current_prototype>` nomination, so a function whose prototype is
    /// unknown is evaluated with the spec's *merged* model and its
    /// register-passed parameters (x86 `__fastcall`/`__thiscall` `ECX`/`EDX`)
    /// are recovered instead of surfacing as reads of an undefined local.
    pub evalcurrentproto: bool,
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
    /// (kuna, Phase 3) Raw `<coretypes>` XML (the fourth ghidra-mode
    /// registerProgram spec document), set by [`Self::set_coretypes_xml`]
    /// before [`init_post_engine`](Architecture::init_post_engine) so
    /// [`build_core_types`](Architecture::build_core_types) decodes the host's
    /// core-type set (with the HOST ids) instead of the defaults.  `None` on
    /// the standalone path.
    coretypes_xml: Option<Vec<u8>>,
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
    /// Owned behind the [`EngineTranslate`] trait object (the
    /// `Architecture`↔translator boundary) rather than a concrete [`Sleigh`], so a
    /// ghidra-mode translator can replace `Sleigh` without `kuna-decomp` naming
    /// a ghidra-specific type (see `crate::engine_translate` /
    /// `docs/rust-port/ghidra-phase2-plan.md` §2.2).  The C++ `Architecture`
    /// is-a `AddrSpaceManager` and owns its `Translate`; here the manager lives
    /// inside the engine, reached through the trait's `manager*` accessors.
    /// Only `Sleigh` implements the trait today.
    translate: Box<dyn EngineTranslate>,
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
        // The standalone (SLEIGH) engine: box the concrete `Sleigh` into the
        // `EngineTranslate` boundary and share the construction with the ghidra-mode
        // path.  `EngineTranslate::manager()` / the provided
        // `Translate::get_unique_start` on the boxed `Sleigh` are the very calls
        // the former direct-`Sleigh` body made (`base().manager()` /
        // `get_unique_start`), so this delegation is behavior-identical for the
        // 675-datatest path.
        Architecture::from_engine_translate(archid, Box::new(translate))
    }

    /// Construct an `Architecture` over an already-initialized disassembly
    /// engine behind the [`EngineTranslate`] boundary — the shared body of
    /// [`Architecture::new`] (the standalone `Sleigh`) and the ghidra-mode
    /// bridge (`kuna-ghidra`'s query-backed `GhidraTranslate`, which
    /// `kuna-decomp` cannot name as a concrete type here — the whole reason the
    /// field is a trait object, see `crate::engine_translate`).
    ///
    /// The `translate` must already be initialized (spaces decoded,
    /// endianness/unique-base set); the architecture reads its space manager and
    /// `getUniqueStart(INJECT)` tempbase for the injection library, then builds
    /// the subsystems whose dependencies exist (the tail of C++
    /// `Architecture::init`/`restoreFromSpec` runs later, in
    /// [`init_post_engine`](Architecture::init_post_engine)).
    pub fn from_engine_translate(
        archid: &str,
        translate: Box<dyn EngineTranslate>,
    ) -> Architecture {
        // C++ PcodeInjectLibrarySleigh(g): tempbase = g->translate->getUniqueStart(INJECT).
        let inject_tempbase = translate.get_unique_start(UniqueLayout::INJECT);

        // C++ buildDatabase(store): create the symbol table + attach the global scope.
        // ScopeInternal sizes its per-space maps to numSpaces(); count before the
        // translate is moved into the struct (the manager accessor borrows it).
        let space_count = translate.manager().num_spaces();
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
            dynreloc_const: Rc::new(Vec::new()),
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
            rodata_string: false, // (kuna) option rodatastring; reset_defaults sets the shipped default
            ptrdepthcap: false, // (kuna) option ptrdepthcap; reset_defaults sets the shipped default
            add_carry_chain: false,
            v850_indirect_branch: false,
            fastfail_noreturn: false, // (kuna) option fastfailnoreturn; reset_defaults sets the shipped default
            msvc_ftol: false, // (kuna) option msvcftol; reset_defaults sets the shipped default
            tail_call_jumps: false,
            tail_call_frame: false, // (kuna) option tailcallframe; reset_defaults sets the shipped default
            funcbound_flow: false, // (kuna) option funcboundflow; reset_defaults sets the shipped default
            overlap_branch: false, // (kuna) option overlapbranch; reset_defaults sets the shipped default
            remove_cleanup_code: false, // (kuna) option cleanupcode; reset_defaults sets the shipped default
            linux_syscall: false, // (kuna) option linuxsyscall; reset_defaults sets the shipped default
            switch_selector_guard: false, // (kuna) option switchselector; reset_defaults sets the shipped default
            noreturn_extern_calls: false, // (kuna) option noreturn_extern, default off
            sparc_struct_return: false,
            ov_less_simplify: false,
            fold_boolean_mask: false,
            simd_lane_fold: false,
            ret_split_global: false,
            input_varnode_adjust: false,
            ret_input_half: false, // (kuna) option retinputhalf; reset_defaults sets the shipped default
            noreturn_ret_use: false, // (kuna) option noreturnretuse; reset_defaults sets the shipped default
            rust_abi: 0,        // (kuna) option rustabi; reset_defaults sets the shipped default
            source_is_rust: false, // (kuna) a load-time fact; set by the console's `load file`
            condexe_block_placement: false,
            dynamic_hash_maxdup_high: false,
            model_stack_probe_loop: false,
            fold_flag_compare: false,
            switch_modulo_bound: false,
            switch_guard_bound: false,
            switch_shared_case: false,
            switch_multi_pred: false,
            unrolled_guard: false,
            jumptable_share_partial: true,
            noreturn_extern_match: true, // (kuna) DIV-13 default-on (angr incorrect-duplication-chcon)
            stack_alias_deadstore: false,
            recover_array_stride: false,
            recover_lowered_switch: false,
            callsite_stack_args: true,
            cookie_scramble: true,
            callee_dead_arg: true,
            callee_preserves: true,
            input_param_gap: true,
            vararg_stack_args: true,
            callee_arity: true,
            callee_arity_fwd: true,
            callee_arity_live: true,
            call_overlap: 0,
            spill_arg_trial: 0,
            load_guard_range: false, // (kuna) option loadguardrange; reset_defaults sets the shipped default
            tied_store_keep: false, // (kuna) option tiedstorekeep; reset_defaults sets the shipped default (on)
            region_structure: true,
            guard_arm: false,
            loop_cond_hoist: false,
            region_loop_refine: false,
            region_edge_order: false,
            outline_spec: String::new(),
            cond_fold: 0,
            reduce_return_gotos: false,
            flatten_ifelse: false,
            revert_cross_jumps: false,
            dup_return_call_tails: false,
            dedup_ite_tail: false,
            iteregion: false,
            iteexpr: false,
            iteboolean: false,
            itecondlist: false,
            param_copy_hoist: false,
            duplicate_shared_returns: false,
            returndup_orchain: false,
            early_return: false,
            switch_return: false,
            recover_loop_break: false,
            fold_call_returns: false,
            strip_stack_guard: false,
            strip_security_check: false,
            branch_flip: false,
            name_style_angr: false,
            name_style_ghidra: false,
            dedup_var_decls: false,
            realtypes: false,
            ctypes: false, // (kuna) option ctypes; reset_defaults sets the shipped default
            framelayout: false, // (kuna) option framelayout; reset_defaults sets the shipped default
            voidtailreturn: false, // (kuna) option voidtailreturn; reset_defaults sets the shipped default
            cortexmpriv: false, // (kuna) option cortexmpriv; reset_defaults sets the shipped default
            cortexmpriv_inject: None, // (kuna) set by init_userops_and_fixups when the language declares the user-op
            present_lessequal: false,
            preserve_thumb_funcptr: false,
            kuna_fn_budget: None,   // (kuna) decompile-all watchdog: no budget by default
            kuna_fn_deadline: None, // (kuna) set per drive from kuna_fn_budget
            kuna_callee_write_cache: std::collections::HashMap::new(),
            kuna_callee_dead_cache: std::collections::HashMap::new(),
            kuna_pending_name_recs: Vec::new(), // (ghidra Phase 4) staged per drive
            kuna_pending_dyn_recs: Vec::new(),  // (ghidra Phase 4) staged per drive
            kuna_pending_proto_model: None,     // (ghidra Phase 4) staged per drive

            // Analysis-pass gates: placeholder values -- `Architecture::new` calls
            // `reset_defaults_internal()` below, the SINGLE source of every
            // effective default (phases.toml's `default` column mirrors it;
            // asserted equal by kuna_phases/tests.rs `live_value` parity).
            analysis_noreturn_known: false,
            analysis_peimportcall: false,
            analysis_libproto: false,
            analysis_libcsigs: false,
            analysis_unmappedentry: false,
            analysis_ppclocalentry: false,
            analysis_picbase: false,
            analysis_entrymainproto: false,
            analysis_machomain: false,
            analysis_strings: false,
            analysis_widestrings: false,
            analysis_entry_disc: false,
            analysis_eh_frame_full: false,
            analysis_fdeinterior: false,
            analysis_funcstart_patterns: false,
            analysis_cortexmvectors: false,
            analysis_ptrentry: false,
            analysis_poolentry: false,
            analysis_arm_markers: false,
            analysis_mips_gp: false,
            analysis_i386_pie_plt: false,
            analysis_ifuncfpret: false, // (kuna) option ifuncfpret, default off (opt-in)
            analysis_relocrebase: false,
            analysis_dynrelocs: false,
            analysis_pdatachained: false,
            analysis_symbolnamerepair: false,
            analysis_symbolnamechars: crate::kuna_symbolnamechars::NameChars::Off,
            analysis_symbolnamebound: None,
            analysis_msvcfpconst: false,
            analysis_mips_isa: false,
            analysis_dwarf: false,
            analysis_datasyms: false,
            analysis_dwarf_lines: false,
            analysis_cppproto: false,
            analysis_typedepth: false,
            analysis_dwarfstructs: false,
            analysis_dwarfvariants: false,
            analysis_cppsig: crate::kuna_cppsig::CppSigMode::Off,
            analysis_callfixup: false,
            analysis_addrtable: false,
            analysis_operand_refs: false,
            analysis_formatstring: false,
            analysis_listing: false,
            analysis_fast_funcdisc: false,
            analysis_noreturn_disc: false,
            analysis_noreturn_discstrict: false,
            analysis_noreturn_propagate: false,
            analysis_noreturn_error: false,
            analysis_noreturn_reach: false,
            error_noreturn_callsites: Vec::new(),
            analysis_fid: false,
            analysis_rtti: false,
            analysis_itaniumrtti: false,
            analysis_aif: false,
            analysis_aifstrict: false,
            analysis_aifcorroborate: false,
            analysis_tailcallentry: false,
            analysis_gopclntab: false,
            analysis_objc: false,
            analysis_pdb: false,
            macho_arm64e: false,

            symboltab,
            remote_scope: None,
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
            evalfp_current_spec: None,
            evalcurrentproto: false,
            default_return_addr: None,
            cspec_xml: None,
            pspec_xml: None,
            coretypes_xml: None,
            lanerecords: Vec::new(),
            inst: Vec::new(),
            opbehaviors: Vec::new(),
            // The engine behind the `EngineTranslate` boundary: a boxed `Sleigh` on
            // the standalone path, a query-backed `GhidraTranslate` on the
            // ghidra-mode path (already boxed by the caller).
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
        self.evalcurrentproto = true; // (kuna) DIV-71 default-on: honor the compiler spec's `<eval_current_prototype>` nomination, so an unknown prototype is evaluated with the spec's MERGED model and a `__fastcall`/`__thiscall` function's ECX/EDX arguments are recovered instead of being read as undefined locals. Byte-identical (0/675): only 6 vendored specs nominate a model, and the 3 datatests on an affected language are unchanged. Restore the `<default_proto>`-only evaluation with `option evalcurrentproto off`
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
        self.rodata_string = true; // (kuna) DIV-113 default-on: a read-only string block copy collapses to builtin_strncpy instead of the invalid-C partial-symbol slice assignments. Byte-identical (0/675) — the corpus carries no data symbols, so the covering-string-symbol guard never fires. Restore the slice assignments with `option rodatastring off`
        self.v850_indirect_branch = false; // (kuna) default: upstream (GH-8817)
        self.fastfail_noreturn = true; // (kuna) DIV-119 default-on: REMOVES CODE. Ends the flow at a Windows `int 0x29` (`__fastfail`), whose SLEIGH lifting is a call with no matching push and so gains 8 bytes of stack pointer from the cspec's `extrapop` at every site. Windows-cspec-gated and shape-gated on `swi(0x29:1)`, so it is structurally inert on the datatest corpus and byte-identical there (0/675); restore the unbalanced fall-through with `option fastfailnoreturn off`
        self.msvc_ftol = true; // (kuna) DIV-74 default-on: x86-32-only, and inert unless the binary imports an `__ftol`/`__ftol2`/`__ftol2_sse` symbol. Byte-identical (0/675) — no corpus function carries one of those names. Restore the un-fixed `__ftol()` rendering with `option msvcftol off`
        self.tail_call_jumps = true; // (kuna) DIV-13 default-on (angr tail-call recovery; per-test opt-out on Long double #1/#2)
        self.tail_call_frame = true; // (kuna) DIV-109 default-on: REMOVES CODE. A direct jmp preceded by a teardown of exactly the entry block's frame is a tail call even when the callee was never discovered. Byte-identical (0/675) on the datatest corpus; restore the flow-into-the-callee decode with `option tailcallframe off`
        self.funcbound_flow = true; // (kuna) DIV-67 default-on: REMOVES CODE. Truncates a fall-through that reaches another known function's entry (a function ending in an unnamed static no-return `exit`/`abort`/`die()` wrapper) instead of decoding the next function's body into it. Byte-identical (0/675) on the datatest corpus; restore upstream flow-into-callee with `option funcboundflow off`
        self.overlap_branch = true; // (kuna) DIV-106 default-on: REMOVES CODE. Ends a conditional branch's fall-through in a halt when the branch's own target lies strictly inside that fall-through instruction's encoding (the anti-disassembly junk-lead-byte overlap), instead of letting the bogus decode swallow the target and desynchronise the stream. Two real instruction starts cannot sit at `next` and strictly inside `next`, so the trigger never matches well-formed code and is byte-identical (0/675) on the datatest corpus; restore the fall-through-wins decode with `option overlapbranch off`
        self.remove_cleanup_code = true; // (kuna) DIV-81 default-on: REMOVES CODE. Deletes the Rust drop/deallocate call sites (`core::ptr::drop_in_place`, `Drop::drop`, `alloc::raw_vec::RawVecInner::deallocate`, `__rust_dealloc`) and the argument setup that only feeds them. Structurally inert outside a Rust binary (no C ELF resolves a call to one of those names), so byte-identical (0/675) on the datatest corpus; keep the drop glue with `option cleanupcode off`
        self.switch_selector_guard = false; // (kuna) option switchselector, default-off this round: the ablation says ON is safe (0/675 datatest assertions, one function changed across the swept corpus and strictly for the better), but shipping a default ON is a DIV-registry change and that registry is not this change's to write
        self.linux_syscall = false; // (kuna) option linuxsyscall, default-off this round: it renames a call and locks a prototype, which is a judgement about the target OS that the vector alone does not prove
        self.noreturn_extern_calls = true; // (kuna) DIV-14 default-on: REMOVES CODE (drops the post-call fall-through after a matched extern no-return). Byte-identical (0/675) — no datatest call resolves to a known no-return name; overlaps `noreturn_known`'s name match for defined/imported symbols, restore upstream with `option noreturn_extern off`
        self.sparc_struct_return = false; // (kuna) default: upstream byte-identical (GH-6882)
        self.ov_less_simplify = true; // (kuna) DIV-2 default-on (GH-7190)
        self.fold_boolean_mask = true; // (kuna) DIV-2 default-on (GH-1282)
        self.ret_split_global = true; // (kuna) DIV-PENDING default-on: a shared RETURN block that stores to GLOBALS is not the bare epilogue `ActionReturnSplit::isSplittable` assumes, so it is no longer cloned into every predecessor. One-directional (it can only decline a split) and byte-identical (0/675) on the datatest corpus; restore the upstream predicate with `option retsplitglobal off`
        self.simd_lane_fold = true; // (kuna) DIV-PENDING default-on: an exact identity (pshufb with a constant mask IS a byte permutation), so a lane read resolves to the source lane instead of an opaque CALLOTHER temporary. Byte-identical (0/675) on the datatest corpus; restore the opaque rendering with `option simdlane off`
        self.input_varnode_adjust = true; // (kuna) DIV-3 default-on (GH-9218)
        self.ret_input_half = true; // (kuna) DIV-85 default-on: a returned register half whose value is an input parameter the function MOVED into the return register is a real return, not leftover; keeping it also keeps the parameter it came from in the recovered signature. 0/675 byte-identical; an untouched return register is still dropped (the GH-6990 SPARC pass-through), restore the strict rule with `option retinputhalf off`
        self.noreturn_ret_use = true; // (kuna) DIV-118 default-on: a status value handed to a no-return failure call at the end of its block cannot compete with the same value at the function's RETURN, so it no longer forces the prototype to void. 0/675 byte-identical on the datatest corpus and 0 changed lines across 23 linked binaries; restore the upstream blanket rejection with `option noreturnretuse off`
        self.rust_abi = 0; // (kuna) option rustabi default off: the pair-keeping rules are opt-in this round
        self.dynamic_hash_maxdup_high = true; // (kuna) DIV-3 default-on (GH-8467)
        self.fold_flag_compare = true; // (kuna) DIV-3 default-on (GH-1276/8777)
        self.switch_modulo_bound = false; // (kuna) default: upstream byte-identical (GH-9191)
        self.switch_guard_bound = false; // (kuna) default: upstream byte-identical (angr opt-in)
        self.switch_shared_case = true; // (kuna) DIV-14 default-on (angr loop-carried-guard PIC switch recovery; slower on the functions it recovers, kept on for quality; 0/675 byte-identical)
        self.switch_multi_pred = true; // (kuna) DIV-13 default-on (angr multi-predecessor unrolled-guard jump-table; 0/675 ablation)
        self.unrolled_guard = false; // (kuna) default: upstream byte-identical (angr opt-in)
        self.jumptable_share_partial = true; // (kuna) DIV: the upstream stageJumpTable shape
        self.noreturn_extern_match = true; // (kuna) DIV-13 default-on (angr incorrect-duplication-chcon; clean 0/675 ablation)
        self.stack_alias_deadstore = false; // (kuna) default: upstream byte-identical (GH-8500)
        self.recover_array_stride = true; // (kuna) DIV-3 default-on (GH-8724)
        self.recover_lowered_switch = true; // (kuna) default-on (angr port)
        self.callsite_stack_args = true; // (kuna) default-on: restores upstream fspec.cc:5618 (0/675 ablation)
        self.cookie_scramble = true; // (kuna) DIV-126 default-on: an `xor rax,rsp` cookie mix no longer collapses the local-alias boundary to the bottom of the frame (0/675 ablation)
        self.callee_dead_arg = true; // (kuna) default-on (DIV-KUNA_DEADARG_DIV): 0/675 datatests, subtractive only
        self.callee_preserves = true; // (kuna) DIV-124 default-on: a fully decoded, call-free callee's own writes narrow the cspec killedbycall set, so a value that crosses a get-PC thunk survives (0/675 ablation)
        self.input_param_gap = true; // (kuna) DIV-114 default-on: an unused argument-register run in the function's OWN input recovery no longer vetoes a later live-in register, so a pointer-table-only callback recovers its full signature instead of reading undefined locals. Byte-identical (0/675) on the datatest corpus; restore upstream's forceInactiveChain veto with `option inputparamgap off`
        self.vararg_stack_args = true; // (kuna) DIV-101 default-on: a variadic call's stack tail is its own fillinMap section (0/675 ablation)
        self.callee_arity = true; // (kuna) DIV-102 default-on: one callee, one argument list across its call sites (0/675 ablation)
        self.callee_arity_fwd = true; // (kuna) DIV-PENDING default-on: retry that reconciliation against the siblings that finalize later (0/675 ablation)
        self.callee_arity_live = true; // (kuna) DIV-PENDING default-on: extend a partial argument list when the callee body agrees (0/675 ablation)
        self.call_overlap = 0; // (kuna) calloverlap: PLACEHOLDER default (set from measurement)
        self.spill_arg_trial = 0; // (kuna) spillargtrial default-OFF opt-in (diverges from upstream onlyOpUse; the failure mode is a spurious trailing argument, which no gate can see)
        self.load_guard_range = true; // (kuna) DIV-77 default-on: restores upstream Heritage::analyzeNewLoadGuards ValueSet range refinement of indexed-stack LOAD/STORE guards (0/675 ablation); `option loadguardrange off` reverts to whole-space guards with no index bound
        self.tied_store_keep = true; // (kuna) DIV-105 default-on: RulePropagateCopy refuses the marker propagation that would orphan an address-tied COPY holding a call return, so a `local = f();` frame store survives dead-code elimination (0/675 ablation, speed -0.13%); `option tiedstorekeep off` restores upstream's propagation
        self.region_structure = true; // (kuna) DIV-12 default-on (region-based Phoenix/SAILR structurer; primary structuring path, falls back to CollapseStructure on irreducible code)
        self.region_loop_refine = true; // (kuna) DIV-13 default-on (region structurer multi-exit/irreducible loop-successor refinement; 0/675 ablation)
        self.region_edge_order = false; // (kuna) SAILR P2 default-OFF opt-in (H2 post-dominator + dominance-tiered edge-virtualization ordering; only reorders which goto is chosen when virtualizing, so OFF is byte-identical)
        self.outline_spec = String::new(); // (kuna) default-OFF opt-in (excise a supplied single-entry region into a synthesized pseudofunction call; destructive, and inert with no region supplied)
        self.cond_fold = 0; // (kuna) default-OFF opt-in (angr Phoenix MultiStatementExpression short-circuit relaxation: fold `A || B` across a sibling carrying a bounded prefix, rendered as a comma expression; OFF is byte-identical)
        self.reduce_return_gotos = true; // (kuna) DIV-13 default-on (angr SAILR goto-reduction; 0/675 ablation)
        self.flatten_ifelse = true; // (kuna) DIV-13 default-on (angr IfElseFlattener; 0/675 ablation)
        self.revert_cross_jumps = true; // (kuna) DIV-13 default-on (angr SAILR CrossJumpReverter; 0/675 ablation)
        self.dup_return_call_tails = true; // (kuna) DIV-13 default-on (angr SAILR ReturnDuplicatorLow return-call-tail dup; 0/675 ablation)
        self.dedup_ite_tail = true; // (kuna) DIV-13 default-on (angr structurer ITE region-dedup — merge duplicated if/else tails; 0/675 ablation)
        self.iteregion = true; // (kuna) DIV-17 default-on (angr ITERegionConverter: assignment-diamond -> `?:` ternary, decbench F5). Per-test opt-out (`option iteregion off`) on the datatests it changes keeps the corpus byte-identical.
        self.iteexpr = false; // (kuna) computed-arm ?: extension: runtime choice, default-off (corpus byte-identical).
        self.itecondlist = true; // (kuna) DIV-56 default-on (iteregion/iteboolean match through a concatenated condition BlockList: a run of N identical diamonds folds N, not ceil(N/2); 0/675 ablation).
        self.iteboolean = true; // (kuna) DIV-51 default-on (short-circuit 0/1 select -> boolean assignment; 0/675 ablation). Per-test opt-out (`option iteboolean off`) on the one stage test it changes keeps the corpus byte-identical.
        self.returndup_orchain = true; // (kuna) DIV-69 default-on: the returndup narrowing. `returndup` splits a shared epilogue whose predecessors are the operand blocks of a short-circuit chain, which permanently blocks `rule_block_or` and turns one source boolean expression into a cascade of constant-return guards. Measured over the whole decbench corpus (85,195 functions, three optimisation levels): +611 GED-perfect at O0 for -13 / -15 at O2 / O2-noinline, +583 net and -967 aggregate GED. Default-ON rather than preset-only because `returndup` is itself a shipped default: 45% of the corpus is over the 500 KiB `auto` threshold and runs `reliable`, where a preset-only gate would leave the split unnarrowed
        self.duplicate_shared_returns = true; // (kuna) DIV-54 default-on, superseding the DIV-18 revert (angr SAILR gotoless ReturnDuplicatorHigh). The -976 GED-perfect regression that reverted it was measured before #137 added the const-return gate (`returndup_is_const_ret`); re-ablated on 52,862 decbench functions the selective pass is +417 GED-perfect / -7,756 aggregate GED, net-positive in every one of nine tested partitions. Per-test opt-out (`option returndup off`) on the datatests it changes keeps the corpus byte-identical.
        self.early_return = true; // (kuna) DIV-23 default-on (angr SAILR ReturnDuplicatorHigh PER-EDGE const-guard early-return hoisting: peel only the CONSTANT arm of a mixed return phi). The const-only narrowing of returndup that returndup's whole-block gate cannot reach; unlike broad returndup (DIV-18, -976 regression), the decbench ablation measured this NET-POSITIVE (+47 perfect matches, -576 summed GED, 158:54 improved:regressed across 508 sailr binaries) because it only recovers genuine source early-return guards. Per-test opt-out (`option earlyreturn off`) on the datatests it changes keeps the corpus byte-identical.
        self.switch_return = true; // (kuna) DIV-25 default-on. The continuation of earlyreturn (DIV-23) to WIDE multi-way switch-phi returns (`switch { case: v=K; break; } return v` above earlyreturn's 16-in-edge cap -> per-case `return K`); same per-edge const-peel machinery so it inherits earlyreturn's safety (peels only CONSTANT arms, so it cannot cause returndup's variable-return regression). The decbench ablation of the wide-switch delta on top of default earlyreturn-on measured NET-POSITIVE (+2 perfect matches, -107 summed GED, 3:0 improved:regressed across 17 sailr binaries, zero regressions). Per-test opt-out (`option switchreturn off`) on the datatests it changes keeps the corpus byte-identical.
        self.recover_loop_break = true; // (kuna) DIV-10 default-on (angr break/continue recovery; scopeBreak port)
        self.fold_call_returns = true; // (kuna) DIV-13 default-on (angr call-return folding; per-test opt-out on the datatests it changes)
        self.strip_security_check = true; // (kuna) DIV-82 default-on: REMOVES CODE (strips rustc's bounds/slice/divide-by-zero panic branches, the SEFCOM Oxidizer SecurityCheckRemover port). Name-triggered on seven Rust-only `core::panicking`/`core::slice::index`/`core::str` helpers, so it is structurally inert on a C binary: 0/675 datatests and 0 changed lines over the C fixtures
        self.strip_stack_guard = true; // (kuna) DIV-14 default-on: REMOVES CODE (strips the -fstack-protector canary epilogue). Per-test opt-out (`option stackguard off`) on the 2 Partial-splitting datatests keeps the corpus byte-identical
        self.branch_flip = true; // (kuna) DIV-13 default-on (angr negated-guard branch flipping; per-test opt-out on the datatests it changes)
        self.name_style_angr = true; // (kuna) default-on: angr-style default naming
        self.dedup_var_decls = true; // (kuna) DIV-7 default-on: collapse duplicate local decls (angr)
        self.realtypes = true; // (kuna) DIV-6 default-on: real C types for unknowns
        self.ctypes = false; // (kuna) DIV-75: default-OFF in the catalog because the datatest corpus pins `int4`/`float8` spellings in 42 assertions; ON in the `aggressive` preset, which `auto` selects under 500 KiB, so valid C is the default RENDERING everywhere a real binary is decompiled
        self.framelayout = true; // (kuna) DIV-97: JSON-surface only (no p-code, no emitted C), so the 675-assertion datatest corpus cannot observe it; measured +1,027 type_match-perfect / -1 over 82,035 decbench functions
        self.voidtailreturn = false; // (kuna) option voidtailreturn; default-OFF until its corpus bidirectional sweep is recorded in a DIV row
        self.cortexmpriv = false; // (kuna) DIV-99: default-OFF -- "the core is privileged" is a modelling judgement, not a proof (Cortex-M Thread mode can run unprivileged); ON in the `aggressive` preset, which `auto` selects under 500 KiB, so it is the default rendering for real firmware
        self.ptrdepthcap = false; // (kuna) DIV-108: default-OFF in the catalog because it changes INFERRED types and the datatest corpus pins the upstream spellings; ON in the `aggressive` preset, which `auto` selects under 500 KiB, so the cap is the default rendering for every real binary
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
        self.analysis_peimportcall = true; // (kuna) DIV-57 PE import-call binding default-on
        self.analysis_libproto = true;
        // (kuna) DIV-65 measured libc signature extension — default-ON.
        self.analysis_libcsigs = true;
        self.analysis_strings = true;
        self.analysis_widestrings = true; // (kuna) DIV-110: the StringsAnalyzer `allCharWidths` 2-byte width default-ON (a wide literal was read as its own first character)
        self.analysis_entry_disc = true;
        // (kuna) Unmapped-CALL-target entry suppression -- default-ON (it only ever
        // withholds an entry the walk already refused to decode).
        self.analysis_unmappedentry = true;
        // (kuna) PPC64 ELFv2 local-entry entry suppression -- default-ON (it only
        // ever withholds the duplicate second entry over a function whose global
        // entry is already a seed, so no body can be lost).
        self.analysis_ppclocalentry = true;
        // (kuna) PIC base-register folding in the xref index -- default-ON. It is
        // a query surface only (no p-code, no emitted C), so no parity gate can
        // observe it; it only ever ADDS an edge, and only one it can prove.
        self.analysis_picbase = true;
        // (kuna) PE CRT entry-function prototype recovery -- default-ON.
        self.analysis_entrymainproto = true;
        // (kuna) Mach-O `LC_MAIN` entry naming + prototype -- default-ON (DIV-111).
        self.analysis_machomain = true;
        // (kuna) `.eh_frame` LSDA landing-pad discovery — default-OFF (opt-in,
        // output-changing: adds the discovered exception landing pads as entries).
        self.analysis_eh_frame_full = false;
        // (kuna) DIV-61 `.eh_frame` FDE-interior entry suppression — default-ON.
        self.analysis_fdeinterior = true;
        self.analysis_funcstart_patterns = false; // full byte-pattern starts default-off (output-changing)
        self.analysis_cortexmvectors = false; // (kuna) widened Cortex-M vector signature default-off (output-changing)
        self.analysis_ptrentry = false; // (kuna) pointer-referenced ARM entries default-off (output-changing)
        self.analysis_poolentry = false; // (kuna) ARM literal-pool inference default-off
        self.analysis_arm_markers = true;
        self.analysis_mips_gp = true;
        self.analysis_i386_pie_plt = true; // (kuna) i386-PIE PLT decode default-on (angr)
        self.analysis_relocrebase = true; // (kuna) DIV-79 relocatable-object analysis rebase default-ON (GH-289)
        self.analysis_dynrelocs = true; // (kuna) DIV-84 linked-image dynamic relocations default-ON
        self.analysis_pdatachained = true; // (kuna) DIV-117 GH-403: a chained-UNWIND_INFO .pdata record is an interior chunk, not a function
        self.analysis_symbolnamerepair = true; // (kuna) DIV: degenerate-symbol-name repair default-ON (it only fires where the load would otherwise fail outright)
        self.analysis_symbolnamechars = crate::kuna_symbolnamechars::NameChars::Safe; // (kuna) DIV-94: symbol-name sanitizing defaults to `safe` -- the structural set only, a measured no-op on every name a real toolchain emits
        self.analysis_symbolnamebound = Some(crate::kuna_symbolnamebound::DEFAULT_SCOPE_DEPTH); // (kuna) DIV-95 GH-338: symbol-name scope bound default 256 (3.2x the deepest :: nesting found in any real binary measured, 79; unbounded, one name turns 600 KB of .strtab into 292 MB)
        self.analysis_msvcfpconst = true; // (kuna) DIV-96 MSVC `__real@` FP-constant recovery default-ON (the mangled name IS the datum)
        self.analysis_mips_isa = true;
        self.analysis_dwarf = true;
        self.analysis_datasyms = true; // (kuna) DIV-76 ELF data-symbol naming default-ON (the DIV-26 arm, now gated)
        self.analysis_dwarf_lines = false; // (kuna) source-line comments default-OFF (output-changing, opt-in)
        self.analysis_typedepth = true; // (kuna) DIV: full-depth DWARF type resolution default-ON (the depth budget truncated ordinary C declarations to void; real-ELF DWARF path only, so every parity gate is byte-identical)
        self.analysis_dwarfstructs = true; // (kuna) DIV: DWARF aggregate-layout import default-ON (a zero-size aggregate is not conservative -- the ABI classifier acts on it; real-ELF DWARF path only, so every parity gate is byte-identical)
        self.analysis_dwarfvariants = true; // (kuna) DIV-87: DWARF variant-part import default-ON (the compiler states the discriminant; real-ELF DWARF path only, so every parity gate is byte-identical)
        self.analysis_cppproto = true; // (kuna) DIV: DWARF C++ prototype arm default-ON (recovers ground truth the name-only walk drops; real-ELF DWARF path only, so every parity gate is byte-identical)
        // (kuna) DIV: demangled C++ signatures default to the PROVEN tier — only
        // the prototypes the mangling entails (ctor/dtor/cv-qualified member/
        // unqualified global), measured at precision 1.0000 on google/leveldb.
        // Real-object path only, so every parity gate is byte-identical.
        self.analysis_cppsig = crate::kuna_cppsig::CppSigMode::Proven;
        self.analysis_callfixup = true;
        self.analysis_addrtable = false; // Ghidra AddressTableAnalyzer default-off
        self.analysis_operand_refs = false; // Ghidra ScalarOperandAnalyzer !isElf default-off
        self.analysis_formatstring = false; // Ghidra FormatStringAnalyzer default-off
        self.analysis_listing = false; // Listing/xref tier default-off
        self.analysis_fast_funcdisc = false; // bounded whole-project discovery default-off
        self.analysis_noreturn_disc = true; // (kuna) DIV-22 default-on: Ghidra's FindNoReturnFunctionsAnalyzer ≥3-evidence discovered-no-return (default-on in Ghidra). REMOVES CODE (marks a callee no-return from ≥3 dead-fall-through sites → drops post-call dead code at callers). Gated on the Listing (default-off), so every parity gate is byte-identical (real-ELF Listing path only); restore with `option noreturn_disc off`
        self.analysis_noreturn_discstrict = true; // (kuna, GH-312) DIV-92 default-on: drop noreturn_disc's decode-failure evidence arm, keeping the terminal arm + the two positive arms. RESTORES CODE (a forged no-return verdict no longer deletes the caller's tail). Gated on the Listing (default-off), so every parity gate is byte-identical (real-ELF Listing path only); restore the legacy three-arm tally with `option noreturn_discstrict off`
        self.analysis_noreturn_propagate = true; // (kuna) DIV-14 default-on: REMOVES CODE (call-graph no-return propagation drops post-call dead code). Gated on the Listing (default-off), so every parity gate is byte-identical (real-ELF Listing path only); restore with `option noreturn_propagate off`
        self.analysis_noreturn_error = true; // (kuna) DIV-16 default-on: REMOVES CODE (conclude error(nonzero,...) wrappers no-return, dropping the dead fall-through at every caller). Sub-rule of noreturn_propagate, gated on the Listing (default-off), so every parity gate is byte-identical (real-ELF Listing path only); restore with `option noreturn_error off`
        self.analysis_noreturn_reach = true; // (kuna) DIV-19 default-on: REMOVES CODE (CFG-reachability no-return, Ghidra's FindNoReturnFunctionsAnalyzer.targetOnlyCallsNoReturn — mid-body no-return calls, dead returns, switch-of-no-return). Sub-rule of noreturn_propagate, gated on the Listing (default-off), so every parity gate is byte-identical (real-ELF Listing path only); restore with `option noreturn_reach off`
        self.analysis_fid = false; // FID fingerprint matcher consumer default-off
        self.analysis_rtti = false; // MSVC RTTI / vftable recovery default-off (PE-only, output-changing)
        self.analysis_itaniumrtti = false; // (kuna, NOVEL) Itanium RTTI / vtable recovery default-off (ELF-only, output-changing)
        self.analysis_aif = false; // Aggressive Instruction Finder gap-walk default-off
        // (kuna, GH-299) AIF gap-cursor aligned slide — default-OFF (it REMOVES
        // entries), carried by the `aggressive` preset.
        self.analysis_aifstrict = false;
        // (kuna, GH-313) AIF corroboration test — default-OFF (it REMOVES entries),
        // carried by the `aggressive` preset.
        self.analysis_aifcorroborate = false;
        self.analysis_tailcallentry = false; // tail-call function-entry recovery default-off
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
            "truthycond" => {
                let (val, msg) = crate::kuna_truthycond::OptionTruthyCond.apply(p1)?;
                self.print_mut().options.set_truthy_cond(val);
                Ok(msg)
            }
            "braceelide" => {
                let (val, msg) = crate::kuna_braceelide::OptionBraceElide.apply(p1)?;
                self.print_mut().options.set_brace_elide(val);
                Ok(msg)
            }
            "warnstyle" => {
                let (val, msg) = crate::kuna_warnstyle::OptionWarnStyle.apply(p1)?;
                self.print_mut().options.set_warn_inline(val);
                Ok(msg)
            }
            "arraycoverwidth" => {
                let (val, msg) =
                    crate::kuna_arraycoverwidth::OptionArrayCoverWidth.apply(p1)?;
                self.print_mut().options.set_array_cover_width(val);
                Ok(msg)
            }
            "emptystrconst" => {
                let (val, msg) = crate::kuna_emptystrconst::OptionEmptyStrConst.apply(p1)?;
                self.print_mut().options.set_empty_str_const(val);
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
            "retsplitglobal" => {
                let (val, msg) =
                    crate::p8_structure::kuna_retsplitglobal::OptionRetSplitGlobal.apply(p1)?;
                self.ret_split_global = val;
                Ok(msg)
            }
            "simdlane" => {
                let (val, msg) = crate::p3_dataflow::kuna_simdlane::OptionSimdLane.apply(p1)?;
                self.simd_lane_fold = val;
                Ok(msg)
            }
            "flagcompare" => on_off!(fold_flag_compare, "Flag-modelled comparison folding"),
            "v850indirectbranch" => on_off!(v850_indirect_branch, "V850 indirect-branch reclassification"),
            "fastfailnoreturn" => on_off!(fastfail_noreturn, "Windows int 0x29 (__fastfail) no-return"),
            "msvcftol" => on_off!(msvc_ftol, "MSVC __ftol-family call-fixup"),
            "tailcalljump" => on_off!(tail_call_jumps, "Tail-call jump recovery"),
            "tailcallframe" => on_off!(tail_call_frame, "Frame-teardown tail-call recovery"),
            "funcboundflow" => on_off!(funcbound_flow, "Fall-through bound at function entries"),
            "overlapbranch" => on_off!(overlap_branch, "Overlapping-branch fall-through truncation"),
            "cleanupcode" => on_off!(remove_cleanup_code, "Rust drop/deallocate call removal"),
            "linuxsyscall" => on_off!(linux_syscall, "Linux int 0x80 syscall naming"),
            "switchselector" => on_off!(switch_selector_guard, "Lowered-switch selector soundness guard"),
            "noreturn_extern" => on_off!(noreturn_extern_calls, "Name-based extern no-return"),
            "inputvarnodeadjust" => on_off!(input_varnode_adjust, "Overlapping input-varnode adjustment"),
            "retinputhalf" => on_off!(ret_input_half, "Returned input-parameter half retention"),
            "noreturnretuse" => on_off!(noreturn_ret_use, "No-return call argument use in return trials"),
            "rustabi" => {
                let (mode, msg) = crate::kuna_rustabi::parse_rust_abi_mode(p1)?;
                self.rust_abi = mode.as_u8();
                Ok(msg)
            }
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
            "rodatastring" => {
                let (form, msg) = crate::kuna_rodatastring::parse_rodata_string_form(p1)?;
                self.rodata_string = form.rodata_string();
                Ok(msg)
            }
            "switchmodbound" => on_off!(switch_modulo_bound, "Switch modulo/and-mask index bound"),
            "switchguardbound" => on_off!(switch_guard_bound, "Switch CBRANCH-guard index bound"),
            "switchsharedcase" => on_off!(switch_shared_case, "Switch loop-carried-guard table"),
            "switchmultipred" => on_off!(switch_multi_pred, "Switch multi-predecessor unrolled-guard table"),
            "unrolledguard" => on_off!(unrolled_guard, "Interleaved unrolled-guard jump-table partial-flow recovery"),
            "jtsharepartial" => on_off!(jumptable_share_partial, "Shared jump-table partial sub-decompilation"),
            "noreturn_externmatch" => on_off!(noreturn_extern_match, "Name-matched extern no-return"),
            "loweredswitch" => {
                let (val, msg) = crate::kuna_loweredswitch::OptionLowerSwitch.apply(p1)?;
                self.recover_lowered_switch = val;
                Ok(msg)
            }
            "callsitestackargs" => {
                let (val, msg) =
                    crate::p4_calls::kuna_callsitestackargs::OptionCallsiteStackArgs.apply(p1)?;
                self.callsite_stack_args = val;
                Ok(msg)
            }
            "cookiescramble" => {
                let (val, msg) =
                    crate::p6_variables::kuna_cookiescramble::OptionCookieScramble.apply(p1)?;
                self.cookie_scramble = val;
                Ok(msg)
            }
            "inputparamgap" => {
                let (val, msg) =
                    crate::p4_calls::kuna_inputparamgap::OptionInputParamGap.apply(p1)?;
                self.input_param_gap = val;
                Ok(msg)
            }
            "calleedeadarg" => {
                let (val, msg) =
                    crate::p4_calls::kuna_calleedeadarg::OptionCalleeDeadArg.apply(p1)?;
                self.callee_dead_arg = val;
                Ok(msg)
            }
            "calleepreserves" => {
                let (val, msg) =
                    crate::p4_calls::kuna_calleepreserves::OptionCalleePreserves.apply(p1)?;
                self.callee_preserves = val;
                Ok(msg)
            }
            "varargstackargs" => {
                let (val, msg) =
                    crate::p4_calls::kuna_varargstackargs::OptionVarargStackArgs.apply(p1)?;
                self.vararg_stack_args = val;
                Ok(msg)
            }
            "calleearitylive" => {
                let (val, msg) =
                    crate::p4_calls::kuna_calleearitylive::OptionCalleeArityLive.apply(p1)?;
                self.callee_arity_live = val;
                Ok(msg)
            }
            "calleearity" => {
                let (val, msg) =
                    crate::p4_calls::kuna_calleearity::OptionCalleeArity.apply(p1)?;
                self.callee_arity = val;
                Ok(msg)
            }
            "calleearityfwd" => {
                let (val, msg) =
                    crate::p4_calls::kuna_calleearityfwd::OptionCalleeArityFwd.apply(p1)?;
                self.callee_arity_fwd = val;
                Ok(msg)
            }
            "calloverlap" => {
                let (val, msg) =
                    crate::p3_dataflow::kuna_calloverlap::OptionCallOverlap.apply(p1)?;
                self.call_overlap = val;
                Ok(msg)
            }
            "spillargtrial" => {
                let (val, msg) =
                    crate::p4_calls::kuna_spillargtrial::OptionSpillArgTrial.apply(p1)?;
                self.spill_arg_trial = val;
                Ok(msg)
            }
            "loadguardrange" => on_off!(load_guard_range, "Indexed-stack guard ValueSet range refinement"),
            "tiedstorekeep" => {
                on_off!(tied_store_keep, "Address-tied store copy-propagation brake")
            }
            "regionstructure" => {
                let (val, msg) =
                    crate::p8_structure::region_structurer::OptionRegionStructure.apply(p1)?;
                self.region_structure = val;
                Ok(msg)
            }
            "guardarm" => {
                let (val, msg) =
                    crate::p8_structure::kuna_ifnoexit::OptionGuardArm.apply(p1)?;
                self.guard_arm = val;
                Ok(msg)
            }
            "loopcondhoist" => {
                let (val, msg) =
                    crate::p8_structure::kuna_ifnoexit::OptionLoopCondHoist.apply(p1)?;
                self.loop_cond_hoist = val;
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
            "outline" => {
                let (val, msg) =
                    crate::p8_structure::kuna_outline::OptionOutline.apply(p1)?;
                self.outline_spec = val;
                Ok(msg)
            }
            "condfold" => {
                let (val, msg) =
                    crate::p8_structure::kuna_condfold::OptionCondFold.apply(p1)?;
                self.cond_fold = val;
                Ok(msg)
            }
            "gotoreduce" => {
                let (val, msg) =
                    crate::p8_structure::kuna_gotoreduce::OptionGotoReduce.apply(p1)?;
                self.reduce_return_gotos = val;
                Ok(msg)
            }
            "ifelseflatten" => {
                let (val, msg) =
                    crate::p8_structure::kuna_ifelseflatten::OptionIfElseFlatten.apply(p1)?;
                self.flatten_ifelse = val;
                Ok(msg)
            }
            "crossjumprevert" => {
                let (val, msg) =
                    crate::p8_structure::kuna_crossjumpreverter::OptionCrossJumpReverter.apply(p1)?;
                self.revert_cross_jumps = val;
                Ok(msg)
            }
            "taildup" => {
                let (val, msg) = crate::p8_structure::kuna_taildup::OptionTailDup.apply(p1)?;
                self.dup_return_call_tails = val;
                Ok(msg)
            }
            "dedupitetail" => {
                let (val, msg) =
                    crate::p8_structure::kuna_dedupitetail::OptionDedupIteTail.apply(p1)?;
                self.dedup_ite_tail = val;
                Ok(msg)
            }
            "iteexpr" => on_off!(iteexpr, "Computed-expression arm ?: recovery (iteregion extension)"),
            "evalcurrentproto" => {
                let (val, msg) =
                    crate::kuna_evalcurrentproto::OptionEvalCurrentProto.apply(p1)?;
                self.evalcurrentproto = val;
                Ok(msg)
            }
            "iteboolean" => {
                let (val, msg) =
                    crate::p8_structure::kuna_iteboolean::OptionIteBoolean.apply(p1)?;
                self.iteboolean = val;
                Ok(msg)
            }
            "itecondlist" => {
                let (val, msg) =
                    crate::p8_structure::kuna_itecondlist::OptionIteCondList.apply(p1)?;
                self.itecondlist = val;
                Ok(msg)
            }
            "paramcopyhoist" => {
                let (val, msg) =
                    crate::p6_variables::kuna_paramcopyhoist::OptionParamCopyHoist.apply(p1)?;
                self.param_copy_hoist = val;
                Ok(msg)
            }
            "iteregion" => {
                let (val, msg) = crate::p8_structure::kuna_iteregion::OptionIteRegion.apply(p1)?;
                self.iteregion = val;
                Ok(msg)
            }
            "returndup" => {
                let (val, msg) =
                    crate::p8_structure::kuna_returndup::OptionReturnDup.apply(p1)?;
                self.duplicate_shared_returns = val;
                Ok(msg)
            }
            "orchain" => {
                let (val, msg) = crate::p8_structure::kuna_orchain::OptionOrChain.apply(p1)?;
                self.returndup_orchain = val;
                Ok(msg)
            }
            "earlyreturn" => {
                let (val, msg) =
                    crate::p8_structure::kuna_earlyreturn::OptionEarlyReturn.apply(p1)?;
                self.early_return = val;
                Ok(msg)
            }
            "switchreturn" => {
                let (val, msg) =
                    crate::p8_structure::kuna_switchreturn::OptionSwitchReturn.apply(p1)?;
                self.switch_return = val;
                Ok(msg)
            }
            "foldcallret" => {
                let (val, msg) = crate::kuna_callretfold::OptionFoldCallRet.apply(p1)?;
                self.fold_call_returns = val;
                Ok(msg)
            }
            "stackguard" => on_off!(strip_stack_guard, "Stack-guard canary stripping"),
            "securitycheck" => {
                on_off!(strip_security_check, "Rust security-check branch stripping")
            }
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
            "ctypes" => on_off!(ctypes, "valid-C core type spelling"),
            "framelayout" => on_off!(framelayout, "recovered stack-frame reporting"),
            "voidtailreturn" => on_off!(voidtailreturn, "void tail-return elision"),
            "ptrdepthcap" => on_off!(ptrdepthcap, "inferred pointer-nesting cap"),
            "cortexmpriv" => on_off!(cortexmpriv, "Cortex-M privileged-mode guard folding"),
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
            "peimportcall" => on_off!(analysis_peimportcall, "PE import-call binding"),
            "libproto" => on_off!(analysis_libproto, "Library-prototype analysis pass"),
            "libcsigs" => on_off!(analysis_libcsigs, "Measured libc signature extension"),
            "strings" => on_off!(analysis_strings, "String-literal analysis pass"),
            "widestrings" => {
                on_off!(analysis_widestrings, "UTF-16LE width of the string-literal pass")
            }
            "entry_disc" => on_off!(analysis_entry_disc, "Entry-discovery analysis pass"),
            "unmappedentry" => {
                on_off!(analysis_unmappedentry, "Unmapped-CALL-target entry suppression")
            }
            "ppclocalentry" => {
                on_off!(analysis_ppclocalentry, "PPC64 ELFv2 local-entry entry suppression")
            }
            "picbase" => {
                on_off!(analysis_picbase, "PIC base-register folding in the xref index")
            }
            "entrymainproto" => {
                on_off!(analysis_entrymainproto, "PE CRT entry-function prototype recovery")
            }
            "machomain" => {
                on_off!(analysis_machomain, "Mach-O LC_MAIN entry naming + prototype")
            }
            "eh_frame_full" => {
                on_off!(analysis_eh_frame_full, ".eh_frame LSDA landing-pad discovery")
            }
            "fdeinterior" => {
                on_off!(analysis_fdeinterior, ".eh_frame FDE-interior entry suppression")
            }
            "funcstart_patterns" => {
                on_off!(analysis_funcstart_patterns, "Full byte-pattern function-start pass")
            }
            "cortexmvectors" => {
                on_off!(analysis_cortexmvectors, "Widened ARM Cortex-M vector-table signature")
            }
            "ptrentry" => {
                on_off!(analysis_ptrentry, "Pointer-referenced ARM function entries")
            }
            "poolentry" => {
                on_off!(
                    analysis_poolentry,
                    "ARM literal-pool inference (entry recall + AIF phantom suppression)"
                )
            }
            "arm_markers" => on_off!(analysis_arm_markers, "ARM/Thumb decode-mode marker pass"),
            "mips_gp" => on_off!(analysis_mips_gp, "MIPS $gp-recovery (t9 tracking) pass"),
            // (kuna) Loader-tier gate: also bridge to the env var the loader reads
            // (the PLT map is baked at `load file`, upstream of this `option`), so
            // an `option i386_pie_plt off` *before* `load file` in the same process
            // takes effect. The CLI sets the env directly on the subprocess too.
            "ifuncfpret" => {
                let val = on_or_off(p1)?;
                self.analysis_ifuncfpret = val;
                crate::kuna_ifuncfpret::set_ifuncfpret_env(val);
                Ok(format!(
                    "IFUNC PLT-stub naming turned {}",
                    if val { "on" } else { "off" }
                ))
            }
            "i386_pie_plt" => {
                let val = on_or_off(p1)?;
                self.analysis_i386_pie_plt = val;
                crate::kuna_i386_pie_plt::set_i386_pie_plt_env(val);
                Ok(format!(
                    "i386-PIE PLT-stub decode turned {}",
                    if val { "on" } else { "off" }
                ))
            }
            // (kuna) Load-time gate: the analyzer tier runs inside `load file`, so
            // bridge to the env var it reads (the CLI sets it on the subprocess too).
            "relocrebase" => {
                let val = on_or_off(p1)?;
                self.analysis_relocrebase = val;
                crate::kuna_relocrebase::set_relocrebase_env(val);
                Ok(format!(
                    "relocatable-object analysis rebase turned {}",
                    if val { "on" } else { "off" }
                ))
            }
            "dynrelocs" => {
                let val = on_or_off(p1)?;
                self.analysis_dynrelocs = val;
                crate::kuna_dynrelocs::set_dynrelocs_env(val);
                Ok(format!(
                    "linked-image dynamic-relocation application turned {}",
                    if val { "on" } else { "off" }
                ))
            }
            "pdatachained" => {
                let val = on_or_off(p1)?;
                self.analysis_pdatachained = val;
                crate::kuna_pdatachained::set_pdatachained_env(val);
                Ok(format!(
                    "PE chained-UNWIND_INFO .pdata entry suppression turned {}",
                    if val { "on" } else { "off" }
                ))
            }
            // (kuna) Load-time gate: the symbol table is installed inside `load
            // file`, so bridge to the env var it reads (the CLI sets it on the
            // subprocess too).
            "symbolnamerepair" => {
                let val = on_or_off(p1)?;
                self.analysis_symbolnamerepair = val;
                crate::kuna_symbolnamerepair::set_symbolnamerepair_env(val);
                Ok(format!(
                    "degenerate-symbol-name repair turned {}",
                    if val { "on" } else { "off" }
                ))
            }
            // (kuna) Load-time gate: symbol names are minted inside `load file`,
            // so bridge the choice to the env var the loader reads (the CLI sets
            // it on the subprocess too).
            "symbolnamechars" => {
                let mode = crate::kuna_symbolnamechars::NameChars::parse(p1).ok_or_else(|| {
                    KunaError::lowlevel(format!(
                        "symbolnamechars must be off|safe|ident, got `{p1}`"
                    ))
                })?;
                self.analysis_symbolnamechars = mode;
                crate::kuna_symbolnamechars::set_symbolnamechars_env(mode);
                Ok(format!("symbol-name character sanitizing set to {}", mode.as_str()))
            }
            // (kuna) Load-time gate on the same seam, and VALUED: the scope
            // ceiling is a number, so `on_or_off` does not apply.
            "symbolnamebound" => {
                let (bound, msg) = crate::kuna_symbolnamebound::parse_symbolnamebound(p1)?;
                self.analysis_symbolnamebound = bound;
                crate::kuna_symbolnamebound::set_symbolnamebound_env(bound);
                Ok(msg)
            }
            // (kuna) Load-time gate: the constant bytes are materialised inside
            // `load file`, so bridge to the env var the loader reads (the CLI
            // sets it on the subprocess too).
            "msvcfpconst" => {
                let val = on_or_off(p1)?;
                self.analysis_msvcfpconst = val;
                crate::kuna_msvcfpconst::set_msvcfpconst_env(val);
                Ok(format!(
                    "MSVC __real@ FP-constant recovery turned {}",
                    if val { "on" } else { "off" }
                ))
            }
            "mips_isa" => on_off!(analysis_mips_isa, "MIPS16 ISA_MODE decode-mode marker pass"),
            "dwarf" => on_off!(analysis_dwarf, "DWARF recovery analysis pass"),
            "datasyms" => {
                on_off!(analysis_datasyms, "ELF data-symbol (STT_OBJECT) naming")
            }
            "dwarf_lines" => {
                on_off!(analysis_dwarf_lines, "DWARF .debug_line source-line comment pass")
            }
            "cppproto" => {
                on_off!(analysis_cppproto, "DWARF C++ prototype recovery arm")
            }
            // (kuna) Load-time gate: also bridge to the env var the type mapper
            // reads (the DWARF types are baked at `load file`, upstream of this
            // `option`), so an `option typedepth off` *before* `load file` in the
            // same process takes effect. The CLI sets the env directly too.
            "typedepth" => {
                let val = on_or_off(p1)?;
                self.analysis_typedepth = val;
                crate::kuna_typedepth::set_typedepth_env(val);
                Ok(format!(
                    "Full-depth DWARF type resolution turned {}",
                    if val { "on" } else { "off" }
                ))
            }
            // (kuna) Load-time gate, same env bridge as `typedepth` above: the
            // aggregate layout is installed on the interned type at `load file`,
            // upstream of this `option`.
            "dwarfstructs" => {
                let val = on_or_off(p1)?;
                self.analysis_dwarfstructs = val;
                crate::kuna_dwarfstructs::set_dwarfstructs_env(val);
                Ok(format!(
                    "DWARF aggregate-layout import turned {}",
                    if val { "on" } else { "off" }
                ))
            }
            // (kuna) Load-time gate, same env bridge as `dwarfstructs` above: the
            // variant overlay is installed on the interned type at `load file`,
            // upstream of this `option`.
            "dwarfvariants" => {
                let val = on_or_off(p1)?;
                self.analysis_dwarfvariants = val;
                crate::kuna_dwarfvariants::set_dwarfvariants_env(val);
                Ok(format!(
                    "DWARF variant-part import turned {}",
                    if val { "on" } else { "off" }
                ))
            }
            "cppsig" => {
                let (mode, msg) = crate::kuna_cppsig::parse_cppsig_mode(p1)?;
                self.analysis_cppsig = mode;
                Ok(msg)
            }
            "callfixup" => on_off!(analysis_callfixup, "Call-fixup analysis pass"),
            "addrtable" => on_off!(analysis_addrtable, "Address-table analysis pass"),
            "operand_refs" => on_off!(analysis_operand_refs, "Scalar/operand reference-markup pass"),
            "formatstring" => {
                on_off!(analysis_formatstring, "Format-string varargs-typing pass")
            }
            "listing" => on_off!(analysis_listing, "Listing/xref disassembly tier"),
            "fast_funcdisc" => {
                on_off!(analysis_fast_funcdisc, "Fast whole-project function discovery")
            }
            "noreturn_disc" => {
                on_off!(analysis_noreturn_disc, "Discovered-no-return Listing consumer")
            }
            "noreturn_discstrict" => {
                on_off!(
                    analysis_noreturn_discstrict,
                    "Discovered-no-return positive-evidence-only tally"
                )
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
            "itaniumrtti" => {
                on_off!(analysis_itaniumrtti, "Itanium (GCC/Clang) RTTI / vtable recovery pass")
            }
            "aif" => {
                on_off!(analysis_aif, "Aggressive Instruction Finder gap-walk Listing consumer")
            }
            "aifstrict" => {
                on_off!(analysis_aifstrict, "AIF gap-cursor aligned slide (GH-299)")
            }
            "aifcorroborate" => {
                on_off!(analysis_aifcorroborate, "AIF accept corroboration test (GH-313)")
            }
            "tailcallentry" => {
                on_off!(analysis_tailcallentry, "Tail-call function-entry recovery Listing consumer")
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

    /// Apply a named, concrete decompiler *mode* preset (`reliable` |
    /// `aggressive` | `fast`): a batch of `(option, value)` overrides fanned out
    /// through [`Self::set_kuna_option`]. Overrides apply in table order; any
    /// later `set_kuna_option` -- another override or a user
    /// `option`/`--option` -- wins (last-write). `reliable` is the shipped
    /// defaults (empty override list, a no-op alias); `aggressive` turns on
    /// every off-by-default pass; `fast` disables expensive whole-program
    /// decode and discovery.
    /// See [`crate::modes`]. The frontend-only `auto` policy errors here because
    /// an Architecture has no input-file size; file frontends resolve it to a
    /// concrete preset before this method. Errors on an unknown mode name.
    pub fn apply_mode(&mut self, name: &str) -> KunaResult<String> {
        if crate::modes::mode_is_automatic(name) {
            return Err(KunaError::parse(
                "Decompiler mode auto requires input binary size; resolve it in a file frontend",
            ));
        }
        let overrides = crate::modes::mode_overrides(name).ok_or_else(|| {
            let known: Vec<&str> = crate::modes::mode_names().collect();
            KunaError::parse(format!(
                "Unknown decompiler mode: {name} (known: {})",
                known.join(", ")
            ))
        })?;
        for (opt, val) in overrides {
            self.set_kuna_option(opt, val)?;
        }
        Ok(format!(
            "mode {name} applied ({} option override{})",
            overrides.len(),
            if overrides.len() == 1 { "" } else { "s" }
        ))
    }

    /// Reset options modifiable by the OptionDatabase, including the action
    /// database (C++ `Architecture::resetDefaults`, `architecture.cc:1463`).
    ///
    /// STUB(W5/W8): the C++ also calls `allacts.resetDefaults()` (the
    /// `ActionDatabase` default-group reset, a W5 surface not yet exposed) and
    /// resets every `PrintLanguage` in `printlist` (W8).  Only the internal
    /// option reset runs here; the action/print resets land with their waves.
    pub fn reset_defaults(&mut self) {
        self.reset_defaults_internal();
        // allacts.resetDefaults();                                 -- STUB(W5)
        // for printlang in printlist: printlang.reset_defaults();  -- STUB(W8)
    }

    /// (kuna, Phase 3) The ghidra-mode `SetOptions` reset (the upstream
    /// `ghidra->resetDefaults()` at the top of `SetOptions::rawAction`,
    /// ghidra_process.cc:435-445): Java DELTA-encodes its option list — only
    /// values differing from the Java default constants travel — so an option
    /// previously sent as non-default must revert when the user sets it back
    /// to default.  Lands every setOptions on the same baseline a fresh
    /// registerProgram produces (the respawn-replay equivalence Java
    /// assumes); the caller re-applies the DIV-77 ghidra-mode preset layer on
    /// top before decoding the list.
    ///
    /// Covered: the engine option defaults (`reset_defaults` →
    /// `reset_defaults_internal`), the printer's context state (integer
    /// format, comment indent + header/instruction flags, namespace strategy,
    /// language — a fresh `PrintContext`), and the PrintC-proper option block
    /// + emitter indent increment
    /// (`PrintC::reset_wire_option_defaults`: nullprinting, inplaceops,
    /// conventionprinting, nocastprinting, hideimpliedexts, the brace
    /// formats, indentincrement).  NOT covered: the action-database
    /// default-group reset (the shared STUB(W5) of [`Self::reset_defaults`] —
    /// the `currentaction` option toggles accumulate for the session), and
    /// `maxlinewidth`/`commentstyle`, which are recorded-no-op printer stubs
    /// with no state to reset.
    pub fn reset_wire_defaults(&mut self) {
        self.reset_defaults();
        self.print.context = crate::printlanguage::PrintContext::new();
        self.print.reset_wire_option_defaults();
    }

    // -----------------------------------------------------------------------
    // Address-space access (C++ Architecture is-a AddrSpaceManager)
    // -----------------------------------------------------------------------

    /// Borrow the address-space manager (C++ `this` viewed as an
    /// `AddrSpaceManager`); forwarded to the owned `Sleigh` engine's
    /// `SleighBase`.
    pub fn manage(&self) -> &AddrSpaceManager {
        self.translate.manager()
    }

    /// Borrow the disassembly engine (C++ `translate`) through the
    /// [`EngineTranslate`] boundary.  External callers reach the `Translate` /
    /// `RegisterLookup` surface directly; those needing the concrete standalone
    /// engine downcast via [`EngineTranslate::as_sleigh`].
    pub fn translate(&self) -> &dyn EngineTranslate {
        &*self.translate
    }

    /// Mutably borrow the disassembly engine.
    pub fn translate_mut(&mut self) -> &mut dyn EngineTranslate {
        &mut *self.translate
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

    /// (kuna, Phase 3) Install the ghidra-mode lazy providers over the wire
    /// fetch seams (the `buildDatabase`/`buildTypegrp` `ScopeGhidra`/
    /// `TypeFactoryGhidra` analog, ghidra_arch.cc:301-314).  Must run AFTER
    /// `init_post_engine` (the cspec `<global>` ranges and pspec property
    /// paints must be in the Database — the C++ `lockDefaultProperties` at
    /// `postSpecFile`) and BEFORE the first decompile.
    pub fn install_remote_provider(
        &mut self,
        fetch: Rc<dyn crate::remote_provider::RemoteProviderFetch>,
        type_fetch: Option<Rc<dyn crate::dtype::RemoteTypeFetch>>,
    ) {
        let mut models = std::collections::BTreeMap::new();
        for (name, model) in &self.proto_models {
            models.insert(name.clone(), Rc::clone(model));
        }
        let base = self.symboltab.build_global_query();
        let scope = crate::remote_provider::RemoteScope::new(
            fetch,
            self.translate.manager_rc(),
            self.types_rc(),
            models,
            self.defaultfp.clone(),
            base,
        );
        self.remote_scope = Some(Rc::new(scope));
        self.types.set_remote_type_fetch(type_fetch);
    }

    /// (kuna, Phase 3) The printer's active comment filter — the union of the
    /// header and per-instruction comment-type masks (C++
    /// `CommentDatabaseGhidra::fillCache`'s `ghidra->print->getHeaderComment()
    /// | getInstructionComment()`, comment_ghidra.cc:38).  The getComments
    /// query is filtered by this; `0` means no comment types display and no
    /// query fires.
    pub fn printer_comment_filter(&self) -> u32 {
        let ctx = &self.print.context;
        ctx.header_comment() | ctx.instruction_comment()
    }

    /// (kuna, Phase 3) The ghidra-mode flushNative reset, in the upstream order
    /// (`FlushNative::rawAction`, ghidra_process.cc:262-273): the lazy symbol
    /// cache (+ its property-map rollback), the non-core data-types, the
    /// comment database, and the decoded-string cache.  kuna has no live
    /// sub-scopes to delete (namespaces are cached path strings inside the
    /// RemoteScope, cleared with it) and no wired constant pool yet.
    pub fn flush_remote_caches(&mut self) {
        if let Some(remote) = &self.remote_scope {
            remote.clear();
        }
        self.types.clear_noncore();
        self.commentdb.clear();
        self.string_manager.borrow_mut().base.clear();
    }

    /// (kuna, Phase 3) The active fallback naming vocabulary (see
    /// [`KunaNameStyle`](crate::database::KunaNameStyle)): `Ghidra` wins over
    /// `Angr` wins over the upstream `Func` default.
    pub fn kuna_name_style(&self) -> crate::database::KunaNameStyle {
        if self.name_style_ghidra {
            crate::database::KunaNameStyle::Ghidra
        } else if self.name_style_angr {
            crate::database::KunaNameStyle::Angr
        } else {
            crate::database::KunaNameStyle::Func
        }
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
        if self.name_style_ghidra {
            // (kuna, Phase 3) ghidra-mode: FUN_%08x (the Java-side dynamic shape)
            return crate::database::ghidra_function_name(addr);
        }
        if self.name_style_angr {
            // (kuna) angr-style: sub_<addr>
            return crate::database::kuna_function_name(addr);
        }
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

    /// (kuna `cppproto`) Park a recovered prototype on the FunctionSymbol at
    /// `addr`, in whatever scope it lives (`find_function_across_scopes`) — the
    /// address-keyed companion of [`Self::set_function_prototype_pieces`].
    ///
    /// Address is the key the READ side already uses
    /// ([`Database::function_proto_pieces`]), and the only key that survives C++:
    /// a demangled template name is normalized (`maxof<int>` is filed as `maxof`)
    /// and a qualified name lives in a nested scope, so the by-name park silently
    /// misses both.  Faithful to Ghidra, whose `DWARFFunction` is keyed by
    /// `getCodeAddress(dwarfBody.getFirstAddress())`.  A silent no-op when no
    /// FunctionSymbol starts at `addr`.
    pub fn set_function_prototype_pieces_at(
        &mut self,
        addr: &Address,
        pieces: crate::fspec::PrototypePieces,
    ) {
        if let Some((sid, _)) = self.symboltab.find_function_across_scopes(addr) {
            self.symboltab.set_function_proto_pieces(sid, pieces);
        }
    }

    // -----------------------------------------------------------------------
    // Funcdata construction (the W3 boot boundary)
    // -----------------------------------------------------------------------

    /// Build a [`Funcdata`] tied to this architecture (the C++ `Funcdata`
    /// constructor, driven from the architecture).
    ///
    /// The W3 [`Funcdata::new`] needs an [`ArchHandle`] carrying the IR-boundary
    /// address-space slice and the analysis unique-start.  Per the established
    /// W3 boundary shape (and because the lift emits varnodes carrying their *own*
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

    /// Build the [`ArchHandle`] (the [`ArchContext`] the W3 IR holds as `glb`).
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
        let mut ctx = ArchContext::new_shared(manage);
        ctx.min_laned_register_size = self.get_minimum_laned_register_size();
        // Carry the laned-register table so the per-function Funcdata reaches
        // `glb->getLanedRegister` (C++ `Architecture::lanerecords`); cheap clones
        // of the small (size,mask) records.  ActionLaneDivide reads these to
        // split XMM/ZMM vector lanes.
        ctx.lanerecords = self.lanerecords.clone();
        // Share the engine's OpBehavior emulation table with `glb` (the C++
        // `Architecture` owns the `TypeOp`s, so `glb->inst[opc]->getBehavior()`
        // reaches them directly).  The `Rc<dyn OpBehavior>` entries are cheap
        // clones; the IR-transform passes (RuleCollapseConstants) fold constants
        // through `glb.op_behavior(opc)`.
        ctx.opbehaviors = self.opbehaviors.clone();
        // Share the processor's float formats with `glb` (the C++ `Architecture`
        // IS-A `Translate`, so `glb->translate->getFloatFormat` reaches them).
        // `SubfloatFlow` reads them off the per-function `glb` to drive the
        // float-precision narrowing (`RuleSubfloatConvert`); cheap clones of the
        // small format records.
        ctx.floatformats = self.translate.float_formats().to_vec();
        // Share the prototype-model registry handles (C++ `glb->defaultfp` /
        // `evalfp_current`) so the proto-recovery actions can set the function's
        // model and run output recovery against the real param lists.
        ctx.defaultfp = self.defaultfp.clone();
        // An explicit `option protoeval <model>` outranks the spec's own
        // `<eval_current_prototype>` nomination, which reaches the function only
        // under `evalcurrentproto` (see `crate::kuna_evalcurrentproto`). With
        // neither, the handle's own accessor falls back to `defaultfp`, exactly as
        // before the option existed.
        ctx.evalfp_current = self.evalfp_current.clone().or_else(|| {
            self.evalcurrentproto.then(|| self.evalfp_current_spec.clone()).flatten()
        });
        // Carry the cspec's return-address storage (C++ `glb->defaultReturnAddr`)
        // so the per-function `Funcdata::testForReturnAddress` can detect a
        // BRANCHIND that is really a tail return through the return-address
        // register (the Switch-return jump-table failure mode `fail_return`).
        ctx.default_return_addr = self.default_return_addr.clone();
        ctx.trim_recurse_max = self.trim_recurse_max;
        ctx.max_implied_ref = self.max_implied_ref;
        ctx.max_term_duplication = self.max_term_duplication;
        ctx.return_single = self.return_single;
        // (kuna GH-9218) carry the unjustified-input forward-absorb gate so
        // `ActionUnjustifiedParams` reaches it via `glb`.
        ctx.input_varnode_adjust = self.input_varnode_adjust;
        // (kuna) carry the returned-input-half gate so `kuna_returnuncomputed`
        // reaches `option retinputhalf` via `glb`.
        ctx.ret_input_half = self.ret_input_half;
        // (kuna) carry the terminal-no-return trial gate so `only_op_use` reaches
        // `option noreturnretuse` via `glb`.
        ctx.noreturn_ret_use = self.noreturn_ret_use;
        // (kuna) carry the Rust return-ABI gate and the detected source language
        // so `kuna_rustabi` reaches both via `glb`.
        ctx.rust_abi = self.rust_abi;
        ctx.source_is_rust = self.source_is_rust;
        ctx.name_style_angr = self.name_style_angr;
        ctx.name_style_ghidra = self.name_style_ghidra;
        // (kuna) carry the duplicate-declaration collapse gate so `emit_local_var_decls`
        // (which reads the ArchContext `arch`) sees `option dedupvardecls`.
        ctx.dedup_var_decls = self.dedup_var_decls;
        // (kuna GH-558) carry the comparison-presentation gate so the
        // `compareform canonical|original` option reaches
        // `ActionPresentCompareForm` via `glb` (the ArchContext read site).
        ctx.present_lessequal = self.present_lessequal;
        // (kuna) carry the remaining stage-model rule gates so their `option
        // <name> on|off` reaches the consuming Rule/Action via `glb` (each rule
        // reads `data.get_arch().<flag>`; the rule is registered `enabled=false`
        // so the live flag drives both the DIV default and the toggle).
        ctx.fold_boolean_mask = self.fold_boolean_mask; // GH-1282 booleanmask
        ctx.simd_lane_fold = self.simd_lane_fold; // simdlane
        ctx.ret_split_global = self.ret_split_global; // retsplitglobal
        // (kuna) resolve the byte-shuffle user-op ids ONCE per program, so the
        // rule can name a CALLOTHER through the ArchSeam (the boundary
        // ArchContext carries no userop table).
        ctx.simd_shuffle_userops = crate::p3_dataflow::kuna_simdlane::SHUFFLE_USEROP_NAMES
            .iter()
            .filter_map(|nm| self.userops.get_op_by_name(nm).map(|u| u.get_index() as kuna_base::types::uint4))
            .collect();
        ctx.fold_flag_compare = self.fold_flag_compare; // GH-1276/8777 flagcompare
        ctx.add_carry_chain = self.add_carry_chain; // GH-8913 addcarrychain
        ctx.ov_less_simplify = self.ov_less_simplify; // GH-7190 ovlesssimplify
        ctx.recover_array_stride = self.recover_array_stride; // GH-8724 arraystride
        ctx.memset_recover = self.memset_recover; // GH-9230/1537 memsetrecover
        ctx.rodata_string = self.rodata_string; // (kuna) rodatastring
        ctx.ptrdepthcap = self.ptrdepthcap; // (kuna) ptrdepthcap
        ctx.model_stack_probe_loop = self.model_stack_probe_loop; // GH-8017 stackprobeloop
        ctx.recover_lowered_switch = self.recover_lowered_switch; // loweredswitch
        ctx.callsite_stack_args = self.callsite_stack_args; // callsitestackargs
        ctx.cookie_scramble = self.cookie_scramble; // cookiescramble
        ctx.callee_dead_arg = self.callee_dead_arg; // calleedeadarg
        ctx.callee_preserves = self.callee_preserves; // calleepreserves
        ctx.input_param_gap = self.input_param_gap; // inputparamgap
        ctx.vararg_stack_args = self.vararg_stack_args; // varargstackargs
        ctx.callee_arity = self.callee_arity; // calleearity
        ctx.callee_arity_fwd = self.callee_arity_fwd; // calleearityfwd
        ctx.callee_arity_live = self.callee_arity_live; // calleearitylive
        ctx.call_overlap = self.call_overlap; // calloverlap
        ctx.spill_arg_trial = self.spill_arg_trial; // spillargtrial
        ctx.load_guard_range = self.load_guard_range; // loadguardrange
        ctx.tied_store_keep = self.tied_store_keep; // tiedstorekeep
        ctx.region_structure = self.region_structure; // regionstructure
        ctx.guard_arm = self.guard_arm; // guardarm
        ctx.loop_cond_hoist = self.loop_cond_hoist; // loopcondhoist
        ctx.region_loop_refine = self.region_loop_refine; // regionlooprefine
        ctx.region_edge_order = self.region_edge_order; // regionedgeorder
        ctx.outline_spec = self.outline_spec.clone(); // outline
        ctx.remove_cleanup_code = self.remove_cleanup_code; // cleanupcode
        ctx.linux_syscall = self.linux_syscall; // linuxsyscall
        ctx.switch_selector_guard = self.switch_selector_guard; // switchselector
        ctx.cond_fold = self.cond_fold; // condfold
        ctx.reduce_return_gotos = self.reduce_return_gotos; // gotoreduce
        ctx.flatten_ifelse = self.flatten_ifelse; // ifelseflatten
        ctx.revert_cross_jumps = self.revert_cross_jumps; // crossjumprevert
        ctx.dup_return_call_tails = self.dup_return_call_tails; // taildup
        ctx.dedup_ite_tail = self.dedup_ite_tail; // dedupitetail
        ctx.iteregion = self.iteregion; // iteregion (diamond -> ?: ternary, runtime-choice)
        ctx.iteexpr = self.iteexpr; // iteexpr (computed-arm ?: extension, runtime-choice)
        ctx.iteboolean = self.iteboolean; // iteboolean (0/1 select -> boolean assignment)
        ctx.itecondlist = self.itecondlist; // itecondlist (condition-list tolerance for iteregion/iteboolean)
        ctx.param_copy_hoist = self.param_copy_hoist; // paramcopyhoist (parameter copy-shadow -> entry block)
        ctx.duplicate_shared_returns = self.duplicate_shared_returns; // returndup
        ctx.returndup_orchain = self.returndup_orchain; // orchain (short-circuit chain protection)
        ctx.early_return = self.early_return; // earlyreturn
        ctx.switch_return = self.switch_return; // switchreturn
        ctx.recover_loop_break = self.recover_loop_break; // loopbreak_recovery
        ctx.fold_call_returns = self.fold_call_returns; // foldcallret
        ctx.strip_stack_guard = self.strip_stack_guard; // stackguard
        ctx.strip_security_check = self.strip_security_check; // securitycheck
        ctx.branch_flip = self.branch_flip; // branchflip (negated-guard branch flipping)
        // (kuna) GH-9203 DIV-3: carry the loop-block COPY-placement gate so the
        // `condexeplace off` option reaches `ActionConditionalConst` via `glb`.
        ctx.condexe_block_placement = self.condexe_block_placement;
        // (kuna) carry the whiledo->for reroll gate (C++ `glb->analyze_for_loops`)
        // so `ActionStructureTransform` reaches it for
        // `Funcdata::finalize_forloop_transform`.
        // (kuna outlang) A language with no C-style `for` header must not have the
        // reroll run at all. `finalize_forloop_transform` physically MOVES the
        // initializer and increment ops into the header's slots and suppresses
        // them from the body; rendering that as a `while` would drop them, and
        // moving them back at print time would let a `continue` skip the
        // increment. With the gate off they stay where the CFG put them, which is
        // already the `while` shape. `caps.c_for` is true for C, so the C path is
        // unchanged.
        ctx.analyze_for_loops =
            self.analyze_for_loops && self.print.out_lang().profile().caps.c_for;
        // Carry the `nanignore all` flag (C++ `glb->nan_ignore_all`) so
        // `RuleIgnoreNan` reaches it via `glb`.
        ctx.nan_ignore_all = self.nan_ignore_all;
        // Share the populated data-type factory so `ActionInferTypes` (run via
        // `glb`) reaches the same interned core types this side cached.
        ctx.types = Some(self.types_rc());
        // Share the decoded-string manager (C++ `glb->stringManager`) so the
        // per-function `Funcdata::getInternalString` registers internal strings
        // into the very instance the printer reads back on this architecture.
        ctx.internal_strings = Some(Rc::clone(&self.string_manager));
        // Jump-table recovery constants (C++ `glb->max_jumptable_size` /
        // `funcptr_align`) and the load image (C++ `glb->loader`) so the
        // jump-table emulator reaches the read-only switch table.
        ctx.max_jumptable_size = self.max_jumptable_size;
        ctx.alias_block_level = self.alias_block_level;
        ctx.funcptr_align = self.funcptr_align;
        // (kuna) PE import-call binding: `query_function` carries a resolved
        // callee's no-return flag onto the proto it hands `ActionDeindirect` only
        // under this gate (the flow half of the binding).
        ctx.peimportcall = self.analysis_peimportcall;
        // (kuna GH-8471) Carry the Thumb-funcptr preservation gate so
        // `RulePtrsubUndo`'s thumb guard reads `glb->preserve_thumb_funcptr`.
        ctx.preserve_thumb_funcptr = self.preserve_thumb_funcptr;
        // (kuna) GH-9191: carry the modulo/and-mask jump-table index-bound gate
        // (`option switchmodbound`) so `JumpBasic::recoverModel` reaches it.
        ctx.switch_modulo_bound = self.switch_modulo_bound;
        // (kuna, angr) carry the CBRANCH-guard jump-table index-bound gate
        // (`option switchguardbound`) so `JumpBasic::recoverModel` reaches it.
        ctx.switch_guard_bound = self.switch_guard_bound;
        // (kuna, angr) carry the loop-carried-base relative-offset jump-table gate
        // (`option switchsharedcase`) so `JumpBasic::recoverModel` reaches it.
        ctx.switch_shared_case = self.switch_shared_case;
        // (kuna, angr) carry the multi-predecessor unrolled-guard jump-table gate
        // (`option switchmultipred`) so `JumpBasic::checkUnrolledGuard` reaches it.
        ctx.switch_multi_pred = self.switch_multi_pred;
        // (kuna, angr) carry the interleaved unrolled-guard partial-flow gate
        // (`option unrolledguard`) so `FlowInfo::collectEdges` reaches it.
        ctx.unrolled_guard = self.unrolled_guard;
        // (kuna) carry the shared-partial gate (`option jtsharepartial`) so
        // `Funcdata::stage_jump_table` reaches it.
        ctx.jumptable_share_partial = self.jumptable_share_partial;
        ctx.loader = Some(self.translate.loader_rc());
        // Carry the read-only-propagation switch (C++ `glb->readonlypropagate`,
        // flipped by `option readonly`) so `ActionVarnodeProps` reaches it to gate
        // `Funcdata::fillinReadOnly` (the readonly-RAM-global constant fold).
        ctx.readonlypropagate = self.readonlypropagate;
        // (kuna `dynrelocs`) Carry the PT_GNU_RELRO-frozen dynamic-relocation
        // slots so `ActionVarnodeProps` folds those loads with global read-only
        // propagation still off. `Rc` clone: the list is built once at load.
        ctx.dynreloc_const = Rc::clone(&self.dynreloc_const);
        // Carry the data-type-splitting toggle bits (C++ `glb->split_datatype_config`)
        // so `SplitDatatype` / `RuleSplit{Copy,Load,Store}` reach them per function.
        ctx.split_datatype_config = self.split_datatype_config;
        // Snapshot the global symbol table onto `glb` so the per-function
        // `setVarnodeProperties` can run `localmap->queryProperties`'s walk into
        // the global scope (C++ `glb` reaches the live `symboltab`; the merged
        // kuna `glb` is a skeleton, so the global scope is wired here, after every
        // `map addr`).  Global-mapped varnodes then pick up `persist`/`addrtied`
        // and their stores survive `ActionDeadCode`.
        ctx.global_query = Some(Rc::new(self.symboltab.build_global_query()));
        // (kuna, Phase 3) the ghidra-mode lazy provider rides the handle so the
        // global reads above query through it; None on the standalone path.
        ctx.remote_scope = self.remote_scope.clone();
        // Snapshot every source-declared callee prototype (parked on the global
        // FunctionSymbols by `set_function_prototype_pieces`) so the per-function
        // `ActionDefaultParams` copies a known callee's locked `FuncProto` into the
        // call site (C++ `coreaction.cc:2385` `fc->copy(otherfunc->getFuncProto())`).
        ctx.callee_protos = self.symboltab.build_callee_proto_pieces();
        // Carry the constant-pointer-inference config (C++ `glb->infer_pointers` /
        // `infer_funcentry`) and the ordered inferable-pointer spaces (C++
        // `glb->inferPtrSpaces`, built by cacheAddrSpaceProperties) so
        // `ActionConstantPtr` (run via `glb`) can rewrite a mapped global-constant
        // address into a typed `PTRSUB(spacebase,off)`.
        ctx.infer_pointers = self.infer_pointers;
        ctx.infer_funcentry = self.infer_funcentry;
        ctx.infer_ptr_spaces = self.infer_ptr_spaces.clone();
        // Snapshot the tracked-register database (C++ `glb->context`'s track base,
        // populated by `set track`) so `ActionConstbase` can query it for the
        // function entry address through the detached per-function skeleton.
        ctx.tracked_sets = self.with_context_db_mut(|db| db.clone_trackbase());
        Rc::new(ctx)
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
        let manager = self.translate.manager_mut();
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
        let manager = self.translate.manager_mut();
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
    /// [`resolve_constant`](crate::context::ArchContext::resolve_constant), this is
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
    /// This is THE boundary the revisit / global-persist path depends on: with the
    /// global scope owning the `ram` range, `Scope::queryProperties`'s `inScope`
    /// discovery branch (database.cc:1276-1281) returns
    /// `mapped | addrtied | persist` for any RAM Varnode with no covering Symbol,
    /// so global RAM stores survive `ActionDeadCode` and hold their call
    /// `INDIRECT`s.
    ///
    /// LOSS: the C++ overlay-space duplication (`addToGlobalScope`,
    /// architecture.cc:838-846) and the `inferPtrSpaces` push (architecture.cc:836,
    /// a pointer-inference stub) are not transcribed — no datatest exercises an
    /// overlay base space here, and `inferPtrSpaces` feeds only `TypeFactory`
    /// pointer inference (a separate stub).  General over any processor's cspec:
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
        let manager = self.translate.manager_rc();
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
                // C++ `Range::Range` register branch (address.cc:239-245).
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
        let manager = self.translate.manager_rc();
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

    /// (kuna `msvcftol`) Register the synthesized MSVC `__ftol`-family call-fixup
    /// (`p2_lift::kuna_msvcftol`) alongside the cspec's own `<callfixup>`
    /// elements, so `parse_inject_all` compiles it with the rest.
    ///
    /// Registration is unconditional on x86-32 rather than gated on the option:
    /// the architecture is bootstrapped at `load file`, which the console script
    /// runs *before* its `option` lines, so the flag is not yet readable here. A
    /// registered payload is inert until something installs it, and the install
    /// (the analysis-tier `callfixup` pass, which runs at `read symbols`, after
    /// the options) is where `option msvcftol off` takes effect.
    ///
    /// Guarded to x86-32 because the body names `ST0..ST7`/`EAX`/`EDX`/`ESP`: on
    /// a language without them the SLEIGH snippet compile would fail at
    /// bootstrap, and `_ftol` exists on no other target.
    ///
    /// Skipped entirely in ghidra mode, exactly as `register_cortexmpriv_fixup`
    /// is: with no local `.sla`, step 3 never compiles the payload, so the
    /// registration could not pay off there anyway.
    fn decode_kuna_call_fixups(&mut self) -> KunaResult<()> {
        use kuna_base::marshal::{IdRegistry, XmlDecode};

        if self.translate.as_sleigh().is_none() {
            return Ok(());
        }
        let code_addr_size = self
            .manage()
            .get_default_code_space()
            .map(|s| s.get_addr_size() as int4)
            .unwrap_or(0);
        let resolve = |nm: &[u8]| self.translate.probe_register_varnode(nm).is_some();
        if !crate::kuna_msvcftol::language_is_x86_32(resolve, code_addr_size) {
            return Ok(());
        }
        let manager = self.translate.manager_rc();
        let mut registry = IdRegistry::with_base_ids();
        crate::pcodeinject::register_ids(&mut registry);
        crate::kuna_msvcftol::decode_payload(|root| {
            let mut decoder = XmlDecode::new_with_root(&manager, &registry, root, 0);
            self.pcodeinjectlib.decode_inject(
                b" : kuna compiler helpers",
                b"",
                crate::pcodeinject::CALLFIXUP_TYPE,
                &mut decoder,
            )?;
            Ok(())
        })
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
            let manager = self.translate.manager_rc();
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

        // 2b. (kuna `cortexmpriv`) Register the synthesized
        //     `isCurrentModePrivileged` callother-fixup alongside the cspec's own,
        //     so step 3 compiles it with the rest. Unconditional on any language
        //     that declares the user-op (the option is not readable at bootstrap);
        //     `decompile_drive::is_injected_userop` is the gate. See
        //     `p2_lift::kuna_cortexmpriv`.
        self.register_cortexmpriv_fixup()?;

        // 3. parseInject: compile every registered injection body (callfixup +
        //    callotherfixup) into a ConstructTpl against the loaded language.
        //    Move the inject library out so the &Sleigh (SnippetLanguageProvider)
        //    borrow of self.translate does not alias the &mut library.
        let mut lib = std::mem::take(&mut self.pcodeinjectlib);
        // The SnippetLanguage is the loaded `SleighBase`; drive parse_inject over
        // it (the &SleighBase read does not alias the &mut library).  Injection
        // compilation is a Sleigh-engine concern, so reach the concrete engine's
        // `SleighBase` through the downcast (only `Sleigh` implements the boundary).
        //
        // In ghidra mode there is no local `.sla` to compile snippets against —
        // the host supplies inject p-code on demand via a `getPcodeInject` query
        // (C++ `PcodeInjectLibraryGhidra`), reached through
        // `EngineTranslate::fetch_inject_pcode` at the two `get_tpl` consumers in
        // `infra::decompile_drive`.  Registration must still happen here: it is
        // what populates the name->id maps and the incidentalcopy/paramshift
        // metadata both paths read.  The guard is a no-op on the standalone path,
        // where `as_sleigh()` is always `Some`.
        let parse_res = match self.translate.as_sleigh() {
            Some(sleigh) => lib.parse_inject_all(sleigh.base()),
            None => Ok(()),
        };
        self.pcodeinjectlib = lib;
        parse_res
    }

    /// (kuna `cortexmpriv`) Install the `isCurrentModePrivileged` callother-fixup
    /// that reports privileged, parking its inject id on
    /// [`cortexmpriv_inject`](Self::cortexmpriv_inject).
    ///
    /// A no-op in ghidra mode (no local `.sla` to compile the body against), on
    /// any language whose translator does not present the user-op (everything but
    /// ARM), and on one where a compiler spec already specialized it — the cspec's
    /// own `<callotherfixup>` wins, exactly as it does for the vendored
    /// `setISAMode` fixups.
    fn register_cortexmpriv_fixup(&mut self) -> KunaResult<()> {
        use crate::kuna_cortexmpriv as fixup;
        // No local `.sla` (ghidra mode) means step 3 cannot compile the body, and
        // an injected user-op with a null template is a hard decompile error --
        // so leave the op unspecialized there, exactly as it is today.
        if self.translate.as_sleigh().is_none() {
            return Ok(());
        }
        match self.userops.get_op_by_name(fixup::USEROP_NAME) {
            Some(op) if op.is_unspecialized() => {}
            _ => return Ok(()),
        }
        let injectid = self.pcodeinjectlib.manual_call_other_fixup(
            fixup::USEROP_NAME,
            fixup::OUTPUT_NAME,
            &[],
            fixup::SNIPPET,
        )?;
        let mut userops = std::mem::take(&mut self.userops);
        let res = userops.manual_call_other_fixup(fixup::USEROP_NAME, injectid);
        self.userops = userops;
        res?;
        self.cortexmpriv_inject = Some(injectid);
        Ok(())
    }

    /// (kuna) Register the fixed set of string-copy builtin user-ops into
    /// `userops` so the printer's `opCallother` path resolves their name,
    /// display, and typed parameters.  Mirrors the lazy
    /// `userops.registerBuiltin(BUILTIN_*)` calls in `ArraySequence::buildStringCopy`
    /// and `Funcdata::getInternalString` (C++ does these on demand during the
    /// transform; the kuna ArchContext can't reach the real `userops`, so they are
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

    /// Share the data-type factory `Rc` so the analysis-side ArchContext (`glb`) reaches
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

    /// (kuna outlang) Select the output language by name, rejecting one no
    /// back-end claims.
    ///
    /// The `ArchOptionContext::set_print_language` sibling is infallible because
    /// it mirrors the upstream setter; this is the surface a front-end should
    /// use, so a typo is an error rather than a silent fall back to C.
    pub fn set_print_language_checked(&mut self, name: &str) -> KunaResult<()> {
        let lang = crate::kuna_lang::OutLang::from_print_name(name).ok_or_else(|| {
            KunaError::parse(format!(
                "unknown output language {name:?} (expected one of: {})",
                crate::kuna_lang::OutLang::names().join(", ")
            ))
        })?;
        self.print.set_name(lang.print_name());
        Ok(())
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
        // The engine boundary exposes the object-safe `with_context_db_dyn` (a
        // `&mut dyn FnMut`, so it survives the `Box<dyn EngineTranslate>` trait
        // object); adapt the generic, value-returning closure over it.  The
        // protocol is synchronous — the closure runs exactly once — so `f` and
        // the result move through `Option` slots cleanly.
        let mut f = Some(f);
        let mut result: Option<R> = None;
        self.translate.with_context_db_dyn(&mut |db| {
            result = Some((f.take().expect("with_context_db_mut: closure runs once"))(db));
        });
        result.expect("with_context_db_mut: closure ran")
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

    /// Number of registered prototype models (C++ `protoModels.size()`).
    pub fn num_proto_models(&self) -> usize {
        self.proto_models.len()
    }

    /// Names of the registered prototype models, in registry (sorted) order.
    pub fn proto_model_names(&self) -> impl Iterator<Item = &str> {
        self.proto_models.keys().map(|s| s.as_str())
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

    /// Seed the core data-types (C++ `ArchitectureGhidra::buildCoreTypes` /
    /// `SleighArchitecture::buildCoreTypes`, ghidra_arch.cc:316-349): when a
    /// wire `<coretypes>` document was installed ([`Self::set_coretypes_xml`] —
    /// the ghidra-mode registerProgram spec, the C++ `store.getTag("coretypes")`
    /// branch) decode it, so the core-type IDS match the host's and every later
    /// `<typeref>`/getDataType exchange resolves; else the verbatim default
    /// `setCoreType` sequence + `cacheCoreTypes`.
    pub fn build_core_types(&mut self) -> KunaResult<()> {
        use type_metatype::*;
        if let Some(xml) = self.coretypes_xml.clone() {
            use kuna_base::marshal::{IdRegistry, XmlDecode};
            let manager = self.translate.manager_rc();
            let mut registry = IdRegistry::with_base_ids();
            crate::dtype::register_type_wire_ids(&mut registry);
            let store = kuna_base::xml::xml_tree(&xml)?;
            let root = store.get_root().clone();
            let mut decoder = XmlDecode::new_with_root(&manager, &registry, &root, 0);
            return self.types.decode_core_types(&mut decoder);
        }
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
                // (kuna) The remaining scalar widths.  Upstream's
                // `decodeDataOrganization` reads these too; kuna had only ever
                // needed int/long/pointer/char/wchar, so the rest fell through the
                // `_ => {}` arm and no consumer could ask for them.  `<float_size>`
                // (60 cspecs), `<double_size>` (61), `<long_double_size>` (56),
                // `<short_size>` (54) and `<long_long_size>` (51) are the five that
                // a per-architecture C spelling needs.
                "short_size" => {
                    if let Some(v) = read(child) {
                        self.types.set_size_of_short(v);
                    }
                }
                "long_long_size" => {
                    if let Some(v) = read(child) {
                        self.types.set_size_of_long_long(v);
                    }
                }
                "float_size" => {
                    if let Some(v) = read(child) {
                        self.types.set_size_of_float(v);
                    }
                }
                "double_size" => {
                    if let Some(v) = read(child) {
                        self.types.set_size_of_double(v);
                    }
                }
                "long_double_size" => {
                    if let Some(v) = read(child) {
                        self.types.set_size_of_long_double(v);
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
    /// STUB(W6 cspec): the *real* default proto model comes from the cspec
    /// `<default_proto><prototype …>` decode (`ProtoModel::decode` building the
    /// param lists from `<input>`/`<output>` `<pentry>` records).  When the
    /// frontend supplied the cspec XML (via [`set_cspec_xml`](Architecture::set_cspec_xml))
    /// the `<default_proto>` input/output parameter lists are decoded here (the
    /// general, spec-driven path — see [`decode_default_proto`](Architecture::decode_default_proto)),
    /// so the recovered model carries real return/parameter storage and the
    /// proto-recovery actions can fire, and the spec's named models
    /// ([`decode_named_protos`](Architecture::decode_named_protos)) are registered
    /// alongside it.  Otherwise a name-only default model is registered so the
    /// engine still has a non-null `defaultfp`.
    pub fn build_default_proto(&mut self) {
        // One parse of the cspec document feeds all three readers below: the
        // top-level <returnaddress> (C++ `Architecture::decodeReturnAddress`,
        // architecture.cc:902), the <default_proto> model, and the named models.
        if let Some(xml) = self.cspec_xml.take() {
            let root = {
                let mut store = kuna_base::xml::DocumentStorage::new();
                store.parse_document(&xml).map(|d| d.get_root().clone())
            };
            if let Ok(root) = root {
                // A missing/empty <returnaddress> leaves `default_return_addr` as
                // `None` (== the C++ `defaultReturnAddr.space == 0`).
                self.default_return_addr = self.decode_default_return_addr(&root);
                if let Ok(model) = self.decode_default_proto(&root) {
                    let rc = Rc::new(model);
                    self.register_model(Rc::clone(&rc));
                    self.defaultfp = Some(Rc::clone(&rc));
                    // The spec's NAMED models (`<prototype>`/`<resolveprototype>`/
                    // `<modelalias>`) join the registry too, so `getModel` resolves
                    // more than the default one.  Registration only: nothing here
                    // selects a model for any function.
                    let named = self.decode_named_protos(&root, &rc);
                    for m in named {
                        self.register_model(m);
                    }
                    // (kuna `evalcurrentproto`) The spec's `<eval_current_prototype>`
                    // nomination (C++ `parseCompilerConfig`'s
                    // `ELEM_EVAL_CURRENT_PROTOTYPE` arm, architecture.cc:1321):
                    // which registered model a function's OWN unlocked prototype is
                    // evaluated with. Recorded here, applied per function in
                    // `build_arch_handle` under the option; a name the registry does
                    // not carry is ignored (the model failed to decode -- the
                    // registration pass skips rather than throws), leaving the
                    // `<default_proto>` evaluation.
                    self.evalfp_current_spec =
                        crate::kuna_evalcurrentproto::eval_current_model_name(&root)
                            .and_then(|name| self.proto_models.get(&name).cloned());
                    return;
                }
                // Fall through to the name-only default on any decode failure
                // (faithful degradation; the recovery simply won't fire).
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

    /// (kuna, Phase 3) Record the wire `<coretypes>` XML for the ghidra-mode
    /// [`build_core_types`](Architecture::build_core_types) decode.  Must be
    /// set before [`init_post_engine`](Architecture::init_post_engine).
    pub fn set_coretypes_xml(&mut self, xml: Vec<u8>) {
        self.coretypes_xml = Some(xml);
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
    /// STUB(W6 pspec): the remaining `<processor_spec>` children (volatile,
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
        // (build_arch_handle) so the painted flagbase reaches the per-function ArchContext.
        if let Some(volatile_el) = find_child(&pspec, "volatile") {
            self.decode_volatile(&volatile_el)?;
        }

        // C++ parseProcessorConfig ELEM_CONTEXT_DATA branch.  A pspec with no
        // <context_data> (e.g. a 32-bit-default processor) leaves the zero
        // context, which is correct for it.
        let Some(context_data) = find_child(&pspec, "context_data") else {
            return Ok(());
        };

        // Ghidra-mode: skip the <context_set> paints (C++
        // `ContextGhidra::decode`/`decodeFromSpec`, ghidra_context.cc, are both a
        // bare `decoder.skipElement()` — "Ignore details handled by ghidra").  In
        // ghidra mode the Java host owns disassembly context and returns
        // already-context-resolved p-code via `getPcode`, so the query-backed
        // engine's own context database is never consulted for disassembly — and
        // it has no variables registered (there is no `.sla` parse to
        // `registerContext` them), so applying `<set name="addrsize" .../>` would
        // raise "Non-existent context variable: addrsize".  `as_sleigh()` is
        // `None` iff this is the query-backed `GhidraTranslate`; the standalone
        // `Sleigh` path returns `Some` and still applies the paints, exactly as
        // the 675 x86 datatests (which need `addrsize`/`opsize` for 64-bit
        // disassembly) require.
        //
        // (kuna, Phase 3) The `<tracked_set>` children are DIFFERENT: they name
        // whole registers with pinned values (x86-64: `DF = 0`), feed only the
        // engine-side `trackbase` (no context variables involved), and are what
        // `ActionConstbase` reads to plant `DF = COPY 0` — without which every
        // string op renders the `(uint8)DF * -2 + 1` direction garbage.  Upstream
        // ghidra-mode recovers the same facts via the getTrackedRegisters query
        // (`ContextGhidra::getTrackedSet`); kuna decodes them straight from the
        // wire pspec it was already handed, resolving register names through the
        // query-backed translator (a getRegister query, legal during
        // registerProgram).  A register the host cannot resolve is skipped, never
        // fatal — upstream ghidra-mode never decodes this block at all.
        if self.translate.as_sleigh().is_none() {
            self.decode_ghidra_tracked_sets(&context_data)?;
            return Ok(());
        }

        // Decode <context_data> against the engine's single address-space
        // manager (so `space="ram"` resolves to the real ram space).  The Rc
        // keeps the manager alive for the decoder while the context database
        // (a sibling RefCell on the engine) is borrowed mutably — no aliasing.
        let manager = self.translate.manager_rc();
        let mut registry = IdRegistry::with_base_ids();
        register_globalcontext_ids(&mut registry);
        let mut decoder = XmlDecode::new_with_root(&manager, &registry, &context_data, 0);
        self.with_context_db_mut(|db| db.decode_from_spec(&mut decoder))?;
        Ok(())
    }

    /// (kuna, Phase 3) Decode ONLY the `<tracked_set>` children of the pspec
    /// `<context_data>` into the engine context database's trackbase — the
    /// ghidra-mode arm of [`parse_processor_config`](Self::parse_processor_config).
    ///
    /// Register names (`<set name="DF" val="0"/>`) resolve through the
    /// query-backed translator's `get_register_varnode` (a getRegister query on
    /// the host); a name the host cannot resolve skips that `<set>` (upstream
    /// ghidra-mode skips the whole block, so nothing here may be fatal).
    fn decode_ghidra_tracked_sets(
        &mut self,
        context_data: &Rc<kuna_base::xml::Element>,
    ) -> KunaResult<()> {
        use kuna_base::address::Range;
        use kuna_base::marshal::{Decoder, IdRegistry, XmlDecode, ATTRIB_NAME, ATTRIB_VAL};
        use kuna_sleigh::globalcontext::{register_globalcontext_ids, TrackedContext, TrackedSet};
        let manager = self.translate.manager_rc();
        let mut registry = IdRegistry::with_base_ids();
        register_globalcontext_ids(&mut registry);
        for child in context_data.get_children() {
            if child.get_name() != "tracked_set" {
                continue;
            }
            let mut decoder = XmlDecode::new_with_root(&manager, &registry, child, 0);
            let sub_id = decoder.open_element()?;
            let range = Range::decode_from_attributes(&mut decoder)?;
            let addr1 = range.get_first_addr();
            let addr2 = range.get_last_addr_open(decoder.get_addr_space_manager());
            let mut set: TrackedSet = Vec::new();
            while decoder.peek_element()? != 0 {
                let set_id = decoder.open_element()?;
                // <set name=… val=…> — resolve the register via the translator
                // (VarnodeData::decode_from_attributes' name path needs a
                // manager-installed RegisterLookup, which ghidra mode has none of).
                let mut loc = None;
                loop {
                    let aid = decoder.get_next_attribute_id()?;
                    if aid == 0 {
                        break;
                    }
                    if aid == ATTRIB_NAME.get_id() {
                        let nm = decoder.read_string()?;
                        loc = self.translate.get_register_varnode(&nm).ok();
                        break;
                    }
                }
                if let Some(loc) = loc {
                    let val = decoder.read_unsigned_integer_id(&ATTRIB_VAL)?;
                    set.push(TrackedContext { loc, val });
                }
                decoder.close_element(set_id)?;
            }
            decoder.close_element(sub_id)?;
            if !set.is_empty() {
                self.with_context_db_mut(|db| *db.create_set(&addr1, &addr2) = set);
            }
        }
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

        let manager = self.translate.manager_rc();
        let registry = IdRegistry::with_base_ids();
        // C++ `decodeVolatile`: each child is a `<range>`; resolve it through
        // `Range::from_properties` exactly as `decode_global` does, then paint
        // [first, lastOpen) with `volatil`.
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
    /// property range); that property subsystem is a separate stub and is not
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
                continue; // no lane sizes (and volatile is a separate stub)
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
            let mut laned_register = LanedRegister::new();
            laned_register.parse_sizes(storage_size, &lane_sizes)?;
            let size_index = laned_register.get_whole_size();
            while (mask_list.len() as int4) <= size_index {
                mask_list.push(0);
            }
            mask_list[size_index as usize] |= laned_register.get_size_bit_mask();
        }
        self.lanerecords.clear();
        for (i, &mask) in mask_list.iter().enumerate() {
            if mask == 0 {
                continue;
            }
            self.lanerecords.push(LanedRegister::with_mask(i as int4, mask));
        }
        Ok(())
    }

    /// Decode the cspec's top-level `<returnaddress>` storage element into the
    /// `defaultReturnAddr` VarnodeData (C++ `Architecture::decodeReturnAddress`,
    /// architecture.cc:902 -> `VarnodeData::decode`).  The element wraps a single
    /// `<register>`/`<varnode>`/`<addr>` storage child; resolve it through the
    /// engine `Translate` exactly as the effect-block decode does.  Returns `None`
    /// when there is no `<returnaddress>` or it is empty (C++ leaves
    /// `defaultReturnAddr.space == 0`).
    fn decode_default_return_addr(
        &self,
        root: &Rc<kuna_base::xml::Element>,
    ) -> Option<kuna_num::pcoderaw::VarnodeData> {
        let ra = find_child(root, "returnaddress")?;
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

    /// Locate the cspec's `<default_proto><prototype>` and decode it (C++
    /// `Architecture::decodeDefaultProto`, architecture.cc:793).
    fn decode_default_proto(&self, root: &Rc<kuna_base::xml::Element>) -> KunaResult<ProtoModel> {
        // Find <default_proto><prototype>.
        let dp = find_child(root, "default_proto")
            .ok_or_else(|| KunaError::lowlevel("cspec has no <default_proto>"))?;
        let proto = find_child(&dp, "prototype")
            .ok_or_else(|| KunaError::lowlevel("<default_proto> has no <prototype>"))?;
        self.decode_proto_model(&proto, root)
    }

    /// Decode one `<prototype>` element into a [`ProtoModel`] (C++
    /// `ProtoModel::decode`, fspec.cc:2563): the
    /// `name`/`extrapop`/`strategy`/`hasthis`/`constructor` attributes, the
    /// `<input>`/`<output>` `<pentry>` lists and the
    /// `<unaffected>`/`<killedbycall>`/`<returnaddress>`/`<internal_storage>`
    /// blocks.  `root` is the enclosing `<compiler_spec>`, consulted only for the
    /// top-level `<returnaddress>` fallback.
    ///
    /// General over any processor's cspec and over any `<prototype>` position:
    /// the one inside `<default_proto>` and every top-level named model
    /// ([`decode_named_protos`](Architecture::decode_named_protos)) go through
    /// this same body, so a named model carries the identical storage/effect
    /// fidelity as the default one.
    fn decode_proto_model(
        &self,
        proto: &Rc<kuna_base::xml::Element>,
        root: &Rc<kuna_base::xml::Element>,
    ) -> KunaResult<ProtoModel> {
        let mut model = ProtoModel::new(self.manage());
        let name = attr_str(proto, "name").unwrap_or_else(|| "__stdcall".to_string());
        // extrapop="unknown" -> EXTRAPOP_UNKNOWN; numeric otherwise.
        if let Some(ep) = attr_str(proto, "extrapop") {
            if ep == "unknown" {
                model.set_extra_pop(crate::fspec::EXTRAPOP_UNKNOWN);
            } else if let Ok(v) = ep.parse::<int4>() {
                model.set_extra_pop(v);
            }
        }
        // `hasthis`/`constructor` (ATTRIB_HASTHIS/ATTRIB_CONSTRUCTOR) mark a model
        // for non-static class methods / constructors.  `set_name` runs LAST so the
        // C++ `if (name == "__thiscall") hasThis = true` override (fspec.cc:2595)
        // wins over an explicit `hasthis="false"`, exactly as upstream orders it.
        if let Some(v) = attr_str(proto, "hasthis") {
            model.set_has_this(decode_bool_attr(&v));
        }
        if let Some(v) = attr_str(proto, "constructor") {
            model.set_constructor(decode_bool_attr(&v));
        }
        model.set_name(&name);
        let strategy = attr_str(proto, "strategy").unwrap_or_default();
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
        // `glb->defaultReturnAddr` is decoded from the cspec's top-level
        // <returnaddress> (C++ Architecture::parseExtraRules / decode); parse that
        // root element directly here so the per-call retaddr store is modeled even
        // when the <prototype> omits its own <returnaddress> (the x86-64-gcc case).
        if !saw_retaddr {
            if let Some(ra_block) = find_child(root, "returnaddress") {
                self.decode_effect_block(
                    &ra_block,
                    &mut model,
                    crate::fspec::effect_type::RETURN_ADDRESS,
                )?;
            }
        }
        // (kuna, ida) State the x86 direction-flag guarantee the compiler spec
        // leaves implicit. See `kuna_dfunaffected`: without it every call plants
        // `DF = INDIRECT(DF, <call>)`, the entry-block `DF = 0` never reaches the
        // string-op stride, and `1 - 2*DF` survives into the output as
        // `(uint8)v18 * -2 + 1`. Applied only where the spec is silent, and a
        // structural no-op on any language with no `DF` register.
        crate::kuna_dfunaffected::assert_direction_flag_unaffected(&mut model, |nm| {
            self.translate.probe_register_varnode(nm)
        });
        Ok(model)
    }

    /// Decode every *named* prototype model the compiler spec declares, in
    /// document order (C++ `Architecture::parseCompilerConfig`'s
    /// `ELEM_PROTOTYPE`/`ELEM_RESOLVEPROTOTYPE` -> `decodeProto`,
    /// architecture.cc:1254/1280, plus the `ELEM_MODELALIAS` arm at
    /// architecture.cc:1310).  `defaultfp` is the already-decoded
    /// `<default_proto>` model, which the later elements may reference by name.
    /// The C++ post-parse invariant "we must have a `__thiscall` calling
    /// convention" (architecture.cc:1342) is honored at the tail.
    ///
    /// Returns the models to register, in registration order.  Unlike the C++,
    /// which throws on any malformed element, a model that fails to decode (an
    /// unknown strategy, a `<pentry>` naming a register this language does not
    /// have, a `<resolveprototype>` whose constituents are not standard lists)
    /// is skipped: the cspec corpus spans every vendored processor, and one
    /// undecodable named model must not cost the architecture its default one.
    ///
    /// `<eval_current_prototype>`/`<eval_called_prototype>` are deliberately NOT
    /// applied here — they change which model every function is evaluated with,
    /// which is a behavior change this registration pass does not make.
    fn decode_named_protos(
        &self,
        root: &Rc<kuna_base::xml::Element>,
        defaultfp: &Rc<ProtoModel>,
    ) -> Vec<Rc<ProtoModel>> {
        let mut byname: std::collections::BTreeMap<String, Rc<ProtoModel>> =
            std::collections::BTreeMap::new();
        byname.insert(defaultfp.get_name().to_string(), Rc::clone(defaultfp));
        let mut out: Vec<Rc<ProtoModel>> = Vec::new();

        for child in root.get_children().iter() {
            let decoded = match child.get_name() {
                "prototype" => self.decode_proto_model(child, root).ok(),
                "resolveprototype" => self.decode_resolve_proto(child, &byname).ok(),
                "modelalias" => match (attr_str(child, "name"), attr_str(child, "parent")) {
                    (Some(nm), Some(parent)) => {
                        byname.get(&parent).and_then(|p| create_model_alias(&nm, p).ok())
                    }
                    _ => None,
                },
                _ => continue,
            };
            let Some(model) = decoded else { continue };
            // C++ throws "Duplicate ProtoModel name"; keep the first, which is the
            // default model when a spec re-declares it.
            if byname.contains_key(model.get_name()) {
                continue;
            }
            let rc = Rc::new(model);
            byname.insert(rc.get_name().to_string(), Rc::clone(&rc));
            out.push(rc);
        }
        // C++ `parseCompilerConfig` tail, architecture.cc:1342 — "We must have a
        // __thiscall calling convention": when the spec declares none, clone it
        // off the default so `getModel("__thiscall")` resolves on every language.
        if !byname.contains_key("__thiscall") {
            if let Ok(m) = create_model_alias("__thiscall", defaultfp) {
                out.push(Rc::new(m));
            }
        }
        out
    }

    /// Decode one `<resolveprototype>` element into a merged model (C++
    /// `ProtoModelMerged::decode`, fspec.cc:2904): each `<model name=".."/>`
    /// child names an already-registered constituent that is folded in, then the
    /// merged input list is finalized.
    fn decode_resolve_proto(
        &self,
        el: &Rc<kuna_base::xml::Element>,
        byname: &std::collections::BTreeMap<String, Rc<ProtoModel>>,
    ) -> KunaResult<ProtoModel> {
        let name = attr_str(el, "name")
            .ok_or_else(|| KunaError::lowlevel("<resolveprototype> has no name"))?;
        let mut model = ProtoModel::new_merged(self.manage());
        model.set_name(&name);
        let mut count = 0;
        for child in el.get_children().iter() {
            if child.get_name() != "model" {
                continue;
            }
            let sub = attr_str(child, "name")
                .ok_or_else(|| KunaError::lowlevel("<model> has no name"))?;
            let constituent = byname
                .get(&sub)
                .ok_or_else(|| KunaError::lowlevel(format!("Missing prototype model: {sub}")))?;
            model.merged_push(Rc::clone(constituent))?;
            count += 1;
        }
        if count == 0 {
            return Err(KunaError::lowlevel("<resolveprototype> has no <model>"));
        }
        model.merged_finalize();
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
        // C++ tail: resourceStart / calcDelay / populateResolver.
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
        // at rest; that broke the `phase status`/`pipeline list (current)`
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
            Rc::new(OwnedFloatFormats::from_translate(self.translate.as_ref()));
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
    pub fn resolve_typeop(&self, opc: kuna_num::opcodes::OpCode) -> crate::context::TypeOp {
        match self.inst.get(opc as usize).and_then(|o| o.as_ref()) {
            Some(info) => info.to_type_op(),
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
        // (kuna) The kuna-owned compiler-helper fixups, registered right after the
        // cspec's own so `init_userops_and_fixups`'s `parse_inject_all` compiles
        // them together.  Keeps the vendored spec tree byte-identical to upstream.
        self.decode_kuna_call_fixups()?;
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
        // through the per-function ArchContext (`glb`), which carries no mutable
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
// (options.cc) reaches; the `// STUB(...)` markers in the options.rs trait doc
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
        // STUB(W4 symboltab + W6 fspec): no function is resolvable here yet.
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
        // C++ `OptionInline::apply`: query the global function, then set its proto inline flag.
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
    fn print_lang_known(&self) -> bool {
        crate::kuna_lang::OutLang::from_print_name(self.print.get_name()).is_some()
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
        // STUB(W5): `ActionDatabase::toggleAction` (action.cc:1036) is not yet
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
        self.translate.get_user_op_names(&mut res);
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
    fn from_translate(translate: &dyn EngineTranslate) -> Self {
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

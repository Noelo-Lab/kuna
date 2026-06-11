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
use kuna_base::error::KunaResult;
use kuna_base::space::AddrSpaceManager;
use kuna_base::types::{int4, uint4};

use kuna_sleigh::sleigh::Sleigh;
use kuna_sleigh::translate::{Translate, UniqueLayout};

use crate::action::ActionDatabase;
use crate::database::Database;
use crate::flow::flow_flags;
use crate::funcdata::Funcdata;
use crate::inject_sleigh::PcodeInjectLibrarySleigh;
use crate::options::{split_datatype, OptionDatabase};
use crate::seams::{ArchHandle, Architecture as ArchSeam};
use crate::userop::UserOpManage;

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
    /// (kuna GH-8500) Hold a store-through-a-stack-pointer-alias across the
    /// deadcode race (C++ `stack_alias_deadstore`).
    pub stack_alias_deadstore: bool,
    /// (kuna GH-8724) Re-express a strided-induction offset as counter*stride
    /// (C++ `recover_array_stride`).
    pub recover_array_stride: bool,
    /// (kuna) Reconstruct a compiler-lowered comparison cascade into a switch
    /// (C++ `recover_lowered_switch`).
    pub recover_lowered_switch: bool,
    /// (kuna) Strip the glibc -fstack-protector canary epilogue
    /// (C++ `strip_stack_guard`).
    pub strip_stack_guard: bool,
    /// (kuna) Use angr-style default naming (vN/aN/dat_/sub_/label_ + comments)
    /// (C++ `name_style_angr`).
    pub name_style_angr: bool,
    /// (kuna GH-558) Restore canonicalized comparisons to LESSEQUAL form for
    /// presentation (C++ `present_lessequal`).
    pub present_lessequal: bool,
    /// (kuna GH-8471) Keep mode-bit-encoded (Thumb) function pointers symbolic
    /// (C++ `preserve_thumb_funcptr`).
    pub preserve_thumb_funcptr: bool,

    // --- Owned subsystems (architecture.hh:211-233) -----------------------
    /// Memory map of global variables and functions (C++ `symboltab`).
    pub symboltab: Database,
    /// Options that can be configured (C++ `options`).
    pub options: OptionDatabase,
    /// Actions that can be applied in this architecture (C++ `allacts`).
    pub allacts: ActionDatabase,
    /// Specifically registered user-defined p-code ops (C++ `userops`).
    pub userops: UserOpManage,
    /// P-code injection manager (C++ `pcodeinjectlib`).  SLEIGH-backed.
    pub pcodeinjectlib: PcodeInjectLibrarySleigh,
    /// Comments for this architecture (C++ `commentdb`).  // SEAM(comment.cc)
    pub commentdb: CommentDatabase,

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

            infer_funcentry: false,
            return_single: false,
            memset_recover: false,
            add_carry_chain: false,
            v850_indirect_branch: false,
            sparc_struct_return: false,
            ov_less_simplify: false,
            fold_boolean_mask: false,
            input_varnode_adjust: false,
            condexe_block_placement: false,
            dynamic_hash_maxdup_high: false,
            model_stack_probe_loop: false,
            fold_flag_compare: false,
            switch_modulo_bound: false,
            stack_alias_deadstore: false,
            recover_array_stride: false,
            recover_lowered_switch: false,
            strip_stack_guard: false,
            name_style_angr: false,
            present_lessequal: false,
            preserve_thumb_funcptr: false,

            symboltab,
            options: OptionDatabase::new(),
            allacts: ActionDatabase::new(),
            userops: UserOpManage::new(),
            pcodeinjectlib: PcodeInjectLibrarySleigh::new(inject_tempbase),
            commentdb: CommentDatabase::new(),
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
        self.sparc_struct_return = false; // (kuna) default: upstream byte-identical (GH-6882)
        self.ov_less_simplify = true; // (kuna) DIV-2 default-on (GH-7190)
        self.fold_boolean_mask = true; // (kuna) DIV-2 default-on (GH-1282)
        self.input_varnode_adjust = true; // (kuna) DIV-3 default-on (GH-9218)
        self.dynamic_hash_maxdup_high = true; // (kuna) DIV-3 default-on (GH-8467)
        self.fold_flag_compare = true; // (kuna) DIV-3 default-on (GH-1276/8777)
        self.switch_modulo_bound = false; // (kuna) default: upstream byte-identical (GH-9191)
        self.stack_alias_deadstore = false; // (kuna) default: upstream byte-identical (GH-8500)
        self.recover_array_stride = true; // (kuna) DIV-3 default-on (GH-8724)
        self.recover_lowered_switch = true; // (kuna) default-on (angr port)
        self.strip_stack_guard = false; // (kuna) default: upstream byte-identical (angr opt-in)
        self.name_style_angr = true; // (kuna) default-on: angr-style default naming
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
    /// SEAM(W4 register-data decode): the `lanerecords` table is populated by
    /// `decodeRegisterData` from the cspec `<register_data>` tags, which need the
    /// W6 type factory + the spec decode flow.  Until then there are no laned
    /// records, so this returns -1 exactly as the C++ does with an empty table.
    pub fn get_minimum_laned_register_size(&self) -> int4 {
        // lanerecords is empty (the register-data decode is a later seam).
        -1
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
        // ostringstream defname; defname << "func_"; addr.printRaw(defname);
        format!("func_{}", print_raw(addr))
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
    /// SEAM(W4): the IR boundary only reaches `manage()` (const/unique/iop/fspec
    /// spaces + `getConstant`) and `getMinimumLanedRegisterSize`.  The handle
    /// carries an IR-boundary `AddrSpaceManager` cloned from this architecture's
    /// space layout; the lift-emitted varnodes carry their own engine spaces, so
    /// the two managers coexist (documented in `verify_w3_ir_flow`).
    pub fn build_arch_handle(&self) -> ArchHandle {
        let manage = ir_boundary_manager(self.manage());
        let mut seam = ArchSeam::new(manage);
        seam.min_laned_register_size = self.get_minimum_laned_register_size();
        Rc::new(seam)
    }
}

// ---------------------------------------------------------------------------
// Free helpers (kept module-level to avoid borrowing `self` while moving fields)
// ---------------------------------------------------------------------------

/// C++ `Address::printRaw` rendered to the canonical `space:0xoffset` form
/// (enough for the `func_<addr>` default name; the full `printRaw` lives in
/// kuna-base, this is the architecture-side default-name shaping).
fn print_raw(addr: &Address) -> String {
    match addr.get_space() {
        Some(spc) => format!("{}:0x{:x}", spc.get_name(), addr.get_offset()),
        None => format!("0x{:x}", addr.get_offset()),
    }
}

/// Build a fresh IR-boundary [`AddrSpaceManager`] carrying the const/unique/iop/
/// fspec spaces the W3 IR factories reach, sized from the architecture's manager.
///
/// SEAM(W4): the W3 IR only consults the constant space (`newConstant`/
/// `getConstant`), the unique space (`newUnique` + the `VarnodeBank` offset
/// allocator), the iop space (`newVarnodeIop`), and the fspec space
/// (`newVarnodeCallSpecs`).  These are reconstructed here with the same byte
/// sizes/word sizes the architecture's spaces use, so the IR boundary is
/// self-consistent without sharing the (non-`Clone`) engine manager.
fn ir_boundary_manager(src: &AddrSpaceManager) -> AddrSpaceManager {
    use kuna_base::space::{
        addrspace_flags, spacetype, AddrSpace, ConstantSpace, IopSpace, FspecSpace, UniqueSpace,
    };

    let mut m = AddrSpaceManager::new();
    // Constant space (index 0, always present).
    m.insert_space(Rc::new(ConstantSpace::new())).expect("ir manager: constant");
    // Unique space (UniqueSpace has a fixed offset size; index 1).
    let big_end = src.get_default_code_space().map(|s| s.is_big_endian()).unwrap_or(false);
    let uniq_index = m.num_spaces();
    m.insert_space(Rc::new(UniqueSpace::new(uniq_index, 0, big_end)))
        .expect("ir manager: unique");
    // A default code space mirroring the engine's default code space byte/word
    // sizes (the IR rarely needs it, but `newCodeRef`/`artificialHalt` may).
    if let Some(code) = src.get_default_code_space() {
        let ram = AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            code.get_name(),
            code.is_big_endian(),
            code.get_addr_size(),
            code.get_word_size(),
            m.num_spaces(),
            addrspace_flags::hasphysical,
            code.get_delay(),
            1,
        );
        // Ignore a name clash with a reserved space (best-effort mirroring).
        let _ = m.insert_space(Rc::new(ram));
    }
    // iop + fspec spaces (the IR call-machinery seam spaces).
    let next = m.num_spaces();
    let _ = m.insert_space(Rc::new(IopSpace::new(next)));
    let next = m.num_spaces();
    let _ = m.insert_space(Rc::new(FspecSpace::new(next)));
    m
}

#[cfg(test)]
mod tests;

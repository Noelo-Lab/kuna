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
use kuna_base::space::AddrSpaceManager;
use kuna_base::types::{int4, uint4};

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

    // --- W6/W8 subsystems wired by `init` (architecture.hh:211-233) -------
    /// Data-type factory (C++ `types`, a `TypeFactory*`).  Empty until
    /// [`build_typegrp`](Architecture::build_typegrp) + `build_core_types`.
    types: TypeFactoryImpl,
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
            // C++ ctor leaves types/print/defaultfp null; init() fills them.
            types: TypeFactoryImpl::new(),
            print: PrintC::new(),
            proto_models: std::collections::BTreeMap::new(),
            defaultfp: None,
            evalfp_current: None,
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
        // Share the engine's OpBehavior emulation table with `glb` (the C++
        // `Architecture` owns the `TypeOp`s, so `glb->inst[opc]->getBehavior()`
        // reaches them directly).  The `Rc<dyn OpBehavior>` entries are cheap
        // clones; the IR-transform passes (RuleCollapseConstants) fold constants
        // through `glb.op_behavior(opc)`.
        seam.opbehaviors = self.opbehaviors.clone();
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

    // -----------------------------------------------------------------------
    // Owned-subsystem accessors (the `glb->types`/`glb->print`/… surface the
    // ifacedecomp porter confirmed were absent — w9x-arch-engine-glue)
    // -----------------------------------------------------------------------

    /// Borrow the data-type factory (C++ `glb->types`).
    pub fn types(&self) -> &dyn TypeFactory {
        &self.types
    }

    /// Borrow the concrete type factory (when the `TypeFactoryImpl`-specific
    /// builders, e.g. `set_core_type`, are needed by the init pipeline).
    pub fn types_impl(&self) -> &TypeFactoryImpl {
        &self.types
    }

    /// Borrow the c-language printer (C++ `glb->print`).
    pub fn print(&self) -> &PrintC {
        &self.print
    }

    /// Mutably borrow the c-language printer (drives `docFunction` + the print
    /// option setters).
    pub fn print_mut(&mut self) -> &mut PrintC {
        &mut self.print
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
        self.types = TypeFactoryImpl::new();
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
    pub fn finish_typegrp(&self) {
        let manage = self.manage();
        let default_size = manage.get_default_size();
        let default_data_addr_size = manage
            .get_default_data_space()
            .map(|s| s.get_addr_size() as int4)
            .unwrap_or(default_size);
        let stack_pointer_size =
            manage.get_stack_space().map(|s| s.get_addr_size() as int4);
        self.types.set_default_alignment_map();
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
    /// param lists from `<input>`/`<output>` `<pentry>` records).  Until that
    /// cspec pipeline is wired here, a name-only default model is registered so
    /// the engine has a non-null `defaultfp` (recorded as a loss).
    pub fn build_default_proto(&mut self) {
        let mut model = ProtoModel::new(self.manage());
        model.set_name("unknown");
        let rc = Rc::new(model);
        self.register_model(Rc::clone(&rc));
        self.defaultfp = Some(rc);
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
        self.build_typegrp();
        // C++ `TypeFactory::TypeFactory` runs `setupSizes()` (the alignment map
        // + the core sizes) in the constructor, *before* `buildCoreTypes` calls
        // `setCoreType` (which queries the alignment map via `getAlignment`).
        // Mirror that ordering here: finish the size/alignment setup first.
        self.finish_typegrp();
        self.build_core_types()?;
        self.build_default_proto();
        self.build_action();
        self.print.initialize_from_architecture();
        // C++ `symboltab->adjustCaches()` resizes the global scope's per-space
        // maps when the spec decode created new spaces.  The Rust `Database`
        // sizes its scope maps from `num_spaces()` at attach time and the
        // post-engine init adds no spaces beyond those the engine already
        // carried, so no resize is needed here (SEAM(W4): the cache-resize
        // surface lands with the scope-space item if a later spacebase decode
        // adds spaces).
        self.build_instructions();
        // C++ `min_funcsymbol_size = translate->getAlignment()` when <= 8
        // (restoreFromSpec, architecture.cc:646).
        let align = self.translate.get_alignment();
        if align <= 8 {
            self.min_funcsymbol_size = align;
        }
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

    // --- per-function properties (W4 symboltab + W6 fspec) -----------------
    fn set_function_inline(&mut self, name: &str, _val: bool) -> KunaResult<()> {
        // SEAM(W4 symboltab + W6 fspec): same as set_function_extra_pop.
        Err(KunaError::recov(format!("Unknown function name: {name}")))
    }
    fn set_function_no_return(&mut self, name: &str, _val: bool) -> KunaResult<()> {
        // SEAM(W4 symboltab + W6 fspec).
        Err(KunaError::recov(format!("Unknown function name: {name}")))
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

#[cfg(test)]
mod tests;

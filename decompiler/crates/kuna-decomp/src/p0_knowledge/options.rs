//! Port of `decompiler/cpp/options.{cc,hh}` — the `ArchOption` configuration
//! dispatch surface (W4).
//!
//! ## What this module is
//!
//! An *option command* is a user request to change one configuration knob on
//! the [`Architecture`](crate::architecture).  The C++ [`OptionDatabase`] maps a
//! command name to an [`ArchOption`] subclass whose `apply()` does the work and
//! returns a confirmation/failure message.  Commands arrive either directly
//! through [`OptionDatabase::set`] or, from a `.cspec`/`.pspec`, as an
//! `<optionslist>` element decoded by [`OptionDatabase::decode`].
//!
//! This module ports faithfully:
//!   - [`ArchOption`] (trait), [`ArchOption::on_or_off`] (the `on`/`off` parser
//!     and its exact `ParseError` text);
//!   - all ~40 *upstream* `ArchOption` subclasses, each `apply()` transcribed
//!     including the integer/bool parsing helpers, the exact error strings, and
//!     the confirmation messages;
//!   - [`OptionDatabase`] — registration, `set` dispatch (by element id), and
//!     the `decodeOne`/`decode` XML path;
//!   - the upstream `ELEM_*` option element ids (options.cc:45-85), used as the
//!     dispatch keys.
//!
//! ## STUB(W4..W10): `ArchOptionContext` — the `glb->` surface options mutate
//!
//! Every upstream `apply()` reaches into the `Architecture` (`glb`): some flip a
//! plain bool/int field (`glb->readonlypropagate`, `glb->infer_pointers`,
//! `glb->flowoptions`, `glb->max_jumptable_size`, ...), others call into a
//! subsystem that is **not yet alive in the Rust port** — the printer (W8:
//! `glb->print`), the action database (W5: `glb->allacts`), prototype models /
//! `defaultfp` (`fspec`, W6), the symbol table (`glb->symboltab`), the translator
//! (`glb->translate`).  Per the porting wave rules this porter owns only
//! `options.rs`; it must NOT reach into the (stub) [`crate::architecture`] module
//! nor invent its fields.
//!
//! The faithful transcription therefore routes every `glb->` access through the
//! **local boundary trait [`ArchOptionContext`]**, defined here.  Each trait method
//! corresponds one-to-one to a `glb->` access in the C++, documented with the
//! exact C++ line it stands in for.  The methods whose subsystem is alive
//! (`flowoptions`, the plain config bools/ints, `split_datatype_config`,
//! `nan_ignore_*`) are concrete; the methods whose subsystem is W5/W6/W8 carry a
//! `// STUB` note and a typed argument so the *parsing and validation* (the part
//! options.cc owns) is fully ported and the only deferred piece is the final
//! subsystem mutation.  `w4-fw-architecture` / `w4-kuna-p0-pack` implement
//! [`ArchOptionContext`] for the real `Architecture` + `P0Store`.
//!
//! A test-only [`RecordingContext`] implements the trait by recording the calls,
//! so the parsing matrices (exact error text, integer-base handling, message
//! strings) and the `<optionslist>` decode path are exercised end-to-end without
//! the W5+ subsystems.
//!
//! ## kuna options (ADR 0006 / `kuna_stages.cc` settableTable)
//!
//! The 22 kuna-registered options (`compareform`, `arraynotation`,
//! `returnpair`, `namestyle`, ... — the settableTable rows) have their own
//! `apply()` bodies living in their own `kuna_*.rs` modules (other W4 porters'
//! files; e.g. `kuna_v850indbranch::V850IndirectBranchOption`).  Their option
//! element ids live in the `4000+` range, defined in those modules, not here.
//! This module supplies the registration door
//! ([`OptionDatabase::register_option`]) those modules plug into and the
//! *catalog* of kuna option names that must be registered
//! ([`KUNA_OPTION_NAMES`]); wiring each kuna `ArchOption` impl into the database
//! is the `w4-kuna-p0-pack` deliverable (it owns the typed `OptionValues`).  See
//! the module-level LOSS note in the porter's structured output.

use std::collections::BTreeMap;

use kuna_base::error::{KunaError, KunaResult};
use kuna_base::marshal::{Decoder, ElementId, IdRegistry, ATTRIB_CONTENT, ELEM_UNKNOWN};
use kuna_base::types::{int4, uint4};

use crate::flow::flow_flags;

// ---------------------------------------------------------------------------
// Marshaling element ids (options.cc:45-85).
//
// These are the upstream option-command element ids; they double as the
// dispatch keys for the OptionDatabase map (the C++ `ElementId::find(name,0)`
// resolves an option name to exactly this id).  Transcribed verbatim, including
// the out-of-sequence late additions (SPLITDATATYPE=270, JUMPTABLEMAX=271,
// NANIGNORE=272, BRACEFORMAT=284) that match the upstream numbering.
// ---------------------------------------------------------------------------

/// Marshaling element `<aliasblock>` (options.cc:45).
pub const ELEM_ALIASBLOCK: ElementId = ElementId::new("aliasblock", 174);
/// Marshaling element `<allowcontextset>` (options.cc:46).
pub const ELEM_ALLOWCONTEXTSET: ElementId = ElementId::new("allowcontextset", 175);
/// Marshaling element `<analyzeforloops>` (options.cc:47).
pub const ELEM_ANALYZEFORLOOPS: ElementId = ElementId::new("analyzeforloops", 176);
/// Marshaling element `<commentheader>` (options.cc:48).
pub const ELEM_COMMENTHEADER: ElementId = ElementId::new("commentheader", 177);
/// Marshaling element `<commentindent>` (options.cc:49).
pub const ELEM_COMMENTINDENT: ElementId = ElementId::new("commentindent", 178);
/// Marshaling element `<commentinstruction>` (options.cc:50).
pub const ELEM_COMMENTINSTRUCTION: ElementId = ElementId::new("commentinstruction", 179);
/// Marshaling element `<commentstyle>` (options.cc:51).
pub const ELEM_COMMENTSTYLE: ElementId = ElementId::new("commentstyle", 180);
/// Marshaling element `<conventionprinting>` (options.cc:52).
pub const ELEM_CONVENTIONPRINTING: ElementId = ElementId::new("conventionprinting", 181);
/// Marshaling element `<currentaction>` (options.cc:53).
pub const ELEM_CURRENTACTION: ElementId = ElementId::new("currentaction", 182);
/// Marshaling element `<defaultprototype>` (options.cc:54).
pub const ELEM_DEFAULTPROTOTYPE: ElementId = ElementId::new("defaultprototype", 183);
/// Marshaling element `<errorreinterpreted>` (options.cc:55).
pub const ELEM_ERRORREINTERPRETED: ElementId = ElementId::new("errorreinterpreted", 184);
/// Marshaling element `<errortoomanyinstructions>` (options.cc:56).
pub const ELEM_ERRORTOOMANYINSTRUCTIONS: ElementId =
    ElementId::new("errortoomanyinstructions", 185);
/// Marshaling element `<errorunimplemented>` (options.cc:57).
pub const ELEM_ERRORUNIMPLEMENTED: ElementId = ElementId::new("errorunimplemented", 186);
/// Marshaling element `<extrapop>` (options.cc:58).
pub const ELEM_EXTRAPOP: ElementId = ElementId::new("extrapop", 187);
/// Marshaling element `<ignoreunimplemented>` (options.cc:59).
pub const ELEM_IGNOREUNIMPLEMENTED: ElementId = ElementId::new("ignoreunimplemented", 188);
/// Marshaling element `<indentincrement>` (options.cc:60).
pub const ELEM_INDENTINCREMENT: ElementId = ElementId::new("indentincrement", 189);
/// Marshaling element `<inferconstptr>` (options.cc:61).
pub const ELEM_INFERCONSTPTR: ElementId = ElementId::new("inferconstptr", 190);
/// Marshaling element `<inline>` (options.cc:62).
pub const ELEM_INLINE: ElementId = ElementId::new("inline", 191);
/// Marshaling element `<inplaceops>` (options.cc:63).
pub const ELEM_INPLACEOPS: ElementId = ElementId::new("inplaceops", 192);
/// Marshaling element `<integerformat>` (options.cc:64).
pub const ELEM_INTEGERFORMAT: ElementId = ElementId::new("integerformat", 193);
/// Marshaling element `<jumpload>` (options.cc:65).
pub const ELEM_JUMPLOAD: ElementId = ElementId::new("jumpload", 194);
/// Marshaling element `<maxinstruction>` (options.cc:66).
pub const ELEM_MAXINSTRUCTION: ElementId = ElementId::new("maxinstruction", 195);
/// Marshaling element `<maxlinewidth>` (options.cc:67).
pub const ELEM_MAXLINEWIDTH: ElementId = ElementId::new("maxlinewidth", 196);
/// Marshaling element `<namespacestrategy>` (options.cc:68).
pub const ELEM_NAMESPACESTRATEGY: ElementId = ElementId::new("namespacestrategy", 197);
/// Marshaling element `<nocastprinting>` (options.cc:69).
pub const ELEM_NOCASTPRINTING: ElementId = ElementId::new("nocastprinting", 198);
/// Marshaling element `<noreturn>` (options.cc:70).
pub const ELEM_NORETURN: ElementId = ElementId::new("noreturn", 199);
/// Marshaling element `<nullprinting>` (options.cc:71).
pub const ELEM_NULLPRINTING: ElementId = ElementId::new("nullprinting", 200);
/// Marshaling element `<optionslist>` (options.cc:72).
pub const ELEM_OPTIONSLIST: ElementId = ElementId::new("optionslist", 201);
/// Marshaling element `<param1>` (options.cc:73).
pub const ELEM_PARAM1: ElementId = ElementId::new("param1", 202);
/// Marshaling element `<param2>` (options.cc:74).
pub const ELEM_PARAM2: ElementId = ElementId::new("param2", 203);
/// Marshaling element `<param3>` (options.cc:75).
pub const ELEM_PARAM3: ElementId = ElementId::new("param3", 204);
/// Marshaling element `<protoeval>` (options.cc:76).
pub const ELEM_PROTOEVAL: ElementId = ElementId::new("protoeval", 205);
/// Marshaling element `<setaction>` (options.cc:77).
pub const ELEM_SETACTION: ElementId = ElementId::new("setaction", 206);
/// Marshaling element `<setlanguage>` (options.cc:78).
pub const ELEM_SETLANGUAGE: ElementId = ElementId::new("setlanguage", 207);
/// Marshaling element `<splitdatatype>` (options.cc:79).
pub const ELEM_SPLITDATATYPE: ElementId = ElementId::new("splitdatatype", 270);
/// Marshaling element `<structalign>` (options.cc:80).
pub const ELEM_STRUCTALIGN: ElementId = ElementId::new("structalign", 208);
/// Marshaling element `<togglerule>` (options.cc:81).
pub const ELEM_TOGGLERULE: ElementId = ElementId::new("togglerule", 209);
/// Marshaling element `<warning>` (options.cc:82).
pub const ELEM_WARNING: ElementId = ElementId::new("warning", 210);
/// Marshaling element `<jumptablemax>` (options.cc:83).
pub const ELEM_JUMPTABLEMAX: ElementId = ElementId::new("jumptablemax", 271);
/// Marshaling element `<nanignore>` (options.cc:84).
pub const ELEM_NANIGNORE: ElementId = ElementId::new("nanignore", 272);
/// Marshaling element `<braceformat>` (options.cc:85).
pub const ELEM_BRACEFORMAT: ElementId = ElementId::new("braceformat", 284);

/// Every upstream option element id, in OptionDatabase-constructor registration
/// order (options.cc:119-155).  Used to seed the dispatch map and to register
/// these ids in a decode-time [`IdRegistry`].
pub const UPSTREAM_OPTION_ELEMENTS: &[ElementId] = &[
    ELEM_EXTRAPOP,
    ELEM_READONLY, // OptionReadOnly: name "readonly", id reused from marshal.cc (17)
    ELEM_IGNOREUNIMPLEMENTED,
    ELEM_ERRORUNIMPLEMENTED,
    ELEM_ERRORREINTERPRETED,
    ELEM_ERRORTOOMANYINSTRUCTIONS,
    ELEM_DEFAULTPROTOTYPE,
    ELEM_INFERCONSTPTR,
    ELEM_ANALYZEFORLOOPS,
    ELEM_INLINE,
    ELEM_NORETURN,
    ELEM_PROTOEVAL,
    ELEM_WARNING,
    ELEM_NULLPRINTING,
    ELEM_INPLACEOPS,
    ELEM_CONVENTIONPRINTING,
    ELEM_NOCASTPRINTING,
    ELEM_MAXLINEWIDTH,
    ELEM_INDENTINCREMENT,
    ELEM_COMMENTINDENT,
    ELEM_COMMENTSTYLE,
    ELEM_COMMENTHEADER,
    ELEM_COMMENTINSTRUCTION,
    ELEM_INTEGERFORMAT,
    ELEM_BRACEFORMAT,
    ELEM_CURRENTACTION,
    ELEM_ALLOWCONTEXTSET,
    ELEM_SETACTION,
    ELEM_SETLANGUAGE,
    ELEM_JUMPTABLEMAX,
    ELEM_JUMPLOAD,
    ELEM_TOGGLERULE,
    ELEM_ALIASBLOCK,
    ELEM_MAXINSTRUCTION,
    ELEM_NAMESPACESTRATEGY,
    ELEM_SPLITDATATYPE,
    ELEM_NANIGNORE,
];

/// `readonly` option name has no dedicated `ELEM_*` in options.cc — its element
/// id is `ELEM_READONLY = ElementId("readonly",151)`, defined in
/// **architecture.cc** (and registered globally there); `OptionReadOnly`
/// resolves to it via `ElementId::find("readonly",0)`.  The canonical Rust home
/// for this id is the (W4-owned) `architecture` module; a copy with the matching
/// id is declared here so this module's `OptionDatabase::new` and a decode-time
/// [`IdRegistry`] agree without a cross-module dependency on the architecture
/// stub.  // STUB(W4): de-duplicate against `architecture::ELEM_READONLY` when alive.
pub const ELEM_READONLY: ElementId = ElementId::new("readonly", 151);
/// `hideextensions` option name.  Upstream registers no dedicated element id and
/// `OptionDatabase` does NOT register `OptionHideExtensions` (it is reachable
/// only via the console `option` command, which matches the name directly).
/// A kuna-local id (4090, in the 4000+ kuna range) is assigned so the
/// struct/apply transcription is complete and testable; it is never wired into
/// the upstream `OptionDatabase::new` map.
pub const ELEM_HIDEEXTENSIONS: ElementId = ElementId::new("hideextensions", 4090);

// ---------------------------------------------------------------------------
// kuna option names (kuna_stages.cc settableTable, options.cc:156-177).
// ---------------------------------------------------------------------------

/// The kuna-registered option names: the 23 stage-model knobs (`OptionDatabase`
/// ctor, options.cc) plus the 8 analysis-pass gates (per-run `--option <id>
/// on|off` enablement of the `kuna_analysis` passes — see
/// `docs/missing-analyses.md`).  The stage-model knobs' `ArchOption` impls +
/// `ELEM_*` (4000+) live in the `kuna_*.rs` modules; the analysis-pass gates flip
/// a plain `analysis_*` bool on the `Architecture` (no `ELEM_*`), consulted by
/// the console's `commit_analysis_output`.  All route through
/// [`crate::architecture::Architecture::set_kuna_option`].  Listed here so the
/// registration set is documented in one place and a missing wiring is a visible
/// gap, not silent — this list must equal the `SETTABLE_TABLE` rows
/// (`phases.toml`), which `kuna catalog --check` cross-checks.
pub const KUNA_OPTION_NAMES: &[&str] = &[
    "compareform",
    "arraynotation",
    "truthycond",
    "braceelide",
    "warnstyle",
    "arraycoverwidth",
    "emptystrconst",
    "thumbfuncptr",
    "inferfuncentry",
    "returnpair",
    "addcarrychain",
    "ovlesssimplify",
    "booleanmask",
    "simdlane",
    "retsplitglobal",
    "flagcompare",
    "v850indirectbranch",
    "fastfailnoreturn",
    "msvcftol",
    "tailcalljump",
    "tailcallframe",
    "funcboundflow",
    "overlapbranch",
    "cleanupcode",
    "linuxsyscall",
    "switchselector",
    "noreturn_extern",
    "inputvarnodeadjust",
    "retinputhalf",
    // (kuna) DIV-118: a CALL on a block that ends in a no-return halt does not
    // veto the RETURN's output trial in `only_op_use`. Default ON.
    "noreturnretuse",
    "rustabi",
    "condexeplace",
    "sparcstructret",
    "arraystride",
    "stackalias",
    "dynamichashmax",
    "stackprobeloop",
    "memsetrecover",
    "rodatastring",
    "switchmodbound",
    "switchguardbound",
    "switchsharedcase",
    "switchmultipred",
    "unrolledguard",
    "jtsharepartial",
    "noreturn_externmatch",
    "loweredswitch",
    "callsitestackargs",
    "cookiescramble",
    "calleedeadarg",
    "calleepreserves",
    "inputparamgap",
    "varargstackargs",
    "calleearity",
    "calleearityfwd",
    "calleearitylive",
    "calloverlap",
    "spillargtrial",
    "loadguardrange",
    "tiedstorekeep",
    "regionstructure",
    "guardarm",
    "loopcondhoist",
    "regionlooprefine",
    "regionedgeorder",
    "outline",
    "condfold",
    "gotoreduce",
    "ifelseflatten",
    "crossjumprevert",
    "taildup",
    "dedupitetail",
    "iteregion",
    "iteexpr",
    "evalcurrentproto",
    "iteboolean",
    "itecondlist",
    "paramcopyhoist",
    "returndup",
    "orchain",
    "earlyreturn",
    "switchreturn",
    "foldcallret",
    "stackguard",
    "securitycheck",
    "branchflip",
    "loopbreak_recovery",
    "namestyle",
    "realtypes",
    "ctypes",
    "framelayout",
    "voidtailreturn",
    "ptrdepthcap",
    "cortexmpriv",
    "dedupvardecls",
    // (kuna) Analysis-pass gates (per-run `--option <id> on|off`): one settable
    // per `kuna_analysis::passes` pass id, default-on (except `addrtable`, off).
    // These do NOT dispatch through the upstream `OptionDatabase`; like the other
    // kuna options they route to `Architecture::set_kuna_option`, which flips the
    // matching `analysis_*` enable flag the console's `commit_analysis_output`
    // reads. See `docs/missing-analyses.md` "Where these live".
    "noreturn_known",
    // (kuna) PE import-call binding: `externref` over the IAT slots (so
    // `ActionDeindirect` resolves `call [slot]` to the import symbol) + upstream's
    // PE-only no-return API name list. PE/COFF-only; a no-op elsewhere.
    "peimportcall",
    "libproto",
    // (kuna) The measured libc signature extension: the ~200 prototypes the
    // 27-entry `libproto` table does not carry, ranked out of the frozen decbench C
    // corpus and reduced from the platform headers. Imported names only.
    "libcsigs",
    "strings",
    // (kuna) The 2-byte (UTF-16LE) width of the string-literal pass -- Ghidra's
    // `StringsAnalyzer.allCharWidths`, which kuna's 1-byte port left as a documented
    // seam. Without it a wide literal is read as its own first character
    // (`LoadLibraryW("n")`). Default-ON; off leaves the markup exactly 1-byte.
    "widestrings",
    "entry_disc",
    // (kuna) Unmapped-CALL-target entry suppression: the Listing's recursive-descent
    // walk gates every INSTRUCTION address on the executable-range universe but took
    // the direct CALL target unconditionally, so a call into unmapped memory (what
    // anti-disassembly junk behind an always-taken branch decodes to) became a
    // `sub_<addr>` with no bytes and no body.  The call reference is still filed;
    // only the function claim is withheld.  Default-ON; off restores the previous
    // discovery set exactly.
    "unmappedentry",
    // (kuna) PPC64 ELFv2 local-entry entry suppression: the OpenPOWER ELFv2 ABI gives
    // a function a global entry (`st_value`, which materialises the TOC pointer) and a
    // local entry `st_other` bytes later, where an intra-module `bl` lands.  Nothing
    // read `st_other`, so the walk minted a function at every such call target and
    // split every locally called function into an 8-byte named husk plus an anonymous
    // body.  Default-ON; off restores the previous discovery set exactly.
    "ppclocalentry",
    // (kuna) PIC base-register folding in the cross-reference index: in 32-bit
    // position-independent code the address of a string or a global is the sum of a
    // GOT pointer the program materialises at run time (`call <next>; pop ebx; add
    // ebx,imm`) and a displacement, so it is nowhere in the image as a constant and
    // every literal reports being referenced by nothing.  The idiom is interpreted
    // and cross-checked against `_GLOBAL_OFFSET_TABLE_`, and only offered to a
    // function whose own body cannot have changed the register.  Query surface only
    // (`kuna xrefs` / `kuna strings`); default-ON, off restores the previous answer.
    "picbase",
    // (kuna) PE CRT entry-function prototype recovery: a `main` that ignores its
    // arguments reads none of the ABI argument registers, so body-driven parameter
    // recovery finds nothing and declares it `void(void)` while the CRT startup a few
    // lines up calls it with three arguments.  On a PE that startup is in the image and
    // fetches each argument through a named CRT accessor, so the call site names the
    // slots.  Default-ON; off restores the `void(void)` form exactly.
    "entrymainproto",
    "machomain",
    // (kuna) `.eh_frame` LSDA landing-pad discovery — a sub-feature of the
    // always-on `entry_disc` pass (GccExceptionAnalyzer). Default-off
    // (output-changing: adds the discovered exception landing pads as entries).
    "eh_frame_full",
    // (kuna) `.eh_frame` FDE-interior entry suppression: kuna's function symbols
    // carry no extent, so a discovery oracle cannot tell that a candidate sits in
    // the MIDDLE of an existing body — the landing pads `eh_frame_full` emits, the
    // gap starts `aif` emits and the prologue-pattern hits all become top-level
    // functions with their parent's live frame pointer.  Each FDE records one
    // function's `[pcBegin, pcBegin+pcRange)`, so an entry strictly inside one is
    // rejected.  Only single-function FDEs are used (never the linker's whole-PLT
    // FDE), and an entry AT an FDE start is always kept.  Default-ON (DIV-61); off
    // restores the previous discovery set exactly.
    "fdeinterior",
    // (kuna) The full byte-pattern function-start pass (Ghidra FunctionStartAnalyzer
    // over the entire vendored pattern corpus), default-OFF (output-changing:
    // discovers more functions). A separate gate from `entry_disc` (whose always-on
    // oracle 5 ports only a minimal subset).
    "funcstart_patterns",
    // (kuna) The widened ARM Cortex-M hardware vector-table signature (any
    // allocated section, an SRAM/CCM/TCM stack word, and a run of Thumb handler
    // pointers instead of `word[1] == e_entry`). Default-OFF (output-changing:
    // discovers more functions on bare-metal ARM firmware).
    "cortexmvectors",
    "ptrentry",
    // (kuna) ARM PC-relative literal-pool inference: the additive pool-end entry
    // fact plus the paired suppression of the AIF accepts inside a pool.
    // Default-OFF (output-changing: it adds and relocates discovered functions).
    "poolentry",
    "arm_markers",
    "mips_gp",
    // (kuna) i386-PIE PLT-stub decode (angr test_decompiling_nl_i386_pie). A
    // loader-tier gate read via the `kuna_i386_pie_plt` env var (not committed
    // through `OptionDatabase`); routes to `Architecture::set_kuna_option`.
    "i386_pie_plt",
    // (kuna) x86-64 IFUNC (IRELATIVE) PLT-stub naming; loader-tier gate read via
    // the `kuna_ifuncfpret` env var. Default-off opt-in.
    "ifuncfpret",
    // (kuna) Relocatable-object analysis-fact rebase (GH-289): a load-time gate
    // read via the `kuna_relocrebase` env var (the analyzer tier runs inside
    // `load file`). Default-ON (DIV-79).
    "relocrebase",
    // (kuna) Linked-image dynamic-relocation application (DIV-84): a load-time
    // gate read via the `kuna_dynrelocs` env var (the relocations are applied
    // inside `ObjectLoadImage::from_bytes`). Default-ON.
    "dynrelocs",
    // (kuna) PE chained-`UNWIND_INFO` `.pdata` entry suppression (GH-403): a
    // load-time gate read via the `kuna_pdatachained` env var (the entry oracles
    // run inside `load file`). Default-ON.
    "pdatachained",
    // (kuna) Degenerate-symbol-name repair: a load-time gate read via the
    // `kuna_symbolnamerepair` env var (the symbol table is installed inside
    // `load file`). Default-ON.
    "symbolnamerepair",
    // (kuna) Symbol-name character sanitizing (`off|safe|ident`): a load-time
    // gate read via the `kuna_symbolnamechars` env var (names are minted inside
    // `load file`). Default `safe`.
    "symbolnamechars",
    // (kuna) Symbol-name scope-path resource bound (GH-338): the same load-time
    // seam as `symbolnamerepair`, read via the `kuna_symbolnamebound` env var.
    // Not on/off -- the value is the scope-component ceiling. Default 256.
    "symbolnamebound",
    // (kuna) MSVC `__real@` FP-constant COMDAT recovery (DIV-96): a load-time
    // gate read via the `kuna_msvcfpconst` env var (the decoded bytes are
    // materialised inside `ObjectLoadImage::from_relocatable`). Default-ON.
    "msvcfpconst",
    "mips_isa",
    "dwarf",
    // (kuna) ELF data-symbol (`STT_OBJECT`) naming — the data half of the
    // `.symtab`/`.dynsym` walks, installed as named `undefined<size>` globals so a
    // copy-relocated libc extern (`stderr`, `optind`) renders by name instead of
    // `dat_<addr>` (DIV-26/DIV-76). The stream is collected at `load file` and
    // committed at `read symbols`, so this gate is consulted at the commit
    // (`commit_analysis_output`), like the analysis-pass gates. Default-ON.
    "datasyms",
    // (kuna) DWARF `.debug_line` source-line comments; default-off (output-changing).
    "dwarf_lines",
    // (kuna) The DWARF C++ prototype arm: resolve a subprogram definition through
    // its `DW_AT_specification`/`DW_AT_abstract_origin` link (an out-of-line
    // member definition carries no `DW_AT_name` of its own), qualify the name by
    // its namespace/class ancestry, map `DW_TAG_class_type`/reference types, and
    // bind the recovered prototype by entry ADDRESS rather than by name. The
    // producing pass runs at `load file`, upstream of the `option` commands, so
    // the facts are stashed apart and this flag gates their COMMIT
    // (`engine.rs::commit_analysis_output`). Default-ON.
    "cppproto",
    // (kuna) The demangled-C++-signature arm: read the class type for `this` and
    // the declared parameter types straight off a MANGLED SYMBOL, which is the
    // only signature source that survives `strip` on a C++ shared library. Not an
    // on/off flag — Itanium mangling cannot tell a static member function from a
    // non-static one, and adding a `this` that is not there shifts every following
    // parameter, so the value picks how much certainty is required:
    // `off|proven|inferred`, default `proven`. Same deferred shape as `cppproto`
    // (the pass runs at `load file`, the gate applies at the analysis commit).
    "cppsig",
    // (kuna) Full-depth DWARF type resolution: the mapper's recursion guard is
    // upstream's per-DIE re-entry counter (`trackRecursion`) rather than a flat
    // three-hop budget that counted the transparent `typedef`/`const` links, so
    // an ordinary `const char **`/`char *const []` resolves instead of falling
    // back to `void`. The mapping happens at `load file`, upstream of the
    // `option` commands, so the live gate is an env var
    // (`kuna_typedepth::TYPEDEPTH_ENV`) that the CLI exports before the load;
    // this registration keeps the option catalog-visible. Default-ON.
    "typedepth",
    // (kuna) DWARF aggregate-LAYOUT import: an aggregate DIE carries its
    // `DW_AT_byte_size` and its `DW_TAG_member` children (offsets verbatim,
    // bitfields included) onto the interned type instead of becoming a named,
    // EMPTY, zero-size shell. Same load-time shape as `typedepth`: the layout is
    // installed at `load file`, upstream of the `option` commands, so the live
    // gate is an env var (`kuna_dwarfstructs::DWARFSTRUCTS_ENV`) that the CLI
    // exports before the load; this registration keeps the option
    // catalog-visible. Default-ON.
    "dwarfstructs",
    // (kuna) DWARF variant-part import: a `DW_TAG_structure_type` carrying a
    // `DW_TAG_variant_part` -- a Rust tagged enum -- recovers its `DW_AT_discr`
    // discriminant member, each `DW_TAG_variant`'s `DW_AT_discr_value`, and each
    // variant's payload struct, instead of the field-less shell `dwarfstructs`
    // leaves it (a Rust enum carries no `DW_TAG_member` of its own). A variant
    // NAME (`Some`, `Ok`) reaches the emitted type only where exactly one variant
    // claims those bytes; where two do -- every `Result` -- the payload is spelled
    // by offset, because a union member selects itself by offset and the
    // discriminant is never consulted. Same load-time shape as `dwarfstructs`: the
    // live gate is an env var (`kuna_dwarfvariants::DWARFVARIANTS_ENV`) that the
    // CLI exports before the load; this registration keeps the option
    // catalog-visible. Default-ON.
    "dwarfvariants",
    "callfixup",
    "addrtable",
    "operand_refs",
    // (kuna) `FormatStringAnalyzer` half B (`DecompilerDependent`): the console
    // `IfcDecompile` reads this flag after the first decompile to type
    // printf/scanf varargs per call site, then re-decompiles.  Default-off
    // (Ghidra `FormatStringAnalyzer.setDefaultEnablement(false)`).
    "formatstring",
    // (kuna) Listing/xref disassembly tier: a program-wide recursive-descent
    // Listing/xref model built once at load (real-object path only) and shared
    // read-only with consumer passes.  Default-off (the Listing is never built).
    "listing",
    // (kuna) Rooted whole-project discovery used by fast mode: recursively
    // follow direct calls from trustworthy roots and validate address-table
    // targets without the exhaustive AIF gap walk.
    "fast_funcdisc",
    // (kuna) Discovered-no-return consumer: the first Listing/xref consumer, a flow
    // heuristic (callee no-return if ≥3 call sites show no valid fall-through,
    // iterated to a fixpoint over the Listing).  The kuna analog of Ghidra's
    // `FindNoReturnFunctionsAnalyzer`.  Default-off (a heuristic that can be wrong;
    // also requires `--option listing on` to build the Listing it reads).
    "noreturn_disc",
    // (kuna, GH-312) The positive-evidence-only tally for `noreturn_disc`: the
    // legacy predicate counts "the successor is not a decoded instruction start"
    // as a vote for the callee being no-return, but the Listing walk always
    // attempts a call's successor, so that arm fires exactly on a kuna decode
    // failure — three spec gaps forge the verdict and DELETE live code at every
    // caller.  When on, only the terminal arm and the two positive arms (the
    // successor is data / another function's entry) count.  Default-ON (DIV-92);
    // requires the Listing, so every parity gate is byte-identical.
    "noreturn_discstrict",
    // (kuna) Structural no-return propagation consumer: the kuna analog of angr's
    // CFGFast call-graph no-return propagation.  Seeds from the Known no-return set
    // and concludes a function no-return when its last real instruction (skipping
    // trailing NOP padding) is a call/tail-jump to an already-no-return callee,
    // with no returning path — iterated to a fixpoint, with NO evidence threshold
    // (unlike `noreturn_disc`).  Catches custom no-return wrappers (e.g.
    // `xalloc_die`) the name list misses and the ≥3-evidence rule does not reach.
    // Default-off (a heuristic that can be wrong; also requires `--option listing
    // on` to build the Listing it reads).
    "noreturn_propagate",
    // (kuna, decbench F2) The `error(status,…)`-conditional no-return recognizer, a
    // sub-rule of `noreturn_propagate`: glibc `error`/`error_at_line` never return
    // when their `status` argument is a nonzero constant (they `exit(status)`), so a
    // wrapper whose tail is `call error(2,…)` is concluded no-return and its callers
    // drop the dead fall-through. REMOVES CODE. Default-ON (DIV-16); requires the
    // Listing + `noreturn_propagate`, so every parity gate is byte-identical.
    "noreturn_error",
    "noreturn_reach",
    // (kuna) FID fingerprint matcher: the kuna analog of Ghidra's FID identification
    // analyzer.  Over the built Listing it fingerprints each function with the
    // byte-exact operand-masked FNV-1a64 hash and looks the full hash up in a kuna
    // `.fid` database (named by the `kuna_fid_db` env var), renaming a matched
    // `FUN_*`/`sub_*` placeholder back to its library name — re-identifying a
    // function in a STRIPPED binary (e.g. `sub_4017c0` -> `kuna_crc32`).  Default-off
    // (real-ELF path only; also requires `--option listing on` to build the Listing
    // it reads and a configured `.fid` DB).
    "fid",
    // (kuna) MSVC RTTI / vftable class-name recovery: the kuna analog of Ghidra's
    // `RttiAnalyzer` (a Microsoft-PE analyzer).  On a Windows PE, parse the
    // CompleteObjectLocator -> RTTI3/2/1 -> RTTI0 graph in `.rdata`/`.data`, demangle
    // each `.?A...@@` class name, and emit `<Class>::vftable` /
    // `<Class>::RTTI_Complete_Object_Locator` / `<Class>::RTTI_Type_Descriptor`
    // labels so the C++ class names (Box/Shape) surface as recovered symbols.
    // PE-only, default-off (output-changing; real-PE path only, so every ELF/XML
    // parity gate is byte-identical).
    "rtti",
    // (kuna, NOVEL) Itanium (GCC/Clang) RTTI + vtable recovery — the capability
    // Ghidra does NOT have (its `RttiAnalyzer` is Microsoft-only; its GCC class
    // recovery is script-tier and never auto-runs).  On an ELF, every `_ZTI…`
    // typeinfo object is located from the dynamic relocation naming its
    // `__cxxabiv1` typeinfo vtable — an anchor `strip --strip-all` cannot remove
    // from a shared object — its `_ZTS…` name demangled to the class, its base
    // list read for the inheritance displacements, and every `_ZTV…` sub-vtable
    // pointing back at it walked.  Emits `<C>::typeinfo` / `<C>::typeinfo_name` /
    // `<C>::vtable` / `<C>::vtable_for_<Base>` labels and a `<C>::vtable_<i>`
    // function symbol per virtual slot.  ELF-only, default-off (output-changing;
    // real-ELF path only, so every XML parity gate is byte-identical).
    "itaniumrtti",
    // (kuna) Aggressive Instruction Finder gap-walk: the kuna analog of Ghidra's
    // `AggressiveInstructionFinderAnalyzer` (which ships off-by-default with the
    // warning "IT MAY CREATE A LOT OF BAD CODE!").  Over the undefined gaps between
    // discovered functions, speculatively decode each gap start and accept it as a
    // NEW function entry when it (a) disassembles into a valid subroutine and (b)
    // matches a function-start fingerprint shared by >= 4 already-discovered
    // functions.  Finds functions reachable ONLY through an indirect/data path that
    // entry discovery + funcsyms miss.  Default-off (a speculative gap-filler that
    // can create false positives; also requires `--option listing on`).
    "aif",
    // (kuna, GH-299) The aligned slide for the AIF gap cursor: it advances to the
    // next 4-byte boundary instead of the next byte, so only an aligned address or a
    // hole's first byte is a candidate function start.  Removes the mid-body phantom
    // entries the byte-slide plants, and recovers the real entries those accepts
    // consumed.  Default-OFF, carried by the `aggressive` preset; inert without `aif`.
    "aifstrict",
    // (kuna, GH-313) The AIF accept corroboration test: upstream rejects a gap
    // candidate on TWO fingerprint tests and kuna ported only the first, so a
    // self-contained routine that merely reaches a `ret` is accepted on four
    // discovered functions sharing its two-mnemonic prologue.  With this on an
    // accept must either add information (a flow into already-discovered code) or
    // match a prologue 50 discovered functions share.  Default-OFF, carried by the
    // `aggressive` preset; inert without `aif`.
    "aifcorroborate",
    // (kuna) Tail-call function-entry recovery: the recursive-descent Listing walk
    // treats every non-CALL flow target as a same-function successor, so a routine
    // reached only by a tail `B` is absorbed into its caller.  Reads the completed
    // walk and admits such a target as a NEW function entry when a containment model
    // says the branch leaves the caller's region (every predecessor is an
    // unconditional branch, the caller's frame is closed at the branch, and the
    // target's flow region is disjoint from the rest of the caller).  Additive —
    // never rebuilds the Listing, so no discovered entry can be lost.  Default-off;
    // also requires `--option listing on`.
    "tailcallentry",
    // (kuna) Go pclntab function-name recovery: parse the embedded pclntab of a Go
    // binary and name each Go function (`main.main`/`runtime.*` instead of
    // `sub_<addr>`).  The kuna analog of Ghidra's `GolangSymbolAnalyzer`
    // (name-recovery half).  Default-on, but the pass is registered ONLY for a Go
    // binary, so it is a structural no-op on every non-Go target.
    "gopclntab",
    // (kuna) Mach-O Objective-C metadata recovery: when the binary is a Mach-O,
    // walk the `__objc_*` metadata (classlist -> class_t -> class_ro_t ->
    // method_list_t) and rename each IMP function `-[Class sel]` / `+[Class sel]`
    // (the FID-precedent label-gated rename of a `sub_*`/`FUN_*` placeholder), plus
    // emit `_OBJC_CLASS_$_<name>` + selector symbols. Selectors are ASCII (no
    // demangler). The kuna analog of Ghidra's `ObjcTypeMetadataAnalyzer`
    // (name-recovery half). Default-OFF, registered ONLY for a Mach-O binary, so it
    // is a structural no-op on every non-Mach-O target. x86-64, no-chained-fixups
    // path (the arm64 + LC_DYLD_CHAINED_FIXUPS resolver is a deferred follow-on).
    "objc",
    // (kuna) PE PDB metadata recovery: on a Windows PE, read the CodeView
    // fingerprint (`{guid, age, path}`), locate the external `.pdb` (tier-1: the
    // `kuna_pdb_path` env var, the fid `kuna_fid_db` precedent), fingerprint-gate
    // it (the supplied `.pdb`'s guid/age must match the PE's CodeView record — never
    // apply a wrong/stale PDB, the FID full-hash-match discipline), and on a match
    // walk the global symbols (`S_PUB32`/`S_GPROC32`) to rename each stripped
    // `FUN_*`/`sub_*` function to its real name (the FID-precedent label-gated rename
    // of a placeholder; a real symbol is never overwritten). The kuna analog of
    // Ghidra's `PdbUniversalAnalyzer` (name-recovery half). Default-OFF, PE-only, and
    // inert without a fingerprint-matching `.pdb`, so every parity gate is
    // byte-identical. Types/typed-locals/lines are the deferred PR-P2/P3.
    "pdb",
    // (kuna) ET_REL relocatable-object (`.o`) loader capability: load a
    // relocatable object (no PT_LOAD segments) by synthesizing a section layout,
    // applying REL/RELA relocations with architecture-aware instruction
    // encoders, and rebasing symbols. Default ON (it only affects `.o` files,
    // which the PT_LOAD-only loader cannot load at all).
    // Unlike the per-function options, this gates the *loader* (run at `load
    // file`, before any `option` command), so it is bridged across the layer via
    // the `RELOC_OBJECTS_ENV` process env var the console handler writes; see
    // `kuna_analysis::loadimage_object::reloc_objects_enabled`.
    "relocobjects",
    // (kuna §3.7) Mach-O arm64e Apple-Silicon SLEIGH-spec selection: an arm64e
    // Mach-O (`cpusubtype` CPU_SUBTYPE_ARM64E) loads with the
    // `AARCH64:LE:64:AppleSilicon` pointer-auth spec instead of the generic v8A.
    // Spec selection is a LOAD-time decision (`language_id_for`), so the live gate
    // is the `KUNA_MACHO_ARM64E` env var the CLI exports for `--option
    // macho-arm64e on`; this option name records the requested state on the
    // Architecture (catalog consistency). Default-OFF (opt-in until proven); a
    // non-arm64e / non-Mach-O target is untouched.
    "macho-arm64e",
];

/// (kuna) Process env var bridging the `relocobjects` console option to the
/// ET_REL loader, which runs at `load file` — upstream of the per-function
/// option machinery, so a flag on `Architecture` would not reach it in time.
/// `Architecture::set_kuna_option("relocobjects", on|off)` writes `"1"`/`"0"`
/// here; the loader reads it at `from_bytes` time (default ON when unset).
pub const RELOC_OBJECTS_ENV: &str = "KUNA_RELOC_OBJECTS";

// ---------------------------------------------------------------------------
// Typed enums for the option-parsing knobs whose target subsystem is W5+.
//
// These let an apply() finish its *parse + validate* faithfully (the part
// options.cc owns) and hand a typed value to the boundary method; the subsystem
// mutation is the only deferred piece.
// ---------------------------------------------------------------------------

/// Brace-formatting style (C++ `Emit::brace_style`, prettyprint.hh:125).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BraceStyle {
    /// Opening brace on the same line (`Emit::same_line` = 0).
    SameLine,
    /// Opening brace on the next line (`Emit::next_line` = 1).
    NextLine,
    /// Opening brace two lines down (`Emit::skip_line` = 2).
    SkipLine,
}

/// Category of code block a brace style applies to (C++ `OptionBraceFormat`
/// first parameter, options.cc:655-664).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BraceCategory {
    /// The main function body (`function`).
    Function,
    /// `if`/`else` blocks (`ifelse`).
    IfElse,
    /// `do`/`while`/`for` loop blocks (`loop`).
    Loop,
    /// A `switch` block (`switch`).
    Switch,
}

/// Namespace-display strategy (C++ `PrintLanguage::namespace_strategy`,
/// printlanguage.hh:176).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NamespaceStrategy {
    /// Print just enough namespace info to resolve a symbol
    /// (`MINIMAL_NAMESPACES` = 0).
    Minimal,
    /// Never print namespace information (`NO_NAMESPACES` = 1).
    None,
    /// Always print all namespace information (`ALL_NAMESPACES` = 2).
    All,
}

/// Comment-type bit flags (C++ `Comment::comment_type`, comment.hh:53).
///
/// Transcription of `Comment::encodeCommentType` (comment.cc:77); lives here
/// (instead of reaching the stub `comment` module) so `commentheader`/
/// `commentinstruction` can finish their parse.  // STUB(W8): the
/// printer-side `comment` module owns the canonical copy when alive.
pub mod comment_type {
    use kuna_base::types::uint4;
    /// The first user-defined property (`Comment::user1`).
    pub const USER1: uint4 = 1;
    /// The second user-defined property (`Comment::user2`).
    pub const USER2: uint4 = 2;
    /// The third user-defined property (`Comment::user3`).
    pub const USER3: uint4 = 4;
    /// Displayed in the function header (`Comment::header`).
    pub const HEADER: uint4 = 8;
    /// Auto-generated alert (`Comment::warning`).
    pub const WARNING: uint4 = 16;
    /// Auto-generated, displayed in the header (`Comment::warningheader`).
    pub const WARNINGHEADER: uint4 = 32;
}

/// Transcription of `Comment::encodeCommentType` (comment.cc:77-91).
///
/// STUB(W8): the canonical encoder lives on the (stub) `comment` module; this
/// copy lets the comment-toggle options finish parsing.  Throws the exact C++
/// `LowlevelError` text on an unknown name.
fn encode_comment_type(name: &str) -> KunaResult<uint4> {
    match name {
        "user1" => Ok(comment_type::USER1),
        "user2" => Ok(comment_type::USER2),
        "user3" => Ok(comment_type::USER3),
        "header" => Ok(comment_type::HEADER),
        "warning" => Ok(comment_type::WARNING),
        "warningheader" => Ok(comment_type::WARNINGHEADER),
        _ => Err(KunaError::lowlevel(format!("Unknown comment type: {name}"))),
    }
}

// ---------------------------------------------------------------------------
// Integer parsing — `istringstream >> int` with unsetf(dec|hex|oct).
// ---------------------------------------------------------------------------

/// `istringstream s(p1); s.unsetf(ios::dec|ios::hex|ios::oct); s >> val;`
///
/// Faithful model of the C++ `operator>>` extraction used pervasively in the
/// integer options.  With the basefield cleared, base is auto-detected from the
/// prefix: optional sign, then `0x`/`0X` => hex, leading `0` => octal, else
/// decimal.  Returns:
///   - `Some(value)` if at least one digit was extracted (C++ stores the parsed
///     value);
///   - `Some(0)` if the field was **non-empty but had no leading digit** (e.g.
///     `"abc"`, `"-"`, `"  abc"`, `"0xZ"`).  C++11 `num_get` sets failbit *and*
///     stores `0` in this case, overwriting the caller's sentinel — so the
///     caller **accepts** the value 0 rather than throwing;
///   - `None` only if the field was empty or whitespace-only (`""`, `"   "`).
///     C++11 leaves the target variable **unchanged** here, so the caller's
///     sentinel default survives and it rejects — verified against a g++/clang++
///     -std=c++11 `istringstream >> int/unsigned` oracle.
///
/// On overflow the C++ `num_get` facet sets failbit and clamps the target to
/// `numeric_limits<T>::max()` (or `min()` for a negative signed result).  The
/// overflow is measured against the **target type's** width (`int4`/`uint4`),
/// not the `u64` accumulator: a value in `(T::MAX, u64::MAX]` saturates to
/// `T::MAX`, it does **not** wrap-truncate.  The callers only test a sentinel,
/// so an overflow that *did* consume digits returns the saturated value (which
/// differs from the sentinel and so passes the caller's check exactly as
/// upstream — e.g. `maxinstruction "3000000000"` stores `INT_MAX`, stays `>=0`,
/// and is accepted).
fn parse_int_auto<T: IntParse>(s: &str) -> Option<T> {
    let bytes = s.as_bytes();
    let mut i = 0usize;
    // Leading whitespace (C locale isspace): operator>> skips it.
    while i < bytes.len()
        && matches!(bytes[i], b' ' | b'\t' | b'\n' | b'\x0b' | b'\x0c' | b'\r')
    {
        i += 1;
    }
    // C++11 `num_get` distinguishes a truly-empty field (input was empty or
    // whitespace-only) from a non-empty field that simply has no leading digit.
    // The former leaves the target *unchanged* (sentinel survives), the latter
    // stores `0` and sets failbit.  This is decided at the point right after the
    // leading-whitespace skip, before the sign/`0x` prefix is consumed.  Verified
    // against a g++/clang++ -std=c++11 oracle: `iss("")>>val` and `iss("   ")>>val`
    // leave `val=-1`, whereas `iss("abc")`/`iss("-")`/`iss("  abc")` store `val=0`.
    let empty_field = i >= bytes.len();
    let mut neg = false;
    if i < bytes.len() && (bytes[i] == b'+' || bytes[i] == b'-') {
        neg = bytes[i] == b'-';
        i += 1;
    }
    let mut base: u64 = 10;
    if i + 1 < bytes.len() && bytes[i] == b'0' && (bytes[i + 1] == b'x' || bytes[i + 1] == b'X') {
        if i + 2 < bytes.len() && bytes[i + 2].is_ascii_hexdigit() {
            base = 16;
            i += 2;
        } else {
            // "0x" with no hex digit: the subject sequence is just "0".
            return Some(T::from_u64(0, false));
        }
    } else if i < bytes.len() && bytes[i] == b'0' {
        base = 8;
    }
    let mut val: u64 = 0;
    let mut overflow = false;
    let mut any = false;
    while i < bytes.len() {
        let c = bytes[i];
        let d: u64 = match c {
            b'0'..=b'9' => (c - b'0') as u64,
            b'a'..=b'f' => (c - b'a') as u64 + 10,
            b'A'..=b'F' => (c - b'A') as u64 + 10,
            _ => break,
        };
        if d >= base {
            break;
        }
        any = true;
        let (m, o1) = val.overflowing_mul(base);
        let (acc, o2) = m.overflowing_add(d);
        if o1 || o2 {
            overflow = true;
        }
        val = acc;
        i += 1;
    }
    if !any {
        // No digit was extracted.  C++11 `num_get`:
        //   - empty/whitespace-only field -> target left unchanged (return None,
        //     so the caller's sentinel survives and it throws/rejects);
        //   - non-empty field with no leading digit (e.g. "abc", "-", "  abc",
        //     "0xZ", ".5") -> `0` stored with failbit set (return Some(0), which
        //     differs from the sentinel and so the caller *accepts* it).
        if empty_field {
            return None;
        }
        return Some(T::from_u64(0, false));
    }
    // Overflow is whatever overflowed the u64 accumulator OR exceeded the
    // *target* type's representable magnitude for this sign — `num_get` measures
    // against `numeric_limits<T>::max()`, not against u64.  (Verified against a
    // C++11 `istringstream >> int/unsigned` oracle: `"3000000000">>int =
    // INT_MAX (fail)`, `"0x100000000">>uint = UINT_MAX (fail)`, while `"-1">>uint
    // = UINT_MAX (no fail)` — the negate-modulo quirk for an in-range magnitude.)
    let width_overflow = val > T::max_magnitude(neg);
    Some(T::from_u64(val, neg).saturate(overflow || width_overflow, neg))
}

/// Helper trait for [`parse_int_auto`] (the two width/signedness targets the
/// integer options use: `int4` and `uint4`).
trait IntParse: Sized + Copy {
    /// Largest unsigned magnitude representable by the target for this sign,
    /// matching the C++ `num_get` overflow boundary (`numeric_limits<T>::max()`
    /// for the positive/unsigned case, `|numeric_limits<T>::min()|` for the
    /// negative signed case).  A parsed magnitude strictly above this saturates.
    fn max_magnitude(neg: bool) -> u64;
    /// Build the value from an in-range unsigned magnitude + sign.
    fn from_u64(mag: u64, neg: bool) -> Self;
    /// Clamp on overflow, matching `numeric_limits<T>::max()/min()`.
    fn saturate(self, overflow: bool, neg: bool) -> Self;
}

impl IntParse for int4 {
    fn max_magnitude(neg: bool) -> u64 {
        // `int4 = i32`: positive magnitudes up to `i32::MAX` (2147483647),
        // negative magnitudes up to `|i32::MIN|` (2147483648) — `"-2147483648"`
        // parses exactly (no failbit), `"-2147483649"` saturates to INT_MIN.
        if neg {
            (int4::MIN as i64).unsigned_abs() // justified: |i32::MIN| = 2^31, fits u64
        } else {
            int4::MAX as u64 // justified: i32::MAX is non-negative
        }
    }
    fn from_u64(mag: u64, neg: bool) -> Self {
        // `int4 = i32`.  C++ `>> int` truncates the parsed value to int width
        // (well-defined for the in-range magnitudes that reach here — overflow
        // is handled by `saturate`); model with a wrapping narrow then sign.
        let m = mag as i64; // justified: in-range int magnitudes fit i64
        let v = if neg { m.wrapping_neg() } else { m };
        v as int4 // justified: faithful to `>> int4` narrowing for in-range input
    }
    fn saturate(self, overflow: bool, neg: bool) -> Self {
        if overflow {
            if neg {
                int4::MIN
            } else {
                int4::MAX
            }
        } else {
            self
        }
    }
}

impl IntParse for uint4 {
    fn max_magnitude(_neg: bool) -> u64 {
        // `uint4 = u32`: the overflow boundary is `u32::MAX` for *both* signs —
        // `"-1">>unsigned` is the in-range magnitude 1 negated mod 2^32 (=
        // UINT_MAX, no failbit), whereas `"-0x100000000">>unsigned` overflows
        // the magnitude (= UINT_MAX, failbit) before any negate.
        uint4::MAX as u64 // justified: u32::MAX is non-negative
    }
    fn from_u64(mag: u64, neg: bool) -> Self {
        // In-range magnitude; a negative sign applies the C++ unsigned
        // negate-modulo-2^32 quirk.
        let v = if neg { mag.wrapping_neg() } else { mag };
        v as uint4 // justified: faithful to `>> uint4` narrowing for in-range input
    }
    fn saturate(self, overflow: bool, _neg: bool) -> Self {
        if overflow {
            uint4::MAX
        } else {
            self
        }
    }
}

// ---------------------------------------------------------------------------
// ArchOptionContext — the local boundary for the `glb->` surface options mutate.
// ---------------------------------------------------------------------------

/// The slice of the [`Architecture`](crate::architecture) that `ArchOption`
/// `apply()` methods read and write, exposed as a trait so this module can be
/// ported without reaching into the (stub / W4-owned) architecture module.
///
/// Each method maps one-to-one to a `glb->` access in options.cc.  The methods
/// whose subsystem is already alive (`flowoptions`, the plain config fields,
/// `split_datatype_config`, `nan_ignore_*`) are expected to be concrete;
/// methods whose subsystem is W5/W6/W8 carry a `// STUB` note — the caller still
/// fully parses + validates and hands a typed value, so only the final mutation
/// is deferred.  `w4-fw-architecture` / `w4-kuna-p0-pack` implement this for the
/// real `Architecture`.
///
/// Methods return [`KunaResult`] where the C++ access can itself fail (an
/// unknown prototype model, an unknown function name); a `Result` mutation that
/// the C++ does unconditionally is modeled as an infallible method.
pub trait ArchOptionContext {
    // --- plain config fields (alive: trivial bool/int members) -------------

    /// `glb->readonlypropagate = val` (options.cc:300).
    fn set_readonly_propagate(&mut self, val: bool);
    /// `glb->infer_pointers = val` (options.cc:333/337).
    fn set_infer_pointers(&mut self, val: bool);
    /// `glb->analyze_for_loops = val` (options.cc:354).
    fn set_analyze_for_loops(&mut self, val: bool);
    /// `glb->max_jumptable_size = val` (options.cc:886).
    fn set_max_jumptable_size(&mut self, val: uint4);
    /// `glb->max_instructions = val` (options.cc:994).
    fn set_max_instructions(&mut self, val: int4);
    /// `glb->alias_block_level` get/set (options.cc:962/964-970).
    fn alias_block_level(&self) -> int4;
    /// `glb->alias_block_level = val` (options.cc:964-970).
    fn set_alias_block_level(&mut self, val: int4);

    // --- flow option flags (alive: `flow_flags`) ---------------------------

    /// `glb->flowoptions` (read+write of the bit field, options.cc:750/773/...).
    fn flow_options(&self) -> uint4;
    /// Set `glb->flowoptions`.
    fn set_flow_options(&mut self, val: uint4);

    // --- split-datatype config (alive, plus an action toggle, W5) ----------

    /// `glb->split_datatype_config` get/set (options.cc:1046-1049).
    fn split_datatype_config(&self) -> uint4;
    /// Set `glb->split_datatype_config`.
    fn set_split_datatype_config(&mut self, val: uint4);

    // --- nan-ignore config (alive bools, plus a rule toggle, W5) -----------

    /// `glb->nan_ignore_all` get/set (options.cc:1077/1081/...).
    fn nan_ignore_all(&self) -> bool;
    /// Set `glb->nan_ignore_all`.
    fn set_nan_ignore_all(&mut self, val: bool);
    /// `glb->nan_ignore_compare` get/set (options.cc:1078/1082/...).
    fn nan_ignore_compare(&self) -> bool;
    /// Set `glb->nan_ignore_compare`.
    fn set_nan_ignore_compare(&mut self, val: bool);

    // --- prototype models (STUB W6: fspec) ---------------------------------

    /// `glb->defaultfp->setExtraPop(expop)` + the eval-model spreads
    /// (options.cc:280-284).  // STUB(W6)
    fn set_default_extra_pop(&mut self, expop: int4);
    /// Set the per-function extrapop: `fd->getFuncProto().setExtraPop(expop)`
    /// after `symboltab->getGlobalScope()->queryFunction(name)`
    /// (options.cc:273-277).  Returns the unknown-function error faithfully.
    /// // STUB(W4 symboltab + W6 fspec)
    fn set_function_extra_pop(&mut self, name: &str, expop: int4) -> KunaResult<()>;
    /// `glb->setDefaultModel(getModel(p1))` (options.cc:313-316); returns the
    /// unknown-model error.  // STUB(W6)
    fn set_default_model(&mut self, name: &str) -> KunaResult<()>;
    /// `glb->evalfp_current = getModel(p1)` / defaultfp (options.cc:844-852);
    /// returns the unknown-model error.  // STUB(W6)
    fn set_eval_current_model(&mut self, name: &str) -> KunaResult<()>;

    // --- per-function properties (STUB W4 symboltab + W6 fspec) ------------

    /// `fd->getFuncProto().setInline(val)` after a name lookup
    /// (options.cc:368-376).  // STUB
    fn set_function_inline(&mut self, name: &str, val: bool) -> KunaResult<()>;
    /// `fd->getFuncProto().setNoReturn(val)` after a name lookup
    /// (options.cc:394-402).  // STUB
    fn set_function_no_return(&mut self, name: &str, val: bool) -> KunaResult<()>;

    // --- printer (STUB W8: printc / printlanguage) -------------------------

    /// Whether the active printer is the C language (`glb->print->getName() ==
    /// "c-language"`, options.cc:441/456/471).  // STUB(W8)
    fn print_is_c_language(&self) -> bool;
    /// Whether the active print language is one kuna can actually emit.
    ///
    /// (kuna outlang) The display options below used to gate on
    /// `print_is_c_language`, which was the same question while C was the only
    /// back-end. It no longer is: brace format, in-place operators, cast
    /// printing and the rest are meaningful in every language kuna emits, and
    /// gating them on "is C" would silently turn them into no-ops the moment a
    /// second language was selected. The default body preserves the old
    /// behaviour for the stub contexts in the tests.
    fn print_lang_known(&self) -> bool {
        self.print_is_c_language()
    }
    /// `PrintC::setNULLPrinting(val)` (options.cc:444).  // STUB(W8)
    fn set_null_printing(&mut self, val: bool);
    /// `PrintC::setInplaceOps(val)` (options.cc:459).  // STUB(W8)
    fn set_inplace_ops(&mut self, val: bool);
    /// `PrintC::setConvention(val)` (options.cc:474).  // STUB(W8)
    fn set_convention_printing(&mut self, val: bool);
    /// `PrintC::setNoCastPrinting(val)` (options.cc:489).  // STUB(W8)
    fn set_no_cast_printing(&mut self, val: bool);
    /// `PrintC::setHideImpliedExts(val)` (options.cc:504).  // STUB(W8)
    fn set_hide_implied_exts(&mut self, val: bool);
    /// `glb->print->setMaxLineSize(val)` (options.cc:524).  // STUB(W8)
    fn set_max_line_size(&mut self, val: int4);
    /// `glb->print->setIndentIncrement(val)` (options.cc:541).  // STUB(W8)
    fn set_indent_increment(&mut self, val: int4);
    /// `glb->print->setLineCommentIndent(val)` (options.cc:559).  // STUB(W8)
    fn set_line_comment_indent(&mut self, val: int4);
    /// `glb->print->setCommentStyle(p1)` (options.cc:570).  // STUB(W8)
    fn set_comment_style(&mut self, style: &str);
    /// `glb->print->getHeaderComment()` (options.cc:583).  // STUB(W8)
    fn header_comment_flags(&self) -> uint4;
    /// `glb->print->setHeaderComment(flags)` (options.cc:589).  // STUB(W8)
    fn set_header_comment_flags(&mut self, flags: uint4);
    /// `glb->print->getInstructionComment()` (options.cc:604).  // STUB(W8)
    fn instruction_comment_flags(&self) -> uint4;
    /// `glb->print->setInstructionComment(flags)` (options.cc:610).  // STUB(W8)
    fn set_instruction_comment_flags(&mut self, flags: uint4);
    /// `glb->print->setIntegerFormat(p1)` (options.cc:623).  // STUB(W8)
    fn set_integer_format(&mut self, fmt: &str);
    /// `glb->print->setNamespaceStrategy(strategy)` (options.cc:1014).
    /// // STUB(W8)
    fn set_namespace_strategy(&mut self, strategy: NamespaceStrategy);
    /// `PrintC::setBraceFormat{Function,IfElse,Loop,Switch}(style)`
    /// (options.cc:655-664).  // STUB(W8)
    fn set_brace_format(&mut self, category: BraceCategory, style: BraceStyle);
    /// `glb->setPrintLanguage(p1)` (options.cc:865).  // STUB(W8)
    fn set_print_language(&mut self, language: &str);

    // --- action database (STUB W5: action / coreaction) --------------------

    /// `glb->allacts.getCurrent()->setWarning(val,p1)`; `false` => bad
    /// action/rule specifier (options.cc:427).  // STUB(W5)
    fn set_action_warning(&mut self, val: bool, name: &str) -> bool;
    /// `glb->allacts.cloneGroup(p1,p2); setCurrent(p2)` (options.cc:682-683).
    /// // STUB(W5)
    fn clone_action_group(&mut self, from: &str, to: &str);
    /// `glb->allacts.setCurrent(p1)` (options.cc:686/707).  // STUB(W5)
    fn set_current_action(&mut self, name: &str);
    /// `glb->allacts.getCurrentName()` (options.cc:714/715).  // STUB(W5)
    fn current_action_name(&self) -> String;
    /// `glb->allacts.toggleAction(grp,sub,val)` (options.cc:709/714).
    /// // STUB(W5)
    fn toggle_action(&mut self, group: &str, sub: &str, val: bool);
    /// `root->enableRule(path)` on the current action (options.cc:938).
    /// // STUB(W5)
    fn enable_rule(&mut self, path: &str) -> bool;
    /// `root->disableRule(path)` on the current action (options.cc:931).
    /// // STUB(W5)
    fn disable_rule(&mut self, path: &str) -> bool;
    /// Whether a current root Action exists (`glb->allacts.getCurrent() != 0`,
    /// options.cc:926-928).  // STUB(W5)
    fn has_current_action(&self) -> bool;

    // --- translator (STUB W2 reached via W4 glb) ---------------------------

    /// `glb->translate->allowContextSet(val)` (options.cc:732).  // STUB(W4)
    fn allow_context_set(&mut self, val: bool);
}

// ---------------------------------------------------------------------------
// ArchOption trait + onOrOff parser.
// ---------------------------------------------------------------------------

/// Base trait for option classes that affect the [`Architecture`] configuration
/// (C++ `ArchOption`, options.hh:75).
///
/// Each instance affects configuration through its [`apply`](ArchOption::apply)
/// method, run once during initialization (or per console command), returning a
/// confirmation/failure message.
pub trait ArchOption {
    /// The name of the option (C++ `ArchOption::getName`).
    fn get_name(&self) -> &str;

    /// Apply a particular configuration option (C++ `ArchOption::apply`,
    /// options.hh:92).  Up to three optional string parameters tailor the
    /// configuration; returns a confirmation/failure message.
    fn apply(
        &self,
        glb: &mut dyn ArchOptionContext,
        p1: &str,
        p2: &str,
        p3: &str,
    ) -> KunaResult<String>;
}

/// Parse an "on" or "off" string (C++ `ArchOption::onOrOff`, options.cc:91).
///
/// Empty or "on" => `true`, "off" => `false`, anything else throws a
/// `ParseError` with the exact upstream text.
pub fn on_or_off(p: &str) -> KunaResult<bool> {
    if p.is_empty() {
        return Ok(true);
    }
    if p == "on" {
        return Ok(true);
    }
    if p == "off" {
        return Ok(false);
    }
    Err(KunaError::parse("Must specify toggle value, on/off"))
}

// ---------------------------------------------------------------------------
// The upstream ArchOption subclasses.  Each is a unit struct; apply() is a
// faithful transcription of the options.cc body.
// ---------------------------------------------------------------------------

macro_rules! arch_option {
    ($(#[$m:meta])* $ty:ident, $name:literal) => {
        $(#[$m])*
        #[derive(Debug, Clone, Copy, Default)]
        pub struct $ty;
        impl $ty {
            /// Construct the option (C++ default constructor sets `name`).
            pub const fn new() -> Self { $ty }
        }
    };
}

arch_option!(
    /// `extrapop`: set the `extrapop` used by the (default) prototype model
    /// (C++ `OptionExtraPop`, options.cc:257).
    OptionExtraPop, "extrapop");
impl ArchOption for OptionExtraPop {
    fn get_name(&self) -> &str {
        "extrapop"
    }
    fn apply(
        &self,
        glb: &mut dyn ArchOptionContext,
        p1: &str,
        p2: &str,
        _p3: &str,
    ) -> KunaResult<String> {
        let mut expop: int4 = -300;
        if p1 == "unknown" {
            expop = PROTOMODEL_EXTRAPOP_UNKNOWN;
        } else if let Some(v) = parse_int_auto::<int4>(p1) {
            expop = v;
        }
        if expop == -300 {
            return Err(KunaError::parse("Bad extrapop adjustment parameter"));
        }
        if !p2.is_empty() {
            glb.set_function_extra_pop(p2, expop)?;
            Ok(format!("ExtraPop set for function {p2}"))
        } else {
            glb.set_default_extra_pop(expop);
            Ok("Global extrapop set".to_string())
        }
    }
}

arch_option!(
    /// `readonly`: toggle propagation of read-only memory values
    /// (C++ `OptionReadOnly`, options.cc:295).
    OptionReadOnly, "readonly");
impl ArchOption for OptionReadOnly {
    fn get_name(&self) -> &str {
        "readonly"
    }
    fn apply(
        &self,
        glb: &mut dyn ArchOptionContext,
        p1: &str,
        _p2: &str,
        _p3: &str,
    ) -> KunaResult<String> {
        if p1.is_empty() {
            return Err(KunaError::parse(
                "Read-only option must be set \"on\" or \"off\"",
            ));
        }
        let val = on_or_off(p1)?;
        glb.set_readonly_propagate(val);
        if val {
            Ok("Read-only memory locations now propagate as constants".to_string())
        } else {
            Ok("Read-only memory locations now do not propagate".to_string())
        }
    }
}

arch_option!(
    /// `defaultprototype`: set the default prototype model
    /// (C++ `OptionDefaultPrototype`, options.cc:310).
    OptionDefaultPrototype, "defaultprototype");
impl ArchOption for OptionDefaultPrototype {
    fn get_name(&self) -> &str {
        "defaultprototype"
    }
    fn apply(
        &self,
        glb: &mut dyn ArchOptionContext,
        p1: &str,
        _p2: &str,
        _p3: &str,
    ) -> KunaResult<String> {
        // getModel(p1)==0 => LowlevelError("Unknown prototype model :"+p1)
        glb.set_default_model(p1)?;
        Ok(format!("Set default prototype to {p1}"))
    }
}

arch_option!(
    /// `inferconstptr`: toggle constant-pointer inference
    /// (C++ `OptionInferConstPtr`, options.cc:325).
    OptionInferConstPtr, "inferconstptr");
impl ArchOption for OptionInferConstPtr {
    fn get_name(&self) -> &str {
        "inferconstptr"
    }
    fn apply(
        &self,
        glb: &mut dyn ArchOptionContext,
        p1: &str,
        _p2: &str,
        _p3: &str,
    ) -> KunaResult<String> {
        let val = on_or_off(p1)?;
        if val {
            glb.set_infer_pointers(true);
            Ok("Constant pointers are now inferred".to_string())
        } else {
            glb.set_infer_pointers(false);
            Ok("Constant pointers must now be set explicitly".to_string())
        }
    }
}

arch_option!(
    /// `analyzeforloops`: toggle for-loop variable recovery
    /// (C++ `OptionForLoops`, options.cc:351).
    OptionForLoops, "analyzeforloops");
impl ArchOption for OptionForLoops {
    fn get_name(&self) -> &str {
        "analyzeforloops"
    }
    fn apply(
        &self,
        glb: &mut dyn ArchOptionContext,
        p1: &str,
        _p2: &str,
        _p3: &str,
    ) -> KunaResult<String> {
        glb.set_analyze_for_loops(on_or_off(p1)?);
        Ok(format!("Recovery of for-loops is {p1}"))
    }
}

arch_option!(
    /// `inline`: mark/unmark a function as inline
    /// (C++ `OptionInline`, options.cc:365).
    OptionInline, "inline");
impl ArchOption for OptionInline {
    fn get_name(&self) -> &str {
        "inline"
    }
    fn apply(
        &self,
        glb: &mut dyn ArchOptionContext,
        p1: &str,
        p2: &str,
        _p3: &str,
    ) -> KunaResult<String> {
        let val = if p2.is_empty() { true } else { p2 == "true" };
        // queryFunction(p1)==0 => RecovError("Unknown function name: "+p1)
        glb.set_function_inline(p1, val)?;
        let prop = if val { "true" } else { "false" };
        Ok(format!("Inline property for function {p1} = {prop}"))
    }
}

arch_option!(
    /// `noreturn`: mark/unmark a function with the noreturn property
    /// (C++ `OptionNoReturn`, options.cc:391).
    OptionNoReturn, "noreturn");
impl ArchOption for OptionNoReturn {
    fn get_name(&self) -> &str {
        "noreturn"
    }
    fn apply(
        &self,
        glb: &mut dyn ArchOptionContext,
        p1: &str,
        p2: &str,
        _p3: &str,
    ) -> KunaResult<String> {
        let val = if p2.is_empty() { true } else { p2 == "true" };
        glb.set_function_no_return(p1, val)?;
        let prop = if val { "true" } else { "false" };
        Ok(format!("No return property for function {p1} = {prop}"))
    }
}

arch_option!(
    /// `warning`: toggle a per-action/rule warning
    /// (C++ `OptionWarning`, options.cc:417).
    OptionWarning, "warning");
impl ArchOption for OptionWarning {
    fn get_name(&self) -> &str {
        "warning"
    }
    fn apply(
        &self,
        glb: &mut dyn ArchOptionContext,
        p1: &str,
        p2: &str,
        _p3: &str,
    ) -> KunaResult<String> {
        if p1.is_empty() {
            return Err(KunaError::parse("No action/rule specified"));
        }
        let val = if p2.is_empty() { true } else { on_or_off(p2)? };
        if !glb.set_action_warning(val, p1) {
            return Err(KunaError::recov(format!("Bad action/rule specifier: {p1}")));
        }
        let prop = if val { "on" } else { "off" };
        Ok(format!("Warnings for {p1} turned {prop}"))
    }
}

arch_option!(
    /// `nullprinting`: toggle whether null pointers print as "NULL"
    /// (C++ `OptionNullPrinting`, options.cc:437).
    OptionNullPrinting, "nullprinting");
impl ArchOption for OptionNullPrinting {
    fn get_name(&self) -> &str {
        "nullprinting"
    }
    fn apply(
        &self,
        glb: &mut dyn ArchOptionContext,
        p1: &str,
        _p2: &str,
        _p3: &str,
    ) -> KunaResult<String> {
        let val = on_or_off(p1)?;
        if !glb.print_lang_known() {
            return Ok("Only a known output language accepts the null printing option".to_string());
        }
        glb.set_null_printing(val);
        let prop = if val { "on" } else { "off" };
        Ok(format!("Null printing turned {prop}"))
    }
}

arch_option!(
    /// `inplaceops`: toggle in-place operators (+=, *=, ...)
    /// (C++ `OptionInPlaceOps`, options.cc:452).
    OptionInPlaceOps, "inplaceops");
impl ArchOption for OptionInPlaceOps {
    fn get_name(&self) -> &str {
        "inplaceops"
    }
    fn apply(
        &self,
        glb: &mut dyn ArchOptionContext,
        p1: &str,
        _p2: &str,
        _p3: &str,
    ) -> KunaResult<String> {
        let val = on_or_off(p1)?;
        if !glb.print_lang_known() {
            return Ok("Can only set inplace operators for a known output language".to_string());
        }
        glb.set_inplace_ops(val);
        let prop = if val { "on" } else { "off" };
        Ok(format!("Inplace operators turned {prop}"))
    }
}

arch_option!(
    /// `conventionprinting`: toggle printing of the calling convention
    /// (C++ `OptionConventionPrinting`, options.cc:467).
    OptionConventionPrinting, "conventionprinting");
impl ArchOption for OptionConventionPrinting {
    fn get_name(&self) -> &str {
        "conventionprinting"
    }
    fn apply(
        &self,
        glb: &mut dyn ArchOptionContext,
        p1: &str,
        _p2: &str,
        _p3: &str,
    ) -> KunaResult<String> {
        let val = on_or_off(p1)?;
        if !glb.print_lang_known() {
            return Ok("Can only set convention printing for a known output language".to_string());
        }
        glb.set_convention_printing(val);
        let prop = if val { "on" } else { "off" };
        Ok(format!("Convention printing turned {prop}"))
    }
}

arch_option!(
    /// `nocastprinting`: toggle whether cast syntax is emitted or stripped
    /// (C++ `OptionNoCastPrinting`, options.cc:482).
    OptionNoCastPrinting, "nocastprinting");
impl ArchOption for OptionNoCastPrinting {
    fn get_name(&self) -> &str {
        "nocastprinting"
    }
    fn apply(
        &self,
        glb: &mut dyn ArchOptionContext,
        p1: &str,
        _p2: &str,
        _p3: &str,
    ) -> KunaResult<String> {
        let val = on_or_off(p1)?;
        // dynamic_cast<PrintC*> == 0 => not C language.
        if !glb.print_lang_known() {
            return Ok("Can only set no cast printing for a known output language".to_string());
        }
        glb.set_no_cast_printing(val);
        let prop = if val { "on" } else { "off" };
        Ok(format!("No cast printing turned {prop}"))
    }
}

arch_option!(
    /// `hideextensions`: toggle whether implied ZEXT/SEXT are printed
    /// (C++ `OptionHideExtensions`, options.cc:497).  Not registered in the
    /// `OptionDatabase` ctor; reachable via the console `option` command.
    OptionHideExtensions, "hideextensions");
impl ArchOption for OptionHideExtensions {
    fn get_name(&self) -> &str {
        "hideextensions"
    }
    fn apply(
        &self,
        glb: &mut dyn ArchOptionContext,
        p1: &str,
        _p2: &str,
        _p3: &str,
    ) -> KunaResult<String> {
        let val = on_or_off(p1)?;
        if !glb.print_lang_known() {
            return Ok("Can only toggle extension hiding for a known output language".to_string());
        }
        glb.set_hide_implied_exts(val);
        let prop = if val { "on" } else { "off" };
        Ok(format!("Implied extension hiding turned {prop}"))
    }
}

arch_option!(
    /// `maxlinewidth`: max characters per decompiled line
    /// (C++ `OptionMaxLineWidth`, options.cc:515).
    OptionMaxLineWidth, "maxlinewidth");
impl ArchOption for OptionMaxLineWidth {
    fn get_name(&self) -> &str {
        "maxlinewidth"
    }
    fn apply(
        &self,
        glb: &mut dyn ArchOptionContext,
        p1: &str,
        _p2: &str,
        _p3: &str,
    ) -> KunaResult<String> {
        // int4 val = -1; s >> val; if(val==-1) ParseError.
        let val = parse_int_auto::<int4>(p1).unwrap_or(-1);
        if val == -1 {
            return Err(KunaError::parse("Must specify integer linewidth"));
        }
        glb.set_max_line_size(val);
        Ok(format!("Maximum line width set to {p1}"))
    }
}

arch_option!(
    /// `indentincrement`: characters to indent per nested scope
    /// (C++ `OptionIndentIncrement`, options.cc:532).
    OptionIndentIncrement, "indentincrement");
impl ArchOption for OptionIndentIncrement {
    fn get_name(&self) -> &str {
        "indentincrement"
    }
    fn apply(
        &self,
        glb: &mut dyn ArchOptionContext,
        p1: &str,
        _p2: &str,
        _p3: &str,
    ) -> KunaResult<String> {
        let val = parse_int_auto::<int4>(p1).unwrap_or(-1);
        if val == -1 {
            return Err(KunaError::parse("Must specify integer increment"));
        }
        glb.set_indent_increment(val);
        Ok(format!("Characters per indent level set to {p1}"))
    }
}

arch_option!(
    /// `commentindent`: characters to indent comment lines
    /// (C++ `OptionCommentIndent`, options.cc:550).
    OptionCommentIndent, "commentindent");
impl ArchOption for OptionCommentIndent {
    fn get_name(&self) -> &str {
        "commentindent"
    }
    fn apply(
        &self,
        glb: &mut dyn ArchOptionContext,
        p1: &str,
        _p2: &str,
        _p3: &str,
    ) -> KunaResult<String> {
        let val = parse_int_auto::<int4>(p1).unwrap_or(-1);
        if val == -1 {
            return Err(KunaError::parse("Must specify integer comment indent"));
        }
        glb.set_line_comment_indent(val);
        Ok(format!("Comment indent set to {p1}"))
    }
}

arch_option!(
    /// `commentstyle`: style of comment emitted
    /// (C++ `OptionCommentStyle`, options.cc:567).
    OptionCommentStyle, "commentstyle");
impl ArchOption for OptionCommentStyle {
    fn get_name(&self) -> &str {
        "commentstyle"
    }
    fn apply(
        &self,
        glb: &mut dyn ArchOptionContext,
        p1: &str,
        _p2: &str,
        _p3: &str,
    ) -> KunaResult<String> {
        glb.set_comment_style(p1);
        Ok(format!("Comment style set to {p1}"))
    }
}

arch_option!(
    /// `commentheader`: toggle header comment types
    /// (C++ `OptionCommentHeader`, options.cc:579).
    OptionCommentHeader, "commentheader");
impl ArchOption for OptionCommentHeader {
    fn get_name(&self) -> &str {
        "commentheader"
    }
    fn apply(
        &self,
        glb: &mut dyn ArchOptionContext,
        p1: &str,
        p2: &str,
        _p3: &str,
    ) -> KunaResult<String> {
        let toggle = on_or_off(p2)?;
        let mut flags = glb.header_comment_flags();
        let val = encode_comment_type(p1)?;
        if toggle {
            flags |= val;
        } else {
            flags &= !val;
        }
        glb.set_header_comment_flags(flags);
        let prop = if toggle { "on" } else { "off" };
        Ok(format!("Header comment type {p1} turned {prop}"))
    }
}

arch_option!(
    /// `commentinstruction`: toggle body comment types
    /// (C++ `OptionCommentInstruction`, options.cc:600).
    OptionCommentInstruction, "commentinstruction");
impl ArchOption for OptionCommentInstruction {
    fn get_name(&self) -> &str {
        "commentinstruction"
    }
    fn apply(
        &self,
        glb: &mut dyn ArchOptionContext,
        p1: &str,
        p2: &str,
        _p3: &str,
    ) -> KunaResult<String> {
        let toggle = on_or_off(p2)?;
        let mut flags = glb.instruction_comment_flags();
        let val = encode_comment_type(p1)?;
        if toggle {
            flags |= val;
        } else {
            flags &= !val;
        }
        glb.set_instruction_comment_flags(flags);
        let prop = if toggle { "on" } else { "off" };
        Ok(format!("Instruction comment type {p1} turned {prop}"))
    }
}

arch_option!(
    /// `integerformat`: integer-emission strategy ("hex"/"dec"/"best")
    /// (C++ `OptionIntegerFormat`, options.cc:620).
    OptionIntegerFormat, "integerformat");
impl ArchOption for OptionIntegerFormat {
    fn get_name(&self) -> &str {
        "integerformat"
    }
    fn apply(
        &self,
        glb: &mut dyn ArchOptionContext,
        p1: &str,
        _p2: &str,
        _p3: &str,
    ) -> KunaResult<String> {
        glb.set_integer_format(p1);
        Ok(format!("Integer format set to {p1}"))
    }
}

arch_option!(
    /// `braceformat`: brace-formatting strategy per block category
    /// (C++ `OptionBraceFormat`, options.cc:640).
    OptionBraceFormat, "braceformat");
impl ArchOption for OptionBraceFormat {
    fn get_name(&self) -> &str {
        "braceformat"
    }
    fn apply(
        &self,
        glb: &mut dyn ArchOptionContext,
        p1: &str,
        p2: &str,
        _p3: &str,
    ) -> KunaResult<String> {
        if !glb.print_lang_known() {
            return Ok("Can only set brace formatting for a known output language".to_string());
        }
        let style = match p2 {
            "same" => BraceStyle::SameLine,
            "next" => BraceStyle::NextLine,
            "skip" => BraceStyle::SkipLine,
            _ => return Err(KunaError::parse(format!("Unknown brace style: {p2}"))),
        };
        let category = match p1 {
            "function" => BraceCategory::Function,
            "ifelse" => BraceCategory::IfElse,
            "loop" => BraceCategory::Loop,
            "switch" => BraceCategory::Switch,
            _ => return Err(KunaError::parse(format!("Unknown brace format category: {p1}"))),
        };
        glb.set_brace_format(category, style);
        Ok(format!("Brace formatting for {p1} set to {p2}"))
    }
}

arch_option!(
    /// `setaction`: establish a new root Action
    /// (C++ `OptionSetAction`, options.cc:675).
    OptionSetAction, "setaction");
impl ArchOption for OptionSetAction {
    fn get_name(&self) -> &str {
        "setaction"
    }
    fn apply(
        &self,
        glb: &mut dyn ArchOptionContext,
        p1: &str,
        p2: &str,
        _p3: &str,
    ) -> KunaResult<String> {
        if p1.is_empty() {
            return Err(KunaError::parse("Must specify preexisting action"));
        }
        if !p2.is_empty() {
            glb.clone_action_group(p1, p2);
            glb.set_current_action(p2);
            return Ok(format!("Created {p2} by cloning {p1} and made it current"));
        }
        glb.set_current_action(p1);
        Ok(format!("Set current action to {p1}"))
    }
}

arch_option!(
    /// `currentaction`: toggle a sub-group of actions in a root Action
    /// (C++ `OptionCurrentAction`, options.cc:698).
    OptionCurrentAction, "currentaction");
impl ArchOption for OptionCurrentAction {
    fn get_name(&self) -> &str {
        "currentaction"
    }
    fn apply(
        &self,
        glb: &mut dyn ArchOptionContext,
        p1: &str,
        p2: &str,
        p3: &str,
    ) -> KunaResult<String> {
        if p1.is_empty() || p2.is_empty() {
            return Err(KunaError::parse("Must specify subaction, on/off"));
        }
        let mut res = "Toggled ".to_string();
        if !p3.is_empty() {
            glb.set_current_action(p1);
            let val = on_or_off(p3)?;
            glb.toggle_action(p1, p2, val);
            res += &format!("{p2} in action {p1}");
        } else {
            let val = on_or_off(p2)?;
            let cur = glb.current_action_name();
            glb.toggle_action(&cur, p1, val);
            res += &format!("{p1} in action {cur}");
        }
        Ok(res)
    }
}

arch_option!(
    /// `allowcontextset`: toggle whether disassembly can modify context
    /// (C++ `OptionAllowContextSet`, options.cc:725).
    OptionAllowContextSet, "allowcontextset");
impl ArchOption for OptionAllowContextSet {
    fn get_name(&self) -> &str {
        "allowcontextset"
    }
    fn apply(
        &self,
        glb: &mut dyn ArchOptionContext,
        p1: &str,
        _p2: &str,
        _p3: &str,
    ) -> KunaResult<String> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        let res = format!("Toggled allowcontextset to {prop}");
        glb.allow_context_set(val);
        Ok(res)
    }
}

arch_option!(
    /// `ignoreunimplemented`: treat unimplemented instructions as no-ops
    /// (C++ `OptionIgnoreUnimplemented`, options.cc:742).
    OptionIgnoreUnimplemented, "ignoreunimplemented");
impl ArchOption for OptionIgnoreUnimplemented {
    fn get_name(&self) -> &str {
        "ignoreunimplemented"
    }
    fn apply(
        &self,
        glb: &mut dyn ArchOptionContext,
        p1: &str,
        _p2: &str,
        _p3: &str,
    ) -> KunaResult<String> {
        let val = on_or_off(p1)?;
        let mut flags = glb.flow_options();
        if val {
            flags |= flow_flags::ignore_unimplemented;
            glb.set_flow_options(flags);
            Ok("Unimplemented instructions are now ignored (treated as nop)".to_string())
        } else {
            flags &= !flow_flags::ignore_unimplemented;
            glb.set_flow_options(flags);
            Ok("Unimplemented instructions now generate warnings".to_string())
        }
    }
}

arch_option!(
    /// `errorunimplemented`: treat unimplemented instructions as fatal
    /// (C++ `OptionErrorUnimplemented`, options.cc:765).
    OptionErrorUnimplemented, "errorunimplemented");
impl ArchOption for OptionErrorUnimplemented {
    fn get_name(&self) -> &str {
        "errorunimplemented"
    }
    fn apply(
        &self,
        glb: &mut dyn ArchOptionContext,
        p1: &str,
        _p2: &str,
        _p3: &str,
    ) -> KunaResult<String> {
        let val = on_or_off(p1)?;
        let mut flags = glb.flow_options();
        if val {
            flags |= flow_flags::error_unimplemented;
            glb.set_flow_options(flags);
            Ok("Unimplemented instructions are now a fatal error".to_string())
        } else {
            flags &= !flow_flags::error_unimplemented;
            glb.set_flow_options(flags);
            Ok("Unimplemented instructions now NOT a fatal error".to_string())
        }
    }
}

arch_option!(
    /// `errorreinterpreted`: treat off-cut reinterpretation as fatal
    /// (C++ `OptionErrorReinterpreted`, options.cc:788).
    OptionErrorReinterpreted, "errorreinterpreted");
impl ArchOption for OptionErrorReinterpreted {
    fn get_name(&self) -> &str {
        "errorreinterpreted"
    }
    fn apply(
        &self,
        glb: &mut dyn ArchOptionContext,
        p1: &str,
        _p2: &str,
        _p3: &str,
    ) -> KunaResult<String> {
        let val = on_or_off(p1)?;
        let mut flags = glb.flow_options();
        if val {
            flags |= flow_flags::error_reinterpreted;
            glb.set_flow_options(flags);
            Ok("Instruction reinterpretation is now a fatal error".to_string())
        } else {
            flags &= !flow_flags::error_reinterpreted;
            glb.set_flow_options(flags);
            Ok("Instruction reinterpretation is now NOT a fatal error".to_string())
        }
    }
}

arch_option!(
    /// `errortoomanyinstructions`: treat too-many-instructions as fatal
    /// (C++ `OptionErrorTooManyInstructions`, options.cc:811).
    OptionErrorTooManyInstructions, "errortoomanyinstructions");
impl ArchOption for OptionErrorTooManyInstructions {
    fn get_name(&self) -> &str {
        "errortoomanyinstructions"
    }
    fn apply(
        &self,
        glb: &mut dyn ArchOptionContext,
        p1: &str,
        _p2: &str,
        _p3: &str,
    ) -> KunaResult<String> {
        let val = on_or_off(p1)?;
        let mut flags = glb.flow_options();
        if val {
            flags |= flow_flags::error_toomanyinstructions;
            glb.set_flow_options(flags);
            Ok("Too many instructions are now a fatal error".to_string())
        } else {
            flags &= !flow_flags::error_toomanyinstructions;
            glb.set_flow_options(flags);
            Ok("Too many instructions are now NOT a fatal error".to_string())
        }
    }
}

arch_option!(
    /// `protoeval`: prototype model for the current function's parameters
    /// (C++ `OptionProtoEval`, options.cc:836).
    OptionProtoEval, "protoeval");
impl ArchOption for OptionProtoEval {
    fn get_name(&self) -> &str {
        "protoeval"
    }
    fn apply(
        &self,
        glb: &mut dyn ArchOptionContext,
        p1: &str,
        _p2: &str,
        _p3: &str,
    ) -> KunaResult<String> {
        if p1.is_empty() {
            return Err(KunaError::parse("Must specify prototype model"));
        }
        // "default" => defaultfp; else getModel(p1)==0 => ParseError.
        glb.set_eval_current_model(p1)?;
        Ok(format!("Set current evaluation to {p1}"))
    }
}

arch_option!(
    /// `setlanguage`: language emitted by the decompiler
    /// (C++ `OptionSetLanguage`, options.cc:860).
    OptionSetLanguage, "setlanguage");
impl ArchOption for OptionSetLanguage {
    fn get_name(&self) -> &str {
        "setlanguage"
    }
    fn apply(
        &self,
        glb: &mut dyn ArchOptionContext,
        p1: &str,
        _p2: &str,
        _p3: &str,
    ) -> KunaResult<String> {
        // (kuna outlang) C++ `Architecture::setPrintLanguage` throws
        // `LowlevelError("Unknown print language: ...")` on a name no capability
        // claims; kuna owns a fixed set, so the check is a table lookup. Without
        // it a typo would silently keep emitting C under a name that says
        // otherwise.
        let Some(lang) = crate::kuna_lang::OutLang::from_print_name(p1) else {
            return Err(KunaError::parse(format!("Unknown print language: {p1}")));
        };
        glb.set_print_language(lang.print_name());
        Ok(format!("Decompiler produces {}", lang.print_name()))
    }
}

arch_option!(
    /// `jumptablemax`: max recovered entries for a single jump table
    /// (C++ `OptionJumpTableMax`, options.cc:877).
    OptionJumpTableMax, "jumptablemax");
impl ArchOption for OptionJumpTableMax {
    fn get_name(&self) -> &str {
        "jumptablemax"
    }
    fn apply(
        &self,
        glb: &mut dyn ArchOptionContext,
        p1: &str,
        _p2: &str,
        _p3: &str,
    ) -> KunaResult<String> {
        // uint4 val = 0; s >> val; if(val==0) ParseError.
        let val = parse_int_auto::<uint4>(p1).unwrap_or(0);
        if val == 0 {
            return Err(KunaError::parse("Must specify integer maximum"));
        }
        glb.set_max_jumptable_size(val);
        Ok(format!("Maximum jumptable size set to {p1}"))
    }
}

arch_option!(
    /// `jumpload`: toggle recording of switch-table loads
    /// (C++ `OptionJumpLoad`, options.cc:895).
    OptionJumpLoad, "jumpload");
impl ArchOption for OptionJumpLoad {
    fn get_name(&self) -> &str {
        "jumpload"
    }
    fn apply(
        &self,
        glb: &mut dyn ArchOptionContext,
        p1: &str,
        _p2: &str,
        _p3: &str,
    ) -> KunaResult<String> {
        let val = on_or_off(p1)?;
        let mut flags = glb.flow_options();
        if val {
            flags |= flow_flags::record_jumploads;
            glb.set_flow_options(flags);
            Ok("Jumptable analysis will record loads required to calculate jump address".to_string())
        } else {
            flags &= !flow_flags::record_jumploads;
            glb.set_flow_options(flags);
            Ok("Jumptable analysis will NOT record loads".to_string())
        }
    }
}

arch_option!(
    /// `togglerule`: toggle a specific Rule in the current Action
    /// (C++ `OptionToggleRule`, options.cc:917).
    OptionToggleRule, "togglerule");
impl ArchOption for OptionToggleRule {
    fn get_name(&self) -> &str {
        "togglerule"
    }
    fn apply(
        &self,
        glb: &mut dyn ArchOptionContext,
        p1: &str,
        p2: &str,
        _p3: &str,
    ) -> KunaResult<String> {
        if p1.is_empty() {
            return Err(KunaError::parse("Must specify rule path"));
        }
        if p2.is_empty() {
            return Err(KunaError::parse("Must specify on/off"));
        }
        let val = on_or_off(p2)?;
        // getCurrent()==0 => LowlevelError("Missing current action")
        if !glb.has_current_action() {
            return Err(KunaError::lowlevel("Missing current action"));
        }
        let mut res;
        if !val {
            if glb.disable_rule(p1) {
                res = "Successfully disabled".to_string();
            } else {
                res = "Failed to disable".to_string();
            }
            res += " rule";
        } else {
            if glb.enable_rule(p1) {
                res = "Successfully enabled".to_string();
            } else {
                res = "Failed to enable".to_string();
            }
            res += " rule";
        }
        Ok(res)
    }
}

arch_option!(
    /// `aliasblock`: how locked data-types on the stack block aliases
    /// (C++ `OptionAliasBlock`, options.cc:957).
    OptionAliasBlock, "aliasblock");
impl ArchOption for OptionAliasBlock {
    fn get_name(&self) -> &str {
        "aliasblock"
    }
    fn apply(
        &self,
        glb: &mut dyn ArchOptionContext,
        p1: &str,
        _p2: &str,
        _p3: &str,
    ) -> KunaResult<String> {
        if p1.is_empty() {
            return Err(KunaError::parse("Must specify alias block level"));
        }
        let old_val = glb.alias_block_level();
        let new_val = match p1 {
            "none" => 0,
            "struct" => 1,
            "array" => 2,
            "all" => 3,
            _ => return Err(KunaError::parse(format!("Unknown alias block level: {p1}"))),
        };
        glb.set_alias_block_level(new_val);
        if old_val == new_val {
            return Ok("Alias block level unchanged".to_string());
        }
        Ok(format!("Alias block level set to {p1}"))
    }
}

arch_option!(
    /// `maxinstruction`: max instructions processed per function
    /// (C++ `OptionMaxInstruction`, options.cc:982).
    OptionMaxInstruction, "maxinstruction");
impl ArchOption for OptionMaxInstruction {
    fn get_name(&self) -> &str {
        "maxinstruction"
    }
    fn apply(
        &self,
        glb: &mut dyn ArchOptionContext,
        p1: &str,
        _p2: &str,
        _p3: &str,
    ) -> KunaResult<String> {
        if p1.is_empty() {
            return Err(KunaError::parse("Must specify number of instructions"));
        }
        // int4 newMax = -1; s1 >> newMax; if(newMax<0) ParseError.
        let new_max = parse_int_auto::<int4>(p1).unwrap_or(-1);
        if new_max < 0 {
            return Err(KunaError::parse("Bad maxinstruction parameter"));
        }
        glb.set_max_instructions(new_max);
        Ok("Maximum instructions per function set".to_string())
    }
}

arch_option!(
    /// `namespacestrategy`: how namespace tokens are displayed
    /// (C++ `OptionNamespaceStrategy`, options.cc:1002).
    OptionNamespaceStrategy, "namespacestrategy");
impl ArchOption for OptionNamespaceStrategy {
    fn get_name(&self) -> &str {
        "namespacestrategy"
    }
    fn apply(
        &self,
        glb: &mut dyn ArchOptionContext,
        p1: &str,
        _p2: &str,
        _p3: &str,
    ) -> KunaResult<String> {
        let strategy = match p1 {
            "minimal" => NamespaceStrategy::Minimal,
            "all" => NamespaceStrategy::All,
            "none" => NamespaceStrategy::None,
            _ => return Err(KunaError::parse("Must specify a valid strategy")),
        };
        glb.set_namespace_strategy(strategy);
        Ok("Namespace strategy set".to_string())
    }
}

/// Split-datatype configuration bits (C++ `OptionSplitDatatypes` enum,
/// options.hh:336).
pub mod split_datatype {
    use kuna_base::types::uint4;
    /// Split combined structure fields (`option_struct` = 1).
    pub const OPTION_STRUCT: uint4 = 1;
    /// Split combined array elements (`option_array` = 2).
    pub const OPTION_ARRAY: uint4 = 2;
    /// Split combined LOAD and STORE operations (`option_pointer` = 4).
    pub const OPTION_POINTER: uint4 = 4;
}

arch_option!(
    /// `splitdatatype`: which data-type assignments are split into multiple ops
    /// (C++ `OptionSplitDatatypes`, options.cc:1043).
    OptionSplitDatatypes, "splitdatatype");
impl OptionSplitDatatypes {
    /// Translate an option string to a configuration bit (C++
    /// `OptionSplitDatatypes::getOptionBit`, options.cc:1026).
    pub fn get_option_bit(val: &str) -> KunaResult<uint4> {
        if val.is_empty() {
            return Ok(0);
        }
        match val {
            "struct" => Ok(split_datatype::OPTION_STRUCT),
            "array" => Ok(split_datatype::OPTION_ARRAY),
            "pointer" => Ok(split_datatype::OPTION_POINTER),
            _ => Err(KunaError::lowlevel(format!(
                "Unknown data-type split option: {val}"
            ))),
        }
    }
}
impl ArchOption for OptionSplitDatatypes {
    fn get_name(&self) -> &str {
        "splitdatatype"
    }
    fn apply(
        &self,
        glb: &mut dyn ArchOptionContext,
        p1: &str,
        p2: &str,
        p3: &str,
    ) -> KunaResult<String> {
        let old_config = glb.split_datatype_config();
        let mut config = Self::get_option_bit(p1)?;
        config |= Self::get_option_bit(p2)?;
        config |= Self::get_option_bit(p3)?;
        glb.set_split_datatype_config(config);

        let cur = glb.current_action_name();
        if (config & (split_datatype::OPTION_STRUCT | split_datatype::OPTION_ARRAY)) == 0 {
            glb.toggle_action(&cur, "splitcopy", false);
            glb.toggle_action(&cur, "splitpointer", false);
        } else {
            let pointers = (config & split_datatype::OPTION_POINTER) != 0;
            glb.toggle_action(&cur, "splitcopy", true);
            glb.toggle_action(&cur, "splitpointer", pointers);
        }

        if old_config == config {
            return Ok("Split data-type configuration unchanged".to_string());
        }
        Ok("Split data-type configuration set".to_string())
    }
}

arch_option!(
    /// `nanignore`: which NaN operations are replaced by false
    /// (C++ `OptionNanIgnore`, options.cc:1074).
    OptionNanIgnore, "nanignore");
impl ArchOption for OptionNanIgnore {
    fn get_name(&self) -> &str {
        "nanignore"
    }
    fn apply(
        &self,
        glb: &mut dyn ArchOptionContext,
        p1: &str,
        _p2: &str,
        _p3: &str,
    ) -> KunaResult<String> {
        let old_ignore_all = glb.nan_ignore_all();
        let old_ignore_compare = glb.nan_ignore_compare();
        match p1 {
            "none" => {
                glb.set_nan_ignore_all(false);
                glb.set_nan_ignore_compare(false);
            }
            "compare" => {
                glb.set_nan_ignore_all(false);
                glb.set_nan_ignore_compare(true);
            }
            "all" => {
                glb.set_nan_ignore_all(true);
                glb.set_nan_ignore_compare(true);
            }
            _ => return Err(KunaError::lowlevel(format!("Unknown nanignore option: {p1}"))),
        }
        if !glb.nan_ignore_all() && !glb.nan_ignore_compare() {
            glb.disable_rule("ignorenan");
        } else {
            glb.enable_rule("ignorenan");
        }
        if old_ignore_all == glb.nan_ignore_all()
            && old_ignore_compare == glb.nan_ignore_compare()
        {
            return Ok("NaN ignore configuration unchanged".to_string());
        }
        Ok(format!("Nan ignore configuration set to: {p1}"))
    }
}

/// `ProtoModel::extrapop_unknown` (fspec.hh) — the sentinel `extrapop` value
/// triggering recovery analysis.  // STUB(W6): fspec owns the canonical const.
pub const PROTOMODEL_EXTRAPOP_UNKNOWN: int4 = 0x8000;

// ---------------------------------------------------------------------------
// OptionDatabase.
// ---------------------------------------------------------------------------

/// A dispatcher for `ArchOption` commands (C++ `OptionDatabase`,
/// options.hh:106).
///
/// An option command changes one configuration knob on the
/// [`Architecture`](crate::architecture).  This struct dispatches the command to
/// the right [`ArchOption`], keyed by the option's element id (the C++
/// `optionmap` is `map<uint4,ArchOption*>`).  Commands arrive through
/// [`set`](OptionDatabase::set) or as an `<optionslist>` element parsed by
/// [`decode`](OptionDatabase::decode).
///
/// The C++ owns its `Architecture *glb`; the Rust port keeps the database
/// decoupled and passes the [`ArchOptionContext`] to `set`/`decode`, so the
/// W4-owned `Architecture` can implement the boundary without this module reaching
/// into it.
#[derive(Default)]
pub struct OptionDatabase {
    /// A map from option element id to its registered `ArchOption` (C++
    /// `optionmap`).  A `BTreeMap` (ADR 0002) for deterministic iteration.
    optionmap: BTreeMap<uint4, Box<dyn ArchOption>>,
}

impl OptionDatabase {
    /// Construct an OptionDatabase with every *upstream* `ArchOption` registered
    /// (C++ `OptionDatabase::OptionDatabase`, options.cc:115; in the same
    /// registration order).
    ///
    /// The kuna options (`KUNA_OPTION_NAMES`) are registered separately by the
    /// `w4-kuna-p0-pack` wiring, which owns their `ArchOption` impls and element
    /// ids; call [`register_option`](OptionDatabase::register_option) for each.
    pub fn new() -> Self {
        let mut db = OptionDatabase { optionmap: BTreeMap::new() };
        // Registration order mirrors options.cc:119-155 exactly.
        db.register_option(ELEM_EXTRAPOP, Box::new(OptionExtraPop));
        db.register_option(ELEM_READONLY, Box::new(OptionReadOnly));
        db.register_option(ELEM_IGNOREUNIMPLEMENTED, Box::new(OptionIgnoreUnimplemented));
        db.register_option(ELEM_ERRORUNIMPLEMENTED, Box::new(OptionErrorUnimplemented));
        db.register_option(ELEM_ERRORREINTERPRETED, Box::new(OptionErrorReinterpreted));
        db.register_option(
            ELEM_ERRORTOOMANYINSTRUCTIONS,
            Box::new(OptionErrorTooManyInstructions),
        );
        db.register_option(ELEM_DEFAULTPROTOTYPE, Box::new(OptionDefaultPrototype));
        db.register_option(ELEM_INFERCONSTPTR, Box::new(OptionInferConstPtr));
        db.register_option(ELEM_ANALYZEFORLOOPS, Box::new(OptionForLoops));
        db.register_option(ELEM_INLINE, Box::new(OptionInline));
        db.register_option(ELEM_NORETURN, Box::new(OptionNoReturn));
        db.register_option(ELEM_PROTOEVAL, Box::new(OptionProtoEval));
        db.register_option(ELEM_WARNING, Box::new(OptionWarning));
        db.register_option(ELEM_NULLPRINTING, Box::new(OptionNullPrinting));
        db.register_option(ELEM_INPLACEOPS, Box::new(OptionInPlaceOps));
        db.register_option(ELEM_CONVENTIONPRINTING, Box::new(OptionConventionPrinting));
        db.register_option(ELEM_NOCASTPRINTING, Box::new(OptionNoCastPrinting));
        db.register_option(ELEM_MAXLINEWIDTH, Box::new(OptionMaxLineWidth));
        db.register_option(ELEM_INDENTINCREMENT, Box::new(OptionIndentIncrement));
        db.register_option(ELEM_COMMENTINDENT, Box::new(OptionCommentIndent));
        db.register_option(ELEM_COMMENTSTYLE, Box::new(OptionCommentStyle));
        db.register_option(ELEM_COMMENTHEADER, Box::new(OptionCommentHeader));
        db.register_option(ELEM_COMMENTINSTRUCTION, Box::new(OptionCommentInstruction));
        db.register_option(ELEM_INTEGERFORMAT, Box::new(OptionIntegerFormat));
        db.register_option(ELEM_BRACEFORMAT, Box::new(OptionBraceFormat));
        db.register_option(ELEM_CURRENTACTION, Box::new(OptionCurrentAction));
        db.register_option(ELEM_ALLOWCONTEXTSET, Box::new(OptionAllowContextSet));
        db.register_option(ELEM_SETACTION, Box::new(OptionSetAction));
        db.register_option(ELEM_SETLANGUAGE, Box::new(OptionSetLanguage));
        db.register_option(ELEM_JUMPTABLEMAX, Box::new(OptionJumpTableMax));
        db.register_option(ELEM_JUMPLOAD, Box::new(OptionJumpLoad));
        db.register_option(ELEM_TOGGLERULE, Box::new(OptionToggleRule));
        db.register_option(ELEM_ALIASBLOCK, Box::new(OptionAliasBlock));
        db.register_option(ELEM_MAXINSTRUCTION, Box::new(OptionMaxInstruction));
        db.register_option(ELEM_NAMESPACESTRATEGY, Box::new(OptionNamespaceStrategy));
        db.register_option(ELEM_SPLITDATATYPE, Box::new(OptionSplitDatatypes));
        db.register_option(ELEM_NANIGNORE, Box::new(OptionNanIgnore));
        db
    }

    /// Register a single `ArchOption` instance keyed by its option element id
    /// (C++ `OptionDatabase::registerOption`, options.cc:106).
    ///
    /// In C++ the key is `ElementId::find(option->getName(),0)`; the Rust API
    /// takes the `ElementId` directly so a caller (the kuna `kuna_*.rs` modules,
    /// or the upstream ctor above) supplies the matching id with no global
    /// registry lookup.  A duplicate id overwrites, matching `optionmap[id] =`.
    pub fn register_option(&mut self, elem: ElementId, option: Box<dyn ArchOption>) {
        self.optionmap.insert(elem.get_id(), option);
    }

    /// Perform an option command directly, given its id and optional parameters
    /// (C++ `OptionDatabase::set`, options.cc:194).
    ///
    /// An unknown id throws `ParseError("Unknown option")`.
    pub fn set(
        &self,
        glb: &mut dyn ArchOptionContext,
        name_id: uint4,
        p1: &str,
        p2: &str,
        p3: &str,
    ) -> KunaResult<String> {
        match self.optionmap.get(&name_id) {
            None => Err(KunaError::parse("Unknown option")),
            Some(opt) => opt.apply(glb, p1, p2, p3),
        }
    }

    /// Parse and execute a single option element (C++
    /// `OptionDatabase::decodeOne`, options.cc:207).
    ///
    /// Reads `<name>` with optional `<param1>`/`<param2>`/`<param3>` children
    /// (or bare text content as param1) and dispatches to [`set`](Self::set).
    pub fn decode_one(
        &self,
        decoder: &mut dyn Decoder,
        glb: &mut dyn ArchOptionContext,
    ) -> KunaResult<()> {
        let mut p1 = String::new();
        let mut p2 = String::new();
        let mut p3 = String::new();

        let elem_id = decoder.open_element()?;
        let mut sub_id = decoder.open_element()?;
        if sub_id == ELEM_PARAM1 {
            p1 = read_content_string(decoder)?;
            decoder.close_element(sub_id)?;
            sub_id = decoder.open_element()?;
            if sub_id == ELEM_PARAM2 {
                p2 = read_content_string(decoder)?;
                decoder.close_element(sub_id)?;
                sub_id = decoder.open_element()?;
                if sub_id == ELEM_PARAM3 {
                    p3 = read_content_string(decoder)?;
                    decoder.close_element(sub_id)?;
                }
            }
        } else if sub_id == 0 {
            // No children: the content is param1.
            p1 = read_content_string(decoder)?;
        }
        decoder.close_element(elem_id)?;
        self.set(glb, elem_id, &p1, &p2, &p3)?;
        Ok(())
    }

    /// Parse an `<optionslist>` element, executing each child as an option
    /// command (C++ `OptionDatabase::decode`, options.cc:236).
    pub fn decode(
        &self,
        decoder: &mut dyn Decoder,
        glb: &mut dyn ArchOptionContext,
    ) -> KunaResult<()> {
        let elem_id = decoder.open_element_id(&ELEM_OPTIONSLIST)?;
        while decoder.peek_element()? != 0 {
            self.decode_one(decoder, glb)?;
        }
        decoder.close_element(elem_id)?;
        Ok(())
    }

    /// (kuna, ghidra-mode) Parse an `<optionslist>` like [`decode`](Self::decode),
    /// but skip any option element that cannot be parsed or applied instead of
    /// failing the list — the documented ghidra-mode divergence
    /// (`docs/ghidra-integration.md` §8): upstream throws on the first unknown
    /// element, `setOptions` answers `f`, and Java fails the whole program open
    /// ("Did not accept decompiler options"), so one option from a newer Java
    /// vocabulary (e.g. 12.2's `baddatacount`, element 290) bricks the decompiler
    /// view.  kuna applies what it knows and reports each skipped element as a
    /// returned warning line (each begins with "Warning", which Java's
    /// `isErrorMessage` treats as non-fatal).
    ///
    /// Structurally hardened: every child of an option element is consumed with
    /// `close_element_skipping`, so an unknown parameter shape leaves the stream
    /// element-aligned and the remaining options still apply.
    pub fn decode_lenient(
        &self,
        decoder: &mut dyn Decoder,
        glb: &mut dyn ArchOptionContext,
    ) -> KunaResult<Vec<String>> {
        let elem_id = decoder.open_element_id(&ELEM_OPTIONSLIST)?;
        let mut warnings = Vec::new();
        while decoder.peek_element()? != 0 {
            let opt_id = decoder.open_element()?;
            if let Err(e) = self.decode_one_lenient(decoder, glb, opt_id) {
                let name = option_element_name(opt_id)
                    .map(|n| format!("<{n}>"))
                    .unwrap_or_else(|| format!("element {opt_id}"));
                warnings.push(format!(
                    "Warning: decompiler option {name} not applied: {}",
                    e.explain()
                ));
            }
        }
        decoder.close_element(elem_id)?;
        Ok(warnings)
    }

    /// One already-opened option element of the lenient list: parse the
    /// `<param1>`/`<param2>`/`<param3>` children (any other child is skipped
    /// whole), close the element, and dispatch to [`set`](Self::set).  The
    /// element is fully consumed on ANY return, error included.
    fn decode_one_lenient(
        &self,
        decoder: &mut dyn Decoder,
        glb: &mut dyn ArchOptionContext,
        opt_id: uint4,
    ) -> KunaResult<String> {
        let mut p1 = String::new();
        let mut p2 = String::new();
        let mut p3 = String::new();
        let mut saw_child = false;
        let mut content_err: Option<KunaError> = None;
        loop {
            let sub_id = decoder.open_element()?;
            if sub_id == 0 {
                break;
            }
            saw_child = true;
            let target = if sub_id == ELEM_PARAM1 {
                Some(&mut p1)
            } else if sub_id == ELEM_PARAM2 {
                Some(&mut p2)
            } else if sub_id == ELEM_PARAM3 {
                Some(&mut p3)
            } else {
                None
            };
            if let Some(slot) = target {
                match read_content_string(decoder) {
                    Ok(v) => *slot = v,
                    Err(e) => content_err = Some(e),
                }
            }
            decoder.close_element_skipping(sub_id)?;
        }
        if !saw_child {
            match read_content_string(decoder) {
                Ok(v) => p1 = v,
                Err(e) => content_err = Some(e),
            }
        }
        decoder.close_element(opt_id)?;
        if let Some(e) = content_err {
            return Err(e);
        }
        self.set(glb, opt_id, &p1, &p2, &p3)
    }
}

/// The upstream name of an option element id, for lenient-decode warnings.
fn option_element_name(id: uint4) -> Option<&'static str> {
    UPSTREAM_OPTION_ELEMENTS
        .iter()
        .find(|e| e.get_id() == id)
        .map(|e| e.get_name())
}

/// `decoder.readString(ATTRIB_CONTENT)` as a (lossily) UTF-8 `String`.
///
/// The C++ `Decoder::readString` returns a `std::string` (raw bytes); option
/// parameters are ASCII keywords, so a lossy conversion is faithful for every
/// value the options accept and keeps the comparison string-typed.
fn read_content_string(decoder: &mut dyn Decoder) -> KunaResult<String> {
    let bytes = decoder.read_string_id(&ATTRIB_CONTENT)?;
    Ok(String::from_utf8_lossy(&bytes).into_owned())
}

/// Register every upstream option element id (`UPSTREAM_OPTION_ELEMENTS` plus
/// the `<param1>`/`<param2>`/`<param3>`/`<optionslist>` framing ids) into an
/// [`IdRegistry`], so a decoder can resolve an `<optionslist>` stream's element
/// names to the dispatch ids.  Mirrors the C++ global `ElementId` registration
/// these options rely on.
pub fn register_option_elements(reg: &mut IdRegistry) {
    for e in UPSTREAM_OPTION_ELEMENTS {
        reg.register_element(e);
    }
    reg.register_element(&ELEM_OPTIONSLIST);
    reg.register_element(&ELEM_PARAM1);
    reg.register_element(&ELEM_PARAM2);
    reg.register_element(&ELEM_PARAM3);
    // The remaining option ids not in the ctor list (kept for completeness so a
    // decode stream naming them resolves to a real id and `set` then reports
    // "Unknown option" rather than failing the decode).
    reg.register_element(&ELEM_STRUCTALIGN);
    reg.register_element(&ELEM_HIDEEXTENSIONS);
    // ELEM_UNKNOWN is the registry default; referenced to document the fallback.
    let _ = ELEM_UNKNOWN;
}

#[cfg(test)]
mod tests;

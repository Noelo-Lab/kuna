//! Engine bootstrap glue for the console front-end (item `w9x-bins-runner`).
//!
//! The C++ `IfcLoadFile` (`consolemain.cc:46`) hands a file path to
//! `ArchitectureCapability::findCapability`, builds the leaf `Architecture`
//! (`XmlArchitecture`), and runs `Architecture::init` — the whole
//! `restoreFromSpec`/`buildTranslator`/`buildTypegrp`/`buildAction` chain.  In
//! the kuna Rust port that chain is assembled by the XML frontend
//! ([`XmlArchitecture`]) through `build_translator`, then
//! [`Architecture::init_post_engine`] (the merged `w9x-arch-engine-glue` item),
//! which the integration-test `decompile_e2e.rs` `bootstrap()` proved end to
//! end.  This module lifts that bootstrap into a reusable shape so the two
//! console paths — the interactive `decomp_dbg` `load file` command and the
//! datatest harness `buildProgram` — drive the **same** real engine assembly.
//!
//! ## What `load file` accepts
//!
//! The C++ console drives a real binary through BFD; the kuna Rust engine's only
//! load-image backend is the XML `<binaryimage>` format the datatests use (the
//! BFD `RawBinaryArchitecture`/`LoadImageBfd` backends are their own port item).
//! So the Rust `load file <path>` accepts a `<binaryimage>` (or
//! `<decompilertest>`-wrapping) XML file — exactly the corpus image format the
//! Python tools (`kuna/decompile.py`) and the datatests feed, which is what the
//! `KUNA_ENGINE=rust` path is wired to drive.
//!
//! ## Symbol resolution (the `readLoaderSymbols`/`queryFunction` seam)
//!
//! The W4 symbol-table population from the loader (`Architecture::readLoaderSymbols`
//! → `Scope::addFunction`) and `Scope::queryFunction` are later port items.  The
//! `<binaryimage>` itself carries `<symbol>` records (name + address), and the
//! opened [`LoadImageXml`] exposes them via `open_symbols`/`get_next_symbol`.
//! This module reads those records once (at `load file`) into a name→address
//! table on the [`ConsoleProgram`], so `load function <name>` resolves a
//! function entry the faithful way (the binaryimage's own symbol records, which
//! is precisely what `readLoaderSymbols` reads).

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::marshal::IdRegistry;
use kuna_base::space::{AddrSpace, AddrSpaceManager};
use kuna_base::types::int4;
use kuna_base::xml::{DocumentStorage, Element};

use kuna_decomp::architecture::Architecture;
use kuna_decomp::options::register_option_elements;
use kuna_decomp::sleigh_arch::{register_sleigh_arch_ids, LanguageDatabase, SleighArchitecture};
use kuna_decomp::xml_arch::XmlArchitectureCapability;

use kuna_sleigh::loadimage::{LoadImage, LoadImageFunc};
use kuna_analysis::loadimage_object::ObjectLoadImage;
use kuna_sleigh::loadimage_xml::LoadImageXml;
use kuna_sleigh::loadimage_xml::register_loadimage_xml_ids;
use kuna_sleigh::translate::register_translate_ids;

/// One function symbol discovered in the `<binaryimage>` (name → entry address).
#[derive(Debug, Clone)]
struct ProgramSymbol {
    name: String,
    addr: Address,
}

/// The console's loaded program: the engine assembly (C++ `dcp->conf`, an
/// `XmlArchitecture : Architecture`) plus the console-owned marshaling registry
/// and option database the `option` command needs.
///
/// In C++ `dcp->conf` IS the leaf `Architecture` (the `XmlArchitecture`
/// subobject); the Rust leaf [`XmlArchitecture`] owns the `Architecture` base,
/// reachable via [`Self::arch_mut`].  The `IdRegistry`/`OptionDatabase` are the
/// process globals the C++ `ElementId::find` + `dcp->conf->options->set` read; we
/// keep them on the program so the console can resolve an option name to its
/// element id and dispatch it against the real architecture.
pub struct ConsoleProgram {
    /// The engine assembly (owns the `Architecture` god object).  Both the XML
    /// `<binaryimage>` frontend and the real-ELF frontend slice their leaf
    /// architecture back to this `SleighArchitecture` once the loader is opened
    /// and handed to the engine, so the console program is loader-agnostic.
    arch: SleighArchitecture,
    /// The marshaling id registry (C++ `ElementId` global table) for option-name
    /// resolution.
    registry: IdRegistry,
    /// The binaryimage's function symbols (name → entry address), read once at
    /// load (the `readLoaderSymbols` seam).
    symbols: Vec<ProgramSymbol>,
    /// A human-readable description of the loaded program (C++
    /// `conf->getDescription()`).
    description: String,
    /// (kuna) Per-pass analysis facts stashed at load (real-ELF path only),
    /// keyed by `AnalysisPass::id`, awaiting the gated commit at `read symbols`.
    ///
    /// The commit is **deferred** out of `bootstrap_from_elf` so it runs AFTER the
    /// per-pass `--option <id> on|off` flags are applied (the CLI emits the
    /// `option` lines before `read symbols`). `IfcReadSymbols` consults each pass's
    /// enable flag on the `Architecture` and commits only the enabled passes'
    /// facts (see [`commit_analysis_passes`]). Empty on the XML datatest path (no
    /// analysis tier runs), so the gated commit is a faithful no-op there. The
    /// stash is drained on commit so a second `read symbols` does not re-commit.
    pending_analysis: Vec<(&'static str, kuna_analysis::pass::AnalysisOutput)>,
    /// (kuna) The engine default code space captured at load, used to build the
    /// `Address`es when the stashed analysis facts are committed at `read symbols`.
    analysis_code_space: Option<Rc<AddrSpace>>,
    /// (kuna) DWARF stack-LOCAL recommendations parked at the analysis commit
    /// (`commit_analysis_output`, DWARF subtask 3), keyed by owning-function entry
    /// VMA. Each is `(name, type, stack_offset)`; the entry's `Vec` is the function's
    /// locals. `IfcDecompile` looks the function up by its entry address and appends
    /// these (built into stack-space `map addr` symbol specs by [`Self::dwarf_locals_for`])
    /// to the `mapped_symbols` it threads through `decompile_func_full_with_override_dyn`,
    /// so the rebuilt `Funcdata`'s `ScopeLocal` is re-seeded with each as a
    /// typelock|namelock stack symbol — the same path the console `map addr` directive
    /// uses. Real-ELF DWARF path only (empty on the XML datatest path).
    dwarf_locals: Vec<(u64, String, Rc<kuna_decomp::dtype::Datatype>, i64)>,
}

impl ConsoleProgram {
    /// Borrow the `Architecture` god object (C++ `dcp->conf`, viewed as the base).
    pub fn arch(&self) -> &Architecture {
        self.arch
            .base()
            .expect("ConsoleProgram: Architecture base present after bootstrap")
    }

    /// Mutably borrow the `Architecture` god object.
    pub fn arch_mut(&mut self) -> &mut Architecture {
        self.arch
            .base_mut()
            .expect("ConsoleProgram: Architecture base present after bootstrap")
    }

    /// The marshaling id registry (for `ElementId::find`).
    pub fn registry(&self) -> &IdRegistry {
        &self.registry
    }

    /// The number of function symbols read from the binaryimage (the
    /// `readLoaderSymbols` seam yield).
    pub fn num_symbols(&self) -> usize {
        self.symbols.len()
    }

    /// C++ `conf->getDescription()` — the load-success description line.
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Resolve a function entry address by symbol name (the `queryFunction`
    /// seam): scan the binaryimage symbols read at load.  `None` if no symbol of
    /// that name exists.
    pub fn lookup_symbol(&self, name: &str) -> Option<Address> {
        self.symbols.iter().find(|s| s.name == name).map(|s| s.addr.clone())
    }

    /// Read the binaryimage's loader symbols into the symbol table as
    /// FunctionSymbols (C++ `Architecture::readLoaderSymbols`, `architecture.cc:347`,
    /// called by `testfunction.cc:160` / `consolemain.cc:104` after load).
    ///
    /// Each loader symbol (name → entry address, already in `self.symbols`) becomes
    /// a `FunctionSymbol` in its (namespace-resolved) scope, so a CALL to that
    /// entry address resolves to the callee's name at flow-analysis time
    /// (`FlowInfo::queryCall`).  Idempotent: a symbol whose function is already in
    /// the table is skipped (the C++ `addFunction` no-ops on an existing match via
    /// `queryFunction`).
    pub fn read_loader_symbols(&mut self) -> KunaResult<()> {
        let type_code = self.arch().types().get_type_code()?;
        let min_size = self.arch().min_funcsymbol_size;
        let num_spaces = self.arch().manage().num_spaces();
        // Clone the (name, addr) pairs so the borrow of `self.arch_mut()` below
        // does not overlap `self.symbols`.
        let records: Vec<(String, Address)> =
            self.symbols.iter().map(|s| (s.name.clone(), s.addr.clone())).collect();
        let arch = self.arch_mut();
        for (name, addr) in records {
            let (scope, basename) = arch
                .symboltab
                .find_create_scope_from_symbol_name(&name, "::", None, num_spaces)?;
            // C++ queryFunction: skip if a function already maps this address.
            if arch.symboltab.find_function(scope, &addr).is_some() {
                continue;
            }
            arch.symboltab.add_function(scope, &addr, &basename, min_size, type_code.clone())?;
        }
        Ok(())
    }

    /// Register a console-created function symbol (the `map function` seam): make
    /// `name`->`addr` resolvable by `load function <name>`.  C++ `Scope::addFunction`
    /// installs the symbol in the symbol table; the kuna console additionally needs
    /// the name->address entry so the (binaryimage-symbol-backed) `load function`
    /// path can find a function the user mapped by hand.  Replaces any prior entry
    /// of the same name.
    pub fn register_symbol(&mut self, name: &str, addr: Address) {
        self.symbols.retain(|s| s.name != name);
        self.symbols.push(ProgramSymbol { name: name.to_string(), addr });
    }

    /// (kuna) Build the `map addr`-shaped stack-symbol specs for the DWARF stack
    /// LOCALS parked on the function whose entry VMA is `func_addr` (DWARF subtask 3).
    ///
    /// Returns `(name, type, stack_Address, flags)` tuples in the exact shape
    /// `Funcdata::seed_mapped_symbols` consumes — `IfcDecompile` appends them to the
    /// `mapped_symbols` it threads into the decompile drive, so each parked DWARF
    /// local is re-seeded into the rebuilt `Funcdata`'s `ScopeLocal` as a
    /// `typelock|namelock` stack symbol (the typelock keeps the DWARF type through
    /// propagation; the namelock keeps the DWARF name). The stack `Address` is built
    /// here against the live stack space (`getStackSpace`), wrapping the signed
    /// `stack_offset` to the space's unsigned offset (the same convention as the
    /// console `map addr s<off>` directive — e.g. copytrim.xml's
    /// `map addr s0xffffffffffffffe4`). Empty when the function has no parked locals
    /// or the architecture has no stack space.
    pub fn dwarf_locals_for(
        &self,
        func_addr: u64,
    ) -> Vec<(String, Rc<kuna_decomp::dtype::Datatype>, Address, kuna_base::types::uint4)> {
        use kuna_decomp::varnode::varnode_flags;
        let Some(stack) = self.arch().manage().get_stack_space() else {
            return Vec::new();
        };
        let flags = varnode_flags::typelock | varnode_flags::namelock;
        self.dwarf_locals
            .iter()
            .filter(|(addr, _, _, _)| *addr == func_addr)
            .map(|(_, name, ty, off)| {
                // Wrap the signed stack offset to the stack space's unsigned address
                // (negative locals live at the high end of the unsigned range, the
                // `map addr s0xffff...` convention).
                let waddr = stack.wrap_offset(*off as u64);
                (
                    name.clone(),
                    Rc::clone(ty),
                    Address::new(Rc::clone(stack), waddr),
                    flags,
                )
            })
            .collect()
    }

    /// (kuna) Commit the stashed per-pass analysis facts, gated by the per-pass
    /// `--option <id> on|off` enable flags — the deferred half of the analysis
    /// seam (conflict #4). Called from `IfcReadSymbols` (`read symbols`), which
    /// runs AFTER the CLI's `option` lines, so a disabled pass's facts are
    /// dropped here rather than committed.
    ///
    /// Drains the stash (so a second `read symbols` does not re-commit), merges
    /// only the **enabled** passes' outputs in pass order, and commits the merged
    /// [`AnalysisOutput`] via [`commit_analysis_output`]. A no-op when nothing is
    /// stashed (the XML datatest path — parity is structurally untouched).
    pub fn commit_pending_analysis(&mut self) -> KunaResult<()> {
        if self.pending_analysis.is_empty() {
            return Ok(());
        }
        let pending = std::mem::take(&mut self.pending_analysis);
        let Some(code_space) = self.analysis_code_space.take() else {
            // No code space captured (should not happen on the ELF path); nothing
            // to commit against.
            return Ok(());
        };
        // Filter by the per-pass enable flags (default-on, set by the user's
        // `--option <id> on|off`), then merge the survivors in pass order.
        let mut merged = kuna_analysis::pass::AnalysisOutput::default();
        for (id, out) in pending {
            if analysis_pass_enabled(self.arch(), id) {
                merged.merge(out);
            }
        }
        commit_analysis_output(self, &code_space, merged)
    }
}

/// (kuna) Read a `kuna_analysis` pass's per-run enable flag off the
/// [`Architecture`] by the pass's `AnalysisPass::id`. An unknown id defaults to
/// enabled (a new pass with no registered gate still runs — fail-open, additive).
fn analysis_pass_enabled(arch: &Architecture, pass_id: &str) -> bool {
    match pass_id {
        "noreturn_known" => arch.analysis_noreturn_known,
        "libproto" => arch.analysis_libproto,
        "strings" => arch.analysis_strings,
        "entry_disc" => arch.analysis_entry_disc,
        "arm_markers" => arch.analysis_arm_markers,
        "mips_gp" => arch.analysis_mips_gp,
        "dwarf" => arch.analysis_dwarf,
        "callfixup" => arch.analysis_callfixup,
        "addrtable" => arch.analysis_addrtable,
        _ => true,
    }
}

/// Build the marshaling [`IdRegistry`] the console bootstrap needs (the same
/// per-module id registration the `decompile_e2e` gate's `build_registry` does:
/// translate + sleigh-arch + loadimage-xml + option element ids).
fn build_registry() -> IdRegistry {
    let mut registry = IdRegistry::with_base_ids();
    register_translate_ids(&mut registry);
    register_sleigh_arch_ids(&mut registry);
    register_loadimage_xml_ids(&mut registry);
    register_option_elements(&mut registry);
    registry
}

/// Locate the SLEIGH specs root (C++ `SleighArchitecture::specpaths`).
///
/// The Python tools pass `-s <specs>` and set `SLEIGHHOME=<specs>`; the bin's
/// arg parser records the spec roots and hands them here.  The first existing
/// root wins (mirroring the C++ `scanForSleighDirectories` over the recorded
/// roots).
fn scan_language_database(spec_roots: &[String], registry: &IdRegistry) -> KunaResult<LanguageDatabase> {
    let mut db = LanguageDatabase::new();
    for root in spec_roots {
        db.scan_for_sleigh_directories(root);
    }
    db.get_descriptions(registry)?;
    Ok(db)
}

/// Find the `<binaryimage>` element inside a parsed document root (which may be a
/// bare `<binaryimage>` or a `<decompilertest>` wrapping one).
fn find_binaryimage(root: &Rc<Element>) -> Option<Rc<Element>> {
    if root.get_name() == "binaryimage" {
        return Some(Rc::clone(root));
    }
    for c in root.get_children() {
        if let Some(found) = find_binaryimage(c) {
            return Some(found);
        }
    }
    None
}

/// Read an attribute as a `String` (lossy ASCII), `None` if absent.
fn attr(el: &Element, name: &str) -> Option<String> {
    el.get_attribute_value(name).ok().map(|b| String::from_utf8_lossy(b).into_owned())
}

/// Run the spec-file resolution, translator build, and `Architecture::init`
/// tail on an already-language-resolved [`SleighArchitecture`] — the chain both
/// the XML and ELF frontends share (C++ `buildSpecFile` → `buildTranslator` →
/// `buildTypegrp`/`buildCoreTypes`/`buildAction`/…).
///
/// The caller must have set `archid` + resolved the language index first; this
/// reads the `.sla`/`.cspec`/`.pspec`, builds the translator (with a [`NullLoad`]
/// placeholder image the caller replaces after open), installs the register
/// lookup, and runs `init_post_engine`.  It deliberately does **not** open or
/// attach the loader — that is the frontend's `postSpecFile` job (the XML path
/// opens the `<binaryimage>`; the ELF path attaches the default code space).
fn build_engine_and_init(sleigh: &mut SleighArchitecture, db: &LanguageDatabase) -> KunaResult<()> {
    // buildSpecFile -> the resolved .sla; buildTranslator (decode the .sla).
    let specs = sleigh.build_spec_file(db)?;
    let resolved_sla = specs
        .slafile
        .ok_or_else(|| KunaError::lowlevel("build_spec_file resolved no .sla"))?;
    let sla = std::fs::read(&resolved_sla)
        .map_err(|e| KunaError::lowlevel(format!("read sla {resolved_sla}: {e}")))?;

    // The loader is handed to the translator as a dummy first; the real opened
    // image replaces it after init (mirrors corpus_bootstrap.rs / the e2e gate).
    sleigh.build_translator(Box::new(NullLoad), &sla)?;

    // Apply the active language's `.ldefs` `<truncate_space>` records (C++
    // `Architecture::restoreFromSpec` -> `SleighArchitecture::modifySpaces`,
    // architecture.cc:631).  This shrinks an address space's addr size before the
    // type factory reads `getDefaultDataSpace()->getAddrSize()` for the default
    // pointer width — e.g. PowerPC:BE:32:e500 truncates `ram` to 4 so a `void *`
    // is a 32-bit pointer even though the GPRs (and the space) are modeled 64-bit.
    {
        let langindex = sleigh.language_index();
        let arch = sleigh
            .base_mut()
            .ok_or_else(|| KunaError::lowlevel("no Architecture base after build_translator"))?;
        db.modify_spaces(langindex, arch.manage())?;
    }

    // Hand the resolved compiler-spec (`.cspec`) XML to the architecture so
    // `build_default_proto` can decode the real `<default_proto>` input/output
    // parameter lists (the C++ `parseCompilerConfig` reads the cspec here).
    // A read failure is non-fatal: the architecture falls back to the name-only
    // default model (proto recovery simply won't fire).
    if !specs.compilerfile.is_empty() {
        if let Ok(cspec) = std::fs::read(&specs.compilerfile) {
            sleigh
                .base_mut()
                .ok_or_else(|| KunaError::lowlevel("no Architecture base after build_translator"))?
                .set_cspec_xml(cspec);
        }
    }

    // Hand the resolved processor-spec (`.pspec`) XML to the architecture so
    // `parse_processor_config` (run inside `init_post_engine`) can apply the
    // `<context_data>` `<context_set>` paints (the C++ `parseProcessorConfig`
    // reads the pspec here).  This is what selects the SLEIGH disassembly mode:
    // without it x86-64 lifts as 16-bit real mode.  A read failure is non-fatal
    // (the engine keeps the zero-default context).
    if !specs.processorfile.is_empty() {
        if let Ok(pspec) = std::fs::read(&specs.processorfile) {
            sleigh
                .base_mut()
                .ok_or_else(|| KunaError::lowlevel("no Architecture base after build_translator"))?
                .set_pspec_xml(pspec);
        }
    }

    // Install the register-name lookup on the engine's manager (the C++
    // `AddrSpace::trans` back-pointer) while the engine is still the sole owner
    // of the manager — before `init_post_engine`'s `parse_processor_config`
    // resolves the pspec `<tracked_set>` register names (e.g. `DF`).
    sleigh
        .base_mut()
        .ok_or_else(|| KunaError::lowlevel("no Architecture base after build_translator"))?
        .translate_mut()
        .install_register_lookup()?;

    // The tail of Architecture::init (buildTypegrp/buildCoreTypes/buildAction/…).
    sleigh
        .base_mut()
        .ok_or_else(|| KunaError::lowlevel("no Architecture base after build_translator"))?
        .init_post_engine()?;
    Ok(())
}

/// Bootstrap a [`ConsoleProgram`] from a `<binaryimage>` element + arch id,
/// against the SLEIGH specs at `spec_roots` (C++ `IfcLoadFile`'s
/// `buildArchitecture` + `conf->init(store)`).
///
/// This is the faithful console body of C++ `IfcLoadFile::execute` reduced to the
/// kuna XML engine backend: build the loader from the `<binaryimage>`, resolve
/// the language, build the spec file, build the translator, run
/// `init_post_engine` (the tail of `Architecture::init`), open the image, hand it
/// to the engine, then read the loader symbols.  Errors carry the C++ failure
/// message so the console's `Could not create architecture` path is faithful.
pub fn bootstrap_program(
    binaryimage: Rc<Element>,
    arch_id: &str,
    spec_roots: &[String],
) -> KunaResult<ConsoleProgram> {
    let registry = build_registry();

    // capa->buildArchitecture(filename,target,...)
    let capability = XmlArchitectureCapability::new();
    let mut arch = capability.build_architecture("loadfile", "");

    // XmlArchitecture::buildLoader (find the <binaryimage>, wrap in LoadImageXml).
    arch.build_loader(Rc::clone(&binaryimage))?;

    // collectSpecFiles + resolveArchitecture (language-id normalization/index).
    let db = scan_language_database(spec_roots, &registry)?;
    arch.sleigh_mut().set_archid(arch_id);
    arch.sleigh_mut().resolve_architecture(&db, arch_id)?;
    if arch.sleigh().language_index() < 0 {
        return Err(KunaError::lowlevel(format!(
            "No sleigh specification for architecture {arch_id}"
        )));
    }

    // buildSpecFile -> buildTranslator -> the Architecture::init tail (shared
    // by both the XML and ELF frontends).
    build_engine_and_init(arch.sleigh_mut(), &db)?;

    // postSpecFile: open the corpus image against the engine spaces.
    let manager_ptr: *const AddrSpaceManager = arch.sleigh().base().unwrap().manage();
    // SAFETY: the manager lives inside `arch` and outlives the open() call; the
    // borrow is released before any &mut use of `arch` (same shape as the e2e
    // gate / corpus_bootstrap.rs).
    arch.open_image(unsafe { &*manager_ptr }, &registry)?;

    // Read the loader symbols (the readLoaderSymbols seam) from the opened image
    // BEFORE handing it to the engine: the LoadImageXml exposes name+address.
    let symbols = read_loader_symbols(arch.loader());

    // C++ `Architecture::fillinReadOnlyFromLoader` (architecture.cc:1375), part of
    // the `Architecture::init` chain: query the load image for its read-only
    // address ranges and OR `Varnode::readonly` over them in the symbol table's
    // property map.  `setVarnodeProperties`/`queryProperties` then paints the
    // `readonly` flag on varnodes reading those ranges, which `ActionVarnodeProps`
    // folds into constants when `option readonly` is on (the float-cluster's
    // IEEE-754 literals live in read-only RAM).  Collected here, while the opened
    // `LoadImageXml` is still in hand, then applied to the symboltab below.
    let readonly_ranges: Vec<(kuna_base::address::Address, kuna_base::address::Address)> =
        if let Some(loader) = arch.loader() {
            use kuna_base::address::RangeList;
            use kuna_sleigh::loadimage::LoadImage;
            let manage_ro: *const AddrSpaceManager = arch.sleigh().base().unwrap().manage();
            let mut rangelist = RangeList::new();
            loader.get_readonly(&mut rangelist);
            // SAFETY: same outlives-the-call shape as the open() borrow above; the
            // manager lives inside `arch` and is only read here.
            let manage_ref = unsafe { &*manage_ro };
            rangelist
                .iter()
                .map(|r| (r.get_first_addr(), r.get_last_addr_open(manage_ref)))
                .collect()
        } else {
            Vec::new()
        };

    // Hand the opened loader to the engine (the C++ `loader` back-pointer the
    // decode reads on load_fill).
    let img = arch
        .take_loader()
        .ok_or_else(|| KunaError::lowlevel("loader vanished after open"))?;
    arch.sleigh_mut().base_mut().unwrap().set_loader(Box::new(img));

    // Apply the collected read-only ranges to the symbol table's property map
    // (C++ `symboltab->setPropertyRange(Varnode::readonly, *iter)`).
    if let Some(base) = arch.sleigh_mut().base_mut() {
        for (first, last_open) in &readonly_ranges {
            base.symboltab
                .set_property_range(kuna_decomp::varnode::varnode_flags::readonly, first, last_open);
        }
    }

    let description = arch.sleigh().base().unwrap().get_description().to_string();

    // Slice the XML leaf back to its `SleighArchitecture` (the XML-specific
    // loader/adjustvma machinery is spent; the engine owns the opened image).
    // The XML path runs no analysis tier, so no facts are stashed and the gated
    // commit at `read symbols` is a no-op (parity is structurally untouched).
    let mut prog = ConsoleProgram {
        arch: arch.into_sleigh(),
        registry,
        symbols,
        description,
        pending_analysis: Vec::new(),
        analysis_code_space: None,
        dwarf_locals: Vec::new(),
    };
    // C++ `conf->readLoaderSymbols("::")` (testfunction.cc:160 / consolemain.cc:104):
    // install the binaryimage symbols as FunctionSymbols so a CALL to one resolves
    // to its callee name at flow-analysis time.
    prog.read_loader_symbols()?;
    Ok(prog)
}

/// Bootstrap a [`ConsoleProgram`] from a **real ELF** binary on disk (the kuna
/// analog of the C++ console's BFD path: `LoadImageBfd` + `RawBinaryArchitecture`/
/// the resolved arch).
///
/// Mirrors `bootstrap_program` but with the ELF [`ObjectLoadImage`] in place of
/// the XML loader: open the ELF (parse machine/segments/symbols), take the
/// SLEIGH language id straight off the loader's `getArchType()` (the
/// `resolveArchitecture` loader branch — C++ `loader->getArchType()`), build the
/// engine, attach the default code space to the loader (the C++
/// `RawBinaryArchitecture::postSpecFile` / `LoadImageBfd::attachToSpace` tail),
/// read the ELF function symbols, then hand the loader to the engine.
///
/// `target` is an optional explicit language id (the `load file <target> <path>`
/// first token, C++ BFD target): when non-empty it overrides the ELF-derived id
/// (so an unmapped machine can still be driven), exactly as the C++
/// `getTarget()` path takes precedence over the loader's arch type.
pub fn bootstrap_from_elf(
    path: &str,
    target: &str,
    spec_roots: &[String],
) -> KunaResult<ConsoleProgram> {
    let registry = build_registry();

    // Read the image bytes once: reused for the loader AND the analysis-pass
    // object::File view (the analyzers need a parsed object the loader drops).
    let bytes = std::fs::read(path)
        .map_err(|e| KunaError::lowlevel(format!("Unable to open image file: {path}: {e}")))?;
    // LoadImageBfd(filename) + open(): parse the ELF (machine, segments, symbols).
    let mut loader = ObjectLoadImage::from_bytes(path, &bytes)?;

    // resolveArchitecture: the arch id is the loader's getArchType() (the ELF
    // machine → SLEIGH language id), unless an explicit target overrides it.
    let arch_type = String::from_utf8_lossy(&loader.get_arch_type()).into_owned();
    let mut sleigh = SleighArchitecture::new(path, target);
    let db = scan_language_database(spec_roots, &registry)?;
    // SleighArchitecture::resolveArchitecture: if target is set it wins (archid
    // stays empty here so the base resolve uses target||arch_type).
    sleigh.resolve_architecture(&db, &arch_type)?;
    if sleigh.language_index() < 0 {
        return Err(KunaError::lowlevel(format!(
            "No sleigh specification for architecture {arch_type}"
        )));
    }

    // buildSpecFile -> buildTranslator -> the Architecture::init tail (shared).
    build_engine_and_init(&mut sleigh, &db)?;

    // postSpecFile: attach the engine's default code space to the loader so its
    // loadFill/getNextSymbol build Addresses in the right space (C++
    // `RawBinaryArchitecture::postSpecFile`'s `attachToSpace(getDefaultCodeSpace())`).
    let code_space = Rc::clone(
        sleigh
            .base()
            .unwrap()
            .manage()
            .get_default_code_space()
            .ok_or_else(|| KunaError::lowlevel("no default code space after init"))?,
    );
    loader.attach_to_space(Rc::clone(&code_space));

    // Architecture::fillinReadOnlyFromLoader on the ELF path (the analog of the
    // XML path's collect-before-handoff above): gather the loader's read-only
    // ranges while the image is still in hand. Load-bearing for string rendering
    // (the printer's push_ptr_char_constant_ir gates a string literal on
    // is_read_only at the constant's address).
    let readonly_ranges: Vec<(Address, Address)> = {
        use kuna_base::address::RangeList;
        use kuna_sleigh::loadimage::LoadImage;
        let manage_ptr: *const AddrSpaceManager = sleigh.base().unwrap().manage();
        let mut rangelist = RangeList::new();
        loader.get_readonly(&mut rangelist);
        // SAFETY: same outlives-the-call shape as the XML path's open() borrow;
        // the manager lives inside `sleigh` and is only read here.
        let manage_ref = unsafe { &*manage_ptr };
        rangelist
            .iter()
            .map(|r| (r.get_first_addr(), r.get_last_addr_open(manage_ref)))
            .collect()
    };

    // Run the program-prep analysis passes (the kuna analyzer tier) over the
    // parsed object, keeping each pass's facts keyed by id. Read-only; the facts
    // are STASHED here and committed (gated) at `read symbols` — NOT eagerly —
    // so the per-pass `--option <id> on|off` flags (emitted by the CLI before
    // `read symbols`) are in effect when the commit runs (the deferred fix for
    // conflict #4, analysis-port-log.md). Bound to the real-ELF path ONLY — the
    // XML <binaryimage> bootstrap never runs these, so the datatest parity oracle
    // is structurally untouched.
    let pending_analysis =
        kuna_analysis::passes::run_default_analyses_per_pass(&bytes, &loader, sleigh.base().unwrap());

    // readLoaderSymbols (the ELF FUNC symbols) BEFORE handing the loader off.
    let symbols = read_loader_symbols_generic(&loader);

    // Hand the loader to the engine (the C++ `loader` back-pointer the decode
    // reads on load_fill).
    sleigh.base_mut().unwrap().set_loader(Box::new(loader));

    let description = sleigh.base().unwrap().get_description().to_string();

    let mut prog = ConsoleProgram {
        arch: sleigh,
        registry,
        symbols,
        description,
        // Stash the per-pass analysis facts + the code space for the gated commit
        // at `read symbols` (IfcReadSymbols -> commit_analysis_passes).
        pending_analysis,
        analysis_code_space: Some(Rc::clone(&code_space)),
        dwarf_locals: Vec::new(),
    };
    // conf->readLoaderSymbols("::"): install the ELF symbols as FunctionSymbols.
    // The deferred analysis commit at `read symbols` REQUIRES this to have run
    // first (no-return/callfixup address+name resolution finds the funcsyms).
    prog.read_loader_symbols()?;

    // Apply the collected read-only ranges to the symbol table property map
    // (C++ symboltab->setPropertyRange(Varnode::readonly, *iter)), now that prog
    // owns the architecture. This is NOT a gated analysis pass (it is loader
    // markup), so it stays eager here, before any analysis commit.
    for (first, last_open) in &readonly_ranges {
        prog.arch_mut().symboltab.set_property_range(
            kuna_decomp::varnode::varnode_flags::readonly,
            first,
            last_open,
        );
    }

    // NB: the analysis-pass facts are committed later, gated, in
    // `commit_analysis_passes` (called from `IfcReadSymbols`), after the per-pass
    // `--option` flags are applied. See `ConsoleProgram::pending_analysis`.
    Ok(prog)
}

/// Commit a merged [`kuna_analysis::pass::AnalysisOutput`] into the engine's
/// symbol/type tables — the kuna-console side of the kuna-analysis pass seam.
///
/// Each fact is **additive** (only adds knowledge) and **idempotent** against the
/// funcsym stream `read_loader_symbols` already committed (the `find_function`
/// overlap check no-ops a duplicate). Bound to the real-ELF path; the XML
/// `<binaryimage>` path never produces an `AnalysisOutput`, so this never runs
/// there. See `docs/missing-analyses.md` for the per-fact-kind API rationale.
fn commit_analysis_output(
    prog: &mut ConsoleProgram,
    code_space: &Rc<AddrSpace>,
    out: kuna_analysis::pass::AnalysisOutput,
) -> KunaResult<()> {
    use kuna_analysis::pass::SymKind;

    let num_spaces = prog.arch().manage().num_spaces();

    // 1. Extra symbols a pass discovered. Function symbols install like the
    //    funcsym stream (idempotent); Data symbols (typed string/data objects)
    //    land via add_symbol_mapped. (No pass emits these yet in this increment;
    //    the commit path is wired so the string/entry passes plug in cleanly.)
    for s in &out.symbols {
        let addr = Address::new(Rc::clone(code_space), s.addr);
        match s.kind {
            SymKind::Function => {
                let type_code = prog.arch().types().get_type_code()?;
                let min_size = prog.arch().min_funcsymbol_size;
                {
                    let arch = prog.arch_mut();
                    let (scope, base) = arch
                        .symboltab
                        .find_create_scope_from_symbol_name(&s.name, "::", None, num_spaces)?;
                    if arch.symboltab.find_function(scope, &addr).is_none() {
                        arch.symboltab.add_function(scope, &addr, &base, min_size, type_code)?;
                    }
                }
                prog.register_symbol(&s.name, addr);
            }
            SymKind::Data => {
                let arch = prog.arch_mut();
                let (scope, base) = arch
                    .symboltab
                    .find_create_scope_from_symbol_name(&s.name, "::", None, num_spaces)?;
                // Untyped data object (a typed string symbol carries its char[N]
                // type through a dedicated fact added with the string pass).
                let ct = arch.types().get_type_code()?;
                let (sid, _) =
                    arch.symboltab.add_symbol_mapped(scope, &base, ct, &addr, &Address::new_invalid())?;
                arch.symboltab
                    .set_attribute(sid, kuna_decomp::varnode::varnode_flags::namelock);
            }
        }
    }

    // 2. Discovered entry points (stripped targets): name + add_function +
    //    register_symbol (the `map function` recipe).
    for &vma in &out.entries {
        let addr = Address::new(Rc::clone(code_space), vma);
        let name = prog.arch().name_function(&addr);
        let type_code = prog.arch().types().get_type_code()?;
        let min_size = prog.arch().min_funcsymbol_size;
        {
            let arch = prog.arch_mut();
            let (scope, base) = arch
                .symboltab
                .find_create_scope_from_symbol_name(&name, "::", None, num_spaces)?;
            if arch.symboltab.find_function(scope, &addr).is_none() {
                arch.symboltab.add_function(scope, &addr, &base, min_size, type_code)?;
            }
        }
        prog.register_symbol(&name, addr);
    }

    // 3. No-return functions: resolve each matched function and set the no-return
    //    flag (the batch form of OptionNoReturn::apply). A fact with no matching
    //    function is the faithful no-op — Ghidra only iterates existing symbols.
    //
    //    Resolution is by **address** (find_function_across_scopes, the same
    //    cross-scope resolver the call resolver uses): the address is the stable
    //    key. The demangle pass renames the funcsym before it is installed, so a
    //    mangled C++ no-return symbol (`_ZSt9terminatev`) is installed as
    //    `std::terminate` in scope `std` — a name lookup of the raw mangled string
    //    would miss it, but the funcsym is still at the same address the no-return
    //    scan saw. The name path is the fallback for an import that exists only as
    //    a differently-addressed PLT stub (e.g. when the scan's `.dynsym` address
    //    is 0 / unmapped but the PLT-named FunctionSymbol resolves by name).
    //    NOTE: the public Database method, NOT the private
    //    Architecture::set_function_no_return.
    let mut nr = out.noreturn.clone();
    nr.sort_by(|a, b| (a.addr, &a.name).cmp(&(b.addr, &b.name)));
    nr.dedup();
    for fact in &nr {
        // Address path (preferred): resolves a demangled/namespaced funcsym.
        let by_addr = if fact.addr != 0 {
            let addr = Address::new(Rc::clone(code_space), fact.addr);
            prog.arch().symboltab.find_function_across_scopes(&addr).map(|(sid, _)| sid)
        } else {
            None
        };
        let sid = match by_addr {
            Some(sid) => Some(sid),
            // Name fallback: an import with no function installed at `fact.addr`
            // (e.g. an undefined-address `.dynsym` entry whose only installed
            // FunctionSymbol is the PLT stub, resolved by its installed name).
            None => prog.arch().query_global_function(&fact.name).ok(),
        };
        if let Some(sid) = sid {
            prog.arch_mut().symboltab.set_function_no_return(sid, true);
        }
    }

    // 4. Detected string literals: place a typelocked `char[len]` data symbol at
    //    each (the kuna analog of `DataUtilities.createData` for an ASCII string),
    //    so the engine+printer render the literal instead of the bare constant.
    //    Ghidra's StringsAnalyzer marks the data and locks its type; the typelock
    //    is what keeps the array char[N] type through type propagation.
    for fact in &out.strings {
        let addr = Address::new(Rc::clone(code_space), fact.addr);
        // Conservative guard: skip an address that already carries a symbol (an
        // existing data/function symbol must not be shadowed). Ghidra likewise
        // only lays string data where the listing is undefined.
        let occupied = {
            let arch = prog.arch();
            match arch.symboltab.get_global_scope() {
                Some(global) => {
                    arch.symboltab.find_function(global, &addr).is_some()
                        || arch
                            .symboltab
                            .find_container(global, &addr, 1, &Address::new_invalid())
                            .is_some()
                }
                None => false,
            }
        };
        if occupied {
            continue;
        }

        // char[len]: getTypeChar(getSizeOfChar()) -> getTypeArray(len, char). Both
        // are fallible TypeFactory queries (the type group must be built by now).
        let char_size = prog.arch().types().get_size_of_char();
        let ch = prog.arch().types().get_type_char(char_size)?;
        let arr = prog.arch().types().get_type_array(fact.len as i32, ch)?;

        // A cosmetic synthetic name `s_<addr>` (the symbol exists to carry the
        // char[N] type at the address; the printer renders the literal, not the
        // name). Placed in its namespace-resolved scope (global here).
        let name = format!("s_{:x}", fact.addr);
        let arch = prog.arch_mut();
        let (scope, base) =
            arch.symboltab.find_create_scope_from_symbol_name(&name, "::", None, num_spaces)?;
        let (sid, _) =
            arch.symboltab.add_symbol_mapped(scope, &base, arr, &addr, &Address::new_invalid())?;
        arch.symboltab.set_attribute(sid, kuna_decomp::varnode::varnode_flags::typelock);
    }

    // 5. Library prototypes (the kuna analog of ApplyDataArchiveAnalyzer): park each
    //    on its named global callee. `ActionDefaultParams` later copies the callee's
    //    prototype into the caller's call spec, so the argument constants get typed
    //    (e.g. `puts(char*)` types `0x400915` as a char pointer, which — with the
    //    read-only markup applied above and the StringManager — renders the string
    //    literal `puts("Username: ")`). A name with no matching FunctionSymbol is a
    //    silent no-op.
    for pieces in out.prototypes {
        let name = pieces.name.clone();
        prog.arch_mut().set_function_prototype_pieces(&name, pieces);
    }

    // 6. Processor-context decode-mode paints (the kuna analog of ARM's
    //    `ARM_ElfExtension`/`ArmSymbolAnalyzer` `programContext.setValue(TMode,…)`).
    //    Paint each over the engine's ContextDatabase BEFORE any instruction is
    //    decoded (we are still inside bootstrap_from_elf, before any `load
    //    function` decode — the timing the ARM Thumb mode requires). `end: None`
    //    is the single-address point set (paint-to-next-change-point, Ghidra's
    //    per-symbol `setValue(v,a,a,val)` shape); `Some(end)` paints the explicit
    //    `[addr, end)` range (the `$t`-run form).
    //
    //    CRITICAL gate-safety: `set_variable`/`set_variable_region` return Err when
    //    the named context variable is NOT registered by the active language (e.g.
    //    `TMode` on x86-64). That MUST be a SILENT no-op — otherwise every non-ARM
    //    ELF decompile would regress. The producing pass already gates on the
    //    object being ARM (so on a non-ARM binary `out.context_paints` is empty),
    //    and this swallow is the belt-and-suspenders second guard.
    for paint in &out.context_paints {
        let begin = Address::new(Rc::clone(code_space), paint.addr);
        // Drop the Result: an unregistered context variable (non-ARM language) is
        // a faithful no-op, mirroring ArmSymbolAnalyzer.canAnalyze == false.
        let _ = prog.arch().with_context_db_mut(|db| match paint.end {
            Some(end) => {
                let endad = Address::new(Rc::clone(code_space), end);
                db.set_variable_region(paint.var.as_bytes(), &begin, &endad, paint.value)
            }
            None => db.set_variable(paint.var.as_bytes(), &begin, paint.value),
        });
    }

    // 7. Tracked register VALUES (the kuna analog of MipsAddressAnalyzer's
    //    per-function register-value seeding / the console `set track <reg> <val>
    //    <start> <end>`). For each fact, resolve the register varnode and seed the
    //    constant over `[func_addr, func_addr+1)` via `create_set` — the exact
    //    `IfcSettrackedrange` recipe (ifacedecomp.rs IfcSettrackedrange). The
    //    per-function `build_arch_handle` then snapshots the track base into the
    //    seam (`seam.tracked_sets = clone_trackbase()`), and `ActionConstbase` (S3)
    //    emits `COPY #val -> reg` at the entry block, which constant propagation
    //    consumes (so a MIPS PIC `$gp`-relative load resolves). Committed here, at
    //    `read symbols`, BEFORE any `load function` decode — the correct timing.
    //
    //    CRITICAL gate-safety (same shape as the context paints above):
    //    `get_register_varnode` returns Err when the named register is NOT defined
    //    by the active language (e.g. `t9` on x86-64). That MUST be a SILENT no-op —
    //    otherwise a non-MIPS decompile would regress. The producing pass already
    //    gates on the object being MIPS (so on a non-MIPS binary `out.tracked_regs`
    //    is empty), and this swallow is the belt-and-suspenders second guard.
    for fact in &out.tracked_regs {
        let Ok(loc) = prog.arch().get_register_varnode(fact.reg.as_bytes()) else {
            // Register not defined by this language (non-MIPS): faithful no-op.
            continue;
        };
        let begin = Address::new(Rc::clone(code_space), fact.func_addr);
        // `[func_addr, func_addr+1)` — the per-function point range Ghidra uses
        // (setRegisterValue(funcAddr, funcAddr, …)); +1 makes the [addr1, addr2)
        // create_set range non-empty (addr2 > addr1, as IfcSettrackedrange requires).
        let end = Address::new(Rc::clone(code_space), fact.func_addr.wrapping_add(1));
        let val = fact.value;
        prog.arch().with_context_db_mut(|db| {
            // C++ createSet(addr1,addr2); track = def (copy default as base); push —
            // the exact IfcSettrackedrange body for a ranged `set track`.
            let def = db.get_tracked_default().clone();
            let track = db.create_set(&begin, &end);
            *track = def;
            track.push(kuna_sleigh::globalcontext::TrackedContext { loc, val });
        });
    }

    // 8. Call-fixups (the kuna analog of CallFixupAnalyzer's install loop): for
    //    each function the pass matched to a cspec call-fixup `<target>`, tag it
    //    with that fixup's inject id so the engine replaces the CALL with the fixup
    //    body (e.g. the `-pg` `mcount`/`__fentry__` profiling call dissolves). This
    //    is precisely the `IfcFixupApply` body (resolve fixup name → inject id;
    //    resolve function name → sid; set_function_inject_id), driven by the
    //    analyzer instead of by hand — and guarded by Ghidra's `getCallFixup()==null`
    //    check (`CallFixupAnalyzer.java:89`): only set when no fixup is already
    //    parked, so a hand-applied `fixup apply` is never clobbered. A name with no
    //    registered fixup or no matching FunctionSymbol is a silent no-op.
    if !out.call_fixups.is_empty() {
        // Rebuild the same target→fixup map the pass used (the fact carries only
        // the function name; the fixup is re-derived from the live pcodeinjectlib).
        let map = kuna_analysis::s1_callfixup::target_fixup_map(prog.arch());
        for fact in &out.call_fixups {
            let Some(fixup_name) =
                kuna_analysis::s1_callfixup::call_fixup_name_for_function(&fact.func_name, &map)
            else {
                continue;
            };
            // fixup name → inject id (CALLFIXUP_TYPE); -1 if unknown (no-op).
            let injectid = prog
                .arch()
                .pcodeinjectlib
                .base
                .get_payload_id(kuna_decomp::pcodeinject::CALLFIXUP_TYPE, fixup_name.as_bytes());
            if injectid < 0 {
                continue;
            }
            // function name → FunctionSymbol id; skip if it no longer resolves.
            let Ok(sid) = prog.arch().query_global_function(&fact.func_name) else {
                continue;
            };
            // getCallFixup()==null guard: only auto-apply when no fixup is set.
            if prog.arch().symboltab.function_inject_id_for_symbol(sid) >= 0 {
                continue;
            }
            prog.arch_mut().symboltab.set_function_inject_id(sid, injectid);
        }
    }

    // 8. DWARF stack LOCALS (the kuna analog of `DWARFFunctionImporter`'s
    //    `commitLocal` loop — DWARF subtask 3): park each named, typed
    //    `DW_OP_fbreg` local on its owning function (by entry VMA). Unlike the
    //    other arms, a local is NOT installed into a persistent symbol table here:
    //    a function's `ScopeLocal` is built fresh per-decompile (it is owned by the
    //    transient `Funcdata`, not the global symboltab). So these are stashed and
    //    re-seeded into the rebuilt `Funcdata`'s stack scope at decompile time by
    //    `IfcDecompile` (via `dwarf_locals_for` -> the `map addr`/`seed_mapped_symbols`
    //    path) — exactly how the console carries a hand-typed `map addr` stack
    //    symbol across the IR rebuild. `stack_offset` is already in stack-space
    //    coordinates (`call_frame_cfa + fbreg`); the Address is built lazily at
    //    lookup so it binds the live stack space.
    for fact in out.locals {
        prog.dwarf_locals
            .push((fact.func_addr, fact.name, fact.type_, fact.stack_offset));
    }

    Ok(())
}

/// Bootstrap from a parsed XML document root (a `<binaryimage>` or a
/// `<decompilertest>` wrapping one), reading the arch id off the
/// `<binaryimage>` element.  Used by `load file` (path → parse → bootstrap).
pub fn bootstrap_from_root(root: &Rc<Element>, spec_roots: &[String]) -> KunaResult<ConsoleProgram> {
    let binaryimage = find_binaryimage(root)
        .ok_or_else(|| KunaError::lowlevel("Could not find binaryimage tag"))?;
    let arch_id = attr(&binaryimage, "arch")
        .ok_or_else(|| KunaError::lowlevel("<binaryimage> has no arch attribute"))?;
    bootstrap_program(binaryimage, &arch_id, spec_roots)
}

/// The ELF magic (`\x7fELF`), used to route `load file` to the real-binary path.
const ELF_MAGIC: [u8; 4] = [0x7f, b'E', b'L', b'F'];

/// Bootstrap from a file path (the `decomp_dbg` `load file [<target>] <path>`
/// body).  Detects the format by its leading bytes: a `\x7fELF` magic routes to
/// the real-ELF [`ObjectLoadImage`] path; anything else is parsed as the XML
/// `<binaryimage>`/`<decompilertest>` corpus format.
///
/// This mirrors the C++ `ArchitectureCapability::findCapability` dispatch: the
/// `xml` capability's `isFileMatch` claims a `<bi…` document, otherwise the BFD
/// path handles the real binary.  `target` is the optional `load file` target
/// token (the C++ BFD target / an explicit SLEIGH language id); it is honored on
/// the ELF path and ignored on the XML path (the XML carries its own `arch`).
pub fn bootstrap_from_file(
    path: &str,
    target: &str,
    spec_roots: &[String],
) -> KunaResult<ConsoleProgram> {
    let bytes = std::fs::read(path)
        .map_err(|e| KunaError::lowlevel(format!("Unable to recognize imagefile {path}: {e}")))?;
    if bytes.len() >= 4 && bytes[..4] == ELF_MAGIC {
        // Real ELF binary: drive the object-crate loader.
        return bootstrap_from_elf(path, target, spec_roots);
    }
    let mut store = DocumentStorage::new();
    let root = store.parse_document(&bytes)?.get_root().clone();
    bootstrap_from_root(&root, spec_roots)
}

/// Iterate the opened [`LoadImageXml`]'s symbol records (name → address) into a
/// [`ProgramSymbol`] list (the `readLoaderSymbols` seam).
fn read_loader_symbols(loader: Option<&LoadImageXml>) -> Vec<ProgramSymbol> {
    match loader {
        Some(l) => read_loader_symbols_generic(l),
        None => Vec::new(),
    }
}

/// `readLoaderSymbols` over any opened [`LoadImage`] (the ELF path reuses this
/// against the [`ObjectLoadImage`]; the symbol must already be attached to a
/// space so `getNextSymbol` can build the `Address`).
fn read_loader_symbols_generic(loader: &dyn LoadImage) -> Vec<ProgramSymbol> {
    let mut out = Vec::new();
    loader.open_symbols();
    loop {
        let mut record = LoadImageFunc::default();
        if !loader.get_next_symbol(&mut record) {
            break;
        }
        let name = String::from_utf8_lossy(&record.name).into_owned();
        out.push(ProgramSymbol { name, addr: record.address });
    }
    out
}

/// A load image that errors on every read — handed to `build_translator` as the
/// placeholder before the real opened image replaces it (the C++ `DummyImg` /
/// e2e-gate shape).
struct NullLoad;

impl LoadImage for NullLoad {
    fn get_file_name(&self) -> &str {
        "null"
    }
    fn load_fill(&mut self, _ptr: &mut [u8], _addr: &Address) -> KunaResult<()> {
        Err(KunaError::data_unavail("null load image"))
    }
    fn get_arch_type(&self) -> Vec<u8> {
        Vec::new()
    }
    fn adjust_vma(&mut self, _adjust: i64) {}
}

/// Default the analysis-size bound (`load function`/`load addr` size 0 = the
/// function's natural extent), mirroring the C++ `IfcFuncload` / `IfcAddrrangeLoad`
/// unbounded follow.
pub const UNBOUNDED_SIZE: int4 = 0;

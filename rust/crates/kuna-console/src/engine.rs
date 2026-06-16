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
use kuna_base::space::AddrSpaceManager;
use kuna_base::types::int4;
use kuna_base::xml::{DocumentStorage, Element};

use kuna_decomp::architecture::Architecture;
use kuna_decomp::options::register_option_elements;
use kuna_decomp::sleigh_arch::{register_sleigh_arch_ids, LanguageDatabase, SleighArchitecture};
use kuna_decomp::xml_arch::XmlArchitectureCapability;

use kuna_sleigh::loadimage::{LoadImage, LoadImageFunc};
use kuna_sleigh::loadimage_object::ObjectLoadImage;
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
    let mut prog = ConsoleProgram { arch: arch.into_sleigh(), registry, symbols, description };
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

    // LoadImageBfd(filename) + open(): parse the ELF (machine, segments, symbols).
    let mut loader = ObjectLoadImage::open(path)?;

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
    loader.attach_to_space(code_space);

    // readLoaderSymbols (the ELF FUNC symbols) BEFORE handing the loader off.
    let symbols = read_loader_symbols_generic(&loader);

    // Hand the loader to the engine (the C++ `loader` back-pointer the decode
    // reads on load_fill).
    sleigh.base_mut().unwrap().set_loader(Box::new(loader));

    let description = sleigh.base().unwrap().get_description().to_string();

    let mut prog = ConsoleProgram { arch: sleigh, registry, symbols, description };
    // conf->readLoaderSymbols("::"): install the ELF symbols as FunctionSymbols.
    prog.read_loader_symbols()?;
    Ok(prog)
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

//! Port of `decompiler/cpp/sleigh_arch.{cc,hh}` (item `w4-fw-arch-frontends`) —
//! the SLEIGH-backed [`Architecture`] front-end.
//!
//! [`SleighArchitecture`] is the C++ class that knows how to natively read in a
//! compiled SLEIGH spec (`.sla`), a processor spec (`.pspec`), and a compiler
//! spec (`.cspec`).  Given a \e language \e id (e.g. `x86:LE:64:default`) it
//! resolves the matching `.ldefs` record, finds the spec files, and builds the
//! `Translate` (Sleigh) engine.
//!
//! ## Inheritance modeled by composition
//!
//! In C++ `SleighArchitecture : public Architecture` and the leaf frontends
//! (`XmlArchitecture`, `RawBinaryArchitecture`) derive from it.  Rust has no
//! class inheritance, so the port models the chain by **composition**:
//! [`SleighArchitecture`] owns an [`Architecture`] (`base`, the W4
//! `architecture.rs` god object) plus the SleighArchitecture-specific fields
//! (`filename`, `target`, `languageindex`, `archid`, `errorstream`).  The leaf
//! frontends ([`crate::xml_arch::XmlArchitecture`],
//! [`crate::raw_arch::RawBinaryArchitecture`]) own a `SleighArchitecture` in
//! turn.  Every C++ `SleighArchitecture::method` becomes an inherent method on
//! [`SleighArchitecture`]; the leaf overrides (`buildLoader`/`postSpecFile`/
//! `resolveArchitecture`) live on the leaf structs and delegate to the base.
//!
//! ## The C++ statics, ported per the W1 no-static convention
//!
//! `sleigh_arch.cc` carries three process-global statics:
//! `static map<int4,Sleigh> translators`, `static vector<LanguageDescription>
//! description`, and `static FileManage specpaths`.  Per the kuna `IdRegistry`/
//! `CapabilityRegistry` convention (no nondeterministic static constructors),
//! these move into an explicit value, [`LanguageDatabase`], that the bootstrap
//! owns and threads through.  `description`/`specpaths` are the load-bearing
//! pieces the language resolution reads; the `translators` cache (a `.sla`
//! reuse optimization) is **not** ported here — each `SleighArchitecture` owns
//! its own built `Sleigh` (the cache only affects build *speed*, never output,
//! and the W4 `Architecture` already owns its `translate` by value).  The
//! `isTranslateReused` reuse flag therefore reports `false` (a fresh build),
//! which is faithful for output and recorded as LOSS (translator reuse cache).
//!
//! ## What is wired vs. seamed
//!
//! - **Wired now**: the `.ldefs` record types ([`CompilerTag`],
//!   [`LanguageDescription`]) and their `decode`; `getCompiler` (default
//!   fallback); `loadLanguageDescription`/`collectSpecFiles`/`getDescriptions`
//!   (the `.ldefs` scan over the spec paths via kuna-base [`FileManage`]);
//!   `resolveArchitecture` (the language-id normalization + index resolution);
//!   `normalizeProcessor`/`normalizeEndian`/`normalizeSize`/
//!   `normalizeArchitecture`; `scanForSleighDirectories`; `buildSpecFile` (find
//!   the pspec/cspec/sla files); `encodeHeader`/`restoreXmlHeader`;
//!   `buildTranslator` (decode the resolved `.sla` into a [`Sleigh`]);
//!   `modifySpaces` (apply `.ldefs` truncations).
//!
//! - **Seamed (W6/W8)**: the full `Architecture::init` / `restoreFromSpec`
//!   flow (it reaches `buildTypegrp` → W6 `TypeFactory`, `print` → W8
//!   `PrintLanguage`, `parseProcessorConfig`/`parseCompilerConfig` →
//!   prototype models + data-organization).  `buildContext`'s
//!   `context->decodeFromSpec` (the `.pspec` `<context_data>` → `Sleigh`
//!   context application) is the piece a full bootstrap needs, but the
//!   `ContextInternal` blob is owned inside the `Sleigh` engine, not a separate
//!   `ContextDatabase` subsystem in the W4 `Architecture`; the per-fixture
//!   context defaults are applied directly to the `Sleigh` (the corpus
//!   bootstrap gate does exactly this).  `buildSymbols` (the `<default_symbols>`
//!   → `Database::addSymbol` markup) reaches the W4 symbol-scope `addSymbol`
//!   surface that is itself a later seam.  Each is documented inline with
//!   `// STUB(...)`.

use kuna_base::error::{KunaError, KunaResult};
use kuna_base::filemanage::FileManage;
use kuna_base::marshal::{
    AttributeId, Decoder, ElementId, IdRegistry, XmlDecode, ATTRIB_CONTENT, ATTRIB_ID, ATTRIB_NAME,
    ATTRIB_SIZE,
};
use kuna_base::space::AddrSpaceManager;
use kuna_base::types::int4;

use kuna_sleigh::globalcontext::ContextInternal;
use kuna_sleigh::loadimage::LoadImage;
use kuna_sleigh::sleigh::Sleigh;
use kuna_sleigh::translate::{TruncationTag, ELEM_TRUNCATE_SPACE};

use crate::architecture::Architecture;

// ---------------------------------------------------------------------------
// Marshaling ids (C++ sleigh_arch.cc:21-34 + architecture.cc:40 for adjustvma)
// ---------------------------------------------------------------------------

/// Marshaling attribute "deprecated" (C++ `ATTRIB_DEPRECATED`).
pub const ATTRIB_DEPRECATED: AttributeId = AttributeId::new("deprecated", 136);
/// Marshaling attribute "endian" (C++ `ATTRIB_ENDIAN`).
pub const ATTRIB_ENDIAN: AttributeId = AttributeId::new("endian", 137);
/// Marshaling attribute "processor" (C++ `ATTRIB_PROCESSOR`).
pub const ATTRIB_PROCESSOR: AttributeId = AttributeId::new("processor", 138);
/// Marshaling attribute "processorspec" (C++ `ATTRIB_PROCESSORSPEC`).
pub const ATTRIB_PROCESSORSPEC: AttributeId = AttributeId::new("processorspec", 139);
/// Marshaling attribute "slafile" (C++ `ATTRIB_SLAFILE`).
pub const ATTRIB_SLAFILE: AttributeId = AttributeId::new("slafile", 140);
/// Marshaling attribute "spec" (C++ `ATTRIB_SPEC`).
pub const ATTRIB_SPEC: AttributeId = AttributeId::new("spec", 141);
/// Marshaling attribute "target" (C++ `ATTRIB_TARGET`).
pub const ATTRIB_TARGET: AttributeId = AttributeId::new("target", 142);
/// Marshaling attribute "variant" (C++ `ATTRIB_VARIANT`).
pub const ATTRIB_VARIANT: AttributeId = AttributeId::new("variant", 143);
/// Marshaling attribute "version" (C++ `ATTRIB_VERSION`).
pub const ATTRIB_VERSION: AttributeId = AttributeId::new("version", 144);
/// Marshaling attribute "adjustvma" (C++ `ATTRIB_ADJUSTVMA`,
/// `architecture.cc:40` — used by the [`crate::raw_arch`]/[`crate::xml_arch`]
/// save-file marshaling).
pub const ATTRIB_ADJUSTVMA: AttributeId = AttributeId::new("adjustvma", 103);

/// Marshaling element \<compiler> (C++ `ELEM_COMPILER`).
pub const ELEM_COMPILER: ElementId = ElementId::new("compiler", 232);
/// Marshaling element \<description> (C++ `ELEM_DESCRIPTION`).
pub const ELEM_DESCRIPTION: ElementId = ElementId::new("description", 233);
/// Marshaling element \<language> (C++ `ELEM_LANGUAGE`).
pub const ELEM_LANGUAGE: ElementId = ElementId::new("language", 234);
/// Marshaling element \<language_definitions> (C++ `ELEM_LANGUAGE_DEFINITIONS`).
pub const ELEM_LANGUAGE_DEFINITIONS: ElementId = ElementId::new("language_definitions", 235);

/// Register every sleigh_arch.cc AttributeId/ElementId on the given registry
/// (the explicit stand-in for C++ static-object registration; follows the
/// `register_translate_ids` convention).  [`ELEM_TRUNCATE_SPACE`] is **not**
/// re-registered here (it is a kuna-sleigh id registered by
/// `register_translate_ids`); the caller must register both for a full ldefs
/// decode that contains `<truncate_space>` children.
pub fn register_sleigh_arch_ids(registry: &mut IdRegistry) {
    registry.register_attribute(&ATTRIB_DEPRECATED);
    registry.register_attribute(&ATTRIB_ENDIAN);
    registry.register_attribute(&ATTRIB_PROCESSOR);
    registry.register_attribute(&ATTRIB_PROCESSORSPEC);
    registry.register_attribute(&ATTRIB_SLAFILE);
    registry.register_attribute(&ATTRIB_SPEC);
    registry.register_attribute(&ATTRIB_TARGET);
    registry.register_attribute(&ATTRIB_VARIANT);
    registry.register_attribute(&ATTRIB_VERSION);
    registry.register_attribute(&ATTRIB_ADJUSTVMA);
    registry.register_element(&ELEM_COMPILER);
    registry.register_element(&ELEM_DESCRIPTION);
    registry.register_element(&ELEM_LANGUAGE);
    registry.register_element(&ELEM_LANGUAGE_DEFINITIONS);
}

// ---------------------------------------------------------------------------
// CompilerTag (sleigh_arch.hh:46)
// ---------------------------------------------------------------------------

/// \brief Contents of a \<compiler> tag in a .ldefs file (C++ `class
/// CompilerTag`, `sleigh_arch.hh:46`).
///
/// Describes a compiler specification file as referenced by the Sleigh language
/// subsystem.
#[derive(Debug, Clone, Default)]
pub struct CompilerTag {
    /// (Human readable) name of the compiler (C++ `name`).
    name: String,
    /// cspec file for this compiler (C++ `spec`).
    spec: String,
    /// Unique id for this compiler (C++ `id`).
    id: String,
}

impl CompilerTag {
    /// Construct an empty tag (C++ default ctor).
    pub fn new() -> CompilerTag {
        CompilerTag::default()
    }

    /// Parse file attributes from a \<compiler> element (C++
    /// `CompilerTag::decode`, `sleigh_arch.cc:43`).
    pub fn decode(&mut self, decoder: &mut dyn Decoder) -> KunaResult<()> {
        let elem_id = decoder.open_element_id(&ELEM_COMPILER)?;
        self.name = read_string(decoder, &ATTRIB_NAME)?;
        self.spec = read_string(decoder, &ATTRIB_SPEC)?;
        self.id = read_string(decoder, &ATTRIB_ID)?;
        decoder.close_element(elem_id)?;
        Ok(())
    }

    /// Get the human readable name of the spec (C++ `getName`).
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Get the file-name (C++ `getSpec`).
    pub fn get_spec(&self) -> &str {
        &self.spec
    }

    /// Get the string used as part of \e language \e id (C++ `getId`).
    pub fn get_id(&self) -> &str {
        &self.id
    }
}

// ---------------------------------------------------------------------------
// LanguageDescription (sleigh_arch.hh:66)
// ---------------------------------------------------------------------------

/// \brief Contents of the \<language> tag in a .ldefs file (C++ `class
/// LanguageDescription`, `sleigh_arch.hh:66`).
///
/// Meta-data describing a single processor and the set of files used to analyze
/// it: a compiled SLEIGH file (`.sla`), a processor spec (`.pspec`), one or more
/// compiler specs (`.cspec`), and address-space truncations.
#[derive(Debug, Clone, Default)]
pub struct LanguageDescription {
    /// Name of processor (C++ `processor`).
    processor: String,
    /// `true` if this processor is \e big-endian (C++ `isbigendian`).
    isbigendian: bool,
    /// Size of address bus in bits (C++ `size`).
    size: int4,
    /// Name of processor variant or "default" (C++ `variant`).
    variant: String,
    /// Version of the specification (C++ `version`).
    version: String,
    /// Name of `.sla` file for processor (C++ `slafile`).
    slafile: String,
    /// Name of `.pspec` file (C++ `processorspec`).
    processorspec: String,
    /// Unique id for this language (C++ `id`).
    id: String,
    /// Human readable description of this language (C++ `description`).
    description: String,
    /// `true` if the specification is considered \e deprecated (C++ `deprecated`).
    deprecated: bool,
    /// List of compiler specifications compatible with this processor
    /// (C++ `compilers`).
    compilers: Vec<CompilerTag>,
    /// Address space truncations required by this processor (C++ `truncations`).
    truncations: Vec<TruncationTag>,
}

impl LanguageDescription {
    /// Construct an empty description (C++ default ctor).
    pub fn new() -> LanguageDescription {
        LanguageDescription::default()
    }

    /// Parse \b this description from a stream (C++ `LanguageDescription::decode`,
    /// `sleigh_arch.cc:55`).
    pub fn decode(&mut self, decoder: &mut dyn Decoder) -> KunaResult<()> {
        let elem_id = decoder.open_element_id(&ELEM_LANGUAGE)?;
        self.processor = read_string(decoder, &ATTRIB_PROCESSOR)?;
        self.isbigendian = read_string(decoder, &ATTRIB_ENDIAN)? == "big";
        self.size = decoder.read_signed_integer_id(&ATTRIB_SIZE)? as int4; // cast: int8 -> int4 member (address bus bits)
        self.variant = read_string(decoder, &ATTRIB_VARIANT)?;
        self.version = read_string(decoder, &ATTRIB_VERSION)?;
        self.slafile = read_string(decoder, &ATTRIB_SLAFILE)?;
        self.processorspec = read_string(decoder, &ATTRIB_PROCESSORSPEC)?;
        self.id = read_string(decoder, &ATTRIB_ID)?;
        self.deprecated = false;
        // Optional attributes: for(;;) { attribId = getNextAttributeId(); ... }
        loop {
            let attrib_id = decoder.get_next_attribute_id()?;
            if attrib_id == 0 {
                break;
            }
            if attrib_id == ATTRIB_DEPRECATED {
                self.deprecated = decoder.read_bool()?;
            }
        }
        // Child elements: <description>, <compiler>, <truncate_space>, others ignored.
        loop {
            let sub_id = decoder.peek_element()?;
            if sub_id == 0 {
                break;
            }
            if sub_id == ELEM_DESCRIPTION {
                decoder.open_element()?;
                self.description = read_string(decoder, &ATTRIB_CONTENT)?;
                decoder.close_element(sub_id)?;
            } else if sub_id == ELEM_COMPILER {
                let mut tag = CompilerTag::new();
                tag.decode(decoder)?;
                self.compilers.push(tag);
            } else if sub_id == ELEM_TRUNCATE_SPACE {
                let mut tag = TruncationTag::default();
                tag.decode(decoder)?;
                self.truncations.push(tag);
            } else {
                // Ignore other child elements.
                decoder.open_element()?;
                decoder.close_element_skipping(sub_id)?;
            }
        }
        decoder.close_element(elem_id)?;
        Ok(())
    }

    /// Get the name of the processor (C++ `getProcessor`).
    pub fn get_processor(&self) -> &str {
        &self.processor
    }

    /// Return `true` if the processor is big-endian (C++ `isBigEndian`).
    pub fn is_big_endian(&self) -> bool {
        self.isbigendian
    }

    /// Get the size of the address bus (C++ `getSize`).
    pub fn get_size(&self) -> int4 {
        self.size
    }

    /// Get the processor variant (C++ `getVariant`).
    pub fn get_variant(&self) -> &str {
        &self.variant
    }

    /// Get the processor version (C++ `getVersion`).
    pub fn get_version(&self) -> &str {
        &self.version
    }

    /// Get filename of the SLEIGH specification (C++ `getSlaFile`).
    pub fn get_sla_file(&self) -> &str {
        &self.slafile
    }

    /// Get the filename of the processor specification (C++ `getProcessorSpec`).
    pub fn get_processor_spec(&self) -> &str {
        &self.processorspec
    }

    /// Get the \e language \e id string (C++ `getId`).
    pub fn get_id(&self) -> &str {
        &self.id
    }

    /// Get a description of the processor (C++ `getDescription`).
    pub fn get_description(&self) -> &str {
        &self.description
    }

    /// Return `true` if this specification is deprecated (C++ `isDeprecated`).
    pub fn is_deprecated(&self) -> bool {
        self.deprecated
    }

    /// Pick out the [`CompilerTag`] associated with the desired \e compiler \e id
    /// string (C++ `LanguageDescription::getCompiler(const string&)`,
    /// `sleigh_arch.cc:101`).
    ///
    /// If the id is not matched, the `"default"` compiler is returned; if there
    /// is no `"default"` either, the first compiler is returned (C++ indexes
    /// `compilers[0]` — UB if empty, mirrored here as a panic).
    pub fn get_compiler(&self, nm: &str) -> &CompilerTag {
        let mut defaultind: i64 = -1;
        // for(int4 i=0;i<compilers.size();++i)
        for (i, c) in self.compilers.iter().enumerate() {
            if c.get_id() == nm {
                return c;
            }
            if c.get_id() == "default" {
                defaultind = i as i64;
            }
        }
        if defaultind != -1 {
            // If can't match compiler, return default
            return &self.compilers[defaultind as usize];
        }
        &self.compilers[0]
    }

    /// Get the number of compiler records (C++ `numCompilers`).
    pub fn num_compilers(&self) -> int4 {
        self.compilers.len() as int4
    }

    /// Get the i-th compiler record (C++ `getCompiler(int4)`).
    pub fn get_compiler_index(&self, i: int4) -> &CompilerTag {
        &self.compilers[i as usize]
    }

    /// Get the number of truncation records (C++ `numTruncations`).
    pub fn num_truncations(&self) -> int4 {
        self.truncations.len() as int4
    }

    /// Get the i-th truncation record (C++ `getTruncation`).
    pub fn get_truncation(&self, i: int4) -> &TruncationTag {
        &self.truncations[i as usize]
    }
}

// ---------------------------------------------------------------------------
// LanguageDatabase — the explicit replacement for the sleigh_arch.cc statics
// ---------------------------------------------------------------------------

/// \brief The explicit value replacing the C++ `SleighArchitecture` statics
/// `description` (the parsed `.ldefs` records) and `specpaths` (the spec
/// directory list).
///
/// Per the kuna no-static convention (`capability.rs` / `IdRegistry`), the
/// process-global `static vector<LanguageDescription> description` and `static
/// FileManage specpaths` become this owned value, threaded through the
/// bootstrap.  `collectSpecFiles`/`getDescriptions`/`scanForSleighDirectories`
/// are inherent methods on it.
///
/// The C++ `static map<int4,Sleigh> translators` reuse cache is intentionally
/// **not** modeled (see module docs): it affects build speed only, never
/// output, and each `SleighArchitecture` owns its own built `Sleigh`.
#[derive(Debug, Clone, Default)]
pub struct LanguageDatabase {
    /// List of languages we know about (C++ `description`).
    description: Vec<LanguageDescription>,
    /// Known directories that contain .ldefs files (C++ `specpaths`).
    specpaths: FileManage,
}

impl LanguageDatabase {
    /// Construct an empty database.
    pub fn new() -> LanguageDatabase {
        LanguageDatabase::default()
    }

    /// Mutable access to the spec-path search list (the C++ `specpaths`
    /// `FileManage`), for the bootstrap to seed directories directly.
    pub fn specpaths_mut(&mut self) -> &mut FileManage {
        &mut self.specpaths
    }

    /// All parsed language descriptions, in `.ldefs` document then file order
    /// (C++ `description`).
    pub fn descriptions(&self) -> &[LanguageDescription] {
        &self.description
    }

    /// \brief Read a SLEIGH .ldefs file (C++
    /// `SleighArchitecture::loadLanguageDescription`, `sleigh_arch.cc:121`).
    ///
    /// Any \<language> tags are added to the description array.  A parse error
    /// warns (via `errs`) and returns without aborting, exactly as the C++
    /// `catch(DecoderError&)` does.  The `registry` supplies the
    /// element/attribute name → id tables (the C++ static `AttributeId::find` /
    /// `ElementId::find`); the caller must register the sleigh_arch + translate
    /// ids on it.
    pub fn load_language_description(
        &mut self,
        specfile: &str,
        errs: &mut String,
        registry: &IdRegistry,
    ) {
        // ifstream s(specfile.c_str()); if (!s) return;
        let bytes = match std::fs::read(specfile) {
            Ok(b) => b,
            Err(_) => return,
        };

        // XmlDecode decoder((const AddrSpaceManager *)0);  -- ldefs never decodes
        // a space, so an empty manager stands in for the C++ null pointer.
        let empty_manager = AddrSpaceManager::new();
        let mut decoder = XmlDecode::new(&empty_manager, registry);
        // try { decoder.ingestStream(s); } catch(DecoderError&) { warn; return; }
        if decoder.ingest_stream(&bytes).is_err() {
            errs.push_str("WARNING: Unable to parse sleigh specfile: ");
            errs.push_str(specfile);
            return;
        }

        // The <language_definitions> body decode (the for(;;) peek loop).  The
        // C++ has no try around the body, so a malformed <language> escapes
        // loadLanguageDescription; we record it into errs (the getDescriptions
        // path turns a non-empty errs into a LowlevelError).
        if self.decode_definitions(&mut decoder).is_err() {
            errs.push_str("WARNING: Unable to parse sleigh specfile: ");
            errs.push_str(specfile);
        }
    }

    /// Decode a `<language_definitions>` blob from an in-memory byte stream
    /// (the [`Self::load_language_description`] body minus the file read).  Useful
    /// for callers that already hold the `.ldefs` bytes (and for tests).  The
    /// `registry` must carry the sleigh_arch + translate ids.
    pub fn load_language_definitions(
        &mut self,
        bytes: &[u8],
        registry: &IdRegistry,
    ) -> KunaResult<()> {
        let empty_manager = AddrSpaceManager::new();
        let mut decoder = XmlDecode::new(&empty_manager, registry);
        decoder.ingest_stream(bytes)?;
        self.decode_definitions(&mut decoder)
    }

    /// The `<language_definitions>` body of [`Self::load_language_description`]
    /// (separated so the C++ `for(;;)` peek loop reads cleanly).
    fn decode_definitions(&mut self, decoder: &mut XmlDecode) -> KunaResult<()> {
        let elem_id = decoder.open_element_id(&ELEM_LANGUAGE_DEFINITIONS)?;
        loop {
            let sub_id = decoder.peek_element()?;
            if sub_id == 0 {
                break;
            }
            if sub_id == ELEM_LANGUAGE {
                let mut desc = LanguageDescription::new();
                desc.decode(decoder)?;
                self.description.push(desc);
            } else {
                decoder.open_element()?;
                decoder.close_element_skipping(sub_id)?;
            }
        }
        decoder.close_element(elem_id)?;
        Ok(())
    }

    /// \brief Gather specification files in normal locations (C++
    /// `SleighArchitecture::collectSpecFiles`, `sleigh_arch.cc:451`).
    ///
    /// Run once when spinning up the decompiler: look for the root `.ldefs`
    /// files within the spec paths and parse them.  Idempotent — if the
    /// description list is already populated, returns immediately (C++ `if
    /// (!description.empty()) return;`).
    pub fn collect_spec_files(&mut self, errs: &mut String, registry: &IdRegistry) {
        if !self.description.is_empty() {
            return; // Have we already collected before
        }
        let mut testspecs: Vec<String> = Vec::new();
        self.specpaths.match_list(&mut testspecs, ".ldefs", true);
        for spec in &testspecs {
            self.load_language_description(spec, errs, registry);
        }
    }

    /// \brief Parse all `.ldef` files and return the list of all
    /// [`LanguageDescription`] objects (C++
    /// `SleighArchitecture::getDescriptions`, `sleigh_arch.cc:616`).
    ///
    /// If there are any parse errors in the `.ldef` files, an error is returned
    /// (the C++ throws `LowlevelError(s.str())`).
    pub fn get_descriptions(
        &mut self,
        registry: &IdRegistry,
    ) -> KunaResult<&[LanguageDescription]> {
        let mut s = String::new();
        self.collect_spec_files(&mut s, registry);
        if !s.is_empty() {
            return Err(KunaError::lowlevel(s));
        }
        Ok(&self.description)
    }

    /// \brief Scan directories for SLEIGH specification files (C++
    /// `SleighArchitecture::scanForSleighDirectories`, `sleigh_arch.cc:572`).
    ///
    /// Assumes a standard `Ghidra/Processors/*/data/languages` layout, scans for
    /// all matching directories, and adds them to the specpaths.
    pub fn scan_for_sleigh_directories(&mut self, rootpath: &str) {
        let mut ghidradir: Vec<String> = Vec::new();
        let mut procdir: Vec<String> = Vec::new();
        let mut procdir2: Vec<String> = Vec::new();
        let mut languagesubdirs: Vec<String> = Vec::new();

        FileManage::scan_directory_recursive(&mut ghidradir, "Ghidra", rootpath, 2);
        // for(uint4 i=0;i<ghidradir.size();++i)
        for g in &ghidradir {
            FileManage::scan_directory_recursive(&mut procdir, "Processors", g, 1); // Processors structure
            FileManage::scan_directory_recursive(&mut procdir, "contrib", g, 1);
        }
        if !procdir.is_empty() {
            for p in &procdir {
                FileManage::directory_list(&mut procdir2, p, false);
            }

            let mut datadirs: Vec<String> = Vec::new();
            for p in &procdir2 {
                FileManage::scan_directory_recursive(&mut datadirs, "data", p, 1);
            }

            let mut languagedirs: Vec<String> = Vec::new();
            for d in &datadirs {
                FileManage::scan_directory_recursive(&mut languagedirs, "languages", d, 1);
            }

            for l in &languagedirs {
                languagesubdirs.push(l.clone());
            }

            // In the old version we have to go down one more level to get to ldefs
            for l in &languagedirs {
                FileManage::directory_list(&mut languagesubdirs, l, false);
            }
        }
        // If we haven't matched this directory structure, just use rootpath as the
        // directory containing the ldef
        if languagesubdirs.is_empty() {
            languagesubdirs.push(rootpath.to_string());
        }

        for l in &languagesubdirs {
            self.specpaths.add_dir_2_path(l);
        }
    }

    /// Apply this database's `.ldefs` space truncations for the active language
    /// to the given manager (C++ `SleighArchitecture::modifySpaces`,
    /// `sleigh_arch.cc:423`).  Walks `language.numTruncations()` and applies
    /// `trans->truncateSpace`.  (The manager truncates through interior
    /// mutability, so a `&AddrSpaceManager` suffices, matching kuna-base.)
    pub fn modify_spaces(
        &self,
        languageindex: int4,
        manager: &AddrSpaceManager,
    ) -> KunaResult<()> {
        let language = &self.description[languageindex as usize];
        // for(int4 i=0;i<language.numTruncations();++i) trans->truncateSpace(...)
        for i in 0..language.num_truncations() {
            let tag = language.get_truncation(i);
            manager.truncate_space(tag.get_name(), tag.get_size())?;
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// SleighArchitecture (sleigh_arch.hh:108)
// ---------------------------------------------------------------------------

/// \brief An [`Architecture`] that uses the decompiler's native SLEIGH
/// translation engine (C++ `class SleighArchitecture : public Architecture`,
/// `sleigh_arch.hh:108`).
///
/// Composition models the C++ inheritance: this struct owns the W4
/// [`Architecture`] (`base`) and adds the SleighArchitecture-specific state.
pub struct SleighArchitecture {
    /// The owned W4 [`Architecture`] god object (the C++ base subobject).
    ///
    /// `None` until [`SleighArchitecture::build_translator`] has decoded the
    /// `.sla` and constructed it (the C++ base subobject exists immediately, but
    /// its `translate`/subsystems are filled by `init`; the Rust `Architecture`
    /// requires a built `Sleigh` to construct, so it is deferred to the build).
    base: Option<Architecture>,

    /// Index (within the [`LanguageDescription`] array) of the active language
    /// (C++ `languageindex`).  `-1` until [`Self::resolve_architecture`] runs.
    languageindex: int4,
    /// Name of active load-image file (C++ `filename`).
    filename: String,
    /// The \e language \e id of the active load-image (C++ `target`).
    target: String,
    /// ID string uniquely describing this architecture (C++
    /// `Architecture::archid`, owned here until the [`Architecture`] base exists,
    /// then mirrored into it).
    archid: String,
    /// Accumulated error-console text (C++ `errorstream`, an `ostream*`).  The
    /// C++ writes warnings here; the Rust port appends to this string so
    /// `printMessage` is observable without an I/O stream.
    errorstream: String,
}

impl SleighArchitecture {
    /// \brief Construct given executable file (C++
    /// `SleighArchitecture::SleighArchitecture`, `sleigh_arch.cc:438`).
    ///
    /// Prepares \b this for analyzing the given executable image.  Full
    /// initialization (creation of the `Translate` object) still must be
    /// performed by the build flow.
    pub fn new(fname: &str, targ: &str) -> SleighArchitecture {
        SleighArchitecture {
            base: None,
            languageindex: -1,
            filename: fname.to_string(),
            target: targ.to_string(),
            archid: String::new(),
            errorstream: String::new(),
        }
    }

    /// Get the executable filename (C++ `getFilename`).
    pub fn get_filename(&self) -> &str {
        &self.filename
    }

    /// Get the \e language \e id of the active processor (C++ `getTarget`).
    pub fn get_target(&self) -> &str {
        &self.target
    }

    /// The active language index (C++ `languageindex`; `-1` before resolution).
    pub fn language_index(&self) -> int4 {
        self.languageindex
    }

    /// The resolved architecture id (C++ `Architecture::archid`).
    pub fn arch_id(&self) -> &str {
        &self.archid
    }

    /// Set the architecture id directly (C++ assigns the protected
    /// `Architecture::archid` member, e.g. `RawBinaryArchitecture::
    /// resolveArchitecture`'s `archid = getTarget()`).
    pub fn set_archid(&mut self, archid: &str) {
        self.archid = archid.to_string();
    }

    /// Accumulated error-console text (the C++ `errorstream` sink).
    pub fn error_text(&self) -> &str {
        &self.errorstream
    }

    /// Print a message to the error console (C++
    /// `SleighArchitecture::printMessage`, `sleigh_arch.hh:138`).
    pub fn print_message(&mut self, message: &str) {
        self.errorstream.push_str(message);
        self.errorstream.push('\n');
    }

    /// Borrow the built [`Architecture`] base, if the translator has been built.
    pub fn base(&self) -> Option<&Architecture> {
        self.base.as_ref()
    }

    /// Mutably borrow the built [`Architecture`] base, if present.
    pub fn base_mut(&mut self) -> Option<&mut Architecture> {
        self.base.as_mut()
    }

    /// Get a string describing this architecture (C++
    /// `SleighArchitecture::getDescription`, `sleigh_arch.cc:158` —
    /// `description[languageindex].getDescription()`).
    pub fn get_description<'a>(&self, db: &'a LanguageDatabase) -> &'a str {
        db.descriptions()[self.languageindex as usize].get_description()
    }

    /// \brief Find best architecture (C++
    /// `SleighArchitecture::resolveArchitecture`, `sleigh_arch.cc:323`).
    ///
    /// Normalizes the target/loader-derived id into a \e language \e id, strips
    /// the `binary-`/`default-` prefixes, derives the `baseid` (id minus the
    /// compiler field), and locates the matching [`LanguageDescription`] index.
    /// `arch_type` is the loader's `getArchType()` string (passed by the caller
    /// because the loader is a leaf-frontend concern — C++
    /// `loader->getArchType()`; used only when archid/target are empty).
    pub fn resolve_architecture(
        &mut self,
        db: &LanguageDatabase,
        arch_type: &str,
    ) -> KunaResult<()> {
        // if (archid.size() == 0) { ... }
        if self.archid.is_empty() {
            if self.target.is_empty() || self.target == "default" {
                self.archid = arch_type.to_string();
            } else {
                self.archid = self.target.clone();
            }
        }
        // if (archid.find("binary-")==0) archid.erase(0,7);
        // else if (archid.find("default-")==0) archid.erase(0,8);
        if let Some(rest) = self.archid.strip_prefix("binary-") {
            self.archid = rest.to_string();
        } else if let Some(rest) = self.archid.strip_prefix("default-") {
            self.archid = rest.to_string();
        }

        self.archid = normalize_architecture(&self.archid)?;
        // string baseid = archid.substr(0,archid.rfind(':'));
        let baseid = match self.archid.rfind(':') {
            Some(pos) => self.archid[..pos].to_string(),
            None => self.archid.clone(),
        };
        self.languageindex = -1;
        let descs = db.descriptions();
        // for(i=0;i<description.size();++i)
        let mut i: int4 = 0;
        while (i as usize) < descs.len() {
            if descs[i as usize].get_id() == baseid {
                self.languageindex = i;
                if descs[i as usize].is_deprecated() {
                    self.print_message(&format!("WARNING: Language {baseid} is deprecated"));
                }
                break;
            }
            i += 1;
        }
        if self.languageindex == -1 {
            return Err(KunaError::lowlevel(format!("No sleigh specification for {baseid}")));
        }
        Ok(())
    }

    /// \brief Test if the last `Translate` object can be reused (C++
    /// `SleighArchitecture::isTranslateReused`, `sleigh_arch.cc:168`).
    ///
    /// Always `false` here: the C++ `translators` reuse cache is not ported (see
    /// module docs / LOSS), so every build decodes a fresh `.sla`.  Faithful for
    /// output; only build speed differs.
    pub fn is_translate_reused(&self) -> bool {
        false
    }

    /// \brief Given a specific language, find the relevant spec files (C++
    /// `SleighArchitecture::buildSpecFile`, `sleigh_arch.cc:354`).
    ///
    /// Resolves the pspec/cspec/sla file paths from the active language record
    /// and the compiler field of `archid`.  The C++ also opens+registers the
    /// pspec/cspec into the `DocumentStorage`; that registration is the caller's
    /// job (it owns the `DocumentStorage` and the W6/W8 decode that consumes the
    /// tags), so this returns the resolved paths in [`BuildSpecFiles`].
    pub fn build_spec_file(&self, db: &LanguageDatabase) -> KunaResult<BuildSpecFiles> {
        let language_reuse = self.is_translate_reused();
        let language = &db.descriptions()[self.languageindex as usize];
        // string compiler = archid.substr(archid.rfind(':')+1);
        let compiler = match self.archid.rfind(':') {
            Some(pos) => &self.archid[pos + 1..],
            None => self.archid.as_str(),
        };
        let compilertag = language.get_compiler(compiler);

        let mut processorfile = String::new();
        let mut compilerfile = String::new();
        let mut slafile = String::new();

        db.specpaths.find_file(&mut processorfile, language.get_processor_spec());
        db.specpaths.find_file(&mut compilerfile, compilertag.get_spec());
        if !language_reuse {
            db.specpaths.find_file(&mut slafile, language.get_sla_file());
            if slafile.is_empty() {
                return Err(KunaError::sleigh(format!(
                    "Could not find .sla file for {}",
                    self.archid
                )));
            }
        }
        Ok(BuildSpecFiles {
            processorfile,
            compilerfile,
            slafile: if slafile.is_empty() { None } else { Some(slafile) },
            language_reuse,
        })
    }

    /// \brief Build a sleigh translator (C++
    /// `SleighArchitecture::buildTranslator`, `sleigh_arch.cc:174`).
    ///
    /// Decodes the resolved `.sla` bytes into a [`Sleigh`] over the given
    /// `loader`, then constructs the owning [`Architecture`] base.  In C++ this
    /// `reset(loader,context)`s a cached or fresh `Sleigh`; here the cache is not
    /// ported, so it always decodes a fresh engine.  Per-fixture context defaults
    /// (the `.pspec`'s `<context_data>` paints, which W4 `buildContext` seams)
    /// can be applied to the returned base's `translate` after the `.sla`
    /// registers the variables.
    ///
    /// `sla_bytes` is the resolved `.sla` file content (the caller reads it from
    /// the `slafile` path of [`Self::build_spec_file`]).
    pub fn build_translator(
        &mut self,
        loader: Box<dyn LoadImage>,
        sla_bytes: &[u8],
    ) -> KunaResult<()> {
        let context = Box::new(ContextInternal::new());
        let mut sleigh = Sleigh::new(loader, context);
        sleigh.initialize_from_sla(sla_bytes)?;
        let archid = self.archid.clone();
        self.base = Some(Architecture::new(&archid, sleigh));
        Ok(())
    }

    /// \brief Encode basic attributes of the active executable (C++
    /// `SleighArchitecture::encodeHeader`, `sleigh_arch.cc:464`).
    ///
    /// Writes the `name` (filename) and `target` attributes.  STUB(W*-encode):
    /// the full `Encoder` marshaling surface lands with the marshaling waves; the
    /// header is the slice the leaf-frontend `encode` paths prepend.  Returns the
    /// two attribute pairs the C++ would write, in C++ order.
    pub fn encode_header(&self) -> Vec<(AttributeId, String)> {
        // encoder.writeString(ATTRIB_NAME, filename);
        // encoder.writeString(ATTRIB_TARGET, target);
        vec![(ATTRIB_NAME, self.filename.clone()), (ATTRIB_TARGET, self.target.clone())]
    }

    /// \brief Restore from basic attributes of an executable (C++
    /// `SleighArchitecture::restoreXmlHeader`, `sleigh_arch.cc:472`).
    ///
    /// Reads the `name` and `target` attributes off the root element of a save
    /// file.  Takes the two attribute values directly (the leaf-frontend
    /// `restoreXml` paths supply them off the `<*_savefile>` root element).
    pub fn restore_xml_header(&mut self, name: &str, target: &str) {
        self.filename = name.to_string();
        self.target = target.to_string();
    }
}

/// The spec-file paths resolved by [`SleighArchitecture::build_spec_file`].
#[derive(Debug, Clone)]
pub struct BuildSpecFiles {
    /// The resolved `.pspec` path (C++ `processorfile`).
    pub processorfile: String,
    /// The resolved `.cspec` path (C++ `compilerfile`).
    pub compilerfile: String,
    /// The resolved `.sla` path, or `None` when translator reuse is active
    /// (C++ `slafile`, empty when reusing).
    pub slafile: Option<String>,
    /// Whether the translator is being reused (C++ `language_reuse`).
    pub language_reuse: bool,
}

// ---------------------------------------------------------------------------
// Language-id normalization (sleigh_arch.cc:483-565)
// ---------------------------------------------------------------------------

/// \brief Try to recover a \e language \e id processor field (C++
/// `SleighArchitecture::normalizeProcessor`, `sleigh_arch.cc:483`).
pub fn normalize_processor(nm: &str) -> String {
    if nm.contains("386") {
        return "x86".to_string();
    }
    nm.to_string()
}

/// \brief Try to recover a \e language \e id endianness field (C++
/// `SleighArchitecture::normalizeEndian`, `sleigh_arch.cc:495`).
pub fn normalize_endian(nm: &str) -> String {
    if nm.contains("big") {
        return "BE".to_string();
    }
    if nm.contains("little") {
        return "LE".to_string();
    }
    nm.to_string()
}

/// \brief Try to recover a \e language \e id size field (C++
/// `SleighArchitecture::normalizeSize`, `sleigh_arch.cc:509`).
///
/// Erases the first `"bit"` and the first `'-'` (C++ erases each once, in that
/// order).
pub fn normalize_size(nm: &str) -> String {
    let mut res = nm.to_string();
    // pos = res.find("bit"); if (pos != npos) res.erase(pos,3);
    if let Some(pos) = res.find("bit") {
        res.replace_range(pos..pos + 3, "");
    }
    // pos = res.find('-'); if (pos != npos) res.erase(pos,1);
    if let Some(pos) = res.find('-') {
        res.replace_range(pos..pos + 1, "");
    }
    res
}

/// \brief Normalize the target string into a valid \e language \e id (C++
/// `SleighArchitecture::normalizeArchitecture`, `sleigh_arch.cc:529`).
///
/// The target must already look like a \e language \e id (4 or 5 colon-separated
/// fields), but may drop the compiler field and be a little sloppier in format.
/// Errors if it does not have 4 or 5 fields, exactly as the C++ throws
/// `LowlevelError`.
pub fn normalize_architecture(nm: &str) -> KunaResult<String> {
    // string::size_type pos[4]; int4 i; curpos=0;
    // for(i=0;i<4;++i){ curpos=nm.find(':',curpos+1); if(npos) break; pos[i]=curpos; }
    let bytes = nm.as_bytes();
    let mut pos = [0usize; 4];
    let mut i: usize = 0;
    let mut curpos: usize = 0;
    while i < 4 {
        // nm.find(':', curpos+1): search starting at curpos+1.  On the first
        // iteration curpos is 0, so the search starts at index 1 (matching the
        // C++ `find(':',curpos+1)` with curpos==0 -> start at 1).
        let start = curpos + 1;
        let found = if start <= bytes.len() {
            bytes[start..].iter().position(|&c| c == b':').map(|p| start + p)
        } else {
            None
        };
        match found {
            Some(p) => {
                curpos = p;
                pos[i] = curpos;
            }
            None => break,
        }
        i += 1;
    }
    // if ((i!=3)&&(i!=4)) throw LowlevelError(...);
    if i != 3 && i != 4 {
        return Err(KunaError::lowlevel(format!(
            "Architecture string does not look like sleigh id: {nm}"
        )));
    }
    // processor = nm.substr(0,pos[0]);
    let processor = &nm[..pos[0]];
    // endian = nm.substr(pos[0]+1,pos[1]-pos[0]-1);
    let endian = &nm[pos[0] + 1..pos[1]];
    // size = nm.substr(pos[1]+1,pos[2]-pos[1]-1);
    let size = &nm[pos[1] + 1..pos[2]];

    let (variant, compile): (String, String) = if i == 4 {
        // variant = nm.substr(pos[2]+1,pos[3]-pos[2]-1); compile = nm.substr(pos[3]+1);
        (nm[pos[2] + 1..pos[3]].to_string(), nm[pos[3] + 1..].to_string())
    } else {
        // variant = nm.substr(pos[2]+1); compile = "default";
        (nm[pos[2] + 1..].to_string(), "default".to_string())
    };

    let processor = normalize_processor(processor);
    let endian = normalize_endian(endian);
    let size = normalize_size(size);
    Ok(format!("{processor}:{endian}:{size}:{variant}:{compile}"))
}

// ---------------------------------------------------------------------------
// helpers
// ---------------------------------------------------------------------------

/// Read a string attribute as a Rust `String` (the `.ldefs` attributes are all
/// ASCII; the decoder yields bytes — `from_utf8_lossy` mirrors the C++ `string`
/// which carries raw bytes, but the spec tree is ASCII so this is lossless).
fn read_string(decoder: &mut dyn Decoder, attrib: &AttributeId) -> KunaResult<String> {
    Ok(String::from_utf8_lossy(&decoder.read_string_id(attrib)?).into_owned())
}

#[cfg(test)]
mod tests;

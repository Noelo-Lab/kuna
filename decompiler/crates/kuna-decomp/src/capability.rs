//! Port of `decompiler/cpp/capability.{cc,hh}` and the
//! `ArchitectureCapability` registry from `architecture.{cc,hh}` (item
//! `w4-fw-architecture`).
//!
//! ## The C++ static-init dance, ported per the W1 IdRegistry convention
//!
//! The C++ `CapabilityPoint` uses the C++ static-initializer feature to
//! \e auto-discover extension points: each extension is a global singleton whose
//! constructor `push_back`s `this` into a function-local static vector
//! (`CapabilityPoint::getList`).  `CapabilityPoint::initializeAll` (called from
//! `startDecompilerLibrary`) then walks that list, calls `initialize()` on each,
//! and clears it; `ArchitectureCapability::initialize` re-files the capability
//! into its own `thelist`.
//!
//! Per ADR 0006 / the W1 `IdRegistry` convention, kuna does **not** rely on
//! static constructors (they are nondeterministic across translation units and
//! invisible to the build).  Instead the registry is an explicit value
//! ([`CapabilityRegistry`]) that a caller populates by calling [`register`]
//! (the analogue of constructing a singleton) in a deterministic order; the
//! [`start_decompiler_library`] flow then calls
//! [`CapabilityRegistry::initialize_all`] (= `initializeAll` +
//! `ArchitectureCapability::sortCapabilities`).
//!
//! [`register`]: CapabilityRegistry::register
//! [`start_decompiler_library`]: crate::libdecomp::start_decompiler_library
//!
//! Only the [`ArchitectureCapability`] flavor of capability is reachable from
//! the ported surface (the print-language / test capabilities live in their own
//! waves), so this module models exactly that: a registry of named
//! architecture-building extensions plus the `findCapability`/`getCapability`/
//! `sortCapabilities` lookups that `libdecomp.cc` and the `xml_arch`/`raw_arch`
//! capabilities use.

use kuna_base::types::uint4;

/// Current \e major version of the decompiler (C++
/// `ArchitectureCapability::majorversion`, `architecture.cc:36`).
pub const MAJOR_VERSION: uint4 = 6;
/// Current \e minor version of the decompiler (C++
/// `ArchitectureCapability::minorversion`, `architecture.cc:37`).
pub const MINOR_VERSION: uint4 = 1;

/// \brief Abstract extension point for building `Architecture` objects (C++
/// `ArchitectureCapability : CapabilityPoint`, `architecture.hh:117`).
///
/// The C++ class is abstract: `buildArchitecture`/`isFileMatch`/`isXmlMatch` are
/// pure-virtual and supplied by `xml_arch`/`raw_arch`/`sleigh_arch` (later
/// items).  This trait names that surface so the registry can route a file/XML
/// document to the capability that handles it.  `buildArchitecture` is left to
/// the deriving capability waves (it needs the full loader/`DocumentStorage`
/// surface), so it is intentionally absent here; the registry only needs the
/// match predicates and the name.
pub trait ArchitectureCapability {
    /// Get the capability identifier (C++ `getName`).
    fn get_name(&self) -> &str;

    /// Determine if this extension can handle the file at `filename`
    /// (C++ pure-virtual `isFileMatch`).
    fn is_file_match(&self, filename: &str) -> bool;

    /// Determine if this extension can handle a parsed XML document whose root
    /// element is named `root_name` (C++ pure-virtual `isXmlMatch(Document*)`,
    /// reduced to the root-element name the matchers actually test).
    fn is_xml_match(&self, root_name: &str) -> bool;

    /// Do specialized initialization (C++ `ArchitectureCapability::initialize`).
    ///
    /// In C++ this re-files the capability into `thelist`; in the Rust registry
    /// the filing IS the [`CapabilityRegistry::register`] call, so the default
    /// is a no-op (deriving capabilities may still override for genuine
    /// per-capability set-up).
    fn initialize(&mut self) {}
}

/// \brief The explicit analogue of `CapabilityPoint::getList()` +
/// `ArchitectureCapability::thelist`.
///
/// Holds the registered architecture capabilities in registration order and
/// supplies the `findCapability`/`getCapability`/`sortCapabilities` lookups.
/// Per the W1 convention this is a plain value, populated deterministically by
/// the bootstrap (no static constructors).
#[derive(Default)]
pub struct CapabilityRegistry {
    /// The registered extensions (C++ `ArchitectureCapability::thelist`).
    thelist: Vec<Box<dyn ArchitectureCapability>>,
    /// Whether [`initialize_all`](Self::initialize_all) has run (the C++
    /// `initializeAll` is called exactly once from `startDecompilerLibrary`).
    initialized: bool,
}

impl std::fmt::Debug for CapabilityRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CapabilityRegistry")
            .field(
                "names",
                &self.thelist.iter().map(|c| c.get_name().to_string()).collect::<Vec<_>>(),
            )
            .field("initialized", &self.initialized)
            .finish()
    }
}

impl CapabilityRegistry {
    /// Construct an empty registry.
    pub fn new() -> CapabilityRegistry {
        CapabilityRegistry::default()
    }

    /// Register an extension (the deterministic analogue of constructing a
    /// `CapabilityPoint` singleton — C++ `CapabilityPoint::CapabilityPoint`
    /// pushing `this` onto `getList()`).
    pub fn register(&mut self, capability: Box<dyn ArchitectureCapability>) {
        self.thelist.push(capability);
    }

    /// Number of registered capabilities.
    pub fn len(&self) -> usize {
        self.thelist.len()
    }

    /// Whether no capability is registered.
    pub fn is_empty(&self) -> bool {
        self.thelist.is_empty()
    }

    /// `true` once [`initialize_all`](Self::initialize_all) has run.
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    /// Finish initialization for all extension points, then order them
    /// (C++ `CapabilityPoint::initializeAll` + `sortCapabilities`, run together
    /// by `startDecompilerLibrary`).
    pub fn initialize_all(&mut self) {
        // C++ initializeAll: walk and `initialize()` each, then clear the
        // CapabilityPoint list (the ArchitectureCapabilities re-file into
        // thelist via initialize()).  Here the list IS thelist, so we just call
        // initialize() on each in registration order.
        for cap in self.thelist.iter_mut() {
            cap.initialize();
        }
        self.sort_capabilities();
        self.initialized = true;
    }

    /// Modify the order extensions are searched so that the "raw" architecture
    /// comes last (C++ `ArchitectureCapability::sortCapabilities`,
    /// `architecture.cc:135`).  Right now that is the only ordering need.
    pub fn sort_capabilities(&mut self) {
        // Find the "raw" capability (C++ for-loop until name == "raw").
        let mut i = 0usize;
        while i < self.thelist.len() {
            if self.thelist[i].get_name() == "raw" {
                break;
            }
            i += 1;
        }
        if i == self.thelist.len() {
            return;
        }
        // Move thelist[i] to the end, sliding the others down by one (a stable
        // rotation, exactly mirroring the C++ shift-and-append).
        let capa = self.thelist.remove(i);
        self.thelist.push(capa);
    }

    /// Find an extension to process the file at `filename`
    /// (C++ `findCapability(const string&)`), or `None`.
    pub fn find_capability_by_file(&self, filename: &str) -> Option<&dyn ArchitectureCapability> {
        for capa in self.thelist.iter() {
            if capa.is_file_match(filename) {
                return Some(capa.as_ref());
            }
        }
        None
    }

    /// Find an extension to process an XML document whose root element is named
    /// `root_name` (C++ `findCapability(Document*)`), or `None`.
    pub fn find_capability_by_xml(&self, root_name: &str) -> Option<&dyn ArchitectureCapability> {
        for capa in self.thelist.iter() {
            if capa.is_xml_match(root_name) {
                return Some(capa.as_ref());
            }
        }
        None
    }

    /// Get a capability by name (C++ `getCapability(const string&)`), or `None`.
    pub fn get_capability(&self, name: &str) -> Option<&dyn ArchitectureCapability> {
        for res in self.thelist.iter() {
            if res.get_name() == name {
                return Some(res.as_ref());
            }
        }
        None
    }

    /// The registered capability names in current (post-sort) order — exposed
    /// for the bootstrap-order tests and for diagnostics.
    pub fn names(&self) -> Vec<&str> {
        self.thelist.iter().map(|c| c.get_name()).collect()
    }
}

/// Get the \e major decompiler version (C++
/// `ArchitectureCapability::getMajorVersion`).
pub fn get_major_version() -> uint4 {
    MAJOR_VERSION
}

/// Get the \e minor decompiler version (C++
/// `ArchitectureCapability::getMinorVersion`).
pub fn get_minor_version() -> uint4 {
    MINOR_VERSION
}

/// A minimal name-only capability used by the bootstrap and tests (the analogue
/// of the C++ singleton extension objects, with the match predicates supplied as
/// data).  Deriving capability waves (`xml_arch`/`raw_arch`/`sleigh_arch`) will
/// provide their own concrete types implementing [`ArchitectureCapability`] with
/// real `buildArchitecture` bodies; until then this lets the registry,
/// `sortCapabilities`, and the lookups be exercised deterministically.
pub struct NamedCapability {
    name: String,
    file_match: bool,
    xml_root: Option<String>,
}

impl NamedCapability {
    /// Construct a named capability that matches files iff `file_match` and
    /// matches an XML document iff its root element name equals `xml_root`.
    pub fn new(name: &str, file_match: bool, xml_root: Option<&str>) -> NamedCapability {
        NamedCapability {
            name: name.to_string(),
            file_match,
            xml_root: xml_root.map(|s| s.to_string()),
        }
    }
}

impl ArchitectureCapability for NamedCapability {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn is_file_match(&self, _filename: &str) -> bool {
        self.file_match
    }
    fn is_xml_match(&self, root_name: &str) -> bool {
        self.xml_root.as_deref() == Some(root_name)
    }
}

#[cfg(test)]
mod tests;

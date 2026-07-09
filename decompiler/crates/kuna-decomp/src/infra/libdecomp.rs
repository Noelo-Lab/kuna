//! Port of `decompiler/cpp/libdecomp.{cc,hh}` (item `w4-fw-architecture`): the
//! library bootstrap, `startDecompilerLibrary` and `shutdownDecompilerLibrary`.
//!
//! Per the W1 `IdRegistry` convention there is **no** global mutable id table in
//! the Rust port: the attribute/element ids are values, and a caller builds an
//! [`IdRegistry`] (via `with_base_ids` + the per-module `register_ids`) instead
//! of relying on static initialization.  So the Rust bootstrap returns the
//! [`DecompilerLibrary`] state (the id registry + the capability registry) that
//! the C++ leaves in globals, with the exact same ordering:
//!
//! 1. build the id registry (= `AttributeId::initialize()` + `ElementId::initialize()`),
//! 2. [`CapabilityRegistry::initialize_all`] (= `initializeAll` + `sortCapabilities`).
//!
//! The sleigh-home / extra-spec-path scan (`SleighArchitecture::scanForSleighDirectories`
//! / `specpaths.addDir2Path`) lives in the `sleigh_arch` item; here the spec
//! search paths are recorded on the returned state for that item to consume.

use kuna_base::marshal::IdRegistry;

use crate::capability::CapabilityRegistry;
use crate::pcodeinject;

/// \brief The bootstrapped decompiler-library state.
///
/// The C++ `startDecompilerLibrary` leaves its results in process globals (the
/// attribute/element id tables and `ArchitectureCapability::thelist`); the Rust
/// port returns them in this value so they are explicit and per-instance.
#[derive(Debug, Default)]
pub struct DecompilerLibrary {
    /// The marshaling id registry (C++ `AttributeId`/`ElementId` global tables).
    pub registry: IdRegistry,
    /// The architecture-capability registry (C++ `ArchitectureCapability::thelist`).
    pub capabilities: CapabilityRegistry,
    /// Extra directories to search for `.lspec`/`.ldefs` spec files
    /// (C++ `SleighArchitecture::specpaths`).  Consumed by the `sleigh_arch`
    /// scan, which is its own item.
    pub spec_paths: Vec<String>,
}

/// Build the marshaling [`IdRegistry`] the C++ `AttributeId::initialize()` /
/// `ElementId::initialize()` calls would populate as globals.
///
/// Registers the base ids plus every ported module's marshaling ids (currently
/// the p-code injection ids; further modules add their `register_ids` here as
/// they land — the C++ analog is each translation unit's static `AttributeId`/
/// `ElementId` constructors filing into the global tables).
fn build_id_registry() -> IdRegistry {
    let mut registry = IdRegistry::with_base_ids();
    pcodeinject::register_ids(&mut registry);
    registry
}

/// Initialize all decompiler capabilities and register any sleigh specifications
/// (C++ `startDecompilerLibrary(const char *sleighhome)`).
///
/// `sleighhome` is the optional ghidra-distribution root; `extra_paths` are extra
/// directories containing `.lspec` files (C++ overloads combine into the same
/// init order).  The caller pre-populates `capabilities` (the explicit analogue
/// of constructing the `CapabilityPoint` singletons); this finishes their
/// initialization and orders them.
///
/// Returns the bootstrapped [`DecompilerLibrary`] state.
pub fn start_decompiler_library(
    sleighhome: Option<&str>,
    extra_paths: &[String],
    mut capabilities: CapabilityRegistry,
) -> DecompilerLibrary {
    let registry = build_id_registry();
    capabilities.initialize_all();

    let mut spec_paths: Vec<String> = Vec::new();
    if let Some(home) = sleighhome {
        // C++ scanForSleighDirectories walks `home`; the scan itself is the
        // sleigh_arch item, so the root is recorded as a search root.
        spec_paths.push(home.to_string());
    }
    for p in extra_paths {
        spec_paths.push(p.clone());
    }

    DecompilerLibrary { registry, capabilities, spec_paths }
}

/// Release library resources (C++ `shutdownDecompilerLibrary`).
///
/// The C++ body is empty (the global tables are torn down by the runtime); the
/// Rust state is owned by the caller and dropped normally, so this consumes it.
pub fn shutdown_decompiler_library(_lib: DecompilerLibrary) {}

#[cfg(test)]
mod tests;

//! Port of `decompiler/cpp/kuna_dynamichashmax.{cc,hh}` — the dynamic-hash
//! collision-budget sub-stage (kuna GH-8467, S6 alias-facets / dynamic-symbol
//! attachment).
//!
//! ## What this corner controls
//!
//! When a HighVariable needs a storage-independent identity (a \e dynamic
//! Symbol keyed on a hash of its local data-flow),
//! [`DynamicHash::unique_hash`](crate::dynamic::DynamicHash::unique_hash)
//! cycles through hash methods and keeps the first whose same-address collision
//! list has size `<= maxduplicates` (hard-coded 8 upstream, `dynamic.cc:433`
//! and `:494`).  On dense, highly-repetitive code -- a fully unrolled AArch64
//! NEON byte-search loop -- more than 8 structurally identical Varnodes share
//! the same address with colliding hashes, so every method's collision list
//! overflows 8, the champion stays empty, and `uniqueHash` returns `hash == 0`;
//! `buildDynamicSymbol` then throws `RecovError("Unable to find unique hash for
//! varnode")` and the function aborts (GH-8467).
//!
//! ## The option
//!
//! The decision is a P0 assertion: `option dynamichashmax on|off`.  Default
//! `off` = upstream collision budget 8, byte-identical output.  When `on`, the
//! budget rises to 16 -- large enough to admit a unique hash for these dense
//! cases while still fitting the existing 3-bit total/position encoding (16 is
//! the exact ceiling of that range).  `unique_hash` reads the live flag
//! [`Architecture::dynamic_hash_maxdup_high`](crate::architecture::Architecture).
//!
//! ## What this port covers (and the seam it cannot)
//!
//! The whole upstream `kuna_dynamichashmax.cc` IS the option class
//! `OptionDynamicHashMax::apply`, which parses on/off and writes
//! `glb->dynamic_hash_maxdup_high`.  This module ports that decision body as
//! [`DynamicHashMaxOption`].  Wiring a new `ArchOption` subclass into the option
//! database / console dispatch is a later wave (the `options.rs`/console surface
//! this pack does not own); the live flag is already threaded through
//! [`Architecture::dynamic_hash_maxdup_high`].  Noted in the structured losses.

/// (kuna GH-8467) The dynamic-hash same-address collision budget toggle
/// (C++ `OptionDynamicHashMax`).
///
/// `enabled == false` (default) keeps the upstream budget of 8 (byte-identical);
/// `enabled == true` raises it to 16.  The resolved value is written into
/// [`Architecture::dynamic_hash_maxdup_high`](crate::architecture::Architecture),
/// which [`DynamicHash::unique_hash`](crate::dynamic::DynamicHash::unique_hash)
/// reads.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DynamicHashMaxOption {
    /// True when the wide (16) collision budget is in effect
    /// (C++ `Architecture::dynamic_hash_maxdup_high`).
    pub enabled: bool,
}

impl Default for DynamicHashMaxOption {
    /// Shipped default for *this option object* is `off` (upstream budget 8).
    ///
    /// Note the live arch flag's runtime default diverges (DIV-3, GH-8467:
    /// `architecture.rs` flips `dynamic_hash_maxdup_high` on by default after
    /// `restore_from_spec`); this object models only the `OptionDynamicHashMax`
    /// decision body, whose neutral state is `off`.
    fn default() -> Self {
        DynamicHashMaxOption { enabled: false }
    }
}

impl DynamicHashMaxOption {
    /// Apply the option (C++ `OptionDynamicHashMax::apply`):
    /// ```text
    ///   bool val = onOrOff(p1);
    ///   glb->dynamic_hash_maxdup_high = val;
    ///   string prop = val ? "on" : "off";
    ///   return "DynamicHash wide collision budget turned "+prop;
    /// ```
    /// The caller writes `enabled` into
    /// [`Architecture::dynamic_hash_maxdup_high`](crate::architecture::Architecture).
    pub fn apply(&mut self, val: bool) -> String {
        self.enabled = val;
        let prop = if val { "on" } else { "off" };
        format!("DynamicHash wide collision budget turned {prop}")
    }

    /// The same-address collision budget this option selects: 8 when off, 16
    /// when on (the `maxduplicates` value read in `DynamicHash::unique_hash`,
    /// `dynamic.cc:433`/`:503`).
    pub fn max_duplicates(&self) -> u32 {
        if self.enabled {
            16
        } else {
            8
        }
    }

    /// Read the gate (C++ `glb->dynamic_hash_maxdup_high`).
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

#[cfg(test)]
#[path = "kuna_dynamichashmax/tests.rs"]
mod tests;

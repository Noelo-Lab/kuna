//! Port of `decompiler/cpp/kuna_stackalias.{cc,hh}` — stack-alias dead-store
//! ordering (kuna GH-8500, S6 alias-facets / heritage-ordering).
//!
//! When a store-through-a-stack-pointer-alias is dead-code-eliminated one round
//! before the aliasing LOAD that reads it resolves, the LOAD re-heritages an
//! uninitialized stack input.  When `option stackalias on`,
//! `ActionDeadCode::lastChanceLoad` also recognizes a LOAD whose pointer lives in
//! the stack address space and conservatively holds the function's stack stores
//! alive for that deadcode round.
//!
//! ## What this module owns vs. ANCHOR-EDIT(coreaction.cc)
//!
//! The assigned C++ scope (`kuna_stackalias.cc/.hh`) contains **only** the
//! `ElementId` and the `ArchOption` that sets the arch flag
//! `stack_alias_deadstore` — *not* the behavioral fix.  As the C++ header states,
//! "This is an ANCHOR-EDIT fix (no new Rule): the guarded logic lives in
//! `ActionDeadCode::lastChanceLoad` (coreaction.cc), reached only when the arch
//! flag `stack_alias_deadstore` is set."  Only the gate
//! ([`Architecture::stack_alias_deadstore`](crate::architecture::Architecture),
//! default off) is owned here.
//!
//! ### ANCHOR-EDIT for W8 / the W-coreaction porter
//!
//! `ActionDeadCode::lastChanceLoad` (an upstream `coreaction.cc` method) gets a
//! kuna anchor edit: when `glb->stack_alias_deadstore` is set, a LOAD whose
//! pointer is in the stack address space (a pointer-to-stack alias) also defers
//! dead-store removal for one round.  That edit lands with the `ActionDeadCode`
//! port (`coreaction.cc`, the W-coreaction wave), gated on
//! [`StackAliasOption`].  See `UPSTREAM.md` *Divergence*; this module supplies
//! only the toggle so the option surface and its ElementId exist.

/// (kuna GH-8500) Toggle preservation of a store-through-a-stack-pointer-alias
/// across the deadcode race (C++ `OptionStackAlias`).
///
/// ANCHOR-EDIT(coreaction.cc): the C++ `OptionStackAlias::apply` flips
/// `glb->stack_alias_deadstore`; here the flag is carried as a plain `bool` whose
/// [`Default`] is the *shipped* default (`option stackalias off`, i.e. \b false —
/// upstream byte-identical; `architecture.cc:1440` sets `stack_alias_deadstore =
/// false`).  `ActionDeadCode::lastChanceLoad` reads this gate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StackAliasOption {
    /// True if a store-through-a-stack-pointer-alias is held across the deadcode
    /// race (C++ `Architecture::stack_alias_deadstore`).
    pub enabled: bool,
}

impl Default for StackAliasOption {
    /// Shipped default: `option stackalias off` (upstream byte-identical;
    /// `architecture.cc` sets `stack_alias_deadstore = false`).
    fn default() -> Self {
        StackAliasOption { enabled: false }
    }
}

impl StackAliasOption {
    /// (kuna) Set the gate (C++ `OptionStackAlias::apply`).
    pub fn apply(&mut self, val: bool) -> &'static str {
        self.enabled = val;
        if val {
            "Stack-alias dead-store preservation turned on"
        } else {
            "Stack-alias dead-store preservation turned off"
        }
    }

    /// Read the gate (C++ `glb->stack_alias_deadstore`).
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

#[cfg(test)]
#[path = "kuna_stackalias/tests.rs"]
mod tests;

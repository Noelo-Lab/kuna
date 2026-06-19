//! WS3 -- the compiler-side p-code build actions.
//!
//! The generic [`kuna_sleigh::pcodecompile::PcodeCompile`] trait (the `createOp`
//! / `createLoad` / `assignBitRange` / ... machinery from `pcodecompile.hh`) is
//! **already fully ported** in `kuna-sleigh/src/pcodecompile.rs` -- it was needed
//! for the runtime `parse line` path.  WS3 supplies the *compiler-specific*
//! pieces that `pcodecompile.hh`/`slgh_compile.hh` add on top:
//!
//! - [`SleighPcode`]: the concrete `PcodeCompile` implementation used by the
//!   compiler (`class SleighPcode : public PcodeCompile`, slgh_compile.hh:282-292).
//!   It overrides the five abstract hooks (`allocateTemp`, `getLocation`,
//!   `reportError`, `reportWarning`, `addSymbol`) to route them into
//!   [`crate::slgh_compile::SleighCompile`] (slgh_compile.cc:1930-1958).
//! - [`MacroBuilder`]: expands a `macro` directive's `OpTpl` list with parameter
//!   substitution (`class MacroBuilder : public PcodeBuilder`,
//!   slgh_compile.hh:256-275; slgh_compile.cc:1785-1928).
//!
//! ## Module ownership: WS3 owns this file exclusively.

#![allow(dead_code)]

use kuna_base::error::KunaResult;
use kuna_sleigh::pcodecompile::Location;
use kuna_sleigh::semantics::{HandleTpl, OpTpl};

/// The compiler's concrete p-code compiler (`SleighPcode`, slgh_compile.hh:282).
///
/// In C++ this *is-a* `PcodeCompile` and holds a back-pointer to the
/// `SleighCompile`.  In the Rust port, `SleighCompile` owns the `PcodeCompile`
/// state and the abstract hooks are dispatched through it; this struct carries
/// the per-section state the compiler needs (temp allocation base, label count)
/// and the symbols collected during a section's parse.  The exact field set is
/// fixed up in WS3/WS4 once the `SleighCompile <-> SleighPcode` ownership split
/// is settled -- the interface freeze here is the *method set*.
#[derive(Default)]
pub struct SleighPcode {
    /// Next free unique-space (temporary) offset; bumped by `allocateTemp`.
    pub unique_base: u32,
    /// Number of labels in the current constructor (`local_labelcount`).
    pub local_labelcount: u32,
    /// Whether the `local` keyword is required for temporaries (`enforceLocalKey`).
    pub enforce_local_key: bool,
}

impl SleighPcode {
    /// Construct an empty p-code compiler (`SleighPcode::SleighPcode`,
    /// slgh_compile.hh:290).
    pub fn new() -> SleighPcode {
        SleighPcode::default()
    }

    /// Allocate the next temporary register offset (`SleighPcode::allocateTemp`,
    /// slgh_compile.cc:1930) -- routes to `SleighCompile::getUniqueAddr`.
    pub fn allocate_temp(&mut self) -> u32 {
        todo!("WS3: SleighPcode::allocateTemp (slgh_compile.cc:1930-1934)")
    }

    /// Look up a symbol's defining location (`SleighPcode::getLocation`,
    /// slgh_compile.cc:1936).
    pub fn get_location(&self, _symbol_name: &[u8]) -> Option<Location> {
        todo!("WS3: SleighPcode::getLocation (slgh_compile.cc:1936-1940)")
    }
}

/// Expands a `macro` directive's body into a caller's `OpTpl` list, substituting
/// the call-site arguments for the macro's formal parameters
/// (`MacroBuilder`, slgh_compile.hh:256-275; bodies slgh_compile.cc:1785-1928).
///
/// In C++ this derives from `PcodeBuilder` and overrides `dump`/`appendBuild`/
/// `delaySlot`/`setLabel`/`appendCrossBuild` so that, instead of emitting raw
/// p-code, it *clones* the macro's `OpTpl`s (with parameter handles swapped) into
/// `outvec`.  WS3 ports the build/transfer logic; the `PcodeBuilder` trait it
/// implements lives in `kuna_sleigh::semantics`.
pub struct MacroBuilder<'a> {
    /// The partial op list to expand the macro into (`outvec`).
    pub outvec: &'a mut Vec<OpTpl>,
    /// Parameter handles to substitute (`params`).
    pub params: Vec<HandleTpl>,
    /// Set true if expansion hit an error (`haserror`).
    pub haserror: bool,
}

impl<'a> MacroBuilder<'a> {
    /// Construct a builder targeting `outvec`, with `labelbase` label offset
    /// (`MacroBuilder::MacroBuilder`, slgh_compile.hh:266).
    pub fn new(outvec: &'a mut Vec<OpTpl>, _labelbase: u32) -> MacroBuilder<'a> {
        MacroBuilder {
            outvec,
            params: Vec::new(),
            haserror: false,
        }
    }

    /// Establish the `MACRO` directive op to expand (`setMacroOp`,
    /// slgh_compile.cc:1809).
    pub fn set_macro_op(&mut self, _macroop: &OpTpl) -> KunaResult<()> {
        todo!("WS3: MacroBuilder::setMacroOp (slgh_compile.cc:1809-1831)")
    }

    /// Clone one op template into `outvec`, substituting parameter handles
    /// (`transferOp`, slgh_compile.cc:1833).
    fn transfer_op(&mut self, _op: &OpTpl, _params: &[HandleTpl]) -> KunaResult<bool> {
        todo!("WS3: MacroBuilder::transferOp (slgh_compile.cc:1833-1885)")
    }

    /// The `PcodeBuilder::dump` override: append (a clone of) `op` to `outvec`
    /// (`MacroBuilder::dump`, slgh_compile.cc:1886).
    pub fn dump(&mut self, _op: &OpTpl) -> KunaResult<()> {
        todo!("WS3: MacroBuilder::dump (slgh_compile.cc:1886-1911)")
    }

    /// Return whether expansion encountered an error (`hasError`).
    pub fn has_error(&self) -> bool {
        self.haserror
    }
}

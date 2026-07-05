//! The `Architecture`↔translator seam: the [`EngineTranslate`] trait.
//!
//! [`Architecture`](crate::architecture::Architecture) owns its disassembly
//! engine behind a `Box<dyn EngineTranslate>` trait object rather than a
//! concrete [`Sleigh`], so a future ghidra-mode translator (p-code fetched
//! from the Java host per instruction, no local `.sla`) can be installed in
//! its place without `kuna-decomp` naming any ghidra-specific type — the
//! ghidra translator lives in the downstream `kuna-ghidra` crate, so an enum
//! variant would be a dependency cycle, but a trait object owned here is not.
//! See `docs/rust-port/ghidra-phase2-plan.md` §2.2.
//!
//! The trait extends [`Translate`] (already `: RegisterLookup`), adding only
//! the handful of engine-owned surfaces `Architecture` and its external
//! callers reach beyond the abstract `Translate` API: the address-space
//! manager handles, the load image, the context database, and the
//! register-storage lookup.  Everything the callers reach through
//! `Translate`/`RegisterLookup` (endianness, float formats, `oneInstruction`,
//! register *names*, …) needs no new method here.
//!
//! Only [`Sleigh`] implements it today; the implementation forwards to the
//! engine's inherent methods, so routing an `Architecture` call through the
//! trait object is behavior-identical to the former direct `Sleigh` call.

use std::cell::RefCell;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_base::space::AddrSpaceManager;
use kuna_num::pcoderaw::VarnodeData;
use kuna_sleigh::globalcontext::ContextDatabase;
use kuna_sleigh::loadimage::LoadImage;
use kuna_sleigh::sleigh::Sleigh;
use kuna_sleigh::translate::Translate;

/// The seam between [`Architecture`](crate::architecture::Architecture) and
/// its disassembly engine (see the module docs).
pub trait EngineTranslate: Translate {
    /// Borrow the engine's address-space manager (C++ the `Architecture`
    /// IS-A `AddrSpaceManager`; in the Rust port it lives inside the engine
    /// and `Architecture::manage()` forwards here).
    fn manager(&self) -> &AddrSpaceManager;

    /// Share the engine's single address-space manager (LOSS-132): the `Rc`
    /// the lift populated, handed to `glb` so every subsystem keys state by
    /// the same space identities/indices.
    fn manager_rc(&self) -> Rc<AddrSpaceManager>;

    /// Mutably borrow the engine's address-space manager — used during init
    /// to insert the analysis-only fspec/iop/join spaces (C++
    /// `Architecture::restoreFromSpec`).
    fn manager_mut(&mut self) -> &mut AddrSpaceManager;

    /// Share the program load image (C++ `Architecture::loader`); the `Rc`
    /// aliases the engine's loader, so a shared handle stays current across
    /// `set_loader`.
    fn loader_rc(&self) -> Rc<RefCell<Box<dyn LoadImage>>>;

    /// Replace the load image (C++ `restoreFromSpec` hands the loader to the
    /// translator once the `.sla` is decoded).
    fn set_loader(&mut self, loader: Box<dyn LoadImage>);

    /// Read a Varnode's-worth of bytes from the load image at `addr`, masked
    /// to `sz` (C++ `Emulate*::getLoadImageValue`); drives jump-table LOAD
    /// emulation.
    fn read_loadimage_value(&self, addr: &Address, sz: i32) -> KunaResult<u64>;

    /// Resolve a register by name to its [`VarnodeData`] storage (C++
    /// `Translate::getRegister(name)`).
    fn get_register_varnode(&self, nm: &[u8]) -> KunaResult<VarnodeData>;

    /// Run a closure with mutable access to the engine's [`ContextDatabase`]
    /// (C++ `glb->context`).  The object-safe form of
    /// [`Sleigh::with_context_db_mut`]: it takes `&mut dyn FnMut` (rather than
    /// a generic closure) so the method stays callable on a trait object;
    /// [`Architecture::with_context_db_mut`](crate::architecture::Architecture::with_context_db_mut)
    /// keeps the generic, value-returning wrapper over it.
    fn with_context_db_dyn(&self, f: &mut dyn FnMut(&mut dyn ContextDatabase));

    /// Downcast to the concrete standalone [`Sleigh`] engine when this
    /// translator is one (default `None`; the `Sleigh` impl returns `Some`).
    /// The standalone-only analysis / FID / console paths — which never run
    /// on a non-Sleigh engine — use this escape hatch to reach the
    /// Sleigh-specific surface (`SleighBase`, `instruction_mask`,
    /// `install_register_lookup`, …).
    fn as_sleigh(&self) -> Option<&Sleigh> {
        None
    }

    /// Mutable [`EngineTranslate::as_sleigh`].
    fn as_sleigh_mut(&mut self) -> Option<&mut Sleigh> {
        None
    }
}

impl EngineTranslate for Sleigh {
    fn manager(&self) -> &AddrSpaceManager {
        self.base().manager()
    }
    fn manager_rc(&self) -> Rc<AddrSpaceManager> {
        Sleigh::manager_rc(self)
    }
    fn manager_mut(&mut self) -> &mut AddrSpaceManager {
        self.base_mut().manager_mut()
    }
    fn loader_rc(&self) -> Rc<RefCell<Box<dyn LoadImage>>> {
        Sleigh::loader_rc(self)
    }
    fn set_loader(&mut self, loader: Box<dyn LoadImage>) {
        Sleigh::set_loader(self, loader)
    }
    fn read_loadimage_value(&self, addr: &Address, sz: i32) -> KunaResult<u64> {
        Sleigh::read_loadimage_value(self, addr, sz)
    }
    fn get_register_varnode(&self, nm: &[u8]) -> KunaResult<VarnodeData> {
        Sleigh::get_register_varnode(self, nm)
    }
    fn with_context_db_dyn(&self, f: &mut dyn FnMut(&mut dyn ContextDatabase)) {
        self.with_context_db_mut(|db| f(db));
    }
    fn as_sleigh(&self) -> Option<&Sleigh> {
        Some(self)
    }
    fn as_sleigh_mut(&mut self) -> Option<&mut Sleigh> {
        Some(self)
    }
}

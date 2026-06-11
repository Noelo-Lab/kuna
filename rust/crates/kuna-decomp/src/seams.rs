//! Cross-wave seam placeholders for the W3 IR data-model.
//!
//! Per ADR 0001 the IR is three `Funcdata`-owned slotmap arenas keyed by the
//! newtypes [`VarnodeId`], [`OpId`], [`BlockId`].  The keys live here (the one
//! place every W3 serial-chain file can name them); the arenas and the
//! `Funcdata`-mediated mutation API are filled by `funcdata`/`op`/`block`.
//!
//! Everything else in this file is a forward-reference placeholder for a type
//! that `varnode.hh` mentions but that belongs to a later wave, annotated with
//! the wave that fills it.  These let `varnode.rs` transcribe the member layout
//! and link structure faithfully without pulling in the unported subsystems.

use slotmap::new_key_type;

new_key_type! {
    /// Arena key for a `Varnode` (ADR 0001).  Generational, so a stale handle
    /// is a caught error, not a use-after-free.
    pub struct VarnodeId;

    /// Arena key for a `PcodeOp` (ADR 0001).
    ///
    /// SEAM(W3): the `PcodeOp` arena and its accessors are `op`/`funcdata`'s
    /// (`w3-ir-op`).  `varnode.rs` stores `OpId`s for `def` and `descend`
    /// links, exactly as the C++ stores `PcodeOp *`.
    pub struct OpId;

    /// Arena key for a `FlowBlock` (ADR 0001).
    ///
    /// SEAM(W3): filled by `block` (`w3-ir-block`); declared here so the shared
    /// key set is in one place.
    pub struct BlockId;
}

/// HighVariable — the high-level variable an instance of which a Varnode is.
///
/// SEAM(W7): filled by `merge`/`HighVariable` (`w7`).  `Varnode` holds an
/// `Option<HighVariableId>` where the C++ holds a `HighVariable *`; until W7
/// every varnode's high is `None` (the C++ `getHigh()` likewise throws until
/// merging builds it).  Modelled as an opaque arena key so the eventual
/// HighVariable arena can slot in without touching the Varnode layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HighVariableId(pub u32);

/// Cover — the def/use address coverage of a Varnode.
///
/// SEAM(W7): filled by `cover`/`Cover` (`w7`).  The C++ `Varnode::cover` is a
/// lazily-built, mutable `Cover *`; the W3 data-model only needs to track the
/// presence/absence and the `coverdirty` flag (carried in `flags`), so a unit
/// placeholder suffices until W7 supplies the real geometry.
#[derive(Debug, Clone, Default)]
pub struct Cover;

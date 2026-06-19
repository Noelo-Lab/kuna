//! WS5 -- the `.sla` encode/writer orchestration.
//!
//! Most of the writer side is **already ported** in `kuna-sleigh`: the
//! `kuna_sleigh::slaformat::FormatEncode` packed-stream encoder, and the
//! per-object `encode(...)` methods on every `SleighSymbol` subclass, on
//! `SymbolTable` (`slghsymbol.rs` `SymbolTable::encode`), on the pattern types
//! (`slghpattern.rs`), the pattern expressions (`slghpatexpress.rs`), and the
//! semantics templates (`semantics.rs`).  Those were written/exercised by the
//! decoder round-trip and are reused verbatim.
//!
//! What is **missing** is the single top-level orchestrator that the C++ compiler
//! calls at the end of `run_compilation`:
//!
//! - `SleighBase::encode` (sleighbase.cc:226-255): opens `<sleigh>`, writes the
//!   version/endian/align/uniqbase/maxdelay/uniqmask/numsections attributes,
//!   emits the source-file indexer, the `<spaces>` block, then `symtab.encode`.
//! - `SleighBase::encodeSlaSpace` (sleighbase.cc:197-225): one `<space>` element
//!   per non-internal address space.
//!
//! WS5 ports those two as compiler-side helpers here (rather than editing
//! `kuna-sleigh`), driving the existing `kuna-sleigh` sub-encodes.  The byte
//! oracle (`python -m kuna.slacomp`) verifies the produced stream is identical to
//! `sleigh_opt`'s.
//!
//! ## Module ownership: WS5 owns this file exclusively.

#![allow(dead_code)]

use kuna_base::error::KunaResult;
use kuna_base::marshal::Encoder;

use crate::slgh_compile::SleighCompile;

/// Emit the entire compiled spec as the `.sla` element stream
/// (`SleighBase::encode`, sleighbase.cc:226).
///
/// Orchestrates: `<sleigh>` header attributes -> source-file indexer ->
/// `<spaces>` -> symbol table.  Each sub-part calls the already-ported
/// `kuna-sleigh` `encode(...)` methods.  Called by
/// [`SleighCompile::run_compilation`](crate::slgh_compile::SleighCompile::run_compilation)
/// once parse + `process` succeed with no errors.
pub fn encode_sleigh(_compiler: &SleighCompile, _encoder: &mut dyn Encoder) -> KunaResult<()> {
    todo!("WS5: SleighBase::encode (sleighbase.cc:226-255) -- top-level .sla emit")
}

/// Emit one `<space>` element for an address space (`SleighBase::encodeSlaSpace`,
/// sleighbase.cc:197).  Skips the internal spaces (constant/fspec/iop/join), as
/// the C++ `encode` loop does.
fn encode_sla_space(_compiler: &SleighCompile, _encoder: &mut dyn Encoder, _space_index: usize) -> KunaResult<()> {
    todo!("WS5: SleighBase::encodeSlaSpace (sleighbase.cc:197-225)")
}

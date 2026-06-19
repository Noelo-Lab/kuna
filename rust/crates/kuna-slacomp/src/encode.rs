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
//! What was **missing** is the single top-level orchestrator that the C++
//! compiler calls at the end of `run_compilation`:
//!
//! - `SleighBase::encode` (sleighbase.cc:226-255): opens `<sleigh>`, writes the
//!   version/endian/align/uniqbase/maxdelay/uniqmask/numsections attributes,
//!   emits the source-file indexer, the `<spaces>` block, then `symtab.encode`.
//! - `SleighBase::encodeSlaSpace` (sleighbase.cc:197-225): one `<space>` element
//!   per non-internal address space.
//!
//! WS5 landed those two **in `kuna-sleigh`** (`SleighBase::encode` /
//! `SleighBase::encode_sla_space`, next to `SleighBase::decode`) because they
//! need `&self` access to the private symbol-table / address-space / template
//! state and the private `SlaTrans` `ConstructTpl`-encode seam.  The plan
//! (`docs/rust-port/sleigh-compiler/map.md` WS5) explicitly allows this and
//! records it as a freeze interface.  This module is the compiler-side wiring:
//! it drives `SleighBase::encode` through a `FormatEncode` (header + packed
//! stream + deflate) to produce the final `.sla` byte buffer, exactly as the
//! C++ `run_compilation` does (`FormatEncode encoder(s,-1); encode(encoder);
//! encoder.flush();`).
//!
//! ## Module ownership: WS5 owns this file exclusively.

#![allow(dead_code)]

use std::io::Write;

use kuna_base::error::KunaResult;
use kuna_base::marshal::Encoder;

use kuna_sleigh::slaformat::FormatEncode;
use kuna_sleigh::sleighbase::SleighBase;

/// Emit the entire compiled spec as the `.sla` element stream
/// (`SleighBase::encode`, sleighbase.cc:226) into an already-open encoder.
///
/// This is the thin compiler-side delegate: the orchestration body lives on
/// `kuna_sleigh::SleighBase::encode` (the C++ method is on `SleighBase`, and
/// `SleighCompile` *is-a* `SleighBase`).  Drive it with a `<sleigh>`-capable
/// `Encoder` -- for the real `.sla` output that is a `FormatEncode`'s
/// [`packed`](kuna_sleigh::slaformat::FormatEncode::packed) view; see
/// [`encode_to_sla_writer`] / [`encode_to_sla_bytes`].
pub fn encode_sleigh(base: &SleighBase, encoder: &mut dyn Encoder) -> KunaResult<()> {
    base.encode(encoder)
}

/// Write the complete `.sla` byte stream for a compiled spec to `w`,
/// byte-for-byte as `sleigh_opt` writes it: the `sla\x04` header followed by the
/// deflate-compressed packed element stream from `SleighBase::encode`.
///
/// Mirrors the tail of C++ `SleighCompile::run_compilation`
/// (slgh_compile.cc:3805-3807): `FormatEncode encoder(s,-1); encode(encoder);
/// encoder.flush();`.  The `-1` is zlib's default compression level.  The
/// `slacomp` binary opens the `<file>.sla` output stream and hands it here.
pub fn encode_to_sla_writer<W: Write>(base: &SleighBase, w: W) -> KunaResult<()> {
    let mut encoder = FormatEncode::new(w, -1);
    {
        let mut packed = encoder.packed();
        base.encode(&mut packed)?;
    }
    encoder.flush()
}

/// Produce the complete `.sla` byte buffer for a compiled spec (the in-memory
/// form of [`encode_to_sla_writer`], used by the byte-identity round-trip
/// tests and any caller that wants the bytes rather than a stream).
pub fn encode_to_sla_bytes(base: &SleighBase) -> KunaResult<Vec<u8>> {
    let mut out: Vec<u8> = Vec::new();
    encode_to_sla_writer(base, &mut out)?;
    Ok(out)
}

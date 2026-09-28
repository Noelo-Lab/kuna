//! Encode a compiled [`SleighBase`] as a `.sla` image.
//!
//! The runtime types emit the element stream; [`FormatEncode`] supplies the
//! header, packed encoding and deflate compression used by Ghidra's compiler.

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
    if let Ok(path) = std::env::var("KUNA_DUMP_XML") {
        let mut buf: Vec<u8> = Vec::new();
        let mut xenc = kuna_base::marshal::XmlEncode::new(&mut buf);
        base.encode(&mut xenc)?;
        let _ = std::fs::write(path, &buf);
    }
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

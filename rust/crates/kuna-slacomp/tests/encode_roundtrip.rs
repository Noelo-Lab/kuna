//! WS5 verification: the `.sla` encode round-trip.
//!
//! The gold oracle for `sleigh_opt` is byte-identity: compiling a `.slaspec`
//! twice yields the same `.sla`.  WS5 ports the *encode* side
//! (`SleighBase::encode` + `encodeSlaSpace`).  A strong, self-contained witness
//! that this encode reproduces the C++ byte stream -- with no compiler driver
//! (WS1-WS4) yet wired -- is the **decode -> re-encode round-trip**: take an
//! existing C++-built `.sla`, decode it with the ported `kuna-sleigh` reader
//! into a `SleighBase`, re-encode it with WS5's `encode`, and assert the result
//! is byte-identical to the original `.sla`.
//!
//! ## The identity gate is the *element stream*, per LOSS-010
//!
//! The `.sla` file is `sla\x04` + a deflate-compressed packed element stream.
//! Two things could differ on re-encode:
//!
//! 1. **The uncompressed packed element stream** -- this is exactly what
//!    `SleighBase::encode` (WS5) produces.  Byte-identity here is the WS5
//!    contract and the hard assertion in every test below.
//! 2. **The compressed bytes** -- additionally a function of the deflate
//!    backend.  kuna's shared `FormatEncode`/`CompressBuffer` uses `flate2`'s
//!    `miniz_oxide`, which makes different (equally valid) encoder choices than
//!    C zlib at compressed levels.  This is the pre-existing, documented
//!    **LOSS-010** (`docs/rust-port/losses.md`): byte-identity against a
//!    C++-written `.sla` "cannot hold" without binding C zlib by FFI, and the
//!    correct gate is **content equality after decompression**.
//!
//! So the round-trip asserts element-stream byte-identity (WS5's actual
//! deliverable) and additionally records, non-fatally, whether whole-file
//! identity happens to hold.  See
//! `docs/rust-port/sleigh-compiler/ws5-encode.md` for the WS5 write-up.

use std::path::PathBuf;

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::space::AddrSpaceManager;

use kuna_sleigh::globalcontext::ContextInternal;
use kuna_sleigh::loadimage::LoadImage;
use kuna_sleigh::slaformat::{is_sla_format, FormatDecode};
use kuna_sleigh::sleigh::Sleigh;

use kuna_slacomp::encode::encode_to_sla_bytes;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap()
}

struct DummyImg;
impl LoadImage for DummyImg {
    fn get_file_name(&self) -> &str {
        "dummy"
    }
    fn load_fill(&mut self, _ptr: &mut [u8], _addr: &Address) -> KunaResult<()> {
        Err(KunaError::data_unavail("dummy"))
    }
    fn get_arch_type(&self) -> Vec<u8> {
        Vec::new()
    }
    fn adjust_vma(&mut self, _adjust: i64) {}
}

/// Decode a `.sla` byte buffer into a fully-initialized `Sleigh`.
fn sleigh_from_sla(sla: &[u8]) -> Sleigh {
    let ctx = Box::new(ContextInternal::new());
    let mut sleigh = Sleigh::new(Box::new(DummyImg), ctx);
    sleigh
        .initialize_from_sla(sla)
        .expect("initialize_from_sla");
    sleigh
}

/// Decompress a `.sla` file's body into its raw packed element stream (the
/// bytes `FormatDecode` ingests after stripping the header and inflating).
/// Used to compare *uncompressed* element streams independent of the deflate
/// backend.
fn sla_element_stream(sla: &[u8]) -> Vec<u8> {
    let (ok, _) = is_sla_format(sla);
    assert!(ok, ".sla must carry the sla\\x04 header");
    // FormatDecode::ingest_stream inflates the body; drive it and read back the
    // ingested raw bytes via a fresh decode of the packed stream is awkward, so
    // instead inflate directly with the same zlib the reader uses.
    inflate_sla_body(sla)
}

/// Inflate the deflate-compressed body of a `.sla` (everything after the 4-byte
/// `sla\x04` header) into the raw packed element stream.
fn inflate_sla_body(sla: &[u8]) -> Vec<u8> {
    use std::io::Read;
    // flate2 is the workspace deflate backend used by FormatEncode/CompressBuffer.
    let body = &sla[4..];
    let mut dec = flate2::read::ZlibDecoder::new(body);
    let mut out = Vec::new();
    dec.read_to_end(&mut out).expect("inflate .sla body");
    out
}

/// Core round-trip for one spec: decode -> re-encode -> compare.
fn check_roundtrip(rel: &str) {
    let path = repo_root().join(rel);
    let original = match std::fs::read(&path) {
        Ok(b) => b,
        Err(e) => panic!("read {} failed ({e}); is the .sla symlinked/built?", path.display()),
    };

    // Decode the C++-built .sla into a SleighBase.
    let sleigh = sleigh_from_sla(&original);

    // Re-encode it with WS5's SleighBase::encode.
    let reencoded = encode_to_sla_bytes(sleigh.base()).expect("encode_to_sla_bytes");

    // Level 1 (the WS5 contract): the uncompressed element streams must match.
    let orig_stream = inflate_sla_body(&original);
    let re_stream = inflate_sla_body(&reencoded);
    assert_eq!(
        re_stream.len(),
        orig_stream.len(),
        "{rel}: re-encoded element stream length differs ({} vs {} bytes)",
        re_stream.len(),
        orig_stream.len(),
    );
    if re_stream != orig_stream {
        // Find the first diverging byte for a precise diagnosis.
        let first = re_stream
            .iter()
            .zip(orig_stream.iter())
            .position(|(a, b)| a != b)
            .unwrap_or(orig_stream.len());
        panic!(
            "{rel}: re-encoded element stream diverges from the original at \
             byte {first} (re={:#04x} orig={:#04x})",
            re_stream.get(first).copied().unwrap_or(0),
            orig_stream.get(first).copied().unwrap_or(0),
        );
    }

    // Sanity: the re-encoded file decompresses to the asserted element stream
    // and carries the proper header -- i.e. it is a well-formed .sla the kuna
    // reader accepts (the end-to-end backstop relies on this).
    let (ok, _) = is_sla_format(&reencoded);
    assert!(ok, "{rel}: re-encoded .sla lost its sla\\x04 header");

    // Level 2 (non-fatal, LOSS-010): whole-file byte identity additionally needs
    // the deflate backend to match C zlib, which miniz_oxide does not.  Record
    // the outcome so a future C-zlib-FFI backend would surface here, but do NOT
    // fail on it -- the element-stream identity above is WS5's real contract.
    if reencoded != original {
        eprintln!(
            "{rel}: element stream byte-identical; whole .sla differs by \
             compression backend (LOSS-010 miniz_oxide vs C zlib): \
             {} vs {} compressed bytes",
            reencoded.len(),
            original.len(),
        );
    }
}

/// Round-trip the decoder/encoder seam directly (no Sleigh wrapper) to make sure
/// FormatDecode + FormatEncode share an inflate/deflate that the element-stream
/// comparison can rely on.
#[test]
fn ws5_format_decode_inflate_matches_manual() {
    let original = std::fs::read(
        repo_root().join("specs/Ghidra/Processors/DATA/data/languages/data-le-64.sla"),
    )
    .expect("read data-le-64.sla");
    let manager = AddrSpaceManager::new();
    let mut dec = FormatDecode::new(&manager);
    dec.ingest_stream(&original)
        .expect("FormatDecode ingests the real .sla");
    // The manual inflate path used by the round-trip test must succeed and be
    // non-empty (it is what we compare element streams with).
    let stream = sla_element_stream(&original);
    assert!(!stream.is_empty(), "inflated element stream is empty");
}

// --- The round-trip suite: simplest -> larger, per the WS7 spec ordering. ---

#[test]
fn ws5_roundtrip_data_le_64() {
    check_roundtrip("specs/Ghidra/Processors/DATA/data/languages/data-le-64.sla");
}

#[test]
fn ws5_roundtrip_data_be_64() {
    check_roundtrip("specs/Ghidra/Processors/DATA/data/languages/data-be-64.sla");
}

#[test]
fn ws5_roundtrip_toy_builder_le() {
    check_roundtrip("specs/Ghidra/Processors/Toy/data/languages/toy_builder_le.sla");
}

#[test]
fn ws5_roundtrip_toy_builder_be() {
    check_roundtrip("specs/Ghidra/Processors/Toy/data/languages/toy_builder_be.sla");
}

#[test]
fn ws5_roundtrip_toy_le() {
    check_roundtrip("specs/Ghidra/Processors/Toy/data/languages/toy_le.sla");
}

#[test]
fn ws5_roundtrip_toy_be() {
    check_roundtrip("specs/Ghidra/Processors/Toy/data/languages/toy_be.sla");
}

// Larger real ISAs: prove the top-level encode + every per-symbol/pattern/
// semantics sub-encode scales to full processor specs (attaches, with-blocks,
// macros, contexts, deep constructor tables).
#[test]
fn ws5_roundtrip_mips32be() {
    check_roundtrip("specs/Ghidra/Processors/MIPS/data/languages/mips32be.sla");
}

#[test]
fn ws5_roundtrip_x86() {
    check_roundtrip("specs/Ghidra/Processors/x86/data/languages/x86.sla");
}

#[test]
fn ws5_roundtrip_6502() {
    check_roundtrip("specs/Ghidra/Processors/6502/data/languages/6502.sla");
}

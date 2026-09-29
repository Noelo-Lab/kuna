//! Verify decoding and re-encoding preserve the `.sla` packed element stream.
//!
//! Inputs are the checkout's built specs, which may come from either compiler.
//! Compare decompressed streams because compressors can encode identical data
//! differently. Independent C++ compiler comparisons live in compiler_parity.rs.

use std::io::Read;
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

/// Validate the header and decompress the packed element stream.
fn sla_element_stream(sla: &[u8]) -> Vec<u8> {
    let (ok, _) = is_sla_format(sla);
    assert!(ok, ".sla must carry the sla\\x04 header");
    let mut dec = flate2::read::ZlibDecoder::new(&sla[4..]);
    let mut out = Vec::new();
    dec.read_to_end(&mut out).expect("inflate .sla body");
    out
}

/// Core round-trip for one spec: decode -> re-encode -> compare.
fn check_roundtrip(rel: &str) {
    let path = repo_root().join(rel);
    let original = match std::fs::read(&path) {
        Ok(b) => b,
        Err(e) => panic!(
            "read {} failed ({e}); is the .sla symlinked/built?",
            path.display()
        ),
    };

    let sleigh = sleigh_from_sla(&original);
    let reencoded = encode_to_sla_bytes(sleigh.base()).expect("encode_to_sla_bytes");
    let orig_stream = sla_element_stream(&original);
    let re_stream = sla_element_stream(&reencoded);
    assert_eq!(
        re_stream.len(),
        orig_stream.len(),
        "{rel}: re-encoded element stream length differs ({} vs {} bytes)",
        re_stream.len(),
        orig_stream.len(),
    );
    if let Some((first, (actual, expected))) = re_stream
        .iter()
        .zip(&orig_stream)
        .enumerate()
        .find(|(_, (actual, expected))| actual != expected)
    {
        panic!(
            "{rel}: re-encoded element stream diverges from the original at \
             byte {first} (re={actual:#04x} orig={expected:#04x})"
        );
    }
}

#[test]
fn format_decoder_accepts_built_sla() {
    let original = std::fs::read(
        repo_root().join("specs/Ghidra/Processors/DATA/data/languages/data-le-64.sla"),
    )
    .expect("read data-le-64.sla");
    let manager = AddrSpaceManager::new();
    let mut dec = FormatDecode::new(&manager);
    dec.ingest_stream(&original)
        .expect("FormatDecode ingests the real .sla");
    let stream = sla_element_stream(&original);
    assert!(!stream.is_empty(), "inflated element stream is empty");
}

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

//! CLI end-to-end gate for `kuna unpack` on PKLITE-compressed DOS programs.
//! Inputs come from the synthetic generator the analysis tier's own tests use,
//! so no PKWARE decompressor stub is vendored: the recovered file must be the
//! original program byte for byte, and a stub variant nothing recognizes must
//! be refused by name without writing anything.

use crate::common;
#[path = "../../kuna-analysis/tests/fixtures/pklite_synth.rs"]
mod synth;

fn packed(name: &str) -> (Vec<u8>, Vec<u8>) {
    let (_, layout, relocs) = synth::layouts().into_iter().find(|(n, ..)| *n == name).unwrap();
    let mut p = synth::program(0x1288, 12_000, relocs, &[]);
    p.overlay = vec![0x02, 0x70, 0xe2, 0xaa, 0x28, 0x10, 0x27];
    (synth::pack(&p, &layout), synth::expected(&p, &layout))
}

fn written(stem: &str, bytes: &[u8]) -> String {
    let path = common::scratch_file(stem, "exe");
    std::fs::write(&path, bytes).unwrap();
    path.to_str().unwrap().to_owned()
}

#[test]
fn unpack_recovers_the_original_dos_program() {
    let (image, want) = packed("v1.00 large");
    let input = written("pklite-large", &image);
    let out = common::scratch_file("pklite-large-out", "exe");
    let (stdout, stderr, code) = common::run_kuna(&["unpack", &input, "-o", out.to_str().unwrap()]);
    assert_eq!(code, 0, "{stderr}");
    assert!(stdout.contains("PKLITE"), "{stdout}");
    assert!(stdout.contains("58 relocations"), "{stdout}");
    assert!(stdout.contains("1b58:0010"), "{stdout}");
    assert_eq!(std::fs::read(&out).unwrap(), want);
}

#[test]
fn unpack_json_reports_a_rebuilt_header_for_extra_compression() {
    let (image, want) = packed("v1.14 XOR-scrambled large extra");
    let input = written("pklite-extra", &image);
    let out = common::scratch_file("pklite-extra-out", "exe");
    let (stdout, stderr, code) =
        common::run_kuna(&["unpack", &input, "-o", out.to_str().unwrap(), "--json"]);
    assert_eq!(code, 0, "{stderr}");
    let j: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(j["packer"], "pklite");
    assert_eq!(j["container"], "exe");
    assert_eq!(j["extra"], "xor");
    assert_eq!(j["scrambled"], true);
    assert_eq!(j["header"], "reconstructed");
    assert_eq!(j["relocations"], 40);
    assert_eq!(j["entry"], "1b58:0010");
    assert_eq!(j["unpacked_size"], want.len());
    assert_eq!(std::fs::read(&out).unwrap(), want);
}

#[test]
fn an_unknown_stub_variant_is_refused_by_name() {
    let (mut image, _) = packed("v1.00 large");
    image[0x70 + 64] = 0x90;
    let input = written("pklite-unknown", &image);
    let out = common::scratch_file("pklite-unknown-out", "exe");
    let (_, stderr, code) = common::run_kuna(&["unpack", &input, "-o", out.to_str().unwrap()]);
    assert_eq!(code, 1);
    assert!(stderr.contains("unsupported PKLITE file"), "{stderr}");
    assert!(stderr.contains("decompressor"), "{stderr}");
    assert!(!out.exists(), "a refused run must write nothing");
}

#[test]
fn a_packed_dos_program_points_at_kuna_unpack() {
    let (image, _) = packed("v1.00 large");
    let input = written("pklite-hint", &image);
    let (_, stderr, _) = common::run_kuna(&["functions", &input]);
    assert!(stderr.contains("PKLITE packed: run 'kuna unpack'"), "{stderr}");
}

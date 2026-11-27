use kuna_dex::{Dex, Options};

use crate::common::fixture;

fn exercise(data: &[u8]) {
    if let Ok(dex) = Dex::parse(data) {
        for r in dex.translate_all(&Options::default(), 1) {
            let _ = r.map(|t| t.bytes.len());
        }
    }
}

#[test]
fn truncated_files_are_errors_not_panics() {
    let data = fixture("release.dex");
    for len in (0..data.len()).step_by(97).chain([0x6f, 0x70, data.len() - 1]) {
        let mut cut = data[..len].to_vec();
        if cut.len() >= 0x24 {
            cut[0x20..0x24].copy_from_slice(&(len as u32).to_le_bytes());
        }
        exercise(&cut);
    }
    assert!(Dex::parse(&data[..0x50]).is_err());
}

#[test]
fn flipped_bytes_are_errors_not_panics() {
    let data = fixture("release.dex");
    let mut seed = 0x2545_f491_4f6c_dd1du64;
    for _ in 0..300 {
        let mut d = data.clone();
        for _ in 0..8 {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            let at = 0x70 + (seed as usize % (d.len() - 0x70));
            d[at] = (seed >> 32) as u8;
        }
        exercise(&d);
    }
}

#[test]
fn unsupported_versions_are_named() {
    let mut data = fixture("release.dex");
    data[4..7].copy_from_slice(b"041");
    let e = Dex::parse(&data).err().unwrap().0;
    assert!(e.contains("041") && e.contains("container"), "{e}");
    assert!(Dex::parse(b"PK\x03\x04 not a dex").is_err());
}

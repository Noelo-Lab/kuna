//! (kuna `blobs`) One well-formed file of every kind, found at its exact
//! length inside padding; each rejected once truncated or damaged.
//!
//! The JPEG, GIF and BMP samples are Pillow output (an 8x8 grey JPEG, a 3x2
//! two-colour GIF, a 3x2 BMP); the zip is Info-ZIP's, the gzip, xz and zstd
//! are those tools' output over the same 384 bytes of text.

use super::*;

const JPEG: &str = "ffd8ffe000104a46494600010100000100010000ffdb0043001b12141714111b1716171e1c1b2028422b28252528513a \
     3d3042605565645f555d5b6a7899816a7190735b5d85b586909ea3abadab6780bcc9baa6c799a8aba4ffc0000b080008 \
     000801011100ffc4001f0000010501010101010100000000000000000102030405060708090a0bffc400b51000020103 \
     03020403050504040000017d01020300041105122131410613516107227114328191a1082342b1c11552d1f024336272 \
     82090a161718191a25262728292a3435363738393a434445464748494a535455565758595a636465666768696a737475 \
     767778797a838485868788898a92939495969798999aa2a3a4a5a6a7a8a9aab2b3b4b5b6b7b8b9bac2c3c4c5c6c7c8c9 \
     cad2d3d4d5d6d7d8d9dae1e2e3e4e5e6e7e8e9eaf1f2f3f4f5f6f7f8f9faffda0008010100003f002bffd9";
const GIF: &str = "47494638376103000200810000ff00000000ff0000000000002c00000000030002000008080001080410004040003b";
const BMP: &str = "424d4e0000000000000036000000280000000300000002000000010018000000000018000000c40e0000c40e00000000 \
     0000000000000000ff0000ff0000ff0000000000ff0000ff0000ff000000";
const ZIP: &str = "504b03040a0000000000d87e725d6bf788970b0000000b0000000b000000626c6f62735f682e74787468656c6c6f2062 \
     6c6f620a504b0304140000000800d37e725d83cda07144000000800100000b000000626c6f62735f742e747874cb2ecd \
     4b5448cac94f5248cbac28292d4ab55228c94855282ccd4cce56482aca2fcf5348cbaf50c82acd2d2856c82f4b2d024b \
     e72456552aa4e4a7eb29648feaa7483f00504b01021e030a0000000000d87e725d6bf788970b0000000b0000000b0000 \
     000000000001000000a48100000000626c6f62735f682e747874504b01021e03140000000800d37e725d83cda0714400 \
     0000800100000b0000000000000001000000a48134000000626c6f62735f742e747874504b0506000000000200020072 \
     000000a10000000000";
const GZIP: &str = "1f8b0800000000000003cb2ecd4b5448cac94f5248cbac28292d4ab55228c94855282ccd4cce56482aca2fcf5348cbaf \
     50c82acd2d2856c82f4b2d024be72456552aa4e4a7eb29648feaa7483f0083cda07180010000";
const XZ: &str = "fd377a585a000004e6d6b44604c04c80032101160000000000000000fd1d4477e0017f00445d00359d49ff7e4119c6d5 \
     3c7644b9b4aea5faa3ad0ee4d758414ebfc3f3ab393b3da4fa148b0fccb03f54345be55f25a992d962ad387c7e110c3d \
     852f5514e0487265aa000000b76048dc960341b500016880030000009646a184b1c467fb020000000004595a";
const ZSTD: &str = "28b52ffd6480000d0200b2430d11903d0650fa43e90fa5cffff7d5e3f9af1c070896eb2dda34179e4667d7333c7f18ce \
     7146949537a93e98696b56e703b671f12c0b02003d43502884a0a0d031fcde";

fn unhex(s: &str) -> Vec<u8> {
    let digits: Vec<u8> = s.bytes().filter(u8::is_ascii_hexdigit).collect();
    digits.chunks(2).map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap()).collect()
}

fn png_chunk(kind: &[u8; 4], data: &[u8]) -> Vec<u8> {
    let mut out = (data.len() as u32).to_be_bytes().to_vec();
    out.extend_from_slice(kind);
    out.extend_from_slice(data);
    out.extend_from_slice(&crc32(&out[4..]).to_be_bytes());
    out
}

fn png_file(with_end: bool) -> Vec<u8> {
    let mut out = b"\x89PNG\r\n\x1a\n".to_vec();
    out.extend(png_chunk(b"IHDR", &[0, 0, 0, 1, 0, 0, 0, 1, 8, 2, 0, 0, 0]));
    out.extend(png_chunk(b"IDAT", &[0x78, 0x9c, 0x63, 0xf8, 0xcf, 0xc0, 0x00, 0x00, 0x03, 0x01, 0x01, 0x00]));
    if with_end {
        out.extend(png_chunk(b"IEND", &[]));
    }
    out
}

fn ico_file(image: &[u8]) -> Vec<u8> {
    let mut out = vec![0, 0, 1, 0, 1, 0, 16, 16, 0, 0, 1, 0, 32, 0];
    out.extend_from_slice(&(image.len() as u32).to_le_bytes());
    out.extend_from_slice(&22u32.to_le_bytes());
    out.extend_from_slice(image);
    out
}

fn dib_icon_image() -> Vec<u8> {
    let mut dib = vec![0u8; 40 + 16 * 16 * 4 + 16 * 4];
    dib[0] = 40;
    dib[4] = 16;
    dib[8] = 32;
    dib[12] = 1;
    dib[14] = 32;
    dib
}

fn samples() -> Vec<(BlobKind, Vec<u8>)> {
    vec![
        (BlobKind::Png, png_file(true)),
        (BlobKind::Jpeg, unhex(JPEG)),
        (BlobKind::Gif, unhex(GIF)),
        (BlobKind::Bmp, unhex(BMP)),
        (BlobKind::Ico, ico_file(&dib_icon_image())),
        (BlobKind::Zip, unhex(ZIP)),
        (BlobKind::Gzip, unhex(GZIP)),
        (BlobKind::Xz, unhex(XZ)),
        (BlobKind::Zstd, unhex(ZSTD)),
    ]
}

fn padded(file: &[u8]) -> Vec<u8> {
    let mut data = b"\x01padding".to_vec();
    data.extend_from_slice(file);
    data.extend_from_slice(b"trailing bytes\x02");
    data
}

#[test]
fn every_kind_is_found_at_its_exact_length() {
    for (kind, file) in samples() {
        let found = scan(&padded(&file));
        assert_eq!(found, [Blob { offset: 8, len: file.len(), kind }], "{kind:?}");
    }
}

#[test]
fn a_truncated_file_is_not_a_blob() {
    for (kind, file) in samples() {
        for cut in [1, file.len() / 2] {
            let found = scan(&file[..file.len() - cut]);
            assert!(found.iter().all(|b| b.kind != kind), "{kind:?} cut by {cut}: {found:?}");
        }
    }
    assert_eq!(scan(&padded(&png_file(false))), []);
}

#[test]
fn a_damaged_checksum_is_not_a_blob() {
    for kind in [BlobKind::Png, BlobKind::Gzip, BlobKind::Xz] {
        let mut file = samples().into_iter().find(|(k, _)| *k == kind).unwrap().1;
        let at = file.len() - 14;
        file[at] ^= 0x40;
        assert_eq!(scan(&file), [], "{kind:?}");
    }
}

#[test]
fn files_side_by_side_are_reported_in_order_and_disjoint() {
    let mut data = Vec::new();
    let mut want = Vec::new();
    for (kind, file) in samples() {
        data.extend_from_slice(b"--");
        want.push(Blob { offset: data.len(), len: file.len(), kind });
        data.extend_from_slice(&file);
    }
    assert_eq!(scan(&data), want);
}

#[test]
fn a_file_inside_another_is_reported_once() {
    let icon = ico_file(&png_file(true));
    assert_eq!(scan(&padded(&icon)), [Blob { offset: 8, len: icon.len(), kind: BlobKind::Ico }]);
}

#[test]
fn bare_magic_numbers_are_not_blobs() {
    let mut data = Vec::new();
    for (_, magic) in MAGICS {
        for _ in 0..64 {
            data.extend_from_slice(magic);
            data.extend_from_slice(&[0, 1, 0, 0, 0, 0, 40, 0, 0xff, 0xff]);
        }
    }
    assert_eq!(scan(&data), []);
}

#[test]
fn an_empty_xz_index_is_not_a_blob() {
    let mut file = unhex(XZ)[..12].to_vec();
    file.extend_from_slice(&[0; 12]);
    let mut footer = vec![0, 0, 0, 0, file[6], file[7]];
    let crc = crc32(&footer);
    file.extend_from_slice(&crc.to_le_bytes());
    file.append(&mut footer);
    file.extend_from_slice(b"YZ");
    assert_eq!(scan(&file), []);
}

#[test]
fn the_work_cap_ends_a_scan_of_nested_scans() {
    let mut data = Vec::new();
    for _ in 0..64 {
        data.extend_from_slice(&unhex(XZ)[..12]);
    }
    data.resize(data.len() + (1 << 20), 0x55);
    data.extend(png_file(true));
    let mut work = Work(4 << 20);
    assert_eq!(scan_within(&data, &mut work), []);
    assert!(work.exhausted());
    assert_eq!(scan_within(&data, &mut Work(usize::MAX)).len(), 1);
}

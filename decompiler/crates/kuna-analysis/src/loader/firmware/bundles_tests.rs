//! Tar, zip and FIT member tables over archives built in the test, and the
//! malformed inputs they must refuse without panicking.

use std::io::Write;

use super::*;

fn tar_member(name: &str, kind: u8, data: &[u8]) -> Vec<u8> {
    let mut header = [0u8; 512];
    header[..name.len()].copy_from_slice(name.as_bytes());
    header[124..135].copy_from_slice(format!("{:011o}", data.len()).as_bytes());
    header[156] = kind;
    header[257..262].copy_from_slice(b"ustar");
    header[148..156].fill(b' ');
    let sum: u32 = header.iter().map(|&b| u32::from(b)).sum();
    header[148..155].copy_from_slice(format!("{sum:06o}\0").as_bytes());
    let mut out = header.to_vec();
    out.extend(data);
    out.resize(out.len().next_multiple_of(512), 0);
    out
}

fn sample_tar() -> Vec<u8> {
    let mut tar = tar_member("dir/", b'5', b"");
    tar.extend(tar_member("dir/a.o", b'0', b"alpha"));
    tar.extend(tar_member("././@LongLink", b'L', b"a/very/long/name.o\0"));
    tar.extend(tar_member("truncated", b'0', b"beta"));
    tar.extend(tar_member("pax", b'x', b"16 path=pax/c.o\n"));
    tar.extend(tar_member("short", 0, b"gamma"));
    tar.extend([0u8; 1024]);
    tar
}

fn deflate(data: &[u8]) -> Vec<u8> {
    let mut enc = flate2::write::DeflateEncoder::new(Vec::new(), flate2::Compression::default());
    enc.write_all(data).unwrap();
    enc.finish().unwrap()
}

fn crc(data: &[u8]) -> u32 {
    let mut sum = flate2::Crc::new();
    sum.update(data);
    sum.sum()
}

/// A zip of `(name, method, flags, data)` members.
fn zip_of(members: &[(&str, u16, u16, &[u8])]) -> Vec<u8> {
    let (mut out, mut directory) = (Vec::new(), Vec::new());
    for (name, method, flags, data) in members {
        let packed = if *method == 8 { deflate(data) } else { data.to_vec() };
        let local = out.len() as u32;
        let mut fixed = Vec::new();
        for field in [20u16, *flags, *method, 0, 0] {
            fixed.extend(field.to_le_bytes());
        }
        for field in [crc(data), packed.len() as u32, data.len() as u32] {
            fixed.extend(field.to_le_bytes());
        }
        fixed.extend((name.len() as u16).to_le_bytes());
        fixed.extend(0u16.to_le_bytes());
        out.extend(b"PK\x03\x04");
        out.extend(&fixed);
        out.extend(name.bytes());
        out.extend(&packed);
        directory.extend(b"PK\x01\x02\x14\x00");
        directory.extend(&fixed);
        directory.extend([0u8; 10]);
        directory.extend(local.to_le_bytes());
        directory.extend(name.bytes());
    }
    let at = out.len() as u32;
    out.extend(&directory);
    out.extend(b"PK\x05\x06\0\0\0\0");
    out.extend((members.len() as u16).to_le_bytes());
    out.extend((members.len() as u16).to_le_bytes());
    out.extend((directory.len() as u32).to_le_bytes());
    out.extend(at.to_le_bytes());
    out.extend(0u16.to_le_bytes());
    out
}

/// A flattened device tree of `tokens` over the given strings block.
fn fdt(structs: &[u8], strings: &[u8], trailer: &[u8]) -> Vec<u8> {
    let total = 40 + structs.len() + strings.len();
    let mut out = FDT_MAGIC.to_vec();
    for field in [total, 40, 40 + structs.len(), 40, 17, 16, 0, strings.len(), structs.len()] {
        out.extend((field as u32).to_be_bytes());
    }
    out.extend(structs);
    out.extend(strings);
    out.resize(out.len().next_multiple_of(4), 0);
    out.extend(trailer);
    out
}

struct Tree {
    structs: Vec<u8>,
    strings: Vec<u8>,
}

impl Tree {
    fn open(&mut self, name: &str) -> &mut Self {
        self.structs.extend(1u32.to_be_bytes());
        self.structs.extend(name.bytes());
        self.structs.push(0);
        self.structs.resize(self.structs.len().next_multiple_of(4), 0);
        self
    }
    fn close(&mut self) -> &mut Self {
        self.structs.extend(2u32.to_be_bytes());
        self
    }
    fn prop(&mut self, name: &str, value: &[u8]) -> &mut Self {
        self.structs.extend(3u32.to_be_bytes());
        self.structs.extend((value.len() as u32).to_be_bytes());
        self.structs.extend((self.strings.len() as u32).to_be_bytes());
        self.strings.extend(name.bytes());
        self.strings.push(0);
        self.structs.extend(value);
        self.structs.resize(self.structs.len().next_multiple_of(4), 0);
        self
    }
    fn build(&mut self, trailer: &[u8]) -> Vec<u8> {
        self.structs.extend(9u32.to_be_bytes());
        fdt(&self.structs, &self.strings, trailer)
    }
}

fn sample_fit() -> Vec<u8> {
    let mut gz = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
    gz.write_all(&[0xc0, 0x03, 0x5f, 0xd6]).unwrap();
    let gz = gz.finish().unwrap();
    let mut tree = Tree { structs: Vec::new(), strings: Vec::new() };
    tree.open("").open("images");
    tree.open("kernel-1").prop("data", &gz).prop("type", b"kernel\0").prop("arch", b"arm64\0");
    tree.prop("compression", b"gzip\0").prop("load", &0x8008_0000u32.to_be_bytes());
    tree.prop("entry", &0x8008_0000u64.to_be_bytes()).open("hash-1").prop("algo", b"crc32\0").close().close();
    tree.open("fdt-1").prop("data-offset", &0u32.to_be_bytes()).prop("data-size", &3u32.to_be_bytes());
    tree.prop("type", b"flat_dt\0").close();
    tree.open("packed").prop("data", b"x").prop("compression", b"lzma\0").close().close();
    tree.open("configurations").prop("default", b"conf-1\0");
    tree.open("conf-1").prop("firmware", b"fdt-1\0").prop("kernel", b"kernel-1\0").close().close().close();
    tree.build(b"dtb")
}

fn refusal(bytes: &[u8]) -> String {
    Bundle::parse(bytes).err().expect("the input is refused")
}

fn names(bundle: &Bundle<'_>) -> Vec<String> {
    bundle.entries.iter().map(|e| e.name.clone()).collect()
}

#[test]
fn tar_lists_regular_files_under_their_long_and_pax_names() {
    let tar = sample_tar();
    let bundle = Bundle::parse(&tar).unwrap().unwrap();
    assert_eq!(bundle.kind, Kind::Tar);
    assert_eq!(names(&bundle), ["dir/a.o", "a/very/long/name.o", "pax/c.o"]);
    assert_eq!(&*bundle.unpack(&bundle.entries[1]).unwrap(), b"beta");
    assert_eq!(bundle.entries[2].size, 5);
}

#[test]
fn gzip_compressed_tar_is_inflated_and_other_gzip_is_not_an_archive() {
    let mut gz = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
    gz.write_all(&sample_tar()).unwrap();
    let gz = gz.finish().unwrap();
    assert!(sniff(&gz));
    let bundle = Bundle::parse(&gz).unwrap().unwrap();
    assert_eq!(&*bundle.unpack(&bundle.entries[0]).unwrap(), b"alpha");

    let mut plain = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
    plain.write_all(&[7u8; 2048]).unwrap();
    assert!(Bundle::parse(&plain.finish().unwrap()).unwrap().is_none());
}

#[test]
fn malformed_tar_is_refused() {
    let mut tar = sample_tar();
    tar[512 + 124..512 + 135].copy_from_slice(b"77777777777");
    tar[512 + 148..512 + 156].fill(b' ');
    let sum: u32 = tar[512..1024].iter().map(|&b| u32::from(b)).sum();
    tar[512 + 148..512 + 155].copy_from_slice(format!("{sum:06o}\0").as_bytes());
    assert!(refusal(&tar).contains("runs past the file"));

    let mut tar = sample_tar();
    tar[1536 + 3] ^= 0x55;
    assert!(refusal(&tar).contains("checksum"));
    assert!(!sniff(&[b'x'; 512]));
    assert!(Bundle::parse(&[b'x'; 600]).unwrap().is_none());
}

#[test]
fn zip_lists_files_and_inflates_checked_members() {
    let zip = zip_of(&[("dir/", 0, 0, b""), ("a.o", 8, 0, &[5u8; 300]), ("b.txt", 0, 0, b"stored"), ("c", 14, 0, b"x"), ("d", 0, 1, b"x")]);
    let bundle = Bundle::parse(&zip).unwrap().unwrap();
    assert_eq!(bundle.kind, Kind::Zip);
    assert_eq!(names(&bundle), ["a.o", "b.txt", "c", "d"]);
    assert_eq!(bundle.entries[0].size, 300);
    assert_eq!(&*bundle.unpack(&bundle.entries[0]).unwrap(), &[5u8; 300]);
    assert_eq!(&*bundle.unpack(&bundle.entries[1]).unwrap(), b"stored");
    assert!(bundle.unpack(&bundle.entries[2]).unwrap_err().contains("method 14"));
    assert!(bundle.unpack(&bundle.entries[3]).unwrap_err().contains("encrypted"));
}

#[test]
fn malformed_zip_is_refused() {
    let good = zip_of(&[("a.o", 8, 0, &[5u8; 300])]);
    let mut bad_crc = good.clone();
    let directory = good.windows(4).position(|w| w == b"PK\x01\x02").unwrap();
    bad_crc[directory + 16] ^= 1;
    let bundle = Bundle::parse(&bad_crc).unwrap().unwrap();
    assert!(bundle.unpack(&bundle.entries[0]).unwrap_err().contains("CRC-32"));

    let mut far = good.clone();
    far[directory + 42..directory + 46].copy_from_slice(&0x7fff_ffffu32.to_le_bytes());
    let bundle = Bundle::parse(&far).unwrap().unwrap();
    assert!(bundle.unpack(&bundle.entries[0]).unwrap_err().contains("runs past the file"));

    let mut count = good.clone();
    let eocd = good.len() - 22;
    count[eocd + 10..eocd + 12].copy_from_slice(&900u16.to_le_bytes());
    assert!(refusal(&count).contains("central directory"));
    assert!(refusal(&good[..good.len() - 30]).contains("central directory"));
    for cut in 0..good.len() {
        let _ = Bundle::parse(&good[..cut]);
    }
}

#[test]
fn fit_lists_images_with_their_load_facts() {
    let fit = sample_fit();
    let bundle = Bundle::parse(&fit).unwrap().unwrap();
    assert_eq!(bundle.kind, Kind::Fit);
    assert_eq!(names(&bundle), ["kernel-1", "fdt-1", "packed"]);
    assert_eq!(bundle.default.as_deref(), Some("kernel-1"));
    let kernel = &bundle.entries[0];
    assert_eq!((kernel.size, kernel.fit.as_ref().unwrap().kind.as_str()), (4, "kernel"));
    let flat = bundle.flat(kernel).unwrap().unwrap();
    assert_eq!(flat.target.as_deref(), Some("AARCH64:LE:64:v8A"));
    assert_eq!((flat.base, flat.entries.as_slice(), flat.bytes.as_slice()), (0x8008_0000, &[0x8008_0000][..], &[0xc0, 0x03, 0x5f, 0xd6][..]));
    assert_eq!(&*bundle.unpack(&bundle.entries[1]).unwrap(), b"dtb", "external data follows the tree");
    assert!(bundle.flat(&bundle.entries[1]).unwrap().is_none());
    assert!(bundle.unpack(&bundle.entries[2]).unwrap_err().contains("lzma"));
}

#[test]
fn single_byte_damage_never_panics() {
    let zip = zip_of(&[("a.o", 8, 0, &[5u8; 300]), ("b", 0, 0, b"stored")]);
    let mut seed = 0x9e37_79b9_7f4a_7c15u64;
    for sample in [sample_tar(), zip, sample_fit()] {
        for _ in 0..4000 {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            let mut damaged = sample.clone();
            damaged[(seed >> 16) as usize % sample.len()] = seed as u8;
            let Ok(Some(bundle)) = Bundle::parse(&damaged) else { continue };
            for entry in &bundle.entries {
                let _decoded = (bundle.unpack(entry), bundle.flat(entry));
            }
        }
    }
}

#[test]
fn a_device_tree_without_images_is_not_a_fit_and_malformed_trees_are_refused() {
    let mut tree = Tree { structs: Vec::new(), strings: Vec::new() };
    tree.open("").prop("model", b"board\0").close();
    assert!(Bundle::parse(&tree.build(b"")).unwrap().is_none());

    let fit = sample_fit();
    for cut in 0..fit.len() {
        let _ = Bundle::parse(&fit[..cut]);
    }
    let mut junk = fit.clone();
    junk[40..44].copy_from_slice(&7u32.to_be_bytes());
    assert!(refusal(&junk).contains("token 0x7"));

    let mut deep = Tree { structs: Vec::new(), strings: Vec::new() };
    for _ in 0..40 {
        deep.open("n");
    }
    assert!(refusal(&deep.build(b"")).contains("nests deeper"));
}

use super::*;

const DATA: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/kernelcache");

fn fixture(name: &str) -> Vec<u8> {
    std::fs::read(format!("{DATA}/{name}")).unwrap_or_else(|e| panic!("{name}: {e}"))
}

/// Every LZFSE block kind round-trips against the reference encoder's output.
#[test]
fn lzfse_decodes_reference_streams() {
    let plain = fixture("text.bin");
    for name in ["text.bvx2.lzfse", "small.bvxn.lzfse", "random.raw.lzfse"] {
        let packed = fixture(name);
        let expected = if name.starts_with("text") {
            plain.clone()
        } else {
            fixture(&name.replace(".bvxn.lzfse", ".bin").replace(".raw.lzfse", ".bin"))
        };
        let out = lzfse::decode(&packed, 0, usize::MAX).unwrap_or_else(|e| panic!("{name}: {e}"));
        assert_eq!(out, expected, "{name}");
    }
}

#[test]
fn lzfse_rejects_truncation_and_size_overrun() {
    let packed = fixture("text.bvx2.lzfse");
    assert!(lzfse::decode(&packed[..packed.len() - 5], 0, usize::MAX).is_err());
    assert!(lzfse::decode(&packed, 0, 100).is_err());
    let mut corrupt = packed.clone();
    let mid = corrupt.len() / 2;
    corrupt[mid] ^= 0x5a;
    corrupt[mid + 1] ^= 0xa5;
    assert!(lzfse::decode(&corrupt, 0, usize::MAX).map(|o| o != fixture("text.bin")).unwrap_or(true));
}

#[test]
fn lzss_decodes_back_references() {
    let stream = [0b1111_0111u8, b'a', b'b', b'c', 0xee, 0xf0, b'x', b'y', b'z', b'w'];
    let out = lzss::decode_stream(&stream, 9).unwrap();
    assert_eq!(out, b"abcabcxyz");
}

/// A back-reference that runs past the declared size is rejected, even when
/// the Adler-32 of the truncated prefix matches.
#[test]
fn lzss_rejects_a_match_past_the_declared_size() {
    let stream = [0b0000_0111u8, b'a', b'b', b'c', 0xee, 0xf0];
    assert!(lzss::decode_stream(&stream, 5).is_err());
    let mut container = lzss::MAGIC.to_vec();
    container.extend_from_slice(&lzss::adler32(b"abcab").to_be_bytes());
    container.extend_from_slice(&5u32.to_be_bytes());
    container.extend_from_slice(&(stream.len() as u32).to_be_bytes());
    container.resize(0x180, 0);
    container.extend_from_slice(&stream);
    assert!(lzss::decode(&container, usize::MAX).unwrap_err().contains("past the declared"));
}

/// A block whose bit streams are empty decodes to nothing instead of
/// overflowing the shift that loads the stream's last word.
#[test]
fn lzfse_empty_bit_stream_is_an_error() {
    let mut block = b"bvx2".to_vec();
    block.extend_from_slice(&[0u8; 28]);
    block[24] = 32;
    block.extend_from_slice(b"bvx$");
    assert_eq!(lzfse::decode(&block, 0, usize::MAX), Ok(Vec::new()));
}

#[test]
fn complzss_checks_size_and_checksum() {
    let payload = b"hello hello hello";
    let mut stream = Vec::new();
    for chunk in payload.chunks(8) {
        stream.push(((1u16 << chunk.len()) - 1) as u8);
        stream.extend_from_slice(chunk);
    }
    let mut container = lzss::MAGIC.to_vec();
    container.extend_from_slice(&lzss::adler32(payload).to_be_bytes());
    container.extend_from_slice(&(payload.len() as u32).to_be_bytes());
    container.extend_from_slice(&(stream.len() as u32).to_be_bytes());
    container.resize(0x180, 0);
    container.extend_from_slice(&stream);
    assert_eq!(lzss::decode(&container, usize::MAX).unwrap(), payload);
    container[8] ^= 1;
    assert!(lzss::decode(&container, usize::MAX).unwrap_err().contains("Adler-32"));
}

fn tlv(tag: u8, value: &[u8]) -> Vec<u8> {
    let mut out = vec![tag];
    if value.len() < 0x80 {
        out.push(value.len() as u8);
    } else {
        out.push(0x84);
        out.extend_from_slice(&(value.len() as u32).to_be_bytes());
    }
    out.extend_from_slice(value);
    out
}

fn im4p(payload: &[u8], extra: &[Vec<u8>]) -> Vec<u8> {
    let mut body = tlv(TAG_IA5STRING, b"IM4P");
    body.extend(tlv(TAG_IA5STRING, b"krnl"));
    body.extend(tlv(TAG_IA5STRING, b"test"));
    body.extend(tlv(TAG_OCTET_STRING, payload));
    for e in extra {
        body.extend_from_slice(e);
    }
    tlv(TAG_SEQUENCE, &body)
}

fn img4(im4p: &[u8]) -> Vec<u8> {
    let mut body = tlv(TAG_IA5STRING, b"IMG4");
    body.extend_from_slice(im4p);
    body.extend(tlv(0xa0, &tlv(TAG_SEQUENCE, &tlv(TAG_IA5STRING, b"IM4M"))));
    tlv(TAG_SEQUENCE, &body)
}

#[test]
fn img4_and_im4p_unwrap_to_the_decoded_payload() {
    let packed = fixture("text.bvx2.lzfse");
    let plain = fixture("text.bin");
    let size = tlv(TAG_SEQUENCE, &[tlv(TAG_INTEGER, &[1]), tlv(TAG_INTEGER, &(plain.len() as u32).to_be_bytes())].concat());
    let bare = im4p(&packed, &[size]);
    for input in [bare.clone(), img4(&bare)] {
        assert!(is_container(&input));
        let peeled = peel(input, &KernelcacheChoice::Auto).unwrap();
        assert_eq!(peeled.bytes, plain);
        assert!(peeled.note.unwrap().contains("'krnl' (LZFSE"));
    }
    let off = peel(img4(&bare), &KernelcacheChoice::Off).unwrap();
    assert_eq!(off.bytes, img4(&bare));
}

#[test]
fn encrypted_im4p_is_refused() {
    let kbag = tlv(TAG_OCTET_STRING, &tlv(TAG_SEQUENCE, &tlv(TAG_SEQUENCE, &tlv(TAG_INTEGER, &[1]))));
    let err = peel(img4(&im4p(b"ciphertext", &[kbag])), &KernelcacheChoice::Auto).err().unwrap();
    assert!(err.contains("encrypted"), "{err}");
}

#[test]
fn fileset_entry_selection() {
    let image = fixture("fileset.macho");
    let all = fileset::entries(&image).unwrap();
    let ids: Vec<&str> = all.iter().map(|e| e.id.as_str()).collect();
    assert_eq!(ids, ["com.apple.kernel", "com.example.driver.Widget"]);

    let kernel = peel(image.clone(), &KernelcacheChoice::Auto).unwrap();
    assert_eq!(&kernel.bytes[12..16], &2u32.to_le_bytes(), "auto loads the MH_EXECUTE kernel");
    let kext = peel(image.clone(), &KernelcacheChoice::Entry("Widget".into())).unwrap();
    assert_eq!(&kext.bytes[12..16], &11u32.to_le_bytes(), "suffix match picks the kext");
    let err = peel(image.clone(), &KernelcacheChoice::Entry("Gadget".into())).err().unwrap();
    assert!(err.contains("com.apple.kernel"), "{err}");
    let list = peel(image, &KernelcacheChoice::Entry("list".into())).err().unwrap();
    assert!(list.ends_with("(2):\ncom.apple.kernel\ncom.example.driver.Widget"), "{list}");
}

/// The entry map names the entry that maps an address, the loaded one first
/// for the `__LINKEDIT` every entry shares.
#[test]
fn fileset_map_names_the_owning_entry() {
    let base = 0xffff_fff0_0700_4000u64;
    let kext = peel(fixture("fileset.macho"), &KernelcacheChoice::Entry("Widget".into())).unwrap();
    let map = kext.fileset.unwrap();
    assert_eq!(map.loaded, "com.example.driver.Widget");
    assert_eq!(map.owner(base + 0x3800), Some("com.example.driver.Widget"), "widget_start");
    assert_eq!(map.owner(base + 0x3000), Some("com.apple.kernel"), "kernel_add");
    assert_eq!(map.owner(base + 0x5000), Some("com.example.driver.Widget"), "shared __LINKEDIT");
    assert_eq!(map.owner(base + 0x6000), None, "__PRELINK_INFO is the collection's own");
    assert!(peel(fixture("text.bin"), &KernelcacheChoice::Auto).unwrap().fileset.is_none());
}

/// The kext's authenticated `__got` slot and the kernel's function table read as
/// real addresses once the collection's chained fixups are applied.
#[test]
fn fileset_pointer_slots_are_rebased() {
    let base = 0xffff_fff0_0700_4000u64;
    let kext = peel(fixture("fileset.macho"), &KernelcacheChoice::Entry("Widget".into())).unwrap();
    let slot = |off: usize| u64::from_le_bytes(kext.bytes[off..off + 8].try_into().unwrap());
    assert_eq!(slot(0x4000), base + 0x3000, "kernel_table[0] -> kernel_add");
    assert_eq!(slot(0x4800), base + 0x3000, "auth __got slot, PAC bits stripped");
    assert_eq!(slot(0x4808), base + 0x2400, "kext __const -> \"Widget\"");
    assert!(kext.note.unwrap().contains("3 chained-fixup pointers rebased"));
}

/// An Auxiliary KC's chains name two cache levels: its own (3), rebased from
/// its base, and the boot collection's (0), which this file does not hold and
/// so leaves the slot as it was.
#[test]
fn auxiliary_collection_rebases_only_its_own_level() {
    let base = 0xffff_fff0_0700_4000u64;
    let mut image = fixture("fileset.macho");
    let word = |image: &[u8], off: usize| u64::from_le_bytes(image[off..off + 8].try_into().unwrap());
    let set_level = |image: &mut Vec<u8>, off: usize, level: u64| {
        let raw = (word(image, off) & !(3 << 30)) | (level << 30);
        image[off..off + 8].copy_from_slice(&raw.to_le_bytes());
    };
    set_level(&mut image, 0x4000, 3);
    set_level(&mut image, 0x4808, 3);
    set_level(&mut image, 0x4800, 0);
    let foreign = word(&image, 0x4800);
    let unlinked = peel(image.clone(), &KernelcacheChoice::Entry("Widget".into())).unwrap();
    assert_eq!(word(&unlinked.bytes, 0x4000), word(&image, 0x4000), "a boot KC does not rebase level 3");
    let pad = image.windows(4).position(|w| w == b"<!--").unwrap();
    let end = pad + image[pad..].windows(3).position(|w| w == b"-->").unwrap() + 3;
    let links = b"<key>_BootKCID</key><key>_PageableKCID</key>";
    image[pad..end].fill(b' ');
    image[pad..pad + links.len()].copy_from_slice(links);
    let kext = peel(image, &KernelcacheChoice::Entry("Widget".into())).unwrap();
    assert_eq!(word(&kext.bytes, 0x4000), base + 0x3000);
    assert_eq!(word(&kext.bytes, 0x4808), base + 0x2400, "kext __const -> \"Widget\"");
    assert_eq!(word(&kext.bytes, 0x4800), foreign, "a boot-KC target is not rebased from this base");
    assert!(kext.note.unwrap().contains("2 chained-fixup pointers rebased"));
}

#[test]
fn compressed_containers_peel_to_the_same_kernel() {
    let expected = peel(fixture("fileset.macho"), &KernelcacheChoice::Auto).unwrap().bytes;
    for name in ["kernelcache.img4", "kernelcache.lzss.im4p"] {
        let peeled = peel(fixture(name), &KernelcacheChoice::Auto).unwrap();
        assert!(peeled.bytes == expected, "{name}");
    }
}

/// A header that declares a huge decoded size allocates by the input it has,
/// not by the declaration, and still fails cleanly.
#[test]
fn declared_sizes_do_not_drive_allocation() {
    let mut container = lzss::MAGIC.to_vec();
    container.extend_from_slice(&0u32.to_be_bytes());
    container.extend_from_slice(&((1u32 << 30) - 1).to_be_bytes());
    container.extend_from_slice(&2u32.to_be_bytes());
    container.resize(0x180, 0);
    container.extend_from_slice(&[0x01, b'a']);
    assert!(lzss::decode(&container, DECODED_LIMIT).is_err());

    let raw_block = [b"bvx-".as_slice(), &u32::MAX.to_le_bytes(), b"xy"].concat();
    assert!(lzfse::decode(&raw_block, usize::MAX, DECODED_LIMIT).is_err());
}

/// A fileset entry whose `sizeofcmds` runs past the file is refused, not copied.
#[test]
fn truncated_fileset_entry_header_is_refused() {
    let mut image = fixture("fileset.macho");
    image[0x2000 + 20..0x2000 + 24].copy_from_slice(&u32::MAX.to_le_bytes());
    assert!(peel(image, &KernelcacheChoice::Entry("Widget".into())).is_err());
}

fn segment_names(image: &[u8]) -> Vec<String> {
    use object::read::{Object, ObjectSegment};
    let file = object::File::parse(image).unwrap();
    file.segments().map(|s| s.name().unwrap().unwrap_or("").to_string()).collect()
}

/// Two entries load as one image: both entries' code and symbols, the kext's
/// `__got` still named through the merged indirect-symbol table.
#[test]
fn several_entries_merge_into_one_image() {
    use object::read::{Object, ObjectSymbol};
    let merged = peel(fixture("fileset.macho"), &KernelcacheChoice::Entry("com.apple.kernel,Widget".into())).unwrap();
    assert!(merged.note.as_deref().unwrap().contains("as one image"), "{:?}", merged.note);
    let file = object::File::parse(&*merged.bytes).unwrap();
    let names: Vec<String> = file.symbols().filter_map(|s| s.name().ok().map(str::to_string)).collect();
    for want in ["_kernel_add", "_kernel_dispatch", "_widget_start", "_widget_via_stub"] {
        assert!(names.iter().any(|n| n == want), "{want} missing from {names:?}");
    }
    let imports = crate::loader::macho_stubs::resolve_macho_imports(&file, &merged.bytes);
    let got = imports.iter().find(|i| i.addr == 0xffff_fff0_0700_8800).map(|i| String::from_utf8_lossy(&i.name).into_owned());
    assert_eq!(got.as_deref(), Some("kernel_add"));
    assert_eq!(segment_names(&merged.bytes).iter().filter(|s| *s == "__TEXT_EXEC").count(), 2);

    let all = peel(fixture("fileset.macho"), &KernelcacheChoice::Entry("all".into())).unwrap();
    assert_eq!(all.bytes, merged.bytes, "`all` is both entries here");
}

/// A collection's `__BRANCH_STUBS` segment is mapped with the chosen entry.
#[test]
fn branch_stub_segments_load_with_the_entry() {
    let mut image = fixture("fileset.macho");
    let ncmds = u32::from_le_bytes(image[16..20].try_into().unwrap());
    let sizeofcmds = u32::from_le_bytes(image[20..24].try_into().unwrap()) as usize;
    let at = 32 + sizeofcmds;
    let mut seg = vec![0u8; 72];
    seg[0..4].copy_from_slice(&0x19u32.to_le_bytes());
    seg[4..8].copy_from_slice(&72u32.to_le_bytes());
    seg[8..22].copy_from_slice(b"__BRANCH_STUBS");
    seg[24..32].copy_from_slice(&0xffff_fff0_0700_7000u64.to_le_bytes());
    seg[32..40].copy_from_slice(&0x800u64.to_le_bytes());
    seg[40..48].copy_from_slice(&0x3000u64.to_le_bytes());
    seg[48..56].copy_from_slice(&0x800u64.to_le_bytes());
    image[at..at + 72].copy_from_slice(&seg);
    image[16..20].copy_from_slice(&(ncmds + 1).to_le_bytes());
    image[20..24].copy_from_slice(&((sizeofcmds + 72) as u32).to_le_bytes());
    let kext = peel(image, &KernelcacheChoice::Entry("Widget".into())).unwrap();
    assert!(kext.note.as_deref().unwrap().contains("1 branch-stub segments"), "{:?}", kext.note);
    assert!(segment_names(&kext.bytes).iter().any(|s| s == "__BRANCH_STUBS"));
}

/// A kernel entry keeps its collection's signed slots beside the image, so
/// its virtual calls resolve against every entry's vtables: slot 0 holds
/// `Base_m0` in both, slot 1 `Base_m1` in the kernel and `Widget_m1` in the
/// kext. The cut-out image alone has no signing context left.
#[test]
fn fileset_signed_slots_resolve_kernel_virtual_calls() {
    use crate::rtti::kuna_arm64evcall::{resolve, resolve_signed, VcallSite};
    let base = 0xffff_fff0_0700_4000u64;
    let kernel = peel(fixture("fileset_vcall.macho"), &KernelcacheChoice::Auto).unwrap();
    let signed = kernel.signed.expect("signed slots");
    assert_eq!(signed.slots.len(), 4);
    let file = object::File::parse(kernel.bytes.as_slice()).unwrap();
    assert!(resolve(&file, &kernel.bytes).is_empty());
    assert_eq!(
        resolve_signed(&file, &signed),
        vec![
            VcallSite { site: base + 0x302c, register: 8, targets: vec![base + 0x3000] },
            VcallSite { site: base + 0x3054, register: 8, targets: vec![base + 0x3008, base + 0x3800] },
        ]
    );
}

/// A standalone kext's absolute symbols are not read as collection-bound
/// imports; the same kext carved from its collection keeps them.
#[test]
fn only_a_collection_kext_reads_bound_imports() {
    use object::read::Object;
    let kext = peel(fixture("fileset.macho"), &KernelcacheChoice::Entry("Widget".into())).unwrap().bytes;
    let bound = |bytes: &[u8]| {
        let file = object::File::parse(bytes).unwrap();
        crate::loader::macho_stubs::resolve_macho_imports(&file, bytes)
            .iter()
            .any(|i| i.addr == 0xffff_fff0_0700_7000 && i.name == b"kernel_add")
    };
    assert!(bound(&kext));
    let mut standalone = kext.clone();
    standalone[27] &= 0x7f;
    assert!(!bound(&standalone));
}

fn macho_header(filetype: u32, ncmds: u32, sizeofcmds: u32) -> Vec<u8> {
    let mut h = Vec::new();
    for word in [0xfeed_facfu32, 0x0100_000c, 0, filetype, ncmds, sizeofcmds, 0, 0] {
        h.extend(word.to_le_bytes());
    }
    h
}

/// A fileset with no entries names nothing to load: an error, not an index panic.
#[test]
fn empty_fileset_is_an_error_not_a_panic() {
    let bytes = macho_header(12, 0, 0);
    assert!(peel(bytes, &KernelcacheChoice::Entry("all".into())).is_err());
}

/// A load command too short for the fields read from it is refused by the
/// fileset walk, so no segment read runs past its command.
#[test]
fn short_segment_command_is_refused() {
    let mut bytes = macho_header(12, 2, 40);
    bytes.extend(0x8000_0035u32.to_le_bytes());
    bytes.extend(32u32.to_le_bytes());
    bytes.extend(0u64.to_le_bytes());
    bytes.extend(0x100u64.to_le_bytes());
    bytes.extend(24u32.to_le_bytes());
    bytes.extend(0u32.to_le_bytes());
    bytes.extend(b"kern\0\0\0\0");
    bytes.extend(0x19u32.to_le_bytes());
    bytes.extend(8u32.to_le_bytes());
    bytes.resize(0x200, 0);
    bytes[0x100 + 12..0x100 + 16].copy_from_slice(&2u32.to_le_bytes());
    assert!(peel(bytes, &KernelcacheChoice::Entry("all".into())).is_err());
}

fn declared(size: u32) -> Vec<u8> {
    tlv(TAG_SEQUENCE, &[tlv(TAG_INTEGER, &[1]), tlv(TAG_INTEGER, &size.to_be_bytes())].concat())
}

/// A payload larger than its IM4P declares stops decoding at the declared size
/// instead of decoding in full and comparing afterwards.
#[test]
fn a_lying_declared_size_stops_the_decode_at_it() {
    let body = vec![b'k'; 64 << 10];
    let raw = [b"bvx-".as_slice(), &(body.len() as u32).to_le_bytes(), &body, b"bvx$"].concat();
    let err = peel(im4p(&raw, &[declared(16)]), &KernelcacheChoice::Auto).err().unwrap();
    assert!(err.contains("size limit"), "{err}");

    let mut lzss = lzss::MAGIC.to_vec();
    lzss.extend_from_slice(&0u32.to_be_bytes());
    lzss.extend_from_slice(&(1u32 << 20).to_be_bytes());
    lzss.extend_from_slice(&2u32.to_be_bytes());
    lzss.resize(0x180, 0);
    lzss.extend_from_slice(&[0x01, b'a']);
    let err = peel(im4p(&lzss, &[declared(16)]), &KernelcacheChoice::Auto).err().unwrap();
    assert!(err.contains("over the 16-byte limit"), "{err}");
}

/// Without a declared size the limit follows the input: a few bytes may not
/// claim a gigabyte.
#[test]
fn an_undeclared_size_is_capped_by_the_input() {
    assert_eq!(decode_limit(100, None), MIN_LIMIT);
    assert_eq!(decode_limit(1 << 20, None), MAX_RATIO << 20);
    assert_eq!(decode_limit(usize::MAX, None), DECODED_LIMIT);
    assert_eq!(decode_limit(100, Some(4096)), 4096);
    assert_eq!(decode_limit(100, Some(usize::MAX)), MIN_LIMIT);
    assert_eq!(decode_limit(1 << 28, Some(usize::MAX)), DECODED_LIMIT);
}

/// A declared size is capped by the same ratio as an undeclared one: a small
/// payload claiming far more output than it could hold is refused, even when
/// the claim is honest about what the stream decodes to.
#[test]
fn a_declared_size_past_the_ratio_is_not_honored() {
    assert_eq!(decode_limit(600_000, Some(1 << 30)), 600_000 * MAX_RATIO);
    let matches = (18usize << 20) / 271;
    let mut body = vec![0xe1, b'a', 0x38, 1];
    for _ in 0..matches {
        body.extend_from_slice(&[0xf0, 0xff]);
    }
    body.push(0x06);
    let n_raw = 1 + 10 + matches * 271;
    let stream =
        [b"bvxn".as_slice(), &(n_raw as u32).to_le_bytes(), &(body.len() as u32).to_le_bytes(), &body, b"bvx$"].concat();
    assert!(lzfse::decode(&stream, 0, usize::MAX).is_ok_and(|o| o.len() == n_raw));
    let err = peel(im4p(&stream, &[declared(n_raw as u32)]), &KernelcacheChoice::Auto).err().unwrap();
    assert!(err.contains("size limit"), "{err}");
}

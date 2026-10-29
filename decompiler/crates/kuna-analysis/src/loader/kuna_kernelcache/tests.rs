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
    let out = lzss::decode_stream(&stream, 9);
    assert_eq!(out, b"abcabcxyz");
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

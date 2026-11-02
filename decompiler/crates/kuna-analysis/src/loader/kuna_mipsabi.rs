//! MIPS architecture and instruction-word helpers shared by the loader.

/// Whether the object reader's architecture tag is MIPS, including an ELF32
/// n32 image (`EF_MIPS_ABI2`), which the reader tags `Mips64_N32`.
pub(crate) fn is_mips(arch: object::Architecture) -> bool {
    matches!(arch, object::Architecture::Mips | object::Architecture::Mips64 | object::Architecture::Mips64_N32)
}

/// The whole 32-bit instruction words at the start of `bytes`, in the image's
/// byte order.
pub(crate) fn words(bytes: &[u8], little: bool) -> impl Iterator<Item = u32> + '_ {
    bytes.chunks_exact(4).map(move |raw| {
        let raw = raw.try_into().unwrap();
        if little { u32::from_le_bytes(raw) } else { u32::from_be_bytes(raw) }
    })
}

//! Explain a known generic PowerPC64 decode limitation without selecting an ISA.

/// Called only after a bad-instruction decode, with `decodehalt` enabled.
/// An `isel` word proves neither that these bytes are code nor which vector ISA
/// the image uses. Suggest a supported override, never retry or change targets.
pub(crate) fn powerpc_isa_hint(
    target: &str,
    offset: u64,
    read: impl FnOnce(&mut [u8]) -> bool,
) -> Option<String> {
    let mut fields = target.split(':');
    let processor = fields.next()?;
    let endian = fields.next()?;
    let width = fields.next()?;
    let variant = fields.next()?;
    if processor != "PowerPC"
        || width != "64"
        || variant != "default"
        || !matches!(endian, "BE" | "LE")
        || offset % 4 != 0
    {
        return None;
    }
    let mut bytes = [0; 4];
    if !read(&mut bytes) {
        return None;
    }
    let word = if endian == "BE" {
        u32::from_be_bytes(bytes)
    } else {
        u32::from_le_bytes(bytes)
    };
    // isel: primary opcode 31, XO (bits 26..30) 15, reserved bit 31 zero.
    if word & 0xfc00_003f != 0x7c00_001e {
        return None;
    }
    Some(format!(
        "isel encoding is not supported by {target}; if this is code, select the matching ISA explicitly \
         (e.g. --target PowerPC:{endian}:64:A2ALT for AltiVec); target unchanged"
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn isel_fields_and_both_byte_orders() {
        for endian in ["BE", "LE"] {
            for word in [0x7c64_289eu32, 0x7c00_001e, 0x7fff_ffde] {
                let target = format!("PowerPC:{endian}:64:default:default");
                let result = powerpc_isa_hint(&target, 0x1004, |buffer| {
                    buffer.copy_from_slice(&if endian == "BE" {
                        word.to_be_bytes()
                    } else {
                        word.to_le_bytes()
                    });
                    true
                })
                .unwrap();
                assert!(result.contains(&target));
                assert!(result.contains(&format!("--target PowerPC:{endian}:64:A2ALT")));
            }
        }
    }

    #[test]
    fn wrong_target_and_alignment_do_not_read_the_image() {
        for target in [
            "x86:LE:64:default",
            "PowerPC:BE:32:default",
            "PowerPC:BE:64:A2ALT",
            "PowerPC:BE:64:A2-32addr",
            "PowerPC:XX:64:default",
            "PowerPC",
        ] {
            assert!(powerpc_isa_hint(target, 0, |_| panic!("unexpected read")).is_none());
        }
        assert!(
            powerpc_isa_hint("PowerPC:BE:64:default", 1, |_| panic!("unexpected read")).is_none()
        );
    }

    #[test]
    fn invalid_ordinary_and_unreadable_words_have_no_isa_hint() {
        for word in [0u32, 0x4e80_0020, 0x7c83_2378, 0x7c64_289f, 0x7864_0182] {
            assert!(powerpc_isa_hint("PowerPC:BE:64:default", 0, |buffer| {
                buffer.copy_from_slice(&word.to_be_bytes());
                true
            })
            .is_none());
        }
        assert!(powerpc_isa_hint("PowerPC:BE:64:default", 0, |_| false).is_none());
    }
}

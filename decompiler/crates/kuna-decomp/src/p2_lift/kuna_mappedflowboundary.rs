//! Preserve decoded ELF x86 flow at authoritative mapped-byte boundaries.

use kuna_base::address::Address;
use kuna_base::marshal::ElementId;
use kuna_sleigh::loadimage::ImageBytes;

pub const ELEM_MAPPEDFLOWBOUNDARY: ElementId = ElementId::new("mappedflowboundary", 4166);

/// Only linked little-endian ELF images with a matching resolved x86 language qualify.
pub fn eligible_elf(bytes: &[u8], language: &str) -> bool {
    if bytes.len() < 20 || &bytes[..4] != b"\x7fELF" || bytes[5] != 1 {
        return false;
    }
    let kind = u16::from_le_bytes([bytes[16], bytes[17]]);
    let machine = u16::from_le_bytes([bytes[18], bytes[19]]);
    let bits = match (bytes[4], machine) {
        (1, 3) => "32",
        (2, 62) => "64",
        _ => return false,
    };
    let mut parts = language.split(':');
    matches!(kind, 2 | 3)
        && parts.next() == Some("x86")
        && parts.next() == Some("LE")
        && parts.next() == Some(bits)
}

/// A staged loader read may return padding; only the image's live map is evidence.
pub fn mapped_start(image: &dyn ImageBytes, addr: &Address) -> bool {
    addr.get_offset()
        .checked_add(1)
        .is_some_and(|hi| image.mapped_covers(addr.get_offset(), hi))
}

/// Rebuild only mapping warnings: an earlier selection may predate overrides or overlays.
pub(crate) fn clear_stale_warnings(db: &mut crate::architecture::CommentDatabase, entry: &Address) {
    use crate::architecture::comment_type;
    db.retain_comments(|comment| {
        if &comment.func_addr != entry {
            return true;
        }
        !((comment.tp == comment_type::warningheader
            && comment.text == "WARNING: Function flows into unmapped memory")
            || (comment.tp == comment_type::warning
                && comment
                    .text
                    .starts_with("WARNING: Function flow reaches unmapped memory:")))
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eligibility_requires_linked_matching_elf_x86_metadata() {
        for (class, machine, bits) in [(1, 3u16, 32), (2, 62, 64)] {
            let mut bytes = [0u8; 20];
            bytes[..4].copy_from_slice(b"\x7fELF");
            bytes[4] = class;
            bytes[5] = 1;
            bytes[18..20].copy_from_slice(&machine.to_le_bytes());
            for kind in [2u16, 3] {
                bytes[16..18].copy_from_slice(&kind.to_le_bytes());
                assert!(eligible_elf(&bytes, &format!("x86:LE:{bits}:default:gcc")));
                for target in ["ARM:LE:32:v7", "x86:BE:32:default", "x86:LE:16:Real Mode"] {
                    assert!(!eligible_elf(&bytes, target));
                }
                assert!(!eligible_elf(
                    &bytes,
                    &format!("x86:LE:{}:default", 96 - bits)
                ));
            }
            bytes[16..18].copy_from_slice(&1u16.to_le_bytes());
            assert!(!eligible_elf(&bytes, &format!("x86:LE:{bits}:default")));
            bytes[16] = 2;
            bytes[5] = 2;
            assert!(!eligible_elf(&bytes, &format!("x86:LE:{bits}:default")));
        }
        assert!(!eligible_elf(b"", "x86:LE:32:default"));
    }
}

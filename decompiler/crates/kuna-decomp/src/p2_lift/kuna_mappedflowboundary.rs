//! Preserve decoded ELF x86 flow at authoritative mapped-byte boundaries.

use kuna_base::address::Address;
use kuna_base::marshal::ElementId;
use kuna_sleigh::loadimage::ImageBytes;
use kuna_sleigh::translate::Translate;

pub const ELEM_MAPPEDFLOWBOUNDARY: ElementId = ElementId::new("mappedflowboundary", 4167);

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

/// Preserve overlapbranch's mapped target without lifting an unmapped fall-through span.
pub(crate) fn unmapped_overlap_step(
    image: &dyn ImageBytes,
    translate: &dyn Translate,
    enabled: bool,
    addr: &Address,
    watch: &Option<(Address, Address)>,
) -> Option<i32> {
    if !enabled {
        return None;
    }
    let (fallthru, target) = watch.as_ref()?;
    if fallthru != addr
        || target.get_space()?.get_index() != addr.get_space()?.get_index()
        || target.get_offset() <= addr.get_offset()
        || !mapped_start(image, addr)
    {
        return None;
    }
    let step = translate.instruction_length(addr).ok()?;
    if !crate::kuna_overlapbranch::kuna_overlaps_pending_branch(
        enabled,
        addr.get_offset(),
        step,
        Some((fallthru.get_offset(), target.get_offset())),
    ) {
        return None;
    }
    let end = instruction_end(addr, step)?;
    if image.mapped_covers(addr.get_offset(), end) {
        return None;
    }
    let target_step = translate.instruction_length(target).ok()?;
    let target_end = instruction_end(target, target_step)?;
    if target_end == end || !image.mapped_covers(target.get_offset(), target_end) {
        return None;
    }
    Some(step)
}

/// A decode that starts on a mapped byte but whose instruction runs past the mapped run.
pub(crate) fn truncated_instruction(
    image: &dyn ImageBytes,
    translate: &dyn Translate,
    addr: &Address,
) -> bool {
    mapped_start(image, addr)
        && translate
            .instruction_length(addr)
            .ok()
            .and_then(|step| instruction_end(addr, step))
            .is_some_and(|end| !image.mapped_covers(addr.get_offset(), end))
}

pub(crate) fn truncated_warning(addr: &Address) -> String {
    let mut raw = String::new();
    let _ = addr.print_raw(&mut raw);
    format!(
        "Function flow reaches unmapped memory: the instruction at {}{raw} runs past the mapped bytes",
        addr.get_shortcut()
    )
}

fn instruction_end(addr: &Address, step: i32) -> Option<u64> {
    let len = u64::try_from(step).ok().filter(|&len| len > 0)?;
    let end = addr.get_offset().checked_add(len)?;
    (end - 1 <= addr.get_space()?.get_highest()).then_some(end)
}

/// Refresh stale mapping warnings after replacement flow succeeds.
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

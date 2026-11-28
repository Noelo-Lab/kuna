//! Address provenance of x86 ELF indirect transfers.

use crate::context::OpId;
use crate::funcdata::Funcdata;
use kuna_base::address::Address;

/// A near memory-indirect x86 transfer with an unambiguous slot address.
pub fn slot_operand(bytes: &[u8], next: u64, long_mode: bool, fixed_image: bool) -> Option<u64> {
    match bytes {
        [0xff, 0x15 | 0x25, disp @ ..] if disp.len() == 4 => {
            let offset = i32::from_le_bytes(disp.try_into().ok()?);
            if long_mode {
                Some(next.wrapping_add_signed(i64::from(offset)))
            } else {
                fixed_image.then_some(u64::from(offset as u32))
            }
        }
        [0xff, 0x14 | 0x24, 0x25, disp @ ..] if fixed_image && disp.len() == 4 => {
            let offset = i32::from_le_bytes(disp.try_into().ok()?);
            Some(if long_mode {
                i64::from(offset) as u64
            } else {
                u64::from(offset as u32)
            })
        }
        _ => None,
    }
}

/// In a position-independent image, a fixed absolute memory word can share the
/// link-time VMA of a GOT slot. Only the RIP-relative form moves with the slot.
pub(crate) fn can_bind_import(data: &Funcdata, op: OpId, slot: &Address) -> bool {
    let arch = data.get_arch();
    if !arch.x86_dynamic_elf {
        return true;
    }
    let Some(site) = data.obank().get(op).map(|op| op.get_addr()) else {
        return false;
    };
    let Some(loader) = &arch.loader else {
        return false;
    };
    let mut bytes = [0; 6];
    if loader.borrow_mut().load_fill(&mut bytes, site).is_err() {
        return false;
    }
    slot_operand(&bytes, site.get_offset().wrapping_add(6), site.get_addr_size() == 8, false) == Some(slot.get_offset())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn slot_operand_preserves_addressing_mode() {
        let rip = [0xff, 0x15, 0xf8, 0xff, 0xff, 0xff];
        let absolute = [0xff, 0x14, 0x25, 0, 0, 0x41, 0];
        assert_eq!(slot_operand(&rip, 0x410008, true, false), Some(0x410000));
        assert_eq!(slot_operand(&absolute, 0, true, false), None);
        assert_eq!(slot_operand(&absolute, 0, true, true), Some(0x410000));
        assert_eq!(slot_operand(&rip, 0, false, true), Some(0xfffffff8));
        assert_eq!(slot_operand(&rip, 0, false, false), None);
    }
}

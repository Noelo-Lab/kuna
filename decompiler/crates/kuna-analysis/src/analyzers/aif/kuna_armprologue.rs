//! Match LR-saving prologues in the address's current ARM decode mode.
use kuna_base::{address::Address, space::AddrSpace};
use kuna_decomp::architecture::Architecture;
use std::rc::Rc;

pub(super) fn matches(
    little: bool,
    arch: &Architecture,
    code_space: &Rc<AddrSpace>,
    vma: u64,
    data: &[u8],
    off: usize,
) -> bool {
    let arm = is_arm_lr_prologue(data, off, little);
    let thumb = is_thumb_lr_prologue_endian(data, off, little);
    if !arm && !thumb {
        return false;
    }
    let addr = Address::new(Rc::clone(code_space), vma);
    let tmode = arch.with_context_db_mut(|db| db.get_variable_value(b"TMode", &addr).unwrap_or(0));
    if tmode != 0 {
        thumb
    } else {
        vma & 3 == 0 && arm
    }
}

fn is_arm_lr_prologue(data: &[u8], off: usize, little: bool) -> bool {
    let Some(bytes) = data.get(off..off + 4) else {
        return false;
    };
    let bytes: [u8; 4] = bytes.try_into().unwrap();
    let word = if little {
        u32::from_le_bytes(bytes)
    } else {
        u32::from_be_bytes(bytes)
    };
    word & 0xffff_4000 == 0xe92d_4000 && word & 0xa000 == 0
}

fn is_thumb_lr_prologue_endian(data: &[u8], off: usize, little: bool) -> bool {
    if little {
        return super::is_thumb_lr_prologue(data, off);
    }
    let Some(bytes) = data.get(off..off + 2) else {
        return false;
    };
    if bytes[0] == 0xb5 {
        return true;
    }
    data.get(off..off + 4)
        .is_some_and(|bytes| bytes[0..2] == [0xe9, 0x2d] && bytes[2] & 0x40 != 0)
}

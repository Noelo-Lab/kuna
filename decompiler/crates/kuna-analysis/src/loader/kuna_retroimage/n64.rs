//! Nintendo 64 ROM in any of its three dump byte orders. The boot code
//! (IPL3, ROM 0x40..0x1000) copies the megabyte at ROM 0x1000 to the header's
//! entry address and jumps there; two boot-code revisions bias that address.

use kuna_base::error::KunaResult;
use object::SectionKind;

use super::{invalid, Bytes, RetroImage, RetroSection};

const FORMAT: &str = "N64 ROM";
pub const LANGUAGE: &str = "MIPS:BE:64:64-32addr";
const BOOT_OFFSET: u64 = 0x1000;
const BOOT_SIZE: u64 = 0x10_0000;

/// The dump's byte order, read from the PI configuration word.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Order {
    Big,
    ByteSwapped,
    Little,
}

fn order(prefix: &[u8]) -> Option<Order> {
    match prefix.get(..4)? {
        [0x80, 0x37, 0x12, 0x40] => Some(Order::Big),
        [0x37, 0x80, 0x40, 0x12] => Some(Order::ByteSwapped),
        [0x40, 0x12, 0x37, 0x80] => Some(Order::Little),
        _ => None,
    }
}

pub fn probe(prefix: &[u8]) -> bool {
    order(prefix).is_some()
}

fn crc32(data: &[u8]) -> u32 {
    let mut crc = !0u32;
    for &byte in data {
        crc ^= u32::from(byte);
        for _ in 0..8 {
            crc = (crc >> 1) ^ (0xedb8_8320 & (crc & 1).wrapping_neg());
        }
    }
    !crc
}

/// How far the boot code at ROM 0x40 moves the header's entry address down.
fn entry_bias(boot_code: &[u8]) -> u64 {
    match crc32(boot_code) {
        0x0b05_0ee0 => 0x10_0000,
        0xacc8_580a => 0x20_0000,
        _ => 0,
    }
}

pub fn parse(filename: &str, bytes: &[u8]) -> KunaResult<RetroImage> {
    let bad = |detail: &str| invalid(filename, FORMAT, detail);
    let order = order(bytes).ok_or_else(|| bad("missing PI configuration word"))?;
    let stored = (bytes.len() as u64).min(BOOT_OFFSET + BOOT_SIZE) & !3;
    if stored <= BOOT_OFFSET {
        return Err(bad("no boot segment follows the 0x1000-byte header and boot code"));
    }
    let mut rom = bytes[..stored as usize].to_vec();
    match order {
        Order::Big => {}
        Order::ByteSwapped => rom.chunks_exact_mut(2).for_each(|pair| pair.swap(0, 1)),
        Order::Little => rom.chunks_exact_mut(4).for_each(|word| word.reverse()),
    }
    let header = Bytes { data: &rom, big: true };
    let entry = u64::from(header.u32(8).ok_or_else(|| bad("truncated header"))?);
    let entry = entry
        .checked_sub(entry_bias(&rom[0x40..BOOT_OFFSET as usize]))
        .ok_or_else(|| bad("entry address underflows its boot-code bias"))?;
    let size = stored - BOOT_OFFSET;
    if entry + size > 1 << 32 {
        return Err(bad("boot segment does not fit the 32-bit address space"));
    }
    Ok(RetroImage {
        format: FORMAT,
        language: LANGUAGE,
        base: entry,
        module: rom.split_off(BOOT_OFFSET as usize),
        entry,
        sections: vec![RetroSection {
            name: ".boot".into(),
            offset: 0,
            size,
            kind: SectionKind::Text,
            file_range: Some((BOOT_OFFSET, size)),
        }],
        symbols: Vec::new(),
        tracked: Vec::new(),
        notes: Vec::new(),
    })
}

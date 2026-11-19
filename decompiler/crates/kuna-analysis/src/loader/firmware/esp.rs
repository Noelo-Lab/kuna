//! Espressif application images (ESP-IDF `esp_app_format.h`): `0xE9`, a
//! segment count, flash parameters and the entry, then on ESP32-family parts
//! a 16-byte extended header naming the chip, then the segments, each a load
//! address, a length and the bytes.
//!
//! The segments load far apart (flash is mapped 64 MiB or more from RAM), so
//! the image is kept as separate runs rather than flattened into one. Each
//! run takes the properties of the region of the chip's memory map
//! ([`super::esprom`]) it loads into.

use kuna_decomp::kuna_esprom::EspChip;
use kuna_sleigh::loadimage::section_flags;

use super::esprom::{self, RegionKind};

/// The most segments an image header can state that the ROM loader accepts.
const MAX_SEGMENTS: usize = 16;
/// `ESP_APP_DESC_MAGIC_WORD`, the first word of an application's constants.
const APP_DESC_MAGIC: [u8; 4] = [0x32, 0x54, 0xcd, 0xab];
/// `esp_chip_id_t` values of RISC-V parts.
const RISCV_CHIP_IDS: [u16; 7] = [0x05, 0x0c, 0x0d, 0x10, 0x12, 0x14, 0x17];

/// One loaded run of an image.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EspSegment {
    /// The memory-map region it loads into, when the chip has one there.
    pub region: Option<&'static str>,
    pub addr: u64,
    pub file_offset: u64,
    pub bytes: Vec<u8>,
    /// [`section_flags`].
    pub flags: u32,
}

/// A parsed application image.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EspImage {
    /// The chip the header names, when it is one with a ROM table.
    pub chip: Option<EspChip>,
    pub riscv: bool,
    pub entry: u64,
    /// The loaded runs, in file order; alignment padding is left out.
    pub segments: Vec<EspSegment>,
}

impl EspImage {
    /// The SLEIGH language of the chip's core.
    pub fn target(&self) -> &'static str {
        if self.riscv {
            "RISCV:LE:32:default"
        } else {
            "Xtensa:LE:32:default"
        }
    }

    /// What the image is, for a note.
    pub fn describe(&self) -> String {
        let chip = self.chip.map_or("unlisted chip", EspChip::as_str);
        format!("ESP application image ({chip}, {} segments, entry {:#x})", self.segments.len(), self.entry)
    }
}

fn le32(b: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_le_bytes(b.get(at..at.checked_add(4)?)?.try_into().ok()?))
}

/// Whether `bytes` can open as an application image, from its first bytes.
pub fn probe(bytes: &[u8]) -> bool {
    bytes.len() >= 16 && bytes[0] == 0xe9 && (1..=MAX_SEGMENTS).contains(&(bytes[1] as usize)) && bytes[2] <= 5
}

/// Parse `bytes` as an application image. `None` unless the header is
/// consistent, every segment lies inside the file and the entry lies inside a
/// segment.
pub fn parse(b: &[u8]) -> Option<EspImage> {
    if !probe(b) {
        return None;
    }
    parse_with(b, true).or_else(|| parse_with(b, false))
}

/// [`parse`] with (`esp32`) or without the extended header: the two layouts
/// share a magic, and only the right one walks to segments that hold the
/// entry.
fn parse_with(b: &[u8], esp32: bool) -> Option<EspImage> {
    let count = b[1] as usize;
    let entry = u64::from(le32(b, 4)?);
    let chip_id = u16::from_le_bytes([*b.get(12)?, *b.get(13)?]);
    if esp32 && (chip_id >= 0x40 || *b.get(23)? > 1) {
        return None;
    }
    let chip = if esp32 { EspChip::from_image_chip_id(chip_id) } else { Some(EspChip::Esp8266) };
    let riscv = esp32 && RISCV_CHIP_IDS.contains(&chip_id);
    let mut off = if esp32 { 24usize } else { 8 };
    let mut segments = Vec::with_capacity(count);
    for _ in 0..count {
        let addr = u64::from(le32(b, off)?);
        let len = le32(b, off.checked_add(4)?)? as usize;
        let start = off.checked_add(8)?;
        let data = b.get(start..start.checked_add(len)?)?;
        off = start + len;
        if addr == 0 || len == 0 {
            continue;
        }
        if !(0x2000_0000..0x7000_0000).contains(&addr) || addr.checked_add(len as u64)? > 0x7000_0000 {
            return None;
        }
        let region = chip.and_then(|chip| esprom::region_at(chip, addr));
        let rodata = data.starts_with(&APP_DESC_MAGIC);
        let flags = match region {
            Some(region) => esprom::load_flags(region.kind, rodata),
            None if rodata => esprom::load_flags(RegionKind::Drom, true),
            None if (0x4000_0000..0x5000_0000).contains(&addr) => section_flags::CODE,
            None => section_flags::DATA,
        };
        segments.push(EspSegment {
            region: region.map(|r| r.name),
            addr,
            file_offset: start as u64,
            bytes: data.to_vec(),
            flags,
        });
    }
    segments.iter().any(|s| s.addr <= entry && entry - s.addr < s.bytes.len() as u64).then_some(())?;
    Some(EspImage { chip, riscv, entry, segments })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn image(chip_id: u16, entry: u32, segments: &[(u32, &[u8])]) -> Vec<u8> {
        let mut b = vec![0xe9, segments.len() as u8, 2, 0x20];
        b.extend(entry.to_le_bytes());
        b.extend([0xee, 0, 0, 0]);
        b.extend(chip_id.to_le_bytes());
        b.extend([0u8; 10]);
        for (addr, data) in segments {
            b.extend(addr.to_le_bytes());
            b.extend((data.len() as u32).to_le_bytes());
            b.extend(*data);
        }
        b
    }

    #[test]
    fn segments_take_their_regions_properties() {
        let rodata = [&APP_DESC_MAGIC[..], &[0u8; 12]].concat();
        let b = image(0, 0x4008_0004, &[(0x3f40_0020, &rodata), (0x3ffb_0000, &[1; 8]), (0x4008_0000, &[2; 8]), (0x400d_0020, &[3; 8])]);
        let img = parse(&b).unwrap();
        assert_eq!((img.chip, img.riscv, img.entry), (Some(EspChip::Esp32), false, 0x4008_0004));
        let rows: Vec<_> = img.segments.iter().map(|s| (s.region.unwrap(), s.flags)).collect();
        let (code, data, ro) = (section_flags::CODE, section_flags::DATA, section_flags::READONLY);
        assert_eq!(rows, [("DROM", data | ro), ("DRAM", data), ("IRAM", code | ro), ("IROM", code | ro)]);
        assert_eq!(img.segments[0].file_offset, 32);
    }

    #[test]
    fn one_bus_flash_splits_on_the_application_descriptor() {
        let rodata = [&APP_DESC_MAGIC[..], &[0u8; 12]].concat();
        let b = image(0x0d, 0x4080_0000, &[(0x4201_0020, &rodata), (0x4080_0000, &[1; 8]), (0x4200_0020, &[2; 8])]);
        let img = parse(&b).unwrap();
        assert_eq!((img.chip, img.riscv), (Some(EspChip::Esp32C6), true));
        let (code, data, ro) = (section_flags::CODE, section_flags::DATA, section_flags::READONLY);
        assert_eq!(img.segments.iter().map(|s| s.flags).collect::<Vec<_>>(), [data | ro, code, code | ro]);
    }

    #[test]
    fn an_image_without_the_extended_header_is_an_esp8266() {
        let mut b = vec![0xe9, 1, 2, 0x20];
        b.extend(0x4010_0004u32.to_le_bytes());
        b.extend(0x4010_0000u32.to_le_bytes());
        b.extend(16u32.to_le_bytes());
        b.extend([0xffu8; 16]);
        let img = parse(&b).unwrap();
        assert_eq!((img.chip, img.target()), (Some(EspChip::Esp8266), "Xtensa:LE:32:default"));
    }

    #[test]
    fn malformed_images_are_refused() {
        let good = image(5, 0x4038_0000, &[(0x4038_0000, &[0; 8])]);
        assert!(parse(&good).is_some());
        assert!(parse(&good[..good.len() - 1]).is_none(), "truncated segment");
        let mut huge = good.clone();
        huge[28..32].copy_from_slice(&u32::MAX.to_le_bytes());
        assert!(parse(&huge).is_none(), "length past the file");
        assert!(parse(&image(5, 0x4200_0000, &[(0x4038_0000, &[0; 8])])).is_none(), "entry outside every segment");
        assert!(parse(&image(5, 0x1000, &[(0x1000, &[0; 8])])).is_none(), "address below every chip's memory");
        assert!(parse(&[0xe9, 0, 0, 0]).is_none());
        assert!(parse(&[]).is_none());
    }

    #[test]
    fn padding_segments_are_skipped() {
        let b = image(0, 0x4008_0000, &[(0, &[0; 16]), (0x4008_0000, &[1; 8])]);
        assert_eq!(parse(&b).unwrap().segments.len(), 1);
    }
}

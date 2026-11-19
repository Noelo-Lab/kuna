//! (kuna `esprom`) Espressif mask-ROM symbols and chip memory maps.
//!
//! An ESP application calls the routines in the chip's mask ROM at fixed
//! addresses the image does not carry. The per-chip tables under `esprom/`
//! name them; `scripts/gen_esprom.py` generates the tables from the ROM linker
//! scripts of ESP-IDF and ESP8266_RTOS_SDK (both Apache-2.0), and each table's
//! header states the release it was generated from.
//!
//! The memory maps are the `SOC_*_LOW`/`SOC_*_HIGH` bounds and the
//! `DR_REG_*_BASE` peripheral bases of ESP-IDF's
//! `components/soc/<chip>/include/soc/soc.h` and `reg_base.h`; the ESP8266 map
//! is that of the ESP8266 Technical Reference.

use kuna_decomp::kuna_esprom::EspChip;
use kuna_sleigh::loadimage::section_flags;

/// What one range of a chip's address space is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RegionKind {
    /// Flash mapped for data: constants.
    Drom,
    /// Flash mapped for instructions.
    Irom,
    /// Flash mapped on one bus for both.
    Flash,
    /// RAM instructions run from.
    Iram,
    /// RAM for data.
    Dram,
    /// RAM on one bus for instructions and data.
    Ram,
    /// Low-power memory that survives deep sleep.
    Rtc,
    /// The mask ROM.
    Rom,
    /// Memory-mapped registers.
    Peripheral,
}

/// One `[start, end)` range of a chip's address space.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Region {
    pub name: &'static str,
    pub start: u64,
    pub end: u64,
    pub kind: RegionKind,
}

const fn region(name: &'static str, start: u64, end: u64, kind: RegionKind) -> Region {
    Region { name, start, end, kind }
}

use RegionKind::{Dram, Drom, Flash, Iram, Irom, Peripheral, Ram, Rom, Rtc};

const ESP32: &[Region] = &[
    region("DROM", 0x3f40_0000, 0x3f80_0000, Drom),
    region("PERIPHERAL", 0x3ff0_0000, 0x3ff8_0000, Peripheral),
    region("RTC_DRAM", 0x3ff8_0000, 0x3ff8_2000, Rtc),
    region("ROM_DATA", 0x3ff9_0000, 0x3ffa_0000, Rom),
    region("DRAM", 0x3ffa_e000, 0x4000_0000, Dram),
    region("ROM", 0x4000_0000, 0x4007_0000, Rom),
    region("IRAM", 0x4008_0000, 0x400c_0000, Iram),
    region("RTC_IRAM", 0x400c_0000, 0x400c_2000, Iram),
    region("IROM", 0x400d_0000, 0x4040_0000, Irom),
    region("RTC_DATA", 0x5000_0000, 0x5000_2000, Rtc),
];

const ESP32S2: &[Region] = &[
    region("DROM", 0x3f00_0000, 0x3f40_0000, Drom),
    region("PERIPHERAL", 0x3f40_0000, 0x3f50_0000, Peripheral),
    region("RTC_DRAM", 0x3ff9_e000, 0x3ffa_0000, Rtc),
    region("ROM_DATA", 0x3ffa_0000, 0x3ffb_0000, Rom),
    region("DRAM", 0x3ffb_0000, 0x4000_0000, Dram),
    region("ROM", 0x4000_0000, 0x4002_0000, Rom),
    region("IRAM", 0x4002_0000, 0x4007_0000, Iram),
    region("RTC_IRAM", 0x4007_0000, 0x4007_2000, Iram),
    region("IROM", 0x4008_0000, 0x4080_0000, Irom),
    region("RTC_DATA", 0x5000_0000, 0x5000_2000, Rtc),
    region("PERIPHERAL", 0x6000_0000, 0x6010_0000, Peripheral),
    region("PERIPHERAL", 0x6180_0000, 0x6180_4000, Peripheral),
];

const ESP32S3: &[Region] = &[
    region("DROM", 0x3c00_0000, 0x3e00_0000, Drom),
    region("DRAM", 0x3fc8_8000, 0x3fd0_0000, Dram),
    region("ROM_DATA", 0x3ff0_0000, 0x3ff2_0000, Rom),
    region("ROM", 0x4000_0000, 0x4006_0000, Rom),
    region("IRAM", 0x4037_0000, 0x403e_0000, Iram),
    region("IROM", 0x4200_0000, 0x4400_0000, Irom),
    region("RTC_DATA", 0x5000_0000, 0x5000_2000, Rtc),
    region("PERIPHERAL", 0x6000_0000, 0x600f_e000, Peripheral),
    region("RTC_FAST", 0x600f_e000, 0x6010_0000, Rtc),
];

const ESP32C2: &[Region] = &[
    region("DROM", 0x3c00_0000, 0x3c40_0000, Drom),
    region("DRAM", 0x3fca_0000, 0x3fce_0000, Dram),
    region("ROM_DATA", 0x3ff0_0000, 0x3ff5_0000, Rom),
    region("ROM", 0x4000_0000, 0x4009_0000, Rom),
    region("IRAM", 0x4037_c000, 0x403c_0000, Iram),
    region("IROM", 0x4200_0000, 0x4240_0000, Irom),
    region("PERIPHERAL", 0x6000_0000, 0x6010_0000, Peripheral),
];

const ESP32C3: &[Region] = &[
    region("DROM", 0x3c00_0000, 0x3c80_0000, Drom),
    region("DRAM", 0x3fc8_0000, 0x3fce_0000, Dram),
    region("ROM_DATA", 0x3ff0_0000, 0x3ff2_0000, Rom),
    region("ROM", 0x4000_0000, 0x4006_0000, Rom),
    region("IRAM", 0x4037_c000, 0x403e_0000, Iram),
    region("IROM", 0x4200_0000, 0x4280_0000, Irom),
    region("RTC", 0x5000_0000, 0x5000_2000, Rtc),
    region("PERIPHERAL", 0x6000_0000, 0x6010_0000, Peripheral),
];

const ESP32C6: &[Region] = &[
    region("PERIPHERAL", 0x2000_0000, 0x3000_0000, Peripheral),
    region("ROM", 0x4000_0000, 0x4005_0000, Rom),
    region("RAM", 0x4080_0000, 0x4088_0000, Ram),
    region("FLASH", 0x4200_0000, 0x4300_0000, Flash),
    region("RTC", 0x5000_0000, 0x5000_4000, Rtc),
    region("PERIPHERAL", 0x6000_0000, 0x6010_0000, Peripheral),
];

const ESP32H2: &[Region] = &[
    region("PERIPHERAL", 0x2000_0000, 0x3000_0000, Peripheral),
    region("ROM", 0x4000_0000, 0x4002_0000, Rom),
    region("RAM", 0x4080_0000, 0x4085_0000, Ram),
    region("FLASH", 0x4200_0000, 0x4300_0000, Flash),
    region("RTC", 0x5000_0000, 0x5000_1000, Rtc),
    region("PERIPHERAL", 0x6000_0000, 0x6010_0000, Peripheral),
];

const ESP8266: &[Region] = &[
    region("PERIPHERAL", 0x3ff0_0000, 0x3ff0_1000, Peripheral),
    region("DRAM", 0x3ffe_8000, 0x4000_0000, Dram),
    region("ROM", 0x4000_0000, 0x4001_0000, Rom),
    region("IRAM", 0x4010_0000, 0x4014_0000, Iram),
    region("IROM", 0x4020_0000, 0x4030_0000, Irom),
    region("PERIPHERAL", 0x6000_0000, 0x6000_2000, Peripheral),
];

/// The chip's memory map, ascending.
pub fn regions(chip: EspChip) -> &'static [Region] {
    match chip {
        EspChip::Esp32 => ESP32,
        EspChip::Esp32S2 => ESP32S2,
        EspChip::Esp32S3 => ESP32S3,
        EspChip::Esp32C2 => ESP32C2,
        EspChip::Esp32C3 => ESP32C3,
        EspChip::Esp32C6 => ESP32C6,
        EspChip::Esp32H2 => ESP32H2,
        EspChip::Esp8266 => ESP8266,
    }
}

/// The region of `chip` holding `addr`.
pub fn region_at(chip: EspChip, addr: u64) -> Option<&'static Region> {
    regions(chip).iter().find(|r| r.start <= addr && addr < r.end)
}

/// The [`section_flags`] of bytes an image loads into a region of `kind`.
/// Flash and the RAM a chip runs instructions from hold the program's own
/// code and constants. `rodata` says a one-bus flash segment is the one that
/// holds the application's constants.
pub fn load_flags(kind: RegionKind, rodata: bool) -> u32 {
    match kind {
        RegionKind::Drom => section_flags::DATA | section_flags::READONLY,
        RegionKind::Flash if rodata => section_flags::DATA | section_flags::READONLY,
        RegionKind::Irom | RegionKind::Iram | RegionKind::Flash => section_flags::CODE | section_flags::READONLY,
        RegionKind::Ram => section_flags::CODE,
        RegionKind::Dram | RegionKind::Rtc | RegionKind::Rom | RegionKind::Peripheral => section_flags::DATA,
    }
}

/// One mask-ROM symbol.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RomSymbol {
    pub addr: u64,
    pub function: bool,
    pub name: &'static str,
}

fn table(chip: EspChip) -> &'static str {
    match chip {
        EspChip::Esp32 => include_str!("esprom/esp32.txt"),
        EspChip::Esp32S2 => include_str!("esprom/esp32s2.txt"),
        EspChip::Esp32S3 => include_str!("esprom/esp32s3.txt"),
        EspChip::Esp32C2 => include_str!("esprom/esp32c2.txt"),
        EspChip::Esp32C3 => include_str!("esprom/esp32c3.txt"),
        EspChip::Esp32C6 => include_str!("esprom/esp32c6.txt"),
        EspChip::Esp32H2 => include_str!("esprom/esp32h2.txt"),
        EspChip::Esp8266 => include_str!("esprom/esp8266.txt"),
    }
}

/// The chip's mask-ROM symbols, ascending by address, one per address.
pub fn symbols(chip: EspChip) -> impl Iterator<Item = RomSymbol> {
    table(chip).lines().filter_map(|line| {
        let mut fields = line.split(' ');
        let addr = u64::from_str_radix(fields.next()?, 16).ok()?;
        let function = match fields.next()? {
            "F" => true,
            "D" => false,
            _ => return None,
        };
        Some(RomSymbol { addr, function, name: fields.next()? })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_table_is_sorted_and_names_rom_addresses() {
        for chip in EspChip::ALL {
            let all: Vec<RomSymbol> = symbols(chip).collect();
            assert!(!all.is_empty(), "{chip:?}");
            assert_eq!(all.len(), table(chip).lines().filter(|l| !l.starts_with('#')).count(), "{chip:?}");
            assert!(all.windows(2).all(|w| w[0].addr < w[1].addr), "{chip:?}");
            for s in all.iter().filter(|s| s.function) {
                assert_eq!(region_at(chip, s.addr).map(|r| r.name), Some("ROM"), "{chip:?} {}", s.name);
            }
        }
    }

    #[test]
    fn the_maps_are_ascending_and_disjoint() {
        for chip in EspChip::ALL {
            assert!(regions(chip).windows(2).all(|w| w[0].end <= w[1].start), "{chip:?}");
        }
    }

    #[test]
    fn well_known_routines_resolve() {
        let find = |chip, name: &str| symbols(chip).find(|s| s.name == name).map(|s| (s.addr, s.function));
        assert_eq!(find(EspChip::Esp32, "memcpy"), Some((0x4000_c2c8, true)));
        assert_eq!(find(EspChip::Esp32, "ets_printf"), Some((0x4000_7d54, true)));
        assert_eq!(find(EspChip::Esp8266, "uart_rx_one_char"), Some((0x4000_3b8c, true)));
        assert_eq!(find(EspChip::Esp32C3, "ets_delay_us"), Some((0x4000_0050, true)));
    }
}

//! (kuna `esprom`) Which Espressif chip's mask ROM and memory map an image
//! runs against.
//!
//! The symbol tables and the memory maps live in the analyzer tier
//! (`kuna_analysis::loader::firmware::esprom`); what lives here is the value
//! the option carries. `auto` is resolved by the front end that peels an ESP
//! application image, from the chip id in the image header, so on every other
//! input `auto` names no chip and the option does nothing.

use kuna_base::error::{KunaError, KunaResult};

/// An Espressif chip with a mask ROM kuna has a table for.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum EspChip {
    Esp32,
    Esp32S2,
    Esp32S3,
    Esp32C2,
    Esp32C3,
    Esp32C6,
    Esp32H2,
    Esp8266,
}

impl EspChip {
    pub const ALL: [EspChip; 8] = [
        EspChip::Esp32,
        EspChip::Esp32S2,
        EspChip::Esp32S3,
        EspChip::Esp32C2,
        EspChip::Esp32C3,
        EspChip::Esp32C6,
        EspChip::Esp32H2,
        EspChip::Esp8266,
    ];

    /// The chip's name as esptool and ESP-IDF spell it.
    pub fn as_str(self) -> &'static str {
        match self {
            EspChip::Esp32 => "esp32",
            EspChip::Esp32S2 => "esp32s2",
            EspChip::Esp32S3 => "esp32s3",
            EspChip::Esp32C2 => "esp32c2",
            EspChip::Esp32C3 => "esp32c3",
            EspChip::Esp32C6 => "esp32c6",
            EspChip::Esp32H2 => "esp32h2",
            EspChip::Esp8266 => "esp8266",
        }
    }

    pub fn from_name(name: &str) -> Option<EspChip> {
        EspChip::ALL.into_iter().find(|c| c.as_str() == name)
    }

    /// The chip an ESP-IDF application image header names (`esp_chip_id_t` of
    /// `esp_app_format.h`).
    pub fn from_image_chip_id(id: u16) -> Option<EspChip> {
        Some(match id {
            0x0000 => EspChip::Esp32,
            0x0002 => EspChip::Esp32S2,
            0x0005 => EspChip::Esp32C3,
            0x0009 => EspChip::Esp32S3,
            0x000c => EspChip::Esp32C2,
            0x000d => EspChip::Esp32C6,
            0x0010 => EspChip::Esp32H2,
            _ => return None,
        })
    }

    /// The core is RISC-V rather than Xtensa.
    pub fn is_riscv(self) -> bool {
        matches!(self, EspChip::Esp32C2 | EspChip::Esp32C3 | EspChip::Esp32C6 | EspChip::Esp32H2)
    }
}

/// The resolved `option esprom auto|off|<chip>` value.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum EspRom {
    #[default]
    Auto,
    Off,
    Chip(EspChip),
}

impl EspRom {
    /// The chip whose ROM and memory map apply, if one is named.
    pub fn chip(self) -> Option<EspChip> {
        match self {
            EspRom::Chip(chip) => Some(chip),
            EspRom::Auto | EspRom::Off => None,
        }
    }

    /// The `option esprom <p1>` token for this value.
    pub fn as_str(self) -> &'static str {
        match self {
            EspRom::Auto => "auto",
            EspRom::Off => "off",
            EspRom::Chip(chip) => chip.as_str(),
        }
    }
}

/// Parse the `option esprom` argument, producing the value plus the
/// confirmation message.
pub fn parse_esprom(p1: &str) -> KunaResult<(EspRom, String)> {
    let value = match p1 {
        "auto" => EspRom::Auto,
        "off" => EspRom::Off,
        name => EspRom::Chip(EspChip::from_name(name).ok_or_else(|| {
            KunaError::parse(
                "esprom takes auto, off or a chip: esp32, esp32s2, esp32s3, esp32c2, esp32c3, esp32c6, esp32h2, esp8266",
            )
        })?),
    };
    Ok((value, format!("Espressif mask-ROM names and memory map set to {}", value.as_str())))
}

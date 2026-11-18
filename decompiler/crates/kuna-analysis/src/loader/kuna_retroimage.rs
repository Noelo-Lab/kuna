//! (kuna `retroimage`) Loaders for five executable containers the `object`
//! crate does not read: PlayStation PS-X EXE, Nintendo 64 ROM, BSD/SunOS/Linux
//! a.out, AmigaDOS hunk load files and classic Mac OS PEF.
//!
//! Each parser turns its header into one [`RetroImage`]: a relocated,
//! contiguous memory module at a base address, the language that decodes it,
//! the entry point, and the sections and symbols the container names. The
//! console maps that module the way it maps a headerless image, so call
//! discovery, naming and every option behave as they do there; only the
//! target, base and entry come from the file instead of the command line.
//!
//! Every size and offset is read from untrusted bytes: arithmetic is checked,
//! and no image may map more than [`MAX_MODULE`] bytes.

use kuna_base::error::{KunaError, KunaResult};
use kuna_sleigh::loadimage::section_flags;
use object::SectionKind;

pub mod aout;
pub mod hunk;
pub mod n64;
pub mod pef;
pub mod psx;

#[cfg(test)]
mod tests;

/// The most bytes one image may map.
pub const MAX_MODULE: u64 = 1 << 28;

/// The longest symbol name a parser keeps.
const MAX_NAME: usize = 512;

/// The number of leading bytes [`is_retro_image`] reads.
pub const RETRO_PROBE_LEN: usize = 32;

/// A mapped range of the module, at `offset` bytes from the image base.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RetroSection {
    pub name: String,
    pub offset: u64,
    pub size: u64,
    pub kind: SectionKind,
    /// `(file offset, size)` of the bytes the file holds for the range.
    pub file_range: Option<(u64, u64)>,
}

impl RetroSection {
    /// The loader flags the section kind implies.
    pub fn flags(&self) -> u32 {
        match self.kind {
            SectionKind::Text => section_flags::CODE,
            SectionKind::ReadOnlyData => section_flags::DATA | section_flags::READONLY,
            _ => section_flags::DATA,
        }
    }
}

/// A name the container gives an address, at `offset` bytes from the base.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RetroSymbol {
    pub offset: u64,
    pub name: String,
    pub code: bool,
}

/// One loaded container.
#[derive(Clone, Debug)]
pub struct RetroImage {
    /// The container's name, for messages.
    pub format: &'static str,
    /// The SLEIGH language id that decodes the image.
    pub language: &'static str,
    /// Address of the first module byte.
    pub base: u64,
    /// The image as loaded: file bytes with relocations applied, zero fill
    /// where the container asks for uninitialized memory.
    pub module: Vec<u8>,
    /// Address of the entry point.
    pub entry: u64,
    pub sections: Vec<RetroSection>,
    pub symbols: Vec<RetroSymbol>,
    /// Registers the container fixes for the whole image (`gp`, the TOC).
    pub tracked: Vec<(&'static str, u64)>,
    /// What the loader could not honor, one sentence each.
    pub notes: Vec<String>,
}

/// Whether `prefix` opens one of the five containers.
pub fn is_retro_image(prefix: &[u8]) -> bool {
    psx::probe(prefix) || n64::probe(prefix) || pef::probe(prefix) || hunk::probe(prefix) || aout::probe(prefix)
}

/// Parse `bytes` as the container [`is_retro_image`] recognized.
pub fn parse(filename: &str, bytes: &[u8]) -> KunaResult<RetroImage> {
    let image = if psx::probe(bytes) {
        psx::parse(filename, bytes)
    } else if n64::probe(bytes) {
        n64::parse(filename, bytes)
    } else if pef::probe(bytes) {
        pef::parse(filename, bytes)
    } else if hunk::probe(bytes) {
        hunk::parse(filename, bytes)
    } else if aout::probe(bytes) {
        aout::parse(filename, bytes)
    } else {
        Err(KunaError::lowlevel(format!("File: {filename} : not a recognized executable container")))
    }?;
    debug_assert!(image.entry >= image.base && image.entry - image.base < image.module.len() as u64);
    Ok(image)
}

/// Every language id a parser can select, for the gate that proves each
/// resolves in the vendored SLEIGH database.
pub fn retro_language_ids() -> Vec<&'static str> {
    let mut ids = vec![psx::LANGUAGE, n64::LANGUAGE, hunk::LANGUAGE, pef::LANGUAGE];
    ids.extend(aout::MACHINES.iter().map(|machine| machine.language));
    ids.sort_unstable();
    ids.dedup();
    ids
}

pub(crate) fn invalid(filename: &str, format: &str, detail: &str) -> KunaError {
    KunaError::lowlevel(format!("File: {filename} : invalid {format} image: {detail}"))
}

/// Bounds-checked fixed-endian reads over the file.
#[derive(Clone, Copy)]
pub(crate) struct Bytes<'a> {
    pub data: &'a [u8],
    pub big: bool,
}

impl<'a> Bytes<'a> {
    pub fn u8(&self, at: u64) -> Option<u8> {
        self.data.get(usize::try_from(at).ok()?).copied()
    }

    pub fn u16(&self, at: u64) -> Option<u16> {
        let raw: [u8; 2] = self.slice(at, 2)?.try_into().ok()?;
        Some(if self.big { u16::from_be_bytes(raw) } else { u16::from_le_bytes(raw) })
    }

    pub fn u32(&self, at: u64) -> Option<u32> {
        let raw: [u8; 4] = self.slice(at, 4)?.try_into().ok()?;
        Some(if self.big { u32::from_be_bytes(raw) } else { u32::from_le_bytes(raw) })
    }

    pub fn slice(&self, at: u64, len: u64) -> Option<&'a [u8]> {
        let start = usize::try_from(at).ok()?;
        let end = start.checked_add(usize::try_from(len).ok()?)?;
        self.data.get(start..end)
    }

    /// The NUL-terminated string at `at`, without its terminator; nothing
    /// when no terminator ends it within the longest name a symbol may have.
    pub fn cstr(&self, at: u64) -> Option<&'a [u8]> {
        let tail = self.data.get(usize::try_from(at).ok()?..)?;
        let len = tail.iter().take(MAX_NAME + 1).position(|&byte| byte == 0)?;
        Some(&tail[..len])
    }
}

/// Read the 32-bit word at `at` of `module`, in the image's byte order.
pub(crate) fn word(module: &[u8], at: u64, big: bool) -> Option<u32> {
    Bytes { data: module, big }.u32(at)
}

/// Add `delta` to the 32-bit word at `at` of `module`; `false` when out of range.
pub(crate) fn add_word(module: &mut [u8], at: u64, delta: u32, big: bool) -> bool {
    let Some(old) = word(module, at, big) else { return false };
    let new = old.wrapping_add(delta);
    let at = at as usize;
    module[at..at + 4].copy_from_slice(&if big { new.to_be_bytes() } else { new.to_le_bytes() });
    true
}

/// A symbol name as text: printable ASCII only, else nothing.
pub(crate) fn symbol_name(raw: &[u8]) -> Option<String> {
    (!raw.is_empty() && raw.len() <= MAX_NAME && raw.iter().all(|byte| (0x21..0x7f).contains(byte)))
        .then(|| String::from_utf8_lossy(raw).into_owned())
}

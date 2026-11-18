//! PlayStation PS-X EXE: a 0x800-byte header, then one text image the BIOS
//! copies to `t_addr` before jumping to `pc0` with `gp` set to `gp0`.

use kuna_base::error::KunaResult;
use object::SectionKind;

use super::{invalid, Bytes, RetroImage, RetroSection, MAX_MODULE};

const FORMAT: &str = "PS-X EXE";
const HEADER: u64 = 0x800;
pub const LANGUAGE: &str = "MIPS:LE:32:default";

pub fn probe(prefix: &[u8]) -> bool {
    prefix.starts_with(b"PS-X EXE")
}

pub fn parse(filename: &str, bytes: &[u8]) -> KunaResult<RetroImage> {
    let bad = |detail: &str| invalid(filename, FORMAT, detail);
    let file = Bytes { data: bytes, big: false };
    let field = |at| file.u32(at).map(u64::from).ok_or_else(|| bad("truncated 0x800-byte header"));
    let (pc0, gp0, t_addr, t_size) = (field(0x10)?, field(0x14)?, field(0x18)?, field(0x1c)?);
    let (b_addr, b_size) = (field(0x28)?, field(0x2c)?);
    let stored = (bytes.len() as u64).saturating_sub(HEADER);
    let text_len = if t_size == 0 { stored } else { t_size.min(stored) };
    if text_len == 0 {
        return Err(bad("no text follows the header"));
    }
    if text_len > MAX_MODULE || t_addr + text_len > 1 << 32 {
        return Err(bad("text does not fit the 32-bit address space"));
    }
    if pc0 < t_addr || pc0 - t_addr >= text_len {
        return Err(bad("pc0 is outside the text the file holds"));
    }
    let mut notes = Vec::new();
    if t_size > stored {
        notes.push(format!("the header declares {t_size:#x} text bytes but the file holds {stored:#x}"));
    }
    let mut module = file.slice(HEADER, text_len).ok_or_else(|| bad("truncated text"))?.to_vec();
    let mut sections = vec![RetroSection {
        name: ".text".into(),
        offset: 0,
        size: text_len,
        kind: SectionKind::Text,
        file_range: Some((HEADER, text_len)),
    }];
    let text_end = t_addr + text_len;
    if b_size != 0 && b_addr >= text_end && b_addr + b_size - t_addr <= MAX_MODULE && b_addr + b_size <= 1 << 32 {
        module.resize((b_addr + b_size - t_addr) as usize, 0);
        sections.push(RetroSection {
            name: ".bss".into(),
            offset: b_addr - t_addr,
            size: b_size,
            kind: SectionKind::UninitializedData,
            file_range: None,
        });
    }
    Ok(RetroImage {
        format: FORMAT,
        language: LANGUAGE,
        base: t_addr,
        module,
        entry: pc0,
        sections,
        symbols: Vec::new(),
        tracked: if gp0 != 0 { vec![("gp", gp0)] } else { Vec::new() },
        notes,
    })
}

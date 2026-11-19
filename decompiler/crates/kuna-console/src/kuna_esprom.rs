//! (kuna `esprom`) Load an Espressif application image and apply its chip's
//! mask-ROM names and memory map.
//!
//! The image loads as the raw image its segments are
//! ([`kuna_analysis::loader::firmware::esp`]): each one a section with the
//! properties of the memory region it loads into, the header's entry the seed
//! raw discovery widens from. What the chip adds is applied at the analysis
//! commit, where the option is known: every ROM function a named function
//! symbol, every ROM data symbol a named global, every peripheral range
//! volatile. A ROM function is a symbol only, so it names its call sites and
//! is not part of the function inventory.

use std::rc::Rc;

use kuna_analysis::loader::firmware::esp::{self, EspImage};
use kuna_analysis::loader::firmware::esprom::{self, RegionKind};
use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_decomp::dtype::type_metatype::TYPE_UNKNOWN;
use kuna_decomp::kuna_esprom::{EspChip, EspRom};
use kuna_decomp::varnode::varnode_flags;
use kuna_sleigh::loadimage::RawLoadImage;

use crate::engine::{ConsoleProgram, ProgramImageMetadata, ProgramSectionMetadata};

/// The file at `path` when it parses as an Espressif application image; the
/// whole file is read only once its first bytes pass the probe.
pub(crate) fn file_esp_image(path: &str) -> Option<EspImage> {
    use std::io::Read as _;
    let mut head = [0u8; 16];
    std::fs::File::open(path).ok()?.read_exact(&mut head).ok()?;
    if !esp::probe(&head) {
        return None;
    }
    esp::parse(&std::fs::read(path).ok()?)
}

/// The segmented raw image of `image`, its base, and the section table a
/// project export prints.
pub(crate) fn raw_image(path: &str, image: &EspImage) -> KunaResult<(RawLoadImage, u64, ProgramImageMetadata)> {
    let runs = image.segments.iter().map(|s| (s.addr, s.bytes.clone(), s.flags)).collect();
    let (loader, base) = RawLoadImage::from_segments(path, runs)
        .ok_or_else(|| KunaError::lowlevel("ESP application image: its segments overlap"))?;
    let sections = image
        .segments
        .iter()
        .map(|s| ProgramSectionMetadata {
            name: s.region.unwrap_or("SEGMENT").to_string(),
            vma: s.addr,
            size: s.bytes.len() as u64,
            kind: section_kind(s.flags).to_string(),
            file_offset: Some(s.file_offset),
            file_size: Some(s.bytes.len() as u64),
            flags: s.flags,
        })
        .collect();
    Ok((loader, base, ProgramImageMetadata { entry: Some(image.entry), sections }))
}

fn section_kind(flags: u32) -> &'static str {
    use kuna_sleigh::loadimage::section_flags::{CODE, READONLY};
    match (flags & CODE != 0, flags & READONLY != 0) {
        (true, _) => "Text",
        (false, true) => "ReadOnlyData",
        (false, false) => "Data",
    }
}

/// The chip in force: the option's, or under `auto` the loaded image's.
fn chip_in_force(prog: &ConsoleProgram) -> Option<EspChip> {
    match prog.arch().esprom {
        EspRom::Auto => prog.arch().esp_image_chip,
        value => value.chip(),
    }
}

/// Apply the chip's ROM names and memory map once, before the commit maps
/// any symbol over the ranges it paints.
pub(crate) fn apply(prog: &mut ConsoleProgram) -> KunaResult<()> {
    let Some(chip) = chip_in_force(prog) else { return Ok(()) };
    if prog.esprom_applied.replace(chip) == Some(chip) {
        return Ok(());
    }
    let Some(space) = prog.arch().manage().get_default_code_space().cloned() else { return Ok(()) };
    let highest = space.get_highest();
    let mapped = prog.sections();
    let is_mapped = |start: u64, end: u64| mapped.iter().any(|&(vma, size, _)| vma < end && start < vma.saturating_add(size));
    for region in esprom::regions(chip).iter().filter(|r| r.kind == RegionKind::Peripheral) {
        if region.end - 1 > highest || is_mapped(region.start, region.end) {
            continue;
        }
        let first = Address::new(Rc::clone(&space), region.start);
        let last_open = Address::new(Rc::clone(&space), region.end);
        prog.arch_mut().symboltab.set_property_range(varnode_flags::volatil, &first, &last_open);
    }
    let symbols: Vec<esprom::RomSymbol> = esprom::symbols(chip).filter(|s| s.addr <= highest).collect();
    for (i, symbol) in symbols.iter().enumerate() {
        let addr = Address::new(Rc::clone(&space), symbol.addr);
        if symbol.function {
            prog.name_external_function(&addr, symbol.name)?;
            continue;
        }
        let room = symbols.get(i + 1).map_or(4, |next| next.addr - symbol.addr);
        let size = if room >= 4 { 4 } else { room.max(1) as i32 };
        name_rom_data(prog, &addr, symbol.name, size)?;
    }
    Ok(())
}

/// A named, untyped global of `size` bytes at `addr`, unless a symbol already
/// covers the address.
fn name_rom_data(prog: &mut ConsoleProgram, addr: &Address, name: &str, size: i32) -> KunaResult<()> {
    let num_spaces = prog.arch().manage().num_spaces() as i32;
    let invalid = Address::new_invalid();
    let Some(global) = prog.arch().symboltab.get_global_scope() else { return Ok(()) };
    let symboltab = &prog.arch().symboltab;
    if symboltab.find_function(global, addr).is_some() || symboltab.find_container(global, addr, size, &invalid).is_some() {
        return Ok(());
    }
    let ct = prog.arch().types().get_base(size, TYPE_UNKNOWN)?;
    let arch = prog.arch_mut();
    let (scope, base) = arch.symboltab.find_create_scope_from_symbol_name(name, "::", None, num_spaces)?;
    let (sid, _) = arch.symboltab.add_symbol_mapped(scope, &base, ct, addr, &invalid)?;
    arch.symboltab.set_attribute(sid, varnode_flags::namelock);
    Ok(())
}

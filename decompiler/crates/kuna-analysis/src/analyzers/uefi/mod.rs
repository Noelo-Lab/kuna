//! (kuna `uefi`) UEFI module markup for PE images with an EFI subsystem.
//!
//! On an EFI application or driver (PE `Subsystem` 10..13) the pass:
//! - types the entry point `EFI_STATUS _ModuleEntryPoint(EFI_HANDLE ImageHandle,
//!   EFI_SYSTEM_TABLE *SystemTable)`, with `EFI_SYSTEM_TABLE`, `EFI_BOOT_SERVICES`
//!   and `EFI_RUNTIME_SERVICES` built from the UEFI specification ([`types`]), so
//!   `SystemTable->BootServices->LocateProtocol(...)` reads as a member call;
//! - names every 16-byte value in a data section that equals a GUID declared by
//!   EDK2 (`gEfiLoadedImageProtocolGuid`, ...) and types it `EFI_GUID`;
//! - types the service tables a core module defines statically, found by their
//!   `EFI_TABLE_HEADER` signature (`BOOTSERV`, `RUNTSERV`, `IBI SYST`,
//!   `DXE_SERV`, `PEI SERV`);
//! - names the ARM PEI-services getters (`mrc p15,0,r0,c13,c0,2; bx lr` and
//!   `mrs x0, tpidr_el0; ret`) and types their result `EFI_PEI_SERVICES **`.
//!
//! It then follows the service tables through the code ([`flow`]): the
//! globals the entry stores them in (`gST`, `gBS`, `gRT`, `gSmst`, ...) are
//! named and typed, a protocol or PPI lookup types its interface out-parameter
//! from its GUID ([`protocols`]), an installed interface instance gets its
//! struct and the functions it points at get their member's name and
//! prototype, and a PEIM or a standalone-MM module gets the entry prototype of
//! its phase. Beside an index `kuna unpack --protocols` wrote ([`index`]), a
//! lookup is annotated with the module of the volume that installs the
//! interface.
//!
//! The pass decodes the module, so it runs at the commit point
//! ([`crate::passes::run_uefi`]) rather than in the load-time list.

mod flow;
pub mod index;
mod protocols;
mod types;

use crate::pass::{
    AnalysisCtx, AnalysisOutput, AnalysisPass, CommentFact, FidMatch, Phase, SymFact, SymKind, TypedDataFact,
};
use kuna_decomp::kuna_uefiprotocols::InstalledInterface;
use object::{Object, ObjectSection, SectionKind};
use std::collections::BTreeMap;
use std::sync::OnceLock;

/// The UEFI module pass.
pub struct UefiPass;

const SIGNATURES: &[(&[u8; 8], &str, &str)] = &[
    (b"IBI SYST", "EFI_SYSTEM_TABLE", "mSystemTable"),
    (b"BOOTSERV", "EFI_BOOT_SERVICES", "mBootServices"),
    (b"RUNTSERV", "EFI_RUNTIME_SERVICES", "mRuntimeServices"),
    (b"DXE_SERV", "EFI_DXE_SERVICES", "mDxeServices"),
    (b"PEI SERV", "EFI_PEI_SERVICES", "mPeiServices"),
];

const PEI_GETTERS: &[(&[u8], u64, &str)] = &[
    (&[0x1d, 0xee, 0x50, 0x0f, 0x70, 0x47], 2, "ArmReadTpidrurw"),
    (&[0x50, 0x0f, 0x1d, 0xee, 0x1e, 0xff, 0x2f, 0xe1], 4, "ArmReadTpidrurw"),
    (&[0x40, 0xd0, 0x3b, 0xd5, 0xc0, 0x03, 0x5f, 0xd6], 4, "ArmReadTpidrurw"),
];

const PEI_SERVICES_GETTER: &str = "EFI_PEI_SERVICES**";

/// GUID bytes (in memory order) to the EDK2 name.
pub fn guid_names() -> &'static BTreeMap<[u8; 16], &'static str> {
    static DB: OnceLock<BTreeMap<[u8; 16], &'static str>> = OnceLock::new();
    DB.get_or_init(|| {
        include_str!("guids.txt")
            .lines()
            .filter(|l| !l.starts_with('#'))
            .filter_map(|l| {
                let (hex, name) = l.split_once(' ')?;
                let mut raw = [0u8; 16];
                for (i, b) in raw.iter_mut().enumerate() {
                    *b = u8::from_str_radix(hex.get(2 * i..2 * i + 2)?, 16).ok()?;
                }
                Some((raw, name))
            })
            .filter(|(raw, _)| raw.iter().filter(|&&b| b == 0).count() < 8)
            .collect()
    })
}

/// The PE optional-header `Subsystem` of `bytes`, when it is a PE image.
pub fn pe_subsystem(bytes: &[u8]) -> Option<u16> {
    let u16_at = |at: usize| bytes.get(at..at + 2).map(|b| u16::from_le_bytes([b[0], b[1]]));
    if bytes.get(0..2) != Some(b"MZ") {
        return None;
    }
    let pe = u32::from_le_bytes(bytes.get(0x3c..0x40)?.try_into().ok()?) as usize;
    if bytes.get(pe..pe.checked_add(4)?) != Some(b"PE\0\0") {
        return None;
    }
    u16_at(pe.checked_add(24 + 68)?)
}

/// Whether `bytes` is a PE image for 32-bit ARM (machine `ARM`, `THUMB` or
/// `ARMNT`), whose code pointers carry the Thumb bit.
pub(crate) fn is_arm32(bytes: &[u8]) -> bool {
    let pe = bytes.get(0x3c..0x40).map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]) as usize);
    let machine = pe.and_then(|pe| bytes.get(pe.checked_add(4)?..pe.checked_add(6)?));
    matches!(machine.map(|b| u16::from_le_bytes([b[0], b[1]])), Some(0x1c0 | 0x1c2 | 0x1c4))
}

/// Whether `bytes` is a PE image with an EFI subsystem (application, boot or
/// runtime driver, ROM).
pub fn is_efi_image(bytes: &[u8]) -> bool {
    matches!(pe_subsystem(bytes), Some(10..=13))
}

/// Hands out each name once, numbering the repeats.
#[derive(Default)]
struct Names(BTreeMap<String, u32>);

impl Names {
    fn unique(&mut self, name: &str) -> String {
        let n = self.0.entry(name.to_string()).or_insert(0);
        *n += 1;
        if *n == 1 { name.to_string() } else { format!("{name}_{}", *n - 1) }
    }
}

impl AnalysisPass for UefiPass {
    fn phase(&self) -> Phase {
        Phase::P1
    }

    fn id(&self) -> &'static str {
        "uefi"
    }

    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        let mut out = AnalysisOutput::default();
        if !is_efi_image(ctx.bytes) {
            return out;
        }
        let ptr = if ctx.file.is_64() { 8 } else { 4 };
        let Some(efi) = types::EfiTypes::build(ctx.arch.types(), ptr) else { return out };
        let mut names = Names::default();

        let getters = pei_getters(ctx, &efi, &mut out);
        let thumb_bit = u64::from(is_arm32(ctx.bytes));
        let entry = ctx.file.entry() & !thumb_bit;
        let volume = ctx.image_path.and_then(index::Index::beside);
        let forced = volume.as_ref().and_then(|v| v.kind_of(ctx.image_path?));
        let foreign = volume.as_ref().map(index::Index::foreign).unwrap_or_default();
        let (kind, found) = match flow::Walk::new(ctx, &efi, &foreign, &getters, entry) {
            Some(walk) if entry != 0 => walk.run(forced),
            _ => (forced.unwrap_or(flow::Kind::Dxe), flow::Found::default()),
        };
        if std::env::var_os("KUNA_UEFI_DEBUG").is_some() {
            eprintln!(
                "uefi: {kind:?}, {} service calls, {} globals, {} installs, {} functions, {} call prototypes",
                found.service_calls(),
                found.globals.len(),
                found.installs.len(),
                found.functions.len(),
                found.call_decls.len()
            );
            for line in &found.trace {
                eprintln!("uefi:   {line}");
            }
        }
        if entry != 0 {
            function_fact(&mut out, &efi, entry, "_ModuleEntryPoint", Some(kind.entry_decl()));
        }

        let mut claimed: Vec<(u64, u64)> = Vec::new();
        for (addr, install) in &found.installs {
            if let Some(t) = efi.get(&install.interface) {
                claimed.push((*addr, addr.saturating_add(t.get_size().max(1) as u64)));
            }
        }
        for addr in found.descriptors.keys() {
            claimed.push((*addr, addr.saturating_add(3 * ptr as u64)));
        }
        static_data(ctx, &efi, &mut names, &mut claimed, &mut out);
        flow_facts(&efi, &found, entry, &claimed, &mut names, &mut out);
        if let Some(volume) = &volume {
            for (func, site, guid) in &found.lookups {
                if let Some(text) = volume.installer_note(guid, ctx.image_path) {
                    out.comments.push(CommentFact { func_addr: *func, addr: *site, text });
                }
            }
        }
        out
    }
}

/// Name the ARM PEI-services getters and return them with their result type.
fn pei_getters(ctx: &AnalysisCtx, efi: &types::EfiTypes, out: &mut AnalysisOutput) -> Vec<(u64, &'static str)> {
    let mut found = Vec::new();
    for section in ctx.file.sections().filter(|s| s.kind() == SectionKind::Text) {
        let Ok(data) = section.data() else { continue };
        for (pattern, step, name) in PEI_GETTERS {
            for off in (0..data.len().saturating_sub(pattern.len() - 1)).step_by(*step as usize) {
                if &data[off..off + pattern.len()] != *pattern {
                    continue;
                }
                let addr = section.address() + off as u64;
                function_fact(out, efi, addr, name, Some("EFI_PEI_SERVICES**()"));
                found.push((addr, PEI_SERVICES_GETTER));
            }
        }
    }
    found
}

/// The facts read off the data sections alone: statically defined service
/// tables, by their header signature, and EDK2 GUID values.
fn static_data(
    ctx: &AnalysisCtx,
    efi: &types::EfiTypes,
    names: &mut Names,
    claimed: &mut Vec<(u64, u64)>,
    out: &mut AnalysisOutput,
) {
    for section in ctx.file.sections() {
        if !matches!(section.kind(), SectionKind::Data | SectionKind::ReadOnlyData) {
            continue;
        }
        let Ok(data) = section.data() else { continue };
        let base = section.address();
        for off in (0..data.len().saturating_sub(7)).step_by(8) {
            let Some((_, ty, name)) = SIGNATURES.iter().find(|(sig, ..)| &data[off..off + 8] == *sig) else {
                continue;
            };
            let Some(t) = efi.get(ty) else { continue };
            let size = t.get_size() as u64;
            if off as u64 + size > data.len() as u64 {
                continue;
            }
            claimed.push((base + off as u64, base + off as u64 + size));
            out.typed_data.push(TypedDataFact { addr: base + off as u64, name: names.unique(name), type_: t });
        }
        let Some(guid) = efi.get("EFI_GUID") else { continue };
        let db = guid_names();
        let mut off = 0usize;
        while off + 16 <= data.len() {
            let addr = base + off as u64;
            let raw: [u8; 16] = data[off..off + 16].try_into().expect("16 bytes");
            match db.get(&raw) {
                Some(name) if !claimed.iter().any(|&(lo, hi)| addr < hi && addr + 16 > lo) => {
                    out.typed_data.push(TypedDataFact { addr, name: names.unique(name), type_: guid.clone() });
                    off += 16;
                }
                _ => off += 4,
            }
        }
    }
}

/// The facts the walk over the code found.
fn flow_facts(
    efi: &types::EfiTypes,
    found: &flow::Found,
    entry: u64,
    claimed: &[(u64, u64)],
    names: &mut Names,
    out: &mut AnalysisOutput,
) {
    for (addr, name) in &found.guids {
        if let (Some(guid), false) = (efi.get("EFI_GUID"), out.typed_data.iter().any(|d| d.addr == *addr)) {
            out.typed_data.push(TypedDataFact { addr: *addr, name: names.unique(name), type_: guid });
        }
    }
    for (addr, install) in &found.installs {
        let Some(type_) = efi.get(&install.interface) else { continue };
        let name = names.unique(&format!("g{}", protocols::stem(&install.interface)));
        out.typed_data.push(TypedDataFact { addr: *addr, name, type_ });
        out.uefi_installs.push(InstalledInterface {
            guid: install.guid,
            addr: *addr,
            interface: install.interface.clone(),
            fields: efi.defined_fields(&install.interface).unwrap_or_default(),
            members: install.members.clone(),
        });
    }
    for (addr, descriptor) in &found.descriptors {
        let st = if descriptor.notify { "EFI_PEI_NOTIFY_DESCRIPTOR" } else { "EFI_PEI_PPI_DESCRIPTOR" };
        if let Some(type_) = efi.get(st) {
            out.typed_data.push(TypedDataFact { addr: *addr, name: names.unique(&descriptor.name), type_ });
        }
    }
    let width = efi.ptr as u64;
    for (addr, spec) in &found.globals {
        if claimed.iter().any(|&(lo, hi)| *addr < hi && addr.saturating_add(width) > lo) {
            continue;
        }
        let Some(type_) = efi.parse(spec) else { continue };
        out.typed_data.push(TypedDataFact { addr: *addr, name: names.unique(&global_name(spec)), type_ });
    }
    for (addr, named) in &found.functions {
        if *addr != entry {
            function_fact(out, efi, *addr, &names.unique(&named.name), named.decl.as_deref());
        }
    }
    for (site, decl) in &found.call_decls {
        if let Some(pieces) = efi.pieces("", decl) {
            out.call_prototypes.push((*site, pieces));
        }
    }
}

/// The name EDK2 gives the global holding a pointer of type `spec`.
fn global_name(spec: &str) -> String {
    match spec {
        "EFI_HANDLE" => "gImageHandle".to_string(),
        "EFI_SYSTEM_TABLE*" => "gST".to_string(),
        "EFI_BOOT_SERVICES*" => "gBS".to_string(),
        "EFI_RUNTIME_SERVICES*" => "gRT".to_string(),
        "EFI_DXE_SERVICES*" => "gDS".to_string(),
        "EFI_SMM_SYSTEM_TABLE2*" => "gSmst".to_string(),
        "EFI_MM_SYSTEM_TABLE*" => "gMmst".to_string(),
        "EFI_PEI_SERVICES**" => "gPeiServices".to_string(),
        other => format!("m{}", protocols::stem(other.trim_end_matches('*'))),
    }
}

/// Name the function at `addr` and, when `decl` parses, lock its prototype.
fn function_fact(out: &mut AnalysisOutput, efi: &types::EfiTypes, addr: u64, name: &str, decl: Option<&str>) {
    out.symbols.push(SymFact { addr, name: name.to_string(), kind: SymKind::Function });
    out.fid_names.push(FidMatch { addr, name: name.to_string(), by_signature: false });
    if let Some(pieces) = decl.and_then(|d| efi.pieces(name, d)) {
        out.prototypes_at.push((addr, pieces));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guid_db_parses() {
        let db = guid_names();
        assert!(db.len() > 900);
        let loaded_image = [
            0xa1, 0x31, 0x1b, 0x5b, 0x62, 0x95, 0xd2, 0x11, 0x8e, 0x3f, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b,
        ];
        assert_eq!(db.get(&loaded_image), Some(&"gEfiLoadedImageProtocolGuid"));
    }

    fn make_factory(ptr: i32) -> kuna_decomp::dtype::TypeFactoryImpl {
        let types = kuna_decomp::dtype::TypeFactoryImpl::new();
        types.set_default_alignment_map();
        types.set_max_basetype_size(8);
        types.setup_sizes(Some(ptr), ptr, 4);
        types.cache_core_types().unwrap();
        types
    }

    #[test]
    fn efi_tables_have_spec_offsets() {
        let factory = make_factory(8);
        let efi = types::EfiTypes::build(&factory, 8).expect("tables build");
        let st = efi.get("EFI_SYSTEM_TABLE").unwrap();
        assert_eq!(st.get_size(), 120);
        let bs = efi.get("EFI_BOOT_SERVICES").unwrap();
        assert_eq!(bs.get_size(), 24 + 44 * 8);
        let factory32 = make_factory(4);
        let efi32 = types::EfiTypes::build(&factory32, 4).expect("tables build");
        assert_eq!(efi32.get("EFI_SYSTEM_TABLE").unwrap().get_size(), 72);
    }

    #[test]
    fn every_generated_interface_builds() {
        for ptr in [8, 4] {
            let factory = make_factory(ptr);
            let efi = types::EfiTypes::build(&factory, ptr).expect("tables build");
            for (guid, interface) in &protocols::table().interfaces {
                assert!(efi.get(interface).is_some(), "{interface} ({guid}) does not build at {ptr}-byte pointers");
                assert!(efi.layout(interface).is_some(), "{interface} has no layout");
            }
        }
    }

    #[test]
    fn interfaces_have_header_offsets() {
        let factory = make_factory(8);
        let efi = types::EfiTypes::build(&factory, 8).expect("tables build");
        let at = |name: &str, member: &str| efi.layout(name).unwrap().iter().find(|m| m.name == member).unwrap().offset;
        assert_eq!(at("EFI_DISK_IO_PROTOCOL", "WriteDisk"), 16);
        assert_eq!(at("EFI_DRIVER_BINDING_PROTOCOL", "DriverBindingHandle"), 40);
        assert_eq!(at("EFI_SMM_SYSTEM_TABLE2", "SmmLocateProtocol"), 0xd0);
        assert_eq!(at("EFI_MM_SYSTEM_TABLE", "MmIo.Io.Write"), 0x48);
        assert_eq!(at("EFI_PEI_SERVICES", "LocatePpi"), 0x28);
        let this = efi.pieces("", "EFI_STATUS(EFI_DISK_IO_PROTOCOL* This)").unwrap();
        assert_eq!(this.intypes[0].get_ptr_to().unwrap().get_size(), 24);
    }

    #[test]
    fn a_defined_struct_does_not_replace_a_table() {
        let factory = make_factory(8);
        let efi = types::EfiTypes::build(&factory, 8).expect("tables build");
        efi.define("EFI_DISK_IO_PROTOCOL", vec![("Field_00".into(), "UINTN".into())]);
        assert_eq!(efi.get("EFI_DISK_IO_PROTOCOL").unwrap().get_size(), 24);
        efi.define("PROTOCOL_12345678", vec![("Field_00".into(), "UINTN".into()), ("Function_08".into(), "FUNC".into())]);
        assert_eq!(efi.get("PROTOCOL_12345678").unwrap().get_size(), 16);
        assert_eq!(efi.defined_fields("PROTOCOL_12345678").unwrap().len(), 2);
    }
}

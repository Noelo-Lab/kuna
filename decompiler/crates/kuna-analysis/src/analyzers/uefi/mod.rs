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
//! The pass states what the image's header and bytes say; it does not track
//! the values the entry stores into `gBS`/`gST`/`gRT`.

mod types;

use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, FidMatch, Phase, SymFact, SymKind, TypedDataFact};
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
    if bytes.get(pe..pe + 4) != Some(b"PE\0\0") {
        return None;
    }
    u16_at(pe + 24 + 68)
}

/// Whether `bytes` is a PE image with an EFI subsystem (application, boot or
/// runtime driver, ROM).
pub fn is_efi_image(bytes: &[u8]) -> bool {
    matches!(pe_subsystem(bytes), Some(10..=13))
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

        let thumb_bit = u64::from(ctx.file.architecture() == object::Architecture::Arm);
        let entry = ctx.file.entry() & !thumb_bit;
        let decl = "EFI_STATUS(EFI_HANDLE ImageHandle, EFI_SYSTEM_TABLE* SystemTable)";
        if entry != 0 {
            if let Some(pieces) = efi.pieces("_ModuleEntryPoint", decl) {
                out.symbols.push(SymFact { addr: entry, name: "_ModuleEntryPoint".to_string(), kind: SymKind::Function });
                out.fid_names.push(FidMatch { addr: entry, name: "_ModuleEntryPoint".to_string(), by_signature: false });
                out.prototypes_at.push((entry, pieces));
            }
        }

        let mut claimed: Vec<(u64, u64)> = Vec::new();
        let mut used_names: BTreeMap<String, u32> = BTreeMap::new();
        let mut unique = |name: &str| {
            let n = used_names.entry(name.to_string()).or_insert(0);
            *n += 1;
            if *n == 1 { name.to_string() } else { format!("{name}_{}", *n - 1) }
        };
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
                out.typed_data.push(TypedDataFact { addr: base + off as u64, name: unique(name), type_: t });
            }
            let Some(guid) = efi.get("EFI_GUID") else { continue };
            let db = guid_names();
            let mut off = 0usize;
            while off + 16 <= data.len() {
                let addr = base + off as u64;
                let raw: [u8; 16] = data[off..off + 16].try_into().expect("16 bytes");
                match db.get(&raw) {
                    Some(name) if !claimed.iter().any(|&(lo, hi)| addr < hi && addr + 16 > lo) => {
                        out.typed_data.push(TypedDataFact { addr, name: unique(name), type_: guid.clone() });
                        off += 16;
                    }
                    _ => off += 4,
                }
            }
        }

        if let Some(getter) = efi.pointer_to("EFI_PEI_SERVICES").and_then(|p| {
            ctx.arch.types().get_type_pointer(ptr, p, 1).ok()
        }) {
            for section in ctx.file.sections().filter(|s| s.kind() == SectionKind::Text) {
                let Ok(data) = section.data() else { continue };
                for (pattern, step, name) in PEI_GETTERS {
                    for off in (0..data.len().saturating_sub(pattern.len() - 1)).step_by(*step as usize) {
                        if &data[off..off + pattern.len()] != *pattern {
                            continue;
                        }
                        let addr = section.address() + off as u64;
                        let mut pieces = kuna_decomp::fspec::PrototypePieces {
                            name: (*name).to_string(),
                            first_var_arg_slot: -1,
                            ..Default::default()
                        };
                        pieces.outtype = Some(getter.clone());
                        out.symbols.push(SymFact { addr, name: (*name).to_string(), kind: SymKind::Function });
                        out.fid_names.push(FidMatch { addr, name: (*name).to_string(), by_signature: false });
                        out.prototypes_at.push((addr, pieces));
                    }
                }
            }
        }
        out
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
}

//! The interface struct of each protocol and PPI GUID.
//!
//! `protocols.txt` is generated from the EDK2 headers by
//! `scripts/gen_uefi_protocols.py` (EDK2 edk2-stable202505, MdePkg and
//! MdeModulePkg, BSD-2-Clause-Patent). A `= STRUCT [gGuid ...]` line opens a
//! struct and names the GUIDs whose interface it is; each following line is one
//! member, `Name SPEC`, in the vocabulary of [`super::types`].

use std::collections::BTreeMap;
use std::sync::OnceLock;

/// The parsed table.
pub(super) struct Table {
    /// Struct name to its members.
    pub(super) structs: BTreeMap<&'static str, Vec<(&'static str, &'static str)>>,
    /// EDK2 GUID name to the struct that is its interface.
    pub(super) interfaces: BTreeMap<&'static str, &'static str>,
}

pub(super) fn table() -> &'static Table {
    static TABLE: OnceLock<Table> = OnceLock::new();
    TABLE.get_or_init(|| parse(include_str!("protocols.txt")))
}

fn parse(text: &'static str) -> Table {
    let mut table = Table { structs: BTreeMap::new(), interfaces: BTreeMap::new() };
    let mut current: Option<&'static str> = None;
    for line in text.lines().filter(|l| !l.starts_with('#')) {
        if let Some(head) = line.strip_prefix("= ") {
            let mut words = head.split(' ');
            let Some(name) = words.next() else { continue };
            for guid in words {
                table.interfaces.entry(guid).or_insert(name);
            }
            table.structs.entry(name).or_default();
            current = Some(name);
        } else if let (Some(name), Some((field, spec))) = (current, line.split_once(' ')) {
            table.structs.entry(name).or_default().push((field, spec));
        }
    }
    table.structs.retain(|_, fields| !fields.is_empty());
    table
}

/// The interface struct of the GUID EDK2 calls `guid_name`.
pub(super) fn interface_of(guid_name: &str) -> Option<&'static str> {
    table().interfaces.get(guid_name).copied()
}

/// `EFI_DISK_IO2_PROTOCOL` as `DiskIo2`: the stem EDK2 gives the functions and
/// instances of an interface.
pub(super) fn stem(interface: &str) -> String {
    let core = interface.strip_prefix("EFI_").unwrap_or(interface);
    let core = core.strip_suffix("_PROTOCOL").or_else(|| core.strip_suffix("_PPI")).unwrap_or(core);
    core.split('_')
        .filter(|w| !w.is_empty())
        .map(|w| {
            let (head, tail) = w.split_at(1);
            format!("{head}{}", tail.to_ascii_lowercase())
        })
        .collect()
}

/// `gEdkiiFooProtocolGuid` as `EDKII_FOO_PROTOCOL`: the struct name EDK2 would
/// give the interface of a GUID whose header this build does not know.
pub(super) fn struct_name_for(guid_name: &str) -> String {
    let core = guid_name.strip_prefix('g').unwrap_or(guid_name);
    let core = core.strip_suffix("Guid").unwrap_or(core);
    let mut out = String::with_capacity(core.len() + 8);
    let mut prev_lower = false;
    for c in core.chars() {
        if c.is_ascii_uppercase() && prev_lower {
            out.push('_');
        }
        prev_lower = c.is_ascii_lowercase() || c.is_ascii_digit();
        out.push(c.to_ascii_uppercase());
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_parses() {
        let t = table();
        assert!(t.interfaces.len() > 300);
        assert_eq!(interface_of("gEfiDiskIoProtocolGuid"), Some("EFI_DISK_IO_PROTOCOL"));
        let disk_io = &t.structs["EFI_DISK_IO_PROTOCOL"];
        assert_eq!(disk_io[0], ("Revision", "UINT64"));
        assert!(disk_io[1].1.starts_with("fn EFI_STATUS(EFI_DISK_IO_PROTOCOL* This"));
        assert_eq!(interface_of("gEfiSimpleTextOutProtocolGuid"), Some("EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL"));
    }

    #[test]
    fn names_follow_edk2() {
        assert_eq!(stem("EFI_DISK_IO2_PROTOCOL"), "DiskIo2");
        assert_eq!(stem("EFI_PEI_READ_ONLY_VARIABLE2_PPI"), "PeiReadOnlyVariable2");
        assert_eq!(struct_name_for("gEdkiiFooBar2ProtocolGuid"), "EDKII_FOO_BAR2_PROTOCOL");
    }
}

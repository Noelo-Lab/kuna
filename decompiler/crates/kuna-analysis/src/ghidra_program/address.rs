use super::*;

const PROGRAM_INFO: &str = "Program Information.";
const PROGRAM_DB_VERSION: &str = "32";

pub(super) struct ProgramInfo {
    layout: Layout,
    language_id: String,
    image_base: u64,
}

pub(super) struct Metadata {
    pub(super) layout: Layout,
    pub(super) language_id: String,
    pub(super) image_base: u64,
    pub(super) elf_original_base: Option<u64>,
    pub(super) executable_sha256: String,
}

impl ProgramInfo {
    pub(super) fn read(db: &Database<'_>, used: &mut HashSet<i32>) -> KunaResult<Self> {
        let program = require_table(db, "Program", 0, (Str, "Key"), &[(Str, "Value")])?;
        let mut language_id = None;
        let mut image_offset = None;
        let mut version = None;
        db.visit_program_strings(program, used, |key, mut columns| {
            let Some(FieldValue::Str(value)) = columns.pop() else {
                return Err(bad("invalid Program record"));
            };
            match key {
                "Language ID" => language_id = value,
                "Image Offset" => image_offset = Some(value),
                "DB Version" => version = value,
                _ => {}
            }
            Ok(())
        })?;
        let version = version.ok_or_else(|| bad("missing Program DB Version"))?;
        if version != PROGRAM_DB_VERSION {
            return Err(bad(format_args!(
                "unsupported Program DB Version {version:?}; only saved version {PROGRAM_DB_VERSION} \
                 (Ghidra 12.1.2) is supported"
            )));
        }
        let image_base = match image_offset {
            Some(offset) => parse_hex(offset.as_deref(), "Program Image Offset")?,
            None => 0,
        };
        let language_id = language_id.ok_or_else(|| bad("missing Program Language ID"))?;
        let layout = Layout::from_language(&language_id)?;
        if image_base > layout.mask() {
            return Err(bad("unsupported address map: image base is outside the default space"));
        }
        Ok(Self { layout, language_id, image_base })
    }
}

impl Metadata {
    pub(super) fn read(
        db: &Database<'_>, used: &mut HashSet<i32>, program: ProgramInfo,
    ) -> KunaResult<Self> {
        let ProgramInfo { layout, language_id, image_base } = program;

        let properties = require_table(db, "Property Table", 0, (Str, "Property Name"), &[
            (Str, "Value"), (Byte, "Type"),
        ])?;
        let mut executable_sha256 = None;
        let mut elf_original_base = None;
        db.visit_program_strings(properties, used, |key, mut columns| {
            let property = key.strip_prefix(PROGRAM_INFO);
            if !matches!(property, Some("Executable SHA256" | "ELF Original Image Base")) {
                return Ok(());
            }
            // OptionType.STRING_TYPE = 2 in the stored ordinal encoding.
            let [FieldValue::Str(value), FieldValue::Byte(2)] = columns.as_mut_slice() else {
                return Err(bad(format_args!("invalid string option {key}")));
            };
            if property == Some("Executable SHA256") {
                executable_sha256 = value.take();
            } else {
                elf_original_base = Some(parse_hex(value.as_deref(), key)?);
            }
            Ok(())
        })?;
        let executable_sha256 = executable_sha256.filter(|digest| {
            digest.len() == 64 && digest.bytes().all(|b| b.is_ascii_hexdigit())
        }).ok_or_else(|| bad("missing or invalid Executable SHA256; cannot verify whole-file import"))?;

        Ok(Self { layout, language_id, image_base, elf_original_base, executable_sha256 })
    }
}

fn parse_hex(text: Option<&str>, property: &str) -> KunaResult<u64> {
    text.and_then(|text| u64::from_str_radix(text.strip_prefix("0x").unwrap_or(text), 16).ok())
        .ok_or_else(|| bad(format_args!("invalid hexadecimal {property}")))
}

pub(super) fn check_image_base(expected: u64, actual: u64) -> KunaResult<()> {
    if actual != expected {
        return Err(bad(format_args!(
            "rebased ProgramDB is unsupported: original image base {expected:#x}, project base {actual:#x}"
        )));
    }
    Ok(())
}

/// The address map persists space names and segments, but not address widths,
/// addressable units, or the default space ID. These layouts are the flat,
/// byte-addressed `ram` defaults declared by the stock processor specifications.
#[derive(Clone, Copy)]
pub(super) struct Layout {
    pub(super) bits: u8,
}

impl Layout {
    fn from_language(id: &str) -> KunaResult<Self> {
        let mut parts = id.split(':');
        let layout = match (parts.next(), parts.next(), parts.next(), parts.next(), parts.next()) {
            (Some("x86"), Some("LE"), Some("32"), Some("default"), None) => 32,
            (Some("x86"), Some("LE"), Some("64"), Some("default" | "compat32"), None) => 64,
            (Some("ARM"), Some("LE" | "BE"), Some("32"),
                Some("v4" | "v4t" | "v5" | "v5t" | "v6" | "v7" | "v8" | "v8T" | "v8-m" | "Cortex"),
                None) => 32,
            (Some("ARM"), Some("LEBE"), Some("32"), Some("v7LEInstruction" | "v8LEInstruction"), None) => 32,
            (Some("AARCH64"), Some("LE" | "BE"), Some("64"), Some("v8A" | "AppleSilicon"), None) => 64,
            (Some("AARCH64"), Some("LE" | "BE"), Some("32"), Some("ilp32"), None) => 64,
            (Some("MIPS" | "PowerPC"), Some("LE" | "BE"), Some("32"), Some("default"), None) => 32,
            (Some("MIPS" | "PowerPC"), Some("LE" | "BE"), Some("64"), Some("default"), None) => 64,
            (Some("RISCV"), Some("LE"), Some("32"), Some("default" | "RV32G" | "RV32GC" | "RV32I" | "RV32IC" | "RV32IMC"), None) => 32,
            (Some("RISCV"), Some("LE"), Some("64"), Some("default" | "RV64G" | "RV64GC" | "RV64I" | "RV64IC"), None) => 64,
            (Some("68000"), Some("BE"), Some("32"), Some("default" | "Coldfire" | "MC68020" | "MC68030"), None) => 32,
            _ => return Err(bad(format_args!("unsupported address map: language {id:?} has no known flat layout"))),
        };
        Ok(Self { bits: layout })
    }

    pub(super) fn mask(self) -> u64 {
        u64::MAX >> (64 - self.bits)
    }
}

pub(super) struct AddressMap {
    pub(super) layout: Layout,
    image_base: u64,
    // Entries for nondefault, deleted, and overlay spaces retain their slots.
    segments: Vec<Option<u64>>,
}

impl AddressMap {
    pub(super) fn read(
        db: &Database<'_>, metadata: &Metadata, used: &mut HashSet<i32>,
    ) -> KunaResult<Self> {
        let table = require_table(db, "Overlay Spaces", 1, (Long, "ID"), &[
            (Str, "Overlay Space Name"), (Str, "Base Space Name"),
        ])?;
        let mut overlays = BTreeSet::new();
        db.visit_records(table, used, |mut record| {
            let [FieldValue::Str(Some(name)), FieldValue::Str(Some(base))] = record.columns.as_mut_slice() else {
                return Err(bad("invalid Overlay Spaces record"));
            };
            if name.is_empty() || base.is_empty() || !overlays.insert(std::mem::take(name)) {
                return Err(bad("invalid or duplicate overlay name"));
            }
            Ok(())
        })?;
        if overlays.contains("ram") {
            return Err(bad("unsupported address map: overlay shadows the stock default space"));
        }

        let table = require_table(db, "ADDRESS MAP", 1, (Long, "Key"), &[
            (Str, "Space Name"), (Int, "Segment"), (Bool, "Deleted"),
        ])?;
        let mut segments = Vec::new();
        let mut default_segments = BTreeSet::new();
        db.visit_records(table, used, |record| {
            let [FieldValue::Str(Some(space)), FieldValue::Int(segment), FieldValue::Bool(deleted)] =
                record.columns.as_slice()
            else {
                return Err(bad("invalid ADDRESS MAP record"));
            };
            if usize::try_from(record.key).ok() != Some(segments.len()) || record.key > 0x0fff_ffff
                || space.is_empty()
            {
                return Err(bad("unsupported address map: invalid or noncontiguous base IDs"));
            }
            let base = if !*deleted && space == "ram" {
                let base = u64::from(*segment as u32) << 32;
                if base > metadata.layout.mask() || !default_segments.insert(base) {
                    return Err(bad("unsupported address map: invalid or duplicate default-space segment"));
                }
                Some(base)
            } else {
                None
            };
            segments.push(base);
            Ok(())
        })?;
        Ok(Self { layout: metadata.layout, image_base: metadata.image_base, segments })
    }

    pub(super) fn decode(&self, key: i64) -> KunaResult<Option<u64>> {
        let key = key as u64;
        let kind = key >> 60;
        match kind {
            1 | 2 => {
                let slot = ((key >> 32) & 0x0fff_ffff) as usize;
                let Some(base) = self.segments.get(slot) else {
                    return Err(bad(format_args!("unsupported address map: key {key:#018x} references absent base {slot}")));
                };
                let Some(base) = *base else { return Ok(None) };
                let offset = base | (key & u64::from(u32::MAX));
                if kind == 2 {
                    Ok(Some(offset.wrapping_add(self.image_base) & self.layout.mask()))
                } else {
                    Ok(Some(offset))
                }
            }
            // Register, stack, external, variable, hash, and no-address keys
            // cannot identify an entry in the default memory space.
            3..=7 => Ok(None),
            15 if key >= u64::MAX - 1 => Ok(None),
            _ => Err(bad(format_args!("unsupported address map key {key:#018x}"))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn address_keys_select_the_recorded_base_and_apply_image_offset_only_to_relocatables() {
        let map = AddressMap {
            layout: Layout::from_language("x86:LE:64:default").unwrap(),
            image_base: 0x0040_0000,
            segments: vec![Some(0x0000_0001_0000_0000), None],
        };
        assert_eq!(map.decode(0x1000_0000_0000_0190).unwrap(), Some(0x0000_0001_0000_0190));
        assert_eq!(map.decode(0x2000_0000_0000_0190).unwrap(), Some(0x0000_0001_0040_0190));
        assert_eq!(map.decode(0x2000_0001_0000_0190).unwrap(), None);
        assert_eq!(map.decode(0x5000_0000_0000_0001).unwrap(), None);
        assert!(map.decode(0x2000_0002_0000_0190).is_err());
        assert!(map.decode(0x0040_0190).is_err());
    }

    #[test]
    fn relocatables_wrap_at_the_default_space_width() {
        let map = AddressMap {
            layout: Layout::from_language("x86:LE:32:default").unwrap(),
            image_base: 0x0040_0000,
            segments: vec![Some(0)],
        };
        assert_eq!(map.decode(0x2000_0000_ffff_0000).unwrap(), Some(0x003f_0000));
        assert!(Layout::from_language("x86:LE:16:Real Mode").is_err());
    }
}

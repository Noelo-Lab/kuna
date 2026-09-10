//! Bounded UEFI TE image loader.
//!
//! A TE image retains the PE/COFF section table but removes the DOS, PE, and
//! optional headers. Its `StrippedSize` records that removal. The file offset of
//! each section is therefore `PointerToRawData - (StrippedSize - 40)`, while its
//! loaded address remains `ImageBase + VirtualAddress`. File offset zero is
//! retained at `ImageBase + StrippedSize - 40` up to the adjusted
//! `BaseOfCode`. Parsing the container directly preserves those coordinates and
//! avoids manufacturing a synthetic PE whose headers could disagree with the TE
//! metadata.

use std::cell::Cell;
use std::rc::Rc;

use kuna_base::address::{Address, RangeList};
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::space::AddrSpace;
use kuna_base::types::Wrap;
use kuna_sleigh::loadimage::{section_flags, LoadImage, LoadImageFunc, LoadImageSection};
use object::pe::{
    IMAGE_FILE_MACHINE_ARM, IMAGE_FILE_MACHINE_THUMB, IMAGE_SUBSYSTEM_EFI_APPLICATION,
    IMAGE_SUBSYSTEM_EFI_ROM,
};
use object::{Architecture, SectionFlags, SectionKind};

use crate::loader::format::pe::coff_section_bits;
use crate::loader::le::{read_u16, read_u32, read_u64};
use crate::loader::format::pe::coff_section_kind;
use crate::loadimage_object::{
    compose_language_id, is_arm32_language, pe_arm_mode_policy, pe_machine_architecture,
    ArmModePolicy, ObjectSectionMetadata, PE_MACHINES,
};

const TE_HEADER_SIZE: usize = 40;
const SECTION_HEADER_SIZE: usize = 40;

#[derive(Clone)]
struct MachineInfo {
    primary: String,
    fallback: Option<String>,
    bits: u8,
    arm32: bool,
    mode_policy: Option<ArmModePolicy>,
}

/// The UEFI calling-convention model per architecture: the x64 binding is the
/// Microsoft convention, every other binding is the architecture's default
/// (C/cdecl for IA-32, AAPCS for AArch32, AAPCS64 for AArch64).
fn uefi_compiler_model(arch: Architecture) -> Option<&'static str> {
    match arch {
        Architecture::X86_64 => Some("windows"),
        _ => None,
    }
}

/// Language selection for a TE machine word, composed by the same
/// `compose_language_id` the object loader uses so a TE and a PE carrying the
/// same machine select the same SLEIGH variant; only the compiler model is the
/// UEFI binding's rather than the Windows one. The ARM decode-mode policy is
/// the object loader's one table too, read for the UEFI container family.
fn machine_info(machine: u16) -> Option<MachineInfo> {
    let arch = pe_machine_architecture(machine)?;
    let primary = compose_language_id(arch, "LE", uefi_compiler_model(arch))?;
    let fallback = compose_language_id(arch, "LE", None).filter(|id| *id != primary);
    let bits = arch.address_size()?.bytes() * 8;
    Some(MachineInfo {
        primary,
        fallback,
        bits,
        arm32: arch == Architecture::Arm,
        mode_policy: pe_arm_mode_policy(machine, true),
    })
}

/// Whether `prefix` opens a UEFI TE image: the `VZ` signature, an EFI
/// subsystem, and a `StrippedSize` that leaves room for the header it counts.
/// The signature alone is two ASCII letters an unrelated file can open with;
/// claiming the format on that misdiagnoses the input for every command that
/// then rejects it. The machine word is deliberately not part of the claim: a
/// TE for a machine kuna has no binding for is still a TE, and the loader's
/// own "unsupported machine" message is the answer it should get.
pub fn is_te_image(prefix: &[u8]) -> bool {
    let Some(head) = prefix.get(..TE_PROBE_LEN) else {
        return false;
    };
    let subsystem = u16::from(head[5]);
    let stripped_size = u16::from_le_bytes([head[6], head[7]]);
    &head[..2] == b"VZ"
        && (IMAGE_SUBSYSTEM_EFI_APPLICATION..=IMAGE_SUBSYSTEM_EFI_ROM).contains(&subsystem)
        && u64::from(stripped_size) > TE_HEADER_SIZE as u64
}

/// The number of leading bytes [`is_te_image`] reads.
pub const TE_PROBE_LEN: usize = 8;

/// Every `(primary, fallback)` language id pair the TE loader can select, for
/// the cross-crate gate that proves each resolves in the vendored SLEIGH
/// database.
pub fn te_language_ids() -> Vec<(String, Option<String>)> {
    PE_MACHINES
        .iter()
        .filter_map(|&machine| machine_info(machine))
        .map(|info| (info.primary, info.fallback))
        .collect()
}

/// The adjusted file offset of the first section that carries raw data — the
/// end of the bytes the file retains for the header. `None` when no section is
/// file-backed, which leaves the header's file extent to its mapped one.
fn first_raw_offset(bytes: &[u8], section_count: usize, stripped_offset: u64) -> Option<u64> {
    (0..section_count)
        .filter_map(|index| {
            let offset = TE_HEADER_SIZE + index * SECTION_HEADER_SIZE;
            let virtual_size = read_u32(bytes, offset + 8)? as u64;
            let raw_size = read_u32(bytes, offset + 16)? as u64;
            let raw_pointer = read_u32(bytes, offset + 20)? as u64;
            let mapped_size = if virtual_size == 0 { raw_size } else { virtual_size };
            (raw_size != 0 && mapped_size != 0).then_some(())?;
            raw_pointer.checked_sub(stripped_offset)
        })
        .min()
}

#[derive(Debug, Clone)]
struct TeSection {
    name: Option<String>,
    vma: u64,
    size: u64,
    data: Vec<u8>,
    flags: u32,
    kind: SectionKind,
}

/// A parsed TE container exposed through the decompiler's generic image API.
#[derive(Debug)]
pub struct TeLoadImage {
    filename: String,
    archtype: Vec<u8>,
    fallback_archtype: Option<Vec<u8>>,
    sections: Vec<TeSection>,
    entry: u64,
    machine_thumb_hint: bool,
    entry_thumb_hint: bool,
    spaceid: Option<Rc<AddrSpace>>,
    cursection: Cell<usize>,
}

impl TeLoadImage {
    pub fn from_bytes(filename: &str, bytes: &[u8]) -> KunaResult<Self> {
        Self::from_bytes_with_target(filename, bytes, "")
    }

    pub fn from_bytes_with_target(filename: &str, bytes: &[u8], target: &str) -> KunaResult<Self> {
        let target = crate::loadimage_object::explicit_language_target(target).unwrap_or("");
        if bytes.len() < TE_HEADER_SIZE {
            return Err(Self::invalid(filename, "truncated 40-byte header"));
        }
        if &bytes[..2] != b"VZ" {
            return Err(Self::invalid(filename, "missing VZ signature"));
        }

        let machine = read_u16(bytes, 2).unwrap();
        let info = machine_info(machine).ok_or_else(|| {
            Self::invalid(
                filename,
                &format!("unsupported machine value 0x{machine:04x}"),
            )
        })?;
        validate_target(filename, target, info.bits)?;

        let section_count = bytes[4] as usize;
        if section_count == 0 {
            return Err(Self::invalid(filename, "section count is zero"));
        }
        let subsystem = u16::from(bytes[5]);
        if !(IMAGE_SUBSYSTEM_EFI_APPLICATION..=IMAGE_SUBSYSTEM_EFI_ROM).contains(&subsystem) {
            return Err(Self::invalid(
                filename,
                &format!("unsupported EFI subsystem value {subsystem}"),
            ));
        }
        let table_size = section_count
            .checked_mul(SECTION_HEADER_SIZE)
            .and_then(|size| size.checked_add(TE_HEADER_SIZE))
            .ok_or_else(|| Self::invalid(filename, "section table size overflows"))?;
        if table_size > bytes.len() {
            return Err(Self::invalid(filename, "truncated section table"));
        }

        let stripped_size = read_u16(bytes, 6).unwrap() as u64;
        if stripped_size <= TE_HEADER_SIZE as u64 {
            return Err(Self::invalid(
                filename,
                "StrippedSize must exceed the TE header size",
            ));
        }
        let stripped_offset = stripped_size - TE_HEADER_SIZE as u64;
        let entry_rva = read_u32(bytes, 8).unwrap() as u64;
        let base_of_code = read_u32(bytes, 12).unwrap() as u64;
        if base_of_code <= stripped_size {
            return Err(Self::invalid(
                filename,
                "BaseOfCode must exceed StrippedSize",
            ));
        }
        let image_base = read_u64(bytes, 16).unwrap();
        if info.bits == 32 && image_base > u32::MAX as u64 {
            return Err(Self::invalid(
                filename,
                "32-bit image base exceeds its address width",
            ));
        }
        // The retained region has two extents, and they coincide only when the
        // image was linked with SectionAlignment == FileAlignment. It is MAPPED
        // from file offset zero up to the adjusted `BaseOfCode`, but the file
        // BACKS it only up to the first section's adjusted raw offset; the rest
        // is alignment slack the loader zero-fills. Deriving the file length
        // from the RVA delta refuses an ordinary 0x200/0x1000-aligned image.
        let header_size = base_of_code - stripped_offset;
        if header_size < table_size as u64 {
            return Err(Self::invalid(
                filename,
                "retained header range does not contain the section table",
            ));
        }
        // A file offset below the section table is reported per section, by the
        // loop below, which can name the section it belongs to. The extent is
        // clamped to the mapped one so no record ever holds more bytes than it
        // maps; with no file-backed section to derive it from, the file itself
        // is the only bound the image gives.
        let header_file_size = match first_raw_offset(bytes, section_count, stripped_offset) {
            Some(offset) => {
                let offset = offset.min(header_size);
                if offset > bytes.len() as u64 {
                    return Err(Self::invalid(
                        filename,
                        "retained header range extends past end of file",
                    ));
                }
                offset
            }
            None => header_size.min(bytes.len() as u64),
        };
        let header_vma = image_base
            .checked_add(stripped_offset)
            .ok_or_else(|| Self::invalid(filename, "header load address overflows"))?;
        let header_end = header_vma
            .checked_add(header_size)
            .ok_or_else(|| Self::invalid(filename, "header load range overflows"))?;
        if info.bits == 32 && header_end.saturating_sub(1) > u32::MAX as u64 {
            return Err(Self::invalid(
                filename,
                "header exceeds the 32-bit address space",
            ));
        }

        let mut sections = Vec::with_capacity(section_count + 1);
        let mut rva_ranges = Vec::with_capacity(section_count);
        let mut file_ranges = Vec::with_capacity(section_count);
        let mut previous_end = 0u64;
        for index in 0..section_count {
            let offset = TE_HEADER_SIZE + index * SECTION_HEADER_SIZE;
            let name_bytes = &bytes[offset..offset + 8];
            let name_len = name_bytes.iter().position(|&byte| byte == 0).unwrap_or(8);
            let name = String::from_utf8_lossy(&name_bytes[..name_len]).into_owned();
            let virtual_size = read_u32(bytes, offset + 8).unwrap() as u64;
            let virtual_address = read_u32(bytes, offset + 12).unwrap() as u64;
            let raw_size = read_u32(bytes, offset + 16).unwrap() as u64;
            let raw_pointer = read_u32(bytes, offset + 20).unwrap() as u64;
            let characteristics = read_u32(bytes, offset + 36).unwrap();
            // PE/COFF VirtualSize is the in-memory extent. SizeOfRawData may be
            // larger solely because the file is padded to FileAlignment; only
            // when VirtualSize is zero does the raw extent define the mapping.
            let mapped_size = if virtual_size == 0 {
                raw_size
            } else {
                virtual_size
            };
            let initialized_size = raw_size.min(mapped_size);
            if mapped_size == 0 {
                continue;
            }
            if virtual_address < stripped_offset {
                return Err(Self::invalid(
                    filename,
                    &format!("section {index} virtual address underflows the stripped offset"),
                ));
            }
            let rva_end = virtual_address
                .checked_add(mapped_size)
                .ok_or_else(|| Self::invalid(filename, "section virtual range overflows"))?;
            if index != 0 && virtual_address < previous_end {
                return Err(Self::invalid(
                    filename,
                    "section virtual ranges overlap or are not ordered",
                ));
            }
            previous_end = rva_end;

            let vma = image_base
                .checked_add(virtual_address)
                .ok_or_else(|| Self::invalid(filename, "section load address overflows"))?;
            let vma_end = vma
                .checked_add(mapped_size)
                .ok_or_else(|| Self::invalid(filename, "section load range overflows"))?;
            if info.bits == 32 && vma_end.saturating_sub(1) > u32::MAX as u64 {
                return Err(Self::invalid(
                    filename,
                    "section exceeds the 32-bit address space",
                ));
            }
            if vma < header_end && header_vma < vma_end {
                return Err(Self::invalid(
                    filename,
                    &format!("section {index} overlaps the retained TE headers"),
                ));
            }

            let data = if raw_size == 0 {
                Vec::new()
            } else {
                if raw_pointer < stripped_offset {
                    return Err(Self::invalid(
                        filename,
                        &format!("section {index} file offset underflows the stripped offset"),
                    ));
                }
                let file_offset = raw_pointer - stripped_offset;
                if file_offset < table_size as u64 {
                    return Err(Self::invalid(
                        filename,
                        &format!("section {index} data overlaps the retained TE headers"),
                    ));
                }
                let file_end = file_offset
                    .checked_add(raw_size)
                    .ok_or_else(|| Self::invalid(filename, "section file range overflows"))?;
                if file_end > bytes.len() as u64 {
                    return Err(Self::invalid(
                        filename,
                        &format!("section {index} data extends past end of file"),
                    ));
                }
                if file_ranges
                    .iter()
                    .any(|&(start, end)| file_offset < end && start < file_end)
                {
                    return Err(Self::invalid(filename, "section file ranges overlap"));
                }
                file_ranges.push((file_offset, file_end));
                bytes[file_offset as usize..file_offset as usize + initialized_size as usize]
                    .to_vec()
            };

            let kind = coff_section_kind(characteristics);
            let flags = coff_section_bits(kind, SectionFlags::Coff { characteristics });

            sections.push(TeSection {
                name: Some(name),
                vma,
                size: mapped_size,
                data,
                flags,
                kind,
            });
            rva_ranges.push((virtual_address, rva_end, initialized_size, flags));
        }

        // The low entry bit is Thumb state only for the language that decodes
        // the image, which an explicit target may make something other than
        // the machine word's: the console folds it by the resolved language,
        // and so does the raw loader, so the same rule applies here.
        let decodes_as_arm32 = if target.is_empty() {
            info.arm32
        } else {
            is_arm32_language(target)
        };
        let normalized_entry_rva = if decodes_as_arm32 {
            entry_rva & !1
        } else {
            entry_rva
        };
        let entry_section = rva_ranges.iter().find(|(start, end, _, flags)| {
            *flags & section_flags::CODE != 0
                && normalized_entry_rva >= *start
                && normalized_entry_rva < *end
        });
        let Some((entry_start, _, entry_raw_size, _)) = entry_section else {
            return Err(Self::invalid(
                filename,
                "entry point is outside a code section",
            ));
        };
        if normalized_entry_rva - entry_start >= *entry_raw_size {
            return Err(Self::invalid(
                filename,
                "entry point is not backed by file data",
            ));
        }
        if !rva_ranges.iter().any(|(start, end, _, flags)| {
            *flags & section_flags::CODE != 0 && base_of_code >= *start && base_of_code < *end
        }) {
            return Err(Self::invalid(
                filename,
                "BaseOfCode is outside a code section",
            ));
        }

        for directory_offset in [24usize, 32usize] {
            let rva = read_u32(bytes, directory_offset).unwrap() as u64;
            let size = read_u32(bytes, directory_offset + 4).unwrap() as u64;
            if size == 0 {
                continue;
            }
            if rva == 0 {
                return Err(Self::invalid(
                    filename,
                    "non-empty data directory has a zero RVA",
                ));
            }
            let end = rva
                .checked_add(size)
                .ok_or_else(|| Self::invalid(filename, "data-directory range overflows"))?;
            if !rva_ranges
                .iter()
                .any(|(start, section_end, _, _)| rva >= *start && end <= *section_end)
            {
                return Err(Self::invalid(
                    filename,
                    "data directory is outside the mapped sections",
                ));
            }
        }

        sections.insert(
            0,
            TeSection {
                name: None,
                vma: header_vma,
                size: header_size,
                data: bytes[..header_file_size as usize].to_vec(),
                flags: section_flags::DATA | section_flags::READONLY,
                kind: SectionKind::ReadOnlyData,
            },
        );

        let entry = image_base
            .checked_add(normalized_entry_rva)
            .ok_or_else(|| Self::invalid(filename, "entry load address overflows"))?;
        let archtype = if target.is_empty() {
            info.primary.into_bytes()
        } else {
            target.as_bytes().to_vec()
        };
        let fallback_archtype = if target.is_empty() {
            info.fallback.map(String::into_bytes)
        } else {
            None
        };

        Ok(Self {
            filename: filename.to_string(),
            archtype,
            fallback_archtype,
            sections,
            entry,
            machine_thumb_hint: decodes_as_arm32
                && info.mode_policy == Some(ArmModePolicy::WholeImageThumb),
            entry_thumb_hint: decodes_as_arm32
                && info.mode_policy == Some(ArmModePolicy::EntryBit)
                && entry_rva & 1 != 0,
            spaceid: None,
            cursection: Cell::new(0),
        })
    }

    fn invalid(filename: &str, detail: &str) -> KunaError {
        KunaError::lowlevel(format!("File: {filename} : invalid TE image: {detail}"))
    }

    pub fn attach_to_space(&mut self, space: Rc<AddrSpace>) {
        self.spaceid = Some(space);
    }

    fn overlay_span(&mut self, vma: u64, data: &[u8]) -> Result<(), String> {
        if data.is_empty() {
            return Err("an overlay needs at least one byte".into());
        }
        let len = data.len() as u64;
        let end = vma
            .checked_add(len)
            .ok_or_else(|| "the overlay wraps past the end of the address space".to_string())?;
        let section = self
            .sections
            .iter_mut()
            .find(|section| {
                if vma < section.vma {
                    return false;
                }
                let offset = vma - section.vma;
                offset < section.size && len <= section.size - offset
            })
            .ok_or_else(|| format!("no loaded segment maps {vma:#x}-{end:#x}"))?;
        let offset = usize::try_from(vma - section.vma)
            .map_err(|_| "the overlay offset does not fit in host memory".to_string())?;
        let stop = offset
            .checked_add(data.len())
            .ok_or_else(|| "the overlay size does not fit in host memory".to_string())?;
        if stop > section.data.len() {
            section.data.resize(stop, 0);
        }
        section.data[offset..stop].copy_from_slice(data);
        Ok(())
    }

    pub fn fallback_arch_id(&self) -> Option<&[u8]> {
        self.fallback_archtype.as_deref()
    }

    /// The mappings the file actually backs, clamped to their initialized
    /// extent. A section's zero-filled virtual tail is mapped memory, but the
    /// image carries no copy of it, so it is not evidence that its contents are
    /// known by construction and must not reach the constant-folding ranges.
    pub fn file_backed_segments(&self) -> Vec<(u64, u64, u32)> {
        self.sections
            .iter()
            .filter_map(|section| {
                let size = section.data.len() as u64;
                (size != 0).then_some((section.vma, size, section.flags))
            })
            .collect()
    }

    /// The named sections as export metadata, in section-table order. The
    /// retained header mapping has no name and is omitted.
    pub fn section_metadata(&self) -> Vec<ObjectSectionMetadata> {
        self.sections
            .iter()
            .filter_map(|section| {
                Some(ObjectSectionMetadata::new(
                    section.name.as_deref()?,
                    section.vma,
                    section.size,
                    section.kind,
                ))
            })
            .collect()
    }

    pub fn entry(&self) -> u64 {
        self.entry
    }

    /// Whether the machine word declares a wholly Thumb instruction stream.
    /// Only `ARMNT` does; `ARM` and `ARMTHUMB_MIXED` may interwork, so they
    /// make no whole-image claim and leave the mode to the entry bit.
    pub fn whole_image_thumb(&self) -> bool {
        self.machine_thumb_hint
    }

    /// An odd mixed-ARM entry proves Thumb state at that normalized entry,
    /// but says nothing about the rest of a potentially mixed-mode image.
    pub fn entry_thumb_hint(&self) -> bool {
        self.entry_thumb_hint
    }
}

impl LoadImage for TeLoadImage {
    fn get_file_name(&self) -> &str {
        &self.filename
    }

    fn load_fill(&mut self, ptr: &mut [u8], addr: &Address) -> KunaResult<()> {
        let space = addr
            .get_space()
            .expect("TeLoadImage::loadFill: address with null space");
        if !self
            .spaceid
            .as_ref()
            .is_some_and(|attached| Rc::ptr_eq(attached, space))
        {
            return Err(KunaError::data_unavail(format!(
                "Trying to get TE image bytes from space: {}",
                space.get_name()
            )));
        }
        if ptr.is_empty() {
            return Ok(());
        }

        let mut cursor = addr.get_offset();
        let mut written = 0usize;
        while written < ptr.len() {
            if let Some(section) = self
                .sections
                .iter()
                .find(|section| cursor >= section.vma && cursor - section.vma < section.size)
            {
                let section_offset = (cursor - section.vma) as usize;
                let count = (section.size - (cursor - section.vma))
                    .min((ptr.len() - written) as u64) as usize;
                let initialized = section.data.get(section_offset..).unwrap_or(&[]);
                let backed = initialized.len().min(count);
                ptr[written..written + backed].copy_from_slice(&initialized[..backed]);
                ptr[written + backed..written + count].fill(0);
                cursor = cursor.wadd(count as u64);
                written += count;
                continue;
            }
            if written == 0 {
                let mut message = format!(
                    "Unable to load {} bytes at {}",
                    ptr.len(),
                    addr.get_shortcut()
                );
                addr.print_raw(&mut message)?;
                return Err(KunaError::data_unavail(message));
            }
            let gap = self
                .sections
                .iter()
                .filter_map(|section| (section.vma > cursor).then(|| section.vma - cursor))
                .min()
                .unwrap_or((ptr.len() - written) as u64)
                .min((ptr.len() - written) as u64) as usize;
            ptr[written..written + gap].fill(0);
            cursor = cursor.wadd(gap as u64);
            written += gap;
        }
        Ok(())
    }

    fn open_symbols(&self) {}

    fn get_next_symbol(&self, _record: &mut LoadImageFunc) -> bool {
        false
    }

    fn open_section_info(&self) {
        self.cursection.set(0);
    }

    fn get_next_section(&self, record: &mut LoadImageSection) -> bool {
        let index = self.cursection.get();
        let Some(section) = self.sections.get(index) else {
            return false;
        };
        let space = self
            .spaceid
            .as_ref()
            .expect("TeLoadImage::getNextSection before attachToSpace");
        record.address = Address::new(Rc::clone(space), section.vma);
        record.size = section.size;
        record.flags = section.flags;
        self.cursection.set(index + 1);
        index + 1 < self.sections.len()
    }

    fn get_segments(&self) -> Vec<(u64, u64, u32)> {
        self.sections
            .iter()
            .map(|section| (section.vma, section.size, section.flags))
            .collect()
    }

    fn kuna_overlay_bytes(&mut self, addr: &Address, data: &[u8]) -> KunaResult<()> {
        let space = addr
            .get_space()
            .expect("TeLoadImage::kuna_overlay_bytes: address with null space");
        if !self
            .spaceid
            .as_ref()
            .is_some_and(|attached| Rc::ptr_eq(attached, space))
        {
            return Err(KunaError::data_unavail(format!(
                "Trying to overlay TE image bytes in space: {}",
                space.get_name()
            )));
        }
        self.overlay_span(addr.get_offset(), data)
            .map_err(KunaError::data_unavail)
    }

    fn get_readonly(&self, list: &mut RangeList) {
        let Some(space) = self.spaceid.as_ref() else {
            return;
        };
        for section in &self.sections {
            if section.flags & section_flags::READONLY != 0 && section.size != 0 {
                list.insert_range(
                    Rc::clone(space),
                    section.vma,
                    section.vma.wadd(section.size).wsub(1),
                );
            }
        }
    }

    fn get_arch_type(&self) -> Vec<u8> {
        self.archtype.clone()
    }

    fn adjust_vma(&mut self, adjust: i64) {
        let space = self
            .spaceid
            .as_ref()
            .expect("TeLoadImage::adjustVma before attachToSpace");
        let byte_adjust = AddrSpace::address_to_byte(adjust as u64, space.get_word_size());
        self.entry = self.entry.wadd(byte_adjust);
        for section in &mut self.sections {
            section.vma = section.vma.wadd(byte_adjust);
        }
    }
}

fn validate_target(filename: &str, target: &str, bits: u8) -> KunaResult<()> {
    if target.is_empty() {
        return Ok(());
    }
    let mut fields = target.split(':');
    let _processor = fields.next();
    if let Some(endian) = fields.next().filter(|endian| *endian != "LE") {
        return Err(TeLoadImage::invalid(
            filename,
            &format!("target {target:?} is {endian}-endian but TE is LE-endian"),
        ));
    }
    if let Some(target_bits) = fields.next().and_then(|value| value.parse::<u8>().ok()) {
        if target_bits != bits {
            return Err(TeLoadImage::invalid(
                filename,
                &format!("target {target:?} is {target_bits}-bit but the TE machine is {bits}-bit"),
            ));
        }
    }
    Ok(())
}

/// Synthetic TE images for regression tests, generated from source so no
/// vendor binary is ever needed: one `.text` section at RVA
/// [`synthetic::CODE_RVA`] under the fixed header layout every kuna TE test
/// shares. Tests that need a malformed header patch the returned bytes.
#[doc(hidden)]
pub mod synthetic {
    use super::*;

    /// `StrippedSize`: the retained header maps at `IMAGE_BASE + 0x1d8 - 40`.
    pub const STRIPPED_SIZE: u16 = 0x01d8;
    /// `ImageBase` of every synthetic image.
    pub const IMAGE_BASE: u64 = 0x0040_0000;
    /// RVA of the `.text` section (also `BaseOfCode`).
    pub const CODE_RVA: u32 = 0x1000;
    /// `PointerToRawData` of `.text`; the stripped bias puts the bytes at
    /// [`CODE_FILE_OFFSET`].
    pub const CODE_RAW_POINTER: u32 = 0x1000;
    /// File offset of the first code byte.
    pub const CODE_FILE_OFFSET: usize = (CODE_RAW_POINTER - (STRIPPED_SIZE as u32 - 40)) as usize;
    /// `IMAGE_SCN_CNT_CODE | IMAGE_SCN_MEM_EXECUTE | IMAGE_SCN_MEM_READ`.
    pub const TEXT_CHARACTERISTICS: u32 = 0x6000_0020;

    /// A one-section TE image: `machine`, `entry_rva`, and the `.text` bytes.
    #[derive(Clone, Debug)]
    pub struct TeImage {
        pub machine: u16,
        pub entry_rva: u32,
        pub code: Vec<u8>,
        /// `.text` `PointerToRawData`. Equal to [`CODE_RVA`] by default (the
        /// SectionAlignment == FileAlignment layout); set it lower for an
        /// ordinary 0x200-file-aligned image.
        pub raw_pointer: u32,
    }

    impl TeImage {
        /// An `ARMTHUMB_MIXED` image whose odd entry is the first code byte.
        pub fn thumb(code: &[u8]) -> Self {
            Self {
                machine: IMAGE_FILE_MACHINE_THUMB,
                entry_rva: CODE_RVA | 1,
                code: code.to_vec(),
                raw_pointer: CODE_RAW_POINTER,
            }
        }

        /// A plain `ARM` image with an even entry at the first code byte.
        pub fn arm(code: &[u8]) -> Self {
            Self {
                machine: IMAGE_FILE_MACHINE_ARM,
                entry_rva: CODE_RVA,
                code: code.to_vec(),
                raw_pointer: CODE_RAW_POINTER,
            }
        }

        pub fn machine(mut self, machine: u16) -> Self {
            self.machine = machine;
            self
        }

        pub fn entry_rva(mut self, entry_rva: u32) -> Self {
            self.entry_rva = entry_rva;
            self
        }

        pub fn raw_pointer(mut self, raw_pointer: u32) -> Self {
            self.raw_pointer = raw_pointer;
            self
        }

        /// The file offset the code lands at: the section's raw pointer less
        /// the stripped-header bias.
        pub fn code_file_offset(&self) -> usize {
            (self.raw_pointer - (STRIPPED_SIZE as u32 - 40)) as usize
        }

        /// The image bytes: header, one `.text` section record, and the code
        /// at [`Self::code_file_offset`] with `VirtualSize == SizeOfRawData ==
        /// code.len()`.
        pub fn build(&self) -> Vec<u8> {
            let size = self.code.len() as u32;
            let start = self.code_file_offset();
            let mut bytes = vec![0u8; start + self.code.len()];
            bytes[0..2].copy_from_slice(b"VZ");
            bytes[2..4].copy_from_slice(&self.machine.to_le_bytes());
            bytes[4] = 1;
            bytes[5] = IMAGE_SUBSYSTEM_EFI_APPLICATION as u8;
            bytes[6..8].copy_from_slice(&STRIPPED_SIZE.to_le_bytes());
            bytes[8..12].copy_from_slice(&self.entry_rva.to_le_bytes());
            bytes[12..16].copy_from_slice(&CODE_RVA.to_le_bytes());
            bytes[16..24].copy_from_slice(&IMAGE_BASE.to_le_bytes());
            bytes[40..45].copy_from_slice(b".text");
            bytes[48..52].copy_from_slice(&size.to_le_bytes());
            bytes[52..56].copy_from_slice(&CODE_RVA.to_le_bytes());
            bytes[56..60].copy_from_slice(&size.to_le_bytes());
            bytes[60..64].copy_from_slice(&self.raw_pointer.to_le_bytes());
            bytes[76..80].copy_from_slice(&TEXT_CHARACTERISTICS.to_le_bytes());
            bytes[start..].copy_from_slice(&self.code);
            bytes
        }
    }
}

#[cfg(test)]
mod tests {
    use super::synthetic::{TeImage, CODE_FILE_OFFSET as SYNTHETIC_HEADER_SIZE, CODE_RVA};
    use super::*;
    use object::pe::{
        IMAGE_FILE_MACHINE_AMD64, IMAGE_FILE_MACHINE_ARM64, IMAGE_FILE_MACHINE_ARMNT,
        IMAGE_FILE_MACHINE_I386,
    };
    use kuna_base::space::{addrspace_flags, spacetype, AddrSpaceManager, ConstantSpace};

    fn synthetic_thumb_te() -> Vec<u8> {
        TeImage::thumb(&[0x07, 0x20, 0x70, 0x47]).build()
    }

    fn manager() -> AddrSpaceManager {
        let mut manager = AddrSpaceManager::new();
        manager.insert_space(Rc::new(ConstantSpace::new())).unwrap();
        manager
            .insert_space(Rc::new(AddrSpace::new(
                spacetype::IPTR_PROCESSOR,
                "ram",
                false,
                4,
                1,
                1,
                addrspace_flags::hasphysical,
                1,
                1,
            )))
            .unwrap();
        manager.set_default_code_space(1).unwrap();
        manager
    }

    #[test]
    fn retained_header_is_mapped_at_its_stripped_load_address() {
        let mut bytes = synthetic_thumb_te();
        bytes[8..12].copy_from_slice(&0x201u32.to_le_bytes());
        bytes[12..16].copy_from_slice(&0x200u32.to_le_bytes());
        bytes[52..56].copy_from_slice(&0x200u32.to_le_bytes());
        bytes[60..64].copy_from_slice(&0x200u32.to_le_bytes());
        bytes[80..84].copy_from_slice(&[0x07, 0x20, 0x70, 0x47]);
        let expected_header = bytes[..80].to_vec();

        let mut image = TeLoadImage::from_bytes("aligned.te", &bytes).unwrap();
        let manager = manager();
        let ram = Rc::clone(manager.get_space_by_name("ram").unwrap());
        image.attach_to_space(Rc::clone(&ram));

        let header_vma = 0x0040_0000 + 0x01d8 - TE_HEADER_SIZE as u64;
        let mut header = vec![0u8; expected_header.len()];
        image
            .load_fill(&mut header, &Address::new(Rc::clone(&ram), header_vma))
            .unwrap();
        assert_eq!(header, expected_header);
        assert!(image.get_segments().contains(&(
            header_vma,
            80,
            section_flags::DATA | section_flags::READONLY,
        )));

        let mut boundary = [0u8; 8];
        image
            .load_fill(&mut boundary, &Address::new(ram, header_vma + 76))
            .unwrap();
        assert_eq!(boundary, [0x20, 0, 0, 0x60, 0x07, 0x20, 0x70, 0x47]);
    }

    /// The ordinary PE layout: SectionAlignment 0x1000, FileAlignment 0x200.
    /// The retained header is 0x50 file bytes mapped over 0xe50 of address
    /// space, so deriving its FILE length from the `BaseOfCode` RVA delta
    /// refuses the image outright.
    #[test]
    fn a_file_aligned_image_maps_its_header_with_a_zero_tail() {
        let bytes = TeImage::thumb(&[0x07, 0x20, 0x70, 0x47]).raw_pointer(0x200).build();
        assert_eq!(bytes.len(), 0x50 + 4, "the file holds only its real header plus the code");

        let mut image = TeLoadImage::from_bytes("file-aligned.te", &bytes).unwrap();
        let manager = manager();
        let ram = Rc::clone(manager.get_space_by_name("ram").unwrap());
        image.attach_to_space(Rc::clone(&ram));

        let header_vma = 0x0040_0000 + 0x01d8 - TE_HEADER_SIZE as u64;
        assert_eq!(
            image.get_segments(),
            vec![
                (header_vma, 0xe50, section_flags::DATA | section_flags::READONLY),
                (0x401000, 4, section_flags::CODE | section_flags::READONLY),
            ]
        );

        // The retained bytes, then the alignment slack the file does not hold.
        let mut retained = [0u8; 4];
        image
            .load_fill(&mut retained, &Address::new(Rc::clone(&ram), header_vma))
            .unwrap();
        assert_eq!(retained, [b'V', b'Z', 0xc2, 0x01]);
        let mut slack = [0xffu8; 4];
        image
            .load_fill(&mut slack, &Address::new(Rc::clone(&ram), header_vma + 0x50))
            .unwrap();
        assert_eq!(slack, [0; 4], "the unbacked header tail reads as zero");

        let mut code = [0u8; 4];
        image.load_fill(&mut code, &Address::new(ram, 0x401000)).unwrap();
        assert_eq!(code, [0x07, 0x20, 0x70, 0x47]);
        assert_eq!(
            image.file_backed_segments(),
            vec![
                (header_vma, 0x50, section_flags::DATA | section_flags::READONLY),
                (0x401000, 4, section_flags::CODE | section_flags::READONLY),
            ]
        );
    }

    /// Reading from inside a mapping's unbacked tail is ordinary — every
    /// zero-filled tail has one — so it must produce zeros. Slicing the
    /// initialized bytes at an offset past their end aborts the process, and
    /// only the exact `offset == len` boundary is legal, so a test that probes
    /// the boundary alone misses it by one byte.
    #[test]
    fn a_read_starting_inside_an_unbacked_tail_reads_zeros() {
        let bytes = TeImage::thumb(&[0x07, 0x20, 0x70, 0x47]).raw_pointer(0x200).build();
        let mut image = TeLoadImage::from_bytes("file-aligned.te", &bytes).unwrap();
        let manager = manager();
        let ram = Rc::clone(manager.get_space_by_name("ram").unwrap());
        image.attach_to_space(Rc::clone(&ram));
        let header_vma = 0x0040_0000 + 0x01d8 - TE_HEADER_SIZE as u64;

        for offset in [0x50, 0x51, 0x52, 0xe00] {
            let mut slack = [0xffu8; 4];
            image
                .load_fill(&mut slack, &Address::new(Rc::clone(&ram), header_vma + offset))
                .unwrap_or_else(|error| panic!("header slack at {offset:#x}: {}", error.explain()));
            assert_eq!(slack, [0; 4], "the header's unbacked slack reads as zero at {offset:#x}");
        }

        // The same shape one section over: a `.text` whose VirtualSize runs
        // past its raw bytes.
        let mut tail = synthetic_thumb_te();
        tail[48..52].copy_from_slice(&8u32.to_le_bytes());
        let mut image = TeLoadImage::from_bytes("zero-tail.te", &tail).unwrap();
        image.attach_to_space(Rc::clone(&ram));
        let mut read = [0xffu8; 2];
        image
            .load_fill(&mut read, &Address::new(ram, 0x401005))
            .expect("the virtual tail is mapped memory");
        assert_eq!(read, [0, 0]);
    }

    /// The entry's low bit is Thumb state for the language that decodes the
    /// image, which an explicit target may make something other than the
    /// machine word's; the console folds by the resolved language, so the
    /// loader must too, or the synthetic entry it installs can never be found.
    #[test]
    fn the_entry_bit_is_read_by_the_decoding_language() {
        let ia32 = TeImage::thumb(&[0xb8, 0x07, 0x00, 0x00, 0x00, 0xc3])
            .machine(IMAGE_FILE_MACHINE_I386)
            .build();
        assert_eq!(TeLoadImage::from_bytes("ia32.te", &ia32).unwrap().entry(), 0x401001);
        let as_arm = TeLoadImage::from_bytes_with_target("ia32.te", &ia32, "ARM:LE:32:v8:default")
            .unwrap();
        assert_eq!(as_arm.entry(), 0x401000, "an ARM decode folds the bit whatever the machine");
        assert!(!as_arm.entry_thumb_hint(), "IA-32 carries no ARM mode policy");

        let arm = synthetic_thumb_te();
        assert_eq!(TeLoadImage::from_bytes("arm.te", &arm).unwrap().entry(), 0x401000);
        let as_x86 =
            TeLoadImage::from_bytes_with_target("arm.te", &arm, "x86:LE:32:default:gcc").unwrap();
        assert_eq!(as_x86.entry(), 0x401001, "an x86 decode keeps every entry bit");
        assert!(!as_x86.entry_thumb_hint());
        assert!(!as_x86.whole_image_thumb());
    }

    /// Section flags come from the rule the PE loader applies to the same
    /// characteristics, so identical section tables read alike whichever
    /// container carries them — including an uninitialized section the
    /// linker gave a nonzero raw size.
    #[test]
    fn section_flags_follow_the_pe_rule() {
        let mut bytes = TeImage::thumb(&[0x07, 0x20, 0x70, 0x47]).build();
        // `.text` becomes a writable uninitialized-data section that still
        // carries raw bytes.
        bytes[76..80].copy_from_slice(&0xc000_0080u32.to_le_bytes());
        bytes[8..12].copy_from_slice(&0x1000u32.to_le_bytes());
        let error = TeLoadImage::from_bytes("bss.te", &bytes).unwrap_err();
        assert!(error.explain().contains("entry point"), "{}", error.explain());

        let mut bytes = TeImage::thumb(&[0x07, 0x20, 0x70, 0x47]).build();
        bytes[4] = 2;
        // Keep `.text` at the front; add a second, writable `.bss` record at
        // RVA 0x2000 whose four raw bytes follow the code in the file.
        let table = 40 + 40;
        bytes.resize(bytes.len() + 4, 0);
        let mut record = vec![0u8; 40];
        record[..4].copy_from_slice(b".bss");
        record[8..12].copy_from_slice(&4u32.to_le_bytes());
        record[12..16].copy_from_slice(&0x2000u32.to_le_bytes());
        record[16..20].copy_from_slice(&4u32.to_le_bytes());
        record[20..24].copy_from_slice(&0x1004u32.to_le_bytes());
        record[36..40].copy_from_slice(&0xc000_0080u32.to_le_bytes());
        bytes[table..table + 40].copy_from_slice(&record);
        let image = TeLoadImage::from_bytes("bss.te", &bytes).unwrap();
        let bss = image.get_segments()[2];
        assert_eq!(bss.0, 0x402000);
        assert_eq!(
            bss.2,
            section_flags::NOLOAD,
            "the PE rule marks an uninitialized section NOLOAD regardless of its raw size, and no DATA"
        );
        assert_eq!(image.section_metadata()[1].kind, "UninitializedData");

        // A discardable `.reloc` is `Other` on the PE path, so it is here too,
        // and `Other` carries no DATA bit.
        let mut bytes = TeImage::thumb(&[0x07, 0x20, 0x70, 0x47]).build();
        bytes.resize(bytes.len() + 4, 0);
        bytes[4] = 2;
        let mut record = vec![0u8; 40];
        record[..6].copy_from_slice(b".reloc");
        record[8..12].copy_from_slice(&4u32.to_le_bytes());
        record[12..16].copy_from_slice(&0x2000u32.to_le_bytes());
        record[16..20].copy_from_slice(&4u32.to_le_bytes());
        record[20..24].copy_from_slice(&0x1004u32.to_le_bytes());
        record[36..40].copy_from_slice(&0x4200_0040u32.to_le_bytes());
        bytes[80..120].copy_from_slice(&record);
        let image = TeLoadImage::from_bytes("reloc.te", &bytes).unwrap();
        assert_eq!(image.get_segments()[2].2, section_flags::READONLY);
        assert_eq!(image.section_metadata()[1].kind, "Other");
    }

    /// The checked-in stage fixture is byte-for-byte what the crate's own
    /// builder produces, so its Python generator and this module cannot drift.
    #[test]
    fn the_stage_fixture_is_the_builders_output() {
        let checked_in = std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/te_entrythumbflow_arm.te"
        ))
        .expect("the stage fixture is vendored");
        let built = TeImage::arm(&[0x07, 0x20, 0x70, 0x47, 0x1e, 0xff, 0x2f, 0xe1])
            .entry_rva(CODE_RVA | 1)
            .build();
        assert_eq!(checked_in, built);
    }

    /// `VZ` is two ASCII letters an unrelated file can open with; claiming the
    /// format on the signature alone misdiagnoses every such input.
    #[test]
    fn the_format_probe_reads_more_than_the_signature() {
        let valid = synthetic_thumb_te();
        assert!(is_te_image(&valid));
        assert!(is_te_image(&valid[..TE_PROBE_LEN]));
        assert!(!is_te_image(&valid[..TE_PROBE_LEN - 1]), "a short prefix claims nothing");

        assert!(!is_te_image(b"VZ this is prose, not a container"));
        // A TE for a machine kuna has no binding for is still a TE: the probe
        // claims it so the loader's own message names the machine.
        let mut unsupported_machine = valid.clone();
        unsupported_machine[2..4].copy_from_slice(&0x0ebcu16.to_le_bytes());
        assert!(is_te_image(&unsupported_machine));
        let error = TeLoadImage::from_bytes("ebc.te", &unsupported_machine).unwrap_err();
        assert!(error.explain().contains("unsupported machine value 0x0ebc"), "{}", error.explain());
        let mut not_efi = valid.clone();
        not_efi[5] = 2;
        assert!(!is_te_image(&not_efi), "a Windows GUI subsystem is not a TE");
        let mut no_stripped_header = valid;
        no_stripped_header[6..8].copy_from_slice(&40u16.to_le_bytes());
        assert!(!is_te_image(&no_stripped_header));
    }

    #[test]
    fn rejects_section_raw_data_inside_the_retained_header() {
        // Adjusted file offset 0x40, inside the 0x50-byte TE header + section
        // table. A section starting exactly AT the table end is legal.
        let mut bytes = synthetic_thumb_te();
        bytes[60..64].copy_from_slice(&0x1f0u32.to_le_bytes());
        let error = TeLoadImage::from_bytes("raw-overlap.te", &bytes).unwrap_err();
        assert!(
            error.explain().contains("overlaps the retained TE headers"),
            "{}",
            error.explain()
        );
    }

    #[test]
    fn rejects_an_unbacked_retained_header() {
        let bytes = synthetic_thumb_te();
        let error = TeLoadImage::from_bytes(
            "unbacked-header.te",
            &bytes[..SYNTHETIC_HEADER_SIZE - 1],
        )
        .unwrap_err();
        assert!(
            error.explain().contains("retained header range extends past end of file"),
            "{}",
            error.explain()
        );
    }

    #[test]
    fn te_defaults_to_uefi_abis_and_honors_explicit_windows_models() {
        for (machine, default, windows) in [
            (IMAGE_FILE_MACHINE_ARM64, "AARCH64:LE:64:v8A:default", "AARCH64:LE:64:v8A:windows"),
            (IMAGE_FILE_MACHINE_I386, "x86:LE:32:default:gcc", "x86:LE:32:default:windows"),
            (IMAGE_FILE_MACHINE_ARMNT, "ARM:LE:32:v8:default", "ARM:LE:32:v8:windows"),
        ] {
            let bytes = TeImage::thumb(&[0x07, 0x20, 0x70, 0x47])
                .machine(machine)
                .entry_rva(CODE_RVA)
                .build();
            let automatic = TeLoadImage::from_bytes("abi.te", &bytes).unwrap();
            assert_eq!(automatic.get_arch_type(), default.as_bytes(), "machine {machine:#x}");
            assert_eq!(automatic.fallback_arch_id(), None, "machine {machine:#x}");
            let explicit = TeLoadImage::from_bytes_with_target("abi.te", &bytes, windows).unwrap();
            assert_eq!(explicit.get_arch_type(), windows.as_bytes(), "machine {machine:#x}");
        }
    }

    #[test]
    fn ignores_zero_extent_section_records() {
        let mut bytes = synthetic_thumb_te();
        bytes[4] = 2;
        let image = TeLoadImage::from_bytes("synthetic.te", &bytes).unwrap();
        assert_eq!(image.get_segments().len(), 2);
        assert_eq!(image.entry(), 0x401000);
    }

    #[test]
    fn parses_stripped_offsets_and_loads_original_vma() {
        let mut image = TeLoadImage::from_bytes("synthetic.te", &synthetic_thumb_te()).unwrap();
        assert_eq!(image.get_arch_type(), b"ARM:LE:32:v8:default");
        assert_eq!(image.fallback_arch_id(), None);
        assert_eq!(image.entry(), 0x401000);
        assert_eq!(
            image.get_segments(),
            vec![
                (
                    0x4001b0,
                    0xe50,
                    section_flags::DATA | section_flags::READONLY,
                ),
                (0x401000, 4, section_flags::CODE | section_flags::READONLY),
            ]
        );
        assert_eq!(image.file_backed_segments(), image.get_segments());
        assert_eq!(
            image.section_metadata(),
            vec![ObjectSectionMetadata {
                name: ".text".to_string(),
                vma: 0x401000,
                size: 4,
                kind: "Text".to_string(),
            }]
        );
        assert!(!image.whole_image_thumb());

        let mut armnt = synthetic_thumb_te();
        armnt[2..4].copy_from_slice(&IMAGE_FILE_MACHINE_ARMNT.to_le_bytes());
        let armnt = TeLoadImage::from_bytes("armnt.te", &armnt).unwrap();
        assert_eq!(armnt.get_arch_type(), b"ARM:LE:32:v8:default");
        assert!(armnt.whole_image_thumb());

        let manager = manager();
        let ram = Rc::clone(manager.get_space_by_name("ram").unwrap());
        image.attach_to_space(Rc::clone(&ram));
        let mut loaded = [0u8; 4];
        image
            .load_fill(&mut loaded, &Address::new(ram, 0x401000))
            .unwrap();
        assert_eq!(loaded, [0x07, 0x20, 0x70, 0x47]);
    }

    #[test]
    fn publishes_every_te_mapping_as_a_segment() {
        let mut image = TeLoadImage::from_bytes("synthetic.te", &synthetic_thumb_te()).unwrap();
        let manager = manager();
        image.attach_to_space(Rc::clone(manager.get_space_by_name("ram").unwrap()));
        let mut walked = Vec::new();
        let mut record = LoadImageSection::default();
        image.open_section_info();
        loop {
            let more = image.get_next_section(&mut record);
            walked.push((record.address.get_offset(), record.size, record.flags));
            if !more {
                break;
            }
        }
        assert_eq!(image.get_segments(), walked);
    }

    /// The machine table is the object loader's own composer: a TE and a PE
    /// carrying the same machine select the same SLEIGH variant, and every id
    /// the table can produce is enumerated for the cross-crate resolve gate.
    #[test]
    fn machine_table_composes_the_shared_language_ids() {
        let mut arm = synthetic_thumb_te();
        arm[2..4].copy_from_slice(&IMAGE_FILE_MACHINE_ARM.to_le_bytes());
        arm[8..12].copy_from_slice(&0x1000u32.to_le_bytes());
        let arm = TeLoadImage::from_bytes("arm.te", &arm).unwrap();
        assert_eq!(arm.get_arch_type(), b"ARM:LE:32:v8:default");
        assert_eq!(arm.fallback_arch_id(), None);
        assert!(!arm.whole_image_thumb());
        assert!(!arm.entry_thumb_hint());

        let mut x64 = synthetic_thumb_te();
        x64[2..4].copy_from_slice(&IMAGE_FILE_MACHINE_AMD64.to_le_bytes());
        x64[8..12].copy_from_slice(&0x1000u32.to_le_bytes());
        let x64 = TeLoadImage::from_bytes("x64.te", &x64).unwrap();
        assert_eq!(x64.get_arch_type(), b"x86:LE:64:default:windows");
        assert_eq!(x64.fallback_arch_id(), Some(b"x86:LE:64:default:gcc".as_slice()));

        let ids = te_language_ids();
        assert_eq!(ids.len(), PE_MACHINES.len());
        assert!(ids.iter().any(|(primary, fallback)| primary == "x86:LE:64:default:windows"
            && fallback.as_deref() == Some("x86:LE:64:default:gcc")));
        assert!(ids.iter().filter(|(primary, _)| primary == "ARM:LE:32:v8:default").count() == 3);
        assert!(machine_info(0x0166).is_none(), "MIPS is not a UEFI binding");
    }

    #[test]
    fn an_overlay_replaces_a_complete_mapped_span_atomically() {
        let mut image = TeLoadImage::from_bytes("synthetic.te", &synthetic_thumb_te()).unwrap();
        let manager = manager();
        let ram = Rc::clone(manager.get_space_by_name("ram").unwrap());
        image.attach_to_space(Rc::clone(&ram));

        image
            .kuna_overlay_bytes(&Address::new(Rc::clone(&ram), 0x401000), &[0x2a, 0x20])
            .expect("the code section maps both bytes");
        let mut loaded = [0u8; 4];
        image
            .load_fill(&mut loaded, &Address::new(Rc::clone(&ram), 0x401000))
            .unwrap();
        assert_eq!(loaded, [0x2a, 0x20, 0x70, 0x47]);

        let error = image
            .kuna_overlay_bytes(&Address::new(Rc::clone(&ram), 0x401002), &[0x90; 4])
            .unwrap_err();
        assert!(error.explain().contains("no loaded segment maps"));
        image
            .load_fill(&mut loaded, &Address::new(ram, 0x401000))
            .unwrap();
        assert_eq!(
            loaded,
            [0x2a, 0x20, 0x70, 0x47],
            "a rejected partial overlay must change nothing"
        );

        let other = Rc::clone(manager.get_space_by_name("const").unwrap());
        assert!(image
            .kuna_overlay_bytes(&Address::new(other, 0x401000), &[0x90])
            .is_err());
    }

    #[test]
    fn an_overlay_materializes_a_mapped_virtual_tail() {
        let mut bytes = synthetic_thumb_te();
        bytes[48..52].copy_from_slice(&8u32.to_le_bytes());
        let mut image = TeLoadImage::from_bytes("zero-tail.te", &bytes).unwrap();
        let manager = manager();
        let ram = Rc::clone(manager.get_space_by_name("ram").unwrap());
        image.attach_to_space(Rc::clone(&ram));

        image
            .kuna_overlay_bytes(&Address::new(Rc::clone(&ram), 0x401006), &[0xde, 0xad])
            .expect("the virtual tail is mapped memory");
        let mut loaded = [0xffu8; 4];
        image
            .load_fill(&mut loaded, &Address::new(ram, 0x401004))
            .unwrap();
        assert_eq!(loaded, [0, 0, 0xde, 0xad]);
    }

    #[test]
    fn virtual_size_excludes_raw_file_alignment_padding() {
        let mut bytes = synthetic_thumb_te();
        bytes.resize(SYNTHETIC_HEADER_SIZE + 16, 0xa5);
        bytes[56..60].copy_from_slice(&16u32.to_le_bytes());

        let mut image = TeLoadImage::from_bytes("padded.te", &bytes).unwrap();
        assert_eq!(
            image.get_segments(),
            vec![
                (
                    0x4001b0,
                    0xe50,
                    section_flags::DATA | section_flags::READONLY,
                ),
                (0x401000, 4, section_flags::CODE | section_flags::READONLY),
            ]
        );
        let manager = manager();
        let ram = Rc::clone(manager.get_space_by_name("ram").unwrap());
        image.attach_to_space(Rc::clone(&ram));
        let mut byte = [0u8; 1];
        assert!(image
            .load_fill(&mut byte, &Address::new(Rc::clone(&ram), 0x401003))
            .is_ok());
        assert!(image
            .load_fill(&mut byte, &Address::new(ram, 0x401004))
            .is_err());

        let mut entry_in_padding = bytes.clone();
        entry_in_padding[8..12].copy_from_slice(&0x1005u32.to_le_bytes());
        let error = TeLoadImage::from_bytes("entry-padding.te", &entry_in_padding).unwrap_err();
        assert!(
            error.explain().contains("entry point"),
            "{}",
            error.explain()
        );

        let mut directory_in_padding = bytes;
        directory_in_padding[24..28].copy_from_slice(&0x1004u32.to_le_bytes());
        directory_in_padding[28..32].copy_from_slice(&1u32.to_le_bytes());
        let error =
            TeLoadImage::from_bytes("directory-padding.te", &directory_in_padding).unwrap_err();
        assert!(
            error.explain().contains("outside the mapped sections"),
            "{}",
            error.explain()
        );
    }

    #[test]
    fn virtual_tail_is_zero_filled_and_zero_virtual_size_uses_raw_extent() {
        let mut bytes = synthetic_thumb_te();
        bytes[48..52].copy_from_slice(&8u32.to_le_bytes());
        let mut image = TeLoadImage::from_bytes("zero-tail.te", &bytes).unwrap();
        assert_eq!(image.get_segments()[1], (0x401000, 8, section_flags::CODE | section_flags::READONLY));
        assert_eq!(
            image.file_backed_segments()[1],
            (0x401000, 4, section_flags::CODE | section_flags::READONLY)
        );
        let manager = manager();
        let ram = Rc::clone(manager.get_space_by_name("ram").unwrap());
        image.attach_to_space(Rc::clone(&ram));
        let mut loaded = [0xffu8; 8];
        image
            .load_fill(&mut loaded, &Address::new(ram, 0x401000))
            .unwrap();
        assert_eq!(loaded, [0x07, 0x20, 0x70, 0x47, 0, 0, 0, 0]);

        let mut zero_virtual = synthetic_thumb_te();
        zero_virtual[48..52].fill(0);
        let image = TeLoadImage::from_bytes("zero-virtual.te", &zero_virtual).unwrap();
        assert_eq!(image.get_segments()[1].1, 4);

        let mut unbacked_entry = synthetic_thumb_te();
        unbacked_entry[48..52].copy_from_slice(&8u32.to_le_bytes());
        unbacked_entry[8..12].copy_from_slice(&0x1005u32.to_le_bytes());
        let error = TeLoadImage::from_bytes("unbacked-entry.te", &unbacked_entry).unwrap_err();
        assert!(
            error.explain().contains("not backed by file data"),
            "{}",
            error.explain()
        );
    }

    #[test]
    fn rejects_malformed_header_section_and_entry_ranges() {
        let valid = synthetic_thumb_te();
        let cases: &[(usize, &[u8], &str)] = &[
            (5, &[0], "subsystem"),
            (6, &40u16.to_le_bytes(), "StrippedSize"),
            (
                12,
                &500u32.to_le_bytes(),
                "does not contain the section table",
            ),
            (12, &0x100u32.to_le_bytes(), "BaseOfCode"),
            (60, &0x100u32.to_le_bytes(), "file offset underflows"),
            (56, &0x1000u32.to_le_bytes(), "past end of file"),
            (8, &0x2001u32.to_le_bytes(), "entry point"),
            (24, &0x1000u32.to_le_bytes(), "zero RVA"),
        ];
        for &(offset, replacement, expected) in cases {
            let mut bytes = valid.clone();
            bytes[offset..offset + replacement.len()].copy_from_slice(replacement);
            if offset == 24 {
                bytes[24..28].fill(0);
                bytes[28..32].copy_from_slice(&4u32.to_le_bytes());
            }
            let error = TeLoadImage::from_bytes("bad.te", &bytes).unwrap_err();
            assert!(
                error.explain().contains(expected),
                "expected {expected:?}, got {}",
                error.explain()
            );
        }

        let mut overlap = valid.clone();
        overlap[52..56].copy_from_slice(&0x0fffu32.to_le_bytes());
        let error = TeLoadImage::from_bytes("overlap.te", &overlap).unwrap_err();
        assert!(
            error.explain().contains("overlaps the retained TE headers"),
            "{}",
            error.explain()
        );

        assert!(TeLoadImage::from_bytes("short.te", &valid[..39]).is_err());
    }

    #[test]
    fn explicit_target_must_match_container_width_and_endian() {
        let bytes = synthetic_thumb_te();
        let automatic =
            TeLoadImage::from_bytes_with_target("synthetic.te", &bytes, "default").unwrap();
        assert_eq!(automatic.get_arch_type(), b"ARM:LE:32:v8:default");
        let endian =
            TeLoadImage::from_bytes_with_target("synthetic.te", &bytes, "ARM:BE:32:v4t:default")
                .unwrap_err();
        assert!(endian.explain().contains("BE-endian"));
        let mixed_endian = TeLoadImage::from_bytes_with_target(
            "synthetic.te",
            &bytes,
            "ARM:LEBE:32:v8LEInstruction:default",
        )
        .unwrap_err();
        assert!(mixed_endian.explain().contains("LEBE-endian"));
        let width = TeLoadImage::from_bytes_with_target(
            "synthetic.te",
            &bytes,
            "AARCH64:LE:64:v8A:default",
        )
        .unwrap_err();
        assert!(width.explain().contains("64-bit"));
        let narrow =
            TeLoadImage::from_bytes_with_target("synthetic.te", &bytes, "avr8:LE:16:default")
                .unwrap_err();
        assert!(narrow.explain().contains("16-bit"));
    }
}

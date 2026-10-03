//! (kuna) A pointer-aligned slot whose bytes are an address inside the loaded
//! image is a pointer, not text.
//!
//! The string matchers accept any printable run ended by a NUL, and the entry of
//! a pointer table often is one: on a non-PIE x86-64 image a function at
//! `0x404226` is stored as `26 42 40 00 00 00 00 00`, which reads as `"&B@"`.
//! Planting a `char[N]` there makes the printer index or store a short literal
//! (`(**(void **)&"&B@"[i * 8])()`, `*this = "dB@"`) where the program uses the
//! table, the vtable or the jump table at that address. [`PointerSlots`] decides
//! whether a pointer-aligned run start is such a slot.
//!
//! An image that is rebased at load time says where every pointer is: a
//! position-independent ELF carries a dynamic relocation and a PE with a `.reloc`
//! directory a base relocation for each one. There the run is refused exactly when
//! a relocation applies at its start. A non-PIE ELF or a PE without base
//! relocations says nothing, so the run is refused when its pointer-sized value,
//! read in the image's byte order, lands in an allocated section. A genuine string
//! is refused there only when its first pointer-sized bytes, NUL and padding
//! included, spell an address of the image, and it then prints as that address,
//! which is never a wrong value. A relocatable object has no image addresses yet,
//! so nothing is refused there.

use object::pe::{IMAGE_REL_BASED_DIR64, IMAGE_REL_BASED_HIGHLOW};
use object::read::pe::{ImageNtHeaders, PeFile};
use object::read::{Object, ObjectSection};
use object::{BinaryFormat, ObjectKind, SectionFlags};

const SHF_ALLOC: u64 = 0x2;

/// The image's pointer width and byte order, its section bytes, the address
/// ranges its allocated sections occupy, and the sorted slots it relocates when
/// it lists every one of them.
pub(crate) struct PointerSlots<'d> {
    width: u64,
    little_endian: bool,
    data: Vec<(u64, &'d [u8])>,
    loaded: Vec<(u64, u64)>,
    relocated: Option<Vec<u64>>,
}

impl<'d> PointerSlots<'d> {
    pub(crate) fn new(file: &object::File<'d>) -> Self {
        let mut slots = PointerSlots {
            width: if file.is_64() { 8 } else { 4 },
            little_endian: file.is_little_endian(),
            data: Vec::new(),
            loaded: Vec::new(),
            relocated: None,
        };
        if file.kind() == ObjectKind::Relocatable {
            return slots;
        }
        slots.relocated = relocated_slots(file);
        for sec in file.sections() {
            let size = sec.size();
            if size == 0 || !is_allocated(sec.flags()) {
                continue;
            }
            let lo = sec.address();
            slots.loaded.push((lo, lo.saturating_add(size)));
            if let Ok(bytes) = sec.data() {
                if !bytes.is_empty() {
                    slots.data.push((lo, bytes));
                }
            }
        }
        slots
    }

    /// Is `addr` a pointer-aligned slot that holds an address of the image: one
    /// the image relocates, or, when it lists none, one whose value lies in an
    /// allocated section?
    pub(crate) fn holds_address(&self, addr: u64) -> bool {
        if self.loaded.is_empty() || addr % self.width != 0 {
            return false;
        }
        if let Some(relocated) = &self.relocated {
            return relocated.binary_search(&addr).is_ok();
        }
        let Some(value) = self.read(addr) else {
            return false;
        };
        self.loaded.iter().any(|&(lo, hi)| value >= lo && value < hi)
    }

    fn read(&self, addr: u64) -> Option<u64> {
        let width = self.width as usize;
        self.data.iter().find_map(|&(lo, bytes)| {
            let off = usize::try_from(addr.checked_sub(lo)?).ok()?;
            let raw = bytes.get(off..off.checked_add(width)?)?;
            let mut buf = [0u8; 8];
            if self.little_endian {
                buf[..width].copy_from_slice(raw);
                Some(u64::from_le_bytes(buf))
            } else {
                buf[8 - width..].copy_from_slice(raw);
                Some(u64::from_be_bytes(buf))
            }
        })
    }
}

/// Every slot a load-time rebase rewrites, for an image that lists them all: a
/// position-independent ELF's dynamic relocations, or a PE's base relocations.
/// `None` for a non-PIE ELF, whose own pointers carry no relocation, and for any
/// other image that lists none.
fn relocated_slots(file: &object::File<'_>) -> Option<Vec<u64>> {
    let mut slots = match file {
        object::File::Pe32(pe) => pe_relocated(pe),
        object::File::Pe64(pe) => pe_relocated(pe),
        _ if file.format() == BinaryFormat::Elf && file.kind() == ObjectKind::Dynamic => {
            file.dynamic_relocations().map(|relocs| relocs.map(|(slot, _)| slot).collect())
        }
        _ => None,
    }?;
    slots.sort_unstable();
    slots.dedup();
    Some(slots)
}

fn pe_relocated<Pe: ImageNtHeaders>(pe: &PeFile<'_, Pe>) -> Option<Vec<u64>> {
    let sections = pe.section_table();
    let blocks = pe.data_directories().relocation_blocks(pe.data(), &sections).ok()??;
    let base = pe.relative_address_base();
    let slots: Vec<u64> = blocks
        .flatten()
        .flatten()
        .filter(|rel| matches!(rel.typ, IMAGE_REL_BASED_HIGHLOW | IMAGE_REL_BASED_DIR64))
        .map(|rel| base + u64::from(rel.virtual_address))
        .collect();
    (!slots.is_empty()).then_some(slots)
}

/// Does a section with these flags occupy addresses in the loaded image? ELF
/// says so with `SHF_ALLOC`; every PE/COFF and Mach-O section of an image is
/// mapped.
fn is_allocated(flags: SectionFlags) -> bool {
    match flags {
        SectionFlags::Elf { sh_flags } => sh_flags & SHF_ALLOC != 0,
        SectionFlags::Coff { .. } | SectionFlags::MachO { .. } => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests;

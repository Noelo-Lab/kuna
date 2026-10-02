//! (kuna) A pointer-aligned slot whose bytes are an address inside the loaded
//! image is a pointer, not text.
//!
//! The string matchers accept any printable run ended by a NUL, and the entry of
//! a pointer table often is one: on a non-PIE x86-64 image a function at
//! `0x404226` is stored as `26 42 40 00 00 00 00 00`, which reads as `"&B@"`.
//! Planting a `char[N]` there makes the printer index or store a short literal
//! (`(**(void **)&"&B@"[i * 8])()`, `*this = "dB@"`) where the program uses the
//! table, the vtable or the jump table at that address. [`PointerSlots`] reads the
//! pointer-sized value at a run's start; when the start is pointer-aligned and the
//! value lands in an allocated section, the run is not planted.
//!
//! A genuine string can only be refused when it starts on a pointer boundary and
//! its first pointer-sized bytes, NUL and padding included, spell an address of
//! the image. The refused run then prints as the address it is, which is never a
//! wrong value. A relocatable object has no image addresses yet, so nothing is
//! refused there.

use object::read::{Object, ObjectSection};
use object::{ObjectKind, SectionFlags};

const SHF_ALLOC: u64 = 0x2;

/// The image's pointer width and byte order, its section bytes, and the address
/// ranges its allocated sections occupy.
pub(crate) struct PointerSlots<'d> {
    width: u64,
    little_endian: bool,
    data: Vec<(u64, &'d [u8])>,
    loaded: Vec<(u64, u64)>,
}

impl<'d> PointerSlots<'d> {
    pub(crate) fn new(file: &object::File<'d>) -> Self {
        let mut slots = PointerSlots {
            width: if file.is_64() { 8 } else { 4 },
            little_endian: file.is_little_endian(),
            data: Vec::new(),
            loaded: Vec::new(),
        };
        if file.kind() == ObjectKind::Relocatable {
            return slots;
        }
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

    /// Is `addr` pointer-aligned, with a pointer-sized value there that lies in
    /// an allocated section?
    pub(crate) fn holds_address(&self, addr: u64) -> bool {
        if self.loaded.is_empty() || addr % self.width != 0 {
            return false;
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

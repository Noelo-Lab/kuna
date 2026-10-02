//! Does an ELF image say it is a program for an operating system's user space?
//!
//! A bare-metal image (firmware, an RTOS, a boot loader) and a Linux program are
//! the same ELF container: same `e_machine`, same ARM EABI version, same
//! `EI_OSABI` of 0. What a Linux or BSD program carries and firmware does not is
//! something only an operating system reads: a program interpreter, a dynamic
//! section that names shared objects, an ABI-tag note, or an `EI_OSABI` naming
//! the system. [`is_os_userland_image`] answers yes only on one of those, so an
//! image with none of them -- a statically linked musl or uClibc program as much
//! as an RTOS -- answers no.

/// `EI_OSABI` values that name a Unix-like system: NetBSD, GNU/Linux, FreeBSD,
/// OpenBSD.
const OS_ABIS: [u8; 4] = [2, 3, 9, 12];

/// Note owners whose type-1 note identifies the system the image was built for
/// (`NT_GNU_ABI_TAG`, `NT_FREEBSD_ABI_TAG`, `NT_NETBSD_IDENT`,
/// `NT_OPENBSD_IDENT`, `NT_ANDROID_TYPE_IDENT`).
const OS_NOTE_OWNERS: [&[u8]; 5] = [b"GNU", b"FreeBSD", b"NetBSD", b"OpenBSD", b"Android"];

const PT_DYNAMIC: u32 = 2;
const PT_INTERP: u32 = 3;
const PT_NOTE: u32 = 4;
const SHT_NOTE: u32 = 7;
const DT_NEEDED: u64 = 1;
const DT_SONAME: u64 = 14;

/// Is `bytes` an ELF image built for an operating system's user space?
///
/// Yes when its `EI_OSABI` names a Unix-like system, when it has a `PT_INTERP`
/// segment, when its `PT_DYNAMIC` segment holds a `DT_NEEDED` or `DT_SONAME`
/// entry, or when a note segment or section is a type-1 ABI note of a
/// system's owner. Anything else, and anything that is not a well-formed ELF,
/// answers `false`.
pub fn is_os_userland_image(bytes: &[u8]) -> bool {
    let Some(elf) = Elf::parse(bytes) else {
        return false;
    };
    if OS_ABIS.contains(&bytes[7]) {
        return true;
    }
    for i in 0..elf.phnum {
        let ph = elf.phoff.saturating_add(i * elf.phentsize);
        let Some(p_type) = elf.u32(ph) else { break };
        let (offset, filesz, align) = (
            elf.word(ph, 8, 4),
            elf.word(ph, 32, 16),
            elf.word(ph, 48, 28),
        );
        let (Some(offset), Some(filesz)) = (offset, filesz) else {
            continue;
        };
        let found = match p_type {
            PT_INTERP => true,
            PT_DYNAMIC => elf.names_shared_object(offset, filesz),
            PT_NOTE => elf.has_os_note(offset, filesz, align.unwrap_or(4)),
            _ => false,
        };
        if found {
            return true;
        }
    }
    for i in 0..elf.shnum {
        let sh = elf.shoff.saturating_add(i * elf.shentsize);
        if elf.u32(sh.saturating_add(4)) != Some(SHT_NOTE) {
            continue;
        }
        let (offset, size, align) = (
            elf.word(sh, 24, 16),
            elf.word(sh, 32, 20),
            elf.word(sh, 48, 32),
        );
        if let (Some(offset), Some(size)) = (offset, size) {
            if elf.has_os_note(offset, size, align.unwrap_or(4)) {
                return true;
            }
        }
    }
    false
}

/// The header fields the checks read, and the image to read the rest from.
struct Elf<'a> {
    bytes: &'a [u8],
    wide: bool,
    big: bool,
    phoff: usize,
    phentsize: usize,
    phnum: usize,
    shoff: usize,
    shentsize: usize,
    shnum: usize,
}

impl<'a> Elf<'a> {
    fn parse(bytes: &'a [u8]) -> Option<Self> {
        if bytes.get(0..4)? != b"\x7fELF" {
            return None;
        }
        let wide = match bytes.get(4)? {
            1 => false,
            2 => true,
            _ => return None,
        };
        let big = match bytes.get(5)? {
            1 => false,
            2 => true,
            _ => return None,
        };
        let mut elf = Elf {
            bytes,
            wide,
            big,
            phoff: 0,
            phentsize: 0,
            phnum: 0,
            shoff: 0,
            shentsize: 0,
            shnum: 0,
        };
        let size = |v: Option<u64>| v.and_then(|v| usize::try_from(v).ok());
        elf.phoff = size(elf.word(0, 32, 28))?;
        elf.shoff = size(elf.word(0, 40, 32))?;
        let rest = if wide { 52 } else { 40 };
        elf.phentsize = elf.u16(rest + 2)? as usize;
        elf.phnum = elf.u16(rest + 4)? as usize;
        elf.shentsize = elf.u16(rest + 6)? as usize;
        elf.shnum = elf.u16(rest + 8)? as usize;
        if elf.phentsize < if wide { 56 } else { 32 } {
            elf.phnum = 0;
        }
        if elf.shentsize < if wide { 64 } else { 40 } {
            elf.shnum = 0;
        }
        Some(elf)
    }

    fn take<const N: usize>(&self, at: usize) -> Option<[u8; N]> {
        self.bytes.get(at..at.checked_add(N)?)?.try_into().ok()
    }

    fn u16(&self, at: usize) -> Option<u16> {
        let b = self.take::<2>(at)?;
        Some(if self.big {
            u16::from_be_bytes(b)
        } else {
            u16::from_le_bytes(b)
        })
    }

    fn u32(&self, at: usize) -> Option<u32> {
        let b = self.take::<4>(at)?;
        Some(if self.big {
            u32::from_be_bytes(b)
        } else {
            u32::from_le_bytes(b)
        })
    }

    fn u64(&self, at: usize) -> Option<u64> {
        let b = self.take::<8>(at)?;
        Some(if self.big {
            u64::from_be_bytes(b)
        } else {
            u64::from_le_bytes(b)
        })
    }

    /// The address-sized field of the structure at `base`: at `wide_at` in a
    /// 64-bit image, at `narrow_at` in a 32-bit one.
    fn word(&self, base: usize, wide_at: usize, narrow_at: usize) -> Option<u64> {
        if self.wide {
            self.u64(base.saturating_add(wide_at))
        } else {
            self.u32(base.saturating_add(narrow_at)).map(u64::from)
        }
    }

    /// The file range `[offset, offset + size)`, when it lies inside the image.
    fn range(&self, offset: u64, size: u64) -> Option<(usize, usize)> {
        let start = usize::try_from(offset).ok()?;
        let end = start.checked_add(usize::try_from(size).ok()?)?;
        (end <= self.bytes.len()).then_some((start, end))
    }

    /// Does the dynamic table at `[offset, offset + size)` name a shared object,
    /// either one the image needs or its own?
    fn names_shared_object(&self, offset: u64, size: u64) -> bool {
        let Some((start, end)) = self.range(offset, size) else {
            return false;
        };
        let entry = if self.wide { 16 } else { 8 };
        let mut at = start;
        while at + entry <= end {
            let tag = if self.wide {
                self.u64(at)
            } else {
                self.u32(at).map(u64::from)
            };
            match tag {
                Some(DT_NEEDED | DT_SONAME) => return true,
                Some(0) | None => return false,
                _ => at += entry,
            }
        }
        false
    }

    /// Does the note area at `[offset, offset + size)` hold a type-1 note of a
    /// system's owner? Entries are padded to `align`, 4 or 8.
    fn has_os_note(&self, offset: u64, size: u64, align: u64) -> bool {
        let Some((start, end)) = self.range(offset, size) else {
            return false;
        };
        let pad = if align == 8 { 8 } else { 4 };
        let up = |n: usize| n.checked_add(pad - 1).map(|n| n & !(pad - 1));
        let mut at = start;
        while at + 12 <= end {
            let (Some(namesz), Some(descsz), Some(kind)) =
                (self.u32(at), self.u32(at + 4), self.u32(at + 8))
            else {
                return false;
            };
            let name_at = at + 12;
            let Some(name_end) = name_at.checked_add(namesz as usize) else {
                return false;
            };
            let Some(name) = self.bytes.get(name_at..name_end.min(end)) else {
                return false;
            };
            let owner = name.strip_suffix(b"\0").unwrap_or(name);
            if kind == 1 && OS_NOTE_OWNERS.contains(&owner) {
                return true;
            }
            let Some(desc_at) = up(name_end) else {
                return false;
            };
            let Some(next) = desc_at.checked_add(descsz as usize).and_then(up) else {
                return false;
            };
            if next <= at {
                return false;
            }
            at = next;
        }
        false
    }
}

#[cfg(test)]
mod tests;

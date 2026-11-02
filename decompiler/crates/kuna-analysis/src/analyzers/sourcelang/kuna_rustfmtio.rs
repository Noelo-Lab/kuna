//! Image evidence for the receiver convention of the Stdout Write implementation.

use std::collections::{BTreeMap, BTreeSet};

use kuna_decomp::kuna_rustfmtio::{ImageSpan, ReceiverAbi, WriterProfile};
use object::{Object, ObjectSegment, ObjectSymbol, SegmentFlags, SymbolKind};

use crate::loader::kuna_dynrelocs::{fixup_sites, relro_range, FixupKind, FixupSite};

const STDOUT_WRITE_FMT: &str = "<&std::io::stdio::Stdout as std::io::Write>::write_fmt";

struct Image<'file, 'data> {
    file: &'file object::File<'data>,
    relro: Option<(u64, u64)>,
    fixups: Vec<FixupSite>,
}

impl<'file, 'data> Image<'file, 'data> {
    fn new(file: &'file object::File<'data>, bytes: &[u8]) -> Self {
        Self {
            file,
            relro: relro_range(bytes)
                .and_then(|(lo, hi)| (lo < (hi & !4095)).then_some((lo, hi & !4095))),
            fixups: fixup_sites(file, bytes).sites,
        }
    }

    fn protected(&self, address: u64, size: usize) -> Option<ImageSpan> {
        if address == 0 || size == 0 || size > 4096 {
            return None;
        }
        let end = address.checked_add(size as u64)?;
        let mut result = None;
        for segment in self.file.segments() {
            let lo = segment.address();
            let hi = lo.checked_add(segment.size())?;
            if end <= lo || hi <= address {
                continue;
            }
            if result.is_some() || address < lo || hi < end {
                return None;
            }
            let SegmentFlags::Elf { p_flags } = segment.flags() else {
                return None;
            };
            let relro = self
                .relro
                .is_some_and(|(lo, hi)| lo <= address && end <= hi);
            if p_flags & object::elf::PF_W != 0 && !relro {
                return None;
            }
            let offset = usize::try_from(address.checked_sub(lo)?).ok()?;
            let bytes = segment
                .data()
                .ok()?
                .get(offset..offset.checked_add(size)?)?;
            result = Some(bytes.to_vec());
        }
        let mut bytes = result?;
        let mut rewritten = BTreeSet::new();
        for fixup in &self.fixups {
            let last = fixup.vma.checked_add(u64::from(fixup.width))?;
            if last <= address || end <= fixup.vma {
                continue;
            }
            if fixup.width != 8
                || fixup.vma < address
                || end < last
                || !matches!(fixup.kind, FixupKind::Rebase | FixupKind::Bind)
            {
                return None;
            }
            for offset in fixup.vma..last {
                if !rewritten.insert(offset) {
                    return None;
                }
            }
            let target = fixup.target?;
            let offset = usize::try_from(fixup.vma - address).ok()?;
            bytes
                .get_mut(offset..offset + 8)?
                .copy_from_slice(&target.to_le_bytes());
        }
        Some(ImageSpan { address, bytes })
    }

    fn code(&self, address: u64, size: usize) -> Option<ImageSpan> {
        let span = self.protected(address, size)?;
        let end = address.checked_add(size as u64)?;
        self.file.segments().any(|segment| {
            matches!(segment.flags(), SegmentFlags::Elf { p_flags } if p_flags & object::elf::PF_X != 0)
                && segment.address() <= address
                && segment.address().checked_add(segment.size()).is_some_and(|hi| end <= hi)
        }).then_some(span)
    }

    fn imported(&self, address: u64, name: &str) -> bool {
        if address == 0 || address % 8 != 0 {
            return false;
        }
        let Some(end) = address.checked_add(8) else {
            return false;
        };
        let mut matches = self.fixups.iter().filter(|fixup| {
            fixup.vma < end
                && fixup
                    .vma
                    .checked_add(u64::from(fixup.width))
                    .is_none_or(|last| address < last)
        });
        let Some(fixup) = matches.next() else {
            return false;
        };
        matches.next().is_none()
            && fixup.vma == address
            && fixup.width == 8
            && fixup.kind == FixupKind::Bind
            && fixup.symbol.as_deref() == Some(name)
            && fixup.addend == Some(0)
            && fixup.target.is_none()
    }

    fn plt(&self, address: u64, name: &str) -> Option<ImageSpan> {
        let span = self.code(address, 6)?;
        if span.bytes.get(..2)? != [0xff, 0x25] {
            return None;
        }
        let displacement = i32::from_le_bytes(span.bytes.get(2..6)?.try_into().ok()?);
        let slot = address
            .checked_add(6)?
            .checked_add_signed(i64::from(displacement))?;
        self.imported(slot, name).then_some(span)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum NameRole {
    StdoutWriteFmt,
    StdoutLockWriteAll,
    ThreadCounter,
    ThreadExhausted,
    FmtWrite,
    ExpectFailed,
    LockContended,
    PanicFmt,
    DropLock,
    DropError,
    PanicCleanup,
    DropAdapter,
    AdapterWriteStr,
    FmtWriteChar,
    FmtWriteFmt,
}

fn name_role(name: &str) -> Option<NameRole> {
    Some(match name {
        STDOUT_WRITE_FMT => NameRole::StdoutWriteFmt,
        "<std::io::stdio::StdoutLock as std::io::Write>::write_all" => NameRole::StdoutLockWriteAll,
        "std::thread::id::ThreadId::new::COUNTER" => NameRole::ThreadCounter,
        "std::thread::id::ThreadId::new::exhausted" => NameRole::ThreadExhausted,
        "core::fmt::write" => NameRole::FmtWrite,
        "core::option::expect_failed" => NameRole::ExpectFailed,
        "std::sys::sync::mutex::futex::Mutex::lock_contended" => NameRole::LockContended,
        "core::panicking::panic_fmt" => NameRole::PanicFmt,
        "core::ptr::drop_in_place<std::io::stdio::StdoutLock>"
        | "core::ptr::drop_in_place<std::io::stdio::StderrLock>" => NameRole::DropLock,
        "core::ptr::drop_in_place<std::io::error::Error>" => NameRole::DropError,
        "core::panicking::panic_in_cleanup" => NameRole::PanicCleanup,
        "<std::io::default_write_fmt::Adapter<T> as core::fmt::Write>::write_str" => {
            NameRole::AdapterWriteStr
        }
        "core::fmt::Write::write_char" => NameRole::FmtWriteChar,
        "core::fmt::Write::write_fmt" => NameRole::FmtWriteFmt,
        name if name
            .starts_with("core::ptr::drop_in_place<std::io::default_write_fmt::Adapter<")
            && name.ends_with(">>") =>
        {
            NameRole::DropAdapter
        }
        _ => return None,
    })
}

struct SymbolEvidence {
    role: Option<NameRole>,
    sizes: BTreeSet<u64>,
    kind: SymbolKind,
}

fn symbol_evidence(
    file: &object::File<'_>,
    addresses: &BTreeSet<u64>,
) -> BTreeMap<u64, SymbolEvidence> {
    let mut result: BTreeMap<u64, SymbolEvidence> = BTreeMap::new();
    for symbol in file.symbols().chain(file.dynamic_symbols()) {
        if !symbol.is_definition() || !addresses.contains(&symbol.address()) {
            continue;
        }
        let role = symbol
            .name()
            .ok()
            .and_then(|name| rustc_demangle::try_demangle(name).ok())
            .and_then(|name| name_role(&format!("{name:#}")));
        let entry = result
            .entry(symbol.address())
            .or_insert_with(|| SymbolEvidence {
                role,
                sizes: BTreeSet::new(),
                kind: symbol.kind(),
            });
        if entry.role != role || entry.kind != symbol.kind() {
            entry.role = None;
        }
        if symbol.size() != 0 {
            entry.sizes.insert(symbol.size());
        }
    }
    result
}

#[derive(Clone, Copy)]
enum ReferenceKind {
    Name(NameRole),
    Import(&'static str),
    Plt(&'static str),
    AdapterTable,
    PanicTemplate,
    PanicLocation,
}

struct BodyPattern {
    bytes: &'static [u8],
    references: &'static [(usize, ReferenceKind)],
}

impl BodyPattern {
    fn references(&self, address: u64, bytes: &[u8]) -> Option<Vec<(ReferenceKind, u64)>> {
        if bytes.len() != self.bytes.len() {
            return None;
        }
        let mut masked = bytes.to_vec();
        let mut targets = Vec::new();
        for &(offset, kind) in self.references {
            let end = offset.checked_add(4)?;
            let displacement = i32::from_le_bytes(bytes.get(offset..end)?.try_into().ok()?);
            let target = address
                .checked_add(end as u64)?
                .checked_add_signed(i64::from(displacement))?;
            masked.get_mut(offset..end)?.fill(0);
            targets.push((kind, target));
        }
        (masked == self.bytes).then_some(targets)
    }
}

const REFERENCE_CELL: BodyPattern = BodyPattern {
    bytes: &[
        0x55, 0x48, 0x89, 0xe5, 0x41, 0x57, 0x41, 0x56, 0x41, 0x54, 0x53, 0x48, 0x83, 0xec, 0x20,
        0x48, 0x89, 0xd3, 0x49, 0x89, 0xf6, 0x48, 0x8b, 0x07, 0x4c, 0x8b, 0x38, 0x64, 0x4c, 0x8b,
        0x24, 0x25, 0xe8, 0xff, 0xff, 0xff, 0x4d, 0x85, 0xe4, 0x74, 0x1d, 0x49, 0x8b, 0x07, 0x49,
        0x39, 0xc4, 0x75, 0x49, 0x41, 0x8b, 0x47, 0x0c, 0x83, 0xf8, 0xff, 0x0f, 0x84, 0x10, 0x01,
        0x00, 0x00, 0xff, 0xc0, 0x41, 0x89, 0x47, 0x0c, 0xeb, 0x52, 0x48, 0x8b, 0x05, 0x00, 0x00,
        0x00, 0x00, 0x0f, 0x1f, 0x00, 0x48, 0x83, 0xf8, 0xff, 0x0f, 0x84, 0xef, 0x00, 0x00, 0x00,
        0x4c, 0x8d, 0x60, 0x01, 0xf0, 0x4c, 0x0f, 0xb1, 0x25, 0x00, 0x00, 0x00, 0x00, 0x75, 0xe7,
        0x64, 0x4c, 0x89, 0x24, 0x25, 0xe8, 0xff, 0xff, 0xff, 0x49, 0x8b, 0x07, 0x49, 0x39, 0xc4,
        0x74, 0xb7, 0xb9, 0x01, 0x00, 0x00, 0x00, 0x31, 0xc0, 0xf0, 0x41, 0x0f, 0xb1, 0x4f, 0x08,
        0x0f, 0x85, 0xc6, 0x00, 0x00, 0x00, 0x4d, 0x89, 0x27, 0x41, 0xc7, 0x47, 0x0c, 0x01, 0x00,
        0x00, 0x00, 0x4c, 0x89, 0x7d, 0xd8, 0x48, 0x8d, 0x45, 0xd8, 0x48, 0x89, 0x45, 0xc8, 0x48,
        0xc7, 0x45, 0xd0, 0x00, 0x00, 0x00, 0x00, 0x48, 0x8d, 0x35, 0x00, 0x00, 0x00, 0x00, 0x48,
        0x8d, 0x7d, 0xc8, 0x4c, 0x89, 0xf2, 0x48, 0x89, 0xd9, 0xe8, 0x00, 0x00, 0x00, 0x00, 0x89,
        0xc1, 0x48, 0x8b, 0x45, 0xd0, 0x84, 0xc9, 0x74, 0x14, 0x48, 0x85, 0xc0, 0x0f, 0x84, 0x8c,
        0x00, 0x00, 0x00, 0x48, 0x8b, 0x75, 0xd8, 0xff, 0x4e, 0x0c, 0x75, 0x28, 0xeb, 0x15, 0x89,
        0xc1, 0x83, 0xe1, 0x03, 0x83, 0xf9, 0x01, 0x74, 0x29, 0x31, 0xc0, 0x48, 0x8b, 0x75, 0xd8,
        0xff, 0x4e, 0x0c, 0x75, 0x11, 0x48, 0xc7, 0x06, 0x00, 0x00, 0x00, 0x00, 0x31, 0xc9, 0x87,
        0x4e, 0x08, 0x83, 0xf9, 0x02, 0x74, 0x75, 0x48, 0x83, 0xc4, 0x20, 0x5b, 0x41, 0x5c, 0x41,
        0x5e, 0x41, 0x5f, 0x5d, 0xc3, 0x4c, 0x8d, 0x70, 0xff, 0x4c, 0x8b, 0x78, 0xff, 0x4c, 0x8b,
        0x60, 0x07, 0x49, 0x8b, 0x04, 0x24, 0x48, 0x85, 0xc0, 0x74, 0x05, 0x4c, 0x89, 0xff, 0xff,
        0xd0, 0x49, 0x83, 0x7c, 0x24, 0x08, 0x00, 0x74, 0x09, 0x4c, 0x89, 0xff, 0xff, 0x15, 0x00,
        0x00, 0x00, 0x00, 0x4c, 0x89, 0xf7, 0xff, 0x15, 0x00, 0x00, 0x00, 0x00, 0xeb, 0xa1, 0xe8,
        0x00, 0x00, 0x00, 0x00, 0xe8, 0x00, 0x00, 0x00, 0x00, 0x49, 0x8d, 0x7f, 0x08, 0xe8, 0x00,
        0x00, 0x00, 0x00, 0xe9, 0x2c, 0xff, 0xff, 0xff, 0x48, 0x8d, 0x3d, 0x00, 0x00, 0x00, 0x00,
        0x48, 0x8d, 0x15, 0x00, 0x00, 0x00, 0x00, 0xbe, 0xad, 0x00, 0x00, 0x00, 0xe8, 0x00, 0x00,
        0x00, 0x00, 0x0f, 0x0b, 0x48, 0x83, 0xc6, 0x08, 0xbf, 0xca, 0x00, 0x00, 0x00, 0xba, 0x81,
        0x00, 0x00, 0x00, 0xb9, 0x01, 0x00, 0x00, 0x00, 0x48, 0x89, 0xc3, 0x31, 0xc0, 0xff, 0x15,
        0x00, 0x00, 0x00, 0x00, 0x48, 0x89, 0xd8, 0xe9, 0x65, 0xff, 0xff, 0xff, 0x48, 0x89, 0xc3,
        0x49, 0x83, 0x7c, 0x24, 0x08, 0x00, 0x74, 0x09, 0x4c, 0x89, 0xff, 0xff, 0x15, 0x00, 0x00,
        0x00, 0x00, 0x4c, 0x89, 0xf7, 0xff, 0x15, 0x00, 0x00, 0x00, 0x00, 0x48, 0x8b, 0x7d, 0xd8,
        0xe8, 0x00, 0x00, 0x00, 0x00, 0x48, 0x89, 0xdf, 0xe8, 0x00, 0x00, 0x00, 0x00, 0x48, 0x89,
        0xc3, 0x48, 0x8b, 0x7d, 0xd0, 0x48, 0x85, 0xff, 0x74, 0x05, 0xe8, 0x00, 0x00, 0x00, 0x00,
        0x48, 0x8b, 0x7d, 0xd8, 0xe8, 0x00, 0x00, 0x00, 0x00, 0x48, 0x89, 0xdf, 0xe8, 0x00, 0x00,
        0x00, 0x00, 0xe8, 0x00, 0x00, 0x00, 0x00,
    ],
    references: &[
        (73, ReferenceKind::Name(NameRole::ThreadCounter)),
        (99, ReferenceKind::Name(NameRole::ThreadCounter)),
        (175, ReferenceKind::AdapterTable),
        (190, ReferenceKind::Name(NameRole::FmtWrite)),
        (314, ReferenceKind::Import("free")),
        (323, ReferenceKind::Import("free")),
        (330, ReferenceKind::Name(NameRole::ThreadExhausted)),
        (335, ReferenceKind::Name(NameRole::ExpectFailed)),
        (344, ReferenceKind::Name(NameRole::LockContended)),
        (356, ReferenceKind::PanicTemplate),
        (363, ReferenceKind::PanicLocation),
        (373, ReferenceKind::Name(NameRole::PanicFmt)),
        (405, ReferenceKind::Import("syscall")),
        (433, ReferenceKind::Import("free")),
        (442, ReferenceKind::Import("free")),
        (451, ReferenceKind::Name(NameRole::DropLock)),
        (459, ReferenceKind::Plt("_Unwind_Resume")),
        (476, ReferenceKind::Name(NameRole::DropError)),
        (485, ReferenceKind::Name(NameRole::DropLock)),
        (493, ReferenceKind::Plt("_Unwind_Resume")),
        (498, ReferenceKind::Name(NameRole::PanicCleanup)),
    ],
};

const PROMOTED_INNER: BodyPattern = BodyPattern {
    bytes: &[
        0x55, 0x48, 0x89, 0xe5, 0x41, 0x57, 0x41, 0x56, 0x41, 0x54, 0x53, 0x48, 0x83, 0xec, 0x20,
        0x48, 0x89, 0xd3, 0x49, 0x89, 0xf6, 0x64, 0x4c, 0x8b, 0x3c, 0x25, 0xe8, 0xff, 0xff, 0xff,
        0x4d, 0x85, 0xff, 0x74, 0x1b, 0x48, 0x8b, 0x07, 0x49, 0x39, 0xc7, 0x75, 0x4f, 0x8b, 0x47,
        0x0c, 0x83, 0xf8, 0xff, 0x0f, 0x84, 0x15, 0x01, 0x00, 0x00, 0xff, 0xc0, 0x89, 0x47, 0x0c,
        0xeb, 0x58, 0x48, 0x8b, 0x05, 0x00, 0x00, 0x00, 0x00, 0x66, 0x66, 0x2e, 0x0f, 0x1f, 0x84,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x48, 0x83, 0xf8, 0xff, 0x0f, 0x84, 0xed, 0x00, 0x00, 0x00,
        0x4c, 0x8d, 0x78, 0x01, 0xf0, 0x4c, 0x0f, 0xb1, 0x3d, 0x00, 0x00, 0x00, 0x00, 0x75, 0xe7,
        0x64, 0x4c, 0x89, 0x3c, 0x25, 0xe8, 0xff, 0xff, 0xff, 0x48, 0x8b, 0x07, 0x49, 0x39, 0xc7,
        0x74, 0xb1, 0xb9, 0x01, 0x00, 0x00, 0x00, 0x31, 0xc0, 0xf0, 0x0f, 0xb1, 0x4f, 0x08, 0x0f,
        0x85, 0xc5, 0x00, 0x00, 0x00, 0x4c, 0x89, 0x3f, 0xc7, 0x47, 0x0c, 0x01, 0x00, 0x00, 0x00,
        0x48, 0x89, 0x7d, 0xd8, 0x48, 0x8d, 0x45, 0xd8, 0x48, 0x89, 0x45, 0xc8, 0x48, 0xc7, 0x45,
        0xd0, 0x00, 0x00, 0x00, 0x00, 0x48, 0x8d, 0x35, 0x00, 0x00, 0x00, 0x00, 0x48, 0x8d, 0x7d,
        0xc8, 0x4c, 0x89, 0xf2, 0x48, 0x89, 0xd9, 0xe8, 0x00, 0x00, 0x00, 0x00, 0x89, 0xc1, 0x48,
        0x8b, 0x45, 0xd0, 0x84, 0xc9, 0x74, 0x14, 0x48, 0x85, 0xc0, 0x0f, 0x84, 0x95, 0x00, 0x00,
        0x00, 0x48, 0x8b, 0x75, 0xd8, 0xff, 0x4e, 0x0c, 0x75, 0x28, 0xeb, 0x15, 0x89, 0xc1, 0x83,
        0xe1, 0x03, 0x83, 0xf9, 0x01, 0x74, 0x29, 0x31, 0xc0, 0x48, 0x8b, 0x75, 0xd8, 0xff, 0x4e,
        0x0c, 0x75, 0x11, 0x48, 0xc7, 0x06, 0x00, 0x00, 0x00, 0x00, 0x31, 0xc9, 0x87, 0x4e, 0x08,
        0x83, 0xf9, 0x02, 0x74, 0x7e, 0x48, 0x83, 0xc4, 0x20, 0x5b, 0x41, 0x5c, 0x41, 0x5e, 0x41,
        0x5f, 0x5d, 0xc3, 0x4c, 0x8d, 0x70, 0xff, 0x4c, 0x8b, 0x78, 0xff, 0x4c, 0x8b, 0x60, 0x07,
        0x49, 0x8b, 0x04, 0x24, 0x48, 0x85, 0xc0, 0x74, 0x05, 0x4c, 0x89, 0xff, 0xff, 0xd0, 0x49,
        0x83, 0x7c, 0x24, 0x08, 0x00, 0x74, 0x09, 0x4c, 0x89, 0xff, 0xff, 0x15, 0x00, 0x00, 0x00,
        0x00, 0x4c, 0x89, 0xf7, 0xff, 0x15, 0x00, 0x00, 0x00, 0x00, 0xeb, 0xa1, 0xe8, 0x00, 0x00,
        0x00, 0x00, 0xe8, 0x00, 0x00, 0x00, 0x00, 0x48, 0x8d, 0x47, 0x08, 0x49, 0x89, 0xfc, 0x48,
        0x89, 0xc7, 0xe8, 0x00, 0x00, 0x00, 0x00, 0x4c, 0x89, 0xe7, 0xe9, 0x24, 0xff, 0xff, 0xff,
        0x48, 0x8d, 0x3d, 0x00, 0x00, 0x00, 0x00, 0x48, 0x8d, 0x15, 0x00, 0x00, 0x00, 0x00, 0xbe,
        0xad, 0x00, 0x00, 0x00, 0xe8, 0x00, 0x00, 0x00, 0x00, 0x0f, 0x0b, 0x48, 0x83, 0xc6, 0x08,
        0xbf, 0xca, 0x00, 0x00, 0x00, 0xba, 0x81, 0x00, 0x00, 0x00, 0xb9, 0x01, 0x00, 0x00, 0x00,
        0x48, 0x89, 0xc3, 0x31, 0xc0, 0xff, 0x15, 0x00, 0x00, 0x00, 0x00, 0x48, 0x89, 0xd8, 0xe9,
        0x5c, 0xff, 0xff, 0xff, 0x48, 0x89, 0xc3, 0x49, 0x83, 0x7c, 0x24, 0x08, 0x00, 0x74, 0x09,
        0x4c, 0x89, 0xff, 0xff, 0x15, 0x00, 0x00, 0x00, 0x00, 0x4c, 0x89, 0xf7, 0xff, 0x15, 0x00,
        0x00, 0x00, 0x00, 0x48, 0x8b, 0x7d, 0xd8, 0xe8, 0x00, 0x00, 0x00, 0x00, 0x48, 0x89, 0xdf,
        0xe8, 0x00, 0x00, 0x00, 0x00, 0x48, 0x89, 0xc3, 0x48, 0x8b, 0x7d, 0xd0, 0x48, 0x85, 0xff,
        0x74, 0x05, 0xe8, 0x00, 0x00, 0x00, 0x00, 0x48, 0x8b, 0x7d, 0xd8, 0xe8, 0x00, 0x00, 0x00,
        0x00, 0x48, 0x89, 0xdf, 0xe8, 0x00, 0x00, 0x00, 0x00, 0xe8, 0x00, 0x00, 0x00, 0x00,
    ],
    references: &[
        (65, ReferenceKind::Name(NameRole::ThreadCounter)),
        (99, ReferenceKind::Name(NameRole::ThreadCounter)),
        (173, ReferenceKind::AdapterTable),
        (188, ReferenceKind::Name(NameRole::FmtWrite)),
        (312, ReferenceKind::Import("free")),
        (321, ReferenceKind::Import("free")),
        (328, ReferenceKind::Name(NameRole::ThreadExhausted)),
        (333, ReferenceKind::Name(NameRole::ExpectFailed)),
        (348, ReferenceKind::Name(NameRole::LockContended)),
        (363, ReferenceKind::PanicTemplate),
        (370, ReferenceKind::PanicLocation),
        (380, ReferenceKind::Name(NameRole::PanicFmt)),
        (412, ReferenceKind::Import("syscall")),
        (440, ReferenceKind::Import("free")),
        (449, ReferenceKind::Import("free")),
        (458, ReferenceKind::Name(NameRole::DropLock)),
        (466, ReferenceKind::Plt("_Unwind_Resume")),
        (483, ReferenceKind::Name(NameRole::DropError)),
        (492, ReferenceKind::Name(NameRole::DropLock)),
        (500, ReferenceKind::Plt("_Unwind_Resume")),
        (505, ReferenceKind::Name(NameRole::PanicCleanup)),
    ],
};

const PANIC_TEMPLATE: &[u8] =
    b"a formatting trait implementation returned an error when the underlying stream did not";
const PANIC_FILE: &[u8] = b"library/std/src/io/mod.rs";

fn word(bytes: &[u8], offset: usize) -> Option<u64> {
    Some(u64::from_le_bytes(
        bytes.get(offset..offset.checked_add(8)?)?.try_into().ok()?,
    ))
}

fn panic_location(image: &Image<'_, '_>, address: u64) -> Option<Vec<ImageSpan>> {
    if address % 8 != 0 {
        return None;
    }
    let location = image.protected(address, 24)?;
    let path_address = word(&location.bytes, 0)?;
    if word(&location.bytes, 8)? != PANIC_FILE.len() as u64
        || location.bytes.get(16..20)? != 648u32.to_le_bytes()
        || location.bytes.get(20..24)? != 17u32.to_le_bytes()
    {
        return None;
    }
    let path = image.protected(path_address, PANIC_FILE.len())?;
    (path.bytes == PANIC_FILE).then_some(vec![location, path])
}

fn candidate(
    image: &Image<'_, '_>,
    address: u64,
    pattern: &BodyPattern,
    receiver: ReceiverAbi,
) -> Option<WriterProfile> {
    let body = image.code(address, pattern.bytes.len())?;
    let mut references = pattern.references(address, &body.bytes)?;
    let table_address = references.iter().find_map(|&(kind, target)| {
        matches!(kind, ReferenceKind::AdapterTable).then_some(target)
    })?;
    if table_address % 8 != 0 {
        return None;
    }
    let table = image.protected(table_address, 48)?;
    if word(&table.bytes, 8)? != 16 || word(&table.bytes, 16)? != 8 {
        return None;
    }
    let callbacks = [
        (word(&table.bytes, 0)?, NameRole::DropAdapter),
        (word(&table.bytes, 24)?, NameRole::AdapterWriteStr),
        (word(&table.bytes, 32)?, NameRole::FmtWriteChar),
        (word(&table.bytes, 40)?, NameRole::FmtWriteFmt),
    ];
    let mut protected = vec![body, table];
    let mut extents = BTreeMap::from([(address, pattern.bytes.len() as u64)]);
    for ((target, _), callback) in callbacks.iter().zip([
        &DROP_ADAPTER,
        &ADAPTER_WRITE_STR,
        &ADAPTER_WRITE_CHAR,
        &ADAPTER_WRITE_FMT,
    ]) {
        let body = image.code(*target, callback.bytes.len())?;
        references.extend(callback.references(*target, &body.bytes)?);
        extents.insert(*target, callback.bytes.len() as u64);
        protected.push(body);
    }
    let mut named = vec![(address, NameRole::StdoutWriteFmt)];
    named.extend(references.iter().filter_map(|&(kind, target)| {
        if let ReferenceKind::Name(role) = kind {
            Some((target, role))
        } else {
            None
        }
    }));
    named.extend(callbacks);
    let addresses = named.iter().map(|&(target, _)| target).collect();
    let symbols = symbol_evidence(image.file, &addresses);
    let mut identities = BTreeMap::new();
    for &(target, role) in &named {
        let evidence = symbols.get(&target)?;
        let data = role == NameRole::ThreadCounter;
        if evidence.role != Some(role)
            || evidence.kind
                != if data {
                    SymbolKind::Data
                } else {
                    SymbolKind::Text
                }
            || evidence.sizes.len() != 1
            || target == 0
        {
            return None;
        }
        let size = *evidence.sizes.first()?;
        if (data && (size != 8 || target % 8 != 0))
            || extents
                .get(&target)
                .is_some_and(|expected| *expected != size)
            || identities
                .insert(role, target)
                .is_some_and(|old| old != target)
        {
            return None;
        }
    }
    for (kind, target) in references {
        match kind {
            ReferenceKind::Name(_) => {}
            ReferenceKind::AdapterTable => {
                if target != table_address {
                    return None;
                }
            }
            ReferenceKind::Import(name) => {
                if !image.imported(target, name) {
                    return None;
                }
            }
            ReferenceKind::Plt(name) => protected.push(image.plt(target, name)?),
            ReferenceKind::PanicTemplate => {
                let span = image.protected(target, PANIC_TEMPLATE.len())?;
                if span.bytes != PANIC_TEMPLATE {
                    return None;
                }
                protected.push(span);
            }
            ReferenceKind::PanicLocation => protected.extend(panic_location(image, target)?),
        }
    }
    protected.sort_by_key(|span| (span.address, span.bytes.len()));
    protected.dedup_by(|left, right| left.address == right.address && left.bytes == right.bytes);
    Some(WriterProfile {
        receiver,
        protected,
        relro: image.relro,
    })
}

const DROP_ADAPTER: BodyPattern = BodyPattern {
    bytes: &[
        0x55, 0x48, 0x89, 0xe5, 0x41, 0x57, 0x41, 0x56, 0x41, 0x54, 0x53, 0x48, 0x8b, 0x47, 0x08,
        0x89, 0xc1, 0x83, 0xe1, 0x03, 0x83, 0xf9, 0x01, 0x74, 0x09, 0x5b, 0x41, 0x5c, 0x41, 0x5e,
        0x41, 0x5f, 0x5d, 0xc3, 0x48, 0x8d, 0x58, 0xff, 0x4c, 0x8b, 0x70, 0xff, 0x4c, 0x8b, 0x60,
        0x07, 0x49, 0x8b, 0x04, 0x24, 0x48, 0x85, 0xc0, 0x74, 0x05, 0x4c, 0x89, 0xf7, 0xff, 0xd0,
        0x49, 0x83, 0x7c, 0x24, 0x08, 0x00, 0x74, 0x09, 0x4c, 0x89, 0xf7, 0xff, 0x15, 0x00, 0x00,
        0x00, 0x00, 0x48, 0x89, 0xdf, 0x5b, 0x41, 0x5c, 0x41, 0x5e, 0x41, 0x5f, 0x5d, 0xff, 0x25,
        0x00, 0x00, 0x00, 0x00, 0x49, 0x89, 0xc7, 0x49, 0x83, 0x7c, 0x24, 0x08, 0x00, 0x74, 0x09,
        0x4c, 0x89, 0xf7, 0xff, 0x15, 0x00, 0x00, 0x00, 0x00, 0x48, 0x89, 0xdf, 0xff, 0x15, 0x00,
        0x00, 0x00, 0x00, 0x4c, 0x89, 0xff, 0xe8, 0x00, 0x00, 0x00, 0x00,
    ],
    references: &[
        (73, ReferenceKind::Import("free")),
        (90, ReferenceKind::Import("free")),
        (110, ReferenceKind::Import("free")),
        (119, ReferenceKind::Import("free")),
        (127, ReferenceKind::Plt("_Unwind_Resume")),
    ],
};

const ADAPTER_WRITE_STR: BodyPattern = BodyPattern {
    bytes: &[
        0x55, 0x48, 0x89, 0xe5, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x53, 0x50, 0x48,
        0x89, 0xfb, 0x48, 0x8b, 0x07, 0x48, 0x8b, 0x38, 0xe8, 0x00, 0x00, 0x00, 0x00, 0x49, 0x89,
        0xc6, 0x48, 0x85, 0xc0, 0x74, 0x12, 0x48, 0x8b, 0x43, 0x08, 0x89, 0xc1, 0x83, 0xe1, 0x03,
        0x83, 0xf9, 0x01, 0x74, 0x19, 0x4c, 0x89, 0x73, 0x08, 0x4d, 0x85, 0xf6, 0x0f, 0x95, 0xc0,
        0x48, 0x83, 0xc4, 0x08, 0x5b, 0x41, 0x5c, 0x41, 0x5d, 0x41, 0x5e, 0x41, 0x5f, 0x5d, 0xc3,
        0x4c, 0x8d, 0x78, 0xff, 0x4c, 0x8b, 0x60, 0xff, 0x4c, 0x8b, 0x68, 0x07, 0x49, 0x8b, 0x45,
        0x00, 0x48, 0x85, 0xc0, 0x74, 0x05, 0x4c, 0x89, 0xe7, 0xff, 0xd0, 0x49, 0x83, 0x7d, 0x08,
        0x00, 0x74, 0x09, 0x4c, 0x89, 0xe7, 0xff, 0x15, 0x00, 0x00, 0x00, 0x00, 0x4c, 0x89, 0xff,
        0xff, 0x15, 0x00, 0x00, 0x00, 0x00, 0xeb, 0xb2, 0x48, 0x89, 0x45, 0xd0, 0x49, 0x83, 0x7d,
        0x08, 0x00, 0x74, 0x09, 0x4c, 0x89, 0xe7, 0xff, 0x15, 0x00, 0x00, 0x00, 0x00, 0x4c, 0x89,
        0xff, 0xff, 0x15, 0x00, 0x00, 0x00, 0x00, 0x4c, 0x89, 0x73, 0x08, 0x48, 0x8b, 0x7d, 0xd0,
        0xe8, 0x00, 0x00, 0x00, 0x00,
    ],
    references: &[
        (24, ReferenceKind::Name(NameRole::StdoutLockWriteAll)),
        (113, ReferenceKind::Import("free")),
        (122, ReferenceKind::Import("free")),
        (144, ReferenceKind::Import("free")),
        (153, ReferenceKind::Import("free")),
        (166, ReferenceKind::Plt("_Unwind_Resume")),
    ],
};

const ADAPTER_WRITE_CHAR: BodyPattern = BodyPattern {
    bytes: &[
        0x55, 0x48, 0x89, 0xe5, 0x41, 0x57, 0x41, 0x56, 0x41, 0x55, 0x41, 0x54, 0x53, 0x48, 0x83,
        0xec, 0x18, 0x48, 0x89, 0xfb, 0xc7, 0x45, 0xd4, 0x00, 0x00, 0x00, 0x00, 0x81, 0xfe, 0x80,
        0x00, 0x00, 0x00, 0x73, 0x0b, 0x40, 0x88, 0x75, 0xd4, 0xba, 0x01, 0x00, 0x00, 0x00, 0xeb,
        0x68, 0x89, 0xf0, 0x24, 0x3f, 0x0c, 0x80, 0x89, 0xf1, 0xc1, 0xe9, 0x06, 0x81, 0xfe, 0x00,
        0x08, 0x00, 0x00, 0x73, 0x10, 0x80, 0xc9, 0xc0, 0x88, 0x4d, 0xd4, 0x88, 0x45, 0xd5, 0xba,
        0x02, 0x00, 0x00, 0x00, 0xeb, 0x45, 0x80, 0xe1, 0x3f, 0x80, 0xc9, 0x80, 0x89, 0xf2, 0xc1,
        0xea, 0x0c, 0x81, 0xfe, 0x00, 0x00, 0x01, 0x00, 0x73, 0x13, 0x80, 0xca, 0xe0, 0x88, 0x55,
        0xd4, 0x88, 0x4d, 0xd5, 0x88, 0x45, 0xd6, 0xba, 0x03, 0x00, 0x00, 0x00, 0xeb, 0x1f, 0x80,
        0xe2, 0x3f, 0x80, 0xca, 0x80, 0xc1, 0xee, 0x12, 0x40, 0x80, 0xce, 0xf0, 0x40, 0x88, 0x75,
        0xd4, 0x88, 0x55, 0xd5, 0x88, 0x4d, 0xd6, 0x88, 0x45, 0xd7, 0xba, 0x04, 0x00, 0x00, 0x00,
        0x48, 0x8b, 0x03, 0x48, 0x8b, 0x38, 0x48, 0x8d, 0x75, 0xd4, 0xe8, 0x00, 0x00, 0x00, 0x00,
        0x49, 0x89, 0xc6, 0x48, 0x85, 0xc0, 0x74, 0x12, 0x48, 0x8b, 0x43, 0x08, 0x89, 0xc1, 0x83,
        0xe1, 0x03, 0x83, 0xf9, 0x01, 0x74, 0x19, 0x4c, 0x89, 0x73, 0x08, 0x4d, 0x85, 0xf6, 0x0f,
        0x95, 0xc0, 0x48, 0x83, 0xc4, 0x18, 0x5b, 0x41, 0x5c, 0x41, 0x5d, 0x41, 0x5e, 0x41, 0x5f,
        0x5d, 0xc3, 0x4c, 0x8d, 0x78, 0xff, 0x4c, 0x8b, 0x60, 0xff, 0x4c, 0x8b, 0x68, 0x07, 0x49,
        0x8b, 0x45, 0x00, 0x48, 0x85, 0xc0, 0x74, 0x05, 0x4c, 0x89, 0xe7, 0xff, 0xd0, 0x49, 0x83,
        0x7d, 0x08, 0x00, 0x74, 0x09, 0x4c, 0x89, 0xe7, 0xff, 0x15, 0x00, 0x00, 0x00, 0x00, 0x4c,
        0x89, 0xff, 0xff, 0x15, 0x00, 0x00, 0x00, 0x00, 0xeb, 0xb2, 0x48, 0x89, 0x45, 0xc8, 0x49,
        0x83, 0x7d, 0x08, 0x00, 0x74, 0x09, 0x4c, 0x89, 0xe7, 0xff, 0x15, 0x00, 0x00, 0x00, 0x00,
        0x4c, 0x89, 0xff, 0xff, 0x15, 0x00, 0x00, 0x00, 0x00, 0x4c, 0x89, 0x73, 0x08, 0x48, 0x8b,
        0x7d, 0xc8, 0xe8, 0x00, 0x00, 0x00, 0x00,
    ],
    references: &[
        (161, ReferenceKind::Name(NameRole::StdoutLockWriteAll)),
        (250, ReferenceKind::Import("free")),
        (259, ReferenceKind::Import("free")),
        (281, ReferenceKind::Import("free")),
        (290, ReferenceKind::Import("free")),
        (303, ReferenceKind::Plt("_Unwind_Resume")),
    ],
};

const ADAPTER_WRITE_FMT: BodyPattern = BodyPattern {
    bytes: &[
        0x48, 0x89, 0xd1, 0x48, 0x89, 0xf2, 0x48, 0x8d, 0x35, 0x00, 0x00, 0x00, 0x00, 0xe9, 0x00,
        0x00, 0x00, 0x00,
    ],
    references: &[
        (9, ReferenceKind::AdapterTable),
        (14, ReferenceKind::Name(NameRole::FmtWrite)),
    ],
};

pub(super) fn collect(
    file: &object::File<'_>,
    bytes: &[u8],
    symbols: &[(u64, Option<String>)],
) -> BTreeMap<u64, WriterProfile> {
    if file.format() != object::BinaryFormat::Elf
        || file.architecture() != object::Architecture::X86_64
        || !file.is_64()
        || !file.is_little_endian()
        || !matches!(
            file.kind(),
            object::ObjectKind::Executable | object::ObjectKind::Dynamic
        )
    {
        return BTreeMap::new();
    }
    let candidates: BTreeSet<_> = symbols
        .iter()
        .filter_map(|(address, name)| {
            (name.as_deref() == Some(STDOUT_WRITE_FMT)).then_some(*address)
        })
        .collect();
    if candidates.is_empty() {
        return BTreeMap::new();
    }
    let image = Image::new(file, bytes);
    candidates
        .into_iter()
        .filter_map(|address| {
            let mut profiles = [
                (&REFERENCE_CELL, ReceiverAbi::ReferenceCell),
                (&PROMOTED_INNER, ReceiverAbi::PromotedInner),
            ]
            .into_iter()
            .filter_map(|(pattern, receiver)| candidate(&image, address, pattern, receiver));
            let profile = profiles.next()?;
            profiles.next().is_none().then_some((address, profile))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const NATIVE: &[u8] = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/rust_fmt_io_x86_64"
    ));
    const PROMOTED: &[u8] = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/rust_fmt_io_promoted_x86_64"
    ));

    fn symbols(file: &object::File<'_>) -> Vec<(u64, Option<String>)> {
        file.symbols()
            .chain(file.dynamic_symbols())
            .filter(|symbol| symbol.is_definition() && symbol.kind() == SymbolKind::Text)
            .map(|symbol| {
                (
                    symbol.address(),
                    symbol
                        .name()
                        .ok()
                        .and_then(|name| rustc_demangle::try_demangle(name).ok())
                        .map(|name| format!("{name:#}")),
                )
            })
            .collect()
    }

    fn profiles(bytes: &[u8]) -> BTreeMap<u64, WriterProfile> {
        let Ok(file) = object::File::parse(bytes) else {
            return BTreeMap::new();
        };
        collect(&file, bytes, &symbols(&file))
    }

    fn file_offset(bytes: &[u8], address: u64, size: usize) -> usize {
        let file = object::File::parse(bytes).unwrap();
        file.segments()
            .find_map(|segment| {
                let delta = address.checked_sub(segment.address())?;
                let (offset, file_size) = segment.file_range();
                (delta.checked_add(size as u64)? <= file_size).then_some((offset + delta) as usize)
            })
            .unwrap()
    }

    fn method(bytes: &[u8]) -> u64 {
        let file = object::File::parse(bytes).unwrap();
        let addresses: BTreeSet<_> = symbols(&file)
            .into_iter()
            .filter_map(|(address, name)| {
                (name.as_deref() == Some(STDOUT_WRITE_FMT)).then_some(address)
            })
            .collect();
        assert_eq!(addresses.len(), 1);
        *addresses.first().unwrap()
    }

    fn adapter_table(image: &Image<'_, '_>, address: u64, pattern: &BodyPattern) -> u64 {
        let body = image.code(address, pattern.bytes.len()).unwrap();
        pattern
            .references(address, &body.bytes)
            .unwrap()
            .into_iter()
            .find_map(|(kind, target)| {
                matches!(kind, ReferenceKind::AdapterTable).then_some(target)
            })
            .unwrap()
    }

    #[test]
    fn complete_native_bodies_establish_both_receiver_conventions() {
        for (bytes, receiver) in [
            (NATIVE, ReceiverAbi::ReferenceCell),
            (PROMOTED, ReceiverAbi::PromotedInner),
        ] {
            let found = profiles(bytes);
            assert_eq!(found.len(), 1);
            let profile = &found[&method(bytes)];
            assert_eq!(profile.receiver, receiver);
            assert!(profile.protected.len() >= 10);
        }
    }

    #[test]
    fn every_method_and_callback_byte_is_constrained() {
        for (bytes, pattern) in [(NATIVE, &REFERENCE_CELL), (PROMOTED, &PROMOTED_INNER)] {
            let address = method(bytes);
            let file = object::File::parse(bytes).unwrap();
            let image = Image::new(&file, bytes);
            let table = adapter_table(&image, address, pattern);
            let table = image.protected(table, 48).unwrap();
            let mut ranges = vec![(address, pattern.bytes.len())];
            ranges.extend(
                [(0, 131), (24, 170), (32, 307), (40, 18)]
                    .map(|(offset, size)| (word(&table.bytes, offset).unwrap(), size)),
            );
            for (target, size) in ranges {
                let start = file_offset(bytes, target, size);
                for offset in start..start + size {
                    let mut changed = bytes.to_vec();
                    changed[offset] ^= 1;
                    assert!(
                        profiles(&changed).is_empty(),
                        "accepted altered byte {offset:#x}"
                    );
                }
            }
        }
    }

    #[test]
    fn generic_callback_identity_cannot_substitute_another_writer() {
        let file = object::File::parse(NATIVE).unwrap();
        let address = method(NATIVE);
        let original = Image::new(&file, NATIVE);
        let table = adapter_table(&original, address, &REFERENCE_CELL);
        let callback = word(&original.protected(table, 48).unwrap().bytes, 24).unwrap();
        let alternatives: BTreeSet<_> = symbols(&file)
            .into_iter()
            .filter_map(|(target, name)| {
                (target != callback
                    && name.as_deref().and_then(name_role) == Some(NameRole::AdapterWriteStr))
                .then_some(target)
            })
            .collect();
        assert!(!alternatives.is_empty());
        let mut identical_bodies = 0;
        for alternate in alternatives {
            let Some(body) = original.code(alternate, ADAPTER_WRITE_STR.bytes.len()) else {
                continue;
            };
            if ADAPTER_WRITE_STR
                .references(alternate, &body.bytes)
                .is_some()
            {
                identical_bodies += 1;
            }
            let mut image = Image::new(&file, NATIVE);
            image
                .fixups
                .iter_mut()
                .find(|site| site.vma == table + 24)
                .unwrap()
                .target = Some(alternate);
            assert!(
                candidate(&image, address, &REFERENCE_CELL, ReceiverAbi::ReferenceCell).is_none()
            );
        }
        assert!(
            identical_bodies > 0,
            "fixture must include a same-body callback with a different writer call"
        );
    }

    #[test]
    fn unresolved_overlapping_and_partial_table_fixups_refuse_recovery() {
        let file = object::File::parse(NATIVE).unwrap();
        let address = method(NATIVE);
        for mutation in 0..6 {
            let mut image = Image::new(&file, NATIVE);
            let table = adapter_table(&image, address, &REFERENCE_CELL);
            let index = image
                .fixups
                .iter()
                .position(|site| site.vma == table + 24)
                .unwrap();
            match mutation {
                0 => image.fixups[index].target = None,
                1 => image.fixups[index].kind = FixupKind::Relocation,
                2 => image.fixups[index].width = 4,
                3 => image.fixups.push(image.fixups[index].clone()),
                4 => image.fixups[index].vma -= 1,
                _ => image.fixups[index].vma += 1,
            }
            assert!(
                candidate(&image, address, &REFERENCE_CELL, ReceiverAbi::ReferenceCell).is_none(),
                "mutation {mutation}"
            );
        }
    }

    #[test]
    fn imported_callee_roles_and_fixup_boundaries_are_exact() {
        let file = object::File::parse(NATIVE).unwrap();
        let address = method(NATIVE);
        for mutation in 0..7 {
            let mut image = Image::new(&file, NATIVE);
            let index = image
                .fixups
                .iter()
                .position(|site| site.symbol.as_deref() == Some("free"))
                .unwrap();
            match mutation {
                0 => image.fixups[index].symbol = Some("user_free".into()),
                1 => image.fixups[index].target = Some(0x1000),
                2 => image.fixups[index].addend = Some(1),
                3 => image.fixups[index].kind = FixupKind::Rebase,
                4 => image.fixups[index].width = 4,
                5 => image.fixups.push(image.fixups[index].clone()),
                _ => {
                    let mut partial = image.fixups[index].clone();
                    partial.vma -= 1;
                    image.fixups.push(partial);
                }
            }
            assert!(
                candidate(&image, address, &REFERENCE_CELL, ReceiverAbi::ReferenceCell).is_none(),
                "mutation {mutation}"
            );
        }
    }

    #[test]
    fn mutable_data_and_nonexecutable_code_do_not_establish_a_profile() {
        let phoff = word(NATIVE, 32).unwrap() as usize;
        let width = u16::from_le_bytes(NATIVE[54..56].try_into().unwrap()) as usize;
        let count = u16::from_le_bytes(NATIVE[56..58].try_into().unwrap()) as usize;
        for remove_relro in [false, true] {
            let mut changed = NATIVE.to_vec();
            let mut edits = 0;
            for index in 0..count {
                let header = phoff + index * width;
                let kind = u32::from_le_bytes(NATIVE[header..header + 4].try_into().unwrap());
                let flags = u32::from_le_bytes(NATIVE[header + 4..header + 8].try_into().unwrap());
                if remove_relro && kind == object::elf::PT_GNU_RELRO {
                    changed[header..header + 4]
                        .copy_from_slice(&object::elf::PT_NULL.to_le_bytes());
                    edits += 1;
                } else if !remove_relro
                    && kind == object::elf::PT_LOAD
                    && flags & object::elf::PF_X != 0
                {
                    changed[header + 4..header + 8]
                        .copy_from_slice(&(flags & !object::elf::PF_X).to_le_bytes());
                    edits += 1;
                }
            }
            assert!(edits > 0);
            assert!(profiles(&changed).is_empty());
        }
    }

    #[test]
    fn method_extent_and_every_alias_must_agree() {
        use object::ObjectSection;
        let file = object::File::parse(NATIVE).unwrap();
        let address = method(NATIVE);
        let table = file
            .section_by_name(".symtab")
            .unwrap()
            .file_range()
            .unwrap()
            .0 as usize;
        let original = file
            .symbols()
            .find(|symbol| symbol.address() == address)
            .unwrap();
        let spare = file
            .symbols()
            .find(|symbol| symbol.name().ok() == Some("write_unsigned"))
            .unwrap();
        let original = table + original.index().0 * 24;
        let spare = table + spare.index().0 * 24;
        for mutation in 0..4 {
            let mut changed = NATIVE.to_vec();
            match mutation {
                0 => changed[original + 16..original + 24].copy_from_slice(&503u64.to_le_bytes()),
                1 => changed[original..original + 4].copy_from_slice(&NATIVE[spare..spare + 4]),
                _ => {
                    changed[spare + 8..spare + 16].copy_from_slice(&address.to_le_bytes());
                    let size = if mutation == 2 { 502u64 } else { 0 };
                    changed[spare + 16..spare + 24].copy_from_slice(&size.to_le_bytes());
                }
            }
            assert!(profiles(&changed).is_empty(), "mutation {mutation}");
        }
    }

    #[test]
    fn other_object_kinds_and_architectures_are_not_receiver_evidence() {
        for (offset, value) in [(16, object::elf::ET_REL), (18, object::elf::EM_AARCH64)] {
            let mut changed = NATIVE.to_vec();
            changed[offset..offset + 2].copy_from_slice(&value.to_le_bytes());
            assert!(profiles(&changed).is_empty());
        }
    }
}

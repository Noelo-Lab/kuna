//! Linker evidence for Binder class factories and their static name arrays.
use std::collections::{BTreeMap, BTreeSet};

use object::{
    Architecture, Object, ObjectKind, ObjectSection, ObjectSymbol, SectionFlags, SectionKind,
    SymbolKind,
};

use crate::entry::kuna_fdeinterior::scan_fde_bodies;
use crate::loader::{elf_plt, kuna_dynrelocs, kuna_elfgotimports};

#[derive(Copy, Clone, Eq, PartialEq)]
pub(super) enum Api {
    Define,
    Map,
}

fn api(name: &[u8]) -> Option<Api> {
    match name {
        b"AIBinder_Class_define" => Some(Api::Define),
        b"AIBinder_Class_setTransactionCodeToFunctionNameMap" => Some(Api::Map),
        _ => None,
    }
}

fn uint(data: &[u8]) -> Option<u64> {
    let mut b = [0; 8];
    b.get_mut(..data.len())?.copy_from_slice(data);
    Some(u64::from_le_bytes(b))
}

fn binder_dependency(file: &object::File) -> Option<()> {
    let strings = file.section_by_name(".dynstr")?.data().ok()?;
    kuna_dynrelocs::dynamic_entries(file)
        .into_iter()
        .any(|(tag, val)| {
            tag == i64::from(object::elf::DT_NEEDED)
                && usize::try_from(val)
                    .ok()
                    .and_then(|i| strings.get(i..))
                    .is_some_and(|s| s.starts_with(b"libbinder_ndk.so\0"))
        })
        .then_some(())
}

pub(super) struct Image<'a> {
    pub(super) file: &'a object::File<'a>,
    pub(super) stubs: BTreeMap<u64, Api>,
    pub(super) got: BTreeMap<u64, Api>,
    pointers: std::cell::OnceCell<crate::loader::kuna_dataptrs::DataPointers>,
    bytes: &'a [u8],
    relro: Option<(u64, u64)>,
    pub(super) width: usize,
}

impl<'a> Image<'a> {
    pub(super) fn new(file: &'a object::File<'a>, bytes: &'a [u8]) -> Option<Self> {
        let width = if file.is_64() { 8 } else { 4 };
        if !file.is_little_endian()
            || !matches!(
                file.architecture(),
                Architecture::Aarch64
                    | Architecture::Arm
                    | Architecture::X86_64
                    | Architecture::I386
            )
        {
            return None;
        }
        let imports: BTreeSet<_> = file
            .dynamic_symbols()
            .filter(|s| s.is_undefined())
            .filter_map(|s| s.name_bytes().ok())
            .filter(|name| api(name).is_some())
            .collect();
        if !imports.contains(&b"AIBinder_Class_define"[..]) {
            return None;
        }
        binder_dependency(file)?;
        let stubs: BTreeMap<_, _> = elf_plt::resolve_plt_imports(file, bytes)
            .into_iter()
            .filter(|s| imports.contains(s.name.as_slice()))
            .filter_map(|s| Some((s.addr, api(&s.name)?)))
            .collect();
        let got: BTreeMap<_, _> = kuna_elfgotimports::resolve(file)
            .into_iter()
            .filter_map(|(at, n)| Some((at, api(&n)?)))
            .collect();
        if !stubs
            .values()
            .chain(got.values())
            .any(|&a| a == Api::Define)
        {
            return None;
        }
        Some(Self {
            file,
            width,
            stubs,
            got,
            bytes,
            pointers: Default::default(),
            relro: kuna_dynrelocs::relro_range(bytes),
        })
    }

    pub(super) fn data(&self, addr: u64) -> Option<&[u8]> {
        self.file.sections().find_map(|s| {
            let offset = usize::try_from(addr.checked_sub(s.address())?).ok()?;
            if s.address() == 0 || offset as u64 >= s.size() {
                return None;
            }
            s.data().ok()?.get(offset..)
        })
    }

    pub(super) fn text(&self, addr: u64) -> Option<&str> {
        let bytes = self.data(addr)?;
        let end = memchr::memchr(0, &bytes[..bytes.len().min(4096)])?;
        let name = std::str::from_utf8(&bytes[..end]).ok()?;
        (!name.is_empty()
            && !name.chars().any(char::is_control)
            && self.immutable(addr, (end + 1) as u64))
        .then_some(name)
    }

    pub(super) fn has_alias(&self, range: std::ops::Range<u64>) -> bool {
        self.file.dynamic_symbols().any(|s| {
            s.is_definition()
                && !s.is_local()
                && s.address() < range.end
                && s.address().saturating_add(s.size().max(1)) > range.start
        }) || self
            .pointers
            .get_or_init(|| crate::loader::kuna_dataptrs::data_pointers(self.file, self.bytes))
            .pointers
            .iter()
            .any(|ptr| range.contains(&ptr.target))
    }

    pub(super) fn immutable(&self, addr: u64, len: u64) -> bool {
        let Some(end) = addr.checked_add(len) else {
            return false;
        };
        self.relro.is_some_and(|(lo, hi)| lo <= addr && end <= hi)
            || self.file.sections().any(|s| {
                s.address() != 0
                    && s.address() <= addr
                    && end - s.address() <= s.size()
                    && matches!(s.flags(), SectionFlags::Elf{sh_flags} if sh_flags & 3 == 2)
            })
    }

    pub(super) fn pointer(
        &self,
        addr: u64,
        machine: &crate::analyzers::nativeflow::Machine,
    ) -> Option<u64> {
        if let Some(target) = machine.relocated(addr) {
            return Some(target);
        }
        let raw = uint(self.data(addr)?.get(..self.width)?)?;
        (raw == 0 || self.file.kind() == ObjectKind::Executable).then_some(raw)
    }

    pub(super) fn pc(&self, raw: u64) -> u64 {
        if self.file.architecture() == Architecture::Arm {
            raw & !1
        } else {
            raw
        }
    }

    pub(super) fn code(&self, raw: u64) -> bool {
        match self.file.architecture() {
            Architecture::Arm if raw & 3 == 2 => return false,
            Architecture::Aarch64 if raw & 3 != 0 => return false,
            _ => {}
        }
        let pc = self.pc(raw);
        self.file.sections().any(|s| {
            s.kind() == SectionKind::Text
                && !s.name().unwrap_or_default().starts_with(".plt")
                && pc.checked_sub(s.address()).is_some_and(|n| n < s.size())
        })
    }

    /// Sized symbols and FDEs survive independently; EHABI supplies starts on 32-bit ARM.
    pub(super) fn bodies(&self) -> BTreeMap<u64, u64> {
        let mut spans: BTreeMap<u64, u64> = scan_fde_bodies(self.file).into_iter().collect();
        if let Some(sec) = self.file.section_by_name(".ARM.exidx") {
            for (i, entry) in sec.data().unwrap_or_default().chunks_exact(8).enumerate() {
                let word = u32::from_le_bytes(entry[..4].try_into().unwrap());
                let delta = ((word << 1) as i32 >> 1) as i64;
                let pc = sec
                    .address()
                    .wrapping_add((i * 8) as u64)
                    .wrapping_add_signed(delta);
                if self.code(pc) {
                    spans.entry(pc).or_insert(0);
                }
            }
        }
        for sym in self
            .file
            .symbols()
            .chain(self.file.dynamic_symbols())
            .filter(|s| s.is_definition() && s.kind() == SymbolKind::Text && self.code(s.address()))
        {
            if self.pc(sym.address()) != sym.address() {
                spans.remove(&self.pc(sym.address()));
            }
            spans.insert(
                sym.address(),
                self.pc(sym.address()).saturating_add(sym.size()),
            );
        }
        let mut next = u64::MAX;
        for (&raw, end) in spans.iter_mut().rev() {
            let pc = self.pc(raw);
            if *end <= pc {
                *end = next;
            }
            *end = (*end).min(next).min(pc.saturating_add(65536));
            next = pc;
        }
        spans
    }

    pub(super) fn end(&self, bodies: &BTreeMap<u64, u64>, raw: u64) -> u64 {
        let pc = self.pc(raw);
        let section_end = self
            .file
            .sections()
            .filter(|s| s.kind() == SectionKind::Text)
            .find(|s| pc.checked_sub(s.address()).is_some_and(|n| n < s.size()))
            .map_or(pc, |s| s.address().saturating_add(s.size()));
        if let Some(&end) = bodies.get(&raw).or_else(|| bodies.get(&pc)) {
            return end.min(section_end);
        }
        let limit = pc.saturating_add(8192).min(section_end);
        bodies
            .range((std::ops::Bound::Excluded(raw), std::ops::Bound::Unbounded))
            .next()
            .map_or(limit, |(&a, _)| self.pc(a).min(limit))
    }

    pub(super) fn direct_target(
        &self,
        from: u64,
        insn: u64,
        target: u64,
        call: bool,
    ) -> Option<u64> {
        let raw = crate::analyzers::nativeflow::direct_entry(
            self.file.architecture() == Architecture::Arm,
            from,
            self.data(insn)?,
            target,
            call,
        )?;
        self.code(raw).then_some(raw)
    }

    /// Screen known function extents for a direct branch to a Binder PLT entry before lifting.
    /// GOT-indirect imports have no fixed branch target and require the bounded lifting fallback.
    pub(super) fn candidate(&self, entry: u64, end: u64) -> bool {
        if !self.got.is_empty() {
            return true;
        }
        let pc = self.pc(entry);
        let Some(data) = self.data(pc) else {
            return false;
        };
        let data = &data[..data.len().min((end - pc) as usize)];
        let is_target =
            |pc: u64, delta: i64| self.stubs.contains_key(&pc.wrapping_add_signed(delta));
        match self.file.architecture() {
            Architecture::X86_64 | Architecture::I386 => {
                data.windows(5).enumerate().any(|(i, b)| {
                    matches!(b[0], 0xe8 | 0xe9)
                        && is_target(
                            pc.wrapping_add(i as u64 + 5),
                            i32::from_le_bytes(b[1..].try_into().unwrap()) as i64,
                        )
                })
            }
            Architecture::Aarch64 => data.chunks_exact(4).enumerate().any(|(i, b)| {
                let w = u32::from_le_bytes(b.try_into().unwrap());
                w & 0x7c00_0000 == 0x1400_0000
                    && is_target(pc.wrapping_add(i as u64 * 4), ((w << 6) as i32 >> 4) as i64)
            }),
            Architecture::Arm if entry & 1 == 0 => {
                data.chunks_exact(4).enumerate().any(|(i, b)| {
                    let w = u32::from_le_bytes(b.try_into().unwrap());
                    w & 0x0e00_0000 == 0x0a00_0000
                        && is_target(
                            pc.wrapping_add(i as u64 * 4 + 8),
                            ((w << 8) as i32 >> 6) as i64,
                        )
                })
            }
            Architecture::Arm => true,
            _ => false,
        }
    }
}

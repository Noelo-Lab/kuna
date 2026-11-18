//! (kuna `kernelbug`) Read a Linux kernel image's `__bug_table`: one
//! [`KernelBugSite`] per `struct bug_entry`, so the flow renders each `WARN`/`BUG`
//! trap as the statement it is (`kuna_decomp::kuna_kernelbug`). Inert on every
//! image without that section.
//!
//! # The table
//!
//! `include/asm-generic/bug.h`:
//!
//! ```text
//!   struct bug_entry {
//!       BUG_REL(unsigned long, bug_addr);   // the trap instruction
//!   #ifdef HAVE_ARCH_BUG_FORMAT
//!       BUG_REL(const char *, format);      // x86-64 since 7.x
//!   #endif
//!   #ifdef CONFIG_DEBUG_BUGVERBOSE
//!       BUG_REL(const char *, file);
//!       unsigned short line;
//!   #endif
//!       unsigned short flags;               // bit 0: BUGFLAG_WARNING
//!   };
//! ```
//!
//! `BUG_REL` is a 32-bit self-relative displacement under
//! `CONFIG_GENERIC_BUG_RELATIVE_POINTERS` (x86-64 and AArch64 always) and an
//! absolute pointer otherwise, so the entry is 8, 12, 16 or 24 bytes depending on
//! the configuration and the kernel version. The layout is not declared anywhere
//! in the image; it is recovered from the section itself:
//!
//! * **A module (`ET_REL`)** keeps `.rela__bug_table`. The entries whose target
//!   is an executable section are the `bug_addr` fields; their spacing is the
//!   entry size, and the other relocations inside one entry are its pointer
//!   fields. The analyzer tier sees the module in the loaded image's address
//!   space (`kuna_relocrebase`), so the field values read back already applied.
//! * **A linked image (`vmlinux`)** is probed even when it keeps its
//!   relocations (an x86-64 `vmlinux` links with `--emit-relocs`, whose offsets
//!   are addresses, not section offsets). Each candidate entry
//!   size in turn is tried, smallest first: the one under which every `bug_addr`
//!   lands in executable bytes wins (under a wrong size the field read is a
//!   string displacement or a line/flags word, which lands outside the code),
//!   and a field is a pointer when every entry's value lands in a
//!   non-executable section. A `bug_addr` need not be a trap: a 7.x kernel's
//!   `WARN()` with arguments is a `call __WARN_trap` the table also names.
//!
//! The one pointer field that is ambiguous — a single string pointer is either
//! `file` (verbose, no format) or `format` (format, not verbose) — is told apart
//! by what it points at: a source path for `file`.
//!
//! Before Linux 5.19 (`69505e3d9a39`, "bug: Use normal relative pointers") a
//! self-relative `file` counted from the start of the entry, not from the field,
//! so read the 5.19 way it lands four bytes into the path. The table is read
//! the old way when that reading puts more `file` fields on the first byte of a
//! string than the new one does.

use object::read::{Object, ObjectSection, ObjectSymbol};
use object::{ObjectKind, RelocationKind, RelocationTarget, SectionKind, SymbolSection};

use kuna_decomp::kuna_kernelbug::KernelBugSite;

use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, Phase};

/// The section the kernel's `BUG()`/`WARN()` macros emit their entries into.
pub const BUG_TABLE_SECTION: &str = "__bug_table";

/// `BUGFLAG_WARNING`: the handler resumes after the trap.
pub const BUGFLAG_WARNING: u16 = 1;

/// The entry sizes a relocation-free table is probed with, smallest first.
const PROBE_STRIDES: [usize; 3] = [8, 12, 16];

/// The longest source path read out of the image.
const MAX_PATH: usize = 512;

/// The entries, spread through the table, the pre-5.19 `file` reading is decided on.
const VOTE_ENTRIES: usize = 64;

/// A recovered `struct bug_entry` layout.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Layout {
    /// The entry size.
    pub stride: usize,
    /// The `bug_addr` field's width: 4 self-relative, 8 absolute.
    pub addr_width: u8,
    /// Pointer fields are `(offset, width)`; width 4 is self-relative, 8 absolute.
    pub pointers: Vec<(usize, u8)>,
    /// Which pointer field is `file`, when the table carries one.
    pub file: Option<usize>,
    /// Where `line` lives, when the table carries one.
    pub line: Option<usize>,
    /// Where `flags` lives.
    pub flags: usize,
    /// A self-relative `file` counts from the entry's start (kernels before 5.19).
    pub entry_relative: bool,
}

/// Read-only view of the image the table is decoded against.
struct Image<'a, 'b> {
    file: &'a object::File<'b>,
    /// The executable `[lo, hi)` extents, sorted and disjoint.
    exec: Vec<(u64, u64)>,
    /// Every section placed at a nonzero address, as `(lo, hi, index)` sorted by `lo`.
    sections: Vec<(u64, u64, object::SectionIndex)>,
}

impl<'a, 'b> Image<'a, 'b> {
    fn new(file: &'a object::File<'b>, mut exec: Vec<(u64, u64)>) -> Self {
        exec.sort_unstable();
        let mut merged: Vec<(u64, u64)> = Vec::with_capacity(exec.len());
        for (lo, hi) in exec {
            match merged.last_mut() {
                Some(last) if lo <= last.1 => last.1 = last.1.max(hi),
                _ => merged.push((lo, hi)),
            }
        }
        let mut sections: Vec<_> = file
            .sections()
            .filter(|s| s.address() != 0 && s.size() != 0)
            .map(|s| (s.address(), s.address().saturating_add(s.size()), s.index()))
            .collect();
        sections.sort_unstable_by_key(|&(lo, hi, _)| (lo, hi));
        Image { file, exec: merged, sections }
    }

    fn is_executable(&self, addr: u64) -> bool {
        let i = self.exec.partition_point(|&(lo, _)| lo <= addr);
        i > 0 && addr < self.exec[i - 1].1
    }

    /// The section holding `addr`, with `addr`'s offset into its data.
    fn section_at(&self, addr: u64) -> Option<(object::Section<'a, 'b>, usize)> {
        let i = self.sections.partition_point(|&(lo, _, _)| lo <= addr).checked_sub(1)?;
        let (lo, hi, index) = self.sections[i];
        if addr >= hi {
            return None;
        }
        Some((self.file.section_by_index(index).ok()?, (addr - lo) as usize))
    }

    /// Is `addr` in a mapped, non-executable section — where a string lives?
    fn is_data(&self, addr: u64) -> bool {
        !self.is_executable(addr)
            && self.section_at(addr).is_some_and(|(s, _)| s.kind() != SectionKind::Text && s.kind() != SectionKind::Metadata)
    }

    /// The NUL-terminated string at `addr`, bounded by [`MAX_PATH`].
    fn string_at(&self, addr: u64) -> Option<String> {
        let (section, off) = self.section_at(addr)?;
        let data = section.data().ok()?;
        let tail = data.get(off..)?;
        let end = tail.iter().take(MAX_PATH).position(|&b| b == 0)?;
        std::str::from_utf8(&tail[..end]).ok().map(str::to_owned)
    }

    /// Is `addr` the first byte of a string: data at its section's start or
    /// right behind a NUL, with a NUL within [`MAX_PATH`]?
    fn string_start(&self, addr: u64) -> bool {
        self.is_data(addr)
            && self.section_at(addr).is_some_and(|(section, off)| {
                let data = section.data().unwrap_or_default();
                (off == 0 || data.get(off - 1) == Some(&0))
                    && data.get(off..).is_some_and(|t| t.iter().take(MAX_PATH).any(|&b| b == 0))
            })
    }
}

fn read_u16(data: &[u8], off: usize, big: bool) -> Option<u16> {
    let b = data.get(off..off + 2)?;
    Some(if big { u16::from_be_bytes([b[0], b[1]]) } else { u16::from_le_bytes([b[0], b[1]]) })
}

fn read_u32(data: &[u8], off: usize, big: bool) -> Option<u32> {
    let b: [u8; 4] = data.get(off..off + 4)?.try_into().ok()?;
    Some(if big { u32::from_be_bytes(b) } else { u32::from_le_bytes(b) })
}

fn read_u64(data: &[u8], off: usize, big: bool) -> Option<u64> {
    let b: [u8; 8] = data.get(off..off + 8)?.try_into().ok()?;
    Some(if big { u64::from_be_bytes(b) } else { u64::from_le_bytes(b) })
}

/// The address a pointer field holds: `base + disp` for a self-relative one,
/// the value itself for an absolute one.
fn pointer(data: &[u8], base: u64, off: usize, width: u8, big: bool) -> Option<u64> {
    match width {
        4 => Some(base.wrapping_add(off as u64).wrapping_add(read_u32(data, off, big)? as i32 as i64 as u64)),
        8 => read_u64(data, off, big),
        _ => None,
    }
}

/// Is `s` a source file path rather than a `WARN` format string?
fn looks_like_source_path(s: &str) -> bool {
    s.contains('/') || [".c", ".h", ".S", ".rs", ".cc", ".cpp"].iter().any(|ext| s.ends_with(ext))
}

/// Does the table's self-relative string field at `off` count from the
/// entry's start? Decided over [`VOTE_ENTRIES`] entries spread evenly through
/// the table by which reading lands more of them on the first byte of a string.
fn entry_relative(image: &Image, data: &[u8], vma: u64, big: bool, stride: usize, (off, width): (usize, u8)) -> bool {
    if width != 4 {
        return false;
    }
    let step = (data.len() / stride).div_ceil(VOTE_ENTRIES).max(1) * stride;
    let (mut field, mut entry) = (0usize, 0usize);
    for e in (0..data.len()).step_by(step).take(VOTE_ENTRIES) {
        if let Some(a) = pointer(data, vma, e + off, width, big) {
            field += usize::from(image.string_start(a));
            entry += usize::from(image.string_start(a.wrapping_sub(off as u64)));
        }
    }
    entry > field
}

/// The address the string field at `off` of the entry at `e` names.
fn string_pointer(layout: &Layout, data: &[u8], vma: u64, e: usize, (off, width): (usize, u8), big: bool) -> Option<u64> {
    let a = pointer(data, vma, e + off, width, big)?;
    Some(if layout.entry_relative && width == 4 { a.wrapping_sub(off as u64) } else { a })
}

/// Complete a layout whose entry size and pointer fields are known: which
/// pointer is `file`, and where `line` and `flags` sit behind the pointers.
/// `None` unless `flags` ends the entry, before at most its alignment padding.
fn finish_layout(image: &Image, data: &[u8], vma: u64, big: bool, stride: usize, addr_width: u8, pointers: Vec<(usize, u8)>) -> Option<Layout> {
    let after = pointers.iter().map(|&(off, w)| off + w as usize).max().unwrap_or(addr_width as usize);
    let entry_relative = pointers.last().is_some_and(|&f| entry_relative(image, data, vma, big, stride, f));
    let mut layout = Layout { stride, addr_width, pointers, file: None, line: None, flags: after, entry_relative };
    layout.file = match layout.pointers.len() {
        0 => None,
        1 => string_pointer(&layout, data, vma, 0, layout.pointers[0], big)
            .and_then(|a| image.string_at(a))
            .is_some_and(|s| looks_like_source_path(&s))
            .then_some(0),
        n => Some(n - 1),
    };
    if layout.file.is_some() {
        (layout.line, layout.flags) = (Some(after), after + 2);
    }
    let align = if layout.addr_width == 8 || layout.pointers.iter().any(|&(_, w)| w == 8) { 8 } else { 4 };
    let end = layout.flags + 2;
    (end == stride || end.next_multiple_of(align) == stride).then_some(layout)
}

/// The layout a module's `.rela__bug_table` describes, or `None` when the
/// relocations do not spell out one.
fn layout_from_relocations(image: &Image, section: &object::Section, data: &[u8], big: bool) -> Option<Layout> {
    let file = image.file;
    let mut code: Vec<(usize, u8)> = Vec::new();
    let mut other: Vec<(usize, u8)> = Vec::new();
    for (off, reloc) in section.relocations() {
        let RelocationTarget::Symbol(index) = reloc.target() else { return None };
        let sym = file.symbol_by_index(index).ok()?;
        let width = match (reloc.kind(), reloc.size()) {
            (RelocationKind::Relative, 32) => 4,
            (RelocationKind::Absolute, 64) => 8,
            _ => return None,
        };
        let SymbolSection::Section(sec) = sym.section() else { return None };
        let is_code = file.section_by_index(sec).ok()?.kind() == SectionKind::Text;
        if is_code { code.push((off as usize, width)) } else { other.push((off as usize, width)) }
    }
    code.sort_unstable();
    let n = code.len();
    let size = data.len();
    if n == 0 || size % n != 0 {
        return None;
    }
    let stride = size / n;
    let addr_width = code[0].1;
    if code.iter().enumerate().any(|(i, &(off, w))| off != i * stride || w != addr_width) {
        return None;
    }
    let mut pointers: Vec<(usize, u8)> = other.iter().map(|&(off, w)| (off % stride, w)).collect();
    pointers.sort_unstable();
    pointers.dedup();
    if pointers.iter().any(|&(off, w)| off < addr_width as usize || off + w as usize > stride) {
        return None;
    }
    finish_layout(image, data, section.address(), big, stride, addr_width, pointers)
}

/// The layout under which every entry of a relocation-free table names code,
/// probing [`PROBE_STRIDES`] in order.
fn layout_by_probe(image: &Image, data: &[u8], vma: u64, big: bool) -> Option<Layout> {
    PROBE_STRIDES.iter().copied().find_map(|stride| {
        if data.len() % stride != 0 || data.is_empty() {
            return None;
        }
        let entries = (0..data.len()).step_by(stride);
        let in_code = entries.clone().all(|e| pointer(data, vma, e, 4, big).is_some_and(|a| image.is_executable(a)));
        if !in_code {
            return None;
        }
        let mut pointers = Vec::new();
        let mut off = 4;
        while off + 4 + 2 <= stride {
            let all_data = entries.clone().all(|e| pointer(data, vma, e + off, 4, big).is_some_and(|a| image.is_data(a)));
            if !all_data {
                break;
            }
            pointers.push((off, 4u8));
            off += 4;
        }
        finish_layout(image, data, vma, big, stride, 4, pointers)
    })
}

/// Decode every entry of the table at `vma` under `layout`.
pub fn decode(layout: &Layout, data: &[u8], vma: u64, big: bool) -> Vec<KernelBugSite> {
    (0..data.len())
        .step_by(layout.stride)
        .filter_map(|e| {
            let addr = pointer(data, vma, e, layout.addr_width, big)?;
            let flags = read_u16(data, e + layout.flags, big)?;
            let file = layout.file.and_then(|i| layout.pointers.get(i)).and_then(|&f| string_pointer(layout, data, vma, e, f, big));
            let line = layout.line.and_then(|off| read_u16(data, e + off, big)).map(u32::from);
            Some(KernelBugSite { addr, warning: flags & BUGFLAG_WARNING != 0, file, line })
        })
        .collect()
}

/// The sites of `file`'s `__bug_table` whose trap lies in one of the executable
/// `[lo, hi)` extents `exec`, sorted by trap address; empty without a table.
pub fn scan(file: &object::File, exec: Vec<(u64, u64)>) -> Vec<KernelBugSite> {
    let Some(section) = file.section_by_name(BUG_TABLE_SECTION) else { return Vec::new() };
    let Ok(data) = section.data() else { return Vec::new() };
    if data.is_empty() {
        return Vec::new();
    }
    let image = Image::new(file, exec);
    let big = !file.is_little_endian();
    let vma = section.address();
    let layout = if file.kind() == ObjectKind::Relocatable && section.relocations().next().is_some() {
        layout_from_relocations(&image, &section, data, big)
    } else {
        layout_by_probe(&image, data, vma, big)
    };
    let Some(layout) = layout else { return Vec::new() };
    let mut sites: Vec<KernelBugSite> = decode(&layout, data, vma, big)
        .into_iter()
        .filter(|s| image.is_executable(s.addr))
        .collect();
    sites.sort_by_key(|s| s.addr);
    sites.dedup_by_key(|s| s.addr);
    sites
}

/// The `__bug_table` reader. Always on: a trap the table names has one meaning.
#[derive(Clone, Copy, Default)]
pub struct KernelBugPass;

impl AnalysisPass for KernelBugPass {
    fn phase(&self) -> Phase {
        Phase::P1
    }

    fn id(&self) -> &'static str {
        "kernelbug"
    }

    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        let mut out = AnalysisOutput::default();
        let exec = ctx
            .image
            .map(|i| i.executable_ranges().into_iter().map(|(vma, size)| (vma, vma.saturating_add(size))).collect())
            .unwrap_or_default();
        out.kernel_bugs = scan(ctx.file, exec);
        out
    }
}

#[cfg(test)]
mod tests;

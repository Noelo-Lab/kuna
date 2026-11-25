//! Go pclntab recovery through `runtime.firstmoduledata` when the `pcHeader`
//! magic is not one of the four known values.
//!
//! A packer or obfuscator can overwrite the magic in the file (putting it back,
//! or skipping the runtime's start-up check, when the program runs) and leave
//! the table itself intact. The `moduledata` record still describes that table
//! completely: its slices give
//! the name table, the functab and the `_func` base, `len(ftab)` gives the
//! function count, and `minpc`/`maxpc`/`text`/`etext` give the PC bounds. The
//! record is recognised by that shape alone, and the table is read from it
//! without consulting the header.
//!
//! Record layouts (`runtime/symtab.go`, words of pointer size):
//! - go1.16+: `pcHeader`, `funcnametab[3]`, `cutab[3]`, `filetab[3]`,
//!   `pctab[3]`, `pclntable[3]`, `ftab[3]`, `findfunctab`, `minpc`, `maxpc`,
//!   `text`, `etext`.
//! - go1.5 .. go1.15: `pclntable[3]`, `ftab[3]`, `filetab[3]`, `findfunctab`,
//!   `minpc`, `maxpc`, `text`, `etext`.
//!
//! The functab entry shape (go1.16 absolute pointers, go1.18+ 32-bit offsets
//! from `text`) is the one whose first entry is `minpc` and whose closing entry
//! is `maxpc`, the identity the runtime's own `moduledataverify1` enforces.

use object::read::{Object, ObjectSection};

use super::{functab_end, rd_ptr, rd_u32, End, GoVer, ImageBytes, PcHeader};

/// Most functions a table may claim, as in the header path.
const MAX_NFUNC: u64 = 2_000_000;

/// The table the image's `moduledata` describes, with the address its
/// `pcHeader` sits at. `None` when no section outside the code holds a record
/// whose slices, bounds and functab agree with each other. Section names and
/// kinds are not consulted: a packer renames them, and a Mach-O `__go_module`
/// has no kind of its own.
pub(super) fn locate(file: &object::File, img: &ImageBytes) -> Option<(u64, PcHeader)> {
    let end = if file.is_little_endian() { End::Little } else { End::Big };
    let ptr_size = if file.is_64() { 8 } else { 4 };
    file.sections()
        .filter(|s| s.kind() != object::SectionKind::Text)
        .find_map(|s| scan(img, s.address(), s.data().ok()?, ptr_size, end))
}

/// The first pointer-aligned record in `data` (loaded at `base`) that
/// validates as a `moduledata` head.
pub(super) fn scan(img: &ImageBytes, base: u64, data: &[u8], ptr_size: u8, end: End) -> Option<(u64, PcHeader)> {
    let ps = ptr_size as usize;
    let lead = (ps - (base as usize % ps)) % ps;
    (lead..data.len()).step_by(ps).find_map(|off| {
        let rec = Record { words: &data[off..], ptr_size, end };
        rec.go116(img).or_else(|| rec.go12(img))
    })
}

/// A candidate record: the section bytes from its first word on.
struct Record<'d> {
    words: &'d [u8],
    ptr_size: u8,
    end: End,
}

impl Record<'_> {
    fn word(&self, i: usize) -> Option<u64> {
        let ps = self.ptr_size as usize;
        Some(rd_ptr(self.words.get(i * ps..(i + 1) * ps)?, self.ptr_size, self.end))
    }

    /// `true` when each slice starting at one of `at` has `len <= cap`.
    fn slices_ok(&self, at: &[usize]) -> bool {
        at.iter().all(|&i| matches!((self.word(i + 1), self.word(i + 2)), (Some(len), Some(cap)) if len <= cap))
    }

    /// The go1.16+ record: `ftab` opens `pclntable`, and the header, the name
    /// table and the functab sit in that order.
    fn go116(&self, img: &ImageBytes) -> Option<(u64, PcHeader)> {
        let (pclntable, ftab) = (self.word(13)?, self.word(16)?);
        if pclntable == 0 || pclntable != ftab {
            return None;
        }
        let (header, funcname) = (self.word(0)?, self.word(1)?);
        if header == 0 || header >= funcname || funcname > pclntable || !self.slices_ok(&[1, 4, 7, 10, 13, 16]) {
            return None;
        }
        let bounds = self.bounds(17, 20)?;
        img.read(funcname, 1)?;
        let table = |ver| PcHeader {
            ver,
            end: self.end,
            ptr_size: self.ptr_size,
            text_start: bounds.text,
            funcname_vma: funcname,
            functab_vma: ftab,
            nfunc: bounds.nfunc,
            funcdata_base: pclntable,
        };
        [GoVer::Go118, GoVer::Go116]
            .into_iter()
            .map(table)
            .find(|h| functab_spans(img, h, bounds.minpc, bounds.maxpc))
            .map(|h| (header, h))
    }

    /// The go1.5 .. go1.15 record: the functab follows the 8-byte header and
    /// the function count at the head of `pclntable`.
    fn go12(&self, img: &ImageBytes) -> Option<(u64, PcHeader)> {
        let (pclntable, ftab) = (self.word(0)?, self.word(3)?);
        if pclntable == 0 || pclntable.checked_add(8 + self.ptr_size as u64)? != ftab || !self.slices_ok(&[0, 3, 6]) {
            return None;
        }
        let bounds = self.bounds(4, 10)?;
        let h = PcHeader {
            ver: GoVer::Go12,
            end: self.end,
            ptr_size: self.ptr_size,
            text_start: 0,
            funcname_vma: pclntable,
            functab_vma: ftab,
            nfunc: bounds.nfunc,
            funcdata_base: pclntable,
        };
        functab_spans(img, &h, bounds.minpc, bounds.maxpc).then_some((pclntable, h))
    }

    /// The function count (`len(ftab) - 1`, word `ftab_len`) and the four PC
    /// bounds from word `minpc` on, which must nest.
    fn bounds(&self, ftab_len: usize, minpc: usize) -> Option<Bounds> {
        let nfunc = self.word(ftab_len)?.checked_sub(1)?;
        let (minpc, maxpc, text, etext) =
            (self.word(minpc)?, self.word(minpc + 1)?, self.word(minpc + 2)?, self.word(minpc + 3)?);
        ((1..=MAX_NFUNC).contains(&nfunc) && text != 0 && text <= minpc && minpc < maxpc && maxpc <= etext)
            .then_some(Bounds { nfunc, minpc, maxpc, text })
    }
}

struct Bounds {
    nfunc: u64,
    minpc: u64,
    maxpc: u64,
    text: u64,
}

/// `true` when the functab `h` describes opens at `minpc` and closes at
/// `maxpc`.
fn functab_spans(img: &ImageBytes, h: &PcHeader, minpc: u64, maxpc: u64) -> bool {
    let first = if h.ver.is_118plus() {
        img.read(h.functab_vma, 4).map(|b| h.text_start.wrapping_add(rd_u32(b, h.end) as u64))
    } else {
        img.read(h.functab_vma, h.ptr_size as usize).map(|b| rd_ptr(b, h.ptr_size, h.end))
    };
    first == Some(minpc) && functab_end(img, h) == Some(maxpc)
}

//! (kuna) The C++ landing pads of an image, read from its exception tables
//! for `option bejoin callers` ([`super`]).
//!
//! The unwinder enters a landing pad with the exception's selector in the
//! second return register, and gcc lays a landing pad out right after a call it
//! knows does not return (`f(); __builtin_unreachable();` in a `try`), so a
//! walk after that call must not read the pad as the call's continuation. The
//! pads come from the LSDA of each `.eh_frame` FDE whose CIE carries an `L`
//! augmentation (PowerPC, MIPS, SPARC) and of each `.ARM.exidx` entry whose
//! `.ARM.extab` data names a personality routine (ARM EHABI). An image whose
//! tables say it has landing pads, but that this reader cannot place, is
//! `blind`; a function whose LSDA does not decode is `unknown`.

use std::collections::BTreeMap;

const OMIT: u8 = 0xff;
const PAGE: u64 = 256;

/// What the exception tables of one image say.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub(super) struct Landing {
    /// The landing pads, sorted.
    pub(super) pads: Vec<u64>,
    /// The entries of the functions whose exception data does not decode,
    /// sorted.
    pub(super) unknown: Vec<u64>,
    /// The image has landing pads this reader cannot place.
    pub(super) blind: bool,
}

impl Landing {
    /// Does a landing pad start in `[lo, hi)`?
    pub(super) fn pad_in(&self, lo: u64, hi: u64) -> bool {
        any_in(&self.pads, lo, hi)
    }

    /// Does a function whose exception data does not decode start in
    /// `[lo, hi)`?
    pub(super) fn unknown_in(&self, lo: u64, hi: u64) -> bool {
        any_in(&self.unknown, lo, hi)
    }
}

fn any_in(sorted: &[u64], lo: u64, hi: u64) -> bool {
    sorted.get(sorted.partition_point(|&a| a < lo)).is_some_and(|&a| a < hi)
}

/// The image's big-endian bytes, read through `read` a page at a time (a
/// byte at a time on a page that does not read whole).
struct Mem<'a> {
    read: &'a mut dyn FnMut(u64, &mut [u8]) -> bool,
    pages: BTreeMap<u64, Vec<Option<u8>>>,
}

impl Mem<'_> {
    fn byte(&mut self, a: u64) -> Option<u8> {
        let base = a & !(PAGE - 1);
        if !self.pages.contains_key(&base) {
            let mut buf = vec![0u8; PAGE as usize];
            let page = if (self.read)(base, &mut buf) {
                buf.into_iter().map(Some).collect()
            } else {
                (base..base + PAGE)
                    .map(|b| {
                        let mut one = [0u8];
                        (self.read)(b, &mut one).then_some(one[0])
                    })
                    .collect()
            };
            self.pages.insert(base, page);
        }
        self.pages.get(&base)?[(a - base) as usize]
    }

    fn word(&mut self, a: u64, n: u64) -> Option<u64> {
        (0..n).try_fold(0u64, |v, i| Some(v << 8 | u64::from(self.byte(a.wrapping_add(i))?)))
    }

    fn uleb(&mut self, p: &mut u64) -> Option<u64> {
        let mut v = 0u64;
        for shift in (0..64).step_by(7) {
            let b = self.byte(*p)?;
            *p += 1;
            v |= u64::from(b & 0x7f) << shift;
            if b & 0x80 == 0 {
                return Some(v);
            }
        }
        None
    }

    fn sleb(&mut self, p: &mut u64) -> Option<i64> {
        let mut v = 0i64;
        let mut shift = 0;
        loop {
            let b = self.byte(*p)?;
            *p += 1;
            v |= i64::from(b & 0x7f) << shift;
            shift += 7;
            if b & 0x80 == 0 {
                return Some(if shift < 64 && b & 0x40 != 0 { v | -1i64 << shift } else { v });
            }
            if shift >= 64 {
                return None;
            }
        }
    }

    /// A `DW_EH_PE` value of format `enc & 0x0f` at `p`, sign-extended.
    fn value(&mut self, enc: u8, p: &mut u64) -> Option<u64> {
        let (n, signed) = match enc & 0x0f {
            0x00 | 0x03 => (4, false),
            0x02 => (2, false),
            0x04 | 0x0c => (8, false),
            0x0a => (2, true),
            0x0b => (4, true),
            0x01 => return self.uleb(p),
            0x09 => return self.sleb(p).map(|v| v as u64),
            _ => return None,
        };
        let v = self.word(*p, n)?;
        *p += n;
        Some(if signed { ((v << (64 - 8 * n)) as i64 >> (64 - 8 * n)) as u64 } else { v })
    }

    /// A `DW_EH_PE` pointer at `p` (absolute or PC-relative): its raw value
    /// and the 32-bit address it names.
    fn pointer(&mut self, enc: u8, p: &mut u64) -> Option<(u64, u64)> {
        let at = *p;
        let raw = self.value(enc, p)?;
        let v = match enc & 0xf0 {
            0x00 => raw,
            0x10 => at.wrapping_add(raw),
            _ => return None,
        };
        Some((raw, v & 0xffff_ffff))
    }
}

/// A CIE's FDE pointer encoding and, when it has an `L` augmentation, its
/// LSDA pointer encoding.
#[derive(Clone, Copy)]
struct Cie {
    fde: u8,
    lsda: Option<u8>,
}

fn cie(mem: &mut Mem, at: u64) -> Option<Cie> {
    if mem.word(at + 4, 4)? != 0 {
        return None;
    }
    let mut p = at + 8;
    let version = mem.byte(p)?;
    p += 1;
    let mut aug = Vec::new();
    loop {
        let b = mem.byte(p)?;
        p += 1;
        if b == 0 {
            break;
        }
        aug.push(b);
        if aug.len() > 16 {
            return None;
        }
    }
    if version >= 4 {
        p += 2;
    }
    mem.uleb(&mut p)?;
    mem.sleb(&mut p)?;
    if version == 1 {
        p += 1;
    } else {
        mem.uleb(&mut p)?;
    }
    let mut out = Cie { fde: 0, lsda: None };
    let Some((&b'z', rest)) = aug.split_first() else {
        return aug.is_empty().then_some(out);
    };
    mem.uleb(&mut p)?;
    for &c in rest {
        match c {
            b'R' => {
                out.fde = mem.byte(p)?;
                p += 1;
            }
            b'L' => {
                out.lsda = Some(mem.byte(p)?);
                p += 1;
            }
            b'P' => {
                let enc = mem.byte(p)?;
                p += 1;
                mem.value(enc, &mut p)?;
            }
            b'S' | b'B' => {}
            _ => return None,
        }
    }
    Some(out)
}

/// The landing pads an LSDA at `at` names, for the function at `func`; `None`
/// when it does not decode or names a pad outside the code.
fn lsda(mem: &mut Mem, at: u64, func: u64, in_code: &dyn Fn(u64) -> bool) -> Option<Vec<u64>> {
    let mut p = at;
    let lp_enc = mem.byte(p)?;
    p += 1;
    let base = if lp_enc == OMIT { func } else { mem.pointer(lp_enc, &mut p)?.1 };
    let tt_enc = mem.byte(p)?;
    p += 1;
    if tt_enc != OMIT {
        mem.uleb(&mut p)?;
    }
    let cs_enc = mem.byte(p)?;
    p += 1;
    if cs_enc & 0xf0 != 0 {
        return None;
    }
    let len = mem.uleb(&mut p)?;
    let end = p.checked_add(len)?;
    let mut pads = Vec::new();
    while p < end {
        mem.value(cs_enc, &mut p)?;
        mem.value(cs_enc, &mut p)?;
        let lp = mem.value(cs_enc, &mut p)?;
        mem.uleb(&mut p)?;
        if lp != 0 {
            let pad = base.wrapping_add(lp) & 0xffff_ffff;
            if !in_code(pad) {
                return None;
            }
            pads.push(pad);
        }
    }
    (p == end).then_some(pads)
}

/// Read `.eh_frame` at `[start, end)`: `false` when its records do not parse.
fn eh_frame(mem: &mut Mem, start: u64, end: u64, in_code: &dyn Fn(u64) -> bool, out: &mut Landing, found: &mut bool) -> bool {
    let mut cies: BTreeMap<u64, Option<Cie>> = BTreeMap::new();
    let mut o = start;
    while o + 4 <= end {
        let Some(len) = mem.word(o, 4) else { return false };
        if len == 0 {
            break;
        }
        let Some(next) = o.checked_add(4 + len).filter(|&n| n <= end && len >= 4 && len != 0xffff_ffff) else { return false };
        let Some(id) = mem.word(o + 4, 4) else { return false };
        if id != 0 {
            let Some(at) = (o + 4).checked_sub(id) else { return false };
            let Some(c) = *cies.entry(at).or_insert_with(|| cie(mem, at)) else { return false };
            if let Some(lenc) = c.lsda {
                let mut p = o + 8;
                let Some((_, func)) = mem.pointer(c.fde, &mut p) else { return false };
                if mem.value(c.fde, &mut p).is_none() || mem.uleb(&mut p).is_none() {
                    return false;
                }
                let Some((raw, at)) = mem.pointer(lenc, &mut p) else { return false };
                if raw != 0 {
                    *found = true;
                    if !in_code(func) {
                        return false;
                    }
                    match lsda(mem, at, func, in_code) {
                        Some(pads) => out.pads.extend(pads),
                        None => out.unknown.push(func),
                    }
                }
            }
        }
        o = next;
    }
    true
}

/// A `prel31` field at `at`: the address it names.
fn prel31(at: u64, w: u64) -> u64 {
    let off = ((w << 33) as i64 >> 33) as u64;
    at.wrapping_add(off) & 0xffff_ffff
}

/// Read `.ARM.exidx` at `[start, end)`: `false` when its entries do not parse.
fn exidx(mem: &mut Mem, start: u64, end: u64, in_code: &dyn Fn(u64) -> bool, out: &mut Landing, found: &mut bool) -> bool {
    let mut e = start;
    while e + 8 <= end {
        let (Some(w0), Some(w1)) = (mem.word(e, 4), mem.word(e + 4, 4)) else { return false };
        if w0 & 0x8000_0000 != 0 {
            return false;
        }
        let func = prel31(e, w0) & !1;
        e += 8;
        if w1 == 1 {
            continue;
        }
        if w1 & 0x8000_0000 != 0 {
            if (w1 >> 24) & 0x7f > 2 {
                out.unknown.push(func);
            }
            continue;
        }
        let x = prel31(e - 4, w1);
        let Some(h) = mem.word(x, 4) else { return false };
        if h & 0x8000_0000 != 0 {
            let compact = match (h >> 24) & 0x7f {
                0 => true,
                1 | 2 => mem.word(x + 4 * (1 + ((h >> 16) & 0xff)), 4) == Some(0),
                _ => false,
            };
            if !compact {
                out.unknown.push(func);
            }
            continue;
        }
        *found = true;
        if !in_code(func) {
            return false;
        }
        let Some(w) = mem.word(x + 4, 4) else { return false };
        match lsda(mem, x + 4 * (2 + ((w >> 24) & 0xff)), func, in_code) {
            Some(pads) => out.pads.extend(pads),
            None => out.unknown.push(func),
        }
    }
    true
}

/// Read the landing pads of the image whose exception-table sections are
/// `sections` (`(name, address, size)`; `None` when the front end gives no
/// section table), through `read` (big-endian bytes at an address). `named`
/// says the image names a C++ personality routine or unwinder entry, and
/// `sjlj` that it unwinds by `setjmp` (its pads come from a dispatch the
/// tables do not name). `in_code` says an address lies in the code.
pub(super) fn read(
    sections: Option<&[(String, u64, u64)]>,
    named: bool,
    sjlj: bool,
    in_code: &dyn Fn(u64) -> bool,
    read: &mut dyn FnMut(u64, &mut [u8]) -> bool,
) -> Landing {
    let mut out = Landing::default();
    let Some(sections) = sections else {
        out.blind = named || sjlj;
        return out;
    };
    let mut mem = Mem { read, pages: BTreeMap::new() };
    let mut found = false;
    let mut ok = true;
    for (name, start, size) in sections {
        let end = start.saturating_add(*size);
        if name == ".eh_frame" {
            ok &= eh_frame(&mut mem, *start, end, in_code, &mut out, &mut found);
        } else if name.starts_with(".ARM.exidx") {
            ok &= exidx(&mut mem, *start, end, in_code, &mut out, &mut found);
        }
    }
    let present = found
        || named
        || sjlj
        || sections.iter().any(|(n, _, _)| n.starts_with(".gcc_except_table") || n.starts_with(".ARM.extab"));
    out.blind = present && (!ok || sjlj);
    out.pads.sort_unstable();
    out.pads.dedup();
    out.unknown.sort_unstable();
    out.unknown.dedup();
    out
}

#[cfg(test)]
#[path = "landing_tests.rs"]
mod tests;

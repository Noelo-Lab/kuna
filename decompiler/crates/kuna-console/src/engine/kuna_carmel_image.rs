//! Frame canonical Carmel CODE before SLEIGH can see any issue or delay slot.

use std::rc::Rc;

use kuna_analysis::pass::ContextPaint;
use kuna_base::{address::Address, error::{KunaError, KunaResult}, space::AddrSpace, xml::Element};
use kuna_sleigh::loadimage::LoadImage;

pub(super) const TARGET: &str = "Carmel:BE:24:default:default";
const SLOT: u64 = 3;
const FETCH: u64 = 2 * SLOT;
const CODE_END: u64 = (1 << 24) * SLOT;
const TEMPLATE: u64 = 12;
const CLIW_BANK_END: u64 = (1 << 10) * TEMPLATE;

const ISSUE: &str = "carmel_issue";
const NEXT: &str = "carmel_next_issue_size";

const CLIW_FIELDS: [&str; 4] = [
    "carmel_cliw_known", "carmel_cliw_95_64", "carmel_cliw_63_32", "carmel_cliw_31_0",
];
type Fetch = [u8; FETCH as usize];

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
enum Issue {
    Parallel = 1,
    SeqLeft = 2,
    SeqRight = 3,
    WideUnmarked = 4,
    WideMarked = 5,
}

impl Issue {
    fn left(fetch: &Fetch) -> Self {
        match (fetch[0] & 0x80 != 0, fetch[3] & 0x80 != 0) {
            (false, false) => Self::Parallel,
            (true, false) => Self::SeqLeft,
            (false, true) => Self::WideUnmarked,
            (true, true) => Self::WideMarked,
        }
    }

    fn size(self) -> u32 {
        match self {
            Self::SeqLeft | Self::SeqRight => SLOT as u32,
            _ => FETCH as u32,
        }
    }
}

#[derive(Default)]
pub(super) struct Framing {
    issues: Vec<ContextPaint>,
    next_sizes: Vec<ContextPaint>,
}

fn paint(paints: &mut Vec<ContextPaint>, var: &'static str, addr: u64, value: u32) {
    let end = Some(addr + SLOT);
    if let Some(last) = paints.last_mut().filter(|p| p.var == var && p.end == Some(addr) && p.value == value) {
        last.end = end;
    } else {
        paints.push(ContextPaint { addr, end, var, value });
    }
}

impl Framing {
    pub(super) fn require_entries(&self, entries: impl IntoIterator<Item = u64>) -> KunaResult<()> {
        for addr in entries {
            let next = self.issues.partition_point(|paint| paint.addr <= addr);
            if addr % SLOT != 0 || next == 0 || addr >= self.issues[next - 1].end.unwrap() {
                return Err(KunaError::lowlevel(format!(
                    "Carmel entry 0x{:x} is not a mapped issue start", addr / SLOT,
                )));
            }
        }
        Ok(())
    }

    pub(super) fn into_paints(mut self) -> Vec<ContextPaint> {
        self.issues.append(&mut self.next_sizes);
        self.issues
    }
}

pub(super) fn validate_raw(base: u64, len: u64, end: u64) -> KunaResult<()> {
    if base % FETCH != 0 || len % FETCH != 0 || end > CODE_END {
        return Err(KunaError::lowlevel(
            "Carmel raw CODE requires an even word base, complete six-byte fetches, and a 24-bit word range",
        ));
    }
    Ok(())
}

/// Reject chunks that the permissive XML loader would silently overlap or pad.
/// Only an explicitly normalized single CLIW bank permits the high-bit alias.
pub(super) fn check_xml_chunks(root: &Element) -> KunaResult<bool> {
    let mut code = Vec::new();
    let mut cliw = Vec::new();
    for chunk in root.get_children() {
        if chunk.get_name() != "bytechunk" {
            continue;
        }
        let (ranges, limit) = match chunk.get_attribute_value("space")? {
            b"code" => (&mut code, CODE_END),
            b"cliw" => (&mut cliw, CODE_END),
            _ => continue,
        };
        let offset = std::str::from_utf8(chunk.get_attribute_value("offset")?)
            .map_err(|_| KunaError::lowlevel("Invalid Carmel XML chunk offset"))?
            .trim();
        let start = if let Some(hex) = offset.strip_prefix("0x").or_else(|| offset.strip_prefix("0X")) {
            u64::from_str_radix(hex, 16)
        } else if let Some(octal) = offset.strip_prefix('0').filter(|_| offset.len() > 1) {
            u64::from_str_radix(octal, 8)
        } else {
            offset.parse()
        }.map_err(|_| KunaError::lowlevel("Invalid Carmel XML chunk offset"))?;
        let mut digits = 0;
        for &byte in chunk.get_content() {
            if byte.is_ascii_hexdigit() {
                digits += 1;
            } else if !byte.is_ascii_whitespace() || digits % 2 != 0 {
                return Err(KunaError::lowlevel("Invalid Carmel XML bytechunk hex"));
            }
        }
        let end = start.checked_add(digits / 2)
            .filter(|&end| end <= limit && digits > 0 && digits % 2 == 0)
            .ok_or_else(|| KunaError::lowlevel("Invalid Carmel XML bytechunk extent"))?;
        ranges.push((start, end));
    }
    for ranges in [&mut code, &mut cliw] {
        ranges.sort_unstable();
        if ranges.windows(2).any(|pair| pair[0].1 > pair[1].0) {
            return Err(KunaError::lowlevel("Overlapping Carmel XML bytechunks"));
        }
    }
    Ok(!cliw.is_empty() && cliw.iter().all(|&(start, end)| {
        start.is_multiple_of(TEMPLATE) && end <= CLIW_BANK_END
    }))
}

fn space_extents<'a>(
    ranges: &'a [(Address, Address)], space: &'a Rc<AddrSpace>,
) -> impl Iterator<Item = (u64, u64)> + 'a {
    ranges.iter().filter_map(move |(start, end)| {
        start.get_space().filter(|spc| Rc::ptr_eq(spc, space))?;
        let limit = if end.get_space().is_some_and(|spc| Rc::ptr_eq(spc, space)) {
            end.get_offset()
        } else {
            space.get_highest() + 1
        };
        Some((start.get_offset(), limit))
    })
}

pub(super) fn frame_xml(
    loader: &mut dyn LoadImage,
    code: &Rc<AddrSpace>,
    cliw: &Rc<AddrSpace>,
    readonly: &[(Address, Address)],
    normalized_bank: bool,
    entries: impl IntoIterator<Item = u64>,
) -> KunaResult<Vec<ContextPaint>> {
    let templates: Vec<_> = if normalized_bank {
        space_extents(readonly, cliw).collect()
    } else {
        Vec::new()
    };
    let mut paints = Vec::new();
    let framing = frame(loader, code, space_extents(readonly, code), |loader, at, fetch| {
        // The standard wide prefilter uses three bits below the right marker.
        // Family key 5 moves the payload seam to 23 and uses left bits 0..9.
        if fetch[3] >> 4 != 0xd || templates.is_empty() {
            return Ok(());
        }
        let index = u64::from(u16::from_be_bytes([fetch[1], fetch[2]]) & 0x3ff);
        let offset = TEMPLATE * index;
        let next = templates.partition_point(|&(start, _)| start <= offset);
        if next == 0 || offset + TEMPLATE > templates[next - 1].1 {
            return Ok(());
        }
        let mut bytes = [0; TEMPLATE as usize];
        loader.load_fill(&mut bytes, &Address::new(Rc::clone(cliw), offset))?;
        for (var, value) in CLIW_FIELDS.into_iter().zip(
            std::iter::once(1).chain(bytes.chunks_exact(4).map(|w| u32::from_be_bytes(w.try_into().unwrap()))),
        ) {
            paint(&mut paints, var, at, value);
        }
        Ok(())
    })?;
    framing.require_entries(entries)?;
    paints.extend(framing.into_paints());
    Ok(paints)
}

/// Each range is an actual mapped byte extent (start, exclusive end).
pub(super) fn frame(
    loader: &mut dyn LoadImage,
    code: &Rc<AddrSpace>,
    ranges: impl IntoIterator<Item = (u64, u64)>,
    mut on_fetch: impl FnMut(&mut dyn LoadImage, u64, &Fetch) -> KunaResult<()>,
) -> KunaResult<Framing> {
    let mut result = Framing::default();
    for (start, end) in ranges {
        let mut addr = start.div_ceil(FETCH) * FETCH;
        let end = end.min(CODE_END) / FETCH * FETCH;
        if addr >= end {
            continue;
        }
        let mut fetch = [0; FETCH as usize];
        let mut next = [0; FETCH as usize];
        loader.load_fill(&mut fetch, &Address::new(Rc::clone(code), addr))?;
        while addr < end {
            let following_size = if addr + FETCH < end {
                loader.load_fill(&mut next, &Address::new(Rc::clone(code), addr + FETCH))?;
                Issue::left(&next).size()
            } else {
                0
            };
            let left = Issue::left(&fetch);
            paint(&mut result.issues, ISSUE, addr, left as u32);
            if left == Issue::SeqLeft {
                paint(&mut result.issues, ISSUE, addr + SLOT, Issue::SeqRight as u32);
                paint(&mut result.next_sizes, NEXT, addr, SLOT as u32);
                if following_size != 0 {
                    paint(&mut result.next_sizes, NEXT, addr + SLOT, following_size);
                }
            } else if following_size != 0 {
                paint(&mut result.next_sizes, NEXT, addr, following_size);
            }
            on_fetch(loader, addr, &fetch)?;
            std::mem::swap(&mut fetch, &mut next);
            addr += FETCH;
        }
    }
    Ok(result)
}

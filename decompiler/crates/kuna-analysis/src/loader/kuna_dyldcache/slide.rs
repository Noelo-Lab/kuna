//! Slide info: the chains through which a cache's data pages record their
//! pointers, rewritten here at slide zero so each slot holds its unslid target.
//!
//! Every version keeps, per page, the offset of the first pointer to rebase and
//! threads the rest through spare bits of each pointer. The versions differ in
//! where the link and the target live (`dyld_cache_format.h`):
//!
//! - v1 is a bitmap of locations; at slide zero the stored values are already
//!   the targets, so there is nothing to rewrite.
//! - v2 (arm64, x86_64 before arm64e) keeps the link under `delta_mask` and the
//!   target in the remaining bits, plus `value_add`. A page may hold several
//!   chains, listed in an extras array.
//! - v3 (arm64e) packs an 11-bit link at bit 51. A plain pointer keeps its
//!   target as 8 high bits at 43 and 43 low bits; an authenticated one keeps a
//!   32-bit offset from the cache base (`auth_value_add`) and drops its
//!   diversity and key, which the decompiler does not model.
//! - v5 (arm64e, iOS 18+) packs a 34-bit runtime offset from `value_add` and an
//!   11-bit link at bit 52; a plain pointer carries its top byte at bit 34.

/// Parsed slide info for one mapping.
#[derive(Clone, Debug)]
pub struct SlideInfo {
    pub page_size: u32,
    starts: Vec<u16>,
    kind: Kind,
}

#[derive(Clone, Debug)]
enum Kind {
    V1,
    V2 { extras: Vec<u16>, delta_mask: u64, value_add: u64 },
    V3 { auth_value_add: u64 },
    V5 { value_add: u64 },
}

const V2_NO_REBASE: u16 = 0x4000;
const V2_EXTRA: u16 = 0x8000;
const V2_END: u16 = 0x8000;
const V2_INDEX: u16 = 0x3FFF;
const V3_NO_REBASE: u16 = 0xFFFF;
const MAX_CHAIN: usize = 1 << 14;

fn u16s(raw: &[u8], offset: usize, count: usize) -> Result<Vec<u16>, String> {
    let end = offset + count * 2;
    let bytes = raw.get(offset..end).ok_or("slide info page table runs past its blob")?;
    Ok(bytes.chunks_exact(2).map(|c| u16::from_le_bytes([c[0], c[1]])).collect())
}

fn u32_at(b: &[u8], off: usize) -> u32 {
    u32::from_le_bytes(b[off..off + 4].try_into().unwrap())
}

fn u64_at(b: &[u8], off: usize) -> u64 {
    u64::from_le_bytes(b[off..off + 8].try_into().unwrap())
}

impl SlideInfo {
    /// Parse a `dyld_cache_slide_info*` blob. `Ok(None)` for a version that
    /// needs no rewrite at slide zero; an error for a version kuna does not read.
    pub fn parse(raw: &[u8]) -> Result<Option<SlideInfo>, String> {
        if raw.len() < 24 {
            return Err("slide info blob is truncated".into());
        }
        let version = u32_at(raw, 0);
        let page_size = u32_at(raw, 4);
        let info = match version {
            1 => SlideInfo { page_size: 4096, starts: Vec::new(), kind: Kind::V1 },
            2 if raw.len() >= 40 => SlideInfo {
                page_size,
                starts: u16s(raw, u32_at(raw, 8) as usize, u32_at(raw, 12) as usize)?,
                kind: Kind::V2 {
                    extras: u16s(raw, u32_at(raw, 16) as usize, u32_at(raw, 20) as usize)?,
                    delta_mask: u64_at(raw, 24),
                    value_add: u64_at(raw, 32),
                },
            },
            3 => SlideInfo {
                page_size,
                starts: u16s(raw, 24, u32_at(raw, 8) as usize)?,
                kind: Kind::V3 { auth_value_add: u64_at(raw, 16) },
            },
            5 => SlideInfo {
                page_size,
                starts: u16s(raw, 24, u32_at(raw, 8) as usize)?,
                kind: Kind::V5 { value_add: u64_at(raw, 16) },
            },
            other => return Err(format!("dyld cache slide info version {other} is not supported")),
        };
        if matches!(info.kind, Kind::V1) {
            return Ok(None);
        }
        if info.page_size == 0 || !info.page_size.is_power_of_two() {
            return Err(format!("slide info page size {page_size:#x} is not a power of two"));
        }
        Ok(Some(info))
    }

    /// Rewrite the pointers in `pages`, whose first page is page `first_page`
    /// of the mapping.
    pub fn apply(&self, pages: &mut [u8], first_page: usize) {
        let size = self.page_size as usize;
        for (i, page) in pages.chunks_mut(size).enumerate() {
            let Some(&start) = self.starts.get(first_page + i) else {
                break;
            };
            match &self.kind {
                Kind::V1 => {}
                Kind::V2 { extras, delta_mask, value_add } => {
                    if start == V2_NO_REBASE {
                        continue;
                    }
                    if start & V2_EXTRA == 0 {
                        v2_chain(page, (start & V2_INDEX) as usize * 4, *delta_mask, *value_add);
                        continue;
                    }
                    for &extra in extras.iter().skip((start & V2_INDEX) as usize).take(MAX_CHAIN) {
                        v2_chain(page, (extra & V2_INDEX) as usize * 4, *delta_mask, *value_add);
                        if extra & V2_END != 0 {
                            break;
                        }
                    }
                }
                Kind::V3 { auth_value_add } => {
                    if start != V3_NO_REBASE {
                        chain64(page, start as usize, 51, |raw| v3_target(raw, *auth_value_add));
                    }
                }
                Kind::V5 { value_add } => {
                    if start != V3_NO_REBASE {
                        chain64(page, start as usize, 52, |raw| v5_target(raw, *value_add));
                    }
                }
            }
        }
    }
}

fn v2_chain(page: &mut [u8], mut offset: usize, delta_mask: u64, value_add: u64) {
    let shift = delta_mask.trailing_zeros().saturating_sub(2);
    for _ in 0..MAX_CHAIN {
        let Some(slot) = page.get_mut(offset..offset + 8) else {
            return;
        };
        let raw = u64::from_le_bytes(slot.try_into().unwrap());
        let delta = ((raw & delta_mask) >> shift) as usize;
        let mut value = raw & !delta_mask;
        if value != 0 {
            value = value.wrapping_add(value_add);
        }
        slot.copy_from_slice(&value.to_le_bytes());
        if delta == 0 {
            return;
        }
        offset += delta;
    }
}

/// Walk a v3/v5 chain whose 11-bit link (in 8-byte units) sits at `link_bit`.
fn chain64(page: &mut [u8], mut offset: usize, link_bit: u32, target: impl Fn(u64) -> u64) {
    for _ in 0..MAX_CHAIN {
        let Some(slot) = page.get_mut(offset..offset + 8) else {
            return;
        };
        let raw = u64::from_le_bytes(slot.try_into().unwrap());
        slot.copy_from_slice(&target(raw).to_le_bytes());
        let next = ((raw >> link_bit) & 0x7FF) as usize;
        if next == 0 {
            return;
        }
        offset += next * 8;
    }
}

fn v3_target(raw: u64, auth_value_add: u64) -> u64 {
    if raw >> 63 != 0 {
        return (raw & 0xFFFF_FFFF).wrapping_add(auth_value_add);
    }
    let value51 = raw & ((1 << 51) - 1);
    let top8 = value51 & 0x0007_F800_0000_0000;
    let bottom43 = value51 & 0x0000_07FF_FFFF_FFFF;
    (top8 << 13) | bottom43
}

fn v5_target(raw: u64, value_add: u64) -> u64 {
    let target = (raw & ((1 << 34) - 1)).wrapping_add(value_add);
    if raw >> 63 != 0 {
        target
    } else {
        target | (((raw >> 34) & 0xFF) << 56)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn blob(version: u32, page_size: u32, words: &[u8]) -> Vec<u8> {
        let mut b = version.to_le_bytes().to_vec();
        b.extend(page_size.to_le_bytes());
        b.extend(words);
        b
    }

    fn page_with(slots: &[(usize, u64)]) -> Vec<u8> {
        let mut page = vec![0u8; 0x1000];
        for &(off, raw) in slots {
            page[off..off + 8].copy_from_slice(&raw.to_le_bytes());
        }
        page
    }

    fn slot(page: &[u8], off: usize) -> u64 {
        u64::from_le_bytes(page[off..off + 8].try_into().unwrap())
    }

    #[test]
    fn v3_plain_and_auth_pointers_resolve_to_unslid_targets() {
        let mut header = Vec::new();
        header.extend(1u32.to_le_bytes());
        header.extend(0u32.to_le_bytes());
        header.extend(0x1_8000_0000u64.to_le_bytes());
        header.extend(0x10u16.to_le_bytes());
        let info = SlideInfo::parse(&blob(3, 0x1000, &header)).unwrap().unwrap();
        let plain = (2u64 << 51) | 0x1_8000_1234;
        let auth = (1u64 << 63) | (1 << 48) | (0x1234 << 32) | 0x5_1234;
        let mut page = page_with(&[(0x10, plain), (0x20, auth)]);
        info.apply(&mut page, 0);
        assert_eq!(slot(&page, 0x10), 0x1_8000_1234);
        assert_eq!(slot(&page, 0x20), 0x1_8005_1234);
    }

    #[test]
    fn v3_plain_pointer_keeps_its_top_byte() {
        let raw = (0x5au64 << 43) | 0x1234;
        assert_eq!(v3_target(raw, 0), (0x5a << 56) | 0x1234);
    }

    #[test]
    fn v2_follows_extras_chains_and_keeps_zero_slots_zero() {
        let delta_mask = 0x00FF_FF00_0000_0000u64;
        let mut header = Vec::new();
        for v in [40u32, 1, 42, 2] {
            header.extend(v.to_le_bytes());
        }
        header.extend(delta_mask.to_le_bytes());
        header.extend(0x10u64.to_le_bytes());
        header.extend(0x8000u16.to_le_bytes());
        header.extend(0x0002u16.to_le_bytes());
        header.extend((0x8000u16 | 0x0040).to_le_bytes());
        let info = SlideInfo::parse(&blob(2, 0x1000, &header)).unwrap().unwrap();
        let link = |bytes: u64| bytes << 38;
        let mut page = page_with(&[
            (0x08, link(8) | 0x1000),
            (0x10, 0),
            (0x100, 0x2000),
        ]);
        info.apply(&mut page, 0);
        assert_eq!(slot(&page, 0x08), 0x1010);
        assert_eq!(slot(&page, 0x10), 0);
        assert_eq!(slot(&page, 0x100), 0x2010);
    }

    #[test]
    fn v5_adds_value_add_and_restores_the_high_byte() {
        let mut header = Vec::new();
        header.extend(1u32.to_le_bytes());
        header.extend(0u32.to_le_bytes());
        header.extend(0x1_8000_0000u64.to_le_bytes());
        header.extend(0u16.to_le_bytes());
        let info = SlideInfo::parse(&blob(5, 0x1000, &header)).unwrap().unwrap();
        let plain = (1u64 << 52) | (0x7f << 34) | 0x4000;
        let auth = (1 << 63) | (0xbeef << 34) | 0x8000;
        let mut page = page_with(&[(0, plain), (8, auth)]);
        info.apply(&mut page, 0);
        assert_eq!(slot(&page, 0), (0x7f << 56) | 0x1_8000_4000);
        assert_eq!(slot(&page, 8), 0x1_8000_8000);
    }

    #[test]
    fn v1_needs_no_rewrite_and_unknown_versions_are_refused() {
        assert!(SlideInfo::parse(&blob(1, 0, &[0; 16])).unwrap().is_none());
        assert!(SlideInfo::parse(&blob(4, 0x1000, &[0; 32])).is_err());
    }
}

//! (kuna) `rexthunk`: on x86-64, an `FF 25 <disp32>` right after a REX byte is
//! the tail of a compiler-emitted `REX FF 25` jump, not an import thunk, unless
//! the image references the `FF` as an address.
//!
//! [`super::pe_iat`] names each `FF 25 <disp32>` (`jmp [rip+disp]`) that jumps
//! through an Import Address Table slot as an import thunk entry. Every PE linker
//! emits that thunk bare: link.exe, lld-link (`ImportThunkChunkX64`) and GNU ld's
//! MinGW import stubs all begin at the `FF`. MSVC, GCC and clang instead put a
//! REX.W prefix on an indirect tail jump (`48 FF 25`), the epilogue form the
//! Windows x64 unwinder recognises. That instruction starts one byte before the
//! `FF`, and the slot still resolves from the `FF`, because a RIP-relative
//! displacement counts from the end of the instruction both readings share.
//!
//! A decoded thunk at `a` is a *REX tail* when the byte at `a - 1` is `40`-`4F`
//! and the six bytes before `a` are not themselves a resolving `FF 25`. A REX tail
//! is dropped unless the image references `a`: a direct `call`/`jmp`/`jcc rel32`
//! or a RIP-relative `lea` in an executable section targets it, its address occurs
//! as an eight-byte little-endian value in a section that is not discardable data
//! (a relocated pointer, a function table entry, a `mov reg, imm64`), or it is a
//! `.pdata` `BeginAddress` or an export. What can precede a real linker thunk:
//!
//! - Section start or alignment fill (`CC`, `90`, multi-byte NOPs ending in `00`
//!   or `90`): not a REX byte.
//! - The previous thunk of a contiguous link.exe table: the byte is its
//!   displacement's high byte, which is `00`-`3F` or `C0`-`FF` in any image under
//!   1 GiB; in a larger layout the look-back recognises that thunk.
//! - The last byte of the preceding function, when the linker placed the thunk
//!   with no padding. `jmp qword ptr [rax+0x48]` ends in `48`, so this is possible.
//!   Such a thunk keeps its name when it is reached through any of the references
//!   above. It loses it only when the image reaches it in some other way, such as
//!   a short branch, a 32-bit absolute or image-relative value, or a computed
//!   address.
//!
//! Short branches are not counted: a random `EB`/`7x` byte within 128 bytes of a
//! REX tail lands on it about one time in 200, and no linker thunk is a short
//! branch target, since COFF has no 8-bit branch relocation. A branch to the `FF`
//! of a REX tail executes the bare jump, so the import name is right wherever a
//! reference keeps one. The REX byte is not named in the tail's place: a function
//! whose whole body is the jump is a compiler-emitted wrapper with a name of its
//! own, found by the call-target walk. PE32 is not affected: in 32-bit code
//! `40`-`4F` are one-byte `inc`/`dec` instructions.

use std::collections::{HashMap, HashSet};
use std::sync::Mutex;

use object::read::pe::{ImageNtHeaders, PeFile};
use object::read::Object;
use object::LittleEndian as LE;

/// Whether the thunk decoded at `data[i]` (section base `vma`) is a REX tail: the
/// byte before it is a REX prefix and no resolving `FF 25` ends at `i`.
pub(crate) fn is_rex_tail(vma: u64, data: &[u8], i: usize, slot_to_name: &HashMap<u64, Vec<u8>>) -> bool {
    if i == 0 || !(0x40..=0x4F).contains(&data[i - 1]) {
        return false;
    }
    !(i >= 6 && data[i - 6] == 0xFF && data[i - 5] == 0x25 && {
        let disp = i32::from_le_bytes([data[i - 4], data[i - 3], data[i - 2], data[i - 1]]);
        let slot = (vma + i as u64).wrapping_add(disp as i64 as u64);
        slot_to_name.contains_key(&slot)
    })
}

/// The addresses an x64 PE declares to begin code: every `.pdata` `BeginAddress`
/// and every export.
pub(crate) fn declared_starts<Pe: ImageNtHeaders>(neutral: &object::File, pe: &PeFile<Pe>, bytes: &[u8]) -> Vec<u64> {
    let mut starts: Vec<u64> = Vec::new();
    if pe.nt_headers().file_header().machine.get(LE) != object::pe::IMAGE_FILE_MACHINE_AMD64 {
        return starts;
    }
    let base = pe.relative_address_base();
    if let Some(dir) = pe.data_directories().get(object::pe::IMAGE_DIRECTORY_ENTRY_EXCEPTION) {
        if let Ok(data) = dir.data(bytes, &pe.section_table()) {
            for rec in data.chunks_exact(12) {
                starts.push(base + u32::from_le_bytes([rec[0], rec[1], rec[2], rec[3]]) as u64);
            }
        }
    }
    if let Ok(exports) = neutral.exports() {
        starts.extend(exports.iter().map(|e| e.address()));
    }
    starts
}

/// [`referenced`], remembered for the last few images: `format::resolve_imports`
/// runs about twenty times per load and the scans cost milliseconds per megabyte.
/// The key hashes every input, section flags and addresses included, so an image
/// never reads another image's answer.
pub(crate) fn referenced_cached(
    code: &[(u64, &[u8])],
    image: &[(u64, &[u8])],
    starts: &[u64],
    wanted: &HashSet<u64>,
) -> HashSet<u64> {
    static CACHE: Mutex<Vec<(u64, HashSet<u64>)>> = Mutex::new(Vec::new());
    let key = fingerprint(code, image, starts, wanted);
    if let Some((_, hit)) = CACHE.lock().unwrap_or_else(|e| e.into_inner()).iter().find(|e| e.0 == key) {
        return hit.clone();
    }
    let found = referenced(code, image, starts, wanted);
    let mut cache = CACHE.lock().unwrap_or_else(|e| e.into_inner());
    if cache.len() == 4 {
        cache.remove(0);
    }
    cache.push((key, found.clone()));
    found
}

fn fingerprint(code: &[(u64, &[u8])], image: &[(u64, &[u8])], starts: &[u64], wanted: &HashSet<u64>) -> u64 {
    const K: u64 = 0x9E37_79B9_7F4A_7C15;
    let mix = |h: u64, v: u64| (h ^ v).wrapping_mul(K).rotate_left(29);
    let mut h = 0x243F_6A88_85A3_08D3u64;
    let mut sorted: Vec<u64> = wanted.iter().copied().collect();
    sorted.sort_unstable();
    let heads = code.iter().chain(image).map(|&(vma, data)| [vma, data.len() as u64]);
    for v in heads.flatten().chain(starts.iter().copied()).chain(sorted) {
        h = mix(h, v);
    }
    for &(_, data) in image {
        let (mut a, mut b) = (h, h.rotate_left(32) ^ K);
        let mut chunks = data.chunks_exact(16);
        for c in &mut chunks {
            a = mix(a, u64::from_le_bytes([c[0], c[1], c[2], c[3], c[4], c[5], c[6], c[7]]));
            b = mix(b, u64::from_le_bytes([c[8], c[9], c[10], c[11], c[12], c[13], c[14], c[15]]));
        }
        for &byte in chunks.remainder() {
            a = mix(a, byte as u64);
        }
        h = mix(a, b);
    }
    h
}

/// The addresses in `wanted` the image references: a direct `call`/`jmp`/`jcc rel32`
/// or RIP-relative `lea` anywhere in `code`, an eight-byte little-endian occurrence
/// anywhere in `image`, or membership in `starts`. Every byte offset is tried, since
/// instruction and data boundaries are unknown at load time.
pub(crate) fn referenced(
    code: &[(u64, &[u8])],
    image: &[(u64, &[u8])],
    starts: &[u64],
    wanted: &HashSet<u64>,
) -> HashSet<u64> {
    let mut sorted: Vec<u64> = wanted.iter().copied().collect();
    sorted.sort_unstable();
    let mut hit: HashSet<u64> = starts.iter().copied().filter(|a| wanted.contains(a)).collect();
    let (Some(&lo), Some(&hi)) = (sorted.first(), sorted.last()) else { return hit };
    let mut note = |target: u64| {
        if (lo..=hi).contains(&target) && sorted.binary_search(&target).is_ok() {
            hit.insert(target);
        }
    };
    for &(vma, data) in code {
        for p in 0..data.len() {
            if let Some(target) = rel_target(vma, data, p) {
                note(target);
            }
        }
    }
    for &(_, data) in image {
        for w in data.windows(8) {
            note(u64::from_le_bytes([w[0], w[1], w[2], w[3], w[4], w[5], w[6], w[7]]));
        }
    }
    hit
}

/// The target of the `call`/`jmp`/`jcc rel32` whose opcode byte is `data[p]`, or of
/// the RIP-relative `lea` whose `8D` is `data[p]`.
fn rel_target(vma: u64, data: &[u8], p: usize) -> Option<u64> {
    let (end, disp_at) = match data[p] {
        0xE8 | 0xE9 => (p + 5, p + 1),
        0x0F if (0x80..=0x8F).contains(data.get(p + 1)?) => (p + 6, p + 2),
        0x8D if p > 0 && (0x40..=0x4F).contains(&data[p - 1]) && data.get(p + 1)? & 0xC7 == 0x05 => (p + 6, p + 2),
        _ => return None,
    };
    let d = data.get(disp_at..disp_at + 4)?;
    let disp = i32::from_le_bytes([d[0], d[1], d[2], d[3]]);
    Some((vma + end as u64).wrapping_add(disp as i64 as u64))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn slots(v: &[u64]) -> HashMap<u64, Vec<u8>> {
        v.iter().map(|&s| (s, b"import".to_vec())).collect()
    }

    fn ff25(at: u64, slot: u64) -> Vec<u8> {
        let mut b = vec![0xFF, 0x25];
        b.extend_from_slice(&((slot.wrapping_sub(at + 6)) as i32).to_le_bytes());
        b
    }

    fn refs(code: &[(u64, &[u8])], wanted: u64) -> bool {
        referenced(code, &[], &[], &HashSet::from([wanted])).contains(&wanted)
    }

    /// The cache answers per image: two images of one size that differ only in a
    /// pointer, or only in which section holds it, get their own answers.
    #[test]
    fn cached_answers_follow_the_image_bytes() {
        let target = 0x140001207u64;
        let wanted = HashSet::from([target]);
        let with = target.to_le_bytes();
        let without = (target + 1).to_le_bytes();
        for _ in 0..2 {
            assert_eq!(referenced_cached(&[], &[(0x140003000, &with[..])], &[], &wanted), wanted);
            assert!(referenced_cached(&[], &[(0x140003000, &without[..])], &[], &wanted).is_empty());
            let code = [(0x140001000u64, &without[..])];
            assert!(referenced_cached(&code, &[(0x140001000, &without[..])], &[], &wanted).is_empty());
        }
    }

    #[test]
    fn a_rex_byte_before_ff25_is_a_tail_and_padding_is_not() {
        let vma = 0x140001000;
        let slot = 0x140002000;
        for (prev, tail) in [(0x48u8, true), (0x40, true), (0x4F, true), (0xCC, false), (0x90, false), (0x00, false)] {
            let mut data = vec![prev];
            data.extend(ff25(vma + 1, slot));
            assert_eq!(is_rex_tail(vma, &data, 1, &slots(&[slot])), tail, "prev {prev:#x}");
        }
        let data = ff25(vma, slot);
        assert!(!is_rex_tail(vma, &data, 0, &slots(&[slot])), "a section-leading FF 25 has no prefix");
    }

    /// A contiguous link.exe table in a layout over 1 GiB: the previous thunk's
    /// displacement high byte is `40`, and the look-back sees that thunk.
    #[test]
    fn a_thunk_after_a_far_thunk_of_the_same_table_is_not_a_tail() {
        let vma = 0x140001000u64;
        let (s0, s1) = (vma + 0x4000_1000, vma + 0x4000_1008);
        let mut data = ff25(vma, s0);
        assert_eq!(data[5], 0x40);
        data.extend(ff25(vma + 6, s1));
        assert!(!is_rex_tail(vma, &data, 6, &slots(&[s0, s1])));
        assert!(is_rex_tail(vma, &data, 6, &slots(&[s1])), "an unresolving look-back proves nothing");
    }

    #[test]
    fn rel32_branches_and_rip_relative_lea_are_references() {
        let vma = 0x140001000u64;
        let target = 0x140001100u64;
        for op in [&[0xE8u8][..], &[0xE9], &[0x0F, 0x84], &[0x48, 0x8D, 0x0D], &[0x4C, 0x8D, 0x05]] {
            let mut code = vec![0x90; 3];
            code.extend_from_slice(op);
            let end = vma + (code.len() + 4) as u64;
            code.extend_from_slice(&((target.wrapping_sub(end)) as i32).to_le_bytes());
            assert!(refs(&[(vma, &code)], target), "{op:x?}");
            assert!(!refs(&[(vma, &code)], target + 1), "{op:x?}");
        }
        let mov = [0x48u8, 0x8B, 0x0D, 0, 0, 0, 0];
        assert!(!refs(&[(vma, &mov)], vma + 7), "a load is not a reference");
        let short = [0xEBu8, 0x10, 0x74, 0x0E];
        assert!(!refs(&[(vma, &short)], vma + 0x12), "a short branch is not counted");
    }

    #[test]
    fn a_reference_from_another_section_counts() {
        let text = (0x140001000u64, vec![0xCCu8; 16]);
        let call_at = 0x140005000u64;
        let mut stub = vec![0xE8];
        stub.extend_from_slice(&((0x140001008u64.wrapping_sub(call_at + 5)) as i32).to_le_bytes());
        let code = [(text.0, &text.1[..]), (call_at, &stub[..])];
        assert!(refs(&code, 0x140001008));
    }

    /// An absolute pointer anywhere in any section, at any alignment, and a declared
    /// start are references; a pointer one byte off is not.
    #[test]
    fn eight_byte_pointers_and_declared_starts_are_references() {
        let target = 0x140001207u64;
        let wanted = HashSet::from([target]);
        let mut data = vec![0u8; 3];
        data.extend_from_slice(&target.to_le_bytes());
        let imm64 = [&[0x48u8, 0xB8][..], &target.to_le_bytes()[..], &[0xFF, 0xD0][..]].concat();
        for blob in [&data[..], &imm64[..]] {
            assert_eq!(referenced(&[], &[(0x140003000, blob)], &[], &wanted), wanted);
        }
        let off = (target + 1).to_le_bytes();
        assert!(referenced(&[], &[(0x140003000, &off[..])], &[], &wanted).is_empty());
        assert_eq!(referenced(&[], &[], &[0x140001000, target], &wanted), wanted);
    }
}

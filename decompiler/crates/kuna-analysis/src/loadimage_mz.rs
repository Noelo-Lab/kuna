//! (kuna) The MS-DOS `MZ` executable: the header, the load module and its
//! segment relocations, read into the flat image an x86 real-mode decoder sees.
//!
//! DOS copies the load module (the file past the header, up to the size the
//! header declares) to a free paragraph, adds that load segment to every word
//! the relocation table names, and starts at `CS:IP` relative to it. kuna does
//! the same at [`MZ_LOAD_SEGMENT`], so near and far references, the entry and
//! the stack all land where the program puts them, at linear `segment * 16 +
//! offset`. Uninitialized memory past the module (the header's minimum
//! allocation) is not mapped: a flow into it is a flow into nothing, not into
//! zeros that decode as code. An `MZ` stub in front of a PE, NE, LE or LX image
//! is not claimed.

use kuna_base::error::{KunaError, KunaResult};

/// The paragraph the load module is placed at, the conventional `1000:0000`.
pub const MZ_LOAD_SEGMENT: u16 = 0x1000;

/// The language a DOS executable decodes as unless the caller names another.
pub const MZ_LANGUAGE: &str = "x86:LE:16:Real Mode:default";

const HEADER_LEN: usize = 0x1c;

/// A parsed DOS executable, its load module already relocated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MzImage {
    /// The load module bytes with every in-range relocation applied.
    pub module: Vec<u8>,
    /// Linear address of the first module byte.
    pub base: u64,
    /// Linear address of the initial `CS:IP`.
    pub entry: u64,
    /// Linear address of the initial `SS:SP`.
    pub stack: u64,
    /// Relocation entries applied.
    pub relocations: usize,
    /// Relocation entries that name a word outside the module.
    pub skipped_relocations: usize,
    /// A recognized executable packer whose stub this image is.
    pub packer: Option<&'static str>,
    /// The code segments the image states, as `(linear, CS)`: the entry's
    /// `CS:IP`, and the target of every far `CALL`/`JMP ptr16:16` whose
    /// segment word a relocation names, when it lies in the module and below
    /// the next segment the image names.
    pub code_segments: Vec<(u64, u16)>,
}

fn word(bytes: &[u8], off: usize) -> u16 {
    u16::from_le_bytes([bytes[off], bytes[off + 1]])
}

/// Whether `e_lfanew` (0x3c..0x40) can hold a pointer to a newer header: it
/// must lie in the header and outside the relocation table, as deark decides.
fn lfanew_is_a_field(bytes: &[u8]) -> bool {
    let header = usize::from(word(bytes, 0x08)) * 16;
    let nrelocs = usize::from(word(bytes, 0x06));
    let table = usize::from(word(bytes, 0x18));
    let overlaps = (0x3d..0x40).contains(&table) || (nrelocs > 0 && table < 0x40 && table + 4 * nrelocs > 0x3c);
    header > 0x3c && !overlaps
}

/// Whether `bytes` is a plain DOS executable: an `MZ` (or `ZM`) header that is
/// not the stub of a newer executable format.
pub fn is_mz_dos_image(bytes: &[u8]) -> bool {
    if bytes.len() < HEADER_LEN || !matches!(&bytes[..2], b"MZ" | b"ZM") {
        return false;
    }
    if bytes.len() >= 0x40 {
        let lfanew = u32::from_le_bytes([bytes[0x3c], bytes[0x3d], bytes[0x3e], bytes[0x3f]]) as usize;
        if let Some(sig) = bytes.get(lfanew..lfanew.saturating_add(4)) {
            if lfanew >= 4 && sig == b"PE\0\0" {
                return false;
            }
            if lfanew >= 0x40 && matches!(&sig[..2], b"NE" | b"LE" | b"LX") && lfanew_is_a_field(bytes) {
                return false;
            }
        }
    }
    !matches!(
        object::FileKind::parse(bytes),
        Ok(object::FileKind::Pe32 | object::FileKind::Pe64)
    )
}

fn packer(bytes: &[u8]) -> Option<&'static str> {
    let tail = bytes.get(0x1c..bytes.len().min(0x80))?;
    if tail.windows(6).any(|w| w.eq_ignore_ascii_case(b"PKLITE")) {
        return Some("PKLITE");
    }
    if matches!(tail.get(..4), Some(b"LZ09") | Some(b"LZ91")) {
        return Some("LZEXE");
    }
    None
}

fn linear(load: u16, segment: u16, offset: u16) -> u64 {
    u64::from(load.wrapping_add(segment)) * 16 + u64::from(offset)
}

impl MzImage {
    /// Read the header, cut out the load module and relocate it at
    /// [`MZ_LOAD_SEGMENT`].
    pub fn parse(bytes: &[u8]) -> KunaResult<MzImage> {
        if !is_mz_dos_image(bytes) {
            return Err(KunaError::lowlevel("not an MS-DOS MZ executable"));
        }
        let last_page = usize::from(word(bytes, 0x02));
        let pages = usize::from(word(bytes, 0x04));
        let nrelocs = usize::from(word(bytes, 0x06));
        let header = usize::from(word(bytes, 0x08)) * 16;
        let image_end = match last_page {
            0 => pages * 512,
            n => (pages.saturating_sub(1) * 512) + n.min(512),
        }
        .min(bytes.len());
        if header < HEADER_LEN || header >= image_end {
            return Err(KunaError::lowlevel(format!(
                "MZ header of {header} bytes leaves no load module in a {image_end}-byte image"
            )));
        }
        let mut module = bytes[header..image_end].to_vec();
        let table = usize::from(word(bytes, 0x18));
        let mut relocations = 0;
        let mut skipped_relocations = 0;
        let mut sites = Vec::new();
        for i in 0..nrelocs {
            let at = table + 4 * i;
            let Some(entry) = bytes.get(at..at + 4) else {
                skipped_relocations += nrelocs - i;
                break;
            };
            let site = usize::from(word(entry, 2)) * 16 + usize::from(word(entry, 0));
            match module.get_mut(site..site + 2) {
                Some(slot) => {
                    let value = u16::from_le_bytes([slot[0], slot[1]]).wrapping_add(MZ_LOAD_SEGMENT);
                    slot.copy_from_slice(&value.to_le_bytes());
                    relocations += 1;
                    sites.push(site);
                }
                None => skipped_relocations += 1,
            }
        }
        let base = u64::from(MZ_LOAD_SEGMENT) * 16;
        let entry = linear(MZ_LOAD_SEGMENT, word(bytes, 0x16), word(bytes, 0x14));
        if entry < base || entry >= base + module.len() as u64 {
            return Err(KunaError::lowlevel(format!(
                "MZ entry {:04x}:{:04x} lies outside the {}-byte load module",
                word(bytes, 0x16),
                word(bytes, 0x14),
                module.len()
            )));
        }
        let entry_cs = MZ_LOAD_SEGMENT.wrapping_add(word(bytes, 0x16));
        let mut code_segments = vec![(entry, entry_cs)];
        let mut named: Vec<u16> = sites.iter().map(|&site| word(&module, site)).collect();
        named.push(entry_cs);
        named.sort_unstable();
        named.dedup();
        code_segments.extend(far_targets(&module, base, &sites, &named));
        code_segments.sort_unstable();
        code_segments.dedup();
        Ok(MzImage {
            module,
            base,
            entry,
            stack: linear(MZ_LOAD_SEGMENT, word(bytes, 0x0e), word(bytes, 0x10)),
            relocations,
            skipped_relocations,
            packer: packer(bytes),
            code_segments,
        })
    }
}

/// The `(linear, CS)` target of each far `CALL`/`JMP ptr16:16` (`9a`/`ea`
/// followed by offset and segment) whose segment word sits at a relocated
/// `site`, when the target lies in the module.
///
/// A far branch lands in its own segment, which ends where the next segment
/// in `named` (every relocated word and the entry's CS) begins. A `9a`/`ea`
/// byte three bytes before a segment word can also be the tail of another
/// instruction (`mov ax, 0x109a; mov dx, seg x` is `b8 9a 10 ba ..`), and the
/// target it implies then lies past that segment, in code that runs with
/// another CS; such a target is not stated.
fn far_targets<'a>(
    module: &'a [u8],
    base: u64,
    sites: &'a [usize],
    named: &'a [u16],
) -> impl Iterator<Item = (u64, u16)> + 'a {
    let end = base + module.len() as u64;
    sites.iter().filter_map(move |&site| {
        let opcode = *module.get(site.checked_sub(3)?)?;
        if !matches!(opcode, 0x9a | 0xea) || site + 2 > module.len() {
            return None;
        }
        let segment = word(module, site);
        let target = u64::from(segment) * 16 + u64::from(word(module, site - 2));
        let next = named.get(named.partition_point(|&other| other <= segment));
        let next = next.map_or(end, |&other| u64::from(other) * 16);
        (base..end.min(next)).contains(&target).then_some((target, segment))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A two-paragraph header, one relocation at module offset 6, entry 0000:0000.
    fn image() -> Vec<u8> {
        let code = [
            0xe8, 0x0d, 0x00, // call 0x10
            0x9a, 0x00, 0x00, 0x02, 0x00, // callf 0002:0000 (segment relocated)
            0xb8, 0x00, 0x4c, 0xcd, 0x21, // mov ax,0x4c00 ; int 0x21
        ];
        let mut module = vec![0x90u8; 0x24];
        module[..code.len()].copy_from_slice(&code);
        module[0x10..0x14].copy_from_slice(&[0xb8, 0x07, 0x00, 0xc3]);
        module[0x20..0x24].copy_from_slice(&[0xb8, 0x09, 0x00, 0xcb]);
        let total = 0x20 + module.len();
        let mut h = vec![0u8; 0x20];
        h[..2].copy_from_slice(b"MZ");
        h[2..4].copy_from_slice(&((total % 512) as u16).to_le_bytes());
        h[4..6].copy_from_slice(&(total.div_ceil(512) as u16).to_le_bytes());
        h[6..8].copy_from_slice(&1u16.to_le_bytes());
        h[8..10].copy_from_slice(&2u16.to_le_bytes());
        h[0x0e..0x10].copy_from_slice(&0x0003u16.to_le_bytes());
        h[0x10..0x12].copy_from_slice(&0x0100u16.to_le_bytes());
        h[0x18..0x1a].copy_from_slice(&0x1cu16.to_le_bytes());
        h[0x1c..0x20].copy_from_slice(&[0x06, 0x00, 0x00, 0x00]);
        h.extend_from_slice(&module);
        h.extend_from_slice(b"overlay bytes past the declared image");
        h
    }

    #[test]
    fn the_module_is_relocated_at_the_load_segment() {
        let mz = MzImage::parse(&image()).unwrap();
        assert_eq!(mz.base, 0x10000);
        assert_eq!(mz.entry, 0x10000);
        assert_eq!(mz.stack, 0x10130);
        assert_eq!(mz.module.len(), 0x24, "the overlay past the image is not mapped");
        assert_eq!(&mz.module[6..8], &0x1002u16.to_le_bytes());
        assert_eq!((mz.relocations, mz.skipped_relocations, mz.packer), (1, 0, None));
        assert_eq!(mz.code_segments, vec![(0x10000, 0x1000), (0x10020, 0x1002)]);
    }

    #[test]
    fn a_far_target_outside_the_module_states_no_segment() {
        let mut bytes = image();
        bytes[0x24..0x26].copy_from_slice(&0x0100u16.to_le_bytes());
        assert_eq!(MzImage::parse(&bytes).unwrap().code_segments, vec![(0x10000, 0x1000)]);
    }

    /// `mov bx, 0x9a00; mov word [bx], seg 0000` (`bb 00 9a c7 07 ..`) reads
    /// as `callf 1000:07c7` from its 9a byte, a target in segment 0x1010,
    /// which the real far call names.
    #[test]
    fn a_far_target_past_the_next_named_segment_states_no_segment() {
        let mut module = vec![0x90u8; 0x800];
        module[..0x0d].copy_from_slice(&[
            0x9a, 0x00, 0x00, 0x10, 0x00, // callf 0010:0000 (segment relocated)
            0xbb, 0x00, 0x9a, // mov bx, 0x9a00
            0xc7, 0x07, 0x00, 0x00, // mov word [bx], seg 0000 (relocated)
            0xcb, // retf
        ]);
        module[0x100..0x104].copy_from_slice(&[0xb8, 0x09, 0x00, 0xcb]);
        let total = 0x30 + module.len();
        let mut h = vec![0u8; 0x30];
        h[..2].copy_from_slice(b"MZ");
        h[2..4].copy_from_slice(&((total % 512) as u16).to_le_bytes());
        h[4..6].copy_from_slice(&(total.div_ceil(512) as u16).to_le_bytes());
        h[6..8].copy_from_slice(&2u16.to_le_bytes());
        h[8..10].copy_from_slice(&3u16.to_le_bytes());
        h[0x18..0x1a].copy_from_slice(&0x1cu16.to_le_bytes());
        h[0x1c..0x24].copy_from_slice(&[0x03, 0x00, 0x00, 0x00, 0x0a, 0x00, 0x00, 0x00]);
        h.extend_from_slice(&module);
        let mz = MzImage::parse(&h).unwrap();
        assert_eq!(&mz.module[0x0a..0x0c], &0x1000u16.to_le_bytes());
        assert_eq!(mz.code_segments, vec![(0x10000, 0x1000), (0x10100, 0x1010)]);
    }

    #[test]
    fn a_relocation_outside_the_module_is_skipped() {
        let mut bytes = image();
        bytes[0x1e..0x20].copy_from_slice(&0x0100u16.to_le_bytes());
        let mz = MzImage::parse(&bytes).unwrap();
        assert_eq!((mz.relocations, mz.skipped_relocations), (0, 1));
        assert_eq!(&mz.module[6..8], &0x0002u16.to_le_bytes());
    }

    #[test]
    fn newer_formats_behind_an_mz_stub_are_not_claimed() {
        let mut bytes = image();
        bytes.resize(0x200, 0);
        assert!(is_mz_dos_image(&bytes));
        for sig in [&b"PE\0\0"[..], b"NE", b"LE", b"LX"] {
            let mut stub = bytes.clone();
            stub[0x08..0x0a].copy_from_slice(&4u16.to_le_bytes());
            stub[0x3c..0x40].copy_from_slice(&0x100u32.to_le_bytes());
            stub[0x100..0x100 + sig.len()].copy_from_slice(sig);
            assert!(!is_mz_dos_image(&stub), "{sig:?}");
        }
        assert!(!is_mz_dos_image(b"MZ"));
        assert!(!is_mz_dos_image(&[0x7f, b'E', b'L', b'F', 0, 0, 0, 0]));
    }

    /// Bytes 0x3c..0x40 are only `e_lfanew` when the header reaches past them
    /// and the relocation table does not cover them.
    #[test]
    fn an_lfanew_that_is_code_or_a_relocation_is_not_a_newer_header() {
        let mut bytes = image();
        bytes.resize(0x200, 0);
        bytes[0x3c..0x40].copy_from_slice(&0x100u32.to_le_bytes());
        bytes[0x100..0x102].copy_from_slice(b"LE");
        assert!(is_mz_dos_image(&bytes), "a two-paragraph header ends before e_lfanew");
        bytes[0x08..0x0a].copy_from_slice(&5u16.to_le_bytes());
        assert!(!is_mz_dos_image(&bytes));
        bytes[0x06..0x08].copy_from_slice(&10u16.to_le_bytes());
        assert!(is_mz_dos_image(&bytes), "the relocation table covers e_lfanew");
    }

    #[test]
    fn an_entry_outside_the_module_is_refused() {
        let mut bytes = image();
        bytes[0x16..0x18].copy_from_slice(&0x0100u16.to_le_bytes());
        assert!(MzImage::parse(&bytes).unwrap_err().to_string().contains("outside"));
    }

    #[test]
    fn a_packer_stub_is_named() {
        let mut bytes = image();
        assert_eq!(packer(&bytes), None);
        bytes[0x30..0x36].copy_from_slice(b"PKLITE");
        assert_eq!(MzImage::parse(&bytes).unwrap().packer, Some("PKLITE"));
        let mut bytes = image();
        bytes[0x1c..0x20].copy_from_slice(b"LZ91");
        assert_eq!(packer(&bytes), Some("LZEXE"));
    }
}

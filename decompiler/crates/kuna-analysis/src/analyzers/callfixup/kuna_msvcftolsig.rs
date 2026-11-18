//! (kuna) Recognize the statically linked MSVC x86-32 float-to-integer CRT
//! helpers in a stripped PE by their bytes, so the `kuna_msvcftol` call-fixup
//! reaches them.
//!
//! The fixup is installed by name, and a stripped image names nothing: every
//! `fld …; call sub_XXXX` then loses its x87 operand. Each recognized entry is
//! reported as a function symbol under its CRT name (plus a placeholder rename
//! for an entry another fact already installed) and as a [`CallFixupFact`],
//! which the commit resolves by that name.
//!
//! Signatures are the complete instruction streams of the VS2005 `libcmt` /
//! `msvcrt` objects `ftol2.obj` and `ftol.obj`, with only the relocated
//! `___sse2_available` operands wildcarded:
//!
//! | block | entries (offset → name) |
//! |---|---|
//! | `ftol2.obj` | 0x00 `__ftol2_sse`, 0x09 `__ftol2_pentium4`, 0x1c `__ftol2_sse_excpt`, 0x36 `__ftol2` |
//! | `ftol2.obj` x87 tail alone | 0x00 `__ftol2` |
//! | `ftol.obj` | 0x00 `__ftol` |

use std::ops::Range;

use memchr::memmem;
use object::read::{Object, ObjectSection};
use object::SectionKind;

use crate::pass::{AnalysisOutput, CallFixupFact, FidMatch, SymFact, SymKind};

/// A wildcard-free byte stream, optionally preceded at `head_at` by a
/// wildcarded head that adds further entries.
struct Signature {
    bytes: &'static [u8],
    entries: &'static [(usize, &'static str)],
    head: Option<Head>,
}

/// `wild` ranges match any byte.
struct Head {
    at: usize,
    bytes: &'static [u8],
    wild: &'static [Range<usize>],
    entries: &'static [(usize, &'static str)],
}

const FTOL2_X87: &[u8] = &[
    0x55, 0x8b, 0xec, 0x83, 0xec, 0x20, 0x83, 0xe4, 0xf0, 0xd9, 0xc0, 0xd9, 0x54, 0x24, 0x18, 0xdf,
    0x7c, 0x24, 0x10, 0xdf, 0x6c, 0x24, 0x10, 0x8b, 0x54, 0x24, 0x18, 0x8b, 0x44, 0x24, 0x10, 0x85,
    0xc0, 0x74, 0x3c, 0xde, 0xe9, 0x85, 0xd2, 0x79, 0x1e, 0xd9, 0x1c, 0x24, 0x8b, 0x0c, 0x24, 0x81,
    0xf1, 0x00, 0x00, 0x00, 0x80, 0x81, 0xc1, 0xff, 0xff, 0xff, 0x7f, 0x83, 0xd0, 0x00, 0x8b, 0x54,
    0x24, 0x14, 0x83, 0xd2, 0x00, 0xeb, 0x2c, 0xd9, 0x1c, 0x24, 0x8b, 0x0c, 0x24, 0x81, 0xc1, 0xff,
    0xff, 0xff, 0x7f, 0x83, 0xd8, 0x00, 0x8b, 0x54, 0x24, 0x14, 0x83, 0xda, 0x00, 0xeb, 0x14, 0x8b,
    0x54, 0x24, 0x14, 0xf7, 0xc2, 0xff, 0xff, 0xff, 0x7f, 0x75, 0xb8, 0xd9, 0x5c, 0x24, 0x18, 0xd9,
    0x5c, 0x24, 0x18, 0xc9, 0xc3,
];

const FTOL2_SSE_HEAD: &[u8] = &[
    0x83, 0x3d, 0x00, 0x00, 0x00, 0x00, 0x00, 0x74, 0x2d, 0x55, 0x8b, 0xec, 0x83, 0xec, 0x08, 0x83,
    0xe4, 0xf8, 0xdd, 0x1c, 0x24, 0xf2, 0x0f, 0x2c, 0x04, 0x24, 0xc9, 0xc3, 0x83, 0x3d, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x74, 0x11, 0x83, 0xec, 0x04, 0xd9, 0x3c, 0x24, 0x58, 0x66, 0x83, 0xe0, 0x7f,
    0x66, 0x83, 0xf8, 0x7f, 0x74, 0xd3,
];

/// The two relocated `___sse2_available` operands.
const FTOL2_SSE_HEAD_WILD: &[Range<usize>] = &[2..6, 0x1e..0x22];

const FTOL: &[u8] = &[
    0x55, 0x8b, 0xec, 0x83, 0xc4, 0xf4, 0x9b, 0xd9, 0x7d, 0xfe, 0x9b, 0x66, 0x8b, 0x45, 0xfe, 0x80,
    0xcc, 0x0c, 0x66, 0x89, 0x45, 0xfc, 0xd9, 0x6d, 0xfc, 0xdf, 0x7d, 0xf4, 0xd9, 0x6d, 0xfe, 0x8b,
    0x45, 0xf4, 0x8b, 0x55, 0xf8, 0xc9, 0xc3,
];

const SIGNATURES: [Signature; 2] = [
    Signature {
        bytes: FTOL2_X87,
        entries: &[(0, "__ftol2")],
        head: Some(Head {
            at: 0x36,
            bytes: FTOL2_SSE_HEAD,
            wild: FTOL2_SSE_HEAD_WILD,
            entries: &[(0, "__ftol2_sse"), (0x09, "__ftol2_pentium4"), (0x1c, "__ftol2_sse_excpt")],
        }),
    },
    Signature { bytes: FTOL, entries: &[(0, "__ftol")], head: None },
];

fn head_matches(code: &[u8], start: usize, head: &Head) -> bool {
    code.get(start..start + head.bytes.len()).is_some_and(|w| {
        w.iter()
            .zip(head.bytes)
            .enumerate()
            .all(|(i, (b, s))| b == s || head.wild.iter().any(|r| r.contains(&i)))
    })
}

/// Every recognized helper entry in `code` (mapped at `base`), as `(vma, name)`.
pub fn scan_bytes(code: &[u8], base: u64) -> Vec<(u64, &'static str)> {
    let mut hits = Vec::new();
    for sig in &SIGNATURES {
        for at in memmem::find_iter(code, sig.bytes) {
            hits.extend(sig.entries.iter().map(|&(off, name)| (base + (at + off) as u64, name)));
            let Some(head) = &sig.head else { continue };
            let Some(start) = at.checked_sub(head.at) else { continue };
            if head_matches(code, start, head) {
                hits.extend(
                    head.entries.iter().map(|&(off, name)| (base + (start + off) as u64, name)),
                );
            }
        }
    }
    hits.sort_unstable();
    hits
}

/// Target gate: a PE32 i386 image. A COFF object keeps its symbols and needs no
/// recognition; no other target links this CRT.
fn is_target(file: &object::File) -> bool {
    file.format() == object::BinaryFormat::Pe && file.architecture() == object::Architecture::I386
}

/// Scan the executable sections of `file` and report each helper entry.
pub fn scan(file: &object::File, out: &mut AnalysisOutput) {
    if !is_target(file) {
        return;
    }
    for sec in file.sections() {
        let exec = sec.kind() == SectionKind::Text
            || matches!(
                sec.flags(),
                object::SectionFlags::Coff { characteristics }
                    if characteristics & object::pe::IMAGE_SCN_MEM_EXECUTE != 0
            );
        if !exec {
            continue;
        }
        let Ok(code) = sec.data() else { continue };
        for (addr, name) in scan_bytes(code, sec.address()) {
            out.symbols.push(SymFact { addr, name: name.to_string(), kind: SymKind::Function });
            out.fid_names.push(FidMatch { addr, name: name.to_string(), by_signature: false });
            if !out.call_fixups.iter().any(|f| f.func_name == name) {
                out.call_fixups.push(CallFixupFact { func_name: name.to_string() });
            }
        }
    }
}

#[cfg(test)]
mod tests;

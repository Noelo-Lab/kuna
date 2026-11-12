//! (kuna) Bind the MSVC x86-32 one-operand x87 math intrinsics to their model.
//!
//! `__CIsin`, `__CIcos`, `__CIsqrt` and their siblings take their operand in `ST0`
//! and return in `ST0`; under any `x86win.cspec` model a call to one recovers no
//! argument, and the expression that computed the operand is dead-code
//! eliminated. Each such function is bound to the dedicated model of
//! [`kuna_decomp::kuna_x87intrinsic`]: by name where the image names it, and in a
//! stripped PE32 i386 image by recognizing the VS2005 CRT object it came from.
//!
//! Recognition stores no CRT code. Each signature is a 16-byte anchor inside the
//! object, the object's length, the byte ranges its relocations patch, and a
//! 64-bit FNV-1a hash of the object with those ranges zeroed. An anchor hit whose
//! window hashes to a signature names the intrinsic entry `__CI<name>` (bound to
//! the model) and the object's C entry `<name>`, which keeps its stack argument.
//! The intrinsic's signature is locked as well (`long double name(long double)`):
//! with only the model, an operand the caller also spills and reloads around the
//! call failed the input trial and the call printed without it.

use std::ops::Range;

use memchr::memmem;
use object::read::Object;

use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, FidMatch, ModelFact, Phase, SymFact, SymKind};

struct Signature {
    intrinsic: &'static str,
    c_name: &'static str,
    len: usize,
    intrinsic_at: usize,
    c_at: usize,
    anchor_at: usize,
    anchor: &'static [u8],
    wild: &'static [Range<usize>],
    hash: u64,
}

/// The VS2005 `libcmt`/`msvcrt` objects `sqrt.obj`, `sin.obj`, `cos.obj`,
/// `tan.obj`, `log.obj`, `log10.obj`, `asin.obj`, `acos.obj` and `atan.obj`.
const SIGNATURES: &[Signature] = &[
    Signature {
        intrinsic: "__CIsqrt",
        c_name: "sqrt",
        len: 0xba,
        intrinsic_at: 0x0,
        c_at: 0x14,
        anchor_at: 0x1d,
        anchor: &[0x52, 0x9b, 0xd9, 0x3c, 0x24, 0x8b, 0x44, 0x24, 0x0c, 0x74, 0x51, 0x66, 0x81, 0x3c, 0x24, 0x7f],
        wild: &[0x7..0xb, 0x19..0x1d, 0x31..0x35, 0x40..0x44, 0x47..0x4b, 0x52..0x56, 0x57..0x5b, 0x73..0x77, 0x92..0x96, 0x9d..0xa1, 0xa4..0xa8, 0xaf..0xb3, 0xb4..0xb8],
        hash: 0xb9e26c8099635694,
    },
    Signature {
        intrinsic: "__CIsin",
        c_name: "sin",
        len: 0x129,
        intrinsic_at: 0x40,
        c_at: 0x0,
        anchor_at: 0x46,
        anchor: &[0x00, 0x74, 0x32, 0x83, 0xec, 0x08, 0x0f, 0xae, 0x5c, 0x24, 0x04, 0x8b, 0x44, 0x24, 0x04, 0x25],
        wild: &[0x2..0x6, 0x3b..0x3f, 0x42..0x46, 0x77..0x7b, 0x82..0x86, 0x94..0x98, 0xa9..0xad, 0xb7..0xbb, 0xbe..0xc2, 0xc9..0xcd, 0xce..0xd2, 0xd4..0xd8, 0xe9..0xed, 0x101..0x105, 0x10c..0x110, 0x113..0x117, 0x11e..0x122, 0x123..0x127],
        hash: 0xc68784078269bed3,
    },
    Signature {
        intrinsic: "__CIcos",
        c_name: "cos",
        len: 0x129,
        intrinsic_at: 0x40,
        c_at: 0x0,
        anchor_at: 0x46,
        anchor: &[0x00, 0x74, 0x32, 0x83, 0xec, 0x08, 0x0f, 0xae, 0x5c, 0x24, 0x04, 0x8b, 0x44, 0x24, 0x04, 0x25],
        wild: &[0x2..0x6, 0x3b..0x3f, 0x42..0x46, 0x77..0x7b, 0x82..0x86, 0x94..0x98, 0xa9..0xad, 0xb7..0xbb, 0xbe..0xc2, 0xc9..0xcd, 0xce..0xd2, 0xd4..0xd8, 0xe9..0xed, 0x101..0x105, 0x10c..0x110, 0x113..0x117, 0x11e..0x122, 0x123..0x127],
        hash: 0xebc3f76ac94afd81,
    },
    Signature {
        intrinsic: "__CItan",
        c_name: "tan",
        len: 0x12d,
        intrinsic_at: 0x40,
        c_at: 0x0,
        anchor_at: 0x46,
        anchor: &[0x00, 0x74, 0x32, 0x83, 0xec, 0x08, 0x0f, 0xae, 0x5c, 0x24, 0x04, 0x8b, 0x44, 0x24, 0x04, 0x25],
        wild: &[0x2..0x6, 0x3b..0x3f, 0x42..0x46, 0x77..0x7b, 0x82..0x86, 0x94..0x98, 0xa9..0xad, 0xb9..0xbd, 0xc0..0xc4, 0xcb..0xcf, 0xd0..0xd4, 0xd6..0xda, 0xed..0xf1, 0x105..0x109, 0x110..0x114, 0x117..0x11b, 0x122..0x126, 0x127..0x12b],
        hash: 0x90b8f7bc0bee5ea9,
    },
    Signature {
        intrinsic: "__CIlog",
        c_name: "log",
        len: 0x150,
        intrinsic_at: 0x40,
        c_at: 0x0,
        anchor_at: 0x46,
        anchor: &[0x00, 0x74, 0x32, 0x83, 0xec, 0x08, 0x0f, 0xae, 0x5c, 0x24, 0x04, 0x8b, 0x44, 0x24, 0x04, 0x25],
        wild: &[0x2..0x6, 0x3b..0x3f, 0x42..0x46, 0x77..0x7b, 0x82..0x86, 0x94..0x98, 0xad..0xb1, 0xc7..0xcb, 0xce..0xd2, 0xd4..0xd8, 0xde..0xe2, 0x104..0x108, 0x110..0x114, 0x128..0x12c, 0x133..0x137, 0x13a..0x13e, 0x140..0x144, 0x14a..0x14e],
        hash: 0xc1903d9ad35617c8,
    },
    Signature {
        intrinsic: "__CIlog10",
        c_name: "log10",
        len: 0x150,
        intrinsic_at: 0x40,
        c_at: 0x0,
        anchor_at: 0x46,
        anchor: &[0x00, 0x74, 0x32, 0x83, 0xec, 0x08, 0x0f, 0xae, 0x5c, 0x24, 0x04, 0x8b, 0x44, 0x24, 0x04, 0x25],
        wild: &[0x2..0x6, 0x3b..0x3f, 0x42..0x46, 0x77..0x7b, 0x82..0x86, 0x94..0x98, 0xad..0xb1, 0xc7..0xcb, 0xce..0xd2, 0xd4..0xd8, 0xde..0xe2, 0x104..0x108, 0x110..0x114, 0x128..0x12c, 0x133..0x137, 0x13a..0x13e, 0x140..0x144, 0x14a..0x14e],
        hash: 0x18af3b5c16a876e5,
    },
    Signature {
        intrinsic: "__CIasin",
        c_name: "asin",
        len: 0x146,
        intrinsic_at: 0x40,
        c_at: 0x0,
        anchor_at: 0x46,
        anchor: &[0x00, 0x74, 0x32, 0x83, 0xec, 0x08, 0x0f, 0xae, 0x5c, 0x24, 0x04, 0x8b, 0x44, 0x24, 0x04, 0x25],
        wild: &[0x2..0x6, 0x3b..0x3f, 0x42..0x46, 0x77..0x7b, 0x82..0x86, 0x94..0x98, 0xa8..0xac, 0xc3..0xc7, 0xca..0xce, 0xd5..0xd9, 0xda..0xde, 0xfb..0xff, 0x106..0x10a, 0x11e..0x122, 0x129..0x12d, 0x130..0x134, 0x13b..0x13f, 0x140..0x144],
        hash: 0x2e1060522c5ecfc8,
    },
    Signature {
        intrinsic: "__CIacos",
        c_name: "acos",
        len: 0x146,
        intrinsic_at: 0x40,
        c_at: 0x0,
        anchor_at: 0x46,
        anchor: &[0x00, 0x74, 0x32, 0x83, 0xec, 0x08, 0x0f, 0xae, 0x5c, 0x24, 0x04, 0x8b, 0x44, 0x24, 0x04, 0x25],
        wild: &[0x2..0x6, 0x3b..0x3f, 0x42..0x46, 0x77..0x7b, 0x82..0x86, 0x94..0x98, 0xa8..0xac, 0xc5..0xc9, 0xcc..0xd0, 0xd7..0xdb, 0xdc..0xe0, 0x106..0x10a, 0x11e..0x122, 0x129..0x12d, 0x130..0x134, 0x13b..0x13f, 0x140..0x144],
        hash: 0x89b6ff803419acbb,
    },
    Signature {
        intrinsic: "__CIatan",
        c_name: "atan",
        len: 0x11a,
        intrinsic_at: 0x40,
        c_at: 0x0,
        anchor_at: 0x46,
        anchor: &[0x00, 0x74, 0x32, 0x83, 0xec, 0x08, 0x0f, 0xae, 0x5c, 0x24, 0x04, 0x8b, 0x44, 0x24, 0x04, 0x25],
        wild: &[0x2..0x6, 0x3b..0x3f, 0x42..0x46, 0x77..0x7b, 0x82..0x86, 0x94..0x98, 0xa9..0xad, 0xb3..0xb7, 0xba..0xbe, 0xc5..0xc9, 0xca..0xce, 0xcf..0xd3, 0xe7..0xeb, 0xfd..0x101, 0x104..0x108, 0x10f..0x113, 0x114..0x118],
        hash: 0xc9fd2d82548815b5,
    },
];

fn masked_hash(window: &[u8], wild: &[Range<usize>]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for (i, &b) in window.iter().enumerate() {
        let b = if wild.iter().any(|r| r.contains(&i)) { 0 } else { b };
        h ^= b as u64;
        h = h.wrapping_mul(0x0100_0000_01b3);
    }
    h
}

/// Every recognized object in `code` (mapped at `base`), as `(object start, signature index)`.
fn scan_bytes(code: &[u8], base: u64) -> Vec<(u64, usize)> {
    let mut hits = Vec::new();
    let mut anchors: Vec<&[u8]> = SIGNATURES.iter().map(|s| s.anchor).collect();
    anchors.sort_unstable();
    anchors.dedup();
    for anchor in anchors {
        for at in memmem::find_iter(code, anchor) {
            for (k, sig) in SIGNATURES.iter().enumerate().filter(|(_, s)| s.anchor == anchor) {
                let Some(start) = at.checked_sub(sig.anchor_at) else { continue };
                let Some(window) = code.get(start..start + sig.len) else { continue };
                if masked_hash(window, sig.wild) == sig.hash {
                    hits.push((base + start as u64, k));
                }
            }
        }
    }
    hits.sort_unstable();
    hits.dedup();
    hits
}

/// Bind the intrinsic at `addr` to the model and lock its signature,
/// `long double name(long double)`, so a call passes its one operand whatever
/// else the caller does with that value.
fn bind(out: &mut AnalysisOutput, ctx: &AnalysisCtx, addr: u64, name: &str) {
    out.models.push(ModelFact {
        func_addr: addr,
        model: kuna_decomp::kuna_x87intrinsic::MODEL_NAME,
        maybe_self: false,
    });
    use kuna_decomp::dtype::{type_metatype, TypeFactory};
    let Ok(ld) = ctx.arch.types().get_base(10, type_metatype::TYPE_FLOAT) else { return };
    out.prototypes_at.push((
        addr,
        kuna_decomp::fspec::PrototypePieces {
            name: name.to_string(),
            outtype: Some(std::rc::Rc::clone(&ld)),
            intypes: vec![ld],
            innames: vec!["x".to_string()],
            first_var_arg_slot: -1,
            output_storage: None,
            input_storage: Vec::new(),
            kuna_library_names: None,
        },
    ));
}

fn name(out: &mut AnalysisOutput, addr: u64, name: &str) {
    out.symbols.push(SymFact { addr, name: name.to_string(), kind: SymKind::Function });
    out.fid_names.push(FidMatch { addr, name: name.to_string() });
}

/// Bind the x87 intrinsics of an i386 image.
#[derive(Clone, Copy, Default)]
pub struct X87IntrinsicPass;

impl AnalysisPass for X87IntrinsicPass {
    fn phase(&self) -> Phase {
        Phase::P1
    }

    fn id(&self) -> &'static str {
        "x87intrinsics"
    }

    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        let mut out = AnalysisOutput::default();
        let file = ctx.file;
        if file.architecture() != object::Architecture::I386 {
            return out;
        }
        for (addr, n) in ctx.image.func_symbols() {
            if kuna_decomp::kuna_x87intrinsic::is_intrinsic(&n) {
                bind(&mut out, ctx, addr, &n);
            }
        }
        if file.format() != object::BinaryFormat::Pe {
            return out;
        }
        for (start, _, code) in crate::entry::executable_sections(file) {
            for (obj, k) in scan_bytes(&code, start) {
                let sig = &SIGNATURES[k];
                let entry = obj + sig.intrinsic_at as u64;
                name(&mut out, entry, sig.intrinsic);
                name(&mut out, obj + sig.c_at as u64, sig.c_name);
                bind(&mut out, ctx, entry, sig.intrinsic);
            }
        }
        out
    }
}

#[cfg(test)]
mod tests;

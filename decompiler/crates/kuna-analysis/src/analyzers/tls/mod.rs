//! (kuna `tlsnames`) The static TLS block of an x86 ELF executable: `PT_TLS`
//! and the `STT_TLS` symbols, for `kuna_decomp::kuna_tlsnames`.

use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, Phase};
use kuna_decomp::kuna_tlsnames::TlsLayout;
use object::{Object, ObjectSymbol, SymbolKind};

pub struct TlsNamesPass;

const PT_TLS: u32 = 7;

/// `(p_memsz, p_align)` of the `PT_TLS` program header.
fn pt_tls(bytes: &[u8]) -> Option<(u64, u64)> {
    let le = *bytes.get(5)? == 1;
    let wide = *bytes.get(4)? == 2;
    let field = |at: usize, n: usize| bytes.get(at..at.checked_add(n)?);
    let u16_at = |at: usize| field(at, 2).map(|b| if le { u16::from_le_bytes([b[0], b[1]]) } else { u16::from_be_bytes([b[0], b[1]]) });
    let u32_at = |at: usize| {
        field(at, 4).map(|b| {
            let a = [b[0], b[1], b[2], b[3]];
            if le { u32::from_le_bytes(a) } else { u32::from_be_bytes(a) }
        })
    };
    let u64_at = |at: usize| {
        field(at, 8).map(|b| {
            let a: [u8; 8] = b.try_into().expect("8 bytes");
            if le { u64::from_le_bytes(a) } else { u64::from_be_bytes(a) }
        })
    };
    let (phoff, phentsize, phnum) = if wide {
        (usize::try_from(u64_at(0x20)?).ok()?, u16_at(0x36)? as usize, u16_at(0x38)? as usize)
    } else {
        (u32_at(0x1c)? as usize, u16_at(0x2a)? as usize, u16_at(0x2c)? as usize)
    };
    (0..phnum).find_map(|i| {
        let ph = phoff.checked_add(i.checked_mul(phentsize)?)?;
        if u32_at(ph)? != PT_TLS {
            return None;
        }
        Some(if wide {
            (u64_at(ph.checked_add(0x28)?)?, u64_at(ph.checked_add(0x30)?)?)
        } else {
            (u64::from(u32_at(ph.checked_add(0x14)?)?), u64::from(u32_at(ph.checked_add(0x1c)?)?))
        })
    })
}

impl AnalysisPass for TlsNamesPass {
    fn phase(&self) -> Phase {
        Phase::P1
    }

    fn id(&self) -> &'static str {
        "tlsnames"
    }

    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        let mut out = AnalysisOutput::default();
        let x86 = matches!(ctx.file.architecture(), object::Architecture::X86_64 | object::Architecture::I386);
        if ctx.file.format() != object::BinaryFormat::Elf || !x86 || ctx.file.kind() == object::ObjectKind::Relocatable {
            return out;
        }
        let Some((memsz, align)) = pt_tls(ctx.bytes) else { return out };
        let symbols: Vec<(u64, u64, String)> = ctx
            .file
            .symbols()
            .filter(|s| s.kind() == SymbolKind::Tls && !s.is_undefined())
            .filter_map(|s| Some((s.address(), s.size(), s.name().ok()?.to_string())))
            .collect();
        out.tls_layout = TlsLayout::new(memsz, align, symbols);
        out
    }
}

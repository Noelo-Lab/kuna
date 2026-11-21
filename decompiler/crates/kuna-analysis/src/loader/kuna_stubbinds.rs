//! (kuna `stubbinds`) Name a Mach-O stub after the import its pointer slot is
//! bound to when the indirect-symbol table no longer says.
//!
//! A `__stubs` entry is named through `LC_DYSYMTAB`: its indirect-symbol entry
//! indexes the undefined `nlist` of the import it jumps to. A chained-fixups
//! link records the same import again in `LC_DYLD_CHAINED_FIXUPS`, and once
//! the image is stripped the `nlist` entry can go while the bind stays: `strip`
//! writes `INDIRECT_SYMBOL_LOCAL | INDIRECT_SYMBOL_ABS` over the stub's entry,
//! and the SwiftPM-built frameworks of App Store apps ship that way for most of
//! their Swift stdlib and runtime calls. The indirect-symbol walk then leaves
//! the stub `sub_<addr>`, and every `bl` to it renders an anonymous call,
//! though the bind on the slot the stub loads still carries the name.
//!
//! This pass decodes each such stub to the slot it jumps through (`adrp; ldr;
//! br` on arm64, the `adrp; add; ldr; braa` of an arm64e `__auth_stubs`
//! entry, `jmp [rip+disp]` on x86-64), reads the slot's addend-free chained
//! bind and names the stub after the import, one leading `_` stripped and
//! Swift and Itanium names demangled, as the indirect-symbol walk's names are.
//! A stub the walk already named, a stub that does not decode, and a slot with
//! no bind or a bind with an addend are left alone. Inert on any image with no
//! chained fixups.

use std::collections::HashSet;

use object::read::macho::{MachOFile, Section as _};
use object::read::{Object, ObjectSection};

use crate::loader::format::macho::{resolve_chained_fixups, ChainedFixups};
use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, Phase, SymFact, SymKind};

/// The stub-naming pass (gate id `stubbinds`).
#[derive(Clone, Copy, Default)]
pub struct StubBindsPass;

impl AnalysisPass for StubBindsPass {
    fn phase(&self) -> Phase {
        Phase::P1
    }

    fn id(&self) -> &'static str {
        "stubbinds"
    }

    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        let mut out = AnalysisOutput::default();
        for (addr, name) in stub_binds(ctx.file, ctx.bytes) {
            out.symbols.push(SymFact { addr, name, kind: SymKind::Function });
        }
        out
    }
}

/// `(stub, name)` for every `S_SYMBOL_STUBS` entry the indirect-symbol walk
/// left unnamed whose slot carries an addend-free chained bind, sorted by stub.
/// The chains are walked only when the walk left some stub unnamed.
pub(crate) fn stub_binds(file: &object::File, bytes: &[u8]) -> Vec<(u64, String)> {
    let object::File::MachO64(macho) = file else { return Vec::new() };
    let decode: fn(&[u8], u64) -> Option<u64> = match file.architecture() {
        object::Architecture::Aarch64 => a64_stub_slot,
        object::Architecture::X86_64 => x64_stub_slot,
        _ => return Vec::new(),
    };
    let sections: Vec<StubSection> = stub_sections(macho).collect();
    if sections.is_empty() {
        return Vec::new();
    }
    let named: HashSet<u64> =
        super::macho_stubs::resolve_macho_imports(file, bytes).into_iter().map(|i| i.addr).collect();
    let mut unnamed = Vec::new();
    for section in &sections {
        let (base, stride) = (section.address(), section.stride);
        let Ok(data) = section.data() else { continue };
        for (i, stub) in data.chunks_exact(stride).enumerate() {
            let Some(addr) = (i as u64).checked_mul(stride as u64).and_then(|o| base.checked_add(o)) else {
                break;
            };
            if !named.contains(&addr) {
                unnamed.push((addr, stub));
            }
        }
    }
    if unnamed.is_empty() {
        return Vec::new();
    }
    let fixups = resolve_chained_fixups(file, bytes);
    let mut out: Vec<(u64, String)> = unnamed
        .into_iter()
        .filter_map(|(addr, stub)| Some((addr, decode(stub, addr).and_then(|slot| bind_name(&fixups, slot))?)))
        .collect();
    out.sort_unstable();
    out
}

/// A stub section with its entry stride.
struct StubSection<'data, 'file> {
    section: object::read::macho::MachOSection64<'data, 'file>,
    stride: usize,
}

impl<'data, 'file> std::ops::Deref for StubSection<'data, 'file> {
    type Target = object::read::macho::MachOSection64<'data, 'file>;
    fn deref(&self) -> &Self::Target {
        &self.section
    }
}

/// Every `S_SYMBOL_STUBS` section whose `reserved2` states a usable stride.
fn stub_sections<'data, 'file>(
    macho: &'file MachOFile<'data, object::macho::MachHeader64<object::Endianness>>,
) -> impl Iterator<Item = StubSection<'data, 'file>> {
    let endian = macho.endian();
    macho.sections().filter_map(move |section| {
        let raw = section.macho_section();
        let stride = raw.reserved2(endian) as usize;
        (raw.section_type(endian) == object::macho::S_SYMBOL_STUBS && (4..=64).contains(&stride))
            .then_some(StubSection { section, stride })
    })
}

/// The import name of the addend-free bind on `slot`, demangled.
fn bind_name(fixups: &ChainedFixups, slot: u64) -> Option<String> {
    let bind = fixups.bind_at(slot).filter(|bind| bind.addend == 0)?;
    let name = std::str::from_utf8(&bind.name).ok()?;
    let name = name.strip_prefix('_').unwrap_or(name);
    if name.is_empty() {
        return None;
    }
    Some(crate::demangle::demangle_name(name).unwrap_or_else(|| name.to_string()))
}

/// The slot an arm64 stub at `pc` jumps through: fold `adrp`, `add` and the
/// last `ldr` into the register the final `br`/`braa`/`braaz` takes.
fn a64_stub_slot(stub: &[u8], pc: u64) -> Option<u64> {
    let mut regs = [None; 32];
    let mut slot = [None; 32];
    for (i, word) in stub.chunks_exact(4).enumerate() {
        let w = u32::from_le_bytes(word.try_into().ok()?);
        let pc = pc.checked_add(i as u64 * 4)?;
        let rd = (w & 31) as usize;
        let rn = ((w >> 5) & 31) as usize;
        if w & 0x9F00_0000 == 0x9000_0000 {
            let imm = ((w >> 3) & 0x001F_FFFC) | ((w >> 29) & 3);
            regs[rd] = Some((pc & !0xFFF).wrapping_add_signed(sign_extend(imm, 21) << 12));
        } else if w & 0xFF80_0000 == 0x9100_0000 {
            let imm = ((w >> 10) & 0xFFF) as u64;
            let shift = if w & (1 << 22) != 0 { 12 } else { 0 };
            regs[rd] = Some(regs[rn]?.wrapping_add(imm << shift));
        } else if w & 0xFFC0_0000 == 0xF940_0000 {
            slot[rd] = Some(regs[rn]?.wrapping_add((((w >> 10) & 0xFFF) as u64) * 8));
            regs[rd] = None;
        } else if w & 0xFFFF_FC1F == 0xD61F_0000
            || w & 0xFFFF_F800 == 0xD71F_0800
            || w & 0xFFFF_F81F == 0xD61F_081F
        {
            return slot[rn];
        } else {
            return None;
        }
    }
    None
}

/// The slot an x86-64 `jmp qword ptr [rip+disp32]` stub at `pc` jumps through.
fn x64_stub_slot(stub: &[u8], pc: u64) -> Option<u64> {
    let [0xFF, 0x25, d0, d1, d2, d3, ..] = *stub else { return None };
    let disp = i32::from_le_bytes([d0, d1, d2, d3]);
    pc.checked_add(6)?.checked_add_signed(disp.into())
}

fn sign_extend(v: u32, bits: u32) -> i64 {
    ((v as i64) << (64 - bits)) >> (64 - bits)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture(name: &str) -> Vec<u8> {
        std::fs::read(format!("{}/tests/fixtures/{name}", env!("CARGO_MANIFEST_DIR"))).expect("read fixture")
    }

    /// `macho_stubs_stripped_arm64`'s one stub has an `INDIRECT_SYMBOL_LOCAL |
    /// ABS` entry; its slot's chained bind names it.
    #[test]
    fn a_stripped_stub_is_named_from_its_slots_bind() {
        let bytes = fixture("macho_stubs_stripped_arm64");
        let file = object::File::parse(bytes.as_slice()).expect("parse Mach-O");
        assert_eq!(stub_binds(&file, &bytes), vec![(0x100000520, "__chkstk_darwin".to_string())]);
    }

    /// The same image before stripping: the indirect-symbol walk names the
    /// stub, so the pass adds nothing.
    #[test]
    fn a_stub_the_indirect_symbol_walk_names_is_left_alone() {
        let bytes = fixture("chkstkdarwin_macho_arm64");
        let file = object::File::parse(bytes.as_slice()).expect("parse Mach-O");
        assert!(stub_binds(&file, &bytes).is_empty());
    }

    #[test]
    fn stub_decoders() {
        // adrp x16, #0x4000; ldr x16, [x16]; br x16  at 0x100000520
        let a64 = [0x30, 0x00, 0x00, 0x90, 0x10, 0x02, 0x40, 0xf9, 0x00, 0x02, 0x1f, 0xd6];
        assert_eq!(a64_stub_slot(&a64, 0x100000520), Some(0x100004000));
        // adrp x17, #0x4000; add x17, x17, #0x18; ldr x16, [x17]; braa x16, x17
        let auth = [0x31, 0x00, 0x00, 0x90, 0x31, 0x62, 0x00, 0x91, 0x30, 0x02, 0x40, 0xf9, 0x11, 0x0a, 0x1f, 0xd7];
        assert_eq!(a64_stub_slot(&auth, 0x100000520), Some(0x100004018));
        assert_eq!(a64_stub_slot(&a64[..8], 0x100000520), None);
        assert_eq!(x64_stub_slot(&[0xff, 0x25, 0xfa, 0x0f, 0x00, 0x00], 0x100000f00), Some(0x100001f00));
        assert_eq!(x64_stub_slot(&[0xe9, 0, 0, 0, 0, 0], 0x100000f00), None);
    }
}

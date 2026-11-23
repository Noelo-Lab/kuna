//! (kuna `flirt make`) FLIRT patterns from relocatable objects.
//!
//! A pattern describes one function of an object file the way a `.pat` line
//! does: its bytes with every byte a relocation rewrites left variable, the
//! public names at its start, and the names it refers to with their offsets.
//! Bytes a linker may rewrite around a relaxable x86 relocation (a GOT load
//! turned into a `lea`, a TLS sequence) are variable too. Functions that only
//! have a local symbol, compiler-generated names and functions with fewer than
//! [`MIN_FIXED`] fixed bytes in the leading [`PATTERN_LEN`] yield no pattern.
//!
//! [`write_pat`] renders the patterns as `.pat` text; the `.sig` tree is
//! [`super::kuna_flirtsig`].

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;

use object::read::{Object, ObjectSection, ObjectSymbol, RelocationFlags, RelocationTarget};
use object::{Architecture, SectionKind, SymbolKind};

use flirt_sig::crc16;

/// Leading bytes of a function that form its tree pattern.
pub const PATTERN_LEN: usize = 32;
/// Fewest fixed bytes the leading pattern needs.
pub const MIN_FIXED: usize = 4;
/// Most bytes after the leading pattern the CRC covers.
pub const MAX_CRC_LEN: usize = 255;

/// One function of an object file.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FuncPattern {
    /// The function's bytes; `None` is a byte a relocation rewrites.
    pub bytes: Vec<Option<u8>>,
    /// The global names at the function's start, the preferred one first.
    pub publics: Vec<String>,
    /// The global names the function refers to, by offset.
    pub refs: Vec<(u32, String)>,
}

/// What the objects of one library have in common.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Target {
    pub architecture: Architecture,
    pub format: object::BinaryFormat,
    pub little_endian: bool,
}

/// The patterns of one object, with the names it defines as data.
pub struct ObjectPatterns {
    pub target: Target,
    pub patterns: Vec<FuncPattern>,
    pub data_names: BTreeSet<String>,
    /// Functions left out for having too few fixed bytes.
    pub short: usize,
}

impl FuncPattern {
    /// The leading [`PATTERN_LEN`] bytes, padded with variable bytes.
    pub fn head(&self) -> [Option<u8>; PATTERN_LEN] {
        let mut head = [None; PATTERN_LEN];
        for (h, b) in head.iter_mut().zip(&self.bytes) {
            *h = *b;
        }
        head
    }

    /// The fixed bytes after the leading pattern that the CRC covers.
    pub fn crc_bytes(&self) -> Vec<u8> {
        self.bytes.iter().skip(PATTERN_LEN).take(MAX_CRC_LEN).map_while(|b| *b).collect()
    }

    /// The CRC-16 of [`Self::crc_bytes`].
    pub fn crc(&self) -> u16 {
        crc16(&self.crc_bytes())
    }

    /// The bytes after the CRC region.
    pub fn tail(&self) -> &[Option<u8>] {
        self.bytes.get(PATTERN_LEN + self.crc_bytes().len()..).unwrap_or(&[])
    }
}

/// Read the patterns of one relocatable object.
pub fn patterns_from_object(data: &[u8]) -> Result<ObjectPatterns, String> {
    let file = object::File::parse(data).map_err(|e| format!("not an object file: {e}"))?;
    let architecture = file.architecture();
    let target = Target { architecture, format: file.format(), little_endian: file.is_little_endian() };
    let thumb_bit = u64::from(architecture == Architecture::Arm);
    let mut data_names = BTreeSet::new();
    let mut starts: BTreeMap<(usize, u64), Vec<(bool, String)>> = BTreeMap::new();
    let mut bounds: BTreeMap<usize, BTreeSet<u64>> = BTreeMap::new();
    let mut sizes: BTreeMap<(usize, u64), u64> = BTreeMap::new();
    for sym in file.symbols() {
        let (Some(index), Ok(name)) = (sym.section_index(), sym.name()) else { continue };
        if !sym.is_definition() {
            continue;
        }
        if sym.kind() == SymbolKind::Data && sym.is_global() {
            data_names.insert(name.to_string());
        }
        if sym.kind() != SymbolKind::Text {
            continue;
        }
        let addr = sym.address() & !thumb_bit;
        bounds.entry(index.0).or_default().insert(addr);
        if sym.size() > 0 {
            let size = sizes.entry((index.0, addr)).or_insert(0);
            *size = (*size).max(sym.size());
        }
        if sym.is_global() && !is_generated(name) {
            starts.entry((index.0, addr)).or_default().push((sym.is_weak(), name.to_string()));
        }
    }
    let mut out = ObjectPatterns { target, patterns: Vec::new(), data_names, short: 0 };
    for section in file.sections().filter(|s| s.kind() == SectionKind::Text) {
        let Ok(code) = section.data() else { continue };
        let index = section.index().0;
        let base = section.address();
        let relocs = section_relocs(&file, &section, code.len());
        for (&(_, addr), names) in starts.range((index, 0)..=(index, u64::MAX)) {
            let Some(start) = addr.checked_sub(base).and_then(|o| usize::try_from(o).ok()) else { continue };
            let next = bounds[&index].range(addr + 1..).next().and_then(|n| usize::try_from(n - base).ok());
            let sized = sizes.get(&(index, addr)).and_then(|s| start.checked_add(usize::try_from(*s).ok()?));
            let end = sized.or(next).unwrap_or(code.len()).min(code.len());
            if start >= end {
                continue;
            }
            let mut bytes: Vec<Option<u8>> = code[start..end].iter().map(|&b| Some(b)).collect();
            let mut refs = Vec::new();
            for r in relocs.range(start.saturating_sub(MAX_RELAX)..end).map(|(_, r)| r) {
                for b in bytes.iter_mut().take(r.end.saturating_sub(start)).skip(r.start.saturating_sub(start)) {
                    *b = None;
                }
                if let (Some(name), true) = (&r.name, r.offset >= start) {
                    refs.push(((r.offset - start) as u32, name.clone()));
                }
            }
            if bytes.iter().take(PATTERN_LEN).flatten().count() < MIN_FIXED {
                out.short += 1;
                continue;
            }
            let mut names = names.clone();
            names.sort_by_key(|(weak, n)| (*weak, n.len() - n.trim_start_matches('_').len(), n.clone()));
            names.dedup();
            out.patterns.push(FuncPattern { bytes, publics: names.into_iter().map(|(_, n)| n).collect(), refs });
        }
    }
    Ok(out)
}

/// Most bytes before a relocated field that a relaxation rewrites.
const MAX_RELAX: usize = 4;

/// A relocation of a code section: the section bytes it makes variable and the
/// global name it refers to.
struct Reloc {
    offset: usize,
    start: usize,
    end: usize,
    name: Option<String>,
}

fn section_relocs<'a>(
    file: &object::File<'a>,
    section: &object::Section<'a, '_>,
    len: usize,
) -> BTreeMap<usize, Reloc> {
    let fixed_width = matches!(
        file.architecture(),
        Architecture::Aarch64
            | Architecture::Mips
            | Architecture::Mips64
            | Architecture::PowerPc
            | Architecture::PowerPc64
            | Architecture::Sparc
            | Architecture::Sparc64
            | Architecture::LoongArch64
    );
    let x86 = matches!(file.architecture(), Architecture::X86_64 | Architecture::I386);
    let mut out = BTreeMap::new();
    for (offset, reloc) in section.relocations() {
        let Ok(offset) = usize::try_from(offset) else { continue };
        let field = match usize::from(reloc.size()).div_ceil(8) {
            0 => 4,
            n if x86 => n,
            n => n.max(4),
        };
        let (mut start, mut end) = (offset, offset.saturating_add(field));
        if fixed_width {
            start &= !3;
            end = end.max(start + 4);
        }
        if let (true, RelocationFlags::Elf { r_type }) = (x86, reloc.flags()) {
            let (before, after) = x86_relaxation(file.architecture(), r_type);
            start = start.saturating_sub(before);
            end = end.saturating_add(after);
        }
        let name = match reloc.target() {
            RelocationTarget::Symbol(index) => file.symbol_by_index(index).ok().and_then(|s| {
                let name = s.name().ok()?;
                let named = s.kind() != SymbolKind::Section && s.is_global() && !is_generated(name);
                named.then(|| name.to_string())
            }),
            _ => None,
        };
        out.insert(offset, Reloc { offset, start, end: end.min(len), name });
    }
    out
}

/// The bytes before and after a relocated x86 field that a linker rewrites
/// when it relaxes the relocation: the opcode and ModRM of a GOT load, and the
/// whole general-dynamic and local-dynamic TLS call sequences.
fn x86_relaxation(architecture: Architecture, r_type: u32) -> (usize, usize) {
    use object::elf::*;
    match (architecture, r_type) {
        (Architecture::X86_64, R_X86_64_GOTPCRELX | R_X86_64_REX_GOTPCRELX | R_X86_64_GOTTPOFF) => (3, 0),
        (Architecture::X86_64, R_X86_64_GOTPC32_TLSDESC) => (3, 0),
        (Architecture::X86_64, R_X86_64_TLSGD) => (4, 8),
        (Architecture::X86_64, R_X86_64_TLSLD) => (3, 6),
        (Architecture::I386, R_386_GOT32X | R_386_TLS_GOTIE | R_386_TLS_IE) => (2, 0),
        (Architecture::I386, R_386_TLS_GD | R_386_TLS_LDM) => (3, 5),
        _ => (0, 0),
    }
}

/// Whether a symbol name was made up by a compiler or assembler rather than
/// written in the source: local labels, mapping symbols, outlined and cloned
/// function parts, PC thunks and static-initialization stubs.
pub fn is_generated(name: &str) -> bool {
    const PREFIXES: [&str; 7] =
        [".L", "$", "__x86.get_pc_thunk.", "_GLOBAL__", "__cxx_global_", "__tls_init", "OUTLINED_FUNCTION_"];
    const PARTS: [&str; 6] = [".cold", ".part.", ".isra.", ".constprop.", ".lto_priv.", ".llvm."];
    name.is_empty()
        || name.bytes().any(|b| b <= 0x20 || b == 0x7f)
        || PREFIXES.iter().any(|p| name.starts_with(p))
        || PARTS.iter().any(|p| name.contains(p))
}

/// Render patterns as `.pat` text: one line per function (64 hex digits of
/// leading bytes with `..` for a variable byte, the CRC length and CRC, the
/// function length, `:offset name` publics, `^offset name` references and the
/// bytes after the CRC region), closed by a `---` line.
pub fn write_pat(patterns: &[FuncPattern]) -> String {
    let mut out = String::new();
    let hex = |out: &mut String, bytes: &[Option<u8>]| {
        for b in bytes {
            match b {
                Some(b) => write!(out, "{b:02X}").unwrap(),
                None => out.push_str(".."),
            }
        }
    };
    for p in patterns {
        hex(&mut out, &p.head());
        write!(out, " {:02X} {:04X} {:04X}", p.crc_bytes().len(), p.crc(), p.bytes.len()).unwrap();
        for name in &p.publics {
            write!(out, " :0000 {name}").unwrap();
        }
        for (offset, name) in &p.refs {
            write!(out, " ^{offset:04X} {name}").unwrap();
        }
        if !p.tail().is_empty() {
            out.push(' ');
            hex(&mut out, p.tail());
        }
        out.push('\n');
    }
    out.push_str("---\n");
    out
}

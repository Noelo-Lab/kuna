//! The Mach-O [`ObjectFormat`] — the Apple object/exe arm of the loader seam.
//!
//! This wires Mach-O through the seam so a Mach-O image **parses, maps its
//! sections, selects the right SLEIGH spec** (`gcc` for x86-64 — macOS x86-64
//! follows the System V AMD64 ABI, the same cspec Ghidra labels `gcc`;
//! `default` for arm64), and **names its imports** (PR-7): a `bl`/`callq`
//! targeting a `__TEXT,__stubs` entry renders `printf(` rather than
//! `sub_<addr>`, via the `LC_DYSYMTAB` indirect-symbol walk in
//! [`crate::s1_loader::macho_stubs`] (design §3.3).
//!
//! ## Section flags
//!
//! Mach-O does **not** carry per-section R/W/X permission in the section header
//! `flags` field — those permissions live in the enclosing segment's
//! `initprot`/`maxprot`. The section `flags` low byte is the section *type*
//! (`S_ZEROFILL`/`S_SYMBOL_STUBS`/…) and the high bits are *attributes*
//! (`S_ATTR_PURE_INSTRUCTIONS`/…). So [`MachOFormat::section_bits`] keys
//! primarily off the neutral [`SectionKind`] (which `object` derives from
//! segment+section), using the section-type/attributes for the NOLOAD/CODE
//! refinements. This is the faithful Mach-O analog of the BFD `SEC_*` mapping.
//!
//! All object-format magics (Mach-O's `0xfeedfac*` / fat `0xcafebabe` included)
//! are admitted by the engine dispatch unconditionally; the XML/datatest corpus
//! never carries a Mach-O magic, so the ELF/XML oracles are untouched.

use object::macho::{S_ATTR_PURE_INSTRUCTIONS, S_GB_ZEROFILL, S_ZEROFILL, SECTION_TYPE};
use object::macho::{CPU_SUBTYPE_ARM64E, CPU_SUBTYPE_MASK};
use object::read::macho::MachHeader;
use object::{Architecture, FileKind, SectionFlags, SectionKind};

use kuna_sleigh::loadimage::section_flags;

use super::{FormatKind, ImportSym, ObjectFormat};

/// The `LC_DYLD_CHAINED_FIXUPS` resolver (design §5 PR-O0). Lives beside the
/// Mach-O format impl because it is loader infra shared by the ObjC walk (and any
/// future chained-fixup consumer), keyed off the same typed Mach-O re-parse.
pub mod chained;
pub use chained::{resolve_chained_fixups, ChainedFixups};

/// The environment-variable gate for arm64e Apple-Silicon spec selection
/// (design §3.7). Off (unset/empty) by default — an arm64e Mach-O then loads
/// with the generic `AARCH64:LE:64:v8A` spec exactly as any arm64. Turned on by
/// the `--option macho-arm64e on` CLI flag (which exports this env var onto the
/// `decomp_dbg` subprocess), making the gate effective at *load* time — before
/// any console `option` command is processed. Read live so a test can toggle it
/// in-process.
pub const MACHO_ARM64E_ENV: &str = "KUNA_MACHO_ARM64E";

/// Whether the arm64e Apple-Silicon spec gate is enabled ([`MACHO_ARM64E_ENV`]).
fn arm64e_gate_enabled() -> bool {
    std::env::var_os(MACHO_ARM64E_ENV).map(|v| !v.is_empty()).unwrap_or(false)
}

/// Whether a thin Mach-O's header reports the arm64e CPU subtype
/// (`CPU_SUBTYPE_ARM64E`). Pure & total: a non-Mach-O / fat / unparsable input
/// (or any non-arm64e subtype) is `false`. The high capability byte
/// (`CPU_SUBTYPE_MASK`) is stripped before comparing, so a pointer-auth-capable
/// arm64e (the `CPU_SUBTYPE_PTRAUTH_ABI` high bits set) still matches.
pub fn is_arm64e(bytes: &[u8]) -> bool {
    fn subtype<Mach: MachHeader>(bytes: &[u8]) -> Option<u32> {
        let header = Mach::parse(bytes, 0).ok()?;
        let endian = header.endian().ok()?;
        Some(header.cpusubtype(endian) & !CPU_SUBTYPE_MASK)
    }
    let sub = match FileKind::parse(bytes) {
        Ok(FileKind::MachO64) => subtype::<object::macho::MachHeader64<object::Endianness>>(bytes),
        Ok(FileKind::MachO32) => subtype::<object::macho::MachHeader32<object::Endianness>>(bytes),
        // Fat input is peeled to a thin slice before this runs; a leftover fat
        // here is not an arm64e thin Mach-O.
        _ => None,
    };
    sub == Some(CPU_SUBTYPE_ARM64E)
}

/// The arm64e Apple-Silicon SLEIGH language id (`AARCH64:LE:64:AppleSilicon` +
/// the `default` compiler) when — and only when — the format is Mach-O, the arch
/// is AArch64, the `macho-arm64e` gate is on, and the image is an arm64e thin
/// Mach-O (design §3.7). Returns `None` (so the generic `v8A` id wins) in every
/// other case, which is the default-off behavior. The id is the one the vendored
/// `AppleSilicon.ldefs` declares — verified resolvable by the console gate.
pub fn apple_silicon_id(
    fmt: &dyn ObjectFormat,
    arch: Architecture,
    bytes: &[u8],
) -> Option<String> {
    if fmt.kind() != FormatKind::MachO
        || arch != Architecture::Aarch64
        || !arm64e_gate_enabled()
        || !is_arm64e(bytes)
    {
        return None;
    }
    Some("AARCH64:LE:64:AppleSilicon:default".to_string())
}

/// The Mach-O (Apple) object format.
pub struct MachOFormat;

impl ObjectFormat for MachOFormat {
    fn kind(&self) -> FormatKind {
        FormatKind::MachO
    }

    fn compiler_model(&self, arch: Architecture) -> Option<&'static str> {
        // macOS x86-64 follows the System V AMD64 ABI — the *same* cspec Ghidra
        // labels `gcc` for x86-64 (design §2.1 footnote ¹): so this returns
        // `gcc`, NOT an invented `macho` token. arm64 uses the AArch64 `default`
        // cspec (`default`, not `windows`). The model must always be one the
        // vendored `.ldefs` declares.
        match arch {
            Architecture::X86_64 | Architecture::I386 => Some("gcc"),
            // Aarch64 / Arm and anything else: the arch default cspec.
            _ => Some("default"),
        }
    }

    fn section_bits(&self, kind: SectionKind, flags: SectionFlags) -> u32 {
        let mflags = match flags {
            SectionFlags::MachO { flags } => flags,
            _ => 0,
        };
        let sec_type = mflags & SECTION_TYPE;
        let pure_instr = mflags & S_ATTR_PURE_INSTRUCTIONS != 0;
        // Zero-fill section types are the `.bss` analog (no file content).
        let zerofill = sec_type == S_ZEROFILL || sec_type == S_GB_ZEROFILL;

        let mut out = 0u32;
        // NOLOAD: zero-fill (uninitialized) sections, or whatever the neutral
        // `SectionKind` already classifies as uninitialized.
        if zerofill || matches!(kind, SectionKind::UninitializedData) {
            out |= section_flags::NOLOAD;
        }
        // READONLY: Mach-O section flags carry no per-section write bit (it lives
        // in the segment initprot). Use the neutral `SectionKind`: a read-only
        // data / text / string section is readonly; writable data is not.
        // `__cstring` is `SectionKind::ReadOnlyString` — marking it READONLY is
        // what lets the printer's constant-string route read a `char *` arg's
        // bytes and render `printf("%d\n", …)` (PR-10).
        if matches!(
            kind,
            SectionKind::ReadOnlyData | SectionKind::ReadOnlyString | SectionKind::Text
        ) {
            out |= section_flags::READONLY;
        }
        // CODE: a `__text`-kind section or a pure-instructions section.
        if pure_instr || matches!(kind, SectionKind::Text) {
            out |= section_flags::CODE;
        }
        if matches!(
            kind,
            SectionKind::Data | SectionKind::ReadOnlyData | SectionKind::ReadOnlyString
        ) {
            out |= section_flags::DATA;
        }
        out
    }

    fn resolve_imports(&self, file: &object::File, bytes: &[u8]) -> Vec<ImportSym> {
        // Mach-O import naming: walk the `LC_DYSYMTAB` indirect-symbol table,
        // naming each `__stubs` entry a `bl`/`callq` targets directly + each
        // symbol-pointer slot (the GOT analog) a folded indirect call resolves,
        // and register the exports — all in `s1_loader/macho_stubs.rs` (design
        // §3.3). Arch-independent (section metadata, no instruction decode).
        // Pure & total: a non-Mach-O / no-LC_DYSYMTAB / unparsable layout yields
        // an empty `Vec`.
        crate::s1_loader::macho_stubs::resolve_macho_imports(file, bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `MachOFormat::compiler_model` is `gcc` for x86-64 (System V AMD64, the
    /// macOS ABI) and `default` for arm64 — never an invented token.
    #[test]
    fn macho_compiler_model_per_arch() {
        let f = MachOFormat;
        assert_eq!(f.compiler_model(Architecture::X86_64), Some("gcc"), "x86-64 macOS is SysV → gcc");
        assert_eq!(f.compiler_model(Architecture::I386), Some("gcc"));
        assert_eq!(f.compiler_model(Architecture::Aarch64), Some("default"), "arm64 macOS → default");
    }

    /// A `__text` Mach-O section (`SectionKind::Text`, `S_ATTR_PURE_INSTRUCTIONS`)
    /// is CODE | READONLY; a zero-fill section is NOLOAD.
    #[test]
    fn macho_section_bits_text_and_bss() {
        let f = MachOFormat;
        let bits = f.section_bits(
            SectionKind::Text,
            SectionFlags::MachO { flags: S_ATTR_PURE_INSTRUCTIONS },
        );
        assert!(bits & section_flags::CODE != 0, "__text is CODE");
        assert!(bits & section_flags::READONLY != 0, "__text is READONLY");

        let bss = f.section_bits(
            SectionKind::UninitializedData,
            SectionFlags::MachO { flags: S_ZEROFILL },
        );
        assert!(bss & section_flags::NOLOAD != 0, "zerofill section is NOLOAD");
    }

    /// A minimal `mach_header_64` buffer with the given cputype/cpusubtype, the
    /// 32-byte fixed header `MachHeader64::parse` reads (the load commands are not
    /// needed for the cpusubtype check). Little-endian on-disk (`MH_MAGIC_64`).
    fn macho64_header(cputype: u32, cpusubtype: u32) -> Vec<u8> {
        let mut b = Vec::new();
        b.extend_from_slice(&0xfeed_facfu32.to_le_bytes()); // MH_MAGIC_64
        b.extend_from_slice(&cputype.to_le_bytes());
        b.extend_from_slice(&cpusubtype.to_le_bytes());
        b.extend_from_slice(&2u32.to_le_bytes()); // filetype = MH_EXECUTE
        b.extend_from_slice(&0u32.to_le_bytes()); // ncmds
        b.extend_from_slice(&0u32.to_le_bytes()); // sizeofcmds
        b.extend_from_slice(&0u32.to_le_bytes()); // flags
        b.extend_from_slice(&0u32.to_le_bytes()); // reserved (64-bit only)
        b
    }

    /// `is_arm64e` is `true` only for an arm64 Mach-O whose cpusubtype is
    /// `CPU_SUBTYPE_ARM64E` (with the high capability byte stripped); a plain
    /// arm64 (`ARM64_ALL`/`ARM64_V8`) or x86-64 is `false`.
    #[test]
    fn is_arm64e_detects_only_arm64e_subtype() {
        use object::macho::{CPU_SUBTYPE_ARM64_ALL, CPU_TYPE_ARM64, CPU_TYPE_X86_64};
        // arm64e — the high capability byte (CPU_SUBTYPE_PTRAUTH_ABI) set or not.
        assert!(is_arm64e(&macho64_header(CPU_TYPE_ARM64 as u32, CPU_SUBTYPE_ARM64E)));
        assert!(
            is_arm64e(&macho64_header(CPU_TYPE_ARM64 as u32, CPU_SUBTYPE_ARM64E | 0x8000_0000)),
            "the capability high byte must be masked off before comparing"
        );
        // plain arm64 — not arm64e.
        assert!(!is_arm64e(&macho64_header(CPU_TYPE_ARM64 as u32, CPU_SUBTYPE_ARM64_ALL)));
        // x86-64 — never arm64e.
        assert!(!is_arm64e(&macho64_header(CPU_TYPE_X86_64 as u32, 3)));
        // non-Mach-O — false (no panic).
        assert!(!is_arm64e(&[0x7f, b'E', b'L', b'F']));
        assert!(!is_arm64e(&[]));
    }

    /// `apple_silicon_id` returns the `AARCH64:LE:64:AppleSilicon` id ONLY when:
    /// Mach-O format + AArch64 arch + the gate on + an arm64e image. Every other
    /// combination is `None` (so the generic v8A id wins) — the default-off path.
    #[test]
    fn apple_silicon_id_gated_and_arm64e_only() {
        use object::macho::{CPU_SUBTYPE_ARM64_ALL, CPU_TYPE_ARM64};

        let macho = MachOFormat;
        let arm64e = macho64_header(CPU_TYPE_ARM64 as u32, CPU_SUBTYPE_ARM64E);
        let arm64 = macho64_header(CPU_TYPE_ARM64 as u32, CPU_SUBTYPE_ARM64_ALL);

        // The env-gate toggling must be serialized: this process-global var is
        // also read by the console gate; keep the on-window tight.
        std::env::remove_var(MACHO_ARM64E_ENV);
        // Gate OFF (default): even an arm64e Mach-O yields None (generic v8A wins).
        assert_eq!(apple_silicon_id(&macho, Architecture::Aarch64, &arm64e), None);

        std::env::set_var(MACHO_ARM64E_ENV, "1");
        // Gate ON + arm64e Mach-O + AArch64 → the AppleSilicon id.
        assert_eq!(
            apple_silicon_id(&macho, Architecture::Aarch64, &arm64e).as_deref(),
            Some("AARCH64:LE:64:AppleSilicon:default")
        );
        // Gate ON but a *plain* arm64 (not arm64e) → None.
        assert_eq!(apple_silicon_id(&macho, Architecture::Aarch64, &arm64), None);
        // Gate ON, arm64e bytes, but wrong arch (x86-64) → None.
        assert_eq!(apple_silicon_id(&macho, Architecture::X86_64, &arm64e), None);
        // Gate ON, arm64e bytes, but non-Mach-O format → None.
        assert_eq!(
            apple_silicon_id(&crate::s1_loader::format::elf::ElfFormat, Architecture::Aarch64, &arm64e),
            None
        );
        std::env::remove_var(MACHO_ARM64E_ENV);
    }
}

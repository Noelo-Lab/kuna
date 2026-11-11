//! Runtime and packer recognition for orientation: which wrapper or toolchain
//! produced an image, and what to do next when native decompilation of it is the
//! wrong tool (a managed assembly, a bundled interpreter, a packed stub).
//!
//! Detection reads raw bytes and a lenient PE header walk that does not insist
//! on the `MZ` magic, so a file whose DOS header was damaged on purpose is still
//! recognized even though it cannot be loaded. `functions --summary` reports the
//! result as `summary.runtime`; every other binary subcommand prints the
//! actionable entries as one `note:` line each on stderr (see `docs/cli.md`).

use kuna_analysis::analyzers::sourcelang::{effective_compiler_bytes, Compiler};
use object::{Object, ObjectSection};
use regex::bytes::Regex;
use std::sync::OnceLock;

use crate::jsonfmt::Json;

/// One recognized runtime, packer or toolchain.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeHint {
    pub id: &'static str,
    pub version: Option<String>,
    pub hint: String,
    /// `true` when native decompilation is not the whole answer and the hint says
    /// what to run instead; only these are echoed to stderr.
    pub actionable: bool,
}

const PYI_MAGIC: &[u8] = b"MEI\x0c\x0b\x0a\x0b\x0e";

#[derive(Clone, Copy, PartialEq, Eq)]
enum Marker {
    PyInstaller,
    AutoIt,
    TwinBasic,
    Vb6,
    Nuitka,
}

const MARKERS: &[(&[u8], Marker)] = &[
    (PYI_MAGIC, Marker::PyInstaller),
    (b"AU3!EA06", Marker::AutoIt),
    (b"AU3!EA05", Marker::AutoIt),
    (b"twinBASIC", Marker::TwinBasic),
    (b"MSVBVM60.DLL", Marker::Vb6),
    (b"msvbvm60.dll", Marker::Vb6),
    (b"MSVBVM60.dll", Marker::Vb6),
    (b"NUITKA_ONEFILE_PARENT", Marker::Nuitka),
    (b"__nuitka_binary_dir", Marker::Nuitka),
];

fn marker_regex() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        let alternatives: Vec<String> = MARKERS
            .iter()
            .map(|(m, _)| m.iter().map(|b| format!("\\x{b:02x}")).collect())
            .collect();
        Regex::new(&format!("(?-u){}", alternatives.join("|"))).expect("marker regex")
    })
}

/// What a lenient PE header walk found.
struct PeView {
    section_names: Vec<Vec<u8>>,
    clr: bool,
    debug_header_export: bool,
}

fn u16_at(b: &[u8], off: usize) -> Option<u16> {
    Some(u16::from_le_bytes(b.get(off..off + 2)?.try_into().ok()?))
}

fn u32_at(b: &[u8], off: usize) -> Option<u32> {
    Some(u32::from_le_bytes(b.get(off..off + 4)?.try_into().ok()?))
}

/// Walk the PE headers without checking the DOS `MZ` magic.
fn pe_view(b: &[u8]) -> Option<PeView> {
    let lfanew = u32_at(b, 0x3c)? as usize;
    if b.get(lfanew..lfanew.checked_add(4)?)? != b"PE\0\0" {
        return None;
    }
    let coff = lfanew + 4;
    let nsections = u16_at(b, coff + 2)? as usize;
    let opt_size = u16_at(b, coff + 16)? as usize;
    let opt = coff + 20;
    let (ndirs_at, dirs_at) = match u16_at(b, opt)? {
        0x10b => (opt + 92, opt + 96),
        0x20b => (opt + 108, opt + 112),
        _ => return None,
    };
    let ndirs = u32_at(b, ndirs_at).unwrap_or(0);
    let clr = ndirs > 14
        && u32_at(b, dirs_at + 14 * 8).unwrap_or(0) != 0
        && u32_at(b, dirs_at + 14 * 8 + 4).unwrap_or(0) != 0;
    let table = opt + opt_size;
    let nsections = nsections.min(96);
    let section_names = (0..nsections)
        .map_while(|i| b.get(table + i * 40..table + i * 40 + 8))
        .map(|n| n.iter().copied().take_while(|&c| c != 0).collect())
        .collect();
    let export_rva = (ndirs > 0).then(|| u32_at(b, dirs_at)).flatten().filter(|&rva| rva != 0);
    let debug_header_export = export_rva
        .and_then(|rva| pe_exports(b, table, nsections, rva, b"DotNetRuntimeDebugHeader"))
        .unwrap_or(false);
    Some(PeView { section_names, clr, debug_header_export })
}

/// The file offset of `rva` under the section table at `table`.
fn pe_offset(b: &[u8], table: usize, nsections: usize, rva: u32) -> Option<usize> {
    (0..nsections).find_map(|i| {
        let s = table + i * 40;
        let delta = rva.checked_sub(u32_at(b, s + 12)?)?;
        let at = (u32_at(b, s + 20)? as usize).checked_add(delta as usize)?;
        (delta < u32_at(b, s + 16)?).then_some(at)
    })
}

/// Whether the export directory at `export_rva` names `name`. The name pointer
/// table of a PE is sorted, so this is a binary search over it.
fn pe_exports(b: &[u8], table: usize, nsections: usize, export_rva: u32, name: &[u8]) -> Option<bool> {
    let offset = |rva: u32| pe_offset(b, table, nsections, rva);
    let dir = offset(export_rva)?;
    let names = offset(u32_at(b, dir.checked_add(32)?)?)?;
    let (mut lo, mut hi) = (0usize, u32_at(b, dir.checked_add(24)?)? as usize);
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        let tail = b.get(offset(u32_at(b, names.checked_add(mid.checked_mul(4)?)?)?)?..)?;
        let tail = &tail[..tail.len().min(name.len() + 1)];
        let entry = tail.split(|&c| c == 0).next().unwrap_or(tail);
        match entry.cmp(name) {
            std::cmp::Ordering::Less => lo = mid + 1,
            std::cmp::Ordering::Greater => hi = mid,
            std::cmp::Ordering::Equal => return Some(true),
        }
    }
    Some(false)
}

/// The Python version a PyInstaller CArchive cookie at `at` records, e.g. `3.12`.
///
/// The cookie is the magic, three big-endian lengths, the big-endian
/// `pyvers` (`312`, or `27` from old builds) and, since PyInstaller 2.1, a
/// 64-byte library name. A stray copy of the magic (the bootloader carries one)
/// is rejected by the version range and the library name.
fn pyinstaller_version(b: &[u8], at: usize) -> Option<String> {
    let field = |i: usize| -> Option<u32> {
        Some(u32::from_be_bytes(b.get(at + 8 + i * 4..at + 12 + i * 4)?.try_into().ok()?))
    };
    let pyvers = field(3)?;
    let (major, minor) = match pyvers {
        100..=399 => (pyvers / 100, pyvers % 100),
        20..=39 => (pyvers / 10, pyvers % 10),
        _ => return None,
    };
    let libname = b.get(at + 24..(at + 88).min(b.len())).unwrap_or(&[]);
    let names_python = libname.windows(6).any(|w| w.eq_ignore_ascii_case(b"python"));
    (names_python || b.len() == at + 24).then(|| format!("{major}.{minor}"))
}

/// The ILC version in a `.comment` string such as
/// `.NET: ilc 10.0.2-servicing.25612.105+hash`.
fn ilc_version(b: &[u8]) -> Option<String> {
    let at = b.windows(4).position(|w| w == b"ilc ")? + 4;
    let rest = b.get(at..)?;
    let end = rest
        .iter()
        .position(|c| !(c.is_ascii_digit() || *c == b'.'))
        .unwrap_or(rest.len());
    let version = std::str::from_utf8(&rest[..end]).ok()?;
    version.contains('.').then(|| version.to_string())
}

/// Strong markers of a .NET NativeAOT image, with the ILC release its
/// `.comment` section records. On ELF, `__managedcode` and `.dotnet_eh_table`
/// are both section names. A Windows link merges the managed sections into
/// `.text` and `.rdata`, so there the marker is the `DotNetRuntimeDebugHeader`
/// data export the runtime publishes for debuggers; a link that keeps the
/// `.unbox` and `.modules` sections is accepted too. Section and export names
/// keep an image that merely spells the markers out in its data from being
/// labeled.
fn nativeaot_version(b: &[u8], pe: Option<&PeView>) -> Option<Option<String>> {
    let windows = pe.is_some_and(|p| {
        let named = |name: &[u8]| p.section_names.iter().any(|n| n.as_slice() == name);
        !p.clr && (p.debug_header_export || (named(b".unbox") && named(b".modules")))
    });
    if !windows && !b.starts_with(b"\x7fELF") {
        return None;
    }
    let file = object::File::parse(b).ok();
    let section = |name: &str| file.as_ref()?.section_by_name(name);
    let elf = section("__managedcode").is_some() && section(".dotnet_eh_table").is_some();
    let comment = || section(".comment").and_then(|s| s.data().ok());
    (windows || elf).then(|| comment().and_then(ilc_version))
}

fn hint(id: &'static str, version: Option<String>, text: String, actionable: bool) -> RuntimeHint {
    RuntimeHint { id, version, hint: text, actionable }
}

/// Recognize the runtimes and packers `bytes` carries. `toolchain` adds the Go
/// and Rust verdicts, which need a full object parse and are informational.
pub fn detect(bytes: &[u8], toolchain: bool) -> Vec<RuntimeHint> {
    let pe = pe_view(bytes);
    let mut seen: Vec<Marker> = Vec::new();
    let mut last_cookie = None;
    for m in marker_regex().find_iter(bytes) {
        let Some(&(_, marker)) = MARKERS.iter().find(|(lit, _)| *lit == m.as_bytes()) else {
            continue;
        };
        if marker == Marker::PyInstaller {
            last_cookie = Some(m.start());
        } else if !seen.contains(&marker) {
            seen.push(marker);
        }
    }
    let windows_only = |m: Marker| pe.is_some() && seen.contains(&m);
    let mut out = Vec::new();

    if pe.as_ref().is_some_and(|p| p.clr) {
        out.push(hint(
            "dotnet",
            None,
            "managed .NET assembly: native decompilation shows only the stub; use ilspycmd".into(),
            true,
        ));
    }
    if let Some(version) = nativeaot_version(bytes, pe.as_ref()) {
        let label = version
            .as_deref()
            .map(|v| format!("NativeAOT (ILC {v})"))
            .unwrap_or_else(|| "NativeAOT".into());
        out.push(hint(
            "nativeaot",
            version,
            format!(
                "{label} image: native code from C#; decompiles as native C, but method names \
                 need the .dbg sidecar or a NativeAOT metadata tool"
            ),
            false,
        ));
    }
    if let Some(version) = last_cookie.and_then(|at| pyinstaller_version(bytes, at)) {
        out.push(hint(
            "pyinstaller",
            Some(version.clone()),
            format!("PyInstaller bundle (Python {version}): extract with pyinstxtractor-ng"),
            true,
        ));
    }
    if seen.contains(&Marker::Nuitka) {
        out.push(hint(
            "nuitka",
            None,
            "Nuitka-compiled Python: the code is C generated from Python; extract a onefile \
             payload with nuitka-extractor"
                .into(),
            true,
        ));
    }
    if windows_only(Marker::AutoIt) {
        out.push(hint(
            "autoit",
            None,
            "AutoIt compiled script: extract the script with autoit-ripper".into(),
            true,
        ));
    }
    let upx_sections = pe.as_ref().is_some_and(|p| {
        p.section_names.iter().any(|n| n.as_slice() == b"UPX0" || n.as_slice() == b"UPX1")
    });
    let upx_magic = bytes[..bytes.len().min(0x1000)].windows(4).any(|w| w == b"UPX!");
    if upx_sections || upx_magic || kuna_analysis::upx::detect(bytes).is_some() {
        out.push(hint("upx", None, "UPX packed: run 'kuna unpack'".into(), true));
    } else if pe.is_some() && kuna_analysis::neolite::detect(bytes) {
        out.push(hint("neolite", None, "NEOLite packed: run 'kuna unpack'".into(), true));
    } else if kuna_analysis::pklite::detect(bytes) {
        out.push(hint("pklite", None, "PKLITE packed: run 'kuna unpack'".into(), true));
    }
    if windows_only(Marker::TwinBasic) {
        out.push(hint(
            "twinbasic",
            None,
            "twinBASIC program: native code over a COM runtime; strings are BSTRs (UTF-16) and \
             most calls go through vtables"
                .into(),
            true,
        ));
    } else if windows_only(Marker::Vb6) {
        out.push(hint(
            "vb6",
            None,
            "Visual Basic 6 program (MSVBVM60): P-Code is not native code; use a VB6 decompiler \
             for forms and P-Code"
                .into(),
            true,
        ));
    }
    if toolchain {
        match effective_compiler_bytes(bytes) {
            Compiler::Go => out.push(hint(
                "go",
                None,
                "Go binary: function names come from pclntab; start at main.main".into(),
                false,
            )),
            Compiler::Rustc => out.push(hint(
                "rust",
                None,
                "Rust binary: output defaults to --language rust".into(),
                false,
            )),
            _ => {}
        }
    }
    out
}

/// The `summary.runtime` array.
pub fn to_json(hints: &[RuntimeHint]) -> Json {
    Json::Array(
        hints
            .iter()
            .map(|h| {
                Json::Object(vec![
                    ("id".into(), Json::Str(h.id.into())),
                    ("version".into(), h.version.clone().map(Json::Str).unwrap_or(Json::Null)),
                    ("hint".into(), Json::Str(h.hint.clone())),
                    ("actionable".into(), Json::Bool(h.actionable)),
                ])
            })
            .collect(),
    )
}

/// Print each actionable hint for `binary` as one `note:` line on stderr.
pub fn note_to_stderr(binary: &str) {
    let Ok(bytes) = kuna_analysis::loader::elf_shdr::read_image(binary) else {
        return;
    };
    for h in detect(&bytes, false).iter().filter(|h| h.actionable) {
        eprintln!("note: {binary}: {}", h.hint);
    }
}

/// The subcommands that take a binary and are not themselves the answer to a
/// hint (`unpack` is what the UPX hint points at).
const NOTED_SUBCOMMANDS: &[&str] = &[
    "decompile",
    "decompile-all",
    "decompile-project",
    "decompile-graph",
    "functions",
    "disassemble",
    "read",
    "xrefs",
    "strings",
];

/// The stderr note for one `kuna <sub> <rest..>` invocation, before dispatch.
///
/// Silent for help, for a `--jobs` worker (its parent already spoke) and for
/// `--summary`, which carries the hints in its own document.
pub fn note_for_invocation(sub: &str, rest: &[String]) {
    if !NOTED_SUBCOMMANDS.contains(&sub)
        || rest.iter().any(|a| {
            matches!(a.as_str(), "-h" | "--help" | "--summary" | "--jobs-worker" | "--raw-image")
        })
    {
        return;
    }
    if let Some(binary) = rest
        .iter()
        .find(|a| !a.starts_with('-') && std::path::Path::new(a.as_str()).is_file())
    {
        note_to_stderr(binary);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pe(opt_magic: u16, clr: bool, sections: &[&[u8]], extra: &[u8]) -> Vec<u8> {
        let mut b = vec![0u8; 0x400];
        b[0] = b'M';
        b[1] = b'Z';
        b[0x3c] = 0x80;
        b[0x80..0x84].copy_from_slice(b"PE\0\0");
        let coff = 0x84;
        b[coff + 2..coff + 4].copy_from_slice(&(sections.len() as u16).to_le_bytes());
        let opt_size: u16 = if opt_magic == 0x20b { 0xf0 } else { 0xe0 };
        b[coff + 16..coff + 18].copy_from_slice(&opt_size.to_le_bytes());
        let opt = coff + 20;
        b[opt..opt + 2].copy_from_slice(&opt_magic.to_le_bytes());
        let (ndirs_at, dirs_at) = if opt_magic == 0x20b { (opt + 108, opt + 112) } else { (opt + 92, opt + 96) };
        b[ndirs_at..ndirs_at + 4].copy_from_slice(&16u32.to_le_bytes());
        if clr {
            b[dirs_at + 112..dirs_at + 116].copy_from_slice(&0x2008u32.to_le_bytes());
            b[dirs_at + 116..dirs_at + 120].copy_from_slice(&0x48u32.to_le_bytes());
        }
        let table = opt + opt_size as usize;
        for (i, name) in sections.iter().enumerate() {
            b[table + i * 40..table + i * 40 + name.len()].copy_from_slice(name);
        }
        b.extend_from_slice(extra);
        b
    }

    /// Point section `index` of a PE32+ image from [`pe`] at `data`, appended.
    fn pe_section_data(mut image: Vec<u8>, index: usize, data: &[u8]) -> Vec<u8> {
        let header = 0x84 + 20 + 0xf0 + index * 40;
        let (len, at) = (data.len() as u32, image.len() as u32);
        image[header + 8..header + 12].copy_from_slice(&len.to_le_bytes());
        image[header + 16..header + 20].copy_from_slice(&len.to_le_bytes());
        image[header + 20..header + 24].copy_from_slice(&at.to_le_bytes());
        image.extend_from_slice(data);
        image
    }

    fn cookie(pyvers: u32, lib: &[u8]) -> Vec<u8> {
        let mut c = PYI_MAGIC.to_vec();
        c.extend_from_slice(&0x1000u32.to_be_bytes());
        c.extend_from_slice(&0x800u32.to_be_bytes());
        c.extend_from_slice(&0x100u32.to_be_bytes());
        c.extend_from_slice(&pyvers.to_be_bytes());
        let mut name = lib.to_vec();
        name.resize(64, 0);
        c.extend_from_slice(&name);
        c
    }

    fn ids(h: &[RuntimeHint]) -> Vec<&'static str> {
        h.iter().map(|h| h.id).collect()
    }

    #[test]
    fn clr_directory_marks_a_managed_assembly() {
        let hints = detect(&pe(0x10b, true, &[b".text"], &[]), false);
        assert_eq!(ids(&hints), ["dotnet"]);
        assert!(hints[0].hint.contains("ilspycmd"));
        assert!(detect(&pe(0x20b, true, &[b".text"], &[]), false)[0].id == "dotnet");
        assert!(detect(&pe(0x10b, false, &[b".text"], &[]), false).is_empty());
    }

    fn elf(sections: &[(&[u8], &[u8])]) -> Vec<u8> {
        use object::{write, Architecture, BinaryFormat, Endianness, SectionKind};
        let mut obj = write::Object::new(BinaryFormat::Elf, Architecture::X86_64, Endianness::Little);
        for (name, data) in sections {
            let id = obj.add_section(Vec::new(), name.to_vec(), SectionKind::Data);
            obj.append_section_data(id, data, 1);
        }
        obj.write().unwrap()
    }

    const ILC_COMMENT: &[u8] = b"\0GCC: (GNU) 13.3.0\0.NET: ilc 10.0.2-servicing.25612.105+hash\0";

    #[test]
    fn nativeaot_elf_is_named_with_its_ilc_version() {
        let elf = elf(&[
            (b"__managedcode", b"\xc3"),
            (b"__unbox", b"\xc3"),
            (b".dotnet_eh_table", b"\0"),
            (b".comment", ILC_COMMENT),
        ]);
        let hints = detect(&elf, false);
        assert_eq!(ids(&hints), ["nativeaot"]);
        assert_eq!(hints[0].version.as_deref(), Some("10.0.2"));
        assert!(hints[0].hint.contains("NativeAOT (ILC 10.0.2)"));
        assert!(!hints[0].actionable);
    }

    #[test]
    fn nativeaot_version_comes_from_the_comment_section() {
        let managed: &[(&[u8], &[u8])] = &[(b"__managedcode", b"\xc3"), (b".dotnet_eh_table", b"\0")];
        let decoy: (&[u8], &[u8]) = (b".rodata", b"built with ilc 1.2.3\0");
        let image = elf(&[managed[0], decoy, managed[1], (b".comment", ILC_COMMENT)]);
        assert_eq!(detect(&image, false)[0].version.as_deref(), Some("10.0.2"));

        let no_comment = detect(&elf(&[managed[0], decoy, managed[1]]), false);
        assert_eq!(ids(&no_comment), ["nativeaot"]);
        assert_eq!(no_comment[0].version, None);
        assert!(no_comment[0].hint.starts_with("NativeAOT image:"));
    }

    #[test]
    fn nativeaot_markers_spelled_out_in_data_are_not_section_names() {
        let image = elf(&[
            (b".text", b"\xc3"),
            (b".rodata", b"__managedcode\0.dotnet_eh_table\0.NET: ilc 1.2.3\0"),
            (b".comment", ILC_COMMENT),
        ]);
        assert!(detect(&image, false).is_empty());
    }

    #[test]
    fn an_image_carrying_the_detector_literals_is_not_nativeaot() {
        let exe = std::fs::read(std::env::current_exe().unwrap()).unwrap();
        let has = |needle: &[u8]| exe.windows(needle.len()).any(|w| w == needle);
        assert!(has(b"__managedcode") && has(b".dotnet_eh_table"));
        assert!(!ids(&detect(&exe, false)).contains(&"nativeaot"));
    }

    #[test]
    fn nativeaot_windows_pe_is_named_by_unbox_and_modules() {
        let sections: &[&[u8]] = &[b".text", b".unbox", b".modules", b".comment"];
        let image = pe_section_data(pe(0x20b, false, sections, b"\0ilc 1.2.3\0"), 3, ILC_COMMENT);
        let hints = detect(&image, false);
        assert_eq!(ids(&hints), ["nativeaot"]);
        assert_eq!(hints[0].version.as_deref(), Some("10.0.2"));

        let only_unbox = pe(0x20b, false, &[b".text", b".unbox"], &[]);
        assert!(detect(&only_unbox, false).is_empty());
    }

    /// A PE32+ image from [`pe`] whose second section is an export directory
    /// naming `names`, which must be sorted.
    fn pe_exporting(names: &[&[u8]]) -> Vec<u8> {
        let rva = 0x2000u32;
        let strings_at = 40 + 4 * names.len() as u32;
        let mut data = vec![0u8; 40];
        data[24..28].copy_from_slice(&(names.len() as u32).to_le_bytes());
        data[32..36].copy_from_slice(&(rva + 40).to_le_bytes());
        let mut strings = Vec::new();
        for name in names {
            data.extend_from_slice(&(rva + strings_at + strings.len() as u32).to_le_bytes());
            strings.extend_from_slice(name);
            strings.push(0);
        }
        data.extend_from_slice(&strings);
        let mut image = pe_section_data(pe(0x20b, false, &[b".text", b".rdata"], &[]), 1, &data);
        let (header, dirs_at) = (0x84 + 20 + 0xf0 + 40, 0x84 + 20 + 112);
        image[header + 12..header + 16].copy_from_slice(&rva.to_le_bytes());
        image[dirs_at..dirs_at + 4].copy_from_slice(&rva.to_le_bytes());
        image[dirs_at + 4..dirs_at + 8].copy_from_slice(&(data.len() as u32).to_le_bytes());
        image
    }

    #[test]
    fn nativeaot_windows_pe_is_named_by_its_debug_header_export() {
        let names: &[&[u8]] = &[b"AppEntry", b"DotNetRuntimeDebugHeader", b"add", b"shutdown"];
        for count in 2..=names.len() {
            let hints = detect(&pe_exporting(&names[..count]), false);
            assert_eq!(ids(&hints), ["nativeaot"]);
            assert_eq!(hints[0].version, None);
            assert!(hints[0].hint.starts_with("NativeAOT image:"));
        }
        assert_eq!(ids(&detect(&pe_exporting(&names[1..2]), false)), ["nativeaot"]);

        let near: &[&[u8]] = &[b"DotNetRuntimeDebugHeade", b"DotNetRuntimeDebugHeaderV2", b"add"];
        assert!(detect(&pe_exporting(near), false).is_empty());
        assert!(detect(&pe_exporting(&[]), false).is_empty());
        let spelled_out = pe(0x20b, false, &[b".text"], b"\0DotNetRuntimeDebugHeader\0");
        assert!(detect(&spelled_out, false).is_empty());

        let mut cut = pe_exporting(names);
        cut.truncate(cut.len() - 30);
        assert!(detect(&cut, false).is_empty());
    }

    #[test]
    fn nativeaot_needs_both_elf_markers_and_an_elf() {
        let only_managed = elf(&[(b"__managedcode", b"\xc3"), (b".comment", ILC_COMMENT)]);
        assert!(detect(&only_managed, false).is_empty());

        let pe_image = pe(0x20b, false, &[b".text"], b"__managedcode\0.dotnet_eh_table\0");
        assert!(detect(&pe_image, false).is_empty());
    }

    #[test]
    fn pyinstaller_cookie_reports_its_python_version() {
        let mut image = pe(0x20b, false, &[b".text"], &[]);
        image.extend(cookie(313, b"python313.dll"));
        let hints = detect(&image, false);
        assert_eq!(ids(&hints), ["pyinstaller"]);
        assert_eq!(hints[0].version.as_deref(), Some("3.13"));
        assert_eq!(hints[0].hint, "PyInstaller bundle (Python 3.13): extract with pyinstxtractor-ng");

        let mut elf = b"\x7fELF".to_vec();
        elf.resize(0x200, 0);
        elf.extend(cookie(312, b"libpython3.12.so"));
        elf.extend(vec![0u8; 0x800]);
        assert_eq!(detect(&elf, false)[0].version.as_deref(), Some("3.12"));
    }

    #[test]
    fn a_bare_cookie_magic_is_not_a_bundle() {
        let mut elf = b"\x7fELF".to_vec();
        elf.resize(0x200, 0);
        elf.extend_from_slice(PYI_MAGIC);
        elf.extend(vec![0xffu8; 0x100]);
        assert!(detect(&elf, false).is_empty());
    }

    #[test]
    fn upx_sections_or_magic_mark_a_packed_image() {
        assert_eq!(ids(&detect(&pe(0x10b, false, &[b"UPX0", b"UPX1"], &[]), false)), ["upx"]);
        let mut elf = b"\x7fELF".to_vec();
        elf.resize(0xe8, 0);
        elf.extend_from_slice(b"UPX!");
        elf.resize(0x400, 0);
        assert_eq!(ids(&detect(&elf, false)), ["upx"]);
    }

    #[test]
    fn windows_script_runtimes_need_a_pe() {
        let autoit = pe(0x10b, false, &[b".text"], b"....AU3!EA06....");
        assert_eq!(ids(&detect(&autoit, false)), ["autoit"]);
        let vb6 = pe(0x10b, false, &[b".text"], b"MSVBVM60.DLL\0");
        assert_eq!(ids(&detect(&vb6, false)), ["vb6"]);
        let mut elf = b"\x7fELF".to_vec();
        elf.extend_from_slice(b"AU3!EA06 twinBASIC MSVBVM60.DLL");
        assert!(detect(&elf, false).is_empty());
    }

    #[test]
    fn a_damaged_dos_magic_still_names_twinbasic() {
        let mut image = pe(0x10b, false, &[b".text"], b"\0twinBASIC\0");
        image[0] = 0x15;
        assert_eq!(ids(&detect(&image, false)), ["twinbasic"]);
    }

    #[test]
    fn nuitka_marker_is_recognized() {
        let mut elf = b"\x7fELF".to_vec();
        elf.extend_from_slice(b"\0NUITKA_ONEFILE_PARENT\0");
        assert_eq!(ids(&detect(&elf, false)), ["nuitka"]);
    }

    #[test]
    fn json_rows_carry_id_version_and_hint() {
        let rows = to_json(&[hint("pyinstaller", Some("3.12".into()), "x".into(), true)]);
        let text = crate::jsonfmt::dumps_compact(&rows);
        assert!(text.contains("\"id\"") && text.contains("\"3.12\"") && text.contains("\"actionable\""));
    }
}

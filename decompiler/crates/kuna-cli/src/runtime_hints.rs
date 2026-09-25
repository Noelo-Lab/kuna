//! Runtime and packer recognition for orientation: which wrapper or toolchain
//! produced an image, and what to do next when native decompilation of it is the
//! wrong tool (a managed assembly, a bundled interpreter, a packed stub).
//!
//! Detection reads raw bytes and a lenient PE header walk that does not insist
//! on the `MZ` magic, so a file whose DOS header was damaged on purpose is still
//! recognized even though it cannot be loaded. `functions --summary` reports the
//! result as `summary.runtime`; every other binary subcommand prints the
//! actionable entries as one `note:` line each on stderr (see `docs/cli.md`).

use kuna_analysis::analyzers::sourcelang::{detect_compiler_bytes, Compiler};
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
    let section_names = (0..nsections.min(96))
        .map_while(|i| b.get(table + i * 40..table + i * 40 + 8))
        .map(|n| n.iter().copied().take_while(|&c| c != 0).collect())
        .collect();
    Some(PeView { section_names, clr })
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
        match detect_compiler_bytes(bytes) {
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

//! (kuna) `pemain` — name the user entry function of a PE image `main`,
//! `wmain`, `WinMain` or `wWinMain` (P1 program prep).
//!
//! A PE's `AddressOfEntryPoint` is the C runtime's startup (`mainCRTStartup`,
//! `WinMainCRTStartup`, MinGW's `__tmainCRTStartup`), not the program. On a
//! stripped image the function the startup eventually calls is one more
//! `sub_<addr>` among thousands, so an agent's first question — where does the
//! program start — is answered only by walking `entry -> __scrt_common_main_seh
//! -> invoke_main` by hand. The runtime is inside the image, though, and each
//! toolchain calls the user entry from a recognizable site:
//!
//! - **MSVC, dynamic UCRT, console.** `invoke_main` fetches `argc`/`argv`/`envp`
//!   through the imported `__p___argc`/`__p___argv`/`_get_initial_*_environment`
//!   accessors immediately before the call. The scan is `entrymainproto`'s; the
//!   wide accessors name the callee `wmain`.
//! - **MSVC, dynamic UCRT, GUI.** `invoke_main` fetches the command line through
//!   `_get_narrow_winmain_command_line` (or the wide spelling) and passes
//!   `&__ImageBase` as `hInstance`; the next direct call into the image that
//!   loads the image base into its first argument is `WinMain` (`wWinMain`).
//! - **MinGW.** `__tmainCRTStartup` stores `envp` into msvcrt's `__initenv`
//!   (`__winitenv`) data import immediately before `main(argc, argv, envp)`;
//!   the reference is direct or through a `.refptr` word holding the IAT slot's
//!   address. The next direct call into the image is `main` (`wmain`).
//! - **MSVC, static CRT (x86-64).** The same `invoke_main` shape with the
//!   accessors linked in: three calls whose results are consumed as a pointer,
//!   a dereferenced pointer and a dereferenced 32-bit count, then the call. No
//!   name vouches for it, so it is used only when no import-anchored shape
//!   matched and only when exactly one site in the image has the shape.
//!
//! An incremental-link `jmp` thunk at the call target is followed to the body.
//!
//! ## What it refuses
//!
//! - anything that is not a PE;
//! - a callee outside every executable section, or at an import thunk/slot;
//! - a callee that already carries a function symbol (COFF `.symtab`, export),
//!   and an image that already defines a symbol with the chosen name;
//! - an ambiguous image: two different callees from the same shape.
//!
//! The name rides the `entry_names` overlay; the prototype `entrymainproto`
//! recovers is parked by address, so the two compose. Computed at LOAD and
//! COMMITTED only when the gate is on: `--option pemain off` restores the
//! `sub_<addr>` inventory exactly.

use object::read::{Object, ObjectSection, ObjectSymbol};

use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, Phase};

use super::{executable_sections, existing_function_addrs, in_executable_section};

/// (kuna) The PE user-entry naming pass (`pemain`).
pub struct PeMainPass;

impl AnalysisPass for PeMainPass {
    fn phase(&self) -> Phase {
        Phase::P1
    }

    fn id(&self) -> &'static str {
        "pemain"
    }

    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        let mut out = AnalysisOutput::default();
        if let Some((vma, name)) = pe_main(ctx.file, ctx.bytes) {
            out.entries.push(vma);
            out.entry_names.push((vma, name.to_string()));
        }
        out
    }
}

/// How far past the anchoring reference the user-entry call may sit.
const WINDOW: u64 = 96;

/// The user entry function's VMA and name, once every refusal has been applied.
pub(crate) fn pe_main(file: &object::File, bytes: &[u8]) -> Option<(u64, &'static str)> {
    if file.format() != object::BinaryFormat::Pe {
        return None;
    }
    let execs = executable_sections(file);
    let imports: Vec<(u64, String)> = crate::loader::format::resolve_imports(file, bytes)
        .into_iter()
        .map(|s| (s.addr, String::from_utf8_lossy(&s.name).into_owned()))
        .collect();
    let mut import_addrs: Vec<u64> = imports.iter().map(|(a, _)| *a).collect();
    import_addrs.sort_unstable();
    import_addrs.dedup();
    let named = |n: &str| imports.iter().any(|(_, i)| i == n);

    let local = |t: u64| in_executable_section(&execs, t) && import_addrs.binary_search(&t).is_err();
    let (site_target, name) = if let Some((t, _)) =
        super::kuna_entrymainproto::ucrt_main_call(file, bytes, &execs)
    {
        let wide = named("__p___wargv") || named("_get_initial_wide_environment");
        (t, if wide { "wmain" } else { "main" })
    } else if let Some(hit) = winmain_call(file, &execs, &imports, &local) {
        hit
    } else if let Some(hit) = mingw_main_call(file, &execs, &imports, &local) {
        hit
    } else if file.architecture() == object::Architecture::X86_64 {
        (static_invoke_main(&execs, &local)?, "main")
    } else {
        return None;
    };

    let vma = follow_jmp_thunks(&execs, site_target);
    if !local(vma) {
        return None;
    }
    if existing_function_addrs(file, bytes).binary_search(&vma).is_ok() {
        return None;
    }
    if file.symbols().chain(file.dynamic_symbols()).any(|s| s.name() == Ok(name)) {
        return None;
    }
    if let Ok(exports) = file.exports() {
        if exports.iter().any(|e| e.name() == name.as_bytes()) {
            return None;
        }
    }
    Some((vma, name))
}

/// Every direct `E8 rel32` in the executable bytes as `(site, target)`.
fn calls(execs: &[(u64, u64, Vec<u8>)]) -> Vec<(u64, u64)> {
    let mut out = Vec::new();
    for (lo, _, data) in execs {
        let mut i = 0usize;
        while i + 5 <= data.len() {
            if data[i] != 0xE8 {
                i += 1;
                continue;
            }
            let site = lo + i as u64;
            let disp = i32::from_le_bytes([data[i + 1], data[i + 2], data[i + 3], data[i + 4]]);
            out.push((site, (site + 5).wrapping_add(disp as i64 as u64)));
            i += 5;
        }
    }
    out
}

/// The bytes of `[from, to)` if both ends lie in one executable section.
fn span(execs: &[(u64, u64, Vec<u8>)], from: u64, to: u64) -> Option<&[u8]> {
    execs.iter().find(|(lo, hi, _)| from >= *lo && to <= *hi && from <= to).map(|(lo, _, d)| {
        &d[(from - lo) as usize..(to - lo) as usize]
    })
}

/// Does `[from, to)` hold a reference to one of `targets` — RIP-relative on
/// x86-64 (a `disp32` whose end plus value lands on it) or absolute on i386?
fn references(
    execs: &[(u64, u64, Vec<u8>)],
    from: u64,
    to: u64,
    targets: &[u64],
    rip_relative: bool,
) -> bool {
    let Some(bytes) = span(execs, from, to) else {
        return false;
    };
    (0..bytes.len().saturating_sub(3)).any(|i| {
        let w = u32::from_le_bytes([bytes[i], bytes[i + 1], bytes[i + 2], bytes[i + 3]]);
        let t = if rip_relative {
            (from + i as u64 + 4).wrapping_add(w as i32 as i64 as u64)
        } else {
            w as u64
        };
        targets.contains(&t)
    })
}

/// The first direct call into the image within [`WINDOW`] after `from` that
/// passes `accept`, skipping calls that leave the image.
fn next_local_call(
    all_calls: &[(u64, u64)],
    from: u64,
    local: &dyn Fn(u64) -> bool,
    accept: &dyn Fn(u64) -> bool,
) -> Option<(u64, u64)> {
    all_calls
        .iter()
        .filter(|(site, _)| *site >= from && *site - from <= WINDOW)
        .find(|(site, t)| local(*t) && accept(*site))
        .copied()
}

/// Exactly one distinct callee across `hits`, else `None`.
fn unique(hits: impl Iterator<Item = u64>) -> Option<u64> {
    let mut v: Vec<u64> = hits.collect();
    v.sort_unstable();
    v.dedup();
    (v.len() == 1).then(|| v[0])
}

/// MSVC GUI: a call to `_get_{narrow,wide}_winmain_command_line`, then the first
/// local call whose argument setup references the image base (`hInstance`).
fn winmain_call(
    file: &object::File,
    execs: &[(u64, u64, Vec<u8>)],
    imports: &[(u64, String)],
    local: &dyn Fn(u64) -> bool,
) -> Option<(u64, &'static str)> {
    let image_base = file.relative_address_base();
    let rip = file.is_64();
    let all = calls(execs);
    for (accessor, name) in [
        ("_get_narrow_winmain_command_line", "WinMain"),
        ("_get_wide_winmain_command_line", "wWinMain"),
    ] {
        let addrs: Vec<u64> =
            imports.iter().filter(|(_, n)| n == accessor).map(|(a, _)| *a).collect();
        if addrs.is_empty() {
            continue;
        }
        let hits = all.iter().filter(|(_, t)| addrs.contains(t)).filter_map(|(site, _)| {
            let after = site + 5;
            next_local_call(&all, after, local, &|call| {
                references(execs, after, call, &[image_base], rip)
            })
            .map(|(_, t)| t)
        });
        if let Some(t) = unique(hits) {
            return Some((t, name));
        }
    }
    None
}

/// MinGW: the `__initenv = envp` store just before `main(argc, argv, envp)`.
fn mingw_main_call(
    file: &object::File,
    execs: &[(u64, u64, Vec<u8>)],
    imports: &[(u64, String)],
    local: &dyn Fn(u64) -> bool,
) -> Option<(u64, &'static str)> {
    let rip = file.is_64();
    let ptr = if rip { 8 } else { 4 };
    let all = calls(execs);
    for (slot_name, name) in [("__initenv", "main"), ("__winitenv", "wmain")] {
        let mut markers: Vec<u64> =
            imports.iter().filter(|(_, n)| n == slot_name).map(|(a, _)| *a).collect();
        if markers.is_empty() {
            continue;
        }
        markers.extend(refptrs(file, &markers, ptr));
        let mut hits = Vec::new();
        for (lo, _, data) in execs {
            for i in 0..data.len().saturating_sub(3) {
                let at = lo + i as u64;
                if !references(execs, at, at + 4, &markers, rip) {
                    continue;
                }
                if let Some((_, t)) = next_local_call(&all, at + 4, local, &|_| true) {
                    hits.push(t);
                }
            }
        }
        if let Some(t) = unique(hits.into_iter()) {
            return Some((t, name));
        }
    }
    None
}

/// The addresses of pointer-width data words holding one of `slots` — MinGW's
/// `.refptr.__imp_<name>` indirection cells.
fn refptrs(file: &object::File, slots: &[u64], ptr: usize) -> Vec<u64> {
    let mut out = Vec::new();
    for sec in file.sections() {
        if sec.kind() == object::SectionKind::Text {
            continue;
        }
        let Ok(data) = sec.data() else { continue };
        let base = sec.address();
        let mut i = 0usize;
        while i + ptr <= data.len() {
            let w = if ptr == 8 {
                u64::from_le_bytes(data[i..i + 8].try_into().unwrap_or([0; 8]))
            } else {
                u32::from_le_bytes(data[i..i + 4].try_into().unwrap_or([0; 4])) as u64
            };
            if slots.contains(&w) {
                out.push(base + i as u64);
            }
            i += ptr;
        }
    }
    out
}

/// MSVC static CRT on x86-64: `call envp; call argv; mov r,[rax]; call argc;
/// mov r32,[rax]; call main` within 64 bytes, unique in the image.
fn static_invoke_main(execs: &[(u64, u64, Vec<u8>)], local: &dyn Fn(u64) -> bool) -> Option<u64> {
    let all = calls(execs);
    let qword_deref_rax = |b: &[u8]| {
        b.windows(3).any(|w| (w[0] == 0x48 || w[0] == 0x4C) && w[1] == 0x8B && w[2] & 0xC7 == 0)
    };
    let dword_deref_rax = |b: &[u8]| {
        (0..b.len().saturating_sub(1)).any(|i| {
            b[i] == 0x8B && b[i + 1] & 0xC7 == 0 && (i == 0 || b[i - 1] & 0xF8 != 0x48)
        })
    };
    let hits = all.windows(4).filter_map(|w| {
        let [(c1, t1), (c2, t2), (c3, t3), (x, m)] = [w[0], w[1], w[2], w[3]];
        if x - c1 > 64 || c2 < c1 + 5 || c3 < c2 + 5 || x < c3 + 5 {
            return None;
        }
        let targets = [t1, t2, t3, m];
        if !targets.iter().all(|t| local(*t)) {
            return None;
        }
        if (1..4).any(|i| targets[..i].contains(&targets[i])) {
            return None;
        }
        let argv = span(execs, c2 + 5, c3)?;
        let argc = span(execs, c3 + 5, x)?;
        (argv.len() <= 16 && qword_deref_rax(argv) && dword_deref_rax(argc)).then_some(m)
    });
    unique(hits)
}

/// Follow up to four `E9 rel32` incremental-link thunks from `vma`.
fn follow_jmp_thunks(execs: &[(u64, u64, Vec<u8>)], mut vma: u64) -> u64 {
    for _ in 0..4 {
        let Some(b) = span(execs, vma, vma + 5) else { break };
        if b[0] != 0xE9 {
            break;
        }
        let disp = i32::from_le_bytes([b[1], b[2], b[3], b[4]]);
        let next = (vma + 5).wrapping_add(disp as i64 as u64);
        if span(execs, next, next + 1).is_none() {
            break;
        }
        vma = next;
    }
    vma
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture(name: &str) -> Vec<u8> {
        let path = format!("{}/tests/fixtures/{}", env!("CARGO_MANIFEST_DIR"), name);
        std::fs::read(&path).unwrap_or_else(|_| panic!("read fixture {path}"))
    }

    fn claim(name: &str) -> Option<(u64, &'static str)> {
        let bytes = fixture(name);
        let file = object::File::parse(bytes.as_slice()).unwrap_or_else(|e| panic!("{name}: {e}"));
        pe_main(&file, bytes.as_slice())
    }

    /// A stripped MinGW image: `__tmainCRTStartup` stores `envp` through
    /// `.refptr.__imp___initenv` and then calls `main`, which its unstripped twin
    /// `pe_imports.exe` names at the same address.
    #[test]
    fn names_the_mingw_main_of_a_stripped_image() {
        assert_eq!(claim("pe_imports_stripped.exe"), Some((0x140001592, "main")));
    }

    /// The unstripped twin already names `main` from its COFF symbols.
    #[test]
    fn defers_to_a_symbol_table_name() {
        assert_eq!(claim("pe_imports.exe"), None);
    }

    /// Structurally inert off PE.
    #[test]
    fn contributes_nothing_off_pe() {
        assert_eq!(claim("cet_pie_x86_64"), None, "ELF");
        assert_eq!(claim("macho_stripped_main"), None, "Mach-O");
    }
}

//! `kuna decompile-project` — whole-binary **project export**.
//!
//! ```text
//!   kuna decompile-project <binary> [-o|--output DIR] [--functions a,b,..]
//!                          [--addr 0xVMA].. [--max-fn-seconds N]
//!                          [--mode reliable|aggressive] [--option N V]..
//!                          [--slice ARCH] [--target T] [--sleighpath D]
//! ```
//!
//! Loads + analyzes the binary once (the `decompile-all` in-process path,
//! Listing on by default) and writes a **project folder** — default
//! `<binary-filename>.kuna/` next to the binary, `-o DIR` overrides — designed
//! so a human or LLM can study the binary and attempt recompilation:
//!
//! * `<name>.c`   — every decompiled function (`// Function: <name> @ <addr>`
//!   headers, exactly the `decompile-all` rendering), `#include "<name>.h"`.
//! * `<name>.h`   — include-guarded recompile prelude (core scalar +
//!   `undefined` typedefs), the user-defined type definitions
//!   (`print_c_types`), and one prototype per decompiled function
//!   (`print_c_prototype` — token-identical to the `.c` definition line).
//! * `<name>.asm` — full labeled linear disassembly of every CODE section:
//!   `<name>:` labels matching the `.c` function names, per-function
//!   `; arg:` / `; stack:` comments mapping decompiled variables to stack
//!   offsets, undecodable bytes as `db` lines, and a `; --- data ---` tail
//!   listing the named globals plus every `dat_<hex>` address the `.c`
//!   references, with raw bytes.
//! * `README.md`  — binary metadata (size, arch, entry point, sections,
//!   function counts) and the artifact/labeling conventions.
//!
//! Exit code 0 on success **even if individual functions failed** (matching
//! `decompile-all` — failures are recorded in the artifacts); nonzero on load
//! errors, an empty target set, or I/O errors.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use kuna_console::engine::ConsoleProgram;
use kuna_decomp::decompile_drive::{print_c_recompile_prelude, print_c_types};
use object::{Object, ObjectSection};

use crate::decompile_all::{
    decompile_targets, load_program, parse_args, render_c, resolve_targets, Args, FuncResult,
};

/// `kuna_sleigh::loadimage::section_flags::CODE` — kuna-cli deliberately does
/// not depend on kuna-sleigh, so the (stable, upstream-fixed) bit value is
/// mirrored here; `ConsoleProgram::sections` documents its `flags` word as
/// exactly that constant set (UNALLOC=1, NOLOAD=2, CODE=4, DATA=8,
/// READONLY=16).
const SECTION_FLAG_CODE: u32 = 4;

/// `dat_` blocks larger than this are truncated in the `.asm` data tail (the
/// printer's `dat_<hex>` names carry no size; the label only marks the start).
const DAT_SIZE_CAP: u64 = 32;

/// `kuna decompile-project` entry point.
pub fn run(argv: &[String]) -> i32 {
    // Wrapper parse: extract `-o/--output`, intercept help, reject the
    // decompile-all-only flags, and hand the remainder to the shared parser.
    let mut output: Option<String> = None;
    let mut rest: Vec<String> = Vec::new();
    let mut i = 0;
    while i < argv.len() {
        match argv[i].as_str() {
            "-o" | "--output" => {
                if i + 1 >= argv.len() {
                    eprintln!("error: {} requires a value", argv[i]);
                    usage();
                    return 2;
                }
                output = Some(argv[i + 1].clone());
                i += 1;
            }
            "-h" | "--help" => {
                usage();
                return 0;
            }
            "--json" | "--no-vars" => {
                eprintln!("error: {} is not a decompile-project option", argv[i]);
                usage();
                return 2;
            }
            other => rest.push(other.to_string()),
        }
        i += 1;
    }
    let args = match parse_args(&rest, "decompile-project") {
        Ok(a) => a,
        Err(e) => {
            eprintln!("error: {e}");
            usage();
            return 2;
        }
    };
    match decompile_project(&args, output.as_deref()) {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("error: {e}");
            1
        }
    }
}

fn usage() {
    eprintln!(
        "usage: kuna decompile-project <binary> [-o|--output DIR] [--functions a,b,..] \\\n\
         \x20                   [--addr 0xVMA].. [--max-fn-seconds N] [--mode reliable|aggressive] \\\n\
         \x20                   [--option N V].. [--slice ARCH] [--target T] [--sleighpath D]\n\
         \n\
         Decompile a whole binary in one in-process load and write a project folder\n\
         (default `<binary-filename>.kuna/` next to the binary; -o DIR overrides):\n\
         \x20 <name>.c    every decompiled function (#include \"<name>.h\")\n\
         \x20 <name>.h    recompile prelude + type definitions + prototypes\n\
         \x20 <name>.asm  labeled disassembly (function labels, stack-var comments,\n\
         \x20             dat_<hex> data labels with raw bytes)\n\
         \x20 README.md   binary metadata (size, arch, entry, sections, counts)\n\
         Individual function failures are recorded in the artifacts; the run still\n\
         exits 0 (load errors / an empty target set / I/O errors exit nonzero)."
    );
}

/// The whole flow: load once → decompile every target (with prototypes) →
/// build the four artifacts → write them all at the end.
fn decompile_project(args: &Args, output: Option<&str>) -> Result<(), String> {
    let binary_path = std::fs::canonicalize(&args.binary)
        .map_err(|_| format!("binary not found: {}", args.binary))?;
    let file_name = binary_path
        .file_name()
        .ok_or_else(|| format!("binary has no file name: {}", binary_path.display()))?
        .to_string_lossy()
        .into_owned();
    let out_dir: PathBuf = match output {
        Some(dir) => PathBuf::from(dir),
        None => binary_path
            .parent()
            .ok_or_else(|| format!("binary has no parent directory: {}", binary_path.display()))?
            .join(format!("{file_name}.kuna")),
    };

    let mut prog = load_program(args, /* default_listing= */ true)?;
    // Per-function watchdog — same driver policy as `decompile-all` (default
    // 120 s, 0 disables): a non-converging function becomes its own error
    // record in the artifacts instead of hanging the export.
    if args.max_fn_seconds > 0 {
        prog.arch_mut().kuna_fn_budget =
            Some(std::time::Duration::from_secs(args.max_fn_seconds));
    }
    let targets = resolve_targets(&prog, args)?;
    if targets.is_empty() {
        return Err(format!("no functions selected/discovered in {}", args.binary));
    }

    let mut results =
        decompile_targets(&mut prog, targets, /* no_vars= */ false, /* want_proto= */ true);
    // Every artifact is address-ordered (resolve_targets only guarantees that
    // for the no-filter default; --addr/--functions arrive in user order).
    results.sort_by(|a, b| a.address.cmp(&b.address).then_with(|| a.name.cmp(&b.name)));

    // `print_c_types` AFTER the decompile loop: user-defined types are interned
    // into the factory as functions decompile.
    let prelude = print_c_recompile_prelude(prog.arch());
    let types = print_c_types(prog.arch_mut());

    let header = build_header(&file_name, &prelude, &types, &results);
    let c_file = build_c(&file_name, &results);
    let dat_addrs = collect_dat_addrs(&results);
    let asm = build_asm(&prog, &results, &dat_addrs, &file_name);
    let readme = build_readme(&binary_path, &file_name, &prog, &results);

    std::fs::create_dir_all(&out_dir)
        .map_err(|e| format!("cannot create {}: {e}", out_dir.display()))?;
    let mut sizes: Vec<(String, usize)> = Vec::new();
    for (base, text) in [
        (format!("{file_name}.c"), &c_file),
        (format!("{file_name}.h"), &header),
        (format!("{file_name}.asm"), &asm),
        ("README.md".to_string(), &readme),
    ] {
        let path = out_dir.join(&base);
        std::fs::write(&path, text).map_err(|e| format!("cannot write {}: {e}", path.display()))?;
        sizes.push((base, text.len()));
    }

    let ok = results.iter().filter(|r| r.error.is_none()).count();
    let failed = results.len() - ok;
    let files = sizes
        .iter()
        .map(|(n, s)| format!("{n} ({s} bytes)"))
        .collect::<Vec<_>>()
        .join(", ");
    println!("wrote {}: {files}; functions: {ok} ok, {failed} failed", out_dir.display());
    Ok(())
}

// --- .h ------------------------------------------------------------------

/// The include-guard macro: the file name sanitized to `[A-Z0-9_]` + `_H`
/// (a leading digit gets a `_` prefix — a macro name can't start with one).
fn sanitize_guard(file_name: &str) -> String {
    let mut s: String = file_name
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c.to_ascii_uppercase() } else { '_' })
        .collect();
    if s.chars().next().is_none_or(|c| c.is_ascii_digit()) {
        s.insert(0, '_');
    }
    s.push_str("_H");
    s
}

/// `<name>.h`: include guard + recompile prelude + user type definitions +
/// the prototype section (successes in address order; failures as comments).
fn build_header(file_name: &str, prelude: &str, types: &str, results: &[FuncResult]) -> String {
    let guard = sanitize_guard(file_name);
    let mut out = String::new();
    out.push_str(&format!("#ifndef {guard}\n#define {guard}\n\n"));
    out.push_str(prelude);
    if !types.is_empty() {
        out.push_str("\n/* user-defined types */\n");
        out.push_str(types);
    }
    out.push_str("\n/* function prototypes */\n");
    for r in results {
        match (&r.proto, &r.error) {
            (Some(proto), None) => {
                out.push_str(proto);
                if !proto.ends_with('\n') {
                    out.push('\n');
                }
            }
            _ => {
                out.push_str(&format!(
                    "/* {} @ 0x{:x}: decompile failed */\n",
                    r.name, r.address
                ));
            }
        }
    }
    out.push_str(&format!("\n#endif /* {guard} */\n"));
    out
}

// --- .c ------------------------------------------------------------------

/// `<name>.c`: the header include + the exact `decompile-all` concatenated-C
/// rendering (`render_c` — `// Function: <name> @ <addr>` blocks, failures as
/// `(error: …)` comments).
fn build_c(file_name: &str, results: &[FuncResult]) -> String {
    format!("#include \"{file_name}.h\"\n\n{}", render_c(results))
}

// --- dat_ collection ------------------------------------------------------

/// Scan every decompiled function's C for `dat_<hex>` tokens — the names the
/// printer generates on the fly (`kuna_global_data_name`, `dat_{off:x}`,
/// lowercase hex) for data addresses no symbol covers.  Hand-rolled (no regex
/// dep): the char before `dat_` must not be an identifier char, the token is
/// `dat_` + 1+ lowercase-hex chars, and the char after the hex run must not
/// be an identifier char either (so a user symbol like `dat_foo` or
/// `dat_12x3` never false-positives — the printer's tokens are exact by
/// construction).  Returns the parsed VMAs.
fn collect_dat_addrs(results: &[FuncResult]) -> BTreeSet<u64> {
    fn is_ident(b: u8) -> bool {
        b.is_ascii_alphanumeric() || b == b'_'
    }
    let mut out = BTreeSet::new();
    for r in results {
        let Some(code) = &r.code else { continue };
        let b = code.as_bytes();
        let mut i = 0;
        while let Some(pos) = code[i..].find("dat_") {
            let start = i + pos;
            let hex_start = start + 4;
            let mut hex_end = hex_start;
            while hex_end < b.len() && matches!(b[hex_end], b'0'..=b'9' | b'a'..=b'f') {
                hex_end += 1;
            }
            let prev_ok = start == 0 || !is_ident(b[start - 1]);
            let next_ok = hex_end == b.len() || !is_ident(b[hex_end]);
            if prev_ok && next_ok && hex_end > hex_start {
                if let Ok(vma) = u64::from_str_radix(&code[hex_start..hex_end], 16) {
                    out.insert(vma);
                }
            }
            i = hex_end.max(hex_start);
        }
    }
    out
}

// --- .asm ------------------------------------------------------------------

/// A function label in the sweep: the (possibly several — alias names at one
/// address survive `resolve_targets`) results anchored at a normalized VMA.
type LabelMap<'a> = BTreeMap<u64, Vec<&'a FuncResult>>;

/// `<name>.asm`: linear sweep of every CODE section with function labels +
/// stack-var comment blocks, then the `; --- data ---` tail (named globals ∪
/// the `dat_<hex>` set, with raw bytes).
fn build_asm(
    prog: &ConsoleProgram,
    results: &[FuncResult],
    dat_addrs: &BTreeSet<u64>,
    file_name: &str,
) -> String {
    // On ARM-family specs a Thumb function's entry VMA carries the mode bit;
    // the sweep walks even byte addresses, so labels are keyed on `vma & !1`.
    let arm = prog.description().starts_with("ARM");
    let normalize = |vma: u64| if arm { vma & !1 } else { vma };

    let mut labels: LabelMap = BTreeMap::new();
    for r in results {
        labels.entry(normalize(r.address)).or_default().push(r);
    }

    let sections = prog.sections();
    let mut out = String::new();
    out.push_str(&format!("; kuna decompile-project — {file_name}\n"));
    out.push_str(&format!("; arch: {}\n", prog.description()));

    let mut code_secs: Vec<(u64, u64)> = sections
        .iter()
        .filter(|(_, _, flags)| flags & SECTION_FLAG_CODE != 0)
        .map(|&(vma, size, _)| (vma, size))
        .collect();
    code_secs.sort_unstable();
    for (vma, size) in code_secs {
        let end = vma.saturating_add(size);
        out.push_str(&format!("\n; --- code section 0x{vma:x}..0x{end:x} ---\n"));
        sweep_code(prog, &labels, vma, end, &mut out);
    }

    emit_data_tail(prog, &sections, dat_addrs, &mut out);
    out
}

/// Disassemble `[start, end)` linearly: labels + stack-var headers at function
/// entries, one instruction line per decode, `db` byte lines (coalesced up to
/// 8, clamped at the next label) where decoding fails.
fn sweep_code(prog: &ConsoleProgram, labels: &LabelMap, start: u64, end: u64, out: &mut String) {
    // Pending `db` bytes not yet flushed: (start VMA, bytes).
    let mut db: Option<(u64, Vec<u8>)> = None;
    let flush_db = |db: &mut Option<(u64, Vec<u8>)>, out: &mut String| {
        if let Some((at, bytes)) = db.take() {
            let hex = bytes.iter().map(|b| format!("0x{b:02x}")).collect::<Vec<_>>().join(", ");
            out.push_str(&format!("  {at:08x}: db {hex}\n"));
        }
    };

    let mut addr = start;
    while addr < end {
        if let Some(rs) = labels.get(&addr) {
            flush_db(&mut db, out);
            for r in rs {
                out.push('\n');
                out.push_str(&format!("{}:  ; 0x{:x}\n", r.name, r.address));
                emit_var_header(r, out);
            }
        }
        // Never decode across the next label (a mis-sync would otherwise
        // swallow a function label into an instruction's byte extent) or the
        // section end.
        let next_stop =
            labels.range(addr + 1..).next().map(|(&a, _)| a.min(end)).unwrap_or(end);
        match prog.disassemble_at(addr) {
            Ok((len, mnem, body)) if len > 0 && addr + len as u64 <= next_stop => {
                flush_db(&mut db, out);
                let bytes = prog.read_bytes(addr, len as usize).unwrap_or_default();
                let hex = bytes.iter().map(|b| format!("{b:02x}")).collect::<Vec<_>>().join(" ");
                let line = format!("  {addr:08x}: {hex:<24}  {mnem:<10} {body}");
                out.push_str(line.trim_end());
                out.push('\n');
                addr += len as u64;
            }
            _ => {
                // Decode failure (or a decode that would cross the next label):
                // one raw byte, coalesced into up-to-8-byte `db` lines.
                match prog.read_bytes(addr, 1) {
                    Some(bytes) => {
                        let (_, pending) = db.get_or_insert_with(|| (addr, Vec::new()));
                        pending.push(bytes[0]);
                        if pending.len() >= 8 {
                            flush_db(&mut db, out);
                        }
                    }
                    None => {
                        flush_db(&mut db, out);
                        out.push_str(&format!("  {addr:08x}: db ?? (unreadable)\n"));
                    }
                }
                addr += 1;
            }
        }
    }
    flush_db(&mut db, out);
}

/// The per-function variable comment block: `; arg:` lines (ABI order), then
/// `; stack:` lines with the signed frame offset the decompiled name lives at.
fn emit_var_header(r: &FuncResult, out: &mut String) {
    for v in r.variables.iter().filter(|v| v.is_param) {
        out.push_str(&format!("; arg:   {} ({})\n", v.name, v.type_name));
    }
    for v in r.variables.iter().filter(|v| !v.is_param) {
        match v.stack_offset {
            Some(off) => {
                let sign = if off < 0 { '-' } else { '+' };
                out.push_str(&format!(
                    "; stack: {} @ [stack{}0x{:x}] ({})\n",
                    v.name,
                    sign,
                    off.unsigned_abs(),
                    v.type_name
                ));
            }
            None => out.push_str(&format!("; stack: {} ({})\n", v.name, v.type_name)),
        }
    }
}

/// One data-tail label: display name, byte size if a typed symbol supplied
/// one, and whether a `dat_<hex>` token also resolves here (the alias case:
/// the named symbol stays the label, the `dat_` spelling is appended so the
/// `.c` token remains greppable — `<name>:  ; 0x<addr> = dat_<hex>`).
struct DataLabel {
    name: String,
    type_size: Option<i64>,
    dat_alias: bool,
}

/// `; --- data ---`: address-sorted deduped labels (named globals ∪ `dat_`
/// tokens), each with a raw-byte dump (16/line + printable-ASCII column) or an
/// `?? (uninitialized/unmapped)` line for image-backed-less ranges (.bss).
fn emit_data_tail(
    prog: &ConsoleProgram,
    sections: &[(u64, u64, u32)],
    dat_addrs: &BTreeSet<u64>,
    out: &mut String,
) {
    let mut data: BTreeMap<u64, DataLabel> = BTreeMap::new();
    for (name, vma, type_size) in prog.global_data_symbols() {
        // First named symbol at a VMA wins (global_data_symbols is
        // (vma, name)-sorted; duplicates at one address are aliases).
        data.entry(vma).or_insert(DataLabel { name, type_size: Some(type_size), dat_alias: false });
    }
    for &vma in dat_addrs {
        data.entry(vma)
            .and_modify(|l| l.dat_alias = true)
            .or_insert_with(|| DataLabel {
                name: format!("dat_{vma:x}"),
                type_size: None,
                dat_alias: false,
            });
    }
    if data.is_empty() {
        return;
    }

    out.push_str("\n; --- data ---\n");
    let addrs: Vec<u64> = data.keys().copied().collect();
    for (idx, (&vma, label)) in data.iter().enumerate() {
        // Size: a typed symbol's datatype size; a bare `dat_` gets
        // min(gap to the next label / containing-section end, 32), floor 1.
        let size = match label.type_size {
            Some(s) if s > 0 => s as u64,
            _ => {
                let next_label = addrs.get(idx + 1).copied();
                let sec_end = sections
                    .iter()
                    .find(|&&(sv, ss, _)| vma >= sv && vma < sv.saturating_add(ss))
                    .map(|&(sv, ss, _)| sv + ss);
                let bound = [next_label, sec_end]
                    .into_iter()
                    .flatten()
                    .filter(|&b| b > vma)
                    .map(|b| b - vma)
                    .min()
                    .unwrap_or(DAT_SIZE_CAP);
                bound.clamp(1, DAT_SIZE_CAP)
            }
        };
        out.push('\n');
        if label.dat_alias {
            out.push_str(&format!("{}:  ; 0x{vma:x} = dat_{vma:x}\n", label.name));
        } else {
            out.push_str(&format!("{}:  ; 0x{vma:x}\n", label.name));
        }
        match prog.read_bytes(vma, size as usize) {
            Some(bytes) => {
                for (row, chunk) in bytes.chunks(16).enumerate() {
                    let hex =
                        chunk.iter().map(|b| format!("{b:02x}")).collect::<Vec<_>>().join(" ");
                    let ascii: String = chunk
                        .iter()
                        .map(|&b| if (0x20..0x7f).contains(&b) { b as char } else { '.' })
                        .collect();
                    out.push_str(&format!(
                        "  {:08x}: {hex:<47}  |{ascii}|\n",
                        vma + row as u64 * 16
                    ));
                }
            }
            None => {
                out.push_str(&format!(
                    "  {vma:08x}: ?? (uninitialized/unmapped, {size} bytes)\n"
                ));
            }
        }
    }
}

// --- README.md ---------------------------------------------------------------

/// `README.md`: binary metadata (file size, arch id, entry point + sections
/// re-parsed via the `object` crate — the engine's LoadImage sections carry no
/// names), function counts, and the artifact inventory / labeling conventions.
fn build_readme(
    binary_path: &Path,
    file_name: &str,
    prog: &ConsoleProgram,
    results: &[FuncResult],
) -> String {
    let file_size =
        std::fs::metadata(binary_path).map(|m| m.len().to_string()).unwrap_or_else(|_| "?".into());
    let ok = results.iter().filter(|r| r.error.is_none()).count();
    let failed = results.len() - ok;

    let mut out = String::new();
    out.push_str(&format!("# {file_name} — kuna project export\n\n"));
    out.push_str("Generated by `kuna decompile-project`.\n\n");
    out.push_str("## Binary\n\n");
    out.push_str("| Field | Value |\n|---|---|\n");
    out.push_str(&format!("| Path | `{}` |\n", binary_path.display()));
    out.push_str(&format!("| File size | {file_size} bytes |\n"));
    out.push_str(&format!("| Architecture | `{}` |\n", prog.description()));

    // Entry point + named sections come from an `object` re-parse (the engine's
    // section iterator has no name field).
    let parsed = std::fs::read(binary_path).ok();
    let parsed = parsed.as_deref().and_then(|b| object::File::parse(b).ok());
    match &parsed {
        Some(f) => out.push_str(&format!("| Entry point | `0x{:x}` |\n", f.entry())),
        None => out.push_str("| Entry point | unavailable |\n"),
    }
    out.push_str(&format!("| Functions | {} total, {ok} decompiled, {failed} failed |\n", results.len()));

    if let Some(f) = &parsed {
        out.push_str("\n## Sections\n\n");
        out.push_str("| Name | Address | Size | Kind |\n|---|---|---|---|\n");
        for s in f.sections() {
            out.push_str(&format!(
                "| `{}` | `0x{:x}` | `0x{:x}` | {:?} |\n",
                s.name().unwrap_or("?"),
                s.address(),
                s.size(),
                s.kind()
            ));
        }
    }

    out.push_str(&format!(
        "\n## Files\n\n\
         - `{file_name}.c` — every decompiled function, address-ordered, each under a\n\
         \x20 `// Function: <name> @ <addr>` header (failures appear as\n\
         \x20 `// Function: <name> @ <addr>  (error: ...)` comments). Includes `{file_name}.h`.\n\
         - `{file_name}.h` — recompilation aid: the generated core-typedef prelude (the\n\
         \x20 Ghidra/kuna `undefined` family included), the user-defined type definitions\n\
         \x20 recovered during decompilation, and one prototype per decompiled function\n\
         \x20 (token-identical to its `.c` definition line).\n\
         - `{file_name}.asm` — labeled linear disassembly of every code section. Function\n\
         \x20 labels match the `.c` names exactly (`main:`, `sub_<addr>:`); under each label\n\
         \x20 a comment block maps the decompiled variables to their storage\n\
         \x20 (`; arg: <name> (<type>)`, `; stack: <name> @ [stack-0x18] (<type>)`).\n\
         \x20 Undecodable bytes appear as `db` lines. The `; --- data ---` tail labels the\n\
         \x20 named globals and every `dat_<hex>` address the `.c` references, with raw\n\
         \x20 bytes (`??` for unmapped/.bss ranges).\n\
         - `README.md` — this file.\n\
         \n\
         ## Labeling conventions\n\n\
         - Functions without a symbol name keep the generated `sub_<addr>` /\n\
         \x20 `FUN_<addr>` name in both `{file_name}.c` and `{file_name}.asm` — the label in the\n\
         \x20 `.asm` is the anchor for the same function in the `.c`.\n\
         - Unnamed data referenced by the C appears as `dat_<hex>` (the hex is the VMA);\n\
         \x20 the same spelling labels the bytes in the `.asm` data tail. When a named\n\
         \x20 global covers that address the symbol name stays the label and the `dat_`\n\
         \x20 spelling is appended: `<name>:  ; 0x<addr> = dat_<hex>`.\n"
    ));
    out
}

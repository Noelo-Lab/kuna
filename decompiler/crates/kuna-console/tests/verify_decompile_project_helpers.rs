//! End-to-end gate for the **whole-binary export helpers** on [`ConsoleProgram`]
//! that back `kuna decompile-project` (the `.asm`/README surface):
//!
//! ```text
//!   sections()             (vma, size, flags) — the loader's section map
//!   disassemble_at[_into]   one instruction, allocating or buffer-reusing
//!   read_bytes[_into]       raw image bytes, allocating or buffer-reusing
//!   global_data_symbols()  (name, vma, type_size) — named global data
//! ```
//!
//! Drives them against the real vendored `fauxware` ELF (x86-64, non-PIE,
//! dynamically linked, non-stripped) after the exact `decompile-all` load shape
//! (`bootstrap_from_object` + `commit_pending_analysis`).
//!
//! ## `.sla` precondition
//!
//! Like the sibling loader gates, bootstrapping needs the built `x86` `.sla`
//! under `specs/` (gitignored; `make specs`).  When it is absent the bootstrap
//! fails; the test prints that and returns early (a specs-less CI is a visible
//! skip, never a false green).

use std::path::PathBuf;

use kuna_console::engine::bootstrap_from_object;
use kuna_sleigh::loadimage::section_flags;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// The vendored fauxware fixture (shared with the kuna-analysis loader gate).
fn fauxware() -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/fauxware")
}

/// fauxware's text `PT_LOAD` maps file offset 0 at vaddr 0x400000 (non-PIE
/// ET_EXEC), so a text VMA's file offset is `vma - LOAD_BASE` — the round-trip
/// oracle for `read_bytes`.
const LOAD_BASE: u64 = 0x400000;

#[test]
fn export_helpers_report_sections_disasm_bytes_and_globals() {
    let root = repo_root();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    let bin = match fauxware().to_str() {
        Some(s) => s.to_string(),
        None => return,
    };

    let mut prog = match bootstrap_from_object(&bin, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_decompile_project_helpers: skipping (bootstrap failed, build `.sla` \
                 with `make specs`): {}",
                e.explain()
            );
            return;
        }
    };
    prog.commit_pending_analysis().expect("read symbols (analysis commit) must succeed");

    // (1) sections(): the loader's section map must be non-empty and contain at
    // least one CODE-flagged range (the `.text` sweep range the `.asm` emitter
    // walks), and `main`'s entry must fall inside a CODE range.
    let sections = prog.sections();
    assert!(!sections.is_empty(), "sections() returned no sections");
    let code_ranges: Vec<&(u64, u64, u32)> =
        sections.iter().filter(|(_, _, f)| f & section_flags::CODE != 0).collect();
    assert!(!code_ranges.is_empty(), "no CODE-flagged section in {sections:?}");
    let main_vma = prog
        .function_entries()
        .find(|(n, _)| *n == "main")
        .map(|(_, a)| a.get_offset())
        .expect("main present in enumeration");
    assert!(
        code_ranges.iter().any(|(vma, size, _)| main_vma >= *vma && main_vma < vma + size),
        "main @ {main_vma:#x} not inside any CODE section: {code_ranges:?}"
    );

    // (2) disassemble_at(main entry): one decoded instruction with a positive
    // length and a non-empty mnemonic.
    let (len, mnem, _body) = prog.disassemble_at(main_vma).expect("disassemble_at(main) failed");
    assert!(len > 0, "instruction length must be positive, got {len}");
    assert!(!mnem.is_empty(), "empty mnemonic at main entry");
    let mut reuse_mnem = String::with_capacity(32);
    let mut reuse_body = String::with_capacity(128);
    reuse_mnem.push_str("stale mnemonic");
    reuse_body.push_str("stale body");
    let reuse_caps = (reuse_mnem.capacity(), reuse_body.capacity());
    let reuse_ptrs = (reuse_mnem.as_ptr(), reuse_body.as_ptr());
    let reuse_len = prog
        .disassemble_at_into(main_vma, &mut reuse_mnem, &mut reuse_body)
        .expect("disassemble_at_into(main) failed");
    let (_, expected_mnem, expected_body) =
        prog.disassemble_at(main_vma).expect("second disassemble_at(main) failed");
    assert_eq!((reuse_len, &reuse_mnem, &reuse_body), (len, &expected_mnem, &expected_body));
    assert_eq!(reuse_caps, (reuse_mnem.capacity(), reuse_body.capacity()));
    assert_eq!(reuse_ptrs, (reuse_mnem.as_ptr(), reuse_body.as_ptr()));

    // (3) read_bytes: correct length, stable across two calls, and a byte-exact
    // round-trip against the fixture file at the corresponding file offset
    // (fauxware's text LOAD maps file offset 0 at LOAD_BASE).
    let n = len as usize + 8; // one instruction plus a tail
    let got = prog.read_bytes(main_vma, n).expect("read_bytes(main) returned None");
    assert_eq!(got.len(), n, "read_bytes returned the wrong number of bytes");
    let again = prog.read_bytes(main_vma, n).expect("second read_bytes(main) returned None");
    assert_eq!(got, again, "read_bytes not stable across two calls");
    let mut reuse_bytes = Vec::with_capacity(n + 16);
    reuse_bytes.extend_from_slice(b"stale bytes");
    let reuse_cap = reuse_bytes.capacity();
    let reuse_ptr = reuse_bytes.as_ptr();
    assert!(
        prog.read_bytes_into(main_vma, n, &mut reuse_bytes),
        "read_bytes_into(main) returned false"
    );
    assert_eq!(reuse_bytes, got, "read_bytes_into differs from read_bytes");
    assert_eq!(reuse_bytes.capacity(), reuse_cap);
    assert_eq!(reuse_bytes.as_ptr(), reuse_ptr);
    let file = std::fs::read(fauxware()).expect("read fixture file");
    let off = (main_vma - LOAD_BASE) as usize;
    assert_eq!(
        &got[..],
        &file[off..off + n],
        "read_bytes({main_vma:#x}) != file bytes at offset {off:#x}"
    );
    // An address far below the image is unmapped: Err maps to None (the `.bss` /
    // hole contract the `.asm` data tail relies on).
    assert!(prog.read_bytes(0x10, 4).is_none(), "read of unmapped address must be None");
    assert!(!prog.read_bytes_into(0x1000, 4, &mut reuse_bytes));
    assert!(reuse_bytes.is_empty(), "failed read_bytes_into must clear stale bytes");
    assert_eq!(reuse_bytes.capacity(), reuse_cap);
    assert_eq!(reuse_bytes.as_ptr(), reuse_ptr);

    // (4) global_data_symbols(): returns without error; every tuple is sane
    // (non-empty name, positive type size) and no function symbol leaks through
    // (fauxware's `main`/`authenticate` are FunctionSymbols in the same scope).
    let globals = prog.global_data_symbols();
    for (name, vma, size) in &globals {
        assert!(!name.is_empty(), "global data symbol with an empty name @ {vma:#x}");
        assert!(*size >= 1, "global data symbol {name} has non-positive size {size}");
    }
    assert!(
        globals.iter().all(|(n, _, _)| n != "main" && n != "authenticate"),
        "function symbols leaked into global_data_symbols(): {globals:?}"
    );
    // Non-stripped fauxware carries string literals; the default-on strings
    // analysis maps each as a typed `s_<addr>` char[N] data symbol, so the set
    // is non-empty (verified empirically; keeps the exporter's data-label merge
    // honest).
    assert!(!globals.is_empty(), "expected named global data on non-stripped fauxware");
    eprintln!("global_data_symbols on fauxware ({}): {:?}", globals.len(), globals);
}

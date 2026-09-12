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

use kuna_console::engine::{bootstrap_from_object, FunctionEntry};
use kuna_console::project::{
    build_asm, build_readme, collect_dat_addrs, decompile_entry, decompile_pulled,
    decompile_targets, render_readme,
    AsmPhase, DecompileOptions, FuncResult, ReadmeCounts, ReadmeFacts, ReadmeLayout,
    StreamPhase, StreamProgress, render_readme_streaming,
};
use kuna_console::project_stream::{
    asm_labels_from_entries, asm_labels_from_results, render_data_tail, render_variables_section,
    AsmSweep,
};
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

/// The entry named `name`, from the canonical inventory.
fn entry_named(prog: &kuna_console::engine::ConsoleProgram, name: &str) -> FunctionEntry {
    prog.function_entries_canonical()
        .into_iter()
        .find(|e| e.name == name)
        .unwrap_or_else(|| panic!("{name} is not in the fauxware inventory"))
}

fn loaded_fauxware() -> Option<kuna_console::engine::ConsoleProgram> {
    let root = repo_root();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    let bin = fauxware().to_str()?.to_string();
    let mut prog = match bootstrap_from_object(&bin, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_decompile_project_helpers: skipping (bootstrap failed, build `.sla` \
                 with `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };
    prog.commit_pending_analysis().expect("read symbols (analysis commit) must succeed");
    Some(prog)
}

/// `--stream`'s scheduling frontier: the entry point reaches `main` (which it
/// only ever names as a pointer argument to `__libc_start_main`), and `main`
/// reaches the function it calls.
#[test]
fn callee_hints_carry_direct_calls_and_address_taken_callees() {
    let Some(mut prog) = loaded_fauxware() else { return };

    let start = entry_named(&prog, "_start");
    let main = entry_named(&prog, "main");
    let authenticate = entry_named(&prog, "authenticate");
    let (start_vma, main_vma, auth_vma) = (
        start.addr.get_offset(),
        main.addr.get_offset(),
        authenticate.addr.get_offset(),
    );

    let opts = DecompileOptions {
        no_vars: true,
        want_callee_hints: true,
        ..DecompileOptions::default()
    };
    let mut pending = vec![start, main].into_iter();
    let mut results: Vec<FuncResult> = Vec::new();
    decompile_pulled(&mut prog, &opts, &mut || pending.next(), &mut |r| results.push(r));

    assert_eq!(results.len(), 2, "one result per target, in pull order");
    assert_eq!((results[0].byte_address, results[1].byte_address), (start_vma, main_vma));
    for r in &results {
        eprintln!("{} @ {:#x} hints: {:x?}", r.name, r.byte_address, r.callee_hints);
        assert!(r.error.is_none(), "{} failed: {:?}", r.name, r.error);
        let mut sorted = r.callee_hints.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted, r.callee_hints, "{}: hints must be sorted and deduped", r.name);
        assert!(
            !r.callee_hints.contains(&r.byte_address),
            "{}: a function is never its own hint",
            r.name
        );
    }
    assert!(
        results[0].callee_hints.contains(&main_vma),
        "_start must reach main ({main_vma:#x}) through the __libc_start_main pointer: {:x?}",
        results[0].callee_hints
    );
    assert!(
        results[1].callee_hints.contains(&auth_vma),
        "main must reach its direct callee authenticate ({auth_vma:#x}): {:x?}",
        results[1].callee_hints
    );

    // Not asked for means not paid for.
    let again = entry_named(&prog, "main");
    let quiet = decompile_targets(&mut prog, vec![again], true, false, false);
    assert!(quiet[0].callee_hints.is_empty(), "hints are opt-in");
}

/// The resumable sweep is the one-shot sweep: the same bytes however the budget
/// chops it up, and the same bytes `build_asm` writes before its data tail.
#[test]
fn resumable_asm_sweep_matches_the_one_shot_sweep() {
    let Some(mut prog) = loaded_fauxware() else { return };

    let targets: Vec<FunctionEntry> = prog
        .function_entries_executable()
        .into_iter()
        .filter(|e| ["main", "authenticate", "accepted", "rejected"].contains(&e.name.as_str()))
        .collect();
    assert!(!targets.is_empty(), "expected named fauxware functions");
    let results = decompile_targets(&mut prog, targets.clone(), false, true, false);

    let labels = asm_labels_from_results(&results);
    let mut one_shot: Vec<u8> = Vec::new();
    let mut sweep = AsmSweep::new(&prog, "fauxware", labels.clone());
    one_shot.extend_from_slice(sweep.header_lines().as_bytes());
    while !sweep.step(&prog, u64::MAX, &mut one_shot).unwrap() {}
    assert!(sweep.is_done());

    for budget in [1u64, 3, 64, 4096] {
        let mut stepped: Vec<u8> = Vec::new();
        let mut sweep = AsmSweep::new(&prog, "fauxware", labels.clone());
        stepped.extend_from_slice(sweep.header_lines().as_bytes());
        let mut steps = 0;
        while !sweep.step(&prog, budget, &mut stepped).unwrap() {
            steps += 1;
            assert!(steps < 1_000_000, "sweep made no progress at budget {budget}");
        }
        assert_eq!(
            String::from_utf8(stepped).unwrap(),
            String::from_utf8(one_shot.clone()).unwrap(),
            "budget {budget} changed the sweep bytes"
        );
        assert!(budget > 64 || steps > 1, "budget {budget} should need several steps");
    }

    // The sweep holds no borrow of the program between steps, so a serial
    // streamed export can interleave sweeping with decompiling.
    let mut interleaved: Vec<u8> = Vec::new();
    let mut sweep = AsmSweep::new(&prog, "fauxware", labels.clone());
    interleaved.extend_from_slice(sweep.header_lines().as_bytes());
    while !sweep.step(&prog, 512, &mut interleaved).unwrap() {
        let again = entry_named(&prog, "main");
        let opts = DecompileOptions { no_vars: true, ..DecompileOptions::default() };
        assert!(decompile_entry(&mut prog, again, &opts).error.is_none());
    }
    assert_eq!(
        String::from_utf8(interleaved).unwrap(),
        String::from_utf8(one_shot.clone()).unwrap(),
        "interleaving decompiles with sweep steps changed the sweep bytes"
    );

    // `build_asm` is the same sweep plus the data tail.
    let dat = collect_dat_addrs(&results);
    let full = build_asm(&prog, &results, &dat, "fauxware");
    let tail = render_data_tail(&prog, &dat);
    assert_eq!(full, format!("{}{tail}", String::from_utf8(one_shot).unwrap()));

    // Target-derived labels carry no variable comments and are address-ordered.
    let entry_labels = asm_labels_from_entries(&prog, &targets);
    assert!(entry_labels.iter().all(|l| l.header.is_empty()));
    assert!(entry_labels.windows(2).all(|w| w[0].byte_address <= w[1].byte_address));
    let mut streamed: Vec<u8> = Vec::new();
    let mut sweep = AsmSweep::new(&prog, "fauxware", entry_labels);
    streamed.extend_from_slice(sweep.header_lines().as_bytes());
    while !sweep.step(&prog, 4096, &mut streamed).unwrap() {}
    let streamed = String::from_utf8(streamed).unwrap();
    let stripped: String = full[..full.len() - tail.len()]
        .lines()
        .filter(|l| !l.starts_with("; arg:") && !l.starts_with("; stack:"))
        .map(|l| format!("{l}\n"))
        .collect();
    assert_eq!(
        streamed, stripped,
        "a header-less sweep is the export sweep with its variable comments removed"
    );

    // Those removed lines are exactly what the variables section carries.
    let variables = render_variables_section(&results);
    assert!(variables.starts_with("\n; --- variables ---\n"));
    for r in &results {
        if r.variables.is_empty() {
            continue;
        }
        assert!(
            variables.contains(&format!("; {}:  ; 0x{:x}\n", r.name, r.address)),
            "{} missing from the variables section",
            r.name
        );
    }
    let moved: Vec<&str> = variables
        .lines()
        .filter(|l| l.starts_with("; arg:") || l.starts_with("; stack:"))
        .collect();
    let original: Vec<&str> = full[..full.len() - tail.len()]
        .lines()
        .filter(|l| l.starts_with("; arg:") || l.starts_with("; stack:"))
        .collect();
    assert_eq!(moved.len(), original.len(), "no variable comment is lost or invented");
}

/// The README a streamed export writes before its program is loaded says so,
/// and the final one is the standard render with the streamed file list.
#[test]
fn streaming_readme_renders_pending_facts_then_the_streamed_layout() {
    let facts = ReadmeFacts::pending(&fauxware(), "/tmp/fauxware", "fauxware");
    let progress = StreamProgress {
        phase: StreamPhase::Loading,
        jobs: 4,
        ..StreamProgress::default()
    };
    let loading = render_readme_streaming(&facts, &progress);
    assert!(loading.starts_with("# fauxware — kuna project export\n\n**This export is still streaming**"));
    assert!(loading.contains("| Architecture | pending |\n"));
    assert!(loading.contains("| Entry point | pending |\n"));
    assert!(loading.contains("| Functions | pending |\n"));
    assert!(loading.contains("| Phase | loading |\n"));
    assert!(loading.contains("| Disassembly | pending |\n"));
    assert!(loading.contains("`index.jsonl`"));
    assert!(loading.contains("`.streaming`"));
    assert!(!loading.contains("| Error |"));

    // Nothing was created, so the README describes the folder as it is: the
    // status file and itself, with no inventory of artifacts that do not exist.
    let failed = render_readme_streaming(
        &facts,
        &StreamProgress {
            phase: StreamPhase::Failed,
            error: Some("cannot load the binary".into()),
            ..StreamProgress::default()
        },
    );
    assert!(failed.contains("| Phase | failed |\n"));
    assert!(failed.contains("| Error | cannot load the binary |\n"));
    assert!(
        !failed.contains("still streaming"),
        "a failed export is not still streaming:\n{failed}"
    );
    assert!(failed.contains("failed before it wrote anything"), "{failed}");
    assert!(failed.contains("`.streaming`"), "it has to say what IS there:\n{failed}");
    assert!(!failed.contains("## Files"), "nothing was written to inventory:\n{failed}");
    assert!(!failed.contains("fauxware.c"), "no .c was created:\n{failed}");
    assert!(
        !failed.contains("While `.streaming` exists the export is incomplete"),
        "the guide to reading a running export does not apply:\n{failed}"
    );
    assert!(failed.contains("| Path | `/tmp/fauxware` |\n"), "{failed}");

    // Once the artifacts exist a failure still inventories them: they are what
    // the folder holds, however far the run got.
    let failed_late = render_readme_streaming(
        &facts,
        &StreamProgress {
            phase: StreamPhase::Failed,
            error: Some("cannot append to fauxware.c: No space left on device".into()),
            artifacts: true,
            total: Some(10),
            done: 4,
            ..StreamProgress::default()
        },
    );
    assert!(failed_late.contains("## Files"), "{failed_late}");
    assert!(failed_late.contains("| Functions written | 4 of 10 (0 failed) |\n"));
    assert!(failed_late.contains("While `.streaming` exists the export is incomplete"));

    let Some(prog) = loaded_fauxware() else { return };
    let facts = ReadmeFacts::snapshot(&fauxware(), "/tmp/fauxware", "fauxware", &prog);
    assert!(facts.description.is_some());
    assert!(facts.entry.is_some());
    assert!(!facts.sections.is_empty());
    let running = render_readme_streaming(
        &facts,
        &StreamProgress {
            phase: StreamPhase::Decompiling,
            jobs: 1,
            total: Some(10),
            done: 4,
            failed: 1,
            asm: AsmPhase::Sweeping,
            error: None,
            elapsed_s: 7,
            artifacts: true,
        },
    );
    assert!(running.contains("| Functions | 10 total, 3 decompiled, 1 failed |\n"));
    assert!(running.contains("| Functions written | 4 of 10 (1 failed) |\n"));
    assert!(running.contains("| Disassembly | sweeping |\n"));
    assert!(running.contains("| Elapsed | 7s |\n"));

    // The finished streamed README drops the banner and the status section; only
    // the file list differs from a non-stream export's.
    let results: Vec<FuncResult> = Vec::new();
    let counts = ReadmeCounts::of(&results);
    let streamed = render_readme(&facts, Some(counts), ReadmeLayout::Streamed);
    let standard = render_readme(&facts, Some(counts), ReadmeLayout::Standard);
    assert!(!streamed.contains("still streaming"));
    assert!(!streamed.contains("## Streaming status"));
    assert!(streamed.contains("`index.jsonl`"));
    assert!(!standard.contains("`index.jsonl`"));
    assert!(streamed.contains("; --- variables ---"));
    assert_eq!(
        standard,
        build_readme(&fauxware(), "/tmp/fauxware", "fauxware", &prog, &results),
        "the Standard layout is what build_readme has always written"
    );
}

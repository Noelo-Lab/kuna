//! ghidra-sim: the real-program ghidra-mode differential harness.
//!
//! Drives the FULL wire lifecycle (registerProgram → setAction → decompileAt ×
//! N → flushNative → decompileAt → deregisterProgram) against the in-process
//! [`GhidraProcess`], with the mock-Java end answered from kuna's OWN analysis
//! of real vendored ELFs (`tests/ghidra_sim/oracle.rs`): real Sleigh p-code,
//! real bytes, real labels — and, since Phase 3, real `<doc><mapsym>`/`<hole>`
//! answers for `getMappedSymbols`/`getExternalRef` built from the committed
//! program facts (the DecompileCallback role).
//!
//! The harness turned the Phase-2 "callees show as sub_ADDR, registers leak
//! into the C" GUI anecdote into **pinned numbers**, and Phase 3 landed by
//! flipping them:
//!
//!   * structure asserts (r5 schema): dual-`<function>` decode, name + entry
//!     echo, markup opref ⊆ ast op-times / varref ⊆ ast create-indices, the
//!     19-query legality + query-legal-command placement, warnings clean;
//!   * badness scanners on the markup-flattened C: raw-register identifier
//!     leaks, `Unique<hex>`/`Stack<hex>` tokens, `sub_`/`FUN_`/`dat_`/`DAT_`
//!     placeholder rate vs what the loader actually knows — all ZERO or
//!     oracle-unnamed-only since Phase 3;
//!   * query-traffic fingerprints: getPcode count vs decoded instructions,
//!     getMappedSymbols traffic (0 in Phase 2; the real query-through count
//!     since Phase 3), flushNative cache-clearing via a label override;
//!   * the differential-C gap: the markup C vs the SAME function through the
//!     in-process CLI path (`kuna decompile`'s drive) as a normalized,
//!     style-normalized line-diff ratio — the number the GUI user experiences.
//!
//! ## Pin discipline
//! The `PIN_*` constants record TODAY's measured reality.  They only move
//! together with the engine/provider change that earns the move.  NEVER
//! re-pin to absorb an unexplained regression.
//!
//! ## Runtime
//! `faillog` (23 KB, 3 functions + 1 repeat) is the fast default.  The
//! `sort`/`grep` breadth test decompiles 3 mid-size functions each and runs in
//! a few seconds in release; it is `#[ignore]`d under the default dev-profile
//! workspace suite to keep `make rust-test` lean, and run explicitly (release)
//! by the CI gates job and `make test-ghidra`.

mod ghidra_sim;

use std::cell::RefCell;
use std::collections::BTreeSet;
use std::path::Path;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::marshal::PackedEncode;

use kuna_ghidra::ids::{ELEM_COMMAND_GETMAPPEDSYMBOLS, ELEM_COMMAND_GETPCODE};
use kuna_ghidra::process::GhidraProcess;

use ghidra_sim::oracle::{
    decompile_cli, generate_tspec, repo_root, SimOracle,
};
use ghidra_sim::{
    cmd_decompile_at, cmd_deregister_program, cmd_flush_native, cmd_register_program,
    cmd_set_action, line_diff_ratio, normalized_lines, parse_decompile_doc, placeholder_addrs,
    query_doc_id, register_leaks, trace_session, unique_leaks, MockReader, MockState, MockWriter,
    ParsedDoc, SessionTrace, QUERY_COMMAND_IDS,
};

/// One driven ghidra-mode session over a vendored ELF.
struct SessionRun {
    oracle: SimOracle,
    trace: SessionTrace,
    /// Parsed decompileAt docs, one per target (the flush-repeat doc is extra).
    docs: Vec<ParsedDoc>,
    /// The raw payloads of the target decompiles (index-aligned with `docs`).
    payloads: Vec<Vec<u8>>,
    /// The raw payload of the repeat decompile of target 0 after flushNative.
    repeat_payload: Vec<u8>,
    /// Resolved target entry addresses (index-aligned with `docs`).
    addrs: Vec<Address>,
    /// The response index of the first decompileAt (1 + the setAction count).
    first_decompile: usize,
}

/// Bootstrap the oracle, drive the whole session, and split the output.
/// Returns `None` when the `.sla` specs are not built (visible skip).
fn run_session(binary: &Path, targets: &[&str]) -> Option<SessionRun> {
    run_session_with(binary, targets, |_| {})
}

/// [`run_session`] with a pre-drive oracle mutation hook (inject
/// `tracked_overrides`/`label_overrides` before the wire lifecycle runs).
fn run_session_with(
    binary: &Path,
    targets: &[&str],
    mutate: impl FnOnce(&mut SimOracle),
) -> Option<SessionRun> {
    run_session_config(binary, targets, &[("decompile", "c")], mutate)
}

/// [`run_session_with`] with an explicit setAction replay — the analyzer
/// configurations replay several `(actionstring, printstring)` pairs
/// (`DecompInterface.initializeProcess`), e.g. the switch analyzer's
/// `("decompile","")+("","noc")+("","notree")+("","jumpload")`.
fn run_session_config(
    binary: &Path,
    targets: &[&str],
    actions: &[(&str, &str)],
    mutate: impl FnOnce(&mut SimOracle),
) -> Option<SessionRun> {
    let mut oracle = SimOracle::bootstrap(binary)?;
    mutate(&mut oracle);

    let addrs: Vec<Address> = targets
        .iter()
        .map(|t| {
            oracle
                .prog
                .find_entry_by_name(t)
                .unwrap_or_else(|| panic!("{binary:?}: no function named {t}"))
                .addr
        })
        .collect();

    let tspec = generate_tspec(&oracle.manager, oracle.big_endian, oracle.unique_base);
    let lang_dir = repo_root().join("specs/Ghidra/Processors/x86/data/languages");
    let pspec = std::fs::read(lang_dir.join("x86-64.pspec")).expect("vendored x86-64.pspec");
    let cspec = std::fs::read(lang_dir.join("x86-64-gcc.cspec")).expect("vendored x86-64-gcc.cspec");
    // Phase 3 decodes the wire corespec for real: send the default-mirroring
    // full set so the ghidra-mode factory matches the oracle's (same hash ids).
    let coretypes: &[u8] = ghidra_sim::DEFAULT_CORETYPES_XML;

    let packed_addr = |a: &Address| -> Vec<u8> {
        let mut v = Vec::new();
        {
            let mut e = PackedEncode::new(&mut v);
            a.encode(&mut e).expect("entry addr encodes");
        }
        v
    };

    // registerProgram, setAction × A, decompileAt × N, flushNative,
    // decompileAt (target 0 again), deregisterProgram.
    let mut commands = Vec::new();
    cmd_register_program(&mut commands, &pspec, &cspec, &tspec, coretypes);
    for (action, printstring) in actions {
        cmd_set_action(&mut commands, "0", action, printstring);
    }
    for a in &addrs {
        cmd_decompile_at(&mut commands, "0", &packed_addr(a));
    }
    cmd_flush_native(&mut commands, "0");
    cmd_decompile_at(&mut commands, "0", &packed_addr(&addrs[0]));
    cmd_deregister_program(&mut commands, "0");
    let n_commands = 1 + actions.len() + addrs.len() + 2 + 1;

    let shared = Rc::new(RefCell::new(MockState::new(commands, oracle)));
    let reader = MockReader { shared: Rc::clone(&shared) };
    let writer = MockWriter { shared: Rc::clone(&shared) };
    let mut process = GhidraProcess::new(reader, writer);
    for i in 0..n_commands {
        let status = process
            .read_command()
            .unwrap_or_else(|e| panic!("command #{i} failed: {e:?}"));
        let expected = if i == n_commands - 1 { 1 } else { 0 };
        assert_eq!(status, expected, "command #{i} unexpected loop status");
    }
    let _ = process.into_inner();

    let state = match Rc::try_unwrap(shared) {
        Ok(cell) => cell.into_inner(),
        Err(_) => panic!("mock state still shared after into_inner"),
    };
    let oracle = state.source;
    let out = state.from_process;
    let trace = trace_session(&out);
    assert_eq!(
        trace.responses.len(),
        n_commands,
        "one command response span per command"
    );

    // registerProgram answered archid 0 with an EMPTY warnings frame (any text
    // here — construction failure, marshaling error — is a defect).
    let reg = &trace.responses[0];
    assert_eq!(reg.payload.as_deref(), Some(b"0".as_slice()), "archid echo");
    assert!(
        reg.warnings.trim().is_empty(),
        "registerProgram shipped a non-empty warnings frame: {}",
        reg.warnings
    );
    // setAction / flushNative / deregister accepted.
    for i in 0..actions.len() {
        assert_eq!(
            trace.responses[1 + i].payload.as_deref(),
            Some(b"t".as_slice()),
            "setAction #{i} not accepted"
        );
    }
    let first_decompile = 1 + actions.len();
    let flush_idx = first_decompile + addrs.len();
    assert_eq!(
        trace.responses[flush_idx].payload.as_deref(),
        Some(b"0".as_slice()),
        "flushNative result"
    );
    assert_eq!(
        trace.responses[n_commands - 1].payload.as_deref(),
        Some(b"1".as_slice()),
        "deregister result"
    );

    let mut docs = Vec::new();
    let mut payloads = Vec::new();
    for (i, a) in addrs.iter().enumerate() {
        let resp = &trace.responses[first_decompile + i];
        assert!(
            resp.warnings.trim().is_empty(),
            "decompileAt #{i} shipped a non-empty warnings frame (degradation): {}",
            resp.warnings
        );
        let payload = resp.payload.clone().expect("decompileAt payload present");
        assert!(
            !payload.is_empty(),
            "decompileAt #{i} emitted an EMPTY payload (warnings: {})",
            resp.warnings
        );
        let parsed = parse_decompile_doc(&payload, &oracle.manager);
        // Name + entry-address echo — the HighFunction.decode hard-throw traps.
        // Compared against what the sim actually SERVED (`code_label` consults
        // `label_overrides`), not the raw program lookup.  (A
        // parammeasures-only paramid doc carries no <function> to echo.)
        if parsed.has_function {
            assert_eq!(
                Some(parsed.name.as_str()),
                oracle.code_label(a.get_offset()).as_deref(),
                "decompileAt #{i}: <function name> must echo the getCodeLabel answer"
            );
            assert_eq!(
                parsed.entry_offset,
                Some(a.get_offset()),
                "decompileAt #{i}: <function> base <addr> must echo the requested entry"
            );
        }
        docs.push(parsed);
        payloads.push(payload);
    }
    let repeat_payload = trace.responses[flush_idx + 1]
        .payload
        .clone()
        .expect("repeat decompileAt payload present");

    Some(SessionRun { oracle, trace, docs, payloads, repeat_payload, addrs, first_decompile })
}

/// The r5 wire/structure contract, per decompiled function.
fn assert_structure(run: &SessionRun) {
    for (i, parsed) in run.docs.iter().enumerate() {
        assert!(
            !parsed.ast_op_times.is_empty(),
            "target #{i}: the <ast> has no ops"
        );
        assert!(parsed.has_markup, "target #{i}: markup <function> missing");
        assert!(
            parsed.markup_oprefs.is_subset(&parsed.ast_op_times),
            "target #{i}: markup oprefs not a subset of ast op times"
        );
        assert!(
            parsed.markup_varrefs.is_subset(&parsed.ast_var_refs),
            "target #{i}: markup varrefs not a subset of ast varnode refs"
        );
        // Both ref classes must be present PER CLASS (both are non-empty on
        // every real function today): an either-or here would let one whole
        // class of click-to-address links silently vanish.
        assert!(
            !parsed.markup_oprefs.is_empty(),
            "target #{i}: markup carries no oprefs — op tokens lost their ast links"
        );
        assert!(
            !parsed.markup_varrefs.is_empty(),
            "target #{i}: markup carries no varrefs — variable tokens lost their ast links"
        );
        assert!(
            !parsed.c_text.trim().is_empty(),
            "target #{i}: flattened markup C is empty"
        );
        // The ghidra-mode banner: a `Kuna v…` plate comment must open every
        // decompiled function (the user-visible "kuna is the active core"
        // marker; ghidra-mode only, so the CLI differential strips it).
        assert!(
            parsed.c_text.contains("Kuna v"),
            "target #{i}: the ghidra-mode `Kuna v…` banner comment is missing\n{}",
            parsed.c_text
        );
        // ---- Phase 4: the full-response children + the Java hard-throw traps.
        ghidra_sim::assert_phase4_traps(parsed, &format!("target #{i}"));
        let symbols = parsed
            .localdb
            .as_ref()
            .unwrap_or_else(|| panic!("target #{i}: <localdb> missing"));
        assert!(
            !symbols.is_empty(),
            "target #{i}: <localdb> carries no symbols (params/locals lost)"
        );
        let highs = parsed
            .highlist
            .as_ref()
            .unwrap_or_else(|| panic!("target #{i}: <highlist> missing"));
        assert!(
            highs.iter().any(|h| h.class == "local" || h.class == "param"),
            "target #{i}: no local/param <high> — rename/retype has no target"
        );
        let proto = parsed
            .prototype
            .as_ref()
            .unwrap_or_else(|| panic!("target #{i}: <prototype> missing"));
        assert!(
            !proto.model.is_empty() && proto.has_extrapop && proto.returnsym_ok,
            "target #{i}: <prototype> header incomplete"
        );
    }

    // Query legality: every callback query is one of the 19 command elements,
    // and queries appear ONLY inside query-legal command responses
    // (registerProgram + decompileAt; never setAction / flushNative /
    // deregisterProgram — Java nulls its callback decoder there).
    let n = run.trace.responses.len();
    let flush_idx = n - 3; // … decompileAt×N, flushNative, decompileAt, deregister
    for (idx, resp) in run.trace.responses.iter().enumerate() {
        let query_legal =
            idx == 0 || (idx >= run.first_decompile && idx != flush_idx && idx != n - 1);
        if !query_legal {
            assert!(
                resp.queries.is_empty(),
                "response #{idx}: {} callback queries during a query-illegal command",
                resp.queries.len()
            );
        }
        for q in &resp.queries {
            let id = query_doc_id(q, &run.oracle.manager);
            assert!(
                QUERY_COMMAND_IDS.contains(&id),
                "response #{idx}: query root element {id} is not one of the 19"
            );
        }
    }
}

/// How many placeholder addresses the loader actually KNOWS a real name for —
/// the symbol-resolution gap Phase 3 closes (each of these renders as
/// `sub_`/`dat_` today but has a committed symbol in the oracle).
fn resolvable_placeholders(oracle: &SimOracle, c: &str) -> usize {
    let ph = placeholder_addrs(c);
    let mut resolvable = 0usize;
    let data_syms: BTreeSet<u64> = oracle
        .prog
        .global_data_symbols()
        .into_iter()
        .map(|(_, vma, _)| vma)
        .collect();
    for (kind, addrs) in &ph {
        for a in addrs {
            let known = match *kind {
                "sub_" | "FUN_" => oracle
                    .prog
                    .function_named_at(*a)
                    .is_some_and(|n| !n.starts_with("sub_") && !n.starts_with("FUN_")),
                _ => data_syms.contains(a),
            };
            if known {
                resolvable += 1;
            }
        }
    }
    resolvable
}

/// Total placeholder occurrences (distinct addresses across all four kinds).
fn placeholder_total(c: &str) -> usize {
    placeholder_addrs(c).values().map(|s| s.len()).sum()
}

/// Drop the ghidra-mode `Kuna v…` banner line (ghidra-mode-only by design;
/// the CLI ground truth has no banner).
fn strip_banner(c: &str) -> String {
    c.lines()
        .filter(|l| !l.contains("Kuna v"))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Rewrite the ghidra-mode naming conventions onto the CLI's so the diff
/// ratio measures the SEMANTIC gap, not the deliberate Phase-3 style
/// divergence (DIV: ghidra-mode defaults): `DAT_%08x`→`dat_%x`,
/// `FUN_%08x`→`sub_%x`, `LAB_%08x`→`label_%x`, `param_<n>`→`a<n-1>`.
fn style_normalize(c: &str) -> String {
    let bytes = c.as_bytes();
    let mut out = String::with_capacity(c.len());
    let mut i = 0usize;
    while i < bytes.len() {
        let ch = bytes[i];
        if ch.is_ascii_alphabetic() || ch == b'_' {
            let start = i;
            while i < bytes.len() && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
                i += 1;
            }
            let id = &c[start..i];
            let mapped = ["DAT_", "FUN_", "LAB_"]
                .iter()
                .zip(["dat_", "sub_", "label_"])
                .find_map(|(gp, cp)| {
                    id.strip_prefix(gp)
                        .filter(|r| !r.is_empty() && r.bytes().all(|b| b.is_ascii_hexdigit()))
                        .and_then(|r| u64::from_str_radix(r, 16).ok())
                        .map(|v| format!("{cp}{v:x}"))
                })
                .or_else(|| {
                    id.strip_prefix("param_")
                        .and_then(|r| r.parse::<u64>().ok())
                        .filter(|n| *n >= 1)
                        .map(|n| format!("a{}", n - 1))
                });
            match mapped {
                Some(m) => out.push_str(&m),
                None => out.push_str(id),
            }
        } else {
            out.push(ch as char);
            i += 1;
        }
    }
    out
}

// ===========================================================================
// faillog — the fast default fixture (x86-64 PIE, stripped, 23 KB)
// ===========================================================================

/// The three fixed targets: `sub_2620` is `main` (call/global/switch-heavy),
/// `sub_3320`/`sub_3ad0` are mid-size leaf-ish functions.
const FAILLOG_TARGETS: &[&str] = &["sub_2620", "sub_3320", "sub_3ad0"];

// ---- TODAY's measured PHASE-3 pins (see "Pin discipline" above) -----------
// Re-measured 2026-08-17 with the Phase-3 providers live (real getMappedSymbols
// answers, the aggressive engine-tier preset, FUN_/DAT_/LAB_ fallback naming,
// the wire coretypes decode, the manager register lookup).  The Phase-2 GUI
// gap these pins used to record — on this base: register leaks 108/58/60,
// Unique tokens 34/4/8, resolvable placeholders 24/18/14, diff ratios
// 0.646/0.867/0.811 — is CLOSED; the pins now hold the line at the new level.
//
// Raw-register identifier occurrences in the markup C, per target
// (AL/EAX/RBP/R12/… rendered as C variables — r3 §8 defect c/d/e).  Phase 3
// drove these to ZERO (the manager register lookup restored local naming).
const PIN_FAILLOG_REGISTER_LEAKS: [usize; 3] = [0, 0, 0];
// Unique<hex>/Stack<hex> token occurrences, per target (defect e): ZERO.
const PIN_FAILLOG_UNIQUE_TOKENS: [usize; 3] = [0, 0, 0];
// Distinct placeholder addresses (sub_/FUN_/dat_/DAT_), per target
// (defects a/b).  The survivors are entities the ORACLE has no name for
// either (stripped local functions, unnamed .bss objects) — the ghidra-mode
// output now names everything the host can name.
const PIN_FAILLOG_PLACEHOLDERS: [usize; 3] = [27, 4, 3];
// …of which the loader KNOWS a real name — ZERO: every PLT import
// (localtime, getopt_long, dcgettext, …) resolves through getMappedSymbols.
const PIN_FAILLOG_RESOLVABLE: [usize; 3] = [0, 0, 0];
// The normalized line-diff ratio vs the CLI path, per target (measured 0.050 /
// 0.079 / 0.099 after style normalization — see `style_normalize`), banded
// from BOTH sides a few points off the measurement: the floor so a further
// improvement must flip the band downward deliberately, the ceiling so a
// markup regression fails instead of saturating unnoticed.
//
// Phase 4 collapsed the Phase-3 band (0.170/0.184/0.264): the getC()
// type-token mangling is gone (the EmitMarkup declarator-token split).  The
// residue is per-function analysis skew — the ghidra path decompiles one
// function against wire-fed facts while the CLI path runs after whole-binary
// analysis commits (jumptable label choices, readonly-driven const folds).
const PIN_FAILLOG_DIFF_FLOOR: [f64; 3] = [0.02, 0.04, 0.06];
const PIN_FAILLOG_DIFF_CEILING: [f64; 3] = [0.09, 0.12, 0.15];
// Normalized non-empty line count of the flattened markup C, per target: the
// structural size of what the GUI renders.  A `<break>`-token regression would
// collapse this to ~1 while leaving every ratio-floor assertion green — this
// pin is what catches it.  (sub_3320 shrank because the noreturn facts now
// truncate the flow overrun that used to decode neighbouring functions into
// it — defect g; the +1 everywhere is the `Kuna v…` banner line.)
// sub_2620 283 -> 285 with `calleearity` (DIV-102): the `getpwent()` loop's
// `sub_34a0()` regains the uid argument its two sibling call sites already pass
// — kuna's own recovered prototype for that callee is `unsigned long sub_34a0(unsigned long a0)`
// — and the uid expression `*(unsigned int *)(v7 + 0x10)` hoists into a named
// local instead of being re-spelled twice inside the guard. Two lines, both
// earned.
const PIN_FAILLOG_C_LINES: [usize; 3] = [285, 39, 92];
// Tokens Java's `getC()` cleaner REWRITES (`IllegalCharCppTransformer`).
// Phase 3 measured 57/10/24 (whole rendered declarators like
// `"unsigned long *"` as single `<type>` tokens, received by scripts/exports
// as `unsigned_long__` — the type-spelling mangling of the live repro).
// Phase 4's EmitMarkup declarator-token split (word `<type>` tokens +
// `<syntax>` separators) drives these to ZERO and must keep them there.
const PIN_FAILLOG_MANGLED_TOKENS: [usize; 3] = [0, 0, 0];
// `<vardecl symref>`s that do NOT resolve against the document's own
// `<localdb>` (the create-index fallback).  Phase 4 originally emitted the
// fallback for EVERY declaration — Java logged "Invalid symbol reference" per
// declaration and rename/retype was dead on declaration lines.  The review
// round drove these to 0/1/0; carrying the Symbol identity through the
// `&symbol` reference bind (C++ `Varnode::setSymbolReference`) closed the last
// one on sub_3320 too, so every declaration in the corpus now resolves.  ZERO
// is the contract, not a high-water mark: a new residue must be justified
// against `ClangVariableDecl.decode` (a log line + a dead rename on that one
// declaration, never a decode throw) before this pin moves up.
const PIN_FAILLOG_VARDECL_UNRESOLVED: [usize; 3] = [0, 0, 0];
// Whole-session query traffic: total getPcode asks (repeat decompiles re-ask
// everything — no p-code cache, faithful to upstream GhidraTranslate) vs
// distinct instruction addresses actually decoded.  Phase 3 REDUCED both
// (1477 → 1314, 1003 → 801): the mapsym noreturn facts stop the flow from
// decoding past `exit()`-style calls into neighbouring functions (defect g).
// (kuna `calleedeadarg`) RAISED 1314 -> 1613 and 801 -> 1044.  The P4 veto reads
// the CALLEE's body to decide whether an argument register is dead at its entry,
// and in ghidra mode every such decode is a getPcode round trip to the host.  The
// walk is bounded (192 instructions, and each path stops at the callee's first
// call or return), cached per callee entry for the whole run, and skipped
// entirely for a function with fewer than two calls — so +243 DISTINCT decoded
// instructions is the union of the probed prologues in this session, not
// per-decompile growth.  Flipping `option calleedeadarg off` restores 1314/801.
// (kuna `calleepreserves`, DIV-124) RAISED the total 1613 -> 1862 and left the
// DISTINCT count at 1044, unmoved.  That pairing is the whole justification: the
// call-guard seam takes the same bounded body walk, but without
// `calleedeadarg`'s "fewer than two calls" skip, so it also probes callees in
// functions that gate declined -- every one of which this session had already
// decoded elsewhere.  No new instruction address is read; the +249 is re-asks of
// known bytes, which cost a host round trip only because ghidra mode has no
// p-code cache (repeat decompiles re-ask everything, faithful to upstream
// GhidraTranslate).  Measured both arms on this tree: `calleepreserves off`
// gives 1613/1044, `on` gives 1862/1044.
const PIN_FAILLOG_GETPCODE_TOTAL: u64 = 1862;
const PIN_FAILLOG_DECODED_INSTS: usize = 1044;
// Whole-session getMappedSymbols traffic: Phase 2 pinned this at 0 (the
// providers did not exist); Phase 3 pins the real query-through traffic —
// every distinct global address the pipeline probes, answered once (holes and
// symbol ranges negative/positive-cache the rest).
const PIN_FAILLOG_GETMAPPED_TOTAL: u64 = 1448;

#[test]
fn ghidra_sim_faillog_pins() {
    let binary = repo_root().join("tests/bug-repro/faillog");
    let Some(run) = run_session(&binary, FAILLOG_TARGETS) else {
        return; // visible skip: specs not built
    };
    assert_structure(&run);

    // ---- measure everything first (one run = every number), assert after ----
    let mut reg_counts = Vec::new();
    let mut reg_names_seen = Vec::new();
    let mut uniq_counts = Vec::new();
    let mut ph_totals = Vec::new();
    let mut ph_resolvables = Vec::new();
    let mut mangled_counts = Vec::new();
    let mut vardecl_unresolved_counts = Vec::new();
    let mut c_line_counts = Vec::new();
    for parsed in &run.docs {
        let c = &parsed.c_text;
        let (reg_count, reg_names) = register_leaks(c, &run.oracle.register_names);
        reg_counts.push(reg_count);
        reg_names_seen.push(reg_names);
        uniq_counts.push(unique_leaks(c));
        ph_totals.push(placeholder_total(c));
        ph_resolvables.push(resolvable_placeholders(&run.oracle, c));
        mangled_counts.push(parsed.mangled_tokens);
        vardecl_unresolved_counts.push(ghidra_sim::vardecl_unresolved(parsed));
        c_line_counts.push(normalized_lines(c).len());
    }

    // The differential-C gap vs the in-process CLI path.
    let SessionRun { mut oracle, docs, addrs, .. } = run;
    let mut ratios = Vec::new();
    for (i, parsed) in docs.iter().enumerate() {
        let cli_c = decompile_cli(&mut oracle.prog, &parsed.name.clone(), &addrs[i]);
        // The banner is ghidra-mode-only by design: strip it before diffing
        // against the (banner-less) CLI ground truth.
        ratios.push(line_diff_ratio(
            &style_normalize(&strip_banner(&parsed.c_text)),
            &cli_c,
        ));
    }

    let mapped = oracle
        .log
        .counts
        .get(&ELEM_COMMAND_GETMAPPEDSYMBOLS.get_id())
        .copied()
        .unwrap_or(0);
    let pcode_total = oracle
        .log
        .counts
        .get(&ELEM_COMMAND_GETPCODE.get_id())
        .copied()
        .unwrap_or(0);

    let mut summary = String::new();
    for (i, t) in FAILLOG_TARGETS.iter().enumerate() {
        summary.push_str(&format!(
            "{t}: registers={} {:?} unique={} placeholders={} resolvable={} mangled={} \
             vardecl_unresolved={} c_lines={} diff_ratio={:.3}\n",
            reg_counts[i], reg_names_seen[i], uniq_counts[i], ph_totals[i], ph_resolvables[i],
            mangled_counts[i], vardecl_unresolved_counts[i], c_line_counts[i], ratios[i]
        ));
    }
    summary.push_str(&format!(
        "getPcode total={pcode_total} distinct-decoded={} getMappedSymbols={mapped}\n",
        oracle.log.decoded_insts.len()
    ));
    eprintln!("ghidra_sim faillog measurements:\n{summary}");

    // ---- the pins ----
    for (i, t) in FAILLOG_TARGETS.iter().enumerate() {
        assert_eq!(
            reg_counts[i], PIN_FAILLOG_REGISTER_LEAKS[i],
            "{t}: raw-register leak count moved (found {:?})\n{}",
            reg_names_seen[i], docs[i].c_text
        );
        assert_eq!(
            uniq_counts[i], PIN_FAILLOG_UNIQUE_TOKENS[i],
            "{t}: Unique/Stack token count moved\n{}",
            docs[i].c_text
        );
        assert_eq!(
            ph_totals[i], PIN_FAILLOG_PLACEHOLDERS[i],
            "{t}: placeholder count moved\n{}",
            docs[i].c_text
        );
        assert_eq!(
            ph_resolvables[i], PIN_FAILLOG_RESOLVABLE[i],
            "{t}: loader-resolvable placeholder count moved (Phase 3 drives this to 0)"
        );
        assert_eq!(
            mangled_counts[i], PIN_FAILLOG_MANGLED_TOKENS[i],
            "{t}: getC()-mangled token count moved (PR-C drives this to 0)"
        );
        assert_eq!(
            vardecl_unresolved_counts[i], PIN_FAILLOG_VARDECL_UNRESOLVED[i],
            "{t}: unresolvable <vardecl symref> count moved — declaration-line \
             rename/retype resolution changed"
        );
        assert!(
            ratios[i] >= PIN_FAILLOG_DIFF_FLOOR[i],
            "{t}: ghidra-vs-CLI diff ratio {:.3} fell below the pinned floor {} — \
             the GUI-path gap shrank!  If a provider change earned this, flip the pin \
             (and celebrate); otherwise investigate.",
            ratios[i], PIN_FAILLOG_DIFF_FLOOR[i]
        );
        assert!(
            ratios[i] <= PIN_FAILLOG_DIFF_CEILING[i],
            "{t}: ghidra-vs-CLI diff ratio {:.3} rose above the pinned ceiling {} — \
             the GUI path got WORSE (a markup/flatten regression?).",
            ratios[i], PIN_FAILLOG_DIFF_CEILING[i]
        );
        assert_eq!(
            c_line_counts[i], PIN_FAILLOG_C_LINES[i],
            "{t}: flattened-C line count moved — the rendered structure changed \
             (a <break>/token regression collapses this while ratios stay in band)"
        );
    }

    // ---- query-traffic fingerprints ----
    assert!(
        mapped >= 1,
        "getMappedSymbols never fired — the Phase-3 lazy providers are dead"
    );
    assert_eq!(
        mapped, PIN_FAILLOG_GETMAPPED_TOTAL,
        "whole-session getMappedSymbols traffic moved"
    );
    assert_eq!(
        pcode_total as usize,
        oracle.log.pcode_addrs.len(),
        "internal log consistency"
    );
    assert_eq!(
        pcode_total, PIN_FAILLOG_GETPCODE_TOTAL,
        "whole-session getPcode traffic moved"
    );
    assert_eq!(
        oracle.log.decoded_insts.len(),
        PIN_FAILLOG_DECODED_INSTS,
        "distinct decoded instruction count moved"
    );
}

/// The SWITCH-ANALYZER configuration (SwitchAnalysisDecompileConfigurer:
/// `noc` + `jumpload` + `notree`, action `decompile`): the response must be a
/// first-`<function>`-only doc — no markup, no `<ast>`, no `<highlist>` — that
/// still carries `<localdb>`, `<prototype>`, and the `<jumptablelist>` with
/// the recovered `<dest>` targets plus the jumpload-collected `<loadtable>`s
/// (`DecompilerSwitchAnalysisCmd` consumes exactly this).
#[test]
fn ghidra_sim_faillog_switch_analyzer_shape() {
    let binary = repo_root().join("tests/bug-repro/faillog");
    // sub_2620 (main) is the switch-heavy target.
    let Some(run) = run_session_config(
        &binary,
        &["sub_2620"],
        &[("decompile", ""), ("", "noc"), ("", "notree"), ("", "jumpload")],
        |_| {},
    ) else {
        return; // visible skip: specs not built
    };
    let parsed = &run.docs[0];
    ghidra_sim::assert_phase4_traps(parsed, "switch-analyzer");
    assert!(!parsed.has_markup, "noc: the markup <function> must be absent");
    assert!(
        !parsed.function_child_ids.contains(&ghidra_sim::ELEM_AST_ID),
        "notree: the <ast> must be absent"
    );
    assert!(
        !parsed.function_child_ids.contains(&ghidra_sim::ELEM_HIGHLIST_ID),
        "notree: the <highlist> must be absent"
    );
    assert!(parsed.localdb.is_some(), "the <localdb> still travels under notree");
    assert!(parsed.prototype.is_some(), "the <prototype> still travels under notree");
    assert!(
        !parsed.jumptables.is_empty(),
        "no <jumptablelist> — the switch analyzer gets nothing (children: {:?})",
        parsed.function_child_ids
    );
    let (dests, loads) = parsed.jumptables[0];
    assert!(dests >= 2, "a recovered switch needs >= 2 <dest> targets, got {dests}");
    assert!(
        loads >= 1,
        "jumpload was toggled but no <loadtable> was collected — \
         FlowInfo::record_jumploads is not reaching the recovery"
    );
}

/// The PARAM-ID ANALYZER configuration (DecompilerParameterIdCmd /
/// ConventionAnalysisDecompileConfigurer: `noc` + `notree` + `parammeasures`,
/// action `paramid`): the response is a `<parammeasures>`-ONLY doc — no
/// `<function>` at all — with the REQUIRED `<rank>` per measure.
#[test]
fn ghidra_sim_faillog_paramid_shape() {
    let binary = repo_root().join("tests/bug-repro/faillog");
    let Some(run) = run_session_config(
        &binary,
        &["sub_3320"],
        &[("paramid", ""), ("", "noc"), ("", "notree"), ("", "parammeasures")],
        |_| {},
    ) else {
        return; // visible skip: specs not built
    };
    let parsed = &run.docs[0];
    ghidra_sim::assert_phase4_traps(parsed, "paramid");
    assert!(
        !parsed.has_function,
        "paramid + parammeasures: the doc must contain ONLY <parammeasures> \
         (ghidra_process.cc:318-321), children: {:?}",
        parsed.function_child_ids
    );
    assert!(!parsed.has_markup, "paramid: no markup <function>");
    let measures = parsed
        .parammeasures
        .as_ref()
        .expect("the <parammeasures> document");
    assert!(
        !measures.is_empty(),
        "paramid produced no measures — the param-ID analyzer gets nothing"
    );
    assert!(
        measures.iter().any(|(is_input, _)| *is_input),
        "paramid produced no <input> measures"
    );
}

/// The byte size of an encoded localdb symbol: the `<type size>` when present
/// (the full form), else the trailing digits of a core `<typeref name>`
/// (`uint8` → 8; Java derives entry sizes from the type, so the harness does
/// too).
fn sim_symbol_size(s: &ghidra_sim::SimSymbol) -> Option<i64> {
    if let Some(sz) = s.type_size {
        if sz > 0 {
            return Some(sz);
        }
    }
    let name = s.type_name.as_deref()?;
    let digits: String =
        name.chars().rev().take_while(|c| c.is_ascii_digit()).collect::<Vec<_>>()
            .into_iter().rev().collect();
    if digits.is_empty() {
        return matches!(name, "bool" | "char" | "byte").then_some(1);
    }
    digits.parse::<i64>().ok().filter(|v| *v > 0 && *v <= 16)
}

/// The RENAME-PERSISTENCE loop (the GUI keystone): a user rename is a DB write
/// (`HighFunctionDBUtil.updateDBVariable`) followed by an event-driven
/// re-decompile whose getMappedSymbols answer now carries the renamed local in
/// the function's `<localdb>` (Java `LocalSymbolMap.grabFromFunction`).  kuna
/// must SEED that local into the fresh Funcdata so the new name — not a fresh
/// `vN` — renders.  Echo-back differential: run once, take a local kuna itself
/// encoded (storage + first-use), serve it back RENAMED, assert the C shows
/// the new name.
#[test]
fn ghidra_sim_faillog_rename_persistence() {
    let binary = repo_root().join("tests/bug-repro/faillog");
    let Some(run1) = run_session(&binary, &["sub_3ad0"]) else {
        return; // visible skip: specs not built
    };
    let parsed = &run1.docs[0];
    let cand = parsed
        .localdb
        .as_ref()
        .expect("<localdb> present")
        .iter()
        .find(|s| {
            s.cat < 0
                && s.entry_storage.as_ref().is_some_and(|(spc, _)| spc != "unique")
                && sim_symbol_size(s).is_some()
                && parsed.c_text.contains(s.name.as_str())
        })
        .expect("a non-unique-storage local kuna named");
    let (spc, off) = cand.entry_storage.clone().expect("mapped storage");
    let size = sim_symbol_size(cand).expect("sized type");
    let first_use = cand.first_use;
    let old_name = cand.name.clone();
    let entry_off = run1.addrs[0].get_offset();

    // typelock=false: the exact shape a plain GUI rename produces (verified
    // against a live Ghidra 12.1.2 DecompileDebug capture — `<symbol
    // typelock="false" namelock="true" cat="-1"><typeref name="undefined8">`)
    // — which kuna must carry as a NAME RECOMMENDATION, the C++
    // `ScopeLocal::nameRecommend` path.
    let renamed = "host_renamed_probe";
    let Some(run2) = run_session_with(&binary, &["sub_3ad0"], move |o| {
        o.local_var_overrides.insert(
            entry_off,
            vec![ghidra_sim::oracle::HostLocalVar {
                name: renamed.to_string(),
                space: spc,
                offset: off,
                size,
                first_use,
                typelock: false,
                hash: 0,
            }],
        );
    }) else {
        return;
    };
    let parsed2 = &run2.docs[0];
    assert!(
        parsed2.c_text.contains(renamed),
        "the host-committed rename ({old_name} -> {renamed}) did not survive \
         the re-decompile:\n{}",
        parsed2.c_text
    );
    // The re-encoded localdb carries the seeded symbol back to Java.
    assert!(
        parsed2
            .localdb
            .as_ref()
            .expect("<localdb> present")
            .iter()
            .any(|s| s.name == renamed),
        "the seeded local is missing from the re-encoded <localdb>"
    );
}


/// C2 (review round): a host-committed local whose only SymbolEntry is a
/// DYNAMIC `<hash>` — the storage class Java writes for every
/// `requiresDynamicStorage` variable (unique-space representatives,
/// `splitOutMergeGroup` products) — must survive the re-decompile.  Before the
/// fix the decoder dropped hash-entry locals outright, so a rename of such a
/// variable silently reverted in front of the user.
///
/// Echo-back differential: take a hash kuna itself encoded for a wire symbol
/// (the link pass hashes conflict-separated / uncovered variables with the
/// upstream budget Java also uses), serve it back as a renamed dynamic local,
/// and assert the name comes back in the C.
#[test]
fn ghidra_sim_faillog_dynamic_rename_persistence() {
    let binary = repo_root().join("tests/bug-repro/faillog");
    let Some(run1) = run_session(&binary, &["sub_3ad0"]) else {
        return; // visible skip: specs not built
    };
    let parsed = &run1.docs[0];
    // A wire symbol kuna encoded with a `<hash>` entry (type="dynamic").
    let cand = parsed
        .localdb
        .as_ref()
        .expect("<localdb> present")
        .iter()
        .find(|s| s.cat < 0 && s.hash != 0 && parsed.c_text.contains(s.name.as_str()));
    let Some(cand) = cand else {
        // No dynamic wire symbol on this target: nothing to assert (the
        // shape is target-dependent; the decode-side unit test covers the
        // wire contract unconditionally).
        return;
    };
    let hash = cand.hash;
    let first_use = cand.first_use;
    let old_name = cand.name.clone();
    let entry_off = run1.addrs[0].get_offset();
    let renamed = "host_dyn_renamed";
    let Some(run2) = run_session_with(&binary, &["sub_3ad0"], move |o| {
        o.local_var_overrides.insert(
            entry_off,
            vec![ghidra_sim::oracle::HostLocalVar {
                name: renamed.to_string(),
                space: "ram".to_string(),
                offset: first_use.unwrap_or(entry_off),
                size: 8,
                first_use,
                typelock: false,
                hash,
            }],
        );
    }) else {
        return;
    };
    assert!(
        run2.docs[0].c_text.contains(renamed),
        "a host rename of a DYNAMIC (hash-storage) local ({old_name} -> {renamed}) \
         did not survive the re-decompile:\n{}",
        run2.docs[0].c_text
    );
}

/// C4 (review round): no `<high>` may borrow another variable's symbol id.
/// The encode-time link pass must never re-derive a container binding — a high
/// the naming pass deliberately separated from the parameter whose storage it
/// overlaps would otherwise inherit the PARAMETER's id, and renaming that
/// variable in the GUI would rename the parameter.
///
/// Invariant asserted on every decompiled function: each `<high symref>` is
/// claimed by at most ONE high (bar the deliberate group sharing of a composite
/// symbol, where the highs also share a `<localdb>` symbol whose type is
/// composite), and every param-class high's symref is a cat-0 symbol.
#[test]
fn ghidra_sim_faillog_high_symrefs_are_not_shared_with_params() {
    let binary = repo_root().join("tests/bug-repro/faillog");
    let Some(run) = run_session(&binary, FAILLOG_TARGETS) else {
        return; // visible skip: specs not built
    };
    for (i, parsed) in run.docs.iter().enumerate() {
        let symbols = parsed.localdb.as_ref().expect("<localdb> present");
        let param_ids: BTreeSet<u64> =
            symbols.iter().filter(|s| s.cat == 0).map(|s| s.id).collect();
        let highs = parsed.highlist.as_ref().expect("<highlist> present");
        for h in highs {
            let Some(sr) = h.symref else { continue };
            if param_ids.contains(&sr) {
                assert_eq!(
                    h.class, "param",
                    "target #{i}: a non-param <high class={}> references the \
                     cat-0 parameter symbol {sr} — renaming that variable in the \
                     GUI would rename the PARAMETER",
                    h.class
                );
            }
        }
    }
}

/// flushNative + repeat-decompile stability: with unchanged host answers a
/// repeat decompile of the same entry after flushNative is byte-identical —
/// the caches clear back to a deterministic state.
#[test]
fn ghidra_sim_faillog_flush_native_stability() {
    let binary = repo_root().join("tests/bug-repro/faillog");
    let Some(run) = run_session(&binary, &["sub_3ad0"]) else {
        return; // visible skip: specs not built
    };
    assert_eq!(
        run.payloads[0], run.repeat_payload,
        "repeat decompileAt after flushNative is not byte-identical"
    );
}

/// The Phase-3 flushNative CACHE-CLEARING semantics (r4 §3.4): decompile a
/// function whose body calls a local helper, then change the "host database"
/// answer for that callee (`label_overrides`), flushNative, and decompile
/// again — the second document MUST carry the new name, proving the lazy
/// symbol cache was actually dropped (a stale cache would keep serving the
/// old decoded record and never re-query).
#[test]
fn ghidra_sim_faillog_flush_native_clears_symbol_cache() {
    let binary = repo_root().join("tests/bug-repro/faillog");
    // sub_2620 (main) calls sub_38d0 in its first statement.
    let Some(run) = run_session_with_override(
        &binary,
        "sub_2620",
        0x38d0,
        "renamed_after_flush",
    ) else {
        return; // visible skip: specs not built
    };
    let (before, after) = run;
    assert!(
        before.contains("sub_38d0"),
        "baseline decompile does not name the callee sub_38d0:\n{before}"
    );
    assert!(
        !before.contains("renamed_after_flush"),
        "baseline decompile already carries the override name"
    );
    assert!(
        after.contains("renamed_after_flush"),
        "post-flush decompile does not reflect the changed host answer — \
         flushNative did not clear the lazy symbol cache:\n{after}"
    );
}

/// Drive registerProgram → decompileAt(target) → [override the label for
/// `override_addr`] → flushNative → decompileAt(target) and return the two
/// flattened C texts.  The override is injected through the AnswerSource seam
/// mid-session via a wrapper.
fn run_session_with_override(
    binary: &Path,
    target: &str,
    override_addr: u64,
    override_name: &str,
) -> Option<(String, String)> {
    let name = override_name.to_string();
    run_flush_epoch_session(
        binary,
        target,
        |_| {},
        move |oracle| {
            oracle.label_overrides.insert(override_addr, name.clone());
        },
    )
}

/// Drive registerProgram → decompileAt(target) → flushNative →
/// decompileAt(target) → deregisterProgram and return the two flattened C
/// texts.  `pre` mutates the oracle before the drive; `at_arm` runs (on every
/// answered query, idempotently) once the FIRST decompile completed — i.e. the
/// "host database changed between the two epochs" seam the flushNative
/// cache-clearing tests exercise.  Arming by watching for the flushNative
/// boundary is impossible from inside the AnswerSource, so the driver arms a
/// shared flag after command #2 completes — race-free in the strictly
/// synchronous mock.
fn run_flush_epoch_session(
    binary: &Path,
    target: &str,
    pre: impl FnOnce(&mut SimOracle),
    at_arm: impl Fn(&mut SimOracle) + 'static,
) -> Option<(String, String)> {
    struct ArmMutate<F: Fn(&mut SimOracle)> {
        oracle: SimOracle,
        armed: std::rc::Rc<std::cell::Cell<bool>>,
        mutate: F,
    }
    impl<F: Fn(&mut SimOracle)> ghidra_sim::AnswerSource for ArmMutate<F> {
        fn respond(&mut self, doc: &[u8]) -> Vec<u8> {
            if self.armed.get() {
                (self.mutate)(&mut self.oracle);
            }
            self.oracle.respond(doc)
        }
    }

    let mut oracle = SimOracle::bootstrap(binary)?;
    pre(&mut oracle);
    let entry = oracle
        .prog
        .find_entry_by_name(target)
        .unwrap_or_else(|| panic!("{binary:?}: no function named {target}"))
        .addr;
    let tspec = generate_tspec(&oracle.manager, oracle.big_endian, oracle.unique_base);
    let lang_dir = repo_root().join("specs/Ghidra/Processors/x86/data/languages");
    let pspec = std::fs::read(lang_dir.join("x86-64.pspec")).expect("vendored x86-64.pspec");
    let cspec =
        std::fs::read(lang_dir.join("x86-64-gcc.cspec")).expect("vendored x86-64-gcc.cspec");
    let manager = Rc::clone(&oracle.manager);

    let packed_addr = |a: &Address| -> Vec<u8> {
        let mut v = Vec::new();
        {
            let mut e = PackedEncode::new(&mut v);
            a.encode(&mut e).expect("entry addr encodes");
        }
        v
    };
    let armed = Rc::new(std::cell::Cell::new(false));
    let source = ArmMutate {
        oracle,
        armed: Rc::clone(&armed),
        mutate: at_arm,
    };

    let mut commands = Vec::new();
    cmd_register_program(
        &mut commands,
        &pspec,
        &cspec,
        &tspec,
        ghidra_sim::DEFAULT_CORETYPES_XML,
    );
    cmd_set_action(&mut commands, "0", "decompile", "c");
    cmd_decompile_at(&mut commands, "0", &packed_addr(&entry));
    cmd_flush_native(&mut commands, "0");
    cmd_decompile_at(&mut commands, "0", &packed_addr(&entry));
    cmd_deregister_program(&mut commands, "0");

    let shared = Rc::new(RefCell::new(MockState::new(commands, source)));
    let reader = MockReader { shared: Rc::clone(&shared) };
    let writer = MockWriter { shared: Rc::clone(&shared) };
    let mut process = GhidraProcess::new(reader, writer);
    for i in 0..6 {
        // Arm the override once the FIRST decompileAt (command #2) completed;
        // the flush + second decompile then run against the changed answers.
        let status = process
            .read_command()
            .unwrap_or_else(|e| panic!("command #{i} failed: {e:?}"));
        if i == 2 {
            armed.set(true);
        }
        assert_eq!(status, if i == 5 { 1 } else { 0 });
    }
    let _ = process.into_inner();
    let state = match Rc::try_unwrap(shared) {
        Ok(cell) => cell.into_inner(),
        Err(_) => panic!("mock state still shared after into_inner"),
    };
    let trace = trace_session(&state.from_process);
    let first = parse_decompile_doc(
        trace.responses[2].payload.as_ref().expect("first decompile payload"),
        &manager,
    );
    let second = parse_decompile_doc(
        trace.responses[4].payload.as_ref().expect("second decompile payload"),
        &manager,
    );
    Some((first.c_text, second.c_text))
}

/// Host-side tracked-register context reaches the engine (the wired
/// ContextGhidra): a sim-served tracked value at the entry (a user 'Set
/// Register Value') must change the decompiled output — `ActionConstbase`
/// plants the constant from the WIRE answer, merged over the pspec defaults.
/// The tracked register is `RSI`, a live-in on this function, so the planted
/// constant visibly folds into the body (faillog has no string ops, so the
/// pspec's own `DF` seed has nothing to change).  Also pins that
/// getTrackedRegisters actually fires.
#[test]
fn ghidra_sim_tracked_register_reaches_output() {
    use kuna_base::space::RegisterLookup;
    let binary = repo_root().join("tests/bug-repro/faillog");
    let target = &["sub_3ad0"];
    let Some(base_run) = run_session(&binary, target) else {
        return; // visible skip: specs not built
    };
    let tracked_base = base_run
        .oracle
        .log
        .counts
        .get(&kuna_ghidra::ids::ELEM_COMMAND_GETTRACKEDREGISTERS.get_id())
        .copied()
        .unwrap_or(0);
    assert!(
        tracked_base >= 1,
        "getTrackedRegisters never fired — the ContextGhidra wiring is dead"
    );
    let Some(over_run) = run_session_with(&binary, target, |oracle| {
        let rsi = {
            let sleigh = oracle
                .prog
                .arch()
                .translate()
                .as_sleigh()
                .expect("oracle engine is a Sleigh");
            RegisterLookup::get_register(sleigh, "RSI").expect("RSI resolves")
        };
        oracle
            .tracked_overrides
            .push(kuna_sleigh::globalcontext::TrackedContext {
                loc: kuna_sleigh::translate::varnode_data_from_storage(&rsi),
                val: 0x1234,
            });
    }) else {
        return;
    };
    assert_ne!(
        base_run.docs[0].c_text, over_run.docs[0].c_text,
        "a host-side tracked RSI value did not change the output — the wire \
         tracked set is not reaching ActionConstbase"
    );
}

/// The tracked-register REVERT contract (the wire merge is applied over the
/// PRISTINE pspec layer, never over a previously-merged set): host serves a
/// tracked `RSI` override, decompile; the host then STOPS reporting it (a
/// user clearing 'Set Register Value'); after flushNative the re-decompile
/// must match the never-overridden baseline — a stale merge would keep
/// folding 0x1234 until deregisterProgram.
#[test]
fn ghidra_sim_tracked_override_reverts_after_flush() {
    use kuna_base::space::RegisterLookup;
    let binary = repo_root().join("tests/bug-repro/faillog");
    let Some(base_run) = run_session(&binary, &["sub_3ad0"]) else {
        return; // visible skip: specs not built
    };
    let baseline = base_run.docs[0].c_text.clone();
    let Some((with_override, after_revert)) = run_flush_epoch_session(
        &binary,
        "sub_3ad0",
        |oracle| {
            let rsi = {
                let sleigh = oracle
                    .prog
                    .arch()
                    .translate()
                    .as_sleigh()
                    .expect("oracle engine is a Sleigh");
                RegisterLookup::get_register(sleigh, "RSI").expect("RSI resolves")
            };
            oracle
                .tracked_overrides
                .push(kuna_sleigh::globalcontext::TrackedContext {
                    loc: kuna_sleigh::translate::varnode_data_from_storage(&rsi),
                    val: 0x1234,
                });
        },
        |oracle| {
            oracle.tracked_overrides.clear();
        },
    ) else {
        return;
    };
    assert_ne!(
        with_override, baseline,
        "the tracked RSI override did not change the first decompile"
    );
    assert_eq!(
        after_revert, baseline,
        "the post-flush decompile did not revert to the pspec-pristine \
         baseline after the host stopped reporting the tracked value — the \
         wire merge leaked into the persistent trackbase"
    );
}

// ===========================================================================
// sort + grep — the heavier breadth fixtures
// ===========================================================================

/// Mid-size functions picked once (deterministic names from the committed
/// enumeration).  Structure + scanner sanity only — the exact pins live on the
/// faillog fixture; here the harness proves breadth (bigger CFGs, jump tables,
/// more PLT traffic) without pinning every number.
#[test]
#[ignore = "heavier fixtures (~10s release, minutes in dev profile); run explicitly: \
            cargo test -p kuna-ghidra --release -- --ignored (the CI gates job and \
            `make test-ghidra` do)"]
fn ghidra_sim_sort_grep_breadth() {
    for (fixture, targets) in [
        ("tests/bug-repro/sort", &["sub_62f0", "sub_6370", "sub_63d0"][..]),
        ("tests/bug-repro/grep", &["sub_e5c0", "sub_e640", "sub_e8f0"][..]),
    ] {
        let binary = repo_root().join(fixture);
        let Some(run) = run_session(&binary, targets) else {
            return; // visible skip: specs not built
        };
        assert_structure(&run);
        for (i, parsed) in run.docs.iter().enumerate() {
            let (reg_count, reg_names) = register_leaks(&parsed.c_text, &run.oracle.register_names);
            let unresolved = ghidra_sim::vardecl_unresolved(parsed);
            eprintln!(
                "{fixture} {}: registers={reg_count} {reg_names:?} unique={} placeholders={} \
                 vardecl_unresolved={unresolved} vardecls={}",
                targets[i],
                unique_leaks(&parsed.c_text),
                placeholder_total(&parsed.c_text),
                parsed.vardecl_symrefs.len(),
            );
            // The structural assertion above only demands that SOME declaration
            // resolves; on breadth it must be ALL of them.  `assert_structure`'s
            // wholesale check went red here (100% unresolved on sort/sub_6370)
            // while the faillog fixture measured healthy, because a stack
            // aggregate reached only through `&sym` is declared off the constant
            // PTRSUB high, which carried no Symbol identity at all.  Pin the
            // count at 0: a genuine residue must be recorded per target with the
            // Java degradation spelled out (`ClangVariableDecl.decode` logs
            // "Invalid symbol reference" and returns -- a dead rename on that one
            // line, never a decode throw), not absorbed silently.
            assert_eq!(
                unresolved, 0,
                "{fixture} {}: {unresolved} of {} <vardecl symref>s do not resolve \
                 against <localdb> — declaration-line rename/retype is dead on \
                 those lines\n{}",
                targets[i],
                parsed.vardecl_symrefs.len(),
                parsed.c_text,
            );
        }
    }
}

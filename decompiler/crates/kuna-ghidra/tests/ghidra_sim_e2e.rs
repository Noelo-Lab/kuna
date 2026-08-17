//! ghidra-sim: the real-program ghidra-mode differential harness.
//!
//! Drives the FULL wire lifecycle (registerProgram → setAction → decompileAt ×
//! N → flushNative → decompileAt → deregisterProgram) against the in-process
//! [`GhidraProcess`], with the mock-Java end answered from kuna's OWN analysis
//! of real vendored ELFs (`tests/ghidra_sim/oracle.rs`): real Sleigh p-code,
//! real bytes, real labels — and, deliberately, EMPTY `getMappedSymbols`/
//! `getExternalRef`, because that is the Phase-2 reality of the GUI path.
//!
//! The harness turns "callees show as sub_ADDR, registers leak into the C"
//! from a GUI anecdote into **pinned numbers**:
//!
//!   * structure asserts (r5 schema): dual-`<function>` decode, name + entry
//!     echo, markup opref ⊆ ast op-times / varref ⊆ ast create-indices, the
//!     19-query legality + query-legal-command placement, warnings clean;
//!   * badness scanners on the markup-flattened C: raw-register identifier
//!     leaks, `Unique<hex>`/`Stack<hex>` tokens, `sub_`/`FUN_`/`dat_`/`DAT_`
//!     placeholder rate vs what the loader actually knows;
//!   * query-traffic fingerprints: getPcode count vs decoded instructions,
//!     getMappedSymbols == 0 (Phase 3 flips this to ≥ 1);
//!   * the differential-C gap: the markup C vs the SAME function through the
//!     in-process CLI path (`kuna decompile`'s drive) as a normalized
//!     line-diff ratio — the number the GUI user experiences.
//!
//! ## Pin discipline
//! The `PIN_*` constants record TODAY's measured reality (Phase 2, no symbol
//! providers).  They only move together with the engine/provider change that
//! earns the move — Phase 3 lands against this file by flipping the
//! placeholder/mapped-symbols/diff pins downward.  NEVER re-pin to absorb an
//! unexplained regression.
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
}

/// Bootstrap the oracle, drive the whole session, and split the output.
/// Returns `None` when the `.sla` specs are not built (visible skip).
fn run_session(binary: &Path, targets: &[&str]) -> Option<SessionRun> {
    let oracle = SimOracle::bootstrap(binary)?;

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
    // The wire corespec is ignored by the Phase-2 process (default core types).
    let coretypes: &[u8] =
        b"<coretypes><type name=\"int\" size=\"4\" metatype=\"int\" id=\"-1\"/></coretypes>";

    let packed_addr = |a: &Address| -> Vec<u8> {
        let mut v = Vec::new();
        {
            let mut e = PackedEncode::new(&mut v);
            a.encode(&mut e).expect("entry addr encodes");
        }
        v
    };

    // registerProgram, setAction, decompileAt × N, flushNative, decompileAt
    // (target 0 again), deregisterProgram.
    let mut commands = Vec::new();
    cmd_register_program(&mut commands, &pspec, &cspec, &tspec, coretypes);
    cmd_set_action(&mut commands, "0", "decompile", "c");
    for a in &addrs {
        cmd_decompile_at(&mut commands, "0", &packed_addr(a));
    }
    cmd_flush_native(&mut commands, "0");
    cmd_decompile_at(&mut commands, "0", &packed_addr(&addrs[0]));
    cmd_deregister_program(&mut commands, "0");
    let n_commands = 2 + addrs.len() + 2 + 1;

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
    assert_eq!(trace.responses[1].payload.as_deref(), Some(b"t".as_slice()));
    let flush_idx = 2 + addrs.len();
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
        let resp = &trace.responses[2 + i];
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
        // `label_overrides`), not the raw program lookup.
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
        docs.push(parsed);
        payloads.push(payload);
    }
    let repeat_payload = trace.responses[flush_idx + 1]
        .payload
        .clone()
        .expect("repeat decompileAt payload present");

    Some(SessionRun { oracle, trace, docs, payloads, repeat_payload, addrs })
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
    }

    // Query legality: every callback query is one of the 19 command elements,
    // and queries appear ONLY inside query-legal command responses
    // (registerProgram + decompileAt; never setAction / flushNative /
    // deregisterProgram — Java nulls its callback decoder there).
    let n = run.trace.responses.len();
    let flush_idx = n - 3; // … decompileAt×N, flushNative, decompileAt, deregister
    for (idx, resp) in run.trace.responses.iter().enumerate() {
        let query_legal = idx == 0 || (idx >= 2 && idx != flush_idx && idx != n - 1);
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

// ===========================================================================
// faillog — the fast default fixture (x86-64 PIE, stripped, 23 KB)
// ===========================================================================

/// The three fixed targets: `sub_2620` is `main` (call/global/switch-heavy),
/// `sub_3320`/`sub_3ad0` are mid-size leaf-ish functions.
const FAILLOG_TARGETS: &[&str] = &["sub_2620", "sub_3320", "sub_3ad0"];

// ---- TODAY's measured Phase-2 pins (see "Pin discipline" above) -----------
// Measured 2026-08-17 on main + this harness (empty getMappedSymbols, no
// options in ghidra mode).  These ARE the GUI-path quality gap, numerically.
//
// Raw-register identifier occurrences in the markup C, per target
// (AL/EAX/RBP/R12/… rendered as C variables — r3 §8 defect c/d/e).
const PIN_FAILLOG_REGISTER_LEAKS: [usize; 3] = [106, 64, 60];
// Unique<hex>/Stack<hex> token occurrences, per target (defect e).
const PIN_FAILLOG_UNIQUE_TOKENS: [usize; 3] = [32, 2, 8];
// Distinct placeholder addresses (sub_/FUN_/dat_/DAT_), per target
// (defects a/b).
const PIN_FAILLOG_PLACEHOLDERS: [usize; 3] = [49, 25, 17];
// …of which the loader KNOWS a real name — the PLT imports (localtime,
// getopt_long, dcgettext, …) that render as sub_ADDR only because
// getMappedSymbols is unanswered.  Phase 3 drives these to 0.
const PIN_FAILLOG_RESOLVABLE: [usize; 3] = [24, 18, 14];
// The normalized line-diff ratio vs the CLI path, per target (measured 0.643 /
// 0.898 / 0.811), banded from BOTH sides a few points off the measurement:
// the floor so a real Phase-3 improvement must flip the band downward
// deliberately, the ceiling so a markup regression that makes the GUI text
// even MORE alien (lost tokens, collapsed structure) fails instead of
// saturating unnoticed.  Unrelated CLI-side drift moves the ratio only a few
// points, which the band absorbs.
//
// What the ratio is made of — so nobody chases an unreachable ≈0 with symbol
// work alone: part is the Phase-3 symbol/type gap (sub_/dat_/register/Unique
// rendering), and part is OPTION-PRESET SKEW — the CLI side runs the
// aggressive-mode presets (`ctypes`, `stackalias`, `iteexpr`, …, applied by
// `apply_cli_default_options`) that the ghidra-mode engine cannot receive
// until setOptions is wired (Phase 3/4).  Expect Phase 3 to land in the
// ~0.1–0.3 range, not 0; the residue is the setOptions work.
const PIN_FAILLOG_DIFF_FLOOR: [f64; 3] = [0.55, 0.80, 0.70];
const PIN_FAILLOG_DIFF_CEILING: [f64; 3] = [0.70, 0.95, 0.90];
// Normalized non-empty line count of the flattened markup C, per target: the
// structural size of what the GUI renders.  A `<break>`-token regression would
// collapse this to ~1 while leaving every ratio-floor assertion green — this
// pin is what catches it.
const PIN_FAILLOG_C_LINES: [usize; 3] = [241, 174, 89];
// Tokens Java's `getC()` cleaner REWRITES (`IllegalCharCppTransformer`): kuna
// emits whole rendered declarators (`"unsigned long *"`) as single `<type>`
// tokens, which scripts/exports receive as `unsigned_long__` — the type-
// spelling mangling of the live repro.  PR-C splits the declarator into
// base-type + syntax tokens and drives these to 0.
const PIN_FAILLOG_MANGLED_TOKENS: [usize; 3] = [21, 13, 7];
// Whole-session query traffic: total getPcode asks (repeat decompiles re-ask
// everything — no p-code cache, faithful to upstream GhidraTranslate) vs
// distinct instruction addresses actually decoded (the flow overruns
// no-return calls into neighbours today — defect g — so this exceeds the real
// function sizes).
const PIN_FAILLOG_GETPCODE_TOTAL: u64 = 1477;
const PIN_FAILLOG_DECODED_INSTS: usize = 1003;

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
        c_line_counts.push(normalized_lines(c).len());
    }

    // The differential-C gap vs the in-process CLI path.
    let SessionRun { mut oracle, docs, addrs, .. } = run;
    let mut ratios = Vec::new();
    for (i, parsed) in docs.iter().enumerate() {
        let cli_c = decompile_cli(&mut oracle.prog, &parsed.name.clone(), &addrs[i]);
        ratios.push(line_diff_ratio(&parsed.c_text, &cli_c));
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
             c_lines={} diff_ratio={:.3}\n",
            reg_counts[i], reg_names_seen[i], uniq_counts[i], ph_totals[i], ph_resolvables[i],
            mangled_counts[i], c_line_counts[i], ratios[i]
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
    assert_eq!(
        mapped, 0,
        "getMappedSymbols fired {mapped}× — Phase 2 never queries it.  If Phase 3 \
         landed, flip this pin to an ≥1 assertion for the global-heavy target."
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

/// flushNative + repeat-decompile stability: TODAY flushNative clears nothing
/// (no provider caches exist), and a repeat decompile of the same entry is
/// byte-identical — the engine is deterministic across decompiles in one
/// session.
///
/// PHASE-3 FLIP: once the lazy symbol/type caches exist, this test grows the
/// cache-clearing semantics of r4 §3.4: change `oracle.label_overrides` for a
/// symbol between the two decompiles — after flushNative the second doc MUST
/// reflect the new answer.  (Today the label is re-queried per decompileAt, so
/// only whole-doc stability is meaningful to pin.)
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
            eprintln!(
                "{fixture} {}: registers={reg_count} {reg_names:?} unique={} placeholders={}",
                targets[i],
                unique_leaks(&parsed.c_text),
                placeholder_total(&parsed.c_text),
            );
        }
    }
}

//! The token source map ([`kuna_console::inspect::FuncDetail`]) over every
//! function of the browser and loader fixtures, in C and in Rust: the tokens
//! rebuild the code exactly, asking for them leaves the code byte-identical,
//! every instruction a token names is one its line maps to, and every mapped
//! instruction inside the function's listing ([`kuna_console::inspect::function_rows`])
//! starts a row of it.
//!
//! Needs the built `.sla` specs; without them each fixture prints a skip.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use kuna_console::engine::{bootstrap_from_image, ConsoleProgram};
use kuna_console::inspect::{function_rows, FUNCTION_ROW_CAP};
use kuna_console::project::{decompile_targets_with, DecompileOptions, FuncResult};
use kuna_decomp::kuna_srcmap::verify;
use kuna_decomp::prettyprint::TokenKind;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn load(binary: &str, language: Option<&str>) -> Option<ConsoleProgram> {
    let specs = vec![repo_root().join("specs").to_string_lossy().into_owned()];
    let mut prog = match bootstrap_from_image(binary, "", &specs) {
        Ok(prog) => prog,
        Err(e) => {
            eprintln!("verify_srcmap: skipping {binary}: {}", e.explain());
            return None;
        }
    };
    prog.arch_mut().apply_mode("aggressive").expect("aggressive mode");
    if let Some(language) = language {
        prog.arch_mut().set_print_language_checked(language).expect("output language");
    }
    prog.commit_pending_analysis().expect("analysis commit");
    Some(prog)
}

fn batch(
    binary: &str,
    language: Option<&str>,
    want_tokens: bool,
) -> Option<(ConsoleProgram, Vec<FuncResult>)> {
    let mut prog = load(binary, language)?;
    let targets = prog.function_entries_executable();
    let opts = DecompileOptions {
        want_tokens,
        single_target: targets.len() == 1,
        ..DecompileOptions::default()
    };
    let out = decompile_targets_with(&mut prog, targets, &opts);
    Some((prog, out))
}

fn check(binary: &str, language: Option<&str>) -> usize {
    let (Some((_, plain)), Some((prog, mapped))) =
        (batch(binary, language, false), batch(binary, language, true))
    else {
        return 0;
    };
    assert_eq!(plain.len(), mapped.len(), "{binary}: batch sizes differ");
    let mut checked = 0;
    for (p, m) in plain.iter().zip(&mapped) {
        let at = format!("{binary} [{}] {} @ 0x{:x}", language.unwrap_or("c"), m.name, m.address);
        assert_eq!(p.code, m.code, "{at}: asking for tokens changed the code");
        let Some(code) = m.code.as_deref() else { continue };
        let detail = m.detail.as_deref().unwrap_or_else(|| panic!("{at}: no detail"));
        assert_eq!(detail.tokens_error, None, "{at}:\n{code}");
        assert_eq!(verify(&detail.tokens, code), Ok(()), "{at}");
        assert!(!detail.tokens.is_empty() || code.trim().is_empty(), "{at}: no tokens");
        let lines: BTreeMap<usize, BTreeSet<u64>> = m
            .line_mappings
            .iter()
            .map(|l| (l.line_number, l.addresses.iter().copied().collect()))
            .collect();
        for t in &detail.tokens {
            if let Some(i) = t.var {
                assert_eq!(m.variables[i].name, t.text, "{at}: token var index");
            }
            if matches!(t.kind, TokenKind::Comment | TokenKind::Label) {
                continue;
            }
            if let Some(address) = t.address {
                assert!(
                    lines.get(&t.line).is_some_and(|set| set.contains(&address)),
                    "{at}: token {:?} on line {} names 0x{address:x}, not in its line mapping {:?}",
                    t.text,
                    t.line,
                    lines.get(&t.line)
                );
            }
        }
        let (rows, _) = function_rows(&prog, m.byte_address, m.size.max(0) as u64, FUNCTION_ROW_CAP);
        if let (Some(first), Some(last)) = (rows.first(), rows.last()) {
            let span = first.addr..last.addr + last.size;
            let starts: BTreeSet<u64> = rows.iter().map(|r| r.addr).collect();
            for address in lines.values().flatten().filter(|a| span.contains(a)) {
                assert!(starts.contains(address), "{at}: 0x{address:x} starts no listed row");
            }
        }
        checked += 1;
    }
    checked
}

#[test]
fn tokens_rebuild_every_fixture_function_in_c_and_rust() {
    let root = repo_root();
    let fixtures = [
        "integrations/web/test/fixtures/sample.elf",
        "integrations/web/test/fixtures/sample_aarch64.o",
        "decompiler/crates/kuna-analysis/tests/fixtures/fauxware",
        "decompiler/crates/kuna-analysis/tests/fixtures/entrymain_arm",
        "decompiler/crates/kuna-analysis/tests/fixtures/pdb_prog.exe",
    ];
    let mut total = 0;
    for fixture in fixtures {
        let path = root.join(fixture);
        let binary = path.to_str().unwrap();
        for language in [None, Some("rust-language")] {
            total += check(binary, language);
        }
    }
    eprintln!("verify_srcmap: {total} function bodies checked");
}

/// What the tokens name on the browser's example: the prototype's function
/// and parameters, a call's callee, a local's row, and literals as values.
#[test]
fn sample_tokens_name_callees_parameters_and_values() {
    let root = repo_root();
    let path = root.join("integrations/web/test/fixtures/sample.elf");
    let Some((_, out)) = batch(path.to_str().unwrap(), None, true) else { return };
    let find = |name: &str| out.iter().find(|f| f.name == name).unwrap_or_else(|| panic!("{name}"));
    let (main, sum_to) = (find("main"), find("sum_to"));
    let tokens = &main.detail.as_deref().expect("detail").tokens;
    let first = tokens.iter().find(|t| t.kind == TokenKind::FuncName).expect("main's name");
    assert_eq!((first.line, first.text.as_str(), first.decl), (1, "main", Some("function")));
    let argc = tokens.iter().find(|t| t.text == "argc").expect("argc");
    assert_eq!((argc.line, argc.decl), (1, Some("param")));
    assert!(argc.var.is_some_and(|i| main.variables[i].is_param), "{argc:?}");
    let call = tokens.iter().find(|t| t.text == "sum_to").expect("the call");
    assert_eq!(call.callee, Some(sum_to.address));
    assert!(call.address.is_some());
    assert!(tokens.iter().any(|t| t.kind == TokenKind::Value && t.text == "3"));
    let sum_tokens = &sum_to.detail.as_deref().expect("detail").tokens;
    let acc = sum_tokens.iter().find(|t| t.text == "acc" && t.decl == Some("local")).expect("acc");
    assert_eq!(sum_to.variables[acc.var.expect("acc row")].name, "acc");
    for comment in sum_tokens.iter().filter(|t| t.kind == TokenKind::Comment) {
        assert_eq!(comment.address, None, "a stack comment names no instruction: {comment:?}");
    }
}

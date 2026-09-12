//! `SleighBase::has_context_commits` over the whole vendored `.sla` corpus.
//!
//! The predicate answers "can one instruction's decode change how a DIFFERENT
//! address decodes?" — the SLEIGH `globalset` mechanism — and it is decided from
//! the constructors that were actually decoded, not from an architecture
//! allow-list. This gate pins both halves: the named languages that must answer
//! each way, and the agreement of the whole corpus with a `globalset` count over
//! each language's own source closure (`.slaspec` plus every `.sinc` it
//! `@include`s, transitively). Zero occurrences must mean `false`, and any
//! occurrence must mean `true`, for all 148 vendored languages.
//!
//! ## `.sla` precondition
//!
//! The `.sla` files are build artifacts (gitignored; `make specs`). With none
//! present the test prints that and returns — a specs-less CI is a visible skip,
//! never a false green.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_sleigh::globalcontext::ContextInternal;
use kuna_sleigh::loadimage::LoadImage;
use kuna_sleigh::sleigh::Sleigh;

struct DummyImg;

impl LoadImage for DummyImg {
    fn get_file_name(&self) -> &str {
        "dummy"
    }
    fn load_fill(&mut self, ptr: &mut [u8], _addr: &Address) -> KunaResult<()> {
        ptr.fill(0);
        Ok(())
    }
    fn get_arch_type(&self) -> Vec<u8> {
        Vec::new()
    }
    fn adjust_vma(&mut self, _adjust: i64) {}
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// Every vendored `.sla`, sorted, as `(processor, file name, path)`.
fn vendored_sla() -> Vec<(String, String, PathBuf)> {
    let procs = repo_root().join("specs/Ghidra/Processors");
    let mut out = Vec::new();
    let Ok(dirs) = std::fs::read_dir(&procs) else {
        return out;
    };
    for d in dirs.flatten() {
        let processor = d.file_name().to_string_lossy().into_owned();
        let langs = d.path().join("data/languages");
        let Ok(files) = std::fs::read_dir(&langs) else {
            continue;
        };
        for f in files.flatten() {
            let p = f.path();
            if p.extension().and_then(|e| e.to_str()) != Some("sla") {
                continue;
            }
            let name = p.file_name().unwrap().to_string_lossy().into_owned();
            out.push((processor.clone(), name, p));
        }
    }
    out.sort();
    out
}

/// The `.slaspec` of a `.sla`, plus every source it `@include`s, transitively.
fn source_closure(slaspec: &Path) -> BTreeSet<PathBuf> {
    let mut seen: BTreeSet<PathBuf> = BTreeSet::new();
    let mut stack = vec![slaspec.to_path_buf()];
    while let Some(f) = stack.pop() {
        let Ok(f) = f.canonicalize() else {
            continue;
        };
        if !seen.insert(f.clone()) {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(&f) else {
            continue;
        };
        let dir = f.parent().unwrap().to_path_buf();
        for line in text.lines() {
            let Some(rest) = line.trim_start().strip_prefix("@include") else {
                continue;
            };
            let Some(open) = rest.find('"') else { continue };
            let Some(close) = rest[open + 1..].find('"') else { continue };
            stack.push(dir.join(&rest[open + 1..open + 1 + close]));
        }
    }
    seen
}

/// How many times `globalset` appears in a language's own source closure.
fn globalset_occurrences(sla: &Path) -> Option<usize> {
    let slaspec = sla.with_extension("slaspec");
    if !slaspec.is_file() {
        return None;
    }
    let mut n = 0;
    for f in source_closure(&slaspec) {
        let Ok(text) = std::fs::read_to_string(&f) else {
            continue;
        };
        n += text.matches("globalset").count();
    }
    Some(n)
}

fn has_commits(sla: &Path) -> Option<bool> {
    let bytes = std::fs::read(sla).ok()?;
    let mut engine = Sleigh::new(Box::new(DummyImg), Box::new(ContextInternal::new()));
    engine.initialize_from_sla(&bytes).ok()?;
    Some(engine.base().has_context_commits())
}

/// Every `.sla` under these processors must answer `false` — the languages the
/// parallel discovery decode is allowed to run on.
const ALL_FALSE: &[&str] = &["x86", "AARCH64", "RISCV", "Sparc", "SuperH", "Z80"];
/// Every `.sla` under these must answer `true`.
const ALL_TRUE: &[&str] = &["ARM", "MIPS", "PowerPC", "PA-RISC"];
/// These carry `globalset` in some of their languages but not all, so the
/// processor-level claim is only that at least one answers `true`.
const SOME_TRUE: &[&str] = &["PIC", "M16C"];

#[test]
fn context_commits_agree_with_the_globalset_sources() {
    let corpus = vendored_sla();
    if corpus.is_empty() {
        eprintln!(
            "verify_context_commits: skipping (no vendored .sla; build them with `make specs`)"
        );
        return;
    }

    let mut answers: Vec<(String, String, bool, usize)> = Vec::new();
    for (processor, name, path) in &corpus {
        let Some(commits) = has_commits(path) else {
            panic!("{name}: the vendored .sla failed to load");
        };
        let Some(occurrences) = globalset_occurrences(path) else {
            panic!("{name}: no .slaspec beside the vendored .sla");
        };
        assert_eq!(
            commits,
            occurrences > 0,
            "{processor}/{name}: has_context_commits() is {commits} but its source closure \
             spells `globalset` {occurrences} times"
        );
        answers.push((processor.clone(), name.clone(), commits, occurrences));
    }

    for p in ALL_FALSE {
        let langs: Vec<_> = answers.iter().filter(|(proc, ..)| proc == p).collect();
        assert!(!langs.is_empty(), "{p}: no vendored .sla to check");
        for (_, name, commits, _) in langs {
            assert!(!commits, "{p}/{name} must be free of context commits");
        }
    }
    for p in ALL_TRUE {
        let langs: Vec<_> = answers.iter().filter(|(proc, ..)| proc == p).collect();
        assert!(!langs.is_empty(), "{p}: no vendored .sla to check");
        for (_, name, commits, _) in langs {
            assert!(commits, "{p}/{name} must carry context commits");
        }
    }
    for p in SOME_TRUE {
        let langs: Vec<_> = answers.iter().filter(|(proc, ..)| proc == p).collect();
        assert!(!langs.is_empty(), "{p}: no vendored .sla to check");
        assert!(
            langs.iter().any(|(.., commits, _)| *commits),
            "{p}: at least one language must carry context commits"
        );
    }

    // The two named languages the parallel decode's gate is written around.
    for name in ["x86.sla", "x86-64.sla"] {
        let found = answers.iter().find(|(_, n, ..)| n == name);
        let (_, _, commits, occurrences) = found.unwrap_or_else(|| panic!("{name} not vendored"));
        assert!(!commits && *occurrences == 0, "{name} must be free of context commits");
    }
}

#[test]
fn delay_slot_bytes_are_zero_off_the_delay_slot_languages() {
    let corpus = vendored_sla();
    if corpus.is_empty() {
        eprintln!("verify_context_commits: skipping (no vendored .sla)");
        return;
    }
    for name in ["x86-64.sla", "AARCH64.sla"] {
        let Some((_, _, path)) = corpus.iter().find(|(_, n, _)| n == name) else {
            continue;
        };
        let bytes = std::fs::read(path).expect("read .sla");
        let mut engine = Sleigh::new(Box::new(DummyImg), Box::new(ContextInternal::new()));
        engine.initialize_from_sla(&bytes).expect("decode .sla");
        assert_eq!(
            engine.base().max_delay_slot_bytes(),
            0,
            "{name} has no delay slots, so a decode never reaches past its instruction"
        );
    }
    // SPARC is the counter-example: a decode there folds the delay slot in.
    if let Some((_, _, path)) = corpus.iter().find(|(_, n, _)| n == "SparcV9_32.sla") {
        let bytes = std::fs::read(path).expect("read .sla");
        let mut engine = Sleigh::new(Box::new(DummyImg), Box::new(ContextInternal::new()));
        engine.initialize_from_sla(&bytes).expect("decode .sla");
        assert!(
            engine.base().max_delay_slot_bytes() > 0,
            "SparcV9_32 decodes a delay slot inside one_instruction"
        );
    }
}

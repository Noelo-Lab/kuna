//! `kuna bsim build | query | info` bodies.
//!
//! Build decompiles every function of one binary through the console's shared
//! per-function decompile step (the same one every other surface runs), signs it
//! with the decompiler's feature generator, and appends it to a corpus. Query does
//! the same for the selected function and scores it against the corpus.

use kuna_analysis::analyzers::bsim::corpus::{BsimCorpus, CorpusExecutable, IngestOutcome};
use kuna_analysis::analyzers::bsim::lshweights::{weights_file_for, LshWeights};
use kuna_analysis::analyzers::bsim::query::{query as run_query, MatchFilters, QueryOptions};
use kuna_analysis::analyzers::bsim::signature::{sign_function, settings_vector};
use serde_json::json;

use crate::args::take_value;

use super::{parse_address, parse_f64, parse_usize, resolve_weights, spec_roots};

/// `kuna bsim build <binary> -o <corpus.kbsm>`
pub(super) fn cmd_build(argv: &[String]) -> i32 {
    let mut binary: Option<String> = None;
    let mut out: Option<String> = None;
    let mut name: Option<String> = None;
    let mut max_functions: Option<usize> = None;
    let mut i = 0;
    while i < argv.len() {
        match argv[i].as_str() {
            "-o" | "--out" => match take_value(argv, &mut i, "-o") {
                Ok(v) => out = Some(v),
                Err(e) => {
                    eprintln!("kuna bsim build: {e}");
                    return 2;
                }
            },
            "--name" => match take_value(argv, &mut i, "--name") {
                Ok(v) => name = Some(v),
                Err(e) => {
                    eprintln!("kuna bsim build: {e}");
                    return 2;
                }
            },
            "--max-functions" => match parse_usize(argv, &mut i, "--max-functions") {
                Ok(n) => max_functions = Some(n),
                Err(e) => {
                    eprintln!("kuna bsim build: {e}");
                    return 2;
                }
            },
            other if other.starts_with('-') => {
                eprintln!("kuna bsim build: unknown flag {other:?}");
                return 2;
            }
            other => {
                if binary.is_some() {
                    eprintln!("kuna bsim build: one binary per run, got {other:?} too");
                    return 2;
                }
                binary = Some(other.to_string());
            }
        }
        i += 1;
    }
    let (Some(binary), Some(out)) = (binary, out) else {
        eprintln!("kuna bsim build: a <binary> and -o <corpus.kbsm> are required");
        return 2;
    };
    match build(&binary, &out, name.as_deref(), max_functions) {
        Ok(summary) => {
            println!(
                "bsim build: signed {} of {} function(s) from {} into {}",
                summary.signed, summary.considered, binary, out
            );
            if summary.no_features > 0 {
                println!("bsim build: {} function(s) had no features and were not stored", summary.no_features);
            }
            if summary.capped {
                println!("bsim build: stopped at --max-functions {}", summary.limit);
            }
            println!(
                "bsim build: corpus now holds {} function(s) from {} executable(s)",
                summary.corpus_functions, summary.corpus_executables
            );
            0
        }
        Err(e) => {
            eprintln!("kuna bsim build: {e}");
            1
        }
    }
}

/// What a build did.
struct BuildSummary {
    considered: usize,
    signed: usize,
    no_features: usize,
    capped: bool,
    limit: usize,
    corpus_functions: usize,
    corpus_executables: usize,
}

fn build(
    binary: &str,
    out: &str,
    name: Option<&str>,
    max_functions: Option<usize>,
) -> Result<BuildSummary, String> {
    // Load through the SAME driver every whole-binary surface uses. A bare
    // `bootstrap_from_object` builds an architecture but runs none of the
    // discovery passes, so the inventory it offers is whatever the loader
    // happened to map (`function_entries_executable`) -- on a static binary that
    // is the PLT stub range alone. `load_program` runs the discovery a corpus
    // must cover, which is the list `kuna functions` prints.
    let mut prog = load_corpus_program(binary)?;
    let arch_id = kuna_decomp::kuna_langfamily::language_id(&prog.arch().archid).to_string();
    let cspec = prog.arch().compiler_spec_id().unwrap_or_else(|| "unknown".to_string());
    let bytes = std::fs::read(binary).map_err(|e| format!("read {binary}: {e}"))?;
    let md5 = md5_hex(&bytes);
    let exec_name = name.map(str::to_string).unwrap_or_else(|| basename(binary));

    // The canonical inventory is the whole program: `function_entries_executable`
    // reports only the entries whose bytes the *loader* mapped as code, which on
    // a static binary is the PLT stub range and nothing else. A corpus must cover
    // every function, so this uses the same list `kuna functions` prints.
    let entries = prog.function_entries_canonical();
    let limit = max_functions.unwrap_or(entries.len().max(1));
    let mut corpus = if std::path::Path::new(out).exists() {
        let existing = std::fs::read(out).map_err(|e| format!("read {out}: {e}"))?;
        BsimCorpus::load(&existing).map_err(|e| format!("load {out}: {}", e.explain()))?
    } else {
        BsimCorpus::new(settings_vector(), arch_id.clone())
    };
    if corpus.settings() != settings_vector() {
        return Err(format!(
            "corpus {out} was built with settings {:#x}, this run uses {:#x}; \
             its stored vectors are not comparable",
            corpus.settings(),
            settings_vector()
        ));
    }

    let mut considered = 0usize;
    let mut no_features = 0usize;
    let mut signed = Vec::new();
    let mut capped = false;
    for entry in entries {
        if signed.len() >= limit {
            capped = true;
            break;
        }
        considered += 1;
        let address = entry.addr.get_offset();
        match sign_one(&mut prog, &entry, address) {
            Ok(sig) => signed.push(sig),
            Err(_reason) => no_features += 1,
        }
    }

    let exec = CorpusExecutable {
        name: exec_name.clone(),
        architecture: arch_id,
        compiler: cspec,
        md5,
        functions: signed,
    };
    let signed_count = exec.functions.len();
    let outcome = corpus.insert_executable(exec);
    let (corpus_functions, corpus_executables) = (corpus.function_count(), corpus.executables().len());
    match outcome {
        IngestOutcome::Inserted => {
            let data = corpus.serialize();
            std::fs::write(out, data).map_err(|e| format!("write {out}: {e}"))?;
        }
        // BSim treats a repeat ingest as success: the corpus already has this
        // exact executable, so there is nothing to write.
        IngestOutcome::AlreadyPresent => {
            println!("bsim build: {exec_name} is already in the corpus (same content hash); nothing added");
        }
        IngestOutcome::NameCollision => {
            return Err(format!(
                "{exec_name} is already in the corpus with different content; \
                 pass --name to store this build under another name"
            ));
        }
    }
    Ok(BuildSummary {
        considered,
        signed: signed_count,
        no_features,
        capped,
        limit,
        corpus_functions,
        corpus_executables,
    })
}

/// Decompile one entry and sign it, using the console's shared decompile step.
fn sign_one(
    prog: &mut kuna_console::engine::ConsoleProgram,
    entry: &kuna_console::engine::FunctionEntry,
    address: u64,
) -> Result<kuna_analysis::analyzers::bsim::signature::BsimSignature, String> {
    let seed = kuna_console::decompile_step::DecompileSeed {
        purpose: kuna_decomp::decompile_drive::DrivePurpose::Batch,
        mapped_symbols: &[],
        usepoint_symbols: &[],
        dynamic_symbols: &[],
        object_assertions: &[],
        pending_proto: None,
        flow_overrides: &[],
        mapped_params: &[],
    };
    let declared = prog.declared_extent(address);
    let step = kuna_console::decompile_step::decompile_one(
        prog.arch_mut(),
        &entry.name,
        entry.addr.clone(),
        declared,
        &seed,
        &[],
    );
    let fd = step.result.map_err(|e| e.explain().to_string())?;
    sign_function(&fd, &entry.name, address).map_err(|outcome| outcome.describe())
}

/// `kuna bsim query <binary> --corpus <file>`
pub(super) fn cmd_query(argv: &[String]) -> i32 {
    let mut binary: Option<String> = None;
    let mut corpus_path: Option<String> = None;
    let mut address: Option<String> = None;
    let mut name: Option<String> = None;
    let mut weights_dir: Option<String> = None;
    let mut min_similarity: Option<f64> = None;
    let mut max_matches: Option<usize> = None;
    let mut json_out = false;
    let mut i = 0;
    while i < argv.len() {
        let flag = argv[i].as_str();
        match flag {
            "--corpus" => match take_value(argv, &mut i, flag) {
                Ok(v) => corpus_path = Some(v),
                Err(e) => {
                    eprintln!("kuna bsim query: {e}");
                    return 2;
                }
            },
            "--address" => match take_value(argv, &mut i, flag) {
                Ok(v) => address = Some(v),
                Err(e) => {
                    eprintln!("kuna bsim query: {e}");
                    return 2;
                }
            },
            "--name" => match take_value(argv, &mut i, flag) {
                Ok(v) => name = Some(v),
                Err(e) => {
                    eprintln!("kuna bsim query: {e}");
                    return 2;
                }
            },
            "--weights" => match take_value(argv, &mut i, flag) {
                Ok(v) => weights_dir = Some(v),
                Err(e) => {
                    eprintln!("kuna bsim query: {e}");
                    return 2;
                }
            },
            "--min-similarity" => match parse_f64(argv, &mut i, flag) {
                Ok(v) => min_similarity = Some(v),
                Err(e) => {
                    eprintln!("kuna bsim query: {e}");
                    return 2;
                }
            },
            "--max-matches" => match parse_usize(argv, &mut i, flag) {
                Ok(v) => max_matches = Some(v),
                Err(e) => {
                    eprintln!("kuna bsim query: {e}");
                    return 2;
                }
            },
            "--json" => json_out = true,
            other if other.starts_with('-') => {
                eprintln!("kuna bsim query: unknown flag {other:?}");
                return 2;
            }
            other => {
                if binary.is_some() {
                    eprintln!("kuna bsim query: one binary per run");
                    return 2;
                }
                binary = Some(other.to_string());
            }
        }
        i += 1;
    }
    let (Some(binary), Some(corpus_path)) = (binary, corpus_path) else {
        eprintln!("kuna bsim query: a <binary> and --corpus <file> are required");
        return 2;
    };
    if address.is_none() && name.is_none() {
        eprintln!("kuna bsim query: pass --address <hex> or --name <symbol> to choose the function");
        return 2;
    }
    let options = QueryOptions {
        min_similarity: min_similarity.unwrap_or(
            kuna_analysis::analyzers::bsim::query::DEFAULT_SIMILARITY_FLOOR,
        ),
        max_matches: max_matches
            .unwrap_or(kuna_analysis::analyzers::bsim::query::DEFAULT_MAX_MATCHES),
        filters: MatchFilters::default(),
    };
    match run_query_cli(&binary, &corpus_path, address.as_deref(), name.as_deref(), weights_dir.as_deref(), &options) {
        Ok(value) => {
            if json_out {
                crate::output::emit_with_status(
                    &format!("{}\n", serde_json::to_string_pretty(&value).unwrap()),
                    0,
                )
            } else {
                print_text(&value);
                0
            }
        }
        Err(e) => {
            eprintln!("kuna bsim query: {e}");
            1
        }
    }
}

fn run_query_cli(
    binary: &str,
    corpus_path: &str,
    address: Option<&str>,
    name: Option<&str>,
    weights_dir: Option<&str>,
    options: &QueryOptions,
) -> Result<serde_json::Value, String> {
    let corpus_bytes = std::fs::read(corpus_path).map_err(|e| format!("read {corpus_path}: {e}"))?;
    let corpus =
        BsimCorpus::load(&corpus_bytes).map_err(|e| format!("load {corpus_path}: {}", e.explain()))?;
    let dir = resolve_weights(weights_dir, Some(corpus_path))?;
    // Load through the SAME driver every whole-binary surface uses. A bare
    // `bootstrap_from_object` builds an architecture but runs none of the
    // discovery passes, so the inventory it offers is whatever the loader
    // happened to map (`function_entries_executable`) -- on a static binary that
    // is the PLT stub range alone. `load_program` runs the discovery a corpus
    // must cover, which is the list `kuna functions` prints.
    let mut prog = load_corpus_program(binary)?;
    let arch_id = kuna_decomp::kuna_langfamily::language_id(&prog.arch().archid).to_string();
    let weights = load_weights(&dir, &arch_id, corpus.language())?;

    let entry = match (address, name) {
        (Some(a), _) => {
            let addr = parse_address(a)?;
            prog.resolve_entry(&kuna_console::engine::EntrySelector::Numeric(addr))
                .map_err(|e| e.to_string())?
        }
        (None, Some(n)) => prog
            .resolve_entry(&kuna_console::engine::EntrySelector::Name(n.to_string()))
            .map_err(|e| e.to_string())?,
        (None, None) => return Err("no function selected".into()),
    };
    let byte_address = entry.addr.get_offset();
    let sig = sign_one(&mut prog, &entry, byte_address)?;
    let result = run_query(&corpus, &weights, &sig, options);
    Ok(json!({
        "binary": binary,
        "corpus": corpus_path,
        "function": { "name": sig.name, "address": format!("0x{byte_address:x}") },
        "features": sig.features.len(),
        "min_similarity": result.min_similarity,
        "considered": result.considered,
        "skipped_settings": result.skipped_settings,
        "truncated": result.truncated,
        "matches": result.matches.iter().map(|m| json!({
            "name": m.name,
            "address": format!("0x{:x}", m.address),
            "executable": m.executable,
            "similarity": m.similarity,
            "significance": m.significance,
        })).collect::<Vec<_>>(),
        "reason": result.empty_reason(options.min_similarity),
    }))
}

fn print_text(value: &serde_json::Value) {
    let func = &value["function"];
    println!(
        "BSim query: {} at {} ({} feature(s))",
        func["name"].as_str().unwrap_or("?"),
        func["address"].as_str().unwrap_or("?"),
        value["features"].as_u64().unwrap_or(0)
    );
    println!(
        "  threshold: similarity >= {:.2}",
        value["min_similarity"].as_f64().unwrap_or(0.0)
    );
    let matches = value["matches"].as_array().map(Vec::as_slice).unwrap_or(&[]);
    if matches.is_empty() {
        if let Some(reason) = value["reason"].as_str() {
            println!("  {reason}");
        }
        return;
    }
    println!(
        "  matches: {} (of {} considered)",
        matches.len(),
        value["considered"].as_u64().unwrap_or(0)
    );
    for (idx, m) in matches.iter().enumerate() {
        println!(
            "  {}. {:.3} similarity, significance {:.3}: {} @ {} in {}",
            idx + 1,
            m["similarity"].as_f64().unwrap_or(0.0),
            m["significance"].as_f64().unwrap_or(0.0),
            m["name"].as_str().unwrap_or("?"),
            m["address"].as_str().unwrap_or("?"),
            m["executable"].as_str().unwrap_or("?"),
        );
    }
    if value["truncated"].as_bool() == Some(true) {
        println!("  (more matches qualified than returned; raise --max-matches)");
    }
}

/// Load the weight table for a pair of languages from `dir`.
fn load_weights(dir: &str, arch_id: &str, corpus_language: &str) -> Result<LshWeights, String> {
    let Some(file) = weights_file_for(arch_id, corpus_language) else {
        return Err(format!(
            "no weights table is defined for {arch_id} against {corpus_language}"
        ));
    };
    let path = std::path::Path::new(dir).join(file);
    let bytes = std::fs::read(&path).map_err(|e| {
        format!("cannot read the weight table {}: {e}", path.display())
    })?;
    LshWeights::parse(&bytes).map_err(|e| format!("{}: {}", path.display(), e.explain()))
}

/// `kuna bsim info <corpus.kbsm>`
pub(super) fn cmd_info(argv: &[String]) -> i32 {
    let mut path: Option<String> = None;
    let mut json_out = false;
    for arg in argv {
        match arg.as_str() {
            "--json" => json_out = true,
            other if other.starts_with('-') => {
                eprintln!("kuna bsim info: unknown flag {other:?}");
                return 2;
            }
            other => path = Some(other.to_string()),
        }
    }
    let Some(path) = path else {
        eprintln!("kuna bsim info: a <corpus.kbsm> path is required");
        return 2;
    };
    let bytes = match std::fs::read(&path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("kuna bsim info: read {path}: {e}");
            return 1;
        }
    };
    let corpus = match BsimCorpus::load(&bytes) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("kuna bsim info: {}: {}", path, e.explain());
            return 1;
        }
    };
    if json_out {
        let value = json!({
            "corpus": path,
            "settings": format!("{:#x}", corpus.settings()),
            "language": corpus.language(),
            "executables": corpus.executables().iter().map(|e| json!({
                "name": e.name,
                "architecture": e.architecture,
                "compiler": e.compiler,
                "md5": e.md5,
                "functions": e.functions.len(),
            })).collect::<Vec<_>>(),
            "functions": corpus.function_count(),
        });
        crate::output::emit_with_status(&format!("{}\n", serde_json::to_string_pretty(&value).unwrap()), 0)
    } else {
        println!(
            "BSim corpus: {} (settings {:#x}, language {}, {} function(s))",
            path,
            corpus.settings(),
            corpus.language(),
            corpus.function_count()
        );
        for exec in corpus.executables() {
            println!(
                "  {}\t{}\t{}\t{}\t{} function(s)",
                exec.name,
                exec.architecture,
                exec.compiler,
                exec.md5,
                exec.functions.len()
            );
        }
        0
    }
}

/// Load `binary` through the standard whole-binary driver, so discovery runs and
/// the entry inventory is the one `kuna functions` reports.
fn load_corpus_program(binary: &str) -> Result<kuna_console::engine::ConsoleProgram, String> {
    let mut args = crate::decompile_all::Args::serial();
    args.binary = binary.to_string();
    // `Decompile`, not `Functions`: this surface decompiles every entry, and the
    // inventory-only bundle skips the analysis that produces the varnodes and
    // symbols a signature measures. With the wrong bundle every function signed
    // as if it were a bare stub.
    crate::decompile_all::load_program(&args, crate::decompile_all::DriverDefaults::Decompile)
}

fn basename(path: &str) -> String {
    std::path::Path::new(path)
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string())
}

/// The MD5 of a byte stream, lowercase hex (BSim's executable fingerprint).
///
/// BSim keys an executable by its MD5 (`ExecutableRecord`'s content hash) and
/// deduplicates an ingest on it, so the field has to be MD5 rather than any
/// modern digest: a corpus built by Ghidra and one built here agree on identity
/// only if both hash the same way. RFC 1321, written out because the workspace
/// has no MD5 dependency and this is the only place that needs one.
fn md5_hex(bytes: &[u8]) -> String {
    const S: [u32; 64] = [
        7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 5, 9, 14, 20, 5, 9, 14, 20, 5,
        9, 14, 20, 5, 9, 14, 20, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 6,
        10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
    ];
    let k: Vec<u32> = (0..64)
        .map(|i| ((i as f64 + 1.0).sin().abs() * 4294967296.0) as u32)
        .collect();
    let mut a0: u32 = 0x6745_2301;
    let mut b0: u32 = 0xefcd_ab89;
    let mut c0: u32 = 0x98ba_dcfe;
    let mut d0: u32 = 0x1032_5476;

    let mut message = bytes.to_vec();
    let bit_len = (bytes.len() as u64).wrapping_mul(8);
    message.push(0x80);
    while message.len() % 64 != 56 {
        message.push(0);
    }
    message.extend_from_slice(&bit_len.to_le_bytes());

    for chunk in message.chunks_exact(64) {
        let m: Vec<u32> = (0..16)
            .map(|i| u32::from_le_bytes([chunk[i * 4], chunk[i * 4 + 1], chunk[i * 4 + 2], chunk[i * 4 + 3]]))
            .collect();
        let (mut a, mut b, mut c, mut d) = (a0, b0, c0, d0);
        for i in 0..64 {
            let (f, g) = match i {
                0..=15 => ((b & c) | (!b & d), i),
                16..=31 => ((d & b) | (!d & c), (5 * i + 1) % 16),
                32..=47 => (b ^ c ^ d, (3 * i + 5) % 16),
                _ => (c ^ (b | !d), (7 * i) % 16),
            };
            let f2 = f
                .wrapping_add(a)
                .wrapping_add(k[i])
                .wrapping_add(m[g]);
            a = d;
            d = c;
            c = b;
            b = b.wrapping_add(f2.rotate_left(S[i]));
        }
        a0 = a0.wrapping_add(a);
        b0 = b0.wrapping_add(b);
        c0 = c0.wrapping_add(c);
        d0 = d0.wrapping_add(d);
    }
    let mut out = String::with_capacity(32);
    for word in [a0, b0, c0, d0] {
        for byte in word.to_le_bytes() {
            out.push_str(&format!("{byte:02x}"));
        }
    }
    out
}

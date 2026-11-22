//! `kuna bsim build | query | info` bodies.
//!
//! Build and query sign normalized IR under the local corpus's fixed profile.

use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use kuna_analysis::analyzers::bsim::corpus::{BsimCorpus, CorpusExecutable, IngestOutcome};
use kuna_analysis::analyzers::bsim::lshweights::{LshProfile, LshWeights};
use kuna_analysis::analyzers::bsim::query::{query as run_query, MatchFilters, QueryOptions};
use kuna_analysis::analyzers::bsim::signature::{
    sign_function_with_settings, BsimSignature, SignatureOutcome,
};
use serde_json::json;

use crate::args::take_value;

use super::{parse_address, parse_f64, parse_usize};

/// `kuna bsim build <binary> -o <corpus.kbsm>`
pub(super) fn cmd_build(argv: &[String]) -> i32 {
    let mut binary: Option<String> = None;
    let mut out: Option<String> = None;
    let mut name: Option<String> = None;
    let mut max_functions: Option<usize> = None;
    let mut profile = None;
    let mut weights_dir = None;
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
                Ok(n) if n > 0 => max_functions = Some(n),
                Ok(_) => {
                    eprintln!("kuna bsim build: --max-functions must be positive");
                    return 2;
                }
                Err(e) => {
                    eprintln!("kuna bsim build: {e}");
                    return 2;
                }
            },
            "--profile" => match take_value(argv, &mut i, "--profile") {
                Ok(v) if v == "nosize" => profile = Some(LshProfile::NoSize),
                Ok(v) => {
                    eprintln!("kuna bsim build: unknown --profile {v:?}; use nosize or omit --profile");
                    return 2;
                }
                Err(e) => {
                    eprintln!("kuna bsim build: {e}");
                    return 2;
                }
            },
            "--weights" => match take_value(argv, &mut i, "--weights") {
                Ok(v) => weights_dir = Some(v),
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
    match build(&binary, Path::new(&out), name.as_deref(), max_functions, profile, weights_dir.as_deref()) {
        Ok(summary) => {
            println!(
                "bsim build: signed {} of {} function(s) from {} into {}",
                summary.signed, summary.considered, binary, out
            );
            summary.skipped.report();
            if summary.capped {
                println!("bsim build: reached --max-functions {}; rerun with a higher cap or without it", max_functions.unwrap());
            }
            if summary.outcome == IngestOutcome::AlreadyPresent {
                println!("bsim build: same content hash already in corpus; nothing added");
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
    skipped: SigningOmissions,
    capped: bool,
    outcome: IngestOutcome,
    corpus_functions: usize,
    corpus_executables: usize,
}

fn build(
    binary: &str,
    out: &Path,
    name: Option<&str>,
    max_functions: Option<usize>,
    requested_profile: Option<LshProfile>,
    weights_dir: Option<&str>,
) -> Result<BuildSummary, String> {
    let (mut prog, md5) = load_corpus_program_with_md5(binary)?;
    let arch_id = kuna_decomp::kuna_langfamily::language_id(&prog.arch().archid).to_owned();
    let cspec = prog.arch().compiler_spec_id()
        .ok_or("binary has no resolved compiler spec")?.to_owned();
    let exec_name = name.map(str::to_owned).unwrap_or_else(|| basename(binary));
    let target = corpus_target(out)?;
    // Lock a stable sidecar, not the corpus inode that the atomic rename
    // replaces. Leave the sidecar in place for the next writer.
    let _lock = lock_corpus(&target)?;
    let existing = match fs::read(&target) {
        Ok(data) => Some(BsimCorpus::load(&data).map_err(|e| format!("load {}: {}", out.display(), e.explain()))?),
        Err(e) if e.kind() == io::ErrorKind::NotFound => None,
        Err(e) => return Err(format!("read {}: {e}", out.display())),
    };
    let profile = match &existing {
        Some(corpus) => {
            let profile = corpus.profile();
            if requested_profile.is_some_and(|requested| requested != profile) {
                return Err(format!("cannot change corpus profile {}; build a new corpus for nosize", profile.name()));
            }
            profile
        }
        None => requested_profile.or_else(|| LshProfile::for_language(&arch_id))
            .ok_or_else(|| format!("no BSim weights profile supports {arch_id}"))?,
    };
    check_language(profile, &arch_id)?;
    let weights = load_weights(weights_dir, out, profile)?;
    let mut corpus = match existing {
        Some(corpus) => {
            corpus.check_weights(&weights).map_err(|e| e.explain().to_string())?;
            corpus
        }
        None => BsimCorpus::new(weights.settings as u32, profile)
            .map_err(|e| e.explain().to_string())?,
    };
    let previous = corpus.executables().iter().find(|e| e.md5 == md5);
    if let Some(previous) = previous {
        if previous.architecture != arch_id || previous.compiler != cspec {
            return Err("executable's resolved language or compiler changed; rebuild the corpus".into());
        }
        if previous.complete {
            return Ok(BuildSummary {
                considered: 0,
                signed: 0,
                skipped: SigningOmissions::default(),
                capped: false,
                outcome: IngestOutcome::AlreadyPresent,
                corpus_functions: corpus.function_count(),
                corpus_executables: corpus.executables().len(),
            });
        }
    } else if corpus.executables().iter().any(|e| e.name == exec_name) {
        return Err(format!("{exec_name} is already in the corpus with different content; pass --name to store this build under another name"));
    }
    let previous_count = previous.map_or(0, |e| e.functions.len());
    let limit = max_functions.unwrap_or(usize::MAX);

    // Use the same canonical inventory as `kuna functions`, excluding import
    // pointer slots rather than trying to sign bytes that are not code.
    let entries = prog.function_entries_canonical();
    let mut skipped = SigningOmissions::default();
    let mut considered = 0usize;
    let mut signed = Vec::new();
    let mut capped = false;
    for entry in entries {
        let address = entry.addr.get_offset();
        if previous.is_some_and(|e| e.function_at(address).is_some()) {
            continue;
        }
        if !prog.is_body_entry(address) {
            skipped.without_body += 1;
            continue;
        }
        if previous_count + signed.len() >= limit {
            capped = true;
            break;
        }
        considered += 1;
        match sign_one(&mut prog, &entry, corpus.settings()) {
            Ok(sig) => signed.push(sig),
            Err(SignFailure::Signature(SignatureOutcome::NoFeatures)) => skipped.no_features += 1,
            Err(SignFailure::Normalize(reason)) => skipped.normalization.record(&entry.name, address, reason),
            Err(SignFailure::Signature(SignatureOutcome::Refused(reason))) => skipped.refused.record(&entry.name, address, reason),
        }
    }
    if previous.is_none() && signed.is_empty() {
        skipped.report();
        return Err(format!("{exec_name}: no functions were signed; nothing stored"));
    }

    let exec = CorpusExecutable {
        name: exec_name,
        architecture: arch_id,
        compiler: cspec,
        md5,
        complete: !capped && skipped.normalization.count == 0 && skipped.refused.count == 0,
        functions: signed,
    };
    let signed_count = exec.functions.len();
    let outcome = corpus.insert_executable(exec).map_err(|e| e.explain().to_string())?;
    match outcome {
        IngestOutcome::Inserted | IngestOutcome::Updated => write_corpus(&target, &corpus)?,
        IngestOutcome::AlreadyPresent => {}
        IngestOutcome::NameCollision => {
            return Err("executable name is already present with different content; use --name".into());
        }
    }
    Ok(BuildSummary {
        considered,
        signed: signed_count,
        skipped,
        capped,
        outcome,
        corpus_functions: corpus.function_count(),
        corpus_executables: corpus.executables().len(),
    })
}

#[derive(Default)]
struct FailureSummary {
    count: usize,
    first: Option<String>,
}

impl FailureSummary {
    fn record(&mut self, function: &str, address: u64, reason: String) {
        self.count += 1;
        if self.first.is_none() {
            self.first = Some(format!("{function} at 0x{address:x}: {reason}"));
        }
    }
}

#[derive(Default)]
struct SigningOmissions {
    without_body: usize,
    no_features: usize,
    normalization: FailureSummary,
    refused: FailureSummary,
}

impl SigningOmissions {
    fn report(&self) {
        if self.without_body > 0 {
            println!("bsim build: {} entry/entries without a local body were not stored", self.without_body);
        }
        if self.no_features > 0 {
            println!("bsim build: {} function(s) had no features and were not stored", self.no_features);
        }
        for (stage, failure) in [("normalization failed", &self.normalization), ("signature generator refused", &self.refused)] {
            if let Some(first) = &failure.first {
                eprintln!("bsim build: incomplete ingest: {stage} for {} function(s); first: {first}", failure.count);
            }
        }
    }
}

enum SignFailure {
    Normalize(String),
    Signature(SignatureOutcome),
}

impl SignFailure {
    fn describe(&self) -> String {
        match self {
            Self::Normalize(reason) => format!("normalization failed ({reason})"),
            Self::Signature(outcome) => outcome.describe(),
        }
    }
}

/// Match the normalize action Ghidra uses for feature extraction, not C emission.
fn sign_one(
    prog: &mut kuna_console::engine::ConsoleProgram,
    entry: &kuna_console::engine::FunctionEntry,
    settings: u32,
) -> Result<BsimSignature, SignFailure> {
    let address = entry.addr.get_offset();
    let declared = prog.declared_extent(address);
    let fd = kuna_decomp::decompile_drive::normalize_function(
        prog.arch_mut(), &entry.name, entry.addr.clone(), declared,
    ).map_err(|e| SignFailure::Normalize(e.explain().to_string()))?;
    sign_function_with_settings(&fd, &entry.name, address, settings).map_err(SignFailure::Signature)
}

fn corpus_dir(path: &Path) -> &Path {
    path.parent().filter(|p| !p.as_os_str().is_empty()).unwrap_or(Path::new("."))
}

fn corpus_target(path: &Path) -> Result<PathBuf, String> {
    match path.canonicalize() {
        Ok(target) => Ok(target),
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            let name = path.file_name().ok_or_else(|| format!("invalid corpus path {}", path.display()))?;
            let dir = corpus_dir(path).canonicalize()
                .map_err(|e| format!("open corpus directory for {}: {e}", path.display()))?;
            Ok(dir.join(name))
        }
        Err(e) => Err(format!("resolve {}: {e}", path.display())),
    }
}

fn lock_corpus(out: &Path) -> Result<File, String> {
    let mut path = out.as_os_str().to_os_string();
    path.push(".lock");
    let path = PathBuf::from(path);
    let lock = OpenOptions::new().read(true).write(true).create(true).truncate(false).open(&path)
        .map_err(|e| format!("open corpus lock {}: {e}", path.display()))?;
    lock.lock().map_err(|e| format!("lock {}: {e}", path.display()))?;
    Ok(lock)
}

fn write_corpus(out: &Path, corpus: &BsimCorpus) -> Result<(), String> {
    let pending = (|| -> io::Result<()> {
        let mut pending = tempfile::NamedTempFile::new_in(corpus_dir(out))?;
        pending.write_all(&corpus.serialize())?;
        pending.as_file().sync_all()?;
        pending.persist(out)?;
        #[cfg(unix)]
        File::open(corpus_dir(out))?.sync_all()?;
        Ok(())
    })();
    pending.map_err(|e| format!("replace {}: {e}", out.display()))
}

/// Query a local corpus, PostgreSQL BSim, or a native Kuna BSim server.
pub(super) fn cmd_query(argv: &[String]) -> i32 {
    let mut binary: Option<String> = None;
    let mut corpus_path: Option<String> = None;
    let mut postgres = false;
    let mut server = None;
    let mut server_profile = None;
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
            "--postgres" => postgres = true,
            "--server" => match take_value(argv, &mut i, flag) {
                Ok(v) => server = Some(v),
                Err(e) => {
                    eprintln!("kuna bsim query: {e}");
                    return 2;
                }
            },
            "--profile" => match take_value(argv, &mut i, flag).ok().and_then(|v| LshProfile::from_name(&v)) {
                Some(profile) => server_profile = Some(profile),
                None => {
                    eprintln!("kuna bsim query: --profile requires a recognized BSim profile");
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
    let Some(binary) = binary else {
        eprintln!("kuna bsim query: a <binary> is required");
        return 2;
    };
    if usize::from(postgres) + usize::from(corpus_path.is_some()) + usize::from(server.is_some()) != 1
        || (corpus_path.is_none() && weights_dir.is_some())
        || (server.is_none() && server_profile.is_some())
    {
        eprintln!("kuna bsim query: choose --corpus <file> [--weights <dir>], --postgres, or --server <url> [--profile PROFILE]");
        return 2;
    }
    if (postgres || server.is_some()) && address.is_some() && name.is_some() {
        eprintln!("kuna bsim query: choose --address or --name, not both");
        return 2;
    }
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
    let selected = (address.as_deref(), name.as_deref());
    let result = if let Some(path) = corpus_path {
        run_query_cli(&binary, &path, selected.0, selected.1, weights_dir.as_deref(), &options)
    } else if let Some(server) = server {
        let max = kuna_bsim_server::MAX_MATCHES;
        if !(0.0..=1.0).contains(&options.min_similarity) || !(1..=max).contains(&options.max_matches) {
            eprintln!("kuna bsim query: Kuna server requires similarity 0..=1 and max matches 1..={max}");
            return 2;
        }
        run_server_query(&binary, &server, selected, server_profile, &options)
    } else {
        if !(0.0..=1.0).contains(&options.min_similarity) || !(1..=10_000).contains(&options.max_matches) {
            eprintln!("kuna bsim query: PostgreSQL requires similarity 0..=1 and max matches 1..=10000");
            return 2;
        }
        run_postgres_query(&binary, selected.0, selected.1, &options)
    };
    match result {
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
            eprintln!("kuna bsim query: {}", super::TerminalText(&e));
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
    let profile = corpus.profile();
    let mut prog = load_corpus_program(binary)?;
    let arch_id = kuna_decomp::kuna_langfamily::language_id(&prog.arch().archid);
    check_language(profile, arch_id)?;
    let weights = load_weights(weights_dir, Path::new(corpus_path), profile)?;
    corpus.check_weights(&weights).map_err(|e| e.explain().to_string())?;

    let entry = select_entry(&prog, address, name)?;
    let byte_address = entry.addr.get_offset();
    if !prog.is_body_entry(byte_address) {
        return Err("selected entry does not have a local function body".into());
    }
    let sig = sign_one(&mut prog, &entry, corpus.settings()).map_err(|e| e.describe())?;
    let result = run_query(&corpus, &weights, &sig, options).map_err(|e| e.explain().to_string())?;
    Ok(json!({
        "binary": binary,
        "corpus": corpus_path,
        "function": { "name": sig.name, "address": format!("0x{byte_address:x}") },
        "features": sig.features.len(),
        "min_similarity": result.min_similarity,
        "considered": result.considered,
        "profile": profile.name(),
        "settings": format!("{:#x}", corpus.settings()),
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

fn run_server_query(
    binary: &str,
    server: &str,
    selected: (Option<&str>, Option<&str>),
    requested: Option<LshProfile>,
    options: &QueryOptions,
) -> Result<serde_json::Value, String> {
    use super::server_client::{Client, MatchId};
    use kuna_bsim_server::{QueryFilters, QueryRequest};

    let mut prog = load_corpus_program(binary)?;
    let entry = select_entry(&prog, selected.0, selected.1)?;
    let offset = entry.addr.get_offset();
    if !prog.is_body_entry(offset) {
        return Err("selected entry does not have a local function body".into());
    }
    let language = kuna_decomp::kuna_langfamily::language_id(&prog.arch().archid);
    let client = Client::new(server)?;
    let profiles = client.profiles()?;
    let profile = requested.or_else(|| LshProfile::for_language(language).filter(|default| {
        profiles.profiles.iter().any(|info| info.executables != 0 && info.profile == default.name())
    })).or_else(|| {
        profiles.profiles.iter().filter(|info| info.executables != 0).find_map(|info| {
            LshProfile::from_name(&info.profile).filter(|p| p.admits(language))
        })
    }).or_else(|| LshProfile::for_language(language))
        .ok_or_else(|| format!("Kuna BSim server has no profile for {language}"))?;
    check_language(profile, language)?;
    let info = profiles.profiles.iter().find(|info| info.profile == profile.name())
        .ok_or_else(|| format!("Kuna BSim server does not provide profile {}", profile.name()))?;
    if !profile.admits_settings(info.settings) {
        return Err("Kuna BSim server returned unsupported signature settings".into());
    }
    let sig = sign_one(&mut prog, &entry, info.settings).map_err(|e| e.describe())?;
    let feature_count = sig.features.len();
    let query = QueryRequest {
        profile: info.profile.clone(),
        settings: info.settings,
        features: sig.features,
        min_similarity: options.min_similarity,
        max_matches: options.max_matches,
        filters: QueryFilters::default(),
    };
    let response = client.query(&query)?;
    let matches = response.matches.iter().map(|hit| {
        let hit_function = &hit.function;
        let match_id = MatchId { profile, sha256: &hit_function.sha256, address: hit_function.address };
        let mut value = json!({
            "match_id": match_id.to_string(),
            "name": hit_function.name,
            "address": format!("{:#x}", hit_function.address),
            "executable": hit_function.executable,
            "architecture": hit_function.architecture,
            "compiler": hit_function.compiler,
            "executable_md5": hit_function.md5,
            "sha256": hit_function.sha256,
            "complete": hit_function.complete,
            "similarity": hit.similarity,
            "significance": hit.significance,
        });
        if let Some(slice) = hit_function.macho_slice.as_ref() {
            value["macho_slice"] = json!(slice);
        }
        value
    }).collect::<Vec<_>>();
    Ok(json!({
        "binary": binary,
        "backend": "kuna-bsim-server",
        "function": { "name": sig.name, "address": format!("{:#x}", prog.output_code_offset(offset)) },
        "features": feature_count,
        "min_similarity": response.min_similarity,
        "profile": info.profile,
        "settings": format!("{:#x}", info.settings),
        "considered": response.considered,
        "incomplete_executables": response.incomplete_executables,
        "truncated": response.truncated,
        "reason": matches.is_empty().then_some("no indexed Kuna BSim match exceeded the thresholds"),
        "matches": matches,
    }))
}

fn run_postgres_query(
    binary: &str,
    address: Option<&str>,
    name: Option<&str>,
    options: &QueryOptions,
) -> Result<serde_json::Value, String> {
    use super::postgres;
    let mut prog = load_corpus_program(binary)?;
    let entry = select_entry(&prog, address, name)?;
    let offset = entry.addr.get_offset();
    let display_address = prog.output_code_offset(offset);
    if !prog.is_body_entry(offset) {
        return Err("selected entry does not have a local function body".into());
    }
    let mut client = postgres::connect()?;
    let mut tx = postgres::read_only(&mut client)?;
    let settings = postgres::settings(&mut tx)?;
    let declared = prog.declared_extent(offset);
    let fd = kuna_decomp::decompile_drive::normalize_function(
        prog.arch_mut(), &entry.name, entry.addr, declared,
    ).map_err(|e| e.explain().to_string())?;
    let sig = kuna_analysis::analyzers::bsim::signature::sign_function_with_settings(
        &fd, &entry.name, offset, settings,
    ).map_err(|e| e.describe())?;
    let (matches, truncated) = postgres::search(
        &mut tx, &sig.features, options.min_similarity, options.max_matches,
    )?;
    let reason = matches.is_empty().then_some("no indexed BSim match exceeded the thresholds");
    Ok(json!({
        "binary": binary,
        "backend": "postgres",
        "function": { "name": sig.name, "address": format!("0x{display_address:x}") },
        "features": sig.features.len(),
        "min_similarity": options.min_similarity,
        "truncated": truncated,
        "matches": matches,
        "reason": reason,
    }))
}

pub(super) fn select_entry(
    prog: &kuna_console::engine::ConsoleProgram,
    address: Option<&str>,
    name: Option<&str>,
) -> Result<kuna_console::engine::FunctionEntry, String> {
    use kuna_console::engine::EntrySelector;
    let selector = match (address, name) {
        (Some(address), _) => EntrySelector::Numeric(parse_address(address)?),
        (None, Some(name)) => EntrySelector::Name(name.to_owned()),
        (None, None) => return Err("no function selected".into()),
    };
    prog.resolve_entry(&selector).map_err(|e| e.to_string())
}

fn print_text(value: &serde_json::Value) {
    let func = &value["function"];
    println!(
        "BSim query: {} at {} ({} feature(s))",
        func["name"].as_str().unwrap_or("?"),
        func["address"].as_str().unwrap_or("?"),
        value["features"].as_u64().unwrap_or(0)
    );
    let comparison = if value["backend"] == "postgres" { ">" } else { ">=" };
    println!("  threshold: similarity {comparison} {:.2}", value["min_similarity"].as_f64().unwrap_or(0.0));
    let matches = value["matches"].as_array().map(Vec::as_slice).unwrap_or(&[]);
    if matches.is_empty() {
        if let Some(reason) = value["reason"].as_str() {
            println!("  {reason}");
        }
        return;
    }
    if let Some(considered) = value["considered"].as_u64() {
        println!("  matches: {} (of {considered} considered)", matches.len());
    } else {
        println!("  matches: {}", matches.len());
    }
    for (idx, m) in matches.iter().enumerate() {
        println!(
            "  {}. {:.3} similarity, significance {:.3}: {} @ {} in {}",
            idx + 1,
            m["similarity"].as_f64().unwrap_or(0.0),
            m["significance"].as_f64().unwrap_or(0.0),
            super::TerminalText(m["name"].as_str().unwrap_or("?")),
            super::TerminalText(m["address"].as_str().unwrap_or("?")),
            super::TerminalText(m["executable"].as_str().unwrap_or("?")),
        );
        if let Some(id) = m["match_id"].as_str() {
            println!("     match ID: {}", super::TerminalText(id));
        }
    }
    if value["truncated"].as_bool() == Some(true) {
        println!("  (more matches qualified than returned)");
    }
}

fn check_language(profile: LshProfile, language: &str) -> Result<(), String> {
    if !profile.admits(language) {
        return Err(format!(
            "language {language} is incompatible with corpus profile {}; choose a compatible corpus or rebuild native inputs with --profile nosize",
            profile.name(),
        ));
    }
    Ok(())
}

/// Resolve the exact file rather than inferring a weights directory from a
/// different profile's file.
fn load_weights(dir: Option<&str>, corpus: &Path, profile: LshProfile) -> Result<LshWeights, String> {
    let dir = dir.map(PathBuf::from)
        .or_else(|| std::env::var_os("KUNA_BSIM_WEIGHTS").filter(|v| !v.is_empty()).map(PathBuf::from));
    let path = dir.as_deref().unwrap_or_else(|| corpus_dir(corpus)).join(profile.weights_file());
    let bytes = match fs::read(&path) {
        Ok(bytes) => bytes,
        Err(e) => {
            // Do not hide broken symlinks or other unreadable adjacent overrides.
            if dir.is_none()
                && e.kind() == io::ErrorKind::NotFound
                && matches!(fs::symlink_metadata(&path), Err(e) if e.kind() == io::ErrorKind::NotFound)
            {
                return profile.bundled_weights().map_err(|e| {
                    format!("bundled weight table {}: {}", profile.weights_file(), e.explain())
                });
            }
            return Err(format!(
                "cannot read weight table {}: {e}; pass --weights <dir> or set KUNA_BSIM_WEIGHTS",
                path.display()
            ));
        }
    };
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
            "profile": corpus.profile().name(),
            "executables": corpus.executables().iter().map(|e| json!({
                "name": e.name,
                "architecture": e.architecture,
                "compiler": e.compiler,
                "md5": e.md5,
                "complete": e.complete,
                "functions": e.functions.len(),
            })).collect::<Vec<_>>(),
            "functions": corpus.function_count(),
        });
        crate::output::emit_with_status(&format!("{}\n", serde_json::to_string_pretty(&value).unwrap()), 0)
    } else {
        println!(
            "BSim corpus: {} (settings {:#x}, profile {}, {} function(s))",
            path,
            corpus.settings(),
            corpus.profile().name(),
            corpus.function_count()
        );
        for exec in corpus.executables() {
            println!(
                "  {}\t{}\t{}\t{}\t{} function(s){}",
                exec.name,
                exec.architecture,
                exec.compiler,
                exec.md5,
                exec.functions.len(),
                if exec.complete { "" } else { " (incomplete)" },
            );
        }
        0
    }
}

/// Load `binary` through the standard whole-binary driver for discovery.
pub(super) fn load_corpus_program(binary: &str) -> Result<kuna_console::engine::ConsoleProgram, String> {
    load_corpus_program_with_source(binary, None)
}

pub(super) fn load_corpus_program_with_md5(
    binary: &str,
) -> Result<(kuna_console::engine::ConsoleProgram, String), String> {
    use md5::{Digest, Md5};
    let mut md5 = None;
    let prog = load_corpus_program_with_source(binary, Some(&mut |bytes| {
        md5 = Some(format!("{:032x}", u128::from_be_bytes(Md5::digest(bytes).into())));
    }))?;
    Ok((prog, md5.ok_or("loaded image did not supply executable bytes")?))
}

fn load_corpus_program_with_source(
    binary: &str,
    observe_source: Option<crate::decompile_all::SourceObserver<'_>>,
) -> Result<kuna_console::engine::ConsoleProgram, String> {
    let mut args = crate::decompile_all::Args::serial();
    args.binary = binary.to_string();
    // `Decompile`, not `Functions`: this surface decompiles every entry, and the
    // inventory-only bundle skips the analysis that produces the varnodes and
    // symbols a signature measures. With the wrong bundle every function signed
    // as if it were a bare stub.
    crate::decompile_all::load_program_with_source(
        &args, crate::decompile_all::DriverDefaults::Decompile, observe_source,
    )
}

fn basename(path: &str) -> String {
    std::path::Path::new(path)
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signature_loader_keeps_the_observed_executable_when_path_changes() {
        let repo = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
        let original = repo.join("tests/fixtures/fid/callees/prog");
        let replacement = repo.join("decompiler/crates/kuna-analysis/tests/fixtures/x86listingswitch_i386");
        let scratch = tempfile::tempdir().unwrap();
        let path = scratch.path().join("program");
        let prepared = scratch.path().join("replacement");
        fs::copy(&original, &path).unwrap();
        fs::copy(replacement, &prepared).unwrap();
        let expected = fs::read(original).unwrap();
        let prog = load_corpus_program_with_source(path.to_str().unwrap(), Some(&mut |bytes| {
            assert_eq!(bytes, expected);
            fs::rename(&prepared, &path).unwrap();
        })).unwrap();
        assert_eq!(kuna_decomp::kuna_langfamily::language_id(&prog.arch().archid), "x86:LE:64:default");
        assert!(prog.is_body_entry(0x401000));
    }
}

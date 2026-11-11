//! Developer CLI: decompile class files, jars and directories with kuna-jvm. Run with `--help`.

use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::rc::Rc;
use std::sync::Arc;

use kuna_jvm::classes_processor::ClassNodeType;
use kuna_jvm::decompiler_context::DecompilerContext;
use kuna_jvm::fernflower::{canonical_option, DecompiledClass, Fernflower};
use kuna_jvm::i_fernflower_logger::{IFernflowerLogger, PrintStreamLogger};
use kuna_jvm::i_fernflower_preferences::IFernflowerPreferences;
use kuna_jvm::kuna_corpus::{overlay_option, reset_engine_thread, with_engine_stack, Thrown};
use kuna_jvm::kuna_driver::{self, DriverConfig, Jobs, Strategy};
use kuna_jvm::kuna_input::{self, LoadOptions, SharedRuntime};
use kuna_jvm::kuna_partition::RootSelection;
use kuna_jvm::rt::try_catch;

const USAGE: &str = "\
usage: jvmdec [--opt key=value]... [--lib PATH]... [--runtime SPEC] [--log trace|info|warn|error]
              [--jobs N|auto [--strategy closure|components] | --naive N] [-o DIR] <class|jar|dir>...

  --opt key=value   a Vineflower option (long or three-letter name, one option either way; later wins)
  --lib PATH        a library: classes used for context, not decompiled
  --runtime SPEC    include-runtime: a JDK home, `current`/`1`, or `0`/`` for none
                    (default: Vineflower's, the current JDK)
  --log LEVEL       log level on stderr (default: warn)
  -o DIR            write each class to DIR/<path> instead of printing it
  --jobs N|auto     decompile one input with the parallel driver on N engine instances
                    (output identical to the single-instance run); prints a report on stderr
  --strategy S      the driver's batching (default closure)
  --naive N         experiment: split the roots into N contiguous parts decompiled by independent
                    instances with no dependency handling (shows what splitting alone changes)

Prints every decompiled class to stdout under a `// ---- <path> ----` header.
Exit status: 0 if every class produced output, 1 if any failed, 2 on usage errors.";

struct Args {
    options: Vec<(String, String)>,
    libs: Vec<PathBuf>,
    inputs: Vec<PathBuf>,
    out: Option<PathBuf>,
    jobs: Option<Jobs>,
    strategy: Strategy,
    naive: Option<usize>,
}

fn parse_args() -> Result<Args, String> {
    let mut a = Args {
        options: vec![(
            IFernflowerPreferences::LOG_LEVEL.to_string(),
            "warn".to_string(),
        )],
        libs: Vec::new(),
        inputs: Vec::new(),
        out: None,
        jobs: None,
        strategy: Strategy::Closure,
        naive: None,
    };
    let mut it = std::env::args().skip(1);
    while let Some(arg) = it.next() {
        let mut value = |name: &str| it.next().ok_or(format!("{name} needs a value"));
        match arg.as_str() {
            "--opt" => {
                let kv = value("--opt")?;
                let Some((k, v)) = kv.split_once('=').filter(|(k, _)| !k.is_empty()) else {
                    return Err(format!("--opt expects key=value, got {kv}"));
                };
                let (k, v) = canonical_option(k, v);
                overlay_option(&mut a.options, &k, &v);
            }
            "--lib" => a.libs.push(PathBuf::from(value("--lib")?)),
            "--runtime" => {
                let spec = value("--runtime")?;
                overlay_option(
                    &mut a.options,
                    IFernflowerPreferences::INCLUDE_JAVA_RUNTIME,
                    &spec,
                );
            }
            "--log" => {
                let level = value("--log")?.to_lowercase();
                if !matches!(level.as_str(), "trace" | "info" | "warn" | "error") {
                    return Err(format!("--log: unknown level {level}"));
                }
                overlay_option(&mut a.options, IFernflowerPreferences::LOG_LEVEL, &level);
            }
            "-o" => a.out = Some(PathBuf::from(value("-o")?)),
            "--jobs" => {
                let v = value("--jobs")?;
                a.jobs = Some(if v == "auto" {
                    Jobs::Auto
                } else {
                    Jobs::Exactly(
                        v.parse()
                            .map_err(|_| format!("--jobs: not a number: {v}"))?,
                    )
                });
            }
            "--strategy" => {
                a.strategy = match value("--strategy")?.as_str() {
                    "closure" => Strategy::Closure,
                    "components" => Strategy::Components,
                    other => return Err(format!("--strategy: unknown {other}")),
                }
            }
            "--naive" => {
                let v = value("--naive")?;
                a.naive = Some(
                    v.parse()
                        .map_err(|_| format!("--naive: not a number: {v}"))?,
                );
            }
            "-h" | "--help" => {
                println!("{USAGE}");
                std::process::exit(0);
            }
            s if s.starts_with('-') && s.len() > 1 => return Err(format!("unknown flag {s}")),
            input => a.inputs.push(PathBuf::from(input)),
        }
    }
    if a.inputs.is_empty() {
        return Err("no inputs".to_string());
    }
    for p in a.inputs.iter().chain(&a.libs) {
        if !p.exists() {
            return Err(format!("{} does not exist", p.display()));
        }
    }
    Ok(a)
}

struct Outcome {
    units: Vec<DecompiledClass>,
    roots: Vec<String>,
    error: Option<Thrown>,
}

fn decompile(args: &Args) -> Outcome {
    reset_engine_thread();
    let logger: Rc<dyn IFernflowerLogger> = Rc::new(PrintStreamLogger::stderr());
    let mut units = Vec::new();
    let mut roots = Vec::new();
    let result = try_catch(|| {
        let mut fernflower = Fernflower::new(&args.options, logger);
        let run = try_catch(|| {
            for lib in &args.libs {
                fernflower.add_library_file(lib);
            }
            for input in &args.inputs {
                fernflower.add_source_file(input);
            }
            fernflower.decompile_context(&mut |unit| units.push(unit));
            roots = DecompilerContext::get_class_processor()
                .get_map_root_classes()
                .borrow()
                .iter()
                .filter(|(_, node)| node.with(|d| d.type_) == ClassNodeType::Root)
                .map(|(name, _)| name.clone())
                .collect();
        });
        let cleared = try_catch(|| fernflower.clear_context());
        run.err().or(cleared.err())
    });
    let error = match result {
        Ok(e) => e,
        Err(e) => Some(e),
    };
    reset_engine_thread();
    Outcome {
        units,
        roots,
        error: error.as_ref().map(Thrown::from_caught),
    }
}

fn write_unit(out: &Path, unit: &DecompiledClass) -> std::io::Result<()> {
    let path = out.join(&unit.path);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, &unit.content)
}

fn option_value(args: &Args, key: &str) -> Option<String> {
    args.options
        .iter()
        .rev()
        .find(|(k, _)| k == key)
        .map(|(_, v)| v.clone())
}

fn driver_config(args: &Args) -> Result<DriverConfig, String> {
    let options = args.options.clone();
    let mut config = DriverConfig::new(
        options,
        Arc::new(|| Rc::new(PrintStreamLogger::stderr()) as Rc<dyn IFernflowerLogger>),
    );
    config.strategy = args.strategy;
    let runtime = option_value(args, IFernflowerPreferences::INCLUDE_JAVA_RUNTIME)
        .unwrap_or_else(|| "1".to_string());
    if !(runtime.is_empty() || runtime == "0") {
        config.runtime = Some(Arc::new(SharedRuntime::open(&runtime)?));
    }
    for lib in &args.libs {
        let set = kuna_input::load_library(lib).map_err(|e| e.to_string())?;
        config.libraries.push(Arc::new(set));
    }
    if let Some(seconds) = option_value(args, IFernflowerPreferences::MAX_PROCESSING_METHOD) {
        config.max_method_seconds = seconds.parse().unwrap_or(0);
    }
    Ok(config)
}

fn run_driver(args: &Args, jobs: Jobs) -> Outcome {
    let failed = |message: String| Outcome {
        units: Vec::new(),
        roots: Vec::new(),
        error: Some(Thrown {
            class: None,
            message,
            location: String::new(),
        }),
    };
    if args.inputs.len() != 1 {
        return failed("--jobs takes exactly one input".to_string());
    }
    let input = match kuna_input::load(&args.inputs[0], &LoadOptions { siblings: false }) {
        Ok(set) => set,
        Err(e) => return failed(e.to_string()),
    };
    let mut config = match driver_config(args) {
        Ok(c) => c,
        Err(e) => return failed(e),
    };
    config.jobs = jobs;
    let mut units = Vec::new();
    match kuna_driver::run(&input, &config, &mut |result| {
        if let Some(unit) = result.unit {
            units.push(unit);
        }
    }) {
        Ok(report) => {
            eprintln!(
                "jvmdec: driver: {} roots, {} workers, {} batches, {} instances, {} roots processed, {} repeated, {} divergent reads, {} leaks, plan {} ms, total {} ms",
                report.roots.len(),
                report.workers,
                report.batches,
                report.instances,
                report.processed,
                report.repeated,
                report.divergent_reads.len(),
                report.leaks.len(),
                report.plan_millis,
                report.millis
            );
            for read in &report.divergent_reads {
                eprintln!("jvmdec: divergent read: {read:?}");
            }
            for run in &report.runs {
                eprintln!(
                    "jvmdec: batch: delivered {} processed {} round {} load {} ms run {} ms",
                    run.delivered, run.processed, run.round, run.load_millis, run.millis
                );
            }
            Outcome {
                units,
                roots: report.roots,
                error: None,
            }
        }
        Err(e) => failed(e.to_string()),
    }
}

/// The experiment of `--naive`: contiguous parts, one independent instance each.
fn run_naive(args: &Args, parts: usize) -> Outcome {
    let input =
        kuna_input::load(&args.inputs[0], &LoadOptions { siblings: false }).expect("load input");
    let config = driver_config(args).expect("config");
    let roots: Vec<String> = with_engine_stack(|| {
        reset_engine_thread();
        let mut ff = Fernflower::new(&args.options, Rc::new(PrintStreamLogger::stderr()));
        for s in input.sources() {
            ff.add_source(s);
        }
        let roots = ff.root_classes();
        ff.clear_context();
        reset_engine_thread();
        roots
    });
    let per = roots.len().div_ceil(parts.max(1)).max(1);
    let chunks: Vec<Vec<String>> = roots.chunks(per).map(|c| c.to_vec()).collect();
    let mut units = Vec::new();
    std::thread::scope(|scope| {
        let handles: Vec<_> = chunks
            .iter()
            .map(|chunk| {
                let input = &input;
                let config = &config;
                std::thread::Builder::new()
                    .stack_size(kuna_jvm::kuna_corpus::engine_stack_bytes())
                    .spawn_scoped(scope, move || {
                        kuna_jvm::rt::stack::enter(kuna_jvm::kuna_corpus::engine_stack_bytes());
                        reset_engine_thread();
                        let mut options = config.options.clone();
                        options.push(("include-runtime".to_string(), "0".to_string()));
                        let mut ff =
                            Fernflower::new(&options, Rc::new(PrintStreamLogger::stderr()));
                        if let Some(rt) = &config.runtime {
                            for s in rt.sources() {
                                ff.add_library(s);
                            }
                        }
                        for s in input.sources() {
                            ff.add_source(s);
                        }
                        let mut out = Vec::new();
                        let selection = RootSelection::only(chunk.iter().cloned());
                        let _ =
                            try_catch(|| ff.decompile_selected(&selection, &mut |u| out.push(u)));
                        let _ = try_catch(|| ff.clear_context());
                        reset_engine_thread();
                        out
                    })
                    .expect("spawn")
            })
            .collect();
        for h in handles {
            units.extend(h.join().expect("join"));
        }
    });
    Outcome {
        units,
        roots,
        error: None,
    }
}

fn main() -> ExitCode {
    let args = match parse_args() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("error: {e}\n{USAGE}");
            return ExitCode::from(2);
        }
    };
    let outcome = if let Some(parts) = args.naive {
        run_naive(&args, parts)
    } else if let Some(jobs) = args.jobs {
        run_driver(&args, jobs)
    } else {
        with_engine_stack(|| decompile(&args))
    };

    let mut failed = false;
    for unit in &outcome.units {
        match &args.out {
            Some(out) => {
                if let Err(e) = write_unit(out, unit) {
                    eprintln!(
                        "jvmdec: cannot write {}: {e}",
                        out.join(&unit.path).display()
                    );
                    failed = true;
                }
            }
            None => {
                println!("// ---- {} ----", unit.path);
                println!("{}", unit.content);
            }
        }
    }
    if let Some(t) = &outcome.error {
        eprintln!("jvmdec: the decompiler threw: {}", t.message);
        eprintln!("  at {}", t.location);
        failed = true;
    }
    let mut missing: Vec<&String> = outcome
        .roots
        .iter()
        .filter(|name| !outcome.units.iter().any(|u| &u.qualified_name == *name))
        .collect();
    missing.sort();
    for name in &missing {
        eprintln!("jvmdec: no output for class {name}");
    }
    failed |= !missing.is_empty();
    if outcome.error.is_some() {
        eprintln!(
            "jvmdec: {} class(es) decompiled before the decompiler failed",
            outcome.units.len()
        );
    } else {
        eprintln!(
            "jvmdec: {} class(es) decompiled, {} failed",
            outcome.units.len(),
            missing.len()
        );
    }
    if failed {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    }
}

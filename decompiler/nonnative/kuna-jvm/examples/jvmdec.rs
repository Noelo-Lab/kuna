//! Developer CLI: decompile class files, jars and directories with kuna-jvm. Run with `--help`.

use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::rc::Rc;

use kuna_jvm::classes_processor::ClassNodeType;
use kuna_jvm::decompiler_context::DecompilerContext;
use kuna_jvm::fernflower::{DecompiledClass, Fernflower};
use kuna_jvm::i_fernflower_logger::{IFernflowerLogger, PrintStreamLogger};
use kuna_jvm::i_fernflower_preferences::IFernflowerPreferences;
use kuna_jvm::kuna_corpus::{overlay_option, reset_engine_thread, with_engine_stack, Thrown};
use kuna_jvm::rt::try_catch;

const USAGE: &str = "\
usage: jvmdec [--opt key=value]... [--lib PATH]... [--runtime SPEC] [--log trace|info|warn|error]
              [-o DIR] <class|jar|dir>...

  --opt key=value   a Vineflower option (long or three-letter name; later wins)
  --lib PATH        a library: classes used for context, not decompiled
  --runtime SPEC    include-runtime: a JDK home, `current`/`1`, or `0`/`` for none
                    (default: Vineflower's, the current JDK)
  --log LEVEL       log level on stderr (default: warn)
  -o DIR            write each class to DIR/<path> instead of printing it

Prints every decompiled class to stdout under a `// ---- <path> ----` header.
Exit status: 0 if every class produced output, 1 if any failed, 2 on usage errors.";

struct Args {
    options: Vec<(String, String)>,
    libs: Vec<PathBuf>,
    inputs: Vec<PathBuf>,
    out: Option<PathBuf>,
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
                overlay_option(&mut a.options, k, v);
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

fn main() -> ExitCode {
    let args = match parse_args() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("error: {e}\n{USAGE}");
            return ExitCode::from(2);
        }
    };
    let outcome = with_engine_stack(|| decompile(&args));

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

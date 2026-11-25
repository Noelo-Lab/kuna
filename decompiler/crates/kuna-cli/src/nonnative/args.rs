//! The managed subcommands' command lines: one parser, a flag table per subcommand.
//!
//! A flag only the native engine understands is refused (exit 2) rather than
//! ignored, `--option` names resolve through the engine's option table, and a
//! native option name is pointed at `kuna catalog --engine <engine>`.

use regex::Regex;

use kuna_managed::{Engine, Kind};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Sub {
    Decompile,
    DecompileAll,
    Project,
    Functions,
}

impl Sub {
    pub(crate) fn parse(name: &str) -> Option<Sub> {
        match name {
            "decompile" => Some(Sub::Decompile),
            "decompile-all" => Some(Sub::DecompileAll),
            "decompile-project" => Some(Sub::Project),
            "functions" => Some(Sub::Functions),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) enum SortKey {
    #[default]
    Addr,
    Size,
    Name,
}

/// The triage flags of `decompile-all` and `functions`, over members.
#[derive(Clone, Debug, Default)]
pub(crate) struct Triage {
    pub(crate) filter: Option<Regex>,
    pub(crate) min_size: Option<u64>,
    pub(crate) max_size: Option<u64>,
    pub(crate) limit: Option<usize>,
    pub(crate) sort: Option<SortKey>,
}

impl Triage {
    pub(crate) fn narrows(&self) -> bool {
        self.filter.is_some()
            || self.min_size.is_some()
            || self.max_size.is_some()
            || self.limit.is_some()
            || self.sort.is_some()
    }
}

pub(crate) struct Args {
    pub(crate) engine: &'static dyn Engine,
    pub(crate) input: String,
    pub(crate) target: Option<String>,
    pub(crate) json: bool,
    pub(crate) omit_line_mappings: bool,
    pub(crate) omit_classes: bool,
    /// The engine's `(key, value)` pairs, in command-line order.
    pub(crate) options: Vec<(String, String)>,
    pub(crate) max_fn_seconds: u64,
    pub(crate) max_fn_memory: Option<u64>,
    /// `None`: `auto`.
    pub(crate) jobs: Option<usize>,
    pub(crate) libs: Vec<String>,
    pub(crate) functions: Option<Vec<String>>,
    pub(crate) triage: Triage,
    pub(crate) output: Option<String>,
}

/// Flags only the native engine has; each is refused on a managed input.
const NATIVE_ONLY: &[&str] = &[
    "--isa",
    "--slice",
    "--image",
    "--target",
    "--base",
    "--entry",
    "--addr",
    "--define-function",
    "--assert",
    "--assert-strict",
    "--kassert",
    "--raw",
    "--regions",
    "--coverage",
    "--sleighpath",
    "--decomp-dbg",
    "--engine",
    "--timeout",
    "--stream",
    "--progress",
    "--jobs-chunk",
    "--jobs-full-load",
    "--jobs-proto",
    "--jobs-provenance",
    "--jobs-types",
    "--jobs-callees",
    "--jobs-pointerargs",
    "--jobs-objects",
    "--jobs-synth",
];

fn take(argv: &[String], i: &mut usize, flag: &str) -> Result<String, String> {
    crate::args::take_value(argv, i, flag)
}

fn count(value: &str, flag: &str) -> Result<u64, String> {
    value
        .trim()
        .parse::<u64>()
        .map_err(|_| format!("invalid {flag} value {value:?} (expected a non-negative integer)"))
}

/// `sub`'s command line `argv` on `input`, or the usage error that refuses it (exit 2).
pub(crate) fn parse(
    engine: &'static dyn Engine,
    sub: Sub,
    argv: &[String],
    input: &str,
    kind: &Kind,
) -> Result<Args, String> {
    let mut args = Args {
        engine,
        input: input.to_string(),
        target: None,
        json: false,
        omit_line_mappings: false,
        omit_classes: false,
        options: Vec::new(),
        max_fn_seconds: kuna_console::project::DEFAULT_FN_BUDGET_SECONDS,
        max_fn_memory: None,
        jobs: None,
        libs: Vec::new(),
        functions: None,
        triage: Triage::default(),
        output: None,
    };
    let (name, docs) = (engine.name(), super::docs(engine));
    let subject = format!("{input} is a {}", super::input_label(engine, kind));
    let decompiles = sub != Sub::Functions;
    let triageable = matches!(sub, Sub::DecompileAll | Sub::Functions);
    let has_json = sub != Sub::Project;
    let mut seen_input = false;
    let mut language: Option<String> = None;
    let mut i = 0;
    while i < argv.len() {
        let a = argv[i].as_str();
        match a {
            "--json" if has_json => args.json = true,
            "--no-vars" if matches!(sub, Sub::Decompile | Sub::DecompileAll) => {}
            "--omit" if matches!(sub, Sub::Decompile | Sub::DecompileAll) => {
                omit(&mut args, &take(argv, &mut i, "--omit")?)?;
            }
            "--option" => {
                if i + 2 >= argv.len() {
                    return Err("--option requires NAME VALUE".into());
                }
                let (option, value) = (&argv[i + 1], &argv[i + 2]);
                i += 2;
                match engine.resolve(option, value) {
                    Ok(pair) => args.options.push(pair),
                    Err(_) if !super::catalog::knows(engine, option) && crate::optname::is_known(option) => {
                        return Err(format!(
                            "option {option}: a native-engine option; {subject}, whose options \
                             `kuna catalog --engine {name}` lists"
                        ));
                    }
                    Err(e) => return Err(e),
                }
            }
            "--mode" => {
                let mode = take(argv, &mut i, "--mode")?;
                if !kuna_decomp::modes::mode_names().any(|m| m == mode) {
                    let names: Vec<&str> = kuna_decomp::modes::mode_names().collect();
                    return Err(format!("unknown --mode {mode:?} (expected {})", names.join("|")));
                }
            }
            "--language" => {
                let value = take(argv, &mut i, "--language")?;
                engine.language(&value, &subject, &mut Vec::new())?;
                language = Some(value);
            }
            "--lib" => args.libs.push(take(argv, &mut i, "--lib")?),
            "--jobs" => {
                let (n, auto) = crate::jobs::parse_jobs(&take(argv, &mut i, "--jobs")?)?;
                args.jobs = (!auto).then_some(n);
            }
            "--max-fn-seconds" if decompiles => {
                args.max_fn_seconds = count(&take(argv, &mut i, "--max-fn-seconds")?, "--max-fn-seconds")?;
            }
            "--max-fn-memory" if decompiles => {
                args.max_fn_memory =
                    Some(crate::decompile_all::parse_max_fn_memory(&take(argv, &mut i, "--max-fn-memory")?)?);
            }
            "--functions" if matches!(sub, Sub::DecompileAll | Sub::Project) => {
                let v = take(argv, &mut i, "--functions")?;
                args.functions = Some(
                    v.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect(),
                );
            }
            "--filter" if triageable => {
                let v = take(argv, &mut i, "--filter")?;
                args.triage.filter =
                    Some(Regex::new(&v).map_err(|e| format!("invalid --filter regex {v:?}: {e}"))?);
            }
            "--min-size" if triageable => {
                args.triage.min_size = Some(count(&take(argv, &mut i, "--min-size")?, "--min-size")?);
            }
            "--max-size" if triageable => {
                args.triage.max_size = Some(count(&take(argv, &mut i, "--max-size")?, "--max-size")?);
            }
            "--limit" if triageable => {
                args.triage.limit = Some(count(&take(argv, &mut i, "--limit")?, "--limit")? as usize);
            }
            "--sort" if triageable => {
                let v = take(argv, &mut i, "--sort")?;
                args.triage.sort = Some(match v.as_str() {
                    "addr" => SortKey::Addr,
                    "size" => SortKey::Size,
                    "name" => SortKey::Name,
                    _ => return Err(format!("invalid --sort {v:?} (expected addr|size|name)")),
                });
            }
            "--summary" | "--reachable-from" if sub == Sub::Functions => {
                return Err(format!(
                    "{a}: {subject}, and the {} engine has no call graph yet; \
                     `kuna functions {input} --json` lists every method ({docs})",
                    engine.label()
                ));
            }
            "-o" | "--output" if sub == Sub::Project => args.output = Some(take(argv, &mut i, a)?),
            "--json" | "--no-vars" if sub == Sub::Project => {
                return Err(format!("{a} is not a decompile-project option"));
            }
            _ if NATIVE_ONLY.contains(&a) || a.starts_with("--image=") => {
                let flag = a.split_once('=').map_or(a, |(flag, _)| flag);
                return Err(format!(
                    "{flag} is a native-engine flag; {subject}, decompiled by the {} engine ({docs})",
                    engine.label()
                ));
            }
            s if s.starts_with('-') && s.len() > 1 => {
                return Err(format!("unknown option {s}"));
            }
            operand => {
                if !seen_input && operand == input {
                    seen_input = true;
                } else if sub == Sub::Decompile && args.target.is_none() {
                    args.target = Some(operand.to_string());
                } else {
                    return Err(format!("unexpected argument {operand:?}"));
                }
            }
        }
        i += 1;
    }
    if sub == Sub::Decompile && args.target.is_none() && !kind.single {
        return Err(format!(
            "decompile requires <input> and <class|method>; `kuna functions {input}` lists them"
        ));
    }
    if !args.json && (args.omit_line_mappings || args.omit_classes) && sub == Sub::Decompile {
        return Err("--omit selects --json record keys; add --json".into());
    }
    if let Some(value) = &language {
        engine.language(value, &subject, &mut args.options)?;
    }
    Ok(args)
}

fn omit(args: &mut Args, value: &str) -> Result<(), String> {
    let mut named = false;
    for key in value.split(',').map(str::trim).filter(|k| !k.is_empty()) {
        match key {
            "line_mappings" => args.omit_line_mappings = true,
            "classes" => args.omit_classes = true,
            "variables" | "types" => {}
            other => {
                return Err(format!(
                    "--omit {other:?}: expected a comma-separated list of line_mappings, classes, variables, types"
                ))
            }
        }
        named = true;
    }
    if named {
        Ok(())
    } else {
        Err("--omit needs at least one of line_mappings, classes, variables, types".into())
    }
}

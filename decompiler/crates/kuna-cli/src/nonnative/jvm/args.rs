//! The JVM subcommands' command lines: one parser, a flag table per subcommand.
//!
//! A flag only the native engine understands is refused (exit 2) rather than
//! ignored, `--option` names resolve through the JVM option table, and a
//! native option name is pointed at `kuna catalog --engine jvm`.

use regex::Regex;

use kuna_jvm::kuna_input::InputKind;
use kuna_jvm::kuna_options;

use super::DOCS_POINTER;

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

/// Which JDK gives the engine its class library.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum RuntimeSpec {
    /// The current JDK, if one is found (the default).
    Current,
    /// None.
    Off,
    /// The JDK home at this path; failing to open it is an error.
    Path(String),
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) enum SortKey {
    #[default]
    Addr,
    Size,
    Name,
}

/// The triage flags of `decompile-all` and `functions`, over methods.
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
    pub(crate) input: String,
    pub(crate) kind: InputKind,
    pub(crate) target: Option<String>,
    pub(crate) json: bool,
    pub(crate) omit_line_mappings: bool,
    pub(crate) omit_classes: bool,
    /// Vineflower `(key, value)` pairs, in command-line order.
    pub(crate) options: Vec<(String, String)>,
    pub(crate) siblings: bool,
    pub(crate) runtime: RuntimeSpec,
    pub(crate) max_fn_seconds: u64,
    pub(crate) max_fn_memory: Option<u64>,
    /// `None`: `auto`.
    pub(crate) jobs: Option<usize>,
    pub(crate) libs: Vec<String>,
    pub(crate) functions: Option<Vec<String>>,
    pub(crate) triage: Triage,
    pub(crate) output: Option<String>,
}

pub(crate) enum Refusal {
    /// A malformed command line (exit 2).
    Usage(String),
    /// A well-formed request the JVM engine cannot answer yet (exit 1).
    Capability(String),
}

impl From<String> for Refusal {
    fn from(e: String) -> Refusal {
        Refusal::Usage(e)
    }
}

/// Flags only the native engine has; each is refused on a JVM input.
const NATIVE_ONLY: &[(&str, bool)] = &[
    ("--isa", true),
    ("--slice", true),
    ("--target", true),
    ("--base", true),
    ("--entry", true),
    ("--addr", false),
    ("--define-function", true),
    ("--assert", true),
    ("--assert-strict", false),
    ("--kassert", true),
    ("--raw", false),
    ("--regions", false),
    ("--sleighpath", true),
    ("--decomp-dbg", true),
    ("--engine", true),
    ("--timeout", true),
    ("--stream", false),
    ("--progress", false),
    ("--jobs-chunk", true),
    ("--jobs-full-load", false),
    ("--jobs-proto", false),
    ("--jobs-provenance", false),
    ("--jobs-types", false),
    ("--jobs-callees", false),
    ("--jobs-pointerargs", false),
    ("--jobs-objects", false),
    ("--jobs-synth", true),
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

pub(crate) fn parse(sub: Sub, argv: &[String], input: &str, kind: InputKind) -> Result<Args, Refusal> {
    let mut args = Args {
        input: input.to_string(),
        kind,
        target: None,
        json: false,
        omit_line_mappings: false,
        omit_classes: false,
        options: Vec::new(),
        siblings: true,
        runtime: RuntimeSpec::Current,
        max_fn_seconds: kuna_console::project::DEFAULT_FN_BUDGET_SECONDS,
        max_fn_memory: None,
        jobs: None,
        libs: Vec::new(),
        functions: None,
        triage: Triage::default(),
        output: None,
    };
    let decompiles = sub != Sub::Functions;
    let triageable = matches!(sub, Sub::DecompileAll | Sub::Functions);
    let has_json = sub != Sub::Project;
    let mut seen_input = false;
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
                    return Err(Refusal::Usage("--option requires NAME VALUE".into()));
                }
                let (name, value) = (argv[i + 1].clone(), argv[i + 2].clone());
                i += 2;
                set_option(&mut args, &name, &value)?;
            }
            "--mode" => {
                let mode = take(argv, &mut i, "--mode")?;
                if !kuna_decomp::modes::mode_names().any(|m| m == mode) {
                    let names: Vec<&str> = kuna_decomp::modes::mode_names().collect();
                    return Err(Refusal::Usage(format!(
                        "unknown --mode {mode:?} (expected {})",
                        names.join("|")
                    )));
                }
            }
            "--language" => {
                let language = take(argv, &mut i, "--language")?;
                if !matches!(language.as_str(), "auto" | "java") {
                    return Err(Refusal::Usage(format!(
                        "--language {language}: {input} is a {}, which decompiles to Java (--language java|auto)",
                        super::input_label(kind)
                    )));
                }
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
                    _ => return Err(Refusal::Usage(format!("invalid --sort {v:?} (expected addr|size|name)"))),
                });
            }
            "--summary" | "--reachable-from" if sub == Sub::Functions => {
                return Err(Refusal::Capability(format!(
                    "{a}: {input} is a {}, and the JVM engine has no call graph yet; \
                     `kuna functions {input} --json` lists every method ({DOCS_POINTER})",
                    super::input_label(kind)
                )));
            }
            "-o" | "--output" if sub == Sub::Project => args.output = Some(take(argv, &mut i, a)?),
            "--json" | "--no-vars" if sub == Sub::Project => {
                return Err(Refusal::Usage(format!("{a} is not a decompile-project option")));
            }
            _ if NATIVE_ONLY.iter().any(|(flag, _)| *flag == a) => {
                return Err(Refusal::Usage(format!(
                    "{a} is a native-engine flag; {input} is a {}, decompiled by the JVM engine ({DOCS_POINTER})",
                    super::input_label(kind)
                )));
            }
            s if s.starts_with('-') && s.len() > 1 => {
                return Err(Refusal::Usage(format!("unknown option {s}")));
            }
            operand => {
                if !seen_input && operand == input {
                    seen_input = true;
                } else if sub == Sub::Decompile && args.target.is_none() {
                    args.target = Some(operand.to_string());
                } else {
                    return Err(Refusal::Usage(format!("unexpected argument {operand:?}")));
                }
            }
        }
        i += 1;
    }
    if sub == Sub::Decompile && args.target.is_none() && kind != InputKind::ClassFile {
        return Err(Refusal::Usage(format!(
            "decompile requires <input> and <class|method>; `kuna functions {input}` lists them"
        )));
    }
    if !args.json && (args.omit_line_mappings || args.omit_classes) && sub == Sub::Decompile {
        return Err(Refusal::Usage("--omit selects --json record keys; add --json".into()));
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

/// Applies one `--option NAME VALUE`.
fn set_option(args: &mut Args, name: &str, value: &str) -> Result<(), String> {
    match kuna_options::resolve(name, value) {
        Ok(("jvm_siblings", v)) => args.siblings = v == "1",
        Ok(("include-runtime", v)) => {
            args.runtime = match v.as_str() {
                "1" | "current" => RuntimeSpec::Current,
                "0" | "" => RuntimeSpec::Off,
                _ => RuntimeSpec::Path(v),
            }
        }
        Ok((key, v)) => args.options.push((key.to_string(), v)),
        Err(_) if kuna_options::find(name).is_none() && crate::optname::is_known(name) => {
            return Err(format!(
                "option {name}: a native-engine option; {} is a {}, whose options \
                 `kuna catalog --engine jvm` lists",
                args.input,
                super::input_label(args.kind)
            ));
        }
        Err(e) => return Err(e),
    }
    Ok(())
}

pub(crate) fn usage(sub: &str) -> String {
    let body = match sub {
        "decompile" => {
            "kuna decompile <class|jar|dir> <class|method> [--json] [--omit line_mappings,classes] \\\n\
             \x20             [--option NAME VALUE].. [--lib PATH].. [--jobs N|auto] [--mode M] \\\n\
             \x20             [--max-fn-seconds N] [--max-fn-memory MiB]\n\
             \n\
             A class prints its whole compilation unit (a nested class: its own part);\n\
             a method prints its slice. Targets: com.ex.Main, com/ex/Main, Main.class,\n\
             com.ex.Main$Inner or com.ex.Main.Inner, bare Main when unique;\n\
             com.ex.Main.check or com.ex.Main#check, optionally with a JVM descriptor:\n\
             com.ex.Main.check(Ljava/lang/String;)Z. A single .class input may omit it."
        }
        "decompile-all" => {
            "kuna decompile-all <class|jar|dir> [--json] [--functions a,b,..] [--filter REGEX] \\\n\
             \x20                 [--min-size N] [--max-size N] [--sort addr|size|name] [--limit N] \\\n\
             \x20                 [--omit line_mappings,classes] [--option NAME VALUE].. [--lib PATH].. \\\n\
             \x20                 [--jobs N|auto] [--mode M] [--max-fn-seconds N] [--max-fn-memory MiB]\n\
             \n\
             Every compilation unit under `// Class:` headers; the triage flags narrow to\n\
             methods (sizes are bytecode lengths) and print `// Method:` slices instead."
        }
        "decompile-project" => {
            "kuna decompile-project <class|jar|dir> [-o DIR] [--functions a,b,..] \\\n\
             \x20                     [--option NAME VALUE].. [--lib PATH].. [--jobs N|auto] [--mode M] \\\n\
             \x20                     [--max-fn-seconds N] [--max-fn-memory MiB]\n\
             \n\
             Writes DIR/<package path>/<Class>.java per top-level class (default DIR:\n\
             <input>.kuna). Exits 0 when at least one class was produced."
        }
        "functions" => {
            "kuna functions <class|jar|dir> [--json] [--filter REGEX] [--min-size N] [--max-size N] \\\n\
             \x20             [--sort addr|size|name] [--limit N] [--option NAME VALUE]..\n\
             \n\
             Every method, read from the class files (nothing is decompiled), as\n\
             `com.ex.Main.check(Ljava/lang/String;)Z`; sizes are bytecode lengths."
        }
        _ => "kuna decompile|decompile-all|decompile-project|functions <class|jar|dir> ..",
    };
    format!(
        "usage: {body}\n\nThis is the JVM engine's form (class files, jars, directories of classes).\n\
         Options: `kuna catalog --engine jvm`; manual: `{DOCS_POINTER}`."
    )
}

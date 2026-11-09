//! Runner for the Vineflower-derived regression corpus: one case is a set of class files, an
//! option map and the Java text expected for the main class.
//!
//! A case is run the way Vineflower's `SingleClassesTest` fixture runs it: the option pairs go to
//! [`Fernflower::new`] (which overlays them on the defaults), every class file is added as a source
//! in manifest order, the context is decompiled and then cleared. The fixture's saver writes every
//! class of a class-file source flat into one directory, so the compared text is the *last*
//! delivered unit whose file name is the case's `compare` name; it is compared with the expected
//! text after the fixture's `getContent` normalization (`\r\n` to `\n`, then `String.trim()`).
//! The manifest format is described in `tests/jvm/README.md`.

use std::cell::{Cell, RefCell};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::time::Instant;

use crate::decompiler_context::DecompilerContext;
use crate::fernflower::{DecompiledClass, Fernflower};
use crate::i_fernflower_logger::{IFernflowerLogger, Severity};
use crate::jdk_runtime::JdkRuntime;
use crate::rt::{self, try_catch, Caught};
use crate::text_buffer::TextBuffer;

pub const MANIFEST_HEADER: &str = "kuna-jvm-corpus";
pub const MANIFEST_VERSION: &str = "1";
pub const FAILED_METHOD_MARKER: &str = "$VF: Couldn't be decompiled";
/// Stack for threads that run the engine: the port recurses as deeply as the Java does.
pub const ENGINE_STACK_BYTES: usize = 512 << 20;
pub const JAVADOC_OPTION: &str = "fabric:javadoc";

/// A JDK a case needs: `${java.N.home}` in an option value, or `current_jdk N`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JdkNeed {
    pub version: String,
    pub home: Option<PathBuf>,
    /// The JDK must be the one `include-runtime=current` / `include-classpath=1` resolves to.
    pub current: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Case {
    pub id: String,
    pub set: String,
    pub set_slug: String,
    pub options: Vec<(String, String)>,
    pub class_files: Vec<PathBuf>,
    pub libraries: Vec<PathBuf>,
    pub expected: PathBuf,
    /// The output file compared, then its fallbacks.
    pub compare: Vec<String>,
    pub javadoc: Option<String>,
    pub jdks: Vec<JdkNeed>,
}

#[derive(Default)]
struct Scoped {
    options: Vec<(String, String)>,
    javadoc: Option<String>,
    current_jdk: Option<String>,
}

struct RawCase {
    line: usize,
    id: String,
    set: String,
    scoped: Scoped,
    classes: Vec<String>,
    libs: Vec<String>,
    expected: Option<String>,
    compare: Vec<String>,
}

/// Puts `key=value` into `options`, replacing an earlier value of `key` in place.
pub fn overlay_option(options: &mut Vec<(String, String)>, key: &str, value: &str) {
    match options.iter_mut().find(|(k, _)| k == key) {
        Some(entry) => entry.1 = value.to_string(),
        None => options.push((key.to_string(), value.to_string())),
    }
}

pub fn escape_field(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '\t' => out.push_str("\\t"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            c => out.push(c),
        }
    }
    out
}

pub fn unescape_field(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('t') => out.push('\t'),
            Some('n') => out.push('\n'),
            Some('r') => out.push('\r'),
            Some(other) => out.push(other),
            None => out.push('\\'),
        }
    }
    out
}

fn record_line(fields: &[&str]) -> String {
    let mut line = fields
        .iter()
        .map(|f| escape_field(f))
        .collect::<Vec<_>>()
        .join("\t");
    line.push('\n');
    line
}

pub fn load_cases(path: &Path) -> Result<Vec<Case>, String> {
    let text = std::fs::read_to_string(path)
        .map_err(|e| format!("cannot read manifest {}: {e}", path.display()))?;
    let base = path.parent().unwrap_or_else(|| Path::new("."));
    parse_cases(&text, base).map_err(|e| format!("{}: {e}", path.display()))
}

/// Parses a manifest; relative paths resolve against `root` (default: `base_dir`).
pub fn parse_cases(text: &str, base_dir: &Path) -> Result<Vec<Case>, String> {
    let mut header = false;
    let mut root: Option<String> = None;
    let mut java_homes: BTreeMap<String, String> = BTreeMap::new();
    let mut sets: BTreeMap<String, (String, Scoped)> = BTreeMap::new();
    let mut cases: Vec<RawCase> = Vec::new();
    enum At {
        Top,
        Set(String),
        Case,
    }
    let mut at = At::Top;

    for (index, raw) in text.split('\n').enumerate() {
        let line_no = index + 1;
        let raw = raw.strip_suffix('\r').unwrap_or(raw);
        if raw.is_empty() || raw.starts_with('#') {
            continue;
        }
        let fields: Vec<String> = raw.split('\t').map(unescape_field).collect();
        let err = |msg: &str| Err(format!("line {line_no}: {msg}"));
        let tag = fields[0].as_str();
        let args = &fields[1..];
        if !header {
            if tag != MANIFEST_HEADER || args.first().map(String::as_str) != Some(MANIFEST_VERSION)
            {
                return err(&format!(
                    "expected header `{MANIFEST_HEADER}\t{MANIFEST_VERSION}`"
                ));
            }
            header = true;
            continue;
        }
        let need = |n: usize| -> Result<(), String> {
            if args.len() < n {
                Err(format!("line {line_no}: `{tag}` needs {n} field(s)"))
            } else {
                Ok(())
            }
        };
        match tag {
            "root" => {
                need(1)?;
                root = Some(args[0].clone());
            }
            "java_home" => {
                need(2)?;
                java_homes.insert(args[0].clone(), args[1].clone());
            }
            "set" => {
                need(1)?;
                let name = args[0].clone();
                let slug = args.get(1).cloned().unwrap_or_default();
                if sets.contains_key(&name) {
                    return err(&format!("set `{name}` defined twice"));
                }
                sets.insert(name.clone(), (slug, Scoped::default()));
                at = At::Set(name);
            }
            "case" => {
                need(2)?;
                cases.push(RawCase {
                    line: line_no,
                    id: args[0].clone(),
                    set: args[1].clone(),
                    scoped: Scoped::default(),
                    classes: Vec::new(),
                    libs: Vec::new(),
                    expected: None,
                    compare: Vec::new(),
                });
                at = At::Case;
            }
            "opt" | "javadoc" | "current_jdk" => {
                need(1)?;
                let scoped = match &at {
                    At::Top => return err(&format!("`{tag}` outside a set or case")),
                    At::Set(name) => &mut sets.get_mut(name).expect("current set").1,
                    At::Case => &mut cases.last_mut().expect("current case").scoped,
                };
                match tag {
                    "opt" => overlay_option(
                        &mut scoped.options,
                        &args[0],
                        args.get(1).map_or("", String::as_str),
                    ),
                    "javadoc" => scoped.javadoc = Some(args[0].clone()),
                    _ => scoped.current_jdk = Some(args[0].clone()),
                }
            }
            "class" | "lib" | "expected" | "compare" => {
                need(1)?;
                if !matches!(at, At::Case) {
                    return err(&format!("`{tag}` outside a case"));
                }
                let case = cases.last_mut().expect("current case");
                match tag {
                    "class" => case.classes.push(args[0].clone()),
                    "lib" => case.libs.push(args[0].clone()),
                    "expected" => case.expected = Some(args[0].clone()),
                    _ => case.compare = args.iter().filter(|a| !a.is_empty()).cloned().collect(),
                }
            }
            other => return err(&format!("unknown record `{other}`")),
        }
    }
    if !header {
        return Err("empty manifest".to_string());
    }

    let root = match root {
        Some(r) => base_dir.join(r),
        None => base_dir.to_path_buf(),
    };
    let resolve = |p: &str| -> PathBuf {
        let p = Path::new(p);
        if p.is_absolute() {
            p.to_path_buf()
        } else {
            root.join(p)
        }
    };

    let mut seen = BTreeSet::new();
    let mut out = Vec::with_capacity(cases.len());
    for raw in cases {
        let err = |msg: String| Err(format!("line {}: case `{}`: {msg}", raw.line, raw.id));
        if !seen.insert(raw.id.clone()) {
            return err("duplicate case id".to_string());
        }
        let Some((slug, set)) = sets.get(&raw.set) else {
            return err(format!("unknown set `{}`", raw.set));
        };
        if raw.classes.is_empty() {
            return err("no class files".to_string());
        }
        let Some(expected) = raw.expected.as_deref() else {
            return err("no expected file".to_string());
        };
        if raw.compare.is_empty() {
            return err("no compare name".to_string());
        }
        let mut options = set.options.clone();
        for (k, v) in &raw.scoped.options {
            overlay_option(&mut options, k, v);
        }
        let mut jdks: Vec<JdkNeed> = Vec::new();
        for (_, value) in options.iter_mut() {
            *value = substitute_java_homes(value, &java_homes, &mut jdks);
        }
        if let Some(version) = raw.scoped.current_jdk.or_else(|| set.current_jdk.clone()) {
            let home = java_homes.get(&version).map(PathBuf::from);
            jdks.push(JdkNeed {
                version,
                home,
                current: true,
            });
        }
        out.push(Case {
            id: raw.id,
            set: raw.set,
            set_slug: slug.clone(),
            options,
            class_files: raw.classes.iter().map(|c| resolve(c)).collect(),
            libraries: raw.libs.iter().map(|c| resolve(c)).collect(),
            expected: resolve(expected),
            compare: raw.compare,
            javadoc: raw.scoped.javadoc.or_else(|| set.javadoc.clone()),
            jdks,
        });
    }
    Ok(out)
}

fn substitute_java_homes(
    value: &str,
    java_homes: &BTreeMap<String, String>,
    needs: &mut Vec<JdkNeed>,
) -> String {
    let mut out = String::new();
    let mut rest = value;
    while let Some(start) = rest.find("${java.") {
        let tail = &rest[start + "${java.".len()..];
        let Some(end) = tail.find(".home}") else {
            break;
        };
        let version = &tail[..end];
        out.push_str(&rest[..start]);
        let home = java_homes.get(version);
        match home {
            Some(home) => out.push_str(home),
            None => out.push_str(&rest[start..start + "${java.".len() + end + ".home}".len()]),
        }
        needs.push(JdkNeed {
            version: version.to_string(),
            home: home.map(PathBuf::from),
            current: false,
        });
        rest = &tail[end + ".home}".len()..];
    }
    out.push_str(rest);
    out
}

/// Why a case cannot run here, if it cannot; machine-specific detail is in parentheses.
pub fn unsupported_reason(case: &Case) -> Option<String> {
    if let Some(style) = &case.javadoc {
        return Some(format!(
            "javadoc provider {style} (kuna-jvm cannot take a `{JAVADOC_OPTION}` object)"
        ));
    }
    for need in &case.jdks {
        let Some(home) = &need.home else {
            return Some(format!(
                "jdk {} has no java_home in the manifest",
                need.version
            ));
        };
        if !JdkRuntime::is_java_home(home) {
            return Some(format!(
                "jdk {} not found ({})",
                need.version,
                home.display()
            ));
        }
        if need.current {
            let current = JdkRuntime::locate(JdkRuntime::CURRENT);
            let same = current.as_ref().is_some_and(|c| same_path(c, home));
            if !same {
                return Some(format!(
                    "jdk {} must be the current JDK (set JAVA_HOME={}; found {})",
                    need.version,
                    home.display(),
                    current.map_or("none".to_string(), |c| c.display().to_string())
                ));
            }
        }
    }
    None
}

fn same_path(a: &Path, b: &Path) -> bool {
    match (std::fs::canonicalize(a), std::fs::canonicalize(b)) {
        (Ok(a), Ok(b)) => a == b,
        _ => a == b,
    }
}

/// A caught Java exception or Rust panic.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Thrown {
    /// The Java exception class; `None` for a Rust panic.
    pub class: Option<String>,
    /// `Throwable.toString()`.
    pub message: String,
    pub location: String,
}

impl Thrown {
    pub fn from_caught(caught: &Caught) -> Thrown {
        Thrown {
            class: caught.exception.as_ref().map(|e| e.class.to_string()),
            message: caught.message.clone(),
            location: caught.location.clone(),
        }
    }

    pub fn innermost(&self) -> &str {
        innermost_message(&self.message)
    }
}

fn is_throwable_class(name: &str) -> bool {
    let simple = name.rsplit('.').next().unwrap_or(name);
    !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '$' || c == '.')
        && simple.starts_with(|c: char| c.is_ascii_uppercase())
        && (simple.ends_with("Exception") || simple.ends_with("Error") || simple == "Throwable")
}

fn split_throwable(message: &str) -> Option<(&str, &str)> {
    match message.find(": ") {
        Some(i) if is_throwable_class(&message[..i]) => Some((&message[..i], &message[i + 2..])),
        _ if is_throwable_class(message) => Some((message, "")),
        _ => None,
    }
}

/// The innermost `Class: message` of a `Throwable.toString()` that wraps others
/// (`RuntimeException: java.util.concurrent.ExecutionException: UnportedError: X` -> `UnportedError: X`).
pub fn innermost_message(message: &str) -> &str {
    let mut m = message;
    while let Some((_, rest)) = split_throwable(m) {
        if split_throwable(rest).is_none() {
            break;
        }
        m = rest;
    }
    m
}

/// `X` of an `UnportedError: X`.
pub fn unported_name(message: &str) -> Option<&str> {
    let inner = innermost_message(message);
    let rest = inner.strip_prefix("UnportedError: ")?;
    Some(rest.lines().next().unwrap_or("").trim())
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LogLine {
    pub severity: Severity,
    pub message: String,
    pub throwable: Option<Thrown>,
}

/// Records every accepted message with its throwable.
struct CorpusLogger {
    severity: Cell<Severity>,
    records: RefCell<Vec<LogLine>>,
}

impl IFernflowerLogger for CorpusLogger {
    fn severity(&self) -> &Cell<Severity> {
        &self.severity
    }

    fn write_message(&self, message: &str, severity: Severity) {
        if self.accepts(severity) {
            self.records.borrow_mut().push(LogLine {
                severity,
                message: message.to_string(),
                throwable: None,
            });
        }
    }

    fn write_message_ex(&self, message: &str, severity: Severity, t: &Caught) {
        if self.accepts(severity) {
            self.records.borrow_mut().push(LogLine {
                severity,
                message: message.to_string(),
                throwable: Some(Thrown::from_caught(t)),
            });
        }
    }
}

/// What one engine run produced.
#[derive(Clone, Debug, Default)]
pub struct EngineRun {
    /// Every delivered unit, in delivery order.
    pub units: Vec<DecompiledClass>,
    /// What escaped the run, if anything did.
    pub error: Option<Thrown>,
    pub log: Vec<LogLine>,
    pub millis: u64,
}

/// Drops this thread's decompilation state: the current context and every arena object.
pub fn reset_engine_thread() {
    DecompilerContext::set_current_context(None);
    rt::reset_session_scope();
}

/// Runs the engine over the case's inputs as the fixture does, containing anything it throws.
pub fn execute(case: &Case) -> EngineRun {
    reset_engine_thread();
    let logger = Rc::new(CorpusLogger {
        severity: Cell::new(Severity::Info),
        records: RefCell::new(Vec::new()),
    });
    let mut units = Vec::new();
    let start = Instant::now();
    let outcome = try_catch(|| {
        let mut fernflower = Fernflower::new(&case.options, logger.clone());
        let run = try_catch(|| {
            for lib in &case.libraries {
                fernflower.add_library_file(lib);
            }
            for class in &case.class_files {
                fernflower.add_source_file(class);
            }
            fernflower.decompile_context(&mut |unit| units.push(unit));
        });
        let cleared = try_catch(|| fernflower.clear_context());
        TextBuffer::check_leaks();
        run.err().or(cleared.err())
    });
    let millis = start.elapsed().as_millis() as u64;
    let error = match outcome {
        Ok(error) => error,
        Err(error) => Some(error),
    };
    let reset = try_catch(reset_engine_thread);
    let mut log = std::mem::take(&mut *logger.records.borrow_mut());
    if let Err(e) = reset {
        log.push(LogLine {
            severity: Severity::Error,
            message: "harness: resetting the engine thread failed".to_string(),
            throwable: Some(Thrown::from_caught(&e)),
        });
    }
    EngineRun {
        units,
        error: error.as_ref().map(Thrown::from_caught),
        log,
        millis,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CaseStatus {
    Match,
    Mismatch,
    /// The run threw (or, isolated, crashed or timed out).
    EngineError,
    /// The case needs something this machine or the port does not have.
    Unsupported,
}

impl CaseStatus {
    pub fn name(self) -> &'static str {
        match self {
            CaseStatus::Match => "Match",
            CaseStatus::Mismatch => "Mismatch",
            CaseStatus::EngineError => "EngineError",
            CaseStatus::Unsupported => "Unsupported",
        }
    }

    pub fn parse(name: &str) -> Option<CaseStatus> {
        [
            CaseStatus::Match,
            CaseStatus::Mismatch,
            CaseStatus::EngineError,
            CaseStatus::Unsupported,
        ]
        .into_iter()
        .find(|s| s.name() == name)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FirstDiff {
    /// 1-based, in the normalized texts.
    pub line: usize,
    /// `None` past the end of the text.
    pub expected: Option<String>,
    pub actual: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CaseResult {
    pub id: String,
    pub set: String,
    pub status: CaseStatus,
    /// The compared unit's text as the engine produced it; `None` when no unit had that name.
    pub actual: Option<String>,
    pub actual_path: Option<String>,
    /// Every delivered unit's path, in delivery order.
    pub units: Vec<String>,
    pub first_diff: Option<FirstDiff>,
    pub failed_methods_actual: usize,
    pub failed_methods_expected: usize,
    /// What the engine threw.
    pub error: Option<Thrown>,
    /// A failure outside the engine: `crash: ...`, `timeout: ...` or `harness: ...`.
    pub failure: Option<String>,
    /// Why the case is `Unsupported`.
    pub reason: Option<String>,
    /// Log lines at `Warn` and above.
    pub log: Vec<LogLine>,
    pub millis: u64,
}

impl CaseResult {
    fn empty(case: &Case, status: CaseStatus) -> CaseResult {
        CaseResult {
            id: case.id.clone(),
            set: case.set.clone(),
            status,
            actual: None,
            actual_path: None,
            units: Vec::new(),
            first_diff: None,
            failed_methods_actual: 0,
            failed_methods_expected: 0,
            error: None,
            failure: None,
            reason: None,
            log: Vec::new(),
            millis: 0,
        }
    }

    pub fn unsupported(case: &Case, reason: String) -> CaseResult {
        let mut r = CaseResult::empty(case, CaseStatus::Unsupported);
        r.reason = Some(reason);
        r
    }

    /// An `EngineError` that happened outside the engine (`crash: ...`, `timeout: ...`, `harness: ...`).
    pub fn failed(case: &Case, failure: String, millis: u64) -> CaseResult {
        let mut r = CaseResult::empty(case, CaseStatus::EngineError);
        r.failure = Some(failure);
        r.millis = millis;
        r
    }
}

pub fn run_case(case: &Case) -> CaseResult {
    if let Some(reason) = unsupported_reason(case) {
        return CaseResult::unsupported(case, reason);
    }
    let run = execute(case);
    evaluate(case, run)
}

/// `DecompilerTestFixture.getContent`: `\r\n` to `\n`, then `String.trim()`.
pub fn normalize(text: &str) -> String {
    text.replace("\r\n", "\n")
        .trim_matches(|c: char| c <= ' ')
        .to_string()
}

pub fn first_diff(expected: &str, actual: &str) -> Option<FirstDiff> {
    if expected == actual {
        return None;
    }
    let mut e = expected.split('\n');
    let mut a = actual.split('\n');
    let mut line = 1;
    loop {
        match (e.next(), a.next()) {
            (Some(x), Some(y)) if x == y => line += 1,
            (None, None) => return None,
            (x, y) => {
                return Some(FirstDiff {
                    line,
                    expected: x.map(str::to_string),
                    actual: y.map(str::to_string),
                })
            }
        }
    }
}

pub fn count_failed_methods(text: &str) -> usize {
    text.matches(FAILED_METHOD_MARKER).count()
}

/// The unit the fixture reads back: for each `compare` name in turn, the last unit with that file name.
pub fn select_unit<'a>(
    units: &'a [DecompiledClass],
    compare: &[String],
) -> Option<&'a DecompiledClass> {
    compare.iter().find_map(|name| {
        units
            .iter()
            .rev()
            .find(|u| u.path.rsplit('/').next() == Some(name.as_str()))
    })
}

/// Compares what a run produced with the case's expected text.
pub fn evaluate(case: &Case, run: EngineRun) -> CaseResult {
    let expected_raw = match std::fs::read(&case.expected) {
        Ok(bytes) => String::from_utf8_lossy(&bytes).into_owned(),
        Err(e) => {
            return CaseResult::failed(
                case,
                format!("harness: cannot read {}: {e}", case.expected.display()),
                run.millis,
            )
        }
    };
    evaluate_text(case, run, &expected_raw)
}

pub fn evaluate_text(case: &Case, run: EngineRun, expected_raw: &str) -> CaseResult {
    let expected = normalize(expected_raw);
    let selected = select_unit(&run.units, &case.compare);
    let first = match selected {
        Some(unit) => first_diff(&expected, &normalize(&unit.content)),
        None => Some(FirstDiff {
            line: 1,
            expected: expected.split('\n').next().map(str::to_string),
            actual: None,
        }),
    };
    let status = if run.error.is_some() {
        CaseStatus::EngineError
    } else if first.is_none() {
        CaseStatus::Match
    } else {
        CaseStatus::Mismatch
    };
    CaseResult {
        id: case.id.clone(),
        set: case.set.clone(),
        status,
        actual: selected.map(|u| u.content.clone()),
        actual_path: selected.map(|u| u.path.clone()),
        units: run.units.iter().map(|u| u.path.clone()).collect(),
        first_diff: first,
        failed_methods_actual: selected.map_or(0, |u| count_failed_methods(&u.content)),
        failed_methods_expected: count_failed_methods(&expected),
        error: run.error,
        failure: None,
        reason: None,
        log: run
            .log
            .into_iter()
            .filter(|l| l.severity >= Severity::Warn)
            .collect(),
        millis: run.millis,
    }
}

/// The run as if the engine had printed the expected text for the compared unit (self-check).
pub fn expected_as_actual(case: &Case) -> Result<EngineRun, String> {
    let content = std::fs::read(&case.expected)
        .map_err(|e| format!("cannot read {}: {e}", case.expected.display()))?;
    let class = case.id.split_once('/').map_or("", |(_, class)| class);
    let dir = class.rsplit_once('/').map_or("", |(dir, _)| dir);
    let decoy = DecompiledClass {
        qualified_name: "decoy/Decoy".to_string(),
        path: format!("decoy/{}", case.compare[0]),
        content: String::new(),
        mapping: None,
        ..DecompiledClass::default()
    };
    let unit = DecompiledClass {
        qualified_name: String::new(),
        path: if dir.is_empty() {
            case.compare[0].clone()
        } else {
            format!("{dir}/{}", case.compare[0])
        },
        content: String::from_utf8_lossy(&content).into_owned(),
        mapping: None,
        ..DecompiledClass::default()
    };
    Ok(EngineRun {
        units: vec![decoy, unit],
        ..EngineRun::default()
    })
}

/// The option map the engine runs a case with: Vineflower's defaults overlaid by the case's pairs,
/// read back from the context `Fernflower::new` installs, sorted by key. A javadoc provider the
/// port cannot take is listed as `fabric:javadoc=$javadoc_provider:<style>`.
pub fn effective_options(case: &Case) -> Result<Vec<(String, String)>, Thrown> {
    reset_engine_thread();
    let result = try_catch(|| {
        let logger: Rc<dyn IFernflowerLogger> = <dyn IFernflowerLogger>::NO_OP();
        let mut fernflower = Fernflower::new(&case.options, logger);
        let mut entries = DecompilerContext::get_current_context()
            .expect("Fernflower installs a context")
            .with(|d| d.properties.entry_vec());
        fernflower.clear_context();
        if let Some(style) = &case.javadoc {
            entries.push((
                JAVADOC_OPTION.to_string(),
                format!("$javadoc_provider:{style}"),
            ));
        }
        entries.sort();
        entries
    });
    let _ = try_catch(reset_engine_thread);
    result.map_err(|e| Thrown::from_caught(&e))
}

/// Runs `f` on a fresh thread with an [`ENGINE_STACK_BYTES`] stack.
pub fn with_engine_stack<R: Send>(f: impl FnOnce() -> R + Send) -> R {
    std::thread::scope(|scope| {
        let handle = std::thread::Builder::new()
            .name("kuna-jvm-engine".to_string())
            .stack_size(ENGINE_STACK_BYTES)
            .spawn_scoped(scope, f)
            .expect("spawn engine thread");
        handle
            .join()
            .unwrap_or_else(|e| std::panic::resume_unwind(e))
    })
}

/// `src/...:line` for a location inside this crate; std paths lose their `/rustc/<hash>/` prefix.
pub fn short_location(location: &str) -> String {
    if let Some(i) = location.find("kuna-jvm/") {
        return location[i + "kuna-jvm/".len()..].to_string();
    }
    if let Some(rest) = location.strip_prefix("/rustc/") {
        if let Some(i) = rest.find('/') {
            return rest[i + 1..].to_string();
        }
    }
    location.to_string()
}

/// The histogram key of something the engine threw.
pub fn thrown_key(t: &Thrown) -> String {
    let inner = t.innermost();
    if let Some(name) = unported_name(inner) {
        return format!("unported: {name}");
    }
    let location = short_location(&t.location);
    let class = split_throwable(inner).map(|(c, _)| c.to_string());
    match (class.or_else(|| t.class.clone()), location.is_empty()) {
        (Some(class), false) => format!("{class}: {location}"),
        (Some(class), true) => format!("{class}: {}", head(inner, 80)),
        (None, false) => format!("panic: {location}"),
        (None, true) => format!("panic: {}", head(inner, 80)),
    }
}

fn head(text: &str, max: usize) -> String {
    let first = text.lines().next().unwrap_or("").trim();
    if first.chars().count() <= max {
        first.to_string()
    } else {
        let cut: String = first.chars().take(max).collect();
        format!("{cut}...")
    }
}

/// A line without a trailing `// 12` / `// 12 13` original-line comment.
fn strip_line_comment(line: &str) -> &str {
    if let Some(i) = line.rfind("//") {
        let tail = line[i + 2..].trim();
        if !tail.is_empty()
            && tail
                .split(' ')
                .all(|t| !t.is_empty() && t.bytes().all(|b| b.is_ascii_digit()))
        {
            return &line[..i];
        }
    }
    line
}

/// A coarse kind of Java source line, for grouping first differences.
pub fn line_class(line: Option<&str>) -> &'static str {
    let Some(line) = line else {
        return "end of file";
    };
    let t = strip_line_comment(line).trim();
    let digits_only = |s: &str| {
        !s.is_empty()
            && s.split_whitespace()
                .all(|w| w.bytes().all(|b| b.is_ascii_digit()))
    };
    if t.is_empty() {
        return "blank line";
    }
    if t == "Lines mapping:" || t == "Not mapped:" || digits_only(t) || t.contains(" <-> ") {
        return "line mapping";
    }
    if (t.starts_with("class '") || t.starts_with("method '")) && t.ends_with('{') {
        return "line mapping";
    }
    if t.starts_with("package ") {
        return "package";
    }
    if t.starts_with("import ") {
        return "import";
    }
    if t.starts_with("//") {
        return if t.contains("$VF") {
            "vf comment"
        } else {
            "comment"
        };
    }
    if t.starts_with("/*") || t.starts_with('*') {
        return "comment";
    }
    if t.starts_with('@') && !t.starts_with("@interface") {
        return "annotation";
    }
    if let Some(rest) = t.strip_prefix('}') {
        let rest = rest.trim_start();
        for kw in ["else", "catch", "finally", "while"] {
            if rest.starts_with(kw) {
                return kw;
            }
        }
        return "closing brace";
    }
    let first = t.split([' ', '(', ';', ':']).next().unwrap_or("");
    const KEYWORDS: [&str; 18] = [
        "if",
        "else",
        "for",
        "while",
        "do",
        "switch",
        "try",
        "catch",
        "finally",
        "synchronized",
        "return",
        "throw",
        "break",
        "continue",
        "case",
        "default",
        "yield",
        "assert",
    ];
    if let Some(kw) = KEYWORDS.into_iter().find(|k| *k == first) {
        return kw;
    }
    let before_brace = t.split('{').next().unwrap_or("");
    let words: Vec<&str> = before_brace.split_whitespace().collect();
    if words
        .iter()
        .any(|w| matches!(*w, "class" | "interface" | "enum" | "record" | "@interface"))
        && !before_brace.contains('=')
        && !before_brace.contains('(')
    {
        return "type declaration";
    }
    if t.ends_with('{') && t.contains('(') && !t.contains("->") && !t.contains('=') {
        return "method declaration";
    }
    if t.ends_with(';') || t.ends_with('{') {
        return "statement";
    }
    "other"
}

/// The histogram key of a non-matching result; `None` for a match.
pub fn failure_key(r: &CaseResult) -> Option<String> {
    match r.status {
        CaseStatus::Match => None,
        CaseStatus::Unsupported => Some(format!(
            "unsupported: {}",
            r.reason.as_deref().map_or("?".to_string(), |s| head(
                s.split(" (").next().unwrap_or(s),
                120
            ))
        )),
        CaseStatus::EngineError => Some(match (&r.failure, &r.error) {
            (Some(f), _) => head(f, 120),
            (None, Some(t)) => thrown_key(&located(t, &r.log)),
            (None, None) => "error".to_string(),
        }),
        CaseStatus::Mismatch => Some(match r.log.iter().find_map(|l| l.throwable.as_ref()) {
            Some(t) => thrown_key(t),
            None => mismatch_key(r),
        }),
    }
}

/// A wrapped escape (`ExecutionException: ...`) carries the wrapper's location; the logged
/// original, when there is one, carries the real one.
fn located(t: &Thrown, log: &[LogLine]) -> Thrown {
    let inner = t.innermost();
    if inner.len() == t.message.len() {
        return t.clone();
    }
    log.iter()
        .filter_map(|l| l.throwable.as_ref())
        .find(|l| l.innermost() == inner)
        .cloned()
        .unwrap_or_else(|| t.clone())
}

fn mismatch_key(r: &CaseResult) -> String {
    if r.actual.is_none() {
        return "mismatch: no output".to_string();
    }
    let Some(d) = &r.first_diff else {
        return "mismatch".to_string();
    };
    if let (Some(e), Some(a)) = (&d.expected, &d.actual) {
        if strip_line_comment(e) == strip_line_comment(a) {
            return "mismatch: original-line comment".to_string();
        }
    }
    format!("mismatch: {}", line_class(d.expected.as_deref()))
}

/// The first line of the most telling error text of a result.
pub fn error_head(r: &CaseResult) -> String {
    if let Some(f) = &r.failure {
        return head(f, 200);
    }
    if let Some(t) = &r.error {
        return head(t.innermost(), 200);
    }
    if let Some(reason) = &r.reason {
        return head(reason, 200);
    }
    r.log
        .iter()
        .find_map(|l| l.throwable.as_ref())
        .map(|t| head(t.innermost(), 200))
        .unwrap_or_default()
}

/// Non-matching cases grouped by [`failure_key`], most frequent first.
pub fn histogram(results: &[CaseResult]) -> Vec<(String, Vec<String>)> {
    let mut groups: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for r in results {
        if let Some(key) = failure_key(r) {
            groups.entry(key).or_default().push(r.id.clone());
        }
    }
    let mut out: Vec<(String, Vec<String>)> = groups.into_iter().collect();
    out.sort_by(|a, b| b.1.len().cmp(&a.1.len()).then_with(|| a.0.cmp(&b.0)));
    out
}

/// Every unported body a result ran into, in the error or anywhere in the log.
pub fn unported_hits(r: &CaseResult) -> BTreeSet<String> {
    r.error
        .iter()
        .chain(r.log.iter().filter_map(|l| l.throwable.as_ref()))
        .filter_map(|t| unported_name(t.innermost()).map(str::to_string))
        .collect()
}

/// Unported bodies by the number of non-matching cases that ran into them, most first.
pub fn unported_reach(results: &[CaseResult]) -> Vec<(String, usize)> {
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();
    for r in results.iter().filter(|r| r.status != CaseStatus::Match) {
        for name in unported_hits(r) {
            *counts.entry(name).or_default() += 1;
        }
    }
    let mut out: Vec<(String, usize)> = counts.into_iter().collect();
    out.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    out
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Totals {
    pub total: usize,
    pub matched: usize,
    pub mismatch: usize,
    pub error: usize,
    pub unsupported: usize,
}

pub fn totals(results: &[CaseResult]) -> Totals {
    let mut t = Totals {
        total: results.len(),
        ..Totals::default()
    };
    for r in results {
        match r.status {
            CaseStatus::Match => t.matched += 1,
            CaseStatus::Mismatch => t.mismatch += 1,
            CaseStatus::EngineError => t.error += 1,
            CaseStatus::Unsupported => t.unsupported += 1,
        }
    }
    t
}

pub fn totals_line(results: &[CaseResult]) -> String {
    let t = totals(results);
    format!(
        "corpus: {}/{} match, {} mismatch, {} error, {} unsupported",
        t.matched, t.total, t.mismatch, t.error, t.unsupported
    )
}

pub const SUMMARY_COLUMNS: [&str; 9] = [
    "id",
    "set",
    "status",
    "first_diff",
    "failed_actual",
    "failed_expected",
    "error",
    "ms",
    "key",
];

fn tsv_field(s: &str) -> String {
    s.replace(['\t', '\n', '\r'], " ")
}

pub fn summary_tsv(results: &[CaseResult]) -> String {
    let mut out = SUMMARY_COLUMNS.join("\t");
    out.push('\n');
    for r in results {
        let fields = [
            r.id.clone(),
            r.set.clone(),
            r.status.name().to_string(),
            r.first_diff
                .as_ref()
                .map_or(String::new(), |d| d.line.to_string()),
            r.failed_methods_actual.to_string(),
            r.failed_methods_expected.to_string(),
            error_head(r),
            r.millis.to_string(),
            failure_key(r).unwrap_or_default(),
        ];
        out.push_str(
            &fields
                .iter()
                .map(|f| tsv_field(f))
                .collect::<Vec<_>>()
                .join("\t"),
        );
        out.push('\n');
    }
    out
}

/// A baseline file: one matching case id per line; blank and `#` lines are ignored.
pub fn parse_baseline(text: &str) -> BTreeSet<String> {
    text.lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .map(str::to_string)
        .collect()
}

pub fn format_baseline(ids: &BTreeSet<String>) -> String {
    let mut out = String::from("# kuna-jvm corpus baseline: the ids of matching cases, sorted\n");
    for id in ids {
        out.push_str(id);
        out.push('\n');
    }
    out
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct BaselineReport {
    /// In the baseline, run, and no longer matching.
    pub regressions: Vec<String>,
    /// Matching now, not in the baseline.
    pub improvements: Vec<String>,
    /// In the baseline but not run (filtered out or gone from the manifest).
    pub not_run: Vec<String>,
}

pub fn compare_baseline(baseline: &BTreeSet<String>, results: &[CaseResult]) -> BaselineReport {
    let run: BTreeMap<&str, CaseStatus> =
        results.iter().map(|r| (r.id.as_str(), r.status)).collect();
    let mut report = BaselineReport::default();
    for id in baseline {
        match run.get(id.as_str()) {
            Some(CaseStatus::Match) => {}
            Some(_) => report.regressions.push(id.clone()),
            None => report.not_run.push(id.clone()),
        }
    }
    for (id, status) in &run {
        if *status == CaseStatus::Match && !baseline.contains(*id) {
            report.improvements.push(id.to_string());
        }
    }
    report
}

/// The baseline to save after a run: the matching ids of the run, plus the ids of `previous`
/// that this run did not cover (a filtered run updates only what it ran).
pub fn next_baseline(previous: &BTreeSet<String>, results: &[CaseResult]) -> BTreeSet<String> {
    let run: BTreeSet<&str> = results.iter().map(|r| r.id.as_str()).collect();
    previous
        .iter()
        .filter(|id| !run.contains(id.as_str()))
        .cloned()
        .chain(
            results
                .iter()
                .filter(|r| r.status == CaseStatus::Match)
                .map(|r| r.id.clone()),
        )
        .collect()
}

const RECORD_BEGIN: &str = "@@kuna-jvm-corpus-result@@";
const RECORD_END: &str = "@@end@@";

fn thrown_fields(t: &Thrown) -> [&str; 3] {
    [t.class.as_deref().unwrap_or(""), &t.message, &t.location]
}

fn thrown_from(fields: &[String]) -> Thrown {
    let get = |i: usize| fields.get(i).cloned().unwrap_or_default();
    Thrown {
        class: Some(get(0)).filter(|c| !c.is_empty()),
        message: get(1),
        location: get(2),
    }
}

impl CaseResult {
    /// The result as framed text, for a child process to hand to its parent.
    pub fn to_record(&self) -> String {
        let mut out = format!("{RECORD_BEGIN}\n");
        let mut put = |fields: &[&str]| out.push_str(&record_line(fields));
        put(&["id", &self.id]);
        put(&["set", &self.set]);
        put(&["status", self.status.name()]);
        if let Some(a) = &self.actual {
            put(&["actual", a]);
        }
        if let Some(p) = &self.actual_path {
            put(&["actual_path", p]);
        }
        for u in &self.units {
            put(&["unit", u]);
        }
        if let Some(d) = &self.first_diff {
            put(&["diff_line", &d.line.to_string()]);
            if let Some(e) = &d.expected {
                put(&["diff_expected", e]);
            }
            if let Some(a) = &d.actual {
                put(&["diff_actual", a]);
            }
        }
        put(&[
            "failed",
            &self.failed_methods_actual.to_string(),
            &self.failed_methods_expected.to_string(),
        ]);
        if let Some(t) = &self.error {
            let mut f = vec!["error"];
            f.extend(thrown_fields(t));
            put(&f);
        }
        if let Some(f) = &self.failure {
            put(&["failure", f]);
        }
        if let Some(r) = &self.reason {
            put(&["reason", r]);
        }
        for l in &self.log {
            let mut f = vec!["log", l.severity.name(), &l.message];
            if let Some(t) = &l.throwable {
                f.extend(thrown_fields(t));
            }
            put(&f);
        }
        put(&["millis", &self.millis.to_string()]);
        out.push_str(RECORD_END);
        out.push('\n');
        out
    }

    /// The last framed record in `text` (a child's whole standard output).
    pub fn from_record(text: &str) -> Result<CaseResult, String> {
        let begin = format!("{RECORD_BEGIN}\n");
        let start = text.rfind(&begin).ok_or("no result record")?;
        let body = &text[start + begin.len() - 1..];
        let end = body
            .find(&format!("\n{RECORD_END}"))
            .ok_or("truncated result record")?;
        let mut r = CaseResult {
            id: String::new(),
            set: String::new(),
            status: CaseStatus::EngineError,
            actual: None,
            actual_path: None,
            units: Vec::new(),
            first_diff: None,
            failed_methods_actual: 0,
            failed_methods_expected: 0,
            error: None,
            failure: None,
            reason: None,
            log: Vec::new(),
            millis: 0,
        };
        let mut status = None;
        for line in body[..end].split('\n').filter(|l| !l.is_empty()) {
            let fields: Vec<String> = line.split('\t').map(unescape_field).collect();
            let get = |i: usize| fields.get(i).cloned().unwrap_or_default();
            let num = |i: usize| -> Result<u64, String> {
                get(i)
                    .parse()
                    .map_err(|_| format!("bad number in `{line}`"))
            };
            match fields[0].as_str() {
                "id" => r.id = get(1),
                "set" => r.set = get(1),
                "status" => status = CaseStatus::parse(&get(1)),
                "actual" => r.actual = Some(get(1)),
                "actual_path" => r.actual_path = Some(get(1)),
                "unit" => r.units.push(get(1)),
                "diff_line" => {
                    r.first_diff = Some(FirstDiff {
                        line: num(1)? as usize,
                        expected: None,
                        actual: None,
                    })
                }
                "diff_expected" => {
                    if let Some(d) = r.first_diff.as_mut() {
                        d.expected = Some(get(1));
                    }
                }
                "diff_actual" => {
                    if let Some(d) = r.first_diff.as_mut() {
                        d.actual = Some(get(1));
                    }
                }
                "failed" => {
                    r.failed_methods_actual = num(1)? as usize;
                    r.failed_methods_expected = num(2)? as usize;
                }
                "error" => r.error = Some(thrown_from(&fields[1..])),
                "failure" => r.failure = Some(get(1)),
                "reason" => r.reason = Some(get(1)),
                "log" => r.log.push(LogLine {
                    severity: Severity::value_of(&get(1))
                        .ok_or(format!("bad severity in `{line}`"))?,
                    message: get(2),
                    throwable: (fields.len() > 3).then(|| thrown_from(&fields[3..])),
                }),
                "millis" => r.millis = num(1)?,
                other => return Err(format!("unknown result field `{other}`")),
            }
        }
        r.status = status.ok_or("result record without a status")?;
        Ok(r)
    }
}

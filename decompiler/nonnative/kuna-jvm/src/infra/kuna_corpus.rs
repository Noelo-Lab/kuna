//! Runner for the regression corpus (`tests/jvm/cases.tsv`): one case is a set of class files,
//! an option map and the Java or Kotlin text pinned for the main class.
//!
//! A case is run the way Vineflower's `SingleClassesTest` fixture runs it: the option pairs go to
//! [`Fernflower::new`] (which overlays them on the defaults) with no plugin loaded (`jvm_plugins`
//! off unless the case sets it; see [`engine_options`]), every class file is added as a source
//! in manifest order, the context is decompiled and then cleared. The fixture's saver writes every
//! class of a class-file source flat into one directory, so the compared text is the *last*
//! delivered unit whose file name is the case's `compare` name; it is compared with the expected
//! text after the fixture's `getContent` normalization (`\r\n` to `\n`, then `String.trim()`), or,
//! for a case with a `kuna` record, its normalized text must have that digest ([`text_digest`]).
//! The manifest format is described in `tests/jvm/README.md`.

use std::cell::{Cell, RefCell};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::rc::Rc;

use crate::class_writer::{FabricJavadocStyle, IFabricJavadocProvider};
use crate::decompiler_context::DecompilerContext;
use crate::fernflower::{DecompiledClass, Fernflower};
use crate::i_fernflower_logger::{IFernflowerLogger, Severity};
use crate::kuna_plugins::JVM_PLUGINS;
use crate::rt::{self, try_catch, Caught};
use crate::struct_class::StructClass;
use crate::struct_field::StructField;
use crate::struct_method::StructMethod;
use crate::text_buffer::TextBuffer;
use sha2::{Digest, Sha256};

pub const MANIFEST_HEADER: &str = "kuna-jvm-corpus";
pub const MANIFEST_VERSION: &str = "1";
pub const FAILED_METHOD_MARKER: &str = "$VF: Couldn't be decompiled";
/// The manifest record that pins a case's digest.
pub const DIGEST_RECORD: &str = "kuna";
/// Stack for threads that run the engine. The recursion guard (`rt::stack`)
/// lets the engine use `rt::stack::DEPTH_LIMIT` of it; the rest is headroom.
pub const ENGINE_STACK_BYTES: usize = 512 << 20;

/// The engine thread stack: [`ENGINE_STACK_BYTES`], or `KUNA_JVM_STACK_MIB`
/// MiB (a developer knob for exercising the recursion guard, `rt::stack`).
pub fn engine_stack_bytes() -> usize {
    static BYTES: std::sync::OnceLock<usize> = std::sync::OnceLock::new();
    *BYTES.get_or_init(|| {
        std::env::var("KUNA_JVM_STACK_MIB")
            .ok()
            .and_then(|v| v.parse::<usize>().ok())
            .filter(|&mib| mib > 0)
            .map_or(ENGINE_STACK_BYTES, |mib| mib << 20)
    })
}

/// The provider `SingleClassesTest`'s Javadoc (`plain`) and Markdown Javadoc (`markdown`) sets pass.
pub struct FixtureJavadocProvider {
    pub style: FabricJavadocStyle,
}

impl FixtureJavadocProvider {
    pub fn for_style(style: &str) -> Option<FixtureJavadocProvider> {
        let style = match style {
            "plain" => FabricJavadocStyle::Html,
            "markdown" => FabricJavadocStyle::Markdown,
            _ => return None,
        };
        Some(FixtureJavadocProvider { style })
    }
}

impl IFabricJavadocProvider for FixtureJavadocProvider {
    fn get_class_doc(&self, struct_class: StructClass) -> Option<String> {
        Some(format!(
            "Class javadoc for '{}'",
            struct_class.qualified_name()
        ))
    }

    fn get_field_doc(&self, _: StructClass, struct_field: StructField) -> Option<String> {
        Some(format!("Field javadoc for '{}'", struct_field.get_name()))
    }

    fn get_method_doc(&self, _: StructClass, struct_method: StructMethod) -> Option<String> {
        Some(format!("Method javadoc for '{}'", struct_method.get_name()))
    }

    fn get_class_javadoc_style(&self, _: StructClass) -> FabricJavadocStyle {
        self.style
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Case {
    pub id: String,
    pub set: String,
    pub set_slug: String,
    pub options: Vec<(String, String)>,
    pub class_files: Vec<PathBuf>,
    pub libraries: Vec<PathBuf>,
    /// The expected text; `None` when `digest` pins it.
    pub expected: Option<PathBuf>,
    /// The output file compared, then its fallbacks.
    pub compare: Vec<String>,
    pub javadoc: Option<String>,
    /// The pinned [`text_digest`] of the compared text, checked instead of an expected file.
    pub digest: Option<String>,
}

#[derive(Default)]
struct Scoped {
    options: Vec<(String, String)>,
    javadoc: Option<String>,
}

struct RawCase {
    line: usize,
    id: String,
    set: String,
    scoped: Scoped,
    stems: Vec<String>,
    classes: Vec<String>,
    libs: Vec<String>,
    expected: Option<String>,
    compare: Vec<String>,
    digest: Option<String>,
}

/// The option pairs the engine runs `case` with: the fixture registers no plugin, so
/// `jvm_plugins` is off unless the case sets it.
pub fn engine_options(case: &Case) -> Vec<(String, String)> {
    let mut options = vec![(JVM_PLUGINS.to_string(), "0".to_string())];
    for (key, value) in &case.options {
        overlay_option(&mut options, key, value);
    }
    options
}

/// Puts `key=value` into `options`, replacing an earlier value of `key` in place.
pub fn overlay_option(options: &mut Vec<(String, String)>, key: &str, value: &str) {
    match options.iter_mut().find(|(k, _)| k == key) {
        Some(entry) => entry.1 = value.to_string(),
        None => options.push((key.to_string(), value.to_string())),
    }
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
    let mut sets: BTreeMap<String, (String, String, Scoped)> = BTreeMap::new();
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
            "set" => {
                need(1)?;
                let name = args[0].clone();
                let slug = args.get(1).cloned().unwrap_or_default();
                let expected_dir = args.get(2).cloned().unwrap_or(format!("expected/{slug}"));
                if sets.contains_key(&name) {
                    return err(&format!("set `{name}` defined twice"));
                }
                sets.insert(name.clone(), (slug, expected_dir, Scoped::default()));
                at = At::Set(name);
            }
            "case" => {
                need(2)?;
                cases.push(RawCase {
                    line: line_no,
                    id: args[0].clone(),
                    set: args[1].clone(),
                    scoped: Scoped::default(),
                    stems: args[2..].to_vec(),
                    classes: Vec::new(),
                    libs: Vec::new(),
                    expected: None,
                    compare: Vec::new(),
                    digest: None,
                });
                at = At::Case;
            }
            "opt" | "javadoc" => {
                need(1)?;
                let scoped = match &at {
                    At::Top => return err(&format!("`{tag}` outside a set or case")),
                    At::Set(name) => &mut sets.get_mut(name).expect("current set").2,
                    At::Case => &mut cases.last_mut().expect("current case").scoped,
                };
                match tag {
                    "opt" => overlay_option(
                        &mut scoped.options,
                        &args[0],
                        args.get(1).map_or("", String::as_str),
                    ),
                    _ => {
                        if FixtureJavadocProvider::for_style(&args[0]).is_none() {
                            return err(&format!("javadoc provider `{}` (not a fixture provider)", args[0]));
                        }
                        scoped.javadoc = Some(args[0].clone());
                    }
                }
            }
            "class" | "lib" | "expected" | "compare" | DIGEST_RECORD => {
                need(1)?;
                if !matches!(at, At::Case) {
                    return err(&format!("`{tag}` outside a case"));
                }
                let case = cases.last_mut().expect("current case");
                match tag {
                    "class" => case.classes.push(args[0].clone()),
                    "lib" => case.libs.push(args[0].clone()),
                    "expected" => case.expected = Some(args[0].clone()),
                    DIGEST_RECORD => case.digest = Some(args[0].clone()),
                    _ => case.compare = args.iter().filter(|a| !a.is_empty()).cloned().collect(),
                }
            }
            other => return err(&format!("unknown record `{other}`")),
        }
    }
    if !header {
        return Err("empty manifest".to_string());
    }

    let root = base_dir.join(root.unwrap_or_default());
    let resolve = |p: &str| -> PathBuf { root.join(p) };

    let mut seen = BTreeSet::new();
    let mut listings = Listings::default();
    let mut out = Vec::with_capacity(cases.len());
    for raw in cases {
        let err = |msg: String| Err(format!("line {}: case `{}`: {msg}", raw.line, raw.id));
        if !seen.insert(raw.id.clone()) {
            return err("duplicate case id".to_string());
        }
        let Some((slug, expected_dir, set)) = sets.get(&raw.set) else {
            return err(format!("unknown set `{}`", raw.set));
        };
        let mut class_files = Vec::new();
        for stem in &raw.stems {
            class_files.extend(listings.expand(&resolve(stem)));
        }
        class_files.extend(raw.classes.iter().map(|c| resolve(c)));
        if class_files.is_empty() {
            return err("no class files".to_string());
        }
        let expected = match (&raw.digest, raw.expected.as_deref()) {
            (Some(_), Some(_)) => return err(format!("both `expected` and `{DIGEST_RECORD}`")),
            (Some(_), None) => None,
            (None, Some(path)) => Some(resolve(path)),
            (None, None) => {
                let tail = raw.id.strip_prefix(&format!("{slug}/")).unwrap_or(&raw.id);
                let base = resolve(expected_dir).join(tail);
                let Some(ext) = [".java", ".kt"]
                    .into_iter()
                    .find(|e| listings.has(&base, e))
                else {
                    return err(format!("no expected file ({}.java or .kt)", base.display()));
                };
                let mut path = base.into_os_string();
                path.push(ext);
                Some(PathBuf::from(path))
            }
        };
        let compare = if raw.compare.is_empty() {
            let stem = class_files[0]
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy();
            let ext = expected
                .as_ref()
                .and_then(|e| e.extension())
                .map_or("java".into(), |e| e.to_string_lossy());
            let mut names = vec![format!("{stem}.{ext}")];
            if ext != "java" {
                names.push(format!("{stem}.java"));
            }
            names
        } else {
            raw.compare
        };
        let mut options = set.options.clone();
        for (k, v) in &raw.scoped.options {
            overlay_option(&mut options, k, v);
        }
        out.push(Case {
            id: raw.id,
            set: raw.set,
            set_slug: slug.clone(),
            options,
            class_files,
            libraries: raw.libs.iter().map(|c| resolve(c)).collect(),
            expected,
            compare,
            javadoc: raw.scoped.javadoc.or_else(|| set.javadoc.clone()),
            digest: raw.digest,
        });
    }
    Ok(out)
}

/// Each directory's file names, sorted, read once per manifest.
#[derive(Default)]
struct Listings(BTreeMap<std::ffi::OsString, Vec<String>>);

impl Listings {
    fn names(&mut self, dir: &Path) -> &[String] {
        self.0.entry(dir.as_os_str().to_owned()).or_insert_with(|| {
            let mut names: Vec<String> = std::fs::read_dir(dir)
                .into_iter()
                .flatten()
                .filter_map(|e| e.ok()?.file_name().into_string().ok())
                .collect();
            names.sort();
            names
        })
    }

    /// Whether `path` with `ext` appended names a file.
    fn has(&mut self, path: &Path, ext: &str) -> bool {
        let name = format!(
            "{}{ext}",
            path.file_name().unwrap_or_default().to_string_lossy()
        );
        let dir = path.parent().unwrap_or(Path::new("."));
        self.names(dir).binary_search(&name).is_ok()
    }

    /// `STEM.class`, then its `STEM$*.class` siblings in name order (the fixture's `addSource` order).
    fn expand(&mut self, stem: &Path) -> Vec<PathBuf> {
        let dir = stem.parent().unwrap_or(Path::new("."));
        let name = stem.file_name().unwrap_or_default().to_string_lossy();
        let prefix = format!("{name}$");
        let names = self.names(dir);
        let siblings = names[names.partition_point(|n| *n < prefix)..]
            .iter()
            .take_while(|n| n.starts_with(&prefix))
            .filter(|n| n.ends_with(".class"));
        std::iter::once(format!("{name}.class"))
            .chain(siblings.cloned())
            .map(|n| dir.join(n))
            .collect()
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
/// (`RuntimeException: java.util.concurrent.ExecutionException: NullPointerException: x` -> `NullPointerException: x`).
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
    let outcome = try_catch(|| {
        let mut fernflower = Fernflower::new(&engine_options(case), logger.clone());
        if let Some(provider) = case
            .javadoc
            .as_deref()
            .and_then(FixtureJavadocProvider::for_style)
        {
            DecompilerContext::set_javadoc_provider(Some(Rc::new(provider)));
        }
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
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CaseStatus {
    Match,
    Mismatch,
    /// The run threw.
    EngineError,
}

impl CaseStatus {
    pub fn name(self) -> &'static str {
        match self {
            CaseStatus::Match => "Match",
            CaseStatus::Mismatch => "Mismatch",
            CaseStatus::EngineError => "EngineError",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FirstDiff {
    /// 1-based, in the normalized texts; 0 for a digest case, whose two digests this holds.
    pub line: usize,
    /// `None` past the end of the text.
    pub expected: Option<String>,
    pub actual: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CaseResult {
    pub id: String,
    pub status: CaseStatus,
    /// The compared unit's text as the engine produced it; `None` when no unit had that name.
    pub actual: Option<String>,
    pub actual_path: Option<String>,
    pub first_diff: Option<FirstDiff>,
    /// What the engine threw.
    pub error: Option<Thrown>,
    /// A failure outside the engine (`harness: ...`).
    pub failure: Option<String>,
    /// Log lines at `Warn` and above.
    pub log: Vec<LogLine>,
}

impl CaseResult {
    fn empty(case: &Case, status: CaseStatus) -> CaseResult {
        CaseResult {
            id: case.id.clone(),
            status,
            actual: None,
            actual_path: None,
            first_diff: None,
            error: None,
            failure: None,
            log: Vec::new(),
        }
    }

    /// An `EngineError` that happened outside the engine (`harness: ...`).
    pub fn failed(case: &Case, failure: String) -> CaseResult {
        let mut r = CaseResult::empty(case, CaseStatus::EngineError);
        r.failure = Some(failure);
        r
    }
}

pub fn run_case(case: &Case) -> CaseResult {
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

/// The first 16 hex digits of the SHA-256 of `text` after the fixture's normalization.
pub fn text_digest(text: &str) -> String {
    Sha256::digest(normalize(text).as_bytes())[..8]
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

/// Compares what a run produced with the case's pinned digest, or else its expected text.
pub fn evaluate(case: &Case, run: EngineRun) -> CaseResult {
    let expected_raw = match case.expected.as_ref().filter(|_| case.digest.is_none()) {
        None => String::new(),
        Some(path) => match std::fs::read(path) {
            Ok(bytes) => String::from_utf8_lossy(&bytes).into_owned(),
            Err(e) => {
                return CaseResult::failed(
                    case,
                    format!("harness: cannot read {}: {e}", path.display()),
                )
            }
        },
    };
    evaluate_text(case, run, &expected_raw)
}

pub fn evaluate_text(case: &Case, run: EngineRun, expected_raw: &str) -> CaseResult {
    let expected = normalize(expected_raw);
    let selected = select_unit(&run.units, &case.compare);
    let first = match (selected, &case.digest) {
        (Some(unit), Some(pinned)) => Some(text_digest(&unit.content))
            .filter(|got| got != pinned)
            .map(|got| FirstDiff {
                line: 0,
                expected: Some(pinned.clone()),
                actual: Some(got),
            }),
        (Some(unit), None) => first_diff(&expected, &normalize(&unit.content)),
        (None, _) => Some(FirstDiff {
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
        status,
        actual: selected.map(|u| u.content.clone()),
        actual_path: selected.map(|u| u.path.clone()),
        first_diff: first,
        error: run.error,
        failure: None,
        log: run
            .log
            .into_iter()
            .filter(|l| l.severity >= Severity::Warn)
            .collect(),
    }
}

/// Runs `f` on a fresh thread with an [`engine_stack_bytes`] stack, guarded
/// against overflow (`rt::stack`).
pub fn with_engine_stack<R: Send>(f: impl FnOnce() -> R + Send) -> R {
    let stack = engine_stack_bytes();
    std::thread::scope(|scope| {
        let handle = std::thread::Builder::new()
            .name("kuna-jvm-engine".to_string())
            .stack_size(stack)
            .spawn_scoped(scope, move || {
                rt::stack::enter(stack);
                f()
            })
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

/// The failure key of something the engine threw.
pub fn thrown_key(t: &Thrown) -> String {
    let inner = t.innermost();
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

/// The failure key of a non-matching result; `None` for a match.
pub fn failure_key(r: &CaseResult) -> Option<String> {
    let digest = r.first_diff.as_ref().filter(|d| d.line == 0);
    if let (CaseStatus::Mismatch, Some(d)) = (r.status, digest) {
        let [pinned, got] = [&d.expected, &d.actual].map(|v| v.as_deref().unwrap_or("-"));
        return Some(format!(
            "digest {got}, pinned {pinned} (--out DIR writes the text)"
        ));
    }
    match r.status {
        CaseStatus::Match => None,
        CaseStatus::EngineError => Some(match (&r.failure, &r.error) {
            (Some(f), _) => head(f, 120),
            (None, Some(t)) => thrown_key(&located(t, &r.log)),
            (None, None) => "error".to_string(),
        }),
        CaseStatus::Mismatch => Some(match r.log.iter().find_map(|l| l.throwable.as_ref()) {
            Some(t) => thrown_key(t),
            None if r.actual.is_none() => "mismatch: no output".to_string(),
            None => "mismatch".to_string(),
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

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Totals {
    pub total: usize,
    pub matched: usize,
    pub mismatch: usize,
    pub error: usize,
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
        }
    }
    t
}

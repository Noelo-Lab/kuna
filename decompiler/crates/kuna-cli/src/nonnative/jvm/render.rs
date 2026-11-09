//! What `decompile`, `decompile-all` and `functions` print for a JVM input.
//!
//! Text is the engine's: Java, or Kotlin for a class the Kotlin plugin claims.
//! A class is its compilation unit exactly as the engine writes it, a method
//! its slice of that unit (by the engine's method spans) with the common
//! indentation removed; JVM methods that share a span, such as a Kotlin
//! property's getter and setter, print it once. The JSON documents keep the
//! native shapes (`decompile-all --json`, `functions --json`, see
//! `docs/cli.md`) and add what a method needs to be named: `class`, `method`,
//! `descriptor`, `access`, `path`, `language` and `line`, and what
//! [`failures`] reports: `nested_errors` and `warnings` per method, `status`
//! per class. Addresses are `null`, `size` is the bytecode length, and
//! `line_mappings` map lines of `code` to bytecode offsets.

use std::collections::BTreeSet;
use std::fmt::Write as _;
use std::sync::Arc;

use kuna_jvm::fernflower::DecompiledClass;
use kuna_jvm::kuna_input::SharedRuntime;
use kuna_jvm::kuna_inventory::{class_access_names, method_access_names};
use kuna_jvm::kuna_spans::MethodSpan;

use super::args::{Args, SortKey, Triage};
use super::failures::{self, Nested, Status, Units};
use super::run::{self, Loaded, Outcome, Watch};
use super::select::{java_name, Target, Universe};
use crate::jsonfmt::{self, Json};

/// One method as a record.
pub(crate) struct Record {
    pub(crate) class: String,
    pub(crate) method: String,
    pub(crate) descriptor: String,
    pub(crate) access: i32,
    pub(crate) size: u64,
    pub(crate) code: String,
    /// The method's own failure, else the bodies written inside it that failed.
    pub(crate) error: Option<String>,
    pub(crate) own_error: Option<String>,
    pub(crate) nested: Vec<Nested>,
    pub(crate) warnings: Vec<String>,
    pub(crate) path: String,
    pub(crate) language: &'static str,
    pub(crate) line: u32,
    pub(crate) lines: Vec<(u32, Vec<u32>)>,
    /// The span's byte range in its unit.
    span: (usize, usize),
}

impl Record {
    pub(crate) fn name(&self) -> String {
        format!("{}.{}", java_name(&self.class), self.method)
    }

    pub(crate) fn signature(&self) -> String {
        format!("{}{}", self.name(), self.descriptor)
    }

    fn from_span(unit: &DecompiledClass, span: &MethodSpan, universe: &Universe) -> Record {
        let raw = unit.content.get(span.start..span.end).unwrap_or("");
        let lines = span
            .lines
            .iter()
            .filter(|(line, _)| (span.start_line..=span.end_line).contains(line))
            .map(|(line, offsets)| (line - span.start_line + 1, offsets.clone()))
            .collect();
        let size = universe
            .method(&span.class, &span.name, &span.descriptor)
            .and_then(|m| m.code_length)
            .unwrap_or(0);
        let nested = Nested::all(&span.nested);
        Record {
            class: span.class.clone(),
            method: span.name.clone(),
            descriptor: span.descriptor.clone(),
            access: span.access,
            size: u64::from(size),
            code: dedent(raw),
            error: failures::method_error(span.error.as_deref(), &nested),
            own_error: span.error.clone(),
            nested,
            warnings: span.warnings.clone(),
            path: unit.path.clone(),
            language: unit.language(),
            line: span.start_line,
            lines,
            span: (span.start, span.end),
        }
    }

    /// Whether `other` is the same text of the same unit (a property's accessors).
    fn shares_text(&self, other: &Record) -> bool {
        self.path == other.path && self.span == other.span
    }

    fn json(&self, omit_line_mappings: bool) -> Json {
        let mut record = vec![
            ("name".into(), Json::Str(self.name())),
            ("address".into(), Json::Null),
            ("address_hex".into(), Json::Null),
            ("aliases".into(), Json::Array(Vec::new())),
            ("object_location".into(), Json::Null),
            ("size".into(), number(self.size)),
            ("code".into(), Json::Str(self.code.clone())),
            ("error".into(), optional(self.error.as_deref())),
            ("unstructured_gotos".into(), number(0)),
        ];
        if !omit_line_mappings {
            let mappings = self
                .lines
                .iter()
                .map(|(line, offsets)| {
                    Json::Object(vec![
                        ("line_number".into(), number(*line)),
                        ("addresses".into(), Json::Array(offsets.iter().map(|o| number(*o)).collect())),
                    ])
                })
                .collect();
            record.push(("line_mappings".into(), Json::Array(mappings)));
        }
        record.extend([
            ("class".into(), Json::Str(java_name(&self.class))),
            ("method".into(), Json::Str(self.method.clone())),
            ("descriptor".into(), Json::Str(self.descriptor.clone())),
            ("access".into(), access_json(method_access_names(self.access as u16))),
            ("path".into(), Json::Str(self.path.clone())),
            ("language".into(), Json::Str(self.language.into())),
            ("line".into(), number(self.line)),
            ("nested_errors".into(), Nested::json(&self.nested)),
            (
                "warnings".into(),
                Json::Array(self.warnings.iter().map(|w| Json::Str(w.clone())).collect()),
            ),
        ]);
        Json::Object(record)
    }
}

/// One written class: a compilation unit, or a nested class's part of one.
struct ClassBlock {
    name: String,
    path: Option<String>,
    language: Option<&'static str>,
    line: Option<u32>,
    size: u64,
    code: Option<String>,
    status: Status,
    /// Why the engine could not decompile the class (`Status::Failed`).
    error: Option<String>,
    /// Why a hidden class has no text.
    note: Option<String>,
    /// Failed bodies in the class's text outside its methods.
    nested: Vec<Nested>,
}

impl ClassBlock {
    /// `class`, written in `unit` (`root_size`: the unit's class-file bytes).
    fn written(unit: &DecompiledClass, class: &str, universe: &Universe, root_size: u64) -> ClassBlock {
        let block = |status, code, line, error, note| ClassBlock {
            name: java_name(class),
            path: (status != Status::Hidden).then(|| unit.path.clone()),
            language: (status != Status::Hidden).then(|| unit.language()),
            line,
            size: if class == unit.qualified_name {
                root_size
            } else {
                universe.get(class).map_or(0, |c| c.size as u64)
            },
            code,
            status,
            error,
            note,
            nested: failures::outside_methods(unit, class),
        };
        match class_slice(unit, class) {
            Some((code, line)) => {
                let line = (class != unit.qualified_name).then_some(line);
                match failures::class_failure(unit, class) {
                    Some(why) => block(Status::Failed, Some(code), line, Some(why), None),
                    None => block(Status::Decompiled, Some(code), line, None, None),
                }
            }
            None => match failures::unit_failure(unit) {
                Some(why) => {
                    let why = format!("{} could not be decompiled: {why}", java_name(&unit.qualified_name));
                    block(Status::Failed, None, None, Some(why), None)
                }
                None => {
                    let why = hidden_member(&Target::Class(class.to_string()), unit.language() == "kotlin");
                    block(Status::Hidden, None, None, None, Some(why))
                }
            },
        }
    }

    /// `class`, whose unit was not delivered, for `why`.
    fn missing(class: &str, size: u64, why: String) -> ClassBlock {
        ClassBlock {
            name: java_name(class),
            path: None,
            language: None,
            line: None,
            size,
            code: None,
            status: Status::Failed,
            error: Some(why),
            note: None,
            nested: Vec::new(),
        }
    }

    fn json(&self) -> Json {
        Json::Object(vec![
            ("name".into(), Json::Str(self.name.clone())),
            ("path".into(), optional(self.path.as_deref())),
            ("language".into(), optional(self.language)),
            ("size".into(), number(self.size)),
            ("code".into(), optional(self.code.as_deref())),
            ("status".into(), Json::Str(self.status.name().into())),
            ("error".into(), optional(self.error.as_deref())),
            ("note".into(), optional(self.note.as_deref())),
            ("nested_errors".into(), Nested::json(&self.nested)),
        ])
    }

    fn text(&self, out: &mut String) {
        match (&self.code, &self.path) {
            (Some(code), Some(path)) => {
                let at = self.line.map(|l| format!(":{l}")).unwrap_or_default();
                let _ = writeln!(out, "// Class: {}  ({path}{at})", self.name);
                out.push_str(code);
                if !code.ends_with('\n') {
                    out.push('\n');
                }
                out.push('\n');
            }
            _ => {
                let (label, why) = match self.status {
                    Status::Hidden => ("hidden", self.note.as_deref()),
                    _ => ("error", self.error.as_deref()),
                };
                let _ = writeln!(out, "// Class: {}  ({label}: {})\n", self.name, why.unwrap_or("no output"));
            }
        }
    }
}

fn number(n: impl ToString) -> Json {
    Json::Number(n.to_string())
}

fn optional(s: Option<&str>) -> Json {
    s.map_or(Json::Null, |s| Json::Str(s.to_string()))
}

fn access_json(names: Vec<&'static str>) -> Json {
    Json::Array(names.into_iter().map(|n| Json::Str(n.into())).collect())
}

/// `text` with the indentation every non-blank line shares removed, without
/// its final newline.
pub(crate) fn dedent(text: &str) -> String {
    let leading = |line: &str| line.len() - line.trim_start_matches([' ', '\t']).len();
    let indent = text
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(leading)
        .min()
        .unwrap_or(0);
    let mut out = String::with_capacity(text.len());
    for line in text.split_inclusive('\n') {
        out.push_str(&line[indent.min(leading(line))..]);
    }
    if out.ends_with('\n') {
        out.pop();
    }
    out
}

fn runtime_json(runtime: Option<&Arc<SharedRuntime>>) -> Json {
    match runtime {
        Some(rt) => Json::Object(vec![
            ("path".into(), Json::Str(rt.home().display().to_string())),
            ("version".into(), optional(rt.version())),
        ]),
        None => Json::Null,
    }
}

/// The language of the units a document holds: `java` or `kotlin`, `mixed`
/// for both, `null` for none.
fn language_of(records: &[Record], classes: &[ClassBlock]) -> Json {
    let found: BTreeSet<&str> = records
        .iter()
        .map(|r| r.language)
        .chain(classes.iter().filter_map(|c| c.language))
        .collect();
    match found.len() {
        0 => Json::Null,
        1 => Json::Str(found.into_iter().next().unwrap_or_default().into()),
        _ => Json::Str("mixed".into()),
    }
}

/// The document head of `decompile --json` and `decompile-all --json`.
fn document(
    args: &Args,
    language: Json,
    count: usize,
    total: Option<usize>,
    error: Option<&str>,
    runtime: Option<&Arc<SharedRuntime>>,
) -> Vec<(String, Json)> {
    let mut omitted = Vec::new();
    if args.omit_line_mappings {
        omitted.push("line_mappings");
    }
    omitted.extend(["variables", "types", "stack_objects"]);
    if args.omit_classes {
        omitted.push("classes");
    }
    let mut head = vec![
        ("binary".to_string(), Json::Str(args.input.clone())),
        ("engine".to_string(), Json::Str("jvm".into())),
        ("language".to_string(), language),
        (
            "omitted".to_string(),
            Json::Array(omitted.into_iter().map(|k| Json::Str(k.into())).collect()),
        ),
        ("count".to_string(), number(count)),
    ];
    if let Some(total) = total {
        head.push(("total".to_string(), number(total)));
    }
    head.push(("error".to_string(), optional(error)));
    head.push(("runtime".to_string(), runtime_json(runtime)));
    head
}

fn emit_document(
    args: &Args,
    head: Vec<(String, Json)>,
    records: &[Record],
    classes: &[ClassBlock],
    status: i32,
) -> i32 {
    let mut tail = Vec::new();
    if !args.omit_classes {
        tail.push(("classes".to_string(), Json::Array(classes.iter().map(ClassBlock::json).collect())));
    }
    tail.push(("assertions".to_string(), Json::Array(Vec::new())));
    let items = records.iter().map(|r| r.json(args.omit_line_mappings));
    match crate::output::emit_streamed(|out| {
        jsonfmt::write_indent2_streamed(out, &head, "functions", items, &tail)?;
        out.write_all(b"\n")
    }) {
        Ok(()) => status,
        Err(e) => crate::output::status_after(e, status),
    }
}

/// The part of `unit` that `class` (written inside it) covers, with its first line.
fn class_slice(unit: &DecompiledClass, class: &str) -> Option<(String, u32)> {
    if unit.qualified_name == class {
        return Some((unit.content.clone(), 1));
    }
    let span = unit.classes.iter().find(|c| c.internal == class)?;
    let text = unit.content.get(span.start..span.end)?;
    Some((dedent(text), span.start_line))
}

/// Whether `span` lies inside the part of `unit` that `class` covers.
fn within_class(unit: &DecompiledClass, class: &str, span: &MethodSpan) -> bool {
    if unit.qualified_name == class {
        return true;
    }
    unit.classes
        .iter()
        .find(|c| c.internal == class)
        .is_some_and(|c| c.start <= span.start && span.end <= c.end)
}

/// Why `target` has no text in its unit, and what prints it; `kotlin` when
/// the unit is Kotlin, whose writer prints declarations rather than methods.
fn hidden_member(target: &Target, kotlin: bool) -> String {
    match target {
        Target::Methods { class, name, .. } if kotlin => format!(
            "{}.{name} has no declaration of its own in the Kotlin source of {}: the Kotlin writer \
             puts a primary constructor in the class header and folds lambda bodies, $default \
             methods and @JvmStatic forwarders into the declarations they serve (--option \
             jvm_kt_decompile_facades on writes a multifile facade's functions); --language java \
             (jvm_kt_enable off) writes the class as Java, method by method, and --option \
             jvm_remove_synthetic off adds its synthetic methods",
            java_name(class),
            java_name(class)
        ),
        Target::Methods { class, name, .. } if name == "<init>" => format!(
            "{}.<init> is not in the decompiled source of {} (the engine leaves out default \
             constructors, and always an anonymous class's; --option jvm_hide_default_constructor off \
             prints the default ones)",
            java_name(class),
            java_name(class)
        ),
        Target::Methods { class, name, .. } => format!(
            "{}.{name} is not in the decompiled source of {} (the engine folds synthetic and bridge \
             methods into their users; --option jvm_remove_synthetic off and jvm_remove_bridge off \
             print them)",
            java_name(class),
            java_name(class)
        ),
        Target::Class(class) => format!(
            "{} is not in the decompiled source: the engine writes an anonymous or local class inside \
             the method that creates it, so not when that method failed, and leaves synthetic classes out \
             (--option jvm_remove_synthetic off prints them)",
            java_name(class)
        ),
    }
}

/// Opens the runtime and drives the engine over `select`, handing the outcome to `finish`.
pub(crate) fn execute(
    args: &Args,
    loaded: &Loaded,
    select: Option<BTreeSet<String>>,
    finish: &(dyn Fn(Outcome, Option<&Arc<SharedRuntime>>) -> i32 + Sync),
) -> i32 {
    let runtime = match run::open_runtime(&args.runtime) {
        Ok(runtime) => runtime,
        Err(e) => {
            eprintln!("error: {e}");
            return 1;
        }
    };
    let watch = Arc::new(Watch::default());
    let config = match run::config(args, runtime.clone(), select, &watch) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("error: {e}");
            return 1;
        }
    };
    run::drive(loaded, args, &config, &watch, &|outcome| finish(outcome, runtime.as_ref()))
}

pub(crate) fn load_or_report(args: &Args) -> Result<Loaded, i32> {
    run::load(args).map_err(|e| {
        eprintln!("error: {e}");
        1
    })
}

// --- decompile ---------------------------------------------------------------

pub(crate) fn decompile(args: &Args) -> i32 {
    let loaded = match load_or_report(args) {
        Ok(loaded) => loaded,
        Err(status) => return status,
    };
    let own = loaded.own_class();
    let target = match args.target.as_deref() {
        Some(text) => loaded.universe.resolve(text, own.as_deref(), &args.input),
        None => own.clone().map(Target::Class).ok_or_else(|| "decompile requires a class or method".to_string()),
    };
    let target = match target {
        Ok(target) => target,
        Err(e) => {
            eprintln!("error: {e}");
            return 1;
        }
    };
    let select: BTreeSet<String> = loaded.universe.chain(target.class()).into_iter().collect();
    execute(args, &loaded, Some(select), &|outcome, runtime| {
        finish_decompile(args, &loaded, &target, &outcome, runtime)
    })
}

fn finish_decompile(
    args: &Args,
    loaded: &Loaded,
    target: &Target,
    outcome: &Outcome,
    runtime: Option<&Arc<SharedRuntime>>,
) -> i32 {
    let universe = &loaded.universe;
    let failed = |name: &str, why: String, unit: Option<&DecompiledClass>, block: Option<ClassBlock>| {
        let message = format!("could not decompile {}: {why}", java_name(name));
        if args.json {
            let language = unit.map_or(Json::Null, |u| Json::Str(u.language().into()));
            let head = document(args, language, 0, None, Some(&message), runtime);
            emit_document(args, head, &[], block.as_slice(), 1);
        } else if let Some(mut code) = block.and_then(|b| b.code) {
            if !code.ends_with('\n') {
                code.push('\n');
            }
            crate::output::emit_with_status(&code, 1);
        }
        eprintln!("error: {message}");
        1
    };
    let units = Units::new(outcome);
    let result = match units.find(universe, target.class()) {
        Ok(result) => result,
        Err(why) => return failed(&universe.root_of(target.class()), why, None, None),
    };
    let root = result.qualified_name.as_str();
    let Some(unit) = &result.unit else {
        let why = result.error.clone().unwrap_or_else(|| "no output".into());
        return failed(root, why, None, None);
    };
    let unit_failed = || failures::unit_failure(unit);
    let hidden = || hidden_member(target, unit.language() == "kotlin");
    let (records, classes): (Vec<Record>, Vec<ClassBlock>) = match target {
        Target::Class(class) => {
            let root_size = units.sizes(universe).get(root).copied().unwrap_or(0);
            let block = ClassBlock::written(unit, class, universe, root_size);
            match block.status {
                Status::Hidden => return failed(root, hidden(), Some(unit), None),
                Status::Failed => {
                    let why = block.error.clone().unwrap_or_default();
                    let name = if block.code.is_some() { class.as_str() } else { root };
                    let why = if block.code.is_some() { why } else { unit_failed().unwrap_or(why) };
                    return failed(name, why, Some(unit), Some(block));
                }
                Status::Decompiled => {}
            }
            let records = unit
                .methods
                .iter()
                .filter(|span| within_class(unit, class, span))
                .map(|span| Record::from_span(unit, span, universe))
                .collect();
            (records, vec![block])
        }
        Target::Methods { .. } => {
            let records: Vec<Record> = unit
                .methods
                .iter()
                .filter(|s| target.matches(&s.class, &s.name, &s.descriptor))
                .map(|span| Record::from_span(unit, span, universe))
                .collect();
            if records.is_empty() {
                let why = unit_failed().unwrap_or_else(hidden);
                return failed(root, why, Some(unit), None);
            }
            (records, Vec::new())
        }
    };
    if records.len() > 1 && matches!(target, Target::Methods { descriptor: None, .. }) {
        eprintln!(
            "note: {} has {} overloads, printed in source order; add a descriptor such as {:?} to pick one",
            records[0].name(),
            records.len(),
            records[0].signature()
        );
    }
    let all_failed = matches!(target, Target::Methods { .. }) && records.iter().all(|r| r.error.is_some());
    let class_level = classes.first().map(|b| (target.class(), b.nested.as_slice()));
    failures::warn(&records, class_level);
    let status = i32::from(all_failed);
    if args.json {
        let error = all_failed.then(|| format!("could not decompile {}", records[0].name()));
        let language = language_of(&records, &classes);
        let head = document(args, language, records.len(), None, error.as_deref(), runtime);
        return emit_document(args, head, &records, &classes, status);
    }
    let text = match target {
        Target::Class(_) => {
            let mut code = classes[0].code.clone().unwrap_or_default();
            if !code.ends_with('\n') {
                code.push('\n');
            }
            code
        }
        Target::Methods { .. } => {
            let mut text = String::new();
            for (i, (record, _)) in shared_text(&records).into_iter().enumerate() {
                if i > 0 {
                    text.push('\n');
                }
                text.push_str(&record.code);
                text.push('\n');
            }
            text
        }
    };
    crate::output::emit_with_status(&text, status)
}

/// `records` with each text once, in order: a record and the signatures of
/// every record that shares its text.
fn shared_text(records: &[Record]) -> Vec<(&Record, Vec<String>)> {
    let mut out: Vec<(&Record, Vec<String>)> = Vec::new();
    for record in records {
        match out.iter_mut().find(|(first, _)| first.shares_text(record)) {
            Some((_, signatures)) => signatures.push(record.signature()),
            None => out.push((record, vec![record.signature()])),
        }
    }
    out
}

// --- decompile-all -----------------------------------------------------------

/// One inventory method, as triage sees it.
struct Candidate<'a> {
    class: &'a str,
    name: &'a str,
    descriptor: &'a str,
    size: u64,
    order: usize,
}

impl Candidate<'_> {
    fn signature(&self) -> String {
        format!("{}.{}{}", java_name(self.class), self.name, self.descriptor)
    }
}

fn candidates(universe: &Universe) -> Vec<Candidate<'_>> {
    let mut out = Vec::new();
    for class in &universe.classes {
        for method in &class.methods {
            out.push(Candidate {
                class: &class.internal,
                name: &method.name,
                descriptor: &method.descriptor,
                size: u64::from(method.code_length.unwrap_or(0)),
                order: out.len(),
            });
        }
    }
    out
}

fn passes(triage: &Triage, signature: &str, size: u64) -> bool {
    triage.filter.as_ref().is_none_or(|re| re.is_match(signature))
        && triage.min_size.is_none_or(|min| size >= min)
        && triage.max_size.is_none_or(|max| size <= max)
}

/// Sorts by `--sort` (sizes largest first) and applies `--limit`.
fn order_and_limit<T>(items: &mut Vec<T>, triage: &Triage, key: impl Fn(&T) -> (u64, String, usize)) {
    match triage.sort.unwrap_or_default() {
        SortKey::Addr => items.sort_by_key(|i| key(i).2),
        SortKey::Size => items.sort_by(|a, b| {
            let (ka, kb) = (key(a), key(b));
            kb.0.cmp(&ka.0).then(ka.2.cmp(&kb.2))
        }),
        SortKey::Name => items.sort_by(|a, b| {
            let (ka, kb) = (key(a), key(b));
            ka.1.cmp(&kb.1).then(ka.2.cmp(&kb.2))
        }),
    }
    if let Some(limit) = triage.limit {
        items.truncate(limit);
    }
}

/// Whether `class` is `ancestor` or written inside it.
fn inside(universe: &Universe, class: &str, ancestor: &str) -> bool {
    let mut current = class.to_string();
    for _ in 0..256 {
        if current == ancestor {
            return true;
        }
        match universe.get(&current).and_then(|c| c.outer.clone()) {
            Some(outer) if outer != current => current = outer,
            _ => return false,
        }
    }
    false
}

pub(crate) fn decompile_all(args: &Args) -> i32 {
    let loaded = match load_or_report(args) {
        Ok(loaded) => loaded,
        Err(status) => return status,
    };
    let universe = &loaded.universe;
    let own = loaded.own_class();
    let mut targets = Vec::new();
    for text in args.functions.iter().flatten() {
        match universe.resolve(text, own.as_deref(), &args.input) {
            Ok(target) => targets.push(target),
            Err(e) => {
                eprintln!("error: {e}");
                return 1;
            }
        }
    }
    let by_method = args.triage.narrows() || targets.iter().any(|t| matches!(t, Target::Methods { .. }));
    let selected = |class: &str, name: &str, descriptor: &str| {
        targets.is_empty()
            || targets.iter().any(|t| match t {
                Target::Class(c) => inside(universe, class, c),
                Target::Methods { .. } => t.matches(class, name, descriptor),
            })
    };
    let total = candidates(universe).len();
    let wanted: Option<BTreeSet<String>> = if targets.is_empty() && !args.triage.narrows() {
        None
    } else {
        let mut picked: Vec<Candidate<'_>> = candidates(universe)
            .into_iter()
            .filter(|c| selected(c.class, c.name, c.descriptor))
            .filter(|c| passes(&args.triage, &c.signature(), c.size))
            .collect();
        if args.triage.limit.is_some() {
            picked.retain(|c| {
                universe
                    .method(c.class, c.name, c.descriptor)
                    .is_some_and(|m| m.access & 0x1040 == 0)
            });
        }
        order_and_limit(&mut picked, &args.triage, |c| (c.size, c.signature(), c.order));
        let mut classes: BTreeSet<String> = picked.iter().map(|c| c.class.to_string()).collect();
        for target in &targets {
            if let Target::Class(class) = target {
                classes.insert(class.clone());
            }
        }
        Some(classes)
    };
    let roots = wanted
        .as_ref()
        .map(|classes| classes.iter().flat_map(|c| universe.chain(c)).collect::<BTreeSet<String>>());
    let shape = Shape {
        targets: &targets,
        wanted: wanted.as_ref(),
        by_method,
        selected: &selected,
        total,
    };
    execute(args, &loaded, roots, &|outcome, runtime| {
        let status = finish_decompile_all(args, &loaded, &shape, &outcome, runtime);
        run::abandon(outcome);
        status
    })
}

/// What `decompile-all` was asked for.
struct Shape<'a> {
    targets: &'a [Target],
    /// The classes selected (`--functions`, triage); `None`: every class.
    wanted: Option<&'a BTreeSet<String>>,
    by_method: bool,
    selected: &'a (dyn Fn(&str, &str, &str) -> bool + Sync),
    total: usize,
}

fn finish_decompile_all(
    args: &Args,
    loaded: &Loaded,
    shape: &Shape<'_>,
    outcome: &Outcome,
    runtime: Option<&Arc<SharedRuntime>>,
) -> i32 {
    let universe = &loaded.universe;
    let (targets, by_method) = (shape.targets, shape.by_method);
    let units = Units::new(outcome);
    let relevant = units.holding(universe, shape.wanted);
    let mut records: Vec<(Record, usize)> = Vec::new();
    for result in &relevant {
        let Some(unit) = &result.unit else { continue };
        for span in &unit.methods {
            if !(shape.selected)(&span.class, &span.name, &span.descriptor) {
                continue;
            }
            let record = Record::from_span(unit, span, universe);
            if passes(&args.triage, &record.signature(), record.size) {
                let order = records.len();
                records.push((record, order));
            }
        }
    }
    order_and_limit(&mut records, &args.triage, |(r, order)| (r.size, r.signature(), *order));
    let records: Vec<Record> = records.into_iter().map(|(r, _)| r).collect();

    let class_roots: Vec<String> = if targets.is_empty() {
        if args.triage.narrows() {
            Vec::new()
        } else {
            relevant
                .iter()
                .map(|r| r.qualified_name.clone())
                .chain(outcome.missing.iter().map(|(n, _)| n.clone()))
                .collect()
        }
    } else {
        targets
            .iter()
            .filter_map(|t| match t {
                Target::Class(class) => Some(class.clone()),
                Target::Methods { .. } => None,
            })
            .collect()
    };
    let sizes = units.sizes(universe);
    let unit_size = |root: &str| sizes.get(root).copied().unwrap_or(0);
    let classes: Vec<ClassBlock> = class_roots
        .iter()
        .map(|class| match units.find(universe, class) {
            Ok(result) => match &result.unit {
                Some(unit) => ClassBlock::written(unit, class, universe, unit_size(&result.qualified_name)),
                None => ClassBlock::missing(
                    class,
                    unit_size(&result.qualified_name),
                    result.error.clone().unwrap_or_else(|| "no output".into()),
                ),
            },
            Err(why) => ClassBlock::missing(class, unit_size(&universe.root_of(class)), why),
        })
        .collect();

    let produced = relevant.iter().any(|r| failures::result_failure(r).is_none());
    let error = match (&outcome.error, produced) {
        (Some(e), _) => Some(e.clone()),
        (None, false) => Some(format!("no class of {} could be decompiled", args.input)),
        (None, true) => None,
    };
    let status = i32::from(!produced || outcome.error.is_some());
    if let Some(e) = &error {
        eprintln!("error: {e}");
    }
    for (name, why) in &outcome.missing {
        eprintln!("warning: could not decompile {}: {why}", java_name(name));
    }
    for result in &relevant {
        if let Some(why) = failures::result_failure(result) {
            eprintln!("warning: could not decompile {}: {why}", java_name(&result.qualified_name));
        }
    }
    if args.json {
        let total = by_method.then_some(shape.total);
        let language = language_of(&records, &classes);
        let head = document(args, language, records.len(), total, error.as_deref(), runtime);
        return emit_document(args, head, &records, &classes, status);
    }
    let mut text = String::new();
    if by_method {
        for block in classes.iter() {
            block.text(&mut text);
        }
        let in_class_block = |record: &Record| {
            targets
                .iter()
                .any(|t| matches!(t, Target::Class(c) if inside(universe, &record.class, c)))
        };
        let outside: Vec<Record> = records.into_iter().filter(|r| !in_class_block(r)).collect();
        for (record, signatures) in shared_text(&outside) {
            let _ = writeln!(text, "// Method: {}  ({}:{})", signatures.join(", "), record.path, record.line);
            text.push_str(&record.code);
            text.push_str("\n\n");
        }
    } else {
        for block in &classes {
            block.text(&mut text);
        }
    }
    crate::output::emit_with_status(&text, status)
}

// --- functions -----------------------------------------------------------------

pub(crate) fn functions(args: &Args) -> i32 {
    let loaded = match load_or_report(args) {
        Ok(loaded) => loaded,
        Err(status) => return status,
    };
    let universe = &loaded.universe;
    let all = candidates(universe);
    let total = all.len();
    let mut listed: Vec<Candidate<'_>> = all
        .into_iter()
        .filter(|c| passes(&args.triage, &c.signature(), c.size))
        .collect();
    order_and_limit(&mut listed, &args.triage, |c| (c.size, c.signature(), c.order));
    if !args.json {
        let mut text = String::new();
        for c in &listed {
            let _ = writeln!(text, "{}", c.signature());
        }
        return crate::output::emit_with_status(&text, 0);
    }
    let owners: BTreeSet<&str> = listed.iter().map(|c| c.class).collect();
    let functions: Vec<Json> = listed
        .iter()
        .map(|c| {
            let access = universe
                .method(c.class, c.name, c.descriptor)
                .map_or(0, |m| m.access);
            Json::Object(vec![
                ("name".into(), Json::Str(format!("{}.{}", java_name(c.class), c.name))),
                ("address".into(), Json::Null),
                ("address_hex".into(), Json::Null),
                ("aliases".into(), Json::Array(Vec::new())),
                ("object_location".into(), Json::Null),
                ("size".into(), number(c.size)),
                ("class".into(), Json::Str(java_name(c.class))),
                ("method".into(), Json::Str(c.name.to_string())),
                ("descriptor".into(), Json::Str(c.descriptor.to_string())),
                ("access".into(), access_json(method_access_names(access))),
            ])
        })
        .collect();
    let classes: Vec<Json> = universe
        .classes
        .iter()
        .filter(|c| !args.triage.narrows() || owners.contains(c.internal.as_str()))
        .map(|c| {
            let flags = c.inner_access.unwrap_or(c.access);
            Json::Object(vec![
                ("name".into(), Json::Str(java_name(&c.internal))),
                ("outer".into(), c.outer.as_deref().map_or(Json::Null, |o| Json::Str(java_name(o)))),
                ("source_file".into(), optional(c.source_file.as_deref())),
                ("size".into(), number(c.size)),
                ("access".into(), access_json(class_access_names(flags))),
            ])
        })
        .collect();
    let doc = Json::Object(vec![
        ("binary".into(), Json::Str(args.input.clone())),
        ("engine".into(), Json::Str("jvm".into())),
        ("count".into(), number(listed.len())),
        ("total".into(), number(total)),
        ("error".into(), Json::Null),
        ("functions".into(), Json::Array(functions)),
        ("classes".into(), Json::Array(classes)),
    ]);
    crate::output::emit_with_status(&format!("{}\n", jsonfmt::dumps_indent2(&doc)), 0)
}

#[cfg(test)]
mod tests {
    use super::dedent;

    #[test]
    fn dedent_removes_the_shared_indent_and_the_final_newline() {
        assert_eq!(dedent("   int f() {\n      return 1;\n   }\n"), "int f() {\n   return 1;\n}");
        assert_eq!(dedent("   a\n\n   b\n"), "a\n\nb");
        assert_eq!(dedent("x\n"), "x");
    }
}

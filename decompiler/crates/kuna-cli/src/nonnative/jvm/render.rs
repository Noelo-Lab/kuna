//! What `decompile`, `decompile-all` and `functions` print for a JVM input.
//!
//! Text is Java: a class is its compilation unit exactly as the engine writes
//! it, a method its slice of that unit with the common indentation removed.
//! The JSON documents keep the native shapes (`decompile-all --json`,
//! `functions --json`, see `docs/cli.md`) and add what a method needs to be
//! named: `class`, `method`, `descriptor`, `access`, `path` and `line`.
//! Addresses are `null`, `size` is the bytecode length, and `line_mappings`
//! map lines of `code` to bytecode offsets.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::sync::Arc;

use kuna_jvm::fernflower::DecompiledClass;
use kuna_jvm::kuna_input::SharedRuntime;
use kuna_jvm::kuna_inventory::{class_access_names, method_access_names};
use kuna_jvm::kuna_spans::MethodSpan;

use super::args::{Args, SortKey, Triage};
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
    pub(crate) error: Option<String>,
    pub(crate) path: String,
    pub(crate) line: u32,
    pub(crate) lines: Vec<(u32, Vec<u32>)>,
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
        Record {
            class: span.class.clone(),
            method: span.name.clone(),
            descriptor: span.descriptor.clone(),
            access: span.access,
            size: u64::from(size),
            code: dedent(raw),
            error: span.error.clone(),
            path: unit.path.clone(),
            line: span.start_line,
            lines,
        }
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
            ("line".into(), number(self.line)),
        ]);
        Json::Object(record)
    }
}

/// One written class: a compilation unit, or a nested class's part of one.
struct ClassBlock {
    name: String,
    path: Option<String>,
    line: Option<u32>,
    size: u64,
    code: Option<String>,
    error: Option<String>,
}

impl ClassBlock {
    fn json(&self) -> Json {
        Json::Object(vec![
            ("name".into(), Json::Str(self.name.clone())),
            ("path".into(), optional(self.path.as_deref())),
            ("size".into(), number(self.size)),
            ("code".into(), optional(self.code.as_deref())),
            ("error".into(), optional(self.error.as_deref())),
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
                let _ = writeln!(
                    out,
                    "// Class: {}  (error: {})\n",
                    self.name,
                    self.error.as_deref().unwrap_or("no output")
                );
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

/// The bytes of the class files each root class is written from.
fn unit_sizes(universe: &Universe) -> BTreeMap<String, u64> {
    let mut sizes = BTreeMap::new();
    for class in &universe.classes {
        *sizes.entry(universe.root_of(&class.internal)).or_insert(0) += class.size as u64;
    }
    sizes
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

/// The document head of `decompile --json` and `decompile-all --json`.
fn document(
    args: &Args,
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
        ("language".to_string(), Json::Str("java".into())),
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

/// The unit delivered for `root`, or why there is none.
fn unit_of<'a>(outcome: &'a Outcome, root: &str) -> Result<&'a DecompiledClass, String> {
    if let Some(result) = outcome.results.iter().find(|r| r.qualified_name == root) {
        return result
            .unit
            .as_ref()
            .ok_or_else(|| result.error.clone().unwrap_or_else(|| "no output".into()));
    }
    Err(outcome
        .missing
        .iter()
        .find(|(name, _)| name == root)
        .map(|(_, why)| why.clone())
        .or_else(|| outcome.error.clone())
        .unwrap_or_else(|| "no output".into()))
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

fn hidden_method(target: &Target) -> String {
    match target {
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
            "{} is not in the decompiled source (--option jvm_remove_synthetic off prints synthetic classes)",
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
    let root = loaded.universe.root_of(target.class());
    let select = BTreeSet::from([root.clone()]);
    execute(args, &loaded, Some(select), &|outcome, runtime| {
        finish_decompile(args, &loaded, &target, &root, &outcome, runtime)
    })
}

fn finish_decompile(
    args: &Args,
    loaded: &Loaded,
    target: &Target,
    root: &str,
    outcome: &Outcome,
    runtime: Option<&Arc<SharedRuntime>>,
) -> i32 {
    let failed = |why: String| {
        let message = format!("could not decompile {}: {why}", java_name(root));
        if args.json {
            let head = document(args, 0, None, Some(&message), runtime);
            emit_document(args, head, &[], &[], 1);
        }
        eprintln!("error: {message}");
        1
    };
    let unit = match unit_of(outcome, root) {
        Ok(unit) => unit,
        Err(why) => return failed(why),
    };
    let (records, classes): (Vec<Record>, Vec<ClassBlock>) = match target {
        Target::Class(class) => {
            let Some((code, line)) = class_slice(unit, class) else {
                return failed(hidden_method(target));
            };
            let records = unit
                .methods
                .iter()
                .filter(|span| within_class(unit, class, span))
                .map(|span| Record::from_span(unit, span, &loaded.universe))
                .collect();
            let block = ClassBlock {
                name: java_name(class),
                path: Some(unit.path.clone()),
                line: (class != root).then_some(line),
                size: if class == root {
                    unit_sizes(&loaded.universe).get(root).copied().unwrap_or(0)
                } else {
                    loaded.universe.get(class).map_or(0, |c| c.size as u64)
                },
                code: Some(code),
                error: if class == root { unit.error.clone() } else { None },
            };
            (records, vec![block])
        }
        Target::Methods { .. } => {
            let records: Vec<Record> = unit
                .methods
                .iter()
                .filter(|s| target.matches(&s.class, &s.name, &s.descriptor))
                .map(|span| Record::from_span(unit, span, &loaded.universe))
                .collect();
            if records.is_empty() {
                return failed(hidden_method(target));
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
    for record in records.iter().filter(|r| r.error.is_some()) {
        eprintln!(
            "warning: {}: {}",
            record.signature(),
            record.error.as_deref().unwrap_or_default()
        );
    }
    let status = i32::from(all_failed);
    if args.json {
        let error = all_failed.then(|| format!("could not decompile {}", records[0].name()));
        let head = document(args, records.len(), None, error.as_deref(), runtime);
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
            for (i, record) in records.iter().enumerate() {
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
    let select: Option<BTreeSet<String>> = if targets.is_empty() && !args.triage.narrows() {
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
        let mut roots: BTreeSet<String> = picked.iter().map(|c| universe.root_of(c.class)).collect();
        for target in &targets {
            if let Target::Class(class) = target {
                roots.insert(universe.root_of(class));
            }
        }
        Some(roots)
    };
    execute(args, &loaded, select, &|outcome, runtime| {
        let status = finish_decompile_all(args, &loaded, &targets, by_method, &selected, total, &outcome, runtime);
        run::abandon(outcome);
        status
    })
}

#[allow(clippy::too_many_arguments)]
fn finish_decompile_all(
    args: &Args,
    loaded: &Loaded,
    targets: &[Target],
    by_method: bool,
    selected: &dyn Fn(&str, &str, &str) -> bool,
    total: usize,
    outcome: &Outcome,
    runtime: Option<&Arc<SharedRuntime>>,
) -> i32 {
    let universe = &loaded.universe;
    let mut records: Vec<(Record, usize)> = Vec::new();
    for result in &outcome.results {
        let Some(unit) = &result.unit else { continue };
        for span in &unit.methods {
            if !selected(&span.class, &span.name, &span.descriptor) {
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
            outcome
                .results
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
    let sizes = unit_sizes(universe);
    let unit_size = |root: &str| sizes.get(root).copied().unwrap_or(0);
    let classes: Vec<ClassBlock> = class_roots
        .iter()
        .map(|class| {
            let root = universe.root_of(class);
            match unit_of(outcome, &root) {
                Ok(unit) => match class_slice(unit, class) {
                    Some((code, line)) => ClassBlock {
                        name: java_name(class),
                        path: Some(unit.path.clone()),
                        line: (*class != root).then_some(line),
                        size: if *class == root {
                            unit_size(&root)
                        } else {
                            universe.get(class).map_or(0, |c| c.size as u64)
                        },
                        code: Some(code),
                        error: if *class == root { unit.error.clone() } else { None },
                    },
                    None => ClassBlock {
                        name: java_name(class),
                        path: None,
                        line: None,
                        size: 0,
                        code: None,
                        error: Some(hidden_method(&Target::Class(class.clone()))),
                    },
                },
                Err(why) => ClassBlock {
                    name: java_name(class),
                    path: None,
                    line: None,
                    size: unit_size(&root),
                    code: None,
                    error: Some(why),
                },
            }
        })
        .collect();

    let produced = outcome.results.iter().any(|r| r.unit.is_some());
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
    if args.json {
        let total = by_method.then_some(total);
        let head = document(args, records.len(), total, error.as_deref(), runtime);
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
        for record in records.iter().filter(|r| !in_class_block(r)) {
            let _ = writeln!(
                text,
                "// Method: {}  ({}:{})",
                record.signature(),
                record.path,
                record.line
            );
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

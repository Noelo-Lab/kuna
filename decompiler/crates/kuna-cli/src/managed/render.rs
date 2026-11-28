//! What `decompile`, `decompile-all` and `functions` print for a managed input.
//!
//! Text is the engine's: for the JVM, Java, or Kotlin for a class the Kotlin
//! plugin claims. A type is its unit exactly as the engine writes it, a member
//! its slice of that unit (by the engine's member spans) with the common
//! indentation removed; members that share a span, such as a Kotlin
//! property's getter and setter, print it once. The JSON documents keep the
//! native shapes (`decompile-all --json`, `functions --json`, see
//! `docs/cli.md`) and add what a member needs to be named: `class`, `method`,
//! `descriptor`, `access`, `path`, `language` and `line`, and what
//! [`failures`] reports: `nested_errors` and `warnings` per member, `status`
//! per type. Addresses are `null`, `size` is the bytecode length, and
//! `line_mappings` map lines of `code` to bytecode offsets.

use std::collections::BTreeSet;
use std::fmt::Write as _;

use kuna_managed::{Failure, Inventory, MemberSpan, Runtime, Target, Unit};

use super::args::{Args, SortKey, Triage};
use super::failures::{self, Status, Units};
use super::run::{self, Outcome};
use crate::jsonfmt::{self, Json};

/// One member as a record.
pub(crate) struct Record {
    pub(crate) class: String,
    /// `class` as printed.
    owner: String,
    pub(crate) method: String,
    pub(crate) descriptor: String,
    access: Vec<&'static str>,
    pub(crate) size: u64,
    pub(crate) code: String,
    /// The member's own failure, else the bodies written inside it that failed.
    pub(crate) error: Option<String>,
    pub(crate) own_error: Option<String>,
    pub(crate) nested: Vec<Failure>,
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
        format!("{}.{}", self.owner, self.method)
    }

    pub(crate) fn signature(&self) -> String {
        format!("{}{}", self.name(), self.descriptor)
    }

    fn from_span(unit: &Unit, span: &MemberSpan, inventory: &Inventory) -> Record {
        let raw = text(unit).get(span.start..span.end).unwrap_or("");
        let lines = span
            .lines
            .iter()
            .filter(|(line, _)| (span.start_line..=span.end_line).contains(line))
            .map(|(line, offsets)| (line - span.start_line + 1, offsets.clone()))
            .collect();
        let size = inventory.member(&span.owner, &span.name, &span.signature).map_or(0, |m| m.size);
        Record {
            class: span.owner.clone(),
            owner: (inventory.display)(&span.owner),
            method: span.name.clone(),
            descriptor: span.signature.clone(),
            access: span.access.clone(),
            size,
            code: dedent(raw),
            error: failures::member_error(span.error.as_deref(), &span.nested),
            own_error: span.error.clone(),
            nested: span.nested.clone(),
            warnings: span.warnings.clone(),
            path: unit.path.clone(),
            language: unit.language,
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
            ("class".into(), Json::Str(self.owner.clone())),
            ("method".into(), Json::Str(self.method.clone())),
            ("descriptor".into(), Json::Str(self.descriptor.clone())),
            ("access".into(), strings(&self.access)),
            ("path".into(), Json::Str(self.path.clone())),
            ("language".into(), Json::Str(self.language.into())),
            ("line".into(), number(self.line)),
            ("nested_errors".into(), failures::nested_json(&self.nested)),
            ("warnings".into(), strings(&self.warnings)),
        ]);
        Json::Object(record)
    }
}

/// One written type: a unit, or a nested type's part of one.
struct ClassBlock {
    name: String,
    path: Option<String>,
    language: Option<&'static str>,
    line: Option<u32>,
    size: u64,
    code: Option<String>,
    status: Status,
    /// Why the engine could not decompile the type (`Status::Failed`).
    error: Option<String>,
    /// Why a hidden type has no text.
    note: Option<String>,
    /// Failed bodies in the type's text outside its members.
    nested: Vec<Failure>,
}

impl ClassBlock {
    /// `class`, written in `unit` (`root_size`: the unit's definition bytes).
    fn written(args: &Args, unit: &Unit, class: &str, inventory: &Inventory, root_size: u64) -> ClassBlock {
        let block = |status, code, line, error, note| ClassBlock {
            name: (inventory.display)(class),
            path: (status != Status::Hidden).then(|| unit.path.clone()),
            language: (status != Status::Hidden).then_some(unit.language),
            line,
            size: if class == unit.name {
                root_size
            } else {
                inventory.get(class).map_or(0, |c| c.size)
            },
            code,
            status,
            error,
            note,
            nested: failures::outside_members(unit, class),
        };
        match class_slice(unit, class) {
            Some((code, line)) => {
                let line = (class != unit.name).then_some(line);
                match failures::type_failure(unit, class) {
                    Some(why) => block(Status::Failed, Some(code), line, Some(why), None),
                    None => block(Status::Decompiled, Some(code), line, None, None),
                }
            }
            None => match failures::unit_failure(unit) {
                Some(why) => {
                    let why = format!("{} could not be decompiled: {why}", (inventory.display)(&unit.name));
                    block(Status::Failed, None, None, Some(why), None)
                }
                None => {
                    let why = args.engine.hidden(&Target::Type(class.to_string()), unit.language);
                    block(Status::Hidden, None, None, None, Some(why))
                }
            },
        }
    }

    /// `class`, whose unit was not delivered, for `why`.
    fn missing(class: &str, inventory: &Inventory, size: u64, why: String) -> ClassBlock {
        ClassBlock {
            name: (inventory.display)(class),
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
            ("nested_errors".into(), failures::nested_json(&self.nested)),
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

fn strings<S: AsRef<str>>(items: &[S]) -> Json {
    Json::Array(items.iter().map(|s| Json::Str(s.as_ref().to_string())).collect())
}

/// A delivered unit's text (empty when it has none).
fn text(unit: &Unit) -> &str {
    unit.text.as_deref().unwrap_or_default()
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

fn runtime_json(runtime: Option<&Runtime>) -> Json {
    match runtime {
        Some(rt) => Json::Object(vec![
            ("path".into(), Json::Str(rt.path.clone())),
            ("version".into(), optional(rt.version.as_deref())),
        ]),
        None => Json::Null,
    }
}

/// The language of the units a document holds: one name, `mixed` for
/// several, `null` for none.
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
    runtime: Option<&Runtime>,
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
        ("engine".to_string(), Json::Str(args.engine.name().into())),
        ("language".to_string(), language),
        ("omitted".to_string(), strings(&omitted)),
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
fn class_slice(unit: &Unit, class: &str) -> Option<(String, u32)> {
    if unit.name == class {
        return Some((text(unit).to_string(), 1));
    }
    let span = unit.types.iter().find(|c| c.name == class)?;
    let part = text(unit).get(span.start..span.end)?;
    Some((dedent(part), span.start_line))
}

/// Whether `span` lies inside the part of `unit` that `class` covers.
fn within_class(unit: &Unit, class: &str, span: &MemberSpan) -> bool {
    if unit.name == class {
        return true;
    }
    unit.types
        .iter()
        .find(|c| c.name == class)
        .is_some_and(|c| c.start <= span.start && span.end <= c.end)
}

// --- decompile ---------------------------------------------------------------

pub(crate) fn decompile(args: &Args) -> i32 {
    let mut program = match run::open(args) {
        Ok(program) => program,
        Err(status) => return status,
    };
    let own = program.inventory().own.clone();
    let target = match args.target.as_deref() {
        Some(text) => program.resolve(text),
        None => own.map(Target::Type).ok_or_else(|| "decompile requires a class or method".to_string()),
    };
    let target = match target {
        Ok(target) => target,
        Err(e) => {
            eprintln!("error: {e}");
            return 1;
        }
    };
    let runtime = match run::prepare(args, &mut *program) {
        Ok(runtime) => runtime,
        Err(status) => return status,
    };
    let inventory = program.inventory();
    let select: BTreeSet<String> = inventory.chain(target.owner()).into_iter().collect();
    run::drive(&*program, args, Some(select), &|outcome| {
        finish_decompile(args, inventory, &target, &outcome, runtime.as_ref())
    })
}

fn finish_decompile(
    args: &Args,
    inventory: &Inventory,
    target: &Target,
    outcome: &Outcome,
    runtime: Option<&Runtime>,
) -> i32 {
    let display = inventory.display;
    let failed = |name: &str, why: String, unit: Option<&Unit>, block: Option<ClassBlock>| {
        let message = format!("could not decompile {}: {why}", display(name));
        if args.json {
            let language = unit.map_or(Json::Null, |u| Json::Str(u.language.into()));
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
    let unit = match units.find(inventory, target.owner()) {
        Ok(unit) => unit,
        Err(why) => return failed(&inventory.root_of(target.owner()), why, None, None),
    };
    let root = unit.name.as_str();
    if unit.text.is_none() {
        let why = unit.error.clone().unwrap_or_else(|| "no output".into());
        return failed(root, why, None, None);
    }
    let unit_failed = || failures::unit_failure(unit);
    let hidden = || args.engine.hidden(target, unit.language);
    let (records, classes): (Vec<Record>, Vec<ClassBlock>) = match target {
        Target::Type(class) => {
            let root_size = units.sizes(inventory).get(root).copied().unwrap_or(0);
            let block = ClassBlock::written(args, unit, class, inventory, root_size);
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
                .members
                .iter()
                .filter(|span| within_class(unit, class, span))
                .map(|span| Record::from_span(unit, span, inventory))
                .collect();
            (records, vec![block])
        }
        Target::Members { .. } => {
            let records: Vec<Record> = unit
                .members
                .iter()
                .filter(|s| target.matches(&s.owner, &s.name, &s.signature))
                .map(|span| Record::from_span(unit, span, inventory))
                .collect();
            if records.is_empty() {
                let why = unit_failed().unwrap_or_else(hidden);
                return failed(root, why, Some(unit), None);
            }
            (records, Vec::new())
        }
    };
    if records.len() > 1 && matches!(target, Target::Members { signature: None, .. }) {
        eprintln!(
            "note: {} has {} overloads, printed in source order; add a descriptor such as {:?} to pick one",
            records[0].name(),
            records.len(),
            records[0].signature()
        );
    }
    let all_failed = matches!(target, Target::Members { .. }) && records.iter().all(|r| r.error.is_some());
    let outside = classes.first().map(|b| (display(target.owner()), b.nested.as_slice()));
    failures::warn(&records, outside);
    let status = i32::from(all_failed);
    if args.json {
        let error = all_failed.then(|| format!("could not decompile {}", records[0].name()));
        let language = language_of(&records, &classes);
        let head = document(args, language, records.len(), None, error.as_deref(), runtime);
        return emit_document(args, head, &records, &classes, status);
    }
    let text = match target {
        Target::Type(_) => {
            let mut code = classes[0].code.clone().unwrap_or_default();
            if !code.ends_with('\n') {
                code.push('\n');
            }
            code
        }
        Target::Members { .. } => {
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

/// One inventory member, as triage sees it.
struct Candidate<'a> {
    class: &'a str,
    name: &'a str,
    descriptor: &'a str,
    size: u64,
    order: usize,
    signature: String,
}

fn candidates(inventory: &Inventory) -> Vec<Candidate<'_>> {
    let mut out = Vec::new();
    for class in &inventory.types {
        let owner = (inventory.display)(&class.name);
        for member in &class.members {
            out.push(Candidate {
                class: &class.name,
                name: &member.name,
                descriptor: &member.signature,
                size: member.size,
                order: out.len(),
                signature: format!("{owner}.{}{}", member.name, member.signature),
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

/// Whether `class` is `ancestor` or declared inside it.
fn inside(inventory: &Inventory, class: &str, ancestor: &str) -> bool {
    let mut current = class.to_string();
    for _ in 0..256 {
        if current == ancestor {
            return true;
        }
        match inventory.get(&current).and_then(|c| c.outer.clone()) {
            Some(outer) if outer != current => current = outer,
            _ => return false,
        }
    }
    false
}

pub(crate) fn decompile_all(args: &Args) -> i32 {
    let mut program = match run::open(args) {
        Ok(program) => program,
        Err(status) => return status,
    };
    let mut targets = Vec::new();
    for text in args.functions.iter().flatten() {
        match program.resolve(text) {
            Ok(target) => targets.push(target),
            Err(e) => {
                eprintln!("error: {e}");
                return 1;
            }
        }
    }
    let runtime = match run::prepare(args, &mut *program) {
        Ok(runtime) => runtime,
        Err(status) => return status,
    };
    let inventory = program.inventory();
    let by_method = args.triage.narrows() || targets.iter().any(|t| matches!(t, Target::Members { .. }));
    let selected = |class: &str, name: &str, descriptor: &str| {
        targets.is_empty()
            || targets.iter().any(|t| match t {
                Target::Type(c) => inside(inventory, class, c),
                Target::Members { .. } => t.matches(class, name, descriptor),
            })
    };
    let total = candidates(inventory).len();
    let wanted: Option<BTreeSet<String>> = if targets.is_empty() && !args.triage.narrows() {
        None
    } else {
        let mut picked: Vec<Candidate<'_>> = candidates(inventory)
            .into_iter()
            .filter(|c| selected(c.class, c.name, c.descriptor))
            .filter(|c| passes(&args.triage, &c.signature, c.size))
            .collect();
        if args.triage.limit.is_some() {
            picked.retain(|c| inventory.member(c.class, c.name, c.descriptor).is_some_and(|m| !m.synthetic));
        }
        order_and_limit(&mut picked, &args.triage, |c| (c.size, c.signature.clone(), c.order));
        let mut classes: BTreeSet<String> = picked.iter().map(|c| c.class.to_string()).collect();
        for target in &targets {
            if let Target::Type(class) = target {
                classes.insert(class.clone());
            }
        }
        Some(classes)
    };
    let roots = wanted
        .as_ref()
        .map(|classes| classes.iter().flat_map(|c| inventory.chain(c)).collect::<BTreeSet<String>>());
    let shape = Shape {
        targets: &targets,
        wanted: wanted.as_ref(),
        by_method,
        selected: &selected,
        total,
    };
    run::drive(&*program, args, roots, &|outcome| {
        let status = finish_decompile_all(args, inventory, &shape, &outcome, runtime.as_ref());
        run::abandon(outcome);
        status
    })
}

/// What `decompile-all` was asked for.
struct Shape<'a> {
    targets: &'a [Target],
    /// The types selected (`--functions`, triage); `None`: every type.
    wanted: Option<&'a BTreeSet<String>>,
    by_method: bool,
    selected: &'a (dyn Fn(&str, &str, &str) -> bool + Sync),
    total: usize,
}

fn finish_decompile_all(
    args: &Args,
    inventory: &Inventory,
    shape: &Shape<'_>,
    outcome: &Outcome,
    runtime: Option<&Runtime>,
) -> i32 {
    let display = inventory.display;
    let (targets, by_method) = (shape.targets, shape.by_method);
    let units = Units::new(outcome);
    let relevant = units.holding(inventory, shape.wanted);
    let mut records: Vec<(Record, usize)> = Vec::new();
    for unit in relevant.iter().filter(|u| u.text.is_some()) {
        for span in &unit.members {
            if !(shape.selected)(&span.owner, &span.name, &span.signature) {
                continue;
            }
            let record = Record::from_span(unit, span, inventory);
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
                .map(|r| r.name.clone())
                .chain(outcome.missing.iter().map(|(n, _)| n.clone()))
                .collect()
        }
    } else {
        targets
            .iter()
            .filter_map(|t| match t {
                Target::Type(class) => Some(class.clone()),
                Target::Members { .. } => None,
            })
            .collect()
    };
    let sizes = units.sizes(inventory);
    let unit_size = |root: &str| sizes.get(root).copied().unwrap_or(0);
    let classes: Vec<ClassBlock> = class_roots
        .iter()
        .map(|class| match units.find(inventory, class) {
            Ok(unit) if unit.text.is_some() => {
                ClassBlock::written(args, unit, class, inventory, unit_size(&unit.name))
            }
            Ok(unit) => ClassBlock::missing(
                class,
                inventory,
                unit_size(&unit.name),
                unit.error.clone().unwrap_or_else(|| "no output".into()),
            ),
            Err(why) => ClassBlock::missing(class, inventory, unit_size(&inventory.root_of(class)), why),
        })
        .collect();

    let produced = relevant.iter().any(|r| failures::unit_failure(r).is_none());
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
        eprintln!("warning: could not decompile {}: {why}", display(name));
    }
    for unit in &relevant {
        if let Some(why) = failures::unit_failure(unit) {
            eprintln!("warning: could not decompile {}: {why}", display(&unit.name));
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
                .any(|t| matches!(t, Target::Type(c) if inside(inventory, &record.class, c)))
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
    let program = match run::open(args) {
        Ok(program) => program,
        Err(status) => return status,
    };
    let inventory = program.inventory();
    let display = inventory.display;
    let all = candidates(inventory);
    let total = all.len();
    let mut listed: Vec<Candidate<'_>> = all
        .into_iter()
        .filter(|c| passes(&args.triage, &c.signature, c.size))
        .collect();
    order_and_limit(&mut listed, &args.triage, |c| (c.size, c.signature.clone(), c.order));
    if !args.json {
        let mut text = String::new();
        for c in &listed {
            let _ = writeln!(text, "{}", c.signature);
        }
        return crate::output::emit_with_status(&text, 0);
    }
    let owners: BTreeSet<&str> = listed.iter().map(|c| c.class).collect();
    let functions: Vec<Json> = listed
        .iter()
        .map(|c| {
            let access = inventory.member(c.class, c.name, c.descriptor).map_or(&[][..], |m| &m.access);
            Json::Object(vec![
                ("name".into(), Json::Str(format!("{}.{}", display(c.class), c.name))),
                ("address".into(), Json::Null),
                ("address_hex".into(), Json::Null),
                ("aliases".into(), Json::Array(Vec::new())),
                ("object_location".into(), Json::Null),
                ("size".into(), number(c.size)),
                ("class".into(), Json::Str(display(c.class))),
                ("method".into(), Json::Str(c.name.to_string())),
                ("descriptor".into(), Json::Str(c.descriptor.to_string())),
                ("access".into(), strings(access)),
            ])
        })
        .collect();
    let classes: Vec<Json> = inventory
        .types
        .iter()
        .filter(|c| !args.triage.narrows() || owners.contains(c.name.as_str()))
        .map(|c| {
            Json::Object(vec![
                ("name".into(), Json::Str(display(&c.name))),
                ("outer".into(), c.outer.as_deref().map_or(Json::Null, |o| Json::Str(display(o)))),
                ("source_file".into(), optional(c.source_file.as_deref())),
                ("size".into(), number(c.size)),
                ("access".into(), strings(&c.access)),
            ])
        })
        .collect();
    let doc = Json::Object(vec![
        ("binary".into(), Json::Str(args.input.clone())),
        ("engine".into(), Json::Str(args.engine.name().into())),
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

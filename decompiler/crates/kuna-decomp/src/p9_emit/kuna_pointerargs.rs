//! Reconcile object-pointer arguments against accepted final declarations.

use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;
use std::sync::Arc;

use crate::context::{HighVariableId, OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use crate::kuna_lang::OutLang;
use kuna_base::address::Address;
use kuna_num::opcodes::OpCode;

pub type Entry = (String, u64);
pub type Storage = (Entry, i32);

pub fn entry(addr: &Address) -> Option<Entry> {
    Some((addr.get_space()?.get_name().to_string(), addr.get_offset()))
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Parameter {
    pub storage: Storage,
    pub spelling: String,
    pub void_pointer: bool,
    pub dependencies: Vec<String>,
    pub forwards: Vec<String>,
}

/// One-based line and zero-based UTF-8 byte column in the emitted document.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Call {
    pub callee: Entry,
    pub index: usize,
    pub storage: Storage,
    pub actual: String,
    pub position: Position,
    pub end: Position,
    pub void_pointer: bool,
    pub text: usize,
    pub text_start: usize,
    pub text_end: usize,
}

/// Facts from the same successful render as the function's code. Nested sites
/// share an outer argument's text rather than copying its expression again.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Record {
    pub entry: Entry,
    pub parameters: Vec<Option<Parameter>>,
    pub calls: Vec<Call>,
    pub language: OutLang,
    pub struct_defs: bool,
    pub texts: Vec<Arc<String>>,
    pub preamble_end: Position,
}

impl Record {
    pub(crate) fn new(fd: &Funcdata, language: OutLang, struct_defs: bool) -> Option<Self> {
        let proto = fd.get_func_proto();
        let slots = if proto.has_store() { proto.num_params().max(0) as usize } else { 0 };
        Some(Self {
            entry: entry(fd.get_address())?,
            parameters: vec![None; slots],
            calls: Vec::new(),
            language,
            struct_defs,
            texts: Vec::new(),
            preamble_end: Position::default(),
        })
    }

    /// Capture each outer argument once before trimming the printer output.
    /// Invalid or ambiguous spans deliberately retain no usable pool reference.
    pub(crate) fn capture_texts(&mut self, untrimmed: &str) {
        self.texts.clear();
        if self.calls.is_empty() {
            return;
        }
        let starts = line_starts(untrimmed);
        let mut spans = Vec::with_capacity(self.calls.len());
        for (index, call) in self.calls.iter_mut().enumerate() {
            call.text = usize::MAX;
            call.text_start = 0;
            call.text_end = 0;
            if let (Some(start), Some(end)) = (
                offset(untrimmed, &starts, call.position),
                offset(untrimmed, &starts, call.end),
            ) {
                if start < end { spans.push((start, end, index)); }
            }
        }
        if spans.windows(2).any(|pair| {
            (pair[0].0, std::cmp::Reverse(pair[0].1))
                > (pair[1].0, std::cmp::Reverse(pair[1].1))
        }) {
            spans.sort_unstable_by_key(|&(start, end, _)| (start, std::cmp::Reverse(end)));
        }
        let rejected = ambiguous_spans(&spans);
        let mut outer = None;
        for (i, (start, end, index)) in spans.into_iter().enumerate() {
            if rejected[i] { continue; }
            let (base, _, text) = match outer {
                Some((base, limit, text)) if start >= base && end <= limit => (base, limit, text),
                _ => {
                    let text = self.texts.len();
                    self.texts.push(Arc::new(untrimmed[start..end].to_string()));
                    outer = Some((start, end, text));
                    (start, end, text)
                }
            };
            let call = &mut self.calls[index];
            call.text = text;
            call.text_start = start - base;
            call.text_end = end - base;
        }
    }

    pub(crate) fn trim_lines(&mut self, untrimmed: &str) {
        let skip = untrimmed.len() - untrimmed.trim_start_matches('\n').len();
        if self.preamble_end.line > 0 && self.preamble_end.line <= skip
            && self.preamble_end.column == 0
        {
            self.preamble_end = Position { line: 1, column: 0 };
        } else {
            self.preamble_end.line = self.preamble_end.line.saturating_sub(skip);
        }
        for call in &mut self.calls {
            call.position.line = call.position.line.saturating_sub(skip);
            call.end.line = call.end.line.saturating_sub(skip);
        }
    }

    /// Rebase same-line identifier edits from the global structure renamer.
    /// Only actual edited identifiers form the metadata name mapping. Pooled
    /// expressions use the original document edits, not lexical substitution.
    pub fn renamed(&mut self, original: &str, edits: &[(usize, usize, String)]) {
        if edits.is_empty() {
            return;
        }
        let mut edits: Vec<_> = edits.iter().collect();
        edits.sort_unstable_by_key(|e| e.0);
        let mut previous = 0;
        let mut names: BTreeMap<&str, &str> = BTreeMap::new();
        let mut invalid = false;
        for &&(start, end, ref to) in &edits {
            let Some(from) = original.get(start..end) else {
                invalid = true;
                break;
            };
            if start < previous || !identifier(from) || !identifier(to)
                || original[..start].chars().next_back().is_some_and(identifier_char)
                || original[end..].chars().next().is_some_and(identifier_char)
                || names.get(from).is_some_and(|&old| old != to.as_str())
            {
                invalid = true;
                break;
            }
            previous = end;
            names.insert(from, to);
        }
        if invalid {
            // The code renamer owns the text; stale contracts must not remain
            // authoritative when its edits cannot be rebased unambiguously.
            self.parameters.fill(None);
            self.calls.clear();
            self.texts.clear();
            self.preamble_end = Position::default();
            return;
        }
        let starts = line_starts(original);
        let mut prefix = vec![0isize];
        for e in &edits {
            prefix.push(prefix.last().unwrap() + e.2.len() as isize - (e.1 - e.0) as isize);
        }
        let mut bases = vec![None; self.texts.len()];
        for call in &self.calls {
            if let Some(base) = bases.get_mut(call.text).filter(|base| base.is_none()) {
                *base = offset(original, &starts, call.position)
                    .and_then(|at| at.checked_sub(call.text_start));
            }
        }
        let mut pool_edits = vec![None; self.texts.len()];
        for (index, text) in self.texts.iter_mut().enumerate() {
            let Some(base) = bases[index] else { continue };
            let Some(end) = base.checked_add(text.len()) else { continue };
            if original.get(base..end) != Some(text.as_str()) {
                continue;
            }
            let first = edits.partition_point(|e| e.1 <= base);
            let last = edits.partition_point(|e| e.0 < end);
            let local = &edits[first..last];
            if local.is_empty() {
                continue;
            }
            if local.iter().any(|e| e.0 < base || e.1 > end) {
                continue;
            }
            let mut changed = String::with_capacity(
                text.len().saturating_add_signed(prefix[last] - prefix[first]),
            );
            let mut at = 0;
            for e in local {
                changed.push_str(&text[at..e.0 - base]);
                changed.push_str(&e.2);
                at = e.1 - base;
            }
            changed.push_str(&text[at..]);
            pool_edits[index] = Some((base, first, last));
            *text = Arc::new(changed);
        }
        for call in &mut self.calls {
            if let Some(&(base, first, last)) = pool_edits.get(call.text).and_then(Option::as_ref) {
                for boundary in [&mut call.text_start, &mut call.text_end] {
                    let count = edits[first..last].partition_point(|e| e.1 - base <= *boundary);
                    *boundary = boundary.saturating_add_signed(prefix[first + count] - prefix[first]);
                }
            }
            rename_identifiers(&mut call.actual, &names);
        }
        for position in std::iter::once(&mut self.preamble_end).chain(
            self.calls.iter_mut().flat_map(|call| [&mut call.position, &mut call.end]),
        ) {
            if let Some(at) = offset(original, &starts, *position) {
                let line_start = starts[position.line - 1];
                let first = edits.partition_point(|e| e.1 <= line_start);
                let last = edits.partition_point(|e| e.1 <= at);
                position.column = position.column.saturating_add_signed(prefix[last] - prefix[first]);
            }
        }
        for parameter in self.parameters.iter_mut().flatten() {
            rename_identifiers(&mut parameter.spelling, &names);
            for text in parameter.dependencies.iter_mut().chain(parameter.forwards.iter_mut()) {
                rename_identifiers(text, &names);
            }
        }
    }
}

fn identifier(text: &str) -> bool {
    let mut chars = text.chars();
    chars.next().is_some_and(|c| c.is_alphabetic() || c == '_' || c == '$')
        && chars.all(identifier_char)
}

fn identifier_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '$'
}

fn rename_identifiers(text: &mut String, names: &BTreeMap<&str, &str>) {
    let mut out = None;
    let mut copied = 0;
    let mut at = 0;
    while at < text.len() {
        let c = text[at..].chars().next().unwrap();
        let end = if identifier_char(c) {
            text[at..].char_indices().find(|&(_, c)| !identifier_char(c))
                .map_or(text.len(), |(i, _)| at + i)
        } else { at + c.len_utf8() };
        let token = &text[at..end];
        if let Some(to) = names.get(token).copied().filter(|&to| to != token) {
            let out = out.get_or_insert_with(|| String::with_capacity(text.len()));
            out.push_str(&text[copied..at]);
            out.push_str(to);
            copied = end;
        }
        at = end;
    }
    if let Some(mut out) = out {
        out.push_str(&text[copied..]);
        *text = out;
    }
}

/// Only equal, unambiguous declarations in the final result set may decide.
pub type Definitions = BTreeMap<Entry, Option<(OutLang, Vec<Option<Parameter>>)>>;

pub fn definitions<'a>(records: impl IntoIterator<Item = &'a Record>) -> Definitions {
    let mut out = Definitions::new();
    for r in records {
        out.entry(r.entry.clone()).and_modify(|old| {
            if old.as_ref().is_none_or(|(language, parameters)| {
                *language != r.language || parameters != &r.parameters
            }) {
                *old = None;
            }
        }).or_insert_with(|| Some((r.language, r.parameters.clone())));
    }
    out
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Insertion {
    pub position: Position,
    pub end: Position,
    pub expression: Arc<String>,
    pub text_start: usize,
    pub text_end: usize,
    pub spelling: String,
    pub language: OutLang,
    pub dependencies: Vec<String>,
    pub forwards: Vec<String>,
}

pub fn insertions(record: &Record, definitions: &Definitions) -> Vec<Insertion> {
    let mut out: Vec<_> = record.calls.iter().filter_map(|call| {
        let (language, parameters) = definitions.get(&call.callee)?.as_ref()?;
        let target = parameters.get(call.index)?.as_ref()?;
        if *language != record.language || target.storage != call.storage
            || target.spelling == call.actual
            || (*language == OutLang::C && (target.void_pointer || call.void_pointer))
        {
            return None;
        }
        Some(Insertion {
            position: call.position,
            end: call.end,
            expression: Arc::clone(record.texts.get(call.text)?),
            text_start: call.text_start,
            text_end: call.text_end,
            spelling: target.spelling.clone(),
            language: *language,
            dependencies: target.dependencies.clone(),
            forwards: target.forwards.clone(),
        })
    }).collect();
    out.sort_by_key(|edit| (edit.position, std::cmp::Reverse(edit.end)));
    out
}

/// The cast text, declined sites, and only the sites actually applied.
#[derive(Debug, PartialEq, Eq)]
pub struct Applied {
    pub code: String,
    pub skipped: Vec<(Insertion, &'static str)>,
    pub inserted: Vec<Insertion>,
}

fn line_starts(code: &str) -> Vec<usize> {
    std::iter::once(0).chain(code.match_indices('\n').map(|(i, _)| i + 1)).collect()
}

fn offset(code: &str, starts: &[usize], position: Position) -> Option<usize> {
    let line = position.line.checked_sub(1)?;
    let start = *starts.get(line)?;
    let end = starts.get(line + 1).map_or(code.len(), |end| end - 1);
    let at = start.checked_add(position.column)?;
    (at <= end && code.is_char_boundary(at)).then_some(at)
}

fn site(code: &str, starts: &[usize], edit: &Insertion) -> Result<(usize, usize), &'static str> {
    if edit.spelling.is_empty() || edit.spelling.contains(['\n', '\r']) {
        return Err("invalid pointer argument type spelling");
    }
    let expected = edit.expression.get(edit.text_start..edit.text_end)
        .filter(|s| !s.is_empty()).ok_or("invalid pointer argument text range")?;
    let start = offset(code, starts, edit.position);
    let end = offset(code, starts, edit.end);
    let (Some(start), Some(end)) = (start, end) else {
        return Err("invalid pointer argument emission position");
    };
    if end.checked_sub(start) != Some(expected.len())
        || (expected.chars().next_back().is_some_and(identifier_char)
            && code[end..].chars().next().is_some_and(identifier_char))
    {
        return Err("pointer argument emission range does not match its expression");
    }
    Ok((start, end))
}

fn ambiguous_spans<T>(sites: &[(usize, usize, T)]) -> Vec<bool> {
    let mut rejected = vec![false; sites.len()];
    let mut stack: Vec<usize> = Vec::new();
    let mut crossing = false;
    for i in 0..sites.len() {
        let (start, end, _) = sites[i];
        while stack.last().is_some_and(|&prior| sites[prior].1 <= start) {
            stack.pop();
        }
        if stack.last().is_some_and(|&prior| sites[prior].1 < end) {
            crossing = true;
            break;
        }
        if i > 0 && (sites[i - 1].0, sites[i - 1].1) == (start, end) {
            rejected[i] = true;
            rejected[i - 1] = true;
        }
        stack.push(i);
    }
    if !crossing {
        return rejected;
    }
    drop(stack);
    rejected.fill(false);
    // Keep rejected intervals active too: a later site can cross an already
    // rejected span while being properly nested inside the span that crossed it.
    let mut active = BTreeSet::new();
    let mut unmarked = BTreeSet::new();
    for i in 0..sites.len() {
        let (start, end, _) = sites[i];
        while active.first().is_some_and(|&end| end <= start) { active.pop_first(); }
        while unmarked.first().is_some_and(|&(end, _)| end <= start) { unmarked.pop_first(); }
        if active.first().is_some_and(|&prior_end| prior_end < end) {
            rejected[i] = true;
        }
        while unmarked.first().is_some_and(|&(prior_end, _)| prior_end < end) {
            let (_, prior) = unmarked.pop_first().unwrap();
            rejected[prior] = true;
        }
        if i > 0 && (sites[i - 1].0, sites[i - 1].1) == (start, end) {
            rejected[i] = true;
            rejected[i - 1] = true;
            unmarked.remove(&(end, i - 1));
        }
        active.insert(end);
        if !rejected[i] { unmarked.insert((end, i)); }
    }
    rejected
}

/// Validate exact whole-argument spans before streaming nested prefix/suffix
/// events. Crossing or identical spans are ambiguous and all participants
/// decline; no textual search or repeated expression replacement is used.
pub fn apply(code: &str, edits: &[Insertion]) -> Applied {
    let starts = line_starts(code);
    let mut skipped = Vec::new();
    let mut sites = Vec::with_capacity(edits.len());
    for edit in edits {
        match site(code, &starts, edit) {
            Ok((start, end)) => sites.push((start, end, edit)),
            Err(reason) => skipped.push((edit.clone(), reason)),
        }
    }
    sites.sort_by_key(|&(start, end, _)| (start, std::cmp::Reverse(end)));
    // A pool maps to a fixed document origin. Compare overlapping captured
    // bytes once, including when many nested sites share a very large pool.
    let mut checked: BTreeMap<(usize, usize), (usize, usize)> = BTreeMap::new();
    sites.retain(|&(start, end, edit)| {
        let key = (Arc::as_ptr(&edit.expression) as usize, start.wrapping_sub(edit.text_start));
        let covered = checked.get(&key).copied();
        let from = covered.filter(|&(lo, hi)| edit.text_start >= lo && edit.text_start <= hi)
            .map_or(edit.text_start, |(_, hi)| hi.max(edit.text_start).min(edit.text_end));
        let valid = code.get(start + (from - edit.text_start)..end)
            == edit.expression.get(from..edit.text_end);
        if valid {
            checked.entry(key).and_modify(|range| {
                if edit.text_start <= range.1 {
                    range.1 = range.1.max(edit.text_end);
                } else { *range = (edit.text_start, edit.text_end); }
            }).or_insert((edit.text_start, edit.text_end));
        } else {
            skipped.push((edit.clone(), "pointer argument text has drifted"));
        }
        valid
    });
    let rejected = ambiguous_spans(&sites);
    let mut accepted = Vec::with_capacity(sites.len());
    for (i, site) in sites.into_iter().enumerate() {
        if rejected[i] {
            skipped.push((site.2.clone(), "ambiguous pointer argument emission spans"));
        } else { accepted.push(site); }
    }
    let extra: usize = accepted.iter().map(|(_, _, e)| e.spelling.len() + 8).sum();
    let mut out = String::with_capacity(code.len() + extra);
    let mut at = 0;
    let mut pending: Vec<(usize, &Insertion)> = Vec::new();
    let mut inserted = Vec::with_capacity(accepted.len());
    for (start, end, edit) in accepted {
        while pending.last().is_some_and(|&(end, _)| end <= start) {
            let (end, closing) = pending.pop().unwrap();
            out.push_str(&code[at..end]);
            close_cast(&mut out, closing);
            at = end;
        }
        out.push_str(&code[at..start]);
        out.push_str("((");
        if edit.language == OutLang::C {
            out.push_str(&edit.spelling);
            out.push_str(")(");
        }
        at = start;
        pending.push((end, edit));
        inserted.push(edit.clone());
    }
    while let Some((end, closing)) = pending.pop() {
        out.push_str(&code[at..end]);
        close_cast(&mut out, closing);
        at = end;
    }
    out.push_str(&code[at..]);
    Applied { code: out, skipped, inserted }
}

fn close_cast(out: &mut String, edit: &Insertion) {
    match edit.language {
        OutLang::C => out.push_str("))"),
        OutLang::Rust => {
            out.push_str(") as ");
            out.push_str(&edit.spelling);
            out.push(')');
        }
    }
}

#[derive(Default)]
pub(crate) struct Declarations(BTreeMap<String, String>);

impl Declarations {
    pub(crate) fn clear(&mut self) { self.0.clear(); }

    pub(crate) fn record(&mut self, name: &str, front: &str, back: &str, array: bool) {
        if back.is_empty() && !array {
            self.0.insert(name.to_string(), front.to_string());
        } else { self.0.remove(name); }
    }

    pub(crate) fn address(&self, name: &str, language: OutLang) -> Option<String> {
        self.0.get(name).map(|base| match language {
            OutLang::C => pointer_to(base),
            OutLang::Rust => format!("*mut {base}"),
        })
    }
}

fn pointer_to(base: &str) -> String {
    format!("{base}{}*", if base.ends_with('*') { "" } else { " " })
}

pub(crate) trait PrintedPointers {
    fn spell(&self, ty: &Rc<Datatype>) -> String;
    fn address(&self, high: HighVariableId) -> Option<(String, String)>;
    fn value_type(&self, vn: VarnodeId, op: OpId) -> Option<Rc<Datatype>>;
}

pub(crate) fn argument(
    p: &dyn PrintedPointers,
    fd: &Funcdata,
    arch: &crate::architecture::Architecture,
    call: OpId,
    slot: i32,
) -> Option<Call> {
    let op = fd.obank().get(call)?;
    if op.code() != OpCode::CPUI_CALL || slot < 1 { return None; }
    let fc = fd.get_call_specs(fd.get_call_specs_index(call)?);
    if fc.format_arity().is_some() || fc.is_dotdotdot() || fc.is_input_locked()
        || fd.get_override().find_proto_override(op.get_addr()).is_some()
    {
        return None;
    }
    let index = (slot - 1) as usize;
    if fc.proto().has_store() && fc.proto().get_param(index as i32)
        .is_some_and(|p| p.is_type_locked() || p.is_this_pointer())
    {
        return None;
    }
    let (storage, size) = fc.final_input_storage().get(index)?;
    let vn = op.get_in(slot)?;
    let value = fd.vbank().get(vn)?;
    let ty = p.value_type(vn, call)?;
    let pointee = ty.get_ptr_to()?;
    if ty.get_metatype() != type_metatype::TYPE_PTR
        || pointee.get_metatype() == type_metatype::TYPE_CODE
        || value.get_size() != arch.types().get_size_of_pointer()
        || ty.get_size() != value.get_size() || *size != value.get_size()
    {
        return None;
    }
    let (actual, void_pointer) = if let Some(actual) = pointer_type(p, fd, vn, 0) {
        (actual, false)
    } else {
        (p.spell(&ty), pointee.get_metatype() == type_metatype::TYPE_VOID)
    };
    Some(Call {
        callee: entry(fc.get_entry_address())?,
        index,
        storage: (entry(storage)?, *size),
        actual,
        position: Position::default(),
        end: Position::default(),
        void_pointer,
        text: 0,
        text_start: 0,
        text_end: 0,
    })
}

fn pointer_type(
    p: &dyn PrintedPointers,
    fd: &Funcdata,
    vn: VarnodeId,
    depth: usize,
) -> Option<String> {
    if depth == 16 { return None; }
    let v = fd.vbank().get(vn)?;
    if !v.is_implied() || v.has_implied_field() { return None; }
    let op = fd.obank().get(v.get_def()?)?;
    match op.code() {
        OpCode::CPUI_COPY => pointer_type(p, fd, op.get_in(0)?, depth + 1),
        OpCode::CPUI_PTRSUB => {
            let base = fd.vbank().get(op.get_in(0)?)?.get_type().get_ptr_to()?;
            if base.get_metatype() != type_metatype::TYPE_SPACEBASE { return None; }
            let (_, ty) = p.address(fd.vbank().get(op.get_in(1)?)?.get_high()?)?;
            Some(ty)
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests;

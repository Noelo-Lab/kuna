//! Match scalar addresses to character-pointer declarations in a completed C batch.

use std::collections::BTreeMap;
use std::rc::Rc;

use crate::context::{HighVariableId, OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
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
}

/// One-based line and UTF-8 byte column. The printer records positions in its
/// own output; [`PrintC::take_pointer_arguments`](crate::printc::PrintC::take_pointer_arguments)
/// rebases them onto the trimmed text, so a taken record always addresses `code`.
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
    pub expression: String,
}

/// Facts from the same successful render as the function's code. A prototype
/// slot without a parameter stays `None`; no inferred declaration crosses a batch.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Record {
    pub entry: Entry,
    pub parameters: Vec<Option<Parameter>>,
    pub calls: Vec<Call>,
}

impl Record {
    pub(crate) fn new(fd: &Funcdata) -> Option<Self> {
        let proto = fd.get_func_proto();
        let slots = if proto.has_store() { proto.num_params().max(0) as usize } else { 0 };
        Some(Self {
            entry: entry(fd.get_address())?,
            parameters: vec![None; slots],
            calls: Vec::new(),
        })
    }

    pub(crate) fn trim_lines(&mut self, untrimmed: &str) {
        let skip = untrimmed.len() - untrimmed.trim_start_matches('\n').len();
        for call in &mut self.calls {
            call.position.line = call.position.line.saturating_sub(skip);
        }
    }

    /// Structure renaming preserves lines, but can move later columns. The
    /// identifier edits of one text never overlap.
    pub fn renamed(&mut self, original: &str, edits: &[(usize, usize, String)]) {
        if self.calls.is_empty() || edits.is_empty() {
            return;
        }
        let mut spans: Vec<_> = edits
            .iter()
            .map(|(start, end, to)| (*start, *end, to.len() as isize - (end - start) as isize))
            .collect();
        spans.sort_unstable_by_key(|&(start, ..)| start);
        let mut shift = vec![0isize];
        for &(.., delta) in &spans {
            shift.push(shift.last().unwrap() + delta);
        }
        let starts: Vec<_> = std::iter::once(0)
            .chain(original.match_indices('\n').map(|(i, _)| i + 1))
            .collect();
        for call in &mut self.calls {
            let Some(&line_start) = call.position.line.checked_sub(1).and_then(|l| starts.get(l))
            else {
                continue;
            };
            let at = line_start + call.position.column;
            let first = spans.partition_point(|&(start, ..)| start < line_start);
            let last = spans.partition_point(|&(_, end, _)| end <= at).max(first);
            call.position.column =
                call.position.column.saturating_add_signed(shift[last] - shift[first]);
        }
    }
}

/// Only equal, unambiguous declarations in the final result set may decide.
pub type Definitions = BTreeMap<Entry, Option<Vec<Option<Parameter>>>>;

pub fn definitions<'a>(records: impl IntoIterator<Item = &'a Record>) -> Definitions {
    let mut out = Definitions::new();
    for r in records {
        out.entry(r.entry.clone())
            .and_modify(|old| {
                if old.as_ref() != Some(&r.parameters) {
                    *old = None;
                }
            })
            .or_insert_with(|| Some(r.parameters.clone()));
    }
    out
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Insertion {
    pub position: Position,
    pub expression: String,
    pub spelling: String,
}

pub fn insertions(record: &Record, definitions: &Definitions) -> Vec<Insertion> {
    let mut out: Vec<_> = record
        .calls
        .iter()
        .filter_map(|call| {
            let target = definitions
                .get(&call.callee)?
                .as_ref()?
                .get(call.index)?
                .as_ref()?;
            (target.storage == call.storage && target.spelling != call.actual).then(|| Insertion {
                position: call.position,
                expression: call.expression.clone(),
                spelling: target.spelling.clone(),
            })
        })
        .collect();
    out.sort_by_key(|edit| edit.position);
    out
}

/// The cast text and the sites [`apply`] left uncast.
#[derive(Debug, PartialEq, Eq)]
pub struct Applied {
    pub code: String,
    pub skipped: Vec<(Insertion, &'static str)>,
}

/// Validate every emitter-recorded site before editing; no text search or
/// inference from a function's name participates in the conversion. A site
/// that does not validate stays uncast: the rest of the document is unaffected.
/// `edits` must be sorted by position, as [`insertions`] returns them.
pub fn apply(code: &str, edits: &[Insertion]) -> Applied {
    let starts: Vec<_> = std::iter::once(0)
        .chain(code.match_indices('\n').map(|(i, _)| i + 1))
        .collect();
    let mut sites: Vec<(usize, &Insertion)> = Vec::with_capacity(edits.len());
    let mut skipped = Vec::new();
    for edit in edits {
        match site(code, &starts, edit) {
            Err(reason) => skipped.push((edit.clone(), reason)),
            Ok(at) if sites
                .last()
                .is_some_and(|&(previous, prior)| previous + prior.expression.len() > at) =>
            {
                skipped.push((edit.clone(), "pointer argument emission positions overlap"));
            }
            Ok(at) => sites.push((at, edit)),
        }
    }
    let mut out = String::with_capacity(
        code.len() + sites.iter().map(|(_, e)| e.spelling.len() + 2).sum::<usize>(),
    );
    let mut previous = 0;
    for &(at, edit) in &sites {
        out.push_str(&code[previous..at]);
        out.push('(');
        out.push_str(&edit.spelling);
        out.push(')');
        previous = at;
    }
    out.push_str(&code[previous..]);
    Applied { code: out, skipped }
}

fn site(code: &str, starts: &[usize], edit: &Insertion) -> Result<usize, &'static str> {
    if edit.spelling.contains(['\n', '\r']) {
        return Err("pointer argument type contains a line break");
    }
    edit.position
        .line
        .checked_sub(1)
        .and_then(|line| {
            let start = *starts.get(line)?;
            let end = starts.get(line + 1).map_or(code.len(), |end| end - 1);
            let at = start.checked_add(edit.position.column)?;
            (at.checked_add(edit.expression.len())? <= end).then_some(at)
        })
        .filter(|&at| {
            code.get(at..).is_some_and(|s| {
                !edit.expression.is_empty()
                    && s.starts_with(&edit.expression)
                    && s[edit.expression.len()..]
                        .chars()
                        .next()
                        .is_none_or(|c| !c.is_alphanumeric() && c != '_' && c != '$')
            })
        })
        .ok_or("pointer argument emission position does not match its expression")
}

#[derive(Default)]
pub(crate) struct Declarations(BTreeMap<String, String>);

impl Declarations {
    pub(crate) fn clear(&mut self) {
        self.0.clear();
    }

    pub(crate) fn record(&mut self, name: &str, front: &str, back: &str, array: bool) {
        if back.is_empty() && !array {
            self.0.insert(name.to_string(), front.to_string());
        } else {
            self.0.remove(name);
        }
    }

    pub(crate) fn address(&self, name: &str) -> Option<String> {
        self.0.get(name).map(|base| pointer_to(base))
    }
}

fn pointer_to(base: &str) -> String {
    format!("{base}{}*", if base.ends_with('*') { "" } else { " " })
}

pub(crate) trait PrintedPointers {
    fn spell(&self, ty: &Rc<Datatype>) -> String;
    fn address(&self, high: HighVariableId) -> Option<(String, String)>;
}

pub(crate) struct Candidate {
    pub address_op: OpId,
    pub call: Call,
}

pub(crate) fn argument(
    p: &dyn PrintedPointers,
    fd: &Funcdata,
    arch: &crate::architecture::Architecture,
    call: OpId,
    slot: i32,
) -> Option<Candidate> {
    let op = fd.obank().get(call)?;
    if op.code() != OpCode::CPUI_CALL || slot < 1 {
        return None;
    }
    let fc = fd.get_call_specs(fd.get_call_specs_index(call)?);
    if fc.format_arity().is_some()
        || fc.is_dotdotdot()
        || fd
            .get_override()
            .find_proto_override(op.get_addr())
            .is_some()
    {
        return None;
    }
    let index = (slot - 1) as usize;
    let (storage, size) = fc.final_input_storage().get(index)?;
    let (address_op, name, actual) = pointer_type(p, fd, op.get_in(slot)?, 0)?;
    if byte_spellings(p, arch).any(|b| pointer_to(&b) == actual) {
        return None;
    }
    Some(Candidate {
        address_op,
        call: Call {
            callee: entry(fc.get_entry_address())?,
            index,
            storage: (entry(storage)?, *size),
            actual,
            position: Position::default(),
            expression: format!("&{name}"),
        },
    })
}

fn pointer_type(
    p: &dyn PrintedPointers,
    fd: &Funcdata,
    vn: VarnodeId,
    depth: usize,
) -> Option<(OpId, String, String)> {
    if depth == 16 {
        return None;
    }
    let v = fd.vbank().get(vn)?;
    if !v.is_implied() || v.has_implied_field() {
        return None;
    }
    let id = v.get_def()?;
    let op = fd.obank().get(id)?;
    match op.code() {
        OpCode::CPUI_COPY => pointer_type(p, fd, op.get_in(0)?, depth + 1),
        OpCode::CPUI_PTRSUB => {
            let base = fd.vbank().get(op.get_in(0)?)?.get_type().get_ptr_to()?;
            if base.get_metatype() != type_metatype::TYPE_SPACEBASE {
                return None;
            }
            let (name, ty) = p.address(fd.vbank().get(op.get_in(1)?)?.get_high()?)?;
            Some((id, name, ty))
        }
        _ => None,
    }
}

fn byte_spellings<'a>(
    p: &'a dyn PrintedPointers,
    arch: &'a crate::architecture::Architecture,
) -> impl Iterator<Item = String> + 'a {
    let types = arch.types();
    let factory = [
        types.get_base(1, type_metatype::TYPE_INT),
        types.get_base_no_char(1, type_metatype::TYPE_INT),
        types.get_base(1, type_metatype::TYPE_UINT),
        types.get_base(1, type_metatype::TYPE_UNKNOWN),
        types.get_type_char(1),
    ];
    ["char", "signed char", "unsigned char", "undefined1"]
        .into_iter()
        .map(String::from)
        .chain(factory.into_iter().flatten().map(move |ty| p.spell(&ty)))
}

pub(crate) fn byte_pointer(ty: &Rc<Datatype>) -> bool {
    ty.get_metatype() == type_metatype::TYPE_PTR
        && ty.get_ptr_to().is_some_and(|p| {
            p.get_size() == 1
                && matches!(
                    p.get_metatype(),
                    type_metatype::TYPE_INT
                        | type_metatype::TYPE_UINT
                        | type_metatype::TYPE_UNKNOWN
                )
        })
}

#[cfg(test)]
mod tests;

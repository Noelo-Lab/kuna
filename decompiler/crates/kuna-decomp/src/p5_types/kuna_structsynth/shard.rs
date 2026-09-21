//! (kuna `structsynth`) The ledger as data: what a whole-program run split
//! across worker processes (`--jobs N`) needs to name every synthesized
//! structure exactly as one process would.
//!
//! One process decides `struct_N` names in decompile order, each lookup reading
//! the structures minted before it. A worker process sees only its own share of
//! the functions, so the names it would mint are its own. This module lets a
//! parent that decompiles nothing take the serial decisions instead:
//!
//! * a worker RECORDS every ledger lookup a function makes ([`SynthRequest`]):
//!   the measured layout, its members, the bytes it accessed without claiming
//!   them, and a recipe for each field type -- and what its own ledger answered
//!   ([`Answer`]);
//! * the parent REPLAYS the lookups in target order through the ledger's own
//!   decision ([`Replay`]), which yields every answer, every minted name and
//!   the superseded set;
//! * a function whose own answers named structures with exactly the members of
//!   the serial ones is RENAMED ([`renaming`], [`rename_identifiers`]): its
//!   text is the serial text with other numbers;
//! * any other function is decompiled again by a worker that INSTALLS the
//!   replayed structures ([`install_table`]) and answers each lookup with its
//!   replayed name, recording what it asked so the parent can check that the
//!   replay was told the truth.
//!
//! What a lookup asks does not depend on what the ledger holds: `synthesize`
//! collects its evidence before it installs anything, and a field takes the
//! type an access carried. The one path by which an earlier answer reaches a
//! later request is a restarted decompile, whose parameter can keep the type
//! the first attempt locked; that is why the forced run's requests are compared
//! with the recorded ones, and why a field typed by a synthesized structure --
//! a nested record's pointer, or a record's pointer to itself -- is never given
//! a recipe. Nor is a field typed by a named type the worker's load
//! did not create ([`AtLoad`]): a pass that interns a type the first time a
//! function needs it (`pebnames`' `PEB` and `TEB`) leaves it in some workers and
//! not in others, so no other process is sure to rebuild it.

use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::rc::Rc;

use kuna_base::types::int4;

use super::ledger::{self, FieldKey, Layout, Member};
use crate::dtype::{Datatype, DatatypeKind, TypeFactory, TypeFactoryImpl, TypeField};

/// How a field type is rebuilt in another process's factory.
///
/// A named type is found by name and checked by id, metatype and size; a
/// pointer or an array is rebuilt around its rebuilt element. Everything else
/// (a pointer into a named space, a relative pointer, an unnamed aggregate, a
/// synthesized structure) has no recipe, and a request carrying one cannot be
/// replayed in another process.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum TypeRecipe {
    /// A type the factory holds by name.
    Named { name: String, id: u64, meta: i32, size: int4 },
    /// `ptrto *`, of `size` bytes, addressing `wordsize` units.
    Pointer { size: int4, wordsize: u32, to: Box<TypeRecipe> },
    /// `elem[count]`.
    Array { count: int4, elem: Box<TypeRecipe> },
}

/// How deep a recipe follows pointers and arrays; the ledger spells a field no
/// deeper than this either.
const MAX_RECIPE_DEPTH: u32 = 8;

impl TypeRecipe {
    /// The recipe for `ct`, or `None` when another factory cannot rebuild it.
    pub fn of(ct: &Datatype) -> Option<TypeRecipe> {
        Self::of_depth(ct, 0)
    }

    fn of_depth(ct: &Datatype, depth: u32) -> Option<TypeRecipe> {
        if depth > MAX_RECIPE_DEPTH {
            return None;
        }
        match &ct.kind {
            DatatypeKind::Pointer { ptrto, spaceid: None, truncate: None, wordsize }
                if ct.get_name().is_empty() =>
            {
                Some(TypeRecipe::Pointer {
                    size: ct.get_size(),
                    wordsize: *wordsize,
                    to: Box::new(Self::of_depth(ptrto, depth + 1)?),
                })
            }
            DatatypeKind::Array { arrayof, arraysize } if ct.get_name().is_empty() => {
                Some(TypeRecipe::Array {
                    count: *arraysize,
                    elem: Box::new(Self::of_depth(arrayof, depth + 1)?),
                })
            }
            DatatypeKind::Pointer { .. } | DatatypeKind::Array { .. } => None,
            DatatypeKind::PointerRel { .. }
            | DatatypeKind::PartialStruct { .. }
            | DatatypeKind::PartialUnion { .. }
            | DatatypeKind::PartialEnum { .. }
            | DatatypeKind::Spacebase { .. } => None,
            _ if ct.get_name().is_empty() => None,
            // A synthesized structure is named by the process that minted it.
            _ if ledger::minted_number(ct).is_some() => None,
            _ => Some(TypeRecipe::Named {
                name: ct.get_name().to_string(),
                id: ct.get_id(),
                meta: ct.get_metatype().as_i32(),
                size: ct.get_size(),
            }),
        }
    }

    /// Does every named type the recipe reaches belong to `at_load`?
    fn held_at_load(&self, at_load: &AtLoad) -> bool {
        match self {
            TypeRecipe::Named { id, .. } => at_load.ids.contains(id),
            TypeRecipe::Pointer { to, .. } => to.held_at_load(at_load),
            TypeRecipe::Array { elem, .. } => elem.held_at_load(at_load),
        }
    }

    /// Rebuild the type in `types`, or `None` when the factory does not hold
    /// exactly what the recipe names.
    pub fn build(&self, types: &dyn TypeFactory) -> Option<Rc<Datatype>> {
        let ct = match self {
            TypeRecipe::Named { name, .. } => types.find_by_name(name).ok().flatten()?,
            TypeRecipe::Pointer { size, wordsize, to } => {
                types.get_type_pointer(*size, to.build(types)?, *wordsize).ok()?
            }
            TypeRecipe::Array { count, elem } => types.get_type_array(*count, elem.build(types)?).ok()?,
        };
        (TypeRecipe::of(&ct).as_ref() == Some(self)).then_some(ct)
    }
}

/// What a worker's program held when its load finished, which every worker of a
/// run holds alike: the named types, and the `struct_N` names.
#[derive(Clone, Debug, Default)]
pub struct AtLoad {
    ids: HashSet<u64>,
    held: Vec<String>,
}

impl AtLoad {
    /// Take the snapshot, before any decompile.
    pub fn of(types: &TypeFactoryImpl) -> AtLoad {
        let ids = types
            .dependent_order()
            .iter()
            .filter(|t| !t.get_name().is_empty() && t.get_id() != 0)
            .map(|t| t.get_id())
            .collect();
        AtLoad { ids, held: held_names(types) }
    }

    /// The `struct_N` names held at load, which [`forget_minted`] keeps.
    pub fn held(&self) -> &[String] {
        &self.held
    }
}

/// One member of a measured layout, as the minting call received it.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FieldRecipe {
    ident: int4,
    offset: int4,
    name: String,
    ty: Option<TypeRecipe>,
}

/// One ledger lookup, as data: everything the decision reads and everything a
/// mint needs.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SynthRequest {
    size: int4,
    want: Layout,
    own: Vec<Member>,
    unclaimed: Vec<(int4, int4)>,
    fields: Vec<FieldRecipe>,
}

impl SynthRequest {
    /// Record the arguments of one `lookup_or_mint` call. A field type gets a
    /// recipe only when every named type it reaches was held at load, and a
    /// field pointing at its own structure (`selfs`) never gets one.
    fn of(
        fields: &[TypeField],
        size: int4,
        unclaimed: &[(int4, int4)],
        selfs: &[int4],
        at_load: &AtLoad,
    ) -> SynthRequest {
        SynthRequest {
            size,
            want: ledger::layout_of_fields(fields, size, selfs),
            own: fields.iter().map(|f| Member::of_with(f, selfs)).collect(),
            unclaimed: unclaimed.to_vec(),
            fields: fields
                .iter()
                .map(|f| FieldRecipe {
                    ident: f.ident,
                    offset: f.offset,
                    name: f.name.clone(),
                    ty: TypeRecipe::of(&f.field_type)
                        .filter(|r| r.held_at_load(at_load) && !selfs.contains(&f.offset)),
                })
                .collect(),
        }
    }

    /// Can another process rebuild every field of this layout?
    pub fn portable(&self) -> bool {
        self.fields.iter().all(|f| f.ty.is_some())
    }

    /// Does a structure minted for `self` have exactly the members of one
    /// minted for `other`? Only a portable request can tell: a field without a
    /// recipe could be any type.
    pub fn defines_same(&self, other: &SynthRequest) -> bool {
        self.portable() && self.size == other.size && self.fields == other.fields
    }
}

/// What one ledger lookup was answered with: the structure's name and the
/// lookup that minted it (`None` for a name held before the run), or `None`
/// for no answer. A recording worker reports its own ledger's; the parent
/// derives the serial run's from the replay.
pub type Answer = Option<(String, Option<SynthRequest>)>;

/// `requests` without the repeats: each distinct lookup once, where a function
/// first made it.
pub fn distinct(requests: &[SynthRequest]) -> Vec<SynthRequest> {
    let mut out: Vec<SynthRequest> = Vec::with_capacity(requests.len());
    for q in requests {
        if !out.contains(q) {
            out.push(q.clone());
        }
    }
    out
}

/// What one decompile asked the ledger, and whether a forced run went off the
/// script it was given.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct FunctionRecord {
    /// Every lookup, in the order the function made them.
    pub requests: Vec<SynthRequest>,
    /// A recording worker's own answer to each of `requests`; empty from a
    /// forced run.
    pub answers: Vec<Answer>,
    /// A forced run asked more lookups than it had answers, fewer, or was
    /// answered with a name its factory does not hold.
    pub off_script: bool,
}

impl FunctionRecord {
    /// The worker's own answer to each distinct lookup ([`distinct`]), or
    /// `None` when there is no answer for every lookup or a repeated lookup
    /// was answered differently.
    pub fn own_answers(&self) -> Option<Vec<Answer>> {
        if self.answers.len() != self.requests.len() {
            return None;
        }
        let mut out: Vec<(&SynthRequest, &Answer)> = Vec::new();
        for (q, a) in self.requests.iter().zip(&self.answers) {
            match out.iter().find(|(seen, _)| *seen == q) {
                Some((_, first)) if first.as_ref().map(|x| &x.0) != a.as_ref().map(|x| &x.0) => return None,
                Some(_) => {}
                None => out.push((q, a)),
            }
        }
        Some(out.into_iter().map(|(_, a)| a.clone()).collect())
    }
}

/// What a worker's ledger does with a lookup.
#[derive(Debug)]
enum Mode {
    /// Record it and answer as the ledger would, remembering the lookup that
    /// minted each structure this process holds.
    Record(HashMap<String, SynthRequest>),
    /// Record it and answer with the next name in the queue, or with the name
    /// an identical earlier lookup of the same function got.
    Force(VecDeque<Option<String>>),
}

/// The per-process hook `synthesize` consults when it is present: the
/// architecture holds it, the driver arms it per function and takes the record
/// after.
#[derive(Debug)]
pub struct ShardHook {
    mode: Mode,
    at_load: Rc<AtLoad>,
    record: FunctionRecord,
    /// A forced function's distinct lookups so far, with their answers.
    answered: Vec<(SynthRequest, Option<String>)>,
}

/// The shared handle an architecture carries.
pub type ShardHandle = Rc<RefCell<ShardHook>>;

impl ShardHook {
    fn with(mode: Mode, at_load: Rc<AtLoad>) -> ShardHandle {
        Rc::new(RefCell::new(ShardHook { mode, at_load, record: FunctionRecord::default(), answered: Vec::new() }))
    }

    /// A hook that records every lookup and lets the ledger answer.
    pub fn recording(at_load: Rc<AtLoad>) -> ShardHandle {
        Self::with(Mode::Record(HashMap::new()), at_load)
    }

    /// A hook that records every lookup and answers from [`ShardHook::begin`].
    pub fn forcing(at_load: Rc<AtLoad>) -> ShardHandle {
        Self::with(Mode::Force(VecDeque::new()), at_load)
    }

    /// Start a function: forget the last record and, when forcing, queue the
    /// answers this function's distinct lookups get, in the order it first
    /// makes them ([`distinct`]).
    pub fn begin(&mut self, answers: &[Option<String>]) {
        self.record = FunctionRecord::default();
        self.answered.clear();
        if let Mode::Force(queue) = &mut self.mode {
            *queue = answers.iter().cloned().collect();
        }
    }

    /// Has the function under way asked anything yet?
    pub fn asked(&self) -> bool {
        !self.record.requests.is_empty()
    }

    /// Remember what a recording worker's own ledger answered `request` with,
    /// and, the first time a name comes back, that `request` minted it.
    fn own_answer(&mut self, answer: Option<&Datatype>, request: SynthRequest) {
        let Mode::Record(minted) = &mut self.mode else { return };
        let held = &self.at_load.held;
        let answer = answer.map(|ct| {
            let name = ct.get_name().to_string();
            let def = (!held.contains(&name)).then(|| minted.entry(name.clone()).or_insert(request).clone());
            (name, def)
        });
        self.record.answers.push(answer);
    }

    /// End a function: what it asked.
    pub fn take(&mut self) -> FunctionRecord {
        let mut record = std::mem::take(&mut self.record);
        if let Mode::Force(queue) = &mut self.mode {
            record.off_script |= !queue.is_empty();
            queue.clear();
        }
        record
    }
}

/// `lookup_or_mint`, through the hook.
pub(super) fn lookup(
    handle: &ShardHandle,
    types: &dyn TypeFactory,
    fields: Vec<TypeField>,
    size: int4,
    unclaimed: &[(int4, int4)],
    selfs: &[int4],
) -> Option<Rc<Datatype>> {
    let mut hook = handle.borrow_mut();
    let request = SynthRequest::of(&fields, size, unclaimed, selfs, &hook.at_load);
    hook.record.requests.push(request.clone());
    if matches!(hook.mode, Mode::Record(_)) {
        drop(hook);
        let answer = ledger::lookup_or_mint(types, fields, size, unclaimed, selfs);
        handle.borrow_mut().own_answer(answer.as_deref(), request);
        return answer;
    }
    // A decompile can ask the same thing twice -- a restarted pass measures the
    // same layout again -- and whether it does depends on what the process
    // decompiled before, so a repeat is answered as the first asking was and
    // does not take a queued answer.
    let repeat = hook.answered.iter().find(|(q, _)| *q == request).map(|(_, a)| a.clone());
    let answer = match (&mut hook.mode, repeat) {
        (Mode::Force(_), Some(answer)) => Some(answer),
        (Mode::Force(queue), None) => {
            let next = queue.pop_front();
            if let Some(answer) = &next {
                hook.answered.push((request, answer.clone()));
            }
            next
        }
        (Mode::Record(_), _) => unreachable!("answered by the ledger above"),
    };
    match answer {
        None => {
            hook.record.off_script = true;
            None
        }
        Some(None) => None,
        Some(Some(name)) => {
            let held =
                types.find_by_name(&name).ok().flatten().filter(|t| ledger::minted_number(t).is_some());
            if held.is_none() {
                hook.record.off_script = true;
            }
            held
        }
    }
}

// --- the replay ---------------------------------------------------------------

/// What the name `struct_<n>` is, to the ledger.
#[derive(Clone, Debug)]
enum Slot {
    /// Free: the first free slot is where a mint lands.
    Free,
    /// Held by something the ledger could not have minted.
    Foreign,
    /// A probe that could not answer; the ledger's scan stops here.
    Unknown,
    /// A synthesized structure.
    Synth { size: int4, layout: Layout, members: Vec<Member> },
}

/// A non-minting lookup whose answer would have been a mint.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WouldMint;

/// The ledger of a whole-program run, kept as data by a parent that decompiles
/// nothing: the same scan, the same decision and the same mint as
/// `ledger::lookup_or_mint`, over the answers the workers recorded.
#[derive(Clone, Debug)]
pub struct Replay {
    slots: Vec<Slot>,
    /// Every mint, in mint order: the table a forced worker installs.
    minted: Vec<(String, SynthRequest)>,
}

fn slot_name(n: usize) -> String {
    format!("struct_{n}")
}

impl Replay {
    /// The ledger as `types` holds it now: the state a run starts from.
    pub fn probe(types: &dyn TypeFactory) -> Replay {
        let slots = (0..ledger::MAX_LEDGER_SLOTS as usize)
            .map(|n| match types.find_by_name(&slot_name(n)) {
                Ok(None) => Slot::Free,
                Ok(Some(t)) => match ledger::layout_of(&t) {
                    Some(layout) => Slot::Synth {
                        size: t.get_size(),
                        layout,
                        members: ledger::members_of(&t),
                    },
                    None => Slot::Foreign,
                },
                Err(_) => Slot::Unknown,
            })
            .collect();
        Replay { slots, minted: Vec::new() }
    }

    /// The synthesized slots whose size lies in `sizes`, and the first free
    /// slot, scanned exactly as `ledger::entries` scans the factory.
    fn entries(&self, sizes: std::ops::RangeInclusive<i64>) -> (Vec<usize>, Option<usize>) {
        let mut held = Vec::new();
        for (n, slot) in self.slots.iter().enumerate() {
            match slot {
                Slot::Free => return (held, Some(n)),
                Slot::Unknown => return (held, None),
                Slot::Foreign => {}
                Slot::Synth { size, .. } => {
                    if sizes.contains(&i64::from(*size)) {
                        held.push(n);
                    }
                }
            }
        }
        (held, None)
    }

    fn synth(&self, n: usize) -> (&Layout, &[Member]) {
        match &self.slots[n] {
            Slot::Synth { layout, members, .. } => (layout, members),
            _ => unreachable!("entries yields synthesized slots only"),
        }
    }

    /// The held answer for `req`, or the free slot a mint would take.
    fn decide(&self, req: &SynthRequest) -> Result<usize, Option<usize>> {
        let (held, free) = self.entries(ledger::lookup_window(req.size));
        let layouts: Vec<&Layout> = held.iter().map(|&n| self.synth(n).0).collect();
        let fits = |i: usize| {
            req.unclaimed.is_empty()
                || ledger::keeps_unclaimed(self.synth(held[i]).1, &req.own, &req.unclaimed)
        };
        match ledger::best_of(&layouts, &req.want, fits) {
            Some(i) => Ok(held[i]),
            None => Err(free),
        }
    }

    /// Answer `req` as the ledger would, minting when nothing held answers.
    pub fn lookup_or_mint(&mut self, req: &SynthRequest) -> Option<String> {
        match self.decide(req) {
            Ok(n) => Some(slot_name(n)),
            Err(free) => {
                let n = free?;
                self.slots[n] = Slot::Synth {
                    size: req.size,
                    layout: req.want.clone(),
                    members: req.own.clone(),
                };
                self.minted.push((slot_name(n), req.clone()));
                Some(slot_name(n))
            }
        }
    }

    /// Answer `req` from what is held now, without minting.
    pub fn lookup(&self, req: &SynthRequest) -> Result<Option<String>, WouldMint> {
        match self.decide(req) {
            Ok(n) => Ok(Some(slot_name(n))),
            Err(Some(_)) => Err(WouldMint),
            Err(None) => Ok(None),
        }
    }

    /// `ledger::superseded_names` over the replayed ledger.
    pub fn superseded_names(&self) -> Vec<String> {
        let (held, _) = self.entries(i64::MIN..=i64::MAX);
        let layouts: Vec<&Layout> = held.iter().map(|&n| self.synth(n).0).collect();
        (0..held.len())
            .filter(|&i| ledger::is_superseded(&layouts, i))
            .map(|i| slot_name(held[i]))
            .collect()
    }

    /// Every structure the replay minted, in mint order.
    pub fn table(&self) -> &[(String, SynthRequest)] {
        &self.minted
    }

    /// The `struct_<n>` names held by something the run did not mint.
    pub fn held(&self) -> Vec<String> {
        let minted: HashSet<&str> = self.minted.iter().map(|(n, _)| n.as_str()).collect();
        (0..self.slots.len())
            .filter(|&n| matches!(self.slots[n], Slot::Foreign | Slot::Synth { .. }))
            .map(slot_name)
            .filter(|name| !minted.contains(name.as_str()))
            .collect()
    }
}

// --- renaming -----------------------------------------------------------------

/// The renaming that turns a function's own answers into the serial ones, as
/// `(own name, serial name)` pairs, or `None` when an answer names a structure
/// whose members differ from the serial answer's, or two answers would share a
/// name on one side only: then the function's text is not the serial text with
/// other numbers.
pub fn renaming(own: &[Answer], serial: &[Answer]) -> Option<Vec<(String, String)>> {
    if own.len() != serial.len() {
        return None;
    }
    let mut map: Vec<(String, String)> = Vec::new();
    for (o, s) in own.iter().zip(serial) {
        match (o, s) {
            (None, None) => {}
            (Some((on, od)), Some((sn, sd))) => {
                let same = match (od, sd) {
                    (None, None) => on == sn,
                    (Some(od), Some(sd)) => od.defines_same(sd),
                    _ => false,
                };
                if !same {
                    return None;
                }
                match map.iter().find(|(a, b)| a == on || b == sn) {
                    Some((a, b)) if a == on && b == sn => {}
                    Some(_) => return None,
                    None => map.push((on.clone(), sn.clone())),
                }
            }
            _ => return None,
        }
    }
    Some(map)
}

/// `text` with every `struct_<N>` identifier renamed by `map`, leaving the
/// names in `keep`. `None` when the text names one neither covers, or spells
/// one inside a string or character literal, whose bytes a rename must not
/// touch and whose meaning it cannot know.
pub fn rename_identifiers(text: &str, map: &[(String, String)], keep: &[String]) -> Option<String> {
    const PREFIX: &[u8] = b"struct_";
    let bytes = text.as_bytes();
    let ident = |b: u8| b.is_ascii_alphanumeric() || b == b'_';
    let mut out = String::with_capacity(text.len());
    let (mut last, mut i) = (0, 0);
    let mut quote: Option<u8> = None;
    // Inside a comment a quote opens nothing: `// can't` is prose.
    let mut comment: Option<&[u8]> = None;
    while i < bytes.len() {
        let b = bytes[i];
        if let Some(q) = quote {
            if b == b'\\' {
                i += 2;
                continue;
            }
            if b == q {
                quote = None;
            } else if bytes[i..].starts_with(PREFIX) {
                return None;
            }
            i += 1;
            continue;
        }
        match comment {
            Some(close) if bytes[i..].starts_with(close) => {
                comment = None;
                i += close.len();
                continue;
            }
            Some(_) => {}
            None if bytes[i..].starts_with(b"//") => {
                comment = Some(b"\n");
                i += 2;
                continue;
            }
            None if bytes[i..].starts_with(b"/*") => {
                comment = Some(b"*/");
                i += 2;
                continue;
            }
            None if b == b'"' || b == b'\'' => {
                quote = Some(b);
                i += 1;
                continue;
            }
            None => {}
        }
        if !bytes[i..].starts_with(PREFIX) || (i > 0 && ident(bytes[i - 1])) {
            i += 1;
            continue;
        }
        let mut end = i + PREFIX.len();
        while end < bytes.len() && bytes[end].is_ascii_digit() {
            end += 1;
        }
        if end == i + PREFIX.len() || (end < bytes.len() && ident(bytes[end])) {
            i = end;
            continue;
        }
        let name = &text[i..end];
        match map.iter().find(|(from, _)| from == name) {
            Some((_, to)) => {
                out.push_str(&text[last..i]);
                out.push_str(to);
                last = end;
            }
            None if keep.iter().any(|k| k == name) => {}
            None => return None,
        }
        i = end;
    }
    out.push_str(&text[last..]);
    Some(out)
}

/// The `struct_<n>` names `types` holds now.
pub fn held_names(types: &dyn TypeFactory) -> Vec<String> {
    (0..ledger::MAX_LEDGER_SLOTS as usize)
        .map(slot_name)
        .filter(|name| types.find_by_name(name).ok().flatten().is_some())
        .collect()
}

/// Destroy every structure this process minted since `held` was taken, and
/// every type built on one, so a replayed table can take the names.
///
/// The types built on a minted structure go too -- the pointer a parameter was
/// typed with, an array or a partial of it -- because a pointer keeps its
/// pointee alive, and a header rendered from this factory would otherwise
/// reach the old structure through it and declare a name twice.
pub fn forget_minted(types: &TypeFactoryImpl, held: &[String]) -> Result<(), String> {
    let mut gone: Vec<Rc<Datatype>> = Vec::new();
    for name in (0..ledger::MAX_LEDGER_SLOTS as usize).map(slot_name) {
        if held.contains(&name) {
            continue;
        }
        // Completing a structure leaves its incomplete shell under the same
        // name, so a name is destroyed until nothing minted answers to it.
        while let Some(ct) = types.find_by_name(&name).ok().flatten() {
            if ledger::minted_number(&ct).is_none() {
                break;
            }
            types.destroy_type(&ct).map_err(|e| format!("{name}: {}", e.explain()))?;
            gone.push(ct);
        }
    }
    if gone.is_empty() {
        return Ok(());
    }
    let mut built_on: std::collections::HashMap<*const Datatype, bool> =
        gone.iter().map(|ct| (Rc::as_ptr(ct), true)).collect();
    for ct in types.dependent_order() {
        if reaches(&ct, &mut built_on, 0) && !ct.is_core_type() {
            types.destroy_type(&ct).map_err(|e| format!("{}: {}", ct.get_name(), e.explain()))?;
        }
    }
    Ok(())
}

/// Is `ct` a minted structure of `seen`, or built on one?
fn reaches(ct: &Rc<Datatype>, seen: &mut std::collections::HashMap<*const Datatype, bool>, depth: u32) -> bool {
    if let Some(&known) = seen.get(&Rc::as_ptr(ct)) {
        return known;
    }
    if depth > MAX_RECIPE_DEPTH * 4 {
        return false;
    }
    let parts: Vec<Rc<Datatype>> = match &ct.kind {
        DatatypeKind::Pointer { ptrto, .. } => vec![Rc::clone(ptrto)],
        DatatypeKind::PointerRel { ptrto, parent, .. } => vec![Rc::clone(ptrto), Rc::clone(parent)],
        DatatypeKind::Array { arrayof, .. } => vec![Rc::clone(arrayof)],
        DatatypeKind::PartialStruct { container, .. } | DatatypeKind::PartialUnion { container, .. } => {
            vec![Rc::clone(container)]
        }
        _ => ct.get_typedef().map(|t| vec![Rc::clone(t)]).unwrap_or_default(),
    };
    let hit = parts.iter().any(|p| reaches(p, seen, depth + 1));
    seen.insert(Rc::as_ptr(ct), hit);
    hit
}

/// Mint every structure of `table` into `types`, in order, exactly as the
/// ledger mints one, and check that each holds the layout the replay decided
/// with.
pub fn install_table(types: &dyn TypeFactory, table: &[(String, SynthRequest)]) -> Result<(), String> {
    for (name, req) in table {
        let fields = req
            .fields
            .iter()
            .map(|f| {
                let ct = f.ty.as_ref().and_then(|r| r.build(types))?;
                Some(TypeField::new(f.ident, f.offset, f.name.clone(), ct))
            })
            .collect::<Option<Vec<TypeField>>>()
            .ok_or_else(|| format!("{name}: a field type this process cannot rebuild"))?;
        if types.find_by_name(name).ok().flatten().is_some() {
            return Err(format!("{name}: the name is already held"));
        }
        let shell = types.get_type_struct(name).map_err(|e| format!("{name}: {}", e.explain()))?;
        let ct = types
            .set_fields_struct_raw(&shell, fields, Vec::new(), req.size, 1, 0)
            .map_err(|e| format!("{name}: {}", e.explain()))?;
        if ledger::layout_of(&ct).as_ref() != Some(&req.want) || ledger::members_of(&ct) != req.own {
            return Err(format!("{name}: the rebuilt layout differs from the recorded one"));
        }
    }
    Ok(())
}

// --- wire format --------------------------------------------------------------

fn put_u32(out: &mut Vec<u8>, v: u32) {
    out.extend_from_slice(&v.to_le_bytes());
}

fn put_i32(out: &mut Vec<u8>, v: int4) {
    out.extend_from_slice(&v.to_le_bytes());
}

fn put_str(out: &mut Vec<u8>, s: &str) {
    put_u32(out, s.len() as u32);
    out.extend_from_slice(s.as_bytes());
}

struct Reader<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl Reader<'_> {
    fn take(&mut self, n: usize) -> Option<&[u8]> {
        let end = self.pos.checked_add(n)?;
        let out = self.bytes.get(self.pos..end)?;
        self.pos = end;
        Some(out)
    }
    fn u8(&mut self) -> Option<u8> {
        Some(self.take(1)?[0])
    }
    fn u32(&mut self) -> Option<u32> {
        Some(u32::from_le_bytes(self.take(4)?.try_into().ok()?))
    }
    fn i32(&mut self) -> Option<int4> {
        Some(int4::from_le_bytes(self.take(4)?.try_into().ok()?))
    }
    fn u64(&mut self) -> Option<u64> {
        Some(u64::from_le_bytes(self.take(8)?.try_into().ok()?))
    }
    fn string(&mut self) -> Option<String> {
        let n = self.u32()? as usize;
        String::from_utf8(self.take(n)?.to_vec()).ok()
    }
    fn bool(&mut self) -> Option<bool> {
        match self.u8()? {
            0 => Some(false),
            1 => Some(true),
            _ => None,
        }
    }
    /// A count read off the wire, bounded by the bytes that remain.
    fn count(&mut self) -> Option<usize> {
        let n = self.u32()? as usize;
        (n <= self.bytes.len() - self.pos).then_some(n)
    }
}

fn put_recipe(out: &mut Vec<u8>, r: &TypeRecipe) {
    match r {
        TypeRecipe::Named { name, id, meta, size } => {
            out.push(0);
            put_str(out, name);
            out.extend_from_slice(&id.to_le_bytes());
            put_i32(out, *meta);
            put_i32(out, *size);
        }
        TypeRecipe::Pointer { size, wordsize, to } => {
            out.push(1);
            put_i32(out, *size);
            put_u32(out, *wordsize);
            put_recipe(out, to);
        }
        TypeRecipe::Array { count, elem } => {
            out.push(2);
            put_i32(out, *count);
            put_recipe(out, elem);
        }
    }
}

fn read_recipe(r: &mut Reader, depth: u32) -> Option<TypeRecipe> {
    if depth > MAX_RECIPE_DEPTH {
        return None;
    }
    Some(match r.u8()? {
        0 => TypeRecipe::Named { name: r.string()?, id: r.u64()?, meta: r.i32()?, size: r.i32()? },
        1 => TypeRecipe::Pointer {
            size: r.i32()?,
            wordsize: r.u32()?,
            to: Box::new(read_recipe(r, depth + 1)?),
        },
        2 => TypeRecipe::Array { count: r.i32()?, elem: Box::new(read_recipe(r, depth + 1)?) },
        _ => return None,
    })
}

fn put_key(out: &mut Vec<u8>, k: &FieldKey) {
    put_i32(out, k.offset);
    put_i32(out, k.width);
    put_str(out, &k.ty);
    out.push(u8::from(k.pointer));
    out.push(u8::from(k.opaque));
}

fn read_key(r: &mut Reader) -> Option<FieldKey> {
    Some(FieldKey {
        offset: r.i32()?,
        width: r.i32()?,
        ty: r.string()?,
        pointer: r.bool()?,
        opaque: r.bool()?,
    })
}

fn put_request(out: &mut Vec<u8>, q: &SynthRequest) {
    put_i32(out, q.size);
    put_i32(out, q.want.size);
    put_u32(out, q.want.fields.len() as u32);
    for k in &q.want.fields {
        put_key(out, k);
    }
    put_u32(out, q.own.len() as u32);
    for m in &q.own {
        put_i32(out, m.offset);
        put_i32(out, m.size);
        match &m.claim {
            Some(k) => {
                out.push(1);
                put_key(out, k);
            }
            None => out.push(0),
        }
    }
    put_u32(out, q.unclaimed.len() as u32);
    for &(o, w) in &q.unclaimed {
        put_i32(out, o);
        put_i32(out, w);
    }
    put_u32(out, q.fields.len() as u32);
    for f in &q.fields {
        put_i32(out, f.ident);
        put_i32(out, f.offset);
        put_str(out, &f.name);
        match &f.ty {
            Some(t) => {
                out.push(1);
                put_recipe(out, t);
            }
            None => out.push(0),
        }
    }
}

fn read_request(r: &mut Reader) -> Option<SynthRequest> {
    let size = r.i32()?;
    let want_size = r.i32()?;
    let nk = r.count()?;
    let mut keys = Vec::with_capacity(nk);
    for _ in 0..nk {
        keys.push(read_key(r)?);
    }
    let nm = r.count()?;
    let mut own = Vec::with_capacity(nm);
    for _ in 0..nm {
        let offset = r.i32()?;
        let msize = r.i32()?;
        let claim = if r.bool()? { Some(read_key(r)?) } else { None };
        own.push(Member { offset, size: msize, claim });
    }
    let nu = r.count()?;
    let mut unclaimed = Vec::with_capacity(nu);
    for _ in 0..nu {
        unclaimed.push((r.i32()?, r.i32()?));
    }
    let nf = r.count()?;
    let mut fields = Vec::with_capacity(nf);
    for _ in 0..nf {
        let ident = r.i32()?;
        let offset = r.i32()?;
        let name = r.string()?;
        let ty = if r.bool()? { Some(read_recipe(r, 0)?) } else { None };
        fields.push(FieldRecipe { ident, offset, name, ty });
    }
    Some(SynthRequest { size, want: Layout { size: want_size, fields: keys }, own, unclaimed, fields })
}

fn put_answer(out: &mut Vec<u8>, a: &Answer) {
    match a {
        None => out.push(0),
        Some((name, def)) => {
            out.push(1);
            put_str(out, name);
            match def {
                Some(q) => {
                    out.push(1);
                    put_request(out, q);
                }
                None => out.push(0),
            }
        }
    }
}

fn read_answer(r: &mut Reader) -> Option<Answer> {
    if !r.bool()? {
        return Some(None);
    }
    let name = r.string()?;
    let def = if r.bool()? { Some(read_request(r)?) } else { None };
    Some(Some((name, def)))
}

impl FunctionRecord {
    /// Append the record's wire form.
    pub fn encode(&self, out: &mut Vec<u8>) {
        out.push(u8::from(self.off_script));
        put_u32(out, self.requests.len() as u32);
        for q in &self.requests {
            put_request(out, q);
        }
        put_u32(out, self.answers.len() as u32);
        for a in &self.answers {
            put_answer(out, a);
        }
    }

    /// Read a record from the front of `bytes`, returning it and the bytes it
    /// took.
    pub fn decode(bytes: &[u8]) -> Option<(FunctionRecord, usize)> {
        let mut r = Reader { bytes, pos: 0 };
        let off_script = r.bool()?;
        let n = r.count()?;
        let mut requests = Vec::with_capacity(n);
        for _ in 0..n {
            requests.push(read_request(&mut r)?);
        }
        let n = r.count()?;
        let mut answers = Vec::with_capacity(n);
        for _ in 0..n {
            answers.push(read_answer(&mut r)?);
        }
        Some((FunctionRecord { requests, answers, off_script }, r.pos))
    }
}

/// The wire form of a replayed table.
pub fn encode_table(table: &[(String, SynthRequest)]) -> Vec<u8> {
    let mut out = Vec::new();
    put_u32(&mut out, table.len() as u32);
    for (name, q) in table {
        put_str(&mut out, name);
        put_request(&mut out, q);
    }
    out
}

/// Read a table written by [`encode_table`].
pub fn decode_table(bytes: &[u8]) -> Option<Vec<(String, SynthRequest)>> {
    let mut r = Reader { bytes, pos: 0 };
    let n = r.count()?;
    let mut out = Vec::with_capacity(n);
    for _ in 0..n {
        out.push((r.string()?, read_request(&mut r)?));
    }
    (r.pos == bytes.len()).then_some(out)
}

#[cfg(test)]
mod tests;

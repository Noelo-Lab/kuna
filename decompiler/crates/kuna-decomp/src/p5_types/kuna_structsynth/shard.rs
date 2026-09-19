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
//!   them, and a recipe for each field type;
//! * the parent REPLAYS the lookups in target order through the ledger's own
//!   decision ([`Replay`]), which yields every answer, every minted name and
//!   the superseded set;
//! * a fresh worker INSTALLS the replayed structures ([`install_table`]) and
//!   decompiles each function again with its lookups FORCED to the replayed
//!   answers, recording what it asked so the parent can check that the replay
//!   was told the truth.
//!
//! What a lookup asks does not depend on what the ledger holds: `synthesize`
//! collects its evidence before it installs anything, and a field takes the
//! type an access carried. The one path by which an earlier answer reaches a
//! later request is a restarted decompile, whose parameter can keep the type
//! the first attempt locked; that is why the forced run's requests are compared
//! with the recorded ones, and why a field typed by a synthesized structure is
//! never given a recipe.

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use kuna_base::types::int4;

use super::ledger::{self, FieldKey, Layout, Member};
use crate::dtype::{Datatype, DatatypeKind, TypeFactory, TypeField};

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
            _ if ledger::layout_of(ct).is_some() => None,
            _ => Some(TypeRecipe::Named {
                name: ct.get_name().to_string(),
                id: ct.get_id(),
                meta: ct.get_metatype().as_i32(),
                size: ct.get_size(),
            }),
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
    /// Record the arguments of one `lookup_or_mint` call.
    fn of(fields: &[TypeField], size: int4, unclaimed: &[(int4, int4)]) -> SynthRequest {
        SynthRequest {
            size,
            want: ledger::layout_of_fields(fields, size),
            own: fields.iter().map(Member::of).collect(),
            unclaimed: unclaimed.to_vec(),
            fields: fields
                .iter()
                .map(|f| FieldRecipe {
                    ident: f.ident,
                    offset: f.offset,
                    name: f.name.clone(),
                    ty: TypeRecipe::of(&f.field_type),
                })
                .collect(),
        }
    }

    /// Can another process rebuild every field of this layout?
    pub fn portable(&self) -> bool {
        self.fields.iter().all(|f| f.ty.is_some())
    }
}

/// What one decompile asked the ledger, and whether a forced run went off the
/// script it was given.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct FunctionRecord {
    /// Every lookup, in the order the function made them.
    pub requests: Vec<SynthRequest>,
    /// A forced run asked more lookups than it had answers, fewer, or was
    /// answered with a name its factory does not hold.
    pub off_script: bool,
}

/// What a worker's ledger does with a lookup.
#[derive(Debug)]
enum Mode {
    /// Record it and answer as the ledger would.
    Record,
    /// Record it and answer with the next name in the queue.
    Force(VecDeque<Option<String>>),
}

/// The per-process hook `synthesize` consults when it is present: the
/// architecture holds it, the driver arms it per function and takes the record
/// after.
#[derive(Debug)]
pub struct ShardHook {
    mode: Mode,
    record: FunctionRecord,
}

/// The shared handle an architecture carries.
pub type ShardHandle = Rc<RefCell<ShardHook>>;

impl ShardHook {
    /// A hook that records every lookup and lets the ledger answer.
    pub fn recording() -> ShardHandle {
        Rc::new(RefCell::new(ShardHook { mode: Mode::Record, record: FunctionRecord::default() }))
    }

    /// A hook that records every lookup and answers from [`ShardHook::begin`].
    pub fn forcing() -> ShardHandle {
        Rc::new(RefCell::new(ShardHook {
            mode: Mode::Force(VecDeque::new()),
            record: FunctionRecord::default(),
        }))
    }

    /// Start a function: forget the last record and, when forcing, queue the
    /// answers this function's lookups get, in order.
    pub fn begin(&mut self, answers: &[Option<String>]) {
        self.record = FunctionRecord::default();
        if let Mode::Force(queue) = &mut self.mode {
            *queue = answers.iter().cloned().collect();
        }
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
    hook: &ShardHandle,
    types: &dyn TypeFactory,
    fields: Vec<TypeField>,
    size: int4,
    unclaimed: &[(int4, int4)],
) -> Option<Rc<Datatype>> {
    let mut hook = hook.borrow_mut();
    hook.record.requests.push(SynthRequest::of(&fields, size, unclaimed));
    let forced = match &mut hook.mode {
        Mode::Record => None,
        Mode::Force(queue) => Some(queue.pop_front()),
    };
    match forced {
        None => {
            drop(hook);
            ledger::lookup_or_mint(types, fields, size, unclaimed)
        }
        Some(None) => {
            hook.record.off_script = true;
            None
        }
        Some(Some(None)) => None,
        Some(Some(Some(name))) => {
            let held = types.find_by_name(&name).ok().flatten().filter(|t| ledger::layout_of(t).is_some());
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

impl FunctionRecord {
    /// Append the record's wire form.
    pub fn encode(&self, out: &mut Vec<u8>) {
        out.push(u8::from(self.off_script));
        put_u32(out, self.requests.len() as u32);
        for q in &self.requests {
            put_request(out, q);
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
        Some((FunctionRecord { requests, off_script }, r.pos))
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

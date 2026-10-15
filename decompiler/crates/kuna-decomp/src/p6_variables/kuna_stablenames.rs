//! Default local names stay with their variables when a directive is replayed.
//!
//! The naming pass numbers unnamed locals in location order (`v1`, `v2`, ... or
//! `iVar1` under Ghidra names). A `name`/`type` directive is read against the
//! first decompile's output and applied by a second, and the second numbered
//! from scratch: the renamed local took no number, so every later local moved
//! down one, and a following directive naming `v9` reached a different
//! variable than the caller saw.
//!
//! [`snapshot`] records the first decompile's numbering by identity as the
//! naming pass hands it out (later actions insert casts and restructure blocks,
//! which change the operations a variable's instances come from), and
//! [`carry`] hands it on: each instance of a numbered variable by three keys
//! of decreasing precision, and each numbered Symbol by its storage. The keys
//! are the defining operation itself (instruction and creation time, which a
//! decompile repeats while its analysis does), the operation's shape
//! (instruction, opcode, output and inputs), and the instance's place
//! (storage, width and instruction). Temporaries, JOIN storage and operation
//! references are renumbered by every decompile, so only their space is part
//! of a key, and a key two numbered variables share names neither. The next
//! decompile of the function matches each instance by the most precise key
//! that names a number, and gives each local the number most of its instances
//! had (see [`best`]; `varmap.rs (ScopeLocal::next_default_name)`).
//!
//! - a renamed local's number stays unused, also for the part of a merged
//!   variable the rename left unbound (named last, and given its number back
//!   when the rename bound nothing at all);
//! - a local that cannot keep its number (a rename took the name, `name v3 v7`)
//!   and a local the first decompile did not have (a split, a retype that
//!   changes the analysis) take a number past the first decompile's last, so a
//!   freed number never names a different local; a variable that is gone
//!   leaves its number unused.
//!
//! A `type` that keeps a local's width and kind ([`retypes_as_unit`]) is
//! carried with the record too, and typed onto every instance of the local
//! after the merge ([`retype_targets`]), so the analysis does not split it. A
//! pointer type inference has to read ([`reaches_inference`]) is also locked
//! before inference runs on every varnode an operation of the variable defines
//! ([`lock_retyped`]), and the merge then follows the previous decompile's
//! variables where the keys still identify them ([`same_variable`]).
//!
//! Without a carried record the pass numbers in sequence, exactly as before.

use std::collections::{BTreeMap, BTreeSet};

use kuna_base::address::Address;
use kuna_base::space::spacetype::{IPTR_FSPEC, IPTR_INTERNAL, IPTR_IOP, IPTR_JOIN};
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::context::{HighVariableId, VarnodeId};
use crate::funcdata::Funcdata;

/// An instance by its defining operation: the instruction and creation time
/// (invalid and `u32::MAX` for an input), its storage and width.
pub type Op = (Address, u32, Address, int4);

/// An instance by the shape of its defining operation: the instruction and
/// opcode (invalid and `None` for an input), its storage and width, and a hash
/// of the storage and width of each input.
pub type Shape = (Address, Option<OpCode>, Address, int4, u64);

/// An instance by its storage, width and use point (the defining instruction,
/// or the entry minus one for an input).
pub type Place = (Address, int4, Address);

/// The three keys of one instance.
pub type Keys = (Op, Shape, Place);

/// One decompile's default numbering, handed to the next decompile of the
/// same function.
#[derive(Clone, Debug, Default)]
pub struct PriorNames {
    /// The name of the local each instance belonged to, by [`Op`]; `None`
    /// where the key belonged to more than one name.
    pub ops: BTreeMap<Op, Option<String>>,
    /// The same by [`Shape`].
    pub shapes: BTreeMap<Shape, Option<String>>,
    /// The same by [`Place`].
    pub places: BTreeMap<Place, Option<String>>,
    /// The number each numbered Symbol had, by its storage and width.
    pub symbols: BTreeMap<(Address, int4), int4>,
    /// The numbers renamed locals held, with the names they were given.
    pub vacated: BTreeMap<int4, String>,
    /// The default-shaped names a rename gave a local (`name v3 v7`).
    pub given: BTreeSet<String>,
    /// The other names a directive's local was printed under before it was
    /// renamed or retyped (a `libcnames` spelling such as `fd`): no other local
    /// is given one again.
    pub vacated_names: BTreeSet<String>,
    /// (kuna `libcnames`) The library name each number printed as.
    pub aliases: BTreeMap<int4, String>,
    /// One past the last number handed out.
    pub end: int4,
    /// Each name-only rename of a printed local: the name it was printed under
    /// and the name it was given (see [`rename_targets`]).
    pub renames: BTreeMap<String, String>,
    /// Each name-only rename of a stack local's Symbol, by the Symbol's
    /// storage and width: the analysis rebuilds the Symbol, and the naming
    /// pass gives it this name instead of a number.
    pub symbol_renames: BTreeMap<(Address, int4), String>,
    /// Each retype that keeps a printed local's width and kind
    /// ([`retypes_as_unit`]): the name it was printed under and its new type,
    /// applied to the whole variable after the merge ([`retype_targets`]).
    pub retypes: BTreeMap<String, UnitRetype>,
    /// The local each varnode's keys matched, as the merge asks it
    /// ([`same_variable`]).
    pub merged_names: std::cell::RefCell<MergeMemo>,
}

/// Each varnode's matched local, by an index into the names seen so far.
#[derive(Clone, Debug, Default)]
pub struct MergeMemo {
    locals: std::collections::HashMap<VarnodeId, Option<u32>>,
    names: BTreeMap<String, u32>,
}

impl PriorNames {
    pub fn is_empty(&self) -> bool {
        self.ops.is_empty()
            && self.symbols.is_empty()
            && self.vacated.is_empty()
            && self.given.is_empty()
            && self.vacated_names.is_empty()
            && self.aliases.is_empty()
            && self.renames.is_empty()
            && self.symbol_renames.is_empty()
            && self.retypes.is_empty()
    }
}

/// The index of a default local name, `v9` -> 9 (or `iVar9` -> 9 under Ghidra
/// names, where a stack Symbol is still `v9`); `None` for any other name.
pub fn default_name_index(name: &str, angr: bool) -> Option<int4> {
    let index = |digits: &str| {
        let canonical = !digits.is_empty() && !digits.starts_with('0') && digits.bytes().all(|b| b.is_ascii_digit());
        if canonical { digits.parse().ok() } else { None }
    };
    if let Some(i) = name.strip_prefix('v').and_then(index) {
        return Some(i);
    }
    let at = name.find("Var").filter(|_| !angr)?;
    if name[..at].chars().count() > 1 {
        return None;
    }
    index(&name[at + 3..])
}

/// `addr` with the offset of storage every decompile renumbers dropped.
fn stable(addr: &Address) -> Address {
    match addr.get_space() {
        Some(space) if matches!(space.get_type(), IPTR_INTERNAL | IPTR_JOIN | IPTR_IOP | IPTR_FSPEC) => {
            Address::new(std::rc::Rc::clone(space), 0)
        }
        _ => addr.clone(),
    }
}

/// The [`Keys`] of `vn`; `None` for a constant.
fn keys_of(fd: &Funcdata, vn: VarnodeId) -> Option<Keys> {
    let v = fd.vbank().get(vn).filter(|v| !v.is_constant())?;
    let storage = stable(v.get_addr());
    let place = (storage.clone(), v.get_size(), fd.vn_use_point(vn));
    Some(match v.get_def().filter(|_| v.is_written()).and_then(|op| fd.obank().get(op)) {
        Some(op) => {
            let inputs = (0..op.num_input())
                .filter_map(|slot| op.get_in(slot).and_then(|input| fd.vbank().get(input)))
                .fold(0xcbf2_9ce4_8422_2325u64, |hash, input| {
                    let addr = stable(input.get_addr());
                    let space = addr.get_space().map_or(u64::MAX, |space| space.get_index() as u64);
                    [space, addr.get_offset(), input.get_size() as u64]
                        .into_iter()
                        .fold(hash, |hash, word| (hash ^ word).wrapping_mul(0x0000_0100_0000_01b3))
                });
            let pc = op.get_addr().clone();
            (
                (pc.clone(), op.get_time(), storage.clone(), v.get_size()),
                (pc, Some(op.code()), storage, v.get_size(), inputs),
                place,
            )
        }
        None => (
            (Address::new_invalid(), u32::MAX, storage.clone(), v.get_size()),
            (Address::new_invalid(), None, storage, v.get_size(), 0),
            place,
        ),
    })
}

/// The [`Keys`] of each non-constant instance of `high`.
fn instances(fd: &Funcdata, high: HighVariableId) -> Vec<Keys> {
    instances_with_keys(fd, high).into_iter().map(|(_, keys)| keys).collect()
}

fn instances_with_keys(fd: &Funcdata, high: HighVariableId) -> Vec<(VarnodeId, Keys)> {
    let Some(h) = fd.high_bank().get(high) else { return Vec::new() };
    (0..h.num_instances()).map(|i| h.get_instance(i)).filter_map(|vn| Some((vn, keys_of(fd, vn)?))).collect()
}

/// The local an instance belonged to: by its keys ([`prior_name`]), or else
/// by the type lock [`lock_retyped`] put on it, when the type inference read
/// changed the operations it comes from.
fn matched_name<'a>(fd: &Funcdata, prior: &'a PriorNames, vn: VarnodeId, keys: &Keys) -> Option<&'a String> {
    if let Some(name) = prior_name(prior, keys) {
        return Some(name);
    }
    let v = fd.vbank().get(vn).filter(|v| v.is_type_lock())?;
    let mut locked =
        prior.retypes.iter().filter(|(_, retype)| retype.infer && std::rc::Rc::ptr_eq(&retype.ct, v.get_type()));
    let (name, _) = locked.next()?;
    locked.next().is_none().then_some(name)
}

/// The name the carried record holds for an instance: by its most precise key
/// the record holds, and none where that key belonged to more than one name.
fn prior_name<'a>(prior: &'a PriorNames, (op, shape, place): &Keys) -> Option<&'a String> {
    match prior.ops.get(op) {
        Some(name) => name.as_ref(),
        None => match prior.shapes.get(shape) {
            Some(name) => name.as_ref(),
            None => prior.places.get(place).and_then(|name| name.as_ref()),
        },
    }
}

/// How many of `high`'s instances belonged to each name in the carried
/// record, as (register and stack instances, temporaries). An instance is
/// matched by the most precise key the record holds: a key it holds for more
/// than one name matches nothing.
fn tally(fd: &Funcdata, high: HighVariableId) -> BTreeMap<String, (usize, usize)> {
    let prior = fd.kuna_prior_names();
    let mut counts: BTreeMap<String, (usize, usize)> = BTreeMap::new();
    for (vn, keys) in instances_with_keys(fd, high) {
        if let Some(name) = matched_name(fd, prior, vn, &keys) {
            let count = counts.entry(name.clone()).or_default();
            if keys.0 .2.get_space().is_some_and(|space| space.get_type() == IPTR_INTERNAL) {
                count.1 += 1;
            } else {
                count.0 += 1;
            }
        }
    }
    counts
}

/// The number most of `high`'s instances had (the lowest on a tie); `None`
/// without a carried record or a match. Instances in registers and on the
/// stack decide before temporaries: a temporary merges wherever its copies
/// lead, so after a retype changes the merge it can sit in a different
/// variable, while the storage the declaration lists is what the reader knows
/// the variable by. A renamed local's number counts only when nothing else
/// matches: a variable that took in part of the renamed one is still the
/// variable it was.
pub fn best(fd: &Funcdata, high: HighVariableId) -> Option<int4> {
    let prior = fd.kuna_prior_names();
    if prior.ops.is_empty() {
        return None;
    }
    let angr = fd.get_arch().name_style_angr;
    let mut counts: BTreeMap<int4, (usize, usize)> = BTreeMap::new();
    for (name, count) in tally(fd, high) {
        if let Some(i) = default_name_index(&name, angr) {
            let total = counts.entry(i).or_default();
            total.0 += count.0;
            total.1 += count.1;
        }
    }
    let renamed = |i: &int4| prior.vacated.contains_key(i);
    let pick = |keep: &dyn Fn(&int4) -> bool| {
        counts.iter().filter(|(i, _)| keep(i)).max_by(|a, b| a.1.cmp(b.1).then(b.0.cmp(a.0))).map(|(i, _)| *i)
    };
    pick(&|i| !renamed(i)).or_else(|| pick(&|_| true))
}

/// The renamed local's number `high` continues (its [`best`]), if any. The
/// naming pass names such a local last: it is the part of a merged variable
/// the rename left unbound, and takes a new number, unless no local printed
/// the rename's name at all, in which case it is the renamed local and keeps
/// its number (see [`orphaned`]).
pub fn renamed_number(fd: &Funcdata, high: HighVariableId) -> Option<int4> {
    best(fd, high).filter(|i| fd.kuna_prior_names().vacated.contains_key(i))
}

/// Did no local take the name the rename of number `i` gave?
pub fn orphaned(fd: &Funcdata, i: int4) -> bool {
    let Some(name) = fd.kuna_prior_names().vacated.get(&i) else { return false };
    !fd.high_bank().iter().any(|(_, h)| h.kuna_name() == Some(name.as_str()))
}

/// The local each name-only rename names: the one holding most of the renamed
/// variable's instances (register and stack instances first). A name-only
/// rename maps no Symbol over the variable, so it cannot change what the
/// analysis merges or how it types it; the name is attached here, after the
/// merge, to the whole variable.
pub fn rename_targets(fd: &Funcdata, tallies: &[(HighVariableId, BTreeMap<String, (usize, usize)>)]) -> BTreeMap<HighVariableId, String> {
    let renames = &fd.kuna_prior_names().renames;
    let mut best: BTreeMap<&String, ((usize, usize), HighVariableId)> = BTreeMap::new();
    for (high, counts) in tallies {
        for (name, count) in counts {
            if let Some((from, _)) = renames.get_key_value(name) {
                if best.get(from).is_none_or(|(held, _)| *count > *held) {
                    best.insert(from, (*count, *high));
                }
            }
        }
    }
    best.into_iter().map(|(from, (_, high))| (high, renames[from].clone())).collect()
}

/// [`tally`] of every local the naming pass names on its own, when a record
/// is carried: a piece of a structure takes its root's name, so it neither
/// keeps a number nor is renamed apart from it.
pub fn tallies(fd: &mut Funcdata) -> Vec<(HighVariableId, BTreeMap<String, (usize, usize)>)> {
    if fd.kuna_prior_names().ops.is_empty() {
        return Vec::new();
    }
    let highs: Vec<HighVariableId> =
        fd.high_bank().iter().filter(|(_, h)| !h.kuna_global()).map(|(high, _)| high).collect();
    let mut out = Vec::new();
    for high in highs {
        let piece = fd
            .high_name_representative(high)
            .and_then(|rep| fd.vbank().get(rep))
            .is_some_and(|v| v.is_proto_partial());
        if !piece && fd.high_has_name(high) {
            out.push((high, tally(fd, high)));
        }
    }
    out
}

/// The number each local keeps, assigned over the whole function: strongest
/// evidence first (most register and stack instances, then temporaries), so a
/// local holding a few pieces of another variable cannot take its number
/// before the variable itself. A renamed local's number goes to no one.
pub fn assign_numbers(
    fd: &Funcdata,
    tallies: &[(HighVariableId, BTreeMap<String, (usize, usize)>)],
) -> BTreeMap<HighVariableId, int4> {
    let prior = fd.kuna_prior_names();
    let angr = fd.get_arch().name_style_angr;
    let mut options: Vec<((usize, usize), int4, HighVariableId)> = Vec::new();
    for (high, counts) in tallies {
        let mut numbers: BTreeMap<int4, (usize, usize)> = BTreeMap::new();
        for (name, count) in counts {
            if let Some(i) = default_name_index(name, angr).filter(|i| !prior.vacated.contains_key(i)) {
                let total = numbers.entry(i).or_default();
                total.0 += count.0;
                total.1 += count.1;
            }
        }
        options.extend(numbers.into_iter().map(|(i, count)| (count, i, *high)));
    }
    options.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)).then(a.2.cmp(&b.2)));
    let mut assigned: BTreeMap<HighVariableId, int4> = BTreeMap::new();
    let mut taken: BTreeSet<int4> = BTreeSet::new();
    for (_, i, high) in options {
        if !assigned.contains_key(&high) && taken.insert(i) {
            assigned.insert(high, i);
        }
    }
    assigned
}

/// The allocator's view of [`PriorNames`] during one naming pass.
#[derive(Clone, Debug, Default)]
pub struct NameState {
    prior: PriorNames,
    active: bool,
    handed: BTreeSet<int4>,
    overflow: int4,
    preferred: Vec<int4>,
    targets: BTreeMap<HighVariableId, String>,
    assigned: BTreeMap<HighVariableId, int4>,
}

impl NameState {
    pub fn new(prior: PriorNames) -> NameState {
        let active = !prior.is_empty();
        let overflow = prior.end;
        NameState {
            prior,
            active,
            handed: BTreeSet::new(),
            overflow,
            preferred: Vec::new(),
            targets: BTreeMap::new(),
            assigned: BTreeMap::new(),
        }
    }

    /// Record the number each local keeps (see [`assign_numbers`]).
    pub fn set_assigned(&mut self, assigned: BTreeMap<HighVariableId, int4>) {
        self.assigned = assigned;
    }

    /// The number `high` keeps, if any.
    pub fn assigned(&self, high: HighVariableId) -> Option<int4> {
        self.assigned.get(&high).copied()
    }

    /// Record which local each name-only rename names (see [`rename_targets`]).
    pub fn set_targets(&mut self, targets: BTreeMap<HighVariableId, String>) {
        self.targets = targets;
    }

    /// The name a name-only rename gives `high`, if any.
    pub fn target(&self, high: HighVariableId) -> Option<&str> {
        self.targets.get(&high).map(String::as_str)
    }

    /// Is a previous decompile's numbering being kept?
    pub fn active(&self) -> bool {
        self.active
    }

    /// The numbers the next allocation should try first (see [`best`]).
    pub fn prefer(&mut self, numbers: Vec<int4>) {
        self.preferred = numbers;
    }

    /// Let number `i`, which a renamed local held, be handed out again: the
    /// rename named nothing (see [`orphaned`]).
    pub fn release(&mut self, i: int4) {
        self.prior.vacated.remove(&i);
    }

    /// The name a rename gave the Symbol at `(addr, size)`, if any.
    pub fn symbol_rename(&self, addr: &Address, size: int4) -> Option<&str> {
        self.prior.symbol_renames.get(&(addr.clone(), size)).map(String::as_str)
    }

    /// Put the number the Symbol at `(addr, size)` had ahead of the others.
    pub fn prefer_symbol(&mut self, addr: &Address, size: int4) {
        if let Some(&i) = self.prior.symbols.get(&(addr.clone(), size)) {
            self.preferred.insert(0, i);
        }
    }

    /// The number for the next local: a preferred one no one holds, else one
    /// past every number handed out. `free` says whether a rendered name is
    /// unclaimed by a locked Symbol.
    pub fn pick(&mut self, render: &dyn Fn(int4) -> String, free: &dyn Fn(&str) -> bool) -> int4 {
        let given = std::mem::take(&mut self.prior.given);
        let free = |name: &str| !given.contains(name) && free(name);
        let number = self.pick_free(render, &free);
        self.prior.given = given;
        number
    }

    fn pick_free(&mut self, render: &dyn Fn(int4) -> String, free: &dyn Fn(&str) -> bool) -> int4 {
        let preferred = std::mem::take(&mut self.preferred);
        let open = |i: int4, state: &NameState| !state.prior.vacated.contains_key(&i) && !state.handed.contains(&i);
        if let Some(i) = preferred.into_iter().find(|&i| open(i, self) && free(&render(i))) {
            return self.hand(i);
        }
        let mut i = self.overflow;
        while !open(i, self) || !free(&render(i)) {
            i += 1;
        }
        self.overflow = i + 1;
        self.hand(i)
    }

    fn hand(&mut self, i: int4) -> int4 {
        self.handed.insert(i);
        i
    }

    /// One past every number this pass handed out.
    pub fn end(&self) -> int4 {
        self.handed.last().map_or(self.overflow, |&i| self.overflow.max(i + 1))
    }
}

/// Note that `key` belonged to `name`; a key that belonged to two names
/// names neither.
fn record<K: Ord>(map: &mut BTreeMap<K, Option<String>>, key: K, name: &str) {
    map.entry(key)
        .and_modify(|held| {
            if held.as_deref() != Some(name) {
                *held = None;
            }
        })
        .or_insert_with(|| Some(name.to_string()));
}

/// The keys of every instance of each local the naming pass just named, with
/// its name, for [`carry`]; taken only when the drive asked for it
/// (`Architecture::kuna_record_names`), since a directive may follow.
pub fn snapshot(fd: &Funcdata) -> Vec<(String, Keys)> {
    let mut out = Vec::new();
    for (high, h) in fd.high_bank().iter() {
        if h.kuna_global() {
            continue;
        }
        if let Some(name) = h.kuna_name() {
            out.extend(instances(fd, high).into_iter().map(|keys| (name.to_string(), keys)));
        }
    }
    out
}

/// A retype carried to the next decompile as the type of the whole printed
/// variable ([`unit_retype`]); `infer` when type inference has to read it
/// ([`reaches_inference`]).
#[derive(Clone, Debug)]
pub struct UnitRetype {
    pub ct: std::rc::Rc<crate::dtype::Datatype>,
    pub infer: bool,
}

/// The unit retype of a `from` variable to `to`, when it keeps the width and
/// kind ([`retypes_as_unit`]).
pub fn unit_retype(from: &crate::dtype::Datatype, to: &std::rc::Rc<crate::dtype::Datatype>) -> Option<UnitRetype> {
    retypes_as_unit(from, to).then(|| UnitRetype { ct: to.clone(), infer: reaches_inference(to) })
}

/// Does retyping a `from` variable to `to` keep its width and kind: integer
/// to integer (the signedness, or `bool`/`char` of that width), pointer to
/// pointer, float to float? Such a retype changes how the variable is
/// declared and cast, not what the analysis merges.
pub fn retypes_as_unit(from: &crate::dtype::Datatype, to: &crate::dtype::Datatype) -> bool {
    use crate::dtype::type_metatype::*;
    let kind = |t: &crate::dtype::Datatype| match t.get_metatype() {
        TYPE_INT | TYPE_UINT | TYPE_UNKNOWN | TYPE_BOOL | TYPE_ENUM_INT | TYPE_ENUM_UINT => Some(0),
        TYPE_PTR | TYPE_PTRREL => Some(1),
        TYPE_FLOAT => Some(2),
        _ => None,
    };
    from.get_size() == to.get_size() && kind(from).is_some() && kind(from) == kind(to)
}

/// Is the directive on `sym` carried to the next decompile as a name (and a
/// unit retype), not as a Symbol: a name-only rename, or a retype that keeps
/// the printed variable's width and kind.
pub fn is_carried_as_name(fd: &Funcdata, sym: crate::database::SymbolId) -> bool {
    is_name_only(fd, sym) || fd.kuna_directive_symbols().iter().any(|d| d.symbol == sym && d.unit_type.is_some())
}

/// Lock each unit retype that drives inference ([`reaches_inference`]) on
/// every varnode defined by an operation that defined an instance of the
/// retyped variable in the previous decompile, before type inference reads
/// the function, so the whole variable is read through the type and its parts
/// still agree when the merge compares them. Only the defining operation
/// itself identifies a varnode here: before the analysis settles, the shape
/// and place of an instance also fit other varnodes, which the lock would
/// type as well.
pub fn lock_retyped(fd: &mut Funcdata) {
    let locks: Vec<(VarnodeId, std::rc::Rc<crate::dtype::Datatype>)> = {
        let prior = fd.kuna_prior_names();
        if !prior.retypes.values().any(|retype| retype.infer) {
            return;
        }
        *prior.merged_names.borrow_mut() = MergeMemo::default();
        fd.vbank()
            .iter_loc()
            .filter_map(|vn| {
                let name = prior.ops.get(&keys_of(fd, vn)?.0)?.as_ref()?;
                let ct = &prior.retypes.get(name).filter(|retype| retype.infer)?.ct;
                let v = fd.vbank().get(vn)?;
                (v.get_size() == ct.get_size() && !v.is_annotation()).then(|| (vn, ct.clone()))
            })
            .collect()
    };
    for (vn, ct) in locks {
        if let Some(v) = fd.vbank_mut().get_mut(vn) {
            v.update_type_locked(ct, true, true);
        }
    }
}

/// Under a replayed retype that drives inference ([`reaches_inference`]), did
/// `a` and `b` belong to one local of the previous decompile (`Some(true)`:
/// every instance either matches names the same one), to different ones
/// (`Some(false)`: no name in common), or is that unknown (`None`)? Inference
/// changes types, and with them the speculative merges, which compare types;
/// the merge follows the previous decompile's variables instead wherever the
/// keys say what they were.
pub fn same_variable(fd: &Funcdata, a: HighVariableId, b: HighVariableId) -> Option<bool> {
    if !replays_inference(fd) {
        return None;
    }
    let (na, nb) = (merged_names(fd, a), merged_names(fd, b));
    if na.is_empty() || nb.is_empty() {
        return None;
    }
    if na.len() == 1 && na == nb {
        return Some(true);
    }
    if na.is_disjoint(&nb) {
        return Some(false);
    }
    None
}

/// The locals `high`'s instances matched, remembered per varnode while the
/// merge asks: the merge tests a pair of highs many times over.
fn merged_names(fd: &Funcdata, high: HighVariableId) -> BTreeSet<u32> {
    let Some(h) = fd.high_bank().get(high) else { return BTreeSet::new() };
    let prior = fd.kuna_prior_names();
    let mut memo = prior.merged_names.borrow_mut();
    let MergeMemo { locals, names } = &mut *memo;
    (0..h.num_instances())
        .filter_map(|i| {
            let vn = h.get_instance(i);
            *locals.entry(vn).or_insert_with(|| {
                let name = matched_name(fd, prior, vn, &keys_of(fd, vn)?)?;
                let next = names.len() as u32;
                Some(*names.entry(name.clone()).or_insert(next))
            })
        })
        .collect()
}

/// The highs among `highs` that hold one local of the previous decompile and
/// nothing else, grouped by that local, under a replayed retype that drives
/// inference; empty otherwise. The merge by type cannot rejoin such a local
/// once inference typed its parts apart.
pub fn variable_groups(fd: &Funcdata, highs: &[HighVariableId]) -> Vec<Vec<HighVariableId>> {
    if !replays_inference(fd) {
        return Vec::new();
    }
    let current: BTreeSet<HighVariableId> = highs
        .iter()
        .filter_map(|&high| fd.high_bank().get(high).filter(|h| h.num_instances() > 0).map(|h| h.get_instance(0)))
        .filter_map(|vn| fd.vbank().get(vn).and_then(|v| v.get_high()))
        .collect();
    let mut groups: BTreeMap<u32, Vec<HighVariableId>> = BTreeMap::new();
    for high in current {
        let names = merged_names(fd, high);
        if names.len() == 1 {
            groups.entry(names.into_iter().next().unwrap()).or_default().push(high);
        }
    }
    groups.into_values().filter(|group| group.len() > 1).collect()
}

/// Does `vn` carry the type [`lock_retyped`] locked on the retyped variable?
/// Such a variable stays explicit: it was printed, and its type may let the
/// expression around it inline it, which would leave the name it was given
/// on nothing.
pub fn holds_retype(fd: &Funcdata, vn: VarnodeId) -> bool {
    let Some(v) = fd.vbank().get(vn).filter(|v| v.is_type_lock()) else { return false };
    fd.kuna_prior_names().retypes.values().any(|retype| retype.infer && std::rc::Rc::ptr_eq(&retype.ct, v.get_type()))
}

fn replays_inference(fd: &Funcdata) -> bool {
    fd.kuna_prior_names().retypes.values().any(|retype| retype.infer)
}

/// Must type inference read a unit retype to `to`: a pointer whose pointee it
/// reads through ([`reads_through`]), where inference is what turns the offsets
/// read through it into fields, elements and calls. Any other unit retype is
/// applied after the merge alone ([`retype_targets`]): what the first decompile
/// inferred still prints valid C under the new declaration.
pub fn reaches_inference(to: &crate::dtype::Datatype) -> bool {
    reads_through(to)
}

/// Does `ct` print the constants it is given in its own way: by an
/// enumeration's names, as `bool`, or as characters?
pub fn prints_literals(ct: &crate::dtype::Datatype) -> bool {
    use crate::dtype::type_metatype::*;
    ct.is_enum_type() || ct.is_char_print() || matches!(ct.get_metatype(), TYPE_ENUM_INT | TYPE_ENUM_UINT | TYPE_BOOL)
}

/// The constants of `size` bytes the variable made of `instances` meets
/// directly: copied into it, tested for equality with it, or combined with it
/// bit by bit (`v |= F_R`). Type inference would give them the variable's type;
/// applied after the merge, the retype gives it them here, and nothing further
/// away. An ordered comparison keeps its constant a number, as it printed
/// before: C compares an enumeration as an integer.
pub fn met_constants(fd: &Funcdata, instances: &[VarnodeId], size: int4) -> Vec<VarnodeId> {
    let constant = |vn: Option<VarnodeId>| {
        vn.filter(|&vn| fd.vbank().get(vn).is_some_and(|v| v.is_constant() && v.get_size() == size))
    };
    let mut out = Vec::new();
    for &vn in instances {
        let Some(v) = fd.vbank().get(vn) else { continue };
        if let Some(def) = v.get_def().and_then(|op| fd.obank().get(op)) {
            if def.code() == OpCode::CPUI_COPY {
                out.extend(constant(def.get_in(0)));
            }
        }
        for op in v.descend_iter() {
            let Some(o) = fd.obank().get(op) else { continue };
            if matches!(
                o.code(),
                OpCode::CPUI_INT_EQUAL
                    | OpCode::CPUI_INT_NOTEQUAL
                    | OpCode::CPUI_INT_OR
                    | OpCode::CPUI_INT_AND
                    | OpCode::CPUI_INT_XOR
            ) {
                out.extend((0..o.num_input()).filter_map(|slot| constant(o.get_in(slot))));
            }
        }
    }
    out.sort();
    out.dedup();
    out
}

/// Is `ct` a pointer whose pointee the analysis reads through: a structure,
/// union, array or function, or a relative pointer? Type inference is what
/// turns its offsets into fields, elements and calls.
pub fn reads_through(ct: &crate::dtype::Datatype) -> bool {
    use crate::dtype::type_metatype::*;
    if ct.is_pointer_rel() {
        return true;
    }
    match ct.get_metatype() {
        TYPE_PTR => ct.get_ptr_to().is_some_and(|to| {
            matches!(to.get_metatype(), TYPE_STRUCT | TYPE_UNION | TYPE_ARRAY | TYPE_PARTIALSTRUCT | TYPE_PARTIALUNION | TYPE_CODE)
        }),
        _ => false,
    }
}

/// The local each unit retype types: the one holding most of the retyped
/// variable's instances, as for [`rename_targets`], among the locals as wide as
/// the new type. A stack local read whole and in halves is several locals of
/// one Symbol, and the declaration is the whole one's.
pub fn retype_targets(
    fd: &Funcdata,
    tallies: &[(HighVariableId, BTreeMap<String, (usize, usize)>)],
) -> BTreeMap<HighVariableId, std::rc::Rc<crate::dtype::Datatype>> {
    let retypes = &fd.kuna_prior_names().retypes;
    directive_targets(fd, tallies)
        .into_iter()
        .filter_map(|(name, high)| retypes.get(&name).map(|retype| (high, retype.ct.clone())))
        .collect()
}

/// The local each carried rename or unit retype reaches, by the name the
/// previous decompile printed it under: the one holding most of that
/// variable's instances, as for [`rename_targets`]; for a retype, among the
/// locals as wide as its type ([`retype_targets`]).
pub fn directive_targets(
    fd: &Funcdata,
    tallies: &[(HighVariableId, BTreeMap<String, (usize, usize)>)],
) -> BTreeMap<String, HighVariableId> {
    let prior = fd.kuna_prior_names();
    let fits = |name: &str, high: HighVariableId| match prior.retypes.get(name) {
        Some(retype) => width(fd, high) == Some(retype.ct.get_size()),
        None => true,
    };
    let mut best: BTreeMap<&String, ((usize, usize), HighVariableId)> = BTreeMap::new();
    for (high, counts) in tallies {
        for (name, count) in counts {
            if (prior.renames.contains_key(name) || prior.retypes.contains_key(name)) && fits(name, *high) {
                if best.get(name).is_none_or(|(held, _)| *count > *held) {
                    best.insert(name, (*count, *high));
                }
            }
        }
    }
    best.into_iter().map(|(name, (_, high))| (name.clone(), high)).collect()
}

fn width(fd: &Funcdata, high: HighVariableId) -> Option<int4> {
    let h = fd.high_bank().get(high)?;
    (h.num_instances() > 0).then(|| fd.vbank().get(h.get_instance(0)).map(|v| v.get_size())).flatten()
}

/// Did a `name` directive rename `sym` without stating a type: a Symbol it
/// mapped over a printed local, or a stack local's Symbol it renamed? Such a
/// rename is carried as a name, not as a Symbol: a Symbol over one definition
/// of a merged variable split it, and a type lock retyped it.
pub fn is_name_only(fd: &Funcdata, sym: crate::database::SymbolId) -> bool {
    let Some(lm) = fd.get_scope_local() else { return false };
    let symbol = lm.database().symbol(sym);
    let mapped = symbol.mapentry.iter().any(|entry| !matches!(entry, crate::database::EntryRef::Dynamic(_)));
    symbol.is_name_locked()
        && !symbol.is_type_locked()
        && symbol.get_category() != crate::database::symbol_category::FUNCTION_PARAMETER
        && fd
            .kuna_directive_symbols()
            .iter()
            .any(|d| d.symbol == sym && (d.bound.is_some() || mapped))
}

/// The record the next decompile of `fd`'s function keeps: `fd`'s numbering
/// as its naming pass handed it out, plus the numbers the directives applied
/// to it since renamed. Empty when no directive touched `fd` and it kept no
/// earlier numbering, so a plain re-decompile numbers from scratch as before.
pub fn carry(fd: &Funcdata) -> PriorNames {
    let prior = fd.kuna_prior_names();
    if prior.is_empty() && fd.kuna_directive_symbols().is_empty() {
        return PriorNames::default();
    }
    let angr = fd.get_arch().name_style_angr;
    let mut next = PriorNames {
        ops: BTreeMap::new(),
        shapes: BTreeMap::new(),
        places: BTreeMap::new(),
        symbols: BTreeMap::new(),
        vacated: prior.vacated.clone(),
        given: prior.given.clone(),
        vacated_names: prior.vacated_names.clone(),
        aliases: prior.aliases.clone(),
        end: prior.end.max(fd.kuna_default_name_end()),
        renames: BTreeMap::new(),
        symbol_renames: prior.symbol_renames.clone(),
        retypes: prior
            .retypes
            .iter()
            .map(|(from, retype)| (prior.renames.get(from).unwrap_or(from).clone(), retype.clone()))
            .collect(),
        merged_names: Default::default(),
    };
    for (name, (op, shape, place)) in fd.kuna_name_snapshot().iter().cloned() {
        record(&mut next.ops, op, &name);
        record(&mut next.shapes, shape, &name);
        record(&mut next.places, place, &name);
    }
    let mut renamed: BTreeMap<String, String> =
        prior.renames.values().map(|name| (name.clone(), name.clone())).collect();
    let Some(lm) = fd.get_scope_local() else { return next };
    let directives = fd.kuna_directive_symbols();
    for (sym, addr, size, name) in lm.named_symbol_storage() {
        let name = match directives.iter().find(|d| d.symbol == sym) {
            Some(d) if !lm.database().symbol(sym).is_name_locked() => &d.printed,
            Some(_) => continue,
            None => &name,
        };
        if let Some(i) = default_name_index(name, angr) {
            next.symbols.entry((addr, size)).or_insert(i);
        }
    }
    for alias in fd.kuna_library_aliases().iter().filter(|a| a.shown) {
        if let Some(i) = default_name_index(&alias.number, angr) {
            next.aliases.insert(i, alias.alias.clone());
        }
    }
    for directive in directives {
        let symbol = lm.database().symbol(directive.symbol);
        match &directive.unit_type {
            Some(retype) => {
                next.retypes.insert(directive.printed.clone(), retype.clone());
            }
            None if directive.retyped => {
                next.retypes.remove(&directive.printed);
            }
            None => {}
        }
        if symbol.is_name_undefined() || !symbol.is_name_locked() {
            continue;
        }
        if let Some(alias) = fd.kuna_library_aliases().iter().find(|a| a.shown && a.number == directive.printed) {
            next.vacated_names.insert(alias.alias.clone());
        }
        if is_carried_as_name(fd, directive.symbol) {
            renamed.insert(directive.printed.clone(), symbol.name.clone());
            if let Some((_, addr, size, _)) =
                lm.named_symbol_storage().into_iter().find(|(sym, ..)| *sym == directive.symbol && directive.bound.is_none())
            {
                next.symbol_renames.insert((addr, size), symbol.name.clone());
            }
        }
        if !next.given.remove(&directive.printed) {
            match default_name_index(&directive.printed, angr) {
                Some(i) => {
                    next.vacated.insert(i, symbol.name.clone());
                }
                None => {
                    next.vacated_names.insert(directive.printed.clone());
                }
            }
        }
        if default_name_index(&symbol.name, angr).is_some() {
            next.given.insert(symbol.name.clone());
        }
    }
    next.renames = renamed;
    next
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_default_shapes_have_an_index() {
        assert_eq!(default_name_index("v9", true), Some(9));
        assert_eq!(default_name_index("v12", true), Some(12));
        for name in ["v", "v0", "v09", "vx", "idx", "a1", "v1_1", "iVar3"] {
            assert_eq!(default_name_index(name, true), None, "{name}");
        }
        assert_eq!(default_name_index("iVar3", false), Some(3));
        assert_eq!(default_name_index("Var3", false), Some(3));
        assert_eq!(default_name_index("v3", false), Some(3));
        assert_eq!(default_name_index("vVar3", false), Some(3));
        for name in ["myVar3", "iVar", "iVar03", "vx"] {
            assert_eq!(default_name_index(name, false), None, "{name}");
        }
    }

    #[test]
    fn a_local_keeps_its_number_and_any_other_takes_one_past_the_last() {
        let prior = PriorNames {
            vacated: [(3, "c".to_string())].into_iter().collect(),
            given: ["v7".to_string()].into_iter().collect(),
            end: 12,
            ..PriorNames::default()
        };
        let mut state = NameState::new(prior);
        let render = |i: int4| format!("v{i}");
        assert!(state.active());
        state.prefer(vec![5]);
        assert_eq!(state.pick(&render, &|_| true), 5);
        state.prefer(vec![7]);
        assert_eq!(state.pick(&render, &|_| true), 12, "a number a rename took as its name");
        state.prefer(vec![5]);
        assert_eq!(state.pick(&render, &|_| true), 13, "a number another local took");
        state.prefer(vec![3]);
        assert_eq!(state.pick(&render, &|name| name != "v14"), 15, "a renamed local's number");
        assert_eq!(state.pick(&render, &|_| true), 16, "a local the first decompile did not have");
        assert_eq!(state.end(), 17);
    }

    #[test]
    fn without_a_record_nothing_is_active() {
        assert!(!NameState::new(PriorNames::default()).active());
    }
}

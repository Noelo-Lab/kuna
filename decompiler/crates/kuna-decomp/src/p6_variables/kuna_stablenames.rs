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
//! Without a carried record the pass numbers in sequence, exactly as before.

use std::collections::{BTreeMap, BTreeSet};

use kuna_base::address::Address;
use kuna_base::space::spacetype::{IPTR_FSPEC, IPTR_INTERNAL, IPTR_IOP, IPTR_JOIN};
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::context::HighVariableId;
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
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PriorNames {
    /// The number each instance of a numbered variable had, by [`Op`]; `None`
    /// where the key named more than one number.
    pub ops: BTreeMap<Op, Option<int4>>,
    /// The same by [`Shape`].
    pub shapes: BTreeMap<Shape, Option<int4>>,
    /// The same by [`Place`].
    pub places: BTreeMap<Place, Option<int4>>,
    /// The number each numbered Symbol had, by its storage and width.
    pub symbols: BTreeMap<(Address, int4), int4>,
    /// The numbers renamed locals held, with the names they were given.
    pub vacated: BTreeMap<int4, String>,
    /// The default-shaped names a rename gave a local (`name v3 v7`).
    pub given: BTreeSet<String>,
    /// One past the last number handed out.
    pub end: int4,
}

impl PriorNames {
    pub fn is_empty(&self) -> bool {
        self.ops.is_empty() && self.symbols.is_empty() && self.vacated.is_empty() && self.given.is_empty()
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

/// The [`Keys`] of each non-constant instance of `high`.
fn instances(fd: &Funcdata, high: HighVariableId) -> Vec<Keys> {
    let Some(h) = fd.high_bank().get(high) else { return Vec::new() };
    (0..h.num_instances())
        .map(|i| h.get_instance(i))
        .filter_map(|vn| {
            let v = fd.vbank().get(vn).filter(|v| !v.is_constant())?;
            let storage = stable(v.get_addr());
            let place = (storage.clone(), v.get_size(), fd.vn_use_point(vn));
            let keys = match v.get_def().filter(|_| v.is_written()).and_then(|op| fd.obank().get(op)) {
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
            };
            Some(keys)
        })
        .collect()
}

/// The number most of `high`'s instances had (the lowest on a tie), each
/// matched by its most precise key that names one; `None` without a carried
/// record or a match. Instances in registers and on the stack decide before
/// temporaries: a temporary merges wherever its copies lead, so after a rename
/// changes the merge it can sit in a different variable, while the storage
/// the declaration lists is what the reader knows the variable by. A renamed
/// local's number counts only when nothing else matches: a variable that took
/// in part of the renamed one is still the variable it was.
pub fn best(fd: &Funcdata, high: HighVariableId) -> Option<int4> {
    let prior = fd.kuna_prior_names();
    if prior.ops.is_empty() {
        return None;
    }
    let mut counts: BTreeMap<int4, (usize, usize)> = BTreeMap::new();
    for (op, shape, place) in instances(fd, high) {
        let number = prior.ops.get(&op).copied().flatten()
            .or_else(|| prior.shapes.get(&shape).copied().flatten())
            .or_else(|| prior.places.get(&place).copied().flatten());
        if let Some(i) = number {
            let temporary = op.2.get_space().is_some_and(|space| space.get_type() == IPTR_INTERNAL);
            let count = counts.entry(i).or_default();
            if temporary {
                count.1 += 1;
            } else {
                count.0 += 1;
            }
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

/// The allocator's view of [`PriorNames`] during one naming pass.
#[derive(Clone, Debug, Default)]
pub struct NameState {
    prior: PriorNames,
    active: bool,
    handed: BTreeSet<int4>,
    overflow: int4,
    preferred: Vec<int4>,
}

impl NameState {
    pub fn new(prior: PriorNames) -> NameState {
        let active = !prior.is_empty();
        let overflow = prior.end;
        NameState { prior, active, handed: BTreeSet::new(), overflow, preferred: Vec::new() }
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

/// Note that `key` names number `i`; a key that names two numbers names none.
fn record<K: Ord>(map: &mut BTreeMap<K, Option<int4>>, key: K, i: int4) {
    map.entry(key).and_modify(|n| *n = n.filter(|&n| n == i)).or_insert(Some(i));
}

/// The keys of every instance of each local the naming pass just numbered,
/// for [`carry`]; taken only when the drive asked for it
/// (`Architecture::kuna_record_names`), since a directive may follow.
pub fn snapshot(fd: &Funcdata) -> Vec<(int4, Keys)> {
    let angr = fd.get_arch().name_style_angr;
    let mut out = Vec::new();
    for (high, h) in fd.high_bank().iter() {
        if h.kuna_global() {
            continue;
        }
        if let Some(i) = h.kuna_name().and_then(|name| default_name_index(name, angr)) {
            out.extend(instances(fd, high).into_iter().map(|keys| (i, keys)));
        }
    }
    out
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
        end: prior.end.max(fd.kuna_default_name_end()),
    };
    for (i, (op, shape, place)) in fd.kuna_name_snapshot().iter().cloned() {
        record(&mut next.ops, op, i);
        record(&mut next.shapes, shape, i);
        record(&mut next.places, place, i);
    }
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
    for directive in directives {
        let symbol = lm.database().symbol(directive.symbol);
        if symbol.is_name_undefined() || !symbol.is_name_locked() {
            continue;
        }
        if !next.given.remove(&directive.printed) {
            if let Some(i) = default_name_index(&directive.printed, angr) {
                next.vacated.insert(i, symbol.name.clone());
            }
        }
        if default_name_index(&symbol.name, angr).is_some() {
            next.given.insert(symbol.name.clone());
        }
    }
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

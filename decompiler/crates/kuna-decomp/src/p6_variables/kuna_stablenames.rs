//! Default local names stay with their variables when a directive is replayed.
//!
//! The naming pass numbers unnamed locals in location order (`v1`, `v2`, ... or
//! `iVar1` under Ghidra names). A `name`/`type` directive is read against the
//! first decompile's output and applied by a second, and the second numbered
//! from scratch: the renamed local took no number, so every later local moved
//! down one, and a following directive naming `v9` reached a different
//! variable than the caller saw.
//!
//! [`carry`] records the first decompile's numbering by identity: the pieces
//! of each numbered variable (storage, width and use point of every instance)
//! and the storage of each numbered Symbol. The next decompile of the function
//! then gives each local the number its pieces had (`varmap.rs
//! (ScopeLocal::next_default_name)`):
//!
//! - a renamed local's number stays unused, so is skipped by every other local,
//!   including the part of a merged variable a rename left unbound;
//! - a local whose number a rename took as its new name (`name v3 v7` gives
//!   `v7`) takes the number a renamed local left, else a fresh one;
//! - a local the first decompile did not have (a split, a retype that changes
//!   the analysis) takes a number past the first decompile's last, and a
//!   variable that is gone leaves its number unused.
//!
//! Without a carried record the pass numbers in sequence, exactly as before.

use std::collections::{BTreeMap, BTreeSet};

use kuna_base::address::Address;
use kuna_base::space::spacetype::{IPTR_INTERNAL, IPTR_JOIN};
use kuna_base::types::int4;

use crate::context::HighVariableId;
use crate::funcdata::Funcdata;

/// A piece of a variable: its storage, width and use point (the defining
/// instruction, or the entry minus one for an input). A temporary or a JOIN is
/// renumbered by every decompile, so only its space is kept.
pub type Piece = (Address, int4, Address);

/// One decompile's default numbering, handed to the next decompile of the
/// same function.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PriorNames {
    /// The number each piece of a numbered variable had.
    pub pieces: BTreeMap<Piece, int4>,
    /// The number each numbered Symbol had, by its storage and width.
    pub symbols: BTreeMap<(Address, int4), int4>,
    /// The numbers renamed locals held.
    pub vacated: BTreeSet<int4>,
    /// The default-shaped names a rename gave a local (`name v3 v7`).
    pub given: BTreeSet<String>,
    /// One past the last number handed out.
    pub end: int4,
}

impl PriorNames {
    pub fn is_empty(&self) -> bool {
        self.pieces.is_empty() && self.symbols.is_empty() && self.vacated.is_empty() && self.given.is_empty()
    }
}

/// The index of a default local name, `v9` -> 9 (or `iVar9` -> 9 under Ghidra
/// names); `None` for any other name.
pub fn default_name_index(name: &str, angr: bool) -> Option<int4> {
    let digits = if angr {
        name.strip_prefix('v')?
    } else {
        let at = name.find("Var")?;
        if name[..at].chars().count() > 1 {
            return None;
        }
        &name[at + 3..]
    };
    if digits.is_empty() || digits.starts_with('0') || !digits.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    digits.parse().ok()
}

/// The pieces of `high` (its non-constant instances).
fn pieces(fd: &Funcdata, high: HighVariableId) -> Vec<Piece> {
    let Some(h) = fd.high_bank().get(high) else { return Vec::new() };
    (0..h.num_instances())
        .map(|i| h.get_instance(i))
        .filter_map(|vn| {
            let v = fd.vbank().get(vn).filter(|v| !v.is_constant())?;
            let addr = v.get_addr();
            let space = addr.get_space()?;
            let storage = match space.get_type() {
                IPTR_INTERNAL | IPTR_JOIN => Address::new(std::rc::Rc::clone(space), 0),
                _ => addr.clone(),
            };
            Some((storage, v.get_size(), fd.vn_use_point(vn)))
        })
        .collect()
}

/// The numbers `high`'s pieces had, most-shared first; empty without a
/// carried record.
pub fn candidates(fd: &Funcdata, high: HighVariableId) -> Vec<int4> {
    let prior = fd.kuna_prior_names();
    if prior.pieces.is_empty() {
        return Vec::new();
    }
    let mut counts: BTreeMap<int4, usize> = BTreeMap::new();
    for piece in pieces(fd, high) {
        if let Some(&i) = prior.pieces.get(&piece) {
            *counts.entry(i).or_default() += 1;
        }
    }
    let mut out: Vec<(int4, usize)> = counts.into_iter().collect();
    out.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    out.into_iter().map(|(i, _)| i).collect()
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

    /// The numbers the next allocation should try first (see [`candidates`]).
    pub fn prefer(&mut self, numbers: Vec<int4>) {
        self.preferred = numbers;
    }

    /// Put the number the Symbol at `(addr, size)` had ahead of the others.
    pub fn prefer_symbol(&mut self, addr: &Address, size: int4) {
        if let Some(&i) = self.prior.symbols.get(&(addr.clone(), size)) {
            self.preferred.insert(0, i);
        }
    }

    /// Withdraw the preference without allocating.
    pub fn clear_preferred(&mut self) {
        self.preferred.clear();
    }

    /// The number for the next local: a preferred one no one holds; else,
    /// when a rename took a preferred number's name, one a renamed local left;
    /// else one past every number handed out. `free` says whether a rendered
    /// name is unclaimed by a locked Symbol.
    pub fn pick(&mut self, render: &dyn Fn(int4) -> String, free: &dyn Fn(&str) -> bool) -> int4 {
        let given = std::mem::take(&mut self.prior.given);
        let free = |name: &str| !given.contains(name) && free(name);
        let number = self.pick_free(render, &free);
        self.prior.given = given;
        number
    }

    fn pick_free(&mut self, render: &dyn Fn(int4) -> String, free: &dyn Fn(&str) -> bool) -> int4 {
        let preferred = std::mem::take(&mut self.preferred);
        let mut displaced = false;
        for i in preferred {
            if self.prior.vacated.contains(&i) || self.handed.contains(&i) {
                continue;
            }
            if free(&render(i)) {
                return self.hand(i);
            }
            displaced = true;
        }
        if displaced {
            let vacant = self.prior.vacated.iter().copied().find(|i| !self.handed.contains(i) && free(&render(*i)));
            if let Some(i) = vacant {
                return self.hand(i);
            }
        }
        let mut i = self.overflow;
        while self.prior.vacated.contains(&i) || self.handed.contains(&i) || !free(&render(i)) {
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

/// The record the next decompile of `fd`'s function keeps: `fd`'s numbering
/// as printed, plus the numbers the directives applied to it since renamed.
/// Empty when no directive touched `fd` and it kept no earlier numbering, so a
/// plain re-decompile numbers from scratch as before.
pub fn carry(fd: &Funcdata) -> PriorNames {
    let prior = fd.kuna_prior_names();
    if prior.is_empty() && fd.kuna_directive_symbols().is_empty() {
        return PriorNames::default();
    }
    let angr = fd.get_arch().name_style_angr;
    let mut next = PriorNames {
        pieces: BTreeMap::new(),
        symbols: BTreeMap::new(),
        vacated: prior.vacated.clone(),
        given: prior.given.clone(),
        end: prior.end.max(fd.kuna_default_name_end()),
    };
    for (high, h) in fd.high_bank().iter() {
        if h.kuna_global() {
            continue;
        }
        let Some(i) = h.kuna_name().and_then(|name| default_name_index(name, angr)) else { continue };
        for piece in pieces(fd, high) {
            next.pieces.entry(piece).or_insert(i);
        }
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
                next.vacated.insert(i);
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
        for name in ["myVar3", "v3", "iVar", "iVar03"] {
            assert_eq!(default_name_index(name, false), None, "{name}");
        }
    }

    #[test]
    fn a_local_keeps_its_number_and_a_displaced_one_takes_a_vacated_number() {
        let prior = PriorNames {
            vacated: [3].into_iter().collect(),
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
        assert_eq!(state.pick(&render, &|_| true), 3);
        state.prefer(vec![5]);
        assert_eq!(state.pick(&render, &|_| true), 12, "a number another local took");
        state.prefer(vec![3]);
        assert_eq!(state.pick(&render, &|name| name != "v13"), 14, "a renamed local's number");
        assert_eq!(state.pick(&render, &|_| true), 15, "a local the first decompile did not have");
        assert_eq!(state.end(), 16);
    }

    #[test]
    fn without_a_record_nothing_is_active() {
        assert!(!NameState::new(PriorNames::default()).active());
    }
}

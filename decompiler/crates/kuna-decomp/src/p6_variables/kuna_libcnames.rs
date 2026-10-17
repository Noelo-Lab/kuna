//! (kuna `libcnames`) Name locals from the library calls they meet.
//!
//! A prototype the analysis parks from a built-in library table carries the
//! library's own names for its parameters and, where the result has a
//! conventional name, for its return value ([`LibraryNames`]). With the option
//! on, a local the naming pass numbered prints the name of the strongest
//! evidence it has instead of its `vN`:
//!
//! 1. the value a library call returns (`fd = open(...)`), counted only when
//!    every library result the local holds has the same name;
//! 2. a library argument it is passed whole as (`read(fd, ...)`), a name
//!    describing what the value is;
//! 3. the same, for a name that only describes the slot's role in that one call
//!    (`dest`/`src`, `s1`/`s2`, ...), which counts only when every role name
//!    the local receives is the same one.
//!
//! Evidence follows a value through the copies and casts the printer folds
//! into the statement, and through nothing else. Within a kind the earliest
//! call decides, then the lowest slot. A name equal to a type the printed code
//! can mention or to a C or Rust keyword is never used.
//!
//! The library name is a second name for a numbered local, not a replacement
//! for its number ([`LibraryAlias`]). The naming pass numbers every local
//! exactly as it does with the option off, records that numbering for a
//! directive replay, and only then gives each candidate its library name
//! ([`apply`]), so `v4` is the same variable in every mode, a directive may
//! name the local by either name, and a replay identifies it by its number. A
//! clash takes `_2`, `_3`, ... in the order of the evidence, and no spelling
//! repeats a name another variable, the scope or a callee holds. A name a
//! directive or the binary chose stays, even when it is shaped like a number
//! (`name fd v9`). Under a
//! carried naming record a local keeps the spelling it printed; a spelling a
//! rename took or vacated is handed to no other local, and a local whose
//! spelling a rename took prints its number.

use std::collections::{BTreeMap, BTreeSet};

use kuna_base::address::Address;
use kuna_num::opcodes::OpCode;

use crate::context::{HighVariableId, VarnodeId};
use crate::funcdata::Funcdata;

/// A library function's own names for its parameters, in slot order (an
/// empty string for a slot its declaration leaves unnamed), and for the value
/// it returns, where that value has a conventional name.
#[derive(Debug, PartialEq, Eq)]
pub struct LibraryNames {
    pub params: &'static [&'static str],
    pub ret: Option<&'static str>,
}

/// Parameter names that say which side of one call a value is on rather than
/// what it is.
const ROLE_NAMES: &[&str] = &[
    "act", "dest", "dst", "e1", "e2", "fdin", "fdout", "from", "fromfd", "haystack", "infd", "needle", "new",
    "newfd", "newmask", "oact", "old", "oldfd", "oldmask", "oset", "outfd", "s1", "s2", "set", "src", "to", "tofd",
    "x", "y",
];

/// How far evidence follows a value through folded copies and casts.
const FOLD_DEPTH: usize = 4;

/// One piece of evidence for a local: its kind (0 a return value, 1 an
/// argument named for what it is, 2 a role name), the call (instruction,
/// creation time), the slot, and the name.
type Evidence = (u8, (Address, u32), i32, &'static str);

fn is_role(name: &str) -> bool {
    ROLE_NAMES.binary_search(&name).is_ok()
}

/// The value `vn` names when it is printed folded into a statement: through
/// the implied copy or cast it feeds (`x = (T)call()`), else itself.
fn folded_into(fd: &Funcdata, mut vn: VarnodeId) -> VarnodeId {
    for _ in 0..FOLD_DEPTH {
        let Some(v) = fd.vbank().get(vn) else { break };
        if !v.is_implied() {
            break;
        }
        let Some(op) = fd.lone_descend(vn).and_then(|op| fd.obank().get(op)) else { break };
        if !matches!(op.code(), OpCode::CPUI_COPY | OpCode::CPUI_CAST) {
            break;
        }
        let Some(out) = op.get_out() else { break };
        vn = out;
    }
    vn
}

/// The value an argument `vn` prints as: through the implied copies and casts
/// that write it (`call((T)x)`), else itself.
fn folded_from(fd: &Funcdata, mut vn: VarnodeId) -> VarnodeId {
    for _ in 0..FOLD_DEPTH {
        let Some(v) = fd.vbank().get(vn) else { break };
        if !(v.is_implied() && v.is_written()) {
            break;
        }
        let Some(op) = v.get_def().and_then(|op| fd.obank().get(op)) else { break };
        if !matches!(op.code(), OpCode::CPUI_COPY | OpCode::CPUI_CAST) {
            break;
        }
        let Some(input) = op.get_in(0) else { break };
        vn = input;
    }
    vn
}

/// The local `vn` is an instance of, when it is one this pass may name: a
/// printed, unmerged-by-speculation variable that is not a function input,
/// a global or a constant.
fn local_of(fd: &mut Funcdata, vn: VarnodeId) -> Option<HighVariableId> {
    let v = fd.vbank().get(vn)?;
    if v.is_constant() || v.is_input() || v.is_persist() || v.is_implied() || v.is_free() {
        return None;
    }
    let high = v.get_high()?;
    let h = fd.high_bank().get(high)?;
    if h.kuna_global() || h.kuna_name().is_some() || h.get_num_merge_classes() > 1 {
        return None;
    }
    let rep = fd.high_name_representative(high)?;
    let r = fd.vbank().get(rep)?;
    if r.is_input() || r.is_free() || r.is_proto_partial() {
        return None;
    }
    Some(high)
}

/// Every piece of library evidence in `fd`, by local.
fn evidence(fd: &mut Funcdata) -> BTreeMap<HighVariableId, Vec<Evidence>> {
    let mut found: BTreeMap<HighVariableId, Vec<Evidence>> = BTreeMap::new();
    for i in 0..fd.num_calls() {
        let fc = fd.get_call_specs(i);
        let Some(names) = fd.get_arch().callee_library_names(fc.get_entry_address()) else { continue };
        if !fc.proto().is_input_locked() {
            continue;
        }
        let op = fc.get_op();
        let sizes: Vec<Option<i32>> =
            (0..fc.proto().num_params()).map(|j| fc.proto().get_param(j).map(|p| p.get_size())).collect();
        let Some(o) = fd.obank().get(op) else { continue };
        let at = (o.get_addr().clone(), o.get_time());
        let out = o.get_out();
        let args: Vec<Option<VarnodeId>> = (1..o.num_input()).map(|slot| o.get_in(slot)).collect();
        if let (Some(ret), Some(out)) = (names.ret, out) {
            let vn = folded_into(fd, out);
            if let Some(high) = local_of(fd, vn) {
                found.entry(high).or_default().push((0, at.clone(), -1, ret));
            }
        }
        for (j, name) in names.params.iter().enumerate() {
            let (Some(Some(arg)), Some(Some(size))) = (args.get(j), sizes.get(j)) else { continue };
            if name.is_empty() || fd.vbank().get(*arg).map(|v| v.get_size()) != Some(*size) {
                continue;
            }
            let vn = folded_from(fd, *arg);
            if let Some(high) = local_of(fd, vn) {
                let kind = if is_role(name) { 2 } else { 1 };
                found.entry(high).or_default().push((kind, at.clone(), j as i32, name));
            }
        }
    }
    found
}

/// The evidence that names a local: the strongest kind, then the earliest
/// call, then the lowest slot. Results that disagree name nothing, and nor do
/// role names that disagree; `usable` drops a name the printed code cannot
/// hold (a type name, a keyword).
fn decide(mut found: Vec<Evidence>, usable: &dyn Fn(&str) -> bool) -> Option<Evidence> {
    found.retain(|e| usable(e.3));
    for kind in [0, 2] {
        let names: BTreeSet<&str> = found.iter().filter(|e| e.0 == kind).map(|e| e.3).collect();
        if names.len() > 1 {
            found.retain(|e| e.0 != kind);
        }
    }
    found.into_iter().min_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)).then(a.2.cmp(&b.2)))
}

/// `name` and its numbered spellings, `name`, `name_2`, `name_3`, ...
fn spelling(name: &str, k: usize) -> String {
    if k <= 1 { name.to_string() } else { format!("{name}_{k}") }
}

/// Is `candidate` one of `name`'s spellings?
fn spells(candidate: &str, name: &str) -> bool {
    candidate == name
        || candidate.strip_prefix(name).and_then(|rest| rest.strip_prefix('_')).is_some_and(|k| {
            !k.starts_with('0') && k.parse::<usize>().is_ok_and(|k| k >= 2)
        })
}

/// C and Rust keywords, and the C names a declaration may not shadow.
const RESERVED: &[&str] = &[
    "Self", "_Bool", "_Complex", "abstract", "as", "async", "auto", "await", "become", "bool", "box", "break",
    "case", "char", "const", "continue", "crate", "default", "do", "double", "dyn", "else", "enum", "extern",
    "false", "final", "float", "fn", "for", "goto", "if", "impl", "in", "inline", "int", "let", "long", "loop",
    "macro", "main", "match", "mod", "move", "mut", "override", "priv", "pub", "ref", "register", "restrict",
    "return", "self", "short", "signed", "sizeof", "static", "struct", "super", "switch", "trait", "true", "try",
    "type", "typedef", "typeof", "union", "unsafe", "unsigned", "unsized", "use", "virtual", "void", "volatile",
    "where", "while", "yield",
];

/// May a local be named `name`: not a keyword, and not a type the printed
/// code could mention (`tm`, `FILE`, a DWARF typedef).
fn usable(fd: &Funcdata, name: &str) -> bool {
    RESERVED.binary_search(&name).is_err()
        && fd.get_arch().types().is_none_or(|types| !matches!(types.find_by_name(name), Ok(Some(_))))
}

/// A numbered local that may print a library name: the local, the library
/// name, and the evidence's call and slot (the order clashes are numbered in).
#[derive(Clone, Debug)]
pub struct Candidate {
    high: HighVariableId,
    base: &'static str,
    at: (Address, u32),
    slot: i32,
}

/// A library name a numbered local answers to.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LibraryAlias {
    /// The local.
    pub high: HighVariableId,
    /// The default name it is numbered under (`v3`): what a replay record and
    /// a directive identify it by.
    pub number: String,
    /// The library name.
    pub alias: String,
    /// Whether the output prints it (`option libcnames`). When it does not, a
    /// directive naming the library name still reaches the local.
    pub shown: bool,
}

/// The library evidence of every local, decided before the naming walk: when
/// the option is on, or when a directive may name a local by its library name
/// (a naming record is being taken or replayed).
pub fn candidates(fd: &mut Funcdata) -> Vec<Candidate> {
    if !fd.get_arch().libc_names && !fd.kuna_record_names() && fd.kuna_prior_names().is_empty() {
        return Vec::new();
    }
    let found = evidence(fd);
    let mut out: Vec<Candidate> = found
        .into_iter()
        .filter_map(|(high, found)| {
            decide(found, &|name| usable(fd, name)).map(|(_, at, slot, base)| Candidate { high, base, at, slot })
        })
        .collect();
    out.sort_by(|a, b| a.at.cmp(&b.at).then(a.slot.cmp(&b.slot)).then(a.high.cmp(&b.high)));
    out
}

/// Give each candidate still printed under its number its library name, after
/// the walk has numbered every local and recorded that numbering. Every
/// spelling is unique against every name a variable or the scope holds, every
/// global the program names, the callees and the function itself; under a
/// carried record a local keeps the spelling it printed, and a spelling a
/// rename took or vacated, or another local printed, goes to no one else.
pub fn apply(fd: &mut Funcdata, candidates: &[Candidate]) {
    let shown = fd.get_arch().libc_names;
    let angr = fd.get_arch().name_style_angr;
    let prior = fd.kuna_prior_names().clone();
    let replay = !prior.is_empty();
    let mut taken: BTreeSet<String> =
        fd.high_bank().iter().filter_map(|(_, h)| h.kuna_name().map(str::to_string)).collect();
    for i in 0..fd.num_calls() {
        taken.insert(fd.get_call_specs(i).get_name().to_string());
    }
    taken.insert(fd.get_display_name().to_string());
    taken.extend(prior.renames.keys().cloned());
    taken.extend(prior.renames.values().cloned());
    taken.extend(prior.vacated_names.iter().cloned());
    let printed_before: BTreeSet<String> = prior.aliases.values().cloned().collect();
    let globals = fd.get_arch().global_name_scope();
    let free = |fd: &Funcdata, taken: &BTreeSet<String>, name: &str| {
        !taken.contains(name)
            && usable(fd, name)
            && fd.get_scope_local().is_none_or(|lm| !lm.local_name_used(name))
            && globals.as_ref().is_none_or(|g| !g.has_symbol_name(name))
    };
    let mut out: Vec<LibraryAlias> = Vec::new();
    for c in candidates {
        let Some(number) = fd.high_bank().get(c.high).and_then(|h| h.kuna_name()).map(str::to_string) else {
            continue;
        };
        let Some(index) = crate::kuna_stablenames::default_name_index(&number, angr) else { continue };
        if out.iter().any(|a| a.number == number) || chosen(fd, &prior, &number) {
            continue;
        }
        let before = crate::kuna_stablenames::best(fd, c.high).unwrap_or(index);
        let alias = match prior.aliases.get(&before) {
            Some(before) if spells(before, c.base) => Some(before.clone()).filter(|s| free(fd, &taken, s)),
            _ => (1..)
                .map(|k| spelling(c.base, k))
                .find(|s| free(fd, &taken, s) && !(replay && printed_before.contains(s))),
        };
        let Some(alias) = alias else { continue };
        taken.insert(alias.clone());
        out.push(LibraryAlias { high: c.high, number, alias, shown });
    }
    if shown {
        let rename: BTreeMap<String, String> = out.iter().map(|a| (a.number.clone(), a.alias.clone())).collect();
        let highs: Vec<HighVariableId> = fd.high_bank().iter().map(|(id, _)| id).collect();
        for high in highs {
            let alias = fd.high_bank().get(high).and_then(|h| h.kuna_name()).and_then(|n| rename.get(n)).cloned();
            if let (Some(alias), Some(h)) = (alias, fd.high_bank_mut().get_mut(high)) {
                h.set_kuna_name(alias);
            }
        }
    }
    fd.set_kuna_library_aliases(out);
}

/// Did a directive or the binary choose `name`, though it is shaped like a
/// number (`name fd v9`, a DWARF local `v1`): a rename gave it, or a
/// name-locked Symbol of the scope holds it. Such a local keeps it.
fn chosen(fd: &Funcdata, prior: &crate::kuna_stablenames::PriorNames, name: &str) -> bool {
    prior.renames.values().any(|given| given == name)
        || prior.given.contains(name)
        || fd.get_scope_local().is_some_and(|lm| {
            lm.query_by_name(name).into_iter().any(|sym| lm.database().symbol(sym).is_name_locked())
        })
}

/// The name a local is printed under, translated to the number a directive
/// replay identifies it by: `fd` reads as the `v3` it was numbered, whether or
/// not the output shows the library name. `None` when no library name is
/// `name`.
pub fn number_for(fd: &Funcdata, name: &str) -> Option<String> {
    fd.kuna_library_aliases().iter().find(|a| a.alias == name).map(|a| a.number.clone())
}

/// The number a local printed under its library name answers to; `None` for
/// every other local.
pub fn shown_number(fd: &Funcdata, high: HighVariableId) -> Option<&str> {
    fd.kuna_library_aliases().iter().find(|a| a.high == high && a.shown).map(|a| a.number.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn role_names_are_sorted_for_lookup() {
        assert!(ROLE_NAMES.windows(2).all(|w| w[0] < w[1]));
    }

    #[test]
    fn spellings_number_from_two() {
        assert_eq!(spelling("fd", 1), "fd");
        assert_eq!(spelling("fd", 2), "fd_2");
        assert!(spells("fd", "fd") && spells("fd_2", "fd") && spells("fd_13", "fd"));
        for other in ["fd_1", "fd_02", "fd_", "fdx", "fd_x", "f"] {
            assert!(!spells(other, "fd"), "{other}");
        }
    }

    #[test]
    fn reserved_names_are_sorted_for_lookup() {
        assert!(RESERVED.windows(2).all(|w| w[0] < w[1]));
    }

    #[test]
    fn a_return_value_outranks_an_argument_and_the_earliest_call_wins() {
        let at = |o: u64| (Address::new_invalid(), o as u32);
        let any = |_: &str| true;
        let e = decide(vec![(1, at(1), 0, "size"), (0, at(5), -1, "len")], &any).unwrap();
        assert_eq!(e.3, "len");
        let e = decide(vec![(1, at(7), 0, "fd"), (1, at(3), 2, "stream")], &any).unwrap();
        assert_eq!(e.3, "stream");
    }

    #[test]
    fn disagreeing_role_names_or_results_name_nothing() {
        let at = |o: u64| (Address::new_invalid(), o as u32);
        let any = |_: &str| true;
        assert!(decide(vec![(2, at(1), 0, "dest"), (2, at(2), 1, "src")], &any).is_none());
        let e = decide(vec![(2, at(1), 0, "s1"), (2, at(2), 1, "s2"), (1, at(3), 0, "s")], &any).unwrap();
        assert_eq!(e.3, "s");
        let e = decide(vec![(2, at(1), 0, "dest"), (2, at(4), 0, "dest")], &any).unwrap();
        assert_eq!(e.3, "dest");
        assert!(decide(vec![(0, at(1), -1, "len"), (0, at(2), -1, "pid")], &any).is_none());
        let e = decide(vec![(0, at(1), -1, "len"), (0, at(2), -1, "pid"), (1, at(3), 0, "n")], &any).unwrap();
        assert_eq!(e.3, "n");
    }

    #[test]
    fn an_unusable_name_gives_way_to_the_next_evidence() {
        let at = |o: u64| (Address::new_invalid(), o as u32);
        let not_tm = |name: &str| name != "tm";
        let e = decide(vec![(0, at(1), -1, "tm"), (1, at(2), 0, "tp")], &not_tm).unwrap();
        assert_eq!(e.3, "tp");
        assert!(decide(vec![(0, at(1), -1, "tm")], &not_tm).is_none());
    }
}

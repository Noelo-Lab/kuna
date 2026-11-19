//! (kuna `itaniumrtti on`) The per-class vtable facts the typed-vtable commit
//! installs ([`kuna_decomp::kuna_vtabletypes`]).
//!
//! Built by the Itanium recovery pass ([`super::kuna_itaniumrtti`]) from the
//! class graph it recovers. For each polymorphic class it reports
//! the primary vtable's slots, and for each slot the function the class holds
//! there plus every override: the same slot of each derived class's sub-vtable
//! for the class's subobject (the primary one for a base at offset 0, the
//! secondary one at the base's displacement otherwise). A slot is named after
//! the method its symbols demangle to; a destructor slot is `dtor` or
//! `deleting_dtor`, and a slot no symbol names is `vfn_<i>`.
//!
//! It also reports every sub-vtable's slot array, the read-only words the
//! commit lets a load fold to its constant.

use std::collections::{HashMap, HashSet};

use object::read::{Object, ObjectSymbol};
use object::SymbolKind;

use kuna_decomp::kuna_vtabletypes::{VtableClass, VtableSlot};

use crate::demangle::demangle_raw;

use super::kuna_itaniumrtti::{SubVtable, TypeInfo};
use super::refkind::RefKind;

/// The typed-vtable facts of a recovered Itanium class graph.
pub(super) fn vtable_facts(file: &object::File, typeinfos: &[TypeInfo], subs: &[SubVtable], rk: &RefKind) -> VtableFacts {
    facts(typeinfos, subs, rk, &method_names(file))
}

/// The typed-vtable facts: the classes to type and the vtable slot words, as
/// inclusive `[first, last]` byte ranges.
#[derive(Clone, Debug, Default)]
pub struct VtableFacts {
    pub classes: Vec<VtableClass>,
    pub slot_ranges: Vec<(u64, u64)>,
}

fn facts(typeinfos: &[TypeInfo], subs: &[SubVtable], rk: &RefKind, names: &HashMap<u64, String>) -> VtableFacts {
    let ptr = rk.ptr_size() as u64;
    let by_addr: HashMap<u64, &TypeInfo> = typeinfos.iter().map(|t| (t.addr, t)).collect();
    let mut slot_ranges: Vec<(u64, u64)> = subs
        .iter()
        .filter(|s| !s.slots.is_empty())
        .map(|s| (s.first_slot, s.first_slot + s.slots.len() as u64 * ptr - 1))
        .collect();
    slot_ranges.sort_unstable();
    slot_ranges.dedup();

    // The sub-vtable each class keeps for the subobject at each displacement.
    let mut subvtables: HashMap<(u64, i64), Vec<&[Option<u64>]>> = HashMap::new();
    for s in subs {
        subvtables.entry((s.typeinfo, -s.offset_to_top)).or_default().push(&s.slots);
    }
    // Every class's (ancestor, displacement) subobjects.
    let mut derived: HashMap<u64, Vec<(u64, i64)>> = HashMap::new();
    for t in typeinfos {
        for (base, disp) in subobjects(t.addr, &by_addr) {
            derived.entry(base).or_default().push((t.addr, disp));
        }
    }

    let mut leaf_count: HashMap<String, usize> = HashMap::new();
    for t in typeinfos {
        *leaf_count.entry(struct_name(&t.demangled)).or_default() += 1;
    }

    let mut classes = Vec::new();
    for t in typeinfos {
        let class = struct_name(&t.demangled);
        if class.is_empty() || leaf_count[&class] > 1 {
            continue;
        }
        let own = subvtables.get(&(t.addr, 0)).map(Vec::as_slice).unwrap_or(&[]);
        let mut views: Vec<&[Option<u64>]> = own.to_vec();
        for &(d, disp) in derived.get(&t.addr).map(Vec::as_slice).unwrap_or(&[]) {
            views.extend(subvtables.get(&(d, disp)).into_iter().flatten());
        }
        // A class the image never instantiates (an abstract base) has no vtable
        // of its own; its slots are the prefix every derived view shares.
        let width = if own.is_empty() {
            views.iter().map(|s| s.len()).min().unwrap_or(0)
        } else {
            own.iter().map(|s| s.len()).max().unwrap_or(0)
        };
        if width == 0 {
            continue;
        }
        let mut used: HashSet<String> = HashSet::new();
        let mut slots = Vec::with_capacity(width);
        for i in 0..width {
            let mut targets: Vec<u64> = Vec::new();
            for v in &views {
                if let Some(Some(f)) = v.get(i) {
                    if !targets.contains(f) {
                        targets.push(*f);
                    }
                }
            }
            let name = targets
                .iter()
                .find_map(|f| names.get(f))
                .filter(|n| !used.contains(*n))
                .cloned()
                .unwrap_or_else(|| format!("vfn_{i}"));
            used.insert(name.clone());
            slots.push(VtableSlot { name, targets });
        }
        // A virtual base's entry holds the (negative) offset of its vbase-offset
        // word, not a displacement: no structure can hold it at a fixed offset.
        let bases = t
            .bases
            .iter()
            .filter(|&&(_, disp)| disp > 0)
            .filter_map(|&(b, disp)| by_addr.get(&b).map(|bt| (struct_name(&bt.demangled), disp)))
            .filter(|(name, _)| !name.is_empty() && leaf_count.get(name) == Some(&1))
            .collect();
        classes.push(VtableClass { class, slots, bases, size: 0 });
    }
    VtableFacts { classes, slot_ranges }
}

/// Every transitive base of `addr` with its displacement in the object.
/// Cycle-safe (a malformed image could point a base back at its derived class).
fn subobjects(addr: u64, by_addr: &HashMap<u64, &TypeInfo>) -> Vec<(u64, i64)> {
    let mut out: Vec<(u64, i64)> = Vec::new();
    let mut stack = vec![(addr, 0i64)];
    while let Some((cur, at)) = stack.pop() {
        let Some(t) = by_addr.get(&cur) else { continue };
        for &(base, disp) in &t.bases {
            let sub = (base, at + disp);
            if base != addr && !out.contains(&sub) && out.len() < 4096 {
                out.push(sub);
                stack.push(sub);
            }
        }
    }
    out
}

/// The structure name the demangled signatures give a class: its innermost
/// component with any template arguments dropped (`ns::Vec<int>` is `Vec`).
fn struct_name(demangled: &str) -> String {
    let inner = top_level_split(demangled).pop().unwrap_or_default();
    let name = inner.split('<').next().unwrap_or_default().trim().to_string();
    if is_identifier(&name) { name } else { String::new() }
}

/// The method name of every function symbol a slot can name, by address.
fn method_names(file: &object::File) -> HashMap<u64, String> {
    let mut out = HashMap::new();
    for sym in file.symbols().chain(file.dynamic_symbols()) {
        if sym.kind() != SymbolKind::Text || sym.address() == 0 {
            continue;
        }
        let Ok(raw) = sym.name() else { continue };
        if let Some(name) = method_name(raw) {
            out.entry(sym.address()).or_insert(name);
        }
    }
    out
}

/// `_ZNK6Square4areaEi` → `area`; a destructor is `dtor`, or `deleting_dtor`
/// for the `D0` variant; a `this`-adjusting thunk names its target.
fn method_name(raw: &str) -> Option<String> {
    let target = thunk_target(raw);
    let raw = target.as_deref().unwrap_or(raw);
    let dem = demangle_raw(raw)?;
    let head = dem.split('(').next()?;
    let leaf = top_level_split(head).pop()?;
    if leaf.starts_with('~') {
        let deleting = raw.contains("D0E");
        return Some(if deleting { "deleting_dtor" } else { "dtor" }.to_string());
    }
    let leaf = leaf.split('<').next()?.trim();
    is_identifier(leaf).then(|| leaf.to_string())
}

/// The mangled name a `this`-adjusting thunk (`_ZThn16_N2MI2m2Ei`,
/// `_ZTv0_n24_N2VD2vbEi`) forwards to.
fn thunk_target(raw: &str) -> Option<String> {
    let mut rest = raw.strip_prefix("_ZT")?;
    let numbers = match rest.as_bytes().first()? {
        b'h' => 1,
        b'v' => 2,
        _ => return None,
    };
    rest = &rest[1..];
    for _ in 0..numbers {
        let digits = rest.strip_prefix('n').unwrap_or(rest);
        let end = digits.find('_')?;
        if end == 0 || !digits[..end].bytes().all(|b| b.is_ascii_digit()) {
            return None;
        }
        rest = &digits[end + 1..];
    }
    Some(format!("_Z{rest}"))
}

/// Split on every `::` outside template brackets.
fn top_level_split(s: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut depth = 0i32;
    let mut cur = String::new();
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '<' | '(' => depth += 1,
            '>' | ')' => depth -= 1,
            ':' if depth == 0 && chars.peek() == Some(&':') => {
                chars.next();
                parts.push(std::mem::take(&mut cur));
                continue;
            }
            _ => {}
        }
        cur.push(c);
    }
    parts.push(cur);
    parts
}

fn is_identifier(s: &str) -> bool {
    s.chars().next().is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
        && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn method_names_come_from_the_mangled_symbol() {
        assert_eq!(method_name("_ZNK6Square4areaEi").as_deref(), Some("area"));
        assert_eq!(method_name("_ZN6SquareD0Ev").as_deref(), Some("deleting_dtor"));
        assert_eq!(method_name("_ZN6SquareD1Ev").as_deref(), Some("dtor"));
        assert_eq!(method_name("_ZThn16_N2MI2m2Ei").as_deref(), Some("m2"));
        assert_eq!(method_name("_ZTv0_n24_N2VD2vbEi").as_deref(), Some("vb"));
        assert_eq!(method_name("_ZN2ns3VecIiE4pushEi").as_deref(), Some("push"));
        assert_eq!(method_name("main"), None);
    }

    #[test]
    fn struct_names_are_innermost_and_template_free() {
        assert_eq!(struct_name("IShape"), "IShape");
        assert_eq!(struct_name("leveldb::TableBuilder"), "TableBuilder");
        assert_eq!(struct_name("ns::Vec<std::pair<int, int> >"), "Vec");
        assert_eq!(struct_name("(anonymous namespace)::Hidden"), "Hidden");
    }
}

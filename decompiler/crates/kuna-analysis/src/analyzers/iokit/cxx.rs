//! (kuna `iokitclasses on`) The recovered IOKit classes as C++ types.
//!
//! Each class with a vtable becomes a [`VtableClass`] the typed-vtable commit
//! installs ([`kuna_decomp::kuna_vtabletypes`]): the class structure `C`
//! laid out to its `OSMetaClass` instance size with a first field
//! `C_vtbl *vtable`, and `C_vtbl` one method pointer per slot. A slot is
//! named after the method its function's name gives (`release`; `dtor` and
//! `deleting_dtor` for slots 0 and 1), inherited from the nearest ancestor
//! that names it, else `vfn_<i>` (an overload of a name an earlier slot took
//! is `<name>_<i>`); its targets are the class's own function
//! there and every descendant's override.
//!
//! Every function a class vtable holds is a virtual member, and every
//! `C::C` / `C::~C` of a recovered class is a constructor or destructor: all
//! take `this` in `x0`. Its class is the nearest class that is an ancestor of
//! every class whose vtable holds the function (for a constructor, `C`).

use std::collections::{BTreeMap, BTreeSet, HashMap};

use kuna_decomp::kuna_vtabletypes::{VtableClass, VtableSlot};

use super::{in_ranges, Recovered};
use crate::rtti::kuna_vtabletypes::VtableFacts;

/// The IOKit classes to type and the class of each member's `this`.
#[derive(Clone, Debug, Default)]
pub struct IokitCxxFacts {
    pub vtables: VtableFacts,
    /// `(function entry, class name)`, ascending by entry.
    pub this_types: Vec<(u64, String)>,
    /// `(address point, class name)`: where each typed class's vtable slots
    /// start, the object a constructor's `this->vtable` store points to.
    pub vtable_points: Vec<(u64, String)>,
    /// `(metaclass object, its MetaClass vtable slots)`: a virtual call on a
    /// `gMetaClass` object calls the method its own vtable holds.
    pub meta_slots: Vec<(u64, Vec<u64>)>,
    /// `(X::MetaClass::alloc, X)`: each returns a new `X`.
    pub allocs: Vec<(u64, String)>,
}

impl Recovered {
    /// The class types and `this` classes, given every name the image's
    /// symbols and the recovery give an address.
    pub(super) fn cxx_facts(&self, names: &HashMap<u64, String>) -> IokitCxxFacts {
        let by_meta = self.class_by_meta();
        let mut count: HashMap<&str, usize> = HashMap::new();
        for c in &self.classes {
            *count.entry(c.name.as_str()).or_default() += 1;
        }
        let unique = |i: usize| count.get(self.classes[i].name.as_str()) == Some(&1);
        let chains: Vec<Vec<usize>> = (0..self.classes.len()).map(|i| self.ancestors(i, &by_meta)).collect();

        let mut targets: Vec<Vec<BTreeSet<u64>>> =
            self.classes.iter().map(|c| vec![BTreeSet::new(); c.vtable.as_ref().map_or(0, |(_, s)| s.len())]).collect();
        for (d, c) in self.classes.iter().enumerate() {
            let Some((_, slots)) = &c.vtable else { continue };
            for &a in &chains[d] {
                let width = targets[a].len().min(slots.len());
                for (j, &f) in slots.iter().take(width).enumerate() {
                    targets[a][j].insert(f);
                }
            }
        }

        let mut slot_names: Vec<Vec<String>> = vec![Vec::new(); self.classes.len()];
        let mut order: Vec<usize> = (0..self.classes.len()).collect();
        order.sort_by_key(|&i| chains[i].len());
        let mut out = IokitCxxFacts::default();
        for i in order {
            let Some((slot0, slots)) = &self.classes[i].vtable else { continue };
            let parent = chains[i].get(1).copied();
            let mut used: BTreeSet<String> = BTreeSet::new();
            let mut fields = Vec::with_capacity(slots.len());
            for (j, &own) in slots.iter().enumerate() {
                let inherited = parent.and_then(|p| slot_names[p].get(j)).filter(|n| !n.starts_with("vfn_"));
                let name = match j {
                    0 => Some("dtor".to_string()),
                    1 => Some("deleting_dtor".to_string()),
                    _ => inherited
                        .cloned()
                        .or_else(|| method(names, own))
                        .or_else(|| targets[i][j].iter().find_map(|&f| method(names, f))),
                }
                .map(|n| if used.contains(&n) { format!("{n}_{j}") } else { n })
                .filter(|n| !used.contains(n))
                .unwrap_or_else(|| format!("vfn_{j}"));
                used.insert(name.clone());
                fields.push(name);
            }
            if unique(i) {
                let mut slot_list = Vec::with_capacity(slots.len());
                for (j, &own) in slots.iter().enumerate() {
                    let mut t = vec![own];
                    t.extend(targets[i][j].iter().copied().filter(|&f| f != own));
                    slot_list.push(VtableSlot { name: fields[j].clone(), targets: t });
                }
                out.vtables.classes.push(VtableClass {
                    class: self.classes[i].name.clone(),
                    slots: slot_list,
                    bases: Vec::new(),
                    size: self.classes[i].size,
                });
                out.vtables.slot_ranges.push((*slot0, slot0.saturating_add((8 * slots.len() as u64).saturating_sub(1))));
                out.vtable_points.push((*slot0, self.classes[i].name.clone()));
            }
            slot_names[i] = fields;
        }
        out.vtable_points.sort_unstable();
        out.vtables.slot_ranges.sort_unstable();
        out.vtables.slot_ranges.dedup();

        let mut this: BTreeMap<u64, String> = BTreeMap::new();
        let mut users: BTreeMap<u64, Vec<usize>> = BTreeMap::new();
        for (i, c) in self.classes.iter().enumerate() {
            for &f in c.vtable.iter().flat_map(|(_, s)| s.iter()) {
                let u = users.entry(f).or_default();
                if !u.contains(&i) {
                    u.push(i);
                }
            }
        }
        for (f, uses) in users {
            if !in_ranges(&self.own_code, f) || self.getters.contains_key(&f) {
                continue;
            }
            let owner = chains[uses[0]].iter().find(|a| uses.iter().all(|&i| chains[i].contains(a)));
            if let Some(&owner) = owner.filter(|&&o| unique(o)) {
                this.insert(f, self.classes[owner].name.clone());
            }
        }
        let by_name: HashMap<&str, usize> =
            (0..self.classes.len()).filter(|&i| unique(i)).map(|i| (self.classes[i].name.as_str(), i)).collect();
        for (&f, name) in names {
            if this.contains_key(&f) || !in_ranges(&self.own_code, f) {
                continue;
            }
            let Some((class, leaf)) = name.rsplit_once("::") else { continue };
            let leaf = leaf.strip_prefix('~').unwrap_or(leaf);
            if leaf == class && by_name.contains_key(class) {
                this.insert(f, class.to_string());
            }
        }
        out.this_types = this.into_iter().collect();

        let alloc_slot = self.meta_alloc_slot();
        let mut allocs: BTreeMap<u64, Vec<usize>> = BTreeMap::new();
        for (i, c) in self.classes.iter().enumerate() {
            let Some(slots) = c.meta_vtable.and_then(|v| self.meta_vtables.get(&v)) else { continue };
            out.meta_slots.push((c.meta, slots.clone()));
            if let Some(&f) = alloc_slot.and_then(|j| slots.get(j)) {
                allocs.entry(f).or_default().push(i);
            }
        }
        out.meta_slots.sort_unstable();
        out.allocs = allocs
            .into_iter()
            .filter_map(|(f, owners)| match owners.as_slice() {
                [i] if unique(*i) => Some((f, self.classes[*i].name.clone())),
                _ => None,
            })
            .collect();
        out
    }
}

/// The method name a function's name gives a vtable slot: the identifier
/// after its last `::`, not a destructor or a placeholder.
fn method(names: &HashMap<u64, String>, f: u64) -> Option<String> {
    let (_, m) = names.get(&f)?.rsplit_once("::")?;
    let ok = m.chars().next().is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
        && m.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
        && !m.starts_with("vfn_");
    ok.then(|| m.to_string())
}

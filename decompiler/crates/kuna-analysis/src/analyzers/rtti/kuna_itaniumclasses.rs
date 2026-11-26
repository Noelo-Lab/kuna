//! (kuna `itaniumrtti`) Class types for the Itanium classes the RTTI graph
//! recovers: a structure per class, a vtable structure per primary vtable, and
//! `this` typed in the class's own functions.
//!
//! The graph names a class and its vtable but gives no size and no data members.
//! The code does. A class's member functions are its primary vtable's slots,
//! attributed to the most-base class whose vtable holds them, and the
//! constructors and destructors the vptr stores name. Each is walked along its
//! fall-through path from the entry (the walk of [`super::kuna_itaniumctors`]),
//! and every load and store through `this` at a constant offset is recorded,
//! together with the size passed to `operator delete(this, N)` in the deleting
//! destructor (sized deallocation, the GCC and Clang default since C++14).
//!
//! A class gets a laid-out structure only when exactly one such size is seen and
//! it covers every access its own functions and its bases' functions make. A
//! structure smaller than the bytes a method reads prints worse than none, so a
//! size the accesses contradict is not used. The structure is `<C>_vtbl *vptr`
//! at 0, then one field per distinct access, gaps left open: `float`/`double`
//! when every access moves the value through a floating-point register,
//! `undefined<N>` when none does. An offset accessed both ways, or an access
//! overlapping the vptr or another access, gives no field. A base's accesses are a derived class's too, so the derived structure
//! repeats them and `this->field_0x8` reads the same in both. `<C>_vtbl` holds
//! one `code *` per slot: the slot's prototype is unknown, and a declared one
//! would be taken as fact at every call site.
//!
//! Every class's vptr object `<C>_vptr` is typed `<C>_vtbl`. The `this` of
//! every member function is an unlocked hint ([`kuna_decomp::kuna_inputhints`])
//! on the first argument register, so parameter recovery keeps the arity it
//! finds: typed `C *` when `C` is laid out, and only named otherwise, since an
//! opaque `C *` would stop structure synthesis from recovering the fields
//! itself. Left alone: a class with more than one
//! sub-vtable or a base that is not its primary one (multiple or virtual
//! inheritance, where a slot's `this` may point at a base subobject), a
//! function the vtables of unrelated classes share (identical code folding),
//! one that reads its `this` from the second parameter (a by-value return), a
//! convention passing `this` on the stack, and a class name another type of
//! the program already uses for something that is not a structure.

use std::collections::{BTreeSet, HashMap, HashSet};
use std::rc::Rc;

use kuna_decomp::dtype::{type_metatype, Datatype, TypeField};

use super::kuna_itaniumctors::{owners, ClassVtables, Walker};
use crate::pass::{AnalysisOutput, InputHintFact, TypedDataFact};

/// How far a member function is walked for its accesses.
const MAX_INSNS: usize = 256;

/// The largest class size taken from an `operator delete` argument.
const MAX_CLASS_SIZE: u64 = 1 << 20;

/// The most accesses one class's functions contribute.
const MAX_ACCESSES: usize = 4096;

/// What the member functions of one class showed.
#[derive(Default)]
struct Evidence {
    functions: Vec<u64>,
    accesses: Vec<(i64, u32, bool)>,
    deletes: BTreeSet<u64>,
}

pub(super) fn emit(
    walker: &Walker,
    classes: &[ClassVtables],
    members: &[(u64, usize)],
    out: &mut AnalysisOutput,
) {
    let Some(reg) = walker.this_register() else {
        return;
    };
    let eligible: Vec<bool> = classes
        .iter()
        .map(|c| c.single && c.ancestors.iter().all(|&a| classes[a].single))
        .collect();
    let owner_of = owners(classes);
    let mut evidence: HashMap<usize, Evidence> = HashMap::new();
    for (f, c) in member_functions(classes, members) {
        if !eligible[c] {
            continue;
        }
        let walk = walker.walk(f, &owner_of, MAX_INSNS);
        if walk.second {
            continue;
        }
        let e = evidence.entry(c).or_default();
        e.functions.push(f);
        e.accesses.extend(walk.accesses);
        e.accesses.truncate(MAX_ACCESSES);
        e.deletes.extend(walk.deletes);
    }
    if evidence.is_empty() {
        return;
    }
    let types = walker.ctx.arch.types();
    let ptr = types.get_size_of_pointer();
    let (_, word_size) = walker.ctx.arch.data_org();
    let names = type_names(classes);
    let mut order: Vec<usize> = (0..classes.len()).filter(|&c| eligible[c]).collect();
    order.sort_by_key(|&c| (classes[c].ancestors.len(), c));
    let mut sizes: HashMap<usize, u64> = HashMap::new();
    for c in order {
        let Some(e) = evidence.get(&c) else { continue };
        let accesses: Vec<(i64, u32, bool)> = std::iter::once(c)
            .chain(classes[c].ancestors.iter().copied())
            .filter_map(|k| evidence.get(&k))
            .flat_map(|e| e.accesses.iter().copied())
            .collect();
        let size = class_size(e, &accesses, ptr as u64).filter(|&n| {
            classes[c]
                .ancestors
                .iter()
                .all(|a| sizes.get(a).is_none_or(|&b| b <= n))
        });
        if let Some(n) = size {
            sizes.insert(c, n);
        }
        let slots = classes[c].primary_slots.len();
        let vtbl_t = (slots > 0)
            .then(|| vtable_type(walker, &names[c], slots))
            .flatten();
        if let (Some(vtbl_t), Some(&vptr)) = (&vtbl_t, classes[c].vptrs.first()) {
            out.typed_data.push(TypedDataFact {
                addr: vptr,
                name: format!("{}_vptr", classes[c].name),
                type_: Rc::clone(vtbl_t),
            });
        }
        let this_t = size
            .and_then(|n| class_type(walker, &names[c], vtbl_t, n, &accesses))
            .and_then(|t| types.get_type_pointer(ptr, t, word_size).ok());
        let mut functions = e.functions.clone();
        functions.sort_unstable();
        functions.dedup();
        for func_addr in functions {
            out.itanium_this_hints.push(InputHintFact {
                func_addr,
                reg: reg.clone().into(),
                name: "this".to_string(),
                type_: this_t.clone(),
            });
        }
    }
}

/// Every function with exactly one owning class: the primary vtable slots and
/// the named constructors and destructors, each owned by the most-base class
/// claiming it. A function claimed by classes of no single chain has none.
fn member_functions(classes: &[ClassVtables], members: &[(u64, usize)]) -> Vec<(u64, usize)> {
    let mut claims: HashMap<u64, HashSet<usize>> = HashMap::new();
    for (c, cls) in classes.iter().enumerate() {
        for f in cls.primary_slots.iter().flatten() {
            claims.entry(*f).or_default().insert(c);
        }
    }
    for &(f, c) in members {
        claims.entry(f).or_default().insert(c);
    }
    let mut out: Vec<(u64, usize)> = claims
        .into_iter()
        .filter_map(|(f, cs)| {
            let owner = cs.iter().copied().find(|&c| {
                cs.iter()
                    .all(|&o| o == c || classes[o].ancestors.contains(&c))
            })?;
            Some((f, owner))
        })
        .collect();
    out.sort_unstable();
    out
}

/// The one size `operator delete` is passed, when it holds every access.
fn class_size(e: &Evidence, accesses: &[(i64, u32, bool)], ptr: u64) -> Option<u64> {
    let mut sizes = e.deletes.iter().copied();
    let n = sizes.next()?;
    if sizes.next().is_some() || n < ptr || n > MAX_CLASS_SIZE {
        return None;
    }
    accesses
        .iter()
        .all(|&(o, w, _)| o >= 0 && (o as u64).saturating_add(u64::from(w)) <= n)
        .then_some(n)
}

/// The fields the accesses give past the vptr: one per offset read or written
/// at a single width, none where two accesses overlap, floating-point where
/// every access moves the value through a floating-point register and none
/// where only some do. A vector access wider than a word (a 16-byte store
/// clearing two members) names no field and keeps none from being named.
fn fields(accesses: &[(i64, u32, bool)], ptr: u64, size: u64) -> Vec<(u64, u32, bool)> {
    let mut widths: HashMap<u64, BTreeSet<u32>> = HashMap::new();
    let mut float: HashMap<u64, BTreeSet<bool>> = HashMap::new();
    for &(o, w, f) in accesses {
        if o >= 0 && (1..=8).contains(&w) {
            widths.entry(o as u64).or_default().insert(w);
            float.entry(o as u64).or_default().insert(f);
        }
    }
    let mut spans: Vec<(u64, u32, bool)> = Vec::new();
    let mut overlapping: HashSet<u64> = HashSet::new();
    let mut all: Vec<(u64, u32)> = widths
        .iter()
        .flat_map(|(&o, ws)| ws.iter().map(move |&w| (o, w)))
        .collect();
    all.sort_unstable();
    for (i, &(o, w)) in all.iter().enumerate() {
        let end = o + u64::from(w);
        if o < ptr || end > size || !w.is_power_of_two() {
            overlapping.insert(o);
            continue;
        }
        for &(p, _) in &all[i + 1..] {
            if p >= end {
                break;
            }
            overlapping.insert(o);
            overlapping.insert(p);
        }
        match float[&o].iter().copied().collect::<Vec<_>>()[..] {
            [f] => spans.push((o, w, f)),
            _ => {
                overlapping.insert(o);
            }
        }
    }
    spans.retain(|(o, ..)| !overlapping.contains(o));
    spans
}

/// Each class's type name: its innermost name, or the whole qualified name with
/// `_` for `::` where two recovered classes share the innermost one.
fn type_names(classes: &[ClassVtables]) -> Vec<String> {
    let leaf = |n: &str| n.rsplit("::").next().unwrap_or(n).to_string();
    let mut count: HashMap<String, usize> = HashMap::new();
    for c in classes {
        *count.entry(leaf(&c.name)).or_default() += 1;
    }
    classes
        .iter()
        .map(|c| {
            let l = leaf(&c.name);
            if count[&l] == 1 {
                l
            } else {
                c.name.replace("::", "_")
            }
        })
        .collect()
}

/// The structure named `name`, laid out to `size` from the accesses with a
/// `vptr` to `vtbl_t` at 0. A complete structure the program already has under
/// that name is the class.
fn class_type(
    walker: &Walker,
    name: &str,
    vtbl_t: Option<Rc<Datatype>>,
    size: u64,
    accesses: &[(i64, u32, bool)],
) -> Option<Rc<Datatype>> {
    let types = walker.ctx.arch.types();
    let shell = match types.find_by_name(name).ok().flatten() {
        Some(t) if t.get_metatype() == type_metatype::TYPE_STRUCT && !t.is_incomplete() => {
            return Some(t)
        }
        Some(t) if t.get_metatype() == type_metatype::TYPE_STRUCT => t,
        Some(_) => return None,
        None => types.kuna_declare_record(name, false).ok()?,
    };
    let ptr = types.get_size_of_pointer();
    let (_, word_size) = walker.ctx.arch.data_org();
    let vptr_t = match vtbl_t {
        Some(t) => types.get_type_pointer(ptr, t, word_size).ok()?,
        None => types
            .get_type_pointer(ptr, types.get_type_void().ok()?, word_size)
            .ok()?,
    };
    let mut fs = vec![TypeField::new(0, 0, "vptr", vptr_t)];
    for (o, w, float) in fields(accesses, ptr as u64, size) {
        let meta = match (float, w) {
            (true, 4 | 8) => type_metatype::TYPE_FLOAT,
            (true, _) => continue,
            (false, _) => type_metatype::TYPE_UNKNOWN,
        };
        let t = types.get_base(w as i32, meta).ok()?;
        fs.push(TypeField::new(
            o as i32,
            o as i32,
            format!("field_0x{o:x}"),
            t,
        ));
    }
    types
        .set_fields_struct_raw(&shell, fs, Vec::new(), size as i32, ptr.max(1), 0)
        .ok()
}

/// `<name>_vtbl`: a `code *` per primary slot.
fn vtable_type(walker: &Walker, name: &str, slots: usize) -> Option<Rc<Datatype>> {
    let types = walker.ctx.arch.types();
    let vname = format!("{name}_vtbl");
    let shell = match types.find_by_name(&vname).ok().flatten() {
        Some(t) if t.get_metatype() == type_metatype::TYPE_STRUCT && !t.is_incomplete() => {
            return Some(t)
        }
        Some(t) if t.get_metatype() == type_metatype::TYPE_STRUCT => t,
        Some(_) => return None,
        None => types.kuna_declare_record(&vname, false).ok()?,
    };
    if slots == 0 {
        return Some(shell);
    }
    let ptr = types.get_size_of_pointer();
    let (_, word_size) = walker.ctx.arch.data_org();
    let code = types.get_type_code().ok()?;
    let slot_t = types.get_type_pointer(ptr, code, word_size).ok()?;
    let fs = (0..slots)
        .map(|i| {
            let o = (i as i32) * ptr;
            TypeField::new(o, o, format!("vfunc_{i}"), Rc::clone(&slot_t))
        })
        .collect();
    types
        .set_fields_struct_raw(&shell, fs, Vec::new(), slots as i32 * ptr, ptr.max(1), 0)
        .ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn class(name: &str, ancestors: &[usize], slots: &[u64]) -> ClassVtables {
        ClassVtables {
            name: name.into(),
            single: true,
            ancestors: ancestors.iter().copied().collect(),
            primary_slots: slots.iter().map(|&s| Some(s)).collect(),
            vptrs: vec![],
        }
    }

    #[test]
    fn a_slot_belongs_to_the_most_base_class_holding_it() {
        let classes = [
            class("Shape", &[], &[0x10, 0x20]),
            class("Circle", &[0], &[0x30, 0x20]),
            class("Other", &[], &[0x20]),
        ];
        let m = member_functions(&classes[..2], &[(0x40, 1)]);
        assert_eq!(m, vec![(0x10, 0), (0x20, 0), (0x30, 1), (0x40, 1)]);
        let folded = member_functions(&classes, &[]);
        assert!(
            !folded.iter().any(|&(f, _)| f == 0x20),
            "unrelated classes share it"
        );
    }

    #[test]
    fn the_size_must_cover_every_access() {
        let e = Evidence {
            deletes: [0x10].into_iter().collect(),
            ..Default::default()
        };
        assert_eq!(
            class_size(&e, &[(8, 4, false), (0xc, 4, false)], 8),
            Some(0x10)
        );
        assert_eq!(class_size(&e, &[(0x10, 4, false)], 8), None);
        let two = Evidence {
            deletes: [0x10, 0x18].into_iter().collect(),
            ..Default::default()
        };
        assert_eq!(class_size(&two, &[], 8), None);
    }

    #[test]
    fn overlapping_and_vptr_accesses_give_no_field() {
        let f = fields(
            &[
                (0, 8, false),
                (8, 4, false),
                (8, 4, false),
                (0xc, 4, false),
                (0x10, 8, false),
                (0x14, 4, false),
                (4, 4, false),
            ],
            8,
            0x20,
        );
        assert_eq!(f, vec![(8, 4, false), (0xc, 4, false)]);
        let f = fields(
            &[
                (8, 8, true),
                (8, 8, true),
                (0x10, 4, true),
                (0x10, 4, false),
            ],
            8,
            0x18,
        );
        assert_eq!(f, vec![(8, 8, true)], "a field read both ways is left out");
        let f = fields(
            &[(0x18, 16, false), (0x18, 8, false), (0x20, 8, false)],
            8,
            0x28,
        );
        assert_eq!(
            f,
            vec![(0x18, 8, false), (0x20, 8, false)],
            "a vector store spans members"
        );
    }

    #[test]
    fn a_shared_innermost_name_keeps_its_scope() {
        let classes = [
            class("a::Node", &[], &[]),
            class("b::Node", &[], &[]),
            class("a::Tree", &[], &[]),
        ];
        assert_eq!(type_names(&classes), vec!["a_Node", "b_Node", "Tree"]);
    }
}

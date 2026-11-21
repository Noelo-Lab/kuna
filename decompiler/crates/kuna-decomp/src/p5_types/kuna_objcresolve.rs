//! (kuna `objcresolve`) What a message send returns and whom it reaches, from
//! its receiver.
//!
//! An Objective-C call site names its selector but not its method: `objc_msgSend`
//! and the `objc_msgSend$<sel>` stubs are typed once for every receiver, so
//! `[[Foo alloc] init]` is an `objc_object *` and `[obj count]` a call into the
//! runtime. The receiver says more: `alloc`/`new` on a class
//! reference return that class, `init…`/`self`/`retain`/`copy` return their
//! receiver (the `instancetype` contract), and a receiver whose class is known
//! resolves the selector along the superclass chain to the method that runs.
//!
//! The ObjC analysis hands the engine its class table ([`ObjcClassTable`]:
//! each class's own methods, its superclass, the `class_t` address its class
//! reference folds to, and every selector string's address). The P5 arm
//! ([`instancetype`]) answers `TypeOpCall::getOutputLocal` for such a call
//! before the stub's locked `id` does; the P9 arm ([`resolved_callee`]) renders
//! the resolved method's label in place of the runtime entry point.

use std::collections::HashMap;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use crate::p4_calls::fspec::FuncCallSpecs;

/// One class of the image: its own methods and where to look next.
#[derive(Debug, Default, Clone)]
pub struct ObjcClass {
    /// The superclass's shown name, when it is a class of this image.
    pub superclass: Option<String>,
    /// `(class method?, selector)` → the IMP's label (`-[Foo bar:]`) and address.
    pub methods: HashMap<(bool, String), (String, u64)>,
}

/// The image's Objective-C classes, keyed by shown name.
#[derive(Debug, Default, Clone)]
pub struct ObjcClassTable {
    pub classes: HashMap<String, ObjcClass>,
    /// `class_t` address → shown name (what a folded class reference is).
    pub class_at: HashMap<u64, String>,
    /// Selector string address → selector (what a folded selector reference is).
    pub selector_at: HashMap<u64, String>,
    /// Selector reference slot address → the selector it holds (what a load
    /// from the slot is).
    pub selref_at: HashMap<u64, String>,
}

/// The runtime entry points that allocate an instance of their class argument.
const ALLOCATORS: [&str; 4] = ["objc_alloc", "objc_alloc_init", "objc_opt_new", "objc_allocWithZone"];

/// Selectors that return a new instance of the receiver class.
fn allocates(sel: &str) -> bool {
    matches!(sel, "alloc" | "allocWithZone:" | "new")
}

/// Selectors that return their receiver (`instancetype` on an instance).
fn returns_receiver(sel: &str) -> bool {
    sel.starts_with("init") || matches!(sel, "self" | "retain" | "autorelease" | "copy" | "mutableCopy")
}

impl ObjcClassTable {
    /// The method `sel` reaches on an instance (or, `class_method`, the class)
    /// of `class`, walking the superclass chain: its label and address.
    pub fn resolve(&self, class: &str, sel: &str, class_method: bool) -> Option<(&str, u64)> {
        let key = (class_method, sel.to_string());
        let mut name = class;
        for _ in 0..64 {
            let c = self.classes.get(name)?;
            if let Some((label, imp)) = c.methods.get(&key) {
                return Some((label, *imp));
            }
            name = c.superclass.as_deref()?;
        }
        None
    }
}

/// The call's selector: the stub's suffix, the folded second argument of a
/// plain `objc_msgSend`, or the implied `alloc` of an allocator.
fn selector(data: &Funcdata, op: OpId, fc: &FuncCallSpecs) -> Option<String> {
    let name = fc.get_name();
    if let Some(sel) = name.strip_prefix("objc_msgSend$") {
        return Some(sel.to_string());
    }
    if ALLOCATORS.contains(&name) {
        return Some("alloc".to_string());
    }
    if name == "objc_msgSend" {
        let vn = data.obank().get(op)?.get_in(2)?;
        return selector_of(data, vn, 0);
    }
    None
}

/// The selector a varnode carries: the string a constant names, the slot a
/// load reads, through copies and casts.
fn selector_of(data: &Funcdata, vn: VarnodeId, depth: u32) -> Option<String> {
    let table = &data.get_arch().kuna_objc_classes;
    if let Some(addr) = constant_address(data, vn, 0) {
        return table.selector_at.get(&addr).cloned();
    }
    if depth > 4 {
        return None;
    }
    let v = data.vbank().get(vn)?;
    let d = data.obank().get(v.get_def()?)?;
    match d.code() {
        OpCode::CPUI_COPY | OpCode::CPUI_CAST => selector_of(data, d.get_in(0)?, depth + 1),
        OpCode::CPUI_LOAD => {
            let slot = constant_address(data, d.get_in(1)?, 0)?;
            table.selref_at.get(&slot).cloned()
        }
        _ => None,
    }
}

/// The class a folded class reference names: the receiver is the constant
/// address of a `class_t`.
fn class_of_constant(data: &Funcdata, vn: VarnodeId) -> Option<String> {
    let addr = constant_address(data, vn, 0)?;
    data.get_arch().kuna_objc_classes.class_at.get(&addr).cloned()
}

/// The address a varnode names: a constant, `&sym` as `PTRSUB(0, addr)`, either
/// through the copies and casts the printer's own passes wrap it in.
pub(crate) fn constant_address(data: &Funcdata, vn: VarnodeId, depth: u32) -> Option<u64> {
    let v = data.vbank().get(vn)?;
    if v.is_constant() {
        return Some(v.get_offset());
    }
    if depth > 4 {
        return None;
    }
    let d = data.obank().get(v.get_def()?)?;
    match d.code() {
        OpCode::CPUI_COPY | OpCode::CPUI_CAST => constant_address(data, d.get_in(0)?, depth + 1),
        OpCode::CPUI_PTRSUB => {
            let base = data.vbank().get(d.get_in(0)?)?;
            let off = data.vbank().get(d.get_in(1)?)?;
            (base.is_constant() && base.get_offset() == 0 && off.is_constant()).then(|| off.get_offset())
        }
        _ => None,
    }
}

/// The class an instance receiver belongs to: its type is a pointer to the
/// class's struct. A receiver defined by another message send is asked the
/// same question first, so `[[Foo alloc] init]` types through.
fn class_of_receiver(data: &Funcdata, vn: VarnodeId, depth: u32) -> Option<Rc<Datatype>> {
    let v = data.vbank().get(vn)?;
    if depth < 6 && !v.is_type_lock() {
        if let Some(d) = v.get_def().and_then(|def| data.obank().get(def).map(|o| (def, o))) {
            let (def, o) = d;
            match o.code() {
                OpCode::CPUI_CALL => {
                    if let Some(fc) = data.get_call_specs_index(def).map(|i| data.get_call_specs(i)) {
                        if let Some(t) = instancetype_at(data, def, fc, depth + 1) {
                            return Some(t);
                        }
                    }
                }
                OpCode::CPUI_COPY | OpCode::CPUI_CAST => {
                    if let Some(t) = class_of_receiver(data, o.get_in(0)?, depth + 1) {
                        return Some(t);
                    }
                }
                _ => {}
            }
        }
    }
    let t = v.get_type();
    pointer_to_struct(t).then(|| Rc::clone(t))
}

fn pointer_to_struct(t: &Rc<Datatype>) -> bool {
    t.get_metatype() == type_metatype::TYPE_PTR
        && t.get_ptr_to().is_some_and(|p| p.get_metatype() == type_metatype::TYPE_STRUCT)
}

fn struct_pointer(data: &Funcdata, class: &str) -> Option<Rc<Datatype>> {
    let arch = data.get_arch();
    let types = arch.types()?;
    let st = types.find_by_name(class).ok().flatten()?;
    if st.get_metatype() != type_metatype::TYPE_STRUCT {
        return None;
    }
    let space = arch.manage.get_default_data_space()?;
    types.get_type_pointer(space.get_addr_size() as i32, st, space.get_word_size()).ok()
}

fn instancetype_at(data: &Funcdata, op: OpId, fc: &FuncCallSpecs, depth: u32) -> Option<Rc<Datatype>> {
    let sel = selector(data, op, fc)?;
    let recv = data.obank().get(op)?.get_in(1)?;
    if allocates(&sel) {
        let class = class_of_constant(data, recv)?;
        return struct_pointer(data, &class);
    }
    if returns_receiver(&sel) {
        return class_of_receiver(data, recv, depth);
    }
    None
}

/// `TypeOpCall::getOutputLocal` for a message send whose result the receiver
/// decides: `Foo *` for `[Foo alloc]` / `objc_alloc_init(Foo)`, the receiver's
/// class pointer for `init…`, `self`, `retain`, `autorelease`, `copy`.
pub fn instancetype(data: &Funcdata, op: OpId, fc: &FuncCallSpecs) -> Option<Rc<Datatype>> {
    if !data.get_arch().objc_resolve {
        return None;
    }
    instancetype_at(data, op, fc, 0)
}

/// A message send whose receiver's class is known: the method it reaches.
#[derive(Debug, Clone)]
pub struct ResolvedSend {
    /// The CALL op's address.
    pub site: Address,
    /// The IMP's entry and label (`-[Foo bar:]`).
    pub imp: u64,
    pub label: String,
    /// The callee is an `objc_msgSend$<sel>` stub (the selector is not an
    /// argument of the call) rather than `objc_msgSend`.
    pub stub: bool,
    /// The receiver's class pointer type, when the receiver is an instance.
    pub self_type: Option<Rc<Datatype>>,
}

/// The method the send at `op` reaches when its receiver's class is known.
pub fn resolve_send(data: &Funcdata, op: OpId) -> Option<ResolvedSend> {
    let arch = data.get_arch();
    if !arch.objc_resolve {
        return None;
    }
    let fc = data.get_call_specs(data.get_call_specs_index(op)?);
    let stub = fc.get_name().starts_with("objc_msgSend$");
    if !stub && fc.get_name() != "objc_msgSend" {
        return None;
    }
    let sel = selector(data, op, fc)?;
    let o = data.obank().get(op)?;
    let recv = o.get_in(1)?;
    let site = o.get_addr().clone();
    let table = &arch.kuna_objc_classes;
    if let Some(class) = class_of_constant(data, recv) {
        let (label, imp) = table.resolve(&class, &sel, true)?;
        return Some(ResolvedSend { site, imp, label: label.to_string(), stub, self_type: None });
    }
    let t = class_of_receiver(data, recv, 0)?;
    let class = t.get_ptr_to()?.get_name().to_string();
    let (label, imp) = table.resolve(&class, &sel, false)?;
    Some(ResolvedSend { site, imp, label: label.to_string(), stub, self_type: Some(t) })
}

/// The method a message send reaches when its receiver's class is known: the
/// label (`-[Foo bar:]`, `+[Foo new]`) the printer renders as the callee.
pub fn resolved_callee(data: &Funcdata, op: OpId) -> Option<String> {
    resolve_send(data, op).map(|r| r.label)
}

/// Every send of the finished function whose method is known and whose call
/// still goes to the runtime: what a second decompile retargets.
pub fn resolved_sends(data: &Funcdata) -> Vec<ResolvedSend> {
    (0..data.num_calls())
        .filter_map(|i| resolve_send(data, data.get_call_specs(i).get_op()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn table() -> ObjcClassTable {
        let mut t = ObjcClassTable::default();
        let mut base = ObjcClass::default();
        base.methods.insert((false, "count".into()), ("-[Base count]".into(), 0x100));
        base.methods.insert((true, "alloc".into()), ("+[Base alloc]".into(), 0x200));
        let mut sub = ObjcClass { superclass: Some("Base".into()), ..Default::default() };
        sub.methods.insert((false, "x".into()), ("-[Sub x]".into(), 0x300));
        t.classes.insert("Base".into(), base);
        t.classes.insert("Sub".into(), sub);
        t
    }

    #[test]
    fn resolution_walks_the_superclass_chain() {
        let t = table();
        assert_eq!(t.resolve("Sub", "x", false), Some(("-[Sub x]", 0x300)));
        assert_eq!(t.resolve("Sub", "count", false), Some(("-[Base count]", 0x100)));
        assert_eq!(t.resolve("Sub", "alloc", true), Some(("+[Base alloc]", 0x200)));
        assert_eq!(t.resolve("Sub", "count", true), None, "a class method is not an instance one");
        assert_eq!(t.resolve("Other", "count", false), None);
    }

    #[test]
    fn instancetype_selectors() {
        assert!(allocates("alloc") && allocates("new") && !allocates("init"));
        assert!(returns_receiver("initWithFrame:") && returns_receiver("self") && !returns_receiver("count"));
    }
}

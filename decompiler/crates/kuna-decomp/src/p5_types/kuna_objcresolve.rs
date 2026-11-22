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

/// One class: its own methods and where to look next. A class of the image
/// carries its IMPs; a class the SDK headers declare (`objcsdk`) carries its
/// declarations only.
#[derive(Debug, Default, Clone)]
pub struct ObjcClass {
    /// The superclass's shown name.
    pub superclass: Option<String>,
    /// `(class method?, selector)` → the method.
    pub methods: HashMap<(bool, String), ObjcMethod>,
}

/// A method a send can reach.
#[derive(Debug, Clone)]
pub struct ObjcMethod {
    /// `-[Foo bar:]` / `+[NSNumber numberWithInt:]`.
    pub label: String,
    /// The IMP's entry, 0 for a method only the SDK headers declare.
    pub imp: u64,
    /// The declared result, when it is a class pointer or `instancetype`.
    pub returns: Option<ObjcReturn>,
}

/// A declared result the receiver or the declaration decides.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObjcReturn {
    /// `instancetype`: the receiver's class.
    Instancetype,
    /// A pointer to the named class.
    Class(String),
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
    /// Data import slot address → the class whose `OBJC_CLASS_$_` the loader
    /// binds it to (what the slot's own value is).
    pub class_slot_at: HashMap<u64, String>,
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
    pub fn resolve(&self, class: &str, sel: &str, class_method: bool) -> Option<&ObjcMethod> {
        let key = (class_method, sel.to_string());
        let mut name = class;
        for _ in 0..64 {
            let c = self.classes.get(name)?;
            if let Some(m) = c.methods.get(&key) {
                return Some(m);
            }
            name = c.superclass.as_deref()?;
        }
        None
    }
}

/// A class factory whose leading noun is the class's own: `numberWith…` on
/// `NSNumber`, `stringWith…` on `NSMutableString`, `URLWith…` on `NSURL`.
fn factory_of(class: &str, sel: &str) -> bool {
    let Some(k) = sel.find("With") else { return false };
    let noun = &sel[..k];
    !noun.is_empty()
        && class.len() >= noun.len()
        && class.is_char_boundary(class.len() - noun.len())
        && class[class.len() - noun.len()..].eq_ignore_ascii_case(noun)
}

/// The call's selector: the stub's suffix, the folded second argument of a
/// plain `objc_msgSend` or `objc_msgSendSuper2`, or the implied `alloc` of an
/// allocator.
fn selector(data: &Funcdata, op: OpId, fc: &FuncCallSpecs) -> Option<String> {
    let name = fc.get_name();
    if let Some(sel) = name.strip_prefix("objc_msgSend$") {
        return Some(sel.to_string());
    }
    if ALLOCATORS.contains(&name) {
        return Some("alloc".to_string());
    }
    if name == "objc_msgSend" || name == SEND_SUPER {
        let vn = data.obank().get(op)?.get_in(2)?;
        return selector_of(data, vn, 0);
    }
    None
}

/// The runtime entry point of `[super sel]`: its receiver is an `objc_super`
/// whose class is the sending method's own, the lookup starting above it.
const SEND_SUPER: &str = "objc_msgSendSuper2";

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

/// The class a class-object receiver names: the constant address of a
/// `class_t` of the image, or the value of a data import slot bound to
/// `OBJC_CLASS_$_<name>`, through copies and casts.
fn class_of_constant(data: &Funcdata, vn: VarnodeId) -> Option<String> {
    let arch = data.get_arch();
    let mut vn = vn;
    for _ in 0..5 {
        if let Some(addr) = constant_address(data, vn, 0) {
            return arch.kuna_objc_classes.class_at.get(&addr).cloned();
        }
        let v = data.vbank().get(vn)?;
        if v.is_persist() && !v.is_constant() {
            let class = arch.kuna_objc_classes.class_slot_at.get(&v.get_offset())?;
            return crate::printc::holds_loaded_value(data, vn, 8).then(|| class.clone());
        }
        let d = data.obank().get(v.get_def()?)?;
        if !matches!(d.code(), OpCode::CPUI_COPY | OpCode::CPUI_CAST) {
            return None;
        }
        vn = d.get_in(0)?;
    }
    None
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
    if let Some(t) = (depth < 6 && !v.is_type_lock()).then(|| class_of_definition(data, vn, depth)).flatten() {
        return Some(t);
    }
    let t = v.get_type();
    pointer_to_struct(t).then(|| Rc::clone(t))
}

fn class_of_definition(data: &Funcdata, vn: VarnodeId, depth: u32) -> Option<Rc<Datatype>> {
    let def = data.vbank().get(vn)?.get_def()?;
    let o = data.obank().get(def)?;
    match o.code() {
        OpCode::CPUI_CALL => {
            let fc = data.get_call_specs(data.get_call_specs_index(def)?);
            instancetype_at(data, def, fc, depth + 1)
        }
        OpCode::CPUI_COPY | OpCode::CPUI_CAST => class_of_receiver(data, o.get_in(0)?, depth + 1),
        _ => None,
    }
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

/// The class `[super …]` starts above, and whether the send is a class
/// method's: the sending method's own class, from its label.
fn super_start(data: &Funcdata) -> Option<(String, bool)> {
    let name = data.get_name();
    let class_method = name.starts_with('+');
    let class = name.strip_prefix(['-', '+'])?.strip_prefix('[')?.split([' ', '(']).next()?;
    let table = &data.get_arch().kuna_objc_classes;
    Some((table.classes.get(class)?.superclass.clone()?, class_method))
}

/// The receiver's class, and whether the send goes to the class object: a
/// class reference, a typed instance, or the `objc_super` of `[super …]`.
enum Receiver {
    Class(String),
    Instance(Rc<Datatype>),
    Super(String, bool),
}

fn receiver(data: &Funcdata, op: OpId, fc: &FuncCallSpecs, depth: u32) -> Option<Receiver> {
    if fc.get_name() == SEND_SUPER {
        let (class, class_method) = super_start(data)?;
        return Some(Receiver::Super(class, class_method));
    }
    let recv = data.obank().get(op)?.get_in(1)?;
    if let Some(class) = class_of_constant(data, recv) {
        return Some(Receiver::Class(class));
    }
    class_of_receiver(data, recv, depth).map(Receiver::Instance)
}

fn instancetype_at(data: &Funcdata, op: OpId, fc: &FuncCallSpecs, depth: u32) -> Option<Rc<Datatype>> {
    let sel = selector(data, op, fc)?;
    let table = &data.get_arch().kuna_objc_classes;
    match receiver(data, op, fc, depth) {
        Some(Receiver::Class(class)) => {
            let result = match table.resolve(&class, &sel, true).and_then(|m| m.returns.as_ref()) {
                _ if allocates(&sel) => class,
                Some(ObjcReturn::Class(c)) => c.clone(),
                Some(ObjcReturn::Instancetype) => class,
                None if factory_of(&class, &sel) => class,
                None => return None,
            };
            struct_pointer(data, &result)
        }
        Some(Receiver::Super(start, class_method)) => {
            let own = data.get_name().strip_prefix(['-', '+'])?.strip_prefix('[')?.split([' ', '(']).next()?;
            let returns = table.resolve(&start, &sel, class_method).and_then(|m| m.returns.as_ref());
            let result = match returns {
                Some(ObjcReturn::Class(c)) => c.as_str(),
                Some(ObjcReturn::Instancetype) => own,
                None if returns_receiver(&sel) || class_method && allocates(&sel) => own,
                None => return None,
            };
            struct_pointer(data, result)
        }
        Some(Receiver::Instance(t)) => {
            if returns_receiver(&sel) {
                return Some(t);
            }
            match table.resolve(t.get_ptr_to()?.get_name(), &sel, false)?.returns.as_ref()? {
                ObjcReturn::Instancetype => Some(t),
                ObjcReturn::Class(c) => struct_pointer(data, c),
            }
        }
        None => None,
    }
}

/// `TypeOpCall::getOutputLocal` for a message send whose result the receiver
/// decides: `Foo *` for `[Foo alloc]` / `objc_alloc_init(Foo)` and for a class
/// factory of `Foo` (`+[NSNumber numberWithInt:]`), the receiver's class
/// pointer for `init…`, `self`, `retain`, `autorelease`, `copy` and an
/// `instancetype` declaration, and the class a declaration returns.
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
    /// The IMP's entry (0 when the call cannot go there: a method only the SDK
    /// declares, or a `[super …]` whose first argument is not the receiver)
    /// and label (`-[Foo bar:]`).
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
    if !stub && fc.get_name() != "objc_msgSend" && fc.get_name() != SEND_SUPER {
        return None;
    }
    let sel = selector(data, op, fc)?;
    let site = data.obank().get(op)?.get_addr().clone();
    let table = &arch.kuna_objc_classes;
    let (m, self_type, retarget) = match receiver(data, op, fc, 0)? {
        Receiver::Class(class) => (table.resolve(&class, &sel, true)?, None, true),
        Receiver::Super(class, class_method) => (table.resolve(&class, &sel, class_method)?, None, false),
        Receiver::Instance(t) => (table.resolve(t.get_ptr_to()?.get_name(), &sel, false)?, Some(t), true),
    };
    let imp = if retarget { m.imp } else { 0 };
    Some(ResolvedSend { site, imp, label: m.label.clone(), stub, self_type })
}

/// The method a message send reaches when its receiver's class is known: the
/// label (`-[Foo bar:]`, `+[Foo new]`) the printer renders as the callee.
pub fn resolved_callee(data: &Funcdata, op: OpId) -> Option<String> {
    resolve_send(data, op).map(|r| r.label)
}

/// Every send of the finished function whose method is an IMP of the image
/// and whose call still goes to the runtime: what a second decompile
/// retargets.
pub fn resolved_sends(data: &Funcdata) -> Vec<ResolvedSend> {
    (0..data.num_calls())
        .filter_map(|i| resolve_send(data, data.get_call_specs(i).get_op()))
        .filter(|r| r.imp != 0)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn table() -> ObjcClassTable {
        let mut t = ObjcClassTable::default();
        let mut base = ObjcClass::default();
        let m = |label: &str, imp| ObjcMethod { label: label.into(), imp, returns: None };
        base.methods.insert((false, "count".into()), m("-[Base count]", 0x100));
        base.methods.insert((true, "alloc".into()), m("+[Base alloc]", 0x200));
        let mut sub = ObjcClass { superclass: Some("Base".into()), ..Default::default() };
        sub.methods.insert((false, "x".into()), m("-[Sub x]", 0x300));
        t.classes.insert("Base".into(), base);
        t.classes.insert("Sub".into(), sub);
        t
    }

    #[test]
    fn resolution_walks_the_superclass_chain() {
        let t = table();
        let at = |c, s, cm| t.resolve(c, s, cm).map(|m| (m.label.as_str(), m.imp));
        assert_eq!(at("Sub", "x", false), Some(("-[Sub x]", 0x300)));
        assert_eq!(at("Sub", "count", false), Some(("-[Base count]", 0x100)));
        assert_eq!(at("Sub", "alloc", true), Some(("+[Base alloc]", 0x200)));
        assert_eq!(at("Sub", "count", true), None, "a class method is not an instance one");
        assert_eq!(at("Other", "count", false), None);
    }

    #[test]
    fn instancetype_selectors() {
        assert!(allocates("alloc") && allocates("new") && !allocates("init"));
        assert!(returns_receiver("initWithFrame:") && returns_receiver("self") && !returns_receiver("count"));
    }

    #[test]
    fn a_factory_names_its_class() {
        assert!(factory_of("NSNumber", "numberWithLongLong:"));
        assert!(factory_of("NSMutableString", "stringWithFormat:"));
        assert!(factory_of("NSURL", "URLWithString:"));
        assert!(!factory_of("NSNumber", "stringWithFormat:"));
        assert!(!factory_of("NSData", "dataTaskWithURL:") && !factory_of("NSNumber", "WithX:"));
    }
}

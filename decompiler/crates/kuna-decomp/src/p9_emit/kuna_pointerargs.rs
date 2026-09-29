//! Preserve the actual C pointer type at a call boundary.

use std::collections::BTreeMap;
use std::rc::Rc;

use crate::context::{HighVariableId, OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use kuna_num::opcodes::OpCode;

#[derive(Default)]
pub(crate) struct Declarations(BTreeMap<String, (String, bool)>);

impl Declarations {
    pub(crate) fn clear(&mut self) {
        self.0.clear();
    }

    pub(crate) fn record(&mut self, name: &str, front: &str, back: &str, array: bool) {
        if back.is_empty() && !array {
            self.0.insert(name.to_string(), (front.to_string(), array));
        }
    }

    pub(crate) fn address(&self, name: &str) -> Option<String> {
        let (base, _) = self.0.get(name)?;
        Some(pointer_to(base))
    }
}

fn pointer_to(base: &str) -> String {
    format!("{base}{}*", if base.ends_with('*') { "" } else { " " })
}

pub(crate) trait PrintedPointers {
    fn spell(&self, ty: &Rc<Datatype>) -> String;
    fn address(&self, high: HighVariableId) -> Option<String>;
}

/// Only an in-scope declared parameter determines a conversion. Varargs,
/// per-call overrides, and speculative input trials provide no such declaration.
pub(crate) fn argument_cast(
    p: &dyn PrintedPointers,
    fd: &Funcdata,
    arch: &crate::architecture::Architecture,
    call: OpId,
    slot: i32,
) -> Option<Rc<Datatype>> {
    let op = fd.obank().get(call)?;
    if op.code() != OpCode::CPUI_CALL || slot < 1 {
        return None;
    }
    let fc = fd.get_call_specs(fd.get_call_specs_index(call)?);
    if fc.format_arity().is_some()
        || fd
            .get_override()
            .find_proto_override(op.get_addr())
            .is_some()
    {
        return None;
    }
    let actual = pointer_type(p, fd, op.get_in(slot)?, 0)?;
    let index = (slot - 1) as usize;
    let target = arch.kuna_pointerargs.borrow_mut().argument(
        fd,
        fc.get_entry_address(),
        index,
        fc.final_input_storage().get(index),
        &actual,
        p,
    )?;
    if !byte_pointer(&target) {
        return None;
    }
    (actual != p.spell(&target) && actual != "void *").then_some(target)
}

fn pointer_type(
    p: &dyn PrintedPointers,
    fd: &Funcdata,
    vn: VarnodeId,
    depth: usize,
) -> Option<String> {
    if depth == 16 {
        return None;
    }
    let v = fd.vbank().get(vn)?;
    if !v.is_implied() {
        return None;
    }
    let op = fd.obank().get(v.get_def()?)?;
    match op.code() {
        OpCode::CPUI_COPY => pointer_type(p, fd, op.get_in(0)?, depth + 1),
        OpCode::CPUI_PTRSUB => {
            let base = fd.vbank().get(op.get_in(0)?)?.get_type().get_ptr_to()?;
            if base.get_metatype() != type_metatype::TYPE_SPACEBASE {
                return None;
            }
            let high = fd.vbank().get(op.get_in(1)?)?.get_high()?;
            p.address(high)
        }
        _ => None,
    }
}

pub type Entry = (i32, u64);
pub type Storage = (Entry, i32);

fn entry(addr: &kuna_base::address::Address) -> Entry {
    (
        addr.get_space().map_or(-1, |sp| sp.get_index()),
        addr.get_offset(),
    )
}

#[derive(Default)]
pub struct Batch {
    active: bool,
    declarations: BTreeMap<Entry, Vec<(kuna_base::address::Address, i32, Rc<Datatype>)>>,
    calls: BTreeMap<Entry, Vec<Call>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Call {
    pub callee: Entry,
    pub index: usize,
    pub storage: Storage,
    pub actual: String,
    pub printed: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Parameter {
    pub storage: Storage,
    pub spelling: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Record {
    pub entry: Entry,
    pub parameters: Vec<Option<Parameter>>,
    pub calls: Vec<Call>,
}

impl Record {
    pub fn needs_reprint(&self, definitions: &BTreeMap<Entry, &Record>) -> bool {
        self.calls.iter().any(|call| {
            let expected = definitions
                .get(&call.callee)
                .and_then(|d| d.parameters.get(call.index))
                .and_then(Option::as_ref)
                .filter(|p| p.storage == call.storage && p.spelling != call.actual)
                .map(|p| p.spelling.clone());
            expected != call.printed
        })
    }
}

impl Batch {
    pub fn start(&mut self) {
        *self = Self {
            active: true,
            ..Self::default()
        };
    }

    pub fn stop(&mut self) {
        *self = Self::default();
    }

    pub(crate) fn definition(&mut self, fd: &Funcdata) {
        if !self.active {
            return;
        }
        let proto = fd.get_func_proto();
        let params = (0..proto.num_params())
            .filter_map(|i| {
                let param = proto.get_param(i)?;
                Some((
                    param.get_address(),
                    param.get_size(),
                    param.get_type()?.clone(),
                ))
            })
            .collect();
        self.declarations.insert(entry(fd.get_address()), params);
        self.calls.remove(&entry(fd.get_address()));
    }

    fn target(
        &self,
        callee: Entry,
        index: usize,
        storage: &(kuna_base::address::Address, i32),
    ) -> Option<Rc<Datatype>> {
        let (addr, size, ty) = self.declarations.get(&callee)?.get(index)?;
        (addr == &storage.0 && *size == storage.1).then(|| ty.clone())
    }

    fn argument(
        &mut self,
        fd: &Funcdata,
        callee: &kuna_base::address::Address,
        index: usize,
        storage: Option<&(kuna_base::address::Address, i32)>,
        actual: &str,
        p: &dyn PrintedPointers,
    ) -> Option<Rc<Datatype>> {
        if !self.active {
            return None;
        }
        let storage = storage?;
        let callee = entry(callee);
        let target = self.target(callee, index, storage);
        let printed = target
            .as_ref()
            .and_then(|ty| conversion(ty, actual, |t| p.spell(t)));
        self.calls
            .entry(entry(fd.get_address()))
            .or_default()
            .push(Call {
                callee,
                index,
                storage: (entry(&storage.0), storage.1),
                actual: actual.to_string(),
                printed,
            });
        target
    }

    pub fn record(
        &self,
        address: &kuna_base::address::Address,
        spell: impl Fn(&Rc<Datatype>) -> String,
    ) -> Option<Record> {
        if !self.active {
            return None;
        }
        let key = entry(address);
        let parameters = self
            .declarations
            .get(&key)?
            .iter()
            .map(|(address, size, ty)| {
                byte_pointer(ty).then(|| Parameter {
                    storage: (entry(address), *size),
                    spelling: spell(ty),
                })
            })
            .collect();
        Some(Record {
            entry: key,
            parameters,
            calls: self.calls.get(&key).cloned().unwrap_or_default(),
        })
    }

    /// Definitions are used only by the printer, never by parameter recovery.
    pub fn disagreements(&self, spell: impl Fn(&Rc<Datatype>) -> String) -> Vec<u64> {
        let records: Vec<_> = self
            .declarations
            .iter()
            .map(|(key, parameters)| Record {
                entry: *key,
                parameters: parameters
                    .iter()
                    .map(|(address, size, ty)| {
                        byte_pointer(ty).then(|| Parameter {
                            storage: (entry(address), *size),
                            spelling: spell(ty),
                        })
                    })
                    .collect(),
                calls: self.calls.get(key).cloned().unwrap_or_default(),
            })
            .collect();
        let definitions = records.iter().map(|r| (r.entry, r)).collect();
        records
            .iter()
            .filter(|r| r.needs_reprint(&definitions))
            .map(|r| r.entry.1)
            .collect()
    }
}

fn conversion(
    ty: &Rc<Datatype>,
    actual: &str,
    spell: impl Fn(&Rc<Datatype>) -> String,
) -> Option<String> {
    if !byte_pointer(ty) || actual == "void *" {
        return None;
    }
    let expected = spell(ty);
    (actual != expected).then_some(expected)
}

/// C character pointers may access an object's representation without
/// changing its effective type or invoking incompatible-type aliasing.
fn byte_pointer(ty: &Rc<Datatype>) -> bool {
    ty.get_metatype() == type_metatype::TYPE_PTR
        && ty.get_ptr_to().is_some_and(|p| {
            p.get_size() == 1
                && matches!(
                    p.get_metatype(),
                    type_metatype::TYPE_INT | type_metatype::TYPE_UINT
                )
        })
}

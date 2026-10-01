//! Cast the address of a scalar local passed where a callee printed earlier in
//! the same callee-first batch declares a character pointer.

use std::collections::BTreeMap;
use std::rc::Rc;

use crate::context::{HighVariableId, OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use kuna_base::address::Address;
use kuna_num::opcodes::OpCode;

/// The declared type of each scalar parameter and local the printer wrote.
#[derive(Default)]
pub(crate) struct Declarations(BTreeMap<String, String>);

impl Declarations {
    pub(crate) fn clear(&mut self) {
        self.0.clear();
    }

    pub(crate) fn record(&mut self, name: &str, front: &str, back: &str, array: bool) {
        if back.is_empty() && !array {
            self.0.insert(name.to_string(), front.to_string());
        } else {
            self.0.remove(name);
        }
    }

    pub(crate) fn address(&self, name: &str) -> Option<String> {
        self.0.get(name).map(|base| pointer_to(base))
    }
}

fn pointer_to(base: &str) -> String {
    format!("{base}{}*", if base.ends_with('*') { "" } else { " " })
}

pub(crate) trait PrintedPointers {
    fn spell(&self, ty: &Rc<Datatype>) -> String;
    fn address(&self, high: HighVariableId) -> Option<String>;
}

/// A printed parameter's storage, size and declared type.
type Parameter = (Address, i32, Rc<Datatype>);

/// The parameter declarations of the functions a callee-first batch has
/// printed so far, keyed by entry point.
#[derive(Default)]
pub struct Batch {
    active: bool,
    declarations: BTreeMap<(i32, u64), Vec<Parameter>>,
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
    }

    fn target(
        &self,
        callee: &Address,
        index: usize,
        storage: &(Address, i32),
    ) -> Option<Rc<Datatype>> {
        if !self.active {
            return None;
        }
        let (addr, size, ty) = self.declarations.get(&entry(callee))?.get(index)?;
        (addr == &storage.0 && *size == storage.1).then(|| ty.clone())
    }
}

fn entry(addr: &Address) -> (i32, u64) {
    (
        addr.get_space().map_or(-1, |sp| sp.get_index()),
        addr.get_offset(),
    )
}

/// The character pointer to cast argument `slot` of `call` to, if any.
///
/// Only a declaration already printed in this batch, at the same position and
/// finalized storage, decides; varargs and per-call overrides never do. An
/// argument that differs from it only in the signedness of a byte keeps its
/// own type.
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
    let index = (slot - 1) as usize;
    let storage = fc.final_input_storage().get(index)?.clone();
    let target = arch
        .kuna_pointerargs
        .borrow()
        .target(fc.get_entry_address(), index, &storage)?;
    if !byte_pointer(&target) {
        return None;
    }
    let actual = pointer_type(p, fd, op.get_in(slot)?, 0)?;
    if actual == p.spell(&target) || byte_spellings(p, arch).any(|b| pointer_to(&b) == actual) {
        return None;
    }
    Some(target)
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
            p.address(fd.vbank().get(op.get_in(1)?)?.get_high()?)
        }
        _ => None,
    }
}

/// Every spelling a one-byte integer or undefined byte takes in this run.
fn byte_spellings<'a>(
    p: &'a dyn PrintedPointers,
    arch: &'a crate::architecture::Architecture,
) -> impl Iterator<Item = String> + 'a {
    let types = arch.types();
    let factory = [
        types.get_base(1, type_metatype::TYPE_INT),
        types.get_base_no_char(1, type_metatype::TYPE_INT),
        types.get_base(1, type_metatype::TYPE_UINT),
        types.get_base(1, type_metatype::TYPE_UNKNOWN),
        types.get_type_char(1),
    ];
    ["char", "signed char", "unsigned char", "undefined1"]
        .into_iter()
        .map(String::from)
        .chain(factory.into_iter().flatten().map(move |ty| p.spell(&ty)))
}

/// C character pointers may access an object's representation without
/// changing its effective type; an undefined byte prints as one.
fn byte_pointer(ty: &Rc<Datatype>) -> bool {
    ty.get_metatype() == type_metatype::TYPE_PTR
        && ty.get_ptr_to().is_some_and(|p| {
            p.get_size() == 1
                && matches!(
                    p.get_metatype(),
                    type_metatype::TYPE_INT
                        | type_metatype::TYPE_UINT
                        | type_metatype::TYPE_UNKNOWN
                )
        })
}

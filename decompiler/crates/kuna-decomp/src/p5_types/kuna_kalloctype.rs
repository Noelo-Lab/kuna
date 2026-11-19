//! (kuna `xnutables`) What a typed allocation returns, from its view.
//!
//! XNU's typed allocators (`kalloc_type_impl`, `IOMallocTypeImpl`,
//! `OSObject_typed_operator_new`, ...) take a `kalloc_type_view` whose
//! `kt_signature` describes the allocated type one 8-byte granule per
//! character, and return a `void *`. The XNU table analysis
//! (`kuna-analysis/src/analyzers/iokit/tables.rs`) reads every view and hands
//! the engine the map from view to what it allocates ([`KallocTypes`]); the
//! struct is interned on the first call that needs it ([`view_struct`]).
//! [`allocated_type`] answers `TypeOpCall::getOutputLocal` for a call to one of the allocators whose view argument folds to a view in
//! that map, so `v = OSObject_typed_operator_new(IOSurfaceClient_ktv, 0x98)`
//! is an `IOSurfaceClient *` and the stores into it are its fields.

use std::collections::HashMap;
use std::rc::Rc;

use crate::context::{OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype, TypeFactory, TypeField};
use crate::funcdata::Funcdata;
use kuna_num::opcodes::OpCode;
use crate::p4_calls::fspec::FuncCallSpecs;

/// XNU's typed allocator entry points: (name, returns the allocation,
/// parameter count). The first parameter is the view.
pub const TYPED_ALLOCATORS: &[(&str, bool, usize)] = &[
    ("kalloc_type_impl", true, 2),
    ("kalloc_type_impl_external", true, 2),
    ("kfree_type_impl_external", false, 2),
    ("kalloc_type_var_impl", true, 4),
    ("kfree_type_var_impl", false, 3),
    ("IOMallocTypeImpl", true, 1),
    ("IOFreeTypeImpl", false, 2),
    ("IOMallocTypeVarImpl", true, 2),
    ("IOFreeTypeVarImpl", false, 3),
    ("OSObject_typed_operator_new", true, 2),
    ("OSObject_typed_operator_delete", false, 3),
];

/// What one view allocates: a struct named `name` laid out from `signature`
/// (one 8-byte granule per character: `1` a pointer, `8` a signed pointer, `2`
/// data, `4` either, `0` padding) over `size` bytes, or for a variable view
/// whose element is a pointer, an array of pointers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KallocView {
    pub name: String,
    pub signature: String,
    pub size: u32,
    /// A libkern class: its first pointer is the `vtable`.
    pub is_class: bool,
    /// The allocation is an array of pointers (`site.T *` in a var view).
    pub pointers: bool,
}

/// View address -> what the view allocates.
pub type KallocTypes = HashMap<u64, KallocView>;

/// The struct a view allocates, interned the first time a call needs it, so
/// a view no decompiled function allocates through adds no type.
pub fn view_struct(types: &dyn TypeFactory, v: &KallocView) -> Option<Rc<Datatype>> {
    if v.size == 0 || v.size > 0x1000_0000 {
        return None;
    }
    let size = v.size as i32;
    if let Some(existing) = types.find_by_name(&v.name).ok()? {
        let same = existing.get_metatype() == type_metatype::TYPE_STRUCT && !existing.is_incomplete() && existing.get_size() == size;
        return same.then_some(existing);
    }
    let ptr = types.get_type_pointer(8, types.get_type_void().ok()?, 1).ok()?;
    let word = types.get_base(8, type_metatype::TYPE_UNKNOWN).ok()?;
    let byte = types.get_base(1, type_metatype::TYPE_UNKNOWN).ok()?;
    let mut fields = Vec::new();
    let mut granules = v.signature.chars();
    let mut off = 0i32;
    while off < size {
        let kind = granules.next().unwrap_or('2');
        let len = (size - off).min(8);
        if kind == '0' {
            off += len;
            continue;
        }
        let ty = match kind {
            '1' | '8' if len == 8 => Rc::clone(&ptr),
            _ if len == 8 => Rc::clone(&word),
            _ => types.get_type_array(len, Rc::clone(&byte)).ok()?,
        };
        let field = if off == 0 && v.is_class && kind == '1' { "vtable".to_string() } else { format!("field_0x{off:x}") };
        fields.push(TypeField::new(fields.len() as i32, off, field, ty));
        off += len;
    }
    let align = if size % 8 == 0 { 8 } else if size % 4 == 0 { 4 } else { 1 };
    let shell = types.get_type_struct(&v.name).ok()?;
    types.set_fields_struct_raw(&shell, fields, Vec::new(), size, align, 0).ok()
}

/// `TypeOpCall::getOutputLocal` for a typed allocation: a pointer to what the
/// view the call passes allocates, when that view is known.
pub fn allocated_type(data: &Funcdata, op: OpId, fc: &FuncCallSpecs) -> Option<Rc<Datatype>> {
    let arch = data.get_arch();
    if arch.kuna_kalloc_types.is_empty() {
        return None;
    }
    let name = fc.get_name();
    TYPED_ALLOCATORS.iter().find(|(n, returns, _)| *returns && *n == name)?;
    let view = data.obank().get(op)?.get_in(1)?;
    let v = arch.kuna_kalloc_types.get(&constant_address(data, view, 0)?)?;
    let types = arch.types()?;
    let space = arch.manage.get_default_data_space()?;
    let (ptr_size, word) = (space.get_addr_size() as i32, space.get_word_size());
    let target = if v.pointers {
        types.get_type_pointer(ptr_size, types.get_type_void().ok()?, word).ok()?
    } else {
        view_struct(types, v)?
    };
    types.get_type_pointer(ptr_size, target, word).ok()
}

/// The address a varnode names: a constant, `&sym` as `PTRSUB(0, addr)`, either
/// through the copies and casts the printer's own passes wrap it in.
fn constant_address(data: &Funcdata, vn: VarnodeId, depth: u32) -> Option<u64> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn view_struct_is_built_once_and_lays_out_granules() {
        let types = &crate::dtype::TypeFactoryImpl::new();
        types.set_default_alignment_map();
        types.set_max_basetype_size(8);
        types.setup_sizes(Some(8), 8, 4);
        types.cache_core_types().unwrap();
        let v = KallocView { name: "Widget".into(), signature: "12".into(), size: 0x14, is_class: true, pointers: false };
        assert!(types.find_by_name("Widget").unwrap().is_none(), "nothing is interned before use");
        let st = view_struct(types, &v).unwrap();
        assert_eq!(st.get_size(), 0x14);
        assert!(Rc::ptr_eq(&st, &view_struct(types, &v).unwrap()));
        let other = KallocView { size: 0x20, ..v };
        assert!(view_struct(types, &other).is_none(), "a different size never reuses the name");
    }
}

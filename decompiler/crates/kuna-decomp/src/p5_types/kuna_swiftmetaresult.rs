//! (kuna `swifttypes`) Type the result of every call to a Swift type
//! metadata accessor as `swift.type_metadata *`; a `sitetypelock` client.
//!
//! An accessor (`Module.Type.type_metadata_accessor`, the demangled `…Ma`
//! of an import stub or the name `swiftmetadata` gives a local one) returns
//! the type's metadata in the first result register whatever arguments it
//! takes: the request alone for a non-generic type, the request and the
//! generic arguments for a generic one. A prototype would have to state
//! those arguments, which an import's name does not tell, so only the
//! result is locked, at each call site, and the arguments stay recovered.
//! The same holds for the instantiation helpers `swifthelpers` names
//! (`__swift_instantiateConcreteTypeFromMangledName…`), whose prototype
//! returns `void *`. A call whose output is already type-locked is left as
//! it is.
//!
//! Refusals, each leaving the site untouched (the shared `sitetypelock`
//! refusals apply on top): no `swift.type_metadata` record in the type
//! factory (the metadata pass did not run), a result wider or narrower than
//! a pointer (a recovered register pair), a result joined through a phi
//! with a value of other provenance (`Funcdata::site_value_is_pure`), and a
//! result that is the base of an address computation (the value witness
//! table below the record, a field offset vector past it).

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_num::opcodes::OpCode;

use crate::dtype::Datatype;
use crate::funcdata::Funcdata;

const ACCESSOR: &str = ".type_metadata_accessor";
const INSTANTIATE: &str = "__swift_instantiateConcreteTypeFromMangledName";

fn returns_metadata(callee: &str) -> bool {
    callee.ends_with(ACCESSOR) || callee.starts_with(INSTANTIATE)
}

/// Whether `vn` is the base of an address computation (`vn + c`, `vn - c`):
/// the value witness table load 8 bytes below the record, a field offset
/// or generic argument past it. Typed `swift.type_metadata *`, such a
/// read would render as an element or field of the 16-byte record it is
/// not part of, so only a result that is passed on or stored is locked.
fn has_offset_use(data: &Funcdata, vn: crate::context::VarnodeId) -> bool {
    let mut seen = std::collections::HashSet::new();
    let mut work = vec![vn];
    while let Some(vn) = work.pop() {
        if !seen.insert(vn) {
            continue;
        }
        let Some(v) = data.vbank().get(vn) else { return true };
        for op in v.descend_iter() {
            let Some(o) = data.obank().get(op) else { continue };
            if is_offset(data, o) {
                return true;
            }
            // The same value carried on: a copy, a register kept across a
            // call, a join.
            if matches!(o.code(), OpCode::CPUI_COPY | OpCode::CPUI_INDIRECT | OpCode::CPUI_MULTIEQUAL) {
                work.extend(o.get_out());
            }
        }
    }
    false
}

fn is_offset(data: &Funcdata, o: &crate::op::PcodeOp) -> bool {
    match o.code() {
        OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_SUB => (0..2).any(|i| {
            o.get_in(i).and_then(|c| data.vbank().get(c)).is_some_and(|c| c.is_constant())
        }),
        OpCode::CPUI_PTRSUB | OpCode::CPUI_PTRADD => true,
        _ => false,
    }
}

/// Register a per-site output type lock for every metadata accessor call.
/// Idempotent.
pub fn contribute(data: &mut Funcdata) {
    let arch = Rc::clone(data.get_arch());
    if !arch.swift_metadata_results {
        return;
    }
    if !(0..data.num_calls()).any(|i| returns_metadata(data.get_call_specs(i).get_name())) {
        return;
    }
    let Some(tlst) = arch.types() else { return };
    let Ok(Some(meta)) = tlst.find_by_name("swift.type_metadata") else { return };
    let Some(space) = arch.manage().get_default_data_space() else { return };
    let width = space.get_addr_size() as i32;
    let Ok(ptr) = tlst.get_type_pointer(width, meta, space.get_word_size()) else { return };
    let mut sites: Vec<(Address, Rc<Datatype>)> = Vec::new();
    for i in 0..data.num_calls() {
        let fc = data.get_call_specs(i);
        if !returns_metadata(fc.get_name()) {
            continue;
        }
        let Some(o) = data.obank().get(fc.get_op()) else { continue };
        let Some(out) = o.get_out() else { continue };
        let Some(v) = data.vbank().get(out) else { continue };
        if v.get_size() != width
            || v.is_type_lock()
            || !data.site_value_is_pure(out)
            || has_offset_use(data, out)
        {
            continue;
        }
        sites.push((o.get_addr().clone(), Rc::clone(&ptr)));
    }
    for (addr, ct) in sites {
        data.get_override_mut().insert_output_typelock(addr, ct);
    }
}

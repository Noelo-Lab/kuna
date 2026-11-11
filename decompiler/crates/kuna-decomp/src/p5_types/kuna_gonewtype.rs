//! **gonewtype** — type a `runtime.newobject` result from the descriptor it
//! allocates (`docs/multilang.md`, the Go pack; the first `sitetypelock`
//! client).
//!
//! ```text
//!   long lVar1;                                 main.Point *pVar1;
//!   lVar1 = runtime.newobject                   pVar1 = runtime.newobject
//!       (&type:main.Point);          -->            (&type:main.Point);
//!   *(long *)(lVar1 + 8) = 3;                   pVar1->Y = 3;
//! ```
//!
//! Go's allocators take the runtime descriptor of the type they allocate:
//! `runtime.newobject(typ *_type) unsafe.Pointer` returns a fresh zeroed
//! object OF that type, and newer compilers (go1.26+) inline the same call
//! into the size-class `runtime.mallocgc*` entries
//! (`mallocgcSmallScanNoHeaderSC4(size, typ, needzero)`, ...), which share
//! `mallocgc`'s signature with the descriptor second.  The `gotypes` harvest
//! interns the program's own named record layouts (`main.Point`) and
//! `gotypeglobals` names each descriptor record (`type:main.Point`), so at
//! any allocator site whose descriptor argument resolves to such a symbol the
//! result's type is a fact stated by the binary's own metadata, not an
//! inference.  This module reads that fact and asserts it through the
//! per-call-site output type lock ([`crate::kuna_sitetypelock`]), which is
//! what makes it stick: an unlocked vote at the same seam is overridden by
//! `typeOrder` folding and `ActionStructSynth` before the declaration prints.
//!
//! Refusals, each leaving the site untouched (the shared `sitetypelock`
//! refusals — no output Varnode, output already type-locked, size mismatch —
//! apply on top):
//!
//!   * the descriptor argument is missing or does not resolve to a constant
//!     address (a dynamic descriptor is somebody's reflection, not a stated
//!     allocation type);
//!   * the constant does not point at the START of a `type:<name>` data
//!     symbol;
//!   * `<name>` is not held by the type factory as a COMPLETE named record —
//!     pointer/array/primitive spellings (`*main.T`, `[]uint8`, `int`) and
//!     empty shells refuse, so a lock never blocks a later, better recovery
//!     (`structsynth` can still complete a layout over an unlocked result);
//!   * a mallocgc-family site whose explicit size argument is not a constant
//!     equal to the record's size (the strongest available cross-check that
//!     THIS record is what the site allocates).

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::context::OpId;
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;

/// The allocator whose descriptor argument states its result type.
const NEWOBJECT: &str = "runtime.newobject";

/// The CALL input slot holding the `*_type` descriptor for a Go allocator
/// entry point, or `None` for any other callee.  `runtime.newobject(typ)`
/// carries it first; `runtime.mallocgc(size, typ, needzero)` and the
/// size-class specializations newer compilers inline it into
/// (`runtime.mallocgcSmallScanNoHeaderSC4`, `runtime.mallocgcTiny`,
/// `runtime.mallocgcLarge`, ...) share the mallocgc signature, so the
/// descriptor sits second and the suffix is held to plain alphanumerics.
fn descriptor_slot(name: &str) -> Option<int4> {
    if name == NEWOBJECT {
        return Some(1);
    }
    let rest = name.strip_prefix("runtime.mallocgc")?;
    rest.bytes().all(|b| b.is_ascii_alphanumeric()).then_some(2)
}

/// Register a per-site output type lock for every qualifying Go allocator
/// call (option `gonewtype`).  Idempotent.
pub fn contribute(data: &mut Funcdata) {
    if !data.get_arch().go_new_type {
        return;
    }
    let mut sites: Vec<(Address, Rc<Datatype>)> = Vec::new();
    for i in 0..data.num_calls() {
        let fc = data.get_call_specs(i);
        let Some(slot) = descriptor_slot(fc.get_name()) else { continue };
        let op = fc.get_op();
        if let Some(site) = resolve_site(data, op, slot) {
            sites.push(site);
        }
    }
    for (addr, ct) in sites {
        data.get_override_mut().insert_output_typelock(addr, ct);
    }
}

/// Resolve one allocator CALL to `(call address, *T lock type)`, or refuse.
/// `slot` is the descriptor argument's input slot ([`descriptor_slot`]); the
/// mallocgc family (slot 2) states the allocation size explicitly in slot 1,
/// which must be a constant equal to the record's size.
fn resolve_site(data: &Funcdata, op: OpId, slot: int4) -> Option<(Address, Rc<Datatype>)> {
    let o = data.obank().get(op)?;
    let out_size = data.vbank().get(o.get_out()?)?.get_size();
    let desc = descriptor_offset(data, o.get_in(slot)?)?;
    let arch = Rc::clone(data.get_arch());
    let spc = Rc::clone(arch.manage().get_default_data_space()?);
    let addr = Address::new(spc, desc);
    let (name, sym_off, _) = arch.name_for_global_varnode(&addr, 1, &Address::default())?;
    if sym_off != 0 {
        return None;
    }
    let spelled = name.strip_prefix("type:")?;
    if spelled.is_empty() {
        return None;
    }
    let tlst = arch.types()?;
    let record = tlst.find_by_name(spelled).ok()??;
    if record.get_metatype() != type_metatype::TYPE_STRUCT || record.get_size() == 0 {
        return None;
    }
    if slot == 2 {
        let size = descriptor_offset(data, o.get_in(1)?)?;
        if size != record.get_size() as u64 {
            return None;
        }
    }
    let wordsize =
        arch.manage().get_default_data_space().map(|s| s.get_word_size()).unwrap_or(1);
    let ptr = tlst.get_type_pointer(out_size, record, wordsize).ok()?;
    Some((o.get_addr().clone(), ptr))
}

/// The constant address the descriptor argument carries: the Varnode itself,
/// or one COPY away (the lifted `lea`).
fn descriptor_offset(data: &Funcdata, vn: crate::context::VarnodeId) -> Option<u64> {
    let v = data.vbank().get(vn)?;
    if v.is_constant() {
        return Some(v.get_offset());
    }
    let def = v.get_def()?;
    let d = data.obank().get(def)?;
    if d.code() != OpCode::CPUI_COPY {
        return None;
    }
    let src = data.vbank().get(d.get_in(0)?)?;
    src.is_constant().then(|| src.get_offset())
}

#[cfg(test)]
#[path = "kuna_gonewtype/tests.rs"]
mod tests;

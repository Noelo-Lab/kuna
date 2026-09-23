//! (kuna `globalref`) A constant address used as a pointer prints as the global
//! it names.
//!
//! `PrintC::pushConstant`'s pointer arm renders a pointer-typed constant as a
//! forced-hex integer behind a typecast, `sub_e4ca((struct_2 *)0x2b080)`.
//! Upstream reaches a symbol only through `ActionConstantPtr`, which links a
//! constant to a global Symbol that already exists; a stripped image carries none
//! for most of its data, so every address a callee's recovered pointer type
//! reaches keeps the cast. The type campaign made that more common, not less:
//! the more parameters are typed as pointers, the more constant arguments are
//! typed as pointers too.
//!
//! When on, such a constant prints as `&dat_<addr>` -- the name kuna already
//! gives an unnamed global it reads directly -- and the function records the
//! global as an object of the pointed-to type, which the `decompile-project`
//! header declares. The expression has exactly the constant's pointer type, so
//! the cast is gone because the object is declared at that type, and its value
//! is the address it replaces wherever `dat_<addr>` is placed at `<addr>`.
//!
//! The rewrite is refused where the name would claim something the function
//! does not show: an address outside every section the loader classifies as
//! holding program data (code, TLS templates, the run-time loader's tables, or
//! no section at all); an address a global Symbol already covers; a value the
//! same function also reads as a number (an ordering compare, a multiply, a
//! divide, a shift on a non-pointer); an object this function also reads or
//! writes directly at another width or type; a pointer to code; and, when the
//! pointed-to type is `void`, any reader for which C's implicit conversion from
//! the declared object's pointer is not exactly the conversion the cast
//! performed.

use std::collections::{BTreeMap, HashSet};
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_base::types::uintb;
use kuna_num::opcodes::OpCode;

use crate::architecture::Architecture;
use crate::context::{OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype, TypeFactory};
use crate::funcdata::Funcdata;
use crate::options::on_or_off;

/// `globalref on|off`.
#[derive(Debug, Clone, Copy, Default)]
pub struct OptionGlobalRef;

impl OptionGlobalRef {
    /// The option name.
    pub const NAME: &'static str = "globalref";

    /// Parse + validate the `on`/`off` value; the caller performs the printer
    /// write ([`crate::printc::PrintCOptions::set_global_ref`]).
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("Constant-address global references turned {prop}")))
    }
}

/// One global a function's C names by address.
#[derive(Debug, Clone)]
pub struct Minted {
    /// The type the function uses the object at.
    pub decl_type: Rc<Datatype>,
    /// True when the pointer pointed at `void`, so `decl_type` is the unknown
    /// byte standing in for an object whose type this function does not know.
    pub unknown: bool,
}

/// A direct access this function makes to global storage.
#[derive(Debug, Clone)]
struct Direct {
    start: u64,
    size: u64,
    ty: Rc<Datatype>,
    /// No global Symbol covers it, so the body prints it as `dat_<start>`.
    unnamed: bool,
}

/// What one function reads an address at.
#[derive(Debug, Clone)]
enum Seen {
    /// Only ever through `void *`.
    Void(Rc<Datatype>),
    /// One pointed-to type other than `void`, perhaps also through `void *`.
    One(Rc<Datatype>),
    /// Two different pointed-to types other than `void`.
    Conflict,
}

impl Seen {
    fn merge(prev: Option<Seen>, to: Rc<Datatype>) -> Seen {
        let is_void = to.get_metatype() == type_metatype::TYPE_VOID;
        match prev {
            None | Some(Seen::Void(_)) if is_void => Seen::Void(to),
            None | Some(Seen::Void(_)) => Seen::One(to),
            Some(Seen::One(t)) if is_void || same_type(&t, &to) => Seen::One(t),
            _ => Seen::Conflict,
        }
    }

    fn object(&self) -> Option<Rc<Datatype>> {
        match self {
            Seen::Void(t) | Seen::One(t) => Some(Rc::clone(t)),
            Seen::Conflict => None,
        }
    }
}

/// The per-function state: the facts every decision reads, computed once when
/// the function's document starts, and the globals it has named so far.
#[derive(Debug, Default)]
pub struct Plan {
    on: bool,
    trace: bool,
    numeric: HashSet<uintb>,
    direct: Vec<Direct>,
    /// The pointed-to types each in-range constant address is read at.
    pointees: BTreeMap<u64, Seen>,
    /// Every global this function's C names, by address.
    pub minted: BTreeMap<u64, Minted>,
}

/// Build the plan for `fd`. Off (and empty) unless the option is on, the image
/// reported data sections, and the output language is C.
pub fn plan(fd: &Funcdata, arch: &Architecture, on: bool, is_c: bool) -> Plan {
    if !on || !is_c || arch.globalref_ranges.is_empty() {
        return Plan::default();
    }
    let data_space = arch.manage().get_default_data_space().map(|s| s.get_index());
    let mut numeric = HashSet::new();
    let mut direct = Vec::new();
    let mut pointees: BTreeMap<u64, Seen> = BTreeMap::new();
    for vn in fd.vbank().iter_loc() {
        let Some(v) = fd.vbank().get(vn) else { continue };
        if v.is_constant() {
            let off = v.get_offset();
            for op in v.descend_iter() {
                let Some(code) = fd.obank().get(op).map(|o| o.code()) else { continue };
                let ct = v.get_type_read_facing(op);
                if ct.get_metatype() != type_metatype::TYPE_PTR {
                    if reads_as_number(code) {
                        numeric.insert(off);
                    }
                    continue;
                }
                if !in_ranges(&arch.globalref_ranges, off) {
                    continue;
                }
                let Some(to) = ct.get_ptr_to() else { continue };
                let seen = pointees.remove(&off);
                pointees.insert(off, Seen::merge(seen, to));
            }
        } else if data_space.is_some() && v.get_addr().get_space().map(|s| s.get_index()) == data_space {
            let unnamed = in_ranges(&arch.globalref_ranges, v.get_offset())
                && fd.get_arch().query_container_global(v.get_addr(), v.get_size(), &Address::new_invalid()).is_none();
            direct.push(Direct {
                start: v.get_offset(),
                size: v.get_size().max(1) as u64,
                ty: Rc::clone(v.get_type()),
                unnamed,
            });
        }
    }
    let trace = std::env::var_os("KUNA_GLOBALREF_TRACE").is_some();
    Plan { on: true, trace, numeric, direct, pointees, minted: BTreeMap::new() }
}

/// Does `opcode` read its operand as a number rather than as an address?
fn reads_as_number(opcode: OpCode) -> bool {
    crate::kuna_inferfuncentry::reads_as_integer(opcode)
}

impl Plan {
    /// Decide whether the pointer-typed constant `vn` (value `off`, read by
    /// `op` at type `ct`) names a global, and record it. Returns the address to
    /// name, or `None` to keep the upstream cast.
    pub fn decide(
        &mut self,
        fd: &Funcdata,
        arch: &Architecture,
        vn: VarnodeId,
        op: OpId,
        off: uintb,
        ct: &Rc<Datatype>,
    ) -> Option<u64> {
        if !self.on {
            return None;
        }
        let verdict = self.judge(fd, arch, vn, op, off, ct);
        if self.trace {
            eprintln!(
                "[globalref] {} 0x{off:x} {} {}",
                fd.get_name(),
                ct.get_name(),
                match &verdict {
                    Ok(()) => "named",
                    Err(why) => why.as_str(),
                }
            );
        }
        verdict.ok().map(|()| off)
    }

    fn judge(
        &mut self,
        fd: &Funcdata,
        arch: &Architecture,
        vn: VarnodeId,
        op: OpId,
        off: uintb,
        ct: &Rc<Datatype>,
    ) -> Result<(), Refusal> {
        if off == 0 || ct.get_metatype() != type_metatype::TYPE_PTR || ct.get_word_size().unwrap_or(1) != 1 {
            return Err(Refusal::NotAPointer);
        }
        if fd.vbank().get(vn).map(|v| v.get_size()) != Some(ct.get_size()) {
            return Err(Refusal::NotAPointer);
        }
        let pointee = ct.get_ptr_to().ok_or(Refusal::NotAPointer)?;
        if matches!(pointee.get_metatype(), type_metatype::TYPE_CODE | type_metatype::TYPE_SPACEBASE) {
            return Err(Refusal::Code);
        }
        if !in_ranges(&arch.globalref_ranges, off) {
            return Err(Refusal::OutsideData);
        }
        if self.numeric.contains(&off) {
            return Err(Refusal::Numeric);
        }
        let object = self.pointees.get(&off).ok_or(Refusal::NotAPointer)?.object().ok_or(Refusal::TwoTypes)?;
        if matches!(object.get_metatype(), type_metatype::TYPE_CODE | type_metatype::TYPE_SPACEBASE) {
            return Err(Refusal::Code);
        }
        let unknown = object.get_metatype() == type_metatype::TYPE_VOID;
        if pointee.get_metatype() == type_metatype::TYPE_VOID && !void_reader_converts(fd, op, vn) {
            return Err(Refusal::VoidReader);
        }
        let space = Rc::clone(arch.manage().get_default_data_space().ok_or(Refusal::OutsideData)?);
        let addr = Address::new(space, off);
        let invalid = Address::new_invalid();
        if fd.get_arch().query_container_global(&addr, 1, &invalid).is_some() {
            return Err(Refusal::Symbol);
        }
        let decl_type = if !unknown {
            object
        } else if let Some(read) = self.direct_type_at(off) {
            read
        } else {
            arch.types_impl().get_base(1, type_metatype::TYPE_UNKNOWN).map_err(|_| Refusal::NotAPointer)?
        };
        let size = decl_type.get_size().max(1) as u64;
        if let Some(d) = self.direct.iter().find(|d| overlaps(d, off, size) && !same_object(d, off, size, &decl_type)) {
            if self.trace {
                eprintln!(
                    "[globalref-direct] {} 0x{off:x} decl={}:{} direct=0x{:x}:{}:{}",
                    fd.get_name(),
                    decl_type.get_name(),
                    size,
                    d.start,
                    d.size,
                    d.ty.get_name()
                );
            }
            return Err(Refusal::DirectAccess);
        }
        self.minted.entry(off).or_insert(Minted { decl_type, unknown });
        Ok(())
    }
}

impl Plan {
    /// The unnamed program data this function reads or writes directly, by
    /// start and type: the `dat_<addr>` it prints that are not addresses taken.
    pub fn direct_objects(&self) -> impl Iterator<Item = (u64, &Rc<Datatype>)> + '_ {
        self.direct.iter().filter(|d| d.unnamed).map(|d| (d.start, &d.ty))
    }

    /// The one type this function reads or writes the storage at `off` at,
    /// when every direct access that touches it starts there at that type.
    fn direct_type_at(&self, off: u64) -> Option<Rc<Datatype>> {
        let mut found: Option<&Direct> = None;
        for d in self.direct.iter().filter(|d| overlaps(d, off, 1)) {
            match found {
                None if d.start == off => found = Some(d),
                Some(f) if d.start == off && d.size == f.size && same_type(&d.ty, &f.ty) => {}
                _ => return None,
            }
        }
        found.map(|d| Rc::clone(&d.ty))
    }
}

/// Why a pointer-typed constant keeps its cast.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Refusal {
    /// Not a byte-addressed data pointer the size of the constant.
    NotAPointer,
    /// A pointer to code.
    Code,
    /// No section the loader classifies as program data holds the address.
    OutsideData,
    /// The function also reads the value as a number.
    Numeric,
    /// The function uses the address at two different pointed-to types.
    TwoTypes,
    /// A `void *` reader for which the declared object's pointer does not convert.
    VoidReader,
    /// A global Symbol already covers the address.
    Symbol,
    /// The function reads or writes the storage directly at another width or type.
    DirectAccess,
}

impl Refusal {
    /// The trace spelling.
    pub fn as_str(self) -> &'static str {
        match self {
            Refusal::NotAPointer => "not-a-pointer",
            Refusal::Code => "code",
            Refusal::OutsideData => "outside-data",
            Refusal::Numeric => "numeric",
            Refusal::TwoTypes => "two-types",
            Refusal::VoidReader => "void-reader",
            Refusal::Symbol => "symbol",
            Refusal::DirectAccess => "direct-access",
        }
    }
}

/// Is `off` inside one of the sorted, inclusive `ranges`?
pub fn in_ranges(ranges: &[(u64, u64)], off: u64) -> bool {
    let idx = ranges.partition_point(|&(lo, _)| lo <= off);
    idx > 0 && ranges[idx - 1].1 >= off
}

fn overlaps(d: &Direct, off: u64, size: u64) -> bool {
    d.start < off.saturating_add(size) && off < d.start.saturating_add(d.size)
}

/// Is the direct access `d` a read or write of exactly the object `off` names,
/// at a type that spells the same C type as `ty`? A wider unknown is the
/// unsigned integer of its size on every surface; a one-byte unknown is `char`
/// on one and `unsigned char` on another, so it matches only itself.
fn same_object(d: &Direct, off: u64, size: u64, ty: &Rc<Datatype>) -> bool {
    if d.start != off || d.size != size {
        return false;
    }
    same_type(&d.ty, ty)
        || (d.ty.get_metatype() == type_metatype::TYPE_UNKNOWN
            && ty.get_metatype() == type_metatype::TYPE_UINT
            && ty.get_size() == d.ty.get_size()
            && ty.get_size() > 1)
}

fn same_type(a: &Rc<Datatype>, b: &Rc<Datatype>) -> bool {
    Rc::ptr_eq(a, b) || a.type_order(b).map(|o| o == 0).unwrap_or(false)
}

/// Is `ct` a pointer to `void`?
fn is_void_pointer(ct: &Datatype) -> bool {
    ct.get_metatype() == type_metatype::TYPE_PTR
        && ct.get_ptr_to().is_some_and(|p| p.get_metatype() == type_metatype::TYPE_VOID)
}

/// For a `void *` constant: does the reader convert any object pointer to
/// `void *` exactly as the cast did? An argument, a returned value, a value
/// stored or copied into `void *` storage, and an equality test against another
/// `void *` do; anything else keeps the cast.
fn void_reader_converts(fd: &Funcdata, op: OpId, vn: VarnodeId) -> bool {
    let Some(o) = fd.obank().get(op) else { return false };
    let slot = o.get_slot(vn);
    let ty_of = |v: Option<VarnodeId>| v.and_then(|v| fd.vbank().get(v)).map(|v| Rc::clone(v.get_type()));
    match o.code() {
        OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_RETURN => slot >= 1,
        OpCode::CPUI_COPY => ty_of(o.get_out()).is_some_and(|t| is_void_pointer(&t)),
        OpCode::CPUI_STORE => {
            slot == 2
                && ty_of(o.get_in(1)).is_some_and(|t| {
                    t.get_metatype() == type_metatype::TYPE_PTR
                        && t.get_ptr_to().is_some_and(|p| is_void_pointer(&p))
                })
        }
        OpCode::CPUI_INT_EQUAL | OpCode::CPUI_INT_NOTEQUAL => {
            let other = o.get_in(1 - slot);
            other
                .and_then(|v| fd.vbank().get(v))
                .is_some_and(|v| is_void_pointer(v.get_type_read_facing(op)))
        }
        _ => false,
    }
}

#[cfg(test)]
#[path = "kuna_globalref/tests.rs"]
mod tests;

//! Whole mapped objects behind named global references actually printed in C.
use std::collections::BTreeMap;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_num::opcodes::OpCode;

use crate::architecture::Architecture;
use crate::context::VarnodeId;
use crate::dtype::{type_metatype, Datatype, TypeFactory};
use crate::funcdata::Funcdata;

#[derive(Debug, Default)]
struct Accesses {
    exact: Vec<Rc<Datatype>>,
    agnostic: bool,
    writes: Vec<Rc<Datatype>>,
}

#[derive(Debug, Default)]
pub struct References {
    objects: BTreeMap<(u64, String), Rc<Datatype>>,
    accesses: BTreeMap<(u64, String), Accesses>,
}

impl References {
    /// A whole-width read or write of a mapped scalar at `address`, `value`
    /// being the varnode read or written there, compiles against the type the
    /// printer gave that varnode, and a declaration at the symbol's own type
    /// (`undefined4` is `unsigned int`) would silently change what a signed
    /// compare, a shift, a division or a float computes. `loaded` marks the
    /// output of a `LOAD` through `&name`, which, when explicit, is assigned to
    /// a variable of its own; the global's storage or a register merged into
    /// it is read as the name wherever it is used, however it was defined.
    #[allow(clippy::too_many_arguments)]
    pub fn record_access(
        &mut self,
        fd: &Funcdata,
        arch: &Architecture,
        address: &Address,
        value: VarnodeId,
        write: bool,
        loaded: bool,
        name: &str,
    ) {
        self.record(fd, arch, address, name);
        let Some(vn) = fd.vbank().get(value) else {
            return;
        };
        let key = (address.get_offset(), name.to_string());
        let Some(ty) = self.objects.get(&key) else {
            return;
        };
        if is_composite(ty) || vn.get_size() != ty.get_size() {
            return;
        }
        let size = vn.get_size();
        let assigned = loaded
            && vn.is_explicit()
            && vn.get_def().and_then(|d| fd.obank().get(d)).is_some_and(|d| d.code() == OpCode::CPUI_LOAD);
        let entry = self.accesses.entry(key).or_default();
        let read = if write || !is_integer(ty) || !is_integer(vn.get_type()) {
            Some(Rc::clone(vn.get_type()))
        } else {
            match if assigned { Ok(None) } else { signedness(fd, value, 0) } {
                Ok(None) => {
                    entry.agnostic = true;
                    None
                }
                Ok(Some(meta)) => arch.types_impl().get_base_no_char(size, meta).ok(),
                Err(()) => {
                    for meta in [type_metatype::TYPE_INT, type_metatype::TYPE_UINT] {
                        if let Ok(t) = arch.types_impl().get_base_no_char(size, meta) {
                            push_new(&mut entry.exact, t);
                        }
                    }
                    None
                }
            }
        };
        if let Some(t) = read {
            push_new(if write { &mut entry.writes } else { &mut entry.exact }, t);
        }
    }

    /// A varnode printed by a global's name: one in program data is that
    /// storage, and a register copy merged into the global's variable reads
    /// the storage of the instance that is.
    pub fn record_named(&mut self, fd: &Funcdata, arch: &Architecture, value: VarnodeId, write: bool, name: &str) {
        let Some(vn) = fd.vbank().get(value) else {
            return;
        };
        let data = arch.manage().get_default_data_space().map(|s| s.get_index());
        let in_data = |v: &crate::varnode::Varnode| v.get_addr().get_space().map(|s| s.get_index()) == data;
        let storage = if in_data(vn) {
            Some(vn.get_addr().clone())
        } else {
            let high = vn.get_high().and_then(|h| fd.high_bank().get(h));
            high.and_then(|h| {
                (0..h.num_instances())
                    .filter_map(|i| fd.vbank().get(h.get_instance(i)))
                    .find(|v| in_data(v))
                    .map(|v| v.get_addr().clone())
            })
        };
        if let Some(address) = storage {
            self.record_access(fd, arch, &address, value, write, false, name);
        }
    }

    /// The types to declare an object at, empty for its own type. A read
    /// that keeps its bits accepts any integer of the width, as does a stored
    /// integer or pointer; the other reads name the type, floats bring every
    /// stored type in, and more than one type is a conflict the header reports.
    fn declared(&self, key: &(u64, String)) -> Vec<&Rc<Datatype>> {
        let (Some(a), Some(whole)) = (self.accesses.get(key), self.objects.get(key)) else {
            return Vec::new();
        };
        let float = a.exact.iter().chain(&a.writes).any(|t| t.get_metatype() == type_metatype::TYPE_FLOAT);
        let mut out: Vec<&Rc<Datatype>> = a.exact.iter().collect();
        if float {
            out.extend(&a.writes);
        } else if out.is_empty() && !a.agnostic {
            out.extend(a.writes.iter().filter(|t| !is_integer(t)));
        }
        if a.agnostic && !out.is_empty() && (float || out.iter().any(|t| !is_integer(t))) {
            out.push(whole);
        }
        out
    }

    /// Retain the whole mapped declaration, never the type of a partial read.
    /// Split mappings and uncertain geometry do not prove a standalone object.
    pub fn record(&mut self, fd: &Funcdata, arch: &Architecture, address: &Address, name: &str) {
        if !plain_identifier(name) {
            return;
        }
        let Some(data) = arch.manage().get_default_data_space() else {
            return;
        };
        if address.get_space().map(|s| s.get_index()) != Some(data.get_index()) {
            return;
        }
        let context = fd.get_arch();
        let Some(container) = context.query_container_global(address, 1, &Address::new_invalid())
        else {
            return;
        };
        if container.symbol_offset != 0 {
            return;
        }
        let Some(ty) = container.symbol_type else {
            return;
        };
        if ty.get_size() <= 0 || ty.get_metatype() == type_metatype::TYPE_CODE {
            return;
        }
        let Some((base, extent)) = context.global_symbol_extent(address) else {
            return;
        };
        if base != container.entry_addr.get_offset() || extent != ty.get_size() {
            return;
        }
        let Some((mapped_name, _, _)) =
            context.name_for_global_varnode(address, 1, &Address::new_invalid())
        else {
            return;
        };
        if name != mapped_name {
            return;
        }
        self.objects.entry((base, name.to_string())).or_insert(ty);
    }

    /// A pointer into a complete named object stays relative to that object.
    pub fn object_address(
        &mut self,
        fd: &Funcdata,
        arch: &Architecture,
        offset: u64,
    ) -> Option<String> {
        let data = arch.manage().get_default_data_space()?;
        if data.get_word_size() != 1
            || offset == 0
            || !crate::kuna_globalref::in_ranges(&arch.globalref_ranges, offset)
        {
            return None;
        }
        let address = Address::new(data.clone(), offset);
        let context = fd.get_arch();
        let container = context.query_container_global(&address, 1, &Address::new_invalid())?;
        let (base, extent) = context.global_symbol_extent(&address)?;
        if container.symbol_offset != 0
            || base != container.entry_addr.get_offset()
            || extent <= 0
            || base.checked_add(extent as u64)? <= offset
        {
            return None;
        }
        let base_address = Address::new(data.clone(), base);
        let (name, _, _, scopes) =
            context.name_for_global_varnode_scoped(&base_address, 1, &Address::new_invalid())?;
        if !scopes.is_empty()
            || !plain_identifier(&name)
            || fd
                .get_scope_local()
                .is_some_and(|scope| scope.local_name_used(&name))
        {
            return None;
        }
        self.record(fd, arch, &base_address, &name);
        if !self.objects.contains_key(&(base, name.clone())) {
            return None;
        }
        let delta = offset.checked_sub(base)?;
        Some(if delta == 0 {
            format!("(&{name})")
        } else {
            format!("((unsigned char *)&{name} + 0x{delta:x})")
        })
    }

    /// Each object with the types its whole-width scalar accesses declare it
    /// at (empty when the function only addresses it or reads part of it).
    pub fn objects(&self) -> impl Iterator<Item = (u64, &str, &Rc<Datatype>, Vec<&Rc<Datatype>>)> {
        self.objects.iter().map(|(key, ty)| (key.0, key.1.as_str(), ty, self.declared(key)))
    }
}

/// The metatype C must give the value `id` reads for it to compute what the
/// printer meant, when the varnode's own type does not settle it: `None`
/// when any integer of its width serves (a copy, a return, a call argument,
/// a store, a truncation, a same-width cast, an equality at `int` width or
/// wider or with zero, an address as wide as a pointer, wrapping arithmetic
/// whose result is assigned), signed or unsigned for an ordering, a
/// division, a remainder, a right shift or an extension, float for a float
/// operation, reached directly or through arithmetic printed inline; `Err`
/// when uses disagree or one is anything else.
fn signedness(fd: &Funcdata, id: VarnodeId, depth: u32) -> Result<Option<type_metatype>, ()> {
    use OpCode as Op;
    use type_metatype::*;
    let vn = fd.vbank().get(id).ok_or(())?;
    if depth > 8 {
        return Err(());
    }
    let (wide, address) = (vn.get_size() >= 4, vn.get_size() >= 8);
    let zero = |other: Option<VarnodeId>| {
        other.and_then(|v| fd.vbank().get(v)).is_some_and(|v| v.is_constant() && v.get_offset() == 0)
    };
    let mut want = None;
    for op in vn.descend_iter() {
        let o = fd.obank().get(op).ok_or(())?;
        let first = o.get_in(0) == Some(id);
        let need = match o.code() {
            Op::CPUI_RETURN | Op::CPUI_MULTIEQUAL | Op::CPUI_INDIRECT | Op::CPUI_SUBPIECE | Op::CPUI_PIECE
            | Op::CPUI_CAST => None,
            Op::CPUI_INT_EQUAL | Op::CPUI_INT_NOTEQUAL if wide || zero(o.get_in(if first { 1 } else { 0 })) => None,
            Op::CPUI_CALL | Op::CPUI_CALLIND if !first => None,
            Op::CPUI_STORE if o.get_in(1) != Some(id) || address => None,
            Op::CPUI_LOAD if address => None,
            Op::CPUI_INT_SRIGHT | Op::CPUI_INT_RIGHT if !first => None,
            Op::CPUI_COPY | Op::CPUI_INT_ADD | Op::CPUI_INT_SUB | Op::CPUI_INT_MULT | Op::CPUI_INT_AND
            | Op::CPUI_INT_OR | Op::CPUI_INT_XOR | Op::CPUI_INT_NEGATE | Op::CPUI_INT_2COMP | Op::CPUI_INT_LEFT => {
                let out = o.get_out().ok_or(())?;
                let v = fd.vbank().get(out).ok_or(())?;
                if v.get_size() != vn.get_size() {
                    return Err(());
                }
                if v.is_explicit() { None } else { signedness(fd, out, depth + 1)? }
            }
            Op::CPUI_INT_SLESS | Op::CPUI_INT_SLESSEQUAL | Op::CPUI_INT_SDIV | Op::CPUI_INT_SREM
            | Op::CPUI_INT_SRIGHT | Op::CPUI_INT_SEXT | Op::CPUI_FLOAT_INT2FLOAT => Some(TYPE_INT),
            Op::CPUI_INT_LESS | Op::CPUI_INT_LESSEQUAL | Op::CPUI_INT_DIV | Op::CPUI_INT_REM | Op::CPUI_INT_RIGHT
            | Op::CPUI_INT_ZEXT => Some(TYPE_UINT),
            Op::CPUI_FLOAT_ADD | Op::CPUI_FLOAT_SUB | Op::CPUI_FLOAT_MULT | Op::CPUI_FLOAT_DIV | Op::CPUI_FLOAT_NEG
            | Op::CPUI_FLOAT_ABS | Op::CPUI_FLOAT_SQRT | Op::CPUI_FLOAT_LESS | Op::CPUI_FLOAT_LESSEQUAL
            | Op::CPUI_FLOAT_EQUAL | Op::CPUI_FLOAT_NOTEQUAL | Op::CPUI_FLOAT_NAN | Op::CPUI_FLOAT_FLOAT2FLOAT
            | Op::CPUI_FLOAT_TRUNC | Op::CPUI_FLOAT_CEIL | Op::CPUI_FLOAT_FLOOR | Op::CPUI_FLOAT_ROUND => {
                Some(TYPE_FLOAT)
            }
            _ => return Err(()),
        };
        match (want, need) {
            (Some(a), Some(b)) if a != b => return Err(()),
            (None, n) => want = n,
            _ => {}
        }
    }
    Ok(want)
}

fn push_new(seen: &mut Vec<Rc<Datatype>>, ty: Rc<Datatype>) {
    if !seen.iter().any(|t| Rc::ptr_eq(t, &ty)) {
        seen.push(ty);
    }
}

/// A scalar integer, or the unknown bytes an undefined scalar prints as.
pub fn is_integer(ty: &Datatype) -> bool {
    matches!(
        ty.get_metatype(),
        type_metatype::TYPE_INT | type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN
    )
}

fn is_composite(ty: &Datatype) -> bool {
    matches!(
        ty.get_metatype(),
        type_metatype::TYPE_STRUCT | type_metatype::TYPE_UNION | type_metatype::TYPE_ARRAY
    )
}

fn plain_identifier(name: &str) -> bool {
    let mut bytes = name.bytes();
    bytes.next().is_some_and(|byte| byte.is_ascii_alphabetic() || byte == b'_')
        && bytes.all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
        && !"auto break case char const continue default do double else enum extern float for goto if inline int long register restrict return short signed sizeof static struct switch typedef union unsigned void volatile while _Alignas _Alignof _Atomic _Bool _Complex _Generic _Imaginary _Noreturn _Static_assert _Thread_local bool true false asm __asm__ typeof __typeof__"
            .split_ascii_whitespace().any(|keyword| keyword == name)
}

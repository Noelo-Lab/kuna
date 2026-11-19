//! (kuna `itaniumrtti on`) Type each recovered C++ vtable, so a virtual call
//! renders through a named slot of the receiver's class.
//!
//! An Itanium virtual call is two loads and an indirect call: the object's
//! first word is the vtable pointer, and the method sits at a fixed slot past
//! the vtable's address point. With the receiver typed only as a pointer to an
//! empty class shell, nothing names either load:
//!
//! ```text
//! v2 = (**(int (**)(void))(*(long *)a0 + 0x10))();
//! ```
//!
//! The RTTI recovery (`kuna_analysis::rtti::kuna_vtabletypes`) knows each
//! polymorphic class's primary vtable and, for every slot, the function the
//! class holds there and every override its derived classes hold. This module
//! turns that into types: a structure `C_vtbl` with one function-pointer field
//! per slot, named after the method (`area`, or `vfn_<i>` when no symbol names
//! it), and the class structure `C` given a first field `C_vtbl *vptr`. The
//! class is otherwise unknown, so it is variable-length: a member access past
//! the vptr keeps its byte offset instead of being read as an array index.
//! Ordinary field rendering then gives `(*a0->vptr->area)(a0,a1)`.
//!
//! A slot's function pointer carries the methods' prototype when every target
//! with a known prototype agrees on the return type and the parameters after
//! `this` (whose type becomes `C *`); the indirect call then takes that
//! prototype, as a call through any typed function pointer does. Otherwise the
//! field is a plain `code *` and the call's arguments are recovered at the
//! call site as before.
//!
//! A class whose structure is already complete (DWARF laid it out) or whose
//! `C_vtbl` name is taken is left alone.

use std::collections::HashMap;
use std::rc::Rc;

use kuna_base::error::KunaResult;
use kuna_base::types::int4;

use kuna_base::address::Address;
use crate::architecture::Architecture;
use crate::dtype::{flags, type_metatype, Datatype, TypeField};
use crate::fspec::PrototypePieces;
use crate::kuna_kalloctype::KallocView;
use kuna_base::space::AddrSpace;

/// One polymorphic class's primary vtable, as the RTTI recovery found it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VtableClass {
    /// The class structure's name: the unqualified, template-stripped name the
    /// demangled signatures give `this` (`TableBuilder`, `Vec`).
    pub class: String,
    /// The slots from the address point on.
    pub slots: Vec<VtableSlot>,
    /// The non-virtual bases at a nonzero offset, as `(structure name, offset)`:
    /// each has a vptr of its own there.
    pub bases: Vec<(String, i64)>,
    /// The instance size when the class registers one (an IOKit class's
    /// `OSMetaClass` size), else 0. A sized class is laid out to that size
    /// with its first field named `vtable`.
    pub size: u64,
}

/// One slot of a [`VtableClass`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VtableSlot {
    /// The field name: the method's unqualified name, or `vfn_<i>`.
    pub name: String,
    /// The function the class holds in this slot (absent for an imported or
    /// pure-virtual slot), then every override a derived class holds there.
    pub targets: Vec<u64>,
}

/// The most slots a vtable is typed with; a longer one is left untyped.
const MAX_SLOTS: usize = 4096;

/// The largest sized class laid out field by field (one field per 8 bytes).
const MAX_SIZED: int4 = 0x4000;

/// Install the vtable and class structures for `classes`; returns how many
/// classes were typed. A class is typed after the bases it embeds.
pub fn install(arch: &Architecture, space: &Rc<AddrSpace>, classes: &[VtableClass]) -> KunaResult<usize> {
    let mut views: HashMap<&str, &KallocView> = HashMap::new();
    for v in arch.kuna_kalloc_types.values().filter(|v| !v.pointers) {
        views.entry(v.name.as_str()).or_insert(v);
    }
    let names: std::collections::HashSet<&str> = classes.iter().map(|c| c.class.as_str()).collect();
    let mut pending: Vec<&VtableClass> = classes.iter().collect();
    let mut typed = 0;
    while !pending.is_empty() {
        let before = pending.len();
        let mut waiting = Vec::new();
        for c in pending {
            let blocked = c.bases.iter().any(|(b, _)| {
                names.contains(b.as_str()) && arch.types().find_by_name(b).ok().flatten().is_none_or(|t| t.is_incomplete())
            });
            if blocked && before > 1 {
                waiting.push(c);
            } else if install_class(arch, space, c, &views)? {
                typed += 1;
            }
        }
        if waiting.len() == before {
            for c in waiting {
                typed += usize::from(install_class(arch, space, c, &views)?);
            }
            break;
        }
        pending = waiting;
    }
    Ok(typed)
}

fn install_class(
    arch: &Architecture,
    space: &Rc<AddrSpace>,
    c: &VtableClass,
    views: &HashMap<&str, &KallocView>,
) -> KunaResult<bool> {
    let types = arch.types();
    let ptr = types.get_size_of_pointer();
    let ws = space.get_word_size();
    let vtbl_name = format!("{}_vtbl", c.class);
    if c.slots.is_empty() || c.slots.len() > MAX_SLOTS || types.find_by_name(&vtbl_name)?.is_some() {
        return Ok(false);
    }
    let class = match types.find_by_name(&c.class)? {
        Some(class) if class.is_incomplete() && class.get_metatype() == type_metatype::TYPE_STRUCT => class,
        Some(_) => return Ok(false),
        None => types.kuna_declare_record(&c.class, false)?,
    };
    let vtbl = types.kuna_declare_record(&vtbl_name, false)?;
    let this = types.get_type_pointer(ptr, Rc::clone(&class), ws)?;
    let mut fields = Vec::with_capacity(c.slots.len());
    for (i, slot) in c.slots.iter().enumerate() {
        let code = match agreed_prototype(arch, space, &slot.targets, &this) {
            Some(pieces) => types.get_type_code_proto(&pieces)?,
            None => types.get_type_code()?,
        };
        let field = types.get_type_pointer(ptr, code, ws)?;
        fields.push(TypeField::new(i as int4, i as int4 * ptr, slot.name.clone(), field));
    }
    let size = c.slots.len() as int4 * ptr;
    let vtbl = types.set_fields_struct_raw(&vtbl, fields, Vec::new(), size, ptr, flags::variable_length)?;
    let vptr = types.get_type_pointer(ptr, vtbl, ws)?;
    if let Some(size) = int4::try_from(c.size).ok().filter(|&s| s >= ptr && s <= MAX_SIZED) {
        let signature = views.get(c.class.as_str()).filter(|v| u64::from(v.size) == c.size).map_or("", |v| v.signature.as_str());
        let mut fields = vec![TypeField::new(0, 0, "vtable", vptr)];
        for f in crate::kuna_kalloctype::granule_fields(types, signature, true, ptr, size).unwrap_or_default() {
            fields.push(TypeField::new(fields.len() as int4, f.offset, f.name, f.field_type));
        }
        types.set_fields_struct_raw(&class, fields, Vec::new(), size, ptr, 0)?;
        return Ok(true);
    }
    let mut fields = vec![TypeField::new(0, 0, "vptr", vptr)];
    let mut size = ptr;
    for (name, off) in &c.bases {
        let Some(base) = types.find_by_name(name)?.filter(|b| !b.is_incomplete()) else { continue };
        let Ok(off) = int4::try_from(*off) else { continue };
        if off < size {
            continue;
        }
        size = off + base.get_size();
        fields.push(TypeField::new(fields.len() as int4, off, name.clone(), base));
    }
    types.set_fields_struct_raw(&class, fields, Vec::new(), size, ptr, flags::variable_length)?;
    Ok(true)
}

/// The prototype every target with a known one shares, with `this` retyped to
/// the slot's class, or `None` when no target has one or two disagree.
fn agreed_prototype(
    arch: &Architecture,
    space: &Rc<AddrSpace>,
    targets: &[u64],
    this: &Rc<Datatype>,
) -> Option<PrototypePieces> {
    let symbols = &arch.symboltab;
    let mut agreed: Option<&PrototypePieces> = None;
    for &t in targets {
        let addr = Address::new(Rc::clone(space), t);
        let Some((sid, _)) = symbols.find_function_across_scopes(&addr) else { continue };
        let Some(pieces) = symbols.function_proto_pieces_of(sid) else { continue };
        if pieces.intypes.is_empty() {
            return None;
        }
        match agreed {
            None => agreed = Some(pieces),
            Some(first) if same_after_this(first, pieces) => {}
            Some(_) => return None,
        }
    }
    let mut pieces = agreed?.clone();
    pieces.intypes[0] = Rc::clone(this);
    if let Some(name) = pieces.innames.first_mut() {
        *name = "this".to_string();
    }
    Some(pieces)
}

fn same_after_this(a: &PrototypePieces, b: &PrototypePieces) -> bool {
    let same = |x: &Rc<Datatype>, y: &Rc<Datatype>| Rc::ptr_eq(x, y) || x.compare(y, 10).is_ok_and(|c| c == 0);
    a.first_var_arg_slot == b.first_var_arg_slot
        && a.intypes.len() == b.intypes.len()
        && a.intypes.iter().zip(&b.intypes).skip(1).all(|(x, y)| same(x, y))
        && match (&a.outtype, &b.outtype) {
            (None, None) => true,
            (Some(x), Some(y)) => same(x, y),
            _ => false,
        }
}

//! (kuna `valuewitness`) Name and type the value witness functions of every
//! local struct and enum.
//!
//! Each value type's metadata is preceded by a pointer to its value witness
//! table: eight functions the runtime calls to copy, move and destroy a value
//! of the type without knowing its layout (`initializeBufferWithCopyOfBuffer`,
//! `destroy`, `initializeWithCopy`, `assignWithCopy`, `initializeWithTake`,
//! `assignWithTake`, `getEnumTagSinglePayload`, `storeEnumTagSinglePayload`),
//! then the size, stride, flags and extra-inhabitant count, and for an enum
//! three more functions (`getEnumTag`, `destructiveProjectEnumData`,
//! `destructiveInjectEnumTag`). A stripped image keeps the tables, so each
//! function is named after its type and slot, the way the demangler spells a
//! value-witness symbol (`Module.Point.initializeWithCopy_value_witness`),
//! and given its runtime prototype (`swift/ABI/ValueWitness.def`).
//!
//! The table is found three ways, each checked before it is read: the static
//! metadata a non-generic accessor returns, the incomplete metadata a type
//! with singleton initialization records, and the value witnesses a generic
//! type's metadata pattern points at. A table the loader binds to another
//! image (the runtime's builtin tables for trivial layouts) names nothing.

use std::collections::BTreeMap;
use std::rc::Rc;

use kuna_decomp::dtype::{Datatype, TypeFactory, type_metatype};
use kuna_decomp::fspec::PrototypePieces;

use super::Namer;
use super::descriptors::{Context, ContextKind, Metadata, Reader};
use super::kuna_swifttypes::{Types, locate_metadata};
use crate::pass::{AnalysisOutput, TypedDataFact};

/// The value witness slots, in table order: the eight every table has, and
/// the three an enum's table adds after the layout words.
pub const WITNESSES: [(&str, u64); 11] = [
    ("initializeBufferWithCopyOfBuffer", 0),
    ("destroy", 8),
    ("initializeWithCopy", 16),
    ("assignWithCopy", 24),
    ("initializeWithTake", 32),
    ("assignWithTake", 40),
    ("getEnumTagSinglePayload", 48),
    ("storeEnumTagSinglePayload", 56),
    ("getEnumTag", 88),
    ("destructiveProjectEnumData", 96),
    ("destructiveInjectEnumTag", 104),
];

/// `ValueWitnessFlags::HasEnumWitnesses`.
pub const HAS_ENUM_WITNESSES: u32 = 0x0020_0000;
const VWT_FLAGS: u64 = 80;

/// The size of a struct or enum descriptor before its trailing objects.
const VALUE_DESCRIPTOR_SIZE: u64 = 28;

/// One located table: its address, its type's path, and whether the
/// static metadata (which `swifttypes` already types) led to it.
pub struct Table {
    pub addr: u64,
    pub path: String,
    pub is_static: bool,
}

/// What the pass found: each table, and each function it claimed with the
/// slot index into [`WITNESSES`].
#[derive(Default)]
pub struct Witnesses {
    pub tables: Vec<Table>,
    pub functions: BTreeMap<u64, Vec<usize>>,
}

fn rel(reader: &Reader, at: u64) -> Option<u64> {
    let off = reader.img.read_i32(at)?;
    (off != 0).then(|| at.wrapping_add(off as i64 as u64))
}

/// Whether `meta` is value metadata describing the descriptor at `desc`.
fn is_metadata_of(reader: &Reader, md: &Metadata, meta: u64, desc: u64) -> bool {
    matches!(reader.img.read_ptr(meta), Some(0x200..=0x202))
        && meta
            .checked_add(md.ptr_size)
            .and_then(|slot| reader.img.read_ptr(slot))
            == Some(desc)
}

/// The local table in the word before the metadata at `meta`.
fn table_before(reader: &Reader, md: &Metadata, meta: u64) -> Option<u64> {
    let slot = meta.checked_sub(md.ptr_size)?;
    if reader.bind_slots.contains(&slot) {
        return None;
    }
    reader.img.read_ptr(slot).filter(|&t| t != 0)
}

/// The value witness table of the struct or enum `ctx`, and whether it came
/// from static metadata.
fn locate(
    reader: &Reader,
    md: &Metadata,
    arch: object::Architecture,
    ctx: &Context,
) -> Option<(u64, bool)> {
    let specific = ctx.flags >> 16;
    let trailing = ctx.addr.checked_add(VALUE_DESCRIPTOR_SIZE)?;
    if ctx.flags & 0x80 != 0 {
        // TypeGenericContextDescriptorHeader: the instantiation cache, then
        // the default instantiation pattern, whose value witnesses follow
        // the instantiation and completion functions and the pattern flags.
        let pattern = rel(reader, trailing.checked_add(4)?)?;
        let at = pattern.checked_add(12)?;
        return Some((rel(reader, at)?, false));
    }
    match specific & 3 {
        0 => {
            let meta = locate_metadata(reader.img, arch, ctx.accessor?)?;
            is_metadata_of(reader, md, meta, ctx.addr).then_some(())?;
            Some((table_before(reader, md, meta)?, true))
        }
        1 => {
            // SingletonMetadataInitialization: the cache, then the
            // incomplete metadata the runtime completes in place.
            let meta = rel(reader, trailing.checked_add(4)?)?;
            is_metadata_of(reader, md, meta, ctx.addr).then_some(())?;
            Some((table_before(reader, md, meta)?, false))
        }
        _ => None,
    }
}

/// Find every local struct's and enum's table and claim its functions.
pub fn claim(
    reader: &Reader,
    md: &Metadata,
    arch: object::Architecture,
    namer: &Namer,
) -> Witnesses {
    let mut out = Witnesses::default();
    let mut contexts: Vec<&Context> = md
        .contexts
        .values()
        .filter(|c| matches!(c.kind, ContextKind::Struct | ContextKind::Enum))
        .collect();
    contexts.sort_by_key(|c| c.addr);
    for ctx in contexts {
        let Some((table, is_static)) = locate(reader, md, arch, ctx) else {
            continue;
        };
        let Some(path) = namer.path(ctx.addr, 0) else {
            continue;
        };
        let Some(flags) = table
            .checked_add(VWT_FLAGS)
            .and_then(|a| reader.img.read_u32(a))
        else {
            continue;
        };
        let count = if flags & HAS_ENUM_WITNESSES != 0 {
            WITNESSES.len()
        } else {
            8
        };
        for (i, (witness, offset)) in WITNESSES[..count].iter().enumerate() {
            let Some(slot) = table.checked_add(*offset) else {
                continue;
            };
            if reader.bind_slots.contains(&slot) {
                continue;
            }
            let Some(f) = reader.img.read_ptr(slot).filter(|&f| namer.is_code(f)) else {
                continue;
            };
            namer.claim(
                f,
                format!("{path}.{witness}_value_witness"),
                &path,
                None,
                false,
            );
            out.functions.entry(f).or_default().push(i);
        }
        out.tables.push(Table {
            addr: table,
            path,
            is_static,
        });
    }
    out
}

/// Type the tables `swifttypes` did not reach through static metadata.
pub fn type_tables(w: &Witnesses, types: &Types, out: &mut AnalysisOutput) {
    for table in w.tables.iter().filter(|t| !t.is_static) {
        let Some(t) = types.table_struct_at(table.addr) else {
            continue;
        };
        out.swift_witnesses.typed_data.push(TypedDataFact {
            addr: table.addr,
            name: format!("{}.value_witness_table", table.path),
            type_: t,
        });
    }
}

/// The runtime signature of witness `i` with its value pointers `value`,
/// its `uint32_t` counts `count` and the type's metadata `meta`: the
/// output type, and the parameters.
fn signature(
    i: usize,
    value: &Rc<Datatype>,
    count: &Rc<Datatype>,
    meta: &Rc<Datatype>,
    void: &Rc<Datatype>,
) -> (Rc<Datatype>, Vec<(&'static str, Rc<Datatype>)>) {
    let v = |n: &'static str| (n, Rc::clone(value));
    let c = |n: &'static str| (n, Rc::clone(count));
    let m = ("self", Rc::clone(meta));
    match WITNESSES[i].0 {
        "destroy" | "destructiveProjectEnumData" => (Rc::clone(void), vec![v("object"), m]),
        "getEnumTagSinglePayload" => (Rc::clone(count), vec![v("value"), c("emptyCases"), m]),
        "storeEnumTagSinglePayload" => (
            Rc::clone(void),
            vec![v("value"), c("whichCase"), c("emptyCases"), m],
        ),
        "getEnumTag" => (Rc::clone(count), vec![v("value"), m]),
        "destructiveInjectEnumTag" => (Rc::clone(void), vec![v("value"), c("tag"), m]),
        _ => (Rc::clone(value), vec![v("dest"), v("src"), m]),
    }
}

/// The runtime prototype of witness `i`, named `name`, with the type's
/// metadata typed `meta`.
fn prototype(
    tf: &dyn TypeFactory,
    i: usize,
    name: String,
    meta: &Rc<Datatype>,
) -> Option<PrototypePieces> {
    let void = tf.get_type_void().ok()?;
    let count = tf.get_base(4, type_metatype::TYPE_UINT).ok()?;
    let value = tf
        .get_type_pointer(tf.get_size_of_pointer(), Rc::clone(&void), 1)
        .ok()?;
    let (outtype, params) = signature(i, &value, &count, meta, &void);
    let (innames, intypes) = params.into_iter().map(|(n, t)| (n.to_string(), t)).unzip();
    Some(PrototypePieces {
        name,
        outtype: Some(outtype),
        intypes,
        innames,
        first_var_arg_slot: -1,
        output_storage: None,
        input_storage: Vec::new(),
        kuna_library_names: None,
        kuna_library_table: false,
    })
}

/// A pointer to a function of witness `i`'s signature: the type of its slot.
pub fn witness_pointer(
    tf: &dyn TypeFactory,
    i: usize,
    meta: &Rc<Datatype>,
) -> Option<Rc<Datatype>> {
    let proto = prototype(tf, i, String::new(), meta)?;
    let code = tf.get_type_code_proto(&proto).ok()?;
    tf.get_type_pointer(tf.get_size_of_pointer(), code, 1).ok()
}

/// The runtime prototype of every claimed function that is one value
/// witness and nothing else, bound by address so a folded body is typed too.
pub fn prototypes(
    w: &Witnesses,
    namer: &Namer,
    tf: &dyn TypeFactory,
    meta: &Rc<Datatype>,
    out: &mut AnalysisOutput,
) {
    let claims = namer.claims.borrow();
    for (&f, slots) in &w.functions {
        let i = slots[0];
        if slots.iter().any(|&s| s != i) || claims.get(&f).is_none_or(|c| c.len() != slots.len()) {
            continue;
        }
        if let Some(p) = prototype(tf, i, claims[&f][0].name.clone(), meta) {
            out.swift_witnesses.prototypes.push((f, p));
        }
    }
}

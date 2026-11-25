//! (kuna `valuewitness`) Name and type the outlined value-witness helpers.
//!
//! swiftc copies, moves and destroys a value whose layout it only learns at
//! run time through small outlined helpers: fetch the type's metadata, load
//! its value witness table from the word before it, call one witness, and
//! hand back the value. On arm64 a helper reads
//!
//! ```text
//!   stp x20, x19, [sp, #-0x20]!; stp x29, x30, [sp, #0x10]; add x29, sp, #0x10
//!   mov x19, x1; mov x20, x0
//!   mov x0, #0; bl <T.type_metadata_accessor>     (or: adrp/add x2, <metadata>)
//!   mov x2, x0; ldur x8, [x0, #-8]; ldr x8, [x8, #0x10]
//!   mov x0, x19; mov x1, x20; blr x8              (initializeWithCopy)
//!   mov x0, x19; ldp ...; ret
//! ```
//!
//! and a helper the linker merged across types takes the accessor as an
//! argument (`blr x2` in place of the `bl`). A stripped image keeps the
//! helpers but not their names; the body says which witness it calls and
//! which type's metadata it fetches, so each is named the way the demangler
//! spells the symbol (`Module.T.outlined_init_with_copy`; a merged one,
//! whose type is a caller's choice, `outlined_init_with_copy.merged`) and
//! given the prototype its body states (`void *f(void *src, void *dest)`,
//! the value it returns, and the accessor a merged one takes).
//!
//! The body is decoded by a small register tracker over the few instruction
//! forms the helpers use; any other instruction, a second call, an argument
//! that does not line up with the witness's signature, or a body that does
//! not return the address it was given (a value witness function of the
//! same shape) leaves the function alone, as does a function another record
//! already claims. The metadata must come from a local type's accessor, an
//! imported type metadata accessor, a local type's static metadata, or an
//! argument. arm64 only.

use std::collections::{BTreeMap, HashMap};
use std::rc::Rc;

use kuna_decomp::dtype::{Datatype, TypeFactory, type_metatype};
use kuna_decomp::fspec::PrototypePieces;

use super::descriptors::{Metadata, Reader};
use super::{Namer, import_name};
use crate::pass::AnalysisOutput;

/// The longest body looked at, in instructions.
const MAX_BODY: usize = 32;

/// The witnesses a helper calls: its table offset, the witness, and the
/// demangler's spelling of the helper.
const HELPERS: [(u64, &str); 5] = [
    (8, "outlined_destroy"),
    (16, "outlined_init_with_copy"),
    (24, "outlined_assign_with_copy"),
    (32, "outlined_init_with_take"),
    (40, "outlined_assign_with_take"),
];

/// Where a helper's metadata comes from.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Source {
    /// A call of the accessor at this address.
    Accessor(u64),
    /// The static metadata at this address.
    Static(u64),
    /// A call of the accessor passed in this argument register.
    Argument(u8),
    /// The metadata in the pointer slot at this address (a bound import or
    /// a rebased local).
    Slot(u64),
    /// A call of the function at this address with the helper's own
    /// arguments in these registers (a merged helper's instantiation call).
    Forwarded(u64, Vec<u8>),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Val {
    Unknown,
    Arg(u8),
    Const(u64),
    /// The word at this address, not yet used.
    Loaded(u64),
    Meta,
    Table,
    /// The witness at this table offset.
    Witness(u64),
    Result,
}

/// Whether `name` ends in a helper's spelling.
pub fn is_helper_name(name: &str) -> bool {
    HELPERS.iter().any(|(_, h)| name.ends_with(h))
}

/// One recognised helper.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Helper {
    /// Index into [`HELPERS`].
    pub kind: usize,
    pub source: Source,
    /// The witness's value arguments, as helper argument registers:
    /// `[object]` for destroy, `[dest, src]` for the others.
    pub values: Vec<u8>,
    /// The argument register the helper returns, if any.
    pub returns: Option<u8>,
}

fn sext(v: u64, bits: u32) -> i64 {
    let shift = 64 - bits;
    ((v << shift) as i64) >> shift
}

/// Recognise the helper whose body starts at `pc` in `words`.
pub fn recognise(words: &[u32], pc: u64) -> Option<Helper> {
    let mut regs = [Val::Unknown; 32];
    for (i, r) in regs.iter_mut().take(8).enumerate() {
        *r = Val::Arg(i as u8);
    }
    let mut source = None;
    let mut witness: Option<(u64, Vec<Val>)> = None;
    let mut done = false;
    for (i, &w) in words.iter().enumerate().take(MAX_BODY) {
        let here = pc.wrapping_add(4 * i as u64);
        let rd = (w & 31) as usize;
        let rn = ((w >> 5) & 31) as usize;
        let pair = w & 0xffc0_0000;
        if matches!(
            pair,
            0xa980_0000 | 0xa900_0000 | 0xa880_0000 | 0xa9c0_0000 | 0xa940_0000 | 0xa8c0_0000
        ) && rn == 31
        {
            if w & 0x0040_0000 != 0 {
                // A reload of saved registers: they hold the caller's values.
                let rt2 = ((w >> 10) & 31) as usize;
                if rd < 19 || rt2 < 19 {
                    return None;
                }
                regs[rd] = Val::Unknown;
                regs[rt2] = Val::Unknown;
            }
            continue;
        }
        if w & 0xffc0_03ff == 0x9100_03fd || w == 0xd503_201f {
            continue;
        }
        if w & 0xffe0_ffe0 == 0xaa00_03e0 {
            regs[rd] = regs[((w >> 16) & 31) as usize];
            continue;
        }
        if w & 0xffe0_0000 == 0xd280_0000 || w & 0xffe0_0000 == 0x5280_0000 {
            regs[rd] = Val::Const(((w >> 5) & 0xffff) as u64);
            continue;
        }
        if w & 0x9f00_0000 == 0x9000_0000 {
            let imm = (((w >> 5) & 0x7_ffff) << 2 | (w >> 29) & 3) as u64;
            let page = (here & !0xfff).wrapping_add((sext(imm, 21) << 12) as u64);
            regs[rd] = Val::Const(page);
            continue;
        }
        if w & 0x9f00_0000 == 0x1000_0000 {
            let imm = (((w >> 5) & 0x7_ffff) << 2 | (w >> 29) & 3) as u64;
            regs[rd] = Val::Const(here.wrapping_add(sext(imm, 21) as u64));
            continue;
        }
        if w & 0xff80_0000 == 0x9100_0000 {
            let Val::Const(base) = regs[rn] else {
                return None;
            };
            let mut imm = ((w >> 10) & 0xfff) as u64;
            if (w >> 22) & 1 != 0 {
                imm <<= 12;
            }
            regs[rd] = Val::Const(base.wrapping_add(imm));
            continue;
        }
        if w & 0xffe0_0c00 == 0xf840_0000 {
            if sext(((w >> 12) & 0x1ff) as u64, 9) != -8 {
                return None;
            }
            let found = match regs[rn] {
                Val::Meta => None,
                Val::Const(m) => Some(Source::Static(m)),
                Val::Loaded(at) => Some(Source::Slot(at)),
                _ => return None,
            };
            if let Some(found) = found {
                if source.is_some() {
                    return None;
                }
                source = Some(found);
                let was = regs[rn];
                for r in regs.iter_mut().filter(|r| **r == was) {
                    *r = Val::Meta;
                }
            }
            regs[rd] = Val::Table;
            continue;
        }
        if w & 0xffc0_0000 == 0xf940_0000 {
            let off = ((w >> 10) & 0xfff) as u64 * 8;
            regs[rd] = match regs[rn] {
                Val::Table if witness.is_none() => Val::Witness(off),
                Val::Const(page) => Val::Loaded(page.wrapping_add(off)),
                _ => return None,
            };
            continue;
        }
        let call = if w & 0xfc00_0000 == 0x9400_0000 {
            Some(None)
        } else if w & 0xffff_fc1f == 0xd63f_0000 {
            Some(Some(regs[rn]))
        } else {
            None
        };
        if let Some(through) = call {
            match through {
                Some(Val::Witness(off)) => {
                    if witness.is_some() {
                        return None;
                    }
                    witness = Some((off, regs[..3].to_vec()));
                    regs[0] = Val::Result;
                }
                _ => {
                    if source.is_some() {
                        return None;
                    }
                    let target = here
                        .wrapping_add((sext((w & 0x03ff_ffff) as u64, 26) as u64).wrapping_mul(4));
                    source = Some(match (through, regs[0]) {
                        (None, Val::Const(0)) => Source::Accessor(target),
                        (Some(Val::Arg(a)), Val::Const(0)) => Source::Argument(a),
                        (None, Val::Arg(a)) => {
                            let mut args = vec![a];
                            match regs[1] {
                                Val::Arg(b) if b != a => args.push(b),
                                _ => {}
                            }
                            Source::Forwarded(target, args)
                        }
                        _ => return None,
                    });
                    regs[0] = Val::Meta;
                }
            }
            for r in regs.iter_mut().take(19).skip(1) {
                *r = Val::Unknown;
            }
            continue;
        }
        if w == 0xd65f_03c0 {
            done = true;
            break;
        }
        return None;
    }
    if !done {
        return None;
    }
    let (slot, args) = witness?;
    let source = source?;
    let kind = HELPERS.iter().position(|&(off, _)| off == slot)?;
    let n = if kind == 0 { 1 } else { 2 };
    if args[n] != Val::Meta {
        return None;
    }
    let values: Vec<u8> = args[..n]
        .iter()
        .map(|v| match v {
            Val::Arg(a) => Some(*a),
            _ => None,
        })
        .collect::<Option<_>>()?;
    let consumed = match &source {
        Source::Argument(a) => vec![*a],
        Source::Forwarded(_, args) => args.clone(),
        _ => Vec::new(),
    };
    if values.iter().any(|v| consumed.contains(v)) {
        return None;
    }
    // swiftc's helper returns the address it was given (`dest` for a copy
    // or take); a value witness function, which returns the witness's result
    // or nothing, is not one.
    let returns = match regs[0] {
        Val::Arg(a) if a == values[0] => Some(a),
        _ => return None,
    };
    Some(Helper {
        kind,
        source,
        values,
        returns,
    })
}

/// What the pass found: each helper by address.
#[derive(Default)]
pub struct Helpers {
    pub found: BTreeMap<u64, Helper>,
}

/// The dotted path of the type whose metadata `source` yields, `None` for
/// an accessor argument, or `Err` when the source names no known type.
fn type_of(
    source: &Source,
    reader: &Reader,
    md: &Metadata,
    namer: &Namer,
    accessors: &HashMap<u64, u64>,
) -> Result<Option<String>, ()> {
    match *source {
        Source::Argument(_) | Source::Forwarded(..) => Ok(None),
        Source::Slot(at) => {
            if reader.bind_slots.contains(&at) {
                return import_name(namer.imports, at, "type_metadata")
                    .map(Some)
                    .ok_or(());
            }
            let m = reader.img.read_ptr(at).ok_or(())?;
            type_of(&Source::Static(m), reader, md, namer, accessors)
        }
        Source::Accessor(f) => {
            if let Some(&desc) = accessors.get(&f) {
                return namer.path(desc, 0).map(Some).ok_or(());
            }
            import_name(namer.imports, f, "type_metadata_accessor")
                .map(Some)
                .ok_or(())
        }
        Source::Static(m) => {
            if !matches!(reader.img.read_ptr(m), Some(0x200..=0x202)) {
                return Err(());
            }
            let desc = reader
                .img
                .read_ptr(m.checked_add(md.ptr_size).ok_or(())?)
                .ok_or(())?;
            md.contexts.contains_key(&desc).then_some(()).ok_or(())?;
            namer.path(desc, 0).map(Some).ok_or(())
        }
    }
}

/// Recognise and claim every helper among `entries`.
pub fn claim(reader: &Reader, md: &Metadata, namer: &Namer, entries: &[u64]) -> Helpers {
    let accessors: HashMap<u64, u64> = md
        .contexts
        .values()
        .filter_map(|c| Some((c.accessor?, c.addr)))
        .collect();
    let mut out = Helpers::default();
    for &f in entries {
        if !namer.is_code(f) || namer.claims.borrow().contains_key(&f) {
            continue;
        }
        let Some(bytes) = [MAX_BODY * 4, 64, 32]
            .iter()
            .find_map(|&n| reader.img.read(f, n))
        else {
            continue;
        };
        if bytes
            .first_chunk::<4>()
            .map(|b| u32::from_le_bytes(*b) & 0xffc0_0000)
            != Some(0xa980_0000)
        {
            continue;
        }
        let words: Vec<u32> = bytes
            .chunks_exact(4)
            .map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
            .collect();
        let Some(h) = recognise(&words, f) else {
            continue;
        };
        let Ok(ty) = type_of(&h.source, reader, md, namer, &accessors) else {
            continue;
        };
        let helper = HELPERS[h.kind].1;
        let (name, owner) = match &ty {
            Some(t) => (format!("{t}.{helper}"), t.clone()),
            None => (format!("{helper}.merged"), String::new()),
        };
        namer.claim(f, name, &owner, None, false);
        out.found.insert(f, h);
    }
    out
}

/// The prototype a helper's body states.
fn prototype(
    tf: &dyn TypeFactory,
    h: &Helper,
    name: String,
    meta: &Rc<Datatype>,
) -> Option<PrototypePieces> {
    let void = tf.get_type_void().ok()?;
    let ptr_size = tf.get_size_of_pointer();
    let value = tf.get_type_pointer(ptr_size, Rc::clone(&void), 1).ok()?;
    let mut extra: Vec<(u8, &str, Rc<Datatype>)> = Vec::new();
    if let Source::Forwarded(_, args) = &h.source {
        for (&a, role) in args.iter().zip(["cache", "record"]) {
            extra.push((a, role, Rc::clone(&value)));
        }
    }
    let accessor = match h.source {
        Source::Argument(a) => {
            let request = tf.get_base(8, type_metatype::TYPE_UINT).ok()?;
            let proto = PrototypePieces {
                name: String::new(),
                outtype: Some(Rc::clone(meta)),
                intypes: vec![request],
                innames: vec!["request".to_string()],
                first_var_arg_slot: -1,
                output_storage: None,
                input_storage: Vec::new(),
                kuna_library_names: None,
                kuna_library_table: false,
            };
            let code = tf.get_type_code_proto(&proto).ok()?;
            Some((a, tf.get_type_pointer(ptr_size, code, 1).ok()?))
        }
        _ => None,
    };
    if let Some((a, t)) = accessor {
        extra.push((a, "accessor", t));
    }
    let count = h
        .values
        .iter()
        .copied()
        .chain(extra.iter().map(|e| e.0))
        .max()? as usize
        + 1;
    let mut slots: Vec<Option<(String, Rc<Datatype>)>> = vec![None; count];
    let roles: &[&str] = if h.kind == 0 {
        &["value"]
    } else {
        &["dest", "src"]
    };
    for (&r, role) in h.values.iter().zip(roles) {
        slots[r as usize] = Some((role.to_string(), Rc::clone(&value)));
    }
    for (a, role, t) in extra {
        slots[a as usize] = Some((role.to_string(), t));
    }
    let (innames, intypes): (Vec<String>, Vec<Rc<Datatype>>) = slots
        .into_iter()
        .collect::<Option<Vec<_>>>()?
        .into_iter()
        .unzip();
    Some(PrototypePieces {
        name,
        outtype: Some(if h.returns.is_some() { value } else { void }),
        intypes,
        innames,
        first_var_arg_slot: -1,
        output_storage: None,
        input_storage: Vec::new(),
        kuna_library_names: None,
        kuna_library_table: false,
    })
}

/// Bind the prototype of every helper whose only claim is its own.
pub fn prototypes(
    h: &Helpers,
    namer: &Namer,
    tf: &dyn TypeFactory,
    meta: &Rc<Datatype>,
    out: &mut AnalysisOutput,
) {
    let claims = namer.claims.borrow();
    for (&f, helper) in &h.found {
        let Some([only]) = claims.get(&f).map(Vec::as_slice) else {
            continue;
        };
        if let Some(p) = prototype(tf, helper, only.name.clone(), meta) {
            out.swift_witnesses.prototypes.push((f, p));
        }
    }
}

#[cfg(test)]
mod tests;

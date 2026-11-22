//! (kuna `switchextent`) Bound a jump table no range check reaches at the next
//! jump table of the same function.
//!
//! A compiler that knows its selector is in range checks nothing before the
//! dispatch: Swift's `switch` over a multi-payload enum indexes a halfword
//! offset table straight with the case `swift_getEnumCaseMultiPayload`
//! returned, and clang does the same for a C `switch` whose `default` is
//! `__builtin_unreachable()`. The basic model then has no guard, the
//! selector's whole 32-bit width exceeds `jumptablemax`, and the dispatch
//! prints as a computed call that ends the function's flow: a 13 KB Swift copy
//! helper decompiled to its first 500 lines.
//!
//! The entries of such a table are offsets and say nothing about where it
//! ends; the table after it does. A function's jump tables are emitted
//! together, back to back in one read-only section, so a table runs exactly
//! to the lowest table of the same function above it. The flow, which holds
//! the translator, decodes the function's extent linearly once ([`function_tables`]:
//! from its entry to the next known entry, or a MiB) before it recovers any
//! table, folds every register through the small lattice the dispatch
//! shapes need, and names the table of each computed jump the fold proves
//! reads one. The model's fallback ([`JumpBasicModel::kuna_try_extent_bound_table`])
//! then re-bounds a table it could not guard to `[0, N)` with `N` the entries
//! that fit before the next one. A function's last table has no such bound
//! and is left as it was: an object the function merely reads may sit
//! anywhere in the section, and a bound taken from one could run the table on
//! into another function's, whose entries are plausible code addresses
//! measured from the wrong base. The run-on cut (`kuna_switchrunon`) still
//! applies to the rows a bound admits.
//!
//! A table a mask bounds only to its selector's width (the `and x8, x0,
//! #0xff` in front of a Swift enum's 45-case string switch) reads 256 rows,
//! and when it is its function's last table the rows past its own cases are
//! the next functions' tables measured from the wrong base. Those rows land
//! all over the next functions and back inside this one, so the run-on cut
//! keeps them. The sanity check of such a table, whose rows leave the
//! function, then asks the flow for the next functions' tables
//! ([`later_tables`]: the extents after this one, until one dispatches
//! through a table above it) and ends the table at the lowest table above
//! it, of this function or of a later one.
//!
//! The same rule reads the table in the discovery Listing
//! (`kuna-analysis/src/listing/kuna_switchextent.rs`), so the case bodies are
//! the dispatcher's own code there too and no gap entry is planted on them.

use std::collections::BTreeSet;

use kuna_base::address::Address;
use kuna_base::space::spacetype;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;
use kuna_sleigh::loadimage::ImageBytes;
use kuna_sleigh::translate::{PcodeEmit, Translate};

use crate::context::{OpId, VarnodeId};
use crate::funcdata::Funcdata;

/// How far past an entry the linear decode reaches when no known entry ends
/// the extent sooner: a Swift copy helper runs to 13 KB, nothing to a MiB.
const MAX_EXTENT: u64 = 1 << 20;

/// How many register values the fold carries: a dispatch chain needs the base
/// and the entry, and a long run of constant materializations must not pile up.
const MAX_TRACKED: usize = 16;

/// The tables the computed jumps in `[entry, end)` dispatch through, sorted,
/// from a linear decode with `translate` over `image` when the flow holds one.
pub fn function_tables(
    translate: &dyn Translate,
    image: Option<&dyn ImageBytes>,
    entry: &Address,
    end: Option<u64>,
) -> Vec<u64> {
    let Some(space) = entry.get_space().cloned() else { return Vec::new() };
    let lo = entry.get_offset();
    let end = end.unwrap_or(u64::MAX).min(lo.saturating_add(MAX_EXTENT));
    let mut fold = Fold { vals: Vec::new(), tables: BTreeSet::new() };
    let mut at = lo;
    while at < end {
        let addr = Address::new(std::rc::Rc::clone(&space), at);
        let len = match image {
            Some(image) => translate.one_instruction_checked(&mut fold, &addr, image),
            None => translate.one_instruction(&mut fold, &addr),
        };
        match len {
            Ok(len) if len > 0 => at = at.wrapping_add(len as u64),
            _ => break,
        }
    }
    fold.tables.into_iter().collect()
}

/// How many functions after this one [`later_tables`] decodes before it
/// gives up on finding a table above the one it bounds.
const MAX_LATER_FUNCTIONS: usize = 32;

/// The tables of the first functions after `end` (this function's end) whose
/// extents dispatch through a table above `above`, sorted: the bound of a
/// table that is its function's last, since the linker lays the next
/// functions' tables out after it. `next_entry` names the known entry after
/// an offset.
pub fn later_tables(
    translate: &dyn Translate,
    image: Option<&dyn ImageBytes>,
    entry: &Address,
    end: u64,
    above: u64,
    next_entry: impl Fn(u64) -> Option<u64>,
) -> Vec<u64> {
    let Some(space) = entry.get_space().cloned() else { return Vec::new() };
    let mut lo = end;
    for _ in 0..MAX_LATER_FUNCTIONS {
        let hi = next_entry(lo);
        let tables = function_tables(translate, image, &Address::new(std::rc::Rc::clone(&space), lo), hi);
        if tables.iter().any(|&t| t > above) {
            return tables;
        }
        match hi {
            Some(hi) if hi > lo => lo = hi,
            _ => break,
        }
    }
    Vec::new()
}

/// What the fold knows about a register: enough to tell a table read from
/// anything else.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Val {
    /// Exactly this constant.
    Const(u64),
    /// This constant plus an unknown: an array address whose index is the
    /// selector.
    Indexed(u64),
    /// A `width`-byte word read out of the array at `table`.
    Entry { table: u64, width: u32 },
    /// An entry scaled to instructions.
    Scaled { table: u64 },
    /// A base plus a scaled or 32-bit entry: a case address.
    Relative { table: u64 },
}

/// A varnode's identity: its space, offset and size.
type Key = (int4, u64, u32);

struct Fold {
    vals: Vec<(Key, Val)>,
    tables: BTreeSet<u64>,
}

impl PcodeEmit for Fold {
    fn dump(&mut self, _addr: &Address, opc: OpCode, outvar: Option<&VarnodeData>, vars: &[VarnodeData]) {
        if opc == OpCode::CPUI_BRANCHIND {
            match vars.first().and_then(|vn| self.value_of(vn)) {
                Some(Val::Relative { table }) => {
                    self.tables.insert(table);
                }
                Some(Val::Entry { table, width }) if vars.first().is_some_and(|vn| vn.size == width) => {
                    self.tables.insert(table);
                }
                _ => {}
            }
        }
        if matches!(opc, OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_CALLOTHER) {
            self.vals.clear();
        }
        let folded = self.fold(opc, outvar, vars);
        let Some(key) = outvar.and_then(key_of) else { return };
        self.vals.retain(|(k, _)| !overlap(*k, key));
        if let Some(val) = folded {
            if self.vals.len() < MAX_TRACKED {
                self.vals.push((key, val));
            }
        }
    }
}

impl Fold {
    fn value_of(&self, vn: &VarnodeData) -> Option<Val> {
        let space = vn.space.as_ref()?;
        if space.get_type() == spacetype::IPTR_CONSTANT {
            return Some(Val::Const(vn.offset & mask(vn.size)));
        }
        let key = key_of(vn)?;
        if let Some((_, v)) = self.vals.iter().rev().find(|(k, _)| *k == key) {
            return Some(*v);
        }
        // The low part of a wider register holding a value: `w8` of `x8`.
        let big = space.is_big_endian();
        self.vals.iter().rev().find_map(|&((s, o, z), v)| {
            if s != key.0 || z <= key.2 {
                return None;
            }
            let low = if big { o + u64::from(z - key.2) == key.1 } else { o == key.1 };
            match v {
                Val::Const(c) if low => Some(Val::Const(c & mask(key.2))),
                Val::Entry { width, .. } if low && width <= key.2 => Some(v),
                _ => None,
            }
        })
    }

    fn fold(&self, opc: OpCode, outvar: Option<&VarnodeData>, vars: &[VarnodeData]) -> Option<Val> {
        let get = |i: usize| -> Option<Val> { self.value_of(vars.get(i)?) };
        let out_size = outvar?.size;
        match opc {
            OpCode::CPUI_COPY => get(0),
            OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT => match get(0)? {
                Val::Const(c) => {
                    let width = vars.first()?.size;
                    Some(Val::Const(if opc == OpCode::CPUI_INT_SEXT { sign_extend(c, width) } else { c }))
                }
                v @ (Val::Entry { .. } | Val::Scaled { .. }) => Some(v),
                Val::Indexed(_) | Val::Relative { .. } => None,
            },
            OpCode::CPUI_INT_ADD => match (get(0), get(1)) {
                (Some(Val::Const(a)), Some(Val::Const(b))) => Some(Val::Const(a.wrapping_add(b) & mask(out_size))),
                (Some(Val::Const(a)), Some(Val::Indexed(b))) | (Some(Val::Indexed(a)), Some(Val::Const(b))) => {
                    Some(Val::Indexed(a.wrapping_add(b)))
                }
                (Some(Val::Entry { table, width: 4 }), Some(Val::Const(_)))
                | (Some(Val::Const(_)), Some(Val::Entry { table, width: 4 }))
                | (Some(Val::Scaled { table }), Some(Val::Const(_)))
                | (Some(Val::Const(_)), Some(Val::Scaled { table })) => Some(Val::Relative { table }),
                // A pointer-wide entry in a pointer-wide add: not a dispatch;
                // a narrower one added to an unknown base is dropped too.
                (Some(Val::Const(a) | Val::Indexed(a)), Some(Val::Entry { width: 1 | 2, .. }))
                | (Some(Val::Entry { width: 1 | 2, .. }), Some(Val::Const(a) | Val::Indexed(a))) => {
                    Some(Val::Indexed(a))
                }
                (Some(Val::Const(a) | Val::Indexed(a)), None) | (None, Some(Val::Const(a) | Val::Indexed(a))) => {
                    Some(Val::Indexed(a))
                }
                _ => None,
            },
            OpCode::CPUI_INT_SUB => match (get(0)?, get(1)?) {
                (Val::Const(a), Val::Const(b)) => Some(Val::Const(a.wrapping_sub(b) & mask(out_size))),
                _ => None,
            },
            OpCode::CPUI_INT_MULT => match (get(0)?, get(1)?) {
                (v, Val::Const(1)) | (Val::Const(1), v) => Some(v),
                (Val::Entry { table, width: 1 | 2 | 4 }, Val::Const(2 | 4))
                | (Val::Const(2 | 4), Val::Entry { table, width: 1 | 2 | 4 }) => Some(Val::Scaled { table }),
                _ => None,
            },
            OpCode::CPUI_INT_LEFT => match (get(0)?, get(1)?) {
                (Val::Entry { table, width: 1 | 2 | 4 }, Val::Const(1..=3)) => Some(Val::Scaled { table }),
                _ => None,
            },
            OpCode::CPUI_LOAD => match get(1)? {
                Val::Indexed(table) | Val::Relative { table } if matches!(out_size, 1 | 2 | 4 | 8) => {
                    Some(Val::Entry { table, width: out_size })
                }
                _ => None,
            },
            _ => None,
        }
    }
}

fn key_of(vn: &VarnodeData) -> Option<Key> {
    let space = vn.space.as_ref()?;
    match space.get_type() {
        spacetype::IPTR_INTERNAL | spacetype::IPTR_PROCESSOR => Some((space.get_index(), vn.offset, vn.size)),
        _ => None,
    }
}

fn overlap(a: Key, b: Key) -> bool {
    a.0 == b.0 && a.1 < b.1.saturating_add(u64::from(b.2)) && b.1 < a.1.saturating_add(u64::from(a.2))
}

fn mask(size: u32) -> u64 {
    if size == 0 || size >= 8 { u64::MAX } else { (1u64 << (8 * size)) - 1 }
}

fn sign_extend(v: u64, size: u32) -> u64 {
    if size == 0 || size >= 8 {
        return v;
    }
    let v = v & mask(size);
    if v >> (8 * size - 1) == 1 { v | !mask(size) } else { v }
}

/// The `(table, stride, index)` of a LOAD whose address is `table + index *
/// stride` (`stride` 1 when the index is not scaled), through the copies and
/// extensions the lifter puts between them.
pub(crate) fn table_read(fd: &Funcdata, load: OpId) -> Option<(u64, u64, VarnodeId)> {
    let ptr = fd.obank().get(load)?.get_in(1)?;
    let add = fd.obank().get(fd.vbank().get(ptr)?.get_def()?)?;
    if add.code() != OpCode::CPUI_INT_ADD {
        return None;
    }
    let (a, b) = (add.get_in(0)?, add.get_in(1)?);
    let (table, scaled) = match (constant(fd, a), constant(fd, b)) {
        (Some(t), None) => (t, b),
        (None, Some(t)) => (t, a),
        _ => return None,
    };
    let mut index = scaled;
    let mut stride = 1;
    loop {
        let Some(def) = fd.vbank().get(index)?.get_def() else { break };
        let op = fd.obank().get(def)?;
        match op.code() {
            OpCode::CPUI_COPY | OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT => index = op.get_in(0)?,
            OpCode::CPUI_INT_LEFT if stride == 1 => {
                stride = 1u64.checked_shl(u32::try_from(constant(fd, op.get_in(1)?)?).ok()?)?;
                index = op.get_in(0)?;
            }
            OpCode::CPUI_INT_MULT if stride == 1 => {
                stride = constant(fd, op.get_in(1)?)?;
                index = op.get_in(0)?;
            }
            _ => break,
        }
    }
    (stride > 0 && stride <= 8).then_some((table, stride, index))
}

fn constant(fd: &Funcdata, vn: VarnodeId) -> Option<u64> {
    let v = fd.vbank().get(vn)?;
    v.is_constant().then(|| v.get_offset())
}

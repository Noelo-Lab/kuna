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
//! A Swift enum's table needs no neighbour. `swift_getEnumCaseMultiPayload`
//! returns a case of the enum whose metadata it is given, and the enum's
//! descriptor states its case count, which `swiftmetadata` records by the
//! enum's metadata accessor. With [`enum_calls`] the extent decode follows an
//! accessor's result into that call and the case it returns into a table,
//! and the table is bounded by the count, also when it is the function's last.
//!
//! The same rule reads the table in the discovery Listing
//! (`kuna-analysis/src/listing/kuna_switchextent.rs`), so the case bodies are
//! the dispatcher's own code there too and no gap entry is planted on them.

use std::collections::{BTreeMap, BTreeSet};

use kuna_base::address::Address;
use kuna_base::space::spacetype;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;
use kuna_sleigh::loadimage::ImageBytes;
use kuna_sleigh::translate::{PcodeEmit, Translate};

use crate::context::{ArchContext, OpId, VarnodeId};
use crate::fspec::effect_type;
use crate::funcdata::Funcdata;

/// How far past an entry the linear decode reaches when no known entry ends
/// the extent sooner: a Swift copy helper runs to 13 KB, nothing to a MiB.
const MAX_EXTENT: u64 = 1 << 20;

/// How many register values the fold carries: a dispatch chain needs the base
/// and the entry, and a long run of constant materializations must not pile
/// up; past it the oldest value other than an enum's metadata or case is
/// dropped.
const MAX_TRACKED: usize = 16;

/// What a direct call's target is to [`function_tables`].
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Callee {
    /// A local Swift enum's metadata accessor; the enum has this many cases.
    EnumAccessor(u64),
    /// `swift_getEnumCaseMultiPayload(value, metadata)`.
    EnumCase,
    Other,
}

/// What [`function_tables`] needs to follow a Swift enum's case into the
/// table its `switch` indexes: the registers a call returns its result in and
/// takes its second argument in, which registers a call preserves, and what
/// each call target is.
pub struct EnumCalls<'a> {
    pub result: VarnodeData,
    pub metadata: VarnodeData,
    pub callee: Box<dyn Fn(u64) -> Callee + 'a>,
    pub saved: Box<dyn Fn(&VarnodeData) -> bool + 'a>,
}

/// The [`EnumCalls`] of a function at `entry` under `arch`, when the image
/// has local Swift enums and the language returns in `x0` / `RAX` and passes
/// the second argument in `x1` / `RSI`.
pub fn enum_calls<'a>(arch: &'a ArchContext, translate: &dyn Translate, entry: &Address) -> Option<EnumCalls<'a>> {
    if arch.swift_enum_cases.is_empty() {
        return None;
    }
    let (result, metadata) = [("x0", "x1"), ("RAX", "RSI")].into_iter().find_map(|(r, m)| {
        Some((translate.get_register_data(r).ok()?, translate.get_register_data(m).ok()?))
    })?;
    let model = arch.default_fp()?;
    let space = entry.get_space()?.clone();
    Some(EnumCalls {
        result,
        metadata,
        callee: Box::new(move |target| {
            if let Some(&cases) = arch.swift_enum_cases.get(&target) {
                return Callee::EnumAccessor(u64::from(cases));
            }
            let name = arch.function_name_at(&Address::new(std::rc::Rc::clone(&space), target));
            match name {
                Some(name) if name.trim_start_matches('_') == "swift_getEnumCaseMultiPayload" => Callee::EnumCase,
                _ => Callee::Other,
            }
        }),
        saved: Box::new(move |vn| {
            let Some(space) = vn.space.clone() else { return false };
            model.has_effect(&Address::new(space, vn.offset), vn.size as int4) == effect_type::UNAFFECTED
        }),
    })
}

/// The jump tables of one function's extent.
#[derive(Default)]
pub struct ExtentTables {
    /// The table of every computed jump the fold proves reads one, sorted.
    pub tables: Vec<u64>,
    /// The tables indexed by a Swift enum's case, with the enum's case count:
    /// `swift_getEnumCaseMultiPayload` returns a case of the enum whose
    /// metadata it is given, so such a table has exactly that many entries.
    pub cases: BTreeMap<u64, u64>,
}

/// The tables the computed jumps in `[entry, end)` dispatch through, from a
/// linear decode with `translate` over `image` when the flow holds one;
/// `calls`, when given, follows Swift enum cases into their tables.
pub fn function_tables(
    translate: &dyn Translate,
    image: Option<&dyn ImageBytes>,
    entry: &Address,
    end: Option<u64>,
    calls: Option<&EnumCalls>,
) -> ExtentTables {
    let Some(space) = entry.get_space().cloned() else { return ExtentTables::default() };
    let lo = entry.get_offset();
    let end = end.unwrap_or(u64::MAX).min(lo.saturating_add(MAX_EXTENT));
    let mut fold =
        Fold { vals: Vec::new(), tables: BTreeSet::new(), cases: BTreeMap::new(), calls, at: u64::MAX, joined: false };
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
    ExtentTables { tables: fold.tables.into_iter().collect(), cases: fold.cases }
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
        let tables = function_tables(translate, image, &Address::new(std::rc::Rc::clone(&space), lo), hi, None).tables;
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
/// anything else. `cases` on the table chain is the enum case count when the
/// index is a Swift enum's case.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Val {
    /// Exactly this constant.
    Const(u64),
    /// This constant plus an unknown: an array address whose index is the
    /// selector.
    Indexed { base: u64, cases: Option<u64> },
    /// A `width`-byte word read out of the array at `table`.
    Entry { table: u64, width: u32, cases: Option<u64> },
    /// An entry scaled to instructions.
    Scaled { table: u64, cases: Option<u64> },
    /// A base plus a scaled or 32-bit entry: a case address.
    Relative { table: u64, cases: Option<u64> },
    /// The metadata of an enum with `cases` cases, held in a register calls
    /// preserve when `saved`.
    Metadata { cases: u64, saved: bool },
    /// A case of such an enum, possibly widened or scaled.
    Case { cases: u64 },
}

/// A varnode's identity: its space, offset and size.
type Key = (int4, u64, u32);

struct Fold<'a> {
    vals: Vec<(Key, Val)>,
    tables: BTreeSet<u64>,
    cases: BTreeMap<u64, u64>,
    calls: Option<&'a EnumCalls<'a>>,
    /// The instruction the last op belonged to, and whether a branch inside
    /// it already passed: what that instruction writes after the branch
    /// joins two values (`csel` picks one of its operands), so it is unknown.
    at: u64,
    joined: bool,
}

impl PcodeEmit for Fold<'_> {
    fn dump(&mut self, addr: &Address, opc: OpCode, outvar: Option<&VarnodeData>, vars: &[VarnodeData]) {
        if addr.get_offset() != self.at {
            self.at = addr.get_offset();
            self.joined = false;
        }
        if opc == OpCode::CPUI_BRANCHIND {
            match vars.first().and_then(|vn| self.value_of(vn)) {
                Some(Val::Relative { table, cases }) => {
                    self.tables.insert(table);
                    if let Some(cases) = cases {
                        self.cases.insert(table, cases);
                    }
                }
                Some(Val::Entry { table, width, .. }) if vars.first().is_some_and(|vn| vn.size == width) => {
                    self.tables.insert(table);
                }
                _ => {}
            }
        }
        if matches!(opc, OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_CALLOTHER) {
            let result = if opc == OpCode::CPUI_CALL { self.call_result(vars) } else { None };
            self.vals.retain(|(_, v)| matches!(v, Val::Metadata { saved: true, .. }));
            if let Some((vn, val)) = result {
                self.set(&vn, Some(val));
            }
            if let Some(vn) = outvar {
                self.set(vn, None);
            }
            return;
        }
        if matches!(opc, OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH)
            && vars.first().and_then(|vn| vn.space.as_ref()).is_some_and(|s| s.get_type() == spacetype::IPTR_CONSTANT)
        {
            self.joined = true;
        }
        let folded = if self.joined { None } else { self.fold(opc, outvar, vars) };
        if let Some(vn) = outvar {
            self.set(vn, folded);
        }
    }
}

impl Fold<'_> {
    fn set(&mut self, vn: &VarnodeData, val: Option<Val>) {
        let Some(key) = key_of(vn) else { return };
        self.vals.retain(|(k, _)| !overlap(*k, key));
        let val = match val {
            Some(Val::Metadata { cases, .. }) => {
                Some(Val::Metadata { cases, saved: self.calls.is_some_and(|c| (c.saved)(vn)) })
            }
            v => v,
        };
        if let Some(val) = val {
            if self.vals.len() >= MAX_TRACKED {
                let oldest = self
                    .vals
                    .iter()
                    .position(|(_, v)| !matches!(v, Val::Metadata { .. } | Val::Case { .. }))
                    .unwrap_or(0);
                self.vals.remove(oldest);
            }
            self.vals.push((key, val));
        }
    }

    /// The result register of a direct call and what it holds after the
    /// call: an enum's metadata from its accessor, or a case of the enum
    /// whose metadata `swift_getEnumCaseMultiPayload` is given.
    fn call_result(&self, vars: &[VarnodeData]) -> Option<(VarnodeData, Val)> {
        let calls = self.calls?;
        let target = vars.first()?;
        let val = match (calls.callee)(target.offset) {
            Callee::EnumAccessor(cases) => Val::Metadata { cases, saved: false },
            Callee::EnumCase => match self.value_of(&calls.metadata)? {
                Val::Metadata { cases, .. } => Val::Case { cases },
                _ => return None,
            },
            Callee::Other => return None,
        };
        Some((calls.result.clone(), val))
    }

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
                Val::Case { .. } if low => Some(v),
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
                v @ (Val::Entry { .. } | Val::Scaled { .. } | Val::Case { .. }) => Some(v),
                Val::Indexed { .. } | Val::Relative { .. } | Val::Metadata { .. } => None,
            },
            OpCode::CPUI_INT_ADD => match (get(0), get(1)) {
                (Some(Val::Const(a)), Some(Val::Const(b))) => Some(Val::Const(a.wrapping_add(b) & mask(out_size))),
                (Some(Val::Const(a)), Some(Val::Indexed { base, cases }))
                | (Some(Val::Indexed { base, cases }), Some(Val::Const(a))) => {
                    Some(Val::Indexed { base: a.wrapping_add(base), cases })
                }
                (Some(Val::Const(base)), Some(Val::Case { cases })) | (Some(Val::Case { cases }), Some(Val::Const(base))) => {
                    Some(Val::Indexed { base, cases: Some(cases) })
                }
                (Some(Val::Entry { table, width: 4, cases }), Some(Val::Const(_)))
                | (Some(Val::Const(_)), Some(Val::Entry { table, width: 4, cases }))
                | (Some(Val::Scaled { table, cases }), Some(Val::Const(_)))
                | (Some(Val::Const(_)), Some(Val::Scaled { table, cases })) => Some(Val::Relative { table, cases }),
                // A pointer-wide entry in a pointer-wide add: not a dispatch;
                // a narrower one added to an unknown base is dropped too.
                (Some(Val::Const(base) | Val::Indexed { base, .. }), Some(Val::Entry { width: 1 | 2, .. }))
                | (Some(Val::Entry { width: 1 | 2, .. }), Some(Val::Const(base) | Val::Indexed { base, .. })) => {
                    Some(Val::Indexed { base, cases: None })
                }
                (Some(Val::Const(base) | Val::Indexed { base, .. }), None)
                | (None, Some(Val::Const(base) | Val::Indexed { base, .. })) => Some(Val::Indexed { base, cases: None }),
                _ => None,
            },
            OpCode::CPUI_INT_SUB => match (get(0)?, get(1)?) {
                (Val::Const(a), Val::Const(b)) => Some(Val::Const(a.wrapping_sub(b) & mask(out_size))),
                _ => None,
            },
            OpCode::CPUI_INT_MULT => match (get(0)?, get(1)?) {
                (v, Val::Const(1)) | (Val::Const(1), v) => Some(v),
                (Val::Entry { table, width: 1 | 2 | 4, cases }, Val::Const(2 | 4))
                | (Val::Const(2 | 4), Val::Entry { table, width: 1 | 2 | 4, cases }) => Some(Val::Scaled { table, cases }),
                (v @ Val::Case { .. }, Val::Const(2 | 4 | 8)) | (Val::Const(2 | 4 | 8), v @ Val::Case { .. }) => Some(v),
                _ => None,
            },
            OpCode::CPUI_INT_LEFT => match (get(0)?, get(1)?) {
                (Val::Entry { table, width: 1 | 2 | 4, cases }, Val::Const(1..=3)) => Some(Val::Scaled { table, cases }),
                (v @ Val::Case { .. }, Val::Const(0..=3)) => Some(v),
                _ => None,
            },
            OpCode::CPUI_LOAD => match get(1)? {
                Val::Indexed { base: table, cases } | Val::Relative { table, cases }
                    if matches!(out_size, 1 | 2 | 4 | 8) =>
                {
                    Some(Val::Entry { table, width: out_size, cases })
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

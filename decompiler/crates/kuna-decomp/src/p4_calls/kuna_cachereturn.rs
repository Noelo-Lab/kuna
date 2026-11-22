//! (kuna) `cachereturn` -- a function whose return register holds the value of
//! one memory cell on every path returns it (P4 output prototype).
//!
//! A lazy accessor reads a cache cell, returns what it found when it is set,
//! and otherwise computes the value, stores it into the cell and returns it:
//! `void *get(void) { void *v = g; if (v) return v; v = make(); g = v; return v; }`.
//! At the RETURN the register is the cell's load on one path and the value
//! stored into the cell on the other. Upstream's `ancestorOpUse` refuses both:
//! the load is also branched on and the stored value is also stored, so the
//! function is recovered `void` and its callers read a result it does not
//! have. Swift's lazy witness-table and metadata accessors and its
//! mangled-name type cache are this shape.
//!
//! A return trial in an output register of the model is taken when, at
//! every live RETURN, each value reaching it through MULTIEQUAL, COPY and
//! INDIRECT is either a LOAD of the cell or a value some STORE writes into the
//! cell, at least one of each, every access of the trial's size and through one
//! pointer: the same Varnode, the same constant, or the same operation on such
//! pointers. A `void` function that tests the cell compiles its load into a
//! scratch register (`ldr x8,[g]`), so it does not match. x86-64 reads a
//! global straight from its address, a COPY of the global and no LOAD, so its
//! accessors of a global are not taken.

use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::funcdata::Funcdata;
use crate::p0_knowledge::options::on_or_off;

/// `option cachereturn on|off`.
pub struct OptionCacheReturn;

impl OptionCacheReturn {
    /// The option name.
    pub const NAME: &'static str = "cachereturn";

    /// Resolve the flag and its confirmation message; the caller writes it into
    /// `Architecture::cache_return`.
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("Cache-cell return recovery turned {prop}")))
    }
}

/// How many values one RETURN may merge before the test gives up.
const MAX_LEAVES: usize = 16;

/// How deep [`same_pointer`] compares two pointer expressions.
const POINTER_DEPTH: usize = 4;

/// Does every live RETURN in `return_ops` hand back, in the trial at `slot`
/// (`[addr, addr+size)`, an output register of the model), the value of one
/// memory cell: its load, or what is stored into it?
pub fn returns_cached(data: &Funcdata, return_ops: &[OpId], slot: int4, addr: &Address, size: int4) -> bool {
    if !data.get_arch().cache_return
        || data.get_func_proto().is_output_locked()
        || !data.get_func_proto().model().possible_output_param(addr, size)
    {
        return false;
    }
    let live: Vec<OpId> = return_ops
        .iter()
        .copied()
        .filter(|&r| data.obank().get(r).is_some_and(|o| !o.is_dead() && o.get_halt_type() == 0))
        .collect();
    let mut cell: Option<(VarnodeId, u64)> = None;
    let (mut loaded, mut stored) = (false, false);
    for &r in &live {
        let Some(vn) = data.obank().get(r).and_then(|o| o.get_in(slot)) else { return false };
        if !data.vbank().get(vn).is_some_and(|v| v.get_addr() == addr && v.get_size() == size) {
            return false;
        }
        let Some(leaves) = leaves(data, vn) else { return false };
        for leaf in leaves {
            let Some((pointer, space, load)) = access(data, leaf, size) else { return false };
            match cell {
                None => cell = Some((pointer, space)),
                Some((p, s)) if s == space && same_pointer(data, p, pointer, POINTER_DEPTH) => {}
                Some(_) => return false,
            }
            if load {
                loaded = true;
            } else {
                stored = true;
            }
        }
    }
    loaded && stored
}

/// The values `vn` merges, through MULTIEQUAL, COPY and an INDIRECT that does
/// not create it.
fn leaves(data: &Funcdata, vn: VarnodeId) -> Option<Vec<VarnodeId>> {
    let mut work = vec![vn];
    let mut seen = Vec::new();
    let mut out = Vec::new();
    while let Some(v) = work.pop() {
        if seen.contains(&v) {
            continue;
        }
        seen.push(v);
        if seen.len() > MAX_LEAVES {
            return None;
        }
        let def = data.vbank().get(v)?.get_def().and_then(|d| data.obank().get(d));
        match def {
            Some(o) if o.code() == OpCode::CPUI_MULTIEQUAL => {
                work.extend((0..o.num_input()).filter_map(|k| o.get_in(k)));
            }
            Some(o) if o.code() == OpCode::CPUI_COPY => work.extend(o.get_in(0)),
            Some(o) if o.code() == OpCode::CPUI_INDIRECT && !o.is_indirect_creation() => work.extend(o.get_in(0)),
            _ => out.push(v),
        }
    }
    Some(out)
}

/// The pointer and space of the cell `leaf` comes from or is stored into, and
/// whether it is that cell's load.
fn access(data: &Funcdata, leaf: VarnodeId, size: int4) -> Option<(VarnodeId, u64, bool)> {
    let node = data.vbank().get(leaf)?;
    let space = |o: &crate::op::PcodeOp| o.get_in(0).and_then(|s| data.vbank().get(s)).map(|s| s.get_offset());
    if let Some(o) = node.get_def().and_then(|d| data.obank().get(d)).filter(|o| o.code() == OpCode::CPUI_LOAD) {
        return Some((o.get_in(1)?, space(o)?, true));
    }
    node.descend_iter().find_map(|d| {
        let o = data.obank().get(d).filter(|o| !o.is_dead() && o.code() == OpCode::CPUI_STORE)?;
        (o.get_in(2) == Some(leaf) && node.get_size() == size).then_some(())?;
        Some((o.get_in(1)?, space(o)?, false))
    })
}

/// Do `a` and `b` compute the same pointer: the same Varnode once copies are
/// stripped, the same constant, or the same operation on such pointers?
fn same_pointer(data: &Funcdata, a: VarnodeId, b: VarnodeId, depth: usize) -> bool {
    let strip = |mut v: VarnodeId| {
        for _ in 0..POINTER_DEPTH {
            match data.vbank().get(v).and_then(|n| n.get_def()).and_then(|d| data.obank().get(d)) {
                Some(o) if o.code() == OpCode::CPUI_COPY => match o.get_in(0) {
                    Some(i) => v = i,
                    None => break,
                },
                _ => break,
            }
        }
        v
    };
    let (a, b) = (strip(a), strip(b));
    if a == b {
        return true;
    }
    if let (Some(x), Some(y)) = (constant(data, a, POINTER_DEPTH), constant(data, b, POINTER_DEPTH)) {
        return x == y;
    }
    let (Some(x), Some(y)) = (data.vbank().get(a), data.vbank().get(b)) else { return false };
    if x.is_constant() || y.is_constant() {
        return x.is_constant() && y.is_constant() && x.get_offset() == y.get_offset() && x.get_size() == y.get_size();
    }
    if depth == 0 {
        return false;
    }
    let def = |n: &crate::varnode::Varnode| n.get_def().and_then(|d| data.obank().get(d));
    let (Some(p), Some(q)) = (def(x), def(y)) else { return false };
    matches!(p.code(), OpCode::CPUI_INT_ADD | OpCode::CPUI_PTRSUB | OpCode::CPUI_PTRADD)
        && p.code() == q.code()
        && p.num_input() == q.num_input()
        && (0..p.num_input()).all(|k| match (p.get_in(k), q.get_in(k)) {
            (Some(i), Some(j)) => same_pointer(data, i, j, depth - 1),
            _ => false,
        })
}

/// The constant `vn` computes through COPY, INT_ADD and PTRSUB of constants
/// (an `adrp` page plus its low offset before the rules fold them).
fn constant(data: &Funcdata, vn: VarnodeId, depth: usize) -> Option<u64> {
    let node = data.vbank().get(vn)?;
    if node.is_constant() {
        return Some(node.get_offset());
    }
    let o = data.obank().get(node.get_def()?)?;
    let mask = if node.get_size() >= 8 { u64::MAX } else { (1u64 << (node.get_size() * 8)) - 1 };
    match o.code() {
        _ if depth == 0 => None,
        OpCode::CPUI_COPY => constant(data, o.get_in(0)?, depth - 1),
        OpCode::CPUI_INT_ADD | OpCode::CPUI_PTRSUB => {
            let (l, r) = (constant(data, o.get_in(0)?, depth - 1)?, constant(data, o.get_in(1)?, depth - 1)?);
            Some(l.wrapping_add(r) & mask)
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests;

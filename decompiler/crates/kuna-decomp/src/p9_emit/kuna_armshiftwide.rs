//! (kuna `armshiftguard wide`) The register shift counts of a lowered
//! double-word shift.
//!
//! 32-bit ARM has no 64-bit shift. gcc and clang inline one, and libgcc's
//! `__aeabi_llsl`/`__aeabi_llsr`/`__aeabi_lasr` implement one, by shifting
//! each word by the program's count `n` and by its complements `n - 32` and
//! `32 - n`, then ORing the pieces. Those complements are out of range for
//! half of all counts, and the sequence is correct only because an ARM
//! register shift by 32 through 255 yields 0. The same holds for a register
//! `ROR`, which SLEIGH lifts as `x >> s | x << (32 - s)`.
//!
//! The signature is two shift counts on one value with opposite coefficients
//! (`n + k` and `32 - n - k`, or `n - 32` and `32 - n`): a source shift by
//! `n - 32` beside `n` does not form it. Counts compare modulo 256 because
//! ARM shifts by the count's low byte; a constant the program added to `n`
//! folds into the complement (`n - 0x21`, `0x21 - n` for `1ULL << (n - 1)`)
//! and a scaled count keeps its factor (`i * 4 - 0x20`, `i * -4 + 0x20`).
//! Counts that reach their shifts through a join (a peeled first iteration, a
//! label entered from two paths) are followed through the MULTIEQUAL: a shift
//! by a phi shifts by each of its inputs on that input's edge. Every left
//! shift by any count connected to such a pair keeps its guard, including `n`
//! itself, which a lowered left shift needs to yield 0 too.

use std::collections::{HashMap, HashSet};

use crate::context::VarnodeId;
use crate::funcdata::Funcdata;
use kuna_num::opcodes::OpCode;

/// A count's low byte as `coef * atom + add`, all modulo 256.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct AffineCount {
    pub(crate) atom: VarnodeId,
    pub(crate) coef: u8,
    pub(crate) add: u8,
}

const WALK_DEPTH: usize = 16;
const COMPONENT_BUDGET: usize = 512;
const PHI_DEPTH: usize = 4;

/// Whether `count`, the count input of a register shift, is connected by
/// affine steps and joins to a value that two of the function's register
/// shift counts treat as the count of a lowered double-word shift. A walk
/// that runs out of budget answers yes, which keeps the guard.
pub(crate) fn double_word_count(fd: &Funcdata, count: VarnodeId) -> bool {
    let Some(start) = affine_count(fd, count) else { return false };
    let mut walk = Component {
        fd,
        atoms: vec![start.atom],
        known: HashSet::from([start.atom]),
        groups: HashMap::new(),
        visited: 0,
    };
    let mut next = 0;
    while let Some(&atom) = walk.atoms.get(next) {
        next += 1;
        for input in phi_inputs(fd, atom) {
            if let Some(form) = affine_count(fd, input) {
                walk.join(form.atom);
            }
        }
        let Some(family) = affine_family(fd, atom, &mut walk.visited) else { return true };
        for (vn, coef, add) in family {
            let Some(v) = fd.vbank().get(vn) else { continue };
            for reader in v.descend_iter() {
                let Some(op) = fd.obank().get(reader) else { continue };
                if register_shift(op.code()) && op.get_in(1) == Some(vn) {
                    walk.record(atom, coef, add, PHI_DEPTH);
                } else if op.code() == OpCode::CPUI_MULTIEQUAL {
                    if let Some(out) = op.get_out() {
                        walk.join(out);
                    }
                }
            }
        }
        if walk.visited > COMPONENT_BUDGET || walk.atoms.len() > COMPONENT_BUDGET {
            return true;
        }
    }
    walk.groups.values().any(|forms| complement_pair(forms))
}

/// The atoms reached so far and the shift count forms recorded on each.
struct Component<'a> {
    fd: &'a Funcdata,
    atoms: Vec<VarnodeId>,
    known: HashSet<VarnodeId>,
    groups: HashMap<VarnodeId, Vec<(u8, u8)>>,
    visited: usize,
}

impl Component<'_> {
    fn join(&mut self, atom: VarnodeId) {
        if self.known.insert(atom) {
            self.atoms.push(atom);
        }
    }

    /// A shift by `coef * atom + add`; when `atom` is a phi, also a shift by
    /// each of its inputs, recorded on that input's own atom.
    fn record(&mut self, atom: VarnodeId, coef: u8, add: u8, depth: usize) {
        self.groups.entry(atom).or_default().push((coef, add));
        if depth == 0 {
            return;
        }
        for input in phi_inputs(self.fd, atom) {
            let Some(form) = affine_count(self.fd, input) else { continue };
            if form.atom == atom {
                continue;
            }
            self.join(form.atom);
            let (c, k) = (coef.wrapping_mul(form.coef), coef.wrapping_mul(form.add).wrapping_add(add));
            if c != 0 {
                self.record(form.atom, c, k, depth - 1);
            }
        }
    }
}

fn register_shift(code: OpCode) -> bool {
    matches!(code, OpCode::CPUI_INT_LEFT | OpCode::CPUI_INT_RIGHT | OpCode::CPUI_INT_SRIGHT)
}

/// The non-constant inputs of `vn`'s defining MULTIEQUAL, if it has one.
fn phi_inputs(fd: &Funcdata, vn: VarnodeId) -> Vec<VarnodeId> {
    let Some(op) = fd.vbank().get(vn).and_then(|v| v.get_def()).and_then(|d| fd.obank().get(d)) else {
        return Vec::new();
    };
    if op.code() != OpCode::CPUI_MULTIEQUAL {
        return Vec::new();
    }
    (0..op.num_input())
        .filter_map(|slot| op.get_in(slot))
        .filter(|&x| fd.vbank().get(x).is_some_and(|v| !v.is_constant()))
        .collect()
}

/// `count`'s low byte as an affine function of the first value its
/// definition chain does not compute by an affine step; `None` when that
/// low byte is a constant.
pub(crate) fn affine_count(fd: &Funcdata, count: VarnodeId) -> Option<AffineCount> {
    let (mut atom, mut coef, mut add) = (count, 1u8, 0u8);
    for _ in 0..WALK_DEPTH {
        let Some((input, c, k)) = affine_step(fd, atom) else { break };
        add = add.wrapping_add(coef.wrapping_mul(k));
        coef = coef.wrapping_mul(c);
        atom = input;
        if coef == 0 {
            return None;
        }
    }
    Some(AffineCount { atom, coef, add })
}

/// `vn = c * input + k (mod 256)` for `vn`'s defining op, when it is one of
/// the steps that keep the low byte affine in a single non-constant input.
fn affine_step(fd: &Funcdata, vn: VarnodeId) -> Option<(VarnodeId, u8, u8)> {
    let op = fd.obank().get(fd.vbank().get(vn)?.get_def()?)?;
    let constant = |slot: i32| {
        op.get_in(slot)
            .and_then(|x| fd.vbank().get(x))
            .filter(|x| x.is_constant())
            .map(|x| x.get_offset())
    };
    let variable = |slot: i32| {
        op.get_in(slot)
            .filter(|&x| fd.vbank().get(x).is_some_and(|v| !v.is_constant()))
    };
    let binary = || match (constant(0), constant(1)) {
        (None, Some(c)) => Some((variable(0)?, c)),
        (Some(c), None) => Some((variable(1)?, c)),
        _ => None,
    };
    match op.code() {
        OpCode::CPUI_COPY | OpCode::CPUI_CAST | OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT => {
            Some((variable(0)?, 1, 0))
        }
        OpCode::CPUI_SUBPIECE if constant(1) == Some(0) => Some((variable(0)?, 1, 0)),
        OpCode::CPUI_INT_AND => match binary()? {
            (x, mask) if mask & 0xff == 0xff => Some((x, 1, 0)),
            _ => None,
        },
        OpCode::CPUI_INT_ADD => binary().map(|(x, c)| (x, 1, c as u8)),
        OpCode::CPUI_INT_SUB => match (constant(0), constant(1)) {
            (None, Some(c)) => Some((variable(0)?, 1, (c as u8).wrapping_neg())),
            (Some(c), None) => Some((variable(1)?, 0xff, c as u8)),
            _ => None,
        },
        OpCode::CPUI_INT_2COMP => Some((variable(0)?, 0xff, 0)),
        OpCode::CPUI_INT_NEGATE => Some((variable(0)?, 0xff, 0xff)),
        OpCode::CPUI_INT_MULT => binary().map(|(x, c)| (x, c as u8, 0)),
        OpCode::CPUI_INT_LEFT => match (variable(0), constant(1)) {
            (Some(x), Some(c)) if c < 8 => Some((x, 1u8 << c, 0)),
            _ => None,
        },
        _ => None,
    }
}

/// Every value derived from `atom` by the steps [`affine_step`] reads
/// backwards, with its form, `atom` itself first; `None` past the budget.
fn affine_family(fd: &Funcdata, atom: VarnodeId, visited: &mut usize) -> Option<Vec<(VarnodeId, u8, u8)>> {
    let mut family = vec![(atom, 1u8, 0u8)];
    let mut seen = HashSet::from([atom]);
    let mut next = 0;
    while let Some(&(vn, coef, add)) = family.get(next) {
        next += 1;
        for reader in fd.vbank().get(vn)?.descend_iter() {
            let Some(out) = fd.obank().get(reader).and_then(|op| op.get_out()) else { continue };
            if seen.contains(&out) {
                continue;
            }
            let Some((input, c, k)) = affine_step(fd, out) else { continue };
            if input != vn {
                continue;
            }
            seen.insert(out);
            *visited += 1;
            if *visited > COMPONENT_BUDGET {
                return None;
            }
            let (coef, add) = (coef.wrapping_mul(c), c.wrapping_mul(add).wrapping_add(k));
            if coef != 0 {
                family.push((out, coef, add));
            }
        }
    }
    Some(family)
}

/// Whether two forms have opposite non-zero coefficients and constants that
/// sum to a multiple of 32: a count and its double-word complement.
pub(crate) fn complement_pair(forms: &[(u8, u8)]) -> bool {
    forms.iter().enumerate().any(|(i, &(a, k))| {
        forms[i + 1..]
            .iter()
            .any(|&(b, l)| a != 0 && b == a.wrapping_neg() && k.wrapping_add(l) % 32 == 0)
    })
}

#[cfg(test)]
mod tests;

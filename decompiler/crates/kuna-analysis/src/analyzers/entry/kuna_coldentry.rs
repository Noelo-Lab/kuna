//! (kuna) `coldentry` — the extra entry points of a multi-entry `.cold` fragment
//! (P1 code/data partition).
//!
//! GCC's hot/cold splitting moves a function's unlikely blocks into a separate
//! `foo.cold` fragment with its own `.eh_frame` FDE. When several unlikely paths
//! of the hot function are split out, they are laid back to back inside that ONE
//! fragment, and each is reached by its own `jmp`/`jcc rel32` from the hot body:
//!
//! ```text
//! 30c20:  mov esi,0x10 ; ... ; call xmalloc_fail ; ud2   <- FDE [0x30c20,0x30c5a) start
//! 30c3d:  mov esi,0x10 ; ... ; call xmalloc_fail ; ud2   <- `jmp 30c3d` from 0x34873
//! ```
//!
//! The FDE oracle yields only `0x30c20`; `0x30c3d` falls strictly inside that
//! body, so no metadata oracle names it and the decompile of the hot function
//! inlines it through the jump. Ghidra makes it a function of its own (its jump
//! target is reached from outside every body that contains it), and on Ubuntu
//! 22.04's stripped `/bin/bash` that is 18 functions in `[0x30c3d, 0x312b3]`
//! that kuna's inventory lacked.
//!
//! ## What counts as one
//!
//! An address `t` strictly inside a single-function FDE body `[b, e)` (the same
//! eligibility [`super::kuna_fdeinterior`] uses) is an entry when:
//!
//! 1. a direct `jmp rel32` / `jcc rel32` OUTSIDE `[b, e)` targets it — found by a
//!    byte scan of the executable sections for the `E9` / `0F 8x` encodings, then
//!    confirmed by decoding the candidate source and checking the decoded branch
//!    names `t`;
//! 2. a linear decode of `[b, e)` from `b` lands on `t` as an instruction
//!    boundary; and
//! 3. the instruction before `t` has no fall-through (`ud2`, `jmp`, `ret`), so
//!    nothing inside the fragment flows into it — `t` is reached only from outside.
//!
//! Guard 3 is what keeps an ordinary function's shared tail (a block reached both
//! by its own fall-through and by another function's jump) from being split. An
//! indirect `jmp` does not satisfy it: the block after one is a switch case as
//! often as an entry.
//!
//! ## Scope
//!
//! x86 / x86-64 ELF only (the encodings and the FDE are both load-bearing), and
//! inert on any image without `.eh_frame`. A byte-scan candidate is decoded only
//! when the bytes before it end in a no-fall-through encoding, and a body only
//! up to its last confirmed candidate, so the cost is one pass over the
//! executable bytes plus short decodes of the fragments that qualify. The entries are reported on
//! [`AnalysisOutput::fde_interior_entries`], which the commit adds after the
//! `fdeinterior` suppression, since by construction every one is FDE-interior.

use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use kuna_base::space::AddrSpace;
use kuna_sleigh::translate::Translate;
use object::read::Object;

use crate::listing::classify::classify;
use crate::listing::decode::decode_one;
use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, Phase};

use super::executable_sections;
use super::kuna_fdeinterior::eligible_fde_bodies;

/// The longest FDE body this pass will linearly decode.
const MAX_BODY: u64 = 0x4000;

/// (kuna) The multi-entry cold-fragment pass (`coldentry`), gated at the commit.
pub struct ColdEntryPass;

impl AnalysisPass for ColdEntryPass {
    fn phase(&self) -> Phase {
        Phase::P1
    }

    fn id(&self) -> &'static str {
        "coldentry"
    }

    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        let file = ctx.file;
        if !matches!(file.format(), object::BinaryFormat::Elf)
            || !matches!(
                file.architecture(),
                object::Architecture::X86_64 | object::Architecture::I386
            )
        {
            return AnalysisOutput::default();
        }
        let Some(code_space) = ctx.arch.manage().get_default_code_space() else {
            return AnalysisOutput::default();
        };
        let bodies = eligible_fde_bodies(file, ctx.bytes);
        let execs = executable_sections(file);
        AnalysisOutput {
            fde_interior_entries: cold_entries(ctx.arch.translate(), code_space, &bodies, &execs),
            ..AnalysisOutput::default()
        }
    }
}

/// The FDE-interior addresses that are entered only by a jump from outside the
/// FDE body. `bodies` must be sorted and disjoint.
pub fn cold_entries(
    translate: &dyn Translate,
    code_space: &Rc<AddrSpace>,
    bodies: &[(u64, u64)],
    execs: &[(u64, u64, Vec<u8>)],
) -> Vec<u64> {
    if bodies.is_empty() {
        return Vec::new();
    }
    let mut by_body: BTreeMap<(u64, u64), BTreeMap<u64, Vec<u64>>> = BTreeMap::new();
    for (sec_vma, _, data) in execs {
        for (src, target) in rel32_branches(*sec_vma, data) {
            let Some(body) = containing_body(bodies, target) else { continue };
            if body.0 == target
                || (body.0 <= src && src < body.1)
                || body.1 - body.0 > MAX_BODY
                || !ends_without_fallthrough(*sec_vma, data, target)
            {
                continue;
            }
            by_body.entry(body).or_default().entry(target).or_default().push(src);
        }
    }
    let mut out = Vec::new();
    for ((start, _), targets) in by_body {
        let confirmed: Vec<u64> = targets
            .into_iter()
            .filter(|(target, srcs)| {
                srcs.iter().any(|&src| branches_to(translate, code_space, src, *target))
            })
            .map(|(target, _)| target)
            .collect();
        let Some(&last) = confirmed.last() else { continue };
        let entered = after_terminator(translate, code_space, start, last);
        out.extend(confirmed.into_iter().filter(|t| entered.contains(t)));
    }
    out
}

/// Every `(source, target)` of a byte pattern that decodes as `jmp rel32` (`E9`)
/// or `jcc rel32` (`0F 80`..`0F 8F`) in a section mapped at `vma`.
fn rel32_branches(vma: u64, data: &[u8]) -> impl Iterator<Item = (u64, u64)> + '_ {
    (0..data.len()).filter_map(move |i| {
        let (op_len, rel_at) = match data[i] {
            0xE9 => (5, i + 1),
            0x0F if data.get(i + 1).is_some_and(|b| b & 0xF0 == 0x80) => (6, i + 2),
            _ => return None,
        };
        let rel = data.get(rel_at..rel_at + 4)?;
        let rel = i32::from_le_bytes([rel[0], rel[1], rel[2], rel[3]]) as i64;
        let src = vma + i as u64;
        Some((src, (src + op_len).wrapping_add_signed(rel)))
    })
}

/// Do the bytes just before `target` (in the section mapped at `vma`) end in one
/// of the no-fall-through encodings the decode then confirms: `ud2`, `ret`,
/// `ret imm16`, `hlt`, `jmp rel8` or `jmp rel32`?
fn ends_without_fallthrough(vma: u64, data: &[u8], target: u64) -> bool {
    let Some(at) = target.checked_sub(vma).map(|o| o as usize) else { return false };
    let back = |n: usize| at.checked_sub(n).and_then(|i| data.get(i)).copied();
    matches!(back(1), Some(0xC3 | 0xF4))
        || (back(2) == Some(0x0F) && back(1) == Some(0x0B))
        || back(2) == Some(0xEB)
        || back(3) == Some(0xC2)
        || back(5) == Some(0xE9)
}

/// The body that holds `vma`, if any.
fn containing_body(bodies: &[(u64, u64)], vma: u64) -> Option<(u64, u64)> {
    let idx = bodies.partition_point(|&(start, _)| start <= vma);
    let body = *bodies.get(idx.checked_sub(1)?)?;
    (vma < body.1).then_some(body)
}

/// The instruction boundaries in `(start, last]` whose preceding instruction has
/// no fall-through, by a linear decode from `start`.
fn after_terminator(
    translate: &dyn Translate,
    code_space: &Rc<AddrSpace>,
    start: u64,
    last: u64,
) -> BTreeSet<u64> {
    let mut out = BTreeSet::new();
    let mut pc = start;
    while pc < last {
        let Ok(d) = decode_one(translate, pc, code_space, false, false) else { break };
        if d.len == 0 {
            break;
        }
        let next = pc + d.len as u64;
        if !classify(&d.ops, pc, d.len).flow.has_fallthrough {
            out.insert(next);
        }
        pc = next;
    }
    out
}

/// Does the instruction at `src` decode as a branch whose target is `target`?
fn branches_to(translate: &dyn Translate, code_space: &Rc<AddrSpace>, src: u64, target: u64) -> bool {
    let Ok(d) = decode_one(translate, src, code_space, false, false) else { return false };
    let c = classify(&d.ops, src, d.len);
    c.flow.is_jump && !c.flow.is_call && c.flows.contains(&target)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rel32_patterns() {
        let data = [0x90, 0xE9, 0x10, 0, 0, 0, 0x0F, 0x84, 0xF0, 0xFF, 0xFF, 0xFF];
        let got: Vec<_> = rel32_branches(0x1000, &data).collect();
        assert!(got.contains(&(0x1001, 0x1016)));
        assert!(got.contains(&(0x1006, 0x1006 + 6 - 0x10)));
    }

    #[test]
    fn containing_body_is_half_open() {
        let bodies = [(0x1000, 0x1100), (0x2000, 0x2010)];
        assert_eq!(containing_body(&bodies, 0x1000), Some((0x1000, 0x1100)));
        assert_eq!(containing_body(&bodies, 0x10ff), Some((0x1000, 0x1100)));
        assert_eq!(containing_body(&bodies, 0x1100), None);
        assert_eq!(containing_body(&bodies, 0x0fff), None);
    }
}

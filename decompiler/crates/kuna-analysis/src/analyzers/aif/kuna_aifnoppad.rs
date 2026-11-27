//! (kuna) `aifnoppad` — no AIF gap entry starts on alignment padding or zero fill
//! (P1 code/data partition).
//!
//! # The defect (GH-299)
//!
//! [`super::run_aif`] probes every hole the recursive-descent walk left, and the
//! holes between functions are mostly filler. Filler is normally refused by the
//! fingerprint test, since no four discovered functions open with two `nop`s — but
//! some images do have them. Wine's unimplemented-function stubs are exported and
//! open with nine `nop`s, so `nop, nop` is a fingerprint the discovered functions
//! share, and the `nop`s between each relay thunk's `ret` and the next thunk's
//! hot-patchable `mov edi,edi` entry then pass both acceptance tests: a fingerprint
//! match, and a valid subroutine that falls through into decoded code. Each run
//! becomes a `sub_<addr>` in front of a function kuna already has. On ARM firmware,
//! zero halfwords around a literal pool are accepted the same way, and an accepted
//! zero run swallows the function behind it.
//!
//! # The rule
//!
//! [`PaddingRuns::classify`] reads the run of filler a candidate opens with:
//!
//!  1. zero fill (x86 `add [eax],al`, A32 `andeq r0,r0,r0`, Thumb `movs r0,r0`),
//!     which no function opens with, is never probed;
//!  2. padding (`nop` of any width, `int3`, a self-`lea`, or a register self-move
//!     on a 32-bit target) is not probed when the run ends exactly at a known
//!     function entry: it is the filler in front of that function;
//!  3. padding that ends at undecoded code is probed as before, but an accept is
//!     planted on the first instruction after it, where the function the walk
//!     validated begins.
//!
//! Padding is never refused in front of undecoded code because it is also a
//! legitimate first instruction: 903 symbol-table function starts in the i386 PE
//! corpus open with `nop` (Wine's stubs) and 56 in the ARM corpus (an empty `-O0`
//! function is `nop; bx lr`). `mov edi,edi` is never padding — it is the MSVC
//! hot-patch prologue that opens 7,827 true starts in the PE corpus — and neither
//! is any self-move on a 64-bit target, where a 32-bit one zero-extends. A self-LEA
//! on a wide target is padding only with a full-width x86-64 register.
//!
//! A refusal is a plain reject, not a body claim: the cursor moves on through the
//! filler and probes what follows it as before.

use std::collections::HashMap;

use crate::listing::Listing;

use super::{GapDecoder, ProbedInsn};

/// What a gap candidate is, read from the filler it opens with.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Filler {
    /// The candidate does not open with filler: probe it as usual.
    No,
    /// Zero fill, or padding in front of a function kuna has: do not probe it.
    Refuse,
    /// Padding in front of undecoded code that starts at the address held: probe
    /// the candidate, and plant an accept there instead.
    Before(u64),
    /// The validator cannot reach the code after this run within its instruction budget.
    Blocked,
}

#[derive(Clone, Copy)]
enum RunEnd {
    Entry,
    BeyondValidationLimit,
    // Minimum padding visits before the destination. A terminal makes it unreachable.
    Code { at: u64, min_visits: u32 },
    Other { blocked: bool },
}

/// The per-walk filler runs, memoized for every instruction of a run so a long
/// run is walked once rather than once per candidate inside it.
pub(super) struct PaddingRuns {
    wide: bool,
    ends: HashMap<u64, RunEnd>,
}

impl PaddingRuns {
    pub(super) fn new(decoder: &GapDecoder) -> Self {
        PaddingRuns { wide: decoder.code_space.get_addr_size() > 4, ends: HashMap::new() }
    }

    /// Classify the gap candidate at `entry` with bounded lookahead.
    pub(super) fn classify(&mut self, listing: &Listing, decoder: &mut GapDecoder, entry: u64, section_slots: bool) -> Filler {
        let Some(first) = decoder.probe(entry) else { return Filler::No };
        if is_zero_fill(&first) {
            return Filler::Refuse;
        }
        if !is_padding(&first, self.wide) {
            return Filler::No;
        }
        let mut run = Vec::new();
        let mut at = entry;
        let mut end = loop {
            if let Some(&end) = self.ends.get(&at) {
                break end;
            }
            if listing.function_at(at).is_some() {
                break RunEnd::Entry;
            }
            // With a declared grid we need the actual destination. Once two
            // validation windows are all padding, no start in the first window
            // can reach it. Leave the second window open for shorter suffixes.
            if section_slots && run.len() == 2 * super::MAX_FOLLOW_INSNS {
                run.truncate(super::MAX_FOLLOW_INSNS);
                break RunEnd::BeyondValidationLimit;
            }
            if (!section_slots && run.len() == super::MAX_FOLLOW_INSNS) || !listing.is_undefined(at) {
                break RunEnd::Other { blocked: false };
            }
            match decoder.probe(at) {
                Some(insn) if is_padding(&insn, self.wide) => {
                    let next = at.saturating_add(u64::from(insn.len));
                    let terminal = !decoder.frame_probe && (insn.is_terminal || insn.kind == super::FlowKind::ComputedJump);
                    let straight = insn.flows.is_empty() && insn.fall_through == Some(next);
                    run.push((at, terminal, straight));
                    at = next;
                }
                Some(insn) if !is_zero_fill(&insn) => break RunEnd::Code { at, min_visits: 0 },
                _ => break RunEnd::Other { blocked: false },
            }
        };
        // Only a straight fallthrough must traverse the next suffix. An alternate
        // flow may skip the rest, and a terminal cannot reach the destination.
        for (start, terminal, straight) in run.into_iter().rev() {
            if let RunEnd::Code { min_visits, .. } = &mut end {
                *min_visits = if terminal {
                    u32::MAX
                } else if straight {
                    min_visits.saturating_add(1)
                } else {
                    1
                };
            } else if let RunEnd::Other { blocked } = &mut end {
                // Even without a code destination (e.g. zero fill), this prefix
                // reaches only padding when its sole path ends at a terminal.
                *blocked = terminal || (straight && *blocked);
            }
            self.ends.insert(start, end);
        }
        match end {
            RunEnd::Entry | RunEnd::BeyondValidationLimit => Filler::Refuse,
            // The destination itself requires one more visit.
            RunEnd::Code { min_visits, .. } if min_visits as usize >= super::MAX_FOLLOW_INSNS => Filler::Blocked,
            RunEnd::Code { at, .. } => Filler::Before(at),
            RunEnd::Other { blocked: true } => Filler::Blocked,
            RunEnd::Other { blocked: false } => Filler::No,
        }
    }
}

/// Whether `insn` is an all-zero encoding.
pub(super) fn is_zero_fill(insn: &ProbedInsn) -> bool {
    is_zero_fill_text(&insn.mnemonic, &insn.operands)
}

/// [`is_zero_fill`] over an instruction's disassembly text.
pub(crate) fn is_zero_fill_text(mnemonic: &str, operands: &str) -> bool {
    let operands_are = |want: &str| {
        operands.chars().filter(|c| !c.is_whitespace()).map(|c| c.to_ascii_lowercase()).eq(want.chars())
    };
    if mnemonic.eq_ignore_ascii_case("add") {
        operands_are("byteptr[eax],al") || operands_are("byteptr[rax],al")
    } else if mnemonic.eq_ignore_ascii_case("andeq") {
        operands_are("r0,r0,r0")
    } else {
        mnemonic.eq_ignore_ascii_case("movs") && operands_are("r0,r0")
    }
}

/// Whether `insn` is alignment padding: `nop` of any width, `int3`, a `lea` of a
/// register into itself, or a register self-move a function cannot open with.
pub(super) fn is_padding(insn: &ProbedInsn, wide: bool) -> bool {
    is_padding_text(&insn.mnemonic, &insn.operands, wide)
}

/// [`is_padding`] over an instruction's disassembly text.
pub(crate) fn is_padding_text(mnemonic: &str, operands: &str, wide: bool) -> bool {
    if ["nop", "nop.w", "int3"].iter().any(|m| mnemonic.eq_ignore_ascii_case(m)) {
        return true;
    }
    let moves = ["mov", "cpy", "xchg"].iter().any(|m| mnemonic.eq_ignore_ascii_case(m));
    if !(moves || mnemonic.eq_ignore_ascii_case("lea")) {
        return false;
    }
    let Some((dst, src)) = operands.split_once(',') else { return false };
    let dst = dst.trim();
    if moves {
        return !wide && dst.eq_ignore_ascii_case(src.trim()) && !dst.eq_ignore_ascii_case("edi");
    }
    let Some(inner) = src.trim().strip_prefix('[').and_then(|s| s.strip_suffix(']')) else { return false };
    let base = match inner.split_once('+') {
        Some((base, disp)) if disp.trim() == "0x0" => base,
        Some(_) => return false,
        None => inner,
    };
    let full_width = !wide || [
        "RAX", "RBX", "RCX", "RDX", "RSI", "RDI", "RBP", "RSP",
        "R8", "R9", "R10", "R11", "R12", "R13", "R14", "R15",
    ].iter().any(|register| dst.eq_ignore_ascii_case(register));
    full_width && base.trim().eq_ignore_ascii_case(dst)
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use super::*;
    use crate::listing::FlowKind;

    fn insn(mnemonic: &str, operands: &str) -> ProbedInsn {
        ProbedInsn {
            len: 1, encoded_len: 1, kind: FlowKind::Fallthrough, is_call: false, is_terminal: false, fall_through: None,
            flows: Vec::new(), mnemonic: Rc::from(mnemonic), operands: operands.to_string(), reusable: None,
        }
    }

    #[test]
    fn zero_fill_forms() {
        assert!(is_zero_fill(&insn("ADD", "byte ptr [EAX],AL")));
        assert!(is_zero_fill(&insn("ADD", "byte ptr [RAX],AL")));
        assert!(is_zero_fill(&insn("andeq", "r0,r0,r0")));
        assert!(is_zero_fill(&insn("movs", "r0,r0")));
        assert!(!is_zero_fill(&insn("movs", "r6,r6")));
        assert!(!is_zero_fill(&insn("ADD", "byte ptr [EAX + 0x1],AL")));
        assert!(!is_zero_fill(&insn("ADD", "dword ptr [EAX],EAX")));
    }

    #[test]
    fn padding_forms() {
        for wide in [false, true] {
            assert!(is_padding(&insn("NOP", ""), wide));
            assert!(is_padding(&insn("NOP", "word ptr CS:[EAX + EAX*0x1]"), wide));
            assert!(is_padding(&insn("INT3", ""), wide));
            assert!(is_padding(&insn("nop.w", ""), wide));
            assert_eq!(is_padding(&insn("LEA", "ESI,[ESI]"), wide), !wide);
            assert_eq!(is_padding(&insn("LEA", "ESI,[ESI + 0x0]"), wide), !wide);
            assert!(!is_padding(&insn("LEA", "ESI,[ESI + 0x8]"), wide));
            assert!(!is_padding(&insn("LEA", "ECX,[ESP + 0x4]"), wide));
            assert!(!is_padding(&insn("MOV", "EBP,ESP"), wide));
            assert!(!is_padding(&insn("MOV", "EDI,EDI"), wide));
            assert!(!is_padding(&insn("PUSH", "EBP"), wide));
            assert!(!is_padding(&insn("movs", "r0,r0"), wide));
        }
        assert!(is_padding(&insn("MOV", "ESI,ESI"), false));
        assert!(is_padding(&insn("XCHG", "AX,AX"), false));
        assert!(is_padding(&insn("mov", "r8,r8"), false));
        assert!(!is_padding(&insn("MOV", "ESI,ESI"), true));
        assert!(!is_padding(&insn("XCHG", "EAX,EAX"), true));
        assert!(!is_padding(&insn("LEA", "ESI,[RSI + 0x0]"), true));
    }

    #[test]
    fn wide_self_lea_preserves_the_whole_register() {
        for register in [
            "RAX", "RBX", "RCX", "RDX", "RSI", "RDI", "RBP", "RSP",
            "R8", "R9", "R10", "R11", "R12", "R13", "R14", "R15",
        ] {
            for displacement in ["", " + 0x0"] {
                let operands = format!("{register},[{register}{displacement}]");
                assert!(is_padding(&insn("LEA", &operands), true), "{operands}");
                assert!(is_padding(&insn("lea", &operands.to_ascii_lowercase()), true));
            }
        }
        for register in [
            "EAX", "EBX", "ECX", "EDX", "ESI", "EDI", "EBP", "ESP",
            "R8D", "R9D", "R10D", "R11D", "R12D", "R13D", "R14D", "R15D",
        ] {
            for displacement in ["", " + 0x0"] {
                let operands = format!("{register},[{register}{displacement}]");
                assert!(!is_padding(&insn("LEA", &operands), true), "{operands}");
                assert!(!is_padding(&insn("lea", &operands.to_ascii_lowercase()), true));
            }
        }
    }

}

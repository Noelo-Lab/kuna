//! (kuna) `aifpoolslide` — no AIF gap entry starts on a literal-pool word; an
//! accept that opens with one is planted on the code after the pool (P1 code/data
//! partition).
//!
//! # The defect (GH-299)
//!
//! ARM code keeps its constants in literal pools right after the code that loads
//! them, so a pool usually sits between a function's last instruction and the next
//! function. The Listing walk never decodes a pool, so it is part of the hole the
//! AIF gap walk probes, and a pool word decodes as an instruction: an A32 word
//! holding an image address `0x608xxxxx` reads as `addvs ...`, the high half of a
//! Thumb SRAM pointer `0x2000xxxx` as `movs r0,#0`. Followed by the real
//! function's prologue, that pair is a fingerprint the discovered functions share
//! as soon as a few phantoms like it exist, and the speculative body runs straight
//! into the real function and validates. The entry lands one word (or half a word)
//! before the function, and since an accept consumes its body, the real start is
//! never probed: on a stripped A32 u-boot, 334 real functions were hidden that way.
//!
//! # The rule
//!
//! A word that some instruction loads PC-relatively is data. [`LiteralWords`]
//! collects those words from the decoded Listing (the absolute `[0x...]` operand
//! the ARM SLEIGH prints for a resolved `ldr`, and the `[pc,#imm]` form of
//! `vldr`/`ldrd`, both words of a 64-bit one)
//! and, as the walk moves up the image, from each body it accepts or consumes, so
//! the pool behind a gap-found function is known when the walk reaches it.
//!
//! A candidate inside a run of such words is still probed, but an accept is
//! planted at the end of the run, the first address after the pool, when its body
//! decoded an instruction there; otherwise the candidate is rejected and the walk
//! probes the code after the pool on its own. The entry therefore moves onto the
//! function the phantom was decoding instead of disappearing with it.
//!
//! Only those two literal forms count as a reference, and only on 32-bit ARM, so
//! no other architecture is affected.
//!
//! # Measured
//!
//! Default mode, each stripped image scored against its unstripped twin's symbols:
//!
//! ```text
//! 93 decbench ARM ELFs (no u-boot)  mid-body 4,375 -> 1,868  true starts +223, lost 0
//! 2 u-boot A32 images               mid-body 1,415 -> 1,081  true starts +334, lost 0
//! CMSIS_DAP.axf, i386 PEs, x86-64   unchanged
//! ```

use std::collections::BTreeSet;

use crate::listing::Listing;

use super::kuna_poolentry::{literal_target, wide, DecodeMode};
use super::GapDecoder;

/// Every word an instruction seen so far loads as a PC-relative literal.
pub(super) struct LiteralWords<'a> {
    mode: DecodeMode<'a>,
    words: BTreeSet<u64>,
}

impl<'a> LiteralWords<'a> {
    /// The words the decoded Listing loads.
    pub(super) fn from_listing(listing: &Listing, mode: DecodeMode<'a>) -> Self {
        let mut out = LiteralWords { mode, words: BTreeSet::new() };
        for (&at, insn) in listing.instructions() {
            out.note(listing, at, &insn.mnemonic, &insn.operands);
        }
        out
    }

    /// Add the words the speculative body `body` loads.
    pub(super) fn add_body(&mut self, listing: &Listing, decoder: &GapDecoder, body: &BTreeSet<u64>) {
        for &at in body {
            let insn = decoder.recorded(at);
            self.note(listing, at, &insn.mnemonic, &insn.operands);
        }
    }

    fn note(&mut self, listing: &Listing, at: u64, mnemonic: &str, operands: &str) {
        let Some(target) = literal_target(at, operands, &self.mode) else { return };
        let count = if wide(mnemonic) { 2 } else { 1 };
        for word in (0..count).map(|k| target + 4 * k) {
            let in_exec = listing.exec_ranges().iter().any(|&(lo, hi)| lo <= word && word < hi);
            if word % 4 == 0 && in_exec && !listing.is_instruction_start(word) {
                self.words.insert(word);
            }
        }
    }

    /// The first address after the run of adjacent literal words that holds `at`.
    pub(super) fn run_end(&self, at: u64) -> Option<u64> {
        run_end(&self.words, at)
    }
}

fn run_end(words: &BTreeSet<u64>, at: u64) -> Option<u64> {
    let &word = words.range(..=at).next_back()?;
    if at >= word + 4 {
        return None;
    }
    let mut end = word + 4;
    while words.contains(&end) {
        end += 4;
    }
    Some(end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_run_ends_after_its_last_adjacent_word() {
        let words: BTreeSet<u64> = [0x1000, 0x1004, 0x100c].into_iter().collect();
        assert_eq!(run_end(&words, 0x1000), Some(0x1008));
        assert_eq!(run_end(&words, 0x1002), Some(0x1008));
        assert_eq!(run_end(&words, 0x1006), Some(0x1008));
        assert_eq!(run_end(&words, 0x1008), None);
        assert_eq!(run_end(&words, 0x100e), Some(0x1010));
        assert_eq!(run_end(&words, 0x0ffe), None);
    }
}

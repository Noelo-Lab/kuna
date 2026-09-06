//! Which words inside a code listing are a **literal pool**, not instructions.
//!
//! On a fixed-width RISC target a constant that will not fit an immediate is
//! parked in `.text` next to the code that uses it and loaded PC-relatively.
//! `main` in the `1337ARM` crackme ends
//!
//! ```text
//!   0x8440  10309fe5  ldr r3,[0x8458]
//!   ...
//!   0x8454  10a89de8  ldmia sp,{r4,r11,sp,pc}
//!   0x8458  39050000  andeq r0,r0,r9, lsr r5
//! ```
//!
//! — and that last row is a lie. `0x8458` is the constant `0x539` (1337), the
//! answer the program compares against; nothing executes it, and an RE agent
//! reading the listing for the success value is handed a bitwise-and instead.
//! Every disassembler an agent would otherwise fall back to renders it as a
//! data word.
//!
//! ## The evidence is inside the listing
//!
//! A pool word is not guessed at here, it is **proved by the range's own
//! instructions**: some instruction in the listed range spells the address out
//! and reads it ([`ConsoleProgram::fixed_refs_at`](kuna_console::engine::ConsoleProgram::fixed_refs_at)),
//! and no instruction in the range names it as a branch target. That makes the
//! rule self-limiting in a way a caller can predict and steer: listing
//! `0x8458-0x845c` on its own contains no such load, so the word decodes as it
//! always did, and a caller who wants the raw decode of a pool word has that as
//! the escape hatch.
//!
//! ## What it refuses
//!
//! Each refusal is a way this could otherwise fabricate data where there is
//! code:
//!
//! * a target in a **writable** section — a GOT slot is read by address too, and
//!   a writable `.text` is a packer, whose "pool word" is very likely code — or
//!   one with a function symbol installed on it, which is code by declaration;
//! * an **unaligned** target, or a width that is not 1/2/4/8 — a real pool word
//!   is a naturally-aligned scalar. The width is the width of the ACCESS, which
//!   a `LOAD`'s address varnode does not carry (it is pointer-sized whatever it
//!   reads), so `ldrh r0,[0x1003c]` folds nothing: two bytes do not tile the
//!   four-byte row the slot decoded as;
//! * a target that any instruction in the range **branches to**, which is a
//!   label whatever else reads it;
//! * a target that does not fall on a decoded instruction boundary, or whose
//!   width does not tile a whole number of decoded rows. Folding only on an
//!   exact tiling is what keeps the rest of the listing byte-for-byte where it
//!   was: no address after a folded word can shift, so a false positive costs
//!   one mis-rendered row and never a re-aligned listing.
//! * the range's own **first** address, which is what the caller asked to see
//!   instructions at.

use std::collections::{BTreeMap, BTreeSet};

/// A decoded row's extent, in listing order: `(address, size)`.
pub(crate) type Boundary = (u64, u64);

/// The widths a literal pool word is spelled in.
fn width_is_scalar(width: u32) -> bool {
    matches!(width, 1 | 2 | 4 | 8)
}

/// The pool words a listing's own evidence proves, as `address -> width`.
///
/// `rows` are the decoded extents in address order, `reads` every
/// `(address, width)` the range's instructions read at a fixed address,
/// `flow_targets` every address they branch or call to, `span` the listed
/// `[lo, hi)`, and `is_pool_slot` whether the program allows a pool word to live
/// at an address at all.
pub(crate) fn pool_words(
    rows: &[Boundary],
    reads: &[(u64, u32)],
    flow_targets: &[u64],
    span: (u64, u64),
    is_pool_slot: &dyn Fn(u64) -> bool,
) -> BTreeMap<u64, u64> {
    let labels: BTreeSet<u64> = flow_targets.iter().copied().collect();
    let starts: BTreeMap<u64, usize> =
        rows.iter().enumerate().map(|(i, (addr, _))| (*addr, i)).collect();
    let mut out = BTreeMap::new();
    for &(addr, width) in reads {
        if !width_is_scalar(width) {
            continue;
        }
        let width = u64::from(width);
        if addr % width != 0 {
            continue;
        }
        if addr <= span.0 || addr.saturating_add(width) > span.1 {
            continue;
        }
        if labels.contains(&addr) || !is_pool_slot(addr) {
            continue;
        }
        if !tiles_whole_rows(rows, &starts, &labels, addr, width) {
            continue;
        }
        out.insert(addr, width);
    }
    out
}

/// Do the decoded rows starting at `addr` cover exactly `width` bytes, with no
/// branch target among the rows after the first?
fn tiles_whole_rows(
    rows: &[Boundary],
    starts: &BTreeMap<u64, usize>,
    labels: &BTreeSet<u64>,
    addr: u64,
    width: u64,
) -> bool {
    let Some(&first) = starts.get(&addr) else {
        return false;
    };
    let mut covered = 0u64;
    for (i, &(at, size)) in rows.iter().enumerate().skip(first) {
        if i > first && labels.contains(&at) {
            return false;
        }
        covered += size;
        if covered >= width {
            break;
        }
    }
    covered == width
}

/// The GAS directive a `width`-byte data word is spelled with.
pub(crate) fn word_mnemonic(width: u64) -> &'static str {
    match width {
        1 => ".byte",
        2 => ".short",
        8 => ".quad",
        _ => ".word",
    }
}

/// The value `bytes` holds, zero-padded to its own width — the spelling a pool
/// word is read for (`0x00000539`, not `0x539`).
pub(crate) fn word_operand(bytes: &[u8], big_endian: bool) -> String {
    let mut value: u64 = 0;
    if big_endian {
        for &b in bytes {
            value = (value << 8) | u64::from(b);
        }
    } else {
        for &b in bytes.iter().rev() {
            value = (value << 8) | u64::from(b);
        }
    }
    format!("0x{:0width$x}", value, width = bytes.len() * 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALWAYS: &dyn Fn(u64) -> bool = &|_| true;
    const NEVER: &dyn Fn(u64) -> bool = &|_| false;

    /// Four 4-byte rows at 0x100..0x110.
    fn rows() -> Vec<Boundary> {
        vec![(0x100, 4), (0x104, 4), (0x108, 4), (0x10c, 4)]
    }

    /// The shape the need is about: an instruction in the range reads a word
    /// that lies later in the same range, so that word is data.
    #[test]
    fn a_word_the_range_reads_at_a_fixed_address_is_a_pool_word() {
        let got = pool_words(&rows(), &[(0x108, 4)], &[], (0x100, 0x110), ALWAYS);
        assert_eq!(got, BTreeMap::from([(0x108, 4)]));
    }

    /// A branch target is a label whatever else reads it.
    #[test]
    fn a_branch_target_is_never_folded() {
        let got = pool_words(&rows(), &[(0x108, 4)], &[0x108], (0x100, 0x110), ALWAYS);
        assert!(got.is_empty());
    }

    /// A writable slot holds whatever the loader last wrote, and an address a
    /// function symbol sits on is code by declaration; neither can hold a pool.
    #[test]
    fn a_slot_the_program_refuses_is_never_folded() {
        let got = pool_words(&rows(), &[(0x108, 4)], &[], (0x100, 0x110), NEVER);
        assert!(got.is_empty());
    }

    /// Outside the listed span there is no evidence and no row to fold.
    #[test]
    fn a_target_outside_the_span_is_never_folded() {
        let outside = pool_words(&rows(), &[(0x200, 4)], &[], (0x100, 0x110), ALWAYS);
        assert!(outside.is_empty());
        let first = pool_words(&rows(), &[(0x100, 4)], &[], (0x100, 0x110), ALWAYS);
        assert!(first.is_empty(), "the address the caller asked to disassemble stays code");
        let past_end = pool_words(&rows(), &[(0x10c, 8)], &[], (0x100, 0x110), ALWAYS);
        assert!(past_end.is_empty());
    }

    /// A real pool word is a naturally-aligned scalar.
    #[test]
    fn an_unaligned_or_odd_width_target_is_never_folded() {
        assert!(pool_words(&rows(), &[(0x106, 4)], &[], (0x100, 0x110), ALWAYS).is_empty());
        assert!(pool_words(&rows(), &[(0x108, 3)], &[], (0x100, 0x110), ALWAYS).is_empty());
    }

    /// Folding only on an exact tiling is what stops the listing re-aligning:
    /// a 4-byte read landing mid-instruction, or covering part of the next one,
    /// is declined.
    #[test]
    fn a_read_that_does_not_tile_whole_rows_is_declined() {
        let mixed = vec![(0x100, 4), (0x104, 2), (0x106, 4), (0x10a, 2), (0x10c, 4)];
        assert!(
            pool_words(&mixed, &[(0x104, 4)], &[], (0x100, 0x110), ALWAYS).is_empty(),
            "0x104 is 2 bytes and 0x106 is 4, so nothing tiles 4 bytes"
        );
        // Two Thumb-sized rows DO tile a 4-byte word.
        let thumb = vec![(0x100, 2), (0x102, 2), (0x104, 2), (0x106, 2)];
        assert_eq!(
            pool_words(&thumb, &[(0x104, 4)], &[], (0x100, 0x108), ALWAYS),
            BTreeMap::from([(0x104, 4)])
        );
    }

    #[test]
    fn a_word_reads_in_the_programs_own_byte_order_padded_to_its_width() {
        assert_eq!(word_operand(&[0x39, 0x05, 0x00, 0x00], false), "0x00000539");
        assert_eq!(word_operand(&[0x00, 0x00, 0x05, 0x39], true), "0x00000539");
        assert_eq!(word_mnemonic(4), ".word");
        assert_eq!(word_mnemonic(2), ".short");
        assert_eq!(word_mnemonic(8), ".quad");
    }
}

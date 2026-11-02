//! (kuna) `option mipsunaligned`: lift a MIPS unaligned load or store pair as
//! the one whole access it makes.
//!
//! MIPS reads and writes a word at an address that may not be aligned with
//! two instructions, `lwl $3,0($2); lwr $3,3($2)` (`ldl`/`ldr`, `swl`/`swr`
//! and `sdl`/`sdr` likewise). Lifted one at a time each half merges memory
//! into the register by a shift its address's low bits pick, so the pair
//! reads the register it overwrites -- a call's dead second return word, an
//! argument -- and prints as a tangle of shifts and masks around that value.
//! The processor spec has whole-access forms for both halves, selected by the
//! `PAIR_INSTRUCTION_FLAG` context bit: the half at the lower address on a
//! big-endian target (the higher on a little-endian one) loads or stores the
//! whole word, and the other does nothing. Ghidra's `MipsPreAnalyzer` sets the
//! bit; this does the same, once per image before the first instruction is
//! decoded ([`paint`]).
//!
//! The scan reads the image's executable ranges. A half starts a pair when its
//! partner -- the other half of the same width, with the same base and data
//! registers, at the offset three (seven) bytes away on the side the target's
//! byte order puts it -- follows within five instructions, and on the way
//! nothing writes the base register, nothing reads or writes the data register
//! of a load or writes that of a store, and no control transfer comes before
//! the partner except one whose delay slot holds it (not a branch-likely,
//! whose delay slot runs only when taken). The first half may not sit in a
//! delay slot itself, a load may not use its data register as its base, and
//! no direct branch or jump in the image (an Octeon `bbit` among them) and no
//! function symbol may land after the first half and at or before the second,
//! since a path entering there would run one half alone. MIPS16, microMIPS
//! and release 6 code is left alone. The whole-access form of a left half
//! sets the bit at the instruction after it too, so a pair whose left half is
//! followed by a half that pairs with nothing is dropped.

use std::collections::BTreeSet;
use std::rc::Rc;

use kuna_base::address::Address;

use crate::architecture::Architecture;

const PAIR_FLAG: &[u8] = b"PAIR_INSTRUCTION_FLAG";

/// How many instructions after a half its partner may come.
const REACH: usize = 5;

/// How many bytes of code one read of the image takes.
const CHUNK: u64 = 1 << 20;

/// One half of an unaligned access.
#[derive(Clone, Copy, PartialEq, Eq)]
struct Half {
    load: bool,
    double: bool,
    left: bool,
    base: u32,
    data: u32,
    offset: i64,
}

impl Half {
    fn of(w: u32) -> Option<Half> {
        let (load, double, left) = match w >> 26 {
            0x22 => (true, false, true),
            0x26 => (true, false, false),
            0x1a => (true, true, true),
            0x1b => (true, true, false),
            0x2a => (false, false, true),
            0x2e => (false, false, false),
            0x2c => (false, true, true),
            0x2d => (false, true, false),
            _ => return None,
        };
        Some(Half { load, double, left, base: (w >> 21) & 0x1f, data: (w >> 16) & 0x1f, offset: w as u16 as i16 as i64 })
    }

    /// Is `other` this half's partner on a target of byte order `big`?
    fn pairs_with(&self, other: &Half, big: bool) -> bool {
        if self.load != other.load || self.double != other.double || self.left == other.left {
            return false;
        }
        if self.base != other.base || self.data != other.data {
            return false;
        }
        let (left, right) = if self.left { (self, other) } else { (other, self) };
        let span = if self.double { 7 } else { 3 };
        if big {
            left.offset + span == right.offset
        } else {
            right.offset + span == left.offset
        }
    }
}

/// How an instruction moves control.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Control {
    None,
    /// A branch or jump with a delay slot that always runs, and its target
    /// when it names one.
    Delayed(Option<u64>),
    /// A branch-likely: its delay slot runs only when it is taken.
    Likely(u64),
    /// A trap, a system call or an exception return.
    Stop,
}

fn control(w: u32, at: u64, octeon: bool) -> Control {
    let (op, rs, rt, funct) = (w >> 26, (w >> 21) & 0x1f, (w >> 16) & 0x1f, w & 0x3f);
    let branch = at.wrapping_add(4).wrapping_add(((w as u16 as i16 as i64) << 2) as u64);
    let jump = (at.wrapping_add(4) & !0x0fff_ffff) | (((w & 0x03ff_ffff) as u64) << 2);
    match op {
        0 => match funct {
            0x08 | 0x09 => Control::Delayed(None),
            0x0c | 0x0d | 0x30..=0x36 => Control::Stop,
            _ => Control::None,
        },
        1 => match rt {
            0 | 1 | 16 | 17 => Control::Delayed(Some(branch)),
            2 | 3 | 18 | 19 => Control::Likely(branch),
            8..=14 => Control::Stop,
            _ => Control::None,
        },
        2 | 3 | 0x1d => Control::Delayed(Some(jump)),
        4..=7 => Control::Delayed(Some(branch)),
        0x14..=0x17 => Control::Likely(branch),
        0x10..=0x13 if rs == 8 => {
            if (w >> 17) & 1 == 1 {
                Control::Likely(branch)
            } else {
                Control::Delayed(Some(branch))
            }
        }
        0x10 if rs == 0x10 => Control::Stop,
        0x32 | 0x36 | 0x3a | 0x3e if octeon => Control::Delayed(Some(branch)),
        _ => Control::None,
    }
}

/// The general registers an instruction may write, as a bit set; every one
/// for an encoding the scan does not know.
fn writes(w: u32) -> u32 {
    let (op, rs, rt, rd, funct) = (w >> 26, (w >> 21) & 0x1f, (w >> 16) & 0x1f, (w >> 11) & 0x1f, w & 0x3f);
    let bits = match op {
        0 => match funct {
            0x08 | 0x0c | 0x0d | 0x0f | 0x11 | 0x13 | 0x18..=0x1f | 0x30..=0x36 => 0,
            _ => 1 << rd,
        },
        1 => match rt {
            16..=19 => 1 << 31,
            _ => 0,
        },
        2 | 4..=7 | 0x14..=0x17 => 0,
        3 | 0x1d => 1 << 31,
        0x08..=0x0f | 0x18 | 0x19 => 1 << rt,
        0x10..=0x13 => match rs {
            0..=3 => 1 << rt,
            _ => 0,
        },
        0x1c => match funct {
            0x02 | 0x20 | 0x21 | 0x24 | 0x25 => 1 << rd,
            0x00 | 0x01 | 0x04 | 0x05 => 0,
            _ => u32::MAX,
        },
        0x1f => match funct {
            0x00..=0x07 | 0x3b => 1 << rt,
            0x20 | 0x24 => 1 << rd,
            _ => u32::MAX,
        },
        0x1a | 0x1b | 0x20..=0x27 | 0x30 | 0x34 | 0x37 | 0x38 | 0x3c => 1 << rt,
        0x28..=0x2f | 0x31..=0x33 | 0x35 | 0x36 | 0x39..=0x3b | 0x3d..=0x3f => 0,
        _ => u32::MAX,
    };
    bits & !1
}

/// The general registers an instruction may read or write, as a bit set.
fn touches(w: u32) -> u32 {
    let reads = match w >> 26 {
        2 | 3 | 0x0f | 0x1d => 0,
        _ => (1u32 << ((w >> 21) & 0x1f)) | (1u32 << ((w >> 16) & 0x1f)),
    };
    (reads | writes(w)) & !1
}

/// Do the instructions in `between` leave `first`'s access whole: no write of
/// its base, and no read or write of a load's data register or write of a
/// store's?
fn undisturbed(first: &Half, between: &[u32]) -> bool {
    between.iter().all(|&w| {
        let data = if first.load { touches(w) } else { writes(w) };
        writes(w) & (1 << first.base) == 0 && data & (1 << first.data) == 0
    })
}

/// The executable words of `arch`'s image, as runs of `(address, words)`.
fn code_words(arch: &Architecture, big: bool) -> Vec<(u64, Vec<u32>)> {
    let Some(code) = arch.manage().get_default_code_space().cloned() else { return Vec::new() };
    let loader = arch.translate().loader_rc();
    let mut runs = Vec::new();
    for &(lo, hi) in arch.litpool_const.iter() {
        let lo = (lo + 3) & !3;
        if hi < lo + 3 {
            continue;
        }
        let mut at = lo;
        let mut words = Vec::new();
        while at.saturating_add(3) <= hi {
            let len = ((hi - at + 1) & !3).min(CHUNK);
            let mut buf = vec![0u8; len as usize];
            let Ok(mut l) = loader.try_borrow_mut() else { break };
            if l.load_fill(&mut buf, &Address::new(Rc::clone(&code), at)).is_err() {
                break;
            }
            drop(l);
            words.extend(buf.chunks_exact(4).map(|b| {
                let b = [b[0], b[1], b[2], b[3]];
                if big {
                    u32::from_be_bytes(b)
                } else {
                    u32::from_le_bytes(b)
                }
            }));
            at += len;
        }
        if !words.is_empty() {
            runs.push((lo, words));
        }
    }
    runs
}

/// Is the context variable `name` set at `at`?
fn context_set(arch: &Architecture, name: &[u8], at: &Address) -> bool {
    arch.with_context_db(|db| db.get_variable_value(name, at).is_ok_and(|v| v != 0))
}

/// The addresses of both halves of every unaligned pair in `arch`'s image.
fn find_pairs(arch: &Architecture, big: bool, double: bool, octeon: bool) -> Vec<u64> {
    let Some(code) = arch.manage().get_default_code_space().cloned() else { return Vec::new() };
    let runs = code_words(arch, big);
    let entries: Vec<u64> = arch.func_sizes.iter().map(|&(a, _)| a).collect();
    pairs_in(&runs, big, double, octeon, &entries, &|a| {
        let here = Address::new(Rc::clone(&code), a);
        context_set(arch, b"ISA_MODE", &here) || context_set(arch, b"REL6", &here)
    })
}

/// [`find_pairs`] over the code `runs`, leaving out a pair that a function
/// of `entries` starts inside or whose first half `other_isa` says decodes in
/// another instruction set.
fn pairs_in(
    runs: &[(u64, Vec<u32>)],
    big: bool,
    double: bool,
    octeon: bool,
    entries: &[u64],
    other_isa: &dyn Fn(u64) -> bool,
) -> Vec<u64> {
    let mut targets: BTreeSet<u64> = entries.iter().copied().collect();
    for (start, words) in runs {
        for (i, &w) in words.iter().enumerate() {
            match control(w, start + 4 * i as u64, octeon) {
                Control::Delayed(Some(t)) | Control::Likely(t) => {
                    targets.insert(t);
                }
                _ => {}
            }
        }
    }
    let mut pairs: Vec<(u64, u64)> = Vec::new();
    let mut lefts: BTreeSet<u64> = BTreeSet::new();
    let mut halves: BTreeSet<u64> = BTreeSet::new();
    for (start, words) in runs {
        let at = |i: usize| start + 4 * i as u64;
        let mut taken = BTreeSet::new();
        for (i, &w) in words.iter().enumerate() {
            let Some(first) = Half::of(w) else { continue };
            if taken.contains(&i) || (first.double && !double) || (first.load && first.base == first.data) {
                continue;
            }
            if i > 0 && control(words[i - 1], at(i - 1), octeon) != Control::None {
                continue;
            }
            let mut partner = None;
            for j in i + 1..(i + 1 + REACH).min(words.len()) {
                if Half::of(words[j]).is_some_and(|h| first.pairs_with(&h, big)) {
                    partner = Some(j);
                    break;
                }
                match control(words[j], at(j), octeon) {
                    Control::None => {}
                    Control::Delayed(_) => {
                        let slot = j + 1;
                        if slot < words.len()
                            && Half::of(words[slot]).is_some_and(|h| first.pairs_with(&h, big))
                            && undisturbed(&first, &words[i + 1..slot])
                        {
                            partner = Some(slot);
                        }
                        break;
                    }
                    Control::Likely(_) | Control::Stop => break,
                }
            }
            let Some(j) = partner else { continue };
            if taken.contains(&j) || !undisturbed(&first, &words[i + 1..j]) || targets.range(at(i) + 1..=at(j)).next().is_some() {
                continue;
            }
            if other_isa(at(i)) {
                continue;
            }
            taken.insert(i);
            taken.insert(j);
            pairs.push((at(i), at(j)));
            for (k, a) in [(i, at(i)), (j, at(j))] {
                halves.insert(a);
                if Half::of(words[k]).is_some_and(|h| h.left) {
                    lefts.insert(a);
                }
            }
        }
    }
    let word_at = |a: u64| {
        runs.iter()
            .find(|(s, w)| *s <= a && a < s + 4 * w.len() as u64)
            .map(|(s, w)| w[((a - s) / 4) as usize])
    };
    loop {
        let leaks: BTreeSet<u64> = lefts
            .iter()
            .filter(|&&a| !halves.contains(&(a + 4)) && word_at(a + 4).and_then(Half::of).is_some())
            .copied()
            .collect();
        if leaks.is_empty() {
            break;
        }
        pairs.retain(|&(a, b)| {
            let keep = !leaks.contains(&a) && !leaks.contains(&b);
            if !keep {
                for x in [a, b] {
                    halves.remove(&x);
                    lefts.remove(&x);
                }
            }
            keep
        });
    }
    halves.into_iter().collect()
}

/// Bring the image's unaligned pairs into line with `option mipsunaligned`:
/// set `PAIR_INSTRUCTION_FLAG` on both halves of every pair when it is on and
/// clear it again when it was turned off. Inert on a language without the
/// flag and on an image with no executable ranges (the XML datatests).
pub fn paint(arch: &mut Architecture) {
    if arch.mips_unaligned == arch.kuna_mips_pairs_painted {
        return;
    }
    if !arch.archid.starts_with("MIPS:") || arch.litpool_const.is_empty() {
        return;
    }
    if arch.with_context_db(|db| db.get_variable(PAIR_FLAG).is_err()) {
        return;
    }
    let Some(code) = arch.manage().get_default_code_space().cloned() else { return };
    if arch.kuna_mips_pairs.is_none() {
        let mut parts = arch.archid.split(':').skip(1);
        let big = parts.next() == Some("BE");
        let double = parts.next() == Some("64");
        let octeon = parts.next().is_some_and(|v| v.starts_with("octeon"));
        arch.kuna_mips_pairs = Some(Rc::new(find_pairs(arch, big, double, octeon)));
    }
    let on = arch.mips_unaligned;
    arch.kuna_mips_pairs_painted = on;
    let Some(pairs) = arch.kuna_mips_pairs.clone().filter(|p| !p.is_empty()) else { return };
    arch.with_context_db_mut(|db| {
        for &a in pairs.iter() {
            let begin = Address::new(Rc::clone(&code), a);
            let end = Address::new(Rc::clone(&code), a + 4);
            let _ = db.set_variable_region(PAIR_FLAG, &begin, &end, on as u32);
        }
    });
}

#[cfg(test)]
#[path = "kuna_mipsunaligned/tests.rs"]
mod tests;

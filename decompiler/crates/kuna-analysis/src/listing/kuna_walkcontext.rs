//! ARM modes belong to queued control-flow edges and successful instruction spans.
//! Decoder mode commits are evidence for successors, never global range writes.

use std::collections::BTreeMap;
use std::rc::Rc;

use kuna_base::{address::Address, space::AddrSpace};
use kuna_decomp::architecture::Architecture;
use kuna_sleigh::translate::{ContextCommitRecord, Translate};

pub(super) struct WalkContext<'a> {
    translate: &'a dyn Translate,
    word: usize,
    shift: u32,
    bits: u32,
    saved_writes: u32,
    original: Vec<(u64, u64, u32)>,
    calls: BTreeMap<u64, u32>,
    inventory: BTreeMap<u64, u32>,
}

impl<'a> WalkContext<'a> {
    pub fn new(
        enabled: bool,
        translate: &'a dyn Translate,
        arch: &Architecture,
        space: &Rc<AddrSpace>,
        exec: &[(u64, u64)],
    ) -> Option<Self> {
        if !enabled {
            return None;
        }
        let var = arch
            .with_context_db_mut(|db| db.get_variable(b"TMode"))
            .ok()?;
        let word = var.get_word() as usize;
        let shift = var.get_shift() as u32;
        let bits = var.get_mask() << shift;
        let read_override = translate.set_context_read_override(word, 0, 0);
        translate.set_context_read_override(word, read_override.0, read_override.1);
        let original = arch.with_context_db_mut(|db| {
            let mut runs: Vec<(u64, u64, u32)> = Vec::new();
            for &(start, end) in exec {
                let mut at = start;
                while at < end {
                    let (words, _, last) =
                        db.get_context_bounds(&Address::new(Rc::clone(space), at));
                    let next = last.saturating_add(1).min(end);
                    let effective =
                        (words[word] & !read_override.0) | (read_override.1 & read_override.0);
                    let value = (effective & bits) >> shift;
                    if let Some(last) = runs.last_mut().filter(|run| run.1 == at && run.2 == value)
                    {
                        last.1 = next;
                    } else {
                        runs.push((at, next, value));
                    }
                    at = next;
                }
            }
            runs
        });
        let saved_writes = translate.set_context_write_mask(word, u32::MAX);
        translate.set_context_write_mask(word, saved_writes & !bits);
        Some(Self {
            translate,
            word,
            shift,
            bits,
            saved_writes,
            original,
            calls: BTreeMap::new(),
            inventory: BTreeMap::new(),
        })
    }

    pub fn seed_inventory_modes(&mut self, seeds: impl Iterator<Item = u64>, modes: &[(u64, u64, u32)]) {
        if modes.is_empty() { return; }
        for at in seeds {
            let Some(index) = modes.partition_point(|&(start, _, _)| start <= at).checked_sub(1) else { continue };
            let (_, end, mode) = modes[index];
            if at < end { self.inventory.insert(at, mode); }
        }
    }

    pub fn entry(&self, at: u64) -> Option<u32> {
        self.calls.get(&at).or_else(|| self.inventory.get(&at)).copied().or_else(|| {
            let index = self
                .original
                .partition_point(|&(start, _, _)| start <= at)
                .checked_sub(1)?;
            let (_, end, mode) = self.original[index];
            (at < end).then_some(mode)
        })
    }

    pub fn successor(
        &self,
        at: u64,
        mode: Option<u32>,
        commits: &[ContextCommitRecord],
    ) -> Option<u32> {
        commits
            .iter()
            .rev()
            .find(|c| {
                c.addr.get_offset() == at
                    && c.word as usize == self.word
                    && c.mask & self.bits == self.bits
            })
            .map(|c| (c.value & self.bits) >> self.shift)
            .or(mode)
    }

    pub fn called(&mut self, at: u64, mode: Option<u32>) {
        if let Some(mode) = mode {
            self.calls.insert(at, mode);
        }
    }

    pub fn reset(&mut self) {
        self.calls.clear();
    }
}

impl Drop for WalkContext<'_> {
    fn drop(&mut self) {
        self.translate
            .set_context_write_mask(self.word, self.saved_writes);
    }
}

/// Select the effective read mode without changing database values or boundaries.
/// A failed decode publishes nothing; success publishes only its full byte span.
pub(super) struct InstructionMode<'a> {
    arch: &'a Architecture,
    translate: &'a dyn Translate,
    space: &'a Rc<AddrSpace>,
    at: u64,
    mode: u32,
    word: usize,
    bits: u32,
    value: u32,
    saved_read: (u32, u32),
}

impl<'a> InstructionMode<'a> {
    pub fn select(
        arch: &'a Architecture,
        translate: &'a dyn Translate,
        space: &'a Rc<AddrSpace>,
        at: u64,
        mode: Option<u32>,
    ) -> Option<Self> {
        let mode = mode?;
        let var = arch
            .with_context_db_mut(|db| db.get_variable(b"TMode"))
            .ok()?;
        let word = var.get_word() as usize;
        let bits = var.get_mask() << var.get_shift();
        let value = mode << var.get_shift();
        let saved_read = translate.set_context_read_override(word, 0, 0);
        translate.set_context_read_override(
            word,
            saved_read.0 | bits,
            (saved_read.1 & !bits) | value,
        );
        Some(Self {
            arch,
            translate,
            space,
            at,
            mode,
            word,
            bits,
            value,
            saved_read,
        })
    }

    pub fn context_word(&self, index: usize, value: u32) -> u32 {
        if index == self.word {
            (value & !self.bits) | self.value
        } else {
            value
        }
    }

    pub fn accept(&self, len: u32) {
        paint(
            self.arch,
            self.space,
            self.at,
            self.at.saturating_add(u64::from(len)),
            self.mode,
        );
    }
}

impl Drop for InstructionMode<'_> {
    fn drop(&mut self) {
        self.translate
            .set_context_read_override(self.word, self.saved_read.0, self.saved_read.1);
    }
}

fn paint(arch: &Architecture, space: &Rc<AddrSpace>, start: u64, end: u64, mode: u32) {
    arch.with_context_db_mut(|db| {
        let addr = Address::new(Rc::clone(space), start);
        let Ok(var) = db.get_variable(b"TMode") else {
            return;
        };
        let (words, _, last) = db.get_context_bounds(&addr);
        if var.get_value(words) != mode || last.saturating_add(1) < end {
            let _ =
                db.set_variable_region(b"TMode", &addr, &Address::new(Rc::clone(space), end), mode);
        }
    });
}

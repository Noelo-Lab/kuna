//! Globals the whole program never writes (option `opaquepredicate globals`).
//!
//! Bogus control flow keys its predicates on globals nothing assigns:
//! `if (y < 10 || x * (x + 1) % 2 == 0)` with `x` and `y` zero-initialized in
//! `.bss`. One function cannot know that; the analysis tier answers it for
//! the whole program (`kuna_analysis::listing::kuna_opaqueglobals`) with the
//! bytes of writable data that some instruction stores to, that a pointer
//! may reach, or that the run-time loader fills. Any other byte still holds
//! what the image gives it when the program reads it, so
//! `kuna_opaquedomain` evaluates a comparison over such a read with that
//! value.
//!
//! The scan decodes the whole program, so it runs when the first comparison
//! over a global asks: that question marks the function, the console scans
//! and decompiles it again, and every later function of the run reads the
//! answer.
//!
//! The answer rests on what a static scan can see. A store through a pointer
//! the scan does not follow back to the global, code the walk does not reach
//! and another module that names the global are all missed, which is why the
//! value is opt-in.

use std::rc::Rc;

use kuna_base::address::calc_mask;

use crate::context::VarnodeId;
use crate::funcdata::Funcdata;

/// The data of the program and the part of it something may write.
#[derive(Debug, Default)]
pub struct UnwrittenGlobals {
    /// `(lo, hi, loaded)` of each data section; `loaded` is false where the
    /// image holds no bytes and the program starts with zeroes.
    data: Vec<(u64, u64, bool)>,
    /// Sorted, disjoint `[lo, hi)` ranges that may be written.
    dirty: Vec<(u64, u64)>,
}

impl UnwrittenGlobals {
    pub fn new(mut data: Vec<(u64, u64, bool)>, mut dirty: Vec<(u64, u64)>) -> UnwrittenGlobals {
        data.sort_unstable();
        dirty.sort_unstable();
        let mut merged: Vec<(u64, u64)> = Vec::with_capacity(dirty.len());
        for (lo, hi) in dirty {
            match merged.last_mut() {
                Some(last) if lo <= last.1 => last.1 = last.1.max(hi),
                _ => merged.push((lo, hi)),
            }
        }
        UnwrittenGlobals { data, dirty: merged }
    }

    /// Whether the image holds the bytes of `[at, at + size)`, when the range
    /// lies in one data section and nothing may write it.
    fn initial(&self, at: u64, size: u64) -> Option<bool> {
        let end = at.checked_add(size)?;
        let &(_, _, loaded) = self.data.iter().find(|&&(lo, hi, _)| lo <= at && end <= hi)?;
        let next = self.dirty.partition_point(|&(_, hi)| hi <= at);
        self.dirty.get(next).is_none_or(|&(lo, _)| lo >= end).then_some(loaded)
    }
}

/// The value of `vn` when it reads a global no code writes.
pub(crate) fn constant_read(data: &Funcdata, vn: VarnodeId) -> Option<u64> {
    let glb = data.get_arch();
    if glb.opaque_globals.is_none() && !glb.opaque_globals_pending {
        return None;
    }
    let v = data.vbank().get(vn)?;
    let size = v.get_size();
    let space = glb.manage().get_default_data_space()?;
    if !(1..=8).contains(&size) || !Rc::ptr_eq(v.get_space(), space) {
        return None;
    }
    if let Some(def) = v.get_def() {
        if !data.obank().get(def)?.is_marker() {
            return None;
        }
    }
    let Some(found) = &glb.opaque_globals else {
        glb.opaque_globals_wanted.set(true);
        return None;
    };
    if !found.initial(v.get_offset(), size as u64)? {
        return Some(0);
    }
    let mut bytes = [0u8; 8];
    let width = size as usize;
    glb.loader_fill(&mut bytes[..width], v.get_addr()).ok()?;
    let value = if space.is_big_endian() {
        bytes[..width].iter().fold(0u64, |acc, &b| (acc << 8) | u64::from(b))
    } else {
        u64::from_le_bytes(bytes)
    };
    Some(value & calc_mask(size))
}

#[cfg(test)]
mod tests;

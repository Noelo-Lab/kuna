//! (kuna `floatglobals`) A global the whole program only moves through float
//! registers holds a float (P5).
//!
//! A global has one declaration for every function, so `float_input_vote`
//! refuses a float parameter stored into one unless something declares it a
//! float: `void fy(int *p, double b) { gd = b; *p = 1; }` printed
//! `fy(unsigned long a0, ..)`, and a caller that passes `gd` to a float
//! parameter printed `sink(((union { .. }){ .from = gd }).to)`.  What the one
//! function sees cannot settle it: another function may read `gd` as an
//! integer (`set_gi_bits` copies a float's bits into an `int` global that
//! `use_gi` computes with).
//!
//! The analysis tier answers it for the whole program
//! (`kuna_analysis::listing::kuna_floatglobals`): every load and store of a
//! global in every function's code, with its width and whether the value goes
//! to or comes from a floating-point register.  A global every access of which
//! is a float register move or a float operation of one width, that no other
//! access overlaps and whose address does not escape the walk, is
//! [`FloatGlobals`]' entry.  The votes ask
//! [`float_only`]; the answer is evidence about the global's declaration, so it
//! never overrides a declared type and never types a value on its own.
//!
//! The scan decodes the whole program, so it runs only when a function asks,
//! and a vote asks only when the global is the one objection left: the first
//! question of a run marks the function, the console scans and decompiles it
//! again, and every later function of the run reads the answer.

use std::collections::BTreeMap;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::types::int4;

use crate::funcdata::Funcdata;

/// The globals the program only moves through float registers, keyed by
/// address, with the width every access has.
pub type FloatGlobals = BTreeMap<u64, u8>;

/// What the scan reads: the loader's sections as `(vma, size, flags)` and
/// the entries of every function the program has.
#[derive(Debug, Clone, Default)]
pub struct FloatScan {
    pub sections: Vec<(u64, u64, u32)>,
    pub seeds: Vec<u64>,
}

/// The most code a run that decompiles one function scans: the scan decodes
/// all of it (about 0.65 s per MiB), so past this a single function refuses
/// every global as it did without the scan.
pub const SINGLE_FUNCTION_SCAN_BYTES: u64 = 256 * 1024;

impl FloatScan {
    /// The bytes of executable code the scan would decode.
    pub fn code_bytes(&self) -> u64 {
        self.sections
            .iter()
            .filter(|&&(_, _, flags)| flags & kuna_sleigh::loadimage::section_flags::CODE != 0)
            .map(|&(_, size, _)| size)
            .sum()
    }
}

/// Does the program only move the `size`-byte global at `addr` through float
/// registers?  `false` while the answer is not known yet, and the function is
/// marked so the console takes the scan and decides it again.
pub(crate) fn float_only(data: &Funcdata, addr: &Address, size: int4) -> bool {
    let glb = data.get_arch();
    if !matches!(size, 4 | 8) {
        return false;
    }
    let data_space = glb.manage().get_default_data_space();
    if !addr.get_space().zip(data_space).is_some_and(|(s, d)| Rc::ptr_eq(s, d)) {
        return false;
    }
    if !crate::kuna_floatreg::moved_as_a_float_here(data, addr, size) {
        return false;
    }
    match &glb.float_globals {
        Some(found) => found.get(&addr.get_offset()) == Some(&(size as u8)),
        None => {
            if glb.float_globals_pending {
                glb.float_globals_wanted.set(true);
            }
            false
        }
    }
}

//! (kuna `mzdgroup`) The functions of an MS-DOS executable run with DS holding
//! the program's data segment.
//!
//! A near data pointer in 16-bit real-mode code is an offset into the segment
//! DS holds. A function's DS is an input the decompiler knows nothing about, so
//! a global read `mov ax, [0x308e]` stayed `*(int *)0x308e`, a pointer constant
//! resolved against DS 0, and no global of the program had a name. The loader
//! reads the segment the startup code loads into DS (the linker's DGROUP,
//! `kuna_analysis::loader::kuna_mzdgroup`); [`apply`] states it as DS's tracked
//! value over the load module, so each function starts with DS set to it
//! (`ActionConstbase`) and a constant near pointer resolves against it. The
//! entry point itself starts with DS holding the PSP and keeps no tracked DS.
//! A function that loads another segment into DS is decompiled with that one
//! from the load on.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_sleigh::globalcontext::TrackedContext;

use crate::architecture::Architecture;

/// The data segment an MS-DOS executable's startup code loads into DS, and the
/// load module it holds over.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MzDataSegment {
    /// The segment value DS holds.
    pub ds: u16,
    /// The linear address of the first load-module byte.
    pub start: u64,
    /// The linear address one past the last load-module byte.
    pub end: u64,
    /// The linear address of the entry point.
    pub entry: u64,
}

/// Add or remove the tracked DS value over the load module, from the
/// architecture's `mzdgroup` option and the loader's [`MzDataSegment`]. Every
/// other tracked register keeps its value; a load without the segment is left
/// untouched.
pub fn apply(arch: &Architecture) {
    let Some(seg) = arch.mz_dgroup else { return };
    let Some(space) = arch.manage().get_default_code_space().cloned() else { return };
    let Ok(loc) = arch.get_register_varnode(b"DS") else { return };
    let on = arch.mz_dgroup_on;
    let at = |offset: u64| Address::new(Rc::clone(&space), offset);
    arch.with_context_db_mut(|db| {
        let mut points = vec![seg.start, seg.entry, seg.entry + 1, seg.end];
        points.extend(
            db.clone_trackbase()
                .iter()
                .map(|(addr, _)| addr)
                .filter(|addr| addr.get_space().is_some_and(|other| Rc::ptr_eq(other, &space)))
                .map(Address::get_offset)
                .filter(|&offset| offset > seg.start && offset < seg.end),
        );
        points.sort_unstable();
        points.dedup();
        for span in points.windows(2) {
            let mut set = db.get_tracked_set(&at(span[0])).clone();
            set.retain(|tracked| tracked.loc != loc);
            if on && span[0] != seg.entry {
                set.push(TrackedContext { loc: loc.clone(), val: u64::from(seg.ds) });
            }
            *db.create_set(&at(span[0]), &at(span[1])) = set;
        }
    });
}

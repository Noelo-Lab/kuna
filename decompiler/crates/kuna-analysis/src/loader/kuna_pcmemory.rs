//! (kuna #1288) The PC memory an MS-DOS program shares with the machine.
//!
//! Three regions of a PC's first megabyte change or are read behind a DOS
//! program's back: the interrupt vector table (0000:0000-03FF), which the CPU
//! reads on every interrupt and other resident programs rewrite; the BIOS data
//! area (0040:0000-00FF), whose timer tick at 0040:006C and keyboard buffer and
//! flags the BIOS interrupt handlers update; and video memory (A000:0000 up to
//! C000:0000), which the display adapter maps, and whose VGA planes a read or
//! a write goes through the adapter's latches. Read as ordinary memory, a loop
//! that polls the tick folds to `while (true) {}` and the first of two writes to
//! one VGA address is dropped as dead. [`paint_volatile`] marks the three
//! volatile.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::AddrSpace;
use kuna_decomp::architecture::Architecture;
use kuna_decomp::varnode::varnode_flags;

/// The `[start, end)` linear ranges of the interrupt vector table, the BIOS
/// data area and video memory.
pub const SHARED: [(u64, u64); 3] = [(0x0, 0x400), (0x400, 0x500), (0xa_0000, 0xc_0000)];

/// Mark each [`SHARED`] range no `mapped` `(vma, size, flags)` range overlaps
/// volatile in `space`, when `arch` is the x86 real-mode language. Painted
/// before any symbol is mapped, so the symbols over it take the property.
pub fn paint_volatile(arch: &mut Architecture, space: &Rc<AddrSpace>, mapped: &[(u64, u64, u32)]) {
    if !arch.archid.starts_with("x86:LE:16:Real Mode") {
        return;
    }
    for &(start, end) in &SHARED {
        let overlaps = mapped
            .iter()
            .any(|&(vma, size, _)| vma < end && start < vma.saturating_add(size));
        if overlaps || end > space.get_highest().saturating_add(1) {
            continue;
        }
        let first = Address::new(Rc::clone(space), start);
        let last_open = Address::new(Rc::clone(space), end);
        arch.symboltab.set_property_range(varnode_flags::volatil, &first, &last_open);
    }
}

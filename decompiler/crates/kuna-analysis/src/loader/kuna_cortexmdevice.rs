//! The Device memory of an M-profile image.
//!
//! The ARMv7-M and ARMv8-M memory maps make two regions Device memory on every
//! part: the Peripheral region (0x40000000-0x5FFFFFFF), where a vendor places
//! its on-chip peripherals, and the System region (0xE0000000-0xFFFFFFFF), the
//! Private Peripheral Bus and the vendor system space. A read there may return
//! a new value each time and every write has an effect, so a status register
//! polled in a loop or a data register written twice is not ordinary memory.
//! [`paint_volatile`] marks both regions volatile, outside the ranges the image
//! maps itself.

use std::rc::Rc;

use kuna_base::address::{Address, Range};
use kuna_base::space::AddrSpace;
use kuna_decomp::architecture::Architecture;
use kuna_decomp::varnode::varnode_flags;

/// The `[start, end)` Peripheral and System regions.
pub const DEVICE: [(u64, u64); 2] = [(0x4000_0000, 0x6000_0000), (0xe000_0000, 0x1_0000_0000)];

/// Whether the image is M-profile: a Cortex-M language (`ARM:*:32:Cortex`,
/// `ARM:*:32:v8-m`), or an ARM ELF whose build attributes state the M profile.
pub fn is_m_profile(archid: &str, file: Option<&object::File<'_>>) -> bool {
    let mut parts = archid.split(':');
    let cortex = parts.next() == Some("ARM") && matches!(parts.nth(2), Some("Cortex" | "v8-m"));
    cortex || file.is_some_and(super::kuna_armfloatabi::m_profile)
}

/// The parts of [`DEVICE`] in `space` that no `mapped` `(vma, size, flags)`
/// range covers, as `[start, end)`.
fn unmapped(space_end: u64, mapped: &[(u64, u64, u32)]) -> Vec<(u64, u64)> {
    let mut taken: Vec<(u64, u64)> = mapped
        .iter()
        .filter(|m| m.1 > 0)
        .map(|&(vma, size, _)| (vma, vma.saturating_add(size)))
        .collect();
    taken.sort_unstable();
    let mut free = Vec::new();
    for &(start, end) in &DEVICE {
        let end = end.min(space_end);
        let mut at = start;
        for &(lo, hi) in &taken {
            if hi <= at || lo >= end {
                continue;
            }
            if lo > at {
                free.push((at, lo));
            }
            at = at.max(hi);
        }
        if at < end {
            free.push((at, end));
        }
    }
    free
}

/// Mark [`DEVICE`] volatile in `space`, except what `mapped` covers. Painted
/// before any data symbol is mapped, so a symbol over it takes the property.
pub fn paint_volatile(arch: &mut Architecture, space: &Rc<AddrSpace>, mapped: &[(u64, u64, u32)]) {
    let space_end = space.get_highest().saturating_add(1);
    for (start, end) in unmapped(space_end, mapped) {
        let last_open =
            Range::new(Rc::clone(space), start, end - 1).get_last_addr_open(arch.manage());
        arch.symboltab.set_property_range(
            varnode_flags::volatil,
            &Address::new(Rc::clone(space), start),
            &last_open,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_m_profile_languages_qualify_without_a_file() {
        assert!(is_m_profile("ARM:LE:32:Cortex:default", None));
        assert!(is_m_profile("ARM:BE:32:v8-m:default", None));
        assert!(!is_m_profile("ARM:LE:32:v8:default", None));
        assert!(!is_m_profile("AARCH64:LE:64:v8A:default", None));
    }

    #[test]
    fn mapped_ranges_are_cut_out_of_the_device_regions() {
        assert_eq!(unmapped(1 << 32, &[]), DEVICE.to_vec());
        let mapped = [
            (0x0800_0000, 0x1000, 5),
            (0x4000_0000, 0x1000, 6),
            (0xe010_0000, 0x10, 6),
        ];
        assert_eq!(
            unmapped(1 << 32, &mapped),
            [
                (0x4000_1000, 0x6000_0000),
                (0xe000_0000, 0xe010_0000),
                (0xe010_0010, 0x1_0000_0000)
            ]
        );
        assert_eq!(unmapped(0x5000_0000, &[]), [(0x4000_0000, 0x5000_0000)]);
    }
}

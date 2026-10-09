//! Code targets materialized by an adjacent SPARC HI22/LO10 sethi/jmpl pair.

use object::{elf, Relocation, RelocationFlags, RelocationTarget, SymbolIndex};
use std::collections::{HashMap, HashSet};

pub(super) fn code_targets(
    bytes: &[u8],
    relocations: &[(u64, Relocation)],
    little_endian: bool,
) -> HashSet<SymbolIndex> {
    let high: HashMap<u64, &Relocation> = relocations
        .iter()
        .filter(|(_, relocation)| {
            matches!(
                relocation.flags(),
                RelocationFlags::Elf {
                    r_type: elf::R_SPARC_HI22
                }
            )
        })
        .map(|(offset, relocation)| (*offset, relocation))
        .collect();
    let mut targets = HashSet::new();
    for (offset, low) in relocations {
        if !matches!(
            low.flags(),
            RelocationFlags::Elf {
                r_type: elf::R_SPARC_LO10
            }
        ) || low.addend() != 0
            || low.has_implicit_addend()
            || offset & 3 != 0
        {
            continue;
        }
        let Some(high) = offset
            .checked_sub(4)
            .and_then(|previous| high.get(&previous))
        else {
            continue;
        };
        let RelocationTarget::Symbol(symbol) = low.target() else {
            continue;
        };
        if high.target() != low.target() || high.addend() != 0 || high.has_implicit_addend() {
            continue;
        }
        let Some((sethi, jmpl)) = instruction_pair(bytes, *offset, little_endian) else {
            continue;
        };
        let register = (sethi >> 25) & 31;
        if sethi & 0xc1c0_0000 == 0x0100_0000
            && register != 0
            && jmpl & 0xc1f8_2000 == 0x81c0_2000
            && (jmpl >> 14) & 31 == register
            && matches!((jmpl >> 25) & 31, 0 | 15)
            && jmpl & 0x1c00 == 0
        {
            targets.insert(symbol);
        }
    }
    targets
}

fn instruction_pair(bytes: &[u8], offset: u64, little_endian: bool) -> Option<(u32, u32)> {
    let offset = usize::try_from(offset).ok()?;
    let pair = bytes.get(offset.checked_sub(4)?..offset.checked_add(4)?)?;
    let high = pair[..4].try_into().ok()?;
    let low = pair[4..].try_into().ok()?;
    Some(if little_endian {
        (u32::from_le_bytes(high), u32::from_le_bytes(low))
    } else {
        (u32::from_be_bytes(high), u32::from_be_bytes(low))
    })
}

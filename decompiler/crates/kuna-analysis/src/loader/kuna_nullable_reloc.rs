//! Preserve nullable symbol identity separately from a pre-link display address.
use super::kuna_mipsreloc;
use kuna_decomp::kuna_nullable_reloc::{NullableRelocation, Operand};
use object::{Object, ObjectSection, ObjectSymbol, RelocationTarget, SectionIndex, SymbolIndex};
use std::collections::HashMap;

pub(super) fn references(
    file: &object::File<'_>,
    vmas: &HashMap<SectionIndex, u64>,
    externs: &HashMap<SymbolIndex, u64>,
    slots: &mut HashMap<SymbolIndex, u64>,
    cursor: &mut u64,
) -> (Vec<NullableRelocation>, Vec<(SymbolIndex, u64)>) {
    let arch = file.architecture();
    let mips = matches!(
        arch,
        object::Architecture::Mips
            | object::Architecture::Mips64
            | object::Architecture::Mips64_N32
    );
    if !mips && arch != object::Architecture::X86_64 {
        return (Vec::new(), Vec::new());
    }
    let pointer_size = if file.is_64() { 8 } else { 4 };
    let mut out = Vec::new();
    let mut allocated = Vec::new();
    for section in file.sections() {
        let text = section.kind() == object::SectionKind::Text;
        if !text
            && !matches!(
                section.kind(),
                object::SectionKind::ReadOnlyData | object::SectionKind::ReadOnlyString
            )
        {
            continue;
        }
        let Some(&base) = vmas.get(&section.index()) else {
            continue;
        };
        let data = section.data().unwrap_or(&[]);
        let relocs: Vec<_> = section.relocations().collect();
        let lows = kuna_mipsreloc::lo_halves(file, &relocs, data, file.is_little_endian());
        for (i, (offset, reloc)) in relocs.iter().enumerate() {
            let RelocationTarget::Symbol(idx) = reloc.target() else {
                continue;
            };
            let Ok(symbol) = file.symbol_by_index(idx) else {
                continue;
            };
            if !symbol.is_undefined() || !symbol.is_weak() {
                continue;
            }
            let Some(&value) = externs.get(&idx) else {
                continue;
            };
            let Some(kind) = kuna_mipsreloc::r_type_of(reloc) else {
                continue;
            };
            let mut addend = reloc.addend();
            if !text {
                let pointer = if mips {
                    kind == if pointer_size == 8 { 18 } else { 2 }
                } else {
                    kind == 1
                };
                if !pointer {
                    continue;
                }
                if reloc.has_implicit_addend() {
                    let Some(bytes) =
                        data.get(*offset as usize..*offset as usize + pointer_size as usize)
                    else {
                        continue;
                    };
                    addend = if file.is_little_endian() {
                        bytes.iter().rev().fold(0u64, |a, &b| (a << 8) | b as u64)
                    } else {
                        bytes.iter().fold(0u64, |a, &b| (a << 8) | b as u64)
                    } as i64;
                }
                let slot = *slots.entry(idx).or_insert_with(|| {
                    *cursor = (*cursor + pointer_size as u64 - 1) & !(pointer_size as u64 - 1);
                    let slot = *cursor;
                    *cursor += pointer_size as u64;
                    allocated.push((idx, slot));
                    slot
                });
                out.push(NullableRelocation {
                    field: base + offset,
                    slot,
                    symbol: value,
                    addend,
                    pointer_size,
                    operand: Operand::DataPointer,
                    shift: 0,
                    bias: 0,
                    patched: 0,
                });
                continue;
            }
            let (operand, shift, bias, patched) = if mips {
                if matches!(kind, 4 | 10) {
                    let mut reference = unsupported(base + offset, value, pointer_size);
                    reference.operand = Operand::CodeReference;
                    out.push(reference);
                    continue;
                }
                if kuna_mipsreloc::targets_code(kind)
                    || matches!(kind & 0xff, 0 | 9 | 19 | 20 | 21 | 22 | 23 | 37)
                {
                    continue;
                }
                let word = data
                    .get(*offset as usize..*offset as usize + 4)
                    .map(|bytes| {
                        let bytes: [u8; 4] = bytes.try_into().unwrap();
                        if file.is_little_endian() {
                            u32::from_le_bytes(bytes)
                        } else {
                            u32::from_be_bytes(bytes)
                        }
                    });
                let Some(word) = word else { continue };
                let (shift, bias) = match kind {
                    5 => (16, 0x8000),
                    6 => (0, 0),
                    28 if pointer_size == 8 => (32, 0x80008000),
                    29 if pointer_size == 8 => (48, 0x800080008000),
                    _ => {
                        out.push(unsupported(base + offset, value, pointer_size));
                        continue;
                    }
                };
                if reloc.has_implicit_addend() {
                    addend = match kind {
                        5 => ((word as u16 as i64) << 16) + lows[i].unwrap_or(0),
                        6 => word as u16 as i16 as i64,
                        _ => {
                            out.push(unsupported(base + offset, value, pointer_size));
                            continue;
                        }
                    };
                }
                let operand = match word >> 26 {
                    15 => Operand::MipsLui,
                    8 | 9 | 10 | 11 | 24 | 25 | 32..=63 => Operand::MipsSigned,
                    12..=14 => Operand::MipsUnsigned,
                    _ => Operand::Unsupported,
                };
                let patched =
                    value.wrapping_add(addend as u64).wrapping_add(bias) >> shift & 0xffff;
                (operand, shift, bias, patched)
            } else {
                if kind == 4
                    || (kind == 2
                        && offset
                            .checked_sub(1)
                            .and_then(|p| data.get(p as usize))
                            .is_some_and(|&b| matches!(b, 0xe8 | 0xe9)))
                {
                    let mut reference = unsupported(base + offset, value, pointer_size);
                    reference.operand = Operand::CodeReference;
                    out.push(reference);
                    continue;
                }
                if reloc.has_implicit_addend() {
                    out.push(unsupported(base + offset, value, pointer_size));
                    continue;
                }
                match kind {
                    1 | 10 | 11 => (Operand::Absolute, 0, 0, 0),
                    2 => (Operand::PcRelative, 0, 0, 0),
                    _ => {
                        out.push(unsupported(base + offset, value, pointer_size));
                        continue;
                    }
                }
            };
            let slot = *slots.entry(idx).or_insert_with(|| {
                *cursor = (*cursor + pointer_size as u64 - 1) & !(pointer_size as u64 - 1);
                let slot = *cursor;
                *cursor += pointer_size as u64;
                allocated.push((idx, slot));
                slot
            });
            out.push(NullableRelocation {
                field: base + offset,
                slot,
                symbol: value,
                addend,
                pointer_size,
                operand,
                shift,
                bias,
                patched,
            });
        }
    }
    out.sort_by_key(|r| r.field);
    (out, allocated)
}

fn unsupported(field: u64, symbol: u64, pointer_size: i32) -> NullableRelocation {
    NullableRelocation {
        field,
        symbol,
        pointer_size,
        slot: 0,
        addend: 0,
        operand: Operand::Unsupported,
        shift: 0,
        bias: 0,
        patched: 0,
    }
}

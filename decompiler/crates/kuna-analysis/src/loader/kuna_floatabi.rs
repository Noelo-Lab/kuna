//! Whether an ELF says its floating-point arguments, doubles included, travel in
//! floating-point registers (a double-precision hard-float procedure-call
//! standard) or in integer registers (soft-float): ARM build attributes and
//! header flags, the MIPS ABI flags and GNU attributes, the PowerPC GNU
//! attributes, and the RISC-V header flags.  A single-precision convention, which
//! passes doubles in integer registers, and anything unstated are `None`.
use super::kuna_armfloatabi::{string, uleb, word};
use object::{Object, ObjectSection};

pub fn float_arg_registers(file: &object::File<'_>) -> Option<bool> {
    use object::Architecture as A;
    if file.format() != object::BinaryFormat::Elf {
        return None;
    }
    match file.architecture() {
        A::Arm => super::kuna_armfloatabi::args_evidence(file),
        A::Mips | A::Mips64 | A::Mips64_N32 => {
            let record = super::kuna_mipsabi::abi_flags(file).ok()?;
            let value = super::kuna_mipsabi::fp_abi(file, record).ok()??;
            match value {
                1 | 5 | 6 | 7 => Some(true),
                3 => Some(false),
                _ => None,
            }
        }
        A::PowerPc | A::PowerPc64 => match gnu_fp_abi(file)?? & 3 {
            1 => Some(true),
            2 => Some(false),
            _ => None,
        },
        A::Riscv32 | A::Riscv64 => match file.flags() {
            object::FileFlags::Elf { e_flags, .. } => match e_flags & 6 {
                0 => Some(false),
                4 => Some(true),
                _ => None,
            },
            _ => None,
        },
        _ => None,
    }
}

/// The file-scope `Tag_GNU_*_ABI_FP` of every `.gnu.attributes` section:
/// `Some(None)` when none states it, `None` when they conflict or are malformed.
pub(super) fn gnu_fp_abi(file: &object::File<'_>) -> Option<Option<u32>> {
    let mips = matches!(
        file.architecture(),
        object::Architecture::Mips
            | object::Architecture::Mips64
            | object::Architecture::Mips64_N32
    );
    let mut answer = None;
    for section in file
        .sections()
        .filter(|s| s.name() == Ok(".gnu.attributes"))
    {
        if let Some(value) = gnu_tag4(section.data().ok()?, file.is_little_endian(), mips)? {
            if answer.is_some_and(|old| old != value) {
                return None;
            }
            answer = Some(value);
        }
    }
    Some(answer)
}

/// Read tag 4 from the `gnu` vendor's file-scope attributes: except for
/// `Tag_compatibility` (32), odd tags take a string and even tags a number.
fn gnu_tag4(data: &[u8], little: bool, zero_is_unspecified: bool) -> Option<Option<u32>> {
    if data.first() != Some(&b'A') || data.len() > 1024 * 1024 {
        return None;
    }
    let mut cursor = 1;
    let mut answer = None;
    let mut budget = 4096;
    while cursor < data.len() {
        let size = word(data.get(cursor..)?, little)? as usize;
        if size < 5 {
            return None;
        }
        let chunk = data.get(cursor..cursor.checked_add(size)?)?;
        let mut pos = 4;
        let vendor = string(chunk, &mut pos)?;
        cursor += size;
        if vendor != b"gnu" {
            continue;
        }
        while pos < chunk.len() {
            let start = pos;
            let kind = uleb(chunk, &mut pos)?;
            let size = word(chunk.get(pos..)?, little)? as usize;
            pos += 4;
            if size < pos - start {
                return None;
            }
            let end = start.checked_add(size)?;
            let scope = chunk.get(..end)?;
            if kind != 1 {
                return None;
            }
            while pos < end {
                if budget == 0 {
                    return None;
                }
                budget -= 1;
                let tag = uleb(scope, &mut pos)?;
                if tag == 32 {
                    uleb(scope, &mut pos)?;
                    string(scope, &mut pos)?;
                } else if tag & 1 != 0 {
                    string(scope, &mut pos)?;
                } else {
                    let value = uleb(scope, &mut pos)?;
                    if tag == 4 && !(zero_is_unspecified && value == 0) {
                        if answer.is_some_and(|old| old != value) {
                            return None;
                        }
                        answer = Some(value);
                    }
                }
            }
        }
    }
    Some(answer)
}

#[cfg(test)]
mod tests {
    use super::*;
    use object::write::Object as Writer;
    use object::{Architecture, BinaryFormat, Endianness, FileFlags, SectionKind};

    fn attributes(vendor: &[u8], tags: &[u8], little: bool) -> Vec<u8> {
        let put = |v: u32| {
            if little {
                v.to_le_bytes()
            } else {
                v.to_be_bytes()
            }
        };
        let mut sub = vec![1];
        sub.extend(put(5 + tags.len() as u32));
        sub.extend(tags);
        let mut chunk = Vec::new();
        chunk.extend(put((4 + vendor.len() + 1 + sub.len()) as u32));
        chunk.extend(vendor);
        chunk.push(0);
        chunk.extend(sub);
        let mut data = vec![b'A'];
        data.extend(chunk);
        data
    }

    fn elf(
        arch: Architecture,
        endian: Endianness,
        e_flags: u32,
        sections: &[(&[u8], Vec<u8>)],
    ) -> Vec<u8> {
        let mut object = Writer::new(BinaryFormat::Elf, arch, endian);
        object.flags = FileFlags::Elf {
            os_abi: 0,
            abi_version: 0,
            e_flags,
        };
        for (name, data) in sections {
            let id = object.add_section(Vec::new(), name.to_vec(), SectionKind::Other);
            object.append_section_data(id, data, 1);
        }
        object.write().unwrap()
    }

    fn evidence(bytes: &[u8]) -> Option<bool> {
        float_arg_registers(&object::File::parse(bytes).unwrap())
    }

    fn abiflags(fp_abi: u8) -> Vec<u8> {
        let mut data = vec![0; 24];
        data[7] = fp_abi;
        data
    }

    #[test]
    fn gnu_attributes_skip_strings_and_read_the_fp_tag() {
        for little in [true, false] {
            let data = attributes(b"gnu", &[5, b'x', 0, 32, 1, b'y', 0, 8, 1, 4, 2], little);
            assert_eq!(gnu_tag4(&data, little, false), Some(Some(2)));
            assert_eq!(
                gnu_tag4(&attributes(b"gnu", &[8, 1], little), little, false),
                Some(None)
            );
            assert_eq!(
                gnu_tag4(&attributes(b"xyz", &[4, 1], little), little, false),
                Some(None)
            );
            let conflict = attributes(b"gnu", &[4, 1, 4, 2], little);
            assert_eq!(gnu_tag4(&conflict, little, false), None);
            let good = attributes(b"gnu", &[4, 1], little);
            for end in 1..good.len() {
                assert_ne!(gnu_tag4(&good[..end], little, false), Some(Some(1)));
            }
        }
    }

    #[test]
    fn mips_abi_flags_and_gnu_attributes_must_agree() {
        for (endian, little) in [(Endianness::Little, true), (Endianness::Big, false)] {
            for (value, expected) in [(1, Some(true)), (3, Some(false)), (0, None), (2, None)] {
                let flags = elf(
                    Architecture::Mips,
                    endian,
                    0x70001000,
                    &[(b".MIPS.abiflags", abiflags(value))],
                );
                assert_eq!(evidence(&flags), expected);
                let n32 = elf(
                    Architecture::Mips,
                    endian,
                    0x80000020,
                    &[(b".MIPS.abiflags", abiflags(value))],
                );
                assert_eq!(evidence(&n32), expected);
                let gnu = elf(
                    Architecture::Mips,
                    endian,
                    0x70001000,
                    &[(b".gnu.attributes", attributes(b"gnu", &[4, value], little))],
                );
                assert_eq!(evidence(&gnu), expected);
            }
            let conflict = elf(
                Architecture::Mips,
                endian,
                0x70001000,
                &[
                    (b".MIPS.abiflags", abiflags(1)),
                    (b".gnu.attributes", attributes(b"gnu", &[4, 3], little)),
                ],
            );
            assert_eq!(evidence(&conflict), None);
            assert_eq!(
                evidence(&elf(Architecture::Mips, endian, 0x70001000, &[])),
                None
            );
        }
    }

    #[test]
    fn mips_fp_any_is_ignored_in_repeated_tags_and_sections() {
        for (endian, little) in [(Endianness::Little, true), (Endianness::Big, false)] {
            for (value, expected) in [
                (1, Some(true)),
                (3, Some(false)),
                (6, Some(true)),
                (0, None),
            ] {
                for tags in [vec![4, 0, 4, value], vec![4, value, 4, 0]] {
                    let bytes = elf(
                        Architecture::Mips,
                        endian,
                        0x80001000,
                        &[(b".gnu.attributes", attributes(b"gnu", &tags, little))],
                    );
                    assert_eq!(evidence(&bytes), expected);
                }
                for (a, b) in [(0, value), (value, 0)] {
                    let bytes = elf(
                        Architecture::Mips,
                        endian,
                        0x80001000,
                        &[
                            (b".gnu.attributes", attributes(b"gnu", &[4, a], little)),
                            (b".gnu.attributes", attributes(b"gnu", &[4, b], little)),
                        ],
                    );
                    assert_eq!(evidence(&bytes), expected);
                }
            }
            let bytes = elf(
                Architecture::Mips,
                endian,
                0x80001000,
                &[(
                    b".gnu.attributes",
                    attributes(b"gnu", &[4, 1, 4, 0, 4, 3], little),
                )],
            );
            assert_eq!(
                gnu_fp_abi(&object::File::parse(bytes.as_slice()).unwrap()),
                None
            );
        }
    }

    #[test]
    fn powerpc_reads_the_gnu_fp_tag_and_riscv_its_header() {
        for (value, expected) in [
            (1, Some(true)),
            (2, Some(false)),
            (3, None),
            (0, None),
            (9, Some(true)),
        ] {
            let bytes = elf(
                Architecture::PowerPc,
                Endianness::Big,
                0,
                &[(b".gnu.attributes", attributes(b"gnu", &[4, value], false))],
            );
            assert_eq!(evidence(&bytes), expected, "{value}");
        }
        assert_eq!(
            evidence(&elf(Architecture::PowerPc, Endianness::Big, 0, &[])),
            None
        );
        for (flags, expected) in [
            (0, Some(false)),
            (2, None),
            (3, None),
            (4, Some(true)),
            (5, Some(true)),
            (6, None),
        ] {
            let bytes = elf(Architecture::Riscv64, Endianness::Little, flags, &[]);
            assert_eq!(evidence(&bytes), expected, "{flags:#x}");
        }
    }

    #[test]
    fn arm_base_variant_is_soft_and_unstated_is_unknown() {
        let tag = |tags: &[u8]| attributes(b"aeabi", tags, true);
        for (tags, expected) in [
            (&[6u8, 10][..], Some(false)),
            (&[28, 0], Some(false)),
            (&[28, 1], Some(true)),
            (&[28, 3], None),
        ] {
            let bytes = elf(
                Architecture::Arm,
                Endianness::Little,
                0x05000000,
                &[(b".ARM.attributes", tag(tags))],
            );
            assert_eq!(evidence(&bytes), expected, "{tags:?}");
        }
        assert_eq!(
            evidence(&elf(Architecture::Arm, Endianness::Little, 0x05000000, &[])),
            None
        );
    }
}

//! ARM ELF build-attribute evidence (AAELF32 and Addenda32): the VFP procedure
//! call standard (`Tag_ABI_VFP_args`) and whether the image can hold A32 code.
use object::{Object, ObjectSection};

/// `Tag_CPU_arch_profile`; the value is a character, `'M'` for a microcontroller.
const TAG_CPU_ARCH_PROFILE: u32 = 7;
/// `Tag_ARM_ISA_use`; 0 says the image may not use A32 instructions.
const TAG_ARM_ISA_USE: u32 = 8;

/// Whether the build attributes rule out A32 code: an M-profile CPU executes
/// only Thumb, and `Tag_ARM_ISA_use` 0 forbids A32 outright. Only an explicit
/// tag counts; an image without attributes may hold either.
pub fn thumb_only(file: &object::File<'_>) -> bool {
    if file.format() != object::BinaryFormat::Elf
        || file.architecture() != object::Architecture::Arm
    {
        return false;
    }
    file.sections()
        .filter(|s| s.name() == Ok(".ARM.attributes"))
        .any(|section| {
            let Ok(data) = section.data() else {
                return false;
            };
            let little = file.is_little_endian();
            let tag = |wanted| file_attribute(data, little, wanted, |v| v, None).flatten();
            tag(TAG_CPU_ARCH_PROFILE) == Some(u32::from(b'M')) || tag(TAG_ARM_ISA_USE) == Some(0)
        })
}

pub fn vfp_args(file: &object::File<'_>) -> bool {
    if file.format() != object::BinaryFormat::Elf
        || file.architecture() != object::Architecture::Arm
    {
        return false;
    }
    let linked = matches!(
        file.kind(),
        object::ObjectKind::Executable | object::ObjectKind::Dynamic
    );
    let flags = match file.flags() {
        object::FileFlags::Elf { e_flags, .. } => e_flags,
        _ => return false,
    };
    let header = if linked && flags >> 24 >= 5 {
        match flags & 0x600 {
            0x400 => Some(true),
            0 | 0x200 => Some(false),
            _ => return false,
        }
    } else {
        None
    };
    let mut attribute = None;
    for section in file
        .sections()
        .filter(|s| s.name() == Ok(".ARM.attributes"))
    {
        let Ok(data) = section.data() else {
            return false;
        };
        let Some(value) = attributes(data, file.is_little_endian()) else {
            return false;
        };
        if let Some(value) = value {
            if attribute.is_some_and(|old| old != value) {
                return false;
            }
            attribute = Some(value);
        }
    }
    match (header, attribute) {
        (Some(a), Some(b)) => a && b,
        (Some(a), None) | (None, Some(a)) => a,
        _ => false,
    }
}

/// The convention the container states for floating-point arguments:
/// `Some(true)` the VFP variant, `Some(false)` the base (core-register) variant,
/// `None` when it states neither or its evidence conflicts or is malformed.
pub fn args_evidence(file: &object::File<'_>) -> Option<bool> {
    if file.format() != object::BinaryFormat::Elf
        || file.architecture() != object::Architecture::Arm
    {
        return None;
    }
    let linked = matches!(
        file.kind(),
        object::ObjectKind::Executable | object::ObjectKind::Dynamic
    );
    let object::FileFlags::Elf { e_flags: flags, .. } = file.flags() else {
        return None;
    };
    let header = match flags & 0x600 {
        0x400 if linked && flags >> 24 >= 5 => Some(true),
        0x200 if linked && flags >> 24 >= 5 => Some(false),
        _ => None,
    };
    let mut attribute = None;
    for section in file
        .sections()
        .filter(|s| s.name() == Ok(".ARM.attributes"))
    {
        let data = section.data().ok()?;
        if let Some(value) = attribute_value(data, file.is_little_endian())? {
            if attribute.is_some_and(|old| old != value) {
                return None;
            }
            attribute = Some(value);
        }
    }
    let attribute = match attribute {
        Some(0) => Some(false),
        Some(1) => Some(true),
        _ => None,
    };
    match (header, attribute) {
        (Some(a), Some(b)) => (a == b).then_some(a),
        (a, b) => a.or(b),
    }
}

pub(super) fn word(data: &[u8], little: bool) -> Option<u32> {
    let bytes: [u8; 4] = data.get(..4)?.try_into().ok()?;
    Some(if little {
        u32::from_le_bytes(bytes)
    } else {
        u32::from_be_bytes(bytes)
    })
}
pub(super) fn uleb(data: &[u8], pos: &mut usize) -> Option<u32> {
    let mut result = 0;
    for shift in (0..35).step_by(7) {
        let b = *data.get(*pos)?;
        *pos += 1;
        if shift == 28 && b > 15 {
            return None;
        }
        result |= u32::from(b & 127) << shift;
        if b & 128 == 0 {
            return Some(result);
        }
    }
    None
}
pub(super) fn string<'a>(data: &'a [u8], pos: &mut usize) -> Option<&'a [u8]> {
    let begin = *pos;
    let end = begin + data.get(begin..)?.iter().position(|&b| b == 0)?;
    *pos = end + 1;
    Some(&data[begin..end])
}
fn attributes(data: &[u8], little: bool) -> Option<Option<bool>> {
    attribute_value(data, little).map(|value| value.map(|v| v == 1))
}

/// Whether the container states floating-point register hardware: `Some(true)`
/// when an `aeabi` subsection names an FP, Advanced SIMD or MVE architecture,
/// `Some(false)` when it names none of them, `None` when the image carries no
/// `aeabi` subsection or its attributes conflict or are malformed.
pub fn fp_hardware(file: &object::File<'_>) -> Option<bool> {
    if file.format() != object::BinaryFormat::Elf
        || file.architecture() != object::Architecture::Arm
    {
        return None;
    }
    let mut answer = None;
    for section in file
        .sections()
        .filter(|s| s.name() == Ok(".ARM.attributes"))
    {
        let data = section.data().ok()?;
        let Some(value) = hardware_value(data, file.is_little_endian())? else {
            continue;
        };
        if answer.is_some_and(|old| old != value) {
            return None;
        }
        answer = Some(value);
    }
    answer
}

fn hardware_value(data: &[u8], little: bool) -> Option<Option<bool>> {
    let fp = tag_value(data, little, 10)?;
    let simd = tag_value(data, little, 12)?;
    let mve = tag_value(data, little, 48)?;
    Some(match (fp, simd, mve) {
        (Some(fp), Some(simd), Some(mve)) => Some(fp != 0 || simd != 0 || mve != 0),
        _ => None,
    })
}

/// The file-scope `Tag_ABI_VFP_args` value, 0 when an `aeabi` subsection omits it.
fn attribute_value(data: &[u8], little: bool) -> Option<Option<u32>> {
    tag_value(data, little, 28)
}

/// The file-scope value of the numeric `tag`, 0 when an `aeabi` subsection omits it.
fn tag_value(data: &[u8], little: bool, wanted: u32) -> Option<Option<u32>> {
    file_attribute(data, little, wanted, |v| v, Some(0))
}

/// The file-scope value of the integer attribute `wanted`, through `map`.
/// A subsection without the tag answers `absent`; subsections that answer
/// differently, or a malformed section, answer `None`.
fn file_attribute<T: Copy + PartialEq>(
    data: &[u8],
    little: bool,
    wanted: u32,
    map: impl Fn(u32) -> T,
    absent: Option<T>,
) -> Option<Option<T>> {
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
        if vendor != b"aeabi" {
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
            let mut current = None;
            while pos < end {
                if budget == 0 {
                    return None;
                }
                budget -= 1;
                let tag = uleb(scope, &mut pos)?;
                if tag == 0 {
                    continue;
                }
                if tag == 32 {
                    uleb(scope, &mut pos)?;
                    string(scope, &mut pos)?;
                } else if tag == 4 || tag == 5 || (tag >= 32 && tag & 1 != 0) {
                    string(scope, &mut pos)?;
                } else {
                    let value = uleb(scope, &mut pos)?;
                    if tag == wanted {
                        let value = map(value);
                        if current.is_some_and(|old| old != value) {
                            return None;
                        }
                        current = Some(value);
                    }
                }
            }
            if let Some(current) = current.or(absent) {
                if answer.is_some_and(|old| old != current) {
                    return None;
                }
                answer = Some(current);
            }
        }
    }
    Some(answer)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn tagged(tags: &[u8], little: bool) -> Vec<u8> {
        let put = |v: usize| {
            if little {
                (v as u32).to_le_bytes()
            } else {
                (v as u32).to_be_bytes()
            }
        };
        let mut data = vec![b'A'];
        data.extend(put(4 + 6 + 5 + tags.len()));
        data.extend(b"aeabi\0");
        data.push(1);
        data.extend(put(5 + tags.len()));
        data.extend(tags);
        data
    }
    #[test]
    fn floating_point_hardware_is_read_from_the_architecture_tags() {
        for little in [true, false] {
            let read = |tags: &[u8]| hardware_value(&tagged(tags, little), little);
            assert_eq!(read(&[6, 10]), Some(Some(false)));
            assert_eq!(read(&[6, 10, 10, 0, 12, 0]), Some(Some(false)));
            assert_eq!(read(&[6, 10, 10, 4]), Some(Some(true)));
            assert_eq!(read(&[12, 2]), Some(Some(true)));
            assert_eq!(read(&[48, 1]), Some(Some(true)));
            assert_eq!(hardware_value(b"A", little), Some(None));
        }
    }
    fn section(value: u8, little: bool) -> Vec<u8> {
        let mut result = vec![b'A'];
        let bytes = if little {
            16u32.to_le_bytes()
        } else {
            16u32.to_be_bytes()
        };
        result.extend(bytes);
        result.extend(b"aeabi\0");
        result.push(1);
        result.extend(if little {
            6u32.to_le_bytes()
        } else {
            6u32.to_be_bytes()
        });
        result.extend([28, value]);
        // File subsection length includes the tag byte and four-byte length.
        let bytes = if little {
            7u32.to_le_bytes()
        } else {
            7u32.to_be_bytes()
        };
        result[12..16].copy_from_slice(&bytes);
        let size = (result.len() - 1) as u32;
        result[1..5].copy_from_slice(&if little {
            size.to_le_bytes()
        } else {
            size.to_be_bytes()
        });
        result
    }
    #[test]
    fn only_the_vfp_calling_convention_is_positive_evidence() {
        for little in [true, false] {
            for value in 0..=3 {
                assert_eq!(
                    attributes(&section(value, little), little),
                    Some(Some(value == 1))
                );
            }
            let good = section(1, little);
            for end in 0..good.len() {
                assert_ne!(attributes(&good[..end], little), Some(Some(true)));
            }
            let mut conflict = good.clone();
            conflict.extend_from_slice(&section(0, little)[1..]);
            assert_eq!(attributes(&conflict, little), None);
        }
    }
    #[test]
    fn elf_flags_attributes_and_conflicts_are_checked_together() {
        use object::write::Object as Writer;
        use object::{Architecture, BinaryFormat, Endianness, FileFlags, SectionKind};
        for endian in [Endianness::Little, Endianness::Big] {
            for linked in [false, true] {
                for flags in [0, 0x05000000, 0x05000200, 0x05000400, 0x05000600] {
                    for value in [None, Some(0), Some(1), Some(2), Some(3)] {
                        let mut object = Writer::new(BinaryFormat::Elf, Architecture::Arm, endian);
                        object.flags = FileFlags::Elf {
                            os_abi: 0,
                            abi_version: 0,
                            e_flags: flags,
                        };
                        if let Some(value) = value {
                            let id = object.add_section(
                                Vec::new(),
                                b".ARM.attributes".to_vec(),
                                SectionKind::Other,
                            );
                            object.append_section_data(
                                id,
                                &section(value, endian == Endianness::Little),
                                1,
                            );
                        }
                        let mut bytes = object.write().unwrap();
                        if linked {
                            bytes[16..18].copy_from_slice(&if endian == Endianness::Little {
                                3u16.to_le_bytes()
                            } else {
                                3u16.to_be_bytes()
                            });
                        }
                        let file = object::File::parse(&*bytes).unwrap();
                        let header = linked && flags >> 24 >= 5;
                        let expected = if header {
                            flags & 0x600 == 0x400 && value.is_none_or(|v| v == 1)
                        } else {
                            value == Some(1)
                        };
                        assert_eq!(
                            vfp_args(&file),
                            expected,
                            "{endian:?} linked={linked} flags={flags:x} attribute={value:?}"
                        );
                    }
                }
            }
        }
    }
    #[test]
    fn scoped_or_malformed_attribute_evidence_is_declined() {
        for little in [true, false] {
            let good = section(1, little);
            for kind in [2, 3] {
                let mut scoped = good.clone();
                scoped[11] = kind;
                assert_eq!(attributes(&scoped, little), None);
            }
            let mut malformed = good.clone();
            malformed[17] = 0x80;
            assert_eq!(attributes(&malformed, little), None);
            let mut unknown = good.clone();
            unknown[5..10].copy_from_slice(b"other");
            assert_eq!(attributes(&unknown, little), Some(None));
        }
    }

    fn arm_object(attributes: Option<Vec<u8>>) -> Vec<u8> {
        use object::write::Object as Writer;
        use object::{Architecture, BinaryFormat, Endianness, SectionKind};
        let mut object = Writer::new(BinaryFormat::Elf, Architecture::Arm, Endianness::Little);
        if let Some(data) = attributes {
            let id =
                object.add_section(Vec::new(), b".ARM.attributes".to_vec(), SectionKind::Other);
            object.append_section_data(id, &data, 1);
        }
        object.write().unwrap()
    }

    #[test]
    fn only_an_explicit_m_profile_or_a32_ban_is_thumb_only() {
        let cases = [
            (None, false),
            (Some(tagged(&[6, 13, 7, b'M', 9, 2], true)), true),
            (Some(tagged(&[6, 10, 7, b'A', 8, 1, 9, 2], true)), false),
            (Some(tagged(&[7, b'R'], true)), false),
            (Some(tagged(&[8, 0], true)), true),
            (Some(tagged(&[28, 1], true)), false),
        ];
        for (attributes, expected) in cases {
            let bytes = arm_object(attributes.clone());
            let file = object::File::parse(&*bytes).unwrap();
            assert_eq!(thumb_only(&file), expected, "{attributes:?}");
        }
    }

    #[test]
    fn the_vfp_answer_ignores_the_other_tags() {
        assert_eq!(
            attributes(&tagged(&[7, b'M', 28, 1], true), true),
            Some(Some(true))
        );
        assert_eq!(attributes(&tagged(&[7, b'M'], true), true), Some(Some(false)));
    }
}

//! ARM ELF VFP procedure-call evidence (AAELF32 and Addenda32 Tag_ABI_VFP_args).
use object::{Object, ObjectSection};

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

fn word(data: &[u8], little: bool) -> Option<u32> {
    let bytes: [u8; 4] = data.get(..4)?.try_into().ok()?;
    Some(if little {
        u32::from_le_bytes(bytes)
    } else {
        u32::from_be_bytes(bytes)
    })
}
fn uleb(data: &[u8], pos: &mut usize) -> Option<u32> {
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
fn string<'a>(data: &'a [u8], pos: &mut usize) -> Option<&'a [u8]> {
    let begin = *pos;
    let end = begin + data.get(begin..)?.iter().position(|&b| b == 0)?;
    *pos = end + 1;
    Some(&data[begin..end])
}
fn attributes(data: &[u8], little: bool) -> Option<Option<bool>> {
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
                    if tag == 28 {
                        let value = value == 1;
                        if current.is_some_and(|old| old != value) {
                            return None;
                        }
                        current = Some(value);
                    }
                }
            }
            let current = current.unwrap_or(false);
            if answer.is_some_and(|old| old != current) {
                return None;
            }
            answer = Some(current);
        }
    }
    Some(answer)
}

#[cfg(test)]
mod tests {
    use super::*;
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
}

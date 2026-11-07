//! Recognize a PIC address-setup prefix before assuming an unsymbolized t9.
use kuna_sleigh::loadimage::ImageBytes;

fn rs(word: u32) -> u32 {
    (word >> 21) & 31
}
fn rt(word: u32) -> u32 {
    (word >> 16) & 31
}
fn rd(word: u32) -> u32 {
    (word >> 11) & 31
}

fn add_t9(word: u32, source: u32) -> Option<u32> {
    if matches!(word & 0xfc00_07ff, 0x21 | 0x2d)
        && ((rs(word) == source && rt(word) == 25) || (rs(word) == 25 && rt(word) == source))
        && !matches!(rd(word), 0 | 25)
    {
        Some(rd(word))
    } else {
        None
    }
}

fn add_immediate(word: u32, reg: u32) -> bool {
    matches!(word >> 26, 9 | 25) && rs(word) == reg && rt(word) == reg
}

fn setup(words: &[u32]) -> bool {
    let [hi, middle, last, ..] = words else {
        return false;
    };
    if hi & 0xffe0_0000 != 0x3c00_0000 || matches!(rt(*hi), 0 | 25) {
        return false;
    }
    let temp = rt(*hi);
    (add_immediate(*middle, temp) && add_t9(*last, temp).is_some())
        || add_t9(*middle, temp).is_some_and(|result| add_immediate(*last, result))
}

fn frame_prefix(word: u32) -> bool {
    (add_immediate(word, 29) && (word as u16 as i16) < 0)
        || (matches!(word >> 26, 0x2b | 0x3f) && rs(word) == 29)
        || matches!(word, 0x03a0_f025 | 0x03a0_f021 | 0x03a0_f02d)
}

fn has_setup(bytes: &[u8], little: bool) -> bool {
    let words: Vec<u32> = super::kuna_mipsabi::words(bytes, little).take(16).collect();
    for index in 0..words.len() {
        if setup(&words[index..]) {
            return true;
        }
        if !frame_prefix(words[index]) {
            return false;
        }
    }
    false
}

/// A function address as a register of the image's pointer class holds it: an
/// ELF32 pointer is sign-extended, as `lw` and 32-bit arithmetic leave it in a
/// 64-bit register. A consumer truncates the value to its register's width.
pub fn class_pointer(address: u64, elf32: bool) -> u64 {
    if elf32 {
        (address as u32 as i32 as i64) as u64
    } else {
        address
    }
}

/// Only the PIC setup convention supplies t9. An interior block or a plain
/// indirect call may instead consume a caller's arbitrary incoming value. The
/// bytes are read without a staging window, so probing leaves later reads alone.
pub fn value(image: &dyn ImageBytes, entry: u64, elf32: bool, little: bool) -> Option<u64> {
    if entry & 3 != 0 {
        return None;
    }
    let mut bytes = [0; 64];
    if image.fill_span(&mut bytes, entry) != 0 || !has_setup(&bytes, little) {
        return None;
    }
    Some(class_pointer(entry, elf32))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct Image([u8; 64]);
    impl ImageBytes for Image {
        fn fill_span(&self, bytes: &mut [u8], _: u64) -> usize {
            bytes.copy_from_slice(&self.0[..bytes.len()]);
            0
        }
        fn mapped_covers(&self, _: u64, _: u64) -> bool { true }
    }

    #[test]
    fn elf32_entry_seeds_sign_extend_high_addresses() {
        for little in [false, true] {
            let mut bytes = [0; 64];
            for (raw, word) in bytes.chunks_exact_mut(4).zip([0x3c1c0002u32, 0x279c8000, 0x0399e021]) {
                raw.copy_from_slice(&if little { word.to_le_bytes() } else { word.to_be_bytes() });
            }
            let image = Image(bytes);
            for offset in [0x1000, 0x80001000, 0xffff1000] {
                assert_eq!(value(&image, offset, true, little),
                    Some((offset as u32 as i32 as i64) as u64));
                assert_eq!(value(&image, offset, false, little), Some(offset));
            }
            assert_eq!(value(&image, 0x1002, true, little), None);
        }
    }

    #[test]
    fn unmapped_entries_have_no_seed() {
        #[derive(Debug)]
        struct Unmapped;
        impl ImageBytes for Unmapped {
            fn fill_span(&self, bytes: &mut [u8], _: u64) -> usize { bytes.len() }
            fn mapped_covers(&self, _: u64, _: u64) -> bool { false }
        }
        assert_eq!(value(&Unmapped, 0x1000, false, false), None);
    }

    #[test]
    fn pic_prefixes_require_the_complete_linked_address_computation() {
        for little in [false, true] {
            let bytes = |words: &[u32]| {
                words
                    .iter()
                    .flat_map(|word| {
                        if little {
                            word.to_le_bytes()
                        } else {
                            word.to_be_bytes()
                        }
                    })
                    .collect::<Vec<_>>()
            };
            for words in [
                vec![0x3c030001, 0x00791821, 0x2463a000],
                vec![0x3c030001, 0x0079182d, 0x6463a000],
                vec![0x3c1c0002, 0x279c8000, 0x0399e021],
                vec![
                    0x27bdffe0, 0xafbe001c, 0x03a0f025, 0x3c030001, 0x00791821, 0x2463a000,
                ],
            ] {
                let raw = bytes(&words);
                assert!(has_setup(&raw, little));
                for end in 0..raw.len() {
                    assert!(!has_setup(&raw[..end], little));
                }
            }
            for words in [
                vec![0x0320f809, 0, 0],
                vec![
                    0x27bdffd8, 0xafb10020, 0x2411ffff, 0xafb0001c, 0x24701ff0, 0xafbf0024,
                    0x0320f809,
                ],
                vec![0x3c030001, 0x00791821, 0x2484a000],
                vec![0x3c030001, 0x00781821, 0x2463a000],
                vec![0x03e00008, 0, 0x3c030001, 0x00791821, 0x2463a000],
                vec![0, 0x3c1c0002, 0x279c8000, 0x0399e021],
                vec![0, 0, 0x27bdffe0, 0x3c030001, 0x00791821, 0x2463a000],
            ] {
                assert!(!has_setup(&bytes(&words), little));
            }
        }
    }
}

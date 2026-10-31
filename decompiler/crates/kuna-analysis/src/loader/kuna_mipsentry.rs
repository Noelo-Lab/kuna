//! Recognize a PIC address-setup prefix before assuming an unsymbolized t9.
use kuna_base::address::Address;
use kuna_sleigh::loadimage::LoadImage;

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
    word == 0
        || (add_immediate(word, 29) && (word as u16 as i16) < 0)
        || (matches!(word >> 26, 0x2b | 0x3f) && rs(word) == 29)
        || matches!(word, 0x03a0_f025 | 0x03a0_f021 | 0x03a0_f02d)
}

fn has_setup(bytes: &[u8], little: bool) -> bool {
    let mut words = [0; 16];
    let count = (bytes.len() / 4).min(words.len());
    for (word, raw) in words.iter_mut().zip(bytes.chunks_exact(4)).take(count) {
        *word = if little {
            u32::from_le_bytes(raw.try_into().unwrap())
        } else {
            u32::from_be_bytes(raw.try_into().unwrap())
        };
    }
    for index in 0..count {
        if setup(&words[index..count]) {
            return true;
        }
        if !frame_prefix(words[index]) {
            return false;
        }
    }
    false
}

/// Only the PIC setup convention supplies t9. An interior block or a plain
/// indirect call may instead consume a caller's arbitrary incoming value.
pub fn value(image: &mut dyn LoadImage, entry: &Address, n32: bool, little: bool) -> Option<u64> {
    if entry.get_offset() & 3 != 0 {
        return None;
    }
    let mut bytes = [0; 64];
    image.load_fill(&mut bytes, entry).ok()?;
    if !has_setup(&bytes, little) {
        return None;
    }
    Some(if n32 {
        (entry.get_offset() as u32 as i32 as i64) as u64
    } else {
        entry.get_offset()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

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
            ] {
                assert!(!has_setup(&bytes(&words), little));
            }
        }
    }
}

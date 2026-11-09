//! `rt::jchar` against every `char` classified by a JDK 21 (`data/GenJcharGolden.java`).

use kuna_jvm::rt::jchar::*;

fn describe(c: u16) -> String {
    let preds = [
        is_java_identifier_start(c),
        is_java_identifier_part(c),
        is_identifier_ignorable(c),
        is_digit(c),
        is_letter(c),
        is_letter_or_digit(c),
        is_whitespace(c),
        is_iso_control(c),
        is_surrogate(c),
        is_high_surrogate(c),
        is_low_surrogate(c),
    ];
    let flags = preds
        .iter()
        .enumerate()
        .fold(0, |acc, (bit, &on)| acc | ((on as i32) << bit));
    format!(
        "{} {} {} {}",
        get_type(c),
        flags,
        digit(c, 10),
        digit(c, 36)
    )
}

#[test]
fn every_char_matches_jdk21() {
    let data = include_str!("data/jchar_golden.txt");
    let mut covered = 0u32;
    for line in data.lines() {
        let mut parts = line.splitn(3, ' ');
        let first = u32::from_str_radix(parts.next().unwrap(), 16).unwrap();
        let last = u32::from_str_radix(parts.next().unwrap(), 16).unwrap();
        let want = parts.next().unwrap();
        for c in first..=last {
            assert_eq!(describe(c as u16), want, "char U+{c:04X}");
            covered += 1;
        }
    }
    assert_eq!(covered, 0x10000);
    assert_eq!(digit(b'7' as u16, 8), 7);
    assert_eq!(digit(b'8' as u16, 8), -1);
    assert_eq!(digit(b'z' as u16, 37), -1);
}

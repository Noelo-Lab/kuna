//! `rt::jnum` against vectors a JDK 21 produced (`data/GenJnumGolden.java`).

use kuna_jvm::rt::jnum::*;
use kuna_jvm::rt::jstring::from_utf16;
use kuna_jvm::rt::{try_catch, JavaEq, JavaHash};

fn decode(tok: &str) -> String {
    if let Some(raw) = tok.strip_prefix('=') {
        return raw.to_string();
    }
    let hex = tok.strip_prefix('x').expect("encoded input");
    let units: Vec<u16> = (0..hex.len() / 4)
        .map(|i| u16::from_str_radix(&hex[i * 4..i * 4 + 4], 16).unwrap())
        .collect();
    from_utf16(&units)
}

fn outcome<T>(f: impl FnOnce() -> T, show: impl Fn(T) -> String) -> String {
    match try_catch(f) {
        Ok(v) => show(v),
        Err(c) => {
            assert!(c.is("NumberFormatException"), "{}", c.message);
            "NFE".to_string()
        }
    }
}

#[test]
fn conversions_match_jdk21() {
    let data = include_str!("data/jnum_golden.txt");
    let mut counts = [0usize; 10];
    for (lineno, line) in data.lines().enumerate() {
        let f: Vec<&str> = line.splitn(3, ' ').collect();
        let at = lineno + 1;
        match f[0] {
            "D" => {
                let v = f64::from_bits(u64::from_str_radix(f[1], 16).unwrap());
                assert_eq!(double_to_string(v), f[2], "Double.toString line {at}");
                counts[0] += 1;
            }
            "F" => {
                let v = f32::from_bits(u32::from_str_radix(f[1], 16).unwrap());
                assert_eq!(float_to_string(v), f[2], "Float.toString line {at}");
                counts[1] += 1;
            }
            "PD" => {
                let s = decode(f[1]);
                let got = outcome(|| parse_double(&s), |v| format!("{:016x}", v.to_bits()));
                assert_eq!(got, f[2], "parseDouble({s:?}) line {at}");
                counts[2] += 1;
            }
            "PF" => {
                let s = decode(f[1]);
                let got = outcome(|| parse_float(&s), |v| format!("{:08x}", v.to_bits()));
                assert_eq!(got, f[2], "parseFloat({s:?}) line {at}");
                counts[3] += 1;
            }
            "PI" => {
                let s = decode(f[1]);
                let got = outcome(|| parse_int(&s), |v| v.to_string());
                assert_eq!(got, f[2], "parseInt({s:?}) line {at}");
                counts[4] += 1;
            }
            "PL" => {
                let s = decode(f[1]);
                let got = outcome(|| parse_long(&s), |v| v.to_string());
                assert_eq!(got, f[2], "parseLong({s:?}) line {at}");
                counts[5] += 1;
            }
            "HI" => {
                assert_eq!(int_to_hex_string(f[1].parse().unwrap()), f[2], "line {at}");
                counts[6] += 1;
            }
            "HL" => {
                assert_eq!(long_to_hex_string(f[1].parse().unwrap()), f[2], "line {at}");
                counts[7] += 1;
            }
            "DB" => {
                let v = f64::from_bits(u64::from_str_radix(f[1], 16).unwrap());
                let rest: Vec<&str> = f[2].split(' ').collect();
                assert_eq!(double_to_long_bits(v).to_string(), rest[0], "line {at}");
                assert_eq!(v.java_hash().to_string(), rest[1], "line {at}");
                counts[8] += 1;
            }
            "FB" => {
                let v = f32::from_bits(u32::from_str_radix(f[1], 16).unwrap());
                let rest: Vec<&str> = f[2].split(' ').collect();
                assert_eq!(float_to_int_bits(v).to_string(), rest[0], "line {at}");
                assert_eq!(v.java_hash().to_string(), rest[1], "line {at}");
                counts[9] += 1;
            }
            other => panic!("unknown record {other} on line {at}"),
        }
    }
    assert_eq!(
        counts,
        [11139, 5332, 8750, 8750, 148, 148, 51, 47, 37, 37],
        "record counts"
    );
}

#[test]
fn float_keys_follow_float_equals() {
    assert!(f64::NAN.java_equals(&f64::from_bits(0x7ff8_0000_0000_0001)));
    assert!(!0.0f64.java_equals(&-0.0));
    assert!(!0.0f32.java_equals(&-0.0));
    assert_eq!(1.0f32.java_hash(), 0x3f80_0000);
    assert_eq!(FLOAT_MIN_VALUE.to_bits(), 1);
    assert_eq!(DOUBLE_MIN_VALUE.to_bits(), 1);
    assert_eq!(float_to_string(FLOAT_MIN_VALUE), "1.4E-45");
    assert_eq!(double_to_string(DOUBLE_MIN_VALUE), "4.9E-324");
}

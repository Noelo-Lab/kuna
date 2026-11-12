use super::sig::{crc16, parse};
use super::*;

/// A version-6 signature with one module: prefix `55 48 89 e5`, CRC over the
/// four bytes at offset 32, length 40, public `myfunc`.
const TINY: &str = "49444153474e0600000000000000000000000000000000000000000000000000000007000001000000746573746c6962010400554889e50004bc1628006d7966756e6300";

fn tiny() -> Sig {
    let bytes: Vec<u8> = (0..TINY.len()).step_by(2).map(|i| u8::from_str_radix(&TINY[i..i + 2], 16).unwrap()).collect();
    parse(&bytes).expect("tiny sig parses")
}

fn code() -> Vec<u8> {
    let mut c = vec![0x55, 0x48, 0x89, 0xe5];
    c.extend([0u8; 28]);
    c.extend([0x90u8; 8]);
    c
}

#[test]
fn parses_the_header_and_the_one_module() {
    let s = tiny();
    assert_eq!(s.library, "testlib");
    assert_eq!(s.pattern_len, 32);
    let mods = s.modules();
    assert_eq!(mods.len(), 1);
    assert_eq!(mods[0].length, 40);
    assert_eq!(mods[0].crc_len, 4);
    assert_eq!(mods[0].publics[0].name, "myfunc");
}

#[test]
fn startup_and_foreign_file_type_signatures_do_not_apply() {
    let mut h = vec![0u8; 41];
    h[..6].copy_from_slice(b"IDASGN");
    assert!(sig::applies_to(&h, Some(18)), "an unset file-type mask applies to every file");
    h[8..12].copy_from_slice(&(1u32 << 11).to_le_bytes());
    assert!(sig::applies_to(&h, Some(11)) && !sig::applies_to(&h, Some(18)), "a PE-only library is not tried on an ELF");
    h[16] = 1;
    assert!(!sig::applies_to(&h, Some(11)), "a startup signature names no function");
}

#[test]
fn crc_is_x25_byte_swapped() {
    assert_eq!(crc16(&[0x90; 4]), 0xbc16);
    assert_eq!(crc16(b"123456789"), 0x6e90);
}

#[test]
fn trie_crc_and_length_decide_the_match() {
    let s = tiny();
    let good = code();
    let m = candidates(&s, &good);
    assert_eq!(m.len(), 1);
    assert!(module_matches(&s, m[0], &good));
    let mut bad_crc = good.clone();
    bad_crc[33] = 0xcc;
    assert!(!module_matches(&s, m[0], &bad_crc));
    let mut bad_prefix = good.clone();
    bad_prefix[1] = 0x00;
    assert!(candidates(&s, &bad_prefix).is_empty());
    assert!(!module_matches(&s, m[0], &good[..36]), "shorter than the module");
    assert_eq!(function_name(m[0], s.pattern_len, false).as_deref(), Some("myfunc"));
}

/// `KUNA_FLIRT_PARSE_ALL=<dir>` parses every `.sig` under it.
#[test]
#[ignore]
fn parses_every_sig_under_a_directory() {
    let Ok(dir) = std::env::var("KUNA_FLIRT_PARSE_ALL") else { return };
    let mut stack = vec![std::path::PathBuf::from(dir)];
    let (mut ok, mut bad) = (0, 0);
    while let Some(d) = stack.pop() {
        for e in std::fs::read_dir(&d).unwrap().flatten() {
            let p = e.path();
            if p.is_dir() {
                stack.push(p);
            } else if p.extension().is_some_and(|x| x == "sig") {
                match parse(&std::fs::read(&p).unwrap()) {
                    Ok(s) => {
                        ok += 1;
                        println!("{} {} {}", p.display(), s.version, s.modules().len());
                    }
                    Err(e) => {
                        bad += 1;
                        println!("FAIL {}: {e}", p.display());
                    }
                }
            }
        }
    }
    println!("parsed {ok}, failed {bad}");
    assert_eq!(bad, 0);
}

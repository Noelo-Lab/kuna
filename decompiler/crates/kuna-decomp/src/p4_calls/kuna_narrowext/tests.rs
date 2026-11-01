use super::*;

#[test]
fn every_value_parses_and_others_are_refused() {
    for (text, mode) in [
        ("off", NarrowExtMode::Off),
        ("abi", NarrowExtMode::Abi),
        ("compiler", NarrowExtMode::Compiler),
    ] {
        let (parsed, _) = OptionNarrowExt.apply(text).unwrap();
        assert_eq!(parsed, mode);
        assert_eq!(parsed.as_str(), text);
    }
    assert!(OptionNarrowExt.apply("on").is_err());
    assert_eq!(NarrowExtMode::default(), NarrowExtMode::Abi);
}

#[test]
fn only_o32_n32_and_n64_state_the_argument_rule() {
    assert!(mips_documented("MIPS:LE:32:default:default"));
    assert!(mips_documented("MIPS:BE:64:default:default"));
    assert!(mips_documented("MIPS:LE:64:64-32addr:n32"));
    assert!(mips_documented("MIPS:LE:64:64-32addr:o32"));
    assert!(!mips_documented("MIPS:LE:64:64-32addr:default"));
    assert!(!mips_documented("MIPS:LE:64:64-32addr:o64"));
    assert!(!mips_documented("MIPS:LE:32:default:eabi"));
    assert!(!mips_documented("MIPS:LE:32:default:windows"));
    for variant in ["octeon", "octeonplus", "octeon2", "octeon3"] {
        for suffix in ["", "-32addr"] {
            for model in ["default", "hard"] {
                assert!(mips_documented(&format!("MIPS:BE:64:{variant}{suffix}:{model}")));
            }
            assert!(!mips_documented(&format!("MIPS:BE:64:{variant}{suffix}:unknown")));
            assert_eq!(
                mips_documented(&format!("MIPS:BE:64:{variant}{suffix}:n32")),
                suffix == "-32addr"
            );
        }
    }
}

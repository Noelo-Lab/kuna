use kuna_decomp::printc::{display_format, format_integer_token, resolve_integer_format};

fn signed_token(value: u64, bytes: i32, display: u32) -> String {
    let (negative, magnitude, display) =
        resolve_integer_format(value, bytes, true, display, false, false);
    format_integer_token(negative, magnitude, display, bytes, false, false, false, "")
}

#[test]
fn extended_signed_offsets_use_the_declared_width() {
    for bytes in 1..=8 {
        assert_eq!(
            signed_token((-4_i64) as u64, bytes, display_format::NONE),
            "-4",
            "{bytes}-byte offset"
        );
    }
}

#[test]
fn upper_bits_cannot_change_a_positive_signed_constant() {
    for bytes in 1..8 {
        let upper_bits = u64::MAX << (bytes * 8);
        assert_eq!(
            signed_token(upper_bits | 4, bytes, display_format::NONE),
            "4",
            "{bytes}-byte constant"
        );
    }
}

#[test]
fn signed_width_normalization_preserves_forced_radices() {
    for bytes in 1..=8 {
        assert_eq!(
            signed_token((-4_i64) as u64, bytes, display_format::FORCE_HEX),
            "-0x4"
        );
        assert_eq!(
            signed_token((-4_i64) as u64, bytes, display_format::FORCE_DEC),
            "-4"
        );
        assert_eq!(
            signed_token((-4_i64) as u64, bytes, display_format::FORCE_OCT),
            "-04"
        );
    }
}

#[test]
fn canonical_and_full_width_signed_values_keep_their_values() {
    assert_eq!(signed_token(0xffff_fffc, 4, display_format::NONE), "-4");
    assert_eq!(
        signed_token(0x7fff_ffff_ffff_ffff, 8, display_format::FORCE_HEX),
        "0x7fffffffffffffff"
    );
    assert_eq!(
        signed_token(0x8000_0000_0000_0000, 8, display_format::FORCE_HEX),
        "-0x8000000000000000"
    );
    assert_eq!(
        resolve_integer_format(0xffff_fffc, 4, false, 0, false, false).1,
        0xffff_fffc
    );
    assert_eq!(
        resolve_integer_format(0x41, 1, true, display_format::FORCE_CHAR, false, false).1,
        0x41
    );
}

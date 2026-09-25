use super::*;

#[test]
fn sign_extend_reads_a_negative_literal() {
    assert_eq!(sign_extend(0xff, 1), -1);
    assert_eq!(sign_extend(0xfffffffc, 4), -4);
    assert_eq!(sign_extend(0x10, 8), 0x10);
}

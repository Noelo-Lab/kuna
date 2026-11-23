// Tests for `kuna_acleintrinsics` (included into `mod tests` in
// kuna_acleintrinsics.rs). The rendered calls are pinned end to end by
// `tests/stages/kuna-acleintrinsics.xml`.

use super::*;

#[test]
fn barrier_operands_recombine_into_crm() {
    assert_eq!(crm(2, 3), Some(11));
    assert_eq!(crm(3, 3), Some(15));
    assert_eq!(crm(0, 0), Some(0));
    assert_eq!(crm(4, 0), None);
}

#[test]
fn isb_option_comes_from_the_encoding() {
    assert_eq!(isb_crm(0xd503_3fdf), Some(15));
    assert_eq!(isb_crm(0xd503_31df), Some(1));
    assert_eq!(isb_crm(0xd503_3fbf), None);
}

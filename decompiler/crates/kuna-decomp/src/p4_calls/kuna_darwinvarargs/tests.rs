use super::*;

#[test]
fn overlapping_storage_is_told_apart_from_its_neighbours() {
    assert!(overlaps((1, 0x4000, 8), (1, 0x4000, 4)));
    assert!(overlaps((1, 0x4000, 8), (1, 0x4004, 4)));
    assert!(!overlaps((1, 0x4000, 8), (1, 0x4008, 8)));
    assert!(!overlaps((1, 0x4000, 8), (2, 0x4000, 8)));
    assert!(!overlaps((1, u64::MAX - 1, 8), (1, 0, 8)));
}

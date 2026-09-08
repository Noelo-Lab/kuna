//! Unit tests for the `litpoolconst` foldable-range derivation.

use super::*;

const CODE_RO: u32 = section_flags::CODE | section_flags::READONLY;

#[test]
fn an_executable_read_only_section_is_foldable() {
    assert_eq!(code_const_ranges(&[(0x8000, 0x600, CODE_RO)], &[]), vec![(0x8000, 0x85ff)]);
}

#[test]
fn read_only_data_is_not_foldable() {
    let ro_data = section_flags::DATA | section_flags::READONLY;
    assert!(code_const_ranges(&[(0x9000, 0x100, ro_data)], &[]).is_empty());
}

#[test]
fn a_writable_code_section_is_not_foldable() {
    assert!(code_const_ranges(&[(0x8000, 0x100, section_flags::CODE)], &[]).is_empty());
}

#[test]
fn an_unallocated_or_uninitialized_section_is_not_foldable() {
    let noload = CODE_RO | section_flags::NOLOAD;
    let unalloc = CODE_RO | section_flags::UNALLOC;
    assert!(code_const_ranges(&[(0x8000, 0x100, noload)], &[]).is_empty());
    assert!(code_const_ranges(&[(0x8000, 0x100, unalloc)], &[]).is_empty());
}

#[test]
fn segments_answer_only_when_no_section_qualifies() {
    let sections = [(0x9000u64, 0x100u64, section_flags::DATA | section_flags::READONLY)];
    let segments = [(0x8000u64, 0x200u64, CODE_RO)];
    assert_eq!(code_const_ranges(&sections, &segments), vec![(0x8000, 0x81ff)]);
    // A section table that DOES carry executable read-only memory wins outright.
    let both = [(0x1000u64, 0x10u64, CODE_RO)];
    assert_eq!(code_const_ranges(&both, &segments), vec![(0x1000, 0x100f)]);
}

#[test]
fn adjacent_ranges_merge_and_sort() {
    let sections =
        [(0x8100, 0x100, CODE_RO), (0x8000, 0x100, CODE_RO), (0x9000, 0x10, CODE_RO)];
    assert_eq!(code_const_ranges(&sections, &[]), vec![(0x8000, 0x81ff), (0x9000, 0x900f)]);
}

#[test]
fn an_empty_image_contributes_nothing() {
    assert!(code_const_ranges(&[], &[]).is_empty());
    assert!(code_const_ranges(&[(0x8000, 0, CODE_RO)], &[]).is_empty());
}

#[test]
fn a_read_inside_a_range_is_constant() {
    let ranges = [(0x8000u64, 0x85ffu64)];
    assert!(contains(&ranges, 0x8458, 4));
    assert!(contains(&ranges, 0x8000, 1));
    assert!(contains(&ranges, 0x85fc, 4));
}

#[test]
fn a_read_that_leaves_the_range_is_not() {
    let ranges = [(0x8000u64, 0x85ffu64)];
    assert!(!contains(&ranges, 0x85fe, 4), "a word straddling the end of .text");
    assert!(!contains(&ranges, 0x7ffc, 4));
    assert!(!contains(&ranges, 0x8600, 4));
    assert!(!contains(&ranges, 0x8458, 0));
    assert!(!contains(&[], 0x8458, 4));
}

#[test]
fn a_read_between_two_ranges_is_not() {
    let ranges = [(0x1000u64, 0x1fffu64), (0x8000u64, 0x85ffu64)];
    assert!(contains(&ranges, 0x1ffc, 4));
    assert!(!contains(&ranges, 0x2000, 4));
    assert!(contains(&ranges, 0x8000, 4));
}

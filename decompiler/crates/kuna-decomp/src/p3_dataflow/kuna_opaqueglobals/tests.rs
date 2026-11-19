use super::*;

#[test]
fn a_read_is_clean_only_inside_one_section_and_clear_of_every_dirty_range() {
    let found = UnwrittenGlobals::new(
        vec![(0x2000, 0x2100, false), (0x1000, 0x1100, true)],
        vec![(0x1010, 0x1014), (0x1020, 0x1028), (0x1012, 0x1018), (0x2000, 0x2004)],
    );
    assert_eq!(found.dirty, vec![(0x1010, 0x1018), (0x1020, 0x1028), (0x2000, 0x2004)]);
    assert_eq!(found.initial(0x1000, 4), Some(true));
    assert_eq!(found.initial(0x100c, 4), Some(true));
    assert_eq!(found.initial(0x100d, 4), None);
    assert_eq!(found.initial(0x1014, 4), None);
    assert_eq!(found.initial(0x1018, 8), Some(true));
    assert_eq!(found.initial(0x1019, 8), None);
    assert_eq!(found.initial(0x1028, 4), Some(true));
    assert_eq!(found.initial(0x10fe, 4), None);
    assert_eq!(found.initial(0x2004, 4), Some(false));
    assert_eq!(found.initial(0x2002, 4), None);
    assert_eq!(found.initial(0x3000, 4), None);
    assert_eq!(found.initial(u64::MAX - 1, 4), None);
}

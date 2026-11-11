use super::own_rows;

const ENTRIES: [u64; 4] = [0x1000, 0x1100, 0x1200, 0x1300];

fn owner(addr: u64) -> Option<u64> {
    ENTRIES.iter().rev().copied().find(|&entry| entry <= addr)
}

#[test]
fn rows_running_on_into_two_neighbours_are_cut() {
    let rows = [0x1010, 0x1020, 0x1030, 0x1110, 0x1120, 0x1210];
    assert_eq!(own_rows(0x1000, &rows, owner), Some(3));
}

#[test]
fn rows_all_in_the_function_are_kept() {
    let rows = [0x1010, 0x1020, 0x10f0];
    assert_eq!(own_rows(0x1000, &rows, owner), None);
}

#[test]
fn rows_coming_back_into_the_function_are_kept() {
    let rows = [0x1010, 0x1110, 0x1210, 0x1020];
    assert_eq!(own_rows(0x1000, &rows, owner), None);
}

#[test]
fn rows_leaving_into_one_function_only_are_kept() {
    let rows = [0x1010, 0x1020, 0x1110, 0x1120, 0x1130];
    assert_eq!(own_rows(0x1000, &rows, owner), None);
}

#[test]
fn a_table_whose_first_row_is_outside_is_kept() {
    let rows = [0x1110, 0x1010, 0x1210];
    assert_eq!(own_rows(0x1000, &rows, owner), None);
}

#[test]
fn rows_before_the_first_known_entry_count_as_another_place() {
    let rows = [0x1010, 0x1110, 0x0800];
    assert_eq!(own_rows(0x1000, &rows, owner), Some(1));
}

#[test]
fn a_table_leaving_below_its_entry_is_kept() {
    let rows = [0x1210, 0x1220, 0x1010, 0x1310, 0x1110];
    assert_eq!(own_rows(0x1200, &rows, owner), None);
}

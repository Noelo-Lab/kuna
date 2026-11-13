use super::decode;

fn record(files: &[&str], code: &[u8]) -> Vec<u8> {
    let mut b = vec![1, files.len() as u8, 0, 0];
    b.extend(0x10001000u32.to_le_bytes());
    b.extend(0x1000u32.to_le_bytes());
    b.extend([0; 4]);
    for file in files {
        b.extend((file.len() as u16).to_le_bytes());
        b.extend(file.as_bytes());
    }
    b.extend(code);
    let size = b.len() as u32;
    b[12..16].copy_from_slice(&size.to_le_bytes());
    b
}
fn routine(code: &mut Vec<u8>, addr: u32, name: &str) {
    code.extend([2, 0]);
    code.extend((name.len() as u16).to_le_bytes());
    code.extend(addr.to_le_bytes());
    code.extend(name.as_bytes());
}

#[test]
fn file_selection_is_relative_and_intervals_end_inclusively() {
    let mut code = Vec::new();
    routine(&mut code, 0x10001000, "FIRST");
    code.extend([3, 0, 0, 0, 0, 0xc3, 10]);
    routine(&mut code, 0x10001004, "SECOND");
    code.extend([3, 0, 0, 0, 0, 0xc1, 10]);
    let t = decode(&record(&["first.for", "second.for"], &code));
    assert_eq!(t.routines.len(), 2);
    assert_eq!(t.lines.len(), 2);
    assert_eq!(
        (t.lines[0].start, t.lines[0].end, t.lines[0].line),
        (0x10001000, 0x10001004, 10)
    );
    assert_eq!(
        (t.lines[1].start, t.lines[1].end, t.lines[1].line),
        (0x10001005, 0x10001006, 20)
    );
    assert_eq!(&*t.lines[1].file, "second.for");
}

#[test]
fn signed_line_deltas_and_all_pc_widths() {
    let mut code = Vec::new();
    routine(&mut code, 0x10001000, "SAMPLE");
    code.extend([
        3, 0, 0, 0, 0, 4, 100, 7, 0, 5, 0xff, 0xff, 8, 1, 0, 6, 0xfe, 0xff, 0xff, 0xff, 9, 2, 0, 0,
        0, 13, 3, 0, 0, 0, 0x80, 0xc0, 0xff,
    ]);
    let t = decode(&record(&["sample.for"], &code));
    assert_eq!(
        t.lines.iter().map(|l| l.line).collect::<Vec<_>>(),
        [100, 99, 97, 97, 98, 97]
    );
    assert_eq!(t.lines.last().unwrap().end, 0x1000100c);
}

#[test]
fn malformed_records_never_read_the_next_record() {
    let mut code = Vec::new();
    routine(&mut code, 0x10001000, "SAFE");
    code.extend([3, 0, 0, 0, 0, 0x80]);
    let good = record(&["safe.for"], &code);
    for len in 0..good.len() {
        assert!(decode(&good[..len]).routines.is_empty());
    }
    let mut bad = record(&[], &[2, 0, 0xff, 0xff]);
    bad.extend(&good);
    assert_eq!(decode(&bad).routines[0].name, "SAFE");
    let mut bad = good.clone();
    bad[12..16].copy_from_slice(&0u32.to_le_bytes());
    assert!(decode(&bad).lines.is_empty());
}

#[test]
fn unknown_program_opcode_discards_its_record_and_incremental_is_skipped() {
    let mut code = Vec::new();
    routine(&mut code, 0x10001000, "SAFE");
    code.extend([3, 0, 0, 0, 0, 0x80]);
    let good = record(&["safe.for"], &code);
    let mut unknown = code.clone();
    unknown.push(0x7f);
    assert!(decode(&record(&["safe.for"], &unknown)).routines.is_empty());
    let mut incremental = good.clone();
    incremental[0] = 11;
    incremental.extend(&good);
    assert_eq!(decode(&incremental).routines.len(), 1);
}

#[test]
fn invalid_file_indices_and_address_overflow_are_declined() {
    let mut code = Vec::new();
    routine(&mut code, 0x10001000, "SAFE");
    code.extend([3, 1, 0, 0, 0, 0x80]);
    assert!(decode(&record(&["safe.for"], &code)).routines.is_empty());
    let mut b = record(&[], &[]);
    b[4..8].copy_from_slice(&u32::MAX.to_le_bytes());
    assert!(decode(&b).lines.is_empty());
}

#[test]
fn extended_routine_marker_and_padding() {
    let mut code = vec![12, 0, 0, 0, 0, 0];
    code.extend(4u16.to_le_bytes());
    code.extend(0x10001000u32.to_le_bytes());
    code.extend(b"TEST");
    code.extend([
        0, 10, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 3, 0, 0, 0, 0, 0x80,
    ]);
    let t = decode(&record(&["test.for"], &code));
    assert_eq!(t.routines[0].name, "TEST");
    assert_eq!(t.lines[0].line, 1);
}

#[test]
fn file_tables_share_one_budget_across_records() {
    let with_files = |count: u32| {
        let mut b = vec![1];
        b.extend(&count.to_le_bytes()[..3]);
        b.extend(0x10001000u32.to_le_bytes());
        b.extend(0x1000u32.to_le_bytes());
        b.extend([0; 4]);
        b.extend(std::iter::repeat_n(0u8, 2 * count as usize));
        routine(&mut b, 0x10001000, "SAFE");
        let size = b.len() as u32;
        b[12..16].copy_from_slice(&size.to_le_bytes());
        b
    };
    assert_eq!(decode(&with_files(65536)).routines.len(), 1);
    assert!(decode(&with_files(65537)).routines.is_empty());
    let mut two = with_files(40000);
    two.extend(with_files(40000));
    assert_eq!(decode(&two).routines.len(), 1);
}

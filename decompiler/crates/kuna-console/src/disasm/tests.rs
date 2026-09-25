use super::*;

fn row(addr: u64, bytes: &[u8], mnemonic: &str, operands: &str) -> Row {
    Row {
        addr,
        size: bytes.len() as u64,
        bytes: bytes.to_vec(),
        mnemonic: mnemonic.into(),
        operands: operands.into(),
    }
}

#[test]
fn instruction_text_carries_exactly_one_space() {
    assert_eq!(row(0, &[0x55], "PUSH", "RBP").text(), "PUSH RBP");
    assert_eq!(row(0, &[0xc3], "RET", "").text(), "RET");
}

/// A byte that will not decode only re-aligns the listing on an
/// architecture that has a grid to re-align to. `witness` is the OR of the
/// decoded rows' addresses and sizes.
/// The predicate the diagnostic rests on: an in-range target no row starts
/// at. A target that starts a row is fine, and one outside the listed span
/// says nothing about the span.
#[test]
fn a_skipped_target_is_one_inside_the_listing_that_no_row_starts_at() {
    let rows = vec![
        row(0x1000, &[0xeb, 0x04], "JMP", "0x1006"),
        row(0x1002, &[0x90], "NOP", ""),
        row(0x1003, &[0xc2, 0xac, 0xeb], "RET", "0xebac"),
        row(0x1006, &[0x90], "NOP", ""),
    ];
    let mut refs = FixedRefs::default();
    // 0x1004 is covered by the RET; 0x1006 starts a row; 0x2000 is out of
    // the span; 0x1000 is the listing's own first row.
    refs.flow_targets = vec![0x1004, 0x1006, 0x2000, 0x1000, 0x1004];
    assert_eq!(skipped_targets(&rows, &refs), vec![0x1004]);
    assert!(skipped_targets(&[], &refs).is_empty());
}

#[test]
fn the_resume_grid_is_the_alignment_the_decoded_rows_share() {
    // ARM: 4-byte rows on 4-byte addresses.
    assert_eq!(resume_grid(2, 0x5c0 | 0x5c4 | 4), 4);
    // Thumb: one 2-byte row is enough to say the grid is 2.
    assert_eq!(resume_grid(2, 0x5c0 | 0x5c2 | 2), 2);
    // A variable-length architecture keeps its byte-at-a-time recovery
    // however aligned the rows it happens to have decoded are.
    assert_eq!(resume_grid(1, 0x401000 | 4), 1);
    // Nothing decoded yet: no grid to infer.
    assert_eq!(resume_grid(4, 0), 1);
    // An odd row address says this listing is not on a grid at all.
    assert_eq!(resume_grid(2, 0x695 | 4), 1);
    // A short listing whose rows happen to share more alignment than any
    // instruction set has is clamped.
    assert_eq!(resume_grid(2, 0x10000), MAX_RESUME_GRID);
}

/// The row spans TO the boundary, so the next row starts on it — a
/// one-byte-at-a-time recovery would leave a row at the unaligned address.
#[test]
fn a_recovery_row_spans_to_the_next_grid_boundary() {
    assert_eq!(recovery_span(0x695, 4, None), 3);
    // Already on the grid: an undecodable slot is one instruction wide.
    assert_eq!(recovery_span(0x694, 4, None), 4);
    // No grid: one byte, which is what every variable-length listing gets.
    assert_eq!(recovery_span(0x694, 1, None), 1);
    // The region's end clips the span, and never to zero.
    assert_eq!(recovery_span(0x6a0, 4, Some(0x6a2)), 2);
    assert_eq!(recovery_span(0x6a0, 4, Some(0x6a4)), 4);
    assert_eq!(recovery_span(0x6a0, 4, Some(0x6a0)), 1);
}

#[test]
fn a_byte_row_spells_every_byte_it_covers() {
    assert_eq!(bad_byte_operand(&[0xb8]), "0xb8");
    assert_eq!(bad_byte_operand(&[0xb8, 0xfe, 0xff, 0xff]), "0xb8,0xfe,0xff,0xff");
}

#[test]
fn bytes_render_as_contiguous_lowercase_hex() {
    assert_eq!(row(0, &[0x48, 0x89, 0xe5], "MOV", "RBP,RSP").hex(), "4889e5");
}

/// The image bound is the end of the run holding the start, and adjacent or
/// overlapping segments are one run: a listing that crosses from one
/// `PT_LOAD` into the next at the byte the first ends has crossed no hole.
#[test]
fn the_image_bound_is_the_end_of_the_run_holding_the_address() {
    let code = section_flags::CODE;
    let data = section_flags::DATA;
    // The witness's own layout: R E [0x8048000,0x80d1904), a hole, then RW.
    let keygenme = [(0x8048000, 0x89904, code), (0x80d2f50, 0x7a9540, data)];
    assert_eq!(mapped_run(&keygenme, 0x80d18b0), Some((0x80d1904, Some(0x80d2f50))));
    assert_eq!(mapped_run(&keygenme, 0x80d2f50), Some((0x887c490, None)));
    // Inside the hole there is no run at all -- the direct query's answer.
    assert_eq!(mapped_run(&keygenme, 0x80d190b), None);
    assert_eq!(mapped_run(&keygenme, 0x8047fff), None);

    // Touching and overlapping segments merge, in either input order.
    let touching = [(0x2000, 0x1000, data), (0x1000, 0x1000, code)];
    assert_eq!(mapped_run(&touching, 0x1800), Some((0x3000, None)));
    let overlapping = [(0x1000, 0x1800, code), (0x2000, 0x1000, data)];
    assert_eq!(mapped_run(&overlapping, 0x1000), Some((0x3000, None)));

    // A zero-size record is not a segment (the `runModel` convention), and
    // a loader that publishes nothing at all is silence, not a bound.
    assert_eq!(mapped_run(&[(0x1000, 0, code)], 0x1000), None);
    assert_eq!(mapped_run(&[], 0x1000), None);
}

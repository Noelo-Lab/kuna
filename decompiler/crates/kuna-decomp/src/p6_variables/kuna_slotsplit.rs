//! (kuna `slotsplit`) A frame row a narrower printed declaration lives inside
//! is split at the declaration's bounds on the `variables` rows (P6).
//!
//! `framelayout` reports a frame slot at the width an early pass saw it, and a
//! span the early layout merged (several spills, or a buffer's tail) reaches
//! the rows as one `undefinedN`, `undefined16` or `undefined5`, even when the
//! printed C declares a local at a home strictly inside it:
//! `char *v3; // stack - 0x40` within `local_40 undefined16`.  Such a row is
//! cut at the homes of the declarations of [`DeclaredSlots`] lying wholly
//! inside it: each declared piece takes the type its declarations spell
//! ([`DeclaredSlots::piece_type`]), and each gap between them stays
//! `undefinedK` at its own offset and width.
//!
//! A row of a scalar's width (1, 2, 4 or 8 bytes) may be one value the type
//! system narrowed, so only a pointer or floating-point declaration splits it;
//! a row of any other width is never one scalar's slot and splits at every
//! declaration inside.  Nothing is split where two pieces would overlap; no
//! piece or gap starts at an offset another row, or a piece already cut from
//! another row, starts at, and a gap ends where such a row starts.  Only rows
//! still width-only (not typed by `slotreport` or `slotptr`) are split.  Every
//! piece after a row's first is marked `split_piece`, so the rows without the
//! mark are the rows reported before the split.
//!
//! Read from the printer after the `print_c` of the same function: nothing here
//! feeds the IR or the emitted C.

use std::collections::HashMap;

use crate::decompile_drive::VarInfo;
use crate::kuna_slotreport::DeclaredSlots;

/// Replace each row of `out` listed in `candidates` (width-only frame rows)
/// with its pieces, where declarations lie inside it.  The offsets the rows
/// start at, and those of every piece already cut, are taken: no piece or gap
/// starts at one, and a gap ends at the first one inside it, so two
/// overlapping rows never both report a piece at one offset and a gap never
/// runs over a row that starts inside it.
pub fn split_rows(slots: &DeclaredSlots, out: &mut Vec<VarInfo>, candidates: &[usize]) {
    let mut starts: HashMap<i64, usize> = HashMap::new();
    for off in out.iter().filter_map(|v| v.stack_offset) {
        *starts.entry(off).or_default() += 1;
    }
    let mut replaced: Vec<(usize, Vec<VarInfo>)> = Vec::new();
    for &i in candidates {
        let row = &out[i];
        let Some(lo) = row.stack_offset else { continue };
        let hi = lo + row.size;
        let taken = |x: i64| starts.get(&x).copied().unwrap_or(0) > usize::from(x == lo);
        let committed_only = matches!(row.size, 1 | 2 | 4 | 8);
        let pieces: Vec<(i64, i64, String)> = slots
            .homes_within(lo, hi)
            .into_iter()
            .filter(|&(off, _)| !taken(off))
            .filter_map(|(off, size)| slots.piece_type(off, size, committed_only).map(|t| (off, size, t)))
            .collect();
        if pieces.is_empty() || pieces.windows(2).any(|p| p[0].0 + p[0].1 > p[1].0) {
            continue;
        }
        let gap = |from: i64, to: i64| {
            let end = starts.keys().copied().filter(|&x| from < x && x < to).min().unwrap_or(to);
            (!taken(from)).then(|| piece(row, from, end - from, format!("undefined{}", end - from)))
        };
        let mut rows = Vec::new();
        let mut cur = lo;
        for (off, size, type_name) in pieces {
            if off > cur {
                rows.extend(gap(cur, off));
            }
            rows.push(piece(row, off, size, type_name));
            cur = off + size;
        }
        if cur < hi {
            rows.extend(gap(cur, hi));
        }
        for (n, r) in rows.iter_mut().enumerate() {
            r.split_piece = n > 0;
            if let Some(off) = r.stack_offset.filter(|&off| off != lo) {
                *starts.entry(off).or_default() += 1;
            }
        }
        replaced.push((i, rows));
    }
    for (i, rows) in replaced.into_iter().rev() {
        out.splice(i..=i, rows);
    }
}

/// The piece of frame row `row` at `off`, `size` bytes wide: the row's own name
/// where it starts the row, else the frame's name for that offset.
fn piece(row: &VarInfo, off: i64, size: i64, type_name: String) -> VarInfo {
    let name = if Some(off) == row.stack_offset {
        row.name.clone()
    } else if off < 0 {
        format!("local_{:x}", -off)
    } else {
        format!("stack_{off:x}")
    };
    VarInfo {
        name,
        type_name,
        stack_offset: Some(off),
        size,
        is_param: false,
        arg_index: None,
        line_numbers: Vec::new(),
        addresses: Vec::new(),
        split_piece: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn frame_row(off: i64, size: i64) -> VarInfo {
        VarInfo {
            name: format!("local_{:x}", -off),
            type_name: format!("undefined{size}"),
            stack_offset: Some(off),
            size,
            is_param: false,
            arg_index: None,
            line_numbers: Vec::new(),
            addresses: Vec::new(),
            split_piece: false,
        }
    }

    fn split(slots: &DeclaredSlots, rows: Vec<VarInfo>) -> Vec<(String, String, i64, i64)> {
        let mut out = rows;
        let candidates: Vec<usize> = (0..out.len()).collect();
        split_rows(slots, &mut out, &candidates);
        out.into_iter().map(|v| (v.name, v.type_name, v.stack_offset.unwrap(), v.size)).collect()
    }

    fn row(name: &str, ty: &str, off: i64, size: i64) -> (String, String, i64, i64) {
        (name.to_string(), ty.to_string(), off, size)
    }

    #[test]
    fn a_pointer_at_the_start_of_a_span_splits_it() {
        let slots = DeclaredSlots::for_test(&[((-0x40, 8), &[("v3", Some("char *"), true)])]);
        assert_eq!(
            split(&slots, vec![frame_row(-0x40, 16)]),
            [row("local_40", "char *", -0x40, 8), row("local_38", "undefined8", -0x38, 8)]
        );
    }

    #[test]
    fn declarations_inside_an_odd_width_row_split_it_with_gaps() {
        let slots = DeclaredSlots::for_test(&[
            ((-0x10, 4), &[("v1", Some("int"), false)]),
            ((-0xc, 1), &[("v2", None, false)]),
        ]);
        assert_eq!(
            split(&slots, vec![frame_row(-0x12, 7)]),
            [
                row("local_12", "undefined2", -0x12, 2),
                row("local_10", "int", -0x10, 4),
                row("local_c", "undefined1", -0xc, 1),
            ]
        );
    }

    #[test]
    fn a_scalar_width_row_splits_only_at_a_pointer_or_float() {
        let int = DeclaredSlots::for_test(&[((-0x10, 4), &[("v1", Some("int"), false)])]);
        assert_eq!(split(&int, vec![frame_row(-0x10, 8)]), [row("local_10", "undefined8", -0x10, 8)]);
        let float = DeclaredSlots::for_test(&[((-0xc, 4), &[("v1", Some("float"), true)])]);
        assert_eq!(
            split(&float, vec![frame_row(-0x10, 8)]),
            [row("local_10", "undefined4", -0x10, 4), row("local_c", "float", -0xc, 4)]
        );
    }

    #[test]
    fn declarations_that_disagree_leave_their_piece_width_only() {
        let slots = DeclaredSlots::for_test(&[((-0x20, 8), &[("v1", Some("long"), false), ("v2", Some("char *"), true)])]);
        assert_eq!(
            split(&slots, vec![frame_row(-0x20, 24)]),
            [row("local_20", "undefined8", -0x20, 8), row("local_18", "undefined16", -0x18, 16)]
        );
    }

    #[test]
    fn overlapping_declarations_leave_the_row_whole() {
        let slots = DeclaredSlots::for_test(&[
            ((-0x20, 8), &[("v1", Some("char *"), true)]),
            ((-0x1c, 8), &[("v2", Some("long"), false)]),
        ]);
        assert_eq!(split(&slots, vec![frame_row(-0x20, 24)]), [row("local_20", "undefined24", -0x20, 24)]);
    }

    #[test]
    fn no_piece_starts_where_another_row_does() {
        let slots = DeclaredSlots::for_test(&[((-0x18, 8), &[("v1", Some("char *"), true)])]);
        let mut symbol = frame_row(-0x18, 8);
        symbol.name = "v1".to_string();
        symbol.type_name = "char *".to_string();
        assert_eq!(
            split(&slots, vec![frame_row(-0x20, 24), symbol]),
            [row("local_20", "undefined24", -0x20, 24), row("v1", "char *", -0x18, 8)]
        );
    }

    #[test]
    fn overlapping_rows_never_report_a_piece_twice() {
        let slots = DeclaredSlots::for_test(&[
            ((-0xa8, 4), &[("v10", Some("void *"), true)]),
            ((-0xa4, 4), &[("v9", Some("unsigned int"), false), ("v12", Some("char *"), true)]),
        ]);
        let mut out = vec![frame_row(-0xb0, 40), frame_row(-0xac, 4), frame_row(-0xa8, 32)];
        split_rows(&slots, &mut out, &[0, 1, 2]);
        let rows: Vec<_> = out.iter().map(|v| (v.name.as_str(), v.type_name.as_str(), v.stack_offset.unwrap(), v.size)).collect();
        assert_eq!(
            rows,
            [
                ("local_b0", "undefined4", -0xb0, 4),
                ("local_a4", "undefined4", -0xa4, 4),
                ("local_a0", "undefined24", -0xa0, 24),
                ("local_ac", "undefined4", -0xac, 4),
                ("local_a8", "void *", -0xa8, 4),
            ]
        );
    }

    #[test]
    fn the_pieces_after_the_first_are_marked() {
        let slots = DeclaredSlots::for_test(&[((-0x18, 8), &[("v1", Some("double"), true)])]);
        let mut out = vec![frame_row(-0x20, 4), frame_row(-0x18, 12)];
        split_rows(&slots, &mut out, &[0, 1]);
        let marks: Vec<_> = out.iter().map(|v| (v.stack_offset.unwrap(), v.split_piece)).collect();
        assert_eq!(marks, [(-0x20, false), (-0x18, false), (-0x10, true)]);
        assert_eq!(out.iter().filter(|v| !v.split_piece).count(), 2);
    }
}

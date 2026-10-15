//! Avoid four-byte refinement pieces that partially overlap known inputs.

use kuna_base::types::int4;

/// Kuna variant of upstream `Heritage::remove13Refinement`: preserve a 1+3 or
/// 3+1 junction only when joining it would straddle a known input.
pub(crate) fn coalesce(
    refine: &mut [int4],
    mut straddles_input: impl FnMut(usize, usize, usize) -> bool,
) {
    if refine.is_empty() {
        return;
    }
    let mut last = refine[0];
    let mut pos = last as usize;
    while pos < refine.len() {
        let current = refine[pos];
        if current == 0 {
            break;
        }
        let start = pos - last as usize;
        let end = pos + current as usize;
        if matches!((last, current), (1, 3) | (3, 1))
            && !straddles_input(start, pos, end)
        {
            refine[start] = 4;
            last = 4;
            pos += current as usize;
        } else {
            last = current;
            pos += last as usize;
        }
    }
}

pub(crate) fn span_straddles_join(
    span: (usize, usize),
    start: usize,
    junction: usize,
    end: usize,
) -> bool {
    (span.0 == junction && span.1 > end) || (span.1 == junction && span.0 < start)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn partitions(refine: &[int4]) -> Vec<(usize, int4)> {
        let mut result = Vec::new();
        let mut pos = 0;
        while pos < refine.len() {
            let size = refine[pos];
            assert!(size > 0);
            result.push((pos, size));
            pos += size as usize;
        }
        assert_eq!(pos, refine.len());
        result
    }

    #[test]
    fn known_argument_boundary_survives_mixed_byte_halfword_word_reads() {
        let mut refine = [2, 0, 1, 1, 3, 0, 0, 1];
        coalesce(&mut refine, |start, junction, end| {
            [(0, 4), (4, 8)]
                .into_iter()
                .any(|span| span_straddles_join(span, start, junction, end))
        });
        assert_eq!(partitions(&refine), [(0, 2), (2, 1), (3, 1), (4, 4)]);
    }

    #[test]
    fn both_slice_orders_preserve_a_straddling_input_boundary() {
        for (mut refine, span) in [
            ([2, 0, 1, 3, 0, 0, 2, 0], (3, 8)),
            ([2, 0, 3, 0, 0, 1, 2, 0], (0, 5)),
        ] {
            let before = partitions(&refine);
            coalesce(&mut refine, |start, junction, end| {
                span_straddles_join(span, start, junction, end)
            });
            assert_eq!(partitions(&refine), before);
        }
    }

    #[test]
    fn contained_narrow_input_keeps_legacy_join() {
        for (mut refine, span) in [([1, 3, 0, 0], (0, 1)), ([3, 0, 0, 1], (3, 4))] {
            coalesce(&mut refine, |start, junction, end| {
                span_straddles_join(span, start, junction, end)
            });
            assert_eq!(partitions(&refine), [(0, 4)]);
        }
    }

    #[test]
    fn ordinary_slices_retain_legacy_coalescing() {
        for mut refine in [[1, 3, 0, 0], [3, 0, 0, 1]] {
            coalesce(&mut refine, |_, _, _| false);
            assert_eq!(partitions(&refine), [(0, 4)]);
        }
    }
}

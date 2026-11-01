//! Split each Varnode that runs past the top of its address space at the wrap.
//!
//! Address arithmetic wraps modulo the size of the space, so an 8-byte load
//! from ram 0xfffffffffffffffb, or a stack access that starts below the entry
//! stack pointer and ends above it, names bytes at both ends of the space.
//! Heritage keys its range maps by start offset and treats a Varnode's bytes as
//! one run upward from that offset. A range that spills past the top never
//! merges with the ranges at the bottom of the space, so the cover stops being
//! disjoint: a partition refined out of the wrapping range lies over a range at
//! offset 0 and collects that range's Varnodes, which run past the partition's
//! end (`build_refinement` indexes past its array), and renaming asks for
//! inputs that overlap ("Overlapping input varnodes"). Where neither fails, a
//! write through the wrapping range never reaches a read of the low bytes.
//!
//! Before heritage walks a space, every wrapping read, write and input in it is
//! split into the bytes up to the top of the space and the bytes from offset 0,
//! the way a refinement splits a Varnode: a `PIECE` of the two pieces for a
//! read, `SUBPIECE`s of the written value for a write or an input. That is the
//! same value byte for byte, and no Varnode heritage sees wraps.

use kuna_base::types::int4;

/// The number of bytes of `[offset, offset + size)` up to and including the
/// space's `highest` offset, when the range runs past it.
pub(crate) fn wrap_cut(offset: u64, size: int4, highest: u64) -> Option<int4> {
    let room = highest.checked_sub(offset)?.checked_add(1)?;
    (size > 1 && room < size as u64).then_some(room as int4)
}

/// The refinement array that splits a `size`-byte Varnode after `cut` bytes,
/// in the form `Heritage::refinement` hands to the refine helpers.
pub(crate) fn wrap_refinement(size: int4, cut: int4) -> Vec<int4> {
    let mut refine = vec![0; size as usize];
    refine[0] = cut;
    refine[cut as usize] = size - cut;
    refine
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cut_at_the_top_of_a_64_bit_space() {
        assert_eq!(wrap_cut(0xffff_ffff_ffff_fffb, 8, u64::MAX), Some(5));
        assert_eq!(wrap_cut(0xffff_ffff_ffff_fffc, 16, u64::MAX), Some(4));
        assert_eq!(wrap_cut(0xffff_ffff_ffff_fff8, 8, u64::MAX), None);
        assert_eq!(wrap_cut(0, 8, u64::MAX), None);
        assert_eq!(wrap_cut(u64::MAX, 1, u64::MAX), None);
        assert_eq!(wrap_cut(u64::MAX, 2, u64::MAX), Some(1));
    }

    #[test]
    fn cut_at_the_top_of_a_32_bit_space() {
        assert_eq!(wrap_cut(0xffff_fffe, 4, 0xffff_ffff), Some(2));
        assert_eq!(wrap_cut(0xffff_fffc, 4, 0xffff_ffff), None);
        assert_eq!(wrap_cut(0x1_0000_0000, 4, 0xffff_ffff), None);
    }

    #[test]
    fn refinement_marks_both_pieces() {
        assert_eq!(wrap_refinement(8, 5), vec![5, 0, 0, 0, 0, 3, 0, 0]);
        assert_eq!(wrap_refinement(2, 1), vec![1, 1]);
    }
}

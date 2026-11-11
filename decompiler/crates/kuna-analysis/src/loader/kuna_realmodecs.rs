//! (kuna #1288) The x86 real-mode code segment in the decode context.
//!
//! `x86-16.sla` keeps the real-mode code segment in the context variable
//! `realCS` (default 0). A near branch decodes inside the 64 KiB segment that
//! starts at `realCS * 16` and passes `realCS` on to its target, a far
//! `CALL`/`JMP ptr16:16` states its target's segment, and a near `CALL` pushes
//! the IP inside the segment. An instruction outside the 64 KiB above
//! `realCS * 16` cannot be in that segment, so its branch is the linear sum of
//! its next address and displacement instead. A value set at an address holds
//! up to the next address it was set at.
//!
//! A loader states the segments it knows by painting `(linear, CS)` points
//! with [`paint`]: a raw image that one segment holds is one point
//! ([`raw_image_cs`]), and a loader that knows more (an MS-DOS executable's
//! entry `CS:IP`) paints its own points after it. Every other language has no
//! `realCS`, and painting it is a silent no-op there.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::AddrSpace;
use kuna_decomp::architecture::Architecture;
use kuna_sleigh::translate::Translate;

/// The SLEIGH context variable holding the real-mode code segment.
pub const REAL_CS: &str = "realCS";

/// The code segment of a raw real-mode image of `size` bytes mapped at `base`,
/// when one segment holds the whole image.
///
/// An image the first 64 KiB holds runs with CS 0, the way a `.COM` program at
/// `0x100` or a boot sector at `0x7c00` is addressed, so IP is the address
/// itself. Another image that fits in 64 KiB is a segment of its own that
/// starts at its base paragraph (a ROM at `0xf0000` is `F000:0000`). A larger
/// image has several segments whose bases it does not state, so it has none:
/// its code keeps the default CS 0, under which a near branch past the first
/// 64 KiB is the linear sum.
pub fn raw_image_cs(base: u64, size: u64) -> Option<u16> {
    let end = base.checked_add(size)?;
    if end <= 0x1_0000 {
        return Some(0);
    }
    let cs = u16::try_from(base >> 4).ok()?;
    (end <= (u64::from(cs) << 4) + 0x1_0000).then_some(cs)
}

/// Paint each `(linear, cs)` point: the code segment is `cs` from `linear` up
/// to the next point that sets it. Returns whether the language has the
/// real-mode code segment.
pub fn paint(arch: &Architecture, space: &Rc<AddrSpace>, points: &[(u64, u16)]) -> bool {
    if arch
        .with_context_db(|db| db.get_variable(REAL_CS.as_bytes()))
        .is_err()
    {
        return false;
    }
    arch.with_context_db_mut(|db| {
        for &(linear, cs) in points {
            let at = Address::new(Rc::clone(space), linear);
            let _ = db.set_variable(REAL_CS.as_bytes(), &at, u32::from(cs));
        }
    });
    true
}

/// Run `f` with `realCS` commits masked off.
///
/// A linear sweep decodes data as code, and a far branch decoded from data
/// would otherwise state a segment at an arbitrary address and change how the
/// code after that address decodes.
pub fn without_commits<T>(
    arch: &Architecture,
    translate: &dyn Translate,
    f: impl FnOnce() -> T,
) -> T {
    let Ok(var) = arch.with_context_db(|db| db.get_variable(REAL_CS.as_bytes())) else {
        return f();
    };
    let word = var.get_word() as usize;
    let saved = translate.set_context_write_mask(word, u32::MAX);
    translate.set_context_write_mask(word, saved & !(var.get_mask() << var.get_shift()));
    let out = f();
    translate.set_context_write_mask(word, saved);
    out
}

#[cfg(test)]
mod tests {
    use super::raw_image_cs;

    #[test]
    fn a_raw_image_in_the_first_64k_runs_with_cs_zero() {
        assert_eq!(raw_image_cs(0x100, 0x4), Some(0));
        assert_eq!(raw_image_cs(0x7c00, 0x200), Some(0));
        assert_eq!(raw_image_cs(0, 0x1_0000), Some(0));
    }

    #[test]
    fn a_raw_image_one_segment_holds_starts_it_at_its_base() {
        assert_eq!(raw_image_cs(0xf_0000, 0x1_0000), Some(0xf000));
        assert_eq!(raw_image_cs(0x100, 0x1_0000), Some(0x10));
        assert_eq!(raw_image_cs(0x2_0008, 0xfff8), Some(0x2000));
    }

    #[test]
    fn a_larger_raw_image_states_no_segment() {
        assert_eq!(raw_image_cs(0x1_0000, 0x2_e2e8), None);
        assert_eq!(raw_image_cs(0x2_0008, 0xfff9), None);
        assert_eq!(raw_image_cs(0x1234_5670, 0x10), None);
    }
}

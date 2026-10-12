//! Retry exact switch-guard intersections after the candidate domain narrows.

use crate::context::VarnodeId;
use crate::funcdata::Funcdata;
use crate::jumptable::GuardRecord;
use crate::rangeutil::CircleRange;

pub(crate) fn intersect(
    fd: &Funcdata,
    guards: &[GuardRecord],
    vn: VarnodeId,
    range: &mut CircleRange,
) {
    let mut bits_preserved = 0;
    let base_vn = GuardRecord::quasi_copy(fd, vn, &mut bits_preserved);
    for guard in guards {
        if guard.value_match(fd, vn, base_vn, bits_preserved) != 0 {
            range.intersect(guard.get_range());
        }
    }
}

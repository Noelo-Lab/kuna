//! Existing Thumb-only inventory recovery when validated armframes is disabled.

use super::*;

pub(super) fn seeds(
    file: &object::File,
    listing: &Listing,
    translate: &dyn Translate,
    code_space: Rc<AddrSpace>,
    exec_ranges: &[(u64, u64)],
) -> Vec<u64> {
    use object::read::Object;
    if file.architecture() != object::Architecture::Arm {
        return Vec::new();
    }
    let mut decoder = GapDecoder::new(translate, code_space, exec_ranges);
    let mut accepted = BTreeSet::new();
    let mut claimed = BTreeSet::new();
    for (sec_addr, _, data) in crate::entry::executable_sections(file) {
        let mut off = (sec_addr as usize) & 1;
        while off + 1 < data.len() {
            if is_thumb_lr_prologue(&data, off) {
                let vma = sec_addr + off as u64;
                if listing.is_undefined(vma) && !claimed.contains(&vma) {
                    let gap_hi = listing.next_instruction_start_after(vma).unwrap_or(u64::MAX);
                    if let Some(body) =
                        check_valid_subroutine(&mut decoder, listing, vma, vma, gap_hi)
                    {
                        accepted.insert(vma);
                        claimed.extend(body);
                    }
                }
            }
            off += 2;
        }
    }
    accepted.into_iter().collect()
}

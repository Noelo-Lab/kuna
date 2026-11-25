//! Match the constant setup that starts a large SPARC register-window frame.
use super::{ClaimedCode, FlowKind, GapDecoder, Listing};

pub(super) fn prefix(
    decoder: &mut GapDecoder,
    listing: &Listing,
    claimed: &ClaimedCode,
    entry: u64,
) -> Option<u64> {
    if entry & 3 != 0 || !decoder.translate.is_big_endian() {
        return None;
    }
    let save = decoder.probe(entry)?;
    if save.len != 4 || save.mnemonic.as_ref() != "save"
        || decoder.translate.get_register("g1").ok()?.size != 4
    {
        return None;
    }
    let start = entry.checked_sub(8)?;
    let end = entry.checked_add(4)?;
    if !decoder.exec_ranges.iter().any(|&(lo, hi)| lo <= start && end <= hi)
        || !listing.is_undefined(start) || !listing.is_undefined(start + 4)
        || claimed.overlaps(start, start) || claimed.overlaps(start + 4, start + 4)
        || listing.function_at(start).is_some() || listing.function_at(start + 4).is_some()
    {
        return None;
    }
    let sethi = decoder.probe(start)?;
    let or = decoder.probe(start + 4)?;
    if sethi.len != 4 || or.len != 4
        || sethi.kind != FlowKind::Fallthrough || or.kind != FlowKind::Fallthrough
        || sethi.mnemonic.as_ref() != "sethi" || or.mnemonic.as_ref() != "or"
        || sethi.fall_through != Some(start + 4) || or.fall_through != Some(entry)
    {
        return None;
    }
    matches(&sethi.operands, &or.operands, &save.operands).then_some(start)
}

pub(super) fn is_v9(decoder: &GapDecoder) -> bool {
    decoder.translate.is_big_endian()
        && decoder.translate.get_register("g1").is_ok_and(|reg| reg.size == 8)
}

pub(super) fn v9_entry(decoder: &mut GapDecoder, start: u64, gap_end: u64) -> bool {
    if start & 3 != 0 || start.checked_add(12).is_none_or(|end| end > gap_end) {
        return false;
    }
    let Some(hi) = decoder.probe(start) else { return false };
    if hi.mnemonic.as_ref() != "sethi" || hi.len != 4 || hi.fall_through != Some(start + 4) {
        return false;
    }
    let Some(xor) = decoder.probe(start + 4) else { return false };
    if xor.mnemonic.as_ref() != "xor" || xor.len != 4 || xor.fall_through != Some(start + 8) {
        return false;
    }
    let Some(save) = decoder.probe(start + 8) else { return false };
    save.mnemonic.as_ref() == "save" && save.len == 4
        && matches_v9(&hi.operands, &xor.operands, &save.operands)
}

fn operands<const N: usize>(text: &str) -> Option<[&str; N]> {
    let mut parts = text.split(',').map(str::trim);
    let mut result = [""; N];
    for part in &mut result { *part = parts.next()?; }
    parts.next().is_none().then_some(result)
}

fn hex(text: &str) -> Option<u32> {
    u32::from_str_radix(text.strip_prefix("0x")?, 16).ok()
}

fn matches(sethi: &str, or: &str, save: &str) -> bool {
    let Some([sp, reg, dest]) = operands(save) else { return false };
    let Some([high, setreg]) = operands(sethi) else { return false };
    let Some([source, low, orreg]) = operands(or) else { return false };
    if sp != "sp" || dest != "sp" || reg == "g0" || reg == "sp"
        || setreg != reg || source != reg || orreg != reg
    {
        return false;
    }
    let Some(high) = high.strip_prefix("%hi(").and_then(|s| s.strip_suffix(')')).and_then(hex)
        else { return false };
    let Some(low) = hex(low) else { return false };
    let frame = (high | low) as i32;
    high & 0x3ff == 0 && low < 0x400 && frame < -4096 && frame & 7 == 0
}

fn matches_v9(sethi: &str, xor: &str, save: &str) -> bool {
    let Some([high, reg]) = operands(sethi) else { return false };
    let Some([source, low, dest]) = operands(xor) else { return false };
    let Some(["sp", frame_reg, "sp"]) = operands(save) else { return false };
    if reg == "g0" || reg == "sp" || source != reg || dest != reg || frame_reg != reg {
        return false;
    }
    let Some(high) = high.strip_prefix("%hi(").and_then(|s| s.strip_suffix(')')).and_then(hex)
        else { return false };
    let Some(low) = low.strip_prefix('-').and_then(hex).filter(|n| (1..=0x400).contains(n))
        else { return false };
    let frame = i64::from(high) ^ -i64::from(low);
    high & 0x3ff == 0 && frame < -4096 && frame & 15 == 0
}

#[cfg(test)]
#[test]
fn v9_large_frame_requires_signed_aligned_size_and_one_temporary() {
    for (high, low) in [("0x2000", "-0xd0"), ("0x40000", "-0x130")] {
        assert!(matches_v9(&format!("%hi({high}),g1"), &format!("g1,{low},g1"), "sp,g1,sp"));
    }
    for (high, xor, save) in [
        ("0x2000", "g1,0xd0,g1", "sp,g1,sp"),
        ("0x2000", "g1,-0xd8,g1", "sp,g1,sp"),
        ("0x0", "g1,-0xd0,g1", "sp,g1,sp"),
        ("0x2000", "g1,-0xd0,g2", "sp,g1,sp"),
        ("0x2000", "g1,-0xd0,g1", "sp,g2,sp"),
    ] {
        assert!(!matches_v9(&format!("%hi({high}),g1"), xor, save), "{high}; {xor}; {save}");
    }
}

#[cfg(test)]
mod tests {
    use super::matches;

    #[test]
    fn matched_registers_own_the_native_prefix() {
        for reg in ["g1", "g3", "o0"] {
            assert!(matches(&format!("%hi(0xffffdc00),{reg}"),
                &format!("{reg},0x388,{reg}"), &format!("sp,{reg},sp")));
        }
        for (high, low) in [
            ("0xffffdc00", "0x2b8"), ("0xffffdc00", "0x288"),
            ("0xffffdc00", "0x388"), ("0xffffdc00", "0x250"),
            ("0xffffbc00", "0x320"), ("0xffffec00", "0x3f8"),
        ] {
            assert!(matches(&format!("%hi({high}),g1"),
                &format!("g1,{low},g1"), "sp,g1,sp"));
        }
    }

    #[test]
    fn unrelated_constants_and_mismatched_operands_are_not_prefixes() {
        for (sethi, or, save) in [
            ("%hi(0xffffdc00),g1", "g1,0x388,g2", "sp,g1,sp"),
            ("%hi(0xffffdc00),g1", "g2,0x388,g1", "sp,g1,sp"),
            ("%hi(0xffffdc00),g1", "g1,0x388,g1", "sp,g2,sp"),
            ("%hi(0xffffdc00),g1", "g1,0x388,g1", "sp,0x1,sp"),
            ("%hi(0x2000),g1", "g1,0x388,g1", "sp,g1,sp"),
            ("%hi(0xfffffc00),g1", "g1,0x388,g1", "sp,g1,sp"),
            ("%hi(0xffffdc00),g1", "g1,0x788,g1", "sp,g1,sp"),
            ("%hi(0xfffff000),g1", "g1,0x0,g1", "sp,g1,sp"),
            ("%hi(0xffffdc00),g1", "g1,0x400,g1", "sp,g1,sp"),
            ("%hi(0xffffdc00),g1", "g1,0x389,g1", "sp,g1,sp"),
            ("%hi(0xffffdc00),g0", "g0,0x388,g0", "sp,g0,sp"),
            ("%hi(0xffffdc00),sp", "sp,0x388,sp", "sp,sp,sp"),
        ] {
            assert!(!matches(sethi, or, save), "{sethi}; {or}; {save}");
        }
    }
}

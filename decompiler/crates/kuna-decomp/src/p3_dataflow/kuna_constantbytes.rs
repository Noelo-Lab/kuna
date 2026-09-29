//! Extract byte ranges from the bounded payloads of wide constants.

use kuna_base::address::calc_mask;

pub(super) fn word(value: u64, offset: i32, size: i32) -> u64 {
    if !(0..8).contains(&offset) || size <= 0 {
        return 0;
    }
    (value >> (offset * 8)) & calc_mask(size)
}

pub(super) fn pair(high: u64, low: u64, low_bytes: i32, offset: i32, size: i32) -> u64 {
    if offset < 0 || size <= 0 {
        return 0;
    }
    if offset >= low_bytes {
        return word(high, offset - low_bytes, size);
    }
    let count = size.min(low_bytes - offset);
    let lower = word(low, offset, count);
    if count >= size || count >= 8 {
        return lower;
    }
    lower | (word(high, 0, size - count) << (count * 8))
}

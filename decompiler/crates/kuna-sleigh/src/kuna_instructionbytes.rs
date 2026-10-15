//! Packed instruction reads with the parser's zero-filled buffer tail.

#[inline]
pub(crate) fn read_bytes(buf: &[u8; 16], offset: usize, size: i32) -> u32 {
    if offset <= 12 && (1..=4).contains(&size) {
        let word = u32::from_be_bytes(buf[offset..offset + 4].try_into().unwrap());
        return word >> (8 * (4 - size));
    }
    let mut result = 0u32;
    for i in 0..size {
        result = (result << 8) | u32::from(buf.get(offset + i as usize).copied().unwrap_or(0));
    }
    result
}

#[inline]
pub(crate) fn read_bits(buf: &[u8; 16], offset: usize, bit: i32, size: i32) -> u32 {
    if offset <= 12 && bit >= 0 && (1..=32).contains(&size) && bit + size <= 32 {
        let word = u32::from_be_bytes(buf[offset..offset + 4].try_into().unwrap());
        return word
            .wrapping_shl(bit as u32)
            .wrapping_shr((32 - size) as u32);
    }
    let bytesize = (bit + size - 1) / 8 + 1;
    read_bytes(buf, offset, bytesize)
        .wrapping_shl((8 * (4 - bytesize) + bit) as u32)
        .wrapping_shr((32 - size) as u32)
}

#[inline]
pub(crate) fn little_endian(value: i64, size: i32) -> i64 {
    match size {
        1 => i64::from(value as u8),
        2 => i64::from((value as u16).swap_bytes()),
        3 => i64::from((value as u32).swap_bytes() >> 8),
        4 => i64::from((value as u32).swap_bytes()),
        5..=8 => ((value as u64).swap_bytes() >> (8 * (8 - size))) as i64,
        _ => {
            let mut value = value;
            kuna_base::address::byte_swap_inplace(&mut value, size);
            value
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BYTES: [u8; 16] = [
        0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0, 0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54, 0x32,
        0x10,
    ];

    #[test]
    fn packed_reads_and_zero_filled_tails() {
        assert_eq!(read_bytes(&BYTES, 0, 1), 0x12);
        assert_eq!(read_bytes(&BYTES, 1, 2), 0x3456);
        assert_eq!(read_bytes(&BYTES, 2, 3), 0x56789a);
        assert_eq!(read_bytes(&BYTES, 3, 4), 0x789abcde);
        assert_eq!(read_bytes(&BYTES, 12, 4), 0x76543210);
        assert_eq!(read_bytes(&BYTES, 13, 4), 0x54321000);
        assert_eq!(read_bytes(&BYTES, 14, 4), 0x32100000);
        assert_eq!(read_bytes(&BYTES, 15, 4), 0x10000000);
        assert_eq!(read_bytes(&BYTES, 0, 5), 0x3456789a);
        assert_eq!(read_bytes(&BYTES, 15, 0), 0);
        assert_eq!(read_bytes(&BYTES, 15, -1), 0);
    }

    #[test]
    fn bit_reads_keep_masked_shifts_and_tail_padding() {
        assert_eq!(read_bits(&BYTES, 0, 0, 32), 0x12345678);
        assert_eq!(read_bits(&BYTES, 0, 4, 12), 0x234);
        assert_eq!(read_bits(&BYTES, 0, 7, 25), 0x345678);
        assert_eq!(read_bits(&BYTES, 13, 0, 32), 0x54321000);
        assert_eq!(read_bits(&BYTES, 15, 4, 16), 0);
        assert_eq!(read_bits(&BYTES, 0, 1, 32), 0x34000000);
        assert_eq!(read_bits(&BYTES, 0, 0, 0), 0x12000000);
    }

    #[test]
    fn little_endian_fields_keep_only_the_requested_bytes() {
        for size in -1..=10 {
            for value in [0, 0x123456789abcdef0, -1, i64::MIN] {
                let mut expected = value;
                kuna_base::address::byte_swap_inplace(&mut expected, size);
                assert_eq!(little_endian(value, size), expected);
            }
        }
    }
}

//! Port of `org/vineflower/kotlin/metadata/BitEncoding.java`.

use crate::rt::{throw, JavaStr};

pub struct BitEncoding;

impl BitEncoding {
    const _8TO7_MODE_MARKER: u16 = 0xFFFF;

    fn add_modulo_byte(data: &mut [u8], increment: i32) {
        for b in data.iter_mut() {
            *b = (((*b as i8 as i32) + increment) & 0x7f) as u8;
        }
    }

    /// `decodeBytes(String[])`; each `char` is cast to a byte.
    pub fn decode_bytes(data: &[String]) -> Vec<u8> {
        let mut data: Vec<Vec<u16>> = data.iter().map(|s| s.utf16()).collect();
        if !data.is_empty() && !data[0].is_empty() {
            let possible_marker = data[0][0];
            if possible_marker == 0 {
                return BitEncoding::strings_to_bytes(&BitEncoding::drop_marker(&data));
            }
            if possible_marker == BitEncoding::_8TO7_MODE_MARKER {
                data = BitEncoding::drop_marker(&data);
            }
        }

        let mut bytes = BitEncoding::combine_string_array_into_bytes(&data);
        BitEncoding::add_modulo_byte(&mut bytes, 0x7f);
        BitEncoding::decode7to8(&bytes)
    }

    fn strings_to_bytes(data: &[Vec<u16>]) -> Vec<u8> {
        data.iter().flatten().map(|&c| c as u8).collect()
    }

    fn drop_marker(data: &[Vec<u16>]) -> Vec<Vec<u16>> {
        let mut result = data.to_vec();
        result[0].remove(0);
        result
    }

    fn combine_string_array_into_bytes(data: &[Vec<u16>]) -> Vec<u8> {
        BitEncoding::strings_to_bytes(data)
    }

    fn decode7to8(data: &[u8]) -> Vec<u8> {
        let result_length = 7i32.wrapping_mul(data.len() as i32) / 8;
        if result_length < 0 {
            throw("NegativeArraySizeException", result_length.to_string());
        }
        let mut result = vec![0u8; result_length as usize];

        let at = |i: i32| -> i32 {
            if i < 0 || i as usize >= data.len() {
                throw(
                    "ArrayIndexOutOfBoundsException",
                    format!("Index {} out of bounds for length {}", i, data.len()),
                );
            }
            data[i as usize] as i8 as i32
        };

        let mut byte_index = 0;
        let mut bit = 0;
        for slot in result.iter_mut() {
            let first_part = ((at(byte_index) & 0xff) as u32 >> bit) as i32;
            byte_index += 1;
            let second_part = (at(byte_index) & ((1 << (bit + 1)) - 1)) << (7 - bit);
            *slot = (first_part + second_part) as u8;

            if bit == 6 {
                byte_index += 1;
                bit = 0;
            } else {
                bit += 1;
            }
        }

        result
    }
}

//! (kuna `bytestorewidth`) Keep a constant integer write into a byte array at its
//! native width.
//!
//! Upstream `SplitDatatype` (`RuleSplitCopy`, `RuleSplitStore`) splits a constant
//! COPY or STORE into one write per component of its destination type. For a
//! character array that is the point: GCC's inlined `memcpy(buf, "abc", 4)` is one
//! `str w1,[x0]` that prints as `buf[0] = 'a'; ... buf[3] = '\0';`, and the
//! stack-string recovery joins such pieces into `builtin_strncpy`. A 4-byte
//! integer that lands in a byte array prints the same way, as four assignments of
//! its bytes (`v1[1].preceding[0] = 3; v1[1].preceding[1] = 0; ...`), and the one
//! native access and its width disappear. That happens through a record view whose
//! array covers the field, and on a stack record whose leading `char` array a
//! `strncpy` destination typed and whose extent then covers the integer fields
//! after it.
//!
//! With the option on, a constant write stays whole when its destination range lies
//! inside one array of one-byte integer or character elements, its bytes are not
//! all equal, and they do not read as text: one is a control character other than
//! NUL, tab, newline and carriage return (`3` is `03 00 00 00`), or a byte above
//! 0x7f is no part of UTF-8 (`0xff`, `0x8000` as `00 80 00 00`, `-2`). A UTF-8
//! character cut at either end of the write still reads as text, since a string
//! is copied in chunks of any alignment. The constant is the value itself, or the
//! `INT_ZEXT` of a constant or `PIECE` of two constants that `SplitDatatype` also
//! splits into constant pieces, and its bytes are read in memory order. A zero or
//! any other one-byte fill still splits, because `memsetrecover` joins those
//! pieces, and so does a write into distinct one-byte record fields, which each
//! have a name of their own.
//!
//! Byte content alone cannot separate an integer from a string chunk: `100` and the
//! tail `"d\0\0\0"` are the same bytes. So the rule is opt-in. With it on, an
//! integer that reads as text (`100`, `0x20`, `0x80` as the tail of a UTF-8
//! character) still splits, and a string chunk that holds an escape such as `\x1b`
//! or a Latin-1 byte stays whole, which keeps the stack-string recovery from
//! joining that string.

use std::rc::Rc;

use kuna_base::address::calc_mask;
use kuna_num::opcodes::OpCode;

use super::kuna_constantbytes::pair;
use crate::context::VarnodeId;
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;

/// Should the constant `vn` holds, written to a destination of type `dest` in a
/// space of the given byte order, stay one write instead of being split?
pub(super) fn keeps_constant_whole(data: &Funcdata, vn: VarnodeId, dest: &Rc<Datatype>, big_endian: bool) -> bool {
    constant_value(data, vn).is_some_and(|(value, size)| keeps_whole(dest, value, size, big_endian))
}

/// The value and size of `vn` when it is a constant, or an `INT_ZEXT` of a
/// constant or a `PIECE` of two constants no wider than eight bytes.
pub(super) fn constant_value(data: &Funcdata, vn: VarnodeId) -> Option<(u64, i32)> {
    let v = data.vbank().get(vn)?;
    let size = v.get_size();
    if v.is_constant() {
        return Some((v.get_offset(), size));
    }
    if !v.is_written() || size > 8 {
        return None;
    }
    let op = data.obank().get(v.get_def()?)?;
    let input = |slot| op.get_in(slot).and_then(|i| data.vbank().get(i)).filter(|i| i.is_constant());
    match op.code() {
        OpCode::CPUI_INT_ZEXT => input(0).map(|low| (low.get_offset() & calc_mask(low.get_size()), size)),
        OpCode::CPUI_PIECE => {
            let (high, low) = (input(0)?, input(1)?);
            Some((pair(high.get_offset(), low.get_offset(), low.get_size(), 0, size), size))
        }
        _ => None,
    }
}

fn keeps_whole(dest: &Rc<Datatype>, value: u64, size: i32, big_endian: bool) -> bool {
    if !(2..=8).contains(&size) || !inside_one_byte_array(dest, size) {
        return false;
    }
    let mut bytes: Vec<u8> = (0..size).map(|i| (value >> (i * 8)) as u8).collect();
    if big_endian {
        bytes.reverse();
    }
    !bytes.iter().all(|&b| b == bytes[0]) && !reads_as_text(&bytes)
}

/// Does `[0, size)` of `dest` fall inside a single array whose elements are
/// one-byte integers or characters? A partial destination is read at its offset in
/// the whole type.
fn inside_one_byte_array(dest: &Rc<Datatype>, size: i32) -> bool {
    let (mut cur, mut off) = match (dest.get_partial_base(), dest.get_partial_offset()) {
        (Some(base), Some(offset)) => (base, offset as i64),
        _ => (Rc::clone(dest), 0),
    };
    for _ in 0..16 {
        if cur.get_metatype() == type_metatype::TYPE_ARRAY {
            if let Some(element) = cur.get_array_base() {
                if element.get_size() == 1 {
                    let integer = matches!(element.get_metatype(), type_metatype::TYPE_INT | type_metatype::TYPE_UINT);
                    return integer && off >= 0 && off + size as i64 <= cur.get_size() as i64;
                }
            }
        }
        match cur.get_sub_type(off) {
            Ok((Some(sub), next)) => {
                cur = sub;
                off = next;
            }
            _ => return false,
        }
    }
    false
}

/// No byte is a control character other than NUL, tab, newline and carriage
/// return, and the bytes above 0x7f form UTF-8, allowing a character cut at
/// either end.
fn reads_as_text(bytes: &[u8]) -> bool {
    let continuation = |b: u8| (0x80..0xc0).contains(&b);
    let mut i = bytes.iter().take(3).take_while(|&&b| continuation(b)).count();
    while i < bytes.len() {
        let b = bytes[i];
        let follow = match b {
            0x20..=0x7e | 0 | b'\t' | b'\n' | b'\r' => 0,
            0xc2..=0xdf => 1,
            0xe0..=0xef => 2,
            0xf0..=0xf4 => 3,
            _ => return false,
        };
        let end = (i + 1 + follow).min(bytes.len());
        if !bytes[i + 1..end].iter().all(|&c| continuation(c)) {
            return false;
        }
        i = end;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dtype::{TypeFactory, TypeFactoryImpl};

    fn factory() -> TypeFactoryImpl {
        let types = TypeFactoryImpl::new();
        types.set_default_alignment_map();
        types.set_max_basetype_size(8);
        types.set_core_type("undefined", 1, type_metatype::TYPE_UNKNOWN, false).unwrap();
        types.cache_core_types().unwrap();
        types
    }

    #[test]
    fn text_has_no_control_bytes_but_nul_and_whitespace() {
        assert!(reads_as_text(b"abc\0"));
        assert!(reads_as_text(b"a\tb\n"));
        assert!(reads_as_text(b"rld\0goo"));
        assert!(reads_as_text(b"lid\xc2\xa3"));
        assert!(!reads_as_text(b"\x03\0\0\0"));
        assert!(!reads_as_text(b"\x01\x10\0\0"));
        assert!(!reads_as_text(b"ab\x7fc"));
        assert!(reads_as_text(b"\xa3ab\xe2\x82"));
        assert!(reads_as_text(b"\xc3\xa9\xc3\xa9"));
        assert!(!reads_as_text(b"\xff\0\0\0"));
        assert!(!reads_as_text(b"\0\x80\0\0"));
        assert!(!reads_as_text(b"\xfe\xff\xff\xff"));
        assert!(!reads_as_text(b"\xc3a\0\0"));
        assert!(!reads_as_text(b"caf\xe9 ok"));
        assert!(reads_as_text(b"\x80\0\0\0"));
    }

    #[test]
    fn only_a_range_inside_one_integer_byte_array_qualifies() {
        let types = factory();
        let byte = types.get_base(1, type_metatype::TYPE_UINT).unwrap();
        let bytes8 = types.get_type_array(8, byte).unwrap();
        assert!(inside_one_byte_array(&bytes8, 4));
        assert!(inside_one_byte_array(&bytes8, 8));
        let bytes4 = types.get_type_array(4, types.get_base(1, type_metatype::TYPE_UINT).unwrap()).unwrap();
        assert!(!inside_one_byte_array(&bytes4, 8));
        let inner = types.get_type_partial_struct(bytes8.clone(), 2, 4).unwrap();
        assert!(inside_one_byte_array(&inner, 4));
        let tail = types.get_type_partial_struct(bytes8.clone(), 6, 2).unwrap();
        assert!(!inside_one_byte_array(&tail, 4));
        let flags = types.get_type_array(8, types.get_base(1, type_metatype::TYPE_BOOL).unwrap()).unwrap();
        assert!(!inside_one_byte_array(&flags, 4));
        let word = types.get_base(4, type_metatype::TYPE_UINT).unwrap();
        assert!(!inside_one_byte_array(&word, 4));
    }

    #[test]
    fn integers_stay_whole_and_text_fills_and_zero_split() {
        let types = factory();
        let bytes8 = types.get_type_array(8, types.get_base(1, type_metatype::TYPE_UINT).unwrap()).unwrap();
        assert!(keeps_whole(&bytes8, 3, 4, false));
        assert!(keeps_whole(&bytes8, 0x1001, 4, false));
        assert!(keeps_whole(&bytes8, 0x8000, 4, false));
        assert!(keeps_whole(&bytes8, 0xffff_fffe, 4, false));
        assert!(!keeps_whole(&bytes8, 0, 4, false));
        assert!(!keeps_whole(&bytes8, 0x0101_0101, 4, false));
        assert!(!keeps_whole(&bytes8, 0x0063_6261, 4, false));
        assert!(!keeps_whole(&bytes8, 3, 1, false));
        assert!(!keeps_whole(&bytes8, 100, 4, false));
        assert!(keeps_whole(&bytes8, 0xe282, 4, false));
        assert!(!keeps_whole(&bytes8, 0xe282, 4, true));
    }
}

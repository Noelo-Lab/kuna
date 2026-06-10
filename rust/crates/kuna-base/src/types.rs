//! Port of `decompiler/cpp/types.h` (W1).
//!
//! The C++ tree computes on fixed-width typedefs (`uintb`, `int4`, ...) and
//! relies on C++'s defined unsigned wraparound.  Per ADR 0003 the port maps
//! those typedefs onto Rust's primitive integers and makes every legitimately
//! wrapping operation explicit through the [`Wrap`] helper trait.
//!
//! Canonical width mapping (use the Rust primitive directly in ported code;
//! the aliases below exist as a checked, greppable record of the mapping):
//!
//! | C++ typedef | underlying        | Rust    |
//! |-------------|-------------------|---------|
//! | `uintb`     | `uint64_t`        | `u64`   |
//! | `intb`      | `int64_t`         | `i64`   |
//! | `uint8`     | `uint64_t` (!)    | `u64`   |
//! | `int8`      | `int64_t` (!)     | `i64`   |
//! | `uint4`     | `uint32_t`        | `u32`   |
//! | `int4`      | `int32_t`         | `i32`   |
//! | `uint2`     | `uint16_t`        | `u16`   |
//! | `int2`      | `int16_t`         | `i16`   |
//! | `uint1`     | `uint8_t`         | `u8`    |
//! | `int1`      | `int8_t`          | `i8`    |
//! | `uintm`     | `uint32_t`        | `u32`   |
//! | `intm`      | `int32_t`         | `i32`   |
//! | `uintp`     | `uintptr_t`       | `usize` |
//!
//! NOTE the C++ names `uint8`/`int8` are **64-bit** ("8 bytes", not 8 bits).

/// C++ `uintb` ("unsigned big integer", `uint64_t`).
#[allow(non_camel_case_types)]
pub type uintb = u64;
/// C++ `intb` ("signed big integer", `int64_t`).
#[allow(non_camel_case_types)]
pub type intb = i64;
/// C++ `uint8` — **64-bit** (`uint64_t`); the digit counts bytes, not bits.
#[allow(non_camel_case_types)]
pub type uint8 = u64;
/// C++ `int8` — **64-bit** (`int64_t`); the digit counts bytes, not bits.
#[allow(non_camel_case_types)]
pub type int8 = i64;
/// C++ `uint4` (`uint32_t`).
#[allow(non_camel_case_types)]
pub type uint4 = u32;
/// C++ `int4` (`int32_t`).
#[allow(non_camel_case_types)]
pub type int4 = i32;
/// C++ `uint2` (`uint16_t`).
#[allow(non_camel_case_types)]
pub type uint2 = u16;
/// C++ `int2` (`int16_t`).
#[allow(non_camel_case_types)]
pub type int2 = i16;
/// C++ `uint1` (`uint8_t`).
#[allow(non_camel_case_types)]
pub type uint1 = u8;
/// C++ `int1` (`int8_t`).
#[allow(non_camel_case_types)]
pub type int1 = i8;
/// C++ `uintm` (deprecated upstream; pinned to 32 bits, `uint32_t`).
#[allow(non_camel_case_types)]
pub type uintm = u32;
/// C++ `intm` (deprecated upstream; pinned to 32 bits, `int32_t`).
#[allow(non_camel_case_types)]
pub type intm = i32;
/// C++ `uintp` (`uintptr_t` — unsigned integer the size of a pointer).
#[allow(non_camel_case_types)]
pub type uintp = usize;

/// Port of the C++ `HOST_ENDIAN` macro: 0 on a little-endian host, 1 on a
/// big-endian host.  (Upstream hard-codes 0 on x86 and otherwise inspects
/// `part[3]` of a union over `int4 whole = 1`, which is 1 exactly on
/// big-endian hosts.)
pub const HOST_ENDIAN: i32 = if cfg!(target_endian = "big") { 1 } else { 0 };

/// Explicit wrapping arithmetic for ported C++ integer expressions (ADR 0003).
///
/// Any C++ expression whose operands can legitimately wrap is transcribed with
/// these helpers, never bare operators; bare `+`/`-`/`*` remain only where
/// overflow would be a genuine bug.  This makes wraparound greppable intent
/// and keeps debug and release builds computing identically.
///
/// Shift semantics: [`Wrap::wshl`]/[`Wrap::wshr`] take the shift count modulo
/// the bit width (`wrapping_shl`/`wrapping_shr`), which matches the x86
/// hardware behavior the C++ oracle binary exhibits for out-of-range shift
/// counts.  `wshr` is a logical shift on unsigned types and an arithmetic
/// shift on signed types, exactly like C++ `>>` on the corresponding typedef.
///
/// Division: `wdiv`/`wrem` still panic on a zero divisor (C++ UB, treated as
/// an internal invariant violation per ADR 0004); `i64::MIN / -1` wraps
/// instead of trapping.
pub trait Wrap: Copy {
    /// Wrapping addition (`a + b` on a C++ unsigned, or intended overflow).
    fn wadd(self, rhs: Self) -> Self;
    /// Wrapping subtraction.
    fn wsub(self, rhs: Self) -> Self;
    /// Wrapping multiplication.
    fn wmul(self, rhs: Self) -> Self;
    /// Wrapping negation (`-a`; e.g. two's-complement of an unsigned).
    fn wneg(self) -> Self;
    /// Wrapping division. Panics on zero divisor.
    fn wdiv(self, rhs: Self) -> Self;
    /// Wrapping remainder. Panics on zero divisor.
    fn wrem(self, rhs: Self) -> Self;
    /// Wrapping left shift; count taken modulo the bit width (x86 semantics).
    fn wshl(self, n: u32) -> Self;
    /// Wrapping right shift; count taken modulo the bit width (x86 semantics).
    /// Logical for unsigned types, arithmetic for signed types.
    fn wshr(self, n: u32) -> Self;
}

macro_rules! impl_wrap {
    ($($t:ty),*) => {$(
        impl Wrap for $t {
            #[inline]
            fn wadd(self, rhs: Self) -> Self { self.wrapping_add(rhs) }
            #[inline]
            fn wsub(self, rhs: Self) -> Self { self.wrapping_sub(rhs) }
            #[inline]
            fn wmul(self, rhs: Self) -> Self { self.wrapping_mul(rhs) }
            #[inline]
            fn wneg(self) -> Self { self.wrapping_neg() }
            #[inline]
            fn wdiv(self, rhs: Self) -> Self { self.wrapping_div(rhs) }
            #[inline]
            fn wrem(self, rhs: Self) -> Self { self.wrapping_rem(rhs) }
            #[inline]
            fn wshl(self, n: u32) -> Self { self.wrapping_shl(n) }
            #[inline]
            fn wshr(self, n: u32) -> Self { self.wrapping_shr(n) }
        }
    )*};
}

impl_wrap!(u8, u16, u32, u64, usize, i8, i16, i32, i64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrap_unsigned_wraparound() {
        assert_eq!(u64::MAX.wadd(1), 0);
        assert_eq!(0u64.wsub(1), u64::MAX);
        assert_eq!(0x8000_0000_0000_0000u64.wmul(2), 0);
        assert_eq!(1u64.wneg(), u64::MAX);
        assert_eq!(u32::MAX.wadd(2), 1);
        assert_eq!(0u32.wsub(1), u32::MAX);
    }

    #[test]
    fn test_wrap_signed_wraparound() {
        assert_eq!(i64::MAX.wadd(1), i64::MIN);
        assert_eq!(i64::MIN.wsub(1), i64::MAX);
        assert_eq!(i64::MIN.wneg(), i64::MIN); // two's-complement edge
        assert_eq!(i64::MIN.wdiv(-1), i64::MIN); // wraps instead of trapping
        assert_eq!(i32::MIN.wmul(-1), i32::MIN);
    }

    #[test]
    fn test_wrap_shift_counts_mask_like_x86() {
        // Count is taken modulo the bit width, matching x86 shifts.
        assert_eq!(1u64.wshl(64), 1);
        assert_eq!(1u64.wshl(65), 2);
        assert_eq!(0x8000_0000u32.wshr(32), 0x8000_0000);
        // Arithmetic shift on signed types.
        assert_eq!((-8i64).wshr(1), -4);
        // Logical shift on unsigned types.
        assert_eq!(0x8000_0000_0000_0000u64.wshr(63), 1);
    }

    #[test]
    fn test_host_endian_value() {
        if cfg!(target_endian = "big") {
            assert_eq!(HOST_ENDIAN, 1);
        } else {
            assert_eq!(HOST_ENDIAN, 0);
        }
    }
}

//! Static integer format specifications in the verified Rust 1.93 template layout.

use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trait {
    Display,
    Debug,
    LowerHex,
    UpperHex,
    Octal,
    Binary,
    LowerExp,
    UpperExp,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Formatter {
    pub primitive: &'static str,
    pub kind: Trait,
}

/// Conflicting full identities, including unknown aliases, invalidate an address.
pub fn formatter_symbols(
    symbols: impl IntoIterator<Item = (u64, Option<String>)>,
) -> BTreeMap<u64, Formatter> {
    let mut entries = BTreeMap::new();
    for (address, name) in symbols {
        let formatter = name.as_deref().and_then(classify);
        entries
            .entry(address)
            .and_modify(|old| {
                if *old != formatter {
                    *old = None;
                }
            })
            .or_insert(formatter);
    }
    entries
        .into_iter()
        .filter_map(|(address, formatter)| formatter.map(|formatter| (address, formatter)))
        .collect()
}

fn classify(name: &str) -> Option<Formatter> {
    let (name, imp) =
        if let Some(name) = name.strip_prefix("core::fmt::num::imp::<impl core::fmt::") {
            (name, true)
        } else {
            (
                name.strip_prefix("core::fmt::num::<impl core::fmt::")?,
                false,
            )
        };
    let (kind, primitive) = name.strip_suffix(">::fmt")?.split_once(" for ")?;
    let kind = match (kind, imp) {
        ("Display", true) => Trait::Display,
        ("LowerExp", true) => Trait::LowerExp,
        ("UpperExp", true) => Trait::UpperExp,
        ("Debug", false) => Trait::Debug,
        ("LowerHex", false) => Trait::LowerHex,
        ("UpperHex", false) => Trait::UpperHex,
        ("Octal", false) => Trait::Octal,
        ("Binary", false) => Trait::Binary,
        _ => return None,
    };
    let primitive = match primitive {
        "u8" => "u8",
        "u16" => "u16",
        "u32" => "u32",
        "u64" | "usize" => "u64",
        "i8" => "i8",
        "i16" => "i16",
        "i32" => "i32",
        "i64" | "isize" => "i64",
        _ => return None,
    };
    Some(Formatter { primitive, kind })
}

const DEFAULT_FLAGS: u32 = 0x6000_0020;
const WIDTH: u32 = 1 << 27;
const PRECISION: u32 = 1 << 28;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Options {
    flags: u32,
    width: u16,
    precision: u16,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            flags: DEFAULT_FLAGS,
            width: 0,
            precision: 0,
        }
    }
}

impl Options {
    pub(crate) fn decode(head: u8, mut take: impl FnMut(usize) -> Option<Vec<u8>>) -> Option<Self> {
        if head & 0xc0 != 0xc0 || head & 0x30 != 0 {
            return None;
        }
        let flags = if head & 1 != 0 {
            u32::from_le_bytes(take(4)?.try_into().ok()?)
        } else {
            DEFAULT_FLAGS
        };
        let fill = char::from_u32(flags & 0x1f_ffff)?;
        let align = (flags >> 29) & 3;
        if flags & 0x8000_0000 != 0
            || flags & (3 << 21) == 3 << 21
            || flags & (3 << 25) == 3 << 25
            || matches!(fill, '{' | '}')
            || (align == 3 && fill != ' ')
            || (head & 2 != 0 && flags & WIDTH == 0)
            || (head & 4 != 0 && flags & PRECISION == 0)
        {
            return None;
        }
        let width = if head & 2 != 0 {
            u16::from_le_bytes(take(2)?.try_into().ok()?)
        } else {
            0
        };
        let precision = if head & 4 != 0 {
            u16::from_le_bytes(take(2)?.try_into().ok()?)
        } else {
            0
        };
        if flags & WIDTH != 0 && width == 0 && flags & (1 << 24) == 0 {
            return None;
        }
        Some(Self {
            flags,
            width,
            precision,
        })
    }

    pub(crate) fn render(self, kind: Trait) -> Option<String> {
        let flags = self.flags;
        let mut spec = String::new();
        let align = (flags >> 29) & 3;
        if align != 3 {
            let fill = char::from_u32(flags & 0x1f_ffff)?;
            if fill != ' ' {
                spec.extend(fill.escape_debug());
            }
            spec.push(['<', '>', '^'][align as usize]);
        }
        if flags & (1 << 21) != 0 {
            spec.push('+');
        } else if flags & (1 << 22) != 0 {
            spec.push('-');
        }
        if flags & (1 << 23) != 0 {
            spec.push('#');
        }
        if flags & (1 << 24) != 0 {
            spec.push('0');
        }
        if flags & WIDTH != 0 {
            spec.push_str(&self.width.to_string());
        }
        if flags & PRECISION != 0 {
            spec.push('.');
            spec.push_str(&self.precision.to_string());
        }
        let debug_hex = (flags >> 25) & 3;
        if debug_hex != 0 && kind != Trait::Debug {
            return None;
        }
        spec.push_str(match kind {
            Trait::Display => "",
            Trait::Debug => match debug_hex {
                0 => "?",
                1 => "x?",
                2 => "X?",
                _ => return None,
            },
            Trait::LowerHex => "x",
            Trait::UpperHex => "X",
            Trait::Octal => "o",
            Trait::Binary => "b",
            Trait::LowerExp => "e",
            Trait::UpperExp => "E",
        });
        Some(spec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn options(head: u8, flags: u32, tail: &[u8]) -> Option<Options> {
        let mut bytes = flags.to_le_bytes().to_vec();
        bytes.extend_from_slice(tail);
        let mut bytes = bytes.as_slice();
        Options::decode(head, |n| {
            let value = bytes.get(..n)?.to_vec();
            bytes = &bytes[n..];
            Some(value)
        })
    }

    #[test]
    fn full_integer_traits_keep_width_signedness_and_alias_conflicts() {
        let symbol = |kind, ty| {
            let imp = if matches!(kind, "Display" | "LowerExp" | "UpperExp") {
                "imp::"
            } else {
                ""
            };
            format!("core::fmt::num::{imp}<impl core::fmt::{kind} for {ty}>::fmt")
        };
        for kind in [
            "Display", "Debug", "LowerHex", "UpperHex", "Octal", "Binary", "LowerExp", "UpperExp",
        ] {
            for ty in ["u8", "u16", "u32", "u64", "i8", "i16", "i32", "i64"] {
                assert_eq!(classify(&symbol(kind, ty)).unwrap().primitive, ty);
            }
            for ty in ["u128", "i128", "f32", "f64", "&u32", "bool", "char"] {
                assert!(classify(&symbol(kind, ty)).is_none());
            }
        }
        assert_eq!(
            formatter_symbols([
                (1, Some(symbol("LowerHex", "usize"))),
                (1, Some(symbol("LowerHex", "u64"))),
                (2, Some(symbol("Display", "u32"))),
                (2, Some(symbol("Debug", "u32"))),
                (3, Some(symbol("Octal", "i16"))),
                (3, Some(symbol("Octal", "u16"))),
                (4, Some(symbol("Binary", "u64"))),
                (4, None),
                (4, Some(symbol("Binary", "u64"))),
            ]),
            BTreeMap::from([(
                1,
                Formatter {
                    primitive: "u64",
                    kind: Trait::LowerHex
                }
            )])
        );
        for name in [
            "fmt",
            "core::fmt::num::u32::fmt",
            "core::fmt::num::imp::<impl core::fmt::Debug for u32>::fmt",
            "<T as core::fmt::Debug>::fmt",
        ] {
            assert!(classify(name).is_none());
        }
    }

    #[test]
    fn static_fields_render_exact_specs_and_zero_defaults() {
        assert_eq!(
            options(0xc3, 0x6900_0020, &[8, 0])
                .unwrap()
                .render(Trait::LowerHex)
                .as_deref(),
            Some("08x")
        );
        assert_eq!(
            options(0xc3, 0x28a0_0020, &[7, 0])
                .unwrap()
                .render(Trait::Display)
                .as_deref(),
            Some(">+#7")
        );
        assert_eq!(
            options(0xc7, 0x5801_f980, &[10, 0, 3, 0])
                .unwrap()
                .render(Trait::Display)
                .as_deref(),
            Some("🦀^10.3")
        );
        assert_eq!(
            options(0xc1, 0x7280_0020, &[])
                .unwrap()
                .render(Trait::Debug)
                .as_deref(),
            Some("#.0x?")
        );
        assert_eq!(
            options(0xc1, 0x6900_0020, &[])
                .unwrap()
                .render(Trait::Display)
                .as_deref(),
            Some("00")
        );
        assert_eq!(
            options(0xc1, 0x6280_0020, &[])
                .unwrap()
                .render(Trait::Display),
            None
        );
        for (kind, spec) in [
            (Trait::Display, ""),
            (Trait::Debug, "?"),
            (Trait::LowerHex, "x"),
            (Trait::UpperHex, "X"),
            (Trait::Octal, "o"),
            (Trait::Binary, "b"),
            (Trait::LowerExp, "e"),
            (Trait::UpperExp, "E"),
        ] {
            assert_eq!(Options::default().render(kind).as_deref(), Some(spec));
        }
    }

    #[test]
    fn invalid_or_dynamic_fields_are_refused() {
        for (head, flags, tail) in [
            (0xc1, 0x8000_0020, &[][..]),
            (0xc1, 0x6000_d800, &[][..]),
            (0xc1, 0x6011_0000, &[][..]),
            (0xc1, 0x6000_007b, &[][..]),
            (0xc1, 0x0000_007d, &[][..]),
            (0xc1, 0x6000_002e, &[][..]),
            (0xc1, 0x6060_0020, &[][..]),
            (0xc1, 0x6600_0020, &[][..]),
            (0xc3, DEFAULT_FLAGS, &[8, 0][..]),
            (0xc5, DEFAULT_FLAGS, &[3, 0][..]),
            (0xc3, DEFAULT_FLAGS | WIDTH, &[8][..]),
            (0xc5, DEFAULT_FLAGS | PRECISION, &[3][..]),
            (0xc1, DEFAULT_FLAGS | WIDTH, &[][..]),
        ] {
            assert!(options(head, flags, tail).is_none(), "{head:#x} {flags:#x}");
        }
        for head in [0, 0x80, 0xd0, 0xe0, 0xff] {
            assert!(Options::decode(head, |_| panic!("must reject before reading")).is_none());
        }
        for n in 0..4 {
            assert!(Options::decode(0xc1, |_| Some(vec![0; n])).is_none());
        }
    }
}

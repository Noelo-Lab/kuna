//! Probe strings at referenced addresses using the eager pass's matching rules.

use object::{Object, ObjectSection};

use crate::image_bytes::ImageBytes;
use kuna_decomp::kuna_lazystrings::Fact;

use super::{is_loaded_initialized, is_string_char, kuna_ptrslot::PointerSlots};

#[derive(Debug)]
pub struct Plan {
    min_len: usize,
    wide_min_len: usize,
    input: Option<ImageBytes>,
}

impl Plan {
    pub fn new(min_len: usize, wide_min_len: usize) -> Self {
        Self {
            min_len,
            wide_min_len,
            input: None,
        }
    }

    pub fn bind_input(&mut self, input: ImageBytes) {
        self.input = Some(input);
    }

    pub fn all(&self) -> Vec<Fact> {
        let Some(input) = &self.input else {
            return Vec::new();
        };
        let Ok(file) = crate::loadimage_object::parse_object(input) else {
            return Vec::new();
        };
        let slots = PointerSlots::new(&file);
        super::scan_strings(&file, self.min_len)
            .into_iter()
            .filter(|fact| !slots.holds_address(fact.addr))
            .map(|fact| Fact {
                addr: fact.addr,
                len: fact.len,
                char_size: 1,
                priority: 3,
            })
            .chain(
                super::kuna_widestrings::scan_wide_strings(&file, self.wide_min_len)
                    .into_iter()
                    .map(|fact| Fact {
                        addr: fact.addr,
                        len: fact.len,
                        char_size: 2,
                        priority: 2,
                    }),
            )
            .collect()
    }

    pub fn probe(&self, values: &[u64], wide: bool) -> Vec<Fact> {
        let Some(input) = &self.input else {
            return Vec::new();
        };
        let Ok(file) = crate::loadimage_object::parse_object(input) else {
            return Vec::new();
        };
        let slots = PointerSlots::new(&file);
        let mut result = Vec::new();
        for section in file.sections().filter(is_loaded_initialized) {
            let Ok(data) = section.data() else { continue };
            for &value in values {
                let Some(offset) = value
                    .checked_sub(section.address())
                    .and_then(|offset| usize::try_from(offset).ok())
                    .filter(|offset| *offset < data.len())
                else {
                    continue;
                };
                if let Some(fact) = at(data, section.address(), offset, 1, self.min_len) {
                    if !slots.holds_address(fact.addr) {
                        result.push(fact);
                    }
                }
                if wide {
                    if let Some(fact) = at(data, section.address(), offset, 2, self.wide_min_len) {
                        result.push(fact);
                    }
                }
            }
        }
        result.sort_unstable();
        result.dedup();
        result
    }
}

fn at(data: &[u8], base: u64, offset: usize, width: usize, min_len: usize) -> Option<Fact> {
    let parity = if width == 2 { (base % 2) as usize } else { 0 };
    if offset < parity {
        return None;
    }
    let mut start = offset - (offset - parity) % width;
    let character = |index: usize| {
        data.get(index).is_some_and(|byte| is_string_char(*byte))
            && (width == 1 || data.get(index + 1) == Some(&0))
    };
    while start >= parity + width && character(start - width) {
        start -= width;
    }
    let mut end = start;
    while character(end) {
        end += width;
    }
    let window = &data[start..end.saturating_add(width).min(data.len())];
    let facts = if width == 1 {
        super::scan_run(window, base + start as u64, min_len)
    } else {
        super::kuna_widestrings::scan_utf16_run(window, base + start as u64, min_len)
    };
    facts
        .into_iter()
        .find(|fact| base + offset as u64 - fact.addr < u64::from(fact.len))
        .map(|fact| Fact {
            addr: fact.addr,
            len: fact.len,
            char_size: width as u8,
            priority: if width == 1 { 3 } else { 2 },
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ascii_probe_preserves_eager_boundaries_at_every_byte() {
        let data = b"\0abc\0abcdef\0hello\tworld\n\0bad\x80text\0tail";
        let expected = super::super::scan_run(data, 0x1000, 4);
        for offset in 0..data.len() {
            let fact = expected.iter().find(|fact| {
                let value = 0x1000 + offset as u64;
                value >= fact.addr && value - fact.addr < u64::from(fact.len)
            });
            assert_eq!(
                at(data, 0x1000, offset, 1, 4).map(|fact| (fact.addr, fact.len)),
                fact.map(|fact| (fact.addr, fact.len)),
                "offset {offset}"
            );
        }
    }

    #[test]
    fn wide_probe_preserves_alignment_and_terminator_bytes() {
        let mut data = Vec::new();
        for word in "short\0x\0long enough\0".encode_utf16() {
            data.extend_from_slice(&word.to_le_bytes());
        }
        for base in [0x1000, 0x1001] {
            let expected = super::super::kuna_widestrings::scan_utf16_run(&data, base, 5);
            for offset in 0..data.len() {
                let value = base + offset as u64;
                let fact = expected
                    .iter()
                    .find(|fact| value >= fact.addr && value - fact.addr < u64::from(fact.len));
                assert_eq!(
                    at(&data, base, offset, 2, 5).map(|fact| (fact.addr, fact.len)),
                    fact.map(|fact| (fact.addr, fact.len)),
                    "base {base:x}, offset {offset}"
                );
            }
        }
    }

    #[test]
    fn image_probe_matches_eager_pass_at_all_referenced_strings() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/macho_objc_arm64");
        let bytes = ImageBytes::from(std::fs::read(path).unwrap());
        let mut plan = Plan::new(4, 5);
        plan.bind_input(bytes);
        let expected = plan.all();
        assert!(!expected.is_empty());
        let values: Vec<_> = expected
            .iter()
            .flat_map(|fact| [fact.addr, fact.addr + u64::from(fact.len) - 1])
            .collect();
        let mut expected = expected;
        expected.sort_unstable();
        expected.dedup();
        assert_eq!(plan.probe(&values, true), expected);
    }
}

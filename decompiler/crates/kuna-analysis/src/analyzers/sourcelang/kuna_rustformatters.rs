//! Preserve formatter trait identity before the loader normalizes Rust names.

use object::{BinaryFormat, Object, ObjectKind, ObjectSymbol, SymbolKind};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Default)]
pub struct FormatterMetadata {
    pub integers: BTreeMap<u64, kuna_decomp::kuna_rustfmtflags::Formatter>,
    pub strings: BTreeSet<u64>,
}

pub fn primitive_display_formatters(bytes: &[u8]) -> BTreeMap<u64, &'static str> {
    integer_formatters(bytes)
        .into_iter()
        .filter_map(|(address, formatter)| {
            (formatter.kind == kuna_decomp::kuna_rustfmtflags::Trait::Display)
                .then_some((address, formatter.primitive))
        })
        .collect()
}

pub fn integer_formatters(bytes: &[u8]) -> BTreeMap<u64, kuna_decomp::kuna_rustfmtflags::Formatter> {
    formatter_metadata(bytes).integers
}

pub fn formatter_metadata(bytes: &[u8]) -> FormatterMetadata {
    let Ok(file) = object::File::parse(bytes) else {
        return FormatterMetadata::default();
    };
    if file.format() != BinaryFormat::Elf
        || !file.is_64()
        || !file.is_little_endian()
        || !matches!(file.kind(), ObjectKind::Executable | ObjectKind::Dynamic)
    {
        return FormatterMetadata::default();
    }
    let symbols: Vec<_> = file.symbols()
            .chain(file.dynamic_symbols())
            .filter(|symbol| symbol.kind() == SymbolKind::Text && symbol.is_definition())
            .map(|symbol| {
                let full = symbol
                    .name()
                    .ok()
                    .filter(|name| name.contains("fmt"))
                    .and_then(|name| rustc_demangle::try_demangle(name).ok())
                    .map(|name| format!("{name:#}"));
                (symbol.address(), full)
            }).collect();
    let strings = super::kuna_rustfmtstr::collect(&file, &symbols);
    let integers = kuna_decomp::kuna_rustfmtflags::formatter_symbols(symbols);
    FormatterMetadata { integers, strings }
}

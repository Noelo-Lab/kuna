//! Preserve formatter trait identity before the loader normalizes Rust names.

use object::{BinaryFormat, Object, ObjectKind, ObjectSymbol, SymbolKind};
use std::collections::BTreeMap;

pub fn primitive_display_formatters(bytes: &[u8]) -> BTreeMap<u64, &'static str> {
    let Ok(file) = object::File::parse(bytes) else {
        return BTreeMap::new();
    };
    if file.format() != BinaryFormat::Elf
        || !file.is_64()
        || !file.is_little_endian()
        || !matches!(file.kind(), ObjectKind::Executable | ObjectKind::Dynamic)
    {
        return BTreeMap::new();
    }
    kuna_decomp::kuna_rustfmtargs::formatter_symbols(
        file.symbols()
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
            }),
    )
}

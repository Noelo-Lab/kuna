//! Exact String writer methods and protected vtable storage.

use object::Object;
use std::collections::BTreeMap;

#[derive(Default)]
pub struct WriterMetadata {
    pub methods: BTreeMap<u64, kuna_decomp::kuna_rustfmtwrite::WriterMethod>,
    pub relro: Option<(u64, u64)>,
}

pub(super) fn collect(
    file: &object::File<'_>,
    bytes: &[u8],
    symbols: &[(u64, Option<String>)],
) -> WriterMetadata {
    if file.architecture() != object::Architecture::X86_64 {
        return WriterMetadata::default();
    }
    WriterMetadata {
        methods: kuna_decomp::kuna_rustfmtwrite::writer_symbols(symbols),
        relro: crate::loader::kuna_dynrelocs::relro_range(bytes)
            .and_then(|(lo, hi)| (lo < (hi & !4095)).then_some((lo, hi & !4095))),
    }
}

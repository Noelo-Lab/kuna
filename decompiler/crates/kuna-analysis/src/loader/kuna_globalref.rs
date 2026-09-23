//! (kuna `globalref`) The address ranges a program's own data objects live in.
//!
//! The printer names a pointer-typed constant `&dat_<addr>` only when the
//! address lands in one of these ranges, so this is the loader's answer to
//! "could a global variable sit here": an allocated section of initialized,
//! read-only or zero-filled data. Code, TLS templates (their addresses are
//! offsets, not run-time locations), and the tables the toolchain and the
//! run-time loader own (`.got`, `.eh_frame`, `.dynamic`, import pointer slots,
//! unwind data) are not, even when their section kind says data.

use object::SectionKind;

/// Does a section of this name and kind hold objects the program's own code
/// names by address?
pub fn holds_program_objects(name: &str, kind: SectionKind) -> bool {
    let data = matches!(
        kind,
        SectionKind::Data
            | SectionKind::ReadOnlyData
            | SectionKind::ReadOnlyDataWithRel
            | SectionKind::ReadOnlyString
            | SectionKind::UninitializedData
            | SectionKind::Common
    );
    data && !is_runtime_table(name)
}

/// The data-kind sections that belong to the toolchain or the run-time loader.
fn is_runtime_table(name: &str) -> bool {
    const PREFIXES: &[&str] = &[
        ".got", ".plt", ".eh_frame", ".gcc_except_table", ".interp", ".dynamic", ".tm_clone_table",
        ".idata", ".edata", ".pdata", ".xdata", ".reloc", ".tls", "__got", "__la_symbol_ptr",
        "__nl_symbol_ptr", "__eh_frame", "__unwind_info", "__compact_unwind", "__auth_got",
        "__thread_",
    ];
    PREFIXES.iter().any(|p| name.starts_with(p))
}

/// `(vma, size)` rows as sorted, merged, inclusive `[start, stop]` ranges.
pub fn merged_ranges(rows: impl IntoIterator<Item = (u64, u64)>) -> Vec<(u64, u64)> {
    let mut out: Vec<(u64, u64)> = rows
        .into_iter()
        .filter(|&(vma, size)| size != 0 && vma != 0)
        .filter_map(|(vma, size)| vma.checked_add(size - 1).map(|stop| (vma, stop)))
        .collect();
    out.sort_unstable();
    let mut merged: Vec<(u64, u64)> = Vec::with_capacity(out.len());
    for (lo, hi) in out {
        match merged.last_mut() {
            Some(last) if lo <= last.1.saturating_add(1) => last.1 = last.1.max(hi),
            _ => merged.push((lo, hi)),
        }
    }
    merged
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn program_data_sections_hold_objects() {
        assert!(holds_program_objects(".data", SectionKind::Data));
        assert!(holds_program_objects(".bss", SectionKind::UninitializedData));
        assert!(holds_program_objects(".rodata", SectionKind::ReadOnlyData));
        assert!(holds_program_objects(".data.rel.ro", SectionKind::Data));
        assert!(holds_program_objects(".rdata", SectionKind::ReadOnlyData));
        assert!(holds_program_objects("__cstring", SectionKind::ReadOnlyString));
    }

    #[test]
    fn code_tls_and_runtime_tables_do_not() {
        assert!(!holds_program_objects(".text", SectionKind::Text));
        assert!(!holds_program_objects(".tdata", SectionKind::Tls));
        assert!(!holds_program_objects(".tbss", SectionKind::UninitializedTls));
        assert!(!holds_program_objects(".got", SectionKind::Data));
        assert!(!holds_program_objects(".got.plt", SectionKind::Data));
        assert!(!holds_program_objects(".eh_frame", SectionKind::ReadOnlyData));
        assert!(!holds_program_objects(".eh_frame_hdr", SectionKind::ReadOnlyData));
        assert!(!holds_program_objects(".interp", SectionKind::ReadOnlyData));
        assert!(!holds_program_objects(".dynsym", SectionKind::Metadata));
        assert!(!holds_program_objects(".comment", SectionKind::OtherString));
        assert!(!holds_program_objects("__la_symbol_ptr", SectionKind::Data));
    }

    #[test]
    fn ranges_sort_merge_and_drop_empty_or_unmapped_rows() {
        let rows = [(0x5000, 0x10), (0x4000, 0x1000), (0x9000, 0), (0, 0x40), (0x7000, 8)];
        assert_eq!(merged_ranges(rows), vec![(0x4000, 0x500f), (0x7000, 0x7007)]);
    }
}

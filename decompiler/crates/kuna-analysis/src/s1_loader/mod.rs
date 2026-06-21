//! S1 loader analyses -- enrich the symbol/data map from the parsed object file
//! before the deep decompiler runs.
//!
//! - [`elf_plt`] -- PLT/GOT import-name resolution (the kuna analog of Ghidra's
//!   `ElfDefaultGotPltMarkup`): map each GOT slot to its `.dynsym` name and
//!   decode each `.plt*` stub so library call sites render as `puts(...)` rather
//!   than `sub_400510(...)`.
//!
//! Planned siblings (see `docs/missing-analyses.md`): a `.symtab`/`.dynsym`
//! defined-function reader lifted out of [`crate::loadimage_object`], and
//! no-return-function detection.

pub mod elf_plt;

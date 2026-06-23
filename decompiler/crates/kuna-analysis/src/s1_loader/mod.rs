//! S1 loader analyses -- enrich the symbol/data map from the parsed object file
//! before the deep decompiler runs.
//!
//! - [`elf_plt`] -- PLT/GOT import-name resolution (the kuna analog of Ghidra's
//!   `ElfDefaultGotPltMarkup`): map each GOT slot to its `.dynsym` name and
//!   decode each `.plt*` stub so library call sites render as `puts(...)` rather
//!   than `sub_400510(...)`.
//!
//! - [`noreturn`] -- known-no-return detection (the kuna analog of Ghidra's
//!   `NoReturnFunctionAnalyzer`): mark `exit`/`abort`/… so the dead fall-through
//!   after a tail `exit()` disappears.
//!
//! - [`arm_markers`] -- ARM/Thumb mapping-symbol (`$t`/`$a`/`$d`) + STT_FUNC-LSB
//!   decode-mode (`TMode`) painting (the kuna analog of ARM's `ARM_ElfExtension`
//!   + `ArmSymbolAnalyzer`): paints the SLEIGH `TMode` context variable so Thumb
//!   code decodes as Thumb. ARM-only (no-op on every other language).
//!
//! Planned siblings (see `docs/missing-analyses.md`): a `.symtab`/`.dynsym`
//! defined-function reader lifted out of [`crate::loadimage_object`].

pub mod arm_markers;
pub mod elf_plt;
pub mod noreturn;

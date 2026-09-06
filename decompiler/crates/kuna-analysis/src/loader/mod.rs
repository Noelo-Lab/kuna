//! S1 loader analyses -- enrich the symbol/data map from the parsed object file
//! before the deep decompiler runs.
//!
//! - [`elf_plt`] -- PLT/GOT import-name resolution (the kuna analog of Ghidra's
//!   `ElfDefaultGotPltMarkup`): map each GOT slot to its `.dynsym` name and
//!   decode each `.plt*` stub so library call sites render as `puts(...)` rather
//!   than `sub_400510(...)`.
//!
//! - [`pe_iat`] -- PE IAT/INT import-name resolution (the kuna analog of Ghidra's
//!   `PeLoader.processImports`): walk the Import Directory pairing the INT (names)
//!   and IAT (slot addresses) in lockstep, name each IAT slot (the GOT-slot
//!   analog), and name the MinGW `FF 25` thunk veneers that jump through them, so
//!   a Windows PE's library calls render `puts(...)` instead of `sub_<addr>(...)`.
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
//! - [`mips_markers`] -- MIPS `$gp` recovery via per-function `t9` register-value
//!   tracking (the kuna analog of Ghidra's `MipsAddressAnalyzer`): seeds
//!   `t9 = func_entry` at each MIPS function entry (the PIC `jalr t9` ABI
//!   convention), so a PIC prologue's `addu gp,gp,t9` folds to the real `$gp` and
//!   `$gp`-relative GOT/`.sdata` loads resolve. MIPS-only (no-op elsewhere).
//!
//! Planned siblings (see `docs/missing-analyses.md`): a `.symtab`/`.dynsym`
//! defined-function reader lifted out of [`crate::loadimage_object`].

pub mod arm_markers;
pub mod elf_plt;
// (kuna) ELF section-table tolerance: an image whose section headers are
// unreadable still has program headers describing every loadable byte.
pub mod elf_shdr;
// (kuna) PE data-directory tolerance: a `NumberOfRvaAndSizes` larger than its own
// optional header is clamped to the directories that are really present.
pub mod pe_datadirs;
// (kuna) The PE header page: the `SizeOfHeaders` file bytes Windows maps
// read-only at `ImageBase`, below the first section and outside the section walk.
pub mod pe_headers;
// (kuna) ET_REL relocatable-object (`.o`) load-layout + relocation engine.
pub mod reloc_object;
// Architecture-aware ELF instruction/data relocation encoders used by the
// relocatable-object layout. Kept separate so bitfield rules are unit-testable.
mod reloc_apply;
// (kuna) `relocrebase`: rebase the load-time analysis facts of a relocatable
// object into the loaded image's address space (GH-289).
pub mod kuna_relocrebase;
// (kuna) `dynrelocs`: apply a LINKED image's dynamic relocations (.rela.dyn /
// .rel.dyn / .rela.plt) so a GOT slot holds the value the run-time loader
// would write, and report the PT_GNU_RELRO-frozen slots as constant.
pub mod kuna_dynrelocs;
// (kuna) `msvcfpconst`: recover an MSVC `__real@` floating-point constant COMDAT
// from its mangled symbol name -- materialise the undefined half's bytes at its
// synthetic extern slot, and mark both halves foldable.
pub mod kuna_msvcfpconst;
pub mod format;
// (kuna) PE import-call binding (`peimportcall`): `externref` over the IAT slots
// + upstream's PE-only no-return API names.
pub mod kuna_peimportcall;
pub mod macho_fat;
pub mod macho_stubs;
pub mod mips_markers;
pub mod noreturn;
pub mod pe_iat;

# Review: w11-elf-loader (round 1)

Branch: `rport/w11-elf-loader` (6024ebd) onto `rust-port` (3d655be)
Verifier: independent (not the porter)
Verdict: **ACCEPT_WITH_LOSSES**

The real-ELF `LoadImage` backend (`rust/crates/kuna-sleigh/src/loadimage_object.rs`,
the `object`-crate substitution for the GPL-3 `LoadImageBfd`/`loadimage_bfd.cc`) is a
faithful, non-special-cased port that genuinely decompiles a real ELF under the Rust
engine with zero regression. Scope is PARTIAL (ELF-only; x86-64 fully wired, other
common Linux/SysV machines mapped), which is exactly the W11 task — hence
ACCEPT_WITH_LOSSES rather than a plain ACCEPT.

## What was verified

### (1) Faithfulness vs `loadimage_bfd.cc`

- `load_fill` is a byte-for-byte port of `LoadImageBfd::loadFill`:
  - 512-byte read buffer (`BUFSIZE`), `bufoffset` sentinel `!0`, reused `buffer`.
  - Buffer-hit check `curaddr>=bufoffset && curaddr+size < bufoffset+bufsize`
    uses `<` for the upper bound, matching C++ exactly (not `<=`).
  - The `while(cursize>0)` walk, the `find_section` lookup, the gap zero-fill
    (`memset` to the next segment), the `offset==0 -> break -> DataUnavailError`
    "initial address not mapped" contract, and the final `memcpy(ptr,buffer,size)`
    all reproduce the C++ control flow.
- `find_section` matches `findSection`: pass 1 = the containing segment, pass 2 =
  the closest segment strictly above `offset`. The Rust relies on construction-time
  vma-sort so the first `vma>offset` IS the minimum — equivalent to the C++ `champ`
  min-scan. Verified the subtle C++ quirk that `secsize` is *unused* on the
  greater-than (champ) branch and only used in the containing (`else`) branch; the
  Rust mirrors this (only `seg_vma - curaddr` on the gap branch).
- `.bss`/`p_memsz > p_filesz` tail: the segment stores only the file extent
  (`seg.data()`), so a RAM tail past it falls into the zero-fill path, exactly as
  BFD reports `!SEC_LOAD` bytes as zero. `copy_segment` zero-fills any overread via
  `data.get(pos).unwrap_or(0)` — safe and faithful.
- `adjust_vma` = `AddrSpace::address_to_byte(adjust, wordsize)` then shifts every
  segment/section/symbol vma; `i64 as u64` + wrapping add gives correct
  two's-complement behavior for negative adjusts. Matches `LoadImageBfd::adjustVma`.
- `open_symbols`/`get_next_symbol` iterate `SymbolKind::Text` (BFD `BSF_FUNCTION`)
  with a non-empty name; `open_section_info`/`get_next_section`/`get_readonly`
  translate ELF `SHF_*`/section-kind into the kuna `section_flags`
  (`UNALLOC`/`NOLOAD`/`READONLY`/`CODE`/`DATA`) mirroring the C++ `SEC_*` map.
  `get_next_section` returns "another section follows", matching C++.
- Language pick (`language_id_for`) reads ELF machine + endianness + class and
  returns the SLEIGH language id `SleighArchitecture::resolveArchitecture` consumes:
  **x86-64 -> `x86:LE:64:default:gcc`** (verified by unit test and a live console
  decompile), i386 -> `x86:LE:32:default:gcc`, plus ARM/AArch64/MIPS/PPC/RISCV/SPARC.
  An unmapped machine is a `LowlevelError` naming the machine (caller falls back to
  `--target`). The `:gcc`/SysV-default compiler-field choice is documented.
- The console wiring (`bootstrap_from_elf`/`bootstrap_from_file` in `engine.rs`,
  `IfcLoadFile` in `ifacedecomp.rs`) faithfully ports `IfcLoadFile::execute`
  (`consolemain.cc:46`): two-token `load file [<target>] <path>` parse, the
  `\x7fELF` magic routing ELF vs XML (the C++ `ArchitectureCapability::findCapability`
  dispatch), and the C++ `resolveArchitecture` precedence (explicit target wins over
  the loader's `getArchType`). The `target=="default"`/empty-target equivalence is
  preserved. The XML/ELF frontends share the new `build_engine_and_init` tail
  (`buildSpecFile -> buildTranslator -> init_post_engine`), a clean refactor of the
  prior `bootstrap_program` body.

### (2) No special-casing (REJECT trigger — clean)

- Grep over `loadimage_object.rs` (production lines 1-545) and the engine ELF path:
  no binary-name, function-name, or address hardcoding; no `todo!`/`unimplemented!`/
  `panic!`/stub in production code. The `.expect()` calls are documented
  null-space-deref mirrors of C++ UB.
- Adversarial symtab test (mine): an ELF with FUNC symbols `alpha`/`beta`/`gamma`,
  an OBJECT symbol `global_table`, and no `main`. Result: the three FUNCs resolve to
  their real `st_value` addresses, the OBJECT symbol is excluded, and `main` returns
  None. Symbols come from the real ELF symtab, not a hardcoded list.

### (3) It really works (a DIFFERENT ELF than the porter's)

- The porter's gate builds an x86-64 ELF for `add`. My adversarial gate built a
  **32-bit i386** ELF (different EI_CLASS/machine/header layout/vma/function name):
  `compute` at `0x8048100` with real `mov eax,[esp+4]; add eax,[esp+8]; ret` bytes.
  Under `bootstrap_from_elf` + `load function compute -> decompile -> print C/raw`:
  - The loader picked the **32-bit** x86 language (description contains "x86",
    NOT "64") — proves the machine drives the language pick, not a constant.
  - The real ELF bytes were decoded at the real i386 vma: the raw listing shows
    `Basic Block 0 0x08048100-0x08048108` (the 9 mapped code bytes) and the `ret`.
  - A real C body (`compute(void){ return; }`) came out — not a crash/empty/stub.
  - (The `a+b` not surviving into C is downstream cdecl stack-arg/return recovery,
    a decompiler-pipeline concern, NOT the loader's; the loader's job — deliver the
    real bytes at the real address — is done correctly.)
- Both adversarial tests passed; the porter's `verify_w11_elf_loader` gate also
  passes (1 test). My temp test file was removed; the worktree is clean.

### (4) No regression

- C++ tree byte-untouched: `git diff -- decompiler/cpp/` and `-- decompiler/` and
  `-- specs/` are all empty (0 lines).
- C++ oracle (main tree): `python -m kuna.run_tests --all --baseline docs/baseline.json`
  -> **207/207 unittests, 675/675 datatest assertions, PARITY OK**, exit 0.
- Rust XML datatest path unchanged: `decomp_test_dbg datatests` ->
  **668 applied / 194 passing** (the W10 baseline; W11 is purely additive — the
  XML corpus starts with `<` so it never takes the ELF magic branch).
- `cargo test --workspace` green (all `test result: ok`, 0 failures; 206 kuna-sleigh
  lib tests incl. 6 new ELF tests; the new console gate passes).
- `cargo clippy` clean on the new W11 code (0 warnings in kuna-sleigh lib; no
  warnings touching `loadimage_object.rs` or the engine ELF additions; the
  remaining workspace warnings are all pre-existing in unrelated files).

## Losses (recorded in docs/rust-port/losses.md)

This work realizes the **LOSS-005 restoration criteria** (libbfd -> object crate).
The residual losses:

- **Dependency substitution (LOSS-005):** GPL-3 `LoadImageBfd` replaced by the
  `object` crate; `getArchType` resolves the SLEIGH id directly off the ELF header
  rather than via the Ghidra Java BFD-name map. Semantics of the `LoadImage`
  interface are preserved.
- **ELF-only scope (PARTIAL):** PE/Mach-O are a seam (rejected with a clear error).
  Matches the W11 task.
- **Machine coverage (PARTIAL):** x86 32/64 fully exercised; ARM/AArch64/MIPS/PPC/
  RISCV/SPARC mapped but not end-to-end driven here. An unmapped machine errors with
  a `--target` fallback path. The `:gcc`/SysV-default compiler field is the only ABI
  a bare ELF identifies (other ABIs are a seam).

These are scope/dependency losses, not unfaithfulness or special-casing, and there
is zero regression — hence ACCEPT_WITH_LOSSES.

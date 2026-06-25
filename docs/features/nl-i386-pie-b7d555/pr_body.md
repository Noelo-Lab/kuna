# nl-i386-pie-b7d555: i386-PIE PLT import resolution (`option i386_pie_plt`, DIV-7)

Closes the gap behind angr `test_decompiling_nl_i386_pie::usage`: kuna decompiled `usage` in
`i386/nl` (an i386 **PIE** ELF) into ~209 loc of broken C — a spurious `do{}while(true)` loop, a
`goto`, three un-unified `// esp` stack values, explicit frame stores, dropped call arguments, a
recovery-failure marker, and `sub_<addr>` call names — where angr emits ~80 clean loc with named
libc calls.

Full root-cause in [`analysis.md`](analysis.md); the approved design in
[`proposal.md`](proposal.md); side-by-side in [`angr-vs-kuna.txt`](angr-vs-kuna.txt).

## Root cause (one construct)

`usage` ends in a shared `exit(a0)` tail. kuna never marked `exit@plt` no-return, so `call exit`
was assumed to fall through, manufacturing a bogus back-edge → a spurious loop → an unresolvable
stack-pointer MULTIEQUAL cycle → the entire structural failure. `exit` wasn't flagged because its
i386-PIE PLT stub was never named: `kuna-analysis::s1_loader::elf_plt::decode_i386` decoded only
the non-PIC `FF 25 <abs32>` (`jmp *abs32`) stub form and **skipped** the PIE
`FF A3 <disp32>` (`jmp *disp32(%ebx)`, GOT-relative) form.

## Mechanism

`decode_i386` now also matches `FF A3 <disp32>` and computes `slot = GOT_base + sign(disp32)`,
where `GOT_base` is the `_GLOBAL_OFFSET_TABLE_` symbol value (fallback `.got.plt`/`.got` section
base) — the value the PIC prologue (`call __x86.get_pc_thunk.bx; add $_,%ebx`) loads into `%ebx`.
Verified on `nl`: `0x9edc + 0x14 = 0x9ef0` = the `R_386_JUMP_SLOT` r_offset for `_exit`. It is the
i386-PIE analog of the already-shipped `decode_x86_64` (RIP-relative) and `decode_aarch64`
(adrp/ldr veneer) decoders; the non-PIC `FF 25` arm is untouched (non-PIE i386 stays
byte-identical). Naming `exit@plt` lets the **pre-existing** `NoReturnKnownPass` flag it
no-return, which collapses the spurious loop and restores stack recovery — a multi-pass cascade
(PLT decode → import-name fact → no-return fact → flow-halt → loop collapse), not a single
op-rewrite.

## Option

`option i386_pie_plt` (default **on** — loader fidelity, like the sibling decoders). Flip with
`kuna decompile <bin> usage --option i386_pie_plt off` to restore the pre-fix rendering.

Because the PLT→name map is baked at `load file` (upstream of every per-function `option`), the
loader reads the gate through the `KUNA_I386_PIE_PLT` env var (`kuna_decomp::kuna_i386_pie_plt`),
which the `kuna` CLI sets on the `decomp_dbg` subprocess; the `Architecture::analysis_i386_pie_plt`
bool exists only so the option is catalog-visible (`stage catalog`). The option carries full
provenance (`source_decompiler = angr`, `change_kind = correctness-fix`) and is registered in
`KUNA_OPTION_NAMES` + the `settableTable`, so `kuna catalog --check` ⇒ `catalog OK`.

## Ablation / parity / speed

- **0 of 675** datatest assertions change with the feature default-ON (the bytechunk corpus never
  reaches the ELF loader / `resolve_plt_imports`, and has no i386-PIE binary) ⇒ shipped
  **default-on** with a [`docs/divergences.md`](../../divergences.md) **DIV-7** entry. `make test`
  stays **PARITY OK** (datatests 675/675); `make test-stages` **PARITY OK** (the KUNA-CATALOG #6
  angr-provenance count bumped 3→4).
- **Speed**: the collapsed spurious loop makes the target *faster* — `usage` **130 ms on vs
  422 ms off (−69%)**, well within the +5% budget.

## Testing

The `tests/stages/*.xml` bytechunk harness cannot carry `.rel.plt`/`.dynsym`/GOT structure to
reach a PLT/no-return loader path, so (per the proposal's approved sub-decision B) this is gated by
a cargo integration test — `kuna-console/tests/verify_i386_pie_plt.rs` (drives
`bootstrap_from_object` over a vendored i386-PIE `nl` ELF → `load function usage` → `print C`,
asserting the named libc calls and the absence of the spurious `do{}while(true)`/`goto`/`sub_<addr>`)
— plus the decoder unit test `elf_plt.rs::tests::i386_pie_plt_decode` (the `FF A3` on/off paths and
the unchanged `FF 25` arm). All three gates green: `kuna catalog --check`, `kuna test --all`
(datatests 675/675 PARITY OK), `make test-stages`, `make rust-test`.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

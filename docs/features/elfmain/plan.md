# elfmain — implementation plan

## 1. The change

One new P1 analysis pass,
`decompiler/crates/kuna-analysis/src/analyzers/entry/kuna_elfmain.rs`
(`ElfMainPass`), structured like its Mach-O counterpart `kuna_machomain.rs`: the
facts are computed at LOAD and applied at the commit boundary only when the gate
is on, so `--option elfmain off` restores the previous output exactly.

```rust
fn main_claim(file, bytes) -> Option<(u64, bool)>   // (main's VMA, do we name it?)
fn scan_runtime_names(file) -> (bool, bool)         // (names __libc_start_main, names main)
fn main_prototype(ctx) -> Option<PrototypePieces>   // int main(int argc, char **argv, char **envp)
```

`run` pushes the VMA into `out.entries` and `(vma, "main")` into `out.entry_names`
(the overlay the commit already consults for the dynamic `_INIT_<i>`/`_DT_INIT`
names), and the prototype into `out.prototypes`, which the commit parks on the
name at step 5.

**The address is never decoded here.** It comes from `super::libc_start_main_target`
— oracle 4 itself — so the pass cannot disagree with the entry the discovery set
already contains, and every architecture oracle 4 covers (x86-64, AArch64, ARM,
RISC-V) is covered by construction.

## 2. Interaction with `armlibcmain`

Oracle 4's ARM arm is written around the `R_ARM_RELATIVE` that relocates crt1's
GOT slot, which a non-PIE ARM32 executable does not carry; `armlibcmain` is the
arm that reads the value out of the image, and it already emits the entry and the
name `main`.

So `elfmain` consults `kuna_armlibcmain::main_vma` as a second source **for the
prototype only** — `named_here == false` suppresses the entry and the name. The
prototype is parked BY NAME, so with `--option armlibcmain off` there is no
function called `main` for it to land on and the park is a silent no-op: that
option still restores its own inventory exactly.

## 3. The refusal set

| refusal | why |
|---|---|
| not an ELF | Mach-O is `machomain`'s, PE is `entrymainproto`'s |
| the image names no `__libc_start_main` | oracle 4's evidence adds an entry; the C runtime's own name is what licenses calling that entry `main` |
| the image already names `main` | a non-stripped ELF has the better name, and a by-name park would be ambiguous (`retain_unambiguous_names`) |
| the address already carries a function symbol | whatever named it wins, through the commit's idempotent cross-scope probe |
| the address is `_start`, 0, or outside every executable section | a misdecode is a clean miss |

Both symbol refusals come off one pass over the static and dynamic names
(`scan_runtime_names`), ahead of the decode and the section scans, so on a large
image that already names its own `main` that pass is the whole cost.

## 4. The one interaction this exposed

Naming an address a second time made a latent precedence bug visible:
`--define-function 0x13c9-0x1420=stage1` and `elfmain` reach the same address, and
which spelling is *reported* was decided by `entry_name_rank`'s length tie-break,
so `main` (4 chars) displaced the caller's `stage1` (6) — `tests/cli/no-cli-function-boundary-override.json`
caught it. A caller's declaration is an assertion, so
`ConsoleProgram::declare_function` now records the declared spelling
(`declared_names`) and both canonicalizers sort it ahead of every discovered name
at that address. The discovered names stay as aliases, so a name-keyed lookup
still resolves through them.

## 5. Gate wiring (copied from `machomain`)

`phases.toml` `[[settable]]` row (tier `analysis`, change_kind `correctness-fix`,
subphase `prototype-source`) · `p0_knowledge/options.rs` registration ·
`Architecture::analysis_elfmain` + `set_kuna_option` arm + the default-ON line ·
`engine.rs::analysis_pass_enabled` arm · `kuna_console.rs` live-value arm · the
`PASS_GATES` allowlist in `kuna_phases/tests.rs` (analysis-tier gate, no
`live_field`) · counters `--fix` (settables 205→206, analysis tier 61→62) +
`phase_catalog.json` recapture · `docs/options.md` regenerated.

## 6. Tests

- 5 unit tests over `main_claim` (both x86-64 encodings, all four oracle-4
  architectures, the two non-PIE ARM shapes, the unstripped and non-ELF refusals);
- `tests/stages/kuna-elfmain.xml`, the two-pass gate over the full
  `load file` → `decompile` path (corpus 293→294, `docs/baseline-stages.json`
  re-recorded — additive only, 1007→1011 keys, no existing assertion moved);
- `tests/cli/elf-libc-start-main-unnamed.json`, the CLI-surface acceptance.

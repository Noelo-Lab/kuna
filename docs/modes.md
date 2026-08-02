# Decompiler modes (`auto` | `reliable` | `aggressive` | `fast`)

A **mode** is a named preset over kuna's runtime option surface: an ordered list
of `(option, value)` overrides layered on top of the shipped defaults. Modes are
**not** catalog options (`kuna catalog` covers the per-option settable surface);
they reference existing option names and live in one table,
`decompiler/crates/kuna-decomp/src/p0_knowledge/modes.rs` (`MODE_TABLE`), applied
by `Architecture::apply_mode` (a fan-out over `set_kuna_option`).

Select a mode with `--mode <name>` on `kuna decompile`, `kuna decompile-all`,
`kuna decompile-project`, or `kuna functions`. If the flag is omitted, these
file-based front-ends use `auto`. The console `mode <name>` command accepts the
three concrete presets; `auto` must be resolved by a front-end that knows the
input file size. A mode's overrides are applied **before** the user's
`--option`/`option` lines, so an explicit `--option` always wins (last-write).
Modes are override batches, not resets: applying a second mode in one console
session changes only the options that second mode names. Discover them with
`kuna modes` (or `kuna modes --json`).

| Mode | What it does |
|---|---|
| `auto` | The default file-front-end policy: `aggressive` below 500 KiB, `reliable` from 500 KiB up to 2 MiB, and `fast` at 2 MiB or larger. |
| `reliable` | The shipped, well-tested defaults — the safe, stable baseline. An **empty** override list. |
| `aggressive` | Maximum recovery: turns **on** every off-by-default quality, structuring, and analysis pass except `v850indirectbranch` and `dwarf_lines`. Slower and more speculative (may over-recover); best for readability and for measuring the recovery ceiling on the benchmark, not for guaranteed faithfulness. |
| `fast` | Latency first: disables the exhaustive Listing consumers, prologue-pattern scan, and AIF gap walk, then substitutes rooted direct-call discovery and conservative pointer-table validation. Keeps the shipped per-function transforms and explicit selectors, but may still discover fewer functions and recover fewer program-wide facts than `reliable`. |

## `auto` (default)

`auto` selects a concrete preset from the raw input file length:

| File size | Selected preset |
|---:|---|
| `< 512,000` bytes (500 KiB) | `aggressive` |
| `512,000` through `2,097,151` bytes | `reliable` |
| `>= 2,097,152` bytes (2 MiB) | `fast` |

The boundary comparisons are exact: a 500 KiB file is `reliable`, and a 2 MiB
file is `fast`. “Standard mode” in the size policy means the existing
`reliable` preset. The native CLI reads filesystem metadata; the browser/WASI
front-end classifies the uploaded virtual file's byte length through the same
Rust function. An explicit concrete `--mode` bypasses size selection, and a
later explicit `--option` still overrides the selected preset.

This makes small binaries favor maximum recovery while bounding the work a
browser or bulk export attempts for large inputs. On private PE
`bc4c15d826aaebeace3fec6360eb687e5662cba8745605093254931dcdb3ae1b`
(3,457,296 bytes), `auto` resolves to `fast`. The original 3.23-second result was
not a valid successful export: its 351 executable targets were 350 import
veneers plus the PE entry, so the project omitted ordinary internal bodies such
as `0x402d80` even though an explicit decompile at that address produced code.
The rooted discovery correction below preserves the latency-first policy while
making the whole-project inventory useful. With a 10-second per-function
watchdog, now the default for unfiltered fast whole-binary runs, a no-flag
confirmation completed in 445.06 seconds at 1,476 MiB peak RSS: 3,140 real C
bodies, 13 isolated failures, and 3,153 definitions total.

## `reliable`

The current defaults are already a well-tuned, net-positive-on-benchmark set
(many angr structuring flags are default-on; see `docs/history.md`).
`reliable` is a stable, named alias for that set — its override list is
deliberately empty. Pinning options here (e.g. `listing off`) would *change*
behaviour versus the surface defaults (`decompile` and `decompile-all`
auto-enable the Listing), so the empty list is the faithful implementation. It
future-proofs the preset: if the defaults later drift more aggressive,
`reliable` can pin them back.

## `aggressive`

`aggressive` flips **every** off-by-default option on, with **two** exclusions.
The options it enables:

- **transform tier**: `switchmodbound`, `switchguardbound` (speed-costly),
  `unrolledguard` (speed-costly), `stackalias`, `sparcstructret`,
  `regionedgeorder`, `returndup`, `iteexpr`
- **analysis tier**: `listing` (the master gate that enables the
  Listing-consuming passes — `fid`, `aif`, the discovered-no-return family),
  `fast_funcdisc`, `eh_frame_full`, `funcstart_patterns`, `addrtable`,
  `operand_refs`, `formatstring`, `fid`, `rtti`, `aif`, `objc`, `pdb`,
  `macho-arm64e`

`returndup` became a shipped default in DIV-54 and is deliberately kept on the
list: the preset **owns** the option, so an earlier `--option returndup off` on
the same command line is re-enabled by `--mode aggressive`, and the preset's
contract ("every recovery pass on") does not silently change if a default drifts
back.

### The two exclusions: `v850indirectbranch` and `dwarf_lines`

Every other off-by-default option is safe to blanket-enable: the format-specific
ones are inert off their target (`rtti`/`pdb` = PE, `objc`/`macho-arm64e` =
Mach-O, `sparcstructret` = the SPARC `unimp`-trap idiom, which cannot occur off
SPARC). Two are not:

- **`v850indirectbranch`** — its predicate (`kuna_is_v850_indirect_jmp`,
  `p2_lift/kuna_v850indbranch.rs`) matches **any** register-indirect `CALLIND`,
  so on x86-64/ARM it would reclassify every `call reg` into an indirect branch
  — corruption, not recovery.
- **`dwarf_lines`** — it recovers nothing. It attaches every instruction's
  `.debug_line` `file:line` as a comment, and those survive into the emitted C,
  so on any `-g` binary the body renders interleaved with `/* src.c:NNN */`
  lines (often several per statement, since one C statement spans many
  instructions). Because `auto` selects `aggressive` under 500 KiB, including it
  here made that annotated form the *default* rendering for small debug
  binaries, which is a readability regression, not a recovery win (DIV-43).

Both therefore stay manual per-run opt-ins (`--option v850indirectbranch on`,
`--option dwarf_lines on`) even under `--mode aggressive`; a named `--option`
still wins over the preset by last-write precedence.

### Caveats

`aggressive` is slower (the Listing build, `funcstart_patterns` discovery, and
`formatstring`'s re-decompile loop) and can over-recover: `aif` is a speculative
gap-walk ("may create bad code") and `addrtable` over-accepts pointer tables.
That is by design — `aggressive` is the recovery-ceiling / measurement envelope,
not the faithful default. Use it to read maximally-recovered output or to A/B
which options net-help; promote an option to a default only when its own ablation
shows net-positive-zero-regression (the DIV process).

`returndup` used to be listed here as a known aggregate-GED regression (DIV-18).
That is no longer true: since #137 gave it angr's const-return gate, an isolating
ablation over the 52,862-function optimized decbench corpus puts it at **+417
GED-perfect functions and −7,756 aggregate GED**. **DIV-54 promoted it to a
shipped default**, so it is no longer an `aggressive`-only pass at all — the
preset merely keeps naming it. See
`docs/decbench/returndup-regression-triage.md`.

## `fast`

`fast` applies exactly four overrides:

```text
listing=off
funcstart_patterns=off
aif=off
fast_funcdisc=on
```

The first three disable the exhaustive program-wide decode and speculative
discovery paths that dominate large-binary latency. Naming `listing` suppresses
its default injection on all three decompile surfaces; naming
`funcstart_patterns` and `aif` suppresses their non-x86-64 injections on
`decompile-all` and `decompile-project`. `funcstart_patterns` is independently
important: on private PE
`bc4c15d826aaebeace3fec6360eb687e5662cba8745605093254931dcdb3ae1b`,
enabling it with the other two disabled expands the inventory from 693 to 4,452
entries.

`fast_funcdisc` performs one recursive SLEIGH Listing walk from loader-backed
entries and promotes every statically reached direct-call target. It then scans
absolute code-pointer tables and admits an indirect-only target only after the
established AIF fingerprint and valid-subroutine checks corroborate it. On
non-ARM objects, tables above 256 slots are ignored; large candidate sets
require two independent table references and validate at most 4096 targets.
ARM uses the established Thumb-pointer prologue oracle. The pointer-derived
roots are not recursively expanded. This supplies real project bodies without
the full prologue corpus or byte-by-byte undefined-gap walk.

The tradeoff remains deliberate. With `listing=off`, ordinary Listing consumers
such as discovered no-return propagation and FID do not run. With full
function-start patterns and AIF disabled, functions that are neither directly
reachable nor represented by an accepted pointer table can remain absent.
Loader symbols, architecture context, import naming, explicit function/address
selection, and all shipped per-function transforms remain active.

The broken metadata-only fast policy completed quickly precisely because it did
not discover the private PE's internal program. The corrected `--mode fast`
export's unfiltered watchdog defaults to 10 seconds per function. The private
export completed in 445.06 seconds at 1,476 MiB peak RSS, producing 3,140 bodies
and 13 isolated failures without an explicit watchdog flag; the non-fast
control remained incomplete after 935.91 seconds. Native explicit name/address
selections retain the 120-second default, and `--max-fn-seconds` always wins
(including `0` to disable). The WASM front-end arms the 10-second budget only
for fast whole-binary decompile/project commands. Public regressions pin both
halves of the discovery replacement: `pdb_prog.exe` contributes a direct
internal callee, and `aif_gap_x86_64` contributes an indirect-only
pointer-table target, with real C bodies in both cases.

Explicit `--addr` commands already have their target set, so the file
front-ends suppress preset-provided `fast_funcdisc` work for those commands.
Name selection keeps discovery enabled because a generated `sub_<addr>` name
does not exist until discovery registers it. Address-selected callers can opt
back in explicitly with `--option fast_funcdisc on`; an explicit option always
wins.

An explicit option selectively restores analysis while keeping the rest of the
preset:

```bash
kuna decompile-project ./a.out --mode fast --option listing on
kuna decompile-project ./a.out --mode fast --option fast_funcdisc off
```

Because explicit options are applied after the mode, the first command restores
ordinary Listing consumers while leaving prologue patterns and AIF disabled;
the second restores the old metadata-only fast inventory for measurement or an
especially tight latency budget.

## Measuring with modes

`aggressive` and `fast` are first-class measurement levers. Run the decbench GED
benchmark with a `kuna-aggressive` backend (or `kuna decompile-all --mode
aggressive`) and compare against an explicit `--mode reliable` baseline to find
options that net-help, which then become candidate default-on flips (a new
`docs/history.md` DIV row). The omitted-mode `auto` policy intentionally changes
with input size and is therefore not a stable benchmark baseline.
Use `fast` for latency-sensitive bulk export and compare its function inventory
and output against `reliable` before adopting the coverage tradeoff.

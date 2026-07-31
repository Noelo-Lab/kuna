# Decompiler modes (`reliable` | `aggressive` | `fast`)

A **mode** is a named preset over kuna's runtime option surface: an ordered list
of `(option, value)` overrides layered on top of the shipped defaults. Modes are
**not** catalog options (`kuna catalog` covers the per-option settable surface);
they reference existing option names and live in one table,
`decompiler/crates/kuna-decomp/src/p0_knowledge/modes.rs` (`MODE_TABLE`), applied
by `Architecture::apply_mode` (a fan-out over `set_kuna_option`).

Select a mode with `--mode <name>` on `kuna decompile`, `kuna decompile-all`,
`kuna decompile-project`, or `kuna functions`, or with the console `mode <name>`
command. A mode's overrides are applied **before** the user's
`--option`/`option` lines, so an explicit `--option` always wins (last-write).
Modes are override batches, not resets: applying a second mode in one console
session changes only the options that second mode names. Discover them with
`kuna modes` (or `kuna modes --json`).

| Mode | What it does |
|---|---|
| `reliable` | The shipped, well-tested defaults — the safe, stable baseline. An **empty** override list, so it is byte-identical to running with no `--mode` at all. |
| `aggressive` | Maximum recovery: turns **on** every off-by-default quality, structuring, and analysis pass. Slower and more speculative (may over-recover); best for readability and for measuring the recovery ceiling on the benchmark, not for guaranteed faithfulness. |
| `fast` | Latency first: disables the Listing, prologue-pattern function discovery, and AIF gap walk. Keeps the shipped per-function transforms and explicit selectors, but may discover fewer functions and recover fewer program-wide facts. |

Running with **no** `--mode` is exactly `reliable` (the current defaults) — the
default path is untouched, so every corpus/test gate is unaffected.

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

`aggressive` flips **every** off-by-default option on, with **one** exclusion.
The options it enables:

- **transform tier**: `switchmodbound`, `switchguardbound` (speed-costly),
  `unrolledguard` (speed-costly), `stackalias`, `sparcstructret`,
  `regionedgeorder`, `returndup`, `iteexpr`
- **analysis tier**: `listing` (the master gate that enables the
  Listing-consuming passes — `fid`, `aif`, the discovered-no-return family),
  `eh_frame_full`, `funcstart_patterns`, `dwarf_lines`, `addrtable`,
  `operand_refs`, `formatstring`, `fid`, `rtti`, `aif`, `objc`, `pdb`,
  `macho-arm64e`

### The one exclusion: `v850indirectbranch`

Every other off-by-default option is safe to blanket-enable: the format-specific
ones are inert off their target (`rtti`/`pdb` = PE, `objc`/`macho-arm64e` =
Mach-O, `sparcstructret` = the SPARC `unimp`-trap idiom, which cannot occur off
SPARC). `v850indirectbranch` is the exception — its predicate
(`kuna_is_v850_indirect_jmp`, `p2_lift/kuna_v850indbranch.rs`) matches **any**
register-indirect `CALLIND`, so on x86-64/ARM it would reclassify every
`call reg` into an indirect branch — corruption, not recovery. It therefore stays
a manual per-target opt-in (`--option v850indirectbranch on`) even under
`--mode aggressive`.

### Caveats

`aggressive` is slower (the Listing build, `funcstart_patterns` discovery, and
`formatstring`'s re-decompile loop) and can over-recover: `aif` is a speculative
gap-walk ("may create bad code"), `addrtable` over-accepts pointer tables, and
`returndup` is known to regress aggregate GED on the benchmark (DIV-18, reverted
as a default) even though it recovers early returns some functions need. That is
by design — `aggressive` is the recovery-ceiling / measurement envelope, not the
faithful default. Use it to read maximally-recovered output or to A/B which
options net-help; promote an option to a default only when its own ablation shows
net-positive-zero-regression (the DIV process).

## `fast`

`fast` applies exactly three overrides:

```text
listing=off
funcstart_patterns=off
aif=off
```

These are the program-wide decode and speculative-discovery paths that dominate
large-binary latency. Naming `listing` suppresses its default injection on all
three decompile surfaces; naming `funcstart_patterns` and `aif` suppresses their
non-x86-64 injections on `decompile-all` and `decompile-project`.
`funcstart_patterns` is independently important: on private PE
`bc4c15d826aaebeace3fec6360eb687e5662cba8745605093254931dcdb3ae1b`,
enabling it with the other two disabled expands the inventory from 693 to 4,452
entries.

The tradeoff is deliberate. With no Listing, Listing consumers such as
discovered no-return propagation do not run. With function-start patterns and
AIF disabled, stripped non-x86-64 binaries can expose fewer functions. Loader
symbols, architecture context, import naming, explicit function/address
selection, and all shipped per-function transforms remain active.

On that private PE, the old default `decompile-project` run had not completed
after 935.91 seconds. `--mode fast` completes in a 64.13-second median
(62.85/64.13/64.87, 141,548 KiB median peak RSS), an observed lower-bound
speedup of 14.6×. Its `.c`, `.h`, and `.asm` artifacts are byte-identical to the
three explicit options above. This measurement isolates the mode; filtering the
binary's non-code import slots is a separate batch-target concern.

An explicit option selectively restores analysis while keeping the rest of the
preset:

```bash
kuna decompile-project ./a.out --mode fast --option listing on
```

Because explicit options are applied after the mode, that command restores the
Listing while leaving speculative function discovery and AIF disabled.

## Measuring with modes

`aggressive` and `fast` are first-class measurement levers. Run the decbench GED
benchmark with a `kuna-aggressive` backend (or `kuna decompile-all --mode
aggressive`) and compare against the default `kuna` (== `reliable`) to find
options that net-help, which then become candidate default-on flips (a new
`docs/history.md` DIV row).
Use `fast` for latency-sensitive bulk export and compare its function inventory
and output against `reliable` before adopting the coverage tradeoff.

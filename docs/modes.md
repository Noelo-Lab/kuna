# Decompiler modes (`reliable` | `aggressive`)

A **mode** is a named preset over kuna's runtime option surface: an ordered list
of `(option, value)` overrides layered on top of the shipped defaults. Modes are
**not** catalog options (`kuna catalog` covers the per-option settable surface);
they reference existing option names and live in one table,
`decompiler/crates/kuna-decomp/src/p0_knowledge/modes.rs` (`MODE_TABLE`), applied
by `Architecture::apply_mode` (a fan-out over `set_kuna_option`).

Select a mode with `--mode <name>` on `kuna decompile`, `kuna decompile-all`, or
`kuna functions`, or with the console `mode <name>` command. A mode's overrides
are applied **before** the user's `--option`/`option` lines, so an explicit
`--option` always wins (last-write). Discover them with `kuna modes` (or
`kuna modes --json`).

| Mode | What it does |
|---|---|
| `reliable` | The shipped, well-tested defaults — the safe, stable baseline. An **empty** override list, so it is byte-identical to running with no `--mode` at all. |
| `aggressive` | Maximum recovery: turns **on** every off-by-default quality, structuring, and analysis pass. Slower and more speculative (may over-recover); best for readability and for measuring the recovery ceiling on the benchmark, not for guaranteed faithfulness. |

Running with **no** `--mode` is exactly `reliable` (the current defaults) — the
default path is untouched, so every corpus/test gate is unaffected.

## `reliable`

The current defaults are already a well-tuned, net-positive-on-benchmark set
(many angr structuring flags are default-on; see `docs/divergences.md`).
`reliable` is a stable, named alias for that set — its override list is
deliberately empty. Pinning options here (e.g. `listing off`) would *change*
behaviour versus the defaults (`decompile-all` auto-enables the Listing, DIV-15),
so the empty list is the faithful implementation. It future-proofs the preset: if
the defaults later drift more aggressive, `reliable` can pin them back.

## `aggressive`

`aggressive` flips **every** off-by-default option on, with **one** exclusion.
The options it enables:

- **transform tier**: `switchmodbound`, `switchguardbound` (speed-costly),
  `unrolledguard` (speed-costly), `stackalias`, `sparcstructret`,
  `regionedgeorder`, `returndup`
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

## Measuring with modes

`aggressive` is a first-class measurement lever: run the decbench GED benchmark
with a `kuna-aggressive` backend (or `kuna decompile-all --mode aggressive`) and
compare against the default `kuna` (== `reliable`) to find options that net-help,
which then become candidate default-on flips (a new `docs/divergences.md` DIV).

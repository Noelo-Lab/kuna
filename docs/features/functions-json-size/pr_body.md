## What was broken

`docs/re-needs/functions-json-size.md` (4 instances, one in-repo fixture + three
dataset crackmes):

> `kuna functions <bin> --json` is the one cheap, whole-binary call kuna offers:
> it loads once and answers "what is in here". Its records carry only `name`,
> `address`, `address_hex` and `aliases` — there is no extent. An agent triaging
> a 250-function binary therefore has no way to order its work except by address,
> and the standard first move ("decompile the three biggest functions") costs a
> full `decompile-all` instead of one inventory call.

**The filed hypothesis was wrong, and the way it was wrong mattered.** The need
recorded that `decompile-all --json` "emits `size` on every record, from the same
`FuncResult`. Only the cheap call drops it", and predicted a four-field struct
next to a five-field one. The field is emitted — but its value is `0` on every
record of every binary:

```
$ kuna decompile-all …/aif_gap_x86_64 --json | jq '[.functions[].size] | unique'
[0]                       # and 0 nonzero sizes across every fixture in the tree
```

`FuncResult.size` came from `Funcdata::get_size()`, which is written once in the
`Funcdata` constructor from the caller's **requested flow bound** and never
updated by flow recovery. `decompile_targets` passes `0` for that bound
(*"UNBOUNDED: the function's natural flow extent"*), so it is always `0` on every
whole-binary surface; the only writer of a nonzero value is the XML decode path,
which no CLI surface reaches.

So this was never a one-field copy. Copying the field across would have satisfied
the acceptance probe's `exists` clause with a column of zeroes and closed nothing.
kuna had no function-extent value on *any* surface.

## Mechanism

kuna's model of a function is its **entry** — the Listing is keyed by entry VMA
and nothing in it records a body. New module
`decompiler/crates/kuna-console/src/funcextent.rs` reconstructs the extent as the
**address-contiguous clip** `[entry, min(next_entry, end_of_containing_CODE_section))`.

That is not invented here: it is the same reconstruction kuna already applies
wherever it needs a body from an entry-keyed model —
`kuna-analysis/src/analyzers/fid/extent.rs::calculate_extent`, whose module doc
documents the identical LOSS, and the `noreturn_disc` call-site reasoning.
It reuses the entry list and the loader section table, both already in hand, so
it adds no decode.

`FunctionEntry` gains `size: u64`, filled in one pass by
`function_entries_canonical` (already ascending, from its `BTreeMap`) and by the
new `ConsoleProgram::function_extent_at` for synthesized `--addr` entries.
`FuncResult.size` carries that number instead of the dead `fd.get_size()`, so
`functions`, `decompile-all`, `decompile-project` and the wasm browser inventory
all report **one number with one meaning** — which is what the need asked for
("match it exactly rather than inventing a second meaning for the same field
name"). The extent is reported on the error and external arms too: it is an
inventory fact and does not depend on the decompile succeeding.

```
$ kuna functions …/aif_gap_x86_64 --json | jq -c '.functions[] | {n:.name, s:.size}'
{"n":"_DT_INIT","s":27}      # exactly .init
{"n":"sub_1020","s":16}      # .plt stub
{"n":"__cxa_finalize","s":8} # .plt.got thunk
…
{"n":"sub_13c9","s":682}     # the .text tail
```

### Two alternatives rejected

- **ELF `st_size`** — exact and free, but `0` on stripped binaries, and the
  probe's own fixture is stripped (6 `.dynsym` entries, all `UND`). It would have
  produced an all-zero column on the very binary the acceptance targets. A hybrid
  makes `size` mean two different things depending on whether the binary is
  stripped.
- **A flow-reachable extent from the Listing** — kuna deliberately does *not*
  build the Listing for `kuna functions` on x86-64 (it is measured entry-neutral
  there), so this would put a full linear decode behind the one call whose whole
  purpose is being cheap. This is the cost question the need's Refutation section
  asked to be settled first.

## What the number means

An **upper bound**, documented as such in `docs/cli.md` and
`docs/spec/01-program-prep.md`: the clip runs to the neighbour, so inter-function
alignment padding is counted in. Measured against ELF `st_size` over the 41
symbolized fixtures (1428 functions with ground truth):

| | |
|---|---|
| never short | **0 undershoots / 1428** |
| exact | 231 |
| median overshoot | +8 bytes |
| worst overshoot | +52 bytes |

An entry in no CODE section — a pointer slot, an undefined external — reports `0`.

## The acceptance probe now passes

```
$ python -m scripts.repipe.verify --need functions-json-size --json
"passed": true, "transition": "closed"
clause functions[0].size exists → actual 27
```

Promoted verbatim to `tests/cli/functions-json-size.json`.

*(Noted, not fixed here: nothing currently **runs** `tests/cli/*.json` — `promote`
writes the directory and the webui displays it, but no target executes a promoted
probe. The guard that actually runs in CI for this need is the pair of cargo
integration tests below, per the track-`tooling` contract. A generic runner is
pipeline infrastructure; it is recorded as a follow-up in `record.json`.)*

## Tests

- `funcextent.rs` — 8 unit tests: the clip rule, section selection, entries
  outside every CODE section, duplicate/backward neighbours, `code_spans`
  filtering (zero-length, non-CODE, wrapping), `extent_at`.
- `functions_json_carries_a_ranking_extent` — the field must exist on every
  record **and discriminate**. An all-zero or all-equal column fails, which is
  exactly what would have shipped had the filed hypothesis been followed.
- `functions_and_decompile_all_agree_on_size` — one field name, one meaning
  across the two surfaces.

Collateral sweep over all 4 `FunctionEntry` construction sites and the non-ELF /
non-x86-64 formats: stripped x86-64 ELF 34/34, ARM Thumb 2/2 (bit normalized),
Cortex-M 74/74, COFF `.obj` 2/2 and 3/3. No negative or absurd extents.

## Speed

Interleaved old/new, 1 warmup + 9 interleaved repetitions, against the main
tree's pre-change release binary at `63a124ae`:

| target | median | min |
|---|---|---|
| `bash` O2 (2538 fns, 1.3 MB) | **−0.18%** | −0.09% |
| `betaflight` Cortex-M (5798 fns) | **−0.58%** | −0.18% |
| `ssh-sk-helper` (802 fns) | **+0.21%** | −0.10% |

No measurable cost — the cheap inventory call stays cheap.

## Gates

| gate | result |
|---|---|
| `make test` | **PARITY OK** — 675/675 assertions |
| `make test-stages` | **PARITY OK** — 574/574 assertions |
| `make rust-test` | green (incl. `decompile_all_cli` 22/22, `funcextent` 8/8) |
| `make check-spec` | check-spec OK |
| `kuna catalog --check` | catalog OK |

No option added: a JSON metadata field cannot change emitted C, so there is no
`phases.toml` row, no `options.rs` registration and no catalog counter to bump.

One unrelated one-liner rides along because `scripts.repipe.mergecheck` rejects
without it: `kuna_phases/tests.rs`'s `settable_count_is_117` is renamed to
`settable_count_is_127`. The stale name is already on `origin/main` (the
assertion inside it has long said 127) and the counter tool matches on the
function name, so every branch is blocked until someone renames it. No assertion
changed.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

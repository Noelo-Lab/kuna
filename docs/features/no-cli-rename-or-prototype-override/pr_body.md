## What is broken

Three backlog records, all severity **major**, all filed in round 2, say the same thing about
three different subjects — an agent can read a kuna decompilation but cannot correct one.

> `rename`, `retype`, `map param`, `map return`, `override prototype` and `parse line extern ...`
> are all functional in the console and none is reachable from `kuna`. [...] For an agent, a
> rename that does not persist is the difference between reading a decompilation once and
> actually working through one.
> — `no-cli-rename-or-prototype-override`, instances 1, rounds [2]

`no-cli-data-code-override` (instances 1) and `no-cli-structuring-override` (instances 1) are
the same wall for data/code ranges and for the CFG. The captain dispatched this need as the
family's **one design**, with three standing instructions: cover all three siblings, answer the
cheap/expensive question as a table, and file the acceptance probe this need has never had.

This PR is that design. **It changes no source file** — the diff is the proposal, its record,
and the need's new `## Acceptance` section.

## What was measured

Everything in the proposal's tables was measured by driving `decomp_dbg` directly at
`e3db5512` on in-repo fixtures. "Reaches C" means the *printed C changed*, not that the command
returned Ok — which turned out to be the distinction that matters.

- **11 console commands reach emitted C today** and only lack a path from the `kuna` binary.
  `parse line extern int4 authenticate(char *user,char *pass);` renames both parameters through
  the whole body; `override flow 0x4006aa branch` turns a CALL into a tail-call and restructures
  the function; `map param 0 [register,0x38,8] char *username` rewrites the signature.
- **The shared Hypothesis is overturned on its second half.** "The expensive half is the stubs"
  assumes engine ports are missing. They are not: `Override::insert_force_goto`,
  `Funcdata::install_jump_table`, `Override::insert_deadcode_delay`, `Symbol::set_isolated` and
  `FuncProto::set_input_lock/set_output_lock` are all already in `kuna-decomp`, and the stub
  message blaming `parse_machaddr`/`parse_C` is stale — both are implemented and in use. Same
  finding, same shape, as `no-cli-function-boundary-override`.
- **The real expensive half is two shipped commands that lie**, which no source reading finds:
  `map return` **panics the process** (`outtype null`, `p4_calls/fspec.rs:2624`), and
  `override prototype` prints `Successfully added override` and changes nothing — measured on
  both a direct known callee and an indirect call.
- **Two further CLI-level defects, in no need:** `kuna decompile --kassert "p9 naming-policy v2
  buf"` exits 0 and does nothing (the assertion is emitted before the first `decompile`, so the
  local does not exist yet and the error goes into a discarded transcript), and `--kassert` is
  rejected outright alongside `--json`.

## The mechanism

`--assert <directive> | @FILE`, repeatable, on `decompile` / `decompile-all` /
`decompile-project` — one intent-keyed vocabulary (`prototype`, `param`, `return`, `name`,
`type`, `data`, `label`, `readonly`, `volatile`, `flow`, `goto`, `jumptable`, …) lowering onto
the console commands that already work, with an `assertions[]` array in every `--json` reporting
each directive as `applied` or `rejected` with a reason. The `@FILE` contract is `--define-
function`'s (#374) verbatim, because that is what makes an override durable across runs.

The in-process half is largely already built: `decompile_step::DecompileSeed` is the shared
"console-only facts" carrier whose slices the whole-binary loop leaves empty, and
`decompile_one` already contains a decompile → adjust → re-decompile loop. Staged as A (this
need) / B (`no-cli-data-code-override`) / C (`no-cli-structuring-override`), so the siblings
collapse into a directive-table row and a stub to wire rather than each burning a builder.

## The acceptance probe

`verify --acceptance-suite` reported this need **`unrunnable`** before this branch — "no
acceptance probe on the record or in the probe store" — so nothing built on it could ever have
been closed by `B_DONE`. It now carries `a-a58fc408288b`:

```
kuna decompile decompiler/crates/kuna-analysis/tests/fixtures/fauxware authenticate --json \
  --assert 'prototype authenticate int4 authenticate(char *user,char *pass)' \
  --assert 'type v2 char[16]' \
  --assert 'name v2 credbuf'
```

asserting `assertions` has 3 rows all `applied`, and that the C contains
`authenticate(char *user,char *pass)` and `char credbuf [16];` and no longer contains
`char v2 [8]`.

It is **runnable and failing**, which is exactly right for an unimplemented acceptance:

```
passed: false  unrunnable: false
  exit_code   expected {"eq": 0}   actual 2       (error: unknown option --assert)
  stdout_is_json   expected true   actual false
  json[0..6]  <stdout is not JSON>
```

Its target C is not aspirational — it was produced end-to-end by `decomp_dbg` at `e3db5512`,
on that same in-repo fixture, by the exact console script Stage A would generate:

```
int4 authenticate(char *user,char *pass) // return-dupe x2
{
  char credbuf [16]; // stack - 0x18
  int4 v1; // eax
  ...
```

The engine already produces the acceptance output. Stage A is the path to it.

## Gates

No source file changed, so no gate can move: the diff is `docs/features/no-cli-rename-or-
prototype-override/{proposal.md,record.json,pr_body.md}`, `docs/re-needs/no-cli-rename-or-
prototype-override.md` and the regenerated `docs/re-needs/index.json`.

- `python -m scripts.repipe.needs reindex` — 35 needs indexed, clean round-trip
- `python -m scripts.repipe.verify --acceptance-suite --need no-cli-rename-or-prototype-override`
  — `unrunnable: false`, `passed: false` (was `unrunnable: true` at `e3db5512`)
- `kuna catalog --json` swept for a covering option: 137 rows, none matching
  rename/retype/prototype-override — not a default-flip candidate

🤖 Generated with [Claude Code](https://claude.com/claude-code)

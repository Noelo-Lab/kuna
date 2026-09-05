## What was broken

> `rename`, `retype`, `map param`, `map return`, `override prototype` and
> `parse line extern ...` are all functional in the console and none is reachable from
> `kuna`. [...] For an agent, a rename that does not persist is the difference between
> reading a decompilation once and actually working through one.
>
> — `docs/re-needs/no-cli-rename-or-prototype-override.md` (severity major, instances 1, rounds [2])

Everything kuna knows about a program it *derived*. The console has carried the commands
that correct each derivation all along; the `kuna` binary's generated script emitted a fixed
vocabulary — `option`, `read symbols`, `load`, `kassert`, `function bounds`, `decompile` —
and nothing else. `--kassert` exists but only its `Option` and `Rename` arms do real work,
and the `Rename` one is a **silent no-op from the CLI**: it is emitted before the first
`decompile`, so the local it names does not exist yet and `No symbol named: v2` goes into a
transcript the CLI discards. `--kassert` is also refused outright with `--json`, so for an
agent consuming machine-readable output the override surface did not exist at all.

This is Stage A of the approved proposal (`docs/features/no-cli-rename-or-prototype-override/proposal.md`).

## The mechanism

`--assert <directive> | @FILE`, repeatable, on `decompile`, `decompile-all`,
`decompile-project` and `functions`.

```bash
kuna decompile ./a.out authenticate --json \
  --assert 'prototype authenticate int authenticate(char *user,char *pass)' \
  --assert 'type v2 char[16]' \
  --assert 'name v2 credbuf'
```

```text
- unsigned long authenticate(char *a0,char *a1)     - char v2 [8];
+ int authenticate(char *user,char *pass)           + char credbuf [16];
```

Nine directives, keyed by **intent** rather than by phase — an agent should not have to know
that renaming is P9 to rename something — each lowering to the console command that already
implements it:

| directive | lowers to | writes at |
|---|---|---|
| `function <start>[-<end>][=<name>]` | `function bounds` | P1 (the `--define-function` spelling) |
| `typedef <C declaration>` | `parse line` | P5 type-propagation |
| `prototype <func> <C declaration>` | `parse line extern` | P4 prototype-source |
| `data <addr> <C typedeclaration>` | `map address` | P5 const-pointer |
| `param [<func>::]<i> <storage> <C typedecl>` | `map param` | P4 prototype-source |
| `return [<func>::]<storage> <C typedecl>` | `map return` | P4 prototype-source |
| `comment [<func>::]<addr> <text>` | `comment instruction` | P9 external-refinement |
| `name [<func>::]<symbol> <newname>` | `rename` | P9 naming-policy |
| `type [<func>::]<symbol> <C type>` | `retype` | P5 type-propagation |

**Machine-readable in, machine-readable out.** Every `--json` document grows an `assertions`
array — one row per directive, in the caller's order, carrying the directive text, its phase
and sub-phase, `applied` or `rejected`, and a reason. A rejection is also spoken on stderr on
both surfaces, is non-fatal by default (a batch of forty renames against a re-decompiled
binary must not lose the other thirty-nine to one stale name), and `--assert-strict` makes it
the run's verdict.

**Three slots, and the ordering is forced rather than stylistic.** Program-scoped directives
are applied right after the analysis commit, for the same reason a declared boundary is: an
assertion outranks discovery. Function-scoped ones become decompile seeds, because a
prototype fact is consumed at flow time. Symbol-scoped ones (`name`, `type`) can only run
*between two decompiles* — the local they name does not exist until one has produced it,
which is exactly the bug that makes `--kassert p9 naming-policy` inert today. The second pass
is emitted only when such a directive bound to the function, so **every run without one costs
what it did before**. An unqualified directive on a multi-function run is rejected with a
detail naming the `<func>::<operand>` form rather than applied to every function that happens
to have a `v2`.

`@FILE` is the durable form, the `--define-function` contract verbatim: kuna does not write
assertions back into the image, so the file is the artifact.

## Also fixed: `map return` aborted the process

`map return <addr> <type>` killed the whole console the moment its function was decompiled —
`outtype null`, `ParamListStandardOut::assignMap`. It parks OUTPUT-ONLY `PrototypePieces`
(explicit storage, no declared return type) and `assignParameterStorage` dereferences
`outtype` unconditionally. `FuncProto::seed_locked_from_pieces` already special-cased
`outtype: None && output_storage: None`; the `Some` case fell through to the abort. The
declared type *is* the return type. Fixed at the engine seam so every caller gets it, with a
regression test that drives the drive with output-only pieces directly.

## The filed hypothesis is overturned

> ADVISORY. The cheap half is exposure [...] The expensive half is the stubs.

The cheap half is **much larger** than filed — 11 console commands were measured reaching
emitted C. The expensive half is **not** the stubs, whose engine entry points are all already
ported: it was two *shipped* commands that lie. One (`map return`) is fixed here. The other
(`override prototype`) is residue, and the proposal's own diagnosis of it (*"the `queryCall`
consume is stubbed OR `applyPrototype` is wired — one is stale"*) is itself wrong: reading
the whole chain, every link is present and looks correct — store, re-seed, install on the
fresh `Funcdata`, `build_override_proto` builds a real locked `FuncProto` — and the C still
does not change (re-measured on this branch: `strcmp` at `0x400689`, 2 params → 3, no
change). Time-boxed per the dispatch; the `prototype` directive lowers to `parse line
extern`, which is measured working and is what the acceptance asserts, so nothing shipped
here depends on it. Per-call-site prototype override is worth its own need.

`hypothesis_status` on the need record is reconciled from `upheld` to `overturned`, matching
its `record.json` and the proposal's verdict.

## Acceptance

`a-a58fc408288b` — FAILED at `e3db5512` (`exit_code 2`, `error: unknown option --assert`),
**PASS** here, all seven clauses. Promoted verbatim to
`tests/cli/no-cli-rename-or-prototype-override.json` (in-repo fixture, so CI needs no
dataset).

## Every directive's test asserts the emitted C CHANGED

Not that the command returned Ok — reviewing on the return value is how `override prototype`
got shipped broken. `crates/kuna-console/tests/verify_assertplane.rs`, 11 tests, each a diff
against the un-asserted baseline: prototype (signature + parameter names), param (locked
input), return (locked output), name+type (`char v2 [8]` → `char credbuf [16]`), typedef+type
(`creds v2;` / `v2.raw`), data (`sneaky` → `shadowpw` at the call), comment (`/* ... */` in
the body), function (`authstub`, 28 bytes), plus directive ordering, the multi-function
qualification rule, and the `map return` abort. `assertdecl` adds 9 syntax tests and
`decompile.rs` 4 more for the script slots and the transcript reader.

`label` (`map label`) is deliberately **not** shipped: it has no observable effect on emitted
C on any fixture tried, so no test could satisfy that rule, and shipping it would have made
this PR an instance of the failure mode it exists to close. Recorded as residue.

## Gates

| gate | result |
|---|---|
| `make test` | **PARITY OK**, 675/675 |
| `make test-stages` | **PARITY OK**, 603/603 |
| `make rust-test` | green |
| `make check-spec` | OK |
| `kuna catalog --check` | OK |
| `verify --need no-cli-rename-or-prototype-override` | acceptance **PASS** |

No `phases.toml` row, no `options.rs` registration, no catalog counter, no `docs/options.md`
regeneration, no DIV row and no `tests/stages/` case: a flag that only carries caller
assertions cannot change emitted C for a run that does not pass one, and the tooling track
scopes the stages corpus away from it. The parity corpora are structurally unaffected —
symbol-less bytechunks driven by XML, with no CLI in the loop.

**Deferred, not skipped:** the P4/P5/P9 `exposure` prose in `phases.toml`. That file was
leased by `b-r2-ppc64-localentry` for this whole wave (captain dispatch item 2). It is prose,
no gate reads it. Stage B (`no-cli-data-code-override`) and Stage C
(`no-cli-structuring-override`) stay parked as their own needs and their own PRs.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

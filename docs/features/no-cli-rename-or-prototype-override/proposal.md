# `--assert`: one override plane for the whole no-cli-* family

**Needs covered:** `no-cli-rename-or-prototype-override` (the dispatched one),
`no-cli-data-code-override`, `no-cli-structuring-override`.
**Track:** tooling. **Scope:** large. **Base:** `e3db5512`.

## 1. The problem

Three backlog records say the same thing about three different subjects:

> `rename`, `retype`, `map param`, `map return`, `override prototype` and `parse line extern ...`
> are all functional in the console and none is reachable from `kuna`.
> — `no-cli-rename-or-prototype-override` (severity major, instances 1, rounds [2])

> `map address <addr> <typedeclaration>`, `parse line`, `type varnode`, `readonly <addr+size>`
> and `volatile <addr+size>` all work in the console and none is reachable from `kuna`.
> — `no-cli-data-code-override` (severity major, instances 1, rounds [2])

> Of the four structuring overrides `phases.toml` advertises in its own `exposure` fields,
> three are `engine_unavailable` stubs [...] and the fourth, `override flow`, works but is
> console-only.
> — `no-cli-structuring-override` (severity major, instances 1, rounds [2])

All three were captain-seeded from a source survey and each has picked up an independent
tester witness since: `keyboard-callback-uses-undefined` (an agent that could state a
callback's prototype would not need kuna to infer R9D/R8D), the three-sighting "kuna will
not tell me about data" family (653d8860 x2 + 69a3822f), and round 1's `switch(0)`.

The delivery vehicle now exists. `no-cli-function-boundary-override` closed via #374, which
shipped `kuna decompile --define-function <start[-end][=name] | @file>`: an agent-supplied
fact, injected from the `kuna` binary, carried in a plain-text file the agent owns, honored
by both the script surface and the in-process surface. This proposal is that shape,
generalised once, for everything else an agent needs to tell kuna.

## 2. The cheap/expensive question, answered by measurement

The captain asked for this as a table. Every row was measured at `e3db5512` by driving
`decomp_dbg` directly on in-repo fixtures (`fauxware`, `aif_gap_x86_64`) — not read off the
source. "Reaches C?" means *the emitted C actually changed*, which is a strictly harder
question than "the command returned Ok".

### Already works in the console — the job is a path from `kuna`

| Console command | Named by | Reaches C? | Measured evidence |
|---|---|---|---|
| `rename <sym> <new>` | rename | **yes** | `char v2 [8]` → `char credbuf [16]` after `rename`+`retype` |
| `retype <sym> <type>` | rename | **yes** | same run; the array widened and the frame re-laid out |
| `map param <i> <addr> <decl>` | rename | **yes** | `authenticate(char *a0,char *a1)` → `authenticate(char *username)` |
| `parse line extern <decl>` | rename | **yes** | → `int4 authenticate(char *user,char *pass)`, names bound through the body |
| `parse line typedef/struct/enum` | data | n/a | interns the type; `verify_w10_union_truncation.rs` exercises it |
| `map address <addr> <decl>` | data | **yes** | `strcmp(a1,sneaky)` → `strcmp(a1,shadowpw)`, retyped |
| `type varnode <vn> <decl>` | data | (usepoint) | live; needs the `[space,offset,size]` varnode syntax, not a bare address |
| `readonly <addr> <size>` | data | not observed | accepted ("Successfully marked range as readonly"); no fold on the fixture tried |
| `volatile <addr> <size>` | data | not observed | accepted |
| `override flow <addr> <kind>` | structuring | **yes** | `override flow 0x4006aa branch` turned the CALL into a tail-call and restructured |
| `map label <name> <addr>` | — | n/a | accepted |
| `comment instruction <addr> <text>` | — | n/a | accepted |
| `map function` / `load addr` | boundary | **yes** | already exposed as `--define-function` (#374) |

### Broken in the console — a fix, then a path

| Console command | What actually happens | Cost |
|---|---|---|
| `map return <addr> <decl>` | **panics the process**: `outtype null`, `p4_calls/fspec.rs:2624` in `assign_map_standard_out`. Reproduces standalone, first command after `load function`. | small: the parked `PrototypePieces` is output-only and carries `outtype: None` where `assign_map_standard_out` unwraps it |
| `override prototype <addr> <decl>` | prints `Successfully added override` and **changes nothing** — measured on a direct known callee (`strcmp` at `0x400689`, 2→3 params) and on an indirect call (`CALL RDX` at `0x1405` in `aif_gap` `stage1`). Its own doc comment says the `FlowInfo::queryCall` consume is stubbed; `flow.rs:2152` says `applyPrototype` is wired. One of the two is stale. | medium: find which, then either wire the consume or fix the store |

### `engine_unavailable` stubs — and the hypothesis they were filed under

The records' shared Hypothesis says *"The expensive half is the stubs."* **That is overturned,
exactly as it was on the boundary need.** Every engine entry point the stubs name is already
ported in `kuna-decomp`:

| Stub command | Engine entry it names | Ported? |
|---|---|---|
| `force goto` | `Override::insertForceGoto` | `p0_knowledge/overrides.rs:256` `insert_force_goto` |
| `override jumptable` | `Funcdata::installJumpTable` | `substrate/funcdata_block.rs:1724` `install_jump_table` |
| `deadcode delay` | `Override::insertDeadcodeDelay` | `p0_knowledge/overrides.rs:265` `insert_deadcode_delay` |
| `isolate` | `Symbol::setIsolated` | `p0_knowledge/database.rs:834` `set_isolated` |
| `prototype lock/unlock` | `FuncProto::setInput/OutputLock` | `p4_calls/fspec.rs:5204/5242` |
| `global add`/`remove` | `ScopeGlobal` range add | `ScopeGlobal` exists; the range call needs checking |
| `name varnode` | `Funcdata::nameRecommend` | not checked |
| `structure blocks` | `BlockGraph` structuring | genuinely large — **out of scope** |
| `analyze range` | `ValueSetSolver` | genuinely large — **out of scope** |

The stub message ("`Architecture print/types/loader/context + parse_machaddr/parse_C grammars
are a later W-item`") is stale on its face: `parse_machaddr` is implemented at
`ifacedecomp.rs:322` and `parse_c` at `grammar.rs:2292`, and both are used by the commands
that work. These are **inert plumbing**, the same finding #374 recorded, not engine ports.

### The `kassert` plane, which was meant to carry all of this

`P9 external-refinement`'s own `exposure` in `phases.toml` reads *"the console itself; kassert
(kuna) is the uniform writer"*. Today:

| `kassert` sub-phase | Dispatch arm | State at `e3db5512` |
|---|---|---|
| `naming-policy` | `Rename` | implemented — but **a silent no-op from the CLI** (below) |
| (option-backed sub-phases) | `Option` | works |
| `prototype-source` | `ProtoLock` | `engine_unavailable` |
| `type-propagation` | `Retype` | `engine_unavailable` |
| `flow-classification` | `FlowOverride` | `engine_unavailable` — *while `override flow` works* |
| `edge-virtualization` | `ForceGoto` | `engine_unavailable` |
| `switch-model` | `MultistageJump` | `engine_unavailable` |
| `merge-aggressiveness` | `Isolate` | `engine_unavailable` |
| `dead-definition-gate` | `DeadcodeDelay` | `engine_unavailable` |
| `code-data-partition`, `const-pointer` | `Unroutable` | routed to their native command |

Two CLI-level facts make `--kassert` unusable as it stands, both measured:

- `kuna decompile <bin> authenticate --kassert "p9 naming-policy v2 buf"` **exits 0 and changes
  nothing.** `build_script` emits `kassert` between `load function` and the *first* `decompile`,
  and a local like `v2` does not exist until a decompile has run, so the arm throws
  `No symbol named: v2` into a transcript the CLI discards. The one working assertion arm is
  dead on the one surface that can reach it.
- `kuna decompile ... --json --kassert ...` → `error: --kassert is not supported with --json`.
  The override surface and the machine-readable surface are mutually exclusive, which for an
  agent means the override surface does not exist.

**Verdict on the shared Hypothesis: `hypothesis_status` should move to `overturned` on all
three records.** The cheap half is much larger than filed (11 commands reach emitted C today),
and the expensive half is not "the stubs" — it is two *shipped* commands that lie (`map
return` panics, `override prototype` no-ops) plus one CLI ordering bug that makes the working
assertion arm inert.

## 3. The mechanism

### `--assert <directive> | @FILE`, repeatable

One flag, one line-oriented vocabulary, on every surface that loads a program. Each directive
is one line; `@FILE` holds many, with `#` comments — the `--define-function` file contract
verbatim, because that is what makes an override durable across invocations. kuna does not
write assertions back into the image; the file is the artifact, and it is text an agent can
generate, diff and version.

```text
kuna decompile ./a.out authenticate --json \
  --assert 'prototype authenticate int4 authenticate(char *user,char *pass)' \
  --assert 'type v2 char[16]' \
  --assert 'name v2 credbuf'

kuna decompile-all ./packed.bin --json --assert @notes/overrides.kuna
```

The vocabulary is **intent-keyed, not phase-keyed**. An agent should not have to know that
renaming is P9 to rename something; `--kassert <phase> <subphase>` stays as the raw writer and
is documented as such.

| Directive | Lowers to | Phase / sub-phase | Stage |
|---|---|---|---|
| `function <start>[-<end>][=<name>]` | `function bounds` | P1 | **shipped** (alias of `--define-function`) |
| `prototype <func> <C declaration>` | `parse line extern <decl>;` | P4 prototype-source | A |
| `param <i> <storage> <C typedecl>` | `map param` | P4 prototype-source | A |
| `return <storage> <C typedecl>` | `map return` | P4 prototype-source | A |
| `name <symbol> <newname>` | `rename` | P9 naming-policy | A |
| `type <symbol> <C type>` | `retype` | P5 type-propagation | A |
| `typedef <C declaration>` | `parse line` | P5 | A |
| `data <addr> <C typedecl>` | `map address` | P5 const-pointer | A |
| `label <addr> <name>` | `map label` | P9 | A |
| `comment <addr> <text>` | `comment instruction` | P9 | A |
| `readonly <addr>+<size>` | `readonly` | P1 code-data-partition | B |
| `volatile <addr>+<size>` | `volatile` | P1 code-data-partition | B |
| `global <addr>+<size>` | `global add` *(stub to wire)* | P1 code-data-partition | B |
| `flow <addr> branch\|call\|callreturn\|return` | `override flow` | P2 flow-classification | C |
| `goto <branch> <target>` | `force goto` *(stub to wire)* | P7 edge-virtualization | C |
| `jumptable <addr> <case>..` | `override jumptable` *(stub to wire)* | P2 switch-model | C |
| `isolate <symbol>` | `isolate` *(stub to wire)* | P6 merge-aggressiveness | C |
| `deadcode-delay <space> <n>` | `deadcode delay` *(stub to wire)* | P3 dead-definition-gate | C |

Deliberately **not** in the plane: `structure blocks` and `analyze range`. Both are real engine
ports (`BlockGraph` structuring, `ValueSetSolver`) and `no-cli-structuring-override` should
record them as a residue rather than have this design half-port them.

### Machine-readable in, machine-readable out

Every surface's `--json` grows one array, and every directive produces exactly one row:

```json
"assertions": [
  {"directive": "prototype authenticate int4 authenticate(char *user,char *pass)",
   "kind": "prototype", "phase": "P4", "subphase": "prototype-source",
   "status": "applied", "detail": null},
  {"directive": "name v9 credbuf", "kind": "name", "phase": "P9",
   "subphase": "naming-policy", "status": "rejected",
   "detail": "No symbol named: v9"}
]
```

`status` is `applied` or `rejected`; a rejection also goes to stderr on the human surface.
A rejected directive is **not** fatal by default (an agent batching forty renames against a
re-decompiled binary must not lose all forty to one stale name), but `--assert-strict` makes
any rejection exit non-zero. `--kassert`'s `--json` prohibition is lifted and its arms report
into the same array, so `kassert list` and the JSON agree.

### Where each directive is applied

The ordering is not cosmetic — it was measured, and getting it wrong is how this design fails
silently.

Script surface (`kuna decompile`, which forks `decomp_dbg` with a generated script):

```text
load file BIN
option ...                     # must precede `read symbols` (analysis commit)
read symbols
function bounds ...            # existing --define-function slot
parse line / typedef / data / label / readonly / volatile / global / comment
load function TARGET
prototype (parse line extern) / param / return / flow / goto / jumptable
decompile                      # pass 1 -- materialises the local symbols
name / type / isolate          # symbol-scoped: `rename v2` fails before pass 1
decompile                      # pass 2, only emitted when a symbol-scoped directive exists
print C
```

The two-pass shape is forced: `rename v2 buf` before the first `decompile` returns
`No symbol named: v2` (measured), which is precisely the bug that makes today's
`--kassert p9 naming-policy` inert. The second `decompile` is emitted **only** when a
symbol-scoped directive is present, so every existing invocation keeps its current cost.

In-process surface (`decompile-all`, `decompile-project`, WASM): the plumbing is already
built. `kuna_console::decompile_step::DecompileSeed` is the shared "console-only facts"
carrier and the whole-binary loop passes empty slices for every field this plane needs —
`mapped_symbols` (`data`), `usepoint_symbols` (`type varnode`), `dynamic_symbols`,
`pending_proto` (`prototype`), `flow_overrides` (`flow`), `mapped_params` (`param`). Stage A
fills them from the parsed directives. `decompile_step::decompile_one` **already** contains a
decompile → adjust → re-decompile loop (the `formatstring` half-B override loop), so the
symbol-scoped second pass has precedent in the exact function that needs it.

### Why not a `kuna console` passthrough

The captain's caution stands and the measurements support it. A passthrough would ship
`map return`'s panic and `override prototype`'s silent no-op straight to agents, would expose
37 commands of which ~14 are stubs, and would have no place to put the `assertions[]` report.
A typed vocabulary is what lets a directive be *rejected* with a reason instead of throwing an
unstructured console diagnostic into a discarded transcript.

## 4. The plan

Each stage is one PR, each closes one need, each is independently shippable.

**Stage A — `no-cli-rename-or-prototype-override` (the dispatched need).**
1. `kuna-cli/src/assertdecl.rs`: the directive parser + `@FILE`, modelled line-for-line on
   `funcdecl.rs`. Unit tests per directive, per malformation.
2. Fix `map return`'s `outtype null` panic; add a `kuna-console` regression test.
3. Diagnose `override prototype`'s no-op — the store side or the `queryCall` consume — and fix
   or, if it is a genuine engine port, record it as Stage-A residue on the need rather than
   silently shipping a directive that lies.
4. Script surface: emit the directives at the slots above, including the conditional second
   `decompile`; parse the transcript back into `assertions[]`.
5. In-process surface: fill `DecompileSeed`; add the symbol-scoped second pass to
   `decompile_one`.
6. `--assert` on `decompile`, `decompile-all`, `decompile-project`; lift the
   `--kassert`+`--json` prohibition; `assertions[]` in every `--json`.
7. `docs/cli.md`, the owning `docs/spec/` chapter, and `phases.toml`'s `exposure` prose for
   P4/P5/P9 (no new `settable` row — a CLI surface cannot change emitted C on its own, and the
   tooling track forbids one).
8. Acceptance `a-a58fc408288b` passes; promote to `tests/cli/`.

**Stage B — `no-cli-data-code-override`.** `readonly`/`volatile`/`global`/`data` directives;
wire the `global add`/`global remove` stubs onto `ScopeGlobal`; find a fixture where a
`readonly` range demonstrably folds (none of the ones tried here does) and file that need's
acceptance probe against it.

**Stage C — `no-cli-structuring-override`.** `flow` (works today), then wire `force goto`,
`override jumptable`, `isolate` and `deadcode delay` onto their ported engine entries, and fill
the matching `kassert` Dispatch arms. Update the `exposure` fields for the sub-phases whose
overrides stop being stubs. Record `structure blocks` and `analyze range` as residue.

Stage A is the load-bearing one: it builds the parser, the report, the ordering and the
`DecompileSeed` fill that B and C only extend. That is why the captain's "one design, three
needs" instruction is the right call — B and C collapse to a directive table row each plus a
stub to wire.

## 5. Risk and speed

**Speed.** Zero on every existing invocation: no directives means no extra console lines, no
second `decompile`, and an empty `assertions[]`. With a symbol-scoped directive the function is
decompiled twice — a bounded ~2x on one function, opt-in, and the same cost shape the
`formatstring` loop already pays. No pipeline pass is added, so `make test` / `make test-stages`
are structurally unaffected.

**Risk, ranked.**
1. *`override prototype` is a genuine engine gap, not a bug.* Then the `prototype` directive
   must lower to `parse line extern` only (which is measured working for the loaded function)
   and per-call-site prototype override becomes Stage-A residue. The acceptance probe is
   written against `parse line extern` for exactly this reason and is unaffected.
2. *Transcript-scraping the script surface is brittle.* `assertions[]` on the forked path is
   derived from console diagnostics. Mitigation: the console prefixes are documented as
   byte-faithful and load-bearing (`ifacedecomp::execute`), and `decompile.rs` already parses
   them (`CONSOLE_DIAGNOSTICS`, `arch_failure_reason`).
3. *A directive that is accepted and does nothing.* The failure mode of this whole family —
   `override prototype` is already an instance, and it is worse than an error. Every directive
   must ship with a test that asserts the emitted C **changed**, not that the command returned
   Ok. Reviewing on "did it return Ok" is how `override prototype` got here.
4. *Symbol names are unstable across runs.* `name v2 credbuf` binds to a generated name that a
   later kuna version may number differently ([[kuna-entry-name-rank-length]] is the same
   hazard one layer over). Mitigation: `name`/`type` also accept a storage form
   (`name [stack,-0x18,8] credbuf`), which is stable; the generated-name form stays as the
   convenient one.
5. *Blast radius.* Nothing here runs unless a directive is passed, so the parity corpora — which
   are symbol-less bytechunks driven by XML, with no CLI in the loop — cannot be affected.

## 6. Acceptance

Filed on the need at `a-a58fc408288b`, and runnable: `verify --acceptance-suite` reported it
`unrunnable` before this branch and now reports `passed: false` with a real clause list
(`exit_code` 2, `error: unknown option --assert`).

```
kuna decompile decompiler/crates/kuna-analysis/tests/fixtures/fauxware authenticate --json \
  --assert 'prototype authenticate int4 authenticate(char *user,char *pass)' \
  --assert 'type v2 char[16]' \
  --assert 'name v2 credbuf'
```

asserting `assertions` has 3 rows all `applied`, and that the C contains
`authenticate(char *user,char *pass)` and `char credbuf [16];` and no longer contains
`char v2 [8]`.

The target C is not aspirational. This is `decomp_dbg` at `e3db5512`, on the same in-repo
fixture, driven by the exact console script Stage A would generate:

```text
load file .../fauxware
read symbols
parse line extern int4 authenticate(char *user,char *pass);
load function authenticate
decompile
retype v2 char[16]
rename v2 credbuf
decompile
print C
--------------------------------------------------
int4 authenticate(char *user,char *pass) // return-dupe x2
{
  char credbuf [16]; // stack - 0x18
  int4 v1; // eax

  credbuf[8] = '\0';
  v1 = strcmp(pass,sneaky);
  ...
```

The engine already produces the acceptance output. Stage A is the path to it.

Sibling acceptance probes are **not** filed here. `no-cli-structuring-override`'s can be
written today against the measured `override flow 0x4006aa branch` result;
`no-cli-data-code-override`'s cannot, because no in-repo fixture tried here shows an
observable `readonly`/`volatile` effect, and filing an acceptance probe whose target was never
measured is the mistake this proposal exists to avoid. Stage B's first task is to find that
fixture.

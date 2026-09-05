## What was broken

> `override flow <addr> branch|call|callreturn|return` is fully ported and works — it is the
> one structuring override that is not a stub — and it is reachable only from the console. The
> `kuna` binary's `--assert` plane carries eleven directives (`function`, `typedef`,
> `prototype`, `data`, `param`, `return`, `comment`, `name`, `type`, `readonly`, `volatile`)
> and `flow` is not among them, so an agent driving the CLI cannot correct a misclassified
> call/branch/return at all.
>
> — `docs/re-needs/no-cli-structuring-override.md` (1 instance, severity major, track tooling)

kuna decides at P2 whether an instruction branches, calls, calls-and-does-not-return, or
returns. On an obfuscated or hand-written image it gets that wrong, and everything downstream
inherits the mistake — the function absorbs its neighbours, the body fills with dead
temporaries, the structurer gives up. The console has had the correction all along; the
binary an agent actually drives did not.

## Mechanism

A twelfth directive: **`flow [<func>::]<addr> branch|call|callreturn|return`**, at P2
flow-classification. No engine change — `kuna-decomp` is untouched by this PR.

* `kuna-console/src/assertions.rs` gains `Body::Flow`, **function-scoped**: a flow fact is
  consumed at flow time (the follower reads it while it is still deciding which bytes belong
  to the function), so it has to be a decompile *seed*, not a post-hoc edit. `seed_one`
  resolves the address in the default code space, maps the caller's word through
  `Override::string_to_type`, and parks the pair in `FunctionSeed::flow_overrides`.
* `kuna-console/src/project.rs` appends that vector to the flow overrides the decompile loop
  already carries — the analysis's `call error(nonzero,…)` no-return prunes. Appended
  **after** them on purpose: `insertFlowOverride` is a map store, so at an address both name
  the caller's fact wins. What was declared outranks what was inferred, the same rule the rest
  of the plane runs on.
* `kuna-cli/src/assertdecl.rs` parses the directive and lowers it to the `Slot::Function`
  console line `override flow <addr> <kind>`, which lands on the already-ported
  `IfcFlowOverride`.

**Both surfaces, because they are different code paths.** The hypothesis asked a builder to
confirm the CLI re-seeds the override the way the console does. It does — but not by the
console's route, and getting this wrong would have shipped a directive that works on
`kuna decompile` and is inert on `kuna decompile --json`, which is the surface the acceptance
probe uses. `--json` does not fork `decomp_dbg`; it loads in-process, where the console's
`pending_flow_overrides` stash is never consulted. Hence the two seedings. Measured emitting
byte-identical C on both, for all four spellings.

## Measured, on the need's own in-repo fixture

`aif_gap_x86_64` / `sub_13c9`: an indirect `call *%rdx` at `0x1405`, then twenty-four more
calls. Baseline is 55 lines, `v1`…`v25`.

| directive | emitted C |
|---|---|
| *(none)* | 55 lines, 25 `// eax` temporaries, a twenty-five-term sum |
| `flow 0x1405 return` | `unsigned int sub_13c9(void) { return dat_4014; }` — 4 lines |
| `flow 0x1405 branch` | re-read as a computed jump, **two-case table recovered**: `switch(dat_4014 & 1) { case 0: … case 1: … }` |
| `flow 0x1405 callreturn` | fall-through pruned: the call alone, nothing after it |
| `flow 0x1405 call` | the engine refuses — `Could not apply flowoverride` — reported as the function's error, exit 1 |

Four words, four distinct outcomes: the type is carried through, not defaulted. `call` is the
one the engine can refuse (an indirect call has no destination to make direct) and that
refusal is *itself* the proof the directive reaches the engine — a directive that reached
nothing would have decompiled the baseline and said nothing.

Controls: no directive ⇒ byte-identical output and an empty `assertions[]`;
`--assert 'flow 0x1405 goto'` exits 2 at parse time naming the four spellings; unqualified on
`decompile-all` is rejected with `name it as <func>::<operand>`, and
`flow sub_13c9::0x1405 return` binds and moves the C.

## The acceptance probe that now passes

`a-158ef4220dee` — the need's own, unchanged, all six clauses:

```
kuna decompile <fixture> --addr 0x13c9 --json --assert 'flow 0x1405 return' --assert-strict
  exit 0 · stdout is JSON · assertions[0].kind == "flow" · assertions[0].status == "applied"
  · code contains "return dat_4014;" · code does not contain "v25"
```

`verify --need no-cli-structuring-override`: **pass 1, fail 0, transition `closed`** (it failed
on all six clauses at `9d5ab78a`). Promoted verbatim to
`tests/cli/no-cli-structuring-override.json`; `make test-cli` is 25/25.

## Also

**`phases.toml` is byte-identical to `main`, and that is the one part of the brief this PR
does not deliver.** The need's hypothesis left the `exposure` call to the builder. The call:
P2 flow-classification's `command override flow; option noreturn` is now untrue, and should
read `command override flow; kuna --assert 'flow <addr> branch|call|callreturn|return'; option
noreturn`. That edit was made and measured — regenerates through `build.rs`, catalog stays
green, no count moves — and then **reverted**, because `mergecheck` shape-C guards
`phases.toml` as a keep-both table and rejects the disappearance of any line present on
`origin/main`. It cannot tell an edited row from a sibling's `[[settable]]` row eaten by a
merge, and that is the right default for that file. Defeating a merge guard for a prose line
is not a trade worth making; the replacement text is recorded as residue, and the additive
route (a new `[[surface]]` row, at the cost of renaming `surface_count_is_107`) is noted there
too.

Not closed, and deliberately: the three `engine_unavailable` structuring stubs this record was
narrowed away from (`force goto`, `override jumptable`, `structure blocks`). Each is real
engine work and the need's own decision log says to re-file them with a witnessed instance.

## Gates

| gate | result |
|---|---|
| `make test` | PARITY OK 675/675 |
| `make test-stages` | PARITY OK |
| `make rust-test` | green (7 new cases in `verify_assertflow.rs`, 1 new + 2 extended in `assertdecl`) |
| `make check-spec` | OK |
| `make test-cli` | 25/25 |
| `kuna catalog --check` | catalog OK |

No option row, no stages case, no catalog counters, no DIV row — a CLI surface cannot change
emitted C on its own; this one changes it only when a caller states a fact.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

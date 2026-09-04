# `xrefs` did not unify a PE import veneer with its IAT slot

Closes the RE-friction need
[`xrefs-unify-pe-import`](../../re-needs/xrefs-unify-pe-import.md) (major, 1 instance,
round 2, challenge `694fd2f60c16072f40f5a4b3`).

## What was broken

> `kuna functions --filter VirtualProtect` exposed both a thunk at `0x140019980`
> and an IAT symbol at `0x1400f4690` under the same name. xrefs to the thunk
> returned count 0, although decompilation calls VirtualProtect; only manually
> targeting the second alias address returned the two references.

The tester is right that the tool invited a question and then answered it wrong, and
`kuna` really does have two `VirtualProtect`s: `pe_iat` names both the **IAT slot** the
loader fills in and the **`FF 25` veneer** (`jmp qword ptr [slot]`) a direct `call` can
target. Two independent defects sat behind the symptom.

**1. An indirect flow op's operand was thrown away.** `data_refs` skipped `in0` for
every flow opcode on the grounds that it is the branch target and is already filed as a
Call/Jump edge. That is true of `BRANCH`/`CBRANCH`/`CALL` and false of
`BRANCHIND`/`CALLIND`: their `in0` is the varnode the destination is *read out of*, and
`classify` files no edge for it at all, so skipping it lost the reference outright. This
is exactly the import-veneer shape — SLEIGH lifts `JMP rm64` as `goto [rm64]`, one
`BRANCHIND` whose `in0` is the slot — so **every import veneer in every program
referenced nothing**. (`CALL rm64` survived only by accident: its SLEIGH constructor
spells `local dest:8 = rm64` first, and the `COPY` was scanned.)

**2. The two addresses of one import answered separately.** Which of them a call site
references is a compiler decision the question was not about, and answering per-address
makes the tool lie by omission in both directions. On the recorded crackme every call
site reads the slot, so the veneer reported 0; on the vendored `pe_imports.exe` `puts`
is called through the veneer, so the *slot* reports 0. Same bug, opposite address.

## The mechanism

`kuna-analysis/src/listing/xrefs.rs`, two changes, both confined to this read-only query:

- `BRANCHIND`/`CALLIND` leave the `is_target_slot` skip list. A `ram` `in0` is now the
  `read` of the slot it is.
- After the walk, each decoded function entry is probed for being a **forwarding
  veneer** (`veneer_at`): its control leaves through a `BRANCHIND` whose `in0` is a
  **decode-time constant** data-space address. That constant is what separates a veneer
  from a jump table — `jmp [rax*8 + table]` computes its address and lifts to a `LOAD`
  through a temporary, never to a `BRANCHIND` on a `ram` varnode — and it is why the
  relation needs no format knowledge (it finds ELF `.plt`, `.plt.sec` and `.plt.got`
  entries too). The bound is 2 instructions, enough for `endbr64; jmp *GOT(%rip)` and
  no more: measured over the whole fixture corpus nothing needs a third, and slack
  would widen the relation from *this function IS the jump* to *this function ends in
  one*, folding a tail-calling wrapper into its callee.
- `refs_to_unified` answers `--to` over the alias class (the connected component of that
  forwarding relation), excluding the forwarding jump itself — it is the callable's
  other half, not a caller of it, and excluding it is what makes both addresses answer
  identically. The class comes from the decoded jump, **never from a shared name**.

`refs_to` itself is untouched, so `kuna strings` and the `decompile-all` call graph read
exactly the edges they did before. `target.aliases` and a `# same import at …` line
disclose the other address, and every row still carries the real `to_address`, so an
agent can see which end each call site actually went through.

## Which references, and why they are the right ones

On the recorded binary the answer is **2**, and both rows are the real uses:

```
0x14000d800  read  sub_14000d560+0x2a0  MOV RAX,qword ptr [0x1400f4690]
0x14000d8f0  read  sub_14000d83f+0xb1   MOV RAX,qword ptr [0x1400f4690]
```

`kuna decompile --addr 0x14000d560` emits `v2 = VirtualProtect(v4[0],v6,v9,…)` — that
call site is `0x14000d800`. The veneer's own `JMP qword ptr [0x1400f4690]` at
`0x140019980` is **not** among them. Asking the slot `0x1400f4690` now returns the same
two rows, and `wcrtomb` — called through its veneer — goes from 0 to its 5 real call
sites at the slot address. `target.name` is still `VirtualProtect`, as the reproduction
probe requires.

## Acceptance

```
$ python -m scripts.repipe.verify --need xrefs-unify-pe-import
acceptance suite  sha 5fec5ff9
  PASS   closed         open           xrefs-unify-pe-import
total=1 pass=1 fail=0 closed=1 regressed=0 indeterminate=0
```

Promoted to `tests/cli/xrefs-unify-pe-import.json`. CI has no dataset, so the probe is
re-pointed at the vendored MinGW twin `pe_imports.exe`, which carries the identical
defect under the identical import name (veneer `0x1400079b0`, IAT slot `0x14000d234`,
count 0 before). The promoted clauses are **stronger than the recorded `count > 0`**,
because `count > 0` is satisfiable by wrong output: they pin the two rows to the real
sites (`0x140001a9e` in `__write_memory.part.0`, `0x140001cce` in
`_pei386_runtime_relocator`), pin `target.name` and the disclosed alias, and forbid the
veneer's own `jmp [slot]` from padding the count. `tests/cli`: 11/11.

## Evidence it does not over-fire, and changes nothing else

- **Every function entry of 12 ELF/Mach-O fixtures, before vs after** (332 queries):
  300 identical, 32 changed, **0 answers shrank**, 3 grew. Every one of the 32 is a PLT
  entry; no ordinary function acquired an alias. The 3 increases are each the
  weak-symbol guard `CMP qword ptr [__cxa_finalize@GOT],0x0` — a real reference that was
  invisible before.
- **All 200 function entries of `pe_imports.exe`**: 99 aliased, and every one is either
  an IAT slot or an entry whose first instruction is literally `JMP qword ptr [slot]`
  (checked by disassembly, including the odd-looking 11/18/51-byte ones, whose sizes are
  inventory distances, not bodies). Zero non-veneer aliases.
- **`decompile-all --json`, `strings --json` and `--reachable-from` are byte-identical**
  before and after over `pe_imports.exe`, `aif_gap_x86_64`, `fauxware`, `plt_aarch64`
  and `macho_imports`. No emitted C changes; `callee_of` maps `read`/`write` to no
  call-graph edge, so the new edges cannot reach the call graph.

## Track and shape

Tooling track: a wrong answer from a query surface, so per `docs/agents.md` this is a
strict bug fix and ships with **no option, no `phases.toml` row and no catalog
counter** — none were touched. The one behaviour change outside the need is that
`aif_gap_x86_64`'s `__cxa_finalize` now reports the guard read alongside the call; the
two `xrefs_cli` tests that pinned the old single-row answer are updated with that
reasoning in place.

## Gates

| Gate | Result |
|---|---|
| `make test` | **PARITY OK** — 675/675 assertions |
| `make test-stages` | **PARITY OK** — 594/594 assertions |
| `make rust-test` | green |
| `make check-spec` | green |
| `kuna catalog --check` | catalog OK |
| `tests/cli` (`scripts.repipe.clitests`) | 11/11 |
| `scripts.repipe.verify --need xrefs-unify-pe-import` | acceptance **PASS** |

Speed: `kuna xrefs` on the 986 KB / 4,168-function recorded PE, interleaved child CPU
time, min of 15 — **+2.57%** (1.3574 s → 1.3923 s) against a **−0.03%** null control
(before vs before through the same harness). The cost is the veneer probe: at most 2
extra decodes per decoded entry, against the hundreds of thousands the walk itself
does. Query surface only — no decompilation path runs it.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

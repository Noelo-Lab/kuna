## What was broken

RE-friction need `getprocaddress-result-discarded` (round 1, gated `admitted`; carried into
round 2), **1 instance**, severity **blocker**, challenge `65acadf3eef082e477ff5ede`
(`ObfuscationFiesta.exe`, PE x86-64, *Anti-debugging* + *Control-flow obfuscation*):

> **GetProcAddress result is discarded and an uninitialized pointer is called.**
> *What I wanted:* dataflow connecting the result of GetProcAddress to the subsequent
> indirect call. *What kuna did:* emitted `v12 = (void *)GetProcAddress(...)`, then called
> `(*v108)(v113,0,v27,0x30)` even though v108 was initialized to NULL and never assigned
> the result.

On `main` at `e4e30655`, `sub_140002530` renders:

```c
    v108 = NULL;
    v12 = (void *)GetProcAddress(v107,"NtQueryInformationProcess");
    if (v12) {
      v113 = GetCurrentProcess();
      v128 = v108;
      v7 = (*v108)(v113,0,v27,0x30);      // calls NULL
```

and a second site in the same function is worse — the call does not even go through the
NULLed local:

```c
    v98 = NULL;
    v12 = (void *)GetProcAddress(v17,"NtSetInformationThread");
    if (v12)
      (*v133)(GetCurrentThread(),v100,0,0);   // v133 is assigned nowhere
```

The dataflow underneath is *correct* — the `CALLIND`'s target Varnode really is the
`GetProcAddress` result — which is what makes this worse than a wrong-looking expression:
an analyst reads a null-pointer call, concludes the import is never used, and the
anti-debugging logic the whole challenge turns on is unreadable.

**Both filed diagnoses are refuted.** The tester guessed "stack-variable recovery split one
logical local into `v12` and `v108`". The captain's replay proposed that the second site's
`v133` is a *mis-attributed stack offset*. It is not: `v133` is `[rsp+0x260]`, a real slot
the disassembly writes at `0x140002bab` (`mov [rsp+0x260],rax` after `mov rax,[rsp+0x160]`),
dropped by the same mechanism one link further down the copy chain. One predicate fixes both
sites.

## Mechanism — `tiedstorekeep` (P3, new option, default **on**, DIV-105)

`mov [rsp+0x198],rax` lowers to an address-tied `stackvn = COPY(RAX)`. `RulePropagateCopy`
(`ruleaction.cc:3943`) then rewrites every reader of `stackvn` to read `RAX` directly.
Ordinary readers are pure gain — same value, and the store stays alive for whoever else
reads the location. A **marker** reader is not: an `INDIRECT` guarding the range across the
following `GetCurrentProcess` call, or a `MULTIEQUAL` at a join, never prints, so once the
marker has swallowed the last remaining reader the `COPY` has no descendants at all and dies
to dead-code elimination — taking the only statement that said where the local's value came
from. `Merge` normally conceals this by merging RAX's HighVariable into the tied location's;
here that merge is **declined** (intersecting covers — RAX carries a dozen other live call
returns in this function) and nothing repairs it.

Upstream already refuses this propagation when the `COPY` output is `addrforce` ("don't
propagate if we are keeping the COPY anyway"), but `addrforce` is set only on heritage's own
guard outputs, never on an ordinary frame store. `tiedstorekeep` widens that refusal by
exactly one case: a marker is about to consume the **last** reader of a non-`persist`
address-tied `COPY` whose input is not itself address-tied and whose value **comes from a
call** — `CALL`/`CALLIND`/`CALLOTHER`, or the `INDIRECT` that carries the return register
across the call site before `ActionActiveReturn` promotes it to the CALL's output.
Propagating there buys nothing (the marker is invisible either way) and costs the store.
`--option tiedstorekeep off` restores upstream's behavior exactly.

**Both narrowing clauses were bought by measurement, not by argument.** This PR starts from
the preserved wave-1 commit `2ad58462`, which had the right mechanism and an unshippable
predicate:

| predicate | `make test` | witness |
|---|---|---|
| `2ad58462` as committed (any address-tied COPY, any marker) | **668/675** + `copytrim` runs >10 min instead of 0.16 s | both sites fixed |
| + `!v.is_persist()` | 668/675, `copytrim` 0.16 s | both sites fixed |
| + value's def ∈ `{CALL,CALLIND,CALLOTHER}` | 675/675 | **neither** site fixed |
| + value's def ∈ `{CALL,CALLIND,CALLOTHER,INDIRECT}` ← **shipped** | **675/675 PARITY OK** | **both sites fixed** |

The 7 moved assertions are `Conditional Constant #11` and `Union #15/#16/#22/#23/#27/#28`:
keeping *ordinary arithmetic* stores alive turns three upstream if/else diamonds into `?:`
ternaries, which then degrades union field resolution (`res = ptr->b->bIntField` becomes
`(int4)ptr->a->aFloatField`). `copytrim`'s subject is precisely propagation out of `persist`
global stores, and a global already has heritage's `RETURN-COPY` (`addrforce`) keeping its
last store printed, so the brake has nothing to add there. `CPUI_INDIRECT` is in the set
because `RulePropagateCopy` fires *before* return-output promotion — without it, parity is
still clean and the bug is untouched.

## The acceptance probe now passes

```
$ python -m scripts.repipe.verify --need getprocaddress-result-discarded --json
counts {'total': 1, 'pass': 1, 'fail': 0, 'closed': 1, 'regressed': 0, 'indeterminate': 0}
  exit_code           True
  stdout_matches[0]   True     \(\*v\d+\)\(v\d+,0,v\d+,0x30\)
  stdout_absent[0]    True     (v\d+) = NULL;\n(?:(?!\s*\1 = )[^\n]*\n){0,10}?[^\n]*\(\*\1\)\(
```

Both sites read correctly on default output:

```c
    v104 = NULL;
    v104 = (void *)GetProcAddress(v103,"NtQueryInformationProcess");
    if (v104) {
      v109 = GetCurrentProcess();
      v124 = v104;
      v95 = (*v104)(v109,0,v22,0x30);
...
    v94 = NULL;
    v94 = (void *)GetProcAddress(v12,"NtSetInformationThread");
    if (v94) {
      v129 = v94;
      (*v129)(GetCurrentThread(),v96,0,0);
    }
```

(The redundant `= NULL` initialiser stays — the source really does write it, and the
acceptance deliberately does not assert it absent.)

## No `tests/stages/` testcase, and no promoted CLI probe — both recorded, not faked

**The defect is import-table-gated, and the XML `binaryimage` format has no PE import
table.** Three fixtures were built and measured; none reproduce it:

1. a synthetic SysV x86-64 construct (call → store to frame → NULL guard → second call →
   indirect call through the slot), assembled with `gcc`: byte-identical in both arms,
   because `Merge` succeeds and the store prints;
2. a 1 KB carve of the witness function at its **real** addresses (`0x1400027ac`–
   `0x140002bd0`) with a synthetic prologue and its `.rdata` mapped: both arms identical and
   already *correct* (`v23 = (code *)(*dat_140007048)(...)`);
3. the whole 6144-byte function likewise, with all 33 out-of-range branch targets stubbed and
   2 KB of `.rdata` mapped: same result.

Without the import table the calls carry no prototypes, so RAX never accumulates the cover
that makes `Merge` decline. A C-source reconstruction compiled at `-O0` was also tried and
only ever *moved* the store, never dropped it.

`scripts.repipe.verify --promote ... --force` likewise refuses: *"target.binary_source is
'dataset', not 'in-repo' — CI has no dataset"*. The need record's round-2 decision log
anticipates this and directs the builder to **say so rather than fake a test**, which is what
this section is.

## Corpus sweep (standing requirement 7)

7 binaries A/B'd whole-binary (`kuna decompile-all`, every function) across ELF
x86-64/i386/ARM and PE x86-64, diffed raw, with `v\d+` normalized, and as **sorted statement
multisets per function**:

| binary | result |
|---|---|
| `x86_64/fauxware`, `i386/all`, `armel/aes` | byte-identical |
| `x86_64/libgcc_s.so.1` | statement multiset **identical** — 5 hunks, all statement *reordering* |
| `x86_64/mv_-O2` | **4 of 390** functions change |
| `x86_64/ALLSTAR_9base_dd` | **1** function changes |
| `ObfuscationFiesta.exe` | the witness — 22 restored store statements, +19 lines |

In `mv_-O2`: declaration order, 7 redundant copy-shadow `vA = vB;` statements collapsing
where the kept store makes them redundant, one dereference hoisted into a temp, one local
eliminated outright in `quotearg_buffer_restyled`, and a net **+6 DWARF source names**
(`rpl_fts_open`'s `p` appears at 7 more sites; `backupfile_internal`'s `name` moves from a
register to a stack local). `rpl_fts_open` was re-checked statement by statement against the
gnulib source: the `root`/`tmp` list update is re-placed at the loop latch and is equivalent.
**Nothing was deleted, and no live assignment was dropped.**

**One degradation, named rather than hidden.** In `ALLSTAR_9base_dd sub_406290` — a
dtoa-shaped function with a stack slot type-punned between `double` and `unsigned long` —
the constant `v32 = 0xfffffff;` re-renders as `v32 = 1.32624736441264e-315;`, the same bit
pattern spelled as a float literal. The declared type is `unsigned long` in both arms;
keeping the store alive changes which datatype wins on that edge. One occurrence in 7
binaries.

## Speed

`scripts.pipeline.timeit --option tiedstorekeep`, witness function, median of 5:

```
speed: off=1919.13 ms  on=1916.67 ms  delta=-0.13%  (budget 5.0%) -> within budget
```

The predicate is four flag reads and a `lone_descend` on a path `RulePropagateCopy` already
walks, and it only ever *declines* work.

## Gates

| gate | result |
|---|---|
| `make test` | **PARITY OK — 675/675** (`docs/baseline.json` unmoved) |
| `make test-stages` | **PARITY OK — 600/600** (unchanged) |
| `make rust-test` | green |
| `make check-spec` | OK |
| `kuna catalog --check` | catalog OK |
| acceptance `a-f2df446f39d5` | **PASS** |

`scripts.repipe.mergecheck --against origin/main` reports **12 rejects, all of them
`assert-keepboth / line-added-twice` on `phases.toml`** — `[[settable]]`, `values = "on|off"`,
`strength = "HARD"` and the other nine boilerplate keys each appear one time more than on
`origin/main`. That is the structural signature of adding **one** `[[settable]]` row, not of a
keep-both merge: there are **zero `line-removed` findings** (the check that actually detects a
lost row), `docs/baseline.json` is byte-identical to `origin/main`, DIV-105 is free, and shape-B
re-derives every shared counter from a fresh capture and agrees (136 settables, tiers
(32, 54, 50), 230 corpus files, next ElementId 4138).

Bundle: `docs/features/getprocaddress-result-discarded/`.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

## What was broken

RE-friction round 2, need **`c-string-objects-become`** (major, 1 instance, challenge
`6547b4d50f4238b24302b588` — `trappy attack.exe`, PE x86-64, C++/MSVC):

> **C++ string objects become five-level pointer types in the checker**
> The checker is usable only after manually tracking fields through declarations such as
> `unsigned long long *****` and `char *****`. These false types dominate the 9.9 KB output
> and make it needlessly difficult to distinguish the input string from the expected string.

33 five-star declarations in one 350-line function.

## The filed hypothesis is REFUTED

> MSVC STL object layouts and constructor prototypes are not propagated into stack locals.

Nothing about this defect is C++, MSVC, STL, layout or prototype specific. It reproduces on a
**0x131-byte hand-built x86-64 ELF bytechunk** with three unnamed extern stubs and no C++
anywhere (`tests/stages/kuna-ptrdepthcap.xml`). The STL only supplies the *idiom*.

## What it actually is

A small-string-optimized object keeps **either** the characters **or** a pointer to them in the
same first 8 bytes, chosen on a capacity field. The compiler emits the two arms as one
MULTIEQUAL whose inputs are

* `PTRSUB(spacebase, -0xN)` — typed *pointer to the mapped local* by the spacebase arm of
  `propagate_add_in2_out`, and
* a LOAD from that very address — typed as *the local itself*.

That is the equation `T = ptr(T)`, which no finite type satisfies. `propagate_one_type` adopts
a type exactly **one pointer level deeper per pass**, and the only thing that stops it is the
empirical seven-pass settle ceiling in `ActionInferTypes::apply`.

Measured, not argued. Making that ceiling settable and sweeping it on the witness
(`0x1400011c0`) gives the maximum pointer depth in the emitted C:

| ceiling | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 12 |
|---|---|---|---|---|---|---|---|---|
| max `*` depth | 2 | 2 | 3 | 4 | 5 | 5 | 5 | 5 |

Tracing every candidate of depth >= 3 that `propagate_type_edge` adopted names the producer:
**1,368 INDIRECT, 958 MULTIEQUAL, 84 PTRSUB, 44 COPY, 42 PTRADD, 4 STORE, 4 INT_EQUAL** — and
every one of those PTRSUBs is `in0 -> out-1` off `register:0x20` (RSP), i.e. `&stack_local`,
one level deeper each pass. The markers are the *carriers*; the spacebase PTRSUB is the
*producer*.

## The mechanism

Upstream already has the rule. `TypeFactory::getTypePointerNoDepth` (`type.cc:1509`) refuses to
build a pointer whose target is a pointer-to-pointer, substituting `ptr(undefined<N>)`, and
returns an existing `ptr(undefined<N>)` unchanged — a **fixed point**. Ghidra calls it from
`TypeOpLoad`/`TypeOpStore::propagateType` only, and kuna's port of that (`propagate_to_pointer`)
is faithful. The escalation here does not go through LOAD/STORE, so the rule never runs.

`ptrdepthcap` calls that same rule at the one funnel every adopted type passes through
(`propagate_type_edge`, right before the `0 > type_order` adoption test), in a new
`p5_types/kuna_ptrdepth.rs`. The collapsed type is *less* specific than the concrete pointer
already held, so the order test rejects it and the lattice settles instead of escalating.
Depth 1 and depth 2 over a concrete base are untouched, so `char **argv` keeps its spelling.

No new pass, no new pass type, one new module, one gated call site.

## The acceptance probe now passes — and was STRENGTHENED first

`a-4ac515df8701` as filed asserted only `stdout_absent` of the two `*****` patterns with **no
`exit_code` clause**, so a crash, a timeout or a decompile that printed nothing would have
passed it. Before relying on it, it was tightened (never relaxed) and the tightening is recorded
in the need's Decision log:

* `+ "exit_code": {"eq": 0}`
* `+ stdout_absent: "\*\*\*"` — no three-deep pointer anywhere, not just the two spellings
* `+ stdout_matches` on all four string literals the tester had to hand-track:
  `"+184V38cC.TRAPPY-ATTACK0"`, `"Correct code, ur guess was: 33791"`, `"M28j2"`, `"24452"`

`scripts.repipe.verify --need c-string-objects-become` reports **PASS**, 8/8 clauses.

## Regression cover

* `tests/stages/kuna-ptrdepthcap.xml` — two-pass. `option ptrdepthcap off` reproduces the bug
  (7 `char ****` spellings: 6 declarations + 1 cast); `option ptrdepthcap on` settles it. Assert
  #2 pins **every** three-or-more-star spelling across BOTH passes to exactly those 7, so any
  leak back into the capped pass raises the count and fails.
* 8 unit tests in `p5_types/kuna_ptrdepth/tests.rs`, including the two that matter: the capped
  type is **idempotent**, and `ptr(capped)` collapses **back** one level — the two properties
  that make the propagation settle rather than oscillate.
* **`verify --promote` refuses this acceptance and that is correct**: its target is
  `binary_source: dataset` (`bin/trappy attack.exe`) and CI has no dataset — the same refusal
  PR #377 got. No in-repo fixture was fabricated with a different sha to get around it; the
  in-repo reproduction is the stages case above, which is worth having on its own merits.

## Delivery

Catalog default **OFF** (it changes inferred types and the XML datatest corpus pins the upstream
spellings while applying no mode), **ON in the `aggressive` preset** — which `auto` selects for
anything under 500 KiB, so the cap is the default rendering of `kuna decompile`,
`decompile-all`, `decompile-project` and the web front-end. This is the `ctypes` (DIV-75),
`voidtailreturn` (DIV-98) and `cortexmpriv` (DIV-99) delivery route, for the same reason.
`--option ptrdepthcap off` restores upstream's unbounded escalation byte for byte.

## Corpus sweep (standing requirement 7)

**90 binaries / 22,307 functions** A/B'd whole-binary (`decompile-all`, every function; RE dataset,
ELF + PE, x86 / x86-64, C and C++, compiler-built and obfuscated).

| | off | on |
|---|---|---|
| functions | 22,307 | 22,307 |
| functions whose C changes | — | **103** (0.46%) |
| `***` occurrences | 1,617 | **28** |
| functions carrying a `***` | 80 | **17** |
| **call sites lost / gained** | — | **0 / 0** |

The call check is the wrongness refutation, not a formality: for all 103 changed functions the
**call-name multiset is identical on both arms**. Nothing is deleted, nothing is moved.

98 of the 103 are type-only by statement skeleton. The 5 with statement-level differences were read
line by line; every residual is an **addressing spelling**:

* `*(char *)((long long)v + N)` becomes `v[N]`
* `v = (unsigned long long ****)((long long)v + N)` becomes `v = &v[N]`
* `if (*(char *)((long long)v + 3) != 'L')` becomes `if (v[3] != 'L')` — plainly better

**One degradation, named rather than hidden.** Where the object really *is* an array of 8-byte
pointers, the narrower element type re-renders a single 8-byte zero store as eight one-byte stores
(`v[0x48] = '\0'; ... v[0x4f] = '\0';`). **12 such lines, in 2 functions, out of 22,307.**

**The cap does not remove every deep pointer either**: 17 functions still carry one. Those are not
reached by `propagate_type_edge` (declared/locked types, pointer-relative types), so this option
cannot and does not touch them.

## Speed

Randomized-interleaved, min and median over 7–15 repeats, each run with an **on-vs-on control arm**.
That control is load-bearing: a *fixed* A/B/A order on this box reported **−29% between two
IDENTICAL arms**, so the ordering had to be randomized before any number meant anything.

| target | delta (min) | delta (median) | on-vs-on control |
|---|---|---|---|
| `crakersme.exe` `decompile-all`, 1,027 fns, **output byte-identical both arms** | **−0.12%** | +0.05% | −0.10% / −0.04% |
| witness binary `decompile-all`, 144 fns, **1 changes** | +5.28% | +5.32% | −0.06% / +0.50% |
| witness function alone (`decompile-all --addr`) | +8.47% | — | −0.44% / +0.91% |

Read together: **the per-edge hook is free** (the inert 1,027-function target is inside its own
control's noise), and the whole cost — about **40 ms** — is spent on the one function whose lattice
now settles instead of freezing at the seven-pass ceiling. It is paid exactly where the defect it
repairs is. `--option ptrdepthcap off` restores the old cost, and the old output, byte for byte.

## Gates

| gate | result |
|---|---|
| `make test` | **PARITY OK 675/675** — `docs/baseline.json` unmoved |
| `make test-stages` | **PARITY OK 605/605** (602 → 605, the new two-pass case) |
| `make rust-test` | green |
| `make check-spec` | OK |
| `kuna catalog --check` | catalog OK (139 settables) |
| `scripts.repipe.verify --need c-string-objects-become` | **acceptance PASS**, 8/8 clauses |

Registry: **DIV-108** (`docs/history.md`). Bundle: `docs/features/c-string-objects-become/`.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

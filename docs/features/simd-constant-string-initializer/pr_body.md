## What was broken

RE-friction round 2, need `simd-constant-string-initializer` (major, 1 instance), challenge
`653d88600f4238b24302b0ec` (macOS x86-64 `Crackme0x04`). The tester wanted

> Readable pseudocode for the initializer so I could recover the generated prompt, result
> messages, and serial without manually simulating SSE byte lanes.

and got, for `sub_100003790`:

> **SIMD constant-string initializer explodes into hundreds of scalar pseudo-operations.**
> The 752-byte initializer expanded into a very large body with pshufb, dozens of SUB161
> lane temporaries, and repeated byte stores.

15,532 bytes, 469 lines, 56 declarations.

## Two defects, and the filed one is the smaller half

The filed hypothesis stands: **`simdlane`** (P3, DIV-115). `pshufb` has no p-code semantics —
the x86 SLEIGH spec models it as an opaque `CALLOTHER` over a 16-byte value — so after
`ActionLaneDivide` splits the vector consumers into byte lanes, every lane read is a
`SUBPIECE` of something no rule can see through, and 32 `SUB161(v7,k)` temporaries all name
the same broadcast byte. The instruction is a pure permutation with an exact per-lane
definition once the mask is known, `dst[i] = (m[i] & 0x80) ? 0 : src[m[i] & (N-1)]`, so the
rewrite is an **identity**: the lane read is re-anchored on the shuffle's source.

But it is **half** the defect, and that was measured before any code was written: folding the
lanes makes the function 278 bytes *longer* (15,532 → 15,810), because the broadcast source is
a global the printer spells at each of its 96 uses. 45.8% of the bytes — 216 statements — was
**one 72-store epilogue emitted three times**. `print raw` shows three basic blocks over the
identical range `0x1000038d6-0x100003a5c`, and a sweep of all 153 non-default option values
moved it by at most 33 bytes, `returndup off` included.

**`retsplitglobal`** (P8, DIV-116) is the other half. `ActionReturnSplit::isSplittable`
(`blockaction.cc:2242`) admits `MULTIEQUAL`/`COPY`/`RETURN` and, for `COPY`, checks only that
no *input* is free — it never looks at the **output**. Upstream that is harmless because the
epilogue COPYs it was written for move registers; a COPY into a *persistent* Varnode is a
store to a global, and it is what P9 prints as `dat_<addr> = ...`. Both splitters share the
one predicate, which is why `option returndup off` changed nothing.

**A bound, not a veto — settled by measurement.** A plain `is_persist()` veto moved four stage
assertions. Three were `ghdec-returncopysplit`: `Heritage::guardReturns` inserts a synthetic
`glob = glob` liveness self-copy per persistent global before *every* return, so counting them
prices a bare epilogue by how many globals the whole program has (now excluded two ways, either
sufficient: the `return_copy` flag, and input/output naming the same storage). The fourth was
`gh180-implied-cover-scarry` — GNU grep `nlscan`, whose epilogue has **two** genuine global
stores, where declining also makes the merged form print `dat_2b818 = v2;` on a path the binary
does not store on. So the shipped rule counts the stores and declines past **8**, this phase's
house range (`crossjumprevert`/`dedupitetail` cap a duplicated leaf run at 8 printed ops,
`taildup` at 12). GH180's two still split, byte-identical to upstream; the witness's 72 do not.

## The acceptance probe now passes

```
python -m scripts.repipe.verify --need simd-constant-string-initializer --json
```

| clause | expected | actual |
|---|---|---|
| `exit_code` | `== 0` | 0 |
| `stdout_bytes` | `600 < n < 12000` | **11,264** |
| `stdout_matches[0]` | `void sub_100003790\(void\)` | matched |
| `stdout_matches[1]` | `100008050` | matched |
| `stdout_absent[0]` | `SUB161\(` | absent |

15,532 → 11,264 bytes; 32 `SUB161(` → 0; 30 declarations dropped; the epilogue emitted once.
With both options off the output is byte-identical to `main` (15,532).

## Sweep

90 RE-dataset binaries, 70 loaded under both arms (20 fail to load identically in both),
**15,365 functions**: **2 changed (0.013%)**, both `retsplitglobal`, both a pure
de-duplication —

| binary / function | base → new | change |
|---|---|---|
| `KeyVal2.exe` `sub_417ca0` | 1,474 → 1,021 B | 17 duplicate statement copies removed; **distinct statement set identical** |
| `libmodplug-1.0.so` `ModPlug_1SetPatternLoopRange` | 997 → 987 B | 2 duplicate `return;` removed |

Zero statements added anywhere, zero distinct statements lost, function sets identical on
every binary. `simdlane` changed **0** functions in the sweep — a constant-mask `pshufb` is
rare outside a vectorised byte loop, which is what makes it inert rather than risky.

## Gates

| gate | result |
|---|---|
| `make test` | **PARITY OK 675/675** (which is what qualifies both default-ON flips) |
| `make test-stages` | **PARITY OK 628/628** (620 + this feature's 8) — no per-test opt-out, no baseline re-pin |
| `make rust-test` | green |
| `make check-spec` | OK |
| `kuna catalog --check` | catalog OK |
| `make test-cli` | 29/29 |
| acceptance probe | **PASS** |

Speed on the witness, interleaved min/median of 11 alternating runs (measured while the workspace suite was running, so the median is noisy): **+0.19% min / -6.93% median** — no measurable cost.

## In-repo cover

`binary_source` is `dataset`, so `verify --promote` refuses. A new fixture
`decompiler/crates/kuna-analysis/tests/fixtures/simdlane_x86_64` (gcc -O3 -mssse3, source
beside it) reproduces the `simdlane` arm exactly — 32 `SUB161` with the option off, none with
it on — and `tests/cli/simd-constant-string-initializer.json` is pinned to it. The
`retsplitglobal` arm needs the witness's three-predecessor CFG, so **both** options are pinned
two-pass on the **real witness bytes** by `tests/stages/kuna-simdlane.xml` and
`tests/stages/kuna-retsplitglobal.xml`.

## Residue

- The `pshufb` CALLOTHER loses its last reader but is not removed (a userop is not
  dead-code-eliminable), so it stays as a bare `pshufb(ZEXT116(dat_100008040),0);` statement.
- AVX2's 32-byte `vpshufb` is two independent 16-byte permutations, not one N=32 permutation,
  so the width is deliberately not matched.
- A non-constant (table-driven) shuffle mask is untouched by design.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

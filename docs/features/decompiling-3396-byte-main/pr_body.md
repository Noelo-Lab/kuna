Attempt 4 on the RE-need `decompiling-3396-byte-main`. A round-2 tester recorded, as a
`major` finding on `nikos_crack_me.exe`:

> **Decompiling the 3396-byte main function takes about 68 seconds** — "The command
> produced no output for roughly 68 seconds. This observation is specifically about
> latency."

Attempts 1–3 (#380, #385, #393) took that witness 71.5 s → 19.4 s → 14.4 s → 11.8 s and
each closed saying the residue was flat. It is. This attempt asked what "flat" is *made
of*, and the answer turned out not to be a pass at all.

## What was measured

A per-Action exclusive timer plus 32 indexed guard slots — and, decisively, a per-Action
**counter** of varnode and op creations, because no timer can report a call count. One
`kuna decompile <crackme> sub_140023350 --json` run performs:

| | count |
|---|---|
| `VarnodeBank::create` / `create_def` | 1,677,343 |
| `VarnodeBank::destroy` | 1,523,008 |
| `VarnodeBank::xref` / `make_free` | 1,110,157 |
| `PcodeOpBank::create` / `destroy` | 1,106,775 |

≈ **9.5 M ordered-container mutations**, costing 3.6 s of an 11.8 s run. The function is
not slow because a pass scales badly; it is IR-volume-bound, and every unit of that
volume pays two `BTreeMap` insertions and later two removals. (`heritage` alone accounts
for 676,043 of the creations; `deadcode` for 857,187 of the destructions.)

## The mechanism

Four changes, none of which can move emitted C.

* **The tree keys stop carrying `Rc`s.** `LocKey`/`DefKey` held an `Address` and a
  `SeqNum`, i.e. two reference-counted space handles per key, cloned on every insert and
  dropped on every remove. They now hold the ordering triple those objects *compare* by —
  sentinel rank, space index, offset — flattened into integers by the new
  `Address::sort_key`. Both keys become `Copy`: no refcount traffic, integer comparisons,
  smaller nodes. The order is unchanged by construction, and an exhaustive-product unit
  test asserts `a.cmp(b) == a.sort_key().cmp(&b.sort_key())` over sentinels, four spaces
  (including two distinct `Rc` handles sharing an index) and five offsets.
* **`VarnodeBank::xref` takes one descent, not two.** Its "is an equivalent varnode
  already here" lookup and the insertion that follows were separate searches for the same
  key; they are now one `entry()`. Safe because the `insert` flag set afterwards is
  outside the `(input|written)` mask the key is built from, so it cannot move the entry.
* **`LocationMap::add` stops re-deriving.** It probed `lower_bound`, then walked to
  `begin()` to decide whether it could step back, then indexed the map again for the
  element's size — and its caller then re-looked that size up a fourth time. Asking for
  the *predecessor* first answers both cases in one descent, and `add` now returns the
  size it already has in hand. 469.6 ms over **1,427,964** calls, which was 71% of
  heritage's per-space varnode scan.
* **`rename_recurse` stops snapshotting whole blocks.** Filling a successor's in-edge phi
  slots reads only the block's *leading* MULTIEQUALs, then breaks; it was materializing
  every op of every successor once per CFG edge per heritage pass.

## Measurement

Interleaved paired A/B — one loop alternating the base and new binaries so both see the
same machine load (sibling builders live at load average 3–6 throughout):

```
pair1 base=13.58 new=11.14      base median  12,055 ms
pair2 base=12.10 new=10.93      new  median  11,225 ms
pair3 base=12.44 new=11.52      median delta    -6.9 %
pair4 base=12.04 new=11.18      paired mean     -7.3 % ± 5.0
pair5 base=11.97 new=11.83      8 / 8 pairs are wins  (4.2 σ)
pair6 base=11.87 new=11.27
pair7 base=11.79 new=10.77
pair8 base=12.07 new=11.89
```

This is **below** the perf track's 20% single-target noise bar and is reported as such.
It is carried on the pairing and the identity sweep instead: the noise-floor argument is
about a single-arm before/after, and here no pair regresses and the output does not move
one byte.

**Output identity:** whole-surface `decompile-all --json` diffed base vs new — stdout
*and* exit code — over the whole `kuna-analysis` fixture corpus plus the round's probe
binaries (ELF x86-64/i386/aarch64/arm/riscv64/mips/ppc64le/sparc64, PE x86-64/i386,
Mach-O x86-64/arm64/fat, COFF and ELF relocatables, UPX-packed, DWARF- and PDB-bearing
images; one of them an 18,032-function image). **0 differences.**

## The acceptance probe still fails, and the need stays open

`a-53d616afcb6a` asks for a median under 10,000 ms. This lands at ~11.2 s, so the probe
is **not** promoted into `tests/cli` — vendoring it would vendor a red test — and the
need is not closed.

**What would close it is now identified and priced.** kuna runs the jump-table partial
sub-decompilation **once per table**; C++ runs it once per function
(`Funcdata::stageJumpTable` guards the clone and the reduced pipeline behind
`if (!partial.isJumptableRecoveryOn())`). On this witness that is two partials —
3,077 ms of action time plus 464 ms of cloning — so sharing removes ≈1.7 s, ~15%.
Combined with this PR it lands the witness near 9.5 s, under the bar. It cannot ship
here: `option unrolledguard` recovers MSVC interleaved tables *because* each table's
partial re-clones the siblings recovered before it, and it fires on this very function
(`[kuna unrolledguard] interleaved jump table at 0x140023d90: skipped 6 undecoded
sibling-table case-target edge(s)`), so sharing changes this function's own output and
needs a `phases.toml` row — a lease the live `direct-address-keyboard-handler` builder
held all round. A fifth dispatch should be given that lease and told to implement exactly
that; it is a scoped feature PR, not another profiling run.

Three leads were **refuted by measurement** and are recorded so attempt 5 does not buy
them: `new_varnode`'s fresh `Rc<Datatype>` per varnode is 111 ms over 1.3 M calls (and
memoizing it would alias pointers that `Rc::ptr_eq` comparisons can see);
`setVarnodeProperties`' two scope containment queries are 86.5 ms over 539 k calls; and
`guard_calls`' loop body outside INDIRECT construction is 97 ms over 546 k iterations —
all 913 ms of `guard_calls` is the 192,528 INDIRECT constructions, i.e. tree mutations
again.

## Gates

| Gate | Result |
|---|---|
| `make test` | PARITY OK (675/675) |
| `make test-stages` | PARITY OK |
| `make rust-test` | green (see note) |
| `make check-spec` | OK |
| `kuna catalog --check` | catalog OK |
| `scripts.repipe.verify --need decompiling-3396-byte-main` | probe PASS, **acceptance FAIL** (~11.2 s vs <10,000 ms) |

> **Note on `make rust-test` in a repipe worktree.** The worker environment exports
> `KUNA_DECOMP_TEST`, and `verify_w10_proto_unlock`'s `cpp_oracle_bin()` reads that
> variable as the path to the *C++* oracle — so the test ends up comparing kuna against
> itself and asserts on a C++ spelling (`xunknown4`) kuna does not use. It fails
> identically on pristine `origin/main` sources in this worktree and passes with the
> variable unset, which is also why CI is green on main. The suite is green here with
> `env -u KUNA_DECOMP_TEST`.

> **Note on `make rust-test` in a repipe worktree.** The worker environment exports
> `KUNA_DECOMP_TEST`, and `verify_w10_proto_unlock`'s `cpp_oracle_bin()` reads that
> variable as the path to the *C++* oracle — so the test ends up comparing kuna against
> itself and asserts on a C++ spelling (`xunknown4`) kuna does not use. It fails
> identically on pristine `origin/main` sources in this worktree and passes with the
> variable unset, which is also why CI is green on main. The suite is green here with
> `env -u KUNA_DECOMP_TEST`.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

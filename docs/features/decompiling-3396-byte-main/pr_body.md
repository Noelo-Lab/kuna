## What was broken

RE-need `decompiling-3396-byte-main` (round 2, `perf`, 1 instance, challenge
`69a3822f7b3cc38c80464da4`). A tester reverse-engineering a PE crackme reported:

> **Decompiling the 3396-byte main function takes about 68 seconds.** The command
> produced no output for roughly 68 seconds, then emitted about 30 KB of highly
> noisy pseudocode. […] this observation is specifically about latency.

Attempt 1 (#380, merged) removed an O(N²) dead-list position scan in the lifter
and took the witness from 71.46 s to 19.42 s. The acceptance bar is a **median
under 10 s**, so the need stayed open.

## What this PR changes

`kuna decompile <bin> <fn>` drives the console with `load function <fn>` and then
`decompile`, and **each of those followed the same function's flow from scratch** —
the whole lift, the block build, and the per-jump-table sub-decompilation, twice.

Upstream never pays that: C++ `IfcFuncload` follows the flow once and
`IfcDecompile` re-runs the action pipeline on *that* `Funcdata` after
`Architecture::clearAnalysis` (`ifacedecomp.cc:889`). kuna rebuilds instead
because the facts a decompile is seeded with are consumed **at flow time** and
`load function` applies none of them: `map address` symbols and DWARF stack
locals, `type varnode` usepoint symbols, `map hash` dynamic symbols, a `parse
line extern` prototype, `map param` storage locks, `override prototype` call-site
overrides. So the rebuild is *required* exactly when one of those exists — and
pure waste when none does, which is every plain `kuna decompile`.

`decompile` now **adopts** the loaded IR when it can prove the rebuild would
repeat the same follow. Three independent guards must all hold
(`kuna-console/src/ifacedecomp.rs`, `PristineFlow`):

1. **Every flow-time seed is empty** — any one present means the loaded IR was
   followed without it.
2. **The architecture is configured as it was at the load** — a `Funcdata`
   snapshots the per-function flags into its ArchSeam handle when it is *built*,
   so a flag flipped afterwards is invisible to it. `formatstring`, the watchdog
   budget and ghidra-mode's staged recommendations all move between the load and
   the drive, and each refuses adoption.
3. **`decompile` is the immediately next command** — `load function` records
   `IfaceStatus::command_seq` and `decompile` requires it to have advanced by
   exactly one, with the same name, entry, declared extent and flow overrides.
   The counter is the whole invalidation story on purpose: an `option`, a
   `kassert`, a `map`, a second `load` — anything at all — advances it, so no
   command needs its own invalidation hook and none can be forgotten.

It is a pure-performance seam: the adopted `Funcdata` is the one the rebuild would
have produced, so the emitted C is byte-identical either way. No option, no
`phases.toml` row, no catalog counter, no DIV — nothing here can change emitted C.

Guard 2 is not hypothetical. The first cut had it missing and
`verify_decompile_all_parity` caught it: with `--option formatstring on` the ARM
fixture rendered `printf((char *)(dat_52c + 0x51c), a0, *a1)` instead of
`printf("%d %s\n", a0, (char *)*a1)`, because `formatstring` turns read-only
propagation on *around* the drive so the PC-relative literal-pool format constant
can be read, and the adopted IR had snapshotted the flag off.

## Measurement

`kuna decompile <crackme> sub_140023350`, interleaved A/B, same worktree, base
binary and new binary alternating in one loop:

| | median | min |
|---|---|---|
| base (`origin/main`) | BASE_MEDIAN | BASE_MIN |
| this PR | NEW_MEDIAN | NEW_MIN |
| | **DELTA_PCT** | |

Where it goes, from `Instant`-instrumented builds of the same tree:

| stage | before | after |
|---|---|---|
| `load file` + `read symbols` | 0.75 s | 0.75 s |
| flow follow #1 (`load function`) | 4.95 s | **0 s** |
| flow follow #2 (`decompile`) | 4.33 s | 4.33 s |
| action pipeline + emit | ~8.8 s | ~8.8 s |

Both follows were dominated by the jump-table sub-decompilation: the function has
2 tables, each staging a 68,370-op partial clone (~0.25 s) plus a reduced
"jumptable" pipeline over it (~1.7 s), and that ran **4 times** (2 tables × 2
follows) for 8.5 s of the 18.8 s run. It now runs twice.

## The acceptance probe still does NOT pass, and the reason is now measured

Acceptance `a-53d616afcb6a` asks for a median under 10,000 ms. This PR does not
reach it, and no further redundancy removal can:

- The **action pipeline plus emit alone is ~8.8 s**, and `load file` + `read
  symbols` is another 0.75 s. That is a ~9.6 s floor before a single jump table is
  touched. Even deleting *all* remaining jump-table work lands at ~10.6 s.
- The residue is not a bug, it is **scaling**. The witness is not a 3396-byte
  function: `kuna functions` derives sizes from the gap to the next entry, and the
  PE's own `.pdata` `RUNTIME_FUNCTION` says `0x140023350..0x14002dbe0` — **43,152
  bytes**, with `sub_140024094` and `sub_14002bb5c` being mid-function labels. The
  flow follow is correct; there is no overrun.
- Measured on `/bin/bash` (`--mode aggressive`, `decompile-all --addr`, load cost
  subtracted), pipeline time against function size:

  | size | alive ops | INDIRECT | MULTIEQUAL | varnodes | pipeline |
  |---|---|---|---|---|---|
  | 5,248 B | 3,233 | 1,004 | 1,047 | 4,715 | 1.6 s |
  | 9,184 B | 8,937 | 4,098 | 2,558 | 13,934 | 9.6 s |
  | 13,488 B | 30,697 | 17,142 | 9,714 | 49,382 | 30.7 s |

  The **IR itself** is superlinear in function size (ops ∝ size^~2.4, driven by
  INDIRECT: calls × heritaged locations, both O(size)), and pipeline time is
  ∝ ops^~1.5-2 on top of that. Closing the last 4 s is a constant-factor campaign
  across the rule pool, p6 merge, p5 infertypes, p3 heritage and p9 emit — a flat
  ~24/18/14/12/10 % split with no single hot spot — not another focused fix.

The full residue map, the profile that produced it, and the two remaining
structural leads (the per-table partial rebuild, blocked because kuna's
`unrolledguard` extension depends on it; and the O(log n) rule-pool cursor) are in
`docs/features/decompiling-3396-byte-main/record.json`.

## Tests

`decompiler/crates/kuna-console/tests/verify_flowreuse.rs` — 5 cases:

- the plain `load function` → `decompile` pair adopts (`adopted_flows == 1`) and
  renders **byte-identical** C to the same pair with an inert `echo` in between
  (which does not adopt);
- `load addr` stamps its follow the same way;
- a second `decompile` does not adopt (the stamp is single-use);
- DWARF stack locals — a flow-time seed present with *nothing* running in
  between — force the rebuild, isolating the seed guard from the counter guard;
- `formatstring` forces the rebuild, and the format string still resolves.

## Gates

| gate | result |
|---|---|
| `make test` | **PARITY OK** (675/675, `docs/baseline.json` untouched) |
| `make test-stages` | **PARITY OK** (`docs/baseline-stages.json` untouched) |
| `make rust-test` | RUSTTEST |
| `make check-spec` | check-spec OK |
| `kuna catalog --check` | catalog OK (no option added) |

Whole-surface regression sweep — `kuna decompile <bin> <addr> --addr` run under
the base binary and this one and byte-compared, stdout **and** stderr **and** exit
code, over every function of three binaries: SWEEPLINE.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

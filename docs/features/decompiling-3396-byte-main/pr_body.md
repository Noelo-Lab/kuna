## What was broken

RE-friction need `decompiling-3396-byte-main` (round 2, 1 instance, challenge `69a3822f7b3cc38c80464da4`):

> **Decompiling the 3396-byte main function takes about 68 seconds** (major)
> The command produced no output for roughly 68 seconds, then emitted about 30 KB of highly noisy
> pseudocode. [...] this observation is specifically about latency.

Four merged attempts took that witness 71.5 → 19.4 (#380) → 14.4 (#385) → 11.8 (#393) → 10.8 s (#396)
and the acceptance bar — a median under **10,000 ms** — stayed unmet. Five captains declined to re-cut it.

## The mechanism

Recovering a `BRANCHIND` means cloning the function's raw p-code into a partial `Funcdata`, building
its blocks and running the reduced `"jumptable"` action set over it, so the switch's index calculation
simplifies into something the emulator can walk. C++ `FlowInfo::recoverJumpTables` (`flow.cc:1429`)
builds **one** `partial` per call, and `Funcdata::stageJumpTable` (`funcdata_block.cc:512`) guards the
clone plus the reduced pipeline behind `if (!partial.isJumptableRecoveryOn())` — so upstream pays for
that sub-decompilation once per function however many `BRANCHIND`s it holds.

kuna paid it **once per table**. On the witness that is two partials: 3,077 ms of action time plus
464 ms of cloning inside an 11 s decompile.

This PR hoists the partial into `recover_jump_tables` as an `Option<Funcdata>` threaded through
`recover_jump_table_flow` into `stage_jump_table`, and builds it only when the slot is empty.
Recovery still runs per table against that shared partial and mutates it, exactly as upstream's does.

**The correctness half turned out to be the bigger one.** Because kuna's later clone re-cloned the
jumpvec *after* an earlier sibling had recovered, that clone's `collect_edges` called `target()` on a
case body decoded only into the parent flow, threw `"Could not find op at target address"`, and
truncated a perfectly recoverable dispatch to a computed `(code *)()` call. `option unrolledguard`
(DIV-13) exists **only** to tolerate that self-inflicted condition. Restoring the upstream shape
removes the cause rather than the symptom.

Shipped behind a settable row — `jtsharepartial`, `on|off`, **default on** — because it changes which
tables recover, and because `unrolledguard`'s per-table re-clone is precisely what it removes, so that
option needs `jtsharepartial off` to keep working. The in-code claim that kuna *cannot* share the
partial because sharing "loses 16 of the 17 switches" was tested rather than believed, and is
**refuted**: with sharing on and `unrolledguard` off, the memcpy witness recovers all 16 and emits 0
computed calls. The comment and the spec prose are corrected here.

## The acceptance probe now passes

`a-53d616afcb6a` (`kuna decompile nikos_crack_me.exe sub_140023350`, median `wall_ms < 10000`):

```
acceptance: PASS   exit_code 0, wall_ms median 9,112 ms  (bar: < 10,000 ms)
transition: closed
```

Interleaved paired A/B with the option itself as the lever (same binary in both arms, so the
`kuna decompile` → `decomp_dbg` fork cannot silently time a different engine), 8 pairs after a warmup
of each arm:

| arm | median | min |
|---|---|---|
| `--option jtsharepartial off` | 10,869 ms | 10,490 ms |
| default (`on`) | **9,138 ms** | 8,843 ms |

**−15.92% median**, paired mean −15.96% ± 2.03, **8/8 pairs a win** (~7.8σ), stdout byte-identical on
the witness. Five attempts: 71.5 → 19.4 → 14.4 → 11.8 → 10.8 → **9.1 s**. The bar was never moved.

## Blast radius — every differing function classified

Whole-surface `decompile-all --json` A/B (option off vs default) over **100 binaries / 21,458
functions**: the 6 round-2 probe binaries, 9 system ELFs, the 83 `kuna-analysis` fixtures
(ELF/PE/Mach-O/COFF; x86-64/i386/ARM/Cortex-M/PPC) and `betaflight_STM32F405.elf`.

**62 functions differ. 61 gains, 0 losses, 1 unclassifiable by name.** Aggregate over the differing
set: **68 truncated `(code *)()` dispatches removed, +237 `switch` statements, +14,770 `case` arms** —
with `unrolledguard` **off** throughout. Largest movers:

| binary | function | truncated | switch | case |
|---|---|---|---|---|
| `mcount_x86_64` | `__memcpy_ssse3_back` | 7 → 2 | 1 → 36 | 144 → 4,384 |
| `mcount_x86_64` | `__vfprintf_internal` | 0 → 0 | 6 → 16 | 158 → 606 |
| `mcount_x86_64` | `_nl_load_domain` | 13 → 0 | 1 → 14 | 5 → 70 |
| `KeyVal2.exe` | `sub_442390` | 0 → 0 | 1 → 15 | 4 → 55 |
| `system.exe` | `sub_14001d220` | 3 → 0 | 1 → 4 | 68 → 98 |

The classifier keys functions by NAME, so `mcount_x86_64` was re-run keyed by **address**: 19 differing
rather than 18 (two distinct `printf_positional` bodies collapse under one name), also all gains.

## Gates

| gate | result |
|---|---|
| `make test` | **PARITY OK 675/675** — `docs/baseline.json` unmoved (this is the default-ON evidence) |
| `make test-stages` | **PARITY OK 610/610** (609 → 610), baseline re-recorded |
| `make rust-test` | **green** (rc=0) |
| `make check-spec` | OK |
| `kuna catalog --check` | catalog OK (140 → 141 settables) |
| `make test-cli` | 24/24 |
| acceptance `a-53d616afcb6a` | **PASS**, 9,112 ms |

`make rust-test`'s first run reported two failures, both artifacts, both handled: a hard-coded
live-value count 45 → 46 (fixed here, alongside every other counter re-derived from a fresh capture),
and `w10_proto_unlock_const_return_collapses_no_tied_roundtrip`, which is a `KUNA_DECOMP_TEST`
environment artifact — that variable is exported in the builder environment and makes the test use the
Rust binary as its "C++ oracle", so the oracle renders `unsigned int` where the test pins `xunknown4`.
Green with the variable unset; nothing in this change can reach type spelling.

## Tests

- `tests/stages/ghangr-optimized-memcpy-6301a9.xml` grows a **third pass**: pass 1 both gates off = the
  bug (7 `code **`, 1 `switch`), pass 2 `unrolledguard on` = its fix (+16), pass 3 `unrolledguard` OFF
  + `jtsharepartial` ON = the shipped default's fix (+16). Asserts `code **` == 7 across the whole
  stream and `switch(` == 33, plus a three-pass sentinel so the counts cannot be one dropped pass.
- `tests/cli/decompiling-3396-byte-main.json` — the promoted regression probe. The dataset acceptance
  is unvendorable *and* wall-clock, so the in-repo twin pins the mechanism on the existing
  `mcount_x86_64` fixture: `__gettextparse` holds two tables in one batch and goes 1 switch + 1
  truncated computed call → 2 switches + 0, with 28 more case arms.

## Recorded as a convergence, not a DIV

`docs/history.md`'s own rule: *"a port defect whose fix moves kuna back onto upstream's default earns
no DIV row, but a corpus-wide output shift still needs a record for anyone bisecting."* After this
change kuna's default **matches** C++ `stageJumpTable`, so there is no divergence left to register.
The row is in the Convergences table. This is the one place the PR departs from the dispatch brief,
and it departs toward the document's stated rule.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

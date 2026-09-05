## What was broken

RE-need `cold-load-xref-lookup` (round 2, 1 instance, challenge `5bd1d1bb33c5d4110a29b31e`):

> **Cold-load xref lookup takes about four seconds on a 466 KB ELF.** The actual
> `kuna xrefs ./target/Obfuscation1 --to 0x80ba3d2 --json` invocation took 4.1303 seconds.
> Every independent query reloads analysis state.

The filed diagnosis — no persistent session — names the *workflow* cost, not the per-query one.
Measured, **one** cold query decoded the program **three times**, 154,608 instructions each: the
analysis-tier Listing walk, `operand_refs`' linear decode, and the reference index's own recursive
descent. A resident session would have amortised that; removing two of the three eliminates it.

Both extra decodes arrived because `--mode auto` promotes a sub-500 KiB image to `aggressive` — a
preset for the quality of emitted **C**, on a command that emits none.

## Why attempt 1 did not merge, and what changed

Attempt 1 (taken over here verbatim and rebased) reached −70% with byte-identical output but left
one blocker: dropping the Listing dropped the **AIF gap-walk** with it, and that is the only thing
the Listing contributed to a *reference* answer. A function reached only through a function-pointer
table has no inbound CALL edge, so no recursive descent finds it — on a stripped i386 PE that cost
`--to 0x43edfa` **61 of its 174 callers**, and `--from` on such a function answered `0`. Both are
now closed **without** restoring the second decode.

## Mechanism

1. **The query seeds its walk with the address it was asked about.** Walked *last*, after the
   seeded descent drains, so an address the natural walk already claimed is already in `decoded`
   and attributed exactly as before — the focus pass can only add coverage, never re-attribute an
   instruction another entry owns. An address that does not decode is dropped, not recorded as a
   function.
2. **The AIF gap-walk runs over the partition the reference walk itself leaves behind**
   (`Listing::from_partition`). The two facts it needs — which bytes are instructions, which
   addresses are functions — are exactly what the walk already computed; only the prologue mnemonics
   its fingerprint histogram reads are rendered, 2 per discovered function, against the whole
   program's worth the Listing path rendered. Its accepted entries are walked like any other seed.
   So `DriverDefaults::Query` takes the DIV-20/DIV-68 discovery flags and declines only the Listing.
3. **SLEIGH stops deep-cloning the matched `DisjointPattern` on every constructor node.** Only
   `Sleigh::instruction_mask` (the default-off `fid` pass) ever reads them, and it re-decodes under
   a flag it sets itself. That clone was **55% of every heap allocation the program made while
   disassembling**. With the per-instruction delay-slot context vector parked on the engine:
   **1,412,338 → 477,392 allocations, 228 MB → 72 MB.**

Plus attempt 1's own: lazy assembly rendering, the PIC-base pass carrying precomputed answers
instead of a clone of every instruction's p-code, and `PcodeCacher::emit` borrowing its input run.

No new option, no `phases.toml` row, no emitted-C change.

## The acceptance probe now passes

```
python -m scripts.repipe.verify --need cold-load-xref-lookup --json   →  passed = true
```

Promoted to `tests/cli/cold-load-xref-lookup.json`, repointed from the dataset image to the largest
in-repo fixture (CI has no dataset) with the bound recalibrated on it: `mcount_x86_64` measured
**1169 ms min before the change, 687-989 ms after**, so `min` of 5 < 1100 fails on the old answer (1169 ms) and passes on the new (687-989 ms).
`min` of 5 rather than `median` because this box is a 2-socket Xeon Silver 4316 whose turbo/base
split makes any single sample bimodal by 1.48x (652–672 ms or 933–1036 ms for identical work);
every number below is `taskset`-pinned and interleaved base/new.

| target | base | new | Δ |
|---|---|---|---|
| `Obfuscation1` (466 KB obfuscated i386 ELF, the witness) | 3573 ms | **715 ms** | **−80.0%** |
| `KeyVal2.exe` (stripped i386 PE) | 2047 ms | **799 ms** | −61.0% |
| `mcount_x86_64` (in-repo, 896 KB) | 1181 ms | **702 ms** | −40.6% |
| `pe_imports.exe` (in-repo, 499 KB) | 282 ms | **161 ms** | −42.9% |

Output on the witness is byte-identical; `KeyVal2.exe --to 0x43edfa` answers all **174** callers
both ways.

## Recall sweep

15 sampled entries × 2 directions on 4 binaries, comparing **row sets** (not counts) against the
full old discovery bundle. `Obfuscation1` 30/30 identical, `system.exe` 30/30 identical,
`KeyVal2.exe` 1 query short by 4 of ~1400 rows (was 5 queries and 61 rows before the gap-walk was
restored), `nikos_crack_me.exe` 1 query short by 134. Two residuals, both recorded in
`record.json`:

* the gap-walk over the reference walk's partition is not bit-identical to the gap-walk over a
  Listing's — the seeds and therefore the gaps differ, so a few speculative accepts differ;
* on an **x86-64 image under 500 KiB** the base answered with `aif` on, because it resolved
  `--mode auto` to `aggressive` — the exact C-quality-preset leak this need is about. The new
  default matches `kuna functions` and `kuna decompile-all` on that architecture (DIV-20: `aif` is
  off on x86-64). `kuna xrefs --option aif on` answers **more** than the base did there (1066 vs
  1044) in 600 ms against the base's ~1000.

## Gates

| gate | result |
|---|---|
| `make test` | **PARITY OK** 675/675 |
| `make test-stages` | **PARITY OK** |
| `make check-spec` | green |
| `kuna catalog --check` | catalog OK |
| `make rust-test` | 212 suites green except one **pre-existing** failure — `kuna-decomp::verify_w10_proto_unlock::w10_proto_unlock_const_return_collapses_no_tied_roundtrip`. Verified pre-existing by restoring `a13f570d`'s `kuna-sleigh/src/sleigh.rs` and re-running that single test alone: it fails identically. Post-rebase the full-workspace run was killed early three times on this box for reasons outside the tree (no failures, no build error, truncated mid-suite); the three crates this PR touches were then re-run in full and are green — `-p kuna-sleigh -p kuna-analysis -p kuna-cli`, 0 failed. |

New cargo coverage: `xrefs_cli::a_function_no_descent_reaches_still_answers_for_itself` and
`xrefs_cli::the_gap_walk_finds_call_sites_a_descent_cannot_reach`, both on the vendored
`cortexm_aifcorroborate_le32` — `0x800039c` is reachable only through a data path, and `0x8000160`
is called both from code every descent reaches and from inside that function.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

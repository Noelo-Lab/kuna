## What was measured

RE-friction need `decompiling-3396-byte-main` was re-filed as `regressed` on the theory that
#406 cost this witness ~680 ms. It did not. Two builds, `fba4ebd8` and `96224463`, each with
its **own** `kuna` *and* `decomp_dbg` (the CLI forks the engine, so pinning only `kuna` times
the other arm), 8 interleaved pairs of the acceptance command:

```
pre  #406   median 9,619 ms   min 9,146   max 10,903
post #406   median 9,742 ms   min 9,111   max 11,740
            paired mean +3.00% ± 6.77 → 0.4σ, median-of-medians +1.28%
```

and both builds emit the same 120,063 bytes (`sha256 8ee55baf…`), so #406 never fires here.
The named follow-up — `simdshufflelane` registered unconditionally in the `analysis` rule
group, i.e. schedule cost that grows with every default-ON pass — was ablated by deleting its
`rrow!` line and rebuilding: **+0.23% ± 5.33**. One more rule on `CPUI_SUBPIECE`, one of the
hottest dispatch lists there is, is not measurable on this function.

Six 7-8 sample medians taken this session on one build ranged **9,002–9,742 ms** against the
10,000 ms bar, and the acceptance suite passed at 9,199 and 9,444 ms with one failure between
them. The margin is ~7% and the box's spread is ±10%: the need is measurement-limited. The bar
is not moved.

## What ships

Not a fix — the instrument that produced those numbers, which five prior attempts each
rebuilt as a throwaway patch and threw away:

```
$ KUNA_ACTION_PROF=/tmp/prof kuna decompile nikos_crack_me.exe sub_140023350
$ head -6 /tmp/prof
total_exclusive_ms 8853.6
    1205.8 ms   13.62%         68 calls  decompile/oppool1
    1180.3 ms   13.33%         20 calls  decompile/heritage
    1083.5 ms   12.24%         27 calls  decompile/deadcode
    1065.3 ms   12.03%         20 calls  decompile/infertypes
     601.4 ms    6.79%          5 calls  jumptable/heritage
```

- Time is **exclusive**: a group is charged only what it runs outside its children, so rows sum
  to the schedule's wall time and a container cannot hide a leaf. A sampler cannot do this, and
  `perf` is unavailable on the machines this engine is tuned on.
- Rows are keyed by **root variant**, which separates the reduced `jumptable` pipeline running
  on a partial clone from the function's own `decompile` pass. The label comes from
  `ActionDatabase::set_current`, not from the root Action — `clone_filtered` keeps the universal
  tree's name, so every row otherwise reads `universal/…`.
- **Call counts**, because "68 calls of `oppool1`" is the fact a flat profile turns on and no
  sampler reports.
- Inert unless `KUNA_ACTION_PROF` is set: one cached read per `apply`, and `apply` is coarse.
  Measured off-cost on the same witness, 8 interleaved pairs: **+0.89% ± 3.84** (i.e. nothing),
  stdout byte-identical.

No option row: this cannot change emitted C. It follows the existing env-gated debug-hook
convention (`KUNA_DEFAULT_ON`, `KUNA_SYMBOLNAMEBOUND`).

## Tests

Four unit tests in `infra/actionprof/tests.rs`: a parent is not charged its child's time, two
roots do not share a row, `render` sorts by cost and totals, and an unbalanced `leave` is inert
rather than a panic. `make test` PARITY OK 675/675 · `make test-stages` PARITY OK 628/628 ·
`make rust-test` green (352 targets) · `make check-spec` OK · `kuna catalog --check` OK ·
`make test-cli` 29/29 · acceptance `a-53d616afcb6a` PASS at a 7-rep median of **9,227 ms**
(8,863–9,484).

🤖 Generated with [Claude Code](https://claude.com/claude-code)

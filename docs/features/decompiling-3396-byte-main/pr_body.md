## What was broken

RE-need `decompiling-3396-byte-main` (round 2, track `perf`, 1 instance,
challenge `69a3822f7b3cc38c80464da4`):

> **Decompiling the 3396-byte main function takes about 68 seconds** (major)
> The command produced no output for roughly 68 seconds, then emitted about 30 KB of
> highly noisy pseudocode. […] this observation is specifically about latency.

Reproduced at **71.5 s median** for `kuna decompile <bin> sub_140023350`.

## The cause is not what was filed

The filed hypothesis was "opaque arithmetic and indirect calls make an analysis pass
scale poorly". The symptom stands, the diagnosis does not: gdb-sampled profiling put
**53% of wall time in one leaf frame, `FlowInfo::xref_control_flow`** — the *lifter*,
before any analysis runs. This function is unusual only in op count (48,169 raw ops,
37,710 of them INDIRECTs across 365 call sites), and op count is exactly what the real
defect squares.

**The dead list is a doubly-linked list whose links three call sites refused to use.**
The C++ caches a `std::list<PcodeOp*>::iterator` on every op (`insertiter`), so "the op
after this one" and "the last op" are O(1). kuna's dead list *is* that list —
`op.rs (IntrusiveList)` keeps prev/next `OpId` links on each op — but position was being
re-derived by scanning:

| site | what it did | how often |
|---|---|---|
| `FlowInfo::dead_next` | scanned all of `iter_dead()` to locate one op | once per emitted p-code op |
| `FlowInfo::dead_tail` | `iter_dead().last()` | once per decoded instruction (the marker idiom) |
| `FlowInfo::delete_remaining_ops` | collected the whole list to find a suffix | once per terminating instruction |
| `Funcdata::op_target` | rebuilt an entire `BTreeMap` predecessor index | once per call |

The list grows to the whole function, so this is O(N²) in op count. It is invisible on
small functions and quadratic on the ones an RE actually cares about.

## The mechanism

1. **`op.rs`** — expose what the list already carries: `PcodeOpBank::dead_front` /
   `dead_back` / `dead_next` / `dead_prev`. Membership is the `dead` flag **plus a live
   link**, because `destroy()` retires an op to `deadandgone` with the flag still set and
   the alive list shares the same link pair; a non-member reports `None`, exactly what a
   scan for an absent op returned.
2. **`flow.rs` / `funcdata_op.rs`** — the four sites above use those accessors.
3. **`action.rs`** — `ActionPool::advance_op_state` already ran the optree search that
   decides `After`/`Done`, so it now keeps the successor's `OpId` and `current_op` reads
   it instead of searching a second time for the op the advance already found. The memo is
   dropped on every `apply()` exit, so a resumed or interleaved sweep re-searches.

No option: none of this can change emitted C, so `docs/agents.md`'s flag rule does not
apply. `phases.toml`, `options.rs`, the catalog counters, `docs/options.md` and
`docs/history.md` are untouched.

## Measured

Interleaved A/B (base and new binaries alternating in one loop; machine load 5–7 from
sibling builders, so both median and min are reported):

| run | base | new | delta |
|---|---|---|---|
| `kuna decompile <bin> sub_140023350` (the probe), median of 5 | **71.46 s** | **19.42 s** | **−72.8%** |
| same, min of 5 | 69.14 s | 18.44 s | −73.3% |
| `kuna decompile-all --addr 0x140023350`, median of 3 | 40.05 s | 14.24 s | −64.4% |
| `kuna decompile-all /bin/gzip` (145 functions), median of 3 | 14.55 s | 13.60 s | −6.5% |

The base and new distributions do not overlap: max(new) = 23.35 s < min(base) = 69.14 s.
The gzip row is the honest control — small functions barely feel a quadratic in op count.

**Output is byte-identical.** Whole-binary `decompile-all --json` captures from an
interleaved before/after build pair compare byte-for-byte on three binaries / 509
functions: the probe crackme (322 fns, 5,653,602 B), `/bin/gzip` (145 fns, 1,196,332 B),
`/usr/bin/xxd` (42 fns, 135,319 B). The 120,052-byte witness function is identical too.

## The acceptance probe does NOT pass — read this part

The need's acceptance asks for a **median under 10 s**. This lands at 19.4 s. The probe
arm (`> 30 s`, i.e. the filed bad behaviour) no longer holds, so the symptom is gone, but
the threshold is not met and **the need should stay open**.

After this fix the profile is flat — p6 merge/ScopeLocal 27.6%, p3 heritage 26.3%, the
rule pool 19.8%, jump-table sub-decompilation 17.4%, p9 dead-code/emit 13.7% of a 12.2 s
in-process run. Nothing left is worth the further 48%; closing that gap is a campaign
across kuna's core indexes, not one focused change. Two concrete leads are recorded in
`docs/features/decompiling-3396-byte-main/record.json`:

- **`kuna decompile` follows flow twice** (~18% of the probe). `follow_flow_on_fd` is hit
  2× and `stage_jump_table` 4× on the console path versus 1×/2× in-process, because
  kuna's `IfcDecompile` rebuilds the `Funcdata` from scratch where the C++ calls
  `clearAnalysis` on the one `IfcFuncload` already followed. Not local: the rebuild exists
  so the seeds (mapped symbols, DWARF locals, prototypes, param maps, overrides) apply at
  flow time.
- **The jump-table partial clone is rebuilt per table.** Upstream builds it once
  (`flow.cc:1437`, guarded at `funcdata_block.cc:513`). Implemented, measured at
  18.0 s → 14.3 s — and **reverted**: it takes `tests/stages/ghangr-optimized-memcpy-6301a9.xml`
  from 2/2 to 0/2, losing 16 of 17 interleaved MSVC switches, because `option unrolledguard`
  recovers them precisely *because* each table's clone re-clones its already-recovered
  siblings. It changes which tables recover, so it needs its own option — and this round's
  `phases.toml`/catalog leases are held by a sibling builder. Left documented in the code
  and in `docs/spec/02-lift-and-flow.md` §2.3.

No `tests/cli/` probe was promoted: the acceptance does not pass, and its target is a
dataset binary (`scripts.repipe.verify.vendorable` refuses anything not in-repo). The
regression guard is a cargo unit test on the cursor semantics instead of a wall-clock
assertion on a CI runner.

## Gates

| gate | result |
|---|---|
| `make test` | **675/675 PARITY OK** (`docs/baseline.json` not re-pinned) |
| `make test-stages` | **597/597 PARITY OK** (`docs/baseline-stages.json` not re-recorded) |
| `make rust-test` | **green**, 342 `test result: ok` — run with `env -u KUNA_DECOMP_TEST`, see below |
| `make check-spec` | check-spec OK (lenient mode) |
| `kuna catalog --check` | catalog OK |

New tests: `substrate::op::tests::dead_cursor_matches_a_full_scan` (the O(1) cursor agrees
with a full `iter_dead()` scan at every position, and reports an alive op, a destroyed op
and a single-element list exactly as the scan did) and
`first_after_seq_id_agrees_with_first_after_seq`.

> **Harness note:** the RE-pipeline worker environment exports `KUNA_DECOMP_TEST` pointing
> at kuna's *own* `decomp_test_dbg`, which makes `verify_w10_proto_unlock`'s
> `cpp_oracle_bin()` compare kuna against kuna and fail on the C++ oracle's `xunknown4`
> versus kuna's DIV-6 `unsigned int`. That failure is the env var, not the change.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

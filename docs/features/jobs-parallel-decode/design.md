# `kuna --jobs N` parallel discovery decode — final design (2026-09-12), annotated as built

Synthesis of a three-design panel (partition / shared-claim / prefetch) and three judges. The
winner is **address-interval ownership with bulk-synchronous reconciliation** ("decode
lanes"), with the runner-up's engine kit, context snapshot, image precondition, flat shards and
self-check grafted in. Full panel material: `DESIGN-partition.md`, `DESIGN-shared-claim.md`,
`DESIGN-prefetch.md`, `JUDGES-pdecode.json`, and the reader map `UNDERSTAND-pdecode.txt`.

## What it does
`kuna functions|decompile-all|decompile-project|decompile-graph <bin> --jobs N` runs the
whole-binary function-discovery walk (`kuna-analysis/src/listing/walk.rs`) on N threads inside
the parent process. Byte-identical output on every surface where it engages; the serial walk
runs unchanged wherever the gate refuses. On the 147 MB x86-64 target the walk is 71 s of a 80 s
load (decode 59 s); projected total at 16 lanes ~18-20 s (~4x), ~15 s after the follow-up that
deletes the merge.

## The algorithm (measured exactly-once, 18/18 runs byte-identical in a serial simulation)
* The sorted seed list (392,814 entries on the target) is cut into K = 32·L address intervals;
  `owner(addr)` is a binary search over K+1 bounds. **A lane decodes only addresses it owns.**
  A successor outside the lane's interval (branch target, fall-through, or an admitted CALL
  target) is routed to the owner's inbox instead of being decoded locally.
* Bulk-synchronous rounds: lanes claim intervals from an atomic cursor (largest-first), run each
  to local quiescence, bucket their crossings into the owners' inboxes at the barrier, and loop
  until a round delivers nothing (measured 2 rounds on the target, 2-3 on libLLVM/gdb, 7-8 on a
  seed-starved image). After round 1 only intervals with non-empty inboxes are claimed.
* One decode body: `walk::step()` extracted from today's inner loop; the serial `walk()` and
  the lanes both call it through a `Successors` sink (serial: push to its worklists; lane: route
  by owner). No second copy of the per-instruction policy can drift.
* Per-lane state is plain data: `Vec<(u64, InsnLite)>` (Insn minus `pcode`, which is the sole
  `!Send` field), `Vec<(u64, DiscoveredFunction)>`, refs buckets, a `visited` set, two inboxes.
  A compile-time `Send` assertion guards it; `model.rs` documents why `Insn` is `!Send`.
* Reconcile (serial): sort each shard, `BTreeMap::from_iter` over the ascending concatenation
  (disjoint by construction; a collision in release discards the shards and re-walks serially),
  `funcs` = seed records pre-inserted (seed beats discovered, exactly walk.rs:132-135) then
  every shard's discovered records via `entry().or_insert`, refs concatenated then the unchanged
  `finalize_refs` sort+dedup. Measured 3.7 s on 20.2 M records; a follow-up PR shards
  `Listing.insns` behind its six query sites and deletes it.

## Why it is byte-identical (the argument the PR body must carry)
Under the gate, `decode_one(a)` is a pure function of (address, image bytes, context values):
no `ContextCommit` in the loaded `.sla` means `apply_commits` (sleigh.rs:2048) never writes the
context database; the context cache re-fetches the blob on every hit; the parser-context pool,
`PcodeCacher` and `ctx_vec` are capacity-only reuse cleared per use. The serial walk's output is
the least fixpoint of a monotone system over a finite lattice (seeds ⊆ F, admitted CALL targets
⊆ F, F ⊆ I, successors ⊆ I); the lanes evaluate the same equations, each address by exactly one
lane, with fair delivery at every barrier — chaotic iteration converges to the same fixpoint.
The two first-writer-wins lines are neutralised: `insns.contains_key` (walk.rs:145) has no race
because ownership is total; `funcs.entry().or_insert` (walk.rs:190) inserts a constant
`discovered(t)` record, and seeds are pre-inserted. A failed decode (walk.rs:151-158) records
nothing in both walks; only wasted work differs. Nothing downstream sees the schedule: every
consumer iterates `BTreeMap`s in address order and `fast_funcdisc` sorts+dedups again.

## The gate (all-or-nothing, computed once before any thread spawns; refusal = serial walk)
| | predicate | why |
|---|---|---|
| G1 | lanes ≥ 2 | `--jobs 1` is today's code path, not "parallel with one lane" |
| G2 | the translator is a `Sleigh` | the Ghidra bridge is an RPC, not a rebuildable engine |
| G3 | **the loaded `.sla` has no `ContextCommit` constructor** (`SleighBase::has_context_commits() == false`) | the correctness gate; a property of the language, not an arch list. x86/x86-64/AARCH64/RISCV/Sparc/SuperH/Z80 pass; ARM/MIPS/PowerPC/PA-RISC/PIC/M16C/… refuse |
| G4 | `max_delay_slot_bytes() == 0` | a delay-slot decode fetches past the instruction inside one `one_instruction`; Sparc waits |
| G5 | the parent loader answers `shared_bytes()` (an `ObjectLoadImage`) and the `ContextPainter` is empty | lanes must read the parent's LIVE patched bytes (dynrelocs, `--assert bytes`), never re-read the file |
| G6 | **image precondition: every address of every exec range `[lo, hi)` is mapped by some segment** | `ObjectLoadImage::load_fill`'s 512-byte window is history-dependent in exactly one case: a fetch whose FIRST byte is unmapped (`fill_span` breaks only at `offset == 0`, loadimage_object.rs:1081/1092; a covering window returns zeroes where a direct fill returns `Err`). Every fetch the walk makes starts at an `in_exec` address, so with G6 no fetch can start unmapped and the window is unobservable. Belt and braces: each lane carries a `saw_unmapped_first_byte` tripwire; if it ever fires, discard and re-walk serially |
| G7 | exec bytes ≥ 8 MiB, overridable by `KUNA_DECODE_MIN_BYTES` (tests set 0) | 2×100 ms of engine builds must not dominate a small walk; the override is what keeps the equivalence tests NON-vacuous |
| G8 | not wasm | the WASI front-end has no threads |
Observability: with `--jobs > 1` exactly one stderr line, `[kuna --jobs] decode: 16 lanes, 512
intervals` or `[kuna --jobs] decode: serial (<reason>)`; nothing printed when `--jobs` is absent.
Post-walk assertion: the parent's context region list is unchanged.

## The per-lane engine kit (built ONCE per `run_listing_consumers`, reused across rebuilds)
A bare `Sleigh` per lane (not an `Architecture`; ~100 ms, ~48 MB, built concurrently) from an
`EngineRecipe { sla: Arc<[u8]>, truncations, … }` captured where the parent's engine is built,
plus `install_register_lookup`, plus the parent's context values restored by COPY through a
`ContextValueSnapshot` (default blob + every split point via `get_context_bounds`, replayed
ascending with `set_context_region`; a `ContextDatabase` cannot be cloned — `FreeArray::clone`
zeroes the explicit-set mask — and `encode()` is blind to the default blob, so no fingerprint),
plus `allow_context_set(false)`, plus a `SharedBytesImage` view (Arc-shared segment bytes, its
OWN 512-byte window, its OWN `Rc<AddrSpace>` — space identity is `Rc::ptr_eq`). Lane startup
compares 1,024 sampled decodes against the parent; `KUNA_DECODE_SELFCHECK=1` re-decodes every
address with the parent's engine (or runs both walks and compares field by field, returning the
serial result) — the acceptance evidence for the feature PR, not a follow-up.

## The knob
`--jobs N|auto` is accepted by `functions` too (rewrite the stale comment at
decompile_all.rs:2550, do not delete it); `--jobs-chunk/--jobs-full-load/--jobs-worker` stay
batch-only. `apply_loadtime_env` bridges it as `KUNA_DECODE_JOBS=N` for the duration of
`load_program` (the existing RAII guard restores it before the pool spawns); `jobs.rs` also
forces `KUNA_DECODE_JOBS=1` in worker children next to the `fast_funcdisc off` forcing. Lane cap
32 (measured throughput knee), distinct from the pool's `MAX_AUTO_JOBS = 16`; document both.
Driver-tier resource setting: no `phases.toml` row, no catalog counts; DIV row narrowing
DIV-145's "the engine cannot thread" (the decompile pipeline still cannot; the discovery decode
now runs N independent engines with only plain data crossing threads); docs/cli.md (the
`--jobs` section, the `functions` usage line, an explicit "`kuna xrefs`/`kuna strings` run a
different, order-dependent walk and are out of scope"), docs/spec/01-program-prep.md prose.

## Failure modes
Lane panic → `catch_unwind` per lane, a poisoned (cancellable) barrier built on Mutex+Condvar
(NOT `std::sync::Barrier`, which would hang the other lanes), discard shards, serial re-walk,
one stderr line. Kit build failure / context mismatch / sampled-decode mismatch → decline
before walking. Merge collision → discard + serial. Memory: +48 MB per lane + measured
+0.28 GB shard fragmentation; ~7.1 GB at 16 lanes on the target. (Measured as built:
8.67 GB at 16 lanes — see "As built".)

## Staging
* **PR-A (byte-identical refactor, no flag):** `walk::step`/`Successors`/`WalkPolicy`;
  `admits_call_entry` POD twin; `InsnLite` + Send guard; `ImageBytes`/`windowed_load_fill`/
  `SharedBytesImage` + `ObjectLoadImage` segments behind an `Arc` + `shared_bytes()` +
  `mapped_covers()`; `SleighBase::has_context_commits()` + `max_delay_slot_bytes()`;
  `ContextValueSnapshot`; `EngineRecipe` + `build_decode_engine()` with a cross-engine
  equality test over every instruction of several fixtures. Proof: four gates, `make test-cli`,
  the `ab.sh` sha matrix, interleaved timing within noise.
* **PR-B (the feature):** `listing/kuna_pdecode.rs` (intervals, router, lanes, rounds,
  reconcile, gate, self-check, stats), plan plumbing (`WalkPlan` from engine.rs through
  `run_listing_consumers` to `build_with_meta_planned`; `build_with_meta` keeps its signature),
  the knob, tests (unit; `verify_pdecode.rs` differential at lanes 2/3/4/8 × K variants ×
  PARTITION_ONLY/FULL with the size override; CLI matrix incl. the plan line and the ARM
  decline; injected-panic fallback; `make test-cli --reps 5` at `KUNA_DECODE_JOBS=8`), docs,
  DIV row, the corpus sweep (1,368 x86-64 + 98 ARM decbench ELFs) and interleaved min-of-5
  timings at lanes 1/4/8/16/32 on the target, libLLVM and gdb (`--max-fn-seconds 0` on any
  decompiling surface).
* **PR-C / PR-D (follow-ups):** sharded `Listing.insns` (deletes the merge); the serial floor
  (three ~94 MB `executable_sections` copies, the unused `_image` second loader, seed-name
  clones).

---

## As built (PR-B)

Where the shipped code differs from the design above, it is here. Everything not
listed landed as written.

* **The gate is split across two moments, not one.** What can be decided from the
  `Architecture` alone — a rebuildable `Sleigh`, `has_context_commits()`,
  `max_delay_slot_bytes()`, `shared_bytes()`, the context snapshot — is decided
  where the engine is (`WalkPlan::from_env`, called once in
  `engine.rs (commit_pending_analysis)`); what needs the walk's own inputs — the
  painter, the seeds, the executable ranges, the size floor — is decided at
  `walk()` (`kuna_pdecode::admit`). The observable contract is unchanged: one
  all-or-nothing decision, one stderr line, the serial walk on refusal.
* **`WalkPlan` carries the kit seed, not a `DecodeKitFactory` trait object.** The
  recipe, the `Arc<dyn ImageBytes>` and the `ContextValueSnapshot` are plain data,
  so there was no need for a trait or for kuna-analysis to reach into kuna-console;
  the plan is built once per `run_listing_consumers` and reused across all four
  rebuild sites.
* **Lane 0 is the calling thread.** The pre-spawn probe has to build a kit on this
  thread anyway, and a `Sleigh` cannot cross a thread boundary, so that kit becomes
  lane 0's and only `lanes - 1` threads are spawned. The probe is free.
* **Shards are a flat `Vec<(u64, InsnLite)>` plus a `HashSet<u64>`**, not a
  `BTreeMap` (judge 0's must-fix): ~130 bytes a record against ~298, and the
  per-shard sort runs on the lanes, before they join, so the merge is a linear
  ascending insert.
* **Callback evidence is collected uncapped per lane** (a `BTreeMap<u64, u64>` of
  target to lowest source) and the union is fed through ONE `CallbackEvidence` at
  the merge. Capping each shard and merging the survivors is not the same function
  as capping the union once; feeding the union is, because a target the cap evicts
  can never be retained again (the set's maximum rank is non-increasing), so the
  result depends on the evidence set and not on its order. The price is that the
  *intermediate* is not bounded by `MAX_CALLBACK_EVIDENCE = 4096` the way the
  walk's own side model is: a lane holds one entry per distinct in-exec `PUSH imm`
  target it saw, ~20 bytes apiece, on the x86 stack-callback path only. It is left
  uncapped deliberately — a per-shard cap can only be sound if it provably evicts
  nothing the union would have retained, which is a stronger claim than anything
  measured here buys.
* **Round bookkeeping takes three barriers**, not two: production, delivery, then
  one lane computing the next active interval list and resetting the cursor. The
  barrier is a Mutex+Condvar generation counter with a `poisoned` flag, and every
  lane body is wrapped in `catch_unwind`, so a lane that dies wakes the others
  instead of leaving them at a barrier nobody will reach.
* **`--jobs auto` resolves twice.** `jobs::parse_jobs` still caps the pool at 16;
  `jobs::decode_lanes` caps the lanes at 32. A worker child is given
  `KUNA_DECODE_JOBS=1` explicitly rather than relying on the parent's RAII guard.
* **Env surface as shipped:** `KUNA_DECODE_JOBS` (the bridge `--jobs` sets),
  `KUNA_DECODE_MIN_BYTES` (the size floor; tests set `0`), `KUNA_DECODE_INTERVALS`
  (intervals per lane), `KUNA_DECODE_SELFCHECK=1|abort`, `KUNA_DECODE_STATS=1`, and
  the test-only `KUNA_DECODE_FAULT=<lane>`.
* **The DIV row is DIV-167**, not DIV-164: three rows landed between the design and
  the implementation.
* **A `MAX_ROUNDS = 1024` runaway guard** exists, which the design body deliberately
  did not adopt as policy (a seed-starved image is still correct at 8 rounds).
  Tripping it is a `round limit` refusal, i.e. a serial re-walk.
* **The tripwire hook the design assumed** is added here rather than in PR-A:
  `SharedBytesImage::tripwire()` plus `build_decode_engine_traced`, leaving PR-A's
  `build_decode_engine` signature untouched.
* **Per-lane kits are rebuilt per `walk()` call**, not hoisted to
  `run_listing_consumers`. Only the *recipe* (`DecodeSeed`) is hoisted onto the
  plan; a `Sleigh` is `!Send`, so lanes 1..N cannot be handed one built elsewhere.
  It costs nothing today: every x86-64 load runs exactly one laned walk (one
  `decode stats` line on fauxware, gdb, libLLVM and the 147 MB target), and the
  callback/ARM re-walk sites that would rebuild refuse on their own language gate.
* **A refused lane spawn is a refusal, not a panic.** `thread::scope`'s `spawn`
  panics on `EAGAIN`/`ENOMEM` and joins the lanes already parked at the first
  barrier before resuming it — a deadlock with no output. The spawner uses
  `Builder::spawn_scoped`, poisons the gate on `Err`, skips lane 0's body and
  refuses with `thread spawn failed`. Reproduced under `ulimit -u`: `--jobs 32`
  on `/usr/bin/gdb` now exits 0 with the serial document.
* **Memory, as measured, not as projected.** The design's "+0.28 GB shard
  fragmentation; ~7.1 GB at 16 lanes" is wrong. Peak resident on the 147 MB target
  is 5.74 GB serial against 8.67 GB at 16 lanes and 9.55 GB at 32, and the
  overhead grows with the lane count while the total shard bytes do not (+0.47 GB
  at 2 lanes, +0.86 at 4, +2.16 at 8, +2.92 at 16, +3.80 at 32) — the glibc
  per-thread-arena signature: shards are allocated on the lane arenas and the
  final `BTreeMap` fresh on the main thread, so both are resident across the
  merge rather than one replacing the other. `MALLOC_ARENA_MAX`, or dropping each
  shard on its producing lane, is the follow-up. Because that peak is what the
  worker pool sizes itself from, `kuna_pdecode::run` records the lane-induced
  excess in a process-wide counter (`lane_peak_excess_bytes()`, also reported by
  `KUNA_DECODE_STATS=1`) and `jobs::worker_estimate` subtracts it.

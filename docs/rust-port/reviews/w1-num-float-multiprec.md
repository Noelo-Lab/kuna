# w1-num-float-multiprec

## Round 1

verdict: REJECT
verifier: independent verifier agent (Claude, not the porter)
date: 2026-06-10
branch: worktree-wf_cea42066-041-4 (worktree `.claude/worktrees/wf_cea42066-041-4`, reviewed at 42a2ec1; adversarial tests committed as 2f47583)
cpp blob shas: all 4 in-scope files match the checklist entry (`git hash-object` verified: float.cc d4632819, float.hh 44d52556, multiprecision.cc 81fd99f3, multiprecision.hh 32d4e971)

gate: `cd rust && cargo test -p kuna-num` -> **FAILED after adversarial
tests**: lib 15 passed, golden_float_vectors 1 passed (3604 oracle rows),
verifier tests 5 passed / **1 FAILED**
(`verify_denormal_carry_to_normal_matches_cpp_oracle`, debug panic at
float.rs:513 — finding F1). Pre-existing porter tests and the golden gate
all pass. `cargo test -p kuna-base` -> ok (35 + 6 passed, incl. cfmt).
clippy: `cargo clippy -p kuna-num -p kuna-base --tests -- -D warnings` -> clean.
mechanical greps: no `todo!`/`unimplemented!`/`HashMap`/`HashSet`/
`sort_unstable` in the item diff.
port-audit: `cargo run -p port-audit -- --item w1-num-float-multiprec` NOT
runnable — no `port-audit` crate in the workspace (carried gap, see
w1-base-foundation F4); blob SHA check performed manually instead.

## Method

Re-derived `float.cc`/`multiprecision.cc` function by function and diffed
against `git diff a65f2ac..42a2ec1 -- rust/` (the item's own commit: float.rs
+978, multiprecision.rs +492, cfmt.rs +207, golden_float.rs +180). Built an
**independent C++ oracle harness linking the real vendored float.cc**
(g++ -O2, x86-64/glibc, AddrSpace vtable stubbed) and ran three differential
sweeps the committed gate does not cover:

- `printf %g`/`%e` vs `cfmt::ostream_general/ostream_scientific`: 13,600
  cells (400 values x precisions, incl. ties, exponent-bump boundaries,
  subnormals, random f64/f32-derived bit patterns) — **0 mismatches**.
- `FloatFormat::printDecimal` (the precision ladder + parse-back, which the
  golden CSV's `dec` rows do NOT exercise — they pin `ostream <<` default
  only, per kuna_goldengen.cc:42-48): 3,268 oracle cells — **0 mismatches**.
- denormal-boundary `getEncoding`/`convertEncoding` probes — **divergence
  found (F1)**, plus `opTrunc` cvttsd2si boundary cells (all match).

multiprecision was differentially verified against native `u128` (20k random
ops) and on constructed inputs that drive the Knuth add-back (`t < 0`) and
qhat-correction paths, located by instrumenting a transcription of the
algorithm (the paths are too rare for random testing to reach).

## Hunt list

- signedness: **clean.** `calc_mask`'s `((uint4)size) < 8` mixed compare
  transcribed deliberately (float.rs:95-97, comment); `roundToNearestEven`'s
  `lowbitpos < 8*sizeof(uintb)` signed/unsigned compare resolved with a
  documented range argument (float.rs:466-471) that I re-derived
  independently: call sites pass [11, 64], including the lowbitpos == 64
  cell (2^-150 tie, oracle-pinned in `verify_denormal_rounding_grid_oracle`).
  All mp index loops stay on i32 like the C++ int4.
- integer widths: **clean.** Every uint4->uint8 promotion in
  knuth_algorithm_d is explicit (`u64::from` at each site,
  multiprecision.rs:263-294 mirrors multiprecision.cc:258-279 including the
  int8 reinterpretation `as i64`, the arithmetic `t >> 32`, and the
  truncating uint4 stores); `(int4)(x & mask)`, `(uintb)maxexponent`,
  `(uintb)exp`, `(int4)floor/ceil` all transcribed at C++ width.
- wrapping: **F1 (major).** The mp side is right: `wadd`/`wsub`/`wmul` at
  every wrap-capable site (carry chains multiprecision.cc:105-141, the
  multiply-subtract sign dance :258-279), `signif.wrapping_add(midbitmask)`
  preserves the round-carry wrap (float.cc:303). But the *consumer* of that
  wrap diverges: `signif >> (-exp)` after the carry bump (F1 below).
- comparator totality: **clean.** No ordering comparators ported; f64
  partial compares appear exactly where C++ compares doubles (opLess etc.),
  NaN-falsity oracle-pinned by golden rows + porter edge tests.
- iteration-order provenance: **clean.** No maps/sets anywhere in the item;
  all loops are index walks whose direction is transcribed (downward C++
  `for(;i>=0;--i)` -> `while i >= 0`), verified per loop.
- off-by-one / do-while / reverse iterators: **clean.** Both C++ do-whiles
  keep at-least-once semantics (`count_leading_zeros` float.rs:118-130 /
  address.cc:1039-1049; the qhat correction `loop { if .. break; ..;
  if rhat > 0xffffffff { break } }` multiprecision.rs:248-261 ==
  multiprecision.cc:251-256 — condition checked after the body). `leftshift`
  multi-branch boundary cells (sa = 0/63/64/65/127) pass against u128.
- erase-while-iterating: **n/a** (no containers).
- exception -> Result partial-state parity: **clean.** `udiv128`'s
  LowlevelError("divide by 0") returns before any output write in both
  languages; the C++'s own inconsistency (64-bit fast path divides by zero ->
  SIGFPE, wide path -> throw) is preserved, with the fast-path trap mapped
  to the Rust divide panic per ADR 0004 and documented in code
  (multiprecision.rs:312-316). Nothing else in scope throws.

## Findings

- F1 (major): denormal round-carry-to-normal executes `signif >> (-exp)`
  with exp == 1 — debug build panics, release matches the oracle only by
  accident of shift masking.
     cpp: decompiler/cpp/float.cc:334-344 (getEncoding denormal path) and
     :407-417 (convertEncoding, same code). When roundToNearestEven carries
     out of bit 63 (host value at/just below the target's smallest normal),
     the TODO fixup sets `signif = 1<<63; exp += 1`; with exp previously 0,
     `signif >> (-exp)` shifts by -1: C++ UB that the x86-64 oracle resolves
     by masking the count to 63, so the fraction code becomes 0 and the
     function returns the (signed) **zero encoding** (upstream bug, but
     oracle-observable behavior).
     rust: rust/crates/kuna-num/src/float.rs:513 (get_encoding) and :592
     (convert_encoding).
     claim/observed: port claims line-for-line parity; observed
     `get_encoding(fmt4, f64::from_bits(0x380fffffffffffff))` and
     `convert_encoding(fmt4 <- fmt8, 0x380fffffffffffff)` **panic in debug**
     ("attempt to shift right with overflow") where the linked C++ oracle
     returns 0x0 (0x80000000 negated). Release returns 0x0 — a debug/release
     divergence ADR 0003 explicitly forbids, and the panic is reachable from
     ordinary engine input (FLOAT_FLOAT2FLOAT constant-folding of any double
     in [2^-126 - 2^-150, 2^-126) to float; every double with >=2 mantissa
     bits in that window triggers it).
     repro: `cd rust && cargo test -p kuna-num --test
     verify_w1_num_float_multiprec verify_denormal_carry` (fails debug,
     passes `--release`).
     work order: make the cell explicit and identical in both profiles —
     reproduce the oracle's masked shift (e.g. shift by `(-exp) as u32 & 63`
     with a comment citing the C++ UB and oracle value, or special-case
     exp == 1 to return the zero encoding). Do NOT "fix" it to the
     IEEE-correct smallest normal: the C++ oracle returns zero. If the team
     instead chooses a deliberate divergence, it needs a losses.md entry and
     still must not differ between debug and release.
- F2 (minor): multiprecision public API drops the C++'s documented in/out
  aliasing capability.
     cpp: decompiler/cpp/multiprecision.cc:24 ("in and out arrays ... can
     point to the same storage"); live aliasing call sites in
     decompiler/cpp/ruleaction.cc:7893 (`leftshift128(power,power,n)`),
     :7894 (`add128(multConst,power,multConst)`), :8183
     (`subtract128(y, power, y)`), :8194-8195.
     rust: rust/crates/kuna-num/src/multiprecision.rs:46-53, 86-150
     (`&[u64;2]` in / `&mut [u64;2]` out — the borrow checker forbids those
     call shapes).
     claim/observed: semantics of the ported functions are correct for
     disjoint buffers (differentially verified); the W2+ port of
     RuleDivOpt/ruleaction must insert copies at every aliasing call site or
     this API must grow in-place variants. Watch item for the ruleaction
     porter; not a defect of the math.
- F3 (note): `FloatFormat::new` panics for sizes other than 4/8 where the
  C++ constructor leaves every field uninitialized (UB downstream).
     cpp: decompiler/cpp/float.cc:36-61; only sizes 4 and 8 are ever
     constructed upstream (translate.cc:984-985).
     rust: rust/crates/kuna-num/src/float.rs:225-232.
     ADR-0004-sanctioned mapping of a C++-UB state; documented in code.
- F4 (note): `udiv128` fast path (numer[1]==0 && denom[1]==0) with a zero
  denominator panics on Rust integer division where the C++ takes a SIGFPE;
  the wide path returns `KunaError::lowlevel("divide by 0")` matching the
  C++ throw exactly.
     cpp: decompiler/cpp/multiprecision.cc:291-303.
     rust: rust/crates/kuna-num/src/multiprecision.rs:312-327.
     Documented in code; trap-for-trap parity per ADR 0004.
- F5 (note): a handful of bare `as` casts lack per-site `// cast:` comments
  (`(x & mask) as i32` float.rs:322, `self.maxexponent as u64`
  float.rs:370/381, `exp as u64` float.rs:536/615, precision `as i32`
  float.rs:391-395). Each mirrors an explicit C++ cast at the same spot
  ((int4)/(uintb) in float.cc:138,199,213,220-222,363) and is range-safe;
  fix opportunistically.
- F6 (note): gate-coverage gap, now partially fenced: the golden CSV `dec`
  rows pin only `ostream <<` default formatting (kuna_goldengen.cc:42-48),
  so `printDecimal`'s precision ladder and `ostream_general/scientific` at
  non-default precisions were untested by the committed gate. Verifier
  sweeps (13,600 %g/%e cells + 3,268 printDecimal cells vs the linked C++
  oracle) found 0 mismatches; representative cells are now pinned in
  `verify_print_decimal_cpp_oracle_cells`. Consider a `golden printdecimal`
  vector kind for the wave.

## Adversarial tests

Committed in the worktree as 2f47583
(`rust/crates/kuna-num/tests/verify_w1_num_float_multiprec.rs`; oracle
values from the verifier's own harness linking the real vendored float.cc):

- verify_denormal_carry_to_normal_matches_cpp_oracle — **FAILS in debug**
  (the F1 evidence): asserts the C++ x86 oracle outputs (zero encodings)
  for the round-carry-to-normal cells via both get_encoding and
  convert_encoding/op_float2float.
- verify_denormal_rounding_grid_oracle — denormal cells away from the broken
  boundary, incl. the 2^-150 tie (lowbitpos == 64 lowbitmask-drop cell) and
  the in-denormal carry that exercises the wrap fixup legitimately.
- verify_udiv128_addback_and_qhat_correction — 6 constructed add-back
  (`t < 0`) inputs + 4 qhat-correction-loop inputs vs native u128 (paths a
  random sweep cannot reach).
- verify_mp_random_differential_u128 — 20k-op LCG differential of
  add/sub/less/shift/div against u128, plus the divide-by-zero error-path
  parity check.
- verify_print_decimal_cpp_oracle_cells — printDecimal ladder cells pinned
  from the C++ oracle (f32 denormal sci/plain, f64 min denormal 15-digit
  start, 1e+22, 2^53+2 16-digit fixed).
- verify_trunc_boundary_cpp_oracle_cells — cvttsd2si boundary cells
  (2^63-1024 in range, +-2^63 edges, -0.5/0.5, negative wrap into masks).

losses: none cited — F1 must be repaired to match the oracle (REJECT), and
no other test-visible divergence from oracle-defined behavior was found.

## Round 2

verdict: ACCEPT
verifier: independent verifier agent (Claude, not the porter)
date: 2026-06-10
branch: worktree-wf_cea42066-041-4 (reviewed at 9d5d354 "repair round fixes";
round-2 adversarial tests committed as 8b485f8)
cpp blob shas: unchanged, all 4 in-scope files re-verified against the
checklist (`git hash-object`: float.cc d4632819, float.hh 44d52556,
multiprecision.cc 81fd99f3, multiprecision.hh 32d4e971)

gate: `cd rust && cargo test -p kuna-num && cargo test -p kuna-base` ->
**PASS**: kuna-num lib 15, golden_float_vectors 1 (3604 oracle rows),
verifier suite 9/9 (6 round-1 + 3 round-2) — green in **both debug and
`--release`** (the ADR 0003 profile-identity requirement F1 violated).
kuna-base 35 + 6 (cfmt) pass. clippy:
`cargo clippy -p kuna-num -p kuna-base --tests -- -D warnings` -> clean.
mechanical greps over the full item diff: no `todo!`/`unimplemented!`/
`HashMap`/`HashSet`/`sort_unstable`. port-audit: still no `port-audit`
crate in the workspace (carried gap, w1-base-foundation F4); blob-sha
check done manually as in round 1.

### Scope of the round

The only delta since the round-1 review is the repair commit 9d5d354:
float.rs +17/-2, replacing `signif >> (-exp)` with
`signif.wrapping_shr((-exp) as u32)` at the two F1 sites
(rust/crates/kuna-num/src/float.rs:522 get_encoding, :607
convert_encoding), each under a comment citing the C++ UB and the oracle
value. No other in-scope file changed; the round-1 hunt-list results for
everything else carry over unmodified.

### F1 re-verification (independent)

- Re-derivation: in both denormal branches `exp` enters in
  `[-frac_size, 0]` (guarded by `exp < -frac_size` early-zero and
  `exp < 1`), and the round-carry fixup adds at most 1, so the C++ shift
  count `-exp` is confined to `{-1} ∪ [0, frac_size<=52]`. For counts in
  `[0, 52]`, `wrapping_shr` is a plain shift (mask `& 63` is identity) —
  the legitimate path is bit-identical to before. For the UB count `-1`,
  `(-1i32) as u32 & 63 == 63` reproduces exactly the x86-64 SHR count
  masking the oracle exhibits, so `setFractionalCode` receives
  `(1<<63)>>63 == 1`, drops it (`code >>= 64-frac_size`), and the signed
  zero encoding returns — float.cc:144-153 re-read to confirm.
- Fresh oracle: I rebuilt my own standalone harness linking the REAL
  vendored float.cc (shimmed address.hh: count_leading_zeros/uintbmasks/
  calc_mask/sign_extend copied verbatim from address.cc/.hh). Confirmed
  `getEncoding(fmt4, 0x380fffffffffffff) == 0x0`,
  negative -> 0x80000000, `convertEncoding(fmt4<-fmt8, same) == 0x0`,
  and — new this round — that the UB family is WIDER than the round-1
  cell: any host whose significand has its top 23 fraction bits all ones
  and that rounds up (e.g. bits 0x380ffffffffff800, ...fbff) carries to
  `exp == 1` and returns zero. All pinned in the new family test.
- Profile stability of the oracle itself: the harness compiled at -O0
  and -O2 produces byte-identical output (both emit hardware-masked SHR),
  so the pinned behavior is not an -O2 accident — it holds for the
  profiles kuna actually builds (decomp_dbg/decomp_opt).
- The round-1 failing test `verify_denormal_carry_to_normal_matches_cpp_oracle`
  now passes in debug and release.

### Hunt list (round-2 delta)

- signedness: clean (repair introduces no new comparisons).
- integer widths: clean. The new `(-exp) as u32` is the one new cast; for
  the only negative reachable value (-1) the 0xFFFF_FFFF it produces is
  deliberately consumed by `wrapping_shr`'s `& 63` — documented in the
  adjacent comment block (accepted as the cast justification; folds into
  round-1 F5).
- wrapping: F1 resolved as above; `round_to_nearest_even`'s
  `wrapping_add` carry producer unchanged and still correct.
- comparator totality / iteration order / off-by-one / erase-while-
  iterating / exception parity: no code in those classes touched this
  round; round-1 results stand (all clean or n/a).

### Findings

- F1 (resolved): repair 9d5d354 verified correct against re-derivation,
  fresh C++ oracle (named cells + 200,000-cell sweep hash), and both
  build profiles. Closed.
- F7 (note, new): the negative-shift UB at float.cc:343/:416 deserves an
  entry in docs/rust-port/upstream-bugs.md (it is exactly that ledger's
  audience: real upstream UB found during the port, reportable to Ghidra)
  with the nuance that — unlike UB-1/UB-2 — the Rust port deliberately
  REPLICATES the x86-64-observed resolution because the value is
  parity-observable through golden vectors and FLOAT2FLOAT constant
  folding. Left to the porter/maintainer; not blocking (the behavior is
  fully documented at both Rust sites and in this review).
- F2 (minor, carried): multiprecision in/out aliasing watch item for the
  W2+ ruleaction porter — unchanged, still open as a watch item.
- F3/F4/F5/F6 (notes, carried): unchanged.

### Adversarial tests (round 2, commit 8b485f8)

Appended to `rust/crates/kuna-num/tests/verify_w1_num_float_multiprec.rs`:

- verify_r2_carry_to_normal_family_oracle — 14 oracle-pinned cells: the
  full carry-to-normal UB family (both signs) plus the legitimate
  boundary neighbours (exact 2^-126, the in-denormal carry exp -1 -> 0
  cell 0x37ffffffffffffff -> 0x00400000, the 2^-149/2^-150 min-denormal
  cluster, and the below-min early-zero path), each through both
  get_encoding and convert_encoding.
- verify_r2_fmt8_deep_denormal_shift_counts — legitimate large shift
  counts (up to 51) through the repaired line: f64 denormal encodings
  must round-trip get_encoding(fmt8) exactly.
- verify_r2_denormal_window_sweep_hash_matches_cpp_oracle — 200,000-cell
  LCG sweep over exponent fields 0x365..=0x382 (both signs, random
  mantissas), FNV-1a-64 hash of (bits, enc4, cnv4) chained per cell,
  pinned to the hash the real float.cc produced (0x28dbe215fe702925);
  any single-cell divergence or debug-only panic breaks it.

All 9 verifier tests pass in debug and release.

losses: none — the repaired behavior is byte-identical to the C++ oracle,
so no losses.md citation is required. ACCEPT.

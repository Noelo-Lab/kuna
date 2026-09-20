# Campaign decisions (why, not what — the plan has the what)

1. libctypes shells are SIZED (glibc x86-64 ABI sizes), bare-named (`FILE`, not `struct _IO_FILE`), pointer-only.
   Why: a zero-size pointee keeps a PTRSUB alive through RulePtrsubUndo and the printer emits functional
   `PTRSUB(p,0x28)` on the exact sites the feature retypes; decbench's normalizer rejects the `struct X*` spelling.
2. Named libc types outrank synthesized structs; struct synthesis declines on any named composite or type lock.
3. bytehonest exports `undefined1` on the JSON surface for a residual TYPE_UNKNOWN byte (C text unchanged).
   Why: it reports what kuna knows (IDA exports `_BYTE`); measured +8 perfects / 0 regressions on 6 binaries.
4. structsynth stays default-off and OUT of `aggressive` (the decbench-scored path) — 19 currently-correct
   char*/int*/char** params would become `struct_0 *` misses.
5. TRex taken piecewise (rounding order → signedness; Fig.6 metric → structscore; colocation/aggregate declines →
   structsynth); the StructuralType substrate is a separate XL [PROPOSAL] the user may request later.
6. Goal 2 = correctness (phantom args via protoorder) + readability measured by varcensus; kuna is already leanest.
7. `kuna decompile` and `decompile-all` may disagree on a callee's types once protoorder parks recovered protos.
8. Never write into /home/mahaloz/github/decbench/results (a dewolf run is live); baselines are kuna-side.
9. 2026-09-17 user decision: GO on all open proposal PRs under the normal rules — no large speed hit (+5% budget)
   and accuracy relatively the same or better (444-slice typesweep). Applied as: signedness lands with default
   `auto` (the measured same-or-better arm that keeps 675/675; prefer-signed stays opt-in because it moves datatest
   assertions); structsynth's default is decided by a proper interleaved speed measurement + the full typesweep
   (≤ +5% and ≥ off − 0.1% → default `param`, else opt-in); protoorder gets the same treatment when its draft opens.
10. 2026-09-18 user decision (supersedes 4): structsynth goes default `param`. The `type_match` cost of anonymous
   `struct_N *` names is accepted; decbench is asked to credit any struct pointer against a pointer-to-struct GT
   (Noelo-Lab/decbench#93). A `struct_N *` on a primitive pointer (`char *`, `unsigned long *`) is still a real
   false positive — the 7 structsynth rows among the final re-measure's 8 worse functions are that class.
11. 2026-09-19 Stage-3 re-measure (`results.md`) against decbench `main` @ 625e892, the checkout the baseline was
   taken with, so the before/after pair shares one metric; newer decbench commits are not mixed in.
12. 2026-09-20 round-C re-measure: the decbench checkout has moved past `625e892` and the newer
   `type_match` drops `_effective_offset`, so the campaign's instruments fail against it. The pin is
   kept by extracting `625e892` read-only (`git archive`) and repointing the venv's PEP-660 editable
   finder at that copy (`final-c/pindb.py`) — the finder outranks `PYTHONPATH`, so a path-only pin
   silently measures with the new metric. Verified by re-running the round-B binary: 986 perfect,
   0 of 10,748 values differ.
13. protoorder's call-site vote is accepted for `type_match` and rejected for struct identity: it is
   +363 perfect on the 444 slices and it drops per-parameter layout precision from .89 to .55,
   because a caller's parameter takes the callee's record even where the caller measured its own
   (`final-c/layout-ablation.md`). Round C ships it on (the metric the campaign is judged by improves
   and no wrong C is emitted); the caller-measured-layout carve-out is the first round-D item.

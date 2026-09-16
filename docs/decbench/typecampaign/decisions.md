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

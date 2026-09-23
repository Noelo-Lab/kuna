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
14. 2026-09-20 round D withdraws §13's attribution. protoorder does NOT overrule a caller's measured
   layout — structsynth type-locks the parameter after the vote, and du -O2 installs the same 36
   records in both arms. The .8945 → .5520 drop was the campaign instrument joining parameter types
   from `decompile-all` against layouts from `decompile-project`, two surfaces that numbered
   `struct_N` in different orders (#693). With one order the round-D figure is .8709 fields-only and
   F1 .1678, above round B on F1 and recall. Lesson kept: a measurement that joins two kuna surfaces
   by a synthesized NAME has to prove the two ledgers agree before the number means anything.
15. 2026-09-20 `argclobber` is default `on` (#689) on the strength of the callee's own recovered
   prototype, not a caller-side heuristic. It is inert wherever protoorder parks nothing — a
   single-function `kuna decompile`, `--addr`/`--functions`, `--jobs N`, `decompile-project`,
   `decompile-graph` — so a serial `decompile-all` and a sharded one legitimately differ by exactly
   these dropped arguments, and any ablation of it MUST use a whole-binary serial run.
16. 2026-09-20 decbench#94 (the missing `DW_TAG_restrict_type` arm) is not a campaign lever. The
   4,412 GT variables it makes unmatchable are 98.2% register-only, which no decompiler scores:
   patching a copy of the pinned metric moves kuna by 8 functions and 0.84 aggregate, and does not
   move a single function onto or off perfect. Worth fixing upstream for honesty, not for score.
17. 2026-09-22 round E: the layout instrument scores only synthesized records, because a libc
   record is a sized shell with no members in the exported header. When libctypes names a record
   that structsynth used to synthesize, the parameter leaves the layout join: recall falls while
   `type_match` and TRex rise. Round E's recall drop (.0929 → .0859) is exactly the nine `obstack *`
   parameters of ls and sort. Read the layout rows beside the class table, never alone.
18. 2026-09-22 `charptr` stays `off` and is no longer a flip candidate. After #704 counts a constant
   `PTRADD` index as a field offset, the option arm is net negative on the 444 slices and the flip
   still fails `make test-cli`. The measured `char *` levers are protoorder parking the recovered
   types of recursive functions (+36 to +46 perfect) and typing `framelayout` filler slots.
19. 2026-09-22 the O2 gap is the metric's ceiling before it is the engine's. At O2 85.8% of scored
   functions hold a GT variable no stripped decompiler can match; on the reachable ones kuna converts
   43.8%, level with O0's 43.5%. The next O2 perfects come from decbench first (inlined-callee
   variables counted as locals: 74 → 153 with no engine change), the way #93 and #94 did.
20. 2026-09-22 `structsynth nest` ships opt-in although every default criterion passes literally:
   its gain is 2 nested fields on 10 builds, and a sharded `--jobs 8` run costs +63% because a
   nested or self field has no portable recipe. The flip waits for a request-relative recipe.
21. 2026-09-23 `slotptr` (#710) is default on although it turns five free matches into misses. The
   pinned metric credits an exported `undefined8` against an 8-byte integer, so a filler slot kuna
   says nothing about scores by luck; typing it from the pointer stored into it is +123 perfect and
   −5, and every one of the five is a body type that was already wrong (`shred::dorewind` merges
   `lseek`'s return with `__errno_location()`'s `int *`). An honest wrong type beats a lucky silence.
22. 2026-09-23 `passthrough` (#708) stays default off even though it is the only round-F lever that
   moves O2 (74 → 81) and O2-noinline (383 → 400). It adds arguments, and `type_match` cannot see a
   call's arity, so the metric's +24 is not evidence for it; DWARF confirms 454 of 506 gained
   parameters over 26 binaries and contradicts none, but nothing yet speaks for the other 52. The
   flip waits for that check, not for a better score.
23. 2026-09-23 a default change that only moves `variables[]` is still a default change. #710 leaves
   P-code and C output byte-identical and moves 425 functions on the metric; ablating it needs
   `--option slotptr off` on a whole-binary serial run, and its regressions are only visible in the
   JSON surface. The campaign's rule — measure the surface decbench scores, not the C text — cuts
   both ways.
24. 2026-09-23 `passthrough` (#716) flips ON, and the evidence is DWARF, not this metric. `type_match`
   cannot see a call's arity, so its +27 is a side effect; what moved the decision is 4,107 of 4,346
   gained parameters confirmed by the unstripped twins over two corpora — the 444 slices and 130 slices
   of 17 disjoint projects — with 0 contradicted and 0 arguments lost. The three refusals shipped with
   it are exactly metric-neutral here: `--option passthrough off` on the round-G build reproduces the
   pre-flip commit value for value (1,582 perfect, mean .3637, aggregate 3,909.56), so the flip is the
   whole gain — and the only round-G lever that moves -O2.
25. 2026-09-23 `calleevote fields` (#711) ships default with its one known cost. A parameter every
   caller passes a string to gets `char *` even where the declaration says `void *`: that is gnulib
   `xmemdup`, three of its three losses. The vote is right about the program and wrong about the
   prototype, and it buys +34 perfect on the metric, the whole of goal 3's movement (TRex O0
   4.4317 → 4.4747, struct-typed parameters 123 → 173, layout recall .0865 → .0932) and +322 credited
   functions under decbench#93. A caller vote is accepted when it is right about the program.
26. 2026-09-23 a strict fix may cost the metric and still ship ungated. #714 strips the stack
   placeholder a call carries into its printed argument list — the call contradicted its own prototype
   in the same output — and four `_Bool` slots lose `boolbyte`'s reading as a side effect
   (`ls::gobble_file` ×3, `cp::make_dir_parents_private`). Correct output first; the `boolbyte`
   follow-up is a lever, not a reason to gate a fix.
27. 2026-09-23 attribute a round by BUILDING each landed commit, not only by ablating options. #714 is
   ungated and its libc table cannot be isolated with `--option libcsigs off` (that arm drops the whole
   28-name base table too), so round G swept `9e07ab931` and `1c57f06b6` as their own binaries. Each
   build needs its own directory: `kuna` forks the `decomp_dbg` BESIDE IT, so two builds sharing a
   directory silently measure one engine (the catalog of an old `kuna` printed the new default until
   the pair was separated).

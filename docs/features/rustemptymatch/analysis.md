# Empty Rust match exits

A direct switch-exit edge becomes an empty Rust match arm. Previously it emitted `break`, which fails with E0268 outside a loop and breaks the enclosing loop when nested. The authored `empty_loop(0,7)` returns 49 natively; its unchanged parent Rust body compiles and returns 0.

P8 `scope_break` marks a direct `CaseOrder` edge `f_break_goto` only when it targets the switch's continuation. P9 `emit_match_arm_rust` now leaves that arm empty. Other case transfers and actual loop-body breaks retain their handling. This is a strict Rust spelling correction, without an option or a changed control-flow graph. Nonempty switch-arm transfers, C fallthrough and residual Rust gotos are outside its scope.

The five authored AMD64 functions use wide guards that recover on the parent. They cover empty, default and shared arms, an enclosing loop, and a real loop exit. Their complete emitted Rust bodies are unchanged in the native harness: rustc at optimization levels 0 and 2 agrees with the original on 6,900 values, including every table entry, out-of-range and signed-width boundaries, negative/zero iteration counts and repeated iterations. The independent public C reproduction is in the PR body; it also passes against the final integrated builds. Its earlier build receipt is explicitly archived as preintegration evidence.

The final comparison covers 42 images, 33,713 distinct functions and 75,855 language records. All C and Go records are identical. Exactly 86 Rust functions lose 113 empty-arm breaks; after those edits, all remaining text is identical except whitespace. The 12 inherited Go parser failures remain unchanged. This is not a claim that every full upstream Rust body recompiles: unrelated types, strings and residual gotos still have gaps. Supplied upstream executables were not run.

Validation passes 675 original assertions, 3663 stage assertions and 10,287 workspace tests, with 39 ignored and no failures. The original baseline is unchanged. Python tooling, lint, option catalog, counters and the live spec also pass.

The predeclared speed study has seven cases, 384 balanced paired rounds per case and one final look. Disjoint physical CPU pairs, frozen specifications and source/binary hashes are recorded. All simultaneous CPU/wall bounds are within the 5% budget. The largest median CPU delta is 0.435% and wall delta 0.397%; the largest upper bounds are 1.926% CPU and 4.636% wall. These bounds apply to the recorded cases and binaries.

Main's declared-byte diagnostics were integrated before sampling. The earlier correctness run is retained under `preintegration/`; the final records use matching independent parent and candidate builds against `db2ddd4d63413227bd4cab9756ed329ad561bf05`. No earlier timing samples were collected. See [the record](record.json), [the whole-output review](breadth-review.json), and [the timing observations](timing-result.json).

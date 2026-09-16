# argclobber — implementation plan

## 1. The change

New module `decompiler/crates/kuna-decomp/src/p4_calls/kuna_argclobber.rs`, one
entry point called from `build_input_from_trials`
(`p4_calls/funcdata_callsite.rs`) immediately after
`kuna_calleearity::unify_with_sibling_call`:

```rust
pub fn drop_clobber_tail_arg(fc: &mut FuncCallSpecs, data: &mut Funcdata)
```

It scores ONE trial `mark_no_use()` and touches nothing else. Preconditions, in
evaluation order: the option is on; the spec is neither input-locked nor variadic;
the op is a live `CPUI_CALL`; there is a last `is_used()` trial and at least one
other used trial; that trial is a register trial with `is_ind_create_formed()`;
its argument Varnode's defining op is an indirect creation or a MULTIEQUAL with an
immediate indirect-creation input, and every one of those creations is a creation
of the trial's OWN register (`clobber_of_this_register_reaches`); the callee's
entry-liveness summary does not `proves_input` those bytes; and
`kuna_calleearity::best_witness_for` has no already-final sibling call to the same
callee entry carrying an argument at that storage.

The last two preconditions were added in review round 2; `analysis.md` §8 is the
evidence that bought them.

**Why this seam, not `check_input_trial_use`**: trials are sorted and
entry-assigned only inside `fillin_map_standard` (`active.sort_trials()` is its
last statement), which runs in `resolve_and_derive_input_map` — so "trailing" is
not computable earlier, and the refuted `argindcreate` design's insertion point
could not have worked.

**Why `mark_no_use` and not a local skip**: `DEFNOUSE` is the flag the whole
deferred arity family already reads (`calleearitylive::capture_partial_call`
explicitly skips such a trial), so one flag write both drops the argument from
`newparam` and stops the additive rules from restoring it. `set_final_input_storage`
is computed from `is_used()` after the drop, so a later sibling sees the shortened
list and reconciles to it.

## 2. Wiring

* `Architecture::arg_clobber` (default `false`), `ArchContext::arg_clobber`, the
  `"argclobber"` option arm, the ctx copy in `apply_runtime_options`' mirror.
* `p0_knowledge/options.rs` registration; `phases.toml` `[[settable]]` row
  (P4 / `active-input-trial-scoring`, `change_kind = "correctness-fix"`,
  `tier = "core"`, `live_field = "arg_clobber"`, `source_decompiler = "ghidra"`).
* `ELEM_ARGCLOBBER = 4173` (next free in the kuna 4000+ range).

## 3. Counter sites moved

Re-derived against `origin/main` at rebase time with
`python3 -m scripts.repipe.counters --rederive/--fix` on a freshly built tree,
never by arithmetic: 208 -> 209 settables (`kuna_phases/tests.rs` fn-name,
`kuna_num_settables`, `SETTABLE_TABLE-len`, tier tuple 67->68 core,
catalog-json-records 207->208; `tests/catalog_bytecompat.rs`, 4 sites) and the
captured `tests/fixtures/phase_catalog.json` (re-captured with `phase catalog`, no
program loaded); `option_values_live_value_present_for_86` -> `_for_87` (the option
has a `live_field`); stage corpus count `kuna-base/src/xml.rs` 294 -> 297 (three
stage files); `docs/baseline-stages.json` re-recorded (1018 keys), not hand-merged;
`docs/options.md` regenerated.

## 4. Test

`tests/stages/kuna-argclobber.xml`, two passes over one hand-assembled x86-64
bytechunk (`caller` / `clobber` / `target` / `helper`): one path calls `clobber`
(whose own first act is a call, so `calleepreserves` cannot narrow the killed set),
the other writes `edx` itself; the join feeds `target(rdi, rsi)`. `target`'s first
act is a call too, so `calleedeadarg` cannot prove `rdx` dead there.

* pass 1, `option argclobber off` (the shipped default): `target(a0,5,v2)` plus
  `unsigned long v1; // rdx` — the bug;
* pass 2, `option argclobber on`: `target(a0,5)` and no `// rdx` declaration.

Two negative cases, one per guard clause, each asserting the SAME call in both
passes:

* `tests/stages/kuna-argclobber-guards.xml` (x86-64) — the same caller shape, but
  the callee is `reader: mov %rdx,%rax; ret`, so the callee clause keeps the
  argument.
* `tests/stages/kuna-argclobber-armreturn.xml` (ARM) — u-boot `sub_60827fa4`
  reduced: `bl producer; cmp r0,#4; movne r3,r0; moveq r3,r0; bl target2`, so the
  creation feeding `r3` is a creation of `r0` and the same-register clause keeps
  the argument.

## 5. Evidence required before any default-on proposal (not this PR)

0/675 on the datatests with the flip, PARITY OK on stages, a clean whole-corpus
sweep wider than the 12 binaries measured here, and a decision about the stated
wrong-output shape (a 16-byte `rax:rdx` return forwarded as a trailing argument).

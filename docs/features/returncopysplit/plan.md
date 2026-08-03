# returncopysplit -- plan

## Scope: small (one predicate, one file)

One added test in one already-ported function. No new module, no new option, no new pass,
no structuring change.

## The change

`decompiler/crates/kuna-decomp/src/p3_dataflow/subflow.rs`,
`SplitDatatype::test_copy_constraints` (upstream `subflow.cc:2390`):

```rust
// (kuna) A COPY whose output lands in a read-only address range is never a
// store the program performs; splitting it turns one invisible whole-range
// assignment into per-element assignments that survive
// `Merge::mark_internal_copies` and print as stores into `.rodata`.
let out_vn = data.obank().get(copy_op).expect("stale copy").get_out().expect("copy out");
if data.vbank().get(out_vn).expect("stale out").is_read_only() {
    return false;
}
```

`Varnode::is_read_only` already exists (`substrate/varnode.rs:797`) and is already used at
`p2_lift/jumptable.rs:1700,2175`. The flag is painted from the symbol table's property map,
which `Architecture::fillinReadOnlyFromLoader` fills from the load image's non-writable
sections -- so it is true exactly for addresses the program cannot store to, on both the
real-ELF path (`kuna-console/src/engine.rs`, `ObjectLoadImage::get_readonly` over section
flags) and the datatest path (`<bytechunk readonly="true">`).

## Why no option

`CLAUDE.md`: an option gates a *feature* -- "behavior that is a judgment call, not
universally better"; "a strict bug fix that only corrects wrong output needs no flag".
Emitting a store the program never performs is wrong output under every reading, and there
is no honest `use_when` prose to write for a knob that re-enables it. A bisector already
has `option togglerule splitcopy off`, which is the rule's registered name in the action
pool (`infra/universalaction.rs:558`).

Because the added test is a divergence from upstream `subflow.cc:2390` (rather than the
restoration of an upstream behaviour a port dropped), it is recorded as a **DIV row**
(DIV-55) in `docs/history.md`, not a Convergence. The precondition it guards against is
itself created by a kuna divergence (`stackguard`, DIV-14), which is the second reason it
cannot be a Convergence.

## Why not the filed predicate

The triage's `is_return_copy()` test is measured to change nothing: instrumentation shows
`rc=true` never reaches a successful split, because upstream's same-address gate already
declines the un-cloned form, and `CloneBlockOps::build_op_clone` drops the flag on the
cloned form. See `analysis.md` section 3.

## Verification plan

1. **Witnesses** -- all three known functions, before/after builds of this worktree:
   `ip` `netns_add` 16 -> 0, `ip` `__get_hz` 32 -> 0, `sshd` `sub_230a0` 64 -> 0.
2. **Regression sweep** -- whole-binary `decompile-all --json` before/after over 12 corpus
   binaries, diffing per function. Requirement: the only functions that change are the
   three witnesses, and every surviving global store lands in a writable section.
3. **Stage test** -- `tests/stages/ghdec-returncopysplit.xml`, the real `__get_hz` bytes at
   their real VMAs plus the read-only literal chunk; pass 1 default (the fix), pass 2
   `option togglerule splitcopy off` (the reference rendering with no split at all).
4. **Gates** -- `make test` (675/675 PARITY OK, no re-pin), `make test-stages` (re-recorded
   baseline), `make rust-test`, `make check-spec`, `kuna catalog --check`.
5. **Speed** -- interleaved A/B medians on the two builds; `scripts.pipeline.timeit` cannot
   be used because it measures an option flip and there is no option.
6. **GED** -- `scripts.decbench.rescore` on the campaign case. Expected ~zero: removing
   statements changes no block and no edge.

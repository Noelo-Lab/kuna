# Plan — `switchguardbound` (option), feature slug `missing-function-call-1101b1`

Scope: **small** (decider-confirmed). One gated `JumpBasic` model-extension method + one new
module owning the option/arch-flag, modelled verbatim on the `switchmodbound` (GH-9191)
precedent. No new pass type, no S7/structuring touch.

## Mechanism

New `JumpBasicModel::kuna_try_guard_bound_table(fd, indop, maxtablesize) -> bool` in
`s2_lift/jumptable.rs`, invoked from `recover_model_basic`'s `jrange.size > maxtablesize`
failure branch, immediately **after** the existing `kuna_try_modulo_bound_table` check, gated
on `fd.get_arch().switch_guard_bound`:

1. For each `selectguards` entry `G` with a branch CBRANCH:
   - storage `(space, offset, size)` of `G.vn`.
   - find a meld common varnode `iv` with the **same storage** (the index, e.g. `eax`).
   - extract the unsigned bound `K` by walking `G`'s CBRANCH boolean def-tree (depth-bounded)
     for a comparison/subtraction constant against the guarded storage — handles the
     `sub K; ja` flag form and the clean `INT_LESS(K, v)` form. `bound = K + 1`.
   - require `bound ∈ [2, maxtablesize]`.
2. Rebound (mirrors modulo): `varnode_index = pos(iv)`,
   `jrange.set_range(CircleRange::new(0, bound, vsize, 1))`, `set_start_vn(iv)`,
   `set_start_op(get_earliest_op(pos))`. Return `true`.
3. On any mismatch / unparseable guard → return `false` (caller declines exactly as today;
   gate-off path untouched ⇒ byte-identical).

`recover_model_basic` then re-passes the `jrange.size <= maxtablesize` check, calls
`mark_foldable_guards`, accepts the model; `buildAddresses` emulates `idx ∈ [0, bound)`
through the meld (LOAD + add) to enumerate the case targets; the structurer/printer emit the
`switch`.

## Wiring (switchmodbound footprint)

- `s2_lift/kuna_switchguardbound.rs` (new module): `SwitchGuardBoundOption` (option struct +
  arch-flag owner), default off. `mod` in `s2_lift/mod.rs`.
- `infra/architecture.rs`: `switch_guard_bound: bool` field, struct-init `false`, reset-path
  `false`, `option` dispatch `"switchguardbound" => on_off!(...)`, **and** the ArchSeam copy
  in `build_arch_handle` (the dual-Architecture trap).
- `substrate/seams.rs`: `switch_guard_bound: bool` field + init `false`.
- `p0_knowledge/options.rs`: add `"switchguardbound"` to `KUNA_OPTION_NAMES`.
- `stages.toml`: `[[settable]]` row (source_decompiler="angr") + `[[surface]]` row.
- `jumptable.rs`: the new method + its call site.

Default **OFF** opt-in regardless of ablation (decider): the guard-bound heuristic asserts a
value correspondence across a memory round-trip it can't prove in dataflow, so a loose match
could over-bound an unrelated indirect jump → wrong output. `--option switchguardbound on`
per program.

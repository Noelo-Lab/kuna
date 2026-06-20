# w9x-arch-engine-glue
verdict: ACCEPT-WITH-LOSSES
verifier: independent-verifier (Claude Opus 4.8 1M)
date: 2026-06-13
round: 1
branch: worktree-wf_8847bd17-6d8-1 (867fc69)

gate: `cd rust && cargo test -p kuna-decomp --test decompile_e2e && cargo test --workspace`
  -> decompile_e2e: 2 passed (`five_corpus_functions_decompile_to_structurally_sane_c`
     produced 5, errored 5; `option_command_mutates_the_real_architecture` ok);
     workspace: ALL PASS. Re-run by the verifier fresh against the worktree.

## Summary

This item wires the merged subsystems into a single function-decompilation path
(`Architecture` god-object init + `option`-command mutation + the
build-flow/perform/print drive), the integration layer LOSS-127..LOSS-130 were
all deferred to. The verdict is **ACCEPT-WITH-LOSSES**: the end-to-end path is
**genuinely real, not faked** (the dominant REJECT trigger is absent), and every
transcribed init step is faithful, with two narrow behavioral losses + a one-step
init omission that the drive behaviorally compensates and that is single-line
fixable.

## End-to-end path is REAL (re-derived against the C++, not faked)

Verified by re-derivation + the verifier's own adversarial tests (below), NOT by
trusting the gate's structural-sanity check:

- **Bootstrap + init are real.** `init_post_engine` (architecture.rs) transcribes
  the tail of C++ `Architecture::init` (architecture.cc:1395) from `buildTypegrp`
  onward. The core-type `setCoreType` sequence is **verbatim** vs
  sleigh_arch.cc:215-237 (22 types, same order, same char-flag args, then
  `cacheCoreTypes`). `build_instructions` populates the real `TypeOp` table
  (adversarial test confirms BRANCH carries branch+coderef flags). The
  size/alignment setup (`finish_typegrp`) runs before `build_core_types`, which is
  the real C++ effect order (`setupSizes` at architecture.cc:1354 inside
  `parseCompilerConfig`, during `restoreFromSpec`, BEFORE `buildCoreTypes` at
  1410). NOTE the rationale comment in `init_post_engine` misattributes `setupSizes`
  to "the TypeFactory constructor" — it is actually in `parseCompilerConfig`; the
  *outcome* order is still faithful (doc nit, F4).
- **followFlow lifts genuine p-code.** Adversarial test
  `w9x_flow_follow_lifts_real_pcode` confirms a real corpus function
  (`boolless`) has a NON-EMPTY op-bank after `build_and_follow_flow` — a stub
  returning an empty `Funcdata` would fail. `is_proc_started()` is set,
  matching the C++ `followFlow`/`startProcessing` invariant.
- **The universalAction perform is the real loop.** `run_pipeline` derives the
  "decompile" root (`set_current` -> filtered clone of the universal tree) and
  calls the ported non-virtual `Action::perform` state machine
  (action.rs:473, ACCEPTED in w4-fw-action). Adversarial test
  `w9x_decompile_func_runs_perform_and_prints` drives it to completion (Ok) on a
  real function and asserts the printed C names the function with balanced braces.
- **PrintC produces real (body-less) C via the real Emit primitives.**
  `doc_function` drives `begin_function`/`begin_func_proto`/`open_brace_indent`/…
  (the actual prettyprint `Emit` driver). The empty body
  (`/* WARNING: body emission is the W9-emit RPN/Emit seam */`) is **pre-existing
  LOSS-117** — grep confirms NO `recurse`/`push_op`/`push_vn`/`emit_op`/
  `emit_block_graph` exists anywhere in the merged tree; the RPN body driver was
  explicitly deferred by the ACCEPTED w8-s9-printc / w8-s9-printlanguage-cast
  items. The body-less shell is therefore NOT a fake introduced here; it is the
  faithful output of the merged printer surface. The `void NAME(void)` signature
  is the documented W4/W6 proto-recovery seam (no recovered `fspec::FuncProto` on
  the merged `Funcdata`).

Demo output the gate produced (verified by re-run with `--nocapture`):

```c
void boolless(void)
{
  /* WARNING: body emission is the W9-emit RPN/Emit seam */
}
```

This is non-empty, structurally-sane C through the full bootstrap->decompile->print
path. NOT byte-parity (the W10 grind), which the gate correctly does not claim.

## ArchOptionContext faithfully mutates the real arch

`impl ArchOptionContext for Architecture` (architecture.rs) wires every
`glb->...` body to the owned subsystem. Spot-verified against options.cc:
- `defaultprototype` -> `set_default_model`: C++ LowlevelError "Unknown prototype
  model :"+p1 (options.cc:315, note the space-before-colon) — Rust
  `KunaError::lowlevel("Unknown prototype model :{name}")`. FAITHFUL kind+text.
- `evalcurrentmodel` -> `set_eval_current_model`: C++ ParseError "Unknown
  prototype model: "+p1 (options.cc:849, colon-SPACE) — Rust `KunaError::parse(
  "Unknown prototype model: {name}")`. FAITHFUL — the two distinct error KINDS and
  the two distinct colon-spacings are correctly distinguished.
- `readonly`/`nullprinting`/`extrapop`/`setaction` round-trips proven by the gate's
  `option_command_mutates_the_real_architecture` + the verifier's tests.

## Hunt list

- **Signedness**: clean. New comparisons are `align <= 8` (alignment, small
  positive), `language_index() < 0` (in test). No int4-vs-uint4 hazards in the
  ported logic.
- **Integer widths**: `self.max_instructions = val as uint4` — C++
  `max_instructions` is `uint4`, `val` is the already-range-validated `int4` from
  `OptionMaxInstruction::apply` (options.cc:982-994, which rejects <0 before
  calling); faithful re-tag. `get_addr_size() as int4` (addr size, bounded small).
  `opc as usize` (opcode enum, non-negative). All faithful; none carry `// cast:`
  comments (minor convention nit, F4) but all are provably bounded re-tags.
- **Wrapping**: N/A — the new code is accessor/registry glue with no arithmetic on
  uintb-lineage values.
- **Comparator totality**: N/A — no new comparators.
- **Iteration-order provenance**: `proto_models` is `BTreeMap<String,...>`,
  matching the C++ `map<string,ProtoModel*>` key order (correctly documented per
  ADR 0002). NO HashMap/HashSet in any ported logic (grep clean; the only
  `.unwrap()`s are in test code).
- **Off-by-one / do-while / reverse iterators**: N/A — no such loops introduced.
- **Erase-while-iterating**: N/A.
- **Exception -> Result partial-state parity**: the proto-model setters return
  `KunaError` faithfully (LowlevelError/ParseError/RecovError kinds matched to the
  C++ throw sites); `set_function_extra_pop`/`_inline`/`_no_return` return
  `RecovError("Unknown function name: ...")` matching options.cc:275 — these are
  the W4-symboltab seams (no resolvable function), recorded.

## Findings

- **F1 (loss / init unfaithfulness, compensated): `build_action` omits
  `resetDefaults`'s `setCurrent("decompile")`.**
  C++ `buildAction` (architecture.cc:590) -> `allacts.resetDefaults()`
  (action.cc:986) ends with `setCurrent("decompile")` (action.cc:1003), so after
  init `getCurrent()` is the derived "decompile" root (non-null) — and C++
  `IfcDecompile::execute` (ifacedecomp.cc:907) relies on that, calling
  `getCurrent()->reset/perform` WITHOUT a prior `setCurrent`. The Rust
  `build_action` (architecture.rs) calls only `install_universal` (which does
  `build_default_groups` + `set_universal`) and leaves `currentactname` EMPTY, so
  right after `init_post_engine` there is NO current action. The drive
  `run_pipeline` (decompile_drive.rs) papers over it by calling
  `set_current("decompile")` itself — a structural deviation from `IfcDecompile`.
  Observable end-to-end behavior is correct (the derived "decompile" root is
  faithful), but the init does not establish the C++ post-buildAction invariant.
  Pinned by adversarial test `w9x_decompile_func_runs_perform_and_prints` (asserts
  the current observed `!has_current_action()` post-init).
       cpp: decompiler/cpp/action.cc:1003 (setCurrent in resetDefaults);
            decompiler/cpp/ifacedecomp.cc:907 (drive relies on the invariant)
       rust: rust/crates/kuna-decomp/src/architecture.rs build_action;
             rust/crates/kuna-decomp/src/decompile_drive.rs run_pipeline
       -> LOSS-131. Fix: add `set_current("decompile")` (+ the actionmap reset of
          `resetDefaults`) to `build_action` and drop the compensating
          `set_current` from `run_pipeline`. Single-line.

- **F2 (loss): `set_default_extra_pop` drops the eval-model spread.**
  C++ global-extrapop branch (options.cc:280-284) sets extrapop on THREE models:
  `defaultfp`, `evalfp_current` (if non-null), and `evalfp_called` (if non-null).
  The Rust impl mutates ONLY `defaultfp`. `evalfp_current` is a real field that
  EXISTS on the merged `Architecture` (could be spread to); `evalfp_called` does
  not exist (a genuine seam). Observable only when `evalfp_current` is explicitly
  set to a model DISTINCT from `defaultfp` (via `option evalcurrentmodel`), which
  needs a multi-model cspec (itself a W6 seam). The trait doc says "(+ eval-model
  spreads)" but the impl skips them.
       cpp: decompiler/cpp/options.cc:281-284
       rust: rust/crates/kuna-decomp/src/architecture.rs set_default_extra_pop
       -> LOSS-132. Fix: after the defaultfp write, also spread to
          `evalfp_current` (and `evalfp_called` when that field lands).

- **F3 (no-op / acceptable hardening): defaultfp null-guard.**
  C++ `setDefaultExtraPop` dereferences `glb->defaultfp` unconditionally
  (assumes init seeded it). The Rust `if let Some(fp) = self.defaultfp` silently
  no-ops when null instead of panicking. Safe superset of the C++ post-init
  invariant; not a finding against behavior. Verified by adversarial test
  `w9x_extrapop_with_no_default_model_is_a_safe_noop`.

- **F4 (doc / convention nits, non-blocking):** (a) the `init_post_engine`
  comment misattributes `setupSizes` to "the TypeFactory constructor" — it is
  actually in `parseCompilerConfig` (architecture.cc:1354); the resulting order is
  still faithful. (b) the bare `as` casts (`opc as usize`, `get_addr_size() as
  int4`, `val as uint4`) lack `// cast:` justification comments though all are
  provably bounded re-tags. No behavioral impact.

## Mechanical pass

- `cargo run -p port-audit` — N/A (no `port-audit` crate in this tree, consistent
  with prior W-item reviews).
- grep `todo!`/`unimplemented!`/`HashMap`/`HashSet`/`sort_unstable` in the new
  code: CLEAN (no occurrences in ported logic).
- `cargo clippy -p kuna-decomp -- -D warnings` (lib): CLEAN. (A pre-existing
  `--all-targets` clippy error in heritage.rs:1602 test code is out of scope —
  not in this diff and not reached by the protocol's lib clippy gate.)
- No new compiler warnings from this diff (the `type_metatype::*`/`SeqNum` unused
  imports are pre-existing in typeop/tests.rs and double.rs).

## Adversarial tests (verifier-authored, landed with the verdict)

In `rust/crates/kuna-decomp/tests/verify_w9x_arch_engine_glue.rs` (all 5 pass):
- `w9x_flow_follow_lifts_real_pcode` — proves the IR is non-empty (path is real).
- `w9x_decompile_func_runs_perform_and_prints` — proves perform runs + prints;
  pins F1 (post-init `!has_current_action()`).
- `w9x_extrapop_with_no_default_model_is_a_safe_noop` — init-order boundary (F3).
- `w9x_unknown_model_is_an_error_not_a_panic` — faithful error kinds, no panic.
- `w9x_doc_function_braces_balanced_under_adversarial_name` — proves the
  structural-sanity invariant is not fooled by a brace-laden name.

## losses

LOSS-131 (F1, init `setCurrent` omission, drive-compensated),
LOSS-132 (F2, extrapop eval-model spread dropped).
Pre-existing depended-on losses correctly carried, not re-introduced: LOSS-117
(W9-emit RPN body), LOSS-127 (ifacedecomp engine sites — this item is the
integration layer those wire to but does not itself re-wire ifacedecomp).

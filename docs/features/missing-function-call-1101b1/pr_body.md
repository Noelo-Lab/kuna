## missing-function-call: recover a guard-bounded GCC PIC jump table (`option switchguardbound`)

Closes one gap where angr's decompiler beats kuna: angr
`test_decompiling_missing_function_call` (binary `adams`, function `main`, x86-64 GCC PIE).

angr recovers `main`'s getopt() dispatch as a proper `switch` (6 cases + default). kuna
fails to recover the jump table and emits `/* WARNING: Treating indirect jump as call */`,
collapsing the **entire switch and the surrounding getopt loop** into one bogus computed
`(*(code *)…)()` call — so every case body (`strdup`/`fprintf`/`atoi`/…) vanishes. The angr
test asserts exactly this is wrong (`fileno` present, the `-e`/strdup case present).

See [`analysis.md`](analysis.md) for the full instrumented root-cause.

### Root cause

The table at `0x96ac` is a textbook GCC PIC jump table whose index is range-guarded by
`sub 0x36; ja DEFAULT` (`idx LEu 0x36`, 55 entries) and **spilled to the stack** before the
table load. kuna's jump-table recovery runs early on the partial/truncated function, before
the `sub;ja` flag idiom is simplified and before the stack spill is collapsed, so
`JumpBasic::analyzeGuards` cannot bound the index (`jrange.size = 2^31 GT maxtablesize`).
The basic model is declined → `recoverAddresses` throws *"Too many branches"* →
`truncateIndirectJump` rewrites the BRANCHIND into a CALLIND.

### Mechanism

A new option-gated `JumpBasic` model-extension `kuna_try_guard_bound_table` (S2
switch-model), sibling of the GH-9191 `switchmodbound` modulo bound, invoked from
`recover_model_basic` after the normal model (and the modulo extension) fail. It EVALUATES
the guard boolean as a function of the index varnode `v` (resolving register-reused siblings
like `m = v + 0x3f` via a linear-offset map) and takes `N` = the first `v` whose routing
flips from `v = 0`'s — composing the simplifier's `idx GT 0x35 and m != 0x75` form to the
correct `N = 0x37` (55, including the boundary `'u'`/userlist case at idx `0x36`, which a
naive constant scan would miss). It re-binds the table index to `[0, N)`; the normal
`buildAddresses` + structurer then emit the switch.

### Option

`option switchguardbound on|off` (default **OFF**, opt-in; `kuna catalog` discoverable).
Flip per decompilation: `kuna decompile ./adams main --option switchguardbound on`.

### Ablation / parity / speed

- Ablation (flag default-ON over the 675-datatest corpus): **0/675 assertions changed** —
  the heuristic over-bounds no corpus switch.
- Parity: `make test` **PARITY OK** (675/675, byte-identical when off);
  `make test-stages` **PARITY OK** (164/164, incl. the new test).
- Speed (target `main`, off vs on): off ≈ 192.9 ms, on ≈ 259.1 ms (**+34.3%**, over the +5%
  budget — the ON cost is doing the full switch+loop recovery instead of truncating to a
  call; the gate-OFF path is unchanged). Per the speed gate the option ships **default-OFF
  opt-in** (`speed_forced_off`; no DIV entry — output byte-identical when off), matching the
  `switchmodbound` precedent + risk profile (the guard-to-index correspondence is asserted
  across a memory spill it cannot prove in dataflow).

A two-pass stage test `tests/stages/ghangr-missing-function-call-1101b1.xml` pins both
states (off = the computed-call bug; on = the recovered switch incl. the boundary
`case 0x36`).

`make rust-test` is green except the pre-existing `verify_w10_proto_unlock` failure (a
removed-C++-oracle test that fails identically on this branch without these changes —
confirmed via `git stash`).

🤖 Generated with [Claude Code](https://claude.com/claude-code)

# passthrough — implementation plan

## The change

A new P4 option `passthrough` (default off), in a `kuna_passthrough.rs` module
inside `p4_calls/`. It reads `protoorder`'s stated callee prototype to recover a
register a function forwards untouched to a callee that reads it, as a parameter
of that function, and the return value of a tail call it hands straight back.

## The seams it touches

- `RecoveredTypes` (`kuna_protoorder.rs`) gains `arity_sound` (the `lock`-mode
  arity bar, computed only when the option is on) and `output` (the callee's
  stated return). Nothing about the stated TYPES changes.
- `CalleeEntryDead::live_input_width` (`kuna_calleedeadarg.rs`) reports the width
  a callee body consumes, so a forwarded pointer read as `edi` is passed as `int`.
- `claim_untouched_registers` runs at the end of `ActionFuncLink` (claims only at
  `first_calls`); `claimed_range` keeps `check_input_trial_use` from scoring a
  trial on a claimed range; `capture` runs first in `build_input_from_trials`
  and `extend_pending` last in `ActionActiveParam::apply`;
  `suppresses_return_trial` guards `guard_returns`; `returns_tail_result`
  scores a RETURN trial in `ActionReturnRecovery`; `tail_return_type` feeds
  `call_output_type_local`.
- Two `Funcdata` fields carry the per-function claim state and the variadic-call
  set; both clear in `Funcdata::clear`.

## Termination

`claim_untouched_registers` is one pass over the call list; `first_calls` is a
monotone forward fixpoint over the blocks (a flag only ever turns on). `producing_call`
walks at most four single-predecessor blocks. `set_up_as_variadic` and
`reads_the_vararg_count` walk one basic block. `resolve_forward_transfer` and the
callee-body probes are `calleedeadarg`'s existing bounded walks.

## Tests

- Unit: `kuna_passthrough/tests.rs` — the option surface, claim overlap
  (`overlaps_claim`: width, neighbour, space), the extension plan (`extension`:
  leading run, stated order, stop at the first gap), and `live_input_width`.
- Stage: `tests/stages/kuna-passthrough.xml` — the negative control (the console
  states no prototype, so both passes render the forwarding functions without the
  parameter).
- CLI: `tests/cli/passthrough-gives-a-forwarding-function-its-parameter.json`
  (fixture `passthrough_x86_64`) and its option-off arm, over the whole
  binary-to-decompile path. The fixture carries controls that must keep every
  option-off argument: `noop(); twoarg(p,3)`, `vout(p); twoarg(p,3)` as a tail
  and a plain call, and tar's `sysinttostr`.
- Corpus: `lostargs.py` over a whole-binary off/on `--json` sweep must report
  `call_lost_args=0 param_lost=0`.

## Default

On, per `default-on-evaluation.md` and `dwarf-confirmation.md`. It shipped off
in round F because the tail-return arm inherited a callee's own wrong return;
`recovered_output` now refuses a return the callee did not compute in every byte
and on every path (`kuna_returnuncomputed::every_return_computes`), and the
whole-corpus check confirms 2,783 of 2,910 gained parameters and 4,012 of 4,088
gained returns against DWARF, with 0 parameters and 0 call arguments lost.

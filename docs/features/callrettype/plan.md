# callrettype: plan

## The seams

- `p4_calls/kuna_callrettype.rs (record)`: after a function's final decompile
  in callee-first order (`kuna-console` `project.rs`, where
  `park_recovered_proto` is set), keep its recovered return (storage, width,
  type) when it has no declared prototype, the type is a pointer, an integer
  wider than a byte or a float, and every live RETURN hands back a computed
  value (`kuna_returnuncomputed::every_return_computes_with`, globals count).
- `p4_calls/kuna_callrettype.rs (seed)`: copy the statements for every callee
  onto the caller's `Funcdata` at `protoorder`'s two seed points, minus the
  ones `refuse` withdrew for this caller; also the loader's data ranges.
- `p5_types/coreaction_infertypes.rs (call_output_type_local)`: an unlocked
  CALL whose output sits in the stated storage and width answers the stated
  type (`stated_return_type`) after `passthrough`'s tail arm, unless
  `output_refuses`, `declared_contradicts` or `accesses_disagree` refuse it.
- `p4_calls/kuna_callrettype.rs (in_place_extensions)` at `seed`, on the raw
  p-code: the instructions that only widen a register into its own container;
  `p3_dataflow/subflow.rs (RuleSubvarZext, RuleSubvarSext)` report when the
  subvariable flow they start narrows the returned value back through one
  (`note_returned_extension`); `declared_contradicts` refuses a statement of
  that width at the other sign for a returned result.
- `kuna-cli/src/decompile_all.rs (callee_vote_rounds, converge_callee_first)`:
  a redo the run discards puts the callee's earlier statement back
  (`statement`, `restore`).
- `kuna-console/src/decompile_step.rs (decompile_one_prefollowed)`: after the
  drive (and the format-string redo), `contradicted` audits the finished
  function; contradicted statements are withdrawn for the caller (`refuse`,
  `Architecture::kuna_callret_refused`) and the caller is driven once more.

## Evidence required (and where it is)

- Value preservation: `kuna-cli/tests/decompile_all_cli.rs
  a_call_result_typed_by_its_callee_round_trips_through_the_printed_c`
  compiles every printed fixture function with gcc and clang at -O0 and -O2,
  option off and on, and checks the program's output; the `cached` control
  pins the tie shape.
- Negative control stage test `tests/stages/kuna-callrettype.xml`; CLI probes
  `tests/cli/callrettype-*.json`.
- castbench full, typesweep (444 slices), whole-corpus hunks
  (`corpus.py`, `hunkclass.py`), speed (`speed.py`): `record.json`.

# structheadless -- plan

## Option

`structheadless off|closed`, P5, subphase `aggregate-union`, `change_kind =
structure-recovery`, tier `transform`, default `off`. Registered in
`p0_knowledge/options.rs`, applied in `infra/architecture.rs`, carried into the
`ArchSeam` in `build_arch_handle` and declared in `substrate/context.rs`.

## Code

* `p5_types/kuna_structheadless.rs` -- the mode and option parse; `admits`
  (a parameter of a closed function, under `closed`, that did not take the type
  its callers state); `is_headless` (two or more offsets, none zero);
  `yields_to_a_declared_pointer` (a callee's synthesized record does not replace
  a pointer a declared call gives the value -- a named record, a `char **`);
  `gives_a_pointee`; `types_a_pointer_as_a_word` (a headless record typing a
  dereferenced member as a word is refused as a vote); `bare_pointer_for` (a
  refused headless record is offered again as `void *`).
* `p4_calls/kuna_calleevote.rs` -- `uncommitted_under` / `decide_ledger_under`:
  under `closed` a headless record is open to the callers' vote, for a type from
  outside the recovery only (`char *`, `char **`, a declared record), and
  `input_vote` lets such a statement replace it.
* `p5_types/kuna_structsynth.rs` -- `accepts` computes `headless` (closed, the
  pointee not given from outside, and no address formed past the accesses,
  `points_past`) and passes it to `accepts_record`, which waives the offset-0
  condition for it; `wants_settle_pass` asks for the extra propagation pass for a
  headless candidate as it does for a lone field; `points_at_synthesized_record`
  and `points_at_headless_record` for the call-site rules.
* `p4_calls/kuna_protoorder.rs` -- `call_argument_vote` is split into the vote
  and `vote_holds`, so the `void *` fallback is held to every refusal the record
  was; the declared-pointer rule is one more refusal there, and
  `pointee_refuses` refuses a headless record that types as a word a member the
  function loads and reads through.
* `p5_types/kuna_structmerge.rs` -- doc only: a headless layout has no claim at
  0, and the agreement floor is unchanged.

## Tests

* Unit: `kuna_structheadless::tests` (option surface; the headless offset
  shape; what gives a pointee; a minted word), `kuna_calleevote::tests`
  (`a_callers_argument_vector_replaces_a_headless_record_only_under_the_rule`,
  `a_callers_synthesized_record_does_not_replace_a_headless_record`) and `kuna_structsynth::tests::a_record_read_only_past_its_start_needs_the_headless_rule`
  (`accepts_record` admits `ls`'s `sub_53e7` layout only under the rule, and
  still declines an index, a uniform run and one narrow field).
* Stage: `tests/stages/structheadless-closed-only.xml`, a negative control -- the
  console harness decompiles one function and never marks it closed, so both
  passes (`off`, `closed`) read through the untyped pointer and mint no record, as the
  `calleevote` stage does.
* CLI, the two-pass coverage over the whole binary-to-decompile path, on the
  fixture `kuna-analysis/tests/fixtures/structheadless_x86_64` (source beside
  it): `structheadless-off-leaves-a-headless-record-untyped.json` (off: every read
  through a cast `void *`) and `structheadless-closed-reads-a-record-past-its-start.json`
  (closed: `mode_of(struct_N *a0)` reads fields; the qsort callback `by_link`
  keeps `void *`); and on `structheadless_argv_x86_64`,
  `structheadless-closed-keeps-the-argv-its-caller-passes.json` (closed:
  `obsolete_count` reads only `argv[1]`/`argv[2]` of `main`'s `char **argv`, which
  `main` also hands `getopt`; both keep `char **` -- the build before the fix
  printed `struct_1 *` for both -- while `mode_of` is still a record).

## Instruments (in this directory)

`census.patch` (the refusal census build), `hunks.py`, `demotions.py`,
`layoutdepth.py`, `speed.py`. Driver scripts for the census, castbench,
typesweep and layout arms are in `record.json`'s `reproduce` block.

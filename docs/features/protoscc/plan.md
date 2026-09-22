# protoscc — implementation plan

## 1. The value

`protoorder` gains a fourth value, `cycles` (`ProtoOrderMode::Cycles`,
`p4_calls/kuna_protoorder.rs`): `types` for the members of a recursive component
too. `ProtoOrderMode::states_types_only` (`types` | `cycles`) routes the park to
`state_recovered_types`; `states_in_cycles` is read by the driver only. `lock` is
untouched and keeps refusing components.

## 2. The order (`kuna-cli/src/decompile_all.rs`)

`callee_first_plan` builds the direct-call edges as before and hands them to
`plan_from_components(edges, self_recursive, cycles)`:

- not `cycles`: Tarjan order, `park = size == 1 && !self_recursive` (unchanged);
- `cycles`: every step parks; a singleton (self-recursive or not) is one step; a
  larger component's members are emitted in `cycle_order`: an iterative
  depth-first walk over the component's own edges, roots = the members entered
  from outside the component (ascending) then the rest (ascending), each member
  emitted after the partners it reaches.

No second round (analysis.md table: +0.39 aggregate for about +35% on bash -O2). The
`structsynth` convergence sweep walks the same plan with the same park flags.

## 3. Default

`cycles` is the shipped default (`Architecture::reset_defaults`, the phases.toml
row, `docs/options.md`, `tests/fixtures/phase_catalog.json` recaptured). The flip
evaluation is `default-on-evaluation.md`.

## 4. Tests

- unit: `callee_first_plan_tests` (5) in `decompile_all.rs`; value parsing and the
  mode predicates in `kuna_protoorder/tests.rs`;
- integration: `recursive_callees_state_their_types_under_cycles`
  (`kuna-cli/tests/decompile_all_cli.rs`) over the new fixture
  `protoorder_cycles_x86_64` (`as && ld`, symbols kept);
- two-pass CLI probes: `tests/cli/protoorder-cycles-types-the-callers-of-a-recursive-function.json`
  (the default) and `tests/cli/protoorder-types-states-nothing-for-a-recursive-function.json`
  (`--option protoorder types`, the old behaviour).

A `tests/stages/` XML cannot show the change: the park is written only by the
whole-program drivers (`decompile_callee_first`), and the console path the stage
harness runs passes `park_recovered_proto: false`. `protoorder` itself shipped with
CLI probes for the same reason, and `tests/stages/kuna-argclobber.xml` documents
the limit.

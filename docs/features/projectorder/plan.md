# Plan

## The change

* `decompile_all.rs`
  * `callee_first_decision(prog, args, selected, whole)` — the one place that
    decides whether a whole-program run takes the callee-first order, and
    whether the option was named. `decompile_all` and `decompile_project` both
    ask it, so the two cannot drift.
  * `decompile_callee_first(prog, args, targets, explicit, base)` — the plan,
    the park and the convergence sweep, with the per-function
    `DecompileOptions` handed in. `decompile_entries_callee_first` is now a
    wrapper that supplies `decompile-all`'s own options.
  * `warn_protoorder_inert` keeps its two callers: `decompile-project --stream`
    and `decompile-graph`.
* `kuna-console/project.rs` — `export_options(single_target)`, the options
  `decompile_export_targets` already ran with, for a driver that owns the loop.
* `decompile_project.rs` — the serial arm takes the order; the `--jobs` arm
  passes `callee_first` as the pool's `serial_callee_first` (so the replay line
  names the serial run it actually replays) and prints the same note
  `decompile-all` prints. Its `--help` block, the `docs/cli.md` synopsis and
  `jobs.rs`'s determinism note say what that identity is against now: `--jobs 1
  --option protoorder off`, not plain `--jobs 1`. Measured on
  `i386_pie_nl`, `--jobs 4 --jobs-chunk 1` is byte-identical to the option-off
  serial export on all four artifacts and differs from the default serial
  export on `.c`/`.h`/`.asm`.
* `docs/cli.md --stream` and `docs/web-integration.md` — the two surfaces that
  keep the address-order schedule now say their prototypes are the option-off
  ones (measured: a streamed `.h`'s prototype set equals the option-off set and
  differs from the default by 18 lines on `i386_pie_nl`).

No new option: the behaviour is `protoorder`'s, and `--option protoorder off`
turns it off on this surface exactly as it does on `decompile-all`.

## Tests

* `decompile_project_cli.rs
  (a_struct_name_means_the_same_record_in_the_export_and_in_decompile_all)` —
  every `struct struct_N { … }` the export declares is the block
  `decompile-all --option structdefs on` prints for the same name, in both the
  default and the `protoorder off` arm. On `protoorder_floatpointee_x86_64`
  three of five blocks disagreed before the change and none does after.
* `tests/cli/sharded-project-export-keeps-serial-structs.json` — the pool's
  replay line now names `--jobs 1 --option protoorder off`, and the parent
  prints the callee-first note.

## Gates and measurements

`hunks.txt` (whole-corpus classification), `agree.txt` (the export now renders
every function the way `decompile-all` does), `layoutscore.log` (both arms),
`decompile-all-identical.txt` (the scored surface is byte-identical),
`speed.json` / `speed.py`.

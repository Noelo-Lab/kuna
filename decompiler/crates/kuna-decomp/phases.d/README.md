# settableTable: one file per option

Each `NNNN-<option>.toml` here is one `[[settable]]` row of the phase registry
(`../phases.toml` holds the group, subphase and surface tables). `build.rs` reads
the files in file-name order, and that order is the catalog order: `kuna catalog`,
`kuna docs options`, `SETTABLE_TABLE`.

- **Adding an option:** create `NNNN-<option>.toml` with `NNNN` one past the
  highest number here. Two branches picking the same number is fine: the files
  differ by name, they merge without a conflict, and the name breaks the tie.
- The file holds exactly one `[[settable]]` row (comments allowed), and
  `<option>` in the file name must equal its `option` field, and no two files
  may name the same option; the build fails otherwise.
- Every field is required except the live-reader triple. `live_field` /
  `live_true` / `live_false` give build.rs the typed `OptionValues` field and the
  live-value mapping (`flag ? live_true : live_false`, the C++
  `kunaLiveValue` ternary); an option with no live reader omits all three.
- Renumbering an existing file reorders the catalog: only do it deliberately.
- With a program loaded, every option that reports a live `current` value must
  report its `default` (kuna-console `verify_catalog_live_defaults`), so a default
  changed in a row but not in the engine, or the other way round, fails a test.

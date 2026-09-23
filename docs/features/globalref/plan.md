# globalref — implementation plan

## 1. Where the decision lives

P9, at the pointer arm of `PrintC::push_vn_explicit_ir`, after the string probe
and before the NULL token and the cast. A constant carries its type into that
arm whatever put it there (`ActionSetCasts` retypes a constant input instead of
inserting a CAST), so this is the one place every `(T *)0x<addr>` is printed.
The rewrite emits `&` and `dat_<addr>` bound to the constant Varnode for
markup; nothing in the IR changes, so variables, types and `type_match` cannot
move.

## 2. The facts it reads

- `Architecture::globalref_ranges`: the sections the object loader classifies as
  holding program data (`kuna-analysis/src/loader/kuna_globalref.rs`), installed
  by the object bootstrap beside the `litpoolconst` ranges. Empty without a
  section table, so the XML datatest corpus is untouched by construction.
- `kuna_globalref::plan`, once per function: the constant values read as numbers,
  the direct accesses to the data space, and the pointed-to types each in-range
  address is read at.

## 3. The declaration

`decompile_drive::extract_global_objects` reads the minted globals and the
unnamed direct accesses back off the printer after `print_c`;
`FuncResult::globals` carries them (and the `--jobs` wire); the project header
declares them (`project.rs::global_declarations`), never as a scalar that
disagrees with a direct access. `names_any_type` reads the declarations so a
`struct_N` only a global names survives the convergence sweep and pruning.

## 4. Default

On. The datatest corpus cannot see it (no section table), no existing stage or
CLI-probe assertion is about the cast itself (four probes pinned the spelling
incidentally and now accept both), the output with the option off is
byte-identical to main, and the type sweep is identical in every slice.

## 5. Tests

- `tests/stages/kuna-globalref.xml`: two passes over `globalref_x86_64` (off,
  default), libc-typed witnesses so the pointer type does not depend on
  decompile order, plus a direct-access control and a numeric control.
- `decompile_all_cli.rs::a_constant_address_named_as_a_global_round_trips_through_the_printed_c`:
  both arms, gcc and clang, names placed at their addresses, run against the
  binary's own output.
- Unit tests: the range lookup and the loader classifier, the per-address type
  merge, which direct accesses may share storage, the adopted direct type, and
  option parsing.

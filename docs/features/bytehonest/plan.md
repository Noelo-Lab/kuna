# bytehonest — implementation plan

## 1. The change

New module `p6_variables/kuna_bytehonest.rs`, two functions:

```rust
pub fn is_uncommitted_byte(dt: &Rc<Datatype>) -> bool {
    dt.get_metatype() == type_metatype::TYPE_UNKNOWN && dt.get_size() == 1
}

pub fn exported_type_name(arch: &Architecture, dt: &Rc<Datatype>) -> String {
    if arch.byte_honest && is_uncommitted_byte(dt) { return "undefined1".into(); }
    crate::printc::type_to_c_string(arch, dt)
}
```

`infra/decompile_drive.rs (extract_variables)` sections 1 (FuncProto parameters)
and 2 (ScopeLocal stack symbols) call `exported_type_name` where they called
`printc::type_to_c_string`. Nothing else moves: the exported `name`,
`stack_offset`, `size`, `is_param` and `arg_index` are built from the same values
as before, and section 3 keeps its own (wider) `frame_slot_type_name` rule.

Deliberate non-choices:

- **Not the section-3 rule.** Reusing `frame_slot_type_name` would rewrite arrays
  of unknown and turn a `char[264]` local into `undefined264`.
- **Size 1 only.** `undefined2/4/8` already reach the surface through the printer;
  only the one-byte carrier renders as a *committed* C type.
- **The datatype, not the spelling.** A string test against `"char"` would also
  catch a genuine `char`, and would do nothing with `realtypes` off.
- **Applies with `realtypes` off too**, where the alternative export is the
  internal `xunknown1`.

## 2. Wiring

`Architecture::byte_honest` (field + `reset_defaults` = ON + `set_kuna_option`
arm), a `[[settable]]` row in `phases.toml` (P6, `naming-policy`,
`change_kind = presentation-default`, `tier = core`, `live_field = byte_honest`),
and the option name in `p0_knowledge/options.rs`. Count sites moved by
`scripts.repipe.counters --rederive/--fix` on a freshly built tree (settables
205 → 206, core tier 66 → 67, catalog JSON records 204 → 205, the byte-compat
fixture recaptured) plus the one manual site `counters` does not derive,
`option_values_live_value_present_for_85` → `_for_86`. `docs/options.md`
regenerated.

## 3. Default-ON evidence

- 0/675 datatest assertions and PARITY OK on stages: no datatest reads the JSON
  variables surface at all, and the emitted C is byte-identical.
- Bidirectional typesweep (decbench `type_match`), both arms the same binary.
- Speed: one metatype+size comparison per exported row; `timeit` block in
  `record.json`.

## 4. Tests

- `p6_variables/kuna_bytehonest/tests.rs` — the predicate: the byte, the wider
  unknowns, the array of unknown (including the 1-element array, whose *size* is
  1), the three committed one-byte types, and the spelling-independence case.
- `kuna-console/tests/verify_bytehonest.rs` — two-pass end-to-end on the vendored
  `fauxware`: same functions, same variable count, same name/offset/size/arg
  index; the only admissible difference is `char` → `undefined1` at size 1, and
  the fixture must exercise it (non-vacuous).
- `tests/cli/bytehonest-uncommitted-byte-arg{,-option-off}.json` — the two-pass
  pair on the CLI surface itself (`i386_pie_nl`,
  `close_stdout_set_ignore_EPIPE`): default → `undefined1`, `--option bytehonest
  off` → `char`, and both arms pin the unchanged C text.

A `tests/stages` XML two-pass test is not possible here: the stage corpus asserts
on console output (`decompile`, `print C`), and this option changes only the
`decompile-all --json` `variables` array, which no console command emits. The
`tests/cli` pair is the two-pass gate for it, and `verify_bytehonest.rs` is the
in-process equivalent over every function of a fixture.

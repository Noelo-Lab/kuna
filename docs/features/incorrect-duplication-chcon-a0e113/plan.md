# incorrect-duplication-chcon-a0e113 — plan

**Option**: `noreturn_externmatch` (default-OFF during dev; ablation decides ship).
**Module**: `decompiler/crates/kuna-decomp/src/s2_lift/kuna_noreturn_externmatch.rs`.
**ElementId**: `4103` (max in use is 4102).
**Stage**: S2 (lift / flow follow) — `FlowEnvironment::query_call_no_return` seam.

## Mechanism

`query_call_no_return(entry)` currently returns
`symboltab.function_is_no_return_across_scopes(entry)` (the proto flag, set by
`option noreturn <name>` or the address-keyed `noreturn_known` scan). When the
new flag is on, *also* return true if the callee name at `entry`
(`function_display_name_across_scopes`) matches the vendored ELF known-no-return
list. flow.rs:1907 ORs this query with the proto flag at the artificialHalt site,
so a name match plants the halt and the dead padding after the call is never
decoded.

On a normal dynamically-linked ELF the proto flag is already set by
`noreturn_known`, so the OR is a no-op → byte-identical. The only behavioral
change is on ET_REL `.o` undefined externs (`address()==0`, no PLT) that the
address-keyed scan missed — exactly the chcon.o `__stack_chk_fail` case.

## Anchors (minimal, each marked `// (kuna)`)

1. `s2_lift/kuna_noreturn_externmatch.rs` (NEW): `ELEM_NORETURN_EXTERNMATCH = 4103`;
   `is_known_noreturn_name(name) -> bool` (strip leading `_`, upstream namespace
   guard global/`std`, match the vendored list pulled via
   `include_str!("../../../kuna-analysis/data/ElfFunctionsThatDoNotReturn")`).
2. `s2_lift/mod.rs`: `pub mod kuna_noreturn_externmatch;`
3. `infra/architecture.rs`: `noreturn_extern_match: bool` field + ctor default
   (false) + reset (false) + `"noreturn_externmatch" => on_off!(...)` dispatch.
4. `infra/decompile_drive.rs`: gated extra check in `query_call_no_return`.
5. `p0_knowledge/options.rs`: add `"noreturn_externmatch"` to `KUNA_OPTION_NAMES`.
6. `stages.toml`: settableTable row (all provenance fields).

## Decider decision (recorded verbatim in record.json)

verdict=implement, scope=small, hook_point=query_call_no_return,
list_strategy=include_str! full vendored list. Risks: name collision on a
returning local named like a listed token (mitigated by namespace guard +
default-OFF pending 0/675 ablation); flow-query-only sets the flow halt but not
the proto flag (matches the documented W4-era seam pattern; dead-code drop is at
flow time so output matches the proven `option noreturn` path).

## Tests / gates

- `tests/stages/ghangr-incorrect-duplication-chcon-a0e113.xml`: two-pass — pass 1
  `option noreturn_externmatch off` asserts the garbage padding line is present;
  pass 2 default asserts it is gone.
- `make test-stages`, `kuna test --all --baseline docs/baseline.json` (PARITY OK),
  `kuna catalog --check`, speed off-vs-on.

# structdefs — implementation plan

## 1. The module (`p9_emit/kuna_structdefs.rs`)

- `OptionStructDefs` — the `on|off` parse, the kuna option idiom
  (`ELEM_STRUCTDEFS = 4174`); the caller (`Architecture::set_kuna_option`) does
  the printer write.
- `referenced_types(fd)` — roots (prototype output + params, every Varnode
  data-type, every HighVariable's mapped Symbol type) then a postorder DFS
  (`visit`) through the typedef base and `Datatype::get_depend`, pushing each
  `is_definable` type after everything it depends on. The mark set keys on `Rc`
  object identity, exactly as `TypeFactory::orderRecurse` does, so a pointer
  cycle terminates with one entry per type.
- `is_definable` — the `render_type_definitions` predicate: non-core, named,
  not an internal partial, and a typedef / struct / union / enum.
- `definition_text` / `dedup_by_name` — the machine-readable half (below).

Ordering is computed over the referenced subset rather than by filtering
`TypeFactoryImpl::dependent_order()`, which walks the WHOLE factory: per
function that would make a whole-binary run quadratic in the program's interned
types instead of linear in the types a function touches.

## 2. The printer hook

`PrintC::emit_type_definition_preamble(fd)`, called from
`emit_function_document` after the `rt_ctx` resolution and before
`begin_function`. Returns immediately unless `options.struct_defs`. The text is
`render_type_definitions(&types, self.rt_ctx)` — the `.h` renderer, now
`pub(crate)` along with `compose_type_body` / `compose_enum_body` /
`compose_typedef_line` / `sanitize_type_name` and `RealTypeCtx::from_arch` —
emitted as whole lines (`tag_line` + `print`), since a type definition has no
`PcodeOp`/`Varnode` for the markup back-end to bind to.

Placing it inside `emit_function_document` rather than in the CLI is what makes
`kuna decompile`, `decompile-all`, the console `print C` and the datatest driver
one surface; it is also why the two-pass stage test can drive the whole feature
through `print C`.

## 3. The project-export suppression

`decompile_pulled` (kuna-console `project.rs`) clears the printer flag for the
duration of a `want_proto` batch — `want_proto` is exactly the
`decompile-project` surface, since it is what asks for the `.h` prototype — and
restores it after, so one loaded program can drive both surfaces. This reaches
the serial, `--jobs N` and `--stream` exporters alike, because all three render
bodies through that one function.

## 4. The JSON surface

`decompile_drive::extract_type_definitions` → `FuncResult::types` →
`decompile-all --json` `functions[].types[]` (`{name, definition, size}`).
Additive: the key is always present, and empty unless the option is on. The
record travels through the `--jobs` worker codec (`kuna-cli/src/jobs.rs`), since
a pooled worker renders the C and the parent has no `Funcdata` left to
re-derive from.

`dedup_by_name` resolves the one shape the text renderer already handles and a
JSON consumer cannot: a DWARF image holds both the forward-declared
`struct _IO_FILE` and the defined one, and two records for one name reporting
size 0 and size 216 is a contradiction. The complete definition wins; an opaque
type with no complete twin is still reported.

## 5. Tests

- 7 unit tests in `kuna_structdefs/tests.rs` over hand-built `Datatype`s: the
  pointee pull-in, nested ordering, cycle termination, the core/unnamed
  refusals, the opaque forward declaration, a complete body, and the dedup rule.
- `tests/stages/kuna-structdefs.xml` — two `print C` passes over one decompiled
  function (`parse line struct mystruct { … };`), default then
  `option structdefs on`: the definition appears exactly once, and the signature
  and body lines appear exactly twice (so the flip is proven to add the preamble
  and change nothing else).
- `tests/cli/structdefs-json-types-array.json` + `-key-off.json` — the `--json`
  `types` contract, on and off.

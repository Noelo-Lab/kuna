# cppproto — implementation plan

Owning phase: **P1 program prep**, the DWARF analyzer tier
(`docs/spec/01-program-prep.md`). Option `cppproto`, tier `analysis`,
`change_kind` `analysis-enablement`, default-**ON**.

## 1. The module — `analyzers/dwarf/kuna_cppproto.rs` (new)

Sits beside `mod.rs` as a child module, so it can read `DieSnap`'s private
fields, and reads the same `BTreeMap<offset, DieSnap>` snapshot the base walk
already builds — no second parse.

* `resolve_subprogram(sub, dies) -> Option<ResolvedSub>` — fuse a definition with
  the declaration reached through its **one-hop `DW_AT_specification`** link
  (the reduction of Ghidra's `DIEAggregate`). Name, return type and parameter
  list come from whichever DIE carries them. `chased` records whether the name
  came from the declaration, so the base arm's symbols are not duplicated.
* `qualified_name(die, dies)` — walk the DIE parents, collecting
  `DW_TAG_namespace`/`class_type`/`structure_type`/`union_type` names outermost
  first (`DWARFName`). Bounded by `MAX_SCOPE_DEPTH`; an anonymous ancestor
  contributes nothing.
* `formal_parameters(die, dies)` — the ordered `DW_TAG_formal_parameter`
  children, **flattening `DW_TAG_GNU_formal_parameter_pack`** (a variadic
  template's expanded arguments live one level down).
* `build_pieces(res, dies, types, word_size)` — the `PrototypePieces`. Each
  parameter goes through `build_param_type`, which degrades rather than fails.
* `build_param_type` / `degrade_datatype` — a type the switch cannot map, OR a
  built type of non-positive size (a by-value opaque aggregate), becomes an
  `undefined<n>` at the DIE's own `DW_AT_byte_size`. Width is what parameter
  storage assignment reads, so a plausible width keeps the rest of the list on
  its real storage.
* `collect_fbreg_locals(...)` — the stack-local collector for a chased
  definition, with the same direct-children / single-`DW_OP_fbreg` scope as the
  base one, plus a zero-width-type skip.

**Rejected, with the measurement in `analysis.md` section 4.1:** following
`DW_AT_abstract_origin` for the subprogram link. It doubles the recovered
function count on a C++ library, and it also picks up GCC IPA clones whose
parameters alias one source parameter — which corrupted coreutils `fmt`.

## 2. `analyzers/dwarf/mod.rs`

* `DieSnap` gains `origin_ref` + `origin_is_spec` (which attribute supplied it)
  and `parent` (for the ancestry walk); `snapshot_unit` fills them.
* `build_datatype` gains a `cpp: bool`. With it false the tag switch is
  byte-identical to before, which is what makes the gate-off path exact. With it
  true: `DW_TAG_class_type` maps like a structure, `DW_TAG_reference_type` /
  `rvalue_reference_type` like a pointer, and the transparent qualifier chain is
  collapsed by `strip_qualifiers` **before** the depth budget is charged.
* `strip_qualifiers` also returns the innermost typedef name, which
  `aggregate_name` uses to name an anonymous aggregate.
* The subprogram arm keeps its existing body verbatim under
  `if !snap.name.is_empty()`, then runs the C++ arm into `out.cpp_dwarf`.

## 3. The gate — where it lives and why

The DWARF pass runs at `load file`, upstream of the `option` commands, so a flag
read inside the pass would see only the default. The C++ facts are therefore
always computed and stashed in a separate `AnalysisOutput::cpp_dwarf`
(`CppDwarfFacts { symbols, locals, prototypes }`), and
`engine.rs::commit_analysis_output` folds them into the normal streams when
`arch.analysis_cppproto` is on and drops them when it is off. That also makes the
flag settable from a test without touching process environment.

## 4. Address-keyed prototype binding

`CppDwarfFacts::prototypes` is `Vec<(u64, PrototypePieces)>` keyed by
`DW_AT_low_pc`. Three small engine additions carry it:

* `Architecture::set_function_prototype_pieces_at(addr, pieces)` —
  `find_function_across_scopes` then `set_function_proto_pieces`.
* `Database::function_proto_pieces_across_scopes(addr)` — the drive's
  own-prototype read (`decompile_drive.rs`), widened past the global scope.
* `Database::build_callee_proto_pieces` — the callee snapshot, likewise widened.

Both widenings are inert before this change: the by-NAME park resolves through
the global scope's `queryByName`, which cannot reach a nested-scope function, so
nothing outside the global scope ever carried parked pieces.

The by-name park still runs first and the address-keyed park second, so the
address-resolved signature wins wherever both reach the same function.

## 5. Tests

`kuna-console/tests/verify_cppproto.rs` — seven two-pass assertions over the new
`cppproto_x86_64` fixture (out-of-line member, `const` member, reference
parameter, `static` member, namespace function, template instantiation,
class-typed pointer parameter). Each runs the fixture twice, once with the option
off (reproducing the bug) and once at the default (the fix). The stage-XML corpus
cannot host this: `run_default_analyses` is bound to the real-ELF bootstrap and
the `<binaryimage>` datatest path never calls it.

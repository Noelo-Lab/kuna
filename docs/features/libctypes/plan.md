# `libctypes` — plan

## Goal

Make a pointer to a libc/POSIX aggregate spell its name on the surface decbench
scores (`kuna decompile-all --json`'s `variables[]`), where every such slot is
`void *` today and `void *` never matches ground truth `FILE *`.

## Shape

| | |
|---|---|
| option | `libctypes`, values `off\|opaque`, default `off` |
| phase / subphase | P1 / `external-refinement` |
| tier / change_kind | `analysis` / `analysis-enablement` |
| owner | `kuna-analysis/src/analyzers/protos/kuna_libctypes.rs` (`LibcTypesPass`) |
| gate | load-time env bridge `KUNA_LIBCTYPES` (`kuna-decomp/src/p0_knowledge/kuna_libctypes.rs`) |
| spec | `docs/spec/01-program-prep.md`, prototype-sources section |

## Steps taken

1. `Ty::NamedPtr(&'static str)` in `protos/mod.rs` plus one `build_ty` arm —
   pointer-only, so a sizeless by-value or by-value-return slot is
   unrepresentable.
2. `NAMED_AGGREGATES`: 16 names with the platform's own `sizeof`/`_Alignof`,
   plus `DIR` at width 1. Minted through `get_type_struct` +
   `set_fields_struct_raw` with an EMPTY field list, the real width and
   `flags::type_incomplete` passed back — sized (so no `PTRSUB(p,off)` renders)
   and still completable by the DWARF importer.
3. `LIBC_NAMED` / `LIBC_EXT_NAMED`: hand-audited restatements of the shipped
   signatures with their aggregate slots named, each keeping the shipped arity
   and variadic slot, plus 9 new stdio names under the `gcc -aux-info`
   provenance rule. The `v*` family is excluded by name.
4. `LibcTypesPass`, registered after `LibProtoPass`/`LibcSigsPass` so its
   prototypes are merged last and therefore committed last (both the by-name
   step 5 and the address-keyed step 5c of `commit_analysis_output`), and
   self-gated on the env bridge at the top of `run()`.
5. The same gate on `declared_libc_prototype`, so `--define-function …=fopen`
   agrees with what the pass parks on an imported `fopen`.
6. Env bridge wired through the console `option` arm, `kuna decompile`'s
   subprocess env and `decompile-all`'s `is_loadtime_gate` / `apply_loadtime_env`.
7. Two-pass stage test `tests/stages/kuna-libctypes.xml` over a real ELF
   (`load file`, the vendored coreutils `fmt`): off = `fclose(void *)` /
   `void * fopen(...)`, on = `fclose(FILE *)` / `FILE * fopen(...)`, plus two
   assertions that the DWARF definition of the same bare name still wins and
   keeps its field layout in BOTH passes.
8. 10 unit tests: `Rc::ptr_eq` on a twice-minted `FILE`, every shell sized and
   incomplete, a completed name left alone, every `NamedPtr` has a width, every
   retarget preserves arity/varargs, the `va_list` family absent, `getdelim`'s
   fourth slot, `__uflow` new to this table, no duplicate names, and the
   declared-name lookup following the gate.

## Default

Ship `off`, with the bidirectional corpus typesweep in `record.json` as the
evidence for the follow-up flip. The 675 datatest assertions are structurally
untouched (no datatest loads a file), so the usual "0/675" is not evidence here;
the sweep is.

## Not in this PR

* `stdin`/`stdout`/`stderr` as typed `FILE *` data symbols (own sweep).
* A `glibc` value installing the public `_IO_FILE` field names.
* `obstack` / `regex_t` (call-site counts measured in `analysis.md`).

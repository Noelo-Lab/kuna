# `libctypes` — plan

> The default moved to `opaque` after this was written; `flip.md` is the record
> of that change and what it moves. The rest of this document describes the
> option as it shipped.

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
   pointer-only, because no libc declaration restated here passes or returns an
   aggregate by value. That is a property of the table: propagation still
   reaches by-value positions (`timespec sub_10210(void)` on `-O2` `ls`), and it
   is the real WIDTH on every shell, not the pointer, that keeps the ABI
   classifier out of the hidden-return-buffer case. See analysis.md §7.
2. `NAMED_AGGREGATES`: 16 names with the platform's own `sizeof`/`_Alignof`,
   plus `DIR` at width 1. Minted through `get_type_struct` +
   `set_fields_struct_raw` with an EMPTY field list, the real width and
   `flags::type_incomplete` passed back — sized (so no `PTRSUB(p,off)` renders),
   incomplete so the `.h` emitter declares it opaque. A name already held is
   ADOPTED when it is a struct of the declared width (the platform's own, on a
   `-g` image; `FILE` also looks under `_IO_FILE`) and declines the signature
   otherwise.
3. `LIBC_NAMED` / `LIBC_EXT_NAMED`: hand-audited restatements of the shipped
   signatures with their aggregate slots named, each keeping the shipped arity
   and variadic slot, plus 9 new stdio names under the `gcc -aux-info`
   provenance rule. The `v*` family is excluded by name.
4. `LibcTypesPass`, registered after `LibProtoPass`/`LibcSigsPass` so its
   prototypes are merged last and therefore committed last (both the by-name
   step 5 and the address-keyed step 5c of `commit_analysis_output`), and after
   `DwarfPass` so the shells it would mint meet the platform's own definitions
   first — completion re-keys a struct into a new `Rc`, so a pointer built
   against a shell before DWARF completes it is stranded on the empty one.
   Seeded from IMPORTED names only, so a DWARF-defined function is never
   outranked. Self-gated on the env bridge at the top of `run()`.
5. The same gate on `declared_libc_prototype`, so `--define-function …=fopen`
   agrees with what the pass parks on an imported `fopen`.
6. Env bridge wired through the console `option` arm, `kuna decompile`'s
   subprocess env and `decompile-all`'s `is_loadtime_gate` / `apply_loadtime_env`.
7. Stage test `tests/stages/kuna-libctypes.xml`, three `load file` passes over
   two real ELFs. Passes 1/2 use `libctypes_stat_x86_64`, written so the two
   halves are separable: it never names `FILE`, so the table must MINT one
   (off = `fclose(void *)` / `void * fopen(...)`, on = `fclose(FILE *)` /
   `FILE * fopen(...)`), and it hands `stat` a real `struct stat *`, so
   `st->st_mode` must read the same in both passes with no `field_0x` anywhere —
   the assertions that fail if a shell is minted over the DWARF definition.
   Pass 3 loads the coreutils `fmt`, whose DWARF DOES define the stream type, and
   pins the adoption of `_IO_FILE` with its real layout.
8. 13 unit tests: `Rc::ptr_eq` on a twice-minted `FILE`, every shell sized and
   incomplete, a completed name left alone, every `NamedPtr` has a width, every
   retarget preserves arity/varargs, the `va_list` family absent, `getdelim`'s
   fourth slot, `__uflow` new to this table, no duplicate names, the platform
   spelling `_IO_FILE` adopted for `FILE`, a zero-width forward declaration
   declined, and the
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

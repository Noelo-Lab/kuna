# [PROPOSAL] Name libc-extern data globals from `.dynsym` / `.symtab` (`dat_20a098` → `optind`)

Part of the **kuna → IDA Pro parity** program (reference oracle: IDA 9.2 on
`tests/x86_64/decompiler/fmt`). Follow-up to the merged data-global naming feature
(PR #151), which named the program's *own* DWARF globals.

## The problem

`fmt/main` still renders the four libc-extern data objects as raw addresses:

| kuna | IDA Pro | ELF symbol |
|---|---|---|
| `dat_20a098` | `optind` | `optind@@GLIBC_2.2.5` (`.dynsym` `STT_OBJECT`) |
| `dat_20a090` | `stdin` | `stdin@@GLIBC_2.2.5` |
| `dat_20a088` | `stdout` | `stdout@@GLIBC_2.2.5` |
| `dat_20a0a0` | `optarg` | `optarg@@GLIBC_2.2.5` |

PR #151 recovers globals declared in the *program's* DWARF (`max_width`, `goal_width`,
`prefix*`). These four are **imported libc objects** — they appear only as `STT_OBJECT`
entries in `.dynsym` (and `.symtab`), never in the program's `.debug_info`, so #151 does not
reach them.

**Root cause.** The loader (`decompiler/crates/kuna-analysis/src/loadimage_object.rs:293,327`)
keeps only `SymbolKind::Text` from `file.symbols()` / `file.dynamic_symbols()` — it drops
every `SymbolKind::Data`. (`lib.rs:29` already flags "`.symtab`/`.dynsym` symbol reader" as a
future item.) A copy-relocated libc global like `optind` has a defined address in the main
binary (a `.bss` `R_X86_64_COPY` slot), so its name+address are available; kuna just never
surfaces them.

## The IDA / Ghidra reference

Both name data globals from the symbol table (`.symtab`/`.dynsym` `STT_OBJECT`), independent
of DWARF. IDA additionally types them from its libc type library; this proposal covers **naming
only** (typing is out of scope — the `undefined<size>` default from #151 applies).

## Proposed implementation

1. **Loader** (`loadimage_object.rs`): in the two symbol loops, additionally collect
   `SymbolKind::Data` defined symbols with a non-empty name and a size into a
   `datasyms: Vec<DataSym{addr,name,size}>` (parallel to `funcsyms`); expose a
   `data_symbols() -> Vec<(u64,String,u64)>` accessor (mirror `func_symbols()`). For a copy-reloc
   extern the `object` symbol address is the `.bss` slot; use it directly.
2. **Engine** (`engine.rs`): reuse the exact `commit_analysis_output` data-object install path
   PR #151 added — `add_symbol_mapped(global_scope, name, get_base(size, TYPE_UNKNOWN), addr, …)`
   with `namelock`, skip-if-occupied. (A `DataObjectFact` already exists; feed the loader
   datasyms through the same field, or add a sibling loader-symbol commit.)
3. **De-dup vs #151**: the skip-if-occupied guard already prevents shadowing a DWARF-named
   global; DWARF wins where both exist.

## Default policy

Default-on, no flag (per the parity program's decision — a clearly-correct naming improvement).
Re-pin `docs/baseline.json` / `docs/baseline-stages.json` only if a datatest binary carries a
named `.dynsym`/`.symtab` data global (none is expected; the corpus is bytechunk/`.o`).

## Speed / risk

- **Speed:** load-time only (a handful of `add_symbol_mapped` calls). No decompile impact.
- **Risk:** LOW. Naming-only; typed `undefined<size>` so type propagation is unchanged. The
  only correctness concern is a wrong address for a copy-reloc extern — mitigated by using the
  `object` crate's resolved defined address and the skip-if-occupied guard.

## Testing

End-to-end `verify_*.rs` on the vendored `regglobal_fmt_x86_64` fixture asserting `main`
renders `optind`/`stdin`/`stdout`/`optarg` (not `dat_20a0??`). Gates: `make test`,
`make test-stages`, `make rust-test`.

## Effort

**Small–Medium.** Reuses #151's install path; the new work is the loader `SymbolKind::Data`
collection + accessor + threading. ~1 focused PR.

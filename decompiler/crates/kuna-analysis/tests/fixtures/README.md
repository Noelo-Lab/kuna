# ELF test fixtures

Small, real, dynamically-linked ELF binaries used by the loader gates in
`loadimage_object.rs`'s test module (PLT/GOT import-name resolution — see
`src/elf_plt.rs`), the analysis-pass unit tests (`s1_demangle`, `s1_protos`,
`s1_entry`, …), and the console e2e gates
(`kuna-console/tests/verify_w11_elf_plt_names.rs`,
`kuna-console/tests/verify_s1_entry.rs`).

The XML datatest corpus cannot exercise these: it embeds raw bytechunks with
explicit `<symbol>` definitions and never constructs an `ObjectLoadImage`, so the
ELF loader (and thus PLT resolution) is off that path. These fixtures drive the
real ELF parser.

| File | What | Exercises |
|---|---|---|
| `fauxware` | classic non-PIE x86-64, not stripped (the angr `fauxware` sample) | `.plt` classic stubs (`FF 25` rip-rel), `.symtab` defined functions; `.eh_frame` FDE starts (`s1_entry`: 7 FDE starts incl. `_start`/`main`/`register_tm_clones`) |
| `cet_pie_x86_64` | PIE x86-64 with CET (`.plt.sec`) | `endbr64; FF 25` CET stubs, naming at the `.plt.sec` call target |
| `stripped_dynamic_x86_64` | PIE x86-64, `.symtab` stripped (only `.dynsym`) | PLT resolution with no `.symtab` (dynsym/rela.plt only); entry discovery (`s1_entry`): `e_entry`=0x1160, `DT_INIT`=0x1000, `DT_FINI`=0x1464, INIT/FINI_ARRAY ptrs, `_start`→`main` idiom → 0x1405, `.eh_frame` FDE starts — `sub_1405` (main) decompiles without `--addr` |
| `cpp_mangled_x86_64` | non-PIE x86-64 C++, not stripped | symbol demangling (`s1_demangle`): a defined `.symtab` C++ method `_ZN3foo3Bar3bazEi` must surface name-only as `foo::Bar::baz` |
| `dwarf_stripped_x86_64` | non-PIE x86-64, **`.symtab`/`.dynsym` FUNC names removed but `.debug_*` kept** | DWARF recovery (`s1_dwarf`): names + typed signatures of `add_values`/`compute`/`main` come **only** from `.debug_info` (the funcsym stream has none) |
| `switchtab_x86_64` | non-PIE x86-64, dense `switch(x){0..7}` | address/jump tables (`addrtable`): an absolute 8-byte jump table in `.rodata` at vma `0x402008` (`jmp *0x402008(,%rdi,8)`) |

Provenance: `fauxware`, `cet_pie_x86_64`, `stripped_dynamic_x86_64` copied
verbatim from `bs-artifacts/binaries/` (`fauxware`, `debug_symbol`,
`debug_symbol_mod_stripped` respectively). `cpp_mangled_x86_64` was built locally
with `g++ -O0 -no-pie -fno-pic` from a tiny `namespace foo { struct Bar { void
baz(int); }; } void foo::Bar::baz(int){...} int main(){...}` source.
`dwarf_stripped_x86_64`: `cc -g -O0 -no-pie -fno-pic t.c -o x` then
`objcopy --wildcard --strip-symbol='*' x dwarf_stripped_x86_64` (empties the symbol
table, keeps `.debug_*` — so DWARF is the sole name source; `t.c` = three funcs
`add_values`/`compute`/`main`). `switchtab_x86_64`: `gcc -O1 -no-pie -fno-pic s.c`
with a `switch(argc){case 0..7}`. They are checked in (each well under 32 KB) so the
gates are hermetic and reproducible. **Pin load-bearing VMAs as test consts** (read
via `objdump`/`readelf` at build time) — addresses shift across toolchains.

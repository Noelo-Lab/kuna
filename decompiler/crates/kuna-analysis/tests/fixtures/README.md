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
| `rust_hello_x86_64` | tiny `#![no_std]` rustc PIE (x86-64), **not stripped** | source-language detection (`s1_sourcelang`): `.comment` carries `rustc version 1.90.0 …` (the faithful `ElfRustSourceLanguage` comment path) AND `.symtab` carries a Rust-mangled symbol `_ZN5nostd1m12rusty_helper17h…E` (the legacy `_ZN…17h<hex>E` heuristic) — both detection paths fire |
| `arm_thumb_le32.o` | bare ARM Thumb **`.o`** (ET_REL, EABI5, LE) — **not linked** (no PT_LOAD; see note) | ARM/Thumb decode-mode markers (`s1_loader::arm_markers`): `.symtab` carries the `$t.0` Thumb mapping symbol at `.text+0x0` AND STT_FUNC syms `thumb_add`@`0x1` / `_start`@`0x15` (LSB-set, the Thumb odd-address convention). The pass emits a `TMode=1` paint for `$t.0` (at `0x0`) and for each LSB-set FUNC normalized to even (`0x0`, `0x14`) |

Provenance: `fauxware`, `cet_pie_x86_64`, `stripped_dynamic_x86_64` copied
verbatim from `bs-artifacts/binaries/` (`fauxware`, `debug_symbol`,
`debug_symbol_mod_stripped` respectively). `cpp_mangled_x86_64` was built locally
with `g++ -O0 -no-pie -fno-pic` from a tiny `namespace foo { struct Bar { void
baz(int); }; } void foo::Bar::baz(int){...} int main(){...}` source.
`dwarf_stripped_x86_64`: `cc -g -O0 -no-pie -fno-pic t.c -o x` then
`objcopy --wildcard --strip-symbol='*' x dwarf_stripped_x86_64` (empties the symbol
table, keeps `.debug_*` — so DWARF is the sole name source; `t.c` = three funcs
`add_values`/`compute`/`main`). `switchtab_x86_64`: `gcc -O1 -no-pie -fno-pic s.c`
with a `switch(argc){case 0..7}`. `rust_hello_x86_64`: built with rustc 1.90.0
(`1159e78c4 2025-09-14`, x86_64-unknown-linux-gnu) as a freestanding `#![no_std]`
`#![no_main]` binary —
`rustc -C panic=abort -C opt-level=1 -C codegen-units=1 --target x86_64-unknown-linux-gnu -C link-args=-nostartfiles tiny.rs -o rust_hello_x86_64`
where `tiny.rs` defines a `#[panic_handler]`, a `#[no_mangle] black_box`, a
`mod m { #[inline(never)] pub fn rusty_helper(x:u64)->u64 {…} }`, and a
`#[no_mangle] _start`. The `#![no_std]` form keeps it tiny (2576 bytes, kept
**un**stripped so the Rust-mangled symbol survives) while still emitting the
`rustc version` `.comment` record and a `_ZN…17h<hex>E` symbol. They are checked
in (each well under 32 KB) so the gates are hermetic and reproducible. **Pin
load-bearing VMAs as test consts** (read via `objdump`/`readelf` at build time) —
addresses shift across toolchains.

`arm_thumb_le32.o` (904 bytes, source vendored alongside as `arm_thumb_le32.c`):
built with `clang --target=arm-linux-gnueabihf -mthumb -nostdlib -c
arm_thumb_le32.c -o arm_thumb_le32.o`. The two `__attribute__((target("thumb")))`
functions force Thumb codegen so the assembler lays the `$t` mapping symbol; the
FUNC symbols carry the LSB-set st_value Thumb convention. **It is a bare ET_REL
`.o`, NOT a linked executable** — this build host has no ARM linker (no lld;
gold/mold are x86-only builds; system `ld` rejects `armelf_linux_eabi`). The
symbol scan unit-tests against the `.o` (which `object` parses fine); the decode
**e2e** (`kuna decompile arm_thumb… main` producing valid Thumb-decoded C) is a
documented follow-up that needs a LINKED ARM exe (ET_EXEC/ET_DYN with PT_LOAD —
`ObjectLoadImage` reads only segments), built off-host.

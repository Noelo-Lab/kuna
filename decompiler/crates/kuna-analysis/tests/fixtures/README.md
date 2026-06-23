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
| `cpp_noreturn_x86_64` | non-PIE x86-64 C++, not stripped (source `cpp_noreturn_x86_64.cpp`) | the **no-return × demangle cross-pass seam** (`s1_loader::noreturn` + `s1_demangle`): `.dynsym` carries the mangled no-return imports `_ZSt9terminatev` (demangled `std::terminate`) and `__cxa_throw`, both UND (`.dynsym` address 0) — their real FunctionSymbols are installed at the PLT stubs `_ZSt9terminatev@plt`=`0x401070`, `__cxa_throw@plt`=`0x4010a0`. The no-return scan emits those **stub addresses** under the raw names, so the commit resolves the *demangled* funcsym **by address** (`find_function_across_scopes`); a name lookup of the mangled string would miss. e2e: `fail()` (`_Z4failv`=`0x401196`, demangled `fail`) tail-calls `std::terminate()` → `void fail(void)` with the `Subroutine does not return` warning and no dead fall-through; `main`=`0x4011a3` |
| `dwarf_stripped_x86_64` | non-PIE x86-64, **`.symtab`/`.dynsym` FUNC names removed but `.debug_*` kept** | DWARF recovery (`s1_dwarf`): names + typed signatures of `add_values`/`compute`/`main` come **only** from `.debug_info` (the funcsym stream has none) |
| `switchtab_x86_64` | non-PIE x86-64, dense `switch(x){0..7}` | address/jump tables (`addrtable`): an absolute 8-byte jump table in `.rodata` at vma `0x402008` (`jmp *0x402008(,%rdi,8)`) |
| `rust_hello_x86_64` | tiny `#![no_std]` rustc PIE (x86-64), **not stripped** | source-language detection (`s1_sourcelang`): `.comment` carries `rustc version 1.90.0 …` (the faithful `ElfRustSourceLanguage` comment path) AND `.symtab` carries a Rust-mangled symbol `_ZN5nostd1m12rusty_helper17h…E` (the legacy `_ZN…17h<hex>E` heuristic) — both detection paths fire |
| `arm_thumb_le32.o` | bare ARM Thumb **`.o`** (ET_REL, EABI5, LE) — **not linked** (no PT_LOAD; see note) | ARM/Thumb decode-mode markers (`s1_loader::arm_markers`): `.symtab` carries the `$t.0` Thumb mapping symbol at `.text+0x0` AND STT_FUNC syms `thumb_add`@`0x1` / `_start`@`0x15` (LSB-set, the Thumb odd-address convention). The pass emits a `TMode=1` paint for `$t.0` (at `0x0`) and for each LSB-set FUNC normalized to even (`0x0`, `0x14`) |
| `mcount_x86_64` | static, non-PIE x86-64, `gcc -pg` (`-O0`), `.debug_*` stripped | call-fixup auto-apply (`s1_callfixup`): the `-pg` prologue emits a direct `call mcount` to the weak `mcount` FUNC symbol (0x44a710); `main` is at 0x401795. The cspec (`x86-64-gcc.cspec`) registers `<callfixup name="mcount"><target name="mcount"/>` (body `temp:1 = 0;`), so tagging `main`'s `mcount` callee with that fixup's inject id dissolves the profiling call — `kuna decompile … main` then shows no `mcount();` line. Also carries `__fentry__` (0x44a770, the `fentry`-fixup target) |
| `fmt_x86_64` | non-PIE x86-64, `gcc -O0`, not stripped (source `fmt_x86_64.c`) | format-string varargs typing (`s1_formatstring` half B, `FormatStringAnalyzer`, **gated off** by default): `main`=0x401136 calls `printf("%d %s\n", argc, argv[0])` (`printf@plt`=0x401040; the `"%d %s\n"` format constant is at `.rodata` vma 0x402004). With `--option formatstring on`, the console reads the format constant at the `printf` call's format slot, parses `%d`→int / `%s`→char\*, installs a per-call-site prototype override, and re-decompiles so the call renders `printf("%d %s\n",a0,(char *)*a1)` (the `%d` arg as a plain `int`, the `%s` arg cast to `char *`) instead of the default untyped `printf("%d %s\n",(uint8)a0,*a1)` |
| `mips_gp_le32` | dynamically-linked MIPS32 **LE** ET_DYN (`-O1 -no-pie`), not stripped | MIPS `$gp` recovery via per-function `t9` tracking (`s1_loader::mips_markers`): the PIC `_init`@`0x4004cc` / `_fini`@`0x400800` compute `gp = _gp_disp + t9` (`lui gp; addiu gp; addu gp,gp,t9`); without `t9` the `$gp`-relative GOT load reads `*(int4 *)(v1 /* t9 */ + 0x10b94)` (unresolved). The pass seeds `t9 = func_entry` per function (`assumeT9EntryAddress`), so the commit's tracked-register arm + `ActionConstbase` fold gp and the load resolves to a concrete GOT slot (`dat_411060`). `main`@`0x400704`, `bump`@`0x4006f0`. `_gp` symbol = `0x419030` = `.got`(`0x411040`) + `0x7ff0` (the MIPS GP bias) — cross-checked by `recover_gp_value`. **Linked ET_DYN with PT_LOAD** (unlike the ARM `.o`): the decode e2e works in-env (this host has a MIPS toolchain) |
| `entrymain_aarch64` | stripped DYNAMIC PIE AArch64 (`int main(int,char**){return c;}`), no unwind tables, `-fvisibility=hidden` (source `entrymain.c`) | cross-arch `_start`→`main` idiom (`s1_entry` oracle 4, Increment 23): `main` is in **no** symbol table — recovered only via `_start`@`0x600`'s `adrp x0,0x10000; ldr x0,[x0,#4080]` → GOT slot `0x10ff0` whose `R_AARCH64_RELATIVE` addend is `main`@`0x714`. The `.eh_frame` FDEs (still present from crt1) do NOT cover `0x714` — oracle 4 is the sole source. e2e: `sub_714` decompiles to `unsigned int sub_714(unsigned int a0){return a0;}` |
| `entrymain_arm` | stripped DYNAMIC PIE ARM/Thumb (same source), no unwind tables, `-fvisibility=hidden` | cross-arch `_start`→`main` idiom + Thumb decode-mode paint (`s1_entry` oracle 4): `.eh_frame` is empty (just the terminator), `main` in no symbol table. `_start`@`0x3dd` (Thumb) loads `r0` GOT-relatively (`.got`@`0x10fd0` + `0x28` = slot `0x10ff8`, `R_ARM_RELATIVE` in-place value `0x4d9` = `main`@`0x4d8` with the Thumb LSB). The discovery pass masks the LSB for the entry AND emits a `TMode=1` `ContextPaint` at `0x4d8` (no `$t` survives stripping), so the body decodes as Thumb. e2e: `sub_4d8` → `unsigned int sub_4d8(unsigned int a0){return a0;}` (a `void {return;}` stub means the Thumb paint regressed) |
| `entrymain_riscv64` | stripped DYNAMIC PIE RISC-V RV64GC (same source), no unwind tables, `-fvisibility=hidden` | cross-arch `_start`→`main` idiom (`s1_entry` oracle 4): `main` in no symbol table (hidden visibility — a plain build leaves `main` a `.dynsym` GLOBAL FUNC that strip cannot remove). `_start`@`0x550` loads `a0` via `auipc a0,0x2; ld a0,-1318(a0)` → GOT slot `0x2030` whose `R_RISCV_RELATIVE` addend is `main`@`0x608`. e2e: `sub_608` → `int8 sub_608(int4 a0){return (int8)a0;}` |

Provenance: `fauxware`, `cet_pie_x86_64`, `stripped_dynamic_x86_64` copied
verbatim from `bs-artifacts/binaries/` (`fauxware`, `debug_symbol`,
`debug_symbol_mod_stripped` respectively). `cpp_mangled_x86_64` was built locally
with `g++ -O0 -no-pie -fno-pic` from a tiny `namespace foo { struct Bar { void
baz(int); }; } void foo::Bar::baz(int){...} int main(){...}` source.
`cpp_noreturn_x86_64`: `g++ -O0 -no-pie -fno-pic -o cpp_noreturn_x86_64
cpp_noreturn_x86_64.cpp` (source vendored alongside) — a `fail()` that tail-calls
`std::terminate()` plus a `throw` (→ `__cxa_throw`); both are mangled no-return
`.dynsym` imports the demangle pass renames, so they verify the address-resolved
no-return commit. `dwarf_stripped_x86_64`: `cc -g -O0 -no-pie -fno-pic t.c -o x` then
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
`rustc version` `.comment` record and a `_ZN…17h<hex>E` symbol.

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

`mcount_x86_64`: `gcc -pg -static -O0 -o mcount_x86_64 t.c` (t.c = `int
main(){return 0;}`), then `strip --strip-debug` (drops `.debug_*` but keeps
`.symtab`, so the `mcount`/`__fentry__`/`main` FUNC symbols survive). It is
**static** on purpose: a dynamic `-pg` build resolves `mcount` to an *indirect*
GOT call (`call *0x…(%rip)`), which has no named-`mcount` FunctionSymbol at the
call target, so the name-matched fixup cannot bind — only the static build emits a
direct `call mcount` to a real `mcount` FUNC symbol. Static glibc makes this
fixture larger (~896 KB) than the others; that size is the unavoidable cost of a
self-contained direct-`call mcount` target.

**No Go fixture is vendored** (the Golang no-return list, Increment 15). Go ELF
binaries are unavoidably large — `go build` emits **~1.1 MB** un-stripped (the
whole runtime is statically embedded) and **~750 KB** stripped — and the
coverage tradeoff is forced: a *stripped* Go binary keeps `.go.buildinfo` (so
`detect_compiler` ⇒ `Go`) but drops `.symtab` entirely (so there is no
`runtime.gopanic` FUNC symbol for the no-return matcher), while only the
*un-stripped* 1.1 MB build carries both. Rather than vendor a 1.1 MB blob, the Go
e2e (`s1_loader::noreturn::tests::real_go_binary_detected_and_flags_runtime_gopanic`)
**builds a tiny real Go program at test runtime** (`go build` into an isolated
temp dir with a private GOCACHE/GOPATH), **guarded on `go` being on PATH** —
skipping cleanly otherwise (the same off-host-toolchain posture as the ARM-link
follow-up). It asserts both halves on a genuine Go binary: `detect_compiler == Go`
AND `runtime.gopanic`/`runtime.throw`/`runtime.goexit.abi0` flagged no-return
under the Go arm but not the C arm. The list-parse/matching logic itself is pinned
hermetically (no fixture, always runs) by `golang_list_gated_on_go_detection` and
the `s1_sourcelang` list tests.

`fmt_x86_64` (~16 KB, source vendored alongside as `fmt_x86_64.c`): built with
`gcc -no-pie -fno-stack-protector -O0 -o fmt_x86_64 fmt_x86_64.c` where
`fmt_x86_64.c` = `int main(int argc,char**argv){printf("%d %s\n", argc,
argv[0]); return 0;}` (kept **un**stripped so `main`/`printf` resolve by name).
The `-no-pie` keeps the format-string constant a fixed absolute address
(`.rodata` vma 0x402004) so the per-call-site format-constant read is
deterministic. Drives the `FormatStringAnalyzer` half-B console gate
(`kuna-console/tests/verify_s1_formatstring.rs`).
`mips_gp_le32` (7684 bytes, source vendored alongside as `mips_gp_le32.c`): built
with `mipsel-linux-gnu-gcc -O1 -no-pie -o mips_gp_le32 mips_gp_le32.c` (Ubuntu
mipsel-linux-gnu-gcc 10.3.0). The dynamic (`-no-pie` but PIC libc) link keeps it
small (7684 bytes) while still emitting the PIC `$gp` prologue (`lui gp; addiu gp;
addu gp,gp,t9` in `_init`/`_fini`) and a `lw t9,-N(gp)` GOT call in `main` — the
`$gp`-relative loads `t9`-tracking must resolve. A **static** build (`-static`)
also works but is ~672 KB (static glibc), so the dynamic form is vendored. `t9.c`
uses a global `counter` + a `printf` call so the prologue sets `$gp`. The `_gp`
LOCAL symbol survives (not stripped) so `recover_gp_value` can read it.

`entrymain_aarch64` / `entrymain_arm` / `entrymain_riscv64` (each <7 KB, shared
source `entrymain.c` = `int main(int c,char**v){return c;}`): the cross-arch
`_start`→`main` idiom fixtures (Increment 23). Built in the `kuna-dev` container
to recover `main` ONLY via the libc-start idiom — DYNAMIC (real crt1 `_start` →
`__libc_start_main(main,…)`), unwind tables dropped (`-fno-asynchronous-unwind-tables
-fno-unwind-tables`, to keep `main` out of `.eh_frame`), `-fvisibility=hidden`
(so `main` is not exported in `.dynsym`), then stripped:

```
docker run --rm -v "$PWD":/w -w /w kuna-dev bash -lc '\
  <triple>-gcc -O0 -fno-asynchronous-unwind-tables -fno-unwind-tables \
    -fvisibility=hidden entrymain.c -o <out> && <triple>-strip <out>'
```

with triples `aarch64-linux-gnu`, `arm-linux-gnueabihf`, `riscv64-linux-gnu`. The
RISC-V cross-libc is not in the base image — install it first (the same package
the MIPS/RISC-V ports used): `sudo apt-get update && sudo apt-get install -y
libc6-dev-riscv64-cross`. Two non-obvious flags are load-bearing: **`-fvisibility=hidden`**
(plain builds leave `main` a `.dynsym` GLOBAL FUNC — on AArch64/ARM strip removes
it, but on RISC-V `.dynsym` entries are load-bearing and survive strip, so without
hidden visibility `main` would already be a funcsym and oracle 4 could not be shown
to contribute it); **`-fno-*-unwind-tables`** isolates oracle 4 from the `.eh_frame`
FDE oracle (AArch64/RISC-V still carry crt1 FDEs, but none cover `main`; ARM's
`.eh_frame` is fully empty). VMAs (`_start`/`main`/GOT slot) are pinned as test
consts in `s1_entry`'s tests + `kuna-console/tests/verify_crossarch_entry_main.rs`
(read via container `objdump`/`readelf`/`nm` at build time). Unlike the ARM `.o`,
these are LINKED PIE executables (ET_DYN + PT_LOAD), so the decode e2e runs.

All other fixtures are checked in well under 32 KB so the gates are hermetic and
reproducible. **Pin load-bearing VMAs as test consts** (read via
`objdump`/`readelf` at build time) — addresses shift across toolchains.

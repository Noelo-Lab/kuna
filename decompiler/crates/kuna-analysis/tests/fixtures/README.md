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
| `eh_lsda_x86_64` | non-PIE x86-64 C++ try/catch, **`.symtab` stripped** (source `eh_lsda_x86_64.cpp`) | `.eh_frame` LSDA landing-pad discovery (`s1_entry::EhFrameLsdaPass`, gated `--option eh_frame_full on`, the GccExceptionAnalyzer full `.gcc_except_table` markup): the `zPLR` CIE's `L` augmentation points each FDE at its LSDA in `.gcc_except_table` (`may_throw`@`0x40218c`, `guarded`@`0x402198`); the call-site tables decode to landing pads `0x4012bf` (may_throw cleanup), `0x4012e2` (guarded catch dispatch), `0x401352`/`0x401366` (guarded cleanup) — all `endbr64`, all **mid-function** (reached only by the unwinder, so NOT FDE pcBegins; the FDE-start oracle misses them). e2e (`verify_eh_frame_full`): with `--option eh_frame_full on`, `0x4012e2` registers as `sub_4012e2` and decompiles by name; default-off it is absent (discovery byte-identical to FDE-pcBegin only). FDE pcBegins (function starts): `may_throw`=`0x401256`, `guarded`=`0x4012d6`, `main`=`0x40137a` |
| `dwarf_stripped_x86_64` | non-PIE x86-64, **`.symtab`/`.dynsym` FUNC names removed but `.debug_*` kept** | DWARF recovery (`s1_dwarf`): names + typed signatures of `add_values`/`compute`/`main` come **only** from `.debug_info` (the funcsym stream has none) |
| `switchtab_x86_64` | non-PIE x86-64, dense `switch(x){0..7}` | address/jump tables (`addrtable`): an absolute 8-byte jump table in `.rodata` at vma `0x402008` (`jmp *0x402008(,%rdi,8)`) |
| `rust_hello_x86_64` | tiny `#![no_std]` rustc PIE (x86-64), **not stripped** | source-language detection (`s1_sourcelang`): `.comment` carries `rustc version 1.90.0 …` (the faithful `ElfRustSourceLanguage` comment path) AND `.symtab` carries a Rust-mangled symbol `_ZN5nostd1m12rusty_helper17h…E` (the legacy `_ZN…17h<hex>E` heuristic) — both detection paths fire |
| `arm_thumb_le32.o` | bare ARM Thumb **`.o`** (ET_REL, EABI5, LE) — **not linked** (no PT_LOAD; see note) | ARM/Thumb decode-mode markers (`s1_loader::arm_markers`): `.symtab` carries the `$t.0` Thumb mapping symbol at `.text+0x0` AND STT_FUNC syms `thumb_add`@`0x1` / `_start`@`0x15` (LSB-set, the Thumb odd-address convention). The pass emits a `TMode=1` paint for `$t.0` (at `0x0`) and for each LSB-set FUNC normalized to even (`0x0`, `0x14`) |
| `arm_thumb_linked_le32` | **LINKED** ARM Thumb ET_EXEC (LE, `-static -nostdlib`) — one PT_LOAD R E at `0x10000` (so `ObjectLoadImage` loads it, unlike the bare `.o`) | ARM/Thumb decode **e2e** (`s1_loader::arm_markers` + the commit seam, `kuna-console/tests/verify_arm_thumb_decode.rs`): the `$t`@`0x100b8` mapping symbol + the LSB-set FUNCs `compute`@`0x100b9` (→ even `0x100b8`) / `_start`@`0x100d7` (→ even `0x100d6`) drive a `TMode=1` paint, so `load function compute` Thumb-decodes `compute(x)` to `return a0 * 3 + 7;` (an ARM-mode misdecode of the same bytes is garbage), and the Thumb-FUNC re-home makes `_start`'s `bl` to compute's even entry render `compute(5)`. **The deferred Increment-8/17 decode e2e, now built in-container** |
| `mcount_x86_64` | static, non-PIE x86-64, `gcc -pg` (`-O0`), `.debug_*` stripped | call-fixup auto-apply (`s1_callfixup`): the `-pg` prologue emits a direct `call mcount` to the weak `mcount` FUNC symbol (0x44a710); `main` is at 0x401795. The cspec (`x86-64-gcc.cspec`) registers `<callfixup name="mcount"><target name="mcount"/>` (body `temp:1 = 0;`), so tagging `main`'s `mcount` callee with that fixup's inject id dissolves the profiling call — `kuna decompile … main` then shows no `mcount();` line. Also carries `__fentry__` (0x44a770, the `fentry`-fixup target) |
| `fmt_x86_64` | non-PIE x86-64, `gcc -O0`, not stripped (source `fmt_x86_64.c`) | format-string varargs typing (`s1_formatstring` half B, `FormatStringAnalyzer`, **gated off** by default): `main`=0x401136 calls `printf("%d %s\n", argc, argv[0])` (`printf@plt`=0x401040; the `"%d %s\n"` format constant is at `.rodata` vma 0x402004). With `--option formatstring on`, the console reads the format constant at the `printf` call's format slot, parses `%d`→int / `%s`→char\*, installs a per-call-site prototype override, and re-decompiles so the call renders `printf("%d %s\n",a0,(char *)*a1)` (the `%d` arg as a plain `int`, the `%s` arg cast to `char *`) instead of the default untyped `printf("%d %s\n",(uint8)a0,*a1)` |
| `fmt_aarch64` | PIE AArch64, `gcc -O0 -fno-stack-protector`, not stripped (source `fmt_aarch64.c`, same C as `fmt_x86_64`) | format-string varargs typing **cross-arch** (`s1_formatstring` half B, **gated off**): `main`=0x754 calls `printf("%d %s\n", argc, argv[0])` (`printf@plt`=0x630); the format address is materialized by `adrp x0,0; add x0,x0,#0x7a8` so the format constant is at `.rodata` vma 0x7a8. With `--option formatstring on` the call renders `printf("%d %s\n",a0,(char *)*a1)` (default-off leaves the `%s` arg untyped). Drives `kuna-console/tests/verify_formatstring_crossarch.rs` |
| `fmt_arm` | PIE ARM (32-bit, Thumb), `gcc -O0 -fno-stack-protector`, not stripped (source `fmt_arm.c`, same C as `fmt_x86_64`) | format-string varargs typing **cross-arch — the read-only literal-pool case** (`s1_formatstring` half B, **gated off**): `main`=0x504 (Thumb, `main`=0x505 in `.symtab`) calls `printf("%d %s\n", argc, argv[0])` (`printf@plt`=0x3e4). The format address is loaded **PC-relatively from the read-only literal pool** (`ldr r3,[pc,#20]` reads the `.word 0xb0` at 0x52c; `add r3,pc` → pc(0x51c)+0xb0 = format constant at `.rodata` vma 0x5cc), so the format-arg varnode is a memory LOAD that constant-folds only under `readonlypropagate`. With `--option formatstring on` the loop enables read-only propagation for the decompile so the call renders `printf("%d %s\n",a0,(char *)*a1)` (default-off leaves the format pointer the unresolved `(char *)(dat_52c + 0x51c)`). Drives `kuna-console/tests/verify_formatstring_crossarch.rs` |
| `fmt_riscv64` | PIE RISC-V64 (RVC, lp64d), `gcc -O0 -fno-stack-protector`, not stripped (source `fmt_riscv64.c`, same C as `fmt_x86_64`) | format-string varargs typing **cross-arch** (`s1_formatstring` half B, **gated off**): `main`=0x668 calls `printf("%d %s\n", argc, argv[0])` (`printf@plt`=0x5a0); the format address is materialized by `auipc a0,0x0; addi a0,a0,32` (pc 0x688 + 32) so the format constant is at `.rodata` vma 0x6a8. With `--option formatstring on` the call renders `printf("%d %s\n",a0,(char *)*a1)` (default-off leaves the `%s` arg untyped; the default `%d` cast is `(int8)`). Drives `kuna-console/tests/verify_formatstring_crossarch.rs` |
| `plt_riscv64` | dynamically-linked RISC-V64 PIE (RVC, lp64d), not stripped (source `plt_riscv64.c`) | RISC-V PLT/GOT import naming end-to-end (`elf_plt::decode_riscv`): `main`=`0x6b8` calls `puts@plt`=`0x5e0` (`auipc t3,0x2; ld t3,-1472(t3); jalr t1,t3; nop` → GOT slot `0x2020`) and `printf@plt`=`0x5f0` (→ GOT `0x2028`); both are `R_RISCV_JUMP_SLOT` relocs in `.rela.plt` naming `puts`/`printf`. **Linked dynamic exe with PT_LOAD** (the RISC-V analog of the x86 `fauxware` PLT e2e and the MIPS linked fixture) — drives `kuna-console/tests/verify_riscv64_plt.rs`, which decompiles `main` to `puts("hello"); printf("%d\n",(int8)a0);` (not `sub_5e0`/`sub_5f0`) |
| `mips_gp_le32` | dynamically-linked MIPS32 **LE** ET_DYN (`-O1 -no-pie`), not stripped | MIPS `$gp` recovery via per-function `t9` tracking (`s1_loader::mips_markers`): the PIC `_init`@`0x4004cc` / `_fini`@`0x400800` compute `gp = _gp_disp + t9` (`lui gp; addiu gp; addu gp,gp,t9`); without `t9` the `$gp`-relative GOT load reads `*(int4 *)(v1 /* t9 */ + 0x10b94)` (unresolved). The pass seeds `t9 = func_entry` per function (`assumeT9EntryAddress`), so the commit's tracked-register arm + `ActionConstbase` fold gp and the load resolves to a concrete GOT slot (`dat_411060`). `main`@`0x400704`, `bump`@`0x4006f0`. `_gp` symbol = `0x419030` = `.got`(`0x411040`) + `0x7ff0` (the MIPS GP bias) — cross-checked by `recover_gp_value`. **Linked ET_DYN with PT_LOAD** (unlike the ARM `.o`): the decode e2e works in-env (this host has a MIPS toolchain) |
| `plt_ppc64le` | dynamically-linked PowerPC64 **ELFv2** (little-endian) PIE, not stripped (source `plt_ppc64le.c`) | PowerPC64 PLT/import-name resolution end-to-end (`elf_plt::decode_ppc_text` / `decode_ppc64_stubs`): ELFv2 has **no `.plt` code section** — `.plt` is a NOBITS data table (the runtime GOT) and the linker synthesizes the call stubs inline in `.text`. `main`=`0x8bc` `bl`s the `puts@plt` stub `0x680` and the `printf@plt` stub `0x660`; each stub is `std r2,24(r1); addis r12,r2,off@ha; ld r12,off@l(r12); mtctr r12; bctr`, loading a `.plt` slot `TOC_base(.got+0x8000=0x27f00) + (off@ha<<16) + off@l` = `0x1fef0` (puts) / `0x1fef8` (printf), both `R_PPC64_JMP_SLOT` relocs in `.rela.plt`. The console e2e (`kuna-console/tests/verify_ppc64_plt.rs`) decompiles `main` to `puts(...); printf(...)` not `sub_680`/`sub_660` — the `.text`-synthesized PLT stubs (previously a documented seam) **are** statically resolvable. **Linked ET_DYN/PIE with PT_LOAD** |
| `entrymain_aarch64` | stripped DYNAMIC PIE AArch64 (`int main(int,char**){return c;}`), no unwind tables, `-fvisibility=hidden` (source `entrymain.c`) | cross-arch `_start`→`main` idiom (`s1_entry` oracle 4, Increment 23): `main` is in **no** symbol table — recovered only via `_start`@`0x600`'s `adrp x0,0x10000; ldr x0,[x0,#4080]` → GOT slot `0x10ff0` whose `R_AARCH64_RELATIVE` addend is `main`@`0x714`. The `.eh_frame` FDEs (still present from crt1) do NOT cover `0x714` — oracle 4 is the sole source. e2e: `sub_714` decompiles to `unsigned int sub_714(unsigned int a0){return a0;}` |
| `entrymain_arm` | stripped DYNAMIC PIE ARM/Thumb (same source), no unwind tables, `-fvisibility=hidden` | cross-arch `_start`→`main` idiom + Thumb decode-mode paint (`s1_entry` oracle 4): `.eh_frame` is empty (just the terminator), `main` in no symbol table. `_start`@`0x3dd` (Thumb) loads `r0` GOT-relatively (`.got`@`0x10fd0` + `0x28` = slot `0x10ff8`, `R_ARM_RELATIVE` in-place value `0x4d9` = `main`@`0x4d8` with the Thumb LSB). The discovery pass masks the LSB for the entry AND emits a `TMode=1` `ContextPaint` at `0x4d8` (no `$t` survives stripping), so the body decodes as Thumb. e2e: `sub_4d8` → `unsigned int sub_4d8(unsigned int a0){return a0;}` (a `void {return;}` stub means the Thumb paint regressed) |
| `entrymain_riscv64` | stripped DYNAMIC PIE RISC-V RV64GC (same source), no unwind tables, `-fvisibility=hidden` | cross-arch `_start`→`main` idiom (`s1_entry` oracle 4): `main` in no symbol table (hidden visibility — a plain build leaves `main` a `.dynsym` GLOBAL FUNC that strip cannot remove). `_start`@`0x550` loads `a0` via `auipc a0,0x2; ld a0,-1318(a0)` → GOT slot `0x2030` whose `R_RISCV_RELATIVE` addend is `main`@`0x608`. e2e: `sub_608` → `int8 sub_608(int4 a0){return (int8)a0;}` |
| `plt_aarch64` | linked, dynamic AArch64 ET_EXEC (`-no-pie`), not stripped (source `plt_aarch64.c`) | AArch64 PLT/import-name resolution end-to-end (`s1_loader::elf_plt::decode_aarch64`): the standard GNU `ld` 16-byte veneer (`adrp x16, GOT_page; ldr x17,[x16,#lo12]; add x16,x16,#lo12; br x17`). `main`@`0x400604` calls `puts("hello")` (`puts@plt`@`0x4004d0`, GOT slot `0x411018`) and `printf("%d\n", argc)` (`printf@plt`@`0x4004e0`, GOT slot `0x411020`); both `R_AARCH64_JUMP_SLOT` in `.rela.plt`. The console e2e (`kuna-console/tests/verify_aarch64_plt.rs`) asserts the call sites render `puts(`/`printf(` not `sub_4004d0`/`sub_4004e0` — the first **linked** AArch64 PLT proof (the decoder was previously synthetic-byte-unit-only). **Linked ET_EXEC with PT_LOAD** (unlike the ARM `.o`): the decode e2e works in-env (this container has the AArch64 toolchain + linker) |
| `plt_sparc64` | linked, dynamic SPARC v9 / ELF64 **big-endian** ET_EXEC, not stripped (source `plt_sparc64.c`) | SPARC PLT/import-name resolution end-to-end (`s1_loader::elf_plt::decode_sparc`): the standard 32-byte SPARC veneer (`sethi %hi(...),%g1; b,a %xcc,<resolver>; nop*6`), preceded by a 4-slot (`0x80`-byte) reserved PLT0 header. SPARC's `R_SPARC_JMP_SLOT` `r_offset` **is** the PLT entry address (the linker rewrites the in-place stub at resolution time), so the decoder strides the `.plt` in 32-byte steps and records any `sethi %g1`-headed entry whose address is a known relocation — stub == name-map key. `main`@`0x100750` calls `puts("hello")` (`puts@plt`@`0x2021c0`) and `printf("%d\n", argc)` (`printf@plt`@`0x2021a0`); both `R_SPARC_JMP_SLOT` in `.rela.plt` naming `puts`/`printf`. The console e2e (`kuna-console/tests/verify_sparc_plt.rs`) asserts the call sites render `puts(`/`printf(` not `sub_2021c0`/`sub_2021a0` — the first **linked** SPARC PLT proof. **Linked ET_EXEC with PT_LOAD**: the decode e2e works in-env (this container has the SPARC toolchain + linker) |
| `plt_mips32` | linked, dynamic MIPS32 **big-endian** ET_EXEC (`-O0`), not stripped (source `plt_mips32.c`) | MIPS o32 import-name resolution end-to-end (`s1_loader::elf_plt::resolve_mips_imports`, Increment 27): **no `.plt` / no `R_MIPS_JUMP_SLOT`** — the o32 ABI calls libc imports indirectly through a `$gp`-relative GOT slot (`lw $t9, off($gp); jalr $t9`). The stub→name correspondence is the dynamic-symbol GOT layout (`DT_MIPS_LOCAL_GOTNO`=6, `DT_MIPS_GOTSYM`=5, `DT_PLTGOT`=`0x411020`): `got_index(i)=6+(i-5)`. `main`@`0x400700` calls `puts` (dynidx 7 → GOT slot `0x411040` → stub `0x400800`) and `printf` (dynidx 8 → GOT slot `0x411044` → stub `0x4007f0`). `resolve_mips_imports` names each `.MIPS.stubs` stub (= the GOT slot's static contents = the dynsym `st_value`) and marks the GOT external slots constant; `bootstrap_from_object` turns on `readonlypropagate` for MIPS so the GOT load folds and the call resolves. The console e2e (`kuna-console/tests/verify_mips_plt.rs`) asserts the call sites render `puts(`/`printf(` not `(*(code *)(dat_411040 & ...))(...)`. **Linked ET_EXEC with PT_LOAD**: the decode e2e works in-env (the container has the MIPS toolchain) |

Provenance: `fauxware`, `cet_pie_x86_64`, `stripped_dynamic_x86_64` copied
verbatim from `bs-artifacts/binaries/` (`fauxware`, `debug_symbol`,
`debug_symbol_mod_stripped` respectively). `cpp_mangled_x86_64` was built locally
with `g++ -O0 -no-pie -fno-pic` from a tiny `namespace foo { struct Bar { void
baz(int); }; } void foo::Bar::baz(int){...} int main(){...}` source.
`cpp_noreturn_x86_64`: `g++ -O0 -no-pie -fno-pic -o cpp_noreturn_x86_64
cpp_noreturn_x86_64.cpp` (source vendored alongside) — a `fail()` that tail-calls
`std::terminate()` plus a `throw` (→ `__cxa_throw`); both are mangled no-return
`.dynsym` imports the demangle pass renames, so they verify the address-resolved
no-return commit. `eh_lsda_x86_64` (14744 bytes, source vendored alongside as
`eh_lsda_x86_64.cpp`): `g++ -O1 -no-pie -fno-pic -fexceptions -o eh_lsda_x86_64
eh_lsda_x86_64.cpp` then `strip eh_lsda_x86_64` (drops `.symtab`; keeps
`.eh_frame` + `.gcc_except_table`). The source is a `guarded()` with a
`try { may_throw(x); } catch (const std::runtime_error&) {...} catch (int) {...}`
over an out-of-line throwing helper — `-fexceptions` (default for C++) emits the
`zPLR`-augmented FDEs whose `L` char points each FDE at an LSDA in
`.gcc_except_table`, and the `catch` blocks become the landing pads. `-no-pie`
keeps the landing-pad VMAs fixed/deterministic for the pinned test consts; `-O1`
keeps it small (14 KB) while still emitting all four landing pads. The landing
pads (`0x4012bf`/`0x4012e2`/`0x401352`/`0x401366`) were decoded by hand from the
`.gcc_except_table` call-site tables and cross-checked against `objdump -d`
(every one is an `endbr64`) and `readelf --debug-dump=frames` (the FDE LSDA
augmentation-data pointers `8c 21 40 00`=`0x40218c`, `98 21 40 00`=`0x402198`).
**Pin the landing-pad VMAs as test consts.** `dwarf_stripped_x86_64`: `cc -g -O0 -no-pie -fno-pic t.c -o x` then
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
**e2e** uses the LINKED `arm_thumb_linked_le32` (below).

`arm_thumb_linked_le32` (1080 bytes, source vendored alongside as
`arm_thumb_linked_le32.c`): the LINKED counterpart to the bare `.o`, built **in
the `kuna-dev` container** (arm-linux-gnueabihf-gcc 11.4.0) with
`arm-linux-gnueabihf-gcc -mthumb -static -nostdlib -e _start arm_thumb_linked_le32.c -o arm_thumb_linked_le32`.
`-mthumb` forces Thumb codegen (the assembler lays the `$t` mapping symbol; the
linker records the STT_FUNC symbols at `entry|1`); `-static -nostdlib -e _start`
keeps it tiny and self-contained. It is a real **ET_EXEC with a PT_LOAD R E
segment** (`readelf -h` Type EXEC / Machine ARM; `readelf -l` one LOAD R E at
`0x10000`), so `ObjectLoadImage` (segments-only) loads it — the property the bare
`.o` lacked. `compute` is `x*3 + 7` (non-trivial Thumb arithmetic) so a correct
Thumb decode is visibly distinct from an ARM-mode misdecode. Drives the deferred
Increment-8/17 decode **e2e** (`kuna-console/tests/verify_arm_thumb_decode.rs`).

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

`fmt_aarch64` (8880 bytes), `fmt_arm` (7816 bytes), `fmt_riscv64` (8472 bytes) —
the **cross-arch** counterparts of `fmt_x86_64`, each built in the `kuna-dev`
container from the same one-line source (`fmt_<arch>.c` =
`int main(int argc,char**argv){printf("%d %s\n", argc, argv[0]); return 0;}`),
kept **un**stripped so `main`/`printf` resolve by name. They drive the cross-arch
`FormatStringAnalyzer` half-B gate (`kuna-console/tests/verify_formatstring_crossarch.rs`).
Build commands (single root container invocation, `apt-get update` so the RISC-V
dev package — `crt1.o` + headers, not in the base image — is installable):
`docker run --rm --user root -v "$PWD":/w -w /w kuna-dev bash -lc 'apt-get update
>/dev/null && apt-get install -y --no-install-recommends libc6-dev-riscv64-cross
>/dev/null; F=decompiler/crates/kuna-analysis/tests/fixtures;
aarch64-linux-gnu-gcc -O0 -fno-stack-protector $F/fmt_aarch64.c -o $F/fmt_aarch64;
arm-linux-gnueabihf-gcc -O0 -fno-stack-protector $F/fmt_arm.c -o $F/fmt_arm;
riscv64-linux-gnu-gcc -O0 -fno-stack-protector $F/fmt_riscv64.c -o $F/fmt_riscv64'`
(Ubuntu gcc 11.4.0 for all three). All three link **dynamic PIE** (the default;
`-no-pie` is unnecessary here since the format-constant read goes through the
recovered IR, not a fixed absolute VMA). On AArch64/RISC-V the format address is
materialized directly (`adrp+add` / `auipc+addi`); on **ARM** it is loaded from a
read-only PC-relative literal pool, so the format-string loop enables
`readonlypropagate` for the decompile (see `verify_formatstring_crossarch.rs`).

`mips_gp_le32` (7684 bytes, source vendored alongside as `mips_gp_le32.c`): built
with `mipsel-linux-gnu-gcc -O1 -no-pie -o mips_gp_le32 mips_gp_le32.c` (Ubuntu
mipsel-linux-gnu-gcc 10.3.0). The dynamic (`-no-pie` but PIC libc) link keeps it
small (7684 bytes) while still emitting the PIC `$gp` prologue (`lui gp; addiu gp;
addu gp,gp,t9` in `_init`/`_fini`) and a `lw t9,-N(gp)` GOT call in `main` — the
`$gp`-relative loads `t9`-tracking must resolve. A **static** build (`-static`)
also works but is ~672 KB (static glibc), so the dynamic form is vendored. `t9.c`
uses a global `counter` + a `printf` call so the prologue sets `$gp`. The `_gp`
LOCAL symbol survives (not stripped) so `recover_gp_value` can read it.
`mips16_le32` (1584 bytes, source vendored alongside as `mips16_le32.c`): built
in the dev container with
`mips-linux-gnu-gcc -mips16 -O1 -no-pie -nostdlib -ffreestanding mips16_le32.c -o mips16_le32`
(Ubuntu mips-linux-gnu-gcc 10.3.0; big-endian — the `_le32` name follows the
sibling `mips_gp_le32`'s convention, endianness is in the ELF header).
**Freestanding** because the container ships the MIPS *runtime* libc but no
`libc6-dev` (no `crt1.o`/headers), so a normal libc link fails — and a decode
fixture needs no runtime, only a decodable MIPS16 body. `m16_square` is
`__attribute__((mips16)) int m16_square(int n){return n*n+3;}` (8 bytes:
`mult a0,a0; mflo v0; jr ra; addiu v0,3`); on this toolchain its STT_FUNC is
recorded at the EVEN entry (`0x400130`) with `st_other & 0xf0 == STO_MIPS_MIPS16`
(the binutils MIPS16 marker) — **not** an LSB-set odd address — exactly the
`MIPS_ElfExtension.applyIsaMode` st_other branch. Drives the MIPS16 `ISA_MODE`
painting unit tests (`s1_loader::mips_markers`) + the console e2e gate
(`kuna-console/tests/verify_mips16_isa.rs`), where it decodes to
`return a0 * a0 + 3;` (MIPS16) vs an empty `void` body (MIPS32 misdecode, the
BEFORE state).
`plt_aarch64` (9056 bytes, source vendored alongside as `plt_aarch64.c`): built
with `aarch64-linux-gnu-gcc -O0 -no-pie plt_aarch64.c -o plt_aarch64` (Ubuntu
aarch64-linux-gnu-gcc 11.4.0, in the `kuna-dev` container —
`docker run --rm -v "$PWD":/w -w /w kuna-dev bash -lc 'aarch64-linux-gnu-gcc -O0
-no-pie decompiler/crates/kuna-analysis/tests/fixtures/plt_aarch64.c -o
decompiler/crates/kuna-analysis/tests/fixtures/plt_aarch64'`). The `-no-pie` keeps
it ET_EXEC with fixed PLT/GOT VMAs so the pinned stub/GOT consts in
`verify_aarch64_plt.rs` are deterministic; `main`/`puts`/`printf` are kept
**un**stripped so the local `main` resolves and the `.dynsym` import names back the
PLT veneers. Drives the AArch64 PLT import-name console gate
(`kuna-console/tests/verify_aarch64_plt.rs`).

`plt_riscv64` (8520 bytes, source vendored alongside as `plt_riscv64.c`): built
with `riscv64-linux-gnu-gcc -O0 plt_riscv64.c -o plt_riscv64`
(`riscv64-linux-gnu-gcc 11.4.0`). `plt_riscv64.c` =
`int main(int argc,char**argv){ puts("hello"); printf("%d\n", argc); return 0; }`
— a normal dynamic RISC-V64 PIE (RVC, lp64d ABI), kept **un**stripped so `main`
resolves by name. It has a real `.plt` + `.rela.plt` (`DT_PLTGOT`=`0x2008`); the
`puts`/`printf` `R_RISCV_JUMP_SLOT` relocations name the GOT slots `0x2020`/`0x2028`,
and the 16-byte `auipc t3; ld t3,lo(t3); jalr t1,t3; nop` PLT veneers
(`puts@plt`=`0x5e0`, `printf@plt`=`0x5f0`) are exactly the form `elf_plt::decode_riscv`
recognizes. Drives the RISC-V PLT import-name console e2e
(`kuna-console/tests/verify_riscv64_plt.rs`). The build host's `kuna-dev` image ships
`libc6-riscv64-cross` (the shared libs) but not the dev package, so the cross-link needs
`libc6-dev-riscv64-cross` (headers + `crt1.o`) installed in the build container —
the exact build command (single root container invocation) is:
`docker run --rm --user root -v "$PWD":/w -w /w kuna-dev bash -lc 'apt-get update >/dev/null
&& apt-get install -y --no-install-recommends libc6-dev-riscv64-cross >/dev/null
&& riscv64-linux-gnu-gcc -O0 decompiler/crates/kuna-analysis/tests/fixtures/plt_riscv64.c
-o decompiler/crates/kuna-analysis/tests/fixtures/plt_riscv64'`.

`plt_sparc64` (12936 bytes, source vendored alongside as `plt_sparc64.c`): built
with `sparc64-linux-gnu-gcc -O0 plt_sparc64.c -o plt_sparc64`. `plt_sparc64.c` =
`int main(int argc,char**argv){ puts("hello"); printf("%d\n", argc); return 0; }`
— a normal dynamic SPARC v9 / ELF64 **big-endian** EXEC, kept **un**stripped so
`main` resolves by name. It has a real `.plt` (`0x202100`, 32-byte entries) +
`.rela.plt`; the `puts`/`printf` `R_SPARC_JMP_SLOT` relocations have `r_offset`
equal to their PLT entry addresses (`0x2021c0`/`0x2021a0` — on SPARC the linker
rewrites the in-place stub at resolution time, so the relocation offset IS the call
target, not a separate GOT word), and the 32-byte `sethi %hi(...),%g1; b,a %xcc,
<resolver>; nop*6` veneers are exactly the form `elf_plt::decode_sparc` recognizes.
Drives the SPARC PLT import-name console e2e (`kuna-console/tests/verify_sparc_plt.rs`).
Like the RISC-V fixture, the `kuna-dev` image ships `sparc64-linux-gnu-gcc` but not
the SPARC libc dev package, so the cross-link needs `libc6-dev-sparc64-cross`
(headers + `crt1.o`) installed in the build container — the exact build command
(single root container invocation) is:
`docker run --rm --user root -v "$PWD":/w -w /w kuna-dev bash -lc 'apt-get update >/dev/null
&& apt-get install -y --no-install-recommends libc6-dev-sparc64-cross >/dev/null
&& sparc64-linux-gnu-gcc -O0 decompiler/crates/kuna-analysis/tests/fixtures/plt_sparc64.c
-o decompiler/crates/kuna-analysis/tests/fixtures/plt_sparc64'`.

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

`plt_ppc64le` (~21 KB, source vendored alongside as `plt_ppc64le.c`): built with
`powerpc64le-linux-gnu-gcc -O0 plt_ppc64le.c -o plt_ppc64le`
(Ubuntu powerpc64le-linux-gnu-gcc 11.4.0, in the `kuna-dev` container).
`plt_ppc64le.c` = `int main(int argc,char**argv){ puts("hello"); printf("%d\n",
argc); return 0; }` — a normal dynamic PPC64le **ELFv2** PIE, kept **un**stripped
so `main` resolves by name. ELFv2 has no `.plt` code section, so the linker
synthesizes the TOC-relative call stubs inline in `.text`
(`std r2,24(r1); addis r12,r2,off@ha; ld r12,off@l(r12); mtctr r12; bctr`) and the
`.plt` (NOBITS) slots carry the `puts`/`printf` `R_PPC64_JMP_SLOT` relocations —
exactly the form `elf_plt::decode_ppc64_stubs` recognizes (TOC base = `.got` vma +
`0x8000`, the ELFv2 convention). Drives the PowerPC64 PLT import-name console e2e
(`kuna-console/tests/verify_ppc64_plt.rs`). The build host's `kuna-dev` image ships
the ppc64el runtime libc but not the dev package, so the cross-link needs
`libc6-dev-ppc64el-cross` (headers + `crt1.o`) installed in the build container —
the exact build command (single root container invocation) is:
`docker run --rm --user root -v "$PWD":/w -w /w kuna-dev bash -lc 'apt-get update >/dev/null
&& apt-get install -y --no-install-recommends libc6-dev-ppc64el-cross >/dev/null
&& powerpc64le-linux-gnu-gcc -O0 decompiler/crates/kuna-analysis/tests/fixtures/plt_ppc64le.c
-o decompiler/crates/kuna-analysis/tests/fixtures/plt_ppc64le'`.

`plt_mips32` (7580 bytes, source vendored alongside as `plt_mips32.c`): built with
`mips-linux-gnu-gcc -O0 plt_mips32.c -o plt_mips32` (Ubuntu mips-linux-gnu-gcc
10.3.0, big-endian). `plt_mips32.c` =
`int main(int argc,char**argv){ puts("hello"); printf("%d\n", argc); return 0; }`
— a normal dynamic MIPS32 executable, kept **un**stripped so `main` resolves by
name. `-O0` keeps the libc calls **plain** `puts`/`printf` (an `-O1`+ build pulls
in glibc's fortified `__printf_chk`). It has **no `.plt` and no `R_MIPS_JUMP_SLOT`
relocations** — the o32 lazy-binding layout uses `.MIPS.stubs` + a `$gp`-relative
GOT, so import names come from the dynamic-symbol GOT correspondence
(`DT_MIPS_LOCAL_GOTNO`/`DT_MIPS_GOTSYM`/`DT_PLTGOT`), exactly the form
`elf_plt::resolve_mips_imports` decodes. Drives the MIPS import-name console e2e
(`kuna-console/tests/verify_mips_plt.rs`). The build host's `kuna-dev` image ships
`libc6-mips-cross` (the shared libs) but not the dev package, so the cross-link
needs `libc6-dev-mips-cross` (headers + `crt1.o`) installed in the build
container — the exact build command (single root container invocation) is:
`docker run --rm --user root -v "$PWD":/w -w /w kuna-dev bash -lc 'apt-get update >/dev/null
&& apt-get install -y --no-install-recommends libc6-dev-mips-cross >/dev/null
&& mips-linux-gnu-gcc -O0 decompiler/crates/kuna-analysis/tests/fixtures/plt_mips32.c
-o decompiler/crates/kuna-analysis/tests/fixtures/plt_mips32'`.

## PE (Windows) fixtures — the multi-format loader (PR-3+4)

`pe_imports.exe` (non-stripped, 487 KB) and `pe_imports_stripped.exe` (`-s`,
38 KB) are **linked Windows PE32+** executables for the PE import-naming gate
(`kuna-console/tests/verify_pe_imports.rs`, design §3.2). Both are built from
`pe_imports.c` =
`int main(int argc,char**argv){ puts("hello"); printf("%d\n", argc); return 0; }`
with MinGW-w64 in the `kuna-dev` container (`x86_64-w64-mingw32-gcc`, shipped by
the dev image):

```bash
docker run --rm -v "$PWD":/w -w /w kuna-dev bash -lc \
  'x86_64-w64-mingw32-gcc -O1 pe_imports.c \
     -o decompiler/crates/kuna-analysis/tests/fixtures/pe_imports.exe'
# stripped variant (the PR-4 IAT-naming proof): add `-s`.
```

ImageBase `0x140000000`. `main`@`0x140001592` calls `puts` through a MinGW thunk
veneer@`0x140007240` (`FF 25` `jmp [rip+disp]` → the `__imp_puts` IAT slot
@`0x14000d33c`) and a *local* MinGW `printf` wrapper@`0x140001550` (a `.text`
function, **not** an import — it internally calls `vfprintf`). In the
**non-stripped** exe the COFF symtab names the thunk (`puts`) and the wrapper
(`printf`); in the **stripped** exe those names are gone, so the `puts` call is
named **only** by `s1_loader::pe_iat`'s Import-Directory walk + `FF 25` thunk
decode — that's the load-bearing PR-4 proof. The local `printf` wrapper stays
`sub_<addr>` in the stripped binary (correctly — it is not an import). The PE
exe is the only non-ELF binary in this tree large enough to statically link the
MinGW CRT (≈0.5 MB), on par with the existing `mcount_x86_64` (0.9 MB).
**Pin the VMAs as test consts** (`x86_64-w64-mingw32-objdump -d/-p`).

`coff_obj.obj` (Intel amd64 COFF object, <1 KB) is a **pre-link COFF object** for
the PR-5 object-loader gate (`kuna-console/tests/verify_coff_object.rs`,
design §3.6). Built (no new packages — `clang` ships in `kuna-dev`):

```bash
docker run --rm -v "$PWD":/w -w /w kuna-dev bash -lc \
  'clang -target x86_64-pc-windows-gnu -O1 -c coff_obj.c \
     -o decompiler/crates/kuna-analysis/tests/fixtures/coff_obj.obj'
```

`coff_obj.c` =
`int compute(int x){ return x*3+1; }` /
`int run(int n){ const char *s="hi"; puts(s); return compute(n)+(int)s[0]; }`.
COFF symtab (`objdump -t`): `compute`@`.text`+0x0, `run`@+0x10, `puts` an
**undefined** external (section 0) — a pre-link object has no IAT, so `puts` is an
unresolved *symbol*, not an address (`CoffFormat::resolve_imports` empty, §3.6).
The `"hi"` literal lands in `.rdata` (the format-agnostic string pass's input).
`compute` sits at `.text`+0, exercising the defined-function-at-VMA-0 case the
loader's `is_undefined()` funcsym skip handles (an `addr == 0` skip would have
dropped it). Proves a COFF `.obj` loads and decompiles a function **resolved by
its COFF-symtab name**.

`msvc_mangled.obj` (Intel amd64 COFF object, <1 KB) is a **COFF object carrying
MSVC C++ mangled symbols** for the PR-9 demangler gate
(`kuna-console/tests/verify_msvc_demangle.rs` +
`loadimage_object::tests::msvc_mangled_coff_symbols_are_demangled_name_only`,
design §5.5). `cl.exe` is unavailable on Linux, but `clang -target
x86_64-pc-windows-msvc` emits the *same* `?`-prefixed MSVC mangling (the MSVC C++
ABI — verified `objdump -t`), so this is a **real** MSVC fixture, not a hand-faked
symtab. Built (no new packages — `clang` ships in `kuna-dev`):

```bash
docker run --rm -v "$PWD":/w -w /w kuna-dev bash -lc \
  'clang -target x86_64-pc-windows-msvc -O1 -c msvc_mangled.cpp \
     -o decompiler/crates/kuna-analysis/tests/fixtures/msvc_mangled.obj'
```

`msvc_mangled.cpp` =
`int Bar::foo(int x){ return x*3+1; }` (member, `?foo@Bar@@QEAAHH@Z`) /
`int ns::g(int a,int b){ return a*b+7; }` (namespaced, `?g@ns@@YAHHH@Z`) /
`int freefunc(int x){ return x+42; }` (free, `?freefunc@@YAHH@Z`). The loader's
MSVC demangle arm rewrites each `?`-symbol to its qualified name-only form
(`Bar::foo`, `ns::g`, `freefunc`); `freefunc` decompiles to `a0 + 0x2a` resolved
by that demangled name. Note `strip_version` (the glibc `@@VERSION` stripper) is
guarded to NOT truncate a leading-`?` name (MSVC uses `@` structurally), or every
MSVC symbol would arrive at the demangler cut to `?foo`.

## Mach-O (Apple) fixtures — the multi-format loader (PR-6+7, the Mach-O headline)

`macho_imports` (x86-64, 16 KB) and `macho_imports_arm64` (arm64, 49 KB) are
**linked Mach-O** executables for the Mach-O import-naming gate
(`kuna-console/tests/verify_macho_imports.rs`, design §3.3). Both are the *same*
source `macho_imports.c` =
`int compute(int n){return n*3+7;} int main(int argc,char**argv){ printf("%d\n", compute(argc)); return 0; }`
(`printf` declared, no header) linked for two arches — proving the `__stubs`
naming is arch-independent. Built in the `kuna-dev` container with bare `clang`
(no macOS SDK) + the rustup-bundled `ld64.lld` (an LLD darwin flavor); the
classic `S_SYMBOL_STUBS` indirect-symbol layout PR-7 walks is what `ld64.lld`
emits. `-undefined dynamic_lookup` lets `_printf` stay external:

```bash
# (x86_64; arm64 = -target arm64-apple-macos11 + -arch arm64)
clang -target x86_64-apple-macos11 -O1 -c macho_imports.c -o m.o
LLD=$(rustc --print sysroot)/lib/rustlib/$(rustc -vV | sed -n 's/host: //p')/bin/gcc-ld/ld64.lld
"$LLD" -arch x86_64 -platform_version macos 11.0 11.0 \
       -undefined dynamic_lookup -e _main -o macho_imports m.o
```

ImageBase `0x100000000` (PIE). `main` reaches `printf` by a **direct branch to
the `__TEXT,__stubs` entry** — x86-64 `callq 0x1000005cc`, arm64
`bl 0x1000005a0` — so there is no slot to constant-fold; naming the stub entry
(`sec.addr + i*reserved2`) is enough and arch-independent. The name comes from
the `LC_DYSYMTAB` indirect-symbol table → `LC_SYMTAB` (`_printf`, `_` stripped).
Pinned VMAs (x86-64): `_compute`@`0x1000005a0`, `_main`@`0x1000005b0`, the
`printf` stub@`0x1000005cc`. The defined `_main` keeps its leading `_` (it comes
from the `file.symbols()` funcsym source, not the stub resolver). **Pin the VMAs
as test consts** (`llvm-objdump --macho -d` / `llvm-otool -Iv`).

## Mach-O fat/universal + arm64e (PR-8)

The fat/universal + arm64e gate (`kuna-console/tests/verify_macho_fat.rs`, design
§3.4 / §3.7) reuses the two thin `macho_imports*` slices above:

- **`macho_fat`** (2-slice universal, ~97 KB) wraps `macho_imports` (x86-64,
  slice 0) + `macho_imports_arm64` (arm64, slice 1) behind a big-endian
  `fat_header` + two `fat_arch` records. `llvm-lipo`/`lipo` are **absent** in the
  container, so the fat wrapper is **hand-built** directly from the two real thin
  slices (the fat format is just a header + per-slice
  `{cputype,cpusubtype,offset,size,align}`; both slices page-aligned at
  `2^14`). The dispatch peels one slice (default x86-64; `--slice arm64` selects
  the other) before `object::File::parse`, which cannot parse a fat header.
  Rebuild: the Python snippet in `Increment 45` of `docs/analysis-port-log.md`
  (read each thin slice's header, emit the wrapper) — or `llvm-lipo a b -create
  -output macho_fat` if a `lipo` is available.

- **`macho_arm64e`** (~49 KB) is the `macho_imports_arm64` fixture with its header
  `cpusubtype` flipped to `CPU_SUBTYPE_ARM64E` (2). arm64e is binary-compatible
  arm64 (same encodings plus PAC), so the real arm64 code decodes under the
  AppleSilicon v8.5-A superset spec. With `--option macho-arm64e on` the loader
  selects `AARCH64:LE:64:AppleSilicon`; off ⇒ generic `v8A`. The **load +
  spec-selection path is real**; only the cpusubtype is synthesized (no
  `clang -arch arm64e` SDK in-container — a genuine Apple-toolchain arm64e binary
  is a follow-up). Rebuild: copy `macho_imports_arm64` and overwrite the 4-byte
  cpusubtype at offset 8 with little-endian `2`.

## Stripped-PE / stripped-Mach-O entry discovery (PR-12+13)

The multi-format **entry-discovery** gate
(`kuna-console/tests/verify_multiformat_entry.rs`, design §4.1 / §5.3) proves a
*stripped* PE/Mach-O recovers its function starts with **no `--addr`**, exactly
as a stripped ELF does (`verify_s1_entry`). The two PE/Mach-O *import* fixtures
above are reused, plus one new stripped Mach-O:

- **PE:** `pe_imports_stripped.exe` (already above) — fully stripped (0 symbols,
  0 exports). The `s1_entry` PE oracles recover its functions from the entry
  point (`AddressOfEntryPoint`@`0x1400014f0`) and the **`.pdata`** exception
  directory (97 `RUNTIME_FUNCTION` records — the `.eh_frame` analog), incl.
  `main`@`0x140001592`. A bare load finds nothing; the oracles find dozens.

- **Mach-O:** `macho_func_starts_stripped` (x86-64, 16 KB) is a **stripped**
  Mach-O whose `helper`@`0x100000590` is `static` (file-local), so `ld64.lld -x`
  removes its symbol — leaving **`LC_FUNCTION_STARTS`** as the only source that
  recovers it. `macho_func_starts_stripped.c` =
  `static int helper(int n){return n*7+3;} int main(int argc,char**argv){ printf("%d\n", helper(argc)); return 0; }`.

  ```bash
  LLD=$(rustc --print sysroot)/lib/rustlib/$(rustc -vV | sed -n 's/host: //p')/bin/gcc-ld/ld64.lld
  clang -target x86_64-apple-macos11 -O0 -fno-inline -c macho_func_starts_stripped.c -o m.o
  "$LLD" -arch x86_64 -platform_version macos 11.0 11.0 -undefined dynamic_lookup \
         -e _main -x -dead_strip -o macho_func_starts_stripped m.o
  ```

  `LC_FUNCTION_STARTS` decodes (ULEB128 deltas off `__TEXT`@`0x100000000`) to
  `[0x100000550 (_main, still symboled — the entry), 0x100000590 (helper,
  stripped)]`. `collect_entries` skips the symboled `_main` and **discovers
  `0x100000590`** — the never-symboled `helper` — the load-bearing PR-13 proof.

## DWARF on MinGW-PE / Mach-O (PR-11)

The multi-format **DWARF** gate (`kuna-console/tests/verify_multiformat_dwarf.rs`,
design §5.2 / §8 PR-11) proves the `s1_dwarf` pass (gimli) recovers DWARF function
names + typed signatures on PE and Mach-O, not just ELF. Both fixtures are the
per-format analog of `dwarf_stripped_x86_64`: the function names live **only** in
the debug sections (the symtab FUNC entries are stripped/renamed, `.debug_*` kept),
so a recovery by name is unambiguously DWARF-sourced. Shared source (no headers,
so it cross-compiles to macOS without an SDK; `pe_dwarf.c` / `macho_dwarf.c` carry
the identical bodies + their build recipes):
`int first_byte(char *label){return label[0];} int add(int a,int b){return a+b;} int main(void){return first_byte("kuna")+add(2,3);}`.

- **`pe_dwarf.exe`** (MinGW `-g`, ~70 KB): MinGW emits standard `.debug_*` sections
  in the PE, which `object::section_by_name(".debug_info")` finds verbatim. Built
  in the `kuna-dev` container, then the COFF-symtab FUNC entries removed (keeping
  `.debug_*`):

  ```bash
  x86_64-w64-mingw32-gcc -g -O0 pe_dwarf.c -o pe_g.exe
  x86_64-w64-mingw32-objcopy --strip-symbol first_byte --strip-symbol add \
      --strip-symbol main  pe_g.exe  pe_dwarf.exe
  ```

  Pinned VMAs (ImageBase `0x140000000`): `first_byte`@`0x140001550`,
  `add`@`0x140001564`. DWARF recovers `int4 first_byte(char *a0)` by name; a
  by-`load addr 0x140001550` decompile (the no-DWARF-name baseline) renders the
  engine's `sub_140001550` placeholder.

- **`macho_dwarf.o`** (clang `-g`, relocatable, ~2 KB): the DWARF lands in the
  `__DWARF,__debug_*` sections; `object` maps gimli's `.debug_info` → the Mach-O
  short-name `__debug_info` (its documented rule), so the *same* section loader
  reads it. A Mach-O object with `SUBSECTIONS_VIA_SYMBOLS` won't let strip drop
  its FUNC symbols (they delimit subsections), so `--redefine-sym` **renames** them
  instead (`_first_byte`→`_l0`, `_add`→`_l1`) — DWARF still names them, the symtab
  no longer does:

  ```bash
  clang -target x86_64-apple-macos11 -g -O0 -c macho_dwarf.c -o macho_dwarf.o
  llvm-objcopy --redefine-sym _first_byte=_l0 --redefine-sym _add=_l1 macho_dwarf.o
  ```

  Pinned VMAs (section-relative in the object): `first_byte`@`0x0`, `add`@`0x20`.
  Same DWARF recovery + `char *` type; `load addr 0x0` is the `sub_0` baseline.

All other fixtures are checked in well under 32 KB so the gates are hermetic and
reproducible. **Pin load-bearing VMAs as test consts** (read via
`objdump`/`readelf` at build time) — addresses shift across toolchains.

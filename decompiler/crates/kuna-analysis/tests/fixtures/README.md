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
| `et_rel_status_arm.o` | project-authored ARM32 **ET_REL** object from `et_rel_status_arm.s` | architecture-specific `R_ARM_CALL`/data relocation application and return recovery when a status value is both returned on the normal path and passed to an explicit no-return call on a terminal guard-failure path; `status_caller` consumes the recovered result |
| `et_rel_status_aarch64.o` | project-authored AArch64 **ET_REL** object from `et_rel_status_aarch64.s` | `R_AARCH64_CALL26`, page/low-12 relocation application, and the same status-return/no-return-path recovery shape on the 64-bit ABI; `status_caller` consumes the recovered result |
| `entry_selectors_x86_64.o` | synthetic x86-64 **ET_REL** object produced from `entry_selectors_{a,b}_x86_64.s` | relocatable-object entry selection: two local `STT_FUNC` definitions share the name `duplicate_local` and raw offset zero but live in distinct `.text.selector_a` / `.text.selector_b` sections, so name and bare-offset selection must report both candidates while a section-qualified selector is exact |
| `fauxware` | classic non-PIE x86-64, not stripped (the angr `fauxware` sample) | `.plt` classic stubs (`FF 25` rip-rel), `.symtab` defined functions; `.eh_frame` FDE starts (`s1_entry`: 7 FDE starts incl. `_start`/`main`/`register_tm_clones`) |
| `cet_pie_x86_64` | PIE x86-64 with CET (`.plt.sec`) | `endbr64; FF 25` CET stubs, naming at the `.plt.sec` call target |
| `stripped_dynamic_x86_64` | PIE x86-64, `.symtab` stripped (only `.dynsym`) | PLT resolution with no `.symtab` (dynsym/rela.plt only); entry discovery (`s1_entry`): `e_entry`=0x1160, `DT_INIT`=0x1000, `DT_FINI`=0x1464, INIT/FINI_ARRAY ptrs, `_start`→`main` idiom → 0x1405, `.eh_frame` FDE starts — `sub_1405` (main) decompiles without `--addr` |
| `cpp_mangled_x86_64` | non-PIE x86-64 C++, not stripped | symbol demangling (`s1_demangle`): a defined `.symtab` C++ method `_ZN3foo3Bar3bazEi` must surface name-only as `foo::Bar::baz` |
| `msvc_rtti_x64.exe` / `msvc_rtti_x86.exe` | linked Windows PE (PE32+/x86-64 and PE32/x86), polymorphic C++ (`Shape` base + `Box` derived, virtual method), source `msvc_rtti.cpp` | MSVC RTTI / vftable class-name recovery (`s1_rtti`, `--option rtti on`): the `CompleteObjectLocator` → RTTI3/2/1 → RTTI0 graph in `.rdata`/`.data` recovers `Box`/`Shape` + labels `Box::vftable` / `<Class>::RTTI_Type_Descriptor` / `Box::RTTI_Complete_Object_Locator`. Exercises BOTH the x64 IBO32 image-base-relative ref path (name offset 16) and the x86 raw-VA path (name offset 8). VMAs pinned below |
| `pe_chainedunwind_x86_64.exe`, `pe_chainedunwind_loop_x86_64.exe`, `pe_chainedunwind_plainft_x86_64.exe` | hand-assembled PE32+/x86-64 (2 KB each; generators `pe_chainedunwind*.py`, regenerate with `python3 <name>.py`) whose `.pdata` splits ONE logical function across two `RUNTIME_FUNCTION` records, the second carrying `UNWIND_INFO` with `UNW_FLAG_CHAININFO` | The chained-record entry skip (`pdatachained`, default-on, GH-403; `--option pdatachained off` restores the defect). The three differ only in what the primary's last instruction is, which is what decides the shape of the damage the bogus entry causes: a conditional branch (`} while ;`, invalid C), a loop latch (the decompile fails outright), and an ordinary fall-through (the second half of the function silently disappears). `kuna-console/tests/verify_pdatachained.rs` is the two-pass e2e over all three; `tests/cli/pe-chained-unwind-truncates-function.json` is the CLI probe. No Windows toolchain on this host, hence the byte-by-byte generators (same pattern as `crtmain_x86_64.py`) |
| `arraycoverwidth_x86_64` | project-authored non-PIE x86-64 ELF from `arraycoverwidth_x86_64.s` (`-nostdlib -Ttext=0x100000 -e vm`, 5 KB): two 16-byte stack banks zeroed with `movaps`, escaped to `sink` so nothing is dead, then swapped with the `movdqa/movdqa/movaps/movaps` quartet | the array-cover width render (`arraycoverwidth`, default-on; `--option arraycoverwidth off` restores the defect). It is the reduction of the crackmes.one `KataVM_L1` VM-interpreter witness whose sixteen-byte bank swap printed `v30[0] = v32[0];`, a one-byte lvalue for a sixteen-byte copy. `vm`@`0x100000`, `sink`@`0x100049`; the trailing `movzbl 0x3(%rsp)` is the genuine one-byte in-element read that must KEEP its `[3]` subscript. `tests/cli/16-byte-vm-state.json` is the CLI probe, `tests/stages/kuna-arraycoverwidth.xml` the two-pass stage test |
| `pe_pdata_arm64.exe` | hand-assembled ARM64 PE32+ (1.5 KB; generator `pe_pdata_arm64.py`) with four functions and four 8-byte ARM `{BeginAddress, UnwindData}` `.pdata` records | The machine-dependent `.pdata` record stride (ungated). Read at the x64 stride of 12 the 32-byte directory yields two entries, one of them only because record 0 sits at offset 0; at the ARM stride all four functions are discovered |
| `pdb_prog.exe` + `pdb_prog.pdb` (+ `pdb_prog_mismatch.pdb`) | x86-64 Windows PE built `-g -gcodeview` with its matching `.pdb`, source `pdb_prog.c` | PE PDB metadata recovery (`s1_pdb`, `--option pdb on` + `kuna_pdb_path=<...>/pdb_prog.pdb`): a stripped `FUN_<addr>` → its real name `pdb_demo_compute` from the PDB `S_PUB32`/`S_GPROC32` stream, gated by the GUID/age fingerprint check. `pdb_prog_mismatch.pdb` (a different content-hash GUID) drives the negative gate (mismatch → no rename). VMA/GUID pinned below |
| `cpp_noreturn_x86_64` | non-PIE x86-64 C++, not stripped (source `cpp_noreturn_x86_64.cpp`) | the **no-return × demangle cross-pass seam** (`s1_loader::noreturn` + `s1_demangle`): `.dynsym` carries the mangled no-return imports `_ZSt9terminatev` (demangled `std::terminate`) and `__cxa_throw`, both UND (`.dynsym` address 0) — their real FunctionSymbols are installed at the PLT stubs `_ZSt9terminatev@plt`=`0x401070`, `__cxa_throw@plt`=`0x4010a0`. The no-return scan emits those **stub addresses** under the raw names, so the commit resolves the *demangled* funcsym **by address** (`find_function_across_scopes`); a name lookup of the mangled string would miss. e2e: `fail()` (`_Z4failv`=`0x401196`, demangled `fail`) tail-calls `std::terminate()` → `void fail(void)` with the `Subroutine does not return` warning and no dead fall-through; `main`=`0x4011a3` |
| `eh_lsda_x86_64` | non-PIE x86-64 C++ try/catch, **`.symtab` stripped** (source `eh_lsda_x86_64.cpp`) | `.eh_frame` LSDA landing-pad discovery (`s1_entry::EhFrameLsdaPass`, gated `--option eh_frame_full on`, the GccExceptionAnalyzer full `.gcc_except_table` markup): the `zPLR` CIE's `L` augmentation points each FDE at its LSDA in `.gcc_except_table` (`may_throw`@`0x40218c`, `guarded`@`0x402198`); the call-site tables decode to landing pads `0x4012bf` (may_throw cleanup), `0x4012e2` (guarded catch dispatch), `0x401352`/`0x401366` (guarded cleanup) — all `endbr64`, all **mid-function** (reached only by the unwinder, so NOT FDE pcBegins; the FDE-start oracle misses them). e2e (`verify_eh_frame_full`): with `--option eh_frame_full on`, `0x4012e2` registers as `sub_4012e2` and decompiles by name; default-off it is absent (discovery byte-identical to FDE-pcBegin only). FDE pcBegins (function starts): `may_throw`=`0x401256`, `guarded`=`0x4012d6`, `main`=`0x40137a` |
| `cppproto_x86_64` | non-PIE x86-64 C++ built `-O0 -g`, not stripped (source `cppproto_x86_64.cpp`) | the DWARF **C++ prototype** arm (`s1_dwarf::kuna_cppproto`, `--option cppproto`, default-on; e2e `verify_cppproto`). Every interesting function is a subprogram DEFINITION whose name is NOT on the definition DIE: `db::inner::scaled_add`@`0x401156` (namespace, `DW_AT_specification`), `Account::deposit`@`0x4011b2` (out-of-line member + artificial `this` typed by a `DW_TAG_class_type`), `Account::available`@`0x40120c` (`const` member -> `const Account *const`, the four-DIE qualifier chain that blew the type-mapper depth cap), `Account::bump`@`0x401232` (`const` member with a `DW_TAG_reference_type` parameter), `Account::make_id`@`0x401264` (`static` member, no artificial `this`). `maxof<int>`@`0x4014aa` / `maxof<double>`@`0x4014ca` DO carry their own `DW_AT_name`, but kuna files the demangled name as `maxof`, so only the ADDRESS-keyed prototype park reaches them. `probe_virtual_call`@`0x40127e` takes a `Shape *` (`void *` before the class arm) |
| `cppsig_x86_64.so` | x86-64 C++ **shared library**, `-O0 -fPIC -fno-inline`, then `strip --strip-all` (source `cppsig_x86_64.cpp`) | the DEMANGLED C++ **signature** arm (`s1_demangle::kuna_cppsig`, `--option cppsig off\|proven\|inferred`, default `proven`; e2e `verify_cppsig`). Fully stripped, so there is no DWARF and no `.symtab` — the exported `.dynsym` mangled names are the only signature source, which is the situation the feature exists for. One function per shape of the `this` decision: `sig::Account::Account` (ctor, `C1`/`C2`) and `sig::Account::~Account` (dtor, `D1`/`D2`) and `sig::Account::balance` (`_ZNK`, `const`) are PROVEN and recover `Account *this` at the default; `sig::Account::deposit` (plain member) and `sig::combine` (namespaced free function) are AMBIGUOUS and need `inferred`, which then gets both right (`this` on the member, none on the free function); `sig::Account::rate` (STATIC member) is the measured cost — refused by `proven`, given a spurious `this` by `inferred`. `sig_global` is an unqualified global (no `this` possible). `balance` also pins the return-type contract: `unsigned int` must survive the input-only prototype lock |
| `itaniumrtti_x86_64.so` | x86-64 C++ **shared library**, `-O0 -fPIC -fvisibility=hidden -fvisibility-inlines-hidden`, then `strip --strip-all` (source `itaniumrtti_x86_64.cpp`) | Itanium (GCC/Clang) RTTI + vtable recovery (`s1_rtti::kuna_itaniumrtti`, `--option itaniumrtti on`, default-off; e2e `verify_itaniumrtti`). Hidden visibility is load-bearing: without it every implicit class method is emitted WEAK and *exported*, so `.dynsym` alone would name them and the recovery would have nothing to prove. Hidden **and** stripped, the only defined dynamic symbols are `probe_shapes` / `probe_widget` / `probe_generic`, so every class name, vtable and virtual method has to come from the `.rela.dyn` `__cxxabiv1` anchor or from nowhere. Covers all three typeinfo flavours — `shapes::Shape` (`__class_type_info`, no bases), `shapes::Circle` (`__si_class_type_info`), `shapes::Widget` (`__vmi_class_type_info`, `Loggable` at +0 and `Drawable` at +16, so the vtable object carries a SECOND sub-vtable of `this`-adjusting thunks with `offset-to-top = -16`) — plus the two naming hazards that silently cost recovery: `shapes::Vec<int>` / `shapes::Vec<double>` (distinct classes whose NAME-ONLY demangling collides) and `(anonymous namespace)::Hidden` (a TU-local type, whose ABI type-name string carries the leading `*` marker). `shapes::Shape::perimeter` is inherited unchanged by `Circle`, so it also pins the defining-base slot attribution |
| `dwarf_stripped_x86_64` | non-PIE x86-64, **`.symtab`/`.dynsym` FUNC names removed but `.debug_*` kept** | DWARF recovery (`s1_dwarf`): names + typed signatures of `add_values`/`compute`/`main` come **only** from `.debug_info` (the funcsym stream has none) |
| `switchtab_x86_64` | non-PIE x86-64, dense `switch(x){0..7}` | address/jump tables (`addrtable`): an absolute 8-byte jump table in `.rodata` at vma `0x402008` (`jmp *0x402008(,%rdi,8)`) |
| `rust_hello_x86_64` | tiny `#![no_std]` rustc PIE (x86-64), **not stripped** | source-language detection (`s1_sourcelang`): `.comment` carries `rustc version 1.90.0 …` (the faithful `ElfRustSourceLanguage` comment path) AND `.symtab` carries a Rust-mangled symbol `_ZN5nostd1m12rusty_helper17h…E` (the legacy `_ZN…17h<hex>E` heuristic) — both detection paths fire |
| `rust_scalarpair_x86_64` | tiny `#![no_std]` rustc **non-PIE** (`-C relocation-model=static`) x86-64, **not stripped** (source `rust_scalarpair_x86_64.rs`) | the rustc **two-register `ScalarPair` return** (`option rustabi`, P4; e2e `kuna-console/tests/verify_rustabi_pair.rs`). `prod`@`0x201270` is `fn(u32) -> Result<u32,u32>`, compiled to the branchless discriminant/payload pair `xor %eax,%eax; setb %al` (RAX, the tag) + `lea 0x7(%rdi),%edx; cmovae %ecx,%edx` (RDX, the payload). `cons`@`0x201290` is the `match` that consumes it, calling `prod` **directly** (`e8 rel32`, which the static relocation model buys) and reading the payload out of RDX after `test $0x1,%al`. Both are `#[inline(never)]`; a volatile-guarded `_start`@`0x2012b0` keeps them from being optimized away. `.comment` carries the `rustc version` record, so `option rustabi auto` fires here without `always`. The same two functions are the `<bytechunk>` in `tests/stages/kuna-rustabi.xml` |
| `rust_clobber_pair_x86_64` | tiny `#![no_std]` rustc **non-PIE** (`-C relocation-model=static`) x86-64, **not stripped**, two `global_asm!` functions (source `rust_clobber_pair_x86_64.rs`) | the `option rustabi` **call-seam NEGATIVE**: `scalar_callee`@`0x201240` is `movq %rdi,%rax; addq $7,%rax; ret` — it provably never writes RDX — while `pair_shaped_reader`@`0x201250` calls it and then reads RDX twice and tests the low byte of RAX, which is byte for byte the caller-side shape of a real `ScalarPair` consumer. Nothing at the call site separates the two, so the seam decodes the callee (`probe_callee_return_writes`) and refuses the pair; the function must render identically with the option off and on. Hand-written asm because no compiler emits a read of a caller-saved register the callee never sets. The same two functions are the second `<bytechunk>` in `tests/stages/kuna-rustabi.xml` |
| `dwarfvariants_x86_64` | tiny `#![no_std]` rustc **non-PIE** x86-64 built with **`-C debuginfo=2`**, **not stripped** (source `dwarfvariants_x86_64.rs`) | DWARF **`DW_TAG_variant_part`** import (`option dwarfvariants`, P1; e2e `kuna-console/tests/verify_dwarfvariants.rs`, stage `tests/stages/kuna-dwarfvariants.xml`). Eight `#[inline(never)]` functions, one per shape the importer has to answer for: `ret_result`@`0x201220` (`Result<u32,u32>`, tag u32 @0 / payload @4, discr 0=`Ok` 1=`Err`), `ret_option`@`0x201240` (`Option<u32>`, a FIELDLESS `None` variant), `ret_niche`@`0x201250` (`Option<&u32>`, NICHE-encoded: `Some` is the DEFAULT variant with no `DW_AT_discr_value`, and its payload overlaps the discriminant), `ret_three`@`0x201260` (THREE variants, one fieldless), `ret_multi`@`0x201290` (a variant with TWO fields), `list_len`@`0x2012c0` (RECURSIVE: `enum List { Cons(u32, *const List), Nil }`), `ret_plain`@`0x2012e0` (a fieldless enum, which rustc emits as `DW_TAG_enumeration_type` and this pass must never see), `ret_pair`@`0x201300` (a plain C-shaped struct, which must be byte-identical either way). The whole file carries **10 `DW_TAG_variant_part`s and 0 NESTED ones** |
| `dwarfvariants_overlay_x86_64` | tiny `#![no_std]` rustc **non-PIE** x86-64 built with **`-C debuginfo=2`**, **not stripped** (source `dwarfvariants_overlay_x86_64.rs`) | The `option dwarfvariants` **NAMING RULE** — what the importer is allowed to name, as opposed to what it can read (e2e `kuna-console/tests/verify_dwarfvariants.rs`, stage `tests/stages/kuna-dwarfvariants.xml`). A union member selects itself by OFFSET and the discriminant is never consulted, so a variant name is sound only where exactly one variant claims the bytes. `r16`@`0x201220` and `use16`@`0x201240` are a `Result<u64,u64>` producer/consumer pair (size 16, tag u64 @0, `Ok` discr 0 and `Err` discr 1 BOTH with `__0` at 8) — the case where it is NOT sound, and this binary rendered `Ok` ten times and `Err` never before the suppression; `put_res`@`0x201260` writes the same payload through a pointer so the store is a field path (`(dst->payload).field_0x8.__0`, no variant named); `put_opt`@`0x201280` does the same for an `Option<u64>`, whose only payload-carrying variant is `Some`, so `(dst->payload).Some.__0` is FORCED and must survive. `_start`@`0x2012a0`. Carries **6 `DW_TAG_variant_part`s** |
| `arm_thumb_le32.o` | bare ARM Thumb **`.o`** (ET_REL, EABI5, LE) — **not linked** (no PT_LOAD; see note) | ARM/Thumb decode-mode markers (`s1_loader::arm_markers`): `.symtab` carries the `$t.0` Thumb mapping symbol at `.text+0x0` AND STT_FUNC syms `thumb_add`@`0x1` / `_start`@`0x15` (LSB-set, the Thumb odd-address convention). The pass emits a `TMode=1` paint for `$t.0` (at `0x0`) and for each LSB-set FUNC normalized to even (`0x0`, `0x14`) |
| `arm_thumb_linked_le32` | **LINKED** ARM Thumb ET_EXEC (LE, `-static -nostdlib`) — one PT_LOAD R E at `0x10000` (so `ObjectLoadImage` loads it, unlike the bare `.o`) | ARM/Thumb decode **e2e** (`s1_loader::arm_markers` + the commit seam, `kuna-console/tests/verify_arm_thumb_decode.rs`): the `$t`@`0x100b8` mapping symbol + the LSB-set FUNCs `compute`@`0x100b9` (→ even `0x100b8`) / `_start`@`0x100d7` (→ even `0x100d6`) drive a `TMode=1` paint, so `load function compute` Thumb-decodes `compute(x)` to `return a0 * 3 + 7;` (an ARM-mode misdecode of the same bytes is garbage), and the Thumb-FUNC re-home makes `_start`'s `bl` to compute's even entry render `compute(5)`. **The deferred Increment-8/17 decode e2e, now built in-container** |
| `arm_thumb_switch_le32` | **LINKED** ARM Thumb ET_EXEC (LE, `-Os -static -nostdlib`), 1304 bytes, source `arm_thumb_switch_le32.c` | ARM/Thumb **jump table + `<callotherfixup>` injection** e2e (`tests/stages/ghdec-isamode-inject.xml`): `dispatch`@`0x100cc` compiles to `tbb [pc,r0]` (the table bytes inline at `0x100d6`) with eight table-reachable case blocks, four of which hold a pair of `bl`s. Both `tbb` and Thumb-2 `bl` lower through SLEIGH `SetThumbMode` → the `setISAMode` CALLOTHER, which `ARM.cspec`'s `<callotherfixup targetop="setISAMode">` declares to be a NOP, so the emitted C must contain no `setISAMode`. Before the P2 injection-drain fix its `dispatch` carried eight `setISAMode(1);` statements. `f0`=`0x100b8`, `f1`=`0x100bc`, `f2`=`0x100c2`, `f3`=`0x100c6`, `_start`=`0x1014e` |
| `mcount_x86_64` | static, non-PIE x86-64, `gcc -pg` (`-O0`), `.debug_*` stripped | call-fixup auto-apply (`s1_callfixup`): the `-pg` prologue emits a direct `call mcount` to the weak `mcount` FUNC symbol (0x44a710); `main` is at 0x401795. The cspec (`x86-64-gcc.cspec`) registers `<callfixup name="mcount"><target name="mcount"/>` (body `temp:1 = 0;`), so tagging `main`'s `mcount` callee with that fixup's inject id dissolves the profiling call — `kuna decompile … main` then shows no `mcount();` line. Also carries `__fentry__` (0x44a770, the `fentry`-fixup target) |
| `fmt_x86_64` | non-PIE x86-64, `gcc -O0`, not stripped (source `fmt_x86_64.c`) | format-string varargs typing (`s1_formatstring` half B, `FormatStringAnalyzer`, **gated off** by default): `main`=0x401136 calls `printf("%d %s\n", argc, argv[0])` (`printf@plt`=0x401040; the `"%d %s\n"` format constant is at `.rodata` vma 0x402004). With `--option formatstring on`, the console reads the format constant at the `printf` call's format slot, parses `%d`→int / `%s`→char\*, installs a per-call-site prototype override, and re-decompiles so the call renders `printf("%d %s\n",a0,(char *)*a1)` (the `%d` arg as a plain `int`, the `%s` arg cast to `char *`) instead of the default untyped `printf("%d %s\n",(uint8)a0,*a1)` |
| `operand_refs_x86_64` | non-PIE x86-64, `gcc -no-pie -fno-pic -mcmodel=large -O0`, not stripped (source `operand_refs_x86_64.c`) | scalar/operand reference markup (`s1_operand_refs`, `ScalarOperandAnalyzer` family, **gated off** by default): `main`=0x40112e materializes the address of the short `.rodata` string `"hi"`@`0x402004` with `movabs $0x402004,%rax` (the large code model puts the absolute address DIRECTLY in code as a bare immediate — the `ScalarOperandAnalyzer` case; a RIP-relative `lea` would not surface a bare scalar) and passes it to the **no-prototype** `mystery`=0x401106. `"hi"` is 2 chars (< 5) so the always-on `StringLiteralPass` skips it, and `mystery` has no libproto/S5 typing, so the literal renders ONLY via `operand_refs`. With `--option operand_refs on` the call renders `mystery("hi")`; default-off `mystery(0x402004)`. Drives `kuna-console/tests/verify_operand_refs.rs` |
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
| `cortexm_ccm_vectors_le32` | hand-assembled, **stripped** bare-metal ARM Cortex-M ELF32 (357 bytes; generator `cortexm_ccm_vectors_le32.py`, regenerate with `python3 cortexm_ccm_vectors_le32.py`) | the **widened Cortex-M vector-table signature** (`s1_entry` oracle 6, `--option cortexmvectors on`, default-off). Reproduces in one file all three reasons the shipped signature rejects real STM32 firmware: `.isr_vector`@`0x08000000` is `SHF_ALLOC` only **and** sits in a read-only `PT_LOAD` (so it is neither `SHF_EXECINSTR` nor inside a `PF_X` load — cleanflight/betaflight); its `word[0]` is `0x1000fff0`, a stack in STM32F4 **CCM RAM**, outside the architectural SRAM window (cleanflight/betaflight); and its `word[1]` (`Reset_Handler|1` = `0x08008001`) is **not** `e_entry`, which the link script points at `_start`@`0x08008011` (crazyflie/nuttx). `.text`@`0x08008000` holds five two-instruction Thumb bodies `movs r0,#k ; bx lr` for k = 1/7/11/21/0 at `0x08008000`/`04`/`08`/`0c`/`10`, four of them reachable ONLY through a vector slot. `kuna-console/tests/verify_cortexmvectors.rs` is the two-pass e2e: default (option off) registers ONLY `sub_8008010` and even that produces no C (nothing paints `TMode=1`, so the Thumb halfwords are read as A32); with the option on all five register and each decompiles to its constant (`sub_8008004` -> `return 7;`). No cross toolchain on this host emits a bare-metal STM32 link layout, hence the byte-by-byte generator |
| `cortexm_ptrentry_le32` | hand-assembled, **stripped** bare-metal ARM Cortex-M ELF32 (405 bytes; generator `cortexm_ptrentry_le32.py`, regenerate with `python3 cortexm_ptrentry_le32.py`) | **pointer-referenced function entries** (`aif::kuna_ptrentry`, `--option ptrentry on`, default-off). Its vector table is the *shipped*-signature shape (`.text`@`0x08000000` is `AX`, `word[0]` = `0x20001000`, `word[1]` = `e_entry` = `0x08000041`), so nothing here depends on `cortexmvectors`. Carries the two shapes the option must tell apart: `LEAF`@`0x08000048` (`movs r0,#7 ; bx lr`) is reachable ONLY through a `.rodata` function-pointer word at `0x08000060` — no `BL`, no frame prologue, two instructions, so every shipped stage rejects it; and `SWCASE`@`0x0800005c`, byte-for-byte the same shape, whose pointer word at `0x08000058` lies in the **same discovered function** as its target, i.e. the `ldr pc,[pc,r]` switch-table layout that must stay rejected. `reset`@`0x08000040` `BL`s `callee`@`0x08000050` so the walk finds both on its own. `kuna-console/tests/verify_ptrentry.rs` is the two-pass e2e: default registers only `sub_8000040`/`sub_8000050` and `LEAF`'s bytes produce no C at all; with the option on `sub_8000048` registers and decompiles to `return 7;` while `sub_800005c` stays undiscovered either way. No cross toolchain on this host emits a bare-metal STM32 link layout, hence the byte-by-byte generator |
| `cortexm_aifstrict_le32` | hand-assembled, **stripped** bare-metal ARM Cortex-M ELF32 (581 bytes; generator `cortexm_aifstrict_le32.py`, regenerate with `python3 cortexm_aifstrict_le32.py`) | **The AIF gap-cursor aligned slide** (`aif::kuna_aifstrict`, `--option aifstrict off` restores the defect; default-on, GH-299). Same vector-table and 20-helper scaffolding as `cortexm_poolentry_le32` above — the *shipped*-signature vector shape, so nothing depends on `cortexmvectors`, and twenty `movs r0,#k ; movs r1,#k ; movs r2,#k ; bx lr` helpers at `0x080000a0` that clear both of AIF's floors. Two shapes: **THE DEFECT** (`A`@`0x08000140` loads `POOL1`@`0x08000148` = `0x20001000`; the byte-granular cursor rejects `POOL1`, slides one byte, and accepts `0x0800014a`, where the pool word's HIGH halfword `0x2000` decodes as `movs r0,#0` and completes the helpers' fingerprint — and because an accept advances the cursor past the accepted body, the real `B`@`0x0800014c` is never probed, so the phantom REPLACES it; with the option on, `0x0800014a` is 2-mod-4 and not a hole start, the slide goes straight to `B`, and `B` is recovered); and **THE CONTROL** (`C`/`POOL2`@`0x0800015c`/`D`@`0x08000160`, identical except `D` opens `movs ; adds`, a fingerprint nothing shares, so the *aligned* probe at the pool end is REJECTED in both passes — the option declines to probe addresses that cannot be instruction boundaries, it never lowers the acceptance bar). `kuna-console/tests/verify_aifstrict.rs` is the two-pass e2e over both, plus an `aif off` inertness pin. No cross toolchain on this host emits a bare-metal STM32 link layout, hence the byte-by-byte generator |
| `cortexm_aifcorroborate_le32` | hand-assembled, **stripped** bare-metal ARM Cortex-M ELF32 (1,165 bytes; generator `cortexm_aifcorroborate_le32.py`, regenerate with `python3 cortexm_aifcorroborate_le32.py`) | **The AIF accept corroboration test** (`aif::kuna_aifcorroborate`, `--option aifcorroborate on`; default-off, in no preset, GH-313). Same *shipped*-signature vector table as the fixtures above, so nothing depends on `cortexmvectors`, but the fingerprint histogram is stocked with **two** counts on purpose: twenty `movs r0,#k ; movs r1,#k ; movs r2,#k ; bx lr` helpers at `0x08000160` give `movs ; movs` a count of 20 (past AIF's floor of 4, below the corroboration threshold of 50) and fifty `movs r0,#k ; adds r1,#k ; adds r2,#k ; bx lr` helpers at `0x08000200` give `movs ; adds` a count of exactly 50; the reset vector `BL`s all seventy so the walk discovers them. Three shapes follow in the trailing undefined gap, one per branch of `startCount >= 50 || corroborated`: **THE DEFECT** (`U`@`0x08000390` opens `movs ; movs` (20), calls nothing, jumps nowhere and only reaches `bx lr` — upstream's `AggressiveInstructionFinderAnalyzer.java:367` refuses exactly this and kuna never ported it, so it is accepted by default and refused with the option on); **THE CORROBORATED CONTROL** (`V`@`0x0800039c`, the SAME count-20 prologue but its third instruction is a `bl` into the discovered `H1`, so upstream's "calls always add info" keeps it in BOTH passes — same count, opposite verdict, which is what proves the option tests corroboration rather than raising the count floor); and **THE COUNT CONTROL** (`W`@`0x080003a8`, as uncorroborated as `U` but opening the count-50 fingerprint, so `50 >= 50` keeps it in both passes). `U`'s own interior at `0x08000392` is deliberately a count-50 `movs ; adds` prologue that still reaches the same `bx lr`: it would be accepted on the count branch if refusing `U` released the gap cursor into `U`'s body, so its absence in both passes pins the reject-claims-its-body pairing (dropping that pairing turns a 361-entry mid-body cut into a 222-entry mid-body RISE on the 3.4 MB PE witness). `kuna-console/tests/verify_aifcorroborate.rs` is the two-pass e2e over all three shapes plus the cursor pairing and an `aif off` inertness pin. No cross toolchain on this host emits a bare-metal STM32 link layout, hence the byte-by-byte generator |
| `cortexm_poolentry_le32` | hand-assembled, **stripped** bare-metal ARM Cortex-M ELF32 (601 bytes; generator `cortexm_poolentry_le32.py`, regenerate with `python3 cortexm_poolentry_le32.py`) | **ARM literal-pool inference** (`aif::kuna_poolentry`, `--option poolentry on`, default-off). Its vector table is the *shipped*-signature shape (`.text`@`0x08000000` is `AX`, `word[0]` = `0x20001000`, `word[1]` = `e_entry` = `0x08000041`), so nothing here depends on `cortexmvectors`. Twenty `movs r0,#k ; movs r1,#k ; movs r2,#k ; bx lr` helpers at `0x080000a0`, all `BL`-reached from the reset vector, clear AIF's two floors at once — `MINIMUM_FUNCTION_COUNT` (20) and `FINGERPRINT_THRESHOLD` (4 functions sharing the `movs ; movs` / 4-byte prologue fingerprint). Three shapes follow: **PHANTOM** (`A`@`0x08000140` loads `POOL1`@`0x08000148` = `0x20001000`, whose HIGH halfword `0x2000` decodes as a dead `movs r0,#0`, so AIF accepts `0x0800014a` and jumps past `B`@`0x0800014c`, which is never probed — with the option on the entry MOVES from `sub_800014a` to `sub_800014c`); **UNPAIRED** (`C`/`POOL2`@`0x0800015c`/`D`@`0x08000160`, identical except `D` opens `movs ; adds`, a fingerprint nothing shares, so no replacement entry exists and the `sub_800015e` phantom must be KEPT — the pairing invariant that takes corpus bodies-destroyed from 531 to 0); and **SPLIT** (`G`@`0x08000168`'s literal resolves onto `F`@`0x08000170`'s own first word, which the Listing never decoded, so the entry moves 4 bytes in to `sub_8000174` and loses `movs r0,#7 ; movs r1,#8` — the single disclosed residue of the corpus measurement, pinned as current behaviour). `kuna-console/tests/verify_poolentry.rs` is the two-pass e2e over all three, plus an `aif off` inertness pin. No cross toolchain on this host emits a bare-metal STM32 link layout, hence the byte-by-byte generator |
| `cortexm_tailcall_le32` | hand-assembled, **stripped** bare-metal ARM Cortex-M ELF32 (437 bytes; generator `cortexm_tailcall_le32.py`, regenerate with `python3 cortexm_tailcall_le32.py`) | **tail-call function entries** (`listing::kuna_tailcallentry`, `--option tailcallentry on`, default-off). Its vector table is the *shipped*-signature shape (`.isr_vector`@`0x08000000` is `AX` in a `PF_X` load, `word[0]` = `0x20008000`, `word[1]` = `e_entry` = `0x08008001`), so nothing here depends on `cortexmvectors`. `.text`@`0x08008000` holds one genuine tail call plus the three near-miss shapes the containment model must keep rejecting — every one of the four is reached ONLY by an unconditional `B`, so the naive rule takes all four. `TAIL`@`0x08008020` (`movs r0,#0x2a ; bx lr`) is branched to from `_start`@`0x08008000` across the discovered entry `helper`@`0x08008010`, so it crosses a function boundary and is **accepted**; `.Lbody`@`0x08008038` stays inside `loopfn`@`0x08008030`'s own entry-ordered region (the rotated-loop-head case) and is rejected; `EPI`@`0x08008058` opens `pop {r4,pc}` (a shared epilogue) and is rejected; `SPIN`@`0x08008060` (`movs r0,#0 ; b .`) never terminates and is rejected. `kuna-console/tests/verify_tailcallentry.rs` is the two-pass e2e: default registers five functions and emits `TAIL`'s body inside `sub_8008000`; with the option on `sub_8008020` registers and decompiles to `return 0x2a;` while the other three stay undiscovered. No cross toolchain on this host emits a bare-metal STM32 link layout, hence the byte-by-byte generator |
| `funcstart_patterns_x86_64` | **stripped**, statically-linked **x86-64** ELF, `gcc -O2 -fno-asynchronous-unwind-tables -fcf-protection=none -no-pie -fno-pic -fno-stack-protector` (source `funcstart_patterns_x86_64.c`) | the **full byte-pattern function-start** pass (`s1_entry::FuncStartPatternPass`, `--option funcstart_patterns on`, default-off): a `static` helper `widget`@**`0x401130`** has the prologue `push rbx; mov rbx,rdi` (`53 48 89 fb`) preceded by an 8-byte NOP pad (`0f 1f 84 00 00 00 00 00`). That is the FULL upstream `<patternpairs>` postpattern `0x534889fb` (PUSH RBX; MOV RBX,RDI) gated by the NOP prepattern `0x0f1f840000000000` — but it is **not** one of the three bare x86-64 prologues the always-on minimal oracle (`entry_disc` oracle 5) ports, and `widget` carries **no symbol** (stripped, `static`), **no `.eh_frame` FDE** (`-fno-asynchronous-unwind-tables`), and is not `e_entry`/INIT/FINI/`main`. So `widget` is discoverable **only** via the full pattern set: `kuna-console/tests/verify_funcstart_patterns.rs` asserts `sub_401130` is found + decompilable with `--option funcstart_patterns on` and **NOT** registered by default. The other helper `ext`@`0x401170` is `T`/global (stripped). Pinned VMAs read from the un-stripped build's `nm` (`widget`=`0x401130`, `ext`=`0x401170`, `main`=`0x401020`). |
| `aif_gap_x86_64` | **STRIPPED** dynamic PIE x86-64 (`-O0`), no unwind tables (source `aif_gap_x86_64.c`) | Aggressive Instruction Finder gap-walk (`s1_aif`, the third Listing/xref consumer; the kuna analog of Ghidra's `AggressiveInstructionFinderAnalyzer`, **gated off** by default). 24 handlers `h0..h23` are called DIRECTLY from `main` (`sub_13c9`, recovered via the PIE `_start`→`main` `lea rdi,[rip+main]` idiom), so the recursive-descent Listing walk reaches them — clearing Ghidra's `MINIMUM_FUNCTION_COUNT` (20, here `function_count`=33) — and their identical `push rbp; mov rsp,rbp; mov edi,-0x14(rbp); …` prologue stocks the function-start fingerprint histogram (one bucket shared by 25 functions, ≥ the acceptance threshold 4). `hidden_handler`@`0x13ae` is the gap target: it is in **no** symbol table (stripped), has **no** `.eh_frame` FDE (built `-fno-asynchronous-unwind-tables`), and is **never** the target of a static CALL — its address lives ONLY in the const `.rodata` function-pointer `table`@`0x3df0` (slot 1=`0x3df8`, an `R_X86_64_RELATIVE` reloc → `0x13ae`), which `main` indexes with a `volatile` (unfoldable) value and calls via `call *reg`. So entry-disc + funcsyms + the static walk all miss it (`main` renders the call as `(**(code **)(…0x3df0))(…)`, unresolved). With `--option listing on --option aif on`, AIF's gap-walk fingerprint-matches `hidden_handler`'s prologue + valid-subroutine-checks it (a clean `ret`, 11 instructions) and emits it as a discovered entry → `sub_13ae`, decompilable by name. Default (off) leaves it undiscovered (byte-identical parity). Drives `kuna-console/tests/verify_aif.rs` |
| `alignednew_x86_64` | tiny non-PIE x86-64 ET_EXEC built `-nostdlib -static` from hand-written asm (source `alignednew_x86_64.s`), not stripped | the **forward** direction of call-site argument reconciliation (`option calleearityfwd`, DIV-103, P4; e2e `kuna-cli/tests/decompile_cli.rs`, stage `tests/stages/kuna-calleearityfwd.xml`, promoted probe `tests/cli/argument-recovery-knobs-still.json`). MSVC's aligned `operator new` shape on SysV: `caller`@`0x401010` calls `callee`@`0x401000` from BOTH arms of `cmp $0x1000,%rdi`. The large arm writes a fresh `rdi` (`lea 0x27(%rdi),%rax; cmp %rdi,%rax; jbe bail; mov %rax,%rdi; call`) and keeps its argument; the small arm passes `rdi` live-in (`test %rdi,%rdi; jz zero; call`), so `Funcdata::only_op_use` rejects the trial on the guard's `CPUI_CBRANCH` and the argument is dropped. The small arm is laid out SECOND and reached by a forward branch, which is what puts its call spec FIRST in `qlst` order: `calleearity` (which reconciles only against an already-final sibling) has no witness yet and declines, and only the end-of-pass retry rescues it. Default renders `callee(a0)` / `callee(a0 + 0x27)`; `--option calleearityfwd off` and `--option calleearity off` both restore `callee();`. Hand-written asm because no compiler emits this shape without a libc allocator behind it |
| `covercopy_x86_64` | non-PIE x86-64, `gcc -O0 -no-pie -fno-pic -fno-stack-protector`, not stripped (source `covercopy_x86_64.c`) | the two **P6 Cover-extension miscompilations** (`kuna-console/tests/verify_cover_miscompile.rs`, DIV-47). `lookup_service` has three `return name;` guards sharing one `-O0` epilogue with a `lookup()` call clobbering the return register in between — pins that the reload `vN = a0;` on the lookup-failed path is emitted (`Merge::checkCopyPair`'s dominance range needs `addRefPoint`, `merge.cc:1121`; without it the emitted C returns NULL where the binary returns the parameter). `two_selects` has two `cond ? g_step : 0` phis both inlined into one `emit(...)` argument — pins that they stay two variables (`Merge::markImplied` must dirty its operands' Covers, `merge.cc:1595-1605`, and a Varnode `coverdirty` must reach its HighVariable, `varnode.cc:377-378`; without it the argument subtracts the second select twice). Both assertions are on the VALUE-carrying statement, not a line count |
| `hostile_size_low32_x86_64` / `hostile_size_neg_x86_64` / `hostile_size_sane_x86_64` | three byte-identical non-PIE x86-64 programs (`gcc -no-pie -nostdlib -e main`, sources beside them) differing ONLY in the `st_size` of the data symbol `g_a`@`0x402000`: `0x100000000`, `0xfffffff0`, and `8` | the **symbol-extent clamp** (GH-339, `kuna-console/tests/verify_hostile_symbol_sizes.rs`). `st_size` is a 64-bit ELF field no header check validates, and arm 4a of `commit_analysis_output` narrows it to the type factory's `int4`. Narrowing BEFORE the clamp let two classes through: low-32-zero truncated to a size-0 type, which `add_symbol_internal` rejects — and because the commit applies its arms in place with `?`, that one symbol aborted the WHOLE commit (`kuna functions` exited 1 with nothing, and the stash is `mem::take`n so a retry commits nothing); sign-bit-set truncated to a NEGATIVE size that indexed the type factory's caches out of bounds and aborted the PROCESS (exit 101). All three fixtures must now load with `g_a` named. The sizes come from the assembler (`.size g_a, …`) — nothing is byte-patched after the link, so they rebuild reproducibly. Neither parity corpus can cover this: both are symbol-less bytechunks that never construct an `ObjectLoadImage` |
| `switchtable_i386` / `switchtable_x86_64` | two tiny non-PIE ELF `dispatch` routines built `gcc -nostdlib -no-pie -Wl,-Ttext=0x100000 -e dispatch` from hand-written asm (sources `switchtable_i386.s` / `switchtable_x86_64.s`), 9 KB each | **jump-table following** in the on-demand xref walk (`listing/kuna_switchtable.rs`; e2e `kuna-console/tests/verify_switchtable.rs`, CLI probe `tests/cli/string-ownership-misses-literal.json`). Each is the reduction of crackmes.one/60be2ad433c5d410b8842c95, whose window procedure dispatches `JMP dword ptr [EAX*0x4 + 0x4017c4]` and whose case bodies — and the literals they push — were invisible to `kuna xrefs` / `kuna strings`. Four cases push a distinct literal and the default arm pushes a fifth; the default arm is reached by the `JA`, so it was always attributed and is the control. The two differ in table stride (`.long` / `.quad`) and in how the literal is materialized (`PUSH imm32` / RIP-relative `LEA`). VMAs pinned in the e2e: `dispatch`@`0x100000` and the table@`0x101000` in both; the dispatch is `0x100009` (i386) / `0x100007` (x86-64) |

Provenance: `fauxware`, `cet_pie_x86_64`, `stripped_dynamic_x86_64` copied
verbatim from `bs-artifacts/binaries/` (`fauxware`, `debug_symbol`,
`debug_symbol_mod_stripped` respectively). `cpp_mangled_x86_64` was built locally
with `g++ -O0 -no-pie -fno-pic` from a tiny `namespace foo { struct Bar { void
baz(int); }; } void foo::Bar::baz(int){...} int main(){...}` source.
`entry_selectors_x86_64.o` is project-authored synthetic assembly under the
repository's Apache-2.0 license. It is reproducible with
`as --64 -o entry_selectors_a_x86_64.o entry_selectors_a_x86_64.s`, the matching
command for `entry_selectors_b_x86_64.s`, then
`ld -r -o entry_selectors_x86_64.o entry_selectors_a_x86_64.o
entry_selectors_b_x86_64.o`; the two intermediate objects are not retained.
`et_rel_status_arm.o` and `et_rel_status_aarch64.o` are project-authored
synthetic assembly under the repository's Apache-2.0 license. Regenerate them
with `arm-linux-gnueabi-as -o et_rel_status_arm.o et_rel_status_arm.s` and
`aarch64-linux-gnu-as -o et_rel_status_aarch64.o et_rel_status_aarch64.s`.
These two committed objects provide the end-to-end ARM/AArch64 status-return
proof. In-memory relocation and layout tests cover the complete supported
ARM/AArch64/PowerPC64 and generic-width matrix, REL/RELA addends, both byte
orders, local and external targets, interworking, bounds/range/alignment errors,
missing TOCs, malformed encodings, and bounded diagnostic aggregation; no
proprietary object is part of the regression suite.
`cpp_noreturn_x86_64`: `g++ -O0 -no-pie -fno-pic -o cpp_noreturn_x86_64
cpp_noreturn_x86_64.cpp` (source vendored alongside) — a `fail()` that tail-calls
`std::terminate()` plus a `throw` (→ `__cxa_throw`); both are mangled no-return
`.dynsym` imports the demangle pass renames, so they verify the address-resolved
no-return commit. `cppproto_x86_64` (24408 bytes, source vendored alongside as
`cppproto_x86_64.cpp`): `g++ -O0 -g -no-pie -fno-pic -o cppproto_x86_64
cppproto_x86_64.cpp`. `-O0` keeps every member function out of line (so each
definition DIE really does carry only `DW_AT_specification`), `-g` keeps
`.debug_info`, and `-no-pie` fixes the VMAs pinned above.
`cppsig_x86_64.so` (source vendored alongside as `cppsig_x86_64.cpp`): `g++ -O0
-shared -fPIC -fno-inline -o cppsig_x86_64.so cppsig_x86_64.cpp` then `strip
--strip-all cppsig_x86_64.so`. A SHARED library so the mangled names survive in
`.dynsym`, `--strip-all` so nothing else does, and `-fno-inline` so every body
stays reachable and distinct. `sig::combine` deliberately calls no member
function: an intra-library call to an exported member emits a PLT stub carrying
the same mangled name, and `load function <name>` would then resolve to the stub.
`itaniumrtti_x86_64.so` (source vendored alongside as `itaniumrtti_x86_64.cpp`):
`g++ -O0 -g0 -fPIC -shared -fvisibility=hidden -fvisibility-inlines-hidden -o
itaniumrtti_x86_64.so itaniumrtti_x86_64.cpp` then `strip --strip-all
itaniumrtti_x86_64.so`. A SHARED library so `.rela.dyn` keeps the undefined
`_ZTVN10__cxxabiv1*_type_infoE` relocations that anchor the whole recovery,
`-fvisibility=hidden` so no class method leaks into `.dynsym` (the two `probe_*`
entry points carry an explicit `visibility("default")` attribute), and
`--strip-all` so nothing else survives. Addresses are NOT pinned: the tests
assert on recovered NAMES, which is what the feature produces.

`eh_lsda_x86_64` (14744 bytes, source vendored alongside as
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

`rust_scalarpair_x86_64` (2088 bytes, source vendored alongside as
`rust_scalarpair_x86_64.rs`): built with the same rustc 1.90.0 as
`rust_hello_x86_64` —
`rustc -C opt-level=2 -C panic=abort -C relocation-model=static -C link-args=-nostartfiles --edition 2021 rust_scalarpair_x86_64.rs -o rust_scalarpair_x86_64`.
`-C relocation-model=static` is load-bearing twice: it makes `cons`'s call to
`prod` a **direct** `e8 rel32` (a PIE cdylib routes it through the GOT, which no
`<bytechunk>` can reproduce) and it fixes the VMAs, so the stage testcase can
embed the bytes at their real addresses. `_start` reads a `static mut` through
`read_volatile` and stores the result through `write_volatile`, which is the
cheapest way to stop `-C opt-level=2` from constant-folding the whole program
away — without it the linker emits a two-byte `jmp .` and nothing else.

`rust_clobber_pair_x86_64` (1800 bytes, source vendored alongside as
`rust_clobber_pair_x86_64.rs`): same rustc 1.90.0 and the same
`-C relocation-model=static` reason as `rust_scalarpair_x86_64` — a direct
`e8 rel32` call and fixed VMAs, so the stage testcase can embed the bytes. Both
functions are `global_asm!` with explicit `.type`/`.size` directives, without
which the linker emits them as zero-sized `NOTYPE` symbols and kuna does not see
them as functions at all.

`dwarfvariants_x86_64` (10560 bytes, source vendored alongside as
`dwarfvariants_x86_64.rs`): built with the same rustc 1.90.0 as the fixtures
above but with **debug info on**, which is the whole point --
`rustc -C opt-level=1 -C debuginfo=2 -C relocation-model=static -C panic=abort
-C link-arg=-nostartfiles -C link-arg=-static dwarfvariants_x86_64.rs -o
dwarfvariants_x86_64`. `-C debuginfo=2` is what emits the `DW_TAG_variant_part`
DIEs; without it a Rust binary carries no type DIEs at all and `option
dwarfvariants` recovers nothing (this is stated as a limitation, not engineered
around). `-C opt-level=1` rather than `2` keeps each `#[inline(never)]` function
a recognisable one-shape body; `-C relocation-model=static` fixes the VMAs so the
stage testcase can name addresses. `#![no_std]` + `-C link-arg=-nostartfiles`
keeps the image at 10 KB with the full DWARF still present.

`dwarfvariants_overlay_x86_64` (8256 bytes, source vendored alongside as
`dwarfvariants_overlay_x86_64.rs`): same recipe and same reasons as
`dwarfvariants_x86_64` above --
`rustc -C opt-level=1 -C debuginfo=2 -C relocation-model=static -C panic=abort
-C target-feature=+crt-static -C link-arg=-nostartfiles
dwarfvariants_overlay_x86_64.rs -o dwarfvariants_overlay_x86_64`. It exists as a
SECOND fixture rather than extra functions in the first because the first one's
VMAs are pinned by the stage testcase and by this table.

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

`arm_thumb_switch_le32` (1304 bytes, source vendored alongside as
`arm_thumb_switch_le32.c`): the jump-table sibling of `arm_thumb_linked_le32`,
built the same way **in the `kuna-dev` container** with
`arm-linux-gnueabihf-gcc -mthumb -Os -static -nostdlib -e _start arm_thumb_switch_le32.c -o arm_thumb_switch_le32`.
`-Os` is what makes gcc pick the dense `tbb [pc,r0]` form (a `-O0` build lowers
the same `switch` into a compare cascade with no table), and the four
`__attribute__((noinline))` leaf helpers keep a real `bl` inside the
table-reachable case blocks. That combination — a recovered jump table plus an
injected user-op inside the blocks only the table reaches — is the shape that
exposed the P2 injection-drain gap; the stage testcase
`tests/stages/ghdec-isamode-inject.xml` loads this file and asserts the emitted
C carries no `setISAMode`.

`mcount_x86_64`: `gcc -pg -static -O0 -o mcount_x86_64 t.c` (t.c = `int
main(){return 0;}`), then `strip --strip-debug` (drops `.debug_*` but keeps
`.symtab`, so the `mcount`/`__fentry__`/`main` FUNC symbols survive). It is
**static** on purpose: a dynamic `-pg` build resolves `mcount` to an *indirect*
GOT call (`call *0x…(%rip)`), which has no named-`mcount` FunctionSymbol at the
call target, so the name-matched fixup cannot bind — only the static build emits a
direct `call mcount` to a real `mcount` FUNC symbol. Static glibc makes this
fixture larger (~896 KB) than the others; that size is the unavoidable cost of a
self-contained direct-`call mcount` target.

`alignednew_x86_64` (source vendored alongside as `alignednew_x86_64.s`): built
locally with
`gcc -nostdlib -static -no-pie -Wl,-Ttext=0x401000 -o alignednew_x86_64 alignednew_x86_64.s`
and NOT stripped, so `callee` / `caller` / `_start` are `STT_FUNC` and the probe
can select `caller` by name. The two things that must not drift if it is ever
rebuilt: `callee` has to be reached from BOTH arms of the size test (one witness,
one loser), and the losing arm has to be the one laid out SECOND and entered by a
forward branch, because that is what makes its call spec finalize first and so
puts it out of `calleearity`'s reach. VMAs: `callee`=`0x401000`,
`caller`=`0x401010`, `_start`=`0x401050`.

`aif_gap_x86_64` (source vendored alongside as `aif_gap_x86_64.c`): built locally
with
`gcc -O0 -fpie -pie -fcf-protection=none -fno-stack-protector -fno-asynchronous-unwind-tables -fno-unwind-tables -o aif_gap_x86_64 aif_gap_x86_64.c`
then `strip aif_gap_x86_64`. The 24 `h0..h23` handlers share an identical prologue
(they differ only by an operand immediate, the operand-insensitive fingerprint
equivalence class) and are all called directly from `main` so the Listing walk
reaches them (≥ 20 functions); `hidden_handler` is referenced ONLY from the const
`.rodata` function-pointer `table` (an `R_X86_64_RELATIVE` reloc) and called
indirectly, so no oracle / static CALL reaches it — it is the AIF gap target. The
`-fno-asynchronous-unwind-tables -fno-unwind-tables` flags strip the `.eh_frame`
FDEs from the program functions (so the `.eh_frame` FDE entry oracle cannot find
`hidden_handler`); `-fcf-protection=none` keeps the prologues `endbr64`-free so the
fingerprint is the plain frame setup. The VMAs (`hidden_handler`=`0x13ae`,
`main`=`0x13c9`, `h0`=`0x1129`, `table`=`0x3df0`) are pinned by
`kuna-console/tests/verify_aif.rs`. **PIE** so the `_start`→`main`
`lea rdi,[rip+main]` idiom (`s1_entry` oracle 4) recovers `main` and seeds the walk.

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

`operand_refs_x86_64` (~15 KB, source vendored alongside as
`operand_refs_x86_64.c`): built with
`gcc -no-pie -fno-pic -mcmodel=large -fno-stack-protector -O0 -o operand_refs_x86_64 operand_refs_x86_64.c`,
kept **un**stripped so `main`/`mystery` resolve by name. The
**`-mcmodel=large`** is load-bearing: it forces gcc to materialize the `"hi"`
string address with a `movabs $0x402004,%rax` (a bare 64-bit immediate — the
address appears DIRECTLY in code), the exact case `ScalarOperandAnalyzer` reads as
a `Scalar` operand. Under the default small/medium model gcc `-O0` emits a
RIP-relative `lea 0xNNN(%rip)` instead, which computes the address as `pc +
displacement` (no bare scalar surfaces), so the pass would correctly find nothing —
faithful to Ghidra's `ADDRESSES_DO_NOT_APPEAR_DIRECTLY_IN_CODE` gate. `mystery` is
`__attribute__((noinline))` so it survives as a real `.text` function with **no
known prototype** (absent from the libproto table), and `"hi"` is 2 chars (< the
`StringLiteralPass` `min_len` 5) — so the `mystery("hi")` literal renders ONLY when
`operand_refs` types the operand, isolating this pass's contribution from
`s1_strings` + libproto. `main`@`0x40112e`, `mystery`@`0x401106`, the `"hi"` string
@`0x402004` (4-byte data prefix at `0x402000`). **Pin the VMAs as test consts**
(`nm`/`objdump -d`/`objdump -s -j .rodata`). Drives
`kuna-console/tests/verify_operand_refs.rs`.

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

`pe_noreturn_import.exe` (6173 B, PE32+/x86-64, source `pe_noreturn_import.c`) is the
PE **import-call binding** fixture (`--option peimportcall`, `tests/stages/ghdec-peimportcall.xml`).
Its point is the one call shape `pe_imports.exe` does not have: a *direct indirect*
`call [__imp_ExitProcess]` through an Import Address Table slot, forced with
`__declspec(dllimport)` (the shape MSVC emits for every Win32 call, and the shape kuna
could not resolve — the CALLIND target is the contents of a global, so `ActionDeindirect`
needs `Varnode::externref` on it). `bail`@`0x140001000` ends in that call; `tally`@`0x140001010`
is deliberately the next function in `.text` and deliberately contains a loop, so an overrun
past the unbound call is visible in one line of C; `entry`@`0x140001040` calls `bail` under a
condition, so the dead fall-through after the bound no-return call is visible too. ImageBase
`0x140000000`, IAT slot `0x140005038`, MinGW `FF 25` veneer `0x140001070`. Built with
MinGW-w64 in the `kuna-dev` container (the same toolchain as `pe_imports.exe`):

```bash
docker run --rm -v "$PWD":/w -w /w kuna-dev bash -lc \
  'x86_64-w64-mingw32-gcc -O1 -nostdlib -Wl,-e,entry \
     decompiler/crates/kuna-analysis/tests/fixtures/pe_noreturn_import.c \
     -o decompiler/crates/kuna-analysis/tests/fixtures/pe_noreturn_import.exe -lkernel32'
```

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

`msvc_rtti_x64.exe` (3584 B, PE32+/x86-64) and `msvc_rtti_x86.exe` (3072 B, PE32/x86)
are **linked Windows PEs carrying the real MSVC C++ RTTI / vftable ABI** in
`.rdata`/`.data`, for the MSVC RTTI class-name recovery gate
(`kuna-console/tests/verify_rtti.rs`, the `s1_rtti` pass, `--option rtti on`). Both
are the same source `msvc_rtti.cpp` (one polymorphic base class `Shape` + one
derived class `Box` with a virtual method) linked for two arches — proving the
recovery is arch-independent (x64 = image-base-relative `IBO32` refs + RTTI0 name at
offset 16; x86 = raw-VA refs + name at offset 8). `cl.exe` is unavailable on Linux,
but `clang -target {x86_64,i686}-pc-windows-msvc -fuse-ld=lld` emits the *same* MSVC
C++ RTTI ABI (the real `CompleteObjectLocator` / RTTI{0..3} bytes in `.rdata`,
verified by `objdump -s -j .rdata`), so these are **real** RTTI PEs, not hand-faked
tables. The `msvc_mangled.obj` recipe already proved `clang` emits the MSVC C++ ABI;
a *linked* PE with a populated `.rdata` is the new need — supplied by `-fuse-ld=lld`
(`lld-link`) + a one-cell inline-asm stub for the CRT `type_info` vftable
(`??_7type_info@@6B@`) the RTTI Type Descriptors reference, so the image links
freestanding (`-nostdlib`) while keeping the genuine RTTI bytes. Built in `kuna-dev`
(no new packages — `clang` + `lld-link` ship in the image):

```bash
docker run --rm -v "$PWD":/w -w /w kuna-dev bash -lc '
  F=decompiler/crates/kuna-analysis/tests/fixtures
  clang -target x86_64-pc-windows-msvc -fuse-ld=lld -O1 -nostdlib \
    -Wl,-subsystem:console -Wl,-entry:mainCRTStartup \
    $F/msvc_rtti.cpp -o $F/msvc_rtti_x64.exe
  clang -target i686-pc-windows-msvc   -fuse-ld=lld -O1 -nostdlib \
    -Wl,-subsystem:console -Wl,-entry:mainCRTStartup \
    $F/msvc_rtti.cpp -o $F/msvc_rtti_x86.exe'
```

Pinned VMAs (from `x86_64-w64-mingw32-objdump -s -j .rdata/.data -d -j .text` + `-p` ImageBase):

| | ImageBase | Box `TypeDescriptor` (RTTI0) | Shape `TypeDescriptor` | Box `CompleteObjectLocator` (RTTI4) | Box vftable | Box vftable slot 0 (`Box::area`) |
|---|---|---|---|---|---|---|
| **x64** | `0x140000000` | `0x140003010` (`.?AUBox@@`) | `0x140003030` (`.?AVShape@@`/`.?AUShape@@`) | `0x140002020` | `0x140002010` | `0x140001040` |
| **x86** | `0x400000` | `0x403010` (`.?AUBox@@`) | `0x403030` (`.?AUShape@@`) | `0x402010` | `0x40200c` | `0x401030` |

With `--option rtti on` the recovery labels the Box `TypeDescriptor`
`Box::RTTI_Type_Descriptor`, the COL `Box::RTTI_Complete_Object_Locator`, the vftable
`Box::vftable`, and the Shape `TypeDescriptor` `Shape::RTTI_Type_Descriptor` — so
`Box`/`Shape` surface as recovered C++ class names; default-off (`rtti off`) they are
absent (the parity proof). The `.?A…@@` names demangle through the existing MSVC
demangler via the Ghidra `RttiUtil` `??_R0…@8` wrap (clang renders `struct` classes
as `.?AU…`; both `V`/`U` recover the bare name).

**vftable discovery + virtual-method naming (R3).** Each recovered class's vftable is
walked from its `Box::vftable` base (`VfTableModel.getVfTableCount`), bounding the slot
array at the first NULL / non-`.text` slot. The Box vftable holds exactly one slot —
`Box::area` (`return side*side;`, the pinned slot-0 target above: `0x140001040` x64 /
`0x401030` x86) — which R3 names `Box::vfunc_0` (a `SymKind::Function`) and marks the
slot array read-only. The slots are **absolute VAs on both arches** (NOT the `IBO32`
displacements the COL/RTTI inter-struct refs use): the x64 vftable cell at `0x140002010`
holds the full 8-byte `0x140001040`, the x86 cell at `0x40200c` the 4-byte `0x401030`.
`kuna-console/tests/verify_rtti.rs` asserts `Box::vfunc_0` exists AND a function symbol
resolves at the slot-0 target VA (the virtual dispatch now points at a named method),
absent with `rtti off`. The slot function's stem is `vfunc_`, never `vftable_`: only the
table data object wears a `vftable` name, so a function inventory never reports an
executable range as a vtable.

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
  Rebuild: the Python snippet in `Increment 45` of the retired analysis-port log (git history)
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

## Mach-O Objective-C metadata (the `s1_objc` headline)

`macho_objc` (x86-64, ~16 KB) is a self-contained Objective-C Mach-O for the
ObjC metadata-recovery gate (`kuna-console/tests/verify_objc.rs`, the kuna
analog of Ghidra's `ObjcTypeMetadataAnalyzer`). Source `macho_objc.m` uses a
**root class** (`objc_root_class`) so it needs **no macOS SDK / Foundation** —
bare `clang` synthesizes the `__objc_*` metadata from the `@interface`/
`@implementation` alone:

```objc
__attribute__((objc_root_class)) @interface Greeter @end
@implementation Greeter
- (int)greet:(int)n { return n*3+7; }
@end
int main(){ return 0; }
```

Built in the `kuna-dev` container (or on a host with `clang` + the rustup
`ld64.lld`) with the **exact `macho_imports` recipe**, plus `-x` (strip local
symbols) so the IMP `-[Greeter greet:]` has NO leftover symbol — only the
`__objc_*` metadata recovers the name:

```bash
clang -target x86_64-apple-macos11 -fobjc-arc -O1 -c macho_objc.m -o m.o
LLD=$(rustc --print sysroot)/lib/rustlib/$(rustc -vV | sed -n 's/host: //p')/bin/gcc-ld/ld64.lld
"$LLD" -arch x86_64 -platform_version macos 11.0 11.0 \
       -undefined dynamic_lookup -x -e _main -o macho_objc m.o
```

ImageBase `0x100000000` (PIE). The metadata chain the pass walks:
`__DATA_CONST,__objc_classlist[0]` → `class_t`@`0x100003000` →
(`data & ~0x7`) `class_ro_t`@`0x100003098` → `.name`=`"Greeter"`,
`.baseMethods` → the **small/relative** `method_list_t`@`0x10000066c`
(`entsizeAndFlags=0x8000000c`, count 1) → `method_t` selector `"greet:"`
(via a selref), types `"i20@0:8i16"`, **IMP**@`0x100000640`. The metaclass
(`isa`@`0x100003028`) has no `+` methods. **Pinned VMAs** (`llvm-objdump
--macho -d` / a manual Mach-O parse): IMP `-[Greeter greet:]`@`0x100000640`,
`class_t Greeter`@`0x100003000`, `class_ro_t`@`0x100003098`,
`method_list_t`@`0x10000066c`. With `--option objc on` the IMP renders
`-[Greeter greet:]`; off, it is `sub_100000640`. x86-64, **no chained fixups**
(the clang on this toolchain emits classic `LC_DYLD_INFO_ONLY` rebase opcodes,
like `macho_imports`) — so this slice is also the **no-op proof** for the
chained-fixup resolver (the resolver yields an empty overlay here, and `read_ptr`
reads raw section words exactly as before).

### `macho_objc_arm64` — the chained-fixup + arm64 slice (PR-O0 + PR-O2)

`macho_objc_arm64` (arm64, ~49 KB) is the **same `macho_objc.m` source** built for
arm64 **with a real `LC_DYLD_CHAINED_FIXUPS`** — the prerequisite for arm64 ObjC.
The only build-recipe change vs the x86-64 slice is the `-arch arm64` target and
the **`-fixup_chains`** linker flag, which makes `ld64.lld` emit chained fixups
(`LC_DYLD_CHAINED_FIXUPS` + `LC_DYLD_EXPORTS_TRIE`) instead of the classic
`LC_DYLD_INFO_ONLY` rebase opcodes:

```bash
clang -target arm64-apple-macos11 -fobjc-arc -O1 -c macho_objc.m -o m.o
LLD=$(rustc --print sysroot)/lib/rustlib/$(rustc -vV | sed -n 's/host: //p')/bin/gcc-ld/ld64.lld
"$LLD" -arch arm64 -platform_version macos 11.0 11.0 -fixup_chains \
       -undefined dynamic_lookup -x -e _main -o macho_objc_arm64 m.o
```

Confirm it carries the chained fixups: `llvm-otool -l macho_objc_arm64 | grep
CHAINED` (or a load-command dump shows `LC_DYLD_CHAINED_FIXUPS dataoff=0xc000
datasize=0x80`). ImageBase `0x100000000` (PIE), `DYLD_CHAINED_PTR_64` (format 2,
4-byte stride). The metadata chain (read through the resolver): `__DATA_CONST,
__objc_classlist[0]` (a chained-fixup slot resolving to) → `class_t`@`0x100008000`
→ (`data & ~0x7`) `class_ro_t`@`0x100008098` → `.name`=`"Greeter"`, `.baseMethods`
→ the small/relative `method_list_t`@`0x100000618` → selector `"greet:"`, types
`"i20@0:8i16"`, **IMP**@`0x1000005f0`. **Pinned VMAs:** IMP@`0x1000005f0`,
`class_t`@`0x100008000`, `class_ro_t`@`0x100008098`, classlist slot@`0x100004000`
(raw word `0x0000000100008000`, resolves to `0x100008000`); `class_t.isa`
slot@`0x100008000` (raw word `0x0020000100008028`, **resolves to `0x100008028`** —
the raw word would be garbage without the resolver, since the `next=4` field leaks
into the high bits). With `--option objc on` the IMP renders `-[Greeter greet:]`;
off, it is `sub_1000005f0`.

The resolver (PR-O0, `s1_loader/format/macho/chained.rs`) handles plain rebase
(`DYLD_CHAINED_PTR_64`/`_64_OFFSET`) + arm64e auth-rebase
(`DYLD_CHAINED_PTR_ARM64E`/`_USERLAND`, PAC bits stripped); **bind/import-ordinal
chains are out of scope** (an external symbol's runtime address is unknown
statically, so a bind slot is left unresolved — the consumer reads the raw word
and falls back, never a wrong address). The container's in-tree `ld64.lld` emits a
`DYLD_CHAINED_PTR_64` (format 2) arm64 fixture; the arm64e auth-rebase path is
covered by the resolver's synthetic-bit-pattern unit tests
(`decode_arm64e_auth_rebase_strips_pac` et al.) since the in-container linker does
not emit an arm64e auth-fixup slice.

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

`funcstart_patterns_x86_64` (source vendored alongside as
`funcstart_patterns_x86_64.c`): built + stripped with

  ```
  gcc -O2 -fno-asynchronous-unwind-tables -fcf-protection=none \
      -no-pie -fno-pic -fno-stack-protector \
      funcstart_patterns_x86_64.c -o funcstart_patterns_x86_64
  strip funcstart_patterns_x86_64
  ```

  The `-fno-asynchronous-unwind-tables` drops the helpers' `.eh_frame` FDEs (so the
  entry-discovery FDE oracle does not find them), `-fcf-protection=none` drops the
  ENDBR64 prefix (so the prologue is the bare `push rbx; mov rbx,rdi` shape), and
  `static` + `strip` removes every symbol for `widget`/`ext`. The `widget` helper's
  `-O2` prologue is exactly `push rbx; mov rbx,rdi` (`53 48 89 fb`) at the
  16-aligned `0x401130`, immediately preceded by gcc's 8-byte inter-function NOP
  pad `0f 1f 84 00 00 00 00 00`. That pair is the FULL upstream x86-64gcc
  `<patternpairs>` (postpattern `0x534889fb`, prepattern `0x0f1f840000000000`) but
  not a minimal-oracle shape, so `widget` is recovered ONLY by
  `--option funcstart_patterns on`. Pinned VMAs (read from the un-stripped build's
  `nm`): `widget`=`0x401130`, `ext`=`0x401170`, `main`=`0x401020`.

## PE CodeView / PDB debug record (s1_pdb PR-P0)

`pdb_min.exe` (x86-64 PE, ~2.5 KB) is a **PE carrying a CodeView/RSDS debug
record** for the PDB CodeView extractor gate
(`kuna-analysis::s1_pdb::codeview::tests::extract_pdb_min_exe_rsds_record`). When
`clang -gcodeview` builds a PE, `lld-link` writes an RSDS record (PDB GUID + age)
plus the `.pdb` path into the PE's `IMAGE_DIRECTORY_ENTRY_DEBUG` directory — the
fingerprint a later PDB-consuming pass uses to find + gate the external `.pdb`.
This fixture carries only that *record*; the matching `.pdb` is **not** needed for
PR-P0 (it lands with the PR-P1 pass). Built freestanding (own entry, no CRT) so it
links with `clang`/`lld-link` on Linux without the MSVC CRT libs (`kuna-dev`):

```bash
# (run from a clean dir; /pdbaltpath keeps the recorded path a bare filename)
clang -target x86_64-pc-windows-msvc -g -gcodeview -fuse-ld=lld -nostdlib \
      -Xlinker /entry:mainCRTStartup -Xlinker /subsystem:console \
      -Xlinker /pdbaltpath:pdb_min.pdb \
      pdb_min.c -o pdb_min.exe   # then discard the emitted pdb_min.pdb
```

`pdb_min.c` = `int add(int a,int b){return a+b;} int mainCRTStartup(void){return add(2,3);}`
(`mainCRTStartup` is the freestanding entry, so no CRT/`main` is needed). The RSDS
record, confirmed via `llvm-readobj --coff-debug-directory pdb_min.exe`:
GUID (raw 16 bytes) = `63 39 AC 61 48 FF 24 90 4C 4C 44 20 50 44 42 2E`
(canonical text `61AC3963-FF48-9024-4C4C-44205044422E`, the Microsoft mixed-endian
form), Age = `1`, PDBFileName = `pdb_min.pdb`. The GUID is content-hash-derived, so
**a rebuild produces a different GUID** — pin the checked-in binary's values as
test consts (re-read with `llvm-readobj` if you ever rebuild it).

## PE + matching PDB — the PDB-consuming pass (s1_pdb PR-P1)

`pdb_prog.exe` (x86-64 PE, ~2.5 KB) **plus its matching `pdb_prog.pdb`** (~72 KB)
is the end-to-end fixture for the PDB-consuming pass (`s1_pdb::PdbPass`, `--option
pdb on`), the kuna analog of Ghidra's `PdbUniversalAnalyzer` — the **stripped
`FUN_<addr>` → real name** recovery. When `clang -g -gcodeview` builds a PE,
`lld-link` writes BOTH the RSDS CodeView record (into the PE) **and** the matching
`.pdb` (the symbol+type streams). kuna's loader does not name functions from the
COFF symbol table, so the uniquely-named `pdb_demo_compute` is a stripped
`FUN_<addr>` *without* the `.pdb`; only the PDB `S_PUB32`/`S_GPROC32` stream
recovers it. Built freestanding (own entry, no CRT) so it links on Linux without
the MSVC CRT libs (`kuna-dev`):

```bash
# (run from a clean dir; /pdbaltpath keeps the recorded path a bare filename)
F=decompiler/crates/kuna-analysis/tests/fixtures
clang -target x86_64-pc-windows-msvc -g -gcodeview -fuse-ld=lld -nostdlib -O1 \
      -Xlinker /entry:mainCRTStartup -Xlinker /subsystem:console \
      -Xlinker /pdbaltpath:pdb_prog.pdb -Xlinker /debug \
      $F/pdb_prog.c -o $F/pdb_prog.exe      # lld-link also emits pdb_prog.pdb
```

`pdb_prog.c` defines `pdb_demo_compute(int,int)` (the distinctively-named function
the rename proves) + the freestanding entry `mainCRTStartup`. The fixture's pinned
values (read with kuna's own `s1_pdb::codeview` extractor + the `pdb` crate — the
`kuna-dev` image has no `llvm-readobj`):
- **ImageBase** = `0x140000000`; **`pdb_demo_compute` VMA** = `0x140001000` (RVA
  `0x1000`); `mainCRTStartup` VMA = `0x140001010`.
- CodeView/PDB **GUID** = `A192EC48-382A-DFBA-4C4C-44205044422E`, **Age** = `1`,
  PDBFileName = `pdb_prog.pdb` (the EXE record and the `.pdb`'s own
  `pdb_information().guid/age` agree — the fingerprint gate passes).

`pdb_prog_mismatch.pdb` (~72 KB, source `pdb_mismatch.c`, a *different* program so
its content-hash GUID differs — `3395B1A2-F530-116C-4C4C-44205044422E`) is the
**fingerprint-gate negative** fixture: supplied for `pdb_prog.exe`, its GUID does
NOT match the EXE's CodeView record, so the pass rejects it (no rename). The
matching `.exe` is not vendored (only the mismatched `.pdb` is needed). `verify_pdb.rs`
proves both: matching `.pdb` → `pdb_demo_compute`; mismatch `.pdb` → still
`FUN_*`. The GUID is content-hash-derived, so **a rebuild produces a different
GUID** — re-read both with the `s1_pdb` extractor + `pdb` crate and re-pin if you
rebuild.

NB: a `.pdb` (MSF container) has a minimum multi-stream page-table overhead, so
`pdb_prog.pdb` / `pdb_prog_mismatch.pdb` are ~72 KB each — the two PDBs are the only
fixtures over 32 KB (a PDB cannot be made smaller). All other fixtures are under 32
KB. **Pin load-bearing VMAs as test consts** (read via `objdump`/`readelf`/the
`s1_pdb` extractor at build time) — addresses shift across toolchains.

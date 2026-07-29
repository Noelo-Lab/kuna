# 01 — Program preparation (kuna-analysis)

```yaml
Anchors:
  - decompiler/crates/kuna-analysis/src
  - decompiler/crates/kuna-decomp/src/p1_partition
```

Everything in this chapter runs **before any function is decompiled**. The
`kuna-analysis` crate is kuna's port of the layer Ghidra keeps *outside* its C++
decompiler — the Java loader, the analyzer tier, and the Listing — rebuilt against
kuna's own symbol/type tables. Untagged prose in this chapter therefore describes a
port of a **Ghidra Java analyzer or loader** (named per pass), not of the C++
decompiler; `(angr)`, `(ida)`, and `(kuna)` mark the other lineages, matching each
pass's row in `decompiler/crates/kuna-decomp/phases.toml`. Every analyzer named
below **is** a settable option under its own name (`--option <id> on|off`) —
except `funcdisc_recursive`, which rides the `funcstart_patterns` flag;
defaults, symptoms, and flip guidance live in the generated catalog,
[`docs/options.md`](../options.md), and are not repeated here.

## 1.1 The tier contract

A program-prep analysis is an implementation of
`decompiler/crates/kuna-analysis/src/pass.rs (AnalysisPass)`: it declares the phase
it feeds (P0/P1, a few feeding back to P2), a stable `id()` that doubles as its
option name, and one method `run(&AnalysisCtx) -> AnalysisOutput`. The contract has
three load-bearing properties:

- **Pure and read-only.** A pass sees only the parsed object (`object::File`), the
  raw image bytes, the opened load image, the resolved `Architecture`, and (for
  Listing consumers, §1.6) the built Listing. It mutates nothing.
- **Additive and total.** A pass only ever contributes *more* knowledge — names,
  types, entries, flags — and never fails: a malformed section, an unknown magic, or
  an out-of-range offset yields an *empty* output, never an error or panic.
- **Facts, not effects.** The output is a flat struct of typed fact lists
  (`pass.rs (AnalysisOutput)`): function/data symbols, sized data globals, discovered
  entries plus an optional name overlay, no-return functions, no-fall-through call
  sites, read-only ranges, string literals, library prototypes, processor-context
  paints, tracked register values, call-fixup tags, DWARF stack locals, source-line
  comments, and FID renames. Merging two outputs is concatenation; deduplication is
  the committer's job.

The passes never touch the pipeline live, and the pipeline never calls an analyzer:
the two meet exactly once, at a commit seam. `decompiler/crates/kuna-console/src/engine.rs
(bootstrap_from_object)` runs every registered pass at `load file`
(`decompiler/crates/kuna-analysis/src/passes.rs (run_default_analyses_per_pass)`) and
**stashes** each pass's output keyed by its id. The commit happens later, at
`read symbols` (`engine.rs (commit_pending_analysis)`) — after the CLI's `option`
lines have been applied — so a disabled pass's already-computed facts are simply
dropped at the gate (`engine.rs (analysis_pass_enabled)`; an id with no registered
gate fails *open*, so a new pass runs by default). The stash is drained on commit, so
a second `read symbols` cannot double-commit.

`engine.rs (commit_analysis_output)` then installs the merged facts into the engine
once, each arm idempotent against the loader's own funcsym stream: a function fact
no-ops where `find_function` already resolves (a real `.symtab` name always beats a
discovered one), sized data globals and string symbols skip occupied addresses (the plain label arm does not), no-return facts resolve by
**address** first (`find_function_across_scopes` — stable across demangling, which
renames the funcsym before install) with a name fallback for imports, and rename
facts (FID, ObjC, PDB) pass a **label gate** (`engine.rs (is_generic_placeholder_name)`)
that only ever overwrites an engine `sub_*`/`func_*`/`FUN_*`/`LAB_*` placeholder. Two fact
kinds are not installed globally: DWARF stack locals are parked per function and
re-seeded into each freshly-rebuilt `Funcdata`'s `ScopeLocal` at decompile time (the
`map addr`/`seed_mapped_symbols` path), and the `error(nonzero,…)` call-site list is
stashed on the `Architecture` for the per-function flow override (§1.7).

Two timing consequences shape the tier. First, anything that must influence the
**loader itself** runs before any `option` line exists, so load-time gates are
bridged across the process by environment variables the CLI exports:
`KUNA_RELOC_OBJECTS` (`relocobjects`), `KUNA_I386_PIE_PLT` (`i386_pie_plt`),
`KUNA_MACHO_ARM64E` (`macho-arm64e`), `KUNA_MACHO_SLICE` (`--slice`). For those,
the option rows exist for discoverability while the live gate is the env var. The
external-artifact paths `kuna_fid_db` and `kuna_pdb_path` are different: they only
*locate* the artifact — the `fid`/`pdb` passes stay flag-gated at the deferred
commit (`decompiler/crates/kuna-console/src/engine.rs (analysis_pass_enabled)`). Second,
anything that must **decode instructions** cannot run at load at all — the engine's
loadimage is attached to the SLEIGH translator only *after* the load-time pass list
runs — so the Listing build, its consumers, and `operand_refs` are deferred to the
commit point too (§1.6).

The XML `<binaryimage>` datatest path never constructs an `ObjectLoadImage` and never
stashes an output, so the entire tier is structurally inert on the 675-assertion
parity oracle; only real binaries feel it.

## 1.2 Load image

`decompiler/crates/kuna-analysis/src/loadimage_object.rs (ObjectLoadImage)` is the
real-binary `LoadImage` backend — the substitution for upstream's GPL-licensed
BFD loader (`LoadImageBfd`), rebuilt on the permissive `object` crate with the C++
interface semantics preserved exactly: the same 512-byte read buffer, the same
containing-segment-else-closest-greater walk with gap zero-fill, and the same
"initial address unmapped → `DataUnavailError`" contract in `loadFill`. The mapping
unit is the ELF **`PT_LOAD` segment** (what the OS actually maps), not the BFD
section list. Where upstream returns a BFD target string for the Java side to
re-map, kuna resolves the SLEIGH language id directly off the object header
(machine + endianness + class → e.g. `x86:LE:64:default:gcc`). The loader's symbol
stream — defined FUNC symbols plus the resolved import stubs of §1.3 — is
`@VERSION`-stripped and demangled (§1.4) before each name is installed as a
`FunctionSymbol`, and the loader's read-only section ranges are applied to the
symbol-table property map eagerly at bootstrap (loader markup, not a gated pass):
they are what lets the printer prove a constant points into read-only memory and
render a string literal.

The **data** half of those same two symbol tables is read alongside the function
half (`loadimage_object.rs (data_symbols)`): every defined, named `STT_OBJECT`
entry with a non-zero `st_size`, deduplicated by address, `.symtab` before
`.dynsym`. Zero-size entries are dropped because the linker's section-boundary
markers (`__bss_start`, `_edata`, `_end`) are exactly the sizeless ones, and a
sizeless symbol would plant a name on the first byte of whatever object follows
it. Each surviving entry becomes a named `undefined<size>` global — the same
shape §1.4's DWARF data globals use, and for the same reason: a size-1 entry does
not contain a 4- or 8-byte access, so the printer's covering-symbol query would
miss and fall back to `dat_<addr>`. Naming what the symbol table names is not
optional and carries no flag, matching IDA Pro and Ghidra, which both name data
objects from the symbol table independently of any debug info.

Precedence is what makes this safe to add underneath the existing sources. The
loader's data symbols commit **last** (`engine.rs (commit_analysis_output)`),
after the DWARF globals and after the detected string literals, and each is
skipped where a function or a covering data symbol already sits. So a
DWARF-described global keeps its DWARF-recovered extent and a detected string
keeps its `char[N]` typelock; the loader arm only fills addresses neither source
reaches. That residue is the interesting one: a copy-relocated libc extern
(`optind`, `stdin`, `stdout`, `optarg`) has a real `.bss` address and a `.dynsym`
entry but no DIE in the program's own `.debug_info`, so nothing else could name
it. Relocatable objects are excluded — `elf_reloc` rebases only the function half
of the symbol table, so a `.o` keeps its previous behavior.

Format dispatch is by magic (`engine.rs (is_object_binary)`): ELF, thin or fat
Mach-O, PE (`MZ`, validated downstream by the typed PE parser), and bare COFF
objects recognized by a whitelisted leading `IMAGE_FILE_MACHINE_*` u16 — anything
else routes to the XML front-end (§1.8). Per-format knowledge is funneled through
one trait, `decompiler/crates/kuna-analysis/src/loader/format/mod.rs (ObjectFormat)`:
the compiler-model id (ELF → `gcc`/`default`, PE → `windows`, with a resolve-time
fallback retry to the arch default when the preferred model has no vendored spec),
the section-flag translation, import resolution (§1.3), and extra constant ranges
(the MIPS GOT). Two format specifics live above the trait:

- **ET_REL relocatables** (angr, `relocobjects`, default-on) — a `.o` has no
  program headers, so the faithful loader maps zero bytes and every lift fails.
  `decompiler/crates/kuna-analysis/src/loader/elf_reloc.rs (RelocLayout)` reproduces
  angr CLE's relocatable backend: lay each `SHF_ALLOC` section out above `0x400000`
  (`RELOC_BASE`, matching CLE so addresses line up with angr's), apply the
  `.rela.*` relocations (`R_X86_64_PC32/PLT32/64/32/32S`; an unhandled kind warns
  and skips, never miscompiles silently), rebase defined symbols, and bind each
  undefined extern to a synthetic call target in an extern area above the sections
  so calls render by name. The result feeds back into `ObjectLoadImage` as the same
  segments/sections/funcsyms triple the linked path produces.
- **Mach-O fat/arm64e** — a universal binary is peeled to one slice's bytes at a
  single canonical point before anything else parses it
  (`decompiler/crates/kuna-analysis/src/loader/macho_fat.rs (select_fat_slice)`;
  preference `--slice`/`--target`, else x86-64 → arm64 → first), so the loader,
  every pass, and the deferred-Listing stash all see the same thin slice. An
  arm64e slice selects the Apple-Silicon pointer-auth SLEIGH spec instead of
  generic v8A only under the `macho-arm64e` env gate
  (`decompiler/crates/kuna-analysis/src/loader/format/macho.rs (MACHO_ARM64E_ENV)`).
  Modern Mach-O pointer slots are chained-fixup entries, not pointers;
  `decompiler/crates/kuna-analysis/src/loader/format/macho/chained.rs (ChainedFixups)`
  parses `LC_DYLD_CHAINED_FIXUPS` into a VMA→resolved-pointer overlay (rebase and
  arm64e auth-rebase handled; bind entries deliberately absent, so a consumer
  misses and falls back rather than reading a wrong address).

## 1.3 Loader markup

Import naming exists because a CALL into a linkage stub carries no symbol: without
it `FlowInfo`'s call query finds nothing and every library call prints
`sub_<addr>(...)`. Each format reconstructs the stub→name map from its own linkage
structures, and all of them emit the same `ImportSym` currency into the loader's
funcsym stream:

- **ELF** (`decompiler/crates/kuna-analysis/src/loader/elf_plt.rs
  (resolve_plt_imports)`, the `ElfDefaultGotPltMarkup` analog): build
  `got_slot → name` from the dynamic relocations, then decode each `.plt*` stub's
  indirect jump per architecture (x86-64/x32, i386, AArch64, ARM, RISC-V, SPARC)
  and match the *decoded* GOT target against the map — self-correcting, since PLT0
  and IRELATIVE/IFUNC stubs jump to non-symbol-bearing slots and fall out
  automatically. `.plt.sec`/`.plt.got` outrank `.plt` so the CET call target wins.
  Two ABIs need special handling: PowerPC (ELFv2 `.plt` is a NOBITS data table, not
  decodable code; PPC32 uses its own secure-PLT stub shape), and **MIPS**, which
  has no PLT and no jump-slot relocations at all — its resolver walks the
  `.MIPS.stubs`/GOT layout from the dynamic table (`DT_MIPS_LOCAL_GOTNO`/
  `DT_MIPS_GOTSYM`) and marks the external GOT slots constant, so with
  read-only propagation the `lw $t9, off($gp); jalr $t9` sequence folds to the
  named import (the bootstrap turns `readonlypropagate` on for MIPS only).
- **i386-PIE stubs** (angr, `i386_pie_plt`, default-on, env-bridged): a PIE i386
  PLT entry is GOT-relative (`jmp *disp(%ebx)`, bytes `FF A3 <disp32>`), so naming
  it needs the GOT base `%ebx` holds at run time; `elf_plt.rs (i386_got_base)`
  derives it once and threads it into the i386 decoder. Off (or non-PIC), only the
  absolute `FF 25` form decodes, as upstream. Without this a 32-bit PIE's `exit`
  stays `sub_<addr>` and is never marked no-return — the spurious
  `do {} while(true)` symptom.
- **PE IAT** (`decompiler/crates/kuna-analysis/src/loader/pe_iat.rs`, the
  `PeLoader.processImports` analog): walk each import descriptor's INT (names) and
  IAT (slots) in lockstep — the i-th name belongs to the slot at
  `image_base + first_thunk_rva + i*ptr` — naming the slot (the GOT analog, folded
  through the read-only `.idata` page) and additionally decoding the MinGW `FF 25`
  thunk veneers so a direct `call thunk` also resolves. Import-by-ordinal
  synthesizes `<DLL>_Ordinal_<n>`.
- **Mach-O stubs** (`decompiler/crates/kuna-analysis/src/loader/macho_stubs.rs`,
  the `MachoProgramBuilder.processIndirectSymbols` analog): the `LC_DYSYMTAB`
  indirect-symbol table indexed by each `__stubs`/symbol-pointer section's
  `reserved1`, entry address `sec.addr + i*stride`. Calls target the stub
  *directly*, so naming the entry is sufficient and arch-independent;
  `__la_symbol_ptr`/`__got` slots are named too for `-fno-plt`-style indirect
  calls. `INDIRECT_SYMBOL_LOCAL`/`ABS` entries are skipped; the C-ABI leading `_`
  is stripped.

Two arch-marker passes paint **decode context** rather than names, because a wrong
decode mode is unrecoverable downstream. `decompiler/crates/kuna-analysis/src/loader/arm_markers.rs
(ArmMarkerPass)` (`arm_markers`) ports ARM's `ARM_ElfExtension`/`ArmSymbolAnalyzer`:
`$t`/`$a` mapping symbols and the STT_FUNC odd-address convention become `TMode`
paints, applied to the engine's `ContextDatabase` at commit, before any decode.
`decompiler/crates/kuna-analysis/src/loader/mips_markers.rs` carries the MIPS pair:
`MipsIsaModePass` (`mips_isa`) paints `ISA_MODE` at MIPS16e/microMIPS entries
(LSB-set or `st_other` STO-marked), and `MipsMarkerPass` (`mips_gp`) is a register
**value** seed, not a context bit — `t9 = func_entry` per function (the PIC
`jalr t9` convention, Ghidra's `MipsAddressAnalyzer`), committed as a tracked-range
so the S3 constant-base action emits `COPY #entry -> t9` at the entry block and the
prologue's `addu gp,gp,t9` folds to a real `$gp`. Both are doubly guarded: the pass
gates on its architecture, and the commit swallows an unregistered-variable /
unknown-register error, so a paint on the wrong language is a faithful no-op.

## 1.4 Metadata analyzers

The always-on core, in pass order (`passes.rs (passes_for)`):

- **Strings** (`strings`, the `StringsAnalyzer` port,
  `decompiler/crates/kuna-analysis/src/analyzers/strings/mod.rs`): scan allocated,
  initialized sections for runs of printable ASCII (plus CR/LF/TAB) ended by a NUL,
  minimum visible length **5**; each hit commits a *typelocked* `char[len+1]` data
  symbol (`s_<addr>`) — the typelock is what carries the array type through type
  propagation, and the printer renders the literal, not the name. LOSS: Ghidra
  additionally scores candidates with a trigram model (`StringModel.sng`, not
  vendored), so kuna over-accepts random printable NUL-terminated runs; real
  literals are unaffected.
- **Library prototypes** (`libproto`, the `ApplyDataArchiveAnalyzer` analog,
  `decompiler/crates/kuna-analysis/src/analyzers/protos/mod.rs (LibProtoPass)`):
  Ghidra ships parsed C headers as `.gdt` archives; kuna substitutes a built-in
  table of common libc signatures (`puts(char*)`, `printf(char*,...)`, …), parked
  on matching callees so `ActionDefaultParams` types the caller's argument
  constants — this typing, plus the read-only markup, is what turns `puts(0x400915)`
  into `puts("Username: ")`.
- **DWARF** (`dwarf`, the `DWARFAnalyzer` port,
  `decompiler/crates/kuna-analysis/src/analyzers/dwarf/mod.rs (DwarfPass)`), the
  parser wholesale-substituted by `gimli` (the same dependency-substitution loss as
  BFD → `object`). Three recoveries: (1) names — each defined `DW_TAG_subprogram`
  emits a function symbol, each top-level `DW_TAG_variable` with a `DW_OP_addr`
  location a data symbol; (2) typed signatures — return + formal-parameter DIEs
  mapped to kuna `Datatype`s (structs as named opaques, with a recursion-depth cap
  against type cycles), registered *after* libproto so real source signatures win;
  (3) stack locals — direct `DW_OP_fbreg` children become typelock|namelock stack
  symbols at `call_frame_cfa + fbreg`, re-seeded per decompile (§1.1); nested
  lexical-block locals and composite locations are a documented loss. (ida) The
  data-global fix (DIV-24): a global used to be mapped with a size-1 type, so any
  multi-byte access queried `queryContainer(addr, 4)` past it and rendered
  `dat_<addr>`; the pass now resolves `DW_AT_type` to a byte size
  (`pass.rs (DataObjectFact)`) and the commit maps an `undefined<size>` entry —
  namelocked but *not* typelocked, so inference still recovers the real type —
  matching how IDA Pro and Ghidra name symbol-table globals (`max_width`, not
  `dat_<addr>`). Declaration-only DIEs are skipped so DWARF never fights libproto
  over imports. `dwarf_lines`
  (`decompiler/crates/kuna-analysis/src/analyzers/dwarf/lines.rs (DwarfLinesPass)`)
  is the separate `.debug_line` pass: each row becomes a `file:line` instruction
  comment in the commentdb; default-off because it changes the output.
- **Demangling** (`decompiler/crates/kuna-analysis/src/analyzers/demangle/mod.rs
  (demangle_name)`, the `GnuDemanglerAnalyzer` analog) is not a registered pass but
  a loader hook: applied to every funcsym name after `@VERSION` stripping, before
  install. Upstream shells out to libiberty; kuna substitutes the `cpp_demangle`
  (Itanium), `rustc_demangle`, and `msvc_demangler` (`?…` names) crates. The hard
  contract is **name-only** reduction: kuna's scope splitter nests on every `::`,
  so signature tails and template argument groups must be stripped or they become
  junk scopes.
- **Source-language detection**
  (`decompiler/crates/kuna-analysis/src/analyzers/sourcelang/mod.rs
  (detect_compiler)`, the `SourceLanguageAnalyzer` detection half) runs once,
  before pass selection, and shapes the pass list: `rustc version` records in
  `.comment` or Rust-mangled symbols → the Rust no-return list;
  `.go.buildinfo`/`.note.go.buildid` (any format's spelling) → the Go list plus
  the pclntab pass; PE detection reads the MSVC Rich header / MinGW `GCC:` records,
  Mach-O the `LC_BUILD_VERSION` family. The `Gcc`/`Clang` values are a kuna
  convenience nothing gates on.
- **Call fixups** (`callfixup`,
  `decompiler/crates/kuna-analysis/src/analyzers/callfixup/mod.rs`, the
  `CallFixupAnalyzer` analog): a function whose name matches a cspec call-fixup
  `<target>` (the `-pg` `mcount`/`__fentry__` stubs) is tagged with the fixup's
  inject id so the engine replaces the CALL with the fixup body; guarded by
  upstream's only-if-no-fixup-set check so a hand-applied fixup is never clobbered.

The format- and language-gated recoveries (each registered only for its format, so
every other binary's pass list is byte-identical to before the pass existed):

- **Go pclntab** (`gopclntab`, Go-detected binaries only, default-on;
  `decompiler/crates/kuna-analysis/src/analyzers/pclntab/mod.rs`): the runtime
  needs the PC→name table for stack traces, so it survives stripping; the pass
  handles all four header magics (go1.2/1.16/1.18/1.20 layouts) and emits one
  function symbol per entry, so a stripped Go binary renders `main.main` and
  `runtime.*` instead of `sub_<addr>`.
- **MSVC RTTI** (`rtti`, PE-only, default-off;
  `decompiler/crates/kuna-analysis/src/analyzers/rtti/mod.rs`): find the shared
  `type_info` vftable, byte-search back from each `.?A…@@` TypeDescriptor to its
  CompleteObjectLocator, validate the COL→RTTI3→RTTI2→RTTI1→RTTI0 reachability
  chain (x86 raw-VA vs x64 image-base-relative refs behind a refkind dispatch), and
  label `<Class>::vftable` / `RTTI_*` with the class names demangled by the
  existing MSVC arm.
- **Objective-C** (`objc`, Mach-O-only, default-off;
  `decompiler/crates/kuna-analysis/src/analyzers/objc/mod.rs`): walk
  `__objc_classlist` → `class_t` → `class_ro_t` → method lists (both absolute and
  small/relative forms), reading pointer slots through the chained-fixup overlay
  (§1.2) on arm64, and rename each IMP `-[Class sel]`/`+[Class sel]` behind the
  placeholder label gate.
- **PDB** (`pdb`, PE-only, default-off;
  `decompiler/crates/kuna-analysis/src/analyzers/pdb/mod.rs`): Windows' debug info
  lives in a separate `.pdb` the PE only fingerprints, so the pass reads the
  CodeView record, locates the file via `kuna_pdb_path`, and applies a hard
  **fingerprint gate** — the `.pdb`'s GUID/age must match or nothing is emitted
  (never apply a stale PDB) — then walks the global symbol stream
  (S_PUB32/S_GPROC32) and renames stripped functions behind the label gate.
  Name-level only; types and lines are deferred.
- **FID** (`fid`, default-off, Listing-gated, DB via `kuna_fid_db`;
  `decompiler/crates/kuna-analysis/src/analyzers/fid/mod.rs (FidPass)`): a
  byte-exact port of Ghidra's FunctionID hashing — the operand-masked FNV-1a64
  full hash over a function's instruction stream (mask via the SLEIGH
  `instruction_mask`, x86 NOP padding skipped) — looked up in a kuna `.fid`
  database built by `kuna fid build`. Only a bucket that collapses to exactly one
  name renames (never guess on a tie), and only through the placeholder label
  gate: the stripped-static-library recovery (`sub_4017c0` → `kuna_crc32`) with no
  way to clobber a real name.
- **Format strings** (`formatstring`, default-off, matching upstream's default;
  `decompiler/crates/kuna-analysis/src/analyzers/formatstring/mod.rs`): the one
  analyzer that is decompiler-*dependent* — the format constant only exists in the
  lifted caller — so it splits into the pure spec-parser (the `FormatStringParser`
  state machine: length modifiers, conversion specs, `%%`, `*` widths, positional
  args; malformed input parses to nothing) plus the call-site classification and
  override construction
  (`decompiler/crates/kuna-analysis/src/analyzers/formatstring/apply.rs
  (classify_variadic_call)`: name contains `printf`/`scanf`, scanf-family takes
  input types). The console driver orchestrates the decompile → read constant →
  install per-call-site prototype override → re-decompile loop; the pipeline itself
  never calls back into the tier.

## 1.5 Entry discovery

Function discovery decides what exists at all, so it is deliberately layered from
free-and-exact to speculative:

**The always-on oracle union** (`entry_disc`,
`decompiler/crates/kuna-analysis/src/analyzers/entry/mod.rs (EntryDiscoveryPass)`)
fuses the feasible subset of Ghidra's `EntryPointAnalyzer`,
`ExternalEntryFunctionAnalyzer`, `FunctionStartAnalyzer`, and the
`GccExceptionAnalyzer` FDE oracle into one additive pass: (1) the ELF `e_entry`;
(2) `DT_INIT`/`DT_FINI` and the `INIT_ARRAY`/`FINI_ARRAY` pointer tables, carrying
Ghidra-faithful names (`_INIT_<i>`/`_FINI_<i>`/`_DT_INIT`/`_DT_FINI`) through the
`entry_names` overlay; (3) every `.eh_frame` FDE's `pcBegin` — the highest-value
oracle on C/C++ binaries, since unwind data survives stripping; (4) the
`_start`→`main` libc-start idiom (x86-64 PC-relative `lea rdi`, and the
AArch64/ARM/RISC-V PIE form that loads `main` indirectly through an
`R_*_RELATIVE`-relocated GOT slot) — (kuna) the disassembly-free stand-in for the
call-target sweep the tier cannot do without a Listing; and (5) a minimal always-on
set of three bare x86-64 gcc prologue byte patterns; and (6, kuna) the reset +
handler pointers of an empirically-detected **ARM Cortex-M hardware vector table**
(`cortexm_vector_entries`) — a stripped bare-metal firmware image has no symbols,
no `.eh_frame`, no libc idiom and no `$t` markers, so the hardware vector table at
the base of the code section is the only entry source. The table is confirmed when
`word[0]` is a plausible SRAM stack pointer (`0x2000_0000..=0x3FFF_FFFF`) and
`word[1] == e_entry` (the reset vector); the odd (Thumb) handler pointers are then
harvested, LSB-masked, up to the start of code. Everything is unioned,
deduped, restricted to executable sections, and skipped where a real funcsym
already exists; a discovered ARM `main` whose GOT pointer had the Thumb LSB set
also emits its own `TMode=1` paint (a stripped binary has no `$t` symbol to paint
from). On a confirmed Cortex-M image the ELF `e_entry` seed is additionally
LSB-masked to its even (decode) address, and `cortexm_thumb_paints` region-paints
`TMode=1` (Thumb) across every executable section — ARMv6/7/8-M is Thumb-only, and
a Thumb `BL` does not `globalset` the callee mode, so the region paint is what lets
`main` and the rest of the reset→main call tree decode as Thumb (wired into both
the analysis commit path and the Listing walk's `ContextPainter`). These ARM paths
are strict no-ops on x86-64 and on any ARM object without the vector-table
signature. PE and Mach-O dispatch to their own oracles (`.pdata`/TLS/entry;
`LC_FUNCTION_STARTS`/`LC_MAIN`/`__mod_init_func`). Failure mode: discovery-only —
a wrong entry is a garbage `sub_<addr>`; a missed one is invisible until a caller
overruns into it (§1.7).

**The full pattern corpus** (`funcstart_patterns`, default-off;
`decompiler/crates/kuna-analysis/src/analyzers/entry/patterns/mod.rs`) is the
faithful `FunctionStartAnalyzer` port over the vendored per-arch pattern XML
(x86/x86-64, AArch64, ARM, RISC-V, MIPS, PPC): a candidate is a start iff a
postpattern (the prologue shape) matches at it *and* a prepattern (RET/JMP/NOP
context) matches immediately before it, at instruction alignment. The
`after="defined"`/`validcode` post-rules need a pseudo-disassembler and are a
documented loss. Output-changing, hence default-off — but (kuna, DIV-20) the
`decompile-all` driver turns it on for non-x86-64 binaries, where it is the
*primary* discovery source on stripped ARM firmware, alongside the always-on
Cortex-M vector-table oracle (6) above: with the vector-table seeds + Thumb region
paint, the pattern scan and the recursive-descent promotion (§1.6) lift betaflight
STM32F405 from 1 to ~1830 discovered functions (and libopencm3 `button` from 1 to
31, with `main` decoding as a real Thumb body rather than A32 garbage).

**LSDA landing pads** (`eh_frame_full`, default-off): the deeper
`GccExceptionAnalyzer` markup — follow each FDE's CIE `L` augmentation to its
`.gcc_except_table` LSDA, decode the call-site table, and emit each landing pad as
an entry; a catch/cleanup block is reached only by the unwinder, so nothing else
can see it. CFI itself (`DW_CFA_*`) is deliberately not recovered — kuna's own
frame analysis rebuilds the stack frame from the code.

**Address tables** (`addrtable`,
`decompiler/crates/kuna-analysis/src/analyzers/addrtable/mod.rs (AddrTablePass)`)
scan `.rodata`/`.data` for runs of pointer-width values all landing in executable
sections — vtables and absolute function-pointer arrays — emitting data symbols
plus a read-only range (never entries, never switch ranges; in-function switch
recovery is the inherited S2 engine machinery, a different thing entirely). Ghidra
ships this analyzer disabled, and kuna goes one further: the pass is implemented
and tested but **left out of the registered pass list** (`passes.rs` keeps its
registration commented out), because a pointer-run scanner over-accepts and the
relocation guard that defends it is weak on non-PIE executables.

## 1.6 The Listing tier

The Listing (`listing`, default-off as an engine option;
`decompiler/crates/kuna-analysis/src/listing/mod.rs (Listing)`) is the program-wide
recursive-descent disassembly the analyzer tier otherwise lacks — three read-only
sub-models behind one facade: instructions, cross-references (call/code edges both
directions), and discovered functions. It is built **at the deferred commit point**,
not at load, for both reasons of §1.1: its gate is an `option` line applied after
`load file`, and its decoder is the engine's own SLEIGH translator, whose loadimage
is attached only after the load-time passes run
(`passes.rs (run_listing_consumers)`, driven from
`engine.rs (commit_pending_analysis)`).

The build: seed with the union of real funcsym entries and the §1.5 oracle
discoveries, exec-filtered and deduped (`passes.rs (listing_seeds)`), plus the full
prologue-pattern starts when `funcstart_patterns` is on. The walk
(`decompiler/crates/kuna-analysis/src/listing/walk.rs (walk)`) is a two-level
worklist mirroring the S2 flow-follower's design without its weight: an outer
function worklist (every direct CALL target becomes a new function entry — the
program-wide recursion `FlowInfo` deliberately never does) and an inner
per-function instruction worklist over branch and fall-through successors, bounded
by the executable ranges and monotonic visit sets. Indirect targets are recorded
with their computed/indirect predicates but contribute no static successor. A
context painter applies the ARM/MIPS decode-mode paints per address before each
decode, so a Thumb or MIPS16 body disassembles in the right ISA. Each instruction
is decoded by driving `Translate::one_instruction` with a capturing p-code sink
(`decompiler/crates/kuna-analysis/src/listing/decode.rs (decode_one)`) and
classified by a lifted transliteration of the S2 flow rules
(`decompiler/crates/kuna-analysis/src/listing/classify.rs (classify)`), whose three
load-bearing gotchas are worth restating: a constant-space branch operand is
p-code-relative (an intra-instruction branch), never a VMA; fall-through is decided
by the *last* op only; and delay slots are already folded into the reported length.

Its **consumers** run over the built Listing and are individually gated: the
no-return consumers of §1.7 (`noreturn_disc`, and `noreturn_propagate` carrying
the `noreturn_error`/`noreturn_reach` sub-rules), the FID matcher (§1.4), the AIF
gap-walk, and (kuna) the recursive-descent promotion `funcdisc_recursive`, which
commits the walk's discovered CALL targets as real functions (coupled to the
`funcstart_patterns` flag; this is what finds call-only targets with no
recognizable prologue). **AIF** (`aif`, default-off with upstream's own "IT MAY
CREATE A LOT OF BAD CODE!" warning;
`decompiler/crates/kuna-analysis/src/analyzers/aif/mod.rs (run_aif)`) speculatively
decodes each undefined gap between discovered functions and accepts a gap start
only when it both disassembles into a valid subroutine (a clean flow to RET, more
than 2 instructions, no bad byte or out-of-range flow) *and* its prologue matches a
start fingerprint shared by at least 4 already-discovered functions
(`FINGERPRINT_THRESHOLD`) — the only oracle that can find a function reachable
solely through a `.rodata` pointer table. `operand_refs` (default-off, matching
upstream's ELF-off default) shares the deferred slot for the same
decoder-availability reason but does its own linear decode rather than reading the
Listing, planting `char[N]` facts for immediate operands that point into read-only
data.

Driver defaults (kuna): `kuna decompile-all` and `kuna decompile` inject
`option listing on` unless the caller names `listing` (DIV-15/DIV-22) — without it
the default-on no-return propagation is a structural no-op and a stripped binary's
unnamed exit wrappers swallow the functions after them. `kuna functions` keeps it
off: name enumeration gains nothing, and the whole-program decode turned a 0.21 s
listing of a stripped tar into 5.7 s (DIV-15). The console and datatest paths never
build one, which is what keeps every parity gate byte-identical while the
consumers' *option* defaults are on.

## 1.7 The no-return family

Whether a call falls through decides the CFG of every caller, so no-return facts
are program-prep facts, computed before any function is decompiled. Five analyzers
cooperate, each subsuming the last's blind spot; all of them emit the same
`NoReturnFact` through the same commit arm (address-resolved
`set_function_no_return`, §1.1), and the flow consequence — an artificial halt at
the call site, dead fall-through never decoded — is inherited from the engine's
flow layer (`decompiler/crates/kuna-decomp/src/p2_lift/flow.rs`), never
re-implemented per pass.

**Known names** (`noreturn_known`,
`decompiler/crates/kuna-analysis/src/loader/noreturn.rs (NoReturnKnownPass)`, the
`NoReturnFunctionAnalyzer` port) flags every function symbol whose name — leading
underscores stripped in a loop, so `__stack_chk_fail` matches `stack_chk_fail` —
appears on a shipped list, under upstream's namespace guard (global names and
exactly `std::`, never a class method like `Menu::_exit`). Which list applies is
format- and language-selected, the `noReturnFunctionConstraints.xml` model: the
vendored ELF list (`decompiler/crates/kuna-analysis/data/ElfFunctionsThatDoNotReturn`
— `exit`, `abort`, `__assert_fail`, `pthread_exit`, the C++ terminate/throw family,
and (kuna, DIV-21) the genuinely-unconditional additions upstream omits: the BSD
`err`/`errx`/`verr`/`verrx`/`errc`/`verrc` family, `quick_exit`,
`__assert_perror_fail`, `__chk_fail`, `__libc_fatal`; `warn`/`warnx` return and
stay out), widened by a Rust wildcard list (`core::panicking::panic*`,
`handle_alloc_error`, `rust_begin_unwind`) or a Go exact list (`runtime.gopanic`,
`runtime.throw`, `runtime.goexit`, …) when source-language detection fires (§1.4),
or replaced by the PE/Mach-O list (`__fastfail`, `_invoke_watson`, plus the shared
C names) off-ELF. The scan mirrors the exact symbol streams the loader installs and
emits the *install* address — for a UND import, the PLT-stub address, since the
`.dynsym` entry is address 0 and demangling means a raw-name lookup would miss.
Name-based: free, exact, and useless on stripped custom wrappers.

**Discovered, ≥3 evidence** (`noreturn_disc`,
`decompiler/crates/kuna-analysis/src/analyzers/noreturn_disc/mod.rs`, the
`FindNoReturnFunctionsAnalyzer` evidence tally; Listing-gated, default-on per
DIV-22 as in Ghidra): a callee is concluded no-return when at least **3** of its
call sites (`EVIDENCE_THRESHOLD`) show no valid fall-through — the byte after the
call is not a decoded instruction start, is data, or is another function's entry —
plus a bounded fixpoint promotion for a caller whose body contains a terminal call
to an already-concluded callee and no RETURN anywhere. The threshold buys
robustness to disassembly noise at the price of blindness to rarely-called
functions; and the predicate has a structural blind spot: a no-return call followed
by alignment **NOP padding** reads as a valid fall-through and contributes no
evidence at all.

**Propagation fixpoint** (angr; `noreturn_propagate`,
`decompiler/crates/kuna-analysis/src/analyzers/noreturn_propagate/mod.rs
(propagate_noreturn)`, the CFGFast returning-analysis idea): seed the terminal set
from the Known-flagged functions, then sweep the call graph to a fixpoint with
**no evidence threshold**. The base rule is a strict tail-call rule
(`function_is_no_return`), conservative by construction: a function is concluded
no-return only when its last *real* instruction — trailing NOP padding skipped,
closing exactly the blind spot above — is a CALL or tail JMP to a terminal-set
member, AND no RETURN exists in the body, AND no computed jump exists, AND every
static branch target stays inside the reachable body or is itself terminal. Each
conclusion joins the terminal set and re-enqueues callers, so a wrapper-of-a-wrapper
converges (sweeps bounded by candidate count + 2). This catches the canonical miss:
a cold wrapper like coreutils' `xalloc_die` — single-digit call sites, under the ≥3
threshold; `call abort` followed by padding, invisible to the evidence predicate —
which unconditionally cannot return. Without it, every caller grows a spurious
fall-through edge into the cold path that structures into an invalid
`while(true)`+`goto`.

Two rules fold into the same fixpoint, both Ghidra-derived:

- **The `error(nonzero,…)` value rule** (`noreturn_error`, DIV-16): glibc `error()`
  and `error_at_line()` exit when `status != 0` but return for `status == 0`, so
  `error` can never be a Known name. The recognizer resolves the `error` entry
  addresses, then per call site backward-scans the straight-line predecessors for
  the defining write of the first integer-argument register (x86-64 SysV
  `EDI`/`RDI`): only a literal `MOV` of a nonzero constant accepts; `XOR EDI,EDI`,
  any non-constant definition, an intervening call or branch all reject — a false
  positive would delete live caller code. A qualifying *tail* call concludes the
  wrapper no-return (GNU `pfatal_with_name`), and independently *every* qualifying
  call site is emitted as a `no_fallthru_calls` fact that the drivers apply as a
  per-site CALL_RETURN flow override
  (`decompiler/crates/kuna-cli/src/decompile_all.rs`) — the fall-through prune that
  stops the flow-follower from absorbing the next function.
- **CFG reachability** (`noreturn_reach`, DIV-19; the
  `targetOnlyCallsNoReturn` rule of Ghidra's discovered analyzer,
  `function_reaches_only_noreturn`): the tail rule is a subset — it cannot conclude
  a wrapper whose no-return call is mid-body with a dead tail after it (openssh
  `sshpkt_fatal`), whose RETURN is present but unreachable, or that routes through
  a switch whose every arm is no-return (`sshpkt_vfatal`). The generalization walks
  the instruction-level reachable graph from entry, treating a transfer to a
  terminal callee as ending its path, and concludes no-return iff no RETURN is
  reachable and at least one path ends at such a transfer. Every uncertainty — a
  reachable RETURN, an unresolved indirect jump, an escape to a possibly-returning
  neighbour, a call with no modelled fall-through — answers "returns". (ida) The
  one recorded over-conclusion and its fix: a GCC `-O2` hot/cold-split check
  (`jcc <.cold>` where the cold fragment is `call abort`) was short-circuited like
  an unconditional transfer, skipping the returning fall-through arm and marking
  the whole `quotearg_*` family no-return; a conditional jump now walks both arms,
  the returning shape IDA Pro and Ghidra both produce.

Finally, (angr) **flow-time extern matching** closes the case no address-keyed fact
can reach: in an ET_REL `.o`, a libc no-return is a UND symbol with no address and
no PLT, so nothing above ever marks it, and flow runs off the function's end into
alignment padding decoded as garbage `add [rax],al` statements.
`noreturn_externmatch`
(`decompiler/crates/kuna-decomp/src/p2_lift/kuna_noreturn_externmatch.rs`) applies
the same vendored name list and namespace guard *at the flow query seam*
(`decompiler/crates/kuna-decomp/src/infra/decompile_drive.rs
(query_call_no_return)`); its sibling `noreturn_extern` applies an equivalent
name match in the same query, differing only in gate flag and name-resolution
path. On a
normally-linked ELF the proto flag is already set, so both are no-ops there. These
two run inside the engine, not the analysis tier — chapter 02 owns the halt
mechanics they feed.

## 1.8 In-engine image binding

Inside the engine, P1 is the architecture/loader binding —
`decompiler/crates/kuna-decomp/src/p1_partition` — three front-ends over one base,
the C++ inheritance chain modeled by composition:

- `decompiler/crates/kuna-decomp/src/p1_partition/sleigh_arch.rs
  (SleighArchitecture)` is the base every path shares: resolve a language id
  against the `.ldefs` records scanned from the spec roots (the C++ file-level
  statics become an explicit `LanguageDatabase` value the bootstrap owns), find the
  `.pspec`/`.cspec`/`.sla` files, build the SLEIGH translator, and run the
  `Architecture::init` tail (type factory, prototype models, print language). The
  upstream translator-reuse cache is deliberately not ported — it affects build
  speed only — and is the recorded loss here.
- `decompiler/crates/kuna-decomp/src/p1_partition/xml_arch.rs (XmlArchitecture)`
  binds the decompiler's XML `<binaryimage>` container — the datatest corpus's
  entire load path, and the reason the analysis tier can be default-on without
  touching parity: this front-end never sees an `ObjectLoadImage`.
- `decompiler/crates/kuna-decomp/src/p1_partition/raw_arch.rs
  (RawBinaryArchitecture)` is the catch-all leaf for a raw byte image: its file
  match always succeeds (so capability sorting pushes it last), the language must
  be supplied by the target, and the loader is a plain offset-mapped
  `RawLoadImage`.

The real-binary path of §1.2 is the fourth binding, console-side: `bootstrap_from_object`
plays the leaf role itself — it resolves the language from the object header
(with the compiler-model fallback retry), runs `build_engine_and_init`, attaches
the default code space to the loader (the `postSpecFile` contract), and hands the
loader to the engine as the byte source every subsequent instruction decode reads
through.

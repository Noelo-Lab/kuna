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
**stashes** each load-time pass's output keyed by its id. The commit happens later,
at `read symbols` (`engine.rs (commit_pending_analysis)`) — after the CLI's
`option` lines have been applied — so a disabled load-time pass's already-computed
facts are simply dropped at the gate (`engine.rs (analysis_pass_enabled)`; an id
with no registered gate fails *open*, so a new pass runs by default). Deferred
decoder-dependent work is dispatched after those options are known: a disabled
Listing consumer, AIF gap walk, or operand-reference scan is not invoked at all,
and its commit gate remains as a defensive check. This is semantically load-bearing
for AIF: speculative SLEIGH decoding can paint processor context, so `aif off`
means no speculative decode, not merely discarding its discovered-entry facts.
The stash is drained on commit, so a second `read symbols` cannot double-commit.

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

The import currency deliberately includes both executable linkage stubs and
pointer slots in data sections: the latter must be function symbols so indirect
calls resolve to a name and library prototype. They are not function bodies.
The complete canonical inventory retains both, while automatic whole-binary
decompilation selects only entries contained by a loader `CODE` section
(`decompiler/crates/kuna-console/src/engine.rs
(ConsoleProgram::function_entries_executable)`). Explicit address selection
remains unrestricted; name selection keeps its normal first-match behavior when
a stub and slot share a name. Loaders without section metadata keep the complete
inventory.

Naming a pointer slot is not by itself enough to bind a call *through* it. An ELF
PLT stub and a Mach-O `__stubs` entry are code, so the call is direct and the name
resolves at flow time; a PE Import Address Table slot is data, so `call dword ptr
[slot]` lifts to a `CALLIND` whose target is the contents of a global. The only pass
that resolves such a target is `ActionDeindirect`, and its external-reference arm
requires the target Varnode to carry `Varnode::externref` — a flag Ghidra sets from
an `ExternRefSymbol` (`Scope::addExternalRef`) that kuna's port never carried, so on
a PE the flag was set nowhere and every Windows API call stayed an unnamed
`(*dat_4112c4)(0)`: no name, no prototype, and no no-return flow effect.
`decompiler/crates/kuna-analysis/src/loader/kuna_peimportcall.rs (PeImportCallPass)`
(`peimportcall`, PE/COFF-only, default-on per DIV-57) closes that with the property
map rather than a second symbol: it reports one `[slot, slot+ptr)` range per import
descriptor entry and the commit ORs `Varnode::externref` over each, the same
`Database::setPropertyRange` the loader's read-only section ranges use.
`Scope::queryProperties` folds the property map into every global Varnode covering
the range, so the slot read now carries `persist|externref` and `ActionDeindirect`
resolves it against the `FunctionSymbol` the IAT walk already registered at that
same slot VA — kuna's `Architecture::query_function` keys on the Varnode's own
address, where upstream indirects through `ExternRefSymbol::refaddr`, so no extra
symbol is needed. The flow half rides the same gate: `query_function` also carries
the resolved callee's no-return flag onto the prototype it hands `ActionDeindirect`
(the snapshot in
`decompiler/crates/kuna-decomp/src/p0_knowledge/database.rs (Database::build_global_query)`
dropped it, where upstream returns the callee's live `Funcdata`), which is what makes
the deindirect schedule the restart whose re-flow plants the artificial halt. Off,
a PE renders byte for byte as before; every non-PE target is unaffected either way.

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
  into `puts("Username: ")`. LOSS: the built-in table is not a header archive, so
  it covers only the names it lists; every other libc callee leaves its caller's
  argument an inferred integer.
- **(kuna) Measured libc signatures** (`libcsigs`,
  `decompiler/crates/kuna-analysis/src/analyzers/protos/kuna_libcsigs.rs (LibcSigsPass)`):
  the second, larger half of the same table, closing most of the LOSS above. Which
  names it carries was *measured*, not guessed — a PLT call-site histogram over the
  frozen decbench C corpus plus a per-callee ranking of the cases where a rival
  decompiler recovers a perfect parameter typing and kuna does not; a name is in
  the table when it clears 100 corpus call sites or 3 such cases. The signatures
  themselves are reduced from the platform's own C declarations (`gcc -aux-info`
  over the standard headers, GCC's builtin types for the FORTIFY `_chk` entry
  points, the `<stdio.h>` `__REDIRECT` for the `__isoc99_*` aliases), never written
  from memory, and any declaration with a slot whose width is not stable across
  ILP32/LP64 — `off_t`, `time_t`, `long long`, a `char` parameter — is **rejected
  rather than approximated**, because a wrong prototype is worse than a missing one:
  it asserts a false type where the inferred integer was merely uninformative.
  Two consequences follow from that same principle. A signature is applied only to
  a name the image **imports** and does not itself define — a PLT/IAT import named
  `error` is the platform's `error(int, int, const char *, …)`, but a *defined*
  `error` is the program's own function that happens to share the spelling (zlib's
  `minigzip` declares `void error(const char *)`), and the base table's
  defined-or-imported matching is left untouched. And the FORTIFY entry points are
  modeled as the distinct functions they are, not as aliases: `__printf_chk` takes
  a leading `int flag` before the format string, `__fprintf_chk` a `FILE *` and a
  flag, so treating either as its plain namesake would shift every argument of the
  most frequent call in the corpus.
- **DWARF** (`dwarf`, the `DWARFAnalyzer` port,
  `decompiler/crates/kuna-analysis/src/analyzers/dwarf/mod.rs (DwarfPass)`), the
  parser wholesale-substituted by `gimli` (the same dependency-substitution loss as
  BFD → `object`). Three recoveries: (1) names — each defined `DW_TAG_subprogram`
  emits a function symbol, each top-level `DW_TAG_variable` with a `DW_OP_addr`
  location a data symbol; (2) typed signatures — return + formal-parameter DIEs
  mapped to kuna `Datatype`s (structs as named opaques, with a cycle guard on the
  DIE walk — see `typedepth` below), registered *after* libproto so real source
  signatures win,
  and read back at *two* points: by a caller's `ActionDefaultParams` for the call
  site, and by the drive as the function's own locked prototype (04 §4.2 —
  `int main(int argc, char **argv)`, not `undefined16 main(uint4, void*)`);
  a `DW_TAG_enumeration_type` becomes a real enum type — name, declared width,
  signedness, and the `DW_TAG_enumerator` value→name map (05 §5.1), which is what
  turns `quotearg_style(4, …)` into
  `quotearg_style(shell_escape_always_quoting_style, …)`; the enum is looked up
  before it is built, because the same declaration recurs in every compilation
  unit that includes its header;
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
- **DWARF C++ prototypes** (`cppproto`, default-on,
  `decompiler/crates/kuna-analysis/src/analyzers/dwarf/kuna_cppproto.rs`) is the
  C++ arm of that same pass. Keying every recovery off a subprogram DIE's own
  `DW_AT_name` is right for C and wrong for C++, where the compiler splits a
  definition from its declaration: an out-of-line member or namespace definition
  carries only `DW_AT_specification`, and a concrete out-of-line instance of an
  inlined function only `DW_AT_abstract_origin`. Neither has a name of its own, so
  the whole DIE — name, signature and stack locals — used to be dropped, and on a
  `-g` C++ binary that is most of the program. This arm fuses the definition with
  the declaration it points at (a **single hop**: what a definition points at is
  always a declaration, never another indirection — the reduction of Ghidra's
  `DIEAggregate`), takes the name, return type and parameter names from whichever
  DIE carries them, and builds the source name by walking the DIE's
  namespace/class ancestry (`DWARFName`), so the installed symbol carries
  `Account::deposit` rather than the bare `deposit` the declaration DIE holds. Three type-mapper corrections ride
  along: `DW_TAG_class_type` maps like a structure and a C++ reference like a
  pointer (both are what Ghidra's importer does, and without the first every
  `Foo *this` degraded to `void *`); the transparent qualifier hops
  (`typedef`/`const`/`volatile`/`restrict`) are collapsed before the type switch
  runs, because a `const` member function's `this` is `const Account *const` —
  four DIEs deep, and under the pre-`typedepth` budget one hop too many; and a
  parameter whose type the switch still cannot
  map degrades to an `undefined<n>` of that DIE's own width instead of discarding
  the entire signature, so one exotic member type costs one parameter's type
  rather than the function's whole prototype. Finally the recovered prototype is
  parked by **entry address** rather than by name. Address is the key the read
  side already uses, and the only one that survives C++: kuna files the demangled
  template name `maxof<int>` as `maxof`, and a qualified name lives in a nested
  scope that a global by-name query cannot reach — so both the drive's own-prototype
  lookup (04 §4.2) and the callee-prototype snapshot resolve across every scope,
  not just the global one. The producing pass runs at `load file`, upstream of the
  `option` commands, so its C++ facts are stashed apart from the always-on ones and
  the gate is applied where they are committed; with `cppproto off` the DWARF
  recovery is the name-only walk, byte for byte. Struct/class **fields** are still
  not populated — a class remains a named opaque, so `this->balance` prints as an
  offset — which is the next increment, not this one.
- **Full-depth DWARF types** (`typedepth`, default-on,
  `decompiler/crates/kuna-analysis/src/analyzers/dwarf/kuna_typedepth.rs`) is the
  type mapper's recursion guard, and it exists because the DIE walk can be handed a
  chain that closes on itself — a `DW_TAG_pointer_type` whose `DW_AT_type` is its
  own offset, a `typedef`/`const` pair pointing at each other — which nothing in
  the format forbids and a truncated or forged `.debug_info` supplies. Upstream
  (`DWARFDataTypeImporter.trackRecursion`) guards it with a **per-DIE-offset
  re-entry counter**: a DIE may be re-entered twice and the third entry is refused,
  which fires only on a cycle because an acyclic chain visits each offset once.
  kuna's port had reduced that to a flat three-hop budget counted over *every*
  link, transparent qualifiers included — which conflates "the same DIE again" with
  "a deep but finite chain". Four DIEs is ordinary C: `const char *const *`,
  `const size_t *`, `char *const []`, `char ***`. All of them ran out of budget and
  fell back to `void`, so a `-g` binary's stack locals, its globals (a truncated
  element type sizes the global at one byte, and the extent is what the container
  query needs — §1.4) and its deeper pointer parameters rendered `void *` while the
  debug info named a concrete type. This restores upstream's counter, with a second
  absolute nesting bound as a native-stack backstop that a Java port does not need;
  termination no longer rests on a cap that also has to be small. Two consequences
  ride along: the qualifier collapse the C++ arm introduced now runs for the C
  callers too — that is what carries an anonymous aggregate's typedef name onto it
  (a local `mbstate_t`, not the shared `anon_struct` every unnamed struct fuses
  into) — and when the borrowed name is one the type factory already holds under
  another kind (kuna registers a core type called `code`, which zlib's
  `inftrees.h` really does typedef an anonymous struct to), the aggregate falls
  back to the anonymous name rather than failing to build and letting the pointer
  arm degrade it to `void *`. Like the other DWARF gates the mapping happens at
  `load file`, upstream of the `option` commands — but unlike `cppproto` this one
  changes how a single fact set is *built* rather than selecting between two, so
  the live gate is the process env var
  (`decompiler/crates/kuna-decomp/src/p0_knowledge/kuna_typedepth.rs`) that the
  CLI exports before the load, the same bridge `relocobjects` and `i386_pie_plt`
  use. With `typedepth off` the mapper is the pre-fix budget, byte for byte.
- **Demangling** (`decompiler/crates/kuna-analysis/src/analyzers/demangle/mod.rs
  (demangle_name)`, the `GnuDemanglerAnalyzer` analog) is not a registered pass but
  a loader hook: applied to every funcsym name after `@VERSION` stripping, before
  install. Upstream shells out to libiberty; kuna substitutes the `cpp_demangle`
  (Itanium), `rustc_demangle`, and `msvc_demangler` (`?…` names) crates. The hard
  contract is **name-only** reduction: kuna's scope splitter nests on every `::`,
  so signature tails and template argument groups must be stripped or they become
  junk scopes.
- **Demangled C++ signatures** (`cppsig`, `off|proven|inferred`, default `proven`;
  `decompiler/crates/kuna-analysis/src/analyzers/demangle/kuna_cppsig.rs`, the
  `DemangledFunction.applyTo` / "Apply Function Signatures" analog) is the
  *signature* half of demangling, and the first consumer of the full c++filt form
  the module has always been able to produce. Where the DWARF arm above needs
  debug info, this one needs only the mangled symbol — which is what a **stripped**
  C++ shared library still exports through `.dynsym` — so the two are
  complementary, and where both reach a function the DWARF prototype (ground truth)
  is applied last and wins over the demangled one (a declaration).
  The declaration is parsed out of the demangled *string*, as upstream's
  `GnuDemanglerParser` does: the last depth-0 parenthesis group is the parameter
  list, the last depth-0 token before it is the qualified name, and a trailing
  `const`/`volatile`/`&`/`&&` is the cv/ref qualifier. Each declared parameter maps
  to a pointer of any depth, a primitive, or — as a POINTEE only — a named opaque
  structure carrying the bare innermost class name (upstream's placeholder
  structure). An aggregate passed **by value**, an array, a function pointer, a
  pointer-to-member or an overloaded operator refuses the whole signature: the
  mangling carries no layout, and a wrong width shifts every following parameter.
  The **return type is deliberately not applied**. Itanium encodes one only for a
  template function, so upstream returns null and keeps whatever the analysis
  recovered; kuna expresses that as a prototype with no `outtype`, which the drive
  reads as "lock the INPUT half only" and leaves return recovery running (04 §4.2).
  What makes this a three-valued option rather than a flag is the **implicit object
  parameter**: Itanium mangles a static member function exactly like a non-static
  one and like a namespaced free function, and inventing a `this` that is not there
  shifts every parameter rather than merely losing precision. `proven` therefore
  applies only the shapes the mangling *entails* — a constructor, a destructor, a
  cv-/ref-qualified member (all three take `this`), an unqualified global name, and
  the MSVC forms, which state the access specifier, `static`, and the calling
  convention outright. `inferred` additionally decides the ambiguous nested names
  from class evidence mined out of the binary's own symbols: a scope that owns a
  constructor, a destructor, a cv-qualified member or a `_ZTV`/`_ZTI`/`_ZTS` symbol
  is a class, so its members take `this`; a scope with no such witness is a
  namespace, so its functions do not. A 32-bit MSVC `__thiscall` member is refused
  under every mode — that ABI passes `this` in ECX rather than as ordinary argument
  0, and selecting the registered `__thiscall` prototype model (04 §4.1) is the
  follow-up. Like the DWARF arm the pass runs at `load file`, so both certainty
  tiers are computed there and stashed apart, and the mode selects which of them
  the analysis commit applies.
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
- **(kuna) Itanium RTTI** (`itaniumrtti`, ELF-only, default-off;
  `decompiler/crates/kuna-analysis/src/analyzers/rtti/kuna_itaniumrtti.rs`): the
  GCC/Clang counterpart of the pass above, and a capability with **no Ghidra
  equivalent at all** — upstream's `RttiAnalyzer` is a Microsoft-PE analyzer and its
  GCC class recovery is script-tier, so on a stripped `g++` binary Ghidra leaves the
  vtable as an unnamed `DAT_<addr>`.

  Where the MSVC sibling has to *guess* which bytes are metadata (it byte-searches
  for `.?A` strings and treats `ref − 12` as a candidate structure), the Itanium
  graph offers an **exact anchor**. The three `__cxxabiv1` typeinfo vtables live in
  libstdc++, so on any dynamically linked C++ image every `_ZTI…` typeinfo object's
  leading `vptr` word is an undefined-symbol dynamic relocation naming
  `__class_type_info`, `__si_class_type_info` or `__vmi_class_type_info` with addend
  `2 × ptr` — and `.rela.dyn` is a loader input that `strip --strip-all` cannot
  remove. The relocation's offset *is* the typeinfo address and its symbol *is* the
  flavour, which fixes the object's layout past the `[vptr][name ptr]` prefix. A
  defined `_ZTI…` symbol is a second discovery source for the unstripped or
  statically linked case, its flavour sniffed from the object's shape.

  Each typeinfo's `_ZTS…` type-name string — the bare mangled-name component, which
  no demangler accepts alone — is recovered by wrapping it back into the `_ZTS`
  symbol form and demangling that, the exact analog of the MSVC `??_R0…@8` wrap and
  likewise adding no new demangler. Two details of that string are load-bearing and
  each one silently costs whole classes when missed. A **leading `*`** marks a type
  whose identity is local to one translation unit (ABI §2.9.1: compare `type_info`s
  by pointer, not by string); it is not part of the mangled name, and leaving it on
  makes every anonymous-namespace class — which is how most C++ spells a concrete
  implementation of an exported interface — undemangleable. And the demangled result
  is turned into an identifier by **folding** template arguments in
  (`Vec<int>` → `Vec_int`) rather than by the module-wide `strip_bracket_groups`
  reduction the rest of the demangler applies: two instantiations are two classes
  with two vtables, and collapsing both to `Vec` makes the second lose the idempotent
  symbol-commit race and keep `sub_<addr>` for every method. The `::` split is
  depth-aware so a separator inside an argument list is not read as a scope boundary.

  The `__si_`/`__vmi_` base lists then give the inheritance graph *with its byte
  displacements*, the datum the MSVC path discards along with its `pmd` fields.

  Vtables are reached **from** the typeinfo rather than guessed: every sub-vtable's
  second header word points at its most-derived class's typeinfo, so one scan for
  pointer slots holding a discovered typeinfo address yields them all, and two exact
  ABI constraints reject the coincidental hits (chiefly the base-class pointers
  inside other typeinfo objects, which also hold a typeinfo address) — `offset-to-top`
  is always `≤ 0`, and a real sub-vtable has at least one slot pointing into an
  executable section. A slot whose file word is zero but which carries a dynamic
  relocation is an *imported* virtual method (`__cxa_pure_virtual`, a base method
  defined in another image), so the walk steps over it instead of terminating and an
  abstract interface keeps its true extent.

  The pass emits `<C>_typeinfo`, `<C>_typeinfo_name`, `<C>_vtable` and `<C>_vptr`
  data labels — the last being the value an object's vptr actually holds, two words
  past the header, which is the constant a constructor stores — plus one
  `<C>::vtable_<i>` function symbol per virtual slot, and marks the slot arrays
  read-only. A secondary sub-vtable takes the name of the base subobject its
  displacement identifies (`Widget_vtable_for_Drawable`), and its slot names are
  prefixed accordingly, because a multiple-inheritance class has several sub-vtables
  whose indices all restart at 0. An inherited slot claimed by several classes'
  tables is attributed to the class that **defines** it, using the recovered base
  graph, so `Shape::perimeter` — repeated verbatim in `Circle`'s and `Square`'s
  tables — is named once, for `Shape`. Data labels join the class to the kind with
  `_` rather than `::` because the C printer emits a global by its leaf name, which
  would otherwise render every class's vptr as a bare, ambiguous `vptr`; function
  symbols keep the `::` form, whose qualification *is* rendered at a call site
  (§9, `cppcallnames`).

  The pass is blind to a `-fno-rtti` build by construction: no typeinfo is emitted,
  so no anchor exists and the output is empty. Independent code-pointer-run scanning
  — which would find such vtables heuristically — is deliberately **not** part of
  this pass.
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
  input types). The **driver** orchestrates the decompile → read constant →
  install per-call-site prototype override → re-decompile loop; the pipeline itself
  never calls back into the tier. That loop is the shared per-function decompile
  step (`decompiler/crates/kuna-console/src/decompile_step.rs (decompile_one)`,
  chapter [00](00-overview.md) §0.2), so it applies identically to the console
  `decompile` command and to every whole-binary surface; when it ran only in the
  console command the option was inert on `decompile-all` (DIV-66) — and once both
  surfaces honoured it, the second decompile's cost (+43% to +75% on a
  printf-heavy whole binary, all of it the re-decompile rather than the read-only
  propagation) took the option out of the `aggressive` preset, so it is a per-run
  opt-in everywhere. Reading a
  format constant needs read-only propagation — on ARM the format address is
  loaded PC-relatively from a literal pool, so the format-arg varnode is a memory
  LOAD that only constant-folds through `Funcdata::fillin_read_only` — so the step
  enables it for the duration of the decompile and restores the prior value.
  That side effect is much broader than the varargs typing itself: with it on,
  every literal-pool pointer in an ARM function resolves, which is why enabling
  `formatstring` rewrites most of a Cortex-M firmware function's body and not
  just its `printf` call sites.

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
the base of the loaded image is the only entry source. The table is confirmed when
`word[0]` is a plausible SRAM stack pointer (`0x2000_0000..=0x3FFF_FFFF`) and
`word[1] == e_entry` (the reset vector); the odd (Thumb) handler pointers are then
harvested, LSB-masked, up to the start of code. The table is looked for in every
section the **program headers** load as executable, not only the `SHF_EXECINSTR`
ones (`decompiler/crates/kuna-analysis/src/analyzers/entry/mod.rs
(phdr_executable_sections)`): a `PT_LOAD` carrying `PF_X` maps its sections as
executable memory whatever their `sh_flags` say, and the table is DATA the CPU
reads, so requiring an executable section header of the *table* was a category
error — what must be executable is what the handler entries POINT AT, which the
harvest still checks. Bare-metal link scripts routinely leave `.isr_vector`
flagged `WA` at the base of the single `RWE` load segment. `SHF_EXECINSTR`
sections are still tried first, so an image that already matched matches the same
section, and an object with no program headers (a relocatable `.o`) has no widened
candidate set at all. Everything is unioned,
deduped, restricted to executable sections, and skipped where a real funcsym
already exists. That funcsym set
(`decompiler/crates/kuna-analysis/src/analyzers/entry/mod.rs (existing_function_addrs)`)
is itself Thumb-masked on 32-bit ARM
(`decompiler/crates/kuna-analysis/src/analyzers/entry/mod.rs (thumb_masked)`),
because an ARM/Thumb function's ELF symbol stores the mode bit in bit 0 of
`st_value` and the odd address is not an instruction boundary. Masking it is what
makes the skip comparable with the already-masked `e_entry` candidate — otherwise
a named function is re-emitted as a "new" start and picks up a generated
`sub_<addr>` name — and it keeps the raw odd address from being seeded as a
function start in its own right, which would yield a phantom entry that decodes
mid-instruction to an empty body. The mask is gated to `Architecture::Arm`: on a
byte-aligned ISA an odd entry address is genuine (x86-64 fixtures have real
functions at `0x40071d` and `0x1357`), and AArch64 has no Thumb state. A
discovered ARM `main` whose GOT pointer had the Thumb LSB set
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

**The widened vector-table signature** (`cortexmvectors`, default-off; kuna;
`decompiler/crates/kuna-analysis/src/analyzers/entry/kuna_cortexmvectors.rs`)
relaxes all three of oracle 6's confirmation predicates, each of which measurement
over the ARM Cortex-M corpus showed over-constrains real firmware. The table is
data the CPU reads, so a bare-metal link script normally emits `.isr_vector` as an
`A`-only section inside a *read-only* `PT_LOAD` — which is neither
`SHF_EXECINSTR` nor inside a `PF_X` load, so even the program-header widening
above cannot see it. STM32F4 and `-M7` parts put the initial stack in CCM/TCM at
`0x1000_0000`, below the architectural SRAM block. And `e_entry` is the ELF's
start symbol, which a link script is free to point somewhere other than the reset
vector (nuttx points it at `__start`, crazyflie at the `.text` base). With the
option on a candidate is therefore **any allocated section** whose `word[0]` lies
anywhere in `0x1000_0000..=0x3FFF_FFFF` and whose slots from `word[1]` on yield at
least three Thumb handler pointers — a run of handlers replaces the `e_entry`
equality, because two conforming words can occur by chance inside a `.data`
structure and three consecutive ones essentially cannot. The run is counted by the
same harvest loop the oracle then seeds from, over accepted *slots* rather than
distinct addresses (a bare-metal table aims most of its vectors at one shared
`Default_Handler`). The harvest's "stop once the scan reaches the lowest handler,
i.e. the start of code" rule is also conditioned on the lowest handler lying at or
above the table's own base, since a table linked into RAM above the flash it
points at (betaflight) otherwise looks one word long. The widened scan runs
**only where the shipped signature found nothing**, so an image that already
resolved a table resolves the same section with the same harvest: the option can
add discovered entries, never remove one. It ships as its own `AnalysisPass`
rather than as a flag inside `entry_disc`, because a load-time pass runs before
`--option` is applied — the stash-at-load/gate-at-commit shape (§1.1) is what
makes an output-changing discovery flag observable at all. The pass emits entry
facts and the Thumb region paint and deliberately does **not** feed the Listing
walk (§1.6): the walk treats an unconditional `B` as same-function flow, so
seeding an ISR stub that tail-calls a shared handler makes the walk absorb that
handler and drop its own entry, which measured as a net loss. Output-changing
(more functions), hence default-off; ARM-only and real-object-path only, so every
XML datatest is structurally untouched.

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

**(kuna) FDE interiors are not function starts** (`fdeinterior`, default-**on**,
DIV-61; `decompiler/crates/kuna-analysis/src/analyzers/entry/kuna_fdeinterior.rs`).
A kuna `FunctionSymbol` is an entry address with no extent, so the commit boundary
cannot answer *is this candidate already inside a known function?* — and every
oracle above is free to plant a `sub_<addr>` in the middle of a body it cannot
see. Three do it on ordinary compiler output: the landing pads `eh_frame_full`
emits sit mid-frame by definition; the aggressive gap walk (§1.6) starts one at the
first undecoded byte of an unwinder-only region, which is routinely *mid
instruction*; and the prologue patterns match an aligned `push rbp; mov rbp,rsp`
inside a larger body. Such a "function" inherits its parent's live frame pointer,
so it decompiles with an uninitialised `rbp` and every local becomes a garbage
dereference. `.eh_frame` supplies exactly the missing extent: each FDE records one
function's `[pcBegin, pcBegin + pcRange)` by construction (one
`.cfi_startproc`/`.cfi_endproc` pair), so an entry strictly inside one is not a
function on the unwinder's own authority — the model IDA Pro uses, where
`get_func()` of a landing pad returns the enclosing function taken from the FDE
range. This pass reports those bodies and the commit filters the *fully merged*
entry set against them (after the deferred Listing consumers, so the gap walk is
covered too). Not every FDE describes one function — the linker gives the whole
PLT a single FDE, and every stub inside it is real — so a range is used only when
it holds no other named function start, no other FDE `pcBegin`, and no linker-stub
section (`.plt`/`.plt.sec`/`.plt.got`/`.iplt`/`.MIPS.stubs`). An entry *at* an FDE
start is always kept, so oracle 3's own product survives. ELF-only and inert
without `.eh_frame` FDEs, which covers essentially the whole bare-metal ARM
population (they unwind through `.ARM.exidx`), so the ARM entry-recall options
compose with it unchanged.

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

The Listing model (`listing`, default-off as an engine option;
`decompiler/crates/kuna-analysis/src/listing/mod.rs (Listing)`) is the program-wide
recursive-descent disassembly the analyzer tier otherwise lacks — three read-only
sub-models behind one facade: instructions, cross-references (call/code edges both
directions), and discovered functions. It is built **at the deferred commit point**,
not at load, when either the full `listing` tier or the bounded
`fast_funcdisc` consumer requests it. Both gates are `option` lines applied after
`load file`, and the decoder is the engine's own SLEIGH translator, whose loadimage
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

**Fast function discovery with conservative pointer validation** (`fast_funcdisc`, default-off;
`decompiler/crates/kuna-analysis/src/analyzers/fast_funcdisc/mod.rs
(pointer_table_seeds)`) reuses that one walk without enabling the full Listing
tier. Its initial roots are only the loader-backed function symbols and the §1.5
format oracles; full `funcstart_patterns` roots are included only when both
`listing` and that option are on. The Listing walk recursively follows every
static CALL from those trustworthy roots, and `fast_funcdisc` commits all
resulting function entries.

The second source covers indirect-only callbacks. On non-ARM objects, scan
allocated, initialized, non-executable data for pointer-width runs of at least
two absolute values into executable ranges. Ignore a table longer than 256
slots. If the remaining tables produce more than 512 unique targets, discard
targets referenced by fewer than two distinct tables. Rank the survivors by
independent-table count and validate at most 4096. A candidate must still be
undefined in the Listing and must satisfy both AIF corroborators: its first two
decoded instruction mnemonics and their byte length form a fingerprint seen at
least four times among already-reached functions, and the bounded
`check_valid_subroutine` probe must cover more than two instructions without a
bad decode or out-of-image flow and reach either a terminal/computed jump or an
informative call/edge into known code. Accepted bodies are claimed so a later
candidate cannot split them. ARM instead reuses the existing Thumb-pointer
oracle: an aligned odd code pointer is accepted only at an undefined
frame-establishing prologue that passes the same valid-subroutine probe.

Pointer-derived roots are committed but are deliberately not fed through a
second recursive walk. Thus the bounded path obtains direct-call closure and
high-confidence callback/vtable roots while avoiding the full prologue scan,
the AIF cursor over every undefined code gap, and recursive expansion from
disconnected pointer roots. Turning on `fast_funcdisc` alone does not run no-return, FID, AIF, or any
other ordinary Listing consumer.

The full Listing **consumers** run over the built model and are individually gated before
invocation (with the commit gate retained defensively): the
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
(`FINGERPRINT_THRESHOLD`) — the exhaustive gap oracle for functions with no
static or accepted pointer-table root. `operand_refs` (default-off, matching
upstream's ELF-off default) shares the deferred slot for the same
decoder-availability reason but does its own linear decode rather than reading the
Listing, planting `char[N]` facts for immediate operands that point into read-only
data.

(kuna) Three ARM-only seed scans run between the walk's first pass and those
consumers, each re-seeding the walk and rebuilding the Listing when it finds
anything, all gated by the `funcstart_patterns` flag: the raw unpaired
Thumb-prologue scan
(`decompiler/crates/kuna-analysis/src/analyzers/aif/mod.rs (raw_thumb_prologue_seeds)`,
angr's `_func_addrs_from_prologues` mirror — every `PUSH {..,lr}` / `PUSH.W {..,lr}`
in an undefined gap that passes the valid-subroutine probe), the code-pointer-table
scan
(`decompiler/crates/kuna-analysis/src/analyzers/aif/mod.rs (code_pointer_table_seeds)`
— every 4-byte-aligned odd word in any allocated section whose masked target lands
in an undefined gap, *and* opens with a frame-establishing Thumb prologue, *and*
passes the same probe), and the AIF gap walk above.

**Pointer-referenced entries** (`ptrentry`, default-off; kuna;
`decompiler/crates/kuna-analysis/src/analyzers/aif/kuna_ptrentry.rs`) re-admits
what the second of those throws away. Measurement over the ARM Cortex-M corpus
found its two shape predicates — a frame prologue, and more than two instructions —
reject the bulk of the pointer-referenced population: 93% of the missed entries
establish no frame at all, and 41% are leaves of eight bytes or less, down to a
bare `bx lr`, which is a perfectly valid Cortex-M exception handler. Deleting the
two predicates is not an option on its own: it admits `ldr pc,[pc,r]` switch tables,
whose slots point *into* the function that holds the table, so a fifth of the new
entries split a real function body — a cost the per-ground-truth-function benchmark
cannot see and a real user pays in full. With the option on, a target is instead
admitted on **containment** evidence: no word referencing it may overlap a decoded
instruction (such a word is an instruction's operand bytes read four-aligned, not a
table slot), and none may lie in the same discovered function as the target itself
(that pairing *is* the switch table). The length floor is replaced by a
terminating-routine check — the same speculative walk, accepting when it reaches a
clean `RET`/computed jump or a call into discovered code with no undecodable byte,
no flow out of the image and no escape into another dark region, with no minimum
instruction count. This is the kuna form of the line Ghidra draws between
`OperandReferenceAnalyzer`, which creates functions from *instruction operands*, and
its data-side sibling `DataOperandReferenceAnalyzer`, which overrides
`createFunctions` to a no-op; kuna cannot use Ghidra's version directly because the
Listing records only control-flow references, so the containment pair recovers the
same discrimination from the code/data partition the walk leaves behind. Table-run
corroboration — requiring a run of consecutive code-pointer words — was measured
and is dominated: the switch tables it targets are runs themselves, so it removes
almost no additional split while costing a fifth of the recovered entries. Unlike
the three scans above, the accepted targets are emitted as an additive entry-fact
stream and **never** re-seed the walk: measured, re-seeding drops hundreds of
already-recovered entries through the same tail-call absorption that constrains
`cortexmvectors` (§1.5), so keeping the pass purely additive makes "never removes
an entry" a property of the wiring rather than of a heuristic. Output-changing
(more functions), hence default-off; ARM-only and Listing-tier, so it is a strict
no-op on every other architecture, with `listing off`, and on the XML datatest path.

**Tail-call entries** (`tailcallentry`, default-off; kuna;
`decompiler/crates/kuna-analysis/src/listing/kuna_tailcallentry.rs (tail_call_entries)`)
closes the walk's other structural blind spot. The recursive-descent walk
(`decompiler/crates/kuna-analysis/src/listing/walk.rs`) makes a new function entry
at a CALL target and treats every other flow target as a same-function successor,
so a routine reached only by a tail `B` is absorbed into whichever function
branched to it and never becomes a function at all — the second largest class of
the ARM entry-recall gap. Splitting at a tail call cannot change *which*
instructions the walk decodes: a function entry is walked, hence decoded, either
way, so moving a target from the instruction worklist to the function worklist
leaves the walk's closure fixed and only grows the function set. The split is
therefore computed **after** the walk, where complete predecessor and region
information is available instead of whatever the worklist order happened to
expose, and — like `ptrentry` — emitted as an additive entry-fact stream that
never rebuilds the Listing. Recognising the tail call is easy; telling one from a
rotated loop head is the whole problem, and the naive rule (split at every
unconditional-branch target whose predecessor ends the flow) measures 39%
precision, splitting a real function body more often than it finds one. Four
guards, each measured on the corpus, take that to 94.6% with no split bodies:
every predecessor of the target must be an unconditional branch (a fall-through or
conditional-branch predecessor means the caller's straight-line code runs into it,
which is ordinary intra-function flow); the branch must **leave the caller's
entry-ordered function region**, so at least one other discovered entry lies
between the branch and its target; the target's flow region must reach a `RETURN`
or a computed jump, the same terminating-routine validity `ptrentry` uses and with
the same absence of a length floor; and the target must not open with a stack
restore, because a function does not begin by tearing down a frame it never built
— that shape is the caller's shared epilogue. The region crossing is the
load-bearing one: dropping it costs 43 points of precision and splits over five
hundred real bodies, while a stack-discipline model (reject a branch taken with an
unmatched `PUSH`/`SUB SP` still open) was implemented, measured, and dominated by
it on both precision and recall. As with `ptrentry`, the region is the
entry-ordered one — the nearest preceding discovered entry — which is the
granularity the tier has and errs conservative on a sparsely discovered image.
Output-changing (more functions), hence default-off; ARM-only and Listing-tier, so
it is a strict no-op on every other architecture, with `listing off`, and on the
XML datatest path.

**Literal-pool inference** (`poolentry`, default-off; kuna;
`decompiler/crates/kuna-analysis/src/analyzers/aif/kuna_poolentry.rs`) is aimed at
the gap walk itself rather than at what the walk misses. AIF advances its cursor by
**one byte** on a reject, with no instruction-alignment filter, because the
undefined-gap query it drives is byte-granular by construction. An ARM PC-relative
literal pool is data, so it *is* an undefined gap, and the cursor probes every byte
of it. On a Cortex-M image the pool words are SRAM addresses `0x2000_xxxx` whose
high halfword decodes as `movs r0,#0`, which clears the two-mnemonic fingerprint
gate as reliably as a real prologue does; AIF therefore accepts an entry one
halfword *before* the real function, falls through into it, reaches its return, and
on accept jumps the cursor past the whole body — so the true entry is never probed
at all. In A32 there is no halfword granularity and conditional execution makes
almost any word a legal instruction, so the same mechanism plants the phantom on the
pool word itself. Upstream Ghidra does not have this defect and needs no equivalent
of this pass: its reference analyzer defines pc-relative literal targets as **data**
before AIF runs, so those bytes are not an undefined gap there. kuna's Listing has no
literal-pool data-definition step, and this pass reconstructs the missing definition
after the fact.

The reconstruction is **reference-driven**. A word counts as a literal only when
some instruction actually loads it: either the resolved absolute `[0x…]` operand the
ARM disassembly prints for `ldr rN,[pc,#imm]`, or the unresolved `[pc,#imm]` form
that `vldr`/`ldrd` print because they compute the target in the semantic body — plus
the second word of a 64-bit literal, which nothing loads on its own. Completing that
second form is not a detail: without it every pool holding a float or a 64-bit
constant under-runs and the additive consumer below plants its entry *on* a pool
word, which is the difference between 19 split bodies at 89.7% precision and one at
98.4% over the measured corpus. The `[pc,#imm]` base needs the decode mode, which is
read from the engine's context database — the same `TMode` the bytes at that address
were decoded under, whichever pass painted it — so a language with no such context
answers "no mode" and the form is disabled outright, which is one of the reasons the
predicate is vacuous off ARM. The scan reads the decoded Listing **and** the
speculatively-decoded bodies of the gap-discovered routines, because a pool
sandwiched between two gap-discovered functions is referenced only from inside one
and a Listing-only scan silently finds nothing at exactly the shape being targeted.
A pool is then a **maximal run of adjacent referenced words**: unreferenced words
break the run, which makes the inference strictly more conservative than an ELF `$d`
mapping-symbol oracle, and bridging them was measured and rejected — a bridged run
swallows short real functions and destroys reachable bodies.

Two consumers hang off that one predicate, and they rest on different warrants. The
**recall** consumer emits an entry fact at the first address after a pool that abuts
a *return-class* terminal, when that address is still undefined and passes AIF's own
fingerprint and valid-subroutine tests. The return class is what separates an
inter-function pool, which follows a `bx lr` or `pop {..pc}`, from an intra-function
pool, which follows the unconditional branch the compiler emits to jump over it; and
because the fact is purely additive and never re-seeds the walk, "never removes an
entry" is a property of the wiring here exactly as it is for `ptrentry` and
`tailcallentry`. The **precision** consumer drops an AIF accept that lies inside an
inferred pool — but only when that pool's end carries a replacement entry, one this
pass just added or one another stage already found. That pairing clause is the whole
safety argument: the predicate's soundness (no accept inside an inferred pool was
ever a real function address, across 4,220 removals on the measured corpus) says
nothing about whether the *body* the phantom was decompiling survives, and unpaired
suppression leaves 531 real functions with no entry at any address while paired
suppression leaves zero. A paired removal is a MOVE, which restores a wiring-level
guarantee to the half that removes.

One residue is disclosed rather than gated away. When a literal reference resolves
onto the first word of a function the Listing never decoded, the inference cannot
tell that word from a pool word, and the entry moves four bytes into a real body.
It happens once in the measured corpus, and the only guard that removes it —
refusing to emit at a known branch target — costs 108 of 189 recovered entries, so
it is dominated. Output-changing (it both adds and relocates functions), hence
default-off; ARM-only in effect and Listing-tier, so it is a strict no-op without
`listing`, without `aif`, on the XML datatest path, and on every architecture whose
constants live in `.rodata` rather than in `.text` interstices.

Driver defaults (kuna): `kuna decompile-all` and `kuna decompile` inject
`option listing on` unless the caller names `listing` (DIV-15/DIV-22) — without it
the default-on no-return propagation is a structural no-op and a stripped binary's
unnamed exit wrappers swallow the functions after them. Under the `fast` preset
(DIV-41), those full-tier injections stay off and `fast_funcdisc` is on for
unfiltered `decompile-all`, `decompile-project`, and `functions` inventory runs.
An explicit address selection suppresses the preset-provided walk unless the
caller spells `--option fast_funcdisc on`; name selection retains discovery so
a generated `sub_<addr>` name can resolve. Selection remains exact even when
analysis is forced on. Under `reliable`, `kuna functions` keeps the Listing off: metadata-only
name enumeration gains nothing from the 0.21 s → 5.7 s full decode measured for a
stripped tar (DIV-15). The console and XML datatest paths never build either model
by default, which keeps every parity gate byte-identical.

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

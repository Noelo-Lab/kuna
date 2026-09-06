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
`KUNA_RELOCREBASE` (`relocrebase`), `KUNA_DYNRELOCS` (`dynrelocs`),
`KUNA_MSVCFPCONST` (`msvcfpconst`), `KUNA_PDATACHAINED` (`pdatachained`),
`KUNA_MACHO_ARM64E` (`macho-arm64e`),
`KUNA_MACHO_SLICE` (`--slice`). For those,
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
"initial address unmapped → `DataUnavailError`" contract in `loadFill`. One
deliberate correction inside that contract: the buffer's `bufoffset` is claimed
at the top of a fill, *before* a byte is read, and a failed fill **releases it
again** (upstream throws with it still claimed). Left claimed, the buffer's own
fast path answers every later request within 512 bytes of the failed address out
of a buffer that was never filled — stale bytes, reported as a successful read.
Nothing upstream reads twice near a failure, which is why it never surfaced
there; a caller that probes addresses in order (the extern-slot
classification of §0.2) walks straight into it. The mapping
unit is the ELF **`PT_LOAD` segment** (what the OS actually maps), not the BFD
section list. Where upstream returns a BFD target string for the Java side to
re-map, kuna resolves the SLEIGH language id directly off the object header
(machine + endianness + class → e.g. `x86:LE:64:default:gcc`). The loader's symbol
stream — defined FUNC symbols plus the resolved import stubs of §1.3 — is
`@VERSION`-stripped, demangled (§1.4) and **character-sanitized** before each
name is installed as a `FunctionSymbol`, and the loader's read-only section ranges are applied to the
symbol-table property map eagerly at bootstrap (loader markup, not a gated pass):
they are what lets the printer prove a constant points into read-only memory and
render a string literal.

(kuna) **An unusable section table is dropped, not fatal**
(`decompiler/crates/kuna-analysis/src/loader/elf_shdr.rs
(tolerate_unusable_section_table)`). An ELF's section table is link-time metadata;
what the loader obeys — the entry point and the `PT_LOAD` map — lives in the ELF
header and the program headers, which is why `readelf -l` still prints a full
segment map for an image whose `e_shoff` is garbage. `object` nevertheless
validates the section table eagerly inside `File::parse`, so a single out-of-range
`e_shoff`, a wrong `e_shentsize`, or an `e_shstrndx` naming no section rejected the
whole image and every kuna surface exited 1 with "not in recognized object file
format". Packers, `sstrip` and CTF authors all produce that shape deliberately. The
image bytes are therefore normalized once, at the same canonical read point as the
Mach-O fat-slice peel, by clearing `e_shoff`/`e_shnum`/`e_shstrndx` — the encoding
of "this ELF has no section table" — so the loader and every analysis pass below
see the same recovered view. The test is pure header arithmetic and runs before any
parse, so an image whose table is usable is passed on byte for byte; the rewrite is
kept only if the rewritten copy actually parses, so corruption elsewhere still
reports `object`'s own error rather than a misleading one about the section table.
What was dropped, and what survived it, is reported on stderr. The CLI surfaces
that parse the image themselves rather than through the loader (`strings`, `xrefs`,
`decompile-graph`, the call graph) read it through the same normalization
(`elf_shdr (read_image)`), so a recovered image is recovered everywhere.

**Character sanitizing** (`symbolnamechars`, `off|safe|ident`, default `safe`;
`decompiler/crates/kuna-decomp/src/p0_knowledge/kuna_symbolnamechars.rs`) is the
last step of that name reduction, and it is the only one that treats the name as
*bytes*. A symbol name is unvalidated binary data, and it is printed verbatim
into the `// Function:` header comment, the `.h` prototype, the definition, every
call site and the `.asm` label. Three shapes therefore restructure the C document
rather than merely look odd — a `*/` closes the header comment early and turns
the rest of the line into code, a raw `0x0a` splits each of those five renderings
across two lines, and a `//` comments out the remainder of the line it lands on —
and a fourth breaks identity rather than syntax: the name is decoded with
`String::from_utf8_lossy`, so two symbols differing only in an invalid byte
become the same `String` and the export carries two definitions and two
prototypes with one name.

`safe` (the default) rewrites exactly that structural set and nothing else: an
ASCII control byte (`0x00`–`0x1F`, `0x7F`), a `"`/`'`/`\`, a `*` or `/` that
forms `*/`, `/*` or `//` with its neighbor (both characters of the pair), and
every byte of an invalid UTF-8 sequence. Each becomes its `_x<hh>` hex escape
rather than a single `_`, because a single `_` is not injective — `a"b`, `a'b`
and `a\nb` would all collapse to `a_b`, reproducing the redefinition defect with
a different trigger — and the escape costs nothing legible, since `safe` fires on
no name a real toolchain emits. A lone `*` or `/` is left alone (it is not a
comment delimiter), as are `.`, `$`, `@`, `-`, `+`, `<`, `>`, `(`, `)`, `;`, `{`,
`}` and all valid multi-byte UTF-8; `::` survives intact, because §0.4's scope
splitter reads it. `ident` additionally reduces each `::` component to
`[A-Za-z0-9_]` through the same routine the Itanium RTTI recovery uses for a
class name (`kuna_symbolnamechars (sanitize_ident_chain)`, called unconditionally
by `kuna_itaniumrtti (sanitize_class_name)`), which is what a reader who intends
to *compile* the export wants; it is not the default because the most common
name in the wild that is not valid C is gcc's clone suffix
(`err_fatal.constprop.0`, `main.part.1`, `add_fdes.cold`), which appears on most
`-O2` binaries and which `safe` is a measured no-op on. `off` restores the
verbatim bytes for someone auditing what a binary literally claims.

The sanitizer runs at the **mint** — after the demangler, so it sees the reduced
name rather than the `_ZN…` envelope, and before the scope splitter, so it never
contends with `symbolnamerepair` over the same empty component — and it covers
the second channel a name arrives through as well: an analysis pass's recovered
name (a DWARF `DW_AT_name`, a Go `pclntab` entry, a PDB public symbol) is
sanitized in `kuna-analysis`'s pass driver
(`decompiler/crates/kuna-analysis/src/pass.rs (AnalysisOutput::sanitize_names)`)
before the commit boundary of §1.4 sees it. Like the other gates consumed inside
`load file` it is carried by a process environment variable. Print-time
sanitizing would be the wrong seam: §0.4 explains that the string in the symbol
table is the key `kuna decompile <name>` and `load function` are passed, so a
name only the printer fixed would be one the tool could no longer be asked for.

The **data** half of those same two symbol tables is read alongside the function
half (`loadimage_object.rs (data_symbols)`): every defined, named `STT_OBJECT`
entry with a non-zero `st_size`, deduplicated by address, `.symtab` before
`.dynsym`. Zero-size entries are dropped because the linker's section-boundary
markers (`__bss_start`, `_edata`, `_end`) are exactly the sizeless ones, and a
sizeless symbol would plant a name on the first byte of whatever object follows
it. Each surviving entry becomes a named `undefined<size>` global — the same
shape §1.4's DWARF data globals use, and for the same reason: a size-1 entry does
not contain a 4- or 8-byte access, so the printer's covering-symbol query would
miss and fall back to `dat_<addr>`. The behavior matches IDA Pro and Ghidra,
which both name data objects from the symbol table independently of any debug
info, and it is what makes `fprintf(stderr, ...)` read as an error path instead
of `fprintf(dat_61a0, ...)` on a stripped binary (GH-184). Per the standing
options contract it is gated by **`datasyms`** (`--option datasyms on|off`,
default ON, DIV-26/DIV-76): the stream is collected at `load file` but committed
at `read symbols`, after the option lines are applied, so the gate is the plain
`Architecture` flag the commit consults — no env bridge is needed on either CLI
path. Off drops the stream at the commit and restores the raw `dat_<addr>`
rendering exactly.

A **declared extent is never trusted to be representable.** `st_size` is a 64-bit
field of the image that no header check validates, while the type factory sizes
types in a 32-bit `int4`, so the commit clamps the declared size into `1 ..=
int4::MAX` **before** narrowing it — the same shape §1.4's DWARF globals use, even
though those arrive already bounded. Clamping after the narrowing inspects the
wrong number and lets two whole classes of size through: one whose low 32 bits are
zero becomes a size-0 type, which the symbol table rejects, and one whose low 32
bits have the sign bit set becomes a negative size. Neither cost only the symbol.
`engine.rs (commit_analysis_output)` applies its arms in place and propagates the
first error, so a rejected symbol abandoned every later arm — prototypes, context
paints, tracked registers, call-fixups, DWARF locals and line comments — and the
pending stash is taken by then, so a second `read symbols` cannot retry; a
negative size was worse still, indexing the type factory's fixed-size caches out
of bounds. `decompiler/crates/kuna-decomp/src/substrate/dtype.rs` therefore also
refuses a negative size at both cache lookups, as an ordinary error rather than an
invariant, because sizes reach them from image bytes. The clamp is saturating, not
narrowing: a hostile extent stays hostile — a symbol claiming ~2 GiB still covers,
and so shadows, every unnamed address above it, exactly as a legal `st_size` of
`0x7fffffff` already does — but it is now a symbol with a wrong extent rather than
a load that fails or a process that stops.

Precedence is what makes this safe to add underneath the existing sources. The
loader's data symbols commit **last** (`engine.rs (commit_analysis_output)`),
after the DWARF globals and after the detected string literals, and each is
skipped where a function or a covering data symbol already sits. So a
DWARF-described global keeps its DWARF-recovered extent and a detected string
keeps its `char[N]` typelock; the loader arm only fills addresses neither source
reaches. That residue is the interesting one: a copy-relocated libc extern
(`optind`, `stdin`, `stdout`, `optarg`) has a real `.bss` address and a `.dynsym`
entry but no DIE in the program's own `.debug_info`, so nothing else could name
it. Relocatable objects are excluded — `reloc_object` rebases only the function half
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

- **Relocatable objects** (angr, `relocobjects`, default-on) — a pre-link object
  does not say where its bytes live, and each format fails that differently. An
  ELF `.o` has no program headers, so the faithful loader maps zero bytes and
  every lift fails. A COFF `.obj` does present segments, but stacks every section
  at VMA 0: the faithful loader maps whichever sorts first and every symbol
  collapses onto address 0, so with MSVC function-level linking (`/Gy`, one COMDAT
  `.text` per function — the default for real builds) all but one function
  disappear. `decompiler/crates/kuna-analysis/src/loader/reloc_object.rs (RelocLayout)`
  reproduces angr CLE's relocatable backend for both: lay each memory-resident,
  non-empty section out above `0x400000` (`RELOC_BASE`, matching CLE so addresses
  line up with angr's), apply the relocations, rebase defined symbols, and bind
  each undefined extern to a synthetic call target in an extern area above the
  sections so calls render by name. The relocation encoder handles generic
  absolute, relative, PLT-relative, and image-offset fields at 8/16/32/64 bits in
  the object's byte order, plus the instruction fields and ABI formulas for ARM
  `CALL`/`JUMP24`/Thumb branches/`REL32`/`PREL31`, AArch64 branch/page/low-12
  forms, and PowerPC64 `REL24`/TOC forms. An entry that cannot be encoded is left
  untouched and classified by reason (unsupported, unresolved target, missing
  TOC, section bounds, required veneer, alignment, range, or invalid encoding).
  The loader reports exact failure totals in at most eight groups with three
  samples per group, once per public load, so machine-readable output remains
  valid and stderr stays bounded. The
  result feeds back into `ObjectLoadImage` as the same segments/sections/funcsyms
  triple the linked path produces. The loader also retains the original section
  index, section name, section-relative offset, symbol binding, and
  defined/undefined provenance beside each synthetic VMA. Front-ends expose that
  coordinate as `.section+0xOFFSET` or `SECTION_INDEX:0xOFFSET`; a bare numeric
  selector first means a mapped synthetic VMA, then falls back to a raw function
  offset only when exactly one definition matches. Name and raw-offset collisions
  report every candidate instead of taking symbol-table order, and only a symbol
  marked undefined is classified as external. Loaders that publish no section
  records, including the XML corpus loader, prove a numeric VMA by probing one
  byte from the load image instead. Which sections are memory-resident
  is the one question that stays per-format — ELF's `SHF_ALLOC` bit and COFF's
  `Characteristics` content bits minus the link-time-only sections (`.drectve`,
  `LNK_REMOVE`, the discardable `.debug$S`/`.debug$T`) — and it is asked through
  `ObjectFormat::is_alloc_section`, alongside `ObjectFormat::relocatable_layout`,
  which decides whether a given file needs this path at all.
  A REL-style relocation table (COFF, 32-bit ELF) stores its addend in the field
  being patched rather than in the entry, so the in-place value is read back and
  added; a RELA entry carries the whole addend and reads back zero.
  ARM function symbols additionally retain the ABI state bit while branch
  relocations are applied. `R_ARM_CALL` and `R_ARM_THM_CALL` convert `BL` to
  `BLX` (or back) when a typed target crosses the ARM/Thumb boundary. A
  cross-state jump cannot make that transition in place, so it is left
  untouched and reported as requiring a linker veneer instead of being encoded
  as a branch in the wrong instruction set. Untyped and undefined targets do
  not infer state from their synthetic slot address. AArch64 branch, page, and
  low-12 relocations preserve the instruction's opcode/register fields, while
  PowerPC64 `REL24` and TOC-family relocations preserve big-endian instruction
  layout and DS-form low bits.

  An undefined symbol reached through any branch or call instruction field —
  not only a call-spelled one — is bound to a named extern slot. A tail call is
  spelled as a plain jump relocation (`R_ARM_JUMP24`, `R_AARCH64_JUMP26`, a
  PowerPC64 `REL24` with its link bit clear), and the branch is patched to point
  at the synthetic slot either way; leaving that slot unnamed makes the *calling*
  function undecompilable, because the flow walk follows the branch into memory
  the layout never backed. The call/jump distinction itself is kept where it is
  load-bearing — the ARM `BL`/`BLX` interworking rewrite — and is not what
  decides whether an extern is named.

  Laying the object out synthetically splits the address space in two, and every
  pass in this chapter reads the *other* half: each one re-parses the file through
  its own `object::File`, which reports the **pre-link, section-relative**
  addresses the linker has not yet assigned. Mixing the two in one inventory is
  what produced a phantom `sub_<section-offset>` beside every real function, a
  single DWARF function at address 0, and string literals that never attached to
  the loaded image at all. `relocrebase` (kuna, default-on) closes that by
  rebasing the analyzer tier's **input** rather than each output fact — necessarily
  so, because a fact is a bare address by the time it reaches `AnalysisOutput`, and
  in a relocatable object every section sits at address 0, which makes `.text`+0x20
  and `.rodata`+0x20 the same number. Worse, the fields that matter are not offsets
  at all until their relocation is applied: an unrelocated `.eh_frame`
  `initial_location` reads back as its own section offset, and an unrelocated
  `DW_AT_low_pc` reads 0 for every subprogram (as does every `DW_FORM_strp`, so the
  whole object's DWARF collapses onto one function named after whatever string sits
  at `.debug_str`+0).
  `decompiler/crates/kuna-analysis/src/loader/kuna_relocrebase.rs (rebased_view)`
  therefore re-presents the object to the tier: each laid-out section carries the
  loader's own relocated bytes and its load VMA (ELF `sh_addr`, COFF
  `VirtualAddress`); each section the layout skipped — every `.debug_*` table — has
  its relocations applied here, resolving a target in a laid-out section to that
  section's load VMA and a debug-to-debug target to its own section-relative offset,
  which is what a single-object link leaves in place; and each ELF symbol defined in
  a laid-out section has its `st_value` shifted by **its own section's** delta,
  since the layout is non-contiguous and there is no single global offset (a COFF
  symbol needs no shift — it is reported as `VirtualAddress + value`, so the section
  write already moved it). Every pass then produces an already-rebased fact with no
  source change of its own.
  A field with no relocation still yields an address in no laid-out section, so
  `kuna_relocrebase (retain_in_image)` **drops** exactly those rather than letting a
  pre-link address through — that is the phantom class. The single exception is a
  no-return fact, which the commit resolves by name when its address does not
  resolve (an undefined `exit` in a `.o` has never had one); it is kept with its
  address zeroed. The Listing/xref tier takes the other answer and declines outright
  for a synthetically laid-out object (§1.6): it exists to find functions an image
  has no symbol for, and a pre-link object always carries the symbol table the
  linker is about to consume.

  Binding an undefined symbol to a synthetic slot resolves the *reference* and
  loses the *value*, and for one class of symbol the value was never elsewhere to
  begin with: MSVC never encodes a floating-point literal into the instruction
  stream — x87 and SSE both load one from memory — so the compiler emits each
  literal as a COMDAT whose **name spells it**. `__real@8@3ffec90fdaa22168c000`
  is π/4. COMDAT folding then keeps the definition in exactly one translation
  unit, so in every other object that symbol is undefined: no section, no bytes,
  a slot with nothing behind it, and an expression written entirely in opaque
  addresses (`(… * dat_402020 + dat_402040) * dat_400ae0`). `msvcfpconst` (kuna,
  default-on, env-bridged,
  `decompiler/crates/kuna-analysis/src/loader/kuna_msvcfpconst.rs (plan)`) reads
  the value back out of the name. Three spellings are accepted:
  `__real@<size>@<20 hex>`, an x87 80-bit extended datum (a 16-bit
  sign/exponent field then a 64-bit mantissa carrying its **explicit** integer
  bit) plus the storage width the program loads it at — `4` for `float`, `8` for
  `double`; and the two bare-bits forms `__real@<16 hex>` (IEEE double) and
  `__real@<8 hex>` (IEEE float, which is what MSVC has emitted for a `float`
  literal since VS2005 — the 20-hex form is the VC6-era one). The decode is
  exact rather than approximate: the source constant was a `float` or a `double`
  before the assembler widened it, so at most 53 of the mantissa's 64 bits are
  set and the `f64` image is lossless. Every x87 encoding class with no faithful
  `f64` image is **refused** rather than approximated — an Inf/NaN exponent, a
  denormal or pseudo-denormal (whose true scale is one binade away from the
  normalized formula), an unnormal, and any value outside `f64` or, at `@4@`,
  outside `float` — as is every other mangling, `__xmm@`/`__ymm@` included: a
  wrong 16-byte datum is worse than an honest `dat_<addr>`.

  As with `dynrelocs` below, decoding is only half of it. The undefined half
  gets the decoded bytes materialised as a segment at its extern slot, which is
  what makes the address readable at all; but the *defined* half needs nothing
  materialised and still renders `dat_<addr>`, because folding a read-only global
  is gated program-wide by `option readonly` (default off). Both halves are
  therefore reported as `ObjectLoadImage::dynreloc_const_ranges` — the same
  narrow "constant by construction, not by policy" exception list `dynrelocs`
  fills on the linked path, carried to `Architecture::dynreloc_const` and folded
  by `ActionVarnodeProps` with global propagation still off (§3.4). Listing only
  one half would be worse than listing neither: one operand of an expression
  would come out a literal and the operand beside it stay opaque. A defined
  COMDAT's mapped bytes are cross-checked against its own name before its range
  is admitted, which is also what keeps the relocatable-object fidelity hazard
  away from this path — a read-only section in a `.o` holds *pre*-relocation
  bytes, but a `__real@` COMDAT carries no relocation, and a disagreement between
  the bytes and the name drops the range with a warning rather than folding it.
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
  `option ifuncfpret` (default off, x86-64) adds a second pass that DOES name those
  IRELATIVE IFUNC stubs — `ifunc_<resolver>`, keyed off the `R_X86_64_IRELATIVE`
  resolver-address map — so a tail `jmp` to a glibc math/mem/str dispatcher's stub
  is recovered as a `tailcalljump` to a discovered function instead of flowing into
  the stub and rendering `(*dat_...)(...)`; the FP-return-type recovery it unblocks
  is a Ghidra-divergent follow-up (`docs/features/ifuncfpret/proposal.md`).
  Two ABIs need special handling: PowerPC (ELFv2 `.plt` is a NOBITS data table, not
  decodable code; PPC32 uses its own secure-PLT stub shape), and **MIPS**, which
  has no PLT and no jump-slot relocations at all — its resolver walks the
  `.MIPS.stubs`/GOT layout from the dynamic table (`DT_MIPS_LOCAL_GOTNO`/
  `DT_MIPS_GOTSYM`) and marks the external GOT slots constant, so with
  read-only propagation the `lw $t9, off($gp); jalr $t9` sequence folds to the
  named import (the bootstrap turns `readonlypropagate` on for MIPS only).
- **Linked-image dynamic relocations** (kuna, `dynrelocs`, default-on,
  env-bridged): the loader maps the `PT_LOAD` bytes the *linker* wrote, which is
  not the image a process runs. Every slot filled by a dynamic relocation —
  `R_*_RELATIVE`, `R_*_GLOB_DAT`, `R_*_JUMP_SLOT` — is left at zero for the
  run-time loader to complete, and in a PIE (which is the default link mode of
  every current toolchain) that is the whole `.got` plus every relocated function
  pointer in `.data.rel.ro`. A call through such a slot therefore reads a null
  target and can never resolve, which is the `(*dat_<addr>)(...)` rendering of a
  callee that is a *named function in the very same image*. Zero is not an
  ambiguity to be judged; it is a byte the run-time image never holds.
  `decompiler/crates/kuna-analysis/src/loader/kuna_dynrelocs.rs (resolve)` walks
  `.rela.dyn`/`.rel.dyn`/`.rela.plt` and computes the value the dynamic loader
  would store: for `RELATIVE` the image's load bias plus the addend (kuna maps a
  linked image at the vaddrs it declares, so the bias is zero and the value is the
  addend); for `GLOB_DAT`/`JUMP_SLOT` the symbol's address, **and only when that
  symbol is defined in this same image**. An undefined one is an import — its
  value lives in another object, there is nothing to write, and the PLT/import
  naming above already covers the call — so it is skipped and that path is
  untouched, lazy-binding stub contents included. A `REL` table (32-bit ELF)
  carries no addend field, so the in-place word is read back as the addend, the
  same in-place convention `relocobjects` uses. Architectures are named by their
  relocation triple: x86-64, AArch64, i386 and ARM; a machine with no entry
  produces nothing rather than guessing at a number that means something else
  there.

  Applying the relocation is only half of it. `.got` is `SHF_WRITE`, so nothing
  downstream would trust a value read out of it, and the constant fold of
  read-only storage is gated program-wide by `option readonly`
  (`readonlypropagate`, default off — turning it on would fold every `.rodata`
  read in the program, a far larger change than this one). The narrow warrant is
  `PT_GNU_RELRO`: the linker's own statement that the segment is `mprotect`ed
  read-only once startup relocation finishes. So the written slots that RELRO
  covers are reported twice — through `getReadonly` (which paints
  `Varnode::readonly` as for any read-only section) and separately as
  `ObjectLoadImage::dynreloc_const_ranges`, carried to
  `Architecture::dynreloc_const` and into every per-function handle, where
  `ActionVarnodeProps` folds a read-only varnode inside one of those ranges even
  with global propagation off (§3.4). The halves are useless apart: relocating
  without the constancy leaves the load unfolded, and declaring constancy without
  relocating would fold the call target to zero. Slots outside RELRO — a
  `RELATIVE` pointer in ordinary `.data` — are still filled in, because that IS
  the value at process start, but are never declared constant, because the
  program may legitimately overwrite them.

  This is a different path from `relocobjects`/`relocrebase` above, which own the
  *pre-link* `ET_REL` object; a relocatable object's relocations are applied by
  the layout pass and this walk does not run for it.
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

Each canonical entry also carries a byte **extent**, so the inventory answers
"how big" as well as "what is here" and a caller can order a binary's functions
by weight without decompiling any of them. kuna's model of a function is its
entry — the Listing is keyed by entry VMA and nothing in it records a body — so
the extent is reconstructed as the address-contiguous clip from the entry to
whichever comes first: the next canonical entry, or the end of the CODE section
containing the entry (`decompiler/crates/kuna-console/src/funcextent.rs`). This
is the same reconstruction the FID extent generator
(`decompiler/crates/kuna-analysis/src/analyzers/fid/extent.rs`) and the
discovered-no-return pass already apply where they need a body from an
entry-keyed model, and it reuses the entry list and the loader section table
rather than decoding anything, so the metadata-only `functions` surface stays
metadata-only.

The number is an upper bound, not the exact body: the clip runs to the neighbour,
so inter-function alignment padding is counted in, and against ELF `st_size` over
the symbolized fixture corpus it is never short. An entry in no CODE section — a
pointer slot, an undefined external — has no body to measure and reports zero,
the same value a synthesized entry carries when there is no program to measure
against. The loss is that the clip is address-contiguous rather than
flow-reachable: an outlined cold half living past the next entry is attributed to
its neighbour. Every whole-binary surface reports this one number under the one
name, including the decompiling ones; their alternative, the recovered
`Funcdata::get_size()`, is the *requested* flow bound rather than a measurement,
and a whole-binary run always requests an unbounded extent.

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
- **Wide strings** (`widestrings`, the `StringsAnalyzer` `allCharWidths` arm,
  `decompiler/crates/kuna-analysis/src/analyzers/strings/kuna_widestrings.rs
  (scan_wide_strings)`): the same matcher over 2-byte little-endian code units —
  the same printable-ASCII recognizer applied to each unit's low byte, the same
  require-NUL-end rule, the same minimum length of 5, over the same section set,
  reading units on even addresses only. Each hit commits a typelocked
  `wchar2[len/2]` instead of a `char[N]`, and the character type's size 2 is what
  makes the printer emit the `L` prefix and read the bytes two at a time. Without
  it a UTF-16LE literal is read at 1-byte width as a ONE-CHARACTER string — the
  NUL behind the first unit closes the run — so a wide Windows-API argument
  rendered as its own first character (`LoadLibraryW("n")` where the image says
  `L"ntdll.dll"`). The two widths cannot claim the same run: a wide unit demands a
  zero high byte, so five consecutive 1-byte-charset bytes never occur inside a
  wide run. They are ordered anyway, and the order is the fix rather than a
  detail — the wide facts commit FIRST, because `operand_refs` puts facts into the
  same stream whose run test accepts a *single* visible character, and at a wide
  literal that test reads the first unit plus its high-byte NUL as a complete
  `char[2]`. Whichever fact is planted first wins the commit's occupied guard, so
  the width that read the whole literal has to go first. Scope: UTF-16**LE** whose
  units are all in the 1-byte charset (the Windows-API case); a big-endian or
  non-Latin wide literal is not recovered. Default **on**; `off` leaves the markup
  exactly the 1-byte pass's.
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
  recovery is the name-only walk, byte for byte. (Struct/class **fields** are the
  sibling `dwarfstructs` increment below; before it, a class stayed a named opaque
  and `this->balance` printed as an offset.)
- **Aggregate layout** (`dwarfstructs`, default-on,
  `decompiler/crates/kuna-analysis/src/analyzers/dwarf/kuna_dwarfstructs.rs`) is
  what turns a recovered aggregate from a *name* into a *type*. The mapper used to
  resolve every `DW_TAG_structure_type`/`union_type`/`class_type` to
  `get_type_struct(name)` — a named, empty, **zero-size** shell — and never read
  `DW_AT_byte_size` or walked a single `DW_TAG_member`. That is enough for
  `struct foo *p` to render, and the shortfall was filed as a fields gap; it is
  worse than that, because a zero width is not a conservative answer. The
  x86-64 parameter-storage model reads the size, so a struct passed **by value**
  had no width to classify and its slot degraded to the raw register it arrives
  in (`int take_struct(unsigned long,int)` for `take_struct(P8,int)`), and an
  8-byte struct **return** — a register return on this ABI — was classified as a
  hidden-return-buffer call: a *phantom* `rethidden` parameter appeared in front of
  the real ones and the body then did arithmetic on it. This arm reads
  `DW_AT_byte_size`, walks the `DW_TAG_member` children, places each at its
  `DW_AT_data_member_location` **verbatim**, and recurses each member's
  `DW_AT_type` through the same DIE switch; bitfields come off `DW_AT_bit_size`
  with either the DWARF 4/5 `DW_AT_data_bit_offset` or the DWARF 2/3
  `byte_size` + `bit_offset` spelling, each placed in the smallest byte span that
  covers it — the geometry the compiler's own access agrees with, and the one the
  printer's `.`-versus-`->` test reads. A "bitfield" occupying whole aligned bytes
  of a natural width is not one and goes in as a plain field of that width, which
  is exact on a little-endian target and keeps a known
  `BitFieldPullTransform` divergence (three bitfields sharing one extraction
  chain) out of reach. Offsets are installed through a raw
  field-setting entry point rather than the C packing rules, because the layout is
  the compiler's own answer for the target ABI and re-deriving it would silently
  disagree with the bytes the decompiler reads.

  Two hazards come with populating fields, and both are handled in the naming.
  The type factory interns by `(name, hash(name))` and refuses a second, different
  definition of a name it already holds; while every aggregate was a sizeless
  shell that was invisible, because two shells compare equal. It goes live the
  moment fields exist — and it is not exotic: `rustc -g` names every enum payload
  struct **bare** (`Some`, `Ok`, `Err`), and a five-function Rust witness carries
  four distinct `Some` DIEs of sizes 16, 24, 16 and 12. Aggregates are therefore
  interned under their **parent-qualified** name (the namespace/class ancestry walk
  the C++ arm already had) and, when that name is still held by an aggregate of a
  different size, under a size-suffixed variant; a name held by a non-aggregate is
  stepped over the same way. The second hazard is self-reference: a
  `struct node { struct node *next; }` reaches its own DIE while its fields are
  being built, so the shell is interned **before** the members are walked and the
  inner resolution finds it by name, with the walk guard refusing a re-entrant
  population. LOSS: because an interned type is immutable in kuna, completing one
  mints a new handle, so the pointer the inner frame captured still refers to the
  pre-completion shell — the name renders but the chain is one level shorter.
  `DW_TAG_variant_part`/`DW_TAG_variant`/`DW_AT_discr`, the Rust tagged-enum
  encoding, are not read by this arm; the sibling `dwarfvariants` increment below
  reads them, and with it off a Rust enum recovers its width and no fields. Same
  load-time shape as `typedepth` below: the layout is installed inside
  `load file`, so the live gate is the process env var
  (`decompiler/crates/kuna-decomp/src/p0_knowledge/kuna_dwarfstructs.rs`) that the
  CLI exports before the load, and `dwarfstructs off` is the name-only mapping byte
  for byte.
- **Discriminated unions** (`dwarfvariants`, default-on,
  `decompiler/crates/kuna-analysis/src/analyzers/dwarf/kuna_dwarfvariants.rs`) is
  the arm for the aggregates the member walk above cannot see at all. A Rust
  tagged enum carries **no `DW_TAG_member` of its own**: its layout hangs off a
  `DW_TAG_variant_part`, whose `DW_AT_discr` points at the artificial member that
  IS the discriminant, and whose `DW_TAG_variant` children each carry a
  `DW_AT_discr_value` plus one `DW_TAG_member` naming the variant (`Ok`, `Err`,
  `Some`, `None`) and referring to its payload struct. So `dwarfstructs` alone
  gave a Rust enum its `DW_AT_byte_size` and zero fields — and a field-less
  aggregate is still an aggregate the ABI classifier acts on, so an 8-byte
  `fn(u32) -> Result<u32,u32>` came out with the same phantom `rethidden`
  parameter described above and a 16-byte one wrote its variants as
  `*(uint *)&r->field_0x4`.

  Reading it from DWARF rather than from codegen is the point. The two questions
  a decompiler cannot answer from shape are *is this a discriminated union* and
  *which variant is which*: two return paths storing different constants at
  offset 0 is equally a `#[repr(C)] struct {kind, val}`, a `(u64,u64)` tuple, a
  bitmask pair, or a `&'static str` fat pointer whose "discriminant" would be a
  `.rodata` address. `DW_AT_discr` and `DW_AT_discr_value` are the compiler
  stating both answers, so no name installed here is an inference — though a name
  DWARF states is still only installed where the union model can select it
  unambiguously, which is the second limitation at the end of this bullet.

  The recovered type is a struct of the discriminant plus a **union** of one
  payload struct per variant. A union's members all sit at offset 0, which is
  exactly a variant overlay, so this uses the existing type model rather than
  adding a `type_metatype` (that enum is matched at ~1,700 sites in this
  workspace — `grep -ro 'type_metatype::' --include=*.rs decompiler/crates | wc -l`
  reports 1678 — mostly non-exhaustively, so a new variant would compile clean and
  behave wrong;
  `sub_metatype` is a contiguous propagation sort key; and `metatype2string`
  writes a fixed vocabulary onto the Ghidra wire). The overlay
  begins at the lowest offset any variant places a field at, and each facet's
  fields are **re-based** to it: DWARF gives a variant's payload struct the width
  of the whole enum with its members at their absolute offsets
  (`Result<u32,u32>::Ok` is 8 bytes with `__0` at 4), which describes an overlay
  at offset 0 and cannot be placed beside a `tag` field at the same offset.
  Every name minted — the facets and the overlay union — is derived from the
  enum's own parent-qualified name and goes through the same collision policy
  `dwarfstructs` established, because rustc names payload structs bare and the
  collision is not hypothetical: the committed `dwarfvariants_x86_64` fixture
  alone carries **3 structure DIEs named `Some` at two different widths** (8 and
  16), and a std-linked `rustc 1.90 -C debuginfo=2` witness with 152 variant parts
  carries 61 named `Some` across 8 byte sizes (0/8/12/16/24/32/48/64), 61 `None`,
  and 31 each `Ok`/`Err` across 7. A suppressed facet, whose name is derived from
  its offset rather than from the variant, can collide inside a single enum
  (`Tree::field_0x8` for both `Leaf` and `Node`); identical layouts then share one
  interned struct and differing ones are given a numeric suffix, because the ABI
  classifier common-refines the union's members.

  Two shapes get specific treatment. A **fieldless** variant (`None`, `Nil`, a
  unit variant) overlays nothing and gets **no union member**: an empty struct of
  the overlay's width is indistinguishable to the union-field scorer from the
  facet that does carry the payload, and it wins the tie by declaration order.
  Ablating the exclusion, a std-linked `rustc -g` witness writes an `Option<i64>`
  payload as `v13.payload.None = ...` and reads drop glue as
  `(*a0).dropfn.drop.None` — 2 functions of 612, measured, which is what fixed it
  this way. The variant is not lost — its name and its discriminant value
  are on the side table, which is where a `match` renderer reads them, and there
  is no payload for a field path to reach. A **niche-encoded** enum, where a
  `DW_TAG_variant` carries no `DW_AT_discr_value` at all (it is the default
  variant: every value the others did not claim) and the discriminant's bytes
  overlap the payload, has no byte range that is only the tag — so the recovered
  type is the **overlay alone**: the union, at the variants' own DWARF offsets,
  under the enum's own name, with no enclosing struct, because a `tag` field would
  have to sit at an offset a variant already owns. The geometry still reaches the
  side table, marked as a niche.

  The geometry — the discriminant's offset and width, each variant's name,
  discriminant value and absolute field offsets, and whether the encoding is a
  niche — is recorded in a side table on the `TypeFactory`
  (`decompiler/crates/kuna-decomp/src/p0_knowledge/kuna_dwarfvariants.rs`). It is
  the `kuna_wire_symbols` arrangement: nothing in the analysis reads it, no
  `Datatype` points at it, and it is not encoded onto the wire, so filling it
  cannot perturb emitted C. It exists so a later pass can render `match` /
  `if let` / `Ok(v)` from the compiler's own answer.

  Every guard REFUSES rather than guesses, and every refusal ends at the same
  answer the `dwarfstructs` path above gives — a named aggregate with the enum's
  byte size and no fields — but by two different routes. Refused BEFORE anything
  is interned, so the DIE simply falls through: no `DW_AT_discr`; a variant with
  anything other than exactly one named member; two variants with no
  `DW_AT_discr_value`, or with the same one, or with the same name; a payload
  struct carrying its own `DW_TAG_variant_part` (a nested variant part, which the
  single-level overlay cannot describe — rustc 1.90 emitted none across the 75
  variant parts in the two witnesses measured for this change); no variant with
  any field at all (a C-like enum, which rustc emits as a
  `DW_TAG_enumeration_type` instead); a discriminant whose type is not
  integer-shaped, zero-width, or wider than the enum; a zero-width or absent
  `DW_AT_byte_size`; and a `DW_AT_declaration` DIE.

  Refused AFTER the shell is interned — it has to exist before the members are
  walked so a recursive payload has something to point at — the answer cannot be
  "leave it to `dwarfstructs`", because a zero-size incomplete type is already
  sitting in the factory under the enum's own name, and downstream that degrades
  to `void`. Those refusals instead SEAL that shell at the enum's
  `DW_AT_byte_size` with no fields, which is byte for byte the `dwarfstructs`
  answer: a member that would extend past the enum; every facet's fields being
  unbuildable, which would leave a zero-member union describing nothing; and the
  overlay union's name being unmintable or any of the three completions being
  refused by the factory. Same load-time env-var gate as `dwarfstructs`, and
  gated on `dwarfstructs` itself as well — this arm extends that one, so
  `dwarfstructs off` stays exactly the pre-DIV-86 name-only mapping its own row
  promises.

  **The limitation is the channel.** This needs full debug info
  (`-C debuginfo=2`, cargo `debug = true`). Where a binary's DWARF carries no type
  DIEs the arm is not degraded, it is inert: it recovers nothing and attempts no
  fallback, because the only available fallback is the shape inference above. A C
  program has no variant part at all, so the arm never fires on one.

  **The second limitation is the union model, and it decides what may be named.**
  Representing a variant overlay as a union means a member selects itself by
  OFFSET; the discriminant is never consulted. For a tagged enum that is not a
  corner case but the definition of the encoding: every payload variant begins
  immediately after the tag, so `Ok` and `Err` are at the same offset, always, and
  the facet the union-field scorer picks is not evidence of anything. Measured, on
  a `Result<u64,u64>` witness the label was not merely uncertain but consistently
  false — `Ok` was printed on both arms of the producing `if`/`else` and on the
  consumer's `Err(e) => e + 100`, and `Err` appeared nowhere in the binary.

  So a variant name is installed **only where it is forced**, and the rule is
  applied per byte range rather than per variant:

  - a facet keeps its `DW_TAG_variant` name only when no other variant claims a
    byte it claims. `Option<T>` has exactly one payload-carrying variant, so
    `Some` survives; `Result<T,E>` has two over one range, so both are spelled
    `field_0x<offset>` — the same offset rendering `dwarfvariants off` produces,
    which is what a reader gets when the answer is unknown. Both the union member
    and the facet's own interned type name are suppressed, because a cast in the
    emitted C prints the type name and suppressing only one of the two would still
    leak the variant. Two suppressed facets that describe the same bytes share one
    interned struct; two that differ get distinct ones, because the ABI classifier
    common-refines the union's members and merging two shapes would change how the
    enum is passed.
  - a field inside a facet keeps its DWARF name only when every other variant
    either claims none of its bytes or names exactly that range the same way.
    rustc names tuple payloads `__0`/`__1`, so `Result`'s two `__0`s agree and the
    name claims nothing about which variant is live; `enum Multi { P{a,b}, Q(u64) }`
    keeps `P.a` (nothing else claims [4,8)) and spells the word at 8 by offset,
    because `P.b` and `Q.__0` disagree there.

  The rule is deliberately conservative at facet granularity: an access WIDTH can
  sometimes single out a variant that the byte range alone cannot (an 8-byte store
  at offset 8 of `Multi` can only be `Q`), but a union member name is fixed when
  the type is built, not per access, so those labels go too. What is given up is
  the label, never the layout — offsets, widths, member types and the enum's own
  size are exactly what DWARF states either way, and every variant's source name
  and `DW_AT_discr_value` remain on the side table. Picking the facet from the tag
  needs a dominating-guard analysis; that is what the side table is recorded for,
  and it is not attempted here.
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
  (Itanium), `rustc_demangle`, and `msvc_demangler` (`?…` names) crates.

  **(kuna, DIV-83) Which crate is asked first is decided by a marker, not by
  which one answers.** Rust's *legacy* scheme reuses the Itanium `_ZN…E`
  envelope, escaping the characters an Itanium identifier cannot hold (`$LT$`
  for `<`, `$C$` for `,`, `$u20$` for a space, `..` for `::`). A C++ demangler
  therefore does not decline such a symbol — it sees a well-formed nested-name
  whose components happen to contain dollar signs, and returns the escapes
  verbatim. Asking Itanium first did not fall through to Rust; it produced a
  wrong answer confidently, and every Rust binary rendered its own call graph as
  escape soup (`core::ptr::drop_in_place$LT$…$GT$` where `nm -C` gives
  `core::ptr::drop_in_place<…>`). `sourcelang::is_rust_mangled` identifies both
  Rust schemes exactly — a `_R` prefix, or the legacy `17h<16 hex>E` hash tail —
  so a symbol carrying either goes to `rustc_demangle` first. A C symbol carries
  neither, so the arm is unreachable for one.

  The v0 arm requires the **leading underscore**, and that requirement is
  load-bearing rather than cosmetic. A prefix test written as
  `strip_prefix('_').unwrap_or(name)` keeps the *original* name when there is no
  underscore to strip, which quietly reduces "begins with `_R`" to "begins with
  `R`" -- and since one matching symbol is enough to classify the whole image,
  any C program importing OpenSSL's `RAND_bytes` or `RSA_new` was reported as
  `Compiler::Rustc`. That misclassification is invisible to both parity corpora,
  whose fixtures are `<bytechunk>` images with no symbol table at all, and it
  reaches the reader through the `--language auto` policy of DIV-80: an ordinary
  C binary rendered as `unsafe fn`.

  A Rust demangling is a *type expression*, not a path: it carries generic
  arguments (`drop_in_place<Vec<u8>>`) and trait qualifiers
  (`<aes::Aes256 as crypto_common::KeyInit>::new`). `normalize_rust_name`
  resolves both, and they cannot be resolved the same way — a generic list
  carries no path and is dropped, but a qualifier carries **two** paths and
  dropping it would leave a leading `::`, an empty scope component the symbol
  table rejects outright. `<X as Y>` keeps the type `X` (so the method stays
  attached to the type that defines it), `<impl X as Y>` keeps the trait `Y`,
  and `<impl Trait for Type>` keeps `Type`. Angle brackets nest, so this is a
  depth-tracking scan iterated to a fixed point rather than a pattern match.

  The hard contract is **name-only** reduction: kuna's scope splitter nests on every `::`,
  so signature tails and template argument groups must be stripped or they become
  junk scopes. **Operator names are exempt from that stripping**: `operator[]`,
  `operator()`, `operator<<`, `operator->` and their siblings are spelled with the
  very characters the reduction removes, so a bracket run directly after an
  operator head is copied verbatim and only the parameter list that follows it is
  dropped. Without the exemption every bracket-spelled overload of a class
  collapsed onto one indistinguishable `Class::operator` — 65 distinct functions
  in `libstdc++` shared the name `std::operator`, which is now split into its real
  `std::operator<<` (33) and `std::operator>>` (32). A `<` followed by an
  identifier character is left to the generic path, where it opens a template
  argument list rather than spelling the operator. **Anonymous namespaces are the
  second exemption**, and the one whose absence was fatal rather than merely lossy:
  Itanium renders `_GLOBAL__N_…` as the parenthesized `(anonymous namespace)` and
  MSVC as the backtick-quoted `` `anonymous namespace' ``, so the reduction used to
  delete the Itanium spelling whole and leave an **empty** component —
  `leveldb::(anonymous namespace)::HandleDumpCommand` reduced to
  `leveldb::::HandleDumpCommand`. The scope splitter rejects an empty component
  (§0.4), and because the symbol table is installed inside `load file` that
  rejection aborts the entire architecture build, so a binary carrying one such
  symbol produced no output from any command. An anonymous namespace is the
  ordinary way C++ gives a definition internal linkage, which put a large share of
  real unstripped C++ binaries — a MinGW malware DLL with 1184 such libstdc++
  symbols, and `libleveldb` — outside what kuna could load at all. Both spellings
  now become the identifier `anonymous_namespace`, matching what
  `decompiler/crates/kuna-analysis/src/analyzers/rtti/kuna_itaniumrtti.rs
  (sanitize_class_name)` already gives the same construct, so one toolchain's
  anonymous namespace is spelled like another's and the component survives as a
  real scope. Two translation units that each define `helper` in an anonymous
  namespace still collide on one name — the Itanium mangling is identical for both,
  so no demangler can separate them — and their distinct addresses are what every
  resolver keys on. `demangle_raw` keeps the faithful c++filt text it is asked for,
  unrewritten.
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
  existing MSVC arm. From each recovered `<Class>::vftable` base the pass then walks
  the slot array (`vftable.rs`), bounded at the first NULL or non-`.text` slot, and
  emits one **function** symbol per surviving slot at the address it points at. That
  symbol is named `<Class>::vfunc_<i>` — the class name comes from the RTTI0
  `TypeDescriptor` and the slot index is the only disambiguator MSVC metadata offers,
  since it records no per-method names. The stem is `vfunc_`, not `vftable_`, because
  the name lands on *code*: a class compiled under multiple inheritance genuinely owns
  more than one vftable, so an indexed `<Class>::vftable_<i>` reads as that class's
  i-th table and made `kuna functions` report hundreds of bytes of executable
  `std::basic_stringbuf` code as vtable objects. Only the table itself wears a
  `vftable` name, and it is unindexed.
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

(kuna) **With no section table, the plausible-code oracle is the program header.**
"Restricted to executable sections" is a filter every oracle above passes through
(`decompiler/crates/kuna-analysis/src/analyzers/entry/mod.rs
(executable_sections)`), and it reads the section table alone. An image that has no
section table therefore has no executable sections, so the filter rejected every
candidate the oracles produced — including the image's own `e_entry` — and
discovery returned nothing on a file the loader had just mapped perfectly well and
could decompile function by function through `--addr`. Where the section table is
*absent* the `PF_X` `PT_LOAD` segments stand in for it
(`executable_segments`): they are the other, independent description of the same
image, and the one the loader itself works from. The substitution is coarser — a
read-execute `PT_LOAD` also spans `.rodata` and the ELF header — which is why it is
reached only when there is no section table at all: an image that has one has
already had its say about what is code, and is left with exactly the ranges it had.
One image is section-less and yet not a candidate: a **UPX-packed** one, whose load
segments are a decompressor around a compressed blob. Discovering the stub's
handful of routines would bury the far more actionable answer kuna already gives
such an image — "image appears UPX-packed; try `kuna unpack`", which is what a run
that discovers nothing produces — so a load segment carrying the `UPX!` magic
declines the fallback and keeps that behavior (`is_packer_stub`).

(kuna) The **data** half of that oracle takes the same substitution, and for the
same reason. A reference query classifies a candidate data operand by asking
whether its value lands in memory the image maps
(`decompiler/crates/kuna-analysis/src/listing/xrefs.rs (mapped_ranges)`), and that
question was also asked of the section table alone — so on a section-less image
every data operand answered "nothing is mapped" and was discarded, while control
flow, which never consults it, survived intact. That asymmetry is what an agent
sees: `kuna disassemble` prints `LEA RDI,[0x6b22]`, and `kuna xrefs --to 0x6b22`
and `kuna strings` both answer zero, so the string a program plainly prints is
owned by no function. Where the section table is *absent* the `PT_LOAD` segments
stand in for it, and only there: an image that has sections which are **not** the
runtime layout — a relocatable object, whose section addresses are pre-link and
describe a different address space — is declined a step earlier and keeps
answering nothing rather than classifying every reference against the wrong
partition. The coarseness a `PT_LOAD` carries costs nothing measurable here,
because the value filter (`looks_like_address`) already rejects everything below
the address floor, which is where the inter-section padding and the ELF header of
a low-based PIE live: on a control pair differing only in whether the section
header table is present, the section-less image now answers the *same* data
references as the sectioned one over the same functions, with none added.

(kuna) **Not every `.pdata` record is a function** — the PE exception directory
answers "where does unwinding start from here", which is a coarser question than
"where does a function start"
(`decompiler/crates/kuna-analysis/src/analyzers/entry/pe_entry.rs (pdata_begins)`).
Two record properties separate the two, and both are read from the image rather
than inferred.

The first is `pdatachained` (default-on; kuna). MSVC splits one function across
several `RUNTIME_FUNCTION` records whenever it shrink-wraps a prologue or moves a
cold block out of line: the first record is the function, and every later one
points at an `UNWIND_INFO` whose flags carry `UNW_FLAG_CHAININFO` (bit `0x4` of
the high five bits of the first byte) plus a trailing chained `RUNTIME_FUNCTION`
naming the primary. Its `BeginAddress` is therefore a point *inside* the primary
— typically a register-save or spill run, never a prologue — and claiming a
function there puts a known entry in the middle of a body, which is exactly the
condition S2's `funcboundflow` truncates a fall-through at. The reported symptom
is the whole function: `sub_140002650` in `dobin/redtest` stops four statements
in, carrying the `funcboundflow` truncation warning, because the shrink-wrapped
chunk at `0x140002712` had become `sub_140002712`. Depending on what the
truncated instruction is, the residue is an empty `if` body, a `} while ;` that is
not C at all, or a decompile that fails outright. On, the third dword is resolved
against the loaded sections and a record whose flags set that bit contributes no
entry — Ghidra gates `markAsFunction` on the same predicate. The read is total: a
null `UnwindInfoAddress`, an RVA no section covers, or an empty slice all read as
*not chained*, so the rule can only ever subtract a record it has positively
identified. Almost always nothing is lost by subtracting it, because the chunk's
bytes are reached as the primary's own fall-through or branch target; measured on
an MSVC crackme with 193 records, 32 of them chained, the inventory drops 45
entries (the 32 chunks plus 13 zero-xref phantoms the chunks had seeded) while the
union of every function's extent is byte-for-byte identical at 196,943 bytes.

The residual is a fragment the primary's flow never reaches: it stays inside the
primary's extent but stops being decompiled, because nothing decodes it any more.
The shape that names it is an `__except` funclet entered only through the
exception dispatcher. A second, 240 KB MSVC image sizes the effect — comparing
`decompile-all --json` `line_mappings` on both arms, not extents, since the extent
union is blind to it. Of the 99 entries the option removes there (716 records, 93
of them chained), 97 keep their decompiled coverage inside the primary, and two
24-byte fragments, `0x140007498` and `0x140015dc8`, lose all of it: 48 bytes.
Neither of those two is a `.pdata` record. Both sit in holes in the exception
directory and are in the inventory only while the chunk entries around them are,
nothing in the image references either (`kuna xrefs --to` reports zero on both
arms), and both were mis-started to begin with — `0x140007498` is eight bytes into
a virtual-call thunk that starts at `0x140007490`, and `0x140015dc8` is one byte
into a `mov [rip+…],rax`, which is why its body read a variable nothing assigned.
That is also why the filter is not narrowed to keep a fragment the primary cannot
reach: the decision is taken in `pdata_begins`, inside `load file` and before a
single instruction is decoded, so "does the primary's flow reach this address" is
not a question the oracle can ask — and on this image no chained record loses
coverage for a narrower predicate to recover. Ghidra has the same residual.
`option pdatachained off` restores the previous x86/x64 discovery set exactly —
the stride below is not part of the gate.

The second is the record *stride*, which is not a judgement call and is therefore
not gated. `RUNTIME_FUNCTION` is the 12-byte `{BeginAddress, EndAddress,
UnwindInfoAddress}` only for x86 and x64; ARM, ARM64, ARM64EC and ARM64X use an
8-byte `{BeginAddress, UnwindData}` record whose `BeginAddress` carries the Thumb
bit in its low bit and whose second dword is an `.xdata` RVA only when its low two
bits are clear (packed unwind data otherwise, and never an address to dereference).
Walking an ARM64 table at the x64 stride reads the wrong dwords at the wrong
offsets: on a four-function probe it recovers two entries, one of them only
because record 0 happens to sit at offset 0. The stride follows
`FileHeader.Machine`, as Ghidra's `ExceptionDataDirectory` dispatches it, and a
machine that is neither an x86 nor an ARM variant — IA64, MIPS, SH, PowerPC, each
with its own record layout, a MIPS one being 20 bytes — has no readable shape, so
the directory is left alone rather than misparsed; Ghidra logs "Exception Data
unsupported architecture" and leaves its `functionEntries` null at the same point.
Chained fragments are not decoded on the ARM form — Ghidra does not decode them
either. Ghidra additionally routes an image whose load-config CHPE metadata
pointer is set to its ARM parser regardless of `Machine`; kuna parses no load
config, so an ARM64EC image that declares itself `AMD64` still reads at 12.

Two known follow-ups sit on the ARM form. The `BeginAddress` low bit is a Thumb
marker and the walk currently masks it off to get the address, so on an ARMNT or
Thumb-2 image the recovered entries carry no decode mode and are decoded as A32 —
still strictly better than reading the table at the wrong stride, but wrong for
Thumb. Painting `TMode=1` at a Thumb-marked `BeginAddress` belongs beside the
Cortex-M whole-image paint, in `ContextPainter::new`
(`decompiler/crates/kuna-analysis/src/listing/context.rs`) for the walk and in the
`EntryDiscoveryPass` commit path for the committed facts, which is where
`cortexm_thumb_paints` already lands. The second is the CHPE routing above. There
is no ARMNT PE in the corpus to measure either against.

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

(kuna) **The PE CRT entry-function prototype** (`entrymainproto`, default-on;
`decompiler/crates/kuna-analysis/src/analyzers/entry/kuna_entrymainproto.rs
(EntryMainProtoPass)`) is discovery's answer to a question the rest of the pipeline
cannot reach. kuna recovers a callee's parameters from the callee's OWN body — an
ABI argument register read before it is written is a parameter — which is why a
stripped PE's helpers come out correctly typed. It is also why `main` comes out
`void(void)`: a `main` that ignores `argc` and `argv` never reads `rcx`/`rdx`/`r8`,
so body-driven recovery finds nothing, and the agent reading that output sees a
callee declared to take nothing being called with three arguments a few lines up in
its own caller. The entry point is the one place where the arguments are visible
*without* the body, because on a PE the C runtime startup is inside the image and
kuna already decompiles it correctly: MSVC's `__scrt_common_main_seh` fetches each
argument through a named UCRT accessor (`__p___argc`, `__p___argv`/`__p___wargv`,
`_get_initial_narrow_environment`/`_get_initial_wide_environment`) in the
instructions immediately before the call. Those names are imported by the startup
and by nothing else, so the window between the accessor cluster and the next direct
call to a non-accessor names the entry function; the pass scans the executable
bytes for `E8 rel32` rather than disassembling, because the CRT startup is ordinary
compiler output with no overlapping encodings. The parameters are typed at the
WIDTH the call site establishes — the 4-byte `argc` slot and the two pointer-width
slots — and named after the accessor that produced each; they are deliberately not
typed `int` / `char **`, which would assert the C library's declaration of `main`,
and the pass has no evidence for that (the same shape carries `wmain`'s
`wchar_t **`, and a hand-rolled entry point need not be `main` at all). The address
rides out with the prototype as a discovered entry, because the prototype is parked
by NAME and on an obfuscated image whose prologue no oracle recognises the callee is
not a registered function, so the park would be a silent no-op.

One consequence is worth naming, because it is the price of the recovery rather
than a defect in it. Declaring `argc` makes the first ABI argument register live at
the entry function's own entry, so a call there to an import kuna has no prototype
for now finds a value in it and renders `IsDebuggerPresent(CONCAT44(dat_c,argc))`
where it used to render `IsDebuggerPresent()`. That is the standing behaviour at any
unprototyped callee reached with a live argument register — the same shape as a
call-site argument recovered for an unnamed helper — and the real answer is a Win32
prototype table beside the libc ones (§1.4), not withholding the entry prototype.
Measured over 139 PE crackmes: the byte scan locates a candidate on 37, the guards
reject 7, and of the 30 that fire, 4 gain one such argument — against 30 that gain
the entry prototype.

Three guards keep it from firing where it would be wrong. It is PE-only, and the reason is the
evidence rather than the symptom: a stripped ELF whose `main` ignores its arguments
comes out `sub_<addr>(void)` too, but on ELF the CRT lives in libc — `_start` hands
`main` to `__libc_start_main` and the argument passing happens in another image — so
there is no call site in the object to read. The ELF `main` oracle above finds the
address; asserting three slots there would be quoting the C convention rather than
observing a caller, a weaker claim than this pass makes (and body-driven recovery
already types the arguments wherever `main` uses them). The callee must carry **no** function symbol — a named `main`, from
`.symtab`, an export, a PDB or DWARF, already has a better signature coming from
that source. And a call to msvcrt's `__getmainargs`
shim family inside the window abandons the cluster: MinGW reaches the same three
values through OUT pointers and its shim calls `__p___argc`/`__p___argv` too, so the
accessor test alone matches inside it and the following call is `_set_new_mode`, not
`main`. Seven crackmes images have that shape. The unnamed-callee guard happens to
reject all seven — every candidate the shim produces is a named import — but that is
luck rather than reasoning, so the shim's own accessors are named and bailed on.

(kuna) **The Mach-O `LC_MAIN` entry** (`machomain`, default-on, DIV-111;
`decompiler/crates/kuna-analysis/src/analyzers/entry/kuna_machomain.rs
(MachoMainPass)`) is the same question on the other container, and there the
answer needs no recovery at all. A stripped Mach-O executable answers
`kuna functions` with an inventory of `sub_<addr>` and nothing else, so the one
function an agent needs first — where the program starts — is indistinguishable
from the other twenty-three, and finding it means reading bodies until one looks
like a prompt loop. The image already says which it is, and says it somewhere
`strip` does not reach: `LC_MAIN` is a load command whose `entryoff` field is
documented as the file offset of `main()`, `ld64` emits it for every
normally-linked executable, and `dyld` calls `__TEXT.vmaddr + entryoff` as
`main(argc, argv, envp, apple)`. The name is therefore a restatement of the
container rather than an inference, and it is applied through the same
`entry_names` overlay the dynamic `_INIT_<i>`/`_DT_INIT` names ride (§1.6), so
the commit's idempotent cross-scope probe still lets a real symbol win. It is
spelled `main`, not `_main`: the underscore is the Mach-O assembler's C-symbol
decoration, and the C name is what was asked for.

The prototype rides the same fact, for the reason `entrymainproto` exists — body-driven
recovery has nothing to find in a `main` that ignores its arguments — but it is
typed differently on purpose. `entrymainproto` reports the widths a recovered PE
call site establishes and refuses to assert the C library's declaration, because
the evidence it has is a call site and the same shape carries `wmain`'s
`wchar_t **`. Mach-O has no in-image call site to read (the C runtime that calls
`main` lives in `libdyld.dylib`) and needs none, because `LC_MAIN` *is* the POSIX
`main` by definition: the honest spelling is `int main(int argc, char **argv)`,
which also lets a string literal render through `argv[i]`. `envp` is deliberately
not declared — `dyld` does pass it and a fourth `apple` pointer, but the extra
unused slots cost more noise than they buy, and a `main` that really reads `envp`
still shows the third argument register in its body.

The refusals are what keep the claim honest, and on the 23 Mach-O images of the
RE corpus they account for every one of the 15 the pass declines: 12 already
carry a `_main` symbol at that address (a named entry has a better name coming
from whatever named it, and the pass never overwrites one), and 3 are
`LC_UNIXTHREAD`-only pre-10.8 images whose entry is the crt's `start`, not
`main`, so nothing is claimed. It also refuses anything that is not an
`MH_EXECUTE` Mach-O, an entry outside every executable section, and an image that
already defines a symbol spelled `main`, which would make the by-name prototype
park ambiguous. Of the 8 that fire, 6 change only the declaration line; 2 also
gain one spurious argument at an unprototyped callee
(`___chkstk_darwin(CONCAT44(v7,argc))`) — the same standing behaviour the PE pass
above measures at 4 of its 30, and the same answer applies: a live argument
register at a callee with no prototype is a prototype-coverage problem, not a
reason to withhold the entry declaration. Structurally inert on every ELF, PE and
COFF target, which is also why neither parity corpus can observe it in either
direction: both are symbol-less ELF bytechunks.

The same `entryoff`-is-not-a-VMA fact is what every *reporting* surface has to
know, so it is stated once as
`decompiler/crates/kuna-analysis/src/analyzers/entry/mod.rs (image_entry_vma)`
rather than re-derived. `object`'s `File::entry()` hands back the raw header/
load-command field, which is already a virtual address for an ELF `e_entry`, for a
PE `AddressOfEntryPoint` and for a Mach-O `LC_UNIXTHREAD` (the saved thread state's
PC), and is a `__TEXT`-relative file offset for `LC_MAIN`. Reporting that field
directly answers `0x1ce0` for a program whose `main` is at `0x100001ce0` — an
address that matches no function, so the name resolves to a bare hex string and any
reachability walk rooted there returns nothing. The helper rebases only the
`LC_MAIN` case (via `macho_main_entry_vma`, which answers for nothing else) and
falls through to the raw field otherwise, so an `LC_UNIXTHREAD` image does not have
`__TEXT.vmaddr` added to an address that already carries it. A `0` entry is
reported as absent rather than as `0x0`, because a relocatable declares no entry
and `0` is a real address there. The consumers are `kuna functions --summary`'s
`entry`/`reachable_from_entry`, `kuna decompile-graph`'s `isEntryPoint`, and the
`kuna decompile-project` README's entry row.

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
with their computed/indirect predicates but contribute no static successor.

(kuna) The two worklists must agree about what counts as code, and
`unmappedentry` (default-on;
`decompiler/crates/kuna-analysis/src/listing/kuna_unmappedentry.rs
(admits_call_entry)`) is what makes them. The instruction worklist gates every
address on the executable ranges above and drops anything outside them; the
function worklist did not, so a direct CALL into unmapped memory still became a
`DiscoveredFunction` that `fast_funcdisc` and `funcdisc_recursive` committed — an
entry with no bytes behind it, reported at size 0 and decompiling to nothing.
Those targets are not decode failures; the CALL decoded correctly and the operand
is junk. An always-taken branch followed by anti-disassembly filler produces one
directly: `xor eax,eax; je +1; e8 ...` puts the `e8` one byte before the real
instruction, and following the (never-executed) fall-through reads a call to an
address four gigabytes above a 25 KB image. The gate applies the *same* predicate
the instruction worklist uses, so the walk claims a function only where it is
willing to disassemble. It withholds the function claim only: the Call
cross-reference is filed in both directions either way, because the instruction
really does encode a call to that address and `kuna xrefs` should still say so. A
target inside an executable section is admitted exactly as before even when the
decode there fails — that is a genuine gap in the walk, not a fabricated entry —
so the gate can never remove an entry that had a body. Measured over 234 crackmes
images it removes 150 entries on 19 of them and adds none; every one is `size: 0`
and outside every executable section, and emitted C over 6,085 functions of those
images changes in exactly one function, where two parameters wrongly typed `code *`
(a phantom sat at the address they pointed to) come back as the data pointers they
are. Off restores the previous, phantom-producing discovery set exactly.

(kuna) The same seam carries `ppclocalentry` (default-on;
`decompiler/crates/kuna-analysis/src/listing/kuna_ppclocalentry.rs (fold_map)`),
which answers a different question about a CALL target: not whether it is code,
but whether it is a *function*. The OpenPOWER ELFv2 ABI gives a PPC64 function
two entry points — the symbol's `st_value`, whose first instructions materialise
the TOC pointer `r2` from `r12`, and a **local entry** a few bytes later, which
is where a caller that already holds the right `r2` (anything in the same module)
branches instead. The distance is recorded per symbol in the ELF `st_other`
field, packed in bits 5-7 as `(1 << n) >> 2 << 2`, and `readelf -sW` prints it as
`[<localentry>: 8]`. Nothing read that field, so the walk saw an intra-module
`bl` land eight bytes past a function symbol and minted a function there like any
other CALL target. On ordinary `gcc` ppc64le output that splits **every locally
called function in two**: the named symbol truncated to its 8-byte TOC prologue,
plus the whole real body under an anonymous `sub_<hex>` — and because S2's
`funcboundflow` truncates a fall-through that reaches a known function entry, the
named symbol then decompiles to an empty husk carrying a `funcboundflow`
truncation warning while its body is reachable only under the generated name. On,
an address that a defined `STT_FUNC` symbol declares to be its own local entry is
never claimed as a function, because by the ABI's own construction the two
entries are the same routine. Four guards keep the fold honest: the `st_other`
field must decode to a real offset (only `n` in 2..6 — 0 and 1 mean the entries
coincide, 7 is reserved), a sized symbol must contain its own local entry, the
local entry must not be the address of some other defined text symbol, and the
global entry must itself be a walk seed with no other seed between the two. That
last guard is what makes the walk's instruction closure invariant under the fold
— the bytes at the local entry are reached as the global entry's fall-through
either way — so the fold can only ever remove the duplicate second entry, never a
body. As with `unmappedentry` only the function claim is withheld; the Call
cross-reference is filed in both directions either way. PPC64-only, and inert on
an image whose symbols carry no local-entry annotation. Off restores the previous,
husk-producing discovery set exactly.

A context painter applies the ARM/MIPS decode-mode paints per address before each
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
static or accepted pointer-table root.

(kuna, GH-299) That gap walk slides its cursor **one byte at a time**, because the
undefined partition is byte-granular by construction, so every byte of every hole is
a candidate function start and both acceptance tests are applied to addresses that
cannot be instruction boundaries — a candidate starting mid-instruction reads the
tail of one encoding plus the head of the next, and that synthetic pair matches a
common prologue about as often as a real one. On a large stripped i386 PE the walk
plants roughly 2,100 entries in the middle of a function body, a third of them inside
a function the discovery set already has an entry for. `aifstrict` (default-off,
carried by the `aggressive` preset;
`decompiler/crates/kuna-analysis/src/analyzers/aif/kuna_aifstrict.rs`) narrows the
cursor: it advances to the next 4-byte boundary rather than the next byte, and a
candidate is probed only when it is 4-byte aligned **or** it is the first byte of its
hole. The hole-start exemption is the whole distinction the option draws — a hole
boundary is evidence, since the recursive-descent walk decoded up to exactly there
and stopped, while an interior byte the cursor slid onto is a guess. The stride is 4
on every architecture deliberately: 16-byte alignment kills nine tenths of the bad
Cortex-M entries but takes four fifths of the real ones with it. Declining a probe
also *recovers* entries rather than only removing them, because an accept advances
the cursor past the accepted body — a phantom accepted one halfword inside a literal
pool consumes the real function behind it. Off restores the byte-granular cursor
exactly.

The complementary reject the issue asks for — refuse a candidate bracketed by a known
function — is deliberately absent. The Listing's function model is entry-ordered and
carries no extents, so "this hole lies inside one body" can only be approximated by
the interval between known entries, and on a sparsely discovered image that
approximation swallows whole unexplored regions rather than one body's interior. It
is the `fdeinterior` question (§1.5) asked of an image that has no unwind extents to
answer it with, and the answer needs real per-instruction walk ownership.

(kuna, GH-313) Upstream applies a **second** fingerprint test that kuna's port
dropped. Its analyzer refuses a candidate twice — once on the shared-prologue count
alone, and again after the validity walk, where a routine that adds no information
must match a prologue shared by fifty discovered functions rather than four. kuna
ported the first refusal and the "no two-instruction routines" half of the second,
so a self-contained routine that calls nothing, jumps nowhere known, and merely
reaches a return is accepted on a two-mnemonic coincidence. `aifcorroborate`
(default-off, **in no preset**;
`decompiler/crates/kuna-analysis/src/analyzers/aif/kuna_aifcorroborate.rs`) restores
it: an accept must either add information — a call, or a jump into already-discovered
code — or match a prologue that fifty discovered functions share. The corroborating
fact is recomputed the upstream way rather than reusing the flag the
valid-subroutine gate already carries, because that one also counts a plain
fall-through out of the hole into decoded code, which is the *signature* of a
mid-body phantom rather than evidence against one. And a refused candidate still
consumes its body: the gap cursor advances past an accepted routine but only one byte
past a rejected one, so an accept-side refusal that released the cursor would hand it
back to the interior of the same hole, replacing one bad entry with a worse one.

The option ships opt-in because it was **measured out of the default path**, not
because it is unevaluated. Over the same corpus `aifstrict` was measured on it cuts
roughly a third of the remaining mid-body entries but costs about half a real
function for each one removed, raises recall on none of the images, and takes real
functions off the A32 targets AIF's remaining justification rests on — which is the
finding, not the failure: upstream's guard assumes a function worth finding calls
something the analyzer already knows, and on bare-metal firmware the functions only
AIF can find are precisely the leaf helpers that call nothing. The per-image numbers
are in the option's catalog row, and the instrument that produced them is
`scripts/decbench/entrysweep.py` (§ the decbench loop), which scores kuna's function
entries for a stripped image against its unstripped twin's symbol table — the
discovery-tier measurement the GED loop cannot make.

`operand_refs` (default-off, matching
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

**The four ARM entry passes reach the default path through the preset** (DIV-93).
`cortexmvectors`, `ptrentry`, `tailcallentry` and `poolentry` each ship default-off in
the catalog, which is what keeps the XML datatest corpus and an explicit
`--mode reliable` byte-identical; but all four are members of `AGGRESSIVE_OVERRIDES`,
and `auto` selects `aggressive` for any binary under 500 KiB, so on the whole-binary
surfaces (`decompile-all`, `functions`, `decompile-project`, the WASM front-end and the
benchmark) a stripped ARM image of that size gets all four. The preset supplies
`listing` and `aif` ahead of them in the same list, which is what the last three
consume; a preset that enabled them without `listing` would enable nothing. They are
evaluated jointly because they compose: measured over the 110 stripped non-x86-64
decbench twins (50,724 symbol-table function starts), entry recall rises 44,957 ->
47,330 (88.63% -> 93.31%) while mid-body false entries *fall* 8,333 -> 7,117. 98.8% of
the 2,402 added entries are real function starts, no ground-truth entry is lost, and
`poolentry`'s 1,217 removals contain no ground-truth address — so the combination
improves recall and precision at the same time rather than trading one for the other.
Off ARM the flip is a measured no-op, not merely an intended one: entry sets are
identical over 90 x86-64 twins and the 12 i386 PE images inside the ARM corpus, and
emitted C is byte-identical over 8 x86-64 binaries. Three of the four enforce that
with an explicit architecture early-return; `poolentry` instead keys on PC-relative
literal pools, which no non-ARM target in the corpus produces. The cost is real work for real
output — discovery-only `kuna functions` runs about 6% longer on a Cortex-M image
because it discovers and reports more functions — and is amortized away end to end,
where the extra bodies dominate the extra discovery.

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

(kuna) **The on-demand cross-reference query**
(`decompiler/crates/kuna-analysis/src/listing/xrefs.rs (build)`) is a second reader
of the same bytes, and is not an `AnalysisPass` at all: it is the read-only index
behind `kuna xrefs` and `kuna strings`, built after the caller has already
committed a program, and it commits nothing back. It repeats the Listing's
two-worklist descent but keeps every input varnode of every p-code op rather than
just `in0`, because the data references an RE agent navigates by — who reads this
global, who takes this string's address — are exactly the part the Listing model
drops. Two rules carry the weight. First, a **direct** flow op's `in0` is the
branch target and is filed as control flow, but an **indirect** one's is not a
target at all: `JMP qword ptr [__imp_VirtualProtect]` lifts to a single
`BRANCHIND` whose `in0` is the import slot, and treating that like a direct
branch's operand made every import veneer in a program reference nothing. Second,
an import has **two addresses under one name** — the IAT/GOT slot and the
forwarding veneer that jumps through it, both of which `pe_iat` (§1.3) names —
so the query joins them into an alias class along the decoded forwarding jump
(`decompiler/crates/kuna-analysis/src/listing/xrefs.rs (veneer_at)`) and answers
`--to` over the whole class, with the forwarding jump itself excluded. A veneer
is recognised only when its indirect jump reads a **decode-time constant**
address, which is what distinguishes it from a jump table (whose address is
computed, and which therefore lifts to a `LOAD` through a temporary); the class is
never derived from a shared symbol name, which would fold genuinely distinct
same-named functions together.

(kuna) The same two addresses make the import's **name** a selector that matches
two entries, and the selector model's answer to that is a refusal naming every
candidate — which would refuse a question that has exactly one answer, since either
address alone answers it. So a contested name is not decided at lookup time at all:
whether its candidates are one callable is a property of the decoded forwarding
jumps, which only exist once the walk has run. The candidates are carried into the
walk as its focus set, so every one of them is decoded, and the ambiguity is settled
afterwards against the alias class
(`decompiler/crates/kuna-cli/src/xrefs.rs (Resolution::settle)`). Candidates that all
lie in one class are one callable and the query proceeds at the class's code half —
the forwarding veneer rather than the pointer slot, because that is the address the
answer will next be disassembled at, with the lowest address breaking a tie between
several veneers through one slot. Candidates that do not all share a class are
distinct functions and keep the refusal, with every candidate still named. The fold
therefore still rests on the decoded jump and never on the shared name; the name only
selects which addresses to check.

(kuna) Because the query runs that descent **itself**, it takes its own analysis
bundle rather than a decompiling surface's. `kuna functions` and `kuna decompile-all`
inject the DIV-15/DIV-20/DIV-68 defaults (the Listing, the prologue-pattern scan,
AIF); the query surface takes the two *discovery* flags and declines the Listing,
because the Listing's walk is this walk — a second recursive descent over the same
bytes, decoded a second time. On a 466 KB obfuscated i386 image that walk, plus
`operand_refs`' third linear decode, was 1.66 s of a 3.4 s answer that is
byte-identical without either.

The two discovery facts the Listing fed are produced from the query's own decode
instead. The `<patternpairs>` prologue starts are handed straight to the walk as
seeds (`decompiler/crates/kuna-analysis/src/listing/xrefs.rs (discovery_seeds)`,
gated by `funcstart_patterns` and to the non-x86-64 architectures the injection
covered, so x86-64's seed set is exactly the caller's inventory). The speculative
gap-walk (`aif`) is run over the instruction partition the query's own walk leaves
behind (`gap_entries` → `Listing::from_partition`), and every function it accepts is
walked like any other seed, so the references inside it join the index. That recall
is not optional decoration: a function whose only inbound edge is an indirect call
through a data table has no CALL edge for any descent to follow, and without the
gap-walk a `--to` query loses every call site that lives inside one — measured on a
stripped i386 PE as 61 of one function's 174 callers.

(kuna) A reference query also seeds the walk with **the address it was asked
about**. The same structural gap applies to the query target itself: `--from <entry>`
about a function no descent reaches answered `count: 0` about a function that plainly
has references. The named address is walked after the seeded descent and the
prologue/gap seeds have drained, so an address the natural walk already claimed is
already in `decoded` and attributed exactly as before — the focus pass can only add
coverage, never re-attribute an instruction another entry owns. An address that does
not decode is dropped rather than recorded as a function, so a byte in the middle of
a string does not become `sub_<addr>`.

(kuna) Both of those rules read a reference out of *one instruction's* p-code,
which is the whole answer on x86-64 and no answer at all in 32-bit
position-independent code. There the address of a string, a global or a function
pointer is never a constant in the instruction that uses it: the program
materialises the GOT pointer at run time — `call <next instruction>; pop ebx; add
ebx,imm`, an idiom that exists for no other purpose — and every literal is reached
as base-plus-displacement, so the address occurs nowhere in the image and the
constant scan reports that every string in the program is referenced by nothing.
**PIC base folding** (`picbase`, default-on,
`decompiler/crates/kuna-analysis/src/listing/kuna_picbase.rs`) closes that with a
deliberately tiny abstract machine over the same whole p-code the query already
keeps: a value is a constant or an offset from the stack pointer, memory is
modelled only at stack offsets (enough to follow the `call`'s push into the
`pop`), a constant is tainted as PC-derived when it equals its own instruction's
fall-through, and only a tainted value may establish a base — so a plain
`mov ebx,imm` cannot. GCC's out-of-line form is covered by the same machine: a
direct call whose callee delivers the return address in a register (probed like a
veneer, at most two instructions) hands that register the call's own
fall-through. Three shapes are then read off each instruction *independently*, with
the base seeded and nothing else assumed, so no state crosses a control-flow
edge: the address a `LOAD` reads, the address a `STORE` writes, and a constant
that lands in a register, which is the address-taken case. A value computed only
into a temporary is deliberately not reported — in an indexed access the array
base lands in one, and filing it would claim a reference the instruction does not
form.

Two claims hide in that and they are licensed differently. A function that runs
the idiom *itself* computes the value and assumes nothing. A function that only
uses an inherited base — which is the case that matters, because kuna's own
inventory splits the filing crackme's prompt routine at its `int3` traps and the
`lea` that forms the prompt lands in a different entry from the idiom that set the
register up — is relying on the i386 System V ABI reserving that register as the
module's GOT pointer, so the recovered value is cross-checked against the image's
own `_GLOBAL_OFFSET_TABLE_` (the `.got.plt`/`.got` address) and every idiom in the
program must agree on one register and one value; absent that, nothing is claimed
module-wide. The rule that keeps ownership honest is refusal rather than
guesswork, because attributing a string to a function that merely sits near it is
worse than reporting nothing and no parity gate could see it: the base is offered
to a function whose body never writes the register at all, or from its own
establishment up to the next write of it (in GCC output, the epilogue's restore),
and to no other function. A body that reuses the register for its own purposes
contributes no references rather than wrong ones.

(kuna) One indirection further out is the same defect on ARM, and PIC base
folding does not reach it: an ARM immediate cannot hold an arbitrary address, so
the compiler parks the constant in a **literal pool** in `.text` and the code
loads it PC-relatively (`ldr r0,[0x86e4]` at 0x862c, and the word at 0x86e4 is
the string). SLEIGH resolves the pool address at decode time, so the walk files
that read — and stops, because 0x86e4 is all the instruction says. The literal
itself is referenced by nothing, and on the filing image `kuna strings --json`
reported `xrefs_count: 0` and no owning function for a string
`__libc_start_main` plainly prints. **Literal-pool following**
(`decompiler/crates/kuna-analysis/src/listing/kuna_poolref.rs`) closes that with
exactly one dereference, of content that cannot change: a `Read` of a
pointer-sized, pointer-aligned location in an **allocated, non-writable** section
with file content, whose word passes the same `checkOperands` value filter the
constant scan uses and lands in a mapped section, files a second edge to that
word's value. The edge is filed from the **instruction**, not from the pool word,
which is the whole point — a pool word is data and belongs to no function, so
attributing the reference to it would answer the question with another address
instead of a name.

Each clause is a refusal that would otherwise be a fabricated reference. The read
must be pointer-*sized*, and the width has to come from the access rather than
from the address varnode, because a `LOAD`'s address is pointer-sized whatever
the access is — without that, `ldrh r0,[pool]`, which is reading a number out of
a pool, reads as a pointer dereference. The section must be non-writable: a GOT
entry or a `.data` pointer holds whatever the loader or the program last wrote
there, and the image's copy of it is not evidence. And the value must clear the
address floor, so a read-only word holding 42 stays a number. The kind filed is
`Data`, the address-taken case, because that is what the load did. Measured over
a 15-image sweep: zero edges added on every x86-64 ELF and PE in it (a RIP-relative
load already encodes the address it forms), zero attributions lost anywhere, and
on the ARM images 2,239 new string-to-function attributions on u-boot and 763 on
the filing crackme — of which an independent capstone-plus-symtab oracle
corroborates 2,234 and 761 as real PC-relative pool loads, with every one of the
remainder confirmed by hand as a load the oracle's own sweep missed.

(kuna) The same pool word is a second defect one surface over, in the **listing**
rather than the reference walk. A function's extent contains its pool, so a
straight-line disassembly of `main` walks off the end of the code and decodes the
constant: `1337ARM`'s `main` ended `ldmia sp,{r4,r11,sp,pc}` and then
`0x8458 39050000 andeq r0,r0,r9, lsr r5` — four bytes nothing executes, listed as
an instruction, in place of the success constant `0x539` the program is about.
**Pool-word folding** (`decompiler/crates/kuna-cli/src/litpool.rs`, fed by
`ConsoleProgram::add_fixed_refs_at`) lists such a word as the constant it holds
(`.word 0x00000539`) instead. The evidence is the listing's own and nothing
wider: as each row is decoded, the fixed addresses it names are harvested from
its p-code — the constant locations it READS, in the two shapes SLEIGH spells one
in (a `LOAD` off a constant address, and a direct memory varnode), and the
constant addresses its flow ops name. A word read by some instruction in the
range, and branched to by none of them, is data.

That the evidence has to be *in the range* is what makes the rule predictable and
steerable rather than a global guess: listing the pool word on its own contains
no such load, so it decodes exactly as before, and that is the escape hatch. Four
further refusals bound it — a writable section (a GOT slot is read by address
too, and a writable `.text` is a packer), an address a function symbol sits on
(code by declaration), an unaligned or non-scalar width, and a width that does
not tile a whole number of decoded rows. As in the reference walk, the width has
to come from the ACCESS rather than from the address varnode, which a `LOAD`
makes pointer-sized whatever it reads; and an instruction's own fall-through is
not counted as a branch target, because every predicated ARM instruction lowers
to a `CBRANCH` over its body and a literal pool is a run of words that decode as
predicated instructions — counting it would veto every pool word but the first. The last one is what keeps the listing
stable: a fold only ever merges whole rows over the same bytes, so no address
after it can shift and a wrong fold costs one mis-rendered row rather than a
re-aligned listing. The residual false positive is a literal that lands on real
code — `cortexm_poolentry_le32` carries one deliberately, where a pool reference
resolves onto an undiscovered function's first word — and it costs exactly that
one row, with the bytes beside it and the raw decode one command away.

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
— `exit`, `abort`, `__assert_fail`, `pthread_exit`, `__cxa_throw` and the C++
terminate family, and two kuna divergence blocks appended in place, each fenced by
its own `# (kuna divergence, DIV-nn)` comment: (DIV-21) the genuinely-unconditional
libc additions upstream omits — the BSD `err`/`errx`/`verr`/`verrx`/`errc`/`verrc`
family, `quick_exit`, `__assert_perror_fail`, `__chk_fail`, `__libc_fatal`;
`warn`/`warnx` return and stay out — and (DIV-78) the libstdc++ `std::__throw_*`
family, below), widened by a Rust wildcard list (`core::panicking::panic*`,
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
call sites (`EVIDENCE_THRESHOLD`) show no valid fall-through, plus a bounded
fixpoint promotion for a caller whose body contains a terminal call to an
already-concluded callee and no RETURN anywhere. The threshold buys robustness to
disassembly noise at the price of blindness to rarely-called functions; and the
predicate has a structural blind spot: a no-return call followed by alignment **NOP
padding** reads as a valid fall-through and contributes no evidence at all.

What counts as "no valid fall-through" is itself a decision, exposed as
`noreturn_discstrict` (default-ON, DIV-92;
`decompiler/crates/kuna-analysis/src/analyzers/noreturn_disc/kuna_discstrict.rs`).
A call with no fall-through address at all — a tail jump lowered to a call — is
evidence under either setting: that is a property of the call instruction. What
differs is how its *successor* is read. On the default only **positive** evidence
counts: the successor is data (outside every executable range, so the compiler
emitted nothing there to fall into), or the successor is another function's entry
(the compiler left the caller no tail at all). With the option off a third arm is
restored ahead of those two — the successor is not a decoded instruction start.

That third arm is a statement about kuna, not about the program. The Listing walk
(§1.7) pushes **every** call's fall-through onto its per-function instruction
worklist unconditionally, and that worklist drains before the function is left, so
a call's successor is always attempted; it fails to become an instruction start in
exactly three ways — `decode_one` returned an error, the decode was zero-length, or
the address is outside every executable range. The Listing records no decode
outcome (a failed decode and an unvisited byte are the same `Undefined` code unit),
so the arm is precisely a decode-failure detector: three bytes kuna cannot decode
are enough to conclude that a function whose body is `mov $7,%eax ; ret` never
returns, after which the flow layer deletes the live tail of every one of its
callers (GH-312). Dropping the arm is also what makes the data arm reachable for
the first time — `is_data` implies `!is_instruction_start`, so under the legacy
order the arm above consumed every site it would have caught. Measured over the 660
stripped x86-64 binaries and 110 stripped non-x86-64 binaries of the decbench
corpus on which the Listing is built, the two tallies conclude the *same* 581
callees no-return: the arm's entire marginal output there is decode-failure votes
that never reach the threshold on their own.

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
name match in the same query, differing in gate flag, name-resolution path, and
name set — `noreturn_extern` carries a frozen hard-coded copy of upstream's 21
names rather than reading the shipped list, so neither kuna divergence block
reaches it; it is queried only after `noreturn_externmatch` (which does read the
list, and is default-on) has already declined. On a
normally-linked ELF the proto flag is already set, so both are no-ops there. These
two run inside the engine, not the analysis tier — chapter 02 owns the halt
mechanics they feed.

**The libstdc++ throw family (kuna, DIV-78).** Every `std::__throw_*` helper in
`<bits/functexcept.h>` (and `__throw_regex_error` in `<bits/regex_error.h>`) is
declared `__attribute__((__noreturn__))` and every body ends in
`_GLIBCXX_THROW_OR_ABORT` — a `throw`, or `abort()` when the library is built
`-fno-exceptions` — so none of them can return. Upstream Ghidra's list names
`__cxa_throw` and the terminate entry points but omits this whole family, and the
attribute is a compile-time fact that survives into no binary artifact: at the
decompiler's boundary `std::__throw_length_error` is an ordinary undefined
`.dynsym` import reached through a PLT stub. Nothing but the shipped list can prove
the call cannot return, so without the entries the fall-through is followed and the
code after every such call — most often clang's `call __stack_chk_fail`
unreachable-trap, or the next function's entry — is emitted as if it ran. The
family is on the list twice over, because the two matchers see two different
spellings of the same symbol: the analysis-tier scan matches the **mangled**
`.dynsym` name *before* demangling, written as a trailing-`*` wildcard over the
Itanium `ZSt<len>__throw_<name>` prefix so a signature change (`__throw_ios_failure`
gained a `const char*, int` overload in GCC 7) cannot age the entry out; the
flow-time matcher sees the **demangled** display name, which the `std`-only
namespace guard admits and the leading-underscore strip reduces to
`throw_length_error`. A same-named method on a user class stays out by the
namespace guard, and the mangled prefixes are exact ABI encodings of
`std::__throw_*`, so neither spelling can reach an unrelated symbol.

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

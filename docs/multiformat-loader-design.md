# kuna Multi-Format Expansion — Implementation Plan

**ELF → PE / Mach-O / COFF, with the analysis tier and decompiler core riding unchanged**

Status: design spec for per-format implementation fan-out. Premise: "a decompiler for most file formats." This synthesizes Draft A (reuse-max: smallest delta surface, ship PE-first behind a flag) and Draft B (faithful-formats: one `ObjectFormat` seam, real-world wrinkles named not stubbed) against the live tree (every file:line below was re-verified). The structure is Draft B's `ObjectFormat` trait — because it makes the 5th-format cost near-zero and keeps format knowledge out of shared passes — sequenced with Draft A's PE-first, flag-gated, gate-green-each-PR discipline.

---

## 0. Thesis and the two load-bearing invariants

**The bet, confirmed.** `bootstrap_from_elf` (`engine.rs:618`) is ~95% format-neutral: after `object::File::parse`, every line keys off `LoadImage`/`Object` trait methods, not ELF structures. The ELF-ness lives in **four small chokepoints** in `loadimage_object.rs` plus **one chokepoint module** (`elf_plt.rs`) plus **one dispatch site** (`engine.rs:1104`). The byte/section/segment plumbing is already multi-format through the `object` crate v0.39 — it is *deliberately* fenced ELF-only.

**Invariant 1 — the oracles are untouchable by construction.** The XML datatest path (675/675) and stage-model corpus (158/158) never reach the object loader. `bootstrap_from_file` (`engine.rs:1097`) branches: `\x7fELF` → `bootstrap_from_elf` (`:1106`), else → `bootstrap_from_root` → `bootstrap_program` (`:1110`, `:498`), the XML loader. The code at `engine.rs:658` even comments: *"The XML datatest path never reaches `bootstrap_from_elf`, so the parity..."*. Every change in this plan is additive to the real-binary arm; the XML arm is byte-identical. **This invariant is free and must never be violated** — no PR may alter `bootstrap_program`/`bootstrap_from_root` or the XML dispatch branch.

**Invariant 2 — the ELF arm is today's code, lifted verbatim.** When we introduce the `ObjectFormat` seam (PR-1), `ElfFormat` is the *current* `section_kind_flags`/`resolve_plt_imports`/`:gcc` logic moved behind a trait with zero behavioral change. The existing ELF fixtures (`fauxware`, `plt_aarch64`, `mips_gp_le32`, …, 30+ in `tests/fixtures/`) are the proof: they must pass unchanged after the lift.

**Sequence:** PE first (most common; toolchain builds in-container *today* — verified; IAT naming is arch-independent; MinGW PE carries DWARF + Itanium mangling so it reuses the existing demangler). Then Mach-O (object files build today; linked exe needs `zig`/`lld` or a vendored stub). COFF objects come nearly free with PE.

---

## 1. The loader abstraction & dispatch

### 1.1 The six chokepoints (all re-verified against the live tree)

| # | Chokepoint | Location (verified) | What it does |
|---|---|---|---|
| A | `object` built **elf-only** | `decompiler/Cargo.toml:30` — `features = ["read_core", "elf", "std"]` | `object::File::parse` physically cannot parse PE/Mach-O/COFF. **The hard blocker.** |
| B | Hard format reject | `loadimage_object.rs:189` — `if !matches!(file.format(), object::BinaryFormat::Elf)` → `"not an ELF object (kuna ELF loader is ELF-only)"` (`:191`) | Rejects everything but ELF. |
| C | ELF-only section flags | `loadimage_object.rs:563` — `fn section_kind_flags(kind, flags)` | Matches only `SectionFlags::Elf { sh_flags }`; non-ELF falls to `0` → wrong readonly/alloc/exec bits. |
| D | `:gcc`/SysV-only spec id | `loadimage_object.rs:611` — `fn language_id_for(file, filename)` | Hardcodes the SysV compiler model. |
| E | ELF magic dispatch | `engine.rs:1085` (`ELF_MAGIC`), `:1104` (`bootstrap_from_file` branch) | Only `\x7fELF` routes to the real-binary backend. |
| F | ELF-only import resolver | `elf_plt.rs:56` (`resolve_plt_imports`) — called at `loadimage_object.rs:260`, `s1_entry/mod.rs:268`, `noreturn.rs:226` | PLT/GOT/`.dynamic` decoding; no PE/Mach-O analog. |

Chokepoint **F is the single most-leveraged seam**: `loadimage_object.rs:260`, `s1_entry/mod.rs:268`, and `noreturn.rs:226` all call `resolve_plt_imports(file)` (verified — three call sites, plus `strip_version` at `noreturn.rs:212`). Fixing it once unblocks the whole agnostic-logic bucket.

### 1.2 The `ObjectFormat` trait (new: `src/s1_loader/format/mod.rs`)

All format-specific knowledge funnels through one trait. ELF becomes *an implementation*, not the privileged default. Adding a 5th format (WASM, raw blob) later = "write one impl + register in `detect()`," touching no pass and no engine code.

```rust
// crates/kuna-analysis/src/s1_loader/format/mod.rs

/// (call-target-or-slot address, clean imported name) — structurally identical
/// to today's elf_plt::PltSym. The universal currency of the import/symbol seam.
pub struct ImportSym { pub addr: u64, pub name: Vec<u8> }

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum FormatKind { Elf, Pe, MachO, Coff }

/// Everything kuna genuinely needs to know that differs per object format.
/// ELF is one impl; PE/Mach-O/COFF are siblings. The format-neutral
/// `ObjectLoadImage` body (segments/sections/load_fill/symbol dedup) is unchanged.
pub trait ObjectFormat {
    fn kind(&self) -> FormatKind;

    /// SLEIGH compiler-model id for this format's default ABI, per arch.
    /// ELF/SysV→"gcc", PE→"windows", Mach-O→"gcc" (x86-64 SysV) / "default" (arm64).
    /// None = no ABI opinion (arch default). MUST return a token the vendored
    /// .ldefs actually declares (validated in PR-1) — an invented token yields the
    /// existing "No sleigh specification" error.
    fn compiler_model(&self, arch: object::Architecture) -> Option<&'static str>;

    /// Per-format arm of today's section_kind_flags (chokepoint C).
    fn section_bits(&self, kind: SectionKind, flags: SectionFlags) -> u32;

    /// Format-dispatching replacement for elf_plt::resolve_plt_imports (chokepoint F).
    /// ELF: PLT/GOT/.dynamic. PE: IAT/INT. Mach-O: __stubs/indirect-symbols.
    /// COFF object: none. Pure & total: never panics/errors; unknown → empty Vec.
    fn resolve_imports(&self, file: &object::File, bytes: &[u8]) -> Vec<ImportSym>;

    /// Read-only VMA ranges to constant-fold (MIPS GOT today; usually empty).
    fn const_ranges(&self, file: &object::File, bytes: &[u8]) -> Vec<(u64, u64)> { Vec::new() }
}

pub fn detect(file: &object::File) -> KunaResult<Box<dyn ObjectFormat>> {
    Ok(match file.format() {
        BinaryFormat::Elf   => Box::new(elf::ElfFormat),
        BinaryFormat::Pe    => Box::new(pe::PeFormat),
        BinaryFormat::MachO => Box::new(macho::MachOFormat),
        BinaryFormat::Coff  => Box::new(coff::CoffFormat),
        other => return Err(KunaError::lowlevel(format!(
            "unsupported object format {other:?} (kuna supports ELF/PE/Mach-O/COFF)"))),
    })
}
```

A free function `resolve_imports(file, bytes) -> Vec<ImportSym>` (the trait dispatch behind a plain fn) is what the **three** call sites call, so `s1_entry/mod.rs:268` and `noreturn.rs:226` need **no format branch** — they keep calling one function that now does the right thing per format. (Their signatures already pass `file`; `bytes` is threaded in — `from_bytes` already owns it, and these passes have `ctx.file` plus can take the image bytes.)

### 1.3 How `ObjectLoadImage::from_bytes` changes — exactly three substituted lines

`from_bytes` (`loadimage_object.rs:180`). Everything outside these three points is **verbatim today's code** (segments `:204-216`, section snapshot loop `:219-223`, `load_fill` `:367-460`, `find_section`/`copy_segment` `:326-359`, cursors `:487-526`, `adjust_vma` `:532-557` — all already use the neutral `ObjectSegment`/`ObjectSection` traits):

- **B → detect.** Replace `:189` reject with `let fmt = format::detect(&file)?;`.
- **C → `fmt.section_bits(...)`.** At the section snapshot (`:219-223`), `section_kind_flags(sec.kind(), sec.flags())` becomes `fmt.section_bits(sec.kind(), sec.flags())`. The ELF impl is the current `section_kind_flags` body (`:563-600`) lifted verbatim.
- **F → `fmt.resolve_imports(&file, bytes)`** at `:260`, with the same `seen`-dedup + `funcsyms.push` loop (`:232-294`) unchanged.

**D → `compiler_model` threaded into `language_id_for`** (§2). The funcsym `.symtab`/`.dynsym` loop (`:238-288`), `get_arch_type` (`:528`), `get_readonly` (`:487-526`), and `adjust_vma` are unchanged.

### 1.4 Dispatch in `engine.rs` (chokepoint E)

- **Rename** `bootstrap_from_elf` → `bootstrap_from_object` (`engine.rs:618`). It is already 95% format-agnostic: `get_arch_type` → `resolve_architecture`, `build_engine_and_init`, the MIPS-only `readonlypropagate` toggle (keyed on `arch_type.starts_with("MIPS:")` — stays a no-op off-MIPS, correct for any format), `attach_to_space`, `get_readonly`, `run_default_analyses_per_pass`, `read_loader_symbols_generic`, the deferred listing/analysis commit — **all reused verbatim**.
- **Replace** the bare `ELF_MAGIC` test (`:1104`) with `is_object_binary(&bytes)`:

```rust
const ELF_MAGIC:  [u8; 4] = [0x7f, b'E', b'L', b'F'];
const MACHO_LE64: [u8; 4] = [0xcf, 0xfa, 0xed, 0xfe];  // 0xfeedfacf LE on-disk
const MACHO_LE32: [u8; 4] = [0xce, 0xfa, 0xed, 0xfe];  // 0xfeedface LE
const MACHO_BE64: [u8; 4] = [0xfe, 0xed, 0xfa, 0xcf];
const MACHO_BE32: [u8; 4] = [0xfe, 0xed, 0xfa, 0xce];
const MACHO_FAT:  [u8; 4] = [0xca, 0xfe, 0xba, 0xbe];  // FAT_MAGIC (BE on disk)

fn is_object_binary(bytes: &[u8]) -> bool {
    if bytes.len() < 4 { return false; }
    let m: [u8; 4] = bytes[..4].try_into().unwrap();
    m == ELF_MAGIC
        || matches!(m, MACHO_LE64 | MACHO_LE32 | MACHO_BE64 | MACHO_BE32 | MACHO_FAT)
        || &bytes[..2] == b"MZ"                          // PE (DOS stub); object validates the PE header
        || u16::from_le_bytes([bytes[0], bytes[1]]) matches a IMAGE_FILE_MACHINE_* set  // COFF object
}
```

The XML fallback (which `object` cannot parse) stays the else-branch. The format build-out shipped behind a `--experimental-formats` flag initially — each format-PR flipped its magic on once green — and once all four were proven the flag was **removed in increment 46**: `is_object_binary` now admits ELF / PE / Mach-O / COFF unconditionally (multi-format is the default, like ELF). The XML/datatest corpus never carries an object-format magic, so its dispatch is byte-identical regardless.

---

## 2. Per-format SLEIGH-spec / arch selection (chokepoint D)

`file.architecture()` is **already format-independent** — `object` collapses `e_machine`, `IMAGE_FILE_MACHINE_*`, and Mach-O `cputype` into one `object::Architecture` enum. So the arch→language-stem match in `language_id_for` (`:616-636`) is reused verbatim across all four formats. The **only** per-format variation is the **compiler-model field**, and (verified against the vendored ldefs) it is *not uniform across archs*:

- **x86** (`x86.ldefs`): the model is the 5th colon-field — `x86:LE:64:default:gcc` vs `:windows` (Visual Studio) / `:clangwindows`. Both `id="windows"` and `id="clangwindows"` confirmed present.
- **AARCH64** (`AARCH64.ldefs`): for `AARCH64:LE:64:v8A` the declared compiler ids are `default`, `windows` (Visual Studio, `AARCH64_win.cspec`), `golang` — all confirmed. There is also a separate `id="AARCH64:LE:64:AppleSilicon"` variant (confirmed) for arm64e.

### 2.1 The machine-type → `.sla` map

| `object::Architecture` | PE machine | Mach-O cputype | kuna language id (PE) | kuna language id (Mach-O) | kuna language id (ELF, today) |
|---|---|---|---|---|---|
| `I386` | `IMAGE_FILE_MACHINE_I386` 0x014c | — | `x86:LE:32:default:windows` | `x86:LE:32:default:gcc` | `x86:LE:32:default:gcc` |
| `X86_64` | `IMAGE_FILE_MACHINE_AMD64` 0x8664 | `CPU_TYPE_X86_64` | `x86:LE:64:default:windows` | `x86:LE:64:default:gcc` ¹ | `x86:LE:64:default:gcc` |
| `Arm` | `IMAGE_FILE_MACHINE_ARMNT` 0x01c4 | — | `ARM:LE:32:v8:windows` | — | `ARM:LE:32:v8:default` |
| `Aarch64` | `IMAGE_FILE_MACHINE_ARM64` 0xAA64 | `CPU_TYPE_ARM64` | `AARCH64:LE:64:v8A:windows` | `AARCH64:LE:64:v8A:default` ² | `AARCH64:LE:64:v8A:default` |

¹ **Faithfulness note:** macOS x86-64 follows the System V AMD64 ABI — the *same* cspec Ghidra labels `gcc` for x86-64. So `MachOFormat::compiler_model(X86_64)` returns `"gcc"`, **not** an invented `"macho"` token. The model must always be one the vendored `.ldefs` declares.
² Mach-O arm64 uses `default`; arm64e selects `AARCH64:LE:64:AppleSilicon` (see §3.7, gated).

### 2.2 The hook

```rust
fn language_id_for(file, fmt: &dyn ObjectFormat, filename) -> KunaResult<Vec<u8>> {
    let endian = if file.is_little_endian() { "LE" } else { "BE" };
    let model  = fmt.compiler_model(file.architecture());
    let id = match file.architecture() {
        X86_64  => format!("x86:LE:64:default:{}", model.unwrap_or("gcc")),
        I386    => format!("x86:LE:32:default:{}", model.unwrap_or("gcc")),
        Aarch64 => format!("AARCH64:{endian}:64:v8A:{}", model.unwrap_or("default")),
        Arm     => format!("ARM:{endian}:32:v8:{}", model.unwrap_or("default")),
        // MIPS/PPC/RISCV/SPARC ABI-suffix arms unaffected (model already gcc/default).
        ...
    };
    // validate against scan_language_database; unmatched id → language_index() < 0
    // → existing "No sleigh specification for architecture ..." error.
    // --target remains the universal manual override.
}
```

`compiler_model` per impl: `ElfFormat` → `gcc`/`default` (today's behavior verbatim); `PeFormat` → `windows`; `MachOFormat` → `gcc` (x86-64) / `default` (arm64); `CoffFormat` → `windows` (COFF objects are overwhelmingly MSVC). **The x86 ELF id string is untouched** — no ELF spec change. **Fallback rule:** if a chosen model isn't vendored, fall back to `gcc`/`default` rather than erroring — wrong calling-convention details beat no decompile. A unit test (PR-1) asserts every produced id resolves in `scan_language_database`.

---

## 3. Per-format import naming (the `elf_plt` analogs)

The contract is identical for all formats and to today's `elf_plt`: produce `Vec<ImportSym { addr, name }>` where `addr` is *what a CALL resolves to* (a code stub the disassembler sees, or a data slot the engine constant-folds) and `name` is the clean imported name. **Pure, total, never errors, unknown→empty.** The downstream commit seam (`seen`-dedup → `FuncSym` → `get_next_symbol` `:466-481` → `FunctionSymbol` → `query_call`, plus `s1_entry::existing_function_addrs` and `noreturn`) is **reused verbatim**.

### 3.1 ELF (`ElfFormat::resolve_imports`)

Today's `elf_plt::resolve_plt_imports` (`elf_plt.rs:56`) moved behind the trait, unchanged: per-arch PLT-stub decode → GOT slot → `.dynsym` name via `build_got_name_map` (`:140`); MIPS GOT + `const_ranges`. Names the stub (the call target). Zero behavioral change — proven by the existing ELF fixtures.

### 3.2 PE (`PeFormat::resolve_imports` → new `s1_loader/pe_iat.rs`)

PE calls an import through an **IAT slot**: `call [rip+disp]` (x64) / `call [abs32]` (x86) / a thunk veneer (ARM). The engine constant-folds the `[slot]` load, so — exactly like ELF's GOT — **the name attaches to the IAT slot VA**, making it **arch-independent** (no per-arch stub decoders). Mirrors Ghidra's `PeLoader.addExternalReference` (names the IAT pointer at `firstThunk + i*ptr + imageBase`).

The generic `Object::imports()` gives name+library but **no slot address** — insufficient. Re-parse `bytes` with the typed parser (branch on `object::FileKind::Pe32`/`Pe64`), walk the Import Directory pairing the INT (names) and IAT (slot addresses) in lockstep:

```rust
let pe = PeFile64::parse(bytes)?;                 // PeFile32 for I386
let image_base = pe.relative_address_base();      // = optional-header ImageBase
let it = pe.import_table()?.unwrap();
let ptr = 8u64;                                   // 4 for PE32
let mut descs = it.descriptors()?;
while let Some(d) = descs.next()? {
    let int_rva = nonzero_or(d.original_first_thunk.get(LE), d.first_thunk.get(LE)); // names
    let iat_rva = d.first_thunk.get(LE);          // IAT slot base
    let mut names = it.thunks(int_rva)?;
    let mut i = 0u64;
    while let Some(t) = names.next::<ImageNtHeaders64>()? {
        let slot_va = image_base + iat_rva + i*ptr;
        match it.import::<ImageNtHeaders64>(t)? {
            Import::Name(_hint, name) => out.push(ImportSym{addr: slot_va, name: name.to_vec()}),
            Import::Ordinal(ord)      => out.push(ImportSym{addr: slot_va,
                                            name: synth_ordinal_name(&dll, ord)}), // §3.5
        }
        i += 1;
    }
}
```

This is byte-for-byte Ghidra's `ImportDataDirectory` loop. **Optionally also name the `__impstub`** (`jmp [rip+IAT]`, `FF 25`) so a *direct* `call __impstub` resolves — and that decoder is *identical* to the existing ELF x86-64 `decode_x86_64` (`elf_plt.rs:234`), reusable verbatim over `.text`. Secondary; IAT-slot naming covers the common indirect-call case.

### 3.3 Mach-O (`MachOFormat::resolve_imports` → new `s1_loader/macho_stubs.rs`)

The simplest analog: a `bl` targets `__TEXT,__stubs` (`S_SYMBOL_STUBS`) **directly**, so kuna names *the stub entry* (Ghidra's `MachoProgramBuilder.processStubs`) — no constant-folding, no per-arch decode. The name comes from the indirect-symbol table:

```rust
let mf = MachOFile64::parse(bytes)?;              // MachOFile32 / fat slice per FileKind
let endian = mf.endian();
let dysym = /* LC_DYSYMTAB via mf.macho_load_commands() */;
let indirect = dysym.indirect_symbols(endian, bytes)?;   // &[U32] symtab indices
let symtab = mf.macho_symbol_table();
for sec in typed_sections(&mf) {
    let entry_size = match sec.section_type(endian) {
        S_SYMBOL_STUBS                                      => sec.reserved2(endian) as u64,
        S_LAZY_SYMBOL_POINTERS | S_NON_LAZY_SYMBOL_POINTERS => ptr,
        _ => continue,
    };
    for (i, sym_idx) in sec.indirect_symbols(endian, indirect)?.iter().enumerate() {
        let raw = sym_idx.get(endian);
        if raw & (INDIRECT_SYMBOL_LOCAL | INDIRECT_SYMBOL_ABS) != 0 { continue; }
        let nlist = symtab.symbol(raw as usize)?;
        if nlist.n_strx(endian) == 0 { continue; }
        let name = strip_leading_underscore(nlist.name(endian, symtab.strings())?);
        out.push(ImportSym{ addr: sec.addr(endian) + (i as u64)*entry_size, name: name.to_vec() });
    }
}
```

- `__stubs` → names the `bl` target directly (primary path, all classic Mach-O).
- `__la_symbol_ptr`/`__got` → name the pointer slots too (defensive, the ELF GOT-slot analog, so a folded indirect `call [ptr]` also resolves).
- Filtering: skip `INDIRECT_SYMBOL_LOCAL`/`ABS` and `n_strx == 0` (the self-correction analog). Strip the leading `_` (the `strip_version` analog).
- **Arch-independent** — naming is by section + `reserved1`/`reserved2` metadata, no instruction decode.

### 3.4 Mach-O fat / universal binaries

`0xcafebabe` (FAT_MAGIC) wraps multiple arch slices. The dispatch in `bootstrap_from_object` must **select one slice** (matching `--target`, else a deterministic default — prefer host-relevant or first arm64/x86-64) before constructing `ObjectLoadImage`. Peel to a single `object::File` slice and everything downstream is single-arch as today. `--target`/`--slice` overrides. Ships with the linked-Mach-O PR.

### 3.5 PE import-by-ordinal

`Import::Ordinal(n)` carries no name. Two-tier: (1) **default, always on** — synthesize `<DLL>_Ordinal_<n>` (named beats unresolved); (2) **gated `pe-ordinal-names`** — resolve against a small vendored ordinal→name table for ubiquitous DLLs (`ws2_32.dll`). Off until a fixture exercises it.

### 3.6 COFF objects

A COFF **object** (`.obj`) is pre-link: no IAT, no resolved imports, externals are unresolved *symbols* not addresses. So `CoffFormat::resolve_imports` returns **empty**. COFF's value is symbols (§4) + strings + (rarely) DWARF, not import naming. A COFF *image* (a linked PE `object` reports as COFF-flavored) routes through the PE arm. This object-vs-image distinction is the COFF impl's one subtlety, documented in its module doc.

### 3.7 arm64e (Mach-O)

Pointer authentication does **not** change import naming or symbols. Only spec selection differs: arm64e → `AARCH64:LE:64:AppleSilicon` (verified present) vs generic `v8A`. The id builder selects AppleSilicon when the Mach-O `cpusubtype` indicates arm64e. **Gated `macho-arm64e`**, off until a fixture exists; arm64 (non-e) uses `v8A:default` (default).

### 3.8 MIPS `const_ranges` stays ELF-only

`mips_got_const_ranges` (`elf_plt.rs`) feeds `get_readonly` (`loadimage_object.rs:513`). PE x64 RIP-relative / x86 absolute IAT loads and Mach-O direct `bl`-to-stub do **not** need readonly-marking to fold — IAT/GOT pages are already in read-only sections the existing `get_readonly` flag scan covers. No new seam.

---

## 4. Per-format symbol / entry reading

The three-source funcsym dedup in `from_bytes` (`.symtab` → import-resolver → `.dynsym`, `:232-288`) generalizes by swapping source #2 (the §3 resolver) and keeping #1/#3 format-neutral via `object`'s `symbols()`/`dynamic_symbols()`:

| Source | ELF | PE | Mach-O | COFF object |
|---|---|---|---|---|
| Defined funcs (`file.symbols()`, `SymbolKind::Text`, `addr!=0`) | `.symtab` | COFF symtab | `LC_SYMTAB` (most) | COFF symtab |
| Imports (§3) | PLT/GOT | IAT/INT | `__stubs`/indirect | — |
| Extra defined | `.dynsym` | `file.exports()` (name+addr) | `file.exports()` + `dynamic_symbols()` | — |

Format-neutral carries-over: `SymbolKind::Text`, `addr != 0`, the `seen: HashSet<u64>` dedup, `demangle_funcsym_name` (`:253`). The `addr==0` skip is the ELF UND convention; for PE/Mach-O it stays a safe filter (imports come from §3, not from `addr==0`). `strip_version` (`@@GLIBC`) is ELF-only, a no-op elsewhere — keep in the ELF arm. **PE/Mach-O exports** (`file.exports()` → name+address) register directly as funcsyms (additive, gated inside the resolver, no pass sees a branch).

### 4.1 Entry point — already format-neutral

`file.entry()` is multi-format and **already used** in `s1_entry` (verified: `mod.rs:139`, `:710`). PE → `AddressOfEntryPoint + ImageBase`; Mach-O → `LC_MAIN`/`LC_UNIXTHREAD`; ELF → `e_entry` — all surfaced by `object` transparently. So the *byte access + entry value* are free across formats. The **entry-oracle markup** (`.dynamic` DT_INIT, `.eh_frame` FDEs, `_start→main` idiom) is the ELF-coupled part of `s1_entry` (Phase E) — but for the MVP, `kuna decompile <bin> <addr> --addr` sidesteps it entirely.

---

## 5. Pass compatibility — the per-pass verdict

Every gate below was re-verified (`BinaryFormat::Elf` enumeration). Three buckets:

### 5.1 Fully format-agnostic (no change — "just work" once loaded)
`s1_formatstring` (pure library, reads no object), the **listing tier** decode/walk/classify (rides loadimage bytes through SLEIGH — fully format-neutral; only its *seed set* from `s1_entry` is coupled), `s1_noreturn_disc` (consumes the Listing).

### 5.2 Agnostic logic, only an ELF gate + symbol-source to generalize
Each is a **one-line-gate-drop PR** once §3's `resolve_imports` + a `SectionFlags` arm exist:
- `s1_strings/mod.rs` (gate via `SHF_ALLOC`; use `SectionKind` or add the flag arm)
- `s1_protos/mod.rs:170` (libc/msvcrt name match; PE imports come from §3, not `SymbolKind::Text`)
- `noreturn.rs:169` (vendored name lists are format-neutral; PE/Mac add their own set: `_invoke_watson`, `__fastfail`, `abort`, `_exit`)
- `s1_callfixup/mod.rs:148`, `s1_addrtable/mod.rs:402` (flag arm + reloc guard)
- `s1_dwarf/mod.rs:555` (gimli is format-neutral; drop gate + map `SectionId` → `__debug_*` for Mach-O / `.debug_*` for MinGW-PE)

### 5.3 Heavily ELF-assuming — per-format reimplementation (Phase E, isolated, flag-gated)
- `s1_entry/mod.rs:102,924` — PE: `AddressOfEntryPoint`, TLS callbacks, `.pdata` (the `.eh_frame` analog); Mach-O: `LC_MAIN`/`LC_UNIXTHREAD`, `__eh_frame`/`__unwind_info`, `__mod_init_func`. Arch-specific oracles (libc-start idiom, prologues) reused.
- `s1_sourcelang/mod.rs:128` — PE `Rich`/`@comp.id`, Mach-O `LC_BUILD_VERSION`; reuse the format-neutral Rust-mangled-symbol path.

### 5.4 ELF + niche-arch — N/A
`arm_markers.rs:113`, `mips_markers.rs:124,225` — ELF/ABI conventions with no PE/Mach-O analog (arm64 has no Thumb; MIPS PE/Mach-O nonexistent). Leave gated.

### 5.5 The MSVC demangler gap
`s1_demangle` (verified: `cpp_demangle` Itanium + `rustc_demangle`, `skip()` keys on `_Z`/`__Z`/`_R`/`_GLOBAL_` at `:85-115`). Covers ELF C++, **Mach-O C++ (clang → Itanium, no gap)**, MinGW PE. **Does not** cover **MSVC** (`?func@@YAXXZ`, leading `?`). Fix: add an **`msvc-demangler`** crate arm in `demangle_raw` (`:124`) + `demangle_name` (`:161`), extend `skip()` (`:68`) to recognize `?`. Exact analog of the existing `cpp_demangle`+`rustc_demangle` dependency-substitution LOSS — record in `docs/rust-port/losses.md`. Own small PR; only bites pure-MSVC fixtures.

---

## 6. Fixtures & toolchains (verified in-container)

**Prerequisite for every non-ELF PR:** `decompiler/Cargo.toml:30` → `features = ["read_core", "elf", "pe", "macho", "coff", "std"]`. Without it `object::File::parse` rejects the format. This is PR-0.

Fixtures only need to **parse + decompile**, never run — a freestanding stub or relocatable object suffices. New fixtures land in `decompiler/crates/kuna-analysis/tests/fixtures/` (alongside the existing 30+ ELF fixtures). Each per-format resolver also gets synthetic-buffer `#[cfg(test)]` unit tests mirroring `elf_plt.rs`'s per-arch decoder tests, so import logic is covered without a linked binary.

**Container today (verified live):** `clang`, `gcc`, `ld` with `i386pe`/`i386pep` PE emulations built in. **No** mingw-w64, zig, or lld.

| Format | Artifact | Build (verified-available status) | Exercises |
|---|---|---|---|
| **PE object** (`.obj`) | COFF object | `clang -target x86_64-pc-windows-gnu -c f.c -o f.obj` — **verified produces `Intel amd64 COFF object`, zero new packages** | symbols, strings, demangle, callfixup, noreturn-name, COFF arm |
| **PE32+ exe** | linked PE | `clang -target x86_64-pc-windows-gnu -c -ffreestanding f.c` then `ld -m i386pep -e mainCRTStartup f.obj -o f.exe` — **`i386pep` verified present in system `ld`** | `s1_entry` (AddressOfEntryPoint), loadimage, IAT naming, full decompile |
| **PE (idiomatic)** | MinGW PE | `apt install gcc-mingw-w64` → `x86_64-w64-mingw32-gcc` (carries DWARF + Itanium mangling) | `s1_dwarf` un-gated, existing demangler |
| **MSVC-mangled** | `?`-symbol COFF | hand-assemble/emit COFF with `?`-prefixed symbols, or vendor a prebuilt (no `cl.exe` on Linux) | §5.5 `msvc-demangler` arm |
| **Mach-O object** (`.o`) | relocatable | `clang -target x86_64-apple-macos11 -c f.c` / `arm64-apple-macos11` — **builds today, no SDK** | symbols, strings, demangle (Itanium), `__DWARF`, noreturn-name |
| **Mach-O exe** | linked | `zig cc -target x86_64-macos` (bundles ld64.lld, SDK-free) **or** `apt install lld` + `clang -fuse-ld=lld -nostdlib -e _main` **or** vendor a tiny prebuilt `_main` stub (x86_64 + arm64) — matching the existing vendored-fixture convention (`arm_thumb_le32.o`, confirmed present) | `s1_entry` (`LC_MAIN`), loadimage (`LC_SEGMENT_64`), `__stubs` naming |
| **Mach-O fat** | universal | vendor a 2-slice prebuilt | §3.4 slice selection |

**Container-update PR (PR-0b):** add `gcc-mingw-w64` to the dev container/Dockerfile (idiomatic PE w/ DWARF + Itanium). For linked Mach-O, prefer **vendoring a tiny prebuilt `_main` stub** (x86_64 + arm64) — zero container change, matches the existing vendored-fixture posture — with `zig`/`lld` as the documented alternative. Bare `clang` + system `ld` already covers PE object, PE32+ exe, and Mach-O objects **today**.

---

## 7. Parity safety (why 675/158 and the ELF path stay green)

1. **The XML oracles never reach the object path** (Invariant 1). `bootstrap_from_file` (`engine.rs:1097`) routes XML to `bootstrap_from_root`/`bootstrap_program` (`:1110`), never to `bootstrap_from_object`. No PR touches that branch or the XML loader. 675/675 + 158/158 are structurally immune.
2. **The ELF arm is today's code lifted verbatim** (Invariant 2). PR-1's `ElfFormat` is the existing `section_kind_flags`/`resolve_plt_imports`/`:gcc` logic behind a trait; the 30+ ELF fixtures prove byte-identical behavior. No `if format ==` branches are sprinkled through shared passes — format knowledge lives in the four `ObjectFormat` impls.
3. **New formats are new dispatch arms.** During the build-out each was gated behind `--experimental-formats` until proven; once all four were proven the flag was removed (increment 46) and PE/Mach-O/COFF are admitted unconditionally. Default behavior is byte-identical regardless — the XML/datatest corpus carries no object-format magic, so `is_object_binary` returns false for it and dispatch routes to the XML branch exactly as before.
4. **Three gates green per PR:** `make test` (PARITY OK 675/675), `make test-stages` (no new failures beyond the 2 known), `make rust-test` (workspace + golden + `.sla` parity). Run all three before every commit (per AGENTS.md).

---

## 8. Ordered PR breakdown

> **Superseded note (increment 46):** the PR entries below are the *as-planned/as-shipped* record from when each format lived behind `--experimental-formats`. That flag has since been **removed** — multi-format is now the unconditional default (see §1's note and `docs/analysis-port-log.md` Increment 46). References to the flag below are historical.

Each PR: small, one fixture testcase, all three gates green, risky resolvers off-by-flag first. Dependencies noted.

### Phase A — the seam (pure refactor; ELF behavior identical)

**PR-1 — `ObjectFormat` seam.** *Scope:* introduce the trait, `FormatKind`, `ImportSym`, `detect()`; move today's `section_kind_flags` (`:563`) + `resolve_plt_imports` (`elf_plt.rs:56`) + `:gcc` logic into `ElfFormat`; thread `compiler_model` into `language_id_for` (`:611`); substitute the three lines in `from_bytes` (`:189`, `:219-223`, `:260`); a free `resolve_imports(file,bytes)` for the `s1_entry`/`noreturn` call sites. *Files:* new `src/s1_loader/format/{mod,elf}.rs`; `loadimage_object.rs`; `s1_entry/mod.rs:268`; `noreturn.rs:226`. *Fixture:* none new — **all existing ELF fixtures + 675 datatests pass unchanged** (this is the faithfulness gate). *Test:* `language_id_for`-resolves-against-`scan_language_database` unit test for every ELF arm. *Risk:* low (pure lift). *Deps:* none.

### Phase B — enable parsing + dispatch

**PR-2 — `object` features + dispatch scaffolding.** *Scope:* `Cargo.toml:30` add `pe`/`macho`/`coff`; rename `bootstrap_from_elf`→`bootstrap_from_object` (`engine.rs:618`); `ELF_MAGIC`→`is_object_binary` (`:1085,:1104`) behind `--experimental-formats`; relax `:189` to `detect()`; add `PeFormat`/`MachOFormat`/`CoffFormat` skeletons (`section_bits` arms + `compiler_model`, empty `resolve_imports`). *Files:* `Cargo.toml`, `engine.rs`, `src/s1_loader/format/{pe,macho,coff}.rs`. *Fixture:* `pe_min.obj` + `macho_min.o` (built in-container today) — assert they *parse*, map sections, pick the right spec. *Before→after:* PE/Mach-O object → "not an ELF object" **→** loads, `print spaces`/`disassemble` works with zero imports. *Risk:* low (XML + ELF paths byte-identical; new magics flag-gated off by default). *Deps:* PR-1.

### Phase C — PE (most valuable, easiest toolchain)

**PR-3 — PE loader: gate + section flags + spec suffix.** *Scope:* admit `Pe` in `detect()`; `PeFormat::section_bits` (`SectionFlags::Coff` arm / `SectionKind` fallback); `compiler_model`→`windows`. *Fixture:* `pe_min.exe` (`clang -ffreestanding` + `ld -m i386pep -e mainCRTStartup`, **verified buildable**). *Before→after:* `kuna decompile pe_min.exe main --addr` → "not an ELF object" **→** decompiles bytes with `x86:LE:64:default:windows`; imports show as `sub_<addr>`. *Risk:* low. *Deps:* PR-2.

**PR-4 — PE import naming (IAT).** *Scope:* `resolve_imports` PE arm in new `s1_loader/pe_iat.rs` (INT/IAT parallel walk, arch-independent); `file.exports()` as a funcsym source; ordinal→synthetic name. *Fixture:* `pe_imports.exe` (MinGW, calls `puts`/`printf`). *Before→after:* calls render `sub_140001000(...)` **→** `puts(...)`/`printf(...)`. *Test:* synthetic import-directory buffer → expected `(slot_va, name)` pairs. Delay imports + ordinal-name-table behind `pe-delay-imports`/`pe-ordinal-names` (off). *Risk:* med (the real new logic). *Deps:* PR-3. **After this, flip `--experimental-formats` on-by-default for PE.**

**PR-5 — COFF object support.** *Scope:* admit `Coff` in `detect()`; reuse PE `section_bits`; `CoffFormat::resolve_imports`→empty (§3.6). *Fixture:* `coff_obj.obj` (`clang -c ...-windows-gnu`, **verified**). *Before→after:* `.obj` rejected **→** strings/protos/noreturn-name/demangle run over a COFF object; COFF-symtab names resolve. *Risk:* low. *Deps:* PR-3.

### Phase D — Mach-O

**PR-0b (container prep, lands before PR-6) — toolchain.** *Scope:* add `gcc-mingw-w64` to the dev container (idiomatic PE); vendor a prebuilt Mach-O `_main` stub (x86_64 + arm64) into `tests/fixtures/`, OR add `zig`/`lld`. *Files:* container Dockerfile, `tests/fixtures/`. *Risk:* low (build-infra only).

**PR-6 — Mach-O loader: gate + section flags + spec.** *Scope:* admit `MachO` in `detect()`; `MachOFormat::section_bits` (`SectionFlags::MachO` arm); `compiler_model`→`gcc`(x86-64)/`default`(arm64); Mach-O undefined-symbol convention check. *Fixture:* `macho_min.o` (`clang -c x86_64-apple-macos11`, **verified**) or the vendored `_main` stub. *Before→after:* Mach-O → "not an ELF object" **→** decompiles bytes + correct spec; imports unnamed. *Risk:* low. *Deps:* PR-2, PR-0b.

**PR-7 — Mach-O import naming (`__stubs`).** *Scope:* `resolve_imports` Mach-O arm in new `s1_loader/macho_stubs.rs` (indirect-symbol table, arch-independent, `_`-strip, LOCAL/ABS filter); `file.exports()` source. *Fixture:* `macho_imports` (zig/lld-linked or vendored, calls `printf`). *Before→after:* `bl sub_<stub>` **→** `printf(...)`. *Test:* synthetic `S_SYMBOL_STUBS` + indirect table → expected names. Chained fixups/binds behind `macho-dyld-binds` (off). *Risk:* med. *Deps:* PR-6. **After this, flip on-by-default for Mach-O.**

**PR-8 — Mach-O fat/universal + arm64e.** *Scope:* slice selection at dispatch (§3.4); `AppleSilicon` spec variant. *Fixture:* a 2-slice fat binary — assert the chosen slice decompiles. arm64e behind `macho-arm64e` (off). *Risk:* med. *Deps:* PR-7.

### Phase E — quality follow-ups (symbols, demangling, agnostic-bucket, format markup)

**PR-9 — MSVC demangler arm.** *Scope:* `msvc-demangler` crate in `s1_demangle`; `skip()` recognizes leading `?`; record LOSS. *Fixture:* `msvc_mangled.obj` (assembled/vendored `?`-symbol COFF). *Before→after:* `?foo@Bar@@QEAAXXZ` raw **→** `Bar::foo(void)`. *Risk:* low. *Deps:* PR-5.

**PR-10 — un-gate the agnostic passes** (§5.2). *Scope:* drop the `BinaryFormat::Elf` gate in `s1_strings`, `s1_protos:170`, `noreturn:169` (+ PE/Mac no-return name sets), `s1_callfixup:148`, `s1_noreturn_disc`, `s1_addrtable:402`; add `SectionFlags` Coff/MachO arms where used. **One pass per commit**, each with a fixture assertion. *Risk:* low (read neutral data / §3 output; no pass body branches on format). *Deps:* PR-4, PR-7.

**PR-11 — `s1_dwarf` for Mach-O/MinGW-PE.** *Scope:* drop ELF gate (`:555`); map gimli `SectionId`→`__debug_*` (Mach-O) / `.debug_*` (PE). *Fixture:* `pe_dwarf.exe` (mingw `-g`), `macho_dwarf.o`. *Before→after:* no DWARF locals/types **→** DWARF names/types appear. PE-MSVC PDB explicitly out of scope. *Risk:* low. *Deps:* PR-5, PR-6.

**PR-12 — PE `s1_entry`.** *Scope:* `AddressOfEntryPoint`+ImageBase, TLS callbacks, `.pdata`; reuse arch-specific oracles. Behind `pe-entry`. *Risk:* med (most ELF-coupled pass). *Deps:* PR-3.

**PR-13 — Mach-O `s1_entry`.** *Scope:* `LC_MAIN`/`LC_UNIXTHREAD`, `__eh_frame`/`__unwind_info`, `__mod_init_func`. Behind `macho-entry`. *Risk:* med. *Deps:* PR-6.

**PR-14 — `s1_sourcelang` per-format compiler detect.** *Scope:* PE `Rich`/`@comp.id`, Mach-O `LC_BUILD_VERSION`; reuse format-neutral Rust-mangled path (`:128`). *Risk:* low. *Deps:* PR-3, PR-6.

**Headline goal delivered after PR-4 (PE w/ named imports) and PR-7 (Mach-O w/ named imports).** PRs 9–14 are quality.

---

## 9. Risks + mitigations

| Risk | Assessment | Mitigation |
|---|---|---|
| **Missing SLEIGH spec for a format/arch** | **Low — verified covered.** Vendored ldefs declare `x86 windows`/`clangwindows`, `AARCH64:LE:64:v8A` with `windows`/`default`/`golang`, and `AARCH64:LE:64:AppleSilicon`. x86/x64/ARM/ARM64 all have `.slaspec`s. | If a chosen compiler model isn't vendored, `compiler_model` **falls back to `gcc`/`default`** rather than erroring. `--target` is the universal override. PR-1 unit test asserts every produced id resolves in `scan_language_database`. |
| **PE import-by-ordinal** (no name) | Common (`ws2_32.dll`). | Default: synthesize `<DLL>_Ordinal_<n>` (§3.5). Gated `pe-ordinal-names` for a vendored ordinal→name table later. |
| **Mach-O fat/universal** | Common on macOS. | Slice-select at dispatch (§3.4); `--target`/`--slice` override; default to host-relevant/first arm64/x86-64 slice. |
| **Mach-O chained fixups / dyld binds** | Modern dyld3+/arm64e may lack the classic indirect table. | Indirect-symbol path (PR-7) covers all classic Mach-O. Bind-opcode parsing behind `macho-dyld-binds` (off) until a fixture proves it. |
| **Relocation / image base** | **Not needed for static decompile.** `object` returns `ImageBase+RVA` (PE) / `vmaddr` (Mach-O) transparently — no rebasing, no `.reloc` application. | Documented as deliberately skipped. `adjust_vma` (`:532`) is the BFD-faithful hook if a rebased-dump need ever arises; not on the default path. |
| **MSVC mangling** | `cl.exe` PEs only (MinGW + Mach-O use Itanium). | `msvc-demangler` arm (PR-9), gated on leading `?`. Deferrable. |
| **PE-MSVC debug info is PDB, not DWARF** | Out of scope. | DWARF (PR-11) covers MinGW-PE + Mach-O `__DWARF`. PDB (`pdb` crate) is named future work, not in this plan. |
| **Linked Mach-O needs a linker the container lacks** | System `ld` can't link Mach-O (verified). | Vendor a prebuilt `_main` stub (matches existing `arm_thumb_le32.o` convention) — zero container change; `zig`/`lld` documented alternatives (PR-0b). |
| **`object` 0.39 `Import` has no slot address** | Known (`// TODO: or ordinal`). | Re-parse with the *typed* `PeFile`/`MachOFile` parsers inside the resolver (§3.2/3.3) — `from_bytes` already owns `bytes`. |
| **Oracle regression** | **Structurally impossible** (§7). | XML never reaches the object path; ELF arm is verbatim; new magics flag-gated; three gates green per PR. |

---

**Files this plan touches** (absolute): `decompiler/Cargo.toml:30` (object features + msvc-demangler dep); `decompiler/crates/kuna-analysis/src/loadimage_object.rs` (`:189` gate, `:219-223`/`:563` flags, `:260` resolver call, `:611` `language_id_for`); new `decompiler/crates/kuna-analysis/src/s1_loader/format/{mod,elf,pe,macho,coff}.rs`; new `decompiler/crates/kuna-analysis/src/s1_loader/{pe_iat,macho_stubs}.rs`; `decompiler/crates/kuna-analysis/src/s1_loader/elf_plt.rs:56` (re-homed behind `ElfFormat`); `decompiler/crates/kuna-analysis/src/s1_demangle/mod.rs` (`:68` skip, `:124`/`:161` arms); the Phase-D/E passes (`s1_strings`, `s1_protos:170`, `noreturn.rs:169,226`, `s1_callfixup:148`, `s1_noreturn_disc`, `s1_addrtable:402`, `s1_entry/mod.rs:102,268,924`, `s1_dwarf:555`, `s1_sourcelang:128`); `decompiler/crates/kuna-console/src/engine.rs` (`:618` `bootstrap_from_elf`→`bootstrap_from_object`, `:1085`/`:1104` magic dispatch); fixtures in `decompiler/crates/kuna-analysis/tests/fixtures/`; the dev container Dockerfile (PR-0b: `gcc-mingw-w64`).

# SLEIGH compiler port — the deep structural map (WS0)

Companion to `plan.md`. This is the fleet's map: it pins every C++ source region to
its Rust module + wave, names the symbol-table objects each grammar production
builds, lists the encode methods (and which already exist), records the
`kuna-sleigh` `pub` "freeze interfaces" WS0 exposed, and orders the 148 specs for
the WS7 byte-parity grind.

The crate skeleton (`rust/crates/kuna-slacomp/`) is **interface-frozen**: every
public signature is transcribed from the C++ headers with a `todo!("WSx: <C++
anchor>")` body. `cargo build -p kuna-slacomp` is GREEN. Fleet agents fill bodies
in their owned module only; module files are disjoint so file-disjoint agents
never collide.

## Module ⇄ wave ⇄ C++ source (1:1)

| Rust module (`src/…`)         | C++ source           | Wave | What it owns |
|-------------------------------|----------------------|------|--------------|
| `slghscan.rs`                 | `slghscan.l`         | WS1  | lexer: start-conditions, tokens, keywords, preprocessor |
| `slghparse.rs`                | `slghparse.y`        | WS2  | hand recursive-descent grammar; calls `SleighCompile` builders |
| `pcodecompile_actions.rs`     | `slgh_compile.{cc,hh}` (SleighPcode, MacroBuilder) | WS3 | compiler-side p-code actions |
| `slgh_compile.rs`             | `slgh_compile.cc`    | WS4  | the `SleighCompile` driver + all subsystems |
| `encode.rs`                   | `sleighbase.cc` (encode side) | WS5 | top-level `.sla` emit orchestration |
| `bin/slacomp.rs`              | `slgh_compile.cc` `main` | WS6 | CLI (DONE — real arg parse, delegates to WS4) |

`lib.rs` re-exports `SleighCompile`. The binary parses args for real and calls
`SleighCompile::run_compilation` (the only `todo!()` in the hot path is WS4's).

**Crucial scope finding:** the consumer-side `PcodeCompile` trait
(`kuna-sleigh/src/pcodecompile.rs`) is **already fully implemented, no stubs** —
all of `createOp`/`createLoad`/`createStore`/`createUserOp`/`assignBitRange`/
`createBitRange`/`addressOf`/`appendOp`/`buildTruncatedVarnode`/… were ported for
the runtime `parse line` path. WS3 therefore does **not** re-port `pcodecompile.cc`'s
expression machinery; it only ports the two compiler-only classes (`SleighPcode`,
`MacroBuilder`). This shrinks WS3 dramatically.

---

## WS1 — `slghscan.l` → `slghscan.rs` (672 lines)

### Start-conditions (flex `%x`, slghscan.l:483-488)

| flex state | `ScanState` | Region |
|---|---|---|
| `INITIAL`    | `Initial`    | top level (define/attach/macro/with/subtable names) |
| `defblock`   | `DefBlock`   | inside `define`/`attach` blocks |
| `macroblock` | `MacroBlock` | macro parameter list `( … )` |
| `print`      | `Print`      | constructor display (mnemonic), up to `is` |
| `pattern`    | `Pattern`    | constructor pattern/context, up to `{` |
| `sem`        | `Sem`        | constructor semantic section `{ … }` |
| `preproc`    | `Preproc`    | transient erasure during a preprocessor directive |

State transitions are driven by structural delimiters and keyword rules
(slghscan.l:489-672). Side effects embedded in the rules (`slgh->nextLine()`,
`slgh->calcContextLayout()` on `attach`/`with`, slghscan.l:501-502) are routed back
to the driver through the `ScannerHost` trait so WS1 stays file-disjoint from WS4.

### Token kinds & keyword table

The token kinds are transcribed into `TokenKind` from the bison enum
(`slghparse.hh:74-191`); discriminants are **not** pinned to bison's integers (the
hand parser matches variants). Keywords (`define`, `attach`, `macro`, `with`,
`token`, `context`, `space`, `ram`, `register`, `default`, `signed`, `noflow`,
`hex`, `dec`, `big`, `little`, `size`, `wordsize`, `offset`, `names`, `values`,
`variables`, `pcodeop`, `is`, `local`, `delayslot`, `crossbuild`, `export`,
`build`, `globalset`, `bitrange`, `goto`, `call`, `return`, `if`, `endian`,
`align`, `unimpl`, …) are recognized inline by the flex rules per start-condition;
WS1 ports them as a per-state match (or a small keyword map keyed by state).

### `find_symbol` (slghscan.l:389)

An identifier that resolves to an existing symbol returns the symbol-kind token
(`VARSYM`, `VALUESYM`, `SUBTABLESYM`, …) instead of `STRING`. WS1 calls
`ScannerHost::find_symbol_kind` (→ `SleighCompile`'s symbol table) to decide.

### Preprocessor layer (slghscan.l:48-389)

A hand-written layer that runs *before* tokenization: `@include "file"` (push file
onto the include stack), `@define NAME VALUE` / `@undef`, `@ifdef`/`@ifndef`/`@if`
… `@elif`/`@else`/`@endif` with a truth stack (`ifstack`, `negative_if`), and
`$(NAME)` macro expansion (`expand_preprocmacros`, slghscan.l:197). Key functions
to port: `preprocess` (:232), `preprocess_if` (:171), `read_boolean_clause` (:120),
`read_defined_operator` (:102), `read_identifier` (:65), `preprocess_string` (:82).
These call `ScannerHost::{get,set,undefine}_preproc_value` and
`parse_from_new_file`/`parse_file_finished`.

### Golden-dump test hook (WS1 verification)

Instrument a `/tmp` copy of `slghscan.l` to dump `(token-kind, semantic-value,
line)` per call and diff against the Rust `next_token` stream for the simplest
specs.

---

## WS2 — `slghparse.y` → `slghparse.rs` (592 lines)

### Grammar areas → builder calls (the symbol-table objects each builds)

| Area (productions) | slghparse.y | Builds via `SleighCompile::…` |
|---|---|---|
| spec / definition / constructorlike | 163-182 | (control flow) |
| endian / align | 184-188 | `set_endian`, `set_alignment` |
| tokens (`tokendef`,`tokenprop`,`fielddef`) | 189-214 | `define_token` → `TokenSymbol`; `add_token_field` → `TokenField`+`FieldQuality` |
| context (`contextdef`,`contextfielddef`) | 197-214 | `add_context_field` → `FieldContext` (→ `ContextSymbol` at layout) |
| spaces (`spacedef`,`spaceprop`) | 216-225 | `new_space` ← `SpaceQuality` → `SpaceSymbol`+`AddrSpace` |
| varnodes (`varnodedef`) | 226-229 | `define_varnodes` → `VarnodeSymbol`s |
| bitranges (`bitrangedef`,`bitrangesingle`) | 230-237 | `define_bitrange` → `BitrangeSymbol` |
| pcodeops (`pcodeopdef`) | 238-239 | `add_user_op` → `UserOpSymbol` |
| attaches (`valueattach`,`nameattach`,`varattach`) | 240-245 | `attach_values`/`attach_names`/`attach_varnodes` (mutate value/name/varnode-list symbols) |
| macros (`macrodef`,`macrostart`) | 246-267 | `create_macro` → `MacroSymbol`; `build_macro`; `create_macro_use` |
| with-blocks (`withblock*`,`id_or_nil`,`bitpat_or_nil`) | 249-265 | `push_with`/`pop_with` ← `WithBlock` |
| constructors (`constructor`,`constructprint`,`subtablestart`) | 268-289 | `new_table` → `SubtableSymbol`; `create_constructor` → `Constructor`; `new_operand` → `OperandSymbol`; `build_constructor` |
| pattern expr/eqn (`pexpression`,`pequation`,`elleq`,`ellrt`,`atomic`,`constraint`,`contextblock`,`contextlist`) | 290-346 | builds `PatternExpression`/`PatternEquation`/`ContextChange`; `constrain_operand`/`define_operand`/`define_invisible_operand`/`self_define`/`context_mod`/`context_set` |
| p-code sections (`rtlbody`,`rtl`,`rtlmid`,`section_def`,`rtlfirstsection`,`rtlcontinue`) | 268-365 | `standalone_section`/`first_named_section`/`next_named_section`/`final_named_section`/`enter_section`/`new_section_symbol`/`record_nop` → `SectionVector` |
| statements/expr (`statement`,`expr`,`sizedstar`,`jumpdest`,`varnode`,`lhsvarnode`,`label`,`exportvarnode`,`integervarnode`) | 366-509 | the `PcodeCompile` builders (already in `kuna-sleigh`) + `set_result_varnode`/`set_result_star_varnode` |
| list helpers (`stringlist`,`intblist`,`valuelist`,`varlist`,`paramlist`,`oplist`,`anysymbol`,`charstring`,`familysymbol`,`specificsymbol`) | 509-592 | (value plumbing) |

### Precedence (slghparse.y:82-93), realized as precedence-climbing

Two expression parsers: `pexpression` (pattern values) and `expr` (p-code), both
following the table (highest binds last): `! ~` (unary, right) > `* / % …` >
`+ - …` > shifts > comparisons (nonassoc) > `== != …` > `& …` > `^ …` > `; ` >
`| …` > `&& ^^` > `||`. The `;` precedence level (`%left ';'`) is the SLEIGH
pattern-concatenation operator and must sit exactly where bison places it.

### Bison semantic-value union → Rust

`union SLEIGHSTYPE` (slghparse.hh:197-242) becomes per-nonterminal Rust return
types in the hand parser; the lexer-produced subset is `slghscan::TokenValue`.
Symbol-bearing alternatives become `SymbolId` (the `SymbolTable` index).

### Golden-dump test hook (WS2)

Dump the post-parse symbol table + constructor list + pattern equations from a
`/tmp` C++ copy (after `sleighparse()`, before `process()`) and diff structurally.

---

## WS3 — compiler-side p-code actions → `pcodecompile_actions.rs`

`pcodecompile.cc` (781 lines) is **mostly already ported** in
`kuna-sleigh/src/pcodecompile.rs` (the `PcodeCompile` trait, no stubs). WS3 ports
only:

- `SleighPcode` (`slgh_compile.hh:282-292`; bodies `slgh_compile.cc:1930-1958`):
  the concrete `PcodeCompile` for the compiler. Overrides `allocateTemp` (→
  `SleighCompile::getUniqueAddr`), `getLocation`, `reportError`, `reportWarning`,
  `addSymbol` (→ the driver's symbol table). In the Rust port `SleighCompile`
  owns the `PcodeCompile` state, so these hooks dispatch through it.
- `MacroBuilder` (`slgh_compile.hh:256-275`; bodies `slgh_compile.cc:1785-1928`):
  `setMacroOp` (:1809), `transferOp` (:1833), `dump` (:1886), `setLabel` (:1912).
  Implements `kuna_sleigh::semantics::PcodeBuilder` (already a trait).

If any `kuna-sleigh` `PcodeCompile` method needs a wider visibility for the
compiler, expose it as a freeze interface (none required at WS0).

---

## WS4 — `slgh_compile.cc` → `slgh_compile.rs` (4091 lines, the bulk)

### Helper structs (compiler-only, `slgh_compile.hh:42-246`) — all in this file

`RtlPair`, `SectionVector` (cc:34-85), `SpaceQuality` (cc:87-100), `FieldQuality`
(cc:102-118), `WithBlock` (cc:120-213), `FieldContext` (cc:1777-1784, sorts by
LSB), `ConsistencyChecker` + `OptimizeRecord` + `UniqueState`.

### `SleighCompile` lifecycle

- **`run_compilation(filein, fileout)`** (cc:3774): `parseFromNewFile` → set the
  parse global → open the lexer file → `sleighparse()` (WS2) → on success
  `process()` → open output → `FormatEncode encoder; encode(encoder);
  encoder.flush()` (WS5). Returns 0/2.
- **`process()`** (cc:2479): `checkNops` → `checkCaseSensitivity` →
  (require default space) → `checkConsistency` → `checkLocalCollisions` →
  `buildPatterns` → `buildDecisionTrees` → `buildXrefs` → `checkUniqueAllocation`
  → `symtab.purge`. Each gated on `errors==0`.

### Subsystems (every one anchored in the skeleton with file:line)

- **ConsistencyChecker passes** (cc:297-1776): `testSizeRestrictions`,
  `testTruncations`, `testLargeTemporary`, `optimizeAll`; the size/optimize
  internals (`sizeRestriction`, `checkVarnodeTruncation`, `optimizeGather1/2`,
  `findValidRule`, `applyOptimization`, `setPostOrder`). The header lists the full
  private method set (`slgh_compile.hh:199-225`) — WS4 ports these into the
  `ConsistencyChecker` impl as it fills the four public passes.
- **MacroBuilder/SleighPcode glue** — lives in WS3's module; WS4 calls it from
  `expandMacros` (cc:3397) / `buildMacro` (cc:3737).
- **WithBlock stack** (cc:3676-3697 + 152-213): `pushWith`/`popWith` +
  `collectAndPrependPattern`/`collectAndPrependContext`/`getCurrentSubtable`.
- **context-field building** (cc:2025-2085, 2515-2545): `calcContextLayout` +
  `calcContextVarLayout` (the packed context layout; `contextlock`).
- **pattern build** (cc:2122-2147): `buildPatterns`; **decision trees**
  (cc:2086-2121): `buildDecisionTrees`; **unique allocation** (cc:3577-3675):
  `checkUniqueAllocation` + the `shiftUnique*` family + `insertCrossBuildRegion`.
- **the parser builder methods** (cc:2640-3759): all wired to the WS2 grammar
  table above.

### Interface-freeze note (the one structural decision the fleet must honor)

`SleighCompile` **composes** `kuna_sleigh::SleighBase` rather than inheriting it.
The skeleton holds `pub base: ()` as a placeholder; **WS4 replaces `()` with
`kuna_sleigh::SleighBase`** (or the minimal owned subset: `SymbolTable`,
`AddrSpaceManager`, root table, alignment/uniqbase/endian). Symbols are referenced
by `SymbolId = u32` (the `SymbolTable.symbollist` index), replacing the C++
`SleighSymbol *`. Pattern equations / ConstructTpls / op-template lists are
referenced by arena ids (`u32`) the driver owns — this keeps the parser (WS2)
from threading raw owned trees through every production.

---

## WS5 — the `.sla` encode/writer side → `encode.rs`

### What already exists in `kuna-sleigh` (reused verbatim — do NOT re-port)

The writer primitives and every per-object `encode(...)` are present (the decoder
round-trip exercised them):

- `slaformat.rs`: `write_sla_header` (:270), `FormatEncode`/`PackedEncode` (:284),
  `register_sla_ids`.
- `sleighbase.rs`: `SourceFileIndexer::encode` (:102).
- `slghsymbol.rs`: `SymbolTable::encode` (:233) + per-symbol `encode` (11 total:
  Space/Token/Userop/Varnode/Value/ValueMap/Name/Context/Bitrange/Operand/
  Subtable/Constructor header + `encode_header`).
- `slghpattern.rs`: 8 `encode` (DisjointPattern/Instruction/ContextPattern/
  CombinePattern/…).
- `slghpatexpress.rs`: 11 `encode` (TokenField/ContextField/Constant/Operand/
  Binary/Unary patexpress + `PatternValue` family).
- `semantics.rs`: 5 `encode` (`ConstTpl`/`VarnodeTpl`/`HandleTpl`/`OpTpl`/
  `ConstructTpl`).

### What is MISSING (WS5 ports these two, compiler-side, in `encode.rs`)

- **`SleighBase::encode`** (`sleighbase.cc:226-255`): the top-level orchestrator —
  open `<sleigh>`, write `version`(=`sla::FORMAT_VERSION`)/`bigendian`/`align`/
  `uniqbase`/`maxdelay`(if>0)/`uniqmask`(if≠0)/`numsections`(if≠0); then
  `indexer.encode`; then `<spaces>` (write `defaultspace`, loop spaces skipping
  constant/fspec/iop/join); then `symtab.encode`; close `<sleigh>`.
- **`SleighBase::encodeSlaSpace`** (`sleighbase.cc:197-225`): one `<space>` element
  per non-internal `AddrSpace` (name/index/size/wordsize/bigendian/flags).

These are ported as compiler-side helpers (`encode_sleigh`, `encode_sla_space`) to
avoid editing `kuna-sleigh`. If WS5 finds it cleaner to land `SleighBase::encode`
*in* `kuna-sleigh/src/sleighbase.rs` (next to its `decode`), that is acceptable —
record it as a freeze interface here.

### `kuna-sleigh` `pub` "freeze interfaces" exposed at WS0

**None.** WS0 added a self-contained new crate and exposed nothing new from
`kuna-sleigh` — all reused items were already `pub`. WS5 (and WS4) will likely
need a small number of getters made `pub` (e.g. on `SleighBase`:
`getUniqueBase`/`isBigEndian`/`alignment`/`numSections`/`maxdelayslotbytes`/
`unique_allocatemask`/space iteration; on the symbol subclasses: the build-side
setters). **Every such promotion must be appended to the table below** so the
freeze stays intentional:

| `kuna-sleigh` item made `pub` | For | Wave |
|---|---|---|
| _(none yet)_ | | |

---

## First byte-parity target & WS7 spec ordering

C++ `sleigh_opt` is byte-deterministic; the gate is byte-identical `.sla` for all
148 specs. Order simplest→hardest:

1. **`DATA/data-le-64.slaspec`** and `data-be-64.slaspec` (33 lines incl. `data.sinc`).
   The trivial DATA "processor": near-empty grammar exercise. **First target.**
2. **`Toy/toy_builder_{be,le}.slaspec`** (91) and `…_align2` (92): smallest *real*
   constructors/patterns/p-code — the first end-to-end exercise of WS2-WS5.
3. **`Toy/toy{,64}_{be,le}.slaspec`** + variants (297-305): full Toy ISA, the
   classic SLEIGH teaching spec; covers attaches, with-blocks, macros, contexts.
4. **`BPF/BPF_le.slaspec`** (186): small real ISA.
5. Then climb by line count: 6502/AVR8/Z80/PIC/8051/M68k/SPARC … (mid),
   then **ARM/AArch64/MIPS/PowerPC/x86** (large; x86 last — biggest, deepest
   macro/context use). `find . -name '*.slaspec'` enumerates all 148; sort by
   the total included `.sinc` line count (the `plan.md` harness already resolves
   includes) to drive the grind.

The intermediate verification for each spec: `kuna.slacomp <spec>` then `cmp` the
two `.sla`; on first mismatch, dump both pre-compression element streams (the
instrumented `/tmp` C++ vs the Rust `FormatEncode` debug mode) and bisect to the
diverging element.

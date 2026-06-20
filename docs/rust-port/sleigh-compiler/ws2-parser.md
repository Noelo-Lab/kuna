# WS2 — the SLEIGH parser (`slghparse.y` → `slghparse.rs`)

Hand recursive-descent port of the bison grammar `decompiler/cpp/slghparse.y`
(593 lines / ~3800 generated) to `rust/crates/kuna-slacomp/src/slghparse.rs`,
following the established kuna bison→hand-RD pattern (as the runtime `parse line`
grammar already in `kuna-sleigh/src/pcodeparse`).

## What landed

A complete recursive-descent parser for every production group:

- **definitions** — endian/align, tokens + token fields, contexts + context
  fields, spaces, varnodes, bitranges, pcodeops, the three attaches.
- **constructorlike** — macros (`macro NAME(params) { rtl }`), with-blocks
  (`with id : bitpat ctxblock { ... }`, nestable), constructors
  (`subtablestart constructprint is pequation contextblock rtlbody`).
- **pattern equations** (`pequation`/`elleq`/`ellrt`/`atomic`/`constraint`) and
  **pattern expressions** (`pexpression`) with the full precedence table
  (slghparse.y:82-93), both realized as precedence-climbing.
- **p-code sections** — `rtlbody` (standalone / named-section vectors / `unimpl`),
  `rtlmid`/`rtl`, the EXPORT tails, every `statement` form, the `expr` operator
  grammar (full precedence, incl. the operand-swapping comparisons `> >= s> s>=
  f> f>=`), `sizedstar`, `jumpdest`, `varnode`/`integervarnode`/`lhsvarnode`/
  `exportvarnode`, `label`, and all list helpers.

### The driver seam (`ParserActions`)

The bison actions are `slgh->...` calls on a `SleighCompile`.  Those builder
bodies land in WS4 (still `todo!()`).  So the parser drives the compile through a
**`ParserActions` trait** (one method per grammar action), threading driver-owned
**arena ids (`u32`)** for pattern equations / expressions / ConstructTpl sections
/ ExprTrees / VarnodeTpls / op lists / StarQuality — exactly as bison threads the
`$$`/`$n` semantic values, and exactly as the WS0 interface-freeze decided
(`map.md` §WS4).  `SleighCompile` implements `ParserActions` + `ScannerHost`; the
golden-parse test mock implements both, so **WS2 is verified independently of
WS4**.  The lexer (WS1) is driven through `ScannerHost`; the combined trait
`ParseDriver: ScannerHost + ParserActions` is the single object the parser holds.

## Verification — golden action traces (the WS2 differential)

The natural WS2 golden artifact is the **action/reduction trace**: the ordered
sequence of grammar actions bison fires.  A throwaway instrumented `/tmp` copy of
`decompiler/cpp/**` (the vendored tree stays byte-untouched) has a `KT("<tok>")`
call prepended to each grammar action in `slghparse.y`; the genuine `sleigh_opt`
then parses a spec and dumps the action tokens in reduction order
(`KTRACE_FILE=...`).  `tests/slghparse_golden.rs` runs the Rust parser over the
SAME spec against a recording mock and asserts the emitted `ParserActions::trace`
stream matches token-for-token.  9 golden specs, climbing in complexity:

- `data-le-64` / `data-be-64` — the simplest real spec (24 actions): endian /
  align / spaces / register varnodes / context field / one `:nop is test=1
  unimpl` constructor.  The WS7 first byte-parity target.
- `toy_builder_{le,be}` + `toy_builder_le_align2` (2692 actions) — the first real
  exercise of with-blocks, macros, context (`globalset`), `export`/`build`,
  pcodeops, named display operands.
- `toy_le` / `toy64_be` (2183) — the full Toy ISA: attaches, subtables,
  comparison/float operators, the bracket-operand display form.
- `BPF_le` (2134) and `6502` (2473) — small real ISAs.

The recording mock (`MockDriver`) maintains a **scoped symbol table mirroring
`SleighCompile`'s `addSymbol` calls** — because the lexer's `find_symbol`
(slghscan.l:389) decides whether an identifier becomes a `*SYM` token or a
`STRING`, and that depends on the symbol table built *as parsing proceeds*.  Every
builder that does an `addSymbol` in `slgh_compile.cc` records the (name→kind) at
the same point, with the same constructor/macro scope push/pop and the same
`predefinedSymbols` seeds (`instruction`/`inst_start`/`inst_next`/`epsilon`/the
spaces), and the attaches replace the kind (Value→ValueMap/Name/VarnodeList).  The
context fields flush to ContextSymbols at `calc_context_layout` (the lexer's
`attach`/`with`/`:`-triggered hook).

### Two grammar subtleties worth recording (verified against the golden)

- **First display piece vs later piece.**  `subtablestart SYMBOLSTRING`
  (slghparse.y:277) is the FIRST display piece only — `addSyntax` if the
  constructor is in the root table, else `newOperand`.  Any later SYMBOLSTRING is
  `constructprint SYMBOLSTRING` (slghparse.y:283), which ALWAYS makes an operand.
  A leading `^` (or STRING / charstring / space) consumes the "first" status, so a
  following SYMBOLSTRING is a `newOperand` (this is the `:^instruction is ... {
  build instruction; }` case).
- **Leading display space.**  A run of print-state spaces right after `:` is
  absorbed by `subtablestart ' '` (slghparse.y:288) and does NOT consume the
  "first piece" status — `Rel8:  addr` yields `subtablestart/space` then
  `constructprint/SYMBOLSTRING` (first piece), while `RS: [rs]` yields
  `subtablestart/space`, `addSyntax/char` (`[`), `newOperand` (`rs`),
  `addSyntax/char` (`]`).  STRING pieces trace `addSyntax/str`, charstring(CHAR)
  pieces trace `addSyntax/char` (distinct so the golden disambiguates).
- **Bottom-up trace order.**  Bison reduces leaves left-to-right before the
  enclosing action, so leaf productions (`lhsvarnode/spec`, `varnode/spec`,
  `sizedstar`, …) trace *before* a sibling RHS is parsed — e.g. `lhsvarnode '='
  expr` traces `lhsvarnode/spec` before the RHS `expr`.

## Freeze interfaces added (for WS4)

WS4 fills these — all are `todo!()` in `slgh_compile.rs` (kept out of the parser,
which is `todo!()`-free).  Two trait impls were appended to `slgh_compile.rs`
(marked `WS2 driver seam -- FREEZE INTERFACE for WS4`):

- **`impl ScannerHost for SleighCompile`** — `read_include` and `find_symbol_kind`
  are `todo!()` (WS4: `parseFromNewFile`+read, and `SymbolTable::findSymbol`
  kind); the rest delegate to existing `SleighCompile` methods.
- **`impl ParserActions for SleighCompile`** — every builder method (~110), each
  `todo!("WS4: … (slgh_compile.cc:NNNN)")` with the C++ anchor, except the two
  pure-threading actions (`context_prop_begin`, `finish_main_rtl`) and the
  delegating `set_endian`/`set_alignment`/`report_error`.

The trait method set and the arena-id convention are the WS2 freeze; WS4 may
rename or re-shape *bodies* but must keep these signatures (or update both files).

### `kuna-sleigh` items promoted to `pub`

None — WS2 added a self-contained trait surface in `kuna-slacomp` and reused only
already-`pub` items.

## The gate (all green)

- `cargo build -p kuna-slacomp` clean; the parser has no `todo!()` (other
  modules' `todo!()` are WS3/WS4/WS5 territory).
- `cargo test --workspace --no-fail-fast` — 0 failures; the 9 golden-parse tests
  pass token-for-token.
- C++ tree byte-untouched (golden harness is a `/tmp` copy); `git diff` touches
  only `rust/crates/kuna-slacomp/{src/slghparse.rs,src/slgh_compile.rs}` and the
  new `tests/slghparse_golden.rs` + `tests/golden/parse/*.trace`, plus this doc.

# WS1 — the SLEIGH lexer (`slghscan.l` → `slghscan.rs`)

Port of the flex scanner `decompiler/cpp/slghscan.l` (672 lines) to a hand-written
longest-match scanner with start-conditions, in `rust/crates/kuna-slacomp/src/slghscan.rs`.

## What landed

- Full per-start-condition scanner (`Initial`/`DefBlock`/`MacroBlock`/`Print`/
  `Pattern`/`Sem`/`Preproc`), reproducing flex's **longest-match + rule-order**
  semantics explicitly (each state's rules tried in slghscan.l source order, the
  number/operator overlaps resolved by longest match).
- The hand-written **preprocessor layer** (slghscan.l:48–371): `@include` /
  `@define` / `@undef` / `@ifdef` / `@ifndef` / `@if` … `@elif`/`@else`/`@endif`
  with the `ifstack`/`negative_if` truth-stack erasure machine, `$(NAME)` macro
  expansion (`preproc_macroexpand`), `defined(...)`, and the `read_*` stream
  helpers over a `ByteStream` (the C++ `istream &s`).
- The `find_symbol` seam (slghscan.l:389): identifiers resolve to a symbol-kind
  token via `ScannerHost::find_symbol_kind` (→ `SymbolTokenKind`), else `STRING`.
- `scan_number` (slghscan.l:459): `std::stoul(digits, 0, radix)` semantics with
  overflow/invalid → `BADINTEGER`; pattern-state numbers are signed (`INTB`),
  defblock/sem unsigned (`INTEGER`).

## Verification — golden token streams

A throwaway instrumented C++ harness (`kuna_dump.cc`) was built in a **/tmp copy**
of `decompiler/cpp/**` (the vendored tree stays byte-untouched). It links the real
`sleigh_opt` object set, drives the genuine flex `sleighlex()` over a `.slaspec`,
and dumps `(token_kind, value)` per token exactly as the bison parser receives
them. The symbol table is empty, so every identifier resolves through `find_symbol`
to `STRING` — the Rust test host mirrors this (`find_symbol_kind` → `None`).

`rust/crates/kuna-slacomp/tests/slghscan_golden.rs` runs the Rust `SleighScanner`
over the SAME snippets (`tests/golden/snips/*.slaspec`) and asserts token-for-token
equality against the captured golden streams (`tests/golden/*.txt`). 14 tests:

- `data_le_64` / `data_be_64`: the simplest real spec end-to-end (incl. the
  `@define`/`@include "data.sinc"`/`$(ENDIAN)` preprocessor path) — the WS7 first
  byte-parity target.
- one targeted snippet per start-condition / keyword family / literal form:
  `macro` (macroblock), `attach_with` (defblock attach + with-block), `printstate`
  (print CHAR/`^`/SYMBOLSTRING/quoted), `numbers` (hex/bin/dec, signed pattern),
  `pattern_ops` (`&`/`|` action-on, `$and`/`$or`/`$xor`, `...`, `globalset`,
  `unimpl`), `sem_ops` (every sem operator + intrinsic + statement keyword),
  `misc` (state-scoped keywords, comments, bitrange/pcodeop), `preproc`
  (`@ifdef`/`@ifndef`/`@else`/`@if defined(...)` erasure).
- longest-match edge cases: `badint` (`0xZZ` → `INTEGER 0` then `xZZ`), `overflow`
  (oversized hex → `BADINTEGER`), `emptyhex` (`0x;` → `INTEGER 0` then `x`),
  `atmid` (an `@` not at column 0 is **not** a directive — the `^` anchor).

## Frozen-interface change (for the parser/driver waves)

The skeleton's `ScannerHost` declared `fn parse_from_new_file(&mut self, fname)`
(returning nothing) plus a separate path for the lexer to obtain include bytes.
That cannot work: the flex `preprocess()` does `parseFromNewFile(fname); fname =
grabCurrentFilePath(); sleighin = fopen(fname); … switch_to_buffer`, i.e. the
driver resolves the path *and* the lexer reads the file. WS1 collapses this into
one method:

```rust
fn read_include(&mut self, fname: &[u8]) -> Option<Vec<u8>>;
```

WS4 (`SleighCompile`) must implement it as: `parseFromNewFile(fname)` (record the
new current file / push the location stack) → `grabCurrentFilePath()` → read the
file, returning its bytes (or `None` → the lexer raises `preproc_error`, which WS4
routes through its error machinery via the `ScannerHost::preproc_error` default).

`ScannerHost::preproc_error(&mut self, msg) -> !` was also added (default `panic!`)
for the C++ `preproc_error`'s `exit(1)`; WS4 may override it.

No other `TokenKind`/`TokenValue`/`ScanState` change was needed — the token set
matches the bison enum (`slghparse.hh:74-191`) the skeleton transcribed. A new
public `SymbolTokenKind` enum was added as the `find_symbol_kind` return projection
(it carries only the token-kind decision the lexer needs; the parser re-resolves
the actual `SleighSymbol`).

## Notes for parity

- The `print`-state `^` rule returns the literal char token `'^'` (not `CHAR`);
  modeled as `Char1(b'^')` carrying a `Char(b'^')` value (C++ sets `sleighlval.ch`).
- String literals keep doubled `""` **verbatim** (C++ stores `string(text+1,
  len-2)` with no un-escaping).
- The `^@…` directive rule and the `<preproc>^.*\n` erasure rule are column-0
  anchored; the scanner tracks an `at_line_start` flag updated on each consume.

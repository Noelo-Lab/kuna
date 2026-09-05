## The problem

`--assert prototype` rejects the standard C scalar types. `int`, `unsigned int`,
`long long` and `double` are all a syntax error, in return position and in
parameter position alike, while Ghidra's internal `int4` / `uint4` / `float8`
spellings work. Five testers filed it independently in one round.

```console
$ kuna decompile decompiler/crates/kuna-analysis/tests/fixtures/fauxware authenticate \
    --assert 'prototype authenticate unsigned int authenticate(char *user,char *pass)'
warning: --assert "prototype authenticate unsigned int authenticate(char *user,char *pass)" rejected: Syntax error at line 0 in stream
extern unsigned int 
                ^--
```

The declaration kuna itself prints cannot be pasted back at it — the same
function's header is `unsigned long authenticate(char *a0,char *a1)`. The
worked example in `docs/cli.md` was one of the rejected forms.

## The fix

- The console's C-declaration grammar learns the standard scalar specifiers —
  `void`, `char`, `short`, `int`, `long`, `float`, `double`, `signed`,
  `unsigned`, `_Bool`, `wchar_t` — in any legal combination, everywhere a type
  may appear.
- Widths come from the compiler spec's `<data_organization>`, the same fields
  the printer's C speller reads, so `long` is 8 bytes on LP64 and 4 on LLP64 and
  the two halves cannot disagree. `signed char` is a 1-byte integer; plain
  `char` stays the text type.
- Additive by construction: a run of exactly one keyword is resolved by
  `findByName` first, so `void`, `char` and any host-supplied type spelled with a
  keyword resolve to exactly the type they did before. Only combinations — which
  were 100% syntax errors — take the new path. None of the 261 `parse line`
  payloads in the two parity corpora uses any of the eleven keywords.
- An impossible combination (`short long`, `float int`, three `long`s) is
  rejected as `Invalid combination of C type specifiers: short long` rather than
  a bare "Syntax error" at the second word, and a keyword whose width the
  compiler spec never declared is rejected rather than resolving to a zero-sized
  type.

## The tests

Seven unit tests in `kuna-console/src/grammar/tests.rs` (the 20-spelling
resolution table, `long` under LP64 vs LLP64, `char` vs `signed char`, nine
impossible combinations, and the `findByName`-still-wins guarantee), two
end-to-end tests in `verify_assertplane.rs` driving the real `--assert` path,
and the promoted probe `tests/cli/prototype-assertions-reject-ordinary.json`.
All three of that probe's directives are rejected before this change and applied
after.

Gates: `make test` PARITY OK 675/675 · `make test-stages` PARITY OK ·
`make rust-test` green · `make check-spec` OK · `kuna catalog --check` OK ·
`make test-cli` 32/32.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

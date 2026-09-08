## The problem

A parameter cannot be named after a type. `code` is one of the core types every
compiler spec registers, so a declaration that names a parameter after it is
rejected with the caret on the parameter's own name:

```bash
kuna decompile ./fauxware authenticate --json \
  --assert 'prototype authenticate unsigned long vm(unsigned char *code,unsigned int index,void *ctx)' \
  --assert 'prototype read int read(unsigned char code)'
```

```text
warning: --assert "prototype authenticate unsigned long vm(unsigned char *code,unsigned int index,void *ctx)" rejected: Syntax error at line 0 in stream
extern unsigned long vm(unsigned char *code,
                                       ^--

warning: --assert "prototype read int read(unsigned char code)" rejected: Multiple type specifiers at line 0 in stream
extern int read(unsigned char code)
                              ^--
```

Renaming `code` to anything else makes the same declaration apply. Every
interned name is in this class, not just the core types: a tag or `typedef`
declared earlier in the same run, and on a `-g` binary every DWARF type name the
program uses.

## The fix

`CParse::lookup_identifier` classifies any spelling `findByName` answers as
`TYPE_NAME`, and two positions had to learn that C reads such a token as a name
when a name is what belongs there:

- The specifier run stops at a `TYPE_NAME` once it has already named a type
  (`declaration_specifier_starts`, and its `specifier_qualifier_list` twin
  inside a struct body). That is the `unsigned char code` half.
- The declarator's name position takes one (`declarator_identifier`), reached
  from `direct_declarator`, `parameter_declaration` and the abstract-declarator
  paths. That is the `unsigned char *code` half, and it also covers
  `int4 (*code)(void)`, a struct field, an enum constant and the tail of an
  `a::b` scoped name.

Only the *unparenthesised* name position moved. `int4 (code)` is genuinely
ambiguous in C and keeps its abstract reading — a function of one `code` — so
the one case where a heuristic would have to guess is left alone.

The change is monotone: both positions it touches were hard errors before
("Syntax error", "Multiple type specifiers"), so no declaration that parsed
parses differently. The 263 `parse line` payloads across `tests/datatests/` and
`tests/stages/` were replayed through the old and new `decomp_dbg` byte-for-byte
identically.

## The tests

Six cases in `kuna-console/src/grammar/tests.rs` (both spellings, through
pointers/arrays/function pointers, a struct member, an enum constant, a scoped
name) plus the pins that `code *p` is still a pointer to `code` and `int4 (code)`
is still a function; one end-to-end case in `verify_assertplane.rs` asserting the
declared name reaches the emitted C; and the promoted probe
`tests/cli/prototype-parser-rejects-valid.json`, whose two directives both read
`rejected` on the unpatched tree.

Gates: `make test` PARITY OK 675/675 · `make test-stages` PARITY OK ·
`make rust-test` green · `make check-spec` green · `make test-cli` 91/91 (including the promoted probe) ·
`kuna catalog --check` OK.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

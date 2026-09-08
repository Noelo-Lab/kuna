## The problem

A struct tag that kuna has just accepted cannot be used as a type. Declare
`struct JSValue`, then name it as a prototype's return type, and the prototype
is thrown away as a syntax error — a typedef alias for the same structure works,
which is what makes it look arbitrary.

```bash
kuna decompile decompiler/crates/kuna-analysis/tests/fixtures/fauxware main \
  --assert 'typedef struct JSValue { unsigned long payload; long tag; };' \
  --assert 'prototype authenticate struct JSValue authenticate(char *user, char *pass)' \
  --assert-strict
```

```
  v2 = authenticate(v1,v3);
  ...
warning: --assert "prototype authenticate struct JSValue authenticate(char *user, char *pass)" rejected: Bad C syntax
exit 1
```

The console shows it without the CLI in the way:

```
[decomp]> parse line struct JSValue { unsigned long payload; long tag; };
[decomp]> parse line struct JSValue v;
Error in C syntax: Syntax error at line 0 in stream
struct JSValue
       ^--
```

## The fix

- The lexer classifies an identifier by asking the type factory for it, so
  interning the tag is what stops `JSValue` reaching the parser as an
  identifier: it comes back as a type name, and `struct_or_union_specifier`
  read its tag from the identifier terminal alone. The tag position now accepts
  a type name too (`CParse::tag_identifier`), and `enum_specifier` reads its tag
  through the same helper.
- The position is unambiguous rather than a guess: a type name after `struct` /
  `union` / `enum` matched no production before, with or without a body, so
  every input this newly accepts used to be a syntax error.
- Which type the tag names is still decided by the construction action, not by
  the token. `oldStruct` re-checks the kind by name, so `struct int4` is now
  refused for saying `struct` ("Identifier does not represent a struct as
  required") instead of for being unparseable.

## The tests

Five cases in `kuna-console/src/grammar/tests.rs` cover the tag as a return
type, as a pointer parameter, outside a prototype, and for `union` / `enum`;
each fails without the change. `reject_struct_type_name_is_syntax_error` pinned
the old expectation and now asserts the kind error under its new name.
`tests/cli/prototype-parser-rejects-previously.json` is the promoted probe.

Gates on the rebased tree: `make test` PARITY OK 675/675, `make test-stages`
PARITY OK, `make rust-test` green, `make check-spec` OK, `make test-cli` 81/81,
`kuna catalog --check` OK. All 263 `parse line` / `parse file` payloads in
`tests/datatests` + `tests/stages` were replayed through the old and new
`decomp_dbg` in their original per-file order: zero output differences.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

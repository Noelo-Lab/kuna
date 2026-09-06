## The problem

`--assert 'prototype <func> …'` documents `<func>` as "which function this
signature describes", but it was only ever looked up as a **name**. An address
is not a name, so an assertion written at an address was parsed, reported
`applied`, and then discarded — the call site kept whatever kuna had recovered.

Against the vendored fixture, `accepted` at `0x4006ed` takes no arguments and
the directive says it takes one:

```
$ kuna decompile decompiler/crates/kuna-analysis/tests/fixtures/fauxware main \
    --json --assert 'prototype 0x4006ed int4 accepted(int4 status)'
```

```
"status": "applied", "detail": null
...
  v2 = authenticate(v1,v3);
  if (v2) {
    accepted();
```

Same shape on the reported image (crackmes.one `640a526833c5d447bc761899`), where
`sqrt` is a PE import thunk at `0x140003ddf` and its four call sites read a value
nothing assigns:

```
$ kuna decompile KeyCheker.exe sub_140001890 --option calloverlap full \
    --assert 'prototype 0x140003ddf float8 sqrt(float8 x)'
      v26 = (double)sqrt();
```

## The fix

- `<func>` is resolved as a name first and an **entry address** second, by one
  shared resolver both surfaces call — `kuna decompile`'s generated `map
  prototype` script line and the in-process `--json` / `decompile-all` path had
  already drifted apart once on this directive.
- An address park goes through `set_function_prototype_pieces_at`, which is the
  key the read side (`ArchContext::callee_proto_pieces`, consulted per call
  site) already uses. That is also why the name form cannot always express this:
  the thunk and the IAT slot it jumps to are two symbols both called `sqrt`, and
  the by-name query answers with the slot while every call goes to the thunk.
- A `0x`-prefixed operand that starts no function is now **rejected**, naming the
  address. A bare hex token stays ambiguous with an identifier (`abc` is both),
  so it takes the address path only when nothing of that name exists, and never
  errors.
- The same resolution serves the `param <func>::<i>` / `return <func>::<storage>`
  qualifier.

```
      v26 = sqrt(v35._0_8_);
```

## The tests

`tests/cli/accepted-sqrt-prototype-still.json` (promoted acceptance) plus four
cases in `verify_assertplane.rs` and two in `decompile_cli.rs` — the selected
function by address, a callee by address, an unbindable address rejected by
address, and a `param` qualifier by address; the CLI ones assert the text and
`--json` surfaces agree. All six fail on the unpatched tree.

`make test` 675/675 PARITY OK · `make test-stages` PARITY OK · `make rust-test`
5,779 passed · `make check-spec` OK · `make test-cli` 54/54 · `kuna catalog
--check` OK. No option, no baseline moved.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

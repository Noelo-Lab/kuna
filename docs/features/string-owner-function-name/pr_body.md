## The problem

kuna calls a function no symbol covers `sub_<addr>`, and it prints such a name for
entries its own inventory does not hold — then refuses the name it just printed.
On a fixture already in the tree:

```console
$ kuna decompile decompiler/crates/kuna-analysis/tests/fixtures/tailcallframe_x86_64 sub_11b0
void sub_11b0(int *a0,int a1)
{
  *a0 = a1;
  if (1 <= a1) {
    a0[1] = (int)strlen((char *)&a0[4]) + 1;
    sub_1170(a0); // warn: tailcallframe: recovered tail call -> introduced call to 0x00001170
    return;
  }
  a0[1] = 0;
}

$ kuna decompile decompiler/crates/kuna-analysis/tests/fixtures/tailcallframe_x86_64 sub_1170
error: no function "sub_1170" in ...; for a stripped binary pass an address with --addr

$ kuna decompile decompiler/crates/kuna-analysis/tests/fixtures/tailcallframe_x86_64 --addr 0x1170
void sub_1170(int *a0) { ... }
```

It was reported off `kuna strings`, which names a literal's owner from the reference
walk's own flow attribution and so reaches starts discovery never recorded: it
reported `sub_100a3be` as the owner of `"No error information"` while
`kuna decompile <bin> sub_100a3be` answered `no function` and `--addr 0x100a3be`
decompiled it (1 instance, RE-need `string-owner-function-name`).

## The fix

- A by-name selection that matches nothing is retried as the address the name
  spells, in `ConsoleProgram::resolve_entry` and `find_entry_by_name` — the two
  name lookups every surface goes through, so `decompile`, `decompile-all
  --functions`, `disassemble` and `xrefs` all answer one name the same way.
- Decided by **minting**, not by parsing: the candidate offset is rendered through
  `Architecture::name_function` and accepted only if it comes back as the requested
  name. The active naming style (`sub_`/`func_`/`FUN_`) and a word-addressed
  space's scaling of the printed offset then follow for free, and a name this build
  would never print is not resolved.
- Gated on the address holding mapped bytes — the numeric selector's own test — so
  a name resolved this way reaches exactly what `--addr` on that address reaches.
- Only fires on a miss, so a binary that really does carry a symbol spelled like a
  placeholder still wins, and every selection that already resolved is untouched.

## The tests

`tests/cli/string-owner-function-name.json` (the promoted acceptance, re-pointed at
the vendored `tailcallframe_x86_64` since CI has no dataset) plus two cases in
`kuna-console/tests/verify_entry_selectors.rs`: the placeholder resolves to the same
entry `Numeric(0x1170)` does, and `sub_deadbeef` / `FUN_00001170` / `func_00001170` /
`handler_1170` / `sub_` all still miss. All three fail with the change reverted.
Gates: `make test` 675/675 PARITY OK, `make test-stages` PARITY OK, `make rust-test`
green, `make check-spec` OK, `make test-cli` 74/74, `kuna catalog --check` OK.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

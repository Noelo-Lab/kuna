`kuna functions` prints a name for every function it finds; feeding one of those
names back to `kuna decompile` stops working the moment the run also asks for a
different naming style.

```bash
$ kuna functions ./stripped_dynamic_x86_64 --json | grep 1249
    {"name": "sub_1249", "address_hex": "0x1249", "aliases": [], "size": 270}

$ kuna decompile ./stripped_dynamic_x86_64 sub_1249 --option namestyle ghidra
error: no function "sub_1249" in ./stripped_dynamic_x86_64; for a stripped binary
pass an address with --addr
$ echo $?
1

$ kuna decompile ./stripped_dynamic_x86_64 func_0x00001249 --option namestyle ghidra
void * func_0x00001249(void *param_1)          # the same function, decompiled in full
```

(`decompiler/crates/kuna-analysis/tests/fixtures/stripped_dynamic_x86_64` is in
the repo. Reported by an agent against a 14 KB crackme, where the name was
`sub_15dc`.)

## The fix

- A generated name like `sub_1249` holds nothing but an address, so a by-name
  miss is already retried as the address it spells. That retry accepted the name
  only if the *active* naming style would mint it — and `option namestyle`
  changes which style is active, which made the styles disjoint name spaces.
- `ConsoleProgram::placeholder_name_address` now accepts the name if **any**
  style would mint it there: `sub_<addr>`, `func_<addr>` or `FUN_<addr>`. The
  three spellings come from the same three functions `Architecture::name_function`
  calls, so nothing is parsed and nothing can drift.
- Unchanged: the retry still runs only after the name match found nothing, and
  still requires mapped bytes at the address. A binary that really carries a
  symbol spelled that way still wins, `sub_deadbeef` still misses, and no name
  that already resolved can move.
- `kuna functions` is deliberately untouched — it reports the default spelling
  whatever the run asks for, and now that spelling always selects.

## Tests

`tests/cli/changing-namestyle-invalidates-discovered.json` (promoted, over the
in-repo fixture) and two cases in `verify_entry_selectors.rs`: a placeholder
resolves in all three styles, and a default-style name survives a namestyle
flip. All three fail on the unpatched tree. One existing case moved — it asserted
that `FUN_00001170` misses under angr naming, which is the behaviour this
changes; `func_00001170` (wrong spelling for every style) still misses in its
place.

Swept every name and alias `kuna functions` reports across all 181 vendored
fixtures, at both naming styles, on the pre- and post-fix binaries: 4,856
selections, 3,566 identical, 1,290 exit 1 -> exit 0, 0 changed. Every unlocked
one is a `namestyle ghidra` run; the default-style column is byte-identical.

Gates: `make test` 675/675, `make test-stages` 714/714, `make test-cli` 103/103,
`make rust-test` green, `check-spec --strict` OK, `catalog --check` OK.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

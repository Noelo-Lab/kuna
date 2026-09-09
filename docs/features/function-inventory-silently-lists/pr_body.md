## The problem

`kuna functions` reports an inventory of import pointer slots as a healthy
listing. On a packed PE that yields no code at all it exits 0 with
`error: null`, while `kuna decompile-all` on the same file names the cause and
exits 1. The fixture below is the reduced witness (crackmes.one
`5ab77f5433c5d40ad448c1c1`, a NEOLite-packed crackme: six imported names,
`error: null`, exit 0):

```console
$ python3 decompiler/crates/kuna-analysis/tests/fixtures/pe_dataimports_i386.py
$ kuna functions .../pe_dataimports_i386.exe --json ; echo "exit $?"
{ "count": 3, "total": 3, "error": null, "functions": [
    { "name": "GetProcAddress",   "address_hex": "0x403000", "size": 0 },
    { "name": "GetModuleHandleA", "address_hex": "0x403004", "size": 0 },
    { "name": "LoadLibraryA",     "address_hex": "0x403008", "size": 0 } ] }
exit 0

$ kuna decompile-all .../pe_dataimports_i386.exe --json ; echo "exit $?"
error: no functions discovered in ...: its entry point 0x402001 lies in section
.stub, which the image does not flag executable -- pass `--define-function
0x402001` to decompile there anyway
exit 1
```

Not one of the three is a body — they are Import Address Table words, kept so a
call to an import renders its name. The diagnostic was already wired into
`functions`; its gate asked `all.is_empty()`, and three pointer slots are not
empty.

## The fix

- `functions` and `functions --summary` now read the zero-discovery verdict off
  the entries a whole-binary run would decompile, which is what `decompile-all`
  already does, so the three surfaces agree by construction.
- The predicate is `function_entries_executable`, not "has a nonzero `size`": it
  excludes a slot on the loader's own import-slot ranges rather than on a flag,
  and on this class of image the inventory extent is 0 for *every* entry, body or
  not — a size test would fire on images whose bodies were all found.
- New `ConsoleProgram::any_executable_entry` answers that over an inventory the
  caller already holds: one section-table walk, no second enumeration.
- The import names still print beside the error — they are what the packed stub
  is going to call. A `--define-function` declaration still clears the verdict.
  The exit code follows the error automatically, as `tests/cli/zero-functions-exit-0.json`
  already pins for this command.

## Tests

`triage_cli.rs` drives the built binary over the new fixture (both surfaces, plus
the declaration that clears it) and fails on the unpatched tree with exit 0
instead of 1; two `kuna-console` tests pin the predicate and its agreement with
`function_entries_executable` on five fixtures;
`tests/cli/function-inventory-silently-lists.json` is the promoted probe.

Swept `kuna functions --json` over 940 RE-dataset files, 250 `/usr/bin` ELFs and
all 222 `kuna-analysis` fixtures for the only shape this can create — `count > 0`
with an error. 0 in `/usr/bin`, 0 in the fixtures, 5 dataset images, and
`decompile-all` already answered `count: 0` with the same error on all five.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

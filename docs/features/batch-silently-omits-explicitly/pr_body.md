## The problem

`--define-function` declares where a function starts and ends, and every kuna
surface honours it except the one that decompiles a whole binary. On a PE that
flags all of its sections as data — which is what a packer produces, since setting
`IMAGE_SCN_MEM_EXECUTE` is the packer's choice and NEOLite does not — the
declaration enumerates and decompiles fine one function at a time, and the batch
drops it and reports success.

```console
$ python3 decompiler/crates/kuna-analysis/tests/fixtures/pe_datasection_entry_i386.py
wrote pe_datasection_entry_i386.exe (1536 bytes)
$ F=decompiler/crates/kuna-analysis/tests/fixtures/pe_datasection_entry_i386.exe

$ kuna functions $F --json --define-function 0x402001-0x40200c=entry
{ "count": 1, "functions": [ { "name": "entry", "address_hex": "0x402001", "size": 11 } ] }

$ kuna decompile $F entry --define-function 0x402001-0x40200c=entry
void entry(void) { sub_401000(); }

$ kuna decompile-all $F --json --define-function 0x402001-0x40200c=entry
{ "count": 0, "error": null, "functions": [] }
$ echo $?
0
```

The same three commands on the reported image (crackmes.one
`5ab77f5433c5d40ad448c1c1`, a NEOLite-packed `CryptoME.exe` whose six sections all
carry `0xc0000040`) give `count: 7`, `void entry(void) { sub_4f7990(); }`, and
`count: 0`. A bare `kuna decompile-all` on it also exits `0` with a null error, so
nothing anywhere tells the caller the run failed.

## The fix

- `ConsoleProgram::function_entries_executable` — the target list behind
  `decompile-all`, `decompile-project`, `decompile_graph` and the wasm walk — keeps
  a caller-declared entry whatever the section flags say. The `CODE` test is a
  guess about where code lives; a declaration is an assertion, and it outranks it.
- The declared entry VMAs are a new set rather than a reuse of `declared_extents`,
  so a bare `--define-function 0x402001` (no extent) works too. `declare_function`
  writes it, so the console `function bounds` spelling gets the same behaviour.
- A run that discovers nothing is now a failure when the image declares an entry
  point inside one of its sections, not only when a section or segment is flagged
  executable. The message names the address and the section and points at
  `--define-function`. A resource-only PE keeps its honest empty answer: with no
  `AddressOfEntryPoint` it is reported as entering at its bare image base, which no
  section covers.

## The tests

`verify_declared_entry_batch.rs` asserts the fixture enumerates nothing until the
declaration and exactly the declared entry after it, and — on `ptx.o` — that
declaring one dropped address does not lift the others. Two probes in `tests/cli/`
pin both halves against the checked-in fixture; both fail on the unpatched tree.
`make test` 675/675, `make test-stages` 677/677, `make test-cli` 75/75,
`check-spec` and `catalog --check` OK. A pre/post `decompile-all --json` sweep over
123 in-repo fixture binaries and 2,973 functions changed no emitted C and no
exit code except the new fixture's.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

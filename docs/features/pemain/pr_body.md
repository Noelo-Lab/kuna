kuna names `main` on stripped ELF and Mach-O images but never on PE: the entry is the CRT startup, and the function it calls is one more `sub_<addr>`. On the in-repo stripped MinGW fixture:

```
$ kuna functions decompiler/crates/kuna-analysis/tests/fixtures/pe_imports_stripped.exe --summary
entry	0x1400014f0	sub_1400014f0
reachable from entry	70
...
$ kuna decompile decompiler/crates/kuna-analysis/tests/fixtures/pe_imports_stripped.exe 0x1400011a0 --addr | grep 140001592
  dat_14000c010 = sub_140001592(dat_14000c028,dat_14000c020);
```

After this change the same commands give `main	0x140001592	main` in the summary and `dat_14000c010 = main(dat_14000c028,dat_14000c020);` at the call site. `kuna decompile <bin> main` and `kuna xrefs <bin> --to main` work too.

## The fix

- New analysis pass `pemain` (default on, `--option pemain off` restores the old inventory). It finds the function the in-image CRT startup calls and names it `main`, `wmain`, `WinMain` or `wWinMain`:
  - MSVC dynamic UCRT, console: the `__p___argc`/`__p___argv`/`_get_initial_*_environment` call cluster that `entrymainproto` already scans for.
  - MSVC dynamic UCRT, GUI: `_get_{narrow,wide}_winmain_command_line`, then the next local call whose argument setup references `__ImageBase`.
  - MinGW: the `__initenv = envp` store, direct or through a `.refptr` cell, just before `main(argc, argv, envp)`.
  - MSVC static CRT (x86-64): the `invoke_main` call shape, used only when it matches exactly one site in the image. Incremental-link `jmp` thunks are followed.
- It never renames a function that already has a symbol or export, and it never uses a name the image already defines.
- `entrymainproto` now parks its prototype by address, so `main` keeps its `argc/argv/envp` slots after the rename.
- `functions --summary` gains a `main` field (`{name,address,address_hex}` or `null`) and a `main` line in text mode.

## Tests

- `tests/stages/kuna-pemain.xml` has two passes: off shows `sub_140001592(...)` and on shows `main(...)`. `tests/cli/pemain.json` checks the summary field.
- New cargo tests: `kuna_pemain` unit tests (MinGW stripped vs unstripped twin, ELF/Mach-O inert), `triage_cli::summary_reports_the_pe_user_entry`, and `verify_entrymainproto::the_prototype_composes_with_pemain`.
- Across all in-repo PE fixtures, only the four with a real CRT startup change, and each gets `main` at the right address. It also names `main` correctly on three real MSVC samples (dynamic UCRT, a Qt app, and a debug static-CRT build).

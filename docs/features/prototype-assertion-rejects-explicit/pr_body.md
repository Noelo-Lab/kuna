## The problem

`kuna decompile` emits the function at an address and then rejects an assertion
that names that same address. Any vendored fixture shows it — `0x400678` is an
instruction boundary inside `authenticate` that discovery does not call a
function start:

```console
$ kuna decompile decompiler/crates/kuna-analysis/tests/fixtures/fauxware 0x400678 --addr \
    --assert-strict --assert 'prototype 0x400678 int4 checkpw(char *user,char *pass)'
warning: --assert "prototype 0x400678 int4 checkpw(char *user,char *pass)" rejected: no function starts at 0x400678
unsigned long sub_400678(void) // return-dupe x2
{
  ...
}
$ echo $?
1
```

The body is right there. Selecting the *same* function by name binds the
identical directive.

## The fix

- `load addr <vma>` builds the `Funcdata` and follows flow without installing a
  `FunctionSymbol`, so the by-address selection path left nothing for the
  assertion plane to resolve against. Pointing `--addr` at an address is the
  claim that a function starts there, so the generated console script now
  declares that entry — the same install `--define-function <start>` performs.
- The declaration sits between the caller's own `--define-function` lines and the
  program-scoped directives, and is skipped when a `--define-function` already
  names that start: a second bare declaration would clear the extent it asked
  for (`declare_extent(vma, 0)` removes one).
- Nothing in the resolver changed. An operand naming some *other* address that
  starts no function is still rejected — that rejection is the only signal an
  agent gets that a directive is inert.
- Two latent defects in `ConsoleProgram::declare_function` that the new caller
  exposed: declaring an import's synthetic address downgraded its
  `UndefinedExternal` provenance to `Mapped`, after which `resolve_entry`
  answered `not mapped in this input` for an entry it used to select; and the
  ARM/Thumb mode bit was not folded, so a declaration at `entry|1` landed on an
  odd shadow no load would find.

```console
$ kuna decompile ... 0x400678 --addr --assert-strict --assert 'prototype 0x400678 int4 checkpw(char *user,char *pass)'
int sub_400678(char *user,char *pass) // return-dupe x2
$ echo $?
0
```

## The tests

`tests/cli/prototype-assertion-rejects-explicit.json` (the promoted acceptance
probe) plus four cargo tests: the end-to-end bind, the still-rejected unselected
address, and one each for the provenance and Thumb guards. The bind test and both
guards fail on the unpatched tree.

Sweep: 464 by-address decompiles across 41 vendored fixtures (ELF x86-64/i386/
ARM/Thumb/Cortex-M/PPC64, PE, Mach-O, ET_REL, COFF) plus the reporting image,
before vs after — 0 differences in C, stderr or exit code.

Gates: `make test` PARITY OK 675/675 · `make test-stages` PARITY OK 674/674 ·
`make rust-test` green · `make check-spec` OK · `make test-cli` 63/63 ·
`kuna catalog --check` OK.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

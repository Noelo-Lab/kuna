## The problem

`--assert type <local> <T>` reaches the locals that sit on the stack and dies on
the ones that sit in a register — which is most of them. The register locals are
the ones kuna prints with a storage comment:

```
$ kuna decompile decompiler/crates/kuna-analysis/tests/fixtures/cet_pie_x86_64 \
      get_elf_header --assert 'type v1 unsigned int *' --json | jq -r '.assertions[0] | "\(.status): \(.detail)"'
rejected: No symbol named: v1
```

`v1` is the local three lines down in that function's own output:

```
Elf64_Ehdr * get_elf_header(_IO_FILE *fp)
{
  unsigned long *v1;  // rax
```

`name v1 hdr` fails the same way, and so does `type v2 char[16]` — the form
`kuna decompile --help` gives as its worked example. Reported once against
crackmes.one `69761b7a39e9c4d85c2f9fc1` (graphy `sub_1005350`), where the whole
VM state lives in `rax`.

## The fix

- When the local scope answers nothing, look the printed identifier up among the
  HighVariables and map an isolated, locked Symbol over that variable's storage —
  the mapping `type varnode %RAX(pc)` already made, keyed by the name the printer
  chose rather than by a hand-written varnode specifier.
- The symbol carries the representative's **use point**. Mapped without one it
  matches every read of the register, two highs bind it, and the printer declares
  the storage twice — invalid C, not a cosmetic difference.
- A bare `type` leaves the symbol **nameless** for the naming pass to number.
  Binding the printed `v6` back as a namelocked symbol keeps the caller's name and
  is also invalid C: the `vN` allocator never consults the scope, so it hands the
  same `v5` to an unrelated temporary. `type v6 <T> <newname>` pins a name.
- Three targets are rejected rather than accepted and dropped, because `applied`
  over unchanged output is the failure this plane exists to end: a decompiler
  temporary (no location that survives the IR rebuild), a type whose width differs
  from the storage (`char *` at EAX's address is RAX), and storage a second named
  variable is represented by (both would take the symbol's name).

Tooling track: the fallback only runs when a caller passed a symbol-scoped
`--assert`, so no run without one reaches new code. No option, no catalog
counter, no DIV row.

## The tests

`tests/cli/local-type-assertion-target.json` is the promoted acceptance, pinned to
the in-repo `cet_pie_x86_64` twin and asserting the retyped declaration, not just
the status — it fails on `main`. Five cases in
`kuna-console/tests/verify_assertplane.rs` cover the type, the name, each of the
rejections, and the batch case where an earlier `name` renamed the symbol a later
`type` names.

Sweep: every printed local of 9 graphy functions × 2 directives (168 runs) — 0
duplicate declarations, 0 undeclared identifiers. `make test` 675/675 PARITY OK,
`make test-stages` PARITY OK, `make rust-test` green, `make check-spec` OK,
`kuna catalog --check` OK, `make test-cli` 114/114.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

## The problem

`--assert 'prototype <func> <decl>'` is dropped, without a word, whenever the
declaration is written under a different name than the function has — which is
the case an agent actually reaches for, since the reason to declare a signature
is usually that the recovered one is wrong and the name is `sub_...`. It exits 0
even under `--assert-strict`, and `--json` applies the same directive.

```console
$ kuna decompile ./fauxware authenticate \
    --assert 'prototype authenticate void *hashit(void *out,void *input)' \
    --assert-strict | head -1
unsigned long authenticate(char *a0,char *a1)

$ kuna decompile ./fauxware authenticate \
    --assert 'prototype authenticate void *hashit(void *out,void *input)' \
    --assert-strict --json | grep -m1 -E '"status"|void \*'
      "status": "applied",
```

(`fauxware` is `decompiler/crates/kuna-analysis/tests/fixtures/fauxware`, in the
repo. Spell the declaration `void *authenticate(...)` instead and the text
surface applies it — the name in the declaration is the discriminator, not the
output format.)

## The fix

- The generated console script lowered every `prototype` directive to
  `parse line extern <decl>`, which keys the parsed signature by the name inside
  the declaration (`Architecture::setPrototype`'s `queryFunction(basename)`), so a
  renaming declaration parked a prototype on a fresh symbol nothing referenced.
  `<func>` was discarded at the lowering.
- The console gains `map prototype <func> <C declaration>`, which parks the pieces
  under `<func>` — the rule the in-process surface already applied
  (`assertions::apply_prototype` overwrites `PrototypePieces::name`). `parse line
  extern` keeps its upstream meaning; no ported command changed.
- The CLI lowers `prototype` to that command, so both surfaces now share one
  semantics instead of two implementations that happened to agree for same-name
  declarations.
- The declaration's own name still does not rename the function, and `docs/cli.md`
  now says so: `--assert` has no function rename (`name` renames a local).

## The tests

`a_prototype_declared_under_another_name_binds_on_both_surfaces` (kuna-cli, drives
the real `decomp_dbg` on the fixture and fails on the unpatched tree),
`a_declaration_written_under_another_name_still_binds_to_its_target`
(kuna-console, the in-process twin), three guards for the new command including
that `map param`/`map addr`/`map fun` still resolve, and the promoted probe
`tests/cli/text-output-silently-ignores.json`. Gates: `make test` PARITY OK
675/675, `make test-stages` PARITY OK 635/635, `make test-cli` 38/38,
`make check-spec` OK, `kuna catalog --check` OK, `make rust-test` green.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

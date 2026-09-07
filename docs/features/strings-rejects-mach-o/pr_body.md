## The problem

A Mach-O universal ("fat") binary loads for `kuna functions` and dies for
`kuna strings`, `kuna xrefs`, and `kuna functions --summary` — on the same file,
in the same run. The vendored 2-slice fixture shows it without a dataset:

```
$ kuna functions decompiler/crates/kuna-analysis/tests/fixtures/macho_fat --json | head -4
{
  "binary": "decompiler/crates/kuna-analysis/tests/fixtures/macho_fat",
  "count": 6,
  "total": 6,

$ kuna strings decompiler/crates/kuna-analysis/tests/fixtures/macho_fat --json
error: could not parse decompiler/crates/kuna-analysis/tests/fixtures/macho_fat: Unsupported file format

$ kuna functions decompiler/crates/kuna-analysis/tests/fixtures/macho_fat --summary --json
error: could not parse decompiler/crates/kuna-analysis/tests/fixtures/macho_fat: Unsupported file format
```

`--slice x86_64` fails identically: the token was parsed and threaded into the
reference walk, but nothing read it before the parse that failed.

## The fix

- `object::File::parse` has no fat arm, so a universal image must be peeled to
  one slice's bytes first. That peel existed at exactly one point — the engine
  dispatch — while the surfaces that re-parse the image for themselves read the
  raw file. The peel is now a shared function (`macho_fat::peel_fat_image`)
  applied at the canonical image read (`elf_shdr::read_image`), where the ELF and
  PE header repairs already live, and the dispatch calls the same policy.
- `--slice`/`--target` resolve through one function (`macho_fat::slice_pref`), so
  the override steers the re-parsing surfaces too instead of being inert on them.
  `--slice x86_64` and `--slice arm64` now select visibly different inventories.
- The remaining raw `std::fs::read` + parse sites move onto `read_image` as well.
  That also fixes an unrelated instance of the same shape: `--summary` reported
  `entry: null` on an image that only parses after the loader's header repairs
  (a corrupt ELF section table, an oversized PE data-directory count).

## The tests

`tests/cli/strings-rejects-mach-o.json` and `tests/cli/adding-summary-makes-working.json`
are the promoted acceptance probes, retargeted onto the in-repo `macho_fat`
fixture (CI has no dataset) with clauses that an empty result cannot satisfy:
the strings scan must find the x86-64 slice's own literal, and the summary's
count must equal the plain inventory's. Both fail on `main`. Cargo cases cover
the peel at `read_image`, the slice override on both surfaces, and the repaired
header entry.

Gates: `make test` 675/675 PARITY OK, `make test-stages` 677/677 PARITY OK,
`make check-spec` OK, `catalog OK`, `make test-cli` 70/70, `make rust-test` green.
An old-vs-new sweep over all 185 fixtures moved `macho_fat` only, plus the two
repaired-header summaries above.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

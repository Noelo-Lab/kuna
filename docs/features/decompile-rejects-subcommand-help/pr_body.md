Asking a command to describe itself is the first thing anyone does with an
unfamiliar CLI, and four of `kuna`'s sixteen subcommands answered it with an
error. Two testers filed the same report against `decompile`; `test`, `catalog`
and `specs` sat one command away with the same defect.

```
$ kuna decompile --help
error: unknown option --help
$ echo $?
2

$ kuna decompile -h                 # -h does not start with `--`, so it was a positional
error: decompile requires <binary> and <func>
$ kuna catalog -h
error: unexpected argument "-h"
$ kuna specs --help                 # forwarded straight to slacomp
Unknown option: --help
```

The other twelve subcommands each print a usage block and exit 0.

## The fix

- The `-h | --help` arm the twelve already had, added to the four that lacked it:
  `decompile.rs`, `main.rs`'s `cmd_test` and `cmd_catalog`, and `specs.rs` ahead
  of the `slacomp` passthrough (slacomp owns no help flag, so the alias has to
  describe itself).
- Each prints its **own** multi-line block, not the one-line summary `kuna --help`
  already carried for it. The report asked to "discover decompile flags through
  subcommand help", and the one-liner names no `--assert` vocabulary, no
  `--define-function` contract and no `--json` shape.
- `scripts/repipe/clitests.py` now asks `verify.vendorable()` whether a promoted
  probe can run instead of re-deriving a stricter rule of its own. The two had
  disagreed, harmlessly until now: `vendorable()` admits a probe that needs no
  binary, `clitests` demanded `target.binary_source == "in-repo"` and refused one.
  `kuna decompile --help` is the first binary-less probe the corpus has held, so
  it failed `make test-cli` the moment it was promoted.

## Tests

`tests/cli/decompile-rejects-subcommand-help.json` is the promoted acceptance
probe. `decompiler/crates/kuna-cli/tests/subcommand_help.rs` (5 tests) asserts the
contract over the **whole** dispatch table rather than the reported case — exit 0,
its own usage block, no binary and no `.sla` required, the repaired blocks naming
the flags that were being looked for, and an unknown option still exiting 2. Four
of the five fail without the fix; the fifth is the guard that the new arm did not
swallow a real usage error, and passes either way.

Gates: `make test` 675/675 PARITY OK · `make test-stages` 635/635 PARITY OK ·
`make rust-test` green · `make check-spec` OK · `make test-cli` 42/42 (41/42 with
the probe promoted and `clitests` unchanged) · `kuna catalog --check` OK.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

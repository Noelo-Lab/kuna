## The problem

One `--assert 'flow ...'` that kuna cannot apply deletes the whole function and
reports success. The text surface exits 0 with an empty stderr and a body that is
not a body:

```console
$ kuna decompile ./aif_gap_x86_64 --addr 0x13c9 --assert 'flow 0x1405 call'; echo "exit $?"
void sub_13c9(void)
{
  /* WARNING: structured blocks unavailable (structuring declined) */
}
exit 0
```

Drop the `--assert` and the same command prints 55 lines of real C. Add `--json`
to the failing one and it exits 1 with `Could not apply flowoverride` — the two
surfaces disagree about the same run, and the one a script is most likely to shell
out to is the one that lies. The `--json` assertion ledger is no better: it reports
`"status": "applied"` for the override the engine had just thrown out.

(The fixture above is `decompiler/crates/kuna-analysis/tests/fixtures/aif_gap_x86_64`,
in the repo. The same three symptoms reproduce byte-for-byte on two crackmes, where
the deleted body was 955 lines.)

## The fix

- A flow override the engine will not take is a **rejected assertion, not a dead
  function**. `Funcdata::overrideFlow` gives up before it rewrites any opcode, so
  the IR that follows is the one the same run without the directive would have
  produced. `FlowInfo::process` now records the refused `(instruction, type,
  reason)` and flow follows on; the recovered body comes back byte-identical to
  the no-directive control.
- The refusal is reported, not swallowed: the console prints
  `Rejected <command>: <reason>` under the `decompile`, the ledger row becomes
  `"status": "rejected"` with the engine's reason, and the row carries a new
  `"fatal": true`.
- **`fatal` exits non-zero without `--assert-strict`**, which the other rejections
  still need. A rename that did not bind leaves a correct body one annotation
  short and you can see which; a refused flow override leaves C that describes a
  different control-flow graph than the one you asked for and looks exactly like
  the C you wanted.
- Separately, the text surface now notices a `decompile` that *raised* at all. It
  only ever looked for the console's swallowed `Skipping <name>:` notice, so every
  other per-function abort exited 0 with the printer's generic shell. Those aborts
  are also stamped onto the retained `Funcdata`, so the shell names its reason
  instead of blaming structuring.

Deliberately not done: making an unappliable override a silent no-op. That
restores the body and keeps exit 0 — which is the same lie in a nicer shirt.

## The tests

`tests/cli/rejected-flow-override-exits.json` (promoted acceptance: exit non-zero,
body back, reason on stderr) and `tests/cli/flow-override-refusal-ledger.json`
(the `--json` row is `rejected`/`fatal`, and the function record has no error).
Both fail on the unpatched tree. `verify_assertflow`'s engine-refusal case is
rewritten to the new contract and a `fatal`-vs-not case added; four transcript
unit tests cover the two new scanners.

Gates: `make test` 675/675 PARITY OK, `make test-stages` 650/650 PARITY OK,
`make check-spec` OK, `make test-cli` 56/56, `kuna catalog --check` OK. A
`decompile-all` before/after sweep over 134 in-repo fixtures with
`noreturn_error`+`listing` on — the options that generate derived flow overrides —
differs on 0 of them.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

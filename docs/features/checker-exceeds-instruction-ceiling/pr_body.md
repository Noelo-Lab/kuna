## The problem

A function whose flow exceeds kuna's 100000-instruction budget decompiled to
nothing at all, and the error named no way out. Any in-repo fixture reproduces it
once the budget is small enough to reach:

```console
$ kuna decompile-all decompiler/crates/kuna-analysis/tests/fixtures/midstring_x86_64 \
      --functions main --option maxinstruction 5 --json
      "size": 48,
      "code": null,
      "error": "Flow exceeded maximum allowable instructions",
```

The budget has always had a second policy — truncate instead of throw — and it did
not truncate. It planted an artificial halt at the overrun address and then decoded
that instruction anyway, so the next address was queued and the walk carried on to
the end of the reachable body, now with one halt per instruction:

```console
$ kuna decompile decompiler/crates/kuna-analysis/tests/fixtures/fauxware main \
      --option maxinstruction 5 --option errortoomanyinstructions off
void main(char *a0,char *a1)
{
  ...
    *(unsigned long *)&v2[-0x50] = 0x40073e;
    puts("Username: ",v2[-0x50]);
    return;
    return;
```

The budget value does not change that outcome, which is the tell. On the 7 MB MBA
crackme this need came from (crackmes.one `69d6affb110488a3205426e2`), whose
`sub_140001000` is one genuine 1,804,480-instruction function, the truncating arm
died at a failed 11.5 GB allocation after 38 s at `maxinstruction` 2000 and at
5,000,000 alike.

## The fix

- `FlowInfo::process_instruction` reports no fall-through at the budget after
  planting the halt, so the walk stops there. Every address still queued is halted
  the same way when it is popped; the decode ends at the budget instead of at the
  end of the body.
- The halt is registered in `visited` as the instruction at that address, so a
  branch arriving there later resolves to it rather than raising "Could not find op
  at target address", and it starts a basic block — the lesson #410 learned for the
  `funcboundflow` truncation.
- With a truncation that truncates, the CLI's decompiling surfaces can afford it and
  take it (`errortoomanyinstructions off`, DIV-120). An inventory or query load never
  follows flow and does not; the engine, console and datatest defaults are unchanged,
  which is why neither parity corpus can move.
- The warning header names both knobs, because an error that names no remedy is what
  sent the reporter through `kuna catalog` looking for an option that was never
  there: `--option maxinstruction N` raises the budget, `--option
  errortoomanyinstructions on` makes the overrun fatal again.

The witness now decompiles: 10,164 lines of C, `error: null`, 37.3 s and 3.76 GB for
the 100000 instructions the budget allows.

## The tests

`tests/cli/checker-exceeds-instruction-ceiling.json` (the promoted acceptance,
restated on a 16 KB in-repo fixture because CI has no dataset) and a two-pass
`kuna-cli` integration test: `--option errortoomanyinstructions on` must still give
`code: null` and the old error. `decompile_e2e.rs` now asserts the bounded run emits
*less* C than the unbounded one — it did not before. `decompile-all` over all 156
analysis fixtures is byte-identical on both arms.

Gates: `make test` PARITY OK 675/675, `make test-stages` PARITY OK, `make rust-test`
green (5,615 passed / 0 failed), `make check-spec` OK, `make test-cli` 34/34, `kuna catalog --check` OK.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

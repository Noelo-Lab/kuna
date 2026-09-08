## The problem

A call is emitted for its side effect and the next line branches on a local the
function never assigns. On any x86-64 binary where a call's guard INDIRECTs get
an op inserted into them, the call silently loses its return value:

```
$ kuna decompile decompiler/crates/kuna-analysis/tests/fixtures/mcount_x86_64 __printf_fp_l
...
        v54 = hack_digit(&v41);
...
        hack_digit(&v41);          <- the result is gone
...
  v35 = hack_digit(&v41);
```

Three calls to the same function, one of them thrown away. On the reported
witness (a Windows PE, `sub_140003f80`) the thrown-away result is an `NTSTATUS`
and the next statement tests it:

```
    BCryptGetProperty(v48,v27,&v50,4,v49,v70 & 0xffffffff00000000);
    if ((v16 < 0) || (v28 = (unsigned long long)v50, !v50)) {
```

`v16` has no definition anywhere in the function. Every other `bcrypt` import in
that function recovers its result normally, including the one immediately before
it whose result is consumed by the identical `test eax,eax; js` pair.

## The fix

- `Funcdata::opInsertAfter` redirects an insertion after an INDIRECT marker to
  the CALL (or STORE) that marker speaks for. kuna carried that redirect as a
  `STUB(W3-varnode)` because the iop decode belonged to a later wave; this
  completes it (`p3_dataflow/kuna_indirectanchor.rs`).
- Without the redirect, `RulePullsubIndirect` — pulling a SUBPIECE through the
  guard INDIRECT of a frame slot whose upper half is read after the call —
  planted that SUBPIECE *inside* the call's guard run.
  `collectOutputTrialVarnodes` walks back from the CALL and stops at the first
  op that is not an INDIRECT, so it stopped 18 ops short of the `EAX` INDIRECT
  creation, every output trial was marked no-use, and the CALL got no output.
- Gated as `option indirectanchor on|off`, default on; `off` is the previous
  placement. It changes emitted C, so it ships behind a name even though it is
  a port completion rather than a judgement call.
- Not fixed here: the same report's second half, `BCryptDecrypt`'s literal-zero
  second argument. That is input-trial recovery for a register whose 4-byte
  sub-range is heritaged separately from the 8-byte value the caller wrote, and
  no existing option moves it. It needs its own change.

## The tests

`p3_dataflow/kuna_indirectanchor/tests.rs` builds the guard-run shape against a
real `Funcdata` and asserts both arms; the three behavioural cases fail with the
option off. `tests/cli/bcryptgetproperty-status-tested-through.json` pins the
`__printf_fp_l` witness above.

Gates: `make test` PARITY OK 675/675 (0 assertions moved), `make test-stages`
PARITY OK, `make rust-test` green, `make test-cli` 81/81, `make check-spec`
green, `kuna catalog --check` OK. Sweep over 35 crackmes (7,919 functions) and
every in-repo fixture (1,676 functions): 51 functions change, 0 added or
removed, every one of them a bare call that regains its return value; net −57
lines. `__printf_fp_l` decompiles 10.2% faster (median of 7).

🤖 Generated with [Claude Code](https://claude.com/claude-code)

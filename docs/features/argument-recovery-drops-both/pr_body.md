## The problem

A call inside a loop renders with an empty argument list while the two
instructions before it plainly load its argument registers. The same function
recovers arguments fine at a call that is not in that loop.

```
$ kuna decompile WeeperVM--Level_1 0x113d9 --addr --define-function 0x110ff=watchdog
    ...
    v8 += 1;
    if (watchdog()) {          <-- 0x1146a MOV ESI,0x2 / 0x1146f MOV EDI,0x13050
      ...                          are the two instructions before CALL 0x110ff
      printf((char *)sub_10e27(v7,v6,v3,v2));   <-- same shape, four arguments
```

The callee uses both: `kuna decompile <bin> 0x110ff --addr` recovers
`sub_110ff(long a0,int a1,...)` and reads `a0` as a timer array indexed by `a1`.

The trials do not fail realism, they fail exclusivity. `Funcdata::onlyOpUse`
walks the descendants of the value reaching a call and rejects when another CALL
also consumes it, and the x86 register-clearing idiom puts a fake such consumer
on every loop: the value passed at the bottom of the body reaches the next
iteration's `xor esi,esi` through the killed-by-call `INDIRECT`, the loop-head
`MULTIEQUAL` and the `SUBPIECE` that slices `esi` out of `rsi`, and from there
the call that follows the clear.

## The fix

- New option `zeroidiomuse` (`on|off`, default **on**, DIV-148). An `INT_XOR` or
  `INT_SUB` whose two operands are the same value is skipped in the descendant
  walk: `INT_XOR(v,v)` is `0` whatever `v` is, so nothing downstream of it can
  observe the value being scored.
- Sameness is structural rather than Varnode identity, because the two `esi`
  reads are still two distinct `SUBPIECE` Varnodes at this point in the
  schedule — the CSE that would merge them, and the constant fold that would
  then fire, run after `ActionActiveParam`. Identical Varnodes, equal constants,
  or the same pure reshaping op (`COPY`/`SUBPIECE`/`PIECE`/`INT_ZEXT`/`INT_SEXT`)
  over operands that are themselves the same, to depth 2.
- `INT_AND` and `INT_OR` are deliberately excluded: `v & v` is `v`, so the value
  really does survive them.
- One-directional — it can only decline a veto, never promote a trial by itself.

## The tests

`tests/stages/ghdec-zeroidiomuse.xml`, two-pass over a vendored reduction of the
witness loop: `option zeroidiomuse off` emits `watchdog()`, the default emits
`watchdog(timers,2)`. 11 unit cases for the predicate, and the acceptance probe
promoted to `tests/cli/argument-recovery-drops-both.json`.

Gates: `make test` PARITY OK 675/675 · `make test-stages` PARITY OK 725/725 ·
`make rust-test` green · `make check-spec` green (strict too) · `make test-cli`
106/106 · `kuna catalog --check` OK.

Whole-corpus sweep, `decompile-all` in both arms over 21 linked x86-64 ELF
binaries / 5,857 functions: **141 functions change, 148 argument positions
recovered against 6 given up, no function loses a statement.** The recoveries
are checkable from the output alone — `__fprintf_chk(f,1,"keycode %3d =")` gains
the value its format string demands, `OPENSSL_init_crypto(0xc)` gains its
settings pointer, and in tar's date parser ten `dcgettext(...); warn();` pairs
fuse back into `warn(dcgettext(...))`. The 6 given up are the upstream
competition rule working as written — a value two calls share can be exclusively
a parameter of only one — and in every case the value stays live at its other
use in the same function.

Speed: -15.72% on the witness (`timeit`, median of 7), inside the ±20%
single-target noise floor and not claimed as a speedup.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

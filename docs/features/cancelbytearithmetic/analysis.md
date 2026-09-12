# Cancelling byte arithmetic: analysis

## Symptom

`crackme_shroud.exe` constructs `"Usage: %s <password>\n"` in a declared
`char usage[22]`, but kuna splits the initializer around byte `usage[0xf]`:

```c
builtin_strncpy(usage,"Usage: %s <pass",0xf);
usage[0xf] = (char)(x & y) * '\xfe' + 'w' + (char)((x & y) << 1);
builtin_strncpy(&usage[0x10],"ord>\n",5);
```

The byte is exactly `'w'`: `0xfe*x + (x << 1)` is `-2*x + 2*x`, and the
one-byte store performs the equality modulo 256. The default result should be
one complete initializer, not a data-dependent middle byte.

## Owning decision

The owning seam is P3 simplification-quiescence, specifically the INT_LEFT arm
of upstream `RuleSubCommute`. Upstream refuses the witness because the shift
input is produced by INT_AND rather than INT_ZEXT or PIECE.

The repaired divergence is option-gated as `cancelbytearithmetic`. Instead of
broadly commuting a low piece through a shift, it recognizes the complete
one-byte tree
`preserved + low8(x)*(-2^s) + low8(x << s)` for `1 <= s < 8` and replaces the
outer add with `COPY(preserved)`. Both SUBPIECEs must use the same `x`, offset
zero and width one; the coefficient must equal `-2^s mod 256`; and every
intermediate result must have exactly the matched consumer. The witness source
must be INT_AND of two nonconstant values. Constant masks, direct CALL/LOAD
producers and all mismatched or shared trees decline.

## Bounds established by measurement

The prior broad commute changed 248 corpus bodies and produced signed-left-shift
undefined behavior plus readability and stale-value regressions. Exhaustive raw
p-code classification found no complete cancellation tree outside the filed
witness. The exact matcher therefore changes only the five witness stores and
never creates a narrowed left shift. The exact 9,024-byte witness changes from
the three writes above to:

```c
builtin_strncpy(usage,"Usage: %s <password>\n",0x15);
```

The stage and promoted CLI acceptance require the whole 21-byte copy. This is
stronger than accepting a standalone `usage[0xf] = 'w'`: deleting the store or
the suffix cannot pass.

The rebuilt immutable CLI found all five exact graphs in the target function
(shifts 1, 2, 3, 3 and 4) and passed both the default-on acceptance and the
three-write option-off control. A fresh sweep of 795 O0/O2/O2-noinline slices
completed all 1,590 arms and compared 90,097 bodies per arm: there were zero
non-OK arms, missing/no-code markers, inventory asymmetries or body changes.
That zero-diff result is expected because none of the corpus bodies contains the
complete exact tree; the target fixture is the independently proved positive.
Fifteen timed samples measured 1,233.3 ms off and 1,162.0 ms on (-5.78%), within
the 5% regression budget.

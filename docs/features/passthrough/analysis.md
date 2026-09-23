# passthrough — analysis

## The symptom

gzip -O2 `gzip_base_name` is `endbr64; jmp last_component`. It hands its own
`rdi` to the callee and returns what the callee returns. In the same
whole-binary run kuna recovers the callee as `char *last_component(char *a0)`,
yet renders the wrapper as

```
void sub_d290(void) { sub_dfd0(); } // tail-call
```

against a DWARF prototype of `char *gzip_base_name(char const *fname)`. coreutils
df `dir_name` (`call mdir_name; test %rax,%rax; ...`) and tar's `*_diag` helpers
are the same shape without the tail call, and the O2 census
(`docs/decbench/typecampaign/final-e/o2census/`) counts 24 O2 / 51 O2-noinline
functions one argument short of their twin, 25 of them a register forwarded to a
call.

## Why the argument is lost

A call site's argument trials come from `Heritage::guardCalls` (`heritage.cc:1444`):
one per HERITAGED storage range the callee's model could take a parameter in. A
range is heritaged only when some op of the function reads or writes it. A
register the function only forwards is named by no op at all, so the call gets no
trial for it, no argument, and the function's own input never has a reader and
gets no parameter. Where the register IS heritaged (the function also tests it),
`AncestorRealistic::execute` (`funcdata_varnode.cc:2159`) refuses an input
Varnode at the call slot outright — upstream expects "active movement into the
parameter", and an untouched input is the absence of one.

The existing arity family cannot reach it: `AncestorRealistic`,
`ActionActiveParam` and the `calleearity*` passes all decide a call's arguments
from the CALLER's data flow or from a SIBLING call to the same callee. A pure
pass-through has neither active movement nor a sibling.

## The evidence

Only the callee can say it wanted the register. `passthrough` reads the same
statement `argclobber` reads in the other direction: the callee's own RECOVERED
prototype, which `protoorder` states for every callee `decompile-all` decompiles
before its callers. A register `R` at a direct, unlocked, non-variadic call is an
argument when:

1. the stated prototype has a parameter starting at `R`, and the list is an
   **arity claim** (`RecoveredTypes::arity_sound`, computed only when the option
   is on): every parameter in a register, the callee's body does not read the
   register its next parameter would arrive in (`proves_input`, the
   register-zeroing idiom excluded — a variadic's register-save prologue reads
   every argument register there is), the list does not end on the last argument
   register, and the model puts those types where recovery found them;
2. the callee's body READS `R` before writing it on some path, for a value that
   reaches something (`calleedeadarg`'s `proves_input`);
3. the caller did not set the call up as variadic (`xor %eax,%eax` before a SysV
   call: `set_up_as_variadic`) and is not variadic itself (`test %al,%al` in its
   entry block: `reads_the_vararg_count`);
4. the value at the call is the function's own incoming `R`.

The argument is as wide as the callee's body reads it (the narrowest of the
range, the stated parameter and the widest low-justified read): a parameter the
callee recovered as `rdi` but reads as `edi` becomes the caller's `int`.

## The mechanism

The rule only adds to a list every other rule has settled. The first cut scored
the forwarded trial active while trials were scored, and review found two ways
that took arguments away:

- tar `sysinttostr` (`mov %rcx,%rsi; cmp %rdx,%rdi; ja; jmp umaxtostr; jmp
  imaxtostr`): `onlyOpUse` rejects the first call's `rsi` because the same value
  feeds the second call, the list comes out empty, and `calleearityfwd` fills
  it from the sibling. An active `rdi` made the list non-empty and switched that
  rescue off, so `umaxtostr(a0,a3)` became `umaxtostr(a0)`. The gnulib `xpalloc`
  copies lost `xrealloc`'s size the same way.
- gcc -O2 `noop(); glob = twoarg(p,3);`: IPA-RA leaves `rdi` in place across
  `noop`, the claimed read at `twoarg` reached it through `noop`'s INDIRECT,
  `AncestorRealistic` called it killed by call, the trial ended
  definitely-not-used and `forceNoUse` dropped the `3` behind it.

So the heritaged register's scoring is upstream's, and the pass acts outside it:
`capture` (first thing in `build_input_from_trials`) keeps the unused trials that
stand on the function's own input Varnode and satisfy the evidence, and
`extend_pending` (last thing in `ActionActiveParam::apply`, after the
`calleearityfwd`/`calleearitylive`/`calleearitybody` retries) extends a final list
that is a leading run of the stated one with the stated registers that follow,
up to the first the function does not forward.

For a register no Varnode of the function touches, `claim_untouched_registers`
(end of `ActionFuncLink`, before the first heritage) registers the trial and a
CALL read at each call no other call can precede (`first_calls`), where heritage
can only link the read to the function's input. Heritage then registers the
range at every other call as it would any heritaged range; only the function's
own RETURN is kept off it (`suppresses_return_trial`). Every trial on a claimed
range is scored inactive (`claimed_range`), where the option-off run's
hole-filling trial starts, and one that ends unused is retired before the rescues
read the trials. An earlier cut suppressed the range at every other call too,
which cut a Cortex-M `double` returned in r0:r1 to r0 (crazyflie `0x8039290`) and
made a later hole fill add a read to a range dead-code removal had visited, a
restart with a different result (betaflight `0x802201c`).

## The tail call's result

When every live RETURN is reached from a direct call with nothing between (at
most four single-predecessor blocks back), every such callee states a non-void
return in the same register, the function's own output is not locked and no op
touches that register (`stated_tail_return`), each RETURN gets a read of it and
the function's return trial is registered (`claim_tail_return`). Upstream's
`ancestorOpUse` refuses an INDIRECT creation at a RETURN ("an indication of an
output trial"), so `ActionReturnRecovery` accepts the creation planted at a
claimed call (`returns_tail_result`); that call keeps its return-value trial, and
its output takes the callee's recovered return type (`tail_return_type`).
`gzip_base_name` becomes `char * sub_d290(char *a0) { return sub_dfd0(a0); }`.

## What it cannot know, and why it ships on

The evidence is a recovery, not a fact.

- **A callee whose own return is wrong.** `version_etc_arn` is DWARF `void`, but
  kuna over-recovers it as returning `long` (its body ends in a `__fprintf_chk`
  whose result stays live at the RETURN). The wrapper `version_etc_ar` then
  inherits that wrong `long` return through the tail-return half: 23 of the 304
  gained returns with a DWARF twin across 26 binaries, all downstream of a
  callee already wrong in the base output.
- **A callee's type.** The forwarded parameter now also reaches the callee, so
  the callee's recovered type votes on it: where that type is an integer the
  caller's `char *` can lose (the mgetgroups copy recovered as `long a0` turns
  `print_group_list`'s `username` into `long`; 6 of the 444 typesweep slices).
- **A callee that over-recovers its list.** The first cut claimed at every call
  and gave ARM firmware ~55 parameters DWARF does not list; a register that
  reaches a call only after another call is not claimed any more, and the
  argument half now has none that DWARF contradicts (454 confirmed, 52 without
  a twin).

`type_match` scores `variables[]` (args, stack symbols and framelayout slots),
never a return type, so the returns were the reason the option first shipped
**off**. With the gate and the whole-corpus check (`dwarf-confirmation.md`:
2,773 of 2,900 gained parameters confirmed, 4 contradicted, nothing lost) it
ships **on** (`default-on-evaluation.md`).

Nothing is added where a callee stated nothing: `kuna decompile`, a narrowed or
sharded `decompile-all`, an import, `--option protoorder off`, and under
`--option protoorder types` a recursive component, all leave every call as it
was (under the default `cycles` a recursive callee states its list like any
other). `tests/stages/kuna-passthrough.xml`
is the negative control for that clause; the positive witness is
`decompiler/crates/kuna-analysis/tests/fixtures/passthrough_x86_64` under
`tests/cli/passthrough-gives-a-forwarding-function-its-parameter.json`, with a
clobbered forward and a variadic callee as its controls, plus three shapes that
must keep every option-off argument (`noop(); twoarg(p,3)`, `vout(p);
twoarg(p,3)`, tar `sysinttostr`).

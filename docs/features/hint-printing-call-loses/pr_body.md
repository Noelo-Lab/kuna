## The problem

A caller that spends an argument register on a throwaway loses **every**
argument at the call that follows, even the one plainly live in `rdi`. Build the
fixture this PR adds and see it (an ELF crackme's XOR string decoder, reduced to
a single XOR):

```
$ as -o f.o decompiler/crates/kuna-analysis/tests/fixtures/calleearityscratch_x86_64.s
$ ld -o f f.o
$ kuna decompile ./f scratch_byte
int scratch_byte(unsigned char *a0)
{
  a0[4] = a0[4] ^ *a0;
  return cal2() + (int)a0;     <-- a0 is live into cal2, which reads rdi
}
```

`cal2` reads `rdi` at its first instruction, and asserting a prototype for it
restores `cal2(a0)` — so the caller-side dataflow is intact and the argument is
lost entirely in unlocked call-site recovery.

## The fix

- `calleearitybody` picks the run `{rdi}` from the callee's own body; the
  callee's decode is cut at its first nested call, so nothing proves `rsi` dead
  and `calleearitycut` is asked to bound the run by the register it stops at.
  That rule requires the boundary to be **quiet** — the caller put nothing in it
  — and here the caller did: it spends `rsi` on the XOR key. The one-argument
  list is dropped.
- New option `calleearityscratch` (P4, core, default on) separates "the caller
  wrote it" from "the caller is passing it". Argument setup is a value used for
  nothing but the call, which is `Funcdata::onlyOpUse` — and
  `check_input_trial_use` has already asked it, marking a trial **active** only
  when the answer is yes. An **inactive** boundary is scratch and bounds the run.
- An **active** boundary stays refused: that is upstream saying the value goes
  nowhere but the call, which is the further argument `calleearitycut` exists not
  to hide. So does a **constant**, however the trial scored.
- `calleearitycut`'s other two conditions are untouched, so the run is still
  contiguous and still stops short: the reachable error stays a *missing
  trailing* argument, never a misplaced one — against a site that rendered no
  arguments at all.

## The tests

`tests/stages/kuna-calleearityscratch.xml` is two-pass over four caller/callee
pairs: the two scratch shapes (64-bit and the witness's own byte XOR) render an
empty list with the option off and recover the argument with it on; an active
boundary and a constant boundary are declined in both passes. Ten unit tests
cover the boundary truth table and that `calleearitycut`'s own guards still
answer first. `tests/cli/hint-printing-call-loses.json` is the promoted probe.

Sweep over 90 binaries in both arms (60 decbench O0/O2/O2-noinline, 30
crackmes.one images), 27,417 functions: **38 functions changed in 9 binaries, 0
added, 0 removed, and 0 calls lost an argument** — a per-binary call-arity
histogram shows no zero-argument form increasing anywhere. Recoveries checkable
against source or the line beside them include `__subvdi3(v5,src)` in OpenBSD
`strlcpy` (which returns `s - src - 1`) and `sub_1ad20(v17,v10,a0,a1)` matching
the sibling branch's `sub_1a2f0(v17,v10,a0,a1)`. Speed: `decompile-all` on a
31-second binary, interleaved, −0.03%.

Gates: `make test` PARITY OK 675/675, `make test-stages` PARITY OK (687 -> 692
keys, purely additive), `make rust-test` green, `make test-cli` 85/85,
`make check-spec` OK, `kuna catalog --check` OK, acceptance probe PASS.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

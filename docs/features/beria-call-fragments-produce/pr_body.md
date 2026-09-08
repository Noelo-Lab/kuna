A `call` whose callee throws the pushed return address away and jumps back into
the caller never returns to the return address — but `CPUI_CALL` is a fall-through
op, so kuna decodes the byte sitting there anyway. On a protected binary that byte
is junk, and it lifts into the emitted C as a store to a global that does not
exist, with everything after it one byte out of phase with the real instruction
stream.

Build the fixture this PR vendors (one `gcc` line, no toolchain beyond `-m32`) and
decompile it on `main`:

```
$ gcc -m32 -nostdlib -static -Wl,-e,_start -o /tmp/tramp \
      decompiler/crates/kuna-analysis/tests/fixtures/calltrampoline_i386.s
$ kuna decompile /tmp/tramp protected
void protected(void)
{
  dat_5d5cc083 = tramp();
}
```

`protected` is `mov 0x8(%ebp),%eax; call tramp; <junk byte>; add $0x5c,%eax; ret`,
and `tramp` is `lea 0x4(%esp),%esp; add $0x12,%eax; jmp <past the junk byte>`.
`dat_5d5cc083` is the junk byte plus the four bytes of the two real instructions
it swallowed; the body is `a0 + 0x6e`.

## The fix

- New `option calltrampoline` (P2 flow-classification, default on, DIV-144).
  `kuna_calltrampoline.rs` decodes a direct call's target out of band and fires
  when the stack pointer passes through exactly `entrySP + <pointer size>` and the
  run then ends in a direct unconditional branch, with no call, conditional
  branch, indirect branch or return in between. The `CALL` becomes a `BRANCH` and
  the fragment is flowed through.
- The stack pointer is tracked symbolically as `entrySP + <constant>`, not by net
  delta: `lea esp,[esp+4]; sub esp,0xcfc; jmp` nets a large allocation, and what
  identifies the shape is the moment the return address stops existing.
- The rewrite is applied in the `CPUI_CALL` arm, not by falling into the
  `CPUI_BRANCH` arm. That arm consults `tail_call_kind`, and the fragment is a
  known function entry precisely *because* it is a call target — so `tailcalljump`
  claims it and ends the caller at `v3 = tramp(); return v3;`, which removes the
  junk store by deleting the whole body after the call.
- A tail-call thunk (`add esp,4; jmp printf`) has the same p-code shape, so
  `ArchFlowEnv::is_return_discarding_trampoline` also requires the jump target to
  be an address `query_call` does not know as a function entry. Those stay a
  `CALL`.

## Tests

`tests/stages/kuna-calltrampoline.xml` is two-pass: `option calltrampoline off`
reproduces the junk store, the default recovers `return a0 + 5`. Sixteen unit
tests cover the recognizer's accept and decline cases (`add esp,8`, a `leave`-style
restore, a partial write to the stack pointer, an inner call, a relative branch).
`tests/cli/beria-call-fragments-produce.json` is the promoted acceptance probe over
the vendored `calltrampoline_i386`.

Gates on the rebased tree: `make test` PARITY OK 675/675, `make test-stages` PARITY
OK 714/714 (2 new assertions, none moved), `make test-cli` 102/102, `make
check-spec` green (strict too), `kuna catalog --check` OK, workspace suite green.
Sweep: `decompile-all` in both arms over 73 PE images plus 155 ELF/mixed binaries of the arena and fixture corpora, **37,465 functions -- 87 changed (0.23%), 0 added, 0 removed**, and all 87 are in the one protected image the need was filed against (every other binary, PE and ELF alike, is byte-identical). All 87 hunks are the same repair: 23 removed lines are a store to a junk global (`dat_<hex> =`), the rest replace arithmetic on undefined registers with the real recovered body -- on `sub_40c580`, `(**(void **)(v2 + 3))(*(unsigned int *)(v2 + 0xf),v5)` plus six lines of garbage becomes the `*(v3 + 0x8c)` resolver-dispatch guard. Speed −2.4% on the
witness (median of 7, 155.98 ms vs 159.81 ms).

🤖 Generated with [Claude Code](https://claude.com/claude-code)

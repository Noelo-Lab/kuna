# tee-o2-tail-jumps: O2 tail-jump to a PLT thunk

angr testcase: `test_decompiling_tee_O2_tail_jumps` — function `setlocale_null_androidfix`
binary `binaries/tests/x86_64/decompiler/tee_O2` (x86_64 PIE), entry `0x4060e0`.

## The construct

At `-O2`, a function whose last action is a call+return is compiled to a **tail jump**:

```asm
00000000000060e0 <setlocale_null_androidfix>:
    60e0:   xor    %esi,%esi          ; arg2 = NULL
    60e2:   jmp    18d0 <setlocale@plt> ; tail call (direct BRANCH) into the PLT thunk
```

The `jmp setlocale@plt` is a direct (relative `e9`) branch to the PLT stub for the
external `setlocale`. There is no `call`/`ret` — the callee returns straight to *our*
caller, so the function's return value IS `setlocale`'s return value.

## angr vs kuna

angr recognizes the tail jump and renders it as a returned call:

```c
char * setlocale_null_androidfix(void)
{
    int v1;  // edi
    return setlocale(v1, NULL);
}
```

kuna (default) inlines the PLT thunk and mis-renders it:

```c
void setlocale_null_androidfix(unsigned long a0)
{
                    /* WARNING: Treating indirect jump as call */
  (*dat_209f68)(a0,0);
  return;
}
```

Three symptoms: (1) a spurious "Treating indirect jump as call" recovery-failure
marker; (2) the callee is an unresolved GOT pointer `(*dat_209f68)` instead of the
named `setlocale`; (3) `void` + `return;` instead of `return setlocale(...)`.

## Root cause (owning stage: S2 flow classification)

`FlowInfo::xref_control_flow` (`decompiler/crates/kuna-decomp/src/s2_lift/flow.rs`,
the `CPUI_BRANCH` arm). kuna's flow follower has `baddr=0, eaddr=~0` (the whole
address space is in-bounds), so the direct `jmp setlocale@plt` is treated as ordinary
**intraprocedural** flow: the `else { self.new_address(curop, &destaddr) }` branch
follows the jump INTO the PLT thunk at `0x18d0`. The thunk's body is
`jmp qword ptr [rip+X]` (an indirect jump through the GOT slot at `0x209f68`); jump-table
recovery fails on it, so `truncate_indirect_jump(FailNormal)` rewrites that BRANCHIND to
a `CALLIND` through the GOT pointer `dat_209f68` and emits the
`"Treating indirect jump as call"` warning. The PLT thunk got *inlined* instead of the
`jmp` being recognized as a **tail call** to the (named) thunk.

kuna already resolves a *normal* `call setlocale@plt` to `setlocale(...)` (see
`__argmatch_die`, which renders `exit(1)`, `dcgettext(...)`, `__fprintf_chk(...)`),
because `FlowEnvironment::query_call(plt_addr)` returns the external name. The tail-jump
path simply never consults that — it flows in.

## Hypothesis / fix

Recognize the tail jump at flow-classification time. In the `CPUI_BRANCH` arm, when a
direct branch's target is the entry of a *known function* (`query_call(dest).is_some()`,
which includes PLT thunks) and is not the current function's own entry, rewrite the
`BRANCH` into a `CPUI_CALL` + an artificial `RETURN`, instead of flowing into the callee.
The existing call machinery then resolves the thunk name (`setlocale`) and the dataflow
makes the callee's return value flow out as the function's return value.

This is the same hook class as the existing `kuna_v850indbranch` S2 feature (a
`FlowEnvironment` predicate consulted inside `xref_control_flow` + an Architecture bool
flag), and the `BRANCH→CALL + artificial_halt` rewrite has in-file precedent in
`truncate_indirect_jump` / `setup_callind_specs` / `check_for_flow_modification`.
Gated behind a new, default-OFF option so default output stays byte-identical.

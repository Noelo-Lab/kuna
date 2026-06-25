## tee-o2-tail-jumps: recover -O2 tail jumps as tail calls (`option tailcalljump`)

Closes the gap where angr's decompiler beats kuna on
`test_decompiling_tee_O2_tail_jumps :: setlocale_null_androidfix`
(`binaries/tests/x86_64/decompiler/tee_O2`, x86-64 PIE, `-O2`).

See [`docs/features/tee-o2-tail-jumps-4a1f49/analysis.md`](docs/features/tee-o2-tail-jumps-4a1f49/analysis.md)
for the full analysis.

### The gap

At `-O2`, a leaf function whose last act is "call X; return" is compiled to a direct
**tail jump** - `jmp X` instead of `call X; ret`. When X is an external symbol the jump
targets the PLT thunk:

```asm
setlocale_null_androidfix:
    xor    %esi,%esi          ; arg2 = NULL
    jmp    setlocale@plt       ; tail jump (direct BRANCH) to the thunk
```

kuna's flow follower runs with the whole address space in-bounds, so it treats the direct
`jmp setlocale@plt` as ordinary intraprocedural flow and follows it **into** the PLT thunk.
The thunk's body is `jmp qword [rip+GOT]` (an indirect jump); jump-table recovery fails on
it, so it is rewritten to a `CALLIND` through the GOT pointer plus a
`"Treating indirect jump as call"` warning. The thunk got *inlined* instead of the `jmp`
being recognized as a tail call, so the callee renders as `(*dat_209f68)(...)` rather than
the named `setlocale`.

### The mechanism

A new S2 flow-classification predicate `kuna_is_tail_call_branch`
(`decompiler/crates/kuna-decomp/src/s2_lift/kuna_tailcalljump.rs`, ElementId 4100), modeled
exactly on the existing `kuna_v850indbranch` feature (a `FlowEnvironment` predicate consulted
inside `xref_control_flow`, gated by an Architecture bool flag). In the `CPUI_BRANCH` arm of
`FlowInfo::xref_control_flow`, when the option is on and a direct branch's target is the entry
of another known function (`query_call(dest).is_some()`, which includes PLT thunks) and not the
function's own entry, the `BRANCH` is rewritten to a `CPUI_CALL` + an artificial `RETURN` (the
in-file `truncate_indirect_jump` / `setup_callind_specs` halt-insert idiom), instead of flowing
into the callee. The callee then resolves by name and the spurious warning / GOT-pointer
indirect call are gone.

(The remaining `void` / `return;` vs angr's `return setlocale(...)` is an S4
return-value-recovery concern - the option-OFF path is *also* `void`, so it is inherent to
kuna's wrapper-return recovery, not introduced or fixable by this S2 change.)

### The option

`option tailcalljump on|off` (default **off**, default-pipeline byte-identical). Flip per
decompilation:

```
kuna decompile ./tee_O2 setlocale_null_androidfix --option tailcalljump on
```

It is LLM-discoverable via `kuna catalog --json` (provenance: `source_decompiler=angr`,
`change_kind=structure-recovery`).

### Ablation / parity / speed

- **Default-OFF (shipped):** byte-identical - `make test` **PARITY OK** (675/675), `make test-stages` **PARITY OK** (162/162).
- **Default-ON ablation:** regresses **2** upstream datatests (`Long double #1/#2`), so the
  feature ships **default-OFF opt-in** (no DIV entry).
- **Speed** (`setlocale_null_androidfix`, median of 5): off 259.0 ms / on 264.9 ms (**+2.26%**,
  within the 5% budget).
- New firing stage test `tests/stages/ghangr-tee-o2-tail-jumps-4a1f49.xml` (pass 1 off proves
  the bug; pass 2 on proves the fix).

🤖 Generated with [Claude Code](https://claude.com/claude-code)

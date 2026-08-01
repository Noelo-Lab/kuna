---
case_id: O0-mydoom-mydoom-msg_b64enc
pool: ida
status: needs-proposal
tier: M
margin: 43
fresh_verdict: still broken on today's build in BOTH modes and on BOTH binaries - the entire main for(;;) read/encode loop is deleted (3 basic blocks survive of a 22-node source CFG); on the non-stripped binary today's build is strictly worse than the 2026-07-27 stored run (it now also folds away the flush tail)
option_closing: null
feature_slug: stackargs
scope: proposal
confidence: high
---

## Side-by-side

IDA (stored, GED 0) - the loop is there, split across a `LABEL_7` back-edge:

```c
  while ( v20 < NumberOfBytesRead )
  {
LABEL_7:
    v3 = v20++;
    v4 = v21++;
    v13[v4] = Buffer[v3];
    if ( v21 == 3 ) { sub_4052B4(v13, v12); for ( i = 0; i <= 3; ++i ) { ... } memset(v13,0,3); v21 = 0; }
  }
  ReadFile(hFile, Buffer, 0x400u, &NumberOfBytesRead, 0);
  if ( NumberOfBytesRead ) { v20 = 0; goto LABEL_7; }
  if ( v21 ) { ... }
```

Ghidra (stored, GED 5) - the faithful shape, one `do { } while(true)` with the break:

```c
  do {
    if (local_434 <= local_14) {
      ReadFile(local_28,local_42e + 6,0x400,&local_434,(LPOVERLAPPED)0x0);
      if (local_434 == 0) {
        if (local_10 != 0) { FUN_004052b4(local_42e + 3,local_42e); ... }
        *(undefined1 *)(local_18 + param_1) = 0;
        CloseHandle(local_28);
        return 0;
      }
      local_14 = 0;
    }
    ...
    if (local_10 == 3) { FUN_004052b4(local_42e + 3,local_42e); for (...) {...} memset(...); local_10 = 0; }
  } while( true );
```

kuna stored (2026-07-27 benchmark, GED 43) - main loop already gone, flush tail still present:

```c
  v4 = 0x4c;
  v3 = (*dat_4112a0)();
  if (v3 == -1) { return 1; }
  v9 = 0; v8 = 0; v7 = 0; v5 = 0;
  (*dat_411350)();
  if (v9 != 0) {
    sub_4052b4();
    if (v9 <= 2) { v2[3] = 0x3d; }
    if (v9 <= 1) { v2[2] = 0x3d; }
    v6 = 0;
    while (v6 <= 3) { v1 = v7 + 1; *(void *)(v7 + a0) = v2[v6]; v6 = v6 + 1; v7 = v1; }
  }
  *(void *)(v7 + a0) = 0;
  (*dat_411298)();
  return 0;
```

kuna fresh, today, stripped binary, `--addr 0x4053c4`, no flags - byte-for-byte the same
structure as the stored block (only DIV-34..39 print normalization differs). Main loop still gone.

kuna fresh, today, non-stripped binary, `msg_b64enc`, identical under no-flag (`auto`->`aggressive`)
AND `--mode reliable` - a further collapse; the flush tail is now folded away too:

```c
bool msg_b64enc(char *a0)
{
  char *outbuf; // stack + 0x4
  int4 v1; // eax

  v1 = (*dat_4112a0)();
  if (v1 != -1) {
    (*dat_411350)();
    *outbuf = '\0';
    (*dat_411298)();
  }
  return v1 == -1;
}
```

`print raw` confirms the p-code: 3 basic blocks, no stack INDIRECTs across either CALLIND,
and both callinds carry zero argument varnodes (`callindr0x004112a0(i)`).

## Source

`~/github/decbench/results/full_run/O0/mydoom/compiled/msg.c:277`:

```c
static int msg_b64enc(char *outbuf, struct msgstate_t *state)
{
	HANDLE hIn;
	BYTE inbuf[1024], t[3], q[3];
	DWORD tp, inp, inlen, outp, i, linepos;
	const DWORD linelen = 76;

	hIn = CreateFile(state->attach_file, GENERIC_READ, ...);
	if (hIn == INVALID_HANDLE_VALUE) return 1;

	for (tp=0, inp=0, inlen=0, outp=0, linepos=0;;) {
		if (inp >= inlen) {
			ReadFile(hIn, inbuf, sizeof(inbuf), &inlen, NULL);
			if (inlen == 0) break;
			inp = 0;
		}
		t[tp++] = inbuf[inp++];
		if (tp == 3) { base64_t2q(t, q); for (i=0; i<4; i++) { outbuf[outp++] = q[i]; if (++linepos >= linelen) {...} } memset(t,0,3); tp = 0; }
	}
	if (tp) { base64_t2q(t, q); if (tp<3) q[3]='='; if (tp<2) q[2]='='; for (i=0;i<4;i++) outbuf[outp++]=q[i]; }
	outbuf[outp] = 0;
	CloseHandle(hIn);
	return 0;
}
```

`inlen` is the loop's only exit condition and it is written **by the callee**, through
`&inlen`, which is passed as ReadFile's 4th argument. The relevant asm:

```
0040543a: MOV  dword ptr [EBP + 0xfffffbd0],0x0   ; inlen = 0
...
00405465: LEA  EAX,[EBP + 0xfffffbd0]             ; &inlen
0040546b: MOV  dword ptr [ESP + 0xc],EAX          ; -> outgoing arg slot 4
0040548c: CALL EAX                                ; ReadFile (via IAT 0x411350)
00405491: MOV  EAX,dword ptr [EBP + 0xfffffbd0]   ; reload inlen
00405497: TEST EAX,EAX
00405499: JZ   0x405577                           ; the break
```

## Analysis

**Structural symptom (one):** the whole `for(;;)` read/encode loop is deleted. kuna emits a
straight-line body (3 basic blocks) where the source has 22 nodes / 30 edges and every other
decompiler emits 2-3 loops. The variable `inlen` does not appear in kuna's output at all, and
the `TEST EAX,EAX / JZ` at 0x405491 has become an unconditional branch to the epilogue - i.e.
kuna *proved* `inlen == 0` after the call, so the loop body is unreachable and is removed.

**Root cause: kuna does not materialize stack-passed call arguments, so a pointer-to-local
handed to a callee on the stack never escapes.** The store `[ESP+0xc] = &inlen` is never
attached to the CALLIND as an argument, so it has no reader, so it is dead; the LEA dies with
it; nothing ever observes that `&inlen` left the frame. The stack slot is then treated as
call-invariant, the pre-call constant `0` propagates straight across `ReadFile`, `if (inlen == 0)
break` folds to always-taken, and P3 dead-code removal takes the loop with it.

This is not mydoom-specific and not i386-specific. Minimal x86-64 reproducer (ELF, gcc -O0),
where only the 7th argument is stack-passed:

```c
extern void fill7(int a,int b,int c,int d,int e,int f,int *out);
int work2(char *buf, int n) {
    int len = 0, i = 0, total = 0;
    for (;;) {
        if (i >= len) { fill7(1,2,3,4,5,6,&len); if (len == 0) break; i = 0; }
        buf[total++] = buf[i++];
        if (total > n) break;
    }
    return total;
}
```

kuna today:

```c
  do {
    if (0 <= i) {
      fill7(1,2,3,4,5,6);     /* the 7th (stack) argument &len is DROPPED */
      return total;            /* len folded to 0 -> the break is unconditional */
    }
    ...
  } while (v2 <= n);
```

The six register arguments are recovered; the one stack argument is silently dropped and the
loop collapses exactly as in msg_b64enc. Compiling the callee **with a full DWARF prototype**
(`void fill7(int,int,int,int,int,int,int*)`, `gcc -O0 -g`) does not change this - the stack
argument is still dropped - so this is not only an unlocked-trial-recovery gap; the locked
path drops it too. The same pattern with `fill(&len)` (a *register* argument) is decompiled
perfectly, loop and all, which is the control.

On i386, where *every* argument is stack-passed, the effect is total: in
`kuna decompile-project mydoom.exe` 259 of 276 rendered calls have an empty argument list, and
on a second, independent i386 PE (`minipig.exe`) it is 90 of 92. Ghidra recovers them even for
unprototyped local callees (`FUN_004052b4(local_42e + 3,local_42e)`), which is why it keeps
`local_434` alive across `ReadFile` and keeps the loop.

**Owning phase: P4 (calls).** The decision is call-site argument materialization / stack-space
parameter trials:
- `decompiler/crates/kuna-decomp/src/p4_calls/coreaction_protos.rs::func_link_input` - the
  locked-prototype `IPTR_SPACEBASE` branch builds the argument with `data.op_stack_load(...)`
  and silently `continue`s on `Err(_)`; the placeholder tail only runs when
  `fc.proto().get_spacebase()` is `Some`.
- `decompiler/crates/kuna-decomp/src/p3_dataflow/heritage.rs::guard_calls` (lines 1439-1526) -
  the input-trial branch is gated on `tryregister`, which is forced `false` whenever
  `fc.get_spacebase_offset() == OFFSET_UNKNOWN`, and `stackoffset` is only ever set by
  `FuncCallSpecs::resolve_spacebase_relative` (`p4_calls/fspec.rs:6903`), reached only through
  the `RuleLoadVarnode` spacebase-placeholder tail (`p3_dataflow/ruleaction_4.rs:400-421`).

The visible damage lands in P3 (constant propagation + dead-code) and P8 (the loop is gone),
but neither made a wrong decision: they were told the callee cannot touch the frame.

**Why the reference wins:** IDA and Ghidra both attach the stack arguments (Ghidra additionally
applies the PE IAT prototype for `ReadFile`, so the 4th parameter is a typed `LPDWORD`), so
`&inlen` escapes, the callee is assumed to write it, and the loop guard survives.

**Not a metric artifact.** Source CFG is 22 nodes / 30 edges (real control flow, not a Joern
parse failure), GED is exact (not approximated), and the reference panes are complete functions,
not truncations. kuna genuinely deleted 19 of 22 blocks.

**No existing option closes it.** Swept `stackalias on`, `condfold wide`, `operand_refs on`,
`listing on`, `--mode reliable`, `--mode aggressive`: all produce the identical collapsed body.
`libproto` (P1, default-on) seeds *libc* signatures onto imports and does not reach a Win32 IAT
import, and in any case the DWARF-prototype control above shows a correct prototype is not
sufficient.

**Secondary observation (not this case's symptom, but the same family).** On the non-stripped
binary the DWARF pass creates stack symbols at the incoming-parameter offsets and they are not
bound to the recovered formals, so parameters are read as uninitialized locals - visible
binary-wide (`void rot13(char *a0,char *a1)` whose body uses `char *buf; // stack + 0x4` and
`char *in; // stack + 0x8` and never touches `a0`/`a1`; `bool msg_b64enc(char *a0)` with an
unbound `char *outbuf; // stack + 0x4`). This is why today's non-stripped output is *worse*
than the 2026-07-27 stored block: with `outp`'s def chain unbound, the flush tail folds away
too. Worth its own case.

## Proposed fix

Not a small fix - this is a missing capability, and it needs a `[PROPOSAL]` PR.

Mechanism, in dependency order:

1. **Make the stack-pointer offset resolvable at every call site.** `stackoffset` must stop
   being `OFFSET_UNKNOWN`, otherwise `guard_calls` never even tries. That means the
   spacebase-placeholder round trip (`func_link_input` -> `create_placeholder` ->
   `RuleLoadVarnode` -> `resolve_spacebase_relative`) has to complete; today
   `resolve_spacebase_relative` ends in `Err("Unresolved stack placeholder")` on the unlocked
   path. Verify against C++ `coreaction.cc ActionStackPtrFlow` + `fspec.cc:4875`.
2. **Register stack-space input trials in `guard_calls`** (drop the `tryregister` short circuit
   for `IPTR_SPACEBASE` once (1) lands) so `ParamActive`/`ActionActiveParam` can score them,
   and **build the argument in `func_link_input`'s locked-stack-param branch** instead of
   `continue`-ing on `op_stack_load` failure.
3. **Confirm the call-effect guard.** Once the argument exists, verify a stack range written
   before an unprototyped call gets its `unknown_effect` INDIRECT from `guard_calls`
   (heritage.rs:1535) - the x86-64 register-argument control already behaves correctly, so the
   guard machinery itself looks sound; it is the escape fact that is missing.
4. Separately (smaller, independent value): apply PE IAT import prototypes so `ReadFile`,
   `CreateFileA`, `CloseHandle` render named and typed rather than `(*dat_411350)()`. kuna
   already *knows* these names - `kuna functions mydoom.exe --json` lists `ReadFile @ 0x411350` -
   the decompiler just does not consume them for the indirect-call target.

Risks: this is upstream-faithful behavior being restored, not a judgment call, so per the
repo rules it is a bug fix rather than an option - but it touches heritage and the call-spec
core, so it will move output on many datatests (calls that render `f()` today will render
`f(a,b)`), and the parity gate has to be worked through carefully. Speed cost is real too:
more live argument varnodes means more heritage work. Suggested first cut is the x86-64 >6-arg
case (small, self-contained, minimal reproducer above) before the i386 blanket case.

Test vectors: the 15-line `work2`/`work3` reproducers above (register-arg control + stack-arg
failing case), plus a `tests/stages/` case built from `mydoom.exe` msg_b64enc asserting the
loop is present.

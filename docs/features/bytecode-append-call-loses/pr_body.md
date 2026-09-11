## The problem

A call renders with fewer arguments than the disassembly passes it, whenever
the missing registers were set up *before* the branch the call sits behind.
The shape is a container `append` with the one-element fast path inlined and
the grow path left as a call — ubiquitous in C++ — and kuna's own recovery of
the callee disagrees with its recovery of the call site.

```
$ kuna decompile vm.exe sub_140004540 | tail -7
  if (*(char **)(a0 + 8) != *(char **)(a0 + 0x10)) {
    **(char **)(a0 + 8) = *(char *)((a1 & 0xff) + 0x14000b290);
    *(long long *)(a0 + 8) = *(long long *)(a0 + 8) + 1;
    return;
  }
  sub_140007be0(a0);
}

$ kuna decompile vm.exe sub_140007be0 | head -1
char *sub_140007be0(long long *a0,long long a1,char *a2)
```

Two live arguments are gone. The disassembly sets both up two instructions
before the test, because both arms need them:

```
$ kuna disassemble vm.exe --addr 0x1400046af --count 8
0x1400046af   MOV RDX,qword ptr [RBX + 0x8]     ; the write cursor
0x1400046b3   LEA R8,[RSI + RDI*0x1]            ; the source byte
0x1400046b7   CMP RDX,qword ptr [RBX + 0x10]
0x1400046bb   JZ 0x1400046c9
0x1400046bd   MOVZX EAX,byte ptr [R8]           ; fast path: LOAD through R8
0x1400046c1   MOV byte ptr [RDX],AL             ;            STORE through RDX
0x1400046c7   JMP 0x1400046d1
0x1400046c9   MOV RCX,RBX
0x1400046cc   CALL 0x140007be0                  ; the other arm
```

`Funcdata::only_op_use` rejects a trial on any `LOAD`/`STORE` among the
value's descendants, and those two accesses are descendants of exactly the
Varnodes the call is passed. No execution path runs both arms, so neither
access can be competing with the call.

## The fix

- New option `exclusivearguse` (`on|off`, default **on**, DIV-161). A
  `LOAD`/`STORE` descendant no longer sinks an input trial when it cannot
  co-execute with the call.
- Mutual unreachability is the soundness argument: a path is one walk from the
  entry block, so a path holding both blocks would make the later reachable
  from the earlier. Two arms of an `if` *inside a loop* do reach each other
  through the back edge, and are declined.
- The value must be **defined in the block that branches**, not merely live at
  it. This is not caution — without it the sweep found real spurious
  arguments: `phantomgate.exe`'s `random_device` constructor opens
  `mov r8,rcx` and dereferences `r8` on the arms that succeed, so every
  `throw` call on a failing arm gained a third argument the disassembly does
  not pass.
- The Varnode must be the access's **address** operand, which leaves the
  `STORE` value slot to `spillargtrial`; and `BRANCH`/`CBRANCH`/`BRANCHIND`
  keep the upstream rejection, which is the shape the whole `calleearity`
  family was built for and warns against relaxing.

## The tests

`tests/stages/kuna-exclusivearguse.xml` is two-pass over a vendored reduction:
off gives `append_slow()`, default gives `append_slow(a0,v1,&table[a1])`, and a
loop control keeps its one argument in both passes. `tests/cli/bytecode-append-call-loses.json`
is the promoted acceptance probe. 11 unit tests cover the predicate, including
the scratch-copy false positive.

## The sweep

`decompile-all` in both arms over 46 images (RE-dataset PE and ELF crackmes
plus Linux system binaries; 3 fail to load in both arms): **3 of 43 changed**,
49 lines removed / 59 added with local names normalised. No function loses a
statement and no call loses an argument. The recoveries are the same shape from
other compilers — `phantomgate.exe` turns libstdc++'s `memcpy()` into
`memcpy(a1,v1)` twice and `memmove()` into `memmove(v1,v2)`.

The pre-narrowing draft changed 10 of 43 and is what the definition test was
measured against.

`make test` PARITY OK 675/675 (no re-pin) · `make test-stages` PARITY OK
768/768 (763 → 768 for the new assertions, none moved) · `make rust-test` green
(6469 tests) · `make check-spec` OK strict · `make test-cli` 125/125 ·
`kuna catalog --check` OK. Speed: 164.61 ms → 161.29 ms on the witness
(`scripts.pipeline.timeit`, median of 7, −2.02%, budget 5%).

🤖 Generated with [Claude Code](https://claude.com/claude-code)

## The problem

Every MSVC frame larger than a page opens with a call to the out-of-line stack
probe, and kuna loses the frame across it. The probe reads `RAX` and never
writes it, so `sub rsp,rax` is a constant allocation — but kuna prints the whole
frame indexed off a local it never assigns:

```
$ kuna decompile crackme_shroud.exe sub_1408dcdf8
void sub_1408dcdf8(unsigned long long *a0,uint4 a1,char *a2,uint8 a3)
{
  int8 v4;
  ...
  v4 = -sub_1408d0000();
  *(uint8 *)&(&Stack0000000000001428)[v4] = dat_1408f1040 ^ (uint8)&(&Stackffffffffffffffe8)[v4];
  ...
    v6 = &(&Stack0000000000000028)[v4];
```

`v4` is the value the call supposedly returns; nothing computes it. The callee
is textbook `__chkstk` and it plainly leaves `RAX` alone:

```
$ cstool -d x64 4883ec104c8914244c895c24084d33db4c8d5424184c2bd04d0f42d3
 0  sub  rsp, 0x10
 4  mov  qword ptr [rsp], r10
 8  mov  qword ptr [rsp + 8], r11
 d  xor  r11, r11
10  lea  r10, [rsp + 0x18]
15  sub  r10, rax          <- reads RAX; nothing in the body writes it
18  cmovb r10, r11
```

Naming the helper `__chkstk`, declaring it `void`, and `--option
calleepreserves on` all fail to recover the constant.

## The fix

- `calleepreserves` already proves this: its body walk completes over all 19
  instructions and records no write to `RAX`. What declined is its
  *positive-evidence* gate, which requires the callee to have written a register
  the convention marks `<unaffected>` — the i386 get-PC thunk it was written
  for. `__chkstk` writes `RSP`, `R10` and `R11`, and `x86-64-win.cspec` names
  none of them in either list. They are scratch, and clobbering scratch is not a
  departure from the convention, so the gate never fires on a well-behaved
  helper.
- New option `calleescratchbody` (`on|off`, default **on**, DIV-149) adds a
  second, independent way for a summary to count as a body. Both marks are
  required: the body wrote **memory** — a register-preserving helper's own save
  slot is why it can use a register at all — and it wrote a register other than
  the stack pointer, which every `RET` writes.
- Requiring the store is what keeps stubs out. `ret`, `endbr64; ret` (how the
  `ghdec-*` fixtures stub every callee outside the chunk under test), a
  placeholder and an entry decoded at the wrong address write no memory at all,
  so their silence is still not evidence.
- Nothing else about `calleepreserves` moves: the walk must still complete, the
  range must still be a register it proves untouched, an explicit effect
  override still wins, and only `killedbycall` is ever downgraded — the pass can
  still only *keep* a value the machine keeps.

With it on, the same function recovers its frame:

```
void sub_1408dcdf8(unsigned long long *a0,uint4 a1,char *a2,uint8 a3)
{
  char v5 [32];
  char v6 [5120];
  uint4 v7 [4];      // stack - 0x1438
  ...
  sub_1408d0000();
  v9 = dat_1408f1040 ^ (uint8)v5;
```

## The tests

`tests/stages/kuna-calleescratchbody.xml` is the two-pass end-to-end case:
`__chkstk` verbatim behind the real calling sequence, with `calleescratchbody
off` pinning the bug and the default pinning the fix, plus an `endbr64; ret`
negative control that must render identically in both passes. Five unit tests in
`kuna_calleescratchbody/tests.rs` pin each way the clause fails closed.

`tests/cli/rax-preserving-stack-probe.json` runs the same shape end to end
through the CLI against a new 4,768-byte fixture,
`decompiler/crates/kuna-analysis/tests/fixtures/calleescratchbody_x86_64` (its
`.s` is checked in beside it). With the option off it renders
`*(unsigned long *)&(&Stack0000000000000008)[-chkstk()] = a0;` and fails.

Gates: `make test` 675/675, `make test-stages` 730/730, `make test-cli` 109/109,
`make rust-test` green, `make check-spec` green, `kuna catalog --check` OK.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

## The problem

An MSVC `/GS` binary's `main` ends by returning the frame-cookie check, although
the machine plainly returns zero: the compiler sets `EAX` *before* the call,
because `__security_check_cookie` never touches `RAX`.

```
$ kuna decompile ./decompiler/crates/kuna-analysis/tests/fixtures/pe_cookiecheck_x86_64.exe 0x140001000 --addr
unsigned int sub_140001000(unsigned int argc,unsigned long long argv,unsigned long long envp)
{
  ...
  v1 = v3 ^ (unsigned long long)v2;
  return sub_140001080(v1);        <-- a value nothing computes
}
```

The disassembly, and what the callee actually does:

```
$ cstool -s x64 "4883ec28488b05221000004833c44889442420 33c0 488b4c24204833cce85a000000 4883c428 c3"
 ...
 0x1013: xor eax, eax             <-- the value main returns
 0x101d: call 0x1080
 0x1026: ret

$ cstool -s x64 "483b0d791000007510 48c1c110 66f7c1ffff 7501 c3 48c1c910 e922000000"
 0x0: cmp rcx, qword ptr [rip + 0x1079]
 0x7: jne 0x19
 0x9: rol rcx, 0x10
 0xd: test cx, 0xffff
 0x12: jne 0x15
 0x14: ret                        <-- the returning path, and no write to rax
```

`x86-64-win.cspec` lists `RAX` in `<killedbycall>`, so the call guard plants an
INDIRECT creation of `RAX` at every call. Declaring the checker `void` does not
help: it trades the invented call result for an uninitialised local. Filed as
RE-need `main-returns-invented-cookie` (2 instances, major).

## The fix

- New option `calleeretpreserves` (default on, DIV-133). For the call's
  **return storage**, a complete decode of the callee's own body that records no
  write to it proves the call has no return value at all: the effect is
  downgraded to `unaffected`, no output trial is registered, and the caller's own
  definition flows across the call.
- `calleepreserves` already narrows `killedbycall` this way but declines here on
  purpose - its evidence is a write to a register the convention promises is
  *preserved*, which a helper that clobbers only what it is allowed to never
  produces. The return register gets its own, sharper finding instead: a callee
  that returns a value in `RAX` must *write* `RAX`.
- Three gates keep the claim one-sided. The range must characterize as the call's
  output. The body must write **no** part of the return storage - a callee that
  writes `RAX` and leaves `RDX` alone is a scalar-returning function whose second
  register is merely dead. And the body must be a body: more than one decoded
  instruction, and a write to a register the convention itself names. That last
  one is not decoration - without it 22 stage assertions regress, because the
  `ghdec-*` fixtures stub every out-of-chunk callee as `c3` or `endbr64; ret`.
- The callee-body probe learns that a Windows `int 0x29` ends the path, under
  `fastfailnoreturn`'s own gate: SLEIGH lifts `INT imm8` to
  `intloc = swi(imm8); call [intloc]`, and the checker's `__fastfail` failure
  path otherwise makes the walk incomplete.

## The tests

`tests/stages/kuna-calleeretpreserves.xml` is the two-pass case (off = the bug,
default = the fix) with an `endbr64; ret` negative control that must render
identically in both passes; `tests/cli/main-returns-invented-cookie.json` runs the
probe over the vendored `pe_cookiecheck_x86_64.exe`. Gates: `make test` PARITY OK
675/675 (0 assertions moved), `make test-stages` PARITY OK 683/683 (purely
additive), `make test-cli` 74/74, `make rust-test` green, `make check-spec` green,
`kuna catalog --check` OK. Sweep over 28 crackmes.one binaries / 9,355 functions:
51 functions changed (0.55%) in 4 binaries, 0 added or removed, every binary exit
0 in both arms. Speed `decompile-all pe_imports.exe` 2.562 s -> 2.486 s (median
of 6).

🤖 Generated with [Claude Code](https://claude.com/claude-code)
